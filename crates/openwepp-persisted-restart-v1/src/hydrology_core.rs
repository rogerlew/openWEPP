use openwepp_hillslope_orchestrator::DirectWaterState;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::HexF64;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DirectHydrologyCoreError {
    #[error("{field} must be finite and nonnegative")]
    WaterDomain { field: &'static str },
}

fn water_value(field: &'static str, value: &HexF64) -> Result<f64, DirectHydrologyCoreError> {
    let decoded = value.to_f64();
    if !decoded.is_finite() || decoded < 0.0 {
        return Err(DirectHydrologyCoreError::WaterDomain { field });
    }
    Ok(decoded)
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectWaterStateRestartV1 {
    pub soil_water_m: HexF64,
    pub infiltration_m: HexF64,
    pub runoff_m: HexF64,
    pub evapotranspiration_m: HexF64,
    pub drainage_m: HexF64,
    pub lateral_flow_m: HexF64,
}

impl DirectWaterStateRestartV1 {
    #[must_use]
    pub fn project(value: &DirectWaterState) -> Self {
        let DirectWaterState {
            soil_water_m,
            infiltration_m,
            runoff_m,
            evapotranspiration_m,
            drainage_m,
            lateral_flow_m,
        } = *value;
        Self {
            soil_water_m: HexF64::from_f64(soil_water_m),
            infiltration_m: HexF64::from_f64(infiltration_m),
            runoff_m: HexF64::from_f64(runoff_m),
            evapotranspiration_m: HexF64::from_f64(evapotranspiration_m),
            drainage_m: HexF64::from_f64(drainage_m),
            lateral_flow_m: HexF64::from_f64(lateral_flow_m),
        }
    }

    pub fn restore(&self) -> Result<DirectWaterState, DirectHydrologyCoreError> {
        Ok(DirectWaterState {
            soil_water_m: water_value("soil_water_m", &self.soil_water_m)?,
            infiltration_m: water_value("infiltration_m", &self.infiltration_m)?,
            runoff_m: water_value("runoff_m", &self.runoff_m)?,
            evapotranspiration_m: water_value("evapotranspiration_m", &self.evapotranspiration_m)?,
            drainage_m: water_value("drainage_m", &self.drainage_m)?,
            lateral_flow_m: water_value("lateral_flow_m", &self.lateral_flow_m)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_runtime_water_state_round_trips_bit_exactly() {
        let runtime = DirectWaterState {
            soil_water_m: 0.25,
            infiltration_m: 0.01,
            runoff_m: -0.0,
            evapotranspiration_m: 0.002,
            drainage_m: 0.003,
            lateral_flow_m: 0.004,
        };
        let restored = DirectWaterStateRestartV1::project(&runtime)
            .restore()
            .expect("valid water state");
        assert_eq!(
            restored.soil_water_m.to_bits(),
            runtime.soil_water_m.to_bits()
        );
        assert_eq!(
            restored.infiltration_m.to_bits(),
            runtime.infiltration_m.to_bits()
        );
        assert_eq!(restored.runoff_m.to_bits(), runtime.runoff_m.to_bits());
        assert_eq!(
            restored.evapotranspiration_m.to_bits(),
            runtime.evapotranspiration_m.to_bits()
        );
        assert_eq!(restored.drainage_m.to_bits(), runtime.drainage_m.to_bits());
        assert_eq!(
            restored.lateral_flow_m.to_bits(),
            runtime.lateral_flow_m.to_bits()
        );
    }
}
