impl Ws10ChannelImpoundmentKernel {
    #[allow(clippy::too_many_lines)]
    fn run_channel_node(
        request: &WatershedKernelRequest<'_>,
    ) -> Result<KernelRunResponse, Ws10GuardError> {
        let node_class = Ws10NodeClass::Channel;
        let dtchr_symbol = WatershedProductionStateSymbol::Dtchr;
        let dtchr = Self::require_state_scalar(request, node_class, dtchr_symbol)?;
        Self::require_state_range(
            node_class,
            dtchr_symbol,
            dtchr,
            Some(WS10_ZERO_THRESHOLD),
            None,
        )?;

        let nchnum_symbol = WatershedProductionStateSymbol::Nchnum;
        let nchnum = Self::require_state_scalar(request, node_class, nchnum_symbol)?;
        Self::require_state_range(
            node_class,
            nchnum_symbol,
            nchnum,
            Some(WS10_ZERO_THRESHOLD),
            None,
        )?;

        let cbase_symbol = WatershedProductionFluxSymbol::Cbase;
        let cbase = Self::require_flux_scalar(request, node_class, cbase_symbol)?;
        Self::require_flux_range(node_class, cbase_symbol, cbase, Some(0.0), None)?;
        let ipeak_branch = Self::require_ipeak_branch(request, node_class)?;

        let roughness_symbol = WatershedProductionStateSymbol::ChannelNode {
            node_id: request.node_id,
            field: WatershedChannelStateField::Chnn,
        };
        let slope_symbol = WatershedProductionStateSymbol::ChannelNode {
            node_id: request.node_id,
            field: WatershedChannelStateField::Ctlslp,
        };
        let conductivity_symbol = WatershedProductionStateSymbol::ChannelNode {
            node_id: request.node_id,
            field: WatershedChannelStateField::Chnk,
        };

        let roughness = Self::require_state_scalar(request, node_class, roughness_symbol)?;
        Self::require_state_range(
            node_class,
            roughness_symbol,
            roughness,
            Some(WS10_ZERO_THRESHOLD),
            None,
        )?;
        let control_slope = Self::require_state_scalar(request, node_class, slope_symbol)?;
        Self::require_state_range(node_class, slope_symbol, control_slope, Some(0.0), None)?;
        let conductivity = Self::require_state_scalar(request, node_class, conductivity_symbol)?;
        Self::require_state_range(
            node_class,
            conductivity_symbol,
            conductivity,
            Some(0.0),
            None,
        )?;
        let sediment_controls = Self::read_ws15_channel_sediment_controls(request, node_class)?;
        let nslpts = Self::require_ws17_channel_segment_scaffold(request, node_class)?;
        let channel_length =
            Self::require_ws11_channel_length_from_scaffold(request, node_class, nslpts)?;
        let sediment_scaffold = Self::derive_ws15_channel_sediment_scaffold(
            node_class,
            request.node_id,
            sediment_controls,
        )?;
        let ishape_value = sediment_controls.ishape.round();
        let ishape = format!("{ishape_value:.0}").parse::<u32>().map_err(|_| {
            Self::domain_violation(
                node_class,
                Self::channel_wave_state_symbol(request.node_id, "ishape"),
                sediment_controls.ishape,
            )
        })?;

        let peak_partition = Self::assemble_incoming_peak_partition(request, node_class)?;
        let incoming_peak = peak_partition.hillslope_peak_cms + peak_partition.dependency_peak_cms;

        let routing_gain = (1.0 + control_slope) / (1.0 + roughness);
        if !routing_gain.is_finite() || routing_gain <= 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("routing_gain"),
                routing_gain,
            ));
        }

        let baseflow_peak = cbase * nchnum * (1.0 + conductivity * dtchr);
        if !baseflow_peak.is_finite() || baseflow_peak < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("baseflow_peak"),
                baseflow_peak,
            ));
        }

        let available_peak = incoming_peak + baseflow_peak;
        if !available_peak.is_finite() || available_peak < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("available_peak"),
                available_peak,
            ));
        }

        let rvolat = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("rvolat"),
            peak_partition.hillslope_volume_m3,
        )?;
        let rvotop = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("rvotop"),
            peak_partition.dependency_volume_m3,
        )?;
        let rvolon = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("rvolon"),
            rvolat + rvotop,
        )?;
        let durlat = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("durlat"),
            peak_partition.hillslope_duration_s,
        )?;
        let durtop = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("durtop"),
            peak_partition.dependency_duration_s,
        )?;
        let durrunon = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("durrunon"),
            durlat.max(durtop),
        )?;
        let durchan = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("durchan"),
            dtchr,
        )?;
        let durirrig = 0.0_f64;
        let watdur = durrunon.max(durchan).max(durirrig);
        if !watdur.is_finite() || watdur <= 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("watdur"),
                watdur,
            ));
        }

        let channel_runoff_volume_m3 = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("rofc"),
            baseflow_peak * dtchr,
        )?;
        let ws11_case_id =
            if rvolon <= WS10_ZERO_THRESHOLD && channel_runoff_volume_m3 <= WS10_ZERO_THRESHOLD {
                1_u32
            } else if channel_runoff_volume_m3 > 0.001 {
                2_u32
            } else if rvolon <= 0.001 {
                4_u32
            } else {
                3_u32
            };
        let tl = if ws11_case_id == 4 { rvolon } else { 0.0 };
        let runvol_case = match ws11_case_id {
            1 | 4 => 0.0,
            2 => rvolon + channel_runoff_volume_m3,
            3 => rvolon,
            _ => unreachable!("ws11_case_id is constrained to [1,4]"),
        };
        let qci = if watdur > WS10_ZERO_THRESHOLD {
            rvolon / watdur
        } else {
            0.0
        };
        let qcf = if watdur > WS10_ZERO_THRESHOLD {
            runvol_case / watdur
        } else {
            0.0
        };

        let qci =
            Self::require_non_negative_computed(node_class, BoundarySymbol::from("qci"), qci)?;
        let qcf =
            Self::require_non_negative_computed(node_class, BoundarySymbol::from("qcf"), qcf)?;

        let mut wave_state: Option<Ws11WaveRoutingState> = None;
        let qpo = if available_peak <= WS10_ZERO_THRESHOLD {
            0.0
        } else {
            match ipeak_branch {
                Ws11IpeakBranch::Rational => {
                    if runvol_case <= 0.001 {
                        0.0
                    } else {
                        (runvol_case / watdur) * routing_gain
                    }
                }
                Ws11IpeakBranch::Creams => {
                    if runvol_case <= 0.001 {
                        0.0
                    } else {
                        let creams_attenuation = 1.0 + (conductivity * dtchr);
                        if !creams_attenuation.is_finite() || creams_attenuation <= 0.0 {
                            return Err(Self::domain_violation(
                                node_class,
                                BoundarySymbol::from("creams_attenuation"),
                                creams_attenuation,
                            ));
                        }

                        let creams_gain = (routing_gain / creams_attenuation).sqrt();
                        if !creams_gain.is_finite() || creams_gain <= 0.0 {
                            return Err(Self::domain_violation(
                                node_class,
                                BoundarySymbol::from("creams_gain"),
                                creams_gain,
                            ));
                        }

                        (runvol_case / watdur) * creams_gain
                    }
                }
                Ws11IpeakBranch::KinematicWave => {
                    if runvol_case <= 0.001 && incoming_peak <= WS10_ZERO_THRESHOLD {
                        0.0
                    } else {
                        let state = Self::compute_kinematic_wave_state(
                            node_class,
                            roughness,
                            conductivity,
                            nchnum,
                            routing_gain,
                            incoming_peak,
                            available_peak,
                            baseflow_peak,
                            dtchr,
                            watdur,
                        )?;
                        let q1 = state.q1;
                        wave_state = Some(state);
                        q1
                    }
                }
                Ws11IpeakBranch::MuskingumCunge => {
                    if runvol_case <= 0.001 && incoming_peak <= WS10_ZERO_THRESHOLD {
                        0.0
                    } else {
                        let prior_qin = Self::optional_channel_wave_state_scalar(
                            request,
                            node_class,
                            request.node_id,
                            "qin",
                        )?;
                        let prior_q1 = Self::optional_channel_wave_state_scalar(
                            request,
                            node_class,
                            request.node_id,
                            "q1",
                        )?;
                        let state = Self::compute_muskingum_cunge_state(
                            node_class,
                            roughness,
                            control_slope,
                            conductivity,
                            nchnum,
                            available_peak,
                            baseflow_peak,
                            dtchr,
                            watdur,
                            prior_qin,
                            prior_q1,
                        )?;
                        let q1 = state.q1;
                        wave_state = Some(state);
                        q1
                    }
                }
                Ws11IpeakBranch::MuskingumCungeVariable => {
                    if runvol_case <= 0.001 && incoming_peak <= WS10_ZERO_THRESHOLD {
                        0.0
                    } else {
                        let prior_qin = Self::optional_channel_wave_state_scalar(
                            request,
                            node_class,
                            request.node_id,
                            "qin",
                        )?;
                        let prior_q1 = Self::optional_channel_wave_state_scalar(
                            request,
                            node_class,
                            request.node_id,
                            "q1",
                        )?;
                        let state = Self::compute_variable_muskingum_cunge_state(
                            node_class,
                            roughness,
                            control_slope,
                            sediment_controls.ctlz,
                            sediment_controls.chnz,
                            ishape,
                            channel_length,
                            available_peak,
                            baseflow_peak,
                            dtchr,
                            watdur,
                            prior_qin,
                            prior_q1,
                        )?;
                        let q1 = state.q1;
                        wave_state = Some(state);
                        q1
                    }
                }
            }
        };

        if !qpo.is_finite() || qpo < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("qpo"),
                qpo,
            ));
        }

        let roff = if matches!(
            ipeak_branch,
            Ws11IpeakBranch::Rational | Ws11IpeakBranch::Creams
        ) {
            if runvol_case <= 0.001 {
                0.0
            } else {
                runvol_case
            }
        } else {
            qpo * watdur
        };
        if !roff.is_finite() || roff < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("roff"),
                roff,
            ));
        }

        let durrof = if qpo <= WS10_ZERO_THRESHOLD {
            0.0
        } else {
            roff / qpo
        };
        if !durrof.is_finite() || durrof < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("durrof"),
                durrof,
            ));
        }
        let sediment_publication = Self::assemble_incoming_sediment_load_and_capacity(
            request,
            node_class,
            watdur,
            qpo,
            roughness,
            sediment_controls,
            nslpts,
            peak_partition,
        )?;

        let Ok(status) =
            SimulationStatus::ok(SimulationPhase::WatershedKernel, WS10_CHANNEL_OK_MESSAGE_ID)
        else {
            unreachable!("status message ids are non-empty WS10 constants")
        };

        let qpo_symbol = WatershedProductionStateSymbol::ChannelNode {
            node_id: request.node_id,
            field: WatershedChannelStateField::Qpo,
        };
        let durrof_symbol = WatershedProductionStateSymbol::ChannelNode {
            node_id: request.node_id,
            field: WatershedChannelStateField::Durrof,
        };
        let roff_symbol = WatershedProductionFluxSymbol::ChannelNode {
            node_id: request.node_id,
            field: WatershedChannelFluxField::Roff,
        };

        let mut state_updates = vec![
            WritebackField::bounded(qpo_symbol, qpo, Some(0.0), None),
            WritebackField::bounded(durrof_symbol, durrof, Some(0.0), None),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "rvolat"),
                rvolat,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "rvotop"),
                rvotop,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "rvolon"),
                rvolon,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "durrunon"),
                durrunon,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "durlat"),
                durlat,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "durtop"),
                durtop,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "durchan"),
                durchan,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "durirrig"),
                durirrig,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "watdur"),
                watdur,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "rofc"),
                channel_runoff_volume_m3,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "tl"),
                tl,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "ws11_runoff_case"),
                f64::from(ws11_case_id),
                Some(1.0),
                Some(4.0),
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "ws11_qci"),
                qci,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "ws11_qcf"),
                qcf,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "ws11_runvol"),
                runvol_case,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "qsed"),
                sediment_publication.qsed,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "tc"),
                sediment_publication.tc,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "ws20_case1_segment_count"),
                f64::from(sediment_publication.ws20_case1_segments),
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "ws20_case2_segment_count"),
                f64::from(sediment_publication.ws20_case2_segments),
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "ws24_case2_detach_segment_count"),
                f64::from(sediment_publication.ws24_case2_detach_segments),
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "ws21_case3_segment_count"),
                f64::from(sediment_publication.ws21_case3_segments),
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "ws21_case4_segment_count"),
                f64::from(sediment_publication.ws21_case4_segments),
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "ws21_enddet_segment_count"),
                f64::from(sediment_publication.ws21_enddet_segments),
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "chz"),
                sediment_scaffold.chz,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "nbarch"),
                sediment_scaffold.nbarch,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "crsh"),
                sediment_scaffold.crsh,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "depmid"),
                sediment_scaffold.depmid,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "depsid"),
                sediment_scaffold.depsid,
                Some(0.0),
                None,
            ),
        ];
        if let Some(widb_points_ft) = &sediment_publication.ws29_widb_points_ft {
            for (point_index, width_ft) in widb_points_ft.iter().copied().enumerate() {
                let point_number = point_index + 1;
                state_updates.push(WritebackField::bounded(
                    Self::channel_wave_state_symbol(
                        request.node_id,
                        &format!("widb_{point_number:04}"),
                    ),
                    width_ft,
                    Some(WS10_ZERO_THRESHOLD),
                    None,
                ));
            }
        }
        if let Some(wida_points_ft) = &sediment_publication.ws31_wida_points_ft {
            for (point_index, width_ft) in wida_points_ft.iter().copied().enumerate() {
                let point_number = point_index + 1;
                state_updates.push(WritebackField::bounded(
                    Self::channel_wave_state_symbol(
                        request.node_id,
                        &format!("wida_{point_number:04}"),
                    ),
                    width_ft,
                    Some(WS10_ZERO_THRESHOLD),
                    None,
                ));
            }
        }
        let class_count_scalar = f64::from(
            u32::try_from(sediment_publication.particle_flow_fractions.len()).unwrap_or(u32::MAX),
        );
        state_updates.push(WritebackField::bounded(
            Self::channel_wave_state_symbol(request.node_id, "particle_class_count"),
            class_count_scalar,
            Some(0.0),
            None,
        ));
        for (class_index, (fraction, diameter)) in sediment_publication
            .particle_flow_fractions
            .iter()
            .zip(sediment_publication.particle_diameters_m.iter())
            .enumerate()
        {
            let class = class_index + 1;
            state_updates.push(WritebackField::bounded(
                Self::channel_wave_state_symbol(
                    request.node_id,
                    &format!("particle_flow_fraction_{class:04}"),
                ),
                *fraction,
                Some(0.0),
                Some(1.0),
            ));
            state_updates.push(WritebackField::bounded(
                Self::channel_wave_state_symbol(
                    request.node_id,
                    &format!("particle_diameter_m_{class:04}"),
                ),
                *diameter,
                Some(WS10_ZERO_THRESHOLD),
                None,
            ));
        }
        if let Some(state) = wave_state {
            let node_id = request.node_id;
            state_updates.push(WritebackField::bounded(
                Self::channel_wave_state_symbol(node_id, "q1"),
                state.q1,
                Some(0.0),
                None,
            ));
            state_updates.push(WritebackField::bounded(
                Self::channel_wave_state_symbol(node_id, "qin"),
                state.qin,
                Some(0.0),
                None,
            ));
            state_updates.push(WritebackField::bounded(
                Self::channel_wave_state_symbol(node_id, "qlat"),
                state.qlat,
                Some(0.0),
                None,
            ));
            state_updates.push(WritebackField::bounded(
                Self::channel_wave_state_symbol(node_id, "c0"),
                state.c0,
                Some(0.0),
                None,
            ));
            state_updates.push(WritebackField::bounded(
                Self::channel_wave_state_symbol(node_id, "c1"),
                state.c1,
                None,
                None,
            ));
            state_updates.push(WritebackField::bounded(
                Self::channel_wave_state_symbol(node_id, "c2"),
                state.c2,
                None,
                None,
            ));
            state_updates.push(WritebackField::bounded(
                Self::channel_wave_state_symbol(node_id, "c3"),
                state.c3,
                None,
                None,
            ));
            state_updates.push(WritebackField::bounded(
                Self::channel_wave_state_symbol(node_id, "c4"),
                state.c4,
                Some(0.0),
                None,
            ));
        }

        let writeback = KernelWritebackPayload::with_updates(
            state_updates,
            vec![WritebackField::bounded(roff_symbol, roff, Some(0.0), None)],
        );

        Ok(KernelRunResponse::new(status, writeback))
    }

    #[allow(clippy::too_many_lines)]
    fn run_impoundment_node(
        request: &WatershedKernelRequest<'_>,
    ) -> Result<KernelRunResponse, Ws10GuardError> {
        let node_class = Ws10NodeClass::Impoundment;

        let h_symbol = WatershedProductionStateSymbol::ImpoundmentNode {
            node_id: request.node_id,
            field: WatershedImpoundmentStateField::H,
        };
        let hfull_symbol = WatershedProductionStateSymbol::ImpoundmentNode {
            node_id: request.node_id,
            field: WatershedImpoundmentStateField::Hfull,
        };
        let deltat_symbol = WatershedProductionStateSymbol::ImpoundmentNode {
            node_id: request.node_id,
            field: WatershedImpoundmentStateField::Deltat,
        };
        let qinf_symbol = WatershedProductionStateSymbol::ImpoundmentNode {
            node_id: request.node_id,
            field: WatershedImpoundmentStateField::Qinf,
        };

        let stage_h = Self::require_state_scalar(request, node_class, h_symbol)?;
        let hfull = Self::require_state_scalar(request, node_class, hfull_symbol)?;
        let deltat = Self::require_state_scalar(request, node_class, deltat_symbol)?;
        let qinf = Self::require_state_scalar(request, node_class, qinf_symbol)?;

        Self::require_state_range(node_class, h_symbol, stage_h, Some(0.0), None)?;
        Self::require_state_range(
            node_class,
            hfull_symbol,
            hfull,
            Some(WS10_ZERO_THRESHOLD),
            None,
        )?;
        if stage_h > hfull {
            return Err(Self::domain_violation(node_class, h_symbol, stage_h));
        }
        Self::require_state_range(
            node_class,
            deltat_symbol,
            deltat,
            Some(WS10_ZERO_THRESHOLD),
            None,
        )?;
        Self::require_state_range(node_class, qinf_symbol, qinf, Some(0.0), None)?;

        let (incoming_peak, incoming_duration) =
            Self::assemble_incoming_peak_and_duration(request, node_class)?;

        let mut family_a = [0.0_f64; 15];
        let mut family_b = [0.0_f64; 15];
        let mut family_c = [0.0_f64; 15];
        let mut family_d = [0.0_f64; 15];
        let mut family_e = [0.0_f64; 15];
        let mut family_head_threshold = [0.0_f64; 15];
        for family_index in 1..=15 {
            family_a[family_index - 1] = Self::require_impoundment_function_coefficient_scalar(
                request,
                request.node_id,
                family_index,
                "a",
            )?;
            family_b[family_index - 1] = Self::require_impoundment_function_coefficient_scalar(
                request,
                request.node_id,
                family_index,
                "b",
            )?;
            family_c[family_index - 1] = Self::require_impoundment_function_coefficient_scalar(
                request,
                request.node_id,
                family_index,
                "c",
            )?;
            family_d[family_index - 1] = Self::require_impoundment_function_coefficient_scalar(
                request,
                request.node_id,
                family_index,
                "d",
            )?;
            family_e[family_index - 1] = Self::require_impoundment_function_coefficient_scalar(
                request,
                request.node_id,
                family_index,
                "e",
            )?;
            family_head_threshold[family_index - 1] =
                Self::require_impoundment_function_coefficient_scalar(
                    request,
                    request.node_id,
                    family_index,
                    "ha",
                )?;
        }

        let a0 = Self::require_impoundment_coefficient_scalar(request, request.node_id, "a0")?;
        let a1 = Self::require_impoundment_coefficient_scalar(request, request.node_id, "a1")?;
        let a2 = Self::require_impoundment_coefficient_scalar(request, request.node_id, "a2")?;
        let _l0 = Self::require_impoundment_coefficient_scalar(request, request.node_id, "l0")?;
        let _l1 = Self::require_impoundment_coefficient_scalar(request, request.node_id, "l1")?;
        let _l2 = Self::require_impoundment_coefficient_scalar(request, request.node_id, "l2")?;

        let coefficients = Ws12ImpoundmentCoefficients {
            a: family_a,
            b: family_b,
            c: family_c,
            d: family_d,
            e: family_e,
            ha: family_head_threshold,
            a0,
            a1,
            a2,
        };

        let incoming_duration_hours = incoming_duration / 3600.0;
        if !incoming_duration_hours.is_finite() || incoming_duration_hours < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("incoming_duration"),
                incoming_duration,
            ));
        }
        let integration_horizon_hours = if incoming_duration_hours > WS10_ZERO_THRESHOLD {
            incoming_duration_hours
        } else {
            deltat
        };
        if !integration_horizon_hours.is_finite()
            || integration_horizon_hours <= WS10_ZERO_THRESHOLD
        {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("integration_horizon_hours"),
                integration_horizon_hours,
            ));
        }

        let (hnext, accepted_deltat) = Self::route_impoundment_stage_over_duration(
            node_class,
            stage_h,
            hfull,
            deltat,
            integration_horizon_hours,
            incoming_peak,
            qinf,
            &coefficients,
        )?;

        let qo = Self::impoundment_outflow_at_stage(node_class, hnext, &coefficients)?;
        let continuity_outflow = qo + qinf;
        if !continuity_outflow.is_finite() || continuity_outflow < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("continuity_outflow"),
                continuity_outflow,
            ));
        }

        let accepted_duration_seconds = accepted_deltat * 3600.0;
        if !accepted_duration_seconds.is_finite() || accepted_duration_seconds < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("accepted_duration_seconds"),
                accepted_duration_seconds,
            ));
        }

        let durout = incoming_duration.max(accepted_duration_seconds);
        if !durout.is_finite() || durout < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("durout"),
                durout,
            ));
        }

        let outflow_volume = qo * durout;
        if !outflow_volume.is_finite() || outflow_volume < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("outflow_volume"),
                outflow_volume,
            ));
        }

        let Ok(status) = SimulationStatus::ok(
            SimulationPhase::WatershedKernel,
            WS10_IMPOUNDMENT_OK_MESSAGE_ID,
        ) else {
            unreachable!("status message ids are non-empty WS10 constants")
        };

        let qo_symbol = WatershedProductionStateSymbol::ImpoundmentNode {
            node_id: request.node_id,
            field: WatershedImpoundmentStateField::Qo,
        };
        let durout_symbol = WatershedProductionStateSymbol::ImpoundmentNode {
            node_id: request.node_id,
            field: WatershedImpoundmentStateField::Durout,
        };
        let hnext_symbol = WatershedProductionStateSymbol::ImpoundmentNode {
            node_id: request.node_id,
            field: WatershedImpoundmentStateField::Hnext,
        };
        let outflow_symbol = WatershedProductionFluxSymbol::ImpoundmentNode {
            node_id: request.node_id,
            field: WatershedImpoundmentFluxField::OutflowVolume,
        };

        let writeback = KernelWritebackPayload::with_updates(
            vec![
                WritebackField::bounded(qo_symbol, qo, Some(0.0), None),
                WritebackField::bounded(durout_symbol, durout, Some(0.0), None),
                WritebackField::bounded(hnext_symbol, hnext, Some(0.0), Some(hfull)),
            ],
            vec![WritebackField::bounded(
                outflow_symbol,
                outflow_volume,
                Some(0.0),
                None,
            )],
        );

        Ok(KernelRunResponse::new(status, writeback))
    }

    fn status_from_guard_error(error: &Ws10GuardError) -> SimulationStatus {
        let Ok(status) = SimulationStatus::failure(
            SimulationPhase::WatershedKernel,
            true,
            false,
            error.boundary_class(),
            error.message_id(),
        ) else {
            unreachable!("status message ids are non-empty WS10 constants")
        };
        status
    }

}
