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

    #[allow(
        clippy::many_single_char_names,
        clippy::too_many_arguments,
        clippy::too_many_lines
    )]
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
            let ap = (q_cfs * n_total / (1.49 * sf.sqrt())).powf(0.375);
            let (w, a, nt) = if flag == 2 {
                if wb <= WS10_ZERO_THRESHOLD {
                    if c1 <= WS10_ZERO_THRESHOLD {
                        return Err(Self::domain_violation(
                            node_class,
                            BoundarySymbol::from("ws18_hydchn_c1"),
                            c1,
                        ));
                    }
                    let y = ap / c1.powf(0.375);
                    let w = 2.0 * y * z;
                    let a = z * y * y;
                    (w, a, n_total)
                } else {
                    let w = wb;
                    let hxb = (ap / w).powf(8.0 / 3.0);
                    let xb = if hxb <= 0.114 {
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
                    (w, a, n_total)
                }
            } else if flag >= 3 {
                let ap_natural = (q_cfs * nbarch / (1.49 * sf.sqrt())).powf(0.375);
                let glc = ap_natural * WS18_WTDH2O * sf / crsh;
                if glc <= 1.84866 {
                    if wb <= WS10_ZERO_THRESHOLD {
                        flag = 1;
                        continue;
                    }
                    flag = 2;
                    continue;
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
                        node_class,
                        BoundarySymbol::from("ws18_hydchn_rstar"),
                        rstar,
                    ));
                }
                let w = (ap_natural / rstar.powf(0.625)) * (0.73 - (1.46 * lc));
                if w <= WS10_ZERO_THRESHOLD {
                    return Err(Self::domain_violation(
                        node_class,
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
                (w, a, nbarch)
            } else {
                if c1 <= WS10_ZERO_THRESHOLD {
                    return Err(Self::domain_violation(
                        node_class,
                        BoundarySymbol::from("ws18_hydchn_c1"),
                        c1,
                    ));
                }
                let y = ap / c1.powf(0.375);
                let w = 2.0 * y * z;
                let a = z * y * y;
                (w, a, n_total)
            };

            let wetted_area = a.max(1.0e-10);
            let velocity = q_cfs / wetted_area;
            let rsh = (velocity * nbarch / (1.49 * sf.sqrt())).powf(1.5);
            let rcov = (velocity * (nt - nbarch) / (1.49 * sf.sqrt())).powf(1.5);
            let effsh = WS18_WTDH2O * rsh * sf;
            let mulsh = WS18_WTDH2O * rcov * sf;
            if mulsh < WS18_COVSH {
                return Ok((w, effsh.max(0.0)));
            }

            n_total = nbarch;
        }

        Err(Self::domain_violation(
            node_class,
            BoundarySymbol::from("ws18_hydchn_iteration_limit"),
            f64::from(flag),
        ))
    }

    #[allow(clippy::similar_names, clippy::too_many_lines)]
    fn ws18_trncap(effsh: f64, qs: &[f64], crdia_ft: &[f64], crspg: &[f64]) -> Vec<f64> {
        let class_count = qs.len();
        if class_count == 0 || effsh <= 0.0 {
            return vec![0.0; class_count];
        }

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

        let mut mycount = 0_u32;
        loop {
            let mut flagd1 = 0_usize;
            let mut flagd2 = 0_usize;
            let mut flagd3 = 0_usize;
            let mut wsqrat = vec![0.0_f64; class_count];

            for k in 0..class_count {
                if qs_local[k] > 0.0 {
                    wsqrat[k] = ws[k] / qs_local[k];
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
                return ws;
            }

            if flagd3 != class_count {
                mycount += 1;
                if mycount > 20 || flagd1 == class_count {
                    let mut smdrat = 0.0_f64;
                    for k in 0..class_count {
                        let denominator = coef[k] * p[k];
                        if denominator > WS10_ZERO_THRESHOLD {
                            smdrat += qs_local[k] / denominator;
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

                    return qs_local.iter().map(|value| a * value).collect();
                }
            }

            let mut smdrqt = 0.0_f64;
            let mut smdrat = 0.0_f64;

            for k in 0..class_count {
                let ratio = if qs_local[k] > 0.0 {
                    ws[k] / qs_local[k]
                } else {
                    0.0
                };
                if ratio >= 1.0 {
                    let denominator = coef[k] * p[k];
                    if denominator > WS10_ZERO_THRESHOLD {
                        smdrqt += qs_local[k] / denominator;
                    }
                    ws[k] = qs_local[k];
                } else {
                    smdrat += dltrat[k];
                }
            }

            let excap = 1.0 - smdrqt;
            let smdrat_guard = if smdrat.abs() <= WS10_ZERO_THRESHOLD {
                1_000_000.0
            } else {
                smdrat
            };
            for k in 0..class_count {
                let ratio = if qs_local[k] > 0.0 {
                    ws[k] / qs_local[k]
                } else {
                    0.0
                };
                if ratio < 1.0 {
                    ws[k] = dltrat[k] / smdrat_guard * excap * p[k] * coef[k];
                }
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
        clippy::too_many_lines,
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
        let mut df = vec![0.0; crfrac.len()];
        let mut depmid = depmid_input;
        let mut werod = werod_input;
        let into_outcome =
            |df_lbs_s_ft2: Vec<f64>, depmid_ft: f64, werod_ft: f64| Ws26DcapOutcome {
                df_lbs_s_ft2,
                depmid_ft,
                werod_ft,
            };
        if effsh <= crsh {
            return Ok(into_outcome(df, depmid, werod));
        }

        let mut timpot = 0.0_f64;
        let mut timsh = tb * (1.0 - (crsh / effsh));
        let mut di = 0.0_f64;

        if depmid > WS10_ZERO_THRESHOLD {
            if flagt == 3 {
                werod = wflow;
            } else {
                let (wtmp, _) = Self::ws18_hydchn(
                    node_class,
                    4,
                    q_cfs,
                    sf.max(WS22_DCAP_MIN_SLOPE),
                    c1,
                    z,
                    wflow,
                    roughness,
                    crsh,
                    nbarch,
                )?;
                werod = wtmp;
            }

            let difsh = effsh - crsh;
            if difsh <= 0.0 {
                return Ok(into_outcome(df, depmid, werod));
            }

            di = excess * chnk * difsh;
            if di <= WS10_ZERO_THRESHOLD {
                return Ok(into_outcome(df, depmid, werod));
            }

            timpot = depmid * WS22_DCAP_WTDSOI / di;
            if timpot >= timsh {
                let mut dct = di * timsh * werod / (tb * wflow);
                if flagm != 1 && dct >= maxe {
                    di *= maxe / dct;
                    dct = maxe;
                }
                for class_offset in 0..crfrac.len() {
                    df[class_offset] = dct * crfrac[class_offset];
                }
                depmid -= di * timsh / WS22_DCAP_WTDSOI;
                if depmid < 0.005 {
                    depmid = 0.0;
                }
                return Ok(into_outcome(df, depmid, werod));
            }
        }

        let timex = timsh - timpot;
        let ab = q_cfs * roughness / (1.49 * sf.max(WS22_DCAP_MIN_SLOPE).sqrt());

        if werod <= WS10_ZERO_THRESHOLD {
            let (wtmp, _) = Self::ws18_hydchn(
                node_class,
                4,
                q_cfs,
                sf.max(WS22_DCAP_MIN_SLOPE),
                c1,
                z,
                wflow,
                roughness,
                crsh,
                nbarch,
            )?;
            werod = wtmp;
        }

        let hxb = ab / werod.powf(8.0 / 3.0);
        let Some(xb) = Self::ws22_table_column2_to_column1(
            &WS18_HYDCHN_XXB,
            &WS18_HYDCHN_FHXB,
            hxb.min(9999.99),
            true,
        ) else {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws22_dcap_hxb"),
                hxb,
            ));
        };

        let difsh = effsh * Self::ws22_shdist(xb) - crsh;
        if difsh <= 0.0 {
            if depmid <= 0.0 {
                return Ok(into_outcome(df, depmid, werod));
            }
            timsh = timpot;
            if di <= WS10_ZERO_THRESHOLD {
                return Ok(into_outcome(df, depmid, werod));
            }
            let mut dct = di * timsh * werod / (tb * wflow);
            if flagm != 1 && dct >= maxe {
                dct = maxe;
            }
            for class_offset in 0..crfrac.len() {
                df[class_offset] = dct * crfrac[class_offset];
            }
            return Ok(into_outcome(df, depmid, werod));
        }

        let dwdti = excess * 2.0 * chnk * difsh / WS22_DCAP_WTDSOI;
        let ad = ab.powf(0.375) * WS18_WTDH2O * sf.max(WS22_DCAP_MIN_SLOPE) / crsh;
        if ad <= WS22_DCAP_FFXCF[WS22_DCAP_FFXCF.len() - 1] {
            return Ok(into_outcome(df, depmid, werod));
        }

        let Some(xcf) = Self::ws22_table_column2_to_column1(
            &WS22_DCAP_XXCF,
            &WS22_DCAP_FFXCF,
            ad.min(999.999),
            false,
        ) else {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws22_dcap_ad"),
                ad,
            ));
        };

        if xcf <= WS10_ZERO_THRESHOLD || (1.0 - (2.0 * xcf)) <= WS10_ZERO_THRESHOLD {
            return Ok(into_outcome(df, depmid, werod));
        }
        let wfin_core = xcf * (1.0 - (2.0 * xcf)) / xcf.powf(8.0 / 3.0);
        if !wfin_core.is_finite() || wfin_core <= WS10_ZERO_THRESHOLD {
            return Ok(into_outcome(df, depmid, werod));
        }
        let wfin = ab.powf(0.375) * wfin_core.powf(0.375);
        if wfin <= werod {
            return Ok(into_outcome(df, depmid, werod));
        }

        let tstar = timex * dwdti / (wfin - werod);
        let wstar = (1.0 - (-1.0176 * tstar).exp()) / 1.0176;
        let we = wstar * (wfin - werod) + werod;
        let eros = (we - werod) * depsid + depmid * werod;
        let mut dct = eros * WS22_DCAP_WTDSOI / (tb * wflow);
        if flagm != 1 && dct >= maxe {
            dct = maxe;
        }

        for class_offset in 0..crfrac.len() {
            df[class_offset] = dct * crfrac[class_offset];
        }
        Ok(into_outcome(df, depmid, we))
    }

    #[allow(
        clippy::too_many_arguments,
        clippy::many_single_char_names,
        clippy::too_many_lines,
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
        let class_count = gstu_lbs_s.len();
        if class_count == 0
            || dlat_lbs_s_ft.len() != class_count
            || du_lbs_s_ft.len() != class_count
            || crdia_ft.len() != class_count
            || crspg.len() != class_count
            || crfrac.len() != class_count
        {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws23_detach_class_cardinality"),
                f64::from(u32::try_from(class_count).unwrap_or(u32::MAX)),
            ));
        }

        if dx_ft <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws23_detach_dx_ft"),
                dx_ft,
            ));
        }

        let mut excess = 1.0_f64;
        let mut dcap_outcome = Self::ws26_dcap(
            node_class,
            1,
            ql_cfs,
            sfl,
            c1,
            z,
            effshl,
            depsid_ft,
            depmid_ft,
            wfl_ft,
            wfl_ft,
            roughness,
            crsh,
            excess,
            tb_s,
            flagc,
            chnk,
            nbarch,
            WS22_DCAP_MAXE,
            crfrac,
        )?;
        let mut df_lbs_s_ft2 = dcap_outcome.df_lbs_s_ft2;

        let mut dl_lbs_s_ft = vec![0.0_f64; class_count];
        let mut potld_lbs_s_ft = vec![0.0_f64; class_count];
        let mut nt3 = 0_usize;
        for class_offset in 0..class_count {
            dl_lbs_s_ft[class_offset] = df_lbs_s_ft2[class_offset] * wfl_ft;
            potld_lbs_s_ft[class_offset] = (gstu_lbs_s[class_offset]
                + (dlat_lbs_s_ft[class_offset] * dx_ft)
                + ((dl_lbs_s_ft[class_offset] + du_lbs_s_ft[class_offset]) * dx_ft / 2.0))
                / wfl_ft;
            if dl_lbs_s_ft[class_offset].abs() <= WS10_ZERO_THRESHOLD
                && potld_lbs_s_ft[class_offset].abs() <= WS10_ZERO_THRESHOLD
            {
                nt3 += 1;
            }
        }

        let mut tcl_lbs_s_ft = vec![0.0_f64; class_count];
        if nt3 < class_count {
            tcl_lbs_s_ft = Self::ws18_trncap(effshl, &potld_lbs_s_ft, crdia_ft, crspg);
        }

        let nt2 = tcl_lbs_s_ft
            .iter()
            .zip(&potld_lbs_s_ft)
            .filter(|(tcl, potld)| **tcl >= **potld)
            .count();
        if nt2 == class_count || nt3 == class_count {
            let mut next_gstu_lbs_s = vec![0.0_f64; class_count];
            for class_offset in 0..class_count {
                next_gstu_lbs_s[class_offset] = potld_lbs_s_ft[class_offset] * wfl_ft;
            }
            return Ok(Ws23DetachClosureOutcome {
                next_gstu_lbs_s,
                werod_ft: dcap_outcome.werod_ft,
            });
        }

        let mut sumtcl = tcl_lbs_s_ft.iter().sum::<f64>();
        let mut sumpld = potld_lbs_s_ft.iter().sum::<f64>();
        if !sumtcl.is_finite() || !sumpld.is_finite() || sumpld.abs() <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws23_detach_sumpld"),
                sumpld,
            ));
        }
        excess = sumtcl / sumpld;
        let mut excold = excess;

        for _ in 0..20 {
            if excess < 0.0 {
                excess = 0.0;
            }

            dcap_outcome = Self::ws26_dcap(
                node_class,
                2,
                ql_cfs,
                sfl,
                c1,
                z,
                effshl,
                depsid_ft,
                depmid_ft,
                wfl_ft,
                wfl_ft,
                roughness,
                crsh,
                excess,
                tb_s,
                flagc,
                chnk,
                nbarch,
                WS22_DCAP_MAXE,
                crfrac,
            )?;
            df_lbs_s_ft2 = dcap_outcome.df_lbs_s_ft2;

            for class_offset in 0..class_count {
                dl_lbs_s_ft[class_offset] = df_lbs_s_ft2[class_offset] * wfl_ft;
                potld_lbs_s_ft[class_offset] = (gstu_lbs_s[class_offset]
                    + (dlat_lbs_s_ft[class_offset] * dx_ft)
                    + ((dl_lbs_s_ft[class_offset] + du_lbs_s_ft[class_offset]) * dx_ft / 2.0))
                    / wfl_ft;
            }
            tcl_lbs_s_ft = Self::ws18_trncap(effshl, &potld_lbs_s_ft, crdia_ft, crspg);

            let mut sumdf = 0.0_f64;
            let mut sumexd = 0.0_f64;
            sumtcl = 0.0;
            sumpld = 0.0;
            for class_offset in 0..class_count {
                sumtcl += tcl_lbs_s_ft[class_offset];
                sumpld += potld_lbs_s_ft[class_offset];
                let exdet = (((tcl_lbs_s_ft[class_offset] * wfl_ft)
                    - gstu_lbs_s[class_offset]
                    - (dlat_lbs_s_ft[class_offset] * dx_ft))
                    * (2.0 / dx_ft)
                    - du_lbs_s_ft[class_offset])
                    / wfl_ft;
                sumexd += exdet;
                sumdf += df_lbs_s_ft2[class_offset];
            }

            if !sumtcl.is_finite() || !sumpld.is_finite() {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("ws23_detach_sumtc_sumpl"),
                    sumtcl,
                ));
            }

            if sumtcl.abs() > WS10_ZERO_THRESHOLD && ((sumtcl - sumpld) / sumtcl).abs() < 0.01 {
                break;
            }

            let mut ratex = if sumdf.abs() > 1.0e-8 {
                sumexd / sumdf
            } else {
                sumtcl / sumpld
            };
            if !ratex.is_finite() || ratex <= 0.0 {
                ratex = sumtcl / sumpld;
            }
            excess = excold * ratex;
            excold = excess;
        }

        let mut next_gstu_lbs_s = vec![0.0_f64; class_count];
        for class_offset in 0..class_count {
            let next_flux = tcl_lbs_s_ft[class_offset] * wfl_ft;
            if !next_flux.is_finite() || next_flux < 0.0 {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("ws23_detach_next_flux"),
                    next_flux,
                ));
            }
            next_gstu_lbs_s[class_offset] = next_flux;
        }
        Ok(Ws23DetachClosureOutcome {
            next_gstu_lbs_s,
            werod_ft: dcap_outcome.werod_ft,
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

    #[allow(
        clippy::too_many_arguments,
        clippy::many_single_char_names,
        clippy::too_many_lines,
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
            return Ok(Ws20SegmentRoutingResult {
                routed_class_masses_kg: Vec::new(),
                diagnostics: Ws20SegmentRoutingDiagnostics::default(),
                widb_points_ft: Vec::new(),
                wida_points_ft: Vec::new(),
            });
        }

        let class_count = class_diameters_m.len();
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

        let node_id = request.node_id;
        let mut x_points_ft = Vec::with_capacity(nslpts);
        let mut slopes = Vec::with_capacity(nslpts);
        let mut depth_a_points_ft = Vec::with_capacity(nslpts);
        let mut depth_b_points_ft = Vec::with_capacity(nslpts);
        let mut width_a_points_ft = Vec::with_capacity(nslpts);
        let mut width_b_points_ft = Vec::with_capacity(nslpts);
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

            x_points_ft.push(x_ft);
            slopes.push(slope.max(WS18_MIN_CHANNEL_SLOPE));
            depth_a_points_ft.push(depth_a_ft);
            depth_b_points_ft.push(depth_b_ft);
            width_a_points_ft.push(width_a_ft);
            width_b_points_ft.push(width_b_ft);
        }

        let Some(&leff_ft) = x_points_ft.last() else {
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

        let mut gstu_lbs_s = vec![0.0_f64; class_count];
        let mut dlat_lbs_s_ft = vec![0.0_f64; class_count];
        let mut crdia_ft = vec![0.0_f64; class_count];
        let mut crspg = vec![0.0_f64; class_count];
        let mut fall_ft_s = vec![0.0_f64; class_count];
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

            gstu_lbs_s[class_offset] = top_flux;
            dlat_lbs_s_ft[class_offset] = lateral_flux / leff_ft;
            crdia_ft[class_offset] =
                class_diameters_m[class_offset] * WS15_DEPTH_FROM_METERS_TO_FEET;
            crspg[class_offset] = specific_gravity;
            fall_ft_s[class_offset] =
                Self::ws20_fall_velocity_ft_s(specific_gravity, crdia_ft[class_offset]);
        }

        let flagct =
            Self::ws30_shape_flag_from_ishape(node_class, node_id, sediment_controls.ishape)?;
        let crsh = sediment_controls.chntcr * WS15_CRSH_FROM_CHNTCR_SCALE;
        let chnk_symbol = BoundarySymbol::from(format!("ws10_channel_{node_id}_chnk"));
        let chnk =
            Self::require_channel_state_symbol_scalar(request, node_class, chnk_symbol.clone())?;
        Self::require_channel_control_range(node_class, chnk_symbol, chnk, Some(0.0), None)?;

        let mut diagnostics = Ws20SegmentRoutingDiagnostics::default();
        for segment_index in 1..nslpts {
            let x_upper_ft = x_points_ft[segment_index - 1];
            let x_lower_ft = x_points_ft[segment_index];
            let dx_ft = x_lower_ft - x_upper_ft;
            if dx_ft <= WS10_ZERO_THRESHOLD {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("ws20_dx_ft"),
                    dx_ft,
                ));
            }

            let qu_cfs = qu_top_cfs + (qlat_cfs_per_ft * x_upper_ft);
            let ql_cfs = qu_top_cfs + (qlat_cfs_per_ft * x_lower_ft);
            if !qu_cfs.is_finite() || qu_cfs < 0.0 {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("ws20_qu_cfs"),
                    qu_cfs,
                ));
            }
            if !ql_cfs.is_finite() || ql_cfs < 0.0 {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("ws20_ql_cfs"),
                    ql_cfs,
                ));
            }

            // Preserve baseline `chnrt.for` boundary-width semantics:
            // upper boundary uses `widb(i-1)`, lower boundary uses `wida(i)`.
            let upper_width_ft = width_b_points_ft[segment_index - 1];
            let lower_width_ft = width_a_points_ft[segment_index];
            let upper_flagc = Self::ws30_apply_erodible_rectangular_fallback(
                flagct,
                depth_b_points_ft[segment_index - 1],
            );
            let lower_flagc = Self::ws30_apply_erodible_rectangular_fallback(
                flagct,
                depth_a_points_ft[segment_index],
            );

            let (mut wfu_ft, mut effshu) = Self::ws18_hydchn(
                node_class,
                upper_flagc,
                qu_cfs,
                slopes[segment_index - 1],
                sediment_controls.ctlz,
                sediment_controls.chnz,
                upper_width_ft,
                roughness,
                crsh,
                sediment_controls.chnnbr,
            )?;
            let (mut wfl_ft, mut effshl) = Self::ws18_hydchn(
                node_class,
                lower_flagc,
                ql_cfs,
                slopes[segment_index],
                sediment_controls.ctlz,
                sediment_controls.chnz,
                lower_width_ft,
                roughness,
                crsh,
                sediment_controls.chnnbr,
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
                    node_class,
                    BoundarySymbol::from("ws20_width_ft"),
                    wfu_ft.min(wfl_ft),
                ));
            }

            let gsu_lbs_s_ft: Vec<f64> = gstu_lbs_s.iter().map(|flux| flux / wfu_ft).collect();
            let tcu_lbs_s_ft = Self::ws18_trncap(effshu, &gsu_lbs_s_ft, &crdia_ft, &crspg);

            let mut potld_lbs_s_ft = vec![0.0_f64; class_count];
            for class_offset in 0..class_count {
                potld_lbs_s_ft[class_offset] =
                    (gstu_lbs_s[class_offset] + (dlat_lbs_s_ft[class_offset] * dx_ft)) / wfl_ft;
            }
            let tcl_lbs_s_ft = Self::ws18_trncap(effshl, &potld_lbs_s_ft, &crdia_ft, &crspg);

            let mut dtcdx_lbs_s_ft2 = vec![0.0_f64; class_count];
            for class_offset in 0..class_count {
                dtcdx_lbs_s_ft2[class_offset] = ((tcl_lbs_s_ft[class_offset] * wfl_ft)
                    - (tcu_lbs_s_ft[class_offset] * wfu_ft))
                    / dx_ft;
            }

            let wfa_ft = 0.5 * (wfl_ft + wfu_ft);
            let qtemp_cfs_per_ft = if qlat_cfs_per_ft > WS10_ZERO_THRESHOLD {
                qlat_cfs_per_ft
            } else {
                0.0
            };
            let phi: Vec<f64> = if qtemp_cfs_per_ft > 0.0 {
                fall_ft_s
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

            if excess > 0.0 {
                if !ws21_case34_enabled {
                    return Err(Self::domain_violation(
                        node_class,
                        BoundarySymbol::from("ws21_case34_enabled"),
                        0.0,
                    ));
                }

                let crfrac = Self::ws22_require_crfrac_vector(request, node_class, class_numbers)?;
                let depsid_ft = sediment_controls.chneds * WS15_DEPTH_FROM_METERS_TO_FEET;
                let tb_s = 2.0 * event_duration;
                let mut depmid_ft = depth_b_points_ft[segment_index - 1];
                let dcap_outcome = Self::ws26_dcap(
                    node_class,
                    1,
                    qu_cfs,
                    slopes[segment_index - 1].max(WS22_DCAP_MIN_SLOPE),
                    sediment_controls.ctlz,
                    sediment_controls.chnz,
                    effshu,
                    depsid_ft,
                    depmid_ft,
                    width_b_points_ft[segment_index - 1],
                    wfu_ft,
                    roughness,
                    crsh,
                    excess,
                    tb_s,
                    upper_flagc,
                    chnk,
                    sediment_controls.chnnbr,
                    WS22_DCAP_MAXE,
                    &crfrac,
                )?;
                let dcap_df_lbs_s_ft2 = dcap_outcome.df_lbs_s_ft2;
                depmid_ft = dcap_outcome.depmid_ft;
                depth_b_points_ft[segment_index - 1] = dcap_outcome.depmid_ft;
                if upper_flagc == 2 && dcap_outcome.werod_ft > wfu_ft {
                    width_b_points_ft[segment_index - 1] = dcap_outcome.werod_ft;
                }

                let mut du_lbs_s_ft = vec![0.0_f64; class_count];
                for class_offset in 0..class_count {
                    du_lbs_s_ft[class_offset] = dcap_df_lbs_s_ft2[class_offset] * wfu_ft;
                }

                let case3_segment = tcl_lbs_s_ft
                    .iter()
                    .zip(&potld_lbs_s_ft)
                    .all(|(tcl, potld)| *tcl <= *potld);

                if case3_segment {
                    diagnostics.case3_segments = diagnostics.case3_segments.saturating_add(1);

                    let mut xdbeg_ft = vec![x_upper_ft; class_count];
                    let nz = du_lbs_s_ft
                        .iter()
                        .filter(|value| **value > WS10_ZERO_THRESHOLD)
                        .count();
                    let nk = gsu_lbs_s_ft
                        .iter()
                        .zip(&tcu_lbs_s_ft)
                        .filter(|(gsu, tcu)| (**gsu - **tcu).abs() <= WS10_ZERO_THRESHOLD)
                        .count();
                    let all_detaching = nz == class_count && nk == class_count;

                    for class_offset in 0..class_count {
                        if tcl_lbs_s_ft[class_offset] < potld_lbs_s_ft[class_offset] {
                            let denxdb = if all_detaching {
                                (2.0 * dlat_lbs_s_ft[class_offset]) + du_lbs_s_ft[class_offset]
                            } else {
                                (du_lbs_s_ft[class_offset] / 2.0) + dlat_lbs_s_ft[class_offset]
                                    - dtcdx_lbs_s_ft2[class_offset]
                            };

                            if denxdb.is_finite() && denxdb.abs() > WS10_ZERO_THRESHOLD {
                                xdbeg_ft[class_offset] = if all_detaching {
                                    ((dx_ft * du_lbs_s_ft[class_offset]) / denxdb) + x_upper_ft
                                } else {
                                    (((tcu_lbs_s_ft[class_offset] * wfu_ft)
                                        - gstu_lbs_s[class_offset])
                                        / denxdb)
                                        + x_upper_ft
                                };
                            }
                        }
                    }

                    let mut next_gstu_lbs_s = vec![0.0_f64; class_count];
                    for class_offset in 0..class_count {
                        let next_flux =
                            if potld_lbs_s_ft[class_offset] <= tcl_lbs_s_ft[class_offset] {
                                potld_lbs_s_ft[class_offset] * wfl_ft
                            } else {
                                let xrat = if x_lower_ft.abs() <= WS10_ZERO_THRESHOLD {
                                    0.0
                                } else {
                                    xdbeg_ft[class_offset] / x_lower_ft
                                };

                                let dl_lbs_s_ft2 = if qlat_cfs_per_ft > WS10_ZERO_THRESHOLD {
                                    let denphi = 1.0 + phi[class_offset];
                                    if denphi.abs() <= WS10_ZERO_THRESHOLD || !denphi.is_finite() {
                                        0.0
                                    } else {
                                        (phi[class_offset] / denphi)
                                            * (dtcdx_lbs_s_ft2[class_offset]
                                                - dlat_lbs_s_ft[class_offset])
                                            * (1.0 - xrat.powf(1.0 + phi[class_offset]))
                                    }
                                } else {
                                    dtcdx_lbs_s_ft2[class_offset]
                                };

                                let dengsl = phi[class_offset] * wfl_ft;
                                let gsl_lbs_s_ft =
                                    if dengsl.abs() <= WS10_ZERO_THRESHOLD || !dengsl.is_finite() {
                                        tcl_lbs_s_ft[class_offset]
                                    } else {
                                        tcl_lbs_s_ft[class_offset]
                                            - (dl_lbs_s_ft2 * x_lower_ft / dengsl)
                                    };

                                gsl_lbs_s_ft * wfl_ft
                            };

                        if !next_flux.is_finite() || next_flux < 0.0 {
                            return Err(Self::domain_violation(
                                node_class,
                                BoundarySymbol::from(format!(
                                    "ws21_case3_next_flux_{:04}",
                                    class_numbers[class_offset]
                                )),
                                next_flux,
                            ));
                        }
                        next_gstu_lbs_s[class_offset] = next_flux;
                    }

                    gstu_lbs_s = next_gstu_lbs_s;
                    continue;
                }

                diagnostics.case4_segments = diagnostics.case4_segments.saturating_add(1);

                let mut potld_case4_lbs_s_ft = vec![0.0_f64; class_count];
                for class_offset in 0..class_count {
                    potld_case4_lbs_s_ft[class_offset] = (gstu_lbs_s[class_offset]
                        + (dlat_lbs_s_ft[class_offset] * dx_ft)
                        + (du_lbs_s_ft[class_offset] * dx_ft / 2.0))
                        / wfl_ft;
                }

                let mut tcl_case4_lbs_s_ft =
                    Self::ws18_trncap(effshl, &potld_case4_lbs_s_ft, &crdia_ft, &crspg);
                let nt_case4 = tcl_case4_lbs_s_ft
                    .iter()
                    .zip(&potld_case4_lbs_s_ft)
                    .filter(|(tcl, potld)| **tcl <= **potld)
                    .count();

                if nt_case4 < class_count {
                    let ws23_outcome = Self::ws23_detach_case4_iterative_closure(
                        node_class,
                        ql_cfs,
                        slopes[segment_index].max(WS22_DCAP_MIN_SLOPE),
                        sediment_controls.ctlz,
                        sediment_controls.chnz,
                        effshl,
                        depsid_ft,
                        depmid_ft,
                        wfl_ft,
                        roughness,
                        crsh,
                        tb_s,
                        lower_flagc,
                        chnk,
                        sediment_controls.chnnbr,
                        &crfrac,
                        &gstu_lbs_s,
                        &dlat_lbs_s_ft,
                        &du_lbs_s_ft,
                        dx_ft,
                        &crdia_ft,
                        &crspg,
                    )?;
                    if lower_flagc == 2 && ws23_outcome.werod_ft > wfl_ft {
                        width_a_points_ft[segment_index] = ws23_outcome.werod_ft;
                    }
                    gstu_lbs_s = ws23_outcome.next_gstu_lbs_s;
                    continue;
                }

                diagnostics.enddet_segments = diagnostics.enddet_segments.saturating_add(1);
                let _ = Self::ws27_case4_enddet_bracket_closure(
                    x_upper_ft,
                    x_lower_ft,
                    wfl_ft,
                    dx_ft,
                    &gstu_lbs_s,
                    &dlat_lbs_s_ft,
                    &du_lbs_s_ft,
                    &mut potld_case4_lbs_s_ft,
                    &mut tcl_case4_lbs_s_ft,
                    |potld| Self::ws18_trncap(effshl, potld, &crdia_ft, &crspg),
                );

                let mut next_gstu_lbs_s = vec![0.0_f64; class_count];
                for class_offset in 0..class_count {
                    let next_flux = tcl_case4_lbs_s_ft[class_offset] * wfl_ft;
                    if !next_flux.is_finite() || next_flux < 0.0 {
                        return Err(Self::domain_violation(
                            node_class,
                            BoundarySymbol::from(format!(
                                "ws21_case4_next_flux_{:04}",
                                class_numbers[class_offset]
                            )),
                            next_flux,
                        ));
                    }
                    next_gstu_lbs_s[class_offset] = next_flux;
                }

                gstu_lbs_s = next_gstu_lbs_s;
                continue;
            }

            let mut saw_case1 = false;
            let mut saw_case2 = false;
            let mut next_gstu_lbs_s = vec![0.0_f64; class_count];
            let mut xde_ft = vec![x_lower_ft; class_count];
            let mut gstde_lbs_s = vec![0.0_f64; class_count];
            let mut case12_nz = 0_usize;
            for class_offset in 0..class_count {
                let xrat = if x_lower_ft > WS10_ZERO_THRESHOLD {
                    x_upper_ft / x_lower_ft
                } else {
                    0.0
                };
                let du_lbs_s_ft2 = if qu_cfs > 1.0e-8 {
                    let candidate = (fall_ft_s[class_offset] * wfu_ft / qu_cfs)
                        * ((tcu_lbs_s_ft[class_offset] * wfu_ft) - gstu_lbs_s[class_offset]);
                    candidate.min(0.0)
                } else if segment_index == 1
                    && qu_cfs < 0.001
                    && dtcdx_lbs_s_ft2[class_offset] < dlat_lbs_s_ft[class_offset]
                {
                    let phi_k = phi[class_offset];
                    if phi_k > WS10_ZERO_THRESHOLD {
                        (phi_k / (1.0 + phi_k))
                            * (dtcdx_lbs_s_ft2[class_offset] - dlat_lbs_s_ft[class_offset])
                    } else {
                        0.0
                    }
                } else {
                    0.0
                };

                let expon = 1.0 + phi[class_offset];
                let mut dl_lbs_s_ft2 = if qlat_cfs_per_ft > WS10_ZERO_THRESHOLD {
                    let phi_k = phi[class_offset];
                    let numerator =
                        phi_k * (dtcdx_lbs_s_ft2[class_offset] - dlat_lbs_s_ft[class_offset]);
                    (numerator / (1.0 + phi_k)) * (1.0 - xrat.powf(expon))
                } else {
                    dtcdx_lbs_s_ft2[class_offset]
                };
                dl_lbs_s_ft2 += du_lbs_s_ft2 * xrat.powf(expon);

                let next_flux = if dl_lbs_s_ft2 <= 0.0 {
                    saw_case1 = true;
                    case12_nz = case12_nz.saturating_add(1);
                    let phi_k = phi[class_offset];
                    let gsl = if phi_k > WS10_ZERO_THRESHOLD {
                        tcl_lbs_s_ft[class_offset] - ((dl_lbs_s_ft2 * x_lower_ft / phi_k) / wfl_ft)
                    } else {
                        0.0
                    };
                    xde_ft[class_offset] = x_lower_ft;
                    gstde_lbs_s[class_offset] = gsl * wfl_ft;
                    gsl * wfl_ft
                } else {
                    saw_case2 = true;
                    let xde_value_ft = if du_lbs_s_ft2.abs() <= WS10_ZERO_THRESHOLD {
                        x_upper_ft
                    } else if qlat_cfs_per_ft > WS10_ZERO_THRESHOLD {
                        let den = dtcdx_lbs_s_ft2[class_offset] - dlat_lbs_s_ft[class_offset];
                        if den.abs() <= WS10_ZERO_THRESHOLD
                            || phi[class_offset] <= WS10_ZERO_THRESHOLD
                        {
                            x_upper_ft
                        } else {
                            let core = (1.0
                                - (((1.0 + phi[class_offset]) / phi[class_offset])
                                    * (du_lbs_s_ft2 / den)))
                                .abs();
                            x_upper_ft * core.powf(1.0 / (1.0 + phi[class_offset]))
                        }
                    } else if dtcdx_lbs_s_ft2[class_offset].abs() <= WS10_ZERO_THRESHOLD {
                        x_upper_ft
                    } else {
                        x_upper_ft * (1.0 - (du_lbs_s_ft2 / dtcdx_lbs_s_ft2[class_offset]))
                    };

                    let gstde_value_lbs_s = if du_lbs_s_ft2.abs() <= WS10_ZERO_THRESHOLD {
                        gstu_lbs_s[class_offset]
                    } else {
                        (dtcdx_lbs_s_ft2[class_offset] * (xde_value_ft - x_upper_ft))
                            + (tcu_lbs_s_ft[class_offset] * wfu_ft)
                    };
                    let gsl_lbs_s_ft = if (xde_value_ft - x_lower_ft).abs() > WS10_ZERO_THRESHOLD {
                        (gstde_value_lbs_s
                            + (dlat_lbs_s_ft[class_offset] * (x_lower_ft - xde_value_ft)))
                            / wfl_ft
                    } else {
                        tcl_lbs_s_ft[class_offset]
                    };
                    xde_ft[class_offset] = xde_value_ft;
                    gstde_lbs_s[class_offset] = gstde_value_lbs_s;

                    gsl_lbs_s_ft * wfl_ft
                };

                if !next_flux.is_finite() || next_flux < 0.0 {
                    return Err(Self::domain_violation(
                        node_class,
                        BoundarySymbol::from(format!(
                            "ws20_case12_next_flux_{:04}",
                            class_numbers[class_offset]
                        )),
                        next_flux,
                    ));
                }
                next_gstu_lbs_s[class_offset] = next_flux;
            }

            if ws21_case34_enabled && saw_case2 && case12_nz < class_count {
                let xdemax_ft = xde_ft.iter().copied().fold(x_upper_ft, f64::max);
                if xdemax_ft + WS10_ZERO_THRESHOLD < x_lower_ft {
                    let dx_remaining_ft = x_lower_ft - xdemax_ft;
                    let mut gstde_transition_lbs_s = gstde_lbs_s.clone();
                    for class_offset in 0..class_count {
                        gstde_transition_lbs_s[class_offset] +=
                            dlat_lbs_s_ft[class_offset] * (xdemax_ft - xde_ft[class_offset]);
                    }

                    let crfrac =
                        Self::ws22_require_crfrac_vector(request, node_class, class_numbers)?;
                    let depmid_ft = sediment_controls.chnedm * WS15_DEPTH_FROM_METERS_TO_FEET;
                    let depsid_ft = sediment_controls.chneds * WS15_DEPTH_FROM_METERS_TO_FEET;
                    let tb_s = 2.0 * event_duration;

                    let ws24_outcome = Self::ws24_case12_detach_transition_closure(
                        node_class,
                        ql_cfs,
                        slopes[segment_index].max(WS22_DCAP_MIN_SLOPE),
                        sediment_controls.ctlz,
                        sediment_controls.chnz,
                        effshl,
                        depsid_ft,
                        depmid_ft,
                        wfl_ft,
                        roughness,
                        crsh,
                        tb_s,
                        lower_flagc,
                        chnk,
                        sediment_controls.chnnbr,
                        &crfrac,
                        &gstde_transition_lbs_s,
                        &dlat_lbs_s_ft,
                        dx_remaining_ft,
                        &crdia_ft,
                        &crspg,
                    )?;
                    if lower_flagc == 2 && ws24_outcome.werod_ft > wfl_ft {
                        width_a_points_ft[segment_index] = ws24_outcome.werod_ft;
                    }
                    gstu_lbs_s = ws24_outcome.next_gstu_lbs_s;
                    diagnostics.ws24_case2_detach_segments =
                        diagnostics.ws24_case2_detach_segments.saturating_add(1);
                    if saw_case1 {
                        diagnostics.case1_segments = diagnostics.case1_segments.saturating_add(1);
                    }
                    if saw_case2 {
                        diagnostics.case2_segments = diagnostics.case2_segments.saturating_add(1);
                    }
                    continue;
                }
            }

            if saw_case1 {
                diagnostics.case1_segments = diagnostics.case1_segments.saturating_add(1);
            }
            if saw_case2 {
                diagnostics.case2_segments = diagnostics.case2_segments.saturating_add(1);
            }
            gstu_lbs_s = next_gstu_lbs_s;
        }

        let mut outgoing_class_mass_kg = vec![0.0_f64; class_count];
        for class_offset in 0..class_count {
            let class_number = class_numbers[class_offset];
            let mass_kg = gstu_lbs_s[class_offset] * event_duration / WS18_LBS_PER_KG;
            if !mass_kg.is_finite() || mass_kg < 0.0 {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from(format!("ws20_outgoing_mass_kg_{class_number:04}")),
                    mass_kg,
                ));
            }
            outgoing_class_mass_kg[class_offset] = mass_kg;
        }

        Ok(Ws20SegmentRoutingResult {
            routed_class_masses_kg: outgoing_class_mass_kg,
            diagnostics,
            widb_points_ft: width_b_points_ft,
            wida_points_ft: width_a_points_ft,
        })
    }


}
