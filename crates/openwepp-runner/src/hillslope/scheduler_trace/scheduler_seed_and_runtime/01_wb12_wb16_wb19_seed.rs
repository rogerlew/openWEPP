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
