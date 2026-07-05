//! Lane D runtime seam SHADOW (`SC-OFEROUTE-001#INV-OFEROUTE-012`
//! activation increment, opt-in via `OPENWEPP_LANED_SHADOW=1`):
//! reconstructs each lane-day's routed source series from the LIVE
//! published surfaces (`dc01_surface_hourly_weights × runvol/area` — the
//! ADR-0036 weights-times-total hourly-flow authority on the lane-local
//! runoff-volume basis over exactly the two GAP-006 D1 limbs), routes event
//! days through the real
//! `ofe_routing::cascade`, and accumulates conservation diagnostics
//! into the manifest. DIAGNOSTIC ONLY: water authority stays DC01;
//! protected outputs are byte-identical with the shadow on or off;
//! production activation remains BLOCKED (`INV-OFEROUTE-011` open).
//!
//! Friction operands are the LABELED bare-cell first cut
//! (`GAP-OFEROUTE-007`): `k_o = 500` (the R-63 base grain roughness
//! used across the paper's validation cases); form/wave/vegetation
//! elements are not sourced from WEPP inputs yet.

use openwepp_hillslope_orchestrator::DirectPublicationDayRow;
use openwepp_hillslope_orchestrator::ofe_routing::cascade::{
    CascadeForcing, CascadeSegment, run_cascade,
};
use openwepp_hillslope_orchestrator::ofe_routing::kinematic_wave::{
    CellParameters, KinematicWaveMesh,
};
use openwepp_hillslope_orchestrator::ofe_routing::seam::{
    SEAM_HOUR_BINS, seam_rate_at, seam_source_rates_from_hourly_depths,
};

/// The GAP-OFEROUTE-007 labeled first-cut base grain roughness.
const LANED_SHADOW_KO: f64 = 500.0;
/// Days below this injected volume are excluded from the MAX relative
/// residual (they still fold into the volume-weighted aggregate).
const RESIDUAL_FLOOR_M3: f64 = 0.1;
/// Cells per OFE for the shadow mesh (the D-val working resolution).
const LANED_SHADOW_CELLS: usize = 10;
/// Routing window: one day.
const LANED_SHADOW_WINDOW_S: f64 = 24.0 * 3600.0;
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
    residual_abs_m3: f64,
}

pub(crate) struct LanedShadowCollector {
    geometry: Vec<LanedShadowLaneGeometry>,
    day_depths: Vec<[f64; SEAM_HOUR_BINS]>,
    day_source_m3: f64,
    day_supply_reference_m3: f64,
    lanes_seen_today: usize,
    day_saw_uniform_shape: bool,
    current_day: Option<usize>,
    summary: LanedShadowSummary,
}

impl LanedShadowCollector {
    #[must_use]
    pub(crate) fn new(geometry: Vec<LanedShadowLaneGeometry>) -> Self {
        let lane_count = geometry.len();
        Self {
            geometry,
            day_depths: vec![[0.0; SEAM_HOUR_BINS]; lane_count],
            day_source_m3: 0.0,
            day_supply_reference_m3: 0.0,
            lanes_seen_today: 0,
            day_saw_uniform_shape: false,
            current_day: None,
            summary: LanedShadowSummary::default(),
        }
    }

    /// Env opt-in: `OPENWEPP_LANED_SHADOW=1`.
    #[must_use]
    pub(crate) fn env_enabled() -> bool {
        std::env::var("OPENWEPP_LANED_SHADOW").is_ok_and(|value| value == "1")
    }

    /// Observe one published lane-day row (stream order: day-major,
    /// lane-minor). Commits the previous day when the day index moves.
    pub(crate) fn observe_row(&mut self, row: &DirectPublicationDayRow) -> Result<(), String> {
        if self.current_day.is_some_and(|day| day != row.day_index) {
            self.commit_day()?;
        }
        self.current_day = Some(row.day_index);
        let lane = row.lane_index;
        if lane >= self.geometry.len() {
            return Err(format!(
                "laned shadow: lane index {lane} outside geometry ({})",
                self.geometry.len()
            ));
        }
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
        self.day_source_m3 += depth_sum * area_m2;
        self.day_supply_reference_m3 += row.runoff.runvol_m3;
        self.lanes_seen_today += 1;
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
            let segments: Vec<CascadeSegment> = self
                .geometry
                .iter()
                .map(|geom| CascadeSegment {
                    mesh: KinematicWaveMesh::uniform(
                        geom.slplen_m,
                        LANED_SHADOW_CELLS,
                        CellParameters::bare(geom.mean_gradient, LANED_SHADOW_KO),
                    ),
                    width_m: geom.width_m,
                })
                .collect();
            let day_depths = std::mem::replace(
                &mut self.day_depths,
                vec![[0.0; SEAM_HOUR_BINS]; self.geometry.len()],
            );
            let mut rate_series = Vec::with_capacity(day_depths.len());
            for depths in &day_depths {
                rate_series.push(
                    seam_source_rates_from_hourly_depths(depths)
                        .map_err(|error| format!("laned shadow seam rates: {error:?}"))?,
                );
            }
            let excess = |ofe: usize, _cell: usize, t: f64| seam_rate_at(&rate_series[ofe], t);
            // Shadow first cut: no rainfall-intensity operand is
            // threaded (skin term I=0); GAP-OFEROUTE-007 labels this.
            let intensity = |_ofe: usize, _t: f64| 0.0;
            let forcing = CascadeForcing {
                rainfall_excess_m_s: &excess,
                rainfall_intensity_m_s: &intensity,
            };
            // Clip the routing window to the active source span (+ a
            // 6 h drain tail) — dry-hour substeps dominate cost on a
            // full-day window otherwise.
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
            #[allow(clippy::cast_precision_loss)]
            let window_s = (((last_active_hour + 1) as f64) * 3600.0 + 6.0 * 3600.0)
                .min(LANED_SHADOW_WINDOW_S);
            let result = run_cascade(
                &segments,
                &forcing,
                window_s,
                LANED_SHADOW_SAMPLE_DT_S,
                LANED_SHADOW_MAX_DT_S,
            )
            .map_err(|error| format!("laned shadow cascade: {error:?}"))?;
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
        } else {
            for depths in &mut self.day_depths {
                *depths = [0.0; SEAM_HOUR_BINS];
            }
        }
        if self.day_saw_uniform_shape {
            self.summary.days_uniform_shape += 1;
        }
        self.day_source_m3 = 0.0;
        self.day_supply_reference_m3 = 0.0;
        self.lanes_seen_today = 0;
        self.day_saw_uniform_shape = false;
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
        Ok(self.summary)
    }
}
