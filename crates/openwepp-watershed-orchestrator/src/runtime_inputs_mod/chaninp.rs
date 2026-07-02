use super::types::WatershedRuntimeInputError;
use openwepp_input_contract::parsers::watershed_impoundment::{
    CulvertPayload, EmergencySpillwayPayload, ImpoundmentRecord,
};

pub(crate) type Ws12ImpoundmentProjectionTuple = (&'static str, f64, Option<f64>, bool);

const STANDARD_GRAVITY_M_S2: f64 = 9.806_65;
const ACTIVE_PROJECTION_STAGE_DELTA_M: f64 = 0.01;
const EMERGENCY_OPEN_CHANNEL_WEIR_COEFFICIENT: f64 = 3.087;
const WS12_FUNCTION_COUNT: usize = 15;

#[derive(Debug, Clone, Copy)]
struct Ws12ActiveProjection {
    drop_coefficient: f64,
    drop_exponent: f64,
    culvert_coefficient: f64,
    culvert_exponent: f64,
    riser_coefficient: f64,
    drop_threshold: f64,
    culvert_threshold: f64,
    riser_threshold: f64,
}

#[derive(Debug, Clone)]
pub(crate) struct Ws12OutflowFunctionFamilies {
    a: [f64; WS12_FUNCTION_COUNT],
    b: [f64; WS12_FUNCTION_COUNT],
    c: [f64; WS12_FUNCTION_COUNT],
    d: [f64; WS12_FUNCTION_COUNT],
    e: [f64; WS12_FUNCTION_COUNT],
    ha: [f64; WS12_FUNCTION_COUNT],
}

impl Ws12OutflowFunctionFamilies {
    fn inactive_default(hfull: f64) -> Self {
        Self {
            a: [0.0; WS12_FUNCTION_COUNT],
            b: [0.0; WS12_FUNCTION_COUNT],
            c: [0.0; WS12_FUNCTION_COUNT],
            d: [0.0; WS12_FUNCTION_COUNT],
            e: [0.0; WS12_FUNCTION_COUNT],
            ha: [hfull; WS12_FUNCTION_COUNT],
        }
    }

    pub(crate) fn coefficient_at(&self, family_index: usize, suffix: &'static str) -> f64 {
        let index = family_index - 1;
        match suffix {
            "a" => self.a[index],
            "b" => self.b[index],
            "c" => self.c[index],
            "d" => self.d[index],
            "e" => self.e[index],
            "ha" => self.ha[index],
            _ => unreachable!("unsupported coefficient suffix"),
        }
    }
}

pub(crate) fn derive_ws12_impoundment_coefficients(
    node_id: usize,
    record: &ImpoundmentRecord,
) -> Result<[Ws12ImpoundmentProjectionTuple; 14], WatershedRuntimeInputError> {
    let has_active_structure = record.structure_flags.has_drop_spillway
        || record.structure_flags.has_culvert_1
        || record.structure_flags.has_culvert_2
        || record.structure_flags.has_rockfill
        || record.structure_flags.has_emergency_spillway
        || record.structure_flags.has_filter_barrier
        || record.structure_flags.has_perforated_riser;

    let (a1, a2) = derive_power_law_curve_coefficients(
        node_id,
        "area",
        &record.stage,
        &record.area,
        record.a0,
    )?;
    let (l1, l2) = derive_power_law_curve_coefficients(
        node_id,
        "length",
        &record.stage,
        &record.length,
        record.l0,
    )?;

    let area_denominator: f64 = record.a0 + a1 * record.h.powf(a2);
    if !area_denominator.is_finite() || area_denominator <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_a0"),
            value: area_denominator,
            rule: "derived stage-area denominator at current stage must be finite and > 0",
        });
    }

    let projection = if has_active_structure {
        derive_ws12_active_structure_projection(node_id, record)?
    } else {
        let threshold = record.hfull;
        Ws12ActiveProjection {
            drop_coefficient: 0.0,
            drop_exponent: 1.0,
            culvert_coefficient: 0.0,
            culvert_exponent: 1.0,
            riser_coefficient: 0.0,
            drop_threshold: threshold,
            culvert_threshold: threshold,
            riser_threshold: threshold,
        }
    };

    Ok([
        ("a", projection.drop_coefficient, Some(0.0), true),
        ("b", projection.drop_exponent, Some(0.0), false),
        ("c", projection.culvert_coefficient, Some(0.0), true),
        ("d", projection.culvert_exponent, Some(0.0), false),
        ("e", projection.riser_coefficient, Some(0.0), true),
        ("ha", projection.drop_threshold, Some(0.0), true),
        ("ht", projection.culvert_threshold, Some(0.0), true),
        ("hlm", projection.riser_threshold, Some(0.0), true),
        ("a0", record.a0, None, true),
        ("a1", a1, Some(0.0), false),
        ("a2", a2, Some(0.0), false),
        ("l0", record.l0, None, true),
        ("l1", l1, Some(0.0), false),
        ("l2", l2, Some(0.0), false),
    ])
}

#[allow(clippy::too_many_lines)]
pub(crate) fn derive_ws12_outflow_function_families(
    node_id: usize,
    record: &ImpoundmentRecord,
) -> Result<Ws12OutflowFunctionFamilies, WatershedRuntimeInputError> {
    let mut families = Ws12OutflowFunctionFamilies::inactive_default(record.hfull);

    project_drop_spillway_function_families(node_id, record, &mut families)?;
    project_culvert_function_families(node_id, &record.culverts[0], 4, &mut families)?;
    project_culvert_function_families(node_id, &record.culverts[1], 7, &mut families)?;
    project_rockfill_function(node_id, record, &mut families)?;
    project_emergency_function(node_id, record, &mut families)?;
    project_filter_function(node_id, record, &mut families)?;
    project_riser_functions(node_id, record, &mut families)?;

    Ok(families)
}

fn project_drop_spillway_function_families(
    node_id: usize,
    record: &ImpoundmentRecord,
    families: &mut Ws12OutflowFunctionFamilies,
) -> Result<(), WatershedRuntimeInputError> {
    use openwepp_input_contract::parsers::watershed_impoundment::DropSpillwayPayload;

    match &record.drop_spillway {
        DropSpillwayPayload::None => Ok(()),
        DropSpillwayPayload::Ids1 { payload, .. } => {
            let denominator =
                1.0 + payload.ke + payload.kb + payload.kc * (payload.lbl + payload.hrh);
            if !denominator.is_finite() || denominator <= 0.0 {
                return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                    symbol: format!("ws10_impoundment_{node_id}_f03_b"),
                    value: denominator,
                    rule: "drop-spillway loss denominator must be finite and > 0",
                });
            }

            families.a[0] = 1.0;
            families.b[0] = payload.coefw * std::f64::consts::PI * payload.diars;
            families.c[0] = 1.5;
            families.ha[0] = payload.hrs;

            families.a[1] = 1.0;
            families.b[1] = payload.coefo * std::f64::consts::PI * payload.diars.powi(2) / 4.0
                * (2.0 * STANDARD_GRAVITY_M_S2).sqrt();
            families.c[1] = 0.5;
            families.ha[1] = payload.hrs;

            families.a[2] = payload.hblot + 0.6 * payload.diabl;
            families.b[2] = std::f64::consts::PI * payload.diabl.powi(2) / 4.0
                * (2.0 * STANDARD_GRAVITY_M_S2).sqrt()
                / denominator.sqrt();
            families.c[2] = 0.5;
            families.ha[2] =
                payload.hrs - (payload.hrh + payload.sbl * payload.lbl - 0.6 * payload.diabl);

            Ok(())
        }
        DropSpillwayPayload::Ids2 { payload, .. } => {
            let denominator =
                1.0 + payload.ke + payload.kb + payload.kc * (payload.lbl + payload.hrh);
            if !denominator.is_finite() || denominator <= 0.0 {
                return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                    symbol: format!("ws10_impoundment_{node_id}_f03_b"),
                    value: denominator,
                    rule: "drop-spillway loss denominator must be finite and > 0",
                });
            }

            families.a[0] = 1.0;
            families.b[0] = payload.coefw * 2.0 * (payload.lenrs + payload.widrs);
            families.c[0] = 1.5;
            families.ha[0] = payload.hrs;

            families.a[1] = 1.0;
            families.b[1] = payload.coefo
                * payload.lenrs
                * payload.widrs
                * (2.0 * STANDARD_GRAVITY_M_S2).sqrt();
            families.c[1] = 0.5;
            families.ha[1] = payload.hrs;

            families.a[2] = payload.hblot + 0.6 * payload.diabl;
            families.b[2] = std::f64::consts::PI * payload.diabl.powi(2) / 4.0
                * (2.0 * STANDARD_GRAVITY_M_S2).sqrt()
                / denominator.sqrt();
            families.c[2] = 0.5;
            families.ha[2] =
                payload.hrs - (payload.hrh + payload.sbl * payload.lbl - 0.6 * payload.diabl);

            Ok(())
        }
        DropSpillwayPayload::Ids3 { payload, .. } => {
            let denominator =
                1.0 + payload.ke + payload.kb + payload.kc * (payload.lbl + payload.hrh);
            if !denominator.is_finite() || denominator <= 0.0 {
                return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                    symbol: format!("ws10_impoundment_{node_id}_f03_b"),
                    value: denominator,
                    rule: "drop-spillway loss denominator must be finite and > 0",
                });
            }

            families.a[0] = 1.0;
            families.b[0] = payload.coefw * 2.0 * (payload.lenrs + payload.widrs);
            families.c[0] = 1.5;
            families.ha[0] = payload.hrs;

            families.a[1] = 1.0;
            families.b[1] = payload.coefo
                * payload.lenrs
                * payload.widrs
                * (2.0 * STANDARD_GRAVITY_M_S2).sqrt();
            families.c[1] = 0.5;
            families.ha[1] = payload.hrs;

            families.a[2] = payload.hblot + 0.6 * payload.hitbl;
            families.b[2] = payload.hitbl * payload.wdbl * (2.0 * STANDARD_GRAVITY_M_S2).sqrt()
                / denominator.sqrt();
            families.c[2] = 0.5;
            families.ha[2] =
                payload.hrs - (payload.hrh + payload.sbl * payload.lbl - 0.6 * payload.hitbl);

            Ok(())
        }
    }
}

fn project_culvert_function_families(
    node_id: usize,
    culvert: &CulvertPayload,
    family_start: usize,
    families: &mut Ws12OutflowFunctionFamilies,
) -> Result<(), WatershedRuntimeInputError> {
    if culvert.icv < 1 {
        return Ok(());
    }

    let Some(parameters) = &culvert.parameters else {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_f{family_start:02}_a"),
            value: f64::from(culvert.icv),
            rule: "active culvert payload must include hydraulic parameters",
        });
    };

    let ncv = f64::from(culvert.ncv);
    let denominator = 1.0 + parameters.ke + parameters.kb + parameters.kc * parameters.lcv;
    if !denominator.is_finite() || denominator <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_f{:02}_b", family_start + 2),
            value: denominator,
            rule: "culvert loss denominator must be finite and > 0",
        });
    }
    if !ncv.is_finite() || ncv <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_f{family_start:02}_a"),
            value: ncv,
            rule: "culvert count must be finite and > 0",
        });
    }
    if !parameters.mus.is_finite() || parameters.mus.abs() <= 1.0e-12 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_f{family_start:02}_c"),
            value: parameters.mus,
            rule: "culvert mus must be finite and non-zero",
        });
    }
    if !parameters.cs.is_finite() || parameters.cs.abs() <= 1.0e-12 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_f{:02}_d", family_start + 1),
            value: parameters.cs,
            rule: "culvert cs must be finite and non-zero",
        });
    }

    let base = family_start - 1;
    families.a[base] = parameters.arcv * parameters.hitcv.sqrt() * ncv;
    families.b[base] = parameters.hitcv * parameters.kus;
    families.c[base] = 1.0 / parameters.mus;
    families.ha[base] = parameters.hcv;

    families.a[base + 1] = parameters.arcv * parameters.hitcv.sqrt() * ncv;
    families.b[base + 1] = parameters.hitcv;
    families.c[base + 1] = 0.5 * parameters.scv - parameters.ys;
    families.d[base + 1] = parameters.cs;
    families.ha[base + 1] = parameters.hcv;

    families.a[base + 2] = parameters.hcvot + 0.6 * parameters.hitcv;
    families.b[base + 2] =
        parameters.arcv * (2.0 * STANDARD_GRAVITY_M_S2).sqrt() * ncv / denominator.sqrt();
    families.c[base + 2] = 0.5;
    families.ha[base + 2] =
        parameters.hcv - parameters.scv * parameters.lcv + 0.6 * parameters.hitcv;

    Ok(())
}

fn project_rockfill_function(
    node_id: usize,
    record: &ImpoundmentRecord,
    families: &mut Ws12OutflowFunctionFamilies,
) -> Result<(), WatershedRuntimeInputError> {
    let Some(rockfill) = &record.rockfill else {
        return Ok(());
    };
    if !rockfill.diarf.is_finite() || rockfill.diarf <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_f10_b"),
            value: rockfill.diarf,
            rule: "rockfill diarf must be finite and > 0",
        });
    }
    if !rockfill.lnrf.is_finite() || rockfill.lnrf <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_f10_b"),
            value: rockfill.lnrf,
            rule: "rockfill lnrf must be finite and > 0",
        });
    }

    let arf = rockfill_arf(rockfill.lnrf, rockfill.diarf);
    let brf_denominator = 1.500_560_9 - 0.000_131_719_05 * rockfill.diarf.ln() / rockfill.diarf;
    if !brf_denominator.is_finite() || brf_denominator <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_f10_c"),
            value: brf_denominator,
            rule: "rockfill brf denominator must be finite and > 0",
        });
    }
    let brf = 1.0 / brf_denominator;

    let index = 9;
    families.a[index] = rockfill.wdrf;
    families.b[index] = rockfill.lnrf * arf;
    families.c[index] = 1.0 / brf;
    families.d[index] = EMERGENCY_OPEN_CHANNEL_WEIR_COEFFICIENT * rockfill.wdrf;
    families.e[index] = rockfill.hotrf;
    families.ha[index] = rockfill.hrf;
    Ok(())
}

fn project_emergency_function(
    node_id: usize,
    record: &ImpoundmentRecord,
    families: &mut Ws12OutflowFunctionFamilies,
) -> Result<(), WatershedRuntimeInputError> {
    let index = 10;
    match &record.emergency_spillway {
        EmergencySpillwayPayload::None => Ok(()),
        EmergencySpillwayPayload::OpenChannel { payload, .. } => {
            let span = (payload.hmxes - payload.hes).max(0.05);
            let mut points = Vec::with_capacity(16);
            points.push((0.0, 0.0));
            for sample_idx in 1..=15_u32 {
                let fraction = f64::from(sample_idx) / 15.0;
                let delta = span * fraction;
                let discharge =
                    EMERGENCY_OPEN_CHANNEL_WEIR_COEFFICIENT * payload.bwes * delta.powf(1.5);
                points.push((delta, discharge.max(0.0)));
            }
            let coefficients = fit_quartic_least_squares(node_id, &points, "f11")?;
            families.a[index] = coefficients[0];
            families.b[index] = coefficients[1];
            families.c[index] = coefficients[2];
            families.d[index] = coefficients[3];
            families.e[index] = coefficients[4];
            families.ha[index] = payload.hes;
            Ok(())
        }
        EmergencySpillwayPayload::RatingCurve { payload, .. } => {
            if payload.hest.len() != payload.qes.len() || payload.hest.is_empty() {
                return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                    symbol: format!("ws10_impoundment_{node_id}_f11_a"),
                    value: f64::from(u32::try_from(payload.hest.len()).unwrap_or(u32::MAX)),
                    rule: "emergency rating curve vectors must have equal non-zero length",
                });
            }
            let mut points = Vec::with_capacity(payload.hest.len() + 1);
            points.push((0.0, 0.0));
            for (&stage_value, &discharge_value) in payload.hest.iter().zip(payload.qes.iter()) {
                let x = (stage_value - payload.hes).max(0.0);
                points.push((x, discharge_value.max(0.0)));
            }
            let coefficients = fit_quartic_least_squares(node_id, &points, "f11")?;
            families.a[index] = coefficients[0];
            families.b[index] = coefficients[1];
            families.c[index] = coefficients[2];
            families.d[index] = coefficients[3];
            families.e[index] = coefficients[4];
            families.ha[index] = payload.hes;
            Ok(())
        }
    }
}

fn project_filter_function(
    node_id: usize,
    record: &ImpoundmentRecord,
    families: &mut Ws12OutflowFunctionFamilies,
) -> Result<(), WatershedRuntimeInputError> {
    let Some(filter) = &record.filter_barrier else {
        return Ok(());
    };
    let index = 11;
    families.a[index] = filter.wdff * filter.vsl;
    families.ha[index] = filter.hff;
    families.d[index] = filter.hotff;

    if filter.hotff <= filter.hff {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_f12_d"),
            value: filter.hotff - filter.hff,
            rule: "filter overtopping stage must be > base stage",
        });
    }

    if record.filter_code == 1 {
        families.b[index] = 3.27 * filter.wdff;
        families.c[index] = (0.4 / (filter.hotff - filter.hff)) * filter.wdff;
    } else {
        families.b[index] = EMERGENCY_OPEN_CHANNEL_WEIR_COEFFICIENT * filter.wdff;
        families.c[index] = 0.0;
    }
    Ok(())
}

fn project_riser_functions(
    node_id: usize,
    record: &ImpoundmentRecord,
    families: &mut Ws12OutflowFunctionFamilies,
) -> Result<(), WatershedRuntimeInputError> {
    let Some(riser) = &record.perforated_riser else {
        return Ok(());
    };

    if !riser.diar.is_finite() || riser.diar <= 0.0 || !riser.diab.is_finite() || riser.diab <= 0.0
    {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_f13_b"),
            value: if riser.diar <= 0.0 {
                riser.diar
            } else {
                riser.diab
            },
            rule: "riser diameters must be finite and > 0",
        });
    }

    let (apr1, apr2) = derive_riser_apr_coefficients(node_id, riser)?;
    let ko = (-0.60721 + 0.329_229 * (riser.diab / riser.diar)).exp();
    let denominator = 1.0 + riser.ke + riser.kb + riser.kc * riser.lbl + ko;
    if !denominator.is_finite() || denominator <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_f15_b"),
            value: denominator,
            rule: "riser loss denominator must be finite and > 0",
        });
    }

    let ab = std::f64::consts::PI * riser.diab.powi(2) / 4.0;
    let index_13 = 12;
    families.a[index_13] = 1.0;
    families.b[index_13] = apr1;
    families.c[index_13] = apr2;
    families.ha[index_13] = riser.hd;

    let index_14 = 13;
    families.a[index_14] = riser.cb * ab * (2.0 * STANDARD_GRAVITY_M_S2).sqrt();
    families.ha[index_14] = riser.hd - riser.hb;

    let index_15 = 14;
    families.b[index_15] = std::f64::consts::PI * riser.diabl.powi(2) / 4.0
        * (2.0 * STANDARD_GRAVITY_M_S2).sqrt()
        / denominator.sqrt();
    families.c[index_15] = 0.5;
    families.ha[index_15] = riser.hr - (riser.hrh + riser.sbl * riser.lbl - 0.6 * riser.diabl);

    Ok(())
}

fn derive_riser_apr_coefficients(
    node_id: usize,
    riser: &openwepp_input_contract::parsers::watershed_impoundment::PerforatedRiserPayload,
) -> Result<(f64, f64), WatershedRuntimeInputError> {
    let points = sample_riser_unsubmerged_curve(node_id, riser)?;
    if points.len() < 2 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_f13_b"),
            value: f64::from(u32::try_from(points.len()).unwrap_or(0)),
            rule: "riser unsubmerged curve sampling requires at least two points",
        });
    }

    let mut sum_inverse_head = 0.0;
    let mut sum_inverse_discharge = 0.0;
    let mut sum_inverse_head_squared = 0.0;
    let mut sum_cross_term = 0.0;
    for &(hp, q) in &points {
        if !hp.is_finite() || hp <= 0.0 || !q.is_finite() || q <= 0.0 {
            return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                symbol: format!("ws10_impoundment_{node_id}_f13_b"),
                value: if hp <= 0.0 { hp } else { q },
                rule: "riser regression points must be finite and > 0",
            });
        }
        let u = 1.0 / hp.powf(1.5);
        let z = 1.0 / q;
        sum_inverse_head += u;
        sum_inverse_discharge += z;
        sum_inverse_head_squared += u * u;
        sum_cross_term += u * z;
    }

    let n = f64::from(u32::try_from(points.len()).unwrap_or(u32::MAX));
    let denominator = (n * sum_inverse_head_squared) - (sum_inverse_head * sum_inverse_head);
    if !denominator.is_finite() || denominator.abs() <= 1.0e-12 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_f13_b"),
            value: denominator,
            rule: "riser regression denominator must be finite and non-zero",
        });
    }

    let apr1 = ((sum_inverse_discharge * sum_inverse_head_squared)
        - (sum_inverse_head * sum_cross_term))
        / denominator;
    let apr2 = ((n * sum_cross_term) - (sum_inverse_head * sum_inverse_discharge)) / denominator;
    if !apr1.is_finite() || !apr2.is_finite() {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_f13_b"),
            value: if apr1.is_finite() { apr2 } else { apr1 },
            rule: "riser regression coefficients must be finite",
        });
    }

    Ok((apr1, apr2))
}

#[allow(clippy::too_many_lines)]
fn sample_riser_unsubmerged_curve(
    node_id: usize,
    riser: &openwepp_input_contract::parsers::watershed_impoundment::PerforatedRiserPayload,
) -> Result<Vec<(f64, f64)>, WatershedRuntimeInputError> {
    let mut points = Vec::new();
    let mut hp_delta = 0.05;
    let mut hp = hp_delta;
    let mut y = -riser.hb;
    let mut iterations = 0_usize;
    let maximum_iterations = 20_000_usize;
    let q_tolerance = 1.0e-12;
    let y_delta = 1.0e-4;

    let ko = (-0.60721 + 0.329_229 * (riser.diab / riser.diar)).exp();
    let ab = std::f64::consts::PI * riser.diab.powi(2) / 4.0;

    while iterations < maximum_iterations && points.len() < 99 {
        iterations += 1;
        let qb_head = riser.hb + y;
        if qb_head <= 0.0 || !qb_head.is_finite() {
            y += y_delta;
            continue;
        }

        let qb = riser.cb * ab * (2.0 * STANDARD_GRAVITY_M_S2 * qb_head).sqrt();
        let qs = compute_riser_qs(hp, y, ko, riser)?;
        if !qb.is_finite() || !qs.is_finite() {
            return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                symbol: format!("ws10_impoundment_{node_id}_f13_b"),
                value: if qb.is_finite() { qs } else { qb },
                rule: "riser sampled discharges must be finite",
            });
        }

        if qb < qs {
            y += y_delta;
            if y >= hp {
                points.push((hp.max(1.0e-6), qb.max(q_tolerance)));
                hp += hp_delta;
                if hp_delta <= 0.0 || !hp_delta.is_finite() {
                    break;
                }
                continue;
            }
            if y > (riser.hr - riser.hd) {
                break;
            }
            continue;
        }

        points.push((hp.max(1.0e-6), qs.max(q_tolerance)));
        hp += hp_delta;
        if points.len() >= 99 {
            break;
        }
        if hp > 5.0 * (riser.hr + riser.hs + riser.hd + 1.0) {
            hp_delta *= 2.0;
            if hp_delta > 10.0 {
                break;
            }
        }
    }

    Ok(points)
}

fn compute_riser_qs(
    hp: f64,
    y: f64,
    ko: f64,
    riser: &openwepp_input_contract::parsers::watershed_impoundment::PerforatedRiserPayload,
) -> Result<f64, WatershedRuntimeInputError> {
    let slot_factor = (riser.cs * riser.as_slot / riser.hs) * (2.0 * STANDARD_GRAVITY_M_S2).sqrt();
    let qs = if hp < riser.hs {
        if y <= 0.0 {
            (2.0 / 3.0) * slot_factor * hp.powf(1.5)
        } else {
            slot_factor * (y * (hp - y).sqrt() + (2.0 / 3.0) * (hp - y).powf(1.5))
        }
    } else if hp <= (riser.hr - riser.hd) {
        if y <= 0.0 {
            (2.0 / 3.0) * slot_factor * (hp.powf(1.5) - (hp - riser.hs).powf(1.5))
        } else if y <= riser.hs {
            slot_factor
                * (y * (hp - y).sqrt()
                    + (2.0 / 3.0) * ((hp - y).powf(1.5) - (hp - riser.hs).powf(1.5)))
        } else {
            (riser.cs * riser.as_slot) * (2.0 * STANDARD_GRAVITY_M_S2 * (hp - y)).sqrt()
        }
    } else {
        let qw = riser.coefw
            * std::f64::consts::PI
            * riser.diar
            * (hp - (riser.hr - riser.hd)).powf(1.5);
        let qo = riser.coefo * std::f64::consts::PI * riser.diar.powi(2) / 4.0
            * (hp - (riser.hr - riser.hd)).sqrt();
        let q_control = qw.min(qo);
        if y <= 0.0 {
            (2.0 / 3.0) * slot_factor * (hp.powf(1.5) - (hp - riser.hs).powf(1.5)) + q_control
        } else if y <= riser.hs {
            slot_factor
                * (y * (hp - y).sqrt()
                    + (2.0 / 3.0) * ((hp - y).powf(1.5) - (hp - riser.hs).powf(1.5)))
                + q_control
        } else {
            (riser.cs * riser.as_slot) * (2.0 * STANDARD_GRAVITY_M_S2 * (hp - y)).sqrt() + q_control
        }
    };

    let _ = ko;
    if !qs.is_finite() {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: "ws10_impoundment_riser_qs".to_owned(),
            value: qs,
            rule: "riser sampled discharge must be finite",
        });
    }
    Ok(qs.max(0.0))
}

fn fit_quartic_least_squares(
    node_id: usize,
    points: &[(f64, f64)],
    family_label: &'static str,
) -> Result<[f64; 5], WatershedRuntimeInputError> {
    if points.is_empty() {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_{family_label}_a"),
            value: 0.0,
            rule: "quartic fit requires at least one point",
        });
    }

    let mut fit_points = points.to_vec();
    while fit_points.len() < 5 {
        let next = if fit_points.len() == 1 {
            let (x, y) = fit_points[0];
            (x + 0.05, y)
        } else {
            let (x_last, y_last) = fit_points[fit_points.len() - 1];
            let (x_prev, y_prev) = fit_points[fit_points.len() - 2];
            let dx = (x_last - x_prev).abs().max(0.05);
            let slope = if dx > 0.0 {
                (y_last - y_prev) / dx
            } else {
                0.0
            };
            (x_last + dx, (y_last + slope * dx).max(0.0))
        };
        fit_points.push(next);
    }

    let mut normal = [[0.0_f64; 5]; 5];
    let mut rhs = [0.0_f64; 5];
    for &(x, y) in &fit_points {
        if !x.is_finite() || !y.is_finite() {
            return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                symbol: format!("ws10_impoundment_{node_id}_{family_label}_a"),
                value: if x.is_finite() { y } else { x },
                rule: "quartic fit points must be finite",
            });
        }
        let powers = [1.0, x, x * x, x * x * x, x * x * x * x];
        for row in 0..5 {
            rhs[row] += y * powers[row];
            for column in 0..5 {
                normal[row][column] += powers[row] * powers[column];
            }
        }
    }

    solve_linear_system_5x5(normal, rhs).ok_or_else(|| {
        WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_{family_label}_a"),
            value: f64::NAN,
            rule: "quartic fit normal system must be solvable",
        }
    })
}

#[allow(clippy::needless_range_loop)]
fn solve_linear_system_5x5(mut matrix: [[f64; 5]; 5], mut rhs: [f64; 5]) -> Option<[f64; 5]> {
    for pivot in 0..5 {
        let mut max_row = pivot;
        let mut max_value = matrix[pivot][pivot].abs();
        for row in (pivot + 1)..5 {
            let candidate = matrix[row][pivot].abs();
            if candidate > max_value {
                max_value = candidate;
                max_row = row;
            }
        }
        if max_value <= 1.0e-12 {
            return None;
        }

        if max_row != pivot {
            matrix.swap(pivot, max_row);
            rhs.swap(pivot, max_row);
        }

        let pivot_value = matrix[pivot][pivot];
        for column in pivot..5 {
            matrix[pivot][column] /= pivot_value;
        }
        rhs[pivot] /= pivot_value;

        for row in 0..5 {
            if row == pivot {
                continue;
            }
            let factor = matrix[row][pivot];
            if factor.abs() <= 1.0e-20 {
                continue;
            }
            for column in pivot..5 {
                matrix[row][column] -= factor * matrix[pivot][column];
            }
            rhs[row] -= factor * rhs[pivot];
        }
    }

    Some(rhs)
}

#[allow(clippy::too_many_lines)]
fn derive_ws12_active_structure_projection(
    node_id: usize,
    record: &ImpoundmentRecord,
) -> Result<Ws12ActiveProjection, WatershedRuntimeInputError> {
    let reference_stage = derive_active_projection_reference_stage(node_id, record)?;
    let mut active_projection_used = false;

    let (drop_coefficient, drop_exponent, drop_threshold) = if let Some((
        projected_drop_coefficient,
        projected_drop_exponent,
        projected_drop_threshold,
    )) =
        derive_drop_spillway_projection(node_id, record)?
    {
        active_projection_used = true;
        (
            projected_drop_coefficient,
            projected_drop_exponent,
            projected_drop_threshold,
        )
    } else {
        (0.0, 1.0, record.hfull)
    };

    let mut c_stage_thresholds = Vec::new();
    if let Some(threshold) = culvert_stage_threshold(&record.culverts[0])? {
        c_stage_thresholds.push(threshold);
    }
    if let Some(threshold) = culvert_stage_threshold(&record.culverts[1])? {
        c_stage_thresholds.push(threshold);
    }
    if let Some(rockfill) = &record.rockfill {
        c_stage_thresholds.push(rockfill.hrf);
    }
    match &record.emergency_spillway {
        EmergencySpillwayPayload::None => {}
        EmergencySpillwayPayload::OpenChannel { payload, .. } => {
            c_stage_thresholds.push(payload.hes);
        }
        EmergencySpillwayPayload::RatingCurve { payload, .. } => {
            c_stage_thresholds.push(payload.hes);
        }
    }
    if let Some(filter) = &record.filter_barrier {
        c_stage_thresholds.push(filter.hff);
    }

    let (culvert_coefficient, culvert_exponent, culvert_threshold) =
        if c_stage_thresholds.is_empty() {
            (0.0, 1.0, record.hfull)
        } else {
            active_projection_used = true;
            let culvert_threshold = c_stage_thresholds
                .iter()
                .copied()
                .fold(f64::INFINITY, f64::min);
            if !culvert_threshold.is_finite() {
                return Err(WatershedRuntimeInputError::ImpoundmentSymbolNonFinite {
                    symbol: format!("ws10_impoundment_{node_id}_ht"),
                    value: culvert_threshold,
                });
            }
            let stage = reference_stage.max(culvert_threshold + ACTIVE_PROJECTION_STAGE_DELTA_M);
            let mut projected_discharge: f64 = 0.0;
            projected_discharge +=
                culvert_pipe_discharge_at_stage(node_id, &record.culverts[0], stage)?;
            projected_discharge +=
                culvert_pipe_discharge_at_stage(node_id, &record.culverts[1], stage)?;
            projected_discharge += rockfill_discharge_at_stage(node_id, record, stage)?;
            projected_discharge += emergency_discharge_at_stage(node_id, record, stage)?;
            projected_discharge += filter_barrier_discharge_at_stage(node_id, record, stage)?;

            if !projected_discharge.is_finite() || projected_discharge < 0.0 {
                return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                    symbol: format!("ws10_impoundment_{node_id}_c"),
                    value: projected_discharge,
                    rule: "projected active-structure discharge must be finite and >= 0",
                });
            }

            let span = stage - culvert_threshold;
            if !span.is_finite() || span <= 0.0 {
                return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                    symbol: format!("ws10_impoundment_{node_id}_ht"),
                    value: span,
                    rule: "reference-stage span above ht must be finite and > 0",
                });
            }

            let culvert_exponent = 0.5;
            let culvert_coefficient = if projected_discharge > 0.0 {
                projected_discharge / span.powf(culvert_exponent)
            } else {
                0.0
            };
            (culvert_coefficient, culvert_exponent, culvert_threshold)
        };

    let (riser_coefficient, riser_threshold) =
        if let Some((riser_reference_discharge, riser_threshold)) =
            perforated_riser_reference_discharge(node_id, record, reference_stage)?
        {
            active_projection_used = true;
            let stage = reference_stage.max(riser_threshold + ACTIVE_PROJECTION_STAGE_DELTA_M);
            let span = stage - riser_threshold;
            if !span.is_finite() || span <= 0.0 {
                return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                    symbol: format!("ws10_impoundment_{node_id}_hlm"),
                    value: span,
                    rule: "reference-stage span above hlm must be finite and > 0",
                });
            }
            (riser_reference_discharge / span, riser_threshold)
        } else {
            (0.0, record.hfull)
        };

    if !active_projection_used {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_a"),
            value: 0.0,
            rule: "active outlet-structure flags require at least one projectable payload branch",
        });
    }

    Ok(Ws12ActiveProjection {
        drop_coefficient,
        drop_exponent,
        culvert_coefficient,
        culvert_exponent,
        riser_coefficient,
        drop_threshold,
        culvert_threshold,
        riser_threshold,
    })
}

fn derive_active_projection_reference_stage(
    node_id: usize,
    record: &ImpoundmentRecord,
) -> Result<f64, WatershedRuntimeInputError> {
    let reference_stage = record.h.max(record.hfull);
    if !reference_stage.is_finite() {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolNonFinite {
            symbol: format!("ws10_impoundment_{node_id}_h"),
            value: reference_stage,
        });
    }
    Ok(reference_stage.max(ACTIVE_PROJECTION_STAGE_DELTA_M))
}

fn derive_drop_spillway_projection(
    node_id: usize,
    record: &ImpoundmentRecord,
) -> Result<Option<(f64, f64, f64)>, WatershedRuntimeInputError> {
    match &record.drop_spillway {
        openwepp_input_contract::parsers::watershed_impoundment::DropSpillwayPayload::None => {
            Ok(None)
        }
        openwepp_input_contract::parsers::watershed_impoundment::DropSpillwayPayload::Ids1 {
            payload,
            ..
        } => {
            let coefficient = payload.coefw * std::f64::consts::PI * payload.diars;
            validate_active_projected_positive(
                node_id,
                "a",
                coefficient,
                "drop-spillway weir coefficient must be finite and > 0",
            )?;
            Ok(Some((coefficient, 1.5, payload.hrs)))
        }
        openwepp_input_contract::parsers::watershed_impoundment::DropSpillwayPayload::Ids2 {
            payload,
            ..
        } => {
            let perimeter = 2.0 * (payload.lenrs + payload.widrs);
            let coefficient = payload.coefw * perimeter;
            validate_active_projected_positive(
                node_id,
                "a",
                coefficient,
                "drop-spillway weir coefficient must be finite and > 0",
            )?;
            Ok(Some((coefficient, 1.5, payload.hrs)))
        }
        openwepp_input_contract::parsers::watershed_impoundment::DropSpillwayPayload::Ids3 {
            payload,
            ..
        } => {
            let perimeter = 2.0 * (payload.lenrs + payload.widrs);
            let coefficient = payload.coefw * perimeter;
            validate_active_projected_positive(
                node_id,
                "a",
                coefficient,
                "drop-spillway weir coefficient must be finite and > 0",
            )?;
            Ok(Some((coefficient, 1.5, payload.hrs)))
        }
    }
}

fn culvert_stage_threshold(
    culvert: &CulvertPayload,
) -> Result<Option<f64>, WatershedRuntimeInputError> {
    if culvert.icv < 1 {
        return Ok(None);
    }
    let Some(parameters) = &culvert.parameters else {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: "ws10_impoundment_active_culvert".to_owned(),
            value: f64::from(culvert.icv),
            rule: "active culvert payload must include hydraulic parameters",
        });
    };
    Ok(Some(
        parameters.hcv - parameters.scv * parameters.lcv + 0.6 * parameters.hitcv,
    ))
}

fn culvert_pipe_discharge_at_stage(
    node_id: usize,
    culvert: &CulvertPayload,
    stage: f64,
) -> Result<f64, WatershedRuntimeInputError> {
    if culvert.icv < 1 {
        return Ok(0.0);
    }
    let Some(parameters) = &culvert.parameters else {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_c"),
            value: f64::from(culvert.icv),
            rule: "active culvert payload must include hydraulic parameters",
        });
    };

    let threshold = parameters.hcv - parameters.scv * parameters.lcv + 0.6 * parameters.hitcv;
    if stage <= threshold {
        return Ok(0.0);
    }

    let denominator = 1.0 + parameters.ke + parameters.kb + parameters.kc * parameters.lcv;
    if !denominator.is_finite() || denominator <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_c"),
            value: denominator,
            rule: "active culvert loss denominator must be finite and > 0",
        });
    }

    let count = f64::from(culvert.ncv);
    if !count.is_finite() || count <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_c"),
            value: count,
            rule: "active culvert count must be finite and > 0",
        });
    }

    let coefficient =
        parameters.arcv * (2.0 * STANDARD_GRAVITY_M_S2).sqrt() * count / denominator.sqrt();
    if !coefficient.is_finite() || coefficient <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_c"),
            value: coefficient,
            rule: "active culvert projected coefficient must be finite and > 0",
        });
    }

    Ok(coefficient * (stage - threshold).sqrt())
}

fn rockfill_discharge_at_stage(
    node_id: usize,
    record: &ImpoundmentRecord,
    stage: f64,
) -> Result<f64, WatershedRuntimeInputError> {
    let Some(rockfill) = &record.rockfill else {
        return Ok(0.0);
    };
    if stage <= rockfill.hrf {
        return Ok(0.0);
    }
    if !rockfill.diarf.is_finite() || rockfill.diarf <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_c"),
            value: rockfill.diarf,
            rule: "rockfill diarf must be finite and > 0",
        });
    }
    if !rockfill.lnrf.is_finite() || rockfill.lnrf <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_c"),
            value: rockfill.lnrf,
            rule: "rockfill lnrf must be finite and > 0",
        });
    }

    let arf = rockfill_arf(rockfill.lnrf, rockfill.diarf);
    let brf_denominator = 1.500_560_9 - 0.000_131_719_05 * rockfill.diarf.ln() / rockfill.diarf;
    if !brf_denominator.is_finite() || brf_denominator <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_c"),
            value: brf_denominator,
            rule: "rockfill brf denominator must be finite and > 0",
        });
    }

    let brf = 1.0 / brf_denominator;
    let b10 = rockfill.lnrf * arf;
    if !b10.is_finite() || b10 <= 0.0 || !brf.is_finite() || brf <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_c"),
            value: if b10 <= 0.0 { b10 } else { brf },
            rule: "rockfill projected coefficients must be finite and > 0",
        });
    }

    let mut discharge = 0.0;
    let stage_delta = stage - rockfill.hrf;
    if stage_delta > 0.0 {
        discharge += rockfill.wdrf * (stage_delta / b10).powf(1.0 / brf);
    }
    let overtopping_delta = stage - rockfill.hotrf;
    if overtopping_delta > 0.0 {
        discharge += 3.087 * rockfill.wdrf * overtopping_delta.powf(1.5);
    }
    if !discharge.is_finite() || discharge < 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_c"),
            value: discharge,
            rule: "rockfill projected discharge must be finite and >= 0",
        });
    }
    Ok(discharge)
}

fn rockfill_arf(length_m: f64, diarf_m: f64) -> f64 {
    if length_m < 0.5 {
        let arf1 = 3.041_846 * diarf_m.powf(-0.346_77);
        let arf2 = 1.910_413 * diarf_m.powf(-0.349_35);
        arf1 - ((arf2 - arf1) / 0.5) * (0.5 - length_m)
    } else if length_m < 1.0 {
        let arf1 = 3.041_846 * diarf_m.powf(-0.346_77);
        let arf2 = 1.910_413 * diarf_m.powf(-0.349_35);
        arf1 + ((arf2 - arf1) / 0.5) * (length_m - 0.5)
    } else if length_m < 2.0 {
        let arf1 = 1.910_413 * diarf_m.powf(-0.349_35);
        let arf2 = 1.196_37 * diarf_m.powf(-0.354_22);
        arf1 + (arf2 - arf1) * (length_m - 1.0)
    } else if length_m < 3.0 {
        let arf1 = 1.196_37 * diarf_m.powf(-0.354_22);
        let arf2 = 0.909_902 * diarf_m.powf(-0.357_05);
        arf1 + (arf2 - arf1) * (length_m - 2.0)
    } else {
        let arf1 = 1.196_37 * diarf_m.powf(-0.354_22);
        let arf2 = 0.909_902 * diarf_m.powf(-0.357_05);
        arf2 + (arf2 - arf1) * (length_m - 3.0)
    }
}

fn emergency_discharge_at_stage(
    node_id: usize,
    record: &ImpoundmentRecord,
    stage: f64,
) -> Result<f64, WatershedRuntimeInputError> {
    match &record.emergency_spillway {
        EmergencySpillwayPayload::None => Ok(0.0),
        EmergencySpillwayPayload::OpenChannel { payload, .. } => {
            if stage <= payload.hes {
                return Ok(0.0);
            }
            let delta = stage - payload.hes;
            let discharge =
                EMERGENCY_OPEN_CHANNEL_WEIR_COEFFICIENT * payload.bwes * delta.powf(1.5);
            if !discharge.is_finite() || discharge < 0.0 {
                return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                    symbol: format!("ws10_impoundment_{node_id}_c"),
                    value: discharge,
                    rule: "emergency open-channel projected discharge must be finite and >= 0",
                });
            }
            Ok(discharge)
        }
        EmergencySpillwayPayload::RatingCurve { payload, .. } => {
            if stage <= payload.hes {
                return Ok(0.0);
            }
            if payload.hest.len() != payload.qes.len() || payload.hest.is_empty() {
                let hest_len = u32::try_from(payload.hest.len()).map_err(|_| {
                    WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                        symbol: format!("ws10_impoundment_{node_id}_c"),
                        value: f64::INFINITY,
                        rule: "emergency rating curve vectors must have equal non-zero length",
                    }
                })?;
                return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                    symbol: format!("ws10_impoundment_{node_id}_c"),
                    value: f64::from(hest_len),
                    rule: "emergency rating curve vectors must have equal non-zero length",
                });
            }
            interpolate_rating_curve_discharge(
                node_id,
                payload.hes,
                &payload.hest,
                &payload.qes,
                stage,
            )
        }
    }
}

fn interpolate_rating_curve_discharge(
    node_id: usize,
    hes: f64,
    stage_values: &[f64],
    discharge_values: &[f64],
    stage: f64,
) -> Result<f64, WatershedRuntimeInputError> {
    let mut previous_stage = hes;
    let mut previous_discharge = 0.0;

    for (&curve_stage, &curve_discharge) in stage_values.iter().zip(discharge_values.iter()) {
        if !curve_stage.is_finite() || !curve_discharge.is_finite() {
            return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                symbol: format!("ws10_impoundment_{node_id}_c"),
                value: if curve_stage.is_finite() {
                    curve_discharge
                } else {
                    curve_stage
                },
                rule: "emergency rating-curve points must be finite",
            });
        }
        if curve_stage <= previous_stage {
            return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                symbol: format!("ws10_impoundment_{node_id}_c"),
                value: curve_stage,
                rule: "emergency rating-curve stage points must be strictly increasing",
            });
        }
        if stage <= curve_stage {
            let fraction = (stage - previous_stage) / (curve_stage - previous_stage);
            let projected = previous_discharge + fraction * (curve_discharge - previous_discharge);
            return Ok(projected.max(0.0));
        }
        previous_stage = curve_stage;
        previous_discharge = curve_discharge;
    }

    if stage_values.len() == 1 {
        return Ok(previous_discharge.max(0.0));
    }

    let last_index = stage_values.len() - 1;
    let stage_left = stage_values[last_index - 1];
    let stage_right = stage_values[last_index];
    let discharge_left = discharge_values[last_index - 1];
    let discharge_right = discharge_values[last_index];
    let slope = (discharge_right - discharge_left) / (stage_right - stage_left);
    let extrapolated = discharge_right + slope * (stage - stage_right);
    Ok(extrapolated.max(0.0))
}

fn filter_barrier_discharge_at_stage(
    node_id: usize,
    record: &ImpoundmentRecord,
    stage: f64,
) -> Result<f64, WatershedRuntimeInputError> {
    let Some(filter) = &record.filter_barrier else {
        return Ok(0.0);
    };

    if stage <= filter.hff {
        return Ok(0.0);
    }
    let through = filter.wdff * filter.vsl * (stage - filter.hff);
    let overtopping = if stage > filter.hotff {
        let delta = stage - filter.hotff;
        if record.filter_code == 1 {
            if filter.hotff <= filter.hff {
                return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                    symbol: format!("ws10_impoundment_{node_id}_c"),
                    value: filter.hotff - filter.hff,
                    rule: "filter overtopping stage must be > base stage",
                });
            }
            let b = 3.27 * filter.wdff;
            let c = (0.4 / (filter.hotff - filter.hff)) * filter.wdff;
            (b + c * delta) * delta.powf(1.5)
        } else {
            3.087 * filter.wdff * delta.powf(1.5)
        }
    } else {
        0.0
    };
    let discharge = through + overtopping;
    if !discharge.is_finite() || discharge < 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_c"),
            value: discharge,
            rule: "filter-barrier projected discharge must be finite and >= 0",
        });
    }
    Ok(discharge)
}

fn perforated_riser_reference_discharge(
    node_id: usize,
    record: &ImpoundmentRecord,
    reference_stage: f64,
) -> Result<Option<(f64, f64)>, WatershedRuntimeInputError> {
    let Some(riser) = &record.perforated_riser else {
        return Ok(None);
    };
    if !riser.diar.is_finite() || riser.diar <= 0.0 || !riser.diab.is_finite() || riser.diab <= 0.0
    {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_e"),
            value: if riser.diar <= 0.0 {
                riser.diar
            } else {
                riser.diab
            },
            rule: "riser diameters must be finite and > 0",
        });
    }
    let ko = (-0.60721 + 0.329_229 * (riser.diab / riser.diar)).exp();
    let denominator = 1.0 + riser.ke + riser.kb + riser.kc * riser.lbl + ko;
    if !denominator.is_finite() || denominator <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_e"),
            value: denominator,
            rule: "riser loss denominator must be finite and > 0",
        });
    }

    let ha = riser.hr - (riser.hrh + riser.sbl * riser.lbl - 0.6 * riser.diabl);
    let coefficient = std::f64::consts::PI * riser.diabl.powi(2) / 4.0
        * (2.0 * STANDARD_GRAVITY_M_S2).sqrt()
        / denominator.sqrt();
    if !coefficient.is_finite() || coefficient <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_e"),
            value: coefficient,
            rule: "riser projected coefficient must be finite and > 0",
        });
    }

    let stage = reference_stage.max(ha + ACTIVE_PROJECTION_STAGE_DELTA_M);
    let discharge = coefficient * (stage - ha).sqrt();
    if !discharge.is_finite() || discharge < 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_e"),
            value: discharge,
            rule: "riser projected discharge must be finite and >= 0",
        });
    }
    Ok(Some((discharge, ha)))
}

fn validate_active_projected_positive(
    node_id: usize,
    suffix: &str,
    value: f64,
    rule: &'static str,
) -> Result<(), WatershedRuntimeInputError> {
    if !value.is_finite() || value <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_{suffix}"),
            value,
            rule,
        });
    }
    Ok(())
}

fn derive_power_law_curve_coefficients(
    node_id: usize,
    curve_family: &'static str,
    stage: &[f64],
    response: &[f64],
    baseline: f64,
) -> Result<(f64, f64), WatershedRuntimeInputError> {
    if stage.is_empty() || stage.len() != response.len() {
        let stage_len = u32::try_from(stage.len()).map_err(|_| {
            WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                symbol: format!("ws10_impoundment_{node_id}_{curve_family}"),
                value: f64::INFINITY,
                rule: "stage/response vectors must have equal non-zero length",
            }
        })?;
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_{curve_family}"),
            value: f64::from(stage_len),
            rule: "stage/response vectors must have equal non-zero length",
        });
    }

    let mut log_stage = Vec::with_capacity(stage.len());
    let mut log_adjusted = Vec::with_capacity(stage.len());
    for (&stage_value, &response_value) in stage.iter().zip(response.iter()) {
        if !stage_value.is_finite() {
            return Err(WatershedRuntimeInputError::ImpoundmentSymbolNonFinite {
                symbol: format!("ws10_impoundment_{node_id}_{curve_family}_stage"),
                value: stage_value,
            });
        }
        if !response_value.is_finite() {
            return Err(WatershedRuntimeInputError::ImpoundmentSymbolNonFinite {
                symbol: format!("ws10_impoundment_{node_id}_{curve_family}_response"),
                value: response_value,
            });
        }
        if stage_value <= 0.0 {
            return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                symbol: format!("ws10_impoundment_{node_id}_{curve_family}"),
                value: stage_value,
                rule: "stage values must be > 0 for coefficient projection",
            });
        }
        let adjusted = response_value - baseline;
        if adjusted <= 0.0 {
            return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                symbol: format!("ws10_impoundment_{node_id}_{curve_family}"),
                value: adjusted,
                rule: "response-baseline values must be > 0 for coefficient projection",
            });
        }

        log_stage.push(stage_value.ln());
        log_adjusted.push(adjusted.ln());
    }

    let log_len_u32 = u32::try_from(log_stage.len()).map_err(|_| {
        WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_{curve_family}"),
            value: f64::INFINITY,
            rule: "stage/response vectors must have equal non-zero length",
        }
    })?;
    let log_len = f64::from(log_len_u32);
    let mean_x = log_stage.iter().sum::<f64>() / log_len;
    let mean_y = log_adjusted.iter().sum::<f64>() / log_len;

    let mut numerator = 0.0;
    let mut denominator = 0.0;
    for (&x, &y) in log_stage.iter().zip(log_adjusted.iter()) {
        let dx = x - mean_x;
        numerator += dx * (y - mean_y);
        denominator += dx * dx;
    }
    if denominator <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_{curve_family}"),
            value: denominator,
            rule: "stage values must span a non-degenerate range for coefficient projection",
        });
    }

    let exponent = numerator / denominator;
    let intercept = mean_y - exponent * mean_x;
    let slope = intercept.exp();
    if !slope.is_finite() || !exponent.is_finite() {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolNonFinite {
            symbol: format!("ws10_impoundment_{node_id}_{curve_family}"),
            value: if slope.is_finite() { exponent } else { slope },
        });
    }
    if slope <= 0.0 || exponent <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_{curve_family}"),
            value: if slope <= 0.0 { slope } else { exponent },
            rule: "derived slope and exponent must be > 0",
        });
    }

    Ok((slope, exponent))
}
