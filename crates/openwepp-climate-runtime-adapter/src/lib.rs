#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]

//! Shared climate parser-to-runtime adapter logic consumed by hillslope and
//! watershed orchestrators.

use std::error::Error;
use std::fmt;

use openwepp_input_contract::parsers::climate::{
    BreakpointDay, ClimateDailyRecord, ClimateFile, NoBreakpointDay,
};

const CLIMATE_MIN_SUPPORTED_DATVER: f64 = 4.0;
const MAX_BREAKPOINTS_PER_DAY: usize = 1_500;
const CLIGEN_POLICY_ICLIG: i32 = 1;
const CLIGEN_LEGACY_OVERRIDE_ICLIG: i32 = 0;
const DATVER_ZERO_TOLERANCE: f64 = 1e-9;
const HOURS_TO_SECONDS: f64 = 3_600.0;
const MILLIMETERS_TO_METERS: f64 = 0.001;
// Legacy WEPP iclig=1 behavior for datver>=4.0 applies ip*=0.70.
// Provenance: /workdir/wepp-forest_260430_baseline/src/stmget.for:176-183
// and CLIM01 detailed spec P1 (openWEPP climate model detailed specification).
const CLIGEN_V4_IP_CORRECTION_FACTOR: f64 = 0.70;
const MAX_STORM_DURATION_HOURS: f64 = 23.999;
const DISAG_DEFAULT_INTERVAL_COUNT: usize = 11;
const DISAG_MIN_INTERVAL_SECONDS: f64 = 300.0;
const DISAG_MIN_TIMEP: f64 = 0.01;
const DISAG_MAX_TIMEP: f64 = 0.99;
const DISAG_MAX_IP: f64 = 60.0;
const DISAG_EQROOT_SOLVER_TOLERANCE: f64 = 0.59e-6;
const DISAG_CLOSURE_TOLERANCE: f64 = 1e-9;

/// Immutable climate runtime request built from parser output.
#[derive(Debug, Clone, PartialEq)]
pub struct SharedClimateRuntimeRequest {
    pub datver: f64,
    pub iclig: i32,
    pub itemp: i32,
    pub ibrkpt: i32,
    pub iwind: i32,
    pub station_id: String,
    pub daily_forcing: Vec<SharedClimateDailyForcing>,
}

/// Runtime daily forcing variants consumed by orchestrator boundaries.
#[derive(Debug, Clone, PartialEq)]
pub enum SharedClimateDailyForcing {
    NoBreakpoint(SharedNoBreakpointForcing),
    Breakpoint(SharedBreakpointForcing),
}

/// Runtime forcing row for `ibrkpt=0`.
#[derive(Debug, Clone, PartialEq)]
pub struct SharedNoBreakpointForcing {
    pub day: i32,
    pub mon: i32,
    pub year: i32,
    pub prcp: f64,
    pub stmdur: f64,
    pub timep: f64,
    pub ip: f64,
    pub ninten: usize,
    pub avrint: f64,
    pub mxint: f64,
    pub timem: Vec<f64>,
    pub intsty: Vec<f64>,
    pub tmax: f64,
    pub tmin: f64,
    pub rad: f64,
    pub vwind: f64,
    pub wind: f64,
    pub tdpt: f64,
}

/// Runtime forcing row for `ibrkpt=1`.
#[derive(Debug, Clone, PartialEq)]
pub struct SharedBreakpointForcing {
    pub day: i32,
    pub mon: i32,
    pub year: i32,
    pub nbrkpt: usize,
    pub stmstr: f64,
    pub prcp: f64,
    pub stmdur: f64,
    pub mxint: f64,
    pub timem: Vec<f64>,
    pub intsty: Vec<f64>,
    pub tmax: f64,
    pub tmin: f64,
    pub rad: f64,
    pub vwind: f64,
    pub wind: f64,
    pub tdpt: f64,
}

/// Typed climate runtime seam failures.
#[derive(Debug, Clone, PartialEq)]
pub enum SharedClimateRuntimeInputError {
    UnsupportedDatver {
        datver: f64,
    },
    UnsupportedItemp {
        itemp: i32,
    },
    EmptyDailyRecords,
    DayIndexOutOfRange {
        day_index: usize,
        available: usize,
    },
    NonFiniteField {
        field: &'static str,
        value: f64,
    },
    NegativeField {
        field: &'static str,
        value: f64,
    },
    PositivePrecipWithNonPositiveDuration {
        prcp: f64,
        stmdur: f64,
    },
    EmptyBreakpointSeries,
    NonMonotoneBreakpointTime {
        previous_s: f64,
        current_s: f64,
    },
    BreakpointCardinalityPolicyExceeded {
        value: usize,
        max: usize,
    },
    BreakpointCountOutOfRange {
        value: usize,
    },
    DisaggregationTimeNotStrictlyIncreasing {
        previous_s: f64,
        current_s: f64,
    },
    DisaggregationRootSolveDomain {
        a: f64,
    },
    DisaggregationRootSolveNonConvergent {
        a: f64,
    },
    DisaggregationClosureResidual {
        expected_prcp_m: f64,
        reconstructed_prcp_m: f64,
    },
    MissingRuntimeContextSymbol {
        symbol: String,
    },
    RuntimeContextSymbolOutOfRange {
        symbol: String,
        value: f64,
        allowed: &'static str,
    },
    InvalidCalendarDate {
        day: i32,
        mon: i32,
        year: i32,
    },
}

impl SharedClimateRuntimeInputError {
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
            // CLIM-RUNTIME-E-010 is intentionally retired by CLIM15 because
            // no reachable guard path can emit it under strict monotonic-time
            // policy.
            Self::BreakpointCardinalityPolicyExceeded { .. } => "CLIM-RUNTIME-E-011",
            Self::BreakpointCountOutOfRange { .. } => "CLIM-RUNTIME-E-011",
            Self::DisaggregationTimeNotStrictlyIncreasing { .. } => "CLIM-RUNTIME-E-012",
            Self::DisaggregationRootSolveDomain { .. } => "CLIM-RUNTIME-E-013",
            Self::DisaggregationRootSolveNonConvergent { .. } => "CLIM-RUNTIME-E-014",
            Self::DisaggregationClosureResidual { .. } => "CLIM-RUNTIME-E-015",
            Self::MissingRuntimeContextSymbol { .. } => "CLIM-RUNTIME-E-016",
            Self::RuntimeContextSymbolOutOfRange { .. } => "CLIM-RUNTIME-E-017",
            Self::InvalidCalendarDate { .. } => "CLIM-RUNTIME-E-018",
        }
    }
}

impl fmt::Display for SharedClimateRuntimeInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: ", self.code())?;
        self.fmt_message(f)
    }
}

impl SharedClimateRuntimeInputError {
    fn fmt_message(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedDatver { datver } => write!(
                f,
                "unsupported climate datver {} (supports datver=0.0 override or datver>=4.0)",
                datver
            ),
            Self::UnsupportedItemp { itemp } => write!(
                f,
                "unsupported climate itemp {}; only continuous-daily itemp=1 is supported",
                itemp
            ),
            Self::EmptyDailyRecords => {
                f.write_str("climate parser output contains no daily forcing records")
            }
            Self::DayIndexOutOfRange {
                day_index,
                available,
            } => write!(
                f,
                "requested day index {} exceeds available climate records {}",
                day_index, available
            ),
            Self::NonFiniteField { field, value } => {
                write!(f, "non-finite climate field {}={}", field, value)
            }
            Self::NegativeField { field, value } => {
                write!(f, "negative climate field {}={}", field, value)
            }
            Self::PositivePrecipWithNonPositiveDuration { prcp, stmdur } => write!(
                f,
                "positive precipitation {} requires positive storm duration, got {}",
                prcp, stmdur
            ),
            Self::EmptyBreakpointSeries => {
                f.write_str("breakpoint forcing record contains zero breakpoint points")
            }
            Self::NonMonotoneBreakpointTime {
                previous_s,
                current_s,
            } => write!(
                f,
                "breakpoint timem must be strictly increasing ({} -> {})",
                previous_s, current_s
            ),
            Self::BreakpointCardinalityPolicyExceeded { value, max } => write!(
                f,
                "breakpoint count {} exceeds runtime policy max {}",
                value, max
            ),
            Self::BreakpointCountOutOfRange { value } => write!(
                f,
                "breakpoint count {} exceeds supported conversion range",
                value
            ),
            Self::DisaggregationTimeNotStrictlyIncreasing {
                previous_s,
                current_s,
            } => write!(
                f,
                "disaggregation time grid must be strictly increasing ({} -> {})",
                previous_s, current_s
            ),
            Self::DisaggregationRootSolveDomain { a } => {
                write!(f, "disaggregation root-solve domain invalid (a={})", a)
            }
            Self::DisaggregationRootSolveNonConvergent { a } => {
                write!(f, "disaggregation root solve failed to converge (a={})", a)
            }
            Self::DisaggregationClosureResidual {
                expected_prcp_m,
                reconstructed_prcp_m,
            } => write!(
                f,
                "disaggregation closure residual exceeded tolerance (expected={}, reconstructed={})",
                expected_prcp_m, reconstructed_prcp_m
            ),
            Self::MissingRuntimeContextSymbol { symbol } => write!(
                f,
                "missing required runtime context symbol {} for active winter forcing synthesis",
                symbol
            ),
            Self::RuntimeContextSymbolOutOfRange {
                symbol,
                value,
                allowed,
            } => write!(
                f,
                "runtime context symbol {}={} is out of domain (allowed {})",
                symbol, value, allowed
            ),
            Self::InvalidCalendarDate { day, mon, year } => write!(
                f,
                "invalid calendar date day={} mon={} year={}",
                day, mon, year
            ),
        }
    }
}

impl Error for SharedClimateRuntimeInputError {}

/// Build a shared climate runtime request from parser output.
pub fn build_climate_runtime_request(
    climate: &ClimateFile,
) -> Result<SharedClimateRuntimeRequest, SharedClimateRuntimeInputError> {
    let iclig = resolve_iclig(climate.datver)?;

    if climate.mode.itemp != 1 {
        return Err(SharedClimateRuntimeInputError::UnsupportedItemp {
            itemp: climate.mode.itemp,
        });
    }
    if climate.daily_records.is_empty() {
        return Err(SharedClimateRuntimeInputError::EmptyDailyRecords);
    }

    let mut daily_forcing = Vec::with_capacity(climate.daily_records.len());
    for record in &climate.daily_records {
        daily_forcing.push(adapt_daily_forcing(record, iclig)?);
    }

    Ok(SharedClimateRuntimeRequest {
        datver: climate.datver,
        iclig,
        itemp: climate.mode.itemp,
        ibrkpt: i32::from(climate.mode.breakpoint_enabled),
        iwind: climate.mode.iwind,
        station_id: climate.station_id.clone(),
        daily_forcing,
    })
}

/// Borrow one climate forcing day from a shared climate runtime request.
pub fn select_day_forcing(
    climate: &SharedClimateRuntimeRequest,
    day_index: usize,
) -> Result<&SharedClimateDailyForcing, SharedClimateRuntimeInputError> {
    climate
        .daily_forcing
        .get(day_index)
        .ok_or(SharedClimateRuntimeInputError::DayIndexOutOfRange {
            day_index,
            available: climate.daily_forcing.len(),
        })
}

fn adapt_daily_forcing(
    record: &ClimateDailyRecord,
    iclig: i32,
) -> Result<SharedClimateDailyForcing, SharedClimateRuntimeInputError> {
    match record {
        ClimateDailyRecord::NoBreakpoint(day) => Ok(SharedClimateDailyForcing::NoBreakpoint(
            adapt_no_breakpoint(day, iclig)?,
        )),
        ClimateDailyRecord::Breakpoint(day) => Ok(SharedClimateDailyForcing::Breakpoint(
            adapt_breakpoint(day)?,
        )),
    }
}

fn adapt_no_breakpoint(
    day: &NoBreakpointDay,
    iclig: i32,
) -> Result<SharedNoBreakpointForcing, SharedClimateRuntimeInputError> {
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
    let stmdur_h = day.stmdur.min(MAX_STORM_DURATION_HOURS);
    let stmdur = stmdur_h * HOURS_TO_SECONDS;
    if prcp > 0.0 && stmdur <= 0.0 {
        return Err(
            SharedClimateRuntimeInputError::PositivePrecipWithNonPositiveDuration { prcp, stmdur },
        );
    }

    let ip = if iclig == CLIGEN_POLICY_ICLIG {
        day.ip * CLIGEN_V4_IP_CORRECTION_FACTOR
    } else {
        day.ip
    };
    let event_shape = build_no_breakpoint_event_shape(prcp, stmdur, day.timep, ip)?;

    Ok(SharedNoBreakpointForcing {
        day: day.day,
        mon: day.mon,
        year: day.year,
        prcp,
        stmdur,
        timep: event_shape.timep,
        ip: event_shape.ip,
        ninten: event_shape.ninten,
        avrint: event_shape.avrint,
        mxint: event_shape.mxint,
        timem: event_shape.timem,
        intsty: event_shape.intsty,
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
) -> Result<SharedBreakpointForcing, SharedClimateRuntimeInputError> {
    require_finite("tmax", day.tmax)?;
    require_finite("tmin", day.tmin)?;
    require_finite("rad", day.rad)?;
    require_finite("vwind", day.vwind)?;
    require_finite("wind", day.wind)?;
    require_finite("tdpt", day.tdpt)?;

    if day.breakpoints.is_empty() {
        if day.nbrkpt == 0 {
            return Ok(SharedBreakpointForcing {
                day: day.day,
                mon: day.mon,
                year: day.year,
                nbrkpt: day.nbrkpt,
                stmstr: 0.0,
                prcp: 0.0,
                stmdur: 0.0,
                mxint: 0.0,
                timem: Vec::new(),
                intsty: Vec::new(),
                tmax: day.tmax,
                tmin: day.tmin,
                rad: day.rad,
                vwind: day.vwind,
                wind: day.wind,
                tdpt: day.tdpt,
            });
        }
        return Err(SharedClimateRuntimeInputError::EmptyBreakpointSeries);
    }
    let effective_breakpoint_count = day.nbrkpt.max(day.breakpoints.len());
    if effective_breakpoint_count > MAX_BREAKPOINTS_PER_DAY {
        return Err(
            SharedClimateRuntimeInputError::BreakpointCardinalityPolicyExceeded {
                value: effective_breakpoint_count,
                max: MAX_BREAKPOINTS_PER_DAY,
            },
        );
    }

    let stmstr = day
        .breakpoints
        .first()
        .map(|point| point.timem)
        .ok_or(SharedClimateRuntimeInputError::EmptyBreakpointSeries)?;

    let mut timem = Vec::with_capacity(day.breakpoints.len());
    let mut pptcum = Vec::with_capacity(day.breakpoints.len());
    for point in &day.breakpoints {
        require_non_negative("timem", point.timem)?;
        require_non_negative("pptcum", point.pptcum)?;
        timem.push((point.timem - stmstr) * HOURS_TO_SECONDS);
        pptcum.push(point.pptcum * MILLIMETERS_TO_METERS);
    }

    let mut intsty = vec![0.0; timem.len()];
    let mut stmdur = 0.0;
    let mut mxint = 0.0;
    for index in 1..timem.len() {
        let previous_time = timem[index - 1];
        let current_time = timem[index];
        if current_time <= previous_time {
            return Err(SharedClimateRuntimeInputError::NonMonotoneBreakpointTime {
                previous_s: previous_time,
                current_s: current_time,
            });
        }

        let drain = pptcum[index] - pptcum[index - 1];
        if drain < 0.0 {
            return Err(SharedClimateRuntimeInputError::NegativeField {
                field: "drain",
                value: drain,
            });
        }

        let delta_time_s = current_time - previous_time;
        let intensity = if drain == 0.0 {
            0.0
        } else {
            drain / delta_time_s
        };
        intsty[index - 1] = intensity;
        stmdur += delta_time_s;
        if intensity > mxint {
            mxint = intensity;
        }
    }

    let prcp = *pptcum
        .last()
        .ok_or(SharedClimateRuntimeInputError::EmptyBreakpointSeries)?;
    if prcp > 0.0 && stmdur <= 0.0 {
        return Err(
            SharedClimateRuntimeInputError::PositivePrecipWithNonPositiveDuration { prcp, stmdur },
        );
    }

    Ok(SharedBreakpointForcing {
        day: day.day,
        mon: day.mon,
        year: day.year,
        nbrkpt: day.nbrkpt,
        stmstr,
        prcp,
        stmdur,
        mxint,
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

#[allow(clippy::similar_names)]
fn build_no_breakpoint_event_shape(
    prcp_m: f64,
    stmdur_s: f64,
    timep: f64,
    ip: f64,
) -> Result<DisaggregatedEventShape, SharedClimateRuntimeInputError> {
    if prcp_m <= 0.0 || stmdur_s <= 0.0 {
        return Ok(DisaggregatedEventShape {
            timep,
            ip,
            ninten: 0,
            avrint: 0.0,
            mxint: 0.0,
            timem: Vec::new(),
            intsty: Vec::new(),
        });
    }

    let mut resolved_ip = ip.max(1.0);
    let mut resolved_timep = timep;
    if resolved_timep > 1.0 || (resolved_ip - 1.0).abs() <= 1e-12 {
        resolved_timep = 1.0;
    } else if resolved_timep <= 0.0 {
        resolved_timep = DISAG_MIN_TIMEP;
    }

    let (timedl, intdl) = build_disaggregation_shape(resolved_timep, resolved_ip, stmdur_s)?;
    let ninten = timedl.len();
    let avrint = prcp_m / stmdur_s;
    let mut timem = Vec::with_capacity(ninten);
    let mut intsty = Vec::with_capacity(ninten);
    for (t, i) in timedl.iter().zip(intdl.iter()) {
        timem.push(*t * stmdur_s);
        intsty.push(*i * prcp_m / stmdur_s);
    }

    for index in 1..timem.len() {
        let previous = timem[index - 1];
        let current = timem[index];
        if current <= previous {
            return Err(
                SharedClimateRuntimeInputError::DisaggregationTimeNotStrictlyIncreasing {
                    previous_s: previous,
                    current_s: current,
                },
            );
        }
    }

    let reconstructed_prcp_m = timem
        .windows(2)
        .zip(intsty.iter())
        .map(|(window, intensity)| (window[1] - window[0]) * *intensity)
        .sum::<f64>();
    if (reconstructed_prcp_m - prcp_m).abs() > DISAG_CLOSURE_TOLERANCE {
        return Err(
            SharedClimateRuntimeInputError::DisaggregationClosureResidual {
                expected_prcp_m: prcp_m,
                reconstructed_prcp_m,
            },
        );
    }

    let mxint = intsty.iter().copied().fold(0.0, f64::max);
    resolved_ip = resolved_ip.min(DISAG_MAX_IP);
    resolved_timep = resolved_timep.min(DISAG_MAX_TIMEP);

    Ok(DisaggregatedEventShape {
        timep: resolved_timep,
        ip: resolved_ip,
        ninten,
        avrint,
        mxint,
        timem,
        intsty,
    })
}

fn build_disaggregation_shape(
    timep: f64,
    ip: f64,
    duration_s: f64,
) -> Result<(Vec<f64>, Vec<f64>), SharedClimateRuntimeInputError> {
    let mut ninten = DISAG_DEFAULT_INTERVAL_COUNT;
    loop {
        if ninten <= 2 {
            return Ok((vec![0.0, 1.0], vec![1.0, 0.0]));
        }

        let (timedl, mut intdl) = if timep >= 1.0 && ip <= 1.0 {
            build_const_shape(ninten)
        } else {
            build_dblex_shape(ninten, timep, ip)?
        };
        intdl[ninten - 1] = 0.0;

        let minimum_spacing_ok = timedl
            .windows(2)
            .all(|window| (window[1] - window[0]) * duration_s >= DISAG_MIN_INTERVAL_SECONDS);
        if minimum_spacing_ok {
            return Ok((timedl, intdl));
        }

        ninten -= 1;
    }
}

#[allow(clippy::cast_precision_loss)]
fn build_const_shape(ninten: usize) -> (Vec<f64>, Vec<f64>) {
    let deltfq = 1.0 / (ninten as f64 - 1.0);
    let mut timedl = vec![0.0; ninten];
    let mut intdl = vec![0.0; ninten];
    let mut fqx = 0.0;
    for index in 1..ninten {
        fqx += deltfq;
        timedl[index] = fqx;
        intdl[index - 1] = 1.0;
    }
    intdl[ninten - 1] = 0.0;
    (timedl, intdl)
}

#[allow(clippy::cast_precision_loss)]
fn build_dblex_shape(
    ninten: usize,
    timep: f64,
    ip: f64,
) -> Result<(Vec<f64>, Vec<f64>), SharedClimateRuntimeInputError> {
    let ip = ip.min(DISAG_MAX_IP);
    let timep = timep.min(DISAG_MAX_TIMEP);
    let u = solve_eqroot(1.0 / ip)?;
    let b = u / timep;
    let a = ip * (-u).exp();
    let d = u / (1.0 - timep);
    let deltfq = 1.0 / (ninten as f64 - 1.0);

    let mut timedl = vec![0.0; ninten];
    timedl[ninten - 1] = 1.0;
    let mut intdl = vec![0.0; ninten];

    let mut fqx = 0.0;
    for index in 0..(ninten - 1) {
        let next = index + 1;
        if index < ninten - 2 {
            fqx += deltfq;
            timedl[next] = if fqx <= timep {
                (1.0 / b) * (1.0 + (b / a) * fqx).ln()
            } else {
                timep - (1.0 / d) * (1.0 - (d / ip) * (fqx - timep)).ln()
            };
        }

        let denominator = timedl[next] - timedl[index];
        intdl[index] = if denominator > 0.0 {
            deltfq / denominator
        } else {
            deltfq / 0.00001
        };
    }
    intdl[ninten - 1] = 0.0;
    Ok((timedl, intdl))
}

#[allow(clippy::many_single_char_names)]
fn solve_eqroot(a: f64) -> Result<f64, SharedClimateRuntimeInputError> {
    if !(a > 0.0 && a <= 1.0) {
        return Err(SharedClimateRuntimeInputError::DisaggregationRootSolveDomain { a });
    }

    if a <= 0.06 {
        return Ok(1.0 / a);
    }
    if a >= 1.0 {
        return Ok(0.0);
    }
    if a >= 0.999 {
        return Ok((3.0 / 2.0) - (6.0 * a - (15.0 / 4.0)).sqrt());
    }

    let mut u = if a <= 0.2 {
        1.0 / a
    } else if a <= 0.5 {
        (0.968_732 / a) - 1.550_98 * a + 0.431_653
    } else if a <= 0.94 {
        (1.132_43 / a) - 0.928_240 * a - 0.207_111
    } else {
        (3.0 / 2.0) - (6.0 * a - (15.0 / 4.0)).sqrt()
    };

    for _ in 0..32 {
        let e = (-u).exp();
        let f = (1.0 - e) / u;
        let d = a - f;
        let tmp = ((u + 1.0) * f) - 1.0;
        let r = a / tmp;
        let s = if r <= 1.0 {
            (d / a).abs()
        } else {
            (d / tmp).abs()
        };
        if s < DISAG_EQROOT_SOLVER_TOLERANCE {
            return Ok(u);
        }

        u *= 1.0 + d / (e - f);
    }

    Err(SharedClimateRuntimeInputError::DisaggregationRootSolveNonConvergent { a })
}

fn require_finite(field: &'static str, value: f64) -> Result<(), SharedClimateRuntimeInputError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(SharedClimateRuntimeInputError::NonFiniteField { field, value })
    }
}

fn require_non_negative(
    field: &'static str,
    value: f64,
) -> Result<(), SharedClimateRuntimeInputError> {
    require_finite(field, value)?;
    if value < 0.0 {
        Err(SharedClimateRuntimeInputError::NegativeField { field, value })
    } else {
        Ok(())
    }
}

fn resolve_iclig(datver: f64) -> Result<i32, SharedClimateRuntimeInputError> {
    require_finite("datver", datver)?;
    if datver.abs() <= DATVER_ZERO_TOLERANCE {
        Ok(CLIGEN_LEGACY_OVERRIDE_ICLIG)
    } else if datver >= CLIMATE_MIN_SUPPORTED_DATVER {
        Ok(CLIGEN_POLICY_ICLIG)
    } else {
        Err(SharedClimateRuntimeInputError::UnsupportedDatver { datver })
    }
}

#[derive(Debug, Clone, PartialEq)]
struct DisaggregatedEventShape {
    timep: f64,
    ip: f64,
    ninten: usize,
    avrint: f64,
    mxint: f64,
    timem: Vec<f64>,
    intsty: Vec<f64>,
}

#[cfg(test)]
mod tests {
    use std::fmt::Write as _;

    use openwepp_input_contract::parsers::climate::{
        ClimateDailyRecord, CompatibilityOptions, ParserMode as ClimateParserMode,
        parse_climate_from_str,
    };

    use super::{
        SharedClimateDailyForcing, SharedClimateRuntimeInputError, build_climate_runtime_request,
    };

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
    fn cqr21_shared_climate_runtime_input_error_characterizes_codes_and_display_strings() {
        let cases = vec![
            (
                SharedClimateRuntimeInputError::UnsupportedDatver { datver: 3.99 },
                "CLIM-RUNTIME-E-001",
                "CLIM-RUNTIME-E-001: unsupported climate datver 3.99 (supports datver=0.0 override or datver>=4.0)",
            ),
            (
                SharedClimateRuntimeInputError::UnsupportedItemp { itemp: 2 },
                "CLIM-RUNTIME-E-002",
                "CLIM-RUNTIME-E-002: unsupported climate itemp 2; only continuous-daily itemp=1 is supported",
            ),
            (
                SharedClimateRuntimeInputError::EmptyDailyRecords,
                "CLIM-RUNTIME-E-003",
                "CLIM-RUNTIME-E-003: climate parser output contains no daily forcing records",
            ),
            (
                SharedClimateRuntimeInputError::DayIndexOutOfRange {
                    day_index: 2,
                    available: 1,
                },
                "CLIM-RUNTIME-E-004",
                "CLIM-RUNTIME-E-004: requested day index 2 exceeds available climate records 1",
            ),
            (
                SharedClimateRuntimeInputError::NonFiniteField {
                    field: "tmax",
                    value: f64::NAN,
                },
                "CLIM-RUNTIME-E-005",
                "CLIM-RUNTIME-E-005: non-finite climate field tmax=NaN",
            ),
            (
                SharedClimateRuntimeInputError::NegativeField {
                    field: "prcp",
                    value: -1.0,
                },
                "CLIM-RUNTIME-E-006",
                "CLIM-RUNTIME-E-006: negative climate field prcp=-1",
            ),
            (
                SharedClimateRuntimeInputError::PositivePrecipWithNonPositiveDuration {
                    prcp: 0.1,
                    stmdur: 0.0,
                },
                "CLIM-RUNTIME-E-007",
                "CLIM-RUNTIME-E-007: positive precipitation 0.1 requires positive storm duration, got 0",
            ),
            (
                SharedClimateRuntimeInputError::EmptyBreakpointSeries,
                "CLIM-RUNTIME-E-008",
                "CLIM-RUNTIME-E-008: breakpoint forcing record contains zero breakpoint points",
            ),
            (
                SharedClimateRuntimeInputError::NonMonotoneBreakpointTime {
                    previous_s: 1.0,
                    current_s: 1.0,
                },
                "CLIM-RUNTIME-E-009",
                "CLIM-RUNTIME-E-009: breakpoint timem must be strictly increasing (1 -> 1)",
            ),
            (
                SharedClimateRuntimeInputError::BreakpointCardinalityPolicyExceeded {
                    value: 1_501,
                    max: 1_500,
                },
                "CLIM-RUNTIME-E-011",
                "CLIM-RUNTIME-E-011: breakpoint count 1501 exceeds runtime policy max 1500",
            ),
            (
                SharedClimateRuntimeInputError::BreakpointCountOutOfRange { value: 1_501 },
                "CLIM-RUNTIME-E-011",
                "CLIM-RUNTIME-E-011: breakpoint count 1501 exceeds supported conversion range",
            ),
            (
                SharedClimateRuntimeInputError::DisaggregationTimeNotStrictlyIncreasing {
                    previous_s: 2.0,
                    current_s: 1.0,
                },
                "CLIM-RUNTIME-E-012",
                "CLIM-RUNTIME-E-012: disaggregation time grid must be strictly increasing (2 -> 1)",
            ),
            (
                SharedClimateRuntimeInputError::DisaggregationRootSolveDomain { a: 0.0 },
                "CLIM-RUNTIME-E-013",
                "CLIM-RUNTIME-E-013: disaggregation root-solve domain invalid (a=0)",
            ),
            (
                SharedClimateRuntimeInputError::DisaggregationRootSolveNonConvergent { a: 0.3 },
                "CLIM-RUNTIME-E-014",
                "CLIM-RUNTIME-E-014: disaggregation root solve failed to converge (a=0.3)",
            ),
            (
                SharedClimateRuntimeInputError::DisaggregationClosureResidual {
                    expected_prcp_m: 0.1,
                    reconstructed_prcp_m: 0.2,
                },
                "CLIM-RUNTIME-E-015",
                "CLIM-RUNTIME-E-015: disaggregation closure residual exceeded tolerance (expected=0.1, reconstructed=0.2)",
            ),
            (
                SharedClimateRuntimeInputError::MissingRuntimeContextSymbol {
                    symbol: "snow.runtime_swe".to_string(),
                },
                "CLIM-RUNTIME-E-016",
                "CLIM-RUNTIME-E-016: missing required runtime context symbol snow.runtime_swe for active winter forcing synthesis",
            ),
            (
                SharedClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
                    symbol: "snow.options.rst".to_string(),
                    value: 2.0,
                    allowed: "0.0..=1.0",
                },
                "CLIM-RUNTIME-E-017",
                "CLIM-RUNTIME-E-017: runtime context symbol snow.options.rst=2 is out of domain (allowed 0.0..=1.0)",
            ),
            (
                SharedClimateRuntimeInputError::InvalidCalendarDate {
                    day: 31,
                    mon: 2,
                    year: 2000,
                },
                "CLIM-RUNTIME-E-018",
                "CLIM-RUNTIME-E-018: invalid calendar date day=31 mon=2 year=2000",
            ),
        ];

        for (error, expected_code, expected_display) in cases {
            assert_eq!(error.code(), expected_code);
            assert_eq!(error.to_string(), expected_display);
        }
    }

    #[test]
    fn runtime_request_accepts_breakpoint_cardinality_at_1500_boundary() {
        let climate =
            parse_climate_from_str(&build_breakpoint_fixture(1_500), ClimateParserMode::Strict)
                .expect("strict parser should accept 1500 breakpoint rows");

        let request = build_climate_runtime_request(&climate)
            .expect("runtime seam should accept 1500 breakpoint rows");

        let forcing = request
            .daily_forcing
            .first()
            .expect("one forcing day expected");
        match forcing {
            SharedClimateDailyForcing::Breakpoint(day) => {
                assert_eq!(day.nbrkpt, 1_500);
                assert_eq!(day.timem.len(), 1_500);
            }
            SharedClimateDailyForcing::NoBreakpoint(_) => panic!("expected breakpoint forcing"),
        }
    }

    #[test]
    fn runtime_request_accepts_breakpoint_zero_cardinality_dry_day() {
        let climate =
            parse_climate_from_str(&build_breakpoint_fixture(0), ClimateParserMode::Strict)
                .expect("strict parser should accept zero-breakpoint dry-day fixtures");

        let request = build_climate_runtime_request(&climate)
            .expect("runtime seam should accept breakpoint-mode dry-day fixtures");

        let forcing = request
            .daily_forcing
            .first()
            .expect("one forcing day expected");
        match forcing {
            SharedClimateDailyForcing::Breakpoint(day) => {
                assert_eq!(day.nbrkpt, 0);
                assert!(day.timem.is_empty());
                assert!(day.intsty.is_empty());
                assert!(day.stmstr.abs() < 1e-12);
                assert!(day.prcp.abs() < 1e-12);
                assert!(day.stmdur.abs() < 1e-12);
                assert!(day.mxint.abs() < 1e-12);
            }
            SharedClimateDailyForcing::NoBreakpoint(_) => panic!("expected breakpoint forcing"),
        }
    }

    #[test]
    fn runtime_request_rejects_breakpoint_cardinality_over_1500_even_with_parser_override() {
        let climate = parse_climate_from_str(
            &build_breakpoint_fixture(1_501),
            ClimateParserMode::Compatibility(CompatibilityOptions {
                allow_single_storm: false,
                allow_breakpoint_cardinality_override: true,
                allow_legacy_zero_drain_non_positive_dtime: false,
            }),
        )
        .expect("compat parser should allow >1500 breakpoint rows with explicit override");

        let error = build_climate_runtime_request(&climate)
            .expect_err("runtime seam must reject >1500 breakpoint rows");
        assert_eq!(error.code(), "CLIM-RUNTIME-E-011");
        assert!(matches!(
            error,
            SharedClimateRuntimeInputError::BreakpointCardinalityPolicyExceeded {
                value: 1_501,
                max: 1_500
            }
        ));
    }

    #[test]
    fn runtime_request_rejects_malformed_positive_cardinality_with_empty_series() {
        let mut climate =
            parse_climate_from_str(&build_breakpoint_fixture(2), ClimateParserMode::Strict)
                .expect("strict parser should accept fixture");
        let record = climate.daily_records.first_mut().expect("one forcing day");
        match record {
            ClimateDailyRecord::Breakpoint(day) => {
                day.breakpoints.clear();
            }
            ClimateDailyRecord::NoBreakpoint(_) => panic!("expected breakpoint forcing"),
        }

        let error = build_climate_runtime_request(&climate)
            .expect_err("positive-cardinality breakpoint day without rows must fail");
        assert_eq!(error.code(), "CLIM-RUNTIME-E-008");
        assert!(matches!(
            error,
            SharedClimateRuntimeInputError::EmptyBreakpointSeries
        ));
    }

    #[test]
    fn runtime_request_rejects_declared_cardinality_over_1500_when_rows_are_truncated() {
        let mut climate =
            parse_climate_from_str(&build_breakpoint_fixture(1_500), ClimateParserMode::Strict)
                .expect("strict parser should accept 1500 breakpoint rows");
        let record = climate.daily_records.first_mut().expect("one forcing day");
        match record {
            ClimateDailyRecord::Breakpoint(day) => {
                day.nbrkpt = 1_501;
            }
            ClimateDailyRecord::NoBreakpoint(_) => panic!("expected breakpoint forcing"),
        }

        let error = build_climate_runtime_request(&climate)
            .expect_err("runtime seam must enforce declared cardinality policy");
        assert_eq!(error.code(), "CLIM-RUNTIME-E-011");
        assert!(matches!(
            error,
            SharedClimateRuntimeInputError::BreakpointCardinalityPolicyExceeded {
                value: 1_501,
                max: 1_500
            }
        ));
    }

    #[test]
    fn runtime_request_rejects_non_monotone_breakpoint_times_with_e009() {
        let mut climate =
            parse_climate_from_str(&build_breakpoint_fixture(3), ClimateParserMode::Strict)
                .expect("strict parser should accept fixture");
        let record = climate.daily_records.first_mut().expect("one forcing day");
        match record {
            ClimateDailyRecord::Breakpoint(day) => {
                let first_timem = day
                    .breakpoints
                    .first()
                    .expect("first breakpoint point should exist")
                    .timem;
                day.breakpoints
                    .get_mut(1)
                    .expect("second breakpoint point should exist")
                    .timem = first_timem;
            }
            ClimateDailyRecord::NoBreakpoint(_) => panic!("expected breakpoint forcing"),
        }

        let error = build_climate_runtime_request(&climate)
            .expect_err("runtime seam must reject non-monotone breakpoint timem");
        assert_eq!(error.code(), "CLIM-RUNTIME-E-009");
        assert!(matches!(
            error,
            SharedClimateRuntimeInputError::NonMonotoneBreakpointTime { .. }
        ));
    }

    #[test]
    fn runtime_request_rejects_negative_breakpoint_drain_with_e006() {
        let mut climate =
            parse_climate_from_str(&build_breakpoint_fixture(3), ClimateParserMode::Strict)
                .expect("strict parser should accept fixture");
        let record = climate.daily_records.first_mut().expect("one forcing day");
        match record {
            ClimateDailyRecord::Breakpoint(day) => {
                day.breakpoints
                    .first_mut()
                    .expect("first breakpoint point should exist")
                    .pptcum = 0.02;
                day.breakpoints
                    .get_mut(1)
                    .expect("second breakpoint point should exist")
                    .pptcum = 0.01;
            }
            ClimateDailyRecord::NoBreakpoint(_) => panic!("expected breakpoint forcing"),
        }

        let error = build_climate_runtime_request(&climate)
            .expect_err("runtime seam must reject negative breakpoint drain");
        assert_eq!(error.code(), "CLIM-RUNTIME-E-006");
        assert!(matches!(
            error,
            SharedClimateRuntimeInputError::NegativeField {
                field: "drain",
                value
            } if value < 0.0
        ));
    }
}
