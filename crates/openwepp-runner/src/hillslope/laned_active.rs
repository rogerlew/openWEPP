//! Lane D ACTIVE production owner selector (`SC-OFEROUTE-001` rev 46,
//! `INV-OFEROUTE-010` / `INV-OFEROUTE-012` activation): explicit opt-in via
//! `OPENWEPP_LANED_ACTIVE=1`, plus conditional default activation when every
//! scheduled lane has native management `routing_coefficients`. The runner
//! builds the per-lane active configuration from the SAME rev-20/21 authority
//! sources as the diagnostic shadow (native management `routing_coefficients`,
//! Wave-1 operand-seed geometry, typed-management `canhgt`) and attaches it to
//! the direct run frame; the orchestrator executor then owns the two-phase
//! active day loop, the DC01-surface-disable, the D13 erosion producer flip,
//! the rev-45 `dx5` active production mesh default, and the day-closure
//! hard-fails. Mutually exclusive with
//! `OPENWEPP_LANED_SHADOW=1` (the shadow's published-row reconstruction
//! basis is DC01-shaped and is not defined over an active run). The explicit
//! disable selector forces the legacy/off path for rollback and cannot coexist
//! with explicit active opt-in.

use crate::HillslopeCliError;

pub(crate) const ACTIVE_MAX_DT_ENV: &str = "OPENWEPP_LANED_ACTIVE_MAX_DT_S";
pub(crate) const ACTIVE_DISABLE_ENV: &str = "OPENWEPP_LANED_ACTIVE_DISABLE";
pub(crate) const ACTIVE_MESH_TARGET_DX_ENV: &str = "OPENWEPP_LANED_ACTIVE_MESH_TARGET_DX_M";
pub(crate) const ACTIVE_STEP_TRACE_ENV: &str = "OPENWEPP_LANED_ACTIVE_STEP_TRACE";
pub(crate) const ACTIVE_TRACE_DETAIL_ENV: &str = "OPENWEPP_LANED_ACTIVE_TRACE_DETAIL";
pub(crate) const ACTIVE_TRACE_ENV: &str = "OPENWEPP_LANED_ACTIVE_TRACE";

/// Env opt-in: `OPENWEPP_LANED_ACTIVE=1`.
#[must_use]
pub(crate) fn env_enabled() -> bool {
    std::env::var("OPENWEPP_LANED_ACTIVE").is_ok_and(|value| value == "1")
}

#[must_use]
pub(crate) fn disable_enabled() -> bool {
    std::env::var(ACTIVE_DISABLE_ENV).is_ok_and(|value| value == "1")
}

#[must_use]
pub(crate) fn trace_enabled() -> bool {
    trace_enabled_from_value(std::env::var(ACTIVE_TRACE_ENV).ok().as_deref())
}

#[derive(Debug, Default, Clone, Copy)]
struct ActiveTraceSelectorState(u8);

impl ActiveTraceSelectorState {
    const ACTIVE: u8 = 1 << 0;
    const TRACE: u8 = 1 << 1;
    const TRACE_DETAIL: u8 = 1 << 2;
    const STEP_TRACE: u8 = 1 << 3;
    const MAX_DT: u8 = 1 << 4;

    const fn with_active(self) -> Self {
        Self(self.0 | Self::ACTIVE)
    }

    const fn with_trace(self) -> Self {
        Self(self.0 | Self::TRACE)
    }

    const fn with_trace_detail(self) -> Self {
        Self(self.0 | Self::TRACE_DETAIL)
    }

    const fn with_step_trace(self) -> Self {
        Self(self.0 | Self::STEP_TRACE)
    }

    const fn with_max_dt(self) -> Self {
        Self(self.0 | Self::MAX_DT)
    }

    const fn has(self, flag: u8) -> bool {
        self.0 & flag != 0
    }
}

fn validate_trace_selector_combination(
    state: ActiveTraceSelectorState,
) -> Result<(), HillslopeCliError> {
    if state.has(ActiveTraceSelectorState::TRACE) && !state.has(ActiveTraceSelectorState::ACTIVE) {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: ACTIVE_TRACE_ENV,
            detail: format!(
                "{ACTIVE_TRACE_ENV}=1 requires OPENWEPP_LANED_ACTIVE=1 so diagnostic trace rows cannot be requested for an inactive/default production path"
            ),
        });
    }
    if state.has(ActiveTraceSelectorState::TRACE_DETAIL)
        && (!state.has(ActiveTraceSelectorState::ACTIVE)
            || !state.has(ActiveTraceSelectorState::TRACE))
    {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: ACTIVE_TRACE_DETAIL_ENV,
            detail: format!(
                "{ACTIVE_TRACE_DETAIL_ENV}=<sim_day:lane> requires {ACTIVE_TRACE_ENV}=1 and OPENWEPP_LANED_ACTIVE=1"
            ),
        });
    }
    if state.has(ActiveTraceSelectorState::MAX_DT)
        && (!state.has(ActiveTraceSelectorState::ACTIVE)
            || !state.has(ActiveTraceSelectorState::TRACE))
    {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: ACTIVE_MAX_DT_ENV,
            detail: format!(
                "{ACTIVE_MAX_DT_ENV}=<seconds> is diagnostic evidence only and requires {ACTIVE_TRACE_ENV}=1 and OPENWEPP_LANED_ACTIVE=1"
            ),
        });
    }
    if state.has(ActiveTraceSelectorState::STEP_TRACE)
        && (!state.has(ActiveTraceSelectorState::ACTIVE)
            || !state.has(ActiveTraceSelectorState::TRACE)
            || !state.has(ActiveTraceSelectorState::TRACE_DETAIL))
    {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: ACTIVE_STEP_TRACE_ENV,
            detail: format!(
                "{ACTIVE_STEP_TRACE_ENV}=1 requires OPENWEPP_LANED_ACTIVE=1, {ACTIVE_TRACE_ENV}=1, and {ACTIVE_TRACE_DETAIL_ENV}=<sim_day:lane>"
            ),
        });
    }
    Ok(())
}

fn trace_enabled_from_value(value: Option<&str>) -> bool {
    value.is_some_and(|value| value == "1")
}

#[must_use]
pub(crate) fn step_trace_enabled() -> bool {
    step_trace_enabled_from_value(std::env::var(ACTIVE_STEP_TRACE_ENV).ok().as_deref())
}

fn step_trace_enabled_from_value(value: Option<&str>) -> bool {
    value.is_some_and(|value| value == "1")
}

pub(crate) fn trace_detail_filter_from_env() -> Result<
    Option<openwepp_hillslope_orchestrator::DirectLanedActiveTraceDetailFilter>,
    HillslopeCliError,
> {
    match std::env::var(ACTIVE_TRACE_DETAIL_ENV) {
        Ok(value) => trace_detail_filter_from_value(Some(&value)),
        Err(std::env::VarError::NotPresent) => trace_detail_filter_from_value(None),
        Err(std::env::VarError::NotUnicode(_)) => Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: ACTIVE_TRACE_DETAIL_ENV,
            detail: "expected UTF-8 one-based sim_day:lane selector".to_string(),
        }),
    }
}

fn trace_detail_filter_from_value(
    value: Option<&str>,
) -> Result<
    Option<openwepp_hillslope_orchestrator::DirectLanedActiveTraceDetailFilter>,
    HillslopeCliError,
> {
    let Some(value) = value else {
        return Ok(None);
    };
    let (day, lane) =
        value
            .split_once(':')
            .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
                surface: ACTIVE_TRACE_DETAIL_ENV,
                detail: "expected one-based sim_day:lane selector, e.g. 792:1".to_string(),
            })?;
    let sim_day =
        day.parse::<usize>()
            .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
                surface: ACTIVE_TRACE_DETAIL_ENV,
                detail: format!("expected positive integer sim_day: {error}"),
            })?;
    let lane_index =
        lane.parse::<usize>()
            .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
                surface: ACTIVE_TRACE_DETAIL_ENV,
                detail: format!("expected positive integer lane: {error}"),
            })?;
    if sim_day == 0 || lane_index == 0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: ACTIVE_TRACE_DETAIL_ENV,
            detail: "sim_day and lane are one-based and must be >= 1".to_string(),
        });
    }
    Ok(Some(
        openwepp_hillslope_orchestrator::DirectLanedActiveTraceDetailFilter {
            day_index: sim_day - 1,
            lane_index: lane_index - 1,
        },
    ))
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

pub(crate) fn max_dt_s_from_env() -> Result<f64, HillslopeCliError> {
    match std::env::var(ACTIVE_MAX_DT_ENV) {
        Ok(value) => max_dt_s_from_value(Some(&value)),
        Err(std::env::VarError::NotPresent) => max_dt_s_from_value(None),
        Err(std::env::VarError::NotUnicode(_)) => Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: ACTIVE_MAX_DT_ENV,
            detail: "expected UTF-8 finite positive max dt in seconds".to_string(),
        }),
    }
}

fn max_dt_s_from_value(value: Option<&str>) -> Result<f64, HillslopeCliError> {
    let Some(value) = value else {
        return Ok(openwepp_hillslope_orchestrator::LANED_ACTIVE_MAX_DT_S);
    };
    let max_dt_s =
        value
            .parse::<f64>()
            .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
                surface: ACTIVE_MAX_DT_ENV,
                detail: format!("expected finite positive max dt in seconds: {error}"),
            })?;
    if !max_dt_s.is_finite()
        || max_dt_s <= 0.0
        || max_dt_s > openwepp_hillslope_orchestrator::LANED_ACTIVE_MAX_DT_S
    {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: ACTIVE_MAX_DT_ENV,
            detail: format!(
                "expected finite positive max dt in (0, {}] seconds, got {max_dt_s}",
                openwepp_hillslope_orchestrator::LANED_ACTIVE_MAX_DT_S
            ),
        });
    }
    Ok(max_dt_s)
}

#[cfg(test)]
mod tests {
    use super::{
        ActiveTraceSelectorState, max_dt_s_from_value, mesh_policy_from_target_dx_value,
        step_trace_enabled_from_value, trace_detail_filter_from_value, trace_enabled_from_value,
        validate_trace_selector_combination,
    };
    use openwepp_hillslope_orchestrator::DirectLanedActiveMeshPolicy;

    #[test]
    fn mesh_policy_parser_defaults_parses_and_rejects_invalid_target_dx() {
        assert_eq!(
            mesh_policy_from_target_dx_value(None).expect("default policy"),
            DirectLanedActiveMeshPolicy::production_default()
        );
        assert_eq!(
            DirectLanedActiveMeshPolicy::production_default()
                .cell_count_for_length_m(300.0)
                .expect("production dx5"),
            60
        );
        assert_eq!(
            mesh_policy_from_target_dx_value(Some("20.0")).expect("target dx policy"),
            DirectLanedActiveMeshPolicy::diagnostic_target_dx(20.0).expect("target dx")
        );
        assert!(mesh_policy_from_target_dx_value(Some("0")).is_err());
        assert!(mesh_policy_from_target_dx_value(Some("not-a-number")).is_err());
    }

    #[test]
    fn max_dt_parser_defaults_parses_and_rejects_invalid_caps() {
        assert!((max_dt_s_from_value(None).expect("default max dt") - 300.0).abs() < f64::EPSILON);
        assert!(
            (max_dt_s_from_value(Some("150")).expect("valid max dt") - 150.0).abs() < f64::EPSILON
        );
        assert!(max_dt_s_from_value(Some("0")).is_err());
        assert!(max_dt_s_from_value(Some("301")).is_err());
        assert!(max_dt_s_from_value(Some("NaN")).is_err());
        assert!(max_dt_s_from_value(Some("not-a-number")).is_err());
    }

    #[test]
    fn trace_selector_requires_explicit_one() {
        assert!(!trace_enabled_from_value(None));
        assert!(!trace_enabled_from_value(Some("0")));
        assert!(trace_enabled_from_value(Some("1")));
    }

    #[test]
    fn step_trace_selector_requires_explicit_one() {
        assert!(!step_trace_enabled_from_value(None));
        assert!(!step_trace_enabled_from_value(Some("0")));
        assert!(step_trace_enabled_from_value(Some("1")));
    }

    #[test]
    fn diagnostic_max_dt_selector_requires_active_trace() {
        assert!(
            validate_trace_selector_combination(
                ActiveTraceSelectorState::default()
                    .with_active()
                    .with_trace()
                    .with_max_dt(),
            )
            .is_ok()
        );
        assert!(
            validate_trace_selector_combination(
                ActiveTraceSelectorState::default()
                    .with_trace()
                    .with_max_dt(),
            )
            .is_err()
        );
        assert!(
            validate_trace_selector_combination(
                ActiveTraceSelectorState::default()
                    .with_active()
                    .with_max_dt(),
            )
            .is_err()
        );
        assert!(
            validate_trace_selector_combination(
                ActiveTraceSelectorState::default()
                    .with_active()
                    .with_trace()
                    .with_trace_detail()
                    .with_step_trace()
                    .with_max_dt(),
            )
            .is_ok()
        );
    }

    #[test]
    fn trace_detail_filter_parses_one_based_day_lane() {
        assert_eq!(trace_detail_filter_from_value(None).expect("none"), None);
        let filter = trace_detail_filter_from_value(Some("792:1"))
            .expect("detail selector")
            .expect("filter");
        assert_eq!(filter.day_index, 791);
        assert_eq!(filter.lane_index, 0);
        assert!(trace_detail_filter_from_value(Some("0:1")).is_err());
        assert!(trace_detail_filter_from_value(Some("792")).is_err());
    }
}
