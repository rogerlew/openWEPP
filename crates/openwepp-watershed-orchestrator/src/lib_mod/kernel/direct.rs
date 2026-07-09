use crate::runtime_inputs::{
    WatershedRuntimeInputError, derive_ws12_impoundment_coefficients,
    derive_ws12_outflow_function_families,
};
use openwepp_topology::TopologyNodeKey;

impl Ws10ChannelImpoundmentKernel {
    pub(crate) fn run_direct_watershed_node(
        input: &DirectWatershedKernelInput<'_>,
    ) -> DirectWatershedKernelResponse {
        let result = match input.step.node.kind {
            TopologyNodeKind::Channel => Self::run_direct_channel_node(input),
            TopologyNodeKind::Impoundment => Self::run_direct_impoundment_node(input),
            TopologyNodeKind::Hillslope => Err(Self::domain_violation(
                Ws10NodeClass::Channel,
                "direct_node_kind",
                -1.0,
            )),
        };

        match result {
            Ok(output) => DirectWatershedKernelResponse {
                status: Self::direct_ok_status(input.step.node.kind),
                output: Some(output),
            },
            Err(error) => DirectWatershedKernelResponse {
                status: Self::status_from_guard_error(&error),
                output: None,
            },
        }
    }

    fn direct_ok_status(kind: TopologyNodeKind) -> SimulationStatus {
        let message_id = match kind {
            TopologyNodeKind::Channel | TopologyNodeKind::Hillslope => WS10_CHANNEL_OK_MESSAGE_ID,
            TopologyNodeKind::Impoundment => WS10_IMPOUNDMENT_OK_MESSAGE_ID,
        };
        let Ok(status) = SimulationStatus::ok(SimulationPhase::WatershedKernel, message_id) else {
            unreachable!("status message ids are non-empty WS10 constants")
        };
        status
    }

    #[allow(clippy::too_many_lines, clippy::similar_names)]
    fn run_direct_channel_node(
        input: &DirectWatershedKernelInput<'_>,
    ) -> Result<DirectWatershedKernelOutput, Ws10GuardError> {
        let node_class = Ws10NodeClass::Channel;
        let node_id = input.step.node.id;
        let globals = &input.frame.routing_globals;
        let control = input
            .frame
            .channel_controls
            .get(&node_id)
            .ok_or_else(|| Self::missing_required(node_class, "channel_control"))?;

        let dtchr = globals.dtchr_seconds;
        Self::direct_require_range(node_class, "dtchr", dtchr, Some(WS10_ZERO_THRESHOLD), None)?;
        let nchnum = globals.nchnum;
        Self::direct_require_range(node_class, "nchnum", nchnum, Some(0.0), None)?;
        let cbase = globals.cbase;
        Self::direct_require_range(node_class, "cbase", cbase, Some(0.0), None)?;
        let ipeak_branch = Self::direct_ipeak_branch(globals.ipeak, node_class)?;

        let roughness = control.chnn;
        Self::direct_require_range(
            node_class,
            "chnn",
            roughness,
            Some(WS10_ZERO_THRESHOLD),
            None,
        )?;
        let control_slope = control.ctlslp;
        Self::direct_require_range(node_class, "ctlslp", control_slope, Some(0.0), None)?;
        let conductivity = control.chnk;
        Self::direct_require_range(node_class, "chnk", conductivity, Some(0.0), None)?;

        let sediment_controls = Self::read_direct_ws15_channel_sediment_controls(control)?;
        let nslpts = Self::require_direct_ws17_channel_segment_scaffold(control)?;
        let channel_length = Self::direct_ws11_channel_length(control)?;
        let _sediment_scaffold =
            Self::derive_ws15_channel_sediment_scaffold(node_class, node_id, sediment_controls)?;
        let ishape_value = sediment_controls.ishape.round();
        let ishape = format!("{ishape_value:.0}")
            .parse::<u32>()
            .map_err(|_| Self::domain_violation(node_class, "ishape", sediment_controls.ishape))?;

        let peak_partition = Self::assemble_direct_incoming_peak_partition(input, node_class)?;
        let incoming_peak = peak_partition.hillslope_peak_cms + peak_partition.dependency_peak_cms;

        let routing_gain = (1.0 + control_slope) / (1.0 + roughness);
        if !routing_gain.is_finite() || routing_gain <= 0.0 {
            return Err(Self::domain_violation(node_class, "routing_gain", routing_gain));
        }

        let channel_baseflow =
            Self::assemble_direct_channel_baseflow(input, node_class, dtchr, cbase, nchnum, conductivity)?;
        let baseflow_peak = channel_baseflow.peak_m3_s;

        let available_peak = incoming_peak + baseflow_peak;
        if !available_peak.is_finite() || available_peak < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                "available_peak",
                available_peak,
            ));
        }

        let rvolat = Self::require_non_negative_computed(
            node_class,
            "rvolat",
            peak_partition.hillslope_volume_m3,
        )?;
        let rvotop = Self::require_non_negative_computed(
            node_class,
            "rvotop",
            peak_partition.dependency_volume_m3,
        )?;
        let rvolon = Self::require_non_negative_computed(node_class, "rvolon", rvolat + rvotop)?;
        let durlat = Self::require_non_negative_computed(
            node_class,
            "durlat",
            peak_partition.hillslope_duration_s,
        )?;
        let durtop = Self::require_non_negative_computed(
            node_class,
            "durtop",
            peak_partition.dependency_duration_s,
        )?;
        let durrunon =
            Self::require_non_negative_computed(node_class, "durrunon", durlat.max(durtop))?;
        let durchan = Self::require_non_negative_computed(node_class, "durchan", dtchr)?;
        let durirrig = 0.0_f64;
        let watdur = durrunon.max(durchan).max(durirrig);
        if !watdur.is_finite() || watdur <= 0.0 {
            return Err(Self::domain_violation(node_class, "watdur", watdur));
        }

        let channel_runoff_volume_m3 =
            Self::require_non_negative_computed(node_class, "rofc", channel_baseflow.volume_m3)?;
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
        let _ = Self::require_non_negative_computed(node_class, "qci", qci)?;
        let _ = Self::require_non_negative_computed(node_class, "qcf", qcf)?;

        let prior_wave = input
            .frame
            .routed_channels
            .get(&node_id)
            .and_then(|state| state.wave_state.as_ref());
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
                                "creams_attenuation",
                                creams_attenuation,
                            ));
                        }

                        let creams_gain = (routing_gain / creams_attenuation).sqrt();
                        if !creams_gain.is_finite() || creams_gain <= 0.0 {
                            return Err(Self::domain_violation(
                                node_class,
                                "creams_gain",
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
                            prior_wave.map(|state| state.qin_m3_s),
                            prior_wave.map(|state| state.q1_m3_s),
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
                            prior_wave.map(|state| state.qin_m3_s),
                            prior_wave.map(|state| state.q1_m3_s),
                        )?;
                        let q1 = state.q1;
                        wave_state = Some(state);
                        q1
                    }
                }
            }
        };

        if !qpo.is_finite() || qpo < 0.0 {
            return Err(Self::domain_violation(node_class, "qpo", qpo));
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
            return Err(Self::domain_violation(node_class, "roff", roff));
        }

        let durrof = if qpo <= WS10_ZERO_THRESHOLD {
            0.0
        } else {
            roff / qpo
        };
        if !durrof.is_finite() || durrof < 0.0 {
            return Err(Self::domain_violation(node_class, "durrof", durrof));
        }

        let sediment_publication = Self::assemble_direct_incoming_sediment_load_and_capacity(
            input,
            control,
            node_class,
            watdur,
            qpo,
            roughness,
            sediment_controls,
            nslpts,
            peak_partition,
        )?;

        let routed_wave_state = wave_state.map(|state| RoutedChannelWaveState {
            q1_m3_s: state.q1,
            qin_m3_s: state.qin,
            qlat_m3_s: state.qlat,
            c0: state.c0,
            c1: state.c1,
            c2: state.c2,
            c3: state.c3,
            c4: state.c4,
        });
        let sediment_state = RoutedChannelSedimentState {
            qsed_kg_s: sediment_publication.qsed,
            transport_capacity_kg_s: sediment_publication.tc,
            particle_flow_fraction: sediment_publication.particle_flow_fractions,
            particle_diameter_m: sediment_publication.particle_diameters_m,
            ws20_case1_segments: sediment_publication.ws20_case1_segments,
            ws20_case2_segments: sediment_publication.ws20_case2_segments,
            ws24_case2_detach_segments: sediment_publication.ws24_case2_detach_segments,
            ws21_case3_segments: sediment_publication.ws21_case3_segments,
            ws21_case4_segments: sediment_publication.ws21_case4_segments,
            ws21_enddet_segments: sediment_publication.ws21_enddet_segments,
        };

        Ok(DirectWatershedKernelOutput::Channel(Box::new(RoutedChannelState {
            node_id,
            runoff_volume_m3: roff,
            channel_inflow_m3: runvol_case,
            channel_outflow_m3: roff,
            channel_storage_m3: 0.0,
            peak_discharge_m3_s: qpo,
            duration_seconds: durrof,
            channel_baseflow_m3: channel_baseflow.volume_m3,
            channel_loss_m3: 0.0,
            groundwater_deep_seepage_m3: channel_baseflow.deep_seepage_m3,
            sediment_yield_kg: sediment_state.qsed_kg_s,
            wave_state: routed_wave_state,
            sediment_state,
        })))
    }

    #[allow(clippy::too_many_lines)]
    fn run_direct_impoundment_node(
        input: &DirectWatershedKernelInput<'_>,
    ) -> Result<DirectWatershedKernelOutput, Ws10GuardError> {
        let node_class = Ws10NodeClass::Impoundment;
        let node_id = input.step.node.id;
        let control = input
            .frame
            .impoundment_controls
            .get(&node_id)
            .ok_or_else(|| Self::missing_required(node_class, "impoundment_control"))?;

        let stage_h = control.h;
        let hfull = control.hfull;
        let deltat = control.deltat;
        let qinf = control.qinf;
        Self::direct_require_range(node_class, "h", stage_h, Some(0.0), None)?;
        Self::direct_require_range(node_class, "hfull", hfull, Some(WS10_ZERO_THRESHOLD), None)?;
        if stage_h > hfull {
            return Err(Self::domain_violation(node_class, "h", stage_h));
        }
        Self::direct_require_range(node_class, "deltat", deltat, Some(WS10_ZERO_THRESHOLD), None)?;
        Self::direct_require_range(node_class, "qinf", qinf, Some(0.0), None)?;

        let (incoming_peak, incoming_duration) =
            Self::assemble_direct_incoming_peak_and_duration(input, node_class)?;
        let coefficients = Self::direct_ws12_impoundment_coefficients(control)?;

        let incoming_duration_hours = incoming_duration / 3600.0;
        if !incoming_duration_hours.is_finite() || incoming_duration_hours < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                "incoming_duration",
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
                "integration_horizon_hours",
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
                "continuity_outflow",
                continuity_outflow,
            ));
        }

        let accepted_duration_seconds = accepted_deltat * 3600.0;
        if !accepted_duration_seconds.is_finite() || accepted_duration_seconds < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                "accepted_duration_seconds",
                accepted_duration_seconds,
            ));
        }

        let durout = incoming_duration.max(accepted_duration_seconds);
        if !durout.is_finite() || durout < 0.0 {
            return Err(Self::domain_violation(node_class, "durout", durout));
        }

        let outflow_volume = qo * durout;
        if !outflow_volume.is_finite() || outflow_volume < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                "outflow_volume",
                outflow_volume,
            ));
        }

        Ok(DirectWatershedKernelOutput::Impoundment(
            RoutedImpoundmentState {
                node_id,
                outflow_volume_m3: outflow_volume,
                outflow_rate_m3_s: qo,
                duration_seconds: durout,
                hnext_m: hnext,
            },
        ))
    }

    fn direct_require_range(
        node_class: Ws10NodeClass,
        label: &'static str,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Result<(), Ws10GuardError> {
        if !value.is_finite() {
            return Err(Self::non_finite(node_class, label, value));
        }
        if let Some(minimum_value) = minimum
            && value < minimum_value
        {
            return Err(Self::domain_violation(node_class, label, value));
        }
        if let Some(maximum_value) = maximum
            && value > maximum_value
        {
            return Err(Self::domain_violation(node_class, label, value));
        }
        Ok(())
    }

    fn direct_ipeak_branch(
        ipeak: i32,
        node_class: Ws10NodeClass,
    ) -> Result<Ws11IpeakBranch, Ws10GuardError> {
        if ipeak < 1 {
            return Err(Self::domain_violation(
                node_class,
                "ipeak",
                f64::from(ipeak),
            ));
        }
        let branch = match ipeak {
            1 => Ws11IpeakBranch::Rational,
            2 => Ws11IpeakBranch::Creams,
            3 => Ws11IpeakBranch::KinematicWave,
            5 => Ws11IpeakBranch::MuskingumCungeVariable,
            _ => Ws11IpeakBranch::MuskingumCunge,
        };
        Ok(branch)
    }

    #[allow(clippy::similar_names)]
    fn read_direct_ws15_channel_sediment_controls(
        control: &WatershedChannelControlRecord,
    ) -> Result<Ws15ChannelSedimentControls, Ws10GuardError> {
        let node_class = Ws10NodeClass::Channel;
        for (label, value, minimum, maximum) in [
            ("ishape", f64::from(control.ishape), Some(1.0), Some(3.0)),
            ("ienslp", f64::from(control.ienslp), Some(1.0), Some(2.0)),
            ("chnz", control.chnz, Some(0.0), None),
            ("chnnbr", control.chnnbr, Some(WS10_ZERO_THRESHOLD), None),
            ("chntcr", control.chntcr, Some(0.0), None),
            ("chnedm", control.chnedm, Some(0.0), None),
            ("chneds", control.chneds, Some(0.0), None),
            ("ctlz", control.ctlz, Some(WS10_ZERO_THRESHOLD), None),
            ("ctln", control.ctln, Some(WS10_ZERO_THRESHOLD), None),
        ] {
            Self::direct_require_range(node_class, label, value, minimum, maximum)?;
        }

        Ok(Ws15ChannelSedimentControls {
            ishape: f64::from(control.ishape),
            ctlz: control.ctlz,
            chnz: control.chnz,
            chnnbr: control.chnnbr,
            chntcr: control.chntcr,
            chnedm: control.chnedm,
            chneds: control.chneds,
        })
    }

    fn require_direct_ws17_channel_segment_scaffold(
        control: &WatershedChannelControlRecord,
    ) -> Result<usize, Ws10GuardError> {
        let node_class = Ws10NodeClass::Channel;
        let nslpts = control.segment_points.len();
        if nslpts < 2 {
            return Err(Self::domain_violation(
                node_class,
                "nslpts",
                f64::from(u32::try_from(nslpts).unwrap_or(u32::MAX)),
            ));
        }

        let mut previous_x: Option<f64> = None;
        for point in &control.segment_points {
            Self::direct_require_range(node_class, "x", point.x_m, Some(0.0), None)?;
            if let Some(previous) = previous_x
                && point.x_m + WS10_ZERO_THRESHOLD < previous
            {
                return Err(Self::domain_violation(node_class, "x", point.x_m));
            }
            Self::direct_require_range(node_class, "slope", point.slope, Some(0.0), None)?;
            Self::direct_require_range(node_class, "depa", point.depth_a_ft, Some(0.0), None)?;
            Self::direct_require_range(node_class, "depb", point.depth_b_ft, Some(0.0), None)?;
            Self::direct_require_range(
                node_class,
                "wida",
                point.width_a_ft,
                Some(WS10_ZERO_THRESHOLD),
                None,
            )?;
            Self::direct_require_range(
                node_class,
                "widb",
                point.width_b_ft,
                Some(WS10_ZERO_THRESHOLD),
                None,
            )?;
            previous_x = Some(point.x_m);
        }

        Ok(nslpts)
    }

    fn direct_ws11_channel_length(
        control: &WatershedChannelControlRecord,
    ) -> Result<f64, Ws10GuardError> {
        let node_class = Ws10NodeClass::Channel;
        let Some(first) = control.segment_points.first() else {
            return Err(Self::domain_violation(node_class, "channel_length", 0.0));
        };
        let Some(last) = control.segment_points.last() else {
            return Err(Self::domain_violation(node_class, "channel_length", 0.0));
        };
        let channel_length = last.x_m - first.x_m;
        if !channel_length.is_finite() || channel_length <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                "channel_length",
                channel_length,
            ));
        }
        Ok(channel_length)
    }

    // One pass over the contributor set feeding BOTH superposition bases
    // (hourly + rectangular fallback); splitting would loop contributors
    // twice or scatter the INV-ROUTE-005 whole-inlet rule.
    #[allow(clippy::too_many_lines)]
    fn assemble_direct_incoming_peak_partition(
        input: &DirectWatershedKernelInput<'_>,
        node_class: Ws10NodeClass,
    ) -> Result<Ws20IncomingPeakPartition, Ws10GuardError> {
        let mut hillslope_peak = 0.0_f64;
        let mut dependency_peak = 0.0_f64;
        let mut hillslope_volume_m3 = 0.0_f64;
        let mut dependency_volume_m3 = 0.0_f64;
        let mut hillslope_duration_s = 0.0_f64;
        let mut dependency_duration_s = 0.0_f64;

        // INV-ROUTE-005 (ADR-0036 D3 / SC-ROUTE-001 rev 49): an inlet is
        // either all-hourly or no-hourly. Partial/malformed hourly authority
        // and hourly hillslopes mixed with dependency nodes fail closed
        // instead of silently falling back to the daily scalar branch.
        let hillslope_hourly_authority =
            Self::direct_hillslope_hourly_authority(input, node_class)?;
        let hourly_resolved = if hillslope_hourly_authority {
            if !input.step.dependency_nodes.is_empty() {
                return Err(Self::domain_violation(
                    node_class,
                    "hillslope_hourly_with_dependency_without_channel_hourly",
                    f64::from(u32::try_from(input.step.dependency_nodes.len()).unwrap_or(u32::MAX)),
                ));
            }
            true
        } else {
            false
        };
        let mut summed_hourly_volume_m3 = [0.0_f64; 24];
        let mut hourly_sediment_inlet_kg = [0.0_f64; 24];

        for &hillslope_id in &input.step.contributor_hillslopes {
            let contribution = input
                .frame
                .hillslope_contributions
                .get(&hillslope_id)
                .ok_or_else(|| Self::missing_required(node_class, "hillslope_contribution"))?;
            let (peak, duration) =
                Self::read_direct_hillslope_peak_payload(contribution, node_class)?;
            let _ = Self::read_direct_hillslope_sediment_payload(contribution, node_class)?;
            if hourly_resolved {
                for (slot, volume_m3) in summed_hourly_volume_m3
                    .iter_mut()
                    .zip(contribution.hourly_runoff_volume_m3.iter())
                {
                    if !volume_m3.is_finite() || *volume_m3 < 0.0 {
                        return Err(Self::domain_violation(
                            node_class,
                            "hillslope_hourly_runoff_volume_m3",
                            *volume_m3,
                        ));
                    }
                    *slot += volume_m3;
                }
                for (slot, sediment_kg) in hourly_sediment_inlet_kg
                    .iter_mut()
                    .zip(contribution.hourly_sediment_mass_kg.iter())
                {
                    if !sediment_kg.is_finite() || *sediment_kg < 0.0 {
                        return Err(Self::domain_violation(
                            node_class,
                            "hillslope_hourly_sediment_mass_kg",
                            *sediment_kg,
                        ));
                    }
                    *slot += sediment_kg;
                }
                continue;
            }
            let volume = peak * duration;
            if !volume.is_finite() {
                return Err(Self::non_finite(node_class, "hillslope_runon_volume", volume));
            }
            if volume < 0.0 {
                return Err(Self::domain_violation(
                    node_class,
                    "hillslope_runon_volume",
                    volume,
                ));
            }
            hillslope_peak += peak;
            hillslope_volume_m3 += volume;
            hillslope_duration_s = hillslope_duration_s.max(duration);
        }

        if hourly_resolved {
            let (peak_cms, volume_m3, duration_s) =
                Self::superposed_hourly_limb(&summed_hourly_volume_m3);
            hillslope_peak = peak_cms;
            hillslope_volume_m3 = volume_m3;
            hillslope_duration_s = duration_s;
        }

        for dependency in &input.step.dependency_nodes {
            let (peak, duration) =
                Self::read_direct_dependency_peak_payload(input.frame, node_class, *dependency)?;
            let volume = peak * duration;
            if !volume.is_finite() {
                return Err(Self::non_finite(node_class, "dependency_runon_volume", volume));
            }
            if volume < 0.0 {
                return Err(Self::domain_violation(
                    node_class,
                    "dependency_runon_volume",
                    volume,
                ));
            }
            dependency_peak += peak;
            dependency_volume_m3 += volume;
            dependency_duration_s = dependency_duration_s.max(duration);
        }

        let incoming_peak = hillslope_peak + dependency_peak;
        if !incoming_peak.is_finite() {
            return Err(Self::non_finite(node_class, "incoming_peak", incoming_peak));
        }
        if incoming_peak < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                "incoming_peak",
                incoming_peak,
            ));
        }
        let incoming_duration = hillslope_duration_s.max(dependency_duration_s);
        if !incoming_duration.is_finite() {
            return Err(Self::non_finite(
                node_class,
                "incoming_duration",
                incoming_duration,
            ));
        }
        if incoming_duration < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                "incoming_duration",
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
            hourly_resolved,
            hourly_sediment_inlet_kg,
        })
    }

    fn assemble_direct_incoming_peak_and_duration(
        input: &DirectWatershedKernelInput<'_>,
        node_class: Ws10NodeClass,
    ) -> Result<(f64, f64), Ws10GuardError> {
        let partition = Self::assemble_direct_incoming_peak_partition(input, node_class)?;
        Ok((
            partition.hillslope_peak_cms + partition.dependency_peak_cms,
            partition
                .hillslope_duration_s
                .max(partition.dependency_duration_s),
        ))
    }

    fn assemble_direct_channel_baseflow(
        input: &DirectWatershedKernelInput<'_>,
        node_class: Ws10NodeClass,
        dtchr: f64,
        cbase: f64,
        nchnum: f64,
        conductivity: f64,
    ) -> Result<Ws10ChannelBaseflowPartition, Ws10GuardError> {
        match input.frame.routing_globals.groundwater_baseflow {
            WatershedGroundwaterRoutingAuthority::Disabled => {
                let generated = Self::generated_groundwater_from_step(input, node_class)?;
                if generated.volume_m3 > WS10_ZERO_THRESHOLD
                    || generated.deep_seepage_m3 > WS10_ZERO_THRESHOLD
                {
                    return Err(Self::domain_violation(
                        node_class,
                        "groundwater_without_gwcoeff_authority",
                        generated.volume_m3.max(generated.deep_seepage_m3),
                    ));
                }
                let baseflow_peak = cbase * nchnum * (1.0 + conductivity * dtchr);
                if !baseflow_peak.is_finite() || baseflow_peak < 0.0 {
                    return Err(Self::domain_violation(
                        node_class,
                        "baseflow_peak",
                        baseflow_peak,
                    ));
                }
                Ok(Ws10ChannelBaseflowPartition {
                    peak_m3_s: baseflow_peak,
                    volume_m3: baseflow_peak * dtchr,
                    deep_seepage_m3: 0.0,
                })
            }
            WatershedGroundwaterRoutingAuthority::LinearReservoir {
                baseflow_threshold_area_ha,
            } => {
                let generated = Self::generated_groundwater_from_step(input, node_class)?;
                let side_area_ha = Self::contributor_area_ha(input, node_class)?;
                let side_baseflow_m3 = if side_area_ha >= baseflow_threshold_area_ha {
                    generated.volume_m3
                } else {
                    0.0
                };
                let dependency_baseflow_m3 =
                    Self::dependency_channel_baseflow_m3(input, node_class)?;
                let volume_m3 = side_baseflow_m3 + dependency_baseflow_m3;
                let peak_m3_s = volume_m3 / 86_400.0;
                if !peak_m3_s.is_finite() || peak_m3_s < 0.0 {
                    return Err(Self::domain_violation(
                        node_class,
                        "generated_baseflow_peak",
                        peak_m3_s,
                    ));
                }
                Ok(Ws10ChannelBaseflowPartition {
                    peak_m3_s,
                    volume_m3,
                    deep_seepage_m3: generated.deep_seepage_m3
                        + Self::dependency_channel_deep_seepage_m3(input, node_class)?,
                })
            }
        }
    }

    fn generated_groundwater_from_step(
        input: &DirectWatershedKernelInput<'_>,
        node_class: Ws10NodeClass,
    ) -> Result<Ws10ChannelBaseflowPartition, Ws10GuardError> {
        let mut volume_m3 = 0.0_f64;
        let mut deep_seepage_m3 = 0.0_f64;
        for &hillslope_id in &input.step.contributor_hillslopes {
            let contribution = input
                .frame
                .hillslope_contributions
                .get(&hillslope_id)
                .ok_or_else(|| Self::missing_required(node_class, "hillslope_contribution"))?;
            Self::direct_require_range(
                node_class,
                "hillslope_generated_baseflow_m3",
                contribution.generated_baseflow_m3,
                Some(0.0),
                None,
            )?;
            Self::direct_require_range(
                node_class,
                "hillslope_groundwater_deep_seepage_m3",
                contribution.groundwater_deep_seepage_m3,
                Some(0.0),
                None,
            )?;
            volume_m3 += contribution.generated_baseflow_m3;
            deep_seepage_m3 += contribution.groundwater_deep_seepage_m3;
        }
        Ok(Ws10ChannelBaseflowPartition {
            peak_m3_s: 0.0,
            volume_m3,
            deep_seepage_m3,
        })
    }

    fn contributor_area_ha(
        input: &DirectWatershedKernelInput<'_>,
        node_class: Ws10NodeClass,
    ) -> Result<f64, Ws10GuardError> {
        let mut area_m2 = 0.0_f64;
        for &hillslope_id in &input.step.contributor_hillslopes {
            let contribution = input
                .frame
                .hillslope_contributions
                .get(&hillslope_id)
                .ok_or_else(|| Self::missing_required(node_class, "hillslope_contribution"))?;
            let Some(contribution_area_m2) = contribution.area_m2 else {
                return Err(Self::missing_required(node_class, "hillslope_area_m2"));
            };
            Self::direct_require_range(
                node_class,
                "hillslope_area_m2",
                contribution_area_m2,
                Some(WS10_ZERO_THRESHOLD),
                None,
            )?;
            area_m2 += contribution_area_m2;
        }
        Ok(area_m2 / 10_000.0)
    }

    fn dependency_channel_baseflow_m3(
        input: &DirectWatershedKernelInput<'_>,
        node_class: Ws10NodeClass,
    ) -> Result<f64, Ws10GuardError> {
        let mut baseflow_m3 = 0.0_f64;
        for dependency in &input.step.dependency_nodes {
            let state = input
                .frame
                .routed_channels
                .get(&dependency.id)
                .ok_or_else(|| Self::missing_required(node_class, "dependency_channel_state"))?;
            baseflow_m3 += state.channel_baseflow_m3;
        }
        Ok(baseflow_m3)
    }

    fn dependency_channel_deep_seepage_m3(
        input: &DirectWatershedKernelInput<'_>,
        node_class: Ws10NodeClass,
    ) -> Result<f64, Ws10GuardError> {
        let mut deep_seepage_m3 = 0.0_f64;
        for dependency in &input.step.dependency_nodes {
            let state = input
                .frame
                .routed_channels
                .get(&dependency.id)
                .ok_or_else(|| Self::missing_required(node_class, "dependency_channel_state"))?;
            deep_seepage_m3 += state.groundwater_deep_seepage_m3;
        }
        Ok(deep_seepage_m3)
    }

    /// INV-ROUTE-005(a) eligibility: every contributor carries the paired
    /// minor-1 24-slot hourly surfaces.
    #[cfg(test)]
    pub(crate) fn hourly_pair_carried_by_all(
        contributions: &std::collections::BTreeMap<u32, HillslopeContribution>,
        contributor_ids: &[u32],
    ) -> bool {
        !contributor_ids.is_empty()
            && contributor_ids.iter().all(|id| {
                contributions.get(id).is_some_and(|contribution| {
                    contribution.hourly_runoff_volume_m3.len() == 24
                        && contribution.hourly_sediment_mass_kg.len() == 24
                })
            })
    }

    fn direct_hillslope_hourly_authority(
        input: &DirectWatershedKernelInput<'_>,
        node_class: Ws10NodeClass,
    ) -> Result<bool, Ws10GuardError> {
        let mut hourly_count = 0_usize;
        let mut no_hourly_count = 0_usize;

        for &hillslope_id in &input.step.contributor_hillslopes {
            let contribution = input
                .frame
                .hillslope_contributions
                .get(&hillslope_id)
                .ok_or_else(|| Self::missing_required(node_class, "hillslope_contribution"))?;
            match (
                contribution.hourly_runoff_volume_m3.len(),
                contribution.hourly_sediment_mass_kg.len(),
            ) {
                (0, 0) => no_hourly_count += 1,
                (24, 24) => hourly_count += 1,
                (runoff_len, sediment_len) => {
                    let observed_len = runoff_len.max(sediment_len);
                    return Err(Self::domain_violation(
                        node_class,
                        "hillslope_hourly_pair_cardinality",
                        f64::from(u32::try_from(observed_len).unwrap_or(u32::MAX)),
                    ));
                }
            }
        }

        if hourly_count > 0 && no_hourly_count > 0 {
            return Err(Self::domain_violation(
                node_class,
                "hillslope_hourly_pair_mixed_authority",
                f64::from(u32::try_from(no_hourly_count).unwrap_or(u32::MAX)),
            ));
        }

        Ok(hourly_count > 0)
    }

    /// INV-ROUTE-005(a): the superposed modeled hydrograph — inlet peak =
    /// the maximum hour-mean discharge (`max_h(Σ V_h) / 3600 s`), volume =
    /// the exact hour-integral, time base = the active-hour span of the
    /// summed shape (0 when no hour is active).
    pub(crate) fn superposed_hourly_limb(
        summed_hourly_volume_m3: &[f64; 24],
    ) -> (f64, f64, f64) {
        let mut first_active_hour: Option<usize> = None;
        let mut last_active_hour: Option<usize> = None;
        for (hour, volume_m3) in summed_hourly_volume_m3.iter().enumerate() {
            if *volume_m3 > 0.0 {
                if first_active_hour.is_none() {
                    first_active_hour = Some(hour);
                }
                last_active_hour = Some(hour);
            }
        }
        let volume_m3: f64 = summed_hourly_volume_m3.iter().sum();
        let peak_cms = summed_hourly_volume_m3
            .iter()
            .fold(0.0_f64, |acc, slot| acc.max(*slot))
            / 3600.0;
        let duration_s = match (first_active_hour, last_active_hour) {
            // Span is bounded by the 24-slot base; u32 conversion is lossless.
            #[allow(clippy::cast_precision_loss)]
            (Some(first), Some(last)) => ((last - first + 1) as f64) * 3600.0,
            _ => 0.0,
        };
        (peak_cms, volume_m3, duration_s)
    }

    fn read_direct_hillslope_peak_payload(
        contribution: &HillslopeContribution,
        node_class: Ws10NodeClass,
    ) -> Result<(f64, f64), Ws10GuardError> {
        let peak = contribution.peak_runoff_m3_s;
        let duration = contribution.duration_seconds;
        Self::direct_require_range(node_class, "hillslope_peak", peak, Some(0.0), None)?;
        Self::direct_require_range(node_class, "hillslope_duration", duration, Some(0.0), None)?;
        Ok((peak, duration))
    }

    #[allow(clippy::too_many_lines)]
    fn read_direct_hillslope_sediment_payload(
        contribution: &HillslopeContribution,
        node_class: Ws10NodeClass,
    ) -> Result<Ws18HillslopeSedimentPayload, Ws10GuardError> {
        let total_detachment = contribution.total_detachment_kg;
        let total_deposition = contribution.total_deposition_kg;
        let class_count = contribution.particle_class_count();
        Self::direct_require_range(
            node_class,
            "total_detachment",
            total_detachment,
            Some(0.0),
            None,
        )?;
        Self::direct_require_range(
            node_class,
            "total_deposition",
            total_deposition,
            Some(0.0),
            None,
        )?;
        if class_count == 0 {
            return Err(Self::domain_violation(node_class, "particle_class_count", 0.0));
        }
        if contribution.particle_diameter_m.len() != class_count
            || contribution.particle_flow_fraction.len() != class_count
        {
            return Err(Self::domain_violation(
                node_class,
                "particle_class_cardinality",
                f64::from(u32::try_from(class_count).unwrap_or(u32::MAX)),
            ));
        }

        let mut fractions = Vec::with_capacity(class_count);
        let mut particle_diameters_m = Vec::with_capacity(class_count);
        let mut concentration_sum = 0.0_f64;
        let mut fraction_sum = 0.0_f64;

        for class_offset in 0..class_count {
            let concentration = contribution.sediment_concentration_kg_m3[class_offset];
            let particle_diameter = contribution.particle_diameter_m[class_offset];
            let fraction = contribution.particle_flow_fraction[class_offset];

            Self::direct_require_range(node_class, "concentration", concentration, Some(0.0), None)?;
            Self::direct_require_range(
                node_class,
                "particle_diameter",
                particle_diameter,
                Some(WS10_ZERO_THRESHOLD),
                None,
            )?;
            Self::direct_require_range(node_class, "particle_fraction", fraction, Some(0.0), Some(1.0))?;
            fractions.push(fraction);
            particle_diameters_m.push(particle_diameter);
            concentration_sum += concentration;
            fraction_sum += fraction;
        }

        // INV-ROUTE-005(a) / ADR-0036 D3: when the contribution carries the
        // serialized minor-1 hourly sediment surface, that surface IS the
        // sediment authority (mass = Σ S_h) — never a reconstruction from
        // the event aggregates. Minor-0 contributions keep the aggregate
        // basis (the labeled fallback scope).
        let mass_kg = if contribution.hourly_sediment_mass_kg.len() == 24 {
            let mut hourly_mass_kg = 0.0_f64;
            for slot_kg in &contribution.hourly_sediment_mass_kg {
                if !slot_kg.is_finite() || *slot_kg < 0.0 {
                    return Err(Self::domain_violation(
                        node_class,
                        "hourly_sediment_mass_kg",
                        *slot_kg,
                    ));
                }
                hourly_mass_kg += slot_kg;
            }
            hourly_mass_kg
        } else {
            (total_detachment - total_deposition).max(0.0)
        };
        if fraction_sum <= WS10_ZERO_THRESHOLD
            && (mass_kg > WS10_ZERO_THRESHOLD || concentration_sum > WS10_ZERO_THRESHOLD)
        {
            return Err(Self::domain_violation(
                node_class,
                "particle_flow_fraction_sum",
                fraction_sum,
            ));
        }

        Ok(Ws18HillslopeSedimentPayload {
            mass_kg,
            fractions,
            particle_diameters_m,
        })
    }

    fn read_direct_dependency_peak_payload(
        frame: &WatershedNetworkFrame,
        node_class: Ws10NodeClass,
        dependency: TopologyNodeKey,
    ) -> Result<(f64, f64), Ws10GuardError> {
        match dependency.kind {
            TopologyNodeKind::Channel => {
                let state = frame
                    .routed_channels
                    .get(&dependency.id)
                    .ok_or_else(|| Self::missing_required(node_class, "dependency_channel"))?;
                Self::direct_require_range(
                    node_class,
                    "dependency_channel_peak",
                    state.peak_discharge_m3_s,
                    Some(0.0),
                    None,
                )?;
                Self::direct_require_range(
                    node_class,
                    "dependency_channel_duration",
                    state.duration_seconds,
                    Some(0.0),
                    None,
                )?;
                Ok((state.peak_discharge_m3_s, state.duration_seconds))
            }
            TopologyNodeKind::Impoundment => {
                let state = frame
                    .routed_impoundments
                    .get(&dependency.id)
                    .ok_or_else(|| Self::missing_required(node_class, "dependency_impoundment"))?;
                Self::direct_require_range(
                    node_class,
                    "dependency_impoundment_peak",
                    state.outflow_rate_m3_s,
                    Some(0.0),
                    None,
                )?;
                Self::direct_require_range(
                    node_class,
                    "dependency_impoundment_duration",
                    state.duration_seconds,
                    Some(0.0),
                    None,
                )?;
                Ok((state.outflow_rate_m3_s, state.duration_seconds))
            }
            TopologyNodeKind::Hillslope => Err(Self::domain_violation(
                node_class,
                "dependency_node_kind",
                -1.0,
            )),
        }
    }

    fn read_direct_channel_sediment_payload(
        state: &RoutedChannelState,
        node_class: Ws10NodeClass,
        event_duration: f64,
    ) -> Result<Ws18HillslopeSedimentPayload, Ws10GuardError> {
        let qsed = state.sediment_state.qsed_kg_s;
        Self::direct_require_range(node_class, "dependency_qsed", qsed, Some(0.0), None)?;
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
                "dependency_incoming_mass",
                mass_kg,
            ));
        }

        let class_count = state.sediment_state.particle_flow_fraction.len();
        if class_count == 0 || state.sediment_state.particle_diameter_m.len() != class_count {
            return Err(Self::domain_violation(
                node_class,
                "dependency_particle_class_count",
                f64::from(u32::try_from(class_count).unwrap_or(u32::MAX)),
            ));
        }

        let mut fraction_sum = 0.0_f64;
        for class_offset in 0..class_count {
            let fraction = state.sediment_state.particle_flow_fraction[class_offset];
            let particle_diameter = state.sediment_state.particle_diameter_m[class_offset];
            Self::direct_require_range(
                node_class,
                "dependency_particle_fraction",
                fraction,
                Some(0.0),
                Some(1.0),
            )?;
            Self::direct_require_range(
                node_class,
                "dependency_particle_diameter",
                particle_diameter,
                Some(WS10_ZERO_THRESHOLD),
                None,
            )?;
            fraction_sum += fraction;
        }
        if fraction_sum <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                "dependency_particle_fraction_sum",
                fraction_sum,
            ));
        }

        Ok(Ws18HillslopeSedimentPayload {
            mass_kg,
            fractions: state.sediment_state.particle_flow_fraction.clone(),
            particle_diameters_m: state.sediment_state.particle_diameter_m.clone(),
        })
    }

    #[allow(
        clippy::similar_names,
        clippy::too_many_lines,
        clippy::too_many_arguments
    )]
    fn assemble_direct_incoming_sediment_load_and_capacity(
        input: &DirectWatershedKernelInput<'_>,
        control: &WatershedChannelControlRecord,
        node_class: Ws10NodeClass,
        event_duration: f64,
        qpo: f64,
        roughness: f64,
        sediment_controls: Ws15ChannelSedimentControls,
        nslpts: usize,
        peak_partition: Ws20IncomingPeakPartition,
    ) -> Result<Ws19ChannelSedimentPublication, Ws10GuardError> {
        if !event_duration.is_finite() || event_duration <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                "event_duration",
                event_duration,
            ));
        }
        Self::direct_require_range(node_class, "qpo", qpo, Some(0.0), None)?;
        Self::direct_require_range(
            node_class,
            "roughness",
            roughness,
            Some(WS10_ZERO_THRESHOLD),
            None,
        )?;

        let mut incoming_sediment_mass_kg = 0.0_f64;
        let mut class_mass_kg: Vec<f64> = Vec::new();
        let mut class_diameter_mass_m: Vec<f64> = Vec::new();
        let mut top_class_mass_kg: Vec<f64> = Vec::new();
        let mut lateral_class_mass_kg: Vec<f64> = Vec::new();

        for &hillslope_id in &input.step.contributor_hillslopes {
            let contribution = input
                .frame
                .hillslope_contributions
                .get(&hillslope_id)
                .ok_or_else(|| Self::missing_required(node_class, "hillslope_contribution"))?;
            let payload = Self::read_direct_hillslope_sediment_payload(contribution, node_class)?;
            incoming_sediment_mass_kg += payload.mass_kg;
            if payload.mass_kg <= WS10_ZERO_THRESHOLD {
                continue;
            }

            let fraction_sum = payload.fractions.iter().sum::<f64>();
            if !fraction_sum.is_finite() || fraction_sum <= WS10_ZERO_THRESHOLD {
                return Err(Self::domain_violation(
                    node_class,
                    "hillslope_particle_fraction_sum",
                    fraction_sum,
                ));
            }

            for class_offset in 0..payload.fractions.len() {
                if class_mass_kg.len() <= class_offset {
                    class_mass_kg.resize(class_offset + 1, 0.0);
                    class_diameter_mass_m.resize(class_offset + 1, 0.0);
                    top_class_mass_kg.resize(class_offset + 1, 0.0);
                    lateral_class_mass_kg.resize(class_offset + 1, 0.0);
                }

                let normalized_fraction = payload.fractions[class_offset] / fraction_sum;
                let class_mass = payload.mass_kg * normalized_fraction;
                class_mass_kg[class_offset] += class_mass;
                class_diameter_mass_m[class_offset] +=
                    class_mass * payload.particle_diameters_m[class_offset];
                lateral_class_mass_kg[class_offset] += class_mass;
            }
        }

        for dependency in &input.step.dependency_nodes {
            if dependency.kind != TopologyNodeKind::Channel {
                continue;
            }
            let state = input
                .frame
                .routed_channels
                .get(&dependency.id)
                .ok_or_else(|| Self::missing_required(node_class, "dependency_channel"))?;
            let payload =
                Self::read_direct_channel_sediment_payload(state, node_class, event_duration)?;
            incoming_sediment_mass_kg += payload.mass_kg;
            if payload.mass_kg <= WS10_ZERO_THRESHOLD {
                continue;
            }

            let fraction_sum = payload.fractions.iter().sum::<f64>();
            if !fraction_sum.is_finite() || fraction_sum <= WS10_ZERO_THRESHOLD {
                return Err(Self::domain_violation(
                    node_class,
                    "dependency_particle_fraction_sum",
                    fraction_sum,
                ));
            }

            for class_offset in 0..payload.fractions.len() {
                if class_mass_kg.len() <= class_offset {
                    class_mass_kg.resize(class_offset + 1, 0.0);
                    class_diameter_mass_m.resize(class_offset + 1, 0.0);
                    top_class_mass_kg.resize(class_offset + 1, 0.0);
                    lateral_class_mass_kg.resize(class_offset + 1, 0.0);
                }

                let normalized_fraction = payload.fractions[class_offset] / fraction_sum;
                let class_mass = payload.mass_kg * normalized_fraction;
                class_mass_kg[class_offset] += class_mass;
                class_diameter_mass_m[class_offset] +=
                    class_mass * payload.particle_diameters_m[class_offset];
                top_class_mass_kg[class_offset] += class_mass;
            }
        }

        if !incoming_sediment_mass_kg.is_finite() || incoming_sediment_mass_kg < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                "incoming_sediment_mass_kg",
                incoming_sediment_mass_kg,
            ));
        }

        let class_mass_total = class_mass_kg.iter().copied().sum::<f64>();
        let mut active_class_mass_kg = Vec::new();
        let mut active_top_class_mass_kg = Vec::new();
        let mut active_lateral_class_mass_kg = Vec::new();
        let mut active_particle_diameters_m = Vec::new();
        let mut active_class_numbers = Vec::new();
        if class_mass_total > WS10_ZERO_THRESHOLD {
            for class_offset in 0..class_mass_kg.len() {
                let class_mass = class_mass_kg[class_offset];
                if class_mass <= WS10_ZERO_THRESHOLD {
                    continue;
                }

                let class_diameter_m = class_diameter_mass_m[class_offset] / class_mass;
                if !class_diameter_m.is_finite() || class_diameter_m <= WS10_ZERO_THRESHOLD {
                    return Err(Self::domain_violation(
                        node_class,
                        "class_diameter_m",
                        class_diameter_m,
                    ));
                }

                active_class_mass_kg.push(class_mass);
                active_top_class_mass_kg.push(*top_class_mass_kg.get(class_offset).unwrap_or(&0.0));
                active_lateral_class_mass_kg
                    .push(*lateral_class_mass_kg.get(class_offset).unwrap_or(&0.0));
                active_particle_diameters_m.push(class_diameter_m);
                active_class_numbers.push(class_offset + 1);
            }
        }

        let mut outgoing_class_mass_kg = active_class_mass_kg.clone();
        let mut ws20_diagnostics = Ws20SegmentRoutingDiagnostics::default();
        let mut ws29_widb_points_ft: Option<Vec<f64>> = None;
        let mut ws31_wida_points_ft: Option<Vec<f64>> = None;
        let ws20_case12_enabled = control.ws20_case12_enabled;
        let ws21_case34_enabled = ws20_case12_enabled || control.ws21_case34_enabled;

        if ws20_case12_enabled
            && qpo > WS10_ZERO_THRESHOLD
            && incoming_sediment_mass_kg > WS10_ZERO_THRESHOLD
            && !active_class_mass_kg.is_empty()
        {
            let profile = Self::direct_ws20_channel_profile(control, nslpts)?;
            let crfrac = if ws21_case34_enabled {
                Some(Self::direct_ws20_crfrac(control, &active_class_numbers)?)
            } else {
                None
            };
            let routing_result = Self::ws20_route_case12_segment_family_core(
                control.node_id,
                node_class,
                ws21_case34_enabled,
                event_duration,
                qpo,
                roughness,
                sediment_controls,
                nslpts,
                peak_partition,
                &active_top_class_mass_kg,
                &active_lateral_class_mass_kg,
                &active_particle_diameters_m,
                &active_class_numbers,
                profile,
                control.chnk,
                crfrac.as_deref(),
            )?;
            outgoing_class_mass_kg = routing_result.routed_class_masses_kg;
            ws20_diagnostics = routing_result.diagnostics;
            ws29_widb_points_ft = Some(routing_result.widb_points_ft);
            ws31_wida_points_ft = Some(routing_result.wida_points_ft);
        }

        // INV-ROUTE-005(a): on an hourly-resolved inlet the quasi-steady
        // sediment-rate time base is the superposed S_h active span — the
        // serialized sediment TIMING, not the event duration (the
        // single-rate reduction itself is the labeled SC-ROUTE-001 scope
        // limit until channels carry hourly surfaces).
        let sediment_rate_duration_s = if peak_partition.hourly_resolved {
            let (_, _, span_s) =
                Self::superposed_hourly_limb(&peak_partition.hourly_sediment_inlet_kg);
            if span_s > WS10_ZERO_THRESHOLD {
                span_s
            } else {
                event_duration
            }
        } else {
            event_duration
        };
        let qsed =
            outgoing_class_mass_kg.iter().copied().sum::<f64>() / sediment_rate_duration_s;
        if !qsed.is_finite() || qsed < 0.0 {
            return Err(Self::domain_violation(node_class, "qsed", qsed));
        }

        let mut particle_flow_fractions = Vec::new();
        let mut particle_diameters_m = Vec::new();
        let routed_class_total = outgoing_class_mass_kg.iter().copied().sum::<f64>();
        if routed_class_total > WS10_ZERO_THRESHOLD {
            for class_offset in 0..outgoing_class_mass_kg.len() {
                let class_mass = outgoing_class_mass_kg[class_offset];
                if class_mass <= WS10_ZERO_THRESHOLD {
                    continue;
                }
                particle_flow_fractions.push(class_mass / routed_class_total);
                particle_diameters_m.push(active_particle_diameters_m[class_offset]);
            }

            let published_fraction_sum = particle_flow_fractions.iter().copied().sum::<f64>();
            if !published_fraction_sum.is_finite() || published_fraction_sum <= WS10_ZERO_THRESHOLD
            {
                return Err(Self::domain_violation(
                    node_class,
                    "published_fraction_sum",
                    published_fraction_sum,
                ));
            }
            for fraction in &mut particle_flow_fractions {
                *fraction /= published_fraction_sum;
            }
        }

        if qpo <= WS10_ZERO_THRESHOLD || incoming_sediment_mass_kg <= WS10_ZERO_THRESHOLD {
            return Ok(Ws19ChannelSedimentPublication {
                qsed,
                tc: 0.0,
                particle_flow_fractions,
                particle_diameters_m,
                ws29_widb_points_ft,
                ws31_wida_points_ft,
                ws20_case1_segments: ws20_diagnostics.case1_segments,
                ws20_case2_segments: ws20_diagnostics.case2_segments,
                ws24_case2_detach_segments: ws20_diagnostics.ws24_case2_detach_segments,
                ws21_case3_segments: ws20_diagnostics.case3_segments,
                ws21_case4_segments: ws20_diagnostics.case4_segments,
                ws21_enddet_segments: ws20_diagnostics.enddet_segments,
            });
        }

        let terminal_point = control
            .segment_points
            .get(nslpts - 1)
            .ok_or_else(|| Self::missing_required(node_class, "terminal_segment"))?;
        Self::direct_require_range(
            node_class,
            "terminal_slope",
            terminal_point.slope,
            Some(WS18_MIN_CHANNEL_SLOPE),
            None,
        )?;
        Self::direct_require_range(
            node_class,
            "terminal_width",
            terminal_point.width_b_ft,
            Some(WS10_ZERO_THRESHOLD),
            None,
        )?;
        Self::direct_require_range(
            node_class,
            "terminal_depth",
            terminal_point.depth_b_ft,
            Some(0.0),
            None,
        )?;

        let q_cfs = qpo * WS18_CFS_PER_CMS;
        if !q_cfs.is_finite() || q_cfs < 0.0 {
            return Err(Self::domain_violation(node_class, "q_cfs", q_cfs));
        }

        let flagct =
            Self::ws30_shape_flag_from_ishape(node_class, control.node_id, sediment_controls.ishape)?;
        let flagc =
            Self::ws30_apply_erodible_rectangular_fallback(flagct, terminal_point.depth_b_ft);
        let crsh = sediment_controls.chntcr * WS15_CRSH_FROM_CHNTCR_SCALE;
        let (flow_width_ft, effsh) = Self::ws18_hydchn(
            node_class,
            flagc,
            q_cfs,
            terminal_point.slope,
            sediment_controls.ctlz,
            sediment_controls.chnz,
            terminal_point.width_b_ft,
            roughness,
            crsh,
            sediment_controls.chnnbr,
        )?;
        if flow_width_ft <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                "flow_width_ft",
                flow_width_ft,
            ));
        }

        let mut qs = Vec::new();
        let mut crdia_ft = Vec::new();
        let mut crspg = Vec::new();
        for class_offset in 0..class_mass_kg.len() {
            let class_mass = class_mass_kg[class_offset];
            if class_mass <= WS10_ZERO_THRESHOLD {
                continue;
            }
            let class_diameter_m = class_diameter_mass_m[class_offset] / class_mass;
            if !class_diameter_m.is_finite() || class_diameter_m <= WS10_ZERO_THRESHOLD {
                return Err(Self::domain_violation(
                    node_class,
                    "capacity_class_diameter_m",
                    class_diameter_m,
                ));
            }
            let class_load_lbs_per_s = class_mass * WS18_LBS_PER_KG / event_duration;
            if !class_load_lbs_per_s.is_finite() || class_load_lbs_per_s < 0.0 {
                return Err(Self::domain_violation(
                    node_class,
                    "class_load_lbs_s",
                    class_load_lbs_per_s,
                ));
            }

            qs.push(class_load_lbs_per_s / flow_width_ft);
            crdia_ft.push(class_diameter_m * WS15_DEPTH_FROM_METERS_TO_FEET);
            let specific_gravity =
                WS18_DEFAULT_CRSPG
                    .get(class_offset)
                    .copied()
                    .ok_or_else(|| {
                        Self::domain_violation(
                            node_class,
                            "particle_class_index",
                            f64::from(u32::try_from(class_offset + 1).unwrap_or(u32::MAX)),
                        )
                    })?;
            crspg.push(specific_gravity);
        }

        let tc_per_width = Self::ws18_trncap(effsh, &qs, &crdia_ft, &crspg);
        let tc_lbs_per_s = tc_per_width.iter().copied().sum::<f64>() * flow_width_ft;
        let tc = tc_lbs_per_s / WS18_LBS_PER_KG;
        if !tc.is_finite() || tc < 0.0 {
            return Err(Self::domain_violation(node_class, "tc", tc));
        }

        Ok(Ws19ChannelSedimentPublication {
            qsed,
            tc,
            particle_flow_fractions,
            particle_diameters_m,
            ws29_widb_points_ft,
            ws31_wida_points_ft,
            ws20_case1_segments: ws20_diagnostics.case1_segments,
            ws20_case2_segments: ws20_diagnostics.case2_segments,
            ws24_case2_detach_segments: ws20_diagnostics.ws24_case2_detach_segments,
            ws21_case3_segments: ws20_diagnostics.case3_segments,
            ws21_case4_segments: ws20_diagnostics.case4_segments,
            ws21_enddet_segments: ws20_diagnostics.enddet_segments,
        })
    }

    fn direct_ws20_channel_profile(
        control: &WatershedChannelControlRecord,
        nslpts: usize,
    ) -> Result<Ws20ChannelProfile, Ws10GuardError> {
        let node_class = Ws10NodeClass::Channel;
        let mut profile = Ws20ChannelProfile {
            x_points_ft: Vec::with_capacity(nslpts),
            slopes: Vec::with_capacity(nslpts),
            depth_a_points_ft: Vec::with_capacity(nslpts),
            depth_b_points_ft: Vec::with_capacity(nslpts),
            width_a_points_ft: Vec::with_capacity(nslpts),
            width_b_points_ft: Vec::with_capacity(nslpts),
        };

        for point in &control.segment_points {
            Self::direct_require_range(node_class, "ws20_x", point.x_m, Some(0.0), None)?;
            Self::direct_require_range(node_class, "ws20_slope", point.slope, Some(0.0), None)?;
            Self::direct_require_range(node_class, "ws20_depa", point.depth_a_ft, Some(0.0), None)?;
            Self::direct_require_range(node_class, "ws20_depb", point.depth_b_ft, Some(0.0), None)?;
            Self::direct_require_range(
                node_class,
                "ws20_wida",
                point.width_a_ft,
                Some(WS10_ZERO_THRESHOLD),
                None,
            )?;
            Self::direct_require_range(
                node_class,
                "ws20_widb",
                point.width_b_ft,
                Some(WS10_ZERO_THRESHOLD),
                None,
            )?;
            profile.x_points_ft.push(point.x_m);
            profile.slopes.push(point.slope.max(WS18_MIN_CHANNEL_SLOPE));
            profile.depth_a_points_ft.push(point.depth_a_ft);
            profile.depth_b_points_ft.push(point.depth_b_ft);
            profile.width_a_points_ft.push(point.width_a_ft);
            profile.width_b_points_ft.push(point.width_b_ft);
        }

        Ok(profile)
    }

    fn direct_ws20_crfrac(
        control: &WatershedChannelControlRecord,
        class_numbers: &[usize],
    ) -> Result<Vec<f64>, Ws10GuardError> {
        let node_class = Ws10NodeClass::Channel;
        let mut crfrac = Vec::with_capacity(class_numbers.len());
        for class_number in class_numbers {
            let Some(value) = class_number
                .checked_sub(1)
                .and_then(|offset| control.crfrac.get(offset))
                .copied()
            else {
                return Err(Self::missing_required(node_class, "ws20_crfrac"));
            };
            Self::direct_require_range(node_class, "ws20_crfrac", value, Some(0.0), Some(1.0))?;
            crfrac.push(value);
        }

        let sum = crfrac.iter().copied().sum::<f64>();
        if !sum.is_finite() || sum <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(node_class, "ws20_crfrac_sum", sum));
        }
        for value in &mut crfrac {
            *value /= sum;
        }
        Ok(crfrac)
    }

    fn direct_ws12_impoundment_coefficients(
        control: &WatershedImpoundmentControlRecord,
    ) -> Result<Ws12ImpoundmentCoefficients, Ws10GuardError> {
        let node_class = Ws10NodeClass::Impoundment;
        let node_id = usize::try_from(control.node_id)
            .map_err(|_| Self::domain_violation(node_class, "impoundment_node_id", 0.0))?;
        let coefficients =
            derive_ws12_impoundment_coefficients(node_id, &control.source_record)
                .map_err(|error| Self::runtime_input_guard_error(node_class, &error))?;
        let families = derive_ws12_outflow_function_families(node_id, &control.source_record)
            .map_err(|error| Self::runtime_input_guard_error(node_class, &error))?;

        let mut family_a = [0.0_f64; 15];
        let mut family_b = [0.0_f64; 15];
        let mut family_c = [0.0_f64; 15];
        let mut family_d = [0.0_f64; 15];
        let mut family_e = [0.0_f64; 15];
        let mut family_head_threshold = [0.0_f64; 15];
        for family_index in 1..=15 {
            family_a[family_index - 1] = families.coefficient_at(family_index, "a");
            family_b[family_index - 1] = families.coefficient_at(family_index, "b");
            family_c[family_index - 1] = families.coefficient_at(family_index, "c");
            family_d[family_index - 1] = families.coefficient_at(family_index, "d");
            family_e[family_index - 1] = families.coefficient_at(family_index, "e");
            family_head_threshold[family_index - 1] =
                families.coefficient_at(family_index, "ha");
        }

        let mut a0 = None;
        let mut a1 = None;
        let mut a2 = None;
        for (suffix, value, _, _) in coefficients {
            match suffix {
                "a0" => a0 = Some(value),
                "a1" => a1 = Some(value),
                "a2" => a2 = Some(value),
                _ => {}
            }
        }

        Ok(Ws12ImpoundmentCoefficients {
            a: family_a,
            b: family_b,
            c: family_c,
            d: family_d,
            e: family_e,
            ha: family_head_threshold,
            a0: a0.ok_or_else(|| Self::missing_required(node_class, "a0"))?,
            a1: a1.ok_or_else(|| Self::missing_required(node_class, "a1"))?,
            a2: a2.ok_or_else(|| Self::missing_required(node_class, "a2"))?,
        })
    }

    fn runtime_input_guard_error(
        node_class: Ws10NodeClass,
        error: &WatershedRuntimeInputError,
    ) -> Ws10GuardError {
        match error {
            WatershedRuntimeInputError::ImpoundmentSymbolNonFinite { value, .. } => {
                Self::non_finite(node_class, "runtime_input", *value)
            }
            WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain { value, .. } => {
                Self::domain_violation(node_class, "runtime_input", *value)
            }
        }
    }
}
