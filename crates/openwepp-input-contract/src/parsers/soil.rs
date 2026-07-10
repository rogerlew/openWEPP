#![allow(
    clippy::float_cmp,
    clippy::manual_let_else,
    clippy::missing_errors_doc,
    clippy::similar_names,
    clippy::too_many_arguments,
    clippy::too_many_lines
)]

use std::error::Error;
use std::fmt;

const DATVER_EPSILON: f64 = 1e-6;
const POLICY_LKEFF_SENTINEL: f64 = -9999.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParserMode {
    Strict,
    Compatibility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopologyScope {
    Hillslope,
    WatershedChannel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoilErrorCode {
    SolE001,
    SolE002,
    SolE003,
    SolE004,
    SolE005,
    SolE006,
    SolE007,
    SolE008,
    SolE009,
}

impl SoilErrorCode {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SolE001 => "SOL-E-001",
            Self::SolE002 => "SOL-E-002",
            Self::SolE003 => "SOL-E-003",
            Self::SolE004 => "SOL-E-004",
            Self::SolE005 => "SOL-E-005",
            Self::SolE006 => "SOL-E-006",
            Self::SolE007 => "SOL-E-007",
            Self::SolE008 => "SOL-E-008",
            Self::SolE009 => "SOL-E-009",
        }
    }
}

impl fmt::Display for SoilErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoilParserError {
    pub code: SoilErrorCode,
    pub line: usize,
    pub field: &'static str,
    pub message: String,
}

impl SoilParserError {
    fn new(
        code: SoilErrorCode,
        line: usize,
        field: &'static str,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            line,
            field,
            message: message.into(),
        }
    }
}

impl fmt::Display for SoilParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at line {} field {}: {}",
            self.code, self.line, self.field, self.message
        )
    }
}

impl Error for SoilParserError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoilDatver {
    V97_5,
    V2006_2,
    V7777,
    V7778,
    V9002,
    V9003,
    V9005,
}

impl SoilDatver {
    #[must_use]
    pub fn numeric(self) -> f64 {
        match self {
            Self::V97_5 => 97.5,
            Self::V2006_2 => 2006.2,
            Self::V7777 => 7777.0,
            Self::V7778 => 7778.0,
            Self::V9002 => 9002.0,
            Self::V9003 => 9003.0,
            Self::V9005 => 9005.0,
        }
    }

    fn from_raw(
        raw: f64,
        options: SoilParserOptions,
        line: usize,
    ) -> Result<(Self, bool), SoilParserError> {
        if approx_eq(raw, 97.5) {
            return Ok((Self::V97_5, false));
        }
        if approx_eq(raw, 2006.2) {
            return Ok((Self::V2006_2, false));
        }
        if approx_eq(raw, 7777.0) {
            return Ok((Self::V7777, false));
        }
        if approx_eq(raw, 7778.0) {
            return Ok((Self::V7778, false));
        }
        if approx_eq(raw, 9002.0) {
            return Ok((Self::V9002, false));
        }
        if approx_eq(raw, 9003.0) {
            return Ok((Self::V9003, false));
        }
        if approx_eq(raw, 9005.0) {
            return Ok((Self::V9005, false));
        }

        if options.mode == ParserMode::Compatibility && options.allow_legacy_aliases {
            if approx_eq(raw, 97.0) {
                return Ok((Self::V97_5, true));
            }
            if approx_eq(raw, 2006.0) {
                return Ok((Self::V2006_2, true));
            }
        }

        Err(SoilParserError::new(
            SoilErrorCode::SolE003,
            line,
            "datver",
            format!("unsupported datver {raw}"),
        ))
    }

    fn requires_policy_row(self) -> bool {
        matches!(self, Self::V9002 | Self::V9003 | Self::V9005)
    }

    fn requires_restrictive_footer(self) -> bool {
        matches!(
            self,
            Self::V2006_2 | Self::V7777 | Self::V7778 | Self::V9002 | Self::V9003 | Self::V9005
        )
    }

    fn layer_arity(self) -> usize {
        match self {
            Self::V97_5 | Self::V2006_2 => 6,
            Self::V7777 => 10,
            Self::V7778 => 11,
            Self::V9002 | Self::V9003 | Self::V9005 => 18,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SoilParserOptions {
    pub mode: ParserMode,
    pub allow_legacy_aliases: bool,
    pub expected_topology_count: Option<usize>,
    pub topology_scope: Option<TopologyScope>,
}

impl Default for SoilParserOptions {
    fn default() -> Self {
        Self {
            mode: ParserMode::Strict,
            allow_legacy_aliases: false,
            expected_topology_count: None,
            topology_scope: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SoilProfile {
    pub datver: SoilDatver,
    pub datver_raw: f64,
    pub datver_alias_applied: bool,
    pub comment: String,
    pub ntemp: usize,
    pub ksflag: bool,
    pub ofes: Vec<SoilOfe>,
    pub restrictive_layer: Option<RestrictiveLayer>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SoilOfe {
    pub slid: String,
    pub texid: String,
    pub nsl: usize,
    pub salb: f64,
    pub sat: f64,
    pub ki: f64,
    pub kr: f64,
    pub shcrit: f64,
    pub avke: f64,
    pub policy: Option<DisturbedPolicy>,
    pub layers: Vec<SoilLayer>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisturbedPolicy {
    V9002 {
        ksatadj: bool,
        luse: String,
        stext: String,
        ksatfac_mm_h: f64,
        ksatrec_per_day: f64,
    },
    V9003 {
        ksatadj: bool,
        luse: String,
        burn_code: i32,
        stext: String,
        lkeff_mm_h: f64,
    },
    V9005 {
        ksatadj: bool,
        luse: String,
        burn_code: i32,
        stext: String,
        texid_enum: i32,
        uksat_mm_h: f64,
        lkeff_mm_h: f64,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct SoilLayer {
    pub depth_mm: f64,
    pub sand_pct: f64,
    pub clay_pct: f64,
    pub orgmat_pct: f64,
    pub cec_meq_100g: f64,
    pub rock_frag_pct: f64,
    pub bulk_density_g_cm3: Option<f64>,
    pub ksat_mm_h: Option<f64>,
    pub anisotropy_ratio: Option<f64>,
    pub fc_measured: Option<f64>,
    pub wp_measured: Option<f64>,
    pub theta_r_rosetta: Option<f64>,
    pub theta_s_rosetta: Option<f64>,
    pub alpha_vg: Option<f64>,
    pub npar_vg: Option<f64>,
    pub ks_rosetta_cm_d: Option<f64>,
    pub wp_rosetta: Option<f64>,
    pub fc_rosetta: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RestrictiveLayer {
    pub slflag: bool,
    pub ui_bdrkth_mm: f64,
    pub kslast_mm_h: f64,
}

struct SoilPreamble {
    datver: SoilDatver,
    datver_raw: f64,
    datver_alias_applied: bool,
    comment: String,
    ntemp: usize,
    ksflag: bool,
}

pub fn parse_soil(input: &str, options: SoilParserOptions) -> Result<SoilProfile, SoilParserError> {
    let mut cursor = LineCursor::new(input, options.mode);
    let preamble = parse_soil_preamble(&mut cursor, options)?;
    let (ofes, per_ofe_restrictive) =
        parse_soil_ofes(&mut cursor, preamble.datver, preamble.ntemp, options.mode)?;
    let restrictive_layer =
        parse_restrictive_footer(&mut cursor, preamble.datver, per_ofe_restrictive)?;
    reject_trailing_records(&mut cursor)?;

    Ok(SoilProfile {
        datver: preamble.datver,
        datver_raw: preamble.datver_raw,
        datver_alias_applied: preamble.datver_alias_applied,
        comment: preamble.comment,
        ntemp: preamble.ntemp,
        ksflag: preamble.ksflag,
        ofes,
        restrictive_layer,
    })
}

fn parse_soil_preamble(
    cursor: &mut LineCursor,
    options: SoilParserOptions,
) -> Result<SoilPreamble, SoilParserError> {
    let (line_no, datver_line) = cursor.next_line().ok_or_else(|| {
        SoilParserError::new(SoilErrorCode::SolE002, 0, "datver", "missing datver line")
    })?;
    let datver_raw = parse_f64(
        single_token(datver_line, line_no, "datver")?,
        line_no,
        "datver",
    )?;
    let (datver, datver_alias_applied) = SoilDatver::from_raw(datver_raw, options, line_no)?;

    let (_comment_line_no, comment_line) = cursor.next_line().ok_or_else(|| {
        SoilParserError::new(
            SoilErrorCode::SolE002,
            line_no,
            "solcom",
            "missing soil comment line",
        )
    })?;
    let comment = comment_line.to_string();

    let (counts_line_no, counts_line) = cursor.next_line().ok_or_else(|| {
        SoilParserError::new(
            SoilErrorCode::SolE002,
            line_no,
            "ntemp,ksflag",
            "missing ntemp/ksflag line",
        )
    })?;
    let counts = tokens_exact(counts_line, 2, counts_line_no, "ntemp,ksflag")?;
    let ntemp = parse_usize(counts[0], counts_line_no, "ntemp")?;
    if ntemp == 0 {
        return Err(SoilParserError::new(
            SoilErrorCode::SolE004,
            counts_line_no,
            "ntemp",
            "ntemp must be > 0",
        ));
    }

    if let Some(expected) = options.expected_topology_count {
        if ntemp != expected {
            let scope = match options.topology_scope {
                Some(TopologyScope::Hillslope) => "hillslope",
                Some(TopologyScope::WatershedChannel) => "watershed-channel",
                None => "unknown-scope",
            };
            return Err(SoilParserError::new(
                SoilErrorCode::SolE007,
                counts_line_no,
                "ntemp",
                format!(
                    "ntemp {ntemp} does not match expected topology count {expected} for {scope}"
                ),
            ));
        }
    }

    let ksflag_raw = parse_i32(counts[1], counts_line_no, "ksflag")?;
    let ksflag = match ksflag_raw {
        0 => false,
        1 => true,
        _ => {
            return Err(SoilParserError::new(
                SoilErrorCode::SolE005,
                counts_line_no,
                "ksflag",
                "ksflag must be 0 or 1",
            ));
        }
    };

    Ok(SoilPreamble {
        datver,
        datver_raw,
        datver_alias_applied,
        comment,
        ntemp,
        ksflag,
    })
}

fn parse_soil_ofes(
    cursor: &mut LineCursor,
    datver: SoilDatver,
    ntemp: usize,
    mode: ParserMode,
) -> Result<(Vec<SoilOfe>, Option<RestrictiveLayer>), SoilParserError> {
    let mut ofes = Vec::with_capacity(ntemp);
    let mut per_ofe_restrictive: Option<RestrictiveLayer> = None;
    for _ in 0..ntemp {
        let (ofe, ofe_restrictive) = parse_ofe_block(cursor, datver, mode)?;
        if let Some(ofe_restrictive) = ofe_restrictive {
            if let Some(existing) = &per_ofe_restrictive {
                if existing != &ofe_restrictive {
                    return Err(SoilParserError::new(
                        SoilErrorCode::SolE006,
                        cursor.current_line_number(),
                        "slflag,ui_bdrkth,kslast",
                        "per-OFE restrictive rows must be identical",
                    ));
                }
            } else {
                per_ofe_restrictive = Some(ofe_restrictive);
            }
        }
        ofes.push(ofe);
    }

    Ok((ofes, per_ofe_restrictive))
}

fn parse_restrictive_footer(
    cursor: &mut LineCursor,
    datver: SoilDatver,
    per_ofe_restrictive: Option<RestrictiveLayer>,
) -> Result<Option<RestrictiveLayer>, SoilParserError> {
    if !datver.requires_restrictive_footer() {
        return Ok(None);
    }

    if let Some(per_ofe_restrictive) = per_ofe_restrictive {
        if let Some((line_no, line)) = cursor.peek_line()
            && line.split_whitespace().count() == 3
        {
            let trailing_restrictive = parse_restrictive_layer(line, line_no)?;
            if trailing_restrictive != per_ofe_restrictive {
                return Err(SoilParserError::new(
                    SoilErrorCode::SolE006,
                    line_no,
                    "slflag,ui_bdrkth,kslast",
                    "trailing restrictive row conflicts with per-OFE restrictive rows",
                ));
            }
            cursor.next_line();
        }
        Ok(Some(per_ofe_restrictive))
    } else {
        let next = cursor.next_line();
        let (footer_line_no, footer_line) = match next {
            Some(v) => v,
            None => {
                return Err(SoilParserError::new(
                    SoilErrorCode::SolE002,
                    cursor.current_line_number(),
                    "slflag,ui_bdrkth,kslast",
                    "missing restrictive-layer footer",
                ));
            }
        };
        Ok(Some(parse_restrictive_layer(footer_line, footer_line_no)?))
    }
}

fn reject_trailing_records(cursor: &mut LineCursor) -> Result<(), SoilParserError> {
    if let Some((line_no_extra, _)) = cursor.next_line() {
        return Err(SoilParserError::new(
            SoilErrorCode::SolE006,
            line_no_extra,
            "file-tail",
            "unexpected trailing records",
        ));
    }

    Ok(())
}

fn parse_ofe_block(
    cursor: &mut LineCursor,
    datver: SoilDatver,
    mode: ParserMode,
) -> Result<(SoilOfe, Option<RestrictiveLayer>), SoilParserError> {
    let header = parse_ofe_header_and_policy(cursor, datver, mode)?;
    let layers = parse_ofe_layers(cursor, datver, header.nsl)?;
    let per_ofe_restrictive = maybe_parse_ofe_restrictive_row(cursor, datver, mode)?;

    Ok((
        SoilOfe {
            slid: header.slid,
            texid: header.texid,
            nsl: header.nsl,
            salb: header.salb,
            sat: header.sat,
            ki: header.ki,
            kr: header.kr,
            shcrit: header.shcrit,
            avke: header.avke,
            policy: header.policy,
            layers,
        },
        per_ofe_restrictive,
    ))
}

struct OfeHeader {
    slid: String,
    texid: String,
    nsl: usize,
    salb: f64,
    sat: f64,
    ki: f64,
    kr: f64,
    shcrit: f64,
    avke: f64,
    policy: Option<DisturbedPolicy>,
}

fn parse_ofe_header_and_policy(
    cursor: &mut LineCursor,
    datver: SoilDatver,
    mode: ParserMode,
) -> Result<OfeHeader, SoilParserError> {
    let next = cursor.next_line();
    let (first_line_no, first_line) = match next {
        Some(v) => v,
        None => {
            return Err(SoilParserError::new(
                SoilErrorCode::SolE002,
                cursor.current_line_number(),
                "slid,texid,nsl,salb,sat,ki,kr,shcrit,avke",
                "missing OFE header line",
            ));
        }
    };

    let mut policy = None;
    let (header_line_no, header_line) = if datver.requires_policy_row() {
        match parse_policy_row(datver, mode, first_line, first_line_no) {
            Ok(policy_first) => {
                policy = Some(policy_first);
                let next_header = cursor.next_line();
                match next_header {
                    Some(v) => v,
                    None => {
                        return Err(SoilParserError::new(
                            SoilErrorCode::SolE002,
                            cursor.current_line_number(),
                            "slid,texid,nsl,salb,sat,ki,kr,shcrit,avke",
                            "missing OFE header line after policy row",
                        ));
                    }
                }
            }
            Err(_) => (first_line_no, first_line),
        }
    } else {
        (first_line_no, first_line)
    };

    let t = parse_ofe_header_tokens(
        header_line,
        header_line_no,
        datver,
        mode,
        "slid,texid,nsl,salb,sat,ki,kr,shcrit,avke",
    )?;

    let slid = t[0].clone();
    let texid = t[1].clone();
    let nsl = parse_usize(&t[2], header_line_no, "nsl")?;
    if nsl == 0 {
        return Err(SoilParserError::new(
            SoilErrorCode::SolE004,
            header_line_no,
            "nsl",
            "nsl must be > 0",
        ));
    }

    let salb = parse_f64(&t[3], header_line_no, "salb")?;
    let sat = parse_f64(&t[4], header_line_no, "sat")?;
    let ki = parse_f64(&t[5], header_line_no, "ki")?;
    let kr = parse_f64(&t[6], header_line_no, "kr")?;
    let shcrit = parse_f64(&t[7], header_line_no, "shcrit")?;
    let avke = parse_f64(&t[8], header_line_no, "avke")?;

    validate_fraction_unit(salb, header_line_no, "salb")?;
    validate_fraction_unit(sat, header_line_no, "sat")?;
    validate_non_negative(ki, header_line_no, "ki")?;
    validate_non_negative(kr, header_line_no, "kr")?;
    validate_non_negative(shcrit, header_line_no, "shcrit")?;
    validate_non_negative(avke, header_line_no, "avke")?;

    if policy.is_none() && datver.requires_policy_row() {
        let next = cursor.next_line();
        let (policy_line_no, policy_line) = match next {
            Some(v) => v,
            None => {
                return Err(SoilParserError::new(
                    SoilErrorCode::SolE002,
                    cursor.current_line_number(),
                    "policy-row",
                    "missing datver-specific policy row",
                ));
            }
        };
        policy = Some(parse_policy_row(datver, mode, policy_line, policy_line_no)?);
    }

    Ok(OfeHeader {
        slid,
        texid,
        nsl,
        salb,
        sat,
        ki,
        kr,
        shcrit,
        avke,
        policy,
    })
}

fn parse_ofe_layers(
    cursor: &mut LineCursor,
    datver: SoilDatver,
    nsl: usize,
) -> Result<Vec<SoilLayer>, SoilParserError> {
    let mut layers = Vec::with_capacity(nsl);
    let mut prev_depth = 0.0;
    for _ in 0..nsl {
        let next = cursor.next_line();
        let (layer_line_no, layer_line) = match next {
            Some(v) => v,
            None => {
                return Err(SoilParserError::new(
                    SoilErrorCode::SolE002,
                    cursor.current_line_number(),
                    "layer-row",
                    "missing layer row",
                ));
            }
        };
        let layer = parse_layer_row(datver, layer_line, layer_line_no)?;

        if layer.depth_mm <= 0.0 {
            return Err(SoilParserError::new(
                SoilErrorCode::SolE005,
                layer_line_no,
                "solthk",
                "layer depth must be > 0",
            ));
        }
        if layer.depth_mm <= prev_depth {
            return Err(SoilParserError::new(
                SoilErrorCode::SolE009,
                layer_line_no,
                "solthk",
                "layer depths must be strictly increasing",
            ));
        }
        prev_depth = layer.depth_mm;

        layers.push(layer);
    }

    Ok(layers)
}

fn maybe_parse_ofe_restrictive_row(
    cursor: &mut LineCursor,
    datver: SoilDatver,
    _mode: ParserMode,
) -> Result<Option<RestrictiveLayer>, SoilParserError> {
    if !datver.requires_restrictive_footer() {
        return Ok(None);
    }

    let Some((line_no, line)) = cursor.peek_line() else {
        return Ok(None);
    };

    if line.split_whitespace().count() != 3 {
        return Ok(None);
    }

    let restrictive = parse_restrictive_layer(line, line_no)?;
    cursor.next_line();
    Ok(Some(restrictive))
}

fn parse_policy_tokens(
    policy_line: &str,
    expected: usize,
    line_no: usize,
    field: &'static str,
    datver: SoilDatver,
) -> Result<Vec<String>, SoilParserError> {
    if matches!(
        datver,
        SoilDatver::V9002 | SoilDatver::V9003 | SoilDatver::V9005
    ) && (policy_line.contains('\'') || policy_line.contains('"'))
    {
        let tokens = tokenize_whitespace_and_quotes(policy_line, line_no, field)?;
        if tokens.len() != expected {
            return Err(SoilParserError::new(
                SoilErrorCode::SolE006,
                line_no,
                field,
                format!(
                    "variant arity mismatch: expected {expected} token(s), found {}",
                    tokens.len()
                ),
            ));
        }
        return Ok(tokens);
    }

    Ok(tokens_exact(policy_line, expected, line_no, field)?
        .into_iter()
        .map(ToString::to_string)
        .collect())
}

fn parse_policy_row(
    datver: SoilDatver,
    _mode: ParserMode,
    policy_line: &str,
    line_no: usize,
) -> Result<DisturbedPolicy, SoilParserError> {
    match datver {
        SoilDatver::V9002 => parse_v9002_policy_row(policy_line, line_no),
        SoilDatver::V9003 => parse_v9003_policy_row(policy_line, line_no),
        SoilDatver::V9005 => parse_v9005_policy_row(policy_line, line_no),
        _ => Err(SoilParserError::new(
            SoilErrorCode::SolE006,
            line_no,
            "policy-row",
            "policy row is not applicable for this datver",
        )),
    }
}

fn parse_v9002_policy_row(
    policy_line: &str,
    line_no: usize,
) -> Result<DisturbedPolicy, SoilParserError> {
    let t = parse_policy_tokens(
        policy_line,
        5,
        line_no,
        "ksatadj,luse,stext,ksatfac,ksatrec",
        SoilDatver::V9002,
    )?;
    let ksatadj = parse_binary_flag(&t[0], line_no, "ksatadj")?;
    let ksatfac = parse_f64(&t[3], line_no, "ksatfac")?;
    let ksatrec = parse_f64(&t[4], line_no, "ksatrec")?;
    validate_non_negative(ksatfac, line_no, "ksatfac")?;
    validate_non_negative(ksatrec, line_no, "ksatrec")?;

    Ok(DisturbedPolicy::V9002 {
        ksatadj,
        luse: t[1].clone(),
        stext: t[2].clone(),
        ksatfac_mm_h: ksatfac,
        ksatrec_per_day: ksatrec,
    })
}

fn parse_v9003_policy_row(
    policy_line: &str,
    line_no: usize,
) -> Result<DisturbedPolicy, SoilParserError> {
    let t = parse_policy_tokens(
        policy_line,
        5,
        line_no,
        "ksatadj,luse,burn_code,stext,lkeff",
        SoilDatver::V9003,
    )?;
    let ksatadj = parse_binary_flag(&t[0], line_no, "ksatadj")?;
    let burn_code = parse_burn_code(&t[2], line_no)?;
    let lkeff = parse_lkeff_policy_value(&t[4], line_no)?;

    Ok(DisturbedPolicy::V9003 {
        ksatadj,
        luse: t[1].clone(),
        burn_code,
        stext: t[3].clone(),
        lkeff_mm_h: lkeff,
    })
}

fn parse_v9005_policy_row(
    policy_line: &str,
    line_no: usize,
) -> Result<DisturbedPolicy, SoilParserError> {
    let t = parse_policy_tokens(
        policy_line,
        7,
        line_no,
        "ksatadj,luse,burn_code,stext,texid_enum,uksat,lkeff",
        SoilDatver::V9005,
    )?;
    let ksatadj = parse_binary_flag(&t[0], line_no, "ksatadj")?;
    let burn_code = parse_burn_code(&t[2], line_no)?;
    let texid_enum = parse_texid_enum(&t[4], line_no)?;
    let uksat = parse_f64(&t[5], line_no, "uksat")?;
    validate_non_negative(uksat, line_no, "uksat")?;
    let lkeff = parse_lkeff_policy_value(&t[6], line_no)?;

    Ok(DisturbedPolicy::V9005 {
        ksatadj,
        luse: t[1].clone(),
        burn_code,
        stext: t[3].clone(),
        texid_enum,
        uksat_mm_h: uksat,
        lkeff_mm_h: lkeff,
    })
}

fn parse_burn_code(token: &str, line_no: usize) -> Result<i32, SoilParserError> {
    let burn_code = parse_i32(token, line_no, "burn_code")?;
    if burn_code < 0 {
        return Err(SoilParserError::new(
            SoilErrorCode::SolE005,
            line_no,
            "burn_code",
            "burn_code must be non-negative",
        ));
    }
    Ok(burn_code)
}

fn parse_texid_enum(token: &str, line_no: usize) -> Result<i32, SoilParserError> {
    let texid_enum = parse_i32(token, line_no, "texid_enum")?;
    if texid_enum <= 0 {
        return Err(SoilParserError::new(
            SoilErrorCode::SolE005,
            line_no,
            "texid_enum",
            "texid_enum must be > 0",
        ));
    }
    Ok(texid_enum)
}

fn parse_lkeff_policy_value(token: &str, line_no: usize) -> Result<f64, SoilParserError> {
    let lkeff = parse_f64(token, line_no, "lkeff")?;
    if lkeff != POLICY_LKEFF_SENTINEL {
        validate_non_negative(lkeff, line_no, "lkeff")?;
    }
    Ok(lkeff)
}

fn parse_layer_row(
    datver: SoilDatver,
    row: &str,
    line_no: usize,
) -> Result<SoilLayer, SoilParserError> {
    let expected_arity = datver.layer_arity();
    let t = tokens_exact(row, expected_arity, line_no, "layer-row")?;

    match datver {
        SoilDatver::V97_5 | SoilDatver::V2006_2 => parse_base_layer_row(&t, line_no),
        SoilDatver::V7777 => parse_7777_layer_row(&t, line_no),
        SoilDatver::V7778 => parse_7778_layer_row(&t, line_no),
        SoilDatver::V9002 | SoilDatver::V9003 | SoilDatver::V9005 => {
            parse_rosetta_layer_row(&t, line_no)
        }
    }
}

fn parse_base_layer_row(t: &[&str], line_no: usize) -> Result<SoilLayer, SoilParserError> {
    let depth_mm = parse_f64(t[0], line_no, "solthk")?;
    let sand_pct = parse_f64(t[1], line_no, "sand")?;
    let clay_pct = parse_f64(t[2], line_no, "clay")?;
    let orgmat_pct = parse_f64(t[3], line_no, "orgmat")?;
    let cec_meq_100g = parse_f64(t[4], line_no, "cec")?;
    let rock_frag_pct = parse_f64(t[5], line_no, "rfg")?;

    validate_percent(sand_pct, line_no, "sand")?;
    validate_percent(clay_pct, line_no, "clay")?;
    validate_percent(orgmat_pct, line_no, "orgmat")?;
    validate_percent(rock_frag_pct, line_no, "rfg")?;
    validate_non_negative(cec_meq_100g, line_no, "cec")?;

    Ok(SoilLayer {
        depth_mm,
        sand_pct,
        clay_pct,
        orgmat_pct,
        cec_meq_100g,
        rock_frag_pct,
        bulk_density_g_cm3: None,
        ksat_mm_h: None,
        anisotropy_ratio: None,
        fc_measured: None,
        wp_measured: None,
        theta_r_rosetta: None,
        theta_s_rosetta: None,
        alpha_vg: None,
        npar_vg: None,
        ks_rosetta_cm_d: None,
        wp_rosetta: None,
        fc_rosetta: None,
    })
}

fn parse_7777_layer_row(t: &[&str], line_no: usize) -> Result<SoilLayer, SoilParserError> {
    let depth_mm = parse_f64(t[0], line_no, "solthk")?;
    let bulk_density_g_cm3 = parse_f64(t[1], line_no, "bd")?;
    let ksat_mm_h = parse_f64(t[2], line_no, "ksat")?;
    let fc_measured = parse_f64(t[3], line_no, "fc")?;
    let wp_measured = parse_f64(t[4], line_no, "wp")?;
    let sand_pct = parse_f64(t[5], line_no, "sand")?;
    let clay_pct = parse_f64(t[6], line_no, "clay")?;
    let orgmat_pct = parse_f64(t[7], line_no, "orgmat")?;
    let cec_meq_100g = parse_f64(t[8], line_no, "cec")?;
    let rock_frag_pct = parse_f64(t[9], line_no, "rfg")?;

    validate_common_extended(
        line_no,
        sand_pct,
        clay_pct,
        orgmat_pct,
        cec_meq_100g,
        rock_frag_pct,
        bulk_density_g_cm3,
        ksat_mm_h,
        fc_measured,
        wp_measured,
    )?;

    Ok(SoilLayer {
        depth_mm,
        sand_pct,
        clay_pct,
        orgmat_pct,
        cec_meq_100g,
        rock_frag_pct,
        bulk_density_g_cm3: Some(bulk_density_g_cm3),
        ksat_mm_h: Some(ksat_mm_h),
        anisotropy_ratio: Some(1.0),
        fc_measured: Some(fc_measured),
        wp_measured: Some(wp_measured),
        theta_r_rosetta: None,
        theta_s_rosetta: None,
        alpha_vg: None,
        npar_vg: None,
        ks_rosetta_cm_d: None,
        wp_rosetta: None,
        fc_rosetta: None,
    })
}

fn parse_7778_layer_row(t: &[&str], line_no: usize) -> Result<SoilLayer, SoilParserError> {
    let depth_mm = parse_f64(t[0], line_no, "solthk")?;
    let bulk_density_g_cm3 = parse_f64(t[1], line_no, "bd")?;
    let ksat_mm_h = parse_f64(t[2], line_no, "ksat")?;
    let anisotropy_ratio = parse_f64(t[3], line_no, "anisotropy")?;
    let fc_measured = parse_f64(t[4], line_no, "fc")?;
    let wp_measured = parse_f64(t[5], line_no, "wp")?;
    let sand_pct = parse_f64(t[6], line_no, "sand")?;
    let clay_pct = parse_f64(t[7], line_no, "clay")?;
    let orgmat_pct = parse_f64(t[8], line_no, "orgmat")?;
    let cec_meq_100g = parse_f64(t[9], line_no, "cec")?;
    let rock_frag_pct = parse_f64(t[10], line_no, "rfg")?;

    validate_common_extended(
        line_no,
        sand_pct,
        clay_pct,
        orgmat_pct,
        cec_meq_100g,
        rock_frag_pct,
        bulk_density_g_cm3,
        ksat_mm_h,
        fc_measured,
        wp_measured,
    )?;
    validate_positive(anisotropy_ratio, line_no, "anisotropy")?;

    Ok(SoilLayer {
        depth_mm,
        sand_pct,
        clay_pct,
        orgmat_pct,
        cec_meq_100g,
        rock_frag_pct,
        bulk_density_g_cm3: Some(bulk_density_g_cm3),
        ksat_mm_h: Some(ksat_mm_h),
        anisotropy_ratio: Some(anisotropy_ratio),
        fc_measured: Some(fc_measured),
        wp_measured: Some(wp_measured),
        theta_r_rosetta: None,
        theta_s_rosetta: None,
        alpha_vg: None,
        npar_vg: None,
        ks_rosetta_cm_d: None,
        wp_rosetta: None,
        fc_rosetta: None,
    })
}

fn parse_rosetta_layer_row(t: &[&str], line_no: usize) -> Result<SoilLayer, SoilParserError> {
    let depth_mm = parse_f64(t[0], line_no, "solthk")?;
    let bulk_density_g_cm3 = parse_f64(t[1], line_no, "bd")?;
    let ksat_mm_h = parse_f64(t[2], line_no, "ksat")?;
    let anisotropy_ratio = parse_f64(t[3], line_no, "anisotropy")?;
    let fc_measured = parse_f64(t[4], line_no, "fc")?;
    let wp_measured = parse_f64(t[5], line_no, "wp")?;
    let sand_pct = parse_f64(t[6], line_no, "sand")?;
    let clay_pct = parse_f64(t[7], line_no, "clay")?;
    let orgmat_pct = parse_f64(t[8], line_no, "orgmat")?;
    let cec_meq_100g = parse_f64(t[9], line_no, "cec")?;
    let rock_frag_pct = parse_f64(t[10], line_no, "rfg")?;
    let theta_r_rosetta = parse_f64(t[11], line_no, "theta_r")?;
    let theta_s_rosetta = parse_f64(t[12], line_no, "theta_s")?;
    let alpha_vg = parse_f64(t[13], line_no, "alpha")?;
    let npar_vg = parse_f64(t[14], line_no, "npar")?;
    let ks_rosetta_cm_d = parse_f64(t[15], line_no, "ks")?;
    let wp_rosetta = parse_f64(t[16], line_no, "wp_rosetta")?;
    let fc_rosetta = parse_f64(t[17], line_no, "fc_rosetta")?;

    validate_common_extended(
        line_no,
        sand_pct,
        clay_pct,
        orgmat_pct,
        cec_meq_100g,
        rock_frag_pct,
        bulk_density_g_cm3,
        ksat_mm_h,
        fc_measured,
        wp_measured,
    )?;
    validate_positive(anisotropy_ratio, line_no, "anisotropy")?;
    validate_fraction_unit(theta_r_rosetta, line_no, "theta_r")?;
    validate_fraction_unit(theta_s_rosetta, line_no, "theta_s")?;
    if theta_r_rosetta > theta_s_rosetta {
        return Err(SoilParserError::new(
            SoilErrorCode::SolE005,
            line_no,
            "theta_r/theta_s",
            "theta_r must be <= theta_s",
        ));
    }
    validate_positive(alpha_vg, line_no, "alpha")?;
    validate_positive(npar_vg, line_no, "npar")?;
    validate_non_negative(ks_rosetta_cm_d, line_no, "ks")?;
    validate_fraction_unit(wp_rosetta, line_no, "wp_rosetta")?;
    validate_fraction_unit(fc_rosetta, line_no, "fc_rosetta")?;

    Ok(SoilLayer {
        depth_mm,
        sand_pct,
        clay_pct,
        orgmat_pct,
        cec_meq_100g,
        rock_frag_pct,
        bulk_density_g_cm3: Some(bulk_density_g_cm3),
        ksat_mm_h: Some(ksat_mm_h),
        anisotropy_ratio: Some(anisotropy_ratio),
        fc_measured: Some(fc_measured),
        wp_measured: Some(wp_measured),
        theta_r_rosetta: Some(theta_r_rosetta),
        theta_s_rosetta: Some(theta_s_rosetta),
        alpha_vg: Some(alpha_vg),
        npar_vg: Some(npar_vg),
        ks_rosetta_cm_d: Some(ks_rosetta_cm_d),
        wp_rosetta: Some(wp_rosetta),
        fc_rosetta: Some(fc_rosetta),
    })
}

fn parse_restrictive_layer(row: &str, line_no: usize) -> Result<RestrictiveLayer, SoilParserError> {
    let t = tokens_exact(row, 3, line_no, "slflag,ui_bdrkth,kslast")?;
    let slflag_raw = parse_i32(t[0], line_no, "slflag")?;
    let slflag = match slflag_raw {
        0 => false,
        1 => true,
        _ => {
            return Err(SoilParserError::new(
                SoilErrorCode::SolE009,
                line_no,
                "slflag",
                "slflag must be 0 or 1",
            ));
        }
    };

    let ui_bdrkth_mm = parse_f64(t[1], line_no, "ui_bdrkth")?;
    let kslast_mm_h = parse_f64(t[2], line_no, "kslast")?;

    if slflag {
        validate_positive(ui_bdrkth_mm, line_no, "ui_bdrkth")?;
        validate_non_negative(kslast_mm_h, line_no, "kslast")?;
    }

    Ok(RestrictiveLayer {
        slflag,
        ui_bdrkth_mm,
        kslast_mm_h,
    })
}

fn validate_common_extended(
    line_no: usize,
    sand_pct: f64,
    clay_pct: f64,
    orgmat_pct: f64,
    cec_meq_100g: f64,
    rock_frag_pct: f64,
    bulk_density_g_cm3: f64,
    ksat_mm_h: f64,
    fc_measured: f64,
    wp_measured: f64,
) -> Result<(), SoilParserError> {
    validate_percent(sand_pct, line_no, "sand")?;
    validate_percent(clay_pct, line_no, "clay")?;
    validate_percent(orgmat_pct, line_no, "orgmat")?;
    validate_percent(rock_frag_pct, line_no, "rfg")?;
    validate_non_negative(cec_meq_100g, line_no, "cec")?;
    validate_positive(bulk_density_g_cm3, line_no, "bd")?;
    validate_non_negative(ksat_mm_h, line_no, "ksat")?;
    validate_fraction_unit(fc_measured, line_no, "fc")?;
    validate_fraction_unit(wp_measured, line_no, "wp")?;
    Ok(())
}

fn validate_non_negative(
    value: f64,
    line_no: usize,
    field: &'static str,
) -> Result<(), SoilParserError> {
    if !value.is_finite() {
        return Err(SoilParserError::new(
            SoilErrorCode::SolE005,
            line_no,
            field,
            "value must be finite",
        ));
    }
    if value < 0.0 {
        return Err(SoilParserError::new(
            SoilErrorCode::SolE005,
            line_no,
            field,
            "value must be >= 0",
        ));
    }
    Ok(())
}

fn validate_positive(
    value: f64,
    line_no: usize,
    field: &'static str,
) -> Result<(), SoilParserError> {
    validate_non_negative(value, line_no, field)?;
    if value == 0.0 {
        return Err(SoilParserError::new(
            SoilErrorCode::SolE005,
            line_no,
            field,
            "value must be > 0",
        ));
    }
    Ok(())
}

fn validate_percent(
    value: f64,
    line_no: usize,
    field: &'static str,
) -> Result<(), SoilParserError> {
    validate_non_negative(value, line_no, field)?;
    if value > 100.0 {
        return Err(SoilParserError::new(
            SoilErrorCode::SolE005,
            line_no,
            field,
            "percent value must be <= 100",
        ));
    }
    Ok(())
}

fn validate_fraction_unit(
    value: f64,
    line_no: usize,
    field: &'static str,
) -> Result<(), SoilParserError> {
    validate_non_negative(value, line_no, field)?;
    if value > 1.0 {
        return Err(SoilParserError::new(
            SoilErrorCode::SolE005,
            line_no,
            field,
            "fraction value must be <= 1",
        ));
    }
    Ok(())
}

fn parse_i32(token: &str, line_no: usize, field: &'static str) -> Result<i32, SoilParserError> {
    token.parse::<i32>().map_err(|_| {
        SoilParserError::new(
            SoilErrorCode::SolE001,
            line_no,
            field,
            format!("failed to parse integer token '{token}'"),
        )
    })
}

fn parse_usize(token: &str, line_no: usize, field: &'static str) -> Result<usize, SoilParserError> {
    token.parse::<usize>().map_err(|_| {
        SoilParserError::new(
            SoilErrorCode::SolE001,
            line_no,
            field,
            format!("failed to parse usize token '{token}'"),
        )
    })
}

fn parse_f64(token: &str, line_no: usize, field: &'static str) -> Result<f64, SoilParserError> {
    token.parse::<f64>().map_err(|_| {
        SoilParserError::new(
            SoilErrorCode::SolE001,
            line_no,
            field,
            format!("failed to parse float token '{token}'"),
        )
    })
}

fn tokens_exact<'a>(
    line: &'a str,
    expected: usize,
    line_no: usize,
    field: &'static str,
) -> Result<Vec<&'a str>, SoilParserError> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.len() != expected {
        return Err(SoilParserError::new(
            SoilErrorCode::SolE006,
            line_no,
            field,
            format!(
                "variant arity mismatch: expected {expected} token(s), found {}",
                tokens.len()
            ),
        ));
    }
    Ok(tokens)
}

fn parse_ofe_header_tokens(
    line: &str,
    line_no: usize,
    datver: SoilDatver,
    _mode: ParserMode,
    field: &'static str,
) -> Result<Vec<String>, SoilParserError> {
    if matches!(
        datver,
        SoilDatver::V7778 | SoilDatver::V9002 | SoilDatver::V9003 | SoilDatver::V9005
    ) && (line.contains('\'') || line.contains('"'))
    {
        let mut tokens = tokenize_whitespace_and_quotes(line, line_no, field)?;
        if tokens.len() == 8 {
            // Canonical quoted header rows may omit `avke`; normalize to explicit 0.0.
            tokens.push("0.0".to_string());
        }
        if tokens.len() != 9 {
            return Err(SoilParserError::new(
                SoilErrorCode::SolE006,
                line_no,
                field,
                format!(
                    "variant arity mismatch: expected 9 token(s), found {}",
                    tokens.len()
                ),
            ));
        }
        return Ok(tokens);
    }

    Ok(tokens_exact(line, 9, line_no, field)?
        .into_iter()
        .map(ToString::to_string)
        .collect())
}

fn tokenize_whitespace_and_quotes(
    line: &str,
    line_no: usize,
    field: &'static str,
) -> Result<Vec<String>, SoilParserError> {
    let mut tokens = Vec::new();
    let mut iter = line.char_indices().peekable();

    while let Some((idx, ch)) = iter.peek().copied() {
        if ch.is_whitespace() {
            iter.next();
            continue;
        }

        if ch == '\'' || ch == '"' {
            let quote_char = ch;
            iter.next();
            let mut token = String::new();
            let mut closed = false;
            let mut escaped = false;

            for (_, quote_ch) in iter.by_ref() {
                if quote_char == '"' && escaped {
                    token.push(quote_ch);
                    escaped = false;
                    continue;
                }

                if quote_char == '"' && quote_ch == '\\' {
                    escaped = true;
                    continue;
                }

                if quote_ch == quote_char {
                    closed = true;
                    break;
                }

                token.push(quote_ch);
            }

            if !closed {
                return Err(SoilParserError::new(
                    SoilErrorCode::SolE006,
                    line_no,
                    field,
                    "unterminated quoted token",
                ));
            }

            if quote_char == '"' && escaped {
                return Err(SoilParserError::new(
                    SoilErrorCode::SolE006,
                    line_no,
                    field,
                    "unterminated escape in double-quoted token",
                ));
            }

            tokens.push(token);
            continue;
        }

        let token_start = idx;
        let mut token_end = line.len();
        while let Some((peek_idx, peek_ch)) = iter.peek().copied() {
            if peek_ch.is_whitespace() {
                token_end = peek_idx;
                break;
            }
            iter.next();
        }
        tokens.push(line[token_start..token_end].to_string());
    }

    Ok(tokens)
}

fn single_token<'a>(
    line: &'a str,
    line_no: usize,
    field: &'static str,
) -> Result<&'a str, SoilParserError> {
    let t: Vec<&str> = line.split_whitespace().collect();
    if t.len() != 1 {
        return Err(SoilParserError::new(
            SoilErrorCode::SolE006,
            line_no,
            field,
            format!("expected exactly 1 token, found {}", t.len()),
        ));
    }
    Ok(t[0])
}

fn parse_binary_flag(
    token: &str,
    line_no: usize,
    field: &'static str,
) -> Result<bool, SoilParserError> {
    match parse_i32(token, line_no, field)? {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(SoilParserError::new(
            SoilErrorCode::SolE005,
            line_no,
            field,
            "binary flag must be 0 or 1",
        )),
    }
}

fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() <= DATVER_EPSILON
}

struct LineCursor {
    lines: Vec<String>,
    index: usize,
}

impl LineCursor {
    fn new(input: &str, mode: ParserMode) -> Self {
        let lines = input
            .lines()
            .filter_map(|raw| {
                let trimmed = raw.trim();
                if trimmed.is_empty() {
                    return None;
                }
                if mode == ParserMode::Compatibility && trimmed.starts_with('#') {
                    return None;
                }
                Some(trimmed.to_string())
            })
            .collect();

        Self { lines, index: 0 }
    }

    fn next_line(&mut self) -> Option<(usize, &str)> {
        if self.index >= self.lines.len() {
            return None;
        }
        let line_no = self.index + 1;
        let line = self.lines[self.index].as_str();
        self.index += 1;
        Some((line_no, line))
    }

    fn current_line_number(&self) -> usize {
        self.index
    }

    fn peek_line(&self) -> Option<(usize, &str)> {
        if self.index >= self.lines.len() {
            return None;
        }
        let line_no = self.index + 1;
        let line = self.lines[self.index].as_str();
        Some((line_no, line))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn soil_input(datver: &str, policy_row: Option<&str>, layer_row: &str) -> String {
        let policy = policy_row.map_or_else(String::new, |row| format!("{row}\n"));
        let footer = if datver == "97.5" { "" } else { "1 500 0.8\n" };
        format!(
            "{datver}\nCQR soil profile\n1 1\nSOIL TEXT 1 0.20 0.55 1.0 0.01 4.0 0.0\n{policy}{layer_row}\n{footer}"
        )
    }

    fn strict_policy(datver: SoilDatver, policy_line: &str) -> DisturbedPolicy {
        parse_policy_row(datver, ParserMode::Strict, policy_line, 17)
            .expect("policy row should parse")
    }

    fn policy_error(datver: SoilDatver, policy_line: &str) -> SoilParserError {
        parse_policy_row(datver, ParserMode::Strict, policy_line, 17)
            .expect_err("policy row should fail")
    }

    #[test]
    fn cqr22_parse_policy_row_characterizes_success_variants() {
        assert_eq!(
            strict_policy(SoilDatver::V9002, "1 'forest use' 'silt loam' 2.5 0.75"),
            DisturbedPolicy::V9002 {
                ksatadj: true,
                luse: "forest use".to_string(),
                stext: "silt loam".to_string(),
                ksatfac_mm_h: 2.5,
                ksatrec_per_day: 0.75,
            }
        );

        assert_eq!(
            strict_policy(SoilDatver::V9003, "0 range 3 loam -9999"),
            DisturbedPolicy::V9003 {
                ksatadj: false,
                luse: "range".to_string(),
                burn_code: 3,
                stext: "loam".to_string(),
                lkeff_mm_h: -9999.0,
            }
        );

        assert_eq!(
            strict_policy(SoilDatver::V9003, "1 range 3 loam 0.25"),
            DisturbedPolicy::V9003 {
                ksatadj: true,
                luse: "range".to_string(),
                burn_code: 3,
                stext: "loam".to_string(),
                lkeff_mm_h: 0.25,
            }
        );

        assert_eq!(
            strict_policy(SoilDatver::V9005, "1 crop 2 sandy 7 12.5 -9999"),
            DisturbedPolicy::V9005 {
                ksatadj: true,
                luse: "crop".to_string(),
                burn_code: 2,
                stext: "sandy".to_string(),
                texid_enum: 7,
                uksat_mm_h: 12.5,
                lkeff_mm_h: -9999.0,
            }
        );
    }

    #[test]
    fn cqr22_parse_policy_row_characterizes_error_branches() {
        let err = policy_error(SoilDatver::V9002, "2 forest silt 2.5 0.75");
        assert_eq!(err.code, SoilErrorCode::SolE005);
        assert_eq!(err.field, "ksatadj");
        assert_eq!(err.message, "binary flag must be 0 or 1");

        let err = policy_error(SoilDatver::V9002, "1 forest silt 2.5");
        assert_eq!(err.code, SoilErrorCode::SolE006);
        assert_eq!(err.field, "ksatadj,luse,stext,ksatfac,ksatrec");
        assert_eq!(
            err.message,
            "variant arity mismatch: expected 5 token(s), found 4"
        );

        let err = policy_error(SoilDatver::V9003, "1 range -1 loam 0.25");
        assert_eq!(err.code, SoilErrorCode::SolE005);
        assert_eq!(err.field, "burn_code");
        assert_eq!(err.message, "burn_code must be non-negative");

        let err = policy_error(SoilDatver::V9003, "1 range 3 loam -0.25");
        assert_eq!(err.code, SoilErrorCode::SolE005);
        assert_eq!(err.field, "lkeff");
        assert_eq!(err.message, "value must be >= 0");

        let err = policy_error(SoilDatver::V9005, "1 crop -1 sandy 7 12.5 -9999");
        assert_eq!(err.code, SoilErrorCode::SolE005);
        assert_eq!(err.field, "burn_code");
        assert_eq!(err.message, "burn_code must be non-negative");

        let err = policy_error(SoilDatver::V9005, "1 crop 2 sandy 0 12.5 -9999");
        assert_eq!(err.code, SoilErrorCode::SolE005);
        assert_eq!(err.field, "texid_enum");
        assert_eq!(err.message, "texid_enum must be > 0");

        let err = policy_error(SoilDatver::V9005, "1 crop 2 sandy 7 -12.5 -9999");
        assert_eq!(err.code, SoilErrorCode::SolE005);
        assert_eq!(err.field, "uksat");
        assert_eq!(err.message, "value must be >= 0");

        let err = policy_error(SoilDatver::V97_5, "1 crop 2 sandy 7 -12.5 -9999");
        assert_eq!(err.code, SoilErrorCode::SolE006);
        assert_eq!(err.field, "policy-row");
        assert_eq!(err.message, "policy row is not applicable for this datver");
    }

    #[test]
    fn cqr23_soil_error_codes_preserve_every_public_label() {
        let labels = [
            (SoilErrorCode::SolE001, "SOL-E-001"),
            (SoilErrorCode::SolE002, "SOL-E-002"),
            (SoilErrorCode::SolE003, "SOL-E-003"),
            (SoilErrorCode::SolE004, "SOL-E-004"),
            (SoilErrorCode::SolE005, "SOL-E-005"),
            (SoilErrorCode::SolE006, "SOL-E-006"),
            (SoilErrorCode::SolE007, "SOL-E-007"),
            (SoilErrorCode::SolE008, "SOL-E-008"),
            (SoilErrorCode::SolE009, "SOL-E-009"),
        ];

        for (code, expected) in labels {
            assert_eq!(code.as_str(), expected);
            assert_eq!(code.to_string(), expected);
        }
    }

    #[test]
    fn cqr23_parse_soil_characterizes_base_and_extended_datver_families() {
        let base = soil_input("97.5", None, "100 50 20 2 10 5");
        let parsed = parse_soil(&base, SoilParserOptions::default()).expect("97.5 should parse");
        assert_eq!(parsed.datver, SoilDatver::V97_5);
        assert!(parsed.restrictive_layer.is_none());
        assert_eq!(parsed.ofes[0].layers[0].bulk_density_g_cm3, None);

        let version_2006 = soil_input("2006.2", None, "100 50 20 2 10 5");
        let parsed =
            parse_soil(&version_2006, SoilParserOptions::default()).expect("2006.2 should parse");
        assert_eq!(parsed.datver, SoilDatver::V2006_2);
        assert!(parsed.restrictive_layer.is_some());

        let version_7777 = soil_input("7777", None, "100 1.2 10 0.30 0.15 50 20 2 10 5");
        let parsed =
            parse_soil(&version_7777, SoilParserOptions::default()).expect("7777 should parse");
        let layer = &parsed.ofes[0].layers[0];
        assert_eq!(parsed.datver, SoilDatver::V7777);
        assert_eq!(layer.anisotropy_ratio, Some(1.0));
        assert_eq!(layer.fc_measured, Some(0.30));

        let version_7778 = soil_input("7778", None, "100 1.2 10 1.1 0.30 0.15 50 20 2 10 5");
        let parsed =
            parse_soil(&version_7778, SoilParserOptions::default()).expect("7778 should parse");
        let layer = &parsed.ofes[0].layers[0];
        assert_eq!(parsed.datver, SoilDatver::V7778);
        assert_eq!(layer.anisotropy_ratio, Some(1.1));
        assert_eq!(layer.wp_measured, Some(0.15));
    }

    #[test]
    fn cqr23_parse_soil_characterizes_disturbed_datver_families() {
        let layer = "100 1.2 10 1.1 0.30 0.15 50 20 2 10 5 0.05 0.45 0.02 1.4 120 0.16 0.31";
        let cases = [
            ("9002", "1 forest silt_loam 0.20 0.001", SoilDatver::V9002),
            ("9003", "1 range 3 loam -9999", SoilDatver::V9003),
            ("9005", "1 crop 2 sandy 7 12.5 -9999", SoilDatver::V9005),
        ];

        for (datver, policy, expected_datver) in cases {
            let input = soil_input(datver, Some(policy), layer);
            let parsed = parse_soil(&input, SoilParserOptions::default())
                .expect("disturbed datver should parse");
            let parsed_layer = &parsed.ofes[0].layers[0];
            assert_eq!(parsed.datver, expected_datver);
            assert!(parsed.ofes[0].policy.is_some());
            assert_eq!(parsed_layer.theta_r_rosetta, Some(0.05));
            assert_eq!(parsed_layer.theta_s_rosetta, Some(0.45));
            assert_eq!(parsed_layer.fc_rosetta, Some(0.31));
        }
    }

    fn assert_soil_error(
        error: SoilParserError,
        code: SoilErrorCode,
        line: usize,
        field: &'static str,
        message: &str,
    ) {
        let SoilParserError {
            code: observed_code,
            line: observed_line,
            field: observed_field,
            message: observed_message,
        } = error;
        assert_eq!(observed_code, code);
        assert_eq!(observed_line, line);
        assert_eq!(observed_field, field);
        assert_eq!(observed_message, message);
    }

    fn preamble_error(input: &str, options: SoilParserOptions) -> SoilParserError {
        let mut cursor = LineCursor::new(input, options.mode);
        match parse_soil_preamble(&mut cursor, options) {
            Err(error) => error,
            Ok(_) => panic!("input must fail"),
        }
    }

    fn ofe_header_error(input: &str, datver: SoilDatver) -> SoilParserError {
        let mut cursor = LineCursor::new(input, ParserMode::Strict);
        match parse_ofe_header_and_policy(&mut cursor, datver, ParserMode::Strict) {
            Err(error) => error,
            Ok(_) => panic!("invalid header/policy input must fail"),
        }
    }

    #[test]
    fn cqr23_datver_numeric_and_raw_alias_contracts_are_exact() {
        let canonical = [
            (SoilDatver::V97_5, 97.5),
            (SoilDatver::V2006_2, 2006.2),
            (SoilDatver::V7777, 7777.0),
            (SoilDatver::V7778, 7778.0),
            (SoilDatver::V9002, 9002.0),
            (SoilDatver::V9003, 9003.0),
            (SoilDatver::V9005, 9005.0),
        ];

        for (datver, raw) in canonical {
            assert_eq!(datver.numeric(), raw);
            assert_eq!(
                SoilDatver::from_raw(
                    raw + (DATVER_EPSILON / 2.0),
                    SoilParserOptions::default(),
                    8
                ),
                Ok((datver, false))
            );
        }

        let strict_error = SoilDatver::from_raw(97.0, SoilParserOptions::default(), 9)
            .expect_err("strict mode must reject the legacy alias");
        assert_soil_error(
            strict_error,
            SoilErrorCode::SolE003,
            9,
            "datver",
            "unsupported datver 97",
        );

        let compatibility = SoilParserOptions {
            mode: ParserMode::Compatibility,
            allow_legacy_aliases: true,
            ..SoilParserOptions::default()
        };
        assert_eq!(
            SoilDatver::from_raw(97.0, compatibility, 10),
            Ok((SoilDatver::V97_5, true))
        );
        assert_eq!(
            SoilDatver::from_raw(2006.0, compatibility, 11),
            Ok((SoilDatver::V2006_2, true))
        );
    }

    #[test]
    fn cqr23_primitive_parsers_validators_and_tokenizers_preserve_fail_closed_errors() {
        assert_eq!(parse_i32("-12", 3, "integer"), Ok(-12));
        assert_eq!(parse_usize("12", 3, "count"), Ok(12));
        assert_eq!(parse_f64("1.25", 3, "value"), Ok(1.25));
        assert_eq!(single_token(" one ", 3, "single"), Ok("one"));
        assert_eq!(tokens_exact("a b", 2, 3, "pair"), Ok(vec!["a", "b"]));
        assert_eq!(parse_binary_flag("0", 3, "flag"), Ok(false));
        assert_eq!(parse_binary_flag("1", 3, "flag"), Ok(true));

        assert_soil_error(
            parse_i32("twelve", 4, "integer").expect_err("non-integer must fail"),
            SoilErrorCode::SolE001,
            4,
            "integer",
            "failed to parse integer token 'twelve'",
        );
        assert_soil_error(
            parse_usize("-1", 5, "count").expect_err("negative usize must fail"),
            SoilErrorCode::SolE001,
            5,
            "count",
            "failed to parse usize token '-1'",
        );
        assert_soil_error(
            parse_f64("none", 6, "value").expect_err("non-float must fail"),
            SoilErrorCode::SolE001,
            6,
            "value",
            "failed to parse float token 'none'",
        );
        assert_soil_error(
            single_token("two tokens", 7, "single").expect_err("two tokens must fail"),
            SoilErrorCode::SolE006,
            7,
            "single",
            "expected exactly 1 token, found 2",
        );
        assert_soil_error(
            tokens_exact("a b", 3, 8, "triple").expect_err("short row must fail"),
            SoilErrorCode::SolE006,
            8,
            "triple",
            "variant arity mismatch: expected 3 token(s), found 2",
        );
        assert_soil_error(
            parse_binary_flag("2", 9, "flag").expect_err("nonbinary flag must fail"),
            SoilErrorCode::SolE005,
            9,
            "flag",
            "binary flag must be 0 or 1",
        );

        assert_eq!(validate_non_negative(0.0, 10, "positive-or-zero"), Ok(()));
        assert_eq!(validate_positive(0.25, 10, "positive"), Ok(()));
        assert_eq!(validate_percent(100.0, 10, "percent"), Ok(()));
        assert_eq!(validate_fraction_unit(1.0, 10, "fraction"), Ok(()));
        assert_soil_error(
            validate_non_negative(f64::NAN, 11, "finite").expect_err("NaN must fail"),
            SoilErrorCode::SolE005,
            11,
            "finite",
            "value must be finite",
        );
        assert_soil_error(
            validate_non_negative(-0.25, 12, "nonnegative").expect_err("negative must fail"),
            SoilErrorCode::SolE005,
            12,
            "nonnegative",
            "value must be >= 0",
        );
        assert_soil_error(
            validate_positive(0.0, 13, "positive").expect_err("zero must fail"),
            SoilErrorCode::SolE005,
            13,
            "positive",
            "value must be > 0",
        );
        assert_soil_error(
            validate_percent(100.1, 14, "percent").expect_err("large percent must fail"),
            SoilErrorCode::SolE005,
            14,
            "percent",
            "percent value must be <= 100",
        );
        assert_soil_error(
            validate_fraction_unit(1.1, 15, "fraction").expect_err("large fraction must fail"),
            SoilErrorCode::SolE005,
            15,
            "fraction",
            "fraction value must be <= 1",
        );

        assert_eq!(
            tokenize_whitespace_and_quotes("one 'two words' \"three\\\"words\"", 16, "quoted"),
            Ok(vec![
                "one".to_string(),
                "two words".to_string(),
                "three\"words".to_string(),
            ])
        );
        assert_soil_error(
            tokenize_whitespace_and_quotes("'not closed", 17, "quoted")
                .expect_err("unterminated quote must fail"),
            SoilErrorCode::SolE006,
            17,
            "quoted",
            "unterminated quoted token",
        );
        assert_soil_error(
            tokenize_whitespace_and_quotes("\"unfinished escape\\", 18, "quoted")
                .expect_err("unterminated escaped quote must fail"),
            SoilErrorCode::SolE006,
            18,
            "quoted",
            "unterminated quoted token",
        );
    }

    #[test]
    fn cqr23_line_cursor_strict_and_compatibility_record_selection_is_exact() {
        let input = "\n  first  \n# comment\n\n second \n";
        let mut strict = LineCursor::new(input, ParserMode::Strict);
        assert_eq!(strict.peek_line(), Some((1, "first")));
        assert_eq!(strict.next_line(), Some((1, "first")));
        assert_eq!(strict.current_line_number(), 1);
        assert_eq!(strict.next_line(), Some((2, "# comment")));
        assert_eq!(strict.next_line(), Some((3, "second")));
        assert_eq!(strict.next_line(), None);
        assert_eq!(strict.current_line_number(), 3);

        let mut compatibility = LineCursor::new(input, ParserMode::Compatibility);
        assert_eq!(compatibility.next_line(), Some((1, "first")));
        assert_eq!(compatibility.next_line(), Some((2, "second")));
        assert_eq!(compatibility.peek_line(), None);
        assert_eq!(compatibility.current_line_number(), 2);
    }

    #[test]
    fn cqr23_preamble_success_alias_and_failure_paths_are_stable() {
        let options = SoilParserOptions {
            mode: ParserMode::Compatibility,
            allow_legacy_aliases: true,
            expected_topology_count: Some(2),
            topology_scope: Some(TopologyScope::WatershedChannel),
        };
        let mut cursor = LineCursor::new("97\n  retained comment  \n2 1", options.mode);
        let preamble = parse_soil_preamble(&mut cursor, options).expect("preamble should parse");
        assert_eq!(preamble.datver, SoilDatver::V97_5);
        assert_eq!(preamble.datver_raw, 97.0);
        assert!(preamble.datver_alias_applied);
        assert_eq!(preamble.comment, "retained comment");
        assert_eq!(preamble.ntemp, 2);
        assert!(preamble.ksflag);

        let cases = [
            (
                "",
                SoilParserOptions::default(),
                SoilErrorCode::SolE002,
                0,
                "datver",
                "missing datver line",
            ),
            (
                "97.5 extra",
                SoilParserOptions::default(),
                SoilErrorCode::SolE006,
                1,
                "datver",
                "expected exactly 1 token, found 2",
            ),
            (
                "97.5",
                SoilParserOptions::default(),
                SoilErrorCode::SolE002,
                1,
                "solcom",
                "missing soil comment line",
            ),
            (
                "97.5\ncomment",
                SoilParserOptions::default(),
                SoilErrorCode::SolE002,
                1,
                "ntemp,ksflag",
                "missing ntemp/ksflag line",
            ),
            (
                "97.5\ncomment\n0 0",
                SoilParserOptions::default(),
                SoilErrorCode::SolE004,
                3,
                "ntemp",
                "ntemp must be > 0",
            ),
            (
                "97.5\ncomment\n1 2",
                SoilParserOptions::default(),
                SoilErrorCode::SolE005,
                3,
                "ksflag",
                "ksflag must be 0 or 1",
            ),
        ];

        for (input, options, code, line, field, message) in cases {
            let error = preamble_error(input, options);
            assert_soil_error(error, code, line, field, message);
        }

        let mismatch_options = SoilParserOptions {
            expected_topology_count: Some(2),
            topology_scope: Some(TopologyScope::Hillslope),
            ..SoilParserOptions::default()
        };
        let error = preamble_error("97.5\ncomment\n1 0", mismatch_options);
        assert_soil_error(
            error,
            SoilErrorCode::SolE007,
            3,
            "ntemp",
            "ntemp 1 does not match expected topology count 2 for hillslope",
        );
    }

    #[test]
    fn cqr23_ofe_header_policy_order_and_header_validation_are_stable() {
        let header = "SOIL TEXT 1 0.20 0.55 1.0 0.01 4.0 0.0";
        let mut cursor = LineCursor::new(header, ParserMode::Strict);
        let parsed =
            parse_ofe_header_and_policy(&mut cursor, SoilDatver::V97_5, ParserMode::Strict)
                .expect("base header should parse");
        assert_eq!(parsed.slid, "SOIL");
        assert_eq!(parsed.texid, "TEXT");
        assert_eq!(parsed.nsl, 1);
        assert_eq!(parsed.salb, 0.20);
        assert_eq!(parsed.avke, 0.0);
        assert!(parsed.policy.is_none());

        let mut policy_first = LineCursor::new(
            "1 'forest use' 'silt loam' 2.5 0.75\n'SOIL ID' 'TEXT ID' 1 0.20 0.55 1.0 0.01 4.0",
            ParserMode::Strict,
        );
        let parsed =
            parse_ofe_header_and_policy(&mut policy_first, SoilDatver::V9002, ParserMode::Strict)
                .expect("policy-first quoted header should parse");
        assert_eq!(parsed.slid, "SOIL ID");
        assert_eq!(parsed.texid, "TEXT ID");
        assert_eq!(parsed.avke, 0.0);
        assert_eq!(
            parsed.policy,
            Some(DisturbedPolicy::V9002 {
                ksatadj: true,
                luse: "forest use".to_string(),
                stext: "silt loam".to_string(),
                ksatfac_mm_h: 2.5,
                ksatrec_per_day: 0.75,
            })
        );

        let mut header_first = LineCursor::new(
            "SOIL TEXT 1 0.20 0.55 1.0 0.01 4.0 0.0\n1 forest silt 2.5 0.75",
            ParserMode::Strict,
        );
        let parsed =
            parse_ofe_header_and_policy(&mut header_first, SoilDatver::V9002, ParserMode::Strict)
                .expect("header-first policy should parse");
        assert!(matches!(parsed.policy, Some(DisturbedPolicy::V9002 { .. })));

        let cases = [
            (
                "",
                SoilDatver::V97_5,
                SoilErrorCode::SolE002,
                0,
                "slid,texid,nsl,salb,sat,ki,kr,shcrit,avke",
                "missing OFE header line",
            ),
            (
                "1 forest silt 2.5 0.75",
                SoilDatver::V9002,
                SoilErrorCode::SolE002,
                1,
                "slid,texid,nsl,salb,sat,ki,kr,shcrit,avke",
                "missing OFE header line after policy row",
            ),
            (
                "SOIL TEXT 1 0.20 0.55 1.0 0.01 4.0 0.0",
                SoilDatver::V9002,
                SoilErrorCode::SolE002,
                1,
                "policy-row",
                "missing datver-specific policy row",
            ),
            (
                "SOIL TEXT 0 0.20 0.55 1.0 0.01 4.0 0.0",
                SoilDatver::V97_5,
                SoilErrorCode::SolE004,
                1,
                "nsl",
                "nsl must be > 0",
            ),
            (
                "SOIL TEXT 1 1.20 0.55 1.0 0.01 4.0 0.0",
                SoilDatver::V97_5,
                SoilErrorCode::SolE005,
                1,
                "salb",
                "fraction value must be <= 1",
            ),
        ];

        for (input, datver, code, line, field, message) in cases {
            let error = ofe_header_error(input, datver);
            assert_soil_error(error, code, line, field, message);
        }
    }

    #[test]
    fn cqr23_layer_depth_missing_and_restrictive_footer_paths_are_stable() {
        let mut good_layers =
            LineCursor::new("100 50 20 2 10 5\n200 45 25 2 12 3", ParserMode::Strict);
        let layers = parse_ofe_layers(&mut good_layers, SoilDatver::V97_5, 2)
            .expect("strictly increasing layers should parse");
        assert_eq!(layers.len(), 2);
        assert_eq!(layers[1].depth_mm, 200.0);

        let cases = [
            (
                "",
                SoilErrorCode::SolE002,
                0,
                "layer-row",
                "missing layer row",
            ),
            (
                "0 50 20 2 10 5",
                SoilErrorCode::SolE005,
                1,
                "solthk",
                "layer depth must be > 0",
            ),
            (
                "100 50 20 2 10 5\n100 45 25 2 12 3",
                SoilErrorCode::SolE009,
                2,
                "solthk",
                "layer depths must be strictly increasing",
            ),
        ];
        for (input, code, line, field, message) in cases {
            let mut cursor = LineCursor::new(input, ParserMode::Strict);
            let requested = if input.is_empty() {
                1
            } else if input.contains('\n') {
                2
            } else {
                1
            };
            let error = parse_ofe_layers(&mut cursor, SoilDatver::V97_5, requested)
                .expect_err("invalid layer sequence must fail");
            assert_soil_error(error, code, line, field, message);
        }

        let restrictive = RestrictiveLayer {
            slflag: true,
            ui_bdrkth_mm: 500.0,
            kslast_mm_h: 0.8,
        };
        let mut no_footer_needed = LineCursor::new("unconsumed", ParserMode::Strict);
        assert_eq!(
            parse_restrictive_footer(&mut no_footer_needed, SoilDatver::V97_5, None),
            Ok(None)
        );
        assert_eq!(no_footer_needed.next_line(), Some((1, "unconsumed")));

        let mut footer = LineCursor::new("1 500 0.8", ParserMode::Strict);
        assert_eq!(
            parse_restrictive_footer(&mut footer, SoilDatver::V2006_2, None),
            Ok(Some(restrictive.clone()))
        );
        assert_eq!(footer.next_line(), None);

        let mut missing_footer = LineCursor::new("", ParserMode::Strict);
        let error = parse_restrictive_footer(&mut missing_footer, SoilDatver::V9002, None)
            .expect_err("required footer must be present");
        assert_soil_error(
            error,
            SoilErrorCode::SolE002,
            0,
            "slflag,ui_bdrkth,kslast",
            "missing restrictive-layer footer",
        );

        let mut matching_trailing = LineCursor::new("1 500 0.8\nextra", ParserMode::Strict);
        assert_eq!(
            parse_restrictive_footer(
                &mut matching_trailing,
                SoilDatver::V9002,
                Some(restrictive.clone()),
            ),
            Ok(Some(restrictive.clone()))
        );
        assert_eq!(matching_trailing.next_line(), Some((2, "extra")));

        let mut conflicting_trailing = LineCursor::new("1 499 0.8", ParserMode::Strict);
        let error = parse_restrictive_footer(
            &mut conflicting_trailing,
            SoilDatver::V9002,
            Some(restrictive),
        )
        .expect_err("conflicting trailing row must fail");
        assert_soil_error(
            error,
            SoilErrorCode::SolE006,
            1,
            "slflag,ui_bdrkth,kslast",
            "trailing restrictive row conflicts with per-OFE restrictive rows",
        );
    }

    #[test]
    fn cqr23_restrictive_row_detection_and_file_tail_rejection_are_stable() {
        let mut no_three_token_row = LineCursor::new("a b c d", ParserMode::Strict);
        assert_eq!(
            maybe_parse_ofe_restrictive_row(
                &mut no_three_token_row,
                SoilDatver::V9003,
                ParserMode::Strict,
            ),
            Ok(None)
        );
        assert_eq!(no_three_token_row.peek_line(), Some((1, "a b c d")));

        let mut per_ofe = LineCursor::new("0 -10 -2", ParserMode::Strict);
        assert_eq!(
            maybe_parse_ofe_restrictive_row(&mut per_ofe, SoilDatver::V9003, ParserMode::Strict),
            Ok(Some(RestrictiveLayer {
                slflag: false,
                ui_bdrkth_mm: -10.0,
                kslast_mm_h: -2.0,
            }))
        );
        assert_eq!(per_ofe.next_line(), None);

        let mut bad_restrictive = LineCursor::new("2 10 1", ParserMode::Strict);
        let error = maybe_parse_ofe_restrictive_row(
            &mut bad_restrictive,
            SoilDatver::V9003,
            ParserMode::Strict,
        )
        .expect_err("invalid restrictive flag must fail");
        assert_soil_error(
            error,
            SoilErrorCode::SolE009,
            1,
            "slflag",
            "slflag must be 0 or 1",
        );

        let mut tail = LineCursor::new("extra record", ParserMode::Strict);
        let error = reject_trailing_records(&mut tail).expect_err("tail must fail");
        assert_soil_error(
            error,
            SoilErrorCode::SolE006,
            1,
            "file-tail",
            "unexpected trailing records",
        );
        let mut empty = LineCursor::new("", ParserMode::Strict);
        assert_eq!(reject_trailing_records(&mut empty), Ok(()));
    }

    #[test]
    fn cqr23_parser_error_display_and_token_helpers_preserve_exact_contracts() {
        let error = SoilParserError::new(
            SoilErrorCode::SolE006,
            12,
            "layer-row",
            "variant arity mismatch",
        );
        assert_eq!(
            error.to_string(),
            "SOL-E-006 at line 12 field layer-row: variant arity mismatch"
        );

        assert_eq!(
            parse_policy_tokens(
                "1 forest silt 2.5 0.75",
                5,
                8,
                "policy-row",
                SoilDatver::V9002,
            ),
            Ok(vec![
                "1".to_string(),
                "forest".to_string(),
                "silt".to_string(),
                "2.5".to_string(),
                "0.75".to_string(),
            ])
        );
        assert_eq!(
            parse_policy_tokens(
                "1 'forest use' 'silt loam' 2.5 0.75",
                5,
                8,
                "policy-row",
                SoilDatver::V9002,
            ),
            Ok(vec![
                "1".to_string(),
                "forest use".to_string(),
                "silt loam".to_string(),
                "2.5".to_string(),
                "0.75".to_string(),
            ])
        );
        assert_eq!(
            parse_policy_tokens(
                "1 \"forest use\" \"silt loam\" 2.5 0.75",
                5,
                8,
                "policy-row",
                SoilDatver::V9002,
            ),
            Ok(vec![
                "1".to_string(),
                "forest use".to_string(),
                "silt loam".to_string(),
                "2.5".to_string(),
                "0.75".to_string(),
            ])
        );
        let error = parse_policy_tokens("1 \"forest use\"", 5, 8, "policy-row", SoilDatver::V9002)
            .expect_err("short quoted policy row must fail");
        assert_soil_error(
            error,
            SoilErrorCode::SolE006,
            8,
            "policy-row",
            "variant arity mismatch: expected 5 token(s), found 2",
        );
        let error = parse_policy_tokens("1 forest", 5, 8, "policy-row", SoilDatver::V9002)
            .expect_err("short policy row must fail");
        assert_soil_error(
            error,
            SoilErrorCode::SolE006,
            8,
            "policy-row",
            "variant arity mismatch: expected 5 token(s), found 2",
        );

        assert_eq!(
            parse_ofe_header_tokens(
                "SOIL TEXT 1 0.2 0.5 1 0.01 4 0",
                9,
                SoilDatver::V97_5,
                ParserMode::Strict,
                "header",
            ),
            Ok(vec![
                "SOIL".to_string(),
                "TEXT".to_string(),
                "1".to_string(),
                "0.2".to_string(),
                "0.5".to_string(),
                "1".to_string(),
                "0.01".to_string(),
                "4".to_string(),
                "0".to_string(),
            ])
        );
        assert_eq!(
            parse_ofe_header_tokens(
                "'SOIL ID' 'TEXT ID' 1 0.2 0.5 1 0.01 4",
                9,
                SoilDatver::V7778,
                ParserMode::Strict,
                "header",
            ),
            Ok(vec![
                "SOIL ID".to_string(),
                "TEXT ID".to_string(),
                "1".to_string(),
                "0.2".to_string(),
                "0.5".to_string(),
                "1".to_string(),
                "0.01".to_string(),
                "4".to_string(),
                "0.0".to_string(),
            ])
        );
        let error = parse_ofe_header_tokens(
            "'SOIL ID' TEXT 1 0.2",
            9,
            SoilDatver::V7778,
            ParserMode::Strict,
            "header",
        )
        .expect_err("short quoted header must fail");
        assert_soil_error(
            error,
            SoilErrorCode::SolE006,
            9,
            "header",
            "variant arity mismatch: expected 9 token(s), found 4",
        );
    }

    #[test]
    fn cqr23_ofe_collection_preserves_restrictive_identity_closure() {
        let profile = "SOIL TEXT 1 0.20 0.55 1.0 0.01 4.0 0.0\n100 1.2 10 1.1 0.30 0.15 50 20 2 10 5\n1 500 0.8";
        let mut matching = LineCursor::new(&format!("{profile}\n{profile}"), ParserMode::Strict);
        let (ofes, restrictive) =
            parse_soil_ofes(&mut matching, SoilDatver::V7778, 2, ParserMode::Strict)
                .expect("matching per-OFE restrictive rows should close");
        assert_eq!(ofes.len(), 2);
        assert_eq!(
            restrictive,
            Some(RestrictiveLayer {
                slflag: true,
                ui_bdrkth_mm: 500.0,
                kslast_mm_h: 0.8,
            })
        );

        let conflicting_profile = format!(
            "{profile}\nSOIL TEXT 1 0.20 0.55 1.0 0.01 4.0 0.0\n100 1.2 10 1.1 0.30 0.15 50 20 2 10 5\n1 499 0.8"
        );
        let mut conflicting = LineCursor::new(&conflicting_profile, ParserMode::Strict);
        let error = parse_soil_ofes(&mut conflicting, SoilDatver::V7778, 2, ParserMode::Strict)
            .expect_err("non-identical per-OFE restrictive rows must fail");
        assert_soil_error(
            error,
            SoilErrorCode::SolE006,
            6,
            "slflag,ui_bdrkth,kslast",
            "per-OFE restrictive rows must be identical",
        );
    }
}
