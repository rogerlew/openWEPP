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
    derive_riser_apr_coefficients_from_points(node_id, &points)
}

fn derive_riser_apr_coefficients_from_points(
    node_id: usize,
    points: &[(f64, f64)],
) -> Result<(f64, f64), WatershedRuntimeInputError> {
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
    for &(hp, q) in points {
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

    let (culvert_coefficient, culvert_exponent, culvert_threshold) =
        if let Some((coefficient, exponent, threshold)) =
            derive_culvert_like_active_projection(node_id, record, reference_stage)?
        {
            active_projection_used = true;
            (coefficient, exponent, threshold)
        } else {
            (0.0, 1.0, record.hfull)
        };

    let (riser_coefficient, riser_threshold) = if let Some((coefficient, threshold)) =
        derive_riser_active_projection(node_id, record, reference_stage)?
    {
        active_projection_used = true;
        (coefficient, threshold)
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

fn derive_culvert_like_active_projection(
    node_id: usize,
    record: &ImpoundmentRecord,
    reference_stage: f64,
) -> Result<Option<(f64, f64, f64)>, WatershedRuntimeInputError> {
    let c_stage_thresholds = collect_culvert_like_stage_thresholds(record)?;
    if c_stage_thresholds.is_empty() {
        return Ok(None);
    }

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
    projected_discharge += culvert_pipe_discharge_at_stage(node_id, &record.culverts[0], stage)?;
    projected_discharge += culvert_pipe_discharge_at_stage(node_id, &record.culverts[1], stage)?;
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
    Ok(Some((
        culvert_coefficient,
        culvert_exponent,
        culvert_threshold,
    )))
}

fn collect_culvert_like_stage_thresholds(
    record: &ImpoundmentRecord,
) -> Result<Vec<f64>, WatershedRuntimeInputError> {
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
    Ok(c_stage_thresholds)
}

fn derive_riser_active_projection(
    node_id: usize,
    record: &ImpoundmentRecord,
    reference_stage: f64,
) -> Result<Option<(f64, f64)>, WatershedRuntimeInputError> {
    if let Some((riser_reference_discharge, riser_threshold)) =
        perforated_riser_reference_discharge(node_id, record, reference_stage)?
    {
        Ok(Some(derive_riser_coefficient_from_reference(
            node_id,
            riser_reference_discharge,
            riser_threshold,
            reference_stage,
        )?))
    } else {
        Ok(None)
    }
}

fn derive_riser_coefficient_from_reference(
    node_id: usize,
    riser_reference_discharge: f64,
    riser_threshold: f64,
    reference_stage: f64,
) -> Result<(f64, f64), WatershedRuntimeInputError> {
    let stage = reference_stage.max(riser_threshold + ACTIVE_PROJECTION_STAGE_DELTA_M);
    let span = stage - riser_threshold;
    if !span.is_finite() || span <= 0.0 {
        return Err(WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
            symbol: format!("ws10_impoundment_{node_id}_hlm"),
            value: span,
            rule: "reference-stage span above hlm must be finite and > 0",
        });
    }
    Ok((riser_reference_discharge / span, riser_threshold))
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

#[cfg(test)]
mod tests {
    use super::*;
    use openwepp_input_contract::parsers::watershed_impoundment::{
        CulvertParameters, DropSpillwayIds1Payload, DropSpillwayIds2Payload,
        DropSpillwayIds3Payload, DropSpillwayPayload, EmergencyOpenChannelPayload,
        EmergencyRatingCurvePayload, FilterBarrierPayload, PerforatedRiserPayload, RockfillPayload,
        StructureFlags,
    };

    #[track_caller]
    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() <= 1.0e-10,
            "actual {actual} differs from expected {expected}"
        );
    }

    #[track_caller]
    fn assert_positive_finite(value: f64) {
        assert!(value.is_finite(), "value should be finite: {value}");
        assert!(value > 0.0, "value should be positive: {value}");
    }

    #[track_caller]
    fn assert_domain_error(
        error: WatershedRuntimeInputError,
        expected_symbol: &str,
        expected_rule: &'static str,
    ) {
        match error {
            WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain { symbol, rule, .. } => {
                assert_eq!(symbol, expected_symbol);
                assert_eq!(rule, expected_rule);
            }
            other @ WatershedRuntimeInputError::ImpoundmentSymbolNonFinite { .. } => {
                panic!("expected out-of-domain error, got {other:?}");
            }
        }
    }

    #[track_caller]
    fn assert_nonfinite_error(error: WatershedRuntimeInputError, expected_symbol: &str) {
        match error {
            WatershedRuntimeInputError::ImpoundmentSymbolNonFinite { symbol, .. } => {
                assert_eq!(symbol, expected_symbol);
            }
            other @ WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain { .. } => {
                panic!("expected non-finite error, got {other:?}");
            }
        }
    }

    fn base_record() -> ImpoundmentRecord {
        ImpoundmentRecord {
            description: [
                "impoundment line 1".to_owned(),
                "impoundment line 2".to_owned(),
                "impoundment line 3".to_owned(),
            ],
            branch_comments: Vec::new(),
            ids: 0,
            culvert_icv: [0, 0],
            rockfill_code: 0,
            emergency_code: 0,
            filter_code: 0,
            riser_code: 0,
            hot: 1.5,
            hfull: 1.2,
            h: 0.4,
            deltat: 60.0,
            qinf: 0.01,
            isize: 1,
            ndiv: 5,
            nalpts: 3,
            hmin: 0.0,
            a0: 20.0,
            l0: 5.0,
            stage: vec![0.5, 1.0, 1.5],
            area: vec![30.0, 50.0, 80.0],
            length: vec![8.0, 12.0, 18.0],
            drop_spillway: DropSpillwayPayload::None,
            culverts: [
                CulvertPayload {
                    icv: 0,
                    ncv: 0,
                    comment: None,
                    parameters: None,
                },
                CulvertPayload {
                    icv: 0,
                    ncv: 0,
                    comment: None,
                    parameters: None,
                },
            ],
            rockfill: None,
            emergency_spillway: EmergencySpillwayPayload::None,
            filter_barrier: None,
            perforated_riser: None,
            structure_flags: StructureFlags {
                has_drop_spillway: false,
                has_culvert_1: false,
                has_culvert_2: false,
                has_rockfill: false,
                has_emergency_spillway: false,
                has_filter_barrier: false,
                has_perforated_riser: false,
            },
        }
    }

    fn drop_ids1() -> DropSpillwayPayload {
        DropSpillwayPayload::Ids1 {
            comment: "ids1".to_owned(),
            payload: DropSpillwayIds1Payload {
                diars: 0.60,
                hrs: 0.25,
                coefw: 3.20,
                coefo: 0.60,
                diabl: 0.40,
                hrh: 0.20,
                lbl: 2.00,
                sbl: 0.01,
                hblot: 0.10,
                ke: 0.50,
                kb: 0.20,
                kc: 0.30,
            },
        }
    }

    fn drop_ids2() -> DropSpillwayPayload {
        DropSpillwayPayload::Ids2 {
            comment: "ids2".to_owned(),
            payload: DropSpillwayIds2Payload {
                lenrs: 1.10,
                widrs: 1.20,
                hrs: 0.50,
                coefw: 3.20,
                coefo: 0.60,
                diabl: 0.45,
                hrh: 0.30,
                lbl: 2.00,
                sbl: 0.01,
                hblot: 0.10,
                ke: 0.50,
                kb: 0.20,
                kc: 0.30,
            },
        }
    }

    fn drop_ids3() -> DropSpillwayPayload {
        DropSpillwayPayload::Ids3 {
            comment: "ids3".to_owned(),
            payload: DropSpillwayIds3Payload {
                lenrs: 1.10,
                widrs: 1.20,
                hrs: 0.50,
                coefw: 3.20,
                coefo: 0.60,
                hitbl: 0.45,
                wdbl: 0.30,
                hrh: 0.30,
                lbl: 2.00,
                sbl: 0.01,
                hblot: 0.11,
                ke: 0.50,
                kb: 0.20,
                kc: 0.30,
            },
        }
    }

    fn culvert_payload(hcv: f64) -> CulvertPayload {
        CulvertPayload {
            icv: 1,
            ncv: 2,
            comment: Some("culvert".to_owned()),
            parameters: Some(CulvertParameters {
                arcv: 0.15,
                hitcv: 0.40,
                hcv,
                lcv: 2.00,
                scv: 0.01,
                hcvot: 0.05,
                ke: 0.50,
                kb: 0.20,
                kc: 0.30,
                kus: 0.90,
                mus: 0.70,
                cs: 0.80,
                ys: 0.10,
            }),
        }
    }

    fn rockfill_payload() -> RockfillPayload {
        RockfillPayload {
            comment: "rockfill".to_owned(),
            lnrf: 1.20,
            hrf: 0.45,
            hotrf: 0.75,
            wdrf: 0.80,
            diarf: 0.35,
        }
    }

    fn emergency_rating_curve() -> EmergencySpillwayPayload {
        EmergencySpillwayPayload::RatingCurve {
            comment: "rating".to_owned(),
            payload: EmergencyRatingCurvePayload {
                hes: 0.50,
                hest: vec![0.60, 0.80, 1.10],
                qes: vec![0.01, 0.03, 0.05],
            },
        }
    }

    fn emergency_open_channel() -> EmergencySpillwayPayload {
        EmergencySpillwayPayload::OpenChannel {
            comment: "open".to_owned(),
            payload: EmergencyOpenChannelPayload {
                bwes: 1.00,
                sses: 2.00,
                nes: 0.04,
                hes: 0.50,
                hmxes: 1.10,
                ses1: 0.01,
                les1: 3.00,
                ses2: 0.02,
                les2: 4.00,
                ses3: 0.03,
            },
        }
    }

    fn filter_payload() -> FilterBarrierPayload {
        FilterBarrierPayload {
            comment: "filter".to_owned(),
            vsl: 0.02,
            wdff: 0.50,
            hff: 0.40,
            hotff: 0.70,
        }
    }

    fn riser_payload() -> PerforatedRiserPayload {
        PerforatedRiserPayload {
            comment: "riser".to_owned(),
            hr: 0.80,
            hb: 0.10,
            hs: 0.30,
            hd: 0.05,
            diar: 0.60,
            as_slot: 0.02,
            diab: 0.40,
            hrh: 0.20,
            lbl: 2.00,
            sbl: 0.01,
            diabl: 0.35,
            cb: 0.60,
            coefw: 3.20,
            coefo: 0.60,
            cs: 0.80,
            ke: 0.50,
            kb: 0.20,
            kc: 0.30,
        }
    }

    fn active_record() -> ImpoundmentRecord {
        let mut record = base_record();
        record.ids = 1;
        record.culvert_icv = [1, 1];
        record.rockfill_code = 1;
        record.emergency_code = 2;
        record.filter_code = 1;
        record.riser_code = 1;
        record.drop_spillway = drop_ids1();
        record.culverts = [culvert_payload(0.30), culvert_payload(0.38)];
        record.rockfill = Some(rockfill_payload());
        record.emergency_spillway = emergency_rating_curve();
        record.filter_barrier = Some(filter_payload());
        record.perforated_riser = Some(riser_payload());
        record.structure_flags = StructureFlags {
            has_drop_spillway: true,
            has_culvert_1: true,
            has_culvert_2: true,
            has_rockfill: true,
            has_emergency_spillway: true,
            has_filter_barrier: true,
            has_perforated_riser: true,
        };
        record
    }

    #[test]
    fn inactive_impoundment_projection_preserves_defaults() {
        let record = base_record();
        let coefficients = derive_ws12_impoundment_coefficients(7, &record)
            .expect("inactive record should project");
        assert_close(coefficients[0].1, 0.0);
        assert_close(coefficients[1].1, 1.0);
        assert_close(coefficients[5].1, record.hfull);
        assert_close(coefficients[8].1, record.a0);
        assert_close(coefficients[11].1, record.l0);

        let families = derive_ws12_outflow_function_families(7, &record)
            .expect("inactive families should project");
        for family_index in 1..=WS12_FUNCTION_COUNT {
            assert_close(families.coefficient_at(family_index, "a"), 0.0);
            assert_close(families.coefficient_at(family_index, "ha"), record.hfull);
        }
    }

    #[test]
    fn active_impoundment_projection_covers_all_function_families() {
        let record = active_record();
        let projection =
            derive_ws12_impoundment_coefficients(3, &record).expect("active record should project");
        assert_close(projection[0].1, 3.20 * std::f64::consts::PI * 0.60);
        assert_close(projection[1].1, 1.5);
        assert_close(projection[2].1, 3.358_440_976_702_923_3);
        assert_close(projection[3].1, 0.5);
        assert_close(projection[4].1, 0.385_570_068_949_592_17);
        assert_close(projection[5].1, 0.25);
        assert_close(projection[6].1, 0.40);
        assert_close(projection[7].1, 0.79);

        let families = derive_ws12_outflow_function_families(3, &record)
            .expect("active families should project");
        for family_index in 1..=WS12_FUNCTION_COUNT {
            assert!(families.coefficient_at(family_index, "ha").is_finite());
        }
        assert_close(
            families.coefficient_at(1, "b"),
            3.20 * std::f64::consts::PI * 0.60,
        );
        assert_close(families.coefficient_at(4, "a"), 0.189_736_659_610_102_75);
        assert_close(families.coefficient_at(4, "b"), 0.36);
        assert_close(families.coefficient_at(4, "c"), 1.428_571_428_571_428_6);
        assert_close(families.coefficient_at(4, "ha"), 0.30);
        assert_positive_finite(families.coefficient_at(7, "a"));
        assert_close(families.coefficient_at(10, "a"), 0.80);
        assert_close(families.coefficient_at(10, "b"), 3.063_002_699_620_817_7);
        assert_close(families.coefficient_at(10, "c"), 1.500_955_990_208_308_5);
        assert_close(families.coefficient_at(10, "d"), 2.469_6);
        assert_close(families.coefficient_at(10, "e"), 0.75);
        assert_close(families.coefficient_at(10, "ha"), 0.45);
        assert_close(families.coefficient_at(11, "a"), 0.0);
        assert_close(families.coefficient_at(11, "b"), 0.092_777_777_777_777_8);
        assert_close(families.coefficient_at(11, "c"), 0.102_777_777_777_777_84);
        assert_close(families.coefficient_at(11, "d"), -0.327_160_493_827_160_8);
        assert_close(families.coefficient_at(11, "e"), 0.216_049_382_716_049_62);
        assert_close(families.coefficient_at(11, "ha"), 0.50);
        assert_close(families.coefficient_at(12, "a"), 0.01);
        assert_close(families.coefficient_at(12, "b"), 1.635);
        assert_close(families.coefficient_at(12, "c"), 0.666_666_666_666_666_9);
        assert_close(families.coefficient_at(12, "d"), 0.70);
        assert_close(families.coefficient_at(12, "ha"), 0.40);
        assert_positive_finite(families.coefficient_at(13, "b"));
        assert_close(families.coefficient_at(14, "a"), 0.333_915_400_830_710_8);
        assert_close(families.coefficient_at(14, "ha"), -0.05);
        assert_close(families.coefficient_at(15, "b"), 0.246_885_305_371_978_8);
        assert_close(families.coefficient_at(15, "c"), 0.50);
        assert_close(families.coefficient_at(15, "ha"), 0.79);
    }

    #[test]
    fn drop_spillway_ids2_and_ids3_project_function_families() {
        for drop_spillway in [drop_ids2(), drop_ids3()] {
            let mut record = base_record();
            record.drop_spillway = drop_spillway;
            record.structure_flags.has_drop_spillway = true;
            let projection = derive_ws12_active_structure_projection(5, &record)
                .expect("drop-only active projection should succeed");
            assert_positive_finite(projection.drop_coefficient);
            assert_close(projection.drop_exponent, 1.5);

            let families = derive_ws12_outflow_function_families(5, &record)
                .expect("drop-only families should project");
            assert_positive_finite(families.coefficient_at(1, "b"));
            assert_positive_finite(families.coefficient_at(2, "b"));
            assert_positive_finite(families.coefficient_at(3, "b"));
        }
    }

    #[test]
    fn emergency_open_channel_and_filter_modes_are_projected() {
        let mut record = base_record();
        record.emergency_code = 1;
        record.filter_code = 2;
        record.emergency_spillway = emergency_open_channel();
        record.filter_barrier = Some(filter_payload());
        record.structure_flags.has_emergency_spillway = true;
        record.structure_flags.has_filter_barrier = true;

        let projection = derive_ws12_active_structure_projection(11, &record)
            .expect("open channel and filter projection should succeed");
        assert_positive_finite(projection.culvert_coefficient);
        assert_close(projection.culvert_exponent, 0.5);

        let families = derive_ws12_outflow_function_families(11, &record)
            .expect("open channel and filter families should project");
        assert!(families.coefficient_at(11, "a").is_finite());
        assert_positive_finite(families.coefficient_at(12, "b"));
        assert_close(families.coefficient_at(12, "c"), 0.0);
    }

    #[test]
    fn riser_sampling_regression_and_qs_branches_are_characterized() {
        let riser = riser_payload();
        let points = sample_riser_unsubmerged_curve(13, &riser)
            .expect("riser sampling should produce points");
        assert!(points.len() >= 2);
        assert!(points.iter().all(|(head, q)| *head > 0.0 && *q > 0.0));

        let (apr1, apr2) =
            derive_riser_apr_coefficients(13, &riser).expect("riser regression should fit");
        assert!(apr1.is_finite());
        assert!(apr2.is_finite());
        assert!(derive_riser_apr_coefficients_from_points(13, &[]).is_err());
        assert!(derive_riser_apr_coefficients_from_points(13, &[(0.0, 1.0), (1.0, 1.0)]).is_err());
        assert!(derive_riser_apr_coefficients_from_points(13, &[(1.0, 1.0), (1.0, 2.0)]).is_err());
        let subnormal = f64::from_bits(1);
        assert!(
            derive_riser_apr_coefficients_from_points(13, &[(1.0, subnormal), (2.0, subnormal)])
                .is_err()
        );

        let ko = (-0.60721 + 0.329_229 * (riser.diab / riser.diar)).exp();
        for (hp, y) in [
            (0.20, -0.01),
            (0.20, 0.05),
            (0.50, -0.01),
            (0.50, 0.10),
            (0.50, 0.40),
            (1.00, -0.01),
            (1.00, 0.10),
            (1.00, 0.40),
        ] {
            let qs = compute_riser_qs(hp, y, ko, &riser).expect("riser qs should evaluate");
            assert!(qs >= 0.0);
        }
    }

    #[test]
    fn quartic_fit_and_solver_cover_success_and_failure_modes() {
        let coefficients = fit_quartic_least_squares(2, &[(0.0, 0.0), (1.0, 1.0)], "f11")
            .expect("short point sets should be padded and fit");
        assert!(coefficients.iter().all(|value| value.is_finite()));

        let matrix = [
            [2.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 3.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 4.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 5.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 6.0],
        ];
        let solved = solve_linear_system_5x5(matrix, [2.0, 6.0, 12.0, 20.0, 30.0])
            .expect("diagonal system should solve");
        for (&actual, expected) in solved.iter().zip([1.0, 2.0, 3.0, 4.0, 5.0]) {
            assert_close(actual, expected);
        }

        assert!(solve_linear_system_5x5([[0.0; 5]; 5], [0.0; 5]).is_none());
        assert!(fit_quartic_least_squares(2, &[], "f11").is_err());
        assert!(fit_quartic_least_squares(2, &[(f64::NAN, 1.0)], "f11").is_err());
        assert!(fit_quartic_least_squares(2, &[(1.0, 1.0); 5], "f11").is_err());
    }

    #[test]
    fn discharge_helpers_cover_thresholds_interpolation_and_errors() {
        let record = active_record();
        assert_close(
            culvert_pipe_discharge_at_stage(
                4,
                &CulvertPayload {
                    icv: 0,
                    ncv: 0,
                    comment: None,
                    parameters: None,
                },
                1.0,
            )
            .expect("inactive culvert should return zero"),
            0.0,
        );
        assert_close(
            culvert_pipe_discharge_at_stage(4, &record.culverts[0], 0.10)
                .expect("below-threshold culvert should return zero"),
            0.0,
        );
        assert_positive_finite(
            culvert_pipe_discharge_at_stage(4, &record.culverts[0], 1.0)
                .expect("active culvert should discharge"),
        );

        assert_close(
            rockfill_discharge_at_stage(4, &base_record(), 1.0)
                .expect("absent rockfill should return zero"),
            0.0,
        );
        assert_close(
            rockfill_discharge_at_stage(4, &record, 0.10)
                .expect("below-threshold rockfill should return zero"),
            0.0,
        );
        assert_positive_finite(
            rockfill_discharge_at_stage(4, &record, 1.0)
                .expect("rockfill should discharge above threshold"),
        );

        assert_close(
            emergency_discharge_at_stage(4, &base_record(), 1.0)
                .expect("absent emergency spillway should return zero"),
            0.0,
        );
        assert_close(
            emergency_discharge_at_stage(4, &record, 0.10)
                .expect("below-threshold emergency spillway should return zero"),
            0.0,
        );
        assert_positive_finite(
            emergency_discharge_at_stage(4, &record, 0.70)
                .expect("rating curve should interpolate"),
        );
        assert_positive_finite(
            emergency_discharge_at_stage(4, &record, 1.30)
                .expect("rating curve should extrapolate"),
        );
        assert_positive_finite(
            interpolate_rating_curve_discharge(4, 0.5, &[0.6], &[0.1], 0.8)
                .expect("single-point rating curve should hold last value"),
        );
        assert!(interpolate_rating_curve_discharge(4, 0.5, &[0.6], &[f64::NAN], 0.55).is_err());
        assert!(interpolate_rating_curve_discharge(4, 0.5, &[0.4], &[0.1], 0.55).is_err());

        assert_close(
            filter_barrier_discharge_at_stage(4, &base_record(), 1.0)
                .expect("absent filter should return zero"),
            0.0,
        );
        assert_close(
            filter_barrier_discharge_at_stage(4, &record, 0.10)
                .expect("below-threshold filter should return zero"),
            0.0,
        );
        assert_positive_finite(
            filter_barrier_discharge_at_stage(4, &record, 0.60)
                .expect("filter through-flow should discharge"),
        );
        assert_positive_finite(
            filter_barrier_discharge_at_stage(4, &record, 0.90)
                .expect("filter overtopping should discharge"),
        );

        let mut code_two = record.clone();
        code_two.filter_code = 2;
        assert_positive_finite(
            filter_barrier_discharge_at_stage(4, &code_two, 0.90)
                .expect("straw-bale overtopping should discharge"),
        );
    }

    #[test]
    fn projection_guards_preserve_error_classes() {
        let mut active_without_payload = base_record();
        active_without_payload.structure_flags.has_culvert_1 = true;
        active_without_payload.culverts[0].icv = 1;
        assert!(derive_ws12_active_structure_projection(9, &active_without_payload).is_err());

        let mut invalid_culvert = culvert_payload(0.30);
        invalid_culvert.parameters.as_mut().expect("parameters").ke = f64::NAN;
        assert!(culvert_pipe_discharge_at_stage(9, &invalid_culvert, 1.0).is_err());

        let mut invalid_count = culvert_payload(0.30);
        invalid_count.ncv = 0;
        assert!(culvert_pipe_discharge_at_stage(9, &invalid_count, 1.0).is_err());

        let mut invalid_rockfill = active_record();
        invalid_rockfill.rockfill.as_mut().expect("rockfill").diarf = 0.0;
        assert!(
            project_rockfill_function(
                9,
                &invalid_rockfill,
                &mut Ws12OutflowFunctionFamilies::inactive_default(1.0)
            )
            .is_err()
        );
        assert!(rockfill_discharge_at_stage(9, &invalid_rockfill, 1.0).is_err());

        let mut invalid_filter = active_record();
        invalid_filter
            .filter_barrier
            .as_mut()
            .expect("filter")
            .hotff = 0.30;
        assert!(
            project_filter_function(
                9,
                &invalid_filter,
                &mut Ws12OutflowFunctionFamilies::inactive_default(1.0)
            )
            .is_err()
        );
        assert!(filter_barrier_discharge_at_stage(9, &invalid_filter, 0.90).is_err());

        let mut invalid_riser = riser_payload();
        invalid_riser.diar = 0.0;
        let mut invalid_riser_record = active_record();
        invalid_riser_record.perforated_riser = Some(invalid_riser);
        assert!(
            project_riser_functions(
                9,
                &invalid_riser_record,
                &mut Ws12OutflowFunctionFamilies::inactive_default(1.0)
            )
            .is_err()
        );

        let mut nonfinite_riser = riser_payload();
        nonfinite_riser.cs = f64::NAN;
        assert!(sample_riser_unsubmerged_curve(9, &nonfinite_riser).is_err());
    }

    #[test]
    fn projection_guards_cover_contract_boundary_failures() {
        let families = Ws12OutflowFunctionFamilies::inactive_default(1.0);
        assert!(
            std::panic::catch_unwind(|| families.coefficient_at(1, "unsupported")).is_err(),
            "unsupported coefficient suffix should remain unreachable"
        );

        let mut bad_area = base_record();
        bad_area.h = f64::NAN;
        assert_domain_error(
            derive_ws12_impoundment_coefficients(17, &bad_area).expect_err("NaN area denominator"),
            "ws10_impoundment_17_a0",
            "derived stage-area denominator at current stage must be finite and > 0",
        );
        assert!(validate_active_projected_positive(17, "a", 1.0, "valid").is_ok());
        assert_domain_error(
            validate_active_projected_positive(17, "a", 0.0, "invalid")
                .expect_err("zero coefficient should fail"),
            "ws10_impoundment_17_a",
            "invalid",
        );

        let mut flagged_without_payload = base_record();
        flagged_without_payload.structure_flags.has_rockfill = true;
        assert!(derive_ws12_active_structure_projection(17, &flagged_without_payload).is_err());
        assert!(
            derive_riser_active_projection(17, &base_record(), 1.0)
                .expect("absent riser projection should be valid")
                .is_none()
        );
        let (coefficient, threshold) = derive_riser_coefficient_from_reference(17, 0.5, 0.25, 1.0)
            .expect("finite riser span should project");
        assert_positive_finite(coefficient);
        assert_close(threshold, 0.25);
        assert!(derive_riser_coefficient_from_reference(17, 0.5, f64::INFINITY, 1.0).is_err());

        let mut nonfinite_reference = active_record();
        nonfinite_reference.h = f64::NAN;
        nonfinite_reference.hfull = f64::NAN;
        assert_nonfinite_error(
            derive_active_projection_reference_stage(17, &nonfinite_reference)
                .expect_err("non-finite reference stage"),
            "ws10_impoundment_17_h",
        );

        let mut nonfinite_threshold = base_record();
        nonfinite_threshold.structure_flags.has_rockfill = true;
        nonfinite_threshold.rockfill = Some(RockfillPayload {
            hrf: f64::NAN,
            ..rockfill_payload()
        });
        assert!(derive_culvert_like_active_projection(17, &nonfinite_threshold, 1.0).is_err());
    }

    #[test]
    fn family_projection_guards_cover_invalid_active_payloads() {
        for mut drop_spillway in [drop_ids1(), drop_ids2(), drop_ids3()] {
            match &mut drop_spillway {
                DropSpillwayPayload::Ids1 { payload, .. } => payload.kc = f64::NAN,
                DropSpillwayPayload::Ids2 { payload, .. } => payload.kc = f64::NAN,
                DropSpillwayPayload::Ids3 { payload, .. } => payload.kc = f64::NAN,
                DropSpillwayPayload::None => unreachable!("test creates active drops"),
            }
            let mut record = base_record();
            record.drop_spillway = drop_spillway;
            record.structure_flags.has_drop_spillway = true;
            assert!(derive_ws12_outflow_function_families(19, &record).is_err());
        }

        let mut families = Ws12OutflowFunctionFamilies::inactive_default(1.0);
        let mut missing_culvert = culvert_payload(0.30);
        missing_culvert.parameters = None;
        assert!(project_culvert_function_families(19, &missing_culvert, 4, &mut families).is_err());
        assert!(culvert_stage_threshold(&missing_culvert).is_err());

        let mut bad_denominator = culvert_payload(0.30);
        bad_denominator.parameters.as_mut().expect("parameters").kc = f64::NAN;
        assert!(project_culvert_function_families(19, &bad_denominator, 4, &mut families).is_err());

        let mut bad_count = culvert_payload(0.30);
        bad_count.ncv = 0;
        assert!(project_culvert_function_families(19, &bad_count, 4, &mut families).is_err());

        let mut bad_mus = culvert_payload(0.30);
        bad_mus.parameters.as_mut().expect("parameters").mus = 0.0;
        assert!(project_culvert_function_families(19, &bad_mus, 4, &mut families).is_err());

        let mut bad_cs = culvert_payload(0.30);
        bad_cs.parameters.as_mut().expect("parameters").cs = 0.0;
        assert!(project_culvert_function_families(19, &bad_cs, 4, &mut families).is_err());

        let mut bad_rockfill = active_record();
        bad_rockfill.rockfill.as_mut().expect("rockfill").lnrf = 0.0;
        assert!(project_rockfill_function(19, &bad_rockfill, &mut families).is_err());

        let mut bad_emergency = active_record();
        bad_emergency.emergency_spillway = EmergencySpillwayPayload::RatingCurve {
            comment: "bad rating".to_owned(),
            payload: EmergencyRatingCurvePayload {
                hes: 0.5,
                hest: Vec::new(),
                qes: Vec::new(),
            },
        };
        assert!(project_emergency_function(19, &bad_emergency, &mut families).is_err());

        let mut bad_riser = active_record();
        bad_riser.perforated_riser.as_mut().expect("riser").kc = f64::NAN;
        assert!(project_riser_functions(19, &bad_riser, &mut families).is_err());
    }

    #[test]
    fn discharge_guards_cover_invalid_payloads() {
        let mut missing_culvert = culvert_payload(0.30);
        missing_culvert.parameters = None;
        assert!(culvert_pipe_discharge_at_stage(23, &missing_culvert, 1.0).is_err());

        let mut zero_coefficient_culvert = culvert_payload(0.30);
        zero_coefficient_culvert
            .parameters
            .as_mut()
            .expect("parameters")
            .arcv = 0.0;
        assert!(culvert_pipe_discharge_at_stage(23, &zero_coefficient_culvert, 1.0).is_err());

        let mut bad_rockfill = active_record();
        bad_rockfill.rockfill.as_mut().expect("rockfill").lnrf = 0.0;
        assert!(rockfill_discharge_at_stage(23, &bad_rockfill, 1.0).is_err());

        let mut negative_rockfill = active_record();
        negative_rockfill.rockfill.as_mut().expect("rockfill").wdrf = -0.80;
        assert!(rockfill_discharge_at_stage(23, &negative_rockfill, 1.0).is_err());

        let mut bad_open_channel = base_record();
        bad_open_channel.emergency_spillway = emergency_open_channel();
        if let EmergencySpillwayPayload::OpenChannel { payload, .. } =
            &mut bad_open_channel.emergency_spillway
        {
            payload.bwes = f64::NAN;
        }
        assert!(emergency_discharge_at_stage(23, &bad_open_channel, 1.0).is_err());

        let mut bad_rating = base_record();
        bad_rating.emergency_spillway = EmergencySpillwayPayload::RatingCurve {
            comment: "bad rating".to_owned(),
            payload: EmergencyRatingCurvePayload {
                hes: 0.5,
                hest: vec![0.6, 0.7],
                qes: vec![0.1],
            },
        };
        assert!(emergency_discharge_at_stage(23, &bad_rating, 1.0).is_err());

        assert!(interpolate_rating_curve_discharge(23, 0.5, &[f64::NAN], &[0.1], 0.55).is_err());

        let mut bad_filter = active_record();
        bad_filter.filter_barrier.as_mut().expect("filter").vsl = f64::NAN;
        assert!(filter_barrier_discharge_at_stage(23, &bad_filter, 0.60).is_err());

        assert!(
            perforated_riser_reference_discharge(23, &base_record(), 1.0)
                .expect("absent riser should be valid")
                .is_none()
        );

        let mut bad_riser_record = active_record();
        bad_riser_record
            .perforated_riser
            .as_mut()
            .expect("riser")
            .diab = 0.0;
        assert!(perforated_riser_reference_discharge(23, &bad_riser_record, 1.0).is_err());

        let mut bad_riser_denominator = active_record();
        bad_riser_denominator
            .perforated_riser
            .as_mut()
            .expect("riser")
            .kc = f64::NAN;
        assert!(perforated_riser_reference_discharge(23, &bad_riser_denominator, 1.0).is_err());

        let mut bad_riser_coefficient = active_record();
        bad_riser_coefficient
            .perforated_riser
            .as_mut()
            .expect("riser")
            .diabl = 0.0;
        assert!(perforated_riser_reference_discharge(23, &bad_riser_coefficient, 1.0).is_err());
    }

    #[test]
    fn piecewise_helpers_cover_remaining_intervals() {
        for length in [0.25, 0.75, 1.50, 2.50, 3.50] {
            assert!(rockfill_arf(length, 0.35).is_finite());
        }

        let mut riser = riser_payload();
        riser.hr = 0.01;
        riser.hs = 0.01;
        let points = sample_riser_unsubmerged_curve(29, &riser)
            .expect("small riser should still terminate cleanly");
        assert!(points.len() <= 99);
    }

    #[test]
    fn power_law_projection_covers_valid_and_invalid_domains() {
        let (slope, exponent) = derive_power_law_curve_coefficients(
            1,
            "area",
            &[0.5, 1.0, 1.5],
            &[30.0, 50.0, 80.0],
            20.0,
        )
        .expect("valid power law should fit");
        assert_positive_finite(slope);
        assert_positive_finite(exponent);

        assert!(derive_power_law_curve_coefficients(1, "area", &[], &[], 20.0).is_err());
        assert!(
            derive_power_law_curve_coefficients(1, "area", &[f64::NAN], &[30.0], 20.0).is_err()
        );
        assert!(derive_power_law_curve_coefficients(1, "area", &[0.0], &[30.0], 20.0).is_err());
        assert!(derive_power_law_curve_coefficients(1, "area", &[1.0], &[20.0], 20.0).is_err());
        assert!(
            derive_power_law_curve_coefficients(1, "area", &[1.0, 1.0], &[30.0, 40.0], 20.0)
                .is_err()
        );
        assert!(derive_power_law_curve_coefficients(1, "area", &[1.0], &[f64::NAN], 20.0).is_err());
        assert!(
            derive_power_law_curve_coefficients(1, "area", &[1.0, 2.0], &[40.0, 30.0], 20.0)
                .is_err()
        );
    }
}
