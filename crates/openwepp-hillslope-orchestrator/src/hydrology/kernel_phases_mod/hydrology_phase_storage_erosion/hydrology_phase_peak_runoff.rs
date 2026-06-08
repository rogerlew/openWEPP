#[allow(clippy::wildcard_imports)]
use crate::hydrology::*;

impl Wb11HydrologyKernel {
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
