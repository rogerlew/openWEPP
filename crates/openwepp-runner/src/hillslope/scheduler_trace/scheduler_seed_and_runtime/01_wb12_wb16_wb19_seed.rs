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
    let projection = project_typed_wb12_reconciliation_seed(
        rainfall_input,
        prcp,
        wb11_soil_water,
        mofe_hourly_carry_active,
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb12_rainfall_input"),
        BoundaryValue::scalar(projection.rainfall_input_m),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb12_runon_input"),
        BoundaryValue::scalar(projection.runon_input_m),
    );
    let carryover_symbol = BoundarySymbol::from("wb12_runoff_carryover");
    if let Some(runoff_carryover_m) = projection.runoff_carryover_m {
        runtime_surface.flux_surface.insert(
            carryover_symbol,
            BoundaryValue::scalar(runoff_carryover_m),
        );
    } else {
        runtime_surface.flux_surface.remove(&carryover_symbol);
    }
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb12_infiltration"),
        BoundaryValue::scalar(projection.infiltration_m),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb12_depression_storage_delta"),
        BoundaryValue::scalar(projection.depression_storage_delta_m),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb12_runoff_observed"),
        BoundaryValue::scalar(projection.runoff_observed_m),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb12_runoff_closure_tolerance"),
        BoundaryValue::scalar(projection.runoff_closure_tolerance_m),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb12_storage_initial"),
        BoundaryValue::scalar(projection.storage_initial_m),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb12_storage_observed"),
        BoundaryValue::scalar(projection.storage_observed_m),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb12_storage_closure_tolerance"),
        BoundaryValue::scalar(projection.storage_closure_tolerance_m),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb12_precip_input"),
        BoundaryValue::scalar(projection.precip_input_m),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb20_forward_solver_lane_enabled"),
        BoundaryValue::scalar(projection.forward_solver_lane_enabled),
    );
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TypedWb12ReconciliationSeedProjection {
    pub(crate) rainfall_input_m: f64,
    pub(crate) runon_input_m: f64,
    pub(crate) runoff_carryover_m: Option<f64>,
    pub(crate) infiltration_m: f64,
    pub(crate) depression_storage_delta_m: f64,
    pub(crate) runoff_observed_m: f64,
    pub(crate) runoff_closure_tolerance_m: f64,
    pub(crate) storage_initial_m: f64,
    pub(crate) storage_observed_m: f64,
    pub(crate) storage_closure_tolerance_m: f64,
    pub(crate) precip_input_m: f64,
    pub(crate) forward_solver_lane_enabled: f64,
}

pub(crate) fn project_typed_wb12_reconciliation_seed(
    rainfall_input_m: f64,
    precip_input_m: f64,
    wb11_soil_water_m: f64,
    mofe_hourly_carry_active: bool,
) -> TypedWb12ReconciliationSeedProjection {
    TypedWb12ReconciliationSeedProjection {
        rainfall_input_m,
        runon_input_m: 0.0,
        runoff_carryover_m: (!mofe_hourly_carry_active).then_some(0.0),
        infiltration_m: 0.0,
        depression_storage_delta_m: 0.0,
        runoff_observed_m: 0.0,
        runoff_closure_tolerance_m: 1.0,
        storage_initial_m: wb11_soil_water_m,
        storage_observed_m: wb11_soil_water_m,
        storage_closure_tolerance_m: 1.0,
        precip_input_m,
        forward_solver_lane_enabled: 1.0,
    }
}

fn seed_wb11_efflen_and_m_if_missing(
    runtime_surface: &mut HillslopeWritebackSurface,
) -> Result<(), HillslopeCliError> {
    let slplen = require_runtime_surface_scalar(runtime_surface, "slplen")?;
    let projection = project_typed_wb11_efflen_and_m(
        runtime_surface_symbol_value(runtime_surface, "efflen"),
        slplen,
        runtime_surface_symbol_value(runtime_surface, "m"),
    )?;
    if projection.efflen_was_defaulted {
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("efflen"),
            BoundaryValue::scalar(projection.efflen_m),
        );
    }
    if projection.exponent_was_defaulted {
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("m"), BoundaryValue::scalar(projection.exponent_m));
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TypedWb11EfflenAndMProjection {
    pub(crate) efflen_m: f64,
    pub(crate) efflen_was_defaulted: bool,
    pub(crate) exponent_m: f64,
    pub(crate) exponent_was_defaulted: bool,
}

pub(crate) fn project_typed_wb11_efflen_and_m(
    efflen_m: Option<f64>,
    slplen_m: f64,
    exponent_m: Option<f64>,
) -> Result<TypedWb11EfflenAndMProjection, HillslopeCliError> {
    let (efflen_m, efflen_was_defaulted) = if let Some(efflen_m) = efflen_m {
        (efflen_m, false)
    } else {
        if slplen_m <= 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb11_seed",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} slplen must be > 0.0 when seeding efflen, observed {slplen_m}"
                ),
            });
        }
        (slplen_m, true)
    };
    Ok(TypedWb11EfflenAndMProjection {
        efflen_m,
        efflen_was_defaulted,
        exponent_m: exponent_m.unwrap_or(1.5),
        exponent_was_defaulted: exponent_m.is_none(),
    })
}

fn seed_wb16_ealpha_compatibility(
    runtime_surface: &mut HillslopeWritebackSurface,
) -> Result<(), HillslopeCliError> {
    let ealpha_seeded_prior =
        runtime_surface_symbol_value(runtime_surface, WB16_EALPHA_COMPATIBILITY_SEED_FLAG_SYMBOL)
            .is_some_and(|value| value >= 0.5);
    let ealpha_runtime_produced_this_day =
        produce_wb16_ealpha_from_runtime_surface(runtime_surface)?.is_some();
    let projection = project_typed_wb16_ealpha_compatibility(
        ealpha_seeded_prior,
        ealpha_runtime_produced_this_day,
    );
    if let Some(default_ealpha) = projection.default_ealpha {
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("ealpha"), BoundaryValue::scalar(default_ealpha));
    }
    runtime_surface.state_surface.insert(
        BoundarySymbol::from(WB16_EALPHA_COMPATIBILITY_SEED_FLAG_SYMBOL),
        BoundaryValue::scalar(projection.seeded_any_day_flag),
    );
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TypedWb16EalphaCompatibilityProjection {
    pub(crate) default_ealpha: Option<f64>,
    pub(crate) seeded_any_day_flag: f64,
}

pub(crate) fn project_typed_wb16_ealpha_compatibility(
    ealpha_seeded_prior: bool,
    ealpha_runtime_produced_this_day: bool,
) -> TypedWb16EalphaCompatibilityProjection {
    let ealpha_seeded_any_day = !ealpha_runtime_produced_this_day || ealpha_seeded_prior;
    TypedWb16EalphaCompatibilityProjection {
        default_ealpha: (!ealpha_runtime_produced_this_day).then_some(1.0),
        seeded_any_day_flag: if ealpha_seeded_any_day { 1.0 } else { 0.0 },
    }
}

fn refresh_wb18_frozen_depth_from_fine_frost_state(
    runtime_surface: &mut HillslopeWritebackSurface,
    nsl: usize,
) -> Result<(), HillslopeCliError> {
    const FINE_COUNT_ROOT: &str = "frost.runtime_nfine";
    const FINE_FROZEN_DEPTH_ROOT: &str = "frost.runtime_slfsd_m";

    let scalar_frost_depth_m =
        runtime_surface_symbol_value(runtime_surface, "frost.runtime_frdp_m")
            .or_else(|| runtime_surface_symbol_value(runtime_surface, "frost.runtime_dfrost"));
    let mut layers = Vec::with_capacity(nsl);
    for layer_index in 1..=nsl {
        let dg_symbol = format!("wb19_dg_{layer_index:04}");
        let dg_legacy_symbol = wb13_primary_layer_symbol("dg", layer_index);
        let depth_m = runtime_surface_symbol_value(runtime_surface, dg_symbol.as_str())
            .or_else(|| runtime_surface_symbol_value(runtime_surface, dg_legacy_symbol.as_str()))
            .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb11_seed",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} missing required layer depth {dg_symbol}/{dg_legacy_symbol} for fine frost aggregate refresh"
                ),
            })?;

        let fine_count_symbol = wb13_primary_layer_symbol(FINE_COUNT_ROOT, layer_index);
        let fine_frozen_depths_m = if let Some(fine_count_raw) =
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
            let mut fine_frozen_depths_m = Vec::with_capacity(fine_count);
            for fine_index in 1..=fine_count {
                let slfsd_symbol =
                    format!("{FINE_FROZEN_DEPTH_ROOT}_{layer_index:04}_{fine_index:04}");
                let slfsd_m =
                    require_runtime_surface_scalar(runtime_surface, slfsd_symbol.as_str())?;
                fine_frozen_depths_m.push(slfsd_m);
            }
            Some(fine_frozen_depths_m)
        } else {
            None
        };
        layers.push(TypedWb11FrozenDepthLayerInput {
            depth_m,
            fine_frozen_depths_m,
        });
    }
    let projection = project_typed_wb11_frozen_depth_refresh(scalar_frost_depth_m, &layers)?;

    for (layer_offset, frozen_depth_m) in projection.frozen_depths_m.iter().copied().enumerate() {
        let layer_index = layer_offset + 1;
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("wb18_perc_frozen_depth_{layer_index:04}")),
            BoundaryValue::scalar(frozen_depth_m),
        );
    }

    Ok(())
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TypedWb11FrozenDepthLayerInput {
    pub(crate) depth_m: f64,
    pub(crate) fine_frozen_depths_m: Option<Vec<f64>>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TypedWb11FrozenDepthRefreshProjection {
    pub(crate) frozen_depths_m: Vec<f64>,
}

pub(crate) fn project_typed_wb11_frozen_depth_refresh(
    scalar_frost_depth_m: Option<f64>,
    layers: &[TypedWb11FrozenDepthLayerInput],
) -> Result<TypedWb11FrozenDepthRefreshProjection, HillslopeCliError> {
    const ZERO_THRESHOLD: f64 = 1.0e-10;

    if layers.is_empty() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!(
                "{SIMPIPE_GUARD_ID} typed fine frost aggregate refresh requires at least one layer"
            ),
        });
    }
    let mut cumulative_depth_m = 0.0_f64;
    let mut projected_frost_profile_m = Vec::with_capacity(layers.len());
    for (layer_offset, layer) in layers.iter().enumerate() {
        let layer_index = layer_offset + 1;
        if !layer.depth_m.is_finite() || layer.depth_m <= ZERO_THRESHOLD {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb11_seed",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} layer {layer_index} depth for fine frost aggregate refresh must be finite and > 0.0, observed {}",
                    layer.depth_m
                ),
            });
        }

        let computed_frost_extent_m = if let Some(fine_frozen_depths_m) = &layer.fine_frozen_depths_m {
            if fine_frozen_depths_m.is_empty() {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} layer {layer_index} fine frost state must contain at least one fine layer"
                    ),
                });
            }
            let mut fine_frozen_depth_m = 0.0_f64;
            for (fine_offset, slfsd_m) in fine_frozen_depths_m.iter().copied().enumerate() {
                let fine_index = fine_offset + 1;
                if !slfsd_m.is_finite() || slfsd_m < -ZERO_THRESHOLD {
                    return Err(HillslopeCliError::RuntimeSurfaceFailure {
                        surface: "wb11_seed",
                        detail: format!(
                            "{SIMPIPE_GUARD_ID} layer {layer_index} fine frost depth {fine_index} must be finite and >= 0.0, observed {slfsd_m}"
                        ),
                    });
                }
                fine_frozen_depth_m += slfsd_m.max(0.0);
            }
            if fine_frozen_depth_m > layer.depth_m + ZERO_THRESHOLD {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} fine frost depth sum for layer {layer_index} exceeds layer depth ({} > {})",
                        fine_frozen_depth_m, layer.depth_m
                    ),
                });
            }
            fine_frozen_depth_m.min(layer.depth_m)
        } else if let Some(scalar_frost_depth_m) = scalar_frost_depth_m {
            if !scalar_frost_depth_m.is_finite() || scalar_frost_depth_m < -ZERO_THRESHOLD {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} scalar frost depth must be finite and >= 0.0 for aggregate refresh, observed {scalar_frost_depth_m}"
                    ),
                });
            }
            (scalar_frost_depth_m - cumulative_depth_m).clamp(0.0, layer.depth_m)
        } else {
            0.0
        };
        projected_frost_profile_m.push(computed_frost_extent_m);
        cumulative_depth_m += layer.depth_m;
    }

    Ok(TypedWb11FrozenDepthRefreshProjection {
        frozen_depths_m: projected_frost_profile_m,
    })
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
    let Some(input) = read_typed_wb16_ealpha_producer_input(runtime_surface)? else {
        return Ok(None);
    };
    let projection = project_typed_wb16_ealpha_producer(&input)?;
    publish_typed_wb16_ealpha_producer_projection(runtime_surface, &projection);
    Ok(Some(projection.ealpha))
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TypedWb16OfeEalphaInput {
    pub(crate) avgslp: f64,
    pub(crate) slplen: f64,
    pub(crate) inrcov: f64,
    pub(crate) rilcov: f64,
    pub(crate) rrinit: f64,
    pub(crate) rspace: f64,
    pub(crate) width: f64,
    pub(crate) rtyp: f64,
    pub(crate) cancov: f64,
    pub(crate) bb: f64,
    pub(crate) bbb: f64,
    pub(crate) flivmx: f64,
    pub(crate) hmax: f64,
    pub(crate) rrc: Option<f64>,
    pub(crate) canhgt: Option<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TypedWb16EalphaProducerInput {
    pub(crate) exponent_m: f64,
    pub(crate) ofes: Vec<TypedWb16OfeEalphaInput>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TypedWb16OfeEalphaProjection {
    pub(crate) alpha: f64,
    pub(crate) frcteq: f64,
    pub(crate) slplen: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TypedWb16EalphaProducerProjection {
    pub(crate) ealpha: f64,
    pub(crate) ofes: Vec<TypedWb16OfeEalphaProjection>,
}

pub(crate) fn project_typed_wb16_ealpha_producer(
    input: &TypedWb16EalphaProducerInput,
) -> Result<TypedWb16EalphaProducerProjection, HillslopeCliError> {
    if input.ofes.is_empty() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb16_ealpha_producer",
            detail: format!("{SIMPIPE_GUARD_ID} typed WB16 ealpha requires at least one OFE"),
        });
    }
    let powers = wb16_ealpha_powers_from_exponent(input.exponent_m)?;
    let mut ofe_results = Vec::with_capacity(input.ofes.len());
    for (ofe_offset, ofe_input) in input.ofes.iter().copied().enumerate() {
        ofe_results.push(project_typed_wb16_ofe_alpha(ofe_offset + 1, ofe_input)?);
    }
    let legacy_results = ofe_results
        .iter()
        .map(|projection| Wb16OfeAlphaResult {
            alpha: projection.alpha,
            slplen: projection.slplen,
        })
        .collect::<Vec<_>>();
    let ealpha = wb16_equivalent_plane_alpha(&legacy_results, powers)?;
    Ok(TypedWb16EalphaProducerProjection {
        ealpha,
        ofes: ofe_results,
    })
}

fn read_typed_wb16_ealpha_producer_input(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<Option<TypedWb16EalphaProducerInput>, HillslopeCliError> {
    let Some(ofe_count) = wb16_ealpha_ofe_count(runtime_surface)? else {
        return Ok(None);
    };
    let powers = wb16_ealpha_powers(runtime_surface)?;
    let mut ofes = Vec::with_capacity(ofe_count);
    for ofe_index in 1..=ofe_count {
        let Some(geometry) = wb16_ofe_geometry(runtime_surface, ofe_index)? else {
            return Ok(None);
        };
        let Some(surface_controls) = wb16_ofe_surface_controls(runtime_surface, ofe_index) else {
            return Ok(None);
        };
        let Some(canopy_controls) = wb16_ofe_canopy_controls(runtime_surface, ofe_index) else {
            return Ok(None);
        };
        let rrc = wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "rrc")
            .or_else(|| wb16_optional_state_scalar(runtime_surface, "rrc"));
        let canhgt = wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "canhgt")
            .or_else(|| wb16_optional_state_scalar(runtime_surface, "canhgt"));
        ofes.push(TypedWb16OfeEalphaInput {
            avgslp: geometry.avgslp,
            slplen: geometry.slplen,
            inrcov: surface_controls.inrcov,
            rilcov: surface_controls.rilcov,
            rrinit: surface_controls.rrinit,
            rspace: surface_controls.rspace,
            width: surface_controls.width,
            rtyp: surface_controls.rtyp,
            cancov: surface_controls.cancov,
            bb: canopy_controls.bb,
            bbb: canopy_controls.bbb,
            flivmx: canopy_controls.flivmx,
            hmax: canopy_controls.hmax,
            rrc,
            canhgt,
        });
    }
    Ok(Some(TypedWb16EalphaProducerInput {
        exponent_m: powers.m,
        ofes,
    }))
}

fn publish_typed_wb16_ealpha_producer_projection(
    runtime_surface: &mut HillslopeWritebackSurface,
    projection: &TypedWb16EalphaProducerProjection,
) {
    for (ofe_offset, ofe_projection) in projection.ofes.iter().copied().enumerate() {
        let ofe_index = ofe_offset + 1;
        wb16_publish_ofe_frcteq(runtime_surface, ofe_index, ofe_projection.frcteq);
        wb16_publish_ofe_alpha(runtime_surface, ofe_index, ofe_projection.alpha);
    }
    wb16_publish_ealpha(runtime_surface, projection.ealpha);
}

fn project_typed_wb16_ofe_alpha(
    ofe_index: usize,
    input: TypedWb16OfeEalphaInput,
) -> Result<TypedWb16OfeEalphaProjection, HillslopeCliError> {
    let geometry = Wb16OfeGeometry {
        avgslp: input.avgslp,
        slplen: input.slplen,
    };
    if !geometry.avgslp.is_finite() || geometry.avgslp <= 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb16_ealpha_producer",
            detail: format!(
                "{SIMPIPE_GUARD_ID} ofe{ofe_index}_avgslp must be finite and > 0, observed {}",
                geometry.avgslp
            ),
        });
    }
    if !geometry.slplen.is_finite() || geometry.slplen <= 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb16_ealpha_producer",
            detail: format!(
                "{SIMPIPE_GUARD_ID} ofe{ofe_index}_slplen must be finite and > 0, observed {}",
                geometry.slplen
            ),
        });
    }
    let surface_controls = Wb16OfeSurfaceControls {
        inrcov: input.inrcov,
        rilcov: input.rilcov,
        rrinit: input.rrinit,
        rspace: input.rspace,
        width: input.width,
        rtyp: input.rtyp,
        cancov: input.cancov,
    };
    let canopy_controls = Wb16OfeCanopyControls {
        bb: input.bb,
        bbb: input.bbb,
        flivmx: input.flivmx,
        hmax: input.hmax,
    };
    wb16_validate_finite_ofe_values(ofe_index, surface_controls, canopy_controls)?;
    wb16_validate_surface_nonnegative(ofe_index, surface_controls)?;
    wb16_validate_canopy_nonnegative(ofe_index, surface_controls, canopy_controls)?;
    let controls =
        wb16_normalize_typed_ofe_controls(ofe_index, surface_controls, canopy_controls, input)?;
    let frlive = wb16_compute_frlive(ofe_index, controls)?;
    let frcteq = wb16_compute_frcteq(ofe_index, controls, frlive)?;
    let alpha = wb16_compute_ofe_alpha(ofe_index, geometry.avgslp, frcteq)?;
    Ok(TypedWb16OfeEalphaProjection {
        alpha,
        frcteq,
        slplen: geometry.slplen,
    })
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
    wb16_ealpha_powers_from_exponent(m)
}

fn wb16_ealpha_powers_from_exponent(m: f64) -> Result<Wb16EalphaPowers, HillslopeCliError> {
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

fn wb16_normalize_typed_ofe_controls(
    ofe_index: usize,
    surface_controls: Wb16OfeSurfaceControls,
    canopy_controls: Wb16OfeCanopyControls,
    input: TypedWb16OfeEalphaInput,
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
    let rrc = wb16_resolve_typed_rrc(ofe_index, rrinit, input.rrc)?;
    let canhgt = wb16_resolve_typed_canhgt(ofe_index, cancov, canopy_controls, input.canhgt)?;

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

fn wb16_resolve_typed_rrc(
    ofe_index: usize,
    rrinit: f64,
    rrc: Option<f64>,
) -> Result<f64, HillslopeCliError> {
    let rrc = rrc.unwrap_or(rrinit);
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

fn wb16_resolve_typed_canhgt(
    ofe_index: usize,
    cancov: f64,
    controls: Wb16OfeCanopyControls,
    canhgt: Option<f64>,
) -> Result<f64, HillslopeCliError> {
    if let Some(canhgt) = canhgt {
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
