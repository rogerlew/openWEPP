#![allow(clippy::many_single_char_names)]
//! E07--E12 C3 `FvCB`, temperature response, and Medlyn coupling.
use crate::VegetationError;
use crate::numerics::{SolverDiagnostics, brent_dekker};

const R: f64 = 8.314_462_618_153_24;

pub fn smaller_root(a: f64, b: f64, c: f64) -> Result<f64, VegetationError> {
    if [a, b, c].iter().any(|v| !v.is_finite()) {
        return Err(VegetationError::Domain("quadratic"));
    }
    if a == 0.0 {
        if b == 0.0 {
            return Err(VegetationError::QuadraticDomain);
        }
        return Ok(-c / b);
    }
    let mut d = b.mul_add(b, -4.0 * a * c);
    let scale = (b * b).abs().max((4.0 * a * c).abs());
    if d < 0.0 {
        if d >= -64.0 * f64::EPSILON * scale {
            d = 0.0;
        } else {
            return Err(VegetationError::QuadraticDomain);
        }
    }
    let q = -0.5 * (b + b.signum() * d.sqrt());
    let r1 = q / a;
    let r2 = if q == 0.0 { -b / (2.0 * a) } else { c / q };
    Ok(r1.min(r2))
}
pub fn arrhenius(t: f64, ha: f64) -> Result<f64, VegetationError> {
    if !t.is_finite() || !ha.is_finite() || t <= 0.0 {
        return Err(VegetationError::Domain("temperature"));
    }
    let v = (ha / (R * 298.15) * (1.0 - 298.15 / t)).exp();
    if v.is_finite() {
        Ok(v)
    } else {
        Err(VegetationError::Domain("Arrhenius overflow"))
    }
}
pub fn peaked_response(t: f64, ha: f64, hd: f64, entropy: f64) -> Result<f64, VegetationError> {
    if ![t, ha, hd, entropy].iter().all(|value| value.is_finite()) || t <= 0.0 {
        return Err(VegetationError::Domain("peaked response"));
    }
    let log_arrhenius = ha / (R * 298.15) * (1.0 - 298.15 / t);
    let numerator_argument = (entropy * 298.15 - hd) / (R * 298.15);
    let denominator_argument = (entropy * t - hd) / (R * t);
    let v = (log_arrhenius + log_one_plus_exp(numerator_argument)
        - log_one_plus_exp(denominator_argument))
    .exp();
    if v.is_finite() {
        Ok(v)
    } else {
        Err(VegetationError::Domain("peaked response"))
    }
}
fn log_one_plus_exp(value: f64) -> f64 {
    if value > 0.0 {
        value + (-value).exp().ln_1p()
    } else {
        value.exp().ln_1p()
    }
}
pub fn electron_transport(par_abs: f64, jmax: f64) -> Result<f64, VegetationError> {
    if !par_abs.is_finite() || !jmax.is_finite() || par_abs < 0.0 || jmax <= 0.0 {
        return Err(VegetationError::Domain("electron transport"));
    }
    if par_abs == 0.0 {
        return Ok(0.0);
    }
    let ipsii = 0.5 * 0.85 * 4.6 * par_abs;
    smaller_root(0.7, -(ipsii + jmax), ipsii * jmax)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FvcbResult {
    pub ac: f64,
    pub aj: f64,
    pub ap: f64,
    pub ag: f64,
    pub an: f64,
    pub j: f64,
}
#[derive(Clone, Copy, Debug)]
pub struct FvcbInput {
    pub ci_pa: f64,
    pub oi_pa: f64,
    pub gamma_pa: f64,
    pub kc_pa: f64,
    pub ko_pa: f64,
    pub vcmax: f64,
    pub jmax: f64,
    pub tp: f64,
    pub rd: f64,
    pub par_abs: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CiSolution {
    pub ci_pa: f64,
    pub cs_pa: f64,
    pub gs_umol_h2o_m2_s: f64,
    pub rs_s_m: f64,
    pub fvcb: FvcbResult,
    pub diagnostics: SolverDiagnostics,
}

#[allow(clippy::too_many_arguments)]
pub fn solve_ci(
    mut biochemical: FvcbInput,
    ca_pa: f64,
    rb_s_m: f64,
    temperature_k: f64,
    vpd_kpa: f64,
    g0: f64,
    g1: f64,
    patm_pa: f64,
    beta_hyd: f64,
) -> Result<CiSolution, VegetationError> {
    if ![
        ca_pa,
        rb_s_m,
        temperature_k,
        vpd_kpa,
        g0,
        g1,
        patm_pa,
        beta_hyd,
    ]
    .iter()
    .all(|v| v.is_finite())
        || ca_pa <= 0.0
        || rb_s_m < 0.0
        || temperature_k <= 0.0
    {
        return Err(VegetationError::Domain("ci solve input"));
    }
    let lower = biochemical.gamma_pa;
    let evaluate = |ci: f64| -> Result<f64, VegetationError> {
        biochemical.ci_pa = ci;
        let photo = fvcb(biochemical)?;
        let cs = carbon_surface(ca_pa, rb_s_m, temperature_k, photo.an)?;
        let gs = medlyn(photo.an, g0, g1, vpd_kpa, cs, patm_pa, beta_hyd)?;
        if gs <= 0.0 {
            return Err(VegetationError::Domain("zero stomatal conductance"));
        }
        let gs_ms = gs * 1e-6 * R * temperature_k / patm_pa;
        let rs = 1.0 / gs_ms;
        Ok(ci - (ca_pa - (1.4 * rb_s_m + 1.6 * rs) * R * temperature_k * photo.an * 1e-6))
    };
    let (ci, diagnostics) = brent_dekker(lower, ca_pa, 1e-6, 1e-10, 1e-8, 64, evaluate)?;
    biochemical.ci_pa = ci;
    let photo = fvcb(biochemical)?;
    let cs = carbon_surface(ca_pa, rb_s_m, temperature_k, photo.an)?;
    let gs = medlyn(photo.an, g0, g1, vpd_kpa, cs, patm_pa, beta_hyd)?;
    let gs_ms = gs * 1e-6 * R * temperature_k / patm_pa;
    if gs_ms <= 0.0 || !gs_ms.is_finite() {
        return Err(VegetationError::Domain("stomatal resistance"));
    }
    Ok(CiSolution {
        ci_pa: ci,
        cs_pa: cs,
        gs_umol_h2o_m2_s: gs,
        rs_s_m: 1.0 / gs_ms,
        fvcb: photo,
        diagnostics,
    })
}
pub fn fvcb(i: FvcbInput) -> Result<FvcbResult, VegetationError> {
    if ![
        i.ci_pa, i.oi_pa, i.gamma_pa, i.kc_pa, i.ko_pa, i.vcmax, i.jmax, i.tp, i.rd, i.par_abs,
    ]
    .iter()
    .all(|value| value.is_finite())
        || i.ci_pa < 0.0
        || i.oi_pa < 0.0
        || i.gamma_pa < 0.0
        || i.kc_pa <= 0.0
        || i.ko_pa <= 0.0
        || i.vcmax <= 0.0
        || i.jmax <= 0.0
        || i.tp <= 0.0
        || i.rd < 0.0
        || i.par_abs < 0.0
    {
        return Err(VegetationError::Domain("FvCB"));
    }
    let j = electron_transport(i.par_abs, i.jmax)?;
    let (ac, aj) = if i.ci_pa < i.gamma_pa {
        (0.0, 0.0)
    } else {
        (
            i.vcmax * (i.ci_pa - i.gamma_pa) / (i.ci_pa + i.kc_pa * (1.0 + i.oi_pa / i.ko_pa)),
            j * (i.ci_pa - i.gamma_pa) / (4.0 * i.ci_pa + 8.0 * i.gamma_pa),
        )
    };
    let ap = 3.0 * i.tp;
    let ai = smaller_root(0.98, -(ac + aj), ac * aj)?;
    let ag = smaller_root(0.95, -(ai + ap), ai * ap)?;
    Ok(FvcbResult {
        ac,
        aj,
        ap,
        ag,
        an: ag - i.rd,
        j,
    })
}
pub fn medlyn(
    an: f64,
    g0: f64,
    g1: f64,
    vpd_kpa: f64,
    cs_pa: f64,
    patm_pa: f64,
    beta: f64,
) -> Result<f64, VegetationError> {
    if ![an, g0, g1, vpd_kpa, cs_pa, patm_pa, beta]
        .iter()
        .all(|v| v.is_finite())
        || g0 < 0.0
        || g1 < 0.0
        || vpd_kpa <= 0.0
        || cs_pa <= 0.0
        || patm_pa <= 0.0
        || !(0.0..=1.0).contains(&beta)
    {
        return Err(VegetationError::Domain("Medlyn"));
    }
    let potential = if an <= 0.0 {
        g0
    } else {
        g0 + 1.6 * (1.0 + g1 / vpd_kpa.sqrt()) * an / (cs_pa / patm_pa)
    };
    Ok(g0 + beta * (potential - g0))
}
pub fn carbon_surface(ca: f64, rb: f64, t: f64, an: f64) -> Result<f64, VegetationError> {
    if ![ca, rb, t, an].iter().all(|v| v.is_finite()) || ca <= 0.0 || rb < 0.0 || t <= 0.0 {
        return Err(VegetationError::Domain("carbon surface"));
    }
    let cs = ca - 1.4 * rb * R * t * an * 1e-6;
    if cs > 0.0 {
        Ok(cs)
    } else {
        Err(VegetationError::Domain("carbon surface pressure"))
    }
}
