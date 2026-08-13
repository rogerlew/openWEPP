#![allow(clippy::many_single_char_names)]
//! E01--E03 exact two-stream radiation and ordered canopy traversal.

use crate::VegetationError;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ColumnLayer {
    pub plant_area: f64,
    pub chi: f64,
    pub rho: f64,
    pub tau: f64,
    pub clumping_index: f64,
}

/// Spectral identity retained through radiation preparation and ownership.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RadiationBand {
    Visible,
    NearInfrared,
}

/// Incident-stream identity retained through radiation preparation and ownership.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IncidentComponent {
    Direct,
    Diffuse,
}

/// Reflectance and transmittance of one physical plant surface in one band.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfaceOptics {
    pub reflectance: f64,
    pub transmittance: f64,
}

/// V3 occupancy-local plant-area and optical operands for one spectral band.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MixedLayer {
    pub leaf_area: f64,
    pub stem_area: f64,
    pub clumping_index: f64,
    pub leaf_angle_chi: f64,
    pub leaf_optics: SurfaceOptics,
    pub stem_optics: SurfaceOptics,
}

/// Prepared and physically owned radiation for one occupancy.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OwnedLayerAbsorption {
    pub band: RadiationBand,
    pub component: IncidentComponent,
    pub plant_area: f64,
    pub effective_reflectance: f64,
    pub effective_transmittance: f64,
    pub beam_extinction_unclumped: Option<f64>,
    pub beam_extinction_effective: Option<f64>,
    pub leaf_absorption_fraction: f64,
    pub stem_absorption_fraction: f64,
    pub absorbed_plant: f64,
    pub absorbed_leaf_sun: f64,
    pub absorbed_leaf_shade: f64,
    pub absorbed_stem: f64,
    pub leaf_sun_area: f64,
    pub leaf_shade_area: f64,
    pub owner_closure_residual: f64,
}

/// Whole-column result for one independently solved band/component identity.
#[derive(Clone, Debug, PartialEq)]
pub struct ColumnRadiationResult {
    pub band: RadiationBand,
    pub component: IncidentComponent,
    pub incident: f64,
    pub top_reflected: f64,
    pub terminal_direct: f64,
    pub terminal_diffuse: f64,
    pub ground_absorbed: f64,
    pub layers: Vec<OwnedLayerAbsorption>,
    pub transport_closure_residual: f64,
    pub owner_closure_residual: f64,
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
            clumping_index: 1.0,
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

#[derive(Clone, Copy)]
struct PreparedMixedLayer {
    transport: ColumnLayer,
    leaf_area: f64,
    leaf_weight: f64,
    leaf_absorption_fraction: f64,
    stem_absorption_fraction: f64,
    zero_absorptivity: bool,
}

fn prepare_mixed_layer(layer: MixedLayer) -> Result<PreparedMixedLayer, VegetationError> {
    let leaf = layer.leaf_optics;
    let stem = layer.stem_optics;
    if !finite(&[
        layer.leaf_area,
        layer.stem_area,
        layer.clumping_index,
        layer.leaf_angle_chi,
        leaf.reflectance,
        leaf.transmittance,
        stem.reflectance,
        stem.transmittance,
    ]) || layer.leaf_area < 0.0
        || layer.stem_area < 0.0
        || !(0.0..=1.0).contains(&leaf.reflectance)
        || !(0.0..=1.0).contains(&leaf.transmittance)
        || leaf.reflectance + leaf.transmittance > 1.0
        || !(0.0..=1.0).contains(&stem.reflectance)
        || !(0.0..=1.0).contains(&stem.transmittance)
        || stem.reflectance + stem.transmittance > 1.0
        || !(0.0 < layer.clumping_index && layer.clumping_index <= 1.0)
        || !(-0.4..=0.6).contains(&layer.leaf_angle_chi)
    {
        return Err(VegetationError::Domain("V3 mixed radiation layer"));
    }
    let plant_area = layer.leaf_area + layer.stem_area;
    if !plant_area.is_finite() {
        return Err(VegetationError::Domain("V3 mixed plant area"));
    }
    let (leaf_weight, stem_weight) = if plant_area == 0.0 {
        (0.0, 0.0)
    } else {
        (layer.leaf_area / plant_area, layer.stem_area / plant_area)
    };
    let effective_reflectance =
        leaf_weight.mul_add(leaf.reflectance, stem_weight * stem.reflectance);
    let effective_transmittance =
        leaf_weight.mul_add(leaf.transmittance, stem_weight * stem.transmittance);
    let leaf_absorptivity = leaf_weight * (1.0 - leaf.reflectance - leaf.transmittance);
    let stem_absorptivity = stem_weight * (1.0 - stem.reflectance - stem.transmittance);
    let total_absorptivity = leaf_absorptivity + stem_absorptivity;
    let (leaf_absorption_fraction, stem_absorption_fraction) = if total_absorptivity == 0.0 {
        (0.0, 0.0)
    } else {
        (
            leaf_absorptivity / total_absorptivity,
            stem_absorptivity / total_absorptivity,
        )
    };
    Ok(PreparedMixedLayer {
        transport: ColumnLayer {
            plant_area,
            chi: layer.leaf_angle_chi,
            rho: effective_reflectance,
            tau: effective_transmittance,
            clumping_index: layer.clumping_index,
        },
        leaf_area: layer.leaf_area,
        leaf_weight,
        leaf_absorption_fraction,
        stem_absorption_fraction,
        zero_absorptivity: total_absorptivity == 0.0,
    })
}

fn beam_extinction(chi: f64, mu: f64) -> f64 {
    let phi1 = 0.5 - 0.633 * chi - 0.33 * chi * chi;
    let phi2 = 0.877 * (1.0 - 2.0 * phi1);
    (phi1 + phi2 * mu) / mu
}

/// Solves one V3 band/component over the complete ordered canopy column.
///
/// `incident` is exclusively the component named by `component`; direct and
/// diffuse identities therefore cannot alias inside the returned owner terms.
#[allow(clippy::too_many_arguments)]
pub fn solve_mixed_column(
    layers: &[MixedLayer],
    band: RadiationBand,
    component: IncidentComponent,
    mu: f64,
    ground_albedo: f64,
    incident: f64,
) -> Result<ColumnRadiationResult, VegetationError> {
    if !finite(&[mu, ground_albedo, incident])
        || !(0.0..=1.0).contains(&ground_albedo)
        || incident < 0.0
        || (component == IncidentComponent::Direct && incident > 0.0 && mu <= 0.0)
    {
        return Err(VegetationError::Domain("V3 radiation forcing"));
    }
    let prepared = layers
        .iter()
        .copied()
        .map(prepare_mixed_layer)
        .collect::<Result<Vec<_>, _>>()?;
    if prepared.is_empty() {
        return Ok(empty_column_result(
            band,
            component,
            ground_albedo,
            incident,
        ));
    }
    let transport_layers = prepared
        .iter()
        .map(|layer| layer.transport)
        .collect::<Vec<_>>();
    let (direct, diffuse) = match component {
        IncidentComponent::Direct => (incident, 0.0),
        IncidentComponent::Diffuse => (0.0, incident),
    };
    let solved = solve_column(&transport_layers, mu, ground_albedo, direct, diffuse)?;
    own_mixed_absorption(
        &prepared,
        &solved,
        band,
        component,
        mu,
        ground_albedo,
        incident,
    )
}

fn empty_column_result(
    band: RadiationBand,
    component: IncidentComponent,
    ground_albedo: f64,
    incident: f64,
) -> ColumnRadiationResult {
    let top_reflected = ground_albedo * incident;
    let ground_absorbed = (1.0 - ground_albedo) * incident;
    ColumnRadiationResult {
        band,
        component,
        incident,
        top_reflected,
        terminal_direct: if component == IncidentComponent::Direct {
            incident
        } else {
            0.0
        },
        terminal_diffuse: if component == IncidentComponent::Diffuse {
            incident
        } else {
            0.0
        },
        ground_absorbed,
        layers: Vec::new(),
        transport_closure_residual: incident - top_reflected - ground_absorbed,
        owner_closure_residual: incident - top_reflected - ground_absorbed,
    }
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn own_mixed_absorption(
    prepared: &[PreparedMixedLayer],
    solved: &[TwoStreamResult],
    band: RadiationBand,
    component: IncidentComponent,
    mu: f64,
    ground_albedo: f64,
    incident: f64,
) -> Result<ColumnRadiationResult, VegetationError> {
    let first = solved
        .first()
        .ok_or(VegetationError::Domain("empty V3 solved column"))?;
    let last = solved
        .last()
        .ok_or(VegetationError::Domain("empty V3 solved column"))?;
    let top_reflected = first.reflected;
    let terminal_direct = last.transmitted_direct;
    let terminal_diffuse = last.transmitted_diffuse;
    let ground_absorbed = (1.0 - ground_albedo) * (terminal_direct + terminal_diffuse);
    let raw_absorbed = solved.iter().map(|value| value.absorbed).sum::<f64>();
    let transport_closure_residual = incident - top_reflected - ground_absorbed - raw_absorbed;
    let directional = component == IncidentComponent::Direct && incident > 0.0;
    let owned = prepared
        .iter()
        .zip(solved)
        .map(|(layer, value)| {
            let k = directional.then(|| beam_extinction(layer.transport.chi, mu));
            let k_eff = k.map(|value| value * layer.transport.clumping_index);
            let leaf_sun_area = if !directional || layer.leaf_area == 0.0 {
                0.0
            } else if let Some(extinction) = k_eff {
                if extinction == 0.0 {
                    layer.leaf_area
                } else {
                    layer.leaf_weight * -(-extinction * layer.transport.plant_area).exp_m1()
                        / extinction
                }
            } else {
                return Err(VegetationError::Domain("missing V3 direct extinction"));
            };
            let (absorbed_plant, plant_sun, plant_shade) = if layer.zero_absorptivity {
                (0.0, 0.0, 0.0)
            } else {
                (value.absorbed, value.sunlit_absorbed, value.shaded_absorbed)
            };
            let absorbed_leaf_sun = layer.leaf_absorption_fraction * plant_sun;
            let absorbed_leaf_shade = layer.leaf_absorption_fraction * plant_shade;
            let absorbed_stem = layer.stem_absorption_fraction * absorbed_plant;
            let owner_sum = absorbed_leaf_sun + absorbed_leaf_shade + absorbed_stem;
            Ok(OwnedLayerAbsorption {
                band,
                component,
                plant_area: layer.transport.plant_area,
                effective_reflectance: layer.transport.rho,
                effective_transmittance: layer.transport.tau,
                beam_extinction_unclumped: k,
                beam_extinction_effective: k_eff,
                leaf_absorption_fraction: layer.leaf_absorption_fraction,
                stem_absorption_fraction: layer.stem_absorption_fraction,
                absorbed_plant,
                absorbed_leaf_sun,
                absorbed_leaf_shade,
                absorbed_stem,
                leaf_sun_area,
                leaf_shade_area: layer.leaf_area - leaf_sun_area,
                owner_closure_residual: absorbed_plant - owner_sum,
            })
        })
        .collect::<Result<Vec<_>, VegetationError>>()?;
    let owner_absorbed = owned
        .iter()
        .map(|layer| layer.absorbed_leaf_sun + layer.absorbed_leaf_shade + layer.absorbed_stem)
        .sum::<f64>();
    Ok(ColumnRadiationResult {
        band,
        component,
        incident,
        top_reflected,
        terminal_direct,
        terminal_diffuse,
        ground_absorbed,
        layers: owned,
        transport_closure_residual,
        owner_closure_residual: incident - top_reflected - ground_absorbed - owner_absorbed,
    })
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
) -> Result<f64, VegetationError> {
    adaptive_simpson(
        |distance| {
            let attenuation = (-k * distance).exp();
            let Ok(state) = solution(a, p, k, distance, y0) else {
                return f64::NAN;
            };
            let mut derivative = a.vector(state);
            derivative[0] += p[0] * attenuation;
            derivative[1] += p[1] * attenuation;
            let local_absorption = k.mul_add(direct * attenuation, derivative[0] - derivative[1]);
            local_absorption * attenuation
        },
        0.0,
        x,
        1.0e-13,
        20,
    )
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
        layer.clumping_index,
        direct_top,
    ]) || layer.plant_area < 0.0
        || !(-0.4..=0.6).contains(&layer.chi)
        || layer.rho < 0.0
        || layer.tau < 0.0
        || layer.rho + layer.tau > 1.0
        || !(0.0 < layer.clumping_index && layer.clumping_index <= 1.0)
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
    let k = if direct_top > 0.0 {
        layer.clumping_index * gmu / mu
    } else {
        0.0
    };
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

#[cfg(test)]
mod tests {
    use super::*;

    const TOLERANCE: f64 = 2.0e-9;

    fn optics(reflectance: f64, transmittance: f64) -> SurfaceOptics {
        SurfaceOptics {
            reflectance,
            transmittance,
        }
    }

    fn fixture_layers(band: RadiationBand) -> [MixedLayer; 2] {
        let (upper_leaf, upper_stem, lower_leaf, lower_stem) = match band {
            RadiationBand::Visible => (
                optics(0.09, 0.06),
                optics(0.18, 0.03),
                optics(0.12, 0.04),
                optics(0.22, 0.02),
            ),
            RadiationBand::NearInfrared => (
                optics(0.41, 0.31),
                optics(0.29, 0.12),
                optics(0.37, 0.27),
                optics(0.25, 0.10),
            ),
        };
        [
            MixedLayer {
                leaf_area: 2.6,
                stem_area: 0.7,
                clumping_index: 0.74,
                leaf_angle_chi: 0.12,
                leaf_optics: upper_leaf,
                stem_optics: upper_stem,
            },
            MixedLayer {
                leaf_area: 1.35,
                stem_area: 0.45,
                clumping_index: 0.86,
                leaf_angle_chi: -0.08,
                leaf_optics: lower_leaf,
                stem_optics: lower_stem,
            },
        ]
    }

    fn close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() <= TOLERANCE,
            "actual={actual:.16e}, expected={expected:.16e}"
        );
    }

    struct FixtureExpected {
        reflected: f64,
        terminal_direct: f64,
        terminal_diffuse: f64,
        ground: f64,
        plant: [f64; 2],
        leaf_sun: [f64; 2],
        leaf_shade: [f64; 2],
        stem: [f64; 2],
    }

    fn assert_fixture(result: &ColumnRadiationResult, expected: &FixtureExpected) {
        close(result.top_reflected, expected.reflected);
        close(result.terminal_direct, expected.terminal_direct);
        close(result.terminal_diffuse, expected.terminal_diffuse);
        close(result.ground_absorbed, expected.ground);
        close(result.transport_closure_residual, 0.0);
        close(result.owner_closure_residual, 0.0);
        for (index, layer) in result.layers.iter().enumerate() {
            close(layer.absorbed_plant, expected.plant[index]);
            close(layer.absorbed_leaf_sun, expected.leaf_sun[index]);
            close(layer.absorbed_leaf_shade, expected.leaf_shade[index]);
            close(layer.absorbed_stem, expected.stem[index]);
            close(layer.owner_closure_residual, 0.0);
        }
    }

    #[test]
    fn v3_two_rank_visible_direct_fixture_and_named_poisons() {
        let result = solve_mixed_column(
            &fixture_layers(RadiationBand::Visible),
            RadiationBand::Visible,
            IncidentComponent::Direct,
            0.67,
            0.14,
            410.0,
        )
        .expect("released V3 VIS direct fixture must solve");
        assert_fixture(
            &result,
            &FixtureExpected {
                reflected: 14.446_633_837_785_413,
                terminal_direct: 20.219_605_260_147_713,
                terminal_diffuse: 2.955_414_289_197_655_4,
                ground: 19.930_516_812_437_016,
                plant: [330.459_444_550_164_5, 45.163_404_799_612_39],
                leaf_sun: [149.754_259_505_496_6, 22.589_470_700_647_695],
                leaf_shade: [114.565_455_462_242_67, 12.109_242_742_956_94],
                stem: [66.139_729_582_425_26, 10.464_691_356_007_748],
            },
        );
        let upper = result.layers[0];
        close(upper.effective_reflectance, 0.109_090_909_090_909_1);
        close(
            upper.beam_extinction_effective.expect("direct K_eff"),
            0.567_855_156_475_223_8,
        );
        close(upper.leaf_absorption_fraction, 0.799_855_229_822_656_6);
        close(upper.leaf_sun_area, 1.174_460_312_891_180_8);
        assert!((upper.effective_reflectance - 0.135).abs() > 1.0e-3); // arithmetic mean
        assert!((upper.leaf_absorption_fraction - 2.6 / 3.3).abs() > 1.0e-3); // area only
        assert!((upper.leaf_sun_area - 1.490_661_166_361_883_2).abs() > 1.0e-3); // plant area
        assert!((result.top_reflected - 14.305_974_140_609_25).abs() > 1.0e-3); // lower boundary
    }

    #[test]
    fn v3_two_rank_nir_direct_fixture_keeps_band_identity() {
        let result = solve_mixed_column(
            &fixture_layers(RadiationBand::NearInfrared),
            RadiationBand::NearInfrared,
            IncidentComponent::Direct,
            0.67,
            0.31,
            355.0,
        )
        .expect("released V3 NIR direct fixture must solve");
        assert_fixture(
            &result,
            &FixtureExpected {
                reflected: 74.792_007_404_953_13,
                terminal_direct: 17.507_219_188_664_482,
                terminal_diffuse: 20.157_178_885_213_963,
                ground: 25.988_434_670_976_126,
                plant: [205.193_147_741_450_3, 49.026_410_182_626_63],
                leaf_sun: [68.692_879_720_745_17, 19.631_974_847_830_993],
                leaf_shade: [62.227_901_660_302_9, 10.974_107_809_531_295],
                stem: [74.272_366_360_402_24, 18.420_327_525_264_34],
            },
        );
        assert_eq!(result.band, RadiationBand::NearInfrared);
        assert_eq!(result.component, IncidentComponent::Direct);
        assert!((result.layers[0].absorbed_plant - 330.459_444_550_164_5).abs() > 1.0);
    }

    #[test]
    fn v3_diffuse_fixtures_have_exact_zero_directional_operands() {
        let cases = [
            (
                RadiationBand::Visible,
                0.14,
                83.0,
                FixtureExpected {
                    reflected: 4.041_500_144_031_665,
                    terminal_direct: 0.0,
                    terminal_diffuse: 0.733_120_981_170_069_3,
                    ground: 0.630_484_043_806_259_6,
                    plant: [75.490_134_462_696_96, 2.837_881_349_464_999],
                    leaf_sun: [0.0, 0.0],
                    leaf_shade: [60.381_178_850_003_72, 2.180_323_475_808_474_4],
                    stem: [15.108_955_612_693_238, 0.657_557_873_656_524_1],
                },
            ),
            (
                RadiationBand::NearInfrared,
                0.31,
                101.0,
                FixtureExpected {
                    reflected: 26.953_746_662_637_137,
                    terminal_direct: 0.0,
                    terminal_diffuse: 4.160_937_262_877_129,
                    ground: 2.871_046_711_385_218_5,
                    plant: [63.753_250_381_796_99, 7.421_956_244_179_347],
                    leaf_sun: [0.0, 0.0],
                    leaf_shade: [40.676_920_488_999_31, 4.633_359_967_464_563],
                    stem: [23.076_329_892_797_68, 2.788_596_276_714_783_6],
                },
            ),
        ];
        for (band, albedo, incident, expected) in cases {
            let result = solve_mixed_column(
                &fixture_layers(band),
                band,
                IncidentComponent::Diffuse,
                0.67,
                albedo,
                incident,
            )
            .expect("released V3 diffuse fixture must solve");
            assert_fixture(&result, &expected);
            for layer in result.layers {
                assert_eq!(layer.beam_extinction_unclumped, None);
                assert_eq!(layer.beam_extinction_effective, None);
                assert_eq!(layer.leaf_sun_area.to_bits(), 0.0_f64.to_bits());
            }
        }
    }

    #[test]
    fn v3_owner_reductions_and_zero_branches_are_exact() {
        let base = fixture_layers(RadiationBand::Visible)[0];
        let leaf_only = MixedLayer {
            leaf_area: 2.1,
            stem_area: 0.0,
            clumping_index: 0.8,
            leaf_angle_chi: 0.1,
            ..base
        };
        let stem_only = MixedLayer {
            leaf_area: 0.0,
            stem_area: 0.8,
            clumping_index: 0.8,
            leaf_angle_chi: 0.1,
            ..base
        };
        let leaf = solve_mixed_column(
            &[leaf_only],
            RadiationBand::Visible,
            IncidentComponent::Direct,
            0.67,
            0.14,
            300.0,
        )
        .expect("leaf-only reduction");
        let stem = solve_mixed_column(
            &[stem_only],
            RadiationBand::Visible,
            IncidentComponent::Direct,
            0.67,
            0.14,
            300.0,
        )
        .expect("stem-only reduction");
        assert_eq!(leaf.layers[0].absorbed_stem.to_bits(), 0.0_f64.to_bits());
        assert_eq!(
            stem.layers[0].absorbed_leaf_sun.to_bits(),
            0.0_f64.to_bits()
        );
        assert_eq!(
            stem.layers[0].absorbed_leaf_shade.to_bits(),
            0.0_f64.to_bits()
        );
        assert_eq!(stem.layers[0].leaf_sun_area.to_bits(), 0.0_f64.to_bits());

        let empty = solve_mixed_column(
            &[],
            RadiationBand::Visible,
            IncidentComponent::Direct,
            0.67,
            0.18,
            100.0,
        )
        .expect("zero-plant column");
        assert!(empty.layers.is_empty());
        close(empty.terminal_direct, 100.0);
        close(empty.top_reflected, 18.0);
        close(empty.ground_absorbed, 82.0);
    }

    #[test]
    fn v3_zero_absorptivity_and_invalid_optics_fail_closed() {
        let zero_absorptivity = MixedLayer {
            leaf_area: 1.4,
            stem_area: 0.6,
            clumping_index: 0.8,
            leaf_angle_chi: 0.1,
            leaf_optics: optics(0.51, 0.49),
            stem_optics: optics(0.51, 0.49),
        };
        let result = solve_mixed_column(
            &[zero_absorptivity],
            RadiationBand::Visible,
            IncidentComponent::Direct,
            0.67,
            0.14,
            300.0,
        )
        .expect("zero-absorptivity exact branch");
        assert_eq!(result.layers[0].absorbed_plant.to_bits(), 0.0_f64.to_bits());
        assert_eq!(
            result.layers[0].absorbed_leaf_sun.to_bits(),
            0.0_f64.to_bits()
        );
        assert_eq!(
            result.layers[0].absorbed_leaf_shade.to_bits(),
            0.0_f64.to_bits()
        );
        assert_eq!(result.layers[0].absorbed_stem.to_bits(), 0.0_f64.to_bits());

        let invalid = MixedLayer {
            clumping_index: 0.0,
            ..zero_absorptivity
        };
        assert_eq!(
            solve_mixed_column(
                &[invalid],
                RadiationBand::Visible,
                IncidentComponent::Direct,
                0.67,
                0.14,
                300.0,
            ),
            Err(VegetationError::Domain("V3 mixed radiation layer"))
        );
    }
}
