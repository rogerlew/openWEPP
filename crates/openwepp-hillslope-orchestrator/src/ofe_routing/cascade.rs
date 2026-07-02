//! MOFEFID Lane D / D5 (SC-OFEROUTE-001, INV-OFEROUTE-008/009): OFE-by-OFE
//! overland-flow cascade. Each OFE is routed with the D4 single-OFE
//! TVD-MacCormack solver; the upstream OFE's outlet hydrograph becomes the
//! downstream OFE's upstream boundary condition (Papanicolaou et al. 2018,
//! assumption 1: route sequentially summit -> outlet). Shadow-first / opt-in;
//! not wired into any production phase span.
//!
//! GAP-OFEROUTE-003 (runon reconciliation with DC01, resolved D6 = SUPERSEDE):
//! this cascade routes SURFACE runon as an hourly hydrograph. When the routing
//! subsystem is active it OWNS the hourly runon supply and SUPERSEDES DC01's
//! daily-lump runon re-infiltration (`SC-RUNOFFPART-001#INV-RUNOFFPART-031`)
//! with hydraulic surface routing. The upstream OFE's outlet hydrograph is a
//! downstream surface boundary condition and is NOT re-infiltrated; per-OFE
//! Green-Ampt infiltration (see `super::infiltration`) acts on the RAINFALL
//! only (Papanicolaou assumption 2). This module implements the routing +
//! handoff; the rainfall->excess coupling is `super::infiltration`.

use super::kinematic_wave::{
    Forcing, HydrographSample, KinematicWaveMesh, KinematicWaveSolver, RoutingError,
};

/// One OFE in the cascade: its routing mesh and flow width. The flow width
/// converts unit-width discharge (m^2/s) between OFEs so total discharge
/// `Q = q * width` is continuous across the handoff.
#[derive(Debug, Clone)]
pub struct CascadeSegment {
    pub mesh: KinematicWaveMesh,
    /// OFE flow width (m).
    pub width_m: f64,
}

/// Per-OFE forcing supplied to the cascade run.
pub struct CascadeForcing<'a> {
    /// Rainfall-excess rate at `(ofe_index, cell_index, time_s)` -> m/s.
    pub rainfall_excess_m_s: &'a dyn Fn(usize, usize, f64) -> f64,
    /// Rainfall intensity at `(ofe_index, time_s)` -> m/s (skin term).
    pub rainfall_intensity_m_s: &'a dyn Fn(usize, f64) -> f64,
}

/// Width-aware cascade mass balance (total volume, m^3).
#[derive(Debug, Clone, Copy, Default)]
pub struct CascadeMassBalance {
    /// Total rainfall-excess volume over all OFEs (m^3).
    pub rainfall_excess_m3: f64,
    /// Outlet volume leaving the terminal OFE (m^3).
    pub outlet_m3: f64,
    /// Storage change summed across all OFEs (m^3).
    pub storage_change_m3: f64,
    /// Positivity-clamp injection summed across all OFEs (m^3).
    pub positivity_clamp_m3: f64,
}

impl CascadeMassBalance {
    /// Clamp-adjusted cascade conservation residual (m^3): should be ~0.
    #[must_use]
    pub fn conservation_residual_m3(&self) -> f64 {
        self.rainfall_excess_m3 - self.outlet_m3 - self.storage_change_m3 + self.positivity_clamp_m3
    }
}

/// Result of a cascade run.
#[derive(Debug, Clone)]
pub struct CascadeResult {
    /// Terminal-OFE outlet hydrograph (unit discharge at the cascade outlet).
    pub outlet_hydrograph: Vec<HydrographSample>,
    /// Peak outlet total discharge (m^3/s) and its time.
    pub peak_total_discharge_m3_s: f64,
    pub time_to_peak_s: f64,
    /// Per-OFE peak unit discharge (m^2/s), upstream to downstream.
    pub per_ofe_peak_unit_discharge_m2_s: Vec<f64>,
    /// Per-OFE outlet volume (m^3), upstream to downstream.
    pub per_ofe_outlet_m3: Vec<f64>,
    /// Per-OFE received upstream (runon) volume (m^3); OFE 0 is 0.
    pub per_ofe_received_upstream_m3: Vec<f64>,
    pub mass_balance: CascadeMassBalance,
    /// Max Courant number over the whole cascade (CFL evidence).
    pub max_courant: f64,
}

/// Linearly interpolate an outlet hydrograph's unit discharge at `time_s`.
/// Clamps to the endpoints outside the recorded range.
fn interpolate_unit_discharge(hydrograph: &[HydrographSample], time_s: f64) -> f64 {
    if hydrograph.is_empty() {
        return 0.0;
    }
    if time_s <= hydrograph[0].time_s {
        return hydrograph[0].outlet_unit_discharge_m2_s.max(0.0);
    }
    let last = hydrograph[hydrograph.len() - 1];
    if time_s >= last.time_s {
        return last.outlet_unit_discharge_m2_s.max(0.0);
    }
    // binary search for the bracketing interval
    let mut lo = 0usize;
    let mut hi = hydrograph.len() - 1;
    while hi - lo > 1 {
        let mid = lo.midpoint(hi);
        if hydrograph[mid].time_s <= time_s {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let a = hydrograph[lo];
    let b = hydrograph[hi];
    let span = b.time_s - a.time_s;
    if span <= 0.0 {
        return a.outlet_unit_discharge_m2_s.max(0.0);
    }
    let frac = (time_s - a.time_s) / span;
    (a.outlet_unit_discharge_m2_s
        + frac * (b.outlet_unit_discharge_m2_s - a.outlet_unit_discharge_m2_s))
        .max(0.0)
}

/// Trapezoidal integral of a hydrograph's unit discharge over its samples
/// (m^2, per unit width).
fn hydrograph_volume_m2(hydrograph: &[HydrographSample]) -> f64 {
    let mut total = 0.0;
    for w in hydrograph.windows(2) {
        let dt = w[1].time_s - w[0].time_s;
        if dt > 0.0 {
            total += 0.5 * (w[0].outlet_unit_discharge_m2_s + w[1].outlet_unit_discharge_m2_s) * dt;
        }
    }
    total
}

/// Run the OFE-by-OFE cascade summit -> outlet. Each OFE is routed with the
/// D4 solver; OFE `i`'s outlet hydrograph, scaled by the width ratio
/// `w_i / w_{i+1}` for discharge continuity, is OFE `i+1`'s upstream boundary.
pub fn run_cascade(
    segments: &[CascadeSegment],
    forcing: &CascadeForcing<'_>,
    end_time_s: f64,
    sample_dt_s: f64,
    max_dt_s: f64,
) -> Result<CascadeResult, RoutingError> {
    if segments.is_empty() {
        return Err(RoutingError::DegenerateConfiguration);
    }
    for seg in segments {
        if !seg.width_m.is_finite() || seg.width_m <= 0.0 {
            return Err(RoutingError::DegenerateConfiguration);
        }
    }

    let mut mass = CascadeMassBalance::default();
    let mut per_ofe_peak = Vec::with_capacity(segments.len());
    let mut per_ofe_outlet_m3 = Vec::with_capacity(segments.len());
    let mut per_ofe_received_m3 = Vec::with_capacity(segments.len());
    let mut prev_hydrograph: Option<Vec<HydrographSample>> = None;
    let mut prev_width = 0.0_f64;
    let mut outlet_hydrograph: Vec<HydrographSample> = Vec::new();
    let mut max_courant = 0.0_f64;

    for (i, seg) in segments.iter().enumerate() {
        // Upstream boundary from the previous OFE's outlet, scaled to this
        // OFE's width for total-discharge continuity. `prev` is an owned local
        // so the forcing closure borrows it cleanly within this iteration.
        let prev = prev_hydrograph.take();
        let width_ratio = if prev.is_some() {
            prev_width / seg.width_m
        } else {
            0.0
        };
        let upstream = |t: f64| -> f64 {
            match &prev {
                Some(h) => interpolate_unit_discharge(h, t) * width_ratio,
                None => 0.0,
            }
        };
        let excess = |cell: usize, t: f64| (forcing.rainfall_excess_m_s)(i, cell, t);
        let intensity = |t: f64| (forcing.rainfall_intensity_m_s)(i, t);
        let ofe_forcing = Forcing {
            rainfall_excess_m_s: &excess,
            upstream_inflow_m2_s: &upstream,
            rainfall_intensity_m_s: &intensity,
        };

        let mut solver = KinematicWaveSolver::new(seg.mesh.clone());
        let result = solver.run(&ofe_forcing, end_time_s, sample_dt_s, max_dt_s)?;

        // Received upstream (runon) volume into this OFE (m^3): the width-scaled
        // upstream unit-discharge integral times this OFE's width.
        let received_m2 = result.mass_balance.inflow_m2;
        per_ofe_received_m3.push(received_m2 * seg.width_m);
        // Rainfall excess volume for this OFE (m^3).
        mass.rainfall_excess_m3 += result.mass_balance.rainfall_excess_m2 * seg.width_m;
        mass.storage_change_m3 += result.mass_balance.storage_change_m2 * seg.width_m;
        mass.positivity_clamp_m3 += result.mass_balance.positivity_clamp_m2 * seg.width_m;
        per_ofe_peak.push(result.peak_unit_discharge_m2_s);
        per_ofe_outlet_m3.push(hydrograph_volume_m2(&result.hydrograph) * seg.width_m);
        if result.max_courant > max_courant {
            max_courant = result.max_courant;
        }

        // Hand off to the next OFE (move, no clone): the terminal OFE's
        // hydrograph is the cascade outlet; interior OFEs feed the next.
        if i + 1 < segments.len() {
            prev_hydrograph = Some(result.hydrograph);
            prev_width = seg.width_m;
        } else {
            outlet_hydrograph = result.hydrograph;
        }
    }

    // Cascade outlet volume: terminal OFE outlet integral times its width.
    let terminal_width = segments[segments.len() - 1].width_m;
    mass.outlet_m3 = hydrograph_volume_m2(&outlet_hydrograph) * terminal_width;

    // Peak total discharge (m^3/s) at the cascade outlet.
    let mut peak_total_discharge = 0.0_f64;
    let mut time_to_peak = 0.0_f64;
    for s in &outlet_hydrograph {
        let total = s.outlet_unit_discharge_m2_s * terminal_width;
        if total > peak_total_discharge {
            peak_total_discharge = total;
            time_to_peak = s.time_s;
        }
    }

    Ok(CascadeResult {
        outlet_hydrograph,
        peak_total_discharge_m3_s: peak_total_discharge,
        time_to_peak_s: time_to_peak,
        per_ofe_peak_unit_discharge_m2_s: per_ofe_peak,
        per_ofe_outlet_m3,
        per_ofe_received_upstream_m3: per_ofe_received_m3,
        mass_balance: mass,
        max_courant,
    })
}

#[cfg(test)]
mod tests {
    use super::super::kinematic_wave::CellParameters;
    use super::*;

    fn bare_segment(length: f64, cells: usize, slope: f64, ko: f64, width: f64) -> CascadeSegment {
        CascadeSegment {
            mesh: KinematicWaveMesh::uniform(length, cells, CellParameters::bare(slope, ko)),
            width_m: width,
        }
    }

    // Two-OFE bare cascade: total rainfall excess must equal the terminal
    // outlet plus total storage (width-aware), and the handoff volume must
    // carry OFE 1's outlet into OFE 2 as received runon.
    #[test]
    fn two_ofe_cascade_conserves_and_hands_off() {
        let v = 60.0 / 3.6e6;
        let segs = vec![
            bare_segment(10.0, 20, 0.08, 500.0, 2.0),
            bare_segment(10.0, 20, 0.06, 500.0, 2.0),
        ];
        let excess = |_ofe: usize, _cell: usize, _t: f64| v;
        let intensity = |_ofe: usize, _t: f64| v;
        let forcing = CascadeForcing {
            rainfall_excess_m_s: &excess,
            rainfall_intensity_m_s: &intensity,
        };
        let res = run_cascade(&segs, &forcing, 3600.0, 10.0, 2.0).expect("cascade runs");

        // clamp-adjusted cascade conservation ~ 0 (relative to rainfall input)
        let rain = res.mass_balance.rainfall_excess_m3;
        assert!(rain > 0.0);
        assert!(
            res.mass_balance.conservation_residual_m3().abs() / rain < 1.0e-2,
            "cascade residual {} vs rain {rain}",
            res.mass_balance.conservation_residual_m3()
        );
        // OFE 2 receives OFE 1's outlet as runon (nonzero), width ratio 1.0
        assert!(
            res.per_ofe_received_upstream_m3[0].abs() < 1.0e-12,
            "top OFE has no runon"
        );
        assert!(
            res.per_ofe_received_upstream_m3[1] > 0.0,
            "OFE 2 must receive runon"
        );
        assert!(
            (res.per_ofe_received_upstream_m3[1] - res.per_ofe_outlet_m3[0]).abs()
                / res.per_ofe_outlet_m3[0]
                < 1.0e-2,
            "runon into OFE 2 must equal OFE 1 outlet (equal widths): received {} vs outlet {}",
            res.per_ofe_received_upstream_m3[1],
            res.per_ofe_outlet_m3[0]
        );
        assert!(res.max_courant <= 1.0 + 1.0e-9);
    }

    // Three-OFE cascade: discharge accumulates downslope (each OFE's outlet >=
    // the previous, given added rainfall), and CFL holds.
    #[test]
    fn three_ofe_cascade_accumulates_downslope() {
        let v = 74.0 / 3.6e6;
        let segs = vec![
            bare_segment(8.0, 16, 0.07, 500.0, 1.0),
            bare_segment(8.0, 16, 0.07, 500.0, 1.0),
            bare_segment(8.0, 16, 0.07, 500.0, 1.0),
        ];
        let excess = |_o: usize, _c: usize, _t: f64| v;
        let intensity = |_o: usize, _t: f64| v;
        let forcing = CascadeForcing {
            rainfall_excess_m_s: &excess,
            rainfall_intensity_m_s: &intensity,
        };
        let res = run_cascade(&segs, &forcing, 3600.0, 10.0, 2.0).expect("cascade runs");
        // steady outlet unit discharge grows downslope (more contributing length)
        let peaks = &res.per_ofe_peak_unit_discharge_m2_s;
        assert!(
            peaks[1] > peaks[0] && peaks[2] > peaks[1],
            "peaks accumulate: {peaks:?}"
        );
        // terminal steady discharge ~ v * total_length (3 * 8 = 24 m)
        let q_terminal = res
            .outlet_hydrograph
            .last()
            .unwrap()
            .outlet_unit_discharge_m2_s;
        assert!(
            (q_terminal - v * 24.0).abs() / (v * 24.0) < 0.05,
            "terminal steady q {q_terminal} vs v*L {}",
            v * 24.0
        );
        assert!(res.max_courant <= 1.0 + 1.0e-9);
    }

    // Case 3 (vegetation patchiness): the robust, roughness-monotone signature
    // of the filter-strip effect. At steady state the outlet DISCHARGE is fixed
    // by mass balance (v*L) regardless of roughness, so vegetation cannot change
    // the steady peak; what it changes is STORAGE - added resistance backs up
    // more water (higher steady depth) and the discharge-depth relation
    // `q = alpha h^1.5` with lower alpha requires a higher h for the same q.
    // This is monotone in f_eq and independent of storm/concentration timing.
    #[test]
    fn case3_vegetated_strip_backs_up_more_water_than_bare() {
        let v = 74.0 / 3.6e6;
        let excess = |_o: usize, _c: usize, _t: f64| v;
        let intensity = |_o: usize, _t: f64| v;
        let forcing = CascadeForcing {
            rainfall_excess_m_s: &excess,
            rainfall_intensity_m_s: &intensity,
        };

        let bare = vec![
            bare_segment(6.1, 24, 0.07, 500.0, 3.6),
            bare_segment(2.45, 12, 0.07, 500.0, 3.6),
        ];
        let bare_res = run_cascade(&bare, &forcing, 3600.0, 5.0, 1.0).expect("bare runs");

        // vegetated downslope strip (Case 3 params: LAI=1, h_c=0.1, C_d=1.0)
        let mut veg_cell = CellParameters::bare(0.07, 500.0);
        veg_cell.leaf_area_index = 1.0;
        veg_cell.canopy_height_m = 0.1;
        veg_cell.vegetation_drag_coefficient = 1.0;
        let veg = vec![
            bare_segment(6.1, 24, 0.07, 500.0, 3.6),
            CascadeSegment {
                mesh: KinematicWaveMesh::uniform(2.45, 12, veg_cell),
                width_m: 3.6,
            },
        ];
        let veg_res = run_cascade(&veg, &forcing, 3600.0, 5.0, 1.0).expect("veg runs");

        let bare_depth = bare_res.outlet_hydrograph.last().unwrap().outlet_depth_m;
        let veg_depth = veg_res.outlet_hydrograph.last().unwrap().outlet_depth_m;
        let bare_q = bare_res
            .outlet_hydrograph
            .last()
            .unwrap()
            .outlet_unit_discharge_m2_s;
        let veg_q = veg_res
            .outlet_hydrograph
            .last()
            .unwrap()
            .outlet_unit_discharge_m2_s;

        // steady discharge is the same (mass balance): roughness does not change it
        assert!(
            (veg_q - bare_q).abs() / bare_q < 1.0e-2,
            "steady outlet discharge must match (mass balance): veg {veg_q} bare {bare_q}"
        );
        // but the vegetated strip backs up more water: higher steady depth
        // (directional, monotone in f_eq; ~1.7% for LAI=1 light canopy at this
        // shallow flow depth - small but unambiguously above float noise).
        assert!(
            veg_depth > bare_depth * 1.005,
            "vegetated strip must back up more water (higher steady depth): veg {veg_depth} bare {bare_depth}"
        );
        // both conserve and hold CFL
        assert!(veg_res.max_courant <= 1.0 + 1.0e-9);
        assert!(
            veg_res.mass_balance.conservation_residual_m3().abs()
                / veg_res.mass_balance.rainfall_excess_m3
                < 1.0e-2
        );
    }

    #[test]
    fn width_change_scales_handoff_for_discharge_continuity() {
        // OFE 1 width 2.0 -> OFE 2 width 1.0: unit discharge doubles at the
        // handoff so total discharge Q = q*w is continuous.
        let v = 60.0 / 3.6e6;
        let segs = vec![
            bare_segment(10.0, 20, 0.08, 500.0, 2.0),
            bare_segment(10.0, 20, 0.08, 500.0, 1.0),
        ];
        let excess = |_o: usize, _c: usize, _t: f64| v;
        let intensity = |_o: usize, _t: f64| v;
        let forcing = CascadeForcing {
            rainfall_excess_m_s: &excess,
            rainfall_intensity_m_s: &intensity,
        };
        let res = run_cascade(&segs, &forcing, 3600.0, 10.0, 2.0).expect("cascade runs");
        // total discharge continuity: OFE 2 received volume (m^3) == OFE 1
        // outlet volume (m^3), independent of width (volumes are width-aware).
        assert!(
            (res.per_ofe_received_upstream_m3[1] - res.per_ofe_outlet_m3[0]).abs()
                / res.per_ofe_outlet_m3[0]
                < 1.0e-2,
            "width-scaled handoff must conserve total volume"
        );
    }

    #[test]
    fn degenerate_cascade_fails_closed() {
        let excess = |_o: usize, _c: usize, _t: f64| 0.0;
        let intensity = |_o: usize, _t: f64| 0.0;
        let forcing = CascadeForcing {
            rainfall_excess_m_s: &excess,
            rainfall_intensity_m_s: &intensity,
        };
        assert!(matches!(
            run_cascade(&[], &forcing, 100.0, 10.0, 2.0),
            Err(RoutingError::DegenerateConfiguration)
        ));
        let bad_width = vec![bare_segment(10.0, 10, 0.05, 100.0, 0.0)];
        assert!(matches!(
            run_cascade(&bad_width, &forcing, 100.0, 10.0, 2.0),
            Err(RoutingError::DegenerateConfiguration)
        ));
    }
}
