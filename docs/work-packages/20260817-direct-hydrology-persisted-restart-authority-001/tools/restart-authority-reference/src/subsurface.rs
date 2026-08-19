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
        Ok(DirectSubsurfaceLayerState {
            theta_m: nonnegative("theta_m", &self.theta_m)?,
            field_capacity_m: nonnegative("field_capacity_m", &self.field_capacity_m)?,
            upper_limit_m: nonnegative("upper_limit_m", &self.upper_limit_m)?,
            conductivity_m_s: nonnegative("conductivity_m_s", &self.conductivity_m_s)?,
            depth_m: nonnegative("depth_m", &self.depth_m)?,
            residual_theta: fraction("residual_theta", &self.residual_theta)?,
            frozen_depth_m: nonnegative("frozen_depth_m", &self.frozen_depth_m)?,
            frozen_water_m: nonnegative("frozen_water_m", &self.frozen_water_m)?,
            porosity: fraction("porosity", &self.porosity)?,
            field_capacity_theta: fraction("field_capacity_theta", &self.field_capacity_theta)?,
            coca: fraction("coca", &self.coca)?,
            lateral_conductivity_m_s: nonnegative(
                "lateral_conductivity_m_s",
                &self.lateral_conductivity_m_s,
            )?,
        })
    }
}
