//! Lane D runtime seam SHADOW (`SC-OFEROUTE-001#INV-OFEROUTE-012`
//! activation increment, opt-in via `OPENWEPP_LANED_SHADOW=1`):
//! reconstructs each lane-day's routed source series from the LIVE
//! published surfaces (`dc01_surface_hourly_weights × runvol/area` — the
//! ADR-0036 weights-times-total hourly-flow authority on the lane-local
//! runoff-volume basis over the D12 surface-source shape limbs), routes event
//! days through the real
//! `ofe_routing::cascade`, and accumulates conservation diagnostics
//! into the manifest. DIAGNOSTIC ONLY: water authority stays DC01;
//! protected outputs are byte-identical with the shadow on or off;
//! production activation remains BLOCKED (`INV-OFEROUTE-011` open).
//!
//! Static friction operands are sourced from the openWEPP native
//! `routing_coefficients` management extension when the shadow is enabled;
//! missing extension data fails closed before streaming starts. Dynamic
//! rainfall-intensity, routed-melt timing, and canopy-state operands are
//! sourced from the live direct day frame: WB14 hourly rainfall depth,
//! source-authorized hourly routed snow liquid, post-growth LAI, and
//! typed-management canopy height.

use openwepp_hillslope_orchestrator::DirectPublicationDayRow;
use openwepp_hillslope_orchestrator::ofe_routing::cascade::{
    CascadeForcing, CascadeSegment, run_cascade,
};
use openwepp_hillslope_orchestrator::ofe_routing::kinematic_wave::{
    CellParameters, KinematicWaveMesh,
};
use openwepp_hillslope_orchestrator::ofe_routing::profile as routing_profile;
use openwepp_hillslope_orchestrator::ofe_routing::seam::{
    SEAM_HOUR_BINS, SEAM_SECONDS_PER_HOUR, seam_rate_at, seam_source_rates_from_hourly_depths,
};
use std::time::Instant;

/// Days below this injected volume are excluded from the MAX relative
/// residual (they still fold into the volume-weighted aggregate).
const RESIDUAL_FLOOR_M3: f64 = 0.1;
/// Cells per OFE for the shadow mesh (the D-val working resolution).
const LANED_SHADOW_CELLS: usize = 10;
/// Source window: the seam publishes one day of hourly source rates. Routing
/// may continue past this with zero source to drain water that entered in hour
/// 24; this is diagnostics-only shadow timing, not an inter-day production
/// carry claim.
const LANED_SHADOW_SOURCE_WINDOW_S: f64 = 24.0 * 3600.0;
/// Drain tail after the last active source hour. D14 added the tail to avoid
/// clipping routed fronts; D15 blocker resolution removes the old one-day cap
/// so an hour-24 source receives the same tail as earlier hours.
const LANED_SHADOW_DRAIN_TAIL_S: f64 = 6.0 * 3600.0;
// Resolution note (H2637 sweep, 2026-07-05): the cascade's run-level
// conservation aggregate on the steep 19-OFE regime is RESOLUTION-
// SENSITIVE in the GAP-OFEROUTE-005 class — (sample_dt, max_dt) of
// (900, 300) → 6.0%, (900, 120) → 10.0%, (120, 300) → 22.1% aggregate
// residual, dt-non-monotone. The constants here are the empirically
// best point; sharpening belongs to the shock-numerics package, and the
// shadow's conservation figure is a DIAGNOSTIC bound, not a physics
// acceptance.
const LANED_SHADOW_SAMPLE_DT_S: f64 = 900.0;
const LANED_SHADOW_MAX_DT_S: f64 = 300.0;

/// Per-lane static geometry for the shadow cascade (from the Wave-1
/// operand seed: slope length, field width, mean profile gradient).
#[derive(Debug, Clone, Copy)]
pub(crate) struct LanedShadowLaneGeometry {
    pub slplen_m: f64,
    pub width_m: f64,
    pub mean_gradient: f64,
    pub routing: LanedShadowRoutingCoefficients,
}

/// Static per-lane friction coefficients sourced from the native management
/// routing extension.
#[derive(Debug, Clone, Copy)]
pub(crate) struct LanedShadowRoutingCoefficients {
    pub skin_friction_coefficient_ko: f64,
    pub form_drag_coefficient: f64,
    pub roughness_element_height_m: f64,
    pub roughness_concentration: f64,
    pub vegetation_drag_coefficient: f64,
}

impl LanedShadowRoutingCoefficients {
    fn cell_parameters(self, slope: f64, operands: &LanedShadowLaneDayOperands) -> CellParameters {
        let mut cell = CellParameters::bare(slope, self.skin_friction_coefficient_ko);
        cell.drag_coefficient = self.form_drag_coefficient;
        cell.element_tip_height_m = self.roughness_element_height_m;
        cell.roughness_concentration = self.roughness_concentration;
        cell.vegetation_drag_coefficient = self.vegetation_drag_coefficient;
        cell.leaf_area_index = operands.leaf_area_index;
        cell.canopy_height_m = operands.canopy_height_m;
        cell
    }
}

/// Per-lane dynamic friction operands for one direct day frame.
#[derive(Debug, Clone)]
pub(crate) struct LanedShadowLaneDayOperands {
    pub hourly_rainfall_m: [f64; SEAM_HOUR_BINS],
    pub hourly_routed_melt_m: [f64; SEAM_HOUR_BINS],
    pub leaf_area_index: f64,
    pub canopy_height_m: f64,
}

/// Run-level shadow accumulators surfaced into the manifest.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct LanedShadowSummary {
    pub days_seen: u64,
    pub days_routed: u64,
    /// Max per-day cascade conservation residual relative to that day's
    /// injected volume, over days with at least `RESIDUAL_FLOOR_M3` of
    /// source (relative residuals on near-zero days are noise).
    pub max_router_conservation_rel: f64,
    /// Volume-weighted aggregate: `Σ|residual| / Σ injected` over ALL
    /// routed days — the run-level conservation figure.
    pub aggregate_router_conservation_rel: f64,
    pub max_supply_reconstruction_rel: f64,
    pub total_source_m3: f64,
    pub total_routed_outlet_m3: f64,
    /// Days whose weights had no hourly shape (uniform fallback — the
    /// DC01 lump-only class, e.g. melt-sourced runoff outside the two
    /// GAP-006 D1 limbs). Surfaced as a labeled seam-coverage finding.
    pub days_uniform_shape: u64,
    pub days_uniform_shape_with_routed_melt: u64,
    pub days_uniform_shape_without_routed_melt: u64,
    residual_abs_m3: f64,
}

/// D14 runner-side slot accumulators (opt-in via
/// `OPENWEPP_LANED_SHADOW_PROFILE=1`). Diagnostics-only: reported to stderr at
/// finalize; never touches published outputs or the manifest.
#[derive(Debug, Clone, Copy, Default)]
struct LanedShadowProfileSlots {
    operand_build_ns: u64,
    observe_row_ns: u64,
    mesh_build_ns: u64,
    rate_series_ns: u64,
    cascade_run_ns: u64,
    rows_observed: u64,
}

pub(crate) struct LanedShadowCollector {
    geometry: Vec<LanedShadowLaneGeometry>,
    day_depths: Vec<[f64; SEAM_HOUR_BINS]>,
    day_operands: Vec<Option<LanedShadowLaneDayOperands>>,
    day_source_m3: f64,
    day_supply_reference_m3: f64,
    lanes_seen_today: usize,
    day_saw_uniform_shape: bool,
    day_saw_uniform_shape_with_routed_melt: bool,
    current_day: Option<usize>,
    summary: LanedShadowSummary,
    profile: Option<LanedShadowProfileSlots>,
}

impl LanedShadowCollector {
    #[must_use]
    pub(crate) fn new(geometry: Vec<LanedShadowLaneGeometry>) -> Self {
        let lane_count = geometry.len();
        let profile = if Self::env_profile_enabled() {
            routing_profile::set_enabled(true);
            let _ = routing_profile::snapshot_and_reset();
            Some(LanedShadowProfileSlots::default())
        } else {
            None
        };
        Self {
            geometry,
            day_depths: vec![[0.0; SEAM_HOUR_BINS]; lane_count],
            day_operands: vec![None; lane_count],
            day_source_m3: 0.0,
            day_supply_reference_m3: 0.0,
            lanes_seen_today: 0,
            day_saw_uniform_shape: false,
            day_saw_uniform_shape_with_routed_melt: false,
            current_day: None,
            summary: LanedShadowSummary::default(),
            profile,
        }
    }

    /// Env opt-in: `OPENWEPP_LANED_SHADOW=1`.
    #[must_use]
    pub(crate) fn env_enabled() -> bool {
        std::env::var("OPENWEPP_LANED_SHADOW").is_ok_and(|value| value == "1")
    }

    /// D14 slot-profiling env opt-in: `OPENWEPP_LANED_SHADOW_PROFILE=1`.
    #[must_use]
    fn env_profile_enabled() -> bool {
        std::env::var("OPENWEPP_LANED_SHADOW_PROFILE").is_ok_and(|value| value == "1")
    }

    fn routing_window_s(last_active_hour: usize) -> f64 {
        #[allow(clippy::cast_precision_loss)]
        let active_end_s = ((last_active_hour + 1) as f64) * SEAM_SECONDS_PER_HOUR;
        active_end_s.min(LANED_SHADOW_SOURCE_WINDOW_S) + LANED_SHADOW_DRAIN_TAIL_S
    }

    /// Start a runner-side profiling span (None when profiling is off).
    #[must_use]
    pub(crate) fn profile_span_start(&self) -> Option<Instant> {
        self.profile.as_ref().map(|_| Instant::now())
    }

    fn span_ns(started: Option<Instant>) -> u64 {
        started.map_or(0, |instant| {
            u64::try_from(instant.elapsed().as_nanos()).unwrap_or(u64::MAX)
        })
    }

    /// Fold an operand-build span (runner streaming closure) into the slots.
    pub(crate) fn record_operand_build(&mut self, started: Option<Instant>) {
        let ns = Self::span_ns(started);
        if let Some(profile) = self.profile.as_mut() {
            profile.operand_build_ns += ns;
        }
    }

    /// Observe one published lane-day row (stream order: day-major,
    /// lane-minor). Commits the previous day when the day index moves.
    pub(crate) fn observe_row(
        &mut self,
        row: &DirectPublicationDayRow,
        operands: Box<LanedShadowLaneDayOperands>,
    ) -> Result<(), String> {
        let operands = *operands;
        if self.current_day.is_some_and(|day| day != row.day_index) {
            self.commit_day()?;
        }
        // Row-local slot only: commit_day above accounts its own cascade
        // slots (mesh/rate/cascade), so the observe span starts after it.
        let observe_span = self.profile_span_start();
        self.current_day = Some(row.day_index);
        let lane = row.lane_index;
        if lane >= self.geometry.len() {
            return Err(format!(
                "laned shadow: lane index {lane} outside geometry ({})",
                self.geometry.len()
            ));
        }
        Self::validate_lane_day_operands(row, &operands)?;
        let area_m2 = row.area_m2;
        if !area_m2.is_finite() || area_m2 <= 0.0 {
            return Err(format!(
                "laned shadow: lane {} day {} area must be finite and > 0, observed {area_m2}",
                lane + 1,
                row.day_index + 1
            ));
        }
        if !row.runoff.runvol_m3.is_finite() || row.runoff.runvol_m3 < 0.0 {
            return Err(format!(
                "laned shadow: lane {} day {} runvol must be finite and nonnegative, observed {}",
                lane + 1,
                row.day_index + 1,
                row.runoff.runvol_m3
            ));
        }
        // The seam depth series: weights x the lane-local runoff-volume
        // basis. Published QOFE intentionally aliases cumulative Q
        // (INV-RUNOFFPART-032), so the shadow must reconstruct from
        // runvol/area instead.
        let local_runoff_m = row.runoff.runvol_m3 / area_m2;
        let mut depths = [0.0_f64; SEAM_HOUR_BINS];
        if local_runoff_m > 0.0 {
            // Uniform weights = the DC01 lump-only fallback (runoff with
            // no hourly shape from the two D1 limbs — e.g. melt-sourced):
            // count the day-class; the routed series still consumes it.
            let uniform = 1.0 / 24.0;
            if row
                .dc01_surface_hourly_weights
                .iter()
                .all(|weight| (weight - uniform).abs() < 1.0e-12)
            {
                self.day_saw_uniform_shape = true;
                if operands.hourly_routed_melt_m.iter().sum::<f64>() > 1.0e-12 {
                    self.day_saw_uniform_shape_with_routed_melt = true;
                }
            }
            for (depth, weight) in depths
                .iter_mut()
                .zip(row.dc01_surface_hourly_weights.iter())
            {
                *depth = weight * local_runoff_m;
            }
        }
        let depth_sum: f64 = depths.iter().sum();
        self.day_depths[lane] = depths;
        self.day_operands[lane] = Some(operands);
        self.day_source_m3 += depth_sum * area_m2;
        self.day_supply_reference_m3 += row.runoff.runvol_m3;
        self.lanes_seen_today += 1;
        let observe_ns = Self::span_ns(observe_span);
        if let Some(profile) = self.profile.as_mut() {
            profile.observe_row_ns += observe_ns;
            profile.rows_observed += 1;
        }
        Ok(())
    }

    fn validate_lane_day_operands(
        row: &DirectPublicationDayRow,
        operands: &LanedShadowLaneDayOperands,
    ) -> Result<(), String> {
        for (hour_index, rainfall_m) in operands.hourly_rainfall_m.iter().enumerate() {
            if !rainfall_m.is_finite() || *rainfall_m < 0.0 {
                return Err(format!(
                    "laned shadow: lane {} day {} hourly rainfall slot {} must be finite and nonnegative, observed {}",
                    row.lane_index + 1,
                    row.day_index + 1,
                    hour_index + 1,
                    rainfall_m
                ));
            }
        }
        for (hour_index, routed_melt_m) in operands.hourly_routed_melt_m.iter().enumerate() {
            if !routed_melt_m.is_finite() || *routed_melt_m < 0.0 {
                return Err(format!(
                    "laned shadow: lane {} day {} hourly routed melt slot {} must be finite and nonnegative, observed {}",
                    row.lane_index + 1,
                    row.day_index + 1,
                    hour_index + 1,
                    routed_melt_m
                ));
            }
        }
        if !operands.leaf_area_index.is_finite() || operands.leaf_area_index < 0.0 {
            return Err(format!(
                "laned shadow: lane {} day {} LAI must be finite and nonnegative, observed {}",
                row.lane_index + 1,
                row.day_index + 1,
                operands.leaf_area_index
            ));
        }
        if !operands.canopy_height_m.is_finite() || operands.canopy_height_m < 0.0 {
            return Err(format!(
                "laned shadow: lane {} day {} canopy height must be finite and nonnegative, observed {}",
                row.lane_index + 1,
                row.day_index + 1,
                operands.canopy_height_m
            ));
        }
        if operands.leaf_area_index > 0.0 && operands.canopy_height_m <= 0.0 {
            return Err(format!(
                "laned shadow: lane {} day {} canopy height must be > 0 when LAI is positive (LAI={}, h_c={})",
                row.lane_index + 1,
                row.day_index + 1,
                operands.leaf_area_index,
                operands.canopy_height_m
            ));
        }
        Ok(())
    }

    /// Route the buffered day (if it carries any source) and fold the
    /// diagnostics; called on day change and at finalize.
    fn commit_day(&mut self) -> Result<(), String> {
        self.summary.days_seen += 1;
        let source_m3 = self.day_source_m3;
        let reference_m3 = self.day_supply_reference_m3;
        // Supply-reconstruction faithfulness: the weights are unit-normalized,
        // so the reconstructed series must resum to lane-local runvol.
        if reference_m3 > 0.0 {
            let rel = (source_m3 - reference_m3).abs() / reference_m3;
            self.summary.max_supply_reconstruction_rel =
                self.summary.max_supply_reconstruction_rel.max(rel);
        }
        if source_m3 > 0.0 {
            self.route_buffered_day(source_m3)?;
        } else {
            for depths in &mut self.day_depths {
                *depths = [0.0; SEAM_HOUR_BINS];
            }
            for operands in &mut self.day_operands {
                *operands = None;
            }
        }
        if self.day_saw_uniform_shape {
            self.summary.days_uniform_shape += 1;
            if self.day_saw_uniform_shape_with_routed_melt {
                self.summary.days_uniform_shape_with_routed_melt += 1;
            } else {
                self.summary.days_uniform_shape_without_routed_melt += 1;
            }
        }
        self.day_source_m3 = 0.0;
        self.day_supply_reference_m3 = 0.0;
        self.lanes_seen_today = 0;
        self.day_saw_uniform_shape = false;
        self.day_saw_uniform_shape_with_routed_melt = false;
        Ok(())
    }

    /// Build the day's cascade segments from lane geometry + dynamic operands.
    fn build_cascade_segments(
        &self,
        day_operands: &[Option<LanedShadowLaneDayOperands>],
    ) -> Result<Vec<CascadeSegment>, String> {
        self.geometry
            .iter()
            .enumerate()
            .map(|(lane_index, geom)| {
                let operands = day_operands[lane_index].as_ref().ok_or_else(|| {
                    format!(
                        "laned shadow: missing dynamic operands for lane {} day {}",
                        lane_index + 1,
                        self.current_day.map_or(0, |day| day + 1)
                    )
                })?;
                Ok(CascadeSegment {
                    mesh: KinematicWaveMesh::uniform(
                        geom.slplen_m,
                        LANED_SHADOW_CELLS,
                        geom.routing.cell_parameters(geom.mean_gradient, operands),
                    ),
                    width_m: geom.width_m,
                })
            })
            .collect::<Result<Vec<_>, String>>()
    }

    /// Build the per-lane source-rate and rainfall-intensity series.
    #[allow(clippy::type_complexity)]
    fn build_day_rate_series(
        &self,
        day_depths: &[[f64; SEAM_HOUR_BINS]],
        day_operands: &[Option<LanedShadowLaneDayOperands>],
    ) -> Result<(Vec<[f64; SEAM_HOUR_BINS]>, Vec<[f64; SEAM_HOUR_BINS]>), String> {
        let mut rate_series = Vec::with_capacity(day_depths.len());
        for depths in day_depths {
            rate_series.push(
                seam_source_rates_from_hourly_depths(depths)
                    .map_err(|error| format!("laned shadow seam rates: {error:?}"))?,
            );
        }
        let mut intensity_series = Vec::with_capacity(day_operands.len());
        for operands in day_operands {
            let operands = operands.as_ref().ok_or_else(|| {
                format!(
                    "laned shadow: missing dynamic operands for day {}",
                    self.current_day.map_or(0, |day| day + 1)
                )
            })?;
            intensity_series.push(
                seam_source_rates_from_hourly_depths(&operands.hourly_rainfall_m)
                    .map_err(|error| format!("laned shadow rainfall-intensity rates: {error:?}"))?,
            );
        }
        Ok((rate_series, intensity_series))
    }

    fn route_buffered_day(&mut self, source_m3: f64) -> Result<(), String> {
        let mesh_span = self.profile_span_start();
        let day_operands =
            std::mem::replace(&mut self.day_operands, vec![None; self.geometry.len()]);
        let segments = self.build_cascade_segments(&day_operands)?;
        let mesh_ns = Self::span_ns(mesh_span);
        let rate_span = self.profile_span_start();
        let day_depths = std::mem::replace(
            &mut self.day_depths,
            vec![[0.0; SEAM_HOUR_BINS]; self.geometry.len()],
        );
        let (rate_series, intensity_series) =
            self.build_day_rate_series(&day_depths, &day_operands)?;
        let rate_ns = Self::span_ns(rate_span);
        let excess = |ofe: usize, _cell: usize, t: f64| seam_rate_at(&rate_series[ofe], t);
        let intensity = |ofe: usize, t: f64| seam_rate_at(&intensity_series[ofe], t);
        let forcing = CascadeForcing {
            rainfall_excess_m_s: &excess,
            rainfall_intensity_m_s: &intensity,
        };
        // Clip the routing window to the active source span (+ a 6 h drain
        // tail) because dry-hour substeps dominate cost on a full-day window.
        let last_active_hour = day_depths
            .iter()
            .flat_map(|depths| {
                depths
                    .iter()
                    .enumerate()
                    .filter(|(_, depth)| **depth > 0.0)
                    .map(|(hour, _)| hour)
            })
            .max()
            .unwrap_or(0);
        let window_s = Self::routing_window_s(last_active_hour);
        let cascade_span = self.profile_span_start();
        let result = run_cascade(
            &segments,
            &forcing,
            window_s,
            LANED_SHADOW_SAMPLE_DT_S,
            LANED_SHADOW_MAX_DT_S,
        )
        .map_err(|error| format!("laned shadow cascade: {error:?}"))?;
        let cascade_ns = Self::span_ns(cascade_span);
        if let Some(profile) = self.profile.as_mut() {
            profile.mesh_build_ns += mesh_ns;
            profile.rate_series_ns += rate_ns;
            profile.cascade_run_ns += cascade_ns;
        }
        let rain_m3 = result.mass_balance.rainfall_excess_m3;
        if rain_m3 > 0.0 {
            let residual_abs = result.mass_balance.conservation_residual_m3().abs();
            self.summary.residual_abs_m3 += residual_abs;
            if rain_m3 >= RESIDUAL_FLOOR_M3 {
                self.summary.max_router_conservation_rel = self
                    .summary
                    .max_router_conservation_rel
                    .max(residual_abs / rain_m3);
            }
        }
        self.summary.days_routed += 1;
        self.summary.total_source_m3 += source_m3;
        self.summary.total_routed_outlet_m3 += result.mass_balance.outlet_m3;
        Ok(())
    }

    /// Commit the trailing day and return the run summary.
    pub(crate) fn finalize(mut self) -> Result<LanedShadowSummary, String> {
        if self.current_day.is_some() {
            self.commit_day()?;
        }
        if self.summary.total_source_m3 > 0.0 {
            self.summary.aggregate_router_conservation_rel =
                self.summary.residual_abs_m3 / self.summary.total_source_m3;
        }
        if let Some(profile) = self.profile {
            Self::emit_profile_report(&profile, &self.summary);
        }
        Ok(self.summary)
    }

    /// D14 slot report: one stderr JSON line, emitted only under
    /// `OPENWEPP_LANED_SHADOW_PROFILE=1`. Protected outputs and the manifest
    /// are untouched; this is the local-CI-discoverable timing diagnostic.
    fn emit_profile_report(profile: &LanedShadowProfileSlots, summary: &LanedShadowSummary) {
        let routing = routing_profile::snapshot_and_reset();
        eprintln!(
            concat!(
                "laned_shadow_profile {{",
                "\"rows_observed\":{},",
                "\"days_seen\":{},",
                "\"days_routed\":{},",
                "\"operand_build_ns\":{},",
                "\"observe_row_ns\":{},",
                "\"mesh_build_ns\":{},",
                "\"rate_series_ns\":{},",
                "\"cascade_run_ns\":{},",
                "\"solver_runs\":{},",
                "\"solver_steps\":{},",
                "\"alpha_evaluations\":{},",
                "\"hydrograph_samples\":{},",
                "\"upstream_interpolation_calls\":{},",
                "\"solver_setup_ns\":{},",
                "\"solver_cfl_ns\":{},",
                "\"solver_step_ns\":{},",
                "\"solver_sample_ns\":{}",
                "}}"
            ),
            profile.rows_observed,
            summary.days_seen,
            summary.days_routed,
            profile.operand_build_ns,
            profile.observe_row_ns,
            profile.mesh_build_ns,
            profile.rate_series_ns,
            profile.cascade_run_ns,
            routing.solver_runs,
            routing.solver_steps,
            routing.alpha_evaluations,
            routing.hydrograph_samples,
            routing.upstream_interpolation_calls,
            routing.solver_setup_ns,
            routing.solver_cfl_ns,
            routing.solver_step_ns,
            routing.solver_sample_ns,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dynamic_canopy_operands_reach_cell_parameters() {
        let routing = LanedShadowRoutingCoefficients {
            skin_friction_coefficient_ko: 501.0,
            form_drag_coefficient: 0.9,
            roughness_element_height_m: 0.02,
            roughness_concentration: 0.15,
            vegetation_drag_coefficient: 1.1,
        };
        let operands = LanedShadowLaneDayOperands {
            hourly_rainfall_m: [0.0; SEAM_HOUR_BINS],
            hourly_routed_melt_m: [0.0; SEAM_HOUR_BINS],
            leaf_area_index: 2.5,
            canopy_height_m: 0.8,
        };

        let cell = routing.cell_parameters(0.04, &operands);

        assert_eq!(cell.friction_coefficient_ko.to_bits(), 501.0_f64.to_bits());
        assert_eq!(cell.drag_coefficient.to_bits(), 0.9_f64.to_bits());
        assert_eq!(cell.element_tip_height_m.to_bits(), 0.02_f64.to_bits());
        assert_eq!(cell.roughness_concentration.to_bits(), 0.15_f64.to_bits());
        assert_eq!(
            cell.vegetation_drag_coefficient.to_bits(),
            1.1_f64.to_bits()
        );
        assert_eq!(cell.leaf_area_index.to_bits(), 2.5_f64.to_bits());
        assert_eq!(cell.canopy_height_m.to_bits(), 0.8_f64.to_bits());
    }

    #[test]
    fn dynamic_rainfall_intensity_changes_routed_cascade_result() {
        let dry_intensity_outlet_m3 = routed_outlet_m3([0.0; SEAM_HOUR_BINS]);
        let mut hourly_rainfall_m = [0.0; SEAM_HOUR_BINS];
        hourly_rainfall_m[0] = 0.025;
        let wet_intensity_outlet_m3 = routed_outlet_m3(hourly_rainfall_m);

        assert!(
            (wet_intensity_outlet_m3 - dry_intensity_outlet_m3).abs() > 1.0e-9,
            "expected nonzero rainfall intensity to change routed outlet volume: dry={dry_intensity_outlet_m3}, wet={wet_intensity_outlet_m3}"
        );
    }

    // D14: runner-side slots accumulate when the profile struct is armed
    // (field set directly here; production arms it via
    // OPENWEPP_LANED_SHADOW_PROFILE=1).
    #[test]
    fn runner_profile_slots_accumulate_for_routed_day() {
        let routing = LanedShadowRoutingCoefficients {
            skin_friction_coefficient_ko: 0.25,
            form_drag_coefficient: 0.0,
            roughness_element_height_m: 0.0,
            roughness_concentration: 0.0,
            vegetation_drag_coefficient: 0.0,
        };
        let mut collector = LanedShadowCollector::new(vec![LanedShadowLaneGeometry {
            slplen_m: 80.0,
            width_m: 1.0,
            mean_gradient: 0.01,
            routing,
        }]);
        collector.profile = Some(LanedShadowProfileSlots::default());
        collector.current_day = Some(0);
        collector.day_depths[0][0] = 0.01;
        collector.day_operands[0] = Some(LanedShadowLaneDayOperands {
            hourly_rainfall_m: [0.0; SEAM_HOUR_BINS],
            hourly_routed_melt_m: [0.0; SEAM_HOUR_BINS],
            leaf_area_index: 0.0,
            canopy_height_m: 0.0,
        });

        collector
            .route_buffered_day(0.01)
            .expect("buffered day should route");

        let profile = collector.profile.expect("profile armed");
        assert!(profile.cascade_run_ns > 0, "cascade slot timed");
        assert!(profile.mesh_build_ns > 0, "mesh slot timed");
        assert!(profile.rate_series_ns > 0, "rate-series slot timed");
    }

    #[test]
    fn routing_window_keeps_drain_tail_for_hour_24_source() {
        let first_hour = LanedShadowCollector::routing_window_s(0);
        let hour_24 = LanedShadowCollector::routing_window_s(SEAM_HOUR_BINS - 1);
        assert!(
            (first_hour - (SEAM_SECONDS_PER_HOUR + LANED_SHADOW_DRAIN_TAIL_S)).abs()
                <= f64::EPSILON
        );
        assert!(
            (hour_24 - (LANED_SHADOW_SOURCE_WINDOW_S + LANED_SHADOW_DRAIN_TAIL_S)).abs()
                <= f64::EPSILON
        );
    }

    fn routed_outlet_m3(hourly_rainfall_m: [f64; SEAM_HOUR_BINS]) -> f64 {
        let routing = LanedShadowRoutingCoefficients {
            skin_friction_coefficient_ko: 0.25,
            form_drag_coefficient: 0.0,
            roughness_element_height_m: 0.0,
            roughness_concentration: 0.0,
            vegetation_drag_coefficient: 0.0,
        };
        let mut collector = LanedShadowCollector::new(vec![LanedShadowLaneGeometry {
            slplen_m: 80.0,
            width_m: 1.0,
            mean_gradient: 0.01,
            routing,
        }]);
        collector.current_day = Some(0);
        collector.day_depths[0][0] = 0.01;
        collector.day_operands[0] = Some(LanedShadowLaneDayOperands {
            hourly_rainfall_m,
            hourly_routed_melt_m: [0.0; SEAM_HOUR_BINS],
            leaf_area_index: 0.0,
            canopy_height_m: 0.0,
        });

        collector
            .route_buffered_day(0.01)
            .expect("buffered day should route");

        collector.summary.total_routed_outlet_m3
    }
}
