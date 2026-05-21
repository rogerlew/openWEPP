#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::too_many_lines
)]

use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const CANONICAL_DATVER: f64 = 99.1;
const COMPAT_MIN_DATVER: f64 = 94.301;
const FLOAT_TOLERANCE: f64 = 1e-9;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatershedChannelParseMode {
    Strict,
    Compatibility,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WatershedChannelParseOptions {
    pub mode: WatershedChannelParseMode,
    pub expected_channel_count: Option<usize>,
    pub chan_inp_present: bool,
    pub tcr_overlay_present: bool,
    pub slplst_override: Option<f64>,
}

impl Default for WatershedChannelParseOptions {
    fn default() -> Self {
        Self {
            mode: WatershedChannelParseMode::Strict,
            expected_channel_count: None,
            chan_inp_present: true,
            tcr_overlay_present: false,
            slplst_override: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelWarningCode {
    ChnW001,
    ChnW002,
    ChnW003,
    ChnW004,
    ChnW005,
}

impl ChannelWarningCode {
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

impl fmt::Display for ChannelWarningCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChannelWarning {
    pub code: ChannelWarningCode,
    pub line: Option<usize>,
    pub message: String,
}

impl ChannelWarning {
    fn new(code: ChannelWarningCode, line: Option<usize>, message: impl Into<String>) -> Self {
        Self {
            code,
            line,
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChannelRatingCurve {
    pub rccoef: f64,
    pub rcexp: f64,
    pub rcoset: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChannelDefinition {
    pub channel_id: usize,
    pub comment_1: String,
    pub comment_2: String,
    pub comment_3: String,
    pub ishape: i32,
    pub icntrl: i32,
    pub ienslp: i32,
    pub flgout: i32,
    pub chnz: f64,
    pub chnnbr: f64,
    pub chnn: f64,
    pub chnk: f64,
    pub chntcr: f64,
    pub chnedm: f64,
    pub chneds: f64,
    pub ctlslp_input: f64,
    pub ctlz_input: f64,
    pub ctln_input: f64,
    pub ctlslp_effective: f64,
    pub ctlz_effective: f64,
    pub ctln_effective: f64,
    pub has_rating_curve: bool,
    pub rating_curve: Option<ChannelRatingCurve>,
    pub control_override_applied: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WatershedChannelFile {
    pub datver: f64,
    pub nchan: usize,
    pub ipeak: i32,
    pub lw: f64,
    pub sidecar_required: bool,
    pub tcr_overlay_present: bool,
    pub channels: Vec<ChannelDefinition>,
    pub warnings: Vec<ChannelWarning>,
}

#[derive(Debug)]
pub enum WatershedChannelParseError {
    Io {
        path: PathBuf,
        source: io::Error,
    },
    TokenParse {
        line: usize,
        field: &'static str,
        token: String,
    },
    RecordClosure {
        context: &'static str,
        expected: usize,
        found: usize,
    },
    UnsupportedDatver {
        line: usize,
        value: f64,
    },
    EnumDomain {
        line: usize,
        field: &'static str,
        value: i32,
    },
    FieldRange {
        line: usize,
        field: &'static str,
        value: f64,
        rule: &'static str,
    },
    RatingCurveClosure {
        line: usize,
        channel_id: usize,
        reason: &'static str,
    },
    ChannelCountMismatch {
        declared: usize,
        expected: usize,
    },
    RequiredSidecarMissing {
        sidecar: &'static str,
        ipeak: i32,
    },
    InvariantViolation {
        line: usize,
        context: &'static str,
    },
}

impl WatershedChannelParseError {
    #[must_use]
    pub const fn contract_error_id(&self) -> &'static str {
        match self {
            Self::Io { .. } => "CHN-E-000",
            Self::TokenParse { .. } => "CHN-E-001",
            Self::RecordClosure { .. } => "CHN-E-002",
            Self::UnsupportedDatver { .. } => "CHN-E-003",
            Self::EnumDomain { .. } => "CHN-E-004",
            Self::FieldRange { .. } => "CHN-E-005",
            Self::RatingCurveClosure { .. } => "CHN-E-006",
            Self::ChannelCountMismatch { .. } => "CHN-E-007",
            Self::RequiredSidecarMissing { .. } => "CHN-E-008",
            Self::InvariantViolation { .. } => "CHN-E-009",
        }
    }
}

impl fmt::Display for WatershedChannelParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, source } => write!(
                f,
                "{}: failed to open/read '{}': {source}",
                self.contract_error_id(),
                path.display()
            ),
            Self::TokenParse { line, field, token } => write!(
                f,
                "{}: line {line} failed to parse field '{field}' from token '{token}'",
                self.contract_error_id()
            ),
            Self::RecordClosure {
                context,
                expected,
                found,
            } => write!(
                f,
                "{}: record closure error in {context}; expected {expected} line(s), found {found}",
                self.contract_error_id()
            ),
            Self::UnsupportedDatver { line, value } => write!(
                f,
                "{}: line {line} unsupported datver '{value}'",
                self.contract_error_id()
            ),
            Self::EnumDomain { line, field, value } => write!(
                f,
                "{}: line {line} enum-domain violation for '{field}' with value {value}",
                self.contract_error_id()
            ),
            Self::FieldRange {
                line,
                field,
                value,
                rule,
            } => write!(
                f,
                "{}: line {line} field '{field}' value {value} violates {rule}",
                self.contract_error_id()
            ),
            Self::RatingCurveClosure {
                line,
                channel_id,
                reason,
            } => write!(
                f,
                "{}: line {line} channel {channel_id} rating-curve closure failure: {reason}",
                self.contract_error_id()
            ),
            Self::ChannelCountMismatch { declared, expected } => write!(
                f,
                "{}: channel count mismatch declared={declared} expected={expected}",
                self.contract_error_id()
            ),
            Self::RequiredSidecarMissing { sidecar, ipeak } => write!(
                f,
                "{}: ipeak={ipeak} requires sidecar '{}'",
                self.contract_error_id(),
                sidecar
            ),
            Self::InvariantViolation { line, context } => write!(
                f,
                "{}: line {line} invariant violation: {context}",
                self.contract_error_id()
            ),
        }
    }
}

impl std::error::Error for WatershedChannelParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}

pub fn parse_watershed_channel_from_path(
    path: impl AsRef<Path>,
    options: WatershedChannelParseOptions,
) -> Result<WatershedChannelFile, WatershedChannelParseError> {
    let path = path.as_ref();
    let content = fs::read_to_string(path).map_err(|source| WatershedChannelParseError::Io {
        path: path.to_path_buf(),
        source,
    })?;

    parse_watershed_channel_from_str(&content, options)
}

pub fn parse_watershed_channel_from_str(
    input: &str,
    options: WatershedChannelParseOptions,
) -> Result<WatershedChannelFile, WatershedChannelParseError> {
    let lines: Vec<&str> = input.lines().collect();
    let mut cursor = 0usize;
    let mut warnings = Vec::new();

    let (datver_line, datver) = parse_single_f64(&lines, &mut cursor, "ver")?;
    validate_datver(datver_line, datver, options.mode, &mut warnings)?;

    let (_, nchan_raw) = parse_single_i32(&lines, &mut cursor, "nchan")?;
    if nchan_raw <= 0 {
        return Err(WatershedChannelParseError::FieldRange {
            line: 2,
            field: "nchan",
            value: f64::from(nchan_raw),
            rule: "> 0",
        });
    }
    let nchan = nchan_raw as usize;

    if let Some(expected) = options.expected_channel_count {
        if expected != nchan {
            return Err(WatershedChannelParseError::ChannelCountMismatch {
                declared: nchan,
                expected,
            });
        }
    }

    let (_, ipeak) = parse_single_i32(&lines, &mut cursor, "ipeak")?;
    if !(1..=5).contains(&ipeak) {
        return Err(WatershedChannelParseError::EnumDomain {
            line: 3,
            field: "ipeak",
            value: ipeak,
        });
    }

    let (_, lw) = parse_single_f64(&lines, &mut cursor, "lw")?;
    ensure_positive(lw, 4, "lw")?;

    let sidecar_required = ipeak > 2;
    if sidecar_required && !options.chan_inp_present {
        if options.mode == WatershedChannelParseMode::Strict {
            return Err(WatershedChannelParseError::RequiredSidecarMissing {
                sidecar: "chan.inp",
                ipeak,
            });
        }
        warnings.push(ChannelWarning::new(
            ChannelWarningCode::ChnW002,
            Some(3),
            "compatibility fallback used for missing chan.inp under ipeak>2",
        ));
    }

    let mut channels = Vec::with_capacity(nchan);
    for channel_idx in 0..nchan {
        let channel_id = channel_idx + 1;
        let comment_1 = read_comment_line(&lines, &mut cursor, "comment_1")?
            .1
            .to_string();
        let comment_2 = read_comment_line(&lines, &mut cursor, "comment_2")?
            .1
            .to_string();
        let comment_3 = read_comment_line(&lines, &mut cursor, "comment_3")?
            .1
            .to_string();

        let (ishape_line, mut ishape) = parse_single_i32(&lines, &mut cursor, "ishape")?;
        match options.mode {
            WatershedChannelParseMode::Strict => {
                if !matches!(ishape, 1 | 2) {
                    return Err(WatershedChannelParseError::EnumDomain {
                        line: ishape_line,
                        field: "ishape",
                        value: ishape,
                    });
                }
            }
            WatershedChannelParseMode::Compatibility => {
                if ishape < 1 {
                    return Err(WatershedChannelParseError::EnumDomain {
                        line: ishape_line,
                        field: "ishape",
                        value: ishape,
                    });
                }
                if ishape > 2 {
                    ishape = 2;
                    warnings.push(ChannelWarning::new(
                        ChannelWarningCode::ChnW003,
                        Some(ishape_line),
                        "compatibility normalized legacy ishape value to naturally eroded class",
                    ));
                }
            }
        }

        let (icntrl_line, icntrl) = parse_single_i32(&lines, &mut cursor, "icntrl")?;
        if !(0..=4).contains(&icntrl) {
            return Err(WatershedChannelParseError::EnumDomain {
                line: icntrl_line,
                field: "icntrl",
                value: icntrl,
            });
        }

        let (ienslp_line, ienslp) = parse_single_i32(&lines, &mut cursor, "ienslp")?;
        if !(1..=2).contains(&ienslp) {
            return Err(WatershedChannelParseError::EnumDomain {
                line: ienslp_line,
                field: "ienslp",
                value: ienslp,
            });
        }

        let (flgout_line, flgout) = parse_single_i32(&lines, &mut cursor, "flgout")?;
        if !(0..=1).contains(&flgout) {
            return Err(WatershedChannelParseError::EnumDomain {
                line: flgout_line,
                field: "flgout",
                value: flgout,
            });
        }

        let (geom_line, geom) = parse_fixed_f64_tuple::<2>(&lines, &mut cursor, "geom_line")?;
        let chnz = geom[0];
        let chnnbr = geom[1];
        ensure_positive(chnz, geom_line, "chnz")?;
        ensure_positive(chnnbr, geom_line, "chnnbr")?;

        let (erod_line, erod) = parse_fixed_f64_tuple::<5>(&lines, &mut cursor, "erod_line")?;
        let chnn = erod[0];
        let chnk = erod[1];
        let chntcr = erod[2];
        let chnedm = erod[3];
        let chneds = erod[4];
        ensure_positive(chnn, erod_line, "chnn")?;
        ensure_non_negative(chnk, erod_line, "chnk")?;
        ensure_non_negative(chntcr, erod_line, "chntcr")?;
        ensure_non_negative(chnedm, erod_line, "chnedm")?;
        ensure_non_negative(chneds, erod_line, "chneds")?;

        if chnn + FLOAT_TOLERANCE < chnnbr {
            return Err(WatershedChannelParseError::FieldRange {
                line: erod_line,
                field: "chnn",
                value: chnn,
                rule: ">= chnnbr",
            });
        }

        let (control_line, control) =
            parse_fixed_f64_tuple::<3>(&lines, &mut cursor, "control_line")?;
        let ctlslp_input = control[0];
        let ctlz_input = control[1];
        let ctln_input = control[2];
        ensure_non_negative(ctlslp_input, control_line, "ctlslp")?;
        ensure_positive(ctlz_input, control_line, "ctlz")?;
        ensure_positive(ctln_input, control_line, "ctln")?;

        let rating_curve = if icntrl == 4 {
            let (rating_line, rating_line_text) =
                next_line(&lines, &mut cursor, "rating_curve_line").map_err(|_| {
                    WatershedChannelParseError::RatingCurveClosure {
                        line: cursor,
                        channel_id,
                        reason: "icntrl==4 requires rating_curve_line",
                    }
                })?;
            let rating_tokens: Vec<&str> = rating_line_text.split_whitespace().collect();
            if rating_tokens.len() != 3 {
                return Err(WatershedChannelParseError::RatingCurveClosure {
                    line: rating_line,
                    channel_id,
                    reason: "rating_curve_line must contain exactly 3 tokens",
                });
            }

            let rccoef = parse_f64_token(rating_line, "rccoef", rating_tokens[0])?;
            let rcexp = parse_f64_token(rating_line, "rcexp", rating_tokens[1])?;
            let rcoset = parse_f64_token(rating_line, "rcoset", rating_tokens[2])?;
            ensure_positive(rccoef, rating_line, "rccoef")?;
            ensure_positive(rcexp, rating_line, "rcexp")?;
            ensure_non_negative(rcoset, rating_line, "rcoset")?;
            Some(ChannelRatingCurve {
                rccoef,
                rcexp,
                rcoset,
            })
        } else {
            None
        };

        let control_override_applied = icntrl == 0;
        let (ctlslp_effective, ctlz_effective, ctln_effective) = if control_override_applied {
            if options.mode == WatershedChannelParseMode::Strict
                && options.slplst_override.is_none()
            {
                return Err(WatershedChannelParseError::InvariantViolation {
                    line: control_line,
                    context: "icntrl==0 requires slplst_override for strict closure",
                });
            }

            let effective_slope = options.slplst_override.unwrap_or(ctlslp_input);
            if options.mode == WatershedChannelParseMode::Compatibility {
                warnings.push(ChannelWarning::new(
                    ChannelWarningCode::ChnW004,
                    Some(control_line),
                    "compatibility applied icntrl=0 control override precedence",
                ));
            }
            (effective_slope, chnz, chnn)
        } else {
            (ctlslp_input, ctlz_input, ctln_input)
        };

        channels.push(ChannelDefinition {
            channel_id,
            comment_1,
            comment_2,
            comment_3,
            ishape,
            icntrl,
            ienslp,
            flgout,
            chnz,
            chnnbr,
            chnn,
            chnk,
            chntcr,
            chnedm,
            chneds,
            ctlslp_input,
            ctlz_input,
            ctln_input,
            ctlslp_effective,
            ctlz_effective,
            ctln_effective,
            has_rating_curve: icntrl == 4,
            rating_curve,
            control_override_applied,
        });
    }

    if cursor < lines.len() {
        let remaining_non_empty = lines[cursor..]
            .iter()
            .filter(|line| !line.trim().is_empty())
            .count();
        if remaining_non_empty > 0 {
            return Err(WatershedChannelParseError::RecordClosure {
                context: "extra_records",
                expected: cursor,
                found: lines.len(),
            });
        }
    }

    if options.tcr_overlay_present {
        warnings.push(ChannelWarning::new(
            ChannelWarningCode::ChnW005,
            None,
            "tcr overlay marker present; overlay must remain non-mutating",
        ));
    }

    Ok(WatershedChannelFile {
        datver,
        nchan,
        ipeak,
        lw,
        sidecar_required,
        tcr_overlay_present: options.tcr_overlay_present,
        channels,
        warnings,
    })
}

fn read_comment_line<'a>(
    lines: &'a [&'a str],
    cursor: &mut usize,
    context: &'static str,
) -> Result<(usize, &'a str), WatershedChannelParseError> {
    let line = next_line(lines, cursor, context)?;
    Ok((line.0, line.1))
}

fn parse_single_i32(
    lines: &[&str],
    cursor: &mut usize,
    field: &'static str,
) -> Result<(usize, i32), WatershedChannelParseError> {
    let (line_no, line) = next_line(lines, cursor, field)?;
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.len() != 1 {
        return Err(WatershedChannelParseError::TokenParse {
            line: line_no,
            field,
            token: line.to_string(),
        });
    }

    let value = tokens[0]
        .parse::<i32>()
        .map_err(|_| WatershedChannelParseError::TokenParse {
            line: line_no,
            field,
            token: tokens[0].to_string(),
        })?;
    Ok((line_no, value))
}

fn parse_single_f64(
    lines: &[&str],
    cursor: &mut usize,
    field: &'static str,
) -> Result<(usize, f64), WatershedChannelParseError> {
    let (line_no, line) = next_line(lines, cursor, field)?;
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.len() != 1 {
        return Err(WatershedChannelParseError::TokenParse {
            line: line_no,
            field,
            token: line.to_string(),
        });
    }

    let value = parse_f64_token(line_no, field, tokens[0])?;
    Ok((line_no, value))
}

fn parse_fixed_f64_tuple<const N: usize>(
    lines: &[&str],
    cursor: &mut usize,
    field: &'static str,
) -> Result<(usize, [f64; N]), WatershedChannelParseError> {
    let (line_no, line) = next_line(lines, cursor, field)?;
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.len() != N {
        return Err(WatershedChannelParseError::TokenParse {
            line: line_no,
            field,
            token: line.to_string(),
        });
    }

    let mut values = [0.0_f64; N];
    for (idx, token) in tokens.iter().enumerate() {
        values[idx] = parse_f64_token(line_no, field, token)?;
    }
    Ok((line_no, values))
}

fn next_line<'a>(
    lines: &'a [&'a str],
    cursor: &mut usize,
    context: &'static str,
) -> Result<(usize, &'a str), WatershedChannelParseError> {
    let index = *cursor;
    let Some(line) = lines.get(index) else {
        return Err(WatershedChannelParseError::RecordClosure {
            context,
            expected: index + 1,
            found: lines.len(),
        });
    };
    *cursor += 1;
    Ok((index + 1, line.trim()))
}

fn parse_f64_token(
    line: usize,
    field: &'static str,
    token: &str,
) -> Result<f64, WatershedChannelParseError> {
    let value = token
        .parse::<f64>()
        .map_err(|_| WatershedChannelParseError::TokenParse {
            line,
            field,
            token: token.to_string(),
        })?;

    if !value.is_finite() {
        return Err(WatershedChannelParseError::FieldRange {
            line,
            field,
            value,
            rule: "finite",
        });
    }

    Ok(value)
}

fn ensure_positive(
    value: f64,
    line: usize,
    field: &'static str,
) -> Result<(), WatershedChannelParseError> {
    if value > 0.0 {
        return Ok(());
    }

    Err(WatershedChannelParseError::FieldRange {
        line,
        field,
        value,
        rule: "> 0",
    })
}

fn ensure_non_negative(
    value: f64,
    line: usize,
    field: &'static str,
) -> Result<(), WatershedChannelParseError> {
    if value >= 0.0 {
        return Ok(());
    }

    Err(WatershedChannelParseError::FieldRange {
        line,
        field,
        value,
        rule: ">= 0",
    })
}

fn validate_datver(
    line: usize,
    datver: f64,
    mode: WatershedChannelParseMode,
    warnings: &mut Vec<ChannelWarning>,
) -> Result<(), WatershedChannelParseError> {
    if (datver - CANONICAL_DATVER).abs() <= FLOAT_TOLERANCE {
        return Ok(());
    }

    match mode {
        WatershedChannelParseMode::Strict => Err(WatershedChannelParseError::UnsupportedDatver {
            line,
            value: datver,
        }),
        WatershedChannelParseMode::Compatibility => {
            if datver + FLOAT_TOLERANCE < COMPAT_MIN_DATVER {
                return Err(WatershedChannelParseError::UnsupportedDatver {
                    line,
                    value: datver,
                });
            }

            warnings.push(ChannelWarning::new(
                ChannelWarningCode::ChnW001,
                Some(line),
                format!(
                    "compatibility accepted legacy datver {datver}; canonical target is {CANONICAL_DATVER}"
                ),
            ));
            Ok(())
        }
    }
}
