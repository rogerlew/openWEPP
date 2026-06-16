#[allow(clippy::wildcard_imports)]
use crate::hydrology::*;

struct Erod13Symbols {
    ie: BoundarySymbol,
    te: BoundarySymbol,
    fs: BoundarySymbol,
    ft: BoundarySymbol,
    taufe: BoundarySymbol,
    q: BoundarySymbol,
    g: BoundarySymbol,
    di: BoundarySymbol,
    beta: BoundarySymbol,
    vf: BoundarySymbol,
    dgdx: BoundarySymbol,
    cntlen: BoundarySymbol,
    kr: BoundarySymbol,
    kradjf: BoundarySymbol,
    tcadjf: BoundarySymbol,
    shrsol: BoundarySymbol,
    tcend: BoundarySymbol,
    shcrit: BoundarySymbol,
    detinr: BoundarySymbol,
    effdrr: BoundarySymbol,
    effdrn: BoundarySymbol,
    veleff: BoundarySymbol,
    pkro: BoundarySymbol,
    tc_k: BoundarySymbol,
    tc_m: BoundarySymbol,
}

impl Erod13Symbols {
    fn new() -> Self {
        Self {
            ie: BoundarySymbol::from(EROD13_SYMBOL_IE),
            te: BoundarySymbol::from(EROD13_SYMBOL_TE),
            fs: BoundarySymbol::from(EROD13_SYMBOL_FS),
            ft: BoundarySymbol::from(EROD13_SYMBOL_FT),
            taufe: BoundarySymbol::from(EROD13_SYMBOL_TAUFE),
            q: BoundarySymbol::from(EROD13_SYMBOL_Q),
            g: BoundarySymbol::from(EROD13_SYMBOL_G),
            di: BoundarySymbol::from(EROD13_SYMBOL_DI),
            beta: BoundarySymbol::from(EROD13_SYMBOL_BETA),
            vf: BoundarySymbol::from(EROD13_SYMBOL_VF),
            dgdx: BoundarySymbol::from(EROD13_SYMBOL_DGDX),
            cntlen: BoundarySymbol::from(EROD13_SYMBOL_CNTLEN),
            kr: BoundarySymbol::from(EROD13_SYMBOL_KR),
            kradjf: BoundarySymbol::from(EROD13_SYMBOL_KRADJF),
            tcadjf: BoundarySymbol::from(EROD13_SYMBOL_TCADJF),
            shrsol: BoundarySymbol::from(EROD13_SYMBOL_SHRSOL),
            tcend: BoundarySymbol::from(EROD13_SYMBOL_TCEND),
            shcrit: BoundarySymbol::from(EROD13_SYMBOL_SHCRIT),
            detinr: BoundarySymbol::from(EROD13_SYMBOL_DETINR),
            effdrr: BoundarySymbol::from(EROD13_SYMBOL_EFFDRR),
            effdrn: BoundarySymbol::from(EROD13_SYMBOL_EFFDRN),
            veleff: BoundarySymbol::from(EROD13_SYMBOL_VELEFF),
            pkro: BoundarySymbol::from(EROD13_SYMBOL_PKRO),
            tc_k: BoundarySymbol::from(EROD13_SYMBOL_TC_K),
            tc_m: BoundarySymbol::from(EROD13_SYMBOL_TC_M),
        }
    }
}

struct Erod13EventInputs {
    fs: f64,
    ft: f64,
    taufe: f64,
    q: f64,
    g: f64,
    di: f64,
    beta: f64,
    vf: f64,
    dgdx: f64,
}

struct Erod13ProcessInputs {
    cntlen: f64,
    kr: f64,
    kradjf: f64,
    tcadjf: f64,
    shrsol: f64,
    tcend: f64,
    shcrit: f64,
    detinr: f64,
    effdrr: f64,
    effdrn: f64,
    veleff: f64,
    pkro: f64,
    tc_k: f64,
    tc_m: f64,
}

struct Erod13RunoffInputs {
    q_runoff: f64,
    peakro: f64,
    watdur: f64,
}

struct Erod13Inputs {
    event: Erod13EventInputs,
    process: Erod13ProcessInputs,
    runoff: Erod13RunoffInputs,
}

struct Erod13DerivedTerms {
    tau_f: f64,
    eta: f64,
    taucn: f64,
    theta: f64,
    phi: f64,
    tc: f64,
}

struct Erod13Fluxes {
    dc: f64,
    df: f64,
}

impl Wb11HydrologyKernel {
    pub(crate) fn run_erod13_wave1_core(
        request: &HillslopeKernelRequest<'_>,
        q_runoff: f64,
        peakro: f64,
        watdur: f64,
    ) -> Result<Vec<WritebackField>, Wb11HydrologyKernelGuardError> {
        if !Self::resolve_erod13_core_enabled(request)? {
            return Ok(Vec::new());
        }

        let symbols = Erod13Symbols::new();
        let inputs = Self::erod13_inputs(request, q_runoff, peakro, watdur, &symbols)?;
        Self::validate_erod13_runoff_continuity(&inputs.runoff)?;
        let derived = Self::erod13_derived_terms(&inputs.event, &inputs.process, &symbols)?;
        let fluxes = Self::erod13_transport_fluxes(&inputs.event, &derived, &symbols)?;
        Self::validate_erod13_dgdx_continuity(&inputs.event, fluxes.df, &symbols)?;

        Ok(Self::erod13_writebacks(&derived, &fluxes))
    }

    fn erod13_inputs(
        request: &HillslopeKernelRequest<'_>,
        q_runoff: f64,
        peakro: f64,
        watdur: f64,
        symbols: &Erod13Symbols,
    ) -> Result<Erod13Inputs, Wb11HydrologyKernelGuardError> {
        let event = Self::erod13_event_inputs(request, symbols)?;
        let process = Self::erod13_process_inputs(request, symbols)?;
        let runoff = Self::erod13_runoff_inputs(q_runoff, peakro, watdur)?;

        Ok(Erod13Inputs {
            event,
            process,
            runoff,
        })
    }

    fn erod13_event_inputs(
        request: &HillslopeKernelRequest<'_>,
        symbols: &Erod13Symbols,
    ) -> Result<Erod13EventInputs, Wb11HydrologyKernelGuardError> {
        let ie = Self::require_erod13_state_scalar(request, &symbols.ie)?;
        Self::require_erod13_domain(&symbols.ie, ie, Some(0.0), None)?;
        let te = Self::require_erod13_state_scalar(request, &symbols.te)?;
        Self::require_erod13_domain(&symbols.te, te, Some(WB11_ZERO_THRESHOLD), None)?;
        let fs = Self::require_erod13_state_scalar(request, &symbols.fs)?;
        Self::require_erod13_domain(&symbols.fs, fs, Some(0.0), None)?;
        let ft = Self::require_erod13_state_scalar(request, &symbols.ft)?;
        Self::require_erod13_domain(&symbols.ft, ft, Some(WB11_ZERO_THRESHOLD), None)?;
        Self::require_erod13_domain(&symbols.fs, fs, Some(0.0), Some(ft))?;
        let taufe = Self::require_erod13_state_scalar(request, &symbols.taufe)?;
        Self::require_erod13_domain(&symbols.taufe, taufe, Some(0.0), None)?;
        let q = Self::require_erod13_state_scalar(request, &symbols.q)?;
        Self::require_erod13_domain(&symbols.q, q, Some(0.0), None)?;
        let g = Self::require_erod13_state_scalar(request, &symbols.g)?;
        Self::require_erod13_domain(&symbols.g, g, Some(0.0), None)?;
        let di = Self::require_erod13_state_scalar(request, &symbols.di)?;
        Self::require_erod13_domain(&symbols.di, di, Some(0.0), None)?;
        let beta = Self::require_erod13_state_scalar(request, &symbols.beta)?;
        Self::require_erod13_domain(&symbols.beta, beta, Some(0.0), None)?;
        let vf = Self::require_erod13_state_scalar(request, &symbols.vf)?;
        Self::require_erod13_domain(&symbols.vf, vf, Some(0.0), None)?;
        let dgdx = Self::require_erod13_state_scalar(request, &symbols.dgdx)?;

        Ok(Erod13EventInputs {
            fs,
            ft,
            taufe,
            q,
            g,
            di,
            beta,
            vf,
            dgdx,
        })
    }

    #[allow(clippy::similar_names)]
    fn erod13_process_inputs(
        request: &HillslopeKernelRequest<'_>,
        symbols: &Erod13Symbols,
    ) -> Result<Erod13ProcessInputs, Wb11HydrologyKernelGuardError> {
        let cntlen = Self::require_erod13_state_scalar(request, &symbols.cntlen)?;
        Self::require_erod13_domain(&symbols.cntlen, cntlen, Some(WB11_ZERO_THRESHOLD), None)?;
        let kr = Self::require_erod13_state_scalar(request, &symbols.kr)?;
        Self::require_erod13_domain(&symbols.kr, kr, Some(WB11_ZERO_THRESHOLD), None)?;
        let kradjf = Self::require_erod13_state_scalar(request, &symbols.kradjf)?;
        Self::require_erod13_domain(&symbols.kradjf, kradjf, Some(WB11_ZERO_THRESHOLD), None)?;
        let tcadjf = Self::require_erod13_state_scalar(request, &symbols.tcadjf)?;
        Self::require_erod13_domain(&symbols.tcadjf, tcadjf, Some(EROD13_MIN_TCADJF), None)?;
        let shrsol = Self::require_erod13_state_scalar(request, &symbols.shrsol)?;
        Self::require_erod13_domain(&symbols.shrsol, shrsol, Some(WB11_ZERO_THRESHOLD), None)?;
        let tcend = Self::require_erod13_state_scalar(request, &symbols.tcend)?;
        Self::require_erod13_domain(&symbols.tcend, tcend, Some(WB11_ZERO_THRESHOLD), None)?;
        let shcrit = Self::require_erod13_state_scalar(request, &symbols.shcrit)?;
        Self::require_erod13_domain(&symbols.shcrit, shcrit, Some(0.0), None)?;
        let detinr = Self::require_erod13_state_scalar(request, &symbols.detinr)?;
        Self::require_erod13_domain(&symbols.detinr, detinr, Some(0.0), None)?;
        let effdrr = Self::require_erod13_state_scalar(request, &symbols.effdrr)?;
        Self::require_erod13_domain(&symbols.effdrr, effdrr, Some(WB11_ZERO_THRESHOLD), None)?;
        let effdrn = Self::require_erod13_state_scalar(request, &symbols.effdrn)?;
        Self::require_erod13_domain(&symbols.effdrn, effdrn, Some(WB11_ZERO_THRESHOLD), None)?;
        let veleff = Self::require_erod13_state_scalar(request, &symbols.veleff)?;
        Self::require_erod13_domain(&symbols.veleff, veleff, Some(0.0), None)?;
        let pkro = Self::require_erod13_state_scalar(request, &symbols.pkro)?;
        Self::require_erod13_domain(&symbols.pkro, pkro, Some(WB11_ZERO_THRESHOLD), None)?;
        let tc_k = Self::require_erod13_state_scalar(request, &symbols.tc_k)?;
        Self::require_erod13_domain(&symbols.tc_k, tc_k, Some(WB11_ZERO_THRESHOLD), None)?;
        let tc_m = Self::require_erod13_state_scalar(request, &symbols.tc_m)?;
        Self::require_erod13_domain(&symbols.tc_m, tc_m, Some(WB11_ZERO_THRESHOLD), None)?;

        Ok(Erod13ProcessInputs {
            cntlen,
            kr,
            kradjf,
            tcadjf,
            shrsol,
            tcend,
            shcrit,
            detinr,
            effdrr,
            effdrn,
            veleff,
            pkro,
            tc_k,
            tc_m,
        })
    }

    fn erod13_runoff_inputs(
        q_runoff: f64,
        peakro: f64,
        watdur: f64,
    ) -> Result<Erod13RunoffInputs, Wb11HydrologyKernelGuardError> {
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

        Ok(Erod13RunoffInputs {
            q_runoff,
            peakro,
            watdur,
        })
    }

    fn validate_erod13_runoff_continuity(
        runoff: &Erod13RunoffInputs,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        let expected_watdur = runoff.q_runoff / runoff.peakro;
        let continuity_residual = (runoff.watdur - expected_watdur).abs();
        if continuity_residual > EROD13_CONTINUITY_TOLERANCE + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                symbol: BoundarySymbol::from(WB16_SYMBOL_WATDUR),
                value: runoff.watdur,
                minimum: Some(expected_watdur - EROD13_CONTINUITY_TOLERANCE),
                maximum: Some(expected_watdur + EROD13_CONTINUITY_TOLERANCE),
            });
        }
        Ok(())
    }

    fn erod13_derived_terms(
        event: &Erod13EventInputs,
        process: &Erod13ProcessInputs,
        symbols: &Erod13Symbols,
    ) -> Result<Erod13DerivedTerms, Wb11HydrologyKernelGuardError> {
        let tau_f = event.taufe * (event.fs / event.ft);
        Self::require_erod13_nonnegative_derived(&symbols.taufe, tau_f)?;

        let eta = (process.cntlen * process.kr * process.kradjf * process.shrsol) / process.tcend;
        Self::require_erod13_nonnegative_derived(&BoundarySymbol::from(EROD13_SYMBOL_ETA), eta)?;
        let taucn = (process.tcadjf * process.shcrit) / process.shrsol;
        Self::require_erod13_nonnegative_derived(&BoundarySymbol::from(EROD13_SYMBOL_TAUCN), taucn)?;
        let theta = ((process.cntlen * process.detinr) / process.tcend)
            * (process.effdrr / process.effdrn);
        Self::require_erod13_nonnegative_derived(&BoundarySymbol::from(EROD13_SYMBOL_THETA), theta)?;
        let phi = (event.beta * process.veleff) / process.pkro;
        Self::require_erod13_nonnegative_derived(&BoundarySymbol::from(EROD13_SYMBOL_PHI), phi)?;

        let tc = process.tcadjf * process.tc_k * tau_f.powf(process.tc_m);
        Self::require_erod13_nonnegative_derived(&BoundarySymbol::from(EROD13_SYMBOL_TC), tc)?;

        Ok(Erod13DerivedTerms {
            tau_f,
            eta,
            taucn,
            theta,
            phi,
            tc,
        })
    }

    fn require_erod13_nonnegative_derived(
        symbol: &BoundarySymbol,
        value: f64,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if !value.is_finite() || value < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                symbol: symbol.clone(),
                value,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        Ok(())
    }

    fn erod13_transport_fluxes(
        event: &Erod13EventInputs,
        derived: &Erod13DerivedTerms,
        symbols: &Erod13Symbols,
    ) -> Result<Erod13Fluxes, Wb11HydrologyKernelGuardError> {
        if derived.tau_f > derived.taucn && event.g < derived.tc {
            return Self::erod13_detachment_fluxes(event, derived);
        }
        if event.g > derived.tc {
            return Self::erod13_deposition_fluxes(event, derived, symbols);
        }
        Ok(Erod13Fluxes { dc: 0.0, df: 0.0 })
    }

    fn erod13_detachment_fluxes(
        event: &Erod13EventInputs,
        derived: &Erod13DerivedTerms,
    ) -> Result<Erod13Fluxes, Wb11HydrologyKernelGuardError> {
        if derived.tc <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                symbol: BoundarySymbol::from(EROD13_SYMBOL_TC),
                value: derived.tc,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }
        let dc = derived.eta * (derived.tau_f - derived.taucn);
        Self::require_erod13_nonnegative_derived(&BoundarySymbol::from(EROD13_SYMBOL_DC), dc)?;
        let df = dc * ((derived.tc - event.g) / derived.tc);
        Self::require_erod13_nonnegative_derived(&BoundarySymbol::from(EROD13_SYMBOL_DF), df)?;
        Ok(Erod13Fluxes { dc, df })
    }

    fn erod13_deposition_fluxes(
        event: &Erod13EventInputs,
        derived: &Erod13DerivedTerms,
        symbols: &Erod13Symbols,
    ) -> Result<Erod13Fluxes, Wb11HydrologyKernelGuardError> {
        Self::require_erod13_domain(&symbols.q, event.q, Some(WB11_ZERO_THRESHOLD), None)?;
        let df = -((event.beta * event.vf / event.q) * (event.g - derived.tc));
        if !df.is_finite() || df > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                symbol: BoundarySymbol::from(EROD13_SYMBOL_DF),
                value: df,
                minimum: None,
                maximum: Some(0.0),
            });
        }
        Ok(Erod13Fluxes { dc: 0.0, df })
    }

    fn validate_erod13_dgdx_continuity(
        event: &Erod13EventInputs,
        df: f64,
        symbols: &Erod13Symbols,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        let expected_dgdx = df + event.di;
        let dgdx_residual = (event.dgdx - expected_dgdx).abs();
        if dgdx_residual > EROD13_CONTINUITY_TOLERANCE + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                symbol: symbols.dgdx.clone(),
                value: event.dgdx,
                minimum: Some(expected_dgdx - EROD13_CONTINUITY_TOLERANCE),
                maximum: Some(expected_dgdx + EROD13_CONTINUITY_TOLERANCE),
            });
        }
        Ok(())
    }

    fn erod13_writebacks(
        derived: &Erod13DerivedTerms,
        fluxes: &Erod13Fluxes,
    ) -> Vec<WritebackField> {
        vec![
            WritebackField::bounded(EROD13_SYMBOL_DC, fluxes.dc, Some(0.0), None),
            WritebackField::bounded(EROD13_SYMBOL_TC, derived.tc, Some(0.0), None),
            WritebackField::unbounded(EROD13_SYMBOL_DF, fluxes.df),
            WritebackField::bounded(EROD13_SYMBOL_ETA, derived.eta, Some(0.0), None),
            WritebackField::bounded(EROD13_SYMBOL_TAUCN, derived.taucn, Some(0.0), None),
            WritebackField::bounded(EROD13_SYMBOL_THETA, derived.theta, Some(0.0), None),
            WritebackField::bounded(EROD13_SYMBOL_PHI, derived.phi, Some(0.0), None),
        ]
    }
}
