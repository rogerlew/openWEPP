fn direct_production_overlay_frost_runtime_carry(
    surface: &mut DirectFrostRunoffSurface,
    lane_index: usize,
    carry: &DirectFrostRuntimeCarry,
) -> Result<(), HillslopeCliError> {
    direct_production_insert_frost_runtime_scalars(
        surface,
        DirectFrostRuntimeScalarSeed {
            dfrost_m: carry.dfrost_m,
            dthaw_m: carry.dthaw_m,
            nft: carry.nft,
            ws_frz_m: carry.ws_frz_m,
            infcap_frz_m_s: carry.infcap_frz_m_s,
            frwatc_soil_water_before_m: carry.frwatc_soil_water_before_m,
            frwatc_soil_water_after_m: carry.frwatc_soil_water_after_m,
            frwatc_frozen_water_before_m: carry.frwatc_frozen_water_before_m,
            frwatc_frozen_water_after_m: carry.frwatc_frozen_water_after_m,
            frwatc_freeze_debit_m: carry.frwatc_freeze_debit_m,
            frwatc_thaw_credit_m: carry.frwatc_thaw_credit_m,
            frwatc_net_liquid_delta_m: carry.frwatc_net_liquid_delta_m,
            frdp_m: carry.frdp_m,
            thdp_m: carry.thdp_m,
            tfrdp_m: carry.tfrdp_m,
            tthawd_m: carry.tthawd_m,
            fgthwd_flag: carry.fgthwd_flag,
            total_fine_layer_count: carry.total_fine_layer_count,
            conductivity_tilled_w_m_k: carry.conductivity_tilled_w_m_k,
            conductivity_untilled_w_m_k: carry.conductivity_untilled_w_m_k,
            conductivity_residue_w_m_k: carry.conductivity_residue_w_m_k,
            shadow_total_water_before_m: carry.shadow_total_water_before_m,
            shadow_total_water_after_m: carry.shadow_total_water_after_m,
            shadow_wb_delta_m: carry.shadow_wb_delta_m,
            shadow_frwatc_residual_m: carry.shadow_frwatc_residual_m,
            watpdg_m: carry.watpdg_m,
            watbtm_m: carry.watbtm_m,
            fine_projection: direct_publication_frost_runtime_carry_has_fine_projection(carry),
        },
    )?;
    for layer in &carry.layer_shadows {
        direct_production_insert_frost_layer_shadow(surface, *layer)?;
    }
    for fine in &carry.fine_layers {
        direct_production_insert_frost_fine_layer(surface, *fine)?;
    }
    if !carry.layer_shadows.is_empty() || !carry.fine_layers.is_empty() {
        let _ = lane_index;
    }
    Ok(())
}

fn direct_production_frost_surface_template(
    seed_surface: &HillslopeWritebackSurface,
) -> DirectFrostRunoffSurface {
    let mut state_surface = seed_surface.state_surface.clone();
    state_surface
        .retain(|symbol, _| direct_production_retains_frost_surface_symbol(symbol.as_str()));
    DirectFrostRunoffSurface::from_surface_maps(state_surface, std::collections::BTreeMap::new())
}

fn direct_production_retains_frost_surface_symbol(symbol: &str) -> bool {
    matches!(
        symbol,
        "wb11_nsl"
            | "nsl"
            | "wb11_soil_water"
            | "thetdr"
            | "thetfc"
            | "solthk"
            | "day"
            | "year"
            | "tmax"
            | "tmin"
            | "vwind"
            | "salb"
            | "canhgt"
            | "rrc"
            | "rrinit"
            | "snow.runtime_depth_m"
            | "snow.runtime_density_kg_m3"
    ) || symbol.starts_with("frost.")
        || symbol.starts_with("wb18_perc_theta_")
        || symbol.starts_with("wb18_perc_ul_")
        || symbol.starts_with("wb18_perc_frozen_depth_")
        || symbol.starts_with("wb18_perc_frzw_")
        || symbol.starts_with("wb19_dg_")
        || symbol.starts_with("dg_")
        || symbol.starts_with("wb19_thetdr_")
        || symbol.starts_with("thetdr_")
        || symbol.starts_with("wb19_bulk_density_kg_m3_")
        || symbol.starts_with("winter.hourly.rad_mj_m2_")
        || symbol.starts_with("winter.hourly.air_temp_c_")
        || symbol.starts_with("winter.hourly.cloud_fraction_")
        || symbol.starts_with("obmaxt_")
        || symbol.starts_with("obmint_")
}

fn direct_production_insert_frost_runtime_scalars(
    surface: &mut DirectFrostRunoffSurface,
    seed: DirectFrostRuntimeScalarSeed,
) -> Result<(), HillslopeCliError> {
    for (symbol, value) in [
        (
            "frost.direct_runtime_carry_present",
            if seed.fine_projection { 1.0 } else { 0.0 },
        ),
        ("frost.runtime_dfrost", seed.dfrost_m),
        ("frost.runtime_dthaw", seed.dthaw_m),
        ("frost.runtime_nft", seed.nft),
        ("frost.runtime_ws_frz", seed.ws_frz_m),
        ("frost.runtime_infcap_frz", seed.infcap_frz_m_s),
        (
            "frost.runtime_frwatc_soil_water_before_m",
            seed.frwatc_soil_water_before_m,
        ),
        (
            "frost.runtime_frwatc_soil_water_after_m",
            seed.frwatc_soil_water_after_m,
        ),
        (
            "frost.runtime_frwatc_frozen_water_before_m",
            seed.frwatc_frozen_water_before_m,
        ),
        (
            "frost.runtime_frwatc_frozen_water_after_m",
            seed.frwatc_frozen_water_after_m,
        ),
        (
            "frost.runtime_frwatc_freeze_debit_m",
            seed.frwatc_freeze_debit_m,
        ),
        (
            "frost.runtime_frwatc_thaw_credit_m",
            seed.frwatc_thaw_credit_m,
        ),
        (
            "frost.runtime_frwatc_net_liquid_delta_m",
            seed.frwatc_net_liquid_delta_m,
        ),
        ("frost.runtime_frdp_m", seed.frdp_m),
        ("frost.runtime_thdp_m", seed.thdp_m),
        ("frost.runtime_tfrdp_m", seed.tfrdp_m),
        ("frost.runtime_tthawd_m", seed.tthawd_m),
        ("frost.runtime_fgthwd_flag", seed.fgthwd_flag),
        (
            "frost.runtime_total_fine_layer_count",
            seed.total_fine_layer_count,
        ),
        ("frost.runtime_kftill_w_m_k", seed.conductivity_tilled_w_m_k),
        (
            "frost.runtime_kfutil_w_m_k",
            seed.conductivity_untilled_w_m_k,
        ),
        ("frost.runtime_kres_w_m_k", seed.conductivity_residue_w_m_k),
        (
            "frost.runtime_shadow_total_water_before_m",
            seed.shadow_total_water_before_m,
        ),
        (
            "frost.runtime_shadow_total_water_after_m",
            seed.shadow_total_water_after_m,
        ),
        ("frost.runtime_shadow_wb_delta_m", seed.shadow_wb_delta_m),
        (
            "frost.runtime_shadow_frwatc_residual_m",
            seed.shadow_frwatc_residual_m,
        ),
        ("frost.runtime_watpdg_m", seed.watpdg_m),
        ("frost.runtime_watbtm_m", seed.watbtm_m),
    ] {
        direct_production_insert_frost_surface_scalar(surface, symbol, value)?;
    }
    Ok(())
}

fn direct_production_insert_frost_layer_shadow(
    surface: &mut DirectFrostRunoffSurface,
    layer: DirectFrostLayerShadowCarry,
) -> Result<(), HillslopeCliError> {
    for (symbol, value) in [
        (
            format!("frost.runtime_shadow_st_m_{:04}", layer.layer_index),
            layer.st_m,
        ),
        (
            format!(
                "frost.runtime_shadow_soil_water_m_{:04}",
                layer.layer_index
            ),
            layer.soil_water_m,
        ),
        (
            format!(
                "frost.runtime_shadow_frozen_depth_m_{:04}",
                layer.layer_index
            ),
            layer.frozen_depth_m,
        ),
        (
            format!("frost.runtime_shadow_frzw_m_{:04}", layer.layer_index),
            layer.frozen_water_m,
        ),
        (
            format!("frost.runtime_shadow_soilf_m_{:04}", layer.layer_index),
            layer.soilf_m,
        ),
        (
            format!("frost.runtime_yst_m_{:04}", layer.layer_index),
            layer.yst_m,
        ),
        (
            format!("frost.runtime_nwfrzz_m_{:04}", layer.layer_index),
            layer.nwfrzz_m,
        ),
    ] {
        direct_production_insert_frost_surface_scalar(surface, symbol.as_str(), value)?;
    }
    Ok(())
}

fn direct_production_insert_frost_fine_layer(
    surface: &mut DirectFrostRunoffSurface,
    fine: DirectFrostFineLayerCarry,
) -> Result<(), HillslopeCliError> {
    for (symbol, value) in [
        (
            format!(
                "frost.runtime_fgfrst_{:04}_{:04}",
                fine.layer_index, fine.fine_index
            ),
            fine.fgfrst,
        ),
        (
            format!(
                "frost.runtime_slfsd_m_{:04}_{:04}",
                fine.layer_index, fine.fine_index
            ),
            fine.slfsd_m,
        ),
        (
            format!(
                "frost.runtime_slsic_m_{:04}_{:04}",
                fine.layer_index, fine.fine_index
            ),
            fine.slsic_m,
        ),
        (
            format!(
                "frost.runtime_slsw_theta_{:04}_{:04}",
                fine.layer_index, fine.fine_index
            ),
            fine.slsw_theta,
        ),
        (
            format!(
                "frost.runtime_sltime_s_{:04}_{:04}",
                fine.layer_index, fine.fine_index
            ),
            fine.sltime_s,
        ),
    ] {
        direct_production_insert_frost_surface_scalar(surface, symbol.as_str(), value)?;
    }
    Ok(())
}

fn direct_production_seed_frost_surface_layers(
    surface: &mut DirectFrostRunoffSurface,
    lane_index: usize,
    layers: &[DirectSubsurfaceLayerState],
    soil_water_m: f64,
) -> Result<(), HillslopeCliError> {
    if !soil_water_m.is_finite() || soil_water_m < 0.0 {
        return Err(direct_production_executor_blocked(format!(
            "direct production lane {} frost soil-water carry must be finite and nonnegative, observed {soil_water_m}",
            lane_index + 1
        )));
    }
    let nsl = usize_to_scalar("direct_production.frost_nsl", layers.len())?;
    direct_production_insert_frost_surface_scalar(surface, "wb11_nsl", nsl)?;
    direct_production_insert_frost_surface_scalar(surface, "nsl", nsl)?;
    direct_production_insert_frost_surface_scalar(surface, "wb11_soil_water", soil_water_m)?;
    for (layer_offset, layer) in layers.iter().enumerate() {
        let layer_index = layer_offset + 1;
        for (symbol, value) in direct_production_frost_layer_seed_scalars(layer_index, layer) {
            direct_production_insert_frost_surface_scalar(surface, symbol.as_str(), value)?;
        }
    }
    Ok(())
}

fn direct_production_frost_soil_conductivity(
    surface: &DirectFrostRunoffSurface,
    layers: &[DirectSubsurfaceLayerState],
) -> Result<f64, HillslopeCliError> {
    if let Some(value) = surface.optional_scalar("wb14_soil_conductivity_m_s") {
        if !value.is_finite() || value < 0.0 {
            return Err(direct_production_executor_blocked(format!(
                "direct production frost soil conductivity must be finite and nonnegative, observed {value}"
            )));
        }
        if value > 0.0 {
            return Ok(value);
        }
    }
    layers
        .first()
        .map(|layer| layer.conductivity_m_s)
        .ok_or_else(|| {
            direct_production_executor_blocked(
                "direct production active frost requires at least one layer conductivity",
            )
        })
}

fn direct_production_insert_frost_surface_scalar(
    surface: &mut DirectFrostRunoffSurface,
    symbol: &str,
    value: f64,
) -> Result<(), HillslopeCliError> {
    if !value.is_finite() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct production frost symbol {symbol} is non-finite ({value})"
            ),
        });
    }
    surface.insert_scalar(symbol, value);
    Ok(())
}

fn direct_production_frost_layer_seed_scalars(
    layer_index: usize,
    layer: &DirectSubsurfaceLayerState,
) -> [(String, f64); 6] {
    [
        (format!("wb18_perc_theta_{layer_index:04}"), layer.theta_m),
        (
            format!("wb18_perc_ul_{layer_index:04}"),
            layer.upper_limit_m,
        ),
        (format!("wb19_dg_{layer_index:04}"), layer.depth_m),
        (
            format!("wb19_thetdr_{layer_index:04}"),
            layer.residual_theta,
        ),
        (
            format!("wb18_perc_frozen_depth_{layer_index:04}"),
            layer.frozen_depth_m,
        ),
        (
            format!("wb18_perc_frzw_{layer_index:04}"),
            layer.frozen_water_m,
        ),
    ]
}

fn direct_production_hourly_symbol(root: &str, hour: usize) -> String {
    format!("{root}_{hour:04}")
}

fn direct_production_required_snow_state_scalar(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &'static str,
    minimum: Option<f64>,
    maximum: Option<f64>,
) -> Result<f64, HillslopeCliError> {
    let value = require_runtime_surface_scalar(runtime_surface, symbol)?;
    if !value.is_finite() || minimum.is_some_and(|minimum| value < minimum)
        || maximum.is_some_and(|maximum| value > maximum)
    {
        let lower = minimum.map_or("-inf".to_string(), |value| value.to_string());
        let upper = maximum.map_or("inf".to_string(), |value| value.to_string());
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} {symbol} must be finite and within [{lower}, {upper}] for direct production snow state, observed {value}"
            ),
        });
    }
    Ok(value)
}

fn direct_production_hyetograph(
    forcing: &HillslopeDirectClimateDayForcing,
) -> Result<Vec<DirectWb14HyetographInterval>, HillslopeCliError> {
    if forcing.timem_s.is_empty() && forcing.intsty_m_s.is_empty() {
        return Ok(vec![DirectWb14HyetographInterval {
            start_s: 0.0,
            end_s: 1.0,
            intensity_m_s: 0.0,
        }]);
    }
    if forcing.timem_s.len() != forcing.intsty_m_s.len() || forcing.timem_s.len() < 2 {
        return Err(direct_production_executor_blocked(format!(
            "direct production typed hyetograph requires matching timem/intsty vectors with at least two points, observed timem={} intsty={}",
            forcing.timem_s.len(),
            forcing.intsty_m_s.len()
        )));
    }
    let mut intervals = Vec::with_capacity(forcing.timem_s.len() - 1);
    for point_index in 0..forcing.timem_s.len() - 1 {
        let start_s = forcing.timem_s[point_index];
        let end_s = forcing.timem_s[point_index + 1];
        let intensity_m_s = forcing.intsty_m_s[point_index];
        if !start_s.is_finite()
            || !end_s.is_finite()
            || !intensity_m_s.is_finite()
            || end_s < start_s
            || intensity_m_s < 0.0
        {
            return Err(direct_production_executor_blocked(format!(
                "direct production typed hyetograph point {} is invalid: start={start_s} end={end_s} intensity={intensity_m_s}",
                point_index + 1
            )));
        }
        intervals.push(DirectWb14HyetographInterval {
            start_s,
            end_s,
            intensity_m_s,
        });
    }
    Ok(intervals)
}

fn direct_production_lane_soil_water(
    lane: &openwepp_hillslope_orchestrator::DirectLaneFrame,
    lane_index: usize,
) -> Result<f64, HillslopeCliError> {
    if !lane.water.soil_water_m.is_finite() || lane.water.soil_water_m < 0.0 {
        return Err(direct_production_executor_blocked(format!(
            "direct production lane {} soil-water carry must be finite and nonnegative, observed {}",
            lane_index + 1,
            lane.water.soil_water_m
        )));
    }
    Ok(lane.water.soil_water_m)
}

fn direct_production_validate_layers(
    lane_index: usize,
    layers: &[DirectSubsurfaceLayerState],
) -> Result<(), HillslopeCliError> {
    if layers.is_empty() {
        return Err(direct_production_executor_blocked(format!(
            "direct production lane {} requires typed subsurface layer state",
            lane_index + 1
        )));
    }
    Ok(())
}

fn direct_production_profile_depth_m(
    layers: &[DirectSubsurfaceLayerState],
) -> Result<f64, HillslopeCliError> {
    let profile_depth_m = layers.iter().map(|layer| layer.depth_m).sum::<f64>();
    if profile_depth_m <= 0.0 {
        return Err(direct_production_executor_blocked(
            "direct production PMET soil profile depth must be > 0.0",
        ));
    }
    Ok(profile_depth_m)
}

fn direct_production_frost_depth_m(layers: &[DirectSubsurfaceLayerState]) -> f64 {
    let mut depth_top_m = 0.0_f64;
    let mut frost_depth_m = 0.0_f64;
    for layer in layers {
        if layer.frozen_depth_m > 1.0e-12 {
            frost_depth_m = depth_top_m + layer.frozen_depth_m;
        }
        depth_top_m += layer.depth_m;
    }
    frost_depth_m
}

fn direct_publication_erosion_wave2_active(
    seed_surface: &HillslopeWritebackSurface,
    hyetograph: &[DirectWb14HyetographInterval],
) -> Result<bool, HillslopeCliError> {
    let wave2_enabled = parse_mofe03_binary_flag(
        "erod14_wave2_enabled",
        runtime_surface_symbol_value(seed_surface, "erod14_wave2_enabled").unwrap_or(0.0),
    )?;
    let rainfall_m = direct_publication_hyetograph_rainfall_m(hyetograph)?;
    Ok(
        wave2_enabled
            && rainfall_m >= DIRECT_PUBLICATION_EROSION_MIN_POST_INTERCEPTION_RAINFALL_M,
    )
}

fn direct_publication_erosion_inputs(
    seed_surface: &HillslopeWritebackSurface,
) -> Result<DirectErosionInputs, HillslopeCliError> {
    let wave1_enabled =
        direct_publication_optional_enabled_flag(seed_surface, "erod13_core_enabled")?
            .unwrap_or(false);
    Ok(DirectErosionInputs {
        wave1_enabled,
        wave2_enabled: true,
        wave1: if wave1_enabled {
            direct_publication_erod13_inputs(seed_surface)?
        } else {
            DirectErod13Inputs::zero()
        },
        wave2: direct_publication_erod14_inputs(seed_surface)?,
    })
}

fn direct_publication_peak_runoff_inputs(
    seed_surface: &HillslopeWritebackSurface,
    hyetograph: Vec<DirectWb14HyetographInterval>,
) -> Result<DirectPeakRunoffInputs, HillslopeCliError> {
    Ok(DirectPeakRunoffInputs {
        hyetograph,
        irrigation_rate_m_s: direct_publication_optional_nonnegative_scalar(
            seed_surface,
            &["irrigation.runtime_rate_m_per_s"],
        )?
        .unwrap_or(0.0),
        efflen_m: require_runtime_surface_scalar(seed_surface, "efflen")?,
        ealpha: require_runtime_surface_scalar(seed_surface, "ealpha")?,
        exponent_m: require_runtime_surface_scalar(seed_surface, "m")?,
    })
}

fn direct_publication_erod13_inputs(
    seed_surface: &HillslopeWritebackSurface,
) -> Result<DirectErod13Inputs, HillslopeCliError> {
    Ok(DirectErod13Inputs {
        ie_m_s: require_runtime_surface_scalar(seed_surface, "Ie")?,
        te_s: require_runtime_surface_scalar(seed_surface, "te")?,
        fs: require_runtime_surface_scalar(seed_surface, "fs")?,
        ft: require_runtime_surface_scalar(seed_surface, "ft")?,
        taufe_pa: require_runtime_surface_scalar(seed_surface, "taufe")?,
        q_m2_s: require_runtime_surface_scalar(seed_surface, "q")?,
        g_kg_s_m: require_runtime_surface_scalar(seed_surface, "G")?,
        di_kg_s_m2: require_runtime_surface_scalar(seed_surface, "Di")?,
        beta: require_runtime_surface_scalar(seed_surface, "beta")?,
        vf_m_s: require_runtime_surface_scalar(seed_surface, "vf")?,
        dgdx_kg_s_m2: require_runtime_surface_scalar(seed_surface, "dGdx")?,
        cntlen_m: require_runtime_surface_scalar(seed_surface, "cntlen")?,
        kr_s_m: require_runtime_surface_scalar(seed_surface, "kr")?,
        kradjf: require_runtime_surface_scalar(seed_surface, "kradjf")?,
        tcadjf: require_runtime_surface_scalar(seed_surface, "tcadjf")?,
        shrsol_pa: require_runtime_surface_scalar(seed_surface, "shrsol")?,
        tcend_kg_s_m: require_runtime_surface_scalar(seed_surface, "tcend")?,
        shcrit_pa: require_runtime_surface_scalar(seed_surface, "shcrit")?,
        detinr_kg_s_m2: require_runtime_surface_scalar(seed_surface, "detinr")?,
        effdrr_m: require_runtime_surface_scalar(seed_surface, "effdrr")?,
        effdrn_m: require_runtime_surface_scalar(seed_surface, "effdrn")?,
        veleff_m_s: require_runtime_surface_scalar(seed_surface, "veleff")?,
        pkro_m3_s: require_runtime_surface_scalar(seed_surface, "pkro")?,
        tc_k: require_runtime_surface_scalar(seed_surface, "erod13_tc_k")?,
        tc_m: require_runtime_surface_scalar(seed_surface, "erod13_tc_m")?,
        q_runoff_m: 0.0,
        peakro_m3_s: 0.0,
        watdur_s: 0.0,
    })
}

fn direct_publication_erod14_inputs(
    seed_surface: &HillslopeWritebackSurface,
) -> Result<DirectErod14Inputs, HillslopeCliError> {
    let slplen_m = require_runtime_surface_scalar(seed_surface, "erod14_slplen")?;
    let hbp_sediment_concentration_scale =
        require_runtime_surface_scalar(seed_surface, "efflen")? / slplen_m;
    let class_count = scalar_to_usize(
        "erod14_class_count",
        require_runtime_surface_scalar(seed_surface, "erod14_class_count")?,
    )?;
    let mut classes = Vec::with_capacity(class_count);
    for class_index in 1..=class_count {
        classes.push(DirectErod14ClassInputs {
            fall_m_s: require_runtime_surface_scalar(
                seed_surface,
                direct_publication_erod14_class_symbol("erod14_fall", class_index).as_str(),
            )?,
            frcflw: require_runtime_surface_scalar(
                seed_surface,
                direct_publication_erod14_class_symbol("erod14_frcflw", class_index).as_str(),
            )?,
            frac: require_runtime_surface_scalar(
                seed_surface,
                direct_publication_erod14_class_symbol("erod14_frac", class_index).as_str(),
            )?,
            fidel: require_runtime_surface_scalar(
                seed_surface,
                direct_publication_erod14_class_symbol("erod14_fidel", class_index).as_str(),
            )?,
            tcf1: require_runtime_surface_scalar(
                seed_surface,
                direct_publication_erod14_class_symbol("erod14_tcf1", class_index).as_str(),
            )?,
            ssa_class: require_runtime_surface_scalar(
                seed_surface,
                direct_publication_erod14_class_symbol("erod14_ssa_class", class_index).as_str(),
            )?,
        });
    }
    Ok(DirectErod14Inputs {
        xtop_m: require_runtime_surface_scalar(seed_surface, "erod14_xtop")?,
        xbot_m: require_runtime_surface_scalar(seed_surface, "erod14_xbot")?,
        xdetst_m: require_runtime_surface_scalar(seed_surface, "erod14_xdetst")?,
        ldtop_kg_s_m: require_runtime_surface_scalar(seed_surface, "erod14_ldtop")?,
        ldbot_kg_s_m: require_runtime_surface_scalar(seed_surface, "erod14_ldbot")?,
        lddend_kg: require_runtime_surface_scalar(seed_surface, "erod14_lddend")?,
        qout_m3_s: require_runtime_surface_scalar(seed_surface, "erod14_qout")?,
        qin_m3_s: require_runtime_surface_scalar(seed_surface, "erod14_qin")?,
        qostar_m: require_runtime_surface_scalar(seed_surface, "erod14_qostar")?,
        hbp_sediment_concentration_scale,
        slplen_m,
        ktrato: require_runtime_surface_scalar(seed_surface, "erod14_ktrato")?,
        aintc: require_runtime_surface_scalar(seed_surface, "erod14_ainftc")?,
        bintc: require_runtime_surface_scalar(seed_surface, "erod14_binftc")?,
        cintc: require_runtime_surface_scalar(seed_surface, "erod14_cinftc")?,
        beta: require_runtime_surface_scalar(seed_surface, "erod14_beta")?,
        qj_minus_1_m3_s: require_runtime_surface_scalar(seed_surface, "erod14_Qj_minus_1")?,
        vj_m: require_runtime_surface_scalar(seed_surface, "erod14_Vj")?,
        qj_m3_s: require_runtime_surface_scalar(seed_surface, "erod14_Qj")?,
        fh_m: require_runtime_surface_scalar(seed_surface, "erod14_Fh")?,
        fp_m: require_runtime_surface_scalar(seed_surface, "erod14_Fp")?,
        case_value: require_runtime_surface_scalar(seed_surface, "erod14_case")?,
        peak_runoff_m3_s: 0.0,
        runoff_duration_s: 0.0,
        ssa_soil: require_runtime_surface_scalar(seed_surface, "erod14_ssa_soil")?,
        theta: require_runtime_surface_scalar(seed_surface, "theta")?,
        classes,
    })
}

fn direct_publication_erod14_class_symbol(root: &str, class_index: usize) -> String {
    format!("{root}_{class_index:04}")
}

fn direct_publication_day_zero_seed_surface(
    climate_request: &HillslopeClimateRuntimeRequest,
    climate_span: &ClimateRunSpanSummary,
    seed_authority: &HillslopeWritebackSurface,
    climate_context_surface: &HillslopeWritebackSurface,
    execution_lane: ExecutionLane,
) -> Result<HillslopeWritebackSurface, HillslopeCliError> {
    let day = climate_span.days.first().ok_or_else(|| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!("{SIMOUT_GUARD_ID} direct publication requires at least one climate day"),
        }
    })?;
    direct_publication_validate_day(day)?;
    let mut seed_surface = seed_authority.clone();
    let mut climate_surface = build_day_climate_surface(climate_request, 0, climate_context_surface, day)?;
    seed_surface = crate::hillslope::intake_lane_setup::merge_runtime_surfaces(
        seed_surface,
        std::mem::take(&mut climate_surface),
    );
    seed_wb11_runtime_surface_inputs(&mut seed_surface, execution_lane)?;
    Ok(seed_surface)
}

fn direct_publication_validate_day(day: &ClimateDayProjection) -> Result<(), HillslopeCliError> {
    if !day.precipitation_mm.is_finite() || day.precipitation_mm < 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct publication precipitation must be finite and >= 0.0, observed {}",
                day.precipitation_mm
            ),
        });
    }
    if !day.effective_temperature_c.is_finite() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct publication effective temperature must be finite, observed {}",
                day.effective_temperature_c
            ),
        });
    }
    Ok(())
}

fn overlay_direct_publication_lane_state(
    seed_surface: &mut HillslopeWritebackSurface,
    day_index: usize,
    lane_index: usize,
    lane: &openwepp_hillslope_orchestrator::DirectLaneFrame,
) -> Result<(), HillslopeCliError> {
    if lane.subsurface_layers.is_empty() {
        if day_index == 0 {
            return Ok(());
        }
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct publication day {} lane {} requires committed direct-carried layers before PMET construction",
                day_index + 1,
                lane_index + 1
            ),
        });
    }
    let nsl = lane.subsurface_layers.len();
    let nsl_u32 = u32::try_from(nsl).map_err(|_| HillslopeCliError::RuntimeSurfaceFailure {
        surface: "direct_publication_frame",
        detail: format!(
            "{SIMOUT_GUARD_ID} direct publication lane {} layer count {nsl} exceeds u32 range",
            lane_index + 1
        ),
    })?;
    let nsl_value = f64::from(nsl_u32);
    insert_direct_seed_scalar(seed_surface, "wb11_nsl", nsl_value, lane_index)?;
    insert_direct_seed_scalar(seed_surface, "nsl", nsl_value, lane_index)?;
    let layer_summary =
        overlay_direct_publication_layer_state(seed_surface, lane_index, &lane.subsurface_layers)?;
    if !lane.water.soil_water_m.is_finite() || lane.water.soil_water_m < 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct publication lane {} soil water carry must be finite and non-negative, observed {}",
                lane_index + 1,
                lane.water.soil_water_m
            ),
        });
    }
    let soil_water_m = lane.water.soil_water_m;
    if (layer_summary.aggregate_soil_water - soil_water_m).abs() > 1.0e-9 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct publication lane {} layer aggregate {} diverges from storage carry {}",
                lane_index + 1,
                layer_summary.aggregate_soil_water,
                soil_water_m
            ),
        });
    }
    if let Some(carry) = &lane.frost_runtime_carry {
        insert_direct_seed_scalar(seed_surface, "wb11_soil_water", soil_water_m, lane_index)?;
        overlay_direct_publication_frost_runtime_carry(seed_surface, lane_index, carry)?;
        return Ok(());
    }
    overlay_direct_publication_frost_fine_state(
        seed_surface,
        lane_index,
        &lane.subsurface_layers,
    )?;
    for (symbol, value) in [
        ("wb11_soil_water", soil_water_m),
        ("frost.runtime_ws_frz", layer_summary.frozen_water),
        (
            "frost.runtime_frwatc_frozen_water_after_m",
            layer_summary.frozen_water,
        ),
        ("frost.runtime_frdp_m", layer_summary.frost_depth),
        ("frost.runtime_dfrost", layer_summary.frost_depth),
    ] {
        insert_direct_seed_scalar(seed_surface, symbol, value, lane_index)?;
    }
    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct DirectPublicationLayerOverlaySummary {
    aggregate_soil_water: f64,
    frozen_water: f64,
    frost_depth: f64,
}

fn overlay_direct_publication_layer_state(
    seed_surface: &mut HillslopeWritebackSurface,
    lane_index: usize,
    layers: &[DirectSubsurfaceLayerState],
) -> Result<DirectPublicationLayerOverlaySummary, HillslopeCliError> {
    let mut aggregate_soil_water = 0.0_f64;
    let mut frozen_water = 0.0_f64;
    let mut frost_depth = 0.0_f64;
    let mut layer_top_m = 0.0_f64;
    for (layer_offset, layer) in layers.iter().enumerate() {
        let layer_index = layer_offset + 1;
        let unfrozen_depth_m = (layer.depth_m - layer.frozen_depth_m).max(0.0);
        aggregate_soil_water += layer.theta_m + layer.residual_theta * unfrozen_depth_m;
        frozen_water += layer.frozen_water_m;
        if layer.frozen_depth_m > 1.0e-12 {
            frost_depth = layer_top_m + layer.frozen_depth_m;
        }
        for (symbol, value) in direct_publication_layer_seed_scalars(layer_index, layer) {
            insert_direct_seed_scalar(seed_surface, symbol.as_str(), value, lane_index)?;
        }
        layer_top_m += layer.depth_m;
    }
    Ok(DirectPublicationLayerOverlaySummary {
        aggregate_soil_water,
        frozen_water,
        frost_depth,
    })
}

fn direct_publication_layer_seed_scalars(
    layer_index: usize,
    layer: &DirectSubsurfaceLayerState,
) -> [(String, f64); 12] {
    [
        (format!("wb18_perc_theta_{layer_index:04}"), layer.theta_m),
        (
            format!("wb18_perc_fc_{layer_index:04}"),
            layer.field_capacity_m,
        ),
        (
            format!("wb18_perc_ul_{layer_index:04}"),
            layer.upper_limit_m,
        ),
        (
            format!("wb18_perc_ssc_{layer_index:04}"),
            layer.conductivity_m_s,
        ),
        (format!("wb19_dg_{layer_index:04}"), layer.depth_m),
        (
            format!("wb19_thetdr_{layer_index:04}"),
            layer.residual_theta,
        ),
        (
            format!("wb18_perc_frozen_depth_{layer_index:04}"),
            layer.frozen_depth_m,
        ),
        (
            format!("wb18_perc_frzw_{layer_index:04}"),
            layer.frozen_water_m,
        ),
        (format!("wb19_por_{layer_index:04}"), layer.porosity),
        (
            format!("wb19_thetfc_{layer_index:04}"),
            layer.field_capacity_theta,
        ),
        (format!("wb19_coca_{layer_index:04}"), layer.coca),
        (format!("coca_{layer_index:04}"), layer.coca),
    ]
}

fn overlay_direct_publication_frost_fine_state(
    seed_surface: &mut HillslopeWritebackSurface,
    lane_index: usize,
    layers: &[DirectSubsurfaceLayerState],
) -> Result<(), HillslopeCliError> {
    if runtime_surface_symbol_value(seed_surface, "frost.options.fineTop").is_none()
        || runtime_surface_symbol_value(seed_surface, "frost.options.fineBot").is_none()
    {
        return Ok(());
    }
    let layer_count = layers.len();
    let fine_top_count =
        direct_publication_frost_fine_count(seed_surface, "frost.options.fineTop")?;
    let fine_bot_count =
        direct_publication_frost_fine_count(seed_surface, "frost.options.fineBot")?;
    for (layer_offset, layer) in layers.iter().enumerate() {
        let layer_index = layer_offset + 1;
        let fine_layer_count = direct_publication_frost_fine_layer_count(
            layer_index,
            layer_count,
            layer.depth_m,
            fine_top_count,
            fine_bot_count,
        )?;
        let fine_layer_thickness_m =
            layer.depth_m / usize_to_scalar("frost.runtime_nfine", fine_layer_count)?;
        let mut remaining_frozen_depth_m = layer.frozen_depth_m;
        let soilf_m = layer.frozen_water_m + layer.residual_theta * layer.frozen_depth_m;
        let ice_per_frozen_m = if layer.frozen_depth_m > 1.0e-12 {
            soilf_m / layer.frozen_depth_m
        } else {
            0.0
        };
        let unfrozen_depth_m = (layer.depth_m - layer.frozen_depth_m).max(0.0);
        let raw_slsw_theta = if unfrozen_depth_m > 1.0e-12 {
            layer.residual_theta + layer.theta_m / unfrozen_depth_m
        } else {
            layer.residual_theta
        };
        let slsw_theta_capacity = layer.residual_theta + layer.upper_limit_m / layer.depth_m;
        let slsw_theta = raw_slsw_theta
            .max(layer.residual_theta)
            .min(slsw_theta_capacity);
        for fine_index in 1..=fine_layer_count {
            let slfsd_m = remaining_frozen_depth_m
                .min(fine_layer_thickness_m)
                .max(0.0);
            remaining_frozen_depth_m = (remaining_frozen_depth_m - slfsd_m).max(0.0);
            let fgfrst = if slfsd_m >= fine_layer_thickness_m - 1.0e-12 {
                1.0
            } else if slfsd_m > 1.0e-12 {
                2.0
            } else {
                0.0
            };
            for (symbol, value) in [
                (
                    format!("frost.runtime_fgfrst_{layer_index:04}_{fine_index:04}"),
                    fgfrst,
                ),
                (
                    format!("frost.runtime_slfsd_m_{layer_index:04}_{fine_index:04}"),
                    slfsd_m,
                ),
                (
                    format!("frost.runtime_slsic_m_{layer_index:04}_{fine_index:04}"),
                    ice_per_frozen_m * slfsd_m,
                ),
                (
                    format!("frost.runtime_slsw_theta_{layer_index:04}_{fine_index:04}"),
                    slsw_theta,
                ),
                (
                    format!("frost.runtime_sltime_s_{layer_index:04}_{fine_index:04}"),
                    0.0,
                ),
            ] {
                insert_direct_seed_scalar(seed_surface, symbol.as_str(), value, lane_index)?;
            }
        }
    }
    Ok(())
}

fn direct_publication_has_frost_runtime_carry(
    seed_surface: &HillslopeWritebackSurface,
) -> Result<bool, HillslopeCliError> {
    direct_publication_optional_enabled_flag(
        seed_surface,
        "frost.direct_runtime_carry_present",
    )
    .map(|value| value.unwrap_or(false))
}

fn direct_publication_frost_runtime_carry_has_fine_projection(
    carry: &DirectFrostRuntimeCarry,
) -> bool {
    !carry.fine_layers.is_empty()
}

fn overlay_direct_publication_frost_runtime_carry(
    seed_surface: &mut HillslopeWritebackSurface,
    lane_index: usize,
    carry: &DirectFrostRuntimeCarry,
) -> Result<(), HillslopeCliError> {
    insert_direct_frost_runtime_scalars(
        seed_surface,
        lane_index,
        DirectFrostRuntimeScalarSeed {
            dfrost_m: carry.dfrost_m,
            dthaw_m: carry.dthaw_m,
            nft: carry.nft,
            ws_frz_m: carry.ws_frz_m,
            infcap_frz_m_s: carry.infcap_frz_m_s,
            frwatc_soil_water_before_m: carry.frwatc_soil_water_before_m,
            frwatc_soil_water_after_m: carry.frwatc_soil_water_after_m,
            frwatc_frozen_water_before_m: carry.frwatc_frozen_water_before_m,
            frwatc_frozen_water_after_m: carry.frwatc_frozen_water_after_m,
            frwatc_freeze_debit_m: carry.frwatc_freeze_debit_m,
            frwatc_thaw_credit_m: carry.frwatc_thaw_credit_m,
            frwatc_net_liquid_delta_m: carry.frwatc_net_liquid_delta_m,
            frdp_m: carry.frdp_m,
            thdp_m: carry.thdp_m,
            tfrdp_m: carry.tfrdp_m,
            tthawd_m: carry.tthawd_m,
            fgthwd_flag: carry.fgthwd_flag,
            total_fine_layer_count: carry.total_fine_layer_count,
            conductivity_tilled_w_m_k: carry.conductivity_tilled_w_m_k,
            conductivity_untilled_w_m_k: carry.conductivity_untilled_w_m_k,
            conductivity_residue_w_m_k: carry.conductivity_residue_w_m_k,
            shadow_total_water_before_m: carry.shadow_total_water_before_m,
            shadow_total_water_after_m: carry.shadow_total_water_after_m,
            shadow_wb_delta_m: carry.shadow_wb_delta_m,
            shadow_frwatc_residual_m: carry.shadow_frwatc_residual_m,
            watpdg_m: carry.watpdg_m,
            watbtm_m: carry.watbtm_m,
            fine_projection: direct_publication_frost_runtime_carry_has_fine_projection(carry),
        },
    )?;
    for layer in &carry.layer_shadows {
        insert_direct_frost_layer_shadow(seed_surface, lane_index, *layer)?;
    }
    for fine in &carry.fine_layers {
        insert_direct_frost_fine_layer(seed_surface, lane_index, *fine)?;
    }
    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct DirectFrostRuntimeScalarSeed {
    dfrost_m: f64,
    dthaw_m: f64,
    nft: f64,
    ws_frz_m: f64,
    infcap_frz_m_s: f64,
    frwatc_soil_water_before_m: f64,
    frwatc_soil_water_after_m: f64,
    frwatc_frozen_water_before_m: f64,
    frwatc_frozen_water_after_m: f64,
    frwatc_freeze_debit_m: f64,
    frwatc_thaw_credit_m: f64,
    frwatc_net_liquid_delta_m: f64,
    frdp_m: f64,
    thdp_m: f64,
    tfrdp_m: f64,
    tthawd_m: f64,
    fgthwd_flag: f64,
    total_fine_layer_count: f64,
    conductivity_tilled_w_m_k: f64,
    conductivity_untilled_w_m_k: f64,
    conductivity_residue_w_m_k: f64,
    shadow_total_water_before_m: f64,
    shadow_total_water_after_m: f64,
    shadow_wb_delta_m: f64,
    shadow_frwatc_residual_m: f64,
    watpdg_m: f64,
    watbtm_m: f64,
    fine_projection: bool,
}

fn insert_direct_frost_runtime_scalars(
    seed_surface: &mut HillslopeWritebackSurface,
    lane_index: usize,
    seed: DirectFrostRuntimeScalarSeed,
) -> Result<(), HillslopeCliError> {
    for (symbol, value) in [
        (
            "frost.direct_runtime_carry_present",
            if seed.fine_projection { 1.0 } else { 0.0 },
        ),
        ("frost.runtime_dfrost", seed.dfrost_m),
        ("frost.runtime_dthaw", seed.dthaw_m),
        ("frost.runtime_nft", seed.nft),
        ("frost.runtime_ws_frz", seed.ws_frz_m),
        ("frost.runtime_infcap_frz", seed.infcap_frz_m_s),
        (
            "frost.runtime_frwatc_soil_water_before_m",
            seed.frwatc_soil_water_before_m,
        ),
        (
            "frost.runtime_frwatc_soil_water_after_m",
            seed.frwatc_soil_water_after_m,
        ),
        (
            "frost.runtime_frwatc_frozen_water_before_m",
            seed.frwatc_frozen_water_before_m,
        ),
        (
            "frost.runtime_frwatc_frozen_water_after_m",
            seed.frwatc_frozen_water_after_m,
        ),
        (
            "frost.runtime_frwatc_freeze_debit_m",
            seed.frwatc_freeze_debit_m,
        ),
        (
            "frost.runtime_frwatc_thaw_credit_m",
            seed.frwatc_thaw_credit_m,
        ),
        (
            "frost.runtime_frwatc_net_liquid_delta_m",
            seed.frwatc_net_liquid_delta_m,
        ),
        ("frost.runtime_frdp_m", seed.frdp_m),
        ("frost.runtime_thdp_m", seed.thdp_m),
        ("frost.runtime_tfrdp_m", seed.tfrdp_m),
        ("frost.runtime_tthawd_m", seed.tthawd_m),
        ("frost.runtime_fgthwd_flag", seed.fgthwd_flag),
        (
            "frost.runtime_total_fine_layer_count",
            seed.total_fine_layer_count,
        ),
        ("frost.runtime_kftill_w_m_k", seed.conductivity_tilled_w_m_k),
        (
            "frost.runtime_kfutil_w_m_k",
            seed.conductivity_untilled_w_m_k,
        ),
        ("frost.runtime_kres_w_m_k", seed.conductivity_residue_w_m_k),
        (
            "frost.runtime_shadow_total_water_before_m",
            seed.shadow_total_water_before_m,
        ),
        (
            "frost.runtime_shadow_total_water_after_m",
            seed.shadow_total_water_after_m,
        ),
        ("frost.runtime_shadow_wb_delta_m", seed.shadow_wb_delta_m),
        (
            "frost.runtime_shadow_frwatc_residual_m",
            seed.shadow_frwatc_residual_m,
        ),
        ("frost.runtime_watpdg_m", seed.watpdg_m),
        ("frost.runtime_watbtm_m", seed.watbtm_m),
    ] {
        insert_direct_seed_scalar(seed_surface, symbol, value, lane_index)?;
    }
    Ok(())
}

fn insert_direct_frost_layer_shadow(
    seed_surface: &mut HillslopeWritebackSurface,
    lane_index: usize,
    layer: DirectFrostLayerShadowCarry,
) -> Result<(), HillslopeCliError> {
    let layer_symbols: [(String, f64); 7] = [
        (
            format!("frost.runtime_shadow_st_m_{:04}", layer.layer_index),
            layer.st_m,
        ),
        (
            format!(
                "frost.runtime_shadow_soil_water_m_{:04}",
                layer.layer_index
            ),
            layer.soil_water_m,
        ),
        (
            format!(
                "frost.runtime_shadow_frozen_depth_m_{:04}",
                layer.layer_index
            ),
            layer.frozen_depth_m,
        ),
        (
            format!("frost.runtime_shadow_frzw_m_{:04}", layer.layer_index),
            layer.frozen_water_m,
        ),
        (
            format!("frost.runtime_shadow_soilf_m_{:04}", layer.layer_index),
            layer.soilf_m,
        ),
        (
            format!("frost.runtime_yst_m_{:04}", layer.layer_index),
            layer.yst_m,
        ),
        (
            format!("frost.runtime_nwfrzz_m_{:04}", layer.layer_index),
            layer.nwfrzz_m,
        ),
    ];
    for (symbol, value) in layer_symbols {
        insert_direct_seed_scalar(seed_surface, symbol.as_str(), value, lane_index)?;
    }
    Ok(())
}

fn insert_direct_frost_fine_layer(
    seed_surface: &mut HillslopeWritebackSurface,
    lane_index: usize,
    fine: DirectFrostFineLayerCarry,
) -> Result<(), HillslopeCliError> {
    for (symbol, value) in [
        (
            format!(
                "frost.runtime_fgfrst_{:04}_{:04}",
                fine.layer_index, fine.fine_index
            ),
            fine.fgfrst,
        ),
        (
            format!(
                "frost.runtime_slfsd_m_{:04}_{:04}",
                fine.layer_index, fine.fine_index
            ),
            fine.slfsd_m,
        ),
        (
            format!(
                "frost.runtime_slsic_m_{:04}_{:04}",
                fine.layer_index, fine.fine_index
            ),
            fine.slsic_m,
        ),
        (
            format!(
                "frost.runtime_slsw_theta_{:04}_{:04}",
                fine.layer_index, fine.fine_index
            ),
            fine.slsw_theta,
        ),
        (
            format!(
                "frost.runtime_sltime_s_{:04}_{:04}",
                fine.layer_index, fine.fine_index
            ),
            fine.sltime_s,
        ),
    ] {
        insert_direct_seed_scalar(seed_surface, symbol.as_str(), value, lane_index)?;
    }
    Ok(())
}

fn insert_direct_seed_scalar(
    seed_surface: &mut HillslopeWritebackSurface,
    symbol: &str,
    value: f64,
    lane_index: usize,
) -> Result<(), HillslopeCliError> {
    if !value.is_finite() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct publication lane {} carried symbol {symbol} is non-finite ({value})",
                lane_index + 1
            ),
        });
    }
    seed_surface
        .state_surface
        .insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
    Ok(())
}

fn direct_publication_percolation_inputs(
    runtime_surface: &HillslopeWritebackSurface,
    _precipitation_m: f64,
) -> Result<DirectPercolationInputs, HillslopeCliError> {
    let layers = direct_publication_layer_states(runtime_surface)?;
    let soil_water_initial_m = require_runtime_surface_scalar(runtime_surface, "wb11_soil_water")?;
    let lane_substeps = scalar_to_usize(
        "wb18_perc_lane_substeps",
        require_runtime_surface_scalar(runtime_surface, "wb18_perc_lane_substeps")?,
    )?;
    let restrictive_layer_enabled =
        direct_publication_optional_enabled_flag(runtime_surface, "slflag")?.unwrap_or(false);
    let restrictive_layer_conductivity_m_s = if restrictive_layer_enabled {
        direct_publication_required_positive_scalar(runtime_surface, "kslast")?
    } else {
        0.0
    };
    let restrictive_layer_thickness_m = if restrictive_layer_enabled && lane_substeps > 1 {
        direct_publication_required_positive_scalar(runtime_surface, "ui_bdrkth")?
    } else {
        0.0
    };
    Ok(DirectPercolationInputs {
        soil_water_initial_m,
        reconcile_legacy_soil_water_from_layers: false,
        same_pass_infiltration_m: 0.0,
        same_pass_infiltration_lineage: false,
        tillage_depth_m: 0.0,
        lane_substeps,
        restrictive_layer_enabled,
        restrictive_layer_conductivity_m_s,
        restrictive_layer_thickness_m,
        layers,
    })
}

fn direct_publication_liquid_input_inputs(
    liquid_input_handoff_m: f64,
) -> Result<DirectLiquidInputInputs, HillslopeCliError> {
    if liquid_input_handoff_m < 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} wb12_rainfall_input must be >= 0.0 for direct R4I liquid input, observed {liquid_input_handoff_m}"
            ),
        });
    }
    Ok(DirectLiquidInputInputs {
        liquid_input_handoff_m,
    })
}

fn direct_publication_storage_input_inputs(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<DirectStorageInputInputs, HillslopeCliError> {
    let precip_input_handoff_m = require_runtime_surface_scalar(runtime_surface, "wb12_precip_input")?;
    if precip_input_handoff_m < 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} wb12_precip_input must be >= 0.0 for direct R4C storage input, observed {precip_input_handoff_m}"
            ),
        });
    }
    Ok(DirectStorageInputInputs {
        precip_input_handoff_m: Some(precip_input_handoff_m),
    })
}

fn direct_publication_interception_state(
    runtime_surface: &HillslopeWritebackSurface,
    rainfall_input_m: f64,
    hyetograph: &[DirectWb14HyetographInterval],
) -> Result<openwepp_hillslope_orchestrator::DirectCanopyInterceptionState, HillslopeCliError> {
    let hyetograph_rainfall_m = direct_publication_hyetograph_rainfall_m(hyetograph)?;
    compute_direct_canopy_interception(DirectCanopyInterceptionInputs {
        hyetograph_rainfall_m,
        interception_rainfall_input_m: rainfall_input_m,
        canopy_cover_fraction: require_runtime_surface_scalar(runtime_surface, "cancov")?,
        leaf_area_index: require_runtime_surface_scalar(runtime_surface, "lai")?,
        vegetative_dry_matter_kg_m2: require_runtime_surface_scalar(runtime_surface, "vdmt")?,
    })
    .map_err(|source| direct_publication_runtime_error(&source))
}

fn direct_publication_snow_liquid_partition(
    runtime_surface: &HillslopeWritebackSurface,
    hyetograph: &[DirectWb14HyetographInterval],
) -> Result<openwepp_hillslope_orchestrator::DirectSnowLiquidPartition, HillslopeCliError> {
    let hyetograph_rainfall_m = direct_publication_hyetograph_rainfall_m(hyetograph)?;
    Wb11HydrologyKernel::compute_direct_snow_liquid_partition(
        &runtime_surface.state_surface,
        &runtime_surface.flux_surface,
        hyetograph_rainfall_m,
    )
    .map_err(|source| HillslopeCliError::RuntimeSurfaceFailure {
        surface: "direct_publication_frame",
        detail: format!("{SIMOUT_GUARD_ID} direct R4G snow/liquid partition failed: {source}"),
    })
}

fn direct_publication_frost_liquid_partition(
    runtime_surface: &HillslopeWritebackSurface,
    layers: &[DirectSubsurfaceLayerState],
) -> Result<openwepp_hillslope_orchestrator::DirectFrostLiquidPartition, HillslopeCliError> {
    let soil_conductivity_m_s = direct_publication_wb14_base_conductivity(runtime_surface, layers)?;
    Wb11HydrologyKernel::compute_direct_frost_liquid_partition(
        &runtime_surface.state_surface,
        &runtime_surface.flux_surface,
        soil_conductivity_m_s,
    )
    .map_err(|source| HillslopeCliError::RuntimeSurfaceFailure {
        surface: "direct_publication_frame",
        detail: format!("{SIMOUT_GUARD_ID} direct R4G frost/liquid partition failed: {source}"),
    })
}

fn apply_direct_publication_frost_infiltration_cap(
    runtime_surface: &mut HillslopeWritebackSurface,
    frost_partition: &openwepp_hillslope_orchestrator::DirectFrostLiquidPartition,
    lane_index: usize,
) -> Result<(), HillslopeCliError> {
    insert_direct_seed_scalar(
        runtime_surface,
        "frost.runtime_infcap_frz",
        frost_partition.infcap_frz_m_s,
        lane_index,
    )
}

fn direct_publication_hydrology_projection_inputs(
    mut profile_inputs: DirectHydrologyProjectionInputs,
    snow_liquid: &openwepp_hillslope_orchestrator::DirectSnowLiquidPartition,
) -> DirectHydrologyProjectionInputs {
    profile_inputs.snow_water_m = snow_liquid.runtime_swe_after_m;
    profile_inputs
}

fn direct_publication_infiltration_depression_inputs(
    runtime_surface: &HillslopeWritebackSurface,
    hyetograph: Vec<DirectWb14HyetographInterval>,
) -> Result<DirectInfiltrationDepressionInputs, HillslopeCliError> {
    let layers = direct_publication_layer_states(runtime_surface)?;
    let effective_conductivity_m_s =
        direct_publication_wb14_effective_conductivity(runtime_surface, &layers)?;
    let matric_potential_m = direct_publication_wb14_matric_potential(runtime_surface, &layers)?;
    let storage_capacity_m = direct_publication_wb14_top_storage_capacity(&layers)?;
    let depression_storage_capacity_m = direct_publication_optional_nonnegative_scalar(
        runtime_surface,
        &[
            "wb14_depression_storage_capacity_m",
            "wb12_depression_storage_capacity_m",
        ],
    )?
    .unwrap_or(0.0);

    Ok(DirectInfiltrationDepressionInputs {
        cumulative_infiltration_handoff_m: 0.0,
        depression_storage_delta_handoff_m: 0.0,
        producer_inputs: Some(DirectWb14InfiltrationProducerInputs {
            hyetograph,
            effective_conductivity_m_s,
            matric_potential_m,
            storage_capacity_m,
            depression_storage_capacity_m,
        }),
    })
}

fn direct_publication_hyetograph_rainfall_m(
    hyetograph: &[DirectWb14HyetographInterval],
) -> Result<f64, HillslopeCliError> {
    let mut total_m = 0.0_f64;
    for interval in hyetograph {
        let duration_s = interval.end_s - interval.start_s;
        if duration_s <= 0.0 || interval.intensity_m_s <= 0.0 {
            continue;
        }
        let rainfall_m = duration_s * interval.intensity_m_s;
        if !rainfall_m.is_finite() || rainfall_m < 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct WB15 hyetograph rainfall must be finite and >= 0.0, observed {rainfall_m}"
                ),
            });
        }
        total_m += rainfall_m;
        if !total_m.is_finite() {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct WB15 hyetograph rainfall total is non-finite"
                ),
            });
        }
    }
    Ok(total_m)
}

fn direct_publication_scaled_hyetograph(
    hyetograph: &[DirectWb14HyetographInterval],
    rainfall_scale: f64,
) -> Result<Vec<DirectWb14HyetographInterval>, HillslopeCliError> {
    if !rainfall_scale.is_finite() || rainfall_scale < 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct WB15 rainfall scale must be finite and >= 0.0, observed {rainfall_scale}"
            ),
        });
    }
    hyetograph
        .iter()
        .map(|interval| {
            let intensity_m_s = interval.intensity_m_s * rainfall_scale;
            if !intensity_m_s.is_finite() || intensity_m_s < 0.0 {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "direct_publication_frame",
                    detail: format!(
                        "{SIMOUT_GUARD_ID} direct WB15 scaled hyetograph intensity must be finite and >= 0.0, observed {intensity_m_s}"
                    ),
                });
            }
            Ok(DirectWb14HyetographInterval {
                start_s: interval.start_s,
                end_s: interval.end_s,
                intensity_m_s,
            })
        })
        .collect()
}

fn direct_publication_scaled_hyetograph_to_rainfall(
    hyetograph: &[DirectWb14HyetographInterval],
    target_rainfall_m: f64,
) -> Result<Vec<DirectWb14HyetographInterval>, HillslopeCliError> {
    if !target_rainfall_m.is_finite() || target_rainfall_m < 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct WB15 target rainfall must be finite and >= 0.0, observed {target_rainfall_m}"
            ),
        });
    }
    let source_rainfall_m = direct_publication_hyetograph_rainfall_m(hyetograph)?;
    if source_rainfall_m <= 0.0 {
        if target_rainfall_m <= 0.0 {
            return direct_publication_scaled_hyetograph(hyetograph, 0.0);
        }
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct WB15 cannot project positive target rainfall {target_rainfall_m} m from a zero-depth hyetograph"
            ),
        });
    }
    direct_publication_scaled_hyetograph(hyetograph, target_rainfall_m / source_rainfall_m)
}

fn direct_publication_hyetograph_with_added_daily_depth(
    hyetograph: &[DirectWb14HyetographInterval],
    added_depth_m: f64,
) -> Result<Vec<DirectWb14HyetographInterval>, HillslopeCliError> {
    if !added_depth_m.is_finite() || added_depth_m < 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct WB15 added daily liquid depth must be finite and >= 0.0, observed {added_depth_m}"
            ),
        });
    }
    if added_depth_m <= 0.0 {
        return Ok(hyetograph.to_vec());
    }
    let mut total_duration_s = 0.0_f64;
    for interval in hyetograph {
        let duration_s = interval.end_s - interval.start_s;
        if duration_s > 0.0 {
            total_duration_s += duration_s;
        }
    }
    if !total_duration_s.is_finite() || total_duration_s <= 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct WB15 added daily liquid depth requires positive hyetograph duration"
            ),
        });
    }
    let added_intensity_m_s = added_depth_m / total_duration_s;
    hyetograph
        .iter()
        .map(|interval| {
            let intensity_m_s = interval.intensity_m_s + added_intensity_m_s;
            if !intensity_m_s.is_finite() || intensity_m_s < 0.0 {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "direct_publication_frame",
                    detail: format!(
                        "{SIMOUT_GUARD_ID} direct WB15 liquid hyetograph intensity must be finite and >= 0.0, observed {intensity_m_s}"
                    ),
                });
            }
            Ok(DirectWb14HyetographInterval {
                start_s: interval.start_s,
                end_s: interval.end_s,
                intensity_m_s,
            })
        })
        .collect()
}

fn direct_publication_hyetograph(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<Vec<DirectWb14HyetographInterval>, HillslopeCliError> {
    let point_symbol = if runtime_surface_symbol_value(runtime_surface, "ninten").is_some() {
        "ninten"
    } else {
        "nbrkpt"
    };
    let point_count = scalar_to_usize(
        point_symbol,
        require_runtime_surface_scalar(runtime_surface, point_symbol)?,
    )?;
    if point_count < 2 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} WB14 direct hyetograph requires at least two time points, observed {point_count}"
            ),
        });
    }
    let mut intervals = Vec::with_capacity(point_count - 1);
    for point_index in 1..point_count {
        let start_symbol = wb13_primary_layer_symbol("timem", point_index);
        let end_symbol = wb13_primary_layer_symbol("timem", point_index + 1);
        let intensity_symbol = wb13_primary_layer_symbol("intsty", point_index);
        let start_s = require_runtime_surface_scalar(runtime_surface, start_symbol.as_str())?;
        let end_s = require_runtime_surface_scalar(runtime_surface, end_symbol.as_str())?;
        let intensity_m_s =
            require_runtime_surface_scalar(runtime_surface, intensity_symbol.as_str())?;
        intervals.push(DirectWb14HyetographInterval {
            start_s,
            end_s,
            intensity_m_s,
        });
    }
    Ok(intervals)
}

fn direct_publication_wb14_effective_conductivity(
    runtime_surface: &HillslopeWritebackSurface,
    layers: &[DirectSubsurfaceLayerState],
) -> Result<f64, HillslopeCliError> {
    if let Some(value) = direct_publication_optional_nonnegative_scalar(
        runtime_surface,
        &[
            "wb14_effective_conductivity_m_s",
            "frost.runtime_infcap_frz",
            "wb14_soil_conductivity_m_s",
        ],
    )? {
        if value > 0.0 {
            return Ok(value);
        }
    }
    layers
        .first()
        .map(|layer| layer.conductivity_m_s)
        .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} WB14 direct infiltration requires at least one layer conductivity"
            ),
        })
}

fn direct_publication_wb14_base_conductivity(
    runtime_surface: &HillslopeWritebackSurface,
    layers: &[DirectSubsurfaceLayerState],
) -> Result<f64, HillslopeCliError> {
    if let Some(value) =
        direct_publication_optional_nonnegative_scalar(runtime_surface, &["wb14_soil_conductivity_m_s"])?
    {
        if value > 0.0 {
            return Ok(value);
        }
    }
    layers
        .first()
        .map(|layer| layer.conductivity_m_s)
        .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} WB14 direct frost partition requires at least one layer conductivity"
            ),
        })
}

fn direct_publication_wb14_matric_potential(
    runtime_surface: &HillslopeWritebackSurface,
    layers: &[DirectSubsurfaceLayerState],
) -> Result<f64, HillslopeCliError> {
    if let Some(value) = direct_publication_optional_nonnegative_scalar(
        runtime_surface,
        &["wb14_matric_potential_m"],
    )? {
        return Ok(value);
    }
    let first_layer = layers
        .first()
        .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} WB14 direct infiltration requires at least one layer for matric potential"
            ),
        })?;
    Ok(first_layer.depth_m * (first_layer.field_capacity_theta - first_layer.residual_theta).max(0.0))
}

fn direct_publication_wb14_top_storage_capacity(
    layers: &[DirectSubsurfaceLayerState],
) -> Result<f64, HillslopeCliError> {
    if layers.is_empty() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} WB14 direct infiltration requires layer storage capacity"
            ),
        });
    }
    Ok(layers
        .iter()
        .take(2)
        .map(|layer| (layer.upper_limit_m - layer.frozen_water_m - layer.theta_m).max(0.0))
        .sum())
}

fn direct_publication_optional_nonnegative_scalar(
    runtime_surface: &HillslopeWritebackSurface,
    symbols: &[&str],
) -> Result<Option<f64>, HillslopeCliError> {
    for symbol in symbols {
        if let Some(value) = runtime_surface_symbol_value(runtime_surface, symbol) {
            if !value.is_finite() || value < 0.0 {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "direct_publication_frame",
                    detail: format!(
                        "{SIMOUT_GUARD_ID} {symbol} must be finite and >= 0.0 for WB14 direct infiltration, observed {value}"
                    ),
                });
            }
            return Ok(Some(value));
        }
    }
    Ok(None)
}

fn direct_publication_subsurface_inputs(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<DirectSubsurfaceComputeInputs, HillslopeCliError> {
    let layer_states = direct_publication_layer_states(runtime_surface)?;
    let soil_depth_m = layer_states.iter().map(|layer| layer.depth_m).sum::<f64>();
    let lane_substeps = scalar_to_usize(
        "wb19_lateral_drain_lane_substeps",
        require_runtime_surface_scalar(runtime_surface, "wb19_lateral_drain_lane_substeps")?,
    )?;
    let drain_enabled = direct_publication_enabled_flag(runtime_surface, "wb19_drain_enabled")?;
    let drain_depth_m = if drain_enabled {
        require_runtime_surface_scalar(runtime_surface, "wb19_drain_depth")?
    } else {
        0.5
    };
    let drain_spacing_m = if drain_enabled {
        require_runtime_surface_scalar(runtime_surface, "wb19_drain_spacing")?
    } else {
        1.0
    };
    let drain_diameter_m = if drain_enabled {
        require_runtime_surface_scalar(runtime_surface, "wb19_drain_diameter")?
    } else {
        0.1
    };
    Ok(DirectSubsurfaceComputeInputs {
        avg_slope: require_runtime_surface_scalar(runtime_surface, "avgslp")?,
        slope_length_m: require_runtime_surface_scalar(runtime_surface, "slplen")?,
        lateral_anisotropy_ratio: require_runtime_surface_scalar(
            runtime_surface,
            "wb19_lateral_anisotropy_ratio",
        )?,
        soil_depth_m,
        solwpv_mode: scalar_to_i32(
            "solwpv",
            require_runtime_surface_scalar(runtime_surface, "solwpv")?,
        )?,
        mofe_hourly_carry_arrays_enabled: lane_substeps == 24,
        lane_substeps,
        drainage_capacity_m: 0.0,
        drain_enabled,
        drain_depth_m,
        drain_spacing_m,
        drain_diameter_m,
        layers: layer_states.into_iter().map(Into::into).collect(),
    })
}

fn direct_publication_layer_states(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<Vec<DirectSubsurfaceLayerState>, HillslopeCliError> {
    let nsl = direct_publication_layer_count(runtime_surface)?;
    let mut layers = Vec::with_capacity(nsl);
    for layer_index in 1..=nsl {
        layers.push(direct_publication_layer_state(
            runtime_surface,
            layer_index,
        )?);
    }
    Ok(layers)
}

fn direct_publication_layer_count(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<usize, HillslopeCliError> {
    let nsl_symbol = if runtime_surface_symbol_value(runtime_surface, "wb11_nsl").is_some() {
        "wb11_nsl"
    } else {
        "nsl"
    };
    scalar_to_usize(
        nsl_symbol,
        require_runtime_surface_scalar(runtime_surface, nsl_symbol)?,
    )
}

