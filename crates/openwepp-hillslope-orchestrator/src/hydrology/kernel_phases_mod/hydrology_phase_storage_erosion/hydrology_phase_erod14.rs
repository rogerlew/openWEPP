#[allow(clippy::wildcard_imports)]
use crate::hydrology::*;

#[derive(Clone, Copy)]
struct Erod14ClassCount {
    value: usize,
    as_f64: f64,
}

struct Erod14RawInputs {
    xtop: f64,
    xbot: f64,
    xdetst: f64,
    ldtop: f64,
    ldbot: f64,
    lddend: f64,
    qout: f64,
    qin: f64,
    qostar: f64,
    slplen: f64,
    ktrato: f64,
    aintc: f64,
    bintc: f64,
    cintc: f64,
    beta: f64,
    qj_minus_1: f64,
    vj: f64,
    qj: f64,
    fh: f64,
    fp: f64,
    case_value: f64,
    ssa_soil: f64,
}

struct Erod14Inputs {
    xtop: f64,
    xbot: f64,
    ldtop: f64,
    ldbot: f64,
    lddend: f64,
    qout: f64,
    qin: f64,
    qostar: f64,
    slplen: f64,
    ktrato: f64,
    aintc: f64,
    bintc: f64,
    cintc: f64,
    beta: f64,
    ssa_soil: f64,
}

struct Erod14ClassState {
    fall: Vec<f64>,
    frcflw: Vec<f64>,
    tcf1: Vec<f64>,
    ssa_class: Vec<f64>,
    ftheta: Vec<f64>,
    gu: Vec<f64>,
    gend: Vec<f64>,
    sedmax: Vec<f64>,
    sed_frac: Vec<f64>,
}

impl From<Erod14RawInputs> for Erod14Inputs {
    fn from(raw: Erod14RawInputs) -> Self {
        Self {
            xtop: raw.xtop,
            xbot: raw.xbot,
            ldtop: raw.ldtop,
            ldbot: raw.ldbot,
            lddend: raw.lddend,
            qout: raw.qout,
            qin: raw.qin,
            qostar: raw.qostar,
            slplen: raw.slplen,
            ktrato: raw.ktrato,
            aintc: raw.aintc,
            bintc: raw.bintc,
            cintc: raw.cintc,
            beta: raw.beta,
            ssa_soil: raw.ssa_soil,
        }
    }
}

impl Wb11HydrologyKernel {
    pub(crate) fn run_erod14_wave2(
        request: &HillslopeKernelRequest<'_>,
        erod13_state_updates: &[WritebackField],
    ) -> Result<Vec<WritebackField>, Wb11HydrologyKernelGuardError> {
        if !Self::resolve_erod14_wave2_enabled(request)? {
            return Ok(Vec::new());
        }

        let class_count = Self::erod14_class_count(request)?;
        let inputs = Self::erod14_inputs(request)?;
        let theta = Self::erod14_theta(request, erod13_state_updates)?;
        let mut classes =
            Self::erod14_load_class_state(request, class_count.value, theta, inputs.ldtop)?;

        if inputs.qout <= WB11_ZERO_THRESHOLD {
            return Ok(Self::erod14_zero_outflow_updates(
                class_count,
                inputs.lddend,
            ));
        }

        let pkro = Self::erod14_pkro(&inputs)?;
        let mut sumg = Self::erod14_project_initial_gend(&inputs, &mut classes, pkro)?;
        Self::erod14_reproportion_to_ldbot(&inputs, &mut classes, &mut sumg)?;
        let sumg = Self::erod14_validate_sumg_and_caps(&classes)?;
        Self::erod14_update_transport_fractions(&mut classes, sumg)?;
        let er = Self::erod14_enrichment_ratio(&inputs, &classes, sumg)?;

        Self::erod14_final_updates(class_count, &inputs, &classes, sumg, er)
    }

    fn erod14_class_count(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<Erod14ClassCount, Wb11HydrologyKernelGuardError> {
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
            return Err(Self::erod14_class_count_violation(class_count_value));
        }

        let class_count = format!("{class_count_rounded:.0}")
            .parse::<usize>()
            .map_err(|_| Self::erod14_class_count_violation(class_count_value))?;
        if class_count == 0 {
            return Err(Self::erod14_class_count_violation(class_count_value));
        }

        let as_f64 = f64::from(
            u32::try_from(class_count)
                .map_err(|_| Self::erod14_class_count_violation(class_count_value))?,
        );
        Ok(Erod14ClassCount {
            value: class_count,
            as_f64,
        })
    }

    fn erod14_class_count_violation(value: f64) -> Wb11HydrologyKernelGuardError {
        Wb11HydrologyKernelGuardError::Erod14DomainViolation {
            symbol: BoundarySymbol::from(EROD14_SYMBOL_CLASS_COUNT),
            value,
            minimum: Some(1.0),
            maximum: None,
        }
    }

    fn erod14_inputs(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<Erod14Inputs, Wb11HydrologyKernelGuardError> {
        let raw = Self::erod14_load_raw_inputs(request)?;
        Self::erod14_validate_raw_inputs(&raw)?;
        Ok(raw.into())
    }

    fn erod14_load_raw_inputs(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<Erod14RawInputs, Wb11HydrologyKernelGuardError> {
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

        Ok(Erod14RawInputs {
            xtop,
            xbot,
            xdetst,
            ldtop,
            ldbot,
            lddend,
            qout,
            qin,
            qostar,
            slplen,
            ktrato,
            aintc,
            bintc,
            cintc,
            beta,
            qj_minus_1,
            vj,
            qj,
            fh,
            fp,
            case_value,
            ssa_soil,
        })
    }

    fn erod14_validate_raw_inputs(
        raw: &Erod14RawInputs,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        Self::require_erod14_domain(
            &BoundarySymbol::from(EROD14_SYMBOL_XTOP),
            raw.xtop,
            Some(0.0),
            None,
        )?;
        Self::require_erod14_domain(
            &BoundarySymbol::from(EROD14_SYMBOL_XBOT),
            raw.xbot,
            Some(raw.xtop),
            None,
        )?;
        Self::require_erod14_domain(
            &BoundarySymbol::from(EROD14_SYMBOL_XDETST),
            raw.xdetst,
            Some(0.0),
            Some(raw.xtop),
        )?;
        Self::require_erod14_domain(
            &BoundarySymbol::from(EROD14_SYMBOL_LDTOP),
            raw.ldtop,
            Some(0.0),
            None,
        )?;
        Self::require_erod14_domain(
            &BoundarySymbol::from(EROD14_SYMBOL_LDBOT),
            raw.ldbot,
            Some(0.0),
            None,
        )?;
        Self::require_erod14_domain(
            &BoundarySymbol::from(EROD14_SYMBOL_LDDEND),
            raw.lddend,
            Some(0.0),
            Some(raw.ldtop),
        )?;
        Self::erod14_validate_flow_inputs(raw)?;
        Self::erod14_validate_case(
            raw.case_value,
            raw.qj_minus_1,
            raw.vj,
            raw.qj,
            raw.fh,
            raw.fp,
        )
    }

    fn erod14_validate_flow_inputs(
        raw: &Erod14RawInputs,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        Self::require_erod14_domain(
            &BoundarySymbol::from(EROD14_SYMBOL_QOUT),
            raw.qout,
            Some(0.0),
            None,
        )?;
        Self::require_erod14_domain(
            &BoundarySymbol::from(EROD14_SYMBOL_QIN),
            raw.qin,
            Some(0.0),
            None,
        )?;
        Self::require_erod14_domain(
            &BoundarySymbol::from(EROD14_SYMBOL_SLP_LEN),
            raw.slplen,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;
        Self::require_erod14_domain(
            &BoundarySymbol::from(EROD14_SYMBOL_KTRATO),
            raw.ktrato,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;
        Self::require_erod14_domain(
            &BoundarySymbol::from(EROD14_SYMBOL_BETA),
            raw.beta,
            Some(0.0),
            None,
        )?;
        Self::require_erod14_domain(
            &BoundarySymbol::from(EROD14_SYMBOL_QJ_MINUS_1),
            raw.qj_minus_1,
            Some(0.0),
            None,
        )?;
        Self::require_erod14_domain(
            &BoundarySymbol::from(EROD14_SYMBOL_VJ),
            raw.vj,
            Some(0.0),
            None,
        )?;
        Self::require_erod14_domain(
            &BoundarySymbol::from(EROD14_SYMBOL_QJ),
            raw.qj,
            Some(0.0),
            None,
        )?;
        Self::require_erod14_domain(
            &BoundarySymbol::from(EROD14_SYMBOL_FH),
            raw.fh,
            Some(0.0),
            None,
        )?;
        Self::require_erod14_domain(
            &BoundarySymbol::from(EROD14_SYMBOL_FP),
            raw.fp,
            Some(0.0),
            None,
        )?;
        Self::require_erod14_domain(
            &BoundarySymbol::from(EROD14_SYMBOL_SSA_SOIL),
            raw.ssa_soil,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )
    }

    fn erod14_validate_case(
        case_value: f64,
        qj_minus_1: f64,
        vj: f64,
        qj: f64,
        fh: f64,
        fp: f64,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        let case_number = Self::erod14_case_number(case_value)?;
        if !Self::erod14_case_matches(case_number, qj_minus_1, vj, qj, fh, fp) {
            return Err(Self::erod14_case_violation(case_value));
        }
        Ok(())
    }

    fn erod14_case_number(case_value: f64) -> Result<i32, Wb11HydrologyKernelGuardError> {
        let case_symbol = BoundarySymbol::from(EROD14_SYMBOL_CASE);
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
            return Err(Self::erod14_case_violation(case_value));
        }
        Ok(case_number)
    }

    fn erod14_case_matches(
        case_number: i32,
        qj_minus_1: f64,
        vj: f64,
        qj: f64,
        fh: f64,
        fp: f64,
    ) -> bool {
        match case_number {
            1 => Self::erod14_case_one_matches(qj_minus_1, vj, qj),
            2 => Self::erod14_case_two_matches(qj_minus_1, vj, qj),
            3 => Self::erod14_case_three_matches(qj_minus_1, vj, qj, fh, fp),
            4 => Self::erod14_case_four_matches(qj_minus_1, vj, qj, fh, fp),
            _ => false,
        }
    }

    fn erod14_case_is_zero(value: f64) -> bool {
        value.abs() <= EROD14_CASE_TOLERANCE
    }

    fn erod14_case_one_matches(qj_minus_1: f64, vj: f64, qj: f64) -> bool {
        Self::erod14_case_is_zero(qj_minus_1)
            && Self::erod14_case_is_zero(vj)
            && Self::erod14_case_is_zero(qj)
    }

    fn erod14_case_two_matches(qj_minus_1: f64, vj: f64, qj: f64) -> bool {
        qj_minus_1 > EROD14_CASE_TOLERANCE
            && vj > EROD14_CASE_TOLERANCE
            && qj > EROD14_CASE_TOLERANCE
    }

    fn erod14_case_three_matches(qj_minus_1: f64, vj: f64, qj: f64, fh: f64, fp: f64) -> bool {
        qj_minus_1 > EROD14_CASE_TOLERANCE
            && Self::erod14_case_is_zero(vj)
            && (fh - fp) > EROD14_CASE_TOLERANCE
            && qj > EROD14_CASE_TOLERANCE
    }

    fn erod14_case_four_matches(qj_minus_1: f64, vj: f64, qj: f64, fh: f64, fp: f64) -> bool {
        qj_minus_1 > EROD14_CASE_TOLERANCE
            && Self::erod14_case_is_zero(vj)
            && (fh - fp) <= EROD14_CASE_TOLERANCE
            && Self::erod14_case_is_zero(qj)
    }

    fn erod14_case_violation(value: f64) -> Wb11HydrologyKernelGuardError {
        Wb11HydrologyKernelGuardError::Erod14DomainViolation {
            symbol: BoundarySymbol::from(EROD14_SYMBOL_CASE),
            value,
            minimum: Some(f64::from(EROD14_CASE_MIN)),
            maximum: Some(f64::from(EROD14_CASE_MAX)),
        }
    }

    fn erod14_theta(
        request: &HillslopeKernelRequest<'_>,
        erod13_state_updates: &[WritebackField],
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
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
        Ok(theta)
    }

    fn erod14_load_class_state(
        request: &HillslopeKernelRequest<'_>,
        class_count: usize,
        theta: f64,
        ldtop: f64,
    ) -> Result<Erod14ClassState, Wb11HydrologyKernelGuardError> {
        let mut state = Erod14ClassState {
            fall: Vec::with_capacity(class_count),
            frcflw: Vec::with_capacity(class_count),
            tcf1: Vec::with_capacity(class_count),
            ssa_class: Vec::with_capacity(class_count),
            ftheta: Vec::with_capacity(class_count),
            gu: Vec::with_capacity(class_count),
            gend: vec![0.0_f64; class_count],
            sedmax: vec![0.0_f64; class_count],
            sed_frac: vec![0.0_f64; class_count],
        };

        for class_index in 1..=class_count {
            Self::erod14_push_class_state(request, &mut state, class_index, theta, ldtop)?;
        }
        Ok(state)
    }

    fn erod14_push_class_state(
        request: &HillslopeKernelRequest<'_>,
        state: &mut Erod14ClassState,
        class_index: usize,
        theta: f64,
        ldtop: f64,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
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

        state.fall.push(fall_value);
        state.frcflw.push(frcflw_value);
        state.tcf1.push(tcf1_value);
        state.ssa_class.push(ssa_class_value);
        state.ftheta.push(fidel_value * theta);
        state.gu.push(frcflw_value * ldtop);
        Ok(())
    }

    fn erod14_zero_outflow_updates(
        class_count: Erod14ClassCount,
        lddend: f64,
    ) -> Vec<WritebackField> {
        let mut updates = Vec::with_capacity(
            EROD14_BASE_UPDATE_FIELD_COUNT
                + (class_count.value * EROD14_CLASS_UPDATE_FIELD_COUNT),
        );
        Self::erod14_push_base_updates(&mut updates, 0.0, 0.0, lddend, class_count);
        for class_index in 1..=class_count.value {
            Self::erod14_push_class_updates(&mut updates, class_index, 0.0, 0.0, 0.0, 0.0, 0.0);
        }
        updates
    }

    fn erod14_pkro(inputs: &Erod14Inputs) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let pkro = (inputs.qout - inputs.qin) / inputs.slplen;
        if !pkro.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: BoundarySymbol::from(EROD14_SYMBOL_QOUT),
                value: pkro,
                minimum: None,
                maximum: None,
            });
        }
        Ok(pkro)
    }

    fn erod14_project_initial_gend(
        inputs: &Erod14Inputs,
        classes: &mut Erod14ClassState,
        pkro: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let tmpvr2 = inputs.xbot + inputs.qostar;
        let tmpvr3 = inputs.xtop + inputs.qostar;
        if tmpvr2.abs() <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: BoundarySymbol::from(EROD14_SYMBOL_QOSTAR),
                value: tmpvr2,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }
        let mut sumg = 0.0_f64;
        for i in 0..classes.gend.len() {
            let gend_i =
                Self::erod14_project_class_gend(inputs, classes, i, pkro, tmpvr2, tmpvr3)?;
            classes.gend[i] = gend_i;
            sumg += gend_i;
        }
        Ok(sumg)
    }

    fn erod14_project_class_gend(
        inputs: &Erod14Inputs,
        classes: &Erod14ClassState,
        i: usize,
        pkro: f64,
        tmpvr2: f64,
        tmpvr3: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let tmpvr4 = tmpvr2 * tmpvr2;
        let tmpvr5 = tmpvr3 * tmpvr3;
        let tmpvr1 = inputs.ktrato * classes.tcf1[i];
        let aa = tmpvr1 * inputs.aintc;
        let bb = tmpvr1 * inputs.bintc;
        let cc = tmpvr1 * inputs.cintc;

        let phi = Self::erod14_phi(inputs, classes.fall[i], pkro);
        let attenuation_factor = Self::erod14_attenuation_factor(inputs, i, phi, tmpvr2, tmpvr3)?;
        let denom_coef1 = phi + 2.0;
        let denom_coef2 = phi + 1.0;
        if denom_coef1.abs() <= WB11_ZERO_THRESHOLD || denom_coef2.abs() <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: Self::erod14_class_symbol(EROD14_ROOT_FALL, i + 1),
                value: phi,
                minimum: Some(-EROD14_MAX_PHI),
                maximum: Some(EROD14_MAX_PHI),
            });
        }

        let coef1 = phi * aa / denom_coef1;
        let coef2 =
            (phi * bb + classes.ftheta[i] - 2.0 * aa * phi * inputs.qostar) / denom_coef2;
        let term1 = coef1 * tmpvr4;
        let term2 = coef2 * tmpvr2;
        let term3 = aa * inputs.qostar * inputs.qostar - bb * inputs.qostar + cc;
        let attenuation_tail = classes.gu[i] - coef1 * tmpvr5 - coef2 * tmpvr3 - term3;
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
        Ok(gend_i)
    }

    fn erod14_phi(inputs: &Erod14Inputs, fall: f64, pkro: f64) -> f64 {
        let mut phi = if pkro.abs() > EROD14_PKRO_ZERO_THRESHOLD {
            (inputs.beta * fall) / pkro
        } else if inputs.qostar >= 0.0 {
            EROD14_MAX_PHI
        } else {
            -EROD14_MAX_PHI
        };
        phi = phi.clamp(-EROD14_MAX_PHI, EROD14_MAX_PHI);
        phi
    }

    fn erod14_attenuation_factor(
        inputs: &Erod14Inputs,
        i: usize,
        phi: f64,
        tmpvr2: f64,
        tmpvr3: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let mut ratio = tmpvr3 / tmpvr2;
        if inputs.qostar >= 0.0 && ratio > 1.0 {
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
        Ok(attenuation_factor)
    }

    fn erod14_reproportion_to_ldbot(
        inputs: &Erod14Inputs,
        classes: &mut Erod14ClassState,
        sumg: &mut f64,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if *sumg <= 0.0 {
            return Ok(());
        }

        for i in 0..classes.gend.len() {
            classes.gend[i] = classes.gend[i] * inputs.ldbot / *sumg;
            classes.sedmax[i] = classes.gu[i] + classes.ftheta[i] * (inputs.xbot - inputs.xtop);
            Self::require_erod14_domain(
                &Self::erod14_class_symbol(EROD14_ROOT_SEDMAX, i + 1),
                classes.sedmax[i],
                Some(0.0),
                None,
            )?;
            if classes.gend[i] < EROD14_CLASS_FLOOR {
                classes.gend[i] = EROD14_CLASS_FLOOR;
            }
        }

        let mut converged = false;
        for _ in 0..EROD14_MAX_REPROPORTION_ITERS {
            if Self::erod14_reproportion_iteration(inputs, classes, sumg)? {
                converged = true;
                break;
            }
        }

        if !converged {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: BoundarySymbol::from(EROD14_SYMBOL_LDBOT),
                value: inputs.ldbot,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        Ok(())
    }

    fn erod14_reproportion_iteration(
        inputs: &Erod14Inputs,
        classes: &mut Erod14ClassState,
        sumg: &mut f64,
    ) -> Result<bool, Wb11HydrologyKernelGuardError> {
        let mut ratbot = 0.0_f64;
        *sumg = 0.0;
        let mut adjusted = false;

        for i in 0..classes.gend.len() {
            if classes.gend[i] > classes.sedmax[i] + WB11_ZERO_THRESHOLD {
                classes.gend[i] = classes.sedmax[i];
                adjusted = true;
            } else if classes.gend[i] < classes.sedmax[i] - WB11_ZERO_THRESHOLD {
                ratbot += classes.gend[i];
            }
            *sumg += classes.gend[i];
        }

        if !adjusted {
            return Ok(true);
        }

        // Baseline enrich.for semantics: when clipping saturates every class
        // (`ratbot == 0`), re-enter the clipping loop instead of failing.
        if ratbot <= WB11_ZERO_THRESHOLD {
            return Ok(false);
        }

        let gdeficit = inputs.ldbot - *sumg;
        for i in 0..classes.gend.len() {
            if classes.gend[i] < classes.sedmax[i] - WB11_ZERO_THRESHOLD {
                let gadd = gdeficit * classes.gend[i] / ratbot;
                let updated = classes.gend[i] + gadd;
                if !updated.is_finite() {
                    return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                        symbol: Self::erod14_class_symbol(EROD14_ROOT_GEND, i + 1),
                        value: updated,
                        minimum: Some(0.0),
                        maximum: None,
                    });
                }
                classes.gend[i] = updated;
            }
        }
        Ok(false)
    }

    fn erod14_validate_sumg_and_caps(
        classes: &Erod14ClassState,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let sumg: f64 = classes.gend.iter().sum();
        if !sumg.is_finite() || sumg < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: BoundarySymbol::from(EROD14_SYMBOL_SUMG),
                value: sumg,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        for i in 0..classes.gend.len() {
            if classes.gend[i] > classes.sedmax[i] + WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                    symbol: Self::erod14_class_symbol(EROD14_ROOT_GEND, i + 1),
                    value: classes.gend[i],
                    minimum: Some(0.0),
                    maximum: Some(classes.sedmax[i]),
                });
            }
        }
        Ok(sumg)
    }

    fn erod14_update_transport_fractions(
        classes: &mut Erod14ClassState,
        sumg: f64,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if sumg > 0.0 {
            for i in 0..classes.gend.len() {
                classes.frcflw[i] = classes.gend[i] / sumg;
                classes.sed_frac[i] = classes.frcflw[i];
            }
            let sed_frac_sum: f64 = classes.sed_frac.iter().sum();
            if (sed_frac_sum - 1.0).abs() > EROD13_CONTINUITY_TOLERANCE + WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                    symbol: BoundarySymbol::from(EROD14_ROOT_SED_FRAC),
                    value: sed_frac_sum,
                    minimum: Some(1.0 - EROD13_CONTINUITY_TOLERANCE),
                    maximum: Some(1.0 + EROD13_CONTINUITY_TOLERANCE),
                });
            }
        } else {
            for i in 0..classes.gend.len() {
                classes.frcflw[i] = 0.0;
                classes.sed_frac[i] = 0.0;
            }
        }
        Ok(())
    }

    fn erod14_enrichment_ratio(
        inputs: &Erod14Inputs,
        classes: &Erod14ClassState,
        sumg: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let mut sumssa = 0.0_f64;
        for i in 0..classes.sed_frac.len() {
            sumssa += classes.sed_frac[i] * classes.ssa_class[i];
        }
        let er = if sumg > 0.0 {
            (sumssa / inputs.ssa_soil) + EROD14_ENRICHMENT_RATIO_OFFSET
        } else {
            0.0
        };
        if !er.is_finite() || er < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: BoundarySymbol::from(EROD14_SYMBOL_ER),
                value: er,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        Ok(er)
    }

    fn erod14_final_updates(
        class_count: Erod14ClassCount,
        inputs: &Erod14Inputs,
        classes: &Erod14ClassState,
        sumg: f64,
        er: f64,
    ) -> Result<Vec<WritebackField>, Wb11HydrologyKernelGuardError> {
        let mut updates = Vec::with_capacity(
            EROD14_BASE_UPDATE_FIELD_COUNT
                + (class_count.value * EROD14_CLASS_UPDATE_FIELD_COUNT),
        );
        Self::erod14_push_base_updates(&mut updates, sumg.max(0.0), er, inputs.lddend, class_count);

        for class_index in 1..=class_count.value {
            let i = class_index - 1;
            let concentration = if inputs.qout > WB11_ZERO_THRESHOLD {
                classes.gend[i] / inputs.qout
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
            Self::erod14_push_class_updates(
                &mut updates,
                class_index,
                classes.gend[i],
                classes.sedmax[i],
                classes.frcflw[i],
                classes.sed_frac[i],
                concentration,
            );
        }

        Ok(updates)
    }

    fn erod14_push_base_updates(
        updates: &mut Vec<WritebackField>,
        sumg: f64,
        er: f64,
        lddend: f64,
        class_count: Erod14ClassCount,
    ) {
        updates.push(WritebackField::bounded(
            EROD14_SYMBOL_SUMG,
            sumg,
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
            sumg,
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
            class_count.as_f64,
            Some(1.0),
            None,
        ));
    }

    fn erod14_push_class_updates(
        updates: &mut Vec<WritebackField>,
        class_index: usize,
        gend: f64,
        sedmax: f64,
        frcflw: f64,
        sed_frac: f64,
        concentration: f64,
    ) {
        updates.push(WritebackField::bounded(
            Self::erod14_class_symbol(EROD14_ROOT_GEND, class_index),
            gend,
            Some(0.0),
            None,
        ));
        updates.push(WritebackField::bounded(
            Self::erod14_class_symbol(EROD14_ROOT_SEDMAX, class_index),
            sedmax,
            Some(0.0),
            None,
        ));
        updates.push(WritebackField::bounded(
            Self::erod14_class_symbol(EROD14_ROOT_FRCFLW, class_index),
            frcflw,
            Some(0.0),
            Some(1.0),
        ));
        updates.push(WritebackField::bounded(
            Self::erod14_class_symbol(EROD14_ROOT_SED_FRAC, class_index),
            sed_frac,
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
            sed_frac,
            Some(0.0),
            Some(1.0),
        ));
    }
}
