// Typed WB11 day-zero seed projection cores owned by the direct runtime.
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
