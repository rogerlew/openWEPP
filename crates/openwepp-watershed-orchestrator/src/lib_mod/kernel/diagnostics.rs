impl Ws10ChannelImpoundmentKernel {
    fn read_channel_opt_in_toggle(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
        suffix: &'static str,
    ) -> Result<bool, Ws10GuardError> {
        let toggle_symbol = Self::channel_wave_state_symbol(request.node_id, suffix);
        let Some(value) = request.state_surface.get(&toggle_symbol) else {
            return Ok(false);
        };

        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Self::non_finite(node_class, toggle_symbol, scalar));
        }
        if scalar.abs() <= WS11_IPEAK_INTEGER_TOLERANCE {
            return Ok(false);
        }
        if (scalar - 1.0).abs() <= WS11_IPEAK_INTEGER_TOLERANCE {
            return Ok(true);
        }

        Err(Self::domain_violation(node_class, toggle_symbol, scalar))
    }

    #[allow(
        clippy::similar_names,
        clippy::too_many_lines,
        clippy::too_many_arguments
    )]
    fn assemble_incoming_sediment_load_and_capacity(
        request: &WatershedKernelRequest<'_>,
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
                BoundarySymbol::from("event_duration"),
                event_duration,
            ));
        }
        if !qpo.is_finite() || qpo < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("qpo"),
                qpo,
            ));
        }
        if !roughness.is_finite() || roughness <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("roughness"),
                roughness,
            ));
        }

        let mut incoming_sediment_mass_kg = 0.0_f64;
        let mut class_mass_kg: Vec<f64> = Vec::new();
        let mut class_diameter_mass_m: Vec<f64> = Vec::new();
        let mut top_class_mass_kg: Vec<f64> = Vec::new();
        let mut lateral_class_mass_kg: Vec<f64> = Vec::new();
        for &hillslope_id in request.contributor_hillslopes {
            let payload = Self::read_hillslope_sediment_payload(request, node_class, hillslope_id)?;
            incoming_sediment_mass_kg += payload.mass_kg;
            if payload.mass_kg <= WS10_ZERO_THRESHOLD {
                continue;
            }

            let fraction_sum = payload.fractions.iter().sum::<f64>();
            if !fraction_sum.is_finite() || fraction_sum <= WS10_ZERO_THRESHOLD {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from(format!("hs{hillslope_id}_particle_flow_fraction_sum")),
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

        for dependency in &request.dependency_nodes {
            let (dependency_class, dependency_id) = Self::parse_dependency(node_class, dependency)?;
            if dependency_class != Ws10NodeClass::Channel {
                continue;
            }

            let payload = Self::read_channel_sediment_payload(
                request,
                node_class,
                dependency_id,
                event_duration,
            )?;
            incoming_sediment_mass_kg += payload.mass_kg;
            if payload.mass_kg <= WS10_ZERO_THRESHOLD {
                continue;
            }

            let fraction_sum = payload.fractions.iter().sum::<f64>();
            if !fraction_sum.is_finite() || fraction_sum <= WS10_ZERO_THRESHOLD {
                return Err(Self::domain_violation(
                    node_class,
                    Self::channel_wave_state_symbol(dependency_id, "particle_flow_fraction_sum"),
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
                BoundarySymbol::from("incoming_sediment_mass_kg"),
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
                        BoundarySymbol::from(format!(
                            "ws19_class_diameter_m_{:04}",
                            class_offset + 1
                        )),
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
        let ws20_case12_enabled =
            Self::read_channel_opt_in_toggle(request, node_class, "ws20_case12_enable")?;
        let ws21_case34_opt_in =
            Self::read_channel_opt_in_toggle(request, node_class, "ws21_case34_enable")?;
        let ws21_case34_enabled = ws20_case12_enabled || ws21_case34_opt_in;

        if ws20_case12_enabled
            && qpo > WS10_ZERO_THRESHOLD
            && incoming_sediment_mass_kg > WS10_ZERO_THRESHOLD
            && !active_class_mass_kg.is_empty()
        {
            let routing_result = Self::ws20_route_case12_segment_family(
                request,
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
            )?;
            outgoing_class_mass_kg = routing_result.routed_class_masses_kg;
            ws20_diagnostics = routing_result.diagnostics;
            ws29_widb_points_ft = Some(routing_result.widb_points_ft);
            ws31_wida_points_ft = Some(routing_result.wida_points_ft);
        }

        let qsed = outgoing_class_mass_kg.iter().copied().sum::<f64>() / event_duration;
        if !qsed.is_finite() || qsed < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("qsed"),
                qsed,
            ));
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
                    BoundarySymbol::from("ws19_published_fraction_sum"),
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

        let node_id = request.node_id;
        let slope_symbol =
            BoundarySymbol::from(format!("ws10_channel_{node_id}_slope_{nslpts:04}"));
        let width_symbol = BoundarySymbol::from(format!("ws10_channel_{node_id}_widb_{nslpts:04}"));
        let terminal_depth_symbol =
            BoundarySymbol::from(format!("ws10_channel_{node_id}_depb_{nslpts:04}"));

        let terminal_slope =
            Self::require_channel_state_symbol_scalar(request, node_class, slope_symbol.clone())?;
        let terminal_width_ft =
            Self::require_channel_state_symbol_scalar(request, node_class, width_symbol.clone())?;
        let terminal_depth_ft = Self::require_channel_state_symbol_scalar(
            request,
            node_class,
            terminal_depth_symbol.clone(),
        )?;
        Self::require_channel_control_range(
            node_class,
            slope_symbol,
            terminal_slope,
            Some(WS18_MIN_CHANNEL_SLOPE),
            None,
        )?;
        Self::require_channel_control_range(
            node_class,
            width_symbol,
            terminal_width_ft,
            Some(WS10_ZERO_THRESHOLD),
            None,
        )?;
        Self::require_channel_control_range(
            node_class,
            terminal_depth_symbol,
            terminal_depth_ft,
            Some(0.0),
            None,
        )?;

        let q_cfs = qpo * WS18_CFS_PER_CMS;
        if !q_cfs.is_finite() || q_cfs < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws18_q_cfs"),
                q_cfs,
            ));
        }

        let flagct =
            Self::ws30_shape_flag_from_ishape(node_class, node_id, sediment_controls.ishape)?;
        let flagc = Self::ws30_apply_erodible_rectangular_fallback(flagct, terminal_depth_ft);
        let c1 = sediment_controls.ctlz;
        let sf = terminal_slope;
        let crsh = sediment_controls.chntcr * WS15_CRSH_FROM_CHNTCR_SCALE;
        let (flow_width_ft, effsh) = Self::ws18_hydchn(
            node_class,
            flagc,
            q_cfs,
            sf,
            c1,
            sediment_controls.chnz,
            terminal_width_ft,
            roughness,
            crsh,
            sediment_controls.chnnbr,
        )?;
        if flow_width_ft <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws18_flow_width_ft"),
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
                    BoundarySymbol::from(format!("ws18_class_diameter_m_{:04}", class_offset + 1)),
                    class_diameter_m,
                ));
            }
            let class_load_lbs_per_s = class_mass * WS18_LBS_PER_KG / event_duration;
            if !class_load_lbs_per_s.is_finite() || class_load_lbs_per_s < 0.0 {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from(format!("ws18_class_load_lbs_s_{:04}", class_offset + 1)),
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
                        let class_index_u32 = u32::try_from(class_offset + 1).unwrap_or(u32::MAX);
                        Self::domain_violation(
                            node_class,
                            BoundarySymbol::from(format!(
                                "ws18_particle_class_index_{:04}",
                                class_offset + 1
                            )),
                            f64::from(class_index_u32),
                        )
                    })?;
            crspg.push(specific_gravity);
        }

        let tc_per_width = Self::ws18_trncap(effsh, &qs, &crdia_ft, &crspg);
        let tc_lbs_per_s = tc_per_width.iter().copied().sum::<f64>() * flow_width_ft;
        let tc = tc_lbs_per_s / WS18_LBS_PER_KG;
        if !tc.is_finite() || tc < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("tc"),
                tc,
            ));
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

    fn require_ipeak_branch(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
    ) -> Result<Ws11IpeakBranch, Ws10GuardError> {
        let ipeak_symbol = WatershedProductionStateSymbol::Ipeak;
        let ipeak = Self::require_state_scalar(request, node_class, ipeak_symbol)?;
        Self::require_state_range(node_class, ipeak_symbol, ipeak, Some(1.0), None)?;

        let rounded_ipeak = ipeak.round();
        if (ipeak - rounded_ipeak).abs() > WS11_IPEAK_INTEGER_TOLERANCE {
            return Err(Self::domain_violation(node_class, ipeak_symbol, ipeak));
        }

        let branch = if (rounded_ipeak - 1.0).abs() <= WS11_IPEAK_INTEGER_TOLERANCE {
            Ws11IpeakBranch::Rational
        } else if (rounded_ipeak - 2.0).abs() <= WS11_IPEAK_INTEGER_TOLERANCE {
            Ws11IpeakBranch::Creams
        } else if (rounded_ipeak - 3.0).abs() <= WS11_IPEAK_INTEGER_TOLERANCE {
            Ws11IpeakBranch::KinematicWave
        } else if (rounded_ipeak - 5.0).abs() <= WS11_IPEAK_INTEGER_TOLERANCE {
            Ws11IpeakBranch::MuskingumCungeVariable
        } else {
            Ws11IpeakBranch::MuskingumCunge
        };

        Ok(branch)
    }

    fn channel_wave_state_symbol(node_id: u32, suffix: &str) -> BoundarySymbol {
        BoundarySymbol::from(format!("ws10_channel_{node_id}_{suffix}"))
    }

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

    fn optional_channel_wave_state_scalar(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
        node_id: u32,
        suffix: &str,
    ) -> Result<Option<f64>, Ws10GuardError> {
        let symbol = Self::channel_wave_state_symbol(node_id, suffix);
        let Some(value) = request.state_surface.get(&symbol) else {
            return Ok(None);
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Self::non_finite(node_class, symbol, scalar));
        }
        Ok(Some(scalar))
    }

    fn require_ws11_channel_length_from_scaffold(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
        nslpts: usize,
    ) -> Result<f64, Ws10GuardError> {
        let first_x_symbol = Self::channel_wave_state_symbol(request.node_id, "x_0001");
        let last_x_symbol =
            Self::channel_wave_state_symbol(request.node_id, &format!("x_{nslpts:04}"));

        let first_x =
            Self::require_channel_state_symbol_scalar(request, node_class, first_x_symbol.clone())?;
        let last_x =
            Self::require_channel_state_symbol_scalar(request, node_class, last_x_symbol.clone())?;
        let channel_length = last_x - first_x;
        if !channel_length.is_finite() || channel_length <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws11_channel_length"),
                channel_length,
            ));
        }
        Ok(channel_length)
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
        if !channel_shape.is_finite() || channel_shape <= WS10_ZERO_THRESHOLD {
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
        if qref < WS10_ZERO_THRESHOLD {
            qref = WS10_ZERO_THRESHOLD;
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
