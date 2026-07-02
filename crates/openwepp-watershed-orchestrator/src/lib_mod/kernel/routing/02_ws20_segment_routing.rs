impl Ws10ChannelImpoundmentKernel {
    fn ws20_empty_segment_result() -> Ws20SegmentRoutingResult {
        Ws20SegmentRoutingResult {
            routed_class_masses_kg: Vec::new(),
            diagnostics: Ws20SegmentRoutingDiagnostics::default(),
            widb_points_ft: Vec::new(),
            wida_points_ft: Vec::new(),
        }
    }

    fn ws20_validate_class_cardinality(
        node_class: Ws10NodeClass,
        class_count: usize,
        top_class_mass_kg: &[f64],
        lateral_class_mass_kg: &[f64],
        class_numbers: &[usize],
    ) -> Result<(), Ws10GuardError> {
        if top_class_mass_kg.len() != class_count
            || lateral_class_mass_kg.len() != class_count
            || class_numbers.len() != class_count
        {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws20_class_cardinality"),
                f64::from(u32::try_from(class_count).unwrap_or(u32::MAX)),
            ));
        }
        Ok(())
    }

    fn ws20_effective_length(
        node_class: Ws10NodeClass,
        profile: &Ws20ChannelProfile,
    ) -> Result<f64, Ws10GuardError> {
        let Some(&leff_ft) = profile.x_points_ft.last() else {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws20_effective_length_ft"),
                0.0,
            ));
        };
        if leff_ft <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws20_effective_length_ft"),
                leff_ft,
            ));
        }
        Ok(leff_ft)
    }

    fn ws20_flow_partition(
        node_class: Ws10NodeClass,
        qpo: f64,
        peak_partition: Ws20IncomingPeakPartition,
        leff_ft: f64,
    ) -> Result<(f64, f64), Ws10GuardError> {
        let q_cfs = qpo * WS18_CFS_PER_CMS;
        if !q_cfs.is_finite() || q_cfs < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws20_q_cfs"),
                q_cfs,
            ));
        }

        let peak_sum_cms = peak_partition.hillslope_peak_cms + peak_partition.dependency_peak_cms;
        if !peak_sum_cms.is_finite() || peak_sum_cms < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws20_peak_sum_cms"),
                peak_sum_cms,
            ));
        }
        let top_fraction = if peak_sum_cms > WS10_ZERO_THRESHOLD {
            peak_partition.dependency_peak_cms / peak_sum_cms
        } else {
            0.0
        };
        if !top_fraction.is_finite() || !(0.0..=1.0).contains(&top_fraction) {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws20_top_fraction"),
                top_fraction,
            ));
        }

        let qu_top_cfs = q_cfs * top_fraction;
        let qlat_cfs_per_ft = (q_cfs - qu_top_cfs) / leff_ft;
        if !qlat_cfs_per_ft.is_finite() || qlat_cfs_per_ft < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws20_qlat_cfs_per_ft"),
                qlat_cfs_per_ft,
            ));
        }

        Ok((qu_top_cfs, qlat_cfs_per_ft))
    }

    #[allow(clippy::too_many_arguments)]
    fn ws20_prepare_class_transport(
        node_class: Ws10NodeClass,
        event_duration: f64,
        leff_ft: f64,
        top_class_mass_kg: &[f64],
        lateral_class_mass_kg: &[f64],
        class_diameters_m: &[f64],
        class_numbers: &[usize],
    ) -> Result<Ws20ClassTransportState, Ws10GuardError> {
        let class_count = class_diameters_m.len();
        let mut state = Ws20ClassTransportState {
            gstu_lbs_s: vec![0.0_f64; class_count],
            dlat_lbs_s_ft: vec![0.0_f64; class_count],
            crdia_ft: vec![0.0_f64; class_count],
            crspg: vec![0.0_f64; class_count],
            fall_ft_s: vec![0.0_f64; class_count],
        };

        for class_offset in 0..class_count {
            let class_number = class_numbers[class_offset];
            let specific_gravity = WS18_DEFAULT_CRSPG
                .get(class_number.saturating_sub(1))
                .copied()
                .ok_or_else(|| {
                    Self::domain_violation(
                        node_class,
                        BoundarySymbol::from(format!("ws20_particle_class_{class_number:04}")),
                        f64::from(u32::try_from(class_number).unwrap_or(u32::MAX)),
                    )
                })?;

            let top_flux = top_class_mass_kg[class_offset] * WS18_LBS_PER_KG / event_duration;
            let lateral_flux =
                lateral_class_mass_kg[class_offset] * WS18_LBS_PER_KG / event_duration;
            if !top_flux.is_finite() || top_flux < 0.0 {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from(format!("ws20_top_flux_{class_number:04}")),
                    top_flux,
                ));
            }
            if !lateral_flux.is_finite() || lateral_flux < 0.0 {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from(format!("ws20_lateral_flux_{class_number:04}")),
                    lateral_flux,
                ));
            }

            state.gstu_lbs_s[class_offset] = top_flux;
            state.dlat_lbs_s_ft[class_offset] = lateral_flux / leff_ft;
            state.crdia_ft[class_offset] =
                class_diameters_m[class_offset] * WS15_DEPTH_FROM_METERS_TO_FEET;
            state.crspg[class_offset] = specific_gravity;
            state.fall_ft_s[class_offset] =
                Self::ws20_fall_velocity_ft_s(specific_gravity, state.crdia_ft[class_offset]);
        }

        Ok(state)
    }

    #[allow(clippy::similar_names)]
    fn ws20_segment_hydraulics(
        ctx: &Ws20RouteContext<'_>,
        segment_index: usize,
        profile: &Ws20ChannelProfile,
    ) -> Result<Ws20SegmentHydraulics, Ws10GuardError> {
        let x_upper_ft = profile.x_points_ft[segment_index - 1];
        let x_lower_ft = profile.x_points_ft[segment_index];
        let dx_ft = x_lower_ft - x_upper_ft;
        if dx_ft <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                ctx.node_class,
                BoundarySymbol::from("ws20_dx_ft"),
                dx_ft,
            ));
        }

        let qu_cfs = ctx.qu_top_cfs + (ctx.qlat_cfs_per_ft * x_upper_ft);
        let ql_cfs = ctx.qu_top_cfs + (ctx.qlat_cfs_per_ft * x_lower_ft);
        if !qu_cfs.is_finite() || qu_cfs < 0.0 {
            return Err(Self::domain_violation(
                ctx.node_class,
                BoundarySymbol::from("ws20_qu_cfs"),
                qu_cfs,
            ));
        }
        if !ql_cfs.is_finite() || ql_cfs < 0.0 {
            return Err(Self::domain_violation(
                ctx.node_class,
                BoundarySymbol::from("ws20_ql_cfs"),
                ql_cfs,
            ));
        }

        let upper_width_ft = profile.width_b_points_ft[segment_index - 1];
        let lower_width_ft = profile.width_a_points_ft[segment_index];
        let upper_flagc = Self::ws30_apply_erodible_rectangular_fallback(
            ctx.flagct,
            profile.depth_b_points_ft[segment_index - 1],
        );
        let lower_flagc = Self::ws30_apply_erodible_rectangular_fallback(
            ctx.flagct,
            profile.depth_a_points_ft[segment_index],
        );

        let (mut wfu_ft, mut effshu) = Self::ws18_hydchn(
            ctx.node_class,
            upper_flagc,
            qu_cfs,
            profile.slopes[segment_index - 1],
            ctx.sediment_controls.ctlz,
            ctx.sediment_controls.chnz,
            upper_width_ft,
            ctx.roughness,
            ctx.crsh,
            ctx.sediment_controls.chnnbr,
        )?;
        let (mut wfl_ft, mut effshl) = Self::ws18_hydchn(
            ctx.node_class,
            lower_flagc,
            ql_cfs,
            profile.slopes[segment_index],
            ctx.sediment_controls.ctlz,
            ctx.sediment_controls.chnz,
            lower_width_ft,
            ctx.roughness,
            ctx.crsh,
            ctx.sediment_controls.chnnbr,
        )?;

        if wfu_ft <= WS10_ZERO_THRESHOLD && qu_cfs <= WS10_ZERO_THRESHOLD {
            wfu_ft = upper_width_ft;
            effshu = 0.0;
        }
        if wfl_ft <= WS10_ZERO_THRESHOLD && ql_cfs <= WS10_ZERO_THRESHOLD {
            wfl_ft = lower_width_ft;
            effshl = 0.0;
        }
        if wfu_ft <= WS10_ZERO_THRESHOLD || wfl_ft <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                ctx.node_class,
                BoundarySymbol::from("ws20_width_ft"),
                wfu_ft.min(wfl_ft),
            ));
        }

        Ok(Ws20SegmentHydraulics {
            segment_index,
            x_upper_ft,
            x_lower_ft,
            dx_ft,
            qu_cfs,
            ql_cfs,
            wfu_ft,
            wfl_ft,
            effshu,
            effshl,
            upper_flagc,
            lower_flagc,
        })
    }

    #[allow(clippy::needless_range_loop, clippy::similar_names)]
    fn ws20_transport_snapshot(
        ctx: &Ws20RouteContext<'_>,
        segment: &Ws20SegmentHydraulics,
        state: &Ws20ClassTransportState,
    ) -> Ws20TransportSnapshot {
        let class_count = state.gstu_lbs_s.len();
        let gsu_lbs_s_ft: Vec<f64> = state
            .gstu_lbs_s
            .iter()
            .map(|flux| flux / segment.wfu_ft)
            .collect();
        let tcu_lbs_s_ft =
            Self::ws18_trncap(segment.effshu, &gsu_lbs_s_ft, &state.crdia_ft, &state.crspg);

        let mut potld_lbs_s_ft = vec![0.0_f64; class_count];
        for class_offset in 0..class_count {
            potld_lbs_s_ft[class_offset] = (state.gstu_lbs_s[class_offset]
                + (state.dlat_lbs_s_ft[class_offset] * segment.dx_ft))
                / segment.wfl_ft;
        }
        let tcl_lbs_s_ft = Self::ws18_trncap(
            segment.effshl,
            &potld_lbs_s_ft,
            &state.crdia_ft,
            &state.crspg,
        );

        let mut dtcdx_lbs_s_ft2 = vec![0.0_f64; class_count];
        for class_offset in 0..class_count {
            dtcdx_lbs_s_ft2[class_offset] = ((tcl_lbs_s_ft[class_offset] * segment.wfl_ft)
                - (tcu_lbs_s_ft[class_offset] * segment.wfu_ft))
                / segment.dx_ft;
        }

        let wfa_ft = 0.5 * (segment.wfl_ft + segment.wfu_ft);
        let qtemp_cfs_per_ft = if ctx.qlat_cfs_per_ft > WS10_ZERO_THRESHOLD {
            ctx.qlat_cfs_per_ft
        } else {
            0.0
        };
        let phi: Vec<f64> = if qtemp_cfs_per_ft > 0.0 {
            state
                .fall_ft_s
                .iter()
                .map(|fall| fall * wfa_ft / qtemp_cfs_per_ft)
                .collect()
        } else {
            vec![0.0; class_count]
        };

        let mut excess = 1.0_f64;
        for class_offset in 0..class_count {
            if tcu_lbs_s_ft[class_offset] <= 1.0e-8 {
                excess = 0.0;
                break;
            }
            excess =
                excess.min(1.0 - (gsu_lbs_s_ft[class_offset] / tcu_lbs_s_ft[class_offset]));
        }

        Ws20TransportSnapshot {
            gsu_lbs_s_ft,
            tcu_lbs_s_ft,
            potld_lbs_s_ft,
            tcl_lbs_s_ft,
            dtcdx_lbs_s_ft2,
            phi,
            excess,
        }
    }

    fn ws20_route_one_segment(
        ctx: &Ws20RouteContext<'_>,
        segment_index: usize,
        profile: &mut Ws20ChannelProfile,
        state: &mut Ws20ClassTransportState,
        diagnostics: &mut Ws20SegmentRoutingDiagnostics,
    ) -> Result<(), Ws10GuardError> {
        let segment = Self::ws20_segment_hydraulics(ctx, segment_index, profile)?;
        let snapshot = Self::ws20_transport_snapshot(ctx, &segment, state);
        if snapshot.excess > 0.0 {
            return Self::ws20_route_case34_segment(
                ctx,
                profile,
                state,
                diagnostics,
                &segment,
                &snapshot,
            );
        }

        Self::ws20_route_case12_segment(
            ctx,
            profile,
            state,
            diagnostics,
            &segment,
            &snapshot,
        )
    }

    fn ws20_segment_crfrac<'a>(
        ctx: &'a Ws20RouteContext<'a>,
    ) -> Result<&'a [f64], Ws10GuardError> {
        if !ctx.ws21_case34_enabled {
            return Err(Self::domain_violation(
                ctx.node_class,
                BoundarySymbol::from("ws21_case34_enabled"),
                0.0,
            ));
        }
        ctx.crfrac.ok_or_else(|| {
            Self::missing_required(ctx.node_class, BoundarySymbol::from("ws20_crfrac"))
        })
    }

    #[allow(clippy::needless_range_loop, clippy::similar_names)]
    fn ws20_route_case34_segment(
        ctx: &Ws20RouteContext<'_>,
        profile: &mut Ws20ChannelProfile,
        state: &mut Ws20ClassTransportState,
        diagnostics: &mut Ws20SegmentRoutingDiagnostics,
        segment: &Ws20SegmentHydraulics,
        snapshot: &Ws20TransportSnapshot,
    ) -> Result<(), Ws10GuardError> {
        let crfrac = Self::ws20_segment_crfrac(ctx)?;
        let depsid_ft = ctx.sediment_controls.chneds * WS15_DEPTH_FROM_METERS_TO_FEET;
        let tb_s = 2.0 * ctx.event_duration;
        let depmid_ft = profile.depth_b_points_ft[segment.segment_index - 1];
        let dcap_outcome = Self::ws26_dcap(
            ctx.node_class,
            1,
            segment.qu_cfs,
            profile.slopes[segment.segment_index - 1].max(WS22_DCAP_MIN_SLOPE),
            ctx.sediment_controls.ctlz,
            ctx.sediment_controls.chnz,
            segment.effshu,
            depsid_ft,
            depmid_ft,
            profile.width_b_points_ft[segment.segment_index - 1],
            segment.wfu_ft,
            ctx.roughness,
            ctx.crsh,
            snapshot.excess,
            tb_s,
            segment.upper_flagc,
            ctx.chnk,
            ctx.sediment_controls.chnnbr,
            WS22_DCAP_MAXE,
            crfrac,
        )?;
        let depmid_ft = dcap_outcome.depmid_ft;
        profile.depth_b_points_ft[segment.segment_index - 1] = dcap_outcome.depmid_ft;
        if segment.upper_flagc == 2 && dcap_outcome.werod_ft > segment.wfu_ft {
            profile.width_b_points_ft[segment.segment_index - 1] = dcap_outcome.werod_ft;
        }

        let mut du_lbs_s_ft = vec![0.0_f64; state.gstu_lbs_s.len()];
        for class_offset in 0..state.gstu_lbs_s.len() {
            du_lbs_s_ft[class_offset] = dcap_outcome.df_lbs_s_ft2[class_offset] * segment.wfu_ft;
        }

        let case3_segment = snapshot
            .tcl_lbs_s_ft
            .iter()
            .zip(&snapshot.potld_lbs_s_ft)
            .all(|(tcl, potld)| *tcl <= *potld);

        if case3_segment {
            diagnostics.case3_segments = diagnostics.case3_segments.saturating_add(1);
            state.gstu_lbs_s = Self::ws20_case3_next_fluxes(ctx, segment, state, snapshot, &du_lbs_s_ft)?;
            return Ok(());
        }

        diagnostics.case4_segments = diagnostics.case4_segments.saturating_add(1);
        Self::ws20_route_case4_segment(
            ctx,
            profile,
            state,
            diagnostics,
            segment,
            crfrac,
            &du_lbs_s_ft,
            depsid_ft,
            depmid_ft,
            tb_s,
        )
    }

    #[allow(clippy::needless_range_loop)]
    fn ws20_case3_next_fluxes(
        ctx: &Ws20RouteContext<'_>,
        segment: &Ws20SegmentHydraulics,
        state: &Ws20ClassTransportState,
        snapshot: &Ws20TransportSnapshot,
        du_lbs_s_ft: &[f64],
    ) -> Result<Vec<f64>, Ws10GuardError> {
        let class_count = state.gstu_lbs_s.len();
        let all_detaching = Self::ws20_case3_all_detaching(class_count, snapshot, du_lbs_s_ft);
        let xdbeg_ft =
            Self::ws20_case3_xdbeg_points(segment, state, snapshot, du_lbs_s_ft, all_detaching);

        let mut next_gstu_lbs_s = vec![0.0_f64; class_count];
        for class_offset in 0..class_count {
            let next_flux =
                Self::ws20_case3_next_flux(ctx, segment, state, snapshot, &xdbeg_ft, class_offset);

            if !next_flux.is_finite() || next_flux < 0.0 {
                return Err(Self::domain_violation(
                    ctx.node_class,
                    BoundarySymbol::from(format!(
                        "ws21_case3_next_flux_{:04}",
                        ctx.class_numbers[class_offset]
                    )),
                    next_flux,
                ));
            }
            next_gstu_lbs_s[class_offset] = next_flux;
        }

        Ok(next_gstu_lbs_s)
    }

    fn ws20_case3_all_detaching(
        class_count: usize,
        snapshot: &Ws20TransportSnapshot,
        du_lbs_s_ft: &[f64],
    ) -> bool {
        let nz = du_lbs_s_ft
            .iter()
            .filter(|value| **value > WS10_ZERO_THRESHOLD)
            .count();
        let nk = snapshot
            .gsu_lbs_s_ft
            .iter()
            .zip(&snapshot.tcu_lbs_s_ft)
            .filter(|(gsu, tcu)| (**gsu - **tcu).abs() <= WS10_ZERO_THRESHOLD)
            .count();
        nz == class_count && nk == class_count
    }

    #[allow(clippy::needless_range_loop)]
    fn ws20_case3_xdbeg_points(
        segment: &Ws20SegmentHydraulics,
        state: &Ws20ClassTransportState,
        snapshot: &Ws20TransportSnapshot,
        du_lbs_s_ft: &[f64],
        all_detaching: bool,
    ) -> Vec<f64> {
        let class_count = state.gstu_lbs_s.len();
        let mut xdbeg_ft = vec![segment.x_upper_ft; class_count];
        for class_offset in 0..class_count {
            if snapshot.tcl_lbs_s_ft[class_offset] < snapshot.potld_lbs_s_ft[class_offset] {
                xdbeg_ft[class_offset] = Self::ws20_case3_xdbeg_value(
                    segment,
                    state,
                    snapshot,
                    du_lbs_s_ft,
                    class_offset,
                    all_detaching,
                );
            }
        }
        xdbeg_ft
    }

    fn ws20_case3_xdbeg_value(
        segment: &Ws20SegmentHydraulics,
        state: &Ws20ClassTransportState,
        snapshot: &Ws20TransportSnapshot,
        du_lbs_s_ft: &[f64],
        class_offset: usize,
        all_detaching: bool,
    ) -> f64 {
        let denxdb = if all_detaching {
            (2.0 * state.dlat_lbs_s_ft[class_offset]) + du_lbs_s_ft[class_offset]
        } else {
            (du_lbs_s_ft[class_offset] / 2.0) + state.dlat_lbs_s_ft[class_offset]
                - snapshot.dtcdx_lbs_s_ft2[class_offset]
        };
        if !denxdb.is_finite() || denxdb.abs() <= WS10_ZERO_THRESHOLD {
            return segment.x_upper_ft;
        }
        if all_detaching {
            return ((segment.dx_ft * du_lbs_s_ft[class_offset]) / denxdb) + segment.x_upper_ft;
        }
        (((snapshot.tcu_lbs_s_ft[class_offset] * segment.wfu_ft) - state.gstu_lbs_s[class_offset])
            / denxdb)
            + segment.x_upper_ft
    }

    fn ws20_case3_next_flux(
        ctx: &Ws20RouteContext<'_>,
        segment: &Ws20SegmentHydraulics,
        state: &Ws20ClassTransportState,
        snapshot: &Ws20TransportSnapshot,
        xdbeg_ft: &[f64],
        class_offset: usize,
    ) -> f64 {
        if snapshot.potld_lbs_s_ft[class_offset] <= snapshot.tcl_lbs_s_ft[class_offset] {
            return snapshot.potld_lbs_s_ft[class_offset] * segment.wfl_ft;
        }

        let xrat = Self::ws20_case3_xrat(segment.x_lower_ft, xdbeg_ft[class_offset]);
        let dl_lbs_s_ft2 =
            Self::ws20_case3_dl_lbs_s_ft2(ctx, state, snapshot, class_offset, xrat);
        let dengsl = snapshot.phi[class_offset] * segment.wfl_ft;
        let gsl_lbs_s_ft =
            Self::ws20_case3_gsl_lbs_s_ft(segment, snapshot, class_offset, dl_lbs_s_ft2, dengsl);
        gsl_lbs_s_ft * segment.wfl_ft
    }

    fn ws20_case3_xrat(x_lower_ft: f64, xdbeg_ft: f64) -> f64 {
        if x_lower_ft.abs() <= WS10_ZERO_THRESHOLD {
            0.0
        } else {
            xdbeg_ft / x_lower_ft
        }
    }

    fn ws20_case3_dl_lbs_s_ft2(
        ctx: &Ws20RouteContext<'_>,
        state: &Ws20ClassTransportState,
        snapshot: &Ws20TransportSnapshot,
        class_offset: usize,
        xrat: f64,
    ) -> f64 {
        if ctx.qlat_cfs_per_ft <= WS10_ZERO_THRESHOLD {
            return snapshot.dtcdx_lbs_s_ft2[class_offset];
        }
        let denphi = 1.0 + snapshot.phi[class_offset];
        if denphi.abs() <= WS10_ZERO_THRESHOLD || !denphi.is_finite() {
            return 0.0;
        }
        (snapshot.phi[class_offset] / denphi)
            * (snapshot.dtcdx_lbs_s_ft2[class_offset] - state.dlat_lbs_s_ft[class_offset])
            * (1.0 - xrat.powf(1.0 + snapshot.phi[class_offset]))
    }

    fn ws20_case3_gsl_lbs_s_ft(
        segment: &Ws20SegmentHydraulics,
        snapshot: &Ws20TransportSnapshot,
        class_offset: usize,
        dl_lbs_s_ft2: f64,
        dengsl: f64,
    ) -> f64 {
        if dengsl.abs() <= WS10_ZERO_THRESHOLD || !dengsl.is_finite() {
            return snapshot.tcl_lbs_s_ft[class_offset];
        }
        snapshot.tcl_lbs_s_ft[class_offset] - (dl_lbs_s_ft2 * segment.x_lower_ft / dengsl)
    }

    #[allow(clippy::similar_names, clippy::too_many_arguments)]
    fn ws20_route_case4_segment(
        ctx: &Ws20RouteContext<'_>,
        profile: &mut Ws20ChannelProfile,
        state: &mut Ws20ClassTransportState,
        diagnostics: &mut Ws20SegmentRoutingDiagnostics,
        segment: &Ws20SegmentHydraulics,
        crfrac: &[f64],
        du_lbs_s_ft: &[f64],
        depsid_ft: f64,
        depmid_ft: f64,
        tb_s: f64,
    ) -> Result<(), Ws10GuardError> {
        let class_count = state.gstu_lbs_s.len();
        let mut potld_case4_lbs_s_ft = vec![0.0_f64; class_count];
        for class_offset in 0..class_count {
            potld_case4_lbs_s_ft[class_offset] = (state.gstu_lbs_s[class_offset]
                + (state.dlat_lbs_s_ft[class_offset] * segment.dx_ft)
                + (du_lbs_s_ft[class_offset] * segment.dx_ft / 2.0))
                / segment.wfl_ft;
        }

        let mut tcl_case4_lbs_s_ft = Self::ws18_trncap(
            segment.effshl,
            &potld_case4_lbs_s_ft,
            &state.crdia_ft,
            &state.crspg,
        );
        let nt_case4 = tcl_case4_lbs_s_ft
            .iter()
            .zip(&potld_case4_lbs_s_ft)
            .filter(|(tcl, potld)| **tcl <= **potld)
            .count();

        if nt_case4 < class_count {
            let ws23_outcome = Self::ws23_detach_case4_iterative_closure(
                ctx.node_class,
                segment.ql_cfs,
                profile.slopes[segment.segment_index].max(WS22_DCAP_MIN_SLOPE),
                ctx.sediment_controls.ctlz,
                ctx.sediment_controls.chnz,
                segment.effshl,
                depsid_ft,
                depmid_ft,
                segment.wfl_ft,
                ctx.roughness,
                ctx.crsh,
                tb_s,
                segment.lower_flagc,
                ctx.chnk,
                ctx.sediment_controls.chnnbr,
                crfrac,
                &state.gstu_lbs_s,
                &state.dlat_lbs_s_ft,
                du_lbs_s_ft,
                segment.dx_ft,
                &state.crdia_ft,
                &state.crspg,
            )?;
            if segment.lower_flagc == 2 && ws23_outcome.werod_ft > segment.wfl_ft {
                profile.width_a_points_ft[segment.segment_index] = ws23_outcome.werod_ft;
            }
            state.gstu_lbs_s = ws23_outcome.next_gstu_lbs_s;
            return Ok(());
        }

        diagnostics.enddet_segments = diagnostics.enddet_segments.saturating_add(1);
        let _ = Self::ws27_case4_enddet_bracket_closure(
            segment.x_upper_ft,
            segment.x_lower_ft,
            segment.wfl_ft,
            segment.dx_ft,
            &state.gstu_lbs_s,
            &state.dlat_lbs_s_ft,
            du_lbs_s_ft,
            &mut potld_case4_lbs_s_ft,
            &mut tcl_case4_lbs_s_ft,
            |potld| Self::ws18_trncap(segment.effshl, potld, &state.crdia_ft, &state.crspg),
        );

        let mut next_gstu_lbs_s = vec![0.0_f64; class_count];
        for class_offset in 0..class_count {
            let next_flux = tcl_case4_lbs_s_ft[class_offset] * segment.wfl_ft;
            if !next_flux.is_finite() || next_flux < 0.0 {
                return Err(Self::domain_violation(
                    ctx.node_class,
                    BoundarySymbol::from(format!(
                        "ws21_case4_next_flux_{:04}",
                        ctx.class_numbers[class_offset]
                    )),
                    next_flux,
                ));
            }
            next_gstu_lbs_s[class_offset] = next_flux;
        }

        state.gstu_lbs_s = next_gstu_lbs_s;
        Ok(())
    }

    fn ws20_route_case12_segment(
        ctx: &Ws20RouteContext<'_>,
        profile: &mut Ws20ChannelProfile,
        state: &mut Ws20ClassTransportState,
        diagnostics: &mut Ws20SegmentRoutingDiagnostics,
        segment: &Ws20SegmentHydraulics,
        snapshot: &Ws20TransportSnapshot,
    ) -> Result<(), Ws10GuardError> {
        let class_count = state.gstu_lbs_s.len();
        let mut saw_case1 = false;
        let mut saw_case2 = false;
        let mut next_gstu_lbs_s = vec![0.0_f64; class_count];
        let mut xde_ft = vec![segment.x_lower_ft; class_count];
        let mut gstde_lbs_s = vec![0.0_f64; class_count];
        let mut case12_nz = 0_usize;
        for class_offset in 0..class_count {
            let update = Self::ws20_case12_class_update(ctx, segment, state, snapshot, class_offset);
            match update.case_kind {
                Ws20Case12ClassKind::Case1 => {
                    saw_case1 = true;
                    case12_nz = case12_nz.saturating_add(1);
                }
                Ws20Case12ClassKind::Case2 => {
                    saw_case2 = true;
                }
            }

            if !update.next_flux_lbs_s.is_finite() || update.next_flux_lbs_s < 0.0 {
                return Err(Self::domain_violation(
                    ctx.node_class,
                    BoundarySymbol::from(format!(
                        "ws20_case12_next_flux_{:04}",
                        ctx.class_numbers[class_offset]
                    )),
                    update.next_flux_lbs_s,
                ));
            }
            next_gstu_lbs_s[class_offset] = update.next_flux_lbs_s;
            xde_ft[class_offset] = update.xde_ft;
            gstde_lbs_s[class_offset] = update.gstde_lbs_s;
        }

        if Self::ws20_try_case12_transition(
            ctx,
            profile,
            state,
            diagnostics,
            segment,
            saw_case1,
            saw_case2,
            case12_nz,
            &xde_ft,
            &gstde_lbs_s,
        )? {
            return Ok(());
        }

        Self::ws20_record_case12_diagnostics(diagnostics, saw_case1, saw_case2);
        state.gstu_lbs_s = next_gstu_lbs_s;
        Ok(())
    }

    #[allow(clippy::similar_names)]
    fn ws20_case12_class_update(
        ctx: &Ws20RouteContext<'_>,
        segment: &Ws20SegmentHydraulics,
        state: &Ws20ClassTransportState,
        snapshot: &Ws20TransportSnapshot,
        class_offset: usize,
    ) -> Ws20Case12ClassUpdate {
        let xrat = if segment.x_lower_ft > WS10_ZERO_THRESHOLD {
            segment.x_upper_ft / segment.x_lower_ft
        } else {
            0.0
        };
        let du_lbs_s_ft2 = if segment.qu_cfs > 1.0e-8 {
            let candidate = (state.fall_ft_s[class_offset] * segment.wfu_ft / segment.qu_cfs)
                * ((snapshot.tcu_lbs_s_ft[class_offset] * segment.wfu_ft)
                    - state.gstu_lbs_s[class_offset]);
            candidate.min(0.0)
        } else if segment.segment_index == 1
            && segment.qu_cfs < 0.001
            && snapshot.dtcdx_lbs_s_ft2[class_offset] < state.dlat_lbs_s_ft[class_offset]
        {
            let phi_k = snapshot.phi[class_offset];
            if phi_k > WS10_ZERO_THRESHOLD {
                (phi_k / (1.0 + phi_k))
                    * (snapshot.dtcdx_lbs_s_ft2[class_offset]
                        - state.dlat_lbs_s_ft[class_offset])
            } else {
                0.0
            }
        } else {
            0.0
        };

        let expon = 1.0 + snapshot.phi[class_offset];
        let mut dl_lbs_s_ft2 = if ctx.qlat_cfs_per_ft > WS10_ZERO_THRESHOLD {
            let phi_k = snapshot.phi[class_offset];
            let numerator =
                phi_k * (snapshot.dtcdx_lbs_s_ft2[class_offset] - state.dlat_lbs_s_ft[class_offset]);
            (numerator / (1.0 + phi_k)) * (1.0 - xrat.powf(expon))
        } else {
            snapshot.dtcdx_lbs_s_ft2[class_offset]
        };
        dl_lbs_s_ft2 += du_lbs_s_ft2 * xrat.powf(expon);

        if dl_lbs_s_ft2 <= 0.0 {
            let phi_k = snapshot.phi[class_offset];
            let gsl = if phi_k > WS10_ZERO_THRESHOLD {
                snapshot.tcl_lbs_s_ft[class_offset]
                    - ((dl_lbs_s_ft2 * segment.x_lower_ft / phi_k) / segment.wfl_ft)
            } else {
                0.0
            };
            return Ws20Case12ClassUpdate {
                next_flux_lbs_s: gsl * segment.wfl_ft,
                xde_ft: segment.x_lower_ft,
                gstde_lbs_s: gsl * segment.wfl_ft,
                case_kind: Ws20Case12ClassKind::Case1,
            };
        }

        let xde_value_ft = if du_lbs_s_ft2.abs() <= WS10_ZERO_THRESHOLD {
            segment.x_upper_ft
        } else if ctx.qlat_cfs_per_ft > WS10_ZERO_THRESHOLD {
            let den = snapshot.dtcdx_lbs_s_ft2[class_offset] - state.dlat_lbs_s_ft[class_offset];
            if den.abs() <= WS10_ZERO_THRESHOLD || snapshot.phi[class_offset] <= WS10_ZERO_THRESHOLD
            {
                segment.x_upper_ft
            } else {
                let core = (1.0
                    - (((1.0 + snapshot.phi[class_offset]) / snapshot.phi[class_offset])
                        * (du_lbs_s_ft2 / den)))
                    .abs();
                segment.x_upper_ft * core.powf(1.0 / (1.0 + snapshot.phi[class_offset]))
            }
        } else if snapshot.dtcdx_lbs_s_ft2[class_offset].abs() <= WS10_ZERO_THRESHOLD {
            segment.x_upper_ft
        } else {
            segment.x_upper_ft * (1.0 - (du_lbs_s_ft2 / snapshot.dtcdx_lbs_s_ft2[class_offset]))
        };

        let gstde_value_lbs_s = if du_lbs_s_ft2.abs() <= WS10_ZERO_THRESHOLD {
            state.gstu_lbs_s[class_offset]
        } else {
            (snapshot.dtcdx_lbs_s_ft2[class_offset] * (xde_value_ft - segment.x_upper_ft))
                + (snapshot.tcu_lbs_s_ft[class_offset] * segment.wfu_ft)
        };
        let gsl_lbs_s_ft = if (xde_value_ft - segment.x_lower_ft).abs() > WS10_ZERO_THRESHOLD {
            (gstde_value_lbs_s
                + (state.dlat_lbs_s_ft[class_offset] * (segment.x_lower_ft - xde_value_ft)))
                / segment.wfl_ft
        } else {
            snapshot.tcl_lbs_s_ft[class_offset]
        };

        Ws20Case12ClassUpdate {
            next_flux_lbs_s: gsl_lbs_s_ft * segment.wfl_ft,
            xde_ft: xde_value_ft,
            gstde_lbs_s: gstde_value_lbs_s,
            case_kind: Ws20Case12ClassKind::Case2,
        }
    }

    #[allow(clippy::similar_names, clippy::too_many_arguments)]
    fn ws20_try_case12_transition(
        ctx: &Ws20RouteContext<'_>,
        profile: &mut Ws20ChannelProfile,
        state: &mut Ws20ClassTransportState,
        diagnostics: &mut Ws20SegmentRoutingDiagnostics,
        segment: &Ws20SegmentHydraulics,
        saw_case1: bool,
        saw_case2: bool,
        case12_nz: usize,
        xde_ft: &[f64],
        gstde_lbs_s: &[f64],
    ) -> Result<bool, Ws10GuardError> {
        let class_count = state.gstu_lbs_s.len();
        if !(ctx.ws21_case34_enabled && saw_case2 && case12_nz < class_count) {
            return Ok(false);
        }

        let xdemax_ft = xde_ft.iter().copied().fold(segment.x_upper_ft, f64::max);
        if xdemax_ft + WS10_ZERO_THRESHOLD >= segment.x_lower_ft {
            return Ok(false);
        }

        let dx_remaining_ft = segment.x_lower_ft - xdemax_ft;
        let mut gstde_transition_lbs_s = gstde_lbs_s.to_vec();
        for class_offset in 0..class_count {
            gstde_transition_lbs_s[class_offset] +=
                state.dlat_lbs_s_ft[class_offset] * (xdemax_ft - xde_ft[class_offset]);
        }

        let crfrac = Self::ws20_segment_crfrac(ctx)?;
        let depmid_ft = ctx.sediment_controls.chnedm * WS15_DEPTH_FROM_METERS_TO_FEET;
        let depsid_ft = ctx.sediment_controls.chneds * WS15_DEPTH_FROM_METERS_TO_FEET;
        let tb_s = 2.0 * ctx.event_duration;

        let ws24_outcome = Self::ws24_case12_detach_transition_closure(
            ctx.node_class,
            segment.ql_cfs,
            profile.slopes[segment.segment_index].max(WS22_DCAP_MIN_SLOPE),
            ctx.sediment_controls.ctlz,
            ctx.sediment_controls.chnz,
            segment.effshl,
            depsid_ft,
            depmid_ft,
            segment.wfl_ft,
            ctx.roughness,
            ctx.crsh,
            tb_s,
            segment.lower_flagc,
            ctx.chnk,
            ctx.sediment_controls.chnnbr,
            crfrac,
            &gstde_transition_lbs_s,
            &state.dlat_lbs_s_ft,
            dx_remaining_ft,
            &state.crdia_ft,
            &state.crspg,
        )?;
        if segment.lower_flagc == 2 && ws24_outcome.werod_ft > segment.wfl_ft {
            profile.width_a_points_ft[segment.segment_index] = ws24_outcome.werod_ft;
        }
        state.gstu_lbs_s = ws24_outcome.next_gstu_lbs_s;
        diagnostics.ws24_case2_detach_segments =
            diagnostics.ws24_case2_detach_segments.saturating_add(1);
        Self::ws20_record_case12_diagnostics(diagnostics, saw_case1, saw_case2);
        Ok(true)
    }

    fn ws20_record_case12_diagnostics(
        diagnostics: &mut Ws20SegmentRoutingDiagnostics,
        saw_case1: bool,
        saw_case2: bool,
    ) {
        if saw_case1 {
            diagnostics.case1_segments = diagnostics.case1_segments.saturating_add(1);
        }
        if saw_case2 {
            diagnostics.case2_segments = diagnostics.case2_segments.saturating_add(1);
        }
    }

    #[allow(clippy::needless_range_loop)]
    fn ws20_outgoing_class_masses(
        ctx: &Ws20RouteContext<'_>,
        state: &Ws20ClassTransportState,
    ) -> Result<Vec<f64>, Ws10GuardError> {
        let class_count = state.gstu_lbs_s.len();
        let mut outgoing_class_mass_kg = vec![0.0_f64; class_count];
        for class_offset in 0..class_count {
            let class_number = ctx.class_numbers[class_offset];
            let mass_kg = state.gstu_lbs_s[class_offset] * ctx.event_duration / WS18_LBS_PER_KG;
            if !mass_kg.is_finite() || mass_kg < 0.0 {
                return Err(Self::domain_violation(
                    ctx.node_class,
                    BoundarySymbol::from(format!("ws20_outgoing_mass_kg_{class_number:04}")),
                    mass_kg,
                ));
            }
            outgoing_class_mass_kg[class_offset] = mass_kg;
        }

        Ok(outgoing_class_mass_kg)
    }

    #[allow(
        clippy::too_many_arguments,
        clippy::many_single_char_names,
        clippy::similar_names
    )]
    fn ws20_route_case12_segment_family_core(
        node_id: u32,
        node_class: Ws10NodeClass,
        ws21_case34_enabled: bool,
        event_duration: f64,
        qpo: f64,
        roughness: f64,
        sediment_controls: Ws15ChannelSedimentControls,
        nslpts: usize,
        peak_partition: Ws20IncomingPeakPartition,
        top_class_mass_kg: &[f64],
        lateral_class_mass_kg: &[f64],
        class_diameters_m: &[f64],
        class_numbers: &[usize],
        mut profile: Ws20ChannelProfile,
        chnk: f64,
        crfrac: Option<&[f64]>,
    ) -> Result<Ws20SegmentRoutingResult, Ws10GuardError> {
        if class_diameters_m.is_empty() {
            return Ok(Self::ws20_empty_segment_result());
        }

        let class_count = class_diameters_m.len();
        Self::ws20_validate_class_cardinality(
            node_class,
            class_count,
            top_class_mass_kg,
            lateral_class_mass_kg,
            class_numbers,
        )?;

        let leff_ft = Self::ws20_effective_length(node_class, &profile)?;
        let (qu_top_cfs, qlat_cfs_per_ft) =
            Self::ws20_flow_partition(node_class, qpo, peak_partition, leff_ft)?;
        let mut class_state = Self::ws20_prepare_class_transport(
            node_class,
            event_duration,
            leff_ft,
            top_class_mass_kg,
            lateral_class_mass_kg,
            class_diameters_m,
            class_numbers,
        )?;

        let flagct = Self::ws30_shape_flag_from_ishape(
            node_class,
            node_id,
            sediment_controls.ishape,
        )?;
        let crsh = sediment_controls.chntcr * WS15_CRSH_FROM_CHNTCR_SCALE;
        let ctx = Ws20RouteContext {
            node_class,
            ws21_case34_enabled,
            event_duration,
            roughness,
            sediment_controls,
            class_numbers,
            qu_top_cfs,
            qlat_cfs_per_ft,
            flagct,
            crsh,
            chnk,
            crfrac,
        };

        let mut diagnostics = Ws20SegmentRoutingDiagnostics::default();
        for segment_index in 1..nslpts {
            Self::ws20_route_one_segment(
                &ctx,
                segment_index,
                &mut profile,
                &mut class_state,
                &mut diagnostics,
            )?;
        }

        let outgoing_class_mass_kg = Self::ws20_outgoing_class_masses(&ctx, &class_state)?;
        Ok(Ws20SegmentRoutingResult {
            routed_class_masses_kg: outgoing_class_mass_kg,
            diagnostics,
            widb_points_ft: profile.width_b_points_ft,
            wida_points_ft: profile.width_a_points_ft,
        })
    }


}
