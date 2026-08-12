#![allow(clippy::many_single_char_names)]
//! E01--E03 exact two-stream radiation and ordered canopy traversal.

use crate::VegetationError;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ColumnLayer {
    pub plant_area: f64,
    pub chi: f64,
    pub rho: f64,
    pub tau: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TwoStreamResult {
    pub absorbed: f64,
    pub reflected: f64,
    pub reflected_direct: f64,
    pub reflected_diffuse: f64,
    pub absorbed_direct: f64,
    pub absorbed_diffuse: f64,
    pub transmitted_direct: f64,
    pub transmitted_diffuse: f64,
    pub terminal_from_direct: f64,
    pub terminal_from_diffuse: f64,
    pub sunlit_lai: f64,
    pub shaded_lai: f64,
    pub sunlit_absorbed: f64,
    pub shaded_absorbed: f64,
    pub closure_residual: f64,
}

#[derive(Clone, Copy)]
struct Matrix2 {
    a11: f64,
    a12: f64,
    a21: f64,
    a22: f64,
}

impl Matrix2 {
    const IDENTITY: Self = Self {
        a11: 1.0,
        a12: 0.0,
        a21: 0.0,
        a22: 1.0,
    };
    fn scale(self, s: f64) -> Self {
        Self {
            a11: self.a11 * s,
            a12: self.a12 * s,
            a21: self.a21 * s,
            a22: self.a22 * s,
        }
    }
    fn add(self, o: Self) -> Self {
        Self {
            a11: self.a11 + o.a11,
            a12: self.a12 + o.a12,
            a21: self.a21 + o.a21,
            a22: self.a22 + o.a22,
        }
    }
    fn sub(self, o: Self) -> Self {
        self.add(o.scale(-1.0))
    }
    fn vector(self, v: [f64; 2]) -> [f64; 2] {
        [
            self.a11.mul_add(v[0], self.a12 * v[1]),
            self.a21.mul_add(v[0], self.a22 * v[1]),
        ]
    }
}

fn finite(values: &[f64]) -> bool {
    values.iter().all(|v| v.is_finite())
}

/// Exact real 2x2 matrix-exponential boundary solution required by E01.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
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
    solve_column(
        &[ColumnLayer {
            plant_area,
            chi,
            rho,
            tau,
        }],
        mu,
        ground_albedo,
        direct,
        diffuse,
    )?
    .into_iter()
    .next()
    .ok_or(VegetationError::Domain("empty two-stream result"))
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn solution(
    a: Matrix2,
    p: [f64; 2],
    k: f64,
    x: f64,
    y0: [f64; 2],
) -> Result<[f64; 2], VegetationError> {
    let homogeneous = exponential(a, x)?.vector(y0);
    let particular = integral_shifted(a, k, x)?.vector(p);
    let decay = (-k * x).exp();
    Ok([
        homogeneous[0] + decay * particular[0],
        homogeneous[1] + decay * particular[1],
    ])
}

fn exponential(a: Matrix2, x: f64) -> Result<Matrix2, VegetationError> {
    let gamma2 = a.a11 * a.a11 + a.a12 * a.a21;
    if gamma2 < -64.0 * f64::EPSILON {
        return Err(VegetationError::Domain("complex two-stream eigenvalue"));
    }
    if gamma2.abs() <= 64.0 * f64::EPSILON {
        return Ok(Matrix2::IDENTITY.add(a.scale(x)));
    }
    let gamma = gamma2.sqrt();
    let gx = gamma * x;
    Ok(Matrix2::IDENTITY
        .scale(gx.cosh())
        .add(a.scale(gx.sinh() / gamma)))
}

/// `integral_0^x exp((a + shift I) u) du`, including exact resonance.
fn integral_shifted(a: Matrix2, shift: f64, x: f64) -> Result<Matrix2, VegetationError> {
    let gamma2 = a.a11 * a.a11 + a.a12 * a.a21;
    if gamma2 < -64.0 * f64::EPSILON {
        return Err(VegetationError::Domain("complex two-stream eigenvalue"));
    }
    if gamma2.abs() <= 64.0 * f64::EPSILON {
        let f0 = exp_integral(shift, x);
        let f1 = exp_first_moment(shift, x);
        return Ok(Matrix2::IDENTITY.scale(f0).add(a.scale(f1)));
    }
    let gamma = gamma2.sqrt();
    let plus = Matrix2::IDENTITY.add(a.scale(1.0 / gamma)).scale(0.5);
    let minus = Matrix2::IDENTITY.sub(a.scale(1.0 / gamma)).scale(0.5);
    Ok(plus
        .scale(exp_integral(shift + gamma, x))
        .add(minus.scale(exp_integral(shift - gamma, x))))
}

fn exp_integral(rate: f64, x: f64) -> f64 {
    if rate == 0.0 {
        x
    } else {
        (rate * x).exp_m1() / rate
    }
}
fn exp_first_moment(rate: f64, x: f64) -> f64 {
    if rate == 0.0 {
        x * x / 2.0
    } else {
        ((rate * x - 1.0) * (rate * x).exp() + 1.0) / (rate * rate)
    }
}

fn sunlit_absorption(
    a: Matrix2,
    p: [f64; 2],
    k: f64,
    x: f64,
    y0: [f64; 2],
    direct: f64,
    omega: f64,
) -> Result<f64, VegetationError> {
    let hminus = integral_shifted(a, -k, x)?;
    let jplus = integral_shifted(a, k, x)?;
    let double = hminus
        .sub(jplus.scale((-2.0 * k * x).exp()))
        .scale(1.0 / (2.0 * k));
    let term1 = a.vector(hminus.vector(y0));
    let term2 = a.vector(double.vector(p));
    let row_term = (term1[0] - term1[1]) + (term2[0] - term2[1]);
    let beam_integral = exp_integral(-2.0 * k, x);
    let local_weighted = row_term + (p[0] - p[1] + k * direct) * beam_integral;
    let direct_weighted = (1.0 - omega) * k * direct * beam_integral;
    let direct_total = (1.0 - omega) * direct * (-k * x).exp_m1().abs();
    Ok(local_weighted - direct_weighted + direct_total)
}

/// Piecewise exact column solve. One bottom boundary condition determines the
/// upward stream through every overlying stratum; no internal layer is treated
/// as an independent ground boundary.
pub(crate) fn solve_column(
    layers: &[ColumnLayer],
    mu: f64,
    ground_albedo: f64,
    direct: f64,
    diffuse: f64,
) -> Result<Vec<TwoStreamResult>, VegetationError> {
    let systems = column_systems(layers, mu, direct)?;
    let total = solve_column_component(&systems, ground_albedo, diffuse)?;
    let direct_only = solve_column_component(&systems, ground_albedo, 0.0)?;
    let diffuse_systems = systems
        .iter()
        .map(|system| LayerSystem {
            p: [0.0, 0.0],
            direct_top: 0.0,
            ..*system
        })
        .collect::<Vec<_>>();
    let diffuse_only = solve_column_component(&diffuse_systems, ground_albedo, diffuse)?;
    Ok(total
        .into_iter()
        .zip(direct_only)
        .zip(diffuse_only)
        .map(|((mut value, direct_value), diffuse_value)| {
            value.reflected_direct = direct_value.reflected;
            value.reflected_diffuse = diffuse_value.reflected;
            value.absorbed_direct = direct_value.absorbed;
            value.absorbed_diffuse = diffuse_value.absorbed;
            let is_terminal_layer =
                value.terminal_from_direct != 0.0 || value.terminal_from_diffuse != 0.0;
            value.terminal_from_direct = if is_terminal_layer {
                direct_value.transmitted_direct + direct_value.transmitted_diffuse
            } else {
                0.0
            };
            value.terminal_from_diffuse = if is_terminal_layer {
                diffuse_value.transmitted_direct + diffuse_value.transmitted_diffuse
            } else {
                0.0
            };
            value
        })
        .collect())
}

#[derive(Clone, Copy)]
struct LayerSystem {
    a: Matrix2,
    p: [f64; 2],
    k: f64,
    area: f64,
    direct_top: f64,
    omega: f64,
}
type LayerBoundaryStates = Vec<([f64; 2], [f64; 2])>;

#[allow(clippy::too_many_lines)]
fn layer_system(
    layer: ColumnLayer,
    mu: f64,
    direct_top: f64,
) -> Result<LayerSystem, VegetationError> {
    if !finite(&[
        layer.plant_area,
        mu,
        layer.chi,
        layer.rho,
        layer.tau,
        direct_top,
    ]) || layer.plant_area < 0.0
        || !(-0.4..=0.6).contains(&layer.chi)
        || layer.rho < 0.0
        || layer.tau < 0.0
        || layer.rho + layer.tau >= 1.0
        || direct_top < 0.0
        || (direct_top > 0.0 && mu <= 0.0)
    {
        return Err(VegetationError::Domain("two-stream column layer"));
    }
    let phi1 = 0.5 - 0.633 * layer.chi - 0.33 * layer.chi * layer.chi;
    let phi2 = 0.877 * (1.0 - 2.0 * phi1);
    let gmu = if direct_top > 0.0 {
        phi1 + phi2 * mu
    } else {
        0.0
    };
    let k = if direct_top > 0.0 { gmu / mu } else { 0.0 };
    let mubar = adaptive_simpson(|mup| mup / (phi1 + phi2 * mup), 0.0, 1.0, 1e-14, 20)?;
    let omega = layer.rho + layer.tau;
    let cosbar = f64::midpoint(1.0, layer.chi);
    let omega_beta = if omega == 0.0 {
        0.0
    } else {
        0.5 * (layer.rho + layer.tau + (layer.rho - layer.tau) * cosbar * cosbar)
    };
    let beta = if omega == 0.0 {
        0.0
    } else {
        omega_beta / omega
    };
    let ascat = if omega == 0.0 || direct_top == 0.0 {
        0.0
    } else {
        0.5 * omega
            * adaptive_simpson(
                |mup| {
                    let gp = phi1 + phi2 * mup;
                    let den = mu * gp + mup * gmu;
                    if den == 0.0 { 0.0 } else { mup * gmu / den }
                },
                0.0,
                1.0,
                1e-14,
                20,
            )?
    };
    let omega_beta0 = if omega == 0.0 || direct_top == 0.0 {
        0.0
    } else {
        (1.0 + mubar * k) * ascat / (mubar * k)
    };
    let beta0 = if omega == 0.0 {
        0.0
    } else {
        omega_beta0 / omega
    };
    let b = 1.0 - (1.0 - beta) * omega;
    let c = omega * beta;
    let d = omega * mubar * k * beta0;
    let f = omega * mubar * k * (1.0 - beta0);
    Ok(LayerSystem {
        a: Matrix2 {
            a11: b / mubar,
            a12: -c / mubar,
            a21: c / mubar,
            a22: -b / mubar,
        },
        p: [-d * direct_top / mubar, f * direct_top / mubar],
        k,
        area: layer.plant_area,
        direct_top,
        omega,
    })
}

fn propagate_column(
    systems: &[LayerSystem],
    top_up: f64,
    top_down: f64,
) -> Result<LayerBoundaryStates, VegetationError> {
    let mut state = [top_up, top_down];
    let mut states = Vec::with_capacity(systems.len());
    for system in systems {
        let terminal = solution(system.a, system.p, system.k, system.area, state)?;
        states.push((state, terminal));
        state = terminal;
    }
    Ok(states)
}

fn column_systems(
    layers: &[ColumnLayer],
    mu: f64,
    direct: f64,
) -> Result<Vec<LayerSystem>, VegetationError> {
    if layers.is_empty() || !finite(&[mu, direct]) || direct < 0.0 || (direct > 0.0 && mu <= 0.0) {
        return Err(VegetationError::Domain("two-stream column"));
    }
    let mut beam = direct;
    let mut systems = Vec::with_capacity(layers.len());
    for layer in layers {
        let system = layer_system(*layer, mu, beam)?;
        beam *= (-system.k * system.area).exp();
        systems.push(system);
    }
    Ok(systems)
}

fn solve_column_component(
    systems: &[LayerSystem],
    ground_albedo: f64,
    diffuse: f64,
) -> Result<Vec<TwoStreamResult>, VegetationError> {
    if systems.is_empty()
        || !finite(&[ground_albedo, diffuse])
        || !(0.0..=1.0).contains(&ground_albedo)
        || diffuse < 0.0
    {
        return Err(VegetationError::Domain("two-stream column component"));
    }
    let beam = systems
        .last()
        .map(|system| system.direct_top * (-system.k * system.area).exp())
        .ok_or(VegetationError::Domain("empty column"))?;
    let base = propagate_column(systems, 0.0, diffuse)?;
    let unit = propagate_column(systems, 1.0, diffuse)?;
    let base_bottom = base
        .last()
        .ok_or(VegetationError::Domain("empty column"))?
        .1;
    let unit_bottom = unit
        .last()
        .ok_or(VegetationError::Domain("empty column"))?
        .1;
    let slope = [
        unit_bottom[0] - base_bottom[0],
        unit_bottom[1] - base_bottom[1],
    ];
    let denominator = slope[0] - ground_albedo * slope[1];
    if !denominator.is_finite() || denominator.abs() <= 64.0 * f64::EPSILON {
        return Err(VegetationError::Domain(
            "two-stream column boundary singular",
        ));
    }
    let top_up = (ground_albedo * (base_bottom[1] + beam) - base_bottom[0]) / denominator;
    let states = propagate_column(systems, top_up, diffuse)?;
    let last = systems.len() - 1;
    systems
        .iter()
        .zip(states)
        .enumerate()
        .map(|(index, (system, (top, bottom)))| {
            let beam_bottom = system.direct_top * (-system.k * system.area).exp();
            let absorbed =
                system.direct_top + top[1] + bottom[0] - beam_bottom - bottom[1] - top[0];
            let sunlit_lai = if system.direct_top == 0.0 {
                0.0
            } else {
                -(-system.k * system.area).exp_m1() / system.k
            };
            let sunlit_absorbed = if system.direct_top == 0.0 {
                0.0
            } else {
                sunlit_absorption(
                    system.a,
                    system.p,
                    system.k,
                    system.area,
                    top,
                    system.direct_top,
                    system.omega,
                )?
            };
            Ok(TwoStreamResult {
                absorbed,
                reflected: if index == 0 { top[0] } else { 0.0 },
                reflected_direct: 0.0,
                reflected_diffuse: 0.0,
                absorbed_direct: 0.0,
                absorbed_diffuse: 0.0,
                transmitted_direct: beam_bottom,
                transmitted_diffuse: bottom[1],
                terminal_from_direct: if index == last {
                    beam_bottom + bottom[1]
                } else {
                    0.0
                },
                terminal_from_diffuse: 0.0,
                sunlit_lai,
                shaded_lai: system.area - sunlit_lai,
                sunlit_absorbed,
                shaded_absorbed: absorbed - sunlit_absorbed,
                closure_residual: 0.0,
            })
        })
        .collect()
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
            return Err(VegetationError::Radiation("quadrature depth limit"));
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
    if !finite(&[fa, fb, fm, whole]) {
        return Err(VegetationError::Radiation("nonfinite quadrature operand"));
    }
    refine(&function, a, b, fa, fm, fb, whole, tolerance, depth)
}
