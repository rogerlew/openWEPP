#[allow(clippy::wildcard_imports)]
use super::*;

pub(crate) const RELATIVE_ERROR_TOLERANCE: f64 = 1.0e-8;
pub(crate) const MASS_ABSOLUTE_TOLERANCE_KG_M2: f64 = 1.0e-9;
pub(crate) const ENERGY_ABSOLUTE_TOLERANCE_J_M2: f64 = 1.0e-6;
const MAXIMUM_REJECTIONS: u32 = 64;
const NANOSECONDS_PER_SECOND_F64: f64 = 1_000_000_000.0;
const MINIMUM_COVERED_CARRIER_NS: u128 = 60_000_000_000;
const MINIMUM_COVERED_CARRIER_SECONDS: f64 = 60.0;

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]
fn exact_grid_nanoseconds(
    seconds: f64,
    zero_is_admitted: bool,
) -> Result<u128, DirectSnowStage3EvaluationError> {
    let nanoseconds = seconds * NANOSECONDS_PER_SECOND_F64;
    if !seconds.is_finite()
        || seconds < 0.0
        || (!zero_is_admitted && seconds == 0.0)
        || !nanoseconds.is_finite()
        || nanoseconds.fract() != 0.0
        || nanoseconds >= u128::MAX as f64
    {
        return Err(DirectSnowStage3EvaluationError::TerminalNumerics(
            SnowTerminalNumericsFailure::BelowCarrierDomain,
        ));
    }
    let nanoseconds = nanoseconds as u128;
    if nanoseconds != 0 && nanoseconds % MINIMUM_COVERED_CARRIER_NS != 0 {
        return Err(DirectSnowStage3EvaluationError::TerminalNumerics(
            SnowTerminalNumericsFailure::BelowCarrierDomain,
        ));
    }
    Ok(nanoseconds)
}

#[allow(clippy::cast_precision_loss)]
fn seconds_from_nanoseconds(nanoseconds: u128) -> f64 {
    nanoseconds as f64 / NANOSECONDS_PER_SECOND_F64
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum SnowVaporDisposition {
    Deposition,
    Sublimation,
    None,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct BoundedSnowVaporTransfer {
    pub raw_mass_kg_m2: f64,
    pub actual_mass_kg_m2: f64,
    pub raw_latent_energy_j_m2: f64,
    pub actual_latent_energy_j_m2: f64,
    pub specific_latent_heat_j_kg: f64,
    pub disposition: SnowVaporDisposition,
}

impl BoundedSnowVaporTransfer {
    fn try_new(
        raw_mass_kg_m2: f64,
        raw_latent_energy_j_m2: f64,
        available_ice_kg_m2: f64,
    ) -> Result<Self, DirectSnowStage3EvaluationError> {
        if !raw_mass_kg_m2.is_finite()
            || !raw_latent_energy_j_m2.is_finite()
            || !available_ice_kg_m2.is_finite()
            || available_ice_kg_m2 < 0.0
        {
            return Err(DirectSnowStage3EvaluationError::TerminalNumerics(
                SnowTerminalNumericsFailure::DomainOrNonFinite,
            ));
        }
        if raw_mass_kg_m2 == 0.0 {
            if raw_latent_energy_j_m2 != 0.0 {
                return Err(DirectSnowStage3EvaluationError::TerminalNumerics(
                    SnowTerminalNumericsFailure::Closure,
                ));
            }
            return Ok(Self {
                raw_mass_kg_m2: 0.0,
                actual_mass_kg_m2: 0.0,
                raw_latent_energy_j_m2: 0.0,
                actual_latent_energy_j_m2: 0.0,
                specific_latent_heat_j_kg: 0.0,
                disposition: SnowVaporDisposition::None,
            });
        }
        let specific_latent_heat_j_kg = raw_latent_energy_j_m2 / raw_mass_kg_m2;
        if !specific_latent_heat_j_kg.is_finite() || specific_latent_heat_j_kg <= 0.0 {
            return Err(DirectSnowStage3EvaluationError::TerminalNumerics(
                SnowTerminalNumericsFailure::Closure,
            ));
        }
        let (actual_mass_kg_m2, disposition) = if raw_mass_kg_m2 > 0.0 {
            (raw_mass_kg_m2, SnowVaporDisposition::Deposition)
        } else {
            (
                -(-raw_mass_kg_m2).min(available_ice_kg_m2),
                SnowVaporDisposition::Sublimation,
            )
        };
        let actual_latent_energy_j_m2 = actual_mass_kg_m2 * specific_latent_heat_j_kg;
        if !actual_latent_energy_j_m2.is_finite() {
            return Err(DirectSnowStage3EvaluationError::TerminalNumerics(
                SnowTerminalNumericsFailure::DomainOrNonFinite,
            ));
        }
        Ok(Self {
            raw_mass_kg_m2,
            actual_mass_kg_m2,
            raw_latent_energy_j_m2,
            actual_latent_energy_j_m2,
            specific_latent_heat_j_kg,
            disposition,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(clippy::struct_field_names)]
pub(crate) struct TerminalState {
    pub ice_kg_m2: f64,
    pub liquid_kg_m2: f64,
    pub cold_content_j_m2: f64,
}

#[derive(Clone, Copy)]
#[allow(clippy::struct_field_names)]
pub(super) struct TerminalFluxIntegral {
    pub complete_energy_j_m2: f64,
    pub vapor_mass_exchange_kg_m2: f64,
    pub shortwave_energy_j_m2: f64,
    pub longwave_energy_j_m2: f64,
    pub sensible_energy_j_m2: f64,
    pub latent_energy_j_m2: f64,
    pub advected_energy_j_m2: f64,
    pub snow_soil_heat_energy_j_m2: f64,
    pub external_liquid_kg_m2: f64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[allow(clippy::struct_field_names)]
pub(crate) struct TerminalLedger {
    pub(super) complete_energy_j_m2: f64,
    pub(super) cold_energy_change_j_m2: f64,
    pub(super) refrozen_kg_m2: f64,
    pub(super) deposition_kg_m2: f64,
    pub(super) sublimation_kg_m2: f64,
    pub(super) melt_kg_m2: f64,
    pub(super) unallocated_energy_j_m2: f64,
    pub(super) shortwave_energy_j_m2: f64,
    pub(super) longwave_energy_j_m2: f64,
    pub(super) sensible_energy_j_m2: f64,
    pub(super) latent_energy_j_m2: f64,
    pub(super) advected_energy_j_m2: f64,
    pub(super) snow_soil_heat_energy_j_m2: f64,
    pub(super) external_liquid_kg_m2: f64,
}

impl TerminalLedger {
    pub(super) fn add(self, other: Self) -> Self {
        Self {
            complete_energy_j_m2: self.complete_energy_j_m2 + other.complete_energy_j_m2,
            cold_energy_change_j_m2: self.cold_energy_change_j_m2 + other.cold_energy_change_j_m2,
            refrozen_kg_m2: self.refrozen_kg_m2 + other.refrozen_kg_m2,
            deposition_kg_m2: self.deposition_kg_m2 + other.deposition_kg_m2,
            sublimation_kg_m2: self.sublimation_kg_m2 + other.sublimation_kg_m2,
            melt_kg_m2: self.melt_kg_m2 + other.melt_kg_m2,
            unallocated_energy_j_m2: self.unallocated_energy_j_m2 + other.unallocated_energy_j_m2,
            shortwave_energy_j_m2: self.shortwave_energy_j_m2 + other.shortwave_energy_j_m2,
            longwave_energy_j_m2: self.longwave_energy_j_m2 + other.longwave_energy_j_m2,
            sensible_energy_j_m2: self.sensible_energy_j_m2 + other.sensible_energy_j_m2,
            latent_energy_j_m2: self.latent_energy_j_m2 + other.latent_energy_j_m2,
            advected_energy_j_m2: self.advected_energy_j_m2 + other.advected_energy_j_m2,
            snow_soil_heat_energy_j_m2: self.snow_soil_heat_energy_j_m2
                + other.snow_soil_heat_energy_j_m2,
            external_liquid_kg_m2: self.external_liquid_kg_m2 + other.external_liquid_kg_m2,
        }
    }
}

impl From<TerminalState> for TerminalStateEvidence {
    fn from(value: TerminalState) -> Self {
        Self {
            ice_kg_m2: value.ice_kg_m2,
            liquid_kg_m2: value.liquid_kg_m2,
            cold_content_j_m2: value.cold_content_j_m2,
        }
    }
}

impl From<TerminalLedger> for TerminalLedgerEvidence {
    fn from(value: TerminalLedger) -> Self {
        Self {
            complete_energy_j_m2: value.complete_energy_j_m2,
            cold_energy_change_j_m2: value.cold_energy_change_j_m2,
            refrozen_kg_m2: value.refrozen_kg_m2,
            deposition_kg_m2: value.deposition_kg_m2,
            sublimation_kg_m2: value.sublimation_kg_m2,
            melt_kg_m2: value.melt_kg_m2,
            unallocated_energy_j_m2: value.unallocated_energy_j_m2,
            shortwave_energy_j_m2: value.shortwave_energy_j_m2,
            longwave_energy_j_m2: value.longwave_energy_j_m2,
            sensible_energy_j_m2: value.sensible_energy_j_m2,
            latent_energy_j_m2: value.latent_energy_j_m2,
            advected_energy_j_m2: value.advected_energy_j_m2,
            snow_soil_heat_energy_j_m2: value.snow_soil_heat_energy_j_m2,
            external_liquid_kg_m2: value.external_liquid_kg_m2,
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct TerminalTrial {
    pub(super) state: TerminalState,
    ledger: TerminalLedger,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TerminalAcceptedMicrostep<J> {
    pub relative_start_ns: u128,
    pub duration_ns: u128,
    /// Compatibility projection for the existing carrier callback/receipt DTO.
    pub relative_start_s: f64,
    /// Compatibility projection for the existing carrier callback/receipt DTO.
    pub duration_s: f64,
    pub beginning: TerminalState,
    pub ending: TerminalState,
    pub ledger: TerminalLedger,
    pub carrier_ending_joint: J,
    pub hydrology_ending_joint: J,
}

impl Wb11HydrologyKernel {
    #[allow(clippy::too_many_arguments)]
    fn terminal_result_from_trial(
        hour_index: usize,
        hour_offset_seconds: f64,
        requested_seconds: f64,
        start: TerminalState,
        state: TerminalState,
        ledger: TerminalLedger,
        requested_ns: u128,
        elapsed_ns: u128,
        event_bracket_lower_ns: u128,
        event_bracket_upper_ns: u128,
        event_bracket_lower_solid_kg_m2: f64,
        event_bracket_upper_solid_kg_m2: f64,
        lte_coarse: TerminalTrial,
        lte_fine: TerminalTrial,
        accepted_trials: u32,
        rejected_trials: u32,
        maximum_scaled_error: f64,
    ) -> Result<DirectSnowTerminalEventResult, DirectSnowStage3EvaluationError> {
        if elapsed_ns > requested_ns || event_bracket_lower_ns > event_bracket_upper_ns {
            return Err(DirectSnowStage3EvaluationError::TerminalNumerics(
                SnowTerminalNumericsFailure::Closure,
            ));
        }
        let elapsed = seconds_from_nanoseconds(elapsed_ns);
        let unevaluated = seconds_from_nanoseconds(requested_ns - elapsed_ns);
        let event_bracket_lower_seconds = seconds_from_nanoseconds(event_bracket_lower_ns);
        let event_bracket_upper_seconds = seconds_from_nanoseconds(event_bracket_upper_ns);
        let event_bracket_width_seconds =
            seconds_from_nanoseconds(event_bracket_upper_ns - event_bracket_lower_ns);
        let event_occurred = state.ice_kg_m2 <= 0.0;
        let solid_residual = start.ice_kg_m2 + ledger.refrozen_kg_m2 + ledger.deposition_kg_m2
            - ledger.sublimation_kg_m2
            - ledger.melt_kg_m2
            - state.ice_kg_m2;
        let liquid_residual = start.liquid_kg_m2 + ledger.external_liquid_kg_m2 + ledger.melt_kg_m2
            - ledger.refrozen_kg_m2
            - state.liquid_kg_m2;
        let energy_residual = ledger.complete_energy_j_m2
            - ledger.cold_energy_change_j_m2
            - STAGE3_LATENT_HEAT_FUSION_J_KG * ledger.melt_kg_m2
            + STAGE3_LATENT_HEAT_FUSION_J_KG * ledger.refrozen_kg_m2
            - ledger.unallocated_energy_j_m2;
        let mass_scale = start.ice_kg_m2
            + ledger.refrozen_kg_m2
            + ledger.deposition_kg_m2
            + ledger.sublimation_kg_m2
            + ledger.melt_kg_m2
            + state.ice_kg_m2
            + start.liquid_kg_m2
            + ledger.external_liquid_kg_m2
            + state.liquid_kg_m2;
        let closure_mass_tolerance = 1.0e-12_f64.max(1.0e-12 * mass_scale);
        if solid_residual.abs() > closure_mass_tolerance
            || liquid_residual.abs() > closure_mass_tolerance
        {
            return Err(DirectSnowStage3EvaluationError::TerminalNumerics(
                SnowTerminalNumericsFailure::Closure,
            ));
        }
        let energy_scale = ledger.complete_energy_j_m2.abs()
            + ledger.cold_energy_change_j_m2.abs()
            + STAGE3_LATENT_HEAT_FUSION_J_KG * (ledger.melt_kg_m2 + ledger.refrozen_kg_m2)
            + ledger.unallocated_energy_j_m2.abs();
        if energy_residual.abs() > 1.0e-6_f64.max(1.0e-12 * energy_scale) {
            return Err(DirectSnowStage3EvaluationError::TerminalNumerics(
                SnowTerminalNumericsFailure::Closure,
            ));
        }
        Ok(DirectSnowTerminalEventResult {
            model: DirectSnowTerminalEventModel::EnthalpyEventV1,
            event_occurred,
            hour_index,
            terminal_entry_offset_seconds: hour_offset_seconds,
            requested_seconds,
            entry_solid_precipitation_kg_m2: 0.0,
            hour_offset_seconds: hour_offset_seconds + elapsed,
            evaluated_seconds: elapsed,
            unevaluated_seconds: unevaluated,
            start_ice_kg_m2: start.ice_kg_m2,
            start_liquid_kg_m2: start.liquid_kg_m2,
            start_cold_content_j_m2: start.cold_content_j_m2,
            end_ice_kg_m2: state.ice_kg_m2,
            terminal_liquid_kg_m2: state.liquid_kg_m2,
            end_cold_content_j_m2: state.cold_content_j_m2,
            complete_energy_j_m2: ledger.complete_energy_j_m2,
            shortwave_energy_j_m2: ledger.shortwave_energy_j_m2,
            longwave_energy_j_m2: ledger.longwave_energy_j_m2,
            sensible_energy_j_m2: ledger.sensible_energy_j_m2,
            latent_energy_j_m2: ledger.latent_energy_j_m2,
            advected_energy_j_m2: ledger.advected_energy_j_m2,
            snow_soil_heat_energy_j_m2: ledger.snow_soil_heat_energy_j_m2,
            external_liquid_kg_m2: ledger.external_liquid_kg_m2,
            cold_energy_change_j_m2: ledger.cold_energy_change_j_m2,
            refrozen_kg_m2: ledger.refrozen_kg_m2,
            deposition_kg_m2: ledger.deposition_kg_m2,
            sublimation_kg_m2: ledger.sublimation_kg_m2,
            melt_kg_m2: ledger.melt_kg_m2,
            terminal_unallocated_energy_j_m2: ledger.unallocated_energy_j_m2,
            solid_mass_closure_residual_kg_m2: solid_residual,
            liquid_mass_closure_residual_kg_m2: liquid_residual,
            energy_closure_residual_j_m2: energy_residual,
            event_bracket_width_seconds,
            event_bracket_lower_seconds,
            event_bracket_upper_seconds,
            event_bracket_lower_solid_kg_m2,
            event_bracket_upper_solid_kg_m2,
            lte_coarse_ice_kg_m2: lte_coarse.state.ice_kg_m2,
            lte_fine_ice_kg_m2: lte_fine.state.ice_kg_m2,
            lte_coarse_liquid_kg_m2: lte_coarse.state.liquid_kg_m2,
            lte_fine_liquid_kg_m2: lte_fine.state.liquid_kg_m2,
            lte_coarse_cold_content_j_m2: lte_coarse.state.cold_content_j_m2,
            lte_fine_cold_content_j_m2: lte_fine.state.cold_content_j_m2,
            lte_coarse_complete_energy_j_m2: lte_coarse.ledger.complete_energy_j_m2,
            lte_fine_complete_energy_j_m2: lte_fine.ledger.complete_energy_j_m2,
            lte_coarse_unallocated_energy_j_m2: lte_coarse.ledger.unallocated_energy_j_m2,
            lte_fine_unallocated_energy_j_m2: lte_fine.ledger.unallocated_energy_j_m2,
            accepted_trials,
            rejected_trials,
            maximum_scaled_error,
        })
    }

    #[cfg(test)]
    fn terminal_prefix_candidate<F, G, J>(
        start: TerminalState,
        initial_joint: &J,
        candidate_seconds: f64,
        attempt_ordinal: &mut u32,
        flux_integral: &mut F,
        join_hydrology_ending: &mut G,
    ) -> Result<(TerminalTrial, J), DirectSnowStage3EvaluationError>
    where
        F: FnMut(
            TerminalState,
            TerminalLedger,
            usize,
            &J,
            f64,
            f64,
            CoveredTerminalTrialRoleV1,
            u32,
        ) -> Result<(TerminalFluxIntegral, J), DirectSnowStage3EvaluationError>,
        G: FnMut(TerminalState, J) -> Result<J, DirectSnowStage3EvaluationError>,
        J: Clone,
    {
        if candidate_seconds == 0.0 {
            return Ok((
                TerminalTrial {
                    state: start,
                    ledger: TerminalLedger::default(),
                },
                initial_joint.clone(),
            ));
        }
        let candidate_ns = exact_grid_nanoseconds(candidate_seconds, false)?;
        let candidate_seconds = seconds_from_nanoseconds(candidate_ns);
        let (flux, carrier_joint) = flux_integral(
            start,
            TerminalLedger::default(),
            0,
            initial_joint,
            0.0,
            candidate_seconds,
            CoveredTerminalTrialRoleV1::Root,
            *attempt_ordinal,
        )?;
        *attempt_ordinal = attempt_ordinal.checked_add(1).ok_or(
            DirectSnowStage3EvaluationError::TerminalNumerics(
                SnowTerminalNumericsFailure::DomainOrNonFinite,
            ),
        )?;
        let transition = Self::terminal_transition(start, flux)?;
        let state = transition.state;
        let ledger = transition.ledger;
        let joint = join_hydrology_ending(state, carrier_joint)?;
        Ok((TerminalTrial { state, ledger }, joint))
    }

    pub(super) fn terminal_transition(
        start: TerminalState,
        flux: TerminalFluxIntegral,
    ) -> Result<TerminalTrial, DirectSnowStage3EvaluationError> {
        if !start.ice_kg_m2.is_finite()
            || !start.liquid_kg_m2.is_finite()
            || !start.cold_content_j_m2.is_finite()
            || start.ice_kg_m2 < 0.0
            || start.liquid_kg_m2 < 0.0
            || start.cold_content_j_m2 < 0.0
            || !flux.complete_energy_j_m2.is_finite()
            || !flux.shortwave_energy_j_m2.is_finite()
            || !flux.longwave_energy_j_m2.is_finite()
            || !flux.sensible_energy_j_m2.is_finite()
            || !flux.advected_energy_j_m2.is_finite()
            || !flux.snow_soil_heat_energy_j_m2.is_finite()
            || !flux.external_liquid_kg_m2.is_finite()
            || flux.external_liquid_kg_m2 < 0.0
        {
            return Err(DirectSnowStage3EvaluationError::TerminalNumerics(
                SnowTerminalNumericsFailure::DomainOrNonFinite,
            ));
        }
        let vapor = BoundedSnowVaporTransfer::try_new(
            flux.vapor_mass_exchange_kg_m2,
            flux.latent_energy_j_m2,
            start.ice_kg_m2,
        )?;
        let complete_energy_j_m2 =
            flux.complete_energy_j_m2 - flux.latent_energy_j_m2 + vapor.actual_latent_energy_j_m2;
        if !complete_energy_j_m2.is_finite() {
            return Err(DirectSnowStage3EvaluationError::TerminalNumerics(
                SnowTerminalNumericsFailure::DomainOrNonFinite,
            ));
        }
        let component_energy_j_m2 = flux.shortwave_energy_j_m2
            + flux.longwave_energy_j_m2
            + flux.sensible_energy_j_m2
            + vapor.actual_latent_energy_j_m2
            + flux.advected_energy_j_m2
            + flux.snow_soil_heat_energy_j_m2;
        let component_energy_scale = complete_energy_j_m2.abs()
            + flux.shortwave_energy_j_m2.abs()
            + flux.longwave_energy_j_m2.abs()
            + flux.sensible_energy_j_m2.abs()
            + vapor.actual_latent_energy_j_m2.abs()
            + flux.advected_energy_j_m2.abs()
            + flux.snow_soil_heat_energy_j_m2.abs();
        if (complete_energy_j_m2 - component_energy_j_m2).abs()
            > ENERGY_ABSOLUTE_TOLERANCE_J_M2.max(1.0e-12 * component_energy_scale)
        {
            return Err(DirectSnowStage3EvaluationError::TerminalNumerics(
                SnowTerminalNumericsFailure::Closure,
            ));
        }
        let deposition_kg_m2 = vapor.actual_mass_kg_m2.max(0.0);
        let sublimation_kg_m2 = (-vapor.actual_mass_kg_m2).max(0.0);
        let water_kg_m2 = start.ice_kg_m2 + start.liquid_kg_m2 + deposition_kg_m2
            - sublimation_kg_m2
            + flux.external_liquid_kg_m2;
        let enthalpy_j_m2 = -start.cold_content_j_m2
            + STAGE3_LATENT_HEAT_FUSION_J_KG * (start.liquid_kg_m2 + flux.external_liquid_kg_m2)
            + complete_energy_j_m2;
        if !water_kg_m2.is_finite() || water_kg_m2 < 0.0 || !enthalpy_j_m2.is_finite() {
            return Err(DirectSnowStage3EvaluationError::TerminalNumerics(
                SnowTerminalNumericsFailure::DomainOrNonFinite,
            ));
        }
        let fusion_capacity_j_m2 = STAGE3_LATENT_HEAT_FUSION_J_KG * water_kg_m2;
        let (ice_kg_m2, liquid_kg_m2, cold_content_j_m2, unallocated_energy_j_m2) =
            if enthalpy_j_m2 < 0.0 {
                (water_kg_m2, 0.0, -enthalpy_j_m2, 0.0)
            } else if enthalpy_j_m2 < fusion_capacity_j_m2 {
                let liquid_kg_m2 = enthalpy_j_m2 / STAGE3_LATENT_HEAT_FUSION_J_KG;
                (water_kg_m2 - liquid_kg_m2, liquid_kg_m2, 0.0, 0.0)
            } else {
                (
                    0.0,
                    water_kg_m2,
                    0.0,
                    (enthalpy_j_m2 - fusion_capacity_j_m2).max(0.0),
                )
            };
        let liquid_pre_kg_m2 = start.liquid_kg_m2 + flux.external_liquid_kg_m2;
        let melt_kg_m2 = (liquid_kg_m2 - liquid_pre_kg_m2).max(0.0);
        let refrozen_kg_m2 = (liquid_pre_kg_m2 - liquid_kg_m2).max(0.0);
        let state = TerminalState {
            ice_kg_m2,
            liquid_kg_m2,
            cold_content_j_m2,
        };
        let ledger = TerminalLedger {
            complete_energy_j_m2,
            cold_energy_change_j_m2: start.cold_content_j_m2 - cold_content_j_m2,
            refrozen_kg_m2,
            deposition_kg_m2,
            sublimation_kg_m2,
            melt_kg_m2,
            unallocated_energy_j_m2,
            shortwave_energy_j_m2: flux.shortwave_energy_j_m2,
            longwave_energy_j_m2: flux.longwave_energy_j_m2,
            sensible_energy_j_m2: flux.sensible_energy_j_m2,
            latent_energy_j_m2: vapor.actual_latent_energy_j_m2,
            advected_energy_j_m2: flux.advected_energy_j_m2,
            snow_soil_heat_energy_j_m2: flux.snow_soil_heat_energy_j_m2,
            external_liquid_kg_m2: flux.external_liquid_kg_m2,
        };
        if (state.ice_kg_m2 > 0.0 && ledger.unallocated_energy_j_m2 > 0.0)
            || vapor.disposition == SnowVaporDisposition::None
                && (vapor.actual_mass_kg_m2 != 0.0 || vapor.actual_latent_energy_j_m2 != 0.0)
        {
            return Err(DirectSnowStage3EvaluationError::TerminalNumerics(
                SnowTerminalNumericsFailure::Closure,
            ));
        }
        Ok(TerminalTrial { state, ledger })
    }

    #[cfg(test)]
    fn terminal_phase_complementarity_transition(
        start: TerminalState,
        flux: TerminalFluxIntegral,
    ) -> Result<TerminalTrial, DirectSnowStage3EvaluationError> {
        let deposition_kg_m2 = flux.vapor_mass_exchange_kg_m2.max(0.0);
        let sublimation_kg_m2 = (-flux.vapor_mass_exchange_kg_m2)
            .max(0.0)
            .min(start.ice_kg_m2);
        let candidate = crate::snow_terminal_phase_competition::simultaneous_complementarity(
            crate::snow_terminal_phase_competition::TerminalPhaseInputs {
                beginning_pack_ice_kg_m2: start.ice_kg_m2,
                beginning_surface_frost_kg_m2: 0.0,
                beginning_liquid_kg_m2: start.liquid_kg_m2,
                beginning_cold_content_j_m2: start.cold_content_j_m2,
                deposition_kg_m2,
                sublimation_kg_m2,
                external_liquid_kg_m2: flux.external_liquid_kg_m2,
                non_vapor_energy_j_m2: flux.complete_energy_j_m2 - flux.latent_energy_j_m2,
                vapor_latent_energy_j_m2: flux.latent_energy_j_m2,
                complete_energy_j_m2: flux.complete_energy_j_m2,
                support_seconds: 1.0,
            },
        )
        .map_err(|_| {
            DirectSnowStage3EvaluationError::TerminalNumerics(SnowTerminalNumericsFailure::Closure)
        })?;
        Ok(TerminalTrial {
            state: TerminalState {
                ice_kg_m2: candidate.ending_pack_ice_kg_m2,
                liquid_kg_m2: candidate.ending_liquid_kg_m2,
                cold_content_j_m2: candidate.ending_cold_content_j_m2,
            },
            ledger: TerminalLedger {
                complete_energy_j_m2: flux.complete_energy_j_m2,
                cold_energy_change_j_m2: start.cold_content_j_m2
                    - candidate.ending_cold_content_j_m2,
                refrozen_kg_m2: candidate.refrozen_kg_m2,
                deposition_kg_m2,
                sublimation_kg_m2,
                melt_kg_m2: candidate.melt_kg_m2,
                unallocated_energy_j_m2: candidate.unallocated_energy_j_m2,
                shortwave_energy_j_m2: flux.shortwave_energy_j_m2,
                longwave_energy_j_m2: flux.longwave_energy_j_m2,
                sensible_energy_j_m2: flux.sensible_energy_j_m2,
                latent_energy_j_m2: flux.latent_energy_j_m2,
                advected_energy_j_m2: flux.advected_energy_j_m2,
                snow_soil_heat_energy_j_m2: flux.snow_soil_heat_energy_j_m2,
                external_liquid_kg_m2: flux.external_liquid_kg_m2,
            },
        })
    }

    fn terminal_scaled_error(full: TerminalTrial, refined: TerminalTrial) -> f64 {
        let components = Self::terminal_error_components(full, refined);
        components[1..]
            .iter()
            .fold(components[0].4, |maximum, component| {
                maximum.max(component.4)
            })
    }

    fn terminal_error_components(
        full: TerminalTrial,
        refined: TerminalTrial,
    ) -> [(f64, f64, f64, f64, f64); 5] {
        let component = |coarse: f64, fine: f64, absolute: f64| {
            let delta = fine - coarse;
            let denominator = absolute + RELATIVE_ERROR_TOLERANCE * coarse.abs().max(fine.abs());
            (coarse, fine, delta, denominator, delta.abs() / denominator)
        };
        [
            component(
                full.state.ice_kg_m2,
                refined.state.ice_kg_m2,
                MASS_ABSOLUTE_TOLERANCE_KG_M2,
            ),
            component(
                full.state.liquid_kg_m2,
                refined.state.liquid_kg_m2,
                MASS_ABSOLUTE_TOLERANCE_KG_M2,
            ),
            component(
                full.state.cold_content_j_m2,
                refined.state.cold_content_j_m2,
                ENERGY_ABSOLUTE_TOLERANCE_J_M2,
            ),
            component(
                full.ledger.complete_energy_j_m2,
                refined.ledger.complete_energy_j_m2,
                ENERGY_ABSOLUTE_TOLERANCE_J_M2,
            ),
            component(
                full.ledger.unallocated_energy_j_m2,
                refined.ledger.unallocated_energy_j_m2,
                ENERGY_ABSOLUTE_TOLERANCE_J_M2,
            ),
        ]
    }

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    #[cfg(test)]
    pub(super) fn solve_terminal_enthalpy_event<F, G, J>(
        phase_class: HillslopeKernelPhaseClass,
        hour_index: usize,
        hour_offset_seconds: f64,
        requested_seconds: f64,
        start: TerminalState,
        initial_joint: J,
        flux_integral: F,
        join_hydrology_ending: G,
    ) -> Result<(DirectSnowTerminalEventResult, J), DirectSnowStage3EvaluationError>
    where
        F: FnMut(
            TerminalState,
            TerminalLedger,
            usize,
            &J,
            f64,
            f64,
            CoveredTerminalTrialRoleV1,
            u32,
        ) -> Result<(TerminalFluxIntegral, J), DirectSnowStage3EvaluationError>,
        G: FnMut(TerminalState, J) -> Result<J, DirectSnowStage3EvaluationError>,
        J: Clone,
    {
        let mut evidence = <NoEvidence as TerminalEvidenceMode<J>>::new_state();
        let (result, joint, _) =
            Self::solve_terminal_enthalpy_event_with_evidence::<F, G, J, NoEvidence>(
                phase_class,
                hour_index,
                hour_offset_seconds,
                requested_seconds,
                start,
                initial_joint,
                flux_integral,
                join_hydrology_ending,
                false,
                false,
                &mut evidence,
            )?;
        Ok((result, joint))
    }

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    pub(super) fn solve_terminal_enthalpy_event_with_evidence<F, G, J, M>(
        _phase_class: HillslopeKernelPhaseClass,
        hour_index: usize,
        hour_offset_seconds: f64,
        requested_seconds: f64,
        start: TerminalState,
        initial_joint: J,
        mut flux_integral: F,
        mut join_hydrology_ending: G,
        discrete_complete_endpoint: bool,
        phase_complementarity_endpoint: bool,
        evidence: &mut M::State,
    ) -> Result<
        (
            DirectSnowTerminalEventResult,
            J,
            Vec<TerminalAcceptedMicrostep<J>>,
        ),
        DirectSnowStage3EvaluationError,
    >
    where
        F: FnMut(
            TerminalState,
            TerminalLedger,
            usize,
            &J,
            f64,
            f64,
            CoveredTerminalTrialRoleV1,
            u32,
        ) -> Result<(TerminalFluxIntegral, J), DirectSnowStage3EvaluationError>,
        G: FnMut(TerminalState, J) -> Result<J, DirectSnowStage3EvaluationError>,
        J: Clone,
        M: TerminalEvidenceMode<J>,
    {
        if !requested_seconds.is_finite()
            || requested_seconds <= 0.0
            || !start.ice_kg_m2.is_finite()
            || !start.liquid_kg_m2.is_finite()
            || !start.cold_content_j_m2.is_finite()
            || start.ice_kg_m2 <= 0.0
            || start.ice_kg_m2 > 1.0
            || start.liquid_kg_m2 < 0.0
            || start.cold_content_j_m2 < 0.0
        {
            return Err(DirectSnowStage3EvaluationError::TerminalNumerics(
                SnowTerminalNumericsFailure::DomainOrNonFinite,
            ));
        }
        let requested_ns = exact_grid_nanoseconds(requested_seconds, false)?;
        let requested_duration_seconds = seconds_from_nanoseconds(requested_ns);
        let total_quanta = requested_ns / MINIMUM_COVERED_CARRIER_NS;
        if discrete_complete_endpoint {
            let (flux, carrier_joint) = flux_integral(
                start,
                TerminalLedger::default(),
                0,
                &initial_joint,
                0.0,
                requested_duration_seconds,
                CoveredTerminalTrialRoleV1::Root,
                0,
            )?;
            #[cfg(test)]
            let trial = if phase_complementarity_endpoint {
                Self::terminal_phase_complementarity_transition(start, flux)?
            } else {
                Self::terminal_transition(start, flux)?
            };
            #[cfg(not(test))]
            let trial = Self::terminal_transition(start, flux)?;
            let joint = join_hydrology_ending(trial.state, carrier_joint.clone())?;
            let result = Self::terminal_result_from_trial(
                hour_index,
                hour_offset_seconds,
                requested_seconds,
                start,
                trial.state,
                trial.ledger,
                requested_ns,
                requested_ns,
                requested_ns,
                requested_ns,
                trial.state.ice_kg_m2,
                trial.state.ice_kg_m2,
                trial,
                trial,
                1,
                0,
                0.0,
            )?;
            return Ok((
                result,
                joint.clone(),
                vec![TerminalAcceptedMicrostep {
                    relative_start_ns: 0,
                    duration_ns: requested_ns,
                    relative_start_s: 0.0,
                    duration_s: requested_duration_seconds,
                    beginning: start,
                    ending: trial.state,
                    ledger: trial.ledger,
                    carrier_ending_joint: carrier_joint,
                    hydrology_ending_joint: joint,
                }],
            ));
        }
        let _ = (discrete_complete_endpoint, phase_complementarity_endpoint);
        let mut state = start;
        let mut accepted_joint = initial_joint.clone();
        let mut ledger = TerminalLedger::default();
        let mut elapsed_quanta = 0_u128;
        // Begin from the complete remaining support. Rejection and halving
        // remain result-blind and retain the same 60-second equations.
        let mut trial_quanta = total_quanta;
        let mut accepted_trials = 0_u32;
        let mut rejected_trials = 0_u32;
        let mut consecutive_rejections = 0_u32;
        let mut maximum_scaled_error: f64 = 0.0;
        let mut attempt_ordinal = 0_u32;
        let mut accepted_microsteps = Vec::new();
        let next_attempt = |value: u32| {
            value
                .checked_add(1)
                .ok_or(DirectSnowStage3EvaluationError::TerminalNumerics(
                    SnowTerminalNumericsFailure::DomainOrNonFinite,
                ))
        };
        let mut event_bracket_lower_ns = 0_u128;
        let mut event_bracket_upper_ns = 0_u128;
        let mut event_bracket_lower_solid_kg_m2 = start.ice_kg_m2;
        let mut event_bracket_upper_solid_kg_m2 = start.ice_kg_m2;
        let mut lte_coarse = TerminalTrial {
            state: start,
            ledger: TerminalLedger::default(),
        };
        let mut lte_fine = lte_coarse;
        while elapsed_quanta < total_quanta && state.ice_kg_m2 > 0.0 {
            let remaining_quanta = total_quanta - elapsed_quanta;
            let candidate_quanta = trial_quanta.min(remaining_quanta);
            let elapsed_ns = elapsed_quanta * MINIMUM_COVERED_CARRIER_NS;
            let candidate_ns = candidate_quanta * MINIMUM_COVERED_CARRIER_NS;
            let elapsed = seconds_from_nanoseconds(elapsed_ns);
            let dt = seconds_from_nanoseconds(candidate_ns);
            let full_role = if consecutive_rejections == 0 {
                CoveredTerminalTrialRoleV1::Full
            } else {
                CoveredTerminalTrialRoleV1::Retry
            };
            let full_attempt = attempt_ordinal;
            let (full_flux, full_carrier_joint) = flux_integral(
                state,
                ledger,
                accepted_microsteps.len(),
                &accepted_joint,
                elapsed,
                dt,
                full_role,
                attempt_ordinal,
            )?;
            attempt_ordinal = next_attempt(attempt_ordinal)?;
            let full = Self::terminal_transition(state, full_flux)?;
            let accepted_full_carrier_joint = full_carrier_joint.clone();
            let (full_joint, captured_full_carrier_joint) = if M::ENABLED {
                (
                    join_hydrology_ending(full.state, full_carrier_joint.clone())?,
                    Some(full_carrier_joint),
                )
            } else {
                (join_hydrology_ending(full.state, full_carrier_joint)?, None)
            };
            if candidate_quanta > 1 {
                if let Some(carrier_joint) = captured_full_carrier_joint.as_ref() {
                    M::selected_trial(
                        evidence,
                        TerminalSelectedTrialHook {
                            position: TerminalPairPosition::Coarse,
                            role: full_role,
                            attempt_ordinal: full_attempt,
                            relative_start_s: elapsed,
                            duration_s: dt,
                            beginning: state.into(),
                            ending: full.state.into(),
                            ledger: full.ledger.into(),
                            beginning_joint: &accepted_joint,
                            carrier_ending_joint: carrier_joint,
                            hydrology_ending_joint: &full_joint,
                        },
                    );
                }
            }
            if candidate_quanta == 1 {
                let calls = M::provider_call_count(evidence);
                M::admission(
                    evidence,
                    TerminalAdmissionEvidenceHook {
                        proposed_duration_s: dt,
                        required_half_duration_s: 0.0,
                        minimum_duration_s: MINIMUM_COVERED_CARRIER_SECONDS,
                        decision: TerminalFloorDecision::Accepted,
                        provider_calls_before: calls,
                        provider_calls_after: calls,
                    },
                );
                let accepted_beginning = state;
                state = full.state;
                accepted_joint = full_joint;
                accepted_microsteps.push(TerminalAcceptedMicrostep {
                    relative_start_ns: elapsed_ns,
                    duration_ns: candidate_ns,
                    relative_start_s: elapsed,
                    duration_s: dt,
                    beginning: accepted_beginning,
                    ending: full.state,
                    ledger: full.ledger,
                    carrier_ending_joint: accepted_full_carrier_joint,
                    hydrology_ending_joint: accepted_joint.clone(),
                });
                ledger = ledger.add(full.ledger);
                elapsed_quanta += 1;
                accepted_trials = next_attempt(accepted_trials)?;
                consecutive_rejections = 0;
                lte_coarse = full;
                lte_fine = full;
                event_bracket_lower_ns = elapsed_ns;
                event_bracket_upper_ns = elapsed_ns + candidate_ns;
                event_bracket_lower_solid_kg_m2 = start.ice_kg_m2;
                event_bracket_upper_solid_kg_m2 = state.ice_kg_m2;
                if state.ice_kg_m2 <= 0.0 {
                    event_bracket_lower_ns = elapsed_ns + candidate_ns;
                    event_bracket_upper_ns = elapsed_ns + candidate_ns;
                    event_bracket_lower_solid_kg_m2 = 0.0;
                    event_bracket_upper_solid_kg_m2 = 0.0;
                    break;
                }
                trial_quanta = (candidate_quanta * 2).min(total_quanta - elapsed_quanta);
                continue;
            }
            let first_quanta = candidate_quanta / 2;
            let second_quanta = candidate_quanta - first_quanta;
            let first_ns = first_quanta * MINIMUM_COVERED_CARRIER_NS;
            let second_ns = second_quanta * MINIMUM_COVERED_CARRIER_NS;
            let first_dt = seconds_from_nanoseconds(first_ns);
            let second_dt = seconds_from_nanoseconds(second_ns);
            let first_attempt = attempt_ordinal;
            let (first_flux, first_carrier_joint) = flux_integral(
                state,
                ledger,
                accepted_microsteps.len(),
                &accepted_joint,
                elapsed,
                first_dt,
                CoveredTerminalTrialRoleV1::Half1,
                attempt_ordinal,
            )?;
            attempt_ordinal = next_attempt(attempt_ordinal)?;
            let first = Self::terminal_transition(state, first_flux)?;
            let accepted_first_carrier_joint = first_carrier_joint.clone();
            let (first_joint, captured_first_carrier_joint) = if M::ENABLED {
                (
                    join_hydrology_ending(first.state, first_carrier_joint.clone())?,
                    Some(first_carrier_joint),
                )
            } else {
                (
                    join_hydrology_ending(first.state, first_carrier_joint)?,
                    None,
                )
            };
            if let Some(carrier_joint) = captured_first_carrier_joint.as_ref() {
                M::selected_trial(
                    evidence,
                    TerminalSelectedTrialHook {
                        position: TerminalPairPosition::Fine1,
                        role: CoveredTerminalTrialRoleV1::Half1,
                        attempt_ordinal: first_attempt,
                        relative_start_s: elapsed,
                        duration_s: first_dt,
                        beginning: state.into(),
                        ending: first.state.into(),
                        ledger: first.ledger.into(),
                        beginning_joint: &accepted_joint,
                        carrier_ending_joint: carrier_joint,
                        hydrology_ending_joint: &first_joint,
                    },
                );
            }
            if first.state.ice_kg_m2 <= 0.0 {
                rejected_trials = next_attempt(rejected_trials)?;
                consecutive_rejections = next_attempt(consecutive_rejections)?;
                trial_quanta = first_quanta;
                if consecutive_rejections > MAXIMUM_REJECTIONS {
                    return Err(DirectSnowStage3EvaluationError::TerminalNumerics(
                        SnowTerminalNumericsFailure::RejectionLimit,
                    ));
                }
                continue;
            }
            let second_attempt = attempt_ordinal;
            let (second_flux, second_carrier_joint) = flux_integral(
                first.state,
                ledger.add(first.ledger),
                accepted_microsteps.len() + 1,
                &first_joint,
                elapsed + first_dt,
                second_dt,
                CoveredTerminalTrialRoleV1::Half2,
                attempt_ordinal,
            )?;
            let second = Self::terminal_transition(first.state, second_flux)?;
            let accepted_second_carrier_joint = second_carrier_joint.clone();
            let (second_joint, captured_second_carrier_joint) = if M::ENABLED {
                (
                    join_hydrology_ending(second.state, second_carrier_joint.clone())?,
                    Some(second_carrier_joint),
                )
            } else {
                (
                    join_hydrology_ending(second.state, second_carrier_joint)?,
                    None,
                )
            };
            if let Some(carrier_joint) = captured_second_carrier_joint.as_ref() {
                M::selected_trial(
                    evidence,
                    TerminalSelectedTrialHook {
                        position: TerminalPairPosition::Fine2,
                        role: CoveredTerminalTrialRoleV1::Half2,
                        attempt_ordinal: second_attempt,
                        relative_start_s: elapsed + first_dt,
                        duration_s: second_dt,
                        beginning: first.state.into(),
                        ending: second.state.into(),
                        ledger: second.ledger.into(),
                        beginning_joint: &first_joint,
                        carrier_ending_joint: carrier_joint,
                        hydrology_ending_joint: &second_joint,
                    },
                );
            }
            attempt_ordinal = next_attempt(attempt_ordinal)?;
            let refined = TerminalTrial {
                state: second.state,
                ledger: first.ledger.add(second.ledger),
            };
            let error = Self::terminal_scaled_error(full, refined);
            let regime_mismatch = (full.state.ice_kg_m2 == 0.0) != (refined.state.ice_kg_m2 == 0.0);
            // A terminal second child proves only that exhaustion lies in this
            // candidate. It may not admit the later parent endpoint; refine
            // until the first terminating 60-second grid boundary is reached.
            let rejected = error > 1.0 || regime_mismatch || second.state.ice_kg_m2 == 0.0;
            if M::ENABLED {
                M::pair(
                    evidence,
                    TerminalPairEvidenceHook {
                        duration_s: dt,
                        proposed_next_duration_s: if rejected {
                            seconds_from_nanoseconds(first_ns)
                        } else if error < 0.125 {
                            seconds_from_nanoseconds(
                                (candidate_quanta * 2).min(remaining_quanta)
                                    * MINIMUM_COVERED_CARRIER_NS,
                            )
                        } else {
                            dt
                        },
                        components: Self::terminal_error_components(full, refined),
                        scaled_error: error,
                        rejected,
                    },
                );
            }
            if rejected {
                rejected_trials = next_attempt(rejected_trials)?;
                consecutive_rejections = next_attempt(consecutive_rejections)?;
                if consecutive_rejections > MAXIMUM_REJECTIONS {
                    return Err(DirectSnowStage3EvaluationError::TerminalNumerics(
                        SnowTerminalNumericsFailure::RejectionLimit,
                    ));
                }
                trial_quanta = first_quanta;
                continue;
            }
            consecutive_rejections = 0;
            if error >= maximum_scaled_error {
                maximum_scaled_error = error;
                lte_coarse = full;
                lte_fine = refined;
            }

            let accepted_beginning = state;
            state = second.state;
            accepted_joint = second_joint;
            accepted_microsteps.push(TerminalAcceptedMicrostep {
                relative_start_ns: elapsed_ns,
                duration_ns: first_ns,
                relative_start_s: elapsed,
                duration_s: first_dt,
                beginning: accepted_beginning,
                ending: first.state,
                ledger: first.ledger,
                carrier_ending_joint: accepted_first_carrier_joint,
                hydrology_ending_joint: first_joint,
            });
            accepted_microsteps.push(TerminalAcceptedMicrostep {
                relative_start_ns: elapsed_ns + first_ns,
                duration_ns: second_ns,
                relative_start_s: elapsed + first_dt,
                duration_s: second_dt,
                beginning: first.state,
                ending: second.state,
                ledger: second.ledger,
                carrier_ending_joint: accepted_second_carrier_joint,
                hydrology_ending_joint: accepted_joint.clone(),
            });
            // Preserve the same left-to-right floating-point accumulation as
            // the accepted H1/H2 publication trace. Grouping the two child
            // ledgers first can change persistent cumulative owner bytes even
            // when the physical values differ only in the last bit.
            ledger = ledger.add(first.ledger).add(second.ledger);
            elapsed_quanta += candidate_quanta;
            accepted_trials = next_attempt(accepted_trials)?;
            if refined.state.ice_kg_m2 <= 0.0 {
                let event_ns = elapsed_quanta * MINIMUM_COVERED_CARRIER_NS;
                event_bracket_lower_ns = event_ns;
                event_bracket_upper_ns = event_ns;
                event_bracket_lower_solid_kg_m2 = 0.0;
                event_bracket_upper_solid_kg_m2 = 0.0;
                break;
            }
            trial_quanta = if error < 0.125 {
                (2 * candidate_quanta).min(total_quanta - elapsed_quanta)
            } else {
                candidate_quanta.min(total_quanta - elapsed_quanta)
            };
        }

        let elapsed_ns = elapsed_quanta * MINIMUM_COVERED_CARRIER_NS;

        let result = Self::terminal_result_from_trial(
            hour_index,
            hour_offset_seconds,
            requested_seconds,
            start,
            state,
            ledger,
            requested_ns,
            elapsed_ns,
            event_bracket_lower_ns,
            event_bracket_upper_ns,
            event_bracket_lower_solid_kg_m2,
            event_bracket_upper_solid_kg_m2,
            lte_coarse,
            lte_fine,
            accepted_trials,
            rejected_trials,
            maximum_scaled_error,
        )?;
        Ok((result, accepted_joint, accepted_microsteps))
    }
}

#[cfg(test)]
#[allow(
    clippy::cast_possible_truncation,
    clippy::float_cmp,
    clippy::ignored_unit_patterns,
    clippy::let_unit_value,
    reason = "contract tests intentionally compare exact grid projections and bounded diagnostic ordinals"
)]
mod tests {
    use super::*;

    const TEST_LATENT_HEAT_SUBLIMATION_J_KG: f64 = 2_834_000.0;

    fn one_quantum_flux(
        complete_energy_j_m2: f64,
        vapor_mass_exchange_kg_m2: f64,
        latent_energy_j_m2: f64,
        external_liquid_kg_m2: f64,
    ) -> TerminalFluxIntegral {
        TerminalFluxIntegral {
            complete_energy_j_m2,
            vapor_mass_exchange_kg_m2,
            shortwave_energy_j_m2: complete_energy_j_m2 - latent_energy_j_m2,
            longwave_energy_j_m2: 0.0,
            sensible_energy_j_m2: 0.0,
            latent_energy_j_m2,
            advected_energy_j_m2: 0.0,
            snow_soil_heat_energy_j_m2: 0.0,
            external_liquid_kg_m2,
        }
    }

    fn solve_one_quantum(
        start: TerminalState,
        flux: TerminalFluxIntegral,
    ) -> Result<DirectSnowTerminalEventResult, DirectSnowStage3EvaluationError> {
        Wb11HydrologyKernel::solve_terminal_enthalpy_event(
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
            0,
            0.0,
            MINIMUM_COVERED_CARRIER_SECONDS,
            start,
            (),
            |_, _, _, _, relative_start, duration, _, _| {
                assert_eq!(relative_start.to_bits(), 0.0_f64.to_bits());
                assert_eq!(
                    duration.to_bits(),
                    MINIMUM_COVERED_CARRIER_SECONDS.to_bits()
                );
                Ok((flux, ()))
            },
            |_, joint| Ok(joint),
        )
        .map(|value| value.0)
    }

    fn assert_closed(result: &DirectSnowTerminalEventResult) {
        assert!(
            result.solid_mass_closure_residual_kg_m2.abs() <= 1.0e-12,
            "{result:?}"
        );
        assert!(
            result.liquid_mass_closure_residual_kg_m2.abs() <= 1.0e-12,
            "{result:?}"
        );
        assert!(
            result.energy_closure_residual_j_m2.abs() <= 1.0e-6,
            "{result:?}"
        );
        assert!(
            result.end_ice_kg_m2 == 0.0 || result.terminal_unallocated_energy_j_m2 == 0.0,
            "{result:?}"
        );
    }

    fn solve(
        start: TerminalState,
        seconds: f64,
        energy_rate_w_m2: f64,
        vapor_rate_kg_m2_s: f64,
    ) -> DirectSnowTerminalEventResult {
        Wb11HydrologyKernel::solve_terminal_enthalpy_event(
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
            0,
            0.0,
            seconds,
            start,
            (),
            |_, _, _, _, _, duration, _, _| {
                let latent_energy_j_m2 =
                    vapor_rate_kg_m2_s * duration * TEST_LATENT_HEAT_SUBLIMATION_J_KG;
                Ok((
                    TerminalFluxIntegral {
                        complete_energy_j_m2: energy_rate_w_m2 * duration + latent_energy_j_m2,
                        vapor_mass_exchange_kg_m2: vapor_rate_kg_m2_s * duration,
                        shortwave_energy_j_m2: energy_rate_w_m2 * duration,
                        longwave_energy_j_m2: 0.0,
                        sensible_energy_j_m2: 0.0,
                        latent_energy_j_m2,
                        advected_energy_j_m2: 0.0,
                        snow_soil_heat_energy_j_m2: 0.0,
                        external_liquid_kg_m2: 0.0,
                    },
                    (),
                ))
            },
            |_, joint| Ok(joint),
        )
        .unwrap()
        .0
    }

    #[test]
    fn production_solver_phase_matrix_closes_every_enthalpy_branch() {
        let fusion = STAGE3_LATENT_HEAT_FUSION_J_KG;
        let cases = [
            (
                "cold-all-ice",
                TerminalState {
                    ice_kg_m2: 0.6,
                    liquid_kg_m2: 0.0,
                    cold_content_j_m2: 10_000.0,
                },
                one_quantum_flux(0.0, 0.0, 0.0, 0.0),
                (0.6, 0.0, 10_000.0, 0.0),
            ),
            (
                "mixed-isothermal",
                TerminalState {
                    ice_kg_m2: 0.6,
                    liquid_kg_m2: 0.0,
                    cold_content_j_m2: 0.0,
                },
                one_quantum_flux(fusion * 0.3, 0.0, 0.0, 0.0),
                (0.3, 0.3, 0.0, 0.0),
            ),
            (
                "exact-melt",
                TerminalState {
                    ice_kg_m2: 0.6,
                    liquid_kg_m2: 0.0,
                    cold_content_j_m2: 0.0,
                },
                one_quantum_flux(fusion * 0.6, 0.0, 0.0, 0.0),
                (0.0, 0.6, 0.0, 0.0),
            ),
            (
                "melt-with-excess",
                TerminalState {
                    ice_kg_m2: 0.6,
                    liquid_kg_m2: 0.0,
                    cold_content_j_m2: 0.0,
                },
                one_quantum_flux(fusion * 0.7, 0.0, 0.0, 0.0),
                (0.0, 0.6, 0.0, fusion * 0.1),
            ),
            (
                "beginning-and-external-liquid-refreeze",
                TerminalState {
                    ice_kg_m2: 0.2,
                    liquid_kg_m2: 0.1,
                    cold_content_j_m2: 0.0,
                },
                one_quantum_flux(-50_000.0, 0.0, 0.0, 0.2),
                (
                    0.5 - (fusion * 0.3 - 50_000.0) / fusion,
                    (fusion * 0.3 - 50_000.0) / fusion,
                    0.0,
                    0.0,
                ),
            ),
        ];
        for (name, start, flux, expected) in cases {
            let result =
                solve_one_quantum(start, flux).unwrap_or_else(|error| panic!("{name}: {error:?}"));
            assert_closed(&result);
            assert!(
                (result.end_ice_kg_m2 - expected.0).abs() <= 1.0e-12,
                "{name}: {result:?}"
            );
            assert!(
                (result.terminal_liquid_kg_m2 - expected.1).abs() <= 1.0e-12,
                "{name}: {result:?}"
            );
            assert!(
                (result.end_cold_content_j_m2 - expected.2).abs() <= 1.0e-6,
                "{name}: {result:?}"
            );
            assert!(
                (result.terminal_unallocated_energy_j_m2 - expected.3).abs() <= 1.0e-6,
                "{name}: {result:?}"
            );
        }
    }

    #[test]
    fn deposition_at_meltout_and_nearby_energy_vapor_controls_use_same_projection() {
        let start = TerminalState {
            ice_kg_m2: 0.6,
            liquid_kg_m2: 0.0,
            cold_content_j_m2: 0.0,
        };
        let deposition = 0.002;
        let latent = deposition * TEST_LATENT_HEAT_SUBLIMATION_J_KG;
        let balance = STAGE3_LATENT_HEAT_FUSION_J_KG * (start.ice_kg_m2 + deposition);
        let below = solve_one_quantum(
            start,
            one_quantum_flux(balance - 1.0e-3, deposition, latent, 0.0),
        )
        .unwrap();
        let exact =
            solve_one_quantum(start, one_quantum_flux(balance, deposition, latent, 0.0)).unwrap();
        let above = solve_one_quantum(
            start,
            one_quantum_flux(balance + 1.0e-3, deposition, latent, 0.0),
        )
        .unwrap();
        let less_vapor = deposition - 1.0e-9;
        let less_vapor_result = solve_one_quantum(
            start,
            one_quantum_flux(
                balance,
                less_vapor,
                less_vapor * TEST_LATENT_HEAT_SUBLIMATION_J_KG,
                0.0,
            ),
        )
        .unwrap();
        let more_vapor = deposition + 1.0e-9;
        let more_vapor_result = solve_one_quantum(
            start,
            one_quantum_flux(
                balance,
                more_vapor,
                more_vapor * TEST_LATENT_HEAT_SUBLIMATION_J_KG,
                0.0,
            ),
        )
        .unwrap();
        for (result, expected_deposition) in [
            (&below, deposition),
            (&exact, deposition),
            (&above, deposition),
            (&less_vapor_result, less_vapor),
            (&more_vapor_result, more_vapor),
        ] {
            assert_closed(result);
            assert!((result.deposition_kg_m2 - expected_deposition).abs() <= 1.0e-15);
        }
        assert!(below.end_ice_kg_m2 > 0.0);
        assert_eq!(
            below.terminal_unallocated_energy_j_m2.to_bits(),
            0.0_f64.to_bits()
        );
        assert_eq!(exact.end_ice_kg_m2.to_bits(), 0.0_f64.to_bits());
        assert_eq!(
            exact.terminal_unallocated_energy_j_m2.to_bits(),
            0.0_f64.to_bits()
        );
        assert_eq!(above.end_ice_kg_m2.to_bits(), 0.0_f64.to_bits());
        assert!(above.terminal_unallocated_energy_j_m2 > 0.0);
        assert_eq!(less_vapor_result.end_ice_kg_m2.to_bits(), 0.0_f64.to_bits());
        assert!(less_vapor_result.terminal_unallocated_energy_j_m2 > 0.0);
        assert!(more_vapor_result.end_ice_kg_m2 > 0.0);
        assert_eq!(
            more_vapor_result.terminal_unallocated_energy_j_m2.to_bits(),
            0.0_f64.to_bits()
        );
    }

    #[test]
    fn bounded_sublimation_truncates_mass_and_latent_energy_together() {
        let raw_mass = -0.9;
        let raw_latent = raw_mass * TEST_LATENT_HEAT_SUBLIMATION_J_KG;
        let actual_latent = -0.6 * TEST_LATENT_HEAT_SUBLIMATION_J_KG;
        let result = solve_one_quantum(
            TerminalState {
                ice_kg_m2: 0.6,
                liquid_kg_m2: 0.0,
                cold_content_j_m2: 0.0,
            },
            one_quantum_flux(
                0.6 * TEST_LATENT_HEAT_SUBLIMATION_J_KG + raw_latent,
                raw_mass,
                raw_latent,
                0.0,
            ),
        )
        .unwrap();
        assert_closed(&result);
        assert!((result.sublimation_kg_m2 - 0.6).abs() <= 1.0e-12);
        assert!((result.latent_energy_j_m2 - actual_latent).abs() <= 1.0e-6);
        assert!(result.complete_energy_j_m2.abs() <= 1.0e-6, "{result:?}");
    }

    #[test]
    fn production_solver_rejects_bounded_vapor_and_component_poison_cases() {
        let start = TerminalState {
            ice_kg_m2: 0.6,
            liquid_kg_m2: 0.0,
            cold_content_j_m2: 0.0,
        };
        let mut cases = vec![
            one_quantum_flux(1.0, 0.0, 1.0, 0.0),
            one_quantum_flux(-1.0, 0.1, -1.0, 0.0),
            one_quantum_flux(1.0, -0.1, 1.0, 0.0),
            one_quantum_flux(f64::NAN, f64::NAN, f64::NAN, 0.0),
            one_quantum_flux(f64::INFINITY, 0.1, f64::INFINITY, 0.0),
        ];
        let mut component_poison = one_quantum_flux(10.0, 0.0, 0.0, 0.0);
        component_poison.shortwave_energy_j_m2 = 9.0;
        cases.push(component_poison);
        for flux in cases {
            assert!(matches!(
                solve_one_quantum(start, flux),
                Err(DirectSnowStage3EvaluationError::TerminalNumerics(
                    SnowTerminalNumericsFailure::Closure
                        | SnowTerminalNumericsFailure::DomainOrNonFinite
                ))
            ));
        }
    }

    #[test]
    fn temporal_floor_rejects_without_provider_call_and_accepts_exactly_one_quantum() {
        let start = TerminalState {
            ice_kg_m2: 0.6,
            liquid_kg_m2: 0.0,
            cold_content_j_m2: 0.0,
        };
        for invalid_seconds in [59.999_999_999, 70.0] {
            let mut calls = 0_u32;
            let result = Wb11HydrologyKernel::solve_terminal_enthalpy_event(
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                0,
                0.0,
                invalid_seconds,
                start,
                (),
                |_, _, _, _, _, _, _, _| {
                    calls += 1;
                    Ok((one_quantum_flux(0.0, 0.0, 0.0, 0.0), ()))
                },
                |_, joint| Ok(joint),
            );
            assert!(matches!(
                result,
                Err(DirectSnowStage3EvaluationError::TerminalNumerics(
                    SnowTerminalNumericsFailure::BelowCarrierDomain
                ))
            ));
            assert_eq!(calls, 0, "invalid support reached provider");
        }

        let mut calls = 0_u32;
        let result = Wb11HydrologyKernel::solve_terminal_enthalpy_event(
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
            0,
            0.0,
            60.0,
            start,
            (),
            |_, _, _, _, relative_start, duration, role, attempt| {
                calls += 1;
                assert_eq!(relative_start.to_bits(), 0.0_f64.to_bits());
                assert_eq!(duration.to_bits(), 60.0_f64.to_bits());
                assert_eq!(role, CoveredTerminalTrialRoleV1::Full);
                assert_eq!(attempt, 0);
                Ok((one_quantum_flux(0.0, 0.0, 0.0, 0.0), ()))
            },
            |_, joint| Ok(joint),
        )
        .unwrap()
        .0;
        assert_eq!(calls, 1);
        assert_eq!(result.accepted_trials, 1);
        assert_eq!(result.rejected_trials, 0);
    }

    #[test]
    fn odd_quantum_composition_tiles_exact_nanoseconds_and_installs_children() {
        let mut calls = Vec::new();
        let mut evidence = <NoEvidence as TerminalEvidenceMode<()>>::new_state();
        let (result, _, microsteps) =
            Wb11HydrologyKernel::solve_terminal_enthalpy_event_with_evidence::<
                _,
                _,
                _,
                NoEvidence,
            >(
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                0,
                0.0,
                180.0,
                TerminalState {
                    ice_kg_m2: 0.6,
                    liquid_kg_m2: 0.0,
                    cold_content_j_m2: 0.0,
                },
                (),
                |_, _, _, _, relative_start, duration, role, attempt| {
                    calls.push((relative_start, duration, role, attempt));
                    Ok((one_quantum_flux(10.0 * duration, 0.0, 0.0, 0.0), ()))
                },
                |_, joint| Ok(joint),
                false,
                false,
                &mut evidence,
            )
            .unwrap();
        assert_eq!(result.accepted_trials, 1);
        assert_eq!(result.rejected_trials, 0);
        assert_eq!(calls.len(), 3);
        assert_eq!(calls[0].1.to_bits(), 180.0_f64.to_bits());
        assert_eq!(calls[1].1.to_bits(), 60.0_f64.to_bits());
        assert_eq!(calls[2].1.to_bits(), 120.0_f64.to_bits());
        assert_eq!(microsteps.len(), 2);
        assert_eq!(microsteps[0].relative_start_ns, 0);
        assert_eq!(microsteps[0].duration_ns, 60_000_000_000);
        assert_eq!(microsteps[1].relative_start_ns, 60_000_000_000);
        assert_eq!(microsteps[1].duration_ns, 120_000_000_000);
        assert_eq!(microsteps[0].beginning.ice_kg_m2.to_bits(), 0.6_f64.to_bits());
        assert_eq!(microsteps[0].beginning.liquid_kg_m2.to_bits(), 0.0_f64.to_bits());
        assert_eq!(microsteps[0].beginning.cold_content_j_m2.to_bits(), 0.0_f64.to_bits());
        assert_eq!(microsteps[0].ending, microsteps[1].beginning);
        assert_eq!(microsteps[1].ending.ice_kg_m2.to_bits(), result.end_ice_kg_m2.to_bits());
        assert_eq!(
            microsteps[1].ending.liquid_kg_m2.to_bits(),
            result.terminal_liquid_kg_m2.to_bits()
        );
        assert_eq!(
            microsteps[1].ending.cold_content_j_m2.to_bits(),
            result.end_cold_content_j_m2.to_bits()
        );
        assert_eq!(
            microsteps.iter().map(|step| step.duration_ns).sum::<u128>(),
            180_000_000_000
        );
        for step in microsteps {
            assert_eq!(
                step.relative_start_s,
                seconds_from_nanoseconds(step.relative_start_ns)
            );
            assert_eq!(step.duration_s, seconds_from_nanoseconds(step.duration_ns));
            assert!(step.duration_ns >= MINIMUM_COVERED_CARRIER_NS);
        }
    }

    #[test]
    fn deterministic_refinement_replay_and_attempt_diagnostics_do_not_change_physics() {
        type Call = (u64, u64, f64, f64, CoveredTerminalTrialRoleV1, u32);
        fn run() -> (
            DirectSnowTerminalEventResult,
            Vec<TerminalAcceptedMicrostep<()>>,
            Vec<Call>,
        ) {
            let mut calls = Vec::new();
            let mut evidence = <NoEvidence as TerminalEvidenceMode<()>>::new_state();
            let (result, _, microsteps) =
                Wb11HydrologyKernel::solve_terminal_enthalpy_event_with_evidence::<
                    _,
                    _,
                    _,
                    NoEvidence,
                >(
                    HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                    0,
                    0.0,
                    360.0,
                    TerminalState {
                        ice_kg_m2: 0.5,
                        liquid_kg_m2: 0.0,
                        cold_content_j_m2: 0.0,
                    },
                    (),
                    |state, _, _, _, relative_start, duration, role, attempt| {
                        calls.push((
                            state.ice_kg_m2.to_bits(),
                            state.cold_content_j_m2.to_bits(),
                            relative_start,
                            duration,
                            role,
                            attempt,
                        ));
                        let energy = (duration / 100.0) * (duration / 100.0);
                        Ok((one_quantum_flux(energy, 0.0, 0.0, 0.0), ()))
                    },
                    |_, joint| Ok(joint),
                    false,
                    false,
                    &mut evidence,
                )
                .unwrap();
            (result, microsteps, calls)
        }

        let first = run();
        let replay = run();
        assert_eq!(first.0, replay.0);
        assert_eq!(first.1, replay.1);
        assert_eq!(first.2, replay.2);
        assert!(first.0.rejected_trials > 0);
        assert!(
            first
                .2
                .iter()
                .enumerate()
                .all(|(index, call)| call.5 == index as u32)
        );
        assert!(first.1.iter().all(|step| {
            step.duration_ns >= MINIMUM_COVERED_CARRIER_NS
                && step.relative_start_ns % MINIMUM_COVERED_CARRIER_NS == 0
                && step.duration_ns % MINIMUM_COVERED_CARRIER_NS == 0
        }));
    }

    #[test]
    fn pure_melt_localizes_analytical_event() {
        let event = solve(
            TerminalState {
                ice_kg_m2: 0.6,
                liquid_kg_m2: 0.0,
                cold_content_j_m2: 0.0,
            },
            1_020.0,
            333.6,
            0.0,
        );
        assert!(event.event_occurred);
        assert!((event.evaluated_seconds - 600.0).abs() <= 1.0e-6);
        assert!((event.melt_kg_m2 - 0.6).abs() <= 1.0e-9, "{event:?}");
        assert!((event.complete_energy_j_m2 - 200_160.0).abs() <= 1.0e-6);
        assert!(event.solid_mass_closure_residual_kg_m2.abs() <= 1.0e-9);
        assert!(event.energy_closure_residual_j_m2.abs() <= 1.0e-6);
    }

    #[test]
    fn capture_mode_retains_rejected_pair_and_separate_floor_admission() {
        let mut evidence = <CaptureEvidence as TerminalEvidenceMode<
            Option<CoveredTerminalJointTrialStateV1>,
        >>::new_state();
        let result = Wb11HydrologyKernel::solve_terminal_enthalpy_event_with_evidence::<
            _,
            _,
            _,
            CaptureEvidence,
        >(
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
            0,
            0.0,
            360.0,
            TerminalState {
                ice_kg_m2: 0.5,
                liquid_kg_m2: 0.0,
                cold_content_j_m2: 0.0,
            },
            None,
            |_, _, _, _, _, duration, _, _| {
                Ok((
                    TerminalFluxIntegral {
                        complete_energy_j_m2: (duration / 100.0) * (duration / 100.0),
                        vapor_mass_exchange_kg_m2: 0.0,
                        shortwave_energy_j_m2: (duration / 100.0) * (duration / 100.0),
                        longwave_energy_j_m2: 0.0,
                        sensible_energy_j_m2: 0.0,
                        latent_energy_j_m2: 0.0,
                        advected_energy_j_m2: 0.0,
                        snow_soil_heat_energy_j_m2: 0.0,
                        external_liquid_kg_m2: 0.0,
                    },
                    None,
                ))
            },
            |_, joint| Ok(joint),
            false,
            false,
            &mut evidence,
        );
        assert!(result.is_ok(), "{result:?}");
        assert!(evidence.pairs.iter().any(|pair| pair.rejected));
        let admission = evidence.admissions.last().unwrap();
        assert_eq!(admission.0.to_bits(), 60.0_f64.to_bits());
        assert_eq!(admission.1.to_bits(), 0.0_f64.to_bits());
        assert_eq!(admission.2.to_bits(), 60.0_f64.to_bits());
        assert_eq!(admission.3, TerminalFloorDecision::Accepted);
        assert_eq!(admission.4, admission.5);
    }

    #[test]
    fn every_adaptive_composition_trial_carries_its_exact_relative_start() {
        let mut trials = Vec::new();
        let (event, _) = Wb11HydrologyKernel::solve_terminal_enthalpy_event(
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
            0,
            0.0,
            1_020.0,
            TerminalState {
                ice_kg_m2: 0.6,
                liquid_kg_m2: 0.0,
                cold_content_j_m2: 0.0,
            },
            (),
            |_, _, _, _, relative_start, duration, role, attempt| {
                trials.push((relative_start, duration, role, attempt));
                Ok((
                    TerminalFluxIntegral {
                        complete_energy_j_m2: 333.6 * duration,
                        vapor_mass_exchange_kg_m2: 0.0,
                        shortwave_energy_j_m2: 333.6 * duration,
                        longwave_energy_j_m2: 0.0,
                        sensible_energy_j_m2: 0.0,
                        latent_energy_j_m2: 0.0,
                        advected_energy_j_m2: 0.0,
                        snow_soil_heat_energy_j_m2: 0.0,
                        external_liquid_kg_m2: 0.0,
                    },
                    (),
                ))
            },
            |_, joint| Ok(joint),
        )
        .unwrap();
        assert!(event.event_occurred);
        assert!(trials.iter().all(|(start, duration, _, _)| {
            start.is_finite()
                && duration.is_finite()
                && *start >= 0.0
                && *duration > 0.0
                && start + duration <= 1_020.0
        }));
        assert!(trials.iter().any(|(start, _, _, _)| *start > 0.0));
        assert!(trials.windows(2).any(|pair| pair[0].0 == pair[1].0));
        assert!(
            trials
                .iter()
                .enumerate()
                .all(|(index, (_, _, _, attempt))| *attempt == index as u32)
        );
        assert!(
            trials
                .iter()
                .all(|(_, _, role, _)| { *role != CoveredTerminalTrialRoleV1::Root })
        );
    }

    #[test]
    fn joint_trial_state_advances_only_along_the_accepted_fine_chain() {
        let mut observed = Vec::new();
        let (result, _) = Wb11HydrologyKernel::solve_terminal_enthalpy_event(
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
            0,
            0.0,
            1_020.0,
            TerminalState {
                ice_kg_m2: 0.5,
                liquid_kg_m2: 0.0,
                cold_content_j_m2: 0.0,
            },
            0_u32,
            |_, _, _, joint, _, duration, role, attempt| {
                observed.push((role, attempt, *joint));
                Ok((
                    TerminalFluxIntegral {
                        complete_energy_j_m2: 0.0,
                        vapor_mass_exchange_kg_m2: 0.0,
                        shortwave_energy_j_m2: 0.0,
                        longwave_energy_j_m2: 0.0,
                        sensible_energy_j_m2: 0.0,
                        latent_energy_j_m2: 0.0,
                        advected_energy_j_m2: 0.0,
                        snow_soil_heat_energy_j_m2: 0.0,
                        external_liquid_kg_m2: 0.0 * duration,
                    },
                    joint + 1,
                ))
            },
            |_, joint| Ok(joint),
        )
        .unwrap();
        assert!(!result.event_occurred);
        assert_eq!(observed[0], (CoveredTerminalTrialRoleV1::Full, 0, 0));
        assert_eq!(observed[1], (CoveredTerminalTrialRoleV1::Half1, 1, 0));
        assert_eq!(observed[2], (CoveredTerminalTrialRoleV1::Half2, 2, 1));
    }

    #[test]
    fn absolute_prefix_candidates_cover_zero_event_tick_and_predecessor_tick() {
        let start = TerminalState {
            ice_kg_m2: 0.6,
            liquid_kg_m2: 0.0,
            cold_content_j_m2: 0.0,
        };
        let mut attempt = 0;
        let mut flux = |state: TerminalState,
                        _: TerminalLedger,
                        _: usize,
                        joint: &u32,
                        relative_start: f64,
                        duration: f64,
                        role,
                        _| {
            if role == CoveredTerminalTrialRoleV1::Root
                && (relative_start != 0.0 || state.ice_kg_m2.to_bits() != start.ice_kg_m2.to_bits())
            {
                return Err(DirectSnowStage3EvaluationError::TerminalNumerics(
                    SnowTerminalNumericsFailure::InvalidEventBracket,
                ));
            }
            Ok((
                TerminalFluxIntegral {
                    complete_energy_j_m2: 3_336.0 * duration,
                    vapor_mass_exchange_kg_m2: 0.0,
                    shortwave_energy_j_m2: 3_336.0 * duration,
                    longwave_energy_j_m2: 0.0,
                    sensible_energy_j_m2: 0.0,
                    latent_energy_j_m2: 0.0,
                    advected_energy_j_m2: 0.0,
                    snow_soil_heat_energy_j_m2: 0.0,
                    external_liquid_kg_m2: 0.0,
                },
                *joint + 1,
            ))
        };
        let mut join = |_: TerminalState, joint| Ok(joint);
        let (zero, zero_joint) = Wb11HydrologyKernel::terminal_prefix_candidate(
            start,
            &0,
            0.0,
            &mut attempt,
            &mut flux,
            &mut join,
        )
        .unwrap();
        assert_eq!(zero.state.ice_kg_m2.to_bits(), start.ice_kg_m2.to_bits());
        assert_eq!(zero_joint, 0);
        let predecessor = Wb11HydrologyKernel::terminal_prefix_candidate(
            start,
            &0,
            59.999_999_999,
            &mut attempt,
            &mut flux,
            &mut join,
        );
        assert!(matches!(
            predecessor,
            Err(DirectSnowStage3EvaluationError::TerminalNumerics(
                SnowTerminalNumericsFailure::BelowCarrierDomain
            ))
        ));
        let (event, _) = Wb11HydrologyKernel::terminal_prefix_candidate(
            start,
            &0,
            60.0,
            &mut attempt,
            &mut flux,
            &mut join,
        )
        .unwrap();
        assert_eq!(event.state.ice_kg_m2.to_bits(), 0.0_f64.to_bits());
    }

    #[test]
    fn pure_sublimation_and_joint_exhaustion_localize_same_event() {
        let start = TerminalState {
            ice_kg_m2: 0.6,
            liquid_kg_m2: 0.0,
            cold_content_j_m2: 0.0,
        };
        let sublimation = solve(start, 1_020.0, 0.0, -0.001);
        assert!(
            sublimation.evaluated_seconds >= 600.0
                && sublimation.evaluated_seconds <= 660.0 + 1.0e-9,
            "{sublimation:?}"
        );
        assert!(
            (sublimation.sublimation_kg_m2 - 0.6).abs() <= 1.0e-9,
            "{sublimation:?}"
        );
        let joint = solve(start, 1_020.0, 1_583.8, -0.0005);
        assert!((joint.evaluated_seconds - 600.0).abs() <= 1.0e-6);
        assert!((joint.melt_kg_m2 - 0.3).abs() <= 1.0e-9);
        assert!((joint.sublimation_kg_m2 - 0.3).abs() <= 1.0e-9);
    }

    #[test]
    fn cooling_deposition_and_refreeze_do_not_create_false_event() {
        let cooling = solve(
            TerminalState {
                ice_kg_m2: 0.5,
                liquid_kg_m2: 0.1,
                cold_content_j_m2: 0.0,
            },
            60.0,
            -3_334.0,
            0.001,
        );
        assert!(!cooling.event_occurred);
        assert!(cooling.refrozen_kg_m2 > 0.0);
        assert!(cooling.deposition_kg_m2 > 0.0);
        assert!(cooling.end_ice_kg_m2 > cooling.start_ice_kg_m2);
        assert!(cooling.solid_mass_closure_residual_kg_m2.abs() <= 1.0e-12);
        assert!(cooling.liquid_mass_closure_residual_kg_m2.abs() <= 1.0e-12);
        assert!(cooling.energy_closure_residual_j_m2.abs() <= 1.0e-6);

        let insufficient_warming = solve(
            TerminalState {
                ice_kg_m2: 0.5,
                liquid_kg_m2: 0.1,
                cold_content_j_m2: 50_000.0,
            },
            60.0,
            100.0,
            0.0,
        );
        assert!(insufficient_warming.refrozen_kg_m2 > 0.0);
        assert!(insufficient_warming.end_cold_content_j_m2 < 44_000.0);
        assert!(insufficient_warming.energy_closure_residual_j_m2.abs() <= 1.0e-6);
    }

    #[test]
    fn sublimation_is_reserved_before_over_demanded_melt() {
        const TEST_LATENT_HEAT_SUBLIMATION_J_KG: f64 = 2_834_000.0;
        let sublimation_latent = -0.3 * TEST_LATENT_HEAT_SUBLIMATION_J_KG;
        let melt_energy = STAGE3_LATENT_HEAT_FUSION_J_KG * 0.3;
        let trial = Wb11HydrologyKernel::terminal_transition(
            TerminalState {
                ice_kg_m2: 0.6,
                liquid_kg_m2: 0.0,
                cold_content_j_m2: 0.0,
            },
            TerminalFluxIntegral {
                complete_energy_j_m2: melt_energy,
                vapor_mass_exchange_kg_m2: -0.3,
                shortwave_energy_j_m2: melt_energy - sublimation_latent,
                longwave_energy_j_m2: 0.0,
                sensible_energy_j_m2: 0.0,
                latent_energy_j_m2: sublimation_latent,
                advected_energy_j_m2: 0.0,
                snow_soil_heat_energy_j_m2: 0.0,
                external_liquid_kg_m2: 0.0,
            },
        )
        .unwrap();
        assert!(trial.state.ice_kg_m2.abs() <= f64::EPSILON);
        assert!((trial.ledger.sublimation_kg_m2 - 0.3).abs() <= 1.0e-12);
        assert!((trial.ledger.melt_kg_m2 - 0.3).abs() <= 1.0e-12);
        assert!(trial.ledger.unallocated_energy_j_m2.abs() <= 1.0e-6);
        let nondyadic_sublimation_latent = -0.2 * TEST_LATENT_HEAT_SUBLIMATION_J_KG;
        let nondyadic_melt_energy = STAGE3_LATENT_HEAT_FUSION_J_KG * 0.5;
        let nondyadic = Wb11HydrologyKernel::terminal_transition(
            TerminalState {
                ice_kg_m2: 0.7,
                liquid_kg_m2: 0.0,
                cold_content_j_m2: 0.0,
            },
            TerminalFluxIntegral {
                complete_energy_j_m2: nondyadic_melt_energy,
                vapor_mass_exchange_kg_m2: -0.2,
                shortwave_energy_j_m2: nondyadic_melt_energy - nondyadic_sublimation_latent,
                longwave_energy_j_m2: 0.0,
                sensible_energy_j_m2: 0.0,
                latent_energy_j_m2: nondyadic_sublimation_latent,
                advected_energy_j_m2: 0.0,
                snow_soil_heat_energy_j_m2: 0.0,
                external_liquid_kg_m2: 0.0,
            },
        )
        .unwrap();
        assert_eq!(nondyadic.state.ice_kg_m2.to_bits(), 0.0_f64.to_bits());
    }

    #[test]
    fn manufactured_state_dependent_event_converges_first_order() {
        fn event_time(step_seconds: f64) -> f64 {
            let rate = 0.000_5;
            let feedback = 1.0;
            let mut state = TerminalState {
                ice_kg_m2: 0.6,
                liquid_kg_m2: 0.0,
                cold_content_j_m2: 0.0,
            };
            let mut elapsed = 0.0;
            loop {
                let melt_rate = rate * (1.0 + feedback * state.ice_kg_m2);
                let trial = Wb11HydrologyKernel::terminal_transition(
                    state,
                    TerminalFluxIntegral {
                        complete_energy_j_m2: STAGE3_LATENT_HEAT_FUSION_J_KG
                            * melt_rate
                            * step_seconds,
                        vapor_mass_exchange_kg_m2: 0.0,
                        shortwave_energy_j_m2: STAGE3_LATENT_HEAT_FUSION_J_KG
                            * melt_rate
                            * step_seconds,
                        longwave_energy_j_m2: 0.0,
                        sensible_energy_j_m2: 0.0,
                        latent_energy_j_m2: 0.0,
                        advected_energy_j_m2: 0.0,
                        snow_soil_heat_energy_j_m2: 0.0,
                        external_liquid_kg_m2: 0.0,
                    },
                )
                .unwrap();
                if trial.state.ice_kg_m2 <= 0.0 {
                    return elapsed + state.ice_kg_m2 / melt_rate;
                }
                state = trial.state;
                elapsed += step_seconds;
            }
        }
        let oracle = 1.6_f64.ln() / 0.000_5;
        let coarse_error = (event_time(120.0) - oracle).abs();
        let medium_error = (event_time(60.0) - oracle).abs();
        let fine_error = (event_time(30.0) - oracle).abs();
        assert!(medium_error < coarse_error);
        assert!(fine_error < medium_error);
        assert!(coarse_error / medium_error > 1.7);
        assert!(medium_error / fine_error > 1.7);
    }
}
