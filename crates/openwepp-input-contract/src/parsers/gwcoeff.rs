#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools
)]

use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseMode {
    Strict,
    Compatibility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamespaceBinding {
    Distinct,
    ConflatedWithChaninp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GwcoeffCrossFileContext {
    pub namespace_binding: NamespaceBinding,
}

impl Default for GwcoeffCrossFileContext {
    fn default() -> Self {
        Self {
            namespace_binding: NamespaceBinding::Distinct,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GwcoeffParseOptions {
    pub mode: ParseMode,
    pub cross_file: GwcoeffCrossFileContext,
}

impl GwcoeffParseOptions {
    #[must_use]
    pub const fn strict() -> Self {
        Self {
            mode: ParseMode::Strict,
            cross_file: GwcoeffCrossFileContext {
                namespace_binding: NamespaceBinding::Distinct,
            },
        }
    }

    #[must_use]
    pub const fn compatibility() -> Self {
        Self {
            mode: ParseMode::Compatibility,
            cross_file: GwcoeffCrossFileContext {
                namespace_binding: NamespaceBinding::Distinct,
            },
        }
    }
}

impl Default for GwcoeffParseOptions {
    fn default() -> Self {
        Self::strict()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GwcoeffParseOutcome {
    MissingBranch,
    ParsedBranch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GwcoeffOpenResult {
    Missing,
    OpenSuccess,
    OpenErrorCollapsedCompat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GwcoeffWarningCode {
    GwW001,
}

impl GwcoeffWarningCode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GwW001 => "GW-W-001",
        }
    }
}

impl fmt::Display for GwcoeffWarningCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GwcoeffWarning {
    pub code: GwcoeffWarningCode,
    pub line: Option<usize>,
    pub message: String,
}

impl GwcoeffWarning {
    fn new(code: GwcoeffWarningCode, line: Option<usize>, message: impl Into<String>) -> Self {
        Self {
            code,
            line,
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GwcoeffFile {
    pub igwstrd: Option<f64>,
    pub bfcoeff: Option<f64>,
    pub dscoeff: Option<f64>,
    pub bftharea: Option<f64>,
    pub gwcoeff_file_present: bool,
    pub lr_bf: i32,
    pub parse_outcome: GwcoeffParseOutcome,
    pub open_result: GwcoeffOpenResult,
    pub line_count_closed: bool,
    pub trailing_token_lines: Vec<usize>,
    pub warnings: Vec<GwcoeffWarning>,
}

#[derive(Debug)]
pub enum GwcoeffParseError {
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
    FieldFiniteError {
        line: usize,
        field: &'static str,
        value: String,
    },
    FieldRangeError {
        line: usize,
        field: &'static str,
        value: f64,
        rule: &'static str,
    },
    CoefficientNamespaceConflation {
        detail: &'static str,
    },
    InvariantViolation {
        detail: &'static str,
    },
    UnsupportedPrefixedVariant {
        line: usize,
        token: String,
    },
}

impl GwcoeffParseError {
    #[must_use]
    pub const fn contract_error_id(&self) -> &'static str {
        match self {
            Self::InputOpenError { .. } => "GW-E-000",
            Self::TokenParseError { .. } => "GW-E-001",
            Self::RecordCountError { .. } => "GW-E-002",
            Self::FieldFiniteError { .. } => "GW-E-003",
            Self::FieldRangeError { .. } => "GW-E-004",
            Self::CoefficientNamespaceConflation { .. } => "GW-E-005",
            Self::InvariantViolation { .. } => "GW-E-006",
            Self::UnsupportedPrefixedVariant { .. } => "GW-E-007",
        }
    }
}

impl fmt::Display for GwcoeffParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputOpenError { path, source } => write!(
                f,
                "{}: could not open/read gwcoeff sidecar '{}': {source}",
                self.contract_error_id(),
                path.display()
            ),
            Self::TokenParseError { line, field, token } => write!(
                f,
                "{}: line {line} failed to parse field '{field}' from token '{token}'",
                self.contract_error_id()
            ),
            Self::RecordCountError { expected, found } => write!(
                f,
                "{}: expected {expected} non-empty records, found {found}",
                self.contract_error_id()
            ),
            Self::FieldFiniteError { line, field, value } => write!(
                f,
                "{}: line {line} field '{field}' is non-finite ('{value}')",
                self.contract_error_id()
            ),
            Self::FieldRangeError {
                line,
                field,
                value,
                rule,
            } => write!(
                f,
                "{}: line {line} field '{field}' value {value} violates rule {rule}",
                self.contract_error_id()
            ),
            Self::CoefficientNamespaceConflation { detail } => write!(
                f,
                "{}: coefficient namespace conflation detected ({detail})",
                self.contract_error_id()
            ),
            Self::InvariantViolation { detail } => write!(
                f,
                "{}: invariant violation ({detail})",
                self.contract_error_id()
            ),
            Self::UnsupportedPrefixedVariant { line, token } => write!(
                f,
                "{}: line {line} unsupported prefixed/datver-like header token '{token}'",
                self.contract_error_id()
            ),
        }
    }
}

impl std::error::Error for GwcoeffParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InputOpenError { source, .. } => Some(source),
            _ => None,
        }
    }
}

pub fn parse_gwcoeff_from_path(
    path: impl AsRef<Path>,
    options: GwcoeffParseOptions,
) -> Result<GwcoeffFile, GwcoeffParseError> {
    enforce_namespace_guard(options.cross_file)?;

    let path = path.as_ref();
    match fs::read_to_string(path) {
        Ok(content) => parse_present_content(&content, options),
        Err(source) if source.kind() == io::ErrorKind::NotFound => {
            let mut result = build_missing_branch(GwcoeffOpenResult::Missing);
            if options.mode == ParseMode::Compatibility {
                result.warnings.push(GwcoeffWarning::new(
                    GwcoeffWarningCode::GwW001,
                    None,
                    "compatibility optional-absence branch taken (lr_bf=0)",
                ));
            }
            enforce_result_invariants(&result)?;
            Ok(result)
        }
        Err(source) => {
            if options.mode == ParseMode::Strict {
                return Err(GwcoeffParseError::InputOpenError {
                    path: path.to_path_buf(),
                    source,
                });
            }

            let mut result = build_missing_branch(GwcoeffOpenResult::OpenErrorCollapsedCompat);
            result.warnings.push(GwcoeffWarning::new(
                GwcoeffWarningCode::GwW001,
                None,
                format!(
                    "compatibility collapsed non-ENOENT open error into missing branch ({source})"
                ),
            ));
            enforce_result_invariants(&result)?;
            Ok(result)
        }
    }
}

pub fn parse_gwcoeff_from_str(
    input: &str,
    options: GwcoeffParseOptions,
) -> Result<GwcoeffFile, GwcoeffParseError> {
    enforce_namespace_guard(options.cross_file)?;
    parse_present_content(input, options)
}

fn parse_present_content(
    input: &str,
    options: GwcoeffParseOptions,
) -> Result<GwcoeffFile, GwcoeffParseError> {
    let lines: Vec<(usize, &str)> = input
        .lines()
        .enumerate()
        .filter_map(|(idx, raw)| {
            let trimmed = raw.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some((idx + 1, trimmed))
            }
        })
        .collect();

    if prefixed_variant_detected(&lines) {
        let token = lines
            .first()
            .and_then(|(_, line)| line.split_whitespace().next())
            .unwrap_or_default()
            .to_string();
        return Err(GwcoeffParseError::UnsupportedPrefixedVariant { line: 1, token });
    }

    if lines.len() != 4 {
        return Err(GwcoeffParseError::RecordCountError {
            expected: 4,
            found: lines.len(),
        });
    }

    let mut trailing_token_lines = Vec::new();
    let igwstrd = parse_record(lines[0], "igwstrd", &mut trailing_token_lines)?;
    let bfcoeff = parse_record(lines[1], "bfcoeff", &mut trailing_token_lines)?;
    let dscoeff = parse_record(lines[2], "dscoeff", &mut trailing_token_lines)?;
    let bftharea = parse_record(lines[3], "bftharea", &mut trailing_token_lines)?;

    ensure_non_negative(lines[0].0, "igwstrd", igwstrd)?;
    ensure_non_negative(lines[1].0, "bfcoeff", bfcoeff)?;
    ensure_non_negative(lines[2].0, "dscoeff", dscoeff)?;
    ensure_non_negative(lines[3].0, "bftharea", bftharea)?;

    let result = GwcoeffFile {
        igwstrd: Some(igwstrd),
        bfcoeff: Some(bfcoeff),
        dscoeff: Some(dscoeff),
        bftharea: Some(bftharea),
        gwcoeff_file_present: true,
        lr_bf: 1,
        parse_outcome: GwcoeffParseOutcome::ParsedBranch,
        open_result: GwcoeffOpenResult::OpenSuccess,
        line_count_closed: true,
        trailing_token_lines,
        warnings: Vec::new(),
    };

    if options.mode == ParseMode::Strict || options.mode == ParseMode::Compatibility {
        enforce_result_invariants(&result)?;
    }

    Ok(result)
}

fn parse_record(
    line: (usize, &str),
    field: &'static str,
    trailing_token_lines: &mut Vec<usize>,
) -> Result<f64, GwcoeffParseError> {
    let (line_no, content) = line;
    let tokens: Vec<&str> = content.split_whitespace().collect();
    let Some(first) = tokens.first() else {
        return Err(GwcoeffParseError::TokenParseError {
            line: line_no,
            field,
            token: String::new(),
        });
    };

    if tokens.len() > 1 {
        trailing_token_lines.push(line_no);
    }

    let value = first
        .parse::<f64>()
        .map_err(|_| GwcoeffParseError::TokenParseError {
            line: line_no,
            field,
            token: (*first).to_string(),
        })?;

    if !value.is_finite() {
        return Err(GwcoeffParseError::FieldFiniteError {
            line: line_no,
            field,
            value: (*first).to_string(),
        });
    }

    Ok(value)
}

fn ensure_non_negative(
    line: usize,
    field: &'static str,
    value: f64,
) -> Result<(), GwcoeffParseError> {
    if value >= 0.0 {
        return Ok(());
    }

    Err(GwcoeffParseError::FieldRangeError {
        line,
        field,
        value,
        rule: ">= 0",
    })
}

fn enforce_namespace_guard(context: GwcoeffCrossFileContext) -> Result<(), GwcoeffParseError> {
    if context.namespace_binding == NamespaceBinding::ConflatedWithChaninp {
        return Err(GwcoeffParseError::CoefficientNamespaceConflation {
            detail: "gwcoeff.bfcoeff and chaninp.cbase namespaces must remain distinct",
        });
    }
    Ok(())
}

fn enforce_result_invariants(result: &GwcoeffFile) -> Result<(), GwcoeffParseError> {
    match result.parse_outcome {
        GwcoeffParseOutcome::MissingBranch => validate_missing_branch(result),
        GwcoeffParseOutcome::ParsedBranch => validate_parsed_branch(result),
    }
}

fn validate_missing_branch(result: &GwcoeffFile) -> Result<(), GwcoeffParseError> {
    if result.lr_bf != 0 {
        return Err(GwcoeffParseError::InvariantViolation {
            detail: "missing_branch requires lr_bf=0",
        });
    }
    if result.gwcoeff_file_present {
        return Err(GwcoeffParseError::InvariantViolation {
            detail: "missing_branch requires gwcoeff_file_present=false",
        });
    }
    if result.igwstrd.is_some()
        || result.bfcoeff.is_some()
        || result.dscoeff.is_some()
        || result.bftharea.is_some()
    {
        return Err(GwcoeffParseError::InvariantViolation {
            detail: "missing_branch forbids implicit coefficient defaults",
        });
    }
    Ok(())
}

fn validate_parsed_branch(result: &GwcoeffFile) -> Result<(), GwcoeffParseError> {
    if result.lr_bf != 1 {
        return Err(GwcoeffParseError::InvariantViolation {
            detail: "parsed_branch requires lr_bf=1",
        });
    }
    if !result.gwcoeff_file_present {
        return Err(GwcoeffParseError::InvariantViolation {
            detail: "parsed_branch requires gwcoeff_file_present=true",
        });
    }
    if result.igwstrd.is_none()
        || result.bfcoeff.is_none()
        || result.dscoeff.is_none()
        || result.bftharea.is_none()
    {
        return Err(GwcoeffParseError::InvariantViolation {
            detail: "parsed_branch requires all 4 coefficient fields",
        });
    }
    if !result.line_count_closed {
        return Err(GwcoeffParseError::InvariantViolation {
            detail: "parsed_branch requires line_count_closed=true",
        });
    }
    Ok(())
}

fn build_missing_branch(open_result: GwcoeffOpenResult) -> GwcoeffFile {
    GwcoeffFile {
        igwstrd: None,
        bfcoeff: None,
        dscoeff: None,
        bftharea: None,
        gwcoeff_file_present: false,
        lr_bf: 0,
        parse_outcome: GwcoeffParseOutcome::MissingBranch,
        open_result,
        line_count_closed: false,
        trailing_token_lines: Vec::new(),
        warnings: Vec::new(),
    }
}

fn prefixed_variant_detected(lines: &[(usize, &str)]) -> bool {
    if lines.len() < 5 {
        return false;
    }

    let has_alpha_leading = lines[0].1.chars().any(|ch| ch.is_ascii_alphabetic());
    if !has_alpha_leading {
        return false;
    }

    lines[1..5].iter().all(|(_, line)| {
        line.split_whitespace()
            .next()
            .is_some_and(|token| token.parse::<f64>().is_ok())
    })
}

#[cfg(test)]
mod m02_tests {
    use super::*;

    #[test]
    fn invariant_error_priority_is_characterized() {
        let missing = build_missing_branch(GwcoeffOpenResult::Missing);
        assert!(enforce_result_invariants(&missing).is_ok());
        for (mutate, expected_detail) in [
            (
                (|row: &mut GwcoeffFile| row.lr_bf = 1) as fn(&mut GwcoeffFile),
                "missing_branch requires lr_bf=0",
            ),
            (
                |row: &mut GwcoeffFile| row.gwcoeff_file_present = true,
                "missing_branch requires gwcoeff_file_present=false",
            ),
            (
                |row: &mut GwcoeffFile| row.bfcoeff = Some(0.1),
                "missing_branch forbids implicit coefficient defaults",
            ),
        ] {
            let mut row = missing.clone();
            mutate(&mut row);
            assert!(matches!(
                enforce_result_invariants(&row),
                Err(GwcoeffParseError::InvariantViolation { detail }) if detail == expected_detail
            ));
        }

        let mut missing_priority = missing.clone();
        missing_priority.lr_bf = 1;
        missing_priority.gwcoeff_file_present = true;
        missing_priority.bfcoeff = Some(0.1);
        assert_invariant_detail(&missing_priority, "missing_branch requires lr_bf=0");
        missing_priority.lr_bf = 0;
        assert_invariant_detail(
            &missing_priority,
            "missing_branch requires gwcoeff_file_present=false",
        );

        let parsed =
            parse_present_content("200\n0.04\n0\n1\n", GwcoeffParseOptions::strict()).unwrap();
        assert!(enforce_result_invariants(&parsed).is_ok());
        for (mutate, expected_detail) in [
            (
                (|row: &mut GwcoeffFile| row.lr_bf = 0) as fn(&mut GwcoeffFile),
                "parsed_branch requires lr_bf=1",
            ),
            (
                |row: &mut GwcoeffFile| row.gwcoeff_file_present = false,
                "parsed_branch requires gwcoeff_file_present=true",
            ),
            (
                |row: &mut GwcoeffFile| row.bftharea = None,
                "parsed_branch requires all 4 coefficient fields",
            ),
            (
                |row: &mut GwcoeffFile| row.line_count_closed = false,
                "parsed_branch requires line_count_closed=true",
            ),
        ] {
            let mut row = parsed.clone();
            mutate(&mut row);
            assert!(matches!(
                enforce_result_invariants(&row),
                Err(GwcoeffParseError::InvariantViolation { detail }) if detail == expected_detail
            ));
        }

        let mut parsed_priority = parsed.clone();
        parsed_priority.lr_bf = 0;
        parsed_priority.gwcoeff_file_present = false;
        parsed_priority.bftharea = None;
        parsed_priority.line_count_closed = false;
        assert_invariant_detail(&parsed_priority, "parsed_branch requires lr_bf=1");
        parsed_priority.lr_bf = 1;
        assert_invariant_detail(
            &parsed_priority,
            "parsed_branch requires gwcoeff_file_present=true",
        );
        parsed_priority.gwcoeff_file_present = true;
        assert_invariant_detail(
            &parsed_priority,
            "parsed_branch requires all 4 coefficient fields",
        );
        parsed_priority.bftharea = Some(1.0);
        assert_invariant_detail(
            &parsed_priority,
            "parsed_branch requires line_count_closed=true",
        );
    }

    fn assert_invariant_detail(row: &GwcoeffFile, expected: &'static str) {
        assert!(matches!(
            enforce_result_invariants(row),
            Err(GwcoeffParseError::InvariantViolation { detail }) if detail == expected
        ));
    }
}
