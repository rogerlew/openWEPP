#[allow(clippy::too_many_lines)]
fn seed_wb11_runtime_surface_inputs(
    runtime_surface: &mut HillslopeWritebackSurface,
    execution_lane: ExecutionLane,
) -> Result<(), HillslopeCliError> {
    const WB11_STATE_SEED_COMPLETED_SYMBOL: &str = "wb11_state_seed_completed";
    const WB18_PERC_LANE_SUBSTEPS_SYMBOL: &str = "wb18_perc_lane_substeps";
    const WB19_LATERAL_DRAIN_LANE_SUBSTEPS_SYMBOL: &str = "wb19_lateral_drain_lane_substeps";

    let nsl_symbol = if runtime_surface_symbol_value(runtime_surface, "wb11_nsl").is_some() {
        "wb11_nsl"
    } else {
        "nsl"
    };
    let nsl = scalar_to_usize(
        nsl_symbol,
        require_runtime_surface_scalar(runtime_surface, nsl_symbol)?,
    )?;
    if nsl == 0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!("{SIMPIPE_GUARD_ID} nsl must be >= 1 for WB11 seeding"),
        });
    }

    let wb18_perc_lane_substeps = match execution_lane {
        ExecutionLane::Daily => 1.0,
        ExecutionLane::Hourly => 24.0,
    };
    let contributor_ofe_count = runtime_surface_ofe_count(runtime_surface)?;
    let mofe_hourly_carry_active = contributor_ofe_count > 1;
    let wb18_perc_lane_substeps = if mofe_hourly_carry_active {
        24.0
    } else {
        wb18_perc_lane_substeps
    };
    runtime_surface.state_surface.insert(
        BoundarySymbol::from(WB18_PERC_LANE_SUBSTEPS_SYMBOL),
        BoundaryValue::scalar(wb18_perc_lane_substeps),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from(WB19_LATERAL_DRAIN_LANE_SUBSTEPS_SYMBOL),
        BoundaryValue::scalar(wb18_perc_lane_substeps),
    );
    seed_mofe_hourly_carry_runtime_surface_inputs(runtime_surface, mofe_hourly_carry_active)?;

    let prcp = require_runtime_surface_scalar(runtime_surface, "prcp")?;
    if prcp < 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!("{SIMPIPE_GUARD_ID} prcp must be >= 0.0, observed {prcp}"),
        });
    }
    let breakpoint_mode =
        runtime_surface_symbol_value(runtime_surface, "ibrkpt").is_some_and(|value| value >= 0.5);
    let hyetograph_point_symbol =
        if breakpoint_mode && runtime_surface_symbol_value(runtime_surface, "nbrkpt").is_some() {
            // Breakpoint climates are authoritative on `nbrkpt`; stale `ninten`
            // from prior days must not truncate the current-day event shape.
            "nbrkpt"
        } else if runtime_surface_symbol_value(runtime_surface, "ninten").is_some() {
            "ninten"
        } else {
            "nbrkpt"
        };
    let mut ninten = scalar_to_usize(
        hyetograph_point_symbol,
        require_runtime_surface_scalar(runtime_surface, hyetograph_point_symbol)?,
    )?;
    if ninten == 0 {
        let stmdur = runtime_surface_symbol_value(runtime_surface, "stmdur")
            .unwrap_or(1.0)
            .max(1.0);
        let intensity = if stmdur > 0.0 { prcp / stmdur } else { prcp };
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("ninten"), BoundaryValue::scalar(2.0));
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("timem_0001"),
            BoundaryValue::scalar(0.0),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("timem_0002"),
            BoundaryValue::scalar(stmdur),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("intsty_0001"),
            BoundaryValue::scalar(intensity.max(0.0)),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("intsty_0002"),
            BoundaryValue::scalar(0.0),
        );
        ninten = 2;
    }
    let ninten_scalar = usize_to_scalar("ninten", ninten)?;
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("ninten"),
        BoundaryValue::scalar(ninten_scalar),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("nbrkpt"),
        BoundaryValue::scalar(ninten_scalar),
    );

    let mut hyetograph_rainfall = 0.0_f64;
    for point_index in 1..ninten {
        let time_symbol = wb13_primary_layer_symbol("timem", point_index);
        let next_time_symbol = wb13_primary_layer_symbol("timem", point_index + 1);
        let intensity_symbol = wb13_primary_layer_symbol("intsty", point_index);

        let time_s = require_runtime_surface_scalar(runtime_surface, time_symbol.as_str())?;
        let next_time_s =
            require_runtime_surface_scalar(runtime_surface, next_time_symbol.as_str())?;
        let intensity = require_runtime_surface_scalar(runtime_surface, intensity_symbol.as_str())?;

        if next_time_s < time_s {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb11_seed",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} {next_time_symbol} ({next_time_s}) must be >= {time_symbol} ({time_s})"
                ),
            });
        }
        if intensity < 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb11_seed",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} {intensity_symbol} must be >= 0.0, observed {intensity}"
                ),
            });
        }

        hyetograph_rainfall += intensity * (next_time_s - time_s);
    }

    let wb11_state_seeded = runtime_surface
        .state_surface
        .get(&BoundarySymbol::from(WB11_STATE_SEED_COMPLETED_SYMBOL))
        .copied()
        .map(BoundaryValue::as_f64)
        .is_some_and(|value| value >= 0.5)
        || runtime_surface_symbol_value(runtime_surface, "wb18_perc_theta_0001").is_some();
    if !wb11_state_seeded {
        let mut wb11_soil_water = 0.0_f64;
        let mut wb11_field_capacity = 0.0_f64;
        let mut wb11_drainable_storage = 0.0_f64;
        let mut wb11_drainage_coefficient = 0.0_f64;
        let mut sat = require_runtime_surface_scalar(runtime_surface, "sat")?;
        if sat < 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb11_seed",
                detail: format!("{SIMPIPE_GUARD_ID} sat must be >= 0.0, observed {sat}"),
            });
        }
        let sat_cap = match execution_lane {
            ExecutionLane::Daily => 0.95,
            ExecutionLane::Hourly => 1.0,
        };
        if sat > sat_cap {
            sat = sat_cap;
        }

        for layer_index in 1..=nsl {
            let dg_symbol = format!("wb19_dg_{layer_index:04}");
            let fc_symbol = format!("wb19_thetfc_{layer_index:04}");
            let wp_symbol = format!("wb19_thetdr_{layer_index:04}");
            let ssc_symbol = wb13_primary_layer_symbol("ssc", layer_index);
            let por_symbol = format!("wb19_por_{layer_index:04}");
            let cpm_symbol = wb13_primary_layer_symbol("cpm", layer_index);

            let dg = require_runtime_surface_scalar(runtime_surface, dg_symbol.as_str())?;
            let thetfc = require_runtime_surface_scalar(runtime_surface, fc_symbol.as_str())?;
            let thetdr = require_runtime_surface_scalar(runtime_surface, wp_symbol.as_str())?;
            let ssc = require_runtime_surface_scalar(runtime_surface, ssc_symbol.as_str())?;
            let por = require_runtime_surface_scalar(runtime_surface, por_symbol.as_str())?;
            let cpm = require_runtime_surface_scalar(runtime_surface, cpm_symbol.as_str())?;

            if dg <= 0.0 {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!("{SIMPIPE_GUARD_ID} {dg_symbol} must be > 0.0, observed {dg}"),
                });
            }
            if thetfc < 0.0 {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} {fc_symbol} must be >= 0.0, observed {thetfc}"
                    ),
                });
            }
            if thetdr < 0.0 {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} {wp_symbol} must be >= 0.0, observed {thetdr}"
                    ),
                });
            }
            if thetdr > thetfc {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} {wp_symbol} must be <= {fc_symbol} (observed {thetdr} > {thetfc})"
                    ),
                });
            }
            if ssc <= 0.0 {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} {ssc_symbol} must be > 0.0, observed {ssc}"
                    ),
                });
            }
            if por <= 0.0 || por > 1.0 {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} {por_symbol} must be within (0,1], observed {por}"
                    ),
                });
            }
            if cpm <= 0.0 || cpm > 1.0 {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} {cpm_symbol} must be within (0,1], observed {cpm}"
                    ),
                });
            }
            if thetdr > por {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} {wp_symbol} must be <= {por_symbol} (observed {thetdr} > {por})"
                    ),
                });
            }

            let saturation_capacity = por * cpm;
            if !saturation_capacity.is_finite() || saturation_capacity <= 0.0 {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} por*cpm must be finite and > 0.0, observed {saturation_capacity}"
                    ),
                });
            }
            let sat_floor = thetdr / saturation_capacity;
            if !sat_floor.is_finite() || !(0.0..=1.0).contains(&sat_floor) {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} derived saturation floor for layer {layer_index} must be within [0,1], observed {sat_floor}"
                    ),
                });
            }
            if sat < sat_floor {
                sat = sat_floor;
            }

            let fc_store = (thetfc - thetdr) * dg;
            if !fc_store.is_finite() || fc_store < 0.0 {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} derived wb18_perc_fc_{layer_index:04} must be finite and >= 0.0, observed {fc_store}"
                    ),
                });
            }

            let ul_store = (por - thetdr) * dg;
            if ul_store <= 0.0 {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} derived WB18 upper-limit store must be > 0.0 for layer {layer_index}"
                    ),
                });
            }

            let saturation_theta = (sat * por) * cpm;
            let mut st_store = (saturation_theta - thetdr) * dg;
            if !st_store.is_finite() {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} derived wb18_perc_theta_{layer_index:04} is non-finite ({st_store})"
                    ),
                });
            }
            if st_store < -1.0e-10 {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} derived wb18_perc_theta_{layer_index:04} must be >= 0.0, observed {st_store}"
                    ),
                });
            }
            if st_store < 1.0e-10 {
                st_store = 0.0;
            }

            let soilw_store = st_store + (thetdr * dg);
            if !soilw_store.is_finite() || soilw_store < 0.0 {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} derived layer soil-water store must be finite and >= 0.0 for layer {layer_index}, observed {soilw_store}"
                    ),
                });
            }

            wb11_soil_water += soilw_store;
            wb11_field_capacity += fc_store;
            wb11_drainable_storage += (st_store - fc_store).max(0.0);
            wb11_drainage_coefficient += ssc * 86_400.0;

            runtime_surface.state_surface.insert(
                BoundarySymbol::from(format!("wb18_perc_theta_{layer_index:04}")),
                BoundaryValue::scalar(st_store),
            );
            runtime_surface.state_surface.insert(
                BoundarySymbol::from(format!("wb18_perc_fc_{layer_index:04}")),
                BoundaryValue::scalar(fc_store),
            );
            runtime_surface.state_surface.insert(
                BoundarySymbol::from(format!("wb18_perc_ul_{layer_index:04}")),
                BoundaryValue::scalar(ul_store),
            );
            runtime_surface.state_surface.insert(
                BoundarySymbol::from(format!("wb18_perc_ssc_{layer_index:04}")),
                BoundaryValue::scalar(ssc),
            );
        }

        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("sat"), BoundaryValue::scalar(sat));
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb11_soil_water"),
            BoundaryValue::scalar(wb11_soil_water),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb11_field_capacity"),
            BoundaryValue::scalar(wb11_field_capacity),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb11_perc_fraction"),
            BoundaryValue::scalar(0.5),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb11_drainage_coefficient"),
            BoundaryValue::scalar(wb11_drainage_coefficient.max(1.0e-6)),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb11_drainable_storage"),
            BoundaryValue::scalar(wb11_drainable_storage),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(WB11_STATE_SEED_COMPLETED_SYMBOL),
            BoundaryValue::scalar(1.0),
        );
    }

    let wb11_soil_water = require_runtime_surface_scalar(runtime_surface, "wb11_soil_water")?;
    if wb11_soil_water < 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!(
                "{SIMPIPE_GUARD_ID} wb11_soil_water must be >= 0.0 before daily reconciliation seeding, observed {wb11_soil_water}"
            ),
        });
    }

    if runtime_surface_symbol_value(runtime_surface, "wb17_residue_interception").is_none() {
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb17_residue_interception"),
            BoundaryValue::scalar(0.0),
        );
    }
    if runtime_surface_symbol_value(runtime_surface, "Ws").is_none() {
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Ws"), BoundaryValue::scalar(1.0));
    }
    if runtime_surface_symbol_value(runtime_surface, "wb19_lateral_anisotropy_ratio").is_none() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!(
                "{SIMPIPE_GUARD_ID} missing required runtime symbol wb19_lateral_anisotropy_ratio"
            ),
        });
    }
    if runtime_surface_symbol_value(runtime_surface, "wb19_drain_enabled").is_none() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!(
                "{SIMPIPE_GUARD_ID} missing required runtime symbol wb19_drain_enabled"
            ),
        });
    }
    let wb19_lateral_anisotropy_ratio =
        require_runtime_surface_scalar(runtime_surface, "wb19_lateral_anisotropy_ratio")?;
    if wb19_lateral_anisotropy_ratio <= 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!(
                "{SIMPIPE_GUARD_ID} wb19_lateral_anisotropy_ratio must be > 0.0, observed {wb19_lateral_anisotropy_ratio}"
            ),
        });
    }
    let wb19_drain_enabled = require_runtime_surface_scalar(runtime_surface, "wb19_drain_enabled")?;
    let wb19_drain_enabled_flag = if wb19_drain_enabled.abs() <= 1.0e-12 {
        false
    } else if (wb19_drain_enabled - 1.0).abs() <= 1.0e-12 {
        true
    } else {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!(
                "{SIMPIPE_GUARD_ID} wb19_drain_enabled must be 0 or 1, observed {wb19_drain_enabled}"
            ),
        });
    };
    if wb19_drain_enabled_flag {
        let wb19_drain_depth = require_runtime_surface_scalar(runtime_surface, "wb19_drain_depth")?;
        let wb19_drain_spacing =
            require_runtime_surface_scalar(runtime_surface, "wb19_drain_spacing")?;
        let wb19_drain_diameter =
            require_runtime_surface_scalar(runtime_surface, "wb19_drain_diameter")?;
        if wb19_drain_depth <= 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb11_seed",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} wb19_drain_depth must be > 0.0 when wb19_drain_enabled=1, observed {wb19_drain_depth}"
                ),
            });
        }
        if wb19_drain_spacing <= 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb11_seed",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} wb19_drain_spacing must be > 0.0 when wb19_drain_enabled=1, observed {wb19_drain_spacing}"
                ),
            });
        }
        if wb19_drain_diameter <= 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb11_seed",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} wb19_drain_diameter must be > 0.0 when wb19_drain_enabled=1, observed {wb19_drain_diameter}"
                ),
            });
        }
    }

    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb12_rainfall_input"),
        BoundaryValue::scalar(hyetograph_rainfall.max(prcp)),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb12_runon_input"),
        BoundaryValue::scalar(0.0),
    );
    runtime_surface.flux_surface.insert(
        BoundarySymbol::from("wb12_runoff_carryover"),
        BoundaryValue::scalar(0.0),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb12_infiltration"),
        BoundaryValue::scalar(0.0),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb12_depression_storage_delta"),
        BoundaryValue::scalar(0.0),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb12_runoff_observed"),
        BoundaryValue::scalar(0.0),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb12_runoff_closure_tolerance"),
        BoundaryValue::scalar(1.0),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb12_storage_initial"),
        BoundaryValue::scalar(wb11_soil_water),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb12_storage_observed"),
        BoundaryValue::scalar(wb11_soil_water),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb12_storage_closure_tolerance"),
        BoundaryValue::scalar(1.0),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb12_precip_input"),
        BoundaryValue::scalar(prcp),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb20_forward_solver_lane_enabled"),
        BoundaryValue::scalar(1.0),
    );

    let wb11_et_seed = compute_wb11_et_demand_seed(runtime_surface)?;
    publish_wb11_et_demand_seed(runtime_surface, wb11_et_seed)?;

    if runtime_surface_symbol_value(runtime_surface, "efflen").is_none() {
        let slplen = require_runtime_surface_scalar(runtime_surface, "slplen")?;
        if slplen <= 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb11_seed",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} slplen must be > 0.0 when seeding efflen, observed {slplen}"
                ),
            });
        }
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("efflen"),
            BoundaryValue::scalar(slplen),
        );
    }
    if runtime_surface_symbol_value(runtime_surface, "m").is_none() {
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("m"), BoundaryValue::scalar(1.5));
    }
    let ealpha_seeded_prior =
        runtime_surface_symbol_value(runtime_surface, WB16_EALPHA_COMPATIBILITY_SEED_FLAG_SYMBOL)
            .is_some_and(|value| value >= 0.5);
    let ealpha_runtime_produced_this_day =
        produce_wb16_ealpha_from_runtime_surface(runtime_surface)?.is_some();
    if !ealpha_runtime_produced_this_day {
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("ealpha"), BoundaryValue::scalar(1.0));
    }
    let ealpha_seeded_any_day = !ealpha_runtime_produced_this_day || ealpha_seeded_prior;
    runtime_surface.state_surface.insert(
        BoundarySymbol::from(WB16_EALPHA_COMPATIBILITY_SEED_FLAG_SYMBOL),
        BoundaryValue::scalar(if ealpha_seeded_any_day { 1.0 } else { 0.0 }),
    );
    seed_mofe03_wave2_runtime_surface_inputs(runtime_surface)?;

    Ok(())
}

const WB16_ACCGAV_M_S2: f64 = 9.807;
const WB16_INRFSO_CROPLAND: f64 = 4.07;
const WB16_FRCSOL_CROPLAND: f64 = 1.11;
const WB16_RRINIT_MIN_M: f64 = 0.006;
const WB16_RSPACE_DEFAULT_M: f64 = 1.0;
const WB16_TEMPORARY_WIDTH_DEFAULT_M: f64 = 0.15;
const WB16_COVER_CAP: f64 = 0.999;

#[allow(clippy::too_many_lines, clippy::similar_names)]
fn produce_wb16_ealpha_from_runtime_surface(
    runtime_surface: &mut HillslopeWritebackSurface,
) -> Result<Option<f64>, HillslopeCliError> {
    let Some(nelem_raw) = runtime_surface_symbol_value(runtime_surface, "nelem") else {
        return Ok(None);
    };
    let ofe_count = scalar_to_usize("nelem", nelem_raw)?;
    if ofe_count == 0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb16_ealpha_producer",
            detail: format!("{SIMPIPE_GUARD_ID} nelem must be >= 1 for WB16 ealpha production"),
        });
    }

    let m = require_runtime_surface_scalar(runtime_surface, "m")?;
    if !m.is_finite() || m <= 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb16_ealpha_producer",
            detail: format!(
                "{SIMPIPE_GUARD_ID} m must be finite and > 0 for WB16 ealpha production, observed {m}"
            ),
        });
    }
    let power2 = 1.0 / m;
    let power3 = power2 + 1.0;

    let mut alpha_values = Vec::with_capacity(ofe_count);
    let mut slplen_values = Vec::with_capacity(ofe_count);

    for ofe_index in 1..=ofe_count {
        let Some(avgslp_raw) = wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "avgslp")
        else {
            return Ok(None);
        };
        let Some(slplen_raw) = wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "slplen")
        else {
            return Ok(None);
        };
        if !avgslp_raw.is_finite() || avgslp_raw <= 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb16_ealpha_producer",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} ofe{ofe_index}_avgslp must be finite and > 0, observed {avgslp_raw}"
                ),
            });
        }
        if !slplen_raw.is_finite() || slplen_raw <= 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb16_ealpha_producer",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} ofe{ofe_index}_slplen must be finite and > 0, observed {slplen_raw}"
                ),
            });
        }

        let Some(inrcov_raw) = wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "inrcov")
        else {
            return Ok(None);
        };
        let Some(rilcov_raw) = wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "rilcov")
        else {
            return Ok(None);
        };
        let Some(rrinit_raw) = wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "rrinit")
        else {
            return Ok(None);
        };
        let Some(rspace_raw) = wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "rspace")
        else {
            return Ok(None);
        };
        let Some(width_raw) = wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "width")
        else {
            return Ok(None);
        };
        let Some(rtyp_raw) = wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "rtyp")
        else {
            return Ok(None);
        };

        let Some(cancov_raw) = wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "cancov")
            .or_else(|| wb16_optional_state_scalar(runtime_surface, "cancov"))
        else {
            return Ok(None);
        };
        let Some(bb_raw) = wb16_optional_state_scalar(
            runtime_surface,
            &format!("pl_growth_ofe{ofe_index}_bb_seed"),
        )
        .or_else(|| wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "bb"))
        .or_else(|| wb16_optional_state_scalar(runtime_surface, "bb")) else {
            return Ok(None);
        };
        let Some(bbb_raw) = wb16_optional_state_scalar(
            runtime_surface,
            &format!("pl_growth_ofe{ofe_index}_bbb_seed"),
        )
        .or_else(|| wb16_optional_state_scalar(runtime_surface, "bbb_seed"))
        .or_else(|| wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "bbb"))
        .or_else(|| wb16_optional_state_scalar(runtime_surface, "bbb")) else {
            return Ok(None);
        };
        let Some(flivmx_raw) = wb16_optional_state_scalar(
            runtime_surface,
            &format!("pl_growth_ofe{ofe_index}_flivmx_seed"),
        )
        .or_else(|| wb16_optional_state_scalar(runtime_surface, "flivmx_seed"))
        .or_else(|| wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "flivmx"))
        .or_else(|| wb16_optional_state_scalar(runtime_surface, "flivmx")) else {
            return Ok(None);
        };
        let Some(hmax_raw) = wb16_optional_state_scalar(
            runtime_surface,
            &format!("pl_growth_ofe{ofe_index}_hmax_seed"),
        )
        .or_else(|| wb16_optional_state_scalar(runtime_surface, "hmax_seed"))
        .or_else(|| wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "hmax"))
        .or_else(|| wb16_optional_state_scalar(runtime_surface, "hmax")) else {
            return Ok(None);
        };

        for (symbol, value) in [
            ("inrcov", inrcov_raw),
            ("rilcov", rilcov_raw),
            ("rrinit", rrinit_raw),
            ("rspace", rspace_raw),
            ("width", width_raw),
            ("rtyp", rtyp_raw),
            ("cancov", cancov_raw),
            ("bb", bb_raw),
            ("bbb", bbb_raw),
            ("flivmx", flivmx_raw),
            ("hmax", hmax_raw),
        ] {
            if !value.is_finite() {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb16_ealpha_producer",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} ofe{ofe_index}_{symbol} must be finite for WB16 ealpha production, observed {value}"
                    ),
                });
            }
        }

        if inrcov_raw < 0.0 || rilcov_raw < 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb16_ealpha_producer",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} ofe{ofe_index}_inrcov/rilcov must be >= 0.0, observed inrcov={inrcov_raw}, rilcov={rilcov_raw}"
                ),
            });
        }
        if rrinit_raw < 0.0 || rspace_raw < 0.0 || width_raw < 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb16_ealpha_producer",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} ofe{ofe_index}_rrinit/rspace/width must be >= 0.0, observed rrinit={rrinit_raw}, rspace={rspace_raw}, width={width_raw}"
                ),
            });
        }
        if cancov_raw < 0.0 || bb_raw < 0.0 || bbb_raw < 0.0 || flivmx_raw < 0.0 || hmax_raw < 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb16_ealpha_producer",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} ofe{ofe_index} canopy/friction controls must be >= 0.0 (cancov={cancov_raw}, bb={bb_raw}, bbb={bbb_raw}, flivmx={flivmx_raw}, hmax={hmax_raw})"
                ),
            });
        }

        let inrcov = inrcov_raw.min(WB16_COVER_CAP);
        let rilcov = rilcov_raw.min(WB16_COVER_CAP);
        let cancov = cancov_raw.min(WB16_COVER_CAP);
        let rrinit = rrinit_raw.max(WB16_RRINIT_MIN_M);
        let rspace = if rspace_raw <= 0.0 {
            WB16_RSPACE_DEFAULT_M
        } else {
            rspace_raw
        };
        let mut width = width_raw;
        let rtyp = if rtyp_raw >= 1.5 { 2 } else { 1 };
        if rtyp == 1 && width <= 0.0 {
            width = WB16_TEMPORARY_WIDTH_DEFAULT_M;
        } else if rtyp == 2 && width <= 0.0 {
            width = rspace;
        }
        if width > rspace {
            width = rspace;
        }

        let rrc = wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "rrc")
            .or_else(|| wb16_optional_state_scalar(runtime_surface, "rrc"))
            .unwrap_or(rrinit);
        if !rrc.is_finite() || rrc < 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb16_ealpha_producer",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} ofe{ofe_index}_rrc must be finite and >= 0.0, observed {rrc}"
                ),
            });
        }

        let mut rrrinr = rrc / rrinit;
        if rrrinr > 1.0 {
            rrrinr = 1.0;
        }
        let inrfo = (3.024 - 5.042 * (-161.0 * rrinit).exp()).exp();
        let mut inrrou = 0.5 * inrfo.powf(1.128) * (-3.088 * (1.0 - rrrinr)).exp();
        if inrrou < WB16_INRFSO_CROPLAND {
            inrrou = WB16_INRFSO_CROPLAND;
        }
        let inrfro = inrrou - WB16_INRFSO_CROPLAND;
        let inrfco = if inrcov > 0.0 {
            14.5 * inrcov.powf(1.5544)
        } else {
            0.0
        };

        let canhgt = if let Some(canhgt_raw) =
            wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "canhgt")
                .or_else(|| wb16_optional_state_scalar(runtime_surface, "canhgt"))
        {
            if !canhgt_raw.is_finite() || canhgt_raw < 0.0 {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb16_ealpha_producer",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} ofe{ofe_index}_canhgt must be finite and >= 0.0, observed {canhgt_raw}"
                    ),
                });
            }
            canhgt_raw
        } else if hmax_raw <= 0.0 || bb_raw <= 0.0 {
            0.0
        } else {
            let mut vdmt = (1.0 - cancov).ln() / (-bb_raw);
            if vdmt < 0.0 {
                vdmt = 0.0;
            }
            (1.0 - (-bbb_raw * vdmt).exp()) * hmax_raw
        };
        let frlive = if hmax_raw > 0.0 {
            (canhgt / hmax_raw) * flivmx_raw
        } else {
            0.0
        };
        if !frlive.is_finite() {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb16_ealpha_producer",
                detail: format!("{SIMPIPE_GUARD_ID} ofe{ofe_index}_frlive is non-finite"),
            });
        }

        let inrfto = inrfro + inrfco + WB16_INRFSO_CROPLAND + frlive;
        let frccov = if rilcov > 0.0 {
            4.5 * rilcov.powf(1.5544)
        } else {
            0.0
        };
        let frctrl = frccov + frlive + WB16_FRCSOL_CROPLAND;
        let rillar = width / rspace;
        let frcteq = if rillar < 1.0 {
            inrfto + rillar * (frctrl - inrfto)
        } else {
            inrfto
        };
        if !frcteq.is_finite() || frcteq <= 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb16_ealpha_producer",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} ofe{ofe_index}_frcteq must be finite and > 0.0, observed {frcteq}"
                ),
            });
        }
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("ofe{ofe_index}_frcteq")),
            BoundaryValue::scalar(frcteq),
        );

        let alpha = ((avgslp_raw * 8.0 * WB16_ACCGAV_M_S2) / frcteq).sqrt();
        if !alpha.is_finite() || alpha <= 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb16_ealpha_producer",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} ofe{ofe_index}_alpha must be finite and > 0.0, observed {alpha}"
                ),
            });
        }
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("ofe{ofe_index}_alpha")),
            BoundaryValue::scalar(alpha),
        );
        if ofe_index == 1 {
            runtime_surface
                .state_surface
                .insert(BoundarySymbol::from("alpha"), BoundaryValue::scalar(alpha));
        }

        alpha_values.push(alpha);
        slplen_values.push(slplen_raw);
    }

    let ealpha = if ofe_count == 1 {
        alpha_values[0]
    } else {
        let suml: f64 = slplen_values.iter().sum();
        if !suml.is_finite() || suml <= 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb16_ealpha_producer",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} WB16 eplane sum length must be finite and > 0.0, observed {suml}"
                ),
            });
        }
        let mut cml = 0.0;
        let mut sdst = 0.0;
        let mut tmpvr2 = 0.0;
        for (slplen, alpha) in slplen_values.iter().zip(alpha_values.iter()) {
            cml += slplen;
            let tmpvr1 = cml.powf(power3);
            sdst += (tmpvr1 - tmpvr2) / alpha.powf(power2);
            tmpvr2 = tmpvr1;
        }
        if !sdst.is_finite() || sdst <= 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb16_ealpha_producer",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} WB16 eplane storage integral must be finite and > 0.0, observed {sdst}"
                ),
            });
        }
        (suml / sdst).powf(m) * suml
    };

    if !ealpha.is_finite() || ealpha <= 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb16_ealpha_producer",
            detail: format!(
                "{SIMPIPE_GUARD_ID} WB16 produced ealpha must be finite and > 0.0, observed {ealpha}"
            ),
        });
    }

    runtime_surface.state_surface.insert(
        BoundarySymbol::from("ealpha"),
        BoundaryValue::scalar(ealpha),
    );
    Ok(Some(ealpha))
}

fn wb16_optional_state_scalar(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &str,
) -> Option<f64> {
    runtime_surface_symbol_value(runtime_surface, symbol)
}

fn wb16_ofe_optional_state_scalar(
    runtime_surface: &HillslopeWritebackSurface,
    ofe_index: usize,
    root: &str,
) -> Option<f64> {
    runtime_surface_symbol_value(runtime_surface, &format!("ofe{ofe_index}_{root}")).or_else(|| {
        if ofe_index == 1 {
            runtime_surface_symbol_value(runtime_surface, root)
        } else {
            None
        }
    })
}

const MOFE03_WAVE2_ENABLE_TOLERANCE: f64 = 1.0e-9;
const MOFE03_WAVE2_MIN_POSITIVE: f64 = 1.0e-6;
const MOFE03_WAVE2_DEFAULT_XTOP: f64 = 0.2;
const MOFE03_WAVE2_DEFAULT_XBOT: f64 = 0.5;
const MOFE03_WAVE2_DEFAULT_XDETST: f64 = 0.1;
const MOFE03_WAVE2_DEFAULT_LDTOP: f64 = 0.8;
const MOFE03_WAVE2_DEFAULT_LDBOT: f64 = 0.6;
const MOFE03_WAVE2_DEFAULT_LDDEND: f64 = 0.3;
const MOFE03_WAVE2_DEFAULT_KTRATO: f64 = 1.1;
const MOFE03_WAVE2_DEFAULT_AINTC: f64 = 0.4;
const MOFE03_WAVE2_DEFAULT_BINTC: f64 = 0.3;
const MOFE03_WAVE2_DEFAULT_CINTC: f64 = 0.2;
const MOFE03_WAVE2_DEFAULT_BETA: f64 = 0.5;
const MOFE03_WAVE2_DEFAULT_QOSTAR: f64 = 0.2;
const MOFE03_WAVE2_DEFAULT_SSA_SOIL: f64 = 5.0;
const MOFE03_ROUTE_SEGMENT_INDEX: usize = 2;

#[derive(Debug, Clone, Copy)]
struct Mofe03Wave2CaseScalars {
    case_value: f64,
    qj_minus_1: f64,
    vj: f64,
    qj: f64,
    fh: f64,
    fp: f64,
}

fn seed_mofe03_wave2_runtime_surface_inputs(
    runtime_surface: &mut HillslopeWritebackSurface,
) -> Result<(), HillslopeCliError> {
    let ofe_count = resolve_mofe03_ofe_count(runtime_surface)?;
    let wave2_enabled = resolve_mofe03_wave2_enabled(runtime_surface, ofe_count)?;
    write_mofe03_wave2_enabled(runtime_surface, wave2_enabled);
    if !wave2_enabled {
        return Ok(());
    }

    let slplen = require_mofe03_positive_runtime_surface_scalar(
        runtime_surface,
        "slplen",
        "Wave-2 seeding",
    )?;
    let qout = resolve_mofe03_wave2_qout(runtime_surface)?;
    let qin = resolve_mofe03_wave2_qin(runtime_surface)?;
    let qostar = (qout - qin).max(0.0);
    let case_scalars = build_mofe03_wave2_case_scalars(qout);

    seed_mofe03_wave2_core_scalars(runtime_surface, ofe_count, slplen, qout, qin, qostar)?;
    seed_mofe03_wave2_route_topology_ingress(runtime_surface, qostar);
    let (beta, theta) = resolve_mofe03_wave2_beta_theta(runtime_surface)?;
    seed_mofe03_wave2_case_state(runtime_surface, case_scalars, beta, theta);
    seed_mofe03_wave2_ssa_soil(runtime_surface)?;
    seed_mofe03_wave2_class_symbols(runtime_surface, ofe_count)?;
    Ok(())
}

fn resolve_mofe03_ofe_count(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<usize, HillslopeCliError> {
    let ofe_count = scalar_to_usize(
        "nelem",
        require_mofe03_runtime_surface_scalar(runtime_surface, "nelem")?,
    )?;
    if ofe_count == 0 {
        return Err(mofe03_wave2_seed_failure(
            "nelem must be >= 1 for MOFE03 activation policy",
        ));
    }
    Ok(ofe_count)
}

fn resolve_mofe03_wave2_enabled(
    runtime_surface: &HillslopeWritebackSurface,
    ofe_count: usize,
) -> Result<bool, HillslopeCliError> {
    if let Some(value) = runtime_surface_symbol_value(runtime_surface, "erod14_wave2_enabled") {
        parse_mofe03_binary_flag("erod14_wave2_enabled", value)
    } else {
        Ok(ofe_count > 1)
    }
}

fn write_mofe03_wave2_enabled(runtime_surface: &mut HillslopeWritebackSurface, enabled: bool) {
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_wave2_enabled"),
        BoundaryValue::scalar(if enabled { 1.0 } else { 0.0 }),
    );
}

fn require_mofe03_positive_runtime_surface_scalar(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &str,
    context: &str,
) -> Result<f64, HillslopeCliError> {
    let value = require_mofe03_runtime_surface_scalar(runtime_surface, symbol)?;
    if value <= 0.0 {
        return Err(mofe03_wave2_seed_failure(format!(
            "{symbol} must be > 0.0 for {context}, observed {value}"
        )));
    }
    Ok(value)
}

fn resolve_mofe03_wave2_qout(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<f64, HillslopeCliError> {
    require_mofe03_non_negative_seed_scalar(
        runtime_surface_symbol_value(runtime_surface, "Q")
            .or_else(|| runtime_surface_symbol_value(runtime_surface, "wb12_runoff_observed"))
            .unwrap_or(0.0),
        "erod14_qout",
    )
}

fn resolve_mofe03_wave2_qin(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<f64, HillslopeCliError> {
    require_mofe03_non_negative_seed_scalar(
        runtime_surface_symbol_value(runtime_surface, "UpStrmQ").unwrap_or(0.0),
        "erod14_qin",
    )
}

fn build_mofe03_wave2_case_scalars(qout: f64) -> Mofe03Wave2CaseScalars {
    if qout > MOFE03_WAVE2_ENABLE_TOLERANCE {
        return Mofe03Wave2CaseScalars {
            case_value: 2.0,
            qj_minus_1: qout.max(MOFE03_WAVE2_MIN_POSITIVE),
            vj: (0.25 * qout).max(MOFE03_WAVE2_MIN_POSITIVE),
            qj: (0.50 * qout).max(MOFE03_WAVE2_MIN_POSITIVE),
            fh: qout.max(MOFE03_WAVE2_MIN_POSITIVE),
            fp: (0.5 * qout).max(MOFE03_WAVE2_MIN_POSITIVE),
        };
    }
    Mofe03Wave2CaseScalars {
        case_value: 4.0,
        qj_minus_1: MOFE03_WAVE2_MIN_POSITIVE,
        vj: 0.0,
        qj: 0.0,
        fh: 0.0,
        fp: 0.0,
    }
}

fn seed_mofe03_wave2_core_scalars(
    runtime_surface: &mut HillslopeWritebackSurface,
    ofe_count: usize,
    slplen: f64,
    qout: f64,
    qin: f64,
    qostar: f64,
) -> Result<(), HillslopeCliError> {
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_class_count"),
        BoundaryValue::scalar(usize_to_scalar("erod14_class_count", ofe_count)?),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_xtop"),
        BoundaryValue::scalar(MOFE03_WAVE2_DEFAULT_XTOP),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_xbot"),
        BoundaryValue::scalar(MOFE03_WAVE2_DEFAULT_XBOT),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_xdetst"),
        BoundaryValue::scalar(MOFE03_WAVE2_DEFAULT_XDETST),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_ldtop"),
        BoundaryValue::scalar(MOFE03_WAVE2_DEFAULT_LDTOP),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_ldbot"),
        BoundaryValue::scalar(MOFE03_WAVE2_DEFAULT_LDBOT),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_lddend"),
        BoundaryValue::scalar(MOFE03_WAVE2_DEFAULT_LDDEND),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_qout"),
        BoundaryValue::scalar(qout),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_qin"),
        BoundaryValue::scalar(qin),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_qostar"),
        BoundaryValue::scalar(qostar.max(MOFE03_WAVE2_DEFAULT_QOSTAR)),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_slplen"),
        BoundaryValue::scalar(slplen),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_ktrato"),
        BoundaryValue::scalar(MOFE03_WAVE2_DEFAULT_KTRATO),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_ainftc"),
        BoundaryValue::scalar(MOFE03_WAVE2_DEFAULT_AINTC),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_binftc"),
        BoundaryValue::scalar(MOFE03_WAVE2_DEFAULT_BINTC),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_cinftc"),
        BoundaryValue::scalar(MOFE03_WAVE2_DEFAULT_CINTC),
    );
    Ok(())
}

fn seed_mofe03_wave2_route_topology_ingress(
    runtime_surface: &mut HillslopeWritebackSurface,
    qostar: f64,
) {
    let xu = runtime_surface_symbol_value(runtime_surface, "erod14_xtop")
        .unwrap_or(MOFE03_WAVE2_DEFAULT_XTOP);
    let xl = runtime_surface_symbol_value(runtime_surface, "erod14_xbot")
        .unwrap_or(MOFE03_WAVE2_DEFAULT_XBOT);
    let xdetst = runtime_surface_symbol_value(runtime_surface, "erod14_xdetst")
        .unwrap_or(MOFE03_WAVE2_DEFAULT_XDETST);
    let lddend = runtime_surface_symbol_value(runtime_surface, "erod14_lddend")
        .unwrap_or(MOFE03_WAVE2_DEFAULT_LDDEND);
    let ainftc = runtime_surface_symbol_value(runtime_surface, "erod14_ainftc")
        .unwrap_or(MOFE03_WAVE2_DEFAULT_AINTC);
    let binftc = runtime_surface_symbol_value(runtime_surface, "erod14_binftc")
        .unwrap_or(MOFE03_WAVE2_DEFAULT_BINTC);
    let cinftc = runtime_surface_symbol_value(runtime_surface, "erod14_cinftc")
        .unwrap_or(MOFE03_WAVE2_DEFAULT_CINTC);
    let segment = MOFE03_ROUTE_SEGMENT_INDEX;

    seed_mofe03_scalar_if_absent(
        runtime_surface,
        "qostar",
        qostar.max(MOFE03_WAVE2_DEFAULT_QOSTAR),
    );
    seed_mofe03_scalar_if_absent(runtime_surface, "xdetst", xdetst);
    seed_mofe03_scalar_if_absent(runtime_surface, "lddend", lddend);
    seed_mofe03_segment_scalar_if_absent(runtime_surface, "xu", segment, xu);
    seed_mofe03_segment_scalar_if_absent(runtime_surface, "xl", segment, xl);
    seed_mofe03_segment_scalar_if_absent(runtime_surface, "ainf", segment, ainftc);
    seed_mofe03_segment_scalar_if_absent(runtime_surface, "binf", segment, binftc);
    seed_mofe03_segment_scalar_if_absent(runtime_surface, "cinf", segment, cinftc);
    seed_mofe03_segment_scalar_if_absent(runtime_surface, "ainftc", segment, ainftc);
    seed_mofe03_segment_scalar_if_absent(runtime_surface, "binftc", segment, binftc);
    seed_mofe03_segment_scalar_if_absent(runtime_surface, "cinftc", segment, cinftc);
}

fn seed_mofe03_scalar_if_absent(
    runtime_surface: &mut HillslopeWritebackSurface,
    symbol: &str,
    value: f64,
) {
    if runtime_surface_symbol_value(runtime_surface, symbol).is_some() {
        return;
    }
    runtime_surface
        .state_surface
        .insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
}

fn seed_mofe03_segment_scalar_if_absent(
    runtime_surface: &mut HillslopeWritebackSurface,
    root: &str,
    segment_index: usize,
    value: f64,
) {
    let symbol = format!("{root}_{segment_index:04}");
    seed_mofe03_scalar_if_absent(runtime_surface, &symbol, value);
}

fn resolve_mofe03_wave2_beta_theta(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<(f64, f64), HillslopeCliError> {
    let beta = match runtime_surface_symbol_value(runtime_surface, "beta") {
        Some(value) => require_mofe03_non_negative_seed_scalar(value, "beta")?,
        None => MOFE03_WAVE2_DEFAULT_BETA,
    };
    let theta = if let Some(value) = runtime_surface_symbol_value(runtime_surface, "theta") {
        require_mofe03_non_negative_seed_scalar(value, "theta")?
    } else {
        let thetdr = require_mofe03_non_negative_seed_scalar(
            require_mofe03_runtime_surface_scalar(runtime_surface, "thetdr")?,
            "thetdr",
        )?;
        let thetfc = require_mofe03_non_negative_seed_scalar(
            require_mofe03_runtime_surface_scalar(runtime_surface, "thetfc")?,
            "thetfc",
        )?;
        0.5 * (thetdr + thetfc)
    };
    Ok((beta, theta))
}

fn seed_mofe03_wave2_case_state(
    runtime_surface: &mut HillslopeWritebackSurface,
    case_scalars: Mofe03Wave2CaseScalars,
    beta: f64,
    theta: f64,
) {
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_beta"),
        BoundaryValue::scalar(beta),
    );
    runtime_surface
        .state_surface
        .insert(BoundarySymbol::from("theta"), BoundaryValue::scalar(theta));
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_Qj_minus_1"),
        BoundaryValue::scalar(case_scalars.qj_minus_1),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_Vj"),
        BoundaryValue::scalar(case_scalars.vj),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_Qj"),
        BoundaryValue::scalar(case_scalars.qj),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_Fh"),
        BoundaryValue::scalar(case_scalars.fh),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_Fp"),
        BoundaryValue::scalar(case_scalars.fp),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_case"),
        BoundaryValue::scalar(case_scalars.case_value),
    );
}

fn seed_mofe03_wave2_ssa_soil(
    runtime_surface: &mut HillslopeWritebackSurface,
) -> Result<(), HillslopeCliError> {
    let ssa_soil = match runtime_surface_symbol_value(runtime_surface, "erod14_ssa_soil") {
        Some(value) => require_mofe03_positive_seed_scalar(value, "erod14_ssa_soil")?,
        None => MOFE03_WAVE2_DEFAULT_SSA_SOIL,
    };
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_ssa_soil"),
        BoundaryValue::scalar(ssa_soil),
    );
    Ok(())
}

fn seed_mofe03_wave2_class_symbols(
    runtime_surface: &mut HillslopeWritebackSurface,
    ofe_count: usize,
) -> Result<(), HillslopeCliError> {
    let class_count_f64 = usize_to_scalar("erod14_class_count", ofe_count)?;
    let class_fraction = 1.0 / class_count_f64;
    for class_index in 1..=ofe_count {
        let class_index_f64 = usize_to_scalar("erod14_class_index", class_index)?;
        let reverse_class_index = ofe_count.saturating_sub(class_index) + 1;
        let reverse_class_index_f64 =
            usize_to_scalar("erod14_reverse_class_index", reverse_class_index)?;
        let class_offset = class_index.saturating_sub(1);
        let class_offset_f64 = usize_to_scalar("erod14_class_offset", class_offset)?;

        seed_mofe03_wave2_class_symbol(
            runtime_surface,
            "erod14_fall",
            class_index,
            (0.02 / class_index_f64).max(MOFE03_WAVE2_MIN_POSITIVE),
        )?;
        seed_mofe03_wave2_class_symbol(
            runtime_surface,
            "erod14_frcflw",
            class_index,
            class_fraction,
        )?;
        seed_mofe03_wave2_class_symbol(
            runtime_surface,
            "erod14_frac",
            class_index,
            class_fraction,
        )?;
        seed_mofe03_wave2_class_symbol(
            runtime_surface,
            "erod14_fidel",
            class_index,
            (0.20 + (0.10 * class_index_f64)).min(0.95),
        )?;
        seed_mofe03_wave2_class_symbol(
            runtime_surface,
            "erod14_tcf1",
            class_index,
            0.20 + (0.05 * reverse_class_index_f64),
        )?;
        seed_mofe03_wave2_class_symbol(
            runtime_surface,
            "erod14_ssa_class",
            class_index,
            1.5 + (2.5 * class_offset_f64),
        )?;
    }
    Ok(())
}

#[allow(clippy::too_many_lines)]
fn execute_scheduler_kernel_lifecycle(
    runtime_surface: HillslopeWritebackSurface,
    context: SchedulerLifecycleContext<'_>,
) -> Result<DailyExecutionResult, HillslopeCliError> {
    let mut runtime_surface = runtime_surface;
    seed_wb11_runtime_surface_inputs(&mut runtime_surface, context.execution_lane)?;
    seed_scheduler_calendar_symbols(&mut runtime_surface, &context);
    let pl_activation_sentinel = pl_runtime_activation_sentinel_value(&runtime_surface);
    prepare_pl_runtime_activation_for_scheduler(&mut runtime_surface)?;
    let trace_day = context
        .hphys0245_trace_config
        .is_some_and(|config| config.includes_day(context.sim_day_index));
    let snow_runtime_before = trace_day.then(|| {
        Hphys0245SnowRuntimeBeforeState::from_surface(
            &runtime_surface,
            context.runtime_swe_before_m,
        )
    });
    let mut hphys0245_trace_rows = Vec::new();
    if trace_day {
        hphys0245_trace_rows.push(build_hphys0245_trace_row(
            context.run_name,
            context.simulation_year,
            context.sim_day_index,
            context.calendar_day.year,
            context.calendar_day.julian_day,
            "post_seed",
            None,
            &runtime_surface,
            None,
            snow_runtime_before,
        ));
    }

    let topology_graph = TopologyGraph::new(1, 0, 0, Vec::new());
    let topology_report = validate_pre_execution_topology(&topology_graph).map_err(|error| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "execution_provenance",
            detail: format!(
                "{SIMPIPE_GUARD_ID} failed building topology precondition report: {error}"
            ),
        }
    })?;

    let scheduler = HillslopePhaseScheduler::canonical();
    let execution_report = if trace_day {
        let mut kernel = Hphys0245TelemetryKernel::new(
            context.run_name,
            context.simulation_year,
            context.sim_day_index,
            context.calendar_day.year,
            context.calendar_day.julian_day,
            snow_runtime_before,
        );
        let report = scheduler
            .execute_with_kernel(&topology_report, &mut kernel, runtime_surface)
            .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "execution_provenance",
                detail: format!("{SIMPIPE_GUARD_ID} scheduler/kernel lifecycle failed: {error}"),
            })?;
        hphys0245_trace_rows.extend(kernel.into_rows());
        report
    } else {
        let mut kernel = Wb11HydrologyKernel;
        scheduler
            .execute_with_kernel(&topology_report, &mut kernel, runtime_surface)
            .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "execution_provenance",
                detail: format!("{SIMPIPE_GUARD_ID} scheduler/kernel lifecycle failed: {error}"),
            })?
    };

    if !execution_report.scheduler_report.is_success() {
        let scheduler_status = &execution_report.scheduler_report.scheduler_status;
        let phase_context = execution_report
            .phase_reports
            .last()
            .map(|phase_report| {
                let mut context = format!(
                    ", last_phase={}, last_kernel_message_id={}, last_decision_outcome={:?}, last_decision_message_id={}",
                    phase_report.phase.as_str(),
                    phase_report.kernel_status.message_id(),
                    phase_report.decision_outcome,
                    phase_report.decision_status.message_id()
                );

                if !phase_report.decision_violations.is_empty() {
                    let violation_summary = phase_report
                        .decision_violations
                        .iter()
                        .take(3)
                        .map(|violation| {
                            format!(
                                "{}:{}:{:?}",
                                violation.check_id, violation.subject, violation.details
                            )
                        })
                        .collect::<Vec<_>>()
                        .join(" | ");
                    context.push_str(", last_decision_violations=");
                    context.push_str(&violation_summary);
                }
                if phase_report.phase.as_str() == "storage_reconciliation" {
                    context.push_str(", wb12_terms=");
                    context.push_str(&format_wb12_storage_terms(
                        &execution_report.writeback_surface,
                    ));
                }
                if phase_report.phase.as_str() == "percolation_deep_seepage"
                    && phase_report.kernel_status.message_id() == "HKERNEL-WB11-PERC-E-003"
                {
                    context.push_str(", wb18_guard_terms=");
                    context.push_str(&format_wb18_perc_guard_terms(
                        &execution_report.writeback_surface,
                    ));
                }

                context
            })
            .unwrap_or_default();
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "execution_provenance",
            detail: format!(
                "{SIMPIPE_GUARD_ID} scheduler lifecycle did not complete successfully (outcome_class={}, status_class={:?}, boundary_class={}, message_id={}{})",
                scheduler_outcome_class_as_str(execution_report.scheduler_report.outcome_class),
                scheduler_status.classification(),
                scheduler_status.boundary_class().as_str(),
                scheduler_status.message_id(),
                phase_context
            ),
        });
    }

    let mut writeback_surface = execution_report.writeback_surface.clone();
    restore_pl_runtime_activation_sentinel_for_next_day(
        &mut writeback_surface,
        pl_activation_sentinel,
    );

    if trace_day {
        hphys0245_trace_rows.push(build_hphys0245_trace_row(
            context.run_name,
            context.simulation_year,
            context.sim_day_index,
            context.calendar_day.year,
            context.calendar_day.julian_day,
            "post_scheduler",
            None,
            &writeback_surface,
            None,
            snow_runtime_before,
        ));
    }

    let wb13_row = build_simulation_owned_wb13_row(
        &writeback_surface,
        context.publication_area_m2,
        context.simulation_year,
        context.sim_day_index,
        context.calendar_day,
        context.runtime_swe_before_m,
    )?;
    if trace_day {
        hphys0245_trace_rows.push(build_hphys0245_trace_row(
            context.run_name,
            context.simulation_year,
            context.sim_day_index,
            context.calendar_day.year,
            context.calendar_day.julian_day,
            "post_wb13",
            None,
            &writeback_surface,
            Some(&wb13_row),
            snow_runtime_before,
        ));
    }
    let coupling_vectors =
        build_simimpl10_coupling_vector_provenance(&writeback_surface, &wb13_row)?;
    let kernel_phase_message_ids = execution_report
        .phase_reports
        .iter()
        .map(|phase| phase.kernel_status.message_id().to_string())
        .collect::<Vec<_>>();
    let erod14_wave2_kernel_status_seen = execution_report.phase_reports.iter().any(|phase| {
        let message_id = phase.kernel_status.message_id();
        message_id.contains("EROD14-WAVE2")
            || message_id.contains("EROD18-ROUTE")
            || message_id.contains("EROD19-ROUTE")
    });

    Ok(DailyExecutionResult {
        scheduler_outcome_class: execution_report.scheduler_report.outcome_class,
        scheduler_status_message_id: execution_report
            .scheduler_report
            .scheduler_status
            .message_id()
            .to_string(),
        coupling_vectors,
        wb13_row,
        runtime_surface: writeback_surface,
        kernel_phase_message_ids,
        erod14_wave2_kernel_status_seen,
        hphys0245_trace_rows,
    })
}

fn pl_runtime_activation_sentinel_value(
    runtime_surface: &HillslopeWritebackSurface,
) -> Option<BoundaryValue> {
    runtime_surface
        .state_surface
        .get(&BoundarySymbol::from("pl_schedule_slot_count"))
        .copied()
}

fn seed_scheduler_calendar_symbols(
    runtime_surface: &mut HillslopeWritebackSurface,
    context: &SchedulerLifecycleContext<'_>,
) {
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("year"),
        BoundaryValue::scalar(f64::from(context.simulation_year)),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("day"),
        BoundaryValue::scalar(f64::from(context.calendar_day.julian_day)),
    );
}

fn restore_pl_runtime_activation_sentinel_for_next_day(
    runtime_surface: &mut HillslopeWritebackSurface,
    sentinel_value: Option<BoundaryValue>,
) {
    if let Some(value) = sentinel_value {
        runtime_surface
            .state_surface
            .entry(BoundarySymbol::from("pl_schedule_slot_count"))
            .or_insert(value);
    }
}

fn prepare_pl_runtime_activation_for_scheduler(
    runtime_surface: &mut HillslopeWritebackSurface,
) -> Result<(), HillslopeCliError> {
    const PL_SCHEDULE_SLOT_COUNT_SYMBOL: &str = "pl_schedule_slot_count";

    if runtime_surface_symbol_value(runtime_surface, PL_SCHEDULE_SLOT_COUNT_SYMBOL).is_none() {
        return Ok(());
    }

    if pl_runtime_has_active_crop_for_scheduler_day(runtime_surface)? {
        return Ok(());
    }

    runtime_surface
        .state_surface
        .remove(&BoundarySymbol::from(PL_SCHEDULE_SLOT_COUNT_SYMBOL));
    Ok(())
}

fn pl_runtime_has_active_crop_for_scheduler_day(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<bool, HillslopeCliError> {
    let slot_count = require_runtime_usize_in_range(runtime_surface, "pl_schedule_slot_count", 1)?;
    let rotation_years =
        require_runtime_usize_in_range(runtime_surface, "pl_schedule_rotation_years", 1)?;
    let rotation_repeats =
        require_runtime_usize_in_range(runtime_surface, "pl_schedule_rotation_repeats", 1)?;
    let runtime_year = require_runtime_usize_in_range(runtime_surface, "year", 1)?;
    let day_of_year = require_runtime_usize_in_range(runtime_surface, "day", 1)?;
    if day_of_year > 366 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pl_runtime_activation",
            detail: format!("day must be in 1..=366 for PL activation, observed {day_of_year}"),
        });
    }

    let max_runtime_year = rotation_repeats
        .checked_mul(rotation_years)
        .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pl_runtime_activation",
            detail: "rotation_repeats * rotation_years overflowed".to_string(),
        })?;
    if runtime_year > max_runtime_year {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pl_runtime_activation",
            detail: format!(
                "year must be in 1..={max_runtime_year} for PL activation, observed {runtime_year}"
            ),
        });
    }

    let rotation_index = ((runtime_year - 1) / rotation_years) + 1;
    let year_in_rotation = ((runtime_year - 1) % rotation_years) + 1;
    let mut slot_candidates = Vec::new();
    for slot_index in 1..=slot_count {
        let ofe_index = require_runtime_usize_in_range(
            runtime_surface,
            &pl_schedule_slot_symbol("ofe_index", slot_index),
            1,
        )?;
        if ofe_index != 1 {
            continue;
        }
        let slot_year_in_rotation = require_runtime_usize_in_range(
            runtime_surface,
            &pl_schedule_slot_symbol("year_in_rotation", slot_index),
            1,
        )?;
        let slot_rotation_index = require_runtime_usize_in_range(
            runtime_surface,
            &pl_schedule_slot_symbol("rotation_index", slot_index),
            1,
        )?;
        if slot_year_in_rotation == year_in_rotation && slot_rotation_index == rotation_index {
            slot_candidates.push(slot_index);
        }
    }

    let [slot_index] = slot_candidates.as_slice() else {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pl_runtime_activation",
            detail: format!(
                "expected exactly one active PL slot for ofe=1 year_in_rotation={year_in_rotation} rotation_index={rotation_index}, observed {slot_candidates:?}"
            ),
        });
    };

    let crop_slots = require_runtime_usize_in_range(
        runtime_surface,
        &pl_schedule_slot_symbol("crop_slots", *slot_index),
        1,
    )?;
    let mut active_crop_count = 0usize;
    for crop_slot_index in 1..=crop_slots {
        if pl_crop_slot_is_active_for_day(
            runtime_surface,
            *slot_index,
            crop_slot_index,
            day_of_year,
        )? {
            active_crop_count += 1;
        }
    }

    match active_crop_count {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pl_runtime_activation",
            detail: format!(
                "expected at most one active PL crop for slot {slot_index} day {day_of_year}, observed {active_crop_count}"
            ),
        }),
    }
}

fn pl_crop_slot_is_active_for_day(
    runtime_surface: &HillslopeWritebackSurface,
    slot_index: usize,
    crop_slot_index: usize,
    day_of_year: usize,
) -> Result<bool, HillslopeCliError> {
    let imngmt = require_runtime_usize_in_range(
        runtime_surface,
        &pl_schedule_slot_crop_symbol("imngmt", slot_index, crop_slot_index),
        1,
    )?;
    if imngmt > 3 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pl_runtime_activation",
            detail: format!("imngmt must be in 1..=3 for PL activation, observed {imngmt}"),
        });
    }

    let jdplt = require_runtime_usize_in_range(
        runtime_surface,
        &pl_growth_slot_crop_symbol("jdplt", slot_index, crop_slot_index),
        0,
    )?;
    if jdplt > 366 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pl_runtime_activation",
            detail: format!("jdplt must be in 0..=366 for PL activation, observed {jdplt}"),
        });
    }
    let jdharv = require_runtime_usize_in_range(
        runtime_surface,
        &pl_growth_slot_crop_symbol("jdharv", slot_index, crop_slot_index),
        0,
    )?;
    if jdharv > 366 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pl_runtime_activation",
            detail: format!("jdharv must be in 0..=366 for PL activation, observed {jdharv}"),
        });
    }

    let (active_end, jdstop) = if imngmt == 2 {
        let jdstop = require_runtime_usize_in_range(
            runtime_surface,
            &pl_growth_slot_crop_symbol("jdstop", slot_index, crop_slot_index),
            0,
        )?;
        if jdstop > 366 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "pl_runtime_activation",
                detail: format!("jdstop must be in 0..=366 for PL activation, observed {jdstop}"),
            });
        }
        if jdplt == 0 {
            return Ok(jdstop == 0 || day_of_year <= jdstop);
        }
        let active_end = if jdstop == 0 { jdharv.max(1) } else { jdstop };
        (active_end, jdstop)
    } else {
        (jdharv.max(1), 0)
    };

    if jdplt == 0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pl_runtime_activation",
            detail: format!(
                "jdplt must be in 1..=366 for non-perennial PL activation, observed jdplt={jdplt} jdharv={jdharv} jdstop={jdstop}"
            ),
        });
    }

    Ok(day_is_within_julian_window(day_of_year, jdplt, active_end))
}

fn require_runtime_usize_in_range(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &str,
    min_allowed: usize,
) -> Result<usize, HillslopeCliError> {
    let value = require_runtime_surface_scalar(runtime_surface, symbol)?;
    let value = scalar_to_usize(symbol, value)?;
    if value < min_allowed {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pl_runtime_activation",
            detail: format!(
                "{symbol} must be >= {min_allowed} for PL activation, observed {value}"
            ),
        });
    }
    Ok(value)
}

fn day_is_within_julian_window(day_of_year: usize, start_day: usize, end_day: usize) -> bool {
    if start_day <= end_day {
        day_of_year >= start_day && day_of_year <= end_day
    } else {
        day_of_year >= start_day || day_of_year <= end_day
    }
}

fn pl_schedule_slot_symbol(root: &str, slot_index: usize) -> String {
    format!("pl_schedule_slot_{slot_index:04}_{root}")
}

fn pl_schedule_slot_crop_symbol(root: &str, slot_index: usize, crop_slot_index: usize) -> String {
    format!("pl_schedule_slot_{slot_index:04}_crop_{crop_slot_index:04}_{root}")
}

fn pl_growth_slot_crop_symbol(root: &str, slot_index: usize, crop_slot_index: usize) -> String {
    format!("pl_growth_slot_{slot_index:04}_crop_{crop_slot_index:04}_{root}")
}

fn hphys0245_trace_config_from_env() -> Result<Option<Hphys0245TraceConfig>, HillslopeCliError> {
    let Some(path_value) = std::env::var_os(HPHYS0245_TRACE_PATH_ENV) else {
        return Ok(None);
    };
    if path_value.is_empty() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "hphys0245_trace",
            detail: format!("{HPHYS0245_TRACE_PATH_ENV} cannot be empty when set"),
        });
    }

    let max_days = match std::env::var(HPHYS0245_TRACE_MAX_DAYS_ENV) {
        Ok(value) => {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                None
            } else {
                let parsed = trimmed.parse::<usize>().map_err(|error| {
                    HillslopeCliError::RuntimeSurfaceFailure {
                        surface: "hphys0245_trace",
                        detail: format!(
                            "{HPHYS0245_TRACE_MAX_DAYS_ENV} must be a positive integer, observed {trimmed}: {error}"
                        ),
                    }
                })?;
                if parsed == 0 {
                    return Err(HillslopeCliError::RuntimeSurfaceFailure {
                        surface: "hphys0245_trace",
                        detail: format!("{HPHYS0245_TRACE_MAX_DAYS_ENV} must be >= 1"),
                    });
                }
                Some(parsed)
            }
        }
        Err(std::env::VarError::NotPresent) => None,
        Err(std::env::VarError::NotUnicode(_)) => {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "hphys0245_trace",
                detail: format!("{HPHYS0245_TRACE_MAX_DAYS_ENV} must be valid UTF-8"),
            });
        }
    };

    Ok(Some(Hphys0245TraceConfig {
        path: PathBuf::from(path_value),
        max_days,
    }))
}

fn write_hphys0245_trace_jsonl(
    config: &Hphys0245TraceConfig,
    rows: &[Hphys0245TraceRow],
) -> Result<(), HillslopeCliError> {
    ensure_output_parent_directory(&config.path)?;
    let mut payload = String::new();
    for row in rows {
        let line = serde_json::to_string(row).map_err(|source| {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "hphys0245_trace",
                detail: format!("failed serializing trace row: {source}"),
            }
        })?;
        payload.push_str(&line);
        payload.push('\n');
    }
    fs::write(&config.path, payload).map_err(|source| HillslopeCliError::OutputWrite {
        path: config.path.clone(),
        source,
    })
}

fn hphys0245_surface_after_writeback(
    request: &HillslopeKernelRequest<'_>,
    payload: &KernelWritebackPayload,
) -> HillslopeWritebackSurface {
    let mut surface = HillslopeWritebackSurface {
        state_surface: request.state_surface.clone(),
        flux_surface: request.flux_surface.clone(),
    };
    for field in &payload.state_updates {
        surface
            .state_surface
            .insert(field.symbol.clone(), field.value);
    }
    for field in &payload.flux_updates {
        surface
            .flux_surface
            .insert(field.symbol.clone(), field.value);
    }
    surface
}

fn hphys0245_et_seed_branch(runtime_surface: &HillslopeWritebackSurface) -> Option<String> {
    if runtime_surface_symbol_value(runtime_surface, "wb11_et_seed_branch_evappm")
        .is_some_and(|value| value >= 0.5)
    {
        return Some("evappm_pmet".to_string());
    }
    if runtime_surface_symbol_value(runtime_surface, "wb11_et_seed_branch_priestley_taylor")
        .is_some_and(|value| value >= 0.5)
    {
        return Some("evap_priestley_taylor".to_string());
    }
    None
}

fn hphys0245_optional_delta(after: Option<f64>, before: Option<f64>) -> Option<f64> {
    match (after, before) {
        (Some(after), Some(before)) => Some(after - before),
        _ => None,
    }
}

#[allow(
    clippy::similar_names,
    clippy::too_many_arguments,
    clippy::too_many_lines
)]
fn build_hphys0245_trace_row(
    run_name: &str,
    simulation_year: i32,
    sim_day_index: usize,
    calendar_year: i32,
    julian_day: u16,
    boundary: &str,
    phase: Option<&str>,
    runtime_surface: &HillslopeWritebackSurface,
    wb13_row: Option<&SimulationOwnedWb13Row>,
    snow_runtime_before: Option<Hphys0245SnowRuntimeBeforeState>,
) -> Hphys0245TraceRow {
    let theta_layers =
        hphys0245_prefixed_surface_values(&runtime_surface.state_surface, "wb18_perc_theta_");
    let wb18_ul_layers_m =
        hphys0245_prefixed_surface_values(&runtime_surface.state_surface, "wb18_perc_ul_");
    let wb18_thetdr_layers = hphys0245_prefixed_surface_values_with_fallback(
        &runtime_surface.state_surface,
        "wb19_thetdr_",
        "thetdr_",
    );
    let wb18_dg_layers_m = hphys0245_prefixed_surface_values_with_fallback(
        &runtime_surface.state_surface,
        "wb19_dg_",
        "dg_",
    );
    let wb18_fc_layers_m =
        hphys0245_prefixed_surface_values(&runtime_surface.state_surface, "wb18_perc_fc_");
    let wb19_coca_layers = hphys0245_prefixed_surface_values_with_fallback(
        &runtime_surface.state_surface,
        "wb19_coca_",
        "coca_",
    );
    let wb19_frzw_layers_m =
        hphys0245_prefixed_surface_values(&runtime_surface.state_surface, "wb18_perc_frzw_");
    let wb19_drfc_layers_m =
        hphys0245_wb19_drfc_layers(&wb18_fc_layers_m, &wb18_dg_layers_m, &wb19_coca_layers);
    let wb19_fzdrfc_layers_m =
        hphys0245_wb19_fzdrfc_layers(&wb19_drfc_layers_m, &wb19_frzw_layers_m);
    let wb18_frozen_depth_layers_m = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "wb18_perc_frozen_depth_",
    );
    let pei_layers =
        hphys0245_prefixed_surface_values(&runtime_surface.flux_surface, "wb18_perc_pei_");
    let potential_uptake_layers_m =
        hphys0245_prefixed_surface_values(&runtime_surface.flux_surface, "UPi_");
    let actual_uptake_layers_m =
        hphys0245_prefixed_surface_values(&runtime_surface.flux_surface, "Ui_");
    let wb19_lateral_withdrawal_layers_m = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "wb19_lateral_withdrawal_",
    );
    let wb19_lateral_capacity_active_count_layers = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "wb19_lateral_capacity_active_count_",
    );
    let wb19_lateral_conductivity_active_count_layers = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "wb19_lateral_conductivity_active_count_",
    );
    let theta_sum = hphys0245_sum_or_none(&theta_layers);
    let pei_sum = hphys0245_sum_or_none(&pei_layers);
    let wb11_soil_water = runtime_surface_symbol_value(runtime_surface, "wb11_soil_water");
    let wb12_infiltration_m = runtime_surface_symbol_value(runtime_surface, "wb12_infiltration");
    let wb12_rainfall_input_m =
        runtime_surface_symbol_value(runtime_surface, "wb12_rainfall_input");
    let wb12_runon_input_m = runtime_surface_symbol_value(runtime_surface, "wb12_runon_input");
    let wb12_depression_storage_delta_m =
        runtime_surface_symbol_value(runtime_surface, "wb12_depression_storage_delta");
    let wb14_soil_conductivity_m_s =
        runtime_surface_symbol_value(runtime_surface, "wb14_soil_conductivity_m_s")
            .or_else(|| runtime_surface_symbol_value(runtime_surface, "ssc"));
    let wb14_frost_infcap_m_s =
        runtime_surface_symbol_value(runtime_surface, "frost.runtime_infcap_frz");
    let wb14_effective_conductivity_m_s =
        runtime_surface_symbol_value(runtime_surface, "wb14_effective_conductivity_m_s")
            .or_else(|| wb14_frost_infcap_m_s.or(wb14_soil_conductivity_m_s));
    let wb14_soil_layer_depth_m = runtime_surface_symbol_value(runtime_surface, "dg");
    let wb14_theta_residual = runtime_surface_symbol_value(runtime_surface, "thetdr");
    let wb14_theta_field_capacity = runtime_surface_symbol_value(runtime_surface, "thetfc");
    let wb14_matric_potential_m =
        runtime_surface_symbol_value(runtime_surface, "wb14_matric_potential_m").or_else(
            || match (
                wb14_soil_layer_depth_m,
                wb14_theta_residual,
                wb14_theta_field_capacity,
            ) {
                (Some(depth), Some(theta_residual), Some(theta_field_capacity)) => {
                    Some(depth * (theta_field_capacity - theta_residual).max(0.0))
                }
                _ => None,
            },
        );
    let wb18_recomputed_soil_water_m = hphys0245_recompute_wb18_soil_water(
        &theta_layers,
        &wb18_thetdr_layers,
        &wb18_dg_layers_m,
        &wb18_frozen_depth_layers_m,
    );
    let wb18_recomputed_minus_wb11_m = match (wb18_recomputed_soil_water_m, wb11_soil_water) {
        (Some(recomputed), Some(wb11)) => Some(recomputed - wb11),
        _ => None,
    };
    let wb11_minus_theta_sum_m = match (wb11_soil_water, theta_sum) {
        (Some(wb11), Some(theta)) => Some(wb11 - theta),
        _ => None,
    };
    let wb13_wat = wb13_row.map(|row| &row.wb13_row);
    let effective_pltol = runtime_surface_symbol_value(runtime_surface, "swu_effective_pltol");
    let wb17_swu_stress_threshold_layers_m =
        hphys0245_swu_stress_threshold_layers(&wb18_ul_layers_m, effective_pltol);
    let wb17_swu_storage_to_threshold_layers = hphys0245_swu_storage_to_threshold_layers(
        &theta_layers,
        &wb17_swu_stress_threshold_layers_m,
    );
    let snow_hourly_rain_sum_m = Some(hphys0245_sum_runtime_prefix(
        runtime_surface,
        "snow.hourly.rain_m_",
    ));
    let snow_hourly_snowfall_depth_sum_m = Some(hphys0245_sum_runtime_prefix(
        runtime_surface,
        "snow.hourly.snowfall_m_",
    ));
    let snow_hourly_melt_sum_m = Some(hphys0245_sum_runtime_prefix(
        runtime_surface,
        "snow.hourly.melt_m_",
    ));
    let snow_hourly_melt_raw_sum_m = Some(hphys0245_sum_runtime_prefix(
        runtime_surface,
        "snow.hourly.melt_raw_m_",
    ));
    let snow_hourly_rain_retained_sum_m = Some(hphys0245_sum_runtime_prefix(
        runtime_surface,
        "snow.hourly.rain_retained_m_",
    ));
    let snow_hourly_rain_released_sum_m = Some(hphys0245_sum_runtime_prefix(
        runtime_surface,
        "snow.hourly.rain_released_m_",
    ));
    let snow_hourly_rain_m =
        hphys0245_prefixed_surface_values(&runtime_surface.state_surface, "snow.hourly.rain_m_");
    let snow_hourly_snowfall_depth_m = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.snowfall_m_",
    );
    let snow_hourly_stmtim_rain_m = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.stmtim.rain_m_",
    );
    let snow_hourly_stmtim_stmdur_s = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.stmtim.stmdur_s_",
    );
    let snow_hourly_stmtim_wntdur_h = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.stmtim.wntdur_h_",
    );
    let snow_hourly_stmtim_wnttim_h = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.stmtim.wnttim_h_",
    );
    let snow_hourly_stmtim_hrtemp_c = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.stmtim.hrtemp_c_",
    );
    let snow_hourly_stmtim_rst_c = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.stmtim.rst_c_",
    );
    let snow_hourly_stmtim_hrrain_m = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.stmtim.hrrain_m_",
    );
    let snow_hourly_stmtim_hrsnow_m = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.stmtim.hrsnow_m_",
    );
    let snow_hourly_stmtim_active_interval = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.stmtim.active_interval_",
    );
    let snow_hourly_stmtim_rain_branch = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.stmtim.rain_branch_",
    );
    let snow_hourly_stmtim_snow_branch = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.stmtim.snow_branch_",
    );
    let snow_hourly_depth_before_m = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.depth_before_m_",
    );
    let snow_hourly_depth_available_m = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.depth_available_m_",
    );
    let snow_hourly_depth_after_m = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.depth_after_m_",
    );
    let snow_hourly_density_before_kg_m3 = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.density_before_kg_m3_",
    );
    let snow_hourly_density_after_kg_m3 = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.density_after_kg_m3_",
    );
    let snow_hourly_melt_raw_m = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.melt_raw_m_",
    );
    let snow_hourly_melt_m =
        hphys0245_prefixed_surface_values(&runtime_surface.state_surface, "snow.hourly.melt_m_");
    let snow_hourly_melt_amelt_in = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.melt_amelt_in_",
    );
    let snow_hourly_melt_bmelt_in = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.melt_bmelt_in_",
    );
    let snow_hourly_melt_cmelt_in = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.melt_cmelt_in_",
    );
    let snow_hourly_melt_dmelt_in = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.melt_dmelt_in_",
    );
    let snow_hourly_melt_hrtef_f = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.melt_hrtef_f_",
    );
    let snow_hourly_melt_hrdtf_f = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.melt_hrdtf_f_",
    );
    let snow_hourly_melt_vwmph = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.melt_vwmph_",
    );
    let snow_hourly_melt_rainin = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.melt_rainin_",
    );
    let snow_hourly_melt_wind_adjustment = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.melt_wind_adjustment_",
    );
    let snow_hourly_melt_branch_active = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.melt_branch_active_",
    );
    let winter_hourly_air_temp_c = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "winter.hourly.air_temp_c_",
    );
    let winter_hourly_rad_mj_m2 = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "winter.hourly.rad_mj_m2_",
    );
    let winter_hourly_cloud_fraction = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "winter.hourly.cloud_fraction_",
    );
    let winter_hourly_dewpoint_c = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "winter.hourly.dewpoint_c_",
    );
    let winter_hourly_wind_m_s = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "winter.hourly.wind_m_s_",
    );
    let snow_hourly_snowfall_water_equiv_sum_m = match (
        snow_hourly_snowfall_depth_sum_m,
        runtime_surface_symbol_value(runtime_surface, "snow.options.newsnw"),
    ) {
        (Some(depth_sum_m), Some(new_snow_density_kg_m3)) => {
            Some(depth_sum_m * new_snow_density_kg_m3 / 1_000.0)
        }
        _ => None,
    };
    let snow_s_m = runtime_surface_symbol_value_prefer_flux(runtime_surface, "S");
    let snow_routed_melt_m =
        runtime_surface_flux_symbol_value(runtime_surface, "snow.routed_melt_m");
    let snow_post_winter_rain_m =
        runtime_surface_flux_symbol_value(runtime_surface, "snow.post_winter_rain_m");
    let wb12_partition_liquid_supply_m = match (
        snow_post_winter_rain_m,
        snow_routed_melt_m,
        wb12_runon_input_m,
    ) {
        (Some(post_winter_rain), Some(routed_melt), Some(runon)) => {
            Some(post_winter_rain + routed_melt + runon)
        }
        _ => None,
    };
    let wb12_partition_residual_before_q_m = match (
        wb12_partition_liquid_supply_m,
        wb12_infiltration_m,
        wb12_depression_storage_delta_m,
    ) {
        (Some(supply), Some(infiltration), Some(depression_storage_delta)) => {
            Some(supply - infiltration - depression_storage_delta)
        }
        _ => None,
    };
    let snow_runtime_swe_m = runtime_surface_symbol_value(runtime_surface, "snow.runtime_swe");
    let snow_runtime_depth_m =
        runtime_surface_symbol_value(runtime_surface, "snow.runtime_depth_m");
    let snow_runtime_density_kg_m3 =
        runtime_surface_symbol_value(runtime_surface, "snow.runtime_density_kg_m3");
    let snow_runtime_settle_day_count =
        runtime_surface_symbol_value(runtime_surface, "snow.runtime_settle_day_count");
    let snow_runtime_swe_before_m = snow_runtime_before.and_then(|state| state.swe_m);
    let snow_runtime_depth_before_m = snow_runtime_before.and_then(|state| state.depth_m);
    let snow_runtime_density_before_kg_m3 =
        snow_runtime_before.and_then(|state| state.density_kg_m3);
    let snow_runtime_settle_day_count_before =
        snow_runtime_before.and_then(|state| state.settle_day_count);
    let snow_runtime_swe_delta_m =
        hphys0245_optional_delta(snow_runtime_swe_m, snow_runtime_swe_before_m);
    let snow_runtime_depth_delta_m =
        hphys0245_optional_delta(snow_runtime_depth_m, snow_runtime_depth_before_m);
    let snow_runtime_density_delta_kg_m3 = hphys0245_optional_delta(
        snow_runtime_density_kg_m3,
        snow_runtime_density_before_kg_m3,
    );
    let snow_runtime_settle_day_count_delta = hphys0245_optional_delta(
        snow_runtime_settle_day_count,
        snow_runtime_settle_day_count_before,
    );
    let snow_runtime_swe_closure_error_m = match (
        snow_s_m,
        snow_hourly_melt_sum_m,
        snow_hourly_snowfall_water_equiv_sum_m,
        snow_hourly_rain_retained_sum_m,
        snow_hourly_rain_released_sum_m,
    ) {
        (
            Some(snow_s_m),
            Some(melt_sum_m),
            Some(snowfall_water_equiv_sum_m),
            Some(rain_retained_sum_m),
            Some(rain_released_sum_m),
        ) => Some(
            snow_s_m
                - ((melt_sum_m - rain_released_sum_m)
                    - snowfall_water_equiv_sum_m
                    - rain_retained_sum_m),
        ),
        _ => None,
    };

    Hphys0245TraceRow {
        schema: HPHYS0245_TRACE_SCHEMA,
        run_name: run_name.to_string(),
        sim_day_index,
        simulation_year,
        calendar_year,
        julian_day,
        boundary: boundary.to_string(),
        phase: phase.map(ToString::to_string),
        wb11_soil_water_m: wb11_soil_water,
        wb11_soil_water_mm: wb11_soil_water.map(|value| value * 1_000.0),
        wb12_infiltration_m,
        wb12_rainfall_input_m,
        wb12_runon_input_m,
        wb12_depression_storage_delta_m,
        wb12_partition_liquid_supply_m,
        wb12_partition_residual_before_q_m,
        wb14_soil_conductivity_m_s,
        wb14_frost_infcap_m_s,
        wb14_effective_conductivity_m_s,
        wb14_soil_layer_depth_m,
        wb14_theta_residual,
        wb14_theta_field_capacity,
        wb14_matric_potential_m,
        wb18_theta_sum_m: theta_sum,
        wb18_theta_layers_m: theta_layers,
        wb18_thetdr_layers,
        wb18_dg_layers_m,
        wb18_fc_layers_m,
        wb19_coca_layers,
        wb19_frzw_layers_m,
        wb19_drfc_layers_m,
        wb19_fzdrfc_layers_m,
        wb18_frozen_depth_layers_m,
        wb18_recomputed_soil_water_m,
        wb18_recomputed_minus_wb11_m,
        wb18_pei_sum_m: pei_sum,
        wb18_pei_layers_m: pei_layers,
        d_m: runtime_surface_symbol_value_prefer_flux(runtime_surface, "D"),
        pe_m: runtime_surface_symbol_value_prefer_flux(runtime_surface, "Pe"),
        wb13_dp_mm: wb13_wat.map(|row| row.dp),
        wb13_total_soil_mm: wb13_wat.map(|row| row.total_soil),
        wb13_soil_water_total_mm: wb13_wat.map(|row| row.soil_water_total),
        snow_runtime_swe_m,
        snow_runtime_depth_m,
        snow_runtime_density_kg_m3,
        snow_runtime_settle_day_count,
        snow_runtime_swe_before_m,
        snow_runtime_depth_before_m,
        snow_runtime_density_before_kg_m3,
        snow_runtime_settle_day_count_before,
        snow_runtime_swe_delta_m,
        snow_runtime_depth_delta_m,
        snow_runtime_density_delta_kg_m3,
        snow_runtime_settle_day_count_delta,
        snow_s_m,
        snow_routed_melt_m,
        snow_post_winter_rain_m,
        snow_hourly_rain_sum_m,
        snow_hourly_rain_retained_sum_m,
        snow_hourly_rain_released_sum_m,
        snow_hourly_snowfall_depth_sum_m,
        snow_hourly_snowfall_water_equiv_sum_m,
        snow_hourly_melt_raw_sum_m,
        snow_hourly_melt_sum_m,
        snow_hourly_rain_m,
        snow_hourly_snowfall_depth_m,
        snow_hourly_stmtim_rain_m,
        snow_hourly_stmtim_stmdur_s,
        snow_hourly_stmtim_wntdur_h,
        snow_hourly_stmtim_wnttim_h,
        snow_hourly_stmtim_hrtemp_c,
        snow_hourly_stmtim_rst_c,
        snow_hourly_stmtim_hrrain_m,
        snow_hourly_stmtim_hrsnow_m,
        snow_hourly_stmtim_active_interval,
        snow_hourly_stmtim_rain_branch,
        snow_hourly_stmtim_snow_branch,
        snow_hourly_depth_before_m,
        snow_hourly_depth_available_m,
        snow_hourly_depth_after_m,
        snow_hourly_density_before_kg_m3,
        snow_hourly_density_after_kg_m3,
        snow_hourly_melt_raw_m,
        snow_hourly_melt_m,
        snow_hourly_melt_amelt_in,
        snow_hourly_melt_bmelt_in,
        snow_hourly_melt_cmelt_in,
        snow_hourly_melt_dmelt_in,
        snow_hourly_melt_hrtef_f,
        snow_hourly_melt_hrdtf_f,
        snow_hourly_melt_vwmph,
        snow_hourly_melt_rainin,
        snow_hourly_melt_wind_adjustment,
        snow_hourly_melt_branch_active,
        winter_hourly_air_temp_c,
        winter_hourly_rad_mj_m2,
        winter_hourly_cloud_fraction,
        winter_hourly_dewpoint_c,
        winter_hourly_wind_m_s,
        snow_runtime_swe_closure_error_m,
        wb13_p_mm: wb13_wat.map(|row| row.p),
        wb13_rm_mm: wb13_wat.map(|row| row.rm),
        wb13_q_mm: wb13_wat.map(|row| row.q),
        wb13_snow_water_mm: wb13_wat.map(|row| row.snow_water),
        wb11_minus_theta_sum_m,
        pl_sumgdd: runtime_surface_symbol_value(runtime_surface, "sumgdd"),
        pl_vdmt: runtime_surface_symbol_value(runtime_surface, "vdmt"),
        pl_cancov: runtime_surface_symbol_value(runtime_surface, "cancov"),
        pl_lai: runtime_surface_symbol_value(runtime_surface, "lai"),
        pl_rtmass: runtime_surface_symbol_value(runtime_surface, "rtmass"),
        pl_rtd: runtime_surface_symbol_value(runtime_surface, "rtd"),
        pl_hia: runtime_surface_symbol_value(runtime_surface, "hia"),
        pl_pltol: runtime_surface_symbol_value(runtime_surface, "pltol"),
        pl_swu_effective_pltol: effective_pltol,
        pmet_sidecar_present: runtime_surface_symbol_value(
            runtime_surface,
            "pmetpara.mode.sidecar_present",
        ),
        pmet_iflget: runtime_surface_symbol_value(runtime_surface, "pmetpara.mode.iflget"),
        pmet_selected_kcb: runtime_surface_symbol_value(runtime_surface, "pmetpara.selected.kcb"),
        pmet_selected_rawp: runtime_surface_symbol_value(runtime_surface, "pmetpara.selected.rawp"),
        pmet_selected_line_index: runtime_surface_symbol_value(
            runtime_surface,
            "pmetpara.selected.line_index",
        ),
        pmet_lookup_fallback_first_row_used: runtime_surface_symbol_value(
            runtime_surface,
            "pmetpara.lookup.fallback_first_row_used",
        ),
        wb11_et_demand_m: runtime_surface_symbol_value(runtime_surface, "wb11_et_demand"),
        wb11_et_seed_branch: hphys0245_et_seed_branch(runtime_surface),
        pmet_etorc_mm: runtime_surface_symbol_value(runtime_surface, "pmet.etorc_mm"),
        pmet_rn_mj_m2: runtime_surface_symbol_value(runtime_surface, "pmet.rn_mj_m2"),
        pmet_fwv_m_s: runtime_surface_symbol_value(runtime_surface, "pmet.fwv_m_s"),
        pmet_rhd_pct: runtime_surface_symbol_value(runtime_surface, "pmet.rhd_pct"),
        pmet_kcbadj: runtime_surface_symbol_value(runtime_surface, "pmet.kcbadj"),
        pmet_kcbcon: runtime_surface_symbol_value(runtime_surface, "pmet.kcbcon"),
        pmet_etke: runtime_surface_symbol_value(runtime_surface, "pmet.etke"),
        pmet_etkr: runtime_surface_symbol_value(runtime_surface, "pmet.etkr"),
        pmet_etks: runtime_surface_symbol_value(runtime_surface, "pmet.etks"),
        pmet_tew_mm: runtime_surface_symbol_value(runtime_surface, "pmet.tew_mm"),
        pmet_rew_mm: runtime_surface_symbol_value(runtime_surface, "pmet.rew_mm"),
        pmet_wfevp_mm: runtime_surface_symbol_value(runtime_surface, "pmet.wfevp_mm"),
        pmet_taw_mm: runtime_surface_symbol_value(runtime_surface, "pmet.taw_mm"),
        pmet_raw_mm: runtime_surface_symbol_value(runtime_surface, "pmet.raw_mm"),
        pmet_wftrp_mm: runtime_surface_symbol_value(runtime_surface, "pmet.wftrp_mm"),
        pmet_es_m: runtime_surface_symbol_value(runtime_surface, "pmet.es_m"),
        pmet_ep_m: runtime_surface_symbol_value(runtime_surface, "pmet.ep_m"),
        etp_m: runtime_surface_symbol_value_prefer_flux(runtime_surface, "Etp"),
        upi_m: runtime_surface_symbol_value_prefer_flux(runtime_surface, "UPi"),
        ui_m: runtime_surface_symbol_value_prefer_flux(runtime_surface, "Ui"),
        wb18_ul_layers_m,
        wb17_swu_stress_threshold_layers_m,
        wb17_swu_storage_to_threshold_layers,
        wb17_upi_layers_m: potential_uptake_layers_m,
        wb17_ui_layers_m: actual_uptake_layers_m,
        ep_m: runtime_surface_symbol_value_prefer_flux(runtime_surface, "Ep"),
        ws: runtime_surface_symbol_value_prefer_flux(runtime_surface, "Ws"),
        wb19_q_lateral_potential_m: runtime_surface_symbol_value(
            runtime_surface,
            "wb19_q_lateral_potential",
        ),
        wb19_q_lateral_target_m: runtime_surface_symbol_value(
            runtime_surface,
            "wb19_q_lateral_target",
        ),
        wb19_lateral_capacity_tdv_m: runtime_surface_symbol_value(
            runtime_surface,
            "wb19_lateral_capacity_tdv",
        ),
        wb19_tdvv_m: runtime_surface_symbol_value(runtime_surface, "wb19_tdvv"),
        wb19_q_lateral_unrealized_m: runtime_surface_symbol_value(
            runtime_surface,
            "wb19_q_lateral_unrealized",
        ),
        wb19_lateral_withdrawal_layers_m,
        wb19_lateral_capacity_active_count_layers,
        wb19_lateral_conductivity_active_count_layers,
        q_m: runtime_surface_symbol_value_prefer_flux(runtime_surface, "q"),
        qdd_m: runtime_surface_symbol_value_prefer_flux(runtime_surface, "Qdd"),
        qd_m: runtime_surface_symbol_value_prefer_flux(runtime_surface, "Qd"),
    }
}

fn hphys0245_prefixed_surface_values(
    surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    prefix: &str,
) -> BTreeMap<String, f64> {
    surface
        .iter()
        .filter_map(|(symbol, value)| {
            let symbol = symbol.as_str();
            symbol
                .strip_prefix(prefix)
                .map(|suffix| (suffix.to_string(), value.as_f64()))
        })
        .collect()
}

fn hphys0245_prefixed_surface_values_with_fallback(
    surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    preferred_prefix: &str,
    fallback_prefix: &str,
) -> BTreeMap<String, f64> {
    let mut values = hphys0245_prefixed_surface_values(surface, fallback_prefix);
    values.extend(hphys0245_prefixed_surface_values(surface, preferred_prefix));
    values
}

fn hphys0245_prefixed_runtime_values(
    runtime_surface: &HillslopeWritebackSurface,
    prefix: &str,
) -> BTreeMap<String, f64> {
    let mut values = hphys0245_prefixed_surface_values(&runtime_surface.state_surface, prefix);
    values.extend(hphys0245_prefixed_surface_values(
        &runtime_surface.flux_surface,
        prefix,
    ));
    values
}

fn hphys0245_sum_runtime_prefix(runtime_surface: &HillslopeWritebackSurface, prefix: &str) -> f64 {
    hphys0245_prefixed_runtime_values(runtime_surface, prefix)
        .values()
        .copied()
        .sum()
}

fn hphys0245_sum_or_none(values: &BTreeMap<String, f64>) -> Option<f64> {
    if values.is_empty() {
        None
    } else {
        Some(values.values().copied().sum())
    }
}

fn hphys0245_swu_stress_threshold_layers(
    ul_layers: &BTreeMap<String, f64>,
    effective_pltol: Option<f64>,
) -> BTreeMap<String, f64> {
    let Some(effective_pltol) = effective_pltol else {
        return BTreeMap::new();
    };
    if !effective_pltol.is_finite() || effective_pltol < 0.0 {
        return BTreeMap::new();
    }
    ul_layers
        .iter()
        .filter_map(|(suffix, ul)| {
            if ul.is_finite() && *ul >= 0.0 {
                let threshold = effective_pltol * *ul;
                if threshold.is_finite() {
                    return Some((suffix.clone(), threshold));
                }
            }
            None
        })
        .collect()
}

fn hphys0245_swu_storage_to_threshold_layers(
    theta_layers: &BTreeMap<String, f64>,
    threshold_layers: &BTreeMap<String, f64>,
) -> BTreeMap<String, f64> {
    threshold_layers
        .iter()
        .filter_map(|(suffix, threshold)| {
            if !threshold.is_finite() || *threshold <= 0.0 {
                return None;
            }
            let theta = theta_layers.get(suffix)?;
            let ratio = *theta / *threshold;
            if ratio.is_finite() {
                Some((suffix.clone(), ratio))
            } else {
                None
            }
        })
        .collect()
}

fn hphys0245_wb19_drfc_layers(
    fc_layers: &BTreeMap<String, f64>,
    dg_layers: &BTreeMap<String, f64>,
    coca_layers: &BTreeMap<String, f64>,
) -> BTreeMap<String, f64> {
    fc_layers
        .iter()
        .filter_map(|(suffix, fc)| {
            let dg = dg_layers.get(suffix)?;
            let coca = coca_layers.get(suffix)?;
            let drfc = *fc + ((1.0 - *coca) * *dg);
            if fc.is_finite() && dg.is_finite() && coca.is_finite() && drfc.is_finite() {
                Some((suffix.clone(), drfc))
            } else {
                None
            }
        })
        .collect()
}

fn hphys0245_wb19_fzdrfc_layers(
    drfc_layers: &BTreeMap<String, f64>,
    frzw_layers: &BTreeMap<String, f64>,
) -> BTreeMap<String, f64> {
    drfc_layers
        .iter()
        .filter_map(|(suffix, drfc)| {
            let frzw = frzw_layers.get(suffix)?;
            let fzdrfc = (*drfc - frzw).max(0.0);
            if drfc.is_finite() && frzw.is_finite() && fzdrfc.is_finite() {
                Some((suffix.clone(), fzdrfc))
            } else {
                None
            }
        })
        .collect()
}

fn hphys0245_recompute_wb18_soil_water(
    theta_layers: &BTreeMap<String, f64>,
    thetdr_layers: &BTreeMap<String, f64>,
    dg_layers: &BTreeMap<String, f64>,
    frozen_depth_layers: &BTreeMap<String, f64>,
) -> Option<f64> {
    if theta_layers.is_empty() {
        return None;
    }
    let mut soil_water = 0.0;
    for (suffix, theta) in theta_layers {
        let thetdr = thetdr_layers.get(suffix)?;
        let dg = dg_layers.get(suffix)?;
        let frozen_depth = frozen_depth_layers.get(suffix).copied().unwrap_or(0.0);
        if !theta.is_finite()
            || !thetdr.is_finite()
            || !dg.is_finite()
            || !frozen_depth.is_finite()
            || *thetdr < 0.0
            || *dg <= 0.0
            || frozen_depth < 0.0
            || frozen_depth > *dg
        {
            return None;
        }
        let layer_soil_water = *theta + (*thetdr * (*dg - frozen_depth));
        if !layer_soil_water.is_finite() {
            return None;
        }
        soil_water += layer_soil_water;
    }
    if soil_water.is_finite() {
        Some(soil_water)
    } else {
        None
    }
}

fn format_wb12_storage_terms(runtime_surface: &HillslopeWritebackSurface) -> String {
    fn get(runtime_surface: &HillslopeWritebackSurface, symbol: &str) -> String {
        runtime_surface_symbol_value(runtime_surface, symbol)
            .map_or_else(|| "NA".to_string(), |value| format!("{value:.10}"))
    }

    let storage_initial = runtime_surface_symbol_value(runtime_surface, "wb12_storage_initial");
    let precip_input = runtime_surface_symbol_value(runtime_surface, "wb12_precip_input");
    let snow_coupling_s = runtime_surface_symbol_value(runtime_surface, "S");
    let irrigation_input = runtime_surface_symbol_value(runtime_surface, "Irr");
    let interception_i = runtime_surface_symbol_value(runtime_surface, "I");
    let q_runoff = runtime_surface_symbol_value(runtime_surface, "Q");
    let et = runtime_surface_symbol_value(runtime_surface, "ET");
    let percolation_loss = runtime_surface_symbol_value(runtime_surface, "D");
    let subsurface_loss = runtime_surface_symbol_value(runtime_surface, "Qd");
    let reconciled_est = match (
        storage_initial,
        precip_input,
        snow_coupling_s,
        irrigation_input,
        interception_i,
        q_runoff,
        et,
        percolation_loss,
        subsurface_loss,
    ) {
        (
            Some(storage_initial),
            Some(precip_input),
            Some(snow_coupling_s),
            Some(irrigation_input),
            Some(interception_i),
            Some(q_runoff),
            Some(et),
            Some(percolation_loss),
            Some(subsurface_loss),
        ) => format!(
            "{:.10}",
            storage_initial + precip_input + snow_coupling_s + irrigation_input
                - interception_i
                - q_runoff
                - et
                - percolation_loss
                - subsurface_loss
        ),
        _ => "NA".to_string(),
    };

    format!(
        "{{storage_initial={},precip_input={},S={},Irr={},I={},Q={},ET={},D={},Qd={},reconciled_est={}}}",
        get(runtime_surface, "wb12_storage_initial"),
        get(runtime_surface, "wb12_precip_input"),
        get(runtime_surface, "S"),
        get(runtime_surface, "Irr"),
        get(runtime_surface, "I"),
        get(runtime_surface, "Q"),
        get(runtime_surface, "ET"),
        get(runtime_surface, "D"),
        get(runtime_surface, "Qd"),
        reconciled_est
    )
}

#[allow(clippy::too_many_lines)]
fn format_wb18_perc_guard_terms(runtime_surface: &HillslopeWritebackSurface) -> String {
    let mut layer_suffixes = runtime_surface
        .state_surface
        .keys()
        .filter_map(|symbol| symbol.as_str().strip_prefix("wb18_perc_fc_"))
        .filter(|suffix| suffix.len() == 4 && suffix.chars().all(|ch| ch.is_ascii_digit()))
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    layer_suffixes.sort_unstable();
    layer_suffixes.dedup();

    if layer_suffixes.is_empty() {
        return "{layers=none}".to_string();
    }

    let fmt_opt = |value: Option<f64>| {
        value.map_or_else(|| "NA".to_string(), |observed| format!("{observed:.10}"))
    };
    let fmt_state = |symbol: &str| fmt_opt(runtime_surface_symbol_value(runtime_surface, symbol));

    let invalid_layers = layer_suffixes
        .iter()
        .filter_map(|suffix| {
            let fc =
                runtime_surface_symbol_value(runtime_surface, &format!("wb18_perc_fc_{suffix}"))?;
            let ul =
                runtime_surface_symbol_value(runtime_surface, &format!("wb18_perc_ul_{suffix}"))?;
            let theta =
                runtime_surface_symbol_value(runtime_surface, &format!("wb18_perc_theta_{suffix}"))?;
            let ssc =
                runtime_surface_symbol_value(runtime_surface, &format!("wb18_perc_ssc_{suffix}"));
            let thetfc = runtime_surface_symbol_value(runtime_surface, &format!("thetfc_{suffix}"));
            let thetdr = runtime_surface_symbol_value(runtime_surface, &format!("thetdr_{suffix}"));
            let dg = runtime_surface_symbol_value(runtime_surface, &format!("dg_{suffix}"));
            let por = runtime_surface_symbol_value(runtime_surface, &format!("por_{suffix}"));
            let cpm = runtime_surface_symbol_value(runtime_surface, &format!("cpm_{suffix}"));
            let frozen_depth = runtime_surface_symbol_value(
                runtime_surface,
                &format!("wb18_perc_frozen_depth_{suffix}"),
            );
            let ratio = fc / ul;
            let stz = theta / ul;
            let dynamic_branch_active = stz.is_finite() && stz < 0.95;
            let ratio_domain_invalid = !ratio.is_finite() || ratio >= 1.0;
            let legacy_bi_zero_candidate = ratio.is_finite() && ratio <= 0.0;

            let mut flags = Vec::new();
            if !ul.is_finite() || ul <= 0.0 {
                flags.push("ul_nonpositive");
            }
            if !theta.is_finite() || theta < 0.0 {
                flags.push("theta_invalid");
            }
            if ratio_domain_invalid {
                flags.push("fc_ul_ratio_invalid");
            }
            if legacy_bi_zero_candidate {
                flags.push("legacy_bi_zero_candidate");
            }
            if let Some(ssc_value) = ssc
                && (!ssc_value.is_finite() || ssc_value <= 0.0)
            {
                flags.push("ssc_nonpositive");
            }
            if let Some(thetdr_value) = thetdr
                && (!thetdr_value.is_finite() || !(0.0..=1.0).contains(&thetdr_value))
            {
                flags.push("thetdr_out_of_range");
            }
            if let Some(dg_value) = dg
                && (!dg_value.is_finite() || dg_value <= 0.0)
            {
                flags.push("dg_nonpositive");
            }
            if let (Some(frozen_depth_value), Some(dg_value)) = (frozen_depth, dg)
                && (!frozen_depth_value.is_finite()
                    || frozen_depth_value < 0.0
                    || frozen_depth_value > dg_value)
            {
                flags.push("frozen_depth_out_of_range");
            }

            let lower_ratio_summary = suffix
                .parse::<usize>()
                .ok()
                .and_then(|index| {
                    let lower_suffix = format!("{:04}", index + 1);
                    let lower_theta = runtime_surface_symbol_value(
                        runtime_surface,
                        &format!("wb18_perc_theta_{lower_suffix}"),
                    )?;
                    let lower_ul = runtime_surface_symbol_value(
                        runtime_surface,
                        &format!("wb18_perc_ul_{lower_suffix}"),
                    )?;
                    Some(lower_theta / lower_ul)
                })
                .filter(|ratio| !ratio.is_finite() || *ratio < 0.0)
                .map(|ratio| format!("lower_ratio={ratio:.10}"));
            if lower_ratio_summary.is_some() {
                flags.push("lower_ratio_invalid");
            }

            if flags.is_empty() {
                return None;
            }
            Some(format!(
                "L{}(flags={},fc={:.10},ul={:.10},theta={:.10},ratio={:.10},stz={:.10},dynamic_branch_active={},ssc={},thetfc={},thetdr={},dg={},frozen_depth={},por={},cpm={}{})",
                suffix,
                flags.join("+"),
                fc,
                ul,
                theta,
                ratio,
                stz,
                dynamic_branch_active,
                fmt_opt(ssc),
                fmt_opt(thetfc),
                fmt_opt(thetdr),
                fmt_opt(dg),
                fmt_opt(frozen_depth),
                fmt_opt(por),
                fmt_opt(cpm),
                lower_ratio_summary.map_or_else(String::new, |summary| format!(",{summary}")),
            ))
        })
        .collect::<Vec<_>>();

    let invalid_summary = if invalid_layers.is_empty() {
        "none".to_string()
    } else {
        invalid_layers.join("|")
    };

    format!(
        "{{layer_count={},lane_substeps={},infiltration={},tillay2={},slflag={},kslast={},ui_bdrkth={},invalid_layers={}}}",
        layer_suffixes.len(),
        fmt_state("wb18_perc_lane_substeps"),
        fmt_state("wb12_infiltration"),
        fmt_state("management.initial.params.tillay2_m"),
        fmt_state("slflag"),
        fmt_state("kslast"),
        fmt_state("ui_bdrkth"),
        invalid_summary
    )
}

#[allow(clippy::too_many_lines)]
fn build_simimpl10_coupling_vector_provenance(
    runtime_surface: &HillslopeWritebackSurface,
    wb13_row: &SimulationOwnedWb13Row,
) -> Result<HillslopeCouplingVectorProvenance, HillslopeCliError> {
    let snow_file_present = parse_simimpl10_binary_flag(
        "snow.options.snow_file_present",
        require_simimpl10_coupling_scalar(runtime_surface, "snow.options.snow_file_present")?,
    )?;
    let rst = require_simimpl10_coupling_scalar(runtime_surface, "snow.options.rst")?;
    let newsnw = require_simimpl10_coupling_scalar(runtime_surface, "snow.options.newsnw")?;
    let ssd = require_simimpl10_coupling_scalar(runtime_surface, "snow.options.ssd")?;
    let runtime_swe = wb13_row.wb13_row.snow_water / 1_000.0;

    if newsnw <= 0.0 {
        return Err(simcoup_failure(format!(
            "snow.options.newsnw must be > 0.0, observed {newsnw}"
        )));
    }
    if ssd <= 0.0 {
        return Err(simcoup_failure(format!(
            "snow.options.ssd must be > 0.0, observed {ssd}"
        )));
    }
    if newsnw > ssd {
        return Err(simcoup_failure(format!(
            "snow.options.newsnw must be <= snow.options.ssd, observed {newsnw} > {ssd}"
        )));
    }
    if runtime_swe < 0.0 {
        return Err(simcoup_failure(format!(
            "snow.runtime_swe must be >= 0.0, observed {runtime_swe}"
        )));
    }

    let frost_file_present = parse_simimpl10_binary_flag(
        "frost.options.frost_file_present",
        require_simimpl10_coupling_scalar(runtime_surface, "frost.options.frost_file_present")?,
    )?;
    let wint_red_enabled = parse_simimpl10_binary_flag(
        "frost.options.wintRed",
        require_simimpl10_coupling_scalar(runtime_surface, "frost.options.wintRed")?,
    )?;
    let dfrost = require_simimpl10_coupling_scalar(runtime_surface, "frost.runtime_dfrost")?;
    let dthaw = require_simimpl10_coupling_scalar(runtime_surface, "frost.runtime_dthaw")?;
    let nft = require_simimpl10_coupling_scalar(runtime_surface, "frost.runtime_nft")?;
    let ws_frz = require_simimpl10_coupling_scalar(runtime_surface, "frost.runtime_ws_frz")?;
    let infcap_frz =
        require_simimpl10_coupling_scalar(runtime_surface, "frost.runtime_infcap_frz")?;
    let ssc = require_simimpl10_coupling_scalar(runtime_surface, "ssc")?;
    let tmax = require_simimpl10_coupling_scalar(runtime_surface, "tmax")?;
    let tmin = require_simimpl10_coupling_scalar(runtime_surface, "tmin")?;
    let winter_active =
        runtime_swe > 0.0 || dfrost > 0.0 || ws_frz > 0.0 || f64::midpoint(tmax, tmin) < 0.0;

    let winter = HillslopeWinterCouplingProvenance {
        active: winter_active,
        snow_file_present,
        rst,
        newsnw,
        ssd,
        runtime_swe,
    };

    if !(0.0..=SIMIMPL10_FROST_MAX_DEPTH_M).contains(&dfrost) {
        return Err(simcoup_failure(format!(
            "frost.runtime_dfrost must be within [0.0,{SIMIMPL10_FROST_MAX_DEPTH_M}], observed {dfrost}"
        )));
    }
    if !(0.0..=SIMIMPL10_FROST_MAX_DEPTH_M).contains(&dthaw) {
        return Err(simcoup_failure(format!(
            "frost.runtime_dthaw must be within [0.0,{SIMIMPL10_FROST_MAX_DEPTH_M}], observed {dthaw}"
        )));
    }
    if nft < 0.0 {
        return Err(simcoup_failure(format!(
            "frost.runtime_nft must be >= 0.0, observed {nft}"
        )));
    }
    if ws_frz < 0.0 {
        return Err(simcoup_failure(format!(
            "frost.runtime_ws_frz must be >= 0.0, observed {ws_frz}"
        )));
    }
    if ssc < 0.0 {
        return Err(simcoup_failure(format!(
            "ssc must be >= 0.0 for frozen-soil coupling, observed {ssc}"
        )));
    }
    if infcap_frz < 0.0 || infcap_frz > ssc {
        return Err(simcoup_failure(format!(
            "frost.runtime_infcap_frz must be within [0.0,ssc], observed {infcap_frz} with ssc={ssc}"
        )));
    }

    let frsoil_active = wint_red_enabled;
    let frsoil = HillslopeFrozenSoilCouplingProvenance {
        active: frsoil_active,
        frost_file_present,
        wint_red_enabled,
        dfrost,
        dthaw,
        nft,
        ws_frz,
        infcap_frz,
    };
    let soil = HillslopeSoilCouplingProvenance {
        ssc,
        infiltration_capacity_frozen: infcap_frz,
        infcap_within_ssc: infcap_frz <= ssc,
    };

    let total_soil = wb13_row.wb13_row.total_soil;
    let frozwt = wb13_row.wb13_row.frozwt;
    let snow_water = wb13_row.wb13_row.snow_water;
    let soil_water_total = wb13_row.wb13_row.soil_water_total;
    let closure_delta = soil_water_total - (total_soil + frozwt);
    let closure_within_tolerance = closure_delta.abs() <= SIMIMPL10_SOIL_WATER_TOTAL_TOLERANCE_MM;
    if !closure_within_tolerance {
        return Err(simcoup_failure(format!(
            "hydout-equivalent closure violated: SoilWaterTotal - (Total-Soil + frozwt) = {closure_delta} exceeds tolerance {SIMIMPL10_SOIL_WATER_TOTAL_TOLERANCE_MM}",
        )));
    }

    let hydout_equivalent = HillslopeHydoutEquivalentCouplingProvenance {
        source: WB13_PUBLICATION_SOURCE_SIMULATION_OWNED.to_string(),
        total_soil,
        frozwt,
        snow_water,
        soil_water_total,
        closure_delta,
        closure_tolerance: SIMIMPL10_SOIL_WATER_TOTAL_TOLERANCE_MM,
        closure_within_tolerance,
    };

    Ok(HillslopeCouplingVectorProvenance {
        guard_id: SIMCOUP_GUARD_ID.to_string(),
        winter,
        soil,
        frsoil,
        hydout_equivalent,
    })
}

const MOFE04_PUBLICATION_OFE_POLICY: &str = "single-row-canonicalized-hillslope-aggregate";
const MOFE04_PUBLICATION_AREA_POLICY: &str = "sum-ofe-geometry-area";
const HPHYS0255_STORAGE_LINEAGE_POLICY: &str = "single-runtime-wb11-state";

fn build_wb13_publication_provenance(
    rows: &[SimulationOwnedWb13Row],
    contributor_ofe_count: usize,
    publication_area_m2: f64,
) -> Result<HillslopeWb13PublicationProvenance, HillslopeCliError> {
    let Some(first_row) = rows.first() else {
        return Err(wb13_simout_failure(
            "WB13 publication requires at least one executed-day row",
        ));
    };
    let Some(last_row) = rows.last() else {
        return Err(wb13_simout_failure(
            "WB13 publication requires at least one executed-day row",
        ));
    };
    if rows.iter().any(|row| row.sim_day_index <= 0) {
        return Err(wb13_simout_failure(
            "sim_day_index must be positive for every WB13 publication row",
        ));
    }
    if contributor_ofe_count == 0 {
        return Err(wb13_simout_failure(
            "contributor_ofe_count must be >= 1 for WB13 publication provenance",
        ));
    }
    if !publication_area_m2.is_finite() || publication_area_m2 <= 0.0 {
        return Err(wb13_simout_failure(format!(
            "publication_area_m2 must be finite and > 0.0, observed {publication_area_m2}"
        )));
    }
    if rows.iter().any(|row| row.wb13_row.ofe != 1) {
        return Err(wb13_simout_failure(
            "MOFE04 canonicalized publication policy requires WB13 OFE key = 1 for all rows",
        ));
    }
    let sim_day_index_monotonic = rows
        .windows(2)
        .all(|window| window[1].sim_day_index > window[0].sim_day_index);

    Ok(HillslopeWb13PublicationProvenance {
        source: WB13_PUBLICATION_SOURCE_SIMULATION_OWNED.to_string(),
        projection_fallback_used: false,
        guard_id: SIMOUT_GUARD_ID.to_string(),
        replay_candidate_surfaces: vec![
            WB13_REPLAY_CANDIDATE_SURFACE_WAT.to_string(),
            WB13_REPLAY_CANDIDATE_SURFACE_PASS.to_string(),
        ],
        publication_ofe_policy: MOFE04_PUBLICATION_OFE_POLICY.to_string(),
        contributor_ofe_count,
        area_policy: MOFE04_PUBLICATION_AREA_POLICY.to_string(),
        storage_lineage_policy: HPHYS0255_STORAGE_LINEAGE_POLICY.to_string(),
        publication_area_m2,
        row_count: rows.len(),
        sim_day_index_monotonic,
        first_row_key: wb13_row_key_provenance(first_row),
        last_row_key: wb13_row_key_provenance(last_row),
    })
}

fn build_mofe_hourly_carry_provenance(
    runtime_surface: &HillslopeWritebackSurface,
    contributor_ofe_count: usize,
) -> Result<HillslopeMofeHourlyCarryProvenance, HillslopeCliError> {
    let active = contributor_ofe_count > 1;
    let upstream_carry_total_m = if active {
        sum_mofe_hourly_carry_array(
            runtime_surface,
            MOFE_HOURLY_UPSTREAM_SATURATION_RUNOFF_ROOT,
            true,
        )? + sum_mofe_hourly_carry_array(
            runtime_surface,
            MOFE_HOURLY_UPSTREAM_LATERAL_RUNOFF_ROOT,
            true,
        )?
    } else {
        0.0
    };
    let current_carry_total_m = if active {
        sum_mofe_hourly_carry_array(
            runtime_surface,
            MOFE_HOURLY_CURRENT_SATURATION_RUNOFF_ROOT,
            true,
        )? + sum_mofe_hourly_carry_array(
            runtime_surface,
            MOFE_HOURLY_CURRENT_LATERAL_RUNOFF_ROOT,
            true,
        )?
    } else {
        0.0
    };

    Ok(HillslopeMofeHourlyCarryProvenance {
        policy: MOFE_HOURLY_CARRY_POLICY.to_string(),
        active,
        substep_count: MOFE_HOURLY_CARRY_ARRAY_COUNT,
        required_arrays: MOFE_HOURLY_REQUIRED_ARRAYS
            .iter()
            .map(|root| (*root).to_string())
            .collect(),
        upstream_carry_total_m,
        current_carry_total_m,
    })
}

fn sum_mofe_hourly_carry_array(
    runtime_surface: &HillslopeWritebackSurface,
    root: &str,
    required: bool,
) -> Result<f64, HillslopeCliError> {
    let mut total = 0.0_f64;
    for hour in 1..=MOFE_HOURLY_CARRY_ARRAY_COUNT {
        let symbol = mofe_hourly_carry_hour_symbol(root, hour);
        let Some(value) = runtime_surface_symbol_value(runtime_surface, &symbol) else {
            if required {
                return Err(mofe_hourly_carry_failure(format!(
                    "missing required runtime symbol {symbol}"
                )));
            }
            continue;
        };
        require_mofe_hourly_carry_non_negative(value, &symbol)?;
        total += value;
    }
    require_mofe_hourly_carry_non_negative(total, root)?;
    Ok(total)
}

fn scheduler_outcome_class_as_str(outcome_class: SchedulerOutcomeClass) -> &'static str {
    match outcome_class {
        SchedulerOutcomeClass::Completed => "completed",
        SchedulerOutcomeClass::TopologyPreconditionFailed => "topology_precondition_failed",
        SchedulerOutcomeClass::PhaseFailure => "phase_failure",
        SchedulerOutcomeClass::SchedulerInvariantFailure => "scheduler_invariant_failure",
    }
}

fn wb13_row_key_provenance(row: &SimulationOwnedWb13Row) -> HillslopeWb13RowKeyProvenance {
    HillslopeWb13RowKeyProvenance {
        year: row.wb13_row.year,
        julian_day: row.wb13_row.julian_day,
        ofe: row.wb13_row.ofe,
        sim_day_index: row.sim_day_index,
    }
}

const HBP_MAGIC: &[u8; 8] = b"WFPHBP01";
const HBP_FOOTER_MAGIC: &[u8; 8] = b"ENDHBP01";
const HBP_SUPPORTED_MAJOR_V1: u16 = 1;
const HBP_DIM_SCALAR: u8 = 0;
const HBP_DIM_NOFE: u8 = 1;
const HBP_DIM_NOFE_LAYERS: u8 = 2;
const HBP_DEFAULT_CALENDAR_YEAR: i32 = 2004;
const HBP_DEFAULT_PARTICLE_DIAMETER_M: f64 = 0.001;
const HBP_SCALE_INV_I64: f64 = 1.0e9;
const HBP_I64_MIN_F64: f64 = -9_223_372_036_854_775_808.0;
const HBP_I64_MAX_F64: f64 = 9_223_372_036_854_775_807.0;
const HBP_REQUIRED_STATE_IDS: &[u16] = &[
    1, 2, 3, 4, 5, 6, 7, 100, 101, 102, 103, 104, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209,
    210, 300, 900, 901,
];
