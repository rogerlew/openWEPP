use thiserror::Error;

/// Typed fail-closed errors for the strict LSE runtime boundary.
#[derive(Clone, Debug, Error, PartialEq)]
pub enum LandSurfaceEnergyError {
    #[error("malformed serialization: {0}")]
    MalformedSerialization(String),
    #[error("identity mismatch for {field}: expected {expected}, found {found}")]
    Identity {
        field: &'static str,
        expected: String,
        found: String,
    },
    #[error("invalid or duplicate topology at {0}")]
    Topology(&'static str),
    #[error("nonfinite operand {0}")]
    NonFinite(&'static str),
    #[error("unsupported domain: {0}")]
    UnsupportedDomain(&'static str),
    #[error("constitutive domain violation: {0}")]
    ConstitutiveDomain(&'static str),
    #[error("water identity or bound violation: {0}")]
    WaterIdentityOrBound(&'static str),
    #[error("state lineage violation: {0}")]
    StateLineage(&'static str),
    #[error("candidate owner envelope violation: {0}")]
    OwnerEnvelope(&'static str),
    #[error("singular numerical system: pivot={pivot}, matrix infinity norm={matrix_norm}")]
    NumericalSingular { pivot: f64, matrix_norm: f64 },
    #[error("numerical backtracking limit")]
    NumericalBacktrackingLimit,
    #[error("numerical iteration limit")]
    NumericalIterationLimit,
    #[error("accepted iterate failed residual or step acceptance")]
    NumericalAcceptedResidual,
    #[error("component energy closure failed: {0}")]
    ComponentClosure(&'static str),
    #[error("control-volume closure failed: {0}")]
    ControlVolumeClosure(&'static str),
    #[error("latent mass-energy join failed: {0}")]
    LatentJoin(&'static str),
    #[error("ground-heat equal/opposite join failed: {0}")]
    GroundHeatJoin(&'static str),
}

pub(crate) fn require_finite(
    value: f64,
    field: &'static str,
) -> Result<(), LandSurfaceEnergyError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(LandSurfaceEnergyError::NonFinite(field))
    }
}

pub(crate) fn require_finite_positive(
    value: f64,
    field: &'static str,
) -> Result<(), LandSurfaceEnergyError> {
    require_finite(value, field)?;
    if value > 0.0 {
        Ok(())
    } else {
        Err(LandSurfaceEnergyError::ConstitutiveDomain(field))
    }
}

pub(crate) fn require_finite_nonnegative(
    value: f64,
    field: &'static str,
) -> Result<(), LandSurfaceEnergyError> {
    require_finite(value, field)?;
    if value >= 0.0 {
        Ok(())
    } else {
        Err(LandSurfaceEnergyError::ConstitutiveDomain(field))
    }
}
