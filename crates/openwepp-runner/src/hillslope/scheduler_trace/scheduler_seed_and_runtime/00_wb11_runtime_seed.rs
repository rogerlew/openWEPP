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
