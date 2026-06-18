impl Wb11HydrologyKernel {
    pub(crate) fn wb14_ksatadj_flag(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<bool, Wb11HydrologyKernelGuardError> {
        let symbol = BoundarySymbol::from("ksatadj");
        let Some(value) = Self::optional_state_scalar_for_symbol(request, phase_class, &symbol)?
        else {
            return Ok(false);
        };
        if value.abs() <= WB11_ZERO_THRESHOLD {
            return Ok(false);
        }
        if (value - 1.0).abs() <= WB11_ZERO_THRESHOLD {
            return Ok(true);
        }
        Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
            phase_class,
            symbol,
            value,
            minimum: Some(0.0),
            maximum: Some(1.0),
        })
    }

    pub(crate) fn wb14_load_top_two_layer_ksatadj_metrics(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<(f64, f64, f64), Wb11HydrologyKernelGuardError> {
        let mut sums = Wb14KsatadjMetricSums::default();
        for layer_index in 1..=2 {
            let layer = Self::wb14_load_ksatadj_layer(request, phase_class, layer_index)?;
            Self::wb14_accumulate_ksatadj_layer(phase_class, &mut sums, &layer)?;
        }
        Self::wb14_finalize_ksatadj_metrics(phase_class, &sums)
    }

    fn wb14_load_ksatadj_layer(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        layer_index: usize,
    ) -> Result<Wb14KsatadjLayerMetrics, Wb11HydrologyKernelGuardError> {
        let theta_symbol = Self::wb18_perc_state_symbol("theta", layer_index);
        let fc_symbol = Self::wb18_perc_state_symbol("fc", layer_index);
        let ul_symbol = Self::wb18_perc_state_symbol("ul", layer_index);
        let (dg_symbol, dg) = Self::require_wb19_dg_scalar(request, phase_class, layer_index)?;
        let (por_symbol, por) = Self::require_wb19_por_scalar(request, phase_class, layer_index)?;
        let (cpm_symbol, cpm) = Self::require_wb19_cpm_scalar(request, phase_class, layer_index)?;
        let (thetfc_symbol, thetfc) =
            Self::require_wb19_thetfc_scalar(request, phase_class, layer_index)?;
        let (thetdr_symbol, thetdr) =
            Self::require_wb19_thetdr_scalar(request, phase_class, layer_index)?;
        let theta = Self::require_state_scalar_for_symbol(request, phase_class, &theta_symbol)?;
        let fc = Self::require_state_scalar_for_symbol(request, phase_class, &fc_symbol)?;
        let ul = Self::require_state_scalar_for_symbol(request, phase_class, &ul_symbol)?;

        Ok(Wb14KsatadjLayerMetrics {
            theta_symbol,
            fc_symbol,
            ul_symbol,
            dg_symbol,
            por_symbol,
            cpm_symbol,
            thetfc_symbol,
            thetdr_symbol,
            theta,
            fc,
            ul,
            dg,
            por,
            cpm,
            thetfc,
            thetdr,
        })
    }

    fn wb14_accumulate_ksatadj_layer(
        phase_class: HillslopeKernelPhaseClass,
        sums: &mut Wb14KsatadjMetricSums,
        layer: &Wb14KsatadjLayerMetrics,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        Self::wb14_validate_ksatadj_layer(phase_class, layer)?;
        sums.theta_storage += layer.theta;
        sums.por_depth += layer.por * layer.dg;
        sums.cpm_depth += layer.cpm * layer.dg;
        sums.thetfc_depth += layer.thetfc * layer.dg;
        sums.thetdr_depth += layer.thetdr * layer.dg;
        sums.tillage_depth += layer.dg;
        Ok(())
    }

    fn wb14_validate_ksatadj_layer(
        phase_class: HillslopeKernelPhaseClass,
        layer: &Wb14KsatadjLayerMetrics,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        Self::wb14_validate_ksatadj_storage_layer_terms(phase_class, layer)?;
        Self::wb14_validate_ksatadj_source_intent_layer_terms(phase_class, layer)?;
        Self::wb14_validate_ksatadj_layer_ordering(phase_class, layer)
    }

    fn wb14_validate_ksatadj_storage_layer_terms(
        phase_class: HillslopeKernelPhaseClass,
        layer: &Wb14KsatadjLayerMetrics,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        Self::require_wb14_ksatadj_minimum(phase_class, layer.theta_symbol.clone(), layer.theta, 0.0)?;
        Self::require_wb14_ksatadj_minimum(phase_class, layer.fc_symbol.clone(), layer.fc, 0.0)?;
        Self::require_wb14_ksatadj_minimum(
            phase_class,
            layer.ul_symbol.clone(),
            layer.ul,
            WB11_ZERO_THRESHOLD,
        )?;
        Self::require_wb14_ksatadj_minimum(
            phase_class,
            layer.dg_symbol.clone(),
            layer.dg,
            WB11_ZERO_THRESHOLD,
        )?;
        Ok(())
    }

    fn wb14_validate_ksatadj_source_intent_layer_terms(
        phase_class: HillslopeKernelPhaseClass,
        layer: &Wb14KsatadjLayerMetrics,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        Self::require_wb14_ksatadj_minimum(
            phase_class,
            layer.por_symbol.clone(),
            layer.por,
            WB11_ZERO_THRESHOLD,
        )?;
        Self::require_wb14_ksatadj_minimum(
            phase_class,
            layer.cpm_symbol.clone(),
            layer.cpm,
            WB11_ZERO_THRESHOLD,
        )?;
        Self::require_wb14_ksatadj_minimum(
            phase_class,
            layer.thetfc_symbol.clone(),
            layer.thetfc,
            WB11_ZERO_THRESHOLD,
        )?;
        Self::require_wb14_ksatadj_minimum(
            phase_class,
            layer.thetdr_symbol.clone(),
            layer.thetdr,
            WB11_ZERO_THRESHOLD,
        )?;
        if layer.por > 1.0 + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: layer.por_symbol.clone(),
                value: layer.por,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: Some(1.0),
            });
        }
        if layer.cpm > 1.0 + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: layer.cpm_symbol.clone(),
                value: layer.cpm,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: Some(1.0),
            });
        }
        if layer.thetfc > 1.0 + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: layer.thetfc_symbol.clone(),
                value: layer.thetfc,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: Some(1.0),
            });
        }
        if layer.thetdr > 1.0 + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: layer.thetdr_symbol.clone(),
                value: layer.thetdr,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: Some(1.0),
            });
        }
        Ok(())
    }

    fn wb14_validate_ksatadj_layer_ordering(
        phase_class: HillslopeKernelPhaseClass,
        layer: &Wb14KsatadjLayerMetrics,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if layer.thetfc <= layer.thetdr + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: layer.thetfc_symbol.clone(),
                value: layer.thetfc,
                minimum: Some(layer.thetdr + WB11_ZERO_THRESHOLD),
                maximum: Some(1.0),
            });
        }
        if layer.fc > layer.ul + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: layer.fc_symbol.clone(),
                value: layer.fc,
                minimum: Some(0.0),
                maximum: Some(layer.ul),
            });
        }
        if layer.theta > layer.ul + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: layer.theta_symbol.clone(),
                value: layer.theta,
                minimum: Some(0.0),
                maximum: Some(layer.ul),
            });
        }
        Ok(())
    }

    fn require_wb14_ksatadj_minimum(
        phase_class: HillslopeKernelPhaseClass,
        symbol: BoundarySymbol,
        value: f64,
        minimum: f64,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        let violated = if minimum <= WB11_ZERO_THRESHOLD {
            value < -WB11_ZERO_THRESHOLD
        } else {
            value <= WB11_ZERO_THRESHOLD
        };
        if violated {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol,
                value,
                minimum: Some(minimum),
                maximum: None,
            });
        }
        Ok(())
    }

    fn wb14_finalize_ksatadj_metrics(
        phase_class: HillslopeKernelPhaseClass,
        sums: &Wb14KsatadjMetricSums,
    ) -> Result<(f64, f64, f64), Wb11HydrologyKernelGuardError> {
        Self::wb14_validate_ksatadj_sums(phase_class, sums)?;
        let sat_frac = Self::wb14_ksatadj_saturation_fraction(phase_class, sums)?;
        let (avthetafc, avthetadr) = Self::wb14_ksatadj_theta_averages(sums);
        Self::wb14_validate_ksatadj_theta_averages(phase_class, avthetafc, avthetadr)?;

        Ok((sat_frac, avthetafc, avthetadr))
    }

    fn wb14_validate_ksatadj_sums(
        phase_class: HillslopeKernelPhaseClass,
        sums: &Wb14KsatadjMetricSums,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if sums.tillage_depth <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("dg_agg_0001_0002"),
                value: sums.tillage_depth,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }
        if sums.por_depth <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("avpor"),
                value: sums.por_depth / sums.tillage_depth,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }
        if sums.cpm_depth <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("avcpm"),
                value: sums.cpm_depth / sums.tillage_depth,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }
        Ok(())
    }

    fn wb14_ksatadj_saturation_fraction(
        phase_class: HillslopeKernelPhaseClass,
        sums: &Wb14KsatadjMetricSums,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let avpor = sums.por_depth / sums.tillage_depth;
        let avcpm = sums.cpm_depth / sums.tillage_depth;
        let avsm15 = sums.thetdr_depth / sums.tillage_depth;
        let denominator = avpor * avcpm;
        if !denominator.is_finite() || denominator <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("avpor_avcpm"),
                value: denominator,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: Some(1.0),
            });
        }

        let mut avsat = (sums.theta_storage / sums.tillage_depth) + avsm15;
        if avsat > avpor {
            avsat = avpor * 0.98;
        }
        if avsat >= denominator {
            avsat = denominator * 0.99;
        }

        let sat_frac = avsat / denominator;
        if !sat_frac.is_finite() || sat_frac < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("sat_frac"),
                value: sat_frac,
                minimum: Some(0.0),
                maximum: Some(1.0),
            });
        }
        Ok(sat_frac.clamp(0.0, 1.0))
    }

    fn wb14_ksatadj_theta_averages(sums: &Wb14KsatadjMetricSums) -> (f64, f64) {
        (
            sums.thetfc_depth / sums.tillage_depth,
            sums.thetdr_depth / sums.tillage_depth,
        )
    }

    fn wb14_validate_ksatadj_theta_averages(
        phase_class: HillslopeKernelPhaseClass,
        avthetafc: f64,
        avthetadr: f64,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if avthetafc <= WB11_ZERO_THRESHOLD || avthetadr <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("avthetafc_avthetadr"),
                value: avthetafc.min(avthetadr),
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }
        if avthetafc <= avthetadr + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("avthetafc"),
                value: avthetafc,
                minimum: Some(avthetadr + WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }
        Ok(())
    }

    pub(crate) fn resolve_wb14_effective_soil_conductivity(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        soil_conductivity: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        if !Self::wb14_ksatadj_flag(request, phase_class)? {
            return Ok(soil_conductivity);
        }

        let solwpv_rounded = Self::wb14_ksatadj_solwpv_mode(request, phase_class)?;
        let (sat_frac, avthetafc, avthetadr) =
            Self::wb14_load_top_two_layer_ksatadj_metrics(request, phase_class)?;
        let upper_ks_mm_h =
            Self::wb14_soil_conductivity_to_mm_h(phase_class, soil_conductivity)?;
        let effective_ks_mm_h = Self::wb14_effective_ks_mm_h(
            request,
            phase_class,
            solwpv_rounded,
            upper_ks_mm_h,
            sat_frac,
            avthetafc,
            avthetadr,
        )?;
        Self::wb14_effective_ks_to_mps(phase_class, effective_ks_mm_h)
    }

    fn wb14_ksatadj_solwpv_mode(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let solwpv_symbol = BoundarySymbol::from("solwpv");
        let solwpv =
            Self::require_state_scalar_for_symbol(request, phase_class, &solwpv_symbol)?;
        if solwpv < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: solwpv_symbol,
                value: solwpv,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        let solwpv_rounded = solwpv.round();
        if (solwpv - solwpv_rounded).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("solwpv"),
                value: solwpv_rounded,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        Ok(solwpv_rounded)
    }

    fn wb14_soil_conductivity_to_mm_h(
        phase_class: HillslopeKernelPhaseClass,
        soil_conductivity: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        openwepp_unit_boundary::ProcessRateMillimetersPerHour::from_meters_per_second(
            soil_conductivity,
        )
        .map_err(|error| {
            Self::unit_conversion_guard_error(phase_class, BoundarySymbol::from("keff"), &error)
        })
        .map(openwepp_unit_boundary::ProcessRateMillimetersPerHour::as_millimeters_per_hour)
    }

    fn wb14_effective_ks_mm_h(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        solwpv_rounded: f64,
        upper_ks_mm_h: f64,
        sat_frac: f64,
        avthetafc: f64,
        avthetadr: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        if (solwpv_rounded - 9001.0).abs() <= WB11_ZERO_THRESHOLD {
            Self::wb14_effective_ks_9001(request, phase_class, upper_ks_mm_h, sat_frac)
        } else if solwpv_rounded >= 9002.0 - WB11_ZERO_THRESHOLD {
            Self::wb14_effective_ks_9002_plus(
                request,
                phase_class,
                solwpv_rounded,
                upper_ks_mm_h,
                sat_frac,
                avthetafc,
                avthetadr,
            )
        } else {
            Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("solwpv"),
                value: solwpv_rounded,
                minimum: Some(9001.0),
                maximum: None,
            })
        }
    }

    fn wb14_effective_ks_9001(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        upper_ks_mm_h: f64,
        sat_frac: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let ksatfac_symbol = BoundarySymbol::from("ksatfac");
        let ksatfac =
            Self::require_positive_state_for_symbol(request, phase_class, ksatfac_symbol.clone())?;
        let ksatrec_symbol = BoundarySymbol::from("ksatrec");
        let ksatrec =
            Self::require_positive_state_for_symbol(request, phase_class, ksatrec_symbol.clone())?;
        let lower_ks_mm_h = upper_ks_mm_h / ksatfac;
        let denominator = (1.0 / ksatrec).exp() - 1.0;
        if denominator <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("ksatrec"),
                value: denominator,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }
        Ok(((upper_ks_mm_h - lower_ks_mm_h) / denominator) * ((sat_frac / ksatrec).exp() - 1.0)
            + lower_ks_mm_h)
    }

    fn wb14_effective_ks_9002_plus(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        solwpv_rounded: f64,
        upper_ks_mm_h: f64,
        sat_frac: f64,
        avthetafc: f64,
        avthetadr: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let exponent = Self::wb14_effective_ks_9002_exponent(phase_class, avthetafc, avthetadr)?;
        let mut effective_ks = upper_ks_mm_h * sat_frac.powf(exponent);
        if (solwpv_rounded - 9003.0).abs() <= WB11_ZERO_THRESHOLD {
            let lkeff_symbol = BoundarySymbol::from("lkeff");
            let lkeff = Self::require_state_scalar_for_symbol(request, phase_class, &lkeff_symbol)?;
            if lkeff > 0.0 && effective_ks < lkeff {
                effective_ks = lkeff;
            }
        }
        Ok(effective_ks)
    }

    fn wb14_effective_ks_9002_exponent(
        phase_class: HillslopeKernelPhaseClass,
        avthetafc: f64,
        avthetadr: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let psi_denominator = avthetafc.ln() - avthetadr.ln();
        if psi_denominator <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("avthetafc_avthetadr"),
                value: psi_denominator,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }
        let psi = (1500.0_f64.ln() - 33.0_f64.ln()) / psi_denominator;
        if psi <= WB11_ZERO_THRESHOLD || !psi.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("psi"),
                value: psi,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }
        let lambda = 1.0 / psi;
        let exponent = (2.0 * lambda) + 3.0;
        if !lambda.is_finite() || !exponent.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("lambda"),
                value: lambda,
                minimum: None,
                maximum: None,
            });
        }
        Ok(exponent)
    }

    fn wb14_effective_ks_to_mps(
        phase_class: HillslopeKernelPhaseClass,
        effective_ks_mm_h: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        if !effective_ks_mm_h.is_finite() || effective_ks_mm_h < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("keff"),
                value: effective_ks_mm_h,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let effective_ks_mm_h = if effective_ks_mm_h < 0.0 {
            0.0
        } else {
            effective_ks_mm_h
        };
        openwepp_unit_boundary::ProcessRateMillimetersPerHour::try_new(effective_ks_mm_h)
            .map_err(|error| {
                Self::unit_conversion_guard_error(
                    phase_class,
                    BoundarySymbol::from("keff"),
                    &error,
                )
            })
            .map(openwepp_unit_boundary::ProcessRateMillimetersPerHour::as_meters_per_second)
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn insert_state(
        state: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
        symbol: &str,
        value: f64,
    ) {
        state.insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
    }

    fn source_intent_ksatadj_state() -> BTreeMap<BoundarySymbol, BoundaryValue> {
        let mut state = BTreeMap::new();
        insert_state(&mut state, "wb18_perc_theta_0001", 0.02);
        insert_state(&mut state, "wb18_perc_theta_0002", 0.04);
        insert_state(&mut state, "wb18_perc_fc_0001", 0.03);
        insert_state(&mut state, "wb18_perc_fc_0002", 0.033);
        insert_state(&mut state, "wb18_perc_ul_0001", 0.20);
        insert_state(&mut state, "wb18_perc_ul_0002", 0.20);
        insert_state(&mut state, "wb19_dg_0001", 0.10);
        insert_state(&mut state, "wb19_dg_0002", 0.10);
        insert_state(&mut state, "wb19_por_0001", 0.55);
        insert_state(&mut state, "wb19_por_0002", 0.55);
        insert_state(&mut state, "cpm_0001", 1.0);
        insert_state(&mut state, "cpm_0002", 1.0);
        insert_state(&mut state, "wb19_thetfc_0001", 0.40);
        insert_state(&mut state, "wb19_thetfc_0002", 0.45);
        insert_state(&mut state, "wb19_thetdr_0001", 0.10);
        insert_state(&mut state, "wb19_thetdr_0002", 0.12);
        state
    }

    fn ksatadj_request(
        state: BTreeMap<BoundarySymbol, BoundaryValue>,
    ) -> HillslopeKernelRequest<'static> {
        let state_surface = Box::leak(Box::new(state));
        let flux_surface = Box::leak(Box::new(BTreeMap::new()));
        HillslopeKernelRequest::with_transition_context(
            "runoff_reconciliation",
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
            HillslopeConsumerAdapter::Watbal,
            None,
            None,
            state_surface,
            flux_surface,
        )
    }

    #[test]
    fn wb14_ksatadj_sat_frac_uses_source_intent_avsat_not_ul_surrogate() {
        let request = ksatadj_request(source_intent_ksatadj_state());
        let (sat_frac, avthetafc, avthetadr) =
            Wb11HydrologyKernel::wb14_load_top_two_layer_ksatadj_metrics(
                &request,
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
            )
            .expect("source-intent operands should form WB14 ksatadj metrics");

        let expected_source_intent_sat_frac = 0.41 / 0.55;
        let legacy_ul_surrogate_sat_frac = 0.06 / 0.40;
        assert!(
            (sat_frac - expected_source_intent_sat_frac).abs() <= 1.0e-12,
            "sat_frac must use avsat/(avpor*avcpm); observed {sat_frac}"
        );
        assert!(
            (sat_frac - legacy_ul_surrogate_sat_frac).abs() > 1.0e-3,
            "non-aliased fixture must reject theta_sum/ul_sum surrogate"
        );
        assert!((avthetafc - 0.425).abs() <= 1.0e-12);
        assert!((avthetadr - 0.11).abs() <= 1.0e-12);
    }

    #[test]
    fn wb14_ksatadj_missing_source_intent_operand_is_typed_failure() {
        let mut state = source_intent_ksatadj_state();
        state.remove(&BoundarySymbol::from("cpm_0002"));
        let request = ksatadj_request(state);
        let error = Wb11HydrologyKernel::wb14_load_top_two_layer_ksatadj_metrics(
            &request,
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
        )
        .expect_err("active ksatadj metrics must require cpm_0002");

        assert_eq!(
            error,
            Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                phase_class: HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                symbol: BoundarySymbol::from("cpm_0002"),
            }
        );
    }
}
