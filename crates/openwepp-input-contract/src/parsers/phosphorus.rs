#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::too_many_lines
)]

use std::error::Error;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

const STRICT_HEADER_LITERAL: &str = "Phosphorus concentration";
const TOTAL_RECORD_COUNT: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseMode {
    Strict,
    Compatibility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhosphorusParseOptions {
    pub mode: ParseMode,
    pub require_sidecar: bool,
    pub expected_hillslope_count: Option<usize>,
}

impl Default for PhosphorusParseOptions {
    fn default() -> Self {
        Self {
            mode: ParseMode::Strict,
            require_sidecar: false,
            expected_hillslope_count: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhosphorusWarningCode {
    PhosW001,
    PhosW002,
}

impl PhosphorusWarningCode {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PhosW001 => "PHOS-W-001",
            Self::PhosW002 => "PHOS-W-002",
        }
    }
}

impl fmt::Display for PhosphorusWarningCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhosphorusWarning {
    pub code: PhosphorusWarningCode,
    pub line: usize,
    pub field: &'static str,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PhosphorusFile {
    pub sidecar_present: bool,
    pub p_flag: i32,
    pub header_text: Option<String>,
    pub header_literal_match: bool,
    pub line_count_closed: bool,
    pub trailing_token_lines: Vec<usize>,
    pub srp_mg_l: Option<f64>,
    pub slfp_mg_l: Option<f64>,
    pub bfp_mg_l: Option<f64>,
    pub scp_mg_kg: Option<f64>,
    pub tmpsrp_mg_l: Vec<f64>,
    pub tmpslfp_mg_l: Vec<f64>,
    pub tmpbfp_mg_l: Vec<f64>,
    pub tmpscp_mg_kg: Vec<f64>,
    pub warnings: Vec<PhosphorusWarning>,
}

#[derive(Debug)]
pub enum PhosphorusParseError {
    InputOpenError {
        path: PathBuf,
        source: std::io::Error,
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
    FieldFiniteError {
        line: usize,
        field: &'static str,
        value: f64,
    },
    FieldRangeError {
        line: usize,
        field: &'static str,
        value: f64,
        expected: &'static str,
    },
    FanoutMismatch {
        field: &'static str,
        expected_count: usize,
        observed_count: usize,
    },
    InvariantViolation {
        context: &'static str,
    },
    HeaderLiteralMismatch {
        line: usize,
        expected: &'static str,
        observed: String,
    },
}

impl PhosphorusParseError {
    #[must_use]
    pub fn contract_error_id(&self) -> &'static str {
        match self {
            Self::InputOpenError { .. } => "PHOS-E-000",
            Self::TokenParseError { .. } => "PHOS-E-001",
            Self::RecordCountError { .. } => "PHOS-E-002",
            Self::FieldFiniteError { .. } => "PHOS-E-003",
            Self::FieldRangeError { .. } => "PHOS-E-004",
            Self::FanoutMismatch { .. } => "PHOS-E-005",
            Self::InvariantViolation { .. } => "PHOS-E-006",
            Self::HeaderLiteralMismatch { .. } => "PHOS-E-007",
        }
    }
}

impl fmt::Display for PhosphorusParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputOpenError { path, source } => write!(
                f,
                "{} failed to read phosphorus sidecar '{}': {source}",
                self.contract_error_id(),
                path.display()
            ),
            Self::TokenParseError { line, field, token } => write!(
                f,
                "{} line {line}: failed to parse field '{field}' from token '{token}'",
                self.contract_error_id()
            ),
            Self::RecordCountError { expected, found } => write!(
                f,
                "{} record-count mismatch: expected={expected}, found={found}",
                self.contract_error_id()
            ),
            Self::FieldFiniteError { line, field, value } => write!(
                f,
                "{} line {line}: field '{field}' must be finite, got {value}",
                self.contract_error_id()
            ),
            Self::FieldRangeError {
                line,
                field,
                value,
                expected,
            } => write!(
                f,
                "{} line {line}: field '{field}' value {value} violates domain {expected}",
                self.contract_error_id()
            ),
            Self::FanoutMismatch {
                field,
                expected_count,
                observed_count,
            } => write!(
                f,
                "{} fanout mismatch for '{field}': expected {expected_count}, observed {observed_count}",
                self.contract_error_id()
            ),
            Self::InvariantViolation { context } => {
                write!(
                    f,
                    "{} invariant violation: {context}",
                    self.contract_error_id()
                )
            }
            Self::HeaderLiteralMismatch {
                line,
                expected,
                observed,
            } => write!(
                f,
                "{} line {line}: header mismatch; expected '{expected}', observed '{observed}'",
                self.contract_error_id()
            ),
        }
    }
}

impl Error for PhosphorusParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InputOpenError { source, .. } => Some(source),
            _ => None,
        }
    }
}

pub fn parse_phosphorus_file(
    path: impl AsRef<Path>,
    options: PhosphorusParseOptions,
) -> Result<PhosphorusFile, PhosphorusParseError> {
    let path = path.as_ref();
    match fs::read_to_string(path) {
        Ok(content) => parse_phosphorus_from_str(&content, options),
        Err(source)
            if source.kind() == std::io::ErrorKind::NotFound && !options.require_sidecar =>
        {
            Ok(absent_sidecar_output(options.mode))
        }
        Err(source) => Err(PhosphorusParseError::InputOpenError {
            path: path.to_path_buf(),
            source,
        }),
    }
}

pub fn parse_phosphorus_from_str(
    input: &str,
    options: PhosphorusParseOptions,
) -> Result<PhosphorusFile, PhosphorusParseError> {
    let mut lines = Vec::new();
    for (line_idx, raw) in input.lines().enumerate() {
        let trimmed = raw.trim();
        if !trimmed.is_empty() {
            lines.push((line_idx + 1, trimmed.to_string()));
        }
    }

    if lines.len() != TOTAL_RECORD_COUNT {
        return Err(PhosphorusParseError::RecordCountError {
            expected: TOTAL_RECORD_COUNT,
            found: lines.len(),
        });
    }

    let (header_line_no, header_text) = &lines[0];
    let header_literal_match = header_text == STRICT_HEADER_LITERAL;
    let mut warnings = Vec::new();

    if !header_literal_match {
        if options.mode == ParseMode::Strict {
            return Err(PhosphorusParseError::HeaderLiteralMismatch {
                line: *header_line_no,
                expected: STRICT_HEADER_LITERAL,
                observed: header_text.clone(),
            });
        }
        warnings.push(PhosphorusWarning {
            code: PhosphorusWarningCode::PhosW002,
            line: *header_line_no,
            field: "header_text",
            message: "non-canonical header accepted in compatibility mode".to_string(),
        });
    }

    let mut trailing_token_lines = Vec::new();
    let srp_mg_l = parse_concentration_line(&lines[1], "srp", &mut trailing_token_lines)?;
    let slfp_mg_l = parse_concentration_line(&lines[2], "slfp", &mut trailing_token_lines)?;
    let bfp_mg_l = parse_concentration_line(&lines[3], "bfp", &mut trailing_token_lines)?;
    let scp_mg_kg = parse_concentration_line(&lines[4], "scp", &mut trailing_token_lines)?;

    let (tmpsrp_mg_l, tmpslfp_mg_l, tmpbfp_mg_l, tmpscp_mg_kg) =
        match options.expected_hillslope_count {
            Some(count) => (
                vec![srp_mg_l; count],
                vec![slfp_mg_l; count],
                vec![bfp_mg_l; count],
                vec![scp_mg_kg; count],
            ),
            None => (Vec::new(), Vec::new(), Vec::new(), Vec::new()),
        };

    verify_fanout(
        "tmpsrp",
        options.expected_hillslope_count,
        tmpsrp_mg_l.len(),
    )?;
    verify_fanout(
        "tmpslfp",
        options.expected_hillslope_count,
        tmpslfp_mg_l.len(),
    )?;
    verify_fanout(
        "tmpbfp",
        options.expected_hillslope_count,
        tmpbfp_mg_l.len(),
    )?;
    verify_fanout(
        "tmpscp",
        options.expected_hillslope_count,
        tmpscp_mg_kg.len(),
    )?;

    let parsed = PhosphorusFile {
        sidecar_present: true,
        p_flag: 1,
        header_text: Some(header_text.clone()),
        header_literal_match,
        line_count_closed: true,
        trailing_token_lines,
        srp_mg_l: Some(srp_mg_l),
        slfp_mg_l: Some(slfp_mg_l),
        bfp_mg_l: Some(bfp_mg_l),
        scp_mg_kg: Some(scp_mg_kg),
        tmpsrp_mg_l,
        tmpslfp_mg_l,
        tmpbfp_mg_l,
        tmpscp_mg_kg,
        warnings,
    };

    validate_state_closure(&parsed)?;
    Ok(parsed)
}

fn parse_concentration_line(
    line: &(usize, String),
    field: &'static str,
    trailing_token_lines: &mut Vec<usize>,
) -> Result<f64, PhosphorusParseError> {
    let (line_no, text) = line;
    let tokens: Vec<&str> = text.split_whitespace().collect();
    let first = tokens
        .first()
        .ok_or_else(|| PhosphorusParseError::TokenParseError {
            line: *line_no,
            field,
            token: String::new(),
        })?;
    if tokens.len() > 1 {
        trailing_token_lines.push(*line_no);
    }
    let value = first
        .parse::<f64>()
        .map_err(|_| PhosphorusParseError::TokenParseError {
            line: *line_no,
            field,
            token: (*first).to_string(),
        })?;

    if !value.is_finite() {
        return Err(PhosphorusParseError::FieldFiniteError {
            line: *line_no,
            field,
            value,
        });
    }
    if value < 0.0 {
        return Err(PhosphorusParseError::FieldRangeError {
            line: *line_no,
            field,
            value,
            expected: ">= 0.0",
        });
    }
    Ok(value)
}

fn verify_fanout(
    field: &'static str,
    expected_hillslope_count: Option<usize>,
    observed_count: usize,
) -> Result<(), PhosphorusParseError> {
    if let Some(expected_count) = expected_hillslope_count {
        if observed_count != expected_count {
            return Err(PhosphorusParseError::FanoutMismatch {
                field,
                expected_count,
                observed_count,
            });
        }
    }
    Ok(())
}

fn validate_state_closure(parsed: &PhosphorusFile) -> Result<(), PhosphorusParseError> {
    if parsed.sidecar_present && parsed.p_flag != 1 {
        return Err(PhosphorusParseError::InvariantViolation {
            context: "sidecar_present=true requires p_flag=1",
        });
    }
    if !parsed.sidecar_present && parsed.p_flag != 0 {
        return Err(PhosphorusParseError::InvariantViolation {
            context: "sidecar_present=false requires p_flag=0",
        });
    }
    if parsed.sidecar_present
        && (parsed.srp_mg_l.is_none()
            || parsed.slfp_mg_l.is_none()
            || parsed.bfp_mg_l.is_none()
            || parsed.scp_mg_kg.is_none())
    {
        return Err(PhosphorusParseError::InvariantViolation {
            context: "enabled phosphorus branch requires all scalar concentrations",
        });
    }
    Ok(())
}

fn absent_sidecar_output(mode: ParseMode) -> PhosphorusFile {
    let mut warnings = Vec::new();
    if mode == ParseMode::Compatibility {
        warnings.push(PhosphorusWarning {
            code: PhosphorusWarningCode::PhosW001,
            line: 0,
            field: "phosphorus_file_present",
            message: "optional phosphorus sidecar absent; p_flag defaults to 0".to_string(),
        });
    }

    PhosphorusFile {
        sidecar_present: false,
        p_flag: 0,
        header_text: None,
        header_literal_match: false,
        line_count_closed: true,
        trailing_token_lines: Vec::new(),
        srp_mg_l: None,
        slfp_mg_l: None,
        bfp_mg_l: None,
        scp_mg_kg: None,
        tmpsrp_mg_l: Vec::new(),
        tmpslfp_mg_l: Vec::new(),
        tmpbfp_mg_l: Vec::new(),
        tmpscp_mg_kg: Vec::new(),
        warnings,
    }
}
