fn validate_irrigation_finite(
    field: &'static str,
    value: f64,
) -> Result<f64, HillslopeRuntimeInputError> {
    if !value.is_finite() {
        return Err(HillslopeRuntimeInputError::NonFiniteIrrigationScheduleField { field, value });
    }
    Ok(value)
}

fn build_hillslope_series_surface(
    forcing: &HillslopeClimateDailyForcing,
) -> Result<ClimateForcingSymbolSurface, ClimateRuntimeInputError> {
    let point_count = forcing_series_point_count(forcing);
    ClimateForcingSymbolSurface::hillslope(point_count)
        .map_err(|error| map_surface_build_error(&error))
}

fn forcing_series_point_count(forcing: &HillslopeClimateDailyForcing) -> usize {
    match forcing {
        HillslopeClimateDailyForcing::NoBreakpoint(day) => day.timem.len(),
        HillslopeClimateDailyForcing::Breakpoint(day) => day.timem.len(),
    }
}

fn map_surface_build_error(error: &ClimateForcingSymbolSurfaceError) -> ClimateRuntimeInputError {
    match error {
        ClimateForcingSymbolSurfaceError::PointCountOutOfRange {
            count,
            supported_max,
        } => ClimateRuntimeInputError::BreakpointCardinalityPolicyExceeded {
            value: *count,
            max: *supported_max,
        },
    }
}

fn insert_typed_series_values(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    symbols: &[BoundarySymbol],
    values: &[f64],
    field: &'static str,
    allowed: &'static str,
    constructor: fn(f64) -> Result<BoundaryValue, BoundaryError>,
) -> Result<(), ClimateRuntimeInputError> {
    debug_assert_eq!(symbols.len(), values.len());
    for (symbol, value) in symbols.iter().zip(values.iter()) {
        let typed_value = climate_boundary_value(field, allowed, constructor(*value))?;
        surface.insert(symbol.clone(), typed_value);
    }
    Ok(())
}

fn climate_boundary_value(
    field: &'static str,
    allowed: &'static str,
    value: Result<BoundaryValue, BoundaryError>,
) -> Result<BoundaryValue, ClimateRuntimeInputError> {
    value.map_err(|error| match error {
        BoundaryError::NonFinite { value, .. } => {
            ClimateRuntimeInputError::NonFiniteField { field, value }
        }
        BoundaryError::BelowMinimum { value, .. } | BoundaryError::AboveMaximum { value, .. } => {
            ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
                symbol: field.to_string(),
                value,
                allowed,
            }
        }
    })
}
