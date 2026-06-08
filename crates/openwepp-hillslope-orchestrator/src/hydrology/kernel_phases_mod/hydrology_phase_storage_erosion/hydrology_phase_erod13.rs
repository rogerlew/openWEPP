#[allow(clippy::wildcard_imports)]
use crate::hydrology::*;

impl Wb11HydrologyKernel {
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

}
