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
        || frost_outcome.soil_water_after_frwatc_m.is_some()
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
