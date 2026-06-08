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

    fn require_state_scalar(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
        symbol: WatershedProductionStateSymbol,
    ) -> Result<f64, Ws10GuardError> {
        let key = BoundarySymbol::from(symbol);
        let Some(value) = request.state_surface.get(&key) else {
            return Err(Self::missing_required(node_class, key));
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Self::non_finite(node_class, key, scalar));
        }
        Ok(scalar)
    }

    fn require_impoundment_coefficient_scalar(
        request: &WatershedKernelRequest<'_>,
        node_id: u32,
        suffix: &'static str,
    ) -> Result<f64, Ws10GuardError> {
        let node_class = Ws10NodeClass::Impoundment;
        let key = BoundarySymbol::from(format!("ws10_impoundment_{node_id}_{suffix}"));
        let Some(value) = request.state_surface.get(&key) else {
            return Err(Self::missing_required(node_class, key));
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Self::non_finite(node_class, key, scalar));
        }
        Ok(scalar)
    }

    fn require_impoundment_function_coefficient_scalar(
        request: &WatershedKernelRequest<'_>,
        node_id: u32,
        family_index: usize,
        suffix: &'static str,
    ) -> Result<f64, Ws10GuardError> {
        let node_class = Ws10NodeClass::Impoundment;
        let key = BoundarySymbol::from(format!(
            "ws10_impoundment_{node_id}_f{family_index:02}_{suffix}"
        ));
        let Some(value) = request.state_surface.get(&key) else {
            return Err(Self::missing_required(node_class, key));
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Self::non_finite(node_class, key, scalar));
        }
        Ok(scalar)
    }

    fn require_channel_control_scalar(
        request: &WatershedKernelRequest<'_>,
        node_id: u32,
        suffix: &'static str,
    ) -> Result<f64, Ws10GuardError> {
        let node_class = Ws10NodeClass::Channel;
        let key = BoundarySymbol::from(format!("ws10_channel_{node_id}_{suffix}"));
        let Some(value) = request.state_surface.get(&key) else {
            return Err(Self::missing_required(node_class, key));
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Self::non_finite(node_class, key, scalar));
        }
        Ok(scalar)
    }

    fn require_channel_state_symbol_scalar(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
        symbol: BoundarySymbol,
    ) -> Result<f64, Ws10GuardError> {
        let Some(value) = request.state_surface.get(&symbol) else {
            return Err(Self::missing_required(node_class, symbol));
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Self::non_finite(node_class, symbol, scalar));
        }
        Ok(scalar)
    }

    fn require_channel_control_range(
        node_class: Ws10NodeClass,
        symbol: BoundarySymbol,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Result<(), Ws10GuardError> {
        if let Some(minimum_value) = minimum
            && value < minimum_value
        {
            return Err(Self::domain_violation(node_class, symbol, value));
        }
        if let Some(maximum_value) = maximum
            && value > maximum_value
        {
            return Err(Self::domain_violation(node_class, symbol, value));
        }
        Ok(())
    }

    #[allow(clippy::similar_names)]
    fn read_ws15_channel_sediment_controls(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
    ) -> Result<Ws15ChannelSedimentControls, Ws10GuardError> {
        let node_id = request.node_id;

        let ishape_symbol = BoundarySymbol::from(format!("ws10_channel_{node_id}_ishape"));
        let ienslp_symbol = BoundarySymbol::from(format!("ws10_channel_{node_id}_ienslp"));
        let chnz_symbol = BoundarySymbol::from(format!("ws10_channel_{node_id}_chnz"));
        let chnnbr_symbol = BoundarySymbol::from(format!("ws10_channel_{node_id}_chnnbr"));
        let chntcr_symbol = BoundarySymbol::from(format!("ws10_channel_{node_id}_chntcr"));
        let chnedm_symbol = BoundarySymbol::from(format!("ws10_channel_{node_id}_chnedm"));
        let chneds_symbol = BoundarySymbol::from(format!("ws10_channel_{node_id}_chneds"));
        let ctlz_symbol = BoundarySymbol::from(format!("ws10_channel_{node_id}_ctlz"));
        let ctln_symbol = BoundarySymbol::from(format!("ws10_channel_{node_id}_ctln"));

        let ishape = Self::require_channel_control_scalar(request, node_id, "ishape")?;
        let ienslp = Self::require_channel_control_scalar(request, node_id, "ienslp")?;
        let chnz = Self::require_channel_control_scalar(request, node_id, "chnz")?;
        let chnnbr = Self::require_channel_control_scalar(request, node_id, "chnnbr")?;
        let chntcr = Self::require_channel_control_scalar(request, node_id, "chntcr")?;
        let chnedm = Self::require_channel_control_scalar(request, node_id, "chnedm")?;
        let chneds = Self::require_channel_control_scalar(request, node_id, "chneds")?;
        let ctlz = Self::require_channel_control_scalar(request, node_id, "ctlz")?;
        let ctln = Self::require_channel_control_scalar(request, node_id, "ctln")?;

        Self::require_channel_control_range(
            node_class,
            ishape_symbol,
            ishape,
            Some(1.0),
            Some(3.0),
        )?;
        Self::require_channel_control_range(
            node_class,
            ienslp_symbol,
            ienslp,
            Some(1.0),
            Some(2.0),
        )?;
        Self::require_channel_control_range(node_class, chnz_symbol, chnz, Some(0.0), None)?;
        Self::require_channel_control_range(
            node_class,
            chnnbr_symbol,
            chnnbr,
            Some(WS10_ZERO_THRESHOLD),
            None,
        )?;
        Self::require_channel_control_range(node_class, chntcr_symbol, chntcr, Some(0.0), None)?;
        Self::require_channel_control_range(node_class, chnedm_symbol, chnedm, Some(0.0), None)?;
        Self::require_channel_control_range(node_class, chneds_symbol, chneds, Some(0.0), None)?;
        Self::require_channel_control_range(
            node_class,
            ctlz_symbol,
            ctlz,
            Some(WS10_ZERO_THRESHOLD),
            None,
        )?;
        Self::require_channel_control_range(
            node_class,
            ctln_symbol,
            ctln,
            Some(WS10_ZERO_THRESHOLD),
            None,
        )?;

        let ishape_rounded = ishape.round();
        if (ishape - ishape_rounded).abs() > WS11_IPEAK_INTEGER_TOLERANCE {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from(format!("ws10_channel_{node_id}_ishape")),
                ishape,
            ));
        }

        let ienslp_rounded = ienslp.round();
        if (ienslp - ienslp_rounded).abs() > WS11_IPEAK_INTEGER_TOLERANCE {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from(format!("ws10_channel_{node_id}_ienslp")),
                ienslp,
            ));
        }

        Ok(Ws15ChannelSedimentControls {
            ishape,
            ctlz,
            chnz,
            chnnbr,
            chntcr,
            chnedm,
            chneds,
        })
    }

    #[allow(clippy::too_many_lines, clippy::similar_names)]
    fn require_ws17_channel_segment_scaffold(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
    ) -> Result<usize, Ws10GuardError> {
        let node_id = request.node_id;
        let nslpts_symbol = BoundarySymbol::from(format!("ws10_channel_{node_id}_nslpts"));
        let nslpts_raw =
            Self::require_channel_state_symbol_scalar(request, node_class, nslpts_symbol.clone())?;
        Self::require_channel_control_range(
            node_class,
            nslpts_symbol.clone(),
            nslpts_raw,
            Some(2.0),
            None,
        )?;

        let nslpts_rounded = nslpts_raw.round();
        if (nslpts_raw - nslpts_rounded).abs() > WS11_IPEAK_INTEGER_TOLERANCE {
            return Err(Self::domain_violation(
                node_class,
                nslpts_symbol,
                nslpts_raw,
            ));
        }
        if nslpts_rounded > f64::from(u32::MAX) {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from(format!("ws10_channel_{node_id}_nslpts")),
                nslpts_raw,
            ));
        }
        let nslpts_u32 = format!("{nslpts_rounded:.0}").parse::<u32>().map_err(|_| {
            Self::domain_violation(
                node_class,
                BoundarySymbol::from(format!("ws10_channel_{node_id}_nslpts")),
                nslpts_raw,
            )
        })?;
        let nslpts = usize::try_from(nslpts_u32).map_err(|_| {
            Self::domain_violation(
                node_class,
                BoundarySymbol::from(format!("ws10_channel_{node_id}_nslpts")),
                nslpts_raw,
            )
        })?;

        let mut previous_x: Option<f64> = None;
        for point_number in 1..=nslpts {
            let x_symbol =
                BoundarySymbol::from(format!("ws10_channel_{node_id}_x_{point_number:04}"));
            let slope_symbol =
                BoundarySymbol::from(format!("ws10_channel_{node_id}_slope_{point_number:04}"));
            let depth_a_symbol =
                BoundarySymbol::from(format!("ws10_channel_{node_id}_depa_{point_number:04}"));
            let depth_b_symbol =
                BoundarySymbol::from(format!("ws10_channel_{node_id}_depb_{point_number:04}"));
            let width_a_symbol =
                BoundarySymbol::from(format!("ws10_channel_{node_id}_wida_{point_number:04}"));
            let width_b_symbol =
                BoundarySymbol::from(format!("ws10_channel_{node_id}_widb_{point_number:04}"));

            let x =
                Self::require_channel_state_symbol_scalar(request, node_class, x_symbol.clone())?;
            let slope = Self::require_channel_state_symbol_scalar(
                request,
                node_class,
                slope_symbol.clone(),
            )?;
            let depth_a = Self::require_channel_state_symbol_scalar(
                request,
                node_class,
                depth_a_symbol.clone(),
            )?;
            let depth_b = Self::require_channel_state_symbol_scalar(
                request,
                node_class,
                depth_b_symbol.clone(),
            )?;
            let width_a = Self::require_channel_state_symbol_scalar(
                request,
                node_class,
                width_a_symbol.clone(),
            )?;
            let width_b = Self::require_channel_state_symbol_scalar(
                request,
                node_class,
                width_b_symbol.clone(),
            )?;

            Self::require_channel_control_range(node_class, x_symbol.clone(), x, Some(0.0), None)?;
            if let Some(previous) = previous_x
                && x + WS10_ZERO_THRESHOLD < previous
            {
                return Err(Self::domain_violation(node_class, x_symbol, x));
            }
            Self::require_channel_control_range(node_class, slope_symbol, slope, Some(0.0), None)?;
            Self::require_channel_control_range(
                node_class,
                depth_a_symbol,
                depth_a,
                Some(0.0),
                None,
            )?;
            Self::require_channel_control_range(
                node_class,
                depth_b_symbol,
                depth_b,
                Some(0.0),
                None,
            )?;
            Self::require_channel_control_range(
                node_class,
                width_a_symbol,
                width_a,
                Some(WS10_ZERO_THRESHOLD),
                None,
            )?;
            Self::require_channel_control_range(
                node_class,
                width_b_symbol,
                width_b,
                Some(WS10_ZERO_THRESHOLD),
                None,
            )?;

            previous_x = Some(x);
        }

        Ok(nslpts)
    }

    fn require_flux_scalar(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
        symbol: WatershedProductionFluxSymbol,
    ) -> Result<f64, Ws10GuardError> {
        let key = BoundarySymbol::from(symbol);
        let Some(value) = request.flux_surface.get(&key) else {
            return Err(Self::missing_required(node_class, key));
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Self::non_finite(node_class, key, scalar));
        }
        Ok(scalar)
    }

    fn require_state_range(
        node_class: Ws10NodeClass,
        symbol: WatershedProductionStateSymbol,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Result<(), Ws10GuardError> {
        if let Some(minimum_value) = minimum
            && value < minimum_value
        {
            return Err(Self::domain_violation(node_class, symbol, value));
        }
        if let Some(maximum_value) = maximum
            && value > maximum_value
        {
            return Err(Self::domain_violation(node_class, symbol, value));
        }
        Ok(())
    }

    fn require_flux_range(
        node_class: Ws10NodeClass,
        symbol: WatershedProductionFluxSymbol,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Result<(), Ws10GuardError> {
        if let Some(minimum_value) = minimum
            && value < minimum_value
        {
            return Err(Self::domain_violation(node_class, symbol, value));
        }
        if let Some(maximum_value) = maximum
            && value > maximum_value
        {
            return Err(Self::domain_violation(node_class, symbol, value));
        }
        Ok(())
    }

    fn parse_dependency(
        node_class: Ws10NodeClass,
        dependency: &str,
    ) -> Result<(Ws10NodeClass, u32), Ws10GuardError> {
        let Some((kind, id_text)) = dependency.split_once(':') else {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("dependency_node"),
                -1.0,
            ));
        };
        let Ok(id) = id_text.parse::<u32>() else {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("dependency_node"),
                -1.0,
            ));
        };

        match kind {
            "channel" => Ok((Ws10NodeClass::Channel, id)),
            "impoundment" => Ok((Ws10NodeClass::Impoundment, id)),
            _ => Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("dependency_node"),
                -1.0,
            )),
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

    fn read_hillslope_peak_payload(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
        hillslope_id: u32,
    ) -> Result<(f64, f64), Ws10GuardError> {
        let peak_symbol = WatershedProductionStateSymbol::HillslopeContributorPeak { hillslope_id };
        let dur_symbol =
            WatershedProductionStateSymbol::HillslopeContributorDuration { hillslope_id };

        let peak = Self::require_state_scalar(request, node_class, peak_symbol)?;
        let duration = Self::require_state_scalar(request, node_class, dur_symbol)?;

        Self::require_state_range(node_class, peak_symbol, peak, Some(0.0), None)?;
        Self::require_state_range(node_class, dur_symbol, duration, Some(0.0), None)?;

        Ok((peak, duration))
    }

    #[allow(clippy::too_many_lines)]
    fn read_hillslope_sediment_payload(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
        hillslope_id: u32,
    ) -> Result<Ws18HillslopeSedimentPayload, Ws10GuardError> {
        let total_detachment_symbol =
            WatershedProductionStateSymbol::HillslopeContributorTotalDetachmentKg { hillslope_id };
        let total_deposition_symbol =
            WatershedProductionStateSymbol::HillslopeContributorTotalDepositionKg { hillslope_id };
        let class_count_symbol =
            WatershedProductionStateSymbol::HillslopeContributorParticleClassCount { hillslope_id };

        let total_detachment =
            Self::require_state_scalar(request, node_class, total_detachment_symbol)?;
        let total_deposition =
            Self::require_state_scalar(request, node_class, total_deposition_symbol)?;
        let class_count_value =
            Self::require_state_scalar(request, node_class, class_count_symbol)?;

        Self::require_state_range(
            node_class,
            total_detachment_symbol,
            total_detachment,
            Some(0.0),
            None,
        )?;
        Self::require_state_range(
            node_class,
            total_deposition_symbol,
            total_deposition,
            Some(0.0),
            None,
        )?;
        Self::require_state_range(
            node_class,
            class_count_symbol,
            class_count_value,
            Some(1.0),
            None,
        )?;

        let rounded_class_count = class_count_value.round();
        if (class_count_value - rounded_class_count).abs() > WS11_IPEAK_INTEGER_TOLERANCE {
            return Err(Self::domain_violation(
                node_class,
                class_count_symbol,
                class_count_value,
            ));
        }
        if rounded_class_count < 1.0 {
            return Err(Self::domain_violation(
                node_class,
                class_count_symbol,
                class_count_value,
            ));
        }
        let class_count = format!("{rounded_class_count:.0}")
            .parse::<usize>()
            .map_err(|_| {
                Self::domain_violation(node_class, class_count_symbol, class_count_value)
            })?;
        if class_count == 0 {
            return Err(Self::domain_violation(
                node_class,
                class_count_symbol,
                class_count_value,
            ));
        }

        let mut fractions = Vec::with_capacity(class_count);
        let mut particle_diameters_m = Vec::with_capacity(class_count);
        let mut fraction_sum = 0.0_f64;

        for class_index in 1..=class_count {
            let concentration_symbol =
                WatershedProductionStateSymbol::HillslopeContributorSedimentConcentrationKgM3 {
                    hillslope_id,
                    class_index,
                };
            let particle_diameter_symbol =
                WatershedProductionStateSymbol::HillslopeContributorParticleDiameterMeters {
                    hillslope_id,
                    class_index,
                };
            let fraction_symbol =
                WatershedProductionStateSymbol::HillslopeContributorParticleFlowFraction {
                    hillslope_id,
                    class_index,
                };

            let concentration =
                Self::require_state_scalar(request, node_class, concentration_symbol)?;
            let particle_diameter =
                Self::require_state_scalar(request, node_class, particle_diameter_symbol)?;
            let fraction = Self::require_state_scalar(request, node_class, fraction_symbol)?;

            Self::require_state_range(
                node_class,
                concentration_symbol,
                concentration,
                Some(0.0),
                None,
            )?;
            Self::require_state_range(
                node_class,
                particle_diameter_symbol,
                particle_diameter,
                Some(WS10_ZERO_THRESHOLD),
                None,
            )?;
            Self::require_state_range(node_class, fraction_symbol, fraction, Some(0.0), Some(1.0))?;
            fractions.push(fraction);
            particle_diameters_m.push(particle_diameter);
            fraction_sum += fraction;
        }

        if fraction_sum <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                class_count_symbol,
                class_count_value,
            ));
        }

        Ok(Ws18HillslopeSedimentPayload {
            mass_kg: (total_detachment - total_deposition).max(0.0),
            fractions,
            particle_diameters_m,
        })
    }

    #[allow(clippy::too_many_lines)]
    fn read_channel_sediment_payload(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
        channel_id: u32,
        event_duration: f64,
    ) -> Result<Ws18HillslopeSedimentPayload, Ws10GuardError> {
        let qsed_symbol = Self::channel_wave_state_symbol(channel_id, "qsed");
        let qsed =
            Self::require_channel_state_symbol_scalar(request, node_class, qsed_symbol.clone())?;
        Self::require_channel_control_range(node_class, qsed_symbol, qsed, Some(0.0), None)?;

        if qsed <= WS10_ZERO_THRESHOLD {
            return Ok(Ws18HillslopeSedimentPayload {
                mass_kg: 0.0,
                fractions: Vec::new(),
                particle_diameters_m: Vec::new(),
            });
        }

        let mass_kg = qsed * event_duration;
        if !mass_kg.is_finite() || mass_kg < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from(format!("ws10_channel_{channel_id}_incoming_mass_kg")),
                mass_kg,
            ));
        }

        let class_count_symbol =
            Self::channel_wave_state_symbol(channel_id, "particle_class_count");
        let class_count_value = Self::require_channel_state_symbol_scalar(
            request,
            node_class,
            class_count_symbol.clone(),
        )?;
        Self::require_channel_control_range(
            node_class,
            class_count_symbol.clone(),
            class_count_value,
            Some(1.0),
            None,
        )?;

        let rounded_class_count = class_count_value.round();
        if (class_count_value - rounded_class_count).abs() > WS11_IPEAK_INTEGER_TOLERANCE {
            return Err(Self::domain_violation(
                node_class,
                class_count_symbol,
                class_count_value,
            ));
        }
        if rounded_class_count < 1.0 {
            return Err(Self::domain_violation(
                node_class,
                class_count_symbol,
                class_count_value,
            ));
        }
        let class_count = format!("{rounded_class_count:.0}")
            .parse::<usize>()
            .map_err(|_| {
                Self::domain_violation(node_class, class_count_symbol.clone(), class_count_value)
            })?;
        if class_count == 0 {
            return Err(Self::domain_violation(
                node_class,
                class_count_symbol,
                class_count_value,
            ));
        }

        let mut fractions = Vec::with_capacity(class_count);
        let mut particle_diameters_m = Vec::with_capacity(class_count);
        let mut fraction_sum = 0.0_f64;
        for class_index in 1..=class_count {
            let fraction_symbol = Self::channel_wave_state_symbol(
                channel_id,
                &format!("particle_flow_fraction_{class_index:04}"),
            );
            let particle_diameter_symbol = Self::channel_wave_state_symbol(
                channel_id,
                &format!("particle_diameter_m_{class_index:04}"),
            );

            let fraction = Self::require_channel_state_symbol_scalar(
                request,
                node_class,
                fraction_symbol.clone(),
            )?;
            let particle_diameter = Self::require_channel_state_symbol_scalar(
                request,
                node_class,
                particle_diameter_symbol.clone(),
            )?;

            Self::require_channel_control_range(
                node_class,
                fraction_symbol,
                fraction,
                Some(0.0),
                Some(1.0),
            )?;
            Self::require_channel_control_range(
                node_class,
                particle_diameter_symbol,
                particle_diameter,
                Some(WS10_ZERO_THRESHOLD),
                None,
            )?;

            fractions.push(fraction);
            particle_diameters_m.push(particle_diameter);
            fraction_sum += fraction;
        }

        if fraction_sum <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                Self::channel_wave_state_symbol(channel_id, "particle_flow_fraction_sum"),
                fraction_sum,
            ));
        }

        Ok(Ws18HillslopeSedimentPayload {
            mass_kg,
            fractions,
            particle_diameters_m,
        })
    }

    fn read_dependency_peak_payload(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
        dependency_class: Ws10NodeClass,
        dependency_id: u32,
    ) -> Result<(f64, f64), Ws10GuardError> {
        let (peak_symbol, duration_symbol) = match dependency_class {
            Ws10NodeClass::Channel => (
                WatershedProductionStateSymbol::ChannelNode {
                    node_id: dependency_id,
                    field: WatershedChannelStateField::Qpo,
                },
                WatershedProductionStateSymbol::ChannelNode {
                    node_id: dependency_id,
                    field: WatershedChannelStateField::Durrof,
                },
            ),
            Ws10NodeClass::Impoundment => (
                WatershedProductionStateSymbol::ImpoundmentNode {
                    node_id: dependency_id,
                    field: WatershedImpoundmentStateField::Qo,
                },
                WatershedProductionStateSymbol::ImpoundmentNode {
                    node_id: dependency_id,
                    field: WatershedImpoundmentStateField::Durout,
                },
            ),
        };

        let peak = Self::require_state_scalar(request, node_class, peak_symbol)?;
        let duration = Self::require_state_scalar(request, node_class, duration_symbol)?;

        Self::require_state_range(node_class, peak_symbol, peak, Some(0.0), None)?;
        Self::require_state_range(node_class, duration_symbol, duration, Some(0.0), None)?;

        Ok((peak, duration))
    }

    fn assemble_incoming_peak_partition(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
    ) -> Result<Ws20IncomingPeakPartition, Ws10GuardError> {
        let mut hillslope_peak = 0.0_f64;
        let mut dependency_peak = 0.0_f64;
        let mut hillslope_volume_m3 = 0.0_f64;
        let mut dependency_volume_m3 = 0.0_f64;
        let mut hillslope_duration_s = 0.0_f64;
        let mut dependency_duration_s = 0.0_f64;
        for &hillslope_id in request.contributor_hillslopes {
            let (peak, duration) =
                Self::read_hillslope_peak_payload(request, node_class, hillslope_id)?;
            let _ = Self::read_hillslope_sediment_payload(request, node_class, hillslope_id)?;
            let volume = peak * duration;
            if !volume.is_finite() {
                return Err(Self::non_finite(
                    node_class,
                    BoundarySymbol::from(format!("hs{hillslope_id}_runon_volume")),
                    volume,
                ));
            }
            if volume < 0.0 {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from(format!("hs{hillslope_id}_runon_volume")),
                    volume,
                ));
            }
            hillslope_peak += peak;
            hillslope_volume_m3 += volume;
            hillslope_duration_s = hillslope_duration_s.max(duration);
        }

        for dependency in &request.dependency_nodes {
            let (dependency_class, dependency_id) = Self::parse_dependency(node_class, dependency)?;
            let (peak, duration) = Self::read_dependency_peak_payload(
                request,
                node_class,
                dependency_class,
                dependency_id,
            )?;
            let volume = peak * duration;
            if !volume.is_finite() {
                return Err(Self::non_finite(
                    node_class,
                    BoundarySymbol::from(format!("dependency_{dependency}_runon_volume")),
                    volume,
                ));
            }
            if volume < 0.0 {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from(format!("dependency_{dependency}_runon_volume")),
                    volume,
                ));
            }
            dependency_peak += peak;
            dependency_volume_m3 += volume;
            dependency_duration_s = dependency_duration_s.max(duration);
        }

        let incoming_peak = hillslope_peak + dependency_peak;

        if !incoming_peak.is_finite() {
            return Err(Self::non_finite(
                node_class,
                BoundarySymbol::from("incoming_peak"),
                incoming_peak,
            ));
        }
        if incoming_peak < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("incoming_peak"),
                incoming_peak,
            ));
        }
        let incoming_duration = hillslope_duration_s.max(dependency_duration_s);
        if !incoming_duration.is_finite() {
            return Err(Self::non_finite(
                node_class,
                BoundarySymbol::from("incoming_duration"),
                incoming_duration,
            ));
        }
        if incoming_duration < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("incoming_duration"),
                incoming_duration,
            ));
        }

        Ok(Ws20IncomingPeakPartition {
            hillslope_peak_cms: hillslope_peak,
            dependency_peak_cms: dependency_peak,
            hillslope_volume_m3,
            dependency_volume_m3,
            hillslope_duration_s,
            dependency_duration_s,
        })
    }

    fn assemble_incoming_peak_and_duration(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
    ) -> Result<(f64, f64), Ws10GuardError> {
        let partition = Self::assemble_incoming_peak_partition(request, node_class)?;
        let incoming_duration = partition
            .hillslope_duration_s
            .max(partition.dependency_duration_s);
        Ok((
            partition.hillslope_peak_cms + partition.dependency_peak_cms,
            incoming_duration,
        ))
    }


}
