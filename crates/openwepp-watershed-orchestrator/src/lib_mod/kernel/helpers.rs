#[derive(Debug, Clone, Copy)]
struct Ws12CulvertFamilyParams {
    start_index: usize,
    first_b_symbol: &'static str,
    middle_bd_symbol: &'static str,
}

impl Ws10ChannelImpoundmentKernel {
    fn missing_required(
        node_class: Ws10NodeClass,
        symbol: impl Into<BoundarySymbol>,
    ) -> Ws10GuardError {
        let _ = symbol.into();
        Ws10GuardError {
            node_class,
            guard_class: Ws10GuardClass::MissingRequiredInput,
        }
    }

    fn non_finite(
        node_class: Ws10NodeClass,
        symbol: impl Into<BoundarySymbol>,
        value: f64,
    ) -> Ws10GuardError {
        let _ = symbol.into();
        let _ = value;
        Ws10GuardError {
            node_class,
            guard_class: Ws10GuardClass::NonFinite,
        }
    }

    fn domain_violation(
        node_class: Ws10NodeClass,
        symbol: impl Into<BoundarySymbol>,
        value: f64,
    ) -> Ws10GuardError {
        let _ = symbol.into();
        let _ = value;
        Ws10GuardError {
            node_class,
            guard_class: Ws10GuardClass::DomainViolation,
        }
    }

    fn impoundment_outflow_at_stage(
        node_class: Ws10NodeClass,
        stage: f64,
        coefficients: &Ws12ImpoundmentCoefficients,
    ) -> Result<f64, Ws10GuardError> {
        let mut q = [0.0_f64; 15];
        let htw = 0.0;

        Self::impoundment_drop_spillway_outflows(stage, coefficients, htw, &mut q);
        Self::impoundment_culvert_family_outflows(
            node_class,
            stage,
            coefficients,
            htw,
            Ws12CulvertFamilyParams {
                start_index: 3,
                first_b_symbol: "f04_b",
                middle_bd_symbol: "f05_bd",
            },
            &mut q,
        )?;
        Self::impoundment_culvert_family_outflows(
            node_class,
            stage,
            coefficients,
            htw,
            Ws12CulvertFamilyParams {
                start_index: 6,
                first_b_symbol: "f07_b",
                middle_bd_symbol: "f08_bd",
            },
            &mut q,
        )?;
        Self::impoundment_rockfill_outflow(node_class, stage, coefficients, &mut q)?;
        Self::impoundment_emergency_spillway_outflow(stage, coefficients, &mut q);
        Self::impoundment_filter_fence_outflow(stage, coefficients, &mut q);
        Self::impoundment_perforated_riser_outflows(node_class, stage, coefficients, &mut q)?;

        Self::impoundment_validate_total_outflow(node_class, &q)
    }

    fn impoundment_drop_spillway_outflows(
        stage: f64,
        coefficients: &Ws12ImpoundmentCoefficients,
        htw: f64,
        q: &mut [f64; 15],
    ) {
        if stage > coefficients.ha[0] {
            q[0] = coefficients.b[0] * (stage - coefficients.ha[0]).powf(coefficients.c[0]);
        }
        if stage > coefficients.ha[1] {
            q[1] = coefficients.b[1] * (stage - coefficients.ha[1]).powf(coefficients.c[1]);
        }
        if stage > coefficients.ha[2] {
            let adjusted_head = if htw > coefficients.a[2] {
                stage - (coefficients.ha[2] + htw - coefficients.a[2])
            } else {
                stage - coefficients.ha[2]
            };
            if adjusted_head > 0.0 {
                q[2] = coefficients.b[2] * adjusted_head.powf(coefficients.c[2]);
            }
        }
    }

    fn impoundment_culvert_family_outflows(
        node_class: Ws10NodeClass,
        stage: f64,
        coefficients: &Ws12ImpoundmentCoefficients,
        htw: f64,
        params: Ws12CulvertFamilyParams,
        q: &mut [f64; 15],
    ) -> Result<(), Ws10GuardError> {
        let first = params.start_index;
        let middle = first + 1;
        let last = first + 2;

        if stage > coefficients.ha[first] {
            if coefficients.b[first] <= WS10_ZERO_THRESHOLD {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from(params.first_b_symbol),
                    coefficients.b[first],
                ));
            }
            let base = (stage - coefficients.ha[first]) / coefficients.b[first];
            if base > 0.0 {
                q[first] = coefficients.a[first] * base.powf(coefficients.c[first]);
            }
        }
        if stage > coefficients.ha[middle] {
            if coefficients.b[middle].abs() <= WS10_ZERO_THRESHOLD
                || coefficients.d[middle].abs() <= WS10_ZERO_THRESHOLD
            {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from(params.middle_bd_symbol),
                    coefficients.b[middle] + coefficients.d[middle],
                ));
            }
            let base = (((stage - coefficients.ha[middle]) / coefficients.b[middle])
                + coefficients.c[middle])
                / coefficients.d[middle];
            if base > 0.0 {
                q[middle] = coefficients.a[middle] * base.sqrt();
            }
        }
        if stage > coefficients.ha[last] {
            let adjusted_head = if htw > coefficients.a[last] {
                stage - (coefficients.ha[last] + htw - coefficients.a[last])
            } else {
                stage - coefficients.ha[last]
            };
            if adjusted_head > 0.0 {
                q[last] = coefficients.b[last] * adjusted_head.powf(coefficients.c[last]);
            }
        }
        Ok(())
    }

    fn impoundment_rockfill_outflow(
        node_class: Ws10NodeClass,
        stage: f64,
        coefficients: &Ws12ImpoundmentCoefficients,
        q: &mut [f64; 15],
    ) -> Result<(), Ws10GuardError> {
        if stage > coefficients.ha[9] {
            if coefficients.b[9] <= WS10_ZERO_THRESHOLD {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("f10_b"),
                    coefficients.b[9],
                ));
            }
            let base = (stage - coefficients.ha[9]) / coefficients.b[9];
            if base > 0.0 {
                q[9] += coefficients.a[9] * base.powf(coefficients.c[9]);
            }
        }
        if stage > coefficients.e[9] {
            q[9] += coefficients.d[9] * (stage - coefficients.e[9]).powf(1.5);
        }
        Ok(())
    }

    fn impoundment_emergency_spillway_outflow(
        stage: f64,
        coefficients: &Ws12ImpoundmentCoefficients,
        q: &mut [f64; 15],
    ) {
        if stage > coefficients.ha[10] {
            let depth = stage - coefficients.ha[10];
            let polynomial = coefficients.a[10]
                + coefficients.b[10] * depth
                + coefficients.c[10] * depth.powi(2)
                + coefficients.d[10] * depth.powi(3)
                + coefficients.e[10] * depth.powi(4);
            if polynomial.is_finite() && polynomial > 0.0 {
                q[10] = polynomial;
            }
        }
    }

    fn impoundment_filter_fence_outflow(
        stage: f64,
        coefficients: &Ws12ImpoundmentCoefficients,
        q: &mut [f64; 15],
    ) {
        if stage > coefficients.ha[11] {
            q[11] = coefficients.a[11] * (stage - coefficients.ha[11]);
            if stage > coefficients.d[11] {
                let overtopping_depth = stage - coefficients.d[11];
                q[11] += (coefficients.b[11] + coefficients.c[11] * overtopping_depth)
                    * overtopping_depth.powf(1.5);
            }
        }
    }

    fn impoundment_perforated_riser_outflows(
        node_class: Ws10NodeClass,
        stage: f64,
        coefficients: &Ws12ImpoundmentCoefficients,
        q: &mut [f64; 15],
    ) -> Result<(), Ws10GuardError> {
        if stage > coefficients.ha[12] {
            let depth = stage - coefficients.ha[12];
            if depth > 0.0 {
                let denominator = coefficients.b[12] + coefficients.c[12] / depth.powf(1.5);
                if denominator <= WS10_ZERO_THRESHOLD || !denominator.is_finite() {
                    return Err(Self::domain_violation(
                        node_class,
                        BoundarySymbol::from("f13_denominator"),
                        denominator,
                    ));
                }
                q[12] = coefficients.a[12] / denominator;
            }
        }
        if stage > coefficients.ha[13] {
            q[13] = coefficients.a[13] * (stage - coefficients.ha[13]).sqrt();
        }
        if stage > coefficients.ha[14] {
            q[14] = coefficients.b[14] * (stage - coefficients.ha[14]).powf(coefficients.c[14]);
        }
        Ok(())
    }

    fn impoundment_validate_total_outflow(
        node_class: Ws10NodeClass,
        q: &[f64; 15],
    ) -> Result<f64, Ws10GuardError> {
        let group_1 = q[0].min(q[1]).min(q[2]);
        let group_2 = q[3].min(q[4]).min(q[5]);
        let group_3 = q[6].min(q[7]).min(q[8]);
        let group_4 = q[12].min(q[13]).min(q[14]);
        let qo = group_1 + group_2 + group_3 + q[9] + q[10] + q[11] + group_4;

        if !qo.is_finite() {
            return Err(Self::non_finite(node_class, BoundarySymbol::from("qo"), qo));
        }
        if qo < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("qo"),
                qo,
            ));
        }
        Ok(qo)
    }

    fn impoundment_area_at_stage(
        node_class: Ws10NodeClass,
        stage: f64,
        coefficients: &Ws12ImpoundmentCoefficients,
    ) -> Result<f64, Ws10GuardError> {
        let area = coefficients.a0 + coefficients.a1 * stage.powf(coefficients.a2);
        if !area.is_finite() || area <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("area"),
                area,
            ));
        }
        Ok(area)
    }

    fn impoundment_continuity_rate(
        node_class: Ws10NodeClass,
        stage: f64,
        incoming_peak: f64,
        qinf: f64,
        coefficients: &Ws12ImpoundmentCoefficients,
    ) -> Result<f64, Ws10GuardError> {
        if !stage.is_finite() {
            return Err(Self::non_finite(
                node_class,
                BoundarySymbol::from("stage"),
                stage,
            ));
        }
        if stage < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("stage"),
                stage,
            ));
        }

        let area = Self::impoundment_area_at_stage(node_class, stage, coefficients)?;
        let qo = Self::impoundment_outflow_at_stage(node_class, stage, coefficients)?;
        let continuity_outflow = qo + qinf;
        if !continuity_outflow.is_finite() || continuity_outflow < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("continuity_outflow"),
                continuity_outflow,
            ));
        }

        let dhdt = (incoming_peak - continuity_outflow) / area;
        if !dhdt.is_finite() {
            return Err(Self::non_finite(
                node_class,
                BoundarySymbol::from("dhdt"),
                dhdt,
            ));
        }
        Ok(dhdt)
    }

    fn impoundment_rk4_step(
        node_class: Ws10NodeClass,
        stage: f64,
        dt: f64,
        incoming_peak: f64,
        qinf: f64,
        coefficients: &Ws12ImpoundmentCoefficients,
    ) -> Result<f64, Ws10GuardError> {
        if !dt.is_finite() || dt <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("dt"),
                dt,
            ));
        }

        let k1 = Self::impoundment_continuity_rate(
            node_class,
            stage,
            incoming_peak,
            qinf,
            coefficients,
        )?;
        let k2 = Self::impoundment_continuity_rate(
            node_class,
            stage + 0.5 * dt * k1,
            incoming_peak,
            qinf,
            coefficients,
        )?;
        let k3 = Self::impoundment_continuity_rate(
            node_class,
            stage + 0.5 * dt * k2,
            incoming_peak,
            qinf,
            coefficients,
        )?;
        let k4 = Self::impoundment_continuity_rate(
            node_class,
            stage + dt * k3,
            incoming_peak,
            qinf,
            coefficients,
        )?;

        let hnext = stage + (dt / 6.0) * (k1 + k4 + 2.0 * (k2 + k3));
        if !hnext.is_finite() {
            return Err(Self::non_finite(
                node_class,
                BoundarySymbol::from("hnext"),
                hnext,
            ));
        }
        Ok(hnext)
    }

    fn crosses_threshold(h_start: f64, h_end: f64, threshold: f64) -> bool {
        (h_start < threshold && h_end > threshold) || (h_start > threshold && h_end < threshold)
    }

    fn impoundment_crosses_regime_transition(
        h_start: f64,
        h_end: f64,
        coefficients: &Ws12ImpoundmentCoefficients,
    ) -> bool {
        coefficients
            .ha
            .iter()
            .copied()
            .any(|threshold| Self::crosses_threshold(h_start, h_end, threshold))
            || Self::crosses_threshold(h_start, h_end, coefficients.e[9])
            || Self::crosses_threshold(h_start, h_end, coefficients.d[11])
    }

    fn integrate_impoundment_stage_with_adaptive_retry(
        node_class: Ws10NodeClass,
        stage_h: f64,
        hfull: f64,
        deltat: f64,
        incoming_peak: f64,
        qinf: f64,
        coefficients: &Ws12ImpoundmentCoefficients,
    ) -> Result<(f64, f64), Ws10GuardError> {
        let mut dt = deltat;
        let mut retries = 0_usize;

        loop {
            if retries >= WS12_IMPOUNDMENT_RETRY_LIMIT {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("adaptive_retry"),
                    dt,
                ));
            }
            if !dt.is_finite() || dt <= WS10_ZERO_THRESHOLD {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("deltat"),
                    dt,
                ));
            }

            let half_dt = 0.5 * dt;
            if half_dt <= WS10_ZERO_THRESHOLD {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("deltat"),
                    dt,
                ));
            }

            let half_stage = Self::impoundment_rk4_step(
                node_class,
                stage_h,
                half_dt,
                incoming_peak,
                qinf,
                coefficients,
            )?;
            let two_half_stage = Self::impoundment_rk4_step(
                node_class,
                half_stage,
                half_dt,
                incoming_peak,
                qinf,
                coefficients,
            )?;
            let full_stage = Self::impoundment_rk4_step(
                node_class,
                stage_h,
                dt,
                incoming_peak,
                qinf,
                coefficients,
            )?;

            let stage_error = two_half_stage - full_stage;
            if !stage_error.is_finite() {
                return Err(Self::non_finite(
                    node_class,
                    BoundarySymbol::from("stage_error"),
                    stage_error,
                ));
            }
            let errmax = stage_error.abs() / WS12_IMPOUNDMENT_ERROR_SCALE;
            if !errmax.is_finite() {
                return Err(Self::non_finite(
                    node_class,
                    BoundarySymbol::from("errmax"),
                    errmax,
                ));
            }
            if errmax > 1.0 {
                dt = 0.9 * dt * errmax.powf(-0.25);
                retries += 1;
                continue;
            }

            let corrected_hnext = two_half_stage + (stage_error / 15.0);
            if !corrected_hnext.is_finite() {
                return Err(Self::non_finite(
                    node_class,
                    BoundarySymbol::from("hnext"),
                    corrected_hnext,
                ));
            }
            if !(0.0..=hfull).contains(&corrected_hnext) {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("hnext"),
                    corrected_hnext,
                ));
            }

            if Self::impoundment_crosses_regime_transition(stage_h, corrected_hnext, coefficients)
                && dt > WS10_ZERO_THRESHOLD * 2.0
            {
                dt *= 0.5;
                retries += 1;
                continue;
            }

            return Ok((corrected_hnext, dt));
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn route_impoundment_stage_over_duration(
        node_class: Ws10NodeClass,
        stage_h: f64,
        hfull: f64,
        deltat: f64,
        total_duration_hours: f64,
        incoming_peak: f64,
        qinf: f64,
        coefficients: &Ws12ImpoundmentCoefficients,
    ) -> Result<(f64, f64), Ws10GuardError> {
        if !total_duration_hours.is_finite() || total_duration_hours <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("total_duration_hours"),
                total_duration_hours,
            ));
        }

        let mut stage = stage_h;
        let mut remaining = total_duration_hours;
        let mut last_accepted_dt = deltat;
        let mut iterations = 0_usize;

        while remaining > WS10_ZERO_THRESHOLD {
            if iterations >= WS12_IMPOUNDMENT_RETRY_LIMIT {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("integration_iterations"),
                    remaining,
                ));
            }
            let step_trial_dt = deltat.min(remaining);
            let (step_hnext, accepted_dt) = Self::integrate_impoundment_stage_with_adaptive_retry(
                node_class,
                stage,
                hfull,
                step_trial_dt,
                incoming_peak,
                qinf,
                coefficients,
            )?;
            if !accepted_dt.is_finite() || accepted_dt <= WS10_ZERO_THRESHOLD {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("accepted_dt"),
                    accepted_dt,
                ));
            }
            if accepted_dt > remaining + WS10_ZERO_THRESHOLD {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("accepted_dt"),
                    accepted_dt,
                ));
            }

            stage = step_hnext;
            remaining -= accepted_dt;
            last_accepted_dt = accepted_dt;
            iterations += 1;
        }

        Ok((stage, last_accepted_dt))
    }
}

#[cfg(test)]
const WS12_TEST_EPS: f64 = 1.0e-9;

#[cfg(test)]
fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= WS12_TEST_EPS,
        "actual {actual} differs from expected {expected}"
    );
}

#[cfg(test)]
fn test_impoundment_coefficients() -> Ws12ImpoundmentCoefficients {
    Ws12ImpoundmentCoefficients {
        a: [0.0; 15],
        b: [0.0; 15],
        c: [1.0; 15],
        d: [0.0; 15],
        e: [0.0; 15],
        ha: [100.0; 15],
        a0: 10.0,
        a1: 0.0,
        a2: 1.0,
    }
}

#[cfg(test)]
fn full_outflow_coefficients() -> Ws12ImpoundmentCoefficients {
    let mut coefficients = test_impoundment_coefficients();
    coefficients.ha = [
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 1.0, 2.0, 3.0, 1.0, 6.0, 9.0,
    ];
    coefficients.a[3] = 5.0;
    coefficients.a[4] = 6.0;
    coefficients.a[6] = 7.0;
    coefficients.a[7] = 8.0;
    coefficients.a[9] = 2.0;
    coefficients.a[10] = 1.0;
    coefficients.a[11] = 2.0;
    coefficients.a[12] = 30.0;
    coefficients.a[13] = 4.0;
    coefficients.b[0] = 2.0;
    coefficients.b[1] = 3.0;
    coefficients.b[2] = 4.0;
    coefficients.b[3] = 1.0;
    coefficients.b[4] = 2.0;
    coefficients.b[5] = 4.0;
    coefficients.b[6] = 1.0;
    coefficients.b[7] = 2.0;
    coefficients.b[8] = 9.0;
    coefficients.b[9] = 3.0;
    coefficients.b[10] = 0.5;
    coefficients.b[11] = 0.5;
    coefficients.b[12] = 2.0;
    coefficients.b[14] = 5.0;
    coefficients.c[4] = 1.0;
    coefficients.c[7] = 1.0;
    coefficients.c[9] = 1.0;
    coefficients.c[10] = 0.1;
    coefficients.c[11] = 0.25;
    coefficients.c[12] = 27.0;
    coefficients.c[14] = 1.0;
    coefficients.d[4] = 1.0;
    coefficients.d[7] = 1.0;
    coefficients.d[9] = 0.5;
    coefficients.d[10] = 0.01;
    coefficients.d[11] = 6.0;
    coefficients.e[9] = 4.0;
    coefficients.e[10] = 0.001;
    coefficients
}

#[test]
fn impoundment_outflow_at_stage_combines_all_structure_families() {
    let coefficients = full_outflow_coefficients();
    let qo = Ws10ChannelImpoundmentKernel::impoundment_outflow_at_stage(
        Ws10NodeClass::Impoundment,
        10.0,
        &coefficients,
    )
    .expect("valid structure families should compute outflow");

    let group_1 = 18.0_f64.min(24.0).min(28.0);
    let group_2 = 30.0_f64.min(6.0 * 3.5_f64.sqrt()).min(16.0);
    let group_3 = 21.0_f64.min(8.0 * 2.0_f64.sqrt()).min(9.0);
    let rockfill = 6.0 + 0.5 * 6.0_f64.powf(1.5);
    let emergency =
        1.0 + 0.5 * 8.0 + 0.1 * 8.0_f64.powi(2) + 0.01 * 8.0_f64.powi(3) + 0.001 * 8.0_f64.powi(4);
    let filter_fence = 14.0 + 1.5 * 4.0_f64.powf(1.5);
    let group_4 = 10.0_f64.min(8.0).min(5.0);
    assert_close(
        qo,
        group_1 + group_2 + group_3 + rockfill + emergency + filter_fence + group_4,
    );
}

#[test]
fn impoundment_outflow_at_stage_rejects_bad_culvert_denominator() {
    let mut coefficients = test_impoundment_coefficients();
    coefficients.ha[3] = 1.0;
    coefficients.a[3] = 1.0;
    coefficients.b[3] = 0.0;

    let error = Ws10ChannelImpoundmentKernel::impoundment_outflow_at_stage(
        Ws10NodeClass::Impoundment,
        2.0,
        &coefficients,
    )
    .expect_err("zero culvert denominator should fail closed");

    assert_eq!(error.boundary_class(), BoundaryClass::DomainViolation);
}

#[test]
fn impoundment_outflow_at_stage_rejects_bad_riser_denominator() {
    let mut coefficients = test_impoundment_coefficients();
    coefficients.ha[12] = 1.0;
    coefficients.a[12] = 1.0;
    coefficients.b[12] = 0.0;
    coefficients.c[12] = 0.0;

    let error = Ws10ChannelImpoundmentKernel::impoundment_outflow_at_stage(
        Ws10NodeClass::Impoundment,
        2.0,
        &coefficients,
    )
    .expect_err("zero riser denominator should fail closed");

    assert_eq!(error.boundary_class(), BoundaryClass::DomainViolation);
}

#[test]
fn impoundment_outflow_at_stage_rejects_negative_total_outflow() {
    let mut coefficients = test_impoundment_coefficients();
    coefficients.ha[11] = 1.0;
    coefficients.a[11] = -2.0;
    coefficients.d[11] = 100.0;

    let error = Ws10ChannelImpoundmentKernel::impoundment_outflow_at_stage(
        Ws10NodeClass::Impoundment,
        2.0,
        &coefficients,
    )
    .expect_err("negative total outflow should fail closed");

    assert_eq!(error.boundary_class(), BoundaryClass::DomainViolation);
}

#[test]
fn impoundment_private_guard_constructors_map_boundary_classes() {
    let missing =
        Ws10ChannelImpoundmentKernel::missing_required(Ws10NodeClass::Impoundment, "required");
    let non_finite =
        Ws10ChannelImpoundmentKernel::non_finite(Ws10NodeClass::Impoundment, "nan", f64::NAN);

    assert_eq!(
        missing.boundary_class(),
        BoundaryClass::MissingRequiredInput
    );
    assert_eq!(non_finite.boundary_class(), BoundaryClass::NonFinite);
}

#[test]
fn impoundment_internal_outflow_helpers_cover_tailwater_and_guards() {
    let mut coefficients = full_outflow_coefficients();
    let mut q = [0.0; 15];
    coefficients.a[2] = 1.0;
    Ws10ChannelImpoundmentKernel::impoundment_drop_spillway_outflows(
        10.0,
        &coefficients,
        2.0,
        &mut q,
    );
    assert_close(q[2], 24.0);

    q = [0.0; 15];
    coefficients.a[5] = 1.0;
    Ws10ChannelImpoundmentKernel::impoundment_culvert_family_outflows(
        Ws10NodeClass::Impoundment,
        10.0,
        &coefficients,
        2.0,
        Ws12CulvertFamilyParams {
            start_index: 3,
            first_b_symbol: "f04_b",
            middle_bd_symbol: "f05_bd",
        },
        &mut q,
    )
    .expect("valid culvert family should compute with tailwater adjustment");
    assert_close(q[5], 12.0);

    coefficients.b[4] = 0.0;
    let error = Ws10ChannelImpoundmentKernel::impoundment_culvert_family_outflows(
        Ws10NodeClass::Impoundment,
        10.0,
        &coefficients,
        0.0,
        Ws12CulvertFamilyParams {
            start_index: 3,
            first_b_symbol: "f04_b",
            middle_bd_symbol: "f05_bd",
        },
        &mut q,
    )
    .expect_err("zero middle culvert denominator should fail closed");
    assert_eq!(error.boundary_class(), BoundaryClass::DomainViolation);

    coefficients.b[4] = 2.0;
    coefficients.b[9] = 0.0;
    let error = Ws10ChannelImpoundmentKernel::impoundment_rockfill_outflow(
        Ws10NodeClass::Impoundment,
        10.0,
        &coefficients,
        &mut q,
    )
    .expect_err("zero rockfill denominator should fail closed");
    assert_eq!(error.boundary_class(), BoundaryClass::DomainViolation);
}

#[test]
fn impoundment_geometry_and_continuity_guards_fail_closed() {
    let coefficients = test_impoundment_coefficients();
    let area_error = Ws10ChannelImpoundmentKernel::impoundment_area_at_stage(
        Ws10NodeClass::Impoundment,
        1.0,
        &coefficients,
    )
    .expect("baseline geometry should be valid");
    assert_close(area_error, 10.0);

    let mut bad_area = coefficients;
    bad_area.a0 = 0.0;
    let error = Ws10ChannelImpoundmentKernel::impoundment_area_at_stage(
        Ws10NodeClass::Impoundment,
        1.0,
        &bad_area,
    )
    .expect_err("non-positive area should fail closed");
    assert_eq!(error.boundary_class(), BoundaryClass::DomainViolation);

    let q = [f64::INFINITY; 15];
    let error = Ws10ChannelImpoundmentKernel::impoundment_validate_total_outflow(
        Ws10NodeClass::Impoundment,
        &q,
    )
    .expect_err("non-finite total outflow should fail closed");
    assert_eq!(error.boundary_class(), BoundaryClass::NonFinite);

    let error = Ws10ChannelImpoundmentKernel::impoundment_continuity_rate(
        Ws10NodeClass::Impoundment,
        f64::NAN,
        1.0,
        0.0,
        &coefficients,
    )
    .expect_err("non-finite stage should fail closed");
    assert_eq!(error.boundary_class(), BoundaryClass::NonFinite);

    let error = Ws10ChannelImpoundmentKernel::impoundment_continuity_rate(
        Ws10NodeClass::Impoundment,
        -1.0,
        1.0,
        0.0,
        &coefficients,
    )
    .expect_err("negative stage should fail closed");
    assert_eq!(error.boundary_class(), BoundaryClass::DomainViolation);

    let error = Ws10ChannelImpoundmentKernel::impoundment_continuity_rate(
        Ws10NodeClass::Impoundment,
        1.0,
        1.0,
        -1.0,
        &coefficients,
    )
    .expect_err("negative continuity outflow should fail closed");
    assert_eq!(error.boundary_class(), BoundaryClass::DomainViolation);

    let error = Ws10ChannelImpoundmentKernel::impoundment_continuity_rate(
        Ws10NodeClass::Impoundment,
        1.0,
        f64::INFINITY,
        0.0,
        &coefficients,
    )
    .expect_err("non-finite dhdt should fail closed");
    assert_eq!(error.boundary_class(), BoundaryClass::NonFinite);
}

#[test]
fn impoundment_rk4_step_rejects_invalid_timestep() {
    let coefficients = test_impoundment_coefficients();
    let error = Ws10ChannelImpoundmentKernel::impoundment_rk4_step(
        Ws10NodeClass::Impoundment,
        1.0,
        0.0,
        1.0,
        0.0,
        &coefficients,
    )
    .expect_err("zero rk4 timestep should fail closed");

    assert_eq!(error.boundary_class(), BoundaryClass::DomainViolation);
}

#[test]
fn integrate_impoundment_stage_accepts_stable_step() {
    let coefficients = test_impoundment_coefficients();
    let (hnext, accepted_dt) =
        Ws10ChannelImpoundmentKernel::integrate_impoundment_stage_with_adaptive_retry(
            Ws10NodeClass::Impoundment,
            1.0,
            2.0,
            1.0,
            1.0,
            0.0,
            &coefficients,
        )
        .expect("stable step should integrate");

    assert_close(hnext, 1.1);
    assert_close(accepted_dt, 1.0);
}

#[test]
fn integrate_impoundment_stage_retries_regime_transition() {
    let mut coefficients = test_impoundment_coefficients();
    coefficients.ha[0] = 1.05;
    let (_hnext, accepted_dt) =
        Ws10ChannelImpoundmentKernel::integrate_impoundment_stage_with_adaptive_retry(
            Ws10NodeClass::Impoundment,
            1.0,
            2.0,
            1.0,
            1.0,
            0.0,
            &coefficients,
        )
        .expect("transition crossing should retry with a smaller step");

    assert!(accepted_dt < 1.0);
    assert!(accepted_dt > 0.0);
}

#[test]
fn integrate_impoundment_stage_rejects_invalid_timestep() {
    let coefficients = test_impoundment_coefficients();
    let error = Ws10ChannelImpoundmentKernel::integrate_impoundment_stage_with_adaptive_retry(
        Ws10NodeClass::Impoundment,
        1.0,
        2.0,
        0.0,
        1.0,
        0.0,
        &coefficients,
    )
    .expect_err("zero timestep should fail closed");

    assert_eq!(error.boundary_class(), BoundaryClass::DomainViolation);
}

#[test]
fn integrate_impoundment_stage_rejects_half_timestep_underflow() {
    let coefficients = test_impoundment_coefficients();
    let error = Ws10ChannelImpoundmentKernel::integrate_impoundment_stage_with_adaptive_retry(
        Ws10NodeClass::Impoundment,
        1.0,
        2.0,
        1.5e-12,
        1.0,
        0.0,
        &coefficients,
    )
    .expect_err("half timestep below threshold should fail closed");

    assert_eq!(error.boundary_class(), BoundaryClass::DomainViolation);
}

#[test]
fn integrate_impoundment_stage_rejects_stage_outside_pool_capacity() {
    let coefficients = test_impoundment_coefficients();
    let error = Ws10ChannelImpoundmentKernel::integrate_impoundment_stage_with_adaptive_retry(
        Ws10NodeClass::Impoundment,
        1.0,
        1.05,
        1.0,
        1.0,
        0.0,
        &coefficients,
    )
    .expect_err("stage above full pool should fail closed");

    assert_eq!(error.boundary_class(), BoundaryClass::DomainViolation);
}

#[test]
fn integrate_impoundment_stage_retries_error_control() {
    let mut coefficients = test_impoundment_coefficients();
    coefficients.ha[11] = 0.0;
    coefficients.a[11] = 5.0;
    coefficients.d[11] = 100.0;

    let (hnext, accepted_dt) =
        Ws10ChannelImpoundmentKernel::integrate_impoundment_stage_with_adaptive_retry(
            Ws10NodeClass::Impoundment,
            1.0,
            2.0,
            1.0,
            0.0,
            0.0,
            &coefficients,
        )
        .expect("large truncation error should retry to a smaller valid step");

    assert!(accepted_dt < 1.0);
    assert!(accepted_dt > 0.0);
    assert!(hnext < 1.0);
    assert!(hnext >= 0.0);
}

#[test]
fn route_impoundment_stage_over_duration_advances_across_multiple_steps() {
    let coefficients = test_impoundment_coefficients();
    let (hnext, accepted_dt) = Ws10ChannelImpoundmentKernel::route_impoundment_stage_over_duration(
        Ws10NodeClass::Impoundment,
        1.0,
        2.0,
        0.25,
        1.0,
        1.0,
        0.0,
        &coefficients,
    )
    .expect("valid horizon should route across substeps");

    assert_close(hnext, 1.1);
    assert_close(accepted_dt, 0.25);
}

#[test]
fn route_impoundment_stage_over_duration_rejects_bad_duration() {
    let coefficients = test_impoundment_coefficients();
    let error = Ws10ChannelImpoundmentKernel::route_impoundment_stage_over_duration(
        Ws10NodeClass::Impoundment,
        1.0,
        2.0,
        1.0,
        0.0,
        1.0,
        0.0,
        &coefficients,
    )
    .expect_err("zero routing duration should fail closed");

    assert_eq!(error.boundary_class(), BoundaryClass::DomainViolation);
}

#[test]
fn route_impoundment_stage_over_duration_rejects_iteration_exhaustion() {
    let coefficients = test_impoundment_coefficients();
    let error = Ws10ChannelImpoundmentKernel::route_impoundment_stage_over_duration(
        Ws10NodeClass::Impoundment,
        1.0,
        100.0,
        1.0e-3,
        1.0,
        1.0,
        0.0,
        &coefficients,
    )
    .expect_err("iteration cap should fail closed when accepted steps cannot exhaust horizon");

    assert_eq!(error.boundary_class(), BoundaryClass::DomainViolation);
}
