#[allow(clippy::wildcard_imports)]
use crate::hydrology::*;

impl Wb11HydrologyKernel {
    #[allow(clippy::too_many_lines)]
pub(crate) fn run_storage_reconciliation(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<KernelRunResponse, Wb11HydrologyKernelGuardError> {
        let phase_class = HillslopeKernelPhaseClass::HydrologyStorageReconciliation;
        let storage_initial =
            Self::require_state_scalar(request, phase_class, WB12_SYMBOL_STORAGE_INITIAL)?;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_STORAGE_INITIAL,
            storage_initial,
            Some(0.0),
            None,
        )?;

        let forward_solver_lane =
            Self::resolve_wb20_forward_solver_lane_enabled(request, phase_class)?;

        let closure_tolerance = Self::require_state_scalar(
            request,
            phase_class,
            WB12_SYMBOL_STORAGE_CLOSURE_TOLERANCE,
        )?;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_STORAGE_CLOSURE_TOLERANCE,
            closure_tolerance,
            Some(0.0),
            None,
        )?;

        let precip_input =
            Self::require_state_scalar(request, phase_class, WB12_SYMBOL_PRECIP_INPUT)?;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_PRECIP_INPUT,
            precip_input,
            Some(0.0),
            None,
        )?;

        let q_runoff = Self::require_flux_scalar(request, phase_class, WB12_SYMBOL_RUNOFF_Q)?;
        Self::require_flux_range(phase_class, WB12_SYMBOL_RUNOFF_Q, q_runoff, Some(0.0), None)?;

        let snow_coupling_s =
            Self::require_flux_scalar(request, phase_class, WB12_SYMBOL_SNOW_COUPLING_S)?;

        let interception_i =
            Self::require_flux_scalar(request, phase_class, WB15_SYMBOL_INTERCEPTION_I)?;
        Self::require_flux_range(
            phase_class,
            WB15_SYMBOL_INTERCEPTION_I,
            interception_i,
            Some(0.0),
            None,
        )?;
        let irrigation_input =
            Self::optional_flux_scalar(request, phase_class, IRRIG_SYMBOL_DAILY_IRRIGATION)?
                .unwrap_or(0.0);
        Self::require_flux_range(
            phase_class,
            IRRIG_SYMBOL_DAILY_IRRIGATION,
            irrigation_input,
            Some(0.0),
            None,
        )?;

        let et = Self::require_flux_scalar(request, phase_class, WB11_SYMBOL_ET)?;
        Self::require_flux_range(phase_class, WB11_SYMBOL_ET, et, Some(0.0), None)?;

        let percolation_loss =
            Self::require_flux_scalar(request, phase_class, WB11_SYMBOL_PERC_LOSS_D)?;
        Self::require_flux_range(
            phase_class,
            WB11_SYMBOL_PERC_LOSS_D,
            percolation_loss,
            Some(0.0),
            None,
        )?;

        let subsurface_loss =
            Self::require_flux_scalar(request, phase_class, WB11_SYMBOL_SUBHYD_QD)?;
        Self::require_flux_range(
            phase_class,
            WB11_SYMBOL_SUBHYD_QD,
            subsurface_loss,
            Some(0.0),
            None,
        )?;

        let storage_reconciled = Self::compute_storage_reconciled_with_interception(
            phase_class,
            storage_initial,
            precip_input,
            snow_coupling_s,
            irrigation_input,
            interception_i,
            q_runoff,
            et,
            percolation_loss,
            subsurface_loss,
        )?;

        let closure_delta = if forward_solver_lane {
            let solver_closure =
                storage_initial + precip_input + snow_coupling_s + irrigation_input
                    - interception_i
                    - q_runoff
                    - et
                    - percolation_loss
                    - subsurface_loss;
            solver_closure - storage_reconciled
        } else {
            let storage_observed =
                Self::require_state_scalar(request, phase_class, WB12_SYMBOL_STORAGE_OBSERVED)?;
            Self::require_state_range(
                phase_class,
                WB12_SYMBOL_STORAGE_OBSERVED,
                storage_observed,
                Some(0.0),
                None,
            )?;
            storage_reconciled - storage_observed
        };
        if closure_delta.abs() > closure_tolerance + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_STORAGE_CLOSURE_DELTA),
                value: closure_delta,
                minimum: Some(-closure_tolerance),
                maximum: Some(closure_tolerance),
            });
        }

        let Ok(status) = SimulationStatus::ok(
            SimulationPhase::HillslopeKernel,
            "HKERNEL-WB12-STORAGE-OK-001",
        ) else {
            unreachable!("status message ids are non-empty WB12 constants")
        };
        let writeback = KernelWritebackPayload::with_updates(
            vec![WritebackField::bounded(
                WB12_SYMBOL_STORAGE_RECONCILED,
                storage_reconciled,
                Some(0.0),
                None,
            )],
            vec![WritebackField::unbounded(
                WB12_SYMBOL_STORAGE_CLOSURE_DELTA,
                closure_delta,
            )],
        );
        Ok(KernelRunResponse::new(status, writeback))
    }

    #[allow(clippy::similar_names, clippy::too_many_lines)]
pub(crate) fn run_erod13_wave1_core(
        request: &HillslopeKernelRequest<'_>,
        q_runoff: f64,
        peakro: f64,
        watdur: f64,
    ) -> Result<Vec<WritebackField>, Wb11HydrologyKernelGuardError> {
        if !Self::resolve_erod13_core_enabled(request)? {
            return Ok(Vec::new());
        }

        let ie_symbol = BoundarySymbol::from(EROD13_SYMBOL_IE);
        let te_symbol = BoundarySymbol::from(EROD13_SYMBOL_TE);
        let fs_symbol = BoundarySymbol::from(EROD13_SYMBOL_FS);
        let ft_symbol = BoundarySymbol::from(EROD13_SYMBOL_FT);
        let taufe_symbol = BoundarySymbol::from(EROD13_SYMBOL_TAUFE);
        let q_symbol = BoundarySymbol::from(EROD13_SYMBOL_Q);
        let g_symbol = BoundarySymbol::from(EROD13_SYMBOL_G);
        let di_symbol = BoundarySymbol::from(EROD13_SYMBOL_DI);
        let beta_symbol = BoundarySymbol::from(EROD13_SYMBOL_BETA);
        let vf_symbol = BoundarySymbol::from(EROD13_SYMBOL_VF);
        let dgdx_symbol = BoundarySymbol::from(EROD13_SYMBOL_DGDX);
        let cntlen_symbol = BoundarySymbol::from(EROD13_SYMBOL_CNTLEN);
        let kr_symbol = BoundarySymbol::from(EROD13_SYMBOL_KR);
        let kradjf_symbol = BoundarySymbol::from(EROD13_SYMBOL_KRADJF);
        let tcadjf_symbol = BoundarySymbol::from(EROD13_SYMBOL_TCADJF);
        let shrsol_symbol = BoundarySymbol::from(EROD13_SYMBOL_SHRSOL);
        let tcend_symbol = BoundarySymbol::from(EROD13_SYMBOL_TCEND);
        let shcrit_symbol = BoundarySymbol::from(EROD13_SYMBOL_SHCRIT);
        let detinr_symbol = BoundarySymbol::from(EROD13_SYMBOL_DETINR);
        let effdrr_symbol = BoundarySymbol::from(EROD13_SYMBOL_EFFDRR);
        let effdrn_symbol = BoundarySymbol::from(EROD13_SYMBOL_EFFDRN);
        let veleff_symbol = BoundarySymbol::from(EROD13_SYMBOL_VELEFF);
        let pkro_symbol = BoundarySymbol::from(EROD13_SYMBOL_PKRO);
        let tc_k_symbol = BoundarySymbol::from(EROD13_SYMBOL_TC_K);
        let tc_m_symbol = BoundarySymbol::from(EROD13_SYMBOL_TC_M);

        let ie = Self::require_erod13_state_scalar(request, &ie_symbol)?;
        Self::require_erod13_domain(&ie_symbol, ie, Some(0.0), None)?;
        let te = Self::require_erod13_state_scalar(request, &te_symbol)?;
        Self::require_erod13_domain(&te_symbol, te, Some(WB11_ZERO_THRESHOLD), None)?;
        let fs = Self::require_erod13_state_scalar(request, &fs_symbol)?;
        Self::require_erod13_domain(&fs_symbol, fs, Some(0.0), None)?;
        let ft = Self::require_erod13_state_scalar(request, &ft_symbol)?;
        Self::require_erod13_domain(&ft_symbol, ft, Some(WB11_ZERO_THRESHOLD), None)?;
        Self::require_erod13_domain(&fs_symbol, fs, Some(0.0), Some(ft))?;
        let taufe = Self::require_erod13_state_scalar(request, &taufe_symbol)?;
        Self::require_erod13_domain(&taufe_symbol, taufe, Some(0.0), None)?;
        let q = Self::require_erod13_state_scalar(request, &q_symbol)?;
        Self::require_erod13_domain(&q_symbol, q, Some(0.0), None)?;
        let g = Self::require_erod13_state_scalar(request, &g_symbol)?;
        Self::require_erod13_domain(&g_symbol, g, Some(0.0), None)?;
        let di = Self::require_erod13_state_scalar(request, &di_symbol)?;
        Self::require_erod13_domain(&di_symbol, di, Some(0.0), None)?;
        let beta = Self::require_erod13_state_scalar(request, &beta_symbol)?;
        Self::require_erod13_domain(&beta_symbol, beta, Some(0.0), None)?;
        let vf = Self::require_erod13_state_scalar(request, &vf_symbol)?;
        Self::require_erod13_domain(&vf_symbol, vf, Some(0.0), None)?;
        let dgdx = Self::require_erod13_state_scalar(request, &dgdx_symbol)?;

        let cntlen = Self::require_erod13_state_scalar(request, &cntlen_symbol)?;
        Self::require_erod13_domain(&cntlen_symbol, cntlen, Some(WB11_ZERO_THRESHOLD), None)?;
        let kr = Self::require_erod13_state_scalar(request, &kr_symbol)?;
        Self::require_erod13_domain(&kr_symbol, kr, Some(WB11_ZERO_THRESHOLD), None)?;
        let kradjf = Self::require_erod13_state_scalar(request, &kradjf_symbol)?;
        Self::require_erod13_domain(&kradjf_symbol, kradjf, Some(WB11_ZERO_THRESHOLD), None)?;
        let tcadjf = Self::require_erod13_state_scalar(request, &tcadjf_symbol)?;
        Self::require_erod13_domain(&tcadjf_symbol, tcadjf, Some(EROD13_MIN_TCADJF), None)?;
        let shrsol = Self::require_erod13_state_scalar(request, &shrsol_symbol)?;
        Self::require_erod13_domain(&shrsol_symbol, shrsol, Some(WB11_ZERO_THRESHOLD), None)?;
        let tcend = Self::require_erod13_state_scalar(request, &tcend_symbol)?;
        Self::require_erod13_domain(&tcend_symbol, tcend, Some(WB11_ZERO_THRESHOLD), None)?;
        let shcrit = Self::require_erod13_state_scalar(request, &shcrit_symbol)?;
        Self::require_erod13_domain(&shcrit_symbol, shcrit, Some(0.0), None)?;
        let detinr = Self::require_erod13_state_scalar(request, &detinr_symbol)?;
        Self::require_erod13_domain(&detinr_symbol, detinr, Some(0.0), None)?;
        let effdrr = Self::require_erod13_state_scalar(request, &effdrr_symbol)?;
        Self::require_erod13_domain(&effdrr_symbol, effdrr, Some(WB11_ZERO_THRESHOLD), None)?;
        let effdrn = Self::require_erod13_state_scalar(request, &effdrn_symbol)?;
        Self::require_erod13_domain(&effdrn_symbol, effdrn, Some(WB11_ZERO_THRESHOLD), None)?;
        let veleff = Self::require_erod13_state_scalar(request, &veleff_symbol)?;
        Self::require_erod13_domain(&veleff_symbol, veleff, Some(0.0), None)?;
        let pkro = Self::require_erod13_state_scalar(request, &pkro_symbol)?;
        Self::require_erod13_domain(&pkro_symbol, pkro, Some(WB11_ZERO_THRESHOLD), None)?;
        let tc_k = Self::require_erod13_state_scalar(request, &tc_k_symbol)?;
        Self::require_erod13_domain(&tc_k_symbol, tc_k, Some(WB11_ZERO_THRESHOLD), None)?;
        let tc_m = Self::require_erod13_state_scalar(request, &tc_m_symbol)?;
        Self::require_erod13_domain(&tc_m_symbol, tc_m, Some(WB11_ZERO_THRESHOLD), None)?;

        Self::require_erod13_domain(
            &BoundarySymbol::from(WB12_SYMBOL_RUNOFF_Q),
            q_runoff,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;
        Self::require_erod13_domain(
            &BoundarySymbol::from(WB16_SYMBOL_PEAKRO),
            peakro,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;
        Self::require_erod13_domain(
            &BoundarySymbol::from(WB16_SYMBOL_WATDUR),
            watdur,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;
        let expected_watdur = q_runoff / peakro;
        let continuity_residual = (watdur - expected_watdur).abs();
        if continuity_residual > EROD13_CONTINUITY_TOLERANCE + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                symbol: BoundarySymbol::from(WB16_SYMBOL_WATDUR),
                value: watdur,
                minimum: Some(expected_watdur - EROD13_CONTINUITY_TOLERANCE),
                maximum: Some(expected_watdur + EROD13_CONTINUITY_TOLERANCE),
            });
        }

        let tau_f = taufe * (fs / ft);
        if !tau_f.is_finite() || tau_f < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                symbol: taufe_symbol.clone(),
                value: tau_f,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let eta = (cntlen * kr * kradjf * shrsol) / tcend;
        if !eta.is_finite() || eta < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                symbol: BoundarySymbol::from(EROD13_SYMBOL_ETA),
                value: eta,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        let taucn = (tcadjf * shcrit) / shrsol;
        if !taucn.is_finite() || taucn < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                symbol: BoundarySymbol::from(EROD13_SYMBOL_TAUCN),
                value: taucn,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        let theta = ((cntlen * detinr) / tcend) * (effdrr / effdrn);
        if !theta.is_finite() || theta < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                symbol: BoundarySymbol::from(EROD13_SYMBOL_THETA),
                value: theta,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        let phi = (beta * veleff) / pkro;
        if !phi.is_finite() || phi < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                symbol: BoundarySymbol::from(EROD13_SYMBOL_PHI),
                value: phi,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let tc = tcadjf * tc_k * tau_f.powf(tc_m);
        if !tc.is_finite() || tc < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                symbol: BoundarySymbol::from(EROD13_SYMBOL_TC),
                value: tc,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let (dc, df) = if tau_f > taucn && g < tc {
            if tc <= WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                    symbol: BoundarySymbol::from(EROD13_SYMBOL_TC),
                    value: tc,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: None,
                });
            }
            let dc = eta * (tau_f - taucn);
            if !dc.is_finite() || dc < -WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                    symbol: BoundarySymbol::from(EROD13_SYMBOL_DC),
                    value: dc,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }
            let df = dc * ((tc - g) / tc);
            if !df.is_finite() || df < -WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                    symbol: BoundarySymbol::from(EROD13_SYMBOL_DF),
                    value: df,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }
            (dc, df)
        } else if g > tc {
            Self::require_erod13_domain(&q_symbol, q, Some(WB11_ZERO_THRESHOLD), None)?;
            let df = -((beta * vf / q) * (g - tc));
            if !df.is_finite() || df > WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                    symbol: BoundarySymbol::from(EROD13_SYMBOL_DF),
                    value: df,
                    minimum: None,
                    maximum: Some(0.0),
                });
            }
            (0.0, df)
        } else {
            (0.0, 0.0)
        };

        let expected_dgdx = df + di;
        let dgdx_residual = (dgdx - expected_dgdx).abs();
        if dgdx_residual > EROD13_CONTINUITY_TOLERANCE + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                symbol: dgdx_symbol,
                value: dgdx,
                minimum: Some(expected_dgdx - EROD13_CONTINUITY_TOLERANCE),
                maximum: Some(expected_dgdx + EROD13_CONTINUITY_TOLERANCE),
            });
        }

        Ok(vec![
            WritebackField::bounded(EROD13_SYMBOL_DC, dc, Some(0.0), None),
            WritebackField::bounded(EROD13_SYMBOL_TC, tc, Some(0.0), None),
            WritebackField::unbounded(EROD13_SYMBOL_DF, df),
            WritebackField::bounded(EROD13_SYMBOL_ETA, eta, Some(0.0), None),
            WritebackField::bounded(EROD13_SYMBOL_TAUCN, taucn, Some(0.0), None),
            WritebackField::bounded(EROD13_SYMBOL_THETA, theta, Some(0.0), None),
            WritebackField::bounded(EROD13_SYMBOL_PHI, phi, Some(0.0), None),
        ])
    }

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
pub(crate) fn erod19_shear(a: f64, b: f64, c: f64, x: f64) -> f64 {
        let mut value = (a * x * x) + (b * x) + c;
        if value < 0.0 {
            value = 0.0;
        }
        let mut shear = value.powf(0.666_666_67);
        if shear <= 0.0 {
            shear = EROD19_SHEAR_FLOOR;
        }
        shear
    }
pub(crate) fn erod19_root(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
        if a.abs() <= WB11_ZERO_THRESHOLD {
            return None;
        }
        let discriminant = (b * b) + (4.0 * a * c);
        if discriminant < 0.0 {
            return None;
        }
        let part = discriminant.sqrt();
        let two_a = 2.0 * a;
        if two_a.abs() <= WB11_ZERO_THRESHOLD {
            return None;
        }
        let mut x1 = (-b - part) / two_a;
        let mut x2 = (-b + part) / two_a;
        if x1 > x2 {
            std::mem::swap(&mut x1, &mut x2);
        }
        Some((x1, x2))
    }

    #[allow(clippy::similar_names, clippy::too_many_lines)]
pub(crate) fn erod19_xcrit_classification(
        a: f64,
        b: f64,
        c: f64,
        tauc: f64,
        xb: f64,
        xe: f64,
    ) -> (f64, f64, f64) {
        let mut xc1 = xb;
        let mut xc2 = xe;
        let mut mshear = 1.0;

        let mut tauchk = tauc.powf(1.5) - c;
        if tauchk < 0.0 {
            tauchk = 0.0;
        }

        let taub = Self::erod19_shear(a, b, c, xb);
        let taue = Self::erod19_shear(a, b, c, xe);

        if a.abs() <= WB11_ZERO_THRESHOLD {
            if b.abs() > WB11_ZERO_THRESHOLD {
                xc1 = tauchk / b;
            } else {
                xc1 = EROD19_UNIFORM_XC_SENTINEL;
            }
            if taue > taub {
                mshear = 3.0;
                if xc1 <= xb {
                    mshear = 2.0;
                }
                if xc1 >= xe {
                    mshear = 1.0;
                }
            } else {
                mshear = 4.0;
                if xc1 >= xe {
                    mshear = 2.0;
                }
                if xc1 <= xb {
                    mshear = 1.0;
                }
            }
        } else if a > 0.0 && taue > taub {
            if taub >= tauc {
                mshear = 2.0;
            } else if taue <= tauc {
                mshear = 1.0;
            } else {
                mshear = 3.0;
                if let Some((x1, x2)) = Self::erod19_root(a, b, tauchk) {
                    if x1 >= xb && x1 <= xe {
                        xc1 = x1;
                    } else if x2 >= xb && x2 <= xe {
                        xc1 = x2;
                    }
                }
            }
        } else if taue >= tauc && taub >= tauc {
            mshear = 2.0;
        } else {
            let part = (b * b) + (4.0 * a * tauchk);
            if part <= 0.0 {
                mshear = 1.0;
            } else if let Some((x1, x2)) = Self::erod19_root(a, b, tauchk) {
                if taub <= tauc && taue >= tauc {
                    mshear = 3.0;
                    xc1 = if x1 <= xb || x1 >= xe { x2 } else { x1 };
                } else if taub >= tauc && taue <= tauc {
                    mshear = 4.0;
                    xc1 = if x1 <= xb || x1 >= xe { x2 } else { x1 };
                } else if taub <= tauc && taue <= tauc {
                    mshear = 5.0;
                    xc1 = x1;
                    xc2 = x2;
                    if x1 < xb
                        || x1 > xe
                        || x2 < xb
                        || x2 > xe
                        || (x1 - x2).abs() <= WB11_ZERO_THRESHOLD
                    {
                        mshear = 1.0;
                    }
                }
            }
        }

        (mshear, xc1.clamp(xb, xe), xc2.clamp(xb, xe))
    }

    #[allow(clippy::too_many_arguments)]
pub(crate) fn erod19_depc(
        xu: f64,
        a: f64,
        b: f64,
        phi: f64,
        theta: f64,
        du: f64,
        ktrato: f64,
        qostar: f64,
    ) -> f64 {
        if (qostar + xu).abs() >= EROD19_DEPC_QOSTAR_XU_EPSILON {
            du - ((a * ktrato * phi * 2.0 * (qostar + xu)) / (phi + 2.0))
                - (((b * ktrato) - (2.0 * a * ktrato * qostar) - theta) * phi / (phi + 1.0))
        } else {
            0.0
        }
    }

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub(crate) fn erod19_depend(
        xu: f64,
        xl: f64,
        a: f64,
        b: f64,
        cdep: f64,
        phi: f64,
        theta: f64,
        ktrato: f64,
        qostar: f64,
    ) -> f64 {
        let tmpvr1 = 2.0 * a * ktrato;
        let r1 = (phi / (1.0 + phi)) * ((b * ktrato) - theta - (tmpvr1 * qostar));
        let r2 = tmpvr1 * phi / (2.0 + phi);

        let mut xdend;
        if qostar >= 0.0 {
            xdend = xl;
            let denominator = xdend + qostar;
            let mut ratio = if denominator.abs() > WB11_ZERO_THRESHOLD {
                (xu + qostar) / denominator
            } else {
                1.0
            };
            if ratio <= 0.0 {
                ratio = 1.0;
            }
            let expon = 1.0 + phi;
            let f = r1 + (r2 * (xdend + qostar)) + (cdep * ratio.powf(expon));
            if f < 0.0 {
                return xdend;
            }
            xdend = xu + EROD19_DEPEND_INITIAL_STEP_POSITIVE;
            if xdend > xl {
                xdend = f64::midpoint(xu, xl);
            }
        } else {
            if (xu + qostar).abs() <= EROD19_DEPEND_XU_QOSTAR_NEAR_ZERO {
                return -qostar;
            }
            xdend = xu + EROD19_DEPEND_INITIAL_STEP_NEGATIVE;
            if xdend > xl {
                xdend = f64::midpoint(xu, xl);
            }
            let denominator = xdend + qostar;
            let mut ratio = if denominator.abs() > WB11_ZERO_THRESHOLD {
                (xu + qostar) / denominator
            } else {
                1.0
            };
            if ratio <= 0.0 {
                ratio = 1.0;
            }
            let expon = 1.0 + phi;
            let f = r1 + (r2 * (xdend + qostar)) + (cdep * ratio.powf(expon));
            if f >= 0.0 {
                return xdend;
            }
        }

        let mut xmin = xl;
        let mut positive_f_count = 0_u32;
        let mut converged = false;
        for _ in 0..EROD19_DEPEND_NEWTON_MAX_ITERS {
            let tmp = xdend + qostar;
            let mut ratio = if tmp.abs() > WB11_ZERO_THRESHOLD {
                (xu + qostar) / tmp
            } else {
                1.0
            };
            if ratio < 0.0 {
                ratio = 1.0;
            }
            let expon = 1.0 + phi;
            let ratio_pow = ratio.powf(expon);
            let f = r1 + (r2 * (xdend + qostar)) + (cdep * ratio_pow);

            if f > 0.0 && qostar < 0.0 {
                positive_f_count += 1;
                if xdend < xmin {
                    xmin = xdend;
                }
            }

            if f.abs() <= EROD19_DEPEND_NEWTON_RESIDUAL_TOLERANCE {
                converged = true;
                break;
            }

            if tmp.abs() > WB11_ZERO_THRESHOLD {
                let df = r2 - (((1.0 + phi) * cdep * ratio_pow) / tmp);
                if df.abs() > WB11_ZERO_THRESHOLD {
                    xdend -= f / df;
                    if qostar < 0.0 {
                        if xdend < xu {
                            xdend = xu + EROD19_DEPEND_INITIAL_STEP_NEGATIVE;
                        }
                        if xdend > -qostar {
                            xdend = -qostar - EROD19_DEPEND_INITIAL_STEP_NEGATIVE;
                        }
                        if xdend > xl {
                            xdend = xl;
                        }
                    }
                } else {
                    xdend = xu + EROD19_DEPEND_INITIAL_STEP_NEGATIVE;
                }
            }

            if xdend < xu {
                xdend = xu + EROD19_DEPEND_INITIAL_STEP_NEGATIVE;
            }
        }

        if !converged && qostar < 0.0 {
            if positive_f_count == 0 {
                xdend = xl;
            } else {
                xdend = xmin;
            }
        }

        xdend
    }

    #[allow(clippy::similar_names, clippy::too_many_lines)]
pub(crate) fn run_erod19_route_segment_migration(
        request: &HillslopeKernelRequest<'_>,
        erod13_state_updates: &[WritebackField],
    ) -> Result<Vec<WritebackField>, Wb11HydrologyKernelGuardError> {
        if !Self::resolve_erod14_wave2_enabled(request)? {
            return Ok(Vec::new());
        }

        let nslpts_symbol = BoundarySymbol::from(EROD18_SYMBOL_NSLPTS);
        let nslpts_value = Self::require_erod18_state_scalar(request, &nslpts_symbol)?;

        let segment_index_u32 =
            u32::try_from(EROD18_ROUTE_SEGMENT_INDEX).map_err(|_| {
                Wb11HydrologyKernelGuardError::Erod18DomainViolation {
                    symbol: nslpts_symbol.clone(),
                    value: nslpts_value,
                    minimum: Some(2.0),
                    maximum: None,
                }
            })?;
        let min_segment_value = f64::from(segment_index_u32);
        Self::require_erod18_domain(&nslpts_symbol, nslpts_value, Some(min_segment_value), None)?;

        let nslpts_rounded = nslpts_value.round();
        if (nslpts_value - nslpts_rounded).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod18DomainViolation {
                symbol: nslpts_symbol.clone(),
                value: nslpts_value,
                minimum: Some(min_segment_value),
                maximum: None,
            });
        }
        let nslpts = format!("{nslpts_rounded:.0}").parse::<usize>().map_err(|_| {
            Wb11HydrologyKernelGuardError::Erod18DomainViolation {
                symbol: nslpts_symbol.clone(),
                value: nslpts_value,
                minimum: Some(min_segment_value),
                maximum: None,
            }
        })?;
        if nslpts < EROD18_ROUTE_SEGMENT_INDEX {
            return Err(Wb11HydrologyKernelGuardError::Erod18DomainViolation {
                symbol: nslpts_symbol.clone(),
                value: nslpts_value,
                minimum: Some(min_segment_value),
                maximum: None,
            });
        }

        let segment_index = EROD18_ROUTE_SEGMENT_INDEX;
        let xu_symbol = Self::erod18_route_segment_symbol(EROD18_ROOT_XU, segment_index);
        let xl_symbol = Self::erod18_route_segment_symbol(EROD18_ROOT_XL, segment_index);
        let ainf_symbol = Self::erod18_route_segment_symbol(EROD18_ROOT_AINF, segment_index);
        let binf_symbol = Self::erod18_route_segment_symbol(EROD18_ROOT_BINF, segment_index);
        let cinf_symbol = Self::erod18_route_segment_symbol(EROD18_ROOT_CINF, segment_index);
        let ainftc_symbol =
            Self::erod18_route_segment_symbol(EROD18_ROOT_AINTC, segment_index);
        let binftc_symbol =
            Self::erod18_route_segment_symbol(EROD18_ROOT_BINTC, segment_index);
        let cinftc_symbol =
            Self::erod18_route_segment_symbol(EROD18_ROOT_CINTC, segment_index);

        let xu = Self::require_erod18_state_scalar(request, &xu_symbol)?;
        let xl = Self::require_erod18_state_scalar(request, &xl_symbol)?;
        let ainf = Self::require_erod18_state_scalar(request, &ainf_symbol)?;
        let binf = Self::require_erod18_state_scalar(request, &binf_symbol)?;
        let cinf = Self::require_erod18_state_scalar(request, &cinf_symbol)?;
        let ainftc = Self::require_erod18_state_scalar(request, &ainftc_symbol)?;
        let binftc = Self::require_erod18_state_scalar(request, &binftc_symbol)?;
        let cinftc = Self::require_erod18_state_scalar(request, &cinftc_symbol)?;

        Self::require_erod18_domain(&xu_symbol, xu, Some(0.0), None)?;
        Self::require_erod18_domain(&xl_symbol, xl, Some(xu), None)?;

        let qostar_symbol = BoundarySymbol::from(EROD18_SYMBOL_QOSTAR);
        let qostar = Self::require_erod18_state_scalar(request, &qostar_symbol)?;

        let xdetst_symbol = BoundarySymbol::from(EROD18_SYMBOL_XDETST);
        let xdetst = Self::require_erod18_state_scalar(request, &xdetst_symbol)?;
        Self::require_erod18_domain(&xdetst_symbol, xdetst, Some(0.0), Some(xl))?;

        let lddend_symbol = BoundarySymbol::from(EROD18_SYMBOL_LDDEND);
        let lddend = Self::require_erod18_state_scalar(request, &lddend_symbol)?;
        Self::require_erod18_domain(&lddend_symbol, lddend, Some(0.0), None)?;

        let ktrato_symbol = BoundarySymbol::from(EROD14_SYMBOL_KTRATO);
        let ktrato = Self::require_erod18_state_scalar(request, &ktrato_symbol)?;
        Self::require_erod18_domain(&ktrato_symbol, ktrato, Some(WB11_ZERO_THRESHOLD), None)?;

        let theta_symbol = BoundarySymbol::from(EROD13_SYMBOL_THETA);
        let theta = if let Some(value) =
            Self::extract_state_update_scalar(erod13_state_updates, EROD13_SYMBOL_THETA)
        {
            value
        } else if request.state_surface.contains_key(&theta_symbol) {
            Self::require_erod18_state_scalar(request, &theta_symbol)?
        } else {
            let cntlen_symbol = BoundarySymbol::from(EROD13_SYMBOL_CNTLEN);
            let detinr_symbol = BoundarySymbol::from(EROD13_SYMBOL_DETINR);
            let tcend_symbol = BoundarySymbol::from(EROD13_SYMBOL_TCEND);
            let effdrr_symbol = BoundarySymbol::from(EROD13_SYMBOL_EFFDRR);
            let effdrn_symbol = BoundarySymbol::from(EROD13_SYMBOL_EFFDRN);

            let cntlen = Self::require_erod18_state_scalar(request, &cntlen_symbol)?;
            let detinr = Self::require_erod18_state_scalar(request, &detinr_symbol)?;
            let tcend = Self::require_erod18_state_scalar(request, &tcend_symbol)?;
            let effdrr = Self::require_erod18_state_scalar(request, &effdrr_symbol)?;
            let effdrn = Self::require_erod18_state_scalar(request, &effdrn_symbol)?;

            Self::require_erod18_domain(&cntlen_symbol, cntlen, Some(WB11_ZERO_THRESHOLD), None)?;
            Self::require_erod18_domain(&detinr_symbol, detinr, Some(0.0), None)?;
            Self::require_erod18_domain(&tcend_symbol, tcend, Some(WB11_ZERO_THRESHOLD), None)?;
            Self::require_erod18_domain(&effdrr_symbol, effdrr, Some(WB11_ZERO_THRESHOLD), None)?;
            Self::require_erod18_domain(&effdrn_symbol, effdrn, Some(WB11_ZERO_THRESHOLD), None)?;

            ((cntlen * detinr) / tcend) * (effdrr / effdrn)
        };
        Self::require_erod18_domain(&theta_symbol, theta, Some(0.0), None)?;

        let phi_symbol = BoundarySymbol::from(EROD13_SYMBOL_PHI);
        let phi = if let Some(value) =
            Self::extract_state_update_scalar(erod13_state_updates, EROD13_SYMBOL_PHI)
        {
            value
        } else if request.state_surface.contains_key(&phi_symbol) {
            Self::require_erod18_state_scalar(request, &phi_symbol)?
        } else if request
            .state_surface
            .contains_key(&BoundarySymbol::from(EROD14_SYMBOL_BETA))
        {
            let route_beta_symbol = BoundarySymbol::from(EROD14_SYMBOL_BETA);
            let route_beta = Self::require_erod18_state_scalar(request, &route_beta_symbol)?;
            Self::require_erod18_domain(&route_beta_symbol, route_beta, Some(0.0), None)?;
            route_beta
        } else {
            let beta_symbol = BoundarySymbol::from(EROD13_SYMBOL_BETA);
            let veleff_symbol = BoundarySymbol::from(EROD13_SYMBOL_VELEFF);
            let pkro_symbol = BoundarySymbol::from(EROD13_SYMBOL_PKRO);

            let beta = Self::require_erod18_state_scalar(request, &beta_symbol)?;
            let veleff = Self::require_erod18_state_scalar(request, &veleff_symbol)?;
            let pkro = Self::require_erod18_state_scalar(request, &pkro_symbol)?;

            Self::require_erod18_domain(&beta_symbol, beta, Some(0.0), None)?;
            Self::require_erod18_domain(&veleff_symbol, veleff, Some(0.0), None)?;
            Self::require_erod18_domain(&pkro_symbol, pkro, Some(WB11_ZERO_THRESHOLD), None)?;

            (beta * veleff) / pkro
        };
        Self::require_erod18_domain(&phi_symbol, phi, Some(0.0), None)?;
        Self::require_erod18_domain(
            &phi_symbol,
            phi,
            Some(WB11_ZERO_THRESHOLD),
            Some(EROD14_MAX_PHI),
        )?;

        let tauc_symbol = BoundarySymbol::from(EROD13_SYMBOL_TAUCN);
        let tauc = if let Some(value) =
            Self::extract_state_update_scalar(erod13_state_updates, EROD13_SYMBOL_TAUCN)
        {
            value
        } else if request.state_surface.contains_key(&tauc_symbol) {
            Self::require_erod18_state_scalar(request, &tauc_symbol)?
        } else if request
            .state_surface
            .contains_key(&BoundarySymbol::from(EROD13_SYMBOL_SHRSOL))
        {
            let tcadjf_symbol = BoundarySymbol::from(EROD13_SYMBOL_TCADJF);
            let shcrit_symbol = BoundarySymbol::from(EROD13_SYMBOL_SHCRIT);
            let shrsol_symbol = BoundarySymbol::from(EROD13_SYMBOL_SHRSOL);

            let tcadjf = Self::require_erod18_state_scalar(request, &tcadjf_symbol)?;
            let shcrit = Self::require_erod18_state_scalar(request, &shcrit_symbol)?;
            let shrsol = Self::require_erod18_state_scalar(request, &shrsol_symbol)?;

            Self::require_erod18_domain(&tcadjf_symbol, tcadjf, Some(EROD13_MIN_TCADJF), None)?;
            Self::require_erod18_domain(&shcrit_symbol, shcrit, Some(0.0), None)?;
            Self::require_erod18_domain(&shrsol_symbol, shrsol, Some(WB11_ZERO_THRESHOLD), None)?;

            (tcadjf * shcrit) / shrsol
        } else {
            theta * EROD19_TAUC_FALLBACK_SCALE
        };
        Self::require_erod18_domain(&tauc_symbol, tauc, Some(0.0), None)?;

        let g_symbol = BoundarySymbol::from(EROD13_SYMBOL_G);
        let ldlast = if let Some(value) = request.state_surface.get(&g_symbol) {
            let scalar = value.as_f64();
            if !scalar.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::Erod18NonFiniteSymbol {
                    symbol: g_symbol,
                    value: scalar,
                });
            }
            Self::require_erod18_domain(&g_symbol, scalar, Some(0.0), None)?;
            scalar
        } else {
            lddend
        };

        let mut dl = if qostar.abs() < EROD19_QOSTAR_NEAR_ZERO_THRESHOLD {
            (phi / (phi + 1.0)) * ((ktrato * binftc) - theta)
        } else {
            if qostar.abs() <= WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::Erod18DomainViolation {
                    symbol: qostar_symbol.clone(),
                    value: qostar,
                    minimum: Some(EROD19_QOSTAR_NEAR_ZERO_THRESHOLD),
                    maximum: None,
                });
            }
            (phi / qostar) * ((ktrato * cinftc) - ldlast)
        };
        if !dl.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::Erod18DomainViolation {
                symbol: qostar_symbol,
                value: dl,
                minimum: None,
                maximum: None,
            });
        }
        let mut du = dl;

        let (mshear, xc1, xc2) = Self::erod19_xcrit_classification(ainf, binf, cinf, tauc, xu, xl);

        let (xdbeg, xdend, ndep, lddend_out, ldlast_out) = if du < 0.0 {
            let cdep = Self::erod19_depc(xu, ainftc, binftc, phi, theta, du, ktrato, qostar);
            if !cdep.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::Erod18DomainViolation {
                    symbol: BoundarySymbol::from(EROD18_SYMBOL_DL),
                    value: cdep,
                    minimum: None,
                    maximum: None,
                });
            }

            let mut xdend =
                Self::erod19_depend(xu, xl, ainftc, binftc, cdep, phi, theta, ktrato, qostar);
            if !xdend.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::Erod18DomainViolation {
                    symbol: BoundarySymbol::from(EROD18_SYMBOL_XDEND),
                    value: xdend,
                    minimum: Some(xu),
                    maximum: Some(xl),
                });
            }
            xdend = xdend.clamp(xu, xl);
            let mut xdbeg = 0.0;
            let mut ndep = 0.0;
            let mut ldlast_out = ldlast;
            let lddend_out;

            if xdend < xl - WB11_ZERO_THRESHOLD {
                let tc_xdend = (((ainftc * xdend * xdend) + (binftc * xdend) + cinftc).max(0.0))
                    * ktrato;
                let tc_xl = (((ainftc * xl * xl) + (binftc * xl) + cinftc).max(0.0)) * ktrato;
                if mshear > EROD18_MSHEAR_MIN + WB11_ZERO_THRESHOLD
                    && ldlast_out > tc_xdend + WB11_ZERO_THRESHOLD
                {
                    ndep = 1.0;
                    xdbeg = xdend;
                    ldlast_out = ldlast_out.min(tc_xl).max(0.0);
                    lddend_out = ldlast_out;
                } else {
                    lddend_out = ldlast_out.max(0.0);
                }
            } else {
                xdend = xl;
                lddend_out = ldlast_out.max(0.0);
            }
            (xdbeg, xdend, ndep, lddend_out, ldlast_out)
        } else {
            dl = 0.0;
            du = 0.0;
            let mut xdbeg = 0.0;
            let xdend = xl;
            let mut ndep = 0.0;
            let mut ldlast_out = ldlast;
            let lddend_out;

            let tc_upper = (ktrato * cinftc).max(0.0);
            let tc_xl = (((ainftc * xl * xl) + (binftc * xl) + cinftc).max(0.0)) * ktrato;
            if ldlast_out > tc_upper + WB11_ZERO_THRESHOLD {
                ndep = 1.0;
                xdbeg = xu;
                ldlast_out = ldlast_out.min(tc_xl).max(0.0);
                lddend_out = ldlast_out;
            } else {
                lddend_out = ldlast_out.max(0.0);
            }
            (xdbeg, xdend, ndep, lddend_out, ldlast_out)
        };

        Self::require_erod18_domain(
            &BoundarySymbol::from(EROD18_SYMBOL_MSHEAR),
            mshear,
            Some(EROD18_MSHEAR_MIN),
            Some(EROD18_MSHEAR_MAX),
        )?;

        let updates = vec![
            WritebackField::bounded(EROD18_SYMBOL_NSLPTS, nslpts_value, Some(min_segment_value), None),
            WritebackField::bounded(xu_symbol, xu, Some(0.0), None),
            WritebackField::bounded(xl_symbol, xl, Some(xu), None),
            WritebackField::unbounded(ainf_symbol, ainf),
            WritebackField::unbounded(binf_symbol, binf),
            WritebackField::unbounded(cinf_symbol, cinf),
            WritebackField::unbounded(ainftc_symbol, ainftc),
            WritebackField::unbounded(binftc_symbol, binftc),
            WritebackField::unbounded(cinftc_symbol, cinftc),
            WritebackField::unbounded(EROD18_SYMBOL_QOSTAR, qostar),
            WritebackField::bounded(EROD18_SYMBOL_XDBEG, xdbeg, Some(0.0), None),
            WritebackField::bounded(EROD18_SYMBOL_XDEND, xdend, Some(xu), Some(xl)),
            WritebackField::bounded(EROD18_SYMBOL_XDETST, xdetst, Some(0.0), Some(xl)),
            WritebackField::bounded(EROD18_SYMBOL_LDLAST, ldlast_out, Some(0.0), None),
            WritebackField::bounded(EROD18_SYMBOL_LDDEND, lddend_out, Some(0.0), None),
            WritebackField::unbounded(EROD18_SYMBOL_DU, du),
            WritebackField::unbounded(EROD18_SYMBOL_DL, dl),
            WritebackField::bounded(
                EROD18_SYMBOL_NDEP,
                ndep,
                Some(0.0),
                Some(1.0),
            ),
            WritebackField::bounded(
                EROD18_SYMBOL_MSHEAR,
                mshear,
                Some(EROD18_MSHEAR_MIN),
                Some(EROD18_MSHEAR_MAX),
            ),
            WritebackField::bounded(EROD18_SYMBOL_XC1, xc1, Some(xu), Some(xl)),
            WritebackField::bounded(EROD18_SYMBOL_XC2, xc2, Some(xu), Some(xl)),
        ];

        Ok(updates)
    }

    #[allow(clippy::too_many_lines)]
pub(crate) fn run_peak_runoff(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<KernelRunResponse, Wb11HydrologyKernelGuardError> {
        let phase_class = HillslopeKernelPhaseClass::HydrologyPeakRunoff;

        let q_runoff = Self::require_flux_scalar(request, phase_class, WB12_SYMBOL_RUNOFF_Q)?;
        Self::require_flux_range(phase_class, WB12_SYMBOL_RUNOFF_Q, q_runoff, Some(0.0), None)?;
        if q_runoff < WB16_RUNOFF_NEAR_ZERO_THRESHOLD {
            let wb11_soil_water =
                Self::require_state_scalar(request, phase_class, WB11_SYMBOL_SOIL_WATER)?;
            Self::require_state_range(
                phase_class,
                WB11_SYMBOL_SOIL_WATER,
                wb11_soil_water,
                Some(0.0),
                None,
            )?;
            let watcon = wb11_soil_water;
            let total_soil = watcon * WB13_DEPTH_TO_MM;
            let soil_water_total = total_soil;

            let Ok(status) = SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HKERNEL-WB16-PEAK-ZERO-001",
            ) else {
                unreachable!("status message ids are non-empty WB16 constants")
            };

            let writeback = KernelWritebackPayload::with_updates(
                vec![
                    WritebackField::bounded(
                        WB16_SYMBOL_PEAKRO,
                        WB16_PEAKRO_FLOOR,
                        Some(WB16_PEAKRO_FLOOR),
                        None,
                    ),
                    WritebackField::bounded(
                        WB16_SYMBOL_WATDUR,
                        0.0,
                        Some(0.0),
                        Some(WB16_MAX_DURATION_S),
                    ),
                    WritebackField::bounded(WB16_SYMBOL_METHOD_BRANCH, 1.0, Some(1.0), Some(4.0)),
                    WritebackField::bounded(
                        WB16_SYMBOL_TSTAR,
                        0.0,
                        Some(0.0),
                        None,
                    ),
                    WritebackField::bounded(
                        WB16_SYMBOL_QPSTAR,
                        0.0,
                        Some(0.0),
                        None,
                    ),
                    WritebackField::bounded(
                        WB16_SYMBOL_VSTAR,
                        0.0,
                        Some(0.0),
                        Some(1.0),
                    ),
                    WritebackField::bounded(
                        BoundarySymbol::from(WB13_STATE_SYMBOL_WATCON),
                        watcon,
                        Some(0.0),
                        None,
                    ),
                    WritebackField::bounded(
                        BoundarySymbol::from(WB13_STATE_SYMBOL_TOTAL_SOIL),
                        total_soil,
                        Some(0.0),
                        None,
                    ),
                    WritebackField::bounded(
                        BoundarySymbol::from(WB13_STATE_SYMBOL_SOIL_WATER_TOTAL),
                        soil_water_total,
                        Some(0.0),
                        None,
                    ),
                ],
                Vec::new(),
            );
            return Ok(KernelRunResponse::new(status, writeback));
        }

        let hyetograph_point_count = Self::resolve_hyetograph_point_count(request, phase_class)?;
        let (hyetograph_times, hyetograph_intensities) =
            Self::load_hyetograph_series(request, phase_class, hyetograph_point_count)?;
        let effdrr = if hyetograph_times.len() >= 2 {
            hyetograph_times[hyetograph_times.len() - 1] - hyetograph_times[0]
        } else {
            0.0
        };
        if !effdrr.is_finite() || effdrr <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("timem_0001"),
                value: effdrr,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let vave = q_runoff / effdrr;
        if !vave.is_finite() || vave <= 0.0 {
            return Err(Wb11HydrologyKernelGuardError::FluxSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_RUNOFF_Q),
                value: vave,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let irrigation_rate_m_per_s =
            Self::require_state_scalar(request, phase_class, IRRIG_SYMBOL_RUNTIME_RATE_MPS)?;
        Self::require_state_range(
            phase_class,
            IRRIG_SYMBOL_RUNTIME_RATE_MPS,
            irrigation_rate_m_per_s,
            Some(0.0),
            None,
        )?;

        let interception_i =
            Self::require_flux_scalar(request, phase_class, WB15_SYMBOL_INTERCEPTION_I)?;
        Self::require_flux_range(
            phase_class,
            WB15_SYMBOL_INTERCEPTION_I,
            interception_i,
            Some(0.0),
            None,
        )?;

        let efflen = Self::require_state_scalar(request, phase_class, WB16_SYMBOL_EFFLEN)?;
        if efflen <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_EFFLEN),
                value: efflen,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let ealpha = Self::require_state_scalar(request, phase_class, WB16_SYMBOL_EALPHA)?;
        if ealpha <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_EALPHA),
                value: ealpha,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let exponent_m = Self::require_state_scalar(request, phase_class, WB16_SYMBOL_EXPONENT_M)?;
        if exponent_m <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_EXPONENT_M),
                value: exponent_m,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let remax = hyetograph_intensities
            .iter()
            .copied()
            .fold(0.0_f64, f64::max)
            + irrigation_rate_m_per_s;
        if !remax.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("intsty_0001"),
                value: remax,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }
        if remax <= WB11_ZERO_THRESHOLD {
            let wb11_soil_water =
                Self::require_state_scalar(request, phase_class, WB11_SYMBOL_SOIL_WATER)?;
            Self::require_state_range(
                phase_class,
                WB11_SYMBOL_SOIL_WATER,
                wb11_soil_water,
                Some(0.0),
                None,
            )?;
            let watcon = wb11_soil_water;
            let total_soil = watcon * WB13_DEPTH_TO_MM;
            let soil_water_total = total_soil;

            let Ok(status) = SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HKERNEL-WB16-PEAK-ZERO-002",
            ) else {
                unreachable!("status message ids are non-empty WB16 constants")
            };

            let writeback = KernelWritebackPayload::with_updates(
                vec![
                    WritebackField::bounded(
                        WB16_SYMBOL_PEAKRO,
                        WB16_PEAKRO_FLOOR,
                        Some(WB16_PEAKRO_FLOOR),
                        None,
                    ),
                    WritebackField::bounded(
                        WB16_SYMBOL_WATDUR,
                        0.0,
                        Some(0.0),
                        Some(WB16_MAX_DURATION_S),
                    ),
                    WritebackField::bounded(WB16_SYMBOL_METHOD_BRANCH, 1.0, Some(1.0), Some(4.0)),
                    WritebackField::bounded(
                        WB16_SYMBOL_TSTAR,
                        0.0,
                        Some(0.0),
                        None,
                    ),
                    WritebackField::bounded(
                        WB16_SYMBOL_QPSTAR,
                        0.0,
                        Some(0.0),
                        None,
                    ),
                    WritebackField::bounded(
                        WB16_SYMBOL_VSTAR,
                        0.0,
                        Some(0.0),
                        Some(1.0),
                    ),
                    WritebackField::bounded(
                        BoundarySymbol::from(WB13_STATE_SYMBOL_WATCON),
                        watcon,
                        Some(0.0),
                        None,
                    ),
                    WritebackField::bounded(
                        BoundarySymbol::from(WB13_STATE_SYMBOL_TOTAL_SOIL),
                        total_soil,
                        Some(0.0),
                        None,
                    ),
                    WritebackField::bounded(
                        BoundarySymbol::from(WB13_STATE_SYMBOL_SOIL_WATER_TOTAL),
                        soil_water_total,
                        Some(0.0),
                        None,
                    ),
                ],
                Vec::new(),
            );
            return Ok(KernelRunResponse::new(status, writeback));
        }

        let vstar = vave / remax;
        if !vstar.is_finite() || vstar <= 0.0 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_VSTAR),
                value: vstar,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let vave_power = vave.powf(exponent_m - 1.0);
        let te_base = efflen / (ealpha * vave_power);
        if !te_base.is_finite() || te_base <= 0.0 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_EFFLEN),
                value: te_base,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let te = te_base.powf(1.0 / exponent_m);
        let tstar = te / effdrr;
        if !tstar.is_finite() || tstar <= 0.0 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_TSTAR),
                value: tstar,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let (method_branch, qpstar) = if tstar >= 1.0 {
            (1.0, 1.0 / tstar.powf(exponent_m))
        } else if vstar < 1.0 {
            let tc_discriminant = 1.0 - (2.4 * (1.0 - vstar) * vstar);
            if !tc_discriminant.is_finite() || tc_discriminant < 0.0 {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB16_SYMBOL_VSTAR),
                    value: tc_discriminant,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }
            let tc_denominator = 1.2 * (1.0 - vstar);
            if !tc_denominator.is_finite() || tc_denominator <= 0.0 {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB16_SYMBOL_VSTAR),
                    value: tc_denominator,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }
            let tc = (1.0 - tc_discriminant.sqrt()) / tc_denominator;
            if !tc.is_finite() || tc <= 0.0 {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB16_SYMBOL_VSTAR),
                    value: tc,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }

            if tstar > tc {
                (2.0, 1.0 / tstar)
            } else {
                (3.0, (1.0 / vstar) - 0.6 * (((1.0 - vstar) / vstar) * tstar))
            }
        } else {
            (4.0, 1.0)
        };
        if !qpstar.is_finite() || qpstar <= 0.0 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_QPSTAR),
                value: qpstar,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let peakro_raw = vave * qpstar;
        if !peakro_raw.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_PEAKRO),
                value: peakro_raw,
                minimum: None,
                maximum: None,
            });
        }

        let peakro = peakro_raw.max(WB16_PEAKRO_FLOOR);
        if !peakro.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_PEAKRO),
                value: peakro,
                minimum: None,
                maximum: None,
            });
        }

        let watdur_raw = q_runoff / peakro;
        if !watdur_raw.is_finite() || watdur_raw < 0.0 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_WATDUR),
                value: watdur_raw,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        let watdur = watdur_raw.min(WB16_MAX_DURATION_S);

        let wb11_soil_water =
            Self::require_state_scalar(request, phase_class, WB11_SYMBOL_SOIL_WATER)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_SOIL_WATER,
            wb11_soil_water,
            Some(0.0),
            None,
        )?;
        let watcon = wb11_soil_water;
        let total_soil = watcon * WB13_DEPTH_TO_MM;
        let soil_water_total = total_soil;

        let erod13_state_updates = Self::run_erod13_wave1_core(request, q_runoff, peakro, watdur)?;
        let erod14_state_updates = Self::run_erod14_wave2(request, &erod13_state_updates)?;
        let erod19_state_updates = Self::run_erod19_route_segment_migration(request, &erod13_state_updates)?;
        let status_message_id = if !erod19_state_updates.is_empty() {
            "HKERNEL-EROD19-ROUTE-OK-001"
        } else if !erod14_state_updates.is_empty() {
            "HKERNEL-EROD14-WAVE2-OK-001"
        } else if !erod13_state_updates.is_empty() {
            "HKERNEL-EROD13-CORE-OK-001"
        } else {
            "HKERNEL-WB16-PEAK-OK-001"
        };

        let Ok(status) = SimulationStatus::ok(SimulationPhase::HillslopeKernel, status_message_id)
        else {
            unreachable!("status message ids are non-empty WB16 constants")
        };

        let mut state_updates = vec![
            WritebackField::bounded(WB16_SYMBOL_PEAKRO, peakro, Some(WB16_PEAKRO_FLOOR), None),
            WritebackField::bounded(
                WB16_SYMBOL_WATDUR,
                watdur,
                Some(0.0),
                Some(WB16_MAX_DURATION_S),
            ),
            WritebackField::bounded(
                WB16_SYMBOL_METHOD_BRANCH,
                method_branch,
                Some(1.0),
                Some(4.0),
            ),
            WritebackField::bounded(WB16_SYMBOL_TSTAR, tstar, Some(0.0), None),
            WritebackField::bounded(WB16_SYMBOL_QPSTAR, qpstar, Some(0.0), None),
            WritebackField::bounded(
                WB16_SYMBOL_VSTAR,
                vstar,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                BoundarySymbol::from(WB13_STATE_SYMBOL_WATCON),
                watcon,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                BoundarySymbol::from(WB13_STATE_SYMBOL_TOTAL_SOIL),
                total_soil,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                BoundarySymbol::from(WB13_STATE_SYMBOL_SOIL_WATER_TOTAL),
                soil_water_total,
                Some(0.0),
                None,
            ),
        ];
        state_updates.extend(erod13_state_updates);
        state_updates.extend(erod14_state_updates);
        state_updates.extend(erod19_state_updates);

        let writeback = KernelWritebackPayload::with_updates(state_updates, Vec::new());
        Ok(KernelRunResponse::new(status, writeback))
    }
}
