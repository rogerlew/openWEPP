use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use openwepp_climate_runtime_adapter::SharedClimateRuntimeRequest as WatershedHillslopeClimateRequest;
use openwepp_input_contract::parsers::{
    chaninp::ChaninpParseOutcome,
    climate::{ClimateMetadata, ClimateMonthlyStats},
};
use openwepp_kernel_contract::ClimateForcingSymbolSurface;

/// Typed errors for parser-to-watershed runtime surface adaptation.
#[derive(Debug, Clone, PartialEq)]
pub enum WatershedRuntimeInputError {
    ParseOutcomeNotRuntimeReady {
        observed: ChaninpParseOutcome,
    },
    MissingOptions,
    NonFiniteDtchrInput {
        value_s: f64,
    },
    NonPositiveDtchrInput {
        value_s: f64,
    },
    NonFiniteCbase {
        value: f64,
    },
    NegativeCbase {
        value: f64,
    },
    NonPositiveNtchr {
        value: i32,
    },
    ChannelCountOutOfRange {
        value: usize,
    },
    ChannelSymbolNonFinite {
        symbol: String,
        value: f64,
    },
    ChannelSymbolOutOfDomain {
        symbol: String,
        value: f64,
        rule: &'static str,
    },
    ImpoundmentSymbolNonFinite {
        symbol: String,
        value: f64,
    },
    ImpoundmentSymbolOutOfDomain {
        symbol: String,
        value: f64,
        rule: &'static str,
    },
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
            Self::ChannelSymbolNonFinite { .. } => "WS-RUNTIME-E-009",
            Self::ChannelSymbolOutOfDomain { .. } => "WS-RUNTIME-E-010",
            Self::ImpoundmentSymbolNonFinite { .. } => "WS-RUNTIME-E-011",
            Self::ImpoundmentSymbolOutOfDomain { .. } => "WS-RUNTIME-E-012",
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
            Self::ChannelSymbolNonFinite { symbol, value } => write!(
                f,
                "{}: channel runtime symbol {} is non-finite ({})",
                self.code(),
                symbol,
                value
            ),
            Self::ChannelSymbolOutOfDomain {
                symbol,
                value,
                rule,
            } => write!(
                f,
                "{}: channel runtime symbol {}={} violates {}",
                self.code(),
                symbol,
                value,
                rule
            ),
            Self::ImpoundmentSymbolNonFinite { symbol, value } => write!(
                f,
                "{}: impoundment runtime symbol {} is non-finite ({})",
                self.code(),
                symbol,
                value
            ),
            Self::ImpoundmentSymbolOutOfDomain {
                symbol,
                value,
                rule,
            } => write!(
                f,
                "{}: impoundment runtime symbol {}={} violates {}",
                self.code(),
                symbol,
                value,
                rule
            ),
        }
    }
}

impl Error for WatershedRuntimeInputError {}

/// Immutable watershed climate assignment payload keyed by hillslope id.
#[derive(Debug, Clone, PartialEq)]
pub struct WatershedClimateRuntimeRequest {
    pub hillslope_forcing: BTreeMap<u32, WatershedHillslopeClimateAssignment>,
}

/// Typed per-hillslope climate assignment with precomputed forcing-series
/// boundary projections.
#[derive(Debug, Clone, PartialEq)]
pub struct WatershedHillslopeClimateAssignment {
    forcing: WatershedHillslopeClimateRequest,
    metadata: ClimateMetadata,
    monthly: ClimateMonthlyStats,
    day_symbol_surfaces: Vec<ClimateForcingSymbolSurface>,
}

impl WatershedHillslopeClimateAssignment {
    #[inline]
    pub(crate) fn forcing(&self) -> &WatershedHillslopeClimateRequest {
        &self.forcing
    }

    #[inline]
    pub(crate) fn metadata(&self) -> &ClimateMetadata {
        &self.metadata
    }

    #[inline]
    pub(crate) fn monthly(&self) -> &ClimateMonthlyStats {
        &self.monthly
    }

    #[inline]
    pub(crate) fn day_symbol_surfaces(&self) -> &[ClimateForcingSymbolSurface] {
        &self.day_symbol_surfaces
    }

    pub(crate) fn new(
        forcing: WatershedHillslopeClimateRequest,
        metadata: ClimateMetadata,
        monthly: ClimateMonthlyStats,
        day_symbol_surfaces: Vec<ClimateForcingSymbolSurface>,
    ) -> Self {
        Self {
            forcing,
            metadata,
            monthly,
            day_symbol_surfaces,
        }
    }
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
    BreakpointCardinalityPolicyExceeded {
        hillslope_id: u32,
        value: usize,
        max: usize,
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
    MissingRuntimeContextSymbol {
        hillslope_id: u32,
        symbol: String,
    },
    RuntimeContextSymbolOutOfRange {
        hillslope_id: u32,
        symbol: String,
        value: f64,
        allowed: &'static str,
    },
    InvalidCalendarDate {
        hillslope_id: u32,
        day: i32,
        mon: i32,
        year: i32,
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
            // CLIM-RUNTIME-E-010 is intentionally retired by CLIM15 because
            // no reachable guard path can emit it under strict monotonic-time
            // policy.
            Self::BreakpointCardinalityPolicyExceeded { .. }
            | Self::BreakpointCountOutOfRange { .. } => "CLIM-RUNTIME-E-011",
            Self::EmptyClimateAssignments => "CLIM-RUNTIME-E-012",
            Self::DisaggregationTimeNotStrictlyIncreasing { .. } => "CLIM-RUNTIME-E-013",
            Self::DisaggregationRootSolveDomain { .. } => "CLIM-RUNTIME-E-014",
            Self::DisaggregationRootSolveNonConvergent { .. } => "CLIM-RUNTIME-E-015",
            Self::DisaggregationClosureResidual { .. } => "CLIM-RUNTIME-E-016",
            Self::MissingRuntimeContextSymbol { .. } => "CLIM-RUNTIME-E-017",
            Self::RuntimeContextSymbolOutOfRange { .. } => "CLIM-RUNTIME-E-018",
            Self::InvalidCalendarDate { .. } => "CLIM-RUNTIME-E-019",
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
            Self::BreakpointCardinalityPolicyExceeded {
                hillslope_id,
                value,
                max,
            } => write!(
                f,
                "{}: hillslope {} breakpoint count {} exceeds runtime policy max {}",
                self.code(),
                hillslope_id,
                value,
                max
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
            Self::MissingRuntimeContextSymbol {
                hillslope_id,
                symbol,
            } => write!(
                f,
                "{}: hillslope {} missing required runtime context symbol {}",
                self.code(),
                hillslope_id,
                symbol
            ),
            Self::RuntimeContextSymbolOutOfRange {
                hillslope_id,
                symbol,
                value,
                allowed,
            } => write!(
                f,
                "{}: hillslope {} runtime context symbol {}={} is out of domain (allowed {})",
                self.code(),
                hillslope_id,
                symbol,
                value,
                allowed
            ),
            Self::InvalidCalendarDate {
                hillslope_id,
                day,
                mon,
                year,
            } => write!(
                f,
                "{}: hillslope {} invalid calendar date day={} mon={} year={}",
                self.code(),
                hillslope_id,
                day,
                mon,
                year
            ),
        }
    }
}

impl Error for WatershedClimateRuntimeInputError {}
