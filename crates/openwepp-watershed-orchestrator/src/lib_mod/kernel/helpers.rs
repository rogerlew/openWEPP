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

    #[allow(clippy::too_many_lines)]
    fn impoundment_outflow_at_stage(
        node_class: Ws10NodeClass,
        stage: f64,
        coefficients: &Ws12ImpoundmentCoefficients,
    ) -> Result<f64, Ws10GuardError> {
        let mut q = [0.0_f64; 15];
        let htw = 0.0;

        // Drop spillway family (qo1..qo3)
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

        // Culvert #1 family (qo4..qo6)
        if stage > coefficients.ha[3] {
            if coefficients.b[3] <= WS10_ZERO_THRESHOLD {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("f04_b"),
                    coefficients.b[3],
                ));
            }
            let base = (stage - coefficients.ha[3]) / coefficients.b[3];
            if base > 0.0 {
                q[3] = coefficients.a[3] * base.powf(coefficients.c[3]);
            }
        }
        if stage > coefficients.ha[4] {
            if coefficients.b[4].abs() <= WS10_ZERO_THRESHOLD
                || coefficients.d[4].abs() <= WS10_ZERO_THRESHOLD
            {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("f05_bd"),
                    coefficients.b[4] + coefficients.d[4],
                ));
            }
            let base = (((stage - coefficients.ha[4]) / coefficients.b[4]) + coefficients.c[4])
                / coefficients.d[4];
            if base > 0.0 {
                q[4] = coefficients.a[4] * base.sqrt();
            }
        }
        if stage > coefficients.ha[5] {
            let adjusted_head = if htw > coefficients.a[5] {
                stage - (coefficients.ha[5] + htw - coefficients.a[5])
            } else {
                stage - coefficients.ha[5]
            };
            if adjusted_head > 0.0 {
                q[5] = coefficients.b[5] * adjusted_head.powf(coefficients.c[5]);
            }
        }

        // Culvert #2 family (qo7..qo9)
        if stage > coefficients.ha[6] {
            if coefficients.b[6] <= WS10_ZERO_THRESHOLD {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("f07_b"),
                    coefficients.b[6],
                ));
            }
            let base = (stage - coefficients.ha[6]) / coefficients.b[6];
            if base > 0.0 {
                q[6] = coefficients.a[6] * base.powf(coefficients.c[6]);
            }
        }
        if stage > coefficients.ha[7] {
            if coefficients.b[7].abs() <= WS10_ZERO_THRESHOLD
                || coefficients.d[7].abs() <= WS10_ZERO_THRESHOLD
            {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("f08_bd"),
                    coefficients.b[7] + coefficients.d[7],
                ));
            }
            let base = (((stage - coefficients.ha[7]) / coefficients.b[7]) + coefficients.c[7])
                / coefficients.d[7];
            if base > 0.0 {
                q[7] = coefficients.a[7] * base.sqrt();
            }
        }
        if stage > coefficients.ha[8] {
            let adjusted_head = if htw > coefficients.a[8] {
                stage - (coefficients.ha[8] + htw - coefficients.a[8])
            } else {
                stage - coefficients.ha[8]
            };
            if adjusted_head > 0.0 {
                q[8] = coefficients.b[8] * adjusted_head.powf(coefficients.c[8]);
            }
        }

        // Rockfill family (qo10)
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

        // Emergency spillway family (qo11)
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

        // Filter fence family (qo12)
        if stage > coefficients.ha[11] {
            q[11] = coefficients.a[11] * (stage - coefficients.ha[11]);
            if stage > coefficients.d[11] {
                let overtopping_depth = stage - coefficients.d[11];
                q[11] += (coefficients.b[11] + coefficients.c[11] * overtopping_depth)
                    * overtopping_depth.powf(1.5);
            }
        }

        // Perforated riser family (qo13..qo15)
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
