use crate::HexF64;
use openwepp_hillslope_orchestrator::DirectSubsurfaceLayerState;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SubsurfaceRestartError {
    #[error("{field} must be finite and nonnegative")]
    Nonnegative { field: &'static str },
    #[error("{field} must be within 0..=1")]
    Fraction { field: &'static str },
    #[error("{field} must be finite and positive")]
    Positive { field: &'static str },
    #[error("{lower} exceeds {upper} beyond the canonical 1e-12 tolerance")]
    CrossField {
        lower: &'static str,
        upper: &'static str,
    },
}
fn nonnegative(field: &'static str, v: &HexF64) -> Result<f64, SubsurfaceRestartError> {
    let x = v.to_f64();
    (x.is_finite() && x >= 0.0)
        .then_some(x)
        .ok_or(SubsurfaceRestartError::Nonnegative { field })
}
fn fraction(field: &'static str, v: &HexF64) -> Result<f64, SubsurfaceRestartError> {
    let x = v.to_f64();
    (x.is_finite() && (0.0..=1.0).contains(&x))
        .then_some(x)
        .ok_or(SubsurfaceRestartError::Fraction { field })
}
fn positive(field: &'static str, v: &HexF64) -> Result<f64, SubsurfaceRestartError> {
    let x = v.to_f64();
    (x.is_finite() && x > 0.0)
        .then_some(x)
        .ok_or(SubsurfaceRestartError::Positive { field })
}
fn positive_fraction(field: &'static str, v: &HexF64) -> Result<f64, SubsurfaceRestartError> {
    let x = v.to_f64();
    (x.is_finite() && x > 0.0 && x <= 1.0)
        .then_some(x)
        .ok_or(SubsurfaceRestartError::Fraction { field })
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectSubsurfaceLayerRestartV1 {
    pub theta_m: HexF64,
    pub field_capacity_m: HexF64,
    pub upper_limit_m: HexF64,
    pub conductivity_m_s: HexF64,
    pub depth_m: HexF64,
    pub residual_theta: HexF64,
    pub frozen_depth_m: HexF64,
    pub frozen_water_m: HexF64,
    pub porosity: HexF64,
    pub field_capacity_theta: HexF64,
    pub coca: HexF64,
    pub lateral_conductivity_m_s: HexF64,
}
impl DirectSubsurfaceLayerRestartV1 {
    pub fn project(v: &DirectSubsurfaceLayerState) -> Self {
        let DirectSubsurfaceLayerState {
            theta_m,
            field_capacity_m,
            upper_limit_m,
            conductivity_m_s,
            depth_m,
            residual_theta,
            frozen_depth_m,
            frozen_water_m,
            porosity,
            field_capacity_theta,
            coca,
            lateral_conductivity_m_s,
        } = *v;
        Self {
            theta_m: HexF64::from_f64(theta_m),
            field_capacity_m: HexF64::from_f64(field_capacity_m),
            upper_limit_m: HexF64::from_f64(upper_limit_m),
            conductivity_m_s: HexF64::from_f64(conductivity_m_s),
            depth_m: HexF64::from_f64(depth_m),
            residual_theta: HexF64::from_f64(residual_theta),
            frozen_depth_m: HexF64::from_f64(frozen_depth_m),
            frozen_water_m: HexF64::from_f64(frozen_water_m),
            porosity: HexF64::from_f64(porosity),
            field_capacity_theta: HexF64::from_f64(field_capacity_theta),
            coca: HexF64::from_f64(coca),
            lateral_conductivity_m_s: HexF64::from_f64(lateral_conductivity_m_s),
        }
    }
    pub fn restore(&self) -> Result<DirectSubsurfaceLayerState, SubsurfaceRestartError> {
        const WB11_ZERO_THRESHOLD: f64 = 1.0e-12;
        let restored = DirectSubsurfaceLayerState {
            theta_m: nonnegative("theta_m", &self.theta_m)?,
            field_capacity_m: nonnegative("field_capacity_m", &self.field_capacity_m)?,
            upper_limit_m: positive("upper_limit_m", &self.upper_limit_m)?,
            conductivity_m_s: positive("conductivity_m_s", &self.conductivity_m_s)?,
            depth_m: positive("depth_m", &self.depth_m)?,
            residual_theta: fraction("residual_theta", &self.residual_theta)?,
            frozen_depth_m: nonnegative("frozen_depth_m", &self.frozen_depth_m)?,
            frozen_water_m: nonnegative("frozen_water_m", &self.frozen_water_m)?,
            porosity: positive_fraction("porosity", &self.porosity)?,
            field_capacity_theta: positive_fraction(
                "field_capacity_theta",
                &self.field_capacity_theta,
            )?,
            coca: positive_fraction("coca", &self.coca)?,
            lateral_conductivity_m_s: positive(
                "lateral_conductivity_m_s",
                &self.lateral_conductivity_m_s,
            )?,
        };
        if restored.field_capacity_m > restored.upper_limit_m + WB11_ZERO_THRESHOLD {
            return Err(SubsurfaceRestartError::CrossField {
                lower: "field_capacity_m",
                upper: "upper_limit_m",
            });
        }
        if restored.frozen_depth_m > restored.depth_m + WB11_ZERO_THRESHOLD {
            return Err(SubsurfaceRestartError::CrossField {
                lower: "frozen_depth_m",
                upper: "depth_m",
            });
        }
        Ok(restored)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn valid() -> DirectSubsurfaceLayerState {
        DirectSubsurfaceLayerState {
            theta_m: -0.0,
            field_capacity_m: 0.2,
            upper_limit_m: 0.3,
            conductivity_m_s: 1e-6,
            depth_m: 0.5,
            residual_theta: 0.1,
            frozen_depth_m: 0.2,
            frozen_water_m: 0.01,
            porosity: 0.45,
            field_capacity_theta: 0.3,
            coca: 0.8,
            lateral_conductivity_m_s: 2e-6,
        }
    }
    #[test]
    fn layer_round_trips_every_field_bit_exactly() {
        let dto = DirectSubsurfaceLayerRestartV1::project(&valid());
        assert_eq!(
            DirectSubsurfaceLayerRestartV1::project(&dto.restore().expect("valid layer")),
            dto
        );
    }
    #[test]
    fn positive_and_cross_field_domains_are_enforced() {
        let mut dto = DirectSubsurfaceLayerRestartV1::project(&valid());
        dto.conductivity_m_s = HexF64::from_f64(0.0);
        assert_eq!(
            dto.restore(),
            Err(SubsurfaceRestartError::Positive {
                field: "conductivity_m_s"
            })
        );
        dto = DirectSubsurfaceLayerRestartV1::project(&valid());
        dto.field_capacity_m = HexF64::from_f64(0.300_000_000_002);
        assert_eq!(
            dto.restore(),
            Err(SubsurfaceRestartError::CrossField {
                lower: "field_capacity_m",
                upper: "upper_limit_m"
            })
        );
        dto = DirectSubsurfaceLayerRestartV1::project(&valid());
        dto.frozen_depth_m = HexF64::from_f64(0.500_000_000_002);
        assert_eq!(
            dto.restore(),
            Err(SubsurfaceRestartError::CrossField {
                lower: "frozen_depth_m",
                upper: "depth_m"
            })
        );
    }
}
