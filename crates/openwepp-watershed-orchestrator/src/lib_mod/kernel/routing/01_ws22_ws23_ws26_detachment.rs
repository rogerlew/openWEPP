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

#[cfg(test)]
mod ws22_ws23_ws26_detachment_tests {
    use super::*;

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() <= 1e-12,
            "actual {actual} expected {expected}"
        );
    }

    fn ws26_test_input(crfrac: &[f64]) -> Ws26DcapInput<'_> {
        Ws26DcapInput {
            node_class: Ws10NodeClass::Channel,
            flagm: 2,
            q_cfs: 10.0,
            sf: 0.05,
            c1: 0.03,
            z: 20.0,
            effsh: 4.0,
            depsid: 1.0,
            wflow: 2.0,
            roughness: 0.05,
            crsh: 1.0,
            excess: 0.5,
            tb: 10.0,
            flagt: 3,
            chnk: 2.0,
            nbarch: 0.04,
            maxe: 0.2,
            crfrac,
        }
    }

    fn ws26_test_state(
        class_count: usize,
        depmid: f64,
        werod: f64,
        timpot: f64,
        timsh: f64,
        di: f64,
    ) -> Ws26DcapLayerState {
        Ws26DcapLayerState {
            df: vec![0.0; class_count],
            depmid,
            werod,
            timpot,
            timsh,
            di,
        }
    }

    #[test]
    fn wshedimpl20_fall_velocity_and_shdist_cover_boundaries() {
        assert_close(
            Ws10ChannelImpoundmentKernel::ws20_fall_velocity_ft_s(2.65, 0.0),
            0.0,
        );
        assert_close(
            Ws10ChannelImpoundmentKernel::ws20_fall_velocity_ft_s(2.65, 1.0e-5),
            2.811_111_111_111_111_8e-5,
        );
        assert_close(
            Ws10ChannelImpoundmentKernel::ws20_fall_velocity_ft_s(2.65, 1.0e-4),
            0.003_066_666_615_362_89,
        );
        assert_close(
            Ws10ChannelImpoundmentKernel::ws20_fall_velocity_ft_s(2.65, 1.0),
            1.05,
        );

        assert_close(Ws10ChannelImpoundmentKernel::ws22_shdist(0.01), 0.065);
        assert_close(
            Ws10ChannelImpoundmentKernel::ws22_shdist(0.02),
            0.129_993_398_320_733_62,
        );
        assert_close(
            Ws10ChannelImpoundmentKernel::ws22_shdist(0.5),
            1.350_044_264_997_289_5,
        );

    }

    #[test]
    fn wshedimpl30_shape_and_rectangular_fallback_characterize_guards() {
        assert_eq!(
            Ws10ChannelImpoundmentKernel::ws30_shape_flag_from_ishape(
                Ws10NodeClass::Channel,
                7,
                1.0,
            )
            .expect("shape 1 should be accepted"),
            1
        );
        assert_eq!(
            Ws10ChannelImpoundmentKernel::ws30_shape_flag_from_ishape(
                Ws10NodeClass::Channel,
                7,
                2.0,
            )
            .expect("shape 2 should be accepted"),
            2
        );
        assert_eq!(
            Ws10ChannelImpoundmentKernel::ws30_shape_flag_from_ishape(
                Ws10NodeClass::Channel,
                7,
                3.0,
            )
            .expect("shape 3 should be accepted"),
            3
        );

        let fractional_shape = Ws10ChannelImpoundmentKernel::ws30_shape_flag_from_ishape(
            Ws10NodeClass::Channel,
            7,
            1.25,
        )
        .expect_err("fractional shape should fail closed");
        assert_eq!(
            fractional_shape.boundary_class(),
            BoundaryClass::DomainViolation
        );
        assert_eq!(fractional_shape.message_id(), WS10_CHANNEL_GUARD_DOMAIN);

        let out_of_range_shape = Ws10ChannelImpoundmentKernel::ws30_shape_flag_from_ishape(
            Ws10NodeClass::Channel,
            7,
            4.0,
        )
        .expect_err("out-of-range shape should fail closed");
        assert_eq!(
            out_of_range_shape.boundary_class(),
            BoundaryClass::DomainViolation
        );
        assert_eq!(out_of_range_shape.message_id(), WS10_CHANNEL_GUARD_DOMAIN);

        assert_eq!(
            Ws10ChannelImpoundmentKernel::ws30_apply_erodible_rectangular_fallback(3, 1.0e-4),
            2
        );
        assert_eq!(
            Ws10ChannelImpoundmentKernel::ws30_apply_erodible_rectangular_fallback(3, 2.0e-4),
            3
        );
    }

    #[test]
    fn wshedimpl24_transition_rejects_nonpositive_length() {
        let dx_error = Ws10ChannelImpoundmentKernel::ws24_case12_detach_transition_closure(
            Ws10NodeClass::Channel,
            1.0,
            0.02,
            0.03,
            2.0,
            1.0,
            1.0,
            0.5,
            1.0,
            0.04,
            0.02,
            100.0,
            3,
            0.1,
            0.04,
            &[1.0],
            &[0.1],
            &[0.01],
            0.0,
            &[0.0002],
            &[2.65],
        )
        .expect_err("nonpositive transition length should fail closed");
        assert_eq!(dx_error.boundary_class(), BoundaryClass::DomainViolation);
        assert_eq!(dx_error.message_id(), WS10_CHANNEL_GUARD_DOMAIN);
    }

    #[test]
    fn wshedimpl22_table_lookup_characterizes_direction_and_bounds() {
        let col1 = [10.0, 20.0, 30.0];
        let increasing = [0.0, 5.0, 10.0];
        let decreasing = [10.0, 5.0, 0.0];

        let increasing_mid = Ws10ChannelImpoundmentKernel::ws22_table_column2_to_column1(
            &col1,
            &increasing,
            2.5,
            true,
        )
        .expect("increasing table should interpolate in range");
        let decreasing_mid = Ws10ChannelImpoundmentKernel::ws22_table_column2_to_column1(
            &col1,
            &decreasing,
            7.5,
            false,
        )
        .expect("decreasing table should interpolate in range");

        assert!((increasing_mid - 15.0).abs() <= 1e-12);
        assert!((decreasing_mid - 15.0).abs() <= 1e-12);
        assert!(Ws10ChannelImpoundmentKernel::ws22_table_column2_to_column1(
            &col1,
            &increasing,
            12.0,
            true,
        )
        .is_none());
        assert!(Ws10ChannelImpoundmentKernel::ws22_table_column2_to_column1(
            &[1.0],
            &[2.0],
            2.0,
            true,
        )
        .is_none());
        assert!(Ws10ChannelImpoundmentKernel::ws22_table_column2_to_column1(
            &[1.0, 2.0],
            &[1.0],
            1.0,
            true,
        )
        .is_none());
    }

    #[test]
    fn wshedimpl23_detach_rejects_invalid_validation_inputs() {
        let crfrac = [1.0];
        let gstu = [0.1];
        let dlat = [0.01, 0.02];
        let du = [0.01];
        let crdia = [0.0002];
        let crspg = [2.65];

        let cardinality_error = Ws10ChannelImpoundmentKernel::ws23_detach_case4_iterative_closure(
            Ws10NodeClass::Channel,
            1.0,
            0.02,
            0.03,
            2.0,
            1.0,
            1.0,
            0.5,
            1.0,
            0.04,
            0.02,
            100.0,
            3,
            0.1,
            0.04,
            &crfrac,
            &gstu,
            &dlat,
            &du,
            10.0,
            &crdia,
            &crspg,
        )
        .expect_err("class-cardinality mismatch should fail closed");
        assert_eq!(
            cardinality_error.boundary_class(),
            BoundaryClass::DomainViolation
        );
        assert_eq!(cardinality_error.message_id(), WS10_CHANNEL_GUARD_DOMAIN);

        let dlat = [0.01];
        let dx_error = Ws10ChannelImpoundmentKernel::ws23_detach_case4_iterative_closure(
            Ws10NodeClass::Channel,
            1.0,
            0.02,
            0.03,
            2.0,
            1.0,
            1.0,
            0.5,
            1.0,
            0.04,
            0.02,
            100.0,
            3,
            0.1,
            0.04,
            &crfrac,
            &gstu,
            &dlat,
            &du,
            0.0,
            &crdia,
            &crspg,
        )
        .expect_err("nonpositive segment length should fail closed");
        assert_eq!(dx_error.boundary_class(), BoundaryClass::DomainViolation);
        assert_eq!(dx_error.message_id(), WS10_CHANNEL_GUARD_DOMAIN);
    }

    #[test]
    fn wshedimpl23_detach_case4_closure_path_is_finite() {
        let outcome = Ws10ChannelImpoundmentKernel::ws23_detach_case4_iterative_closure(
            Ws10NodeClass::Channel,
            10.0,
            0.05,
            0.03,
            20.0,
            8.0,
            1.0,
            0.25,
            1.0,
            0.05,
            0.02,
            100.0,
            3,
            0.5,
            0.04,
            &[0.2, 0.3, 0.5],
            &[0.05, 0.04, 0.03],
            &[0.005, 0.004, 0.003],
            &[0.002, 0.002, 0.002],
            10.0,
            &[0.0001, 0.0002, 0.0003],
            &[2.60, 2.65, 2.65],
        )
        .expect("case-4 detachment closure should converge for characterized inputs");

        let expected_flux = [
            7.315_907_199_757_94,
            10.898_860_799_636_909,
            18.084_767_999_394_845,
        ];
        for (actual, expected) in outcome.next_gstu_lbs_s.iter().zip(expected_flux) {
            assert!((*actual - expected).abs() <= 1e-12);
        }
        assert!((outcome.werod_ft - 8.256_153_333_081_187).abs() <= 1e-12);
    }

    #[test]
    fn wshedimpl23_detach_case4_iterative_loop_low_shear_is_characterized() {
        let outcome = Ws10ChannelImpoundmentKernel::ws23_detach_case4_iterative_closure(
            Ws10NodeClass::Channel,
            10.0,
            0.05,
            0.03,
            20.0,
            0.021,
            1.0,
            0.25,
            1.0,
            0.05,
            0.02,
            100.0,
            3,
            0.5,
            0.04,
            &[0.2, 0.3, 0.5],
            &[10.0, 8.0, 6.0],
            &[0.005, 0.004, 0.003],
            &[0.002, 0.002, 0.002],
            10.0,
            &[0.0001, 0.0002, 0.0003],
            &[2.60, 2.65, 2.65],
        )
        .expect("low-shear case-4 detachment should iterate to a finite outcome");

        let expected_flux = [
            0.010_271_239_667_832_036,
            0.007_376_358_644_761_355,
            0.006_762_460_067_254_847,
        ];
        for (actual, expected) in outcome.next_gstu_lbs_s.iter().zip(expected_flux) {
            assert_close(*actual, expected);
        }
        assert_close(outcome.werod_ft, 1.0);
    }

    #[test]
    fn wshedimpl23_detach_leaf_helpers_characterize_sums_and_flux_guards() {
        let crfrac = [0.4, 0.6];
        let gstu = [0.05, 0.04];
        let dlat = [0.005, 0.004];
        let du = [0.002, 0.003];
        let crdia = [0.0001, 0.0002];
        let crspg = [2.60, 2.65];
        let input = Ws23DetachInput {
            node_class: Ws10NodeClass::Channel,
            ql_cfs: 5.0,
            sfl: 0.04,
            c1: 0.03,
            z: 15.0,
            effshl: 6.0,
            depsid_ft: 1.0,
            depmid_ft: 0.25,
            wfl_ft: 1.0,
            roughness: 0.05,
            crsh: 0.02,
            tb_s: 100.0,
            flagc: 3,
            chnk: 0.5,
            nbarch: 0.04,
            crfrac: &crfrac,
            gstu_lbs_s: &gstu,
            dlat_lbs_s_ft: &dlat,
            du_lbs_s_ft: &du,
            dx_ft: 10.0,
            crdia_ft: &crdia,
            crspg: &crspg,
        };
        let working = Ws10ChannelImpoundmentKernel::ws23_build_detach_working(
            &input,
            Ws26DcapOutcome {
                df_lbs_s_ft2: vec![0.01, 0.02],
                depmid_ft: 0.2,
                werod_ft: 1.25,
            },
        );
        let sums = Ws10ChannelImpoundmentKernel::ws23_detach_transport_sums(&input, &working);
        assert!((sums.sumtcl - 0.0).abs() <= 1e-12);
        assert!((sums.sumpld - 0.355).abs() <= 1e-12);
        assert!((sums.sumdf - 0.03).abs() <= 1e-12);
        assert!((sums.sumexd - -0.041).abs() <= 1e-12);
        Ws10ChannelImpoundmentKernel::ws23_validate_detach_sums(
            Ws10NodeClass::Channel,
            sums.sumtcl,
            sums.sumpld,
        )
        .expect("finite nonzero potential-load sum should validate");

        let potential =
            Ws10ChannelImpoundmentKernel::ws23_potential_load_outcome(&input, &working);
        assert_eq!(potential.next_gstu_lbs_s, vec![0.16, 0.195]);
        assert!((potential.werod_ft - 1.25).abs() <= 1e-12);

        let mut final_working = working.clone();
        final_working.tcl_lbs_s_ft = vec![0.03, 0.04];
        let final_outcome =
            Ws10ChannelImpoundmentKernel::ws23_final_detach_outcome(&input, &final_working)
                .expect("nonnegative final fluxes should pass");
        assert_eq!(final_outcome.next_gstu_lbs_s, vec![0.03, 0.04]);
        assert!((final_outcome.werod_ft - 1.25).abs() <= 1e-12);

        final_working.tcl_lbs_s_ft[0] = -0.01;
        assert!(
            Ws10ChannelImpoundmentKernel::ws23_final_detach_outcome(&input, &final_working)
                .is_err()
        );
        assert!(Ws10ChannelImpoundmentKernel::ws23_validate_detach_sums(
            Ws10NodeClass::Channel,
            f64::NAN,
            1.0,
        )
        .is_err());
        assert!(Ws10ChannelImpoundmentKernel::ws23_validate_detach_sums(
            Ws10NodeClass::Channel,
            1.0,
            0.0,
        )
        .is_err());
    }

    #[test]
    fn wshedimpl26_dcap_characterizes_expanding_width_path() {
        let outcome = Ws10ChannelImpoundmentKernel::ws26_dcap(
            Ws10NodeClass::Channel,
            2,
            10.0,
            0.05,
            0.03,
            20.0,
            120.0,
            1.0,
            0.25,
            1.0,
            1.0,
            0.05,
            0.02,
            1.0,
            100.0,
            3,
            100.0,
            0.04,
            WS22_DCAP_MAXE,
            &[0.2, 0.3, 0.5],
        )
        .expect("expanding-width detachment capacity should evaluate");

        let expected_df = [
            2.633_526_687_031_146_5,
            3.950_290_030_546_719,
            6.583_816_717_577_865,
        ];
        for (actual, expected) in outcome.df_lbs_s_ft2.iter().zip(expected_df) {
            assert!((*actual - expected).abs() <= 1e-12);
        }
        assert!((outcome.depmid_ft - 0.25).abs() <= 1e-12);
        assert!((outcome.werod_ft - 14.466_284_828_287_218).abs() <= 1e-12);
    }

    #[test]
    fn wshedimpl26_dcap_midlayer_step_characterizes_terminals_and_caps() {
        let crfrac = [0.25, 0.75];
        let input = ws26_test_input(&crfrac);

        match Ws10ChannelImpoundmentKernel::ws26_dcap_midlayer_step(
            &input,
            ws26_test_state(crfrac.len(), 0.0, 0.0, 0.0, 2.0, 0.0),
        )
        .expect("empty midlayer should continue to width step")
        {
            Ws26DcapLayerStep::Continue(state) => {
                assert_close(state.depmid, 0.0);
                assert_close(state.werod, 0.0);
            }
            Ws26DcapLayerStep::Complete(_) => panic!("empty midlayer should not complete"),
        }

        let low_shear_input = Ws26DcapInput {
            effsh: 1.0,
            crsh: 2.0,
            ..input
        };
        match Ws10ChannelImpoundmentKernel::ws26_dcap_midlayer_step(
            &low_shear_input,
            ws26_test_state(crfrac.len(), 0.25, 0.0, 0.0, 2.0, 0.0),
        )
        .expect("negative shear difference should complete without detachment")
        {
            Ws26DcapLayerStep::Complete(outcome) => {
                assert_eq!(outcome.df_lbs_s_ft2, vec![0.0, 0.0]);
                assert_close(outcome.depmid_ft, 0.25);
                assert_close(outcome.werod_ft, 2.0);
            }
            Ws26DcapLayerStep::Continue(_) => panic!("negative shear difference should complete"),
        }

        let zero_di_input = Ws26DcapInput {
            excess: 0.0,
            ..input
        };
        match Ws10ChannelImpoundmentKernel::ws26_dcap_midlayer_step(
            &zero_di_input,
            ws26_test_state(crfrac.len(), 0.25, 0.0, 0.0, 2.0, 0.0),
        )
        .expect("zero excess should complete without detachment")
        {
            Ws26DcapLayerStep::Complete(outcome) => {
                assert_eq!(outcome.df_lbs_s_ft2, vec![0.0, 0.0]);
                assert_close(outcome.depmid_ft, 0.25);
                assert_close(outcome.werod_ft, 2.0);
            }
            Ws26DcapLayerStep::Continue(_) => panic!("zero excess should complete"),
        }

        match Ws10ChannelImpoundmentKernel::ws26_dcap_midlayer_step(
            &input,
            ws26_test_state(crfrac.len(), 0.01, 0.0, 0.0, 2.0, 0.0),
        )
        .expect("sub-timestep detachment should continue")
        {
            Ws26DcapLayerStep::Continue(state) => {
                assert_close(state.di, 3.0);
                assert_close(state.timpot, 0.32);
                assert_close(state.werod, 2.0);
            }
            Ws26DcapLayerStep::Complete(_) => panic!("sub-timestep detachment should continue"),
        }

        match Ws10ChannelImpoundmentKernel::ws26_dcap_midlayer_step(
            &input,
            ws26_test_state(crfrac.len(), 0.1, 0.0, 0.0, 2.0, 0.0),
        )
        .expect("capped complete detachment should evaluate")
        {
            Ws26DcapLayerStep::Complete(outcome) => {
                assert_close(outcome.df_lbs_s_ft2[0], 0.05);
                assert_close(outcome.df_lbs_s_ft2[1], 0.15);
                assert_close(outcome.depmid_ft, 0.079_166_666_666_666_68);
                assert_close(outcome.werod_ft, 2.0);
            }
            Ws26DcapLayerStep::Continue(_) => panic!("capped detachment should complete"),
        }

        let uncapped_input = Ws26DcapInput {
            flagm: 1,
            maxe: WS22_DCAP_MAXE,
            ..input
        };
        match Ws10ChannelImpoundmentKernel::ws26_dcap_midlayer_step(
            &uncapped_input,
            ws26_test_state(crfrac.len(), 0.065, 0.0, 0.0, 2.0, 0.0),
        )
        .expect("depleted layer should clamp to zero")
        {
            Ws26DcapLayerStep::Complete(outcome) => {
                assert_close(outcome.df_lbs_s_ft2[0], 0.15);
                assert_close(outcome.df_lbs_s_ft2[1], 0.45);
                assert_close(outcome.depmid_ft, 0.0);
                assert_close(outcome.werod_ft, 2.0);
            }
            Ws26DcapLayerStep::Continue(_) => panic!("depleted layer should complete"),
        }
    }

    #[test]
    fn wshedimpl26_dcap_expanding_width_characterizes_terminal_and_cap_paths() {
        let crfrac = [0.2, 0.3, 0.5];
        let input = Ws26DcapInput {
            node_class: Ws10NodeClass::Channel,
            flagm: 2,
            q_cfs: 10.0,
            sf: 0.05,
            c1: 0.03,
            z: 20.0,
            effsh: 120.0,
            depsid: 1.0,
            wflow: 1.0,
            roughness: 0.05,
            crsh: 0.02,
            excess: 1.0,
            tb: 100.0,
            flagt: 3,
            chnk: 100.0,
            nbarch: 0.04,
            maxe: 0.1,
            crfrac: &crfrac,
        };
        let state = Ws26DcapLayerState {
            df: vec![0.0, 0.0, 0.0],
            depmid: 0.25,
            werod: 1.0,
            timpot: 0.0,
            timsh: 0.0,
            di: 0.0,
        };
        let ab = input.q_cfs * input.roughness / (1.49 * input.sf.sqrt());

        let capped = Ws10ChannelImpoundmentKernel::ws26_dcap_expanding_width_outcome(
            &input,
            state.clone(),
            100.0,
            ab,
            100.0,
        )
        .expect("capped expanding-width path should evaluate");
        assert_close(capped.df_lbs_s_ft2[0], 0.02);
        assert_close(capped.df_lbs_s_ft2[1], 0.03);
        assert_close(capped.df_lbs_s_ft2[2], 0.05);
        assert_close(capped.df_lbs_s_ft2.iter().sum::<f64>(), 0.1);
        assert!(capped.werod_ft > state.werod);

        let low_ad_input = Ws26DcapInput {
            crsh: 1.0e9,
            ..input
        };
        let low_ad = Ws10ChannelImpoundmentKernel::ws26_dcap_expanding_width_outcome(
            &low_ad_input,
            state.clone(),
            100.0,
            ab,
            100.0,
        )
        .expect("low ad should return the incoming layer state");
        assert_eq!(low_ad.df_lbs_s_ft2, vec![0.0, 0.0, 0.0]);
        assert_close(low_ad.depmid_ft, 0.25);
        assert_close(low_ad.werod_ft, 1.0);

        let wide_state = Ws26DcapLayerState {
            werod: 100.0,
            ..state
        };
        let already_wide = Ws10ChannelImpoundmentKernel::ws26_dcap_expanding_width_outcome(
            &input, wide_state, 100.0, ab, 100.0,
        )
        .expect("already-wide layer should return without widening");
        assert_eq!(already_wide.df_lbs_s_ft2, vec![0.0, 0.0, 0.0]);
        assert_close(already_wide.depmid_ft, 0.25);
        assert_close(already_wide.werod_ft, 100.0);
    }

    #[test]
    fn wshedimpl26_dcap_low_width_shear_outcome_characterizes_terminals() {
        let crfrac = [0.4, 0.6];
        let input = Ws26DcapInput {
            node_class: Ws10NodeClass::Channel,
            flagm: 2,
            q_cfs: 10.0,
            sf: 0.05,
            c1: 0.03,
            z: 20.0,
            effsh: 1.0,
            depsid: 1.0,
            wflow: 1.0,
            roughness: 0.05,
            crsh: 2.0,
            excess: 1.0,
            tb: 100.0,
            flagt: 3,
            chnk: 0.5,
            nbarch: 0.04,
            maxe: WS22_DCAP_MAXE,
            crfrac: &crfrac,
        };

        let outcome = Ws10ChannelImpoundmentKernel::ws26_dcap_low_width_shear_outcome(
            &input,
            Ws26DcapLayerState {
                df: vec![0.0, 0.0],
                depmid: 1.0,
                werod: 1.0,
                timpot: 2.0,
                timsh: 10.0,
                di: 0.5,
            },
        );
        assert!((outcome.df_lbs_s_ft2[0] - 0.004).abs() <= 1e-12);
        assert!((outcome.df_lbs_s_ft2[1] - 0.006).abs() <= 1e-12);
        assert!((outcome.df_lbs_s_ft2.iter().sum::<f64>() - 0.01).abs() <= 1e-12);
        assert!((outcome.depmid_ft - 1.0).abs() <= 1e-12);
        assert!((outcome.werod_ft - 1.0).abs() <= 1e-12);

        let empty_depmid = Ws10ChannelImpoundmentKernel::ws26_dcap_low_width_shear_outcome(
            &input,
            Ws26DcapLayerState {
                df: vec![0.0, 0.0],
                depmid: 0.0,
                werod: 1.0,
                timpot: 2.0,
                timsh: 10.0,
                di: 0.5,
            },
        );
        assert_eq!(empty_depmid.df_lbs_s_ft2, vec![0.0, 0.0]);

        let no_detachment = Ws10ChannelImpoundmentKernel::ws26_dcap_low_width_shear_outcome(
            &input,
            Ws26DcapLayerState {
                df: vec![0.0, 0.0],
                depmid: 1.0,
                werod: 1.0,
                timpot: 2.0,
                timsh: 10.0,
                di: 0.0,
            },
        );
        assert_eq!(no_detachment.df_lbs_s_ft2, vec![0.0, 0.0]);

        let capped_input = Ws26DcapInput {
            maxe: 0.005,
            ..input
        };
        let capped = Ws10ChannelImpoundmentKernel::ws26_dcap_low_width_shear_outcome(
            &capped_input,
            Ws26DcapLayerState {
                df: vec![0.0, 0.0],
                depmid: 1.0,
                werod: 1.0,
                timpot: 2.0,
                timsh: 10.0,
                di: 0.5,
            },
        );
        assert_close(capped.df_lbs_s_ft2[0], 0.002);
        assert_close(capped.df_lbs_s_ft2[1], 0.003);
        assert_close(capped.df_lbs_s_ft2.iter().sum::<f64>(), 0.005);
    }

    #[test]
    fn wshedimpl23_detach_start_and_iteration_helpers_cover_terminals() {
        let complete_crfrac = [1.0];
        let complete_gstu = [0.0];
        let complete_dlat = [0.0];
        let complete_du = [0.0];
        let complete_crdia = [0.0002];
        let complete_crspg = [2.65];
        let complete_input = Ws23DetachInput {
            node_class: Ws10NodeClass::Channel,
            ql_cfs: 1.0,
            sfl: 0.02,
            c1: 0.03,
            z: 2.0,
            effshl: 0.01,
            depsid_ft: 1.0,
            depmid_ft: 0.25,
            wfl_ft: 1.0,
            roughness: 0.04,
            crsh: 0.02,
            tb_s: 100.0,
            flagc: 3,
            chnk: 0.1,
            nbarch: 0.04,
            crfrac: &complete_crfrac,
            gstu_lbs_s: &complete_gstu,
            dlat_lbs_s_ft: &complete_dlat,
            du_lbs_s_ft: &complete_du,
            dx_ft: 10.0,
            crdia_ft: &complete_crdia,
            crspg: &complete_crspg,
        };
        match Ws10ChannelImpoundmentKernel::ws23_initial_detach_working(&complete_input)
            .expect("zero-load initial detach should evaluate")
        {
            Ws23DetachStart::Complete(outcome) => {
                assert_eq!(outcome.next_gstu_lbs_s, vec![0.0]);
                assert_close(outcome.werod_ft, 1.0);
            }
            Ws23DetachStart::Iterate(_) => panic!("zero-load initial detach should complete"),
        }

        let crfrac = [0.2, 0.3, 0.5];
        let gstu = [10.0, 8.0, 6.0];
        let dlat = [0.005, 0.004, 0.003];
        let du = [0.002, 0.002, 0.002];
        let crdia = [0.0001, 0.0002, 0.0003];
        let crspg = [2.60, 2.65, 2.65];
        let iterate_input = Ws23DetachInput {
            node_class: Ws10NodeClass::Channel,
            ql_cfs: 10.0,
            sfl: 0.05,
            c1: 0.03,
            z: 20.0,
            effshl: 0.021,
            depsid_ft: 1.0,
            depmid_ft: 0.25,
            wfl_ft: 1.0,
            roughness: 0.05,
            crsh: 0.02,
            tb_s: 100.0,
            flagc: 3,
            chnk: 0.5,
            nbarch: 0.04,
            crfrac: &crfrac,
            gstu_lbs_s: &gstu,
            dlat_lbs_s_ft: &dlat,
            du_lbs_s_ft: &du,
            dx_ft: 10.0,
            crdia_ft: &crdia,
            crspg: &crspg,
        };
        match Ws10ChannelImpoundmentKernel::ws23_initial_detach_working(&iterate_input)
            .expect("loaded initial detach should evaluate")
        {
            Ws23DetachStart::Iterate(working) => {
                let sums =
                    Ws10ChannelImpoundmentKernel::ws23_detach_transport_sums(&iterate_input, &working);
                assert!(sums.sumtcl > 0.0);
                assert!(sums.sumpld > 0.0);
            }
            Ws23DetachStart::Complete(_) => panic!("loaded initial detach should iterate"),
        }

        let iteration =
            Ws10ChannelImpoundmentKernel::ws23_detach_iteration_working(&iterate_input, 1.0)
                .expect("iteration helper should evaluate");
        let sums =
            Ws10ChannelImpoundmentKernel::ws23_detach_transport_sums(&iterate_input, &iteration);
        assert!(sums.sumtcl.is_finite());
        assert!(sums.sumpld.is_finite());
        assert!(sums.sumdf > 0.0);
        assert!(iteration.tcl_lbs_s_ft.iter().all(|value| value.is_finite()));
    }

    #[test]
    fn wshedimpl27_enddet_bracket_characterizes_iteration_cap() {
        let mut potld_case4 = [0.0];
        let mut tcl_case4 = [0.0];
        let progress = Ws10ChannelImpoundmentKernel::ws27_case4_enddet_bracket_closure(
            0.0,
            10.0,
            1.0,
            10.0,
            &[0.0],
            &[0.0],
            &[0.0],
            &mut potld_case4,
            &mut tcl_case4,
            |_| vec![0.0],
        );
        assert_eq!(progress.iteration_count, 4);
        assert!(progress.used_xdbig_rebracket);
    }
}
