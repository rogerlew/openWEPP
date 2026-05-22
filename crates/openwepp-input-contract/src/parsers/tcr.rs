#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::too_many_lines
)]

use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const FLOAT_TOLERANCE: f64 = 1e-12;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TcrParseMode {
    Strict,
    Compatibility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TcrOpenResult {
    Missing,
    OpenSuccess,
    OpenErrorCollapsedCompat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TcrParseOutcome {
    MissingBranch,
    ParsedBranch,
    OpenErrorCollapsedCompat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TcrWarningCode {
    TcrW001,
    TcrW002,
    TcrW003,
}

impl TcrWarningCode {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TcrW001 => "TCR-W-001",
            Self::TcrW002 => "TCR-W-002",
            Self::TcrW003 => "TCR-W-003",
        }
    }
}

impl fmt::Display for TcrWarningCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TcrWarning {
    pub code: TcrWarningCode,
    pub message: String,
}

impl TcrWarning {
    fn new(code: TcrWarningCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TcrChannelContext {
    pub nchan: usize,
    pub channel_element_ids: Vec<i32>,
    pub chnslp_terminal: Vec<f64>,
    pub chntcr_from_channel_file: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TcrParseOptions {
    pub mode: TcrParseMode,
    pub channel_context: Option<TcrChannelContext>,
}

impl Default for TcrParseOptions {
    fn default() -> Self {
        Self {
            mode: TcrParseMode::Strict,
            channel_context: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub struct TcrParseOutput {
    pub taumin: Option<f64>,
    pub taumax: Option<f64>,
    pub kch: Option<f64>,
    pub nch: Option<f64>,
    pub tcr_file_present: bool,
    pub tcrflg: i32,
    pub parse_outcome: TcrParseOutcome,
    pub open_result: TcrOpenResult,
    pub line_count_closed: bool,
    pub trailing_token_lines: Vec<usize>,
    pub taumin_taumax_relational_warning_emitted: bool,
    pub chntcr_override_applied: bool,
    pub effective_chntcr: Vec<f64>,
    pub warnings: Vec<TcrWarning>,
}

#[derive(Debug)]
pub enum TcrParseError {
    InputOpenError {
        path: PathBuf,
        source: io::Error,
    },
    TokenParseError {
        line: usize,
        field: &'static str,
        token: String,
    },
    RecordCountError {
        expected: usize,
        found: usize,
    },
    NonFiniteError {
        line: usize,
        field: &'static str,
        value: f64,
    },
    DomainError {
        line: usize,
        field: &'static str,
        value: f64,
        allowed: &'static str,
    },
    CrossFileDependencyError {
        field: &'static str,
        message: String,
    },
    UnsupportedPrefixedVariant {
        line: usize,
        token: String,
    },
    CurveDomainError {
        index: usize,
        channel_id: i32,
        slope: f64,
        denominator: f64,
        message: &'static str,
    },
    RelationalInvariantError {
        taumin: f64,
        taumax: f64,
    },
}

impl TcrParseError {
    #[must_use]
    pub fn contract_error_id(&self) -> &'static str {
        match self {
            Self::InputOpenError { .. } => "TCR-E-000",
            Self::TokenParseError { .. } => "TCR-E-001",
            Self::RecordCountError { .. } => "TCR-E-002",
            Self::NonFiniteError { .. } => "TCR-E-003",
            Self::DomainError { .. } => "TCR-E-004",
            Self::CrossFileDependencyError { .. } => "TCR-E-005",
            Self::UnsupportedPrefixedVariant { .. } => "TCR-E-007",
            Self::CurveDomainError { .. } => "TCR-E-008",
            Self::RelationalInvariantError { .. } => "TCR-E-009",
        }
    }
}

impl fmt::Display for TcrParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputOpenError { path, source } => write!(
                f,
                "{}: could not open/read '{}': {source}",
                self.contract_error_id(),
                path.display()
            ),
            Self::TokenParseError { line, field, token } => write!(
                f,
                "{}: line {line} token parse error for {field} from '{token}'",
                self.contract_error_id()
            ),
            Self::RecordCountError { expected, found } => write!(
                f,
                "{}: expected {expected} records, found {found}",
                self.contract_error_id()
            ),
            Self::NonFiniteError { line, field, value } => write!(
                f,
                "{}: line {line} non-finite value for {field} ({value})",
                self.contract_error_id()
            ),
            Self::DomainError {
                line,
                field,
                value,
                allowed,
            } => write!(
                f,
                "{}: line {line} domain violation for {field} ({value}); expected {allowed}",
                self.contract_error_id()
            ),
            Self::CrossFileDependencyError { field, message } => write!(
                f,
                "{}: cross-file dependency error for {field}: {message}",
                self.contract_error_id()
            ),
            Self::UnsupportedPrefixedVariant { line, token } => write!(
                f,
                "{}: line {line} unsupported prefixed/datver-like variant token '{token}'",
                self.contract_error_id()
            ),
            Self::CurveDomainError {
                index,
                channel_id,
                slope,
                denominator,
                message,
            } => write!(
                f,
                "{}: channel index {index} id {channel_id} slope {slope} denominator {denominator} invalid ({message})",
                self.contract_error_id()
            ),
            Self::RelationalInvariantError { taumin, taumax } => write!(
                f,
                "{}: relational invariant violated (taumin <= taumax) for taumin={taumin}, taumax={taumax}",
                self.contract_error_id()
            ),
        }
    }
}

impl std::error::Error for TcrParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InputOpenError { source, .. } => Some(source),
            _ => None,
        }
    }
}

pub fn parse_tcr_from_path(
    path: impl AsRef<Path>,
    options: TcrParseOptions,
) -> Result<TcrParseOutput, TcrParseError> {
    let path = path.as_ref();

    match fs::read_to_string(path) {
        Ok(content) => parse_tcr_from_str(&content, options),
        Err(source) if source.kind() == io::ErrorKind::NotFound => Ok(missing_output(options.mode)),
        Err(source) => {
            if options.mode == TcrParseMode::Strict {
                Err(TcrParseError::InputOpenError {
                    path: path.to_path_buf(),
                    source,
                })
            } else {
                let mut output = missing_output(TcrParseMode::Compatibility);
                output.open_result = TcrOpenResult::OpenErrorCollapsedCompat;
                output.parse_outcome = TcrParseOutcome::OpenErrorCollapsedCompat;
                output.warnings.push(TcrWarning::new(
                    TcrWarningCode::TcrW002,
                    format!(
                        "compatibility collapsed non-ENOENT open error into missing branch ({source})"
                    ),
                ));
                Ok(output)
            }
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn parse_tcr_from_str(
    input: &str,
    options: TcrParseOptions,
) -> Result<TcrParseOutput, TcrParseError> {
    let lines = materialize_nonempty_lines(input);

    if lines.is_empty() {
        if options.mode == TcrParseMode::Compatibility {
            return Ok(missing_output(TcrParseMode::Compatibility));
        }
        return Err(TcrParseError::RecordCountError {
            expected: 4,
            found: 0,
        });
    }

    if detect_prefixed_variant(lines[0].text) {
        let token = first_token(lines[0].text).to_string();
        return Err(TcrParseError::UnsupportedPrefixedVariant {
            line: lines[0].number,
            token,
        });
    }

    if lines.len() != 4 {
        return Err(TcrParseError::RecordCountError {
            expected: 4,
            found: lines.len(),
        });
    }

    let mut trailing_token_lines = Vec::new();
    let taumin = parse_canonical_scalar(lines[0], "taumin", &mut trailing_token_lines)?;
    let taumax = parse_canonical_scalar(lines[1], "taumax", &mut trailing_token_lines)?;
    let kch = parse_canonical_scalar(lines[2], "kch", &mut trailing_token_lines)?;
    let nch = parse_canonical_scalar(lines[3], "nch", &mut trailing_token_lines)?;

    ensure_finite(lines[0].number, "taumin", taumin)?;
    ensure_finite(lines[1].number, "taumax", taumax)?;
    ensure_finite(lines[2].number, "kch", kch)?;
    ensure_finite(lines[3].number, "nch", nch)?;

    if taumin < 0.0 {
        return Err(TcrParseError::DomainError {
            line: lines[0].number,
            field: "taumin",
            value: taumin,
            allowed: ">= 0",
        });
    }
    if taumax < 0.0 {
        return Err(TcrParseError::DomainError {
            line: lines[1].number,
            field: "taumax",
            value: taumax,
            allowed: ">= 0",
        });
    }
    if kch <= 0.0 {
        return Err(TcrParseError::DomainError {
            line: lines[2].number,
            field: "kch",
            value: kch,
            allowed: "> 0",
        });
    }
    if nch <= 0.0 {
        return Err(TcrParseError::DomainError {
            line: lines[3].number,
            field: "nch",
            value: nch,
            allowed: "> 0",
        });
    }

    let mut warnings = Vec::new();
    let mut relational_warning = false;
    if taumin > taumax + FLOAT_TOLERANCE {
        if options.mode == TcrParseMode::Strict {
            return Err(TcrParseError::RelationalInvariantError { taumin, taumax });
        }
        warnings.push(TcrWarning::new(
            TcrWarningCode::TcrW003,
            "compatibility accepted taumin > taumax and preserved legacy value flow",
        ));
        relational_warning = true;
    }

    let mut output = TcrParseOutput {
        taumin: Some(taumin),
        taumax: Some(taumax),
        kch: Some(kch),
        nch: Some(nch),
        tcr_file_present: true,
        tcrflg: 1,
        parse_outcome: TcrParseOutcome::ParsedBranch,
        open_result: TcrOpenResult::OpenSuccess,
        line_count_closed: true,
        trailing_token_lines,
        taumin_taumax_relational_warning_emitted: relational_warning,
        chntcr_override_applied: false,
        effective_chntcr: Vec::new(),
        warnings,
    };

    apply_cross_file_override(&mut output, options.channel_context.as_ref())?;

    Ok(output)
}

#[derive(Clone, Copy)]
struct LocatedLine<'a> {
    number: usize,
    text: &'a str,
}

fn materialize_nonempty_lines(input: &str) -> Vec<LocatedLine<'_>> {
    input
        .lines()
        .enumerate()
        .filter_map(|(idx, raw)| {
            let trimmed = raw.trim();
            if trimmed.is_empty() {
                return None;
            }
            Some(LocatedLine {
                number: idx + 1,
                text: trimmed,
            })
        })
        .collect()
}

fn detect_prefixed_variant(line: &str) -> bool {
    let token = first_token(line);
    token.chars().any(|ch| ch.is_ascii_alphabetic())
}

fn first_token(line: &str) -> &str {
    line.split_whitespace().next().unwrap_or(line)
}

fn parse_canonical_scalar(
    line: LocatedLine<'_>,
    field: &'static str,
    trailing_token_lines: &mut Vec<usize>,
) -> Result<f64, TcrParseError> {
    let mut tokens = line.text.split_whitespace();
    let first = tokens.next().ok_or(TcrParseError::TokenParseError {
        line: line.number,
        field,
        token: String::new(),
    })?;

    if tokens.next().is_some() {
        trailing_token_lines.push(line.number);
    }

    first
        .parse::<f64>()
        .map_err(|_| TcrParseError::TokenParseError {
            line: line.number,
            field,
            token: first.to_string(),
        })
}

fn ensure_finite(line: usize, field: &'static str, value: f64) -> Result<(), TcrParseError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(TcrParseError::NonFiniteError { line, field, value })
    }
}

fn apply_cross_file_override(
    output: &mut TcrParseOutput,
    channel_context: Option<&TcrChannelContext>,
) -> Result<(), TcrParseError> {
    let context = channel_context.ok_or_else(|| TcrParseError::CrossFileDependencyError {
        field: "channel_context",
        message: "missing channel topology/slope context for tcr override".to_string(),
    })?;

    if context.nchan == 0 {
        return Err(TcrParseError::CrossFileDependencyError {
            field: "nchan",
            message: "nchan must be > 0 when tcr sidecar is present".to_string(),
        });
    }
    if context.channel_element_ids.len() != context.nchan {
        return Err(TcrParseError::CrossFileDependencyError {
            field: "channel_element_ids",
            message: format!(
                "expected {} channel IDs, found {}",
                context.nchan,
                context.channel_element_ids.len()
            ),
        });
    }
    if context.chnslp_terminal.len() != context.nchan {
        return Err(TcrParseError::CrossFileDependencyError {
            field: "chnslp_terminal",
            message: format!(
                "expected {} terminal slopes, found {}",
                context.nchan,
                context.chnslp_terminal.len()
            ),
        });
    }
    if context.chntcr_from_channel_file.len() != context.nchan {
        return Err(TcrParseError::CrossFileDependencyError {
            field: "chntcr_from_channel_file",
            message: format!(
                "expected {} baseline chntcr values, found {}",
                context.nchan,
                context.chntcr_from_channel_file.len()
            ),
        });
    }

    let taumin = output
        .taumin
        .ok_or_else(|| TcrParseError::CrossFileDependencyError {
            field: "taumin",
            message: "missing taumin before override application".to_string(),
        })?;
    let taumax = output
        .taumax
        .ok_or_else(|| TcrParseError::CrossFileDependencyError {
            field: "taumax",
            message: "missing taumax before override application".to_string(),
        })?;
    let kch = output
        .kch
        .ok_or_else(|| TcrParseError::CrossFileDependencyError {
            field: "kch",
            message: "missing kch before override application".to_string(),
        })?;
    let nch = output
        .nch
        .ok_or_else(|| TcrParseError::CrossFileDependencyError {
            field: "nch",
            message: "missing nch before override application".to_string(),
        })?;

    let mut effective = Vec::with_capacity(context.nchan);

    for (idx, ((channel_id, slope), _baseline)) in context
        .channel_element_ids
        .iter()
        .zip(context.chnslp_terminal.iter())
        .zip(context.chntcr_from_channel_file.iter())
        .enumerate()
    {
        if !slope.is_finite() {
            return Err(TcrParseError::CurveDomainError {
                index: idx,
                channel_id: *channel_id,
                slope: *slope,
                denominator: f64::NAN,
                message: "non-finite terminal slope",
            });
        }

        let left = kch.powf(nch);
        let right = slope.powf(nch);
        let denominator = left + right;
        if !denominator.is_finite() || denominator <= 0.0 {
            return Err(TcrParseError::CurveDomainError {
                index: idx,
                channel_id: *channel_id,
                slope: *slope,
                denominator,
                message: "kch^nch + slope^nch must be finite and > 0",
            });
        }

        let numerator = right;
        let ratio = numerator / denominator;
        let override_value = taumin + (taumax - taumin) * ratio;

        if !override_value.is_finite() {
            return Err(TcrParseError::CurveDomainError {
                index: idx,
                channel_id: *channel_id,
                slope: *slope,
                denominator,
                message: "computed chntcr override is non-finite",
            });
        }

        effective.push(override_value);
    }

    output.chntcr_override_applied = true;
    output.effective_chntcr = effective;

    Ok(())
}

fn missing_output(mode: TcrParseMode) -> TcrParseOutput {
    let mut warnings = Vec::new();
    if mode == TcrParseMode::Compatibility {
        warnings.push(TcrWarning::new(
            TcrWarningCode::TcrW001,
            "compatibility optional-sidecar missing branch selected (tcrflg=0)",
        ));
    }

    TcrParseOutput {
        taumin: None,
        taumax: None,
        kch: None,
        nch: None,
        tcr_file_present: false,
        tcrflg: 0,
        parse_outcome: TcrParseOutcome::MissingBranch,
        open_result: TcrOpenResult::Missing,
        line_count_closed: false,
        trailing_token_lines: Vec::new(),
        taumin_taumax_relational_warning_emitted: false,
        chntcr_override_applied: false,
        effective_chntcr: Vec::new(),
        warnings,
    }
}
