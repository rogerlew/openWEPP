use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use openwepp_input_contract::parsers::chaninp::{ChaninpFile, ChaninpParseOutcome};
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use openwepp_input_contract::parsers::chaninp::{
        ChaninpParseOptions, ParseMode, parse_chaninp_from_str,
    };
    use openwepp_kernel_contract::BoundarySymbol;

    use super::{WatershedRuntimeInputError, build_watershed_runtime_surface_from_chaninp};

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
}
