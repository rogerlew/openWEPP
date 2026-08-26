#[allow(clippy::wildcard_imports)]
use super::super::*;
use super::snow_mass_transition::{
    SNOW_SOLID_TO_LIQUID_CLOSURE_TOLERANCE_M, SNOW_STAGE3_LIQUID_CLOSURE_TOLERANCE_M,
};
use crate::snow_stage3_terminal_handoff::Stage3SnowSurfaceBoundaryReceiptV1;
use openwepp_coupled_time::{Digest32, FramedField, framed_sha256};
use openwepp_coupled_time::{ModelTimeNs, TimeSupport};
use openwepp_meteorology::surface_energy::{
    EnergyFluxWattsPerSquareMeter, MassFluxKilogramsPerSquareMeterSecond, PositiveLengthMeters,
    PrecipitationAdvectedHeatInputs, PrecipitationMassFluxKilogramsPerSquareMeterSecond,
    PressurePascals, RadiativeFluxWattsPerSquareMeter, SnowLongwaveInputs,
    SurfaceEnergyBalanceTerms, ThermalConductivityWattsPerMeterKelvin, TurbulentFluxDiagnostics,
    TurbulentFluxInputs, TurbulentTransferOptions, conductive_heat_flux,
    latent_heat_flux_from_mass_flux, latent_heat_for_surface_temperature, net_shortwave_radiation,
    precipitation_advected_heat_flux, saturation_vapor_pressure_snobal_pa,
    snow_effective_thermal_conductivity_snobal, snow_longwave_dilley_unsworth, specific_heat_ice,
    specific_heat_water, surface_energy_balance, turbulent_fluxes_monin_obukhov_with_diagnostics,
};
use openwepp_unit_boundary::{FractionUnitInterval, LinearRateMetersPerSecond, TemperatureCelsius};
use std::collections::BTreeMap;

mod stage3_solver;

const STAGE3_RHO_WATER_KG_M3: f64 = 1_000.0;
const STAGE3_LATENT_HEAT_FUSION_J_KG: f64 = 333_600.0;
const STAGE3_SPECIFIC_HEAT_ICE_J_KG_K: f64 = 2_100.0;
pub(crate) const STAGE3_DEFAULT_SNOW_ALBEDO: f64 = 0.82;
const STAGE3_SECONDS_PER_HOUR: f64 = 3_600.0;
const STAGE3_ACTIVE_LAYER_MAX_DEPTH_M: f64 = 0.25;
const STAGE3_NORMAL_TIMESTEP_MASS_KG_M2: f64 = 60.0;
const STAGE3_MEDIUM_TIMESTEP_MASS_KG_M2: f64 = 10.0;
pub(crate) const STAGE3_MINIMUM_RESOLVED_THERMAL_MASS_SWE_M: f64 = 0.001;

pub(crate) fn stage3_total_represented_ice_swe_m(state: &DirectSnowStage3PersistentState) -> f64 {
    state
        .layers
        .iter()
        .filter(|layer| snow_density_layer_has_resolved_mass(layer.mass_swe_m))
        .map(|layer| layer.mass_swe_m)
        .sum()
}

pub(crate) fn stage3_has_represented_ice(state: &DirectSnowStage3PersistentState) -> bool {
    state
        .layers
        .iter()
        .any(|layer| snow_density_layer_has_resolved_mass(layer.mass_swe_m))
}

pub(crate) fn stage3_is_resolved_thermal_domain(state: &DirectSnowStage3PersistentState) -> bool {
    stage3_total_represented_ice_swe_m(state) > STAGE3_MINIMUM_RESOLVED_THERMAL_MASS_SWE_M
}

pub(crate) fn stage3_is_terminal_event_domain(state: &DirectSnowStage3PersistentState) -> bool {
    state.schema_version == 2
        && state.terminal_event_model == Some(DirectSnowTerminalEventModel::EnthalpyEventV1)
        && stage3_has_represented_ice(state)
        && stage3_total_represented_ice_swe_m(state) <= STAGE3_MINIMUM_RESOLVED_THERMAL_MASS_SWE_M
}
const STAGE3_MEDIUM_TIMESTEP_SECONDS: f64 = 900.0;
const STAGE3_SMALL_TIMESTEP_SECONDS: f64 = 60.0;
const STAGE3_ENERGY_CLOSURE_TOLERANCE_J_M2: f64 = 1.0e-6;
const STAGE3_BULK_EQUIVALENT_LAYER_CLOSURE_TOLERANCE_M: f64 = 1.0e-9;
const STAGE3_BULK_EQUIVALENT_MAX_LAYERS: usize = 16;

impl SnowStage3ConductivityError {
    /// Replay the exact rejected SNOBAL conductivity primitive.
    ///
    /// # Errors
    ///
    /// Returns the same typed meteorology error when the captured inputs remain
    /// outside the primitive's domain.
    pub fn replay(
        &self,
    ) -> Result<ThermalConductivityWattsPerMeterKelvin, openwepp_meteorology::MeteorologyError>
    {
        let pressure = PressurePascals::try_new(self.atmospheric_pressure_pa)?;
        snow_effective_thermal_conductivity_snobal(
            self.layer.density_kg_m3,
            self.control_volume_temperature,
            pressure,
        )
    }
}

#[derive(Clone, Copy)]
struct Stage3AggregateState {
    swe_after_m: f64,
    depth_after_m: f64,
    density_after_kg_m3: f64,
    settle_day_count_after: f64,
}

#[derive(Clone)]
struct Stage3HourlySurfaceEnergy {
    total_j_m2: f64,
    shortwave_j_m2: f64,
    longwave_j_m2: f64,
    latent_j_m2: f64,
    vapor_mass_exchange_kg_m2: f64,
    latent_mass_energy_j_m2: f64,
    sublimation_m: f64,
    mass_latent_identity_residual_j_m2: f64,
    diagnostics: Option<DirectSnowSurfaceEnergyHourDiagnostics>,
    reconciliation: Option<Stage3CarrierReconciliation>,
}

#[derive(Clone, Copy)]
struct Stage3CarrierReconciliation {
    air_temperature_c: f64,
    dewpoint_c: f64,
    wind_speed_m_s: f64,
    air_pressure_pa: f64,
    hourly_radiation_mj_m2: f64,
    daily_solar_radiation_mj_m2: f64,
    daily_extraterrestrial_radiation_mj_m2: f64,
    daylight: bool,
    canopy_cover_fraction: f64,
    rain_m: f64,
    snowfall_geometric_m: f64,
    rain_mass_flux_kg_m2_s: f64,
    snow_mass_flux_kg_m2_s: f64,
    rain_temperature_c: f64,
    snow_temperature_c: f64,
    rain_specific_heat_j_kg_k: f64,
    snow_specific_heat_j_kg_k: f64,
    incoming_shortwave_w_m2: f64,
    snow_albedo_fraction: f64,
    snow_albedo_source_id: &'static str,
    snow_albedo_model_id: Option<&'static str>,
    snow_albedo_accumulated_positive_temperature_c_day: Option<f64>,
    net_shortwave_w_m2: f64,
    actual_vapor_pressure_pa: f64,
    longwave_cloud_fraction: f64,
    sky_view_fraction: f64,
    atmospheric_longwave_w_m2: f64,
    canopy_longwave_w_m2: f64,
    subcanopy_longwave_w_m2: f64,
    outgoing_longwave_w_m2: f64,
    net_longwave_w_m2: f64,
    longwave_model_id: &'static str,
    sublimation_model_id: &'static str,
    air_temperature_height_m: f64,
    vapor_pressure_height_m: f64,
    wind_speed_height_m: f64,
    aerodynamic_roughness_length_m: f64,
    turbulent_options: TurbulentTransferOptions,
    surface_vapor_pressure_pa: f64,
    surface_latent_heat_j_kg: Option<f64>,
    turbulent: Option<TurbulentFluxDiagnostics>,
    vapor_mass_flux_kg_m2_s: f64,
    sensible_flux_w_m2: f64,
    latent_flux_w_m2: f64,
    precipitation_advected_flux_w_m2: f64,
    snow_soil_heat_flux_w_m2: f64,
    complete_external_flux_w_m2: f64,
}

#[derive(Clone, Copy)]
struct Stage3ReconciliationState {
    active_layer_count: usize,
    total_layer_count: usize,
    active_fingerprint: u64,
    total_fingerprint: u64,
    effective_input_fingerprint: u64,
    active_ice_mass_kg_m2: f64,
    total_ice_mass_kg_m2: f64,
    total_retained_liquid_kg_m2: f64,
    active_depth_m: f64,
    active_density_kg_m3: f64,
    active_cold_j_m2: f64,
    total_cold_j_m2: f64,
    surface_temperature_c: f64,
}

#[allow(clippy::struct_field_names)]
#[derive(Clone, Copy)]
struct Stage3ReconciliationTransfer {
    lower_cold_before_conduction_j_m2: Option<f64>,
    lower_cold_after_conduction_j_m2: Option<f64>,
    active_cold_energy_change_j_m2: Option<f64>,
    lower_cold_energy_change_j_m2: Option<f64>,
    cold_content_export_j_m2: Option<f64>,
    internal_active_lower_conduction_j_m2: Option<f64>,
    melt_kg_m2: Option<f64>,
    refrozen_kg_m2: Option<f64>,
    sublimation_kg_m2: Option<f64>,
    deposition_kg_m2: Option<f64>,
    legacy_sequential_complete_j_m2: Option<f64>,
    energy_closure_residual_j_m2: Option<f64>,
}

impl Stage3ReconciliationTransfer {
    const SAME_STATE: Self = Self {
        lower_cold_before_conduction_j_m2: None,
        lower_cold_after_conduction_j_m2: None,
        active_cold_energy_change_j_m2: None,
        lower_cold_energy_change_j_m2: None,
        cold_content_export_j_m2: None,
        internal_active_lower_conduction_j_m2: None,
        melt_kg_m2: None,
        refrozen_kg_m2: None,
        sublimation_kg_m2: None,
        deposition_kg_m2: None,
        legacy_sequential_complete_j_m2: None,
        energy_closure_residual_j_m2: None,
    };
}

#[derive(Clone, Copy)]
struct Stage3SurfaceInterval {
    surface_temperature_c: f64,
    snow_depth_m: f64,
    snow_density_kg_m3: f64,
    duration_seconds: f64,
    forcing_duration_seconds: f64,
    boundary: Option<Stage3SnowSurfaceBoundaryReceiptV1>,
}

/// Controls whether terminal chronology is forbidden, inspected without
/// publication, or required at one exact covered endpoint.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CoveredTerminalExecutionMode {
    PersistentReject,
    DiscoveryProbe,
    ExactEndpoint { expected_tick: ModelTimeNs },
}

mod terminal_evidence_sealed {
    pub trait Sealed {}
}

#[derive(Clone, Copy)]
pub(crate) struct TerminalPairEvidenceHook {
    pub duration_s: f64,
    pub proposed_next_duration_s: f64,
    pub components: [(f64, f64, f64, f64, f64); 5],
    pub scaled_error: f64,
    pub rejected: bool,
}

#[derive(Clone, Copy)]
pub(crate) struct TerminalAdmissionEvidenceHook<'a> {
    pub proposed_duration_s: f64,
    pub required_half_duration_s: f64,
    pub minimum_duration_s: f64,
    pub outcome: &'a SnowTerminalNumericsFailure,
    pub provider_calls_before: u64,
    pub provider_calls_after: u64,
}

pub(crate) trait TerminalEvidenceMode<J>: terminal_evidence_sealed::Sealed {
    const ENABLED: bool;
    type State;
    type ProviderState;
    type ProviderProjection;
    type ProviderFailureProjection;
    type CouplingState;
    fn new_state() -> Self::State;
    fn new_provider_state() -> Self::ProviderState;
    fn new_coupling_state() -> Self::CouplingState;
    fn project_provider_success(
        _: &CoveredTerminalTrialRequestV1,
        _: &crate::v9_real_consumer_shadow::CoveredCarrierPhaseResultV1,
    ) -> Self::ProviderProjection;
    fn project_provider_failure(
        _: &crate::v9_real_consumer_shadow::DirectV11RealConsumerError,
    ) -> Self::ProviderFailureProjection;
    fn provider_success(_: &mut Self::ProviderState, _: &CoveredTerminalTrialRequestV1, _: Self::ProviderProjection) {}
    fn provider_failure(_: &mut Self::ProviderState, _: &CoveredTerminalTrialRequestV1, _: Self::ProviderFailureProjection) {}
    fn merge_provider(_: &mut Self::State, _: Self::ProviderState) {}
    fn provider_call_count(_: &Self::State) -> u64 { 0 }
    fn pair(_: &mut Self::State, _: TerminalPairEvidenceHook) {}
    fn admission(_: &mut Self::State, _: TerminalAdmissionEvidenceHook<'_>) {}
    fn coupling_iteration(_: &mut Self::CouplingState, _: TerminalCouplingIterationHook) {}
    fn coupling_selection(_: &mut Self::CouplingState, _: TerminalCouplingSelectionHook) {}
    fn merge_coupling(_: &mut Self::State, _: Self::CouplingState) {}
    fn selected_trial(_: &mut Self::State, _: TerminalSelectedTrialHook<'_, J>) {}
}

pub(crate) enum NoEvidence {}
impl terminal_evidence_sealed::Sealed for NoEvidence {}
impl<J> TerminalEvidenceMode<J> for NoEvidence {
    const ENABLED: bool = false;
    type State = ();
    type ProviderState = ();
    type ProviderProjection = ();
    type ProviderFailureProjection = ();
    type CouplingState = ();
    #[inline(always)] fn new_state() {}
    #[inline(always)] fn new_provider_state() {}
    #[inline(always)] fn new_coupling_state() {}
    #[inline(always)] fn project_provider_success(_: &CoveredTerminalTrialRequestV1, _: &crate::v9_real_consumer_shadow::CoveredCarrierPhaseResultV1) {}
    #[inline(always)] fn project_provider_failure(_: &crate::v9_real_consumer_shadow::DirectV11RealConsumerError) {}
    #[inline(always)] fn provider_success(_: &mut (), _: &CoveredTerminalTrialRequestV1, _: ()) {}
    #[inline(always)] fn provider_failure(_: &mut (), _: &CoveredTerminalTrialRequestV1, _: ()) {}
    #[inline(always)] fn merge_provider(_: &mut (), _: ()) {}
    #[inline(always)] fn provider_call_count(_: &()) -> u64 { 0 }
    #[inline(always)] fn pair(_: &mut (), _: TerminalPairEvidenceHook) {}
    #[inline(always)] fn admission(_: &mut (), _: TerminalAdmissionEvidenceHook<'_>) {}
}

#[derive(Clone)]
pub(crate) struct TerminalCouplingIterationHook {
    pub request: CoveredTerminalTrialRequestV1,
    pub outgoing: CoveredTerminalEndingSnowHintV1,
    pub comparisons: Option<[(f64, f64, f64, f64, bool); 4]>,
    pub converged: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TerminalCouplingSelectionReason {
    FourComponentConvergenceBreak,
    IterationLoopExhausted,
}

#[derive(Clone)]
pub(crate) struct TerminalCouplingSelectionHook {
    pub request: CoveredTerminalTrialRequestV1,
    pub reason: TerminalCouplingSelectionReason,
    pub post_loop_three_component_check: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TerminalPairPosition { Coarse, Fine1, Fine2 }

pub(crate) struct TerminalSelectedTrialHook<'a, J> {
    pub position: TerminalPairPosition,
    pub role: CoveredTerminalTrialRoleV1,
    pub attempt_ordinal: u32,
    pub relative_start_s: f64,
    pub duration_s: f64,
    pub beginning: TerminalStateEvidence,
    pub ending: TerminalStateEvidence,
    pub ledger: TerminalLedgerEvidence,
    pub beginning_joint: &'a J,
    pub carrier_ending_joint: &'a J,
    pub hydrology_ending_joint: &'a J,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TerminalStateEvidence { pub ice_kg_m2: f64, pub liquid_kg_m2: f64, pub cold_content_j_m2: f64 }

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TerminalLedgerEvidence {
    pub complete_energy_j_m2: f64, pub cold_energy_change_j_m2: f64,
    pub refrozen_kg_m2: f64, pub deposition_kg_m2: f64, pub sublimation_kg_m2: f64,
    pub melt_kg_m2: f64, pub unallocated_energy_j_m2: f64,
    pub shortwave_energy_j_m2: f64, pub longwave_energy_j_m2: f64,
    pub sensible_energy_j_m2: f64, pub latent_energy_j_m2: f64,
    pub advected_energy_j_m2: f64, pub snow_soil_heat_energy_j_m2: f64,
    pub external_liquid_kg_m2: f64,
}

#[cfg(test)]
#[derive(Clone, Default)]
pub(crate) struct CaptureState {
    pub provider_calls: Vec<CapturedProviderCall>,
    pub pairs: Vec<CapturedPair>,
    pub admissions: Vec<(f64, f64, f64, SnowTerminalNumericsFailure, u64, u64)>,
    pub coupling_iterations: Vec<CapturedCouplingIteration>,
    pub coupling_selections: Vec<TerminalCouplingSelectionHook>,
    pub selected_trials: Vec<CapturedSelectedTrial>,
}

#[cfg(test)]
#[derive(Clone)]
pub(crate) struct CapturedPair {
    pub duration_s: f64, pub proposed_next_duration_s: f64,
    /// Ordered ice, liquid, cold content, complete energy, unallocated energy.
    /// Each tuple is `(coarse, refined, delta, denominator, scaled)`.
    pub components: Vec<(f64, f64, f64, f64, f64)>,
    pub maximum_scaled: f64, pub rejected: bool,
}

#[cfg(test)]
#[derive(Clone)]
pub(crate) enum CapturedProviderOutcome {
    Success(Box<crate::v9_real_consumer_shadow::CoveredCarrierPhaseResultV1>),
    Failure(CapturedProviderFailure),
}
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CapturedProviderFailure {
    Runtime, Vegetation, Serialization, Identity(&'static str), CoveredBoundary,
    Stage3, Stage3PrecipitationCustody(&'static str), Stage3SnowSoilHeatCustody(&'static str),
}
#[cfg(test)]
#[derive(Clone)]
pub(crate) struct CapturedProviderCall { pub ordinal: u64, pub request: CoveredTerminalTrialRequestV1, pub outcome: CapturedProviderOutcome }
#[cfg(test)]
#[derive(Clone)]
pub(crate) struct CapturedCouplingIteration { pub hook: TerminalCouplingIterationHook, pub provider_ordinal: Option<u64> }
#[cfg(test)]
#[derive(Clone)]
pub(crate) struct CapturedSelectedTrial {
    pub position: TerminalPairPosition, pub role: CoveredTerminalTrialRoleV1, pub attempt_ordinal: u32,
    pub relative_start_s: f64, pub duration_s: f64, pub beginning: TerminalStateEvidence,
    pub ending: TerminalStateEvidence, pub ledger: TerminalLedgerEvidence,
    pub beginning_joint: Option<CoveredTerminalJointTrialStateV1>,
    pub carrier_ending_joint: Option<CoveredTerminalJointTrialStateV1>,
    pub hydrology_ending_joint: Option<CoveredTerminalJointTrialStateV1>,
}

#[cfg(test)]
pub(crate) struct ValidatedCaptureState {
    pub provider_calls: Vec<CapturedProviderCall>,
    pub coupling_iterations: Vec<ValidatedCouplingIteration>,
    pub coupling_selections: Vec<TerminalCouplingSelectionHook>,
    pub pairs: Vec<ValidatedCapturedPair>,
    pub floor: ValidatedFloorAdmission,
    pub call_count_through_final_pair: u64,
    pub call_count_at_floor: u64,
}

#[cfg(test)]
pub(crate) struct ValidatedCouplingIteration {
    pub draft: TerminalCouplingIterationHook,
    pub provider_ordinal: u64,
}

#[cfg(test)]
pub(crate) struct ValidatedSelectedTrial {
    pub draft: CapturedSelectedTrial,
    pub support: TimeSupport,
    pub coupling_selection_index: usize,
    pub selected_provider_ordinal: u64,
}

#[cfg(test)]
pub(crate) struct ValidatedCapturedPair {
    pub trials: [ValidatedSelectedTrial; 3],
    pub decision: CapturedPair,
}

#[cfg(test)]
pub(crate) struct ValidatedFloorAdmission {
    pub proposed_duration_s: f64,
    pub required_half_duration_s: f64,
    pub minimum_duration_s: f64,
    pub outcome: SnowTerminalNumericsFailure,
}

#[cfg(test)]
fn same_terminal_call_key(
    left: &CoveredTerminalTrialRequestV1,
    right: &CoveredTerminalTrialRequestV1,
) -> bool {
    left.lane_id == right.lane_id
        && left.support == right.support
        && left.role == right.role
        && left.attempt_ordinal == right.attempt_ordinal
        && left.coupling_iteration == right.coupling_iteration
        && left.beginning_joint.receipt_sha256() == right.beginning_joint.receipt_sha256()
}

#[cfg(test)]
fn same_terminal_coupling_group(
    left: &CoveredTerminalTrialRequestV1,
    right: &CoveredTerminalTrialRequestV1,
) -> bool {
    left.lane_id == right.lane_id
        && left.support == right.support
        && left.role == right.role
        && left.attempt_ordinal == right.attempt_ordinal
        && left.beginning_joint.receipt_sha256() == right.beginning_joint.receipt_sha256()
}

#[cfg(test)]
impl CaptureState {
    pub(crate) fn validate(self) -> Result<ValidatedCaptureState, &'static str> {
        if self.admissions.len() != 1 || self.pairs.is_empty() {
            return Err("floor/pair cardinality");
        }
        if self
            .provider_calls
            .iter()
            .enumerate()
            .any(|(ordinal, call)| call.ordinal != ordinal as u64)
        {
            return Err("provider order");
        }
        if self.selected_trials.len() != self.pairs.len() * 3 {
            return Err("selected trial cardinality");
        }
        if self.coupling_selections.len() != self.selected_trials.len()
            || self
                .coupling_selections
                .iter()
                .zip(&self.selected_trials)
                .any(|(selection, trial)| {
                    selection.request.role != trial.role
                        || selection.request.attempt_ordinal != trial.attempt_ordinal
                        || f64::from_bits(selection.request.support.duration_s_bits()).to_bits()
                            != trial.duration_s.to_bits()
                })
        {
            return Err("selection/trial order");
        }
        for (pair, trials) in self.pairs.iter().zip(self.selected_trials.chunks_exact(3)) {
            if trials[0].position != TerminalPairPosition::Coarse
                || trials[1].position != TerminalPairPosition::Fine1
                || trials[2].position != TerminalPairPosition::Fine2
                || !matches!(trials[0].role, CoveredTerminalTrialRoleV1::Full | CoveredTerminalTrialRoleV1::Retry)
                || trials[1].role != CoveredTerminalTrialRoleV1::Half1
                || trials[2].role != CoveredTerminalTrialRoleV1::Half2
                || trials[2].beginning != trials[1].ending
                || trials[2].beginning_joint != trials[1].hydrology_ending_joint
                || trials[0].attempt_ordinal.checked_add(1) != Some(trials[1].attempt_ordinal)
                || trials[1].attempt_ordinal.checked_add(1) != Some(trials[2].attempt_ordinal)
                || trials[0].beginning != trials[1].beginning
                || trials[0].beginning_joint != trials[1].beginning_joint
                || trials[0].relative_start_s.to_bits() != trials[1].relative_start_s.to_bits()
                || trials[2].relative_start_s.to_bits()
                    != (trials[1].relative_start_s + trials[1].duration_s).to_bits()
                || trials[0].duration_s.to_bits() != pair.duration_s.to_bits()
                || trials[1].duration_s.to_bits() != (pair.duration_s / 2.0).to_bits()
                || trials[2].duration_s.to_bits() != trials[1].duration_s.to_bits()
            {
                return Err("selected trial order/join");
            }
            if pair.components.len() != 5 {
                return Err("decision cardinality");
            }
            let coarse = [
                trials[0].ending.ice_kg_m2,
                trials[0].ending.liquid_kg_m2,
                trials[0].ending.cold_content_j_m2,
                trials[0].ledger.complete_energy_j_m2,
                trials[0].ledger.unallocated_energy_j_m2,
            ];
            let refined = [
                trials[2].ending.ice_kg_m2,
                trials[2].ending.liquid_kg_m2,
                trials[2].ending.cold_content_j_m2,
                trials[1].ledger.complete_energy_j_m2
                    + trials[2].ledger.complete_energy_j_m2,
                trials[1].ledger.unallocated_energy_j_m2
                    + trials[2].ledger.unallocated_energy_j_m2,
            ];
            for (index, component) in pair.components.iter().enumerate() {
                let absolute = if index < 2 {
                    stage3_solver::MASS_ABSOLUTE_TOLERANCE_KG_M2
                } else {
                    stage3_solver::ENERGY_ABSOLUTE_TOLERANCE_J_M2
                };
                let delta = refined[index] - coarse[index];
                let denominator = absolute
                    + stage3_solver::RELATIVE_ERROR_TOLERANCE
                        * coarse[index].abs().max(refined[index].abs());
                let expected = (
                    coarse[index],
                    refined[index],
                    delta,
                    denominator,
                    delta.abs() / denominator,
                );
                if component.0.to_bits() != expected.0.to_bits()
                    || component.1.to_bits() != expected.1.to_bits()
                    || component.2.to_bits() != expected.2.to_bits()
                    || component.3.to_bits() != expected.3.to_bits()
                    || component.4.to_bits() != expected.4.to_bits()
                {
                    return Err("decision reconstruction");
                }
            }
            let maximum = pair.components[1..]
                .iter()
                .fold(pair.components[0].4, |current, component| {
                    current.max(component.4)
                });
            if maximum.to_bits() != pair.maximum_scaled.to_bits()
                || pair.rejected != (pair.maximum_scaled > 1.0 && pair.components[0].1 > 0.0)
            {
                return Err("decision predicate");
            }
        }
        let successful_calls = self
            .provider_calls
            .iter()
            .filter(|call| matches!(call.outcome, CapturedProviderOutcome::Success(_)))
            .collect::<Vec<_>>();
        for (index, call) in self.provider_calls.iter().enumerate() {
            if self.provider_calls[index + 1..]
                .iter()
                .any(|other| same_terminal_call_key(&call.request, &other.request))
            {
                return Err("provider key uniqueness");
            }
            if matches!(call.outcome, CapturedProviderOutcome::Failure(_))
                && self
                    .coupling_iterations
                    .iter()
                    .any(|iteration| same_terminal_call_key(&call.request, &iteration.hook.request))
            {
                return Err("failed provider iteration join");
            }
            if let CapturedProviderOutcome::Success(result) = &call.outcome {
                if result.transition.boundary.support != call.request.support
                    || result.transition.beginning_joint != call.request.beginning_joint
                    || result.transition.probe_child_identity.trial_support
                        != call.request.support
                    || result.transition.probe_child_identity.role != call.request.role
                    || result.transition.probe_child_identity.attempt_ordinal
                        != call.request.attempt_ordinal
                    || result.transition.probe_child_identity.beginning_joint_sha256
                        != call.request.beginning_joint.receipt_sha256()
                {
                    return Err("provider result/request join");
                }
            }
        }
        if successful_calls.len() != self.coupling_iterations.len() {
            return Err("provider/iteration cardinality");
        }
        let mut validated_iterations = Vec::with_capacity(self.coupling_iterations.len());
        let mut preceding_provider_ordinal = None;
        for iteration in &self.coupling_iterations {
            let matching = successful_calls
                .iter()
                .filter(|call| same_terminal_call_key(&call.request, &iteration.hook.request))
                .collect::<Vec<_>>();
            if matching.len() != 1 {
                return Err("provider/iteration join");
            }
            let zero = iteration.hook.request.coupling_iteration == 0;
            if zero != iteration.hook.comparisons.is_none()
                || zero != iteration.hook.request.ending_snow_hint.is_none()
            {
                return Err("iteration zero optionality");
            }
            if let Some(previous) = iteration.hook.request.ending_snow_hint {
                let expected = terminal_coupling_comparisons(previous, iteration.hook.outgoing);
                if iteration.hook.comparisons != Some(expected)
                    || iteration.hook.converged != expected.iter().all(|comparison| comparison.4)
                {
                    return Err("iteration comparison reconstruction");
                }
            } else if iteration.hook.converged {
                return Err("iteration zero convergence");
            }
            if preceding_provider_ordinal.is_some_and(|ordinal| ordinal >= matching[0].ordinal) {
                return Err("coupling iteration order");
            }
            preceding_provider_ordinal = Some(matching[0].ordinal);
            validated_iterations.push(ValidatedCouplingIteration {
                draft: iteration.hook.clone(),
                provider_ordinal: matching[0].ordinal,
            });
        }
        if successful_calls.iter().any(|call| {
            self.coupling_iterations
                .iter()
                .filter(|iteration| same_terminal_call_key(&call.request, &iteration.hook.request))
                .count()
                != 1
        }) {
            return Err("provider/iteration reverse join");
        }
        let mut group_start = 0;
        while group_start < self.coupling_iterations.len() {
            let first = &self.coupling_iterations[group_start].hook;
            if first.request.coupling_iteration != 0
                || first.request.ending_snow_hint.is_some()
            {
                return Err("coupling group start");
            }
            let mut group_end = group_start + 1;
            while group_end < self.coupling_iterations.len()
                && same_terminal_coupling_group(
                    &first.request,
                    &self.coupling_iterations[group_end].hook.request,
                )
            {
                let previous = &self.coupling_iterations[group_end - 1].hook;
                let current = &self.coupling_iterations[group_end].hook;
                if current.request.coupling_iteration
                    != previous.request.coupling_iteration.checked_add(1).ok_or("coupling ordinal")?
                    || current.request.ending_snow_hint != Some(previous.outgoing)
                {
                    return Err("coupling group chain");
                }
                group_end += 1;
            }
            let final_request = &self.coupling_iterations[group_end - 1].hook.request;
            if self
                .coupling_selections
                .iter()
                .filter(|selection| same_terminal_call_key(&selection.request, final_request))
                .count()
                != 1
                || self.coupling_selections.iter().any(|selection| {
                    same_terminal_coupling_group(&selection.request, &first.request)
                        && !same_terminal_call_key(&selection.request, final_request)
                })
            {
                return Err("coupling group selection");
            }
            group_start = group_end;
        }
        for selection in &self.coupling_selections {
            let matching = self
                .coupling_iterations
                .iter()
                .filter(|iteration| same_terminal_call_key(&iteration.hook.request, &selection.request))
                .collect::<Vec<_>>();
            if matching.len() != 1
                || selection.post_loop_three_component_check != true
                || (selection.reason
                    == TerminalCouplingSelectionReason::FourComponentConvergenceBreak)
                    != matching[0].hook.converged
            {
                return Err("coupling selection join");
            }
        }
        let final_pair = self.pairs.last().ok_or("final pair")?;
        let final_trials = &self.selected_trials[self.selected_trials.len() - 3..];
        if final_trials[0].role != CoveredTerminalTrialRoleV1::Retry {
            return Err("final retry role");
        }
        let floor = &self.admissions[0];
        if floor.0.to_bits() != final_pair.proposed_next_duration_s.to_bits()
            || floor.1.to_bits() != (floor.0 / 2.0).to_bits()
            || floor.1 >= floor.2
            || floor.2.to_bits() != 0.6_f64.to_bits()
            || floor.3 != SnowTerminalNumericsFailure::BelowCarrierDomain
        {
            return Err("floor join");
        }
        let call_count_through_final_pair = validated_iterations
            .iter()
            .filter(|iteration| {
                final_trials.iter().any(|trial| {
                    iteration.draft.request.role == trial.role
                        && iteration.draft.request.attempt_ordinal == trial.attempt_ordinal
                })
            })
            .map(|iteration| iteration.provider_ordinal + 1)
            .max()
            .ok_or("final pair calls")?;
        let call_count_at_floor = self.provider_calls.len() as u64;
        if call_count_through_final_pair != call_count_at_floor {
            return Err("floor provider call boundary");
        }
        let mut validated_pairs = Vec::with_capacity(self.pairs.len());
        for (decision, trials) in self.pairs.into_iter().zip(self.selected_trials.chunks_exact(3)) {
            let validated_trials = trials
                .iter()
                .cloned()
                .map(|draft| {
                    let matching = self
                        .coupling_selections
                        .iter()
                        .enumerate()
                        .filter(|(_, selection)| {
                            selection.request.role == draft.role
                                && selection.request.attempt_ordinal == draft.attempt_ordinal
                        })
                        .collect::<Vec<_>>();
                    if matching.len() != 1 {
                        return Err("trial/coupling selection join");
                    }
                    let (selection_index, selection) = matching[0];
                    let iteration = validated_iterations
                        .iter()
                        .find(|iteration| {
                            same_terminal_call_key(&iteration.draft.request, &selection.request)
                        })
                        .ok_or("selected iteration")?;
                    if draft.beginning_joint.as_ref() != Some(&selection.request.beginning_joint) {
                        return Err("trial beginning joint");
                    }
                    let provider = self
                        .provider_calls
                        .get(iteration.provider_ordinal as usize)
                        .ok_or("selected provider ordinal")?;
                    let CapturedProviderOutcome::Success(result) = &provider.outcome else {
                        return Err("selected provider outcome");
                    };
                    if draft.carrier_ending_joint.as_ref() != Some(result.ending_candidates.joint()) {
                        return Err("trial carrier ending joint");
                    }
                    let expected_hydrology_ending = draft
                        .carrier_ending_joint
                        .as_ref()
                        .ok_or("trial carrier ending joint")?
                        .with_terminal_hydrology_state(
                            selection.request.lane_id,
                            draft.ending.ice_kg_m2,
                            draft.ending.liquid_kg_m2,
                            draft.ending.cold_content_j_m2,
                        )
                        .map_err(|_| "trial hydrology ending reconstruction")?;
                    if draft.hydrology_ending_joint.as_ref()
                        != Some(&expected_hydrology_ending)
                    {
                        return Err("trial hydrology ending joint");
                    }
                    if iteration.draft.outgoing.ice_kg_m2.to_bits()
                        != draft.ending.ice_kg_m2.to_bits()
                        || iteration.draft.outgoing.liquid_kg_m2.to_bits()
                            != draft.ending.liquid_kg_m2.to_bits()
                        || iteration.draft.outgoing.cold_content_j_m2.to_bits()
                            != draft.ending.cold_content_j_m2.to_bits()
                    {
                        return Err("selected coupling/trial ending");
                    }
                    Ok(ValidatedSelectedTrial {
                        support: selection.request.support,
                        coupling_selection_index: selection_index,
                        selected_provider_ordinal: iteration.provider_ordinal,
                        draft,
                    })
                })
                .collect::<Result<Vec<_>, &'static str>>()?
                .try_into()
                .map_err(|_| "validated trial array")?;
            validated_pairs.push(ValidatedCapturedPair { trials: validated_trials, decision });
        }
        Ok(ValidatedCaptureState {
            provider_calls: self.provider_calls,
            coupling_iterations: validated_iterations,
            coupling_selections: self.coupling_selections,
            pairs: validated_pairs,
            floor: ValidatedFloorAdmission {
                proposed_duration_s: floor.0,
                required_half_duration_s: floor.1,
                minimum_duration_s: floor.2,
                outcome: floor.3.clone(),
            },
            call_count_through_final_pair,
            call_count_at_floor,
        })
    }
}

#[cfg(test)]
pub(crate) enum CaptureEvidence {}
#[cfg(test)]
impl terminal_evidence_sealed::Sealed for CaptureEvidence {}
#[cfg(test)]
impl TerminalEvidenceMode<Option<CoveredTerminalJointTrialStateV1>> for CaptureEvidence {
    const ENABLED: bool = true;
    type State = CaptureState;
    type ProviderState = Vec<CapturedProviderCall>;
    type ProviderProjection = crate::v9_real_consumer_shadow::CoveredCarrierPhaseResultV1;
    type ProviderFailureProjection = CapturedProviderFailure;
    type CouplingState = (Vec<TerminalCouplingIterationHook>, Vec<TerminalCouplingSelectionHook>);
    fn new_state() -> Self::State { CaptureState::default() }
    fn new_provider_state() -> Self::ProviderState { Vec::new() }
    fn new_coupling_state() -> Self::CouplingState { (Vec::new(), Vec::new()) }
    fn project_provider_success(_: &CoveredTerminalTrialRequestV1, result: &crate::v9_real_consumer_shadow::CoveredCarrierPhaseResultV1) -> Self::ProviderProjection { result.clone() }
    fn project_provider_failure(error: &crate::v9_real_consumer_shadow::DirectV11RealConsumerError) -> Self::ProviderFailureProjection {
        use crate::v9_real_consumer_shadow::DirectV11RealConsumerError as Error;
        match error {
            Error::Runtime(_) => CapturedProviderFailure::Runtime,
            Error::Vegetation(_) => CapturedProviderFailure::Vegetation,
            Error::Serialization(_) => CapturedProviderFailure::Serialization,
            Error::Identity(value) => CapturedProviderFailure::Identity(value),
            Error::CoveredBoundary(_) => CapturedProviderFailure::CoveredBoundary,
            Error::Stage3(_) => CapturedProviderFailure::Stage3,
            Error::Stage3PrecipitationCustody(value) => CapturedProviderFailure::Stage3PrecipitationCustody(value),
            Error::Stage3SnowSoilHeatCustody(value) => CapturedProviderFailure::Stage3SnowSoilHeatCustody(value),
        }
    }
    fn provider_success(state: &mut Self::ProviderState, request: &CoveredTerminalTrialRequestV1, result: Self::ProviderProjection) {
        state.push(CapturedProviderCall {
            ordinal: state.len() as u64,
            request: request.clone(),
            outcome: CapturedProviderOutcome::Success(Box::new(result)),
        });
    }
    fn provider_failure(state: &mut Self::ProviderState, request: &CoveredTerminalTrialRequestV1, error: Self::ProviderFailureProjection) {
        state.push(CapturedProviderCall { ordinal: state.len() as u64, request: request.clone(), outcome: CapturedProviderOutcome::Failure(error) });
    }
    fn merge_provider(state: &mut Self::State, mut provider: Self::ProviderState) {
        let base = state.provider_calls.len() as u64;
        for call in &mut provider {
            call.ordinal += base;
        }
        state.provider_calls.extend(provider);
    }
    fn provider_call_count(state: &Self::State) -> u64 { state.provider_calls.len() as u64 }
    fn pair(state: &mut Self::State, value: TerminalPairEvidenceHook) {
        state.pairs.push(CapturedPair { duration_s: value.duration_s,
            proposed_next_duration_s: value.proposed_next_duration_s,
            components: value.components.into_iter().collect(), maximum_scaled: value.scaled_error,
            rejected: value.rejected });
    }
    fn admission(state: &mut Self::State, value: TerminalAdmissionEvidenceHook<'_>) {
        state.admissions.push((value.proposed_duration_s, value.required_half_duration_s, value.minimum_duration_s, value.outcome.clone(), value.provider_calls_before, value.provider_calls_after));
    }
    fn coupling_iteration(state: &mut Self::CouplingState, value: TerminalCouplingIterationHook) { state.0.push(value); }
    fn coupling_selection(state: &mut Self::CouplingState, value: TerminalCouplingSelectionHook) { state.1.push(value); }
    fn merge_coupling(state: &mut Self::State, coupling: Self::CouplingState) {
        state.coupling_iterations.extend(coupling.0.into_iter().map(|hook| CapturedCouplingIteration { hook, provider_ordinal: None }));
        state.coupling_selections.extend(coupling.1);
    }
    fn selected_trial(state: &mut Self::State, value: TerminalSelectedTrialHook<'_, Option<CoveredTerminalJointTrialStateV1>>) {
        state.selected_trials.push(CapturedSelectedTrial { position: value.position, role: value.role, attempt_ordinal: value.attempt_ordinal,
            relative_start_s: value.relative_start_s, duration_s: value.duration_s, beginning: value.beginning, ending: value.ending,
            ledger: value.ledger, beginning_joint: value.beginning_joint.clone(), carrier_ending_joint: value.carrier_ending_joint.clone(),
            hydrology_ending_joint: value.hydrology_ending_joint.clone() });
    }
}

/// Pure input presented to the covered carrier for every adaptive and
/// event-root terminal trial.  The support is the exact absolute interval
/// beginning at the supplied trial state; it is never a scaled parent receipt.
#[derive(Clone, Debug)]
pub(crate) struct CoveredTerminalTrialRequestV1 {
    pub lane_id: u32,
    pub support: TimeSupport,
    pub role: CoveredTerminalTrialRoleV1,
    pub attempt_ordinal: u32,
    /// Zero-based iteration within one adaptive/root attempt. This is not an
    /// accepted chronology ordinal and may never escape probe evaluation.
    pub coupling_iteration: u32,
    pub ice_kg_m2: f64,
    pub liquid_kg_m2: f64,
    pub cold_content_j_m2: f64,
    pub surface_temperature_c: f64,
    pub snow_depth_m: f64,
    pub snow_density_kg_m3: f64,
    /// Hydrology ending-state estimate from the preceding coupled replay.
    /// `None` denotes the first replay from the immutable trial-start joint.
    pub ending_snow_hint: Option<CoveredTerminalEndingSnowHintV1>,
    pub beginning_joint: CoveredTerminalJointTrialStateV1,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct CoveredTerminalEndingSnowHintV1 {
    pub ice_kg_m2: f64,
    pub liquid_kg_m2: f64,
    pub cold_content_j_m2: f64,
    pub surface_temperature_c: f64,
}

pub(crate) fn terminal_coupling_comparisons(
    previous: CoveredTerminalEndingSnowHintV1,
    next: CoveredTerminalEndingSnowHintV1,
) -> [(f64, f64, f64, f64, bool); 4] {
    let comparison = |left: f64, right: f64, tolerance: f64| {
        let difference = (left - right).abs();
        (left, right, difference, tolerance, difference <= tolerance)
    };
    [
        comparison(previous.ice_kg_m2, next.ice_kg_m2, 1.0e-9),
        comparison(previous.liquid_kg_m2, next.liquid_kg_m2, 1.0e-9),
        comparison(previous.cold_content_j_m2, next.cold_content_j_m2, 1.0e-6),
        comparison(
            previous.surface_temperature_c,
            next.surface_temperature_c,
            1.0e-9,
        ),
    ]
}

pub(crate) fn terminal_coupling_four_component_converged(
    previous: CoveredTerminalEndingSnowHintV1,
    next: CoveredTerminalEndingSnowHintV1,
) -> bool {
    terminal_coupling_comparisons(previous, next)
        .into_iter()
        .all(|comparison| comparison.4)
}

pub(crate) fn terminal_coupling_post_loop_three_component_converged(
    previous: CoveredTerminalEndingSnowHintV1,
    next: CoveredTerminalEndingSnowHintV1,
) -> bool {
    terminal_coupling_comparisons(previous, next)[..3]
        .iter()
        .all(|comparison| comparison.4)
}

/// Immutable, unpublished seven-owner candidate carried between covered
/// terminal trials. These bytes are never installed by the hydrology solver.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CoveredTerminalJointTrialStateV1 {
    authority: JointTrialAuthorityV1,
    owner_bytes: BTreeMap<String, Vec<u8>>,
    receipt_sha256: Digest32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct JointTrialAuthorityV1 {
    pub source_owner_set_sha256: Digest32,
    pub lane_id: u32,
    pub source_snow_owner_sha256: Digest32,
    pub interval_index: u64,
    pub state_support: TimeSupport,
    pub accepted_predecessors: Vec<Digest32>,
}

impl CoveredTerminalJointTrialStateV1 {
    pub(crate) fn try_new(
        authority: JointTrialAuthorityV1,
        owner_bytes: BTreeMap<String, Vec<u8>>,
    ) -> Result<Self, DirectSnowStage3EvaluationError> {
        const OWNER_IDS: [&str; 7] = [
            "vegetation",
            "snow",
            "land_surface_energy",
            "hydrology",
            "bgc",
            "soil_thermal",
            "surface_liquid",
        ];
        if owner_bytes.len() != OWNER_IDS.len()
            || OWNER_IDS.iter().any(|id| !owner_bytes.contains_key(*id))
        {
            return Err(Wb11HydrologyKernel::stage3_domain_error(
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                "snow.terminal_joint_trial_complete_owner_set",
                owner_bytes.len() as f64,
                Some(7.0),
                Some(7.0),
            )
            .into());
        }
        if authority.source_owner_set_sha256 == Digest32::zero()
            || authority.source_snow_owner_sha256 == Digest32::zero()
            || authority.accepted_predecessors.contains(&Digest32::zero())
            || authority
                .accepted_predecessors
                .iter()
                .collect::<std::collections::BTreeSet<_>>()
                .len()
                != authority.accepted_predecessors.len()
        {
            return Err(DirectSnowStage3EvaluationError::TerminalCustody("terminal joint authority"));
        }
        let receipt_sha256 = covered_terminal_joint_digest(&authority, &owner_bytes)?;
        Ok(Self {
            authority,
            owner_bytes,
            receipt_sha256,
        })
    }

    pub(crate) fn owner_bytes(&self) -> &BTreeMap<String, Vec<u8>> {
        &self.owner_bytes
    }
    pub(crate) const fn receipt_sha256(&self) -> Digest32 {
        self.receipt_sha256
    }
    pub(crate) const fn authority(&self) -> &JointTrialAuthorityV1 {
        &self.authority
    }

    /// Seal the hydrology-owned aggregate snow candidate after one carrier
    /// trial. The carrier may evolve the other six unpublished candidates,
    /// but it cannot claim an ending snow owner before the terminal operator
    /// has applied the accepted flux integral.
    pub(super) fn with_terminal_hydrology_state(
        &self,
        lane_id: u32,
        ice_kg_m2: f64,
        liquid_kg_m2: f64,
        cold_content_j_m2: f64,
    ) -> Result<Self, DirectSnowStage3EvaluationError> {
        if [ice_kg_m2, liquid_kg_m2, cold_content_j_m2]
            .iter()
            .any(|value| !value.is_finite() || *value < 0.0)
        {
            return Err(Wb11HydrologyKernel::stage3_domain_error(
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                "snow.terminal_joint_trial_hydrology_state",
                1.0,
                Some(0.0),
                Some(0.0),
            )
            .into());
        }
        let prior_snow = self.owner_bytes.get("snow").ok_or_else(|| {
            DirectSnowStage3EvaluationError::TerminalCustody(
                "terminal joint missing snow owner",
            )
        })?;
        let mut snow = Vec::with_capacity(100);
        snow.extend_from_slice(b"OPENWEPP_STAGE3_TERMINAL_TRIAL_SNOW_V1\0");
        snow.extend_from_slice(&lane_id.to_be_bytes());
        snow.extend_from_slice(openwepp_coupled_time::digest_bytes(prior_snow).as_bytes());
        snow.extend_from_slice(&ice_kg_m2.to_bits().to_be_bytes());
        snow.extend_from_slice(&liquid_kg_m2.to_bits().to_be_bytes());
        snow.extend_from_slice(&cold_content_j_m2.to_bits().to_be_bytes());
        let mut owners = self.owner_bytes.clone();
        owners.insert("snow".to_owned(), snow);
        let mut authority = self.authority.clone();
        // `self` is the carrier-complete trial result presented to the
        // hydrology join. Its identity is the accepted fine predecessor;
        // discarded coarse/retry alternatives never reach the returned chain.
        let predecessor = self.receipt_sha256;
        if authority.accepted_predecessors.contains(&predecessor) {
            return Err(DirectSnowStage3EvaluationError::TerminalCustody(
                "duplicate terminal predecessor",
            ));
        }
        authority.accepted_predecessors.push(predecessor);
        Self::try_new(authority, owners)
    }
}

fn covered_terminal_joint_digest(
    authority: &JointTrialAuthorityV1,
    owner_bytes: &BTreeMap<String, Vec<u8>>,
) -> Result<Digest32, DirectSnowStage3EvaluationError> {
    const OWNER_IDS: [&str; 7] = [
        "vegetation", "snow", "land_surface_energy", "hydrology", "bgc",
        "soil_thermal", "surface_liquid",
    ];
    let schema = 1_u32.to_be_bytes();
    let mut fields = Vec::with_capacity(8 + owner_bytes.len() * 2);
    fields.push(FramedField { tag: "schema", value: &schema });
    let lane = authority.lane_id.to_be_bytes();
    let interval = authority.interval_index.to_be_bytes();
    let start = authority.state_support.start_ns().get().to_be_bytes();
    let end = authority.state_support.end_ns().get().to_be_bytes();
    let mut predecessors = (authority.accepted_predecessors.len() as u32).to_be_bytes().to_vec();
    for predecessor in &authority.accepted_predecessors {
        predecessors.extend_from_slice(predecessor.as_bytes());
    }
    fields.extend([
        FramedField { tag: "source_owner_set", value: authority.source_owner_set_sha256.as_bytes() },
        FramedField { tag: "lane", value: &lane },
        FramedField { tag: "source_snow_owner", value: authority.source_snow_owner_sha256.as_bytes() },
        FramedField { tag: "interval_index", value: &interval },
        FramedField { tag: "state_support_start", value: &start },
        FramedField { tag: "state_support_end", value: &end },
        FramedField { tag: "accepted_predecessors", value: &predecessors },
    ]);
    for owner_id in OWNER_IDS {
        let bytes = owner_bytes.get(owner_id).ok_or(
            DirectSnowStage3EvaluationError::TerminalCustody(
                "terminal joint canonical owner order",
            ),
        )?;
        fields.push(FramedField {
            tag: "owner_id",
            value: owner_id.as_bytes(),
        });
        fields.push(FramedField {
            tag: "owner_bytes",
            value: bytes,
        });
    }
    framed_sha256("covered-terminal-joint-trial-state", &fields).map_err(|_| {
        Wb11HydrologyKernel::stage3_domain_error(
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
            "snow.terminal_joint_trial_canonical_framing",
            1.0,
            Some(0.0),
            Some(0.0),
        )
        .into()
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CoveredProbeChildIdentityV1 {
    pub parent_transaction_sha256: Digest32,
    pub enclosing_parent_support: TimeSupport,
    pub trial_support: TimeSupport,
    pub physical_child_ordinal: u32,
    pub role: CoveredTerminalTrialRoleV1,
    pub attempt_ordinal: u32,
    pub beginning_joint_sha256: Digest32,
    pub beginning_owner_set_sha256: Digest32,
    pub complete_forcing_sha256: Digest32,
    pub topology_sha256: Digest32,
    pub receipt_sha256: Digest32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ProbeChildAuthorityV1 {
    pub parent_transaction_sha256: Digest32,
    pub enclosing_parent_support: TimeSupport,
    pub trial_support: TimeSupport,
    pub physical_child_ordinal: u32,
    pub attempt_ordinal: u32,
    pub role: CoveredTerminalTrialRoleV1,
    pub beginning_joint_sha256: Digest32,
    pub beginning_owner_set_sha256: Digest32,
    pub complete_forcing_sha256: Digest32,
    pub topology_sha256: Digest32,
}

impl CoveredProbeChildIdentityV1 {
    pub(crate) fn try_new(
        authority: ProbeChildAuthorityV1,
    ) -> Result<Self, DirectSnowStage3EvaluationError> {
        let ProbeChildAuthorityV1 { parent_transaction_sha256, enclosing_parent_support, trial_support, physical_child_ordinal, attempt_ordinal, role, beginning_joint_sha256, beginning_owner_set_sha256, complete_forcing_sha256, topology_sha256 } = authority;
        if [parent_transaction_sha256, beginning_joint_sha256, beginning_owner_set_sha256, complete_forcing_sha256, topology_sha256].contains(&Digest32::zero())
            || trial_support.start_ns() < enclosing_parent_support.start_ns()
            || trial_support.end_ns() > enclosing_parent_support.end_ns()
        {
            return Err(Wb11HydrologyKernel::stage3_domain_error(
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                "snow.terminal_probe_child_support",
                1.0,
                Some(0.0),
                Some(0.0),
            )
            .into());
        }
        let ordinal = physical_child_ordinal.to_be_bytes();
        let role_byte = [role as u8];
        let attempt = attempt_ordinal.to_be_bytes();
        let parent_start = enclosing_parent_support.start_ns().get().to_be_bytes();
        let parent_end = enclosing_parent_support.end_ns().get().to_be_bytes();
        let trial_start = trial_support.start_ns().get().to_be_bytes();
        let trial_end = trial_support.end_ns().get().to_be_bytes();
        let schema = 1_u32.to_be_bytes();
        let receipt_sha256 = framed_sha256(
            "covered-probe-child-identity",
            &[
                FramedField {
                    tag: "schema",
                    value: &schema,
                },
                FramedField {
                    tag: "parent_transaction",
                    value: parent_transaction_sha256.as_bytes(),
                },
                FramedField {
                    tag: "enclosing_support_start",
                    value: &parent_start,
                },
                FramedField {
                    tag: "enclosing_support_end",
                    value: &parent_end,
                },
                FramedField {
                    tag: "trial_support_start",
                    value: &trial_start,
                },
                FramedField {
                    tag: "trial_support_end",
                    value: &trial_end,
                },
                FramedField {
                    tag: "physical_child_ordinal",
                    value: &ordinal,
                },
                FramedField {
                    tag: "attempt",
                    value: &attempt,
                },
                FramedField {
                    tag: "role",
                    value: &role_byte,
                },
                FramedField {
                    tag: "beginning_joint",
                    value: beginning_joint_sha256.as_bytes(),
                },
                FramedField { tag: "beginning_owner_set", value: beginning_owner_set_sha256.as_bytes() },
                FramedField { tag: "complete_forcing", value: complete_forcing_sha256.as_bytes() },
                FramedField { tag: "topology", value: topology_sha256.as_bytes() },
            ],
        )
        .map_err(|_| {
            Wb11HydrologyKernel::stage3_domain_error(
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                "snow.terminal_probe_child_canonical_framing",
                1.0,
                Some(0.0),
                Some(0.0),
            )
        })?;
        Ok(Self {
            parent_transaction_sha256,
            enclosing_parent_support,
            trial_support,
            physical_child_ordinal,
            role,
            attempt_ordinal,
            beginning_joint_sha256,
            beginning_owner_set_sha256,
            complete_forcing_sha256,
            topology_sha256,
            receipt_sha256,
        })
    }
}

#[derive(Clone, Debug)]
pub(crate) struct CoveredTerminalTrialTransitionV1 {
    pub boundary: Stage3SnowSurfaceBoundaryReceiptV1,
    pub beginning_joint: CoveredTerminalJointTrialStateV1,
    pub ending_joint: CoveredTerminalJointTrialStateV1,
    pub probe_child_identity: CoveredProbeChildIdentityV1,
    pub trial_snow_soil_receipt:
        Option<crate::v9_real_consumer_shadow::TerminalSnowSoilTrialReceiptV1>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub(crate) enum CoveredTerminalTrialRoleV1 {
    Full = 0,
    Half1 = 1,
    Half2 = 2,
    Retry = 3,
    BracketLower = 4,
    BracketUpper = 5,
    Root = 6,
}

pub(crate) type CoveredTerminalTrialProviderV1<'a> = dyn FnMut(
        CoveredTerminalTrialRequestV1,
    ) -> Result<CoveredTerminalTrialTransitionV1, DirectSnowStage3EvaluationError>
    + 'a;

#[derive(Clone, Debug)]
pub(crate) struct CoveredTerminalLaneTrialStateV2 {
    pub lane_id: u32,
    pub ice_kg_m2: f64,
    pub liquid_kg_m2: f64,
    pub cold_content_j_m2: f64,
    pub surface_temperature_c: f64,
    pub snow_depth_m: f64,
    pub snow_density_kg_m3: f64,
    pub resolved_beginning: bool,
    pub candidate_event_tick: Option<ModelTimeNs>,
}

#[derive(Clone, Debug)]
pub(crate) struct CoveredTerminalBatchTrialRequestV2 {
    pub support: TimeSupport,
    pub role: CoveredTerminalTrialRoleV1,
    pub attempt_ordinal: u32,
    pub lanes: BTreeMap<u32, CoveredTerminalLaneTrialStateV2>,
    pub beginning_joint: CoveredTerminalJointTrialStateV1,
}

#[derive(Clone, Debug)]
pub(crate) struct CoveredTerminalBatchCarrierCandidatesV2 {
    pub support: TimeSupport,
    pub beginning_joint_sha256: Digest32,
    pub carrier_joint: CoveredTerminalJointTrialStateV1,
    pub boundaries_by_lane: BTreeMap<u32, Stage3SnowSurfaceBoundaryReceiptV1>,
    pub ordered_q_ss_receipts_by_lane: BTreeMap<
        u32,
        crate::v9_real_consumer_shadow::TerminalSnowSoilTrialReceiptV1,
    >,
}

#[derive(Clone, Debug)]
pub(crate) struct CoveredTerminalBatchTrialResultV2 {
    pub support: TimeSupport,
    pub hydrology_endings_by_lane: BTreeMap<u32, DirectSnowStage3PersistentState>,
    pub carrier_candidates: CoveredTerminalBatchCarrierCandidatesV2,
    pub ending_joint: CoveredTerminalJointTrialStateV1,
}

pub(crate) type CoveredTerminalBatchTrialProviderV2<'a> = dyn FnMut(
        &CoveredTerminalBatchTrialRequestV2,
    ) -> Result<CoveredTerminalBatchCarrierCandidatesV2, DirectSnowStage3EvaluationError>
    + 'a;

pub(crate) type CoveredTerminalBatchHydrologyJoinV2<'a> = dyn FnMut(
        &CoveredTerminalBatchTrialRequestV2,
        &CoveredTerminalBatchCarrierCandidatesV2,
        &BTreeMap<u32, DirectSnowStage3PersistentState>,
    ) -> Result<CoveredTerminalJointTrialStateV1, DirectSnowStage3EvaluationError>
    + 'a;

// Transitional aliases keep the already-landed solver call sites source-compatible while
// carrier integration moves to the canonical trial terminology above.
pub(crate) type CoveredTerminalBatchPrefixRequestV2 = CoveredTerminalBatchTrialRequestV2;
pub(crate) type CoveredTerminalBatchJoinedResultV2 = CoveredTerminalBatchTrialResultV2;
pub(crate) type CoveredTerminalBatchProviderV2<'a> = CoveredTerminalBatchTrialProviderV2<'a>;

#[derive(Clone, Copy)]
struct Stage3ThermalControlVolume {
    mass_swe_m: f64,
    depth_m: f64,
    density_kg_m3: f64,
    cold_content_j_m2: f64,
    conductivity_w_m_k: f64,
}

#[derive(Clone, Copy)]
struct Stage3ConductionExchange {
    requested_active_energy: f64,
    flux: f64,
    active_energy: f64,
    lower_energy: f64,
    rejected_active_energy: f64,
}

impl Stage3ConductionExchange {
    const ZERO: Self = Self {
        requested_active_energy: 0.0,
        flux: 0.0,
        active_energy: 0.0,
        lower_energy: 0.0,
        rejected_active_energy: 0.0,
    };
}

#[derive(Clone, Copy)]
struct Stage3SubstepDiagnostics {
    surface: DirectSnowSurfaceEnergyHourDiagnostics,
    duration_seconds: f64,
    applied_j_m2: f64,
    unused_j_m2: f64,
    active: Stage3ThermalControlVolume,
    lower: Option<Stage3ThermalControlVolume>,
    conduction: Stage3ConductionExchange,
    active_energy_closure_residual_j_m2: f64,
    lower_energy_closure_residual_j_m2: f64,
    atmospheric_pressure_pa: f64,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Stage3EvaluationTag {
    operator: SnowStage3EvaluationOperator,
    source_snapshot_id: &'static str,
    support_id: &'static str,
    cadence_id: &'static str,
    carrier_id: &'static str,
    coverage_id: &'static str,
    claim_class: &'static str,
    unresolved_boundaries_id: &'static str,
    pairing_id: Option<&'static str>,
    arm_ids: [&'static str; 2],
    arm_count: u8,
}

impl Stage3EvaluationTag {
    const fn new(operator: SnowStage3EvaluationOperator) -> Self {
        let paired = matches!(
            operator,
            SnowStage3EvaluationOperator::SameStatePairedCarrierV1
        );
        let persistent = matches!(
            operator,
            SnowStage3EvaluationOperator::PersistentAccumulationShadowV1
        );
        Self {
            operator,
            source_snapshot_id: if persistent {
                "pre_interval_authoritative_initial_snapshot_v1"
            } else {
                "post_coe_daily_initial_snapshot_v1"
            },
            support_id: if persistent {
                "stage3_persistent_daily_24_hour_support_v1"
            } else {
                "stage3_daily_24_hour_support_v1"
            },
            cadence_id: if paired {
                "stage3_fixed_hourly_immutable_snapshot_v1"
            } else {
                "stage3_dynamic_substep_with_hourly_forcing_v1"
            },
            carrier_id: if paired {
                "stage3_carrier_pair_v1"
            } else {
                "stage3_complete_carrier_v1"
            },
            coverage_id: "evaluated_seconds_over_requested_seconds_v1",
            claim_class: operator.claim_class(),
            unresolved_boundaries_id: "snow_ground_cross_day_terminal_recipient_unresolved_v1",
            pairing_id: if paired {
                Some("stage3_carrier_pair_v1")
            } else {
                None
            },
            arm_ids: if paired {
                ["stage3_surface_energy_v1", "stage3_complete_carrier_v1"]
            } else {
                ["stage3_complete_carrier_v1", "not_applicable"]
            },
            arm_count: if paired { 2 } else { 1 },
        }
    }
}

#[derive(Clone)]
struct Stage3ShadowSummary {
    tag: Stage3EvaluationTag,
    source_fingerprint: u64,
    forcing_fingerprint: u64,
    geometry_fingerprint: u64,
    non_formulation_fingerprint: u64,
    surface_arm_non_formulation_fingerprint: u64,
    complete_arm_non_formulation_fingerprint: u64,
    requested_seconds: f64,
    evaluated_seconds: f64,
    surface_arm_shortwave_j_m2: f64,
    surface_arm_longwave_j_m2: f64,
    surface_arm_latent_j_m2: f64,
    surface_arm_total_j_m2: f64,
    complete_shortwave_j_m2: f64,
    complete_longwave_j_m2: f64,
    complete_sensible_j_m2: f64,
    complete_latent_j_m2: f64,
    complete_advected_j_m2: f64,
    complete_snow_soil_heat_j_m2: f64,
    internal_active_lower_conduction_j_m2: f64,
    complete_vapor_mass_exchange_kg_m2: f64,
    cold_content_export_j_m2: f64,
    available_ice_kg_m2: f64,
    complete_energy_j_m2: f64,
    cold_energy_change_j_m2: f64,
    excess_energy_j_m2: f64,
    sublimation_kg_m2: f64,
    melt_kg_m2: f64,
    unallocated_after_exhaustion_j_m2: f64,
    maximum_energy_closure_residual_j_m2: f64,
    hourly: [DirectSnowStage3EvaluationHourDiagnostics; 24],
    reconciliation: DirectSnowStage3OperatorReconciliation,
    final_layers: Vec<DirectSnowLayerState>,
    terminal_event: Option<DirectSnowTerminalEventResult>,
    terminal_intervals: Vec<DirectSnowTerminalEventResult>,
    terminal_ending_joint: Option<CoveredTerminalJointTrialStateV1>,
    terminal_refrozen_kg_m2: f64,
    persistent_refrozen_kg_m2: f64,
    terminal_deposition_kg_m2: f64,
}

impl Stage3ShadowSummary {
    const fn new(tag: Stage3EvaluationTag) -> Self {
        Self {
            tag,
            source_fingerprint: 0,
            forcing_fingerprint: 0,
            geometry_fingerprint: 0,
            non_formulation_fingerprint: 0,
            surface_arm_non_formulation_fingerprint: 0,
            complete_arm_non_formulation_fingerprint: 0,
            requested_seconds: 24.0 * STAGE3_SECONDS_PER_HOUR,
            evaluated_seconds: 0.0,
            surface_arm_shortwave_j_m2: 0.0,
            surface_arm_longwave_j_m2: 0.0,
            surface_arm_latent_j_m2: 0.0,
            surface_arm_total_j_m2: 0.0,
            complete_shortwave_j_m2: 0.0,
            complete_longwave_j_m2: 0.0,
            complete_sensible_j_m2: 0.0,
            complete_latent_j_m2: 0.0,
            complete_advected_j_m2: 0.0,
            complete_snow_soil_heat_j_m2: 0.0,
            internal_active_lower_conduction_j_m2: 0.0,
            complete_vapor_mass_exchange_kg_m2: 0.0,
            cold_content_export_j_m2: 0.0,
            available_ice_kg_m2: 0.0,
            complete_energy_j_m2: 0.0,
            cold_energy_change_j_m2: 0.0,
            excess_energy_j_m2: 0.0,
            sublimation_kg_m2: 0.0,
            melt_kg_m2: 0.0,
            unallocated_after_exhaustion_j_m2: 0.0,
            maximum_energy_closure_residual_j_m2: 0.0,
            hourly: [DirectSnowStage3EvaluationHourDiagnostics::zero(); 24],
            reconciliation: DirectSnowStage3OperatorReconciliation {
                schema_version: 6,
                hourly_status: [DirectSnowStage3ReconciliationHourStatus::not_selected(); 24],
                tuples: Vec::new(),
            },
            final_layers: Vec::new(),
            terminal_event: None,
            terminal_intervals: Vec::new(),
            terminal_ending_joint: None,
            terminal_refrozen_kg_m2: 0.0,
            persistent_refrozen_kg_m2: 0.0,
            terminal_deposition_kg_m2: 0.0,
        }
    }
}

fn inactive_direct_winter_frost_partition() -> DirectWinterFrostPartitionOutcome {
    DirectWinterFrostPartitionOutcome {
        active_frost_coupling: false,
        dthaw_after_m: 0.0,
        nft_after: 0.0,
        infcap_frz_m_s: 0.0,
        soil_water_after_frwatc_m: None,
        frwatc_soil_water_before_m: 0.0,
        frwatc_soil_water_after_m: 0.0,
        frwatc_frozen_water_before_m: 0.0,
        frwatc_frozen_water_after_m: 0.0,
        frwatc_freeze_debit_m: 0.0,
        frwatc_thaw_credit_m: 0.0,
        frwatc_net_liquid_delta_m: 0.0,
        frozen_water_after_m: 0.0,
        frost_depth_after_m: 0.0,
        thdp_after_m: 0.0,
        tfrdp_after_m: 0.0,
        tthawd_after_m: 0.0,
        fgthwd_flag_after: 0.0,
        total_fine_layer_count: 0.0,
        conductivity_tilled_w_m_k: 0.0,
        conductivity_untilled_w_m_k: 0.0,
        conductivity_residue_w_m_k: 0.0,
        shadow_total_water_before_m: 0.0,
        shadow_total_water_after_m: 0.0,
        shadow_wb_delta_m: 0.0,
        shadow_frwatc_residual_m: 0.0,
        watpdg_m: 0.0,
        watbtm_m: 0.0,
        layer_projection: Vec::new(),
        layer_shadow_projection: Vec::new(),
        fine_layer_projection: Vec::new(),
    }
}

fn active_direct_winter_frost_partition(
    frost_coupling: &FrostCouplingOutcome,
) -> DirectWinterFrostPartitionOutcome {
    DirectWinterFrostPartitionOutcome {
        active_frost_coupling: true,
        dthaw_after_m: frost_coupling.dthaw,
        nft_after: frost_coupling.nft,
        infcap_frz_m_s: frost_coupling.infcap_frz,
        soil_water_after_frwatc_m: frost_coupling.soil_water_after_frwatc,
        frwatc_soil_water_before_m: frost_coupling.frwatc_soil_water_before,
        frwatc_soil_water_after_m: frost_coupling.frwatc_soil_water_after,
        frwatc_frozen_water_before_m: frost_coupling.frwatc_frozen_water_before,
        frwatc_frozen_water_after_m: frost_coupling.frwatc_frozen_water_after,
        frwatc_freeze_debit_m: frost_coupling.frwatc_freeze_debit,
        frwatc_thaw_credit_m: frost_coupling.frwatc_thaw_credit,
        frwatc_net_liquid_delta_m: frost_coupling.frwatc_net_liquid_delta,
        frozen_water_after_m: frost_coupling.frwatc_frozen_water_after,
        frost_depth_after_m: frost_coupling.frdp_m,
        thdp_after_m: frost_coupling.thdp_m,
        tfrdp_after_m: frost_coupling.tfrdp_m,
        tthawd_after_m: frost_coupling.tthawd_m,
        fgthwd_flag_after: frost_coupling.fgthwd_flag,
        total_fine_layer_count: frost_coupling.total_fine_layer_count,
        conductivity_tilled_w_m_k: frost_coupling.conductivity_tilled_w_m_k,
        conductivity_untilled_w_m_k: frost_coupling.conductivity_untilled_w_m_k,
        conductivity_residue_w_m_k: frost_coupling.conductivity_residue_w_m_k,
        shadow_total_water_before_m: frost_coupling.shadow_total_water_before_m,
        shadow_total_water_after_m: frost_coupling.shadow_total_water_after_m,
        shadow_wb_delta_m: frost_coupling.shadow_wb_delta_m,
        shadow_frwatc_residual_m: frost_coupling.shadow_frwatc_residual_m,
        watpdg_m: frost_coupling.watpdg_m,
        watbtm_m: frost_coupling.watbtm_m,
        layer_projection: frost_coupling
            .layer_topology_state
            .iter()
            .map(|layer| DirectFrostLayerProjection {
                layer_index: layer.layer_index,
                theta_after_m: layer.theta_after_m,
                frozen_depth_m: layer.frozen_depth_m,
                frozen_water_m: layer.frzw_m,
            })
            .collect(),
        layer_shadow_projection: frost_coupling
            .shadow_layer_state
            .iter()
            .map(|layer| DirectFrostLayerShadowProjection {
                layer_index: layer.layer_index,
                st_m: layer.st_m,
                soil_water_m: layer.soil_water_m,
                frozen_depth_m: layer.frozen_depth_m,
                frozen_water_m: layer.frzw_m,
                soilf_m: layer.soilf_m,
                yst_m: layer.yst_m,
                nwfrzz_m: layer.nwfrzz_m,
            })
            .collect(),
        fine_layer_projection: frost_coupling
            .fine_layer_state
            .iter()
            .map(|fine| DirectFrostFineLayerProjection {
                layer_index: fine.layer_index,
                fine_index: fine.fine_index,
                fgfrst: fine.fgfrst,
                slfsd_m: fine.slfsd_m,
                slsic_m: fine.slsic_m,
                slsw_theta: fine.slsw_theta,
                sltime_s: fine.sltime_s,
            })
            .collect(),
    }
}

impl Wb11HydrologyKernel {
    /// Attaches an evaluation-only Stage 3 record to an authoritative inactive
    /// snow partition without requesting forcing or advancing snow state.
    ///
    /// The returned schema-v6 record declares the full daily support as
    /// requested but unevaluated. Its empty tuple inventory and
    /// `operator_not_selected` hourly statuses make the inactive lifecycle
    /// explicit while preserving the authoritative partition byte-for-byte.
    #[must_use]
    pub fn attach_inactive_stage3_evaluation(
        authoritative: DirectSnowLiquidPartition,
        operator: SnowStage3EvaluationOperator,
    ) -> DirectSnowStage3EvaluationWithReconciliationResult {
        let mut summary = Stage3ShadowSummary::new(Stage3EvaluationTag::new(operator));
        for hourly in &mut summary.hourly {
            hourly.requested_seconds = STAGE3_SECONDS_PER_HOUR;
        }
        let evaluation = Self::stage3_evaluation_diagnostics(&summary);
        DirectSnowStage3EvaluationWithReconciliationResult {
            result: DirectSnowStage3EvaluationResult {
                authoritative,
                evaluation: Some(evaluation),
            },
            reconciliation: Some(Box::new(summary.reconciliation)),
        }
    }

    pub(crate) fn resolve_snow_partition_terms(
        phase_class: HillslopeKernelPhaseClass,
        hyetograph_rainfall: f64,
        snow_coupling: &SnowCouplingOutcome,
    ) -> Result<(f64, f64), Wb11HydrologyKernelGuardError> {
        let runoff_snow_term = snow_coupling.signed_s
            + snow_coupling.accumulation
            + snow_coupling.rain_retained
            + snow_coupling.rain_released;
        Self::require_dynamic_state_range_with(
            phase_class,
            || BoundarySymbol::from("snow.routed_melt_m"),
            runoff_snow_term,
            Some(0.0),
            None,
        )?;
        let runoff_snow_term = Self::normalize_non_negative_within_tolerance(runoff_snow_term);
        let hyetograph_liquid_input_raw = hyetograph_rainfall
            - snow_coupling.accumulation
            - snow_coupling.rain_retained
            - snow_coupling.rain_released;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_RAINFALL_INPUT,
            hyetograph_liquid_input_raw,
            Some(0.0),
            None,
        )?;
        let hyetograph_liquid_input =
            Self::normalize_non_negative_within_tolerance(hyetograph_liquid_input_raw);
        Self::require_dynamic_state_range_with(
            phase_class,
            || BoundarySymbol::from("snow.post_winter_rain_m"),
            hyetograph_liquid_input,
            Some(0.0),
            None,
        )?;

        Ok((runoff_snow_term, hyetograph_liquid_input))
    }

    pub fn compute_direct_winter_frost_partition(
        inputs: &DirectActiveFrostPartitionInputs,
    ) -> Result<DirectWinterFrostPartitionOutcome, Wb11HydrologyKernelGuardError> {
        let phase_class = HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_SOIL_CONDUCTIVITY,
            inputs.soil_conductivity_m_s,
            Some(0.0),
            None,
        )?;
        if !inputs.controls.wint_red_enabled {
            return Ok(inactive_direct_winter_frost_partition());
        }
        let frost_coupling = Self::compute_active_frost_coupling_from_typed(phase_class, inputs)?;
        Ok(active_direct_winter_frost_partition(&frost_coupling))
    }

    // This public conservation boundary keeps the snow-coupling, density,
    // Stage-3 energy, and aggregate-state handoffs visible in one sequence.
    pub fn compute_direct_snow_liquid_partition_from_typed(
        inputs: &DirectActiveSnowPartitionInputs,
    ) -> Result<DirectSnowLiquidPartition, Wb11HydrologyKernelGuardError> {
        Self::compute_direct_snow_liquid_partition_with_capture(
            inputs,
            DirectSnowDiagnosticCapture::Verbose,
        )
    }

    /// Computes the authoritative snow mass transition and optionally retains
    /// the allocation-heavy diagnostics needed by the selected trace consumer.
    #[allow(clippy::too_many_lines)]
    pub fn compute_direct_snow_liquid_partition_with_capture(
        inputs: &DirectActiveSnowPartitionInputs,
        capture: DirectSnowDiagnosticCapture,
    ) -> Result<DirectSnowLiquidPartition, Wb11HydrologyKernelGuardError> {
        match Self::compute_direct_snow_liquid_partition_with_capture_and_evaluation(
            inputs, capture, None,
        ) {
            Ok(result) => Ok(result.authoritative),
            Err(DirectSnowStage3EvaluationError::Kernel(source)) => Err(*source),
            Err(DirectSnowStage3EvaluationError::TurbulentTransfer(snapshot)) => {
                Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class: snapshot.phase_class,
                    symbol: BoundarySymbol::from("snow.stage3_shadow_turbulent_flux"),
                    value: snapshot.wind_speed_m_s,
                    minimum: Some(0.0),
                    maximum: None,
                })
            }
            Err(DirectSnowStage3EvaluationError::TerminalNumerics(_)) => {
                Err(Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                    phase_class: HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                    symbol: BoundarySymbol::from("snow.unreachable_terminal_numerics"),
                })
            }
            Err(DirectSnowStage3EvaluationError::TerminalCustody(_)) => {
                Err(Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                    phase_class: HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                    symbol: BoundarySymbol::from("snow.terminal_custody"),
                })
            }
        }
    }

    /// Computes a verbose authoritative result plus one bounded evaluation
    /// operator without changing the legacy options record or default entry point.
    pub fn compute_direct_snow_liquid_partition_with_evaluation(
        inputs: &DirectActiveSnowPartitionInputs,
        operator: SnowStage3EvaluationOperator,
    ) -> Result<DirectSnowStage3EvaluationResult, DirectSnowStage3EvaluationError> {
        Self::compute_direct_snow_liquid_partition_with_capture_and_evaluation(
            inputs,
            DirectSnowDiagnosticCapture::Verbose,
            Some(operator),
        )
    }

    /// Computes the authoritative result with an additive, evaluator-only
    /// request used by the selected internal trace consumer.
    #[allow(clippy::too_many_lines)]
    pub fn compute_direct_snow_liquid_partition_with_capture_and_evaluation(
        inputs: &DirectActiveSnowPartitionInputs,
        capture: DirectSnowDiagnosticCapture,
        evaluation_operator: Option<SnowStage3EvaluationOperator>,
    ) -> Result<DirectSnowStage3EvaluationResult, DirectSnowStage3EvaluationError> {
        Self::compute_direct_snow_liquid_partition_with_capture_and_reconciliation(
            inputs,
            capture,
            evaluation_operator,
        )
        .map(|result| result.result)
    }

    /// Computes the protected evaluation result plus its enabled-only schema-v6
    /// reconciliation companion for the internal trace consumer.
    #[allow(clippy::too_many_lines)]
    pub fn compute_direct_snow_liquid_partition_with_capture_and_reconciliation(
        inputs: &DirectActiveSnowPartitionInputs,
        capture: DirectSnowDiagnosticCapture,
        evaluation_operator: Option<SnowStage3EvaluationOperator>,
    ) -> Result<DirectSnowStage3EvaluationWithReconciliationResult, DirectSnowStage3EvaluationError>
    {
        let phase_class = HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from(WB12_SYMBOL_RAINFALL_INPUT),
            inputs.hyetograph_rainfall_m,
            Some(0.0),
            None,
        )?;
        let typed_hourly_snowfall_present = inputs
            .hourly
            .iter()
            .any(|hour| hour.snowfall_m > WB11_ZERO_THRESHOLD);
        let active_snow_coupling = inputs.runtime_swe_m > WB11_ZERO_THRESHOLD
            || typed_hourly_snowfall_present
            || (inputs.hyetograph_rainfall_m > WB11_ZERO_THRESHOLD
                && f64::midpoint(inputs.tmax_c, inputs.tmin_c) < 0.0);
        let snow_coupling = if active_snow_coupling {
            Self::compute_active_snow_coupling_from_typed(phase_class, inputs, capture)?
        } else {
            Self::inactive_snow_coupling_from_typed(phase_class, inputs, capture)?
        };
        let (routed_melt_m, post_winter_rain_m) = Self::resolve_snow_partition_terms(
            phase_class,
            inputs.hyetograph_rainfall_m,
            &snow_coupling,
        )?;
        let density_outcome =
            Self::resolve_typed_snow_density_outcome(phase_class, inputs, &snow_coupling)?;
        let mut density_process_diagnostics = density_outcome.density_process_diagnostics;
        let mut snow_layers_after = density_outcome.layers_after;
        let stage3_resolution = Self::resolve_stage3_liquid_routing(
            phase_class,
            inputs,
            routed_melt_m,
            Stage3AggregateState {
                swe_after_m: density_outcome.runtime_swe_after_m,
                depth_after_m: density_outcome.runtime_depth_after_m,
                density_after_kg_m3: density_outcome.runtime_density_after_kg_m3,
                settle_day_count_after: snow_coupling.runtime_settle_day_count,
            },
            &mut snow_layers_after,
            capture,
            evaluation_operator,
        )?;
        snow_layers_after.retain(|layer| snow_density_layer_has_resolved_mass(layer.mass_swe_m));
        let runtime_swe_after_m = (density_outcome.runtime_swe_after_m
            - stage3_resolution.outcome.sublimation_m)
            .max(0.0);
        let runtime_depth_after_m = if stage3_resolution.outcome.enabled {
            snow_layers_after
                .iter()
                .map(|layer| layer.thickness_m)
                .sum::<f64>()
        } else {
            density_outcome.runtime_depth_after_m
        };
        let runtime_density_after_kg_m3 = if runtime_swe_after_m <= WB11_ZERO_THRESHOLD {
            0.0
        } else if stage3_resolution.outcome.sublimation_m > 0.0 {
            (runtime_swe_after_m * STAGE3_RHO_WATER_KG_M3 / runtime_depth_after_m)
                .min(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3)
        } else {
            density_outcome.runtime_density_after_kg_m3
        };
        let (coe_boundary_depth_after_m, coe_boundary_density_after_kg_m3) =
            if stage3_resolution.outcome.sublimation_m > 0.0 {
                (runtime_depth_after_m, runtime_density_after_kg_m3)
            } else {
                (
                    density_outcome.coe_boundary_depth_after_m,
                    density_outcome.coe_boundary_density_after_kg_m3,
                )
            };
        density_process_diagnostics
            .apply_downstream_stage3_density(runtime_density_after_kg_m3)
            .map_err(|error| {
                Self::snow_density_guard_error(
                    phase_class,
                    &error,
                    inputs.runtime_swe_m,
                    inputs.runtime_depth_m,
                    &inputs.snow_layers,
                )
            })?;
        let accumulation_melt_diagnostics =
            snow_coupling.verbose_diagnostics.as_deref().map(|verbose| {
                DirectSnowAccumulationMeltDiagnostics {
                    wind_m_s: inputs.wind_m_s,
                    dewpoint_c: inputs.dewpoint_c,
                    canopy_cover_fraction: inputs.canopy_cover_fraction,
                    hourly_active_precipitation_m: std::array::from_fn(|index| {
                        inputs.hourly[index].active_precipitation_m
                    }),
                    hourly_rain_m: std::array::from_fn(|index| inputs.hourly[index].rain_m),
                    hourly_snowfall_depth_m: std::array::from_fn(|index| {
                        inputs.hourly[index].snowfall_m
                    }),
                    hourly_snowfall_swe_m: std::array::from_fn(|index| {
                        inputs.hourly[index].snowfall_m * 0.1
                    }),
                    hourly_air_temperature_c: std::array::from_fn(|index| {
                        inputs.hourly[index].air_temperature_c
                    }),
                    hourly_radiation_mj_m2: std::array::from_fn(|index| {
                        inputs.hourly[index].radiation_mj_m2
                    }),
                    hourly_cloud_fraction: std::array::from_fn(|index| {
                        inputs.hourly[index].cloud_fraction
                    }),
                    hourly_rain_fraction: std::array::from_fn(|index| {
                        inputs.hourly[index].rain_fraction
                    }),
                    hourly_snow_fraction: std::array::from_fn(|index| {
                        inputs.hourly[index].snow_fraction
                    }),
                    hourly_phase_model: std::array::from_fn(|index| {
                        inputs.hourly[index].phase_model
                    }),
                    hourly_hydrometeor_temperature_c: std::array::from_fn(|index| {
                        inputs.hourly[index].hydrometeor_temperature_c
                    }),
                    hourly_melt: verbose.hourly_melt,
                    hourly_routed_melt_m: snow_coupling.hourly_routed_melt,
                    hourly_liquid_holding_capacity_m: verbose.hourly_trace.liquid_holding_capacity,
                    hourly_liquid_water_retained_before_m: verbose
                        .hourly_trace
                        .liquid_water_retained_before,
                    hourly_liquid_water_retained_after_m: verbose
                        .hourly_trace
                        .liquid_water_retained_after,
                    hourly_liquid_water_released_m: verbose.hourly_trace.liquid_water_released,
                    hourly_rain_released_m: verbose.hourly_trace.rain_released,
                    hourly_sublimation_m: verbose.hourly_trace.sublimation,
                    hourly_pack_depth_before_m: verbose.hourly_trace.pack_depth_before,
                    hourly_pack_depth_after_m: verbose.hourly_trace.pack_depth_after,
                    hourly_pack_density_before_kg_m3: verbose.hourly_trace.pack_density_before,
                    hourly_pack_density_after_kg_m3: verbose.hourly_trace.pack_density_after,
                    modeled_wind_redistribution_m: [0.0; 24],
                }
            });
        let verbose_diagnostics =
            match (accumulation_melt_diagnostics, stage3_resolution.diagnostics) {
                (Some(accumulation_melt), Some(stage3)) => {
                    Some(Box::new(DirectSnowVerboseDiagnostics {
                        accumulation_melt,
                        stage3,
                    }))
                }
                (None, None) => None,
                _ => {
                    return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                        phase_class,
                        symbol: BoundarySymbol::from("snow.verbose_diagnostic_capture_mismatch"),
                        value: 1.0,
                        minimum: Some(0.0),
                        maximum: Some(0.0),
                    }
                    .into());
                }
            };

        let solid_to_liquid_ledger = DirectSnowSolidToLiquidLedger {
            raw_signed_melt_m: snow_coupling.raw_melt,
            redistributed_positive_melt_m: snow_coupling.redistributed_melt,
            snowpack_swe_loss_m: snow_coupling.snowpack_state_loss,
            rain_released_m: snow_coupling.rain_released,
            liquid_handoff_m: routed_melt_m,
        };

        let mass_transition_ledgers = DirectSnowMassTransitionLedgers::from_authoritative_parts(
            solid_to_liquid_ledger,
            stage3_resolution.liquid_disposition_ledger,
            stage3_resolution.outcome,
        );
        mass_transition_ledgers.validate().map_err(|source| {
            Wb11HydrologyKernelGuardError::SnowMassTransitionLedger {
                phase_class,
                source,
            }
        })?;
        let partition = DirectSnowLiquidPartition {
            active_snow_coupling,
            snow_density_model: inputs.snow_density_model,
            snow_coupling_signed_s_m: snow_coupling.signed_s,
            mass_transition_ledgers,
            hourly_routed_melt_m: snow_coupling.hourly_routed_melt,
            accumulation_m: snow_coupling.accumulation,
            rain_retained_m: snow_coupling.rain_retained,
            liquid_holding_capacity_after_m: snow_coupling.liquid_holding_capacity,
            liquid_water_retained_after_m: snow_coupling.liquid_water_retained,
            liquid_water_released_m: snow_coupling.liquid_water_released,
            sublimation_m: snow_coupling.sublimation + stage3_resolution.outcome.sublimation_m,
            post_winter_rain_m,
            runtime_swe_after_m,
            runtime_depth_after_m,
            runtime_density_after_kg_m3,
            runtime_settle_day_count_after: snow_coupling.runtime_settle_day_count,
            coe_boundary_depth_after_m,
            coe_boundary_density_after_kg_m3,
            coe_boundary_settle_day_count_after: snow_coupling.runtime_settle_day_count,
            density_swe_identity_residual_m: density_outcome.max_abs_swe_identity_residual_m,
            density_unbounded_swe_residual_m: density_outcome.max_abs_unbounded_swe_residual_m,
            density_process_diagnostics,
            verbose_diagnostics,
            snow_albedo_state_after: snow_coupling.snow_albedo_state_after,
            snow_layers_after,
        };
        Self::validate_direct_snow_storage_closure(phase_class, inputs, &partition)?;
        Ok(DirectSnowStage3EvaluationWithReconciliationResult {
            result: DirectSnowStage3EvaluationResult {
                authoritative: partition,
                evaluation: stage3_resolution.evaluation,
            },
            reconciliation: stage3_resolution.reconciliation,
        })
    }

    fn validate_direct_snow_storage_closure(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
        partition: &DirectSnowLiquidPartition,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        let typed_snowfall_swe_m = inputs
            .hourly
            .iter()
            .map(|hour| hour.snowfall_m * 0.1)
            .sum::<f64>();
        let residual_m = inputs.runtime_swe_m + typed_snowfall_swe_m + partition.rain_retained_m
            - partition
                .mass_transition_ledgers
                .solid_to_liquid()
                .snowpack_swe_loss_m
            - partition.sublimation_m
            - partition.runtime_swe_after_m;
        Self::validate_direct_snow_storage_residual(phase_class, residual_m)
    }

    fn validate_direct_snow_storage_residual(
        phase_class: HillslopeKernelPhaseClass,
        residual_m: f64,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        let symbol = || BoundarySymbol::from("snow.daily_storage_closure_residual_m");
        if !residual_m.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol: symbol(),
                value: residual_m,
            });
        }
        if residual_m.abs() > SNOW_SOLID_TO_LIQUID_CLOSURE_TOLERANCE_M {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: symbol(),
                value: residual_m,
                minimum: Some(-SNOW_SOLID_TO_LIQUID_CLOSURE_TOLERANCE_M),
                maximum: Some(SNOW_SOLID_TO_LIQUID_CLOSURE_TOLERANCE_M),
            });
        }
        Ok(())
    }

    fn inactive_snow_coupling_from_typed(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
        capture: DirectSnowDiagnosticCapture,
    ) -> Result<SnowCouplingOutcome, Wb11HydrologyKernelGuardError> {
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from(WB14_SYMBOL_SNOW_RUNTIME_SWE),
            inputs.runtime_swe_m,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from(SNOW_RUNTIME_DEPTH_M_SYMBOL),
            inputs.runtime_depth_m,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from(SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL),
            inputs.runtime_density_kg_m3,
            Some(0.0),
            Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from(SNOW_RUNTIME_SETTLE_DAY_COUNT_SYMBOL),
            inputs.runtime_settle_day_count,
            Some(0.0),
            None,
        )?;
        Ok(SnowCouplingOutcome {
            signed_s: 0.0,
            accumulation: 0.0,
            rain_retained: 0.0,
            rain_released: 0.0,
            liquid_holding_capacity: 0.0,
            liquid_water_retained: inputs.liquid_water_retained_m,
            liquid_water_released: 0.0,
            sublimation: 0.0,
            raw_melt: 0.0,
            redistributed_melt: 0.0,
            wet_compaction_liquid_input_m: 0.0,
            hourly_routed_melt: [0.0; 24],
            verbose_diagnostics: capture.is_verbose().then(|| {
                Box::new(SnowCouplingVerboseDiagnostics {
                    hourly_melt: [DirectSnowMeltHourDiagnostics::default(); 24],
                    hourly_trace: SnowHourlyTrace::default(),
                })
            }),
            snowpack_state_loss: 0.0,
            runtime_swe: inputs.runtime_swe_m,
            runtime_depth_m: inputs.runtime_depth_m,
            runtime_density_kg_m3: inputs.runtime_density_kg_m3,
            runtime_settle_day_count: inputs.runtime_settle_day_count,
            snow_albedo_state_after: inputs.snow_albedo_state,
        })
    }

    // The orchestration is intentionally linear so every conservation operand
    // remains visible in the same closure boundary.
    #[allow(clippy::too_many_lines)]
    fn resolve_typed_snow_density_outcome(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
        snow_coupling: &SnowCouplingOutcome,
    ) -> Result<SnowDensityRuntimeOutcome, Wb11HydrologyKernelGuardError> {
        let mean_air_temperature_c = inputs
            .hourly
            .iter()
            .map(|hour| hour.air_temperature_c)
            .sum::<f64>()
            / 24.0;
        update_snow_density_runtime_state(&SnowDensityRuntimeInputs {
            model: inputs.snow_density_model,
            prior_swe_m: inputs.runtime_swe_m,
            prior_depth_m: inputs.runtime_depth_m,
            prior_density_kg_m3: inputs.runtime_density_kg_m3,
            prior_settle_day_count: inputs.runtime_settle_day_count,
            prior_layers: inputs.snow_layers.clone(),
            boundary_swe_after_m: snow_coupling.runtime_swe,
            boundary_depth_after_m: snow_coupling.runtime_depth_m,
            boundary_density_after_kg_m3: snow_coupling.runtime_density_kg_m3,
            snow_input_m: snow_coupling.accumulation,
            liquid_for_compaction_m: snow_coupling.wet_compaction_liquid_input_m,
            mean_air_temperature_c,
            runtime_density_cap_kg_m3: SIMIMPL29_SNOW_DENSITY_CAP_KG_M3,
            sturm_climate_class: inputs.sturm_climate_class,
            sturm_day_of_year: inputs.sturm_day_of_year,
        })
        .map_err(|error| {
            Self::snow_density_guard_error(
                phase_class,
                &error,
                inputs.runtime_swe_m,
                inputs.runtime_depth_m,
                &inputs.snow_layers,
            )
        })
    }

    fn snow_density_guard_error(
        phase_class: HillslopeKernelPhaseClass,
        error: &SnowDensityError,
        prior_swe_m: f64,
        prior_depth_m: f64,
        prior_layers: &[DirectSnowLayerState],
    ) -> Wb11HydrologyKernelGuardError {
        match error {
            SnowDensityError::NonFiniteInput { symbol, value } => {
                Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                    phase_class,
                    symbol: BoundarySymbol::from(*symbol),
                    value: *value,
                }
            }
            SnowDensityError::OutOfRangeInput {
                symbol,
                value,
                minimum,
                maximum,
            } => Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(*symbol),
                value: *value,
                minimum: *minimum,
                maximum: *maximum,
            },
            SnowDensityError::MissingClimateClassAssignment { .. } => {
                Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                    phase_class,
                    symbol: BoundarySymbol::from("snow_climate_class"),
                }
            }
            SnowDensityError::MissingSturmDayOfYear { .. } => {
                Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                    phase_class,
                    symbol: BoundarySymbol::from("sturm2010_density_day_of_year"),
                }
            }
            SnowDensityError::MissingClimateClassDensityParameters { .. } => {
                Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                    phase_class,
                    symbol: BoundarySymbol::from("sturm2010_density_parameters"),
                }
            }
            SnowDensityError::LayerAggregateMismatch {
                symbol,
                value,
                expected,
            } => Wb11HydrologyKernelGuardError::SnowLayerAggregateMismatch(Box::new(
                SnowLayerAggregateMismatchError {
                    phase_class,
                    symbol,
                    value: *value,
                    expected: *expected,
                    prior_swe_m,
                    prior_depth_m,
                    prior_layers: prior_layers.to_vec(),
                },
            )),
            SnowDensityError::DiagnosticClosureViolation {
                residual_kg_m3,
                tolerance_kg_m3,
            } => Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("snow_density_process_closure_residual_kg_m3"),
                value: *residual_kg_m3,
                minimum: Some(-*tolerance_kg_m3),
                maximum: Some(*tolerance_kg_m3),
            },
        }
    }

    // Finiteness + range guard with the symbol name built only on failure;
    // see require_state_range_with.
    pub(crate) fn require_direct_typed_snow_value_with(
        phase_class: HillslopeKernelPhaseClass,
        symbol: impl Fn() -> BoundarySymbol,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if !value.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol: symbol(),
                value,
            });
        }
        Self::require_state_range_with(phase_class, symbol, value, minimum, maximum)
    }

    pub(crate) fn redistribute_daily_signed_snowmelt(
        hourly_state: &mut [SnowHourlyState],
    ) -> SnowMeltRedistributionOutcome {
        // SNOWSCI-S1: runtime snow storage is single-sourced from the depth/density
        // store, so routed snowpack melt must match the positive water-equivalent
        // loss already applied to that store. Negative raw melt remains available
        // through `melt_raw_m` diagnostics, but it cannot create a second SWE debit.
        let positive_melt_total_m = hourly_state
            .iter()
            .map(|hourly| hourly.melt_m.max(0.0))
            .sum::<f64>();

        if positive_melt_total_m <= WB11_ZERO_THRESHOLD {
            for hourly in hourly_state {
                hourly.melt_m = hourly.melt_m.max(0.0);
            }
            return SnowMeltRedistributionOutcome {
                routed_melt_total_m: positive_melt_total_m,
                snowpack_state_loss_m: positive_melt_total_m,
            };
        }

        for hourly in hourly_state {
            hourly.melt_m = hourly.melt_m.max(0.0);
        }
        SnowMeltRedistributionOutcome {
            routed_melt_total_m: positive_melt_total_m,
            snowpack_state_loss_m: positive_melt_total_m,
        }
    }

    pub(crate) fn normalize_non_negative_within_tolerance(value: f64) -> f64 {
        if (-WB11_ZERO_THRESHOLD..0.0).contains(&value) {
            return 0.0;
        }
        value
    }
}

#[cfg(test)]
mod cqr_row5_tests {
    use super::*;

    #[test]
    fn child1_term_coupling_020_exhaustion_fails_closed_with_complete_diagnostics() {
        let common = CoveredTerminalEndingSnowHintV1 {
            ice_kg_m2: 0.2,
            liquid_kg_m2: 0.01,
            cold_content_j_m2: -125.0,
            surface_temperature_c: -1.0e-9,
        };
        let mut previous = None;
        let mut four_component_break = false;
        let mut final_pair = None;
        let mut evidence = CaptureState::default();
        for iteration in 0..32_u32 {
            let next = CoveredTerminalEndingSnowHintV1 {
                surface_temperature_c: if iteration % 2 == 0 { -1.0e-9 } else { 1.0e-9 },
                ..common
            };
            if let Some(prior) = previous {
                let comparisons = terminal_coupling_comparisons(prior, next);
                assert!(comparisons[..3].iter().all(|comparison| comparison.4));
                assert!(!comparisons[3].4);
                four_component_break |= terminal_coupling_four_component_converged(prior, next);
                final_pair = Some((prior, next));
            }
            evidence.coupling_iterations.push(CapturedCouplingIteration {
                hook: TerminalCouplingIterationHook {
                    request: coupling_test_request(iteration, previous),
                    outgoing: next,
                    comparisons: previous.map(|prior| terminal_coupling_comparisons(prior, next)),
                    converged: previous.is_some_and(|prior| {
                        terminal_coupling_four_component_converged(prior, next)
                    }),
                },
                provider_ordinal: Some(u64::from(iteration)),
            });
            previous = Some(next);
        }

        assert!(!four_component_break, "all 32 iterations exhaust the live four-component loop");
        let (prior, next) = final_pair.expect("iteration pair");
        assert!(terminal_coupling_post_loop_three_component_converged(prior, next));
        assert!(!terminal_coupling_four_component_converged(prior, next));
        evidence.coupling_selections.push(TerminalCouplingSelectionHook {
            request: coupling_test_request(31, Some(prior)),
            reason: TerminalCouplingSelectionReason::IterationLoopExhausted,
            post_loop_three_component_check: true,
        });
        let result: Result<(), DirectSnowStage3EvaluationError> =
            Err(DirectSnowStage3EvaluationError::TerminalCustody(
                "covered terminal coupled trial nonconvergence",
            ));
        assert!(matches!(
            result,
            Err(DirectSnowStage3EvaluationError::TerminalCustody(
                "covered terminal coupled trial nonconvergence"
            ))
        ));
        assert_eq!(evidence.coupling_iterations.len(), 32);
        assert_eq!(evidence.coupling_selections.len(), 1);
        assert_eq!(
            evidence.coupling_selections[0].reason,
            TerminalCouplingSelectionReason::IterationLoopExhausted
        );
        assert!(evidence.selected_trials.is_empty());
    }

    fn coupling_test_request(
        coupling_iteration: u32,
        ending_snow_hint: Option<CoveredTerminalEndingSnowHintV1>,
    ) -> CoveredTerminalTrialRequestV1 {
        let support = TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(600_000_000))
            .expect("test support");
        let beginning_joint = CoveredTerminalJointTrialStateV1::try_new(
            JointTrialAuthorityV1 {
                source_owner_set_sha256: Digest32::from_bytes([1; 32]),
                lane_id: 1,
                source_snow_owner_sha256: Digest32::from_bytes([2; 32]),
                interval_index: 0,
                state_support: support,
                accepted_predecessors: Vec::new(),
            },
            BTreeMap::from([
                ("vegetation".to_owned(), vec![1]),
                ("snow".to_owned(), vec![2]),
                ("land_surface_energy".to_owned(), vec![3]),
                ("hydrology".to_owned(), vec![4]),
                ("bgc".to_owned(), vec![5]),
                ("soil_thermal".to_owned(), vec![6]),
                ("surface_liquid".to_owned(), vec![7]),
            ]),
        )
        .expect("test joint");
        CoveredTerminalTrialRequestV1 {
            lane_id: 1,
            support,
            role: CoveredTerminalTrialRoleV1::Full,
            attempt_ordinal: 0,
            coupling_iteration,
            ice_kg_m2: 0.2,
            liquid_kg_m2: 0.01,
            cold_content_j_m2: -125.0,
            surface_temperature_c: -1.0e-9,
            snow_depth_m: 0.02,
            snow_density_kg_m3: 100.0,
            ending_snow_hint,
            beginning_joint,
        }
    }

    #[test]
    fn eb04w2b_storage_guard_enforces_exact_tolerance_and_nonfinite_rejection() {
        let phase_class = HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;
        for residual_m in [
            -SNOW_SOLID_TO_LIQUID_CLOSURE_TOLERANCE_M,
            0.0,
            SNOW_SOLID_TO_LIQUID_CLOSURE_TOLERANCE_M,
        ] {
            Wb11HydrologyKernel::validate_direct_snow_storage_residual(phase_class, residual_m)
                .expect("exact-tolerance daily snow closure residual must be accepted");
        }

        for residual_m in [
            f64::from_bits(SNOW_SOLID_TO_LIQUID_CLOSURE_TOLERANCE_M.to_bits() + 1),
            -f64::from_bits(SNOW_SOLID_TO_LIQUID_CLOSURE_TOLERANCE_M.to_bits() + 1),
        ] {
            let error =
                Wb11HydrologyKernel::validate_direct_snow_storage_residual(phase_class, residual_m)
                    .expect_err("over-tolerance daily snow closure residual must fail closed");
            assert!(matches!(
                error,
                Wb11HydrologyKernelGuardError::StateSymbolOutOfRange { .. }
            ));
            assert_eq!(error.code(), "HKERNEL-WB14-RUNOFF-E-003");
            assert!(
                error
                    .to_string()
                    .contains("snow.daily_storage_closure_residual_m")
            );
        }

        let error =
            Wb11HydrologyKernel::validate_direct_snow_storage_residual(phase_class, f64::NAN)
                .expect_err("non-finite daily snow closure residual must fail closed");
        assert!(matches!(
            error,
            Wb11HydrologyKernelGuardError::NonFiniteStateSymbol { .. }
        ));
        assert_eq!(error.code(), "HKERNEL-WB14-RUNOFF-E-002");
        assert!(
            error
                .to_string()
                .contains("snow.daily_storage_closure_residual_m")
        );
    }

    #[test]
    fn eb04c_lower_volume_threshold_is_strict_on_native_swe() {
        let threshold = STAGE3_MINIMUM_RESOLVED_THERMAL_MASS_SWE_M;
        let just_below = f64::from_bits(threshold.to_bits() - 1);
        let just_above = f64::from_bits(threshold.to_bits() + 1);

        assert!(Wb11HydrologyKernel::stage3_lower_volume_is_subresolution_swe_m(just_below));
        assert!(!Wb11HydrologyKernel::stage3_lower_volume_is_subresolution_swe_m(threshold));
        assert!(!Wb11HydrologyKernel::stage3_lower_volume_is_subresolution_swe_m(just_above));
    }

    #[test]
    fn partial_sublimation_retains_mass_resolved_subnanometer_swe_remainder() {
        let original_mass_swe_m = 1.0e-6;
        let represented_remainder_swe_m = 5.0e-10;
        let requested_m = original_mass_swe_m - represented_remainder_swe_m;
        let mut layer = DirectSnowLayerState::new(original_mass_swe_m, 2.0e-6, 500.0, 8.0);
        layer.liquid_water_m = 2.0e-7;
        layer.refrozen_liquid_m = 1.0e-7;
        let mut layers = vec![layer];
        let original_cold_content_j_m2 = 2.1;
        let mut cold_content_by_layer = vec![original_cold_content_j_m2];
        let mut active_layer_count = 1;

        let (removed_m, exported_j_m2, removed_layer_count) =
            Wb11HydrologyKernel::remove_stage3_active_sublimation(
                requested_m,
                &mut layers,
                &mut cold_content_by_layer,
                &mut active_layer_count,
            );

        assert_eq!(layers.len(), 1);
        assert_eq!(active_layer_count, 1);
        assert_eq!(removed_layer_count, 0);
        assert!(snow_density_layer_has_resolved_mass(layers[0].mass_swe_m));
        assert!((layers[0].mass_swe_m - represented_remainder_swe_m).abs() <= 1.0e-18);
        assert!((removed_m + layers[0].mass_swe_m - original_mass_swe_m).abs() <= 1.0e-18);
        assert!(
            (exported_j_m2 + cold_content_by_layer[0] - original_cold_content_j_m2).abs()
                <= 1.0e-12
        );
        assert!((layers[0].liquid_water_m - 1.0e-10).abs() <= 1.0e-18);
        assert!((layers[0].refrozen_liquid_m - 5.0e-11).abs() <= 1.0e-18);
    }

    #[test]
    fn stage3_target_trim_preserves_coupled_mass_resolved_remainder() {
        let original_mass_swe_m = 2.0e-6;
        let represented_remainder_swe_m = 5.0e-10;
        let removal_m = original_mass_swe_m - represented_remainder_swe_m;
        let mut surface = DirectSnowLayerState::new(original_mass_swe_m, 4.0e-6, 500.0, 9.0);
        surface.temperature_c = -4.0;
        surface.liquid_water_m = 4.0e-7;
        surface.cold_content_j_m2 = 16.8;
        surface.refrozen_liquid_m = 2.0e-7;
        let lower = DirectSnowLayerState::new(0.1, 0.2, 500.0, 20.0);
        let target_swe_m = surface.mass_swe_m + lower.mass_swe_m - removal_m;
        let mut layers = vec![surface, lower];

        Wb11HydrologyKernel::adjust_stage3_layer_swe_to_target(
            &mut layers,
            target_swe_m,
            0.2,
            500.0,
            20.0,
        );

        assert_eq!(layers.len(), 2);
        let retained = layers[0];
        let retained_fraction = retained.mass_swe_m / original_mass_swe_m;
        assert!((retained.mass_swe_m - represented_remainder_swe_m).abs() <= 1.0e-15);
        assert!(snow_density_layer_has_resolved_mass(retained.mass_swe_m));
        assert!(
            (retained.liquid_water_m - surface.liquid_water_m * retained_fraction).abs() <= 1.0e-18
        );
        assert!(
            (retained.refrozen_liquid_m - surface.refrozen_liquid_m * retained_fraction).abs()
                <= 1.0e-18
        );
        assert!(
            (retained.cold_content_j_m2 - surface.cold_content_j_m2 * retained_fraction).abs()
                <= 1.0e-15
        );
        assert_eq!(
            retained.density_kg_m3.to_bits(),
            surface.density_kg_m3.to_bits()
        );
        assert_eq!(
            retained.temperature_c.to_bits(),
            surface.temperature_c.to_bits()
        );
        assert_eq!(
            retained.settle_day_count.to_bits(),
            surface.settle_day_count.to_bits()
        );
        let reconstructed_swe_m = layers.iter().map(|layer| layer.mass_swe_m).sum::<f64>();
        assert!((reconstructed_swe_m - target_swe_m).abs() <= 1.0e-15);
    }

    #[test]
    fn stage3_target_trim_continues_below_residual_tolerance_across_layers() {
        let mut removed = DirectSnowLayerState::new(2.0e-6, 4.0e-6, 500.0, 9.0);
        removed.liquid_water_m = 4.0e-7;
        removed.cold_content_j_m2 = 16.8;
        removed.refrozen_liquid_m = 2.0e-7;
        let mut retained = DirectSnowLayerState::new(2.0e-9, 4.0e-9, 500.0, 12.0);
        retained.temperature_c = -3.0;
        retained.liquid_water_m = 8.0e-10;
        retained.cold_content_j_m2 = 4.2e-3;
        retained.refrozen_liquid_m = 4.0e-10;
        let target_swe_m = 1.5e-9;
        let mut layers = vec![removed, retained];

        Wb11HydrologyKernel::adjust_stage3_layer_swe_to_target(
            &mut layers,
            target_swe_m,
            3.0e-9,
            500.0,
            12.0,
        );

        assert_eq!(layers.len(), 1);
        let result = layers[0];
        let retained_fraction = 0.75;
        assert!((result.mass_swe_m - target_swe_m).abs() <= 1.0e-18);
        assert!(snow_density_layer_has_resolved_mass(result.mass_swe_m));
        assert!(
            (result.liquid_water_m - retained.liquid_water_m * retained_fraction).abs() <= 1.0e-18
        );
        assert!(
            (result.refrozen_liquid_m - retained.refrozen_liquid_m * retained_fraction).abs()
                <= 1.0e-18
        );
        assert!(
            (result.cold_content_j_m2 - retained.cold_content_j_m2 * retained_fraction).abs()
                <= 1.0e-15
        );
        assert_eq!(
            result.density_kg_m3.to_bits(),
            retained.density_kg_m3.to_bits()
        );
        assert_eq!(
            result.temperature_c.to_bits(),
            retained.temperature_c.to_bits()
        );
        assert_eq!(
            result.settle_day_count.to_bits(),
            retained.settle_day_count.to_bits()
        );
    }

    #[test]
    fn snow_density_guard_error_maps_all_error_variants() {
        let phase_class = HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;
        let replay_layers = [DirectSnowLayerState::new(0.2, 0.4, 500.0, 2.0)];
        let cases = [
            SnowDensityError::NonFiniteInput {
                symbol: "row5.nonfinite",
                value: f64::NAN,
            },
            SnowDensityError::OutOfRangeInput {
                symbol: "row5.range",
                value: -1.0,
                minimum: Some(0.0),
                maximum: Some(1.0),
            },
            SnowDensityError::MissingClimateClassAssignment { model: "sturm2010" },
            SnowDensityError::MissingSturmDayOfYear { model: "sturm2010" },
            SnowDensityError::MissingClimateClassDensityParameters { class: "alpine" },
            SnowDensityError::LayerAggregateMismatch {
                symbol: "prior_layers.thickness_m",
                value: 0.4,
                expected: 0.5,
            },
            SnowDensityError::DiagnosticClosureViolation {
                residual_kg_m3: 2.0e-9,
                tolerance_kg_m3: 1.0e-9,
            },
        ];

        let mapped = cases
            .iter()
            .map(|error| {
                Wb11HydrologyKernel::snow_density_guard_error(
                    phase_class,
                    error,
                    0.2,
                    0.5,
                    &replay_layers,
                )
            })
            .collect::<Vec<_>>();

        assert!(matches!(
            mapped[0],
            Wb11HydrologyKernelGuardError::NonFiniteStateSymbol { .. }
        ));
        assert!(matches!(
            mapped[1],
            Wb11HydrologyKernelGuardError::StateSymbolOutOfRange { .. }
        ));
        assert!(matches!(
            mapped[2],
            Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol { .. }
        ));
        assert!(mapped[2].to_string().contains("snow_climate_class"));
        assert!(
            mapped[3]
                .to_string()
                .contains("sturm2010_density_day_of_year")
        );
        assert!(
            mapped[4]
                .to_string()
                .contains("sturm2010_density_parameters")
        );
        assert!(matches!(
            mapped[5],
            Wb11HydrologyKernelGuardError::SnowLayerAggregateMismatch(_)
        ));
        assert!(matches!(
            mapped[6],
            Wb11HydrologyKernelGuardError::StateSymbolOutOfRange { .. }
        ));
        if let Wb11HydrologyKernelGuardError::SnowLayerAggregateMismatch(snapshot) = &mapped[5] {
            assert!((snapshot.replay_value() - snapshot.value).abs() <= f64::EPSILON);
            assert!((snapshot.replay_value() - snapshot.expected).abs() > f64::EPSILON);
            assert!((snapshot.expected - snapshot.prior_depth_m).abs() <= f64::EPSILON);
            let replay_swe_m = snapshot
                .prior_layers
                .iter()
                .map(|layer| layer.mass_swe_m)
                .sum::<f64>();
            assert!((replay_swe_m - snapshot.prior_swe_m).abs() <= f64::EPSILON);
        }
        assert!(
            mapped[5]
                .to_string()
                .contains("prior_layers.thickness_m=0.4 does not match expected 0.5")
        );
    }
}
