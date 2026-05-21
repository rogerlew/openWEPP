#![allow(
    clippy::missing_errors_doc,
    clippy::struct_excessive_bools,
    clippy::too_many_lines
)]

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::{Path, PathBuf};

const DEFAULT_WINT_RED: i32 = 1;
const DEFAULT_FINE_TOP: i32 = 10;
const DEFAULT_FINE_BOT: i32 = 10;
const DEFAULT_KSNOWF: f64 = 1.0;
const DEFAULT_KRESF: f64 = 1.0;
const DEFAULT_KSOILF: f64 = 1.0;
const DEFAULT_KFACTOR1: f64 = 0.000_01;
const DEFAULT_KFACTOR2: f64 = 0.000_01;
const DEFAULT_KFACTOR3: f64 = 0.5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseMode {
    Strict,
    Compatibility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrostWarningCode {
    FrostW001,
    FrostW002,
    FrostW003,
}

impl FrostWarningCode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FrostW001 => "FROST-W-001",
            Self::FrostW002 => "FROST-W-002",
            Self::FrostW003 => "FROST-W-003",
        }
    }
}

impl Display for FrostWarningCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrostWarning {
    pub code: FrostWarningCode,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FrostParseOutput {
    pub wint_red: i32,
    pub fine_top: i32,
    pub fine_bot: i32,
    pub ksnowf: f64,
    pub kresf: f64,
    pub ksoilf: f64,
    pub kfactor1: f64,
    pub kfactor2: f64,
    pub kfactor3: f64,
    pub frost_file_present: bool,
    pub line2_present: bool,
    pub legacy_clamp_applied: bool,
    pub legacy_clamp_fields: Vec<&'static str>,
    pub prefix_variant_detected: bool,
    pub warnings: Vec<FrostWarning>,
}

impl FrostParseOutput {
    #[must_use]
    pub fn defaults_for_missing_file(mode: ParseMode) -> Self {
        let mut warnings = Vec::new();
        if mode == ParseMode::Compatibility {
            warnings.push(FrostWarning {
                code: FrostWarningCode::FrostW001,
                message: "optional frost file missing; legacy defaults applied".to_string(),
            });
        }

        Self {
            wint_red: DEFAULT_WINT_RED,
            fine_top: DEFAULT_FINE_TOP,
            fine_bot: DEFAULT_FINE_BOT,
            ksnowf: DEFAULT_KSNOWF,
            kresf: DEFAULT_KRESF,
            ksoilf: DEFAULT_KSOILF,
            kfactor1: DEFAULT_KFACTOR1,
            kfactor2: DEFAULT_KFACTOR2,
            kfactor3: DEFAULT_KFACTOR3,
            frost_file_present: false,
            line2_present: false,
            legacy_clamp_applied: false,
            legacy_clamp_fields: Vec::new(),
            prefix_variant_detected: false,
            warnings,
        }
    }
}

#[derive(Debug)]
pub enum FrostParseError {
    FrostE000 {
        path: PathBuf,
        message: String,
    },
    FrostE001 {
        line: usize,
        field: &'static str,
        message: String,
    },
    FrostE002 {
        line: usize,
        field: &'static str,
        message: String,
    },
    FrostE003 {
        line: usize,
        field: &'static str,
        value: String,
    },
    FrostE004 {
        line: usize,
        field: &'static str,
        value: f64,
        allowed: &'static str,
    },
    FrostE005 {
        message: String,
    },
    FrostE006 {
        line: usize,
        token: String,
    },
}

impl FrostParseError {
    #[must_use]
    pub const fn contract_error_id(&self) -> &'static str {
        match self {
            Self::FrostE000 { .. } => "FROST-E-000",
            Self::FrostE001 { .. } => "FROST-E-001",
            Self::FrostE002 { .. } => "FROST-E-002",
            Self::FrostE003 { .. } => "FROST-E-003",
            Self::FrostE004 { .. } => "FROST-E-004",
            Self::FrostE005 { .. } => "FROST-E-005",
            Self::FrostE006 { .. } => "FROST-E-006",
        }
    }
}

impl Display for FrostParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FrostE000 { path, message } => {
                write!(
                    f,
                    "failed to open/read frost file '{}': {message}",
                    path.display()
                )
            }
            Self::FrostE001 {
                line,
                field,
                message,
            }
            | Self::FrostE002 {
                line,
                field,
                message,
            } => {
                write!(f, "line {line} parse error for {field}: {message}")
            }
            Self::FrostE003 { line, field, value } => {
                write!(
                    f,
                    "line {line}: non-finite value '{value}' for field {field}"
                )
            }
            Self::FrostE004 {
                line,
                field,
                value,
                allowed,
            } => {
                write!(
                    f,
                    "line {line}: value {value} for field {field} is out of range ({allowed})"
                )
            }
            Self::FrostE005 { message } => {
                write!(f, "closure invariant failure: {message}")
            }
            Self::FrostE006 { line, token } => {
                write!(
                    f,
                    "line {line}: unsupported prefixed/version-like leading token '{token}'"
                )
            }
        }
    }
}

impl Error for FrostParseError {}

pub fn parse_frost_from_path(
    path: impl AsRef<Path>,
    mode: ParseMode,
) -> Result<FrostParseOutput, FrostParseError> {
    let path = path.as_ref();

    if !path.exists() {
        return Ok(FrostParseOutput::defaults_for_missing_file(mode));
    }

    let raw = fs::read_to_string(path).map_err(|err| FrostParseError::FrostE000 {
        path: path.to_path_buf(),
        message: err.to_string(),
    })?;

    parse_frost_from_str(&raw, mode)
}

pub fn parse_frost_from_str(
    input: &str,
    mode: ParseMode,
) -> Result<FrostParseOutput, FrostParseError> {
    let records = collect_non_empty_lines(input);
    if records.is_empty() {
        return Ok(FrostParseOutput::defaults_for_missing_file(mode));
    }

    let (line1_no, line1) = records[0];
    if detect_prefixed_variant(line1) {
        let token = tokenize(line1)
            .first()
            .map_or_else(String::new, |t| (*t).to_string());
        return Err(FrostParseError::FrostE006 {
            line: line1_no,
            token,
        });
    }

    let line1_tokens = tokenize(line1);
    if line1_tokens.len() != 3 {
        return Err(FrostParseError::FrostE001 {
            line: line1_no,
            field: "line1",
            message: format!("expected 3 tokens, found {}", line1_tokens.len()),
        });
    }

    let wint_red = parse_i32(line1_tokens[0], line1_no, "wintRed", true)?;
    let fine_top = parse_i32(line1_tokens[1], line1_no, "fineTop", true)?;
    let fine_bot = parse_i32(line1_tokens[2], line1_no, "fineBot", true)?;

    let mut output = FrostParseOutput {
        wint_red,
        fine_top,
        fine_bot,
        ksnowf: DEFAULT_KSNOWF,
        kresf: DEFAULT_KRESF,
        ksoilf: DEFAULT_KSOILF,
        kfactor1: DEFAULT_KFACTOR1,
        kfactor2: DEFAULT_KFACTOR2,
        kfactor3: DEFAULT_KFACTOR3,
        frost_file_present: true,
        line2_present: false,
        legacy_clamp_applied: false,
        legacy_clamp_fields: Vec::new(),
        prefix_variant_detected: false,
        warnings: Vec::new(),
    };

    if let Some((line2_no, line2)) = records.get(1).copied() {
        output.line2_present = true;
        let line2_tokens = tokenize(line2);

        if line2_tokens.len() == 6 {
            match parse_line2_tokens(&line2_tokens, line2_no, mode) {
                Ok(parsed_line2) => {
                    output.ksnowf = parsed_line2.ksnowf;
                    output.kresf = parsed_line2.kresf;
                    output.ksoilf = parsed_line2.ksoilf;
                    output.kfactor1 = parsed_line2.kfactor1;
                    output.kfactor2 = parsed_line2.kfactor2;
                    output.kfactor3 = parsed_line2.kfactor3;

                    if mode == ParseMode::Compatibility {
                        let mut compat_tracker = CompatClampTracker::default();
                        output.wint_red = clamp_wint_red(output.wint_red, &mut compat_tracker);
                        output.fine_top =
                            clamp_fine_count(output.fine_top, "fineTop", &mut compat_tracker);
                        output.fine_bot =
                            clamp_fine_count(output.fine_bot, "fineBot", &mut compat_tracker);

                        output.ksnowf =
                            clamp_ks_factor(output.ksnowf, "ksnowf", &mut compat_tracker);
                        output.kresf = clamp_ks_factor(output.kresf, "kresf", &mut compat_tracker);
                        output.ksoilf =
                            clamp_ks_factor(output.ksoilf, "ksoilf", &mut compat_tracker);
                        output.kfactor1 = clamp_kfactor(
                            output.kfactor1,
                            "kfactor1",
                            DEFAULT_KFACTOR1,
                            &mut compat_tracker,
                        );
                        output.kfactor2 = clamp_kfactor(
                            output.kfactor2,
                            "kfactor2",
                            DEFAULT_KFACTOR2,
                            &mut compat_tracker,
                        );
                        output.kfactor3 = clamp_kfactor(
                            output.kfactor3,
                            "kfactor3",
                            DEFAULT_KFACTOR3,
                            &mut compat_tracker,
                        );

                        if compat_tracker.clamp_applied {
                            output.legacy_clamp_applied = true;
                            output.legacy_clamp_fields = compat_tracker.fields;
                            output.warnings.push(FrostWarning {
                                code: FrostWarningCode::FrostW003,
                                message: "legacy frost clamp/default normalization applied"
                                    .to_string(),
                            });
                        }
                    }
                }
                Err(err) => {
                    if mode == ParseMode::Strict {
                        return Err(err);
                    }

                    apply_compat_line2_defaults(&mut output);
                    output.warnings.push(FrostWarning {
                        code: FrostWarningCode::FrostW002,
                        message: "line2 parse failure in compatibility mode; defaults applied"
                            .to_string(),
                    });
                    output.line2_present = false;
                }
            }
        } else {
            if mode == ParseMode::Strict {
                return Err(FrostParseError::FrostE002 {
                    line: line2_no,
                    field: "line2",
                    message: format!("expected 6 tokens, found {}", line2_tokens.len()),
                });
            }

            apply_compat_line2_defaults(&mut output);
            output.warnings.push(FrostWarning {
                code: FrostWarningCode::FrostW002,
                message: "line2 missing/invalid arity in compatibility mode; defaults applied"
                    .to_string(),
            });
            output.line2_present = false;
        }
    } else if mode == ParseMode::Strict {
        return Err(FrostParseError::FrostE002 {
            line: line1_no,
            field: "line2",
            message: "missing required line2 record".to_string(),
        });
    } else {
        output.line2_present = false;
        apply_compat_line2_defaults(&mut output);
        output.warnings.push(FrostWarning {
            code: FrostWarningCode::FrostW002,
            message: "line2 missing in compatibility mode; defaults applied".to_string(),
        });
    }

    if mode == ParseMode::Strict {
        validate_strict_ranges(&output, line1_no)?;
    } else {
        let mut compat_tracker = CompatClampTracker::default();
        output.wint_red = clamp_wint_red(output.wint_red, &mut compat_tracker);
        output.fine_top = clamp_fine_count(output.fine_top, "fineTop", &mut compat_tracker);
        output.fine_bot = clamp_fine_count(output.fine_bot, "fineBot", &mut compat_tracker);
        output.ksnowf = clamp_ks_factor(output.ksnowf, "ksnowf", &mut compat_tracker);
        output.kresf = clamp_ks_factor(output.kresf, "kresf", &mut compat_tracker);
        output.ksoilf = clamp_ks_factor(output.ksoilf, "ksoilf", &mut compat_tracker);
        output.kfactor1 = clamp_kfactor(
            output.kfactor1,
            "kfactor1",
            DEFAULT_KFACTOR1,
            &mut compat_tracker,
        );
        output.kfactor2 = clamp_kfactor(
            output.kfactor2,
            "kfactor2",
            DEFAULT_KFACTOR2,
            &mut compat_tracker,
        );
        output.kfactor3 = clamp_kfactor(
            output.kfactor3,
            "kfactor3",
            DEFAULT_KFACTOR3,
            &mut compat_tracker,
        );

        if compat_tracker.clamp_applied {
            output.legacy_clamp_applied = true;
            for field in compat_tracker.fields {
                if !output.legacy_clamp_fields.contains(&field) {
                    output.legacy_clamp_fields.push(field);
                }
            }
            if !output
                .warnings
                .iter()
                .any(|warning| warning.code == FrostWarningCode::FrostW003)
            {
                output.warnings.push(FrostWarning {
                    code: FrostWarningCode::FrostW003,
                    message: "legacy frost clamp/default normalization applied".to_string(),
                });
            }
        }
    }

    if output.legacy_clamp_applied && output.legacy_clamp_fields.is_empty() {
        return Err(FrostParseError::FrostE005 {
            message: "legacy_clamp_applied=true requires non-empty legacy_clamp_fields".to_string(),
        });
    }

    Ok(output)
}

#[derive(Debug, Clone, Copy)]
struct ParsedLine2 {
    ksnowf: f64,
    kresf: f64,
    ksoilf: f64,
    kfactor1: f64,
    kfactor2: f64,
    kfactor3: f64,
}

fn parse_line2_tokens(
    line2_tokens: &[&str],
    line_no: usize,
    mode: ParseMode,
) -> Result<ParsedLine2, FrostParseError> {
    let ksnowf = parse_f64(line2_tokens[0], line_no, "ksnowf", false)?;
    let kresf = parse_f64(line2_tokens[1], line_no, "kresf", false)?;
    let ksoilf = parse_f64(line2_tokens[2], line_no, "ksoilf", false)?;
    let kfactor1 = parse_f64(line2_tokens[3], line_no, "kfactor1", false)?;
    let kfactor2 = parse_f64(line2_tokens[4], line_no, "kfactor2", false)?;
    let kfactor3 = parse_f64(line2_tokens[5], line_no, "kfactor3", false)?;

    if mode == ParseMode::Strict {
        for (field, value) in [
            ("ksnowf", ksnowf),
            ("kresf", kresf),
            ("ksoilf", ksoilf),
            ("kfactor1", kfactor1),
            ("kfactor2", kfactor2),
            ("kfactor3", kfactor3),
        ] {
            if !value.is_finite() {
                return Err(FrostParseError::FrostE003 {
                    line: line_no,
                    field,
                    value: value.to_string(),
                });
            }
        }
    }

    Ok(ParsedLine2 {
        ksnowf,
        kresf,
        ksoilf,
        kfactor1,
        kfactor2,
        kfactor3,
    })
}

#[derive(Default)]
struct CompatClampTracker {
    clamp_applied: bool,
    fields: Vec<&'static str>,
}

impl CompatClampTracker {
    fn mark(&mut self, field: &'static str) {
        self.clamp_applied = true;
        if !self.fields.contains(&field) {
            self.fields.push(field);
        }
    }
}

fn apply_compat_line2_defaults(output: &mut FrostParseOutput) {
    output.ksnowf = DEFAULT_KSNOWF;
    output.kresf = DEFAULT_KRESF;
    output.ksoilf = DEFAULT_KSOILF;
    output.kfactor1 = DEFAULT_KFACTOR1;
    output.kfactor2 = DEFAULT_KFACTOR2;
    output.kfactor3 = DEFAULT_KFACTOR3;

    output.legacy_clamp_applied = true;
    for field in [
        "ksnowf", "kresf", "ksoilf", "kfactor1", "kfactor2", "kfactor3",
    ] {
        if !output.legacy_clamp_fields.contains(&field) {
            output.legacy_clamp_fields.push(field);
        }
    }

    if !output
        .warnings
        .iter()
        .any(|warning| warning.code == FrostWarningCode::FrostW003)
    {
        output.warnings.push(FrostWarning {
            code: FrostWarningCode::FrostW003,
            message: "legacy frost clamp/default normalization applied".to_string(),
        });
    }
}

fn validate_strict_ranges(
    output: &FrostParseOutput,
    line_no: usize,
) -> Result<(), FrostParseError> {
    if output.wint_red != 0 && output.wint_red != 1 {
        return Err(FrostParseError::FrostE004 {
            line: line_no,
            field: "wintRed",
            value: f64::from(output.wint_red),
            allowed: "{0,1}",
        });
    }

    validate_range_i32(output.fine_top, "fineTop", 1, 10, line_no)?;
    validate_range_i32(output.fine_bot, "fineBot", 1, 10, line_no)?;

    validate_range_f64(output.ksnowf, "ksnowf", 0.1, 10.0, line_no)?;
    validate_range_f64(output.kresf, "kresf", 0.1, 10.0, line_no)?;
    validate_range_f64(output.ksoilf, "ksoilf", 0.1, 10.0, line_no)?;

    validate_kfactor(output.kfactor1, "kfactor1", line_no)?;
    validate_kfactor(output.kfactor2, "kfactor2", line_no)?;
    validate_kfactor(output.kfactor3, "kfactor3", line_no)?;

    Ok(())
}

fn validate_range_i32(
    value: i32,
    field: &'static str,
    min: i32,
    max: i32,
    line_no: usize,
) -> Result<(), FrostParseError> {
    if value < min || value > max {
        return Err(FrostParseError::FrostE004 {
            line: line_no,
            field,
            value: f64::from(value),
            allowed: "inclusive integer range",
        });
    }
    Ok(())
}

fn validate_range_f64(
    value: f64,
    field: &'static str,
    min: f64,
    max: f64,
    line_no: usize,
) -> Result<(), FrostParseError> {
    if !value.is_finite() {
        return Err(FrostParseError::FrostE003 {
            line: line_no,
            field,
            value: value.to_string(),
        });
    }

    if value < min || value > max {
        return Err(FrostParseError::FrostE004 {
            line: line_no,
            field,
            value,
            allowed: "inclusive floating range",
        });
    }
    Ok(())
}

fn validate_kfactor(
    value: f64,
    field: &'static str,
    line_no: usize,
) -> Result<(), FrostParseError> {
    if !value.is_finite() {
        return Err(FrostParseError::FrostE003 {
            line: line_no,
            field,
            value: value.to_string(),
        });
    }
    if value <= 0.0 || value > 1.0 {
        return Err(FrostParseError::FrostE004 {
            line: line_no,
            field,
            value,
            allowed: "(0,1]",
        });
    }
    Ok(())
}

fn clamp_wint_red(value: i32, tracker: &mut CompatClampTracker) -> i32 {
    if value == 0 || value == 1 {
        value
    } else {
        tracker.mark("wintRed");
        DEFAULT_WINT_RED
    }
}

fn clamp_fine_count(value: i32, field: &'static str, tracker: &mut CompatClampTracker) -> i32 {
    if (1..=10).contains(&value) {
        value
    } else {
        tracker.mark(field);
        10
    }
}

fn clamp_ks_factor(value: f64, field: &'static str, tracker: &mut CompatClampTracker) -> f64 {
    if value.is_finite() && (0.1..=10.0).contains(&value) {
        value
    } else {
        tracker.mark(field);
        1.0
    }
}

fn clamp_kfactor(
    value: f64,
    field: &'static str,
    fallback: f64,
    tracker: &mut CompatClampTracker,
) -> f64 {
    if value.is_finite() && value > 0.0 && value <= 1.0 {
        value
    } else {
        tracker.mark(field);
        fallback
    }
}

fn parse_i32(
    token: &str,
    line_no: usize,
    field: &'static str,
    is_line1: bool,
) -> Result<i32, FrostParseError> {
    token.parse::<i32>().map_err(|_| {
        if is_line1 {
            FrostParseError::FrostE001 {
                line: line_no,
                field,
                message: format!("failed to parse integer token '{token}'"),
            }
        } else {
            FrostParseError::FrostE002 {
                line: line_no,
                field,
                message: format!("failed to parse integer token '{token}'"),
            }
        }
    })
}

fn parse_f64(
    token: &str,
    line_no: usize,
    field: &'static str,
    is_line1: bool,
) -> Result<f64, FrostParseError> {
    token.parse::<f64>().map_err(|_| {
        if is_line1 {
            FrostParseError::FrostE001 {
                line: line_no,
                field,
                message: format!("failed to parse real token '{token}'"),
            }
        } else {
            FrostParseError::FrostE002 {
                line: line_no,
                field,
                message: format!("failed to parse real token '{token}'"),
            }
        }
    })
}

fn detect_prefixed_variant(line: &str) -> bool {
    let tokens = tokenize(line);
    if tokens.len() != 1 {
        return false;
    }

    let token = tokens[0].to_ascii_lowercase();
    token.parse::<f64>().is_ok()
        || token.contains("datver")
        || token.contains("version")
        || token.starts_with('v')
            && token
                .chars()
                .skip(1)
                .all(|c| c.is_ascii_digit() || c == '.')
}

fn collect_non_empty_lines(input: &str) -> Vec<(usize, &str)> {
    input
        .lines()
        .enumerate()
        .filter_map(|(index, raw)| {
            let trimmed = raw.trim();
            (!trimmed.is_empty()).then_some((index + 1, trimmed))
        })
        .collect()
}

fn tokenize(line: &str) -> Vec<&str> {
    line.split(|c: char| c.is_whitespace() || c == ',')
        .filter(|token| !token.is_empty())
        .collect()
}
