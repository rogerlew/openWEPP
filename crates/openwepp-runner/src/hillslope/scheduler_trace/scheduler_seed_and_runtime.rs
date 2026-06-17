const WB11_STATE_SEED_COMPLETED_SYMBOL: &str = "wb11_state_seed_completed";
const WB18_PERC_LANE_SUBSTEPS_SYMBOL: &str = "wb18_perc_lane_substeps";
const WB19_LATERAL_DRAIN_LANE_SUBSTEPS_SYMBOL: &str =
    "wb19_lateral_drain_lane_substeps";

pub(super) fn seed_wb11_runtime_surface_inputs(
    runtime_surface: &mut HillslopeWritebackSurface,
    execution_lane: ExecutionLane,
) -> Result<(), HillslopeCliError> {
    let nsl = resolve_wb11_seed_nsl(runtime_surface)?;
    let mofe_hourly_carry_active =
        seed_wb11_lane_substep_controls(runtime_surface, execution_lane)?;
    let prcp = require_nonnegative_wb11_prcp(runtime_surface)?;
    let rainfall_input = seed_wb11_hyetograph_inputs(runtime_surface, prcp)?;
    seed_initial_wb11_storage_if_needed(runtime_surface, nsl, execution_lane)?;
    refresh_wb18_frozen_depth_from_fine_frost_state(runtime_surface, nsl)?;
    let wb11_soil_water = require_nonnegative_wb11_soil_water_for_reconciliation(runtime_surface)?;
    seed_wb11_optional_default_symbols(runtime_surface);
    validate_wb19_lateral_and_drainage_inputs(runtime_surface)?;
    seed_wb12_reconciliation_runtime_inputs(
        runtime_surface,
        rainfall_input,
        prcp,
        wb11_soil_water,
        mofe_hourly_carry_active,
    );

    let wb11_et_seed = crate::hillslope::intake_lane_setup::compute_wb11_et_demand_seed(runtime_surface)?;
    crate::hillslope::intake_lane_setup::publish_wb11_et_demand_seed(runtime_surface, wb11_et_seed)?;

    seed_wb11_efflen_if_missing(runtime_surface)?;
    if runtime_surface_symbol_value(runtime_surface, "m").is_none() {
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("m"), BoundaryValue::scalar(1.5));
    }
    seed_wb16_ealpha_compatibility(runtime_surface)?;
    seed_mofe03_wave2_runtime_surface_inputs(runtime_surface)?;

    Ok(())
}

fn resolve_wb11_seed_nsl(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<usize, HillslopeCliError> {
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
    Ok(nsl)
}

fn seed_wb11_lane_substep_controls(
    runtime_surface: &mut HillslopeWritebackSurface,
    execution_lane: ExecutionLane,
) -> Result<bool, HillslopeCliError> {
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
    Ok(mofe_hourly_carry_active)
}

fn require_nonnegative_wb11_prcp(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<f64, HillslopeCliError> {
    let prcp = require_runtime_surface_scalar(runtime_surface, "prcp")?;
    if prcp < 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!("{SIMPIPE_GUARD_ID} prcp must be >= 0.0, observed {prcp}"),
        });
    }
    Ok(prcp)
}

fn seed_wb11_hyetograph_inputs(
    runtime_surface: &mut HillslopeWritebackSurface,
    prcp: f64,
) -> Result<f64, HillslopeCliError> {
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
        synthesize_zero_point_wb11_hyetograph(runtime_surface, prcp);
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
    let hyetograph_rainfall = accumulate_wb11_hyetograph_rainfall(runtime_surface, ninten)?;
    Ok(hyetograph_rainfall.max(prcp))
}

fn synthesize_zero_point_wb11_hyetograph(
    runtime_surface: &mut HillslopeWritebackSurface,
    prcp: f64,
) {
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
}

fn accumulate_wb11_hyetograph_rainfall(
    runtime_surface: &HillslopeWritebackSurface,
    ninten: usize,
) -> Result<f64, HillslopeCliError> {
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
    Ok(hyetograph_rainfall)
}

#[derive(Clone, Copy)]
struct Wb11LayerSeedInputs {
    dg: f64,
    thetfc: f64,
    thetdr: f64,
    ssc: f64,
    por: f64,
    cpm: f64,
}

#[derive(Clone, Copy)]
struct Wb11LayerSeedStores {
    theta: f64,
    field_capacity: f64,
    upper_limit: f64,
    soil_water: f64,
}

struct Wb11LayerSeedSymbols {
    dg: String,
    fc: String,
    wp: String,
    ssc: String,
    por: String,
    cpm: String,
}

#[derive(Clone, Copy, Default)]
struct Wb11InitialSeedTotals {
    soil_water: f64,
    field_capacity: f64,
    drainable_storage: f64,
    drainage_coefficient: f64,
}

fn seed_initial_wb11_storage_if_needed(
    runtime_surface: &mut HillslopeWritebackSurface,
    nsl: usize,
    execution_lane: ExecutionLane,
) -> Result<(), HillslopeCliError> {
    if wb11_runtime_state_is_seeded(runtime_surface) {
        return Ok(());
    }

    let mut totals = Wb11InitialSeedTotals::default();
    let mut sat = initial_wb11_saturation(runtime_surface, execution_lane)?;
    for layer_index in 1..=nsl {
        let layer = require_wb11_layer_seed_inputs(runtime_surface, layer_index)?;
        sat = apply_wb11_layer_saturation_floor(layer_index, sat, layer)?;
        let stores = derive_wb11_layer_seed_stores(layer_index, sat, layer)?;

        totals.soil_water += stores.soil_water;
        totals.field_capacity += stores.field_capacity;
        totals.drainable_storage += (stores.theta - stores.field_capacity).max(0.0);
        totals.drainage_coefficient += layer.ssc * 86_400.0;

        publish_wb11_layer_seed_stores(runtime_surface, layer_index, layer, stores);
    }
    publish_wb11_initial_seed_totals(runtime_surface, sat, totals);
    Ok(())
}

fn wb11_runtime_state_is_seeded(runtime_surface: &HillslopeWritebackSurface) -> bool {
    runtime_surface
        .state_surface
        .get(&BoundarySymbol::from(WB11_STATE_SEED_COMPLETED_SYMBOL))
        .copied()
        .map(BoundaryValue::as_f64)
        .is_some_and(|value| value >= 0.5)
        || runtime_surface_symbol_value(runtime_surface, "wb18_perc_theta_0001").is_some()
}

fn initial_wb11_saturation(
    runtime_surface: &HillslopeWritebackSurface,
    execution_lane: ExecutionLane,
) -> Result<f64, HillslopeCliError> {
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
    Ok(sat)
}

fn require_wb11_layer_seed_inputs(
    runtime_surface: &HillslopeWritebackSurface,
    layer_index: usize,
) -> Result<Wb11LayerSeedInputs, HillslopeCliError> {
    let symbols = Wb11LayerSeedSymbols {
        dg: format!("wb19_dg_{layer_index:04}"),
        fc: format!("wb19_thetfc_{layer_index:04}"),
        wp: format!("wb19_thetdr_{layer_index:04}"),
        ssc: wb13_primary_layer_symbol("ssc", layer_index),
        por: format!("wb19_por_{layer_index:04}"),
        cpm: wb13_primary_layer_symbol("cpm", layer_index),
    };

    let dg = require_runtime_surface_scalar(runtime_surface, symbols.dg.as_str())?;
    let thetfc = require_runtime_surface_scalar(runtime_surface, symbols.fc.as_str())?;
    let thetdr = require_runtime_surface_scalar(runtime_surface, symbols.wp.as_str())?;
    let ssc = require_runtime_surface_scalar(runtime_surface, symbols.ssc.as_str())?;
    let por = require_runtime_surface_scalar(runtime_surface, symbols.por.as_str())?;
    let cpm = require_runtime_surface_scalar(runtime_surface, symbols.cpm.as_str())?;

    validate_wb11_layer_seed_inputs(
        &symbols,
        Wb11LayerSeedInputs {
            dg,
            thetfc,
            thetdr,
            ssc,
            por,
            cpm,
        },
    )
}

fn validate_wb11_layer_seed_inputs(
    symbols: &Wb11LayerSeedSymbols,
    layer: Wb11LayerSeedInputs,
) -> Result<Wb11LayerSeedInputs, HillslopeCliError> {
    validate_wb11_layer_storage_geometry(symbols, layer)?;
    validate_wb11_layer_storage_order(symbols, layer)?;
    validate_wb11_layer_transport_scalars(symbols, layer)?;
    Ok(layer)
}

fn validate_wb11_layer_storage_geometry(
    symbols: &Wb11LayerSeedSymbols,
    layer: Wb11LayerSeedInputs,
) -> Result<(), HillslopeCliError> {
    if layer.dg <= 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!(
                "{SIMPIPE_GUARD_ID} {} must be > 0.0, observed {}",
                symbols.dg,
                layer.dg
            ),
        });
    }
    if layer.thetfc < 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!(
                "{SIMPIPE_GUARD_ID} {} must be >= 0.0, observed {}",
                symbols.fc,
                layer.thetfc
            ),
        });
    }
    if layer.thetdr < 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!(
                "{SIMPIPE_GUARD_ID} {} must be >= 0.0, observed {}",
                symbols.wp,
                layer.thetdr
            ),
        });
    }
    Ok(())
}

fn validate_wb11_layer_storage_order(
    symbols: &Wb11LayerSeedSymbols,
    layer: Wb11LayerSeedInputs,
) -> Result<(), HillslopeCliError> {
    if layer.thetdr > layer.thetfc {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!(
                "{SIMPIPE_GUARD_ID} {} must be <= {} (observed {} > {})",
                symbols.wp, symbols.fc, layer.thetdr, layer.thetfc
            ),
        });
    }
    Ok(())
}

fn validate_wb11_layer_transport_scalars(
    symbols: &Wb11LayerSeedSymbols,
    layer: Wb11LayerSeedInputs,
) -> Result<(), HillslopeCliError> {
    if layer.ssc <= 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!(
                "{SIMPIPE_GUARD_ID} {} must be > 0.0, observed {}",
                symbols.ssc,
                layer.ssc
            ),
        });
    }
    if layer.por <= 0.0 || layer.por > 1.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!(
                "{SIMPIPE_GUARD_ID} {} must be within (0,1], observed {}",
                symbols.por,
                layer.por
            ),
        });
    }
    if layer.cpm <= 0.0 || layer.cpm > 1.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!(
                "{SIMPIPE_GUARD_ID} {} must be within (0,1], observed {}",
                symbols.cpm,
                layer.cpm
            ),
        });
    }
    if layer.thetdr > layer.por {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!(
                "{SIMPIPE_GUARD_ID} {} must be <= {} (observed {} > {})",
                symbols.wp, symbols.por, layer.thetdr, layer.por
            ),
        });
    }
    Ok(())
}

fn apply_wb11_layer_saturation_floor(
    layer_index: usize,
    mut sat: f64,
    layer: Wb11LayerSeedInputs,
) -> Result<f64, HillslopeCliError> {
    let saturation_capacity = layer.por * layer.cpm;
    if !saturation_capacity.is_finite() || saturation_capacity <= 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!(
                "{SIMPIPE_GUARD_ID} por*cpm must be finite and > 0.0, observed {saturation_capacity}"
            ),
        });
    }
    let sat_floor = layer.thetdr / saturation_capacity;
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
    Ok(sat)
}

fn derive_wb11_layer_seed_stores(
    layer_index: usize,
    sat: f64,
    layer: Wb11LayerSeedInputs,
) -> Result<Wb11LayerSeedStores, HillslopeCliError> {
    let fc_store = (layer.thetfc - layer.thetdr) * layer.dg;
    if !fc_store.is_finite() || fc_store < 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!(
                "{SIMPIPE_GUARD_ID} derived wb18_perc_fc_{layer_index:04} must be finite and >= 0.0, observed {fc_store}"
            ),
        });
    }

    let ul_store = (layer.por - layer.thetdr) * layer.dg;
    if ul_store <= 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!(
                "{SIMPIPE_GUARD_ID} derived WB18 upper-limit store must be > 0.0 for layer {layer_index}"
            ),
        });
    }

    let saturation_theta = (sat * layer.por) * layer.cpm;
    let mut theta_store = (saturation_theta - layer.thetdr) * layer.dg;
    if !theta_store.is_finite() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!(
                "{SIMPIPE_GUARD_ID} derived wb18_perc_theta_{layer_index:04} is non-finite ({theta_store})"
            ),
        });
    }
    if theta_store < -1.0e-10 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!(
                "{SIMPIPE_GUARD_ID} derived wb18_perc_theta_{layer_index:04} must be >= 0.0, observed {theta_store}"
            ),
        });
    }
    if theta_store < 1.0e-10 {
        theta_store = 0.0;
    }

    let soilw_store = theta_store + (layer.thetdr * layer.dg);
    if !soilw_store.is_finite() || soilw_store < 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!(
                "{SIMPIPE_GUARD_ID} derived layer soil-water store must be finite and >= 0.0 for layer {layer_index}, observed {soilw_store}"
            ),
        });
    }

    Ok(Wb11LayerSeedStores {
        theta: theta_store,
        field_capacity: fc_store,
        upper_limit: ul_store,
        soil_water: soilw_store,
    })
}

fn publish_wb11_layer_seed_stores(
    runtime_surface: &mut HillslopeWritebackSurface,
    layer_index: usize,
    layer: Wb11LayerSeedInputs,
    stores: Wb11LayerSeedStores,
) {
    runtime_surface.state_surface.insert(
        BoundarySymbol::from(format!("wb18_perc_theta_{layer_index:04}")),
        BoundaryValue::scalar(stores.theta),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from(format!("wb18_perc_fc_{layer_index:04}")),
        BoundaryValue::scalar(stores.field_capacity),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from(format!("wb18_perc_ul_{layer_index:04}")),
        BoundaryValue::scalar(stores.upper_limit),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from(format!("wb18_perc_ssc_{layer_index:04}")),
        BoundaryValue::scalar(layer.ssc),
    );
}

fn publish_wb11_initial_seed_totals(
    runtime_surface: &mut HillslopeWritebackSurface,
    sat: f64,
    totals: Wb11InitialSeedTotals,
) {
    runtime_surface
        .state_surface
        .insert(BoundarySymbol::from("sat"), BoundaryValue::scalar(sat));
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(totals.soil_water),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb11_field_capacity"),
        BoundaryValue::scalar(totals.field_capacity),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb11_perc_fraction"),
        BoundaryValue::scalar(0.5),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb11_drainage_coefficient"),
        BoundaryValue::scalar(totals.drainage_coefficient.max(1.0e-6)),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb11_drainable_storage"),
        BoundaryValue::scalar(totals.drainable_storage),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from(WB11_STATE_SEED_COMPLETED_SYMBOL),
        BoundaryValue::scalar(1.0),
    );
}

fn require_nonnegative_wb11_soil_water_for_reconciliation(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<f64, HillslopeCliError> {
    let wb11_soil_water = require_runtime_surface_scalar(runtime_surface, "wb11_soil_water")?;
    if wb11_soil_water < 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!(
                "{SIMPIPE_GUARD_ID} wb11_soil_water must be >= 0.0 before daily reconciliation seeding, observed {wb11_soil_water}"
            ),
        });
    }
    Ok(wb11_soil_water)
}

fn seed_wb11_optional_default_symbols(runtime_surface: &mut HillslopeWritebackSurface) {
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
}

fn validate_wb19_lateral_and_drainage_inputs(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<(), HillslopeCliError> {
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
    if resolve_wb19_drain_enabled_flag(wb19_drain_enabled)? {
        validate_wb19_drain_geometry(runtime_surface)?;
    }
    Ok(())
}

fn resolve_wb19_drain_enabled_flag(
    wb19_drain_enabled: f64,
) -> Result<bool, HillslopeCliError> {
    if wb19_drain_enabled.abs() <= 1.0e-12 {
        Ok(false)
    } else if (wb19_drain_enabled - 1.0).abs() <= 1.0e-12 {
        Ok(true)
    } else {
        Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!(
                "{SIMPIPE_GUARD_ID} wb19_drain_enabled must be 0 or 1, observed {wb19_drain_enabled}"
            ),
        })
    }
}

fn validate_wb19_drain_geometry(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<(), HillslopeCliError> {
    require_positive_wb19_drain_geometry_scalar(runtime_surface, "wb19_drain_depth")?;
    require_positive_wb19_drain_geometry_scalar(runtime_surface, "wb19_drain_spacing")?;
    require_positive_wb19_drain_geometry_scalar(runtime_surface, "wb19_drain_diameter")?;
    Ok(())
}

fn require_positive_wb19_drain_geometry_scalar(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &str,
) -> Result<(), HillslopeCliError> {
    let value = require_runtime_surface_scalar(runtime_surface, symbol)?;
    if value <= 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!(
                "{SIMPIPE_GUARD_ID} {symbol} must be > 0.0 when wb19_drain_enabled=1, observed {value}"
            ),
        });
    }
    Ok(())
}

fn seed_wb12_reconciliation_runtime_inputs(
    runtime_surface: &mut HillslopeWritebackSurface,
    rainfall_input: f64,
    prcp: f64,
    wb11_soil_water: f64,
    mofe_hourly_carry_active: bool,
) {
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb12_rainfall_input"),
        BoundaryValue::scalar(rainfall_input),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb12_runon_input"),
        BoundaryValue::scalar(0.0),
    );
    let carryover_symbol = BoundarySymbol::from("wb12_runoff_carryover");
    if mofe_hourly_carry_active {
        runtime_surface.flux_surface.remove(&carryover_symbol);
    } else {
        runtime_surface
            .flux_surface
            .insert(carryover_symbol, BoundaryValue::scalar(0.0));
    }
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
}

fn seed_wb11_efflen_if_missing(
    runtime_surface: &mut HillslopeWritebackSurface,
) -> Result<(), HillslopeCliError> {
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
    Ok(())
}

fn seed_wb16_ealpha_compatibility(
    runtime_surface: &mut HillslopeWritebackSurface,
) -> Result<(), HillslopeCliError> {
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
    Ok(())
}

fn refresh_wb18_frozen_depth_from_fine_frost_state(
    runtime_surface: &mut HillslopeWritebackSurface,
    nsl: usize,
) -> Result<(), HillslopeCliError> {
    const FINE_COUNT_ROOT: &str = "frost.runtime_nfine";
    const FINE_FROZEN_DEPTH_ROOT: &str = "frost.runtime_slfsd_m";
    const ZERO_THRESHOLD: f64 = 1.0e-10;

    let scalar_frost_depth_m =
        runtime_surface_symbol_value(runtime_surface, "frost.runtime_frdp_m")
            .or_else(|| runtime_surface_symbol_value(runtime_surface, "frost.runtime_dfrost"));
    let mut cumulative_depth_m = 0.0_f64;
    for layer_index in 1..=nsl {
        let dg_symbol = format!("wb19_dg_{layer_index:04}");
        let dg_legacy_symbol = wb13_primary_layer_symbol("dg", layer_index);
        let dg = runtime_surface_symbol_value(runtime_surface, dg_symbol.as_str())
            .or_else(|| runtime_surface_symbol_value(runtime_surface, dg_legacy_symbol.as_str()))
            .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb11_seed",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} missing required layer depth {dg_symbol}/{dg_legacy_symbol} for fine frost aggregate refresh"
                ),
            })?;
        if !dg.is_finite() || dg <= ZERO_THRESHOLD {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb11_seed",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} layer depth for fine frost aggregate refresh must be finite and > 0.0, observed {dg}"
                ),
            });
        }

        let fine_count_symbol = wb13_primary_layer_symbol(FINE_COUNT_ROOT, layer_index);
        let frozen_depth_m = if let Some(fine_count_raw) =
            runtime_surface_symbol_value(runtime_surface, fine_count_symbol.as_str())
        {
            let fine_count = scalar_to_usize(fine_count_symbol.as_str(), fine_count_raw)?;
            if fine_count == 0 {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} {fine_count_symbol} must be >= 1 when fine frost state is present"
                    ),
                });
            }
            let mut fine_frozen_depth_m = 0.0_f64;
            for fine_index in 1..=fine_count {
                let slfsd_symbol =
                    format!("{FINE_FROZEN_DEPTH_ROOT}_{layer_index:04}_{fine_index:04}");
                let slfsd_m =
                    require_runtime_surface_scalar(runtime_surface, slfsd_symbol.as_str())?;
                if !slfsd_m.is_finite() || slfsd_m < -ZERO_THRESHOLD {
                    return Err(HillslopeCliError::RuntimeSurfaceFailure {
                        surface: "wb11_seed",
                        detail: format!(
                            "{SIMPIPE_GUARD_ID} {slfsd_symbol} must be finite and >= 0.0, observed {slfsd_m}"
                        ),
                    });
                }
                fine_frozen_depth_m += slfsd_m.max(0.0);
            }
            if fine_frozen_depth_m > dg + ZERO_THRESHOLD {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} fine frost depth sum for layer {layer_index} exceeds layer depth ({fine_frozen_depth_m} > {dg})"
                    ),
                });
            }
            fine_frozen_depth_m.min(dg)
        } else if let Some(scalar_frost_depth_m) = scalar_frost_depth_m {
            if !scalar_frost_depth_m.is_finite() || scalar_frost_depth_m < -ZERO_THRESHOLD {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} scalar frost depth must be finite and >= 0.0 for aggregate refresh, observed {scalar_frost_depth_m}"
                    ),
                });
            }
            (scalar_frost_depth_m - cumulative_depth_m).clamp(0.0, dg)
        } else {
            0.0
        };

        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("wb18_perc_frozen_depth_{layer_index:04}")),
            BoundaryValue::scalar(frozen_depth_m),
        );
        cumulative_depth_m += dg;
    }

    Ok(())
}

pub(super) const WB16_ACCGAV_M_S2: f64 = 9.807;
pub(super) const WB16_INRFSO_CROPLAND: f64 = 4.07;
pub(super) const WB16_FRCSOL_CROPLAND: f64 = 1.11;
pub(super) const WB16_RRINIT_MIN_M: f64 = 0.006;
pub(super) const WB16_RSPACE_DEFAULT_M: f64 = 1.0;
pub(super) const WB16_TEMPORARY_WIDTH_DEFAULT_M: f64 = 0.15;
pub(super) const WB16_COVER_CAP: f64 = 0.999;

#[derive(Clone, Copy)]
struct Wb16EalphaPowers {
    m: f64,
    power2: f64,
    power3: f64,
}

#[derive(Clone, Copy)]
struct Wb16OfeGeometry {
    avgslp: f64,
    slplen: f64,
}

#[derive(Clone, Copy)]
struct Wb16OfeSurfaceControls {
    inrcov: f64,
    rilcov: f64,
    rrinit: f64,
    rspace: f64,
    width: f64,
    rtyp: f64,
    cancov: f64,
}

#[derive(Clone, Copy)]
struct Wb16OfeCanopyControls {
    bb: f64,
    bbb: f64,
    flivmx: f64,
    hmax: f64,
}

#[derive(Clone, Copy)]
struct Wb16OfeNormalizedControls {
    inrcov: f64,
    rilcov: f64,
    rrinit: f64,
    rspace: f64,
    width: f64,
    rrc: f64,
    canhgt: f64,
    flivmx: f64,
    hmax: f64,
}

#[derive(Clone, Copy)]
struct Wb16OfeAlphaResult {
    alpha: f64,
    slplen: f64,
}

pub(super) fn produce_wb16_ealpha_from_runtime_surface(
    runtime_surface: &mut HillslopeWritebackSurface,
) -> Result<Option<f64>, HillslopeCliError> {
    let Some(ofe_count) = wb16_ealpha_ofe_count(runtime_surface)? else {
        return Ok(None);
    };
    let powers = wb16_ealpha_powers(runtime_surface)?;
    let mut ofe_results = Vec::with_capacity(ofe_count);

    for ofe_index in 1..=ofe_count {
        let Some(ofe_result) = wb16_produce_ofe_alpha(runtime_surface, ofe_index)? else {
            return Ok(None);
        };
        ofe_results.push(ofe_result);
    }

    let ealpha = wb16_equivalent_plane_alpha(&ofe_results, powers)?;
    wb16_publish_ealpha(runtime_surface, ealpha);
    Ok(Some(ealpha))
}

fn wb16_ealpha_ofe_count(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<Option<usize>, HillslopeCliError> {
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
    Ok(Some(ofe_count))
}

fn wb16_ealpha_powers(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<Wb16EalphaPowers, HillslopeCliError> {
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
    Ok(Wb16EalphaPowers { m, power2, power3 })
}

fn wb16_produce_ofe_alpha(
    runtime_surface: &mut HillslopeWritebackSurface,
    ofe_index: usize,
) -> Result<Option<Wb16OfeAlphaResult>, HillslopeCliError> {
    let Some(geometry) = wb16_ofe_geometry(runtime_surface, ofe_index)? else {
        return Ok(None);
    };
    let Some(surface_controls) = wb16_ofe_surface_controls(runtime_surface, ofe_index) else {
        return Ok(None);
    };
    let Some(canopy_controls) = wb16_ofe_canopy_controls(runtime_surface, ofe_index) else {
        return Ok(None);
    };

    wb16_validate_finite_ofe_values(ofe_index, surface_controls, canopy_controls)?;
    wb16_validate_surface_nonnegative(ofe_index, surface_controls)?;
    wb16_validate_canopy_nonnegative(ofe_index, surface_controls, canopy_controls)?;
    let controls =
        wb16_normalize_ofe_controls(runtime_surface, ofe_index, surface_controls, canopy_controls)?;
    let frlive = wb16_compute_frlive(ofe_index, controls)?;
    let frcteq = wb16_compute_frcteq(ofe_index, controls, frlive)?;
    wb16_publish_ofe_frcteq(runtime_surface, ofe_index, frcteq);
    let alpha = wb16_compute_ofe_alpha(ofe_index, geometry.avgslp, frcteq)?;
    wb16_publish_ofe_alpha(runtime_surface, ofe_index, alpha);
    Ok(Some(Wb16OfeAlphaResult {
        alpha,
        slplen: geometry.slplen,
    }))
}

fn wb16_ofe_geometry(
    runtime_surface: &HillslopeWritebackSurface,
    ofe_index: usize,
) -> Result<Option<Wb16OfeGeometry>, HillslopeCliError> {
    let Some(avgslp) = wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "avgslp") else {
        return Ok(None);
    };
    let Some(slplen) = wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "slplen") else {
        return Ok(None);
    };
    if !avgslp.is_finite() || avgslp <= 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb16_ealpha_producer",
            detail: format!(
                "{SIMPIPE_GUARD_ID} ofe{ofe_index}_avgslp must be finite and > 0, observed {avgslp}"
            ),
        });
    }
    if !slplen.is_finite() || slplen <= 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb16_ealpha_producer",
            detail: format!(
                "{SIMPIPE_GUARD_ID} ofe{ofe_index}_slplen must be finite and > 0, observed {slplen}"
            ),
        });
    }
    Ok(Some(Wb16OfeGeometry { avgslp, slplen }))
}

fn wb16_ofe_surface_controls(
    runtime_surface: &HillslopeWritebackSurface,
    ofe_index: usize,
) -> Option<Wb16OfeSurfaceControls> {
    Some(Wb16OfeSurfaceControls {
        inrcov: wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "inrcov")?,
        rilcov: wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "rilcov")?,
        rrinit: wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "rrinit")?,
        rspace: wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "rspace")?,
        width: wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "width")?,
        rtyp: wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "rtyp")?,
        cancov: wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "cancov")
            .or_else(|| wb16_optional_state_scalar(runtime_surface, "cancov"))?,
    })
}

fn wb16_ofe_canopy_controls(
    runtime_surface: &HillslopeWritebackSurface,
    ofe_index: usize,
) -> Option<Wb16OfeCanopyControls> {
    Some(Wb16OfeCanopyControls {
        bb: wb16_optional_state_scalar(
            runtime_surface,
            &format!("pl_growth_ofe{ofe_index}_bb_seed"),
        )
        .or_else(|| wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "bb"))
        .or_else(|| wb16_optional_state_scalar(runtime_surface, "bb"))?,
        bbb: wb16_optional_state_scalar(
            runtime_surface,
            &format!("pl_growth_ofe{ofe_index}_bbb_seed"),
        )
        .or_else(|| wb16_optional_state_scalar(runtime_surface, "bbb_seed"))
        .or_else(|| wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "bbb"))
        .or_else(|| wb16_optional_state_scalar(runtime_surface, "bbb"))?,
        flivmx: wb16_optional_state_scalar(
            runtime_surface,
            &format!("pl_growth_ofe{ofe_index}_flivmx_seed"),
        )
        .or_else(|| wb16_optional_state_scalar(runtime_surface, "flivmx_seed"))
        .or_else(|| wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "flivmx"))
        .or_else(|| wb16_optional_state_scalar(runtime_surface, "flivmx"))?,
        hmax: wb16_optional_state_scalar(
            runtime_surface,
            &format!("pl_growth_ofe{ofe_index}_hmax_seed"),
        )
        .or_else(|| wb16_optional_state_scalar(runtime_surface, "hmax_seed"))
        .or_else(|| wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "hmax"))
        .or_else(|| wb16_optional_state_scalar(runtime_surface, "hmax"))?,
    })
}

fn wb16_validate_finite_ofe_values(
    ofe_index: usize,
    surface_controls: Wb16OfeSurfaceControls,
    canopy_controls: Wb16OfeCanopyControls,
) -> Result<(), HillslopeCliError> {
    for (symbol, value) in [
        ("inrcov", surface_controls.inrcov),
        ("rilcov", surface_controls.rilcov),
        ("rrinit", surface_controls.rrinit),
        ("rspace", surface_controls.rspace),
        ("width", surface_controls.width),
        ("rtyp", surface_controls.rtyp),
        ("cancov", surface_controls.cancov),
        ("bb", canopy_controls.bb),
        ("bbb", canopy_controls.bbb),
        ("flivmx", canopy_controls.flivmx),
        ("hmax", canopy_controls.hmax),
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
    Ok(())
}

fn wb16_validate_surface_nonnegative(
    ofe_index: usize,
    controls: Wb16OfeSurfaceControls,
) -> Result<(), HillslopeCliError> {
    if controls.inrcov < 0.0 || controls.rilcov < 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb16_ealpha_producer",
            detail: format!(
                "{SIMPIPE_GUARD_ID} ofe{ofe_index}_inrcov/rilcov must be >= 0.0, observed inrcov={inrcov}, rilcov={rilcov}",
                inrcov = controls.inrcov,
                rilcov = controls.rilcov
            ),
        });
    }
    if controls.rrinit < 0.0 || controls.rspace < 0.0 || controls.width < 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb16_ealpha_producer",
            detail: format!(
                "{SIMPIPE_GUARD_ID} ofe{ofe_index}_rrinit/rspace/width must be >= 0.0, observed rrinit={rrinit}, rspace={rspace}, width={width}",
                rrinit = controls.rrinit,
                rspace = controls.rspace,
                width = controls.width
            ),
        });
    }
    Ok(())
}

fn wb16_validate_canopy_nonnegative(
    ofe_index: usize,
    surface_controls: Wb16OfeSurfaceControls,
    canopy_controls: Wb16OfeCanopyControls,
) -> Result<(), HillslopeCliError> {
    if surface_controls.cancov < 0.0
        || canopy_controls.bb < 0.0
        || canopy_controls.bbb < 0.0
        || canopy_controls.flivmx < 0.0
        || canopy_controls.hmax < 0.0
    {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb16_ealpha_producer",
            detail: format!(
                "{SIMPIPE_GUARD_ID} ofe{ofe_index} canopy/friction controls must be >= 0.0 (cancov={cancov}, bb={bb}, bbb={bbb}, flivmx={flivmx}, hmax={hmax})",
                cancov = surface_controls.cancov,
                bb = canopy_controls.bb,
                bbb = canopy_controls.bbb,
                flivmx = canopy_controls.flivmx,
                hmax = canopy_controls.hmax
            ),
        });
    }
    Ok(())
}

fn wb16_normalize_ofe_controls(
    runtime_surface: &HillslopeWritebackSurface,
    ofe_index: usize,
    surface_controls: Wb16OfeSurfaceControls,
    canopy_controls: Wb16OfeCanopyControls,
) -> Result<Wb16OfeNormalizedControls, HillslopeCliError> {
    let inrcov = surface_controls.inrcov.min(WB16_COVER_CAP);
    let rilcov = surface_controls.rilcov.min(WB16_COVER_CAP);
    let cancov = surface_controls.cancov.min(WB16_COVER_CAP);
    let rrinit = surface_controls.rrinit.max(WB16_RRINIT_MIN_M);
    let rspace = if surface_controls.rspace <= 0.0 {
        WB16_RSPACE_DEFAULT_M
    } else {
        surface_controls.rspace
    };
    let rtyp = if surface_controls.rtyp >= 1.5 { 2 } else { 1 };
    let width = wb16_normalized_width(surface_controls.width, rspace, rtyp);
    let rrc = wb16_resolve_rrc(runtime_surface, ofe_index, rrinit)?;
    let canhgt = wb16_resolve_canhgt(
        runtime_surface,
        ofe_index,
        cancov,
        canopy_controls,
    )?;

    Ok(Wb16OfeNormalizedControls {
        inrcov,
        rilcov,
        rrinit,
        rspace,
        width,
        rrc,
        canhgt,
        flivmx: canopy_controls.flivmx,
        hmax: canopy_controls.hmax,
    })
}

fn wb16_normalized_width(width: f64, rspace: f64, rtyp: u8) -> f64 {
    let mut width = width;
    if rtyp == 1 && width <= 0.0 {
        width = WB16_TEMPORARY_WIDTH_DEFAULT_M;
    } else if rtyp == 2 && width <= 0.0 {
        width = rspace;
    }
    if width > rspace {
        width = rspace;
    }
    width
}

fn wb16_resolve_rrc(
    runtime_surface: &HillslopeWritebackSurface,
    ofe_index: usize,
    rrinit: f64,
) -> Result<f64, HillslopeCliError> {
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
    Ok(rrc)
}

fn wb16_resolve_canhgt(
    runtime_surface: &HillslopeWritebackSurface,
    ofe_index: usize,
    cancov: f64,
    controls: Wb16OfeCanopyControls,
) -> Result<f64, HillslopeCliError> {
    if let Some(canhgt) = wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "canhgt")
        .or_else(|| wb16_optional_state_scalar(runtime_surface, "canhgt"))
    {
        if !canhgt.is_finite() || canhgt < 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb16_ealpha_producer",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} ofe{ofe_index}_canhgt must be finite and >= 0.0, observed {canhgt}"
                ),
            });
        }
        return Ok(canhgt);
    }
    if controls.hmax <= 0.0 || controls.bb <= 0.0 {
        return Ok(0.0);
    }

    let mut vdmt = (1.0 - cancov).ln() / (-controls.bb);
    if vdmt < 0.0 {
        vdmt = 0.0;
    }
    Ok((1.0 - (-controls.bbb * vdmt).exp()) * controls.hmax)
}

fn wb16_compute_frlive(
    ofe_index: usize,
    controls: Wb16OfeNormalizedControls,
) -> Result<f64, HillslopeCliError> {
    let frlive = if controls.hmax > 0.0 {
        (controls.canhgt / controls.hmax) * controls.flivmx
    } else {
        0.0
    };
    if !frlive.is_finite() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb16_ealpha_producer",
            detail: format!("{SIMPIPE_GUARD_ID} ofe{ofe_index}_frlive is non-finite"),
        });
    }
    Ok(frlive)
}

fn wb16_compute_frcteq(
    ofe_index: usize,
    controls: Wb16OfeNormalizedControls,
    frlive: f64,
) -> Result<f64, HillslopeCliError> {
    let mut rrrinr = controls.rrc / controls.rrinit;
    if rrrinr > 1.0 {
        rrrinr = 1.0;
    }
    let roughness_factor = (3.024 - 5.042 * (-161.0 * controls.rrinit).exp()).exp();
    let mut roughness_total =
        0.5 * roughness_factor.powf(1.128) * (-3.088 * (1.0 - rrrinr)).exp();
    if roughness_total < WB16_INRFSO_CROPLAND {
        roughness_total = WB16_INRFSO_CROPLAND;
    }
    let roughness_delta = roughness_total - WB16_INRFSO_CROPLAND;
    let interrill_cover = if controls.inrcov > 0.0 {
        14.5 * controls.inrcov.powf(1.5544)
    } else {
        0.0
    };
    let interrill_total = roughness_delta + interrill_cover + WB16_INRFSO_CROPLAND + frlive;
    let rill_cover = if controls.rilcov > 0.0 {
        4.5 * controls.rilcov.powf(1.5544)
    } else {
        0.0
    };
    let rill_control = rill_cover + frlive + WB16_FRCSOL_CROPLAND;
    let rill_area_ratio = controls.width / controls.rspace;
    let frcteq = if rill_area_ratio < 1.0 {
        interrill_total + rill_area_ratio * (rill_control - interrill_total)
    } else {
        interrill_total
    };
    if !frcteq.is_finite() || frcteq <= 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb16_ealpha_producer",
            detail: format!(
                "{SIMPIPE_GUARD_ID} ofe{ofe_index}_frcteq must be finite and > 0.0, observed {frcteq}"
            ),
        });
    }
    Ok(frcteq)
}

fn wb16_publish_ofe_frcteq(
    runtime_surface: &mut HillslopeWritebackSurface,
    ofe_index: usize,
    frcteq: f64,
) {
    runtime_surface.state_surface.insert(
        BoundarySymbol::from(format!("ofe{ofe_index}_frcteq")),
        BoundaryValue::scalar(frcteq),
    );
}

fn wb16_compute_ofe_alpha(
    ofe_index: usize,
    avgslp: f64,
    frcteq: f64,
) -> Result<f64, HillslopeCliError> {
    let alpha = ((avgslp * 8.0 * WB16_ACCGAV_M_S2) / frcteq).sqrt();
    if !alpha.is_finite() || alpha <= 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb16_ealpha_producer",
            detail: format!(
                "{SIMPIPE_GUARD_ID} ofe{ofe_index}_alpha must be finite and > 0.0, observed {alpha}"
            ),
        });
    }
    Ok(alpha)
}

fn wb16_publish_ofe_alpha(
    runtime_surface: &mut HillslopeWritebackSurface,
    ofe_index: usize,
    alpha: f64,
) {
    runtime_surface.state_surface.insert(
        BoundarySymbol::from(format!("ofe{ofe_index}_alpha")),
        BoundaryValue::scalar(alpha),
    );
    if ofe_index == 1 {
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("alpha"), BoundaryValue::scalar(alpha));
    }
}

fn wb16_equivalent_plane_alpha(
    ofe_results: &[Wb16OfeAlphaResult],
    powers: Wb16EalphaPowers,
) -> Result<f64, HillslopeCliError> {
    if ofe_results.len() == 1 {
        return wb16_validate_ealpha(ofe_results[0].alpha);
    }

    let suml: f64 = ofe_results.iter().map(|result| result.slplen).sum();
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
    for result in ofe_results {
        cml += result.slplen;
        let tmpvr1 = cml.powf(powers.power3);
        sdst += (tmpvr1 - tmpvr2) / result.alpha.powf(powers.power2);
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
    wb16_validate_ealpha((suml / sdst).powf(powers.m) * suml)
}

fn wb16_validate_ealpha(ealpha: f64) -> Result<f64, HillslopeCliError> {
    if !ealpha.is_finite() || ealpha <= 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb16_ealpha_producer",
            detail: format!(
                "{SIMPIPE_GUARD_ID} WB16 produced ealpha must be finite and > 0.0, observed {ealpha}"
            ),
        });
    }
    Ok(ealpha)
}

fn wb16_publish_ealpha(runtime_surface: &mut HillslopeWritebackSurface, ealpha: f64) {
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("ealpha"),
        BoundaryValue::scalar(ealpha),
    );
}

pub(super) fn wb16_optional_state_scalar(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &str,
) -> Option<f64> {
    runtime_surface_symbol_value(runtime_surface, symbol)
}

pub(super) fn wb16_ofe_optional_state_scalar(
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

pub(super) const MOFE03_WAVE2_ENABLE_TOLERANCE: f64 = 1.0e-9;
pub(super) const MOFE03_WAVE2_MIN_POSITIVE: f64 = 1.0e-6;
pub(super) const MOFE03_WAVE2_DEFAULT_XTOP: f64 = 0.2;
pub(super) const MOFE03_WAVE2_DEFAULT_XBOT: f64 = 0.5;
pub(super) const MOFE03_WAVE2_DEFAULT_XDETST: f64 = 0.1;
pub(super) const MOFE03_WAVE2_DEFAULT_LDTOP: f64 = 0.8;
pub(super) const MOFE03_WAVE2_DEFAULT_LDBOT: f64 = 0.6;
pub(super) const MOFE03_WAVE2_DEFAULT_LDDEND: f64 = 0.3;
pub(super) const MOFE03_WAVE2_DEFAULT_KTRATO: f64 = 1.1;
pub(super) const MOFE03_WAVE2_DEFAULT_AINTC: f64 = 0.4;
pub(super) const MOFE03_WAVE2_DEFAULT_BINTC: f64 = 0.3;
pub(super) const MOFE03_WAVE2_DEFAULT_CINTC: f64 = 0.2;
pub(super) const MOFE03_WAVE2_DEFAULT_BETA: f64 = 0.5;
pub(super) const MOFE03_WAVE2_DEFAULT_QOSTAR: f64 = 0.2;
pub(super) const MOFE03_WAVE2_DEFAULT_SSA_SOIL: f64 = 5.0;
pub(super) const MOFE03_ROUTE_SEGMENT_INDEX: usize = 2;

#[derive(Debug, Clone, Copy)]
pub(super) struct Mofe03Wave2CaseScalars {
    case_value: f64,
    qj_minus_1: f64,
    vj: f64,
    qj: f64,
    fh: f64,
    fp: f64,
}

pub(super) fn seed_mofe03_wave2_runtime_surface_inputs(
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

pub(super) fn resolve_mofe03_ofe_count(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<usize, HillslopeCliError> {
    let (symbol, value) = match runtime_surface_symbol_value(
        runtime_surface,
        "mofe.static_lane.contributor_ofe_count",
    ) {
        Some(value) => ("mofe.static_lane.contributor_ofe_count", value),
        None => (
            "nelem",
            require_mofe03_runtime_surface_scalar(runtime_surface, "nelem")?,
        ),
    };
    let ofe_count = scalar_to_usize(symbol, value)?;
    if ofe_count == 0 {
        return Err(mofe03_wave2_seed_failure(
            "MOFE03 OFE count must be >= 1 for activation policy",
        ));
    }
    Ok(ofe_count)
}

pub(super) fn resolve_mofe03_wave2_enabled(
    runtime_surface: &HillslopeWritebackSurface,
    ofe_count: usize,
) -> Result<bool, HillslopeCliError> {
    if let Some(value) = runtime_surface_symbol_value(runtime_surface, "erod14_wave2_enabled") {
        parse_mofe03_binary_flag("erod14_wave2_enabled", value)
    } else {
        Ok(ofe_count > 1)
    }
}

pub(super) fn write_mofe03_wave2_enabled(runtime_surface: &mut HillslopeWritebackSurface, enabled: bool) {
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_wave2_enabled"),
        BoundaryValue::scalar(if enabled { 1.0 } else { 0.0 }),
    );
}

pub(super) fn require_mofe03_positive_runtime_surface_scalar(
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

pub(super) fn resolve_mofe03_wave2_qout(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<f64, HillslopeCliError> {
    require_mofe03_non_negative_seed_scalar(
        runtime_surface_symbol_value(runtime_surface, "Q")
            .or_else(|| runtime_surface_symbol_value(runtime_surface, "wb12_runoff_observed"))
            .unwrap_or(0.0),
        "erod14_qout",
    )
}

pub(super) fn resolve_mofe03_wave2_qin(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<f64, HillslopeCliError> {
    require_mofe03_non_negative_seed_scalar(
        runtime_surface_symbol_value(runtime_surface, "UpStrmQ").unwrap_or(0.0),
        "erod14_qin",
    )
}

pub(super) fn build_mofe03_wave2_case_scalars(qout: f64) -> Mofe03Wave2CaseScalars {
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

pub(super) fn seed_mofe03_wave2_core_scalars(
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

pub(super) fn seed_mofe03_wave2_route_topology_ingress(
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

pub(super) fn seed_mofe03_scalar_if_absent(
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

pub(super) fn seed_mofe03_segment_scalar_if_absent(
    runtime_surface: &mut HillslopeWritebackSurface,
    root: &str,
    segment_index: usize,
    value: f64,
) {
    let symbol = format!("{root}_{segment_index:04}");
    seed_mofe03_scalar_if_absent(runtime_surface, &symbol, value);
}

pub(super) fn resolve_mofe03_wave2_beta_theta(
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

pub(super) fn seed_mofe03_wave2_case_state(
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

pub(super) fn seed_mofe03_wave2_ssa_soil(
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

pub(super) fn seed_mofe03_wave2_class_symbols(
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
pub(super) fn execute_scheduler_kernel_lifecycle(
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

struct PersistentLaneInputPreparation {
    lane_inputs: Vec<OfeLaneExecutionInput>,
    pl_activation_sentinels: Vec<Option<BoundaryValue>>,
    previous_storage_totals_mm: Vec<f64>,
}

fn prepare_persistent_lane_inputs(
    lane_state: &mut OfeLanePersistentStateSequence,
    climate_surface: &HillslopeWritebackSurface,
    stale_climate_symbols: &[BoundarySymbol],
    context: &SchedulerLifecycleContext<'_>,
) -> Result<PersistentLaneInputPreparation, HillslopeCliError> {
    let mut lane_inputs = Vec::with_capacity(lane_state.lane_states().len());
    let mut pl_activation_sentinels = Vec::with_capacity(lane_state.lane_states().len());
    let mut previous_storage_totals_mm = Vec::with_capacity(lane_state.lane_states().len());

    for lane in lane_state.lane_states_mut() {
        super::indexed_shadow_surface::observe_clone_source_surface(&lane.writeback_surface)?;
        let lane_ofe_id = lane.ofe_id;
        let upstream_area_ratio = lane.upstream_area_ratio;
        let mut lane_surface = lane.take_execution_input().writeback_surface;
        for symbol in stale_climate_symbols {
            lane_surface.state_surface.remove(symbol);
            lane_surface.flux_surface.remove(symbol);
        }

        crate::hillslope::intake_lane_setup::extend_runtime_surface_from(
            &mut lane_surface,
            climate_surface,
        );
        seed_wb11_runtime_surface_inputs(&mut lane_surface, context.execution_lane)?;
        seed_scheduler_calendar_symbols(&mut lane_surface, context);
        previous_storage_totals_mm.push(internal_wb13_storage_total_mm_from_surface(
            &lane_surface,
        )?);
        pl_activation_sentinels.push(pl_runtime_activation_sentinel_value(&lane_surface));
        prepare_pl_runtime_activation_for_scheduler(&mut lane_surface)?;
        lane_inputs.push(OfeLaneExecutionInput::with_upstream_area_ratio(
            lane_ofe_id,
            upstream_area_ratio,
            lane_surface,
        ));
    }

    Ok(PersistentLaneInputPreparation {
        lane_inputs,
        pl_activation_sentinels,
        previous_storage_totals_mm,
    })
}

fn execute_persistent_ofe_sequence(
    lane_inputs: Vec<OfeLaneExecutionInput>,
) -> Result<OfeLaneSequenceExecutionReport, HillslopeCliError> {
    let topology_graph = TopologyGraph::new(1, 0, 0, Vec::new());
    let topology_report = validate_pre_execution_topology(&topology_graph).map_err(|error| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "per_ofe_dynamic_state",
            detail: format!(
                "{SIMPIPE_GUARD_ID} failed building persistent OFE topology precondition report: {error}"
            ),
        }
    })?;

    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;
    scheduler
        .execute_ofe_sequence_with_kernel(&topology_report, &mut kernel, lane_inputs)
        .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "per_ofe_dynamic_state",
            detail: format!(
                "{SIMPIPE_GUARD_ID} persistent OFE scheduler/kernel lifecycle failed: {error}"
            ),
        })
}

fn require_persistent_lane_sequence_success(
    sequence_report: &OfeLaneSequenceExecutionReport,
) -> Result<(), HillslopeCliError> {
    for lane_report in &sequence_report.lane_reports {
        if !lane_report.kernel_report.scheduler_report.is_success() {
            let scheduler_status = &lane_report.kernel_report.scheduler_report.scheduler_status;
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "per_ofe_dynamic_state",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} persistent OFE {} scheduler lifecycle did not complete successfully (outcome_class={}, status_class={:?}, boundary_class={}, message_id={})",
                    lane_report.ofe_id,
                    scheduler_outcome_class_as_str(
                        lane_report.kernel_report.scheduler_report.outcome_class
                    ),
                    scheduler_status.classification(),
                    scheduler_status.boundary_class().as_str(),
                    scheduler_status.message_id()
                ),
            });
        }
    }
    Ok(())
}

fn restore_persistent_lane_pl_sentinels(
    lane_state: &mut OfeLanePersistentStateSequence,
    pl_activation_sentinels: Vec<Option<BoundaryValue>>,
) {
    for (lane, sentinel) in lane_state
        .lane_states_mut()
        .iter_mut()
        .zip(pl_activation_sentinels)
    {
        restore_pl_runtime_activation_sentinel_for_next_day(&mut lane.writeback_surface, sentinel);
    }
}

fn persistent_outlet_runtime_surface(
    lane_state: &OfeLanePersistentStateSequence,
) -> Result<HillslopeWritebackSurface, HillslopeCliError> {
    lane_state
        .lane_states()
        .last()
        .map(|lane| lane.writeback_surface.clone())
        .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "per_ofe_dynamic_state",
            detail: format!("{SIMPIPE_GUARD_ID} persistent OFE state has no outlet lane"),
        })
}

fn persistent_kernel_phase_message_ids(
    sequence_report: &OfeLaneSequenceExecutionReport,
) -> Vec<String> {
    sequence_report
        .lane_reports
        .iter()
        .flat_map(|lane_report| {
            lane_report
                .kernel_report
                .phase_reports
                .iter()
                .map(|phase| phase.kernel_status.message_id().to_string())
        })
        .collect::<Vec<_>>()
}

fn persistent_erod14_wave2_kernel_status_seen(
    sequence_report: &OfeLaneSequenceExecutionReport,
) -> bool {
    sequence_report.lane_reports.iter().any(|lane_report| {
        lane_report
            .kernel_report
            .phase_reports
            .iter()
            .any(|phase| {
                let message_id = phase.kernel_status.message_id();
                message_id.contains("EROD14-WAVE2")
                    || message_id.contains("EROD18-ROUTE")
                    || message_id.contains("EROD19-ROUTE")
            })
    })
}

struct PersistentSequenceSummary {
    scheduler_outcome_class: SchedulerOutcomeClass,
    scheduler_status_message_id: String,
    kernel_phase_message_ids: Vec<String>,
    erod14_wave2_kernel_status_seen: bool,
}

fn persistent_sequence_summary(
    sequence_report: &OfeLaneSequenceExecutionReport,
) -> Result<PersistentSequenceSummary, HillslopeCliError> {
    let last_lane_report =
        sequence_report
            .lane_reports
            .last()
            .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "per_ofe_dynamic_state",
                detail: format!("{SIMPIPE_GUARD_ID} persistent OFE sequence produced no lanes"),
            })?;

    Ok(PersistentSequenceSummary {
        scheduler_outcome_class: last_lane_report.kernel_report.scheduler_report.outcome_class,
        scheduler_status_message_id: last_lane_report
            .kernel_report
            .scheduler_report
            .scheduler_status
            .message_id()
            .to_string(),
        kernel_phase_message_ids: persistent_kernel_phase_message_ids(sequence_report),
        erod14_wave2_kernel_status_seen: persistent_erod14_wave2_kernel_status_seen(
            sequence_report,
        ),
    })
}

fn replace_persistent_lane_state_from_report_moving(
    lane_state: &mut OfeLanePersistentStateSequence,
    sequence_report: OfeLaneSequenceExecutionReport,
) -> Result<(), HillslopeCliError> {
    let lane_reports = sequence_report.lane_reports;

    if lane_state.lane_states().len() != lane_reports.len() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "per_ofe_dynamic_state",
            detail: format!(
                "{SIMPIPE_GUARD_ID} persistent OFE state replacement failed: expected {} lanes, observed {} lanes",
                lane_state.lane_states().len(),
                lane_reports.len()
            ),
        });
    }

    for (state, lane_report) in lane_state.lane_states_mut().iter_mut().zip(lane_reports) {
        if state.ofe_id != lane_report.ofe_id {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "per_ofe_dynamic_state",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} persistent OFE state replacement failed: expected OFE {}, observed OFE {}",
                    state.ofe_id,
                    lane_report.ofe_id
                ),
            });
        }

        state.writeback_surface = lane_report.kernel_report.writeback_surface;
    }

    Ok(())
}

fn refresh_persistent_lane_indexed_authority(
    lane_state: &mut OfeLanePersistentStateSequence,
    registry: &SymbolRegistry,
) -> Result<(), HillslopeCliError> {
    lane_state
        .refresh_indexed_writeback_authority(registry)
        .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "indexed_runtime_surface",
            detail: error.to_string(),
        })
}

pub(super) fn execute_persistent_scheduler_kernel_lifecycle(
    lane_state: &mut OfeLanePersistentStateSequence,
    climate_surface: &HillslopeWritebackSurface,
    stale_climate_symbols: &[BoundarySymbol],
    lane_areas_m2: &[f64],
    runoff_publication_geometries: &[Wb13RunoffPublicationGeometry],
    context: SchedulerLifecycleContext<'_>,
) -> Result<PersistentDailyExecutionResult, HillslopeCliError> {
    let lane_preparation = prepare_persistent_lane_inputs(
        lane_state,
        climate_surface,
        stale_climate_symbols,
        &context,
    )?;
    let sequence_report = execute_persistent_ofe_sequence(lane_preparation.lane_inputs)?;
    require_persistent_lane_sequence_success(&sequence_report)?;
    let internal_wb13_collection = DailyInternalPerOfeWb13Collection::from_sequence_report(
        &sequence_report,
        lane_areas_m2,
        runoff_publication_geometries,
        &lane_preparation.previous_storage_totals_mm,
        context,
    )?;
    let sequence_summary = persistent_sequence_summary(&sequence_report)?;
    replace_persistent_lane_state_from_report_moving(lane_state, sequence_report)?;
    restore_persistent_lane_pl_sentinels(lane_state, lane_preparation.pl_activation_sentinels);
    refresh_persistent_lane_indexed_authority(lane_state, context.symbol_registry)?;

    let outlet_runtime_surface = persistent_outlet_runtime_surface(lane_state)?;
    super::indexed_shadow_surface::validate_shadow_surface(&outlet_runtime_surface)?;
    let outlet_row = internal_wb13_collection
        .outlet_row()
        .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "per_ofe_internal_wb13",
            detail: format!("{SIMPIPE_GUARD_ID} internal WB13 collection has no outlet row"),
        })?;
    let coupling_vectors =
        build_simimpl10_coupling_vector_provenance(&outlet_runtime_surface, outlet_row)?;

    Ok(PersistentDailyExecutionResult {
        scheduler_outcome_class: sequence_summary.scheduler_outcome_class,
        scheduler_status_message_id: sequence_summary.scheduler_status_message_id,
        coupling_vectors,
        runtime_surface: outlet_runtime_surface,
        internal_wb13_collection,
        kernel_phase_message_ids: sequence_summary.kernel_phase_message_ids,
        erod14_wave2_kernel_status_seen: sequence_summary.erod14_wave2_kernel_status_seen,
        hphys0245_trace_rows: Vec::new(),
    })
}

pub(super) fn pl_runtime_activation_sentinel_value(
    runtime_surface: &HillslopeWritebackSurface,
) -> Option<BoundaryValue> {
    runtime_surface
        .state_surface
        .get(&BoundarySymbol::from("pl_schedule_slot_count"))
        .copied()
}

pub(super) fn seed_scheduler_calendar_symbols(
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

pub(super) fn restore_pl_runtime_activation_sentinel_for_next_day(
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

pub(super) fn prepare_pl_runtime_activation_for_scheduler(
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

pub(super) fn pl_runtime_has_active_crop_for_scheduler_day(
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

pub(super) fn pl_crop_slot_is_active_for_day(
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

pub(super) fn require_runtime_usize_in_range(
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

pub(super) fn day_is_within_julian_window(day_of_year: usize, start_day: usize, end_day: usize) -> bool {
    if start_day <= end_day {
        day_of_year >= start_day && day_of_year <= end_day
    } else {
        day_of_year >= start_day || day_of_year <= end_day
    }
}
