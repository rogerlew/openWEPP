#[allow(clippy::wildcard_imports)]
use super::*;

const RELATIVE_ERROR_TOLERANCE: f64 = 1.0e-8;
const MASS_ABSOLUTE_TOLERANCE_KG_M2: f64 = 1.0e-9;
const ENERGY_ABSOLUTE_TOLERANCE_J_M2: f64 = 1.0e-6;
const MINIMUM_TRIAL_SECONDS: f64 = 1.0e-9;
const MAXIMUM_TRIAL_SECONDS: f64 = 60.0;
const EVENT_TIME_TOLERANCE_SECONDS: f64 = 1.0e-6;
const MAXIMUM_REJECTIONS: u32 = 64;
const MAXIMUM_EVENT_BISECTIONS: u32 = 64;

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
            complete_energy_j_m2: self.complete_energy_j_m2
                + other.complete_energy_j_m2,
            cold_energy_change_j_m2: self.cold_energy_change_j_m2
                + other.cold_energy_change_j_m2,
            refrozen_kg_m2: self.refrozen_kg_m2 + other.refrozen_kg_m2,
            deposition_kg_m2: self.deposition_kg_m2 + other.deposition_kg_m2,
            sublimation_kg_m2: self.sublimation_kg_m2 + other.sublimation_kg_m2,
            melt_kg_m2: self.melt_kg_m2 + other.melt_kg_m2,
            unallocated_energy_j_m2: self.unallocated_energy_j_m2
                + other.unallocated_energy_j_m2,
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

#[derive(Clone, Copy)]
struct TerminalTrial {
    state: TerminalState,
    ledger: TerminalLedger,
}

impl Wb11HydrologyKernel {
    fn terminal_transition(
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
            let refrozen = (state.cold_content_j_m2 / STAGE3_LATENT_HEAT_FUSION_J_KG)
                .min(state.liquid_kg_m2);
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
        let mass = |a: f64, b: f64| {
            (a - b).abs()
                / (MASS_ABSOLUTE_TOLERANCE_KG_M2
                    + RELATIVE_ERROR_TOLERANCE * a.abs().max(b.abs()))
        };
        let energy = |a: f64, b: f64| {
            (a - b).abs()
                / (ENERGY_ABSOLUTE_TOLERANCE_J_M2
                    + RELATIVE_ERROR_TOLERANCE * a.abs().max(b.abs()))
        };
        mass(full.state.ice_kg_m2, refined.state.ice_kg_m2)
            .max(mass(full.state.liquid_kg_m2, refined.state.liquid_kg_m2))
            .max(energy(
                full.state.cold_content_j_m2,
                refined.state.cold_content_j_m2,
            ))
            .max(energy(
                full.ledger.complete_energy_j_m2,
                refined.ledger.complete_energy_j_m2,
            ))
            .max(energy(
                full.ledger.unallocated_energy_j_m2,
                refined.ledger.unallocated_energy_j_m2,
            ))
    }

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    pub(super) fn solve_terminal_enthalpy_event<F>(
        _phase_class: HillslopeKernelPhaseClass,
        hour_index: usize,
        hour_offset_seconds: f64,
        requested_seconds: f64,
        start: TerminalState,
        mut flux_integral: F,
    ) -> Result<DirectSnowTerminalEventResult, DirectSnowStage3EvaluationError>
    where
        F: FnMut(TerminalState, f64, f64) -> Result<TerminalFluxIntegral, DirectSnowStage3EvaluationError>,
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
        let mut ledger = TerminalLedger::default();
        let mut elapsed = 0.0;
        let mut trial_seconds = requested_seconds.min(MAXIMUM_TRIAL_SECONDS);
        let mut accepted_trials = 0_u32;
        let mut rejected_trials = 0_u32;
        let mut consecutive_rejections = 0_u32;
        let mut maximum_scaled_error: f64 = 0.0;
        let mut event_bracket_width_seconds = 0.0;
        let mut event_bracket_lower_seconds = 0.0;
        let mut event_bracket_upper_seconds = 0.0;
        let mut event_bracket_lower_solid_kg_m2 = start.ice_kg_m2;
        let mut event_bracket_upper_solid_kg_m2 = start.ice_kg_m2;
        let mut lte_coarse = TerminalTrial { state: start, ledger: TerminalLedger::default() };
        let mut lte_fine = lte_coarse;
        while elapsed < requested_seconds && state.ice_kg_m2 > 0.0 {
            let remaining = requested_seconds - elapsed;
            let dt = trial_seconds.min(remaining);
            let full_flux = flux_integral(state, elapsed, dt)?;
            let full = Self::terminal_transition(state, full_flux);
            let half_dt = 0.5 * dt;
            let first = Self::terminal_transition(state, flux_integral(state, elapsed, half_dt)?);
            let second = Self::terminal_transition(
                first.state,
                flux_integral(first.state, elapsed + half_dt, half_dt)?,
            );
            let refined = TerminalTrial {
                state: second.state,
                ledger: first.ledger.add(second.ledger),
            };
            let error = Self::terminal_scaled_error(full, refined);
            if error > 1.0 {
                rejected_trials += 1;
                consecutive_rejections += 1;
                if consecutive_rejections > MAXIMUM_REJECTIONS
                    || dt <= MINIMUM_TRIAL_SECONDS
                {
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
                let (event_start, event_prefix_seconds, event_limit_seconds, prefix_ledger) =
                    if first.state.ice_kg_m2 <= 0.0 {
                        (state, 0.0, half_dt, TerminalLedger::default())
                    } else {
                        (first.state, half_dt, half_dt, first.ledger)
                    };
                let mut lower = 0.0;
                let mut upper = event_limit_seconds;
                let mut lower_solid = event_start.ice_kg_m2;
                let mut event = Self::terminal_transition(
                    event_start,
                    flux_integral(event_start, elapsed + event_prefix_seconds, upper)?,
                );
                if event.state.ice_kg_m2 > 0.0 {
                    return Err(DirectSnowStage3EvaluationError::TerminalNumerics(
                        SnowTerminalNumericsFailure::InvalidEventBracket,
                    ));
                }
                let mut upper_solid = event.state.ice_kg_m2;
                for _ in 0..MAXIMUM_EVENT_BISECTIONS {
                    if upper - lower <= EVENT_TIME_TOLERANCE_SECONDS {
                        break;
                    }
                    let middle = 0.5 * (lower + upper);
                    let middle_trial = Self::terminal_transition(
                        event_start,
                        flux_integral(
                            event_start,
                            elapsed + event_prefix_seconds,
                            middle,
                        )?,
                    );
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
                        upper_solid = middle_trial.state.ice_kg_m2;
                    } else {
                        lower = middle;
                        lower_solid = middle_trial.state.ice_kg_m2;
                    }
                }
                event_bracket_width_seconds = upper - lower;
                event_bracket_lower_seconds = event_prefix_seconds + lower;
                event_bracket_upper_seconds = event_prefix_seconds + upper;
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
                ledger = ledger.add(prefix_ledger).add(event.ledger);
                elapsed += event_prefix_seconds + upper;
                accepted_trials += 1;
                break;
            }

            state = second.state;
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
        let solid_residual = start.ice_kg_m2 + ledger.refrozen_kg_m2
            + ledger.deposition_kg_m2
            - ledger.sublimation_kg_m2
            - ledger.melt_kg_m2
            - state.ice_kg_m2;
        let liquid_residual = start.liquid_kg_m2 + ledger.external_liquid_kg_m2
            + ledger.melt_kg_m2
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
            + STAGE3_LATENT_HEAT_FUSION_J_KG
                * (ledger.melt_kg_m2 + ledger.refrozen_kg_m2)
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
        })
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
            |_, _, duration| {
                Ok(TerminalFluxIntegral {
                    complete_energy_j_m2: energy_rate_w_m2 * duration,
                    vapor_mass_exchange_kg_m2: vapor_rate_kg_m2_s * duration,
                    shortwave_energy_j_m2: energy_rate_w_m2 * duration,
                    longwave_energy_j_m2: 0.0,
                    sensible_energy_j_m2: 0.0,
                    latent_energy_j_m2: 0.0,
                    advected_energy_j_m2: 0.0,
                    snow_soil_heat_energy_j_m2: 0.0,
                    external_liquid_kg_m2: 0.0,
                })
            },
        )
        .unwrap()
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
    fn every_adaptive_and_root_trial_carries_its_exact_relative_start() {
        let mut trials = Vec::new();
        let event = Wb11HydrologyKernel::solve_terminal_enthalpy_event(
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
            0,
            0.0,
            1_000.0,
            TerminalState {
                ice_kg_m2: 0.6,
                liquid_kg_m2: 0.0,
                cold_content_j_m2: 0.0,
            },
            |_, relative_start, duration| {
                trials.push((relative_start, duration));
                Ok(TerminalFluxIntegral {
                    complete_energy_j_m2: 333.6 * duration,
                    vapor_mass_exchange_kg_m2: 0.0,
                    shortwave_energy_j_m2: 333.6 * duration,
                    longwave_energy_j_m2: 0.0,
                    sensible_energy_j_m2: 0.0,
                    latent_energy_j_m2: 0.0,
                    advected_energy_j_m2: 0.0,
                    snow_soil_heat_energy_j_m2: 0.0,
                    external_liquid_kg_m2: 0.0,
                })
            },
        )
        .unwrap();
        assert!(event.event_occurred);
        assert!(trials.iter().all(|(start, duration)| {
            start.is_finite()
                && duration.is_finite()
                && *start >= 0.0
                && *duration > 0.0
                && start + duration <= 1_000.0
        }));
        assert!(trials.iter().any(|(start, _)| *start > 0.0));
        assert!(trials.windows(2).any(|pair| pair[0].0 == pair[1].0));
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
        assert!((sublimation.sublimation_kg_m2 - 0.6).abs() <= 1.0e-9, "{sublimation:?}");
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
