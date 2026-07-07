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
    CascadeSegment, UpstreamHandoff, route_single_ofe, route_single_ofe_hybrid,
};
use crate::ofe_routing::kinematic_wave::{CellParameters, KinematicWaveMesh, RoutingResult};
use crate::ofe_routing::seam::{
    SEAM_HOUR_BINS, SEAM_SECONDS_PER_HOUR, seam_rate_at, seam_source_rates_from_hourly_depths,
};

/// Cells per OFE for the active mesh (the D-val working resolution — the
/// same constant the diagnostic shadow uses).
pub(crate) const LANED_ACTIVE_CELLS: usize = 10;
/// Outlet-bin sample width (s); 3600/900 = 4 bins per hour, so the erosion
/// hourly mapping is exact hour-aligned bin sums.
pub(crate) const LANED_ACTIVE_SAMPLE_DT_S: f64 = 900.0;
/// CFL step cap (s).
pub(crate) const LANED_ACTIVE_MAX_DT_S: f64 = 300.0;
/// One day of hourly source (s).
pub(crate) const LANED_ACTIVE_SOURCE_WINDOW_S: f64 = 24.0 * 3600.0;
/// Drain tail after the last active source hour (rev-27 window row).
pub(crate) const LANED_ACTIVE_DRAIN_TAIL_S: f64 = 6.0 * 3600.0;
/// Rev-27 tolerance: per-lane-day supply reconstruction (relative).
pub(crate) const LANED_ACTIVE_SUPPLY_REL_TOL: f64 = 1.0e-9;
/// Rev-27 tolerance: per-day clamp-adjusted cascade residual (relative).
pub(crate) const LANED_ACTIVE_CASCADE_REL_TOL: f64 = 1.0e-9;
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

/// Run-level active configuration, attached to the run frame by the runner
/// when `OPENWEPP_LANED_ACTIVE=1`. Its presence IS the activation selector
/// inside the orchestrator.
#[derive(Debug, Clone, PartialEq)]
pub struct DirectLanedActiveConfig {
    pub lanes: Vec<DirectLanedActiveLaneConfig>,
    /// T3 (rev 28/30, EXPERIMENTAL opt-in `OPENWEPP_LANED_ACTIVE_IMPLICIT=1`
    /// composing with the active selector): route lanes with HYBRID
    /// stepping — implicit backward-Euler on AGGRESSIVE smooth bins (zero
    /// source on every cell; upstream inflow booked exactly by the implicit
    /// step — the rev-30 rule superseding rev-28 strict), the explicit D10B
    /// scheme elsewhere, with the rev-30 cross-span deficit carry. Default
    /// false: rev-27 behavior unchanged.
    pub hybrid_implicit: bool,
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
        Ok(())
    }
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
}

/// Run-level evidence summary accumulated by the executor and surfaced by
/// the runner manifest.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct DirectLanedActiveRunSummary {
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

/// Route one active lane-day: rev-21 operand validation, the shared
/// single-OFE cascade routine, the D13 erosion producer flip, the day-frame
/// evidence record, and the day-books fold. Returns the conservative
/// upstream handoff for the next lane.
#[allow(clippy::too_many_lines, clippy::too_many_arguments)]
pub(crate) fn laned_active_route_lane(
    day_frame: &mut DirectDayFrame,
    lane_config: &DirectLanedActiveLaneConfig,
    area_m2: f64,
    upstream: Option<&UpstreamHandoff>,
    window_s: f64,
    books: &mut DirectLanedActiveDayBooks,
    source: &LanedActiveLaneSource,
    hybrid_implicit: bool,
) -> Result<UpstreamHandoff, DirectRuntimeError> {
    validate_finite("laned_active.area_m2", area_m2)?;
    if area_m2 <= 0.0 {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "laned_active.area_m2",
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
    let segment = CascadeSegment {
        mesh: KinematicWaveMesh::uniform(lane_config.slplen_m, LANED_ACTIVE_CELLS, cell),
        width_m: lane_config.width_m,
    };

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
    let result = if hybrid_implicit {
        route_single_ofe_hybrid(
            &segment,
            &source_rates,
            &intensity_rates,
            upstream,
            &breakpoints[..breakpoint_count],
            window_s,
            LANED_ACTIVE_SAMPLE_DT_S,
            LANED_ACTIVE_MAX_DT_S,
        )
    } else {
        route_single_ofe(
            &segment,
            &excess,
            &intensity,
            upstream,
            &breakpoints[..breakpoint_count],
            window_s,
            LANED_ACTIVE_SAMPLE_DT_S,
            LANED_ACTIVE_MAX_DT_S,
        )
    }
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
            1.0,
            None,
            3600.0,
            &mut books,
            &dry_lane_source(),
            false,
        )
        .expect("positive post-growth canhgt should satisfy the vegetated guard");

        let mut stale_static_day = vegetated_day_with_post_growth_canhgt(0.0);
        let mut stale_static_books = DirectLanedActiveDayBooks::default();
        assert!(matches!(
            laned_active_route_lane(
                &mut stale_static_day,
                &lane_config_with_static_canhgt(Some(0.75)),
                1.0,
                None,
                3600.0,
                &mut stale_static_books,
                &dry_lane_source(),
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
