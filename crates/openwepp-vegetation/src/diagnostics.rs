//! Canonical V3 numerical failure identities and payloads.

use openwepp_kernel_contract::{OccupancyId, SoilLayerId, TransactionId};
use serde::{Deserialize, Serialize};

use crate::{MODEL_SHA256, VegetationError};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CoupledSolvePass {
    Potential,
    Capped,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SolveIdentity {
    SunCi,
    ShadeCi,
    CanopyEnergy,
    HydraulicSystem,
    OuterGasEnergyHydraulicCoupling,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct NormalizedResidual {
    pub identity: String,
    pub value: f64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BoundIdentity(pub String);

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct NumericalFailureDiagnostics {
    pub model_definition_sha256: String,
    pub transaction_id: TransactionId,
    pub occupancy_id: OccupancyId,
    pub pass: CoupledSolvePass,
    pub solve: SolveIdentity,
    pub iterations: u32,
    pub residual_norms: Vec<NormalizedResidual>,
    pub step_norm: Option<f64>,
    pub backtracking_count: u32,
    pub active_bounds: Vec<BoundIdentity>,
    pub active_water_caps: Vec<SoilLayerId>,
    pub bracket: Option<(f64, f64)>,
    pub pivot_magnitude: Option<f64>,
    pub matrix_norm: Option<f64>,
}

impl NumericalFailureDiagnostics {
    pub fn validate(&self) -> Result<(), VegetationError> {
        if self.model_definition_sha256 != MODEL_SHA256
            || self
                .residual_norms
                .iter()
                .any(|residual| !residual.value.is_finite())
            || self.step_norm.is_some_and(|value| !value.is_finite())
            || self
                .bracket
                .is_some_and(|(lower, upper)| !lower.is_finite() || !upper.is_finite())
            || self.pivot_magnitude.is_some_and(|value| !value.is_finite())
            || self.matrix_norm.is_some_and(|value| !value.is_finite())
        {
            return Err(VegetationError::Domain("V3 numerical failure diagnostics"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openwepp_kernel_contract::{StratumId, TileId};

    fn payload() -> NumericalFailureDiagnostics {
        NumericalFailureDiagnostics {
            model_definition_sha256: MODEL_SHA256.into(),
            transaction_id: TransactionId(9),
            occupancy_id: OccupancyId {
                stratum_id: StratumId::try_new("tree").expect("stratum"),
                tile_id: TileId::try_new("tile").expect("tile"),
            },
            pass: CoupledSolvePass::Potential,
            solve: SolveIdentity::OuterGasEnergyHydraulicCoupling,
            iterations: 50,
            residual_norms: vec![NormalizedResidual {
                identity: "sun_gas_equals_q1".into(),
                value: 1.1,
            }],
            step_norm: Some(0.01),
            backtracking_count: 20,
            active_bounds: vec![BoundIdentity("beta_sun_lower".into())],
            active_water_caps: Vec::new(),
            bracket: None,
            pivot_magnitude: Some(1.0e-14),
            matrix_norm: Some(1.0),
        }
    }

    #[test]
    fn exact_v3_failure_payload_is_finite_and_identity_bound() {
        let payload = payload();
        payload.validate().expect("canonical diagnostics");
        assert_eq!(payload.iterations, 50);
        assert_eq!(payload.backtracking_count, 20);

        let mut wrong = payload.clone();
        wrong.model_definition_sha256 = "0".repeat(64);
        assert!(wrong.validate().is_err());
        let mut nonfinite = payload;
        nonfinite.residual_norms[0].value = f64::NAN;
        assert!(nonfinite.validate().is_err());
    }
}
