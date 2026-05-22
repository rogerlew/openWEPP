use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use openwepp_input_contract::parsers::{
    climate::{BreakpointDay, ClimateDailyRecord, ClimateFile, NoBreakpointDay},
    soil::SoilProfile,
};
use openwepp_kernel_contract::{BoundarySymbol, BoundaryValue};

use crate::HillslopeWritebackSurface;

const CLIMATE_MIN_SUPPORTED_DATVER: f64 = 4.0;
const CLIGEN_POLICY_ICLIG: i32 = 1;
const CLIGEN_LEGACY_OVERRIDE_ICLIG: i32 = 0;
const DATVER_ZERO_TOLERANCE: f64 = 1e-9;
const HOURS_TO_SECONDS: f64 = 3_600.0;
const MILLIMETERS_TO_METERS: f64 = 0.001;

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

/// Immutable, parser-derived climate runtime request owned by hillslope
/// orchestration.
#[derive(Debug, Clone, PartialEq)]
pub struct HillslopeClimateRuntimeRequest {
    pub datver: f64,
    pub iclig: i32,
    pub itemp: i32,
    pub ibrkpt: i32,
    pub iwind: i32,
    pub station_id: String,
    pub daily_forcing: Vec<HillslopeClimateDailyForcing>,
}

/// Runtime daily forcing variants consumed by hillslope execution boundaries.
#[derive(Debug, Clone, PartialEq)]
pub enum HillslopeClimateDailyForcing {
    NoBreakpoint(HillslopeNoBreakpointForcing),
    Breakpoint(HillslopeBreakpointForcing),
}

/// Runtime forcing row for `ibrkpt=0`.
#[derive(Debug, Clone, PartialEq)]
pub struct HillslopeNoBreakpointForcing {
    pub day: i32,
    pub mon: i32,
    pub year: i32,
    pub prcp: f64,
    pub stmdur: f64,
    pub timep: f64,
    pub ip: f64,
    pub tmax: f64,
    pub tmin: f64,
    pub rad: f64,
    pub vwind: f64,
    pub wind: f64,
    pub tdpt: f64,
}

/// Runtime forcing row for `ibrkpt=1`.
#[derive(Debug, Clone, PartialEq)]
pub struct HillslopeBreakpointForcing {
    pub day: i32,
    pub mon: i32,
    pub year: i32,
    pub nbrkpt: usize,
    pub prcp: f64,
    pub stmdur: f64,
    pub timem: Vec<f64>,
    pub intsty: Vec<f64>,
    pub tmax: f64,
    pub tmin: f64,
    pub rad: f64,
    pub vwind: f64,
    pub wind: f64,
    pub tdpt: f64,
}

/// Typed climate runtime seam failures (`HS-CLIM-SEAM-001`).
#[derive(Debug, Clone, PartialEq)]
pub enum ClimateRuntimeInputError {
    UnsupportedDatver { datver: f64 },
    UnsupportedItemp { itemp: i32 },
    EmptyDailyRecords,
    DayIndexOutOfRange { day_index: usize, available: usize },
    NonFiniteField { field: &'static str, value: f64 },
    NegativeField { field: &'static str, value: f64 },
    PositivePrecipWithNonPositiveDuration { prcp: f64, stmdur: f64 },
    EmptyBreakpointSeries,
    NonMonotoneBreakpointTime { previous_s: f64, current_s: f64 },
    PositiveBreakpointDrainWithNonPositiveDeltaTime { drain_m: f64, delta_time_s: f64 },
    BreakpointCountOutOfRange { value: usize },
}

impl ClimateRuntimeInputError {
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::UnsupportedDatver { .. } => "CLIM-RUNTIME-E-001",
            Self::UnsupportedItemp { .. } => "CLIM-RUNTIME-E-002",
            Self::EmptyDailyRecords => "CLIM-RUNTIME-E-003",
            Self::DayIndexOutOfRange { .. } => "CLIM-RUNTIME-E-004",
            Self::NonFiniteField { .. } => "CLIM-RUNTIME-E-005",
            Self::NegativeField { .. } => "CLIM-RUNTIME-E-006",
            Self::PositivePrecipWithNonPositiveDuration { .. } => "CLIM-RUNTIME-E-007",
            Self::EmptyBreakpointSeries => "CLIM-RUNTIME-E-008",
            Self::NonMonotoneBreakpointTime { .. } => "CLIM-RUNTIME-E-009",
            Self::PositiveBreakpointDrainWithNonPositiveDeltaTime { .. } => "CLIM-RUNTIME-E-010",
            Self::BreakpointCountOutOfRange { .. } => "CLIM-RUNTIME-E-011",
        }
    }
}

impl fmt::Display for ClimateRuntimeInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedDatver { datver } => write!(
                f,
                "{}: unsupported climate datver {} (supports datver=0.0 override or datver>=4.0)",
                self.code(),
                datver
            ),
            Self::UnsupportedItemp { itemp } => write!(
                f,
                "{}: unsupported climate itemp {}; only continuous-daily itemp=1 is supported",
                self.code(),
                itemp
            ),
            Self::EmptyDailyRecords => write!(
                f,
                "{}: climate parser output contains no daily forcing records",
                self.code()
            ),
            Self::DayIndexOutOfRange {
                day_index,
                available,
            } => write!(
                f,
                "{}: requested day index {} exceeds available climate records {}",
                self.code(),
                day_index,
                available
            ),
            Self::NonFiniteField { field, value } => write!(
                f,
                "{}: non-finite climate field {}={}",
                self.code(),
                field,
                value
            ),
            Self::NegativeField { field, value } => write!(
                f,
                "{}: negative climate field {}={}",
                self.code(),
                field,
                value
            ),
            Self::PositivePrecipWithNonPositiveDuration { prcp, stmdur } => write!(
                f,
                "{}: positive precipitation {} requires positive storm duration, got {}",
                self.code(),
                prcp,
                stmdur
            ),
            Self::EmptyBreakpointSeries => write!(
                f,
                "{}: breakpoint forcing record contains zero breakpoint points",
                self.code()
            ),
            Self::NonMonotoneBreakpointTime {
                previous_s,
                current_s,
            } => write!(
                f,
                "{}: breakpoint timem must be monotone nondecreasing ({} -> {})",
                self.code(),
                previous_s,
                current_s
            ),
            Self::PositiveBreakpointDrainWithNonPositiveDeltaTime {
                drain_m,
                delta_time_s,
            } => write!(
                f,
                "{}: positive breakpoint rainfall increment {} requires positive elapsed time, got {}",
                self.code(),
                drain_m,
                delta_time_s
            ),
            Self::BreakpointCountOutOfRange { value } => write!(
                f,
                "{}: breakpoint count {} exceeds supported conversion range",
                self.code(),
                value
            ),
        }
    }
}

impl Error for ClimateRuntimeInputError {}

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
    let iclig = resolve_iclig(climate.datver)?;

    if climate.mode.itemp != 1 {
        return Err(ClimateRuntimeInputError::UnsupportedItemp {
            itemp: climate.mode.itemp,
        });
    }
    if climate.daily_records.is_empty() {
        return Err(ClimateRuntimeInputError::EmptyDailyRecords);
    }

    let mut daily_forcing = Vec::with_capacity(climate.daily_records.len());
    for record in &climate.daily_records {
        daily_forcing.push(adapt_daily_forcing(record)?);
    }

    Ok(HillslopeClimateRuntimeRequest {
        datver: climate.datver,
        iclig,
        itemp: climate.mode.itemp,
        ibrkpt: i32::from(climate.mode.breakpoint_enabled),
        iwind: climate.mode.iwind,
        station_id: climate.station_id.clone(),
        daily_forcing,
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
    let forcing = climate.daily_forcing.get(day_index).ok_or(
        ClimateRuntimeInputError::DayIndexOutOfRange {
            day_index,
            available: climate.daily_forcing.len(),
        },
    )?;

    let state_surface = &mut runtime_surface.state_surface;
    state_surface.insert(
        BoundarySymbol::from("datver"),
        BoundaryValue::scalar(climate.datver),
    );
    state_surface.insert(
        BoundarySymbol::from("iclig"),
        BoundaryValue::scalar(f64::from(climate.iclig)),
    );
    state_surface.insert(
        BoundarySymbol::from("itemp"),
        BoundaryValue::scalar(f64::from(climate.itemp)),
    );
    state_surface.insert(
        BoundarySymbol::from("ibrkpt"),
        BoundaryValue::scalar(f64::from(climate.ibrkpt)),
    );
    state_surface.insert(
        BoundarySymbol::from("iwind"),
        BoundaryValue::scalar(f64::from(climate.iwind)),
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
        }
        HillslopeClimateDailyForcing::Breakpoint(day) => {
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

            for (index, value) in day.timem.iter().enumerate() {
                let symbol = format!("timem_{:04}", index + 1);
                state_surface.insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(*value));
            }
            for (index, value) in day.intsty.iter().enumerate() {
                let symbol = format!("intsty_{:04}", index + 1);
                state_surface.insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(*value));
            }
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

fn adapt_daily_forcing(
    record: &ClimateDailyRecord,
) -> Result<HillslopeClimateDailyForcing, ClimateRuntimeInputError> {
    match record {
        ClimateDailyRecord::NoBreakpoint(day) => Ok(HillslopeClimateDailyForcing::NoBreakpoint(
            adapt_no_breakpoint(day)?,
        )),
        ClimateDailyRecord::Breakpoint(day) => Ok(HillslopeClimateDailyForcing::Breakpoint(
            adapt_breakpoint(day)?,
        )),
    }
}

fn adapt_no_breakpoint(
    day: &NoBreakpointDay,
) -> Result<HillslopeNoBreakpointForcing, ClimateRuntimeInputError> {
    require_non_negative("prcp", day.prcp)?;
    require_non_negative("stmdur", day.stmdur)?;
    require_non_negative("timep", day.timep)?;
    require_non_negative("ip", day.ip)?;
    require_finite("tmax", day.tmax)?;
    require_finite("tmin", day.tmin)?;
    require_finite("rad", day.rad)?;
    require_finite("vwind", day.vwind)?;
    require_finite("wind", day.wind)?;
    require_finite("tdpt", day.tdpt)?;

    let prcp = day.prcp * MILLIMETERS_TO_METERS;
    let stmdur = day.stmdur * HOURS_TO_SECONDS;
    if prcp > 0.0 && stmdur <= 0.0 {
        return Err(
            ClimateRuntimeInputError::PositivePrecipWithNonPositiveDuration { prcp, stmdur },
        );
    }

    Ok(HillslopeNoBreakpointForcing {
        day: day.day,
        mon: day.mon,
        year: day.year,
        prcp,
        stmdur,
        timep: day.timep,
        ip: day.ip,
        tmax: day.tmax,
        tmin: day.tmin,
        rad: day.rad,
        vwind: day.vwind,
        wind: day.wind,
        tdpt: day.tdpt,
    })
}

fn adapt_breakpoint(
    day: &BreakpointDay,
) -> Result<HillslopeBreakpointForcing, ClimateRuntimeInputError> {
    require_finite("tmax", day.tmax)?;
    require_finite("tmin", day.tmin)?;
    require_finite("rad", day.rad)?;
    require_finite("vwind", day.vwind)?;
    require_finite("wind", day.wind)?;
    require_finite("tdpt", day.tdpt)?;

    if day.breakpoints.is_empty() {
        return Err(ClimateRuntimeInputError::EmptyBreakpointSeries);
    }

    let mut timem = Vec::with_capacity(day.breakpoints.len());
    let mut pptcum = Vec::with_capacity(day.breakpoints.len());
    for point in &day.breakpoints {
        require_non_negative("timem", point.timem)?;
        require_non_negative("pptcum", point.pptcum)?;
        timem.push(point.timem * HOURS_TO_SECONDS);
        pptcum.push(point.pptcum * MILLIMETERS_TO_METERS);
    }

    let mut intsty = vec![0.0; timem.len()];
    for index in 1..timem.len() {
        let previous_time = timem[index - 1];
        let current_time = timem[index];
        if current_time <= previous_time {
            return Err(ClimateRuntimeInputError::NonMonotoneBreakpointTime {
                previous_s: previous_time,
                current_s: current_time,
            });
        }

        let drain = pptcum[index] - pptcum[index - 1];
        if drain < 0.0 {
            return Err(ClimateRuntimeInputError::NegativeField {
                field: "drain",
                value: drain,
            });
        }

        let delta_time_s = current_time - previous_time;
        if delta_time_s <= 0.0 {
            return Err(
                ClimateRuntimeInputError::PositiveBreakpointDrainWithNonPositiveDeltaTime {
                    drain_m: drain,
                    delta_time_s,
                },
            );
        }
        intsty[index - 1] = drain / delta_time_s;
    }

    let prcp = *pptcum
        .last()
        .ok_or(ClimateRuntimeInputError::EmptyBreakpointSeries)?;
    let stmdur = timem
        .last()
        .zip(timem.first())
        .map_or(0.0, |(end, start)| end - start);
    if prcp > 0.0 && stmdur <= 0.0 {
        return Err(
            ClimateRuntimeInputError::PositivePrecipWithNonPositiveDuration { prcp, stmdur },
        );
    }

    Ok(HillslopeBreakpointForcing {
        day: day.day,
        mon: day.mon,
        year: day.year,
        nbrkpt: day.nbrkpt,
        prcp,
        stmdur,
        timem,
        intsty,
        tmax: day.tmax,
        tmin: day.tmin,
        rad: day.rad,
        vwind: day.vwind,
        wind: day.wind,
        tdpt: day.tdpt,
    })
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

fn require_finite(field: &'static str, value: f64) -> Result<(), ClimateRuntimeInputError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(ClimateRuntimeInputError::NonFiniteField { field, value })
    }
}

fn require_non_negative(field: &'static str, value: f64) -> Result<(), ClimateRuntimeInputError> {
    require_finite(field, value)?;
    if value < 0.0 {
        Err(ClimateRuntimeInputError::NegativeField { field, value })
    } else {
        Ok(())
    }
}

fn resolve_iclig(datver: f64) -> Result<i32, ClimateRuntimeInputError> {
    require_finite("datver", datver)?;
    if datver.abs() <= DATVER_ZERO_TOLERANCE {
        Ok(CLIGEN_LEGACY_OVERRIDE_ICLIG)
    } else if datver >= CLIMATE_MIN_SUPPORTED_DATVER {
        Ok(CLIGEN_POLICY_ICLIG)
    } else {
        Err(ClimateRuntimeInputError::UnsupportedDatver { datver })
    }
}

#[cfg(test)]
mod tests {
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
    const VALID_9002: &str = include_str!("../../../tests/fixtures/infile/soil/valid_9002.sol");
    const VALID_97_5: &str = include_str!("../../../tests/fixtures/infile/soil/valid_97_5.sol");

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

        assert!((datver - 5.3).abs() < 1e-12);
        assert!((iclig - 1.0).abs() < 1e-12);
        assert!((prcp - 0.01).abs() < 1e-12);
        assert!((stmdur - 7_200.0).abs() < 1e-12);
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
        assert!((iclig - 0.0).abs() < 1e-12);
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
