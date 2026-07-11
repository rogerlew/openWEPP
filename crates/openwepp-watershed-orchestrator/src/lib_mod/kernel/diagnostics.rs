impl Ws10ChannelImpoundmentKernel {
    fn require_non_negative_computed(
        node_class: Ws10NodeClass,
        symbol: impl Into<BoundarySymbol>,
        value: f64,
    ) -> Result<f64, Ws10GuardError> {
        let symbol = symbol.into();
        if !value.is_finite() {
            return Err(Self::non_finite(node_class, symbol, value));
        }
        if value < 0.0 {
            return Err(Self::domain_violation(node_class, symbol, value));
        }
        Ok(value)
    }

    fn require_finite_computed(
        node_class: Ws10NodeClass,
        symbol: impl Into<BoundarySymbol>,
        value: f64,
    ) -> Result<f64, Ws10GuardError> {
        let symbol = symbol.into();
        if !value.is_finite() {
            return Err(Self::non_finite(node_class, symbol, value));
        }
        Ok(value)
    }

    fn ws11_muskingum_geometry_from_depth(
        node_class: Ws10NodeClass,
        ishape: u32,
        channel_width: f64,
        channel_shape: f64,
        depth: f64,
    ) -> Result<(f64, f64, f64, f64), Ws10GuardError> {
        if !depth.is_finite() || depth <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws11_muskingum_depth"),
                depth,
            ));
        }
        if !channel_width.is_finite() || channel_width <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws11_muskingum_channel_width"),
                channel_width,
            ));
        }
        if !channel_shape.is_finite()
            || (ishape != 2 && channel_shape <= WS10_ZERO_THRESHOLD)
        {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws11_muskingum_channel_shape"),
                channel_shape,
            ));
        }

        match ishape {
            1 => {
                let top_width = 2.0 * depth * channel_shape;
                let area = channel_shape * depth * depth;
                let wetted_perimeter = 2.0 * depth * (1.0 + (channel_shape * channel_shape)).sqrt();
                Ok((top_width, area, wetted_perimeter, channel_shape))
            }
            2 => {
                let top_width = channel_width;
                let area = channel_width * depth;
                let wetted_perimeter = channel_width + (2.0 * depth);
                Ok((top_width, area, wetted_perimeter, channel_width))
            }
            3 => {
                let chnz0 = channel_width * channel_shape / 8.0;
                if !chnz0.is_finite() || chnz0 <= WS10_ZERO_THRESHOLD {
                    return Err(Self::domain_violation(
                        node_class,
                        BoundarySymbol::from("ws11_muskingum_chnz0"),
                        chnz0,
                    ));
                }
                let top_width = 4.0 * (chnz0 * depth).sqrt();
                let area = (8.0 / 3.0) * depth * (depth * chnz0).sqrt();
                let wetted_perimeter = (2.0 * (depth * (chnz0 + depth)).sqrt())
                    + (2.0
                        * chnz0
                        * (((1.0 + (depth / chnz0)).sqrt()) + (depth / chnz0).sqrt()).ln());
                Ok((top_width, area, wetted_perimeter, chnz0))
            }
            4.. => {
                let top_width = channel_width + (2.0 * channel_shape * depth);
                let area = (channel_width + (channel_shape * depth)) * depth;
                let wetted_perimeter = channel_width
                    + (2.0 * depth * (1.0 + (channel_shape * channel_shape)).sqrt());
                Ok((top_width, area, wetted_perimeter, channel_shape))
            }
            _ => Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws11_muskingum_ishape"),
                f64::from(ishape),
            )),
        }
    }

    fn ws11_manning_discharge_for_depth(
        node_class: Ws10NodeClass,
        ishape: u32,
        channel_width: f64,
        channel_shape: f64,
        roughness: f64,
        slope: f64,
        depth: f64,
    ) -> Result<f64, Ws10GuardError> {
        if !roughness.is_finite() || roughness <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws11_manning_roughness"),
                roughness,
            ));
        }
        if !slope.is_finite() || slope <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws11_manning_slope"),
                slope,
            ));
        }

        let (_, area, wetted_perimeter, _) = Self::ws11_muskingum_geometry_from_depth(
            node_class,
            ishape,
            channel_width,
            channel_shape,
            depth,
        )?;
        if !area.is_finite() || area <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws11_manning_area"),
                area,
            ));
        }
        if !wetted_perimeter.is_finite() || wetted_perimeter <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws11_manning_wetted_perimeter"),
                wetted_perimeter,
            ));
        }

        let hydraulic_radius = area / wetted_perimeter;
        let discharge = (1.0 / roughness) * area * hydraulic_radius.powf(2.0 / 3.0) * slope.sqrt();
        if !discharge.is_finite() || discharge < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws11_manning_discharge"),
                discharge,
            ));
        }
        Ok(discharge)
    }

    fn ws11_solve_depth_for_discharge(
        node_class: Ws10NodeClass,
        ishape: u32,
        channel_width: f64,
        channel_shape: f64,
        roughness: f64,
        slope: f64,
        discharge: f64,
    ) -> Result<f64, Ws10GuardError> {
        if !discharge.is_finite() || discharge <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws11_reference_discharge"),
                discharge,
            ));
        }

        let mut lower = WS10_ZERO_THRESHOLD;
        let mut upper = 1.0_f64;
        let mut upper_discharge = Self::ws11_manning_discharge_for_depth(
            node_class,
            ishape,
            channel_width,
            channel_shape,
            roughness,
            slope,
            upper,
        )?;
        let mut guard_iter = 0_u32;
        while upper_discharge < discharge && guard_iter < 64 {
            upper *= 2.0;
            upper_discharge = Self::ws11_manning_discharge_for_depth(
                node_class,
                ishape,
                channel_width,
                channel_shape,
                roughness,
                slope,
                upper,
            )?;
            guard_iter += 1;
        }

        if upper_discharge < discharge {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws11_depth_solver_bracket"),
                upper_discharge,
            ));
        }

        for _ in 0..80 {
            let mid = 0.5 * (lower + upper);
            let mid_discharge = Self::ws11_manning_discharge_for_depth(
                node_class,
                ishape,
                channel_width,
                channel_shape,
                roughness,
                slope,
                mid,
            )?;
            if mid_discharge < discharge {
                lower = mid;
            } else {
                upper = mid;
            }
        }

        if !upper.is_finite() || upper <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws11_depth_solver_output"),
                upper,
            ));
        }
        Ok(upper)
    }

    #[allow(clippy::too_many_arguments)]
    fn compute_kinematic_wave_state(
        node_class: Ws10NodeClass,
        roughness: f64,
        conductivity: f64,
        nchnum: f64,
        routing_gain: f64,
        incoming_peak: f64,
        available_peak: f64,
        baseflow_peak: f64,
        dtchr: f64,
        event_duration: f64,
    ) -> Result<Ws11WaveRoutingState, Ws10GuardError> {
        let qin = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("qin"),
            available_peak,
        )?;
        let qin_previous = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("q1_previous"),
            incoming_peak,
        )?;
        let qlat = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("qlat"),
            baseflow_peak / event_duration,
        )?;

        let wave_storage = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("wave_storage"),
            1.0 + (roughness * dtchr) + (conductivity * nchnum),
        )?;
        if wave_storage <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("wave_storage"),
                wave_storage,
            ));
        }

        let c0 = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("c0"),
            1.0 / wave_storage,
        )?;
        let c1 = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("c1"),
            routing_gain / (1.0 + routing_gain),
        )?;
        let c2 = 0.0;
        let c3 =
            Self::require_non_negative_computed(node_class, BoundarySymbol::from("c3"), 1.0 - c1)?;
        let c4 = Self::require_non_negative_computed(node_class, BoundarySymbol::from("c4"), qlat)?;
        let q1 = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("q1"),
            ((c1 * qin) + (c3 * qin_previous) + c4) * c0,
        )?;

        Ok(Ws11WaveRoutingState {
            q1,
            qin,
            qlat,
            c0,
            c1,
            c2,
            c3,
            c4,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn compute_muskingum_cunge_state(
        node_class: Ws10NodeClass,
        roughness: f64,
        control_slope: f64,
        conductivity: f64,
        nchnum: f64,
        available_peak: f64,
        baseflow_peak: f64,
        dtchr: f64,
        event_duration: f64,
        prior_qin: Option<f64>,
        prior_q1: Option<f64>,
    ) -> Result<Ws11WaveRoutingState, Ws10GuardError> {
        let qin = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("qin"),
            available_peak,
        )?;
        let qin_previous = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("qin_previous"),
            prior_qin.unwrap_or(qin),
        )?;
        let q1_previous = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("q1_previous"),
            prior_q1.unwrap_or(qin + (baseflow_peak / event_duration)),
        )?;
        let qlat = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("qlat"),
            baseflow_peak / event_duration,
        )?;

        let mc_translation = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("mc_translation"),
            1.0 + (conductivity * dtchr),
        )?;
        if mc_translation <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("mc_translation"),
                mc_translation,
            ));
        }

        let mc_storage = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("mc_storage"),
            1.0 + (roughness * dtchr) + (control_slope * nchnum),
        )?;
        if mc_storage <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("mc_storage"),
                mc_storage,
            ));
        }

        let denominator = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("mc_denominator"),
            mc_translation + mc_storage,
        )?;
        if denominator <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("mc_denominator"),
                denominator,
            ));
        }

        let c0 = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("c0"),
            1.0 / denominator,
        )?;
        let c1 = Self::require_finite_computed(
            node_class,
            BoundarySymbol::from("c1"),
            mc_translation * c0,
        )?;
        let c2 = Self::require_finite_computed(
            node_class,
            BoundarySymbol::from("c2"),
            0.5 * mc_storage * c0,
        )?;
        let c3 =
            Self::require_finite_computed(node_class, BoundarySymbol::from("c3"), 1.0 - c1 - c2)?;
        let c4 = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("c4"),
            2.0 * qlat * dtchr * c0,
        )?;
        let q1 = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("q1"),
            (c1 * qin) + (c2 * qin_previous) + (c3 * q1_previous) + c4,
        )?;

        Ok(Ws11WaveRoutingState {
            q1,
            qin,
            qlat,
            c0,
            c1,
            c2,
            c3,
            c4,
        })
    }

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    fn compute_variable_muskingum_cunge_state(
        node_class: Ws10NodeClass,
        roughness: f64,
        control_slope: f64,
        channel_width: f64,
        channel_shape: f64,
        ishape: u32,
        channel_length: f64,
        available_peak: f64,
        baseflow_peak: f64,
        dtchr: f64,
        event_duration: f64,
        prior_qin: Option<f64>,
        prior_q1: Option<f64>,
    ) -> Result<Ws11WaveRoutingState, Ws10GuardError> {
        let qin = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("qin"),
            available_peak,
        )?;
        let qin_previous = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("qin_previous"),
            prior_qin.unwrap_or(qin),
        )?;
        let q1_previous = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("q1_previous"),
            prior_q1.unwrap_or(qin + (baseflow_peak / event_duration)),
        )?;
        let qlat = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("qlat"),
            baseflow_peak / event_duration,
        )?;
        let channel_length = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("channel_length"),
            channel_length,
        )?;
        if channel_length <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("channel_length"),
                channel_length,
            ));
        }

        let mut qref = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("qref_dynamic"),
            (qin + qin_previous + q1_previous) / 3.0,
        )?;
        if qref < WS11_DYNAMIC_MC_QREF_EPS_CMS {
            qref = WS11_DYNAMIC_MC_QREF_EPS_CMS;
        }

        let depth = Self::ws11_solve_depth_for_discharge(
            node_class,
            ishape,
            channel_width,
            channel_shape,
            roughness,
            control_slope,
            qref,
        )?;
        let (bt, _, ap, chnz0) = Self::ws11_muskingum_geometry_from_depth(
            node_class,
            ishape,
            channel_width,
            channel_shape,
            depth,
        )?;

        let ckref = match ishape {
            1 => Self::require_non_negative_computed(
                node_class,
                BoundarySymbol::from("ckref"),
                4.0 * qref / (3.0 * chnz0 * depth * depth),
            )?,
            2 => Self::require_non_negative_computed(
                node_class,
                BoundarySymbol::from("ckref"),
                (qref / (channel_width * depth))
                    * (1.0 + (2.0 * channel_width / (3.0 * (channel_width + (2.0 * depth))))),
            )?,
            3 => {
                let dqdy = (2.5 / depth - (4.0 / (3.0 * ap) * (1.0 + (bt / depth)).sqrt())) * qref;
                Self::require_non_negative_computed(
                    node_class,
                    BoundarySymbol::from("ckref"),
                    dqdy / bt,
                )?
            }
            _ => {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("ws11_muskingum_ishape"),
                    f64::from(ishape),
                ));
            }
        };
        if ckref <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ckref"),
                ckref,
            ));
        }

        let tk = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("tk"),
            channel_length / ckref,
        )?;
        let dencx = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("dencx"),
            bt * ckref * control_slope * channel_length,
        )?;
        if dencx <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("dencx"),
                dencx,
            ));
        }

        let mut cx = Self::require_finite_computed(
            node_class,
            BoundarySymbol::from("cx"),
            0.5 * (1.0 - (qref / dencx)),
        )?;
        if cx < -10.0 {
            cx = -10.0;
        }

        let denominator = Self::require_finite_computed(
            node_class,
            BoundarySymbol::from("mc_denominator"),
            (2.0 * tk * (1.0 - cx)) + dtchr,
        )?;
        if denominator.abs() <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("mc_denominator"),
                denominator,
            ));
        }

        let c0 = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("c0"),
            1.0 / denominator,
        )?;
        let c1 = Self::require_finite_computed(
            node_class,
            BoundarySymbol::from("c1"),
            (dtchr - (2.0 * tk * cx)) * c0,
        )?;
        let c2 = Self::require_finite_computed(
            node_class,
            BoundarySymbol::from("c2"),
            (dtchr + (2.0 * tk * cx)) * c0,
        )?;
        let c3 =
            Self::require_finite_computed(node_class, BoundarySymbol::from("c3"), 1.0 - c1 - c2)?;
        let c4 = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("c4"),
            2.0 * qlat * dtchr * c0,
        )?;
        let q1 = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("q1"),
            (c1 * qin) + (c2 * qin_previous) + (c3 * q1_previous) + c4,
        )?;

        Ok(Ws11WaveRoutingState {
            q1,
            qin,
            qlat,
            c0,
            c1,
            c2,
            c3,
            c4,
        })
    }
}
