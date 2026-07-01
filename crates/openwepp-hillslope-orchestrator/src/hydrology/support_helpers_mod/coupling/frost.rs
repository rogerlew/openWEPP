#[allow(clippy::wildcard_imports)]
use super::super::super::*;

#[allow(clippy::wildcard_imports)]
use super::*;

const TMPADJ_WIND_MEASUREMENT_HEIGHT_M: f64 = 2.0;

#[derive(Debug, Clone, Copy)]
#[allow(clippy::struct_field_names)]
struct TmpadjAerodynamicRoughness {
    displacement_m: f64,
    wind_roughness_m: f64,
    transfer_roughness_m: f64,
}

#[derive(Debug, Clone, Copy)]
#[allow(clippy::struct_field_names)]
struct TmpadjGradientDepths {
    total_m: f64,
    tilled_m: f64,
    untilled_m: f64,
}

impl Wb11HydrologyKernel {
    pub(super) fn frost_layer_soilf_sum(layers: &[FrostLayerWaterState]) -> f64 {
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

    fn fine_layer_unfrozen_depth_m(fine: &FrostFineLayerState) -> f64 {
        (fine.fine_layer_thickness_m - fine.slfsd_m).max(0.0)
    }

    fn fine_layer_capacity_per_m(layer: &FrostLayerWaterState) -> f64 {
        layer.thetdr + layer.upper_limit_m / layer.dg_m
    }

    pub(super) fn fine_layer_ice_capacity_m(
        layer: &FrostLayerWaterState,
        fine: &FrostFineLayerState,
    ) -> f64 {
        Self::fine_layer_capacity_per_m(layer) * fine.slfsd_m
    }

    pub(super) fn fine_layer_liquid_theta_capacity(layer: &FrostLayerWaterState) -> f64 {
        Self::fine_layer_capacity_per_m(layer)
    }

    fn fine_layer_total_liquid_m(fine: &FrostFineLayerState) -> f64 {
        fine.slsw_theta * Self::fine_layer_unfrozen_depth_m(fine)
    }

    pub(super) fn canonicalize_near_upper_bound(value: f64, upper: f64) -> f64 {
        if value > upper && value <= upper + WB11_ZERO_THRESHOLD {
            upper
        } else {
            value
        }
    }

    pub(super) fn canonicalize_near_lower_bound(value: f64, lower: f64) -> f64 {
        if value < lower && value >= lower - FROST_RUNTIME_FINE_THETA_BOUND_TOLERANCE {
            lower
        } else {
            value
        }
    }

    pub(super) fn canonicalize_fine_layer_liquid_theta(
        fine: &mut FrostFineLayerState,
        water_layer: &FrostLayerWaterState,
    ) {
        fine.slsw_theta = Self::canonicalize_near_lower_bound(fine.slsw_theta, water_layer.thetdr);
        fine.slsw_theta = Self::canonicalize_near_upper_bound(
            fine.slsw_theta,
            Self::fine_layer_liquid_theta_capacity(water_layer),
        );
    }

    fn add_unfrozen_liquid_to_fine_layer(
        fine: &mut FrostFineLayerState,
        water_layer: &FrostLayerWaterState,
        water_m: f64,
    ) -> f64 {
        let unfrozen_depth_m = Self::fine_layer_unfrozen_depth_m(fine);
        if water_m <= WB11_ZERO_THRESHOLD || unfrozen_depth_m <= WB11_ZERO_THRESHOLD {
            return water_m.max(0.0);
        }
        let current_m = Self::fine_layer_total_liquid_m(fine);
        let capacity_m =
            Self::fine_layer_liquid_theta_capacity(water_layer) * unfrozen_depth_m;
        let room_m = (capacity_m - current_m).max(0.0);
        let accepted_m = water_m.min(room_m);
        if accepted_m > WB11_ZERO_THRESHOLD {
            fine.slsw_theta = (current_m + accepted_m) / unfrozen_depth_m;
        }
        (water_m - accepted_m).max(0.0)
    }

    fn add_unfrozen_liquid_to_layer(
        fine_layers: &mut [FrostFineLayerState],
        water_layer: &FrostLayerWaterState,
        mut water_m: f64,
    ) -> f64 {
        for fine in fine_layers
            .iter_mut()
            .filter(|fine| fine.layer_index == water_layer.layer_index)
        {
            water_m = Self::add_unfrozen_liquid_to_fine_layer(fine, water_layer, water_m);
            if water_m <= WB11_ZERO_THRESHOLD {
                return 0.0;
            }
        }
        water_m.max(0.0)
    }

    fn route_unfrozen_liquid_downward(
        fine_layers: &mut [FrostFineLayerState],
        water_layers: &[FrostLayerWaterState],
        start_layer_index: usize,
        start_fine_index: usize,
        mut water_m: f64,
    ) -> f64 {
        if water_m <= WB11_ZERO_THRESHOLD {
            return 0.0;
        }
        for fine in fine_layers.iter_mut().filter(|fine| {
            fine.layer_index > start_layer_index
                || (fine.layer_index == start_layer_index && fine.fine_index >= start_fine_index)
        }) {
            let Some(water_layer) = water_layers
                .iter()
                .find(|layer| layer.layer_index == fine.layer_index)
            else {
                continue;
            };
            water_m = Self::add_unfrozen_liquid_to_fine_layer(fine, water_layer, water_m);
            if water_m <= WB11_ZERO_THRESHOLD {
                return 0.0;
            }
        }
        water_m.max(0.0)
    }

    fn route_unfrozen_liquid_upward(
        fine_layers: &mut [FrostFineLayerState],
        water_layers: &[FrostLayerWaterState],
        start_layer_index: usize,
        start_fine_index: usize,
        mut water_m: f64,
    ) -> f64 {
        if water_m <= WB11_ZERO_THRESHOLD {
            return 0.0;
        }
        for fine in fine_layers.iter_mut().rev().filter(|fine| {
            fine.layer_index < start_layer_index
                || (fine.layer_index == start_layer_index && fine.fine_index <= start_fine_index)
        }) {
            let Some(water_layer) = water_layers
                .iter()
                .find(|layer| layer.layer_index == fine.layer_index)
            else {
                continue;
            };
            water_m = Self::add_unfrozen_liquid_to_fine_layer(fine, water_layer, water_m);
            if water_m <= WB11_ZERO_THRESHOLD {
                return 0.0;
            }
        }
        water_m.max(0.0)
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

    #[allow(clippy::too_many_lines)]
    pub(super) fn require_shadow_fine_state_domains(
        phase_class: HillslopeKernelPhaseClass,
        fine: &FrostFineLayerState,
        layer: &FrostLayerWaterState,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        let thetdr = layer.thetdr;
        Self::require_frost_fine_state_range(
            phase_class,
            FROST_RUNTIME_FINE_FGFRST_ROOT,
            fine.layer_index,
            fine.fine_index,
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
        Self::require_frost_fine_state_range(
            phase_class,
            FROST_RUNTIME_FINE_SLFSD_M_ROOT,
            fine.layer_index,
            fine.fine_index,
            fine.slfsd_m,
            Some(0.0),
            Some(fine.fine_layer_thickness_m),
        )?;
        Self::require_frost_fine_state_range(
            phase_class,
            FROST_RUNTIME_FINE_SLSIC_M_ROOT,
            fine.layer_index,
            fine.fine_index,
            fine.slsic_m,
            Some(0.0),
            Some(Self::fine_layer_ice_capacity_m(layer, fine)),
        )?;
        Self::require_frost_fine_state_range(
            phase_class,
            FROST_RUNTIME_FINE_SLSW_THETA_ROOT,
            fine.layer_index,
            fine.fine_index,
            fine.slsw_theta,
            Some(thetdr),
            Some(Self::fine_layer_liquid_theta_capacity(layer)),
        )?;
        Self::require_frost_fine_state_range(
            phase_class,
            FROST_RUNTIME_FINE_SLTIME_S_ROOT,
            fine.layer_index,
            fine.fine_index,
            fine.sltime_s,
            Some(0.0),
            Some(FROST_RUNTIME_SECONDS_PER_HOUR),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn require_frost_fine_state_range(
        phase_class: HillslopeKernelPhaseClass,
        symbol_root: &'static str,
        layer_index: usize,
        fine_index: usize,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if let Some(minimum_value) = minimum
            && value < minimum_value - WB11_ZERO_THRESHOLD
        {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: Self::frost_fine_layer_symbol(symbol_root, layer_index, fine_index),
                value,
                minimum,
                maximum,
            });
        }
        if let Some(maximum_value) = maximum
            && value > maximum_value + WB11_ZERO_THRESHOLD
        {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: Self::frost_fine_layer_symbol(symbol_root, layer_index, fine_index),
                value,
                minimum,
                maximum,
            });
        }
        Ok(())
    }

    fn apply_shadow_frwatc_ingress(
        fine_layers: &mut [FrostFineLayerState],
        layer: &mut FrostLayerExchangeState,
        water_layer: &FrostLayerWaterState,
        watbtm_m: &mut f64,
    ) {
        let mut remaining_delta_m = layer.st_m - layer.yst_m;
        if remaining_delta_m > WB11_ZERO_THRESHOLD {
            if layer.frozen_m > WB11_ZERO_THRESHOLD {
                let frozen_zone_capacity_m = (Self::fine_layer_capacity_per_m(water_layer)
                    * layer.frozen_m
                    - layer.soilf_m
                    - layer.nwfrzz_m)
                    .max(0.0);
                let requested_frozen_zone_m = remaining_delta_m * layer.frozen_m / water_layer.dg_m;
                let into_frozen_zone_m = requested_frozen_zone_m.min(frozen_zone_capacity_m);
                layer.nwfrzz_m += into_frozen_zone_m;
                remaining_delta_m -= into_frozen_zone_m;
            }
            if remaining_delta_m > WB11_ZERO_THRESHOLD {
                remaining_delta_m =
                    Self::add_unfrozen_liquid_to_layer(fine_layers, water_layer, remaining_delta_m);
            }
            if remaining_delta_m > WB11_ZERO_THRESHOLD {
                *watbtm_m += remaining_delta_m;
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
                for fine in fine_layers.iter_mut() {
                    if remaining_drain_m <= WB11_ZERO_THRESHOLD {
                        break;
                    }
                    let unfrozen_depth_m =
                        (fine.fine_layer_thickness_m - fine.slfsd_m).max(0.0);
                    if unfrozen_depth_m > WB11_ZERO_THRESHOLD {
                        let available_m =
                            (fine.slsw_theta - layer.thetdr).max(0.0) * unfrozen_depth_m;
                        let drained_m = available_m.min(remaining_drain_m);
                        let retained_m = (available_m - drained_m).max(0.0);
                        fine.slsw_theta = layer.thetdr + retained_m / unfrozen_depth_m;
                        remaining_drain_m -= drained_m;
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

    pub(super) fn compute_shadow_fine_state_from_typed(
        prior_state: &DirectFrostPriorStateInput,
        phase_class: HillslopeKernelPhaseClass,
        layers: &[FrostLayerWaterState],
    ) -> Result<FrostFineShadowState, Wb11HydrologyKernelGuardError> {
        let mut all_fine_layers = Vec::new();
        let mut shadow_layers = Vec::with_capacity(layers.len());
        let mut total_water_before_m = 0.0;
        let mut total_water_after_m = 0.0;
        let mut wb_delta_m = 0.0;
        let watpdg_m = 0.0;
        let mut watbtm_m = 0.0;

        for layer in layers {
            let mut remaining_frozen_depth_m = layer.frozen_depth_m;
            let mut fine_layers = Vec::with_capacity(layer.fine_layer_count);
            for fine_index in 1..=layer.fine_layer_count {
                let default = Self::default_fine_layer_from_coarse(
                    layer,
                    fine_index,
                    &mut remaining_frozen_depth_m,
                );
                let prior = prior_state
                    .fine_layers
                    .iter()
                    .find(|fine| {
                        fine.layer_index == layer.layer_index && fine.fine_index == fine_index
                    });
                let mut fine = FrostFineLayerState {
                    layer_index: layer.layer_index,
                    fine_index,
                    fine_layer_thickness_m: layer.fine_layer_thickness_m,
                    fgfrst: prior.map_or(default.fgfrst, |fine| fine.fgfrst),
                    slfsd_m: prior.map_or(default.slfsd_m, |fine| fine.slfsd_m),
                    slsic_m: prior.map_or(default.slsic_m, |fine| fine.slsic_m),
                    slsw_theta: prior.map_or(default.slsw_theta, |fine| fine.slsw_theta),
                sltime_s: prior.map_or(0.0, |fine| fine.sltime_s),
                };
                Self::canonicalize_fine_layer_liquid_theta(&mut fine, layer);
                Self::require_shadow_fine_state_domains(phase_class, &fine, layer)?;
                fine_layers.push(fine);
            }

            let prior_layer = prior_state
                .layer_shadows
                .iter()
                .find(|shadow| shadow.layer_index == layer.layer_index);
            let nwfrzz_m = prior_layer.map_or(0.0, |shadow| shadow.nwfrzz_m);
            Self::require_dynamic_state_range_with(
                phase_class,
                || Self::frost_layer_symbol(FROST_RUNTIME_LAYER_NWFRZZ_M_ROOT, layer.layer_index),
                nwfrzz_m,
                Some(0.0),
                None,
            )?;
            let st_m = layer.theta_m + nwfrzz_m;
            let yst_m = prior_layer.map_or(st_m, |shadow| shadow.yst_m);
            Self::require_dynamic_state_range_with(
                phase_class,
                || Self::frost_layer_symbol(FROST_RUNTIME_LAYER_YST_M_ROOT, layer.layer_index),
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
            Self::apply_shadow_frwatc_ingress(
                &mut fine_layers,
                &mut shadow_layer,
                layer,
                &mut watbtm_m,
            );
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
            residual_m: total_water_after_m + watpdg_m + watbtm_m
                - total_water_before_m
                - wb_delta_m,
            watpdg_m,
            watbtm_m,
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

    pub(super) fn derived_frost_depths_from_fine_state(
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

    pub(super) fn aggregate_active_layers_from_fine_state(
        fine_layers: &mut [FrostFineLayerState],
        exchange_layers: &mut [FrostLayerExchangeState],
        water_layers: &mut [FrostLayerWaterState],
    ) {
        for water_layer in water_layers {
            let layer_frozen_sum_m = fine_layers
                .iter()
                .filter(|fine| fine.layer_index == water_layer.layer_index)
                .map(|fine| fine.slfsd_m)
                .sum::<f64>();
            if layer_frozen_sum_m > water_layer.dg_m
                && layer_frozen_sum_m <= water_layer.dg_m + WB11_ZERO_THRESHOLD
            {
                let mut excess_m = layer_frozen_sum_m - water_layer.dg_m;
                for fine in fine_layers
                    .iter_mut()
                    .rev()
                    .filter(|fine| fine.layer_index == water_layer.layer_index)
                {
                    if excess_m <= 0.0 {
                        break;
                    }
                    let debit_m = fine.slfsd_m.min(excess_m);
                    if debit_m > 0.0 {
                        let ice_per_m = if fine.slfsd_m > WB11_ZERO_THRESHOLD {
                            fine.slsic_m / fine.slfsd_m
                        } else {
                            0.0
                        };
                        fine.slfsd_m -= debit_m;
                        fine.slsic_m = (fine.slsic_m - ice_per_m * debit_m).max(0.0);
                        Self::refresh_fine_frost_flag(fine);
                        excess_m -= debit_m;
                    }
                }
            }
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
            let raw_frzw_m =
                (soilf_m - water_layer.thetdr * water_layer.frozen_depth_m).max(0.0);
            water_layer.frzw_m =
                Self::canonicalize_near_upper_bound(raw_frzw_m, water_layer.upper_limit_m);
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

    pub(super) fn frost_surface_heat_path(
        frdp_m: f64,
        snow_depth_m: f64,
        snow_conductivity_w_m_k: f64,
        residue_depth_m: f64,
        residue_conductivity_w_m_k: f64,
        apply_shallow_front_minimum: bool,
        shallow_front_minimum_path_m: f64,
    ) -> (f64, f64, f64) {
        let effective_frdp_m = if apply_shallow_front_minimum {
            frdp_m.max(shallow_front_minimum_path_m.max(0.0))
        } else {
            frdp_m
        };
        let tilled_frozen_depth_m = effective_frdp_m.min(FROST_RUNTIME_TILLAGE_DEPTH_M);
        let untilled_frozen_depth_m = (effective_frdp_m - tilled_frozen_depth_m).max(0.0);
        let mut resistance_m2_c_w = 0.0;
        let mut total_frozen_path_m = 0.0;
        if snow_depth_m > SIMIMPL29_MIN_CONDUCTIVE_SNOW_DEPTH_M
            && snow_conductivity_w_m_k > WB11_ZERO_THRESHOLD
        {
            resistance_m2_c_w += snow_depth_m / snow_conductivity_w_m_k;
            total_frozen_path_m += snow_depth_m;
        }
        if residue_depth_m > SIMIMPL29_MIN_CONDUCTIVE_SNOW_DEPTH_M
            && residue_conductivity_w_m_k > WB11_ZERO_THRESHOLD
        {
            resistance_m2_c_w += residue_depth_m / residue_conductivity_w_m_k;
            total_frozen_path_m += residue_depth_m;
        }
        if tilled_frozen_depth_m > WB11_ZERO_THRESHOLD {
            resistance_m2_c_w += tilled_frozen_depth_m / FROST_RUNTIME_KFTILL_W_M_K;
            total_frozen_path_m += tilled_frozen_depth_m;
        }
        if untilled_frozen_depth_m > WB11_ZERO_THRESHOLD {
            resistance_m2_c_w += untilled_frozen_depth_m / FROST_RUNTIME_KFUTIL_W_M_K;
            total_frozen_path_m += untilled_frozen_depth_m;
        }

        if resistance_m2_c_w <= WB11_ZERO_THRESHOLD {
            resistance_m2_c_w = 0.5 / FROST_RUNTIME_KFTILL_W_M_K;
        }
        let ksrf_w_m_k = total_frozen_path_m.max(0.005) / resistance_m2_c_w;
        (resistance_m2_c_w, total_frozen_path_m, ksrf_w_m_k)
    }

    pub(super) fn shallow_front_minimum_conduction_path_m(fine_layers: &[FrostFineLayerState]) -> f64 {
        fine_layers
            .first()
            .map(|fine| fine.fine_layer_thickness_m / 2.0)
            .filter(|path| path.is_finite() && *path > WB11_ZERO_THRESHOLD)
            .unwrap_or(FROST_RUNTIME_SHALLOW_FRONT_MIN_CONDUCTION_PATH_M)
    }

    fn legacy_tmpfun(
        annual_mean_c: f64,
        amplitude_c: f64,
        monthly_mean_c: &[f64; 12],
        phase_shift_days: f64,
    ) -> f64 {
        let mut square_error = 0.0;
        for (index, observed) in monthly_mean_c.iter().enumerate() {
            let tday = 15.0 + Self::diagnostic_count_to_f64(index) * 30.5;
            let estimated = annual_mean_c
                + amplitude_c
                    * ((std::f64::consts::TAU / 365.0) * (tday - phase_shift_days)).sin();
            square_error += (estimated - observed).powi(2);
        }
        (square_error / 12.0).sqrt()
    }

    pub(super) fn fit_legacy_tmpcft_curve(
        monthly_max_c: &[f64; 12],
        monthly_min_c: &[f64; 12],
    ) -> FrostSeasonalTemperatureCurve {
        let mut monthly_mean_c = [0.0; 12];
        let mut annual_mean_c = 0.0;
        let mut maximum_mean_c = f64::NEG_INFINITY;
        let mut minimum_mean_c = f64::INFINITY;
        for (index, value) in monthly_mean_c.iter_mut().enumerate() {
            *value = f64::midpoint(monthly_max_c[index], monthly_min_c[index]);
            annual_mean_c += *value;
            maximum_mean_c = maximum_mean_c.max(*value);
            minimum_mean_c = minimum_mean_c.min(*value);
        }
        annual_mean_c /= 12.0;
        let amplitude_c = (maximum_mean_c - minimum_mean_c) / 2.0;
        if amplitude_c <= WB11_ZERO_THRESHOLD {
            return FrostSeasonalTemperatureCurve {
                annual_mean_c,
                amplitude_c,
                phase_shift_days: 0.0,
            };
        }

        let mut phase_shift_days = 0.0;
        let mut delta_phase_days: f64 = 1.0;
        let mut iteration = 0;
        while delta_phase_days.abs() > 0.00001 && iteration < 20 {
            let mut first_derivative = 0.0;
            let mut second_derivative = 0.0;
            iteration += 1;
            for (index, observed) in monthly_mean_c.iter().enumerate() {
                let tday = 15.0 + Self::diagnostic_count_to_f64(index) * 30.5;
                let theta = (std::f64::consts::TAU / 365.0) * (tday - phase_shift_days);
                first_derivative -=
                    (annual_mean_c + amplitude_c * theta.sin() - observed) * theta.cos();
                second_derivative -= (std::f64::consts::TAU / 365.0)
                    * ((annual_mean_c - observed) * theta.sin()
                        - amplitude_c * (2.0 * theta).cos());
            }
            if second_derivative < 0.0 {
                phase_shift_days += 365.0 / 2.0;
            } else if second_derivative.abs() > WB11_ZERO_THRESHOLD {
                delta_phase_days = -first_derivative / second_derivative;
                phase_shift_days += delta_phase_days;
            } else {
                iteration = 20;
                break;
            }
            phase_shift_days %= 365.0;
            if phase_shift_days < 0.0 {
                phase_shift_days += 365.0;
            }
        }

        if iteration >= 20 || !phase_shift_days.is_finite() {
            let mut minimum_error =
                Self::legacy_tmpfun(annual_mean_c, amplitude_c, &monthly_mean_c, 0.0);
            phase_shift_days = 0.0;
            for day in 1..=365 {
                let candidate = Self::diagnostic_count_to_f64(day);
                let error =
                    Self::legacy_tmpfun(annual_mean_c, amplitude_c, &monthly_mean_c, candidate);
                if error < minimum_error {
                    minimum_error = error;
                    phase_shift_days = candidate;
                }
            }
        }

        FrostSeasonalTemperatureCurve {
            annual_mean_c,
            amplitude_c,
            phase_shift_days,
        }
    }

    fn seasonal_lower_front_temperature_c(
        seasonal_curve: FrostSeasonalTemperatureCurve,
        sdate: f64,
        frdp_m: f64,
    ) -> f64 {
        let tmpdp = frdp_m + FROST_RUNTIME_UNFROZEN_LOWER_HEAT_PATH_M;
        seasonal_curve.annual_mean_c
            + seasonal_curve.amplitude_c
                * (-tmpdp / FROST_RUNTIME_SOIL_DAMPING_DEPTH_M).exp()
                * ((std::f64::consts::TAU / 365.0)
                    * (sdate - seasonal_curve.phase_shift_days)
                    - tmpdp / FROST_RUNTIME_SOIL_DAMPING_DEPTH_M)
                    .sin()
    }

    fn unfrozen_soil_conductivity_w_m_k(
        slsw_theta: f64,
        bulk_density_kg_m3: f64,
        ksoilf: f64,
    ) -> f64 {
        let moisture_factor = 0.5096 + 7.4493 * slsw_theta - 8.7484 * slsw_theta.powi(2);
        let density_factor = 0.001_413_9 * bulk_density_kg_m3 - 1.0588;
        let conductivity_w_m_k = moisture_factor * density_factor * ksoilf;
        if conductivity_w_m_k.is_finite() && conductivity_w_m_k > WB11_ZERO_THRESHOLD {
            conductivity_w_m_k
        } else {
            0.0
        }
    }

    fn lower_front_unfrozen_conductivity_w_m_k(
        fine_layers: &[FrostFineLayerState],
        water_layers: &[FrostLayerWaterState],
        frdp_m: f64,
        ksoilf: f64,
    ) -> f64 {
        let path_top_m = frdp_m.max(0.0);
        let path_bottom_m = path_top_m + FROST_RUNTIME_UNFROZEN_LOWER_HEAT_PATH_M;
        let mut cursor_m = 0.0;
        let mut resistance_m2_c_w = 0.0;

        for fine in fine_layers {
            let fine_top_m = cursor_m;
            let fine_bottom_m = fine_top_m + fine.fine_layer_thickness_m;
            cursor_m = fine_bottom_m;

            let overlap_m =
                (fine_bottom_m.min(path_bottom_m) - fine_top_m.max(path_top_m)).max(0.0);
            if overlap_m <= WB11_ZERO_THRESHOLD {
                continue;
            }
            let Some(water_layer) = water_layers
                .iter()
                .find(|layer| layer.layer_index == fine.layer_index)
            else {
                continue;
            };
            let conductivity_w_m_k = Self::unfrozen_soil_conductivity_w_m_k(
                fine.slsw_theta,
                water_layer.bulk_density_kg_m3,
                ksoilf,
            );
            if conductivity_w_m_k > WB11_ZERO_THRESHOLD {
                resistance_m2_c_w += overlap_m / conductivity_w_m_k;
            }
        }

        if resistance_m2_c_w > WB11_ZERO_THRESHOLD {
            1.0 / resistance_m2_c_w
        } else {
            FROST_RUNTIME_UNFROZEN_CONDUCTIVITY_FALLBACK_W_M_K
        }
    }

    pub(super) fn lower_front_heat_w_m2(
        seasonal_curve: FrostSeasonalTemperatureCurve,
        sdate: f64,
        frdp_m: f64,
        fine_layers: &[FrostFineLayerState],
        water_layers: &[FrostLayerWaterState],
        ksoilf: f64,
    ) -> f64 {
        let tmpbl_c = Self::seasonal_lower_front_temperature_c(seasonal_curve, sdate, frdp_m);
        if tmpbl_c <= 0.0 {
            0.0
        } else {
            Self::lower_front_unfrozen_conductivity_w_m_k(
                fine_layers,
                water_layers,
                frdp_m,
                ksoilf,
            ) * tmpbl_c
                / FROST_RUNTIME_UNFROZEN_LOWER_HEAT_PATH_M
        }
    }

    fn freeze_fine_front_step(
        fine_layers: &mut [FrostFineLayerState],
        water_layers: &[FrostLayerWaterState],
        mut energy_j_m2: f64,
        watbtm_m: &mut f64,
    ) -> (f64, bool) {
        for index in 0..fine_layers.len() {
            if energy_j_m2 <= WB11_ZERO_THRESHOLD {
                break;
            }
            let Some(water_layer) = water_layers
                .iter()
                .find(|layer| layer.layer_index == fine_layers[index].layer_index)
            else {
                continue;
            };
            let unfrozen_depth_m =
                (fine_layers[index].fine_layer_thickness_m - fine_layers[index].slfsd_m).max(0.0);
            if unfrozen_depth_m <= WB11_ZERO_THRESHOLD {
                Self::refresh_fine_frost_flag(&mut fine_layers[index]);
                continue;
            }
            let liquid_capacity_per_m = Self::fine_layer_liquid_theta_capacity(water_layer);
            let water_per_m = fine_layers[index]
                .slsw_theta
                .max(water_layer.thetdr)
                .min(liquid_capacity_per_m);
            if water_per_m <= WB11_ZERO_THRESHOLD {
                continue;
            }
            let energy_limited_depth_m =
                energy_j_m2 / (FROST_RUNTIME_LATENT_HEAT_WATER_J_M3 * water_per_m);
            let freeze_depth_m = unfrozen_depth_m.min(energy_limited_depth_m);
            if freeze_depth_m <= WB11_ZERO_THRESHOLD {
                continue;
            }
            let overflow_m =
                (fine_layers[index].slsw_theta - liquid_capacity_per_m).max(0.0)
                    * freeze_depth_m;
            let overflow_start_layer = fine_layers[index].layer_index;
            let overflow_start_fine = fine_layers[index].fine_index;
            fine_layers[index].slsic_m += water_per_m * freeze_depth_m;
            fine_layers[index].slfsd_m += freeze_depth_m;
            energy_j_m2 -=
                FROST_RUNTIME_LATENT_HEAT_WATER_J_M3 * water_per_m * freeze_depth_m;
            Self::refresh_fine_frost_flag(&mut fine_layers[index]);
            if overflow_m > WB11_ZERO_THRESHOLD {
                let remaining_overflow_m = Self::route_unfrozen_liquid_downward(
                    fine_layers,
                    water_layers,
                    overflow_start_layer,
                    overflow_start_fine,
                    overflow_m,
                );
                *watbtm_m += remaining_overflow_m;
            }
            return (energy_j_m2, true);
        }
        (energy_j_m2, false)
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn freeze_fine_front_with_resistance_feedback(
        fine_layers: &mut [FrostFineLayerState],
        exchange_layers: &mut [FrostLayerExchangeState],
        water_layers: &[FrostLayerWaterState],
        frzflg: f64,
        surface_temp_c: f64,
        lower_front_heat_w_m2: f64,
        snow_depth_m: f64,
        snow_conductivity_w_m_k: f64,
        residue_depth_m: f64,
        residue_conductivity_w_m_k: f64,
        watbtm_m: &mut f64,
    ) {
        let mut remaining_seconds = FROST_RUNTIME_SECONDS_PER_HOUR;
        while remaining_seconds > WB11_ZERO_THRESHOLD {
            let depth = Self::derived_frost_depths_from_fine_state(fine_layers);
            let (resistance_m2_c_w, _, _) = Self::frost_surface_heat_path(
                depth.frdp,
                snow_depth_m,
                snow_conductivity_w_m_k,
                residue_depth_m,
                residue_conductivity_w_m_k,
                surface_temp_c < 0.0,
                Self::shallow_front_minimum_conduction_path_m(fine_layers),
            );
            let signed_surface_flux_w_m2 = surface_temp_c / resistance_m2_c_w;
            let signed_net_flux_w_m2 = signed_surface_flux_w_m2 + lower_front_heat_w_m2;
            let freeze_flux_w_m2 = if (frzflg - 2.0).abs() <= WB11_ZERO_THRESHOLD {
                (-signed_surface_flux_w_m2).max(0.0)
            } else {
                (-signed_net_flux_w_m2).max(0.0)
            };
            if freeze_flux_w_m2 <= WB11_ZERO_THRESHOLD {
                break;
            }

            let energy_for_remaining_hour_j_m2 = freeze_flux_w_m2 * remaining_seconds;
            let after_refreeze_j_m2 = Self::refreeze_frozen_zone_liquid(
                fine_layers,
                exchange_layers,
                water_layers,
                energy_for_remaining_hour_j_m2,
            );
            let refreeze_consumed_j_m2 =
                energy_for_remaining_hour_j_m2 - after_refreeze_j_m2;
            if refreeze_consumed_j_m2 > WB11_ZERO_THRESHOLD {
                remaining_seconds =
                    (remaining_seconds - refreeze_consumed_j_m2 / freeze_flux_w_m2).max(0.0);
                if remaining_seconds <= WB11_ZERO_THRESHOLD
                    || after_refreeze_j_m2 <= WB11_ZERO_THRESHOLD
                {
                    break;
                }
            }

            let (after_front_j_m2, advanced) = Self::freeze_fine_front_step(
                fine_layers,
                water_layers,
                after_refreeze_j_m2,
                watbtm_m,
            );
            let front_consumed_j_m2 = after_refreeze_j_m2 - after_front_j_m2;
            if front_consumed_j_m2 > WB11_ZERO_THRESHOLD {
                remaining_seconds =
                    (remaining_seconds - front_consumed_j_m2 / freeze_flux_w_m2).max(0.0);
            }
            if !advanced
                || remaining_seconds <= WB11_ZERO_THRESHOLD
                || after_front_j_m2 <= WB11_ZERO_THRESHOLD
                || front_consumed_j_m2 <= WB11_ZERO_THRESHOLD
            {
                break;
            }
        }
    }

    fn release_frozen_zone_liquid(
        exchange_layers: &mut [FrostLayerExchangeState],
        layer_index: usize,
        thaw_depth_m: f64,
    ) -> f64 {
        let Some(exchange_layer) = exchange_layers
            .iter_mut()
            .find(|layer| layer.layer_index == layer_index)
        else {
            return 0.0;
        };
        if thaw_depth_m <= WB11_ZERO_THRESHOLD
            || exchange_layer.nwfrzz_m <= WB11_ZERO_THRESHOLD
            || exchange_layer.frozen_m <= WB11_ZERO_THRESHOLD
        {
            exchange_layer.frozen_m = (exchange_layer.frozen_m - thaw_depth_m).max(0.0);
            return 0.0;
        }
        let released_m = (exchange_layer.nwfrzz_m * thaw_depth_m / exchange_layer.frozen_m)
            .min(exchange_layer.nwfrzz_m)
            .max(0.0);
        exchange_layer.nwfrzz_m = (exchange_layer.nwfrzz_m - released_m).max(0.0);
        exchange_layer.frozen_m = (exchange_layer.frozen_m - thaw_depth_m).max(0.0);
        if exchange_layer.frozen_m <= WB11_ZERO_THRESHOLD {
            exchange_layer.frozen_m = 0.0;
            let remaining_m = exchange_layer.nwfrzz_m;
            exchange_layer.nwfrzz_m = 0.0;
            released_m + remaining_m
        } else {
            released_m
        }
    }

    fn thaw_fine_bottom_step(
        fine_layers: &mut [FrostFineLayerState],
        exchange_layers: &mut [FrostLayerExchangeState],
        water_layers: &[FrostLayerWaterState],
        mut energy_j_m2: f64,
        watbtm_m: &mut f64,
    ) -> (f64, bool) {
        let initial_energy_j_m2 = energy_j_m2;
        for index in (0..fine_layers.len()).rev() {
            if energy_j_m2 <= WB11_ZERO_THRESHOLD {
                break;
            }
            let fine = &fine_layers[index];
            if fine.slfsd_m <= WB11_ZERO_THRESHOLD || fine.slsic_m <= WB11_ZERO_THRESHOLD {
                Self::refresh_fine_frost_flag(&mut fine_layers[index]);
                continue;
            }
            let Some(_water_layer) = water_layers
                .iter()
                .find(|layer| layer.layer_index == fine.layer_index)
            else {
                continue;
            };
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
            let layer_index = fine.layer_index;
            let fine_index = fine.fine_index;
            let old_unfrozen_m = (fine.fine_layer_thickness_m - fine.slfsd_m).max(0.0);
            let old_liquid_m = fine.slsw_theta * old_unfrozen_m;
            let melted_m = ice_per_m * thaw_depth_m;
            let released_m =
                Self::release_frozen_zone_liquid(exchange_layers, layer_index, thaw_depth_m);
            {
                let fine = &mut fine_layers[index];
                fine.slsic_m = (fine.slsic_m - melted_m).max(0.0);
                fine.slfsd_m = (fine.slfsd_m - thaw_depth_m).max(0.0);
                let new_unfrozen_m = (fine.fine_layer_thickness_m - fine.slfsd_m).max(0.0);
                if new_unfrozen_m > WB11_ZERO_THRESHOLD {
                    fine.slsw_theta = old_liquid_m / new_unfrozen_m;
                }
                if fine.slfsd_m > WB11_ZERO_THRESHOLD {
                    fine.fgfrst = 2.0;
                }
                Self::refresh_fine_frost_flag(fine);
            }
            let remaining_m = Self::route_unfrozen_liquid_downward(
                fine_layers,
                water_layers,
                layer_index,
                fine_index,
                melted_m + released_m,
            );
            *watbtm_m += remaining_m;
            energy_j_m2 -= FROST_RUNTIME_LATENT_HEAT_WATER_J_M3 * melted_m;
            return (energy_j_m2, initial_energy_j_m2 - energy_j_m2 > WB11_ZERO_THRESHOLD);
        }
        (energy_j_m2, false)
    }

    pub(super) fn thaw_fine_bottom_with_resistance_feedback(
        fine_layers: &mut [FrostFineLayerState],
        exchange_layers: &mut [FrostLayerExchangeState],
        water_layers: &[FrostLayerWaterState],
        seasonal_curve: FrostSeasonalTemperatureCurve,
        sdate: f64,
        ksoilf: f64,
        watbtm_m: &mut f64,
    ) {
        let mut remaining_seconds = FROST_RUNTIME_SECONDS_PER_HOUR;
        while remaining_seconds > WB11_ZERO_THRESHOLD {
            let depth = Self::derived_frost_depths_from_fine_state(fine_layers);
            let bottom_flux_w_m2 = Self::lower_front_heat_w_m2(
                seasonal_curve,
                sdate,
                depth.frdp,
                fine_layers,
                water_layers,
                ksoilf,
            );
            if bottom_flux_w_m2 <= WB11_ZERO_THRESHOLD {
                break;
            }
            let energy_j_m2 = bottom_flux_w_m2 * remaining_seconds;
            let (remaining_energy_j_m2, thawed) = Self::thaw_fine_bottom_step(
                fine_layers,
                exchange_layers,
                water_layers,
                energy_j_m2,
                watbtm_m,
            );
            let consumed_j_m2 = energy_j_m2 - remaining_energy_j_m2;
            if consumed_j_m2 > WB11_ZERO_THRESHOLD {
                remaining_seconds =
                    (remaining_seconds - consumed_j_m2 / bottom_flux_w_m2).max(0.0);
            }
            if !thawed
                || consumed_j_m2 <= WB11_ZERO_THRESHOLD
                || remaining_energy_j_m2 <= WB11_ZERO_THRESHOLD
            {
                break;
            }
        }
    }

    fn thaw_surface_heat_path(
        depth_summary: FrostDepthSummary,
        snow_depth_m: f64,
        snow_conductivity_w_m_k: f64,
        residue_depth_m: f64,
        residue_conductivity_w_m_k: f64,
    ) -> f64 {
        let mut resistance_m2_c_w = 0.0;
        if snow_depth_m > SIMIMPL29_MIN_CONDUCTIVE_SNOW_DEPTH_M
            && snow_conductivity_w_m_k > WB11_ZERO_THRESHOLD
        {
            resistance_m2_c_w += snow_depth_m / snow_conductivity_w_m_k;
        }
        if residue_depth_m > SIMIMPL29_MIN_CONDUCTIVE_SNOW_DEPTH_M
            && residue_conductivity_w_m_k > WB11_ZERO_THRESHOLD
        {
            resistance_m2_c_w += residue_depth_m / residue_conductivity_w_m_k;
        }
        let thawed_path_m =
            FROST_RUNTIME_TILLAGE_DEPTH_M.max(depth_summary.frdp) + depth_summary.thdp;
        if thawed_path_m > WB11_ZERO_THRESHOLD {
            resistance_m2_c_w += thawed_path_m / FROST_RUNTIME_KFTILL_W_M_K;
        }
        if resistance_m2_c_w <= WB11_ZERO_THRESHOLD {
            FROST_RUNTIME_TILLAGE_DEPTH_M / FROST_RUNTIME_KFTILL_W_M_K
        } else {
            resistance_m2_c_w
        }
    }

    fn thaw_fine_top_step(
        fine_layers: &mut [FrostFineLayerState],
        exchange_layers: &mut [FrostLayerExchangeState],
        water_layers: &[FrostLayerWaterState],
        mut energy_j_m2: f64,
    ) -> (f64, f64, bool) {
        let initial_energy_j_m2 = energy_j_m2;
        let mut watpdg_m = 0.0;
        for index in 0..fine_layers.len() {
            if energy_j_m2 <= WB11_ZERO_THRESHOLD {
                break;
            }
            let fine = &fine_layers[index];
            if fine.slfsd_m <= WB11_ZERO_THRESHOLD || fine.slsic_m <= WB11_ZERO_THRESHOLD {
                Self::refresh_fine_frost_flag(&mut fine_layers[index]);
                continue;
            }
            let Some(_water_layer) = water_layers
                .iter()
                .find(|layer| layer.layer_index == fine.layer_index)
            else {
                continue;
            };
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
            let layer_index = fine.layer_index;
            let fine_index = fine.fine_index;
            let old_unfrozen_m = (fine.fine_layer_thickness_m - fine.slfsd_m).max(0.0);
            let old_liquid_m = fine.slsw_theta * old_unfrozen_m;
            let melted_m = ice_per_m * thaw_depth_m;
            let released_m =
                Self::release_frozen_zone_liquid(exchange_layers, layer_index, thaw_depth_m);
            {
                let fine = &mut fine_layers[index];
                fine.slsic_m = (fine.slsic_m - melted_m).max(0.0);
                fine.slfsd_m = (fine.slfsd_m - thaw_depth_m).max(0.0);
                let new_unfrozen_m = (fine.fine_layer_thickness_m - fine.slfsd_m).max(0.0);
                if new_unfrozen_m > WB11_ZERO_THRESHOLD {
                    fine.slsw_theta = old_liquid_m / new_unfrozen_m;
                }
                if fine.slfsd_m > WB11_ZERO_THRESHOLD {
                    fine.fgfrst = 3.0;
                }
                Self::refresh_fine_frost_flag(fine);
            }
            let remaining_m = Self::route_unfrozen_liquid_upward(
                fine_layers,
                water_layers,
                layer_index,
                fine_index,
                melted_m + released_m,
            );
            watpdg_m += remaining_m;
            energy_j_m2 -= FROST_RUNTIME_LATENT_HEAT_WATER_J_M3 * melted_m;
            return (
                energy_j_m2,
                watpdg_m,
                initial_energy_j_m2 - energy_j_m2 > WB11_ZERO_THRESHOLD,
            );
        }
        (energy_j_m2, watpdg_m, false)
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn thaw_fine_top_with_resistance_feedback(
        fine_layers: &mut [FrostFineLayerState],
        exchange_layers: &mut [FrostLayerExchangeState],
        water_layers: &[FrostLayerWaterState],
        surface_temp_c: f64,
        snow_depth_m: f64,
        snow_conductivity_w_m_k: f64,
        residue_depth_m: f64,
        residue_conductivity_w_m_k: f64,
    ) -> f64 {
        if surface_temp_c <= WB11_ZERO_THRESHOLD {
            return 0.0;
        }
        let mut remaining_seconds = FROST_RUNTIME_SECONDS_PER_HOUR;
        let mut watpdg_m = 0.0;
        while remaining_seconds > WB11_ZERO_THRESHOLD {
            let depth = Self::derived_frost_depths_from_fine_state(fine_layers);
            let resistance_m2_c_w = Self::thaw_surface_heat_path(
                depth,
                snow_depth_m,
                snow_conductivity_w_m_k,
                residue_depth_m,
                residue_conductivity_w_m_k,
            );
            let top_flux_w_m2 = surface_temp_c / resistance_m2_c_w;
            if top_flux_w_m2 <= WB11_ZERO_THRESHOLD {
                break;
            }
            let energy_j_m2 = top_flux_w_m2 * remaining_seconds;
            let (remaining_energy_j_m2, step_watpdg_m, thawed) = Self::thaw_fine_top_step(
                fine_layers,
                exchange_layers,
                water_layers,
                energy_j_m2,
            );
            watpdg_m += step_watpdg_m;
            let consumed_j_m2 = energy_j_m2 - remaining_energy_j_m2;
            if consumed_j_m2 > WB11_ZERO_THRESHOLD {
                remaining_seconds =
                    (remaining_seconds - consumed_j_m2 / top_flux_w_m2).max(0.0);
            }
            if !thawed
                || consumed_j_m2 <= WB11_ZERO_THRESHOLD
                || remaining_energy_j_m2 <= WB11_ZERO_THRESHOLD
            {
                break;
            }
        }
        watpdg_m
    }

    pub(super) fn reset_fine_layer_hour_timers(fine_layers: &mut [FrostFineLayerState]) {
        for fine in fine_layers {
            fine.sltime_s = 0.0;
        }
    }

    pub(super) fn select_frost_branch(
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

    pub(super) fn tmpadj_snow_conductivity_w_m_k(
        phase_class: HillslopeKernelPhaseClass,
        snow_density_kg_m3: f64,
        ksnowf: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
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
        Ok((base * ksnowf).max(WB11_ZERO_THRESHOLD))
    }

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    pub(super) fn legacy_tmpadj_surface_temperature_from_typed(
        phase_class: HillslopeKernelPhaseClass,
        hour: usize,
        hourly_forcing: DirectFrostHourlyForcing,
        tmpadj: ActiveFrostTmpadjContext,
        snow_depth_m: f64,
        snow_density_kg_m3: f64,
        ksnowf: f64,
        residue_depth_m: f64,
        residue_conductivity_w_m_k: f64,
        depth_summary: FrostDepthSummary,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        const TMPADJ_STEFAN_BOLTZMANN_W_M2_K4: f64 = 5.6697e-8;
        const TMPADJ_VON_KARMAN: f64 = 0.4;
        const TMPADJ_AIR_DENSITY_KG_M3: f64 = 1.2;
        const TMPADJ_AIR_HEAT_CAPACITY_J_KG_K: f64 = 1012.0;
        const TMPADJ_SURFACE_EMISSIVITY: f64 = 1.0;
        const TMPADJ_SNOW_ALBEDO: f64 = 0.5;

        let hourly_air_temp_c = hourly_forcing.air_temperature_c;
        let hourly_rad_mj_m2 = hourly_forcing.radiation_mj_m2;
        let cloud_fraction = hourly_forcing.cloud_fraction;
        Self::require_state_range_with(
            phase_class,
            || Self::hourly_symbol(WINTER_HOURLY_AIR_TEMP_ROOT, hour),
            hourly_air_temp_c,
            Some(-273.16),
            None,
        )?;
        Self::require_state_range_with(
            phase_class,
            || Self::hourly_symbol(WINTER_HOURLY_RAD_ROOT, hour),
            hourly_rad_mj_m2,
            Some(0.0),
            None,
        )?;
        Self::require_state_range_with(
            phase_class,
            || Self::hourly_symbol(WINTER_HOURLY_CLOUD_ROOT, hour),
            cloud_fraction,
            Some(0.0),
            Some(1.0),
        )?;
        Self::require_state_range_with(
            phase_class,
            || BoundarySymbol::from("vwind"),
            tmpadj.wind_m_s,
            Some(0.0),
            None,
        )?;
        Self::require_state_range_with(
            phase_class,
            || BoundarySymbol::from("salb"),
            tmpadj.albedo,
            Some(0.0),
            Some(1.0),
        )?;
        Self::require_state_range_with(
            phase_class,
            || BoundarySymbol::from("canhgt"),
            tmpadj.canopy_height_m,
            Some(0.0),
            None,
        )?;
        Self::require_state_range_with(
            phase_class,
            || BoundarySymbol::from("rrc"),
            tmpadj.random_roughness_m,
            Some(0.0),
            None,
        )?;

        let albedo = if snow_depth_m > 0.01 {
            TMPADJ_SNOW_ALBEDO
        } else {
            tmpadj.albedo
        };
        let incoming_shortwave_w_m2 = (hourly_rad_mj_m2 / FROST_RUNTIME_SECONDS_PER_HOUR) * 1.0e6;
        let air_temp_k = hourly_air_temp_c + 273.16;
        let atmospheric_emissivity = (1.0 - 0.84 * cloud_fraction)
            * (1.0 - 0.261 * (7.77e-4 * hourly_air_temp_c.powi(2)).exp())
            + (0.84 * cloud_fraction);

        let roughness = Self::tmpadj_aerodynamic_roughness(
            snow_depth_m,
            tmpadj.canopy_height_m,
            tmpadj.random_roughness_m,
        );
        let convective_heat_transfer_j_m3_k = (TMPADJ_VON_KARMAN.powi(2)
            * TMPADJ_AIR_DENSITY_KG_M3
            * TMPADJ_AIR_HEAT_CAPACITY_J_KG_K)
            / (((TMPADJ_WIND_MEASUREMENT_HEIGHT_M - roughness.displacement_m
                + roughness.transfer_roughness_m)
                / roughness.transfer_roughness_m)
                .ln()
                * ((TMPADJ_WIND_MEASUREMENT_HEIGHT_M - roughness.displacement_m
                    + roughness.wind_roughness_m)
                    / roughness.wind_roughness_m)
                    .ln());
        let longwave_transfer_w_m2_k = 4.0
            * TMPADJ_SURFACE_EMISSIVITY
            * TMPADJ_STEFAN_BOLTZMANN_W_M2_K4
            * air_temp_k.powi(3);
        let net_radiation_w_m2 = (1.0 - albedo) * incoming_shortwave_w_m2
            + (atmospheric_emissivity - TMPADJ_SURFACE_EMISSIVITY)
                * TMPADJ_STEFAN_BOLTZMANN_W_M2_K4
                * air_temp_k.powi(4);

        let gradient_depths = Self::tmpadj_gradient_depths(hourly_air_temp_c, depth_summary);

        let mut gradient_depth_m = gradient_depths.total_m;
        let mut tilled_gradient_depth_m = gradient_depths.tilled_m;
        let untilled_gradient_depth_m = gradient_depths.untilled_m;
        let mut system_depth_m = snow_depth_m + residue_depth_m + gradient_depth_m;
        let snow_conductivity_w_m_k = if snow_depth_m < 0.0001 {
            1.0
        } else {
            Self::tmpadj_snow_conductivity_w_m_k(phase_class, snow_density_kg_m3, ksnowf)?
        };
        let residue_conductivity_w_m_k = if residue_depth_m < 0.0001 {
            1.0
        } else {
            residue_conductivity_w_m_k
        };
        let mut tilled_conductivity_w_m_k = if tilled_gradient_depth_m < 0.0001 {
            1.0
        } else {
            FROST_RUNTIME_KFTILL_W_M_K
        };
        let untilled_conductivity_w_m_k = if untilled_gradient_depth_m < 0.0001 {
            1.0
        } else {
            FROST_RUNTIME_KFUTIL_W_M_K
        };
        if system_depth_m < 0.0001 {
            tilled_conductivity_w_m_k = FROST_RUNTIME_KFTILL_W_M_K;
            gradient_depth_m = 0.001;
            tilled_gradient_depth_m = 0.001;
            system_depth_m = snow_depth_m + residue_depth_m + gradient_depth_m;
        }

        let numerator = (snow_conductivity_w_m_k
            * residue_conductivity_w_m_k
            * tilled_conductivity_w_m_k
            * untilled_conductivity_w_m_k)
            * (snow_depth_m + residue_depth_m + gradient_depth_m);
        let denominator = (snow_conductivity_w_m_k
            * residue_conductivity_w_m_k
            * tilled_conductivity_w_m_k
            * untilled_gradient_depth_m)
            + (snow_conductivity_w_m_k
                * residue_conductivity_w_m_k
                * tilled_gradient_depth_m
                * untilled_conductivity_w_m_k)
            + (snow_conductivity_w_m_k
                * residue_depth_m
                * tilled_conductivity_w_m_k
                * untilled_conductivity_w_m_k)
            + (snow_depth_m
                * residue_conductivity_w_m_k
                * tilled_conductivity_w_m_k
                * untilled_conductivity_w_m_k);
        let effective_conductivity_w_m_k = if denominator.abs() > 0.0001 {
            numerator / denominator
        } else {
            0.0
        };

        let turbulent_exchange_w_m2_k =
            longwave_transfer_w_m2_k + convective_heat_transfer_j_m3_k * tmpadj.wind_m_s;
        let surface_temp_c = if system_depth_m > 0.0 {
            (net_radiation_w_m2 + turbulent_exchange_w_m2_k * hourly_air_temp_c)
                / (turbulent_exchange_w_m2_k + effective_conductivity_w_m_k / system_depth_m)
        } else {
            (net_radiation_w_m2 + turbulent_exchange_w_m2_k * hourly_air_temp_c)
                / turbulent_exchange_w_m2_k
        };
        if surface_temp_c > 0.0 && snow_depth_m > 0.001 {
            Ok(0.0)
        } else {
            Ok(surface_temp_c)
        }
    }

    fn tmpadj_aerodynamic_roughness(
        snow_depth_m: f64,
        canopy_height_m: f64,
        random_roughness_m: f64,
    ) -> TmpadjAerodynamicRoughness {
        let mut displacement_m = 0.77 * canopy_height_m;
        if displacement_m >= TMPADJ_WIND_MEASUREMENT_HEIGHT_M {
            displacement_m = 0.77 * TMPADJ_WIND_MEASUREMENT_HEIGHT_M;
        }
        let wind_roughness_m = if snow_depth_m < 0.01 && canopy_height_m > 0.0 {
            0.13 * canopy_height_m
        } else if snow_depth_m < 0.01 {
            random_roughness_m
        } else if snow_depth_m > canopy_height_m {
            0.0002
        } else {
            0.13 * (canopy_height_m - snow_depth_m)
        }
        .clamp(0.001, 0.26);
        TmpadjAerodynamicRoughness {
            displacement_m,
            wind_roughness_m,
            transfer_roughness_m: 0.2 * wind_roughness_m,
        }
    }

    fn tmpadj_gradient_depths(
        hourly_air_temp_c: f64,
        depth_summary: FrostDepthSummary,
    ) -> TmpadjGradientDepths {
        let gradient_depth_m =
            Self::tmpadj_gradient_depth_m(hourly_air_temp_c, depth_summary);
        Self::tmpadj_split_gradient_depth(gradient_depth_m)
    }

    fn tmpadj_gradient_depth_m(
        hourly_air_temp_c: f64,
        depth_summary: FrostDepthSummary,
    ) -> f64 {
        if hourly_air_temp_c < 0.0 {
            return Self::tmpadj_freezing_gradient_depth_m(depth_summary);
        }
        if depth_summary.thdp > 0.001
            && (depth_summary.tfrdp > 0.001 || depth_summary.frdp > 0.001)
        {
            depth_summary.thdp
        } else {
            0.0
        }
    }

    fn tmpadj_freezing_gradient_depth_m(depth_summary: FrostDepthSummary) -> f64 {
        if depth_summary.tfrdp > 0.001 {
            if depth_summary.thdp > 0.001 {
                0.0
            } else {
                depth_summary.tfrdp
            }
        } else if depth_summary.frdp > 0.001 && depth_summary.thdp <= 0.001 {
            depth_summary.frdp
        } else {
            0.0
        }
    }

    fn tmpadj_split_gradient_depth(gradient_depth_m: f64) -> TmpadjGradientDepths {
        let (mut tilled_m, mut untilled_m) =
            if gradient_depth_m <= FROST_RUNTIME_TILLAGE_DEPTH_M {
                (gradient_depth_m, 0.0)
            } else {
                (
                    FROST_RUNTIME_TILLAGE_DEPTH_M,
                    gradient_depth_m - FROST_RUNTIME_TILLAGE_DEPTH_M,
                )
            };
        if tilled_m < 0.001 {
            tilled_m = 0.0;
        }
        if untilled_m < 0.001 {
            untilled_m = 0.0;
        }
        TmpadjGradientDepths {
            total_m: if gradient_depth_m < 0.001 {
                0.0
            } else {
                gradient_depth_m
            },
            tilled_m,
            untilled_m,
        }
    }
}
