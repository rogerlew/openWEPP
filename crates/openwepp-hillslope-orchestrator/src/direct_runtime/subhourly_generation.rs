#[cfg(test)]
use super::runoff::compute_wb14_subhourly_profile;
use super::runoff::{
    DC01_HOUR_BIN_COUNT, WAT5_INTERVAL_SECONDS, WAT5_INTERVALS_PER_DAY, WAT5_INTERVALS_PER_HOUR,
    Wat5AdditionalSupplySegmentV1, compute_wb14_subhourly_profile_with_exact_segments,
    reconcile_wat5_zero_raw_generation_hour_v1,
};
use super::{DirectDayFrame, DirectRuntimeError, validate_finite, validate_nonnegative_direct_m};
use openwepp_land_surface_energy::OfeId;

const WAT5_CLOSURE_TOLERANCE_M: f64 = 1.0e-12;
const WAT5_INTERVALS_PER_HOUR_F64: f64 = 12.0;

#[derive(Debug, Clone, PartialEq)]
pub struct DirectFiveMinuteGenerationInterval {
    pub hour_index: usize,
    pub subinterval_index: usize,
    pub interval_start_s: f64,
    pub interval_duration_s: f64,
    pub rainfall_depth_m: f64,
    pub additional_supply_depth_m: f64,
    pub raw_green_ampt_infiltration_depth_m: f64,
    pub depression_storage_retention_depth_m: f64,
    pub raw_wb14_post_depression_generation_depth_m: f64,
    pub closed_wb14_generation_depth_m: f64,
    pub saturation_return_depth_m: f64,
    pub closing_surface_generation_depth_m: f64,
    pub closing_surface_generation_intensity_m_s: f64,
    pub hourly_authoritative_runoff_depth_m: f64,
    pub hourly_mean_generation_intensity_m_s: f64,
    pub hourly_power_equivalent_generation_intensity_m_s: Option<f64>,
    pub hourly_power_equivalent_duration_s: Option<f64>,
    pub power_exponent: Option<f64>,
    pub method_code: &'static str,
    pub source_completeness_code: &'static str,
    pub hourly_closure_residual_m: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectFiveMinuteGenerationEvent {
    pub event_ordinal: u32,
    pub first_active_subinterval: Option<usize>,
    pub last_active_subinterval: Option<usize>,
    pub intervals: Vec<DirectFiveMinuteGenerationInterval>,
}

impl DirectFiveMinuteGenerationEvent {
    fn dry() -> Self {
        Self {
            event_ordinal: 0,
            first_active_subinterval: None,
            last_active_subinterval: None,
            intervals: Vec::new(),
        }
    }
}

fn validate_wat5_closure(
    observed_m: f64,
    authoritative_m: f64,
    field: &'static str,
) -> Result<f64, DirectRuntimeError> {
    validate_finite("wat5.observed_generation_depth_m", observed_m)?;
    validate_finite("wat5.authoritative_generation_depth_m", authoritative_m)?;
    let residual_m = observed_m - authoritative_m;
    let tolerance_m = WAT5_CLOSURE_TOLERANCE_M * authoritative_m.abs().max(1.0);
    if residual_m.abs() > tolerance_m {
        return Err(DirectRuntimeError::DirectClosureToleranceExceeded { field });
    }
    Ok(residual_m)
}

impl DirectDayFrame {
    /// Builds the optional SC-OUTPUT-WAT5-001 diagnostic after the closing
    /// WB14/WB19 hourly ledger is authoritative. The result is publication
    /// only and is not read by peak, routing, HBP, or erosion code.
    #[allow(clippy::too_many_lines)]
    pub fn run_wat5_subhourly_generation(&mut self) -> Result<(), DirectRuntimeError> {
        self.run_wat5_subhourly_generation_with_segments(&[], None, None)
    }

    #[cfg(test)]
    pub(crate) fn run_wat5_subhourly_generation_with_exact_segments(
        &mut self,
        segments: &[Wat5AdditionalSupplySegmentV1],
        destination_ofe_id: &OfeId,
    ) -> Result<(), DirectRuntimeError> {
        self.run_wat5_subhourly_generation_with_segments(segments, Some(destination_ofe_id), None)
    }

    pub(crate) fn run_wat5_subhourly_generation_with_accepted_profile(
        &mut self,
        profile: super::runoff::DirectWb14SubhourlyProfile,
    ) -> Result<(), DirectRuntimeError> {
        self.run_wat5_subhourly_generation_with_segments(&[], None, Some(profile))
    }

    #[allow(clippy::too_many_lines)]
    fn run_wat5_subhourly_generation_with_segments(
        &mut self,
        segments: &[Wat5AdditionalSupplySegmentV1],
        destination_ofe_id: Option<&OfeId>,
        accepted_profile: Option<super::runoff::DirectWb14SubhourlyProfile>,
    ) -> Result<(), DirectRuntimeError> {
        self.wat5_subhourly_generation = None;
        if !self.wat5_subhourly_requested {
            return Ok(());
        }

        let Some(producer_inputs) = self.infiltration_depression_inputs.producer_inputs.as_ref()
        else {
            if self.runoff_downstream_operands.q_runoff_m > 0.0 {
                return Err(DirectRuntimeError::MissingDirectUpstream {
                    upstream: "five-minute generation producer inputs for positive runoff",
                });
            }
            self.wat5_subhourly_generation = Some(Box::new(DirectFiveMinuteGenerationEvent::dry()));
            return Ok(());
        };

        let raw = if let Some(profile) = accepted_profile {
            profile
        } else {
            compute_wb14_subhourly_profile_with_exact_segments(
                producer_inputs,
                segments,
                destination_ofe_id,
            )?
        };
        let raw_supply_m: f64 =
            raw.rainfall_m.iter().sum::<f64>() + raw.additional_supply_m.iter().sum::<f64>();
        let raw_accounted_m: f64 = raw.infiltration_m.iter().sum::<f64>()
            + raw.depression_storage_retention_m.iter().sum::<f64>()
            + raw.post_depression_excess_m.iter().sum::<f64>();
        validate_wat5_closure(
            raw.depression_storage_retention_m.iter().sum(),
            raw.depression_storage_delta_m,
            "WAT5-E-004 wat5.depression_storage_retention_residual_m",
        )?;
        validate_wat5_closure(
            raw_accounted_m,
            raw_supply_m,
            "WAT5-E-004 wat5.raw_event_closure_residual_m",
        )?;
        let saturation = self
            .subsurface_compute_shadow_projection
            .as_ref()
            .map_or([0.0; DC01_HOUR_BIN_COUNT], |projection| {
                projection.hourly_saturation_carry_m
            });

        let mut closed_wb14 = [0.0_f64; WAT5_INTERVALS_PER_DAY];
        let mut hourly_residual = [0.0_f64; DC01_HOUR_BIN_COUNT];
        let mut daily_observed_m = 0.0_f64;
        let mut daily_authoritative_m = 0.0_f64;
        for hour in 0..DC01_HOUR_BIN_COUNT {
            validate_nonnegative_direct_m(
                "wat5.authoritative_wb14_generation_depth_m",
                self.wb14_hourly_excess_m[hour],
            )?;
            validate_nonnegative_direct_m("wat5.saturation_return_depth_m", saturation[hour])?;
            let start = hour * WAT5_INTERVALS_PER_HOUR;
            let end = start + WAT5_INTERVALS_PER_HOUR;
            let raw_total_m: f64 = raw.post_depression_excess_m[start..end].iter().sum();
            validate_finite("wat5.raw_hourly_generation_depth_m", raw_total_m)?;
            let authority_m = self.wb14_hourly_excess_m[hour];
            if authority_m > 0.0 && raw_total_m == 0.0 {
                let reconciliation = reconcile_wat5_zero_raw_generation_hour_v1(
                    hour,
                    &self.wb14_hourly_excess_m,
                    self.runoff_partition.partition_runoff_m,
                    &raw,
                )?;
                closed_wb14[start..end].copy_from_slice(&reconciliation.closing_ledger_m);
            } else if authority_m > 0.0 {
                let scale = authority_m / raw_total_m;
                for (closed_m, raw_m) in closed_wb14[start..end]
                    .iter_mut()
                    .zip(&raw.post_depression_excess_m[start..end])
                {
                    *closed_m = *raw_m * scale;
                }
            }
            let saturation_per_bin_m = saturation[hour] / WAT5_INTERVALS_PER_HOUR_F64;
            let observed_m: f64 = closed_wb14[start..end]
                .iter()
                .map(|closed_m| *closed_m + saturation_per_bin_m)
                .sum();
            let authoritative_m = authority_m + saturation[hour];
            hourly_residual[hour] = validate_wat5_closure(
                observed_m,
                authoritative_m,
                "WAT5-E-004 wat5.hourly_closure_residual_m",
            )?;
            daily_observed_m += observed_m;
            daily_authoritative_m += authoritative_m;
            validate_finite("wat5.daily_observed_generation_depth_m", daily_observed_m)?;
            validate_finite(
                "wat5.daily_authoritative_generation_depth_m",
                daily_authoritative_m,
            )?;
        }
        validate_wat5_closure(
            daily_observed_m,
            daily_authoritative_m,
            "WAT5-E-004 wat5.daily_closure_residual_m",
        )?;

        let is_active = |bin: usize| {
            let hour = bin / WAT5_INTERVALS_PER_HOUR;
            raw.rainfall_m[bin] > 0.0
                || raw.additional_supply_m[bin] > 0.0
                || raw.infiltration_m[bin] > 0.0
                || raw.post_depression_excess_m[bin] > 0.0
                || closed_wb14[bin] > 0.0
                || saturation[hour] > 0.0
        };
        let first_active_subinterval = (0..WAT5_INTERVALS_PER_DAY).find(|bin| is_active(*bin));
        let last_active_subinterval = (0..WAT5_INTERVALS_PER_DAY)
            .rev()
            .find(|bin| is_active(*bin));
        let Some(first) = first_active_subinterval else {
            self.wat5_subhourly_generation = Some(Box::new(DirectFiveMinuteGenerationEvent::dry()));
            return Ok(());
        };
        let last = last_active_subinterval.unwrap_or(first);
        let mut intervals = Vec::with_capacity(last - first + 1);
        for (bin, closed_wb14_generation_depth_m) in closed_wb14
            .iter()
            .copied()
            .enumerate()
            .take(last + 1)
            .skip(first)
        {
            let hour = bin / WAT5_INTERVALS_PER_HOUR;
            let saturation_return_depth_m = saturation[hour] / WAT5_INTERVALS_PER_HOUR_F64;
            let closing_surface_generation_depth_m =
                closed_wb14_generation_depth_m + saturation_return_depth_m;
            let hourly_authoritative_runoff_depth_m =
                self.wb14_hourly_excess_m[hour] + saturation[hour];
            let bin_u32 =
                u32::try_from(bin).map_err(|_| DirectRuntimeError::DirectDomainViolation {
                    field: "wat5.subinterval_index",
                })?;
            intervals.push(DirectFiveMinuteGenerationInterval {
                hour_index: hour,
                subinterval_index: bin,
                interval_start_s: f64::from(bin_u32) * WAT5_INTERVAL_SECONDS,
                interval_duration_s: WAT5_INTERVAL_SECONDS,
                rainfall_depth_m: raw.rainfall_m[bin],
                additional_supply_depth_m: raw.additional_supply_m[bin],
                raw_green_ampt_infiltration_depth_m: raw.infiltration_m[bin],
                depression_storage_retention_depth_m: raw.depression_storage_retention_m[bin],
                raw_wb14_post_depression_generation_depth_m: raw.post_depression_excess_m[bin],
                closed_wb14_generation_depth_m,
                saturation_return_depth_m,
                closing_surface_generation_depth_m,
                closing_surface_generation_intensity_m_s: closing_surface_generation_depth_m
                    / WAT5_INTERVAL_SECONDS,
                hourly_authoritative_runoff_depth_m,
                hourly_mean_generation_intensity_m_s: hourly_authoritative_runoff_depth_m / 3_600.0,
                hourly_power_equivalent_generation_intensity_m_s: None,
                hourly_power_equivalent_duration_s: None,
                power_exponent: None,
                method_code: "water_only_no_erosion_adoption",
                source_completeness_code:
                    "rainfall_and_exact_typed_additional_segments_saturation_hourly_zero_order_hold",
                hourly_closure_residual_m: hourly_residual[hour],
            });
        }

        self.wat5_subhourly_generation = Some(Box::new(DirectFiveMinuteGenerationEvent {
            event_ordinal: 0,
            first_active_subinterval,
            last_active_subinterval,
            intervals,
        }));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::direct_runtime::runoff::{
        Wat5AdditionalSupplySourceKindV1, compute_wb14_infiltration_depression_with_profile,
        wat5_additional_supply_source_receipt_sha256, wat5_hourly_additional_supply_from_segments,
    };
    use crate::{
        DirectInfiltrationDepressionInputs, DirectRunIdentity,
        DirectSubsurfaceComputeShadowProjection, DirectWb14HyetographInterval,
        DirectWb14InfiltrationProducerInputs,
    };

    fn assert_f64_slices_bit_equal(actual: &[f64], expected: &[f64]) {
        assert_eq!(actual.len(), expected.len());
        assert!(
            actual
                .iter()
                .zip(expected)
                .all(|(actual, expected)| actual.to_bits() == expected.to_bits())
        );
    }

    fn inputs(
        mut hyetograph: Vec<DirectWb14HyetographInterval>,
    ) -> DirectWb14InfiltrationProducerInputs {
        if hyetograph.is_empty() {
            hyetograph.push(DirectWb14HyetographInterval {
                start_s: 0.0,
                end_s: 0.0,
                intensity_m_s: 0.0,
            });
        }
        DirectWb14InfiltrationProducerInputs {
            hyetograph,
            hourly_additional_supply_m: [0.0; 24],
            effective_conductivity_m_s: 1.0e-8,
            matric_potential_m: 0.1,
            storage_capacity_m: 1.0,
            depression_storage_capacity_m: 0.0,
        }
    }

    fn day_with(inputs: DirectWb14InfiltrationProducerInputs) -> DirectDayFrame {
        let identity = DirectRunIdentity::new(1, 1, 1, 1).expect("test identity");
        let outcome =
            compute_wb14_infiltration_depression_with_profile(&inputs).expect("test WB14 outcome");
        let mut day = DirectDayFrame::seed(identity, 0, 0).expect("test day");
        day.wat5_subhourly_requested = true;
        day.wb14_hourly_excess_m = outcome.hourly_excess_m;
        day.runoff_downstream_operands.q_runoff_m = outcome.hourly_excess_m.iter().sum();
        day.infiltration_depression_inputs = DirectInfiltrationDepressionInputs {
            cumulative_infiltration_handoff_m: 0.0,
            depression_storage_delta_handoff_m: 0.0,
            producer_inputs: Some(inputs),
        };
        day
    }

    fn wat5_reconciliation_segment(
        source_identity: &str,
        start_s: f64,
        end_s: f64,
        depth_m_ofe_ground: f64,
        destination_ofe_id: &OfeId,
    ) -> Wat5AdditionalSupplySegmentV1 {
        let source_kind = Wat5AdditionalSupplySourceKindV1::LitterPhaseOverflow;
        let transaction_id = "wat5-v4-reconciliation-transaction".to_owned();
        let source_receipt_sha256 = wat5_additional_supply_source_receipt_sha256(
            source_kind,
            source_identity,
            &transaction_id,
            destination_ofe_id,
            start_s,
            end_s,
            depth_m_ofe_ground,
        )
        .expect("seal exact WAT5 source segment");
        Wat5AdditionalSupplySegmentV1 {
            source_kind,
            source_identity: source_identity.to_owned(),
            source_receipt_sha256,
            transaction_id,
            destination_ofe_id: destination_ofe_id.clone(),
            start_s,
            end_s,
            depth_m_ofe_ground,
        }
    }

    fn wat5_reconciliation_case() -> (
        DirectWb14InfiltrationProducerInputs,
        Vec<Wat5AdditionalSupplySegmentV1>,
        OfeId,
    ) {
        let destination_ofe_id = OfeId::try_new("wat5-v4-destination").expect("test OFE");
        let segments = vec![
            wat5_reconciliation_segment(
                "litter-overflow-7500-7800",
                7_500.0,
                7_800.0,
                3.0e-7,
                &destination_ofe_id,
            ),
            wat5_reconciliation_segment(
                "litter-overflow-10500-10800",
                10_500.0,
                10_800.0,
                6.0e-7,
                &destination_ofe_id,
            ),
        ];
        let mut producer = inputs(Vec::new());
        producer.hourly_additional_supply_m =
            wat5_hourly_additional_supply_from_segments(&segments, Some(&destination_ofe_id))
                .expect("reconstruct exact hourly additional supply");
        (producer, segments, destination_ofe_id)
    }

    const TRACED_WAT5_CLOSING_RESIDUAL_M: f64 = 2.998_903_209_094_905_3e-19;

    #[test]
    fn dry_day_emits_an_empty_event() {
        let mut day = day_with(inputs(Vec::new()));
        day.run_wat5_subhourly_generation().expect("dry WAT5");
        let event = day.wat5_subhourly_generation.expect("dry event");
        assert!(event.intervals.is_empty());
        assert_eq!(event.first_active_subinterval, None);
        assert_eq!(event.last_active_subinterval, None);
    }

    #[test]
    fn pulse_crossing_a_boundary_closes_to_wb14_and_keeps_power_null() {
        let mut day = day_with(inputs(vec![DirectWb14HyetographInterval {
            start_s: 250.0,
            end_s: 650.0,
            intensity_m_s: 5.0e-5,
        }]));
        day.run_wat5_subhourly_generation().expect("rain WAT5");
        let event = day.wat5_subhourly_generation.expect("rain event");
        assert_eq!(event.first_active_subinterval, Some(0));
        assert_eq!(event.last_active_subinterval, Some(2));
        let closed_m: f64 = event
            .intervals
            .iter()
            .map(|interval| interval.closed_wb14_generation_depth_m)
            .sum();
        assert!((closed_m - day.wb14_hourly_excess_m[0]).abs() <= WAT5_CLOSURE_TOLERANCE_M);
        assert!(event.intervals.iter().all(|interval| {
            interval.power_exponent.is_none()
                && interval
                    .hourly_power_equivalent_generation_intensity_m_s
                    .is_none()
                && interval.hourly_power_equivalent_duration_s.is_none()
        }));
    }

    #[test]
    fn exact_and_hour_crossing_boundaries_use_day_wide_bins() {
        let exact = compute_wb14_subhourly_profile(&inputs(vec![DirectWb14HyetographInterval {
            start_s: 300.0,
            end_s: 600.0,
            intensity_m_s: 1.0e-5,
        }]))
        .expect("exact boundary replay");
        assert!(exact.rainfall_m[0].abs() <= f64::EPSILON);
        assert!((exact.rainfall_m[1] - 0.003).abs() <= f64::EPSILON);
        assert!(exact.rainfall_m[2].abs() <= f64::EPSILON);

        let mut day = day_with(inputs(vec![DirectWb14HyetographInterval {
            start_s: 3_500.0,
            end_s: 3_700.0,
            intensity_m_s: 5.0e-5,
        }]));
        day.run_wat5_subhourly_generation()
            .expect("hour-crossing WAT5");
        let event = day.wat5_subhourly_generation.expect("hour-crossing event");
        assert_eq!(event.first_active_subinterval, Some(11));
        assert_eq!(event.last_active_subinterval, Some(12));
        assert_eq!(event.intervals[0].subinterval_index, 11);
        assert_eq!(event.intervals[1].subinterval_index, 12);
        assert!((event.intervals[0].rainfall_depth_m - 0.005).abs() <= f64::EPSILON);
        assert!((event.intervals[1].rainfall_depth_m - 0.005).abs() <= f64::EPSILON);
    }

    #[test]
    fn multiple_source_intervals_accumulate_within_one_bin() {
        let raw = compute_wb14_subhourly_profile(&inputs(vec![
            DirectWb14HyetographInterval {
                start_s: 0.0,
                end_s: 100.0,
                intensity_m_s: 1.0e-5,
            },
            DirectWb14HyetographInterval {
                start_s: 100.0,
                end_s: 200.0,
                intensity_m_s: 2.0e-5,
            },
        ]))
        .expect("multi-interval replay");
        assert!((raw.rainfall_m[0] - 0.003).abs() <= f64::EPSILON);
        assert!(raw.rainfall_m[1].abs() <= f64::EPSILON);
    }

    #[test]
    fn nonponding_rain_is_retained_as_diagnostic_support_without_runoff() {
        let mut producer = inputs(vec![DirectWb14HyetographInterval {
            start_s: 300.0,
            end_s: 600.0,
            intensity_m_s: 1.0e-9,
        }]);
        producer.effective_conductivity_m_s = 1.0e-5;
        let mut day = day_with(producer);
        day.run_wat5_subhourly_generation()
            .expect("nonponding WAT5");
        let event = day.wat5_subhourly_generation.expect("nonponding event");
        assert_eq!(event.intervals.len(), 1);
        assert!(event.intervals[0].rainfall_depth_m > 0.0);
        assert!(event.intervals[0].closed_wb14_generation_depth_m.abs() <= f64::EPSILON);
    }

    #[test]
    fn positive_additional_supply_fails_without_subhourly_timing() {
        let mut producer = inputs(Vec::new());
        producer.hourly_additional_supply_m[0] = 0.001;
        let mut day = day_with(producer);
        let error = day
            .run_wat5_subhourly_generation()
            .expect_err("must reject runon timing");
        assert!(error.to_string().contains("WAT5-E-001"));
    }

    #[test]
    fn any_positive_additional_supply_fails_even_below_closure_tolerance() {
        let mut producer = inputs(Vec::new());
        producer.hourly_additional_supply_m[0] = WAT5_CLOSURE_TOLERANCE_M / 2.0;
        let mut day = day_with(producer);
        let error = day
            .run_wat5_subhourly_generation()
            .expect_err("must not use closure tolerance as a source classifier");
        assert!(error.to_string().contains("WAT5-E-001"));
    }

    #[test]
    fn delayed_ponding_is_solved_at_five_minute_boundaries() {
        let mut producer = inputs(vec![DirectWb14HyetographInterval {
            start_s: 0.0,
            end_s: 600.0,
            intensity_m_s: 2.0e-5,
        }]);
        producer.effective_conductivity_m_s = 1.0e-5;
        producer.matric_potential_m = 0.01;
        let raw = compute_wb14_subhourly_profile(&producer).expect("WAT5 raw replay");
        assert!(raw.post_depression_excess_m[0].abs() <= f64::EPSILON);
        assert!(raw.post_depression_excess_m[1] > 0.0);
        assert!((raw.rainfall_m[0] - 0.006).abs() <= f64::EPSILON);
        assert!((raw.infiltration_m[0] - raw.rainfall_m[0]).abs() <= f64::EPSILON);
    }

    #[test]
    fn raw_event_closure_carries_depression_storage() {
        let mut producer = inputs(vec![DirectWb14HyetographInterval {
            start_s: 0.0,
            end_s: 600.0,
            intensity_m_s: 5.0e-5,
        }]);
        producer.depression_storage_capacity_m = 0.002;
        let raw = compute_wb14_subhourly_profile(&producer).expect("WAT5 raw replay");
        let rainfall_m: f64 = raw.rainfall_m.iter().sum();
        let accounted_m = raw.infiltration_m.iter().sum::<f64>()
            + raw.depression_storage_retention_m.iter().sum::<f64>()
            + raw.post_depression_excess_m.iter().sum::<f64>();
        assert!(raw.depression_storage_delta_m > 0.0);
        assert!(raw.depression_storage_retention_m[0] > 0.0);
        assert!(
            (raw.depression_storage_retention_m.iter().sum::<f64>()
                - raw.depression_storage_delta_m)
                .abs()
                <= f64::EPSILON
        );
        validate_wat5_closure(
            accounted_m,
            rainfall_m,
            "WAT5-E-004 wat5.raw_event_closure_residual_m",
        )
        .expect("raw event closure");
    }

    #[test]
    fn out_of_day_hyetograph_support_fails_before_splitting() {
        for (start_s, end_s) in [(-1.0, 300.0), (86_400.0, 86_401.0)] {
            let producer = inputs(vec![DirectWb14HyetographInterval {
                start_s,
                end_s,
                intensity_m_s: 1.0e-5,
            }]);
            let Err(error) = compute_wb14_subhourly_profile(&producer) else {
                panic!("out-of-day WAT5 source must fail");
            };
            assert!(error.to_string().contains("WAT5-E-003"));
        }
    }

    #[test]
    fn positive_rain_below_closure_tolerance_is_not_omitted() {
        let mut producer = inputs(vec![DirectWb14HyetographInterval {
            start_s: 300.0,
            end_s: 600.0,
            intensity_m_s: 1.0e-15,
        }]);
        producer.effective_conductivity_m_s = 1.0e-5;
        let mut day = day_with(producer);
        day.run_wat5_subhourly_generation()
            .expect("tiny positive rain WAT5");
        let event = day.wat5_subhourly_generation.expect("tiny positive event");
        assert_eq!(event.first_active_subinterval, Some(1));
        assert_eq!(event.last_active_subinterval, Some(1));
        assert_eq!(event.intervals[0].subinterval_index, 1);
        assert!(event.intervals[0].rainfall_depth_m > 0.0);
    }

    #[test]
    fn positive_authority_without_raw_support_fails_closed() {
        let mut day = day_with(inputs(Vec::new()));
        day.wb14_hourly_excess_m[0] = 0.001;
        day.runoff_downstream_operands.q_runoff_m = 0.001;
        let error = day
            .run_wat5_subhourly_generation()
            .expect_err("must reject missing support");
        assert!(error.to_string().contains("WAT5-E-002"));
    }

    #[test]
    fn wat5_bounded_reconciliation_places_on_latest_positive_source_piece() {
        let (producer, segments, destination) = wat5_reconciliation_case();
        let raw = compute_wb14_subhourly_profile_with_exact_segments(
            &producer,
            &segments,
            Some(&destination),
        )
        .expect("exact WAT5 replay");
        let mut accepted = [0.0; DC01_HOUR_BIN_COUNT];
        accepted[2] = TRACED_WAT5_CLOSING_RESIDUAL_M;
        let reconciliation = reconcile_wat5_zero_raw_generation_hour_v1(
            2,
            &accepted,
            TRACED_WAT5_CLOSING_RESIDUAL_M,
            &raw,
        )
        .expect("bounded partition-ledger reconciliation");

        assert_eq!(
            reconciliation.selected_piece_start_s.to_bits(),
            10_500.0_f64.to_bits()
        );
        assert_eq!(
            reconciliation.selected_piece_end_s.to_bits(),
            10_800.0_f64.to_bits()
        );
        assert_eq!(reconciliation.selected_bin_index, 35);
        assert_eq!(
            reconciliation.closing_ledger_m[11].to_bits(),
            TRACED_WAT5_CLOSING_RESIDUAL_M.to_bits()
        );
        assert!(
            reconciliation.closing_ledger_m[..11]
                .iter()
                .all(|value| value.to_bits() == 0.0_f64.to_bits())
        );
    }

    #[test]
    fn wat5_bounded_reconciliation_preserves_raw_supply_infiltration_closure() {
        let (producer, segments, destination) = wat5_reconciliation_case();
        let raw = compute_wb14_subhourly_profile_with_exact_segments(
            &producer,
            &segments,
            Some(&destination),
        )
        .expect("exact WAT5 replay");
        let rainfall_before = raw.rainfall_m;
        let additional_before = raw.additional_supply_m;
        let infiltration_before = raw.infiltration_m;
        let depression_before = raw.depression_storage_retention_m;
        let generation_before = raw.post_depression_excess_m;
        let pieces_before = raw.canonical_source_pieces.clone();
        let mut accepted = [0.0; DC01_HOUR_BIN_COUNT];
        accepted[2] = TRACED_WAT5_CLOSING_RESIDUAL_M;

        reconcile_wat5_zero_raw_generation_hour_v1(
            2,
            &accepted,
            TRACED_WAT5_CLOSING_RESIDUAL_M,
            &raw,
        )
        .expect("bounded partition-ledger reconciliation");

        assert_f64_slices_bit_equal(&raw.rainfall_m, &rainfall_before);
        assert_f64_slices_bit_equal(&raw.additional_supply_m, &additional_before);
        assert_f64_slices_bit_equal(&raw.infiltration_m, &infiltration_before);
        assert_f64_slices_bit_equal(&raw.depression_storage_retention_m, &depression_before);
        assert_f64_slices_bit_equal(&raw.post_depression_excess_m, &generation_before);
        assert_eq!(raw.canonical_source_pieces, pieces_before);
        let source_m: f64 =
            raw.rainfall_m.iter().sum::<f64>() + raw.additional_supply_m.iter().sum::<f64>();
        let raw_accounted_m = raw.infiltration_m.iter().sum::<f64>()
            + raw.depression_storage_retention_m.iter().sum::<f64>()
            + raw.post_depression_excess_m.iter().sum::<f64>();
        assert!((source_m - raw_accounted_m).abs() <= 1.0e-12);
    }

    #[test]
    fn wat5_bounded_reconciliation_preserves_authoritative_positive_hour() {
        let (producer, segments, destination) = wat5_reconciliation_case();
        let mut day = day_with(producer);
        day.wb14_hourly_excess_m = [0.0; DC01_HOUR_BIN_COUNT];
        day.wb14_hourly_excess_m[2] = TRACED_WAT5_CLOSING_RESIDUAL_M;
        day.runoff_partition.partition_runoff_m = TRACED_WAT5_CLOSING_RESIDUAL_M;
        day.runoff_downstream_operands.q_runoff_m = TRACED_WAT5_CLOSING_RESIDUAL_M;

        day.run_wat5_subhourly_generation_with_exact_segments(&segments, &destination)
            .expect("source-supported bounded WAT5 generation");

        assert_eq!(
            day.wb14_hourly_excess_m[2].to_bits(),
            TRACED_WAT5_CLOSING_RESIDUAL_M.to_bits()
        );
        let event = day.wat5_subhourly_generation.expect("WAT5 event");
        let hour_generation_m: f64 = event
            .intervals
            .iter()
            .filter(|interval| interval.hour_index == 2)
            .map(|interval| interval.closed_wb14_generation_depth_m)
            .sum();
        assert_eq!(
            hour_generation_m.to_bits(),
            TRACED_WAT5_CLOSING_RESIDUAL_M.to_bits()
        );
        assert_eq!(
            event
                .intervals
                .iter()
                .find(|interval| interval.subinterval_index == 35)
                .expect("latest positive source bin")
                .closed_wb14_generation_depth_m
                .to_bits(),
            TRACED_WAT5_CLOSING_RESIDUAL_M.to_bits()
        );
    }

    #[test]
    fn wat5_bounded_reconciliation_is_source_order_independent() {
        let (producer, segments, destination) = wat5_reconciliation_case();
        let raw_forward = compute_wb14_subhourly_profile_with_exact_segments(
            &producer,
            &segments,
            Some(&destination),
        )
        .expect("forward exact WAT5 replay");
        let mut reversed = segments;
        reversed.reverse();
        let raw_reversed = compute_wb14_subhourly_profile_with_exact_segments(
            &producer,
            &reversed,
            Some(&destination),
        )
        .expect("reversed exact WAT5 replay");
        let mut accepted = [0.0; DC01_HOUR_BIN_COUNT];
        accepted[2] = TRACED_WAT5_CLOSING_RESIDUAL_M;
        let forward = reconcile_wat5_zero_raw_generation_hour_v1(
            2,
            &accepted,
            TRACED_WAT5_CLOSING_RESIDUAL_M,
            &raw_forward,
        )
        .expect("forward reconciliation");
        let reversed = reconcile_wat5_zero_raw_generation_hour_v1(
            2,
            &accepted,
            TRACED_WAT5_CLOSING_RESIDUAL_M,
            &raw_reversed,
        )
        .expect("reversed reconciliation");
        assert_eq!(forward, reversed);
    }

    #[test]
    fn wat5_bounded_reconciliation_rejects_first_uniform_or_duplicate_placement() {
        let (producer, segments, destination) = wat5_reconciliation_case();
        let raw = compute_wb14_subhourly_profile_with_exact_segments(
            &producer,
            &segments,
            Some(&destination),
        )
        .expect("exact WAT5 replay");
        let mut accepted = [0.0; DC01_HOUR_BIN_COUNT];
        accepted[2] = TRACED_WAT5_CLOSING_RESIDUAL_M;
        let canonical = reconcile_wat5_zero_raw_generation_hour_v1(
            2,
            &accepted,
            TRACED_WAT5_CLOSING_RESIDUAL_M,
            &raw,
        )
        .expect("canonical reconciliation");

        let mut first = canonical.clone();
        first.closing_ledger_m = [0.0; WAT5_INTERVALS_PER_HOUR];
        first.closing_ledger_m[0] = TRACED_WAT5_CLOSING_RESIDUAL_M;
        assert!(
            first
                .validate_against(&accepted, TRACED_WAT5_CLOSING_RESIDUAL_M, &raw)
                .expect_err("first-bin placement must fail")
                .to_string()
                .contains("WAT5-E-002")
        );

        let mut uniform = canonical.clone();
        uniform.closing_ledger_m = [TRACED_WAT5_CLOSING_RESIDUAL_M / 12.0; WAT5_INTERVALS_PER_HOUR];
        assert!(
            uniform
                .validate_against(&accepted, TRACED_WAT5_CLOSING_RESIDUAL_M, &raw)
                .expect_err("uniform placement must fail")
                .to_string()
                .contains("WAT5-E-002")
        );

        let mut duplicate = canonical;
        duplicate.closing_ledger_m[10] = TRACED_WAT5_CLOSING_RESIDUAL_M;
        assert!(
            duplicate
                .validate_against(&accepted, TRACED_WAT5_CLOSING_RESIDUAL_M, &raw)
                .expect_err("duplicate placement must fail")
                .to_string()
                .contains("WAT5-E-002")
        );
    }

    #[test]
    fn wat5_bounded_reconciliation_rejects_zero_foreign_or_missing_source_support() {
        let (producer, segments, destination) = wat5_reconciliation_case();
        let raw = compute_wb14_subhourly_profile_with_exact_segments(
            &producer,
            &segments,
            Some(&destination),
        )
        .expect("exact WAT5 replay");
        let mut accepted = [0.0; DC01_HOUR_BIN_COUNT];
        accepted[2] = TRACED_WAT5_CLOSING_RESIDUAL_M;

        let mut zero =
            compute_wb14_subhourly_profile(&inputs(Vec::new())).expect("zero-source WAT5 replay");
        zero.canonical_source_pieces = raw.canonical_source_pieces.clone();
        assert!(
            reconcile_wat5_zero_raw_generation_hour_v1(
                2,
                &accepted,
                TRACED_WAT5_CLOSING_RESIDUAL_M,
                &zero,
            )
            .expect_err("zero source must fail")
            .to_string()
            .contains("WAT5-E-002")
        );

        let mut foreign = raw;
        for piece in &mut foreign.canonical_source_pieces {
            piece.start_s += 3_600.0;
            piece.end_s += 3_600.0;
        }
        assert!(
            reconcile_wat5_zero_raw_generation_hour_v1(
                2,
                &accepted,
                TRACED_WAT5_CLOSING_RESIDUAL_M,
                &foreign,
            )
            .expect_err("foreign-hour source support must fail")
            .to_string()
            .contains("WAT5-E-002")
        );

        foreign.canonical_source_pieces.clear();
        assert!(
            reconcile_wat5_zero_raw_generation_hour_v1(
                2,
                &accepted,
                TRACED_WAT5_CLOSING_RESIDUAL_M,
                &foreign,
            )
            .expect_err("missing source support must fail")
            .to_string()
            .contains("WAT5-E-002")
        );
    }

    #[test]
    fn wat5_bounded_reconciliation_accepts_exact_tolerance_boundary() {
        let (producer, segments, destination) = wat5_reconciliation_case();
        let raw = compute_wb14_subhourly_profile_with_exact_segments(
            &producer,
            &segments,
            Some(&destination),
        )
        .expect("exact WAT5 replay");
        let exact_boundary_m = 1.0e-12;
        let mut accepted = [0.0; DC01_HOUR_BIN_COUNT];
        accepted[2] = exact_boundary_m;
        let reconciliation =
            reconcile_wat5_zero_raw_generation_hour_v1(2, &accepted, exact_boundary_m, &raw)
                .expect("exact TOL-WAT5-002 boundary must pass");
        assert_eq!(
            reconciliation.closing_residual_m.to_bits(),
            exact_boundary_m.to_bits()
        );
    }

    #[test]
    fn wat5_bounded_reconciliation_rejects_first_value_above_tolerance() {
        let (producer, segments, destination) = wat5_reconciliation_case();
        let raw = compute_wb14_subhourly_profile_with_exact_segments(
            &producer,
            &segments,
            Some(&destination),
        )
        .expect("exact WAT5 replay");
        let first_above_m = f64::from_bits(1.0e-12_f64.to_bits() + 1);
        let mut accepted = [0.0; DC01_HOUR_BIN_COUNT];
        accepted[2] = first_above_m;
        let error = reconcile_wat5_zero_raw_generation_hour_v1(2, &accepted, first_above_m, &raw)
            .expect_err("first value above TOL-WAT5-002 must fail");
        assert!(error.to_string().contains("WAT5-E-002"));
    }

    #[test]
    fn saturation_only_hour_is_a_labeled_zero_order_hold() {
        let mut day = day_with(inputs(Vec::new()));
        let mut hourly_saturation_carry_m = [0.0; 24];
        hourly_saturation_carry_m[3] = 0.012;
        day.subsurface_compute_shadow_projection = Some(DirectSubsurfaceComputeShadowProjection {
            lane_index: 0,
            day_index: 0,
            soil_water_before_m: 0.0,
            soil_water_after_m: 0.0,
            lateral_flow_m: 0.0,
            tile_drainage_m: 0.0,
            subsurface_loss_m: 0.0,
            lateral_target_m: 0.0,
            drainage_target_m: 0.0,
            lateral_capacity_m: 0.0,
            hourly_lateral_carry_m: [0.0; 24],
            hourly_saturation_carry_m,
            layer_state_after: Vec::new(),
            lateral_layer_withdrawal_m: Vec::new(),
        });
        day.runoff_downstream_operands.q_runoff_m = 0.012;
        day.run_wat5_subhourly_generation()
            .expect("saturation WAT5");
        let event = day.wat5_subhourly_generation.expect("saturation event");
        assert_eq!(event.first_active_subinterval, Some(36));
        assert_eq!(event.last_active_subinterval, Some(47));
        assert_eq!(event.intervals.len(), 12);
        assert_eq!(event.intervals[0].subinterval_index, 36);
        assert_eq!(event.intervals[11].subinterval_index, 47);
        assert!(event.intervals.iter().all(|interval| {
            (interval.saturation_return_depth_m - 0.001).abs() <= f64::EPSILON
                && interval
                    .source_completeness_code
                    .contains("hourly_zero_order_hold")
        }));
    }

    #[test]
    fn rain_and_saturation_compose_to_the_authoritative_hour() {
        let mut day = day_with(inputs(vec![DirectWb14HyetographInterval {
            start_s: 0.0,
            end_s: 600.0,
            intensity_m_s: 5.0e-5,
        }]));
        let mut hourly_saturation_carry_m = [0.0; 24];
        hourly_saturation_carry_m[0] = 0.012;
        day.subsurface_compute_shadow_projection = Some(DirectSubsurfaceComputeShadowProjection {
            lane_index: 0,
            day_index: 0,
            soil_water_before_m: 0.0,
            soil_water_after_m: 0.0,
            lateral_flow_m: 0.0,
            tile_drainage_m: 0.0,
            subsurface_loss_m: 0.0,
            lateral_target_m: 0.0,
            drainage_target_m: 0.0,
            lateral_capacity_m: 0.0,
            hourly_lateral_carry_m: [0.0; 24],
            hourly_saturation_carry_m,
            layer_state_after: Vec::new(),
            lateral_layer_withdrawal_m: Vec::new(),
        });
        day.run_wat5_subhourly_generation()
            .expect("rain plus saturation WAT5");
        let event = day.wat5_subhourly_generation.expect("composed event");
        let hour_total_m: f64 = event
            .intervals
            .iter()
            .filter(|interval| interval.hour_index == 0)
            .map(|interval| interval.closing_surface_generation_depth_m)
            .sum();
        let authority_m = day.wb14_hourly_excess_m[0] + 0.012;
        assert!((hour_total_m - authority_m).abs() <= WAT5_CLOSURE_TOLERANCE_M);
        assert!(event.intervals.iter().all(|interval| {
            interval.hour_index != 0
                || interval.hourly_closure_residual_m.abs() <= WAT5_CLOSURE_TOLERANCE_M
        }));
    }

    #[test]
    fn nonfinite_and_negative_source_values_fail_typed_validation() {
        for (start_s, end_s, intensity_m_s) in [
            (0.0, 300.0, -1.0e-5),
            (f64::NAN, 300.0, 1.0e-5),
            (300.0, f64::INFINITY, 1.0e-5),
        ] {
            let producer = inputs(vec![DirectWb14HyetographInterval {
                start_s,
                end_s,
                intensity_m_s,
            }]);
            assert!(compute_wb14_subhourly_profile(&producer).is_err());
        }
    }

    #[test]
    fn constant_intensity_is_stable_across_required_input_resolutions() {
        let reference = compute_wb14_infiltration_depression_with_profile(&inputs(vec![
            DirectWb14HyetographInterval {
                start_s: 0.0,
                end_s: 3_600.0,
                intensity_m_s: 5.0e-5,
            },
        ]))
        .expect("reference WB14");
        for duration_s in [1_800.0, 600.0, 300.0, 150.0, 60.0] {
            let mut intervals = Vec::new();
            let mut start_s = 0.0;
            while start_s < 3_600.0 {
                intervals.push(DirectWb14HyetographInterval {
                    start_s,
                    end_s: start_s + duration_s,
                    intensity_m_s: 5.0e-5,
                });
                start_s += duration_s;
            }
            let observed = compute_wb14_infiltration_depression_with_profile(&inputs(intervals))
                .expect("split WB14");
            assert!(
                (observed.state.cumulative_infiltration_m
                    - reference.state.cumulative_infiltration_m)
                    .abs()
                    <= 1.0e-12
            );
            assert!((observed.hourly_excess_m[0] - reference.hourly_excess_m[0]).abs() <= 1.0e-12);
        }
    }

    #[test]
    fn wat5_e004_rejects_hour_and_day_closure_mismatches() {
        for field in [
            "WAT5-E-004 wat5.raw_event_closure_residual_m",
            "WAT5-E-004 wat5.hourly_closure_residual_m",
            "WAT5-E-004 wat5.daily_closure_residual_m",
        ] {
            let error =
                validate_wat5_closure(0.002, 0.001, field).expect_err("closure mismatch must fail");
            assert!(error.to_string().contains("WAT5-E-004"));
        }
    }
}
