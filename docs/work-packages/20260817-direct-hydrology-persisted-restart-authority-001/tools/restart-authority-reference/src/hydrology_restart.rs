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
    DirectDayConstructorInputs, DirectLaneFrame, DirectLaneTransferLedger, DirectPhasePlan,
    DirectPublicationFrame, DirectRunFrame, DirectRunIdentity, DirectRunTransferShadowProjection,
};
use serde::{Deserialize, Serialize};
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
        if &self.day_inputs_sha256 != expected_day_inputs_sha256 {
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
#[serde(deny_unknown_fields)]
pub struct DirectHydrologyRestartV1 {
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
            laned_active,
            laned_active_summary,
        } = value;
        if laned_active.is_some() || laned_active_summary.is_some() {
            return Err(HydrologyRestartError::Unsupported("laned_active"));
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
        })
    }

    pub fn restore(
        &self,
        phase_plan_sha256: &Sha256Hex,
        day_inputs: Vec<Vec<DirectDayConstructorInputs>>,
        day_input_digests: &[Sha256Hex],
    ) -> Result<DirectRunFrame, HydrologyRestartError> {
        if &self.phase_plan_sha256 != phase_plan_sha256
            || self.lanes.len() != day_inputs.len()
            || self.lanes.len() != day_input_digests.len()
            || self.lane_count != self.lanes.len() as u64
        {
            return Err(HydrologyRestartError::Join("run configuration/cardinality"));
        }
        let lanes = self
            .lanes
            .iter()
            .zip(day_inputs)
            .zip(day_input_digests)
            .map(|((lane, inputs), digest)| lane.restore(inputs, digest))
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
            phase_plan: DirectPhasePlan::default(),
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
                .map(DirectSurfaceLiquidOwnedStateRestartV1::restore)
                .transpose()
                .map_err(nested)?
                .map(Box::new),
            laned_active: None,
            laned_active_summary: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openwepp_hillslope_orchestrator::{
        DirectLaneConstructorInputs, DirectRunConstructorInputs,
    };

    fn sha(byte: char) -> Sha256Hex {
        Sha256Hex::try_new(byte.to_string().repeat(64)).unwrap()
    }
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

    #[test]
    fn complete_projection_restoration_equivalence_and_cache_reconstruction() {
        let source = frame();
        let phase = sha('a');
        let days = vec![sha('b'), sha('c')];
        let dto = DirectHydrologyRestartV1::project(&source, phase.clone(), &days).unwrap();
        let restored = dto
            .restore(
                &phase,
                source
                    .lanes
                    .iter()
                    .map(|lane| lane.day_inputs.clone())
                    .collect(),
                &days,
            )
            .unwrap();
        assert_eq!(
            DirectHydrologyRestartV1::project(&restored, phase, &days).unwrap(),
            dto
        );
        assert!(restored.lane_transfer_shadow_projection.is_none());
        assert!(restored.lanes.iter().all(|lane| lane.snow_runtime_carry.is_none() && lane.frost_runtime_carry.is_none()));
    }

    #[test]
    fn configuration_topology_and_numeric_poisons_reject_without_touching_source() {
        let source = frame();
        let beginning = format!("{source:?}");
        let phase = sha('a');
        let days = vec![sha('b'), sha('c')];
        let mut dto = DirectHydrologyRestartV1::project(&source, phase.clone(), &days).unwrap();
        dto.lanes[1].lane_id = 99;
        assert!(
            dto.restore(
                &phase,
                source
                    .lanes
                    .iter()
                    .map(|lane| lane.day_inputs.clone())
                    .collect(),
                &days
            )
            .is_err()
        );
        assert_eq!(format!("{source:?}"), beginning);
        let mut dto = DirectHydrologyRestartV1::project(&source, phase.clone(), &days).unwrap();
        dto.lanes[0].area_m2 = HexF64::from_f64(f64::NAN);
        assert!(
            dto.restore(
                &phase,
                source
                    .lanes
                    .iter()
                    .map(|lane| lane.day_inputs.clone())
                    .collect(),
                &days
            )
            .is_err()
        );
        assert_eq!(format!("{source:?}"), beginning);
    }
}
