#![allow(
    clippy::cast_sign_loss,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::similar_names,
    clippy::too_many_lines
)]

use std::collections::BTreeSet;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const DEFAULT_DTCHR_LOWER_BOUND_S: i32 = 60;
const DEFAULT_DTCHR_UPPER_BOUND_S: i32 = 3_600;
const DEFAULT_MXTCHR: i32 = 1_440;
const SECONDS_PER_DAY: f64 = 86_400.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseMode {
    Strict,
    Compatibility,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChaninpParseOptions {
    pub mode: ParseMode,
    pub ipeak: i32,
    pub nchan: usize,
    pub dtchr_lower_bound_s: i32,
    pub dtchr_upper_bound_s: i32,
    pub mxtchr: i32,
}

impl ChaninpParseOptions {
    #[must_use]
    pub const fn strict(ipeak: i32, nchan: usize) -> Self {
        Self {
            mode: ParseMode::Strict,
            ipeak,
            nchan,
            dtchr_lower_bound_s: DEFAULT_DTCHR_LOWER_BOUND_S,
            dtchr_upper_bound_s: DEFAULT_DTCHR_UPPER_BOUND_S,
            mxtchr: DEFAULT_MXTCHR,
        }
    }

    #[must_use]
    pub const fn compatibility(ipeak: i32, nchan: usize) -> Self {
        Self {
            mode: ParseMode::Compatibility,
            ipeak,
            nchan,
            dtchr_lower_bound_s: DEFAULT_DTCHR_LOWER_BOUND_S,
            dtchr_upper_bound_s: DEFAULT_DTCHR_UPPER_BOUND_S,
            mxtchr: DEFAULT_MXTCHR,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChaninpParseOutcome {
    NotApplicable,
    ParsedBranch,
    DefaultedCompat,
    OpenErrorCollapsedCompat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChaninpWarningCode {
    ChnW001,
    ChnW002,
    ChnW003,
    ChnW004,
    ChnW005,
}

impl ChaninpWarningCode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ChnW001 => "CHN-W-001",
            Self::ChnW002 => "CHN-W-002",
            Self::ChnW003 => "CHN-W-003",
            Self::ChnW004 => "CHN-W-004",
            Self::ChnW005 => "CHN-W-005",
        }
    }
}

impl fmt::Display for ChaninpWarningCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChaninpWarning {
    pub code: ChaninpWarningCode,
    pub line: Option<usize>,
    pub message: String,
}

impl ChaninpWarning {
    fn new(code: ChaninpWarningCode, line: Option<usize>, message: impl Into<String>) -> Self {
        Self {
            code,
            line,
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChaninpFile {
    pub chaninp_required: bool,
    pub parse_outcome: ChaninpParseOutcome,
    pub ipeak: i32,
    pub nchan: usize,
    pub line_count_closed: bool,
    pub trailing_token_lines: Vec<usize>,
    pub unknown_ichnum_retained_warning_emitted: bool,
    pub warnings: Vec<ChaninpWarning>,
    pub options: Option<ChaninpOptions>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChaninpOptions {
    pub ichout: i32,
    pub dtchr_input_s: f64,
    pub cbase_m3_s_m2: f64,
    pub nchnum_input: i32,
    pub ichnum_input: Vec<i32>,
    pub dtchr_norm_s: i32,
    pub ntchr: i32,
    pub nchnum_norm: i32,
    pub ichnum_norm: Vec<i32>,
    pub chan_output_enabled: bool,
}

#[derive(Debug)]
pub enum ChaninpParseError {
    ChnE000 {
        path: PathBuf,
        source: io::Error,
    },
    ChnE001 {
        line: usize,
        field: &'static str,
        token: String,
    },
    ChnE002 {
        line: usize,
        field: &'static str,
        expected: usize,
        found: usize,
    },
    ChnE003 {
        line: usize,
        field: &'static str,
        token: String,
    },
    ChnE004 {
        line: usize,
        field: &'static str,
        value: f64,
        rule: &'static str,
    },
    ChnE005 {
        line: usize,
        field: &'static str,
        value: i32,
        detail: &'static str,
    },
    ChnE006 {
        dtchr_norm_s: i32,
        ntchr: i32,
        context: &'static str,
    },
    ChnE007 {
        context: &'static str,
    },
    ChnE008 {
        line: usize,
        token: String,
    },
    ChnE009 {
        path: PathBuf,
    },
}

impl ChaninpParseError {
    #[must_use]
    pub const fn contract_error_id(&self) -> &'static str {
        match self {
            Self::ChnE000 { .. } => "CHN-E-000",
            Self::ChnE001 { .. } => "CHN-E-001",
            Self::ChnE002 { .. } => "CHN-E-002",
            Self::ChnE003 { .. } => "CHN-E-003",
            Self::ChnE004 { .. } => "CHN-E-004",
            Self::ChnE005 { .. } => "CHN-E-005",
            Self::ChnE006 { .. } => "CHN-E-006",
            Self::ChnE007 { .. } => "CHN-E-007",
            Self::ChnE008 { .. } => "CHN-E-008",
            Self::ChnE009 { .. } => "CHN-E-009",
        }
    }
}

impl fmt::Display for ChaninpParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ChnE000 { path, source } => write!(
                f,
                "{}: failed to open/read '{}': {source}",
                self.contract_error_id(),
                path.display()
            ),
            Self::ChnE001 { line, field, token } => write!(
                f,
                "{}: line {line} token parse failure for '{field}' from token '{token}'",
                self.contract_error_id()
            ),
            Self::ChnE002 {
                line,
                field,
                expected,
                found,
            } => write!(
                f,
                "{}: line {line} record closure for '{field}' expected {expected} token(s), found {found}",
                self.contract_error_id()
            ),
            Self::ChnE003 { line, field, token } => write!(
                f,
                "{}: line {line} non-finite numeric value for '{field}' from token '{token}'",
                self.contract_error_id()
            ),
            Self::ChnE004 {
                line,
                field,
                value,
                rule,
            } => write!(
                f,
                "{}: line {line} field '{field}' value {value} violates {rule}",
                self.contract_error_id()
            ),
            Self::ChnE005 {
                line,
                field,
                value,
                detail,
            } => write!(
                f,
                "{}: line {line} topology closure for '{field}' with value {value}: {detail}",
                self.contract_error_id()
            ),
            Self::ChnE006 {
                dtchr_norm_s,
                ntchr,
                context,
            } => write!(
                f,
                "{}: normalized timestep closure failed (dtchr_norm_s={dtchr_norm_s}, ntchr={ntchr}): {context}",
                self.contract_error_id()
            ),
            Self::ChnE007 { context } => {
                write!(
                    f,
                    "{}: invariant violation: {context}",
                    self.contract_error_id()
                )
            }
            Self::ChnE008 { line, token } => write!(
                f,
                "{}: line {line} unsupported prefixed/datver-style variant token '{token}'",
                self.contract_error_id()
            ),
            Self::ChnE009 { path } => write!(
                f,
                "{}: required sidecar missing '{}'; strict mode requires chan.inp when ipeak>2",
                self.contract_error_id(),
                path.display()
            ),
        }
    }
}

impl std::error::Error for ChaninpParseError {}

pub fn parse_chaninp_from_path(
    path: impl AsRef<Path>,
    options: ChaninpParseOptions,
    valid_channel_element_ids: &BTreeSet<i32>,
) -> Result<ChaninpFile, ChaninpParseError> {
    let path = path.as_ref();

    if options.ipeak <= 2 {
        return Ok(not_applicable_output(options));
    }

    if !path.exists() {
        return match options.mode {
            ParseMode::Strict => Err(ChaninpParseError::ChnE009 {
                path: path.to_path_buf(),
            }),
            ParseMode::Compatibility => Ok(defaulted_output(
                options,
                ChaninpParseOutcome::DefaultedCompat,
                ChaninpWarning::new(
                    ChaninpWarningCode::ChnW001,
                    None,
                    "compatibility fallback applied for missing chan.inp under ipeak>2",
                ),
            )),
        };
    }

    match fs::read_to_string(path) {
        Ok(raw) => parse_chaninp_from_str(&raw, options, valid_channel_element_ids),
        Err(source) if source.kind() == io::ErrorKind::NotFound => match options.mode {
            ParseMode::Strict => Err(ChaninpParseError::ChnE009 {
                path: path.to_path_buf(),
            }),
            ParseMode::Compatibility => Ok(defaulted_output(
                options,
                ChaninpParseOutcome::DefaultedCompat,
                ChaninpWarning::new(
                    ChaninpWarningCode::ChnW001,
                    None,
                    "compatibility fallback applied for missing chan.inp under ipeak>2",
                ),
            )),
        },
        Err(source) => match options.mode {
            ParseMode::Strict => Err(ChaninpParseError::ChnE000 {
                path: path.to_path_buf(),
                source,
            }),
            ParseMode::Compatibility => Ok(defaulted_output(
                options,
                ChaninpParseOutcome::OpenErrorCollapsedCompat,
                ChaninpWarning::new(
                    ChaninpWarningCode::ChnW002,
                    None,
                    format!(
                        "compatibility collapsed non-ENOENT open error into default branch ({source})"
                    ),
                ),
            )),
        },
    }
}

pub fn parse_chaninp_from_str(
    input: &str,
    options: ChaninpParseOptions,
    valid_channel_element_ids: &BTreeSet<i32>,
) -> Result<ChaninpFile, ChaninpParseError> {
    if options.ipeak <= 2 {
        return Ok(not_applicable_output(options));
    }

    let lines = collect_non_empty_lines(input);
    if let Some((line, token)) = detect_prefixed_variant(&lines) {
        return Err(ChaninpParseError::ChnE008 { line, token });
    }

    match parse_required_branch(&lines, options, valid_channel_element_ids) {
        Ok(parsed) => Ok(parsed),
        Err(
            error @ (ChaninpParseError::ChnE002 { field: "line4", .. }
            | ChaninpParseError::ChnE008 { .. }),
        ) => Err(error),
        Err(error) => match options.mode {
            ParseMode::Strict => Err(error),
            ParseMode::Compatibility => Ok(defaulted_output(
                options,
                ChaninpParseOutcome::DefaultedCompat,
                ChaninpWarning::new(
                    ChaninpWarningCode::ChnW003,
                    None,
                    format!(
                        "compatibility collapsed parse/count failure into default branch ({})",
                        error.contract_error_id()
                    ),
                ),
            )),
        },
    }
}

fn parse_required_branch(
    lines: &[(usize, &str)],
    options: ChaninpParseOptions,
    valid_channel_element_ids: &BTreeSet<i32>,
) -> Result<ChaninpFile, ChaninpParseError> {
    if lines.len() < 3 {
        return Err(ChaninpParseError::ChnE002 {
            line: lines.last().map_or(1, |entry| entry.0),
            field: "file",
            expected: 3,
            found: lines.len(),
        });
    }

    let mut trailing_token_lines = Vec::new();
    let (line1_no, line1) = lines[0];
    let line1_tokens = tokenize(line1);
    if line1_tokens.len() < 2 {
        return Err(ChaninpParseError::ChnE002 {
            line: line1_no,
            field: "line1",
            expected: 2,
            found: line1_tokens.len(),
        });
    }
    if line1_tokens.len() > 2 {
        trailing_token_lines.push(line1_no);
    }

    let (line2_no, line2) = lines[1];
    let line2_tokens = tokenize(line2);
    if line2_tokens.is_empty() {
        return Err(ChaninpParseError::ChnE002 {
            line: line2_no,
            field: "line2",
            expected: 1,
            found: 0,
        });
    }
    if line2_tokens.len() > 1 {
        trailing_token_lines.push(line2_no);
    }

    let (line3_no, line3) = lines[2];
    let line3_tokens = tokenize(line3);
    if line3_tokens.is_empty() {
        return Err(ChaninpParseError::ChnE002 {
            line: line3_no,
            field: "line3",
            expected: 1,
            found: 0,
        });
    }
    if line3_tokens.len() > 1 {
        trailing_token_lines.push(line3_no);
    }

    let ichout_raw = parse_i32(line1_tokens[0], line1_no, "ichout")?;
    let dtchr_input_s_raw = parse_f64(line1_tokens[1], line1_no, "dtchr")?;
    let cbase_raw = parse_f64(line2_tokens[0], line2_no, "cbase")?;
    let nchnum_input_raw = parse_i32(line3_tokens[0], line3_no, "nchnum")?;

    let mut warnings = Vec::new();
    let ichout = normalize_ichout(ichout_raw, options.mode, line1_no, &mut warnings)?;
    let cbase = normalize_cbase(cbase_raw, options.mode, line2_no, &mut warnings)?;
    let (dtchr_input_s, dtchr_norm_s, ntchr) =
        normalize_dtchr(dtchr_input_s_raw, options, line1_no, &mut warnings)?;

    let raw_nonnegative_count = usize::try_from(nchnum_input_raw).ok();
    let expected_record_count = match raw_nonnegative_count {
        Some(0) | None => 3,
        Some(_) => 4,
    };
    if lines.len() != expected_record_count {
        if let Some(expected) = raw_nonnegative_count {
            if expected > 0 && lines.len() < 4 {
                return Err(ChaninpParseError::ChnE002 {
                    line: line3_no + 1,
                    field: "line4",
                    expected,
                    found: 0,
                });
            }
            if expected == 0 && lines.len() == 4 {
                return Err(ChaninpParseError::ChnE002 {
                    line: lines[3].0,
                    field: "line4",
                    expected: 0,
                    found: tokenize(lines[3].1).len(),
                });
            }
        }
        let line = lines
            .get(expected_record_count)
            .or_else(|| lines.last())
            .map_or(line3_no, |entry| entry.0);
        return Err(ChaninpParseError::ChnE002 {
            line,
            field: "file",
            expected: expected_record_count,
            found: lines.len(),
        });
    }

    let (line4_no, ichnum_input) = if raw_nonnegative_count.is_none_or(|count| count == 0) {
        (line3_no, Vec::new())
    } else {
        let (line_no, line) = lines[3];
        let tokens = tokenize(line);
        if raw_nonnegative_count != Some(tokens.len()) {
            return Err(ChaninpParseError::ChnE002 {
                line: line_no,
                field: "line4",
                expected: raw_nonnegative_count.unwrap_or(0),
                found: tokens.len(),
            });
        }
        (line_no, parse_ichnum_tokens(line_no, &tokens)?)
    };

    if let Some(expected) = raw_nonnegative_count {
        if expected != ichnum_input.len() {
            return Err(ChaninpParseError::ChnE002 {
                line: line4_no,
                field: "line4",
                expected,
                found: ichnum_input.len(),
            });
        }
    }

    let (nchnum_input, nchnum_norm) =
        normalize_nchnum(nchnum_input_raw, options, line3_no, &mut warnings)?;
    let normalized_count =
        usize::try_from(nchnum_norm).map_err(|_| ChaninpParseError::ChnE007 {
            context: "nchnum_norm must convert to a non-negative list cardinality",
        })?;

    let mut unknown_ichnum_retained_warning_emitted = false;
    let mut ichnum_norm = Vec::new();
    for (index, id) in ichnum_input.iter().enumerate() {
        if !valid_channel_element_ids.contains(id) {
            match options.mode {
                ParseMode::Strict => {
                    return Err(ChaninpParseError::ChnE005 {
                        line: line4_no,
                        field: "ichnum",
                        value: *id,
                        detail: "channel id is not present in valid topology set",
                    });
                }
                ParseMode::Compatibility => {
                    unknown_ichnum_retained_warning_emitted = true;
                    if !warnings
                        .iter()
                        .any(|warning| warning.code == ChaninpWarningCode::ChnW005)
                    {
                        warnings.push(ChaninpWarning::new(
                            ChaninpWarningCode::ChnW005,
                            Some(line4_no),
                            "compatibility retained unknown ichnum id outside topology closure",
                        ));
                    }
                }
            }
        }
        if index < normalized_count {
            ichnum_norm.push(*id);
        }
    }

    let chan_output_enabled = ichout > 0 && nchnum_norm > 0;
    if chan_output_enabled != (ichout > 0 && !ichnum_norm.is_empty()) {
        return Err(ChaninpParseError::ChnE007 {
            context: "chan_output_enabled closure mismatch against normalized mode/count",
        });
    }

    if ntchr <= 0 || dtchr_norm_s <= 0 {
        return Err(ChaninpParseError::ChnE006 {
            dtchr_norm_s,
            ntchr,
            context: "normalized timestep must remain positive",
        });
    }

    if usize::try_from(nchnum_norm).ok() != Some(ichnum_norm.len()) {
        return Err(ChaninpParseError::ChnE007 {
            context: "nchnum_norm cardinality must equal normalized ichnum list length",
        });
    }

    Ok(ChaninpFile {
        chaninp_required: true,
        parse_outcome: ChaninpParseOutcome::ParsedBranch,
        ipeak: options.ipeak,
        nchan: options.nchan,
        line_count_closed: true,
        trailing_token_lines,
        unknown_ichnum_retained_warning_emitted,
        warnings,
        options: Some(ChaninpOptions {
            ichout,
            dtchr_input_s,
            cbase_m3_s_m2: cbase,
            nchnum_input,
            ichnum_input,
            dtchr_norm_s,
            ntchr,
            nchnum_norm,
            ichnum_norm,
            chan_output_enabled,
        }),
    })
}

fn normalize_ichout(
    value: i32,
    mode: ParseMode,
    line_no: usize,
    warnings: &mut Vec<ChaninpWarning>,
) -> Result<i32, ChaninpParseError> {
    let canonical_domain = 0..=3;

    if mode == ParseMode::Strict {
        if !canonical_domain.contains(&value) {
            return Err(ChaninpParseError::ChnE004 {
                line: line_no,
                field: "ichout",
                value: f64::from(value),
                rule: "strict domain requires 0..3",
            });
        }
        return Ok(value);
    }

    // W4DR-004 ratifies interoperability normalization toward the writer subset {1,3}.
    let normalized = match value {
        0 | 1 => 1,
        2 | 3 => 3,
        v if v < 0 => 1,
        _ => 3,
    };

    if normalized != value {
        warnings.push(ChaninpWarning::new(
            ChaninpWarningCode::ChnW004,
            Some(line_no),
            format!(
                "compatibility normalized ichout {value} to {normalized} for writer-interoperable domain {{1,3}}"
            ),
        ));
    }

    Ok(normalized)
}

fn normalize_cbase(
    value: f64,
    mode: ParseMode,
    line_no: usize,
    warnings: &mut Vec<ChaninpWarning>,
) -> Result<f64, ChaninpParseError> {
    if !value.is_finite() {
        return Err(ChaninpParseError::ChnE003 {
            line: line_no,
            field: "cbase",
            token: value.to_string(),
        });
    }

    if mode == ParseMode::Strict {
        if value < 0.0 {
            return Err(ChaninpParseError::ChnE004 {
                line: line_no,
                field: "cbase",
                value,
                rule: "cbase must be non-negative",
            });
        }
        return Ok(value);
    }

    if value < 0.0 {
        warnings.push(ChaninpWarning::new(
            ChaninpWarningCode::ChnW004,
            Some(line_no),
            format!(
                "compatibility clamped negative cbase {value} to 0.0 while preserving cbase namespace semantics"
            ),
        ));
        return Ok(0.0);
    }

    Ok(value)
}

#[allow(clippy::cast_possible_truncation)]
fn normalize_dtchr(
    value: f64,
    options: ChaninpParseOptions,
    line_no: usize,
    warnings: &mut Vec<ChaninpWarning>,
) -> Result<(f64, i32, i32), ChaninpParseError> {
    if !value.is_finite() {
        return Err(ChaninpParseError::ChnE003 {
            line: line_no,
            field: "dtchr",
            token: value.to_string(),
        });
    }

    let lower = f64::from(options.dtchr_lower_bound_s);
    let upper = f64::from(options.dtchr_upper_bound_s);

    let adjusted = if options.mode == ParseMode::Strict {
        if value < lower || value > upper {
            return Err(ChaninpParseError::ChnE004 {
                line: line_no,
                field: "dtchr",
                value,
                rule: "strict dtchr must satisfy lower<=dtchr<=upper",
            });
        }
        value
    } else {
        let mut adjusted = value;
        if adjusted < lower {
            adjusted = lower;
        }
        if adjusted > upper {
            adjusted = upper;
        }
        if (adjusted - value).abs() > f64::EPSILON {
            warnings.push(ChaninpWarning::new(
                ChaninpWarningCode::ChnW004,
                Some(line_no),
                format!("compatibility normalized dtchr from {value} to bounded value {adjusted}"),
            ));
        }
        adjusted
    };

    let mut ntchr_f = (SECONDS_PER_DAY / adjusted) + 0.99;
    if !ntchr_f.is_finite() {
        return Err(ChaninpParseError::ChnE006 {
            dtchr_norm_s: 0,
            ntchr: 0,
            context: "ntchr computation produced non-finite result",
        });
    }

    if ntchr_f < 1.0 {
        ntchr_f = 1.0;
    }

    let mut ntchr = ntchr_f.floor() as i32;
    if ntchr > options.mxtchr {
        if options.mode == ParseMode::Compatibility {
            warnings.push(ChaninpWarning::new(
                ChaninpWarningCode::ChnW004,
                Some(line_no),
                format!(
                    "compatibility capped ntchr {ntchr} to mxtchr {}",
                    options.mxtchr
                ),
            ));
        }
        ntchr = options.mxtchr;
    }

    if ntchr <= 0 || ntchr > options.mxtchr {
        return Err(ChaninpParseError::ChnE006 {
            dtchr_norm_s: 0,
            ntchr,
            context: "ntchr must satisfy 1..=mxtchr",
        });
    }

    let dtchr_norm_f = SECONDS_PER_DAY / f64::from(ntchr);
    let dtchr_norm_s = dtchr_norm_f.round() as i32;

    if dtchr_norm_s <= 0 {
        return Err(ChaninpParseError::ChnE006 {
            dtchr_norm_s,
            ntchr,
            context: "rounded dtchr_norm_s must remain positive",
        });
    }

    Ok((adjusted, dtchr_norm_s, ntchr))
}

fn normalize_nchnum(
    value: i32,
    options: ChaninpParseOptions,
    line_no: usize,
    warnings: &mut Vec<ChaninpWarning>,
) -> Result<(i32, i32), ChaninpParseError> {
    if options.mode == ParseMode::Strict {
        if value < 0 {
            return Err(ChaninpParseError::ChnE004 {
                line: line_no,
                field: "nchnum",
                value: f64::from(value),
                rule: "strict nchnum must be non-negative",
            });
        }
        let nchan_i32 = i32::try_from(options.nchan).map_err(|_| ChaninpParseError::ChnE007 {
            context: "nchan exceeds i32 conversion bounds",
        })?;
        if value > nchan_i32 {
            return Err(ChaninpParseError::ChnE005 {
                line: line_no,
                field: "nchnum",
                value,
                detail: "nchnum exceeds topology channel count",
            });
        }
        return Ok((value, value));
    }

    let nchan_i32 = i32::try_from(options.nchan).map_err(|_| ChaninpParseError::ChnE007 {
        context: "nchan exceeds i32 conversion bounds",
    })?;
    let mut normalized = value;
    if normalized < 0 {
        normalized = 0;
    }
    if normalized > nchan_i32 {
        normalized = nchan_i32;
    }

    if normalized != value {
        warnings.push(ChaninpWarning::new(
            ChaninpWarningCode::ChnW004,
            Some(line_no),
            format!(
                "compatibility clamped nchnum from {value} to {normalized} within topology bounds"
            ),
        ));
    }

    Ok((value, normalized))
}

fn parse_ichnum_tokens(line_no: usize, tokens: &[&str]) -> Result<Vec<i32>, ChaninpParseError> {
    let mut parsed = Vec::with_capacity(tokens.len());
    for token in tokens {
        parsed.push(parse_i32(token, line_no, "ichnum")?);
    }
    Ok(parsed)
}

fn parse_i32(token: &str, line: usize, field: &'static str) -> Result<i32, ChaninpParseError> {
    token
        .parse::<i32>()
        .map_err(|_| ChaninpParseError::ChnE001 {
            line,
            field,
            token: token.to_string(),
        })
}

fn parse_f64(token: &str, line: usize, field: &'static str) -> Result<f64, ChaninpParseError> {
    let parsed = token
        .parse::<f64>()
        .map_err(|_| ChaninpParseError::ChnE001 {
            line,
            field,
            token: token.to_string(),
        })?;
    if !parsed.is_finite() {
        return Err(ChaninpParseError::ChnE003 {
            line,
            field,
            token: token.to_string(),
        });
    }
    Ok(parsed)
}

fn detect_prefixed_variant(lines: &[(usize, &str)]) -> Option<(usize, String)> {
    let (line_no, line) = *lines.first()?;
    let tokens = tokenize(line);
    if tokens.len() != 1 {
        return None;
    }

    let token = tokens[0];
    let lowered = token.to_ascii_lowercase();
    let looks_like_numeric_datver = token.parse::<f64>().is_ok();
    let looks_like_prefix = lowered.contains("datver")
        || lowered.contains("version")
        || lowered.starts_with('v')
            && lowered
                .chars()
                .skip(1)
                .all(|c| c.is_ascii_digit() || c == '.');

    if looks_like_numeric_datver || looks_like_prefix {
        Some((line_no, token.to_string()))
    } else {
        None
    }
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

fn not_applicable_output(options: ChaninpParseOptions) -> ChaninpFile {
    ChaninpFile {
        chaninp_required: false,
        parse_outcome: ChaninpParseOutcome::NotApplicable,
        ipeak: options.ipeak,
        nchan: options.nchan,
        line_count_closed: true,
        trailing_token_lines: Vec::new(),
        unknown_ichnum_retained_warning_emitted: false,
        warnings: Vec::new(),
        options: None,
    }
}

#[allow(clippy::cast_possible_truncation)]
fn defaulted_output(
    options: ChaninpParseOptions,
    parse_outcome: ChaninpParseOutcome,
    warning: ChaninpWarning,
) -> ChaninpFile {
    let dtchr_input_s = f64::from(options.dtchr_lower_bound_s);
    let ntchr = options.mxtchr;
    let dtchr_norm_s = (SECONDS_PER_DAY / f64::from(ntchr)).round() as i32;

    ChaninpFile {
        chaninp_required: true,
        parse_outcome,
        ipeak: options.ipeak,
        nchan: options.nchan,
        line_count_closed: false,
        trailing_token_lines: Vec::new(),
        unknown_ichnum_retained_warning_emitted: false,
        warnings: vec![warning],
        options: Some(ChaninpOptions {
            ichout: 0,
            dtchr_input_s,
            cbase_m3_s_m2: 0.0,
            nchnum_input: 0,
            ichnum_input: Vec::new(),
            dtchr_norm_s,
            ntchr,
            nchnum_norm: 0,
            ichnum_norm: Vec::new(),
            chan_output_enabled: false,
        }),
    }
}
