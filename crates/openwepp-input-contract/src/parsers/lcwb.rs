#![allow(clippy::missing_errors_doc)]

use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LcwbParserMode {
    Strict,
    Compatibility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LcwbRunContext {
    Watershed,
    Hillslope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LcwbOpenResult {
    Missing,
    OpenSuccess,
    OpenErrorCollapsedCompat,
    NotApplicableCompat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LcwbOfeRowSelectionPolicyMode {
    LastOfeOnly,
    AllOfe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LcwbWarningCode {
    LcwbW001,
    LcwbW002,
    LcwbW003,
    LcwbW004,
}

impl LcwbWarningCode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LcwbW001 => "LCWB-W-001",
            Self::LcwbW002 => "LCWB-W-002",
            Self::LcwbW003 => "LCWB-W-003",
            Self::LcwbW004 => "LCWB-W-004",
        }
    }
}

impl fmt::Display for LcwbWarningCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LcwbWarning {
    pub code: LcwbWarningCode,
    pub message: String,
}

impl LcwbWarning {
    fn new(code: LcwbWarningCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LcwbParserOptions {
    pub mode: LcwbParserMode,
    pub run_context: LcwbRunContext,
    pub requested_channel_watbal_mode: bool,
}

impl Default for LcwbParserOptions {
    fn default() -> Self {
        Self {
            mode: LcwbParserMode::Strict,
            run_context: LcwbRunContext::Watershed,
            requested_channel_watbal_mode: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::struct_excessive_bools)]
pub struct LcwbParseResult {
    pub lcwb_requested: i32,
    pub lcwbflg: i32,
    pub lcwb_file_present: bool,
    pub payload_bytes: usize,
    pub payload_nonempty: bool,
    pub payload_nonwhitespace: bool,
    pub payload_ignored_warning_emitted: bool,
    pub open_result: LcwbOpenResult,
    pub run_context: LcwbRunContext,
    pub mode_divergence: bool,
    pub ofe_row_selection_policy_mode: LcwbOfeRowSelectionPolicyMode,
    pub warnings: Vec<LcwbWarning>,
}

#[derive(Debug)]
pub enum LcwbParseError {
    InputOpenError {
        path: PathBuf,
        source: io::Error,
    },
    SentinelPayloadNotEmpty {
        path: PathBuf,
        payload_bytes: usize,
    },
    UnsupportedRunContext {
        run_context: LcwbRunContext,
    },
    ModeClosureMismatch {
        lcwb_requested: i32,
        lcwbflg: i32,
        open_result: LcwbOpenResult,
    },
}

impl LcwbParseError {
    #[must_use]
    pub const fn contract_error_id(&self) -> &'static str {
        match self {
            Self::InputOpenError { .. } => "LCWB-E-000",
            Self::SentinelPayloadNotEmpty { .. } => "LCWB-E-001",
            Self::UnsupportedRunContext { .. } => "LCWB-E-002",
            Self::ModeClosureMismatch { .. } => "LCWB-E-003",
        }
    }
}

impl fmt::Display for LcwbParseError {
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
                "{}: strict mode requires empty/whitespace-only sentinel but {} has {} byte(s)",
                self.contract_error_id(),
                path.display(),
                payload_bytes
            ),
            Self::UnsupportedRunContext { run_context } => write!(
                f,
                "{}: lcwb parser is watershed-only; got run_context={run_context:?}",
                self.contract_error_id()
            ),
            Self::ModeClosureMismatch {
                lcwb_requested,
                lcwbflg,
                open_result,
            } => write!(
                f,
                "{}: requested lcwb={} but effective lcwbflg={} for open_result={open_result:?}",
                self.contract_error_id(),
                lcwb_requested,
                lcwbflg
            ),
        }
    }
}

impl std::error::Error for LcwbParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InputOpenError { source, .. } => Some(source),
            _ => None,
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn parse_lcwb_from_path(
    path: impl AsRef<Path>,
    options: LcwbParserOptions,
) -> Result<LcwbParseResult, LcwbParseError> {
    let path = path.as_ref();
    let requested = i32::from(options.requested_channel_watbal_mode);

    if options.run_context != LcwbRunContext::Watershed {
        if options.mode == LcwbParserMode::Strict {
            return Err(LcwbParseError::UnsupportedRunContext {
                run_context: options.run_context,
            });
        }

        let lcwbflg = 0;
        return Ok(LcwbParseResult {
            lcwb_requested: requested,
            lcwbflg,
            lcwb_file_present: false,
            payload_bytes: 0,
            payload_nonempty: false,
            payload_nonwhitespace: false,
            payload_ignored_warning_emitted: false,
            open_result: LcwbOpenResult::NotApplicableCompat,
            run_context: options.run_context,
            mode_divergence: requested != lcwbflg,
            ofe_row_selection_policy_mode: derive_ofe_policy_mode(lcwbflg),
            warnings: vec![LcwbWarning::new(
                LcwbWarningCode::LcwbW004,
                "compatibility treated non-watershed context as typed not-applicable branch",
            )],
        });
    }

    let mut warnings = Vec::new();
    let mut payload_ignored_warning_emitted = false;

    let (open_result, payload, lcwb_file_present) = match fs::read(path) {
        Ok(payload) => (LcwbOpenResult::OpenSuccess, payload, true),
        Err(source) if source.kind() == io::ErrorKind::NotFound => {
            if options.mode == LcwbParserMode::Compatibility {
                warnings.push(LcwbWarning::new(
                    LcwbWarningCode::LcwbW001,
                    "compatibility optional-surface missing branch defaulted lcwbflg=0",
                ));
            }
            (LcwbOpenResult::Missing, Vec::new(), false)
        }
        Err(source) => {
            if options.mode == LcwbParserMode::Strict {
                return Err(LcwbParseError::InputOpenError {
                    path: path.to_path_buf(),
                    source,
                });
            }

            warnings.push(LcwbWarning::new(
                LcwbWarningCode::LcwbW003,
                format!(
                    "compatibility collapsed non-ENOENT open error into missing branch ({source})"
                ),
            ));
            warnings.push(LcwbWarning::new(
                LcwbWarningCode::LcwbW001,
                "compatibility optional-surface missing branch defaulted lcwbflg=0 after open-error collapse",
            ));
            (LcwbOpenResult::OpenErrorCollapsedCompat, Vec::new(), false)
        }
    };

    let payload_bytes = payload.len();
    let payload_nonempty = payload_bytes > 0;
    let payload_nonwhitespace = payload.iter().any(|byte| !byte.is_ascii_whitespace());

    if lcwb_file_present && payload_nonwhitespace {
        if options.mode == LcwbParserMode::Strict {
            return Err(LcwbParseError::SentinelPayloadNotEmpty {
                path: path.to_path_buf(),
                payload_bytes,
            });
        }

        warnings.push(LcwbWarning::new(
            LcwbWarningCode::LcwbW002,
            "compatibility accepted non-empty sentinel payload and ignored body",
        ));
        payload_ignored_warning_emitted = true;
    }

    let lcwbflg = i32::from(lcwb_file_present);
    let mode_divergence = requested != lcwbflg;

    if mode_divergence && options.mode == LcwbParserMode::Strict {
        return Err(LcwbParseError::ModeClosureMismatch {
            lcwb_requested: requested,
            lcwbflg,
            open_result,
        });
    }

    Ok(LcwbParseResult {
        lcwb_requested: requested,
        lcwbflg,
        lcwb_file_present,
        payload_bytes,
        payload_nonempty,
        payload_nonwhitespace,
        payload_ignored_warning_emitted,
        open_result,
        run_context: options.run_context,
        mode_divergence,
        ofe_row_selection_policy_mode: derive_ofe_policy_mode(lcwbflg),
        warnings,
    })
}

const fn derive_ofe_policy_mode(lcwbflg: i32) -> LcwbOfeRowSelectionPolicyMode {
    if lcwbflg == 1 {
        LcwbOfeRowSelectionPolicyMode::LastOfeOnly
    } else {
        LcwbOfeRowSelectionPolicyMode::AllOfe
    }
}
