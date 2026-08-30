#[allow(clippy::wildcard_imports)]
use super::super::*;
use openwepp_unit_boundary::ProcessRateMillimetersPerHour;

/// Per-layer operands for the source-intent disturbed-soil `ksatadj`
/// effective-conductivity model.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectKsatadjLayerInputs {
    pub theta_m: f64,
    pub field_capacity_m: f64,
    pub upper_limit_m: f64,
    pub depth_m: f64,
    pub porosity: f64,
    pub cpm: f64,
    pub field_capacity_theta: f64,
    pub residual_theta: f64,
}

/// Typed direct-runtime input surface for `INV-SUBHYD-032`.
#[derive(Debug, Clone, PartialEq)]
pub struct DirectKsatadjEffectiveConductivityInputs {
    pub ksatadj: bool,
    pub solwpv: f64,
    pub soil_conductivity_m_s: f64,
    pub ksatfac_mm_h: Option<f64>,
    pub ksatrec_per_day: Option<f64>,
    pub lkeff_mm_h: Option<f64>,
    pub layers: Vec<DirectKsatadjLayerInputs>,
}

/// Conductivity and lineage diagnostics from the source-intent `ksatadj`
/// effective-conductivity model.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectKsatadjEffectiveConductivityOutcome {
    pub effective_conductivity_m_s: f64,
    pub effective_conductivity_mm_h: f64,
    pub sat_frac: f64,
    pub avthetafc: f64,
    pub avthetadr: f64,
}

#[derive(Debug, Default)]
struct DirectKsatadjMetricSums {
    theta_storage: f64,
    porosity_depth: f64,
    cpm_depth: f64,
    field_capacity_theta_depth: f64,
    residual_theta_depth: f64,
    tillage_depth: f64,
}

#[derive(Debug, Clone, Copy)]
struct DirectKsatadjMetrics {
    sat_frac: f64,
    avthetafc: f64,
    avthetadr: f64,
}

impl Wb11HydrologyKernel {
    pub fn compute_direct_ksatadj_effective_conductivity(
        inputs: &DirectKsatadjEffectiveConductivityInputs,
    ) -> Result<Option<DirectKsatadjEffectiveConductivityOutcome>, Wb11HydrologyKernelGuardError>
    {
        let phase_class = HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;
        if !inputs.ksatadj {
            return Ok(None);
        }

        let solwpv = Self::direct_ksatadj_solwpv_mode(phase_class, inputs.solwpv)?;
        let metrics = Self::direct_ksatadj_top_two_metrics(phase_class, &inputs.layers)?;
        let upper_ks_mm_h = Self::direct_ksatadj_m_s_to_mm_h(
            phase_class,
            BoundarySymbol::from(WB14_SYMBOL_SOIL_CONDUCTIVITY),
            inputs.soil_conductivity_m_s,
        )?;
        let effective_conductivity_mm_h = Self::direct_ksatadj_effective_mm_h(
            phase_class,
            inputs,
            solwpv,
            upper_ks_mm_h,
            metrics,
        )?;
        let effective_conductivity_m_s = Self::direct_ksatadj_mm_h_to_m_s(
            phase_class,
            BoundarySymbol::from("Keff_ksatadj"),
            effective_conductivity_mm_h,
        )?;

        Ok(Some(DirectKsatadjEffectiveConductivityOutcome {
            effective_conductivity_m_s,
            effective_conductivity_mm_h,
            sat_frac: metrics.sat_frac,
            avthetafc: metrics.avthetafc,
            avthetadr: metrics.avthetadr,
        }))
    }

    fn direct_ksatadj_top_two_metrics(
        phase_class: HillslopeKernelPhaseClass,
        layers: &[DirectKsatadjLayerInputs],
    ) -> Result<DirectKsatadjMetrics, Wb11HydrologyKernelGuardError> {
        let mut sums = DirectKsatadjMetricSums::default();
        for layer_index in 1..=2 {
            let layer = layers.get(layer_index - 1).ok_or_else(|| {
                Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                    phase_class,
                    symbol: Self::wb18_perc_state_symbol("theta", layer_index),
                }
            })?;
            Self::direct_ksatadj_validate_layer(phase_class, layer_index, layer)?;
            sums.theta_storage += layer.theta_m;
            sums.porosity_depth += layer.porosity * layer.depth_m;
            sums.cpm_depth += layer.cpm * layer.depth_m;
            sums.field_capacity_theta_depth += layer.field_capacity_theta * layer.depth_m;
            sums.residual_theta_depth += layer.residual_theta * layer.depth_m;
            sums.tillage_depth += layer.depth_m;
        }
        Self::direct_ksatadj_finalize_metrics(phase_class, &sums)
    }

    fn direct_ksatadj_validate_layer(
        phase_class: HillslopeKernelPhaseClass,
        layer_index: usize,
        layer: &DirectKsatadjLayerInputs,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        Self::direct_ksatadj_require_value(
            phase_class,
            Self::wb18_perc_state_symbol("theta", layer_index),
            layer.theta_m,
            Some(0.0),
            None,
        )?;
        Self::direct_ksatadj_require_value(
            phase_class,
            Self::wb18_perc_state_symbol("fc", layer_index),
            layer.field_capacity_m,
            Some(0.0),
            None,
        )?;
        Self::direct_ksatadj_require_value(
            phase_class,
            Self::wb18_perc_state_symbol("ul", layer_index),
            layer.upper_limit_m,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;
        Self::direct_ksatadj_require_value(
            phase_class,
            Self::wb19_dg_symbol(layer_index),
            layer.depth_m,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;
        Self::direct_ksatadj_require_value(
            phase_class,
            BoundarySymbol::from(format!("wb19_por_{layer_index:04}")),
            layer.porosity,
            Some(WB11_ZERO_THRESHOLD),
            Some(1.0),
        )?;
        Self::direct_ksatadj_require_value(
            phase_class,
            BoundarySymbol::from(format!("cpm_{layer_index:04}")),
            layer.cpm,
            Some(WB11_ZERO_THRESHOLD),
            Some(1.0),
        )?;
        Self::direct_ksatadj_require_value(
            phase_class,
            BoundarySymbol::from(format!("wb19_thetfc_{layer_index:04}")),
            layer.field_capacity_theta,
            Some(WB11_ZERO_THRESHOLD),
            Some(1.0),
        )?;
        Self::direct_ksatadj_require_value(
            phase_class,
            Self::wb19_thetdr_symbol(layer_index),
            layer.residual_theta,
            Some(WB11_ZERO_THRESHOLD),
            Some(1.0),
        )?;
        if layer.field_capacity_theta <= layer.residual_theta + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(format!("wb19_thetfc_{layer_index:04}")),
                value: layer.field_capacity_theta,
                minimum: Some(layer.residual_theta + WB11_ZERO_THRESHOLD),
                maximum: Some(1.0),
            });
        }
        Ok(())
    }

    fn direct_ksatadj_finalize_metrics(
        phase_class: HillslopeKernelPhaseClass,
        sums: &DirectKsatadjMetricSums,
    ) -> Result<DirectKsatadjMetrics, Wb11HydrologyKernelGuardError> {
        Self::direct_ksatadj_validate_sums(phase_class, sums)?;
        let avthetafc = sums.field_capacity_theta_depth / sums.tillage_depth;
        let avthetadr = sums.residual_theta_depth / sums.tillage_depth;
        Self::direct_ksatadj_require_value(
            phase_class,
            BoundarySymbol::from("avthetafc"),
            avthetafc,
            Some(WB11_ZERO_THRESHOLD),
            Some(1.0),
        )?;
        Self::direct_ksatadj_require_value(
            phase_class,
            BoundarySymbol::from("avthetadr"),
            avthetadr,
            Some(WB11_ZERO_THRESHOLD),
            Some(1.0),
        )?;
        if avthetafc <= avthetadr + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("avthetafc"),
                value: avthetafc,
                minimum: Some(avthetadr + WB11_ZERO_THRESHOLD),
                maximum: Some(1.0),
            });
        }
        let sat_frac = Self::direct_ksatadj_saturation_fraction(phase_class, sums)?;
        Ok(DirectKsatadjMetrics {
            sat_frac,
            avthetafc,
            avthetadr,
        })
    }

    fn direct_ksatadj_validate_sums(
        phase_class: HillslopeKernelPhaseClass,
        sums: &DirectKsatadjMetricSums,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        Self::direct_ksatadj_require_value(
            phase_class,
            BoundarySymbol::from("dg_agg_0001_0002"),
            sums.tillage_depth,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;
        Self::direct_ksatadj_require_value(
            phase_class,
            BoundarySymbol::from("avpor"),
            sums.porosity_depth / sums.tillage_depth,
            Some(WB11_ZERO_THRESHOLD),
            Some(1.0),
        )?;
        Self::direct_ksatadj_require_value(
            phase_class,
            BoundarySymbol::from("avcpm"),
            sums.cpm_depth / sums.tillage_depth,
            Some(WB11_ZERO_THRESHOLD),
            Some(1.0),
        )?;
        Ok(())
    }

    fn direct_ksatadj_saturation_fraction(
        phase_class: HillslopeKernelPhaseClass,
        sums: &DirectKsatadjMetricSums,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let avpor = sums.porosity_depth / sums.tillage_depth;
        let avcpm = sums.cpm_depth / sums.tillage_depth;
        let avsm15 = sums.residual_theta_depth / sums.tillage_depth;
        let denominator = avpor * avcpm;
        Self::direct_ksatadj_require_value(
            phase_class,
            BoundarySymbol::from("avpor_avcpm"),
            denominator,
            Some(WB11_ZERO_THRESHOLD),
            Some(1.0),
        )?;

        let mut avsat = (sums.theta_storage / sums.tillage_depth) + avsm15;
        if avsat > avpor {
            avsat = avpor * 0.98;
        }
        if avsat >= denominator {
            avsat = denominator * 0.99;
        }
        let sat_frac = avsat / denominator;
        Self::direct_ksatadj_require_value(
            phase_class,
            BoundarySymbol::from("sat_frac"),
            sat_frac,
            Some(0.0),
            Some(1.0),
        )?;
        Ok(sat_frac.min(1.0))
    }

    fn direct_ksatadj_solwpv_mode(
        phase_class: HillslopeKernelPhaseClass,
        solwpv: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        Self::direct_ksatadj_require_value(
            phase_class,
            BoundarySymbol::from("solwpv"),
            solwpv,
            Some(0.0),
            None,
        )?;
        let rounded = solwpv.round();
        if (solwpv - rounded).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("solwpv"),
                value: solwpv,
                minimum: Some(rounded),
                maximum: Some(rounded),
            });
        }
        Ok(rounded)
    }

    fn direct_ksatadj_effective_mm_h(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectKsatadjEffectiveConductivityInputs,
        solwpv: f64,
        upper_ks_mm_h: f64,
        metrics: DirectKsatadjMetrics,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        if (solwpv - 9001.0).abs() <= WB11_ZERO_THRESHOLD {
            return Self::direct_ksatadj_effective_9001_mm_h(
                phase_class,
                inputs,
                upper_ks_mm_h,
                metrics.sat_frac,
            );
        }
        if solwpv >= 9002.0 - WB11_ZERO_THRESHOLD {
            return Self::direct_ksatadj_effective_9002_plus_mm_h(
                phase_class,
                inputs,
                solwpv,
                upper_ks_mm_h,
                metrics,
            );
        }
        Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
            phase_class,
            symbol: BoundarySymbol::from("solwpv"),
            value: solwpv,
            minimum: Some(9001.0),
            maximum: None,
        })
    }

    fn direct_ksatadj_effective_9001_mm_h(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectKsatadjEffectiveConductivityInputs,
        upper_ks_mm_h: f64,
        sat_frac: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let ksatfac = Self::direct_ksatadj_required_positive_option(
            phase_class,
            BoundarySymbol::from("ksatfac"),
            inputs.ksatfac_mm_h,
        )?;
        let ksatrec = Self::direct_ksatadj_required_positive_option(
            phase_class,
            BoundarySymbol::from("ksatrec"),
            inputs.ksatrec_per_day,
        )?;
        let lower_ks_mm_h = upper_ks_mm_h / ksatfac;
        let denominator = (1.0 / ksatrec).exp() - 1.0;
        Self::direct_ksatadj_require_value(
            phase_class,
            BoundarySymbol::from("ksatrec"),
            denominator,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;
        Ok(
            ((upper_ks_mm_h - lower_ks_mm_h) / denominator) * ((sat_frac / ksatrec).exp() - 1.0)
                + lower_ks_mm_h,
        )
    }

    fn direct_ksatadj_effective_9002_plus_mm_h(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectKsatadjEffectiveConductivityInputs,
        solwpv: f64,
        upper_ks_mm_h: f64,
        metrics: DirectKsatadjMetrics,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let exponent =
            Self::direct_ksatadj_9002_exponent(phase_class, metrics.avthetafc, metrics.avthetadr)?;
        let mut effective_ks_mm_h = upper_ks_mm_h * metrics.sat_frac.powf(exponent);
        if (solwpv - 9003.0).abs() <= WB11_ZERO_THRESHOLD {
            let lkeff = Self::direct_ksatadj_required_finite_option(
                phase_class,
                BoundarySymbol::from("lkeff"),
                inputs.lkeff_mm_h,
            )?;
            if lkeff > 0.0 && effective_ks_mm_h < lkeff {
                effective_ks_mm_h = lkeff;
            }
        }
        Ok(effective_ks_mm_h)
    }

    fn direct_ksatadj_9002_exponent(
        phase_class: HillslopeKernelPhaseClass,
        avthetafc: f64,
        avthetadr: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let psi_denominator = avthetafc.ln() - avthetadr.ln();
        Self::direct_ksatadj_require_value(
            phase_class,
            BoundarySymbol::from("avthetafc_avthetadr"),
            psi_denominator,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;
        let psi = (1500.0_f64.ln() - 33.0_f64.ln()) / psi_denominator;
        Self::direct_ksatadj_require_value(
            phase_class,
            BoundarySymbol::from("psi"),
            psi,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;
        let lambda = 1.0 / psi;
        Self::direct_ksatadj_require_value(
            phase_class,
            BoundarySymbol::from("lambda"),
            lambda,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;
        let exponent = (2.0 * lambda) + 3.0;
        Self::direct_ksatadj_require_value(
            phase_class,
            BoundarySymbol::from("ksatadj_exponent"),
            exponent,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;
        Ok(exponent)
    }

    fn direct_ksatadj_required_positive_option(
        phase_class: HillslopeKernelPhaseClass,
        symbol: BoundarySymbol,
        value: Option<f64>,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let value =
            Self::direct_ksatadj_required_finite_option(phase_class, symbol.clone(), value)?;
        Self::direct_ksatadj_require_value(
            phase_class,
            symbol,
            value,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;
        Ok(value)
    }

    fn direct_ksatadj_required_finite_option(
        phase_class: HillslopeKernelPhaseClass,
        symbol: BoundarySymbol,
        value: Option<f64>,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let value =
            value.ok_or_else(
                || Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                    phase_class,
                    symbol: symbol.clone(),
                },
            )?;
        if !value.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol,
                value,
            });
        }
        Ok(value)
    }

    fn direct_ksatadj_m_s_to_mm_h(
        phase_class: HillslopeKernelPhaseClass,
        symbol: BoundarySymbol,
        value_m_s: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        ProcessRateMillimetersPerHour::from_meters_per_second(value_m_s)
            .map_err(|error| Self::unit_conversion_guard_error(phase_class, symbol, &error))
            .map(ProcessRateMillimetersPerHour::as_millimeters_per_hour)
    }

    fn direct_ksatadj_mm_h_to_m_s(
        phase_class: HillslopeKernelPhaseClass,
        symbol: BoundarySymbol,
        value_mm_h: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        ProcessRateMillimetersPerHour::try_new(value_mm_h)
            .map_err(|error| Self::unit_conversion_guard_error(phase_class, symbol, &error))
            .map(ProcessRateMillimetersPerHour::as_meters_per_second)
    }

    fn direct_ksatadj_require_value(
        phase_class: HillslopeKernelPhaseClass,
        symbol: BoundarySymbol,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if !value.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol,
                value,
            });
        }
        Self::require_state_range_with(phase_class, || symbol.clone(), value, minimum, maximum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(observed: f64, expected: f64) {
        assert!(
            (observed - expected).abs() <= 1.0e-12,
            "observed {observed}, expected {expected}"
        );
    }

    fn source_intent_layers() -> Vec<DirectKsatadjLayerInputs> {
        vec![
            DirectKsatadjLayerInputs {
                theta_m: 0.02,
                field_capacity_m: 0.03,
                upper_limit_m: 0.20,
                depth_m: 0.10,
                porosity: 0.55,
                cpm: 1.0,
                field_capacity_theta: 0.40,
                residual_theta: 0.10,
            },
            DirectKsatadjLayerInputs {
                theta_m: 0.04,
                field_capacity_m: 0.033,
                upper_limit_m: 0.20,
                depth_m: 0.10,
                porosity: 0.55,
                cpm: 1.0,
                field_capacity_theta: 0.45,
                residual_theta: 0.12,
            },
        ]
    }

    fn source_intent_inputs(solwpv: f64) -> DirectKsatadjEffectiveConductivityInputs {
        DirectKsatadjEffectiveConductivityInputs {
            ksatadj: true,
            solwpv,
            soil_conductivity_m_s: 2.5e-6,
            ksatfac_mm_h: Some(2.5),
            ksatrec_per_day: Some(0.75),
            lkeff_mm_h: Some(0.1),
            layers: source_intent_layers(),
        }
    }

    #[test]
    fn direct_ksatadj_inactive_returns_no_adjustment() {
        let mut inputs = source_intent_inputs(9002.0);
        inputs.ksatadj = false;

        let outcome = Wb11HydrologyKernel::compute_direct_ksatadj_effective_conductivity(&inputs)
            .expect("inactive ksatadj should not fail");

        assert!(outcome.is_none());
    }

    #[test]
    fn direct_ksatadj_sat_frac_uses_source_intent_avsat_not_ul_surrogate() {
        let inputs = source_intent_inputs(9002.0);
        let outcome = Wb11HydrologyKernel::compute_direct_ksatadj_effective_conductivity(&inputs)
            .expect("source-intent operands should evaluate")
            .expect("active ksatadj should return an outcome");

        let expected_source_intent_sat_frac = 0.41 / 0.55;
        let storage_over_upper_limit_surrogate = 0.06 / 0.40;
        assert_close(outcome.sat_frac, expected_source_intent_sat_frac);
        assert!(
            (outcome.sat_frac - storage_over_upper_limit_surrogate).abs() > 1.0e-3,
            "non-aliased vector must reject the theta_sum/ul_sum surrogate"
        );
        assert_close(outcome.avthetafc, 0.425);
        assert_close(outcome.avthetadr, 0.11);
    }

    #[test]
    fn direct_ksatadj_9001_branch_uses_exponential_recovery_formula() {
        let inputs = source_intent_inputs(9001.0);
        let outcome = Wb11HydrologyKernel::compute_direct_ksatadj_effective_conductivity(&inputs)
            .expect("9001 ksatadj should evaluate")
            .expect("active ksatadj should return an outcome");
        let upper_ks_mm_h = 2.5e-6 * 3.6e6;
        let lower_ks_mm_h = upper_ks_mm_h / 2.5;
        let sat_frac = 0.41 / 0.55;
        let denominator = (1.0_f64 / 0.75_f64).exp() - 1.0;
        let expected_mm_h = ((upper_ks_mm_h - lower_ks_mm_h) / denominator)
            * ((sat_frac / 0.75_f64).exp() - 1.0)
            + lower_ks_mm_h;

        assert_close(outcome.effective_conductivity_mm_h, expected_mm_h);
        assert_close(outcome.effective_conductivity_m_s, expected_mm_h / 3.6e6);
    }

    #[test]
    fn direct_ksatadj_9003_branch_applies_lkeff_floor() {
        let mut inputs = source_intent_inputs(9003.0);
        inputs.soil_conductivity_m_s = 1.0e-8;
        inputs.lkeff_mm_h = Some(0.1);

        let outcome = Wb11HydrologyKernel::compute_direct_ksatadj_effective_conductivity(&inputs)
            .expect("9003 ksatadj should evaluate")
            .expect("active ksatadj should return an outcome");

        assert_close(outcome.effective_conductivity_mm_h, 0.1);
        assert_close(outcome.effective_conductivity_m_s, 0.1 / 3.6e6);
    }

    #[test]
    fn direct_ksatadj_saturated_storage_caps_avsat_instead_of_rejecting_ul_excess() {
        let mut inputs = source_intent_inputs(9002.0);
        inputs.layers[0].theta_m = 0.09;
        inputs.layers[0].upper_limit_m = 0.05;

        let outcome = Wb11HydrologyKernel::compute_direct_ksatadj_effective_conductivity(&inputs)
            .expect("source-intent saturated storage should cap avsat")
            .expect("active ksatadj should return an outcome");

        assert_close(outcome.sat_frac, 0.98);
    }

    #[test]
    fn direct_ksatadj_9001_missing_recovery_operand_is_typed_failure() {
        let mut inputs = source_intent_inputs(9001.0);
        inputs.ksatfac_mm_h = None;

        let error = Wb11HydrologyKernel::compute_direct_ksatadj_effective_conductivity(&inputs)
            .expect_err("9001 ksatadj must require ksatfac");

        assert_eq!(
            error,
            Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                phase_class: HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                symbol: BoundarySymbol::from("ksatfac"),
            }
        );
    }

    #[test]
    fn direct_ksatadj_missing_second_layer_is_typed_failure() {
        let mut inputs = source_intent_inputs(9002.0);
        inputs.layers.pop();

        let error = Wb11HydrologyKernel::compute_direct_ksatadj_effective_conductivity(&inputs)
            .expect_err("active ksatadj must require top two tillage layers");

        assert_eq!(
            error,
            Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                phase_class: HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                symbol: BoundarySymbol::from("wb18_perc_theta_0002"),
            }
        );
    }
}
