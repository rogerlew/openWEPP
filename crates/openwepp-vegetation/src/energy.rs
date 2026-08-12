//! E06/E13 neutral aerodynamics, saturation humidity, and energy closure.
use crate::VegetationError;

pub const STEFAN_BOLTZMANN: f64 = 5.670_374_419e-8;
pub const LATENT_HEAT_VAPORIZATION: f64 = 2_501_000.0;
const ESAT_HPA: [f64; 9] = [
    6.112_134_76,
    0.444_007_856,
    0.014_306_423_4,
    0.000_264_461_437,
    0.000_003_059_035_58,
    1.962_372_41e-8,
    8.923_447_72e-11,
    -3.732_084_10e-13,
    2.093_399_97e-16,
];

pub fn neutral_resistance(
    z_ref: f64,
    displacement: f64,
    z0_a: f64,
    z0_b: f64,
    wind: f64,
) -> Result<f64, VegetationError> {
    if [z_ref, displacement, z0_a, z0_b, wind]
        .iter()
        .any(|v| !v.is_finite())
        || z0_a <= 0.0
        || z0_b <= 0.0
        || wind <= 0.0
        || z_ref <= displacement + z0_a.max(z0_b)
    {
        return Err(VegetationError::Unsupported(
            "CALM_OR_NONNEUTRAL_AERODYNAMICS",
        ));
    }
    Ok(
        ((z_ref - displacement) / z0_a).ln() * ((z_ref - displacement) / z0_b).ln()
            / (0.4_f64.powi(2) * wind),
    )
}

pub fn leaf_boundary_conductance(wind_leaf: f64, dimension: f64) -> Result<f64, VegetationError> {
    if !wind_leaf.is_finite() || !dimension.is_finite() || wind_leaf < 0.0 || dimension <= 0.0 {
        return Err(VegetationError::Domain("leaf boundary conductance"));
    }
    Ok(0.01 * (wind_leaf / dimension).sqrt())
}

pub fn saturation_vapor_pressure_pa(temperature_k: f64) -> Result<f64, VegetationError> {
    if !temperature_k.is_finite() || !(273.15..=373.15).contains(&temperature_k) {
        return Err(VegetationError::Domain("liquid saturation temperature"));
    }
    let celsius = temperature_k - 273.15;
    let hpa = ESAT_HPA
        .iter()
        .rev()
        .fold(0.0, |sum, coefficient| sum * celsius + coefficient);
    Ok(hpa * 100.0)
}

pub fn saturation_specific_humidity(
    temperature_k: f64,
    pressure_pa: f64,
) -> Result<f64, VegetationError> {
    let esat = saturation_vapor_pressure_pa(temperature_k)?;
    if !pressure_pa.is_finite() || pressure_pa <= 0.378 * esat {
        return Err(VegetationError::Domain("saturation specific humidity"));
    }
    Ok(0.622 * esat / (pressure_pa - 0.378 * esat))
}

pub fn energy_residual(
    absorbed: f64,
    sensible: f64,
    evaporation_kg_m2_s: f64,
    storage_or_conduction: f64,
) -> Result<f64, VegetationError> {
    if [
        absorbed,
        sensible,
        evaporation_kg_m2_s,
        storage_or_conduction,
    ]
    .iter()
    .any(|value| !value.is_finite())
    {
        return Err(VegetationError::Domain("energy ledger"));
    }
    Ok(
        absorbed
            - sensible
            - evaporation_kg_m2_s * LATENT_HEAT_VAPORIZATION
            - storage_or_conduction,
    )
}
