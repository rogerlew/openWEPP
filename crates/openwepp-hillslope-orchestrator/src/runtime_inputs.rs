use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use openwepp_climate_runtime_adapter::{
    SharedClimateDailyForcing as HillslopeClimateDailyForcing,
    SharedClimateRuntimeInputError as ClimateRuntimeInputError,
    SharedClimateRuntimeRequest as SharedHillslopeClimateRuntimeRequest,
    build_climate_runtime_request, select_day_forcing,
};
use openwepp_input_contract::parsers::{climate::ClimateFile, soil::SoilProfile};
use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, ClimateForcingSymbolSurface, ClimateForcingSymbolSurfaceError,
};

use crate::HillslopeWritebackSurface;

/// Typed errors for parser-to-hillslope runtime surface adaptation.
#[derive(Debug, Clone, PartialEq)]
pub enum HillslopeRuntimeInputError {
    MissingSoilOfe,
    MissingSoilLayer,
    MissingThetaResidual,
    MissingThetaFieldCapacity,
    NonFiniteProfileDepth { value_mm: f64 },
    NonPositiveProfileDepth { value_mm: f64 },
    NonFiniteTopLayerDepth { value_mm: f64 },
    NonPositiveTopLayerDepth { value_mm: f64 },
    NonFiniteThetaResidual { value: f64 },
    NonFiniteThetaFieldCapacity { value: f64 },
}

impl HillslopeRuntimeInputError {
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::MissingSoilOfe => "HS-RUNTIME-E-001",
            Self::MissingSoilLayer => "HS-RUNTIME-E-002",
            Self::MissingThetaResidual => "HS-RUNTIME-E-003",
            Self::MissingThetaFieldCapacity => "HS-RUNTIME-E-004",
            Self::NonFiniteProfileDepth { .. } => "HS-RUNTIME-E-005",
            Self::NonPositiveProfileDepth { .. } => "HS-RUNTIME-E-006",
            Self::NonFiniteTopLayerDepth { .. } => "HS-RUNTIME-E-007",
            Self::NonPositiveTopLayerDepth { .. } => "HS-RUNTIME-E-008",
            Self::NonFiniteThetaResidual { .. } => "HS-RUNTIME-E-009",
            Self::NonFiniteThetaFieldCapacity { .. } => "HS-RUNTIME-E-010",
        }
    }
}

impl fmt::Display for HillslopeRuntimeInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingSoilOfe => {
                write!(f, "{}: soil profile contains no OFE blocks", self.code())
            }
            Self::MissingSoilLayer => {
                write!(f, "{}: primary OFE contains no soil layers", self.code())
            }
            Self::MissingThetaResidual => write!(
                f,
                "{}: primary soil layer missing required theta_r_rosetta (thetdr)",
                self.code()
            ),
            Self::MissingThetaFieldCapacity => write!(
                f,
                "{}: primary soil layer missing required fc_rosetta (thetfc)",
                self.code()
            ),
            Self::NonFiniteProfileDepth { value_mm } => write!(
                f,
                "{}: non-finite soil profile depth_mm value {}",
                self.code(),
                value_mm
            ),
            Self::NonPositiveProfileDepth { value_mm } => write!(
                f,
                "{}: non-positive soil profile depth_mm value {}",
                self.code(),
                value_mm
            ),
            Self::NonFiniteTopLayerDepth { value_mm } => write!(
                f,
                "{}: non-finite top-layer depth_mm value {}",
                self.code(),
                value_mm
            ),
            Self::NonPositiveTopLayerDepth { value_mm } => write!(
                f,
                "{}: non-positive top-layer depth_mm value {}",
                self.code(),
                value_mm
            ),
            Self::NonFiniteThetaResidual { value } => {
                write!(f, "{}: non-finite thetdr value {}", self.code(), value)
            }
            Self::NonFiniteThetaFieldCapacity { value } => {
                write!(f, "{}: non-finite thetfc value {}", self.code(), value)
            }
        }
    }
}

impl Error for HillslopeRuntimeInputError {}

/// Typed hillslope climate runtime request with precomputed boundary alias
/// projections for forcing series surfaces.
#[derive(Debug, Clone, PartialEq)]
pub struct HillslopeClimateRuntimeRequest {
    shared: SharedHillslopeClimateRuntimeRequest,
    day_symbol_surfaces: Vec<ClimateForcingSymbolSurface>,
}

/// Build an orchestrator-owned hillslope runtime surface from parsed soil input.
///
/// This seam is strict by design: missing runtime-critical fields fail
/// explicitly instead of defaulting.
///
/// # Errors
///
/// Returns `HillslopeRuntimeInputError` when required parser outputs are
/// missing or non-finite.
pub fn build_hillslope_runtime_surface_from_soil(
    soil: &SoilProfile,
) -> Result<HillslopeWritebackSurface, HillslopeRuntimeInputError> {
    let primary_ofe = soil
        .ofes
        .first()
        .ok_or(HillslopeRuntimeInputError::MissingSoilOfe)?;
    let top_layer = primary_ofe
        .layers
        .first()
        .ok_or(HillslopeRuntimeInputError::MissingSoilLayer)?;
    let bottom_layer = primary_ofe
        .layers
        .last()
        .ok_or(HillslopeRuntimeInputError::MissingSoilLayer)?;

    let profile_depth_mm = bottom_layer.depth_mm;
    if !profile_depth_mm.is_finite() {
        return Err(HillslopeRuntimeInputError::NonFiniteProfileDepth {
            value_mm: profile_depth_mm,
        });
    }
    if profile_depth_mm <= 0.0 {
        return Err(HillslopeRuntimeInputError::NonPositiveProfileDepth {
            value_mm: profile_depth_mm,
        });
    }

    let top_layer_depth_mm = top_layer.depth_mm;
    if !top_layer_depth_mm.is_finite() {
        return Err(HillslopeRuntimeInputError::NonFiniteTopLayerDepth {
            value_mm: top_layer_depth_mm,
        });
    }
    if top_layer_depth_mm <= 0.0 {
        return Err(HillslopeRuntimeInputError::NonPositiveTopLayerDepth {
            value_mm: top_layer_depth_mm,
        });
    }

    let thetdr = top_layer
        .theta_r_rosetta
        .ok_or(HillslopeRuntimeInputError::MissingThetaResidual)?;
    if !thetdr.is_finite() {
        return Err(HillslopeRuntimeInputError::NonFiniteThetaResidual { value: thetdr });
    }

    let thetfc = top_layer
        .fc_rosetta
        .ok_or(HillslopeRuntimeInputError::MissingThetaFieldCapacity)?;
    if !thetfc.is_finite() {
        return Err(HillslopeRuntimeInputError::NonFiniteThetaFieldCapacity { value: thetfc });
    }

    let mut state_surface = BTreeMap::new();
    state_surface.insert(
        BoundarySymbol::from("solthk"),
        BoundaryValue::scalar(profile_depth_mm / 1_000.0),
    );
    state_surface.insert(
        BoundarySymbol::from("dg"),
        BoundaryValue::scalar(top_layer_depth_mm / 1_000.0),
    );
    state_surface.insert(
        BoundarySymbol::from("thetdr"),
        BoundaryValue::scalar(thetdr),
    );
    state_surface.insert(
        BoundarySymbol::from("thetfc"),
        BoundaryValue::scalar(thetfc),
    );

    Ok(HillslopeWritebackSurface {
        state_surface,
        flux_surface: BTreeMap::new(),
    })
}

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

fn insert_common_day_symbols(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    day: i32,
    mon: i32,
    year: i32,
) {
    surface.insert(
        BoundarySymbol::from("day"),
        BoundaryValue::scalar(f64::from(day)),
    );
    surface.insert(
        BoundarySymbol::from("mon"),
        BoundaryValue::scalar(f64::from(mon)),
    );
    surface.insert(
        BoundarySymbol::from("year"),
        BoundaryValue::scalar(f64::from(year)),
    );
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
        ClimateForcingSymbolSurfaceError::PointCountOutOfRange { count, .. } => {
            ClimateRuntimeInputError::BreakpointCountOutOfRange { value: *count }
        }
    }
}

fn insert_series_values(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    symbols: &[BoundarySymbol],
    values: &[f64],
) {
    debug_assert_eq!(symbols.len(), values.len());
    for (symbol, value) in symbols.iter().zip(values.iter()) {
        surface.insert(symbol.clone(), BoundaryValue::scalar(*value));
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Write as _;

    use openwepp_input_contract::parsers::{
        climate::{CompatibilityOptions, ParserMode as ClimateParserMode, parse_climate_from_str},
        soil::{ParserMode, SoilParserOptions, parse_soil},
    };
    use openwepp_kernel_contract::BoundarySymbol;

    use super::{
        ClimateRuntimeInputError, HillslopeRuntimeInputError,
        build_hillslope_runtime_surface_from_climate, build_hillslope_runtime_surface_from_soil,
    };

    const VALID_CLIMATE: &str =
        include_str!("../../../tests/fixtures/infile/climate/strict_valid.cli");
    const LEGACY_DATVER_CLIMATE: &str =
        include_str!("../../../tests/fixtures/infile/climate/legacy_datver_0.cli");
    const SINGLE_STORM_CLIMATE: &str =
        include_str!("../../../tests/fixtures/infile/climate/single_storm_itemp2.cli");
    const BREAKPOINT_OVERFLOW_CLIMATE: &str =
        include_str!("../../../tests/fixtures/infile/climate/breakpoint_overflow_51.cli");
    const WC1_BREAKPOINT_STMSTR_NONZERO: &str = include_str!(
        "../../../tests/fixtures/infile/climate/wc1_major_restlessness_breakpoint_stmstr_nonzero.cli"
    );
    const WC1_BREAKPOINT_NBRKPT_42: &str = include_str!(
        "../../../tests/fixtures/infile/climate/wc1_major_restlessness_breakpoint_nbrkpt_42.cli"
    );
    const WC1_CANOGA_DAY1: &str =
        include_str!("../../../tests/fixtures/infile/climate/wc1_canoga_day1.cli");
    const WC1_CANOGA_STMDUR_CAP: &str =
        include_str!("../../../tests/fixtures/infile/climate/wc1_canoga_stmdur_cap.cli");
    const VALID_9002: &str = include_str!("../../../tests/fixtures/infile/soil/valid_9002.sol");
    const VALID_97_5: &str = include_str!("../../../tests/fixtures/infile/soil/valid_97_5.sol");

    fn build_breakpoint_fixture(nbrkpt: usize) -> String {
        let mut climate = format!(
            "5.30\n1 1 0\nTEST STATION 1500\nDAY MON YEAR NBRKPT TMAX TMIN RAD VWIND WIND TDPT\n45.0 -120.0 1000.0 30 2000 1\nMONTHLY MAX TEMP HEADER\n1 2 3 4 5 6 7 8 9 10 11 12\nMONTHLY MIN TEMP HEADER\n-5 -4 -3 -2 -1 0 1 2 3 4 5 6\nMONTHLY RAD HEADER\n100 101 102 103 104 105 106 107 108 109 110 111\nMONTHLY RAIN HEADER\n10 11 12 13 14 15 16 17 18 19 20 21\nDAILY HEADER\nDAILY UNITS\n1 1 2000 {nbrkpt} 11.0 1.0 180.0 2.0 170.0 -2.0\n"
        );
        if nbrkpt == 0 {
            return climate;
        }
        let denom_u32 = u32::try_from((nbrkpt - 1).max(1))
            .expect("breakpoint fixture helper expects small cardinalities");
        let denom = f64::from(denom_u32);
        for index in 0..nbrkpt {
            let idx_u32 = u32::try_from(index)
                .expect("breakpoint fixture helper expects small cardinalities");
            let idx = f64::from(idx_u32);
            let timem = (24.0 * idx) / denom;
            let pptcum = (120.0 * idx) / denom;
            writeln!(&mut climate, "{timem:.4} {pptcum:.3}")
                .expect("writing synthetic breakpoint fixture should succeed");
        }
        climate
    }

    #[test]
    fn soil_runtime_surface_contains_canonical_state_symbols() {
        let soil = parse_soil(
            VALID_9002,
            SoilParserOptions {
                mode: ParserMode::Strict,
                allow_legacy_aliases: false,
                expected_topology_count: None,
                topology_scope: None,
            },
        )
        .expect("9002 soil fixture should parse");

        let surface = build_hillslope_runtime_surface_from_soil(&soil)
            .expect("runtime surface should build from parsed soil");

        let solthk = surface
            .state_surface
            .get(&BoundarySymbol::from("solthk"))
            .expect("solthk should be present")
            .as_f64();
        let dg = surface
            .state_surface
            .get(&BoundarySymbol::from("dg"))
            .expect("dg should be present")
            .as_f64();
        let thetdr = surface
            .state_surface
            .get(&BoundarySymbol::from("thetdr"))
            .expect("thetdr should be present")
            .as_f64();
        let thetfc = surface
            .state_surface
            .get(&BoundarySymbol::from("thetfc"))
            .expect("thetfc should be present")
            .as_f64();

        assert!((solthk - 0.25).abs() < 1e-12);
        assert!((dg - 0.1).abs() < 1e-12);
        assert!((thetdr - 0.05).abs() < 1e-12);
        assert!((thetfc - 0.31).abs() < 1e-12);
    }

    #[test]
    fn soil_runtime_surface_rejects_missing_theta_fields() {
        let soil = parse_soil(VALID_97_5, SoilParserOptions::default())
            .expect("97.5 soil fixture should parse");

        let error = build_hillslope_runtime_surface_from_soil(&soil)
            .expect_err("missing theta fields must fail runtime adaptation");
        assert_eq!(error.code(), "HS-RUNTIME-E-003");
        assert!(matches!(
            error,
            HillslopeRuntimeInputError::MissingThetaResidual
        ));
    }

    #[test]
    fn climate_runtime_surface_contains_canonical_daily_symbols() {
        let climate = parse_climate_from_str(VALID_CLIMATE, ClimateParserMode::Strict)
            .expect("strict climate fixture should parse");
        let surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect("climate runtime surface should build");

        let datver = surface
            .state_surface
            .get(&BoundarySymbol::from("datver"))
            .expect("datver should exist")
            .as_f64();
        let iclig = surface
            .state_surface
            .get(&BoundarySymbol::from("iclig"))
            .expect("iclig should exist")
            .as_f64();
        let prcp = surface
            .state_surface
            .get(&BoundarySymbol::from("prcp"))
            .expect("prcp should exist")
            .as_f64();
        let stmdur = surface
            .state_surface
            .get(&BoundarySymbol::from("stmdur"))
            .expect("stmdur should exist")
            .as_f64();
        let ip = surface
            .state_surface
            .get(&BoundarySymbol::from("ip"))
            .expect("ip should exist")
            .as_f64();
        let ninten = surface
            .state_surface
            .get(&BoundarySymbol::from("ninten"))
            .expect("ninten should exist")
            .as_f64();
        let timem_first = surface
            .state_surface
            .get(&BoundarySymbol::from("timem_0001"))
            .expect("timem_0001 should exist")
            .as_f64();
        let intsty_first = surface
            .state_surface
            .get(&BoundarySymbol::from("intsty_0001"))
            .expect("intsty_0001 should exist")
            .as_f64();

        assert!((datver - 5.3).abs() < 1e-12);
        assert!((iclig - 1.0).abs() < 1e-12);
        assert!((prcp - 0.01).abs() < 1e-12);
        assert!((stmdur - 7_200.0).abs() < 1e-12);
        assert!((ip - 2.1).abs() < 1e-12);
        assert!(ninten >= 2.0);
        assert!(timem_first.abs() < 1e-12);
        assert!(intsty_first.is_finite());
    }

    #[test]
    fn breakpoint_runtime_surface_projects_stmstr_elapsed_timem_and_mxint() {
        let climate =
            parse_climate_from_str(WC1_BREAKPOINT_STMSTR_NONZERO, ClimateParserMode::Strict)
                .expect("curated wc1 breakpoint fixture should parse");
        let surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect("breakpoint runtime surface should build");

        let stmstr = surface
            .state_surface
            .get(&BoundarySymbol::from("stmstr"))
            .expect("stmstr should exist")
            .as_f64();
        let prcp = surface
            .state_surface
            .get(&BoundarySymbol::from("prcp"))
            .expect("prcp should exist")
            .as_f64();
        let stmdur = surface
            .state_surface
            .get(&BoundarySymbol::from("stmdur"))
            .expect("stmdur should exist")
            .as_f64();
        let mxint = surface
            .state_surface
            .get(&BoundarySymbol::from("mxint"))
            .expect("mxint should exist")
            .as_f64();
        let timem_1 = surface
            .state_surface
            .get(&BoundarySymbol::from("timem_0001"))
            .expect("timem_0001 should exist")
            .as_f64();
        let timem_2 = surface
            .state_surface
            .get(&BoundarySymbol::from("timem_0002"))
            .expect("timem_0002 should exist")
            .as_f64();
        let intsty_5 = surface
            .state_surface
            .get(&BoundarySymbol::from("intsty_0005"))
            .expect("intsty_0005 should exist")
            .as_f64();

        let times_h = [4.8667_f64, 17.2667, 19.4333, 21.3667, 23.9833];
        let pptcum_mm = [0.0_f64, 2.01, 4.02, 6.04, 7.35];
        let expected_stmdur = (times_h[4] - times_h[0]) * 3_600.0;
        let expected_timem_2 = (times_h[1] - times_h[0]) * 3_600.0;
        let mut expected_mxint: f64 = 0.0;
        for index in 1..times_h.len() {
            let drain_m = (pptcum_mm[index] - pptcum_mm[index - 1]) * 0.001;
            let delta_time_s = (times_h[index] - times_h[index - 1]) * 3_600.0;
            expected_mxint = expected_mxint.max(drain_m / delta_time_s);
        }

        assert!((stmstr - 4.8667).abs() < 1e-12);
        assert!((prcp - 0.00735).abs() < 1e-12);
        assert!((stmdur - expected_stmdur).abs() < 1e-6);
        assert!((mxint - expected_mxint).abs() < 1e-12);
        assert!(timem_1.abs() < 1e-12);
        assert!((timem_2 - expected_timem_2).abs() < 1e-6);
        assert!(intsty_5.abs() < 1e-12);
    }

    #[test]
    fn breakpoint_runtime_surface_supports_curated_wc1_42_point_event_shape() {
        let climate = parse_climate_from_str(WC1_BREAKPOINT_NBRKPT_42, ClimateParserMode::Strict)
            .expect("42-point wc1 fixture should parse");
        let surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect("42-point breakpoint surface should build");

        let nbrkpt = surface
            .state_surface
            .get(&BoundarySymbol::from("nbrkpt"))
            .expect("nbrkpt should exist")
            .as_f64();
        let timem_first = surface
            .state_surface
            .get(&BoundarySymbol::from("timem_0001"))
            .expect("timem_0001 should exist")
            .as_f64();
        let timem_last = surface
            .state_surface
            .get(&BoundarySymbol::from("timem_0042"))
            .expect("timem_0042 should exist")
            .as_f64();
        let intsty_last = surface
            .state_surface
            .get(&BoundarySymbol::from("intsty_0042"))
            .expect("intsty_0042 should exist")
            .as_f64();

        assert!((nbrkpt - 42.0).abs() < 1e-12);
        assert!(timem_first.abs() < 1e-12);
        assert!(timem_last > timem_first);
        assert!(intsty_last.abs() < 1e-12);
    }

    #[test]
    fn climate_runtime_surface_accepts_breakpoint_cardinality_at_1500_boundary() {
        let climate =
            parse_climate_from_str(&build_breakpoint_fixture(1_500), ClimateParserMode::Strict)
                .expect("strict parser should accept 1500 breakpoint rows");
        let surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect("runtime seam should accept 1500 breakpoint rows");

        let nbrkpt = surface
            .state_surface
            .get(&BoundarySymbol::from("nbrkpt"))
            .expect("nbrkpt should exist")
            .as_f64();
        assert!((nbrkpt - 1_500.0).abs() < 1e-12);
    }

    #[test]
    fn climate_runtime_surface_rejects_breakpoint_cardinality_over_1500_even_with_parser_override()
    {
        let climate = parse_climate_from_str(
            &build_breakpoint_fixture(1_501),
            ClimateParserMode::Compatibility(CompatibilityOptions {
                allow_single_storm: false,
                allow_breakpoint_cardinality_override: true,
                allow_legacy_zero_drain_non_positive_dtime: false,
            }),
        )
        .expect("compat parser should allow >1500 breakpoint rows with explicit override");

        let error = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect_err("runtime seam must reject >1500 breakpoint rows");
        assert_eq!(error.code(), "CLIM-RUNTIME-E-011");
        assert!(matches!(
            error,
            ClimateRuntimeInputError::BreakpointCardinalityPolicyExceeded {
                value: 1_501,
                max: 1_500
            }
        ));
    }

    #[test]
    fn climate_runtime_surface_supports_explicit_datver_zero_override() {
        let climate = parse_climate_from_str(LEGACY_DATVER_CLIMATE, ClimateParserMode::Strict)
            .expect("legacy datver fixture should parse");
        let surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect("seam should accept explicit datver=0.0 override");

        let iclig = surface
            .state_surface
            .get(&BoundarySymbol::from("iclig"))
            .expect("iclig should exist for datver override")
            .as_f64();
        let ip = surface
            .state_surface
            .get(&BoundarySymbol::from("ip"))
            .expect("ip should exist for datver override")
            .as_f64();
        assert!((iclig - 0.0).abs() < 1e-12);
        assert!((ip - 2.0).abs() < 1e-12);
    }

    #[test]
    fn climate_runtime_surface_applies_timep_floor_for_wet_nonconstant_events() {
        let climate = parse_climate_from_str(WC1_CANOGA_DAY1, ClimateParserMode::Strict)
            .expect("wc1 fixture should parse");
        let surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect("wc1 runtime surface should build");

        let timep = surface
            .state_surface
            .get(&BoundarySymbol::from("timep"))
            .expect("timep should exist")
            .as_f64();
        let ip = surface
            .state_surface
            .get(&BoundarySymbol::from("ip"))
            .expect("ip should exist")
            .as_f64();
        assert!((timep - 0.01).abs() < 1e-12);
        assert!((ip - 2.94).abs() < 1e-12);
    }

    #[test]
    fn climate_runtime_surface_caps_storm_duration_to_23_999_hours() {
        let climate = parse_climate_from_str(WC1_CANOGA_STMDUR_CAP, ClimateParserMode::Strict)
            .expect("wc1 duration-cap fixture should parse");
        let surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect("duration-cap fixture should build runtime surface");

        let stmdur = surface
            .state_surface
            .get(&BoundarySymbol::from("stmdur"))
            .expect("stmdur should exist")
            .as_f64();
        let ip = surface
            .state_surface
            .get(&BoundarySymbol::from("ip"))
            .expect("ip should exist")
            .as_f64();
        assert!((stmdur - (23.999 * 3_600.0)).abs() < 1e-9);
        assert!((ip - 22.589).abs() < 1e-12);
    }

    #[test]
    fn climate_runtime_surface_rejects_pre4_nonzero_datver_branch() {
        let mut climate = parse_climate_from_str(VALID_CLIMATE, ClimateParserMode::Strict)
            .expect("strict climate fixture should parse");
        climate.datver = 3.9;

        let error = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect_err("pre-4 nonzero branch must be rejected");
        assert_eq!(error.code(), "CLIM-RUNTIME-E-001");
        assert!(matches!(
            error,
            ClimateRuntimeInputError::UnsupportedDatver { datver } if (datver - 3.9).abs() < 1e-12
        ));
    }

    #[test]
    fn climate_runtime_surface_rejects_single_storm_even_in_compat_parser_mode() {
        let climate = parse_climate_from_str(
            SINGLE_STORM_CLIMATE,
            ClimateParserMode::Compatibility(CompatibilityOptions {
                allow_single_storm: true,
                allow_breakpoint_cardinality_override: false,
                allow_legacy_zero_drain_non_positive_dtime: false,
            }),
        )
        .expect("compat parser should accept itemp=2 when explicitly enabled");

        let error = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect_err("runtime seam must reject single-storm itemp=2");
        assert_eq!(error.code(), "CLIM-RUNTIME-E-002");
        assert!(matches!(
            error,
            ClimateRuntimeInputError::UnsupportedItemp { itemp: 2 }
        ));
    }

    #[test]
    fn climate_runtime_surface_rejects_duplicate_breakpoint_times() {
        let mut climate = parse_climate_from_str(
            BREAKPOINT_OVERFLOW_CLIMATE,
            ClimateParserMode::Compatibility(CompatibilityOptions {
                allow_single_storm: false,
                allow_breakpoint_cardinality_override: true,
                allow_legacy_zero_drain_non_positive_dtime: false,
            }),
        )
        .expect("breakpoint fixture should parse in compatibility mode");

        let day = climate
            .daily_records
            .first_mut()
            .expect("one breakpoint day expected");
        match day {
            openwepp_input_contract::parsers::climate::ClimateDailyRecord::Breakpoint(record) => {
                let first_timem = record
                    .breakpoints
                    .first()
                    .expect("first breakpoint point should exist")
                    .timem;
                record
                    .breakpoints
                    .get_mut(1)
                    .expect("second breakpoint point should exist")
                    .timem = first_timem;
            }
            openwepp_input_contract::parsers::climate::ClimateDailyRecord::NoBreakpoint(_) => {
                panic!("expected breakpoint daily record")
            }
        }

        let error = build_hillslope_runtime_surface_from_climate(&climate, 0)
            .expect_err("duplicate breakpoint timem must fail seam guard");
        assert_eq!(error.code(), "CLIM-RUNTIME-E-009");
        assert!(matches!(
            error,
            ClimateRuntimeInputError::NonMonotoneBreakpointTime { .. }
        ));
    }
}
