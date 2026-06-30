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

#[derive(Debug, Clone, PartialEq)]
pub struct HillslopeDirectClimateDayForcing {
    pub prcp_m: f64,
    pub tmax_c: f64,
    pub tmin_c: f64,
    pub rad_ly: f64,
    pub vwind_m_s: f64,
    pub wind_deg: f64,
    pub tdpt_c: f64,
    pub timem_s: Vec<f64>,
    pub intsty_m_s: Vec<f64>,
}

impl HillslopeClimateRuntimeRequest {
    /// Borrow typed forcing for direct-runtime production inputs without
    /// materializing climate symbols onto a writeback surface.
    ///
    /// # Errors
    ///
    /// Returns `ClimateRuntimeInputError` when `day_index` exceeds the
    /// precomputed climate forcing span.
    pub fn direct_day_forcing(
        &self,
        day_index: usize,
    ) -> Result<HillslopeDirectClimateDayForcing, ClimateRuntimeInputError> {
        match select_day_forcing(&self.shared, day_index)? {
            HillslopeClimateDailyForcing::NoBreakpoint(day) => {
                Ok(HillslopeDirectClimateDayForcing {
                    prcp_m: day.prcp,
                    tmax_c: day.tmax,
                    tmin_c: day.tmin,
                    rad_ly: day.rad,
                    vwind_m_s: day.vwind,
                    wind_deg: day.wind,
                    tdpt_c: day.tdpt,
                    timem_s: day.timem.clone(),
                    intsty_m_s: day.intsty.clone(),
                })
            }
            HillslopeClimateDailyForcing::Breakpoint(day) => {
                Ok(HillslopeDirectClimateDayForcing {
                    prcp_m: day.prcp,
                    tmax_c: day.tmax,
                    tmin_c: day.tmin,
                    rad_ly: day.rad,
                    vwind_m_s: day.vwind,
                    wind_deg: day.wind,
                    tdpt_c: day.tdpt,
                    timem_s: day.timem.clone(),
                    intsty_m_s: day.intsty.clone(),
                })
            }
        }
    }

    #[must_use]
    pub const fn direct_latitude_degrees(&self) -> f64 {
        self.metadata.deglat
    }

    #[must_use]
    pub const fn direct_elevation_m(&self) -> f64 {
        self.metadata.elev
    }

    #[must_use]
    pub const fn direct_monthly_max_c(&self) -> [f64; 12] {
        self.monthly.obmaxt
    }

    #[must_use]
    pub const fn direct_monthly_min_c(&self) -> [f64; 12] {
        self.monthly.obmint
    }
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
    state_surface.insert(
        BoundarySymbol::from("deglat"),
        BoundaryValue::scalar(climate.metadata.deglat),
    );
    state_surface.insert(
        BoundarySymbol::from("elevm"),
        BoundaryValue::scalar(climate.metadata.elev),
    );
    insert_monthly_climate_symbols(state_surface, &climate.monthly)?;

    match forcing {
        HillslopeClimateDailyForcing::NoBreakpoint(day) => {
            insert_common_day_symbols(state_surface, day.day, day.mon, day.year);
            state_surface.insert(
                BoundarySymbol::from("prcp"),
                climate_boundary_value("prcp", ">= 0", BoundaryValue::water_depth_meters(day.prcp))?,
            );
            state_surface.insert(
                BoundarySymbol::from("stmdur"),
                climate_boundary_value(
                    "stmdur",
                    ">= 0",
                    BoundaryValue::elapsed_time_seconds(day.stmdur),
                )?,
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
                climate_boundary_value(
                    "avrint",
                    ">= 0",
                    BoundaryValue::linear_rate_meters_per_second(day.avrint),
                )?,
            );
            state_surface.insert(
                BoundarySymbol::from("mxint"),
                climate_boundary_value(
                    "mxint",
                    ">= 0",
                    BoundaryValue::linear_rate_meters_per_second(day.mxint),
                )?,
            );
            state_surface.insert(
                BoundarySymbol::from("tmax"),
                climate_boundary_value("tmax", "finite", BoundaryValue::temperature_celsius(day.tmax))?,
            );
            state_surface.insert(
                BoundarySymbol::from("tmin"),
                climate_boundary_value("tmin", "finite", BoundaryValue::temperature_celsius(day.tmin))?,
            );
            state_surface.insert(
                BoundarySymbol::from("rad"),
                climate_boundary_value(
                    "rad",
                    ">= 0",
                    BoundaryValue::solar_radiation_langleys_per_day(day.rad),
                )?,
            );
            state_surface.insert(
                BoundarySymbol::from("vwind"),
                climate_boundary_value(
                    "vwind",
                    ">= 0",
                    BoundaryValue::linear_rate_meters_per_second(day.vwind),
                )?,
            );
            state_surface.insert(
                BoundarySymbol::from("wind"),
                climate_boundary_value(
                    "wind",
                    "0..=360",
                    BoundaryValue::direction_degrees(day.wind),
                )?,
            );
            state_surface.insert(
                BoundarySymbol::from("tdpt"),
                climate_boundary_value("tdpt", "finite", BoundaryValue::temperature_celsius(day.tdpt))?,
            );
            insert_typed_series_values(
                state_surface,
                day_symbols.timem_symbols(),
                &day.timem,
                "timem_*",
                ">= 0",
                BoundaryValue::elapsed_time_seconds,
            )?;
            insert_typed_series_values(
                state_surface,
                day_symbols.intsty_symbols(),
                &day.intsty,
                "intsty_*",
                ">= 0",
                BoundaryValue::linear_rate_meters_per_second,
            )?;
        }
        HillslopeClimateDailyForcing::Breakpoint(day) => {
            insert_common_day_symbols(state_surface, day.day, day.mon, day.year);
            state_surface.insert(
                BoundarySymbol::from("stmstr"),
                climate_boundary_value("stmstr", "0..=24", BoundaryValue::hour_of_day(day.stmstr))?,
            );
            state_surface.insert(
                BoundarySymbol::from("prcp"),
                climate_boundary_value("prcp", ">= 0", BoundaryValue::water_depth_meters(day.prcp))?,
            );
            state_surface.insert(
                BoundarySymbol::from("stmdur"),
                climate_boundary_value(
                    "stmdur",
                    ">= 0",
                    BoundaryValue::elapsed_time_seconds(day.stmdur),
                )?,
            );
            state_surface.insert(
                BoundarySymbol::from("mxint"),
                climate_boundary_value(
                    "mxint",
                    ">= 0",
                    BoundaryValue::linear_rate_meters_per_second(day.mxint),
                )?,
            );
            state_surface.insert(
                BoundarySymbol::from("tmax"),
                climate_boundary_value("tmax", "finite", BoundaryValue::temperature_celsius(day.tmax))?,
            );
            state_surface.insert(
                BoundarySymbol::from("tmin"),
                climate_boundary_value("tmin", "finite", BoundaryValue::temperature_celsius(day.tmin))?,
            );
            state_surface.insert(
                BoundarySymbol::from("rad"),
                climate_boundary_value(
                    "rad",
                    ">= 0",
                    BoundaryValue::solar_radiation_langleys_per_day(day.rad),
                )?,
            );
            state_surface.insert(
                BoundarySymbol::from("vwind"),
                climate_boundary_value(
                    "vwind",
                    ">= 0",
                    BoundaryValue::linear_rate_meters_per_second(day.vwind),
                )?,
            );
            state_surface.insert(
                BoundarySymbol::from("wind"),
                climate_boundary_value(
                    "wind",
                    "0..=360",
                    BoundaryValue::direction_degrees(day.wind),
                )?,
            );
            state_surface.insert(
                BoundarySymbol::from("tdpt"),
                climate_boundary_value("tdpt", "finite", BoundaryValue::temperature_celsius(day.tdpt))?,
            );

            let nbrkpt = u32::try_from(day.nbrkpt).map_err(|_| {
                ClimateRuntimeInputError::BreakpointCountOutOfRange { value: day.nbrkpt }
            })?;
            state_surface.insert(
                BoundarySymbol::from("nbrkpt"),
                BoundaryValue::scalar(f64::from(nbrkpt)),
            );

            insert_typed_series_values(
                state_surface,
                day_symbols.timem_symbols(),
                &day.timem,
                "timem_*",
                ">= 0",
                BoundaryValue::elapsed_time_seconds,
            )?;
            insert_typed_series_values(
                state_surface,
                day_symbols.intsty_symbols(),
                &day.intsty,
                "intsty_*",
                ">= 0",
                BoundaryValue::linear_rate_meters_per_second,
            )?;
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
