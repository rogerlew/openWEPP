use crate::{
    DirectErosionDownstreamRestartV1, DirectErosionInflowIntakeRestartV1,
    DirectErosionRuntimeCarryRestartV1, DirectEvapotranspirationStageRestartV1,
    DirectGroundwaterRunStateRestartV1, DirectGrowthStateSurfaceRestartV1,
    DirectLaneTransferLedgerRestartV1, DirectRunTransferDownstreamOperandsRestartV1,
    DirectSubsurfaceLayerRestartV1, DirectSurfaceLiquidOwnedStateRestartV1,
    DirectTransferBuffersRestartV1, DirectWaterStateRestartV1, DirectWinterColumnRestartV1, HexF64,
    Sha256Hex,
};
use openwepp_hillslope_orchestrator::{
    DirectDayConstructorInputs, DirectLaneFrame, DirectLaneTransferLedger, DirectPhaseKind,
    DirectPhasePlan, DirectPublicationFrame, DirectRunFrame, DirectRunIdentity,
    DirectRunTransferShadowProjection, DirectSurfaceLiquidConfiguration,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HydrologyRestartError {
    #[error("nested restart owner failed: {0}")]
    Nested(String),
    #[error("unsupported runtime posture: {0}")]
    Unsupported(&'static str),
    #[error("identity, cardinality, topology, configuration, or cache join failed: {0}")]
    Join(&'static str),
    #[error("numeric domain failed: {0}")]
    Domain(&'static str),
}

fn nested(error: impl std::fmt::Display) -> HydrologyRestartError {
    HydrologyRestartError::Nested(error.to_string())
}
fn hexify_floats(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Number(number) if number.is_f64() => {
            if let Some(float) = number.as_f64() {
                *value = serde_json::Value::String(format!("0x{:016x}", float.to_bits()));
            }
        }
        serde_json::Value::Array(values) => values.iter_mut().for_each(hexify_floats),
        serde_json::Value::Object(values) => values.values_mut().for_each(hexify_floats),
        _ => {}
    }
}
pub(crate) fn canonical_operand_sha256(
    domain: &str,
    value: &impl Serialize,
) -> Result<Sha256Hex, HydrologyRestartError> {
    let mut projected = serde_json::to_value(value)
        .map_err(|_| HydrologyRestartError::Join("immutable operand projection"))?;
    hexify_floats(&mut projected);
    let bytes = crate::to_canonical_bytes(&(domain, projected))
        .map_err(|_| HydrologyRestartError::Join("immutable operand canonical bytes"))?;
    Sha256Hex::try_new(format!("{:x}", Sha256::digest(bytes)))
        .map_err(|_| HydrologyRestartError::Join("immutable operand digest"))
}
pub(crate) fn canonical_phase_plan_sha256(
    value: &DirectPhasePlan,
) -> Result<Sha256Hex, HydrologyRestartError> {
    let phases = value
        .phases()
        .iter()
        .map(|phase| match phase {
            DirectPhaseKind::Normalization => "normalization",
            DirectPhaseKind::StorageBounds => "storage_bounds",
            DirectPhaseKind::DecompositionTransition => "decomposition_transition",
            DirectPhaseKind::ResiduePartitionTransition => "residue_partition_transition",
            DirectPhaseKind::AnnualGrowthTransition => "annual_growth_transition",
            DirectPhaseKind::PerennialGrowthTransition => "perennial_growth_transition",
            DirectPhaseKind::PercolationDeepSeepage => "percolation_deep_seepage",
            DirectPhaseKind::Evapotranspiration => "evapotranspiration",
            DirectPhaseKind::Drainage => "drainage",
            DirectPhaseKind::LateralTransfer => "lateral_transfer",
            DirectPhaseKind::PlantRootUptake => "plant_root_uptake",
            DirectPhaseKind::RunoffReconciliation => "runoff_reconciliation",
            DirectPhaseKind::StorageReconciliation => "storage_reconciliation",
            DirectPhaseKind::ClosureDiagnostics => "closure_diagnostics",
        })
        .collect::<Vec<_>>();
    canonical_operand_sha256("DirectPhasePlanV1", &phases)
}
fn finite(field: &'static str, value: &HexF64) -> Result<f64, HydrologyRestartError> {
    let value = value.to_f64();
    value
        .is_finite()
        .then_some(value)
        .ok_or(HydrologyRestartError::Domain(field))
}
fn positive(field: &'static str, value: &HexF64) -> Result<f64, HydrologyRestartError> {
    let value = finite(field, value)?;
    (value > 0.0)
        .then_some(value)
        .ok_or(HydrologyRestartError::Domain(field))
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectLaneRestartV1 {
    pub lane_id: u32,
    pub upstream_lane_id: u32,
    pub downstream_lane_id: u32,
    pub upstream_area_ratio: HexF64,
    pub area_m2: HexF64,
    pub runoff_publication_q_scale: HexF64,
    pub runoff_publication_qofe_scale: HexF64,
    pub runoff_publication_efflen_m: HexF64,
    pub runoff_publication_cumulative_length_m: HexF64,
    pub runoff_publication_ofe_length_m: HexF64,
    pub water: DirectWaterStateRestartV1,
    pub transfer: DirectTransferBuffersRestartV1,
    pub erosion_downstream_operands: DirectErosionDownstreamRestartV1,
    pub erosion_inflow_intake: Option<Box<DirectErosionInflowIntakeRestartV1>>,
    pub subsurface_layers: Vec<DirectSubsurfaceLayerRestartV1>,
    pub evapotranspiration_stage_state: Option<Box<DirectEvapotranspirationStageRestartV1>>,
    pub plant_growth_state: Box<DirectGrowthStateSurfaceRestartV1>,
    pub plant_water_stress: HexF64,
    pub winter_column: Box<DirectWinterColumnRestartV1>,
    pub erosion_runtime_carry: DirectErosionRuntimeCarryRestartV1,
    pub day_inputs_sha256: Sha256Hex,
}

impl DirectLaneRestartV1 {
    pub fn project(
        value: &DirectLaneFrame,
        day_inputs_sha256: Sha256Hex,
    ) -> Result<Self, HydrologyRestartError> {
        let DirectLaneFrame {
            lane_id,
            upstream_lane_id,
            downstream_lane_id,
            upstream_area_ratio,
            area_m2,
            runoff_publication_q_scale,
            runoff_publication_qofe_scale,
            runoff_publication_efflen_m,
            runoff_publication_cumulative_length_m,
            runoff_publication_ofe_length_m,
            water,
            transfer,
            publication: _,
            erosion_downstream_operands,
            erosion_inflow_intake,
            subsurface_layers,
            evapotranspiration_stage_state,
            plant_growth_state,
            plant_water_stress,
            winter_column,
            snow_runtime_carry,
            frost_runtime_carry,
            erosion_runtime_carry,
            day_inputs: _,
        } = value;
        let projected_winter = DirectWinterColumnRestartV1::project(
            winter_column,
            snow_runtime_carry.as_deref(),
            frost_runtime_carry.as_ref(),
        )
        .map_err(nested)?;
        projected_winter
            .validate_child4_snow_free()
            .map_err(nested)?;
        Ok(Self {
            lane_id: *lane_id,
            upstream_lane_id: *upstream_lane_id,
            downstream_lane_id: *downstream_lane_id,
            upstream_area_ratio: HexF64::from_f64(*upstream_area_ratio),
            area_m2: HexF64::from_f64(*area_m2),
            runoff_publication_q_scale: HexF64::from_f64(*runoff_publication_q_scale),
            runoff_publication_qofe_scale: HexF64::from_f64(*runoff_publication_qofe_scale),
            runoff_publication_efflen_m: HexF64::from_f64(*runoff_publication_efflen_m),
            runoff_publication_cumulative_length_m: HexF64::from_f64(
                *runoff_publication_cumulative_length_m,
            ),
            runoff_publication_ofe_length_m: HexF64::from_f64(*runoff_publication_ofe_length_m),
            water: DirectWaterStateRestartV1::project(water),
            transfer: DirectTransferBuffersRestartV1::project(transfer),
            erosion_downstream_operands: DirectErosionDownstreamRestartV1::project(
                erosion_downstream_operands,
            ),
            erosion_inflow_intake: erosion_inflow_intake
                .as_deref()
                .map(DirectErosionInflowIntakeRestartV1::project)
                .map(Box::new),
            subsurface_layers: subsurface_layers
                .iter()
                .map(DirectSubsurfaceLayerRestartV1::project)
                .collect(),
            evapotranspiration_stage_state: evapotranspiration_stage_state
                .as_deref()
                .map(DirectEvapotranspirationStageRestartV1::project)
                .map(Box::new),
            plant_growth_state: Box::new(DirectGrowthStateSurfaceRestartV1::project(
                plant_growth_state,
            )),
            plant_water_stress: HexF64::from_f64(*plant_water_stress),
            winter_column: Box::new(projected_winter),
            erosion_runtime_carry: DirectErosionRuntimeCarryRestartV1::project(
                erosion_runtime_carry,
            ),
            day_inputs_sha256,
        })
    }

    pub fn restore(
        &self,
        day_inputs: Vec<DirectDayConstructorInputs>,
        expected_day_inputs_sha256: &Sha256Hex,
    ) -> Result<DirectLaneFrame, HydrologyRestartError> {
        let actual = canonical_operand_sha256("DirectDayConstructorInputsV1", &day_inputs)?;
        if &self.day_inputs_sha256 != expected_day_inputs_sha256 || self.day_inputs_sha256 != actual
        {
            return Err(HydrologyRestartError::Join("day_inputs_sha256"));
        }
        self.winter_column
            .validate_child4_snow_free()
            .map_err(nested)?;
        let winter = self.winter_column.restore().map_err(nested)?;
        let plant_water_stress = finite("plant_water_stress", &self.plant_water_stress)?;
        if !(0.0..=1.0).contains(&plant_water_stress) {
            return Err(HydrologyRestartError::Domain("plant_water_stress"));
        }
        Ok(DirectLaneFrame {
            lane_id: self.lane_id,
            upstream_lane_id: self.upstream_lane_id,
            downstream_lane_id: self.downstream_lane_id,
            upstream_area_ratio: positive("upstream_area_ratio", &self.upstream_area_ratio)?,
            area_m2: positive("area_m2", &self.area_m2)?,
            runoff_publication_q_scale: positive(
                "runoff_publication_q_scale",
                &self.runoff_publication_q_scale,
            )?,
            runoff_publication_qofe_scale: positive(
                "runoff_publication_qofe_scale",
                &self.runoff_publication_qofe_scale,
            )?,
            runoff_publication_efflen_m: positive(
                "runoff_publication_efflen_m",
                &self.runoff_publication_efflen_m,
            )?,
            runoff_publication_cumulative_length_m: positive(
                "runoff_publication_cumulative_length_m",
                &self.runoff_publication_cumulative_length_m,
            )?,
            runoff_publication_ofe_length_m: positive(
                "runoff_publication_ofe_length_m",
                &self.runoff_publication_ofe_length_m,
            )?,
            water: self.water.restore().map_err(nested)?,
            transfer: self.transfer.restore().map_err(nested)?,
            publication: DirectPublicationFrame::empty(),
            erosion_downstream_operands: self
                .erosion_downstream_operands
                .restore()
                .map_err(nested)?,
            erosion_inflow_intake: self
                .erosion_inflow_intake
                .as_deref()
                .map(DirectErosionInflowIntakeRestartV1::restore)
                .transpose()
                .map_err(nested)?
                .map(Box::new),
            subsurface_layers: self
                .subsurface_layers
                .iter()
                .map(DirectSubsurfaceLayerRestartV1::restore)
                .collect::<Result<_, _>>()
                .map_err(nested)?,
            evapotranspiration_stage_state: self
                .evapotranspiration_stage_state
                .as_deref()
                .map(DirectEvapotranspirationStageRestartV1::restore)
                .transpose()
                .map_err(nested)?
                .map(Box::new),
            plant_growth_state: Box::new(self.plant_growth_state.restore().map_err(nested)?),
            plant_water_stress,
            winter_column: Box::new(winter.winter_column),
            snow_runtime_carry: winter.snow_runtime_carry.map(Box::new),
            frost_runtime_carry: winter.frost_runtime_carry,
            erosion_runtime_carry: self.erosion_runtime_carry.restore().map_err(nested)?,
            day_inputs,
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DirectRuntimePostureV1 {
    Standard,
    UnsupportedLanedActive,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectHydrologyRestartV1 {
    pub runtime_posture: DirectRuntimePostureV1,
    pub run_id: u64,
    pub hillslope_id: u32,
    pub lane_count: u64,
    pub day_count: u64,
    pub phase_plan_sha256: Sha256Hex,
    pub lanes: Vec<DirectLaneRestartV1>,
    pub lane_transfer_ledger: Vec<DirectLaneTransferLedgerRestartV1>,
    pub lane_transfer_downstream_operands: DirectRunTransferDownstreamOperandsRestartV1,
    pub groundwater: DirectGroundwaterRunStateRestartV1,
    pub surface_liquid_owned_state: Option<Box<DirectSurfaceLiquidOwnedStateRestartV1>>,
    pub snow_stage3_v11_attachment: serde_json::Value,
}

pub struct ExpectedDirectHydrologyRestartContext<'a> {
    pub phase_plan: &'a DirectPhasePlan,
    pub phase_plan_sha256: &'a Sha256Hex,
    pub day_inputs: &'a [Vec<DirectDayConstructorInputs>],
    pub day_input_digests: &'a [Sha256Hex],
    pub surface_liquid_configuration: &'a DirectSurfaceLiquidConfiguration,
}

impl DirectHydrologyRestartV1 {
    pub fn project(
        value: &DirectRunFrame,
        phase_plan_sha256: Sha256Hex,
        day_input_digests: &[Sha256Hex],
    ) -> Result<Self, HydrologyRestartError> {
        let DirectRunFrame {
            identity,
            lanes,
            phase_plan,
            publication: _,
            lane_transfer_ledger,
            lane_transfer_downstream_operands,
            lane_transfer_shadow_projection,
            groundwater,
            surface_liquid_shadow,
            snow_stage3_v11_attachment,
            laned_active,
            laned_active_summary,
        } = value;
        if phase_plan_sha256 != canonical_phase_plan_sha256(phase_plan)?
            || lanes.len() != day_input_digests.len()
            || lanes.iter().zip(day_input_digests).any(|(lane, digest)| {
                canonical_operand_sha256("DirectDayConstructorInputsV1", &lane.day_inputs)
                    .map_or(true, |actual| &actual != digest)
            })
        {
            return Err(HydrologyRestartError::Join("immutable operand digest"));
        }
        if laned_active.is_some() || laned_active_summary.is_some() {
            return Err(HydrologyRestartError::Unsupported("laned_active"));
        }
        if snow_stage3_v11_attachment.is_some() {
            return Err(HydrologyRestartError::Unsupported(
                "snow_stage3_v11_attachment_v2",
            ));
        }
        if phase_plan != &DirectPhasePlan::default() {
            return Err(HydrologyRestartError::Unsupported(
                "noncanonical_phase_plan",
            ));
        }
        if lanes.len() != day_input_digests.len() || identity.lane_count != lanes.len() {
            return Err(HydrologyRestartError::Join("lane_count"));
        }
        if let Some(cache) = lane_transfer_shadow_projection
            && (cache.lane_count != lane_transfer_downstream_operands.lane_count
                || cache.outlet_lane_id != lane_transfer_downstream_operands.outlet_lane_id
                || cache.total_outgoing_surface_m.to_bits()
                    != lane_transfer_downstream_operands
                        .total_outgoing_surface_m
                        .to_bits()
                || cache.total_outgoing_lateral_m.to_bits()
                    != lane_transfer_downstream_operands
                        .total_outgoing_lateral_m
                        .to_bits()
                || cache.total_received_surface_m.to_bits()
                    != lane_transfer_downstream_operands
                        .total_received_surface_m
                        .to_bits()
                || cache.total_received_lateral_m.to_bits()
                    != lane_transfer_downstream_operands
                        .total_received_lateral_m
                        .to_bits()
                || cache.total_net_transfer_m.to_bits()
                    != lane_transfer_downstream_operands
                        .total_net_transfer_m
                        .to_bits())
        {
            return Err(HydrologyRestartError::Join(
                "lane_transfer_shadow_projection",
            ));
        }
        Ok(Self {
            runtime_posture: DirectRuntimePostureV1::Standard,
            run_id: identity.run_id,
            hillslope_id: identity.hillslope_id,
            lane_count: u64::try_from(identity.lane_count)
                .map_err(|_| HydrologyRestartError::Domain("lane_count"))?,
            day_count: u64::try_from(identity.day_count)
                .map_err(|_| HydrologyRestartError::Domain("day_count"))?,
            phase_plan_sha256,
            lanes: lanes
                .iter()
                .zip(day_input_digests)
                .map(|(lane, digest)| DirectLaneRestartV1::project(lane, digest.clone()))
                .collect::<Result<_, _>>()?,
            lane_transfer_ledger: lane_transfer_ledger
                .iter()
                .map(DirectLaneTransferLedgerRestartV1::project)
                .collect::<Result<_, _>>()
                .map_err(nested)?,
            lane_transfer_downstream_operands:
                DirectRunTransferDownstreamOperandsRestartV1::project(
                    lane_transfer_downstream_operands,
                )
                .map_err(nested)?,
            groundwater: DirectGroundwaterRunStateRestartV1::project(groundwater),
            surface_liquid_owned_state: surface_liquid_shadow
                .as_deref()
                .map(DirectSurfaceLiquidOwnedStateRestartV1::project)
                .transpose()
                .map_err(nested)?
                .map(Box::new),
            snow_stage3_v11_attachment: serde_json::Value::Null,
        })
    }

    pub fn restore(
        &self,
        context: &ExpectedDirectHydrologyRestartContext<'_>,
    ) -> Result<DirectRunFrame, HydrologyRestartError> {
        if self.runtime_posture != DirectRuntimePostureV1::Standard {
            return Err(HydrologyRestartError::Unsupported("laned_active"));
        }
        if !self.snow_stage3_v11_attachment.is_null() {
            return Err(HydrologyRestartError::Unsupported(
                "snow_stage3_v11_attachment_v2",
            ));
        }
        if &self.phase_plan_sha256 != context.phase_plan_sha256
            || self.phase_plan_sha256 != canonical_phase_plan_sha256(context.phase_plan)?
            || self.lanes.len() != context.day_inputs.len()
            || self.lanes.len() != context.day_input_digests.len()
            || self.lane_count != self.lanes.len() as u64
        {
            return Err(HydrologyRestartError::Join("run configuration/cardinality"));
        }
        let lanes = self
            .lanes
            .iter()
            .zip(context.day_inputs)
            .zip(context.day_input_digests)
            .map(|((lane, inputs), digest)| lane.restore(inputs.clone(), digest))
            .collect::<Result<Vec<_>, _>>()?;
        for (index, lane) in lanes.iter().enumerate() {
            let expected_id =
                u32::try_from(index + 1).map_err(|_| HydrologyRestartError::Domain("lane_id"))?;
            if lane.lane_id != expected_id
                || lane.upstream_lane_id != expected_id.saturating_sub(1)
                || lane.downstream_lane_id
                    != if index + 1 == lanes.len() {
                        0
                    } else {
                        expected_id + 1
                    }
            {
                return Err(HydrologyRestartError::Join("lane topology"));
            }
        }
        let total_area_m2 = lanes.iter().try_fold(0.0, |sum, lane| {
            let next = sum + lane.area_m2;
            next.is_finite()
                .then_some(next)
                .ok_or(HydrologyRestartError::Domain("total_area_m2"))
        })?;
        let downstream = self
            .lane_transfer_downstream_operands
            .restore()
            .map_err(nested)?;
        let shadow = (downstream.lane_count != 0).then_some(DirectRunTransferShadowProjection {
            lane_count: downstream.lane_count,
            outlet_lane_id: downstream.outlet_lane_id,
            total_outgoing_surface_m: downstream.total_outgoing_surface_m,
            total_outgoing_lateral_m: downstream.total_outgoing_lateral_m,
            total_received_surface_m: downstream.total_received_surface_m,
            total_received_lateral_m: downstream.total_received_lateral_m,
            total_net_transfer_m: downstream.total_net_transfer_m,
        });
        let lane_transfer_ledger = self
            .lane_transfer_ledger
            .iter()
            .map(DirectLaneTransferLedgerRestartV1::restore)
            .collect::<Result<Vec<DirectLaneTransferLedger>, _>>()
            .map_err(nested)?;
        if lane_transfer_ledger.len() != lanes.len()
            || lane_transfer_ledger
                .iter()
                .zip(&lanes)
                .any(|(entry, lane)| {
                    entry.lane_id != lane.lane_id
                        || entry.upstream_lane_id != lane.upstream_lane_id
                        || entry.downstream_lane_id != lane.downstream_lane_id
                        || entry.area_m2.to_bits() != lane.area_m2.to_bits()
                        || entry.upstream_area_ratio.to_bits() != lane.upstream_area_ratio.to_bits()
                })
        {
            return Err(HydrologyRestartError::Join("lane_transfer_ledger"));
        }
        Ok(DirectRunFrame {
            identity: DirectRunIdentity::new(
                self.run_id,
                self.hillslope_id,
                usize::try_from(self.lane_count)
                    .map_err(|_| HydrologyRestartError::Domain("lane_count"))?,
                usize::try_from(self.day_count)
                    .map_err(|_| HydrologyRestartError::Domain("day_count"))?,
            )
            .map_err(nested)?,
            lanes,
            phase_plan: context.phase_plan.clone(),
            publication: DirectPublicationFrame::empty(),
            lane_transfer_ledger,
            lane_transfer_downstream_operands: downstream,
            lane_transfer_shadow_projection: shadow,
            groundwater: self
                .groundwater
                .restore_for_total_area(Some(total_area_m2))
                .map_err(nested)?,
            surface_liquid_shadow: self
                .surface_liquid_owned_state
                .as_deref()
                .map(|state| state.restore_with_configuration(context.surface_liquid_configuration))
                .transpose()
                .map_err(nested)?
                .map(Box::new),
            snow_stage3_v11_attachment: None,
            laned_active: None,
            laned_active_summary: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openwepp_hillslope_orchestrator::{
        DirectLaneConstructorInputs, DirectRunConstructorInputs, DirectSurfaceLiquidConfiguration,
    };
    use openwepp_kernel_contract::ResourceOwnerId;

    fn frame() -> DirectRunFrame {
        let identity = DirectRunIdentity::new(9, 4, 2, 1).unwrap();
        let mut first = DirectLaneConstructorInputs::from_topology(0, 2, 1).unwrap();
        first.area_m2 = 10.0;
        let mut second = DirectLaneConstructorInputs::from_topology(1, 2, 1).unwrap();
        second.area_m2 = 20.0;
        let mut frame = DirectRunFrame::from_constructor_inputs(DirectRunConstructorInputs::new(
            identity,
            vec![first, second],
        ))
        .unwrap();
        frame.lane_transfer_ledger = vec![
            DirectLaneTransferLedger {
                lane_id: 1,
                upstream_lane_id: 0,
                downstream_lane_id: 2,
                upstream_area_ratio: 1.0,
                area_m2: 10.0,
                outgoing_surface_m: 0.0,
                outgoing_lateral_m: 0.0,
                received_surface_m: 0.0,
                received_lateral_m: 0.0,
                net_transfer_m: 0.0,
            },
            DirectLaneTransferLedger {
                lane_id: 2,
                upstream_lane_id: 1,
                downstream_lane_id: 0,
                upstream_area_ratio: 1.0,
                area_m2: 20.0,
                outgoing_surface_m: 0.0,
                outgoing_lateral_m: 0.0,
                received_surface_m: 0.0,
                received_lateral_m: 0.0,
                net_transfer_m: 0.0,
            },
        ];
        frame
    }
    fn surface_configuration() -> DirectSurfaceLiquidConfiguration {
        DirectSurfaceLiquidConfiguration {
            owner_id: ResourceOwnerId::try_new("surface").unwrap(),
            run_id: 9,
            configuration_sha256: "a".repeat(64),
            ofe_topology: vec![],
            ofe_bindings: vec![],
            records: vec![],
        }
    }
    fn cache_digests(source: &DirectRunFrame) -> (Sha256Hex, Vec<Sha256Hex>) {
        let phase = canonical_phase_plan_sha256(&source.phase_plan).unwrap();
        let days = source
            .lanes
            .iter()
            .map(|lane| {
                canonical_operand_sha256("DirectDayConstructorInputsV1", &lane.day_inputs).unwrap()
            })
            .collect();
        (phase, days)
    }

    #[test]
    fn complete_projection_restoration_equivalence_and_cache_reconstruction() {
        let source = frame();
        let (phase, days) = cache_digests(&source);
        let dto = DirectHydrologyRestartV1::project(&source, phase.clone(), &days).unwrap();
        let day_inputs = source
            .lanes
            .iter()
            .map(|lane| lane.day_inputs.clone())
            .collect::<Vec<_>>();
        let surface = surface_configuration();
        let context = ExpectedDirectHydrologyRestartContext {
            phase_plan: &source.phase_plan,
            phase_plan_sha256: &phase,
            day_inputs: &day_inputs,
            day_input_digests: &days,
            surface_liquid_configuration: &surface,
        };
        let restored = dto.restore(&context).unwrap();
        assert_eq!(
            DirectHydrologyRestartV1::project(&restored, phase, &days).unwrap(),
            dto
        );
        assert!(restored.lane_transfer_shadow_projection.is_none());
        assert!(restored.lanes.iter().all(|lane| lane.snow_runtime_carry.is_none() && lane.frost_runtime_carry.is_none()));
        let mut wrong_inputs = day_inputs.clone();
        wrong_inputs[0][0].interception_m = 1.0;
        let poisoned_context = ExpectedDirectHydrologyRestartContext {
            phase_plan: &source.phase_plan,
            phase_plan_sha256: &dto.phase_plan_sha256,
            day_inputs: &wrong_inputs,
            day_input_digests: &days,
            surface_liquid_configuration: &surface,
        };
        assert!(matches!(
            dto.restore(&poisoned_context),
            Err(HydrologyRestartError::Join("day_inputs_sha256"))
        ));
    }

    #[test]
    fn configuration_topology_and_numeric_poisons_reject_without_touching_source() {
        let source = frame();
        let beginning = format!("{source:?}");
        let (phase, days) = cache_digests(&source);
        let mut dto = DirectHydrologyRestartV1::project(&source, phase.clone(), &days).unwrap();
        dto.lanes[1].lane_id = 99;
        let day_inputs = source
            .lanes
            .iter()
            .map(|lane| lane.day_inputs.clone())
            .collect::<Vec<_>>();
        let surface = surface_configuration();
        let context = ExpectedDirectHydrologyRestartContext {
            phase_plan: &source.phase_plan,
            phase_plan_sha256: &phase,
            day_inputs: &day_inputs,
            day_input_digests: &days,
            surface_liquid_configuration: &surface,
        };
        assert!(dto.restore(&context).is_err());
        assert_eq!(format!("{source:?}"), beginning);
        let mut dto = DirectHydrologyRestartV1::project(&source, phase.clone(), &days).unwrap();
        dto.lanes[0].area_m2 = HexF64::from_f64(f64::NAN);
        assert!(dto.restore(&context).is_err());
        assert_eq!(format!("{source:?}"), beginning);
    }
}
