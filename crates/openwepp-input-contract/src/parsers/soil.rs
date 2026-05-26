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

pub fn parse_soil(input: &str, options: SoilParserOptions) -> Result<SoilProfile, SoilParserError> {
    let mut cursor = LineCursor::new(input, options.mode);

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

    let mut ofes = Vec::with_capacity(ntemp);
    let mut compat_per_ofe_restrictive: Option<RestrictiveLayer> = None;
    for _ in 0..ntemp {
        let (ofe, ofe_restrictive) = parse_ofe_block(&mut cursor, datver, options.mode)?;
        if let Some(ofe_restrictive) = ofe_restrictive {
            if let Some(existing) = &compat_per_ofe_restrictive {
                if existing != &ofe_restrictive {
                    return Err(SoilParserError::new(
                        SoilErrorCode::SolE006,
                        cursor.current_line_number(),
                        "slflag,ui_bdrkth,kslast",
                        "compatibility per-OFE restrictive rows must be identical",
                    ));
                }
            } else {
                compat_per_ofe_restrictive = Some(ofe_restrictive);
            }
        }
        ofes.push(ofe);
    }

    let restrictive_layer = if datver.requires_restrictive_footer() {
        if options.mode == ParserMode::Compatibility {
            if let Some(per_ofe_restrictive) = compat_per_ofe_restrictive {
                if let Some((line_no, line)) = cursor.peek_line() {
                    if line.split_whitespace().count() == 3 {
                        let trailing_restrictive = parse_restrictive_layer(line, line_no)?;
                        if trailing_restrictive != per_ofe_restrictive {
                            return Err(SoilParserError::new(
                                SoilErrorCode::SolE006,
                                line_no,
                                "slflag,ui_bdrkth,kslast",
                                "compatibility trailing restrictive row conflicts with per-OFE restrictive rows",
                            ));
                        }
                        cursor.next_line();
                    }
                }
                Some(per_ofe_restrictive)
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
                Some(parse_restrictive_layer(footer_line, footer_line_no)?)
            }
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
            Some(parse_restrictive_layer(footer_line, footer_line_no)?)
        }
    } else {
        None
    };

    if let Some((line_no_extra, _)) = cursor.next_line() {
        return Err(SoilParserError::new(
            SoilErrorCode::SolE006,
            line_no_extra,
            "file-tail",
            "unexpected trailing records",
        ));
    }

    Ok(SoilProfile {
        datver,
        datver_raw,
        datver_alias_applied,
        comment,
        ntemp,
        ksflag,
        ofes,
        restrictive_layer,
    })
}

fn parse_ofe_block(
    cursor: &mut LineCursor,
    datver: SoilDatver,
    mode: ParserMode,
) -> Result<(SoilOfe, Option<RestrictiveLayer>), SoilParserError> {
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
    let (header_line_no, header_line) =
        if datver.requires_policy_row() && mode == ParserMode::Compatibility {
            match parse_policy_row(datver, first_line, first_line_no) {
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
        policy = Some(parse_policy_row(datver, policy_line, policy_line_no)?);
    }

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

    let compat_restrictive = maybe_parse_compat_ofe_restrictive_row(cursor, datver, mode)?;

    Ok((
        SoilOfe {
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
            layers,
        },
        compat_restrictive,
    ))
}

fn maybe_parse_compat_ofe_restrictive_row(
    cursor: &mut LineCursor,
    datver: SoilDatver,
    mode: ParserMode,
) -> Result<Option<RestrictiveLayer>, SoilParserError> {
    if mode != ParserMode::Compatibility || !datver.requires_restrictive_footer() {
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

fn parse_policy_row(
    datver: SoilDatver,
    policy_line: &str,
    line_no: usize,
) -> Result<DisturbedPolicy, SoilParserError> {
    match datver {
        SoilDatver::V9002 => {
            let t = tokens_exact(
                policy_line,
                5,
                line_no,
                "ksatadj,luse,stext,ksatfac,ksatrec",
            )?;
            let ksatadj = parse_binary_flag(t[0], line_no, "ksatadj")?;
            let ksatfac = parse_f64(t[3], line_no, "ksatfac")?;
            let ksatrec = parse_f64(t[4], line_no, "ksatrec")?;
            validate_non_negative(ksatfac, line_no, "ksatfac")?;
            validate_non_negative(ksatrec, line_no, "ksatrec")?;

            Ok(DisturbedPolicy::V9002 {
                ksatadj,
                luse: t[1].to_string(),
                stext: t[2].to_string(),
                ksatfac_mm_h: ksatfac,
                ksatrec_per_day: ksatrec,
            })
        }
        SoilDatver::V9003 => {
            let t = tokens_exact(
                policy_line,
                5,
                line_no,
                "ksatadj,luse,burn_code,stext,lkeff",
            )?;
            let ksatadj = parse_binary_flag(t[0], line_no, "ksatadj")?;
            let burn_code = parse_i32(t[2], line_no, "burn_code")?;
            if burn_code < 0 {
                return Err(SoilParserError::new(
                    SoilErrorCode::SolE005,
                    line_no,
                    "burn_code",
                    "burn_code must be non-negative",
                ));
            }

            let lkeff = parse_f64(t[4], line_no, "lkeff")?;
            if lkeff != -9999.0 {
                validate_non_negative(lkeff, line_no, "lkeff")?;
            }

            Ok(DisturbedPolicy::V9003 {
                ksatadj,
                luse: t[1].to_string(),
                burn_code,
                stext: t[3].to_string(),
                lkeff_mm_h: lkeff,
            })
        }
        SoilDatver::V9005 => {
            let t = tokens_exact(
                policy_line,
                7,
                line_no,
                "ksatadj,luse,burn_code,stext,texid_enum,uksat,lkeff",
            )?;
            let ksatadj = parse_binary_flag(t[0], line_no, "ksatadj")?;
            let burn_code = parse_i32(t[2], line_no, "burn_code")?;
            if burn_code < 0 {
                return Err(SoilParserError::new(
                    SoilErrorCode::SolE005,
                    line_no,
                    "burn_code",
                    "burn_code must be non-negative",
                ));
            }

            let texid_enum = parse_i32(t[4], line_no, "texid_enum")?;
            if texid_enum <= 0 {
                return Err(SoilParserError::new(
                    SoilErrorCode::SolE005,
                    line_no,
                    "texid_enum",
                    "texid_enum must be > 0",
                ));
            }

            let uksat = parse_f64(t[5], line_no, "uksat")?;
            validate_non_negative(uksat, line_no, "uksat")?;

            let lkeff = parse_f64(t[6], line_no, "lkeff")?;
            if lkeff != -9999.0 {
                validate_non_negative(lkeff, line_no, "lkeff")?;
            }

            Ok(DisturbedPolicy::V9005 {
                ksatadj,
                luse: t[1].to_string(),
                burn_code,
                stext: t[3].to_string(),
                texid_enum,
                uksat_mm_h: uksat,
                lkeff_mm_h: lkeff,
            })
        }
        _ => Err(SoilParserError::new(
            SoilErrorCode::SolE006,
            line_no,
            "policy-row",
            "policy row is not applicable for this datver",
        )),
    }
}

fn parse_layer_row(
    datver: SoilDatver,
    row: &str,
    line_no: usize,
) -> Result<SoilLayer, SoilParserError> {
    let expected_arity = datver.layer_arity();
    let t = tokens_exact(row, expected_arity, line_no, "layer-row")?;

    match datver {
        SoilDatver::V97_5 | SoilDatver::V2006_2 => {
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
        SoilDatver::V7777 => {
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
        SoilDatver::V7778 => {
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
        SoilDatver::V9002 | SoilDatver::V9003 | SoilDatver::V9005 => {
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
    }
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
    mode: ParserMode,
    field: &'static str,
) -> Result<Vec<String>, SoilParserError> {
    if mode == ParserMode::Compatibility
        && matches!(
            datver,
            SoilDatver::V7778 | SoilDatver::V9002 | SoilDatver::V9003 | SoilDatver::V9005
        )
        && line.contains('\'')
    {
        let mut tokens = tokenize_whitespace_and_single_quotes(line, line_no, field)?;
        if tokens.len() == 8 {
            // Legacy quoted compatibility rows can omit `avke`; normalize to explicit 0.0.
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

fn tokenize_whitespace_and_single_quotes(
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

        if ch == '\'' {
            iter.next();
            let quoted_start = idx + ch.len_utf8();
            let mut end = None;

            for (quote_idx, quote_ch) in iter.by_ref() {
                if quote_ch == '\'' {
                    end = Some(quote_idx);
                    break;
                }
            }

            let quoted_end = end.ok_or_else(|| {
                SoilParserError::new(
                    SoilErrorCode::SolE006,
                    line_no,
                    field,
                    "unterminated quoted header token",
                )
            })?;
            tokens.push(line[quoted_start..quoted_end].to_string());
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
