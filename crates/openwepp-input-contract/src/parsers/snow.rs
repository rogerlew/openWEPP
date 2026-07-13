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

#[cfg(test)]
mod m03_tests {
    use super::*;

    fn present_output() -> SnowParseOutput {
        SnowParseOutput {
            sidecar_present: true,
            defaults_applied: false,
            rst: DEFAULT_RST,
            newsnw: DEFAULT_NEWSNW,
            ssd: DEFAULT_SSD,
            surplus_record_count: 0,
            trailing_token_lines: Vec::new(),
            prefix_variant_detected: false,
            warnings: Vec::new(),
        }
    }

    #[test]
    fn error_display_ids_and_sources_are_exact() {
        let errors = [
            SnowParseError::Io {
                path: PathBuf::from("snow.txt"),
                source: std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied"),
            },
            SnowParseError::TokenParseError {
                line: 2,
                field: "newsnw",
                token: "bad".to_string(),
            },
            SnowParseError::MissingRecordError {
                expected: 3,
                found: 2,
            },
            SnowParseError::NonFiniteError {
                line: 1,
                field: "rst",
            },
            SnowParseError::FieldRangeError {
                line: 3,
                field: "ssd",
                value: 0.0,
            },
            SnowParseError::InvariantViolation {
                line: 0,
                context: "context",
            },
            SnowParseError::StrictSurplusRecordError { surplus: 2 },
            SnowParseError::StrictTrailingTokenError {
                line: 1,
                field: "rst",
            },
            SnowParseError::UnsupportedPrefixVariantError {
                line: 1,
                token: "v1".to_string(),
            },
        ];
        let expected = [
            "SNOW-E-000 failed to read snow sidecar 'snow.txt': denied",
            "SNOW-E-001 line 2: failed to parse 'newsnw' from token 'bad'",
            "SNOW-E-002 missing record closure: expected 3, found 2",
            "SNOW-E-003 line 1: non-finite value in field 'rst'",
            "SNOW-E-004 line 3: value '0' violates domain for 'ssd'",
            "SNOW-E-005 line 0: invariant violation: context",
            "SNOW-E-006 strict-mode surplus record count '2'",
            "SNOW-E-007 line 1: strict-mode trailing token in 'rst'",
            "SNOW-E-008 line 1: unsupported prefix/version-like leading token 'v1'",
        ];

        for (error, expected) in errors.iter().zip(expected) {
            assert_eq!(error.to_string(), expected);
        }
        assert!(errors[0].source().is_some());
        for error in &errors[1..] {
            assert!(error.source().is_none());
        }
    }

    #[test]
    fn warning_codes_and_messages_are_exact() {
        for (code, expected) in [
            (SnowWarningCode::SnowW001, "SNOW-W-001"),
            (SnowWarningCode::SnowW002, "SNOW-W-002"),
            (SnowWarningCode::SnowW003, "SNOW-W-003"),
        ] {
            assert_eq!(code.as_str(), expected);
            assert_eq!(code.to_string(), expected);
        }

        let missing = default_output(ParseMode::Compatibility);
        assert_eq!(
            missing.warnings[0].message,
            "missing snow sidecar default branch applied"
        );
        let compatible = parse_snow_from_str(
            "0 1\n100\n250\n999\n",
            SnowParseOptions {
                mode: ParseMode::Compatibility,
            },
        )
        .unwrap();
        assert_eq!(
            compatible.warnings[0].message,
            "trailing tokens accepted in compatibility mode"
        );
        assert_eq!(
            compatible.warnings[1].message,
            "surplus snow records ignored in compatibility mode"
        );
    }

    #[test]
    fn invariant_branches_and_combined_invalid_priority_are_exact() {
        type InvariantCase = (fn(&mut SnowParseOutput), ParseMode, &'static str);

        let valid = present_output();
        assert!(enforce_invariants(&valid, ParseMode::Strict).is_ok());
        let cases: [InvariantCase; 5] = [
            (
                |row| {
                    row.sidecar_present = false;
                    row.defaults_applied = false;
                    row.prefix_variant_detected = true;
                },
                ParseMode::Strict,
                "missing-file branch must set defaults_applied=true",
            ),
            (
                |row| row.defaults_applied = true,
                ParseMode::Strict,
                "present-file branch must set defaults_applied=false",
            ),
            (
                |row| row.surplus_record_count = 1,
                ParseMode::Strict,
                "strict mode cannot export surplus_record_count>0",
            ),
            (
                |row| row.trailing_token_lines.push(1),
                ParseMode::Strict,
                "strict mode cannot export trailing_token_lines",
            ),
            (
                |row| row.prefix_variant_detected = true,
                ParseMode::Compatibility,
                "prefix_variant_detected=true must reject parse path",
            ),
        ];
        for (mutate, mode, expected) in cases {
            let mut row = valid.clone();
            mutate(&mut row);
            assert!(matches!(
                enforce_invariants(&row, mode),
                Err(SnowParseError::InvariantViolation { line: 0, context }) if context == expected
            ));
        }
    }

    #[test]
    fn finite_and_range_error_order_is_exact() {
        for (input, expected_line, expected_field) in [
            ("NaN\nNaN\nNaN\n", 1, "rst"),
            ("0\nNaN\nNaN\n", 2, "newsnw"),
            ("0\n100\nNaN\n", 3, "ssd"),
        ] {
            assert!(matches!(
                parse_snow_from_str(input, SnowParseOptions::default()),
                Err(SnowParseError::NonFiniteError { line, field })
                    if line == expected_line && field == expected_field
            ));
        }
        for (input, expected_line, expected_field) in
            [("0\n0\n0\n", 2, "newsnw"), ("0\n100\n0\n", 3, "ssd")]
        {
            assert!(matches!(
                parse_snow_from_str(input, SnowParseOptions::default()),
                Err(SnowParseError::FieldRangeError { line, field, .. })
                    if line == expected_line && field == expected_field
            ));
        }
    }

    #[test]
    fn private_token_and_prefix_boundaries_are_characterized() {
        let line = |number, text| LocatedLine { number, text };
        assert!(!detect_prefix_variant(&[
            line(1, "header"),
            line(2, "1"),
            line(3, "2"),
        ]));
        assert!(!detect_prefix_variant(&[
            line(1, "0"),
            line(2, "1"),
            line(3, "2"),
            line(4, "3"),
        ]));
        assert!(detect_prefix_variant(&[
            line(1, "version"),
            line(2, "1"),
            line(3, "2"),
            line(4, "3"),
        ]));
        assert!(!detect_prefix_variant(&[
            line(1, "version"),
            line(2, "bad"),
            line(3, "2"),
            line(4, "3"),
        ]));

        assert_eq!(parse_first_token_f64("1 trailing"), Some(1.0));
        assert_eq!(parse_first_token_f64("bad"), None);
        assert_eq!(parse_first_token_f64(""), None);

        let mut output = present_output();
        assert!(matches!(
            parse_canonical_scalar(
                &line(7, "1 trailing"),
                "rst",
                ParseMode::Strict,
                &mut output,
            ),
            Err(SnowParseError::StrictTrailingTokenError {
                line: 7,
                field: "rst"
            })
        ));
        assert!(matches!(
            parse_canonical_scalar(&line(8, "bad"), "rst", ParseMode::Strict, &mut output,),
            Err(SnowParseError::TokenParseError {
                line: 8,
                field: "rst",
                ..
            })
        ));
    }
}
