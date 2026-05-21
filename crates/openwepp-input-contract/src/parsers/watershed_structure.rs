#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

use std::error::Error;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

const STR_DATVER_MIN: f64 = 94.301;
const STR_DATVER_EPS: f64 = 1e-6;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseMode {
    Strict,
    Compatibility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatverSource {
    ExplicitHeader,
    LegacyCompatNoDatver,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WatershedStructureParseOptions {
    pub mode: ParseMode,
    pub nhill: usize,
    pub expected_rows: Option<usize>,
    pub expected_channel_count: Option<usize>,
    pub expected_impoundment_count: Option<usize>,
}

impl WatershedStructureParseOptions {
    #[must_use]
    pub const fn strict(nhill: usize, expected_rows: usize) -> Self {
        Self {
            mode: ParseMode::Strict,
            nhill,
            expected_rows: Some(expected_rows),
            expected_channel_count: None,
            expected_impoundment_count: None,
        }
    }

    #[must_use]
    pub const fn compatibility(nhill: usize, expected_rows: usize) -> Self {
        Self {
            mode: ParseMode::Compatibility,
            nhill,
            expected_rows: Some(expected_rows),
            expected_channel_count: None,
            expected_impoundment_count: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatershedStructureWarningCode {
    StrW001,
}

impl WatershedStructureWarningCode {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::StrW001 => "STR-W-001",
        }
    }
}

impl fmt::Display for WatershedStructureWarningCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WatershedStructureWarning {
    pub code: WatershedStructureWarningCode,
    pub line: usize,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WatershedStructureRow {
    pub record_index: usize,
    pub element_id: i32,
    pub element_type_code: i32,
    pub element_local_index: usize,
    pub nhleft: i32,
    pub nhrght: i32,
    pub nhtop: i32,
    pub ncleft: i32,
    pub ncrght: i32,
    pub nctop: i32,
    pub nileft: i32,
    pub nirght: i32,
    pub nitop: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WatershedStructureSummary {
    pub channel_count: usize,
    pub impoundment_count: usize,
    pub max_hillslope_ref: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WatershedStructureFile {
    pub datver: f64,
    pub datver_source: DatverSource,
    pub nhill: usize,
    pub rows: Vec<WatershedStructureRow>,
    pub summary: WatershedStructureSummary,
    pub warnings: Vec<WatershedStructureWarning>,
}

impl WatershedStructureFile {
    fn push_warning(
        &mut self,
        code: WatershedStructureWarningCode,
        line: usize,
        message: impl Into<String>,
    ) {
        self.warnings.push(WatershedStructureWarning {
            code,
            line,
            message: message.into(),
        });
    }
}

#[derive(Debug)]
pub enum WatershedStructureParseError {
    InputOpenError {
        path: PathBuf,
        source: std::io::Error,
    },
    TokenParseError {
        line: usize,
        field: &'static str,
        token: String,
    },
    RecordArityError {
        line: usize,
        expected: usize,
        found: usize,
    },
    LegacyNoDatverDisallowed {
        line: usize,
        token: String,
    },
    UnsupportedDatver {
        line: usize,
        datver: f64,
        min_supported: f64,
    },
    ElementTypeDomainError {
        line: usize,
        value: i32,
    },
    DisconnectedElementError {
        line: usize,
        record_index: usize,
    },
    ContributorDomainError {
        line: usize,
        field: &'static str,
        value: i32,
        expected: &'static str,
    },
    ChannelCountMismatch {
        expected: usize,
        observed: usize,
    },
    ImpoundmentCountMismatch {
        expected: usize,
        observed: usize,
    },
    HillslopeCoverageMismatch {
        expected_nhill: usize,
        observed_nhmax: usize,
    },
    NhillContextError {
        nhill: usize,
    },
    RecordCountMismatch {
        expected: usize,
        observed: usize,
    },
    InvariantViolation {
        context: &'static str,
    },
}

impl WatershedStructureParseError {
    #[must_use]
    pub fn contract_error_id(&self) -> &'static str {
        match self {
            Self::InputOpenError { .. } => "STR-E-000",
            Self::TokenParseError { .. } => "STR-E-001",
            Self::RecordArityError { .. } => "STR-E-002",
            Self::LegacyNoDatverDisallowed { .. } | Self::UnsupportedDatver { .. } => "STR-E-003",
            Self::ElementTypeDomainError { .. } => "STR-E-004",
            Self::DisconnectedElementError { .. } => "STR-E-005",
            Self::ContributorDomainError { .. } => "STR-E-006",
            Self::ChannelCountMismatch { .. } => "STR-E-007",
            Self::ImpoundmentCountMismatch { .. } => "STR-E-008",
            Self::HillslopeCoverageMismatch { .. } | Self::NhillContextError { .. } => "STR-E-009",
            Self::InvariantViolation { .. } => "STR-E-010",
            Self::RecordCountMismatch { .. } => "STR-E-011",
        }
    }
}

impl fmt::Display for WatershedStructureParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputOpenError { path, source } => write!(
                f,
                "{} failed to open watershed structure file '{}': {source}",
                self.contract_error_id(),
                path.display()
            ),
            Self::TokenParseError { line, field, token } => write!(
                f,
                "{} line {line}: failed to parse field '{field}' from token '{token}'",
                self.contract_error_id()
            ),
            Self::RecordArityError {
                line,
                expected,
                found,
            } => write!(
                f,
                "{} line {line}: expected {expected} token(s), found {found}",
                self.contract_error_id()
            ),
            Self::LegacyNoDatverDisallowed { line, token } => write!(
                f,
                "{} line {line}: strict mode requires explicit datver, got leading token '{token}'",
                self.contract_error_id()
            ),
            Self::UnsupportedDatver {
                line,
                datver,
                min_supported,
            } => write!(
                f,
                "{} line {line}: unsupported datver {datver}; minimum supported {min_supported}",
                self.contract_error_id()
            ),
            Self::ElementTypeDomainError { line, value } => write!(
                f,
                "{} line {line}: invalid element type code {value}; expected 2 or 3",
                self.contract_error_id()
            ),
            Self::DisconnectedElementError { line, record_index } => write!(
                f,
                "{} line {line}: structure row {record_index} has no non-zero contributors",
                self.contract_error_id()
            ),
            Self::ContributorDomainError {
                line,
                field,
                value,
                expected,
            } => write!(
                f,
                "{} line {line}: contributor field '{field}' has invalid value {value}; expected {expected}",
                self.contract_error_id()
            ),
            Self::ChannelCountMismatch { expected, observed } => write!(
                f,
                "{} channel count mismatch: expected {expected}, observed {observed}",
                self.contract_error_id()
            ),
            Self::ImpoundmentCountMismatch { expected, observed } => write!(
                f,
                "{} impoundment count mismatch: expected {expected}, observed {observed}",
                self.contract_error_id()
            ),
            Self::HillslopeCoverageMismatch {
                expected_nhill,
                observed_nhmax,
            } => write!(
                f,
                "{} hillslope coverage mismatch: expected nhill {expected_nhill}, observed nhmax {observed_nhmax}",
                self.contract_error_id()
            ),
            Self::NhillContextError { nhill } => write!(
                f,
                "{} invalid nhill context {nhill}; expected > 0",
                self.contract_error_id()
            ),
            Self::RecordCountMismatch { expected, observed } => write!(
                f,
                "{} structure row-count mismatch: expected {expected}, observed {observed}",
                self.contract_error_id()
            ),
            Self::InvariantViolation { context } => {
                write!(
                    f,
                    "{} invariant violation: {context}",
                    self.contract_error_id()
                )
            }
        }
    }
}

impl Error for WatershedStructureParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InputOpenError { source, .. } => Some(source),
            _ => None,
        }
    }
}

pub fn parse_watershed_structure_from_path(
    path: impl AsRef<Path>,
    options: WatershedStructureParseOptions,
) -> Result<WatershedStructureFile, WatershedStructureParseError> {
    let path = path.as_ref();
    let content = fs::read_to_string(path).map_err(|source| {
        WatershedStructureParseError::InputOpenError {
            path: path.to_path_buf(),
            source,
        }
    })?;
    parse_watershed_structure_from_str(&content, options)
}

pub fn parse_watershed_structure_from_str(
    input: &str,
    options: WatershedStructureParseOptions,
) -> Result<WatershedStructureFile, WatershedStructureParseError> {
    if options.nhill == 0 {
        return Err(WatershedStructureParseError::NhillContextError {
            nhill: options.nhill,
        });
    }

    let expected_rows =
        options
            .expected_rows
            .ok_or(WatershedStructureParseError::InvariantViolation {
                context: "expected_rows topology context is required for row closure",
            })?;

    let lines = materialize_lines(input);
    if lines.is_empty() {
        return Err(WatershedStructureParseError::RecordCountMismatch {
            expected: expected_rows,
            observed: 0,
        });
    }

    let first = &lines[0];
    let first_token =
        first
            .tokens
            .first()
            .ok_or(WatershedStructureParseError::InvariantViolation {
                context: "non-empty logical line must have at least one token",
            })?;
    let first_numeric = parse_f64(first.line, "datver_or_elmt", first_token)?;

    let (datver, datver_source, start_row_index, no_datver_warning_line) = if first_numeric > 10.0 {
        if first.tokens.len() != 1 {
            return Err(WatershedStructureParseError::RecordArityError {
                line: first.line,
                expected: 1,
                found: first.tokens.len(),
            });
        }
        if first_numeric + STR_DATVER_EPS < STR_DATVER_MIN {
            return Err(WatershedStructureParseError::UnsupportedDatver {
                line: first.line,
                datver: first_numeric,
                min_supported: STR_DATVER_MIN,
            });
        }
        (first_numeric, DatverSource::ExplicitHeader, 1usize, None)
    } else {
        match options.mode {
            ParseMode::Strict => {
                return Err(WatershedStructureParseError::LegacyNoDatverDisallowed {
                    line: first.line,
                    token: first_token.clone(),
                });
            }
            ParseMode::Compatibility => (
                STR_DATVER_MIN,
                DatverSource::LegacyCompatNoDatver,
                0usize,
                Some(first.line),
            ),
        }
    };

    let observed_rows = lines.len().saturating_sub(start_row_index);
    if observed_rows != expected_rows {
        return Err(WatershedStructureParseError::RecordCountMismatch {
            expected: expected_rows,
            observed: observed_rows,
        });
    }

    let mut parsed = WatershedStructureFile {
        datver,
        datver_source,
        nhill: options.nhill,
        rows: Vec::with_capacity(expected_rows),
        summary: WatershedStructureSummary {
            channel_count: 0,
            impoundment_count: 0,
            max_hillslope_ref: 0,
        },
        warnings: Vec::new(),
    };

    if let Some(line) = no_datver_warning_line {
        parsed.push_warning(
            WatershedStructureWarningCode::StrW001,
            line,
            "legacy no-datver acceptance path used in compatibility mode",
        );
    }

    let mut channel_local_index = 0usize;
    let mut impoundment_local_index = 0usize;

    for (offset, logical) in lines[start_row_index..].iter().enumerate() {
        if logical.tokens.len() != 10 {
            return Err(WatershedStructureParseError::RecordArityError {
                line: logical.line,
                expected: 10,
                found: logical.tokens.len(),
            });
        }

        let record_index = offset + 1;
        let element_id = (options.nhill + record_index) as i32;

        let elmt = parse_i32(logical.line, "elmt", &logical.tokens[0])?;
        let nhleft = parse_i32(logical.line, "nhleft", &logical.tokens[1])?;
        let nhrght = parse_i32(logical.line, "nhrght", &logical.tokens[2])?;
        let nhtop = parse_i32(logical.line, "nhtop", &logical.tokens[3])?;
        let ncleft = parse_i32(logical.line, "ncleft", &logical.tokens[4])?;
        let ncrght = parse_i32(logical.line, "ncrght", &logical.tokens[5])?;
        let nctop = parse_i32(logical.line, "nctop", &logical.tokens[6])?;
        let nileft = parse_i32(logical.line, "nileft", &logical.tokens[7])?;
        let nirght = parse_i32(logical.line, "nirght", &logical.tokens[8])?;
        let nitop = parse_i32(logical.line, "nitop", &logical.tokens[9])?;

        if elmt != 2 && elmt != 3 {
            return Err(WatershedStructureParseError::ElementTypeDomainError {
                line: logical.line,
                value: elmt,
            });
        }

        validate_hillslope_contributor(logical.line, "nhleft", nhleft, options.nhill)?;
        validate_hillslope_contributor(logical.line, "nhrght", nhrght, options.nhill)?;
        validate_hillslope_contributor(logical.line, "nhtop", nhtop, options.nhill)?;

        let min_element_ref = (options.nhill + 1) as i32;
        validate_structure_contributor(
            logical.line,
            "ncleft",
            ncleft,
            min_element_ref,
            element_id,
        )?;
        validate_structure_contributor(
            logical.line,
            "ncrght",
            ncrght,
            min_element_ref,
            element_id,
        )?;
        validate_structure_contributor(logical.line, "nctop", nctop, min_element_ref, element_id)?;
        validate_structure_contributor(
            logical.line,
            "nileft",
            nileft,
            min_element_ref,
            element_id,
        )?;
        validate_structure_contributor(
            logical.line,
            "nirght",
            nirght,
            min_element_ref,
            element_id,
        )?;
        validate_structure_contributor(logical.line, "nitop", nitop, min_element_ref, element_id)?;

        let contributors = [
            nhleft, nhrght, nhtop, ncleft, ncrght, nctop, nileft, nirght, nitop,
        ];
        if contributors.iter().all(|value| *value == 0) {
            return Err(WatershedStructureParseError::DisconnectedElementError {
                line: logical.line,
                record_index,
            });
        }

        let max_hillslope_in_row = [nhleft, nhrght, nhtop]
            .into_iter()
            .max()
            .unwrap_or(0)
            .max(0) as usize;
        parsed.summary.max_hillslope_ref =
            parsed.summary.max_hillslope_ref.max(max_hillslope_in_row);

        let element_local_index = if elmt == 2 {
            channel_local_index += 1;
            parsed.summary.channel_count += 1;
            channel_local_index
        } else {
            impoundment_local_index += 1;
            parsed.summary.impoundment_count += 1;
            impoundment_local_index
        };

        parsed.rows.push(WatershedStructureRow {
            record_index,
            element_id,
            element_type_code: elmt,
            element_local_index,
            nhleft,
            nhrght,
            nhtop,
            ncleft,
            ncrght,
            nctop,
            nileft,
            nirght,
            nitop,
        });
    }

    if parsed.summary.max_hillslope_ref != options.nhill {
        return Err(WatershedStructureParseError::HillslopeCoverageMismatch {
            expected_nhill: options.nhill,
            observed_nhmax: parsed.summary.max_hillslope_ref,
        });
    }

    if let Some(expected_channel_count) = options.expected_channel_count
        && parsed.summary.channel_count != expected_channel_count
    {
        return Err(WatershedStructureParseError::ChannelCountMismatch {
            expected: expected_channel_count,
            observed: parsed.summary.channel_count,
        });
    }

    if let Some(expected_impoundment_count) = options.expected_impoundment_count
        && parsed.summary.impoundment_count != expected_impoundment_count
    {
        return Err(WatershedStructureParseError::ImpoundmentCountMismatch {
            expected: expected_impoundment_count,
            observed: parsed.summary.impoundment_count,
        });
    }

    Ok(parsed)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LogicalLine {
    line: usize,
    tokens: Vec<String>,
}

fn materialize_lines(input: &str) -> Vec<LogicalLine> {
    let mut lines = Vec::new();
    for (index, raw_line) in input.lines().enumerate() {
        let line_number = index + 1;
        let trimmed = raw_line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let tokens = trimmed
            .split_whitespace()
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        lines.push(LogicalLine {
            line: line_number,
            tokens,
        });
    }
    lines
}

fn parse_f64(
    line: usize,
    field: &'static str,
    token: &str,
) -> Result<f64, WatershedStructureParseError> {
    token
        .parse::<f64>()
        .map_err(|_| WatershedStructureParseError::TokenParseError {
            line,
            field,
            token: token.to_string(),
        })
}

fn parse_i32(
    line: usize,
    field: &'static str,
    token: &str,
) -> Result<i32, WatershedStructureParseError> {
    token
        .parse::<i32>()
        .map_err(|_| WatershedStructureParseError::TokenParseError {
            line,
            field,
            token: token.to_string(),
        })
}

fn validate_hillslope_contributor(
    line: usize,
    field: &'static str,
    value: i32,
    nhill: usize,
) -> Result<(), WatershedStructureParseError> {
    if value < 0 {
        return Err(WatershedStructureParseError::ContributorDomainError {
            line,
            field,
            value,
            expected: ">= 0",
        });
    }

    if (value as usize) > nhill {
        return Err(WatershedStructureParseError::ContributorDomainError {
            line,
            field,
            value,
            expected: "0 or valid hillslope id <= nhill",
        });
    }

    Ok(())
}

fn validate_structure_contributor(
    line: usize,
    field: &'static str,
    value: i32,
    min_element_ref: i32,
    current_element_id: i32,
) -> Result<(), WatershedStructureParseError> {
    if value < 0 {
        return Err(WatershedStructureParseError::ContributorDomainError {
            line,
            field,
            value,
            expected: ">= 0",
        });
    }

    if value == 0 {
        return Ok(());
    }

    if value < min_element_ref || value >= current_element_id {
        return Err(WatershedStructureParseError::ContributorDomainError {
            line,
            field,
            value,
            expected: "0 or prior element id in watershed-element domain",
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn materialize_lines_skips_comment_and_blank_lines() {
        let parsed = materialize_lines("\n# comment\n2 1 0 0 0 0 0 0 0 0\n");
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].line, 3);
        assert_eq!(parsed[0].tokens[0], "2");
    }
}
