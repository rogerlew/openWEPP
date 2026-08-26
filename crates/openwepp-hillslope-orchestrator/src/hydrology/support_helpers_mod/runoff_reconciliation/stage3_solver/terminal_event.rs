#[allow(clippy::wildcard_imports)]
use super::*;

pub(crate) const RELATIVE_ERROR_TOLERANCE: f64 = 1.0e-8;
pub(crate) const MASS_ABSOLUTE_TOLERANCE_KG_M2: f64 = 1.0e-9;
pub(crate) const ENERGY_ABSOLUTE_TOLERANCE_J_M2: f64 = 1.0e-6;
const MINIMUM_TRIAL_SECONDS: f64 = 1.0e-9;
const MAXIMUM_TRIAL_SECONDS: f64 = 60.0;
const EVENT_TIME_TOLERANCE_SECONDS: f64 = 1.0e-6;
const MAXIMUM_REJECTIONS: u32 = 64;
const MAXIMUM_EVENT_BISECTIONS: u32 = 64;
const MINIMUM_COVERED_CARRIER_SECONDS: f64 = 0.6;

#[derive(Clone, Copy)]
#[allow(clippy::struct_field_names)]
pub(super) struct TerminalState {
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

#[derive(Clone, Copy, Default)]
#[allow(clippy::struct_field_names)]
struct TerminalLedger {
    complete_energy_j_m2: f64,
    cold_energy_change_j_m2: f64,
    refrozen_kg_m2: f64,
    deposition_kg_m2: f64,
    sublimation_kg_m2: f64,
    melt_kg_m2: f64,
    unallocated_energy_j_m2: f64,
    shortwave_energy_j_m2: f64,
    longwave_energy_j_m2: f64,
    sensible_energy_j_m2: f64,
    latent_energy_j_m2: f64,
    advected_energy_j_m2: f64,
    snow_soil_heat_energy_j_m2: f64,
    external_liquid_kg_m2: f64,
}

impl TerminalLedger {
    fn add(self, other: Self) -> Self {
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
    fn from(value: TerminalState) -> Self { Self { ice_kg_m2: value.ice_kg_m2, liquid_kg_m2: value.liquid_kg_m2, cold_content_j_m2: value.cold_content_j_m2 } }
}

impl From<TerminalLedger> for TerminalLedgerEvidence {
    fn from(value: TerminalLedger) -> Self { Self {
        complete_energy_j_m2: value.complete_energy_j_m2, cold_energy_change_j_m2: value.cold_energy_change_j_m2,
        refrozen_kg_m2: value.refrozen_kg_m2, deposition_kg_m2: value.deposition_kg_m2,
        sublimation_kg_m2: value.sublimation_kg_m2, melt_kg_m2: value.melt_kg_m2,
        unallocated_energy_j_m2: value.unallocated_energy_j_m2, shortwave_energy_j_m2: value.shortwave_energy_j_m2,
        longwave_energy_j_m2: value.longwave_energy_j_m2, sensible_energy_j_m2: value.sensible_energy_j_m2,
        latent_energy_j_m2: value.latent_energy_j_m2, advected_energy_j_m2: value.advected_energy_j_m2,
        snow_soil_heat_energy_j_m2: value.snow_soil_heat_energy_j_m2, external_liquid_kg_m2: value.external_liquid_kg_m2,
    } }
}

#[derive(Clone, Copy)]
pub(super) struct TerminalTrial {
    pub(super) state: TerminalState,
    ledger: TerminalLedger,
}

impl Wb11HydrologyKernel {
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
            &J,
            f64,
            f64,
            CoveredTerminalTrialRoleV1,
            u32,
        ) -> Result<(TerminalFluxIntegral, J), DirectSnowStage3EvaluationError>,
        G: FnMut(TerminalState, J) -> Result<J, DirectSnowStage3EvaluationError>,
        J: Clone,
    {
        if !candidate_seconds.is_finite() || candidate_seconds < 0.0 {
            return Err(DirectSnowStage3EvaluationError::TerminalNumerics(
                SnowTerminalNumericsFailure::DomainOrNonFinite,
            ));
        }
        if candidate_seconds == 0.0 {
            return Ok((TerminalTrial { state: start, ledger: TerminalLedger::default() }, initial_joint.clone()));
        }
        if candidate_seconds < MINIMUM_COVERED_CARRIER_SECONDS {
            return Err(DirectSnowStage3EvaluationError::TerminalNumerics(
                SnowTerminalNumericsFailure::BelowCarrierDomain,
            ));
        }
        let (flux, carrier_joint) = flux_integral(
            start,
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
        let transition = Self::terminal_transition(start, flux);
        let state = transition.state;
        let ledger = transition.ledger;
        let joint = join_hydrology_ending(state, carrier_joint)?;
        Ok((TerminalTrial { state, ledger }, joint))
    }

    pub(super) fn terminal_transition(
        start: TerminalState,
        flux: TerminalFluxIntegral,
    ) -> TerminalTrial {
        let mut state = start;
        let mut ledger = TerminalLedger {
            complete_energy_j_m2: flux.complete_energy_j_m2,
            shortwave_energy_j_m2: flux.shortwave_energy_j_m2,
            longwave_energy_j_m2: flux.longwave_energy_j_m2,
            sensible_energy_j_m2: flux.sensible_energy_j_m2,
            latent_energy_j_m2: flux.latent_energy_j_m2,
            advected_energy_j_m2: flux.advected_energy_j_m2,
            snow_soil_heat_energy_j_m2: flux.snow_soil_heat_energy_j_m2,
            external_liquid_kg_m2: flux.external_liquid_kg_m2,
            ..TerminalLedger::default()
        };
        state.liquid_kg_m2 += flux.external_liquid_kg_m2;
        let reserved_sublimation = (-flux.vapor_mass_exchange_kg_m2)
            .max(0.0)
            .min(start.ice_kg_m2);
        let provisional_cold_content = state.cold_content_j_m2 - flux.complete_energy_j_m2;
        if provisional_cold_content > 0.0 {
            state.cold_content_j_m2 = provisional_cold_content;
            let refrozen =
                (state.cold_content_j_m2 / STAGE3_LATENT_HEAT_FUSION_J_KG).min(state.liquid_kg_m2);
            state.liquid_kg_m2 -= refrozen;
            state.ice_kg_m2 = (start.ice_kg_m2 - reserved_sublimation) + refrozen;
            state.cold_content_j_m2 -= STAGE3_LATENT_HEAT_FUSION_J_KG * refrozen;
            ledger.refrozen_kg_m2 = refrozen;
        } else {
            state.cold_content_j_m2 = 0.0;
            let excess = (-provisional_cold_content).max(0.0);
            let melt_available = (start.ice_kg_m2 - reserved_sublimation).max(0.0);
            let melt = (excess / STAGE3_LATENT_HEAT_FUSION_J_KG).min(melt_available);
            state.ice_kg_m2 = melt_available - melt;
            state.liquid_kg_m2 += melt;
            ledger.melt_kg_m2 = melt;
            ledger.unallocated_energy_j_m2 =
                (excess - STAGE3_LATENT_HEAT_FUSION_J_KG * melt).max(0.0);
        }
        ledger.cold_energy_change_j_m2 = start.cold_content_j_m2 - state.cold_content_j_m2;

        if reserved_sublimation > 0.0 {
            ledger.sublimation_kg_m2 = reserved_sublimation;
        } else {
            state.ice_kg_m2 += flux.vapor_mass_exchange_kg_m2;
            ledger.deposition_kg_m2 = flux.vapor_mass_exchange_kg_m2;
        }
        TerminalTrial { state, ledger }
    }

    fn terminal_scaled_error(full: TerminalTrial, refined: TerminalTrial) -> f64 {
        let components = Self::terminal_error_components(full, refined);
        components[1..]
            .iter()
            .fold(components[0].4, |maximum, component| {
                maximum.max(component.4)
            })
    }

    fn terminal_error_components(full: TerminalTrial, refined: TerminalTrial) -> [(f64, f64, f64, f64, f64); 5] {
        let component = |coarse: f64, fine: f64, absolute: f64| {
            let delta = fine - coarse;
            let denominator = absolute + RELATIVE_ERROR_TOLERANCE * coarse.abs().max(fine.abs());
            (coarse, fine, delta, denominator, delta.abs() / denominator)
        };
        [
            component(full.state.ice_kg_m2, refined.state.ice_kg_m2, MASS_ABSOLUTE_TOLERANCE_KG_M2),
            component(full.state.liquid_kg_m2, refined.state.liquid_kg_m2, MASS_ABSOLUTE_TOLERANCE_KG_M2),
            component(full.state.cold_content_j_m2, refined.state.cold_content_j_m2, ENERGY_ABSOLUTE_TOLERANCE_J_M2),
            component(full.ledger.complete_energy_j_m2, refined.ledger.complete_energy_j_m2, ENERGY_ABSOLUTE_TOLERANCE_J_M2),
            component(full.ledger.unallocated_energy_j_m2, refined.ledger.unallocated_energy_j_m2, ENERGY_ABSOLUTE_TOLERANCE_J_M2),
        ]
    }

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
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
        F: FnMut(TerminalState, &J, f64, f64, CoveredTerminalTrialRoleV1, u32)
            -> Result<(TerminalFluxIntegral, J), DirectSnowStage3EvaluationError>,
        G: FnMut(TerminalState, J) -> Result<J, DirectSnowStage3EvaluationError>,
        J: Clone,
    {
        let mut evidence = <NoEvidence as TerminalEvidenceMode<J>>::new_state();
        Self::solve_terminal_enthalpy_event_with_evidence::<F, G, J, NoEvidence>(
            phase_class, hour_index, hour_offset_seconds, requested_seconds, start,
            initial_joint, flux_integral, join_hydrology_ending, &mut evidence,
        )
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
        evidence: &mut M::State,
    ) -> Result<(DirectSnowTerminalEventResult, J), DirectSnowStage3EvaluationError>
    where
        F: FnMut(
            TerminalState,
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
        let mut state = start;
        let mut accepted_joint = initial_joint.clone();
        let mut ledger = TerminalLedger::default();
        let mut elapsed = 0.0;
        let mut trial_seconds = requested_seconds.min(MAXIMUM_TRIAL_SECONDS);
        let mut accepted_trials = 0_u32;
        let mut rejected_trials = 0_u32;
        let mut consecutive_rejections = 0_u32;
        let mut maximum_scaled_error: f64 = 0.0;
        let mut attempt_ordinal = 0_u32;
        let next_attempt = |value: u32| {
            value
                .checked_add(1)
                .ok_or(DirectSnowStage3EvaluationError::TerminalNumerics(
                    SnowTerminalNumericsFailure::DomainOrNonFinite,
                ))
        };
        let mut event_bracket_width_seconds = 0.0;
        let mut event_bracket_lower_seconds = 0.0;
        let mut event_bracket_upper_seconds = 0.0;
        let mut event_bracket_lower_solid_kg_m2 = start.ice_kg_m2;
        let mut event_bracket_upper_solid_kg_m2 = start.ice_kg_m2;
        let mut lte_coarse = TerminalTrial {
            state: start,
            ledger: TerminalLedger::default(),
        };
        let mut lte_fine = lte_coarse;
        while elapsed < requested_seconds && state.ice_kg_m2 > 0.0 {
            let remaining = requested_seconds - elapsed;
            let dt = trial_seconds.min(remaining);
            if dt < 2.0 * MINIMUM_COVERED_CARRIER_SECONDS {
                let outcome = SnowTerminalNumericsFailure::BelowCarrierDomain;
                let calls = M::provider_call_count(evidence);
                M::admission(evidence, TerminalAdmissionEvidenceHook {
                    proposed_duration_s: dt,
                    required_half_duration_s: 0.5 * dt,
                    minimum_duration_s: MINIMUM_COVERED_CARRIER_SECONDS,
                    outcome: &outcome,
                    provider_calls_before: calls,
                    provider_calls_after: calls,
                });
                return Err(DirectSnowStage3EvaluationError::TerminalNumerics(outcome));
            }
            let full_role = if consecutive_rejections == 0 {
                CoveredTerminalTrialRoleV1::Full
            } else {
                CoveredTerminalTrialRoleV1::Retry
            };
            let full_attempt = attempt_ordinal;
            let (full_flux, full_carrier_joint) = flux_integral(
                state,
                &accepted_joint,
                elapsed,
                dt,
                full_role,
                attempt_ordinal,
            )?;
            attempt_ordinal = next_attempt(attempt_ordinal)?;
            let full = Self::terminal_transition(state, full_flux);
            let (full_joint, captured_full_carrier_joint) = if M::ENABLED {
                (join_hydrology_ending(full.state, full_carrier_joint.clone())?, Some(full_carrier_joint))
            } else {
                (join_hydrology_ending(full.state, full_carrier_joint)?, None)
            };
            if let Some(carrier_joint) = captured_full_carrier_joint.as_ref() { M::selected_trial(evidence, TerminalSelectedTrialHook {
                position: TerminalPairPosition::Coarse, role: full_role, attempt_ordinal: full_attempt,
                relative_start_s: elapsed, duration_s: dt, beginning: state.into(), ending: full.state.into(),
                ledger: full.ledger.into(), beginning_joint: &accepted_joint,
                carrier_ending_joint: carrier_joint, hydrology_ending_joint: &full_joint,
            }); }
            let half_dt = 0.5 * dt;
            let first_attempt = attempt_ordinal;
            let (first_flux, first_carrier_joint) = flux_integral(
                state,
                &accepted_joint,
                elapsed,
                half_dt,
                CoveredTerminalTrialRoleV1::Half1,
                attempt_ordinal,
            )?;
            attempt_ordinal = next_attempt(attempt_ordinal)?;
            let first = Self::terminal_transition(state, first_flux);
            let (first_joint, captured_first_carrier_joint) = if M::ENABLED {
                (join_hydrology_ending(first.state, first_carrier_joint.clone())?, Some(first_carrier_joint))
            } else {
                (join_hydrology_ending(first.state, first_carrier_joint)?, None)
            };
            if let Some(carrier_joint) = captured_first_carrier_joint.as_ref() { M::selected_trial(evidence, TerminalSelectedTrialHook {
                position: TerminalPairPosition::Fine1, role: CoveredTerminalTrialRoleV1::Half1, attempt_ordinal: first_attempt,
                relative_start_s: elapsed, duration_s: half_dt, beginning: state.into(), ending: first.state.into(),
                ledger: first.ledger.into(), beginning_joint: &accepted_joint,
                carrier_ending_joint: carrier_joint, hydrology_ending_joint: &first_joint,
            }); }
            let second_attempt = attempt_ordinal;
            let (second_flux, second_carrier_joint) = flux_integral(
                first.state,
                &first_joint,
                elapsed + half_dt,
                half_dt,
                CoveredTerminalTrialRoleV1::Half2,
                attempt_ordinal,
            )?;
            let second = Self::terminal_transition(first.state, second_flux);
            let (second_joint, captured_second_carrier_joint) = if M::ENABLED {
                (join_hydrology_ending(second.state, second_carrier_joint.clone())?, Some(second_carrier_joint))
            } else {
                (join_hydrology_ending(second.state, second_carrier_joint)?, None)
            };
            if let Some(carrier_joint) = captured_second_carrier_joint.as_ref() { M::selected_trial(evidence, TerminalSelectedTrialHook {
                position: TerminalPairPosition::Fine2, role: CoveredTerminalTrialRoleV1::Half2, attempt_ordinal: second_attempt,
                relative_start_s: elapsed + half_dt, duration_s: half_dt, beginning: first.state.into(), ending: second.state.into(),
                ledger: second.ledger.into(), beginning_joint: &first_joint,
                carrier_ending_joint: carrier_joint, hydrology_ending_joint: &second_joint,
            }); }
            attempt_ordinal = next_attempt(attempt_ordinal)?;
            let refined = TerminalTrial {
                state: second.state,
                ledger: first.ledger.add(second.ledger),
            };
            let error = Self::terminal_scaled_error(full, refined);
            let rejected = error > 1.0 && refined.state.ice_kg_m2 > 0.0;
            if M::ENABLED { M::pair(evidence, TerminalPairEvidenceHook {
                duration_s: dt,
                proposed_next_duration_s: if rejected { (0.5 * dt).max(MINIMUM_TRIAL_SECONDS) } else if error < 0.125 { (2.0 * dt).min(MAXIMUM_TRIAL_SECONDS) } else { dt },
                components: Self::terminal_error_components(full, refined),
                scaled_error: error,
                rejected,
            }); }
            // LTE is discontinuous across exhaustion because terminal
            // transition censors the coarse/fine residual energy at different
            // substep boundaries. Once the refined path brackets zero ice,
            // localize that physical event below instead of shrinking the
            // adaptive step forever against a non-smooth endpoint.
            if rejected {
                rejected_trials += 1;
                consecutive_rejections += 1;
                if consecutive_rejections > MAXIMUM_REJECTIONS || dt <= MINIMUM_TRIAL_SECONDS {
                    return Err(DirectSnowStage3EvaluationError::TerminalNumerics(
                        if dt <= MINIMUM_TRIAL_SECONDS {
                            SnowTerminalNumericsFailure::StepUnderflow
                        } else {
                            SnowTerminalNumericsFailure::RejectionLimit
                        },
                    ));
                }
                trial_seconds = (0.5 * dt).max(MINIMUM_TRIAL_SECONDS);
                continue;
            }
            consecutive_rejections = 0;
            if error >= maximum_scaled_error {
                maximum_scaled_error = error;
                lte_coarse = full;
                lte_fine = refined;
            }

            if refined.state.ice_kg_m2 <= 0.0 {
                // Root selection is over absolute cursor-prefix duration. Every
                // candidate restarts from the immutable cursor state/joint;
                // bracket-local state and support are never solver operands.
                let mut lower = elapsed;
                let mut upper = elapsed + dt;
                let (mut event, mut event_joint) = Self::terminal_prefix_candidate(
                    start,
                    &initial_joint,
                    upper,
                    &mut attempt_ordinal,
                    &mut flux_integral,
                    &mut join_hydrology_ending,
                )?;
                if event.state.ice_kg_m2 > 0.0 {
                    return Err(DirectSnowStage3EvaluationError::TerminalNumerics(
                        SnowTerminalNumericsFailure::InvalidEventBracket,
                    ));
                }
                let (lower_trial, _) = Self::terminal_prefix_candidate(
                    start,
                    &initial_joint,
                    lower,
                    &mut attempt_ordinal,
                    &mut flux_integral,
                    &mut join_hydrology_ending,
                )?;
                let mut lower_solid = lower_trial.state.ice_kg_m2;
                let mut upper_solid = event.state.ice_kg_m2;
                for _ in 0..MAXIMUM_EVENT_BISECTIONS {
                    if upper - lower <= EVENT_TIME_TOLERANCE_SECONDS {
                        break;
                    }
                    let middle = 0.5 * (lower + upper);
                    let (middle_trial, middle_joint) = Self::terminal_prefix_candidate(
                        start,
                        &initial_joint,
                        middle,
                        &mut attempt_ordinal,
                        &mut flux_integral,
                        &mut join_hydrology_ending,
                    )?;
                    if middle_trial.state.ice_kg_m2 > lower_solid
                        || middle_trial.state.ice_kg_m2 < upper_solid
                    {
                        return Err(DirectSnowStage3EvaluationError::TerminalNumerics(
                            SnowTerminalNumericsFailure::InvalidEventBracket,
                        ));
                    }
                    if middle_trial.state.ice_kg_m2 <= 0.0 {
                        upper = middle;
                        event = middle_trial;
                        event_joint = middle_joint;
                        upper_solid = middle_trial.state.ice_kg_m2;
                    } else {
                        lower = middle;
                        lower_solid = middle_trial.state.ice_kg_m2;
                    }
                }
                event_bracket_width_seconds = upper - lower;
                event_bracket_lower_seconds = lower;
                event_bracket_upper_seconds = upper;
                event_bracket_lower_solid_kg_m2 = lower_solid;
                event_bracket_upper_solid_kg_m2 = upper_solid;
                if event_bracket_width_seconds > EVENT_TIME_TOLERANCE_SECONDS
                    || event.state.ice_kg_m2 > 0.0
                {
                    return Err(DirectSnowStage3EvaluationError::TerminalNumerics(
                        SnowTerminalNumericsFailure::EventIterationLimit,
                    ));
                }
                state = event.state;
                accepted_joint = event_joint;
                ledger = event.ledger;
                elapsed = upper;
                accepted_trials += 1;
                break;
            }

            state = second.state;
            accepted_joint = second_joint;
            ledger = ledger.add(refined.ledger);
            elapsed += dt;
            accepted_trials += 1;
            trial_seconds = if error < 0.125 {
                (2.0 * dt).min(MAXIMUM_TRIAL_SECONDS)
            } else {
                dt
            };
        }

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
        Ok((DirectSnowTerminalEventResult {
            model: DirectSnowTerminalEventModel::EnthalpyEventV1,
            event_occurred,
            hour_index,
            terminal_entry_offset_seconds: hour_offset_seconds,
            requested_seconds,
            entry_solid_precipitation_kg_m2: 0.0,
            hour_offset_seconds: hour_offset_seconds + elapsed,
            evaluated_seconds: elapsed,
            unevaluated_seconds: (requested_seconds - elapsed).max(0.0),
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
        }, accepted_joint))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            |_, _, _, duration, _, _| {
                Ok((
                    TerminalFluxIntegral {
                        complete_energy_j_m2: energy_rate_w_m2 * duration,
                        vapor_mass_exchange_kg_m2: vapor_rate_kg_m2_s * duration,
                        shortwave_energy_j_m2: energy_rate_w_m2 * duration,
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
        .unwrap()
        .0
    }

    #[test]
    fn pure_melt_localizes_analytical_event() {
        let event = solve(
            TerminalState {
                ice_kg_m2: 0.6,
                liquid_kg_m2: 0.0,
                cold_content_j_m2: 0.0,
            },
            1_000.0,
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
        let mut evidence = <CaptureEvidence as TerminalEvidenceMode<Option<CoveredTerminalJointTrialStateV1>>>::new_state();
        let result = Wb11HydrologyKernel::solve_terminal_enthalpy_event_with_evidence::<
            _, _, _, CaptureEvidence,
        >(
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
            0,
            0.0,
            3.75,
            TerminalState { ice_kg_m2: 0.5, liquid_kg_m2: 0.0, cold_content_j_m2: 0.0 },
            None,
            |_, _, _, duration, _, _| Ok((TerminalFluxIntegral {
                complete_energy_j_m2: duration * duration,
                vapor_mass_exchange_kg_m2: 0.0,
                shortwave_energy_j_m2: duration * duration,
                longwave_energy_j_m2: 0.0,
                sensible_energy_j_m2: 0.0,
                latent_energy_j_m2: 0.0,
                advected_energy_j_m2: 0.0,
                snow_soil_heat_energy_j_m2: 0.0,
                external_liquid_kg_m2: 0.0,
            }, None)),
            |_, joint| Ok(joint),
            &mut evidence,
        );
        assert!(matches!(result, Err(DirectSnowStage3EvaluationError::TerminalNumerics(
            SnowTerminalNumericsFailure::BelowCarrierDomain
        ))));
        assert_eq!(evidence.pairs.last().unwrap().duration_s.to_bits(), 1.875_f64.to_bits());
        let admission = evidence.admissions.last().unwrap();
        assert_eq!(admission.0.to_bits(), 0.9375_f64.to_bits());
        assert_eq!(admission.1.to_bits(), 0.46875_f64.to_bits());
        assert_eq!(admission.2.to_bits(), 0.6_f64.to_bits());
        assert_eq!(admission.3, SnowTerminalNumericsFailure::BelowCarrierDomain);
        assert_eq!(admission.4, admission.5);
    }

    #[test]
    fn every_adaptive_and_root_trial_carries_its_exact_relative_start() {
        let mut trials = Vec::new();
        let (event, _) = Wb11HydrologyKernel::solve_terminal_enthalpy_event(
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
            0,
            0.0,
            1_000.0,
            TerminalState {
                ice_kg_m2: 0.6,
                liquid_kg_m2: 0.0,
                cold_content_j_m2: 0.0,
            },
            (),
            |_, _, relative_start, duration, role, attempt| {
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
                && start + duration <= 1_000.0
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
                .any(|(_, _, role, _)| *role == CoveredTerminalTrialRoleV1::Root)
        );
    }

    #[test]
    fn joint_trial_state_advances_only_along_the_accepted_fine_chain() {
        let mut observed = Vec::new();
        let (result, _) = Wb11HydrologyKernel::solve_terminal_enthalpy_event(
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
            0,
            0.0,
            10.0,
            TerminalState {
                ice_kg_m2: 0.5,
                liquid_kg_m2: 0.0,
                cold_content_j_m2: 0.0,
            },
            0_u32,
            |_, joint, _, duration, role, attempt| {
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
            Ok((TerminalFluxIntegral {
                complete_energy_j_m2: 333_600.0 * duration,
                vapor_mass_exchange_kg_m2: 0.0,
                shortwave_energy_j_m2: 333_600.0 * duration,
                longwave_energy_j_m2: 0.0,
                sensible_energy_j_m2: 0.0,
                latent_energy_j_m2: 0.0,
                advected_energy_j_m2: 0.0,
                snow_soil_heat_energy_j_m2: 0.0,
                external_liquid_kg_m2: 0.0,
            }, *joint + 1))
        };
        let mut join = |_: TerminalState, joint| Ok(joint);
        let (zero, zero_joint) = Wb11HydrologyKernel::terminal_prefix_candidate(
            start, &0, 0.0, &mut attempt, &mut flux, &mut join,
        ).unwrap();
        assert_eq!(zero.state.ice_kg_m2.to_bits(), start.ice_kg_m2.to_bits());
        assert_eq!(zero_joint, 0);
        let predecessor = Wb11HydrologyKernel::terminal_prefix_candidate(
            start, &0, 0.599_999_999, &mut attempt, &mut flux, &mut join,
        );
        assert!(matches!(predecessor, Err(DirectSnowStage3EvaluationError::TerminalNumerics(
            SnowTerminalNumericsFailure::BelowCarrierDomain
        ))));
        let (event, _) = Wb11HydrologyKernel::terminal_prefix_candidate(
            start, &0, 0.6, &mut attempt, &mut flux, &mut join,
        ).unwrap();
        assert_eq!(event.state.ice_kg_m2.to_bits(), 0.0_f64.to_bits());
    }

    #[test]
    fn pure_sublimation_and_joint_exhaustion_localize_same_event() {
        let start = TerminalState {
            ice_kg_m2: 0.6,
            liquid_kg_m2: 0.0,
            cold_content_j_m2: 0.0,
        };
        let sublimation = solve(start, 1_000.0, 0.0, -0.001);
        assert!((sublimation.evaluated_seconds - 600.0).abs() <= 1.0e-6);
        assert!(
            (sublimation.sublimation_kg_m2 - 0.6).abs() <= 1.0e-9,
            "{sublimation:?}"
        );
        let joint = solve(start, 1_000.0, 166.8, -0.0005);
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
            -556.0,
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
        let trial = Wb11HydrologyKernel::terminal_transition(
            TerminalState {
                ice_kg_m2: 0.6,
                liquid_kg_m2: 0.0,
                cold_content_j_m2: 0.0,
            },
            TerminalFluxIntegral {
                complete_energy_j_m2: 300_240.0,
                vapor_mass_exchange_kg_m2: -0.3,
                shortwave_energy_j_m2: 300_240.0,
                longwave_energy_j_m2: 0.0,
                sensible_energy_j_m2: 0.0,
                latent_energy_j_m2: 0.0,
                advected_energy_j_m2: 0.0,
                snow_soil_heat_energy_j_m2: 0.0,
                external_liquid_kg_m2: 0.0,
            },
        );
        assert!(trial.state.ice_kg_m2.abs() <= f64::EPSILON);
        assert!((trial.ledger.sublimation_kg_m2 - 0.3).abs() <= 1.0e-12);
        assert!((trial.ledger.melt_kg_m2 - 0.3).abs() <= 1.0e-12);
        assert!(trial.ledger.unallocated_energy_j_m2 > 100_000.0);
        let nondyadic = Wb11HydrologyKernel::terminal_transition(
            TerminalState {
                ice_kg_m2: 0.7,
                liquid_kg_m2: 0.0,
                cold_content_j_m2: 0.0,
            },
            TerminalFluxIntegral {
                complete_energy_j_m2: STAGE3_LATENT_HEAT_FUSION_J_KG * 0.5,
                vapor_mass_exchange_kg_m2: -0.2,
                shortwave_energy_j_m2: STAGE3_LATENT_HEAT_FUSION_J_KG * 0.5,
                longwave_energy_j_m2: 0.0,
                sensible_energy_j_m2: 0.0,
                latent_energy_j_m2: 0.0,
                advected_energy_j_m2: 0.0,
                snow_soil_heat_energy_j_m2: 0.0,
                external_liquid_kg_m2: 0.0,
            },
        );
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
                );
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
