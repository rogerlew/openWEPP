impl Ws10ChannelImpoundmentKernel {
    #[allow(clippy::similar_names)]
    fn derive_ws15_channel_sediment_scaffold(
        node_class: Ws10NodeClass,
        node_id: u32,
        controls: Ws15ChannelSedimentControls,
    ) -> Result<Ws15ChannelSedimentScaffold, Ws10GuardError> {
        let crsh = controls.chntcr * WS15_CRSH_FROM_CHNTCR_SCALE;
        let depmid = controls.chnedm * WS15_DEPTH_FROM_METERS_TO_FEET;
        let depsid = controls.chneds * WS15_DEPTH_FROM_METERS_TO_FEET;

        for (suffix, value) in [
            ("chz", controls.chnz),
            ("nbarch", controls.chnnbr),
            ("crsh", crsh),
            ("depmid", depmid),
            ("depsid", depsid),
        ] {
            if !value.is_finite() {
                return Err(Self::non_finite(
                    node_class,
                    BoundarySymbol::from(format!("ws10_channel_{node_id}_{suffix}")),
                    value,
                ));
            }
            if value < 0.0 {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from(format!("ws10_channel_{node_id}_{suffix}")),
                    value,
                ));
            }
        }

        Ok(Ws15ChannelSedimentScaffold {
            chz: controls.chnz,
            nbarch: controls.chnnbr,
            crsh,
            depmid,
            depsid,
        })
    }

    fn ws18_linear_interpolate(x1: f64, y1: f64, x2: f64, y2: f64, x: f64) -> f64 {
        let denominator = x2 - x1;
        if denominator.abs() <= WS10_ZERO_THRESHOLD {
            0.5 * (y1 + y2)
        } else {
            y1 + ((y2 - y1) * (x - x1) / denominator)
        }
    }

    fn ws18_inverse_interpolate(
        xs: &[f64],
        ys: &[f64],
        given: f64,
        increasing: bool,
    ) -> Option<f64> {
        if xs.len() != ys.len() || xs.len() < 2 {
            return None;
        }

        for index in 1..xs.len() {
            let y0 = ys[index - 1];
            let y1 = ys[index];
            let in_range = if increasing {
                given >= y0 && given <= y1
            } else {
                given <= y0 && given >= y1
            };
            if in_range {
                return Some(Self::ws18_linear_interpolate(
                    y0,
                    xs[index - 1],
                    y1,
                    xs[index],
                    given,
                ));
            }
        }

        None
    }

    fn ws18_shield_parameter(reyn: f64) -> f64 {
        if reyn <= WS10_ZERO_THRESHOLD {
            return WS18_SHIELD_VALUES[0];
        }

        let reynolds = reyn.ln();
        if reyn < WS18_SHIELD_REYNOLDS[0] {
            let i = 1;
            let slope = (WS18_SHIELD_VALUES[i].ln() - WS18_SHIELD_VALUES[i - 1].ln())
                / (WS18_SHIELD_REYNOLDS[i].ln() - WS18_SHIELD_REYNOLDS[i - 1].ln());
            let ycr =
                WS18_SHIELD_VALUES[0].ln() - (slope * (WS18_SHIELD_REYNOLDS[0].ln() - reynolds));
            return ycr.exp();
        }

        if reyn > WS18_SHIELD_REYNOLDS[WS18_SHIELD_REYNOLDS.len() - 1] {
            let i = WS18_SHIELD_REYNOLDS.len() - 1;
            let slope = (WS18_SHIELD_VALUES[i].ln() - WS18_SHIELD_VALUES[i - 1].ln())
                / (WS18_SHIELD_REYNOLDS[i].ln() - WS18_SHIELD_REYNOLDS[i - 1].ln());
            let ycr = WS18_SHIELD_VALUES[i] + (slope * (reynolds - WS18_SHIELD_REYNOLDS[i].ln()));
            return ycr.exp();
        }

        for i in 1..WS18_SHIELD_REYNOLDS.len() {
            if reyn >= WS18_SHIELD_REYNOLDS[i - 1] && reyn <= WS18_SHIELD_REYNOLDS[i] {
                let slope = (WS18_SHIELD_VALUES[i].ln() - WS18_SHIELD_VALUES[i - 1].ln())
                    / (WS18_SHIELD_REYNOLDS[i].ln() - WS18_SHIELD_REYNOLDS[i - 1].ln());
                let ycr = WS18_SHIELD_VALUES[i - 1].ln()
                    + (slope * (reynolds - WS18_SHIELD_REYNOLDS[i - 1].ln()));
                return ycr.exp();
            }
        }

        WS18_SHIELD_VALUES[WS18_SHIELD_VALUES.len() - 1]
    }

    #[allow(clippy::many_single_char_names, clippy::too_many_arguments)]
    fn ws18_hydchn(
        node_class: Ws10NodeClass,
        flagc: i32,
        q_cfs: f64,
        sf: f64,
        c1: f64,
        z: f64,
        wb: f64,
        n: f64,
        crsh: f64,
        nbarch: f64,
    ) -> Result<(f64, f64), Ws10GuardError> {
        if q_cfs <= WS10_ZERO_THRESHOLD {
            return Ok((0.0, 0.0));
        }
        if sf <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws18_hydchn_sf"),
                sf,
            ));
        }
        if n <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws18_hydchn_n"),
                n,
            ));
        }
        if nbarch <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws18_hydchn_nbarch"),
                nbarch,
            ));
        }
        if crsh <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws18_hydchn_crsh"),
                crsh,
            ));
        }

        let mut flag = flagc;
        let mut n_total = n;
        for _ in 0..8 {
            let args = Ws18HydchnArgs {
                node_class,
                flag,
                q_cfs,
                sf,
                c1,
                z,
                wb,
                n_total,
                crsh,
                nbarch,
            };
            let geometry = match Self::ws18_hydchn_geometry(args)? {
                Ws18HydchnStep::Geometry(geometry) => geometry,
                Ws18HydchnStep::Reclassify(next_flag) => {
                    flag = next_flag;
                    continue;
                }
            };

            let wetted_area = geometry.a.max(1.0e-10);
            let velocity = q_cfs / wetted_area;
            let rsh = (velocity * nbarch / (1.49 * sf.sqrt())).powf(1.5);
            let rcov = (velocity * (geometry.nt - nbarch) / (1.49 * sf.sqrt())).powf(1.5);
            let effsh = WS18_WTDH2O * rsh * sf;
            let mulsh = WS18_WTDH2O * rcov * sf;
            if mulsh < WS18_COVSH {
                return Ok((geometry.w, effsh.max(0.0)));
            }

            n_total = nbarch;
        }

        Err(Self::domain_violation(
            node_class,
            BoundarySymbol::from("ws18_hydchn_iteration_limit"),
            f64::from(flag),
        ))
    }

    fn ws18_hydchn_geometry(args: Ws18HydchnArgs) -> Result<Ws18HydchnStep, Ws10GuardError> {
        let ap = (args.q_cfs * args.n_total / (1.49 * args.sf.sqrt())).powf(0.375);
        if args.flag == 2 {
            return Self::ws18_hydchn_flag2_geometry(args, ap);
        }
        if args.flag >= 3 {
            return Self::ws18_hydchn_natural_geometry(args);
        }
        Self::ws18_hydchn_triangular_geometry(args, ap)
    }

    fn ws18_hydchn_triangular_geometry(
        args: Ws18HydchnArgs,
        ap: f64,
    ) -> Result<Ws18HydchnStep, Ws10GuardError> {
        if args.c1 <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                args.node_class,
                BoundarySymbol::from("ws18_hydchn_c1"),
                args.c1,
            ));
        }
        let y = ap / args.c1.powf(0.375);
        let w = 2.0 * y * args.z;
        let a = args.z * y * y;
        Ok(Ws18HydchnStep::Geometry(Ws18HydchnGeometry {
            w,
            a,
            nt: args.n_total,
        }))
    }

    fn ws18_hydchn_flag2_geometry(
        args: Ws18HydchnArgs,
        ap: f64,
    ) -> Result<Ws18HydchnStep, Ws10GuardError> {
        if args.wb <= WS10_ZERO_THRESHOLD {
            return Self::ws18_hydchn_triangular_geometry(args, ap);
        }

        let w = args.wb;
        let hxb = (ap / w).powf(8.0 / 3.0);
        let xb = if hxb <= 0.114 {
            Self::ws18_hydchn_low_hxb_xb(hxb)
        } else {
            Self::ws18_inverse_interpolate(
                &WS18_HYDCHN_XXB,
                &WS18_HYDCHN_FHXB,
                hxb.min(9999.99),
                true,
            )
            .unwrap_or(WS18_HYDCHN_XXB[WS18_HYDCHN_XXB.len() - 1])
        };
        let denominator = (1.0 - (2.0 * xb)).max(WS10_ZERO_THRESHOLD);
        let y = w * xb / denominator;
        let a = y * w;
        Ok(Ws18HydchnStep::Geometry(Ws18HydchnGeometry {
            w,
            a,
            nt: args.n_total,
        }))
    }

    fn ws18_hydchn_low_hxb_xb(hxb: f64) -> f64 {
        let mut xbo = 0.2_f64;
        let mut xbn = xbo;
        for _ in 0..32 {
            let core = ((1.0 - (2.0 * xbo)) * hxb).max(0.0);
            xbn = core.powf(0.6);
            if xbn.abs() <= WS10_ZERO_THRESHOLD {
                xbn = 1.0e-10;
            }
            let dif = ((xbn - xbo) / xbn).abs();
            if dif <= 0.001 {
                break;
            }
            xbo = xbn;
        }
        xbn
    }

    fn ws18_hydchn_natural_geometry(
        args: Ws18HydchnArgs,
    ) -> Result<Ws18HydchnStep, Ws10GuardError> {
        let ap_natural = (args.q_cfs * args.nbarch / (1.49 * args.sf.sqrt())).powf(0.375);
        let glc = ap_natural * WS18_WTDH2O * args.sf / args.crsh;
        if glc <= 1.84866 {
            if args.wb <= WS10_ZERO_THRESHOLD {
                return Ok(Ws18HydchnStep::Reclassify(1));
            }
            return Ok(Ws18HydchnStep::Reclassify(2));
        }
        let lc = Self::ws18_inverse_interpolate(
            &WS18_HYDCHN_XLC,
            &WS18_HYDCHN_FGLC,
            glc.min(99_999.999),
            false,
        )
        .unwrap_or(WS18_HYDCHN_XLC[WS18_HYDCHN_XLC.len() - 1]);
        let rstar = (-0.34707 * (0.5 - lc).powi(3)) - (0.54213 * (0.5 - lc).powi(2))
            + (0.66383 * (0.5 - lc));
        if rstar <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                args.node_class,
                BoundarySymbol::from("ws18_hydchn_rstar"),
                rstar,
            ));
        }
        let w = (ap_natural / rstar.powf(0.625)) * (0.73 - (1.46 * lc));
        if w <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                args.node_class,
                BoundarySymbol::from("ws18_hydchn_w"),
                w,
            ));
        }
        let hxb = (ap_natural / w).powf(8.0 / 3.0);
        let xb = Self::ws18_inverse_interpolate(
            &WS18_HYDCHN_XXB,
            &WS18_HYDCHN_FHXB,
            hxb.min(9999.99),
            true,
        )
        .unwrap_or(WS18_HYDCHN_XXB[WS18_HYDCHN_XXB.len() - 1]);
        let denominator = (1.0 - (2.0 * xb)).max(WS10_ZERO_THRESHOLD);
        let y = w * xb / denominator;
        let a = y * w;
        Ok(Ws18HydchnStep::Geometry(Ws18HydchnGeometry {
            w,
            a,
            nt: args.nbarch,
        }))
    }

    #[allow(clippy::similar_names)]
    fn ws18_trncap(effsh: f64, qs: &[f64], crdia_ft: &[f64], crspg: &[f64]) -> Vec<f64> {
        let class_count = qs.len();
        if class_count == 0 || effsh <= 0.0 {
            return vec![0.0; class_count];
        }

        let mut state = Self::ws18_trncap_initial_state(effsh, qs, crdia_ft, crspg);
        let mut mycount = 0_u32;
        loop {
            if let Some(result) = Self::ws18_trncap_terminal_result(&state, &mut mycount) {
                return result;
            }
            Self::ws18_trncap_redistribute(&mut state);
        }
    }

    fn ws18_trncap_initial_state(
        effsh: f64,
        qs: &[f64],
        crdia_ft: &[f64],
        crspg: &[f64],
    ) -> Ws18TrncapState {
        let class_count = qs.len();
        let vstar = (effsh / WS18_MSDH2O).sqrt();
        let coef_base = vstar * WS18_AGRAV * WS18_MSDH2O;

        let mut coef = vec![0.0_f64; class_count];
        let mut delta = vec![0.0_f64; class_count];
        let mut p = vec![0.0_f64; class_count];
        let mut dltrat = vec![0.0_f64; class_count];
        let mut ws = vec![0.0_f64; class_count];
        let mut qs_local = vec![0.0_f64; class_count];

        for k in 0..class_count {
            coef[k] = coef_base * crdia_ft[k] * crspg[k];
            qs_local[k] = qs[k].max(1.0e-31);
        }

        let mut t = 0.0_f64;
        for k in 0..class_count {
            let reyn = vstar * crdia_ft[k] / WS18_KNVIS;
            let ycrit = Self::ws18_shield_parameter(reyn.max(1.0e-12));
            let mut delta_k =
                (vstar * vstar / ((crspg[k] - 1.0) * WS18_AGRAV * crdia_ft[k] * ycrit)) - 1.0;
            if delta_k <= 0.0 || !delta_k.is_finite() {
                delta_k = 0.0;
                p[k] = 0.0;
            } else {
                let sigma = delta_k * 2.45 * crspg[k].powf(-0.4) * ycrit.sqrt();
                if sigma <= WS10_ZERO_THRESHOLD {
                    p[k] = 0.0;
                } else {
                    p[k] = WS18_YALCON * delta_k * (1.0 - ((1.0 / sigma) * (1.0 + sigma).ln()));
                }
            }
            delta[k] = delta_k;
            t += delta_k;
        }

        if t == 0.0 {
            t = 1000.0;
        }

        for k in 0..class_count {
            dltrat[k] = delta[k] / t;
            ws[k] = p[k] * dltrat[k] * coef[k];
        }

        Ws18TrncapState {
            coef,
            p,
            dltrat,
            ws,
            qs_local,
        }
    }

    #[allow(clippy::needless_range_loop)]
    fn ws18_trncap_terminal_result(
        state: &Ws18TrncapState,
        mycount: &mut u32,
    ) -> Option<Vec<f64>> {
        let class_count = state.qs_local.len();
        let mut flagd1 = 0_usize;
        let mut flagd2 = 0_usize;
        let mut flagd3 = 0_usize;
        let mut wsqrat = vec![0.0_f64; class_count];

        for k in 0..class_count {
            if state.qs_local[k] > 0.0 {
                wsqrat[k] = state.ws[k] / state.qs_local[k];
                if wsqrat[k] > 1.0 {
                    flagd3 += 1;
                }
                if wsqrat[k] >= 1.0 {
                    flagd1 += 1;
                }
                if wsqrat[k] <= 1.0 {
                    flagd2 += 1;
                }
            }
        }

        if flagd2 == class_count || flagd3 == class_count {
            return Some(state.ws.clone());
        }

        if flagd3 != class_count {
            *mycount += 1;
            if *mycount > 20 || flagd1 == class_count {
                let mut smdrat = 0.0_f64;
                for k in 0..class_count {
                    let denominator = state.coef[k] * state.p[k];
                    if denominator > WS10_ZERO_THRESHOLD {
                        smdrat += state.qs_local[k] / denominator;
                    }
                }
                let a = if smdrat > WS10_ZERO_THRESHOLD {
                    let mut scale = 1.0 / smdrat;
                    if scale > 0.999_99 && scale < 1.000_009_9 {
                        scale = 1.0;
                    }
                    scale
                } else {
                    1.0
                };

                return Some(state.qs_local.iter().map(|value| a * value).collect());
            }
        }

        None
    }

    #[allow(clippy::similar_names)]
    fn ws18_trncap_redistribute(state: &mut Ws18TrncapState) {
        let class_count = state.qs_local.len();
        let mut smdrqt = 0.0_f64;
        let mut smdrat = 0.0_f64;

        for k in 0..class_count {
            let ratio = if state.qs_local[k] > 0.0 {
                state.ws[k] / state.qs_local[k]
            } else {
                0.0
            };
            if ratio >= 1.0 {
                let denominator = state.coef[k] * state.p[k];
                if denominator > WS10_ZERO_THRESHOLD {
                    smdrqt += state.qs_local[k] / denominator;
                }
                state.ws[k] = state.qs_local[k];
            } else {
                smdrat += state.dltrat[k];
            }
        }

        let excap = 1.0 - smdrqt;
        let smdrat_guard = if smdrat.abs() <= WS10_ZERO_THRESHOLD {
            1_000_000.0
        } else {
            smdrat
        };
        for k in 0..class_count {
            let ratio = if state.qs_local[k] > 0.0 {
                state.ws[k] / state.qs_local[k]
            } else {
                0.0
            };
            if ratio < 1.0 {
                state.ws[k] =
                    state.dltrat[k] / smdrat_guard * excap * state.p[k] * state.coef[k];
            }
        }
    }

}
