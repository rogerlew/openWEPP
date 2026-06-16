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

struct Erod19RouteTopology {
    nslpts_symbol: BoundarySymbol,
    nslpts_value: f64,
    min_segment_value: f64,
    segment_index: usize,
}

struct Erod19RouteSegmentSymbols {
    xu: BoundarySymbol,
    xl: BoundarySymbol,
    ainf: BoundarySymbol,
    binf: BoundarySymbol,
    cinf: BoundarySymbol,
    ainftc: BoundarySymbol,
    binftc: BoundarySymbol,
    cinftc: BoundarySymbol,
}

impl Erod19RouteSegmentSymbols {
    fn for_segment(segment_index: usize) -> Self {
        Self {
            xu: Wb11HydrologyKernel::erod18_route_segment_symbol(
                EROD18_ROOT_XU,
                segment_index,
            ),
            xl: Wb11HydrologyKernel::erod18_route_segment_symbol(
                EROD18_ROOT_XL,
                segment_index,
            ),
            ainf: Wb11HydrologyKernel::erod18_route_segment_symbol(
                EROD18_ROOT_AINF,
                segment_index,
            ),
            binf: Wb11HydrologyKernel::erod18_route_segment_symbol(
                EROD18_ROOT_BINF,
                segment_index,
            ),
            cinf: Wb11HydrologyKernel::erod18_route_segment_symbol(
                EROD18_ROOT_CINF,
                segment_index,
            ),
            ainftc: Wb11HydrologyKernel::erod18_route_segment_symbol(
                EROD18_ROOT_AINTC,
                segment_index,
            ),
            binftc: Wb11HydrologyKernel::erod18_route_segment_symbol(
                EROD18_ROOT_BINTC,
                segment_index,
            ),
            cinftc: Wb11HydrologyKernel::erod18_route_segment_symbol(
                EROD18_ROOT_CINTC,
                segment_index,
            ),
        }
    }
}

struct Erod19RouteSegmentScalars {
    xu: f64,
    xl: f64,
    ainf: f64,
    binf: f64,
    cinf: f64,
    ainftc: f64,
    binftc: f64,
    cinftc: f64,
}

struct Erod19RouteDrivers {
    qostar_symbol: BoundarySymbol,
    qostar: f64,
    xdetst: f64,
    ktrato: f64,
    theta: f64,
    phi: f64,
    tauc: f64,
    ldlast: f64,
}

struct Erod19RouteDepositionInputs {
    xu: f64,
    xl: f64,
    ainftc: f64,
    binftc: f64,
    cinftc: f64,
    phi: f64,
    theta: f64,
    ktrato: f64,
    qostar: f64,
    ldlast: f64,
    mshear: f64,
    du: f64,
}

struct Erod19RouteDepositionOutputs {
    xdbeg: f64,
    xdend: f64,
    ndep: f64,
    lddend_out: f64,
    ldlast_out: f64,
    du: f64,
    dl: f64,
}

struct Erod19RouteClassification {
    mshear: f64,
    xc1: f64,
    xc2: f64,
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

    fn erod19_route_topology(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<Erod19RouteTopology, Wb11HydrologyKernelGuardError> {
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

        Ok(Erod19RouteTopology {
            nslpts_symbol,
            nslpts_value,
            min_segment_value,
            segment_index: EROD18_ROUTE_SEGMENT_INDEX,
        })
    }

    fn erod19_route_segment_scalars(
        request: &HillslopeKernelRequest<'_>,
        symbols: &Erod19RouteSegmentSymbols,
    ) -> Result<Erod19RouteSegmentScalars, Wb11HydrologyKernelGuardError> {
        let xu = Self::require_erod18_state_scalar(request, &symbols.xu)?;
        let xl = Self::require_erod18_state_scalar(request, &symbols.xl)?;
        let ainf = Self::require_erod18_state_scalar(request, &symbols.ainf)?;
        let binf = Self::require_erod18_state_scalar(request, &symbols.binf)?;
        let cinf = Self::require_erod18_state_scalar(request, &symbols.cinf)?;
        let ainftc = Self::require_erod18_state_scalar(request, &symbols.ainftc)?;
        let binftc = Self::require_erod18_state_scalar(request, &symbols.binftc)?;
        let cinftc = Self::require_erod18_state_scalar(request, &symbols.cinftc)?;

        Self::require_erod18_domain(&symbols.xu, xu, Some(0.0), None)?;
        Self::require_erod18_domain(&symbols.xl, xl, Some(xu), None)?;

        Ok(Erod19RouteSegmentScalars {
            xu,
            xl,
            ainf,
            binf,
            cinf,
            ainftc,
            binftc,
            cinftc,
        })
    }

    fn erod19_route_theta(
        request: &HillslopeKernelRequest<'_>,
        erod13_state_updates: &[WritebackField],
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let theta_symbol = BoundarySymbol::from(EROD13_SYMBOL_THETA);
        let theta = if let Some(value) =
            Self::extract_state_update_scalar(erod13_state_updates, EROD13_SYMBOL_THETA)
        {
            value
        } else if request.state_surface.contains_key(&theta_symbol) {
            Self::require_erod18_state_scalar(request, &theta_symbol)?
        } else {
            Self::erod19_route_theta_from_erosion_inputs(request)?
        };
        Self::require_erod18_domain(&theta_symbol, theta, Some(0.0), None)?;
        Ok(theta)
    }

    fn erod19_route_theta_from_erosion_inputs(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let cntlen_symbol = BoundarySymbol::from(EROD13_SYMBOL_CNTLEN);
        let detinr_symbol = BoundarySymbol::from(EROD13_SYMBOL_DETINR);
        let tcend_symbol = BoundarySymbol::from(EROD13_SYMBOL_TCEND);
        let runoff_efficiency_symbol = BoundarySymbol::from(EROD13_SYMBOL_EFFDRR);
        let normalizing_efficiency_symbol = BoundarySymbol::from(EROD13_SYMBOL_EFFDRN);

        let cntlen = Self::require_erod18_state_scalar(request, &cntlen_symbol)?;
        let detinr = Self::require_erod18_state_scalar(request, &detinr_symbol)?;
        let tcend = Self::require_erod18_state_scalar(request, &tcend_symbol)?;
        let runoff_efficiency =
            Self::require_erod18_state_scalar(request, &runoff_efficiency_symbol)?;
        let normalizing_efficiency =
            Self::require_erod18_state_scalar(request, &normalizing_efficiency_symbol)?;

        Self::require_erod18_domain(&cntlen_symbol, cntlen, Some(WB11_ZERO_THRESHOLD), None)?;
        Self::require_erod18_domain(&detinr_symbol, detinr, Some(0.0), None)?;
        Self::require_erod18_domain(&tcend_symbol, tcend, Some(WB11_ZERO_THRESHOLD), None)?;
        Self::require_erod18_domain(
            &runoff_efficiency_symbol,
            runoff_efficiency,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;
        Self::require_erod18_domain(
            &normalizing_efficiency_symbol,
            normalizing_efficiency,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;

        Ok(((cntlen * detinr) / tcend) * (runoff_efficiency / normalizing_efficiency))
    }

    fn erod19_route_phi(
        request: &HillslopeKernelRequest<'_>,
        erod13_state_updates: &[WritebackField],
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
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
            Self::erod19_route_phi_from_route_beta(request)?
        } else {
            Self::erod19_route_phi_from_erosion_inputs(request)?
        };
        Self::require_erod18_domain(&phi_symbol, phi, Some(0.0), None)?;
        Self::require_erod18_domain(
            &phi_symbol,
            phi,
            Some(WB11_ZERO_THRESHOLD),
            Some(EROD14_MAX_PHI),
        )?;
        Ok(phi)
    }

    fn erod19_route_phi_from_route_beta(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let route_beta_symbol = BoundarySymbol::from(EROD14_SYMBOL_BETA);
        let route_beta = Self::require_erod18_state_scalar(request, &route_beta_symbol)?;
        Self::require_erod18_domain(&route_beta_symbol, route_beta, Some(0.0), None)?;
        Ok(route_beta)
    }

    fn erod19_route_phi_from_erosion_inputs(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let beta_symbol = BoundarySymbol::from(EROD13_SYMBOL_BETA);
        let veleff_symbol = BoundarySymbol::from(EROD13_SYMBOL_VELEFF);
        let pkro_symbol = BoundarySymbol::from(EROD13_SYMBOL_PKRO);

        let beta = Self::require_erod18_state_scalar(request, &beta_symbol)?;
        let veleff = Self::require_erod18_state_scalar(request, &veleff_symbol)?;
        let pkro = Self::require_erod18_state_scalar(request, &pkro_symbol)?;

        Self::require_erod18_domain(&beta_symbol, beta, Some(0.0), None)?;
        Self::require_erod18_domain(&veleff_symbol, veleff, Some(0.0), None)?;
        Self::require_erod18_domain(&pkro_symbol, pkro, Some(WB11_ZERO_THRESHOLD), None)?;

        Ok((beta * veleff) / pkro)
    }

    fn erod19_route_tauc(
        request: &HillslopeKernelRequest<'_>,
        erod13_state_updates: &[WritebackField],
        theta: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
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
            Self::erod19_route_tauc_from_shear_inputs(request)?
        } else {
            theta * EROD19_TAUC_FALLBACK_SCALE
        };
        Self::require_erod18_domain(&tauc_symbol, tauc, Some(0.0), None)?;
        Ok(tauc)
    }

    fn erod19_route_tauc_from_shear_inputs(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let tcadjf_symbol = BoundarySymbol::from(EROD13_SYMBOL_TCADJF);
        let shcrit_symbol = BoundarySymbol::from(EROD13_SYMBOL_SHCRIT);
        let shrsol_symbol = BoundarySymbol::from(EROD13_SYMBOL_SHRSOL);

        let tcadjf = Self::require_erod18_state_scalar(request, &tcadjf_symbol)?;
        let shcrit = Self::require_erod18_state_scalar(request, &shcrit_symbol)?;
        let shrsol = Self::require_erod18_state_scalar(request, &shrsol_symbol)?;

        Self::require_erod18_domain(&tcadjf_symbol, tcadjf, Some(EROD13_MIN_TCADJF), None)?;
        Self::require_erod18_domain(&shcrit_symbol, shcrit, Some(0.0), None)?;
        Self::require_erod18_domain(&shrsol_symbol, shrsol, Some(WB11_ZERO_THRESHOLD), None)?;

        Ok((tcadjf * shcrit) / shrsol)
    }

    fn erod19_route_ldlast(
        request: &HillslopeKernelRequest<'_>,
        lddend: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let g_symbol = BoundarySymbol::from(EROD13_SYMBOL_G);
        if let Some(value) = request.state_surface.get(&g_symbol) {
            let scalar = value.as_f64();
            if !scalar.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::Erod18NonFiniteSymbol {
                    symbol: g_symbol,
                    value: scalar,
                });
            }
            Self::require_erod18_domain(&g_symbol, scalar, Some(0.0), None)?;
            Ok(scalar)
        } else {
            Ok(lddend)
        }
    }

    fn erod19_route_drivers(
        request: &HillslopeKernelRequest<'_>,
        erod13_state_updates: &[WritebackField],
        xl: f64,
    ) -> Result<Erod19RouteDrivers, Wb11HydrologyKernelGuardError> {
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

        let theta = Self::erod19_route_theta(request, erod13_state_updates)?;
        let phi = Self::erod19_route_phi(request, erod13_state_updates)?;
        let tauc = Self::erod19_route_tauc(request, erod13_state_updates, theta)?;
        let ldlast = Self::erod19_route_ldlast(request, lddend)?;

        Ok(Erod19RouteDrivers {
            qostar_symbol,
            qostar,
            xdetst,
            ktrato,
            theta,
            phi,
            tauc,
            ldlast,
        })
    }

    fn erod19_initial_deposition_rate(
        segment: &Erod19RouteSegmentScalars,
        drivers: &Erod19RouteDrivers,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let dl = if drivers.qostar.abs() < EROD19_QOSTAR_NEAR_ZERO_THRESHOLD {
            (drivers.phi / (drivers.phi + 1.0))
                * ((drivers.ktrato * segment.binftc) - drivers.theta)
        } else {
            if drivers.qostar.abs() <= WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::Erod18DomainViolation {
                    symbol: drivers.qostar_symbol.clone(),
                    value: drivers.qostar,
                    minimum: Some(EROD19_QOSTAR_NEAR_ZERO_THRESHOLD),
                    maximum: None,
                });
            }
            (drivers.phi / drivers.qostar)
                * ((drivers.ktrato * segment.cinftc) - drivers.ldlast)
        };
        if !dl.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::Erod18DomainViolation {
                symbol: drivers.qostar_symbol.clone(),
                value: dl,
                minimum: None,
                maximum: None,
            });
        }
        Ok(dl)
    }

    fn erod19_transport_capacity(a: f64, b: f64, c: f64, x: f64, ktrato: f64) -> f64 {
        (((a * x * x) + (b * x) + c).max(0.0)) * ktrato
    }

    fn erod19_route_deposition_outputs(
        inputs: &Erod19RouteDepositionInputs,
    ) -> Result<Erod19RouteDepositionOutputs, Wb11HydrologyKernelGuardError> {
        if inputs.du < 0.0 {
            Self::erod19_negative_deposition_outputs(inputs)
        } else {
            Ok(Self::erod19_nonnegative_deposition_outputs(inputs))
        }
    }

    fn erod19_negative_deposition_outputs(
        inputs: &Erod19RouteDepositionInputs,
    ) -> Result<Erod19RouteDepositionOutputs, Wb11HydrologyKernelGuardError> {
        let cdep = Self::erod19_depc(
            inputs.xu,
            inputs.ainftc,
            inputs.binftc,
            inputs.phi,
            inputs.theta,
            inputs.du,
            inputs.ktrato,
            inputs.qostar,
        );
        if !cdep.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::Erod18DomainViolation {
                symbol: BoundarySymbol::from(EROD18_SYMBOL_DL),
                value: cdep,
                minimum: None,
                maximum: None,
            });
        }

        let mut xdend = Self::erod19_depend(
            inputs.xu,
            inputs.xl,
            inputs.ainftc,
            inputs.binftc,
            cdep,
            inputs.phi,
            inputs.theta,
            inputs.ktrato,
            inputs.qostar,
        );
        if !xdend.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::Erod18DomainViolation {
                symbol: BoundarySymbol::from(EROD18_SYMBOL_XDEND),
                value: xdend,
                minimum: Some(inputs.xu),
                maximum: Some(inputs.xl),
            });
        }
        xdend = xdend.clamp(inputs.xu, inputs.xl);

        Ok(Self::erod19_finish_negative_deposition(inputs, xdend))
    }

    fn erod19_finish_negative_deposition(
        inputs: &Erod19RouteDepositionInputs,
        mut xdend: f64,
    ) -> Erod19RouteDepositionOutputs {
        let mut xdbeg = 0.0;
        let mut ndep = 0.0;
        let mut ldlast_out = inputs.ldlast;
        let lddend_out;

        if xdend < inputs.xl - WB11_ZERO_THRESHOLD {
            let tc_xdend = Self::erod19_transport_capacity(
                inputs.ainftc,
                inputs.binftc,
                inputs.cinftc,
                xdend,
                inputs.ktrato,
            );
            let tc_xl = Self::erod19_transport_capacity(
                inputs.ainftc,
                inputs.binftc,
                inputs.cinftc,
                inputs.xl,
                inputs.ktrato,
            );
            if inputs.mshear > EROD18_MSHEAR_MIN + WB11_ZERO_THRESHOLD
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
            xdend = inputs.xl;
            lddend_out = ldlast_out.max(0.0);
        }

        Erod19RouteDepositionOutputs {
            xdbeg,
            xdend,
            ndep,
            lddend_out,
            ldlast_out,
            du: inputs.du,
            dl: inputs.du,
        }
    }

    fn erod19_nonnegative_deposition_outputs(
        inputs: &Erod19RouteDepositionInputs,
    ) -> Erod19RouteDepositionOutputs {
        let mut xdbeg = 0.0;
        let xdend = inputs.xl;
        let mut ndep = 0.0;
        let mut ldlast_out = inputs.ldlast;
        let lddend_out;

        let tc_upper = (inputs.ktrato * inputs.cinftc).max(0.0);
        let tc_xl = Self::erod19_transport_capacity(
            inputs.ainftc,
            inputs.binftc,
            inputs.cinftc,
            inputs.xl,
            inputs.ktrato,
        );
        if ldlast_out > tc_upper + WB11_ZERO_THRESHOLD {
            ndep = 1.0;
            xdbeg = inputs.xu;
            ldlast_out = ldlast_out.min(tc_xl).max(0.0);
            lddend_out = ldlast_out;
        } else {
            lddend_out = ldlast_out.max(0.0);
        }

        Erod19RouteDepositionOutputs {
            xdbeg,
            xdend,
            ndep,
            lddend_out,
            ldlast_out,
            du: 0.0,
            dl: 0.0,
        }
    }

    fn erod19_route_writebacks(
        topology: &Erod19RouteTopology,
        symbols: &Erod19RouteSegmentSymbols,
        segment: &Erod19RouteSegmentScalars,
        drivers: &Erod19RouteDrivers,
        deposition: &Erod19RouteDepositionOutputs,
        classification: &Erod19RouteClassification,
    ) -> Vec<WritebackField> {
        vec![
            WritebackField::bounded(
                topology.nslpts_symbol.clone(),
                topology.nslpts_value,
                Some(topology.min_segment_value),
                None,
            ),
            WritebackField::bounded(symbols.xu.clone(), segment.xu, Some(0.0), None),
            WritebackField::bounded(symbols.xl.clone(), segment.xl, Some(segment.xu), None),
            WritebackField::unbounded(symbols.ainf.clone(), segment.ainf),
            WritebackField::unbounded(symbols.binf.clone(), segment.binf),
            WritebackField::unbounded(symbols.cinf.clone(), segment.cinf),
            WritebackField::unbounded(symbols.ainftc.clone(), segment.ainftc),
            WritebackField::unbounded(symbols.binftc.clone(), segment.binftc),
            WritebackField::unbounded(symbols.cinftc.clone(), segment.cinftc),
            WritebackField::unbounded(EROD18_SYMBOL_QOSTAR, drivers.qostar),
            WritebackField::bounded(EROD18_SYMBOL_XDBEG, deposition.xdbeg, Some(0.0), None),
            WritebackField::bounded(
                EROD18_SYMBOL_XDEND,
                deposition.xdend,
                Some(segment.xu),
                Some(segment.xl),
            ),
            WritebackField::bounded(
                EROD18_SYMBOL_XDETST,
                drivers.xdetst,
                Some(0.0),
                Some(segment.xl),
            ),
            WritebackField::bounded(EROD18_SYMBOL_LDLAST, deposition.ldlast_out, Some(0.0), None),
            WritebackField::bounded(EROD18_SYMBOL_LDDEND, deposition.lddend_out, Some(0.0), None),
            WritebackField::unbounded(EROD18_SYMBOL_DU, deposition.du),
            WritebackField::unbounded(EROD18_SYMBOL_DL, deposition.dl),
            WritebackField::bounded(EROD18_SYMBOL_NDEP, deposition.ndep, Some(0.0), Some(1.0)),
            WritebackField::bounded(
                EROD18_SYMBOL_MSHEAR,
                classification.mshear,
                Some(EROD18_MSHEAR_MIN),
                Some(EROD18_MSHEAR_MAX),
            ),
            WritebackField::bounded(
                EROD18_SYMBOL_XC1,
                classification.xc1,
                Some(segment.xu),
                Some(segment.xl),
            ),
            WritebackField::bounded(
                EROD18_SYMBOL_XC2,
                classification.xc2,
                Some(segment.xu),
                Some(segment.xl),
            ),
        ]
    }

pub(crate) fn run_erod19_route_segment_migration(
        request: &HillslopeKernelRequest<'_>,
        erod13_state_updates: &[WritebackField],
    ) -> Result<Vec<WritebackField>, Wb11HydrologyKernelGuardError> {
        if !Self::resolve_erod14_wave2_enabled(request)? {
            return Ok(Vec::new());
        }

        let topology = Self::erod19_route_topology(request)?;
        let symbols = Erod19RouteSegmentSymbols::for_segment(topology.segment_index);
        let segment = Self::erod19_route_segment_scalars(request, &symbols)?;

        let drivers = Self::erod19_route_drivers(request, erod13_state_updates, segment.xl)?;

        let du = Self::erod19_initial_deposition_rate(&segment, &drivers)?;

        let (mshear, xc1, xc2) = Self::erod19_xcrit_classification(
            segment.ainf,
            segment.binf,
            segment.cinf,
            drivers.tauc,
            segment.xu,
            segment.xl,
        );
        let classification = Erod19RouteClassification { mshear, xc1, xc2 };
        let deposition = Self::erod19_route_deposition_outputs(
            &Erod19RouteDepositionInputs {
                xu: segment.xu,
                xl: segment.xl,
                ainftc: segment.ainftc,
                binftc: segment.binftc,
                cinftc: segment.cinftc,
                phi: drivers.phi,
                theta: drivers.theta,
                ktrato: drivers.ktrato,
                qostar: drivers.qostar,
                ldlast: drivers.ldlast,
                mshear: classification.mshear,
                du,
            },
        )?;

        Self::require_erod18_domain(
            &BoundarySymbol::from(EROD18_SYMBOL_MSHEAR),
            classification.mshear,
            Some(EROD18_MSHEAR_MIN),
            Some(EROD18_MSHEAR_MAX),
        )?;

        Ok(Self::erod19_route_writebacks(
            &topology,
            &symbols,
            &segment,
            &drivers,
            &deposition,
            &classification,
        ))
    }

}
