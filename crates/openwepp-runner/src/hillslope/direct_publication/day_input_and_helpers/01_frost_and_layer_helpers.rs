#[allow(dead_code)]
fn direct_production_frost_typed_authority(
    seed_surface: &HillslopeWritebackSurface,
    layers: &[DirectSubsurfaceLayerState],
    frost_file_present: bool,
    frost_wint_red_enabled: bool,
    frost_projection_present: bool,
) -> Result<Option<DirectProductionFrostTypedAuthority>, HillslopeCliError> {
    if !frost_projection_present {
        return Ok(None);
    }
    let controls = DirectFrostControlInputs {
        frost_file_present,
        wint_red_enabled: frost_wint_red_enabled,
        fine_top_count: direct_publication_frost_fine_count(seed_surface, "frost.options.fineTop")?,
        fine_bot_count: direct_publication_frost_fine_count(seed_surface, "frost.options.fineBot")?,
        ksnowf: direct_publication_required_positive_scalar(seed_surface, "frost.options.ksnowf")?,
        kresf: direct_publication_required_positive_scalar(seed_surface, "frost.options.kresf")?,
        ksoilf: direct_publication_required_positive_scalar(seed_surface, "frost.options.ksoilf")?,
        kfactor1: require_runtime_surface_scalar(seed_surface, "frost.options.kfactor1")?,
        kfactor2: require_runtime_surface_scalar(seed_surface, "frost.options.kfactor2")?,
        kfactor3: require_runtime_surface_scalar(seed_surface, "frost.options.kfactor3")?,
        landuse_class_proxy: runtime_surface_symbol_value(seed_surface, "landuse.class_proxy"),
    };
    let mut layer_bulk_density_kg_m3 = Vec::with_capacity(layers.len());
    for layer_index in 1..=layers.len() {
        let symbol = format!("wb19_bulk_density_kg_m3_{layer_index:04}");
        let value = require_runtime_surface_scalar(seed_surface, symbol.as_str())?;
        if !value.is_finite() || value <= 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} {symbol} must be finite and > 0.0 for direct production frost typed solver, observed {value}"
                ),
            });
        }
        layer_bulk_density_kg_m3.push(value);
    }
    Ok(Some(DirectProductionFrostTypedAuthority {
        controls,
        layer_bulk_density_kg_m3,
        soil_conductivity_m_s: direct_publication_optional_nonnegative_scalar(
            seed_surface,
            &["wb14_soil_conductivity_m_s"],
        )?,
        residue_depth_m: direct_publication_optional_nonnegative_scalar(
            seed_surface,
            &["frost.runtime_residue_depth_m", "resdep"],
        )?
        .unwrap_or(0.0),
        theta_residual: require_runtime_surface_scalar(seed_surface, "thetdr")?,
        theta_field_capacity: require_runtime_surface_scalar(seed_surface, "thetfc")?,
        albedo: require_runtime_surface_scalar(seed_surface, "salb")?,
        canopy_height_m: direct_publication_optional_nonnegative_scalar(seed_surface, &["canhgt"])?
            .unwrap_or(0.0),
        random_roughness_m: direct_publication_optional_nonnegative_scalar(
            seed_surface,
            &["rrc", "rrinit"],
        )?
        .unwrap_or(0.0),
        monthly_max_c: direct_production_monthly_temperature(seed_surface, "obmaxt")?,
        monthly_min_c: direct_production_monthly_temperature(seed_surface, "obmint")?,
    }))
}

#[allow(dead_code)]
fn direct_production_monthly_temperature(
    seed_surface: &HillslopeWritebackSurface,
    root: &str,
) -> Result<[f64; 12], HillslopeCliError> {
    let mut values = [0.0; 12];
    for month in 1..=12 {
        let symbol = format!("{root}_{month:04}");
        values[month - 1] = require_runtime_surface_scalar(seed_surface, symbol.as_str())?;
    }
    Ok(values)
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
fn direct_publication_erod14_class_symbol(root: &str, class_index: usize) -> String {
    format!("{root}_{class_index:04}")
}

#[allow(dead_code)]
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

fn direct_publication_frost_runtime_carry_from_lane_state(
    state: &DirectFrostLaneState,
) -> Option<DirectFrostRuntimeCarry> {
    state.has_runtime_state().then(|| state.clone().into())
}

fn direct_production_frost_prior_state_input(
    state: &DirectFrostLaneState,
) -> DirectFrostPriorStateInput {
    DirectFrostPriorStateInput {
        active_frost_coupling: state.active_frost_coupling,
        dfrost_m: state.dfrost_m,
        dthaw_m: state.dthaw_m,
        nft: state.nft,
        ws_frz_m: state.ws_frz_m,
        infcap_frz_m_s: state.infcap_frz_m_s,
        frwatc_soil_water_before_m: state.frwatc_soil_water_before_m,
        frwatc_soil_water_after_m: state.frwatc_soil_water_after_m,
        frwatc_frozen_water_before_m: state.frwatc_frozen_water_before_m,
        frwatc_frozen_water_after_m: state.frwatc_frozen_water_after_m,
        frwatc_freeze_debit_m: state.frwatc_freeze_debit_m,
        frwatc_thaw_credit_m: state.frwatc_thaw_credit_m,
        frwatc_net_liquid_delta_m: state.frwatc_net_liquid_delta_m,
        frdp_m: state.frdp_m,
        thdp_m: state.thdp_m,
        tfrdp_m: state.tfrdp_m,
        tthawd_m: state.tthawd_m,
        fgthwd_flag: state.fgthwd_flag,
        total_fine_layer_count: state.total_fine_layer_count,
        conductivity_tilled_w_m_k: state.conductivity_tilled_w_m_k,
        conductivity_untilled_w_m_k: state.conductivity_untilled_w_m_k,
        conductivity_residue_w_m_k: state.conductivity_residue_w_m_k,
        shadow_total_water_before_m: state.shadow_total_water_before_m,
        shadow_total_water_after_m: state.shadow_total_water_after_m,
        shadow_wb_delta_m: state.shadow_wb_delta_m,
        shadow_frwatc_residual_m: state.shadow_frwatc_residual_m,
        watpdg_m: state.watpdg_m,
        watbtm_m: state.watbtm_m,
        layer_shadows: state
            .layer_shadows
            .iter()
            .map(|layer| DirectFrostLayerShadowProjection {
                layer_index: layer.layer_index,
                st_m: layer.st_m,
                soil_water_m: layer.soil_water_m,
                frozen_depth_m: layer.frozen_depth_m,
                frozen_water_m: layer.frozen_water_m,
                soilf_m: layer.soilf_m,
                yst_m: layer.yst_m,
                nwfrzz_m: layer.nwfrzz_m,
            })
            .collect(),
        fine_layers: state
            .fine_layers
            .iter()
            .map(|fine| DirectFrostFineLayerProjection {
                layer_index: fine.layer_index,
                fine_index: fine.fine_index,
                fgfrst: fine.fgfrst,
                slfsd_m: fine.slfsd_m,
                slsic_m: fine.slsic_m,
                slsw_theta: fine.slsw_theta,
                sltime_s: fine.sltime_s,
            })
            .collect(),
    }
}

fn direct_production_frost_layer_inputs(
    lane_index: usize,
    layers: &[DirectSubsurfaceLayerState],
    bulk_density_kg_m3: &[f64],
) -> Result<Vec<DirectFrostLayerInput>, HillslopeCliError> {
    if layers.len() != bulk_density_kg_m3.len() {
        return Err(direct_production_executor_blocked(format!(
            "direct production lane {} frost typed solver requires {} bulk-density values, observed {}",
            lane_index + 1,
            layers.len(),
            bulk_density_kg_m3.len()
        )));
    }
    Ok(layers
        .iter()
        .zip(bulk_density_kg_m3.iter().copied())
        .enumerate()
        .map(|(offset, (layer, bulk_density_kg_m3))| DirectFrostLayerInput {
            layer_index: offset + 1,
            theta_m: layer.theta_m,
            upper_limit_m: layer.upper_limit_m,
            depth_m: layer.depth_m,
            residual_theta: layer.residual_theta,
            bulk_density_kg_m3,
            frozen_depth_m: layer.frozen_depth_m,
            frozen_water_m: layer.frozen_water_m,
        })
        .collect())
}

fn direct_production_same_day_frost_hydrology_layers(
    lane_index: usize,
    layers: &[DirectSubsurfaceLayerState],
    frost_outcome: &DirectWinterFrostPartitionOutcome,
    target_soil_water_m: f64,
    clear_no_final_hydrology_layers: bool,
) -> Result<Vec<DirectSubsurfaceLayerState>, HillslopeCliError> {
    direct_production_validate_layers(lane_index, layers)?;
    let mut hydrology_layers = layers.to_vec();
    if !clear_no_final_hydrology_layers
        || direct_production_frost_outcome_has_final_frozen_projection(frost_outcome)
    {
        return Ok(hydrology_layers);
    }
    clear_direct_production_no_final_frost_layers(
        lane_index,
        &mut hydrology_layers,
        target_soil_water_m,
    )?;
    Ok(hydrology_layers)
}

fn direct_production_frost_outcome_has_final_frozen_projection(
    frost_outcome: &DirectWinterFrostPartitionOutcome,
) -> bool {
    frost_outcome.frost_depth_after_m > 1.0e-12
        || frost_outcome.frozen_water_after_m > 1.0e-12
}

fn clear_direct_production_no_final_frost_layers(
    lane_index: usize,
    layers: &mut [DirectSubsurfaceLayerState],
    target_soil_water_m: f64,
) -> Result<(), HillslopeCliError> {
    if !target_soil_water_m.is_finite() || target_soil_water_m < 0.0 {
        return Err(direct_production_executor_blocked(format!(
            "direct production lane {} no-final-frost clear requires finite nonnegative target soil water, observed {target_soil_water_m}",
            lane_index + 1
        )));
    }
    for (layer_offset, layer) in layers.iter_mut().enumerate() {
        let layer_number = layer_offset + 1;
        for (field, value) in [
            ("theta_m", layer.theta_m),
            ("residual_theta", layer.residual_theta),
            ("depth_m", layer.depth_m),
            ("frozen_depth_m", layer.frozen_depth_m),
            ("frozen_water_m", layer.frozen_water_m),
        ] {
            if !value.is_finite() || value < 0.0 {
                return Err(direct_production_executor_blocked(format!(
                    "direct production lane {} layer {} no-final-frost clear requires finite nonnegative {field}, observed {value}",
                    lane_index + 1,
                    layer_number
                )));
            }
        }
        if layer.frozen_depth_m > layer.depth_m + 1.0e-12 {
            return Err(direct_production_executor_blocked(format!(
                "direct production lane {} layer {} no-final-frost clear requires frozen depth <= layer depth, observed {} > {}",
                lane_index + 1,
                layer_number,
                layer.frozen_depth_m,
                layer.depth_m
            )));
        }
        if layer.frozen_depth_m <= 1.0e-12 && layer.frozen_water_m <= 1.0e-12 {
            layer.frozen_depth_m = 0.0;
            layer.frozen_water_m = 0.0;
            continue;
        }
        layer.frozen_depth_m = 0.0;
        layer.frozen_water_m = 0.0;
    }
    rebalance_direct_production_no_final_frost_layers_to_storage(
        lane_index,
        layers,
        target_soil_water_m,
    )?;
    Ok(())
}

fn rebalance_direct_production_no_final_frost_layers_to_storage(
    lane_index: usize,
    layers: &mut [DirectSubsurfaceLayerState],
    target_soil_water_m: f64,
) -> Result<(), HillslopeCliError> {
    let mut aggregate_m = 0.0_f64;
    for layer in layers.iter() {
        aggregate_m += layer.theta_m + layer.residual_theta * layer.depth_m;
        if !aggregate_m.is_finite() {
            return Err(direct_production_executor_blocked(format!(
                "direct production lane {} no-final-frost clear produced nonfinite aggregate storage {aggregate_m}",
                lane_index + 1
            )));
        }
    }
    let delta_m = target_soil_water_m - aggregate_m;
    if !delta_m.is_finite() {
        return Err(direct_production_executor_blocked(format!(
            "direct production lane {} no-final-frost clear produced nonfinite storage delta {delta_m}",
            lane_index + 1
        )));
    }
    if delta_m.abs() <= 1.0e-12 {
        return Ok(());
    }
    if layers.is_empty() {
        return Err(direct_production_executor_blocked(format!(
            "direct production lane {} no-final-frost clear requires at least one layer",
            lane_index + 1
        )));
    }
    if delta_m > 0.0 {
        let top_layer = &mut layers[0];
        top_layer.theta_m += delta_m;
        if !top_layer.theta_m.is_finite() || top_layer.theta_m < 0.0 {
            return Err(direct_production_executor_blocked(format!(
                "direct production lane {} no-final-frost clear produced invalid top-layer theta {}",
                lane_index + 1,
                top_layer.theta_m
            )));
        }
    } else {
        let mut remaining_m = -delta_m;
        for layer in layers.iter_mut() {
            if remaining_m <= 1.0e-12 {
                break;
            }
            if !layer.theta_m.is_finite() || layer.theta_m < 0.0 {
                return Err(direct_production_executor_blocked(format!(
                    "direct production lane {} no-final-frost clear requires finite nonnegative layer theta, observed {}",
                    lane_index + 1,
                    layer.theta_m
                )));
            }
            let debit_m = layer.theta_m.min(remaining_m);
            layer.theta_m -= debit_m;
            remaining_m -= debit_m;
            if layer.theta_m < 0.0 && layer.theta_m.abs() <= 1.0e-12 {
                layer.theta_m = 0.0;
            }
        }
        if remaining_m > 1.0e-12 {
            return Err(direct_production_executor_blocked(format!(
                "direct production lane {} no-final-frost clear cannot debit storage delta {delta_m}",
                lane_index + 1
            )));
        }
    }
    let aggregate_after_m = layers
        .iter()
        .try_fold(0.0_f64, |total, layer| {
            let aggregate = total + layer.theta_m + layer.residual_theta * layer.depth_m;
            if aggregate.is_finite() {
                Ok(aggregate)
            } else {
                Err(direct_production_executor_blocked(format!(
                    "direct production lane {} no-final-frost clear produced nonfinite aggregate after rebalance",
                    lane_index + 1
                )))
            }
        })?;
    if (aggregate_after_m - target_soil_water_m).abs() > 1.0e-12 {
        return Err(direct_production_executor_blocked(format!(
            "direct production lane {} no-final-frost clear aggregate {} differs from target {}",
            lane_index + 1,
            aggregate_after_m,
            target_soil_water_m
        )));
    }
    Ok(())
}

fn direct_production_typed_frost_soil_conductivity(
    authority: &DirectProductionFrostTypedAuthority,
    layers: &[DirectSubsurfaceLayerState],
) -> Result<f64, HillslopeCliError> {
    if let Some(value) = authority.soil_conductivity_m_s
        && value > 0.0
    {
        return Ok(value);
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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
