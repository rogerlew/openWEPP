/// Build a hillslope-owned climate runtime request from parser output
/// (`HS-CLIM-SEAM-001`).
///
/// Runtime policy at this seam enforces `datver=0.0` override (`iclig=0`) or
/// `datver>=4.0` (`iclig=1`).
///
/// # Errors
///
/// Returns `ClimateRuntimeInputError` when climate parser output violates
/// runtime seam policy or numeric invariants.
pub fn build_hillslope_climate_runtime_request(
    climate: &ClimateFile,
) -> Result<HillslopeClimateRuntimeRequest, ClimateRuntimeInputError> {
    let shared = build_climate_runtime_request(climate)?;
    let mut day_symbol_surfaces = Vec::with_capacity(shared.daily_forcing.len());
    for forcing in &shared.daily_forcing {
        day_symbol_surfaces.push(build_hillslope_series_surface(forcing)?);
    }

    Ok(HillslopeClimateRuntimeRequest {
        shared,
        metadata: climate.metadata.clone(),
        monthly: climate.monthly.clone(),
        day_symbol_surfaces,
    })
}

/// Seed a hillslope runtime writeback surface with one climate forcing record.
///
/// # Errors
///
/// Returns `ClimateRuntimeInputError` when requested day index is invalid or
/// climate forcing cannot be losslessly projected onto runtime symbols.
#[allow(clippy::too_many_lines)]
pub fn seed_hillslope_runtime_surface_from_climate(
    runtime_surface: &mut HillslopeWritebackSurface,
    climate: &HillslopeClimateRuntimeRequest,
    day_index: usize,
) -> Result<(), ClimateRuntimeInputError> {
    let forcing = select_day_forcing(&climate.shared, day_index)?;
    let day_symbols = climate.day_symbol_surfaces.get(day_index).ok_or(
        ClimateRuntimeInputError::DayIndexOutOfRange {
            day_index,
            available: climate.day_symbol_surfaces.len(),
        },
    )?;

    let state_surface = &mut runtime_surface.state_surface;
    state_surface.insert(
        BoundarySymbol::from("datver"),
        BoundaryValue::scalar(climate.shared.datver),
    );
    state_surface.insert(
        BoundarySymbol::from("iclig"),
        BoundaryValue::scalar(f64::from(climate.shared.iclig)),
    );
    state_surface.insert(
        BoundarySymbol::from("itemp"),
        BoundaryValue::scalar(f64::from(climate.shared.itemp)),
    );
    state_surface.insert(
        BoundarySymbol::from("ibrkpt"),
        BoundaryValue::scalar(f64::from(climate.shared.ibrkpt)),
    );
    state_surface.insert(
        BoundarySymbol::from("iwind"),
        BoundaryValue::scalar(f64::from(climate.shared.iwind)),
    );
    insert_monthly_climate_symbols(state_surface, &climate.monthly)?;

    match forcing {
        HillslopeClimateDailyForcing::NoBreakpoint(day) => {
            insert_common_day_symbols(state_surface, day.day, day.mon, day.year);
            state_surface.insert(
                BoundarySymbol::from("prcp"),
                BoundaryValue::scalar(day.prcp),
            );
            state_surface.insert(
                BoundarySymbol::from("stmdur"),
                BoundaryValue::scalar(day.stmdur),
            );
            state_surface.insert(
                BoundarySymbol::from("timep"),
                BoundaryValue::scalar(day.timep),
            );
            state_surface.insert(BoundarySymbol::from("ip"), BoundaryValue::scalar(day.ip));
            let ninten = u32::try_from(day.ninten).map_err(|_| {
                ClimateRuntimeInputError::BreakpointCountOutOfRange { value: day.ninten }
            })?;
            state_surface.insert(
                BoundarySymbol::from("ninten"),
                BoundaryValue::scalar(f64::from(ninten)),
            );
            state_surface.insert(
                BoundarySymbol::from("avrint"),
                BoundaryValue::scalar(day.avrint),
            );
            state_surface.insert(
                BoundarySymbol::from("mxint"),
                BoundaryValue::scalar(day.mxint),
            );
            state_surface.insert(
                BoundarySymbol::from("tmax"),
                BoundaryValue::scalar(day.tmax),
            );
            state_surface.insert(
                BoundarySymbol::from("tmin"),
                BoundaryValue::scalar(day.tmin),
            );
            state_surface.insert(BoundarySymbol::from("rad"), BoundaryValue::scalar(day.rad));
            state_surface.insert(
                BoundarySymbol::from("vwind"),
                BoundaryValue::scalar(day.vwind),
            );
            state_surface.insert(
                BoundarySymbol::from("wind"),
                BoundaryValue::scalar(day.wind),
            );
            state_surface.insert(
                BoundarySymbol::from("tdpt"),
                BoundaryValue::scalar(day.tdpt),
            );
            insert_series_values(state_surface, day_symbols.timem_symbols(), &day.timem);
            insert_series_values(state_surface, day_symbols.intsty_symbols(), &day.intsty);
        }
        HillslopeClimateDailyForcing::Breakpoint(day) => {
            insert_common_day_symbols(state_surface, day.day, day.mon, day.year);
            state_surface.insert(
                BoundarySymbol::from("stmstr"),
                BoundaryValue::scalar(day.stmstr),
            );
            state_surface.insert(
                BoundarySymbol::from("prcp"),
                BoundaryValue::scalar(day.prcp),
            );
            state_surface.insert(
                BoundarySymbol::from("stmdur"),
                BoundaryValue::scalar(day.stmdur),
            );
            state_surface.insert(
                BoundarySymbol::from("mxint"),
                BoundaryValue::scalar(day.mxint),
            );
            state_surface.insert(
                BoundarySymbol::from("tmax"),
                BoundaryValue::scalar(day.tmax),
            );
            state_surface.insert(
                BoundarySymbol::from("tmin"),
                BoundaryValue::scalar(day.tmin),
            );
            state_surface.insert(BoundarySymbol::from("rad"), BoundaryValue::scalar(day.rad));
            state_surface.insert(
                BoundarySymbol::from("vwind"),
                BoundaryValue::scalar(day.vwind),
            );
            state_surface.insert(
                BoundarySymbol::from("wind"),
                BoundaryValue::scalar(day.wind),
            );
            state_surface.insert(
                BoundarySymbol::from("tdpt"),
                BoundaryValue::scalar(day.tdpt),
            );

            let nbrkpt = u32::try_from(day.nbrkpt).map_err(|_| {
                ClimateRuntimeInputError::BreakpointCountOutOfRange { value: day.nbrkpt }
            })?;
            state_surface.insert(
                BoundarySymbol::from("nbrkpt"),
                BoundaryValue::scalar(f64::from(nbrkpt)),
            );

            insert_series_values(state_surface, day_symbols.timem_symbols(), &day.timem);
            insert_series_values(state_surface, day_symbols.intsty_symbols(), &day.intsty);
        }
    }

    Ok(())
}

/// Seed a hillslope runtime writeback surface with one climate forcing record
/// and optional static context for SIMIMPL28 hourly winter forcing synthesis.
///
/// # Errors
///
/// Returns `ClimateRuntimeInputError` when base climate projection fails or
/// active winter-forcing synthesis encounters missing/non-finite/out-of-domain
/// required context symbols.
pub fn seed_hillslope_runtime_surface_from_climate_with_context(
    runtime_surface: &mut HillslopeWritebackSurface,
    climate: &HillslopeClimateRuntimeRequest,
    day_index: usize,
    winter_context_state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
) -> Result<(), ClimateRuntimeInputError> {
    seed_hillslope_runtime_surface_from_climate(runtime_surface, climate, day_index)?;
    let forcing = select_day_forcing(&climate.shared, day_index)?;
    let winter_symbols = build_simimpl28_hourly_winter_forcing_symbols(
        forcing,
        &climate.metadata,
        winter_context_state_surface,
    )?;
    runtime_surface.state_surface.extend(winter_symbols);
    Ok(())
}

/// Build a hillslope runtime surface from climate parser output and selected day
/// index.
///
/// # Errors
///
/// Returns `ClimateRuntimeInputError` when climate runtime request projection
/// fails.
pub fn build_hillslope_runtime_surface_from_climate(
    climate: &ClimateFile,
    day_index: usize,
) -> Result<HillslopeWritebackSurface, ClimateRuntimeInputError> {
    let request = build_hillslope_climate_runtime_request(climate)?;
    let mut surface = HillslopeWritebackSurface::default();
    seed_hillslope_runtime_surface_from_climate(&mut surface, &request, day_index)?;
    Ok(surface)
}

/// Build a hillslope runtime surface from climate parser output and selected
/// day index, using static runtime context for SIMIMPL28 hourly winter forcing
/// synthesis.
///
/// # Errors
///
/// Returns `ClimateRuntimeInputError` when climate runtime request projection
/// fails or when active winter forcing synthesis context is invalid.
pub fn build_hillslope_runtime_surface_from_climate_with_context(
    climate: &ClimateFile,
    day_index: usize,
    winter_context_state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
) -> Result<HillslopeWritebackSurface, ClimateRuntimeInputError> {
    let request = build_hillslope_climate_runtime_request(climate)?;
    build_hillslope_runtime_surface_from_climate_request_with_context(
        &request,
        day_index,
        winter_context_state_surface,
    )
}

/// Build a hillslope runtime surface from a precomputed climate runtime request
/// and selected day index, using static runtime context for SIMIMPL28 hourly
/// winter forcing synthesis.
///
/// # Errors
///
/// Returns `ClimateRuntimeInputError` when day selection fails or active winter
/// forcing synthesis context is invalid.
pub fn build_hillslope_runtime_surface_from_climate_request_with_context(
    climate: &HillslopeClimateRuntimeRequest,
    day_index: usize,
    winter_context_state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
) -> Result<HillslopeWritebackSurface, ClimateRuntimeInputError> {
    let mut surface = HillslopeWritebackSurface::default();
    seed_hillslope_runtime_surface_from_climate_with_context(
        &mut surface,
        climate,
        day_index,
        winter_context_state_surface,
    )?;
    Ok(surface)
}

