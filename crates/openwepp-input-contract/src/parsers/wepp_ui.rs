#![allow(clippy::missing_errors_doc, clippy::too_many_lines)]

use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const SOIL_MIN_HOURLY_VERSION: f64 = 7778.0;
const FLOAT_TOLERANCE: f64 = 1e-9;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeppUiParserMode {
    Strict,
    Compatibility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeppUiOpenResult {
    Missing,
    OpenSuccess,
    OpenErrorCollapsedCompat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeppUiSoilCompatibilityState {
    Compatible7778OrNewer,
    Legacy2006,
    Unresolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeppUiWarningCode {
    WuiW001,
    WuiW002,
    WuiW003,
    WuiW004,
}

impl WeppUiWarningCode {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WuiW001 => "WUI-W-001",
            Self::WuiW002 => "WUI-W-002",
            Self::WuiW003 => "WUI-W-003",
            Self::WuiW004 => "WUI-W-004",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeppUiWarning {
    pub code: WeppUiWarningCode,
    pub message: String,
}

impl WeppUiWarning {
    fn new(code: WeppUiWarningCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WeppUiParserOptions {
    pub mode: WeppUiParserMode,
    pub requested_hourly_seepage: bool,
    pub soil_versions: Vec<f64>,
}

impl Default for WeppUiParserOptions {
    fn default() -> Self {
        Self {
            mode: WeppUiParserMode::Strict,
            requested_hourly_seepage: false,
            soil_versions: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WeppUiParseResult {
    pub ui_run_requested: i32,
    pub ui_run: i32,
    pub wepp_ui_file_present: bool,
    pub payload_bytes: usize,
    pub payload_nonempty: bool,
    pub open_result: WeppUiOpenResult,
    pub solwpv: Vec<f64>,
    pub solwpv_reduced_min: Option<f64>,
    pub soil_compatibility_state: WeppUiSoilCompatibilityState,
    pub mode_divergence: bool,
    pub warnings: Vec<WeppUiWarning>,
}

#[derive(Debug)]
pub enum WeppUiParseError {
    InputOpenError {
        path: PathBuf,
        source: io::Error,
    },
    SentinelPayloadNotEmpty {
        path: PathBuf,
        payload_bytes: usize,
    },
    SoilCompatibilityStrict {
        solwpv_reduced_min: f64,
    },
    ModeClosureMismatch {
        ui_run_requested: i32,
        ui_run: i32,
        open_result: WeppUiOpenResult,
    },
    MissingSoilVersionSurface {
        reason: &'static str,
    },
}

impl WeppUiParseError {
    #[must_use]
    pub fn contract_error_id(&self) -> &'static str {
        match self {
            Self::InputOpenError { .. } => "WUI-E-000",
            Self::SentinelPayloadNotEmpty { .. } => "WUI-E-001",
            Self::SoilCompatibilityStrict { .. } => "WUI-E-002",
            Self::ModeClosureMismatch { .. } => "WUI-E-003",
            Self::MissingSoilVersionSurface { .. } => "WUI-E-004",
        }
    }
}

impl fmt::Display for WeppUiParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputOpenError { path, source } => write!(
                f,
                "{}: could not open {} ({source})",
                self.contract_error_id(),
                path.display()
            ),
            Self::SentinelPayloadNotEmpty {
                path,
                payload_bytes,
            } => write!(
                f,
                "{}: strict mode requires empty sentinel but {} has {} byte(s)",
                self.contract_error_id(),
                path.display(),
                payload_bytes
            ),
            Self::SoilCompatibilityStrict { solwpv_reduced_min } => write!(
                f,
                "{}: strict hourly mode requires soil version >= {SOIL_MIN_HOURLY_VERSION}; observed reduced min {}",
                self.contract_error_id(),
                solwpv_reduced_min
            ),
            Self::ModeClosureMismatch {
                ui_run_requested,
                ui_run,
                open_result,
            } => write!(
                f,
                "{}: requested ui_run={} but effective ui_run={} for open_result={open_result:?}",
                self.contract_error_id(),
                ui_run_requested,
                ui_run
            ),
            Self::MissingSoilVersionSurface { reason } => write!(
                f,
                "{}: missing/invalid soil version surface ({reason})",
                self.contract_error_id()
            ),
        }
    }
}

impl std::error::Error for WeppUiParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InputOpenError { source, .. } => Some(source),
            _ => None,
        }
    }
}

pub fn parse_wepp_ui_from_path(
    path: impl AsRef<Path>,
    options: WeppUiParserOptions,
) -> Result<WeppUiParseResult, WeppUiParseError> {
    let path = path.as_ref();
    let requested = i32::from(options.requested_hourly_seepage);
    let mut warnings = Vec::new();

    let (open_result, payload_bytes) = match fs::read(path) {
        Ok(payload) => (WeppUiOpenResult::OpenSuccess, payload.len()),
        Err(source) if source.kind() == io::ErrorKind::NotFound => {
            if options.mode == WeppUiParserMode::Compatibility && options.requested_hourly_seepage {
                warnings.push(WeppUiWarning::new(
                    WeppUiWarningCode::WuiW001,
                    "compatibility fallback: sentinel missing, defaulted to daily mode",
                ));
            }
            (WeppUiOpenResult::Missing, 0usize)
        }
        Err(source) => {
            if options.mode == WeppUiParserMode::Strict {
                return Err(WeppUiParseError::InputOpenError {
                    path: path.to_path_buf(),
                    source,
                });
            }
            warnings.push(WeppUiWarning::new(
                WeppUiWarningCode::WuiW004,
                format!(
                    "compatibility collapsed non-ENOENT open error into missing branch ({source})"
                ),
            ));
            if options.requested_hourly_seepage {
                warnings.push(WeppUiWarning::new(
                    WeppUiWarningCode::WuiW001,
                    "compatibility fallback: sentinel missing after open-error collapse, defaulted to daily mode",
                ));
            }
            (WeppUiOpenResult::OpenErrorCollapsedCompat, 0usize)
        }
    };

    let wepp_ui_file_present = matches!(open_result, WeppUiOpenResult::OpenSuccess);
    let payload_nonempty = payload_bytes > 0;

    if wepp_ui_file_present && payload_nonempty {
        if options.mode == WeppUiParserMode::Strict {
            return Err(WeppUiParseError::SentinelPayloadNotEmpty {
                path: path.to_path_buf(),
                payload_bytes,
            });
        }
        warnings.push(WeppUiWarning::new(
            WeppUiWarningCode::WuiW002,
            "compatibility accepted non-empty sentinel payload and ignored body",
        ));
    }

    let ui_run = i32::from(wepp_ui_file_present);
    let mode_divergence = requested != ui_run;
    if mode_divergence && options.mode == WeppUiParserMode::Strict {
        return Err(WeppUiParseError::ModeClosureMismatch {
            ui_run_requested: requested,
            ui_run,
            open_result,
        });
    }

    let mut solwpv_reduced_min = None;
    let mut soil_compatibility_state = WeppUiSoilCompatibilityState::Unresolved;
    if ui_run == 1 {
        let reduced_min = reduce_min_finite(&options.soil_versions);
        if reduced_min.is_none() && options.mode == WeppUiParserMode::Strict {
            return Err(WeppUiParseError::MissingSoilVersionSurface {
                reason: "hourly mode requires at least one finite soil version",
            });
        }

        if let Some(value) = reduced_min {
            solwpv_reduced_min = Some(value);
            if value + FLOAT_TOLERANCE >= SOIL_MIN_HOURLY_VERSION {
                soil_compatibility_state = WeppUiSoilCompatibilityState::Compatible7778OrNewer;
            } else {
                soil_compatibility_state = WeppUiSoilCompatibilityState::Legacy2006;
                if options.mode == WeppUiParserMode::Strict {
                    return Err(WeppUiParseError::SoilCompatibilityStrict {
                        solwpv_reduced_min: value,
                    });
                }
                warnings.push(WeppUiWarning::new(
                    WeppUiWarningCode::WuiW003,
                    format!(
                        "compatibility accepted non-recommended soil version min {value} for hourly mode"
                    ),
                ));
            }
        }
    }

    Ok(WeppUiParseResult {
        ui_run_requested: requested,
        ui_run,
        wepp_ui_file_present,
        payload_bytes,
        payload_nonempty,
        open_result,
        solwpv: options.soil_versions,
        solwpv_reduced_min,
        soil_compatibility_state,
        mode_divergence,
        warnings,
    })
}

fn reduce_min_finite(values: &[f64]) -> Option<f64> {
    values
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .fold(None, |acc: Option<f64>, value| match acc {
            Some(current) => Some(current.min(value)),
            None => Some(value),
        })
}
