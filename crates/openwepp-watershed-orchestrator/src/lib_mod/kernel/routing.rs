#[derive(Debug, Clone)]
struct Ws20ChannelProfile {
    x_points_ft: Vec<f64>,
    slopes: Vec<f64>,
    depth_a_points_ft: Vec<f64>,
    depth_b_points_ft: Vec<f64>,
    width_a_points_ft: Vec<f64>,
    width_b_points_ft: Vec<f64>,
}

#[derive(Debug, Clone)]
struct Ws20ClassTransportState {
    gstu_lbs_s: Vec<f64>,
    dlat_lbs_s_ft: Vec<f64>,
    crdia_ft: Vec<f64>,
    crspg: Vec<f64>,
    fall_ft_s: Vec<f64>,
}

#[derive(Debug, Clone)]
struct Ws20SegmentHydraulics {
    segment_index: usize,
    x_upper_ft: f64,
    x_lower_ft: f64,
    dx_ft: f64,
    qu_cfs: f64,
    ql_cfs: f64,
    wfu_ft: f64,
    wfl_ft: f64,
    effshu: f64,
    effshl: f64,
    upper_flagc: i32,
    lower_flagc: i32,
}

#[derive(Debug, Clone)]
struct Ws20TransportSnapshot {
    gsu_lbs_s_ft: Vec<f64>,
    tcu_lbs_s_ft: Vec<f64>,
    potld_lbs_s_ft: Vec<f64>,
    tcl_lbs_s_ft: Vec<f64>,
    dtcdx_lbs_s_ft2: Vec<f64>,
    phi: Vec<f64>,
    excess: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ws20Case12ClassKind {
    Case1,
    Case2,
}

#[derive(Debug, Clone, Copy)]
struct Ws20Case12ClassUpdate {
    next_flux_lbs_s: f64,
    xde_ft: f64,
    gstde_lbs_s: f64,
    case_kind: Ws20Case12ClassKind,
}

#[derive(Debug, Clone, Copy)]
struct Ws20RouteContext<'a> {
    node_class: Ws10NodeClass,
    ws21_case34_enabled: bool,
    event_duration: f64,
    roughness: f64,
    sediment_controls: Ws15ChannelSedimentControls,
    class_numbers: &'a [usize],
    qu_top_cfs: f64,
    qlat_cfs_per_ft: f64,
    flagct: i32,
    crsh: f64,
    chnk: f64,
}

#[derive(Debug, Clone, Copy)]
struct Ws18HydchnArgs {
    node_class: Ws10NodeClass,
    flag: i32,
    q_cfs: f64,
    sf: f64,
    c1: f64,
    z: f64,
    wb: f64,
    n_total: f64,
    crsh: f64,
    nbarch: f64,
}

#[derive(Debug, Clone, Copy)]
struct Ws18HydchnGeometry {
    w: f64,
    a: f64,
    nt: f64,
}

#[derive(Debug, Clone, Copy)]
enum Ws18HydchnStep {
    Geometry(Ws18HydchnGeometry),
    Reclassify(i32),
}

#[derive(Debug, Clone)]
struct Ws18TrncapState {
    coef: Vec<f64>,
    p: Vec<f64>,
    dltrat: Vec<f64>,
    ws: Vec<f64>,
    qs_local: Vec<f64>,
}

#[derive(Debug, Clone, Copy)]
struct Ws26DcapInput<'a> {
    node_class: Ws10NodeClass,
    flagm: i32,
    q_cfs: f64,
    sf: f64,
    c1: f64,
    z: f64,
    effsh: f64,
    depsid: f64,
    wflow: f64,
    roughness: f64,
    crsh: f64,
    excess: f64,
    tb: f64,
    flagt: i32,
    chnk: f64,
    nbarch: f64,
    maxe: f64,
    crfrac: &'a [f64],
}

#[derive(Debug, Clone)]
struct Ws26DcapLayerState {
    df: Vec<f64>,
    depmid: f64,
    werod: f64,
    timpot: f64,
    timsh: f64,
    di: f64,
}

#[derive(Debug, Clone)]
enum Ws26DcapLayerStep {
    Complete(Ws26DcapOutcome),
    Continue(Ws26DcapLayerState),
}

#[derive(Debug, Clone, Copy)]
struct Ws23DetachInput<'a> {
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
    crfrac: &'a [f64],
    gstu_lbs_s: &'a [f64],
    dlat_lbs_s_ft: &'a [f64],
    du_lbs_s_ft: &'a [f64],
    dx_ft: f64,
    crdia_ft: &'a [f64],
    crspg: &'a [f64],
}

#[derive(Debug, Clone)]
struct Ws23DetachWorking {
    dcap_outcome: Ws26DcapOutcome,
    df_lbs_s_ft2: Vec<f64>,
    dl_lbs_s_ft: Vec<f64>,
    potld_lbs_s_ft: Vec<f64>,
    tcl_lbs_s_ft: Vec<f64>,
}

#[derive(Debug, Clone)]
enum Ws23DetachStart {
    Complete(Ws23DetachClosureOutcome),
    Iterate(Ws23DetachWorking),
}

#[derive(Debug, Clone, Copy)]
struct Ws23DetachSums {
    sumtcl: f64,
    sumpld: f64,
    sumdf: f64,
    sumexd: f64,
}

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

    #[allow(clippy::similar_names)]
    fn ws20_load_channel_profile(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
        nslpts: usize,
    ) -> Result<Ws20ChannelProfile, Ws10GuardError> {
        let node_id = request.node_id;
        let mut profile = Ws20ChannelProfile {
            x_points_ft: Vec::with_capacity(nslpts),
            slopes: Vec::with_capacity(nslpts),
            depth_a_points_ft: Vec::with_capacity(nslpts),
            depth_b_points_ft: Vec::with_capacity(nslpts),
            width_a_points_ft: Vec::with_capacity(nslpts),
            width_b_points_ft: Vec::with_capacity(nslpts),
        };

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

            let x_ft =
                Self::require_channel_state_symbol_scalar(request, node_class, x_symbol.clone())?;
            let slope = Self::require_channel_state_symbol_scalar(
                request,
                node_class,
                slope_symbol.clone(),
            )?;
            let depth_a_ft = Self::require_channel_state_symbol_scalar(
                request,
                node_class,
                depth_a_symbol.clone(),
            )?;
            let depth_b_ft = Self::require_channel_state_symbol_scalar(
                request,
                node_class,
                depth_b_symbol.clone(),
            )?;
            let width_a_ft = Self::require_channel_state_symbol_scalar(
                request,
                node_class,
                width_a_symbol.clone(),
            )?;
            let width_b_ft = Self::require_channel_state_symbol_scalar(
                request,
                node_class,
                width_b_symbol.clone(),
            )?;

            Self::require_channel_control_range(node_class, x_symbol, x_ft, Some(0.0), None)?;
            Self::require_channel_control_range(node_class, slope_symbol, slope, Some(0.0), None)?;
            Self::require_channel_control_range(
                node_class,
                depth_a_symbol,
                depth_a_ft,
                Some(0.0),
                None,
            )?;
            Self::require_channel_control_range(
                node_class,
                depth_b_symbol,
                depth_b_ft,
                Some(0.0),
                None,
            )?;
            Self::require_channel_control_range(
                node_class,
                width_a_symbol,
                width_a_ft,
                Some(WS10_ZERO_THRESHOLD),
                None,
            )?;
            Self::require_channel_control_range(
                node_class,
                width_b_symbol,
                width_b_ft,
                Some(WS10_ZERO_THRESHOLD),
                None,
            )?;

            profile.x_points_ft.push(x_ft);
            profile.slopes.push(slope.max(WS18_MIN_CHANNEL_SLOPE));
            profile.depth_a_points_ft.push(depth_a_ft);
            profile.depth_b_points_ft.push(depth_b_ft);
            profile.width_a_points_ft.push(width_a_ft);
            profile.width_b_points_ft.push(width_b_ft);
        }

        Ok(profile)
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

    fn ws20_channel_chnk(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
    ) -> Result<f64, Ws10GuardError> {
        let node_id = request.node_id;
        let chnk_symbol = BoundarySymbol::from(format!("ws10_channel_{node_id}_chnk"));
        let chnk =
            Self::require_channel_state_symbol_scalar(request, node_class, chnk_symbol.clone())?;
        Self::require_channel_control_range(node_class, chnk_symbol, chnk, Some(0.0), None)?;
        Ok(chnk)
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
        request: &WatershedKernelRequest<'_>,
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
                request,
                ctx,
                profile,
                state,
                diagnostics,
                &segment,
                &snapshot,
            );
        }

        Self::ws20_route_case12_segment(
            request,
            ctx,
            profile,
            state,
            diagnostics,
            &segment,
            &snapshot,
        )
    }

    fn ws20_segment_crfrac(
        request: &WatershedKernelRequest<'_>,
        ctx: &Ws20RouteContext<'_>,
    ) -> Result<Vec<f64>, Ws10GuardError> {
        if !ctx.ws21_case34_enabled {
            return Err(Self::domain_violation(
                ctx.node_class,
                BoundarySymbol::from("ws21_case34_enabled"),
                0.0,
            ));
        }
        Self::ws22_require_crfrac_vector(request, ctx.node_class, ctx.class_numbers)
    }

    #[allow(clippy::needless_range_loop, clippy::similar_names)]
    fn ws20_route_case34_segment(
        request: &WatershedKernelRequest<'_>,
        ctx: &Ws20RouteContext<'_>,
        profile: &mut Ws20ChannelProfile,
        state: &mut Ws20ClassTransportState,
        diagnostics: &mut Ws20SegmentRoutingDiagnostics,
        segment: &Ws20SegmentHydraulics,
        snapshot: &Ws20TransportSnapshot,
    ) -> Result<(), Ws10GuardError> {
        let crfrac = Self::ws20_segment_crfrac(request, ctx)?;
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
            &crfrac,
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
            &crfrac,
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
        request: &WatershedKernelRequest<'_>,
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
            request,
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
        request: &WatershedKernelRequest<'_>,
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

        let crfrac = Self::ws22_require_crfrac_vector(request, ctx.node_class, ctx.class_numbers)?;
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
            &crfrac,
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
    fn ws20_route_case12_segment_family(
        request: &WatershedKernelRequest<'_>,
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

        let mut profile = Self::ws20_load_channel_profile(request, node_class, nslpts)?;
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
            request.node_id,
            sediment_controls.ishape,
        )?;
        let crsh = sediment_controls.chntcr * WS15_CRSH_FROM_CHNTCR_SCALE;
        let chnk = Self::ws20_channel_chnk(request, node_class)?;
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
        };

        let mut diagnostics = Ws20SegmentRoutingDiagnostics::default();
        for segment_index in 1..nslpts {
            Self::ws20_route_one_segment(
                request,
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
