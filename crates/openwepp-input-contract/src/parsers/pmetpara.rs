#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::similar_names,
    clippy::too_many_lines
)]

use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

const LEGACY_CROP_KEY_WIDTH: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseMode {
    Strict,
    Compatibility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PmetparaParseOptions {
    pub mode: ParseMode,
    pub require_sidecar: bool,
}

impl Default for PmetparaParseOptions {
    fn default() -> Self {
        Self {
            mode: ParseMode::Strict,
            require_sidecar: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PmetWarningCode {
    PmetW001,
    PmetW002,
    PmetW003,
    PmetW004,
}

impl PmetWarningCode {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PmetW001 => "PMET-W-001",
            Self::PmetW002 => "PMET-W-002",
            Self::PmetW003 => "PMET-W-003",
            Self::PmetW004 => "PMET-W-004",
        }
    }
}

impl fmt::Display for PmetWarningCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PmetWarning {
    pub code: PmetWarningCode,
    pub line: usize,
    pub field: &'static str,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PmetparaRecord {
    pub crop_name: String,
    pub normalized_crop_key: String,
    pub kcb: f64,
    pub rawp: f64,
    pub line_index: i32,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PmetLookupState {
    pub fallback_first_row_used: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PmetparaFile {
    pub sidecar_present: bool,
    pub iflget: i32,
    pub record_count: usize,
    pub line_count_closed: bool,
    pub records: Vec<PmetparaRecord>,
    pub warnings: Vec<PmetWarning>,
    pub lookup: PmetLookupState,
}

impl PmetparaFile {
    pub fn lookup_record(
        &mut self,
        crop_name: &str,
        mode: ParseMode,
    ) -> Result<&PmetparaRecord, PmetparaParseError> {
        if !self.sidecar_present || self.records.is_empty() {
            return Err(PmetparaParseError::InvariantViolation {
                line: 0,
                context: "lookup attempted without parsed PMET records",
            });
        }

        let (query_key, truncated) = normalize_lookup_query(crop_name, mode);
        if mode == ParseMode::Compatibility && truncated {
            self.push_warning(
                PmetWarningCode::PmetW002,
                0,
                "crop_name",
                "lookup crop key was normalized/truncated for compatibility",
            );
        }

        if let Some(index) = self
            .records
            .iter()
            .position(|record| record.normalized_crop_key == query_key)
        {
            self.lookup.fallback_first_row_used = false;
            return Ok(&self.records[index]);
        }

        match mode {
            ParseMode::Strict => Err(PmetparaParseError::CropNameMissingError {
                crop_name: crop_name.to_string(),
                normalized_crop_key: query_key,
            }),
            ParseMode::Compatibility => {
                self.lookup.fallback_first_row_used = true;
                self.push_warning(
                    PmetWarningCode::PmetW003,
                    0,
                    "crop_name",
                    "no PMET crop-key hit; using first-row fallback",
                );
                Ok(&self.records[0])
            }
        }
    }

    fn push_warning(
        &mut self,
        code: PmetWarningCode,
        line: usize,
        field: &'static str,
        message: impl Into<String>,
    ) {
        self.warnings.push(PmetWarning {
            code,
            line,
            field,
            message: message.into(),
        });
    }
}

#[derive(Debug)]
pub enum PmetparaParseError {
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    RequiredSidecarMissingError {
        path: PathBuf,
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
    RecordCountError {
        declared: usize,
        parsed: usize,
    },
    FieldRangeError {
        line: usize,
        field: &'static str,
        value: String,
    },
    DuplicateCropKeyError {
        line: usize,
        normalized_crop_key: String,
    },
    UnsupportedHeaderVariant {
        line: usize,
        token: String,
    },
    CropNameMissingError {
        crop_name: String,
        normalized_crop_key: String,
    },
    ActlnamTokenizationError {
        line: usize,
        value: String,
    },
    InvariantViolation {
        line: usize,
        context: &'static str,
    },
}

impl PmetparaParseError {
    #[must_use]
    pub fn contract_error_id(&self) -> &'static str {
        match self {
            Self::Io { .. } => "PMET-E-000",
            Self::TokenParseError { .. } | Self::RecordArityError { .. } => "PMET-E-001",
            Self::RecordCountError { .. } => "PMET-E-002",
            Self::FieldRangeError { .. } | Self::DuplicateCropKeyError { .. } => "PMET-E-003",
            Self::UnsupportedHeaderVariant { .. } => "PMET-E-004",
            Self::CropNameMissingError { .. } => "PMET-E-005",
            Self::InvariantViolation { .. } => "PMET-E-006",
            Self::RequiredSidecarMissingError { .. } => "PMET-E-007",
            Self::ActlnamTokenizationError { .. } => "PMET-E-008",
        }
    }
}

impl fmt::Display for PmetparaParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, source } => write!(
                f,
                "{} failed to read pmetpara sidecar '{}': {source}",
                self.contract_error_id(),
                path.display()
            ),
            Self::RequiredSidecarMissingError { path } => write!(
                f,
                "{} required pmetpara sidecar is missing: '{}'",
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
                "{} line {line}: expected {expected} row token(s), found {found}",
                self.contract_error_id()
            ),
            Self::RecordCountError { declared, parsed } => write!(
                f,
                "{} row-count mismatch: declared={declared}, parsed={parsed}",
                self.contract_error_id()
            ),
            Self::FieldRangeError { line, field, value } => write!(
                f,
                "{} line {line}: value '{value}' violates domain for '{field}'",
                self.contract_error_id()
            ),
            Self::DuplicateCropKeyError {
                line,
                normalized_crop_key,
            } => write!(
                f,
                "{} line {line}: duplicate normalized crop key '{normalized_crop_key}'",
                self.contract_error_id()
            ),
            Self::UnsupportedHeaderVariant { line, token } => write!(
                f,
                "{} line {line}: unsupported pmetpara header variant token '{token}'",
                self.contract_error_id()
            ),
            Self::CropNameMissingError {
                crop_name,
                normalized_crop_key,
            } => write!(
                f,
                "{} no PMET record found for crop '{crop_name}' (normalized '{normalized_crop_key}')",
                self.contract_error_id()
            ),
            Self::ActlnamTokenizationError { line, value } => write!(
                f,
                "{} line {line}: unsupported strict actlnam tokenization '{value}'",
                self.contract_error_id()
            ),
            Self::InvariantViolation { line, context } => write!(
                f,
                "{} line {line}: invariant violation: {context}",
                self.contract_error_id()
            ),
        }
    }
}

impl Error for PmetparaParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}

pub fn parse_pmetpara_file(
    path: impl AsRef<Path>,
    options: PmetparaParseOptions,
) -> Result<PmetparaFile, PmetparaParseError> {
    let path = path.as_ref();

    match fs::read_to_string(path) {
        Ok(content) => parse_pmetpara_from_str(&content, options),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            if options.require_sidecar && options.mode == ParseMode::Strict {
                Err(PmetparaParseError::RequiredSidecarMissingError {
                    path: path.to_path_buf(),
                })
            } else {
                Ok(absent_sidecar_output(options.mode))
            }
        }
        Err(source) => Err(PmetparaParseError::Io {
            path: path.to_path_buf(),
            source,
        }),
    }
}

pub fn parse_pmetpara_from_str(
    input: &str,
    options: PmetparaParseOptions,
) -> Result<PmetparaFile, PmetparaParseError> {
    let mut warnings = Vec::new();
    let lines = materialize_lines(input, options.mode);

    if lines.is_empty() {
        return Err(PmetparaParseError::TokenParseError {
            line: 0,
            field: "irecord",
            token: String::new(),
        });
    }

    let header = lines[0];
    let declared_count = parse_declared_count(header.number, header.text)?;

    let mut records = Vec::with_capacity(declared_count);
    for line in &lines[1..] {
        records.push(parse_record_line(*line, options.mode, &mut warnings)?);
    }

    if records.len() != declared_count {
        return Err(PmetparaParseError::RecordCountError {
            declared: declared_count,
            parsed: records.len(),
        });
    }

    if options.mode == ParseMode::Strict {
        let mut seen = HashSet::with_capacity(records.len());
        for record in &records {
            if !seen.insert(record.normalized_crop_key.clone()) {
                let line = usize::try_from(record.line_index).map_err(|_| {
                    PmetparaParseError::InvariantViolation {
                        line: 0,
                        context: "record line_index could not be represented as usize",
                    }
                })?;
                return Err(PmetparaParseError::DuplicateCropKeyError {
                    line,
                    normalized_crop_key: record.normalized_crop_key.clone(),
                });
            }
        }
    }

    Ok(PmetparaFile {
        sidecar_present: true,
        iflget: 2,
        record_count: declared_count,
        line_count_closed: true,
        records,
        warnings,
        lookup: PmetLookupState {
            fallback_first_row_used: false,
        },
    })
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
        .filter_map(|(index, raw)| {
            let trimmed = raw.trim();
            if trimmed.is_empty() {
                return None;
            }
            if mode == ParseMode::Compatibility && trimmed.starts_with('#') {
                return None;
            }
            Some(LocatedLine {
                number: index + 1,
                text: trimmed,
            })
        })
        .collect()
}

fn parse_declared_count(line: usize, token_line: &str) -> Result<usize, PmetparaParseError> {
    let token = token_line.trim();

    if token.contains(',') {
        return Err(PmetparaParseError::UnsupportedHeaderVariant {
            line,
            token: token.to_string(),
        });
    }

    let parsed_i64 = token.parse::<i64>();
    match parsed_i64 {
        Ok(value) => {
            if value <= 0 {
                return Err(PmetparaParseError::FieldRangeError {
                    line,
                    field: "irecord",
                    value: value.to_string(),
                });
            }
            usize::try_from(value).map_err(|_| PmetparaParseError::FieldRangeError {
                line,
                field: "irecord",
                value: value.to_string(),
            })
        }
        Err(_) => {
            if token.parse::<f64>().is_ok() {
                Err(PmetparaParseError::UnsupportedHeaderVariant {
                    line,
                    token: token.to_string(),
                })
            } else {
                Err(PmetparaParseError::TokenParseError {
                    line,
                    field: "irecord",
                    token: token.to_string(),
                })
            }
        }
    }
}

fn parse_record_line(
    line: LocatedLine<'_>,
    mode: ParseMode,
    warnings: &mut Vec<PmetWarning>,
) -> Result<PmetparaRecord, PmetparaParseError> {
    let parts = tokenize_row(line.number, line.text, mode, warnings)?;

    let crop_name = parts[0].trim().to_string();
    if crop_name.is_empty() {
        return Err(PmetparaParseError::FieldRangeError {
            line: line.number,
            field: "names",
            value: String::from("<empty>"),
        });
    }

    let (normalized_crop_key, key_normalization_warned) = normalize_crop_key(&crop_name, mode);
    if key_normalization_warned {
        warnings.push(PmetWarning {
            code: PmetWarningCode::PmetW002,
            line: line.number,
            field: "names",
            message: String::from("crop key normalized/truncated in compatibility mode"),
        });
    }

    if mode == ParseMode::Strict && crop_name.trim().len() > LEGACY_CROP_KEY_WIDTH {
        return Err(PmetparaParseError::FieldRangeError {
            line: line.number,
            field: "names",
            value: crop_name,
        });
    }

    let kcb = parse_f64(line.number, "kcb", &parts[1])?;
    let rawp = parse_f64(line.number, "rawp", &parts[2])?;
    let line_index = parse_i32(line.number, "line", &parts[3])?;

    if !kcb.is_finite() {
        return Err(PmetparaParseError::FieldRangeError {
            line: line.number,
            field: "kcb",
            value: parts[1].clone(),
        });
    }
    if !rawp.is_finite() {
        return Err(PmetparaParseError::FieldRangeError {
            line: line.number,
            field: "rawp",
            value: parts[2].clone(),
        });
    }
    if line_index <= 0 {
        return Err(PmetparaParseError::FieldRangeError {
            line: line.number,
            field: "line",
            value: line_index.to_string(),
        });
    }

    let description = normalize_description(line.number, &parts[4], mode, warnings)?;

    Ok(PmetparaRecord {
        crop_name,
        normalized_crop_key,
        kcb,
        rawp,
        line_index,
        description,
    })
}

fn tokenize_row(
    line: usize,
    text: &str,
    mode: ParseMode,
    warnings: &mut Vec<PmetWarning>,
) -> Result<[String; 5], PmetparaParseError> {
    if text.contains(',') {
        let fields: Vec<&str> = text.split(',').map(str::trim).collect();
        if fields.len() != 5 {
            return Err(PmetparaParseError::RecordArityError {
                line,
                expected: 5,
                found: fields.len(),
            });
        }

        return Ok([
            fields[0].to_string(),
            fields[1].to_string(),
            fields[2].to_string(),
            fields[3].to_string(),
            fields[4].to_string(),
        ]);
    }

    let fields: Vec<&str> = text.split_whitespace().collect();
    if fields.len() < 5 {
        return Err(PmetparaParseError::RecordArityError {
            line,
            expected: 5,
            found: fields.len(),
        });
    }

    if fields.len() == 5 {
        return Ok([
            fields[0].to_string(),
            fields[1].to_string(),
            fields[2].to_string(),
            fields[3].to_string(),
            fields[4].to_string(),
        ]);
    }

    if mode == ParseMode::Strict {
        return Err(PmetparaParseError::ActlnamTokenizationError {
            line,
            value: fields[4..].join(" "),
        });
    }

    let description = fields[4..].join(" ");
    warnings.push(PmetWarning {
        code: PmetWarningCode::PmetW004,
        line,
        field: "actlnam",
        message: String::from("non-canonical multi-token actlnam normalized"),
    });

    Ok([
        fields[0].to_string(),
        fields[1].to_string(),
        fields[2].to_string(),
        fields[3].to_string(),
        description,
    ])
}

fn normalize_description(
    line: usize,
    value: &str,
    mode: ParseMode,
    warnings: &mut Vec<PmetWarning>,
) -> Result<String, PmetparaParseError> {
    let has_spaces = value.chars().any(char::is_whitespace);
    let has_quotes = value.contains('"') || value.contains('\'');

    if mode == ParseMode::Strict && (has_spaces || has_quotes) {
        return Err(PmetparaParseError::ActlnamTokenizationError {
            line,
            value: value.to_string(),
        });
    }

    if mode == ParseMode::Compatibility && (has_spaces || has_quotes) {
        warnings.push(PmetWarning {
            code: PmetWarningCode::PmetW004,
            line,
            field: "actlnam",
            message: String::from("actlnam normalized for compatibility"),
        });

        let stripped = value.trim_matches(|c| c == '\'' || c == '"').trim();
        let normalized = stripped.split_whitespace().collect::<Vec<_>>().join("_");
        return Ok(normalized);
    }

    Ok(value.trim().to_string())
}

fn normalize_crop_key(raw: &str, mode: ParseMode) -> (String, bool) {
    match mode {
        ParseMode::Strict => (raw.trim().to_string(), false),
        ParseMode::Compatibility => {
            let upper = raw.trim().to_ascii_uppercase();
            if upper.len() > LEGACY_CROP_KEY_WIDTH {
                let truncated = upper[..LEGACY_CROP_KEY_WIDTH].to_string();
                (truncated, true)
            } else {
                let changed = upper != raw.trim();
                (upper, changed)
            }
        }
    }
}

fn normalize_lookup_query(raw: &str, mode: ParseMode) -> (String, bool) {
    match mode {
        ParseMode::Strict => (raw.trim().to_string(), false),
        ParseMode::Compatibility => {
            let upper = raw.trim().to_ascii_uppercase();
            if upper.len() > LEGACY_CROP_KEY_WIDTH {
                (upper[..LEGACY_CROP_KEY_WIDTH].to_string(), true)
            } else {
                (upper, false)
            }
        }
    }
}

fn parse_f64(line: usize, field: &'static str, token: &str) -> Result<f64, PmetparaParseError> {
    token
        .parse::<f64>()
        .map_err(|_| PmetparaParseError::TokenParseError {
            line,
            field,
            token: token.to_string(),
        })
}

fn parse_i32(line: usize, field: &'static str, token: &str) -> Result<i32, PmetparaParseError> {
    token
        .parse::<i32>()
        .map_err(|_| PmetparaParseError::TokenParseError {
            line,
            field,
            token: token.to_string(),
        })
}

fn absent_sidecar_output(mode: ParseMode) -> PmetparaFile {
    let mut output = PmetparaFile {
        sidecar_present: false,
        iflget: 1,
        record_count: 0,
        line_count_closed: true,
        records: Vec::new(),
        warnings: Vec::new(),
        lookup: PmetLookupState {
            fallback_first_row_used: false,
        },
    };

    if mode == ParseMode::Compatibility {
        output.push_warning(
            PmetWarningCode::PmetW001,
            0,
            "pmetpara.txt",
            "missing sidecar branch applied (iflget=1)",
        );
    }

    output
}
