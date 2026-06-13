#[allow(clippy::wildcard_imports)]
use super::super::*;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StaticOfeLaneSlice {
    pub(crate) ofe_id: usize,
    pub(crate) slope_ofe_index: usize,
    pub(crate) soil_ofe_index: usize,
    pub(crate) management_ofe_index: usize,
    pub(crate) width_m: f64,
    pub(crate) length_m: f64,
    pub(crate) area_m2: f64,
}

pub(crate) fn build_static_per_ofe_lane_slices(
    slope: &SlopeProfile,
    soil: &openwepp_input_contract::parsers::soil::SoilProfile,
    management_topology_count: usize,
) -> Result<Vec<StaticOfeLaneSlice>, HillslopeCliError> {
    validate_hillslope_ofe_topology_parity(slope.ofe_count, management_topology_count, soil.ntemp)?;

    if slope.ofes.len() != slope.ofe_count {
        return Err(per_ofe_state_failure(format!(
            "slope OFE vector length {} does not match declared ofe_count {}",
            slope.ofes.len(),
            slope.ofe_count
        )));
    }
    if soil.ofes.len() != slope.ofe_count {
        return Err(per_ofe_state_failure(format!(
            "soil OFE vector length {} does not match slope ofe_count {}",
            soil.ofes.len(),
            slope.ofe_count
        )));
    }

    let mut seen_ofe_ids = std::collections::BTreeSet::new();
    let mut slices = Vec::with_capacity(slope.ofe_count);
    for (position, (slope_ofe, _soil_ofe)) in slope.ofes.iter().zip(&soil.ofes).enumerate() {
        let ofe_id = position + 1;
        if !seen_ofe_ids.insert(ofe_id) {
            return Err(per_ofe_state_failure(format!(
                "duplicate static OFE lane id {ofe_id}"
            )));
        }
        if !slope_ofe.fwidth.is_finite() || slope_ofe.fwidth <= 0.0 {
            return Err(per_ofe_state_failure(format!(
                "OFE {ofe_id} fwidth must be finite and > 0.0, observed {}",
                slope_ofe.fwidth
            )));
        }
        if !slope_ofe.slplen.is_finite() || slope_ofe.slplen <= 0.0 {
            return Err(per_ofe_state_failure(format!(
                "OFE {ofe_id} slplen must be finite and > 0.0, observed {}",
                slope_ofe.slplen
            )));
        }

        slices.push(StaticOfeLaneSlice {
            ofe_id,
            slope_ofe_index: slope_ofe.index,
            soil_ofe_index: position,
            management_ofe_index: position,
            width_m: slope_ofe.fwidth,
            length_m: slope_ofe.slplen,
            area_m2: slope_ofe.fwidth * slope_ofe.slplen,
        });
    }

    Ok(slices)
}

fn per_ofe_state_failure(detail: impl Into<String>) -> HillslopeCliError {
    HillslopeCliError::RuntimeSurfaceFailure {
        surface: "per_ofe_static_lane_slices",
        detail: format!("{SIMPIPE_GUARD_ID} {}", detail.into()),
    }
}

pub(crate) fn validate_hillslope_ofe_topology_parity(
    slope_ofe_count: usize,
    management_topology_count: usize,
    soil_topology_count: usize,
) -> Result<(), HillslopeCliError> {
    if slope_ofe_count == management_topology_count && slope_ofe_count == soil_topology_count {
        return Ok(());
    }

    Err(HillslopeCliError::OfeTopologyMismatch {
        slope_ofe_count,
        management_topology_count,
        soil_topology_count,
    })
}

pub(crate) fn build_mode_selection_provenance(
    wepp_ui: &WeppUiParseResult,
) -> Result<HillslopeModeSelectionProvenance, HillslopeCliError> {
    if !matches!(wepp_ui.ui_run_requested, 0 | 1) {
        return Err(mode_selection_failure(format!(
            "requested ui_run must be in {{0,1}}, observed {}",
            wepp_ui.ui_run_requested
        )));
    }
    if !matches!(wepp_ui.ui_run, 0 | 1) {
        return Err(mode_selection_failure(format!(
            "effective ui_run must be in {{0,1}}, observed {}",
            wepp_ui.ui_run
        )));
    }

    let expected_divergence = wepp_ui.ui_run_requested != wepp_ui.ui_run;
    if wepp_ui.mode_divergence != expected_divergence {
        return Err(mode_selection_failure(format!(
            "mode_divergence mismatch: expected {} from requested/effective tuple ({}, {}), observed {}",
            expected_divergence, wepp_ui.ui_run_requested, wepp_ui.ui_run, wepp_ui.mode_divergence
        )));
    }

    let selected_lane = lane_name_from_effective_ui_run(wepp_ui.ui_run)?;

    Ok(HillslopeModeSelectionProvenance {
        wepp_ui: WeppUiModeSelectionProvenance {
            requested: wepp_ui.ui_run_requested,
            effective: wepp_ui.ui_run,
            selected_lane: selected_lane.to_string(),
            mode_divergence: wepp_ui.mode_divergence,
            guard_id: WUI_MODE_GUARD_ID.to_string(),
        },
    })
}

pub(crate) fn lane_name_from_effective_ui_run(
    effective_ui_run: i32,
) -> Result<&'static str, HillslopeCliError> {
    match effective_ui_run {
        0 => Ok(DAILY_EXECUTION_LANE),
        1 => Ok(HOURLY_EXECUTION_LANE),
        _ => Err(mode_selection_failure(format!(
            "effective ui_run must map to daily/hourly lane, observed {effective_ui_run}"
        ))),
    }
}

pub(crate) fn mode_name_from_ui_run(ui_run: i32) -> Result<&'static str, HillslopeCliError> {
    match ui_run {
        0 => Ok(DAILY_EXECUTION_LANE),
        1 => Ok(HOURLY_EXECUTION_LANE),
        _ => Err(timestep_policy_failure(format!(
            "ui_run must map to daily/hourly mode, observed {ui_run}"
        ))),
    }
}

pub(crate) fn build_execution_lane_context(
    mode_selection: &HillslopeModeSelectionProvenance,
) -> Result<ExecutionLaneContext, HillslopeCliError> {
    let requested_mode = mode_name_from_ui_run(mode_selection.wepp_ui.requested)?;
    let effective_mode = mode_name_from_ui_run(mode_selection.wepp_ui.effective)?;
    let lane = ExecutionLane::parse(mode_selection.wepp_ui.selected_lane.as_str())?;
    if lane.as_str() != effective_mode {
        return Err(timestep_policy_failure(format!(
            "selected lane '{}' must match effective mode '{effective_mode}'",
            lane.as_str()
        )));
    }

    Ok(ExecutionLaneContext {
        lane,
        requested_mode,
        effective_mode,
        timestep_policy: TimestepPolicy::from_lane(lane),
    })
}

pub(crate) fn build_timestep_policy_provenance(
    lane_context: &ExecutionLaneContext,
) -> HillslopeTimestepPolicyProvenance {
    let subhourly_scaffold = TimestepPolicy::scaffold_subhourly(900);
    HillslopeTimestepPolicyProvenance {
        scheduler_mode: lane_context.timestep_policy.scheduler_mode().to_string(),
        requested_mode: lane_context.requested_mode.to_string(),
        effective_mode: lane_context.effective_mode.to_string(),
        selected_lane: lane_context.lane.as_str().to_string(),
        policy: lane_context.timestep_policy.policy_name().to_string(),
        timestep_seconds: lane_context.timestep_policy.timestep_seconds(),
        physics_enabled: lane_context.timestep_policy.physics_enabled(),
        subhourly_scaffold_available: !subhourly_scaffold.physics_enabled(),
        guard_id: SIMMODE_TIMESTEP_GUARD_ID.to_string(),
    }
}

pub(crate) fn build_adapter_boundary_provenance(
    lane_context: &ExecutionLaneContext,
) -> Result<HillslopeAdapterBoundaryProvenance, HillslopeCliError> {
    let reject_surfaces_excluded = true;
    let defer_surfaces_excluded = true;
    if !reject_surfaces_excluded || !defer_surfaces_excluded {
        return Err(simcons_intake_failure(
            "SIMIMPL09 requires reject/defer intake surfaces to remain excluded",
        ));
    }

    Ok(HillslopeAdapterBoundaryProvenance {
        selected_lane: lane_context.lane.as_str().to_string(),
        scheduler_mode: lane_context.timestep_policy.scheduler_mode().to_string(),
        requested_mode: lane_context.requested_mode.to_string(),
        effective_mode: lane_context.effective_mode.to_string(),
        adopt_profile: SIMIMPL09_ADOPT_PROFILE.to_string(),
        reject_surfaces_excluded,
        defer_surfaces_excluded,
        guard_id: SIMCONS_INTAKE_GUARD_ID.to_string(),
    })
}
