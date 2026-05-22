use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use openwepp_climate_runtime_adapter::{
    SharedClimateDailyForcing as WatershedClimateDailyForcing, SharedClimateRuntimeInputError,
    SharedClimateRuntimeRequest as WatershedHillslopeClimateRequest, build_climate_runtime_request,
    select_day_forcing,
};
use openwepp_input_contract::parsers::{
    chaninp::{ChaninpFile, ChaninpParseOutcome},
    climate::ClimateFile,
};
use openwepp_kernel_contract::{BoundarySymbol, BoundaryValue};

use crate::WatershedWritebackSurface;

/// Typed errors for parser-to-watershed runtime surface adaptation.
#[derive(Debug, Clone, PartialEq)]
pub enum WatershedRuntimeInputError {
    ParseOutcomeNotRuntimeReady { observed: ChaninpParseOutcome },
    MissingOptions,
    NonFiniteDtchrInput { value_s: f64 },
    NonPositiveDtchrInput { value_s: f64 },
    NonFiniteCbase { value: f64 },
    NegativeCbase { value: f64 },
    NonPositiveNtchr { value: i32 },
    ChannelCountOutOfRange { value: usize },
}

impl WatershedRuntimeInputError {
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::ParseOutcomeNotRuntimeReady { .. } => "WS-RUNTIME-E-001",
            Self::MissingOptions => "WS-RUNTIME-E-002",
            Self::NonFiniteDtchrInput { .. } => "WS-RUNTIME-E-003",
            Self::NonPositiveDtchrInput { .. } => "WS-RUNTIME-E-004",
            Self::NonFiniteCbase { .. } => "WS-RUNTIME-E-005",
            Self::NegativeCbase { .. } => "WS-RUNTIME-E-006",
            Self::NonPositiveNtchr { .. } => "WS-RUNTIME-E-007",
            Self::ChannelCountOutOfRange { .. } => "WS-RUNTIME-E-008",
        }
    }
}

impl fmt::Display for WatershedRuntimeInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseOutcomeNotRuntimeReady { observed } => write!(
                f,
                "{}: chan.inp parse outcome {:?} is not runtime-consumable",
                self.code(),
                observed
            ),
            Self::MissingOptions => write!(
                f,
                "{}: chan.inp parse output is missing required options payload",
                self.code()
            ),
            Self::NonFiniteDtchrInput { value_s } => write!(
                f,
                "{}: non-finite dtchr_input_s value {}",
                self.code(),
                value_s
            ),
            Self::NonPositiveDtchrInput { value_s } => write!(
                f,
                "{}: non-positive dtchr_input_s value {}",
                self.code(),
                value_s
            ),
            Self::NonFiniteCbase { value } => {
                write!(f, "{}: non-finite cbase value {}", self.code(), value)
            }
            Self::NegativeCbase { value } => {
                write!(f, "{}: negative cbase value {}", self.code(), value)
            }
            Self::NonPositiveNtchr { value } => {
                write!(f, "{}: non-positive ntchr value {}", self.code(), value)
            }
            Self::ChannelCountOutOfRange { value } => write!(
                f,
                "{}: nchan value {} exceeds lossless conversion range",
                self.code(),
                value
            ),
        }
    }
}

impl Error for WatershedRuntimeInputError {}

/// Immutable watershed climate assignment payload keyed by hillslope id.
#[derive(Debug, Clone, PartialEq)]
pub struct WatershedClimateRuntimeRequest {
    pub hillslope_forcing: BTreeMap<u32, WatershedHillslopeClimateRequest>,
}

/// Typed climate runtime seam failures (`WS-CLIM-SEAM-001`).
#[derive(Debug, Clone, PartialEq)]
pub enum WatershedClimateRuntimeInputError {
    UnsupportedDatver {
        datver: f64,
    },
    UnsupportedItemp {
        itemp: i32,
    },
    EmptyDailyRecords {
        hillslope_id: u32,
    },
    DayIndexOutOfRange {
        hillslope_id: u32,
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
        hillslope_id: u32,
        prcp: f64,
        stmdur: f64,
    },
    EmptyBreakpointSeries {
        hillslope_id: u32,
    },
    NonMonotoneBreakpointTime {
        hillslope_id: u32,
        previous_s: f64,
        current_s: f64,
    },
    PositiveBreakpointDrainWithNonPositiveDeltaTime {
        hillslope_id: u32,
        drain_m: f64,
        delta_time_s: f64,
    },
    BreakpointCountOutOfRange {
        hillslope_id: u32,
        value: usize,
    },
    EmptyClimateAssignments,
    DisaggregationTimeNotStrictlyIncreasing {
        hillslope_id: u32,
        previous_s: f64,
        current_s: f64,
    },
    DisaggregationRootSolveDomain {
        hillslope_id: u32,
        a: f64,
    },
    DisaggregationRootSolveNonConvergent {
        hillslope_id: u32,
        a: f64,
    },
    DisaggregationClosureResidual {
        hillslope_id: u32,
        expected_prcp_m: f64,
        reconstructed_prcp_m: f64,
    },
}

impl WatershedClimateRuntimeInputError {
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::UnsupportedDatver { .. } => "CLIM-RUNTIME-E-001",
            Self::UnsupportedItemp { .. } => "CLIM-RUNTIME-E-002",
            Self::EmptyDailyRecords { .. } => "CLIM-RUNTIME-E-003",
            Self::DayIndexOutOfRange { .. } => "CLIM-RUNTIME-E-004",
            Self::NonFiniteField { .. } => "CLIM-RUNTIME-E-005",
            Self::NegativeField { .. } => "CLIM-RUNTIME-E-006",
            Self::PositivePrecipWithNonPositiveDuration { .. } => "CLIM-RUNTIME-E-007",
            Self::EmptyBreakpointSeries { .. } => "CLIM-RUNTIME-E-008",
            Self::NonMonotoneBreakpointTime { .. } => "CLIM-RUNTIME-E-009",
            Self::PositiveBreakpointDrainWithNonPositiveDeltaTime { .. } => "CLIM-RUNTIME-E-010",
            Self::BreakpointCountOutOfRange { .. } => "CLIM-RUNTIME-E-011",
            Self::EmptyClimateAssignments => "CLIM-RUNTIME-E-012",
            Self::DisaggregationTimeNotStrictlyIncreasing { .. } => "CLIM-RUNTIME-E-013",
            Self::DisaggregationRootSolveDomain { .. } => "CLIM-RUNTIME-E-014",
            Self::DisaggregationRootSolveNonConvergent { .. } => "CLIM-RUNTIME-E-015",
            Self::DisaggregationClosureResidual { .. } => "CLIM-RUNTIME-E-016",
        }
    }
}

impl fmt::Display for WatershedClimateRuntimeInputError {
    #[allow(clippy::too_many_lines)]
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
            Self::EmptyDailyRecords { hillslope_id } => write!(
                f,
                "{}: hillslope {} has no climate daily records",
                self.code(),
                hillslope_id
            ),
            Self::DayIndexOutOfRange {
                hillslope_id,
                day_index,
                available,
            } => write!(
                f,
                "{}: hillslope {} requested day index {} exceeds available records {}",
                self.code(),
                hillslope_id,
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
            Self::PositivePrecipWithNonPositiveDuration {
                hillslope_id,
                prcp,
                stmdur,
            } => write!(
                f,
                "{}: hillslope {} has positive precipitation {} with non-positive storm duration {}",
                self.code(),
                hillslope_id,
                prcp,
                stmdur
            ),
            Self::EmptyBreakpointSeries { hillslope_id } => write!(
                f,
                "{}: hillslope {} breakpoint forcing record contains zero points",
                self.code(),
                hillslope_id
            ),
            Self::NonMonotoneBreakpointTime {
                hillslope_id,
                previous_s,
                current_s,
            } => write!(
                f,
                "{}: hillslope {} breakpoint timem must be strictly increasing ({} -> {})",
                self.code(),
                hillslope_id,
                previous_s,
                current_s
            ),
            Self::PositiveBreakpointDrainWithNonPositiveDeltaTime {
                hillslope_id,
                drain_m,
                delta_time_s,
            } => write!(
                f,
                "{}: hillslope {} has positive breakpoint drain {} with non-positive elapsed time {}",
                self.code(),
                hillslope_id,
                drain_m,
                delta_time_s
            ),
            Self::BreakpointCountOutOfRange {
                hillslope_id,
                value,
            } => write!(
                f,
                "{}: hillslope {} breakpoint count {} exceeds supported conversion range",
                self.code(),
                hillslope_id,
                value
            ),
            Self::EmptyClimateAssignments => write!(
                f,
                "{}: no hillslope climate assignments supplied for watershed runtime seam",
                self.code()
            ),
            Self::DisaggregationTimeNotStrictlyIncreasing {
                hillslope_id,
                previous_s,
                current_s,
            } => write!(
                f,
                "{}: hillslope {} disaggregation timem must be strictly increasing ({} -> {})",
                self.code(),
                hillslope_id,
                previous_s,
                current_s
            ),
            Self::DisaggregationRootSolveDomain { hillslope_id, a } => write!(
                f,
                "{}: hillslope {} has disaggregation root-solve input outside 0<a<=1 (a={})",
                self.code(),
                hillslope_id,
                a
            ),
            Self::DisaggregationRootSolveNonConvergent { hillslope_id, a } => write!(
                f,
                "{}: hillslope {} disaggregation root-solve did not converge for a={}",
                self.code(),
                hillslope_id,
                a
            ),
            Self::DisaggregationClosureResidual {
                hillslope_id,
                expected_prcp_m,
                reconstructed_prcp_m,
            } => write!(
                f,
                "{}: hillslope {} disaggregation closure residual exceeded tolerance (expected {}, reconstructed {})",
                self.code(),
                hillslope_id,
                expected_prcp_m,
                reconstructed_prcp_m
            ),
        }
    }
}

impl Error for WatershedClimateRuntimeInputError {}

/// Build an orchestrator-owned watershed runtime surface from parsed chan.inp
/// output.
///
/// This seam is strict by design: only parsed-branch outcomes are runtime
/// consumable.
///
/// # Errors
///
/// Returns `WatershedRuntimeInputError` when parse outcome/options are not
/// runtime-ready or required numeric values are invalid.
pub fn build_watershed_runtime_surface_from_chaninp(
    chaninp: &ChaninpFile,
) -> Result<WatershedWritebackSurface, WatershedRuntimeInputError> {
    if chaninp.parse_outcome != ChaninpParseOutcome::ParsedBranch {
        return Err(WatershedRuntimeInputError::ParseOutcomeNotRuntimeReady {
            observed: chaninp.parse_outcome,
        });
    }

    let options = chaninp
        .options
        .as_ref()
        .ok_or(WatershedRuntimeInputError::MissingOptions)?;

    let dtchr_input_s = options.dtchr_input_s;
    if !dtchr_input_s.is_finite() {
        return Err(WatershedRuntimeInputError::NonFiniteDtchrInput {
            value_s: dtchr_input_s,
        });
    }
    if dtchr_input_s <= 0.0 {
        return Err(WatershedRuntimeInputError::NonPositiveDtchrInput {
            value_s: dtchr_input_s,
        });
    }

    let cbase = options.cbase_m3_s_m2;
    if !cbase.is_finite() {
        return Err(WatershedRuntimeInputError::NonFiniteCbase { value: cbase });
    }
    if cbase < 0.0 {
        return Err(WatershedRuntimeInputError::NegativeCbase { value: cbase });
    }

    if options.ntchr <= 0 {
        return Err(WatershedRuntimeInputError::NonPositiveNtchr {
            value: options.ntchr,
        });
    }
    let nchan = u32::try_from(chaninp.nchan).map_err(|_| {
        WatershedRuntimeInputError::ChannelCountOutOfRange {
            value: chaninp.nchan,
        }
    })?;

    let mut state_surface = BTreeMap::new();
    state_surface.insert(
        BoundarySymbol::from("ipeak"),
        BoundaryValue::scalar(f64::from(chaninp.ipeak)),
    );
    state_surface.insert(
        BoundarySymbol::from("nchan"),
        BoundaryValue::scalar(f64::from(nchan)),
    );
    state_surface.insert(
        BoundarySymbol::from("dtchr"),
        BoundaryValue::scalar(dtchr_input_s),
    );
    state_surface.insert(
        BoundarySymbol::from("ntchr"),
        BoundaryValue::scalar(f64::from(options.ntchr)),
    );
    state_surface.insert(
        BoundarySymbol::from("nchnum"),
        BoundaryValue::scalar(f64::from(options.nchnum_norm)),
    );

    let mut flux_surface = BTreeMap::new();
    flux_surface.insert(BoundarySymbol::from("cbase"), BoundaryValue::scalar(cbase));

    Ok(WatershedWritebackSurface {
        state_surface,
        flux_surface,
    })
}

/// Build a watershed climate runtime request from per-hillslope parser outputs
/// (`WS-CLIM-SEAM-001`).
///
/// Runtime policy at this seam enforces `datver=0.0` override (`iclig=0`) or
/// `datver>=4.0` (`iclig=1`).
///
/// # Errors
///
/// Returns `WatershedClimateRuntimeInputError` when assignments or per-hillslope
/// climate payloads violate seam policy/invariants.
pub fn build_watershed_climate_runtime_request_from_assignments(
    assignments: &BTreeMap<u32, ClimateFile>,
) -> Result<WatershedClimateRuntimeRequest, WatershedClimateRuntimeInputError> {
    if assignments.is_empty() {
        return Err(WatershedClimateRuntimeInputError::EmptyClimateAssignments);
    }

    let mut hillslope_forcing = BTreeMap::new();
    for (&hillslope_id, climate) in assignments {
        hillslope_forcing.insert(
            hillslope_id,
            build_climate_runtime_request(climate)
                .map_err(|error| map_shared_error_for_hillslope(hillslope_id, &error))?,
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

    for (&hillslope_id, request) in &climate.hillslope_forcing {
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

        let forcing = select_day_forcing(request, day_index)
            .map_err(|error| map_shared_error_for_hillslope(hillslope_id, &error))?;

        match forcing {
            WatershedClimateDailyForcing::NoBreakpoint(day) => {
                insert_hillslope_common_day_symbols(
                    state_surface,
                    hillslope_id,
                    day.day,
                    day.mon,
                    day.year,
                );
                insert_hillslope_symbol(state_surface, hillslope_id, "prcp", day.prcp);
                insert_hillslope_symbol(state_surface, hillslope_id, "stmdur", day.stmdur);
                insert_hillslope_symbol(state_surface, hillslope_id, "timep", day.timep);
                insert_hillslope_symbol(state_surface, hillslope_id, "ip", day.ip);
                let ninten = u32::try_from(day.ninten).map_err(|_| {
                    WatershedClimateRuntimeInputError::BreakpointCountOutOfRange {
                        hillslope_id,
                        value: day.ninten,
                    }
                })?;
                insert_hillslope_symbol(state_surface, hillslope_id, "ninten", f64::from(ninten));
                insert_hillslope_symbol(state_surface, hillslope_id, "avrint", day.avrint);
                insert_hillslope_symbol(state_surface, hillslope_id, "mxint", day.mxint);
                insert_hillslope_symbol(state_surface, hillslope_id, "tmax", day.tmax);
                insert_hillslope_symbol(state_surface, hillslope_id, "tmin", day.tmin);
                insert_hillslope_symbol(state_surface, hillslope_id, "rad", day.rad);
                insert_hillslope_symbol(state_surface, hillslope_id, "vwind", day.vwind);
                insert_hillslope_symbol(state_surface, hillslope_id, "wind", day.wind);
                insert_hillslope_symbol(state_surface, hillslope_id, "tdpt", day.tdpt);
                for (index, value) in day.timem.iter().enumerate() {
                    let symbol = format!("timem_{:04}", index + 1);
                    insert_hillslope_symbol(state_surface, hillslope_id, &symbol, *value);
                }
                for (index, value) in day.intsty.iter().enumerate() {
                    let symbol = format!("intsty_{:04}", index + 1);
                    insert_hillslope_symbol(state_surface, hillslope_id, &symbol, *value);
                }
            }
            WatershedClimateDailyForcing::Breakpoint(day) => {
                insert_hillslope_common_day_symbols(
                    state_surface,
                    hillslope_id,
                    day.day,
                    day.mon,
                    day.year,
                );
                insert_hillslope_symbol(state_surface, hillslope_id, "stmstr", day.stmstr);
                insert_hillslope_symbol(state_surface, hillslope_id, "prcp", day.prcp);
                insert_hillslope_symbol(state_surface, hillslope_id, "stmdur", day.stmdur);
                insert_hillslope_symbol(state_surface, hillslope_id, "mxint", day.mxint);
                insert_hillslope_symbol(state_surface, hillslope_id, "tmax", day.tmax);
                insert_hillslope_symbol(state_surface, hillslope_id, "tmin", day.tmin);
                insert_hillslope_symbol(state_surface, hillslope_id, "rad", day.rad);
                insert_hillslope_symbol(state_surface, hillslope_id, "vwind", day.vwind);
                insert_hillslope_symbol(state_surface, hillslope_id, "wind", day.wind);
                insert_hillslope_symbol(state_surface, hillslope_id, "tdpt", day.tdpt);

                let nbrkpt = u32::try_from(day.nbrkpt).map_err(|_| {
                    WatershedClimateRuntimeInputError::BreakpointCountOutOfRange {
                        hillslope_id,
                        value: day.nbrkpt,
                    }
                })?;
                insert_hillslope_symbol(state_surface, hillslope_id, "nbrkpt", f64::from(nbrkpt));

                for (index, value) in day.timem.iter().enumerate() {
                    let symbol = format!("timem_{:04}", index + 1);
                    insert_hillslope_symbol(state_surface, hillslope_id, &symbol, *value);
                }
                for (index, value) in day.intsty.iter().enumerate() {
                    let symbol = format!("intsty_{:04}", index + 1);
                    insert_hillslope_symbol(state_surface, hillslope_id, &symbol, *value);
                }
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
        SharedClimateRuntimeInputError::PositiveBreakpointDrainWithNonPositiveDeltaTime {
            drain_m,
            delta_time_s,
        } => WatershedClimateRuntimeInputError::PositiveBreakpointDrainWithNonPositiveDeltaTime {
            hillslope_id,
            drain_m: *drain_m,
            delta_time_s: *delta_time_s,
        },
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

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use openwepp_input_contract::parsers::{
        chaninp::{ChaninpParseOptions, ParseMode, parse_chaninp_from_str},
        climate::{CompatibilityOptions, ParserMode as ClimateParserMode, parse_climate_from_str},
    };
    use openwepp_kernel_contract::BoundarySymbol;

    use super::{
        WatershedClimateRuntimeInputError, WatershedRuntimeInputError,
        build_watershed_runtime_surface_from_chaninp,
        build_watershed_runtime_surface_from_climate_assignments,
    };

    const STRICT_VALID_CLIMATE: &str =
        include_str!("../../../tests/fixtures/infile/climate/strict_valid.cli");
    const LEGACY_DATVER_CLIMATE: &str =
        include_str!("../../../tests/fixtures/infile/climate/legacy_datver_0.cli");
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
    const STRICT_VALID_CHANINP: &str =
        include_str!("../../../tests/fixtures/infile/chaninp/strict_valid.chaninp");

    #[test]
    fn chaninp_runtime_surface_contains_required_symbols() {
        let valid_channel_element_ids = BTreeSet::from([4, 5]);
        let parsed = parse_chaninp_from_str(
            STRICT_VALID_CHANINP,
            ChaninpParseOptions::strict(3, 2),
            &valid_channel_element_ids,
        )
        .expect("strict chan.inp fixture should parse");

        let surface = build_watershed_runtime_surface_from_chaninp(&parsed)
            .expect("runtime surface should build from strict parsed branch");

        let dtchr = surface
            .state_surface
            .get(&BoundarySymbol::from("dtchr"))
            .expect("dtchr should be present")
            .as_f64();
        let ntchr = surface
            .state_surface
            .get(&BoundarySymbol::from("ntchr"))
            .expect("ntchr should be present")
            .as_f64();
        let cbase = surface
            .flux_surface
            .get(&BoundarySymbol::from("cbase"))
            .expect("cbase should be present")
            .as_f64();

        assert!((dtchr - 600.0).abs() < 1e-12);
        assert!((ntchr - 144.0).abs() < 1e-12);
        assert!((cbase - 0.000_001).abs() < 1e-12);
    }

    #[test]
    fn chaninp_runtime_surface_rejects_compat_defaulted_parse_outcome() {
        let valid_channel_element_ids = BTreeSet::from([4, 5]);
        let parsed = parse_chaninp_from_str(
            "invalid\nbranch\nfor\nstrict",
            ChaninpParseOptions {
                mode: ParseMode::Compatibility,
                ..ChaninpParseOptions::compatibility(3, 2)
            },
            &valid_channel_element_ids,
        )
        .expect("compat parser should return defaulted branch instead of hard failure");

        let error = build_watershed_runtime_surface_from_chaninp(&parsed)
            .expect_err("defaulted compat branch is not runtime consumable");
        assert_eq!(error.code(), "WS-RUNTIME-E-001");
        assert!(matches!(
            error,
            WatershedRuntimeInputError::ParseOutcomeNotRuntimeReady { .. }
        ));
    }

    #[test]
    fn climate_runtime_surface_contains_per_hillslope_symbols() {
        let climate = parse_climate_from_str(STRICT_VALID_CLIMATE, ClimateParserMode::Strict)
            .expect("strict climate fixture should parse");
        let assignments = BTreeMap::from([(1_u32, climate.clone()), (3_u32, climate)]);

        let surface = build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
            .expect("watershed climate runtime surface should build");

        let nclimhs = surface
            .state_surface
            .get(&BoundarySymbol::from("nclimhs"))
            .expect("nclimhs should be present")
            .as_f64();
        let hs1_prcp = surface
            .state_surface
            .get(&BoundarySymbol::from("hs1_prcp"))
            .expect("hs1_prcp should be present")
            .as_f64();
        let hs3_stmdur = surface
            .state_surface
            .get(&BoundarySymbol::from("hs3_stmdur"))
            .expect("hs3_stmdur should be present")
            .as_f64();
        let hs1_ip = surface
            .state_surface
            .get(&BoundarySymbol::from("hs1_ip"))
            .expect("hs1_ip should be present")
            .as_f64();
        let hs1_ninten = surface
            .state_surface
            .get(&BoundarySymbol::from("hs1_ninten"))
            .expect("hs1_ninten should be present")
            .as_f64();

        assert!((nclimhs - 2.0).abs() < 1e-12);
        assert!((hs1_prcp - 0.01).abs() < 1e-12);
        assert!((hs3_stmdur - 7_200.0).abs() < 1e-12);
        assert!((hs1_ip - 2.1).abs() < 1e-12);
        assert!(hs1_ninten >= 2.0);
    }

    #[test]
    fn breakpoint_runtime_surface_projects_stmstr_elapsed_timem_and_mxint() {
        let climate =
            parse_climate_from_str(WC1_BREAKPOINT_STMSTR_NONZERO, ClimateParserMode::Strict)
                .expect("curated wc1 breakpoint fixture should parse");
        let assignments = BTreeMap::from([(21_u32, climate)]);
        let surface = build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
            .expect("breakpoint runtime assignments should build");

        let stmstr = surface
            .state_surface
            .get(&BoundarySymbol::from("hs21_stmstr"))
            .expect("hs21_stmstr should exist")
            .as_f64();
        let prcp = surface
            .state_surface
            .get(&BoundarySymbol::from("hs21_prcp"))
            .expect("hs21_prcp should exist")
            .as_f64();
        let stmdur = surface
            .state_surface
            .get(&BoundarySymbol::from("hs21_stmdur"))
            .expect("hs21_stmdur should exist")
            .as_f64();
        let mxint = surface
            .state_surface
            .get(&BoundarySymbol::from("hs21_mxint"))
            .expect("hs21_mxint should exist")
            .as_f64();
        let timem_1 = surface
            .state_surface
            .get(&BoundarySymbol::from("hs21_timem_0001"))
            .expect("hs21_timem_0001 should exist")
            .as_f64();
        let timem_2 = surface
            .state_surface
            .get(&BoundarySymbol::from("hs21_timem_0002"))
            .expect("hs21_timem_0002 should exist")
            .as_f64();
        let intsty_5 = surface
            .state_surface
            .get(&BoundarySymbol::from("hs21_intsty_0005"))
            .expect("hs21_intsty_0005 should exist")
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
        let assignments = BTreeMap::from([(22_u32, climate)]);
        let surface = build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
            .expect("42-point breakpoint assignments should build");

        let nbrkpt = surface
            .state_surface
            .get(&BoundarySymbol::from("hs22_nbrkpt"))
            .expect("hs22_nbrkpt should exist")
            .as_f64();
        let timem_first = surface
            .state_surface
            .get(&BoundarySymbol::from("hs22_timem_0001"))
            .expect("hs22_timem_0001 should exist")
            .as_f64();
        let timem_last = surface
            .state_surface
            .get(&BoundarySymbol::from("hs22_timem_0042"))
            .expect("hs22_timem_0042 should exist")
            .as_f64();
        let intsty_last = surface
            .state_surface
            .get(&BoundarySymbol::from("hs22_intsty_0042"))
            .expect("hs22_intsty_0042 should exist")
            .as_f64();

        assert!((nbrkpt - 42.0).abs() < 1e-12);
        assert!(timem_first.abs() < 1e-12);
        assert!(timem_last > timem_first);
        assert!(intsty_last.abs() < 1e-12);
    }

    #[test]
    fn climate_runtime_surface_rejects_empty_assignment_map() {
        let assignments: BTreeMap<u32, openwepp_input_contract::parsers::climate::ClimateFile> =
            BTreeMap::new();
        let error = build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
            .expect_err("empty assignment map must fail");

        assert_eq!(error.code(), "CLIM-RUNTIME-E-012");
        assert!(matches!(
            error,
            WatershedClimateRuntimeInputError::EmptyClimateAssignments
        ));
    }

    #[test]
    fn climate_runtime_surface_supports_explicit_datver_zero_override() {
        let legacy = parse_climate_from_str(LEGACY_DATVER_CLIMATE, ClimateParserMode::Strict)
            .expect("legacy datver fixture should parse");
        let assignments = BTreeMap::from([(5_u32, legacy)]);

        let surface = build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
            .expect("datver=0.0 override should be accepted");
        let iclig = surface
            .state_surface
            .get(&BoundarySymbol::from("hs5_iclig"))
            .expect("hs5_iclig should exist")
            .as_f64();
        let ip = surface
            .state_surface
            .get(&BoundarySymbol::from("hs5_ip"))
            .expect("hs5_ip should exist")
            .as_f64();
        assert!((iclig - 0.0).abs() < 1e-12);
        assert!((ip - 2.0).abs() < 1e-12);
    }

    #[test]
    fn climate_runtime_surface_applies_timep_floor_for_wet_nonconstant_events() {
        let climate = parse_climate_from_str(WC1_CANOGA_DAY1, ClimateParserMode::Strict)
            .expect("wc1 fixture should parse");
        let assignments = BTreeMap::from([(11_u32, climate)]);

        let surface = build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
            .expect("wc1 runtime assignments should build");
        let timep = surface
            .state_surface
            .get(&BoundarySymbol::from("hs11_timep"))
            .expect("hs11_timep should exist")
            .as_f64();
        let ip = surface
            .state_surface
            .get(&BoundarySymbol::from("hs11_ip"))
            .expect("hs11_ip should exist")
            .as_f64();
        assert!((timep - 0.01).abs() < 1e-12);
        assert!((ip - 2.94).abs() < 1e-12);
    }

    #[test]
    fn climate_runtime_surface_caps_storm_duration_to_23_999_hours() {
        let climate = parse_climate_from_str(WC1_CANOGA_STMDUR_CAP, ClimateParserMode::Strict)
            .expect("wc1 duration-cap fixture should parse");
        let assignments = BTreeMap::from([(12_u32, climate)]);

        let surface = build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
            .expect("duration-cap assignments should build");
        let stmdur = surface
            .state_surface
            .get(&BoundarySymbol::from("hs12_stmdur"))
            .expect("hs12_stmdur should exist")
            .as_f64();
        let ip = surface
            .state_surface
            .get(&BoundarySymbol::from("hs12_ip"))
            .expect("hs12_ip should exist")
            .as_f64();
        assert!((stmdur - (23.999 * 3_600.0)).abs() < 1e-9);
        assert!((ip - 22.589).abs() < 1e-12);
    }

    #[test]
    fn climate_runtime_surface_rejects_pre4_nonzero_datver_branch() {
        let mut climate = parse_climate_from_str(STRICT_VALID_CLIMATE, ClimateParserMode::Strict)
            .expect("strict climate fixture should parse");
        climate.datver = 3.9;
        let assignments = BTreeMap::from([(7_u32, climate)]);

        let error = build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
            .expect_err("pre-4 nonzero datver must fail at watershed seam");
        assert_eq!(error.code(), "CLIM-RUNTIME-E-001");
        assert!(matches!(
            error,
            WatershedClimateRuntimeInputError::UnsupportedDatver { datver } if (datver - 3.9).abs() < 1e-12
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
        let assignments = BTreeMap::from([(2_u32, climate)]);

        let error = build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
            .expect_err("duplicate breakpoint timem must fail seam guard");
        assert_eq!(error.code(), "CLIM-RUNTIME-E-009");
        assert!(matches!(
            error,
            WatershedClimateRuntimeInputError::NonMonotoneBreakpointTime { .. }
        ));
    }
}
