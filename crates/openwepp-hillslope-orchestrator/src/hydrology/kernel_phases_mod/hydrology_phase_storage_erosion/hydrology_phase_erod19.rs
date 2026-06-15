#[allow(clippy::wildcard_imports)]
use crate::hydrology::*;

struct Erod19XcritInputs {
    coefficient_a: f64,
    coefficient_b: f64,
    critical_shear: f64,
    segment_start: f64,
    segment_end: f64,
    threshold_offset: f64,
    upstream_shear: f64,
    downstream_shear: f64,
}

struct Erod19XcritResult {
    shear_class: f64,
    critical_start: f64,
    critical_end: f64,
}

impl Erod19XcritResult {
    fn clamped_tuple(self, segment_start: f64, segment_end: f64) -> (f64, f64, f64) {
        (
            self.shear_class,
            self.critical_start.clamp(segment_start, segment_end),
            self.critical_end.clamp(segment_start, segment_end),
        )
    }
}

impl Wb11HydrologyKernel {
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

pub(crate) fn erod19_xcrit_classification(
        a: f64,
        b: f64,
        c: f64,
        tauc: f64,
        xb: f64,
        xe: f64,
    ) -> (f64, f64, f64) {
        let mut tauchk = tauc.powf(1.5) - c;
        if tauchk < 0.0 {
            tauchk = 0.0;
        }

        let inputs = Erod19XcritInputs {
            coefficient_a: a,
            coefficient_b: b,
            critical_shear: tauc,
            segment_start: xb,
            segment_end: xe,
            threshold_offset: tauchk,
            upstream_shear: Self::erod19_shear(a, b, c, xb),
            downstream_shear: Self::erod19_shear(a, b, c, xe),
        };
        let result = Self::erod19_xcrit_unclamped(&inputs);

        result.clamped_tuple(xb, xe)
    }

    fn erod19_xcrit_unclamped(inputs: &Erod19XcritInputs) -> Erod19XcritResult {
        if inputs.coefficient_a.abs() <= WB11_ZERO_THRESHOLD {
            return Self::erod19_linear_xcrit_classification(inputs);
        }
        if inputs.coefficient_a > 0.0 && inputs.downstream_shear > inputs.upstream_shear {
            return Self::erod19_rising_xcrit_classification(inputs);
        }
        if inputs.downstream_shear >= inputs.critical_shear
            && inputs.upstream_shear >= inputs.critical_shear
        {
            return Erod19XcritResult {
                shear_class: 2.0,
                critical_start: inputs.segment_start,
                critical_end: inputs.segment_end,
            };
        }

        Self::erod19_curved_xcrit_classification(inputs)
    }

    fn erod19_linear_xcrit_classification(
        inputs: &Erod19XcritInputs,
    ) -> Erod19XcritResult {
        let critical_start = if inputs.coefficient_b.abs() > WB11_ZERO_THRESHOLD {
            inputs.threshold_offset / inputs.coefficient_b
        } else {
            EROD19_UNIFORM_XC_SENTINEL
        };
        let shear_class = if inputs.downstream_shear > inputs.upstream_shear {
            Self::erod19_increasing_linear_shear_class(
                critical_start,
                inputs.segment_start,
                inputs.segment_end,
            )
        } else {
            Self::erod19_decreasing_linear_shear_class(
                critical_start,
                inputs.segment_start,
                inputs.segment_end,
            )
        };

        Erod19XcritResult {
            shear_class,
            critical_start,
            critical_end: inputs.segment_end,
        }
    }

    fn erod19_increasing_linear_shear_class(
        critical_start: f64,
        segment_start: f64,
        segment_end: f64,
    ) -> f64 {
        let mut shear_class = 3.0;
        if critical_start <= segment_start {
            shear_class = 2.0;
        }
        if critical_start >= segment_end {
            shear_class = 1.0;
        }
        shear_class
    }

    fn erod19_decreasing_linear_shear_class(
        critical_start: f64,
        segment_start: f64,
        segment_end: f64,
    ) -> f64 {
        let mut shear_class = 4.0;
        if critical_start >= segment_end {
            shear_class = 2.0;
        }
        if critical_start <= segment_start {
            shear_class = 1.0;
        }
        shear_class
    }

    fn erod19_rising_xcrit_classification(
        inputs: &Erod19XcritInputs,
    ) -> Erod19XcritResult {
        if inputs.upstream_shear >= inputs.critical_shear {
            return Erod19XcritResult {
                shear_class: 2.0,
                critical_start: inputs.segment_start,
                critical_end: inputs.segment_end,
            };
        }
        if inputs.downstream_shear <= inputs.critical_shear {
            return Erod19XcritResult {
                shear_class: 1.0,
                critical_start: inputs.segment_start,
                critical_end: inputs.segment_end,
            };
        }

        let mut critical_start = inputs.segment_start;
        if let Some((lower_root, upper_root)) = Self::erod19_root(
            inputs.coefficient_a,
            inputs.coefficient_b,
            inputs.threshold_offset,
        ) {
            if lower_root >= inputs.segment_start && lower_root <= inputs.segment_end {
                critical_start = lower_root;
            } else if upper_root >= inputs.segment_start && upper_root <= inputs.segment_end {
                critical_start = upper_root;
            }
        }

        Erod19XcritResult {
            shear_class: 3.0,
            critical_start,
            critical_end: inputs.segment_end,
        }
    }

    fn erod19_curved_xcrit_classification(
        inputs: &Erod19XcritInputs,
    ) -> Erod19XcritResult {
        let part = (inputs.coefficient_b * inputs.coefficient_b)
            + (4.0 * inputs.coefficient_a * inputs.threshold_offset);
        if part <= 0.0 {
            return Erod19XcritResult {
                shear_class: 1.0,
                critical_start: inputs.segment_start,
                critical_end: inputs.segment_end,
            };
        }
        if let Some((lower_root, upper_root)) = Self::erod19_root(
            inputs.coefficient_a,
            inputs.coefficient_b,
            inputs.threshold_offset,
        ) {
            return Self::erod19_curved_root_xcrit_classification(
                inputs, lower_root, upper_root,
            );
        }

        Erod19XcritResult {
            shear_class: 1.0,
            critical_start: inputs.segment_start,
            critical_end: inputs.segment_end,
        }
    }

    fn erod19_curved_root_xcrit_classification(
        inputs: &Erod19XcritInputs,
        lower_root: f64,
        upper_root: f64,
    ) -> Erod19XcritResult {
        if inputs.upstream_shear <= inputs.critical_shear
            && inputs.downstream_shear >= inputs.critical_shear
        {
            return Erod19XcritResult {
                shear_class: 3.0,
                critical_start: Self::erod19_segment_root(lower_root, upper_root, inputs),
                critical_end: inputs.segment_end,
            };
        }
        if inputs.upstream_shear >= inputs.critical_shear
            && inputs.downstream_shear <= inputs.critical_shear
        {
            return Erod19XcritResult {
                shear_class: 4.0,
                critical_start: Self::erod19_segment_root(lower_root, upper_root, inputs),
                critical_end: inputs.segment_end,
            };
        }
        if inputs.upstream_shear <= inputs.critical_shear
            && inputs.downstream_shear <= inputs.critical_shear
        {
            let shear_class = if lower_root < inputs.segment_start
                || lower_root > inputs.segment_end
                || upper_root < inputs.segment_start
                || upper_root > inputs.segment_end
                || (lower_root - upper_root).abs() <= WB11_ZERO_THRESHOLD
            {
                1.0
            } else {
                5.0
            };
            return Erod19XcritResult {
                shear_class,
                critical_start: lower_root,
                critical_end: upper_root,
            };
        }

        Erod19XcritResult {
            shear_class: 1.0,
            critical_start: inputs.segment_start,
            critical_end: inputs.segment_end,
        }
    }

    fn erod19_segment_root(
        lower_root: f64,
        upper_root: f64,
        inputs: &Erod19XcritInputs,
    ) -> f64 {
        if lower_root <= inputs.segment_start || lower_root >= inputs.segment_end {
            upper_root
        } else {
            lower_root
        }
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

}
