impl Ws10ChannelImpoundmentKernel {
    fn ws20_fall_velocity_ft_s(specific_gravity: f64, particle_diameter_ft: f64) -> f64 {
        if particle_diameter_ft <= WS10_ZERO_THRESHOLD {
            return 0.0;
        }

        let rtsid = ((specific_gravity - 1.0) * WS18_AGRAV * particle_diameter_ft.powi(3)
            / WS18_KNVIS.powi(2))
            * (8.0 / 6.0);
        if rtsid >= 0.024 {
            let rtsid_ln = rtsid.ln();
            for index in 1..WS20_FALVEL_CDRE2.len() {
                if WS20_FALVEL_CDRE2[index] > rtsid_ln {
                    let x0 = WS20_FALVEL_CDRE2[index - 1];
                    let x1 = WS20_FALVEL_CDRE2[index];
                    let y0 = WS20_FALVEL_CDRE[index - 1];
                    let y1 = WS20_FALVEL_CDRE[index];
                    let reynolds_log = y0 + (((rtsid_ln - x0) / (x1 - x0)) * (y1 - y0));
                    return reynolds_log.exp() * WS18_KNVIS / particle_diameter_ft;
                }
            }

            return WS20_FALVEL_CDRE[WS20_FALVEL_CDRE.len() - 1].exp() * WS18_KNVIS
                / particle_diameter_ft;
        }

        (particle_diameter_ft.powi(2) * (specific_gravity - 1.0) * WS18_AGRAV) / (WS18_KNVIS * 18.0)
    }

    fn ws30_shape_flag_from_ishape(
        node_class: Ws10NodeClass,
        node_id: u32,
        ishape: f64,
    ) -> Result<i32, Ws10GuardError> {
        let ishape_rounded = ishape.round();
        if (ishape - ishape_rounded).abs() > WS11_IPEAK_INTEGER_TOLERANCE {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from(format!("ws10_channel_{node_id}_ishape")),
                ishape,
            ));
        }

        if !(1.0..=3.0).contains(&ishape_rounded) {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from(format!("ws10_channel_{node_id}_ishape")),
                ishape,
            ));
        }

        if (ishape_rounded - 1.0).abs() <= WS11_IPEAK_INTEGER_TOLERANCE {
            Ok(1)
        } else if (ishape_rounded - 2.0).abs() <= WS11_IPEAK_INTEGER_TOLERANCE {
            Ok(2)
        } else {
            Ok(3)
        }
    }

    fn ws30_apply_erodible_rectangular_fallback(flagct: i32, erodible_depth_ft: f64) -> i32 {
        if flagct == 3 && erodible_depth_ft <= WS30_ERODIBLE_RECTANGULAR_DEPTH_THRESHOLD_FT {
            2
        } else {
            flagct
        }
    }

    fn ws22_require_crfrac_vector(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
        class_numbers: &[usize],
    ) -> Result<Vec<f64>, Ws10GuardError> {
        let mut crfrac = Vec::with_capacity(class_numbers.len());
        for class_number in class_numbers {
            let symbol = BoundarySymbol::from(format!(
                "ws10_channel_{}_crfrac_{:04}",
                request.node_id, class_number
            ));
            let value =
                Self::require_channel_state_symbol_scalar(request, node_class, symbol.clone())?;
            Self::require_channel_control_range(node_class, symbol, value, Some(0.0), Some(1.0))?;
            crfrac.push(value);
        }

        let sum = crfrac.iter().copied().sum::<f64>();
        if !sum.is_finite() || sum <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from(format!("ws10_channel_{}_crfrac_sum", request.node_id)),
                sum,
            ));
        }
        for value in &mut crfrac {
            *value /= sum;
        }
        Ok(crfrac)
    }

    fn ws22_table_column2_to_column1(
        col1: &[f64],
        col2: &[f64],
        given: f64,
        column2_increasing: bool,
    ) -> Option<f64> {
        if col1.len() != col2.len() || col1.len() < 2 {
            return None;
        }

        for index in 1..col1.len() {
            let left = col2[index - 1];
            let right = col2[index];
            let in_range = if column2_increasing {
                given >= left && given <= right
            } else {
                given <= left && given >= right
            };
            if in_range {
                return Some(Self::ws18_linear_interpolate(
                    left,
                    col1[index - 1],
                    right,
                    col1[index],
                    given,
                ));
            }
        }

        None
    }

    fn ws22_shdist(x: f64) -> f64 {
        if x >= 0.02 {
            return (0.12692
                - (0.51634 * x.ln())
                - (0.40825 * x.ln().powi(2))
                - (0.03442 * x.ln().powi(3)))
            .exp();
        }
        0.13 * x / 0.02
    }

    #[allow(
        clippy::too_many_arguments,
        clippy::similar_names
    )]
    pub(crate) fn ws26_dcap(
        node_class: Ws10NodeClass,
        flagm: i32,
        q_cfs: f64,
        sf: f64,
        c1: f64,
        z: f64,
        effsh: f64,
        depsid: f64,
        depmid_input: f64,
        werod_input: f64,
        wflow: f64,
        roughness: f64,
        crsh: f64,
        excess: f64,
        tb: f64,
        flagt: i32,
        chnk: f64,
        nbarch: f64,
        maxe: f64,
        crfrac: &[f64],
    ) -> Result<Ws26DcapOutcome, Ws10GuardError> {
        let input = Ws26DcapInput {
            node_class,
            flagm,
            q_cfs,
            sf,
            c1,
            z,
            effsh,
            depsid,
            wflow,
            roughness,
            crsh,
            excess,
            tb,
            flagt,
            chnk,
            nbarch,
            maxe,
            crfrac,
        };
        let df = vec![0.0; crfrac.len()];
        if effsh <= crsh {
            return Ok(Self::ws26_dcap_outcome(df, depmid_input, werod_input));
        }

        let layer_state = Ws26DcapLayerState {
            df,
            depmid: depmid_input,
            werod: werod_input,
            timpot: 0.0,
            timsh: tb * (1.0 - (crsh / effsh)),
            di: 0.0,
        };

        match Self::ws26_dcap_midlayer_step(&input, layer_state)? {
            Ws26DcapLayerStep::Complete(outcome) => Ok(outcome),
            Ws26DcapLayerStep::Continue(state) => Self::ws26_dcap_width_step(&input, state),
        }
    }

    fn ws26_dcap_outcome(
        df_lbs_s_ft2: Vec<f64>,
        depmid_ft: f64,
        werod_ft: f64,
    ) -> Ws26DcapOutcome {
        Ws26DcapOutcome {
            df_lbs_s_ft2,
            depmid_ft,
            werod_ft,
        }
    }

    fn ws26_apply_dct_to_classes(df: &mut [f64], dct: f64, crfrac: &[f64]) {
        for class_offset in 0..crfrac.len() {
            df[class_offset] = dct * crfrac[class_offset];
        }
    }

    fn ws26_dcap_erodible_width(
        input: &Ws26DcapInput<'_>,
        fallback_width: f64,
    ) -> Result<f64, Ws10GuardError> {
        let (wtmp, _) = Self::ws18_hydchn(
            input.node_class,
            4,
            input.q_cfs,
            input.sf.max(WS22_DCAP_MIN_SLOPE),
            input.c1,
            input.z,
            fallback_width,
            input.roughness,
            input.crsh,
            input.nbarch,
        )?;
        Ok(wtmp)
    }

    fn ws26_dcap_midlayer_step(
        input: &Ws26DcapInput<'_>,
        mut state: Ws26DcapLayerState,
    ) -> Result<Ws26DcapLayerStep, Ws10GuardError> {
        if state.depmid <= WS10_ZERO_THRESHOLD {
            return Ok(Ws26DcapLayerStep::Continue(state));
        }

        state.werod = if input.flagt == 3 {
            input.wflow
        } else {
            Self::ws26_dcap_erodible_width(input, input.wflow)?
        };

        let difsh = input.effsh - input.crsh;
        if difsh <= 0.0 {
            return Ok(Ws26DcapLayerStep::Complete(Self::ws26_dcap_outcome(
                state.df,
                state.depmid,
                state.werod,
            )));
        }

        state.di = input.excess * input.chnk * difsh;
        if state.di <= WS10_ZERO_THRESHOLD {
            return Ok(Ws26DcapLayerStep::Complete(Self::ws26_dcap_outcome(
                state.df,
                state.depmid,
                state.werod,
            )));
        }

        state.timpot = state.depmid * WS22_DCAP_WTDSOI / state.di;
        if state.timpot < state.timsh {
            return Ok(Ws26DcapLayerStep::Continue(state));
        }

        let mut dct = state.di * state.timsh * state.werod / (input.tb * input.wflow);
        if input.flagm != 1 && dct >= input.maxe {
            state.di *= input.maxe / dct;
            dct = input.maxe;
        }
        Self::ws26_apply_dct_to_classes(&mut state.df, dct, input.crfrac);
        state.depmid -= state.di * state.timsh / WS22_DCAP_WTDSOI;
        if state.depmid < 0.005 {
            state.depmid = 0.0;
        }
        Ok(Ws26DcapLayerStep::Complete(Self::ws26_dcap_outcome(
            state.df,
            state.depmid,
            state.werod,
        )))
    }

    fn ws26_dcap_width_step(
        input: &Ws26DcapInput<'_>,
        mut state: Ws26DcapLayerState,
    ) -> Result<Ws26DcapOutcome, Ws10GuardError> {
        let timex = state.timsh - state.timpot;
        let ab = input.q_cfs * input.roughness
            / (1.49 * input.sf.max(WS22_DCAP_MIN_SLOPE).sqrt());

        if state.werod <= WS10_ZERO_THRESHOLD {
            state.werod = Self::ws26_dcap_erodible_width(input, input.wflow)?;
        }

        let hxb = ab / state.werod.powf(8.0 / 3.0);
        let Some(xb) = Self::ws22_table_column2_to_column1(
            &WS18_HYDCHN_XXB,
            &WS18_HYDCHN_FHXB,
            hxb.min(9999.99),
            true,
        ) else {
            return Err(Self::domain_violation(
                input.node_class,
                BoundarySymbol::from("ws22_dcap_hxb"),
                hxb,
            ));
        };

        let difsh = input.effsh * Self::ws22_shdist(xb) - input.crsh;
        if difsh <= 0.0 {
            return Ok(Self::ws26_dcap_low_width_shear_outcome(input, state));
        }

        Self::ws26_dcap_expanding_width_outcome(input, state, timex, ab, difsh)
    }

    fn ws26_dcap_low_width_shear_outcome(
        input: &Ws26DcapInput<'_>,
        mut state: Ws26DcapLayerState,
    ) -> Ws26DcapOutcome {
        if state.depmid <= 0.0 {
            return Self::ws26_dcap_outcome(state.df, state.depmid, state.werod);
        }
        state.timsh = state.timpot;
        if state.di <= WS10_ZERO_THRESHOLD {
            return Self::ws26_dcap_outcome(state.df, state.depmid, state.werod);
        }
        let mut dct = state.di * state.timsh * state.werod / (input.tb * input.wflow);
        if input.flagm != 1 && dct >= input.maxe {
            dct = input.maxe;
        }
        Self::ws26_apply_dct_to_classes(&mut state.df, dct, input.crfrac);
        Self::ws26_dcap_outcome(state.df, state.depmid, state.werod)
    }

    fn ws26_dcap_expanding_width_outcome(
        input: &Ws26DcapInput<'_>,
        mut state: Ws26DcapLayerState,
        timex: f64,
        ab: f64,
        difsh: f64,
    ) -> Result<Ws26DcapOutcome, Ws10GuardError> {
        let dwdti = input.excess * 2.0 * input.chnk * difsh / WS22_DCAP_WTDSOI;
        let ad = ab.powf(0.375) * WS18_WTDH2O * input.sf.max(WS22_DCAP_MIN_SLOPE) / input.crsh;
        if ad <= WS22_DCAP_FFXCF[WS22_DCAP_FFXCF.len() - 1] {
            return Ok(Self::ws26_dcap_outcome(
                state.df,
                state.depmid,
                state.werod,
            ));
        }

        let Some(xcf) = Self::ws22_table_column2_to_column1(
            &WS22_DCAP_XXCF,
            &WS22_DCAP_FFXCF,
            ad.min(999.999),
            false,
        ) else {
            return Err(Self::domain_violation(
                input.node_class,
                BoundarySymbol::from("ws22_dcap_ad"),
                ad,
            ));
        };

        if xcf <= WS10_ZERO_THRESHOLD || (1.0 - (2.0 * xcf)) <= WS10_ZERO_THRESHOLD {
            return Ok(Self::ws26_dcap_outcome(
                state.df,
                state.depmid,
                state.werod,
            ));
        }
        let wfin_core = xcf * (1.0 - (2.0 * xcf)) / xcf.powf(8.0 / 3.0);
        if !wfin_core.is_finite() || wfin_core <= WS10_ZERO_THRESHOLD {
            return Ok(Self::ws26_dcap_outcome(
                state.df,
                state.depmid,
                state.werod,
            ));
        }
        let wfin = ab.powf(0.375) * wfin_core.powf(0.375);
        if wfin <= state.werod {
            return Ok(Self::ws26_dcap_outcome(
                state.df,
                state.depmid,
                state.werod,
            ));
        }

        let tstar = timex * dwdti / (wfin - state.werod);
        let wstar = (1.0 - (-1.0176 * tstar).exp()) / 1.0176;
        let we = wstar * (wfin - state.werod) + state.werod;
        let eros = (we - state.werod) * input.depsid + state.depmid * state.werod;
        let mut dct = eros * WS22_DCAP_WTDSOI / (input.tb * input.wflow);
        if input.flagm != 1 && dct >= input.maxe {
            dct = input.maxe;
        }

        Self::ws26_apply_dct_to_classes(&mut state.df, dct, input.crfrac);
        Ok(Self::ws26_dcap_outcome(state.df, state.depmid, we))
    }

    #[allow(
        clippy::too_many_arguments,
        clippy::many_single_char_names,
        clippy::similar_names
    )]
    fn ws23_detach_case4_iterative_closure(
        node_class: Ws10NodeClass,
        ql_cfs: f64,
        sfl: f64,
        c1: f64,
        z: f64,
        effshl: f64,
        depsid_ft: f64,
        depmid_ft: f64,
        wfl_ft: f64,
        roughness: f64,
        crsh: f64,
        tb_s: f64,
        flagc: i32,
        chnk: f64,
        nbarch: f64,
        crfrac: &[f64],
        gstu_lbs_s: &[f64],
        dlat_lbs_s_ft: &[f64],
        du_lbs_s_ft: &[f64],
        dx_ft: f64,
        crdia_ft: &[f64],
        crspg: &[f64],
    ) -> Result<Ws23DetachClosureOutcome, Ws10GuardError> {
        let input = Ws23DetachInput {
            node_class,
            ql_cfs,
            sfl,
            c1,
            z,
            effshl,
            depsid_ft,
            depmid_ft,
            wfl_ft,
            roughness,
            crsh,
            tb_s,
            flagc,
            chnk,
            nbarch,
            crfrac,
            gstu_lbs_s,
            dlat_lbs_s_ft,
            du_lbs_s_ft,
            dx_ft,
            crdia_ft,
            crspg,
        };
        Self::ws23_validate_detach_input(&input)?;

        let mut working = match Self::ws23_initial_detach_working(&input)? {
            Ws23DetachStart::Complete(outcome) => return Ok(outcome),
            Ws23DetachStart::Iterate(working) => working,
        };
        let mut sums = Self::ws23_detach_transport_sums(&input, &working);
        Self::ws23_validate_detach_sums(input.node_class, sums.sumtcl, sums.sumpld)?;

        let mut excess = sums.sumtcl / sums.sumpld;
        let mut excold = excess;

        for _ in 0..20 {
            if excess < 0.0 {
                excess = 0.0;
            }

            working = Self::ws23_detach_iteration_working(&input, excess)?;
            sums = Self::ws23_detach_transport_sums(&input, &working);

            if !sums.sumtcl.is_finite() || !sums.sumpld.is_finite() {
                return Err(Self::domain_violation(
                    input.node_class,
                    BoundarySymbol::from("ws23_detach_sumtc_sumpl"),
                    sums.sumtcl,
                ));
            }

            if sums.sumtcl.abs() > WS10_ZERO_THRESHOLD
                && ((sums.sumtcl - sums.sumpld) / sums.sumtcl).abs() < 0.01
            {
                break;
            }

            let mut ratex = if sums.sumdf.abs() > 1.0e-8 {
                sums.sumexd / sums.sumdf
            } else {
                sums.sumtcl / sums.sumpld
            };
            if !ratex.is_finite() || ratex <= 0.0 {
                ratex = sums.sumtcl / sums.sumpld;
            }
            excess = excold * ratex;
            excold = excess;
        }

        Self::ws23_final_detach_outcome(&input, &working)
    }

    fn ws23_validate_detach_input(input: &Ws23DetachInput<'_>) -> Result<(), Ws10GuardError> {
        let class_count = input.gstu_lbs_s.len();
        if class_count == 0
            || input.dlat_lbs_s_ft.len() != class_count
            || input.du_lbs_s_ft.len() != class_count
            || input.crdia_ft.len() != class_count
            || input.crspg.len() != class_count
            || input.crfrac.len() != class_count
        {
            return Err(Self::domain_violation(
                input.node_class,
                BoundarySymbol::from("ws23_detach_class_cardinality"),
                f64::from(u32::try_from(class_count).unwrap_or(u32::MAX)),
            ));
        }

        if input.dx_ft <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                input.node_class,
                BoundarySymbol::from("ws23_detach_dx_ft"),
                input.dx_ft,
            ));
        }

        Ok(())
    }

    fn ws23_initial_detach_working(
        input: &Ws23DetachInput<'_>,
    ) -> Result<Ws23DetachStart, Ws10GuardError> {
        let dcap_outcome = Self::ws26_dcap(
            input.node_class,
            1,
            input.ql_cfs,
            input.sfl,
            input.c1,
            input.z,
            input.effshl,
            input.depsid_ft,
            input.depmid_ft,
            input.wfl_ft,
            input.wfl_ft,
            input.roughness,
            input.crsh,
            1.0,
            input.tb_s,
            input.flagc,
            input.chnk,
            input.nbarch,
            WS22_DCAP_MAXE,
            input.crfrac,
        )?;
        let mut working = Self::ws23_build_detach_working(input, dcap_outcome);
        let nt3 = working
            .dl_lbs_s_ft
            .iter()
            .zip(&working.potld_lbs_s_ft)
            .filter(|(dl, potld)| {
                dl.abs() <= WS10_ZERO_THRESHOLD && potld.abs() <= WS10_ZERO_THRESHOLD
            })
            .count();
        if nt3 < input.gstu_lbs_s.len() {
            working.tcl_lbs_s_ft = Self::ws18_trncap(
                input.effshl,
                &working.potld_lbs_s_ft,
                input.crdia_ft,
                input.crspg,
            );
        }

        let nt2 = working
            .tcl_lbs_s_ft
            .iter()
            .zip(&working.potld_lbs_s_ft)
            .filter(|(tcl, potld)| **tcl >= **potld)
            .count();
        if nt2 == input.gstu_lbs_s.len() || nt3 == input.gstu_lbs_s.len() {
            return Ok(Ws23DetachStart::Complete(
                Self::ws23_potential_load_outcome(input, &working),
            ));
        }

        Ok(Ws23DetachStart::Iterate(working))
    }

    fn ws23_detach_iteration_working(
        input: &Ws23DetachInput<'_>,
        excess: f64,
    ) -> Result<Ws23DetachWorking, Ws10GuardError> {
        let dcap_outcome = Self::ws26_dcap(
            input.node_class,
            2,
            input.ql_cfs,
            input.sfl,
            input.c1,
            input.z,
            input.effshl,
            input.depsid_ft,
            input.depmid_ft,
            input.wfl_ft,
            input.wfl_ft,
            input.roughness,
            input.crsh,
            excess,
            input.tb_s,
            input.flagc,
            input.chnk,
            input.nbarch,
            WS22_DCAP_MAXE,
            input.crfrac,
        )?;
        let mut working = Self::ws23_build_detach_working(input, dcap_outcome);
        working.tcl_lbs_s_ft = Self::ws18_trncap(
            input.effshl,
            &working.potld_lbs_s_ft,
            input.crdia_ft,
            input.crspg,
        );
        Ok(working)
    }

    fn ws23_build_detach_working(
        input: &Ws23DetachInput<'_>,
        dcap_outcome: Ws26DcapOutcome,
    ) -> Ws23DetachWorking {
        let class_count = input.gstu_lbs_s.len();
        let df_lbs_s_ft2 = dcap_outcome.df_lbs_s_ft2.clone();
        let mut dl_lbs_s_ft = vec![0.0_f64; class_count];
        let mut potld_lbs_s_ft = vec![0.0_f64; class_count];
        for class_offset in 0..class_count {
            dl_lbs_s_ft[class_offset] = df_lbs_s_ft2[class_offset] * input.wfl_ft;
            potld_lbs_s_ft[class_offset] = (input.gstu_lbs_s[class_offset]
                + (input.dlat_lbs_s_ft[class_offset] * input.dx_ft)
                + ((dl_lbs_s_ft[class_offset] + input.du_lbs_s_ft[class_offset])
                    * input.dx_ft
                    / 2.0))
                / input.wfl_ft;
        }
        Ws23DetachWorking {
            dcap_outcome,
            df_lbs_s_ft2,
            dl_lbs_s_ft,
            potld_lbs_s_ft,
            tcl_lbs_s_ft: vec![0.0_f64; class_count],
        }
    }

    fn ws23_detach_transport_sums(
        input: &Ws23DetachInput<'_>,
        working: &Ws23DetachWorking,
    ) -> Ws23DetachSums {
        let mut sums = Ws23DetachSums {
            sumtcl: 0.0,
            sumpld: 0.0,
            sumdf: 0.0,
            sumexd: 0.0,
        };
        for class_offset in 0..input.gstu_lbs_s.len() {
            sums.sumtcl += working.tcl_lbs_s_ft[class_offset];
            sums.sumpld += working.potld_lbs_s_ft[class_offset];
            let exdet = (((working.tcl_lbs_s_ft[class_offset] * input.wfl_ft)
                - input.gstu_lbs_s[class_offset]
                - (input.dlat_lbs_s_ft[class_offset] * input.dx_ft))
                * (2.0 / input.dx_ft)
                - input.du_lbs_s_ft[class_offset])
                / input.wfl_ft;
            sums.sumexd += exdet;
            sums.sumdf += working.df_lbs_s_ft2[class_offset];
        }
        sums
    }

    fn ws23_validate_detach_sums(
        node_class: Ws10NodeClass,
        sumtcl: f64,
        sumpld: f64,
    ) -> Result<(), Ws10GuardError> {
        if !sumtcl.is_finite() || !sumpld.is_finite() || sumpld.abs() <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws23_detach_sumpld"),
                sumpld,
            ));
        }
        Ok(())
    }

    #[allow(clippy::needless_range_loop)]
    fn ws23_potential_load_outcome(
        input: &Ws23DetachInput<'_>,
        working: &Ws23DetachWorking,
    ) -> Ws23DetachClosureOutcome {
        let mut next_gstu_lbs_s = vec![0.0_f64; input.gstu_lbs_s.len()];
        for class_offset in 0..input.gstu_lbs_s.len() {
            next_gstu_lbs_s[class_offset] =
                working.potld_lbs_s_ft[class_offset] * input.wfl_ft;
        }
        Ws23DetachClosureOutcome {
            next_gstu_lbs_s,
            werod_ft: working.dcap_outcome.werod_ft,
        }
    }

    #[allow(clippy::needless_range_loop)]
    fn ws23_final_detach_outcome(
        input: &Ws23DetachInput<'_>,
        working: &Ws23DetachWorking,
    ) -> Result<Ws23DetachClosureOutcome, Ws10GuardError> {
        let mut next_gstu_lbs_s = vec![0.0_f64; input.gstu_lbs_s.len()];
        for class_offset in 0..input.gstu_lbs_s.len() {
            let next_flux = working.tcl_lbs_s_ft[class_offset] * input.wfl_ft;
            if !next_flux.is_finite() || next_flux < 0.0 {
                return Err(Self::domain_violation(
                    input.node_class,
                    BoundarySymbol::from("ws23_detach_next_flux"),
                    next_flux,
                ));
            }
            next_gstu_lbs_s[class_offset] = next_flux;
        }
        Ok(Ws23DetachClosureOutcome {
            next_gstu_lbs_s,
            werod_ft: working.dcap_outcome.werod_ft,
        })
    }

    #[allow(clippy::too_many_arguments, clippy::similar_names)]
    fn ws24_case12_detach_transition_closure(
        node_class: Ws10NodeClass,
        ql_cfs: f64,
        sfl: f64,
        c1: f64,
        z: f64,
        effshl: f64,
        depsid_ft: f64,
        depmid_ft: f64,
        wfl_ft: f64,
        roughness: f64,
        crsh: f64,
        tb_s: f64,
        flagc: i32,
        chnk: f64,
        nbarch: f64,
        crfrac: &[f64],
        gstde_lbs_s: &[f64],
        dlat_lbs_s_ft: &[f64],
        dx_ft_remaining: f64,
        crdia_ft: &[f64],
        crspg: &[f64],
    ) -> Result<Ws23DetachClosureOutcome, Ws10GuardError> {
        if dx_ft_remaining <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws24_case12_dx_remaining"),
                dx_ft_remaining,
            ));
        }

        let zero_du_lbs_s_ft = vec![0.0_f64; gstde_lbs_s.len()];
        Self::ws23_detach_case4_iterative_closure(
            node_class,
            ql_cfs,
            sfl,
            c1,
            z,
            effshl,
            depsid_ft,
            depmid_ft,
            wfl_ft,
            roughness,
            crsh,
            tb_s,
            flagc,
            chnk,
            nbarch,
            crfrac,
            gstde_lbs_s,
            dlat_lbs_s_ft,
            &zero_du_lbs_s_ft,
            dx_ft_remaining,
            crdia_ft,
            crspg,
        )
    }

    #[allow(
        clippy::too_many_arguments,
        clippy::many_single_char_names,
        clippy::similar_names
    )]
    pub(crate) fn ws27_case4_enddet_bracket_closure(
        x_upper_ft: f64,
        x_lower_ft: f64,
        wfl_ft: f64,
        dx_ft: f64,
        gstu_lbs_s: &[f64],
        dlat_lbs_s_ft: &[f64],
        du_lbs_s_ft: &[f64],
        potld_case4_lbs_s_ft: &mut [f64],
        tcl_case4_lbs_s_ft: &mut [f64],
        mut trncap: impl FnMut(&[f64]) -> Vec<f64>,
    ) -> Ws27EnddetBracketProgress {
        let class_count = potld_case4_lbs_s_ft.len();
        let mut progress = Ws27EnddetBracketProgress::default();
        let mut xdsmal_ft = x_upper_ft;
        let mut xdbig_ft = x_lower_ft;
        let mut xdbmin_ft = x_lower_ft;
        let mut ndep = 0_u8;
        let mut recompute_xdbeg = true;

        loop {
            if recompute_xdbeg {
                let mut xdbeg_ft = vec![x_lower_ft; class_count];
                for class_offset in 0..class_count {
                    if potld_case4_lbs_s_ft[class_offset] > tcl_case4_lbs_s_ft[class_offset]
                        && du_lbs_s_ft[class_offset].abs() > WS10_ZERO_THRESHOLD
                    {
                        xdbeg_ft[class_offset] = ((2.0
                            * ((tcl_case4_lbs_s_ft[class_offset] * wfl_ft)
                                - gstu_lbs_s[class_offset]
                                - (dlat_lbs_s_ft[class_offset] * dx_ft)))
                            / du_lbs_s_ft[class_offset])
                            + x_upper_ft;
                    }
                }

                xdbmin_ft = xdbeg_ft.iter().copied().fold(x_lower_ft, f64::min);
                if xdbmin_ft <= xdsmal_ft {
                    xdbmin_ft = xdsmal_ft;
                }
            }

            for class_offset in 0..class_count {
                potld_case4_lbs_s_ft[class_offset] = (gstu_lbs_s[class_offset]
                    + (dlat_lbs_s_ft[class_offset] * dx_ft)
                    + (du_lbs_s_ft[class_offset] * (xdbmin_ft - x_upper_ft) / 2.0))
                    / wfl_ft;
            }
            tcl_case4_lbs_s_ft.copy_from_slice(&trncap(potld_case4_lbs_s_ft));

            ndep = ndep.saturating_add(1);
            progress.iteration_count = ndep;
            if ndep == 4 {
                break;
            }

            let mut nt = 0_usize;
            let mut sumtc = 0.0_f64;
            let mut sumpl = 0.0_f64;
            for class_offset in 0..class_count {
                sumtc += tcl_case4_lbs_s_ft[class_offset];
                sumpl += potld_case4_lbs_s_ft[class_offset];
                if tcl_case4_lbs_s_ft[class_offset] <= potld_case4_lbs_s_ft[class_offset] {
                    nt += 1;
                }
            }

            if sumtc.abs() > WS10_ZERO_THRESHOLD && ((sumtc - sumpl) / sumtc).abs() < 0.01 {
                break;
            }

            if nt < class_count {
                xdsmal_ft = xdbmin_ft;
                xdbmin_ft = 0.5 * (xdsmal_ft + xdbig_ft);
                recompute_xdbeg = false;
                progress.used_midpoint_rebracket = true;
            } else {
                xdbig_ft = xdbmin_ft;
                recompute_xdbeg = true;
                progress.used_xdbig_rebracket = true;
            }
        }

        progress
    }

}
