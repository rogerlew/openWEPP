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
//! Static friction operands are sourced from the openWEPP native
//! `routing_coefficients` management extension when the shadow is enabled;
//! missing extension data fails closed before streaming starts. Dynamic
//! rainfall-intensity and canopy-state operands are sourced from the live
//! direct day frame: WB14 hourly rainfall depth, post-growth LAI, and
//! typed-management canopy height.

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
    fn cell_parameters(self, slope: f64, operands: LanedShadowLaneDayOperands) -> CellParameters {
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
#[derive(Debug, Clone, Copy)]
pub(crate) struct LanedShadowLaneDayOperands {
    pub hourly_rainfall_m: [f64; SEAM_HOUR_BINS],
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
    residual_abs_m3: f64,
}

pub(crate) struct LanedShadowCollector {
    geometry: Vec<LanedShadowLaneGeometry>,
    day_depths: Vec<[f64; SEAM_HOUR_BINS]>,
    day_operands: Vec<Option<LanedShadowLaneDayOperands>>,
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
            day_operands: vec![None; lane_count],
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
    pub(crate) fn observe_row(
        &mut self,
        row: &DirectPublicationDayRow,
        operands: LanedShadowLaneDayOperands,
    ) -> Result<(), String> {
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
        Self::validate_lane_day_operands(row, operands)?;
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
        self.day_operands[lane] = Some(operands);
        self.day_source_m3 += depth_sum * area_m2;
        self.day_supply_reference_m3 += row.runoff.runvol_m3;
        self.lanes_seen_today += 1;
        Ok(())
    }

    fn validate_lane_day_operands(
        row: &DirectPublicationDayRow,
        operands: LanedShadowLaneDayOperands,
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
        }
        self.day_source_m3 = 0.0;
        self.day_supply_reference_m3 = 0.0;
        self.lanes_seen_today = 0;
        self.day_saw_uniform_shape = false;
        Ok(())
    }

    fn route_buffered_day(&mut self, source_m3: f64) -> Result<(), String> {
        let day_operands =
            std::mem::replace(&mut self.day_operands, vec![None; self.geometry.len()]);
        let segments: Vec<CascadeSegment> = self
            .geometry
            .iter()
            .enumerate()
            .map(|(lane_index, geom)| {
                let operands = day_operands[lane_index].ok_or_else(|| {
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
            .collect::<Result<Vec<_>, String>>()?;
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
        let mut intensity_series = Vec::with_capacity(day_operands.len());
        for operands in &day_operands {
            let operands = operands.ok_or_else(|| {
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
        #[allow(clippy::cast_precision_loss)]
        let window_s =
            (((last_active_hour + 1) as f64) * 3600.0 + 6.0 * 3600.0).min(LANED_SHADOW_WINDOW_S);
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
            leaf_area_index: 2.5,
            canopy_height_m: 0.8,
        };

        let cell = routing.cell_parameters(0.04, operands);

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
            leaf_area_index: 0.0,
            canopy_height_m: 0.0,
        });

        collector
            .route_buffered_day(0.01)
            .expect("buffered day should route");

        collector.summary.total_routed_outlet_m3
    }
}
