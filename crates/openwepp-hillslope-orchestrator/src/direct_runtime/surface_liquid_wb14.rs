use crate::constants::WB11_ZERO_THRESHOLD;

use super::{
    DirectRuntimeError, validate_finite, validate_nonnegative_direct_m, validate_positive_direct,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct DirectWb14ContinuationIntervalInputs {
    pub cumulative_supply_m: f64,
    pub cumulative_infiltration_m: f64,
    pub interval_supply_m: f64,
    pub interval_duration_s: f64,
    pub effective_conductivity_m_s: f64,
    pub matric_potential_m: f64,
    pub storage_capacity_m: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct DirectWb14IntervalTransitionInputs {
    pub cumulative_supply_m: f64,
    pub cumulative_infiltration_m: f64,
    pub interval_supply_m: f64,
    pub interval_duration_s: f64,
    pub interval_intensity_m_s: f64,
    pub effective_conductivity_m_s: f64,
    pub matric_potential_m: f64,
    pub storage_capacity_m: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(clippy::struct_field_names)]
pub(super) struct DirectWb14ContinuationIntervalOutcome {
    pub cumulative_supply_m: f64,
    pub cumulative_infiltration_m: f64,
    pub interval_infiltration_m: f64,
    pub interval_excess_m: f64,
}

/// Own the complete production WB14 interval state transition.
///
/// The unchanged daily wrapper and the persistent 1800-second continuation
/// both enter here so storage exhaustion, guards, clamps, and cumulative-state
/// arithmetic cannot drift between those two execution shapes.
pub(super) fn advance_wb14_interval_state(
    inputs: DirectWb14IntervalTransitionInputs,
) -> Result<DirectWb14ContinuationIntervalOutcome, DirectRuntimeError> {
    validate_nonnegative_direct_m(
        "infiltration_depression.cumulative_supply_m",
        inputs.cumulative_supply_m,
    )?;
    validate_nonnegative_direct_m(
        "infiltration_depression.cumulative_infiltration_m",
        inputs.cumulative_infiltration_m,
    )?;
    validate_nonnegative_direct_m(
        "infiltration_depression.interval_supply_m",
        inputs.interval_supply_m,
    )?;
    validate_positive_direct(
        "infiltration_depression.interval_duration_s",
        inputs.interval_duration_s,
    )?;
    validate_nonnegative_direct_m(
        "infiltration_depression.interval_intensity_m_s",
        inputs.interval_intensity_m_s,
    )?;
    validate_positive_direct(
        "infiltration_depression.effective_conductivity_m_s",
        inputs.effective_conductivity_m_s,
    )?;
    validate_nonnegative_direct_m(
        "infiltration_depression.matric_potential_m",
        inputs.matric_potential_m,
    )?;
    validate_nonnegative_direct_m(
        "infiltration_depression.storage_capacity_m",
        inputs.storage_capacity_m,
    )?;
    if inputs.cumulative_infiltration_m > inputs.cumulative_supply_m
        || inputs.cumulative_infiltration_m > inputs.storage_capacity_m
    {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "infiltration_depression.continuation_bounds",
        });
    }

    let remaining_storage_m =
        (inputs.storage_capacity_m - inputs.cumulative_infiltration_m).max(0.0);
    let computed_interval_infiltration_m = if remaining_storage_m <= WB11_ZERO_THRESHOLD {
        0.0
    } else {
        super::runoff::compute_green_ampt_interval_infiltration(
            inputs.cumulative_infiltration_m,
            inputs.interval_supply_m.min(remaining_storage_m),
            inputs.interval_duration_s,
            inputs.interval_intensity_m_s,
            inputs.effective_conductivity_m_s,
            inputs.matric_potential_m,
        )?
    };
    if computed_interval_infiltration_m > inputs.interval_supply_m + 1.0e-9 {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "infiltration_depression.interval_infiltration_m",
        });
    }

    let cumulative_supply_m = inputs.cumulative_supply_m + inputs.interval_supply_m;
    validate_finite(
        "infiltration_depression.cumulative_supply_m",
        cumulative_supply_m,
    )?;
    let cumulative_infiltration_m = (inputs.cumulative_infiltration_m
        + computed_interval_infiltration_m.min(inputs.interval_supply_m))
    .min(inputs.storage_capacity_m)
    .min(cumulative_supply_m);
    validate_finite(
        "infiltration_depression.cumulative_infiltration_m",
        cumulative_infiltration_m,
    )?;
    let interval_excess_m = (inputs.interval_supply_m
        - (cumulative_infiltration_m - inputs.cumulative_infiltration_m))
        .max(0.0);
    validate_nonnegative_direct_m(
        "infiltration_depression.interval_excess_m",
        interval_excess_m,
    )?;
    let interval_infiltration_m = inputs.interval_supply_m - interval_excess_m;

    Ok(DirectWb14ContinuationIntervalOutcome {
        cumulative_supply_m,
        cumulative_infiltration_m,
        interval_infiltration_m,
        interval_excess_m,
    })
}

/// Advance the persistent continuation without replaying accepted day state.
pub(super) fn advance_wb14_continuation_interval(
    inputs: DirectWb14ContinuationIntervalInputs,
) -> Result<DirectWb14ContinuationIntervalOutcome, DirectRuntimeError> {
    advance_wb14_interval_state(DirectWb14IntervalTransitionInputs {
        cumulative_supply_m: inputs.cumulative_supply_m,
        cumulative_infiltration_m: inputs.cumulative_infiltration_m,
        interval_supply_m: inputs.interval_supply_m,
        interval_duration_s: inputs.interval_duration_s,
        interval_intensity_m_s: inputs.interval_supply_m / inputs.interval_duration_s,
        effective_conductivity_m_s: inputs.effective_conductivity_m_s,
        matric_potential_m: inputs.matric_potential_m,
        storage_capacity_m: inputs.storage_capacity_m,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::direct_runtime::runoff::{
        DC01_HOUR_BIN_COUNT, DirectWb14HyetographInterval, DirectWb14InfiltrationProducerInputs,
        compute_wb14_infiltration_depression_with_profile,
    };

    fn independent_ponded_oracle(
        cumulative_m: f64,
        rainfall_m: f64,
        duration_s: f64,
        intensity_m_s: f64,
        conductivity_m_s: f64,
        matric_m: f64,
    ) -> f64 {
        let threshold_m = conductivity_m_s * matric_m / (intensity_m_s - conductivity_m_s);
        let unponded_m = (threshold_m - cumulative_m).clamp(0.0, rainfall_m);
        let target_m = conductivity_m_s * (duration_s - unponded_m / intensity_m_s);
        let start_m = cumulative_m + unponded_m;
        let residual = |end_m: f64| {
            (end_m - start_m)
                - matric_m * ((end_m + matric_m) / (start_m + matric_m)).ln()
                - target_m
        };
        let mut low_m = start_m;
        let mut high_m = start_m + rainfall_m + matric_m;
        for _ in 0..160 {
            let mid_m = 0.5 * (low_m + high_m);
            if residual(mid_m) < 0.0 {
                low_m = mid_m;
            } else {
                high_m = mid_m;
            }
        }
        (unponded_m + 0.5 * (low_m + high_m) - start_m).min(rainfall_m)
    }

    fn assert_core_continuation_parity(
        inputs: DirectWb14ContinuationIntervalInputs,
    ) -> Result<DirectWb14ContinuationIntervalOutcome, DirectRuntimeError> {
        let direct = advance_wb14_interval_state(DirectWb14IntervalTransitionInputs {
            cumulative_supply_m: inputs.cumulative_supply_m,
            cumulative_infiltration_m: inputs.cumulative_infiltration_m,
            interval_supply_m: inputs.interval_supply_m,
            interval_duration_s: inputs.interval_duration_s,
            interval_intensity_m_s: inputs.interval_supply_m / inputs.interval_duration_s,
            effective_conductivity_m_s: inputs.effective_conductivity_m_s,
            matric_potential_m: inputs.matric_potential_m,
            storage_capacity_m: inputs.storage_capacity_m,
        });
        let continuation = advance_wb14_continuation_interval(inputs);
        assert_eq!(continuation, direct);
        continuation
    }

    fn assert_daily_continuation_parity(
        interval_supplies_m: &[f64],
        storage_capacity_m: f64,
        effective_conductivity_m_s: f64,
    ) -> DirectWb14ContinuationIntervalOutcome {
        let mut hyetograph = Vec::with_capacity(interval_supplies_m.len());
        for (index, supply_m) in interval_supplies_m.iter().copied().enumerate() {
            let start_s = f64::from(u32::try_from(index).expect("small test index")) * 1_800.0;
            hyetograph.push(DirectWb14HyetographInterval {
                start_s,
                end_s: start_s + 1_800.0,
                intensity_m_s: supply_m / 1_800.0,
            });
        }
        let inputs = DirectWb14InfiltrationProducerInputs {
            hyetograph,
            hourly_additional_supply_m: [0.0; DC01_HOUR_BIN_COUNT],
            effective_conductivity_m_s,
            matric_potential_m: 0.12,
            storage_capacity_m,
            depression_storage_capacity_m: 0.0,
        };
        let daily = compute_wb14_infiltration_depression_with_profile(&inputs)
            .expect("daily production wrapper");
        let mut outcome = DirectWb14ContinuationIntervalOutcome {
            cumulative_supply_m: 0.0,
            cumulative_infiltration_m: 0.0,
            interval_infiltration_m: 0.0,
            interval_excess_m: 0.0,
        };
        let mut continuation_excess_m = 0.0;
        for interval in &inputs.hyetograph {
            let interval_supply_m = interval.intensity_m_s * 1_800.0;
            outcome = advance_wb14_continuation_interval(DirectWb14ContinuationIntervalInputs {
                cumulative_supply_m: outcome.cumulative_supply_m,
                cumulative_infiltration_m: outcome.cumulative_infiltration_m,
                interval_supply_m,
                interval_duration_s: 1_800.0,
                effective_conductivity_m_s: inputs.effective_conductivity_m_s,
                matric_potential_m: inputs.matric_potential_m,
                storage_capacity_m: inputs.storage_capacity_m,
            })
            .expect("stateful interval");
            continuation_excess_m += outcome.interval_excess_m;
        }
        assert_eq!(
            outcome.cumulative_infiltration_m.to_bits(),
            daily.state.cumulative_infiltration_m.to_bits()
        );
        let daily_excess_m = daily.hourly_excess_m.iter().sum::<f64>();
        assert!((continuation_excess_m - daily_excess_m).abs() <= 1.0e-12);
        outcome
    }

    #[test]
    fn forty_eight_stateful_intervals_match_the_existing_daily_wb14_wrapper() {
        let outcome = assert_daily_continuation_parity(&[0.000_45; 48], 0.015, 1.1e-7);
        assert!(outcome.cumulative_infiltration_m <= 0.015);
    }

    #[test]
    fn zero_threshold_branch_matches_daily_wrapper() {
        let outcome = assert_daily_continuation_parity(&[0.000_45], WB11_ZERO_THRESHOLD, 1.1e-3);
        assert_eq!(outcome.interval_infiltration_m.to_bits(), 0.0_f64.to_bits());
        assert_eq!(
            outcome.interval_excess_m.to_bits(),
            outcome.cumulative_supply_m.to_bits()
        );
        assert_core_continuation_parity(DirectWb14ContinuationIntervalInputs {
            cumulative_supply_m: 0.001,
            cumulative_infiltration_m: 0.001,
            interval_supply_m: 0.000_45,
            interval_duration_s: 1_800.0,
            effective_conductivity_m_s: 1.1e-3,
            matric_potential_m: 0.12,
            storage_capacity_m: 0.001 + WB11_ZERO_THRESHOLD,
        })
        .expect("remaining-storage threshold branch");
    }

    #[test]
    fn storage_capacity_clamp_branch_matches_daily_wrapper() {
        let capacity_m = 0.000_8;
        let outcome = assert_daily_continuation_parity(&[0.000_45; 4], capacity_m, 1.1e-3);
        assert_eq!(
            outcome.cumulative_infiltration_m.to_bits(),
            capacity_m.to_bits()
        );
        assert!(outcome.interval_excess_m > 0.0);
    }

    #[test]
    fn roundoff_clamp_branch_matches_daily_wrapper() {
        let capacity_m = 0.1 + 0.2;
        let outcome =
            assert_daily_continuation_parity(&[0.1, 0.1, 0.1, 1.0e-8], capacity_m, 1.1e-3);
        assert_eq!(
            outcome.cumulative_infiltration_m.to_bits(),
            capacity_m.to_bits()
        );
        assert!(outcome.interval_excess_m >= 0.0);
    }

    #[test]
    fn beginning_bounds_guards_match_the_shared_transition() {
        let above_supply = DirectWb14ContinuationIntervalInputs {
            cumulative_supply_m: 0.001,
            cumulative_infiltration_m: 0.001_000_000_000_1,
            interval_supply_m: 0.000_45,
            interval_duration_s: 1_800.0,
            effective_conductivity_m_s: 1.1e-3,
            matric_potential_m: 0.12,
            storage_capacity_m: 0.002,
        };
        assert!(assert_core_continuation_parity(above_supply).is_err());

        let above_storage = DirectWb14ContinuationIntervalInputs {
            cumulative_supply_m: 0.002,
            cumulative_infiltration_m: 0.001_000_000_000_1,
            storage_capacity_m: 0.001,
            ..above_supply
        };
        assert!(assert_core_continuation_parity(above_storage).is_err());
    }

    #[test]
    fn variable_duration_shared_kernel_matches_independent_nonlinear_oracle() {
        let cumulative_m = 0.001_7;
        let intensity_m_s = 8.0e-6;
        let conductivity_m_s = 1.1e-7;
        let matric_m = 0.12;
        for duration_s in [75.0, 437.0, 1_125.0, 1_800.0] {
            let rainfall_m = intensity_m_s * duration_s;
            let actual = super::super::runoff::compute_green_ampt_interval_infiltration(
                cumulative_m,
                rainfall_m,
                duration_s,
                intensity_m_s,
                conductivity_m_s,
                matric_m,
            )
            .expect("shared Green-Ampt transition");
            let expected = independent_ponded_oracle(
                cumulative_m,
                rainfall_m,
                duration_s,
                intensity_m_s,
                conductivity_m_s,
                matric_m,
            );
            assert!(
                (actual - expected).abs() <= 1.0e-12,
                "duration={duration_s}"
            );
        }
    }

    #[test]
    fn nonlinear_partial_transition_is_not_proportional_full_bin_scaling() {
        let cumulative_m = 0.001_7;
        let intensity_m_s = 8.0e-6;
        let conductivity_m_s = 1.1e-7;
        let matric_m = 0.12;
        let partial_duration_s = 437.0;
        let partial = super::super::runoff::compute_green_ampt_interval_infiltration(
            cumulative_m,
            intensity_m_s * partial_duration_s,
            partial_duration_s,
            intensity_m_s,
            conductivity_m_s,
            matric_m,
        )
        .expect("partial transition");
        let full = super::super::runoff::compute_green_ampt_interval_infiltration(
            cumulative_m,
            intensity_m_s * 1_800.0,
            1_800.0,
            intensity_m_s,
            conductivity_m_s,
            matric_m,
        )
        .expect("full transition");
        let naive = full * partial_duration_s / 1_800.0;
        assert!((partial - naive).abs() > 1.0e-8);

        let outcome = advance_wb14_interval_state(DirectWb14IntervalTransitionInputs {
            cumulative_supply_m: 0.002_4,
            cumulative_infiltration_m: cumulative_m,
            interval_supply_m: intensity_m_s * partial_duration_s,
            interval_duration_s: partial_duration_s,
            interval_intensity_m_s: intensity_m_s,
            effective_conductivity_m_s: conductivity_m_s,
            matric_potential_m: matric_m,
            storage_capacity_m: 0.02,
        })
        .expect("partial shared wrapper with beginning storage state");
        assert!((outcome.interval_infiltration_m - partial).abs() <= 1.0e-12);
        assert_eq!(
            outcome.cumulative_supply_m,
            0.002_4 + intensity_m_s * partial_duration_s
        );
        assert!(
            (outcome.interval_infiltration_m + outcome.interval_excess_m
                - intensity_m_s * partial_duration_s)
                .abs()
                <= 1.0e-14
        );
    }
}
