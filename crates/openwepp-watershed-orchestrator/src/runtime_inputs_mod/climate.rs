use std::collections::BTreeMap;

use super::types::{
    WatershedClimateRuntimeInputError, WatershedClimateRuntimeRequest,
    WatershedHillslopeClimateAssignment,
};
use crate::WatershedWritebackSurface;
use openwepp_climate_runtime_adapter::{
    SharedClimateDailyForcing as WatershedClimateDailyForcing, SharedClimateRuntimeInputError,
    build_climate_runtime_request, select_day_forcing,
};
use openwepp_input_contract::parsers::climate::{ClimateFile, ClimateMonthlyStats};
use openwepp_kernel_contract::{
    BoundaryError, BoundarySymbol, BoundaryValue, ClimateForcingSymbolSurface,
    ClimateForcingSymbolSurfaceError,
};

#[allow(clippy::missing_errors_doc)]
pub fn build_watershed_climate_runtime_request_from_assignments(
    assignments: &BTreeMap<u32, ClimateFile>,
) -> Result<WatershedClimateRuntimeRequest, WatershedClimateRuntimeInputError> {
    if assignments.is_empty() {
        return Err(WatershedClimateRuntimeInputError::EmptyClimateAssignments);
    }

    let mut hillslope_forcing = BTreeMap::new();
    for (&hillslope_id, climate) in assignments {
        let forcing = build_climate_runtime_request(climate)
            .map_err(|error| map_shared_error_for_hillslope(hillslope_id, &error))?;
        let mut day_symbol_surfaces = Vec::with_capacity(forcing.daily_forcing.len());
        for daily_forcing in &forcing.daily_forcing {
            day_symbol_surfaces.push(build_watershed_series_surface(hillslope_id, daily_forcing)?);
        }
        hillslope_forcing.insert(
            hillslope_id,
            WatershedHillslopeClimateAssignment::new(
                forcing,
                climate.metadata.clone(),
                climate.monthly.clone(),
                day_symbol_surfaces,
            ),
        );
    }

    Ok(WatershedClimateRuntimeRequest { hillslope_forcing })
}

/// Seed a watershed runtime surface with climate assignments for one day index.
///
/// # Errors
///
/// Returns `WatershedClimateRuntimeInputError` when the requested day index is
/// invalid for any assigned hillslope or projection fails.
#[allow(clippy::too_many_lines)]
pub fn seed_watershed_runtime_surface_from_climate(
    runtime_surface: &mut WatershedWritebackSurface,
    climate: &WatershedClimateRuntimeRequest,
    day_index: usize,
) -> Result<(), WatershedClimateRuntimeInputError> {
    let assignment_count = u32::try_from(climate.hillslope_forcing.len()).map_err(|_| {
        WatershedClimateRuntimeInputError::BreakpointCountOutOfRange {
            hillslope_id: 0,
            value: climate.hillslope_forcing.len(),
        }
    })?;

    let state_surface = &mut runtime_surface.state_surface;
    state_surface.insert(
        BoundarySymbol::from("nclimhs"),
        BoundaryValue::scalar(f64::from(assignment_count)),
    );

    for (&hillslope_id, assignment) in &climate.hillslope_forcing {
        let request = assignment.forcing();
        insert_hillslope_symbol(state_surface, hillslope_id, "datver", request.datver);
        insert_hillslope_symbol(
            state_surface,
            hillslope_id,
            "iclig",
            f64::from(request.iclig),
        );
        insert_hillslope_symbol(
            state_surface,
            hillslope_id,
            "itemp",
            f64::from(request.itemp),
        );
        insert_hillslope_symbol(
            state_surface,
            hillslope_id,
            "ibrkpt",
            f64::from(request.ibrkpt),
        );
        insert_hillslope_symbol(
            state_surface,
            hillslope_id,
            "iwind",
            f64::from(request.iwind),
        );
        insert_hillslope_symbol(
            state_surface,
            hillslope_id,
            "deglat",
            assignment.metadata().deglat,
        );
        insert_hillslope_symbol(
            state_surface,
            hillslope_id,
            "elevm",
            assignment.metadata().elev,
        );
        insert_hillslope_monthly_climate_symbols(
            state_surface,
            hillslope_id,
            assignment.monthly(),
        )?;

        let forcing = select_day_forcing(request, day_index)
            .map_err(|error| map_shared_error_for_hillslope(hillslope_id, &error))?;
        let day_symbols = assignment.day_symbol_surfaces().get(day_index).ok_or(
            WatershedClimateRuntimeInputError::DayIndexOutOfRange {
                hillslope_id,
                day_index,
                available: assignment.day_symbol_surfaces().len(),
            },
        )?;

        match forcing {
            WatershedClimateDailyForcing::NoBreakpoint(day) => {
                insert_hillslope_common_day_symbols(
                    state_surface,
                    hillslope_id,
                    day.day,
                    day.mon,
                    day.year,
                );
                insert_typed_hillslope_symbol(
                    state_surface,
                    hillslope_id,
                    "prcp",
                    day.prcp,
                    ">= 0",
                    BoundaryValue::water_depth_meters,
                )?;
                insert_typed_hillslope_symbol(
                    state_surface,
                    hillslope_id,
                    "stmdur",
                    day.stmdur,
                    ">= 0",
                    BoundaryValue::elapsed_time_seconds,
                )?;
                insert_hillslope_symbol(state_surface, hillslope_id, "timep", day.timep);
                insert_hillslope_symbol(state_surface, hillslope_id, "ip", day.ip);
                let ninten = u32::try_from(day.ninten).map_err(|_| {
                    WatershedClimateRuntimeInputError::BreakpointCountOutOfRange {
                        hillslope_id,
                        value: day.ninten,
                    }
                })?;
                insert_hillslope_symbol(state_surface, hillslope_id, "ninten", f64::from(ninten));
                insert_typed_hillslope_symbol(
                    state_surface,
                    hillslope_id,
                    "avrint",
                    day.avrint,
                    ">= 0",
                    BoundaryValue::linear_rate_meters_per_second,
                )?;
                insert_typed_hillslope_symbol(
                    state_surface,
                    hillslope_id,
                    "mxint",
                    day.mxint,
                    ">= 0",
                    BoundaryValue::linear_rate_meters_per_second,
                )?;
                insert_typed_hillslope_symbol(
                    state_surface,
                    hillslope_id,
                    "tmax",
                    day.tmax,
                    "finite",
                    BoundaryValue::temperature_celsius,
                )?;
                insert_typed_hillslope_symbol(
                    state_surface,
                    hillslope_id,
                    "tmin",
                    day.tmin,
                    "finite",
                    BoundaryValue::temperature_celsius,
                )?;
                insert_typed_hillslope_symbol(
                    state_surface,
                    hillslope_id,
                    "rad",
                    day.rad,
                    ">= 0",
                    BoundaryValue::solar_radiation_langleys_per_day,
                )?;
                insert_typed_hillslope_symbol(
                    state_surface,
                    hillslope_id,
                    "vwind",
                    day.vwind,
                    ">= 0",
                    BoundaryValue::linear_rate_meters_per_second,
                )?;
                insert_typed_hillslope_symbol(
                    state_surface,
                    hillslope_id,
                    "wind",
                    day.wind,
                    "0..=360",
                    BoundaryValue::direction_degrees,
                )?;
                insert_typed_hillslope_symbol(
                    state_surface,
                    hillslope_id,
                    "tdpt",
                    day.tdpt,
                    "finite",
                    BoundaryValue::temperature_celsius,
                )?;
                insert_typed_series_values(
                    state_surface,
                    hillslope_id,
                    day_symbols.timem_symbols(),
                    &day.timem,
                    "timem_*",
                    ">= 0",
                    BoundaryValue::elapsed_time_seconds,
                )?;
                insert_typed_series_values(
                    state_surface,
                    hillslope_id,
                    day_symbols.intsty_symbols(),
                    &day.intsty,
                    "intsty_*",
                    ">= 0",
                    BoundaryValue::linear_rate_meters_per_second,
                )?;
            }
            WatershedClimateDailyForcing::Breakpoint(day) => {
                insert_hillslope_common_day_symbols(
                    state_surface,
                    hillslope_id,
                    day.day,
                    day.mon,
                    day.year,
                );
                insert_typed_hillslope_symbol(
                    state_surface,
                    hillslope_id,
                    "stmstr",
                    day.stmstr,
                    "0..=24",
                    BoundaryValue::hour_of_day,
                )?;
                insert_typed_hillslope_symbol(
                    state_surface,
                    hillslope_id,
                    "prcp",
                    day.prcp,
                    ">= 0",
                    BoundaryValue::water_depth_meters,
                )?;
                insert_typed_hillslope_symbol(
                    state_surface,
                    hillslope_id,
                    "stmdur",
                    day.stmdur,
                    ">= 0",
                    BoundaryValue::elapsed_time_seconds,
                )?;
                insert_typed_hillslope_symbol(
                    state_surface,
                    hillslope_id,
                    "mxint",
                    day.mxint,
                    ">= 0",
                    BoundaryValue::linear_rate_meters_per_second,
                )?;
                insert_typed_hillslope_symbol(
                    state_surface,
                    hillslope_id,
                    "tmax",
                    day.tmax,
                    "finite",
                    BoundaryValue::temperature_celsius,
                )?;
                insert_typed_hillslope_symbol(
                    state_surface,
                    hillslope_id,
                    "tmin",
                    day.tmin,
                    "finite",
                    BoundaryValue::temperature_celsius,
                )?;
                insert_typed_hillslope_symbol(
                    state_surface,
                    hillslope_id,
                    "rad",
                    day.rad,
                    ">= 0",
                    BoundaryValue::solar_radiation_langleys_per_day,
                )?;
                insert_typed_hillslope_symbol(
                    state_surface,
                    hillslope_id,
                    "vwind",
                    day.vwind,
                    ">= 0",
                    BoundaryValue::linear_rate_meters_per_second,
                )?;
                insert_typed_hillslope_symbol(
                    state_surface,
                    hillslope_id,
                    "wind",
                    day.wind,
                    "0..=360",
                    BoundaryValue::direction_degrees,
                )?;
                insert_typed_hillslope_symbol(
                    state_surface,
                    hillslope_id,
                    "tdpt",
                    day.tdpt,
                    "finite",
                    BoundaryValue::temperature_celsius,
                )?;

                let nbrkpt = u32::try_from(day.nbrkpt).map_err(|_| {
                    WatershedClimateRuntimeInputError::BreakpointCountOutOfRange {
                        hillslope_id,
                        value: day.nbrkpt,
                    }
                })?;
                insert_hillslope_symbol(state_surface, hillslope_id, "nbrkpt", f64::from(nbrkpt));

                insert_typed_series_values(
                    state_surface,
                    hillslope_id,
                    day_symbols.timem_symbols(),
                    &day.timem,
                    "timem_*",
                    ">= 0",
                    BoundaryValue::elapsed_time_seconds,
                )?;
                insert_typed_series_values(
                    state_surface,
                    hillslope_id,
                    day_symbols.intsty_symbols(),
                    &day.intsty,
                    "intsty_*",
                    ">= 0",
                    BoundaryValue::linear_rate_meters_per_second,
                )?;
            }
        }
    }

    Ok(())
}

/// Build a watershed runtime surface directly from per-hillslope climate parser
/// outputs and selected day index.
///
/// # Errors
///
/// Returns `WatershedClimateRuntimeInputError` when climate assignment seam
/// projection fails.
pub fn build_watershed_runtime_surface_from_climate_assignments(
    assignments: &BTreeMap<u32, ClimateFile>,
    day_index: usize,
) -> Result<WatershedWritebackSurface, WatershedClimateRuntimeInputError> {
    let request = build_watershed_climate_runtime_request_from_assignments(assignments)?;
    let mut surface = WatershedWritebackSurface::default();
    seed_watershed_runtime_surface_from_climate(&mut surface, &request, day_index)?;
    Ok(surface)
}

#[allow(clippy::too_many_lines)]
fn map_shared_error_for_hillslope(
    hillslope_id: u32,
    error: &SharedClimateRuntimeInputError,
) -> WatershedClimateRuntimeInputError {
    match error {
        SharedClimateRuntimeInputError::UnsupportedDatver { datver } => {
            WatershedClimateRuntimeInputError::UnsupportedDatver { datver: *datver }
        }
        SharedClimateRuntimeInputError::UnsupportedItemp { itemp } => {
            WatershedClimateRuntimeInputError::UnsupportedItemp { itemp: *itemp }
        }
        SharedClimateRuntimeInputError::EmptyDailyRecords => {
            WatershedClimateRuntimeInputError::EmptyDailyRecords { hillslope_id }
        }
        SharedClimateRuntimeInputError::DayIndexOutOfRange {
            day_index,
            available,
        } => WatershedClimateRuntimeInputError::DayIndexOutOfRange {
            hillslope_id,
            day_index: *day_index,
            available: *available,
        },
        SharedClimateRuntimeInputError::NonFiniteField { field, value } => {
            WatershedClimateRuntimeInputError::NonFiniteField {
                field,
                value: *value,
            }
        }
        SharedClimateRuntimeInputError::NegativeField { field, value } => {
            WatershedClimateRuntimeInputError::NegativeField {
                field,
                value: *value,
            }
        }
        SharedClimateRuntimeInputError::PositivePrecipWithNonPositiveDuration { prcp, stmdur } => {
            WatershedClimateRuntimeInputError::PositivePrecipWithNonPositiveDuration {
                hillslope_id,
                prcp: *prcp,
                stmdur: *stmdur,
            }
        }
        SharedClimateRuntimeInputError::EmptyBreakpointSeries => {
            WatershedClimateRuntimeInputError::EmptyBreakpointSeries { hillslope_id }
        }
        SharedClimateRuntimeInputError::NonMonotoneBreakpointTime {
            previous_s,
            current_s,
        } => WatershedClimateRuntimeInputError::NonMonotoneBreakpointTime {
            hillslope_id,
            previous_s: *previous_s,
            current_s: *current_s,
        },
        SharedClimateRuntimeInputError::BreakpointCardinalityPolicyExceeded { value, max } => {
            WatershedClimateRuntimeInputError::BreakpointCardinalityPolicyExceeded {
                hillslope_id,
                value: *value,
                max: *max,
            }
        }
        SharedClimateRuntimeInputError::BreakpointCountOutOfRange { value } => {
            WatershedClimateRuntimeInputError::BreakpointCountOutOfRange {
                hillslope_id,
                value: *value,
            }
        }
        SharedClimateRuntimeInputError::DisaggregationTimeNotStrictlyIncreasing {
            previous_s,
            current_s,
        } => WatershedClimateRuntimeInputError::DisaggregationTimeNotStrictlyIncreasing {
            hillslope_id,
            previous_s: *previous_s,
            current_s: *current_s,
        },
        SharedClimateRuntimeInputError::DisaggregationRootSolveDomain { a } => {
            WatershedClimateRuntimeInputError::DisaggregationRootSolveDomain {
                hillslope_id,
                a: *a,
            }
        }
        SharedClimateRuntimeInputError::DisaggregationRootSolveNonConvergent { a } => {
            WatershedClimateRuntimeInputError::DisaggregationRootSolveNonConvergent {
                hillslope_id,
                a: *a,
            }
        }
        SharedClimateRuntimeInputError::DisaggregationClosureResidual {
            expected_prcp_m,
            reconstructed_prcp_m,
        } => WatershedClimateRuntimeInputError::DisaggregationClosureResidual {
            hillslope_id,
            expected_prcp_m: *expected_prcp_m,
            reconstructed_prcp_m: *reconstructed_prcp_m,
        },
        SharedClimateRuntimeInputError::MissingRuntimeContextSymbol { symbol } => {
            WatershedClimateRuntimeInputError::MissingRuntimeContextSymbol {
                hillslope_id,
                symbol: symbol.clone(),
            }
        }
        SharedClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
            symbol,
            value,
            allowed,
        } => WatershedClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
            hillslope_id,
            symbol: symbol.clone(),
            value: *value,
            allowed,
        },
        SharedClimateRuntimeInputError::InvalidCalendarDate { day, mon, year } => {
            WatershedClimateRuntimeInputError::InvalidCalendarDate {
                hillslope_id,
                day: *day,
                mon: *mon,
                year: *year,
            }
        }
    }
}

fn build_watershed_series_surface(
    hillslope_id: u32,
    forcing: &WatershedClimateDailyForcing,
) -> Result<ClimateForcingSymbolSurface, WatershedClimateRuntimeInputError> {
    let point_count = forcing_series_point_count(forcing);
    ClimateForcingSymbolSurface::watershed_hillslope(hillslope_id, point_count)
        .map_err(|error| map_surface_build_error(hillslope_id, &error))
}

fn forcing_series_point_count(forcing: &WatershedClimateDailyForcing) -> usize {
    match forcing {
        WatershedClimateDailyForcing::NoBreakpoint(day) => day.timem.len(),
        WatershedClimateDailyForcing::Breakpoint(day) => day.timem.len(),
    }
}

fn map_surface_build_error(
    hillslope_id: u32,
    error: &ClimateForcingSymbolSurfaceError,
) -> WatershedClimateRuntimeInputError {
    match error {
        ClimateForcingSymbolSurfaceError::PointCountOutOfRange {
            count,
            supported_max,
        } => WatershedClimateRuntimeInputError::BreakpointCardinalityPolicyExceeded {
            hillslope_id,
            value: *count,
            max: *supported_max,
        },
    }
}

fn insert_hillslope_common_day_symbols(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    hillslope_id: u32,
    day: i32,
    mon: i32,
    year: i32,
) {
    insert_hillslope_symbol(surface, hillslope_id, "day", f64::from(day));
    insert_hillslope_symbol(surface, hillslope_id, "mon", f64::from(mon));
    insert_hillslope_symbol(surface, hillslope_id, "year", f64::from(year));
}

fn insert_hillslope_symbol(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    hillslope_id: u32,
    symbol: &str,
    value: f64,
) {
    let key = format!("hs{hillslope_id}_{symbol}");
    surface.insert(BoundarySymbol::from(key), BoundaryValue::scalar(value));
}

fn insert_typed_hillslope_symbol(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    hillslope_id: u32,
    symbol: &str,
    value: f64,
    allowed: &'static str,
    constructor: fn(f64) -> Result<BoundaryValue, BoundaryError>,
) -> Result<(), WatershedClimateRuntimeInputError> {
    let key = format!("hs{hillslope_id}_{symbol}");
    let boundary_value = constructor(value).map_err(|_| {
        WatershedClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
            hillslope_id,
            symbol: key.clone(),
            value,
            allowed,
        }
    })?;
    surface.insert(BoundarySymbol::from(key), boundary_value);
    Ok(())
}

fn insert_hillslope_monthly_climate_symbols(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    hillslope_id: u32,
    monthly: &ClimateMonthlyStats,
) -> Result<(), WatershedClimateRuntimeInputError> {
    insert_hillslope_monthly_vector_symbols(
        surface,
        hillslope_id,
        "obmaxt",
        "obmaxt[*]",
        &monthly.obmaxt,
    )?;
    insert_hillslope_monthly_vector_symbols(
        surface,
        hillslope_id,
        "obmint",
        "obmint[*]",
        &monthly.obmint,
    )?;
    insert_hillslope_monthly_vector_symbols(
        surface,
        hillslope_id,
        "radave",
        "radave[*]",
        &monthly.radave,
    )?;
    insert_hillslope_monthly_vector_symbols(
        surface,
        hillslope_id,
        "obrain",
        "obrain[*]",
        &monthly.obrain,
    )?;
    Ok(())
}

fn insert_hillslope_monthly_vector_symbols(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    hillslope_id: u32,
    root: &str,
    field: &'static str,
    values: &[f64; 12],
) -> Result<(), WatershedClimateRuntimeInputError> {
    for (index, value) in values.iter().enumerate() {
        if !value.is_finite() {
            return Err(WatershedClimateRuntimeInputError::NonFiniteField {
                field,
                value: *value,
            });
        }
        let month = index + 1;
        insert_hillslope_symbol(surface, hillslope_id, &format!("{root}_{month:04}"), *value);
    }
    Ok(())
}

fn insert_typed_series_values(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    hillslope_id: u32,
    symbols: &[BoundarySymbol],
    values: &[f64],
    root: &'static str,
    allowed: &'static str,
    constructor: fn(f64) -> Result<BoundaryValue, BoundaryError>,
) -> Result<(), WatershedClimateRuntimeInputError> {
    debug_assert_eq!(symbols.len(), values.len());
    for (symbol, value) in symbols.iter().zip(values.iter()) {
        let boundary_value = constructor(*value).map_err(|_| {
            WatershedClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
                hillslope_id,
                symbol: format!("{root}:{}", symbol.as_str()),
                value: *value,
                allowed,
            }
        })?;
        surface.insert(symbol.clone(), boundary_value);
    }
    Ok(())
}
