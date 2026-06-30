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

    seed_wb11_efflen_and_m_if_missing(runtime_surface)?;
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
    let contributor_ofe_count = runtime_surface_ofe_count(runtime_surface)?;
    let projection =
        project_typed_wb11_lane_substeps(execution_lane, contributor_ofe_count)?;
    runtime_surface.state_surface.insert(
        BoundarySymbol::from(WB18_PERC_LANE_SUBSTEPS_SYMBOL),
        BoundaryValue::scalar(projection.wb18_perc_lane_substeps),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from(WB19_LATERAL_DRAIN_LANE_SUBSTEPS_SYMBOL),
        BoundaryValue::scalar(projection.wb19_lateral_drain_lane_substeps),
    );
    seed_mofe_hourly_carry_runtime_surface_inputs(
        runtime_surface,
        projection.mofe_hourly_carry_active,
    )?;
    Ok(projection.mofe_hourly_carry_active)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TypedWb11LaneSubstepProjection {
    pub(crate) wb18_perc_lane_substeps: f64,
    pub(crate) wb19_lateral_drain_lane_substeps: f64,
    pub(crate) mofe_hourly_carry_active: bool,
}

pub(crate) fn project_typed_wb11_lane_substeps(
    execution_lane: ExecutionLane,
    contributor_ofe_count: usize,
) -> Result<TypedWb11LaneSubstepProjection, HillslopeCliError> {
    if contributor_ofe_count == 0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!("{SIMPIPE_GUARD_ID} contributor OFE count must be >= 1 for typed WB11 lane substeps"),
        });
    }
    let lane_substeps = match execution_lane {
        ExecutionLane::Daily => 1.0,
        ExecutionLane::Hourly => 24.0,
    };
    let mofe_hourly_carry_active = contributor_ofe_count > 1;
    let lane_substeps = if mofe_hourly_carry_active {
        24.0
    } else {
        lane_substeps
    };
    Ok(TypedWb11LaneSubstepProjection {
        wb18_perc_lane_substeps: lane_substeps,
        wb19_lateral_drain_lane_substeps: lane_substeps,
        mofe_hourly_carry_active,
    })
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
    let point_count_raw = require_runtime_surface_scalar(runtime_surface, hyetograph_point_symbol)?;
    let point_count = scalar_to_usize(hyetograph_point_symbol, point_count_raw)?;
    let intervals = if point_count == 0 {
        Vec::new()
    } else {
        read_typed_wb11_hyetograph_intervals(runtime_surface, point_count)?
    };
    let projection = project_typed_wb11_hyetograph(
        prcp,
        hyetograph_point_symbol,
        point_count,
        runtime_surface_symbol_value(runtime_surface, "stmdur"),
        &intervals,
    )?;
    if let Some(synthesized) = projection.synthesized_zero_event {
        publish_typed_wb11_synthesized_hyetograph(runtime_surface, synthesized);
    }
    let ninten_scalar = usize_to_scalar("ninten", projection.point_count)?;
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("ninten"),
        BoundaryValue::scalar(ninten_scalar),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("nbrkpt"),
        BoundaryValue::scalar(ninten_scalar),
    );
    Ok(projection.rainfall_input_m)
}

fn read_typed_wb11_hyetograph_intervals(
    runtime_surface: &HillslopeWritebackSurface,
    point_count: usize,
) -> Result<Vec<TypedWb11HyetographInterval>, HillslopeCliError> {
    let mut intervals = Vec::with_capacity(point_count.saturating_sub(1));
    for point_index in 1..point_count {
        let time_symbol = wb13_primary_layer_symbol("timem", point_index);
        let next_time_symbol = wb13_primary_layer_symbol("timem", point_index + 1);
        let intensity_symbol = wb13_primary_layer_symbol("intsty", point_index);
        intervals.push(TypedWb11HyetographInterval {
            point_index,
            time_s: require_runtime_surface_scalar(runtime_surface, time_symbol.as_str())?,
            next_time_s: require_runtime_surface_scalar(
                runtime_surface,
                next_time_symbol.as_str(),
            )?,
            intensity_m_s: require_runtime_surface_scalar(
                runtime_surface,
                intensity_symbol.as_str(),
            )?,
        });
    }
    Ok(intervals)
}

fn publish_typed_wb11_synthesized_hyetograph(
    runtime_surface: &mut HillslopeWritebackSurface,
    synthesized: TypedWb11SynthesizedHyetograph,
) {
    runtime_surface
        .state_surface
        .insert(BoundarySymbol::from("ninten"), BoundaryValue::scalar(2.0));
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("timem_0001"),
        BoundaryValue::scalar(synthesized.time_0001_s),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("timem_0002"),
        BoundaryValue::scalar(synthesized.time_0002_s),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("intsty_0001"),
        BoundaryValue::scalar(synthesized.intensity_0001_m_s),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("intsty_0002"),
        BoundaryValue::scalar(synthesized.intensity_0002_m_s),
    );
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TypedWb11HyetographInterval {
    pub(crate) point_index: usize,
    pub(crate) time_s: f64,
    pub(crate) next_time_s: f64,
    pub(crate) intensity_m_s: f64,
}

#[allow(clippy::struct_field_names)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TypedWb11SynthesizedHyetograph {
    pub(crate) time_0001_s: f64,
    pub(crate) time_0002_s: f64,
    pub(crate) intensity_0001_m_s: f64,
    pub(crate) intensity_0002_m_s: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TypedWb11HyetographProjection {
    pub(crate) point_count: usize,
    pub(crate) rainfall_input_m: f64,
    pub(crate) synthesized_zero_event: Option<TypedWb11SynthesizedHyetograph>,
}

pub(crate) fn project_typed_wb11_hyetograph(
    prcp: f64,
    point_count_symbol: &str,
    point_count: usize,
    storm_duration_s: Option<f64>,
    intervals: &[TypedWb11HyetographInterval],
) -> Result<TypedWb11HyetographProjection, HillslopeCliError> {
    if point_count == 0 {
        let stmdur = storm_duration_s.unwrap_or(1.0).max(1.0);
        let intensity = if stmdur > 0.0 { prcp / stmdur } else { prcp };
        let synthesized_rainfall_m = (intensity.max(0.0) * stmdur).max(prcp);
        return Ok(TypedWb11HyetographProjection {
            point_count: 2,
            rainfall_input_m: synthesized_rainfall_m,
            synthesized_zero_event: Some(TypedWb11SynthesizedHyetograph {
                time_0001_s: 0.0,
                time_0002_s: stmdur,
                intensity_0001_m_s: intensity.max(0.0),
                intensity_0002_m_s: 0.0,
            }),
        });
    }
    if intervals.len() != point_count.saturating_sub(1) {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!(
                "{SIMPIPE_GUARD_ID} {point_count_symbol}={point_count} requires {} hyetograph intervals, observed {}",
                point_count.saturating_sub(1),
                intervals.len()
            ),
        });
    }
    let mut hyetograph_rainfall = 0.0_f64;
    for interval in intervals {
        let time_symbol = wb13_primary_layer_symbol("timem", interval.point_index);
        let next_time_symbol = wb13_primary_layer_symbol("timem", interval.point_index + 1);
        let intensity_symbol = wb13_primary_layer_symbol("intsty", interval.point_index);

        if interval.next_time_s < interval.time_s {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb11_seed",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} {next_time_symbol} ({}) must be >= {time_symbol} ({})",
                    interval.next_time_s,
                    interval.time_s
                ),
            });
        }
        if interval.intensity_m_s < 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb11_seed",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} {intensity_symbol} must be >= 0.0, observed {}",
                    interval.intensity_m_s
                ),
            });
        }

        hyetograph_rainfall += interval.intensity_m_s * (interval.next_time_s - interval.time_s);
    }
    Ok(TypedWb11HyetographProjection {
        point_count,
        rainfall_input_m: hyetograph_rainfall.max(prcp),
        synthesized_zero_event: None,
    })
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TypedWb11LayerSeedInput {
    pub(crate) dg: f64,
    pub(crate) thetfc: f64,
    pub(crate) thetdr: f64,
    pub(crate) ssc: f64,
    pub(crate) por: f64,
    pub(crate) cpm: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TypedWb11LayerSeedProjection {
    pub(crate) theta: f64,
    pub(crate) field_capacity: f64,
    pub(crate) upper_limit: f64,
    pub(crate) soil_water: f64,
}

struct Wb11LayerSeedSymbols {
    dg: String,
    fc: String,
    wp: String,
    ssc: String,
    por: String,
    cpm: String,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct TypedWb11InitialStorageTotals {
    pub(crate) soil_water: f64,
    pub(crate) field_capacity: f64,
    pub(crate) drainable_storage: f64,
    pub(crate) drainage_coefficient: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TypedWb11InitialStorageProjection {
    pub(crate) saturation: f64,
    pub(crate) layers: Vec<TypedWb11LayerSeedProjection>,
    pub(crate) totals: TypedWb11InitialStorageTotals,
}

fn seed_initial_wb11_storage_if_needed(
    runtime_surface: &mut HillslopeWritebackSurface,
    nsl: usize,
    execution_lane: ExecutionLane,
) -> Result<(), HillslopeCliError> {
    if wb11_runtime_state_is_seeded(runtime_surface) {
        return Ok(());
    }

    let sat = require_runtime_surface_scalar(runtime_surface, "sat")?;
    let mut layers = Vec::with_capacity(nsl);
    for layer_index in 1..=nsl {
        layers.push(require_wb11_layer_seed_inputs(runtime_surface, layer_index)?);
    }
    let projection = project_typed_wb11_initial_storage(sat, execution_lane, &layers)?;
    publish_typed_wb11_initial_storage_projection(runtime_surface, &layers, &projection);
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

pub(crate) fn project_typed_wb11_initial_storage(
    sat: f64,
    execution_lane: ExecutionLane,
    layers: &[TypedWb11LayerSeedInput],
) -> Result<TypedWb11InitialStorageProjection, HillslopeCliError> {
    if layers.is_empty() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!("{SIMPIPE_GUARD_ID} typed WB11 initial storage requires at least one layer"),
        });
    }
    let mut totals = TypedWb11InitialStorageTotals::default();
    let mut sat = cap_typed_wb11_initial_saturation(sat, execution_lane)?;
    let mut layer_projections = Vec::with_capacity(layers.len());
    for (layer_offset, layer) in layers.iter().copied().enumerate() {
        let layer_index = layer_offset + 1;
        validate_typed_wb11_layer_seed_input(layer_index, layer)?;
        sat = apply_typed_wb11_layer_saturation_floor(layer_index, sat, layer)?;
        let projection = project_typed_wb11_layer_seed(layer_index, sat, layer)?;

        totals.soil_water += projection.soil_water;
        totals.field_capacity += projection.field_capacity;
        totals.drainable_storage += (projection.theta - projection.field_capacity).max(0.0);
        totals.drainage_coefficient += layer.ssc * 86_400.0;
        layer_projections.push(projection);
    }
    Ok(TypedWb11InitialStorageProjection {
        saturation: sat,
        layers: layer_projections,
        totals,
    })
}

fn cap_typed_wb11_initial_saturation(
    mut sat: f64,
    execution_lane: ExecutionLane,
) -> Result<f64, HillslopeCliError> {
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

pub(crate) fn require_wb11_layer_seed_inputs(
    runtime_surface: &HillslopeWritebackSurface,
    layer_index: usize,
) -> Result<TypedWb11LayerSeedInput, HillslopeCliError> {
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
        TypedWb11LayerSeedInput {
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
    layer: TypedWb11LayerSeedInput,
) -> Result<TypedWb11LayerSeedInput, HillslopeCliError> {
    validate_wb11_layer_storage_geometry(symbols, layer)?;
    validate_wb11_layer_storage_order(symbols, layer)?;
    validate_wb11_layer_transport_scalars(symbols, layer)?;
    Ok(layer)
}

fn validate_wb11_layer_storage_geometry(
    symbols: &Wb11LayerSeedSymbols,
    layer: TypedWb11LayerSeedInput,
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
    layer: TypedWb11LayerSeedInput,
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
    layer: TypedWb11LayerSeedInput,
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

fn validate_typed_wb11_layer_seed_input(
    layer_index: usize,
    layer: TypedWb11LayerSeedInput,
) -> Result<(), HillslopeCliError> {
    let symbols = Wb11LayerSeedSymbols {
        dg: format!("typed.wb19_dg_{layer_index:04}"),
        fc: format!("typed.wb19_thetfc_{layer_index:04}"),
        wp: format!("typed.wb19_thetdr_{layer_index:04}"),
        ssc: format!("typed.ssc_{layer_index:04}"),
        por: format!("typed.wb19_por_{layer_index:04}"),
        cpm: format!("typed.cpm_{layer_index:04}"),
    };
    validate_wb11_layer_seed_inputs(&symbols, layer)?;
    Ok(())
}

fn apply_typed_wb11_layer_saturation_floor(
    layer_index: usize,
    mut sat: f64,
    layer: TypedWb11LayerSeedInput,
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

fn project_typed_wb11_layer_seed(
    layer_index: usize,
    sat: f64,
    layer: TypedWb11LayerSeedInput,
) -> Result<TypedWb11LayerSeedProjection, HillslopeCliError> {
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

    Ok(TypedWb11LayerSeedProjection {
        theta: theta_store,
        field_capacity: fc_store,
        upper_limit: ul_store,
        soil_water: soilw_store,
    })
}

fn publish_wb11_layer_seed_stores(
    runtime_surface: &mut HillslopeWritebackSurface,
    layer_index: usize,
    layer: TypedWb11LayerSeedInput,
    stores: TypedWb11LayerSeedProjection,
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
    totals: TypedWb11InitialStorageTotals,
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

fn publish_typed_wb11_initial_storage_projection(
    runtime_surface: &mut HillslopeWritebackSurface,
    inputs: &[TypedWb11LayerSeedInput],
    projection: &TypedWb11InitialStorageProjection,
) {
    for (layer_offset, (layer, stores)) in inputs.iter().zip(projection.layers.iter()).enumerate() {
        publish_wb11_layer_seed_stores(
            runtime_surface,
            layer_offset + 1,
            *layer,
            *stores,
        );
    }
    publish_wb11_initial_seed_totals(runtime_surface, projection.saturation, projection.totals);
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
    let projection = project_typed_wb11_optional_defaults(
        runtime_surface_symbol_value(runtime_surface, "wb17_residue_interception"),
        runtime_surface_symbol_value(runtime_surface, "Ws"),
    );
    if projection.residue_interception_was_defaulted {
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb17_residue_interception"),
            BoundaryValue::scalar(projection.residue_interception_m),
        );
    }
    if projection.water_stress_was_defaulted {
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Ws"), BoundaryValue::scalar(projection.water_stress));
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TypedWb11OptionalDefaultsProjection {
    pub(crate) residue_interception_m: f64,
    pub(crate) residue_interception_was_defaulted: bool,
    pub(crate) water_stress: f64,
    pub(crate) water_stress_was_defaulted: bool,
}

pub(crate) fn project_typed_wb11_optional_defaults(
    residue_interception_m: Option<f64>,
    water_stress: Option<f64>,
) -> TypedWb11OptionalDefaultsProjection {
    TypedWb11OptionalDefaultsProjection {
        residue_interception_m: residue_interception_m.unwrap_or(0.0),
        residue_interception_was_defaulted: residue_interception_m.is_none(),
        water_stress: water_stress.unwrap_or(1.0),
        water_stress_was_defaulted: water_stress.is_none(),
    }
}
