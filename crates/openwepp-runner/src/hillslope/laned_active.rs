//! Lane D ACTIVE production owner selector (`SC-OFEROUTE-001` rev 27,
//! `INV-OFEROUTE-012` activation): opt-in via `OPENWEPP_LANED_ACTIVE=1` on
//! the production publication-stream path. The runner builds the per-lane
//! active configuration from the SAME rev-20/21 authority sources as the
//! diagnostic shadow (native management `routing_coefficients`, Wave-1
//! operand-seed geometry, typed-management `canhgt`) and attaches it to the
//! direct run frame; the orchestrator executor then owns the two-phase
//! active day loop, the DC01-surface-disable, the D13 erosion producer
//! flip, and the day-closure hard-fails. Mutually exclusive with
//! `OPENWEPP_LANED_SHADOW=1` (the shadow's published-row reconstruction
//! basis is DC01-shaped and is not defined over an active run).

use crate::HillslopeCliError;

pub(crate) const ACTIVE_MESH_TARGET_DX_ENV: &str = "OPENWEPP_LANED_ACTIVE_MESH_TARGET_DX_M";
pub(crate) const ACTIVE_TRACE_ENV: &str = "OPENWEPP_LANED_ACTIVE_TRACE";

/// Env opt-in: `OPENWEPP_LANED_ACTIVE=1`.
#[must_use]
pub(crate) fn env_enabled() -> bool {
    std::env::var("OPENWEPP_LANED_ACTIVE").is_ok_and(|value| value == "1")
}

#[must_use]
pub(crate) fn trace_enabled() -> bool {
    trace_enabled_from_value(std::env::var(ACTIVE_TRACE_ENV).ok().as_deref())
}

pub(crate) fn validate_trace_selector_env() -> Result<(), HillslopeCliError> {
    if trace_enabled() && !env_enabled() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: ACTIVE_TRACE_ENV,
            detail: format!(
                "{ACTIVE_TRACE_ENV}=1 requires OPENWEPP_LANED_ACTIVE=1 so diagnostic trace rows cannot be requested for an inactive/default production path"
            ),
        });
    }
    Ok(())
}

fn trace_enabled_from_value(value: Option<&str>) -> bool {
    value.is_some_and(|value| value == "1")
}

pub(crate) fn mesh_policy_from_env()
-> Result<openwepp_hillslope_orchestrator::DirectLanedActiveMeshPolicy, HillslopeCliError> {
    match std::env::var(ACTIVE_MESH_TARGET_DX_ENV) {
        Ok(value) => mesh_policy_from_target_dx_value(Some(&value)),
        Err(std::env::VarError::NotPresent) => mesh_policy_from_target_dx_value(None),
        Err(std::env::VarError::NotUnicode(_)) => Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: ACTIVE_MESH_TARGET_DX_ENV,
            detail: "expected UTF-8 finite positive target dx in meters".to_string(),
        }),
    }
}

fn mesh_policy_from_target_dx_value(
    value: Option<&str>,
) -> Result<openwepp_hillslope_orchestrator::DirectLanedActiveMeshPolicy, HillslopeCliError> {
    let Some(value) = value else {
        return Ok(
            openwepp_hillslope_orchestrator::DirectLanedActiveMeshPolicy::production_default(),
        );
    };
    let target_dx_m =
        value
            .parse::<f64>()
            .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
                surface: ACTIVE_MESH_TARGET_DX_ENV,
                detail: format!("expected finite positive target dx in meters: {error}"),
            })?;
    openwepp_hillslope_orchestrator::DirectLanedActiveMeshPolicy::diagnostic_target_dx(target_dx_m)
        .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
            surface: ACTIVE_MESH_TARGET_DX_ENV,
            detail: format!("{error}"),
        })
}

/// ADR-0037 abandonment removal: the former implicit selector is rejected
/// whenever present so stale operator environments cannot silently proceed.
pub(crate) fn reject_abandoned_implicit_selector_env() -> Result<(), HillslopeCliError> {
    if std::env::var_os("OPENWEPP_LANED_ACTIVE_IMPLICIT").is_some() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "OPENWEPP_LANED_ACTIVE_IMPLICIT",
            detail: "OPENWEPP_LANED_ACTIVE_IMPLICIT was removed by ADR-0037: hybrid implicit-explicit stepping is abandoned and archived on branch abandoned/hybrid-implicit-stepping; unset this variable to run the active plain router".to_string(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{mesh_policy_from_target_dx_value, trace_enabled_from_value};
    use openwepp_hillslope_orchestrator::DirectLanedActiveMeshPolicy;

    #[test]
    fn mesh_policy_parser_defaults_parses_and_rejects_invalid_target_dx() {
        assert_eq!(
            mesh_policy_from_target_dx_value(None).expect("default policy"),
            DirectLanedActiveMeshPolicy::production_default()
        );
        assert_eq!(
            mesh_policy_from_target_dx_value(Some("20.0")).expect("target dx policy"),
            DirectLanedActiveMeshPolicy::diagnostic_target_dx(20.0).expect("target dx")
        );
        assert!(mesh_policy_from_target_dx_value(Some("0")).is_err());
        assert!(mesh_policy_from_target_dx_value(Some("not-a-number")).is_err());
    }

    #[test]
    fn trace_selector_requires_explicit_one() {
        assert!(!trace_enabled_from_value(None));
        assert!(!trace_enabled_from_value(Some("0")));
        assert!(trace_enabled_from_value(Some("1")));
    }
}
