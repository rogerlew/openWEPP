//! E14--E15 four-potential plant hydraulics helpers.
use crate::VegetationError;

pub fn vulnerability(psi: f64, p50: f64, shape: f64) -> Result<f64, VegetationError> {
    if !psi.is_finite() || !p50.is_finite() || p50 >= 0.0 || !shape.is_finite() || shape <= 0.0 {
        return Err(VegetationError::Domain("vulnerability"));
    }
    Ok(2.0_f64.powf(-(psi / p50).abs().powf(shape)))
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SoilHydraulicLayer {
    pub psi_mm: f64,
    pub root_fraction: f64,
    pub root_conductance: f64,
    pub soil_conductance: f64,
    pub accessible: bool,
    pub frozen: bool,
}
pub fn layer_fluxes(
    psi_root: f64,
    lai: f64,
    sai: f64,
    root_to_leaf: f64,
    layers: &[SoilHydraulicLayer],
) -> Result<Vec<f64>, VegetationError> {
    let mut out = Vec::with_capacity(layers.len());
    for l in layers {
        if !l.accessible || l.frozen || l.root_conductance == 0.0 || l.soil_conductance == 0.0 {
            out.push(0.0);
            continue;
        }
        if l.root_fraction < 0.0 || l.root_conductance < 0.0 || l.soil_conductance < 0.0 {
            return Err(VegetationError::Domain("hydraulic layer"));
        }
        let k = l.root_conductance * l.soil_conductance / (l.root_conductance + l.soil_conductance);
        let rai = (lai + sai) * l.root_fraction * root_to_leaf;
        let q = k * rai * (l.psi_mm - psi_root);
        if q < 0.0 {
            return Err(VegetationError::Hydraulic(
                "hydraulic redistribution unsupported",
            ));
        }
        out.push(q);
    }
    Ok(out)
}
pub fn finalize_under_caps(
    requests: &[f64],
    authorizations: &[f64],
) -> Result<Vec<f64>, VegetationError> {
    if requests.len() != authorizations.len() {
        return Err(VegetationError::Hydraulic("authorization shape"));
    }
    requests
        .iter()
        .zip(authorizations)
        .map(|(&d, &a)| {
            if !d.is_finite() || !a.is_finite() || d < 0.0 || a < 0.0 || a > d {
                return Err(VegetationError::Hydraulic("authorization domain"));
            }
            Ok(d.min(a))
        })
        .collect()
}
