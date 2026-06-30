pub(super) const MOFE03_WAVE2_ENABLE_TOLERANCE: f64 = 1.0e-9;
pub(super) const MOFE03_WAVE2_MIN_POSITIVE: f64 = 1.0e-6;
pub(super) const MOFE03_WAVE2_DEFAULT_XTOP: f64 = 0.2;
pub(super) const MOFE03_WAVE2_DEFAULT_XBOT: f64 = 0.5;
pub(super) const MOFE03_WAVE2_DEFAULT_XDETST: f64 = 0.1;
pub(super) const MOFE03_WAVE2_DEFAULT_LDTOP: f64 = 0.8;
pub(super) const MOFE03_WAVE2_DEFAULT_LDBOT: f64 = 0.6;
pub(super) const MOFE03_WAVE2_DEFAULT_LDDEND: f64 = 0.0;
pub(super) const MOFE03_WAVE2_DEFAULT_KTRATO: f64 = 1.1;
pub(super) const MOFE03_WAVE2_DEFAULT_AINTC: f64 = 0.4;
pub(super) const MOFE03_WAVE2_DEFAULT_BINTC: f64 = 0.3;
pub(super) const MOFE03_WAVE2_DEFAULT_CINTC: f64 = 0.2;
pub(super) const MOFE03_WAVE2_DEFAULT_BETA: f64 = 0.5;
pub(super) const MOFE03_WAVE2_DEFAULT_QOSTAR: f64 = 0.2;
pub(super) const MOFE03_WAVE2_DEFAULT_SSA_SOIL: f64 = 5.0;
pub(super) const MOFE03_WAVE2_PARTICLE_CLASS_COUNT: usize = 5;
pub(super) const MOFE03_ROUTE_SEGMENT_INDEX: usize = 2;

#[derive(Debug, Clone, Copy)]
pub(super) struct Mofe03Wave2CaseScalars {
    pub(super) case_value: f64,
    pub(super) qj_minus_1: f64,
    pub(super) vj: f64,
    pub(super) qj: f64,
    pub(super) fh: f64,
    pub(super) fp: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TypedMofe03Wave2ClassProjection {
    pub(crate) fall_m_s: f64,
    pub(crate) frcflw: f64,
    pub(crate) frac: f64,
    pub(crate) fidel: f64,
    pub(crate) tcf1: f64,
    pub(crate) ssa_class: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TypedMofe03Wave2Projection {
    pub(crate) wave2_enabled: bool,
    pub(crate) class_count: usize,
    pub(crate) xtop_m: f64,
    pub(crate) xbot_m: f64,
    pub(crate) xdetst_m: f64,
    pub(crate) ldtop_kg_s_m: f64,
    pub(crate) ldbot_kg_s_m: f64,
    pub(crate) lddend_kg: f64,
    pub(crate) qout_m3_s: f64,
    pub(crate) qin_m3_s: f64,
    pub(crate) qostar_m: f64,
    pub(crate) slplen_m: f64,
    pub(crate) ktrato: f64,
    pub(crate) aintc: f64,
    pub(crate) bintc: f64,
    pub(crate) cintc: f64,
    pub(crate) beta: f64,
    pub(crate) theta: f64,
    pub(crate) qj_minus_1_m3_s: f64,
    pub(crate) vj_m: f64,
    pub(crate) qj_m3_s: f64,
    pub(crate) fh_m: f64,
    pub(crate) fp_m: f64,
    pub(crate) case_value: f64,
    pub(crate) ssa_soil: f64,
    pub(crate) route_qostar_m: f64,
    pub(crate) route_xdetst_m: f64,
    pub(crate) route_lddend_kg: f64,
    pub(crate) route_xu_m: f64,
    pub(crate) route_xl_m: f64,
    pub(crate) route_ainf: f64,
    pub(crate) route_binf: f64,
    pub(crate) route_cinf: f64,
    pub(crate) route_ainftc: f64,
    pub(crate) route_binftc: f64,
    pub(crate) route_cinftc: f64,
    pub(crate) classes: Vec<TypedMofe03Wave2ClassProjection>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct TypedMofe03Wave2Input {
    pub(crate) wave2_enabled: bool,
    pub(crate) slplen_m: f64,
    pub(crate) qout_m3_s: f64,
    pub(crate) qin_m3_s: f64,
    pub(crate) efflen_m: Option<f64>,
    pub(crate) ssa_soil: Option<f64>,
    pub(crate) beta: f64,
    pub(crate) theta: f64,
}

pub(super) fn seed_mofe03_wave2_runtime_surface_inputs(
    runtime_surface: &mut HillslopeWritebackSurface,
) -> Result<(), HillslopeCliError> {
    let ofe_count = resolve_mofe03_ofe_count(runtime_surface)?;
    let wave2_enabled = resolve_mofe03_wave2_enabled(runtime_surface, ofe_count)?;
    write_mofe03_wave2_enabled(runtime_surface, wave2_enabled);
    if !wave2_enabled {
        return Ok(());
    }

    let slplen_m = require_mofe03_positive_runtime_surface_scalar(
        runtime_surface,
        "slplen",
        "Wave-2 seeding",
    )?;
    let qout = resolve_mofe03_wave2_qout(runtime_surface)?;
    let qin = resolve_mofe03_wave2_qin(runtime_surface)?;
    let (beta, theta) = resolve_mofe03_wave2_beta_theta(runtime_surface)?;
    let projection = project_typed_mofe03_wave2(TypedMofe03Wave2Input {
        wave2_enabled,
        slplen_m,
        qout_m3_s: qout,
        qin_m3_s: qin,
        efflen_m: runtime_surface_symbol_value(runtime_surface, "efflen"),
        ssa_soil: runtime_surface_symbol_value(runtime_surface, "erod14_ssa_soil"),
        beta,
        theta,
    })?;
    publish_typed_mofe03_wave2_projection(runtime_surface, &projection)?;
    Ok(())
}

pub(crate) fn project_typed_mofe03_wave2(
    input: TypedMofe03Wave2Input,
) -> Result<TypedMofe03Wave2Projection, HillslopeCliError> {
    if !input.wave2_enabled {
        return Ok(TypedMofe03Wave2Projection::disabled());
    }
    require_mofe03_positive_seed_scalar(input.slplen_m, "slplen")?;
    let qout_m3_s = require_mofe03_non_negative_seed_scalar(input.qout_m3_s, "erod14_qout")?;
    let qin_m3_s = require_mofe03_non_negative_seed_scalar(input.qin_m3_s, "erod14_qin")?;
    let qostar = project_typed_mofe03_wave2_qostar(
        input.slplen_m,
        qout_m3_s,
        qin_m3_s,
        input.efflen_m,
    )?;
    let ssa_soil = match input.ssa_soil {
        Some(value) => require_mofe03_positive_seed_scalar(value, "erod14_ssa_soil")?,
        None => MOFE03_WAVE2_DEFAULT_SSA_SOIL,
    };
    let beta = require_mofe03_non_negative_seed_scalar(input.beta, "beta")?;
    let theta = require_mofe03_non_negative_seed_scalar(input.theta, "theta")?;
    let case_scalars = build_mofe03_wave2_case_scalars(qout_m3_s);
    let route_qostar_m = qostar.max(MOFE03_WAVE2_DEFAULT_QOSTAR);
    Ok(TypedMofe03Wave2Projection {
        wave2_enabled: true,
        class_count: MOFE03_WAVE2_PARTICLE_CLASS_COUNT,
        xtop_m: MOFE03_WAVE2_DEFAULT_XTOP,
        xbot_m: MOFE03_WAVE2_DEFAULT_XBOT,
        xdetst_m: MOFE03_WAVE2_DEFAULT_XDETST,
        ldtop_kg_s_m: MOFE03_WAVE2_DEFAULT_LDTOP,
        ldbot_kg_s_m: MOFE03_WAVE2_DEFAULT_LDBOT,
        lddend_kg: MOFE03_WAVE2_DEFAULT_LDDEND,
        qout_m3_s,
        qin_m3_s,
        qostar_m: qostar,
        slplen_m: input.slplen_m,
        ktrato: MOFE03_WAVE2_DEFAULT_KTRATO,
        aintc: MOFE03_WAVE2_DEFAULT_AINTC,
        bintc: MOFE03_WAVE2_DEFAULT_BINTC,
        cintc: MOFE03_WAVE2_DEFAULT_CINTC,
        beta,
        theta,
        qj_minus_1_m3_s: case_scalars.qj_minus_1,
        vj_m: case_scalars.vj,
        qj_m3_s: case_scalars.qj,
        fh_m: case_scalars.fh,
        fp_m: case_scalars.fp,
        case_value: case_scalars.case_value,
        ssa_soil,
        route_qostar_m,
        route_xdetst_m: MOFE03_WAVE2_DEFAULT_XDETST,
        route_lddend_kg: MOFE03_WAVE2_DEFAULT_LDDEND,
        route_xu_m: MOFE03_WAVE2_DEFAULT_XTOP,
        route_xl_m: MOFE03_WAVE2_DEFAULT_XBOT,
        route_ainf: MOFE03_WAVE2_DEFAULT_AINTC,
        route_binf: MOFE03_WAVE2_DEFAULT_BINTC,
        route_cinf: MOFE03_WAVE2_DEFAULT_CINTC,
        route_ainftc: MOFE03_WAVE2_DEFAULT_AINTC,
        route_binftc: MOFE03_WAVE2_DEFAULT_BINTC,
        route_cinftc: MOFE03_WAVE2_DEFAULT_CINTC,
        classes: project_typed_mofe03_wave2_classes()?,
    })
}

impl TypedMofe03Wave2Projection {
    fn disabled() -> Self {
        Self {
            wave2_enabled: false,
            class_count: 0,
            xtop_m: 0.0,
            xbot_m: 0.0,
            xdetst_m: 0.0,
            ldtop_kg_s_m: 0.0,
            ldbot_kg_s_m: 0.0,
            lddend_kg: 0.0,
            qout_m3_s: 0.0,
            qin_m3_s: 0.0,
            qostar_m: 0.0,
            slplen_m: 0.0,
            ktrato: 0.0,
            aintc: 0.0,
            bintc: 0.0,
            cintc: 0.0,
            beta: 0.0,
            theta: 0.0,
            qj_minus_1_m3_s: 0.0,
            vj_m: 0.0,
            qj_m3_s: 0.0,
            fh_m: 0.0,
            fp_m: 0.0,
            case_value: 0.0,
            ssa_soil: 0.0,
            route_qostar_m: 0.0,
            route_xdetst_m: 0.0,
            route_lddend_kg: 0.0,
            route_xu_m: 0.0,
            route_xl_m: 0.0,
            route_ainf: 0.0,
            route_binf: 0.0,
            route_cinf: 0.0,
            route_ainftc: 0.0,
            route_binftc: 0.0,
            route_cinftc: 0.0,
            classes: Vec::new(),
        }
    }
}

fn project_typed_mofe03_wave2_qostar(
    slplen_m: f64,
    qout_m3_s: f64,
    qin_m3_s: f64,
    efflen_m: Option<f64>,
) -> Result<f64, HillslopeCliError> {
    let del = qout_m3_s - qin_m3_s;
    let qostar = if qout_m3_s <= 0.0 {
        let efflen_m = efflen_m.unwrap_or(slplen_m);
        require_mofe03_positive_seed_scalar(efflen_m, "efflen")?;
        -efflen_m / slplen_m
    } else if del.abs() > 1.0e-10 {
        if qin_m3_s <= 0.0 { 0.0 } else { qin_m3_s / del }
    } else if del >= 0.0 {
        qin_m3_s / 1.0e-10
    } else {
        -qin_m3_s / 1.0e-10
    };
    require_mofe03_seed_scalar(qostar, "erod14_qostar")?;
    Ok(if (qostar + 1.0).abs() <= MOFE03_WAVE2_ENABLE_TOLERANCE {
        -1.001
    } else {
        qostar
    })
}

fn project_typed_mofe03_wave2_classes(
) -> Result<Vec<TypedMofe03Wave2ClassProjection>, HillslopeCliError> {
    let class_count_f64 = usize_to_scalar("erod14_class_count", MOFE03_WAVE2_PARTICLE_CLASS_COUNT)?;
    let class_fraction = 1.0 / class_count_f64;
    let mut classes = Vec::with_capacity(MOFE03_WAVE2_PARTICLE_CLASS_COUNT);
    for class_index in 1..=MOFE03_WAVE2_PARTICLE_CLASS_COUNT {
        let class_index_f64 = usize_to_scalar("erod14_class_index", class_index)?;
        let reverse_class_index = MOFE03_WAVE2_PARTICLE_CLASS_COUNT.saturating_sub(class_index) + 1;
        let reverse_class_index_f64 =
            usize_to_scalar("erod14_reverse_class_index", reverse_class_index)?;
        let class_offset = class_index.saturating_sub(1);
        let class_offset_f64 = usize_to_scalar("erod14_class_offset", class_offset)?;
        classes.push(TypedMofe03Wave2ClassProjection {
            fall_m_s: (0.02 / class_index_f64).max(MOFE03_WAVE2_MIN_POSITIVE),
            frcflw: class_fraction,
            frac: class_fraction,
            fidel: (0.20 + (0.10 * class_index_f64)).min(0.95),
            tcf1: 0.20 + (0.05 * reverse_class_index_f64),
            ssa_class: 1.5 + (2.5 * class_offset_f64),
        });
    }
    Ok(classes)
}

fn publish_typed_mofe03_wave2_projection(
    runtime_surface: &mut HillslopeWritebackSurface,
    projection: &TypedMofe03Wave2Projection,
) -> Result<(), HillslopeCliError> {
    write_mofe03_wave2_enabled(runtime_surface, projection.wave2_enabled);
    if !projection.wave2_enabled {
        return Ok(());
    }
    seed_mofe03_wave2_core_scalars(runtime_surface, projection)?;
    seed_mofe03_wave2_route_topology_ingress(runtime_surface, projection);
    seed_mofe03_wave2_case_state(runtime_surface, projection);
    seed_mofe03_wave2_ssa_soil(runtime_surface, projection);
    seed_mofe03_wave2_class_symbols(runtime_surface, projection)?;
    Ok(())
}

pub(super) fn resolve_mofe03_ofe_count(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<usize, HillslopeCliError> {
    let (symbol, value) = match runtime_surface_symbol_value(
        runtime_surface,
        "mofe.static_lane.contributor_ofe_count",
    ) {
        Some(value) => ("mofe.static_lane.contributor_ofe_count", value),
        None => (
            "nelem",
            require_mofe03_runtime_surface_scalar(runtime_surface, "nelem")?,
        ),
    };
    let ofe_count = scalar_to_usize(symbol, value)?;
    if ofe_count == 0 {
        return Err(mofe03_wave2_seed_failure(
            "MOFE03 OFE count must be >= 1 for activation policy",
        ));
    }
    Ok(ofe_count)
}

pub(super) fn resolve_mofe03_wave2_enabled(
    runtime_surface: &HillslopeWritebackSurface,
    ofe_count: usize,
) -> Result<bool, HillslopeCliError> {
    if let Some(value) = runtime_surface_symbol_value(runtime_surface, "erod14_wave2_enabled") {
        parse_mofe03_binary_flag("erod14_wave2_enabled", value)
    } else {
        Ok(ofe_count > 1)
    }
}

pub(super) fn write_mofe03_wave2_enabled(runtime_surface: &mut HillslopeWritebackSurface, enabled: bool) {
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_wave2_enabled"),
        BoundaryValue::scalar(if enabled { 1.0 } else { 0.0 }),
    );
}

pub(super) fn require_mofe03_positive_runtime_surface_scalar(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &str,
    context: &str,
) -> Result<f64, HillslopeCliError> {
    let value = require_mofe03_runtime_surface_scalar(runtime_surface, symbol)?;
    if value <= 0.0 {
        return Err(mofe03_wave2_seed_failure(format!(
            "{symbol} must be > 0.0 for {context}, observed {value}"
        )));
    }
    Ok(value)
}

pub(super) fn resolve_mofe03_wave2_qout(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<f64, HillslopeCliError> {
    require_mofe03_non_negative_seed_scalar(
        runtime_surface_symbol_value(runtime_surface, "Q")
            .or_else(|| runtime_surface_symbol_value(runtime_surface, "wb12_runoff_observed"))
            .unwrap_or(0.0),
        "erod14_qout",
    )
}

pub(super) fn resolve_mofe03_wave2_qin(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<f64, HillslopeCliError> {
    require_mofe03_non_negative_seed_scalar(
        runtime_surface_symbol_value(runtime_surface, "UpStrmQ").unwrap_or(0.0),
        "erod14_qin",
    )
}

pub(super) fn build_mofe03_wave2_case_scalars(qout: f64) -> Mofe03Wave2CaseScalars {
    if qout > MOFE03_WAVE2_ENABLE_TOLERANCE {
        return Mofe03Wave2CaseScalars {
            case_value: 2.0,
            qj_minus_1: qout.max(MOFE03_WAVE2_MIN_POSITIVE),
            vj: (0.25 * qout).max(MOFE03_WAVE2_MIN_POSITIVE),
            qj: (0.50 * qout).max(MOFE03_WAVE2_MIN_POSITIVE),
            fh: qout.max(MOFE03_WAVE2_MIN_POSITIVE),
            fp: (0.5 * qout).max(MOFE03_WAVE2_MIN_POSITIVE),
        };
    }
    Mofe03Wave2CaseScalars {
        case_value: 4.0,
        qj_minus_1: MOFE03_WAVE2_MIN_POSITIVE,
        vj: 0.0,
        qj: 0.0,
        fh: 0.0,
        fp: 0.0,
    }
}

pub(super) fn seed_mofe03_wave2_core_scalars(
    runtime_surface: &mut HillslopeWritebackSurface,
    projection: &TypedMofe03Wave2Projection,
) -> Result<(), HillslopeCliError> {
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_class_count"),
        BoundaryValue::scalar(usize_to_scalar("erod14_class_count", projection.class_count)?),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_xtop"),
        BoundaryValue::scalar(projection.xtop_m),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_xbot"),
        BoundaryValue::scalar(projection.xbot_m),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_xdetst"),
        BoundaryValue::scalar(projection.xdetst_m),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_ldtop"),
        BoundaryValue::scalar(projection.ldtop_kg_s_m),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_ldbot"),
        BoundaryValue::scalar(projection.ldbot_kg_s_m),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_lddend"),
        BoundaryValue::scalar(projection.lddend_kg),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_qout"),
        BoundaryValue::scalar(projection.qout_m3_s),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_qin"),
        BoundaryValue::scalar(projection.qin_m3_s),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_qostar"),
        BoundaryValue::scalar(projection.qostar_m.max(MOFE03_WAVE2_DEFAULT_QOSTAR)),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_slplen"),
        BoundaryValue::scalar(projection.slplen_m),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_ktrato"),
        BoundaryValue::scalar(projection.ktrato),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_ainftc"),
        BoundaryValue::scalar(projection.aintc),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_binftc"),
        BoundaryValue::scalar(projection.bintc),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_cinftc"),
        BoundaryValue::scalar(projection.cintc),
    );
    Ok(())
}

pub(super) fn seed_mofe03_wave2_route_topology_ingress(
    runtime_surface: &mut HillslopeWritebackSurface,
    projection: &TypedMofe03Wave2Projection,
) {
    let segment = MOFE03_ROUTE_SEGMENT_INDEX;

    seed_mofe03_scalar_if_absent(
        runtime_surface,
        "qostar",
        projection.route_qostar_m,
    );
    seed_mofe03_scalar_if_absent(runtime_surface, "xdetst", projection.route_xdetst_m);
    seed_mofe03_scalar_if_absent(runtime_surface, "lddend", projection.route_lddend_kg);
    seed_mofe03_segment_scalar_if_absent(runtime_surface, "xu", segment, projection.route_xu_m);
    seed_mofe03_segment_scalar_if_absent(runtime_surface, "xl", segment, projection.route_xl_m);
    seed_mofe03_segment_scalar_if_absent(runtime_surface, "ainf", segment, projection.route_ainf);
    seed_mofe03_segment_scalar_if_absent(runtime_surface, "binf", segment, projection.route_binf);
    seed_mofe03_segment_scalar_if_absent(runtime_surface, "cinf", segment, projection.route_cinf);
    seed_mofe03_segment_scalar_if_absent(runtime_surface, "ainftc", segment, projection.route_ainftc);
    seed_mofe03_segment_scalar_if_absent(runtime_surface, "binftc", segment, projection.route_binftc);
    seed_mofe03_segment_scalar_if_absent(runtime_surface, "cinftc", segment, projection.route_cinftc);
}

pub(super) fn seed_mofe03_scalar_if_absent(
    runtime_surface: &mut HillslopeWritebackSurface,
    symbol: &str,
    value: f64,
) {
    if runtime_surface_symbol_value(runtime_surface, symbol).is_some() {
        return;
    }
    runtime_surface
        .state_surface
        .insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
}

pub(super) fn seed_mofe03_segment_scalar_if_absent(
    runtime_surface: &mut HillslopeWritebackSurface,
    root: &str,
    segment_index: usize,
    value: f64,
) {
    let symbol = format!("{root}_{segment_index:04}");
    seed_mofe03_scalar_if_absent(runtime_surface, &symbol, value);
}

pub(super) fn resolve_mofe03_wave2_beta_theta(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<(f64, f64), HillslopeCliError> {
    let beta = match runtime_surface_symbol_value(runtime_surface, "beta") {
        Some(value) => require_mofe03_non_negative_seed_scalar(value, "beta")?,
        None => MOFE03_WAVE2_DEFAULT_BETA,
    };
    let erod13_core_enabled = runtime_surface_symbol_value(runtime_surface, "erod13_core_enabled")
        .map(|value| parse_mofe03_binary_flag("erod13_core_enabled", value))
        .transpose()?
        .unwrap_or(false);
    let theta = if erod13_core_enabled {
        let value = require_mofe03_runtime_surface_scalar(runtime_surface, "theta")?;
        require_mofe03_non_negative_seed_scalar(value, "theta")?
    } else {
        let thetdr = require_mofe03_non_negative_seed_scalar(
            require_mofe03_runtime_surface_scalar(runtime_surface, "thetdr")?,
            "thetdr",
        )?;
        let thetfc = require_mofe03_non_negative_seed_scalar(
            require_mofe03_runtime_surface_scalar(runtime_surface, "thetfc")?,
            "thetfc",
        )?;
        0.5 * (thetdr + thetfc)
    };
    Ok((beta, theta))
}

pub(super) fn seed_mofe03_wave2_case_state(
    runtime_surface: &mut HillslopeWritebackSurface,
    projection: &TypedMofe03Wave2Projection,
) {
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_beta"),
        BoundaryValue::scalar(projection.beta),
    );
    runtime_surface
        .state_surface
        .insert(BoundarySymbol::from("theta"), BoundaryValue::scalar(projection.theta));
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_Qj_minus_1"),
        BoundaryValue::scalar(projection.qj_minus_1_m3_s),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_Vj"),
        BoundaryValue::scalar(projection.vj_m),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_Qj"),
        BoundaryValue::scalar(projection.qj_m3_s),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_Fh"),
        BoundaryValue::scalar(projection.fh_m),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_Fp"),
        BoundaryValue::scalar(projection.fp_m),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_case"),
        BoundaryValue::scalar(projection.case_value),
    );
}

pub(super) fn seed_mofe03_wave2_ssa_soil(
    runtime_surface: &mut HillslopeWritebackSurface,
    projection: &TypedMofe03Wave2Projection,
) {
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_ssa_soil"),
        BoundaryValue::scalar(projection.ssa_soil),
    );
}

pub(super) fn seed_mofe03_wave2_class_symbols(
    runtime_surface: &mut HillslopeWritebackSurface,
    projection: &TypedMofe03Wave2Projection,
) -> Result<(), HillslopeCliError> {
    for (class_offset, class_projection) in projection.classes.iter().enumerate() {
        let class_index = class_offset + 1;
        seed_mofe03_wave2_class_symbol(
            runtime_surface,
            "erod14_fall",
            class_index,
            class_projection.fall_m_s,
        )?;
        seed_mofe03_wave2_class_symbol(
            runtime_surface,
            "erod14_frcflw",
            class_index,
            class_projection.frcflw,
        )?;
        seed_mofe03_wave2_class_symbol(
            runtime_surface,
            "erod14_frac",
            class_index,
            class_projection.frac,
        )?;
        seed_mofe03_wave2_class_symbol(
            runtime_surface,
            "erod14_fidel",
            class_index,
            class_projection.fidel,
        )?;
        seed_mofe03_wave2_class_symbol(
            runtime_surface,
            "erod14_tcf1",
            class_index,
            class_projection.tcf1,
        )?;
        seed_mofe03_wave2_class_symbol(
            runtime_surface,
            "erod14_ssa_class",
            class_index,
            class_projection.ssa_class,
        )?;
    }
    Ok(())
}
