//! Complete heterogeneous-tile potential/final execution over one OFE owner set.
//!
//! Every projected tile is solved twice from one immutable beginning problem:
//! first without owner caps to construct one request batch, then under its
//! exact subset of the single real-owner authorization. There is no companion
//! request, use, LSE-state, or soil-thermal-candidate input surface.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::TileId;
use openwepp_land_surface_energy::{
    AcceptedOpenSurface, ClosureValue, CoveredColumnInputs, CoveredLowerBoundaryEnergyOperands,
    CoveredPotentialPhase, CoveredTileEnergyOperandSet, FinalCoveredTileCandidate,
    FinalTileCandidate, GroundWaterKey, OfeId, OpenPotentialPhase, OpenSurfaceProblem,
    PotentialWaterRequestBatch, RootRuntimeIdentity, RuntimeTileIdentity,
    StandGroundWaterAmountBasis, TileEnergyOperandSet, TileState, WaterAmount, WaterAuthorization,
    WaterProtocol, WeightedTileEnergyOperands, canonical_tile_fraction_sum_closes,
    solve_covered_potential_phase, solve_open_potential_phase, validate_weighted_ofe_energy,
};

use super::v8_input_projection::V8SoilThermalPhysicalBeginning;
use super::{
    CoveredIngressSchedule, DirectSurfaceLiquidConfiguration,
    LandSurfaceEnergyRealHydrologyAdapter, LandSurfaceEnergyShadowError, OwnerKind,
    RealHydrologySourceKey, SoilThermalTileCandidate, UnifiedLseFinalization,
    UnifiedRealHydrologyCandidate, UnifiedReceiverExpectations,
    finalize_covered_phase_with_soil_thermal_beginning,
    finalize_open_phase_with_soil_thermal_beginning,
};

/// Strictly projected open-tile problem and its numerical trials.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct StrictProjectedOpenTile {
    pub(crate) identity: RuntimeTileIdentity,
    pub(crate) beginning: OpenSurfaceProblem,
    pub(crate) potential_initial_trial: Option<Vec<f64>>,
    pub(crate) final_initial_trial: Option<Vec<f64>>,
    pub(crate) soil_thermal: V8SoilThermalPhysicalBeginning,
}

/// Strictly projected covered-tile problem and its numerical trials.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct StrictProjectedCoveredTile {
    pub(crate) identity: RuntimeTileIdentity,
    pub(crate) beginning: CoveredColumnInputs,
    pub(crate) roots: Vec<RootRuntimeIdentity>,
    pub(crate) potential_initial_trial: Vec<f64>,
    pub(crate) final_initial_trial: Vec<f64>,
    pub(crate) soil_thermal: V8SoilThermalPhysicalBeginning,
}

/// Topology-preserving member whose physical snow surface is owned entirely by
/// Stage 3. It must not enter the ordinary open-ground solver.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct StrictProjectedStage3OpenSnowTile {
    pub(crate) identity: RuntimeTileIdentity,
    pub(crate) beginning_state: TileState,
    pub(crate) soil_thermal: V8SoilThermalPhysicalBeginning,
}

/// One member of the exact configured heterogeneous tile set.
#[derive(Clone, Debug, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub(crate) enum StrictProjectedTileProblem {
    Open(StrictProjectedOpenTile),
    Stage3OpenSnow(StrictProjectedStage3OpenSnowTile),
    Covered(StrictProjectedCoveredTile),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MultiTileFailurePhase {
    PotentialTile(usize),
    CombinedRequests,
    Authorization,
    FinalTile(usize),
    E04Ingress,
    OpenIngress,
    UnifiedHydrology,
    LocalEnergy,
    OfeEnergy,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PendingPayloadKind {
    CombinedRequest,
    Authorization,
    FinalTileUse,
    FinalProtocol,
    Ingress,
}

type FailureHook<'a> =
    Option<&'a dyn Fn(MultiTileFailurePhase) -> Result<(), LandSurfaceEnergyShadowError>>;
type PendingEnvelopeHook<'a> =
    Option<&'a dyn Fn(PendingPayloadKind, &[u8]) -> Result<(), LandSurfaceEnergyShadowError>>;

impl StrictProjectedTileProblem {
    fn identity(&self) -> &RuntimeTileIdentity {
        match self {
            Self::Open(value) => &value.identity,
            Self::Stage3OpenSnow(value) => &value.identity,
            Self::Covered(value) => &value.identity,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum PotentialTilePhase {
    Open {
        phase: OpenPotentialPhase,
        final_initial_trial: Option<Vec<f64>>,
        soil_thermal: V8SoilThermalPhysicalBeginning,
    },
    Stage3OpenSnow {
        identity: RuntimeTileIdentity,
        beginning_state: TileState,
        soil_thermal: V8SoilThermalPhysicalBeginning,
        request_batch: PotentialWaterRequestBatch,
    },
    Covered {
        phase: CoveredPotentialPhase,
        final_initial_trial: Vec<f64>,
        soil_thermal: V8SoilThermalPhysicalBeginning,
    },
}

impl PotentialTilePhase {
    fn identity(&self) -> &RuntimeTileIdentity {
        match self {
            Self::Open { phase, .. } => &phase.identity,
            Self::Stage3OpenSnow { identity, .. } => identity,
            Self::Covered { phase, .. } => phase.identity(),
        }
    }

    pub(super) fn request_batch(&self) -> Option<&PotentialWaterRequestBatch> {
        match self {
            Self::Open { phase, .. } => Some(&phase.request_batch),
            Self::Stage3OpenSnow { request_batch, .. } => Some(request_batch),
            Self::Covered { phase, .. } => Some(phase.request_batch()),
        }
    }

    pub(crate) fn covered(&self) -> Option<&CoveredPotentialPhase> {
        match self {
            Self::Open { .. } => None,
            Self::Stage3OpenSnow { .. } => None,
            Self::Covered { phase, .. } => Some(phase),
        }
    }
}

/// Accepted fixed-cap candidate for one exact tile.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum FinalizedRuntimeTile {
    Open(FinalTileCandidate<AcceptedOpenSurface>),
    Stage3OpenSnow {
        identity: RuntimeTileIdentity,
        ending_tile_state_pre_ingress: TileState,
        soil_thermal: SoilThermalTileCandidate,
        water_protocol: WaterProtocol,
        rollback_hashes: Vec<openwepp_land_surface_energy::OwnerRollbackHash>,
    },
    Covered(FinalCoveredTileCandidate),
}

impl FinalizedRuntimeTile {
    #[must_use]
    fn identity(&self) -> &RuntimeTileIdentity {
        match self {
            Self::Open(value) => &value.identity,
            Self::Stage3OpenSnow { identity, .. } => identity,
            Self::Covered(value) => &value.identity,
        }
    }

    pub(crate) fn covered(&self) -> Option<&FinalCoveredTileCandidate> {
        match self {
            Self::Open(_) => None,
            Self::Stage3OpenSnow { .. } => None,
            Self::Covered(value) => Some(value),
        }
    }

    #[must_use]
    pub(super) fn water_protocol(&self) -> Option<&WaterProtocol> {
        match self {
            Self::Open(value) => Some(&value.water_protocol),
            Self::Stage3OpenSnow { water_protocol, .. } => Some(water_protocol),
            Self::Covered(value) => Some(&value.water_protocol),
        }
    }

    #[must_use]
    fn ending_tile_state_pre_ingress(&self) -> &TileState {
        match self {
            Self::Open(value) => &value.ending_tile_state_pre_ingress,
            Self::Stage3OpenSnow {
                ending_tile_state_pre_ingress,
                ..
            } => ending_tile_state_pre_ingress,
            Self::Covered(value) => &value.ending_tile_state_pre_ingress,
        }
    }

    #[must_use]
    fn soil_thermal(&self) -> &SoilThermalTileCandidate {
        match self {
            Self::Open(value) => &value.soil_thermal,
            Self::Stage3OpenSnow { soil_thermal, .. } => soil_thermal,
            Self::Covered(value) => &value.soil_thermal,
        }
    }

    #[must_use]
    fn energy_operands(&self) -> RuntimeTileEnergyOperands<'_> {
        match self {
            Self::Open(value) => RuntimeTileEnergyOperands::Open(&value.energy_operands),
            Self::Stage3OpenSnow { .. } => RuntimeTileEnergyOperands::Stage3OpenSnow,
            Self::Covered(value) => RuntimeTileEnergyOperands::Covered(&value.energy_operands),
        }
    }
}

/// Borrowed local energy operands retaining open/covered type identity.
#[derive(Clone, Copy, Debug)]
enum RuntimeTileEnergyOperands<'a> {
    Open(&'a TileEnergyOperandSet),
    Stage3OpenSnow,
    Covered(&'a CoveredTileEnergyOperandSet),
}

impl RuntimeTileEnergyOperands<'_> {
    fn validate(&self) -> Result<(), LandSurfaceEnergyShadowError> {
        match self {
            Self::Open(value) => value.validate()?,
            Self::Stage3OpenSnow => {}
            Self::Covered(value) => value.validate()?,
        }
        Ok(())
    }
}

/// Independently reconstructed weighted ground-control-volume join for one
/// complete OFE tile set.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct WeightedOfeEnergyJoin {
    ofe_id: OfeId,
    ordered_tile_ids: Vec<TileId>,
    operands: Vec<WeightedTileEnergyOperands>,
    closure: ClosureValue,
}

/// Complete multi-tile result. Final tiles are generated internally and are
/// never accepted from the caller.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct MultiTileRuntimeResult {
    potential_request_batch: PotentialWaterRequestBatch,
    potential_tiles: Vec<PotentialTilePhase>,
    finalized_tiles: Vec<FinalizedRuntimeTile>,
    weighted_ofe_energy: Vec<WeightedOfeEnergyJoin>,
    hydrology_candidate: UnifiedRealHydrologyCandidate,
}

impl MultiTileRuntimeResult {
    #[must_use]
    #[allow(dead_code)]
    pub(crate) const fn potential_request_batch(&self) -> &PotentialWaterRequestBatch {
        &self.potential_request_batch
    }

    #[must_use]
    pub(crate) fn potential_tiles(&self) -> &[PotentialTilePhase] {
        &self.potential_tiles
    }

    #[must_use]
    pub(crate) fn finalized_tiles(&self) -> &[FinalizedRuntimeTile] {
        &self.finalized_tiles
    }

    #[must_use]
    #[allow(dead_code)]
    pub(crate) fn weighted_ofe_energy(&self) -> &[WeightedOfeEnergyJoin] {
        &self.weighted_ofe_energy
    }

    #[must_use]
    pub(crate) const fn hydrology_candidate(&self) -> &UnifiedRealHydrologyCandidate {
        &self.hydrology_candidate
    }

    /// Versioned, exhaustive projection of the covered-only physical tiles
    /// retained by [`ProvisionalCoveredV8PhysicalEvaluationV1`]. This stays
    /// test-only and returns bytes, not an owner or cloneable physical value.
    #[cfg(test)]
    pub(crate) fn canonical_covered_physical_projection_v1(
        &self,
    ) -> Result<Vec<u8>, LandSurfaceEnergyShadowError> {
        let mut out = CanonicalCoveredPhysicalWriterV1::new(
            b"OPENWEPP_MULTI_TILE_COVERED_PHYSICAL_PROJECTION_V1\0",
        );
        out.potential_request_batch(&self.potential_request_batch)?;
        out.usize(self.potential_tiles.len());
        for potential in &self.potential_tiles {
            out.potential_tile(potential)?;
        }
        out.usize(self.finalized_tiles.len());
        for finalized in &self.finalized_tiles {
            out.finalized_tile(finalized)?;
        }
        out.usize(self.weighted_ofe_energy.len());
        for weighted in &self.weighted_ofe_energy {
            out.string(weighted.ofe_id.as_str());
            out.usize(weighted.ordered_tile_ids.len());
            for tile_id in &weighted.ordered_tile_ids {
                out.string(tile_id.as_str());
            }
            out.usize(weighted.operands.len());
            for operand in &weighted.operands {
                out.f64s(&[
                    operand.tile_fraction,
                    operand.local_input_j_m2_tile,
                    operand.local_output_j_m2_tile,
                    operand.local_storage_change_j_m2_tile,
                    operand.local_sum_abs_integrated_components_j_m2_tile,
                ]);
            }
            out.f64s(&[
                weighted.closure.reconstructed_residual,
                weighted.closure.tolerance,
            ]);
        }
        Ok(out.finish())
    }
}

#[cfg(test)]
pub(crate) fn canonical_runtime_identity_projection_v1(
    identity: &openwepp_land_surface_energy::RuntimeTileIdentity,
) -> Vec<u8> {
    let mut out =
        CanonicalCoveredPhysicalWriterV1::new(b"OPENWEPP_RUNTIME_TILE_IDENTITY_PROJECTION_V1\0");
    out.runtime_identity(identity);
    out.finish()
}

#[cfg(test)]
pub(crate) fn canonical_covered_beginning_projection_v1(
    beginning: &openwepp_land_surface_energy::CoveredColumnInputs,
) -> Result<Vec<u8>, LandSurfaceEnergyShadowError> {
    let mut out =
        CanonicalCoveredPhysicalWriterV1::new(b"OPENWEPP_COVERED_COLUMN_BEGINNING_PROJECTION_V1\0");
    out.covered_beginning(beginning)?;
    Ok(out.finish())
}

#[cfg(test)]
pub(crate) fn canonical_finalized_runtime_tile_projection_v1(
    tile: &FinalizedRuntimeTile,
) -> Result<Vec<u8>, LandSurfaceEnergyShadowError> {
    let mut out =
        CanonicalCoveredPhysicalWriterV1::new(b"OPENWEPP_FINALIZED_RUNTIME_TILE_PROJECTION_V1\0");
    out.finalized_tile(tile)?;
    Ok(out.finish())
}

#[cfg(test)]
pub(crate) fn canonical_soil_thermal_candidate_set_projection_v1(
    schema: &[u8],
    candidates: &[SoilThermalTileCandidate],
) -> Result<Vec<u8>, LandSurfaceEnergyShadowError> {
    let mut out = CanonicalCoveredPhysicalWriterV1::new(schema);
    out.usize(candidates.len());
    for candidate in candidates {
        out.soil_candidate(candidate)?;
    }
    Ok(out.finish())
}

#[cfg(test)]
struct CanonicalCoveredPhysicalWriterV1 {
    bytes: Vec<u8>,
}

#[cfg(test)]
impl CanonicalCoveredPhysicalWriterV1 {
    fn new(schema: &[u8]) -> Self {
        Self {
            bytes: schema.to_vec(),
        }
    }

    fn finish(self) -> Vec<u8> {
        self.bytes
    }

    fn usize(&mut self, value: usize) {
        self.bytes.extend_from_slice(&(value as u64).to_be_bytes());
    }

    fn u128(&mut self, value: u128) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    fn string(&mut self, value: &str) {
        self.usize(value.len());
        self.bytes.extend_from_slice(value.as_bytes());
    }

    fn f64(&mut self, value: f64) {
        self.bytes.extend_from_slice(&value.to_bits().to_be_bytes());
    }

    fn f64s(&mut self, values: &[f64]) {
        self.usize(values.len());
        for value in values {
            self.f64(*value);
        }
    }

    fn option_f64(&mut self, value: Option<f64>) {
        self.bytes.push(u8::from(value.is_some()));
        if let Some(value) = value {
            self.f64(value);
        }
    }

    fn option_string(&mut self, value: Option<&str>) {
        self.bytes.push(u8::from(value.is_some()));
        if let Some(value) = value {
            self.string(value);
        }
    }

    fn json<T: serde::Serialize>(&mut self, value: &T) -> Result<(), LandSurfaceEnergyShadowError> {
        let bytes = serde_json::to_vec(value).map_err(|_| {
            LandSurfaceEnergyShadowError::Identity("covered physical projection serialization")
        })?;
        self.usize(bytes.len());
        self.bytes.extend_from_slice(&bytes);
        Ok(())
    }

    fn runtime_identity(&mut self, value: &openwepp_land_surface_energy::RuntimeTileIdentity) {
        self.u128(value.transaction_id.0);
        self.u128(value.soil_thermal_transaction_id.0);
        for field in [
            value.lse_owner_id.as_str(),
            value.hydrology_owner_id.as_str(),
            value.soil_thermal_owner_id.as_str(),
            value.vegetation_owner_id.as_str(),
            value.biogeochemistry_owner_id.as_str(),
            value.configuration_sha256.as_str(),
            value.beginning_lse_state_sha256.as_str(),
            value.beginning_hydrology_snapshot_sha256.as_str(),
            value.beginning_soil_thermal_state_sha256.as_str(),
            value.beginning_vegetation_state_sha256.as_str(),
            value.beginning_biogeochemistry_state_sha256.as_str(),
            value.ofe_id.as_str(),
            value.tile_id.as_str(),
            value.surface_id.as_str(),
            value.ground_source_id.as_str(),
        ] {
            self.string(field);
        }
        self.bytes.push(match value.surface_class {
            openwepp_land_surface_energy::SurfaceClass::BareMineralSoil => 0,
            openwepp_land_surface_energy::SurfaceClass::ForestLitter => 1,
        });
        self.bytes.push(match value.ground_source_type {
            openwepp_land_surface_energy::WaterSourceType::SurfaceLiquid => 0,
            openwepp_land_surface_energy::WaterSourceType::LitterLiquid => 1,
            openwepp_land_surface_energy::WaterSourceType::SoilLayerLiquid => 2,
        });
        self.option_string(
            value
                .ground_source_tile_id
                .as_ref()
                .map(|value| value.as_str()),
        );
        self.option_string(
            value
                .ground_soil_layer_id
                .as_ref()
                .map(|value| value.as_str()),
        );
        self.f64s(&[value.tile_fraction, value.interval_s]);
    }

    fn potential_request_batch(
        &mut self,
        value: &openwepp_land_surface_energy::PotentialWaterRequestBatch,
    ) -> Result<(), LandSurfaceEnergyShadowError> {
        value.validate()?;
        self.u128(value.transaction_id.0);
        self.string(value.beginning_lse_state_sha256.as_str());
        self.json(&value.requests)?;
        self.string(value.potential_signature_sha256.as_str());
        Ok(())
    }

    fn band(&mut self, value: openwepp_land_surface_energy::BandDirectionalFluxes) {
        self.f64s(&[
            value.direct_vis,
            value.diffuse_vis,
            value.direct_nir,
            value.diffuse_nir,
        ]);
    }

    fn ground_water(
        &mut self,
        value: openwepp_land_surface_energy::GroundWaterFlux,
    ) -> Result<(), LandSurfaceEnergyShadowError> {
        self.f64s(&[
            value.law_kg_m2_tile_s,
            value.final_kg_m2_tile_s,
            value.request_kg_m2_stand_ground,
            value.finalized_use_kg_m2_stand_ground,
            value.condensation_credit_kg_m2_stand_ground,
        ]);
        self.option_f64(value.authorization_kg_m2_stand_ground);
        self.json(&value.branch)
    }

    fn source_water(
        &mut self,
        value: &openwepp_land_surface_energy::SourceWaterFlux,
    ) -> Result<(), LandSurfaceEnergyShadowError> {
        self.string(&value.occupancy_id);
        self.string(&value.layer_id);
        self.f64s(&[
            value.law_kg_m2_tile_s,
            value.final_kg_m2_tile_s,
            value.request_kg_m2_stand_ground,
            value.finalized_use_kg_m2_stand_ground,
        ]);
        self.option_f64(value.authorization_kg_m2_stand_ground);
        self.json(&value.branch)
    }

    fn covered_candidate(
        &mut self,
        value: &openwepp_land_surface_energy::CoveredColumnCandidate,
    ) -> Result<(), LandSurfaceEnergyShadowError> {
        self.f64s(&value.solution);
        self.f64s(&value.evaluation.raw_residuals);
        self.f64s(&value.evaluation.normalized_residuals);
        self.f64s(&value.evaluation.tolerances);
        self.usize(value.evaluation.occupancies.len());
        for occupancy in &value.evaluation.occupancies {
            self.f64s(&occupancy.residuals);
            self.f64s(&occupancy.tolerances);
            self.usize(occupancy.source_water.len());
            for source in &occupancy.source_water {
                self.source_water(source)?;
            }
            self.f64s(&[
                occupancy.canopy_sensible_w_m2,
                occupancy.canopy_vapor_kg_m2_s,
                occupancy.wet_vapor_kg_m2_s,
            ]);
            self.json(&occupancy.wet_branch)?;
            self.f64s(&occupancy.component_temperatures_k);
            self.f64s(&occupancy.ci_pa);
            for branch in occupancy.gas_branches {
                self.bytes.push(match branch {
                    openwepp_land_surface_energy::V10LeafGasBranch::Inactive => 0,
                    openwepp_land_surface_energy::V10LeafGasBranch::ExactZeroPar => 1,
                    openwepp_land_surface_energy::V10LeafGasBranch::RespirationDominated => 2,
                    openwepp_land_surface_energy::V10LeafGasBranch::PositiveAssimilation => 3,
                });
            }
            self.f64s(&occupancy.gross_assimilation_umol_co2_m2_leaf_s);
            self.f64s(&occupancy.net_assimilation_umol_co2_m2_leaf_s);
            self.f64s(&occupancy.dark_respiration_umol_co2_m2_leaf_s);
            self.f64s(&occupancy.emax_kg_m2_s);
            self.json(&occupancy.liquid)?;
            for fields in [
                &occupancy.absorbed_shortwave_w_m2,
                &occupancy.net_longwave_w_m2,
                &occupancy.sensible_to_canopy_air_w_m2,
                &occupancy.signed_vapor_to_canopy_air_kg_m2_s,
                &occupancy.component_areas_m2_m2_tile,
                &occupancy.component_emissive_areas_m2_m2_tile,
                &occupancy.component_heat_conductance_m_s_tile,
                &occupancy.component_vapor_conductance_m_s_tile,
                &occupancy.component_surface_specific_humidity_kg_kg,
            ] {
                self.f64s(fields);
            }
            for authorization in occupancy.component_vapor_authorization_kg_m2_tile_s {
                self.option_f64(authorization);
            }
        }
        self.f64s(&[
            value.evaluation.canopy_air_temperature_k,
            value.evaluation.canopy_air_specific_humidity_kg_kg,
            value.evaluation.ground_temperature_k,
        ]);
        self.f64s(&value.evaluation.soil_temperature_k);
        self.ground_water(value.evaluation.ground_water)?;
        self.f64s(&value.evaluation.ground_heat_cn_w_m2_tile);
        self.f64s(&[
            value.evaluation.ground_storage_w_m2_tile,
            value.evaluation.ending_surface_enthalpy_j_m2_tile,
        ]);
        let longwave = &value.evaluation.whole_column_longwave;
        self.f64s(&longwave.transmissivities);
        self.f64s(&longwave.downward_boundaries_w_m2);
        self.f64s(&longwave.upward_boundaries_w_m2);
        self.usize(longwave.component_net_w_m2.len());
        for component in &longwave.component_net_w_m2 {
            self.f64s(component);
        }
        self.f64s(&[
            longwave.ground_net_w_m2,
            longwave.top_upward_w_m2,
            value.evaluation.ground_canopy_release_kg_m2_tile,
            value.evaluation.ground_stemflow_kg_m2_tile,
            value.evaluation.ground_sensible_to_canopy_air_w_m2,
            value.evaluation.lower_boundary_vapor_to_canopy_air_kg_m2_s,
            value.evaluation.canopy_sensible_w_m2,
            value.evaluation.canopy_vapor_kg_m2_s,
            value.evaluation.sensible_to_reference_air_w_m2,
            value.evaluation.vapor_to_reference_air_kg_m2_s,
            value.evaluation.shared_heat_residual_w_m2,
            value.evaluation.shared_heat_tolerance_w_m2,
            value.evaluation.shared_vapor_residual_kg_m2_s,
            value.evaluation.shared_vapor_tolerance_kg_m2_s,
            value.surface_enthalpy_j_m2_tile,
        ]);
        self.f64s(&value.soil_temperature_k);
        self.usize(value.root_water.len());
        for source in &value.root_water {
            self.source_water(source)?;
        }
        self.ground_water(value.ground_water)?;
        self.bytes
            .extend_from_slice(&value.iterations.to_be_bytes());
        self.bytes
            .extend_from_slice(&value.backtracking_count.to_be_bytes());
        self.f64s(&[
            value.step_norms.hydraulic_mm,
            value.step_norms.beta,
            value.step_norms.temperature_k,
            value.step_norms.humidity_kg_kg,
            value.step_norms.ci_pa,
        ]);
        Ok(())
    }

    fn open_candidate(
        &mut self,
        value: &AcceptedOpenSurface,
    ) -> Result<(), LandSurfaceEnergyShadowError> {
        self.f64s(&value.solution);
        let evaluation = &value.evaluation;
        self.f64s(&evaluation.raw_residuals);
        self.f64s(&evaluation.normalized_residuals);
        self.f64s(&evaluation.tolerances);
        self.f64(evaluation.surface_temperature_k);
        self.f64s(&evaluation.soil_temperature_k);
        self.band(evaluation.shortwave_absorbed_w_m2_tile);
        self.band(evaluation.shortwave_reflected_w_m2_tile);
        self.f64s(&[
            evaluation.longwave_net_w_m2_tile,
            evaluation.sensible_w_m2_tile,
            evaluation.vapor_energy_w_m2_tile,
            evaluation.surface_storage_w_m2_tile,
            evaluation.ending_surface_enthalpy_j_m2_tile,
        ]);
        self.f64s(&evaluation.ground_heat_cn_w_m2_tile);
        self.usize(evaluation.soil_thermal.len());
        for layer in &evaluation.soil_thermal {
            self.string(&layer.layer_id);
            self.f64s(&[
                layer.incoming_cn_w_m2,
                layer.outgoing_cn_w_m2,
                layer.storage_w_m2,
                layer.residual_w_m2,
            ]);
        }
        self.ground_water(evaluation.water)?;
        self.f64s(&[
            evaluation.neutral_resistances.heat_s_m,
            evaluation.neutral_resistances.vapor_s_m,
        ]);
        self.bytes
            .push(u8::from(evaluation.bare_soil_vapor.is_some()));
        if let Some(vapor) = evaluation.bare_soil_vapor {
            self.f64s(&[
                vapor.signed_flux_kg_m2_s,
                vapor.saturation,
                vapor.volumetric_liquid,
                vapor.matric_potential_mm,
                vapor.kelvin_factor,
                vapor.theta_air,
                vapor.dry_layer_m,
                vapor.pore_tortuosity,
                vapor.vapor_diffusivity_m2_s,
                vapor.soil_resistance_s_m,
                vapor.surface_specific_humidity_kg_kg,
            ]);
            self.bytes.push(u8::from(vapor.zero_flux_branch));
        }
        self.f64s(&[
            value.candidate.surface_enthalpy_j_m2_tile,
            value.candidate.surface_temperature_warm_start_k,
        ]);
        self.f64s(&value.candidate.soil_temperature_k);
        self.bytes
            .extend_from_slice(&value.iterations.to_be_bytes());
        self.bytes
            .extend_from_slice(&value.backtracking_count.to_be_bytes());
        self.f64(value.step_norm);
        Ok(())
    }

    fn soil_beginning(
        &mut self,
        value: &V8SoilThermalPhysicalBeginning,
    ) -> Result<(), LandSurfaceEnergyShadowError> {
        match value {
            V8SoilThermalPhysicalBeginning::V1(beginning) => {
                self.bytes.push(1);
                self.json(beginning)?;
            }
            V8SoilThermalPhysicalBeginning::V2(beginning) => {
                self.bytes.push(2);
                self.json(beginning.beginning_owner())?;
            }
            V8SoilThermalPhysicalBeginning::V2Unpublished(beginning) => {
                self.bytes.push(3);
                self.json(beginning.authority().beginning_owner())?;
                self.json(beginning.predecessor_trial().ending_state())?;
                self.u128(beginning.transaction_id().0);
                self.u128(beginning.support_start_ns());
                self.u128(beginning.support_end_ns());
            }
        }
        Ok(())
    }

    fn potential_tile(
        &mut self,
        value: &PotentialTilePhase,
    ) -> Result<(), LandSurfaceEnergyShadowError> {
        match value {
            PotentialTilePhase::Open {
                phase,
                final_initial_trial,
                soil_thermal,
            } => {
                self.bytes.push(0);
                self.runtime_identity(&phase.identity);
                self.open_surface_beginning(phase.test_support_beginning());
                self.open_candidate(&phase.accepted)?;
                self.potential_request_batch(&phase.request_batch)?;
                self.bytes.push(u8::from(final_initial_trial.is_some()));
                if let Some(trial) = final_initial_trial {
                    self.f64s(trial);
                }
                self.soil_beginning(soil_thermal)?;
            }
            PotentialTilePhase::Stage3OpenSnow {
                identity,
                beginning_state,
                soil_thermal,
                request_batch,
            } => {
                self.bytes.push(1);
                self.runtime_identity(identity);
                self.json(beginning_state)?;
                self.soil_beginning(soil_thermal)?;
                self.potential_request_batch(request_batch)?;
            }
            PotentialTilePhase::Covered {
                phase,
                final_initial_trial,
                soil_thermal,
            } => {
                self.bytes.push(2);
                self.runtime_identity(phase.identity());
                self.covered_beginning(phase.test_support_beginning())?;
                self.covered_candidate(phase.test_support_accepted())?;
                let roots = phase.test_support_root_identities();
                self.usize(roots.len());
                for ((solver_occupancy, solver_layer), root) in roots {
                    self.string(solver_occupancy);
                    self.string(solver_layer);
                    self.string(&root.solver_occupancy_id);
                    self.string(root.requesting_owner_id.as_str());
                    self.string(root.occupancy_id.as_str());
                    self.string(root.layer_id.as_str());
                    self.string(root.source_id.as_str());
                }
                let gas_branches = phase.test_support_gas_branches();
                self.usize(gas_branches.len());
                for branches in gas_branches {
                    for branch in branches {
                        self.bytes.push(match branch {
                            openwepp_land_surface_energy::V10LeafGasBranch::Inactive => 0,
                            openwepp_land_surface_energy::V10LeafGasBranch::ExactZeroPar => 1,
                            openwepp_land_surface_energy::V10LeafGasBranch::RespirationDominated => 2,
                            openwepp_land_surface_energy::V10LeafGasBranch::PositiveAssimilation => 3,
                        });
                    }
                }
                self.potential_request_batch(phase.request_batch())?;
                phase.potential_vegetation_operands.validate()?;
                self.json(&phase.potential_vegetation_operands)?;
                self.f64s(final_initial_trial);
                self.soil_beginning(soil_thermal)?;
            }
        }
        Ok(())
    }

    fn soil_candidate(
        &mut self,
        value: &SoilThermalTileCandidate,
    ) -> Result<(), LandSurfaceEnergyShadowError> {
        self.string(value.owner_id.as_str());
        self.string(value.beginning_state_sha256.as_str());
        match &value.beginning_identity {
            openwepp_land_surface_energy::SoilThermalCandidateBeginningIdentity::V1 {
                configuration_sha256,
                last_accepted_transaction_id,
            } => {
                self.bytes.push(1);
                self.string(configuration_sha256.as_str());
                self.bytes
                    .push(u8::from(last_accepted_transaction_id.is_some()));
                if let Some(transaction_id) = last_accepted_transaction_id {
                    self.u128(transaction_id.0);
                }
            }
            openwepp_land_surface_energy::SoilThermalCandidateBeginningIdentity::V2 {
                owner_tag,
                schema_sha256,
                exact_carry_definition_sha256,
                parent_v1_state_sha256,
                contract_version,
                model_version,
                model_definition_sha256,
                run_id,
                configuration_sha256,
                transaction_id,
                expected_predecessor_transaction_id,
                support_start_ns,
                support_end_ns,
                receipt_chain_sha256,
            } => {
                self.bytes.push(2);
                for field in [
                    owner_tag.as_str(),
                    schema_sha256.as_str(),
                    exact_carry_definition_sha256.as_str(),
                    parent_v1_state_sha256.as_str(),
                    model_version.as_str(),
                    model_definition_sha256.as_str(),
                    run_id.as_str(),
                    configuration_sha256.as_str(),
                    receipt_chain_sha256.as_str(),
                ] {
                    self.string(field);
                }
                self.bytes
                    .extend_from_slice(&contract_version.to_be_bytes());
                self.u128(transaction_id.0);
                self.bytes
                    .push(u8::from(expected_predecessor_transaction_id.is_some()));
                if let Some(predecessor) = expected_predecessor_transaction_id {
                    self.u128(predecessor.0);
                }
                self.u128(*support_start_ns);
                self.u128(*support_end_ns);
            }
        }
        self.string(value.ofe_id.as_str());
        self.string(value.tile_id.as_str());
        self.usize(value.layers.len());
        for layer in &value.layers {
            self.string(layer.layer_id.as_str());
            self.f64s(&[
                layer.beginning_enthalpy_j_m2_ofe_ground,
                layer.ground_heat_credit_j_m2_ofe_ground,
                layer.infiltration_enthalpy_credit_j_m2_ofe_ground,
                layer.ending_enthalpy_j_m2_ofe_ground,
                layer.ending_temperature_k,
            ]);
            self.json(&layer.beginning_enthalpy_carry)?;
        }
        Ok(())
    }

    fn surface_energy(&mut self, value: openwepp_land_surface_energy::SurfaceEnergyOperands) {
        self.f64s(&[
            value.absorbed_shortwave_w_m2,
            value.net_longwave_w_m2,
            value.sensible_w_m2,
            value.signed_vapor_kg_m2_s,
            value.surface_temperature_k,
            value.ground_heat_w_m2,
            value.storage_w_m2,
        ]);
    }

    fn tile_energy(&mut self, value: &openwepp_land_surface_energy::TileEnergyOperandSet) {
        self.surface_energy(value.surface);
        self.f64s(&[
            value.latent.signed_vapor_kg_m2_s,
            value.latent.interval_s,
            value.latent.surface_temperature_k,
            value.latent.signed_water_amount_kg_m2,
            value.latent.vapor_energy_j_m2,
        ]);
        self.usize(value.ground_heat.len());
        for ground in &value.ground_heat {
            self.f64s(&[ground.surface_debit_j_m2, ground.soil_credit_j_m2]);
        }
    }

    fn covered_energy(
        &mut self,
        value: &openwepp_land_surface_energy::CoveredTileEnergyOperandSet,
    ) -> Result<(), LandSurfaceEnergyShadowError> {
        self.bytes.push(match value.authority {
            openwepp_land_surface_energy::CoveredColumnAuthority::HistoricalV8 => 0,
            openwepp_land_surface_energy::CoveredColumnAuthority::V10NonpositiveAssimilation => 1,
            openwepp_land_surface_energy::CoveredColumnAuthority::V11SnowCovered => 2,
        });
        match &value.lower_boundary {
            openwepp_land_surface_energy::CoveredLowerBoundaryEnergyOperands::SnowFree(ground) => {
                self.bytes.push(0);
                self.tile_energy(ground);
            }
            openwepp_land_surface_energy::CoveredLowerBoundaryEnergyOperands::Stage3SnowCovered(
                stage3,
            ) => {
                self.bytes.push(1);
                self.string(stage3.optical.ofe_id.as_str());
                self.string(stage3.optical.tile_id.as_str());
                self.band(stage3.optical.terminal_w_m2_tile);
                self.band(stage3.optical.absorbed_w_m2_tile);
                self.band(stage3.optical.reflected_w_m2_tile);
                self.f64s(&[
                    stage3.optical.snow_vis_albedo,
                    stage3.optical.snow_nir_albedo,
                    stage3.snow_temperature_k,
                    stage3.vapor_to_canopy_air_kg_m2_tile_s,
                    stage3.interval_s,
                    stage3.latent_heat_j_kg,
                    stage3.latent_energy_to_canopy_air_j_m2_tile,
                    stage3.sensible_to_canopy_air_w_m2_tile,
                    stage3.net_longwave_w_m2_tile,
                    stage3.precipitation_advection_w_m2_tile,
                    stage3.boundary_energy_w_m2_tile,
                ]);
                for digest in [
                    Some(&stage3.optical.stage3_albedo_state_sha256),
                    Some(&stage3.optical.forcing_receipt_sha256),
                    Some(&stage3.optical.receipt_sha256),
                    Some(&stage3.carrier_receipt_id),
                    stage3.optical_receipt_sha256.as_ref(),
                    stage3.reciprocal_longwave_receipt_sha256.as_ref(),
                    stage3.final_canopy_boundary_receipt_sha256.as_ref(),
                ] {
                    self.option_string(digest.map(|value| value.as_str()));
                }
            }
        }
        let column = &value.column;
        self.usize(column.occupancies.len());
        for occupancy in &column.occupancies {
            self.string(&occupancy.occupancy_id);
            for surface in [
                occupancy.sun_leaf,
                occupancy.shade_leaf,
                occupancy.wet_surface,
                occupancy.dry_stem,
            ] {
                self.f64s(&[
                    surface.surface_area_m2_m2_tile,
                    surface.emissive_area_m2_m2_tile,
                    surface.heat_conductance_m_s_tile,
                    surface.vapor_conductance_m_s_tile,
                    surface.surface_specific_humidity_kg_kg,
                    surface.absorbed_shortwave_w_m2_tile,
                    surface.net_longwave_w_m2_tile,
                    surface.sensible_to_canopy_air_w_m2_tile,
                    surface.signed_vapor_to_canopy_air_kg_m2_tile_s,
                    surface.surface_temperature_k,
                    surface.latent_heat_j_kg,
                ]);
                self.option_f64(surface.vapor_authorization_kg_m2_tile_s);
            }
        }
        let air = column.canopy_air;
        self.f64s(&[
            air.rho_air_kg_m3,
            air.cp_air_j_kg_k,
            air.canopy_air_temperature_k,
            air.canopy_air_specific_humidity_kg_kg,
            air.ground_sensible_to_canopy_air_w_m2_tile,
            air.ground_vapor_to_canopy_air_kg_m2_tile_s,
            air.canopy_sensible_w_m2_tile,
            air.canopy_vapor_kg_m2_tile_s,
            air.sensible_to_reference_air_w_m2_tile,
            air.vapor_to_reference_air_kg_m2_tile_s,
            air.shared_heat_residual_w_m2_tile,
            air.shared_heat_tolerance_w_m2_tile,
            air.shared_vapor_residual_kg_m2_tile_s,
            air.shared_vapor_tolerance_kg_m2_tile_s,
        ]);
        for band in [
            column.shortwave.incident_w_m2_tile,
            column.shortwave.top_reflected_w_m2_tile,
            column.shortwave.ground_absorbed_by_incident_w_m2_tile,
            column.shortwave.ground_terminal_w_m2_tile,
            column.shortwave.ground_absorbed_w_m2_tile,
            column.shortwave.ground_reflected_w_m2_tile,
        ] {
            self.band(band);
        }
        self.usize(column.shortwave.occupancies.len());
        for occupancy in &column.shortwave.occupancies {
            self.string(&occupancy.occupancy_id);
            self.band(occupancy.sun_leaf_absorbed_w_m2_tile);
            self.band(occupancy.shade_leaf_absorbed_w_m2_tile);
            self.band(occupancy.stem_absorbed_w_m2_tile);
        }
        self.f64s(&[
            column.longwave.atmospheric_downward_w_m2_tile,
            column.longwave.top_upward_w_m2_tile,
            column.longwave.ground_net_w_m2_tile,
        ]);
        self.f64s(&column.longwave.transmissivities);
        self.f64s(&column.longwave.downward_boundaries_w_m2_tile);
        self.f64s(&column.longwave.upward_boundaries_w_m2_tile);
        self.usize(column.longwave.occupancy_component_net_w_m2_tile.len());
        for (identity, values) in &column.longwave.occupancy_component_net_w_m2_tile {
            self.string(identity);
            self.f64s(values);
        }
        self.f64(column.stage3_lower_boundary_energy_w_m2_tile);
        for digest in [
            column.optical_receipt_sha256.as_ref(),
            column.reciprocal_longwave_receipt_sha256.as_ref(),
            column.final_canopy_boundary_receipt_sha256.as_ref(),
        ] {
            self.option_string(digest.map(|value| value.as_str()));
        }
        Ok(())
    }

    fn final_covered_tile(
        &mut self,
        value: &openwepp_land_surface_energy::FinalCoveredTileCandidate,
    ) -> Result<(), LandSurfaceEnergyShadowError> {
        self.u128(value.transaction_id.0);
        self.runtime_identity(&value.identity);
        self.covered_candidate(&value.final_solver_candidate)?;
        self.json(&value.water_protocol)?;
        self.json(&value.ending_tile_state_pre_ingress)?;
        self.soil_candidate(&value.soil_thermal)?;
        self.covered_energy(&value.energy_operands)?;
        self.json(&value.diagnostics)?;
        self.json(&value.rollback_hashes)?;
        value.vegetation_operands.validate()?;
        self.json(&value.vegetation_operands)?;
        Ok(())
    }

    fn finalized_tile(
        &mut self,
        value: &FinalizedRuntimeTile,
    ) -> Result<(), LandSurfaceEnergyShadowError> {
        match value {
            FinalizedRuntimeTile::Open(value) => {
                self.bytes.push(0);
                self.u128(value.transaction_id.0);
                self.runtime_identity(&value.identity);
                self.open_candidate(&value.final_solver_candidate)?;
                self.json(&value.water_protocol)?;
                self.json(&value.ending_tile_state_pre_ingress)?;
                self.soil_candidate(&value.soil_thermal)?;
                self.tile_energy(&value.energy_operands);
                self.json(&value.diagnostics)?;
                self.json(&value.rollback_hashes)?;
            }
            FinalizedRuntimeTile::Stage3OpenSnow {
                identity,
                ending_tile_state_pre_ingress,
                soil_thermal,
                water_protocol,
                rollback_hashes,
            } => {
                self.bytes.push(1);
                self.runtime_identity(identity);
                self.json(ending_tile_state_pre_ingress)?;
                self.soil_candidate(soil_thermal)?;
                self.json(water_protocol)?;
                self.json(rollback_hashes)?;
            }
            FinalizedRuntimeTile::Covered(value) => {
                self.bytes.push(2);
                self.final_covered_tile(value)?;
            }
        }
        Ok(())
    }

    fn covered_beginning(
        &mut self,
        value: &openwepp_land_surface_energy::CoveredColumnInputs,
    ) -> Result<(), LandSurfaceEnergyShadowError> {
        self.bytes.push(match value.authority {
            openwepp_land_surface_energy::CoveredColumnAuthority::HistoricalV8 => 0,
            openwepp_land_surface_energy::CoveredColumnAuthority::V10NonpositiveAssimilation => 1,
            openwepp_land_surface_energy::CoveredColumnAuthority::V11SnowCovered => 2,
        });
        self.f64s(&[
            value.interval_s,
            value.tile_fraction,
            value.pressure_pa,
            value.air_temperature_k,
            value.air_specific_humidity_kg_kg,
            value.reference_wind_m_s,
            value.atmospheric_downward_longwave_w_m2,
            value.ca_pa,
            value.canopy_to_atmosphere_heat_resistance_s_m,
            value.canopy_to_atmosphere_vapor_resistance_s_m,
            value.latent_heat_j_kg,
            value.top_rain_kg_m2_tile,
            value.under_canopy_geometry.canopy_height_m,
            value.under_canopy_geometry.canopy_roughness_m,
            value.under_canopy_geometry.reference_height_m,
            value.under_canopy_geometry.leaf_area_index,
        ]);
        self.open_surface_beginning(&value.ground);
        self.usize(value.occupancies.len());
        for occupancy in &value.occupancies {
            self.string(&occupancy.occupancy_id);
            self.f64s(&[
                occupancy.medlyn_g1_kpa_sqrt,
                occupancy.g0_umol_m2_s,
                occupancy.sun.leaf_area_m2_m2_tile,
                occupancy.sun.absorbed_shortwave_w_m2_tile,
                occupancy.sun.absorbed_par_w_m2_leaf,
                occupancy.sun.vcmax25,
                occupancy.sun.jmax25,
                occupancy.sun.rd25,
                occupancy.shade.leaf_area_m2_m2_tile,
                occupancy.shade.absorbed_shortwave_w_m2_tile,
                occupancy.shade.absorbed_par_w_m2_leaf,
                occupancy.shade.vcmax25,
                occupancy.shade.jmax25,
                occupancy.shade.rd25,
                occupancy.biochemical.ha_vcmax_j_mol,
                occupancy.biochemical.hd_vcmax_j_mol,
                occupancy.biochemical.entropy_vcmax_j_mol_k,
                occupancy.biochemical.ha_jmax_j_mol,
                occupancy.biochemical.hd_jmax_j_mol,
                occupancy.biochemical.entropy_jmax_j_mol_k,
                occupancy.biochemical.kc25_pa,
                occupancy.biochemical.ha_kc_j_mol,
                occupancy.biochemical.ko25_pa,
                occupancy.biochemical.ha_ko_j_mol,
                occupancy.biochemical.gamma25_pa,
                occupancy.biochemical.ha_gamma_j_mol,
                occupancy.biochemical.oxygen_partial_pressure_pa,
                occupancy.biochemical.tp_vcmax_ratio,
                occupancy.biochemical.electron_quantum_yield,
                occupancy.biochemical.par_photon_umol_per_j,
                occupancy.biochemical.electron_curvature,
                occupancy.biochemical.ac_aj_curvature,
                occupancy.biochemical.ag_ap_curvature,
                occupancy.stem_area_m2_m2_tile,
                occupancy.stem_absorbed_shortwave_w_m2_tile,
                occupancy.beginning_canopy_liquid_kg_m2_tile,
                occupancy.liquid_interception_fraction,
                occupancy.liquid_capacity_kg_m2_plant,
                occupancy.stemflow_fraction,
                occupancy.gb_leaf_m_s,
                occupancy.gb_wet_m_s,
                occupancy.gb_stem_m_s,
                occupancy.lai,
                occupancy.sai,
                occupancy.clumping_index,
                occupancy.k1_sun_max_s1,
                occupancy.k1_shade_max_s1,
                occupancy.k2_max,
                occupancy.k3_max_m_s,
                occupancy.height_m,
                occupancy.root_to_leaf_area,
                occupancy.p50_leaf_mm,
                occupancy.p50_xylem_mm,
                occupancy.p50_root_mm,
                occupancy.vulnerability_exponent,
            ]);
            self.usize(occupancy.root_layers.len());
            for root in &occupancy.root_layers {
                self.string(&root.layer_id);
                self.bytes.push(u8::from(root.accessible));
                self.bytes.push(u8::from(root.frozen));
                self.f64s(&[
                    root.root_fraction,
                    root.soil_potential_mm,
                    root.gravity_head_mm,
                    root.z3_m,
                    root.dxroot_m,
                    root.ksoil_m2_s,
                ]);
            }
        }
        for band in [
            value.shortwave.incident_w_m2_tile,
            value.shortwave.top_reflected_w_m2_tile,
            value.shortwave.ground_absorbed_by_incident_w_m2_tile,
        ] {
            self.band(band);
        }
        self.usize(value.shortwave.occupancies.len());
        for occupancy in &value.shortwave.occupancies {
            self.string(&occupancy.occupancy_id);
            self.band(occupancy.sun_leaf_absorbed_w_m2_tile);
            self.band(occupancy.shade_leaf_absorbed_w_m2_tile);
            self.band(occupancy.stem_absorbed_w_m2_tile);
        }
        self.bytes
            .push(u8::from(value.stage3_lower_boundary.is_some()));
        if let Some(boundary) = &value.stage3_lower_boundary {
            self.stage3_lower_boundary(boundary);
        }
        self.bytes.push(u8::from(value.stage3_optical.is_some()));
        if let Some(optical) = &value.stage3_optical {
            self.stage3_optical(optical);
        }
        Ok(())
    }

    fn stage3_lower_boundary(
        &mut self,
        value: &openwepp_land_surface_energy::Stage3SnowCoveredLowerBoundary,
    ) {
        self.f64s(&[
            value.snow_temperature_k,
            value.latent_heat_j_kg,
            value.sensible_to_canopy_air_w_m2,
            value.vapor_to_canopy_air_kg_m2_s,
            value.net_longwave_w_m2,
            value.shortwave_absorbed_w_m2,
            value.precipitation_advection_w_m2,
            value.snow_vis_albedo,
            value.snow_nir_albedo,
        ]);
        for digest in [
            Some(&value.carrier_receipt_id),
            Some(&value.stage3_albedo_state_sha256),
            Some(&value.forcing_receipt_sha256),
            value.optical_receipt_sha256.as_ref(),
            value.reciprocal_longwave_receipt_sha256.as_ref(),
            value.final_canopy_boundary_receipt_sha256.as_ref(),
        ] {
            self.option_string(digest.map(|digest| digest.as_str()));
        }
    }

    fn stage3_optical(
        &mut self,
        value: &openwepp_land_surface_energy::Stage3SnowOpticalBoundaryReceiptV1,
    ) {
        self.string(value.ofe_id.as_str());
        self.string(value.tile_id.as_str());
        self.band(value.terminal_w_m2_tile);
        self.band(value.absorbed_w_m2_tile);
        self.band(value.reflected_w_m2_tile);
        self.f64s(&[value.snow_vis_albedo, value.snow_nir_albedo]);
        for digest in [
            &value.stage3_albedo_state_sha256,
            &value.forcing_receipt_sha256,
            &value.receipt_sha256,
        ] {
            self.string(digest.as_str());
        }
    }

    fn open_surface_beginning(&mut self, value: &openwepp_land_surface_energy::OpenSurfaceProblem) {
        self.f64s(&[
            value.interval_s,
            value.tile_fraction,
            value.surface_vis_albedo,
            value.surface_nir_albedo,
            value.surface_emissivity,
            value.surface_depth_m,
            value.surface_conductivity_w_m_k,
            value.surface_dry_heat_capacity_j_m2_k,
            value.open_geometry.reference_height_m,
            value.open_geometry.roughness_momentum_m,
            value.open_geometry.roughness_heat_m,
            value.open_geometry.roughness_vapor_m,
            value.air_temperature_k,
            value.air_specific_humidity_kg_kg,
            value.air_pressure_pa,
            value.reference_wind_m_s,
            value.atmospheric_downward_longwave_w_m2,
            value.surface_liquid_kg_m2_tile,
            value.surface_enthalpy_j_m2_tile,
            value.surface_temperature_warm_start_k,
        ]);
        self.band(value.terminal_shortwave_w_m2_tile);
        self.bytes.push(match value.class {
            openwepp_land_surface_energy::SurfaceClassKind::BareMineralSoil => 0,
            openwepp_land_surface_energy::SurfaceClassKind::ForestLitter => 1,
        });
        self.bytes.push(match value.storage_branch {
            openwepp_land_surface_energy::SurfaceStorageBranch::FiniteCapacity => 0,
            openwepp_land_surface_energy::SurfaceStorageBranch::EquilibriumZero => 1,
        });
        self.option_f64(value.litter_capacity_kg_m2_tile);
        self.bytes.push(u8::from(value.bare_soil.is_some()));
        if let Some(bare) = value.bare_soil {
            self.f64s(&[
                bare.top_layer_liquid_kg_m2,
                bare.top_layer_ice_kg_m2,
                bare.porosity,
                bare.saturated_matric_potential_mm,
                bare.clapp_hornberger_b,
                bare.theta_initial,
            ]);
        }
        self.usize(value.soil_nodes.len());
        for node in &value.soil_nodes {
            self.string(&node.layer_id);
            self.f64s(&[
                node.depth_m,
                node.conductivity_w_m_k,
                node.heat_capacity_j_m2_k,
                node.beginning_temperature_k,
            ]);
        }
    }
}

/// Execute all strictly projected open and covered tiles against one real
/// hydrology authorization. Covered ingress is constructed only from the
/// accepted fixed-cap E04 ledgers by the derived-ingress owner boundary.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub(crate) fn execute_multi_tile_runtime(
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
    receiver_expectations: &UnifiedReceiverExpectations,
    projected_tiles: Vec<StrictProjectedTileProblem>,
    soil_sources: &BTreeMap<GroundWaterKey, RealHydrologySourceKey>,
    ingress_schedule: &CoveredIngressSchedule,
    failure_hook: FailureHook<'_>,
    pending_hook: PendingEnvelopeHook<'_>,
) -> Result<MultiTileRuntimeResult, LandSurfaceEnergyShadowError> {
    execute_multi_tile_runtime_with_mode(
        soil_adapter,
        surface_configuration,
        receiver_expectations,
        projected_tiles,
        soil_sources,
        ingress_schedule,
        failure_hook,
        pending_hook,
        true,
    )
}

/// Execute a candidate physical endpoint while retaining local/component
/// closures but deferring the aggregate OFE energy gate. This is only for the
/// uncommitted first pass of the covered longwave fixed point; callers must
/// rerun the final pass with `execute_multi_tile_runtime` before acceptance.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub(crate) fn execute_multi_tile_runtime_provisional(
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
    receiver_expectations: &UnifiedReceiverExpectations,
    projected_tiles: Vec<StrictProjectedTileProblem>,
    soil_sources: &BTreeMap<GroundWaterKey, RealHydrologySourceKey>,
    ingress_schedule: &CoveredIngressSchedule,
    failure_hook: FailureHook<'_>,
    pending_hook: PendingEnvelopeHook<'_>,
) -> Result<MultiTileRuntimeResult, LandSurfaceEnergyShadowError> {
    execute_multi_tile_runtime_with_mode(
        soil_adapter,
        surface_configuration,
        receiver_expectations,
        projected_tiles,
        soil_sources,
        ingress_schedule,
        failure_hook,
        pending_hook,
        false,
    )
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn execute_multi_tile_runtime_with_mode(
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
    receiver_expectations: &UnifiedReceiverExpectations,
    projected_tiles: Vec<StrictProjectedTileProblem>,
    soil_sources: &BTreeMap<GroundWaterKey, RealHydrologySourceKey>,
    ingress_schedule: &CoveredIngressSchedule,
    failure_hook: FailureHook<'_>,
    pending_hook: PendingEnvelopeHook<'_>,
    validate_ofe_energy: bool,
) -> Result<MultiTileRuntimeResult, LandSurfaceEnergyShadowError> {
    use crate::snow_stage3_v11_attachment::{
        begin_adaptive_parent_fixed_point_phase_v1 as profile_start,
        record_adaptive_parent_profile_detail_v1 as profile_record,
    };

    let topology_started = profile_start();
    let projected_tiles =
        validate_and_sort_projected_tiles(projected_tiles, surface_configuration)?;
    profile_record("physical topology", topology_started);
    let potential_started = profile_start();
    let potential_phases = solve_all_potential(projected_tiles, failure_hook)?;
    #[cfg(test)]
    crate::v9_real_consumer_shadow::record_snow_free_vapor_operation_v1();
    profile_record("physical potential", potential_started);
    let request_started = profile_start();
    let request_batch = combined_request_batch(&potential_phases)?;
    publish_pending_debug(
        pending_hook,
        PendingPayloadKind::CombinedRequest,
        &request_batch,
    )?;
    run_failure_hook(failure_hook, MultiTileFailurePhase::CombinedRequests)?;
    profile_record("physical request", request_started);
    let mut retained_final_tiles = None;

    let unified_started = profile_start();
    let hydrology_candidate = super::covered_derived_ingress::execute_unified_with_derived_ingress(
        soil_adapter,
        surface_configuration,
        receiver_expectations,
        &request_batch,
        soil_sources,
        ingress_schedule,
        |authorizations| {
            let final_tile_started = profile_start();
            publish_pending(
                pending_hook,
                PendingPayloadKind::Authorization,
                authorizations,
            )?;
            run_failure_hook(failure_hook, MultiTileFailurePhase::Authorization)?;
            let final_tiles = finalize_all_tiles(
                &potential_phases,
                authorizations,
                failure_hook,
                pending_hook,
            )?;
            #[cfg(test)]
            crate::v9_real_consumer_shadow::record_snow_free_phase_operation_v1();
            profile_record("physical final tile", final_tile_started);
            let protocol_started = profile_start();
            let protocol = combined_protocol(&request_batch, authorizations, &final_tiles)?;
            publish_pending(pending_hook, PendingPayloadKind::FinalProtocol, &protocol)?;
            let ending_tiles = final_tiles
                .iter()
                .map(|tile| tile.ending_tile_state_pre_ingress().clone())
                .collect();
            let soil_thermal = final_tiles
                .iter()
                .map(|tile| tile.soil_thermal().clone())
                .collect();
            let rollback_hashes = common_owner_rollback_hashes(&final_tiles)?;
            let sealed = UnifiedLseFinalization::try_new(
                receiver_expectations,
                protocol.clone(),
                ending_tiles,
                soil_thermal,
                rollback_hashes,
            )?;
            profile_record("physical protocol", protocol_started);
            let ingress_started = profile_start();
            let covered_final_tiles = final_tiles
                .iter()
                .filter_map(|tile| match tile {
                    FinalizedRuntimeTile::Open(_) | FinalizedRuntimeTile::Stage3OpenSnow { .. } => {
                        None
                    }
                    FinalizedRuntimeTile::Covered(value) => Some(value.clone()),
                })
                .collect::<Vec<_>>();
            let ingress = super::covered_derived_ingress::derive_fixed_cap_canopy_ingress(
                surface_configuration,
                &covered_final_tiles,
                ingress_schedule,
            )?;
            publish_pending(pending_hook, PendingPayloadKind::Ingress, &ingress)?;
            if ingress.tile_ingress.iter().any(|row| {
                matches!(
                    row,
                    crate::DirectTileGroundIngress::CoveredCanopyRelease { .. }
                        | crate::DirectTileGroundIngress::CoveredCanopyReleaseAndRunon { .. }
                )
            }) {
                run_failure_hook(failure_hook, MultiTileFailurePhase::E04Ingress)?;
            }
            if ingress.tile_ingress.iter().any(|row| {
                matches!(
                    row,
                    crate::DirectTileGroundIngress::OpenRawPrecipitation { .. }
                        | crate::DirectTileGroundIngress::OpenLiquidParcels { .. }
                )
            }) {
                run_failure_hook(failure_hook, MultiTileFailurePhase::OpenIngress)?;
            }
            retained_final_tiles = Some(final_tiles);
            profile_record("physical ingress", ingress_started);
            Ok((sealed, ingress))
        },
    )?;
    profile_record("physical unified", unified_started);
    let post_started = profile_start();
    run_failure_hook(failure_hook, MultiTileFailurePhase::UnifiedHydrology)?;

    let finalized_tiles = retained_final_tiles.ok_or(LandSurfaceEnergyShadowError::Identity(
        "multi-tile finalizer returned no tile candidates",
    ))?;
    for tile in &finalized_tiles {
        tile.energy_operands().validate()?;
    }
    run_failure_hook(failure_hook, MultiTileFailurePhase::LocalEnergy)?;
    let weighted_ofe_energy = if validate_ofe_energy {
        reconstruct_weighted_ofe_energy(&finalized_tiles)?
    } else {
        Vec::new()
    };
    run_failure_hook(failure_hook, MultiTileFailurePhase::OfeEnergy)?;
    profile_record("physical post", post_started);
    Ok(MultiTileRuntimeResult {
        potential_request_batch: request_batch,
        potential_tiles: potential_phases,
        finalized_tiles,
        weighted_ofe_energy,
        hydrology_candidate,
    })
}

fn validate_and_sort_projected_tiles(
    mut projected: Vec<StrictProjectedTileProblem>,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
) -> Result<Vec<StrictProjectedTileProblem>, LandSurfaceEnergyShadowError> {
    surface_configuration.validate()?;
    if projected.is_empty() {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "empty projected tile set",
        ));
    }
    let configured_rank = surface_configuration
        .records
        .iter()
        .enumerate()
        .map(|(rank, record)| {
            (
                (record.key.ofe_id.clone(), record.key.tile_id.clone()),
                rank,
            )
        })
        .collect::<BTreeMap<_, _>>();
    projected.sort_by_key(|tile| {
        configured_rank
            .get(&(
                tile.identity().ofe_id.clone(),
                tile.identity().tile_id.clone(),
            ))
            .copied()
            .unwrap_or(usize::MAX)
    });
    let first = projected[0].identity();
    let mut identities = BTreeSet::new();
    let mut fractions = BTreeMap::<OfeId, f64>::new();
    for tile in &projected {
        let identity = tile.identity();
        if identity.transaction_id != first.transaction_id
            || identity.configuration_sha256 != first.configuration_sha256
            || identity.beginning_lse_state_sha256 != first.beginning_lse_state_sha256
            || identity.beginning_hydrology_snapshot_sha256
                != first.beginning_hydrology_snapshot_sha256
            || identity.beginning_soil_thermal_state_sha256
                != first.beginning_soil_thermal_state_sha256
            || identity.lse_owner_id != first.lse_owner_id
            || identity.hydrology_owner_id != first.hydrology_owner_id
            || identity.soil_thermal_owner_id != first.soil_thermal_owner_id
            || identity.interval_s.to_bits() != first.interval_s.to_bits()
            || !identities.insert((identity.ofe_id.clone(), identity.tile_id.clone()))
        {
            return Err(LandSurfaceEnergyShadowError::Identity(
                "mixed or duplicate projected tile lineage",
            ));
        }
        let configured = surface_configuration
            .records
            .iter()
            .find(|record| {
                record.key.ofe_id == identity.ofe_id && record.key.tile_id == identity.tile_id
            })
            .ok_or(LandSurfaceEnergyShadowError::Identity(
                "projected tile absent from configured topology",
            ))?;
        if identity.hydrology_owner_id != surface_configuration.owner_id
            || identity.tile_fraction.to_bits() != configured.tile_fraction.to_bits()
            || identity.surface_id != configured.key.surface_id
            || identity.surface_class != configured.key.surface_class
            || identity.ground_source_type != configured.key.source_type
            || identity.ground_source_id != configured.key.source_id
            || identity.ground_source_tile_id.as_ref() != Some(&configured.key.tile_id)
            || identity.ground_soil_layer_id.is_some()
        {
            return Err(LandSurfaceEnergyShadowError::Identity(
                "projected/configured tile operand mismatch",
            ));
        }
        validate_projected_soil_order(tile, surface_configuration)?;
        *fractions.entry(identity.ofe_id.clone()).or_default() += identity.tile_fraction;
    }
    if fractions
        .values()
        .any(|sum| !canonical_tile_fraction_sum_closes(*sum))
    {
        return Err(LandSurfaceEnergyShadowError::Bound(
            "projected OFE tile fractions do not sum to one",
        ));
    }
    let configured = surface_configuration
        .records
        .iter()
        .map(|record| (record.key.ofe_id.clone(), record.key.tile_id.clone()))
        .collect::<BTreeSet<_>>();
    if configured != identities || configured.len() != surface_configuration.records.len() {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "projected/configured tile identity set",
        ));
    }
    Ok(projected)
}

fn validate_projected_soil_order(
    tile: &StrictProjectedTileProblem,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
) -> Result<(), LandSurfaceEnergyShadowError> {
    let identity = tile.identity();
    let snapshot = match tile {
        StrictProjectedTileProblem::Open(value) => &value.soil_thermal,
        StrictProjectedTileProblem::Stage3OpenSnow(value) => &value.soil_thermal,
        StrictProjectedTileProblem::Covered(value) => &value.soil_thermal,
    };
    snapshot
        .validate()
        .map_err(|_| LandSurfaceEnergyShadowError::Identity("projected soil beginning"))?;
    let binding = surface_configuration
        .ofe_bindings
        .iter()
        .find(|binding| binding.ofe_id == identity.ofe_id)
        .ok_or(LandSurfaceEnergyShadowError::Identity(
            "missing configured soil ordering",
        ))?;
    let projected_ofes = snapshot
        .ordered_ofes()
        .into_iter()
        .map(|ofe| ofe.ofe_id.clone())
        .collect::<Vec<_>>();
    let layers = snapshot
        .ordered_ofes()
        .into_iter()
        .find(|ofe| ofe.ofe_id == identity.ofe_id)
        .map(|ofe| {
            ofe.ordered_layers
                .iter()
                .map(|layer| layer.layer_id.clone())
                .collect::<Vec<_>>()
        });
    if snapshot.owner_id() != &identity.soil_thermal_owner_id
        || snapshot.state_sha256() != &identity.beginning_soil_thermal_state_sha256
        || projected_ofes != surface_configuration.ofe_topology
        || layers.as_ref() != Some(&binding.ordered_soil_layer_ids)
    {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "projected/configured soil thermal ordering",
        ));
    }
    Ok(())
}

fn solve_all_potential(
    projected: Vec<StrictProjectedTileProblem>,
    failure_hook: FailureHook<'_>,
) -> Result<Vec<PotentialTilePhase>, LandSurfaceEnergyShadowError> {
    projected
        .into_iter()
        .enumerate()
        .map(|(index, tile)| {
            let phase: PotentialTilePhase = match tile {
                StrictProjectedTileProblem::Open(value) => {
                    let phase = solve_open_potential_phase(
                        value.identity,
                        &value.beginning,
                        value.potential_initial_trial,
                    )?;
                    Ok::<_, LandSurfaceEnergyShadowError>(PotentialTilePhase::Open {
                        phase,
                        final_initial_trial: value.final_initial_trial,
                        soil_thermal: value.soil_thermal,
                    })
                }
                StrictProjectedTileProblem::Stage3OpenSnow(value) => {
                    let request_batch = PotentialWaterRequestBatch::try_new(
                        value.identity.transaction_id,
                        value.identity.beginning_lse_state_sha256.clone(),
                        vec![WaterAmount {
                            key: GroundWaterKey {
                                transaction_id: value.identity.transaction_id,
                                requesting_owner_id: value.identity.lse_owner_id.clone(),
                                requesting_component:
                                    openwepp_land_surface_energy::RequestingComponent::GroundSurface,
                                ofe_id: value.identity.ofe_id.clone(),
                                requesting_tile_id: value.identity.tile_id.clone(),
                                occupancy_id: None,
                                surface_id: Some(value.identity.surface_id.clone()),
                                surface_class: Some(value.identity.surface_class),
                                source_type: value.identity.ground_source_type,
                                source_id: value.identity.ground_source_id.clone(),
                                source_tile_id: value.identity.ground_source_tile_id.clone(),
                                soil_layer_id: value.identity.ground_soil_layer_id.clone(),
                                amount_basis:
                                    StandGroundWaterAmountBasis::KgH2oM2StandGroundInterval,
                            },
                            amount_kg_m2_stand_ground: 0.0,
                        }],
                    )?;
                    Ok::<_, LandSurfaceEnergyShadowError>(
                        PotentialTilePhase::Stage3OpenSnow {
                            identity: value.identity,
                            beginning_state: value.beginning_state,
                            soil_thermal: value.soil_thermal,
                            request_batch,
                        },
                    )
                }
                StrictProjectedTileProblem::Covered(value) => {
                    let phase = solve_covered_potential_phase(
                        value.identity,
                        &value.beginning,
                        value.roots,
                        value.potential_initial_trial,
                    )?;
                    Ok::<_, LandSurfaceEnergyShadowError>(PotentialTilePhase::Covered {
                        phase,
                        final_initial_trial: value.final_initial_trial,
                        soil_thermal: value.soil_thermal,
                    })
                }
            }?;
            run_failure_hook(failure_hook, MultiTileFailurePhase::PotentialTile(index))?;
            Ok(phase)
        })
        .collect()
}

fn combined_request_batch(
    phases: &[PotentialTilePhase],
) -> Result<PotentialWaterRequestBatch, LandSurfaceEnergyShadowError> {
    let first = phases
        .first()
        .ok_or(LandSurfaceEnergyShadowError::Identity(
            "empty potential tile set",
        ))?;
    let requests = phases
        .iter()
        .filter_map(PotentialTilePhase::request_batch)
        .flat_map(|batch| batch.requests.iter().cloned())
        .collect();
    Ok(PotentialWaterRequestBatch::try_new(
        first.identity().transaction_id,
        first.identity().beginning_lse_state_sha256.clone(),
        requests,
    )?)
}

fn authorization_subset(
    phase: &PotentialTilePhase,
    authorizations: &[WaterAuthorization],
) -> Result<Vec<WaterAuthorization>, LandSurfaceEnergyShadowError> {
    let keys = phase
        .request_batch()
        .into_iter()
        .flat_map(|batch| batch.requests.iter())
        .map(|row| row.key.clone())
        .collect::<BTreeSet<_>>();
    let subset = authorizations
        .iter()
        .filter(|row| keys.contains(&row.key))
        .cloned()
        .collect::<Vec<_>>();
    if subset.len() != keys.len() {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "incomplete tile authorization subset",
        ));
    }
    Ok(subset)
}

fn finalize_all_tiles(
    phases: &[PotentialTilePhase],
    authorizations: &[WaterAuthorization],
    failure_hook: FailureHook<'_>,
    pending_hook: PendingEnvelopeHook<'_>,
) -> Result<Vec<FinalizedRuntimeTile>, LandSurfaceEnergyShadowError> {
    phases
        .iter()
        .enumerate()
        .map(|(index, phase)| {
            let mut subset = authorization_subset(phase, authorizations)?;
            let finalized: FinalizedRuntimeTile = match phase {
                PotentialTilePhase::Open {
                    phase,
                    final_initial_trial,
                    soil_thermal,
                } => {
                    if subset.len() != 1 {
                        return Err(LandSurfaceEnergyShadowError::Identity(
                            "open tile authorization cardinality",
                        ));
                    }
                    Ok::<_, LandSurfaceEnergyShadowError>(FinalizedRuntimeTile::Open(
                        finalize_open_phase_with_soil_thermal_beginning(
                            phase,
                            &phase.identity.beginning_lse_state_sha256,
                            &subset.remove(0),
                            final_initial_trial.clone(),
                            soil_thermal.finalization_beginning(),
                        )?,
                    ))
                }
                PotentialTilePhase::Stage3OpenSnow {
                    identity,
                    beginning_state,
                    soil_thermal,
                    request_batch,
                } => {
                    if subset.len() != 1
                        || subset[0].amount_kg_m2_stand_ground.to_bits() != 0.0f64.to_bits()
                    {
                        return Err(LandSurfaceEnergyShadowError::Identity(
                            "Stage-3 open-snow pass-through authorization",
                        ));
                    }
                    let thermal =
                        openwepp_land_surface_energy::build_soil_thermal_passthrough_candidate(
                            identity,
                            soil_thermal.finalization_beginning(),
                        )?;
                    let water_protocol = WaterProtocol {
                        transaction_id: identity.transaction_id,
                        hydrology_owner_id: identity.hydrology_owner_id.clone(),
                        beginning_snapshot_sha256: identity
                            .beginning_hydrology_snapshot_sha256
                            .clone(),
                        requests: request_batch.requests.clone(),
                        authorizations: subset,
                        finalized_uses: request_batch.requests.clone(),
                        condensation_credits: Vec::new(),
                    };
                    water_protocol.validate()?;
                    Ok::<_, LandSurfaceEnergyShadowError>(FinalizedRuntimeTile::Stage3OpenSnow {
                        identity: identity.clone(),
                        ending_tile_state_pre_ingress: beginning_state.clone(),
                        soil_thermal: thermal,
                        water_protocol,
                        rollback_hashes: pass_through_rollback_hashes(identity),
                    })
                }
                PotentialTilePhase::Covered {
                    phase,
                    final_initial_trial,
                    soil_thermal,
                } => Ok::<_, LandSurfaceEnergyShadowError>(FinalizedRuntimeTile::Covered(
                    finalize_covered_phase_with_soil_thermal_beginning(
                        phase,
                        &phase.identity().beginning_lse_state_sha256,
                        subset,
                        final_initial_trial.clone(),
                        soil_thermal.finalization_beginning(),
                    )?,
                )),
            }?;
            if let Some(protocol) = finalized.water_protocol() {
                publish_pending(pending_hook, PendingPayloadKind::FinalTileUse, protocol)?;
            }
            run_failure_hook(failure_hook, MultiTileFailurePhase::FinalTile(index))?;
            Ok(finalized)
        })
        .collect()
}

fn run_failure_hook(
    hook: FailureHook<'_>,
    phase: MultiTileFailurePhase,
) -> Result<(), LandSurfaceEnergyShadowError> {
    if let Some(hook) = hook {
        hook(phase)?;
    }
    Ok(())
}

fn publish_pending<T: serde::Serialize + ?Sized>(
    hook: PendingEnvelopeHook<'_>,
    kind: PendingPayloadKind,
    value: &T,
) -> Result<(), LandSurfaceEnergyShadowError> {
    if let Some(hook) = hook {
        let bytes = serde_json::to_vec(value).map_err(|_| {
            LandSurfaceEnergyShadowError::Identity("pending envelope serialization")
        })?;
        hook(kind, &bytes)?;
    }
    Ok(())
}

fn publish_pending_debug<T: std::fmt::Debug + ?Sized>(
    hook: PendingEnvelopeHook<'_>,
    kind: PendingPayloadKind,
    value: &T,
) -> Result<(), LandSurfaceEnergyShadowError> {
    if let Some(hook) = hook {
        let bytes = format!("{value:?}").into_bytes();
        hook(kind, &bytes)?;
    }
    Ok(())
}

fn combined_protocol(
    request_batch: &PotentialWaterRequestBatch,
    authorizations: &[WaterAuthorization],
    final_tiles: &[FinalizedRuntimeTile],
) -> Result<WaterProtocol, LandSurfaceEnergyShadowError> {
    if final_tiles.is_empty() {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "empty final tile set",
        ));
    }
    let finalized_uses = final_tiles
        .iter()
        .filter_map(FinalizedRuntimeTile::water_protocol)
        .flat_map(|protocol| protocol.finalized_uses.iter().cloned())
        .collect();
    let condensation_credits = final_tiles
        .iter()
        .filter_map(FinalizedRuntimeTile::water_protocol)
        .flat_map(|protocol| protocol.condensation_credits.iter().cloned())
        .collect();
    let first_protocol = final_tiles
        .iter()
        .find_map(FinalizedRuntimeTile::water_protocol)
        .ok_or(LandSurfaceEnergyShadowError::Identity(
            "Stage-3 open-only hydrology transaction is not yet admitted",
        ))?;
    let protocol = WaterProtocol {
        transaction_id: request_batch.transaction_id,
        hydrology_owner_id: first_protocol.hydrology_owner_id.clone(),
        beginning_snapshot_sha256: first_protocol.beginning_snapshot_sha256.clone(),
        requests: request_batch.requests.clone(),
        authorizations: authorizations.to_vec(),
        finalized_uses,
        condensation_credits,
    };
    protocol.validate()?;
    Ok(protocol)
}

fn pass_through_rollback_hashes(
    identity: &RuntimeTileIdentity,
) -> Vec<openwepp_land_surface_energy::OwnerRollbackHash> {
    [
        (
            OwnerKind::LandSurfaceEnergy,
            identity.lse_owner_id.as_str(),
            &identity.beginning_lse_state_sha256,
        ),
        (
            OwnerKind::Hydrology,
            identity.hydrology_owner_id.as_str(),
            &identity.beginning_hydrology_snapshot_sha256,
        ),
        (
            OwnerKind::SoilThermal,
            identity.soil_thermal_owner_id.as_str(),
            &identity.beginning_soil_thermal_state_sha256,
        ),
    ]
    .into_iter()
    .map(
        |(owner_kind, owner_id, digest)| openwepp_land_surface_energy::OwnerRollbackHash {
            owner_kind,
            owner_id: owner_id.to_owned(),
            before_sha256: digest.clone(),
            after_sha256: digest.clone(),
        },
    )
    .collect()
}

fn common_owner_rollback_hashes(
    final_tiles: &[FinalizedRuntimeTile],
) -> Result<Vec<openwepp_land_surface_energy::OwnerRollbackHash>, LandSurfaceEnergyShadowError> {
    let first = final_tiles
        .first()
        .ok_or(LandSurfaceEnergyShadowError::Identity(
            "empty rollback tile set",
        ))?;
    let selected = |tile: &FinalizedRuntimeTile| {
        let rows = match tile {
            FinalizedRuntimeTile::Open(value) => &value.rollback_hashes,
            FinalizedRuntimeTile::Stage3OpenSnow {
                rollback_hashes, ..
            } => rollback_hashes,
            FinalizedRuntimeTile::Covered(value) => &value.rollback_hashes,
        };
        rows.iter()
            .filter(|row| {
                matches!(
                    row.owner_kind,
                    OwnerKind::LandSurfaceEnergy | OwnerKind::Hydrology | OwnerKind::SoilThermal
                )
            })
            .cloned()
            .collect::<Vec<_>>()
    };
    let baseline = selected(first);
    if baseline.len() != 3
        || final_tiles
            .iter()
            .skip(1)
            .any(|tile| selected(tile) != baseline)
    {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "multi-tile owner rollback identity",
        ));
    }
    Ok(baseline)
}

fn weighted_operand(
    identity: &RuntimeTileIdentity,
    energy: RuntimeTileEnergyOperands<'_>,
) -> Result<WeightedTileEnergyOperands, LandSurfaceEnergyShadowError> {
    energy.validate()?;
    let (
        local_input_j_m2_tile,
        local_output_j_m2_tile,
        local_storage_change_j_m2_tile,
        local_sum_abs_integrated_components_j_m2_tile,
    ) = match energy {
        RuntimeTileEnergyOperands::Open(ground) => {
            let surface = ground.surface;
            (
                (surface.absorbed_shortwave_w_m2 + surface.net_longwave_w_m2) * identity.interval_s,
                surface.sensible_w_m2 * identity.interval_s
                    + ground.latent.vapor_energy_j_m2
                    + surface.ground_heat_w_m2 * identity.interval_s,
                surface.storage_w_m2 * identity.interval_s,
                (surface.absorbed_shortwave_w_m2.abs()
                    + surface.net_longwave_w_m2.abs()
                    + surface.sensible_w_m2.abs()
                    + surface.ground_heat_w_m2.abs()
                    + surface.storage_w_m2.abs())
                    * identity.interval_s
                    + ground.latent.vapor_energy_j_m2.abs(),
            )
        }
        RuntimeTileEnergyOperands::Stage3OpenSnow => (0.0, 0.0, 0.0, 0.0),
        RuntimeTileEnergyOperands::Covered(covered) => {
            let column = &covered.column;
            let incident_shortwave = column.shortwave.incident_w_m2_tile.total();
            let reflected_shortwave = column.shortwave.top_reflected_w_m2_tile.total();
            let canopy_latent_terms = column
                .occupancies
                .iter()
                .flat_map(|occupancy| {
                    [
                        occupancy.sun_leaf,
                        occupancy.shade_leaf,
                        occupancy.wet_surface,
                        occupancy.dry_stem,
                    ]
                })
                .map(|surface| {
                    surface.signed_vapor_to_canopy_air_kg_m2_tile_s
                        * surface.latent_heat_j_kg
                        * identity.interval_s
                })
                .collect::<Vec<_>>();
            let canopy_latent_j_m2 = canopy_latent_terms.iter().sum::<f64>();
            let (
                lower_latent_j_m2,
                lower_ground_heat_w_m2,
                lower_storage_w_m2,
                stage3_boundary_abs_j_m2,
            ) = match &covered.lower_boundary {
                CoveredLowerBoundaryEnergyOperands::SnowFree(ground) => (
                    ground.latent.vapor_energy_j_m2,
                    ground.surface.ground_heat_w_m2,
                    ground.surface.storage_w_m2,
                    0.0,
                ),
                CoveredLowerBoundaryEnergyOperands::Stage3SnowCovered(stage3) => (
                    stage3.latent_energy_to_canopy_air_j_m2_tile,
                    0.0,
                    0.0,
                    (stage3.optical.absorbed_w_m2_tile.total().abs()
                        + stage3.net_longwave_w_m2_tile.abs()
                        + stage3.sensible_to_canopy_air_w_m2_tile.abs()
                        + (stage3.vapor_to_canopy_air_kg_m2_tile_s * stage3.latent_heat_j_kg)
                            .abs())
                        * stage3.interval_s,
                ),
            };
            let sum_abs_latent_j_m2 = canopy_latent_terms
                .iter()
                .map(|value| value.abs())
                .sum::<f64>()
                + lower_latent_j_m2.abs();
            covered_external_energy(
                identity.interval_s,
                incident_shortwave,
                reflected_shortwave,
                column.longwave.atmospheric_downward_w_m2_tile,
                column.longwave.top_upward_w_m2_tile,
                column.canopy_air.sensible_to_reference_air_w_m2_tile,
                canopy_latent_j_m2 + lower_latent_j_m2,
                lower_ground_heat_w_m2,
                lower_storage_w_m2,
                sum_abs_latent_j_m2,
                column.stage3_lower_boundary_energy_w_m2_tile,
                stage3_boundary_abs_j_m2,
            )
        }
    };
    Ok(WeightedTileEnergyOperands {
        tile_fraction: identity.tile_fraction,
        local_input_j_m2_tile,
        local_output_j_m2_tile,
        local_storage_change_j_m2_tile,
        local_sum_abs_integrated_components_j_m2_tile,
    })
}

#[allow(clippy::too_many_arguments)]
fn covered_external_energy(
    interval_s: f64,
    incident_shortwave_w_m2: f64,
    top_reflected_shortwave_w_m2: f64,
    atmospheric_downward_longwave_w_m2: f64,
    top_upward_longwave_w_m2: f64,
    sensible_to_reference_air_w_m2: f64,
    latent_to_reference_air_j_m2: f64,
    ground_heat_w_m2: f64,
    storage_w_m2: f64,
    sum_abs_latent_j_m2: f64,
    stage3_lower_boundary_energy_w_m2: f64,
    stage3_boundary_abs_j_m2: f64,
) -> (f64, f64, f64, f64) {
    (
        (incident_shortwave_w_m2 + atmospheric_downward_longwave_w_m2) * interval_s,
        (top_reflected_shortwave_w_m2
            + top_upward_longwave_w_m2
            + sensible_to_reference_air_w_m2
            + ground_heat_w_m2)
            * interval_s
            + latent_to_reference_air_j_m2
            + stage3_lower_boundary_energy_w_m2 * interval_s,
        storage_w_m2 * interval_s,
        (incident_shortwave_w_m2.abs()
            + top_reflected_shortwave_w_m2.abs()
            + atmospheric_downward_longwave_w_m2.abs()
            + top_upward_longwave_w_m2.abs()
            + sensible_to_reference_air_w_m2.abs()
            + ground_heat_w_m2.abs()
            + storage_w_m2.abs())
            * interval_s
            + sum_abs_latent_j_m2
            + stage3_boundary_abs_j_m2,
    )
}

fn reconstruct_weighted_ofe_energy(
    final_tiles: &[FinalizedRuntimeTile],
) -> Result<Vec<WeightedOfeEnergyJoin>, LandSurfaceEnergyShadowError> {
    let mut by_ofe = BTreeMap::<OfeId, Vec<&FinalizedRuntimeTile>>::new();
    for tile in final_tiles {
        by_ofe
            .entry(tile.identity().ofe_id.clone())
            .or_default()
            .push(tile);
    }
    by_ofe
        .into_iter()
        .map(|(ofe_id, tiles)| {
            let interval_s = tiles[0].identity().interval_s;
            if tiles
                .iter()
                .any(|tile| tile.identity().interval_s.to_bits() != interval_s.to_bits())
            {
                return Err(LandSurfaceEnergyShadowError::Identity(
                    "mixed OFE energy intervals",
                ));
            }
            let ordered_tile_ids = tiles
                .iter()
                .map(|tile| tile.identity().tile_id.clone())
                .collect();
            let operands = tiles
                .iter()
                .map(|tile| weighted_operand(tile.identity(), tile.energy_operands()))
                .collect::<Result<Vec<_>, _>>()?;
            let closure = match validate_weighted_ofe_energy(interval_s, &operands) {
                Ok(value) => value,
                Err(error) => {
                    return Err(error.into());
                }
            };
            Ok(WeightedOfeEnergyJoin {
                ofe_id,
                ordered_tile_ids,
                operands,
                closure,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_problem_surface_has_no_companion_owner_arrays() {
        fn accepts_only_projected_tiles(_: Vec<StrictProjectedTileProblem>) {}
        accepts_only_projected_tiles(Vec::new());
        assert!(std::mem::size_of::<StrictProjectedTileProblem>() > 0);
    }

    #[test]
    fn finalized_tile_type_requires_real_open_or_covered_candidate() {
        assert!(std::mem::size_of::<FinalizedRuntimeTile>() > 0);
    }

    #[test]
    fn topology_closure_uses_configured_tolerance_without_normalizing() {
        let admitted = 1.0 + 32.0 * f64::EPSILON;
        assert!(canonical_tile_fraction_sum_closes(admitted));
        assert_eq!(admitted.to_bits(), (1.0 + 32.0 * f64::EPSILON).to_bits());
        assert!(!canonical_tile_fraction_sum_closes(
            1.0 + 65.0 * f64::EPSILON
        ));
        assert!(!canonical_tile_fraction_sum_closes(f64::NAN));
    }

    #[test]
    fn covered_weighting_uses_every_external_column_boundary() {
        let interval = 10.0;
        let baseline = covered_external_energy(
            interval, 100.0, 10.0, 50.0, 20.0, 30.0, 200.0, 5.0, 65.0, 200.0, 0.0, 0.0,
        );
        assert_eq!(baseline, (1_500.0, 850.0, 650.0, 3_000.0));
        let with_stage3_boundary = covered_external_energy(
            interval, 100.0, 10.0, 50.0, 20.0, 30.0, 200.0, 5.0, 65.0, 200.0, 25.0, 300.0,
        );
        assert_eq!(with_stage3_boundary, (1_500.0, 1_100.0, 650.0, 3_300.0));
        let closes = |terms: (f64, f64, f64, f64), fraction: f64| {
            validate_weighted_ofe_energy(
                interval,
                &[WeightedTileEnergyOperands {
                    tile_fraction: fraction,
                    local_input_j_m2_tile: terms.0,
                    local_output_j_m2_tile: terms.1,
                    local_storage_change_j_m2_tile: terms.2,
                    local_sum_abs_integrated_components_j_m2_tile: terms.3,
                }],
            )
        };
        assert!(closes(baseline, 1.0).is_ok());

        // Every omitted external boundary fails the validator used by the runtime path.
        for poisoned in [
            covered_external_energy(
                interval, 0.0, 10.0, 50.0, 20.0, 30.0, 200.0, 5.0, 65.0, 200.0, 0.0, 0.0,
            ),
            covered_external_energy(
                interval, 100.0, 0.0, 50.0, 20.0, 30.0, 200.0, 5.0, 65.0, 200.0, 0.0, 0.0,
            ),
            covered_external_energy(
                interval, 100.0, 10.0, 0.0, 20.0, 30.0, 200.0, 5.0, 65.0, 200.0, 0.0, 0.0,
            ),
            covered_external_energy(
                interval, 100.0, 10.0, 50.0, 0.0, 30.0, 200.0, 5.0, 65.0, 200.0, 0.0, 0.0,
            ),
            covered_external_energy(
                interval, 100.0, 10.0, 50.0, 20.0, 0.0, 200.0, 5.0, 65.0, 200.0, 0.0, 0.0,
            ),
            covered_external_energy(
                interval, 100.0, 10.0, 50.0, 20.0, 30.0, 0.0, 5.0, 65.0, 0.0, 0.0, 0.0,
            ),
            covered_external_energy(
                interval, 100.0, 10.0, 50.0, 20.0, 30.0, 200.0, 0.0, 65.0, 200.0, 0.0, 0.0,
            ),
            covered_external_energy(
                interval, 100.0, 10.0, 50.0, 20.0, 30.0, 200.0, 5.0, 0.0, 200.0, 0.0, 0.0,
            ),
        ] {
            assert!(closes(poisoned, 1.0).is_err());
        }
        assert!(closes(baseline, 0.5).is_err());
        assert!(closes(baseline, 2.0).is_err());
    }
}
