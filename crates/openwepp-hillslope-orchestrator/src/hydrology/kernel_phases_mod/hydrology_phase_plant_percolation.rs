#[allow(clippy::wildcard_imports)]
use crate::hydrology::*;

struct Wb18PercolationLayers {
    theta: Vec<f64>,
    field_capacity: Vec<f64>,
    upper_limit: Vec<f64>,
    conductivity: Vec<f64>,
    depth: Vec<f64>,
}

struct Wb18SamePassInfiltration {
    depth: Option<f64>,
    lineage: bool,
}

struct Wb18PercolationLaneConfig {
    lane_substeps: f64,
    daily_lane: bool,
    restrictive_layer_enabled: bool,
    restrictive_layer_conductivity: f64,
    restrictive_layer_thickness: f64,
    restrictive_layer_conductivity_symbol: BoundarySymbol,
    restrictive_layer_thickness_symbol: BoundarySymbol,
}

struct Wb18PercolationRoutingResult {
    per_layer_flux: Vec<f64>,
    percolation_loss: f64,
}

struct Wb18PercolationSoilWaterLedger {
    soil_water: f64,
    reconcile_legacy_soil_water_from_layers: bool,
    computed_soil_water_before: f64,
    same_pass_infiltration_depth: f64,
    percolation_loss: f64,
}

impl Wb11HydrologyKernel {
pub(crate) fn effective_swu_plant_tolerance(raw_plant_tolerance: f64) -> f64 {
        if raw_plant_tolerance <= 0.0 {
            0.25
        } else {
            raw_plant_tolerance.clamp(WB17_PLTOL_MIN, WB17_PLTOL_MAX)
        }
    }

    #[allow(clippy::too_many_lines)]
pub(crate) fn run_plant_root_uptake(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<KernelRunResponse, Wb11HydrologyKernelGuardError> {
        let phase_class = HillslopeKernelPhaseClass::HydrologyPlantRootUptake;
        let base_et = Self::require_flux_scalar(request, phase_class, WB11_SYMBOL_ET)?;
        Self::require_flux_range(phase_class, WB11_SYMBOL_ET, base_et, Some(0.0), None)?;
        let soil_water = Self::require_state_scalar(request, phase_class, WB11_SYMBOL_SOIL_WATER)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_SOIL_WATER,
            soil_water,
            Some(0.0),
            None,
        )?;

        let etp_symbol = BoundarySymbol::from(WB17_FLUX_SYMBOL_ETP);
        let etp = Self::require_flux_scalar_for_symbol(request, phase_class, &etp_symbol)?;
        Self::require_flux_range_for_symbol(
            phase_class,
            &etp_symbol,
            etp,
            Some(0.0),
            None,
        )?;

        let (nsl_symbol, layer_count) = Self::require_wb11_layer_count(request, phase_class)?;
        if layer_count == 0 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: nsl_symbol,
                value: 0.0,
                minimum: Some(1.0),
                maximum: None,
            });
        }

        let mut layer_storage = Vec::with_capacity(layer_count);
        let mut layer_depth = Vec::with_capacity(layer_count);
        let mut layer_upper_limit = Vec::with_capacity(layer_count);
        for layer_index in 1..=layer_count {
            let theta_symbol = Self::wb18_perc_state_symbol("theta", layer_index);
            let theta =
                Self::require_state_scalar_for_symbol(request, phase_class, &theta_symbol)?;
            Self::require_state_range_for_symbol(
                phase_class,
                &theta_symbol,
                theta,
                Some(0.0),
                None,
            )?;

            let (dg_symbol, dg) =
                Self::require_wb19_dg_scalar(request, phase_class, layer_index)?;
            Self::require_state_range_for_symbol(
                phase_class,
                &dg_symbol,
                dg,
                Some(WB11_ZERO_THRESHOLD),
                None,
            )?;

            let ul_symbol = Self::wb18_perc_state_symbol("ul", layer_index);
            let ul = Self::require_state_scalar_for_symbol(request, phase_class, &ul_symbol)?;
            Self::require_state_range_for_symbol(
                phase_class,
                &ul_symbol,
                ul,
                Some(WB11_ZERO_THRESHOLD),
                None,
            )?;

            layer_storage.push(theta);
            layer_depth.push(dg);
            layer_upper_limit.push(ul);
        }

        let root_depth_symbol = BoundarySymbol::from(PL_GROWTH_STATE_RTD_SYMBOL);
        let root_depth =
            Self::require_state_scalar_for_symbol(request, phase_class, &root_depth_symbol)?;
        Self::require_state_range_for_symbol(
            phase_class,
            &root_depth_symbol,
            root_depth,
            Some(0.0),
            None,
        )?;

        let plant_tolerance_symbol = BoundarySymbol::from("pltol");
        let raw_plant_tolerance =
            Self::require_state_scalar_for_symbol(request, phase_class, &plant_tolerance_symbol)?;
        let plant_tolerance = Self::effective_swu_plant_tolerance(raw_plant_tolerance);
        Self::require_state_range_for_symbol(
            phase_class,
            &plant_tolerance_symbol,
            plant_tolerance,
            Some(WB17_PLTOL_MIN),
            Some(WB17_PLTOL_MAX),
        )?;

        let profile_depth: f64 = layer_depth.iter().sum();
        let effective_root_depth = root_depth.min(profile_depth);
        let soil_water_before =
            Self::wb18_aggregate_soil_water_after_percolation(request, phase_class, &layer_storage)?;
        let mut layer_potential_uptake = vec![0.0_f64; layer_count];
        let mut layer_actual_uptake = vec![0.0_f64; layer_count];
        let mut transpiration_actual = 0.0;
        if etp > WB11_ZERO_THRESHOLD && effective_root_depth > WB11_ZERO_THRESHOLD {
            let mut rooted_layer_count = layer_count;
            let mut root_cumulative_depth = 0.0;
            for (index, depth) in layer_depth.iter().enumerate() {
                root_cumulative_depth += *depth;
                if effective_root_depth <= root_cumulative_depth + WB11_ZERO_THRESHOLD {
                    rooted_layer_count = index + 1;
                    break;
                }
            }

            let uptake_potential_symbol = BoundarySymbol::from(WB17_FLUX_SYMBOL_UPI);
            let mut previous_cumulative_potential = 0.0;
            let mut layer_cumulative_depth = 0.0;
            for index in 0..rooted_layer_count {
                layer_cumulative_depth += layer_depth[index];
                let gx = if index + 1 < rooted_layer_count {
                    layer_cumulative_depth
                } else {
                    effective_root_depth
                };
                let cumulative_potential = (1.0
                    - (-WB17_SWU_UB * gx / effective_root_depth).exp())
                    * etp
                    / WB17_SWU_UOB;
                let mut potential_uptake = cumulative_potential - previous_cumulative_potential;
                if potential_uptake < 0.0 && potential_uptake.abs() <= WB11_ZERO_THRESHOLD {
                    potential_uptake = 0.0;
                }
                layer_potential_uptake[index] = potential_uptake;
                Self::require_flux_range_for_symbol(
                    phase_class,
                    &uptake_potential_symbol,
                    potential_uptake,
                    Some(0.0),
                    None,
                )?;

                let stress_threshold = plant_tolerance * layer_upper_limit[index];
                let mut layer_uptake = potential_uptake;
                if layer_storage[index] < stress_threshold {
                    layer_uptake *= layer_storage[index] / stress_threshold;
                }
                if layer_storage[index] < layer_uptake {
                    layer_uptake = layer_storage[index];
                }
                let remaining_transpiration = (etp - transpiration_actual).max(0.0);
                if layer_uptake > remaining_transpiration {
                    layer_uptake = remaining_transpiration;
                }
                if layer_uptake < 1.0e-10 {
                    layer_uptake = 0.0;
                }
                layer_actual_uptake[index] = layer_uptake;
                layer_storage[index] -= layer_uptake;
                if layer_storage[index] < 1.0e-10 {
                    layer_storage[index] = 0.0;
                }
                transpiration_actual += layer_uptake;
                previous_cumulative_potential = cumulative_potential;
            }
        }

        let upi: f64 = layer_potential_uptake.iter().sum();
        let ui: f64 = layer_actual_uptake.iter().sum();
        Self::require_flux_range(phase_class, WB17_SYMBOL_EP, ui, Some(0.0), Some(etp))?;

        let mut soil_water_after = soil_water;
        if ui > WB11_ZERO_THRESHOLD {
            soil_water_after = Self::wb18_aggregate_soil_water_after_percolation(
                request,
                phase_class,
                &layer_storage,
            )?;
            let storage_uptake_m = soil_water_before - soil_water_after;
            let storage_correction_m = storage_uptake_m - ui;
            if storage_correction_m.abs() > f64::EPSILON
                && let Some(index) = layer_actual_uptake.iter().rposition(|value| *value > 0.0)
            {
                layer_storage[index] += storage_correction_m;
                soil_water_after = Self::wb18_aggregate_soil_water_after_percolation(
                    request,
                    phase_class,
                    &layer_storage,
                )?;
            }
        }
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_SOIL_WATER,
            soil_water_after,
            Some(0.0),
            None,
        )?;

        let actual_et = base_et + ui;
        Self::require_flux_range(phase_class, WB11_SYMBOL_ET, actual_et, Some(0.0), None)?;

        let uptake_potential_symbol = BoundarySymbol::from(WB17_FLUX_SYMBOL_UPI);
        let uptake_actual_symbol = BoundarySymbol::from(WB17_FLUX_SYMBOL_UI);
        Self::require_flux_range_for_symbol(
            phase_class,
            &uptake_potential_symbol,
            upi,
            Some(0.0),
            None,
        )?;
        Self::require_flux_range_for_symbol(
            phase_class,
            &uptake_actual_symbol,
            ui,
            Some(0.0),
            None,
        )?;
        for index in 0..layer_count {
            let potential_symbol =
                Self::wb17_layer_flux_symbol(WB17_FLUX_SYMBOL_UPI, index + 1);
            let actual_symbol = Self::wb17_layer_flux_symbol(WB17_FLUX_SYMBOL_UI, index + 1);
            Self::require_flux_range_for_symbol(
                phase_class,
                &potential_symbol,
                layer_potential_uptake[index],
                Some(0.0),
                None,
            )?;
            Self::require_flux_range_for_symbol(
                phase_class,
                &actual_symbol,
                layer_actual_uptake[index],
                Some(0.0),
                Some(layer_potential_uptake[index]),
            )?;
        }

        let ws = if etp <= WB11_ZERO_THRESHOLD || effective_root_depth <= WB11_ZERO_THRESHOLD {
            1.0
        } else {
            (ui / etp).min(1.0)
        };
        Self::require_flux_range(phase_class, WB11_SYMBOL_WS, ws, Some(0.0), Some(1.0))?;

        let Ok(status) =
            SimulationStatus::ok(SimulationPhase::HillslopeKernel, "HKERNEL-WB17-SWU-OK-001")
        else {
            unreachable!("status message ids are non-empty WB17 constants")
        };
        let effective_plant_tolerance_symbol = BoundarySymbol::from("swu_effective_pltol");
        let mut state_updates = vec![
            WritebackField::bounded(WB11_SYMBOL_SOIL_WATER, soil_water_after, Some(0.0), None),
            WritebackField::bounded(
                plant_tolerance_symbol,
                plant_tolerance,
                Some(WB17_PLTOL_MIN),
                Some(WB17_PLTOL_MAX),
            ),
            WritebackField::bounded(
                effective_plant_tolerance_symbol,
                plant_tolerance,
                Some(WB17_PLTOL_MIN),
                Some(WB17_PLTOL_MAX),
            ),
        ];
        for (index, value) in layer_storage.iter().enumerate() {
            state_updates.push(WritebackField::bounded(
                Self::wb18_perc_state_symbol("theta", index + 1),
                *value,
                Some(0.0),
                None,
            ));
        }

        let mut flux_updates = vec![
            WritebackField::bounded(WB11_SYMBOL_ET, actual_et, Some(0.0), None),
            WritebackField::bounded(WB11_SYMBOL_WS, ws, Some(0.0), Some(1.0)),
            WritebackField::bounded(WB17_SYMBOL_EP, ui, Some(0.0), None),
            WritebackField::bounded(etp_symbol, etp, Some(0.0), None),
            WritebackField::bounded(uptake_potential_symbol, upi, Some(0.0), None),
            WritebackField::bounded(uptake_actual_symbol, ui, Some(0.0), None),
        ];
        for index in 0..layer_count {
            flux_updates.push(WritebackField::bounded(
                Self::wb17_layer_flux_symbol(WB17_FLUX_SYMBOL_UPI, index + 1),
                layer_potential_uptake[index],
                Some(0.0),
                None,
            ));
            flux_updates.push(WritebackField::bounded(
                Self::wb17_layer_flux_symbol(WB17_FLUX_SYMBOL_UI, index + 1),
                layer_actual_uptake[index],
                Some(0.0),
                Some(layer_potential_uptake[index]),
            ));
        }

        let writeback = KernelWritebackPayload::with_updates(state_updates, flux_updates);
        Ok(KernelRunResponse::new(status, writeback))
    }
pub(crate) fn wb18_aggregate_soil_water_after_percolation(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        theta: &[f64],
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let mut soil_water_after = 0.0;
        for (index, layer_theta) in theta.iter().enumerate() {
            let layer_index = index + 1;
            let (thetdr_symbol, thetdr) =
                Self::require_wb19_thetdr_scalar(request, phase_class, layer_index)?;
            let (dg_symbol, dg) =
                Self::require_wb19_dg_scalar(request, phase_class, layer_index)?;
            let frozen_depth_symbol = Self::wb18_perc_state_symbol("frozen_depth", layer_index);

            Self::require_state_range_for_symbol(
                phase_class,
                &thetdr_symbol,
                thetdr,
                Some(0.0),
                Some(1.0),
            )?;

            Self::require_state_range_for_symbol(
                phase_class,
                &dg_symbol,
                dg,
                Some(WB11_ZERO_THRESHOLD),
                None,
            )?;

            let frozen_depth = Self::optional_state_scalar_for_symbol(
                request,
                phase_class,
                &frozen_depth_symbol,
            )?
            .unwrap_or(0.0);
            let frozen_depth = Self::resolve_effective_wb18_frozen_depth(
                request,
                phase_class,
                layer_index,
                dg,
                frozen_depth,
            )?;
            Self::require_state_range_for_symbol(
                phase_class,
                &frozen_depth_symbol,
                frozen_depth,
                Some(0.0),
                Some(dg),
            )?;

            let unfrozen_depth_m = (dg - frozen_depth).max(0.0);
            let layer_soil_water = *layer_theta + thetdr * unfrozen_depth_m;
            if !layer_soil_water.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                    phase_class,
                    symbol: BoundarySymbol::from(WB11_SYMBOL_SOIL_WATER),
                    value: layer_soil_water,
                });
            }
            soil_water_after += layer_soil_water;
        }

        if !soil_water_after.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol: BoundarySymbol::from(WB11_SYMBOL_SOIL_WATER),
                value: soil_water_after,
            });
        }
        Ok(soil_water_after)
    }

    fn resolve_effective_wb18_frozen_depth(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        layer_index: usize,
        dg: f64,
        aggregate_frozen_depth_m: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        if aggregate_frozen_depth_m <= dg + WB11_ZERO_THRESHOLD {
            return Ok(aggregate_frozen_depth_m.min(dg));
        }

        let fine_count_symbol =
            Self::frost_layer_symbol(FROST_RUNTIME_LAYER_FINE_COUNT_ROOT, layer_index);
        if let Some(fine_count_raw) =
            Self::optional_state_scalar_for_symbol(request, phase_class, &fine_count_symbol)?
        {
            let rounded = fine_count_raw.round();
            if (fine_count_raw - rounded).abs() > WB11_ZERO_THRESHOLD || rounded < 1.0 {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: fine_count_symbol.clone(),
                    value: fine_count_raw,
                    minimum: Some(1.0),
                    maximum: Some(Self::diagnostic_count_to_f64(usize::MAX)),
                });
            }
            let fine_count = format!("{rounded:.0}").parse::<usize>().map_err(|_| {
                Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: fine_count_symbol.clone(),
                    value: fine_count_raw,
                    minimum: Some(1.0),
                    maximum: Some(Self::diagnostic_count_to_f64(usize::MAX)),
                }
            })?;
            let mut fine_frozen_depth_m = 0.0_f64;
            for fine_index in 1..=fine_count {
                let slfsd_symbol = Self::frost_fine_layer_symbol_for_request(
                    request,
                    FROST_RUNTIME_FINE_SLFSD_M_ROOT,
                    layer_index,
                    fine_index,
                );
                let slfsd_m =
                    Self::require_state_scalar_for_symbol(request, phase_class, &slfsd_symbol)?;
                Self::require_state_range_for_symbol(
                    phase_class,
                    &slfsd_symbol,
                    slfsd_m,
                    Some(0.0),
                    None,
                )?;
                fine_frozen_depth_m += slfsd_m;
            }
            Self::require_state_range_for_symbol(
                phase_class,
                &Self::wb18_perc_state_symbol("frozen_depth", layer_index),
                fine_frozen_depth_m,
                Some(0.0),
                Some(dg),
            )?;
            return Ok(fine_frozen_depth_m.min(dg));
        }

        let scalar_frost_depth = Self::optional_state_scalar_for_symbol(
            request,
            phase_class,
            &BoundarySymbol::from(FROST_RUNTIME_FRDP_M_SYMBOL),
        )?
        .or_else(|| {
            Self::optional_state_scalar_for_symbol(
                request,
                phase_class,
                &BoundarySymbol::from(WB14_SYMBOL_FROST_RUNTIME_DFROST),
            )
            .ok()
            .flatten()
        });
        let Some(scalar_frost_depth) = scalar_frost_depth else {
            return Ok(0.0);
        };
        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from(FROST_RUNTIME_FRDP_M_SYMBOL),
            scalar_frost_depth,
            Some(0.0),
            None,
        )?;

        let mut cumulative_depth_m = 0.0_f64;
        for prior_layer_index in 1..layer_index {
            let (prior_dg_symbol, prior_dg) =
                Self::require_wb19_dg_scalar(request, phase_class, prior_layer_index)?;
            Self::require_state_range_for_symbol(
                phase_class,
                &prior_dg_symbol,
                prior_dg,
                Some(WB11_ZERO_THRESHOLD),
                None,
            )?;
            cumulative_depth_m += prior_dg;
        }

        Ok((scalar_frost_depth - cumulative_depth_m).clamp(0.0, dg))
    }
pub(crate) fn resolve_infiltration_tillage_depth(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        layer_depth: &[f64],
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let first_layer_depth = *layer_depth.first().ok_or_else(|| {
            Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("nsl"),
                value: 0.0,
                minimum: Some(1.0),
                maximum: None,
            }
        })?;
        let profile_depth = layer_depth.iter().sum::<f64>();
        let tillage_depth_symbol = BoundarySymbol::from("management.initial.params.tillay2_m");
        let tillage_depth = Self::optional_state_scalar_for_symbol(
            request,
            phase_class,
            &tillage_depth_symbol,
        )?
        .unwrap_or(0.0);
        Self::require_state_range_for_symbol(
            phase_class,
            &tillage_depth_symbol,
            tillage_depth,
            Some(0.0),
            Some(profile_depth),
        )?;

        if tillage_depth > WB11_ZERO_THRESHOLD {
            Ok(tillage_depth)
        } else {
            Ok(first_layer_depth)
        }
    }
    pub(crate) fn apply_same_pass_infiltration_to_layer_storage(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        theta: &mut [f64],
        layer_depth: &[f64],
        infiltration: f64,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_INFILTRATION,
            infiltration,
            Some(0.0),
            None,
        )?;
        if infiltration <= WB11_ZERO_THRESHOLD {
            return Ok(());
        }
        if theta.len() != layer_depth.len() || theta.is_empty() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("nsl"),
                value: Self::diagnostic_count_to_f64(theta.len()),
                minimum: Some(1.0),
                maximum: Some(Self::diagnostic_count_to_f64(layer_depth.len())),
            });
        }

        let tillage_depth =
            Self::resolve_infiltration_tillage_depth(request, phase_class, layer_depth)?;
        let mut remaining_infiltration = infiltration;
        let mut cumulative_depth = 0.0_f64;
        for (index, layer_theta) in theta.iter_mut().enumerate() {
            if remaining_infiltration <= WB11_ZERO_THRESHOLD {
                break;
            }
            cumulative_depth += layer_depth[index];
            let add_to_layer = if cumulative_depth < tillage_depth - WB11_ZERO_THRESHOLD {
                remaining_infiltration * layer_depth[index] / tillage_depth
            } else {
                remaining_infiltration
            };
            if !add_to_layer.is_finite() || add_to_layer < -WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB12_SYMBOL_INFILTRATION),
                    value: add_to_layer,
                    minimum: Some(0.0),
                    maximum: Some(infiltration),
                });
            }
            *layer_theta += add_to_layer.max(0.0);
            Self::require_state_range_for_symbol(
                phase_class,
                &Self::wb18_perc_state_symbol("theta", index + 1),
                *layer_theta,
                Some(0.0),
                None,
            )?;
            remaining_infiltration -= add_to_layer;
        }

        if remaining_infiltration > WB11_ZERO_THRESHOLD {
            let last_index = theta.len() - 1;
            theta[last_index] += remaining_infiltration;
            Self::require_state_range_for_symbol(
                phase_class,
                &Self::wb18_perc_state_symbol("theta", last_index + 1),
                theta[last_index],
                Some(0.0),
                None,
            )?;
        }
        Ok(())
    }

    fn apply_wb18_storage_roundoff_delta_to_layer_storage(
        phase_class: HillslopeKernelPhaseClass,
        theta: &mut [f64],
        delta_m: f64,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if delta_m.abs() <= f64::EPSILON {
            return Ok(());
        }
        if theta.is_empty() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("nsl"),
                value: 0.0,
                minimum: Some(1.0),
                maximum: None,
            });
        }

        if delta_m > 0.0 {
            let last_index = theta.len() - 1;
            theta[last_index] += delta_m;
            Self::require_state_range_for_symbol(
                phase_class,
                &Self::wb18_perc_state_symbol("theta", last_index + 1),
                theta[last_index],
                Some(0.0),
                None,
            )?;
            return Ok(());
        }

        let debit_m = -delta_m;
        if let Some(index) = theta
            .iter()
            .rposition(|storage| *storage + WB11_ZERO_THRESHOLD >= debit_m)
        {
            theta[index] = (theta[index] - debit_m).max(0.0);
            Self::require_state_range_for_symbol(
                phase_class,
                &Self::wb18_perc_state_symbol("theta", index + 1),
                theta[index],
                Some(0.0),
                None,
            )?;
            return Ok(());
        }

        Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
            phase_class,
            symbol: BoundarySymbol::from(WB11_SYMBOL_SOIL_WATER),
            value: debit_m,
            minimum: Some(0.0),
            maximum: Some(theta.iter().sum()),
        })
    }

    pub(crate) fn apply_post_et_upper_limit_redistribution(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        theta: &mut [f64],
        upper_limit: &[f64],
        outside_water_active: bool,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if theta.len() != upper_limit.len() || theta.is_empty() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("nsl"),
                value: Self::diagnostic_count_to_f64(theta.len()),
                minimum: Some(1.0),
                maximum: Some(Self::diagnostic_count_to_f64(upper_limit.len())),
            });
        }

        let frozen_water = if outside_water_active {
            Some(Self::wb19_frozen_water_by_layer(request, phase_class, theta.len())?)
        } else {
            None
        };

        for index in (1..theta.len()).rev() {
            let layer_upper_limit = upper_limit[index];
            if layer_upper_limit <= WB11_ZERO_THRESHOLD || !layer_upper_limit.is_finite() {
                let ul_symbol = Self::wb18_perc_state_symbol("ul", index + 1);
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: ul_symbol,
                    value: layer_upper_limit,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: None,
                });
            }

            let active_cap = if let Some(frozen_water_by_layer) = &frozen_water {
                (layer_upper_limit - frozen_water_by_layer[index]).max(0.0)
            } else {
                layer_upper_limit
            };
            if !active_cap.is_finite() {
                let cap_symbol = Self::wb18_perc_state_symbol("ul", index + 1);
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: cap_symbol,
                    value: active_cap,
                    minimum: Some(0.0),
                    maximum: Some(layer_upper_limit),
                });
            }

            if theta[index] > active_cap + WB11_ZERO_THRESHOLD {
                let excess = theta[index] - active_cap;
                theta[index] = active_cap;
                theta[index - 1] += excess;

                for affected_index in [index - 1, index] {
                    Self::require_state_range_for_symbol(
                        phase_class,
                        &Self::wb18_perc_state_symbol("theta", affected_index + 1),
                        theta[affected_index],
                        Some(0.0),
                        None,
                    )?;
                }
            }
        }

        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    pub(crate) fn run_percolation(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<KernelRunResponse, Wb11HydrologyKernelGuardError> {
        let phase_class = HillslopeKernelPhaseClass::HydrologyPercolationDeepSeepage;
        let (soil_water, reconcile_legacy_soil_water_from_layers) =
            Self::validate_wb18_legacy_percolation_inputs(request, phase_class)?;
        let (nsl_symbol, layer_count) = Self::require_wb11_layer_count(request, phase_class)?;
        if layer_count == 0 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: nsl_symbol,
                value: 0.0,
                minimum: Some(1.0),
                maximum: None,
            });
        }
        let mut layers = Self::read_wb18_percolation_layers(request, phase_class, layer_count)?;
        let computed_soil_water_before =
            Self::wb18_aggregate_soil_water_after_percolation(request, phase_class, &layers.theta)?
                .max(0.0);
        if reconcile_legacy_soil_water_from_layers {
            let reconciled_soil_water = computed_soil_water_before;
            Self::require_state_range(
                phase_class,
                WB11_SYMBOL_SOIL_WATER,
                reconciled_soil_water.max(0.0),
                Some(0.0),
                None,
            )?;
        }

        let same_pass_infiltration =
            Self::resolve_wb18_percolation_same_pass_infiltration(request, phase_class)?;
        let lane_config = Self::resolve_wb18_percolation_lane_config(request, phase_class)?;
        let mut routing = Self::run_wb18_percolation_routing(
            request,
            phase_class,
            &mut layers,
            &same_pass_infiltration,
            &lane_config,
        )?;
        Self::canonicalize_wb18_deep_percolation_roundoff(&mut layers, &mut routing);

        let soil_water_after = Self::resolve_wb18_percolation_soil_water_after(
            request,
            phase_class,
            &mut layers,
            &Wb18PercolationSoilWaterLedger {
                soil_water,
                reconcile_legacy_soil_water_from_layers,
                computed_soil_water_before,
                same_pass_infiltration_depth: same_pass_infiltration.depth.unwrap_or(0.0),
                percolation_loss: routing.percolation_loss,
            },
        )?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_SOIL_WATER,
            soil_water_after,
            Some(0.0),
            None,
        )?;
        Self::require_flux_range(
            phase_class,
            WB11_SYMBOL_PERC_LOSS_D,
            routing.percolation_loss,
            Some(0.0),
            None,
        )?;

        Ok(Self::build_wb18_percolation_response(
            soil_water_after,
            &layers,
            &routing,
            &same_pass_infiltration,
        ))
    }

    fn validate_wb18_legacy_percolation_inputs(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<(f64, bool), Wb11HydrologyKernelGuardError> {
        let soil_water = Self::require_state_scalar(request, phase_class, WB11_SYMBOL_SOIL_WATER)?;
        let reconcile_legacy_soil_water_from_layers = soil_water < -WB11_ZERO_THRESHOLD;
        if !reconcile_legacy_soil_water_from_layers {
            Self::require_state_range(
                phase_class,
                WB11_SYMBOL_SOIL_WATER,
                soil_water.max(0.0),
                Some(0.0),
                None,
            )?;
        }

        // Keep legacy WB11 symbol validation to preserve mixed-lane seam guard
        // posture while WB18 per-layer symbols carry the execution authority.
        let field_capacity_legacy =
            Self::require_state_scalar(request, phase_class, WB11_SYMBOL_FIELD_CAPACITY)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_FIELD_CAPACITY,
            field_capacity_legacy,
            Some(0.0),
            None,
        )?;
        let perc_fraction_legacy =
            Self::require_state_scalar(request, phase_class, WB11_SYMBOL_PERC_FRACTION)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_PERC_FRACTION,
            perc_fraction_legacy,
            Some(0.0),
            Some(1.0),
        )?;

        Ok((soil_water, reconcile_legacy_soil_water_from_layers))
    }

    fn read_wb18_percolation_layers(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        layer_count: usize,
    ) -> Result<Wb18PercolationLayers, Wb11HydrologyKernelGuardError> {
        let mut theta = Vec::with_capacity(layer_count);
        let mut field_capacity = Vec::with_capacity(layer_count);
        let mut upper_limit = Vec::with_capacity(layer_count);
        let mut conductivity = Vec::with_capacity(layer_count);
        let mut depth = Vec::with_capacity(layer_count);

        for layer_index in 1..=layer_count {
            let theta_symbol = Self::wb18_perc_state_symbol("theta", layer_index);
            let fc_symbol = Self::wb18_perc_state_symbol("fc", layer_index);
            let ul_symbol = Self::wb18_perc_state_symbol("ul", layer_index);
            let ssc_symbol = Self::wb18_perc_state_symbol("ssc", layer_index);
            let (dg_symbol, dg) =
                Self::require_wb19_dg_scalar(request, phase_class, layer_index)?;

            let layer_theta =
                Self::require_state_scalar_for_symbol(request, phase_class, &theta_symbol)?;
            Self::require_state_range_for_symbol(
                phase_class,
                &theta_symbol,
                layer_theta,
                Some(0.0),
                None,
            )?;

            let layer_fc = Self::require_state_scalar_for_symbol(request, phase_class, &fc_symbol)?;
            Self::require_state_range_for_symbol(
                phase_class,
                &fc_symbol,
                layer_fc,
                Some(0.0),
                None,
            )?;

            let layer_ul = Self::require_state_scalar_for_symbol(request, phase_class, &ul_symbol)?;
            if layer_ul <= WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: ul_symbol,
                    value: layer_ul,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: None,
                });
            }
            if layer_fc > layer_ul + WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: fc_symbol,
                    value: layer_fc,
                    minimum: Some(0.0),
                    maximum: Some(layer_ul),
                });
            }

            let layer_ssc =
                Self::require_state_scalar_for_symbol(request, phase_class, &ssc_symbol)?;
            if layer_ssc <= WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: ssc_symbol,
                    value: layer_ssc,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: None,
                });
            }

            Self::require_state_range_for_symbol(
                phase_class,
                &dg_symbol,
                dg,
                Some(WB11_ZERO_THRESHOLD),
                None,
            )?;

            theta.push(layer_theta);
            field_capacity.push(layer_fc);
            upper_limit.push(layer_ul);
            conductivity.push(layer_ssc);
            depth.push(dg);
        }

        Ok(Wb18PercolationLayers {
            theta,
            field_capacity,
            upper_limit,
            conductivity,
            depth,
        })
    }

    fn resolve_wb18_percolation_same_pass_infiltration(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<Wb18SamePassInfiltration, Wb11HydrologyKernelGuardError> {
        let depth = if request
            .state_surface
            .contains_key(&BoundarySymbol::from("management.initial.params.tillay2_m"))
        {
            Self::resolve_wb18_same_pass_infiltration_lineage(request, phase_class)?
        } else {
            None
        };
        let lineage = if depth.is_some() {
            Self::wb18_should_reconstruct_same_pass_infiltration_lineage(request, phase_class)?
        } else {
            false
        };

        Ok(Wb18SamePassInfiltration { depth, lineage })
    }

    fn resolve_wb18_percolation_lane_config(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<Wb18PercolationLaneConfig, Wb11HydrologyKernelGuardError> {
        let lane_substeps_symbol = BoundarySymbol::from("wb18_perc_lane_substeps");
        let lane_substeps_raw =
            Self::optional_state_scalar_for_symbol(request, phase_class, &lane_substeps_symbol)?
                .unwrap_or(1.0);
        if lane_substeps_raw < 1.0 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: lane_substeps_symbol.clone(),
                value: lane_substeps_raw,
                minimum: Some(1.0),
                maximum: None,
            });
        }
        let lane_substeps = lane_substeps_raw.round();
        if (lane_substeps_raw - lane_substeps).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: lane_substeps_symbol,
                value: lane_substeps_raw,
                minimum: Some(1.0),
                maximum: None,
            });
        }
        let daily_lane = (lane_substeps - 1.0).abs() <= WB11_ZERO_THRESHOLD;

        let restrictive_layer_flag_symbol = BoundarySymbol::from("slflag");
        let restrictive_layer_flag_raw = Self::optional_state_scalar_for_symbol(
            request,
            phase_class,
            &restrictive_layer_flag_symbol,
        )?
        .unwrap_or(0.0);
        let restrictive_layer_enabled =
            if restrictive_layer_flag_raw.abs() <= WB11_ZERO_THRESHOLD {
                false
            } else if (restrictive_layer_flag_raw - 1.0).abs() <= WB11_ZERO_THRESHOLD {
                true
            } else {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: restrictive_layer_flag_symbol,
                    value: restrictive_layer_flag_raw,
                    minimum: Some(0.0),
                    maximum: Some(1.0),
                });
            };
        let restrictive_layer_conductivity_symbol = BoundarySymbol::from("kslast");
        let restrictive_layer_conductivity = if restrictive_layer_enabled {
            let observed = Self::require_state_scalar_for_symbol(
                request,
                phase_class,
                &restrictive_layer_conductivity_symbol,
            )?;
            if observed <= WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: restrictive_layer_conductivity_symbol.clone(),
                    value: observed,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: None,
                });
            }
            observed
        } else {
            0.0
        };
        let restrictive_layer_thickness_symbol = BoundarySymbol::from("ui_bdrkth");
        let restrictive_layer_thickness = if restrictive_layer_enabled && !daily_lane {
            let observed = Self::require_state_scalar_for_symbol(
                request,
                phase_class,
                &restrictive_layer_thickness_symbol,
            )?;
            if observed <= WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: restrictive_layer_thickness_symbol.clone(),
                    value: observed,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: None,
                });
            }
            observed
        } else {
            0.0
        };

        Ok(Wb18PercolationLaneConfig {
            lane_substeps,
            daily_lane,
            restrictive_layer_enabled,
            restrictive_layer_conductivity,
            restrictive_layer_thickness,
            restrictive_layer_conductivity_symbol,
            restrictive_layer_thickness_symbol,
        })
    }

    fn run_wb18_percolation_routing(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        layers: &mut Wb18PercolationLayers,
        same_pass_infiltration: &Wb18SamePassInfiltration,
        lane_config: &Wb18PercolationLaneConfig,
    ) -> Result<Wb18PercolationRoutingResult, Wb11HydrologyKernelGuardError> {
        let layer_count = layers.theta.len();
        let mut per_layer_flux = vec![0.0_f64; layer_count];
        let mut percolation_loss = 0.0_f64;

        // Bottom-up routing mirrors legacy WEPP percolation ordering in PURK.
        let mut lane_substep_index = 0.0_f64;
        while lane_substep_index < lane_config.lane_substeps {
            if let Some(infiltration) = same_pass_infiltration.depth {
                Self::apply_same_pass_infiltration_to_layer_storage(
                    request,
                    phase_class,
                    &mut layers.theta,
                    &layers.depth,
                    infiltration / lane_config.lane_substeps,
                )?;
            }

            let substep_percolation_loss = Self::run_wb18_percolation_substep(
                phase_class,
                layers,
                lane_config,
                &mut per_layer_flux,
            )?;
            percolation_loss += substep_percolation_loss;
            lane_substep_index += 1.0;
        }

        Ok(Wb18PercolationRoutingResult {
            per_layer_flux,
            percolation_loss,
        })
    }

    fn run_wb18_percolation_substep(
        phase_class: HillslopeKernelPhaseClass,
        layers: &mut Wb18PercolationLayers,
        lane_config: &Wb18PercolationLaneConfig,
        per_layer_flux: &mut [f64],
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let mut substep_percolation_loss = 0.0_f64;
        for layer_index in (0..layers.theta.len()).rev() {
            substep_percolation_loss += Self::route_wb18_percolation_layer(
                phase_class,
                layers,
                lane_config,
                per_layer_flux,
                layer_index,
            )?;
        }
        Ok(substep_percolation_loss)
    }

    fn route_wb18_percolation_layer(
        phase_class: HillslopeKernelPhaseClass,
        layers: &mut Wb18PercolationLayers,
        lane_config: &Wb18PercolationLaneConfig,
        per_layer_flux: &mut [f64],
        layer_index: usize,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let layer_theta = layers.theta[layer_index];
        let layer_fc = layers.field_capacity[layer_index];
        let layer_ul = layers.upper_limit[layer_index];
        let layer_ssc = layers.conductivity[layer_index];
        let layer_count = layers.theta.len();

        let excess = layer_theta - layer_fc;
        if excess <= WB11_ZERO_THRESHOLD {
            return Ok(0.0);
        }

        let stz = layer_theta / layer_ul;
        if !stz.is_finite() || stz < 0.0 {
            let theta_symbol = Self::wb18_perc_state_symbol("theta", layer_index + 1);
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: theta_symbol,
                value: stz,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let is_bottom_layer = layer_index == layer_count - 1;
        let fx = Self::wb18_percolation_layer_fx(
            phase_class,
            layer_index,
            stz,
            layer_fc,
            layer_ul,
            lane_config.daily_lane,
            is_bottom_layer,
        )?;
        let layer_ssc_effective = Self::wb18_effective_layer_conductivity(
            phase_class,
            layers,
            lane_config,
            layer_index,
            layer_ssc,
            is_bottom_layer,
        )?;
        let ks_adjusted = layer_ssc_effective * fx;
        let pei_pre = (WB18_PERC_TIMESTEP_S * ks_adjusted).min(excess);
        let pei_unscaled =
            Self::wb18_layer_pei_unscaled(phase_class, layers, layer_index, pei_pre)?;
        let pei = pei_unscaled / lane_config.lane_substeps;

        let pei_symbol = Self::wb18_perc_flux_symbol(layer_index + 1);
        Self::require_flux_range_for_symbol(
            phase_class,
            &pei_symbol,
            pei,
            Some(0.0),
            Some(excess),
        )?;

        layers.theta[layer_index] -= pei;
        let theta_symbol = Self::wb18_perc_state_symbol("theta", layer_index + 1);
        Self::require_state_range_for_symbol(
            phase_class,
            &theta_symbol,
            layers.theta[layer_index],
            Some(0.0),
            None,
        )?;

        if layer_index < layer_count - 1 {
            layers.theta[layer_index + 1] += pei;
            per_layer_flux[layer_index] += pei;
            Ok(0.0)
        } else {
            per_layer_flux[layer_index] += pei;
            Ok(pei)
        }
    }

    fn wb18_percolation_layer_fx(
        phase_class: HillslopeKernelPhaseClass,
        layer_index: usize,
        stz: f64,
        layer_fc: f64,
        layer_ul: f64,
        daily_lane: bool,
        is_bottom_layer: bool,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let mut fx = if stz < WB18_PERC_SATURATION_THRESHOLD {
            let fc_ul_ratio = layer_fc / layer_ul;
            if !fc_ul_ratio.is_finite() || fc_ul_ratio >= 1.0 {
                let fc_symbol = Self::wb18_perc_state_symbol("fc", layer_index + 1);
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: fc_symbol,
                    value: fc_ul_ratio,
                    minimum: Some(0.0),
                    maximum: Some(1.0),
                });
            }
            // Legacy-authoritative fallback: watbal.for sets hk=0 when FC/UL <= 0.
            let bi = if fc_ul_ratio <= 0.0 {
                0.0
            } else {
                let derived = -WB18_PERC_BI_COEFFICIENT / fc_ul_ratio.log10();
                if !derived.is_finite() || derived < 0.0 {
                    let fc_symbol = Self::wb18_perc_state_symbol("fc", layer_index + 1);
                    return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                        phase_class,
                        symbol: fc_symbol,
                        value: derived,
                        minimum: Some(0.0),
                        maximum: None,
                    });
                }
                derived
            };
            stz.powf(bi).max(WB18_PERC_MIN_FX)
        } else {
            1.0
        };
        if !daily_lane && is_bottom_layer {
            fx = 1.0;
        }
        if !fx.is_finite() || fx <= 0.0 {
            let ssc_symbol = Self::wb18_perc_state_symbol("ssc", layer_index + 1);
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: ssc_symbol,
                value: fx,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }
        Ok(fx)
    }

    fn wb18_effective_layer_conductivity(
        phase_class: HillslopeKernelPhaseClass,
        layers: &Wb18PercolationLayers,
        lane_config: &Wb18PercolationLaneConfig,
        layer_index: usize,
        layer_ssc: f64,
        is_bottom_layer: bool,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        if !(lane_config.restrictive_layer_enabled && is_bottom_layer) {
            return Ok(layer_ssc);
        }

        if lane_config.daily_lane {
            let denominator = layer_ssc + lane_config.restrictive_layer_conductivity;
            if denominator <= WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: lane_config.restrictive_layer_conductivity_symbol.clone(),
                    value: denominator,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: None,
                });
            }
            let harmonic_mean =
                (2.0 * layer_ssc * lane_config.restrictive_layer_conductivity) / denominator;
            if !harmonic_mean.is_finite() || harmonic_mean <= WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: lane_config.restrictive_layer_conductivity_symbol.clone(),
                    value: harmonic_mean,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: None,
                });
            }
            Ok(harmonic_mean)
        } else {
            let denominator = (layers.depth[layer_index] / layer_ssc)
                + (lane_config.restrictive_layer_thickness
                    / lane_config.restrictive_layer_conductivity);
            if denominator <= WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: lane_config.restrictive_layer_thickness_symbol.clone(),
                    value: denominator,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: None,
                });
            }
            let thickness_weighted =
                (layers.depth[layer_index] + lane_config.restrictive_layer_thickness)
                    / denominator;
            if !thickness_weighted.is_finite() || thickness_weighted <= WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: lane_config.restrictive_layer_thickness_symbol.clone(),
                    value: thickness_weighted,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: None,
                });
            }
            Ok(thickness_weighted)
        }
    }

    fn wb18_layer_pei_unscaled(
        phase_class: HillslopeKernelPhaseClass,
        layers: &Wb18PercolationLayers,
        layer_index: usize,
        pei_pre: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        if layer_index >= layers.theta.len() - 1 {
            return Ok(pei_pre);
        }
        let lower_ratio = layers.theta[layer_index + 1] / layers.upper_limit[layer_index + 1];
        if !lower_ratio.is_finite() || lower_ratio < 0.0 {
            let lower_theta_symbol = Self::wb18_perc_state_symbol("theta", layer_index + 2);
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: lower_theta_symbol,
                value: lower_ratio,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        let lower_ratio_clamped = lower_ratio.min(WB18_PERC_SATURATION_THRESHOLD);
        let lower_factor = (1.0 - lower_ratio_clamped).sqrt();
        Ok(pei_pre * lower_factor)
    }

    fn canonicalize_wb18_deep_percolation_roundoff(
        layers: &mut Wb18PercolationLayers,
        routing: &mut Wb18PercolationRoutingResult,
    ) {
        if (0.0..=WB18_DEEP_PERCOLATION_ROUNDOFF_TOLERANCE_M)
            .contains(&routing.percolation_loss)
        {
            if routing.percolation_loss > 0.0 {
                let bottom_index = layers.theta.len() - 1;
                layers.theta[bottom_index] += routing.percolation_loss;
                routing.per_layer_flux[bottom_index] =
                    (routing.per_layer_flux[bottom_index] - routing.percolation_loss).max(0.0);
            }
            routing.percolation_loss = 0.0;
        }
    }

    fn resolve_wb18_percolation_soil_water_after(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        layers: &mut Wb18PercolationLayers,
        ledger: &Wb18PercolationSoilWaterLedger,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let mut computed_soil_water_after =
            Self::wb18_aggregate_soil_water_after_percolation(request, phase_class, &layers.theta)?
                .max(0.0);
        let preserve_scalar_ledger = !ledger.reconcile_legacy_soil_water_from_layers
            && (ledger.soil_water.max(0.0) - ledger.computed_soil_water_before).abs()
                <= WB18_STORAGE_ROUNDOFF_TOLERANCE_M;
        let soil_water_after = if preserve_scalar_ledger {
            let ledger_soil_water_after = ledger.soil_water.max(0.0)
                + ledger.same_pass_infiltration_depth
                - ledger.percolation_loss;
            if ledger_soil_water_after < -WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB11_SYMBOL_SOIL_WATER),
                    value: ledger_soil_water_after,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }
            ledger_soil_water_after.max(0.0)
        } else {
            computed_soil_water_after
        };
        if preserve_scalar_ledger {
            let storage_roundoff_delta_m = soil_water_after - computed_soil_water_after;
            if storage_roundoff_delta_m.abs() <= WB18_STORAGE_ROUNDOFF_TOLERANCE_M {
                Self::apply_wb18_storage_roundoff_delta_to_layer_storage(
                    phase_class,
                    &mut layers.theta,
                    storage_roundoff_delta_m,
                )?;
                computed_soil_water_after =
                    Self::wb18_aggregate_soil_water_after_percolation(
                        request,
                        phase_class,
                        &layers.theta,
                    )?
                    .max(0.0);
            }
            if (computed_soil_water_after - soil_water_after).abs()
                > WB18_STORAGE_ROUNDOFF_TOLERANCE_M
            {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB11_SYMBOL_SOIL_WATER),
                    value: computed_soil_water_after,
                    minimum: Some(soil_water_after - WB18_STORAGE_ROUNDOFF_TOLERANCE_M),
                    maximum: Some(soil_water_after + WB18_STORAGE_ROUNDOFF_TOLERANCE_M),
                });
            }
        }
        Ok(soil_water_after)
    }

    fn build_wb18_percolation_response(
        soil_water_after: f64,
        layers: &Wb18PercolationLayers,
        routing: &Wb18PercolationRoutingResult,
        same_pass_infiltration: &Wb18SamePassInfiltration,
    ) -> KernelRunResponse {
        let Ok(status) =
            SimulationStatus::ok(SimulationPhase::HillslopeKernel, "HKERNEL-WB11-PERC-OK-001")
        else {
            unreachable!("status message ids are non-empty WB11 constants")
        };
        let mut state_updates = Vec::with_capacity(layers.theta.len() + 1);
        state_updates.push(WritebackField::bounded(
            WB11_SYMBOL_SOIL_WATER,
            soil_water_after,
            Some(0.0),
            None,
        ));
        if let Some(infiltration) = same_pass_infiltration.depth {
            state_updates.push(WritebackField::bounded(
                WB12_SYMBOL_INFILTRATION,
                infiltration,
                Some(0.0),
                None,
            ));
            if same_pass_infiltration.lineage {
                state_updates.push(WritebackField::bounded(
                    WB12_SYMBOL_INFILTRATION_SAME_PASS_LINEAGE,
                    1.0,
                    Some(0.0),
                    Some(1.0),
                ));
            }
        }
        for (index, value) in layers.theta.iter().enumerate() {
            state_updates.push(WritebackField::bounded(
                Self::wb18_perc_state_symbol("theta", index + 1),
                *value,
                Some(0.0),
                None,
            ));
        }

        let mut flux_updates = Vec::with_capacity(layers.theta.len() + 2);
        for (index, value) in routing.per_layer_flux.iter().enumerate() {
            flux_updates.push(WritebackField::bounded(
                Self::wb18_perc_flux_symbol(index + 1),
                *value,
                Some(0.0),
                None,
            ));
        }
        flux_updates.push(WritebackField::bounded(
            WB11_SYMBOL_PERC_LOSS_D,
            routing.percolation_loss,
            Some(0.0),
            None,
        ));
        flux_updates.push(WritebackField::bounded(
            WB11_SYMBOL_PERC_RECHARGE_PE,
            routing.percolation_loss,
            Some(0.0),
            None,
        ));

        let writeback = KernelWritebackPayload::with_updates(state_updates, flux_updates);
        KernelRunResponse::new(status, writeback)
    }


}
