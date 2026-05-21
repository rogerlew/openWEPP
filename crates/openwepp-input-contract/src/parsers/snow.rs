#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::too_many_lines
)]

use std::error::Error;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

const DEFAULT_RST: f64 = 0.0;
const DEFAULT_NEWSNW: f64 = 100.0;
const DEFAULT_SSD: f64 = 250.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseMode {
    Strict,
    Compatibility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SnowParseOptions {
    pub mode: ParseMode,
}

impl Default for SnowParseOptions {
    fn default() -> Self {
        Self {
            mode: ParseMode::Strict,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnowWarningCode {
    SnowW001,
    SnowW002,
    SnowW003,
}

impl SnowWarningCode {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SnowW001 => "SNOW-W-001",
            Self::SnowW002 => "SNOW-W-002",
            Self::SnowW003 => "SNOW-W-003",
        }
    }
}

impl fmt::Display for SnowWarningCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnowWarning {
    pub code: SnowWarningCode,
    pub line: usize,
    pub field: &'static str,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SnowParseOutput {
    pub sidecar_present: bool,
    pub defaults_applied: bool,
    pub rst: f64,
    pub newsnw: f64,
    pub ssd: f64,
    pub surplus_record_count: usize,
    pub trailing_token_lines: Vec<usize>,
    pub prefix_variant_detected: bool,
    pub warnings: Vec<SnowWarning>,
}

impl SnowParseOutput {
    fn push_warning(
        &mut self,
        code: SnowWarningCode,
        line: usize,
        field: &'static str,
        message: impl Into<String>,
    ) {
        self.warnings.push(SnowWarning {
            code,
            line,
            field,
            message: message.into(),
        });
    }
}

#[derive(Debug)]
pub enum SnowParseError {
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    TokenParseError {
        line: usize,
        field: &'static str,
        token: String,
    },
    MissingRecordError {
        expected: usize,
        found: usize,
    },
    NonFiniteError {
        line: usize,
        field: &'static str,
    },
    FieldRangeError {
        line: usize,
        field: &'static str,
        value: f64,
    },
    InvariantViolation {
        line: usize,
        context: &'static str,
    },
    StrictSurplusRecordError {
        surplus: usize,
    },
    StrictTrailingTokenError {
        line: usize,
        field: &'static str,
    },
    UnsupportedPrefixVariantError {
        line: usize,
        token: String,
    },
}

impl SnowParseError {
    #[must_use]
    pub fn contract_error_id(&self) -> &'static str {
        match self {
            Self::Io { .. } => "SNOW-E-000",
            Self::TokenParseError { .. } => "SNOW-E-001",
            Self::MissingRecordError { .. } => "SNOW-E-002",
            Self::NonFiniteError { .. } => "SNOW-E-003",
            Self::FieldRangeError { .. } => "SNOW-E-004",
            Self::InvariantViolation { .. } => "SNOW-E-005",
            Self::StrictSurplusRecordError { .. } => "SNOW-E-006",
            Self::StrictTrailingTokenError { .. } => "SNOW-E-007",
            Self::UnsupportedPrefixVariantError { .. } => "SNOW-E-008",
        }
    }
}

impl fmt::Display for SnowParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, source } => write!(
                f,
                "{} failed to read snow sidecar '{}': {source}",
                self.contract_error_id(),
                path.display()
            ),
            Self::TokenParseError { line, field, token } => write!(
                f,
                "{} line {line}: failed to parse '{field}' from token '{token}'",
                self.contract_error_id()
            ),
            Self::MissingRecordError { expected, found } => write!(
                f,
                "{} missing record closure: expected {expected}, found {found}",
                self.contract_error_id()
            ),
            Self::NonFiniteError { line, field } => write!(
                f,
                "{} line {line}: non-finite value in field '{field}'",
                self.contract_error_id()
            ),
            Self::FieldRangeError { line, field, value } => write!(
                f,
                "{} line {line}: value '{value}' violates domain for '{field}'",
                self.contract_error_id()
            ),
            Self::InvariantViolation { line, context } => write!(
                f,
                "{} line {line}: invariant violation: {context}",
                self.contract_error_id()
            ),
            Self::StrictSurplusRecordError { surplus } => write!(
                f,
                "{} strict-mode surplus record count '{surplus}'",
                self.contract_error_id()
            ),
            Self::StrictTrailingTokenError { line, field } => write!(
                f,
                "{} line {line}: strict-mode trailing token in '{field}'",
                self.contract_error_id()
            ),
            Self::UnsupportedPrefixVariantError { line, token } => write!(
                f,
                "{} line {line}: unsupported prefix/version-like leading token '{token}'",
                self.contract_error_id()
            ),
        }
    }
}

impl Error for SnowParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}

pub fn parse_snow_file(
    path: impl AsRef<Path>,
    options: SnowParseOptions,
) -> Result<SnowParseOutput, SnowParseError> {
    let path = path.as_ref();

    match fs::read_to_string(path) {
        Ok(content) => parse_snow_from_str(&content, options),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            let output = default_output(options.mode);
            enforce_invariants(&output, options.mode)?;
            Ok(output)
        }
        Err(source) => Err(SnowParseError::Io {
            path: path.to_path_buf(),
            source,
        }),
    }
}

pub fn parse_snow_from_str(
    input: &str,
    options: SnowParseOptions,
) -> Result<SnowParseOutput, SnowParseError> {
    let lines = materialize_lines(input, options.mode);

    if lines.len() < 3 {
        return Err(SnowParseError::MissingRecordError {
            expected: 3,
            found: lines.len(),
        });
    }

    if detect_prefix_variant(&lines) {
        return Err(SnowParseError::UnsupportedPrefixVariantError {
            line: lines[0].number,
            token: lines[0]
                .text
                .split_whitespace()
                .next()
                .unwrap_or(lines[0].text)
                .to_string(),
        });
    }

    let mut output = SnowParseOutput {
        sidecar_present: true,
        defaults_applied: false,
        rst: 0.0,
        newsnw: 0.0,
        ssd: 0.0,
        surplus_record_count: 0,
        trailing_token_lines: Vec::new(),
        prefix_variant_detected: false,
        warnings: Vec::new(),
    };

    output.rst = parse_canonical_scalar(&lines[0], "rst", options.mode, &mut output)?;
    output.newsnw = parse_canonical_scalar(&lines[1], "newsnw", options.mode, &mut output)?;
    output.ssd = parse_canonical_scalar(&lines[2], "ssd", options.mode, &mut output)?;

    ensure_finite(lines[0].number, "rst", output.rst)?;
    ensure_finite(lines[1].number, "newsnw", output.newsnw)?;
    ensure_finite(lines[2].number, "ssd", output.ssd)?;

    if output.newsnw <= 0.0 {
        return Err(SnowParseError::FieldRangeError {
            line: lines[1].number,
            field: "newsnw",
            value: output.newsnw,
        });
    }
    if output.ssd <= 0.0 {
        return Err(SnowParseError::FieldRangeError {
            line: lines[2].number,
            field: "ssd",
            value: output.ssd,
        });
    }

    if lines.len() > 3 {
        output.surplus_record_count = lines.len() - 3;
        match options.mode {
            ParseMode::Strict => {
                return Err(SnowParseError::StrictSurplusRecordError {
                    surplus: output.surplus_record_count,
                });
            }
            ParseMode::Compatibility => {
                output.push_warning(
                    SnowWarningCode::SnowW003,
                    lines[3].number,
                    "surplus_records",
                    "surplus snow records ignored in compatibility mode",
                );
            }
        }
    }

    enforce_invariants(&output, options.mode)?;

    Ok(output)
}

#[derive(Clone, Copy)]
struct LocatedLine<'a> {
    number: usize,
    text: &'a str,
}

fn materialize_lines(input: &str, mode: ParseMode) -> Vec<LocatedLine<'_>> {
    input
        .lines()
        .enumerate()
        .filter_map(|(idx, raw)| {
            let trimmed = raw.trim();
            if trimmed.is_empty() {
                return None;
            }
            if mode == ParseMode::Compatibility && trimmed.starts_with('#') {
                return None;
            }
            Some(LocatedLine {
                number: idx + 1,
                text: trimmed,
            })
        })
        .collect()
}

fn detect_prefix_variant(lines: &[LocatedLine<'_>]) -> bool {
    if lines.len() < 4 {
        return false;
    }

    let first = lines[0].text;
    let has_alpha = first.chars().any(|c| c.is_ascii_alphabetic());
    if !has_alpha {
        return false;
    }

    let second = parse_first_token_f64(lines[1].text);
    let third = parse_first_token_f64(lines[2].text);
    let fourth = parse_first_token_f64(lines[3].text);
    second.is_some() && third.is_some() && fourth.is_some()
}

fn parse_canonical_scalar(
    line: &LocatedLine<'_>,
    field: &'static str,
    mode: ParseMode,
    output: &mut SnowParseOutput,
) -> Result<f64, SnowParseError> {
    let tokens: Vec<&str> = line.text.split_whitespace().collect();
    if tokens.is_empty() {
        return Err(SnowParseError::TokenParseError {
            line: line.number,
            field,
            token: String::new(),
        });
    }

    if tokens.len() > 1 {
        match mode {
            ParseMode::Strict => {
                return Err(SnowParseError::StrictTrailingTokenError {
                    line: line.number,
                    field,
                });
            }
            ParseMode::Compatibility => {
                output.trailing_token_lines.push(line.number);
                output.push_warning(
                    SnowWarningCode::SnowW002,
                    line.number,
                    field,
                    "trailing tokens accepted in compatibility mode",
                );
            }
        }
    }

    tokens[0]
        .parse::<f64>()
        .map_err(|_| SnowParseError::TokenParseError {
            line: line.number,
            field,
            token: tokens[0].to_string(),
        })
}

fn parse_first_token_f64(line: &str) -> Option<f64> {
    line.split_whitespace().next()?.parse::<f64>().ok()
}

fn ensure_finite(line: usize, field: &'static str, value: f64) -> Result<(), SnowParseError> {
    if value.is_finite() {
        return Ok(());
    }

    Err(SnowParseError::NonFiniteError { line, field })
}

fn default_output(mode: ParseMode) -> SnowParseOutput {
    let mut output = SnowParseOutput {
        sidecar_present: false,
        defaults_applied: true,
        rst: DEFAULT_RST,
        newsnw: DEFAULT_NEWSNW,
        ssd: DEFAULT_SSD,
        surplus_record_count: 0,
        trailing_token_lines: Vec::new(),
        prefix_variant_detected: false,
        warnings: Vec::new(),
    };

    if mode == ParseMode::Compatibility {
        output.push_warning(
            SnowWarningCode::SnowW001,
            0,
            "snow.txt",
            "missing snow sidecar default branch applied",
        );
    }

    output
}

fn enforce_invariants(output: &SnowParseOutput, mode: ParseMode) -> Result<(), SnowParseError> {
    if !output.sidecar_present && !output.defaults_applied {
        return Err(SnowParseError::InvariantViolation {
            line: 0,
            context: "missing-file branch must set defaults_applied=true",
        });
    }
    if output.sidecar_present && output.defaults_applied {
        return Err(SnowParseError::InvariantViolation {
            line: 0,
            context: "present-file branch must set defaults_applied=false",
        });
    }
    if mode == ParseMode::Strict && output.surplus_record_count > 0 {
        return Err(SnowParseError::InvariantViolation {
            line: 0,
            context: "strict mode cannot export surplus_record_count>0",
        });
    }
    if mode == ParseMode::Strict && !output.trailing_token_lines.is_empty() {
        return Err(SnowParseError::InvariantViolation {
            line: 0,
            context: "strict mode cannot export trailing_token_lines",
        });
    }
    if output.prefix_variant_detected {
        return Err(SnowParseError::InvariantViolation {
            line: 0,
            context: "prefix_variant_detected=true must reject parse path",
        });
    }

    Ok(())
}
