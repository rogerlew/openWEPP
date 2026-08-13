//! Canonical V3 numerical failure identities and payloads.

use openwepp_kernel_contract::{
    OccupancyId, ResourceAmountBasis, ResourceOwnerId, SoilLayerId, TransactionId,
};
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
    AuthorizationValidation,
    SunCi,
    ShadeCi,
    CanopyEnergy,
    HydraulicSystem,
    OuterGasEnergyHydraulicCoupling,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FixedAuthorizationIdentity {
    pub transaction_id: TransactionId,
    pub owner_id: ResourceOwnerId,
    pub occupancy_id: OccupancyId,
    pub basis: ResourceAmountBasis,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct NormalizedResidual {
    pub identity: String,
    pub value: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CappedResidualOperands {
    pub identity: String,
    pub raw_kg_m2_tile_s: f64,
    pub scale_kg_m2_tile_s: f64,
    pub tolerance: f64,
    pub normalized: f64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BoundIdentity(pub String);

/// V5 fixed-authorization layer operands retained at the exact accepted or
/// failed nonlinear iterate, in configured root-layer order.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CappedLayerNumericalOperands {
    pub layer_id: SoilLayerId,
    pub cap_rate_kg_m2_tile_s: f64,
    pub q_law_kg_m2_tile_s: f64,
    pub q_final_kg_m2_tile_s: f64,
    pub authorization_active_or_tie: bool,
    pub soil_potential_mm: f64,
    pub gravity_head_mm: f64,
    pub root_fraction: f64,
    pub z3_m: f64,
    pub ksoil_m2_s: f64,
    pub dxroot_m: f64,
    pub accessible: bool,
    pub frozen: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CappedNumericalOperands {
    pub water_residual_scale_kg_m2_tile_s: f64,
    pub psi_sunleaf_mm: f64,
    pub psi_shadeleaf_mm: f64,
    pub psi_stem_mm: f64,
    pub psi_root_mm: f64,
    pub beta_sun: f64,
    pub beta_shade: f64,
    pub emax_sun_kg_m2_s: f64,
    pub emax_shade_kg_m2_s: f64,
    pub gas_sun_kg_m2_s: f64,
    pub gas_shade_kg_m2_s: f64,
    pub q1_sun_kg_m2_s: f64,
    pub q1_shade_kg_m2_s: f64,
    pub q2_kg_m2_s: f64,
    pub residuals: Vec<CappedResidualOperands>,
    pub layers: Vec<CappedLayerNumericalOperands>,
}

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
    /// Present only for a V5 capped solve after fixed layer operands have been
    /// evaluated. V3 potential diagnostics remain byte/shape compatible with
    /// `None` through serde's omission rule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capped_operands: Option<CappedNumericalOperands>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_authorization_identity: Option<FixedAuthorizationIdentity>,
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
            || self.capped_operands.as_ref().is_some_and(|operands| {
                ![
                    operands.water_residual_scale_kg_m2_tile_s,
                    operands.psi_sunleaf_mm,
                    operands.psi_shadeleaf_mm,
                    operands.psi_stem_mm,
                    operands.psi_root_mm,
                    operands.beta_sun,
                    operands.beta_shade,
                    operands.emax_sun_kg_m2_s,
                    operands.emax_shade_kg_m2_s,
                    operands.gas_sun_kg_m2_s,
                    operands.gas_shade_kg_m2_s,
                    operands.q1_sun_kg_m2_s,
                    operands.q1_shade_kg_m2_s,
                    operands.q2_kg_m2_s,
                ]
                .iter()
                .all(|value| value.is_finite())
                    || operands.water_residual_scale_kg_m2_tile_s < 0.0
                    || operands.layers.iter().any(|layer| {
                        ![
                            layer.cap_rate_kg_m2_tile_s,
                            layer.q_law_kg_m2_tile_s,
                            layer.q_final_kg_m2_tile_s,
                            layer.soil_potential_mm,
                            layer.gravity_head_mm,
                            layer.root_fraction,
                            layer.z3_m,
                            layer.ksoil_m2_s,
                            layer.dxroot_m,
                        ]
                        .iter()
                        .all(|value| value.is_finite())
                    })
                    || operands.residuals.iter().any(|residual| {
                        residual.identity.is_empty()
                            || ![
                                residual.raw_kg_m2_tile_s,
                                residual.scale_kg_m2_tile_s,
                                residual.tolerance,
                                residual.normalized,
                            ]
                            .iter()
                            .all(|value| value.is_finite())
                            || residual.scale_kg_m2_tile_s < 0.0
                            || residual.tolerance <= 0.0
                    })
            })
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
            capped_operands: None,
            fixed_authorization_identity: None,
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
