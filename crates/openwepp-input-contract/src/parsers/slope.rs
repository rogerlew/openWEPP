use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

/// Canonical slope-file datver accepted in strict mode.
pub const SLOPE_CANONICAL_DATVER: f64 = 97.5;
/// Legacy compatibility floor from legacy `slpchk` threshold.
pub const SLOPE_COMPAT_MIN_DATVER: f64 = 91.5;
/// Absolute tolerance for closure checks.
pub const SLOPE_ABS_TOLERANCE: f64 = 1e-6;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlopeParserMode {
    Strict,
    Compatibility,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SlopeParserOptions {
    pub mode: SlopeParserMode,
    pub canonical_datver: f64,
    pub compatibility_min_datver: f64,
    pub abs_tolerance: f64,
}

impl SlopeParserOptions {
    pub const fn strict() -> Self {
        Self {
            mode: SlopeParserMode::Strict,
            canonical_datver: SLOPE_CANONICAL_DATVER,
            compatibility_min_datver: SLOPE_COMPAT_MIN_DATVER,
            abs_tolerance: SLOPE_ABS_TOLERANCE,
        }
    }

    pub const fn compatibility() -> Self {
        Self {
            mode: SlopeParserMode::Compatibility,
            canonical_datver: SLOPE_CANONICAL_DATVER,
            compatibility_min_datver: SLOPE_COMPAT_MIN_DATVER,
            abs_tolerance: SLOPE_ABS_TOLERANCE,
        }
    }
}

impl Default for SlopeParserOptions {
    fn default() -> Self {
        Self::strict()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatverSource {
    Header,
    LegacyCompatImputed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistanceMode {
    Absolute,
    Normalized,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SlopePoint {
    pub xinput: f64,
    pub slpinp: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SlopeOfe {
    pub index: usize,
    pub azm: f64,
    pub fwidth: f64,
    pub nslpts: usize,
    pub slplen: f64,
    pub distance_mode: DistanceMode,
    pub points: Vec<SlopePoint>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SlopeProfile {
    pub datver: f64,
    pub datver_source: DatverSource,
    pub ofe_count: usize,
    pub ofes: Vec<SlopeOfe>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SlopeParserError {
    InputFileMissing {
        path: PathBuf,
    },
    InputFileOpenError {
        path: PathBuf,
        message: String,
    },
    TokenParseError {
        line: usize,
        column: usize,
        token: String,
        expected: &'static str,
    },
    RecordCountError {
        context: String,
    },
    MissingDatverHeaderError,
    UnsupportedDatver {
        datver: f64,
        mode: SlopeParserMode,
        canonical_datver: f64,
        compatibility_min_datver: f64,
    },
    FieldRangeError {
        field: &'static str,
        value: f64,
        expected: &'static str,
        guard_id: &'static str,
        ofe_index: Option<usize>,
    },
    DistanceModeMixError {
        ofe_index: usize,
        message: String,
    },
    EndpointConstraintError {
        ofe_index: usize,
        message: String,
    },
    CrossOfeBoundaryError {
        left_ofe_index: usize,
        right_ofe_index: usize,
        left_terminal_slope: f64,
        right_initial_slope: f64,
        tolerance: f64,
    },
    InvariantViolation {
        guard_id: &'static str,
        message: String,
    },
}

impl fmt::Display for SlopeParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputFileMissing { path } => write!(f, "missing slope file: {}", path.display()),
            Self::InputFileOpenError { path, message } => {
                write!(f, "failed to open slope file {}: {message}", path.display())
            }
            Self::TokenParseError {
                line,
                column,
                token,
                expected,
            } => write!(
                f,
                "token parse error at line {line}, column {column}: expected {expected}, got '{token}'"
            ),
            Self::RecordCountError { context } => write!(f, "record count error: {context}"),
            Self::MissingDatverHeaderError => {
                write!(f, "missing required datver header in strict mode")
            }
            Self::UnsupportedDatver {
                datver,
                mode,
                canonical_datver,
                compatibility_min_datver,
            } => write!(
                f,
                "unsupported datver {datver} for mode {:?} (strict requires {canonical_datver}, compat min {compatibility_min_datver})",
                mode
            ),
            Self::FieldRangeError {
                field,
                value,
                expected,
                guard_id,
                ofe_index,
            } => {
                if let Some(idx) = ofe_index {
                    write!(
                        f,
                        "field range error [{guard_id}] for {field} in OFE {}: got {value}, expected {expected}",
                        idx + 1
                    )
                } else {
                    write!(
                        f,
                        "field range error [{guard_id}] for {field}: got {value}, expected {expected}"
                    )
                }
            }
            Self::DistanceModeMixError { ofe_index, message } => {
                write!(f, "distance mode mix in OFE {}: {message}", ofe_index + 1)
            }
            Self::EndpointConstraintError { ofe_index, message } => {
                write!(f, "endpoint constraint in OFE {}: {message}", ofe_index + 1)
            }
            Self::CrossOfeBoundaryError {
                left_ofe_index,
                right_ofe_index,
                left_terminal_slope,
                right_initial_slope,
                tolerance,
            } => write!(
                f,
                "cross-OFE boundary slope mismatch OFE {} -> OFE {} ({left_terminal_slope} vs {right_initial_slope}, tol {tolerance})",
                left_ofe_index + 1,
                right_ofe_index + 1
            ),
            Self::InvariantViolation { guard_id, message } => {
                write!(f, "invariant violation [{guard_id}]: {message}")
            }
        }
    }
}

impl std::error::Error for SlopeParserError {}

pub fn parse_slope_file(
    path: &Path,
    options: SlopeParserOptions,
) -> Result<SlopeProfile, SlopeParserError> {
    if !path.exists() {
        return Err(SlopeParserError::InputFileMissing {
            path: path.to_path_buf(),
        });
    }

    let contents = fs::read_to_string(path).map_err(|err| {
        if err.kind() == std::io::ErrorKind::NotFound {
            SlopeParserError::InputFileMissing {
                path: path.to_path_buf(),
            }
        } else {
            SlopeParserError::InputFileOpenError {
                path: path.to_path_buf(),
                message: err.to_string(),
            }
        }
    })?;

    parse_slope_str(&contents, options)
}

pub fn parse_slope_str(
    contents: &str,
    options: SlopeParserOptions,
) -> Result<SlopeProfile, SlopeParserError> {
    let tokens = tokenize(contents);
    let mut cursor = TokenCursor::new(tokens);

    if cursor.is_empty() {
        return Err(SlopeParserError::RecordCountError {
            context: "slope file contains no numeric records".to_string(),
        });
    }

    let first_token = cursor
        .next_token()
        .ok_or_else(|| SlopeParserError::RecordCountError {
            context: "missing first record".to_string(),
        })?;
    let first_value = parse_f64(first_token)?;

    let (datver, datver_source, ofe_count) = if first_value > 10.0 {
        validate_datver(first_value, options)?;
        let nelem = parse_count(
            cursor
                .next_token()
                .ok_or_else(|| SlopeParserError::RecordCountError {
                    context: "missing nelem after datver".to_string(),
                })?,
            "nelem",
            "G-SLP-002",
            None,
        )?;
        (first_value, DatverSource::Header, nelem)
    } else {
        if options.mode == SlopeParserMode::Strict {
            return Err(SlopeParserError::MissingDatverHeaderError);
        }

        let nelem = parse_count(first_token, "nelem", "G-SLP-002", None)?;
        (
            options.canonical_datver,
            DatverSource::LegacyCompatImputed,
            nelem,
        )
    };

    let mut ofes = Vec::with_capacity(ofe_count);

    for ofe_index in 0..ofe_count {
        let azm = parse_f64(cursor.next_required("missing azm", 2 * (ofe_count - ofe_index))?)?;

        let fwidth =
            parse_f64(cursor.next_required("missing fwidth", 2 * (ofe_count - ofe_index))?)?;
        if !fwidth.is_finite() || fwidth <= 0.0 {
            return Err(SlopeParserError::FieldRangeError {
                field: "fwidth",
                value: fwidth,
                expected: "> 0 and finite",
                guard_id: "G-SLP-003",
                ofe_index: Some(ofe_index),
            });
        }

        let nslpts = parse_count(
            cursor.next_required("missing nslpts", 2 * (ofe_count - ofe_index))?,
            "nslpts",
            "G-SLP-004",
            Some(ofe_index),
        )?;
        if nslpts < 2 {
            return Err(SlopeParserError::FieldRangeError {
                field: "nslpts",
                value: nslpts as f64,
                expected: ">= 2",
                guard_id: "G-SLP-004",
                ofe_index: Some(ofe_index),
            });
        }

        let slplen = parse_f64(cursor.next_required("missing slplen", 2 * nslpts)?)?;
        if !slplen.is_finite() || slplen <= 0.0 {
            return Err(SlopeParserError::FieldRangeError {
                field: "slplen",
                value: slplen,
                expected: "> 0 and finite",
                guard_id: "G-SLP-004",
                ofe_index: Some(ofe_index),
            });
        }

        let mut points = Vec::with_capacity(nslpts);
        for _ in 0..nslpts {
            let xinput = parse_f64(cursor.next_required("missing xinput", 2)?)?;
            let slpinp = parse_f64(cursor.next_required("missing slpinp", 1)?)?;

            if !xinput.is_finite() {
                return Err(SlopeParserError::FieldRangeError {
                    field: "xinput",
                    value: xinput,
                    expected: "finite",
                    guard_id: "G-SLP-006",
                    ofe_index: Some(ofe_index),
                });
            }

            if !slpinp.is_finite() {
                return Err(SlopeParserError::FieldRangeError {
                    field: "slpinp",
                    value: slpinp,
                    expected: "finite",
                    guard_id: "G-SLP-006",
                    ofe_index: Some(ofe_index),
                });
            }

            points.push(SlopePoint { xinput, slpinp });
        }

        let distance_mode =
            derive_distance_mode(ofe_index, slplen, &points, options.abs_tolerance)?;

        ofes.push(SlopeOfe {
            index: ofe_index,
            azm,
            fwidth,
            nslpts,
            slplen,
            distance_mode,
            points,
        });
    }

    if let Some(extra) = cursor.next_token() {
        return Err(SlopeParserError::RecordCountError {
            context: format!(
                "unexpected trailing tokens beginning at line {}, column {}",
                extra.line, extra.column
            ),
        });
    }

    verify_cross_ofe_boundary_continuity(&ofes, options.abs_tolerance)?;

    Ok(SlopeProfile {
        datver,
        datver_source,
        ofe_count,
        ofes,
    })
}

fn validate_datver(datver: f64, options: SlopeParserOptions) -> Result<(), SlopeParserError> {
    match options.mode {
        SlopeParserMode::Strict => {
            if approx_eq(datver, options.canonical_datver, options.abs_tolerance) {
                Ok(())
            } else {
                Err(SlopeParserError::UnsupportedDatver {
                    datver,
                    mode: options.mode,
                    canonical_datver: options.canonical_datver,
                    compatibility_min_datver: options.compatibility_min_datver,
                })
            }
        }
        SlopeParserMode::Compatibility => {
            if datver + options.abs_tolerance >= options.compatibility_min_datver {
                Ok(())
            } else {
                Err(SlopeParserError::UnsupportedDatver {
                    datver,
                    mode: options.mode,
                    canonical_datver: options.canonical_datver,
                    compatibility_min_datver: options.compatibility_min_datver,
                })
            }
        }
    }
}

fn derive_distance_mode(
    ofe_index: usize,
    slplen: f64,
    points: &[SlopePoint],
    abs_tolerance: f64,
) -> Result<DistanceMode, SlopeParserError> {
    if points.len() < 2 {
        return Err(SlopeParserError::InvariantViolation {
            guard_id: "G-SLP-004",
            message: format!("OFE {} has fewer than 2 points after parse", ofe_index + 1),
        });
    }

    let first_x = points[0].xinput;
    if !approx_eq(first_x, 0.0, abs_tolerance) {
        return Err(SlopeParserError::EndpointConstraintError {
            ofe_index,
            message: format!("first xinput must be 0.0 (+/- {abs_tolerance}), got {first_x}"),
        });
    }

    for window in points.windows(2) {
        let left = window[0].xinput;
        let right = window[1].xinput;
        if right + abs_tolerance < left {
            return Err(SlopeParserError::EndpointConstraintError {
                ofe_index,
                message: format!(
                    "xinput must be monotonic non-decreasing; found {} then {}",
                    left, right
                ),
            });
        }
    }

    let last_x =
        points
            .last()
            .map(|pt| pt.xinput)
            .ok_or_else(|| SlopeParserError::InvariantViolation {
                guard_id: "G-SLP-006",
                message: format!("OFE {} has no terminal point", ofe_index + 1),
            })?;

    let ends_as_normalized = approx_eq(last_x, 1.0, abs_tolerance);
    let ends_as_absolute = approx_eq(last_x, slplen, abs_tolerance);

    let mode = match (ends_as_normalized, ends_as_absolute) {
        (true, true) => DistanceMode::Normalized,
        (true, false) => DistanceMode::Normalized,
        (false, true) => DistanceMode::Absolute,
        (false, false) => {
            return Err(SlopeParserError::EndpointConstraintError {
                ofe_index,
                message: format!(
                    "terminal xinput must equal 1.0 (normalized) or slplen ({slplen}) within +/- {abs_tolerance}; got {last_x}"
                ),
            })
        }
    };

    match mode {
        DistanceMode::Normalized => {
            if let Some(point) = points.iter().find(|pt| pt.xinput > 1.0 + abs_tolerance) {
                return Err(SlopeParserError::DistanceModeMixError {
                    ofe_index,
                    message: format!(
                        "normalized endpoint with out-of-range xinput {} (> 1.0 + tol)",
                        point.xinput
                    ),
                });
            }
        }
        DistanceMode::Absolute => {
            let has_absolute_scale = points.iter().any(|pt| pt.xinput > 1.0 + abs_tolerance);
            let has_normalized_fraction = points
                .iter()
                .skip(1)
                .take(points.len().saturating_sub(2))
                .any(|pt| pt.xinput > abs_tolerance && pt.xinput < 1.0 - abs_tolerance);

            if slplen > 1.0 + abs_tolerance && has_absolute_scale && has_normalized_fraction {
                return Err(SlopeParserError::DistanceModeMixError {
                    ofe_index,
                    message: "absolute endpoint with mixed fractional and dimensional interior xinput values"
                        .to_string(),
                });
            }
        }
    }

    Ok(mode)
}

fn verify_cross_ofe_boundary_continuity(
    ofes: &[SlopeOfe],
    abs_tolerance: f64,
) -> Result<(), SlopeParserError> {
    for (left_index, pair) in ofes.windows(2).enumerate() {
        let left = &pair[0];
        let right = &pair[1];

        let left_terminal_slope = left.points.last().map(|pt| pt.slpinp).ok_or_else(|| {
            SlopeParserError::InvariantViolation {
                guard_id: "G-SLP-007",
                message: format!("OFE {} missing terminal slope", left.index + 1),
            }
        })?;

        let right_initial_slope = right.points.first().map(|pt| pt.slpinp).ok_or_else(|| {
            SlopeParserError::InvariantViolation {
                guard_id: "G-SLP-007",
                message: format!("OFE {} missing initial slope", right.index + 1),
            }
        })?;

        if !approx_eq(left_terminal_slope, right_initial_slope, abs_tolerance) {
            return Err(SlopeParserError::CrossOfeBoundaryError {
                left_ofe_index: left_index,
                right_ofe_index: left_index + 1,
                left_terminal_slope,
                right_initial_slope,
                tolerance: abs_tolerance,
            });
        }
    }

    Ok(())
}

fn parse_f64(token: &Token) -> Result<f64, SlopeParserError> {
    token
        .text
        .parse::<f64>()
        .map_err(|_| SlopeParserError::TokenParseError {
            line: token.line,
            column: token.column,
            token: token.text.clone(),
            expected: "real",
        })
}

fn parse_count(
    token: &Token,
    field: &'static str,
    guard_id: &'static str,
    ofe_index: Option<usize>,
) -> Result<usize, SlopeParserError> {
    let parsed = token
        .text
        .parse::<i64>()
        .map_err(|_| SlopeParserError::TokenParseError {
            line: token.line,
            column: token.column,
            token: token.text.clone(),
            expected: "integer",
        })?;

    if parsed < 1 {
        return Err(SlopeParserError::FieldRangeError {
            field,
            value: parsed as f64,
            expected: ">= 1",
            guard_id,
            ofe_index,
        });
    }

    usize::try_from(parsed).map_err(|_| SlopeParserError::FieldRangeError {
        field,
        value: parsed as f64,
        expected: "within usize domain",
        guard_id,
        ofe_index,
    })
}

fn approx_eq(left: f64, right: f64, abs_tolerance: f64) -> bool {
    (left - right).abs() <= abs_tolerance
}

#[derive(Debug, Clone)]
struct Token {
    text: String,
    line: usize,
    column: usize,
}

#[derive(Debug)]
struct TokenCursor {
    tokens: Vec<Token>,
    index: usize,
}

impl TokenCursor {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }

    fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    fn next_token(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.index);
        if token.is_some() {
            self.index += 1;
        }
        token
    }

    fn next_required(
        &mut self,
        missing_context: &str,
        remaining_required_tokens: usize,
    ) -> Result<&Token, SlopeParserError> {
        self.next_token()
            .ok_or_else(|| SlopeParserError::RecordCountError {
                context: format!(
                "{missing_context}; at least {remaining_required_tokens} more token(s) required"
            ),
            })
    }
}

fn tokenize(contents: &str) -> Vec<Token> {
    let mut out = Vec::new();

    for (line_idx, raw_line) in contents.lines().enumerate() {
        let line_no = line_idx + 1;
        let trimmed = raw_line.trim_start();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let mut scan_start = 0usize;
        for part in raw_line.split_whitespace() {
            if let Some(offset) = raw_line[scan_start..].find(part) {
                let column = scan_start + offset + 1;
                out.push(Token {
                    text: part.to_string(),
                    line: line_no,
                    column,
                });
                scan_start = scan_start + offset + part.len();
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizer_skips_blank_and_comment_lines() {
        let src = "\n#comment\n  # indented comment\n97.5\n1\n";
        let tokens = tokenize(src);
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].text, "97.5");
        assert_eq!(tokens[1].text, "1");
    }
}
