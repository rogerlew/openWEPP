#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TcParseMode {
    Strict,
    Compatibility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TcRunContext {
    Watershed,
    Hillslope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TcOpenResult {
    Missing,
    OpenSuccess,
    OpenErrorCollapsedCompat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TcWarningCode {
    TcW001,
    TcW002,
    TcW003,
}

impl TcWarningCode {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TcW001 => "TC-W-001",
            Self::TcW002 => "TC-W-002",
            Self::TcW003 => "TC-W-003",
        }
    }
}

impl fmt::Display for TcWarningCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TcWarning {
    pub code: TcWarningCode,
    pub message: String,
}

impl TcWarning {
    fn new(code: TcWarningCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TcParseOptions {
    pub mode: TcParseMode,
    pub requested_tc_output: bool,
    pub run_context: TcRunContext,
}

impl Default for TcParseOptions {
    fn default() -> Self {
        Self {
            mode: TcParseMode::Strict,
            requested_tc_output: false,
            run_context: TcRunContext::Watershed,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub struct TcParseResult {
    pub luntc_requested: i32,
    pub luntc: i32,
    pub tc_file_present: bool,
    pub payload_bytes: usize,
    pub payload_nonempty: bool,
    pub payload_ignored_warning_emitted: bool,
    pub open_result: TcOpenResult,
    pub run_context: TcRunContext,
    pub mode_divergence: bool,
    pub tc_out_expected: bool,
    pub warnings: Vec<TcWarning>,
}

#[derive(Debug)]
pub enum TcParseError {
    InputOpenError {
        path: PathBuf,
        source: io::Error,
    },
    UnsupportedRunContext {
        run_context: TcRunContext,
    },
    TcOutExpectationInvariant {
        luntc: i32,
        tc_out_expected: bool,
    },
    ModeClosureInvariant {
        luntc_requested: i32,
        luntc: i32,
        mode_divergence: bool,
    },
}

impl TcParseError {
    #[must_use]
    pub fn contract_error_id(&self) -> &'static str {
        match self {
            Self::InputOpenError { .. } => "TC-E-000",
            Self::UnsupportedRunContext { .. } => "TC-E-001",
            Self::TcOutExpectationInvariant { .. } => "TC-E-002",
            Self::ModeClosureInvariant { .. } => "TC-E-003",
        }
    }
}

impl fmt::Display for TcParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputOpenError { path, source } => write!(
                f,
                "{}: could not open {} ({source})",
                self.contract_error_id(),
                path.display()
            ),
            Self::UnsupportedRunContext { run_context } => write!(
                f,
                "{}: tc.txt is unsupported for run_context={run_context:?}",
                self.contract_error_id()
            ),
            Self::TcOutExpectationInvariant {
                luntc,
                tc_out_expected,
            } => write!(
                f,
                "{}: tc_out_expected={tc_out_expected} inconsistent with luntc={luntc}",
                self.contract_error_id()
            ),
            Self::ModeClosureInvariant {
                luntc_requested,
                luntc,
                mode_divergence,
            } => write!(
                f,
                "{}: mode_divergence={mode_divergence} inconsistent for requested={} effective={}",
                self.contract_error_id(),
                luntc_requested,
                luntc
            ),
        }
    }
}

impl std::error::Error for TcParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InputOpenError { source, .. } => Some(source),
            _ => None,
        }
    }
}

pub fn parse_tc_from_path(
    path: impl AsRef<Path>,
    options: TcParseOptions,
) -> Result<TcParseResult, TcParseError> {
    if options.run_context != TcRunContext::Watershed {
        return Err(TcParseError::UnsupportedRunContext {
            run_context: options.run_context,
        });
    }

    let requested = i32::from(options.requested_tc_output);
    let mut warnings = Vec::new();

    let (open_result, payload_bytes) = match fs::read(path.as_ref()) {
        Ok(payload) => (TcOpenResult::OpenSuccess, payload.len()),
        Err(source) if source.kind() == io::ErrorKind::NotFound => {
            if options.mode == TcParseMode::Compatibility {
                warnings.push(TcWarning::new(
                    TcWarningCode::TcW001,
                    "compatibility accepted missing optional tc.txt sentinel",
                ));
            }
            (TcOpenResult::Missing, 0usize)
        }
        Err(source) => {
            if options.mode == TcParseMode::Strict {
                return Err(TcParseError::InputOpenError {
                    path: path.as_ref().to_path_buf(),
                    source,
                });
            }
            warnings.push(TcWarning::new(
                TcWarningCode::TcW002,
                format!(
                    "compatibility collapsed non-ENOENT tc.txt open error into missing branch ({source})"
                ),
            ));
            (TcOpenResult::OpenErrorCollapsedCompat, 0usize)
        }
    };

    let tc_file_present = matches!(open_result, TcOpenResult::OpenSuccess);
    let payload_nonempty = payload_bytes > 0;
    let mut payload_ignored_warning_emitted = false;

    if tc_file_present && payload_nonempty && options.mode == TcParseMode::Compatibility {
        warnings.push(TcWarning::new(
            TcWarningCode::TcW003,
            "compatibility ignored non-empty tc.txt sentinel payload body",
        ));
        payload_ignored_warning_emitted = true;
    }

    let luntc = i32::from(tc_file_present);
    let mode_divergence = requested != luntc;
    let tc_out_expected = luntc == 1;

    if tc_out_expected != (luntc == 1) {
        return Err(TcParseError::TcOutExpectationInvariant {
            luntc,
            tc_out_expected,
        });
    }

    if mode_divergence != (requested != luntc) {
        return Err(TcParseError::ModeClosureInvariant {
            luntc_requested: requested,
            luntc,
            mode_divergence,
        });
    }

    Ok(TcParseResult {
        luntc_requested: requested,
        luntc,
        tc_file_present,
        payload_bytes,
        payload_nonempty,
        payload_ignored_warning_emitted,
        open_result,
        run_context: options.run_context,
        mode_divergence,
        tc_out_expected,
        warnings,
    })
}
