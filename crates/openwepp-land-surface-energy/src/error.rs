use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LandSurfaceEnergyErrorClass {
    Malformed,
    Identity,
    Domain,
    Unsupported,
    Cardinality,
    Bound,
    Closure,
    OwnerEnvelope,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyErrorClass {
    Domain,
    Cardinality,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WaterErrorClass {
    Identity,
    Domain,
    Cardinality,
    Bound,
    Closure,
}

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
    #[error("invalid or duplicate topology at {detail}")]
    Topology {
        class: TopologyErrorClass,
        detail: &'static str,
    },
    #[error("nonfinite operand {0}")]
    NonFinite(&'static str),
    #[error("unsupported domain: {0}")]
    UnsupportedDomain(&'static str),
    #[error("constitutive domain violation: {0}")]
    ConstitutiveDomain(&'static str),
    #[error("water identity or bound violation: {detail}")]
    WaterIdentityOrBound {
        class: WaterErrorClass,
        detail: &'static str,
    },
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

impl LandSurfaceEnergyError {
    #[must_use]
    pub const fn topology_domain(detail: &'static str) -> Self {
        Self::Topology {
            class: TopologyErrorClass::Domain,
            detail,
        }
    }

    #[must_use]
    pub const fn topology_cardinality(detail: &'static str) -> Self {
        Self::Topology {
            class: TopologyErrorClass::Cardinality,
            detail,
        }
    }

    #[must_use]
    pub const fn water_identity(detail: &'static str) -> Self {
        Self::WaterIdentityOrBound {
            class: WaterErrorClass::Identity,
            detail,
        }
    }

    #[must_use]
    pub const fn water_domain(detail: &'static str) -> Self {
        Self::WaterIdentityOrBound {
            class: WaterErrorClass::Domain,
            detail,
        }
    }

    #[must_use]
    pub const fn water_cardinality(detail: &'static str) -> Self {
        Self::WaterIdentityOrBound {
            class: WaterErrorClass::Cardinality,
            detail,
        }
    }

    #[must_use]
    pub const fn water_bound(detail: &'static str) -> Self {
        Self::WaterIdentityOrBound {
            class: WaterErrorClass::Bound,
            detail,
        }
    }

    #[must_use]
    pub const fn water_closure(detail: &'static str) -> Self {
        Self::WaterIdentityOrBound {
            class: WaterErrorClass::Closure,
            detail,
        }
    }

    #[must_use]
    pub const fn class(&self) -> LandSurfaceEnergyErrorClass {
        match self {
            Self::MalformedSerialization(_) => LandSurfaceEnergyErrorClass::Malformed,
            Self::Identity { .. } | Self::StateLineage(_) => LandSurfaceEnergyErrorClass::Identity,
            Self::Topology { class, .. } => match class {
                TopologyErrorClass::Domain => LandSurfaceEnergyErrorClass::Domain,
                TopologyErrorClass::Cardinality => LandSurfaceEnergyErrorClass::Cardinality,
            },
            Self::NonFinite(_)
            | Self::ConstitutiveDomain(_)
            | Self::NumericalSingular { .. }
            | Self::NumericalBacktrackingLimit
            | Self::NumericalIterationLimit
            | Self::NumericalAcceptedResidual => LandSurfaceEnergyErrorClass::Domain,
            Self::UnsupportedDomain(_) => LandSurfaceEnergyErrorClass::Unsupported,
            Self::WaterIdentityOrBound { class, .. } => match class {
                WaterErrorClass::Identity => LandSurfaceEnergyErrorClass::Identity,
                WaterErrorClass::Domain => LandSurfaceEnergyErrorClass::Domain,
                WaterErrorClass::Cardinality => LandSurfaceEnergyErrorClass::Cardinality,
                WaterErrorClass::Bound => LandSurfaceEnergyErrorClass::Bound,
                WaterErrorClass::Closure => LandSurfaceEnergyErrorClass::Closure,
            },
            Self::OwnerEnvelope(_) => LandSurfaceEnergyErrorClass::OwnerEnvelope,
            Self::ComponentClosure(_)
            | Self::ControlVolumeClosure(_)
            | Self::LatentJoin(_)
            | Self::GroundHeatJoin(_) => LandSurfaceEnergyErrorClass::Closure,
        }
    }
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

pub(crate) fn require_finite_water_nonnegative(
    value: f64,
    field: &'static str,
) -> Result<(), LandSurfaceEnergyError> {
    require_finite(value, field)?;
    if value >= 0.0 {
        Ok(())
    } else {
        Err(LandSurfaceEnergyError::water_bound(field))
    }
}

pub(crate) fn require_finite_water_positive(
    value: f64,
    field: &'static str,
) -> Result<(), LandSurfaceEnergyError> {
    require_finite(value, field)?;
    if value > 0.0 {
        Ok(())
    } else {
        Err(LandSurfaceEnergyError::water_bound(field))
    }
}

#[cfg(test)]
mod tests {
    use super::{LandSurfaceEnergyError, LandSurfaceEnergyErrorClass};

    #[test]
    fn every_error_variant_and_typed_subcategory_has_an_explicit_class() {
        use LandSurfaceEnergyErrorClass as Class;

        let cases = vec![
            (
                LandSurfaceEnergyError::MalformedSerialization("bad".into()),
                Class::Malformed,
            ),
            (
                LandSurfaceEnergyError::Identity {
                    field: "owner",
                    expected: "a".into(),
                    found: "b".into(),
                },
                Class::Identity,
            ),
            (
                LandSurfaceEnergyError::topology_domain("open_trial_shape"),
                Class::Domain,
            ),
            (
                LandSurfaceEnergyError::topology_cardinality("duplicate tile"),
                Class::Cardinality,
            ),
            (LandSurfaceEnergyError::NonFinite("operand"), Class::Domain),
            (
                LandSurfaceEnergyError::UnsupportedDomain("snow"),
                Class::Unsupported,
            ),
            (
                LandSurfaceEnergyError::ConstitutiveDomain("temperature"),
                Class::Domain,
            ),
            (
                LandSurfaceEnergyError::water_identity("identity"),
                Class::Identity,
            ),
            (
                LandSurfaceEnergyError::water_domain("domain"),
                Class::Domain,
            ),
            (
                LandSurfaceEnergyError::water_cardinality("missing authorization"),
                Class::Cardinality,
            ),
            (LandSurfaceEnergyError::water_bound("D/A/F"), Class::Bound),
            (
                LandSurfaceEnergyError::water_closure("pre_ingress_source_mass_closure"),
                Class::Closure,
            ),
            (
                LandSurfaceEnergyError::StateLineage("stale"),
                Class::Identity,
            ),
            (
                LandSurfaceEnergyError::OwnerEnvelope("owner"),
                Class::OwnerEnvelope,
            ),
            (
                LandSurfaceEnergyError::NumericalSingular {
                    pivot: 0.0,
                    matrix_norm: 1.0,
                },
                Class::Domain,
            ),
            (
                LandSurfaceEnergyError::NumericalBacktrackingLimit,
                Class::Domain,
            ),
            (
                LandSurfaceEnergyError::NumericalIterationLimit,
                Class::Domain,
            ),
            (
                LandSurfaceEnergyError::NumericalAcceptedResidual,
                Class::Domain,
            ),
            (
                LandSurfaceEnergyError::ComponentClosure("component"),
                Class::Closure,
            ),
            (
                LandSurfaceEnergyError::ControlVolumeClosure("volume"),
                Class::Closure,
            ),
            (LandSurfaceEnergyError::LatentJoin("latent"), Class::Closure),
            (
                LandSurfaceEnergyError::GroundHeatJoin("ground"),
                Class::Closure,
            ),
        ];
        for (error, expected) in cases {
            assert_eq!(error.class(), expected, "{error}");
        }
    }
}
