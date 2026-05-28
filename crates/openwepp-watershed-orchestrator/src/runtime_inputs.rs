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
    climate::{ClimateFile, ClimateMonthlyStats},
    slope::{DistanceMode, SlopeProfile},
    watershed_channel::WatershedChannelFile,
    watershed_impoundment::{
        CulvertPayload, EmergencySpillwayPayload, ImpoundmentRecord, WatershedImpoundmentFile,
    },
};
use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, ClimateForcingSymbolSurface, ClimateForcingSymbolSurfaceError,
    WatershedProductionStateSymbol,
};

use crate::WatershedWritebackSurface;

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

type Ws12ImpoundmentProjectionTuple = (&'static str, f64, Option<f64>, bool);

const STANDARD_GRAVITY_M_S2: f64 = 9.806_65;
const ACTIVE_PROJECTION_STAGE_DELTA_M: f64 = 0.01;
const EMERGENCY_OPEN_CHANNEL_WEIR_COEFFICIENT: f64 = 3.087;
const WS12_FUNCTION_COUNT: usize = 15;
const WS17_METERS_TO_FEET: f64 = 3.281;
const WS34_MANNING_RELATION_TOLERANCE: f64 = 1.0e-9;

#[derive(Debug, Clone, Copy)]
struct Ws12ActiveProjection {
    drop_coefficient: f64,
    drop_exponent: f64,
    culvert_coefficient: f64,
    culvert_exponent: f64,
    riser_coefficient: f64,
    drop_threshold: f64,
    culvert_threshold: f64,
    riser_threshold: f64,
}

#[derive(Debug, Clone)]
struct Ws12OutflowFunctionFamilies {
    a: [f64; WS12_FUNCTION_COUNT],
    b: [f64; WS12_FUNCTION_COUNT],
    c: [f64; WS12_FUNCTION_COUNT],
    d: [f64; WS12_FUNCTION_COUNT],
    e: [f64; WS12_FUNCTION_COUNT],
    ha: [f64; WS12_FUNCTION_COUNT],
}

impl Ws12OutflowFunctionFamilies {
    fn inactive_default(hfull: f64) -> Self {
        Self {
            a: [0.0; WS12_FUNCTION_COUNT],
            b: [0.0; WS12_FUNCTION_COUNT],
            c: [0.0; WS12_FUNCTION_COUNT],
            d: [0.0; WS12_FUNCTION_COUNT],
            e: [0.0; WS12_FUNCTION_COUNT],
            ha: [hfull; WS12_FUNCTION_COUNT],
        }
    }

    fn coefficient_at(&self, family_index: usize, suffix: &'static str) -> f64 {
        let index = family_index - 1;
        match suffix {
            "a" => self.a[index],
            "b" => self.b[index],
            "c" => self.c[index],
            "d" => self.d[index],
            "e" => self.e[index],
            "ha" => self.ha[index],
            _ => unreachable!("unsupported coefficient suffix"),
        }
    }
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
    monthly: ClimateMonthlyStats,
    day_symbol_surfaces: Vec<ClimateForcingSymbolSurface>,
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
        BoundarySymbol::from(WatershedProductionStateSymbol::Ipeak),
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

/// Seed WS10 channel runtime symbols from parsed watershed channel input.
///
/// Inserts per-channel production-kernel controls:
/// - `ws10_channel_{id}_chnn`
/// - `ws10_channel_{id}_ctlslp`
/// - `ws10_channel_{id}_chnk`
/// - `ws10_channel_{id}_ishape`
/// - `ws10_channel_{id}_ienslp`
/// - `ws10_channel_{id}_icntrl`
/// - `ws10_channel_{id}_flgout`
/// - `ws10_channel_{id}_chnz`
/// - `ws10_channel_{id}_chnnbr`
/// - `ws10_channel_{id}_chntcr`
/// - `ws10_channel_{id}_chnedm`
/// - `ws10_channel_{id}_chneds`
/// - `ws10_channel_{id}_ctlz`
/// - `ws10_channel_{id}_ctln`
/// - `ws10_channel_{id}_rccoef` (`icntrl==4` only)
/// - `ws10_channel_{id}_rcexp` (`icntrl==4` only)
/// - `ws10_channel_{id}_rcoset` (`icntrl==4` only)
///
/// # Errors
///
/// Returns `WatershedRuntimeInputError` when required symbols are non-finite or
/// violate declared domains.
#[allow(clippy::similar_names)]
pub fn seed_watershed_runtime_surface_from_watershed_channel(
    runtime_surface: &mut WatershedWritebackSurface,
    channel: &WatershedChannelFile,
) -> Result<(), WatershedRuntimeInputError> {
    for definition in &channel.channels {
        let node_id = definition.channel_id;
        if !(1..=3).contains(&definition.ishape) {
            return Err(WatershedRuntimeInputError::ChannelSymbolOutOfDomain {
                symbol: format!("ws10_channel_{node_id}_ishape"),
                value: f64::from(definition.ishape),
                rule: "ishape must be within [1,3] (1=rectangular, 2=triangular, 3=naturally eroded)",
            });
        }
        if !(1..=2).contains(&definition.ienslp) {
            return Err(WatershedRuntimeInputError::ChannelSymbolOutOfDomain {
                symbol: format!("ws10_channel_{node_id}_ienslp"),
                value: f64::from(definition.ienslp),
                rule: "ienslp must be within [1,2]",
            });
        }
        if !(0..=4).contains(&definition.icntrl) {
            return Err(WatershedRuntimeInputError::ChannelSymbolOutOfDomain {
                symbol: format!("ws10_channel_{node_id}_icntrl"),
                value: f64::from(definition.icntrl),
                rule: "icntrl must be within [0,4]",
            });
        }
        if !(0..=1).contains(&definition.flgout) {
            return Err(WatershedRuntimeInputError::ChannelSymbolOutOfDomain {
                symbol: format!("ws10_channel_{node_id}_flgout"),
                value: f64::from(definition.flgout),
                rule: "flgout must be within [0,1]",
            });
        }
        if definition.chnn + WS34_MANNING_RELATION_TOLERANCE < definition.chnnbr {
            return Err(WatershedRuntimeInputError::ChannelSymbolOutOfDomain {
                symbol: format!("ws10_channel_{node_id}_chnn"),
                value: definition.chnn,
                rule: "chnn must be >= chnnbr",
            });
        }

        for (suffix, value, min, allow_zero) in [
            ("ishape", f64::from(definition.ishape), Some(0.0), false),
            ("ienslp", f64::from(definition.ienslp), Some(0.0), false),
            ("icntrl", f64::from(definition.icntrl), Some(0.0), true),
            ("flgout", f64::from(definition.flgout), Some(0.0), true),
            ("chnn", definition.chnn, Some(0.0), false),
            ("ctlslp", definition.ctlslp_effective, Some(0.0), true),
            ("chnk", definition.chnk, Some(0.0), true),
            ("chnz", definition.chnz, Some(0.0), false),
            ("chnnbr", definition.chnnbr, Some(0.0), false),
            ("chntcr", definition.chntcr, Some(0.0), true),
            ("chnedm", definition.chnedm, Some(0.0), true),
            ("chneds", definition.chneds, Some(0.0), true),
            ("ctlz", definition.ctlz_effective, Some(0.0), false),
            ("ctln", definition.ctln_effective, Some(0.0), false),
        ] {
            let symbol = format!("ws10_channel_{node_id}_{suffix}");
            validate_ws10_channel_value(symbol.as_str(), value, min, allow_zero)?;
            runtime_surface.state_surface.insert(
                BoundarySymbol::from(symbol.as_str()),
                BoundaryValue::scalar(value),
            );
        }

        let rating_curve_symbol = format!("ws10_channel_{node_id}_rating_curve");
        match (definition.icntrl == 4, definition.rating_curve.as_ref()) {
            (true, Some(rating_curve)) => {
                for (suffix, value, allow_zero) in [
                    ("rccoef", rating_curve.rccoef, false),
                    ("rcexp", rating_curve.rcexp, false),
                    ("rcoset", rating_curve.rcoset, true),
                ] {
                    let symbol = format!("ws10_channel_{node_id}_{suffix}");
                    validate_ws10_channel_value(symbol.as_str(), value, Some(0.0), allow_zero)?;
                    runtime_surface.state_surface.insert(
                        BoundarySymbol::from(symbol.as_str()),
                        BoundaryValue::scalar(value),
                    );
                }
            }
            (true, None) => {
                return Err(WatershedRuntimeInputError::ChannelSymbolOutOfDomain {
                    symbol: rating_curve_symbol,
                    value: f64::from(definition.icntrl),
                    rule: "icntrl==4 requires rating-curve payload (rccoef, rcexp, rcoset)",
                });
            }
            (false, Some(_)) => {
                return Err(WatershedRuntimeInputError::ChannelSymbolOutOfDomain {
                    symbol: rating_curve_symbol,
                    value: f64::from(definition.icntrl),
                    rule: "rating-curve payload is only valid when icntrl==4",
                });
            }
            (false, None) => {}
        }
    }

    Ok(())
}

/// Seed WS10 channel segment geometry/hydraulic scaffold symbols from parsed
/// slope profile input.
///
/// Inserts per-channel segment families:
/// - `ws10_channel_{id}_nslpts`
/// - `ws10_channel_{id}_x_{point:04}`
/// - `ws10_channel_{id}_slope_{point:04}`
/// - `ws10_channel_{id}_depa_{point:04}`
/// - `ws10_channel_{id}_depb_{point:04}`
/// - `ws10_channel_{id}_wida_{point:04}`
/// - `ws10_channel_{id}_widb_{point:04}`
///
/// # Errors
///
/// Returns `WatershedRuntimeInputError` when slope/channel cardinality mapping
/// is invalid or projected symbols are non-finite/out-of-domain.
#[allow(clippy::too_many_lines, clippy::similar_names)]
pub fn seed_watershed_runtime_surface_from_slope_channel_profile(
    runtime_surface: &mut WatershedWritebackSurface,
    channel: &WatershedChannelFile,
    slope: &SlopeProfile,
) -> Result<(), WatershedRuntimeInputError> {
    for definition in &channel.channels {
        let node_id = definition.channel_id;
        if node_id == 0 {
            return Err(WatershedRuntimeInputError::ChannelSymbolOutOfDomain {
                symbol: "ws10_channel_0_nslpts".to_owned(),
                value: 0.0,
                rule: "channel_id must be >= 1 for slope-profile mapping",
            });
        }

        let slope_index = node_id - 1;
        let Some(ofe) = slope.ofes.get(slope_index) else {
            let profile_count = u32::try_from(slope.ofes.len()).unwrap_or(u32::MAX);
            return Err(WatershedRuntimeInputError::ChannelSymbolOutOfDomain {
                symbol: format!("ws10_channel_{node_id}_nslpts"),
                value: f64::from(profile_count),
                rule: "slope profile count must cover every channel id (ordered by channel id)",
            });
        };

        if ofe.points.len() < 2 {
            let nslpts = u32::try_from(ofe.points.len()).unwrap_or(u32::MAX);
            return Err(WatershedRuntimeInputError::ChannelSymbolOutOfDomain {
                symbol: format!("ws10_channel_{node_id}_nslpts"),
                value: f64::from(nslpts),
                rule: "channel segment profile requires at least 2 slope points",
            });
        }

        let nslpts = u32::try_from(ofe.points.len()).map_err(|_| {
            WatershedRuntimeInputError::ChannelCountOutOfRange {
                value: ofe.points.len(),
            }
        })?;
        let nslpts_symbol = format!("ws10_channel_{node_id}_nslpts");
        validate_ws10_channel_value(nslpts_symbol.as_str(), f64::from(nslpts), Some(1.0), false)?;
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(nslpts_symbol),
            BoundaryValue::scalar(f64::from(nslpts)),
        );

        let width_ft = ofe.fwidth * WS17_METERS_TO_FEET;
        let depth_ft = definition.chnedm * WS17_METERS_TO_FEET;
        validate_ws10_channel_value(
            format!("ws10_channel_{node_id}_wida_0001").as_str(),
            width_ft,
            Some(0.0),
            false,
        )?;
        validate_ws10_channel_value(
            format!("ws10_channel_{node_id}_widb_0001").as_str(),
            width_ft,
            Some(0.0),
            false,
        )?;
        validate_ws10_channel_value(
            format!("ws10_channel_{node_id}_depa_0001").as_str(),
            depth_ft,
            Some(0.0),
            true,
        )?;
        validate_ws10_channel_value(
            format!("ws10_channel_{node_id}_depb_0001").as_str(),
            depth_ft,
            Some(0.0),
            true,
        )?;

        let mut previous_x = 0.0;
        for (point_index, point) in ofe.points.iter().enumerate() {
            let point_number = point_index + 1;
            let x_raw = match ofe.distance_mode {
                DistanceMode::Absolute => point.xinput,
                DistanceMode::Normalized => point.xinput * ofe.slplen,
            };
            let slope_value = point.slpinp;

            let x_symbol = format!("ws10_channel_{node_id}_x_{point_number:04}");
            let slope_symbol = format!("ws10_channel_{node_id}_slope_{point_number:04}");
            let depth_a_symbol = format!("ws10_channel_{node_id}_depa_{point_number:04}");
            let depth_b_symbol = format!("ws10_channel_{node_id}_depb_{point_number:04}");
            let width_a_symbol = format!("ws10_channel_{node_id}_wida_{point_number:04}");
            let width_b_symbol = format!("ws10_channel_{node_id}_widb_{point_number:04}");

            validate_ws10_channel_value(x_symbol.as_str(), x_raw, Some(0.0), true)?;
            if point_index > 0 && x_raw + 1.0e-12 < previous_x {
                return Err(WatershedRuntimeInputError::ChannelSymbolOutOfDomain {
                    symbol: x_symbol,
                    value: x_raw,
                    rule: "channel segment x positions must be monotonic non-decreasing",
                });
            }
            validate_ws10_channel_value(slope_symbol.as_str(), slope_value, Some(0.0), true)?;
            validate_ws10_channel_value(depth_a_symbol.as_str(), depth_ft, Some(0.0), true)?;
            validate_ws10_channel_value(depth_b_symbol.as_str(), depth_ft, Some(0.0), true)?;
            validate_ws10_channel_value(width_a_symbol.as_str(), width_ft, Some(0.0), false)?;
            validate_ws10_channel_value(width_b_symbol.as_str(), width_ft, Some(0.0), false)?;

            runtime_surface
                .state_surface
                .insert(BoundarySymbol::from(x_symbol), BoundaryValue::scalar(x_raw));
            runtime_surface.state_surface.insert(
                BoundarySymbol::from(slope_symbol),
                BoundaryValue::scalar(slope_value),
            );
            runtime_surface.state_surface.insert(
                BoundarySymbol::from(depth_a_symbol),
                BoundaryValue::scalar(depth_ft),
            );
            runtime_surface.state_surface.insert(
                BoundarySymbol::from(depth_b_symbol),
                BoundaryValue::scalar(depth_ft),
            );
            runtime_surface.state_surface.insert(
                BoundarySymbol::from(width_a_symbol),
                BoundaryValue::scalar(width_ft),
            );
            runtime_surface.state_surface.insert(
                BoundarySymbol::from(width_b_symbol),
                BoundaryValue::scalar(width_ft),
            );

            previous_x = x_raw;
        }
    }

    Ok(())
}

/// Seed WS10 impoundment runtime symbols from parsed watershed impoundment
/// input.
///
/// Inserts per-impoundment production-kernel controls:
/// - `ws10_impoundment_{id}_h`
/// - `ws10_impoundment_{id}_hfull`
/// - `ws10_impoundment_{id}_deltat`
/// - `ws10_impoundment_{id}_qinf`
/// - `ws10_impoundment_{id}_{a,b,c,d,e,ha,ht,hlm,a0,a1,a2,l0,l1,l2}`
///
/// # Errors
///
/// Returns `WatershedRuntimeInputError` when required symbols are non-finite or
/// violate declared domains.
pub fn seed_watershed_runtime_surface_from_watershed_impoundment(
    runtime_surface: &mut WatershedWritebackSurface,
    impoundment: &WatershedImpoundmentFile,
) -> Result<(), WatershedRuntimeInputError> {
    for (index, record) in impoundment.items.iter().enumerate() {
        let node_id = index + 1;
        let h_symbol = format!("ws10_impoundment_{node_id}_h");
        let hfull_symbol = format!("ws10_impoundment_{node_id}_hfull");
        let deltat_symbol = format!("ws10_impoundment_{node_id}_deltat");
        let qinf_symbol = format!("ws10_impoundment_{node_id}_qinf");

        validate_ws10_impoundment_value(h_symbol.as_str(), record.h, Some(0.0), true)?;
        validate_ws10_impoundment_value(hfull_symbol.as_str(), record.hfull, Some(0.0), false)?;
        validate_ws10_impoundment_value(deltat_symbol.as_str(), record.deltat, Some(0.0), false)?;
        validate_ws10_impoundment_value(qinf_symbol.as_str(), record.qinf, Some(0.0), true)?;
        if record.h > record.hfull {
            return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                symbol: h_symbol,
                value: record.h,
                rule: "<= ws10_impoundment_{id}_hfull",
            });
        }

        runtime_surface.state_surface.insert(
            BoundarySymbol::from(h_symbol.as_str()),
            BoundaryValue::scalar(record.h),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(hfull_symbol.as_str()),
            BoundaryValue::scalar(record.hfull),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(deltat_symbol.as_str()),
            BoundaryValue::scalar(record.deltat),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(qinf_symbol.as_str()),
            BoundaryValue::scalar(record.qinf),
        );

        let coefficients = derive_ws12_impoundment_coefficients(node_id, record)?;
        for (suffix, value, minimum, allow_equal_minimum) in coefficients {
            let symbol = format!("ws10_impoundment_{node_id}_{suffix}");
            validate_ws10_impoundment_value(symbol.as_str(), value, minimum, allow_equal_minimum)?;
            runtime_surface
                .state_surface
                .insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
        }

        let function_families = derive_ws12_outflow_function_families(node_id, record)?;
        for family_index in 1..=WS12_FUNCTION_COUNT {
            for coefficient_suffix in ["a", "b", "c", "d", "e", "ha"] {
                let symbol =
                    format!("ws10_impoundment_{node_id}_f{family_index:02}_{coefficient_suffix}");
                let coefficient =
                    function_families.coefficient_at(family_index, coefficient_suffix);
                validate_ws10_impoundment_value(symbol.as_str(), coefficient, None, true)?;
                runtime_surface.state_surface.insert(
                    BoundarySymbol::from(symbol),
                    BoundaryValue::scalar(coefficient),
                );
            }
        }
    }

    Ok(())
}

fn derive_ws12_impoundment_coefficients(
    node_id: usize,
    record: &ImpoundmentRecord,
) -> Result<[Ws12ImpoundmentProjectionTuple; 14], WatershedRuntimeInputError> {
    let has_active_structure = record.structure_flags.has_drop_spillway
        || record.structure_flags.has_culvert_1
        || record.structure_flags.has_culvert_2
        || record.structure_flags.has_rockfill
        || record.structure_flags.has_emergency_spillway
        || record.structure_flags.has_filter_barrier
        || record.structure_flags.has_perforated_riser;

    let (a1, a2) = derive_power_law_curve_coefficients(
        node_id,
        "area",
        &record.stage,
        &record.area,
        record.a0,
    )?;
    let (l1, l2) = derive_power_law_curve_coefficients(
        node_id,
        "length",
        &record.stage,
        &record.length,
        record.l0,
    )?;

    let area_denominator = record.a0 + a1 * record.h.powf(a2);
    if !area_denominator.is_finite() || area_denominator <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_a0"),
            value: area_denominator,
            rule: "derived stage-area denominator at current stage must be finite and > 0",
        });
    }

    let projection = if has_active_structure {
        derive_ws12_active_structure_projection(node_id, record)?
    } else {
        let threshold = record.hfull;
        Ws12ActiveProjection {
            drop_coefficient: 0.0,
            drop_exponent: 1.0,
            culvert_coefficient: 0.0,
            culvert_exponent: 1.0,
            riser_coefficient: 0.0,
            drop_threshold: threshold,
            culvert_threshold: threshold,
            riser_threshold: threshold,
        }
    };

    Ok([
        ("a", projection.drop_coefficient, Some(0.0), true),
        ("b", projection.drop_exponent, Some(0.0), false),
        ("c", projection.culvert_coefficient, Some(0.0), true),
        ("d", projection.culvert_exponent, Some(0.0), false),
        ("e", projection.riser_coefficient, Some(0.0), true),
        ("ha", projection.drop_threshold, Some(0.0), true),
        ("ht", projection.culvert_threshold, Some(0.0), true),
        ("hlm", projection.riser_threshold, Some(0.0), true),
        ("a0", record.a0, None, true),
        ("a1", a1, Some(0.0), false),
        ("a2", a2, Some(0.0), false),
        ("l0", record.l0, None, true),
        ("l1", l1, Some(0.0), false),
        ("l2", l2, Some(0.0), false),
    ])
}

#[allow(clippy::too_many_lines)]
fn derive_ws12_outflow_function_families(
    node_id: usize,
    record: &ImpoundmentRecord,
) -> Result<Ws12OutflowFunctionFamilies, WatershedRuntimeInputError> {
    let mut families = Ws12OutflowFunctionFamilies::inactive_default(record.hfull);

    project_drop_spillway_function_families(node_id, record, &mut families)?;
    project_culvert_function_families(node_id, &record.culverts[0], 4, &mut families)?;
    project_culvert_function_families(node_id, &record.culverts[1], 7, &mut families)?;
    project_rockfill_function(node_id, record, &mut families)?;
    project_emergency_function(node_id, record, &mut families)?;
    project_filter_function(node_id, record, &mut families)?;
    project_riser_functions(node_id, record, &mut families)?;

    Ok(families)
}

fn project_drop_spillway_function_families(
    node_id: usize,
    record: &ImpoundmentRecord,
    families: &mut Ws12OutflowFunctionFamilies,
) -> Result<(), WatershedRuntimeInputError> {
    use openwepp_input_contract::parsers::watershed_impoundment::DropSpillwayPayload;

    match &record.drop_spillway {
        DropSpillwayPayload::None => Ok(()),
        DropSpillwayPayload::Ids1 { payload, .. } => {
            let denominator =
                1.0 + payload.ke + payload.kb + payload.kc * (payload.lbl + payload.hrh);
            if !denominator.is_finite() || denominator <= 0.0 {
                return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                    symbol: format!("ws10_impoundment_{node_id}_f03_b"),
                    value: denominator,
                    rule: "drop-spillway loss denominator must be finite and > 0",
                });
            }

            families.a[0] = 1.0;
            families.b[0] = payload.coefw * std::f64::consts::PI * payload.diars;
            families.c[0] = 1.5;
            families.ha[0] = payload.hrs;

            families.a[1] = 1.0;
            families.b[1] = payload.coefo * std::f64::consts::PI * payload.diars.powi(2) / 4.0
                * (2.0 * STANDARD_GRAVITY_M_S2).sqrt();
            families.c[1] = 0.5;
            families.ha[1] = payload.hrs;

            families.a[2] = payload.hblot + 0.6 * payload.diabl;
            families.b[2] = std::f64::consts::PI * payload.diabl.powi(2) / 4.0
                * (2.0 * STANDARD_GRAVITY_M_S2).sqrt()
                / denominator.sqrt();
            families.c[2] = 0.5;
            families.ha[2] =
                payload.hrs - (payload.hrh + payload.sbl * payload.lbl - 0.6 * payload.diabl);

            Ok(())
        }
        DropSpillwayPayload::Ids2 { payload, .. } => {
            let denominator =
                1.0 + payload.ke + payload.kb + payload.kc * (payload.lbl + payload.hrh);
            if !denominator.is_finite() || denominator <= 0.0 {
                return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                    symbol: format!("ws10_impoundment_{node_id}_f03_b"),
                    value: denominator,
                    rule: "drop-spillway loss denominator must be finite and > 0",
                });
            }

            families.a[0] = 1.0;
            families.b[0] = payload.coefw * 2.0 * (payload.lenrs + payload.widrs);
            families.c[0] = 1.5;
            families.ha[0] = payload.hrs;

            families.a[1] = 1.0;
            families.b[1] = payload.coefo
                * payload.lenrs
                * payload.widrs
                * (2.0 * STANDARD_GRAVITY_M_S2).sqrt();
            families.c[1] = 0.5;
            families.ha[1] = payload.hrs;

            families.a[2] = payload.hblot + 0.6 * payload.diabl;
            families.b[2] = std::f64::consts::PI * payload.diabl.powi(2) / 4.0
                * (2.0 * STANDARD_GRAVITY_M_S2).sqrt()
                / denominator.sqrt();
            families.c[2] = 0.5;
            families.ha[2] =
                payload.hrs - (payload.hrh + payload.sbl * payload.lbl - 0.6 * payload.diabl);

            Ok(())
        }
        DropSpillwayPayload::Ids3 { payload, .. } => {
            let denominator =
                1.0 + payload.ke + payload.kb + payload.kc * (payload.lbl + payload.hrh);
            if !denominator.is_finite() || denominator <= 0.0 {
                return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                    symbol: format!("ws10_impoundment_{node_id}_f03_b"),
                    value: denominator,
                    rule: "drop-spillway loss denominator must be finite and > 0",
                });
            }

            families.a[0] = 1.0;
            families.b[0] = payload.coefw * 2.0 * (payload.lenrs + payload.widrs);
            families.c[0] = 1.5;
            families.ha[0] = payload.hrs;

            families.a[1] = 1.0;
            families.b[1] = payload.coefo
                * payload.lenrs
                * payload.widrs
                * (2.0 * STANDARD_GRAVITY_M_S2).sqrt();
            families.c[1] = 0.5;
            families.ha[1] = payload.hrs;

            families.a[2] = payload.hblot + 0.6 * payload.hitbl;
            families.b[2] = payload.hitbl * payload.wdbl * (2.0 * STANDARD_GRAVITY_M_S2).sqrt()
                / denominator.sqrt();
            families.c[2] = 0.5;
            families.ha[2] =
                payload.hrs - (payload.hrh + payload.sbl * payload.lbl - 0.6 * payload.hitbl);

            Ok(())
        }
    }
}

fn project_culvert_function_families(
    node_id: usize,
    culvert: &CulvertPayload,
    family_start: usize,
    families: &mut Ws12OutflowFunctionFamilies,
) -> Result<(), WatershedRuntimeInputError> {
    if culvert.icv < 1 {
        return Ok(());
    }

    let Some(parameters) = &culvert.parameters else {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_f{family_start:02}_a"),
            value: f64::from(culvert.icv),
            rule: "active culvert payload must include hydraulic parameters",
        });
    };

    let ncv = f64::from(culvert.ncv);
    let denominator = 1.0 + parameters.ke + parameters.kb + parameters.kc * parameters.lcv;
    if !denominator.is_finite() || denominator <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_f{:02}_b", family_start + 2),
            value: denominator,
            rule: "culvert loss denominator must be finite and > 0",
        });
    }
    if !ncv.is_finite() || ncv <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_f{family_start:02}_a"),
            value: ncv,
            rule: "culvert count must be finite and > 0",
        });
    }
    if !parameters.mus.is_finite() || parameters.mus.abs() <= 1.0e-12 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_f{family_start:02}_c"),
            value: parameters.mus,
            rule: "culvert mus must be finite and non-zero",
        });
    }
    if !parameters.cs.is_finite() || parameters.cs.abs() <= 1.0e-12 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_f{:02}_d", family_start + 1),
            value: parameters.cs,
            rule: "culvert cs must be finite and non-zero",
        });
    }

    let base = family_start - 1;
    families.a[base] = parameters.arcv * parameters.hitcv.sqrt() * ncv;
    families.b[base] = parameters.hitcv * parameters.kus;
    families.c[base] = 1.0 / parameters.mus;
    families.ha[base] = parameters.hcv;

    families.a[base + 1] = parameters.arcv * parameters.hitcv.sqrt() * ncv;
    families.b[base + 1] = parameters.hitcv;
    families.c[base + 1] = 0.5 * parameters.scv - parameters.ys;
    families.d[base + 1] = parameters.cs;
    families.ha[base + 1] = parameters.hcv;

    families.a[base + 2] = parameters.hcvot + 0.6 * parameters.hitcv;
    families.b[base + 2] =
        parameters.arcv * (2.0 * STANDARD_GRAVITY_M_S2).sqrt() * ncv / denominator.sqrt();
    families.c[base + 2] = 0.5;
    families.ha[base + 2] =
        parameters.hcv - parameters.scv * parameters.lcv + 0.6 * parameters.hitcv;

    Ok(())
}

fn project_rockfill_function(
    node_id: usize,
    record: &ImpoundmentRecord,
    families: &mut Ws12OutflowFunctionFamilies,
) -> Result<(), WatershedRuntimeInputError> {
    let Some(rockfill) = &record.rockfill else {
        return Ok(());
    };
    if !rockfill.diarf.is_finite() || rockfill.diarf <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_f10_b"),
            value: rockfill.diarf,
            rule: "rockfill diarf must be finite and > 0",
        });
    }
    if !rockfill.lnrf.is_finite() || rockfill.lnrf <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_f10_b"),
            value: rockfill.lnrf,
            rule: "rockfill lnrf must be finite and > 0",
        });
    }

    let arf = rockfill_arf(rockfill.lnrf, rockfill.diarf);
    let brf_denominator = 1.500_560_9 - 0.000_131_719_05 * rockfill.diarf.ln() / rockfill.diarf;
    if !brf_denominator.is_finite() || brf_denominator <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_f10_c"),
            value: brf_denominator,
            rule: "rockfill brf denominator must be finite and > 0",
        });
    }
    let brf = 1.0 / brf_denominator;

    let index = 9;
    families.a[index] = rockfill.wdrf;
    families.b[index] = rockfill.lnrf * arf;
    families.c[index] = 1.0 / brf;
    families.d[index] = EMERGENCY_OPEN_CHANNEL_WEIR_COEFFICIENT * rockfill.wdrf;
    families.e[index] = rockfill.hotrf;
    families.ha[index] = rockfill.hrf;
    Ok(())
}

fn project_emergency_function(
    node_id: usize,
    record: &ImpoundmentRecord,
    families: &mut Ws12OutflowFunctionFamilies,
) -> Result<(), WatershedRuntimeInputError> {
    let index = 10;
    match &record.emergency_spillway {
        EmergencySpillwayPayload::None => Ok(()),
        EmergencySpillwayPayload::OpenChannel { payload, .. } => {
            let span = (payload.hmxes - payload.hes).max(0.05);
            let mut points = Vec::with_capacity(16);
            points.push((0.0, 0.0));
            for sample_idx in 1..=15_u32 {
                let fraction = f64::from(sample_idx) / 15.0;
                let delta = span * fraction;
                let discharge =
                    EMERGENCY_OPEN_CHANNEL_WEIR_COEFFICIENT * payload.bwes * delta.powf(1.5);
                points.push((delta, discharge.max(0.0)));
            }
            let coefficients = fit_quartic_least_squares(node_id, &points, "f11")?;
            families.a[index] = coefficients[0];
            families.b[index] = coefficients[1];
            families.c[index] = coefficients[2];
            families.d[index] = coefficients[3];
            families.e[index] = coefficients[4];
            families.ha[index] = payload.hes;
            Ok(())
        }
        EmergencySpillwayPayload::RatingCurve { payload, .. } => {
            if payload.hest.len() != payload.qes.len() || payload.hest.is_empty() {
                return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                    symbol: format!("ws10_impoundment_{node_id}_f11_a"),
                    value: f64::from(u32::try_from(payload.hest.len()).unwrap_or(u32::MAX)),
                    rule: "emergency rating curve vectors must have equal non-zero length",
                });
            }
            let mut points = Vec::with_capacity(payload.hest.len() + 1);
            points.push((0.0, 0.0));
            for (&stage_value, &discharge_value) in payload.hest.iter().zip(payload.qes.iter()) {
                let x = (stage_value - payload.hes).max(0.0);
                points.push((x, discharge_value.max(0.0)));
            }
            let coefficients = fit_quartic_least_squares(node_id, &points, "f11")?;
            families.a[index] = coefficients[0];
            families.b[index] = coefficients[1];
            families.c[index] = coefficients[2];
            families.d[index] = coefficients[3];
            families.e[index] = coefficients[4];
            families.ha[index] = payload.hes;
            Ok(())
        }
    }
}

fn project_filter_function(
    node_id: usize,
    record: &ImpoundmentRecord,
    families: &mut Ws12OutflowFunctionFamilies,
) -> Result<(), WatershedRuntimeInputError> {
    let Some(filter) = &record.filter_barrier else {
        return Ok(());
    };
    let index = 11;
    families.a[index] = filter.wdff * filter.vsl;
    families.ha[index] = filter.hff;
    families.d[index] = filter.hotff;

    if filter.hotff <= filter.hff {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_f12_d"),
            value: filter.hotff - filter.hff,
            rule: "filter overtopping stage must be > base stage",
        });
    }

    if record.filter_code == 1 {
        families.b[index] = 3.27 * filter.wdff;
        families.c[index] = (0.4 / (filter.hotff - filter.hff)) * filter.wdff;
    } else {
        families.b[index] = EMERGENCY_OPEN_CHANNEL_WEIR_COEFFICIENT * filter.wdff;
        families.c[index] = 0.0;
    }
    Ok(())
}

fn project_riser_functions(
    node_id: usize,
    record: &ImpoundmentRecord,
    families: &mut Ws12OutflowFunctionFamilies,
) -> Result<(), WatershedRuntimeInputError> {
    let Some(riser) = &record.perforated_riser else {
        return Ok(());
    };

    if !riser.diar.is_finite() || riser.diar <= 0.0 || !riser.diab.is_finite() || riser.diab <= 0.0
    {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_f13_b"),
            value: if riser.diar <= 0.0 {
                riser.diar
            } else {
                riser.diab
            },
            rule: "riser diameters must be finite and > 0",
        });
    }

    let (apr1, apr2) = derive_riser_apr_coefficients(node_id, riser)?;
    let ko = (-0.60721 + 0.329_229 * (riser.diab / riser.diar)).exp();
    let denominator = 1.0 + riser.ke + riser.kb + riser.kc * riser.lbl + ko;
    if !denominator.is_finite() || denominator <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_f15_b"),
            value: denominator,
            rule: "riser loss denominator must be finite and > 0",
        });
    }

    let ab = std::f64::consts::PI * riser.diab.powi(2) / 4.0;
    let index_13 = 12;
    families.a[index_13] = 1.0;
    families.b[index_13] = apr1;
    families.c[index_13] = apr2;
    families.ha[index_13] = riser.hd;

    let index_14 = 13;
    families.a[index_14] = riser.cb * ab * (2.0 * STANDARD_GRAVITY_M_S2).sqrt();
    families.ha[index_14] = riser.hd - riser.hb;

    let index_15 = 14;
    families.b[index_15] = std::f64::consts::PI * riser.diabl.powi(2) / 4.0
        * (2.0 * STANDARD_GRAVITY_M_S2).sqrt()
        / denominator.sqrt();
    families.c[index_15] = 0.5;
    families.ha[index_15] = riser.hr - (riser.hrh + riser.sbl * riser.lbl - 0.6 * riser.diabl);

    Ok(())
}

fn derive_riser_apr_coefficients(
    node_id: usize,
    riser: &openwepp_input_contract::parsers::watershed_impoundment::PerforatedRiserPayload,
) -> Result<(f64, f64), WatershedRuntimeInputError> {
    let points = sample_riser_unsubmerged_curve(node_id, riser)?;
    if points.len() < 2 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_f13_b"),
            value: f64::from(u32::try_from(points.len()).unwrap_or(0)),
            rule: "riser unsubmerged curve sampling requires at least two points",
        });
    }

    let mut sum_inverse_head = 0.0;
    let mut sum_inverse_discharge = 0.0;
    let mut sum_inverse_head_squared = 0.0;
    let mut sum_cross_term = 0.0;
    for &(hp, q) in &points {
        if !hp.is_finite() || hp <= 0.0 || !q.is_finite() || q <= 0.0 {
            return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                symbol: format!("ws10_impoundment_{node_id}_f13_b"),
                value: if hp <= 0.0 { hp } else { q },
                rule: "riser regression points must be finite and > 0",
            });
        }
        let u = 1.0 / hp.powf(1.5);
        let z = 1.0 / q;
        sum_inverse_head += u;
        sum_inverse_discharge += z;
        sum_inverse_head_squared += u * u;
        sum_cross_term += u * z;
    }

    let n = f64::from(u32::try_from(points.len()).unwrap_or(u32::MAX));
    let denominator = (n * sum_inverse_head_squared) - (sum_inverse_head * sum_inverse_head);
    if !denominator.is_finite() || denominator.abs() <= 1.0e-12 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_f13_b"),
            value: denominator,
            rule: "riser regression denominator must be finite and non-zero",
        });
    }

    let apr1 = ((sum_inverse_discharge * sum_inverse_head_squared)
        - (sum_inverse_head * sum_cross_term))
        / denominator;
    let apr2 = ((n * sum_cross_term) - (sum_inverse_head * sum_inverse_discharge)) / denominator;
    if !apr1.is_finite() || !apr2.is_finite() {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_f13_b"),
            value: if apr1.is_finite() { apr2 } else { apr1 },
            rule: "riser regression coefficients must be finite",
        });
    }

    Ok((apr1, apr2))
}

#[allow(clippy::too_many_lines)]
fn sample_riser_unsubmerged_curve(
    node_id: usize,
    riser: &openwepp_input_contract::parsers::watershed_impoundment::PerforatedRiserPayload,
) -> Result<Vec<(f64, f64)>, WatershedRuntimeInputError> {
    let mut points = Vec::new();
    let mut hp_delta = 0.05;
    let mut hp = hp_delta;
    let mut y = -riser.hb;
    let mut iterations = 0_usize;
    let maximum_iterations = 20_000_usize;
    let q_tolerance = 1.0e-12;
    let y_delta = 1.0e-4;

    let ko = (-0.60721 + 0.329_229 * (riser.diab / riser.diar)).exp();
    let ab = std::f64::consts::PI * riser.diab.powi(2) / 4.0;

    while iterations < maximum_iterations && points.len() < 99 {
        iterations += 1;
        let qb_head = riser.hb + y;
        if qb_head <= 0.0 || !qb_head.is_finite() {
            y += y_delta;
            continue;
        }

        let qb = riser.cb * ab * (2.0 * STANDARD_GRAVITY_M_S2 * qb_head).sqrt();
        let qs = compute_riser_qs(hp, y, ko, riser)?;
        if !qb.is_finite() || !qs.is_finite() {
            return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                symbol: format!("ws10_impoundment_{node_id}_f13_b"),
                value: if qb.is_finite() { qs } else { qb },
                rule: "riser sampled discharges must be finite",
            });
        }

        if qb < qs {
            y += y_delta;
            if y >= hp {
                points.push((hp.max(1.0e-6), qb.max(q_tolerance)));
                hp += hp_delta;
                if hp_delta <= 0.0 || !hp_delta.is_finite() {
                    break;
                }
                continue;
            }
            if y > (riser.hr - riser.hd) {
                break;
            }
            continue;
        }

        points.push((hp.max(1.0e-6), qs.max(q_tolerance)));
        hp += hp_delta;
        if points.len() >= 99 {
            break;
        }
        if hp > 5.0 * (riser.hr + riser.hs + riser.hd + 1.0) {
            hp_delta *= 2.0;
            if hp_delta > 10.0 {
                break;
            }
        }
    }

    Ok(points)
}

fn compute_riser_qs(
    hp: f64,
    y: f64,
    ko: f64,
    riser: &openwepp_input_contract::parsers::watershed_impoundment::PerforatedRiserPayload,
) -> Result<f64, WatershedRuntimeInputError> {
    let slot_factor = (riser.cs * riser.as_slot / riser.hs) * (2.0 * STANDARD_GRAVITY_M_S2).sqrt();
    let qs = if hp < riser.hs {
        if y <= 0.0 {
            (2.0 / 3.0) * slot_factor * hp.powf(1.5)
        } else {
            slot_factor * (y * (hp - y).sqrt() + (2.0 / 3.0) * (hp - y).powf(1.5))
        }
    } else if hp <= (riser.hr - riser.hd) {
        if y <= 0.0 {
            (2.0 / 3.0) * slot_factor * (hp.powf(1.5) - (hp - riser.hs).powf(1.5))
        } else if y <= riser.hs {
            slot_factor
                * (y * (hp - y).sqrt()
                    + (2.0 / 3.0) * ((hp - y).powf(1.5) - (hp - riser.hs).powf(1.5)))
        } else {
            (riser.cs * riser.as_slot) * (2.0 * STANDARD_GRAVITY_M_S2 * (hp - y)).sqrt()
        }
    } else {
        let qw = riser.coefw
            * std::f64::consts::PI
            * riser.diar
            * (hp - (riser.hr - riser.hd)).powf(1.5);
        let qo = riser.coefo * std::f64::consts::PI * riser.diar.powi(2) / 4.0
            * (hp - (riser.hr - riser.hd)).sqrt();
        let q_control = qw.min(qo);
        if y <= 0.0 {
            (2.0 / 3.0) * slot_factor * (hp.powf(1.5) - (hp - riser.hs).powf(1.5)) + q_control
        } else if y <= riser.hs {
            slot_factor
                * (y * (hp - y).sqrt()
                    + (2.0 / 3.0) * ((hp - y).powf(1.5) - (hp - riser.hs).powf(1.5)))
                + q_control
        } else {
            (riser.cs * riser.as_slot) * (2.0 * STANDARD_GRAVITY_M_S2 * (hp - y)).sqrt() + q_control
        }
    };

    let _ = ko;
    if !qs.is_finite() {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: "ws10_impoundment_riser_qs".to_owned(),
            value: qs,
            rule: "riser sampled discharge must be finite",
        });
    }
    Ok(qs.max(0.0))
}

fn fit_quartic_least_squares(
    node_id: usize,
    points: &[(f64, f64)],
    family_label: &'static str,
) -> Result<[f64; 5], WatershedRuntimeInputError> {
    if points.is_empty() {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_{family_label}_a"),
            value: 0.0,
            rule: "quartic fit requires at least one point",
        });
    }

    let mut fit_points = points.to_vec();
    while fit_points.len() < 5 {
        let next = if fit_points.len() == 1 {
            let (x, y) = fit_points[0];
            (x + 0.05, y)
        } else {
            let (x_last, y_last) = fit_points[fit_points.len() - 1];
            let (x_prev, y_prev) = fit_points[fit_points.len() - 2];
            let dx = (x_last - x_prev).abs().max(0.05);
            let slope = if dx > 0.0 {
                (y_last - y_prev) / dx
            } else {
                0.0
            };
            (x_last + dx, (y_last + slope * dx).max(0.0))
        };
        fit_points.push(next);
    }

    let mut normal = [[0.0_f64; 5]; 5];
    let mut rhs = [0.0_f64; 5];
    for &(x, y) in &fit_points {
        if !x.is_finite() || !y.is_finite() {
            return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                symbol: format!("ws10_impoundment_{node_id}_{family_label}_a"),
                value: if x.is_finite() { y } else { x },
                rule: "quartic fit points must be finite",
            });
        }
        let powers = [1.0, x, x * x, x * x * x, x * x * x * x];
        for row in 0..5 {
            rhs[row] += y * powers[row];
            for column in 0..5 {
                normal[row][column] += powers[row] * powers[column];
            }
        }
    }

    solve_linear_system_5x5(normal, rhs).ok_or_else(|| {
        WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_{family_label}_a"),
            value: f64::NAN,
            rule: "quartic fit normal system must be solvable",
        }
    })
}

#[allow(clippy::needless_range_loop)]
fn solve_linear_system_5x5(mut matrix: [[f64; 5]; 5], mut rhs: [f64; 5]) -> Option<[f64; 5]> {
    for pivot in 0..5 {
        let mut max_row = pivot;
        let mut max_value = matrix[pivot][pivot].abs();
        for row in (pivot + 1)..5 {
            let candidate = matrix[row][pivot].abs();
            if candidate > max_value {
                max_value = candidate;
                max_row = row;
            }
        }
        if max_value <= 1.0e-12 {
            return None;
        }

        if max_row != pivot {
            matrix.swap(pivot, max_row);
            rhs.swap(pivot, max_row);
        }

        let pivot_value = matrix[pivot][pivot];
        for column in pivot..5 {
            matrix[pivot][column] /= pivot_value;
        }
        rhs[pivot] /= pivot_value;

        for row in 0..5 {
            if row == pivot {
                continue;
            }
            let factor = matrix[row][pivot];
            if factor.abs() <= 1.0e-20 {
                continue;
            }
            for column in pivot..5 {
                matrix[row][column] -= factor * matrix[pivot][column];
            }
            rhs[row] -= factor * rhs[pivot];
        }
    }

    Some(rhs)
}

#[allow(clippy::too_many_lines)]
fn derive_ws12_active_structure_projection(
    node_id: usize,
    record: &ImpoundmentRecord,
) -> Result<Ws12ActiveProjection, WatershedRuntimeInputError> {
    let reference_stage = derive_active_projection_reference_stage(node_id, record)?;
    let mut active_projection_used = false;

    let (drop_coefficient, drop_exponent, drop_threshold) = if let Some((
        projected_drop_coefficient,
        projected_drop_exponent,
        projected_drop_threshold,
    )) =
        derive_drop_spillway_projection(node_id, record)?
    {
        active_projection_used = true;
        (
            projected_drop_coefficient,
            projected_drop_exponent,
            projected_drop_threshold,
        )
    } else {
        (0.0, 1.0, record.hfull)
    };

    let mut c_stage_thresholds = Vec::new();
    if let Some(threshold) = culvert_stage_threshold(&record.culverts[0])? {
        c_stage_thresholds.push(threshold);
    }
    if let Some(threshold) = culvert_stage_threshold(&record.culverts[1])? {
        c_stage_thresholds.push(threshold);
    }
    if let Some(rockfill) = &record.rockfill {
        c_stage_thresholds.push(rockfill.hrf);
    }
    match &record.emergency_spillway {
        EmergencySpillwayPayload::None => {}
        EmergencySpillwayPayload::OpenChannel { payload, .. } => {
            c_stage_thresholds.push(payload.hes);
        }
        EmergencySpillwayPayload::RatingCurve { payload, .. } => {
            c_stage_thresholds.push(payload.hes);
        }
    }
    if let Some(filter) = &record.filter_barrier {
        c_stage_thresholds.push(filter.hff);
    }

    let (culvert_coefficient, culvert_exponent, culvert_threshold) =
        if c_stage_thresholds.is_empty() {
            (0.0, 1.0, record.hfull)
        } else {
            active_projection_used = true;
            let culvert_threshold = c_stage_thresholds
                .iter()
                .copied()
                .fold(f64::INFINITY, f64::min);
            if !culvert_threshold.is_finite() {
                return Err(WatershedRuntimeInputError::ImpoundmentSymbolNonFinite {
                    symbol: format!("ws10_impoundment_{node_id}_ht"),
                    value: culvert_threshold,
                });
            }
            let stage = reference_stage.max(culvert_threshold + ACTIVE_PROJECTION_STAGE_DELTA_M);
            let mut projected_discharge = 0.0;
            projected_discharge +=
                culvert_pipe_discharge_at_stage(node_id, &record.culverts[0], stage)?;
            projected_discharge +=
                culvert_pipe_discharge_at_stage(node_id, &record.culverts[1], stage)?;
            projected_discharge += rockfill_discharge_at_stage(node_id, record, stage)?;
            projected_discharge += emergency_discharge_at_stage(node_id, record, stage)?;
            projected_discharge += filter_barrier_discharge_at_stage(node_id, record, stage)?;

            if !projected_discharge.is_finite() || projected_discharge < 0.0 {
                return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                    symbol: format!("ws10_impoundment_{node_id}_c"),
                    value: projected_discharge,
                    rule: "projected active-structure discharge must be finite and >= 0",
                });
            }

            let span = stage - culvert_threshold;
            if !span.is_finite() || span <= 0.0 {
                return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                    symbol: format!("ws10_impoundment_{node_id}_ht"),
                    value: span,
                    rule: "reference-stage span above ht must be finite and > 0",
                });
            }

            let culvert_exponent = 0.5;
            let culvert_coefficient = if projected_discharge > 0.0 {
                projected_discharge / span.powf(culvert_exponent)
            } else {
                0.0
            };
            (culvert_coefficient, culvert_exponent, culvert_threshold)
        };

    let (riser_coefficient, riser_threshold) =
        if let Some((riser_reference_discharge, riser_threshold)) =
            perforated_riser_reference_discharge(node_id, record, reference_stage)?
        {
            active_projection_used = true;
            let stage = reference_stage.max(riser_threshold + ACTIVE_PROJECTION_STAGE_DELTA_M);
            let span = stage - riser_threshold;
            if !span.is_finite() || span <= 0.0 {
                return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                    symbol: format!("ws10_impoundment_{node_id}_hlm"),
                    value: span,
                    rule: "reference-stage span above hlm must be finite and > 0",
                });
            }
            (riser_reference_discharge / span, riser_threshold)
        } else {
            (0.0, record.hfull)
        };

    if !active_projection_used {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_a"),
            value: 0.0,
            rule: "active outlet-structure flags require at least one projectable payload branch",
        });
    }

    Ok(Ws12ActiveProjection {
        drop_coefficient,
        drop_exponent,
        culvert_coefficient,
        culvert_exponent,
        riser_coefficient,
        drop_threshold,
        culvert_threshold,
        riser_threshold,
    })
}

fn derive_active_projection_reference_stage(
    node_id: usize,
    record: &ImpoundmentRecord,
) -> Result<f64, WatershedRuntimeInputError> {
    let reference_stage = record.h.max(record.hfull);
    if !reference_stage.is_finite() {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolNonFinite {
            symbol: format!("ws10_impoundment_{node_id}_h"),
            value: reference_stage,
        });
    }
    Ok(reference_stage.max(ACTIVE_PROJECTION_STAGE_DELTA_M))
}

fn derive_drop_spillway_projection(
    node_id: usize,
    record: &ImpoundmentRecord,
) -> Result<Option<(f64, f64, f64)>, WatershedRuntimeInputError> {
    match &record.drop_spillway {
        openwepp_input_contract::parsers::watershed_impoundment::DropSpillwayPayload::None => {
            Ok(None)
        }
        openwepp_input_contract::parsers::watershed_impoundment::DropSpillwayPayload::Ids1 {
            payload,
            ..
        } => {
            let coefficient = payload.coefw * std::f64::consts::PI * payload.diars;
            validate_active_projected_positive(
                node_id,
                "a",
                coefficient,
                "drop-spillway weir coefficient must be finite and > 0",
            )?;
            Ok(Some((coefficient, 1.5, payload.hrs)))
        }
        openwepp_input_contract::parsers::watershed_impoundment::DropSpillwayPayload::Ids2 {
            payload,
            ..
        } => {
            let perimeter = 2.0 * (payload.lenrs + payload.widrs);
            let coefficient = payload.coefw * perimeter;
            validate_active_projected_positive(
                node_id,
                "a",
                coefficient,
                "drop-spillway weir coefficient must be finite and > 0",
            )?;
            Ok(Some((coefficient, 1.5, payload.hrs)))
        }
        openwepp_input_contract::parsers::watershed_impoundment::DropSpillwayPayload::Ids3 {
            payload,
            ..
        } => {
            let perimeter = 2.0 * (payload.lenrs + payload.widrs);
            let coefficient = payload.coefw * perimeter;
            validate_active_projected_positive(
                node_id,
                "a",
                coefficient,
                "drop-spillway weir coefficient must be finite and > 0",
            )?;
            Ok(Some((coefficient, 1.5, payload.hrs)))
        }
    }
}

fn culvert_stage_threshold(
    culvert: &CulvertPayload,
) -> Result<Option<f64>, WatershedRuntimeInputError> {
    if culvert.icv < 1 {
        return Ok(None);
    }
    let Some(parameters) = &culvert.parameters else {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: "ws10_impoundment_active_culvert".to_owned(),
            value: f64::from(culvert.icv),
            rule: "active culvert payload must include hydraulic parameters",
        });
    };
    Ok(Some(
        parameters.hcv - parameters.scv * parameters.lcv + 0.6 * parameters.hitcv,
    ))
}

fn culvert_pipe_discharge_at_stage(
    node_id: usize,
    culvert: &CulvertPayload,
    stage: f64,
) -> Result<f64, WatershedRuntimeInputError> {
    if culvert.icv < 1 {
        return Ok(0.0);
    }
    let Some(parameters) = &culvert.parameters else {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_c"),
            value: f64::from(culvert.icv),
            rule: "active culvert payload must include hydraulic parameters",
        });
    };

    let threshold = parameters.hcv - parameters.scv * parameters.lcv + 0.6 * parameters.hitcv;
    if stage <= threshold {
        return Ok(0.0);
    }

    let denominator = 1.0 + parameters.ke + parameters.kb + parameters.kc * parameters.lcv;
    if !denominator.is_finite() || denominator <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_c"),
            value: denominator,
            rule: "active culvert loss denominator must be finite and > 0",
        });
    }

    let count = f64::from(culvert.ncv);
    if !count.is_finite() || count <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_c"),
            value: count,
            rule: "active culvert count must be finite and > 0",
        });
    }

    let coefficient =
        parameters.arcv * (2.0 * STANDARD_GRAVITY_M_S2).sqrt() * count / denominator.sqrt();
    if !coefficient.is_finite() || coefficient <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_c"),
            value: coefficient,
            rule: "active culvert projected coefficient must be finite and > 0",
        });
    }

    Ok(coefficient * (stage - threshold).sqrt())
}

fn rockfill_discharge_at_stage(
    node_id: usize,
    record: &ImpoundmentRecord,
    stage: f64,
) -> Result<f64, WatershedRuntimeInputError> {
    let Some(rockfill) = &record.rockfill else {
        return Ok(0.0);
    };
    if stage <= rockfill.hrf {
        return Ok(0.0);
    }
    if !rockfill.diarf.is_finite() || rockfill.diarf <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_c"),
            value: rockfill.diarf,
            rule: "rockfill diarf must be finite and > 0",
        });
    }
    if !rockfill.lnrf.is_finite() || rockfill.lnrf <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_c"),
            value: rockfill.lnrf,
            rule: "rockfill lnrf must be finite and > 0",
        });
    }

    let arf = rockfill_arf(rockfill.lnrf, rockfill.diarf);
    let brf_denominator = 1.500_560_9 - 0.000_131_719_05 * rockfill.diarf.ln() / rockfill.diarf;
    if !brf_denominator.is_finite() || brf_denominator <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_c"),
            value: brf_denominator,
            rule: "rockfill brf denominator must be finite and > 0",
        });
    }

    let brf = 1.0 / brf_denominator;
    let b10 = rockfill.lnrf * arf;
    if !b10.is_finite() || b10 <= 0.0 || !brf.is_finite() || brf <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_c"),
            value: if b10 <= 0.0 { b10 } else { brf },
            rule: "rockfill projected coefficients must be finite and > 0",
        });
    }

    let mut discharge = 0.0;
    let stage_delta = stage - rockfill.hrf;
    if stage_delta > 0.0 {
        discharge += rockfill.wdrf * (stage_delta / b10).powf(1.0 / brf);
    }
    let overtopping_delta = stage - rockfill.hotrf;
    if overtopping_delta > 0.0 {
        discharge += 3.087 * rockfill.wdrf * overtopping_delta.powf(1.5);
    }
    if !discharge.is_finite() || discharge < 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_c"),
            value: discharge,
            rule: "rockfill projected discharge must be finite and >= 0",
        });
    }
    Ok(discharge)
}

fn rockfill_arf(length_m: f64, diarf_m: f64) -> f64 {
    if length_m < 0.5 {
        let arf1 = 3.041_846 * diarf_m.powf(-0.346_77);
        let arf2 = 1.910_413 * diarf_m.powf(-0.349_35);
        arf1 - ((arf2 - arf1) / 0.5) * (0.5 - length_m)
    } else if length_m < 1.0 {
        let arf1 = 3.041_846 * diarf_m.powf(-0.346_77);
        let arf2 = 1.910_413 * diarf_m.powf(-0.349_35);
        arf1 + ((arf2 - arf1) / 0.5) * (length_m - 0.5)
    } else if length_m < 2.0 {
        let arf1 = 1.910_413 * diarf_m.powf(-0.349_35);
        let arf2 = 1.196_37 * diarf_m.powf(-0.354_22);
        arf1 + (arf2 - arf1) * (length_m - 1.0)
    } else if length_m < 3.0 {
        let arf1 = 1.196_37 * diarf_m.powf(-0.354_22);
        let arf2 = 0.909_902 * diarf_m.powf(-0.357_05);
        arf1 + (arf2 - arf1) * (length_m - 2.0)
    } else {
        let arf1 = 1.196_37 * diarf_m.powf(-0.354_22);
        let arf2 = 0.909_902 * diarf_m.powf(-0.357_05);
        arf2 + (arf2 - arf1) * (length_m - 3.0)
    }
}

fn emergency_discharge_at_stage(
    node_id: usize,
    record: &ImpoundmentRecord,
    stage: f64,
) -> Result<f64, WatershedRuntimeInputError> {
    match &record.emergency_spillway {
        EmergencySpillwayPayload::None => Ok(0.0),
        EmergencySpillwayPayload::OpenChannel { payload, .. } => {
            if stage <= payload.hes {
                return Ok(0.0);
            }
            let delta = stage - payload.hes;
            let discharge =
                EMERGENCY_OPEN_CHANNEL_WEIR_COEFFICIENT * payload.bwes * delta.powf(1.5);
            if !discharge.is_finite() || discharge < 0.0 {
                return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                    symbol: format!("ws10_impoundment_{node_id}_c"),
                    value: discharge,
                    rule: "emergency open-channel projected discharge must be finite and >= 0",
                });
            }
            Ok(discharge)
        }
        EmergencySpillwayPayload::RatingCurve { payload, .. } => {
            if stage <= payload.hes {
                return Ok(0.0);
            }
            if payload.hest.len() != payload.qes.len() || payload.hest.is_empty() {
                let hest_len = u32::try_from(payload.hest.len()).map_err(|_| {
                    WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                        symbol: format!("ws10_impoundment_{node_id}_c"),
                        value: f64::INFINITY,
                        rule: "emergency rating curve vectors must have equal non-zero length",
                    }
                })?;
                return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                    symbol: format!("ws10_impoundment_{node_id}_c"),
                    value: f64::from(hest_len),
                    rule: "emergency rating curve vectors must have equal non-zero length",
                });
            }
            interpolate_rating_curve_discharge(
                node_id,
                payload.hes,
                &payload.hest,
                &payload.qes,
                stage,
            )
        }
    }
}

fn interpolate_rating_curve_discharge(
    node_id: usize,
    hes: f64,
    stage_values: &[f64],
    discharge_values: &[f64],
    stage: f64,
) -> Result<f64, WatershedRuntimeInputError> {
    let mut previous_stage = hes;
    let mut previous_discharge = 0.0;

    for (&curve_stage, &curve_discharge) in stage_values.iter().zip(discharge_values.iter()) {
        if !curve_stage.is_finite() || !curve_discharge.is_finite() {
            return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                symbol: format!("ws10_impoundment_{node_id}_c"),
                value: if curve_stage.is_finite() {
                    curve_discharge
                } else {
                    curve_stage
                },
                rule: "emergency rating-curve points must be finite",
            });
        }
        if curve_stage <= previous_stage {
            return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                symbol: format!("ws10_impoundment_{node_id}_c"),
                value: curve_stage,
                rule: "emergency rating-curve stage points must be strictly increasing",
            });
        }
        if stage <= curve_stage {
            let fraction = (stage - previous_stage) / (curve_stage - previous_stage);
            let projected = previous_discharge + fraction * (curve_discharge - previous_discharge);
            return Ok(projected.max(0.0));
        }
        previous_stage = curve_stage;
        previous_discharge = curve_discharge;
    }

    if stage_values.len() == 1 {
        return Ok(previous_discharge.max(0.0));
    }

    let last_index = stage_values.len() - 1;
    let stage_left = stage_values[last_index - 1];
    let stage_right = stage_values[last_index];
    let discharge_left = discharge_values[last_index - 1];
    let discharge_right = discharge_values[last_index];
    let slope = (discharge_right - discharge_left) / (stage_right - stage_left);
    let extrapolated = discharge_right + slope * (stage - stage_right);
    Ok(extrapolated.max(0.0))
}

fn filter_barrier_discharge_at_stage(
    node_id: usize,
    record: &ImpoundmentRecord,
    stage: f64,
) -> Result<f64, WatershedRuntimeInputError> {
    let Some(filter) = &record.filter_barrier else {
        return Ok(0.0);
    };

    if stage <= filter.hff {
        return Ok(0.0);
    }
    let through = filter.wdff * filter.vsl * (stage - filter.hff);
    let overtopping = if stage > filter.hotff {
        let delta = stage - filter.hotff;
        if record.filter_code == 1 {
            if filter.hotff <= filter.hff {
                return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                    symbol: format!("ws10_impoundment_{node_id}_c"),
                    value: filter.hotff - filter.hff,
                    rule: "filter overtopping stage must be > base stage",
                });
            }
            let b = 3.27 * filter.wdff;
            let c = (0.4 / (filter.hotff - filter.hff)) * filter.wdff;
            (b + c * delta) * delta.powf(1.5)
        } else {
            3.087 * filter.wdff * delta.powf(1.5)
        }
    } else {
        0.0
    };
    let discharge = through + overtopping;
    if !discharge.is_finite() || discharge < 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_c"),
            value: discharge,
            rule: "filter-barrier projected discharge must be finite and >= 0",
        });
    }
    Ok(discharge)
}

fn perforated_riser_reference_discharge(
    node_id: usize,
    record: &ImpoundmentRecord,
    reference_stage: f64,
) -> Result<Option<(f64, f64)>, WatershedRuntimeInputError> {
    let Some(riser) = &record.perforated_riser else {
        return Ok(None);
    };
    if !riser.diar.is_finite() || riser.diar <= 0.0 || !riser.diab.is_finite() || riser.diab <= 0.0
    {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_e"),
            value: if riser.diar <= 0.0 {
                riser.diar
            } else {
                riser.diab
            },
            rule: "riser diameters must be finite and > 0",
        });
    }
    let ko = (-0.60721 + 0.329_229 * (riser.diab / riser.diar)).exp();
    let denominator = 1.0 + riser.ke + riser.kb + riser.kc * riser.lbl + ko;
    if !denominator.is_finite() || denominator <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_e"),
            value: denominator,
            rule: "riser loss denominator must be finite and > 0",
        });
    }

    let ha = riser.hr - (riser.hrh + riser.sbl * riser.lbl - 0.6 * riser.diabl);
    let coefficient = std::f64::consts::PI * riser.diabl.powi(2) / 4.0
        * (2.0 * STANDARD_GRAVITY_M_S2).sqrt()
        / denominator.sqrt();
    if !coefficient.is_finite() || coefficient <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_e"),
            value: coefficient,
            rule: "riser projected coefficient must be finite and > 0",
        });
    }

    let stage = reference_stage.max(ha + ACTIVE_PROJECTION_STAGE_DELTA_M);
    let discharge = coefficient * (stage - ha).sqrt();
    if !discharge.is_finite() || discharge < 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_e"),
            value: discharge,
            rule: "riser projected discharge must be finite and >= 0",
        });
    }
    Ok(Some((discharge, ha)))
}

fn validate_active_projected_positive(
    node_id: usize,
    suffix: &str,
    value: f64,
    rule: &'static str,
) -> Result<(), WatershedRuntimeInputError> {
    if !value.is_finite() || value <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_{suffix}"),
            value,
            rule,
        });
    }
    Ok(())
}

fn derive_power_law_curve_coefficients(
    node_id: usize,
    curve_family: &'static str,
    stage: &[f64],
    response: &[f64],
    baseline: f64,
) -> Result<(f64, f64), WatershedRuntimeInputError> {
    if stage.is_empty() || stage.len() != response.len() {
        let stage_len = u32::try_from(stage.len()).map_err(|_| {
            WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                symbol: format!("ws10_impoundment_{node_id}_{curve_family}"),
                value: f64::INFINITY,
                rule: "stage/response vectors must have equal non-zero length",
            }
        })?;
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_{curve_family}"),
            value: f64::from(stage_len),
            rule: "stage/response vectors must have equal non-zero length",
        });
    }

    let mut log_stage = Vec::with_capacity(stage.len());
    let mut log_adjusted = Vec::with_capacity(stage.len());
    for (&stage_value, &response_value) in stage.iter().zip(response.iter()) {
        if !stage_value.is_finite() {
            return Err(WatershedRuntimeInputError::ImpoundmentSymbolNonFinite {
                symbol: format!("ws10_impoundment_{node_id}_{curve_family}_stage"),
                value: stage_value,
            });
        }
        if !response_value.is_finite() {
            return Err(WatershedRuntimeInputError::ImpoundmentSymbolNonFinite {
                symbol: format!("ws10_impoundment_{node_id}_{curve_family}_response"),
                value: response_value,
            });
        }
        if stage_value <= 0.0 {
            return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                symbol: format!("ws10_impoundment_{node_id}_{curve_family}"),
                value: stage_value,
                rule: "stage values must be > 0 for coefficient projection",
            });
        }
        let adjusted = response_value - baseline;
        if adjusted <= 0.0 {
            return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                symbol: format!("ws10_impoundment_{node_id}_{curve_family}"),
                value: adjusted,
                rule: "response-baseline values must be > 0 for coefficient projection",
            });
        }

        log_stage.push(stage_value.ln());
        log_adjusted.push(adjusted.ln());
    }

    let log_len_u32 = u32::try_from(log_stage.len()).map_err(|_| {
        WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_{curve_family}"),
            value: f64::INFINITY,
            rule: "stage/response vectors must have equal non-zero length",
        }
    })?;
    let log_len = f64::from(log_len_u32);
    let mean_x = log_stage.iter().sum::<f64>() / log_len;
    let mean_y = log_adjusted.iter().sum::<f64>() / log_len;

    let mut numerator = 0.0;
    let mut denominator = 0.0;
    for (&x, &y) in log_stage.iter().zip(log_adjusted.iter()) {
        let dx = x - mean_x;
        numerator += dx * (y - mean_y);
        denominator += dx * dx;
    }
    if denominator <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_{curve_family}"),
            value: denominator,
            rule: "stage values must span a non-degenerate range for coefficient projection",
        });
    }

    let exponent = numerator / denominator;
    let intercept = mean_y - exponent * mean_x;
    let slope = intercept.exp();
    if !slope.is_finite() || !exponent.is_finite() {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolNonFinite {
            symbol: format!("ws10_impoundment_{node_id}_{curve_family}"),
            value: if slope.is_finite() { exponent } else { slope },
        });
    }
    if slope <= 0.0 || exponent <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_{curve_family}"),
            value: if slope <= 0.0 { slope } else { exponent },
            rule: "derived slope and exponent must be > 0",
        });
    }

    Ok((slope, exponent))
}

fn validate_ws10_channel_value(
    symbol: &str,
    value: f64,
    minimum: Option<f64>,
    allow_equal_minimum: bool,
) -> Result<(), WatershedRuntimeInputError> {
    if !value.is_finite() {
        return Err(WatershedRuntimeInputError::ChannelSymbolNonFinite {
            symbol: symbol.to_owned(),
            value,
        });
    }
    if let Some(minimum_value) = minimum {
        let violated = if allow_equal_minimum {
            value < minimum_value
        } else {
            value <= minimum_value
        };
        if violated {
            let rule = if allow_equal_minimum {
                ">= minimum"
            } else {
                "> minimum"
            };
            return Err(WatershedRuntimeInputError::ChannelSymbolOutOfDomain {
                symbol: symbol.to_owned(),
                value,
                rule,
            });
        }
    }
    Ok(())
}

fn validate_ws10_impoundment_value(
    symbol: &str,
    value: f64,
    minimum: Option<f64>,
    allow_equal_minimum: bool,
) -> Result<(), WatershedRuntimeInputError> {
    if !value.is_finite() {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolNonFinite {
            symbol: symbol.to_owned(),
            value,
        });
    }
    if let Some(minimum_value) = minimum {
        let violated = if allow_equal_minimum {
            value < minimum_value
        } else {
            value <= minimum_value
        };
        if violated {
            let rule = if allow_equal_minimum {
                ">= minimum"
            } else {
                "> minimum"
            };
            return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                symbol: symbol.to_owned(),
                value,
                rule,
            });
        }
    }
    Ok(())
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
        let forcing = build_climate_runtime_request(climate)
            .map_err(|error| map_shared_error_for_hillslope(hillslope_id, &error))?;
        let mut day_symbol_surfaces = Vec::with_capacity(forcing.daily_forcing.len());
        for daily_forcing in &forcing.daily_forcing {
            day_symbol_surfaces.push(build_watershed_series_surface(hillslope_id, daily_forcing)?);
        }
        hillslope_forcing.insert(
            hillslope_id,
            WatershedHillslopeClimateAssignment {
                forcing,
                monthly: climate.monthly.clone(),
                day_symbol_surfaces,
            },
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
        let request = &assignment.forcing;
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
        insert_hillslope_monthly_climate_symbols(state_surface, hillslope_id, &assignment.monthly)?;

        let forcing = select_day_forcing(request, day_index)
            .map_err(|error| map_shared_error_for_hillslope(hillslope_id, &error))?;
        let day_symbols = assignment.day_symbol_surfaces.get(day_index).ok_or(
            WatershedClimateRuntimeInputError::DayIndexOutOfRange {
                hillslope_id,
                day_index,
                available: assignment.day_symbol_surfaces.len(),
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
                insert_series_values(state_surface, day_symbols.timem_symbols(), &day.timem);
                insert_series_values(state_surface, day_symbols.intsty_symbols(), &day.intsty);
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

                insert_series_values(state_surface, day_symbols.timem_symbols(), &day.timem);
                insert_series_values(state_surface, day_symbols.intsty_symbols(), &day.intsty);
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
    use std::collections::{BTreeMap, BTreeSet};
    use std::fmt::Write as _;

    use openwepp_input_contract::parsers::{
        chaninp::{ChaninpParseOptions, ParseMode, parse_chaninp_from_str},
        climate::{CompatibilityOptions, ParserMode as ClimateParserMode, parse_climate_from_str},
        slope::{SlopeParserOptions, parse_slope_str},
        watershed_channel::{WatershedChannelParseOptions, parse_watershed_channel_from_str},
        watershed_impoundment::{
            WatershedImpoundmentParseOptions, parse_watershed_impoundment_from_str,
        },
    };
    use openwepp_kernel_contract::BoundarySymbol;

    use super::{
        WatershedClimateRuntimeInputError, WatershedRuntimeInputError, WatershedWritebackSurface,
        build_watershed_runtime_surface_from_chaninp,
        build_watershed_runtime_surface_from_climate_assignments,
        seed_watershed_runtime_surface_from_slope_channel_profile,
        seed_watershed_runtime_surface_from_watershed_channel,
        seed_watershed_runtime_surface_from_watershed_impoundment,
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
    const WC1_UNPALATABLE_RIND_BREAKPOINT_NBRKPT_0: &str = include_str!(
        "../../../tests/fixtures/infile/climate/wc1_unpalatable_rind_breakpoint_nbrkpt_0.cli"
    );
    const WC1_CANOGA_DAY1: &str =
        include_str!("../../../tests/fixtures/infile/climate/wc1_canoga_day1.cli");
    const WC1_CANOGA_STMDUR_CAP: &str =
        include_str!("../../../tests/fixtures/infile/climate/wc1_canoga_stmdur_cap.cli");
    const STRICT_VALID_CHANINP: &str =
        include_str!("../../../tests/fixtures/infile/chaninp/strict_valid.chaninp");
    const STRICT_VALID_WATERSHED_CHANNEL: &str = include_str!(
        "../../../tests/fixtures/infile/watershed_channel/strict_valid_single_channel.chn"
    );
    const STRICT_ISHAPE_NATURALLY_ERODED_WATERSHED_CHANNEL: &str = include_str!(
        "../../../tests/fixtures/infile/watershed_channel/strict_ishape_naturally_eroded.chn"
    );
    const STRICT_VALID_SLOPE: &str =
        include_str!("../../../tests/fixtures/infile/slope/strict_valid_canonical.slp");
    const STRICT_VALID_WATERSHED_IMPOUNDMENT: &str = include_str!(
        "../../../tests/fixtures/infile/watershed_impoundment/strict_valid_minimal.imp"
    );
    const STRICT_VALID_WATERSHED_IMPOUNDMENT_ACTIVE: &str = include_str!(
        "../../../tests/fixtures/infile/watershed_impoundment/strict_valid_active_payloads.imp"
    );

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
    #[allow(clippy::similar_names, clippy::too_many_lines)]
    fn watershed_channel_runtime_seed_projects_ws10_symbols() {
        let parsed = parse_watershed_channel_from_str(
            STRICT_VALID_WATERSHED_CHANNEL,
            WatershedChannelParseOptions::default(),
        )
        .expect("strict watershed channel fixture should parse");

        let mut surface = WatershedWritebackSurface::default();
        seed_watershed_runtime_surface_from_watershed_channel(&mut surface, &parsed)
            .expect("ws10 channel runtime seed should project symbols");

        let chnn = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_channel_1_chnn"))
            .expect("ws10_channel_1_chnn should be present")
            .as_f64();
        let ctlslp = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_channel_1_ctlslp"))
            .expect("ws10_channel_1_ctlslp should be present")
            .as_f64();
        let conductivity = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_channel_1_chnk"))
            .expect("ws10_channel_1_chnk should be present")
            .as_f64();
        let ishape = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_channel_1_ishape"))
            .expect("ws10_channel_1_ishape should be present")
            .as_f64();
        let icntrl = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_channel_1_icntrl"))
            .expect("ws10_channel_1_icntrl should be present")
            .as_f64();
        let flgout = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_channel_1_flgout"))
            .expect("ws10_channel_1_flgout should be present")
            .as_f64();
        let chnz = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_channel_1_chnz"))
            .expect("ws10_channel_1_chnz should be present")
            .as_f64();
        let chnnbr = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_channel_1_chnnbr"))
            .expect("ws10_channel_1_chnnbr should be present")
            .as_f64();
        let chntcr = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_channel_1_chntcr"))
            .expect("ws10_channel_1_chntcr should be present")
            .as_f64();
        let chnedm = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_channel_1_chnedm"))
            .expect("ws10_channel_1_chnedm should be present")
            .as_f64();
        let chneds = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_channel_1_chneds"))
            .expect("ws10_channel_1_chneds should be present")
            .as_f64();
        let ctlz = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_channel_1_ctlz"))
            .expect("ws10_channel_1_ctlz should be present")
            .as_f64();
        let ctln = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_channel_1_ctln"))
            .expect("ws10_channel_1_ctln should be present")
            .as_f64();
        let rccoef = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_channel_1_rccoef"))
            .expect("ws10_channel_1_rccoef should be present")
            .as_f64();
        let rcexp = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_channel_1_rcexp"))
            .expect("ws10_channel_1_rcexp should be present")
            .as_f64();
        let rcoset = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_channel_1_rcoset"))
            .expect("ws10_channel_1_rcoset should be present")
            .as_f64();

        assert!((chnn - 0.04).abs() < 1e-12);
        assert!((ctlslp - 0.02).abs() < 1e-12);
        assert!((conductivity - 0.000_001).abs() < 1e-12);
        assert!((ishape - 1.0).abs() < 1e-12);
        assert!((icntrl - 4.0).abs() < 1e-12);
        assert!((flgout - 0.0).abs() < 1e-12);
        assert!((chnz - 19.99).abs() < 1e-12);
        assert!((chnnbr - 0.03).abs() < 1e-12);
        assert!((chntcr - 19.0).abs() < 1e-12);
        assert!((chnedm - 900.0).abs() < 1e-12);
        assert!((chneds - 0.0001).abs() < 1e-12);
        assert!((ctlz - 4.0).abs() < 1e-12);
        assert!((ctln - 0.04).abs() < 1e-12);
        assert!((rccoef - 1.25).abs() < 1e-12);
        assert!((rcexp - 1.5).abs() < 1e-12);
        assert!((rcoset - 0.1).abs() < 1e-12);
    }

    #[test]
    fn watershed_channel_runtime_seed_projects_naturally_eroded_ishape() {
        let parsed = parse_watershed_channel_from_str(
            STRICT_ISHAPE_NATURALLY_ERODED_WATERSHED_CHANNEL,
            WatershedChannelParseOptions::default(),
        )
        .expect("strict naturally eroded watershed channel fixture should parse");

        let mut surface = WatershedWritebackSurface::default();
        seed_watershed_runtime_surface_from_watershed_channel(&mut surface, &parsed)
            .expect("ws10 channel runtime seed should project naturally eroded ishape");

        let ishape = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_channel_1_ishape"))
            .expect("ws10_channel_1_ishape should be present")
            .as_f64();
        assert!((ishape - 3.0).abs() < 1e-12);
    }

    #[test]
    fn watershed_channel_runtime_seed_rejects_out_of_domain_ishape() {
        let mut parsed = parse_watershed_channel_from_str(
            STRICT_VALID_WATERSHED_CHANNEL,
            WatershedChannelParseOptions::default(),
        )
        .expect("strict watershed channel fixture should parse");
        parsed.channels[0].ishape = 4;

        let mut surface = WatershedWritebackSurface::default();
        let error = seed_watershed_runtime_surface_from_watershed_channel(&mut surface, &parsed)
            .expect_err("out-of-domain ishape must fail");

        assert_eq!(error.code(), "WS-RUNTIME-E-010");
        assert!(matches!(
            error,
            WatershedRuntimeInputError::ChannelSymbolOutOfDomain { symbol, .. }
            if symbol == "ws10_channel_1_ishape"
        ));
    }

    #[test]
    fn watershed_channel_runtime_seed_rejects_out_of_domain_ienslp() {
        let mut parsed = parse_watershed_channel_from_str(
            STRICT_VALID_WATERSHED_CHANNEL,
            WatershedChannelParseOptions::default(),
        )
        .expect("strict watershed channel fixture should parse");
        parsed.channels[0].ienslp = 3;

        let mut surface = WatershedWritebackSurface::default();
        let error = seed_watershed_runtime_surface_from_watershed_channel(&mut surface, &parsed)
            .expect_err("out-of-domain ienslp must fail");

        assert_eq!(error.code(), "WS-RUNTIME-E-010");
        assert!(matches!(
            error,
            WatershedRuntimeInputError::ChannelSymbolOutOfDomain { symbol, .. }
            if symbol == "ws10_channel_1_ienslp"
        ));
    }

    #[test]
    fn watershed_channel_runtime_seed_rejects_out_of_domain_icntrl() {
        let mut parsed = parse_watershed_channel_from_str(
            STRICT_VALID_WATERSHED_CHANNEL,
            WatershedChannelParseOptions::default(),
        )
        .expect("strict watershed channel fixture should parse");
        parsed.channels[0].icntrl = 5;

        let mut surface = WatershedWritebackSurface::default();
        let error = seed_watershed_runtime_surface_from_watershed_channel(&mut surface, &parsed)
            .expect_err("out-of-domain icntrl must fail");

        assert_eq!(error.code(), "WS-RUNTIME-E-010");
        assert!(matches!(
            error,
            WatershedRuntimeInputError::ChannelSymbolOutOfDomain { symbol, .. }
            if symbol == "ws10_channel_1_icntrl"
        ));
    }

    #[test]
    fn watershed_channel_runtime_seed_rejects_out_of_domain_flgout() {
        let mut parsed = parse_watershed_channel_from_str(
            STRICT_VALID_WATERSHED_CHANNEL,
            WatershedChannelParseOptions::default(),
        )
        .expect("strict watershed channel fixture should parse");
        parsed.channels[0].flgout = 2;

        let mut surface = WatershedWritebackSurface::default();
        let error = seed_watershed_runtime_surface_from_watershed_channel(&mut surface, &parsed)
            .expect_err("out-of-domain flgout must fail");

        assert_eq!(error.code(), "WS-RUNTIME-E-010");
        assert!(matches!(
            error,
            WatershedRuntimeInputError::ChannelSymbolOutOfDomain { symbol, .. }
            if symbol == "ws10_channel_1_flgout"
        ));
    }

    #[test]
    fn watershed_channel_runtime_seed_rejects_missing_rating_curve_payload_for_icntrl4() {
        let mut parsed = parse_watershed_channel_from_str(
            STRICT_VALID_WATERSHED_CHANNEL,
            WatershedChannelParseOptions::default(),
        )
        .expect("strict watershed channel fixture should parse");
        parsed.channels[0].rating_curve = None;

        let mut surface = WatershedWritebackSurface::default();
        let error = seed_watershed_runtime_surface_from_watershed_channel(&mut surface, &parsed)
            .expect_err("missing rating curve payload for icntrl=4 must fail");

        assert_eq!(error.code(), "WS-RUNTIME-E-010");
        assert!(matches!(
            error,
            WatershedRuntimeInputError::ChannelSymbolOutOfDomain { symbol, .. }
            if symbol == "ws10_channel_1_rating_curve"
        ));
    }

    #[test]
    fn watershed_channel_runtime_seed_rejects_rating_curve_payload_when_icntrl_not4() {
        let mut parsed = parse_watershed_channel_from_str(
            STRICT_VALID_WATERSHED_CHANNEL,
            WatershedChannelParseOptions::default(),
        )
        .expect("strict watershed channel fixture should parse");
        parsed.channels[0].icntrl = 3;

        let mut surface = WatershedWritebackSurface::default();
        let error = seed_watershed_runtime_surface_from_watershed_channel(&mut surface, &parsed)
            .expect_err("rating curve payload with icntrl!=4 must fail");

        assert_eq!(error.code(), "WS-RUNTIME-E-010");
        assert!(matches!(
            error,
            WatershedRuntimeInputError::ChannelSymbolOutOfDomain { symbol, .. }
            if symbol == "ws10_channel_1_rating_curve"
        ));
    }

    #[test]
    fn watershed_channel_runtime_seed_rejects_out_of_domain_rccoef() {
        let mut parsed = parse_watershed_channel_from_str(
            STRICT_VALID_WATERSHED_CHANNEL,
            WatershedChannelParseOptions::default(),
        )
        .expect("strict watershed channel fixture should parse");
        let rating_curve = parsed.channels[0]
            .rating_curve
            .as_mut()
            .expect("strict valid fixture should have rating curve");
        rating_curve.rccoef = 0.0;

        let mut surface = WatershedWritebackSurface::default();
        let error = seed_watershed_runtime_surface_from_watershed_channel(&mut surface, &parsed)
            .expect_err("non-positive rccoef must fail");

        assert_eq!(error.code(), "WS-RUNTIME-E-010");
        assert!(matches!(
            error,
            WatershedRuntimeInputError::ChannelSymbolOutOfDomain { symbol, .. }
            if symbol == "ws10_channel_1_rccoef"
        ));
    }

    #[test]
    fn watershed_channel_runtime_seed_rejects_out_of_domain_rcexp() {
        let mut parsed = parse_watershed_channel_from_str(
            STRICT_VALID_WATERSHED_CHANNEL,
            WatershedChannelParseOptions::default(),
        )
        .expect("strict watershed channel fixture should parse");
        let rating_curve = parsed.channels[0]
            .rating_curve
            .as_mut()
            .expect("strict valid fixture should have rating curve");
        rating_curve.rcexp = 0.0;

        let mut surface = WatershedWritebackSurface::default();
        let error = seed_watershed_runtime_surface_from_watershed_channel(&mut surface, &parsed)
            .expect_err("non-positive rcexp must fail");

        assert_eq!(error.code(), "WS-RUNTIME-E-010");
        assert!(matches!(
            error,
            WatershedRuntimeInputError::ChannelSymbolOutOfDomain { symbol, .. }
            if symbol == "ws10_channel_1_rcexp"
        ));
    }

    #[test]
    fn watershed_channel_runtime_seed_rejects_out_of_domain_rcoset() {
        let mut parsed = parse_watershed_channel_from_str(
            STRICT_VALID_WATERSHED_CHANNEL,
            WatershedChannelParseOptions::default(),
        )
        .expect("strict watershed channel fixture should parse");
        let rating_curve = parsed.channels[0]
            .rating_curve
            .as_mut()
            .expect("strict valid fixture should have rating curve");
        rating_curve.rcoset = -0.1;

        let mut surface = WatershedWritebackSurface::default();
        let error = seed_watershed_runtime_surface_from_watershed_channel(&mut surface, &parsed)
            .expect_err("negative rcoset must fail");

        assert_eq!(error.code(), "WS-RUNTIME-E-010");
        assert!(matches!(
            error,
            WatershedRuntimeInputError::ChannelSymbolOutOfDomain { symbol, .. }
            if symbol == "ws10_channel_1_rcoset"
        ));
    }

    #[test]
    fn watershed_channel_runtime_seed_rejects_chnn_less_than_chnnbr() {
        let mut parsed = parse_watershed_channel_from_str(
            STRICT_VALID_WATERSHED_CHANNEL,
            WatershedChannelParseOptions::default(),
        )
        .expect("strict watershed channel fixture should parse");
        parsed.channels[0].chnn = parsed.channels[0].chnnbr - 0.001;

        let mut surface = WatershedWritebackSurface::default();
        let error = seed_watershed_runtime_surface_from_watershed_channel(&mut surface, &parsed)
            .expect_err("chnn below chnnbr must fail");

        assert_eq!(error.code(), "WS-RUNTIME-E-010");
        assert!(matches!(
            error,
            WatershedRuntimeInputError::ChannelSymbolOutOfDomain { symbol, .. }
            if symbol == "ws10_channel_1_chnn"
        ));
    }

    #[test]
    fn watershed_channel_runtime_seed_rejects_out_of_domain_symbol() {
        let mut parsed = parse_watershed_channel_from_str(
            STRICT_VALID_WATERSHED_CHANNEL,
            WatershedChannelParseOptions::default(),
        )
        .expect("strict watershed channel fixture should parse");
        parsed.channels[0].chnn = 0.0;

        let mut surface = WatershedWritebackSurface::default();
        let error = seed_watershed_runtime_surface_from_watershed_channel(&mut surface, &parsed)
            .expect_err("non-positive channel roughness must fail");

        assert_eq!(error.code(), "WS-RUNTIME-E-010");
        assert!(matches!(
            error,
            WatershedRuntimeInputError::ChannelSymbolOutOfDomain { symbol, .. }
            if symbol == "ws10_channel_1_chnn"
        ));
    }

    #[test]
    fn watershed_channel_slope_runtime_seed_projects_ws17_segment_symbols() {
        let channel = parse_watershed_channel_from_str(
            STRICT_VALID_WATERSHED_CHANNEL,
            WatershedChannelParseOptions::default(),
        )
        .expect("strict watershed channel fixture should parse");
        let slope = parse_slope_str(STRICT_VALID_SLOPE, SlopeParserOptions::strict())
            .expect("strict slope fixture should parse");

        let mut surface = WatershedWritebackSurface::default();
        seed_watershed_runtime_surface_from_slope_channel_profile(&mut surface, &channel, &slope)
            .expect("ws17 slope-to-channel seeding should project segment symbols");

        let nslpts = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_channel_1_nslpts"))
            .expect("ws10_channel_1_nslpts should be present")
            .as_f64();
        let x2 = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_channel_1_x_0002"))
            .expect("ws10_channel_1_x_0002 should be present")
            .as_f64();
        let x3 = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_channel_1_x_0003"))
            .expect("ws10_channel_1_x_0003 should be present")
            .as_f64();
        let slope2 = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_channel_1_slope_0002"))
            .expect("ws10_channel_1_slope_0002 should be present")
            .as_f64();
        let depa2 = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_channel_1_depa_0002"))
            .expect("ws10_channel_1_depa_0002 should be present")
            .as_f64();
        let wida2 = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_channel_1_wida_0002"))
            .expect("ws10_channel_1_wida_0002 should be present")
            .as_f64();

        assert!((nslpts - 3.0).abs() < 1.0e-12);
        assert!((x2 - 36.0).abs() < 1.0e-12);
        assert!((x3 - 60.0).abs() < 1.0e-12);
        assert!((slope2 - 0.08).abs() < 1.0e-12);
        assert!((depa2 - 2_952.9).abs() < 1.0e-9);
        assert!((wida2 - 98.43).abs() < 1.0e-12);
    }

    #[test]
    fn watershed_channel_slope_runtime_seed_rejects_profile_count_mismatch() {
        let mut channel = parse_watershed_channel_from_str(
            STRICT_VALID_WATERSHED_CHANNEL,
            WatershedChannelParseOptions::default(),
        )
        .expect("strict watershed channel fixture should parse");
        let mut second = channel.channels[0].clone();
        second.channel_id = 2;
        channel.channels.push(second);

        let slope = parse_slope_str(
            "97.5\n1\n180.0 30.0\n3 60.0\n0.0 0.02 0.6 0.08 1.0 0.06\n",
            SlopeParserOptions::strict(),
        )
        .expect("single-profile slope fixture should parse");

        let mut surface = WatershedWritebackSurface::default();
        let error = seed_watershed_runtime_surface_from_slope_channel_profile(
            &mut surface,
            &channel,
            &slope,
        )
        .expect_err("slope profile count mismatch must fail");

        assert_eq!(error.code(), "WS-RUNTIME-E-010");
        assert!(matches!(
            error,
            WatershedRuntimeInputError::ChannelSymbolOutOfDomain { symbol, .. }
            if symbol == "ws10_channel_2_nslpts"
        ));
    }

    #[test]
    fn watershed_impoundment_runtime_seed_projects_ws10_symbols() {
        let parsed = parse_watershed_impoundment_from_str(
            STRICT_VALID_WATERSHED_IMPOUNDMENT,
            WatershedImpoundmentParseOptions::strict(),
        )
        .expect("strict watershed impoundment fixture should parse");

        let mut surface = WatershedWritebackSurface::default();
        seed_watershed_runtime_surface_from_watershed_impoundment(&mut surface, &parsed)
            .expect("ws10 impoundment runtime seed should project symbols");

        let h = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_impoundment_1_h"))
            .expect("ws10_impoundment_1_h should be present")
            .as_f64();
        let hfull = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_impoundment_1_hfull"))
            .expect("ws10_impoundment_1_hfull should be present")
            .as_f64();
        let deltat = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_impoundment_1_deltat"))
            .expect("ws10_impoundment_1_deltat should be present")
            .as_f64();
        let a0 = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_impoundment_1_a0"))
            .expect("ws10_impoundment_1_a0 should be present")
            .as_f64();
        let a1 = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_impoundment_1_a1"))
            .expect("ws10_impoundment_1_a1 should be present")
            .as_f64();
        let a2 = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_impoundment_1_a2"))
            .expect("ws10_impoundment_1_a2 should be present")
            .as_f64();
        let l0 = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_impoundment_1_l0"))
            .expect("ws10_impoundment_1_l0 should be present")
            .as_f64();
        let l1 = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_impoundment_1_l1"))
            .expect("ws10_impoundment_1_l1 should be present")
            .as_f64();
        let l2 = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_impoundment_1_l2"))
            .expect("ws10_impoundment_1_l2 should be present")
            .as_f64();
        let ha = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_impoundment_1_ha"))
            .expect("ws10_impoundment_1_ha should be present")
            .as_f64();
        let f01_ha = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_impoundment_1_f01_ha"))
            .expect("ws10_impoundment_1_f01_ha should be present")
            .as_f64();
        let f15_b = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_impoundment_1_f15_b"))
            .expect("ws10_impoundment_1_f15_b should be present")
            .as_f64();

        assert!((h - 0.70).abs() < 1e-12);
        assert!((hfull - 0.75).abs() < 1e-12);
        assert!((deltat - 1.0).abs() < 1e-12);
        assert!((a0 - 100.0).abs() < 1e-12);
        assert!(a1 > 0.0);
        assert!(a2 > 0.0);
        assert!((l0 - 20.0).abs() < 1e-12);
        assert!(l1 > 0.0);
        assert!(l2 > 0.0);
        assert!((ha - 0.75).abs() < 1e-12);
        assert!((f01_ha - 0.75).abs() < 1e-12);
        assert!(f15_b.abs() <= 1.0e-12);
    }

    #[test]
    fn watershed_impoundment_runtime_seed_projects_active_structure_coefficients() {
        let parsed = parse_watershed_impoundment_from_str(
            STRICT_VALID_WATERSHED_IMPOUNDMENT_ACTIVE,
            WatershedImpoundmentParseOptions::strict(),
        )
        .expect("strict watershed impoundment fixture should parse");
        assert!(
            parsed.items[0].structure_flags.has_drop_spillway,
            "fixture should carry active outlet structures"
        );

        let mut surface = WatershedWritebackSurface::default();
        seed_watershed_runtime_surface_from_watershed_impoundment(&mut surface, &parsed)
            .expect("active structure payloads should project runtime coefficient symbols");

        let a = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_impoundment_1_a"))
            .expect("ws10_impoundment_1_a should be present")
            .as_f64();
        let c = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_impoundment_1_c"))
            .expect("ws10_impoundment_1_c should be present")
            .as_f64();
        let e = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_impoundment_1_e"))
            .expect("ws10_impoundment_1_e should be present")
            .as_f64();
        let f01_b = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_impoundment_1_f01_b"))
            .expect("ws10_impoundment_1_f01_b should be present")
            .as_f64();
        let f04_a = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_impoundment_1_f04_a"))
            .expect("ws10_impoundment_1_f04_a should be present")
            .as_f64();
        let f10_d = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_impoundment_1_f10_d"))
            .expect("ws10_impoundment_1_f10_d should be present")
            .as_f64();
        let f13_b = surface
            .state_surface
            .get(&BoundarySymbol::from("ws10_impoundment_1_f13_b"))
            .expect("ws10_impoundment_1_f13_b should be present")
            .as_f64();

        assert!(a.is_finite() && a > 0.0);
        assert!(c.is_finite() && c > 0.0);
        assert!(e.is_finite() && e > 0.0);
        assert!(f01_b.is_finite() && f01_b > 0.0);
        assert!(f04_a.is_finite() && f04_a > 0.0);
        assert!(f10_d.is_finite() && f10_d > 0.0);
        assert!(f13_b.is_finite() && f13_b > 0.0);
    }

    #[test]
    fn watershed_impoundment_runtime_seed_rejects_h_above_hfull() {
        let mut parsed = parse_watershed_impoundment_from_str(
            STRICT_VALID_WATERSHED_IMPOUNDMENT,
            WatershedImpoundmentParseOptions::strict(),
        )
        .expect("strict watershed impoundment fixture should parse");
        parsed.items[0].h = parsed.items[0].hfull + 0.1;

        let mut surface = WatershedWritebackSurface::default();
        let error =
            seed_watershed_runtime_surface_from_watershed_impoundment(&mut surface, &parsed)
                .expect_err("impoundment stage above hfull must fail");

        assert_eq!(error.code(), "WS-RUNTIME-E-012");
        assert!(matches!(
            error,
            WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain { symbol, .. }
            if symbol == "ws10_impoundment_1_h"
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
    fn breakpoint_runtime_surface_accepts_curated_wc1_zero_breakpoint_dry_day() {
        let climate = parse_climate_from_str(
            WC1_UNPALATABLE_RIND_BREAKPOINT_NBRKPT_0,
            ClimateParserMode::Strict,
        )
        .expect("wc1 zero-breakpoint fixture should parse");
        let assignments = BTreeMap::from([(23_u32, climate)]);
        let surface = build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
            .expect("zero-breakpoint dry day should project watershed runtime surface");

        let nbrkpt = surface
            .state_surface
            .get(&BoundarySymbol::from("hs23_nbrkpt"))
            .expect("hs23_nbrkpt should exist")
            .as_f64();
        let prcp = surface
            .state_surface
            .get(&BoundarySymbol::from("hs23_prcp"))
            .expect("hs23_prcp should exist")
            .as_f64();
        let stmdur = surface
            .state_surface
            .get(&BoundarySymbol::from("hs23_stmdur"))
            .expect("hs23_stmdur should exist")
            .as_f64();
        let mxint = surface
            .state_surface
            .get(&BoundarySymbol::from("hs23_mxint"))
            .expect("hs23_mxint should exist")
            .as_f64();
        let stmstr = surface
            .state_surface
            .get(&BoundarySymbol::from("hs23_stmstr"))
            .expect("hs23_stmstr should exist")
            .as_f64();

        assert!(nbrkpt.abs() < 1e-12);
        assert!(prcp.abs() < 1e-12);
        assert!(stmdur.abs() < 1e-12);
        assert!(mxint.abs() < 1e-12);
        assert!(stmstr.abs() < 1e-12);
        assert!(
            !surface
                .state_surface
                .contains_key(&BoundarySymbol::from("hs23_timem_0001"))
        );
        assert!(
            !surface
                .state_surface
                .contains_key(&BoundarySymbol::from("hs23_intsty_0001"))
        );
    }

    #[test]
    fn climate_runtime_surface_accepts_breakpoint_cardinality_at_1500_boundary() {
        let climate =
            parse_climate_from_str(&build_breakpoint_fixture(1_500), ClimateParserMode::Strict)
                .expect("strict parser should accept 1500 breakpoint rows");
        let assignments = BTreeMap::from([(14_u32, climate)]);

        let surface = build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
            .expect("runtime seam should accept 1500 breakpoint rows");
        let nbrkpt = surface
            .state_surface
            .get(&BoundarySymbol::from("hs14_nbrkpt"))
            .expect("hs14_nbrkpt should exist")
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
        let assignments = BTreeMap::from([(16_u32, climate)]);

        let error = build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
            .expect_err("runtime seam must reject >1500 breakpoint rows");
        assert_eq!(error.code(), "CLIM-RUNTIME-E-011");
        assert!(matches!(
            error,
            WatershedClimateRuntimeInputError::BreakpointCardinalityPolicyExceeded {
                hillslope_id: 16,
                value: 1_501,
                max: 1_500
            }
        ));
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

    #[test]
    fn climate_runtime_surface_rejects_negative_breakpoint_drain() {
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
                record
                    .breakpoints
                    .first_mut()
                    .expect("first breakpoint point should exist")
                    .pptcum = 0.02;
                record
                    .breakpoints
                    .get_mut(1)
                    .expect("second breakpoint point should exist")
                    .pptcum = 0.01;
            }
            openwepp_input_contract::parsers::climate::ClimateDailyRecord::NoBreakpoint(_) => {
                panic!("expected breakpoint daily record")
            }
        }
        let assignments = BTreeMap::from([(2_u32, climate)]);

        let error = build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
            .expect_err("negative breakpoint drain must fail seam guard");
        assert_eq!(error.code(), "CLIM-RUNTIME-E-006");
        assert!(matches!(
            error,
            WatershedClimateRuntimeInputError::NegativeField {
                field: "drain",
                value
            } if value < 0.0
        ));
    }
}
