//! MOFEFID Lane D / D15A (`SC-OFEROUTE-001` rev 27,
//! `INV-OFEROUTE-009/010/012`): the opt-in ACTIVE production owner for
//! OFE-by-OFE surface-water routing.
//!
//! With `OPENWEPP_LANED_ACTIVE=1` the publication-stream executor runs the
//! two-phase active day loop: all lanes' hydrology first (with DC01 SURFACE
//! runon admission disabled — the LATERAL admission is unchanged, the router
//! supersedes surface runon only), then per-lane routing → erosion → ledger
//! in cascade order over the shared day window. This module owns the
//! per-lane routing step, the day-closure hard-fails (rev-27 tolerance
//! notes), and the run-level evidence summary. Default/off takes none of
//! these paths (`INV-OFEROUTE-010`).

use super::{
    DirectDayFrame, DirectErosionHydrographShapeAuthority, DirectRuntimeError, validate_finite,
    validate_nonnegative_direct_m,
};
use crate::constants::WB11_ZERO_THRESHOLD;
use crate::ofe_routing::cascade::{
    CascadeSegment, UpstreamHandoff, route_single_ofe_with_step_trace,
};
use crate::ofe_routing::kinematic_wave::{CellParameters, KinematicWaveMesh, RoutingResult};
use crate::ofe_routing::kinematic_wave::{
    KinematicWaveStageLimiterTrace, KinematicWaveStepTraceRecord, KinematicWaveTvdTrace,
};
use crate::ofe_routing::seam::{
    SEAM_HOUR_BINS, SEAM_SECONDS_PER_HOUR, seam_rate_at, seam_source_rates_from_hourly_depths,
};

/// Production-default cells per OFE for the active mesh.
pub(crate) const LANED_ACTIVE_DEFAULT_CELLS: usize = 10;
/// Rev-38 scheme-regime floor for target-`dx` diagnostics.
pub(crate) const LANED_ACTIVE_MESH_MIN_CELLS: usize = 10;
/// Rev-38 fail-closed safety cap for target-`dx` diagnostics.
pub(crate) const LANED_ACTIVE_MESH_MAX_CELLS: usize = 4096;
/// Outlet-bin sample width (s); 3600/900 = 4 bins per hour, so the erosion
/// hourly mapping is exact hour-aligned bin sums.
pub(crate) const LANED_ACTIVE_SAMPLE_DT_S: f64 = 900.0;
/// Production CFL step cap (s).
pub const LANED_ACTIVE_MAX_DT_S: f64 = 300.0;
/// One day of hourly source (s).
pub(crate) const LANED_ACTIVE_SOURCE_WINDOW_S: f64 = 24.0 * 3600.0;
/// Drain tail after the last active source hour (rev-27 window row).
pub(crate) const LANED_ACTIVE_DRAIN_TAIL_S: f64 = 6.0 * 3600.0;
/// Rev-27 tolerance: per-lane-day supply reconstruction (relative).
pub(crate) const LANED_ACTIVE_SUPPLY_REL_TOL: f64 = 1.0e-9;
/// Rev-27 tolerance: per-day clamp-adjusted cascade residual (relative).
pub(crate) const LANED_ACTIVE_CASCADE_REL_TOL: f64 = 1.0e-9;
/// Rev-40 active publication guard: positivity-clamp injection may close the
/// numerical ledger but may not exceed the external source mass routed that day.
pub(crate) const LANED_ACTIVE_CLAMP_INPUT_REL_CAP: f64 = 1.0;
/// Rev-27 tolerance: per-day soil-to-router SEAM cross-ledger residual
/// (relative) — the router's booked injection vs the soil books' released
/// runoff volume. The two quantities come from INDEPENDENT ledgers (the
/// solver's own mass booking vs `q_runoff × area` from the R4A/R4B soil
/// surfaces), so this check cannot be satisfied by producer
/// self-consistency; it is the check class that caught the day-one
/// mesh-basis defect, made exact by the hourly forcing breakpoints.
pub(crate) const LANED_ACTIVE_SEAM_REL_TOL: f64 = 1.0e-9;
/// Rev-27 tolerance: assembled hillslope-day identity (relative to the max
/// operand magnitude).
pub(crate) const LANED_ACTIVE_IDENTITY_REL_TOL: f64 = 1.0e-6;

/// Per-lane static configuration for the active owner. Sources
/// (rev 20/21/36, same as the shadow): native management
/// `routing_coefficients` for the friction statics and the Wave-1 operand
/// seed for geometry. Daily `LAI`/`canhgt` are consumed from the post-growth
/// day frame at route time.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectLanedActiveLaneConfig {
    pub slplen_m: f64,
    pub width_m: f64,
    pub mean_gradient: f64,
    pub skin_friction_coefficient_ko: f64,
    pub form_drag_coefficient: f64,
    pub roughness_element_height_m: f64,
    pub roughness_concentration: f64,
    pub vegetation_drag_coefficient: f64,
    /// Static typed-management `canhgt` seed retained for validation and
    /// compatibility surfaces; active friction uses post-growth day-frame
    /// canopy height.
    pub canopy_height_m: Option<f64>,
}

impl DirectLanedActiveLaneConfig {
    fn validate(&self) -> Result<(), DirectRuntimeError> {
        for (field, value) in [
            ("laned_active.config.slplen_m", self.slplen_m),
            ("laned_active.config.width_m", self.width_m),
            ("laned_active.config.mean_gradient", self.mean_gradient),
            (
                "laned_active.config.skin_friction_coefficient_ko",
                self.skin_friction_coefficient_ko,
            ),
            (
                "laned_active.config.form_drag_coefficient",
                self.form_drag_coefficient,
            ),
            (
                "laned_active.config.roughness_element_height_m",
                self.roughness_element_height_m,
            ),
            (
                "laned_active.config.roughness_concentration",
                self.roughness_concentration,
            ),
            (
                "laned_active.config.vegetation_drag_coefficient",
                self.vegetation_drag_coefficient,
            ),
        ] {
            validate_finite(field, value)?;
            validate_nonnegative_direct_m(field, value)?;
        }
        if self.slplen_m <= 0.0 || self.width_m <= 0.0 || self.mean_gradient <= 0.0 {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "laned_active.config.geometry",
            });
        }
        if let Some(canopy_height_m) = self.canopy_height_m {
            validate_finite("laned_active.config.canopy_height_m", canopy_height_m)?;
            validate_nonnegative_direct_m("laned_active.config.canopy_height_m", canopy_height_m)?;
        }
        Ok(())
    }
}

/// Rev-38 active mesh policy. The production default is fixed-cell; target
/// `dx` is diagnostic evidence-gathering only until T2R ratifies a production
/// policy.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DirectLanedActiveMeshPolicy {
    FixedCells {
        cells: usize,
    },
    TargetDx {
        target_dx_m: f64,
        min_cells: usize,
        max_cells: usize,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectLanedActiveMeshPolicySummary {
    pub mode: &'static str,
    pub fixed_cells: Option<usize>,
    pub target_dx_m: Option<f64>,
    pub min_cells: usize,
    pub max_cells: usize,
}

impl Default for DirectLanedActiveMeshPolicySummary {
    fn default() -> Self {
        Self {
            mode: "fixed_cells",
            fixed_cells: Some(LANED_ACTIVE_DEFAULT_CELLS),
            target_dx_m: None,
            min_cells: LANED_ACTIVE_MESH_MIN_CELLS,
            max_cells: LANED_ACTIVE_MESH_MAX_CELLS,
        }
    }
}

impl Default for DirectLanedActiveMeshPolicy {
    fn default() -> Self {
        Self::production_default()
    }
}

impl DirectLanedActiveMeshPolicy {
    #[must_use]
    pub const fn production_default() -> Self {
        Self::FixedCells {
            cells: LANED_ACTIVE_DEFAULT_CELLS,
        }
    }

    pub fn diagnostic_target_dx(target_dx_m: f64) -> Result<Self, DirectRuntimeError> {
        let policy = Self::TargetDx {
            target_dx_m,
            min_cells: LANED_ACTIVE_MESH_MIN_CELLS,
            max_cells: LANED_ACTIVE_MESH_MAX_CELLS,
        };
        policy.validate()?;
        Ok(policy)
    }

    pub fn validate(&self) -> Result<(), DirectRuntimeError> {
        match *self {
            Self::FixedCells { cells } => {
                if !(LANED_ACTIVE_MESH_MIN_CELLS..=LANED_ACTIVE_MESH_MAX_CELLS).contains(&cells) {
                    return Err(DirectRuntimeError::DirectDomainViolation {
                        field: "laned_active.mesh_policy.fixed_cells",
                    });
                }
            }
            Self::TargetDx {
                target_dx_m,
                min_cells,
                max_cells,
            } => {
                validate_finite("laned_active.mesh_policy.target_dx_m", target_dx_m)?;
                if target_dx_m <= 0.0 {
                    return Err(DirectRuntimeError::DirectDomainViolation {
                        field: "laned_active.mesh_policy.target_dx_m",
                    });
                }
                if min_cells != LANED_ACTIVE_MESH_MIN_CELLS
                    || max_cells != LANED_ACTIVE_MESH_MAX_CELLS
                    || min_cells == 0
                    || min_cells > max_cells
                {
                    return Err(DirectRuntimeError::DirectDomainViolation {
                        field: "laned_active.mesh_policy.cell_bounds",
                    });
                }
            }
        }
        Ok(())
    }

    pub fn cell_count_for_length_m(&self, slplen_m: f64) -> Result<usize, DirectRuntimeError> {
        validate_finite("laned_active.mesh_policy.slplen_m", slplen_m)?;
        if slplen_m <= 0.0 {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "laned_active.mesh_policy.slplen_m",
            });
        }
        self.validate()?;
        match *self {
            Self::FixedCells { cells } => Ok(cells),
            Self::TargetDx {
                target_dx_m,
                min_cells,
                max_cells,
            } => {
                let raw_cells = (slplen_m / target_dx_m).ceil();
                let cap_count_u32 = u32::try_from(max_cells).map_err(|_| {
                    DirectRuntimeError::DirectDomainViolation {
                        field: "laned_active.mesh_policy.max_cells",
                    }
                })?;
                let cap_count_magnitude = f64::from(cap_count_u32);
                if !raw_cells.is_finite() || raw_cells < 1.0 || raw_cells > cap_count_magnitude {
                    return Err(DirectRuntimeError::DirectKernelGuardFailure {
                        phase: "laned_active_mesh_policy",
                        detail: format!(
                            "target dx {target_dx_m} m over slope length {slplen_m} m requires {raw_cells} cells, outside max {max_cells}"
                        ),
                    });
                }
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let cells = raw_cells as usize;
                Ok(cells.max(min_cells))
            }
        }
    }

    #[must_use]
    pub fn summary(&self) -> DirectLanedActiveMeshPolicySummary {
        match *self {
            Self::FixedCells { cells } => DirectLanedActiveMeshPolicySummary {
                mode: "fixed_cells",
                fixed_cells: Some(cells),
                target_dx_m: None,
                min_cells: LANED_ACTIVE_MESH_MIN_CELLS,
                max_cells: LANED_ACTIVE_MESH_MAX_CELLS,
            },
            Self::TargetDx {
                target_dx_m,
                min_cells,
                max_cells,
            } => DirectLanedActiveMeshPolicySummary {
                mode: "target_dx",
                fixed_cells: None,
                target_dx_m: Some(target_dx_m),
                min_cells,
                max_cells,
            },
        }
    }
}

/// Run-level active configuration, attached to the run frame by the runner
/// when `OPENWEPP_LANED_ACTIVE=1`. Its presence IS the activation selector
/// inside the orchestrator.
#[derive(Debug, Clone, PartialEq)]
pub struct DirectLanedActiveConfig {
    pub lanes: Vec<DirectLanedActiveLaneConfig>,
    pub mesh_policy: DirectLanedActiveMeshPolicy,
    pub max_dt_s: f64,
    pub trace_enabled: bool,
    pub trace_detail_filter: Option<DirectLanedActiveTraceDetailFilter>,
    pub step_trace_enabled: bool,
}

impl DirectLanedActiveConfig {
    pub fn validate(&self, lane_count: usize) -> Result<(), DirectRuntimeError> {
        if self.lanes.len() != lane_count {
            return Err(DirectRuntimeError::FrameLaneCountMismatch {
                identity_lane_count: lane_count,
                actual_lane_count: self.lanes.len(),
            });
        }
        for lane in &self.lanes {
            lane.validate()?;
        }
        self.mesh_policy.validate()?;
        validate_finite("laned_active.max_dt_s", self.max_dt_s)?;
        if self.max_dt_s <= 0.0 || self.max_dt_s > LANED_ACTIVE_MAX_DT_S {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "laned_active.max_dt_s",
            });
        }
        if self.trace_detail_filter.is_some() && !self.trace_enabled {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "laned_active.trace_detail_filter",
            });
        }
        if self.step_trace_enabled && (self.trace_detail_filter.is_none() || !self.trace_enabled) {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "laned_active.step_trace_enabled",
            });
        }
        Ok(())
    }
}

/// Optional diagnostic trace detail target. Indices are zero-based inside the
/// orchestrator; runner env parsing accepts one-based `sim_day:lane`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectLanedActiveTraceDetailFilter {
    pub day_index: usize,
    pub lane_index: usize,
}

impl DirectLanedActiveTraceDetailFilter {
    #[must_use]
    pub const fn matches(self, day_index: usize, lane_index: usize) -> bool {
        self.day_index == day_index && self.lane_index == lane_index
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectLanedActiveTraceDetail {
    pub mesh_cell_count: usize,
    pub mesh_dx_m: f64,
    pub max_dt_s: f64,
    pub outlet_bin_m3: Vec<f64>,
    pub outlet_bin_spans_s: Vec<f64>,
    pub hydrograph_time_s: Vec<f64>,
    pub hydrograph_outlet_m3_s: Vec<f64>,
    pub hydrograph_outlet_depth_m: Vec<f64>,
    pub step_trace: Option<Vec<DirectLanedActiveStepTraceRecord>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectLanedActiveStageLimiterTrace {
    pub reductions: u32,
    pub max_reduction_m3_s: f64,
    pub face_index: usize,
    pub face_x_m: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectLanedActiveTvdTrace {
    pub scale: f64,
    pub max_abs_delta_m: f64,
    pub cell_index: usize,
    pub cell_center_x_m: f64,
    pub signed_delta_m: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectLanedActiveStepTraceRecord {
    pub step_index: u64,
    pub t_start_s: f64,
    pub t_end_s: f64,
    pub dt_s: f64,
    pub max_courant: f64,
    pub max_courant_cell_index: usize,
    pub max_courant_cell_center_x_m: f64,
    pub q_up_m3_s: f64,
    pub source_m3: f64,
    pub upstream_inflow_m3: f64,
    pub outflow_m3: f64,
    pub storage_before_m3: f64,
    pub storage_after_m3: f64,
    pub clamp_injected_m3: f64,
    pub pred_out_face_m3_s: f64,
    pub corr_out_face_m3_s: f64,
    pub outlet_depth_m: f64,
    pub outlet_discharge_m3_s: f64,
    pub predictor_limiter: DirectLanedActiveStageLimiterTrace,
    pub corrector_limiter: DirectLanedActiveStageLimiterTrace,
    pub tvd: DirectLanedActiveTvdTrace,
}

/// Per-lane-day routed evidence, stored on the day frame so the runner's
/// row consumer can fold the manifest `laned_active` block.
#[derive(Debug, Clone, PartialEq)]
pub struct DirectLanedActiveDayRouting {
    pub source_m3: f64,
    pub outlet_m3: f64,
    pub mesh_end_storage_m3: f64,
    pub clamp_m3: f64,
    pub tail_fold_m3: f64,
    pub routed_weights: [f64; 24],
    pub uniform_shape: bool,
    pub erosion_source_shape_degenerate: bool,
    pub trace_detail: Option<Box<DirectLanedActiveTraceDetail>>,
}

/// Run-level evidence summary accumulated by the executor and surfaced by
/// the runner manifest.
#[derive(Debug, Clone, PartialEq)]
pub struct DirectLanedActiveRunSummary {
    pub mesh_policy: DirectLanedActiveMeshPolicySummary,
    pub max_dt_s: f64,
    pub days_seen: u64,
    pub days_routed: u64,
    pub total_source_m3: f64,
    pub total_routed_outlet_m3: f64,
    pub total_end_window_storage_m3: f64,
    pub total_clamp_m3: f64,
    pub total_tail_fold_m3: f64,
    pub total_latqcc_outlet_m3: f64,
    pub max_supply_reconstruction_rel: f64,
    pub max_day_cascade_residual_rel: f64,
    pub max_day_seam_residual_rel: f64,
    pub max_day_identity_residual_rel: f64,
    pub days_uniform_shape: u64,
    /// Rev-27 full-mesh-hold degeneracy class: lane-days whose erosion shape
    /// degenerated to the normalized routed source series (zero outlet mass
    /// above the wet-gate).
    pub lane_days_erosion_source_shape_degenerate: u64,
    /// Diagnostic-only lane-day trace rows. `None` is the production default
    /// and preserves the active path's normal memory/output posture.
    pub trace_records: Option<Vec<DirectLanedActiveTraceRecord>>,
}

/// Rev-38 diagnostic mesh-adjudication row. The runner serializes these rows
/// only under the explicit `OPENWEPP_LANED_ACTIVE_TRACE=1` evidence opt-in.
#[derive(Debug, Clone, PartialEq)]
pub struct DirectLanedActiveTraceRecord {
    pub day_index: usize,
    pub lane_index: usize,
    pub max_dt_s: f64,
    pub is_terminal_lane: bool,
    pub source_m3: f64,
    pub outlet_m3: f64,
    pub terminal_day_outlet_m3: Option<f64>,
    pub mesh_end_storage_m3: f64,
    pub clamp_m3: f64,
    pub tail_fold_m3: f64,
    pub routed_weights: [f64; 24],
    pub uniform_shape: bool,
    pub erosion_source_shape_degenerate: bool,
    pub trace_detail: Option<Box<DirectLanedActiveTraceDetail>>,
}

impl Default for DirectLanedActiveRunSummary {
    fn default() -> Self {
        Self {
            mesh_policy: DirectLanedActiveMeshPolicySummary::default(),
            max_dt_s: LANED_ACTIVE_MAX_DT_S,
            days_seen: 0,
            days_routed: 0,
            total_source_m3: 0.0,
            total_routed_outlet_m3: 0.0,
            total_end_window_storage_m3: 0.0,
            total_clamp_m3: 0.0,
            total_tail_fold_m3: 0.0,
            total_latqcc_outlet_m3: 0.0,
            max_supply_reconstruction_rel: 0.0,
            max_day_cascade_residual_rel: 0.0,
            max_day_seam_residual_rel: 0.0,
            max_day_identity_residual_rel: 0.0,
            days_uniform_shape: 0,
            lane_days_erosion_source_shape_degenerate: 0,
            trace_records: None,
        }
    }
}

impl DirectLanedActiveRunSummary {
    #[must_use]
    pub fn for_mesh_policy(
        mesh_policy: DirectLanedActiveMeshPolicy,
        max_dt_s: f64,
        trace_enabled: bool,
    ) -> Self {
        Self {
            mesh_policy: mesh_policy.summary(),
            max_dt_s,
            trace_records: trace_enabled.then(Vec::new),
            ..Self::default()
        }
    }
}

pub(crate) fn laned_active_record_trace(
    summary: &mut DirectLanedActiveRunSummary,
    day_frame: &DirectDayFrame,
    is_terminal_lane: bool,
    terminal_day_outlet_m3: f64,
) -> Result<(), DirectRuntimeError> {
    let Some(trace_records) = summary.trace_records.as_mut() else {
        return Ok(());
    };
    let routing = day_frame.laned_active_routing.as_ref().ok_or(
        DirectRuntimeError::DirectKernelGuardFailure {
            phase: "laned_active_trace",
            detail: format!(
                "trace enabled but lane {} day {} has no active routing record",
                day_frame.lane_index + 1,
                day_frame.day_index + 1
            ),
        },
    )?;
    trace_records.push(DirectLanedActiveTraceRecord {
        day_index: day_frame.day_index,
        lane_index: day_frame.lane_index,
        max_dt_s: summary.max_dt_s,
        is_terminal_lane,
        source_m3: routing.source_m3,
        outlet_m3: routing.outlet_m3,
        terminal_day_outlet_m3: is_terminal_lane.then_some(terminal_day_outlet_m3),
        mesh_end_storage_m3: routing.mesh_end_storage_m3,
        clamp_m3: routing.clamp_m3,
        tail_fold_m3: routing.tail_fold_m3,
        routed_weights: routing.routed_weights,
        uniform_shape: routing.uniform_shape,
        erosion_source_shape_degenerate: routing.erosion_source_shape_degenerate,
        trace_detail: routing.trace_detail.clone(),
    });
    Ok(())
}

/// Per-day closure books (rev-27 tolerance notes). `lane_net_m3` accumulates
/// each lane's R4B identity `Σ A·(IN − OUT − ΔS)` with `q_runoff` on the OUT
/// side; the router terms then replace `q_runoff` so the composed residual
/// isolates lateral telescoping and operand lineage.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct DirectLanedActiveDayBooks {
    pub injected_m3: f64,
    /// Soil-side released surface water, `Σ q_runoff × A_lane` (m³) — the
    /// INDEPENDENT ledger the seam check compares against the router's
    /// booked injection.
    pub soil_release_m3: f64,
    pub terminal_outlet_m3: f64,
    pub mesh_storage_m3: f64,
    pub clamp_m3: f64,
    pub lane_net_m3: f64,
    pub latqcc_outlet_m3: f64,
    pub max_abs_term_m3: f64,
    pub max_supply_reconstruction_rel: f64,
    pub tail_fold_m3: f64,
    pub uniform_shape_days: u64,
    pub erosion_source_shape_degenerate_lane_days: u64,
    pub routed: bool,
}

impl DirectLanedActiveDayBooks {
    fn note_term(&mut self, term_m3: f64) {
        let magnitude = term_m3.abs();
        if magnitude > self.max_abs_term_m3 {
            self.max_abs_term_m3 = magnitude;
        }
    }
}

/// The per-lane source decomposition for one active day.
pub(crate) struct LanedActiveLaneSource {
    pub depths_m: [f64; SEAM_HOUR_BINS],
    pub q_runoff_m: f64,
    pub uniform_shape: bool,
    pub supply_reconstruction_rel: f64,
}

/// Build the lane-day routed source series: the ADR-0036 weights-times-total
/// form over the three D12 limbs (`wb14_hourly_excess` + `ui_SCrunf`-lineage
/// carry + routed melt), consumed from the LIVE day frame. Fails closed on
/// the hourly-lane precondition (missing R4O projection) and on supply
/// non-reconstruction (rev-27 tolerance (a)).
pub(crate) fn laned_active_lane_source(
    day_frame: &DirectDayFrame,
) -> Result<LanedActiveLaneSource, DirectRuntimeError> {
    let runoff = day_frame.runoff_shadow_projection.as_ref().ok_or(
        DirectRuntimeError::MissingDirectUpstream {
            upstream: "laned_active R4A runoff partition producer",
        },
    )?;
    let subsurface = day_frame
        .subsurface_compute_shadow_projection
        .as_ref()
        .ok_or(DirectRuntimeError::MissingDirectUpstream {
            upstream: "laned_active R4O hourly carries (hourly lane required)",
        })?;
    let q_runoff_m = runoff.q_runoff_m;
    validate_finite("laned_active.q_runoff_m", q_runoff_m)?;
    validate_nonnegative_direct_m("laned_active.q_runoff_m", q_runoff_m)?;
    let weights = super::runoff::dc01_surface_runoff_hourly_weights(
        q_runoff_m,
        &day_frame.wb14_hourly_excess_m,
        &subsurface.hourly_saturation_carry_m,
        day_frame
            .snow_coupling_downstream_operands
            .hourly_routed_melt_m
            .as_ref(),
    )?;
    let mut depths_m = [0.0_f64; SEAM_HOUR_BINS];
    let mut uniform_shape = false;
    if q_runoff_m > 0.0 {
        let uniform = 1.0 / 24.0;
        uniform_shape = weights
            .iter()
            .all(|weight| (weight - uniform).abs() < 1.0e-12);
        for (depth, weight) in depths_m.iter_mut().zip(weights.iter()) {
            *depth = weight * q_runoff_m;
        }
    }
    let total: f64 = depths_m.iter().sum();
    let supply_reconstruction_rel = if q_runoff_m > WB11_ZERO_THRESHOLD {
        (total - q_runoff_m).abs() / q_runoff_m
    } else {
        0.0
    };
    if supply_reconstruction_rel > LANED_ACTIVE_SUPPLY_REL_TOL {
        return Err(DirectRuntimeError::DirectClosureToleranceExceeded {
            field: "laned_active.supply_reconstruction",
        });
    }
    Ok(LanedActiveLaneSource {
        depths_m,
        q_runoff_m,
        uniform_shape,
        supply_reconstruction_rel,
    })
}

/// INV-OFEROUTE-009 runtime guard: on an active lane the DC01 SURFACE runon
/// must be dead (the router owns it). The resolved R4J surface total is the
/// observable; the LATERAL carry (`subsurface_carry_m`) is legitimately
/// nonzero and not checked here.
pub(crate) fn laned_active_assert_no_dc01_surface_feed(
    day_frame: &DirectDayFrame,
) -> Result<(), DirectRuntimeError> {
    let surface_runon_m = day_frame.runon_carry_downstream_operands.runon_input_m;
    if surface_runon_m > WB11_ZERO_THRESHOLD {
        return Err(DirectRuntimeError::DirectKernelGuardFailure {
            phase: "laned_active_dc01_double_feed_guard",
            detail: format!(
                "lane {} day {} resolved DC01 surface runon {surface_runon_m} m on an active lane (INV-OFEROUTE-009: routed and daily-lump paths must never both feed one lane)",
                day_frame.lane_index + 1,
                day_frame.day_index + 1
            ),
        });
    }
    Ok(())
}

/// The rev-27 day window: `(last active source hour + 1) h + 6 h` drain
/// tail, capped at the 24 h source window before the tail.
#[must_use]
pub(crate) fn laned_active_window_s(last_active_hour: usize) -> f64 {
    #[allow(clippy::cast_precision_loss)]
    let active_end_s = ((last_active_hour + 1) as f64) * SEAM_SECONDS_PER_HOUR;
    active_end_s.min(LANED_ACTIVE_SOURCE_WINDOW_S) + LANED_ACTIVE_DRAIN_TAIL_S
}

/// Map the routed outlet bin series to the 24 erosion hourly weights
/// (rev-27 mapping): hour-aligned bin-mass sums; drain-tail mass (bins at or
/// beyond hour 24) FOLDS into hour 24; unit-normalized for positive-runoff
/// lanes and all-zero otherwise (the `SC-SED-001#INV-SED-013` dry rule).
/// Full-mesh-hold degeneracy (rev 27): a lane-day above the erosion wet-gate
/// whose mesh depths never crossed the solver's dry floor discharges ZERO
/// outlet mass — no unit-sum outlet shape exists, and the shape DEGENERATES
/// to the normalized routed SOURCE series (counted, never silent).
/// Returns `(weights, tail_fold_m3, source_shape_degenerate)`.
pub(crate) fn laned_active_routed_erosion_weights(
    result: &RoutingResult,
    width_m: f64,
    q_runoff_m: f64,
    routed_source_depths_m: &[f64; SEAM_HOUR_BINS],
) -> Result<([f64; 24], f64, bool), DirectRuntimeError> {
    let mut hour_mass_m3 = [0.0_f64; 24];
    let mut tail_fold_m3 = 0.0_f64;
    let bin_dt_s = result.outlet_bin_dt_s;
    for (k, bin_m2) in result.outlet_bin_outflow_m2.iter().enumerate() {
        let mass_m3 = bin_m2 * width_m;
        #[allow(clippy::cast_precision_loss)]
        let bin_start_s = k as f64 * bin_dt_s;
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let hour = (bin_start_s / SEAM_SECONDS_PER_HOUR) as usize;
        if hour >= 24 {
            hour_mass_m3[23] += mass_m3;
            tail_fold_m3 += mass_m3;
        } else {
            hour_mass_m3[hour] += mass_m3;
        }
    }
    let total_m3: f64 = hour_mass_m3.iter().sum();
    validate_finite("laned_active.routed_outlet_total_m3", total_m3)?;
    let mut weights = [0.0_f64; 24];
    let mut source_shape_degenerate = false;
    if q_runoff_m > WB11_ZERO_THRESHOLD {
        if total_m3 > 0.0 {
            for (weight, mass) in weights.iter_mut().zip(hour_mass_m3.iter()) {
                *weight = mass / total_m3;
            }
        } else {
            let source_total_m: f64 = routed_source_depths_m.iter().sum();
            if source_total_m <= 0.0 {
                // Above the wet-gate with no source at all: upstream state
                // corruption, fail closed.
                return Err(DirectRuntimeError::DirectKernelGuardFailure {
                    phase: "laned_active_routed_erosion_weights",
                    detail: format!(
                        "q_runoff {q_runoff_m} m above the wet-gate with zero routed outlet AND zero source series"
                    ),
                });
            }
            source_shape_degenerate = true;
            for (weight, depth) in weights.iter_mut().zip(routed_source_depths_m.iter()) {
                *weight = depth / source_total_m;
            }
        }
    }
    Ok((weights, tail_fold_m3, source_shape_degenerate))
}

fn laned_active_stage_trace_from_solver(
    trace: KinematicWaveStageLimiterTrace,
    width_m: f64,
) -> DirectLanedActiveStageLimiterTrace {
    DirectLanedActiveStageLimiterTrace {
        reductions: trace.reductions,
        max_reduction_m3_s: trace.max_reduction_m2_s * width_m,
        face_index: trace.face_index,
        face_x_m: trace.face_x_m,
    }
}

fn laned_active_tvd_trace_from_solver(trace: KinematicWaveTvdTrace) -> DirectLanedActiveTvdTrace {
    DirectLanedActiveTvdTrace {
        scale: trace.scale,
        max_abs_delta_m: trace.max_abs_delta_m,
        cell_index: trace.cell_index,
        cell_center_x_m: trace.cell_center_x_m,
        signed_delta_m: trace.signed_delta_m,
    }
}

fn laned_active_step_trace_from_solver(
    record: &KinematicWaveStepTraceRecord,
    width_m: f64,
) -> DirectLanedActiveStepTraceRecord {
    DirectLanedActiveStepTraceRecord {
        step_index: record.step_index,
        t_start_s: record.t_start_s,
        t_end_s: record.t_end_s,
        dt_s: record.dt_s,
        max_courant: record.max_courant,
        max_courant_cell_index: record.max_courant_cell_index,
        max_courant_cell_center_x_m: record.max_courant_cell_center_x_m,
        q_up_m3_s: record.q_up_m2_s * width_m,
        source_m3: record.source_m2 * width_m,
        upstream_inflow_m3: record.upstream_inflow_m2 * width_m,
        outflow_m3: record.outflow_m2 * width_m,
        storage_before_m3: record.storage_before_m2 * width_m,
        storage_after_m3: record.storage_after_m2 * width_m,
        clamp_injected_m3: record.clamp_injected_m2 * width_m,
        pred_out_face_m3_s: record.pred_out_face_m2_s * width_m,
        corr_out_face_m3_s: record.corr_out_face_m2_s * width_m,
        outlet_depth_m: record.outlet_depth_m,
        outlet_discharge_m3_s: record.outlet_unit_discharge_m2_s * width_m,
        predictor_limiter: laned_active_stage_trace_from_solver(record.predictor_limiter, width_m),
        corrector_limiter: laned_active_stage_trace_from_solver(record.corrector_limiter, width_m),
        tvd: laned_active_tvd_trace_from_solver(record.tvd),
    }
}

/// Route one active lane-day: rev-21 operand validation, the shared
/// single-OFE cascade routine, the D13 erosion producer flip, the day-frame
/// evidence record, and the day-books fold. Returns the conservative
/// upstream handoff for the next lane.
#[allow(clippy::too_many_lines, clippy::too_many_arguments)]
pub(crate) fn laned_active_route_lane(
    day_frame: &mut DirectDayFrame,
    lane_config: &DirectLanedActiveLaneConfig,
    mesh_policy: &DirectLanedActiveMeshPolicy,
    area_m2: f64,
    upstream: Option<&UpstreamHandoff>,
    window_s: f64,
    books: &mut DirectLanedActiveDayBooks,
    source: &LanedActiveLaneSource,
    max_dt_s: f64,
    trace_detail: bool,
    trace_steps: bool,
) -> Result<UpstreamHandoff, DirectRuntimeError> {
    validate_finite("laned_active.area_m2", area_m2)?;
    if area_m2 <= 0.0 {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "laned_active.area_m2",
        });
    }
    validate_finite("laned_active.max_dt_s", max_dt_s)?;
    if max_dt_s <= 0.0 || max_dt_s > LANED_ACTIVE_MAX_DT_S {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "laned_active.max_dt_s",
        });
    }
    laned_active_assert_no_dc01_surface_feed(day_frame)?;
    if source.supply_reconstruction_rel > books.max_supply_reconstruction_rel {
        books.max_supply_reconstruction_rel = source.supply_reconstruction_rel;
    }

    // Rev-21/36 dynamic operands from the live post-growth day frame (same
    // guards as the shadow builder, typed).
    let leaf_area_index = day_frame.evapotranspiration_compute_inputs.leaf_area_index;
    validate_finite("laned_active.leaf_area_index", leaf_area_index)?;
    validate_nonnegative_direct_m("laned_active.leaf_area_index", leaf_area_index)?;
    let canopy_height_m = day_frame.evapotranspiration_compute_inputs.canopy_height_m;
    validate_finite("laned_active.canopy_height_m", canopy_height_m)?;
    validate_nonnegative_direct_m("laned_active.canopy_height_m", canopy_height_m)?;
    let canopy_height_m = match canopy_height_m {
        value if value > 0.0 => value,
        _ if leaf_area_index > 0.0 => {
            return Err(DirectRuntimeError::DirectKernelGuardFailure {
                phase: "laned_active_rev21_operands",
                detail: format!(
                    "lane {} day {} has LAI {leaf_area_index} > 0 with missing/non-positive post-growth canhgt (rev-21/rev-36 fail-closed)",
                    day_frame.lane_index + 1,
                    day_frame.day_index + 1
                ),
            });
        }
        value => value,
    };
    for (hour, rainfall_m) in day_frame.wb14_hourly_rainfall_m.iter().enumerate() {
        if !rainfall_m.is_finite() || *rainfall_m < 0.0 {
            return Err(DirectRuntimeError::DirectKernelGuardFailure {
                phase: "laned_active_rev21_operands",
                detail: format!(
                    "lane {} day {} hourly rainfall slot {} invalid ({rainfall_m})",
                    day_frame.lane_index + 1,
                    day_frame.day_index + 1,
                    hour + 1
                ),
            });
        }
    }

    // Mesh: rev-20 static friction operands + rev-21 dynamic canopy state.
    let mut cell = CellParameters::bare(
        lane_config.mean_gradient,
        lane_config.skin_friction_coefficient_ko,
    );
    cell.drag_coefficient = lane_config.form_drag_coefficient;
    cell.element_tip_height_m = lane_config.roughness_element_height_m;
    cell.roughness_concentration = lane_config.roughness_concentration;
    cell.vegetation_drag_coefficient = lane_config.vegetation_drag_coefficient;
    cell.leaf_area_index = leaf_area_index;
    cell.canopy_height_m = canopy_height_m;
    let active_cells = mesh_policy.cell_count_for_length_m(lane_config.slplen_m)?;
    let segment = CascadeSegment {
        mesh: KinematicWaveMesh::uniform(lane_config.slplen_m, active_cells, cell),
        width_m: lane_config.width_m,
    };
    let mesh_dx_m = segment.mesh.cell_length_m;

    // Seam basis conversion (rev 27, recorded helper): the soil books
    // release `q_runoff × area` m³; the 1-D mesh's plan area is
    // `slplen × width`. Scale the depth series so the router receives
    // EXACTLY the soil-released volume on any geometry (factor 1.0 when the
    // lane area equals the mesh plan area).
    let mesh_plan_area_m2 = lane_config.slplen_m * lane_config.width_m;
    let basis_factor = area_m2 / mesh_plan_area_m2;
    if !basis_factor.is_finite() || basis_factor <= 0.0 {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "laned_active.mesh_plan_area_basis",
        });
    }
    let mut routed_depths_m = source.depths_m;
    for depth in &mut routed_depths_m {
        *depth *= basis_factor;
    }

    // Seam rate series (the recorded /3600 helpers).
    let source_rates = seam_source_rates_from_hourly_depths(&routed_depths_m).map_err(|error| {
        DirectRuntimeError::DirectKernelGuardFailure {
            phase: "laned_active_seam_rates",
            detail: format!("{error:?}"),
        }
    })?;
    let intensity_rates = seam_source_rates_from_hourly_depths(&day_frame.wb14_hourly_rainfall_m)
        .map_err(|error| DirectRuntimeError::DirectKernelGuardFailure {
        phase: "laned_active_seam_rates",
        detail: format!("{error:?}"),
    })?;
    let excess = |_cell: usize, t: f64| seam_rate_at(&source_rates, t);
    let intensity = |t: f64| seam_rate_at(&intensity_rates, t);

    // Rev-27 seam exactness: the source/intensity series are
    // piecewise-constant per hour, so the hourly boundaries are forcing
    // discontinuities and no solver step may straddle one (the D10B High-2
    // exact-source-history rule). With the breakpoints, the solver's booked
    // injection integrates the seam series exactly, which is what makes the
    // soil-to-router seam check a machine-exact cross-ledger identity.
    let mut breakpoints = [0.0_f64; SEAM_HOUR_BINS];
    let mut breakpoint_count = 0_usize;
    for hour in 1..=SEAM_HOUR_BINS {
        #[allow(clippy::cast_precision_loss)]
        let boundary_s = hour as f64 * SEAM_SECONDS_PER_HOUR;
        if boundary_s < window_s {
            breakpoints[breakpoint_count] = boundary_s;
            breakpoint_count += 1;
        }
    }
    let result = route_single_ofe_with_step_trace(
        &segment,
        &excess,
        &intensity,
        upstream,
        &breakpoints[..breakpoint_count],
        window_s,
        LANED_ACTIVE_SAMPLE_DT_S,
        max_dt_s,
        trace_steps,
    )
    .map_err(|error| DirectRuntimeError::DirectKernelGuardFailure {
        phase: "laned_active_cascade",
        detail: format!(
            "lane {} day {} routing failed: {error:?}",
            day_frame.lane_index + 1,
            day_frame.day_index + 1
        ),
    })?;

    // D13 producer flip: the routed outlet shape becomes the Wave-1 hourly
    // water-shape authority for this lane-day (fail-closed validation lives
    // in the erosion consumer, `r7d8_routed_hydrograph_hourly_weights`).
    let (routed_weights, tail_fold_m3, source_shape_degenerate) =
        laned_active_routed_erosion_weights(
            &result,
            lane_config.width_m,
            source.q_runoff_m,
            &routed_depths_m,
        )?;
    if source_shape_degenerate {
        books.erosion_source_shape_degenerate_lane_days += 1;
    }
    day_frame.erosion_inputs.hydrograph_shape_authority =
        DirectErosionHydrographShapeAuthority::RoutedHydrograph;
    day_frame.erosion_inputs.routed_hydrograph_runoff_fraction = Some(Box::new(routed_weights));

    // Books (m³): router terms + the lane's R4B identity terms. The router
    // books live on the mesh basis (`slplen × width`); the SOIL books live
    // on the lane-area basis. `source_m3` (evidence) is the area-basis
    // supply; `injected` books the SCHEME-ACTUAL mass so the day cascade
    // residual is the router's own exact identity, and the day-identity
    // check converts the soil `q_runoff` to the mesh basis at the seam so
    // the two ledgers compose without an area/width aliasing term.
    let width = lane_config.width_m;
    let source_m3 = source.depths_m.iter().sum::<f64>() * area_m2;
    let soil_release_m3 = source.q_runoff_m * area_m2;
    let injected_m3 = result.mass_balance.rainfall_excess_m2 * width;
    let outlet_m3 = result.mass_balance.outflow_m2 * width;
    let mesh_storage_m3 = result.mass_balance.storage_change_m2 * width;
    let clamp_m3 = result.mass_balance.positivity_clamp_m2 * width;
    let step_trace = result.step_trace.as_ref().map(|records| {
        records
            .iter()
            .map(|record| laned_active_step_trace_from_solver(record, width))
            .collect()
    });
    let trace_detail = trace_detail.then(|| {
        Box::new(DirectLanedActiveTraceDetail {
            mesh_cell_count: active_cells,
            mesh_dx_m,
            max_dt_s,
            outlet_bin_m3: result
                .outlet_bin_outflow_m2
                .iter()
                .map(|value| value * width)
                .collect(),
            outlet_bin_spans_s: result.outlet_bin_spans_s.clone(),
            hydrograph_time_s: result
                .hydrograph
                .iter()
                .map(|sample| sample.time_s)
                .collect(),
            hydrograph_outlet_m3_s: result
                .hydrograph
                .iter()
                .map(|sample| sample.outlet_unit_discharge_m2_s * width)
                .collect(),
            hydrograph_outlet_depth_m: result
                .hydrograph
                .iter()
                .map(|sample| sample.outlet_depth_m)
                .collect(),
            step_trace,
        })
    });
    books.injected_m3 += injected_m3;
    books.soil_release_m3 += soil_release_m3;
    books.mesh_storage_m3 += mesh_storage_m3;
    books.clamp_m3 += clamp_m3;
    books.tail_fold_m3 += tail_fold_m3;
    books.terminal_outlet_m3 = outlet_m3; // overwritten each lane; terminal survives
    books.routed = true;
    if source.uniform_shape {
        books.uniform_shape_days = 1;
    }
    books.note_term(source_m3);
    books.note_term(outlet_m3);
    books.note_term(mesh_storage_m3);
    books.note_term(clamp_m3);

    let storage = &day_frame.storage_downstream_operands;
    let lane_in_m3 = (storage.precip_input_m
        + storage.snow_coupling_m
        + storage.frost_liquid_delta_m
        + storage.runon_input_m
        + storage.evapotranspiration_storage_return_m)
        * area_m2;
    let lane_out_m3 = (storage.interception_m
        + storage.q_runoff_m
        + storage.evapotranspiration_m
        + storage.deep_seepage_m
        + storage.subsurface_loss_m)
        * area_m2;
    let lane_ds_m3 = (storage.storage_reconciled_m - storage.storage_initial_m) * area_m2;
    books.lane_net_m3 += lane_in_m3 - lane_out_m3 - lane_ds_m3;
    books.note_term(lane_in_m3);
    books.note_term(lane_out_m3);
    books.note_term(lane_ds_m3);
    // The outlet lane's lateral export is the INV-OFEROUTE-012 latqcc bypass
    // term; recorded every lane, the terminal value survives.
    books.latqcc_outlet_m3 = day_frame
        .subsurface_compute_shadow_projection
        .as_ref()
        .map_or(0.0, |subsurface| subsurface.lateral_flow_m * area_m2);

    day_frame.laned_active_routing = Some(Box::new(DirectLanedActiveDayRouting {
        source_m3,
        outlet_m3,
        mesh_end_storage_m3: mesh_storage_m3,
        clamp_m3,
        tail_fold_m3,
        routed_weights,
        uniform_shape: source.uniform_shape,
        erosion_source_shape_degenerate: source_shape_degenerate,
        trace_detail,
    }));

    Ok(UpstreamHandoff {
        samples: result.hydrograph,
        bins_m2: result.outlet_bin_outflow_m2,
        bin_spans_s: result.outlet_bin_spans_s,
        bin_dt_s: result.outlet_bin_dt_s,
        width_m: width,
    })
}

/// Rev-27 day-closure hard-fail: (b) the clamp-adjusted day cascade residual
/// and (c) the assembled hillslope-day identity. Folds the day into the run
/// summary on success.
pub(crate) fn laned_active_enforce_day_closure(
    day_index: usize,
    books: &DirectLanedActiveDayBooks,
    summary: &mut DirectLanedActiveRunSummary,
) -> Result<(), DirectRuntimeError> {
    summary.days_seen += 1;
    // CR-L1: the latqcc bypass total covers ALL days (the terminal lane's
    // lateral export exists on zero-source days too).
    summary.total_latqcc_outlet_m3 += books.latqcc_outlet_m3;
    if !books.routed {
        return Ok(());
    }
    summary.days_routed += 1;
    summary.total_source_m3 += books.injected_m3;
    summary.total_routed_outlet_m3 += books.terminal_outlet_m3;
    summary.total_end_window_storage_m3 += books.mesh_storage_m3;
    summary.total_clamp_m3 += books.clamp_m3;
    summary.total_tail_fold_m3 += books.tail_fold_m3;
    summary.days_uniform_shape += books.uniform_shape_days;
    summary.lane_days_erosion_source_shape_degenerate +=
        books.erosion_source_shape_degenerate_lane_days;
    if books.max_supply_reconstruction_rel > summary.max_supply_reconstruction_rel {
        summary.max_supply_reconstruction_rel = books.max_supply_reconstruction_rel;
    }

    // Rev 40 WA numerics guard: clamp mass is an auditable numerical
    // correction, not an unbounded source term. If the active day needs more
    // positivity-clamp injection than the external source mass fed into the
    // router, fail before publication even if the clamp-adjusted identity can
    // be made algebraically exact.
    let clamp_cap_m3 = books.injected_m3 * LANED_ACTIVE_CLAMP_INPUT_REL_CAP;
    let clamp_slack_m3 = (LANED_ACTIVE_CASCADE_REL_TOL * books.injected_m3.max(1.0)).max(1.0e-12);
    if books.clamp_m3 > clamp_cap_m3 + clamp_slack_m3 {
        let clamp_rel = if books.injected_m3 > 0.0 {
            books.clamp_m3 / books.injected_m3
        } else {
            f64::INFINITY
        };
        return Err(DirectRuntimeError::DirectKernelGuardFailure {
            phase: "laned_active_clamp_exceeds_source",
            detail: format!(
                "day {} positivity clamp {} m3 exceeds active routed source cap {} m3 (rel {clamp_rel} > {LANED_ACTIVE_CLAMP_INPUT_REL_CAP})",
                day_index + 1,
                books.clamp_m3,
                clamp_cap_m3
            ),
        });
    }

    // (b) day cascade residual: injected + clamp − terminal outlet − ΣΔS_mesh.
    // ROUTER-INTERNAL identity: all four operands come from the solver
    // family's own mass ledgers, so this validates per-lane solver
    // conservation AND the inter-lane handoff telescoping (lane i+1's
    // booked inflow vs lane i's booked outflow), but NOT the soil↔router
    // seam — that is check (c).
    let cascade_residual_m3 =
        books.injected_m3 + books.clamp_m3 - books.terminal_outlet_m3 - books.mesh_storage_m3;
    let cascade_rel = if books.injected_m3 > 0.0 {
        cascade_residual_m3.abs() / books.injected_m3
    } else {
        0.0
    };
    if cascade_rel > summary.max_day_cascade_residual_rel {
        summary.max_day_cascade_residual_rel = cascade_rel;
    }
    if cascade_rel > LANED_ACTIVE_CASCADE_REL_TOL {
        return Err(DirectRuntimeError::DirectKernelGuardFailure {
            phase: "laned_active_day_cascade_residual",
            detail: format!(
                "day {} cascade residual {cascade_residual_m3} m3 (rel {cascade_rel} > {LANED_ACTIVE_CASCADE_REL_TOL}): injected {} + clamp {} - outlet {} - mesh_storage {}",
                day_index + 1,
                books.injected_m3,
                books.clamp_m3,
                books.terminal_outlet_m3,
                books.mesh_storage_m3
            ),
        });
    }

    // (c) SEAM cross-ledger check: the router's booked injection must equal
    // the soil books' released runoff volume. The operands come from
    // independent ledgers (solver mass booking vs `q_runoff × area` from
    // the R4A soil surface via the supply-reconstructed weights series), so
    // producer self-consistency cannot satisfy it; exactness is delivered
    // by the hourly forcing breakpoints + the recorded mesh-basis
    // conversion.
    let seam_residual_m3 = books.injected_m3 - books.soil_release_m3;
    let seam_scale_m3 = books.soil_release_m3.max(1.0e-6);
    let seam_rel = seam_residual_m3.abs() / seam_scale_m3;
    if seam_rel > summary.max_day_seam_residual_rel {
        summary.max_day_seam_residual_rel = seam_rel;
    }
    if seam_rel > LANED_ACTIVE_SEAM_REL_TOL {
        return Err(DirectRuntimeError::DirectKernelGuardFailure {
            phase: "laned_active_day_seam_residual",
            detail: format!(
                "day {} soil-to-router seam residual {seam_residual_m3} m3 (rel {seam_rel} > {LANED_ACTIVE_SEAM_REL_TOL}): router injected {} vs soil released {}",
                day_index + 1,
                books.injected_m3,
                books.soil_release_m3
            ),
        });
    }

    // (d) assembled hillslope-day identity, presented with the SOIL-side
    // release entering the router (so it composes (b) and (c) plus the lane
    // R4B nets). NOTE the honesty caveat recorded in the package evidence:
    // the per-lane R4B residual is zero by construction (the reconciled
    // storage is defined by the same identity), so `lane_net_m3` carries
    // only fp re-association; the content of (d) is carried by (b) + (c),
    // and the lane-level water truthfulness rests on the kernel spans' own
    // domain/closure guards upstream.
    let identity_residual_m3 = books.lane_net_m3 + books.soil_release_m3 + books.clamp_m3
        - books.terminal_outlet_m3
        - books.mesh_storage_m3;
    let scale_m3 = books.max_abs_term_m3.max(1.0e-6);
    let identity_rel = identity_residual_m3.abs() / scale_m3;
    if identity_rel > summary.max_day_identity_residual_rel {
        summary.max_day_identity_residual_rel = identity_rel;
    }
    if identity_rel > LANED_ACTIVE_IDENTITY_REL_TOL {
        return Err(DirectRuntimeError::DirectKernelGuardFailure {
            phase: "laned_active_day_identity_residual",
            detail: format!(
                "day {} identity residual {identity_residual_m3} m3 (rel {identity_rel} > {LANED_ACTIVE_IDENTITY_REL_TOL}, scale {scale_m3}): lane_net {} + soil_release {} + clamp {} - outlet {} - mesh_storage {}",
                day_index + 1,
                books.lane_net_m3,
                books.soil_release_m3,
                books.clamp_m3,
                books.terminal_outlet_m3,
                books.mesh_storage_m3
            ),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::direct_runtime::DirectRunIdentity;

    fn lane_config_with_static_canhgt(canopy_height_m: Option<f64>) -> DirectLanedActiveLaneConfig {
        DirectLanedActiveLaneConfig {
            slplen_m: 1.0,
            width_m: 1.0,
            mean_gradient: 0.01,
            skin_friction_coefficient_ko: 500.0,
            form_drag_coefficient: 0.0,
            roughness_element_height_m: 0.0,
            roughness_concentration: 0.0,
            vegetation_drag_coefficient: 0.2,
            canopy_height_m,
        }
    }

    fn vegetated_day_with_post_growth_canhgt(canopy_height_m: f64) -> DirectDayFrame {
        let identity =
            DirectRunIdentity::new(36, 2637, 1, 1).expect("valid direct identity should construct");
        let mut day =
            DirectDayFrame::seed(identity, 0, 0).expect("valid direct day should construct");
        day.evapotranspiration_compute_inputs.leaf_area_index = 0.3;
        day.evapotranspiration_compute_inputs.canopy_height_m = canopy_height_m;
        day
    }

    fn dry_lane_source() -> LanedActiveLaneSource {
        LanedActiveLaneSource {
            depths_m: [0.0; SEAM_HOUR_BINS],
            q_runoff_m: 0.0,
            uniform_shape: false,
            supply_reconstruction_rel: 0.0,
        }
    }

    fn routing_result_with_bins(bins_m2: Vec<f64>, bin_dt_s: f64) -> RoutingResult {
        let spans = vec![bin_dt_s; bins_m2.len()];
        RoutingResult {
            hydrograph: Vec::new(),
            mass_balance: crate::ofe_routing::kinematic_wave::MassBalance::default(),
            peak_unit_discharge_m2_s: 0.0,
            time_to_peak_s: 0.0,
            max_courant: 0.0,
            max_homogeneous_tv_increase_m2_s: 0.0,
            outlet_bin_outflow_m2: bins_m2,
            outlet_bin_dt_s: bin_dt_s,
            outlet_bin_spans_s: spans,
            step_trace: None,
        }
    }

    #[test]
    fn routed_erosion_weights_hour_align_and_tail_fold() {
        // 900 s bins: 4 per hour; put mass in hour 0 (bins 0-3), hour 25
        // (bins 100-103 -> tail fold into hour 24 / index 23).
        let mut bins = vec![0.0_f64; 104];
        bins[0] = 1.0;
        bins[1] = 1.0;
        bins[100] = 2.0;
        let result = routing_result_with_bins(bins, 900.0);
        let zero_source = [0.0_f64; SEAM_HOUR_BINS];
        let (weights, tail_fold_m3, degenerate) =
            laned_active_routed_erosion_weights(&result, 2.0, 0.01, &zero_source).expect("weights");
        assert!(!degenerate);
        // total mass = (1+1+2) m2 x 2 m width = 8 m3; hour0 = 4 m3, hour24 fold = 4 m3
        assert!((tail_fold_m3 - 4.0).abs() < 1.0e-12);
        assert!((weights[0] - 0.5).abs() < 1.0e-12);
        assert!((weights[23] - 0.5).abs() < 1.0e-12);
        let sum: f64 = weights.iter().sum();
        assert!((sum - 1.0).abs() < 1.0e-12, "unit sum, got {sum}");
    }

    #[test]
    fn active_route_uses_post_growth_canhgt_not_static_lane_config() {
        let mut day = vegetated_day_with_post_growth_canhgt(0.45);
        let mut books = DirectLanedActiveDayBooks::default();
        laned_active_route_lane(
            &mut day,
            &lane_config_with_static_canhgt(Some(0.0)),
            &DirectLanedActiveMeshPolicy::production_default(),
            1.0,
            None,
            3600.0,
            &mut books,
            &dry_lane_source(),
            150.0,
            true,
            false,
        )
        .expect("positive post-growth canhgt should satisfy the vegetated guard");
        let trace_detail = day
            .laned_active_routing
            .as_ref()
            .and_then(|routing| routing.trace_detail.as_ref())
            .expect("trace detail");
        assert!((trace_detail.max_dt_s - 150.0).abs() < f64::EPSILON);
        let mut summary = DirectLanedActiveRunSummary::for_mesh_policy(
            DirectLanedActiveMeshPolicy::production_default(),
            150.0,
            true,
        );
        laned_active_record_trace(&mut summary, &day, true, 0.0).expect("trace row");
        let trace_record = &summary.trace_records.as_ref().expect("records")[0];
        assert!((trace_record.max_dt_s - 150.0).abs() < f64::EPSILON);

        let mut stale_static_day = vegetated_day_with_post_growth_canhgt(0.0);
        let mut stale_static_books = DirectLanedActiveDayBooks::default();
        assert!(matches!(
            laned_active_route_lane(
                &mut stale_static_day,
                &lane_config_with_static_canhgt(Some(0.75)),
                &DirectLanedActiveMeshPolicy::production_default(),
                1.0,
                None,
                3600.0,
                &mut stale_static_books,
                &dry_lane_source(),
                LANED_ACTIVE_MAX_DT_S,
                false,
                false,
            ),
            Err(DirectRuntimeError::DirectKernelGuardFailure {
                phase: "laned_active_rev21_operands",
                ..
            })
        ));
    }

    #[test]
    fn routed_erosion_weights_dry_lane_is_all_zero() {
        let result = routing_result_with_bins(vec![1.0, 2.0], 900.0);
        let zero_source = [0.0_f64; SEAM_HOUR_BINS];
        let (weights, _, degenerate) =
            laned_active_routed_erosion_weights(&result, 1.0, 0.0, &zero_source).expect("weights");
        assert!(!degenerate);
        assert!(weights.iter().all(|w| *w == 0.0));
    }

    #[test]
    fn routed_erosion_weights_degenerate_to_source_shape_on_full_mesh_hold() {
        // Zero outlet mass above the wet-gate: the rev-27 degeneracy uses the
        // normalized routed source series and is counted, never silent.
        let result = routing_result_with_bins(vec![0.0, 0.0], 900.0);
        let mut source = [0.0_f64; SEAM_HOUR_BINS];
        source[3] = 3.0e-12;
        source[4] = 1.0e-12;
        let (weights, _, degenerate) =
            laned_active_routed_erosion_weights(&result, 1.0, 1.0e-11, &source)
                .expect("degenerate weights");
        assert!(degenerate);
        assert!((weights[3] - 0.75).abs() < 1.0e-12);
        assert!((weights[4] - 0.25).abs() < 1.0e-12);
        let sum: f64 = weights.iter().sum();
        assert!((sum - 1.0).abs() < 1.0e-12);
        // Above the wet-gate with zero outlet AND zero source: corruption,
        // fail closed.
        let zero_source = [0.0_f64; SEAM_HOUR_BINS];
        assert!(matches!(
            laned_active_routed_erosion_weights(&result, 1.0, 0.01, &zero_source),
            Err(DirectRuntimeError::DirectKernelGuardFailure {
                phase: "laned_active_routed_erosion_weights",
                ..
            })
        ));
    }

    #[test]
    fn day_window_matches_rev27_rule() {
        assert!(
            (laned_active_window_s(0) - (3600.0 + LANED_ACTIVE_DRAIN_TAIL_S)).abs() < f64::EPSILON
        );
        assert!(
            (laned_active_window_s(23)
                - (LANED_ACTIVE_SOURCE_WINDOW_S + LANED_ACTIVE_DRAIN_TAIL_S))
                .abs()
                < f64::EPSILON
        );
    }

    #[test]
    fn mesh_policy_resolves_fixed_target_floor_and_cap() {
        let fixed = DirectLanedActiveMeshPolicy::production_default();
        assert_eq!(
            fixed.cell_count_for_length_m(26.0).expect("fixed cells"),
            10
        );

        let dx20 = DirectLanedActiveMeshPolicy::diagnostic_target_dx(20.0).expect("target dx");
        assert_eq!(dx20.cell_count_for_length_m(26.0).expect("min floor"), 10);
        assert_eq!(dx20.cell_count_for_length_m(300.0).expect("ceil dx"), 15);

        assert!(DirectLanedActiveMeshPolicy::diagnostic_target_dx(0.0).is_err());
        assert!(dx20.cell_count_for_length_m(90_000.0).is_err());
    }

    #[test]
    fn day_closure_enforces_cascade_and_identity_tolerances() {
        let mut summary = DirectLanedActiveRunSummary::default();
        // Balanced books pass.
        let books = DirectLanedActiveDayBooks {
            injected_m3: 100.0,
            soil_release_m3: 100.0,
            terminal_outlet_m3: 90.0,
            mesh_storage_m3: 12.0,
            clamp_m3: 2.0,
            lane_net_m3: -(100.0 + 2.0 - 90.0 - 12.0),
            latqcc_outlet_m3: 5.0,
            max_abs_term_m3: 100.0,
            max_supply_reconstruction_rel: 0.0,
            tail_fold_m3: 0.0,
            uniform_shape_days: 0,
            erosion_source_shape_degenerate_lane_days: 0,
            routed: true,
        };
        laned_active_enforce_day_closure(0, &books, &mut summary).expect("closed day");
        assert_eq!(summary.days_routed, 1);

        // Equality at the source cap is allowed; only clamp > source fails.
        let at_clamp_cap = DirectLanedActiveDayBooks {
            terminal_outlet_m3: 188.0,
            clamp_m3: 100.0,
            lane_net_m3: -(100.0 + 100.0 - 188.0 - 12.0),
            ..books
        };
        laned_active_enforce_day_closure(0, &at_clamp_cap, &mut summary)
            .expect("clamp equal to source cap is allowed");

        // Broken router books fail (b).
        let broken = DirectLanedActiveDayBooks {
            terminal_outlet_m3: 80.0,
            ..books
        };
        assert!(matches!(
            laned_active_enforce_day_closure(0, &broken, &mut summary),
            Err(DirectRuntimeError::DirectKernelGuardFailure {
                phase: "laned_active_day_cascade_residual",
                ..
            })
        ));
        // Clamp mass cannot exceed the active day source cap even when the
        // clamp-adjusted router books are algebraically exact.
        let excessive_clamp = DirectLanedActiveDayBooks {
            terminal_outlet_m3: 189.0,
            clamp_m3: 101.0,
            lane_net_m3: 0.0,
            ..books
        };
        assert!(matches!(
            laned_active_enforce_day_closure(0, &excessive_clamp, &mut summary),
            Err(DirectRuntimeError::DirectKernelGuardFailure {
                phase: "laned_active_clamp_exceeds_source",
                ..
            })
        ));

        // Zero-source, nonzero-clamp active books are never publishable even
        // if outlet/storage terms make the algebraic identity exact.
        let zero_source_clamp = DirectLanedActiveDayBooks {
            injected_m3: 0.0,
            soil_release_m3: 0.0,
            terminal_outlet_m3: 0.0,
            mesh_storage_m3: 1.0e-6,
            clamp_m3: 1.0e-6,
            lane_net_m3: 0.0,
            max_abs_term_m3: 1.0e-6,
            ..books
        };
        assert!(matches!(
            laned_active_enforce_day_closure(0, &zero_source_clamp, &mut summary),
            Err(DirectRuntimeError::DirectKernelGuardFailure {
                phase: "laned_active_clamp_exceeds_source",
                ..
            })
        ));

        // Router books exact but the soil ledger disagrees: the SEAM check
        // fires (the cross-ledger guard QA-H2 demanded).
        let broken_seam = DirectLanedActiveDayBooks {
            soil_release_m3: 101.0,
            lane_net_m3: -(101.0 + 2.0 - 90.0 - 12.0),
            ..books
        };
        assert!(matches!(
            laned_active_enforce_day_closure(0, &broken_seam, &mut summary),
            Err(DirectRuntimeError::DirectKernelGuardFailure {
                phase: "laned_active_day_seam_residual",
                ..
            })
        ));
        // Router books exact but lane identity broken fails (d).
        let broken_identity = DirectLanedActiveDayBooks {
            lane_net_m3: 1.0,
            ..books
        };
        assert!(matches!(
            laned_active_enforce_day_closure(0, &broken_identity, &mut summary),
            Err(DirectRuntimeError::DirectKernelGuardFailure {
                phase: "laned_active_day_identity_residual",
                ..
            })
        ));
    }
}
