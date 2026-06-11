#[allow(clippy::wildcard_imports)]
use super::super::*;

#[derive(Debug, Clone)]
struct FrostLayerWaterState {
    layer_index: usize,
    fine_layer_count: usize,
    fine_layer_thickness_m: f64,
    dg_m: f64,
    thetdr: f64,
    theta_m: f64,
    upper_limit_m: f64,
    frozen_depth_m: f64,
    frzw_m: f64,
}

#[derive(Debug, Clone)]
struct FrostFineLayerState {
    layer_index: usize,
    fine_index: usize,
    fine_layer_thickness_m: f64,
    fgfrst: f64,
    slfsd_m: f64,
    slsic_m: f64,
    slsw_theta: f64,
    sltime_s: f64,
}

#[derive(Debug, Clone)]
struct FrostLayerExchangeState {
    layer_index: usize,
    thetdr: f64,
    st_m: f64,
    yst_m: f64,
    nwfrzz_m: f64,
    frozen_m: f64,
    frzw_m: f64,
    soilf_m: f64,
    soil_water_m: f64,
}

#[derive(Debug, Clone)]
struct FrostFineShadowState {
    fine_layers: Vec<FrostFineLayerState>,
    layer_state: Vec<FrostLayerExchangeState>,
    total_water_before_m: f64,
    total_water_after_m: f64,
    wb_delta_m: f64,
    residual_m: f64,
}

#[derive(Debug, Clone, Copy)]
struct FrostDepthSummary {
    frdp: f64,
    thdp: f64,
    tfrdp: f64,
    tthawd: f64,
}

impl Wb11HydrologyKernel {
    pub(crate) fn interval_overlap_duration(
        interval_start: f64,
        interval_end: f64,
        active_duration: f64,
    ) -> f64 {
        if active_duration <= 0.0 {
            return 0.0;
        }
        let overlap_start = interval_start.max(0.0);
        let overlap_end = interval_end.min(active_duration);
        (overlap_end - overlap_start).max(0.0)
    }

    pub(crate) fn bounded_interval_overlap_duration(
        interval_start: f64,
        interval_end: f64,
        active_start: f64,
        active_end: f64,
    ) -> f64 {
        if active_end <= active_start {
            return 0.0;
        }
        let overlap_start = interval_start.max(active_start);
        let overlap_end = interval_end.min(active_end);
        (overlap_end - overlap_start).max(0.0)
    }

    fn frost_layer_soilf_sum(layers: &[FrostLayerWaterState]) -> f64 {
        layers
            .iter()
            .map(|layer| layer.frzw_m + layer.thetdr * layer.frozen_depth_m)
            .sum()
    }

    fn fine_layer_total_water(fine_layers: &[FrostFineLayerState], nwfrzz_m: f64) -> f64 {
        fine_layers
            .iter()
            .map(|fine| {
                let unfrozen_depth_m =
                    (fine.fine_layer_thickness_m - fine.slfsd_m).max(0.0);
                fine.slsic_m + fine.slsw_theta * unfrozen_depth_m
            })
            .sum::<f64>()
            + nwfrzz_m
    }

    fn default_fine_layer_from_coarse(
        layer: &FrostLayerWaterState,
        fine_index: usize,
        remaining_frozen_depth_m: &mut f64,
    ) -> FrostFineLayerState {
        let slfsd_m = remaining_frozen_depth_m
            .min(layer.fine_layer_thickness_m)
            .max(0.0);
        *remaining_frozen_depth_m = (*remaining_frozen_depth_m - slfsd_m).max(0.0);
        let fgfrst = if slfsd_m >= layer.fine_layer_thickness_m - WB11_ZERO_THRESHOLD {
            1.0
        } else if slfsd_m > WB11_ZERO_THRESHOLD {
            2.0
        } else {
            0.0
        };
        let soilf_m = layer.frzw_m + layer.thetdr * layer.frozen_depth_m;
        let ice_per_frozen_m = if layer.frozen_depth_m > WB11_ZERO_THRESHOLD {
            soilf_m / layer.frozen_depth_m
        } else {
            0.0
        };
        let unfrozen_depth_m = (layer.dg_m - layer.frozen_depth_m).max(0.0);
        let slsw_theta = if unfrozen_depth_m > WB11_ZERO_THRESHOLD {
            layer.thetdr + layer.theta_m / unfrozen_depth_m
        } else {
            layer.thetdr
        };

        FrostFineLayerState {
            layer_index: layer.layer_index,
            fine_index,
            fine_layer_thickness_m: layer.fine_layer_thickness_m,
            fgfrst,
            slfsd_m,
            slsic_m: ice_per_frozen_m * slfsd_m,
            slsw_theta,
            sltime_s: 0.0,
        }
    }

    fn require_shadow_fine_state_domains(
        phase_class: HillslopeKernelPhaseClass,
        fine: &FrostFineLayerState,
        thetdr: f64,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        Self::require_dynamic_state_range(
            phase_class,
            Self::frost_fine_layer_symbol(
                FROST_RUNTIME_FINE_FGFRST_ROOT,
                fine.layer_index,
                fine.fine_index,
            ),
            fine.fgfrst,
            Some(0.0),
            Some(3.0),
        )?;
        let rounded = fine.fgfrst.round();
        if (fine.fgfrst - rounded).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: Self::frost_fine_layer_symbol(
                    FROST_RUNTIME_FINE_FGFRST_ROOT,
                    fine.layer_index,
                    fine.fine_index,
                ),
                value: fine.fgfrst,
                minimum: Some(0.0),
                maximum: Some(3.0),
            });
        }
        Self::require_dynamic_state_range(
            phase_class,
            Self::frost_fine_layer_symbol(
                FROST_RUNTIME_FINE_SLFSD_M_ROOT,
                fine.layer_index,
                fine.fine_index,
            ),
            fine.slfsd_m,
            Some(0.0),
            Some(fine.fine_layer_thickness_m),
        )?;
        Self::require_dynamic_state_range(
            phase_class,
            Self::frost_fine_layer_symbol(
                FROST_RUNTIME_FINE_SLSIC_M_ROOT,
                fine.layer_index,
                fine.fine_index,
            ),
            fine.slsic_m,
            Some(0.0),
            None,
        )?;
        Self::require_dynamic_state_range(
            phase_class,
            Self::frost_fine_layer_symbol(
                FROST_RUNTIME_FINE_SLSW_THETA_ROOT,
                fine.layer_index,
                fine.fine_index,
            ),
            fine.slsw_theta,
            Some(thetdr),
            None,
        )?;
        Self::require_dynamic_state_range(
            phase_class,
            Self::frost_fine_layer_symbol(
                FROST_RUNTIME_FINE_SLTIME_S_ROOT,
                fine.layer_index,
                fine.fine_index,
            ),
            fine.sltime_s,
            Some(0.0),
            Some(FROST_RUNTIME_SECONDS_PER_HOUR),
        )
    }

    fn read_or_default_shadow_fine_state(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        layer: &FrostLayerWaterState,
    ) -> Result<Vec<FrostFineLayerState>, Wb11HydrologyKernelGuardError> {
        let mut remaining_frozen_depth_m = layer.frozen_depth_m;
        let mut fine_layers = Vec::with_capacity(layer.fine_layer_count);
        for fine_index in 1..=layer.fine_layer_count {
            let default =
                Self::default_fine_layer_from_coarse(layer, fine_index, &mut remaining_frozen_depth_m);
            let fgfrst_symbol = Self::frost_fine_layer_symbol(
                FROST_RUNTIME_FINE_FGFRST_ROOT,
                layer.layer_index,
                fine_index,
            );
            let slfsd_symbol = Self::frost_fine_layer_symbol(
                FROST_RUNTIME_FINE_SLFSD_M_ROOT,
                layer.layer_index,
                fine_index,
            );
            let slsic_symbol = Self::frost_fine_layer_symbol(
                FROST_RUNTIME_FINE_SLSIC_M_ROOT,
                layer.layer_index,
                fine_index,
            );
            let slsw_symbol = Self::frost_fine_layer_symbol(
                FROST_RUNTIME_FINE_SLSW_THETA_ROOT,
                layer.layer_index,
                fine_index,
            );
            let sltime_symbol = Self::frost_fine_layer_symbol(
                FROST_RUNTIME_FINE_SLTIME_S_ROOT,
                layer.layer_index,
                fine_index,
            );

            let fine = FrostFineLayerState {
                layer_index: layer.layer_index,
                fine_index,
                fine_layer_thickness_m: layer.fine_layer_thickness_m,
                fgfrst: Self::optional_state_scalar_for_symbol(
                    request,
                    phase_class,
                    &fgfrst_symbol,
                )?
                .unwrap_or(default.fgfrst),
                slfsd_m: Self::optional_state_scalar_for_symbol(
                    request,
                    phase_class,
                    &slfsd_symbol,
                )?
                .unwrap_or(default.slfsd_m),
                slsic_m: Self::optional_state_scalar_for_symbol(
                    request,
                    phase_class,
                    &slsic_symbol,
                )?
                .unwrap_or(default.slsic_m),
                slsw_theta: Self::optional_state_scalar_for_symbol(
                    request,
                    phase_class,
                    &slsw_symbol,
                )?
                .unwrap_or(default.slsw_theta),
                sltime_s: Self::optional_state_scalar_for_symbol(
                    request,
                    phase_class,
                    &sltime_symbol,
                )?
                .unwrap_or(0.0),
            };
            Self::require_shadow_fine_state_domains(phase_class, &fine, layer.thetdr)?;
            fine_layers.push(fine);
        }
        Ok(fine_layers)
    }

    fn apply_shadow_frwatc_ingress(
        fine_layers: &mut [FrostFineLayerState],
        layer: &mut FrostLayerExchangeState,
    ) {
        let mut remaining_delta_m = layer.st_m - layer.yst_m;
        if remaining_delta_m > WB11_ZERO_THRESHOLD {
            if layer.frozen_m > WB11_ZERO_THRESHOLD {
                let frozen_zone_capacity_m =
                    (layer.soilf_m + (layer.st_m - layer.nwfrzz_m).max(0.0)
                        - layer.nwfrzz_m)
                        .max(0.0);
                let into_frozen_zone_m = remaining_delta_m.min(frozen_zone_capacity_m);
                layer.nwfrzz_m += into_frozen_zone_m;
                remaining_delta_m -= into_frozen_zone_m;
            }
            let unfrozen_depth_m = fine_layers
                .iter()
                .map(|fine| (fine.fine_layer_thickness_m - fine.slfsd_m).max(0.0))
                .sum::<f64>();
            if remaining_delta_m > WB11_ZERO_THRESHOLD
                && unfrozen_depth_m > WB11_ZERO_THRESHOLD
            {
                let theta_increment = remaining_delta_m / unfrozen_depth_m;
                for fine in fine_layers {
                    if fine.fine_layer_thickness_m > fine.slfsd_m + WB11_ZERO_THRESHOLD {
                        fine.slsw_theta += theta_increment;
                    }
                }
            }
        } else if remaining_delta_m < -WB11_ZERO_THRESHOLD {
            let mut remaining_drain_m = -remaining_delta_m;
            let drain_nwfrzz_m = layer.nwfrzz_m.min(remaining_drain_m);
            layer.nwfrzz_m -= drain_nwfrzz_m;
            remaining_drain_m -= drain_nwfrzz_m;
            let available_liquid_m = fine_layers
                .iter()
                .map(|fine| {
                    let unfrozen_depth_m =
                        (fine.fine_layer_thickness_m - fine.slfsd_m).max(0.0);
                    (fine.slsw_theta - layer.thetdr).max(0.0) * unfrozen_depth_m
                })
                .sum::<f64>();
            if remaining_drain_m > WB11_ZERO_THRESHOLD
                && available_liquid_m > WB11_ZERO_THRESHOLD
            {
                let drain_fraction = (remaining_drain_m / available_liquid_m).min(1.0);
                for fine in fine_layers {
                    let unfrozen_depth_m =
                        (fine.fine_layer_thickness_m - fine.slfsd_m).max(0.0);
                    if unfrozen_depth_m > WB11_ZERO_THRESHOLD {
                        let available_m =
                            (fine.slsw_theta - layer.thetdr).max(0.0) * unfrozen_depth_m;
                        fine.slsw_theta -= (available_m * drain_fraction) / unfrozen_depth_m;
                        fine.slsw_theta = fine.slsw_theta.max(layer.thetdr);
                    }
                }
            }
        }
    }

    fn aggregate_shadow_layer(
        fine_layers: &[FrostFineLayerState],
        layer: &mut FrostLayerExchangeState,
    ) {
        let mut frozen_m = 0.0;
        let mut soilf_m = 0.0;
        let mut soil_water_m = 0.0;
        let mut st_m = 0.0;
        for fine in fine_layers {
            let unfrozen_depth_m = (fine.fine_layer_thickness_m - fine.slfsd_m).max(0.0);
            frozen_m += fine.slfsd_m;
            soilf_m += fine.slsic_m;
            soil_water_m += fine.slsw_theta * unfrozen_depth_m;
            st_m += (fine.slsw_theta - layer.thetdr).max(0.0) * unfrozen_depth_m;
        }
        if frozen_m < 0.001 {
            layer.nwfrzz_m = 0.0;
            soilf_m = 0.0;
        }
        layer.frozen_m = frozen_m;
        layer.soilf_m = soilf_m;
        layer.frzw_m = (soilf_m - layer.thetdr * frozen_m).max(0.0);
        layer.soil_water_m = soil_water_m + layer.nwfrzz_m;
        layer.st_m = st_m + layer.nwfrzz_m;
        layer.yst_m = layer.st_m;
    }

    fn compute_shadow_fine_state(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        layers: &[FrostLayerWaterState],
    ) -> Result<FrostFineShadowState, Wb11HydrologyKernelGuardError> {
        let mut all_fine_layers = Vec::new();
        let mut shadow_layers = Vec::with_capacity(layers.len());
        let mut total_water_before_m = 0.0;
        let mut total_water_after_m = 0.0;
        let mut wb_delta_m = 0.0;

        for layer in layers {
            let mut fine_layers =
                Self::read_or_default_shadow_fine_state(request, phase_class, layer)?;
            let yst_symbol = Self::frost_layer_symbol(FROST_RUNTIME_LAYER_YST_M_ROOT, layer.layer_index);
            let nwfrzz_symbol =
                Self::frost_layer_symbol(FROST_RUNTIME_LAYER_NWFRZZ_M_ROOT, layer.layer_index);
            let nwfrzz_m =
                Self::optional_state_scalar_for_symbol(request, phase_class, &nwfrzz_symbol)?
                    .unwrap_or(0.0);
            Self::require_dynamic_state_range(
                phase_class,
                nwfrzz_symbol,
                nwfrzz_m,
                Some(0.0),
                None,
            )?;
            let st_m = layer.theta_m + nwfrzz_m;
            let yst_m = Self::optional_state_scalar_for_symbol(request, phase_class, &yst_symbol)?
                .unwrap_or(st_m);
            Self::require_dynamic_state_range(
                phase_class,
                yst_symbol,
                yst_m,
                Some(0.0),
                None,
            )?;
            let soilf_m = layer.frzw_m + layer.thetdr * layer.frozen_depth_m;
            let mut shadow_layer = FrostLayerExchangeState {
                layer_index: layer.layer_index,
                thetdr: layer.thetdr,
                st_m,
                yst_m,
                nwfrzz_m,
                frozen_m: layer.frozen_depth_m,
                frzw_m: layer.frzw_m,
                soilf_m,
                soil_water_m: layer.theta_m
                    + layer.thetdr * (layer.dg_m - layer.frozen_depth_m).max(0.0)
                    + nwfrzz_m,
            };
            let before_m = Self::fine_layer_total_water(&fine_layers, shadow_layer.nwfrzz_m);
            wb_delta_m += shadow_layer.st_m - shadow_layer.yst_m;
            Self::apply_shadow_frwatc_ingress(&mut fine_layers, &mut shadow_layer);
            Self::aggregate_shadow_layer(&fine_layers, &mut shadow_layer);
            let after_m = Self::fine_layer_total_water(&fine_layers, shadow_layer.nwfrzz_m);
            total_water_before_m += before_m;
            total_water_after_m += after_m;
            shadow_layers.push(shadow_layer);
            all_fine_layers.extend(fine_layers);
        }

        Ok(FrostFineShadowState {
            fine_layers: all_fine_layers,
            layer_state: shadow_layers,
            total_water_before_m,
            total_water_after_m,
            wb_delta_m,
            residual_m: total_water_after_m - total_water_before_m - wb_delta_m,
        })
    }

    fn push_frost_segment(segments: &mut Vec<(bool, f64)>, frozen: bool, length_m: f64) {
        if length_m <= WB11_ZERO_THRESHOLD {
            return;
        }
        if let Some((last_frozen, last_length_m)) = segments.last_mut()
            && *last_frozen == frozen
        {
            *last_length_m += length_m;
            return;
        }
        segments.push((frozen, length_m));
    }

    fn derived_frost_depths_from_fine_state(
        fine_layers: &[FrostFineLayerState],
    ) -> FrostDepthSummary {
        let mut segments = Vec::new();
        for fine in fine_layers {
            let frozen_m = fine
                .slfsd_m
                .clamp(0.0, fine.fine_layer_thickness_m);
            let thawed_m = (fine.fine_layer_thickness_m - frozen_m).max(0.0);
            let flag = fine.fgfrst.round();
            if frozen_m <= WB11_ZERO_THRESHOLD {
                Self::push_frost_segment(&mut segments, false, fine.fine_layer_thickness_m);
            } else if frozen_m >= fine.fine_layer_thickness_m - WB11_ZERO_THRESHOLD {
                Self::push_frost_segment(&mut segments, true, fine.fine_layer_thickness_m);
            } else if (flag - 3.0).abs() <= WB11_ZERO_THRESHOLD {
                Self::push_frost_segment(&mut segments, false, thawed_m);
                Self::push_frost_segment(&mut segments, true, frozen_m);
            } else {
                Self::push_frost_segment(&mut segments, true, frozen_m);
                Self::push_frost_segment(&mut segments, false, thawed_m);
            }
        }

        let mut cursor_m = 0.0;
        let mut index = 0usize;
        while index < segments.len() && !segments[index].0 {
            cursor_m += segments[index].1;
            index += 1;
        }
        if index >= segments.len() {
            return FrostDepthSummary {
                frdp: 0.0,
                thdp: 0.0,
                tfrdp: 0.0,
                tthawd: 0.0,
            };
        }

        let thdp_m = cursor_m;
        let mut top_frozen_m = 0.0;
        while index < segments.len() && segments[index].0 {
            top_frozen_m += segments[index].1;
            cursor_m += segments[index].1;
            index += 1;
        }

        let mut top_thawed_m = 0.0;
        while index < segments.len() && !segments[index].0 {
            top_thawed_m += segments[index].1;
            cursor_m += segments[index].1;
            index += 1;
        }

        let mut bottom_frost_depth_m = thdp_m + top_frozen_m;
        let has_sandwich = segments[index..].iter().any(|(frozen, _)| *frozen);
        while index < segments.len() {
            cursor_m += segments[index].1;
            if segments[index].0 {
                bottom_frost_depth_m = cursor_m;
            }
            index += 1;
        }

        if has_sandwich {
            let tfrdp_m = thdp_m + top_frozen_m;
            FrostDepthSummary {
                frdp: bottom_frost_depth_m,
                thdp: thdp_m,
                tfrdp: tfrdp_m,
                tthawd: tfrdp_m + top_thawed_m,
            }
        } else {
            FrostDepthSummary {
                frdp: thdp_m + top_frozen_m,
                thdp: thdp_m,
                tfrdp: 0.0,
                tthawd: 0.0,
            }
        }
    }

    fn refresh_fine_frost_flag(fine: &mut FrostFineLayerState) {
        fine.slfsd_m = fine
            .slfsd_m
            .clamp(0.0, fine.fine_layer_thickness_m);
        if fine.slfsd_m <= WB11_ZERO_THRESHOLD {
            fine.slfsd_m = 0.0;
            fine.slsic_m = 0.0;
            fine.fgfrst = 0.0;
        } else if fine.slfsd_m >= fine.fine_layer_thickness_m - WB11_ZERO_THRESHOLD {
            fine.slfsd_m = fine.fine_layer_thickness_m;
            fine.fgfrst = 1.0;
        } else if (fine.fgfrst.round() - 3.0).abs() <= WB11_ZERO_THRESHOLD {
            fine.fgfrst = 3.0;
        } else {
            fine.fgfrst = 2.0;
        }
    }

    fn aggregate_active_layers_from_fine_state(
        fine_layers: &[FrostFineLayerState],
        exchange_layers: &mut [FrostLayerExchangeState],
        water_layers: &mut [FrostLayerWaterState],
    ) {
        for water_layer in water_layers {
            let mut frozen_m = 0.0;
            let mut soilf_m = 0.0;
            let mut active_liquid_m = 0.0;
            for fine in fine_layers
                .iter()
                .filter(|fine| fine.layer_index == water_layer.layer_index)
            {
                let unfrozen_depth_m =
                    (fine.fine_layer_thickness_m - fine.slfsd_m).max(0.0);
                frozen_m += fine.slfsd_m;
                soilf_m += fine.slsic_m;
                active_liquid_m +=
                    (fine.slsw_theta - water_layer.thetdr).max(0.0) * unfrozen_depth_m;
            }
            let nwfrzz_m = exchange_layers
                .iter()
                .find(|layer| layer.layer_index == water_layer.layer_index)
                .map_or(0.0, |layer| layer.nwfrzz_m);
            water_layer.frozen_depth_m = frozen_m.clamp(0.0, water_layer.dg_m);
            water_layer.frzw_m =
                (soilf_m - water_layer.thetdr * water_layer.frozen_depth_m).max(0.0);
            water_layer.theta_m = active_liquid_m + nwfrzz_m;

            if let Some(exchange_layer) = exchange_layers
                .iter_mut()
                .find(|layer| layer.layer_index == water_layer.layer_index)
            {
                exchange_layer.frozen_m = water_layer.frozen_depth_m;
                exchange_layer.soilf_m = soilf_m;
                exchange_layer.frzw_m = water_layer.frzw_m;
                exchange_layer.st_m = water_layer.theta_m;
                exchange_layer.yst_m = exchange_layer.st_m;
                exchange_layer.soil_water_m = active_liquid_m
                    + water_layer.thetdr * (water_layer.dg_m - water_layer.frozen_depth_m).max(0.0)
                    + nwfrzz_m;
            }
        }
    }

    fn refreeze_frozen_zone_liquid(
        fine_layers: &mut [FrostFineLayerState],
        exchange_layers: &mut [FrostLayerExchangeState],
        water_layers: &[FrostLayerWaterState],
        mut energy_j_m2: f64,
    ) -> f64 {
        for exchange_layer in exchange_layers {
            if energy_j_m2 <= WB11_ZERO_THRESHOLD
                || exchange_layer.nwfrzz_m <= WB11_ZERO_THRESHOLD
            {
                continue;
            }
            let Some(water_layer) = water_layers
                .iter()
                .find(|layer| layer.layer_index == exchange_layer.layer_index)
            else {
                continue;
            };
            for fine in fine_layers
                .iter_mut()
                .filter(|fine| fine.layer_index == exchange_layer.layer_index)
            {
                if energy_j_m2 <= WB11_ZERO_THRESHOLD
                    || exchange_layer.nwfrzz_m <= WB11_ZERO_THRESHOLD
                {
                    break;
                }
                if fine.slfsd_m <= WB11_ZERO_THRESHOLD {
                    continue;
                }
                let layer_capacity_m =
                    water_layer.upper_limit_m / water_layer.dg_m * fine.slfsd_m;
                let fine_capacity_m = (layer_capacity_m - fine.slsic_m).max(0.0);
                if fine_capacity_m <= WB11_ZERO_THRESHOLD {
                    continue;
                }
                let energy_limited_m = energy_j_m2 / FROST_RUNTIME_LATENT_HEAT_WATER_J_M3;
                let frozen_m = exchange_layer
                    .nwfrzz_m
                    .min(fine_capacity_m)
                    .min(energy_limited_m);
                if frozen_m <= WB11_ZERO_THRESHOLD {
                    continue;
                }
                fine.slsic_m += frozen_m;
                exchange_layer.nwfrzz_m -= frozen_m;
                energy_j_m2 -= frozen_m * FROST_RUNTIME_LATENT_HEAT_WATER_J_M3;
            }
            if exchange_layer.nwfrzz_m <= WB11_ZERO_THRESHOLD {
                exchange_layer.nwfrzz_m = 0.0;
            }
        }
        energy_j_m2
    }

    fn freeze_fine_front(
        fine_layers: &mut [FrostFineLayerState],
        water_layers: &[FrostLayerWaterState],
        mut energy_j_m2: f64,
    ) -> f64 {
        for fine in fine_layers {
            if energy_j_m2 <= WB11_ZERO_THRESHOLD {
                break;
            }
            let Some(water_layer) = water_layers
                .iter()
                .find(|layer| layer.layer_index == fine.layer_index)
            else {
                continue;
            };
            let unfrozen_depth_m =
                (fine.fine_layer_thickness_m - fine.slfsd_m).max(0.0);
            if unfrozen_depth_m <= WB11_ZERO_THRESHOLD {
                Self::refresh_fine_frost_flag(fine);
                continue;
            }
            let water_per_m = fine.slsw_theta.max(water_layer.thetdr);
            if water_per_m <= WB11_ZERO_THRESHOLD {
                continue;
            }
            let energy_limited_depth_m =
                energy_j_m2 / (FROST_RUNTIME_LATENT_HEAT_WATER_J_M3 * water_per_m);
            let freeze_depth_m = unfrozen_depth_m.min(energy_limited_depth_m);
            if freeze_depth_m <= WB11_ZERO_THRESHOLD {
                continue;
            }
            fine.slsic_m += water_per_m * freeze_depth_m;
            fine.slfsd_m += freeze_depth_m;
            energy_j_m2 -=
                FROST_RUNTIME_LATENT_HEAT_WATER_J_M3 * water_per_m * freeze_depth_m;
            Self::refresh_fine_frost_flag(fine);
        }
        energy_j_m2
    }

    fn thaw_fine_bottom(
        fine_layers: &mut [FrostFineLayerState],
        mut energy_j_m2: f64,
    ) -> f64 {
        for fine in fine_layers.iter_mut().rev() {
            if energy_j_m2 <= WB11_ZERO_THRESHOLD {
                break;
            }
            if fine.slfsd_m <= WB11_ZERO_THRESHOLD || fine.slsic_m <= WB11_ZERO_THRESHOLD {
                Self::refresh_fine_frost_flag(fine);
                continue;
            }
            let ice_per_m = fine.slsic_m / fine.slfsd_m;
            if ice_per_m <= WB11_ZERO_THRESHOLD {
                continue;
            }
            let energy_limited_depth_m =
                energy_j_m2 / (FROST_RUNTIME_LATENT_HEAT_WATER_J_M3 * ice_per_m);
            let thaw_depth_m = fine.slfsd_m.min(energy_limited_depth_m);
            if thaw_depth_m <= WB11_ZERO_THRESHOLD {
                continue;
            }
            let old_unfrozen_m = (fine.fine_layer_thickness_m - fine.slfsd_m).max(0.0);
            let old_liquid_m = fine.slsw_theta * old_unfrozen_m;
            let melted_m = ice_per_m * thaw_depth_m;
            fine.slsic_m = (fine.slsic_m - melted_m).max(0.0);
            fine.slfsd_m = (fine.slfsd_m - thaw_depth_m).max(0.0);
            let new_unfrozen_m = (fine.fine_layer_thickness_m - fine.slfsd_m).max(0.0);
            if new_unfrozen_m > WB11_ZERO_THRESHOLD {
                fine.slsw_theta = (old_liquid_m + melted_m) / new_unfrozen_m;
            }
            energy_j_m2 -= FROST_RUNTIME_LATENT_HEAT_WATER_J_M3 * melted_m;
            Self::refresh_fine_frost_flag(fine);
        }
        energy_j_m2
    }

    fn reset_fine_layer_hour_timers(fine_layers: &mut [FrostFineLayerState]) {
        for fine in fine_layers {
            fine.sltime_s = 0.0;
        }
    }

    fn select_frost_branch(
        signed_surface_flux_w_m2: f64,
        lower_front_heat_w_m2: f64,
        signed_net_flux_w_m2: f64,
        depth_summary: FrostDepthSummary,
    ) -> f64 {
        let sandwich_active = depth_summary.tthawd > 0.001 || depth_summary.thdp > 0.001;
        if sandwich_active {
            if signed_surface_flux_w_m2 < -WB11_ZERO_THRESHOLD {
                2.0
            } else if signed_surface_flux_w_m2 > WB11_ZERO_THRESHOLD {
                3.0
            } else if lower_front_heat_w_m2 > WB11_ZERO_THRESHOLD {
                4.0
            } else {
                0.0
            }
        } else if depth_summary.frdp <= WB11_ZERO_THRESHOLD {
            if signed_net_flux_w_m2 < -WB11_ZERO_THRESHOLD {
                1.0
            } else {
                0.0
            }
        } else if signed_surface_flux_w_m2 > WB11_ZERO_THRESHOLD {
            3.0
        } else if signed_net_flux_w_m2 < -WB11_ZERO_THRESHOLD {
            1.0
        } else if lower_front_heat_w_m2 > WB11_ZERO_THRESHOLD {
            4.0
        } else {
            0.0
        }
    }

    pub(crate) fn resolve_active_snow_coupling(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<bool, Wb11HydrologyKernelGuardError> {
        let key = BoundarySymbol::from(WB14_SYMBOL_SNOW_FILE_PRESENT);
        if let Some(value) = request.state_surface.get(&key) {
            let scalar = value.as_f64();
            if !scalar.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                    phase_class,
                    symbol: key,
                    value: scalar,
                });
            }
            if !(-WB11_ZERO_THRESHOLD..=1.0 + WB11_ZERO_THRESHOLD).contains(&scalar) {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB14_SYMBOL_SNOW_FILE_PRESENT),
                    value: scalar,
                    minimum: Some(0.0),
                    maximum: Some(1.0),
                });
            }

            let rounded = scalar.round();
            if (scalar - rounded).abs() > WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB14_SYMBOL_SNOW_FILE_PRESENT),
                    value: scalar,
                    minimum: Some(0.0),
                    maximum: Some(1.0),
                });
            }
        }

        let runtime_swe = Self::validate_runtime_snow_state_domains(request, phase_class)?;

        let tmax = Self::optional_state_scalar(request, phase_class, WB14_SYMBOL_TMAX)?;
        let tmin = Self::optional_state_scalar(request, phase_class, WB14_SYMBOL_TMIN)?;
        let cold_day_active = match (tmax, tmin) {
            (Some(tmax), Some(tmin)) => f64::midpoint(tmax, tmin) < 0.0,
            _ => false,
        };
        let snow_controls_projected = request
            .state_surface
            .contains_key(&BoundarySymbol::from(WB14_SYMBOL_SNOW_RST))
            && request
                .state_surface
                .contains_key(&BoundarySymbol::from(WB14_SYMBOL_SNOW_NEWSNW))
            && request
                .state_surface
                .contains_key(&BoundarySymbol::from(WB14_SYMBOL_SNOW_SSD));

        let active_snow_coupling =
            runtime_swe > WB11_ZERO_THRESHOLD || (cold_day_active && snow_controls_projected);
        Ok(active_snow_coupling)
    }

    pub(crate) fn validate_runtime_snow_state_domains(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let runtime_swe_symbol = BoundarySymbol::from(WB14_SYMBOL_SNOW_RUNTIME_SWE);
        let depth_symbol = BoundarySymbol::from(SNOW_RUNTIME_DEPTH_M_SYMBOL);
        let density_symbol = BoundarySymbol::from(SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL);
        let settle_day_count_symbol = BoundarySymbol::from(SNOW_RUNTIME_SETTLE_DAY_COUNT_SYMBOL);
        let snow_option_symbols = [
            BoundarySymbol::from(WB14_SYMBOL_SNOW_FILE_PRESENT),
            BoundarySymbol::from(WB14_SYMBOL_SNOW_RST),
            BoundarySymbol::from(WB14_SYMBOL_SNOW_NEWSNW),
            BoundarySymbol::from(WB14_SYMBOL_SNOW_SSD),
        ];

        let snow_projection_present = [
            &runtime_swe_symbol,
            &depth_symbol,
            &density_symbol,
            &settle_day_count_symbol,
        ]
        .into_iter()
        .chain(snow_option_symbols.iter())
        .any(|symbol| request.state_surface.contains_key(symbol));
        if !snow_projection_present {
            return Ok(0.0);
        }

        let runtime_swe =
            Self::require_state_scalar_for_symbol(request, phase_class, &runtime_swe_symbol)?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_SNOW_RUNTIME_SWE,
            runtime_swe,
            Some(0.0),
            None,
        )?;

        let runtime_depth_m =
            Self::require_state_scalar_for_symbol(request, phase_class, &depth_symbol)?;
        Self::require_dynamic_state_range(
            phase_class,
            depth_symbol,
            runtime_depth_m,
            Some(0.0),
            None,
        )?;

        let runtime_density_kg_m3 =
            Self::require_state_scalar_for_symbol(request, phase_class, &density_symbol)?;
        Self::require_dynamic_state_range(
            phase_class,
            density_symbol,
            runtime_density_kg_m3,
            Some(0.0),
            Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
        )?;

        let runtime_settle_day_count =
            Self::require_state_scalar_for_symbol(request, phase_class, &settle_day_count_symbol)?;
        Self::require_dynamic_state_range(
            phase_class,
            settle_day_count_symbol,
            runtime_settle_day_count,
            Some(0.0),
            None,
        )?;

        Ok(runtime_swe)
    }

    pub(crate) fn resolve_active_frost_coupling(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<bool, Wb11HydrologyKernelGuardError> {
        if let Some(value) = request
            .state_surface
            .get(&BoundarySymbol::from(WB14_SYMBOL_FROST_FILE_PRESENT))
        {
            let scalar = value.as_f64();
            if !scalar.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                    phase_class,
                    symbol: BoundarySymbol::from(WB14_SYMBOL_FROST_FILE_PRESENT),
                    value: scalar,
                });
            }
            if !(-WB11_ZERO_THRESHOLD..=1.0 + WB11_ZERO_THRESHOLD).contains(&scalar) {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB14_SYMBOL_FROST_FILE_PRESENT),
                    value: scalar,
                    minimum: Some(0.0),
                    maximum: Some(1.0),
                });
            }

            let rounded = scalar.round();
            if (scalar - rounded).abs() > WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB14_SYMBOL_FROST_FILE_PRESENT),
                    value: scalar,
                    minimum: Some(0.0),
                    maximum: Some(1.0),
                });
            }
        }

        let Some(wint_red) =
            Self::optional_state_scalar(request, phase_class, WB14_SYMBOL_FROST_WINT_RED)?
        else {
            return Ok(false);
        };
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_FROST_WINT_RED,
            wint_red,
            Some(0.0),
            Some(1.0),
        )?;
        let wint_rounded = wint_red.round();
        if (wint_red - wint_rounded).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_FROST_WINT_RED),
                value: wint_red,
                minimum: Some(0.0),
                maximum: Some(1.0),
            });
        }

        Ok(wint_rounded >= 1.0 - WB11_ZERO_THRESHOLD)
    }

    #[allow(clippy::too_many_lines)]
    pub(crate) fn compute_active_frost_coupling(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        soil_conductivity: f64,
    ) -> Result<FrostCouplingOutcome, Wb11HydrologyKernelGuardError> {
        let wint_red =
            Self::require_state_scalar(request, phase_class, WB14_SYMBOL_FROST_WINT_RED)?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_FROST_WINT_RED,
            wint_red,
            Some(0.0),
            Some(1.0),
        )?;
        let wint_rounded = wint_red.round();
        if (wint_red - wint_rounded).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_FROST_WINT_RED),
                value: wint_red,
                minimum: Some(0.0),
                maximum: Some(1.0),
            });
        }
        if wint_rounded < 1.0 - WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_FROST_WINT_RED),
                value: wint_red,
                minimum: Some(1.0),
                maximum: Some(1.0),
            });
        }

        let fine_top =
            Self::require_state_scalar(request, phase_class, WB14_SYMBOL_FROST_FINE_TOP)?;
        let fine_bot =
            Self::require_state_scalar(request, phase_class, WB14_SYMBOL_FROST_FINE_BOT)?;
        for (symbol, value) in [
            (WB14_SYMBOL_FROST_FINE_TOP, fine_top),
            (WB14_SYMBOL_FROST_FINE_BOT, fine_bot),
        ] {
            Self::require_state_range(phase_class, symbol, value, Some(1.0), Some(10.0))?;
            let rounded = value.round();
            if (value - rounded).abs() > WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(symbol),
                    value,
                    minimum: Some(1.0),
                    maximum: Some(10.0),
                });
            }
        }

        let ksnowf = Self::require_state_scalar(request, phase_class, WB14_SYMBOL_FROST_KSNOWF)?;
        let kresf = Self::require_state_scalar(request, phase_class, WB14_SYMBOL_FROST_KRESF)?;
        let ksoilf = Self::require_state_scalar(request, phase_class, WB14_SYMBOL_FROST_KSOILF)?;
        for (symbol, value) in [
            (WB14_SYMBOL_FROST_KSNOWF, ksnowf),
            (WB14_SYMBOL_FROST_KRESF, kresf),
            (WB14_SYMBOL_FROST_KSOILF, ksoilf),
        ] {
            Self::require_state_range(phase_class, symbol, value, Some(0.1), Some(10.0))?;
        }

        let kfactor1 =
            Self::require_state_scalar(request, phase_class, WB14_SYMBOL_FROST_KFACTOR1)?;
        let kfactor2 =
            Self::require_state_scalar(request, phase_class, WB14_SYMBOL_FROST_KFACTOR2)?;
        let kfactor3 =
            Self::require_state_scalar(request, phase_class, WB14_SYMBOL_FROST_KFACTOR3)?;
        for (symbol, value) in [
            (WB14_SYMBOL_FROST_KFACTOR1, kfactor1),
            (WB14_SYMBOL_FROST_KFACTOR2, kfactor2),
            (WB14_SYMBOL_FROST_KFACTOR3, kfactor3),
        ] {
            if value <= 0.0 + WB11_ZERO_THRESHOLD || value > 1.0 + WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(symbol),
                    value,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: Some(1.0),
                });
            }
        }

        let fine_top_count = {
            let rounded = fine_top.round();
            let parsed = format!("{rounded:.0}")
                .parse::<usize>()
                .map_err(|_| Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB14_SYMBOL_FROST_FINE_TOP),
                    value: fine_top,
                    minimum: Some(1.0),
                    maximum: Some(10.0),
                })?;
            if !(1..=10).contains(&parsed) {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB14_SYMBOL_FROST_FINE_TOP),
                    value: fine_top,
                    minimum: Some(1.0),
                    maximum: Some(10.0),
                });
            }
            parsed
        };
        let fine_bot_count = {
            let rounded = fine_bot.round();
            let parsed = format!("{rounded:.0}")
                .parse::<usize>()
                .map_err(|_| Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB14_SYMBOL_FROST_FINE_BOT),
                    value: fine_bot,
                    minimum: Some(1.0),
                    maximum: Some(10.0),
                })?;
            if !(1..=10).contains(&parsed) {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB14_SYMBOL_FROST_FINE_BOT),
                    value: fine_bot,
                    minimum: Some(1.0),
                    maximum: Some(10.0),
                });
            }
            parsed
        };

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

        let mut layer_water_state = Vec::with_capacity(layer_count);
        let mut total_fine_layer_count = 0usize;
        for layer_index in 1..=layer_count {
            let (dg_symbol, dg_m) =
                Self::require_wb19_dg_scalar(request, phase_class, layer_index)?;
            Self::require_state_range_for_symbol(
                phase_class,
                &dg_symbol,
                dg_m,
                Some(WB11_ZERO_THRESHOLD),
                None,
            )?;

            let fine_layer_count = if layer_index == layer_count {
                let spacing_mm = if layer_index > 2 {
                    200.0 / Self::diagnostic_count_to_f64(fine_bot_count)
                } else {
                    // UNIT-CONVERSION-ALLOW: cm_m_scale percentage allocation, not dimensional conversion.
                    100.0 / Self::diagnostic_count_to_f64(fine_top_count)
                };
                let dg_mm =
                    openwepp_unit_boundary::conversions::meters_to_millimeters(dg_m).map_err(
                        |error| {
                            Self::unit_conversion_guard_error(
                                phase_class,
                                dg_symbol.clone(),
                                &error,
                            )
                        },
                    )?;
                let dg_mm_trunc = dg_mm.trunc();
                let ratio_trunc = (dg_mm / spacing_mm).trunc();
                let mut count = format!("{ratio_trunc:.0}")
                    .parse::<usize>()
                    .map_err(|_| Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                        phase_class,
                        symbol: dg_symbol.clone(),
                        value: ratio_trunc,
                        minimum: Some(0.0),
                        maximum: Some(Self::diagnostic_count_to_f64(usize::MAX)),
                    })?;
                let count_trunc_mm = (Self::diagnostic_count_to_f64(count) * spacing_mm).trunc();
                if (count_trunc_mm - dg_mm_trunc).abs() > WB11_ZERO_THRESHOLD {
                    count += 1;
                }
                count.max(1)
            } else if layer_index < 3 {
                fine_top_count
            } else {
                fine_bot_count
            };

            total_fine_layer_count += fine_layer_count;
            let theta_symbol = Self::wb18_perc_state_symbol("theta", layer_index);
            let theta_m =
                Self::require_state_scalar_for_symbol(request, phase_class, &theta_symbol)?;
            Self::require_state_range_for_symbol(
                phase_class,
                &theta_symbol,
                theta_m,
                Some(0.0),
                None,
            )?;

            let upper_limit_symbol = Self::wb18_perc_state_symbol("ul", layer_index);
            let upper_limit_m =
                Self::require_state_scalar_for_symbol(request, phase_class, &upper_limit_symbol)?;
            Self::require_state_range_for_symbol(
                phase_class,
                &upper_limit_symbol,
                upper_limit_m,
                Some(0.0),
                None,
            )?;

            let (thetdr_symbol, thetdr) =
                Self::require_wb19_thetdr_scalar(request, phase_class, layer_index)?;
            Self::require_state_range_for_symbol(
                phase_class,
                &thetdr_symbol,
                thetdr,
                Some(0.0),
                Some(1.0),
            )?;

            let frozen_depth_symbol = Self::wb18_perc_state_symbol("frozen_depth", layer_index);
            let frozen_depth_m = Self::optional_state_scalar_for_symbol(
                request,
                phase_class,
                &frozen_depth_symbol,
            )?
            .unwrap_or(0.0);
            Self::require_state_range_for_symbol(
                phase_class,
                &frozen_depth_symbol,
                frozen_depth_m,
                Some(0.0),
                Some(dg_m),
            )?;

            let frzw_symbol = Self::wb18_perc_state_symbol("frzw", layer_index);
            let frzw_m =
                Self::optional_state_scalar_for_symbol(request, phase_class, &frzw_symbol)?
                    .unwrap_or(0.0);
            Self::require_state_range_for_symbol(
                phase_class,
                &frzw_symbol,
                frzw_m,
                Some(0.0),
                Some(upper_limit_m),
            )?;

            layer_water_state.push(FrostLayerWaterState {
                layer_index,
                fine_layer_count,
                fine_layer_thickness_m: dg_m / Self::diagnostic_count_to_f64(fine_layer_count),
                dg_m,
                thetdr,
                theta_m,
                upper_limit_m,
                frozen_depth_m,
                frzw_m,
            });
        }

        let profile_depth_symbol = BoundarySymbol::from(PL_GROWTH_SOIL_DEPTH_SYMBOL);
        let profile_depth_m =
            Self::require_state_scalar_for_symbol(request, phase_class, &profile_depth_symbol)?;
        Self::require_dynamic_state_range(
            phase_class,
            profile_depth_symbol,
            profile_depth_m,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;
        let prior_layer_frozen_depth_m = layer_water_state
            .iter()
            .map(|layer| layer.frozen_depth_m)
            .sum::<f64>();
        let prior_layer_frozen_store_m = Self::frost_layer_soilf_sum(&layer_water_state);
        let prior_layer_state_active = prior_layer_frozen_depth_m > WB11_ZERO_THRESHOLD
            || prior_layer_frozen_store_m > WB11_ZERO_THRESHOLD;
        let mut shadow_fine_state =
            Self::compute_shadow_fine_state(request, phase_class, &layer_water_state)?;
        let prior_depth_summary =
            Self::derived_frost_depths_from_fine_state(&shadow_fine_state.fine_layers);
        let prior_fine_frozen_store_m = shadow_fine_state
            .layer_state
            .iter()
            .map(|layer| layer.soilf_m)
            .sum::<f64>();
        let prior_fine_state_active = prior_depth_summary.frdp > WB11_ZERO_THRESHOLD
            || prior_fine_frozen_store_m > WB11_ZERO_THRESHOLD;

        let snow_depth_symbol = BoundarySymbol::from(FROST_RUNTIME_SNOW_DEPTH_SYMBOL);
        let snow_depth_m =
            Self::require_state_scalar_for_symbol(request, phase_class, &snow_depth_symbol)?;
        Self::require_dynamic_state_range(
            phase_class,
            snow_depth_symbol,
            snow_depth_m,
            Some(0.0),
            None,
        )?;

        let residue_depth_symbol = BoundarySymbol::from(FROST_RUNTIME_RESIDUE_DEPTH_SYMBOL);
        let residue_depth_m =
            Self::require_state_scalar_for_symbol(request, phase_class, &residue_depth_symbol)?;
        Self::require_dynamic_state_range(
            phase_class,
            residue_depth_symbol,
            residue_depth_m,
            Some(0.0),
            None,
        )?;

        let tmax = Self::require_state_scalar(request, phase_class, WB14_SYMBOL_TMAX)?;
        let tmin = Self::require_state_scalar(request, phase_class, WB14_SYMBOL_TMIN)?;

        let frost_depth_symbol = BoundarySymbol::from(FROST_RUNTIME_FRDP_M_SYMBOL);
        let prior_frdp_m = Self::optional_state_scalar_for_symbol(
            request,
            phase_class,
            &frost_depth_symbol,
        )?
        .unwrap_or(0.0);
        Self::require_dynamic_state_range(
            phase_class,
            frost_depth_symbol.clone(),
            prior_frdp_m,
            Some(0.0),
            Some(profile_depth_m),
        )?;
        let effective_prior_frdp_m = if prior_fine_state_active {
            prior_depth_summary.frdp
        } else if prior_layer_state_active {
            prior_layer_frozen_depth_m
        } else {
            prior_frdp_m
        };
        Self::require_dynamic_state_range(
            phase_class,
            frost_depth_symbol,
            effective_prior_frdp_m,
            Some(0.0),
            Some(profile_depth_m),
        )?;

        let thaw_depth_symbol = BoundarySymbol::from(FROST_RUNTIME_THDP_M_SYMBOL);
        let prior_thdp_m = Self::optional_state_scalar_for_symbol(
            request,
            phase_class,
            &thaw_depth_symbol,
        )?
        .unwrap_or(0.0);
        Self::require_dynamic_state_range(
            phase_class,
            thaw_depth_symbol.clone(),
            prior_thdp_m,
            Some(0.0),
            Some(profile_depth_m),
        )?;

        let top_frost_depth_symbol = BoundarySymbol::from(FROST_RUNTIME_TFRDP_M_SYMBOL);
        let prior_top_frost_depth_m = Self::optional_state_scalar_for_symbol(
            request,
            phase_class,
            &top_frost_depth_symbol,
        )?
        .unwrap_or(0.0);
        Self::require_dynamic_state_range(
            phase_class,
            top_frost_depth_symbol.clone(),
            prior_top_frost_depth_m,
            Some(0.0),
            Some(profile_depth_m),
        )?;

        let top_thaw_depth_symbol = BoundarySymbol::from(FROST_RUNTIME_TTHAWD_M_SYMBOL);
        let prior_tthawd_m = Self::optional_state_scalar_for_symbol(
            request,
            phase_class,
            &top_thaw_depth_symbol,
        )?
        .unwrap_or(0.0);
        Self::require_dynamic_state_range(
            phase_class,
            top_thaw_depth_symbol.clone(),
            prior_tthawd_m,
            Some(0.0),
            Some(profile_depth_m),
        )?;

        let fgthwd_symbol = BoundarySymbol::from(FROST_RUNTIME_FGTHWD_FLAG_SYMBOL);
        let prior_fgthwd_flag = Self::optional_state_scalar_for_symbol(
            request,
            phase_class,
            &fgthwd_symbol,
        )?
        .unwrap_or(0.0);
        Self::require_dynamic_state_range(
            phase_class,
            fgthwd_symbol,
            prior_fgthwd_flag,
            Some(0.0),
            Some(1.0),
        )?;

        let prior_nft = Self::optional_state_scalar(request, phase_class, WB14_SYMBOL_FROST_RUNTIME_NFT)?
            .unwrap_or(0.0);
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_FROST_RUNTIME_NFT,
            prior_nft,
            Some(0.0),
            None,
        )?;

        let theta_residual =
            Self::require_state_scalar(request, phase_class, WB14_SYMBOL_SOIL_THETA_RESIDUAL)?;
        let theta_field_capacity =
            Self::require_state_scalar(request, phase_class, WB14_SYMBOL_SOIL_THETA_FIELD_CAPACITY)?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_SOIL_THETA_RESIDUAL,
            theta_residual,
            Some(0.0),
            None,
        )?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_SOIL_THETA_FIELD_CAPACITY,
            theta_field_capacity,
            Some(0.0),
            None,
        )?;
        if theta_field_capacity < theta_residual - WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_SOIL_THETA_FIELD_CAPACITY),
                value: theta_field_capacity,
                minimum: Some(theta_residual),
                maximum: None,
            });
        }

        let soil_water = Self::require_state_scalar(request, phase_class, WB11_SYMBOL_SOIL_WATER)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_SOIL_WATER,
            soil_water,
            Some(0.0),
            None,
        )?;

        let mut fgthwd_flag = prior_fgthwd_flag;

        let prior_runtime_ws_frz =
            Self::optional_state_scalar(request, phase_class, WB14_SYMBOL_FROST_RUNTIME_WS_FRZ)?
                .unwrap_or(0.0);
        let prior_ws_frz = if prior_fine_state_active {
            prior_fine_frozen_store_m
        } else if prior_layer_state_active {
            prior_layer_frozen_store_m
        } else {
            prior_runtime_ws_frz
        };
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_FROST_RUNTIME_WS_FRZ,
            prior_ws_frz,
            Some(0.0),
            None,
        )?;

        let kfactor_selected = Self::resolve_frozen_soil_kfactor(
            request,
            phase_class,
            kfactor1,
            kfactor2,
            kfactor3,
        )?;

        let conductivity_residue_w_m_k = FROST_RUNTIME_KRES_BASE_W_M_K * kresf;

        let snow_density_kg_m3 = Self::optional_state_scalar_for_symbol(
            request,
            phase_class,
            &BoundarySymbol::from(SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL),
        )?
        .unwrap_or(0.0);
        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from(SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL),
            snow_density_kg_m3,
            Some(0.0),
            Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
        )?;

        let snow_conductivity_w_m_k = if snow_depth_m > SIMIMPL29_MIN_CONDUCTIVE_SNOW_DEPTH_M
            && snow_density_kg_m3 > 0.0
        {
            let density_g_cm3 =
                openwepp_unit_boundary::conversions::kilograms_per_cubic_meter_to_grams_per_cubic_centimeter(
                    snow_density_kg_m3,
                )
                .map_err(|error| {
                    Self::unit_conversion_guard_error(
                        phase_class,
                        BoundarySymbol::from(SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL),
                        &error,
                    )
                })?;
            let base = if snow_density_kg_m3 < 156.0 {
                0.023 + (0.234 * density_g_cm3)
            } else {
                0.138 - 1.01 * density_g_cm3 + 3.233 * density_g_cm3.powi(2)
            };
            (base * ksnowf).max(WB11_ZERO_THRESHOLD)
        } else {
            0.0
        };

        let mut freeze_started = false;
        let mut hourly_state = std::array::from_fn(|hour_index| FrostHourlyState {
            hour: hour_index + 1,
            frzflg: 0.0,
            qsrf_w_m2: 0.0,
            quf_w_m2: 0.0,
            ksrf_w_m_k: FROST_RUNTIME_KFUTIL_W_M_K,
            snow_depth_m,
            residue_depth_m,
            tilled_frozen_depth_m: 0.0,
            untilled_frozen_depth_m: 0.0,
        });
        for hourly in &mut hourly_state {
            Self::reset_fine_layer_hour_timers(&mut shadow_fine_state.fine_layers);
            let depth_before =
                Self::derived_frost_depths_from_fine_state(&shadow_fine_state.fine_layers);
            let mut hourly_frdp_m = depth_before.frdp.min(profile_depth_m);
            let hourly_air_temp_c = Self::resolve_frost_hourly_air_temperature_c(
                request,
                phase_class,
                tmax,
                tmin,
                hourly.hour,
            )?;
            let surface_temp_c = if snow_depth_m > SIMIMPL29_MIN_CONDUCTIVE_SNOW_DEPTH_M
                && hourly_air_temp_c > 0.0
            {
                0.0
            } else {
                hourly_air_temp_c
            };

            let tilled_frozen_depth_before_m =
                depth_before.frdp.min(FROST_RUNTIME_TILLAGE_DEPTH_M);
            let untilled_frozen_depth_before_m =
                (depth_before.frdp - tilled_frozen_depth_before_m).max(0.0);
            let mut resistance_m2_c_w = 0.0;
            if snow_depth_m > SIMIMPL29_MIN_CONDUCTIVE_SNOW_DEPTH_M
                && snow_conductivity_w_m_k > WB11_ZERO_THRESHOLD
            {
                resistance_m2_c_w += snow_depth_m / snow_conductivity_w_m_k;
            }
            if residue_depth_m > SIMIMPL29_MIN_CONDUCTIVE_SNOW_DEPTH_M
                && conductivity_residue_w_m_k > WB11_ZERO_THRESHOLD
            {
                resistance_m2_c_w += residue_depth_m / conductivity_residue_w_m_k;
            }
            if tilled_frozen_depth_before_m > WB11_ZERO_THRESHOLD {
                resistance_m2_c_w += tilled_frozen_depth_before_m / FROST_RUNTIME_KFTILL_W_M_K;
            }
            if untilled_frozen_depth_before_m > WB11_ZERO_THRESHOLD {
                resistance_m2_c_w +=
                    untilled_frozen_depth_before_m / FROST_RUNTIME_KFUTIL_W_M_K;
            }

            if resistance_m2_c_w <= WB11_ZERO_THRESHOLD {
                resistance_m2_c_w = 0.5 / FROST_RUNTIME_KFTILL_W_M_K;
            }

            let total_frozen_path_m = snow_depth_m
                + residue_depth_m
                + tilled_frozen_depth_before_m
                + untilled_frozen_depth_before_m;
            let ksrf_w_m_k = if resistance_m2_c_w > WB11_ZERO_THRESHOLD {
                let path_m = total_frozen_path_m.max(0.005);
                path_m / resistance_m2_c_w
            } else {
                FROST_RUNTIME_KFUTIL_W_M_K
            };
            let signed_surface_flux_w_m2 = surface_temp_c / resistance_m2_c_w;
            let unfrozen_conductivity_w_m_k =
                (FROST_RUNTIME_KFUTIL_W_M_K * ksoilf).max(WB11_ZERO_THRESHOLD);
            let lower_front_temp_c = FROST_RUNTIME_STABLE_SOIL_TEMP_C.max(f64::midpoint(tmax, tmin));
            let lower_front_heat_w_m2 = if lower_front_temp_c > 0.0 {
                unfrozen_conductivity_w_m_k * lower_front_temp_c
                    / FROST_RUNTIME_UNFROZEN_HEAT_PATH_M
            } else {
                0.0
            };
            let signed_net_flux_w_m2 = signed_surface_flux_w_m2 + lower_front_heat_w_m2;
            hourly.qsrf_w_m2 = (-signed_surface_flux_w_m2).max(0.0);
            hourly.quf_w_m2 = lower_front_heat_w_m2;
            hourly.frzflg = Self::select_frost_branch(
                signed_surface_flux_w_m2,
                lower_front_heat_w_m2,
                signed_net_flux_w_m2,
                depth_before,
            );

            if signed_net_flux_w_m2 < -WB11_ZERO_THRESHOLD
                && ((hourly.frzflg - 1.0).abs() <= WB11_ZERO_THRESHOLD
                    || (hourly.frzflg - 2.0).abs() <= WB11_ZERO_THRESHOLD)
            {
                let energy_j_m2 = -signed_net_flux_w_m2 * FROST_RUNTIME_SECONDS_PER_HOUR;
                let remaining_energy_j_m2 = Self::refreeze_frozen_zone_liquid(
                    &mut shadow_fine_state.fine_layers,
                    &mut shadow_fine_state.layer_state,
                    &layer_water_state,
                    energy_j_m2,
                );
                Self::freeze_fine_front(
                    &mut shadow_fine_state.fine_layers,
                    &layer_water_state,
                    remaining_energy_j_m2,
                );
                let depth_after =
                    Self::derived_frost_depths_from_fine_state(&shadow_fine_state.fine_layers);
                hourly_frdp_m = depth_after.frdp.min(profile_depth_m);
                if hourly_frdp_m > WB11_ZERO_THRESHOLD {
                    fgthwd_flag = 0.0;
                    if effective_prior_frdp_m <= WB11_ZERO_THRESHOLD {
                        freeze_started = true;
                    }
                }
            } else if signed_net_flux_w_m2 > WB11_ZERO_THRESHOLD
                && depth_before.frdp > WB11_ZERO_THRESHOLD
            {
                let energy_j_m2 = signed_net_flux_w_m2 * FROST_RUNTIME_SECONDS_PER_HOUR;
                Self::thaw_fine_bottom(&mut shadow_fine_state.fine_layers, energy_j_m2);
                let depth_after =
                    Self::derived_frost_depths_from_fine_state(&shadow_fine_state.fine_layers);
                hourly_frdp_m = depth_after.frdp.min(profile_depth_m);
                fgthwd_flag = if hourly_frdp_m <= WB11_ZERO_THRESHOLD {
                    1.0
                } else {
                    0.0
                };
                if fgthwd_flag > 0.0 {
                    hourly_frdp_m = 0.0;
                }
            }
            hourly.ksrf_w_m_k = ksrf_w_m_k.max(WB11_ZERO_THRESHOLD);
            hourly.tilled_frozen_depth_m = hourly_frdp_m.min(FROST_RUNTIME_TILLAGE_DEPTH_M);
            hourly.untilled_frozen_depth_m =
                (hourly_frdp_m - hourly.tilled_frozen_depth_m).max(0.0);
        }

        Self::aggregate_active_layers_from_fine_state(
            &shadow_fine_state.fine_layers,
            &mut shadow_fine_state.layer_state,
            &mut layer_water_state,
        );
        let final_depth =
            Self::derived_frost_depths_from_fine_state(&shadow_fine_state.fine_layers);
        let dfrost = final_depth.frdp.min(profile_depth_m);
        let thdp_m = final_depth.thdp;
        let tfrdp_m = final_depth.tfrdp;
        let tthawd_m = final_depth.tthawd;
        let bottom_retreat_m = (prior_depth_summary.frdp - dfrost).max(0.0);
        let dthaw = thdp_m.max(bottom_retreat_m);
        let nft = if freeze_started { prior_nft + 1.0 } else { prior_nft };
        let ws_frz = Self::frost_layer_soilf_sum(&layer_water_state);
        let raw_frwatc_freeze_exchange = if ws_frz > prior_ws_frz + WB11_ZERO_THRESHOLD {
            ws_frz - prior_ws_frz
        } else {
            0.0
        };
        let frwatc_freeze_exchange =
            if raw_frwatc_freeze_exchange > soil_water
                && raw_frwatc_freeze_exchange <= soil_water + WB11_ZERO_THRESHOLD
            {
                soil_water
            } else {
                raw_frwatc_freeze_exchange
            };
        let frwatc_thaw_release = if prior_ws_frz > ws_frz + WB11_ZERO_THRESHOLD {
            prior_ws_frz - ws_frz
        } else {
            0.0
        };
        if frwatc_freeze_exchange > soil_water + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB11_SYMBOL_SOIL_WATER),
                value: soil_water,
                minimum: Some(frwatc_freeze_exchange),
                maximum: None,
            });
        }
        let frwatc_net_liquid_delta = frwatc_thaw_release - frwatc_freeze_exchange;
        let frwatc_soil_water_after = soil_water + frwatc_net_liquid_delta;
        let soil_water_after_frwatc = if frwatc_freeze_exchange > WB11_ZERO_THRESHOLD
            || frwatc_thaw_release > WB11_ZERO_THRESHOLD
        {
            Some(frwatc_soil_water_after)
        } else {
            None
        };
        let freeze_fraction = (dfrost / FROST_RUNTIME_TILLAGE_DEPTH_M).clamp(0.0, 1.0);
        let infcap_frz =
            soil_conductivity * (1.0 - freeze_fraction + freeze_fraction * kfactor_selected);

        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_FROST_RUNTIME_DFROST,
            dfrost,
            Some(0.0),
            Some(profile_depth_m),
        )?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_FROST_RUNTIME_DTHAW,
            dthaw,
            Some(0.0),
            Some(profile_depth_m),
        )?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_FROST_RUNTIME_NFT,
            nft,
            Some(0.0),
            None,
        )?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_FROST_RUNTIME_WS_FRZ,
            ws_frz,
            Some(0.0),
            None,
        )?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_FROST_RUNTIME_INFCAP_FRZ,
            infcap_frz,
            Some(0.0),
            Some(soil_conductivity),
        )?;

        Ok(FrostCouplingOutcome {
            dfrost,
            dthaw,
            nft,
            ws_frz,
            infcap_frz,
            soil_water_after_frwatc,
            frwatc_soil_water_before: soil_water,
            frwatc_soil_water_after,
            frwatc_frozen_water_before: prior_ws_frz,
            frwatc_frozen_water_after: ws_frz,
            frwatc_freeze_debit: frwatc_freeze_exchange,
            frwatc_thaw_credit: frwatc_thaw_release,
            frwatc_net_liquid_delta,
            frdp_m: dfrost,
            thdp_m: dthaw,
            tfrdp_m,
            tthawd_m,
            profile_depth_m,
            fgthwd_flag,
            total_fine_layer_count: Self::diagnostic_count_to_f64(total_fine_layer_count),
            conductivity_tilled_w_m_k: FROST_RUNTIME_KFTILL_W_M_K,
            conductivity_untilled_w_m_k: FROST_RUNTIME_KFUTIL_W_M_K,
            conductivity_residue_w_m_k,
            shadow_total_water_before_m: shadow_fine_state.total_water_before_m,
            shadow_total_water_after_m: shadow_fine_state.total_water_after_m,
            shadow_wb_delta_m: shadow_fine_state.wb_delta_m,
            shadow_frwatc_residual_m: shadow_fine_state.residual_m,
            hourly_state,
            layer_topology_state: layer_water_state
                .into_iter()
                .map(|layer| FrostLayerTopologyState {
                    layer_index: layer.layer_index,
                    fine_layer_count: layer.fine_layer_count,
                    fine_layer_thickness_m: layer.fine_layer_thickness_m,
                    dg_m: layer.dg_m,
                    theta_after_m: layer.theta_m,
                    frozen_depth_m: layer.frozen_depth_m,
                    frzw_m: layer.frzw_m,
                })
                .collect(),
            shadow_layer_state: shadow_fine_state
                .layer_state
                .into_iter()
                .map(|layer| FrostLayerShadowState {
                    layer_index: layer.layer_index,
                    st_m: layer.st_m,
                    soil_water_m: layer.soil_water_m,
                    frozen_depth_m: layer.frozen_m,
                    frzw_m: layer.frzw_m,
                    soilf_m: layer.soilf_m,
                    yst_m: layer.yst_m,
                    nwfrzz_m: layer.nwfrzz_m,
                })
                .collect(),
            fine_layer_state: shadow_fine_state
                .fine_layers
                .into_iter()
                .map(|fine| FrostFineLayerDiagnosticState {
                    layer_index: fine.layer_index,
                    fine_index: fine.fine_index,
                    fgfrst: fine.fgfrst,
                    slfsd_m: fine.slfsd_m,
                    slsic_m: fine.slsic_m,
                    slsw_theta: fine.slsw_theta,
                    sltime_s: fine.sltime_s,
                })
                .collect(),
        })
    }

}
