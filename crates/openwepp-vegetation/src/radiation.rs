#![allow(clippy::many_single_char_names)]
//! E01--E03 radiation primitives and ordered canopy traversal.
use crate::VegetationError;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RadiationBand {
    pub direct: f64,
    pub diffuse: f64,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ClassRadiation {
    pub sunlit_lai: f64,
    pub shaded_lai: f64,
    pub sunlit_absorbed: f64,
    pub shaded_absorbed: f64,
    pub transmitted: RadiationBand,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TwoStreamResult {
    pub absorbed: f64,
    pub reflected: f64,
    pub transmitted_direct: f64,
    pub transmitted_diffuse: f64,
    pub sunlit_lai: f64,
    pub closure_residual: f64,
}

/// Solves the admitted Sellers/CLM two-stream ODEs using deterministic RK4.
#[allow(clippy::too_many_arguments)]
pub fn two_stream(
    plant_area: f64,
    mu: f64,
    chi: f64,
    rho: f64,
    tau: f64,
    ground_albedo: f64,
    direct: f64,
    diffuse: f64,
) -> Result<TwoStreamResult, VegetationError> {
    if !(-0.4..=0.6).contains(&chi)
        || plant_area < 0.0
        || direct < 0.0
        || diffuse < 0.0
        || rho < 0.0
        || tau < 0.0
        || rho + tau >= 1.0
        || !(0.0..=1.0).contains(&ground_albedo)
        || (direct > 0.0 && mu <= 0.0)
    {
        return Err(VegetationError::Domain("two-stream radiation"));
    }
    if plant_area == 0.0 {
        return Ok(TwoStreamResult {
            absorbed: 0.0,
            reflected: ground_albedo * (direct + diffuse),
            transmitted_direct: direct,
            transmitted_diffuse: diffuse,
            sunlit_lai: 0.0,
            closure_residual: 0.0,
        });
    }
    let phi1 = 0.5 - 0.633 * chi - 0.33 * chi * chi;
    let phi2 = 0.877 * (1.0 - 2.0 * phi1);
    let gmu = phi1 + phi2 * mu;
    let kbeam = if direct > 0.0 { gmu / mu } else { 0.0 };
    let mubar = adaptive_simpson(|mup| mup / (phi1 + phi2 * mup), 0.0, 1.0, 1e-14, 20)?;
    let omega = rho + tau;
    let cosbar = f64::midpoint(1.0, chi);
    let omega_beta = 0.5 * (rho + tau + (rho - tau) * cosbar * cosbar);
    let b = 1.0 - omega + omega_beta;
    let c = omega_beta;
    let integral = if direct > 0.0 {
        adaptive_simpson(
            |mup| {
                let gp = phi1 + phi2 * mup;
                let denominator = mu * gp + mup * gmu;
                if denominator == 0.0 {
                    0.0
                } else {
                    mup * gmu / denominator
                }
            },
            0.0,
            1.0,
            1e-14,
            20,
        )?
    } else {
        0.0
    };
    let scatter = 0.5 * omega * integral;
    let beta0 = if omega > 0.0 && direct > 0.0 {
        (1.0 + mubar * kbeam) / (mubar * kbeam) * scatter / omega
    } else {
        0.0
    };
    let d = omega * mubar * kbeam * beta0;
    let f = omega * mubar * kbeam * (1.0 - beta0);
    let integrate =
        |up0: f64| integrate_odes(up0, diffuse, plant_area, direct, kbeam, mubar, b, c, d, f);
    let base = integrate(0.0);
    let unit = integrate(1.0);
    let slope_up = unit.0 - base.0;
    let slope_down = unit.1 - base.1;
    let terminal_direct = direct * (-kbeam * plant_area).exp();
    let denominator = slope_up - ground_albedo * slope_down;
    if denominator.abs() < 64.0 * f64::EPSILON {
        return Err(VegetationError::Domain("two-stream boundary"));
    }
    let reflected = (ground_albedo * (base.1 + terminal_direct) - base.0) / denominator;
    let terminal = integrate(reflected);
    let transmitted_diffuse = terminal.1;
    let transmitted = transmitted_diffuse + terminal_direct;
    let incident = direct + diffuse;
    let absorbed = incident - reflected - (1.0 - ground_albedo) * transmitted;
    let closure = incident - absorbed - reflected - (1.0 - ground_albedo) * transmitted;
    let sunlit_lai = if direct > 0.0 {
        -(-kbeam * plant_area).exp_m1() / kbeam
    } else {
        0.0
    };
    Ok(TwoStreamResult {
        absorbed,
        reflected,
        transmitted_direct: terminal_direct,
        transmitted_diffuse,
        sunlit_lai,
        closure_residual: closure,
    })
}

#[allow(clippy::too_many_arguments)]
fn integrate_odes(
    up0: f64,
    down0: f64,
    area: f64,
    direct: f64,
    kb: f64,
    mubar: f64,
    b: f64,
    c: f64,
    d: f64,
    f: f64,
) -> (f64, f64) {
    let steps = 4000_u32;
    let h = area / f64::from(steps);
    let mut up = up0;
    let mut down = down0;
    let deriv = |x: f64, u: f64, v: f64| {
        let beam = direct * (-kb * x).exp();
        (
            (b * u - c * v - d * beam) / mubar,
            (f * beam - b * v + c * u) / mubar,
        )
    };
    for step in 0..steps {
        let x = f64::from(step) * h;
        let k1 = deriv(x, up, down);
        let k2 = deriv(x + h / 2.0, up + h * k1.0 / 2.0, down + h * k1.1 / 2.0);
        let k3 = deriv(x + h / 2.0, up + h * k2.0 / 2.0, down + h * k2.1 / 2.0);
        let k4 = deriv(x + h, up + h * k3.0, down + h * k3.1);
        up += h * (k1.0 + 2.0 * k2.0 + 2.0 * k3.0 + k4.0) / 6.0;
        down += h * (k1.1 + 2.0 * k2.1 + 2.0 * k3.1 + k4.1) / 6.0;
    }
    (up, down)
}

fn adaptive_simpson<F: Fn(f64) -> f64>(
    function: F,
    a: f64,
    b: f64,
    tolerance: f64,
    depth: u32,
) -> Result<f64, VegetationError> {
    #[allow(clippy::too_many_arguments)]
    fn refine<F: Fn(f64) -> f64>(
        f: &F,
        a: f64,
        b: f64,
        fa: f64,
        fm: f64,
        fb: f64,
        whole: f64,
        tol: f64,
        depth: u32,
    ) -> Result<f64, VegetationError> {
        if depth == 0 {
            return Err(VegetationError::Domain("radiation quadrature depth"));
        }
        let center = f64::midpoint(a, b);
        let lm = f64::midpoint(a, center);
        let rm = f64::midpoint(center, b);
        let fl = f(lm);
        let fr = f(rm);
        let left = (center - a) * (fa + 4.0 * fl + fm) / 6.0;
        let right = (b - center) * (fm + 4.0 * fr + fb) / 6.0;
        let delta = left + right - whole;
        if delta.abs() <= 15.0 * tol {
            Ok(left + right + delta / 15.0)
        } else {
            Ok(
                refine(f, a, center, fa, fl, fm, left, tol / 2.0, depth - 1)?
                    + refine(f, center, b, fm, fr, fb, right, tol / 2.0, depth - 1)?,
            )
        }
    }
    let fa = function(a);
    let fb = function(b);
    let mid = f64::midpoint(a, b);
    let fm = function(mid);
    let whole = (b - a) * (fa + 4.0 * fm + fb) / 6.0;
    refine(&function, a, b, fa, fm, fb, whole, tolerance, depth)
}

pub fn beam_extinction(g_mu: f64, mu: f64, clumping: f64) -> Result<f64, VegetationError> {
    if !g_mu.is_finite()
        || !mu.is_finite()
        || !clumping.is_finite()
        || g_mu <= 0.0
        || mu <= 0.0
        || !(0.0..=1.0).contains(&clumping)
    {
        return Err(VegetationError::Domain("beam extinction"));
    }
    Ok(g_mu * clumping / mu)
}

pub fn sunlit_shaded(
    lai: f64,
    kb: f64,
    incident: RadiationBand,
    absorptivity: f64,
) -> Result<ClassRadiation, VegetationError> {
    if !lai.is_finite()
        || lai < 0.0
        || !kb.is_finite()
        || kb <= 0.0
        || !absorptivity.is_finite()
        || !(0.0..=1.0).contains(&absorptivity)
        || incident.direct < 0.0
        || incident.diffuse < 0.0
    {
        return Err(VegetationError::Domain("radiation"));
    }
    if lai == 0.0 {
        return Ok(ClassRadiation {
            sunlit_lai: 0.0,
            shaded_lai: 0.0,
            sunlit_absorbed: 0.0,
            shaded_absorbed: 0.0,
            transmitted: incident,
        });
    }
    let transmitted_direct = incident.direct * (-kb * lai).exp();
    let kd = 0.8_f64;
    let transmitted_diffuse = incident.diffuse * (-kd * lai).exp();
    let sunlit_lai = -(-kb * lai).exp_m1() / kb;
    let shaded_lai = lai - sunlit_lai;
    let absorbed_direct = absorptivity * (incident.direct - transmitted_direct);
    let absorbed_diffuse = absorptivity * (incident.diffuse - transmitted_diffuse);
    let diffuse_sun = absorbed_diffuse * sunlit_lai / lai;
    Ok(ClassRadiation {
        sunlit_lai,
        shaded_lai,
        sunlit_absorbed: absorbed_direct + diffuse_sun,
        shaded_absorbed: absorbed_diffuse - diffuse_sun,
        transmitted: RadiationBand {
            direct: transmitted_direct,
            diffuse: transmitted_diffuse,
        },
    })
}

pub fn traverse_column(
    lai_by_rank: &[f64],
    kb: f64,
    incident: RadiationBand,
    absorptivity: f64,
) -> Result<Vec<ClassRadiation>, VegetationError> {
    let mut flux = incident;
    let mut out = Vec::with_capacity(lai_by_rank.len());
    for &lai in lai_by_rank {
        let class = sunlit_shaded(lai, kb, flux, absorptivity)?;
        flux = class.transmitted;
        out.push(class);
    }
    Ok(out)
}
