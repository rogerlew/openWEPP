#![allow(clippy::missing_errors_doc)]

use std::collections::BTreeSet;

use openwepp_kernel_contract::TransactionId;
use serde::{Deserialize, Serialize};

use crate::{
    ComponentId, GroundWaterKey, LandSurfaceEnergyError, MODEL_DEFINITION_SHA256, MODEL_VERSION,
    OfeId, Sha256Digest, require_finite, require_finite_nonnegative,
};
use openwepp_kernel_contract::TileId;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SolvePass {
    Potential,
    FinalFixedCap,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SolveIdentity {
    JointCanopyGround,
    CanopyComponentCi,
    CanopyComponentEnergy,
    SharedCanopyAir,
    SurfaceEnergy,
    HydraulicSystem,
    SoilThermalSystem,
    IndependentClosure,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticFailureKind {
    MalformedSerialization,
    Identity,
    TopologyOrOwner,
    NonfiniteOperand,
    UnsupportedDomain,
    ConstitutiveDomain,
    WaterIdentityOrBound,
    SingularPivot,
    BacktrackingLimit,
    IterationLimit,
    AcceptedStepOrResidual,
    ComponentClosure,
    ControlVolumeClosure,
    CrossOwnerJoin,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum NumericalFailureCode {
    #[serde(rename = "LSEB-E-030")]
    LsebE030,
    #[serde(rename = "LSEB-E-031")]
    LsebE031,
    #[serde(rename = "LSEB-E-032")]
    LsebE032,
    #[serde(rename = "LSEB-E-033")]
    LsebE033,
    #[serde(rename = "LSEB-E-034")]
    LsebE034,
    #[serde(rename = "LSEB-E-035")]
    LsebE035,
    #[serde(rename = "LSEB-E-036")]
    LsebE036,
    #[serde(rename = "LSEB-E-037")]
    LsebE037,
    #[serde(rename = "LSEB-E-038")]
    LsebE038,
    #[serde(rename = "LSEB-E-039")]
    LsebE039,
    #[serde(rename = "LSEB-E-040")]
    LsebE040,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResidualUnit {
    #[serde(rename = "w_m-2")]
    WattsPerSquareMeter,
    #[serde(rename = "kg_m-2_s-1")]
    KilogramsPerSquareMeterSecond,
    #[serde(rename = "pa")]
    Pascal,
    #[serde(rename = "mm")]
    Millimeter,
    #[serde(rename = "kg_kg-1")]
    KilogramPerKilogram,
    Dimensionless,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NormalizedResidual {
    pub identity: String,
    pub raw: f64,
    pub scale: f64,
    pub tolerance: f64,
    pub normalized: f64,
    pub unit: ResidualUnit,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StepNorms {
    pub temperature_k: Option<f64>,
    pub humidity_kg_kg: Option<f64>,
    pub ci_pa: Option<f64>,
    pub hydraulic_mm: Option<f64>,
    pub beta: Option<f64>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnerKind {
    Vegetation,
    Hydrology,
    LandSurfaceEnergy,
    Biogeochemistry,
    SoilThermal,
    Envelope,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OwnerRollbackHash {
    pub owner_kind: OwnerKind,
    pub owner_id: String,
    pub before_sha256: Sha256Digest,
    pub after_sha256: Sha256Digest,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NumericalBracket {
    pub lower: f64,
    pub upper: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NumericalDiagnostics {
    pub model_version: String,
    pub canonical_contract: String,
    pub model_definition_sha256: Sha256Digest,
    pub configuration_sha256: Sha256Digest,
    pub beginning_state_sha256: Sha256Digest,
    pub transaction_id: TransactionId,
    pub ofe_id: OfeId,
    pub tile_id: TileId,
    pub occupancy_id: Option<ComponentId>,
    pub pass: SolvePass,
    pub solve: SolveIdentity,
    pub accepted: bool,
    pub failure_code: Option<NumericalFailureCode>,
    pub failure_kind: Option<DiagnosticFailureKind>,
    pub iterations: u32,
    pub backtracking_count: u32,
    pub ordered_residuals: Vec<NormalizedResidual>,
    pub step_norms: StepNorms,
    pub active_bounds: Vec<String>,
    pub active_water_caps: Vec<GroundWaterKey>,
    pub bracket: Option<NumericalBracket>,
    pub pivot_magnitude: Option<f64>,
    pub matrix_infinity_norm: Option<f64>,
    pub owner_rollback_hashes: Vec<OwnerRollbackHash>,
}

impl NumericalDiagnostics {
    pub fn validate(&self) -> Result<(), LandSurfaceEnergyError> {
        if self.model_version != MODEL_VERSION
            || self.model_definition_sha256.as_str() != MODEL_DEFINITION_SHA256
            || self.canonical_contract != "SC-LANDSURFACEENERGY-001@3"
        {
            return Err(LandSurfaceEnergyError::Identity {
                field: "diagnostics model authority",
                expected: format!("{MODEL_VERSION}/SC-LANDSURFACEENERGY-001@3"),
                found: format!("{}/{}", self.model_version, self.canonical_contract),
            });
        }
        if self.transaction_id.0 == 0 {
            return Err(LandSurfaceEnergyError::StateLineage(
                "zero diagnostics transaction",
            ));
        }
        if self.accepted != self.failure_kind.is_none()
            || self.accepted != self.failure_code.is_none()
        {
            return Err(LandSurfaceEnergyError::OwnerEnvelope(
                "accepted/failure diagnostics mismatch",
            ));
        }
        if self.failure_kind == Some(DiagnosticFailureKind::UnsupportedDomain)
            && self.failure_code != Some(NumericalFailureCode::LsebE030)
        {
            return Err(LandSurfaceEnergyError::OwnerEnvelope(
                "unsupported-domain failure code mismatch",
            ));
        }
        if matches!(
            self.failure_kind,
            Some(
                DiagnosticFailureKind::SingularPivot
                    | DiagnosticFailureKind::BacktrackingLimit
                    | DiagnosticFailureKind::IterationLimit
            )
        ) && self.failure_code != Some(NumericalFailureCode::LsebE034)
        {
            return Err(LandSurfaceEnergyError::OwnerEnvelope(
                "numerical failure code mismatch",
            ));
        }
        let mut residual_ids = BTreeSet::new();
        for residual in &self.ordered_residuals {
            if residual.identity.trim().is_empty() || !residual_ids.insert(&residual.identity) {
                return Err(LandSurfaceEnergyError::topology_cardinality(
                    "duplicate residual identity",
                ));
            }
            require_finite(residual.raw, "diagnostics residual raw")?;
            require_finite_nonnegative(residual.scale, "diagnostics residual scale")?;
            if !residual.tolerance.is_finite() || residual.tolerance <= 0.0 {
                return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                    "diagnostics residual tolerance",
                ));
            }
            require_finite(residual.normalized, "diagnostics normalized residual")?;
        }
        for value in [
            self.step_norms.temperature_k,
            self.step_norms.humidity_kg_kg,
            self.step_norms.ci_pa,
            self.step_norms.hydraulic_mm,
            self.step_norms.beta,
            self.pivot_magnitude,
            self.matrix_infinity_norm,
        ]
        .into_iter()
        .flatten()
        {
            require_finite_nonnegative(value, "diagnostics nonnegative scalar")?;
        }
        if let Some(bracket) = self.bracket {
            require_finite(bracket.lower, "diagnostics bracket lower")?;
            require_finite(bracket.upper, "diagnostics bracket upper")?;
            if bracket.lower > bracket.upper {
                return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                    "diagnostics bracket ordering",
                ));
            }
        }
        let rollback: BTreeSet<_> = self
            .owner_rollback_hashes
            .iter()
            .map(|row| (row.owner_kind, row.owner_id.as_str()))
            .collect();
        if rollback.len() != self.owner_rollback_hashes.len() || rollback.len() < 5 {
            return Err(LandSurfaceEnergyError::OwnerEnvelope(
                "rollback owner set incomplete or duplicate",
            ));
        }
        Ok(())
    }
}
