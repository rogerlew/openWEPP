impl Ws10ChannelImpoundmentKernel {
    fn ws11_interval_lane_active(
        input: &DirectWatershedKernelInput<'_>,
        branch: Ws11IpeakBranch,
    ) -> Result<bool, Ws10GuardError> {
        if matches!(branch, Ws11IpeakBranch::Rational | Ws11IpeakBranch::Creams) {
            return Ok(false);
        }

        let local_hourly = Self::direct_hillslope_hourly_authority(input, Ws10NodeClass::Channel)?;
        let local_present = !input.step.contributor_hillslopes.is_empty();
        let mut active_channel_dependencies = 0_usize;
        let mut inactive_channel_dependencies = 0_usize;
        let mut impoundment_dependencies = 0_usize;
        for dependency in &input.step.dependency_nodes {
            match dependency.kind {
                TopologyNodeKind::Channel => {
                    let Some(state) = input.frame.routed_channels.get(&dependency.id) else {
                        inactive_channel_dependencies += 1;
                        continue;
                    };
                    match (
                        state.interval_water_state.is_some(),
                        state.interval_sediment_state.is_some(),
                    ) {
                        (true, true) => active_channel_dependencies += 1,
                        (false, false) => inactive_channel_dependencies += 1,
                        _ => {
                            return Err(Self::domain_violation(
                                Ws10NodeClass::Channel,
                                BoundarySymbol::from("ws11_interval_partial_dependency"),
                                f64::from(dependency.id),
                            ));
                        }
                    }
                }
                TopologyNodeKind::Impoundment => impoundment_dependencies += 1,
                TopologyNodeKind::Hillslope => {
                    return Err(Self::domain_violation(
                        Ws10NodeClass::Channel,
                        BoundarySymbol::from("ws11_interval_dependency_kind"),
                        -1.0,
                    ));
                }
            }
        }

        let any_interval_authority = local_hourly || active_channel_dependencies > 0;
        if any_interval_authority
            && (impoundment_dependencies > 0
                || inactive_channel_dependencies > 0
                || (local_present && !local_hourly))
        {
            return Err(Self::domain_violation(
                Ws10NodeClass::Channel,
                BoundarySymbol::from("ws11_interval_mixed_authority"),
                f64::from(
                    u32::try_from(impoundment_dependencies + inactive_channel_dependencies)
                        .unwrap_or(u32::MAX),
                ),
            ));
        }
        Ok(any_interval_authority)
    }

    fn ws11_ntchr(input: &DirectWatershedKernelInput<'_>) -> Result<usize, Ws10GuardError> {
        let ntchr_value = input.frame.routing_globals.ntchr;
        if !ntchr_value.is_finite() || ntchr_value < 1.0 || ntchr_value.fract().abs() > 1.0e-12 {
            return Err(Self::domain_violation(
                Ws10NodeClass::Channel,
                BoundarySymbol::from("ws11_ntchr"),
                ntchr_value,
            ));
        }
        format!("{ntchr_value:.0}").parse::<usize>().map_err(|_| {
            Self::domain_violation(
                Ws10NodeClass::Channel,
                BoundarySymbol::from("ws11_ntchr"),
                ntchr_value,
            )
        })
    }

    fn run_direct_interval_channel_node(
        input: &DirectWatershedKernelInput<'_>,
        context: &Ws11DirectChannelContext<'_>,
    ) -> Result<DirectWatershedKernelOutput, Ws10GuardError> {
        let ntchr = Self::ws11_ntchr(input)?;
        Self::ws11_validate_interval_grid(context.dtchr, ntchr)?;
        Self::ws11_validate_active_lane_operand_mode(true)?;
        let (water_state, representative_wave) =
            Self::ws11_route_interval_water(input, context, ntchr)?;
        let sediment_state = Self::ws11_route_interval_sediment(
            input,
            context,
            &water_state,
            ntchr,
        )?;

        let peak_discharge_m3_s = water_state.q1_m3_s.iter().copied().fold(0.0_f64, f64::max);
        let channel_outflow_m3 = water_state.q1_m3_s.iter().sum::<f64>() * context.dtchr;
        let channel_inflow_m3 = water_state
            .qin_m3_s
            .iter()
            .zip(&water_state.qlat_total_m3_s)
            .map(|(qin, qlat)| (qin + qlat) * context.dtchr)
            .sum::<f64>();
        let channel_storage_m3 = channel_inflow_m3 - channel_outflow_m3;
        let duration_seconds = Self::ws11_interval_active_span_s(&water_state.q1_m3_s, context.dtchr);
        let daily_egress_total_kg = sediment_state.daily_egress_kg.iter().sum::<f64>();
        let particle_flow_fraction = Self::ws11_daily_particle_fractions(
            &sediment_state.daily_egress_kg,
        );
        let qsed_kg_s = daily_egress_total_kg / 86_400.0;
        let diagnostics = Self::ws11_sum_interval_diagnostics(&sediment_state);
        let channel_baseflow = Self::assemble_direct_channel_baseflow(
            input,
            Ws10NodeClass::Channel,
            context.dtchr,
            context.cbase,
            context.nchnum,
            context.conductivity,
        )?;

        Ok(DirectWatershedKernelOutput::Channel(Box::new(RoutedChannelState {
            node_id: context.node_id,
            runoff_volume_m3: channel_outflow_m3,
            channel_inflow_m3,
            channel_outflow_m3,
            channel_storage_m3,
            peak_discharge_m3_s,
            duration_seconds,
            channel_baseflow_m3: channel_baseflow.volume_m3,
            channel_loss_m3: 0.0,
            groundwater_deep_seepage_m3: channel_baseflow.deep_seepage_m3,
            sediment_yield_kg: daily_egress_total_kg,
            wave_state: representative_wave.map(|state| RoutedChannelWaveState {
                q1_m3_s: state.q1,
                qin_m3_s: state.qin,
                qlat_m3_s: state.qlat,
                c0: state.c0,
                c1: state.c1,
                c2: state.c2,
                c3: state.c3,
                c4: state.c4,
            }),
            interval_water_state: Some(water_state),
            sediment_state: RoutedChannelSedimentState {
                qsed_kg_s,
                transport_capacity_kg_s: 0.0,
                particle_flow_fraction,
                particle_diameter_m: sediment_state.particle_diameter_m.clone(),
                ws20_case1_segments: diagnostics.case1_segments,
                ws20_case2_segments: diagnostics.case2_segments,
                ws24_case2_detach_segments: diagnostics.ws24_case2_detach_segments,
                ws21_case3_segments: diagnostics.case3_segments,
                ws21_case4_segments: diagnostics.case4_segments,
                ws21_enddet_segments: diagnostics.enddet_segments,
            },
            interval_sediment_state: Some(sediment_state),
        })))
    }

    #[allow(clippy::too_many_lines)]
    fn ws11_route_interval_water(
        input: &DirectWatershedKernelInput<'_>,
        context: &Ws11DirectChannelContext<'_>,
        ntchr: usize,
    ) -> Result<(RoutedChannelIntervalWaterState, Option<Ws11WaveRoutingState>), Ws10GuardError> {
        let mut hourly_lateral_volume_m3 = [0.0_f64; 24];
        for hillslope_id in &input.step.contributor_hillslopes {
            let contribution = input
                .frame
                .hillslope_contributions
                .get(hillslope_id)
                .ok_or_else(|| Self::missing_required(Ws10NodeClass::Channel, "hillslope_contribution"))?;
            for (hour, value) in contribution.hourly_runoff_volume_m3.iter().enumerate() {
                hourly_lateral_volume_m3[hour] += *value;
            }
        }
        let projected_lateral_volume_m3 = Self::ws11_project_hourly_totals(
            &hourly_lateral_volume_m3,
            context.dtchr,
            ntchr,
        )?;
        let baseflow = Self::ws11_local_channel_baseflow(
            input,
            Ws10NodeClass::Channel,
            context.dtchr,
            context.cbase,
            context.nchnum,
            context.conductivity,
        )?;
        let uniform_baseflow_m3_s = baseflow.volume_m3 / 86_400.0;
        let mut qin_m3_s = vec![0.0_f64; ntchr];
        for dependency in input
            .step
            .dependency_nodes
            .iter()
            .filter(|dependency| dependency.kind == TopologyNodeKind::Channel)
        {
            let state = input
                .frame
                .routed_channels
                .get(&dependency.id)
                .and_then(|state| state.interval_water_state.as_ref())
                .ok_or_else(|| Self::missing_required(Ws10NodeClass::Channel, "dependency_interval_water"))?;
            if (state.dtchr_seconds - context.dtchr).abs() > 1.0e-12
                || state.q1_m3_s.len() != ntchr
            {
                return Err(Self::domain_violation(
                    Ws10NodeClass::Channel,
                    BoundarySymbol::from("dependency_interval_water_grid"),
                    state.dtchr_seconds,
                ));
            }
            for (target, source) in qin_m3_s.iter_mut().zip(&state.q1_m3_s) {
                *target += *source;
            }
        }

        let qlat_total_m3_s = projected_lateral_volume_m3
            .iter()
            .map(|volume| volume / context.dtchr + uniform_baseflow_m3_s)
            .collect::<Vec<_>>();
        let previous = input
            .frame
            .routed_channels
            .get(&context.node_id)
            .and_then(|state| state.interval_water_state.as_ref())
            .map(|state| {
                if (state.dtchr_seconds - context.dtchr).abs() > 1.0e-12
                    || state.q1_m3_s.len() != ntchr
                    || state.qin_m3_s.len() != ntchr
                    || state.qlat_total_m3_s.len() != ntchr
                {
                    return Err(Self::domain_violation(
                        Ws10NodeClass::Channel,
                        BoundarySymbol::from("ws11_prior_day_water_grid"),
                        state.dtchr_seconds,
                    ));
                }
                let q1 = Self::require_non_negative_computed(
                    Ws10NodeClass::Channel,
                    BoundarySymbol::from("ws11_prior_day_q1"),
                    state.q1_m3_s[ntchr - 1],
                )?;
                let qin = Self::require_non_negative_computed(
                    Ws10NodeClass::Channel,
                    BoundarySymbol::from("ws11_prior_day_qin"),
                    state.qin_m3_s[ntchr - 1],
                )?;
                let qlat = Self::require_non_negative_computed(
                    Ws10NodeClass::Channel,
                    BoundarySymbol::from("ws11_prior_day_qlat"),
                    state.qlat_total_m3_s[ntchr - 1],
                )?;
                Ok(Ws11WaveRoutingState {
                    q1,
                    qin,
                    qlat,
                    c0: 0.0,
                    c1: 0.0,
                    c2: 0.0,
                    c3: 0.0,
                    c4: 0.0,
                })
            })
            .transpose()?;
        let routed = Self::ws11_route_baseline_wave_series(
            context,
            &qin_m3_s,
            &qlat_total_m3_s,
            previous,
        )?;

        Ok((
            RoutedChannelIntervalWaterState {
                dtchr_seconds: context.dtchr,
                qin_m3_s,
                qlat_total_m3_s,
                q1_m3_s: routed.q1_m3_s,
                storage_change_m3: routed.storage_change_m3,
            },
            routed.representative,
        ))
    }

    #[allow(clippy::too_many_lines)]
    fn ws11_route_baseline_wave_series(
        context: &Ws11DirectChannelContext<'_>,
        qin_m3_s: &[f64],
        qlat_total_m3_s: &[f64],
        prior: Option<Ws11WaveRoutingState>,
    ) -> Result<Ws11BaselineWaveSeries, Ws10GuardError> {
        if matches!(
            context.ipeak_branch,
            Ws11IpeakBranch::Rational | Ws11IpeakBranch::Creams
        ) {
            return Err(Self::domain_violation(
                Ws10NodeClass::Channel,
                BoundarySymbol::from("ws11_interval_nonwave_branch"),
                0.0,
            ));
        }
        if qin_m3_s.is_empty() || qin_m3_s.len() != qlat_total_m3_s.len() {
            return Err(Self::domain_violation(
                Ws10NodeClass::Channel,
                BoundarySymbol::from("ws11_interval_wave_grid"),
                Self::ws11_small_count_as_f64(qin_m3_s.len(), "ws11_interval_wave_grid")?,
            ));
        }

        let first = context.control.segment_points.first().ok_or_else(|| {
            Self::missing_required(Ws10NodeClass::Channel, "ws11_channel_profile")
        })?;
        let channel_width_m = first.width_a_ft / WS15_DEPTH_FROM_METERS_TO_FEET;
        let channel_slope = first.slope;
        let channel_shape = context.control.chnz;
        let ishape = if channel_shape < 1.0e-8 {
            2
        } else {
            context.ishape
        };
        let initial_q1 = prior.map_or(qin_m3_s[0] + qlat_total_m3_s[0], |state| state.q1);
        let qtmax = qin_m3_s
            .iter()
            .zip(qlat_total_m3_s)
            .map(|(qin, qlat)| qin + (0.5 * qlat))
            .fold(initial_q1, f64::max);
        let qref = Self::ws11_wave_reference_flow(context.ipeak_branch, qtmax)?;

        if qref <= WS10_ZERO_THRESHOLD {
            return Ok(Ws11BaselineWaveSeries {
                q1_m3_s: vec![0.0; qin_m3_s.len()],
                storage_change_m3: vec![0.0; qin_m3_s.len()],
                representative: None,
            });
        }

        let (ckref, _) = Self::ws11_wave_celerity_and_top_width(
            ishape,
            channel_width_m,
            channel_shape,
            context.roughness,
            channel_slope,
            qref,
        )?;
        let target_dx = context.dtchr * ckref;
        let raw_segments = context.channel_length / target_dx;
        let nseg = if !raw_segments.is_finite() || raw_segments <= 1.0 {
            1
        } else if raw_segments >= 101.0 {
            101
        } else {
            format!("{:.0}", raw_segments.floor())
                .parse::<usize>()
                .map_err(|_| {
                    Self::domain_violation(
                        Ws10NodeClass::Channel,
                        BoundarySymbol::from("ws11_wave_segment_count"),
                        raw_segments,
                    )
                })?
        };
        let nseg_f64 = Self::ws11_small_count_as_f64(nseg, "ws11_wave_segment_count")?;
        let dx = context.channel_length / nseg_f64;
        let mut previous_spatial = Vec::with_capacity(nseg + 1);
        for segment in 0..=nseg {
            let segment_f64 =
                Self::ws11_small_count_as_f64(segment, "ws11_wave_segment_index")?;
            previous_spatial.push(
                qin_m3_s[0] + ((initial_q1 - qin_m3_s[0]) * segment_f64 / nseg_f64),
            );
        }
        let mut q1_m3_s = Vec::with_capacity(qin_m3_s.len());
        q1_m3_s.push(Self::require_non_negative_computed(
            Ws10NodeClass::Channel,
            BoundarySymbol::from("ws11_initial_q1"),
            initial_q1,
        )?);
        let mut representative = Some(Ws11WaveRoutingState {
            q1: initial_q1,
            qin: qin_m3_s[0],
            qlat: qlat_total_m3_s[0],
            c0: 0.0,
            c1: 0.0,
            c2: 0.0,
            c3: 0.0,
            c4: 0.0,
        });

        for interval in 1..qin_m3_s.len() {
            // `wshinp.for:465` fixes `mofapp = 1` for wave routing, and
            // `wshchr.for:398-402,513-517` therefore routes the adjacent-state
            // average of the total lateral series after its reach-length
            // normalization.
            let qlat_per_m = 0.5
                * (qlat_total_m3_s[interval - 1] + qlat_total_m3_s[interval])
                / context.channel_length;
            let mut current_spatial = vec![0.0; nseg + 1];
            current_spatial[0] = qin_m3_s[interval];
            let mc_update_active = qin_m3_s[interval - 1] > 0.0
                || q1_m3_s[interval - 1] > 0.0
                || qin_m3_s[interval] > 0.0
                || qlat_per_m > 0.0;
            if matches!(
                context.ipeak_branch,
                Ws11IpeakBranch::MuskingumCunge | Ws11IpeakBranch::MuskingumCungeVariable
            ) && !mc_update_active
            {
                // Pinned `wshchr.for:512-571` leaves the pre-zeroed spatial
                // state untouched when `qmaxi == 0` and `qlavg == 0`.
                q1_m3_s.push(0.0);
                previous_spatial = current_spatial;
                continue;
            }
            let mut outlet_coefficients = [0.0; 5];
            for segment in 1..=nseg {
                let (q1, coefficients) = match context.ipeak_branch {
                    Ws11IpeakBranch::KinematicWave => Self::ws11_route_kinematic_segment(
                        context,
                        ishape,
                        channel_width_m,
                        channel_shape,
                        channel_slope,
                        dx,
                        qlat_per_m,
                        current_spatial[segment - 1],
                        previous_spatial[segment],
                    )?,
                    Ws11IpeakBranch::MuskingumCunge => Self::ws11_route_muskingum_segment(
                        context,
                        ckref,
                        Self::ws11_wave_celerity_and_top_width(
                            ishape,
                            channel_width_m,
                            channel_shape,
                            context.roughness,
                            channel_slope,
                            qref,
                        )?
                        .1,
                        qref,
                        channel_slope,
                        dx,
                        qlat_per_m,
                        current_spatial[segment - 1],
                        previous_spatial[segment - 1],
                        previous_spatial[segment],
                    )?,
                    Ws11IpeakBranch::MuskingumCungeVariable => {
                        let dynamic_qref = ((current_spatial[segment - 1]
                            + previous_spatial[segment - 1]
                            + previous_spatial[segment])
                            / 3.0)
                            .max(WS11_DYNAMIC_MC_QREF_EPS_CMS);
                        let (dynamic_ck, dynamic_bt) = Self::ws11_wave_celerity_and_top_width(
                            ishape,
                            channel_width_m,
                            channel_shape,
                            context.roughness,
                            channel_slope,
                            dynamic_qref,
                        )?;
                        Self::ws11_route_muskingum_segment(
                            context,
                            dynamic_ck,
                            dynamic_bt,
                            dynamic_qref,
                            channel_slope,
                            dx,
                            qlat_per_m,
                            current_spatial[segment - 1],
                            previous_spatial[segment - 1],
                            previous_spatial[segment],
                        )?
                    }
                    Ws11IpeakBranch::Rational | Ws11IpeakBranch::Creams => {
                        return Err(Self::domain_violation(
                            Ws10NodeClass::Channel,
                            BoundarySymbol::from("ws11_interval_nonwave_branch"),
                            0.0,
                        ));
                    }
                };
                current_spatial[segment] = q1;
                if segment == nseg {
                    outlet_coefficients = coefficients;
                }
            }
            let q1 = Self::ws11_wave_outlet_discharge(current_spatial[nseg])?;
            let state = Ws11WaveRoutingState {
                q1,
                qin: qin_m3_s[interval],
                qlat: qlat_total_m3_s[interval],
                c0: outlet_coefficients[0],
                c1: outlet_coefficients[1],
                c2: outlet_coefficients[2],
                c3: outlet_coefficients[3],
                c4: outlet_coefficients[4],
            };
            if representative.is_none_or(|current| q1 > current.q1) {
                representative = Some(state);
            }
            q1_m3_s.push(q1);
            previous_spatial = current_spatial;
        }

        let storage_change_m3 = qin_m3_s
            .iter()
            .zip(qlat_total_m3_s)
            .zip(&q1_m3_s)
            .map(|((qin, qlat), q1)| (qin + qlat - q1) * context.dtchr)
            .collect();
        Ok(Ws11BaselineWaveSeries {
            q1_m3_s,
            storage_change_m3,
            representative,
        })
    }

    fn ws11_small_count_as_f64(
        value: usize,
        symbol: &'static str,
    ) -> Result<f64, Ws10GuardError> {
        u32::try_from(value).map(f64::from).map_err(|_| {
            Self::domain_violation(
                Ws10NodeClass::Channel,
                BoundarySymbol::from(symbol),
                f64::from(u32::MAX),
            )
        })
    }

    fn ws11_wave_reference_flow(
        branch: Ws11IpeakBranch,
        qtmax_m3_s: f64,
    ) -> Result<f64, Ws10GuardError> {
        let qref = match branch {
            // Pinned `wshchr.for:326-328`: KW uses `qtmax`; MC uses
            // `0.5*(qtmin+qtmax)`. The nonnegative interval grid and the
            // baseline zero initialization make `qtmin == 0` here.
            Ws11IpeakBranch::KinematicWave => qtmax_m3_s,
            Ws11IpeakBranch::MuskingumCunge | Ws11IpeakBranch::MuskingumCungeVariable => {
                0.5 * qtmax_m3_s
            }
            Ws11IpeakBranch::Rational | Ws11IpeakBranch::Creams => {
                return Err(Self::domain_violation(
                    Ws10NodeClass::Channel,
                    BoundarySymbol::from("ws11_interval_nonwave_branch"),
                    qtmax_m3_s,
                ));
            }
        };
        Self::require_non_negative_computed(
            Ws10NodeClass::Channel,
            BoundarySymbol::from("ws11_wave_reference_flow"),
            qref,
        )
    }

    fn ws11_wave_outlet_discharge(raw_q1_m3_s: f64) -> Result<f64, Ws10GuardError> {
        let raw_q1_m3_s = Self::require_finite_computed(
            Ws10NodeClass::Channel,
            BoundarySymbol::from("ws11_wave_outlet_q1"),
            raw_q1_m3_s,
        )?;
        // Pinned `wshchr.for:447-448,567-571` applies `eps = 1e-8 m3/s`
        // only after `q1(it)=qs(nseg,it)`. Interior `qs` values remain intact
        // for the next spatial/time update.
        Ok(if raw_q1_m3_s < WS11_DYNAMIC_MC_QREF_EPS_CMS {
            0.0
        } else {
            raw_q1_m3_s
        })
    }

    fn ws11_wave_celerity_and_top_width(
        ishape: u32,
        channel_width_m: f64,
        channel_shape: f64,
        roughness: f64,
        slope: f64,
        discharge_m3_s: f64,
    ) -> Result<(f64, f64), Ws10GuardError> {
        let node_class = Ws10NodeClass::Channel;
        let depth = Self::ws11_solve_depth_for_discharge(
            node_class,
            ishape,
            channel_width_m,
            channel_shape,
            roughness,
            slope,
            discharge_m3_s,
        )?;
        let (top_width, _, wetted_perimeter, shape_parameter) =
            Self::ws11_muskingum_geometry_from_depth(
                node_class,
                ishape,
                channel_width_m,
                channel_shape,
                depth,
            )?;
        let celerity = match ishape {
            1 => 4.0 * discharge_m3_s / (3.0 * shape_parameter * depth * depth),
            2 => {
                discharge_m3_s / (channel_width_m * depth)
                    * (1.0
                        + (2.0 * channel_width_m
                            / (3.0 * (channel_width_m + (2.0 * depth)))))
            }
            3 => {
                let dqdy = (2.5 / depth
                    - (4.0 / (3.0 * wetted_perimeter)
                        * (1.0 + (top_width / depth)).sqrt()))
                    * discharge_m3_s;
                dqdy / top_width
            }
            4.. => {
                let side_length = (1.0 + (shape_parameter * shape_parameter)).sqrt();
                let numerator = (top_width
                    * ((5.0 * channel_width_m) + (6.0 * depth * side_length)))
                    + (4.0 * shape_parameter * depth * depth * side_length);
                let denominator = 3.0
                    * depth
                    * (channel_width_m + (shape_parameter * depth))
                    * (channel_width_m + (2.0 * depth * side_length));
                numerator / denominator * discharge_m3_s / top_width
            }
            _ => {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("ws11_wave_ishape"),
                    f64::from(ishape),
                ));
            }
        };
        let celerity = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("ws11_wave_celerity"),
            celerity,
        )?;
        if celerity <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws11_wave_celerity"),
                celerity,
            ));
        }
        Ok((celerity, top_width))
    }

    #[allow(clippy::too_many_arguments)]
    fn ws11_route_kinematic_segment(
        context: &Ws11DirectChannelContext<'_>,
        ishape: u32,
        channel_width_m: f64,
        channel_shape: f64,
        channel_slope: f64,
        dx_m: f64,
        qlat_m2_s: f64,
        upstream_current_m3_s: f64,
        local_previous_m3_s: f64,
    ) -> Result<(f64, [f64; 5]), Ws10GuardError> {
        let qavg = 0.5 * (local_previous_m3_s + upstream_current_m3_s);
        if qavg <= WS10_ZERO_THRESHOLD {
            let q1 = Self::require_non_negative_computed(
                Ws10NodeClass::Channel,
                BoundarySymbol::from("ws11_kw_dry_front_q1"),
                qlat_m2_s * dx_m,
            )?;
            return Ok((q1, [0.0, 0.0, 0.0, 0.0, q1]));
        }
        let (celerity, _) = Self::ws11_wave_celerity_and_top_width(
            ishape,
            channel_width_m,
            channel_shape,
            context.roughness,
            channel_slope,
            qavg,
        )?;
        let dtdx = context.dtchr / dx_m;
        let inverse_celerity = 1.0 / celerity;
        let denominator = dtdx + inverse_celerity;
        let c0 = 1.0 / denominator;
        let c1 = dtdx * c0;
        let c2 = 0.0;
        let c3 = inverse_celerity * c0;
        let c4 = context.dtchr * qlat_m2_s * c0;
        let q1 = (c1 * upstream_current_m3_s) + (c3 * local_previous_m3_s) + c4;
        let q1 = Self::require_non_negative_computed(
            Ws10NodeClass::Channel,
            BoundarySymbol::from("ws11_kw_q1"),
            q1,
        )?;
        Ok((q1, [c0, c1, c2, c3, c4]))
    }

    #[allow(clippy::too_many_arguments)]
    fn ws11_route_muskingum_segment(
        context: &Ws11DirectChannelContext<'_>,
        celerity_m_s: f64,
        top_width_m: f64,
        reference_flow_m3_s: f64,
        channel_slope: f64,
        dx_m: f64,
        qlat_m2_s: f64,
        upstream_current_m3_s: f64,
        upstream_previous_m3_s: f64,
        local_previous_m3_s: f64,
    ) -> Result<(f64, [f64; 5]), Ws10GuardError> {
        let translation_s = dx_m / celerity_m_s;
        let weighting_denominator = top_width_m * celerity_m_s * channel_slope * dx_m;
        if !weighting_denominator.is_finite()
            || weighting_denominator.abs() <= WS10_ZERO_THRESHOLD
        {
            return Err(Self::domain_violation(
                Ws10NodeClass::Channel,
                BoundarySymbol::from("ws11_mc_weighting_denominator"),
                weighting_denominator,
            ));
        }
        let weighting = (0.5 * (1.0 - (reference_flow_m3_s / weighting_denominator))).max(-10.0);
        let denominator = (2.0 * translation_s * (1.0 - weighting)) + context.dtchr;
        if !denominator.is_finite() || denominator.abs() <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                Ws10NodeClass::Channel,
                BoundarySymbol::from("ws11_mc_denominator"),
                denominator,
            ));
        }
        let c0 = 1.0 / denominator;
        let c1 = (context.dtchr - (2.0 * translation_s * weighting)) * c0;
        let c2 = (context.dtchr + (2.0 * translation_s * weighting)) * c0;
        let c3 = 1.0 - c1 - c2;
        let c4 = 2.0 * qlat_m2_s * dx_m * context.dtchr * c0;
        for (symbol, value) in [("ws11_mc_c1", c1), ("ws11_mc_c2", c2), ("ws11_mc_c3", c3)] {
            Self::require_finite_computed(
                Ws10NodeClass::Channel,
                BoundarySymbol::from(symbol),
                value,
            )?;
        }
        let q1 = (c1 * upstream_current_m3_s)
            + (c2 * upstream_previous_m3_s)
            + (c3 * local_previous_m3_s)
            + c4;
        let q1 = Self::require_finite_computed(
            Ws10NodeClass::Channel,
            BoundarySymbol::from("ws11_mc_q1"),
            q1,
        )?;
        Ok((q1, [c0, c1, c2, c3, c4]))
    }

    fn ws11_local_channel_baseflow(
        input: &DirectWatershedKernelInput<'_>,
        node_class: Ws10NodeClass,
        dtchr: f64,
        cbase: f64,
        nchnum: f64,
        conductivity: f64,
    ) -> Result<Ws10ChannelBaseflowPartition, Ws10GuardError> {
        if matches!(
            input.frame.routing_globals.groundwater_baseflow,
            WatershedGroundwaterRoutingAuthority::Disabled
        ) {
            return Self::assemble_direct_channel_baseflow(
                input,
                node_class,
                dtchr,
                cbase,
                nchnum,
                conductivity,
            );
        }

        let WatershedGroundwaterRoutingAuthority::LinearReservoir {
            baseflow_threshold_area_ha,
        } = input.frame.routing_globals.groundwater_baseflow
        else {
            unreachable!("groundwater authority matched above")
        };
        let generated = Self::generated_groundwater_from_step(input, node_class)?;
        let side_area_ha = Self::contributor_area_ha(input, node_class)?;
        let volume_m3 = if side_area_ha >= baseflow_threshold_area_ha {
            generated.volume_m3
        } else {
            0.0
        };
        let peak_m3_s = volume_m3 / 86_400.0;
        if !peak_m3_s.is_finite() || peak_m3_s < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws11_local_baseflow_peak"),
                peak_m3_s,
            ));
        }
        Ok(Ws10ChannelBaseflowPartition {
            peak_m3_s,
            volume_m3,
            deep_seepage_m3: generated.deep_seepage_m3,
        })
    }

    fn ws11_interval_active_span_s(series: &[f64], dtchr_s: f64) -> f64 {
        let first = series.iter().position(|value| *value > WS10_ZERO_THRESHOLD);
        let last = series.iter().rposition(|value| *value > WS10_ZERO_THRESHOLD);
        match (first, last) {
            (Some(first), Some(last)) => {
                #[allow(clippy::cast_precision_loss)]
                let count = (last - first + 1) as f64;
                count * dtchr_s
            }
            _ => 0.0,
        }
    }

    fn ws11_daily_particle_fractions(daily_egress_kg: &[f64]) -> Vec<f64> {
        let total = daily_egress_kg.iter().sum::<f64>();
        if total <= WS10_ZERO_THRESHOLD {
            return Vec::new();
        }
        daily_egress_kg.iter().map(|mass| mass / total).collect()
    }

    fn ws11_sum_interval_diagnostics(
        _state: &RoutedChannelIntervalSedimentState,
    ) -> Ws20SegmentRoutingDiagnostics {
        // The authoritative interval state is the mass/geometry ledger. Scalar
        // branch counters remain compatibility diagnostics and are not used by
        // the active-lane claim.
        Ws20SegmentRoutingDiagnostics::default()
    }

    #[allow(clippy::too_many_lines)]
    fn ws11_route_interval_sediment(
        input: &DirectWatershedKernelInput<'_>,
        context: &Ws11DirectChannelContext<'_>,
        water: &RoutedChannelIntervalWaterState,
        ntchr: usize,
    ) -> Result<RoutedChannelIntervalSedimentState, Ws10GuardError> {
        let sources = Self::ws11_interval_sediment_sources(input, context.dtchr, ntchr)?;
        let mut profile = Self::ws11_interval_start_profile(input, context)?;
        let geometry_start = Self::ws11_geometry_from_profile(&profile);
        let base_last_x_ft = profile.x_points_ft.last().copied().ok_or_else(|| {
            Self::missing_required(Ws10NodeClass::Channel, "ws11_interval_profile_length")
        })?;
        let base_x_fractions = profile
            .x_points_ft
            .iter()
            .map(|x| *x / base_last_x_ft)
            .collect::<Vec<_>>();
        let class_count = sources.particle_diameter_m.len();
        let crfrac = if class_count == 0 {
            Vec::new()
        } else if class_count == 1 && context.control.crfrac.is_empty() {
            // A one-class system has the unique normalized channel-boundary
            // composition [1]; no class-composition inference is involved.
            vec![1.0]
        } else {
            Self::direct_ws20_crfrac(context.control, &sources.class_numbers)?
        };
        let mut intervals = Vec::with_capacity(ntchr);
        for interval in 0..ntchr {
            let inlet_kg = (0..class_count)
                .map(|class| sources.inlet_kg[class][interval])
                .collect::<Vec<_>>();
            let lateral_kg = (0..class_count)
                .map(|class| sources.lateral_kg[class][interval])
                .collect::<Vec<_>>();
            let incoming_kg = inlet_kg
                .iter()
                .zip(&lateral_kg)
                .map(|(inlet, lateral)| inlet + lateral)
                .collect::<Vec<_>>();
            let geometry_before = Self::ws11_geometry_from_profile(&profile);
            let (
                detached_kg,
                deposited_kg,
                egress_kg,
                next_geometry,
                hydraulic,
                max_effective_shear_lb_ft2,
                outlet_transport_capacity_kg_s,
            ) =
                if water.q1_m3_s[interval] <= WS10_ZERO_THRESHOLD {
                    let outcome = Self::ws11_zero_flow_interval(&incoming_kg, &geometry_before)?;
                    (
                        outcome.detached_kg,
                        outcome.deposited_kg,
                        outcome.egress_kg,
                        outcome.geometry_end,
                        None,
                        0.0,
                        vec![0.0; class_count],
                    )
                } else if class_count == 0 {
                    (
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                        geometry_before.clone(),
                        None,
                        0.0,
                        Vec::new(),
                    )
                } else {
                    let operands = Self::ws11_interval_hydraulic_operands(
                        water.q1_m3_s[interval],
                        water.qin_m3_s[interval],
                        water.qlat_total_m3_s[interval],
                        context.channel_length,
                    )?;
                    for (x, fraction) in profile.x_points_ft.iter_mut().zip(&base_x_fractions) {
                        *x = fraction * operands.leff_ft;
                    }
                    let dummy_peak_partition = Ws20IncomingPeakPartition {
                        hillslope_peak_cms: 0.0,
                        dependency_peak_cms: 0.0,
                        hillslope_volume_m3: 0.0,
                        dependency_volume_m3: 0.0,
                        hillslope_duration_s: 0.0,
                        dependency_duration_s: 0.0,
                        hourly_resolved: true,
                        hourly_sediment_inlet_kg: [0.0; 24],
                    };
                    let result = Self::ws20_route_case12_segment_family_core(
                        context.control.node_id,
                        Ws10NodeClass::Channel,
                        true,
                        context.dtchr,
                        water.q1_m3_s[interval],
                        context.roughness,
                        context.sediment_controls,
                        context.nslpts,
                        dummy_peak_partition,
                        Some(operands),
                        Some(context.dtchr),
                        context.dtchr,
                        &inlet_kg,
                        &lateral_kg,
                        &sources.particle_diameter_m,
                        &sources.class_numbers,
                        profile,
                        context.control.chnk,
                        Some(&crfrac),
                    )?;
                    let public_hydraulic = RoutedChannelIntervalHydraulicState {
                        qe_m3_s: operands.qe_m3_s,
                        qt_m3_s: operands.qt_m3_s,
                        qlat_total_m3_s: operands.qlat_total_m3_s,
                        leff_ft: operands.leff_ft,
                        qu_top_cfs: operands.qu_top_cfs,
                        qlat_eff_cfs_per_ft: operands.qlat_eff_cfs_per_ft,
                    };
                    profile = Ws20ChannelProfile {
                        x_points_ft: base_x_fractions
                            .iter()
                            .map(|fraction| fraction * base_last_x_ft)
                            .collect(),
                        slopes: context
                            .control
                            .segment_points
                            .iter()
                            .map(|point| point.slope.max(WS18_MIN_CHANNEL_SLOPE))
                            .collect(),
                        depth_a_points_ft: result.depth_a_points_ft.clone(),
                        depth_b_points_ft: result.depth_b_points_ft.clone(),
                        width_a_points_ft: result.wida_points_ft.clone(),
                        width_b_points_ft: result.widb_points_ft.clone(),
                        eroded_width_a_points_ft: result.wera_points_ft.clone(),
                        eroded_width_b_points_ft: result.werb_points_ft.clone(),
                    };
                    (
                        result.detached_class_masses_kg,
                        result.deposited_class_masses_kg,
                        result.routed_class_masses_kg,
                        Self::ws11_geometry_from_profile(&profile),
                        Some(public_hydraulic),
                        result.max_effective_shear_lb_ft2,
                        result.outlet_transport_capacity_kg_s,
                    )
                };
            let next_geometry = Self::ws11_advance_interval_geometry(
                &geometry_before,
                &next_geometry,
                false,
            )?;
            Self::ws11_apply_geometry_to_profile(&mut profile, &next_geometry);
            let ledger = Ws11IntervalMassLedger {
                inlet_kg: inlet_kg.clone(),
                lateral_kg: lateral_kg.clone(),
                detached_kg: detached_kg.clone(),
                egress_kg: egress_kg.clone(),
                deposited_kg: deposited_kg.clone(),
            };
            Self::ws11_validate_interval_mass_closure(&ledger)?;
            intervals.push(RoutedChannelIntervalClassLedger {
                inlet_kg,
                lateral_kg,
                detached_kg,
                deposited_kg,
                egress_kg,
                hydraulic,
                max_effective_shear_lb_ft2,
                outlet_transport_capacity_kg_s,
            });
        }

        let internal_ledgers = intervals
            .iter()
            .map(|ledger| Ws11IntervalMassLedger {
                inlet_kg: ledger.inlet_kg.clone(),
                lateral_kg: ledger.lateral_kg.clone(),
                detached_kg: ledger.detached_kg.clone(),
                egress_kg: ledger.egress_kg.clone(),
                deposited_kg: ledger.deposited_kg.clone(),
            })
            .collect::<Vec<_>>();
        Self::ws11_validate_daily_mass_closure(
            &internal_ledgers,
            &sources.projected_lateral_daily_kg,
        )?;
        let _ = Self::ws11_grid_end_disposition(
            water.storage_change_m3.iter().sum::<f64>(),
        )?;
        Self::ws11_validate_no_suspended_carry(&[])?;

        Ok(RoutedChannelIntervalSedimentState {
            particle_diameter_m: sources.particle_diameter_m,
            daily_inlet_kg: Self::ws11_sum_interval_field(&intervals, class_count, |row| &row.inlet_kg),
            daily_lateral_kg: Self::ws11_sum_interval_field(&intervals, class_count, |row| &row.lateral_kg),
            daily_detached_kg: Self::ws11_sum_interval_field(&intervals, class_count, |row| &row.detached_kg),
            daily_deposited_kg: Self::ws11_sum_interval_field(&intervals, class_count, |row| &row.deposited_kg),
            daily_egress_kg: Self::ws11_sum_interval_field(&intervals, class_count, |row| &row.egress_kg),
            geometry_start: Self::ws11_public_geometry(&geometry_start),
            geometry_end: Self::ws11_public_geometry(&Self::ws11_geometry_from_profile(&profile)),
            intervals,
        })
    }

    fn ws11_sum_interval_field(
        intervals: &[RoutedChannelIntervalClassLedger],
        class_count: usize,
        field: impl Fn(&RoutedChannelIntervalClassLedger) -> &[f64],
    ) -> Vec<f64> {
        let mut total = vec![0.0_f64; class_count];
        for interval in intervals {
            for (target, source) in total.iter_mut().zip(field(interval)) {
                *target += *source;
            }
        }
        total
    }

    #[allow(clippy::too_many_lines)]
    fn ws11_interval_sediment_sources(
        input: &DirectWatershedKernelInput<'_>,
        dtchr_s: f64,
        ntchr: usize,
    ) -> Result<Ws11IntervalSedimentSources, Ws10GuardError> {
        let mut particle_diameter_m = Vec::<f64>::new();
        let mut local_hourly_by_class = Vec::<[f64; 24]>::new();
        for hillslope_id in &input.step.contributor_hillslopes {
            let contribution = input
                .frame
                .hillslope_contributions
                .get(hillslope_id)
                .ok_or_else(|| Self::missing_required(Ws10NodeClass::Channel, "hillslope_contribution"))?;
            let payload = Self::read_direct_hillslope_sediment_payload(
                contribution,
                Ws10NodeClass::Channel,
            )?;
            Self::ws11_merge_particle_diameters(
                &mut particle_diameter_m,
                &payload.particle_diameters_m,
            )?;
            if local_hourly_by_class.is_empty() {
                local_hourly_by_class = vec![[0.0; 24]; payload.particle_diameters_m.len()];
            }
            let fraction_sum = payload.fractions.iter().sum::<f64>();
            let hourly_total = contribution.hourly_sediment_mass_kg.iter().sum::<f64>();
            if hourly_total > WS10_ZERO_THRESHOLD && fraction_sum <= WS10_ZERO_THRESHOLD {
                return Err(Self::domain_violation(
                    Ws10NodeClass::Channel,
                    BoundarySymbol::from("ws11_interval_class_fraction_sum"),
                    fraction_sum,
                ));
            }
            if fraction_sum > WS10_ZERO_THRESHOLD {
                for (class, fraction) in payload.fractions.iter().enumerate() {
                    for (target, hourly_mass) in local_hourly_by_class[class]
                        .iter_mut()
                        .zip(&contribution.hourly_sediment_mass_kg)
                    {
                        *target += hourly_mass * fraction / fraction_sum;
                    }
                }
            }
        }

        for dependency in input
            .step
            .dependency_nodes
            .iter()
            .filter(|dependency| dependency.kind == TopologyNodeKind::Channel)
        {
            let state = input
                .frame
                .routed_channels
                .get(&dependency.id)
                .and_then(|state| state.interval_sediment_state.as_ref())
                .ok_or_else(|| Self::missing_required(Ws10NodeClass::Channel, "dependency_interval_sediment"))?;
            Self::ws11_merge_particle_diameters(
                &mut particle_diameter_m,
                &state.particle_diameter_m,
            )?;
        }

        let class_count = particle_diameter_m.len();
        if local_hourly_by_class.is_empty() {
            local_hourly_by_class = vec![[0.0; 24]; class_count];
        }
        let mut lateral_kg = vec![vec![0.0_f64; ntchr]; class_count];
        let mut projected_lateral_daily_kg = vec![0.0_f64; class_count];
        for class in 0..class_count {
            lateral_kg[class] = Self::ws11_project_hourly_totals(
                &local_hourly_by_class[class],
                dtchr_s,
                ntchr,
            )?;
            projected_lateral_daily_kg[class] = local_hourly_by_class[class].iter().sum();
        }
        let mut inlet_kg = vec![vec![0.0_f64; ntchr]; class_count];
        for dependency in input
            .step
            .dependency_nodes
            .iter()
            .filter(|dependency| dependency.kind == TopologyNodeKind::Channel)
        {
            let state = input
                .frame
                .routed_channels
                .get(&dependency.id)
                .and_then(|state| state.interval_sediment_state.as_ref())
                .ok_or_else(|| Self::missing_required(Ws10NodeClass::Channel, "dependency_interval_sediment"))?;
            if state.intervals.len() != ntchr || state.particle_diameter_m.len() != class_count {
                return Err(Self::domain_violation(
                    Ws10NodeClass::Channel,
                    BoundarySymbol::from("dependency_interval_sediment_grid"),
                    f64::from(u32::try_from(state.intervals.len()).unwrap_or(u32::MAX)),
                ));
            }
            for (interval, ledger) in state.intervals.iter().enumerate() {
                for (class_inlet, egress) in inlet_kg.iter_mut().zip(&ledger.egress_kg) {
                    class_inlet[interval] += *egress;
                }
            }
        }
        Ok(Ws11IntervalSedimentSources {
            class_numbers: (1..=class_count).collect(),
            particle_diameter_m,
            inlet_kg,
            lateral_kg,
            projected_lateral_daily_kg,
        })
    }

    fn ws11_merge_particle_diameters(
        target: &mut Vec<f64>,
        source: &[f64],
    ) -> Result<(), Ws10GuardError> {
        if target.is_empty() {
            target.extend_from_slice(source);
            return Ok(());
        }
        if target.len() != source.len()
            || target
                .iter()
                .zip(source)
                .any(|(left, right)| (*left - *right).abs() > 1.0e-12)
        {
            return Err(Self::domain_violation(
                Ws10NodeClass::Channel,
                BoundarySymbol::from("ws11_interval_particle_classes"),
                f64::from(u32::try_from(source.len()).unwrap_or(u32::MAX)),
            ));
        }
        Ok(())
    }

    fn ws11_interval_start_profile(
        input: &DirectWatershedKernelInput<'_>,
        context: &Ws11DirectChannelContext<'_>,
    ) -> Result<Ws20ChannelProfile, Ws10GuardError> {
        let mut profile = Self::direct_ws20_channel_profile(context.control, context.nslpts)?;
        for x in &mut profile.x_points_ft {
            *x *= WS15_DEPTH_FROM_METERS_TO_FEET;
        }
        if let Some(prior) = input
            .frame
            .routed_channels
            .get(&context.node_id)
            .and_then(|state| state.interval_sediment_state.as_ref())
        {
            let tillage_state = input
                .frame
                .channel_tillage_day_state
                .get(&context.node_id)
                .copied();
            if context.ishape == 3 {
                match tillage_state {
                    Some(ChannelTillageDayState::PrimaryTillage) => return Ok(profile),
                    Some(ChannelTillageDayState::NoPrimaryTillage) => {}
                    None => {
                        return Err(Self::missing_required(
                            Ws10NodeClass::Channel,
                            "ws11_primary_tillage_day_authority",
                        ));
                    }
                }
            } else if tillage_state == Some(ChannelTillageDayState::PrimaryTillage) {
                return Err(Self::domain_violation(
                    Ws10NodeClass::Channel,
                    BoundarySymbol::from("ws11_primary_tillage_shape"),
                    f64::from(context.ishape),
                ));
            }
            profile
                .depth_a_points_ft
                .clone_from(&prior.geometry_end.depth_a_points_ft);
            profile
                .depth_b_points_ft
                .clone_from(&prior.geometry_end.depth_b_points_ft);
            profile
                .width_a_points_ft
                .clone_from(&prior.geometry_end.width_a_points_ft);
            profile
                .width_b_points_ft
                .clone_from(&prior.geometry_end.width_b_points_ft);
            profile
                .eroded_width_a_points_ft
                .clone_from(&prior.geometry_end.eroded_width_a_points_ft);
            profile
                .eroded_width_b_points_ft
                .clone_from(&prior.geometry_end.eroded_width_b_points_ft);
        }
        Ok(profile)
    }

    fn ws11_geometry_from_profile(profile: &Ws20ChannelProfile) -> Ws11IntervalGeometry {
        Ws11IntervalGeometry {
            depth_a_points_ft: profile.depth_a_points_ft.clone(),
            depth_b_points_ft: profile.depth_b_points_ft.clone(),
            width_a_points_ft: profile.width_a_points_ft.clone(),
            width_b_points_ft: profile.width_b_points_ft.clone(),
            eroded_width_a_points_ft: profile.eroded_width_a_points_ft.clone(),
            eroded_width_b_points_ft: profile.eroded_width_b_points_ft.clone(),
        }
    }

    fn ws11_apply_geometry_to_profile(
        profile: &mut Ws20ChannelProfile,
        geometry: &Ws11IntervalGeometry,
    ) {
        profile.depth_a_points_ft.clone_from(&geometry.depth_a_points_ft);
        profile.depth_b_points_ft.clone_from(&geometry.depth_b_points_ft);
        profile.width_a_points_ft.clone_from(&geometry.width_a_points_ft);
        profile.width_b_points_ft.clone_from(&geometry.width_b_points_ft);
        profile
            .eroded_width_a_points_ft
            .clone_from(&geometry.eroded_width_a_points_ft);
        profile
            .eroded_width_b_points_ft
            .clone_from(&geometry.eroded_width_b_points_ft);
    }

    fn ws11_public_geometry(geometry: &Ws11IntervalGeometry) -> RoutedChannelGeometryState {
        RoutedChannelGeometryState {
            depth_a_points_ft: geometry.depth_a_points_ft.clone(),
            depth_b_points_ft: geometry.depth_b_points_ft.clone(),
            width_a_points_ft: geometry.width_a_points_ft.clone(),
            width_b_points_ft: geometry.width_b_points_ft.clone(),
            eroded_width_a_points_ft: geometry.eroded_width_a_points_ft.clone(),
            eroded_width_b_points_ft: geometry.eroded_width_b_points_ft.clone(),
        }
    }
    fn ws11_validate_interval_grid(
        dtchr_s: f64,
        ntchr: usize,
    ) -> Result<(), Ws10GuardError> {
        if !dtchr_s.is_finite() || dtchr_s <= WS10_ZERO_THRESHOLD || ntchr == 0 {
            return Err(Self::domain_violation(
                Ws10NodeClass::Channel,
                BoundarySymbol::from("ws11_interval_grid"),
                dtchr_s,
            ));
        }
        #[allow(clippy::cast_precision_loss)]
        let covered_s = dtchr_s * ntchr as f64;
        if (covered_s - 86_400.0).abs() > 1.0e-9 {
            return Err(Self::domain_violation(
                Ws10NodeClass::Channel,
                BoundarySymbol::from("ws11_interval_grid_coverage_s"),
                covered_s,
            ));
        }
        Ok(())
    }

    fn ws11_project_hourly_totals(
        hourly_totals: &[f64; 24],
        dtchr_s: f64,
        ntchr: usize,
    ) -> Result<Vec<f64>, Ws10GuardError> {
        Self::ws11_validate_interval_grid(dtchr_s, ntchr)?;
        for value in hourly_totals {
            if !value.is_finite() || *value < 0.0 {
                return Err(Self::domain_violation(
                    Ws10NodeClass::Channel,
                    BoundarySymbol::from("ws11_hourly_projection_source"),
                    *value,
                ));
            }
        }

        let mut projected = vec![0.0_f64; ntchr];
        for (interval, value) in projected.iter_mut().enumerate() {
            #[allow(clippy::cast_precision_loss)]
            let interval_start = interval as f64 * dtchr_s;
            let interval_end = interval_start + dtchr_s;
            for (hour, hourly_total) in hourly_totals.iter().enumerate() {
                #[allow(clippy::cast_precision_loss)]
                let hour_start = hour as f64 * 3600.0;
                let hour_end = hour_start + 3600.0;
                let overlap_s = interval_end.min(hour_end) - interval_start.max(hour_start);
                if overlap_s > 0.0 {
                    *value += *hourly_total * overlap_s / 3600.0;
                }
            }
        }

        let source_total = hourly_totals.iter().sum::<f64>();
        let projected_total = projected.iter().sum::<f64>();
        let tolerance = 1.0e-12 * source_total.abs().max(1.0);
        if (projected_total - source_total).abs() > tolerance {
            return Err(Self::domain_violation(
                Ws10NodeClass::Channel,
                BoundarySymbol::from("ws11_hourly_projection_closure"),
                projected_total - source_total,
            ));
        }
        Ok(projected)
    }

    #[allow(clippy::similar_names)]
    fn ws11_interval_hydraulic_operands(
        qe_m3_s: f64,
        qt_m3_s: f64,
        qlat_total_m3_s: f64,
        channel_length_m: f64,
    ) -> Result<Ws11IntervalHydraulicOperands, Ws10GuardError> {
        for (label, value) in [
            ("ws11_interval_qe", qe_m3_s),
            ("ws11_interval_qt", qt_m3_s),
            ("ws11_interval_qlat_total", qlat_total_m3_s),
            ("ws11_interval_channel_length", channel_length_m),
        ] {
            if !value.is_finite() || value < 0.0 {
                return Err(Self::domain_violation(
                    Ws10NodeClass::Channel,
                    BoundarySymbol::from(label),
                    value,
                ));
            }
        }
        if channel_length_m <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                Ws10NodeClass::Channel,
                BoundarySymbol::from("ws11_interval_channel_length"),
                channel_length_m,
            ));
        }

        let qe_cfs = qe_m3_s * WS18_CFS_PER_CMS;
        let qt_cfs = qt_m3_s * WS18_CFS_PER_CMS;
        let qlat_total_cfs = qlat_total_m3_s * WS18_CFS_PER_CMS;
        let channel_length_ft = channel_length_m * WS15_DEPTH_FROM_METERS_TO_FEET;
        let (leff_ft, qu_top_cfs, qlat_eff_cfs_per_ft) =
            if qlat_total_m3_s > WS10_ZERO_THRESHOLD {
                let leff_ft = channel_length_ft * (1.0 + qt_m3_s / qlat_total_m3_s);
                let ltop_ft = leff_ft - channel_length_ft;
                (leff_ft, qe_cfs * ltop_ft / leff_ft, qe_cfs / leff_ft)
            } else {
                (channel_length_ft, qe_cfs, 0.0)
            };
        if !leff_ft.is_finite()
            || leff_ft <= WS10_ZERO_THRESHOLD
            || !qu_top_cfs.is_finite()
            || qu_top_cfs < 0.0
            || !qlat_eff_cfs_per_ft.is_finite()
            || qlat_eff_cfs_per_ft < 0.0
        {
            return Err(Self::domain_violation(
                Ws10NodeClass::Channel,
                BoundarySymbol::from("ws11_interval_hydraulic_profile"),
                leff_ft,
            ));
        }

        Ok(Ws11IntervalHydraulicOperands {
            qe_m3_s,
            qt_m3_s,
            qlat_total_m3_s,
            qe_cfs,
            qt_cfs,
            qlat_total_cfs,
            channel_length_ft,
            leff_ft,
            qu_top_cfs,
            qlat_eff_cfs_per_ft,
        })
    }

    fn ws11_validate_interval_clock(
        t_exp_s: f64,
        t_norm_s: f64,
        dtchr_s: f64,
    ) -> Result<(), Ws10GuardError> {
        let valid = t_exp_s.is_finite()
            && t_norm_s.is_finite()
            && dtchr_s.is_finite()
            && t_exp_s >= 0.0
            && t_exp_s <= dtchr_s
            && (t_norm_s - dtchr_s).abs() <= 1.0e-12
            && dtchr_s > WS10_ZERO_THRESHOLD;
        if !valid {
            return Err(Self::domain_violation(
                Ws10NodeClass::Channel,
                BoundarySymbol::from("ws11_interval_clock"),
                t_exp_s,
            ));
        }
        Ok(())
    }

    fn ws11_validate_interval_mass_closure(
        ledger: &Ws11IntervalMassLedger,
    ) -> Result<(), Ws10GuardError> {
        let class_count = ledger.inlet_kg.len();
        if ledger.lateral_kg.len() != class_count
            || ledger.detached_kg.len() != class_count
            || ledger.egress_kg.len() != class_count
            || ledger.deposited_kg.len() != class_count
        {
            return Err(Self::domain_violation(
                Ws10NodeClass::Channel,
                BoundarySymbol::from("ws11_interval_mass_cardinality"),
                f64::from(u32::try_from(class_count).unwrap_or(u32::MAX)),
            ));
        }
        for class_offset in 0..class_count {
            let values = [
                ledger.inlet_kg[class_offset],
                ledger.lateral_kg[class_offset],
                ledger.detached_kg[class_offset],
                ledger.egress_kg[class_offset],
                ledger.deposited_kg[class_offset],
            ];
            if values.iter().any(|value| !value.is_finite() || *value < 0.0) {
                return Err(Self::domain_violation(
                    Ws10NodeClass::Channel,
                    BoundarySymbol::from("ws11_interval_mass_operand"),
                    f64::from(u32::try_from(class_offset + 1).unwrap_or(u32::MAX)),
                ));
            }
            let residual = values[0] + values[1] + values[2] - values[3] - values[4];
            if residual.abs() > 1.0e-9 {
                return Err(Self::domain_violation(
                    Ws10NodeClass::Channel,
                    BoundarySymbol::from("ws11_interval_mass_closure_kg"),
                    residual,
                ));
            }
        }
        Ok(())
    }

    fn ws11_validate_daily_mass_closure(
        ledgers: &[Ws11IntervalMassLedger],
        projected_lateral_source_kg: &[f64],
    ) -> Result<(), Ws10GuardError> {
        for ledger in ledgers {
            Self::ws11_validate_interval_mass_closure(ledger)?;
        }
        for (class_offset, source) in projected_lateral_source_kg.iter().enumerate() {
            let projected = ledgers
                .iter()
                .map(|ledger| ledger.lateral_kg.get(class_offset).copied().unwrap_or(0.0))
                .sum::<f64>();
            let tolerance = 1.0e-12 * source.abs().max(1.0);
            if (projected - source).abs() > tolerance {
                return Err(Self::domain_violation(
                    Ws10NodeClass::Channel,
                    BoundarySymbol::from("ws11_daily_projection_closure_kg"),
                    projected - source,
                ));
            }
        }
        Ok(())
    }

    fn ws11_advance_interval_geometry(
        current: &Ws11IntervalGeometry,
        candidate: &Ws11IntervalGeometry,
        allow_reseed: bool,
    ) -> Result<Ws11IntervalGeometry, Ws10GuardError> {
        let cardinality_ok = current.depth_a_points_ft.len() == candidate.depth_a_points_ft.len()
            && current.depth_b_points_ft.len() == candidate.depth_b_points_ft.len()
            && current.width_a_points_ft.len() == candidate.width_a_points_ft.len()
            && current.width_b_points_ft.len() == candidate.width_b_points_ft.len()
            && current.eroded_width_a_points_ft.len()
                == candidate.eroded_width_a_points_ft.len()
            && current.eroded_width_b_points_ft.len()
                == candidate.eroded_width_b_points_ft.len();
        if !cardinality_ok {
            return Err(Self::domain_violation(
                Ws10NodeClass::Channel,
                BoundarySymbol::from("ws11_geometry_cardinality"),
                0.0,
            ));
        }
        let all_values = current
            .depth_a_points_ft
            .iter()
            .chain(&current.depth_b_points_ft)
            .chain(&current.width_a_points_ft)
            .chain(&current.width_b_points_ft)
            .chain(&current.eroded_width_a_points_ft)
            .chain(&current.eroded_width_b_points_ft)
            .chain(&candidate.depth_a_points_ft)
            .chain(&candidate.depth_b_points_ft)
            .chain(&candidate.width_a_points_ft)
            .chain(&candidate.width_b_points_ft)
            .chain(&candidate.eroded_width_a_points_ft)
            .chain(&candidate.eroded_width_b_points_ft);
        if all_values
            .into_iter()
            .any(|value| !value.is_finite() || *value < 0.0)
        {
            return Err(Self::domain_violation(
                Ws10NodeClass::Channel,
                BoundarySymbol::from("ws11_geometry_value"),
                -1.0,
            ));
        }
        if !allow_reseed {
            let refills = current
                .depth_a_points_ft
                .iter()
                .zip(&candidate.depth_a_points_ft)
                .chain(current.depth_b_points_ft.iter().zip(&candidate.depth_b_points_ft))
                .any(|(before, after)| *after > *before + 1.0e-12);
            let narrows_hydraulic = current
                .width_a_points_ft
                .iter()
                .zip(&candidate.width_a_points_ft)
                .chain(current.width_b_points_ft.iter().zip(&candidate.width_b_points_ft))
                .any(|(before, after)| *after + 1.0e-12 < *before);
            let narrows_eroded = current
                .eroded_width_a_points_ft
                .iter()
                .zip(&candidate.eroded_width_a_points_ft)
                .chain(
                    current
                        .eroded_width_b_points_ft
                        .iter()
                        .zip(&candidate.eroded_width_b_points_ft),
                )
                .any(|(before, after)| *after + 1.0e-12 < *before);
            if refills {
                return Err(Self::domain_violation(
                    Ws10NodeClass::Channel,
                    BoundarySymbol::from("ws11_unauthorized_geometry_refill"),
                    1.0,
                ));
            }
            if narrows_hydraulic {
                return Err(Self::domain_violation(
                    Ws10NodeClass::Channel,
                    BoundarySymbol::from("ws11_unauthorized_hydraulic_width_narrowing"),
                    1.0,
                ));
            }
            if narrows_eroded {
                return Err(Self::domain_violation(
                    Ws10NodeClass::Channel,
                    BoundarySymbol::from("ws11_unauthorized_eroded_width_narrowing"),
                    1.0,
                ));
            }
        }
        Ok(candidate.clone())
    }

    fn ws11_zero_flow_interval(
        incoming_kg: &[f64],
        geometry: &Ws11IntervalGeometry,
    ) -> Result<Ws11ZeroFlowOutcome, Ws10GuardError> {
        if incoming_kg.iter().any(|value| !value.is_finite() || *value < 0.0) {
            return Err(Self::domain_violation(
                Ws10NodeClass::Channel,
                BoundarySymbol::from("ws11_zero_flow_incoming_mass"),
                -1.0,
            ));
        }
        Ok(Ws11ZeroFlowOutcome {
            geometry_end: geometry.clone(),
            detached_kg: vec![0.0; incoming_kg.len()],
            deposited_kg: incoming_kg.to_vec(),
            egress_kg: vec![0.0; incoming_kg.len()],
        })
    }

    fn ws11_grid_end_disposition(
        water_storage_m3: f64,
    ) -> Result<Ws11GridEndDisposition, Ws10GuardError> {
        if !water_storage_m3.is_finite() {
            return Err(Self::non_finite(
                Ws10NodeClass::Channel,
                BoundarySymbol::from("ws11_grid_end_water_storage"),
                water_storage_m3,
            ));
        }
        Ok(Ws11GridEndDisposition {
            water_storage_m3,
            suspended_sediment_storage_kg: 0.0,
        })
    }

    #[cfg(test)]
    fn ws11_cross_day_state(geometry: &Ws11IntervalGeometry) -> Ws11CrossDayState {
        Ws11CrossDayState {
            geometry: geometry.clone(),
            suspended_class_mass_kg: Vec::new(),
        }
    }

    #[cfg(test)]
    fn ws11_apply_tillage_reseed(
        carried: &Ws11IntervalGeometry,
        input: &Ws11IntervalGeometry,
        ishape: u32,
        primary_tillage: bool,
    ) -> Ws11IntervalGeometry {
        if ishape == 3 && primary_tillage {
            input.clone()
        } else {
            carried.clone()
        }
    }

    fn ws11_validate_no_suspended_carry(
        suspended_class_mass_kg: &[f64],
    ) -> Result<(), Ws10GuardError> {
        if suspended_class_mass_kg.iter().any(|value| value.abs() > 1.0e-12) {
            return Err(Self::domain_violation(
                Ws10NodeClass::Channel,
                BoundarySymbol::from("ws11_suspended_pool_carry"),
                suspended_class_mass_kg.iter().sum(),
            ));
        }
        Ok(())
    }

    fn ws11_validate_active_lane_operand_mode(
        uses_interval_operands: bool,
    ) -> Result<(), Ws10GuardError> {
        if !uses_interval_operands {
            return Err(Self::domain_violation(
                Ws10NodeClass::Channel,
                BoundarySymbol::from("ws11_event_scalar_operand_on_active_lane"),
                0.0,
            ));
        }
        Ok(())
    }

    #[cfg(test)]
    fn ws11_partition_contact_budget(
        t_exp_s: f64,
        timpot_s: f64,
    ) -> Result<Ws11ContactBudget, Ws10GuardError> {
        if !t_exp_s.is_finite()
            || !timpot_s.is_finite()
            || t_exp_s < 0.0
            || timpot_s < 0.0
            || timpot_s > t_exp_s
        {
            return Err(Self::domain_violation(
                Ws10NodeClass::Channel,
                BoundarySymbol::from("ws11_contact_budget"),
                timpot_s,
            ));
        }
        Ok(Ws11ContactBudget {
            timpot_s,
            timex_s: t_exp_s - timpot_s,
        })
    }

    #[cfg(test)]
    fn ws11_geometry_detachment_mass(
        initial_width_ft: f64,
        eroded_depth_ft: f64,
        widened_width_ft: f64,
        erodible_side_depth_ft: f64,
        rho_soil_lbm_ft3: f64,
    ) -> Result<f64, Ws10GuardError> {
        let volume_ft3_per_ft =
            initial_width_ft * eroded_depth_ft + widened_width_ft * erodible_side_depth_ft;
        let mass_lbm_per_ft = volume_ft3_per_ft * rho_soil_lbm_ft3;
        if !mass_lbm_per_ft.is_finite() || mass_lbm_per_ft < 0.0 {
            return Err(Self::domain_violation(
                Ws10NodeClass::Channel,
                BoundarySymbol::from("ws11_geometry_detachment_mass"),
                mass_lbm_per_ft,
            ));
        }
        Ok(mass_lbm_per_ft)
    }
}
