#[allow(clippy::wildcard_imports)]
use crate::hydrology::*;

impl Wb11HydrologyKernel {
    #[allow(clippy::too_many_lines)]
pub(crate) fn run_erod14_wave2(
        request: &HillslopeKernelRequest<'_>,
        erod13_state_updates: &[WritebackField],
    ) -> Result<Vec<WritebackField>, Wb11HydrologyKernelGuardError> {
        if !Self::resolve_erod14_wave2_enabled(request)? {
            return Ok(Vec::new());
        }

        let class_count_symbol = BoundarySymbol::from(EROD14_SYMBOL_CLASS_COUNT);
        let class_count_value = Self::require_erod14_state_scalar(request, &class_count_symbol)?;
        if class_count_value < 1.0 - WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: class_count_symbol,
                value: class_count_value,
                minimum: Some(1.0),
                maximum: None,
            });
        }
        let class_count_rounded = class_count_value.round();
        if (class_count_value - class_count_rounded).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: BoundarySymbol::from(EROD14_SYMBOL_CLASS_COUNT),
                value: class_count_value,
                minimum: Some(1.0),
                maximum: None,
            });
        }
        let class_count = format!("{class_count_rounded:.0}")
            .parse::<usize>()
            .map_err(|_| Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: BoundarySymbol::from(EROD14_SYMBOL_CLASS_COUNT),
                value: class_count_value,
                minimum: Some(1.0),
                maximum: None,
            })?;
        if class_count == 0 {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: BoundarySymbol::from(EROD14_SYMBOL_CLASS_COUNT),
                value: class_count_value,
                minimum: Some(1.0),
                maximum: None,
            });
        }
        let class_count_f64 = f64::from(u32::try_from(class_count).map_err(|_| {
            Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: BoundarySymbol::from(EROD14_SYMBOL_CLASS_COUNT),
                value: class_count_value,
                minimum: Some(1.0),
                maximum: None,
            }
        })?);

        let xtop_symbol = BoundarySymbol::from(EROD14_SYMBOL_XTOP);
        let xbot_symbol = BoundarySymbol::from(EROD14_SYMBOL_XBOT);
        let xdetst_symbol = BoundarySymbol::from(EROD14_SYMBOL_XDETST);
        let ldtop_symbol = BoundarySymbol::from(EROD14_SYMBOL_LDTOP);
        let ldbot_symbol = BoundarySymbol::from(EROD14_SYMBOL_LDBOT);
        let lddend_symbol = BoundarySymbol::from(EROD14_SYMBOL_LDDEND);
        let qout_symbol = BoundarySymbol::from(EROD14_SYMBOL_QOUT);
        let qin_symbol = BoundarySymbol::from(EROD14_SYMBOL_QIN);
        let qostar_symbol = BoundarySymbol::from(EROD14_SYMBOL_QOSTAR);
        let slplen_symbol = BoundarySymbol::from(EROD14_SYMBOL_SLP_LEN);
        let ktrato_symbol = BoundarySymbol::from(EROD14_SYMBOL_KTRATO);
        let aintc_symbol = BoundarySymbol::from(EROD14_SYMBOL_AINTC);
        let bintc_symbol = BoundarySymbol::from(EROD14_SYMBOL_BINTC);
        let cintc_symbol = BoundarySymbol::from(EROD14_SYMBOL_CINTC);
        let beta_symbol = BoundarySymbol::from(EROD14_SYMBOL_BETA);
        let qj_minus_1_symbol = BoundarySymbol::from(EROD14_SYMBOL_QJ_MINUS_1);
        let vj_symbol = BoundarySymbol::from(EROD14_SYMBOL_VJ);
        let qj_symbol = BoundarySymbol::from(EROD14_SYMBOL_QJ);
        let fh_runon_symbol = BoundarySymbol::from(EROD14_SYMBOL_FH);
        let fp_potential_symbol = BoundarySymbol::from(EROD14_SYMBOL_FP);
        let case_symbol = BoundarySymbol::from(EROD14_SYMBOL_CASE);
        let sumg_symbol = BoundarySymbol::from(EROD14_SYMBOL_SUMG);
        let er_symbol = BoundarySymbol::from(EROD14_SYMBOL_ER);
        let ssa_soil_symbol = BoundarySymbol::from(EROD14_SYMBOL_SSA_SOIL);

        let xtop = Self::require_erod14_state_scalar(request, &xtop_symbol)?;
        let xbot = Self::require_erod14_state_scalar(request, &xbot_symbol)?;
        let xdetst = Self::require_erod14_state_scalar(request, &xdetst_symbol)?;
        let ldtop = Self::require_erod14_state_scalar(request, &ldtop_symbol)?;
        let ldbot = Self::require_erod14_state_scalar(request, &ldbot_symbol)?;
        let lddend = Self::require_erod14_state_scalar(request, &lddend_symbol)?;
        let qout = Self::require_erod14_state_scalar(request, &qout_symbol)?;
        let qin = Self::require_erod14_state_scalar(request, &qin_symbol)?;
        let qostar = Self::require_erod14_state_scalar(request, &qostar_symbol)?;
        let slplen = Self::require_erod14_state_scalar(request, &slplen_symbol)?;
        let ktrato = Self::require_erod14_state_scalar(request, &ktrato_symbol)?;
        let aintc = Self::require_erod14_state_scalar(request, &aintc_symbol)?;
        let bintc = Self::require_erod14_state_scalar(request, &bintc_symbol)?;
        let cintc = Self::require_erod14_state_scalar(request, &cintc_symbol)?;
        let beta = Self::require_erod14_state_scalar(request, &beta_symbol)?;
        let qj_minus_1 = Self::require_erod14_state_scalar(request, &qj_minus_1_symbol)?;
        let vj = Self::require_erod14_state_scalar(request, &vj_symbol)?;
        let qj = Self::require_erod14_state_scalar(request, &qj_symbol)?;
        let fh = Self::require_erod14_state_scalar(request, &fh_runon_symbol)?;
        let fp = Self::require_erod14_state_scalar(request, &fp_potential_symbol)?;
        let case_value = Self::require_erod14_state_scalar(request, &case_symbol)?;
        let ssa_soil = Self::require_erod14_state_scalar(request, &ssa_soil_symbol)?;

        Self::require_erod14_domain(&xtop_symbol, xtop, Some(0.0), None)?;
        Self::require_erod14_domain(&xbot_symbol, xbot, Some(xtop), None)?;
        Self::require_erod14_domain(&xdetst_symbol, xdetst, Some(0.0), Some(xtop))?;
        Self::require_erod14_domain(&ldtop_symbol, ldtop, Some(0.0), None)?;
        Self::require_erod14_domain(&ldbot_symbol, ldbot, Some(0.0), None)?;
        Self::require_erod14_domain(&lddend_symbol, lddend, Some(0.0), Some(ldtop))?;
        Self::require_erod14_domain(&qout_symbol, qout, Some(0.0), None)?;
        Self::require_erod14_domain(&qin_symbol, qin, Some(0.0), None)?;
        Self::require_erod14_domain(&slplen_symbol, slplen, Some(WB11_ZERO_THRESHOLD), None)?;
        Self::require_erod14_domain(&ktrato_symbol, ktrato, Some(WB11_ZERO_THRESHOLD), None)?;
        Self::require_erod14_domain(&beta_symbol, beta, Some(0.0), None)?;
        Self::require_erod14_domain(&qj_minus_1_symbol, qj_minus_1, Some(0.0), None)?;
        Self::require_erod14_domain(&vj_symbol, vj, Some(0.0), None)?;
        Self::require_erod14_domain(&qj_symbol, qj, Some(0.0), None)?;
        Self::require_erod14_domain(&fh_runon_symbol, fh, Some(0.0), None)?;
        Self::require_erod14_domain(&fp_potential_symbol, fp, Some(0.0), None)?;
        Self::require_erod14_domain(&ssa_soil_symbol, ssa_soil, Some(WB11_ZERO_THRESHOLD), None)?;

        let case_rounded = case_value.round();
        if (case_value - case_rounded).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: case_symbol,
                value: case_value,
                minimum: Some(f64::from(EROD14_CASE_MIN)),
                maximum: Some(f64::from(EROD14_CASE_MAX)),
            });
        }
        let case_number = format!("{case_rounded:.0}").parse::<i32>().map_err(|_| {
            Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: BoundarySymbol::from(EROD14_SYMBOL_CASE),
                value: case_value,
                minimum: Some(f64::from(EROD14_CASE_MIN)),
                maximum: Some(f64::from(EROD14_CASE_MAX)),
            }
        })?;
        if !(EROD14_CASE_MIN..=EROD14_CASE_MAX).contains(&case_number) {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: BoundarySymbol::from(EROD14_SYMBOL_CASE),
                value: case_value,
                minimum: Some(f64::from(EROD14_CASE_MIN)),
                maximum: Some(f64::from(EROD14_CASE_MAX)),
            });
        }

        let case_is_zero = |value: f64| value.abs() <= EROD14_CASE_TOLERANCE;
        let case_matches = match case_number {
            1 => case_is_zero(qj_minus_1) && case_is_zero(vj) && case_is_zero(qj),
            2 => {
                qj_minus_1 > EROD14_CASE_TOLERANCE
                    && vj > EROD14_CASE_TOLERANCE
                    && qj > EROD14_CASE_TOLERANCE
            }
            3 => {
                qj_minus_1 > EROD14_CASE_TOLERANCE
                    && case_is_zero(vj)
                    && (fh - fp) > EROD14_CASE_TOLERANCE
                    && qj > EROD14_CASE_TOLERANCE
            }
            4 => {
                qj_minus_1 > EROD14_CASE_TOLERANCE
                    && case_is_zero(vj)
                    && (fh - fp) <= EROD14_CASE_TOLERANCE
                    && case_is_zero(qj)
            }
            _ => false,
        };
        if !case_matches {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: BoundarySymbol::from(EROD14_SYMBOL_CASE),
                value: case_value,
                minimum: Some(f64::from(EROD14_CASE_MIN)),
                maximum: Some(f64::from(EROD14_CASE_MAX)),
            });
        }

        let theta_symbol = BoundarySymbol::from(EROD13_SYMBOL_THETA);
        let theta = if let Some(value) =
            Self::extract_state_update_scalar(erod13_state_updates, EROD13_SYMBOL_THETA)
        {
            if !value.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::Erod14NonFiniteSymbol {
                    symbol: theta_symbol,
                    value,
                });
            }
            value
        } else {
            Self::require_erod14_state_scalar(request, &theta_symbol)?
        };
        Self::require_erod14_domain(&theta_symbol, theta, Some(0.0), None)?;

        let mut fall = Vec::with_capacity(class_count);
        let mut frcflw = Vec::with_capacity(class_count);
        let mut fidel = Vec::with_capacity(class_count);
        let mut tcf1 = Vec::with_capacity(class_count);
        let mut ssa_class = Vec::with_capacity(class_count);
        let mut ftheta = Vec::with_capacity(class_count);
        let mut gu = Vec::with_capacity(class_count);
        let mut gend = vec![0.0_f64; class_count];
        let mut sedmax = vec![0.0_f64; class_count];
        let mut sed_frac = vec![0.0_f64; class_count];

        for class_index in 1..=class_count {
            let fall_symbol = Self::erod14_class_symbol(EROD14_ROOT_FALL, class_index);
            let frcflw_symbol = Self::erod14_class_symbol(EROD14_ROOT_FRCFLW, class_index);
            let frac_symbol = Self::erod14_class_symbol(EROD14_ROOT_FRAC, class_index);
            let fidel_symbol = Self::erod14_class_symbol(EROD14_ROOT_FIDEL, class_index);
            let tcf1_symbol = Self::erod14_class_symbol(EROD14_ROOT_TCF1, class_index);
            let ssa_class_symbol = Self::erod14_class_symbol(EROD14_ROOT_SSA_CLASS, class_index);

            let fall_value = Self::require_erod14_state_scalar(request, &fall_symbol)?;
            let frcflw_value = Self::require_erod14_state_scalar(request, &frcflw_symbol)?;
            let frac_value = Self::require_erod14_state_scalar(request, &frac_symbol)?;
            let fidel_value = Self::require_erod14_state_scalar(request, &fidel_symbol)?;
            let tcf1_value = Self::require_erod14_state_scalar(request, &tcf1_symbol)?;
            let ssa_class_value = Self::require_erod14_state_scalar(request, &ssa_class_symbol)?;

            Self::require_erod14_domain(&fall_symbol, fall_value, Some(0.0), None)?;
            Self::require_erod14_domain(&frcflw_symbol, frcflw_value, Some(0.0), Some(1.0))?;
            Self::require_erod14_domain(&frac_symbol, frac_value, Some(0.0), Some(1.0))?;
            Self::require_erod14_domain(&fidel_symbol, fidel_value, Some(0.0), Some(1.0))?;
            Self::require_erod14_domain(&tcf1_symbol, tcf1_value, Some(0.0), None)?;
            Self::require_erod14_domain(
                &ssa_class_symbol,
                ssa_class_value,
                Some(WB11_ZERO_THRESHOLD),
                None,
            )?;

            fall.push(fall_value);
            frcflw.push(frcflw_value);
            fidel.push(fidel_value);
            tcf1.push(tcf1_value);
            ssa_class.push(ssa_class_value);
            ftheta.push(fidel_value * theta);
            gu.push(frcflw_value * ldtop);
        }

        if qout <= WB11_ZERO_THRESHOLD {
            for i in 0..class_count {
                frcflw[i] = 0.0;
                sed_frac[i] = 0.0;
            }
            let mut updates = Vec::with_capacity(
                EROD14_BASE_UPDATE_FIELD_COUNT + (class_count * EROD14_CLASS_UPDATE_FIELD_COUNT),
            );
            updates.push(WritebackField::bounded(
                EROD14_SYMBOL_SUMG,
                0.0,
                Some(0.0),
                None,
            ));
            updates.push(WritebackField::bounded(
                EROD14_SYMBOL_ER,
                0.0,
                Some(0.0),
                None,
            ));
            updates.push(WritebackField::bounded(
                EROD15_SYMBOL_TOTAL_DETACHMENT_KG,
                0.0,
                Some(0.0),
                None,
            ));
            updates.push(WritebackField::bounded(
                EROD15_SYMBOL_TOTAL_DEPOSITION_KG,
                lddend.max(0.0),
                Some(0.0),
                None,
            ));
            updates.push(WritebackField::bounded(
                EROD15_SYMBOL_PARTICLE_CLASS_COUNT,
                class_count_f64,
                Some(1.0),
                None,
            ));
            for class_index in 1..=class_count {
                updates.push(WritebackField::bounded(
                    Self::erod14_class_symbol(EROD14_ROOT_GEND, class_index),
                    0.0,
                    Some(0.0),
                    None,
                ));
                updates.push(WritebackField::bounded(
                    Self::erod14_class_symbol(EROD14_ROOT_SEDMAX, class_index),
                    0.0,
                    Some(0.0),
                    None,
                ));
                updates.push(WritebackField::bounded(
                    Self::erod14_class_symbol(EROD14_ROOT_FRCFLW, class_index),
                    0.0,
                    Some(0.0),
                    Some(1.0),
                ));
                updates.push(WritebackField::bounded(
                    Self::erod14_class_symbol(EROD14_ROOT_SED_FRAC, class_index),
                    0.0,
                    Some(0.0),
                    Some(1.0),
                ));
                updates.push(WritebackField::bounded(
                    Self::erod14_class_symbol(
                        EROD15_ROOT_SEDIMENT_CONCENTRATION_KG_M3,
                        class_index,
                    ),
                    0.0,
                    Some(0.0),
                    None,
                ));
                updates.push(WritebackField::bounded(
                    Self::erod14_class_symbol(EROD15_ROOT_PARTICLE_FLOW_FRACTION, class_index),
                    0.0,
                    Some(0.0),
                    Some(1.0),
                ));
            }
            return Ok(updates);
        }

        let pkro = (qout - qin) / slplen;
        if !pkro.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: BoundarySymbol::from(EROD14_SYMBOL_QOUT),
                value: pkro,
                minimum: None,
                maximum: None,
            });
        }

        let tmpvr2 = xbot + qostar;
        let tmpvr3 = xtop + qostar;
        if tmpvr2.abs() <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: qostar_symbol,
                value: tmpvr2,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }
        let tmpvr4 = tmpvr2 * tmpvr2;
        let tmpvr5 = tmpvr3 * tmpvr3;

        let mut sumg = 0.0_f64;
        for i in 0..class_count {
            let tmpvr1 = ktrato * tcf1[i];
            let aa = tmpvr1 * aintc;
            let bb = tmpvr1 * bintc;
            let cc = tmpvr1 * cintc;

            let mut phi = if pkro.abs() > EROD14_PKRO_ZERO_THRESHOLD {
                (beta * fall[i]) / pkro
            } else if qostar >= 0.0 {
                EROD14_MAX_PHI
            } else {
                -EROD14_MAX_PHI
            };
            phi = phi.clamp(-EROD14_MAX_PHI, EROD14_MAX_PHI);

            let mut ratio = tmpvr3 / tmpvr2;
            if qostar >= 0.0 && ratio > 1.0 {
                ratio = 1.0;
            }
            if ratio < 0.0 {
                return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                    symbol: BoundarySymbol::from(EROD14_SYMBOL_QOSTAR),
                    value: ratio,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }

            let denom_coef1 = phi + 2.0;
            let denom_coef2 = phi + 1.0;
            if denom_coef1.abs() <= WB11_ZERO_THRESHOLD || denom_coef2.abs() <= WB11_ZERO_THRESHOLD
            {
                return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                    symbol: Self::erod14_class_symbol(EROD14_ROOT_FALL, i + 1),
                    value: phi,
                    minimum: Some(-EROD14_MAX_PHI),
                    maximum: Some(EROD14_MAX_PHI),
                });
            }

            let mut attenuation_factor = ratio.powf(phi);
            if !attenuation_factor.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                    symbol: Self::erod14_class_symbol(EROD14_ROOT_FALL, i + 1),
                    value: attenuation_factor,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }
            if attenuation_factor < EROD14_ATTENUATION_FLOOR {
                attenuation_factor = 0.0;
            }

            let coef1 = phi * aa / denom_coef1;
            let coef2 = (phi * bb + ftheta[i] - 2.0 * aa * phi * qostar) / denom_coef2;
            let term1 = coef1 * tmpvr4;
            let term2 = coef2 * tmpvr2;
            let term3 = aa * qostar * qostar - bb * qostar + cc;
            let attenuation_tail = gu[i] - coef1 * tmpvr5 - coef2 * tmpvr3 - term3;
            let mut gend_i = term1 + term2 + term3 + attenuation_factor * attenuation_tail;
            if !gend_i.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                    symbol: Self::erod14_class_symbol(EROD14_ROOT_GEND, i + 1),
                    value: gend_i,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }
            if gend_i < 0.0 {
                gend_i = 0.0;
            }
            gend[i] = gend_i;
            sumg += gend_i;
        }

        if sumg > 0.0 {
            for i in 0..class_count {
                gend[i] = gend[i] * ldbot / sumg;
                sedmax[i] = gu[i] + ftheta[i] * (xbot - xtop);
                Self::require_erod14_domain(
                    &Self::erod14_class_symbol(EROD14_ROOT_SEDMAX, i + 1),
                    sedmax[i],
                    Some(0.0),
                    None,
                )?;
                if gend[i] < EROD14_CLASS_FLOOR {
                    gend[i] = EROD14_CLASS_FLOOR;
                }
            }

            let mut converged = false;
            for _ in 0..EROD14_MAX_REPROPORTION_ITERS {
                let mut ratbot = 0.0_f64;
                sumg = 0.0;
                let mut adjusted = false;

                for i in 0..class_count {
                    if gend[i] > sedmax[i] + WB11_ZERO_THRESHOLD {
                        gend[i] = sedmax[i];
                        adjusted = true;
                    } else if gend[i] < sedmax[i] - WB11_ZERO_THRESHOLD {
                        ratbot += gend[i];
                    }
                    sumg += gend[i];
                }

                if !adjusted {
                    converged = true;
                    break;
                }

                // Baseline enrich.for semantics: when clipping saturates every class
                // (`ratbot == 0`), re-enter the clipping loop instead of failing.
                if ratbot <= WB11_ZERO_THRESHOLD {
                    continue;
                }

                let gdeficit = ldbot - sumg;
                for i in 0..class_count {
                    if gend[i] < sedmax[i] - WB11_ZERO_THRESHOLD {
                        let gadd = gdeficit * gend[i] / ratbot;
                        let updated = gend[i] + gadd;
                        if !updated.is_finite() {
                            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                                symbol: Self::erod14_class_symbol(EROD14_ROOT_GEND, i + 1),
                                value: updated,
                                minimum: Some(0.0),
                                maximum: None,
                            });
                        }
                        gend[i] = updated;
                    }
                }
            }

            if !converged {
                return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                    symbol: BoundarySymbol::from(EROD14_SYMBOL_LDBOT),
                    value: ldbot,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }
        }

        sumg = gend.iter().sum();
        if !sumg.is_finite() || sumg < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: sumg_symbol,
                value: sumg,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        for i in 0..class_count {
            if gend[i] > sedmax[i] + WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                    symbol: Self::erod14_class_symbol(EROD14_ROOT_GEND, i + 1),
                    value: gend[i],
                    minimum: Some(0.0),
                    maximum: Some(sedmax[i]),
                });
            }
        }

        if sumg > 0.0 {
            for i in 0..class_count {
                frcflw[i] = gend[i] / sumg;
                sed_frac[i] = frcflw[i];
            }
            let sed_frac_sum: f64 = sed_frac.iter().sum();
            if (sed_frac_sum - 1.0).abs() > EROD13_CONTINUITY_TOLERANCE + WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                    symbol: BoundarySymbol::from(EROD14_ROOT_SED_FRAC),
                    value: sed_frac_sum,
                    minimum: Some(1.0 - EROD13_CONTINUITY_TOLERANCE),
                    maximum: Some(1.0 + EROD13_CONTINUITY_TOLERANCE),
                });
            }
        } else {
            for i in 0..class_count {
                frcflw[i] = 0.0;
                sed_frac[i] = 0.0;
            }
        }

        let mut sumssa = 0.0_f64;
        for i in 0..class_count {
            sumssa += sed_frac[i] * ssa_class[i];
        }
        let er = if sumg > 0.0 {
            (sumssa / ssa_soil) + EROD14_ENRICHMENT_RATIO_OFFSET
        } else {
            0.0
        };
        if !er.is_finite() || er < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: er_symbol,
                value: er,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let mut updates = Vec::with_capacity(
            EROD14_BASE_UPDATE_FIELD_COUNT + (class_count * EROD14_CLASS_UPDATE_FIELD_COUNT),
        );
        updates.push(WritebackField::bounded(
            EROD14_SYMBOL_SUMG,
            sumg.max(0.0),
            Some(0.0),
            None,
        ));
        updates.push(WritebackField::bounded(
            EROD14_SYMBOL_ER,
            er,
            Some(0.0),
            None,
        ));
        updates.push(WritebackField::bounded(
            EROD15_SYMBOL_TOTAL_DETACHMENT_KG,
            sumg.max(0.0),
            Some(0.0),
            None,
        ));
        updates.push(WritebackField::bounded(
            EROD15_SYMBOL_TOTAL_DEPOSITION_KG,
            lddend.max(0.0),
            Some(0.0),
            None,
        ));
        updates.push(WritebackField::bounded(
            EROD15_SYMBOL_PARTICLE_CLASS_COUNT,
            class_count_f64,
            Some(1.0),
            None,
        ));

        for class_index in 1..=class_count {
            let i = class_index - 1;
            let concentration = if qout > WB11_ZERO_THRESHOLD {
                gend[i] / qout
            } else {
                0.0
            };
            if !concentration.is_finite() || concentration < 0.0 {
                return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                    symbol: Self::erod14_class_symbol(
                        EROD15_ROOT_SEDIMENT_CONCENTRATION_KG_M3,
                        class_index,
                    ),
                    value: concentration,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }
            updates.push(WritebackField::bounded(
                Self::erod14_class_symbol(EROD14_ROOT_GEND, class_index),
                gend[i],
                Some(0.0),
                None,
            ));
            updates.push(WritebackField::bounded(
                Self::erod14_class_symbol(EROD14_ROOT_SEDMAX, class_index),
                sedmax[i],
                Some(0.0),
                None,
            ));
            updates.push(WritebackField::bounded(
                Self::erod14_class_symbol(EROD14_ROOT_FRCFLW, class_index),
                frcflw[i],
                Some(0.0),
                Some(1.0),
            ));
            updates.push(WritebackField::bounded(
                Self::erod14_class_symbol(EROD14_ROOT_SED_FRAC, class_index),
                sed_frac[i],
                Some(0.0),
                Some(1.0),
            ));
            updates.push(WritebackField::bounded(
                Self::erod14_class_symbol(EROD15_ROOT_SEDIMENT_CONCENTRATION_KG_M3, class_index),
                concentration,
                Some(0.0),
                None,
            ));
            updates.push(WritebackField::bounded(
                Self::erod14_class_symbol(EROD15_ROOT_PARTICLE_FLOW_FRACTION, class_index),
                sed_frac[i],
                Some(0.0),
                Some(1.0),
            ));
        }

        Ok(updates)
    }

}
