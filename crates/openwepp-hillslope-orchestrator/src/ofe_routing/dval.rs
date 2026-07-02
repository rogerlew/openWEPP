//! MOFEFID-D7 D-val case builders (analysis over the shadow `ofe_routing`
//! kernels — no production wiring). Constructs the four Papanicolaou (2018)
//! validation cases from the authoritative supplemental operands and returns
//! the openWEPP outlet hydrograph, so the CLI dumper (`examples/dval_case`)
//! and the cited-scalar reproduction tests share one definition.
//!
//! **Method-fidelity, not nature.** These runs compare openWEPP to
//! Papanicolaou's *enhanced-WEPP* traces (`NS_trace`); the paper's
//! enhanced-vs-observed `Ef_obs` stays a citation (see `SC-OFEROUTE-001`
//! `INV-OFEROUTE-011`). Green-Ampt soil parameters for Cases 1-3 are
//! texture-derived operands of record — see the package `artifacts/operands.md`.
//! No copyrighted series are embedded; the cited peak scalars in the tests are
//! digitized published values with provenance.

use super::cascade::CascadeSegment;
use super::infiltration::{GreenAmptSoil, RainfallInterval, run_infiltrated_cascade};
use super::kinematic_wave::{
    CellParameters, Forcing, KinematicWaveMesh, KinematicWaveSolver, RoutingError,
};

/// One recorded outlet sample (time, unit discharge).
#[derive(Debug, Clone, Copy)]
pub struct Sample {
    pub time_s: f64,
    pub q_m2_s: f64,
}

/// Result of a D-val case run.
#[derive(Debug, Clone)]
pub struct DvalRun {
    pub hydrograph: Vec<Sample>,
    pub peak_m2_s: f64,
    pub time_to_peak_s: f64,
    pub max_courant: f64,
    /// Event runoff coefficient (Cases 1-3; `None` for the impermeable flume).
    pub runoff_coefficient: Option<f64>,
}

fn mm_h_to_m_s(v: f64) -> f64 {
    v / 3.6e6
}

/// Case 4 (Iwagaki 1955, shock): impermeable 24 m flume, three 8 m sections at
/// 2/1.5/1 %, per-section lateral inflow 0.108/0.0638/0.08 cm/s for 10 s.
/// Single mesh with per-cell slope. `k_o` is unspecified in the paper (operand
/// gap); the caller supplies it.
pub fn run_iwagaki(ko: f64) -> Result<DvalRun, RoutingError> {
    let n = 120usize;
    let dx = 24.0 / f64::from(u32::try_from(n).unwrap_or(u32::MAX));
    let mut cells = Vec::with_capacity(n);
    for i in 0..n {
        let x = (f64::from(u32::try_from(i).unwrap_or(u32::MAX)) + 0.5) * dx;
        let slope = if x < 8.0 {
            0.02
        } else if x < 16.0 {
            0.015
        } else {
            0.01
        };
        cells.push(CellParameters::bare(slope, ko));
    }
    let mut solver = KinematicWaveSolver::new(KinematicWaveMesh {
        cell_length_m: dx,
        cells,
    });
    let dur = 10.0;
    let excess = |i: usize, t: f64| {
        if t > dur {
            return 0.0;
        }
        let x = (f64::from(u32::try_from(i).unwrap_or(u32::MAX)) + 0.5) * dx;
        if x < 8.0 {
            0.108e-2
        } else if x < 16.0 {
            0.0638e-2
        } else {
            0.08e-2
        }
    };
    let inflow = |_t: f64| 0.0;
    // Iwagaki supplies water LATERALLY, not as rainfall: there is no raindrop
    // impact, so the skin-resistance rainfall intensity I is zero (the lateral
    // supply enters as rainfall-excess, above). Feeding the lateral rate into
    // I would spuriously inflate f_s = (3393 I^0.407 + k_o)/Re.
    let intensity = |_t: f64| 0.0;
    let forcing = Forcing {
        rainfall_excess_m_s: &excess,
        upstream_inflow_m2_s: &inflow,
        rainfall_intensity_m_s: &intensity,
    };
    let res = solver.run(&forcing, 80.0, 1.0, 0.5)?;
    Ok(DvalRun {
        hydrograph: res
            .hydrograph
            .iter()
            .map(|s| Sample {
                time_s: s.time_s,
                q_m2_s: s.outlet_unit_discharge_m2_s,
            })
            .collect(),
        peak_m2_s: res.peak_unit_discharge_m2_s,
        time_to_peak_s: res.time_to_peak_s,
        max_courant: res.max_courant,
        runoff_coefficient: None,
    })
}

/// Operands for a rainfall-on-soil validation case (Cases 1-3).
#[derive(Debug, Clone, Copy)]
pub struct RainCase {
    pub intensity_mm_h: f64,
    pub slope: f64,
    pub length_m: f64,
    pub width_m: f64,
    pub cells: usize,
    pub ko: f64,
    /// Texture-derived Green-Ampt operands (operands of record; see operands.md).
    pub ks_mm_h: f64,
    pub psi_m: f64,
    pub dtheta: f64,
    /// Isolated-roughness `(Cd, Dr, lambda)` (Case 2) if present.
    pub form: Option<(f64, f64, f64)>,
    /// Vegetation `(LAI, hc, Cd)` (Case 3) if present.
    pub vegetation: Option<(f64, f64, f64)>,
    pub duration_s: f64,
}

/// Case 1 (Abban 2017, bare): 60 mm/h, 9 %, 7.5x1.2 m, Tama silt/silty-clay
/// loam, 5 h. Green-Ampt Ks 6.8 mm/h (Rawls et al. 1983, silt loam).
#[must_use]
pub fn case1_bare() -> RainCase {
    RainCase {
        intensity_mm_h: 60.0,
        slope: 0.09,
        length_m: 7.5,
        width_m: 1.2,
        cells: 20,
        ko: 500.0,
        ks_mm_h: 6.8,
        psi_m: 0.167,
        dtheta: 0.35,
        form: None,
        vegetation: None,
        duration_s: 5.0 * 3600.0,
    }
}

/// Case 2 (Jomaa 2012, isolated roughness): 74 mm/h, 2.2 %, 6x1 m, sandy/gravel.
#[must_use]
pub fn case2_isolated() -> RainCase {
    RainCase {
        intensity_mm_h: 74.0,
        slope: 0.022,
        length_m: 6.0,
        width_m: 1.0,
        cells: 16,
        ko: 500.0,
        ks_mm_h: 20.0,
        psi_m: 0.11,
        dtheta: 0.30,
        form: Some((1.0, 0.06, 0.2)),
        vegetation: None,
        duration_s: 3.0 * 3600.0,
    }
}

/// Case 3 (Neibling & Alberts 1979, vegetation patchiness): 74 mm/h, 7 %,
/// 6.1x1.8 m, Miami silt loam.
#[must_use]
pub fn case3_vegetation() -> RainCase {
    RainCase {
        intensity_mm_h: 74.0,
        slope: 0.07,
        length_m: 6.1,
        width_m: 1.8,
        cells: 16,
        ko: 500.0,
        ks_mm_h: 6.8,
        psi_m: 0.167,
        dtheta: 0.35,
        form: None,
        vegetation: Some((1.0, 0.1, 1.0)),
        duration_s: 1.0 * 3600.0,
    }
}

/// Run a rainfall-on-soil case through Green-Ampt infiltration + routing.
pub fn run_rain_case(c: &RainCase) -> Result<DvalRun, RoutingError> {
    let mut cell = CellParameters::bare(c.slope, c.ko);
    if let Some((cd, dr, lambda)) = c.form {
        cell.drag_coefficient = cd;
        cell.element_tip_height_m = dr;
        cell.roughness_concentration = lambda;
    }
    if let Some((lai, hc, cd)) = c.vegetation {
        cell.leaf_area_index = lai;
        cell.canopy_height_m = hc;
        cell.vegetation_drag_coefficient = cd;
    }
    let segments = vec![CascadeSegment {
        mesh: KinematicWaveMesh::uniform(c.length_m, c.cells, cell),
        width_m: c.width_m,
    }];
    let soils = vec![GreenAmptSoil {
        saturated_conductivity_m_s: mm_h_to_m_s(c.ks_mm_h),
        suction_head_m: c.psi_m,
        moisture_deficit: c.dtheta,
    }];
    let rainfall = vec![vec![RainfallInterval {
        start_s: 0.0,
        end_s: c.duration_s,
        rate_m_s: mm_h_to_m_s(c.intensity_mm_h),
    }]];
    let end = c.duration_s + 1800.0;
    let res = run_infiltrated_cascade(&segments, &rainfall, &soils, 30.0, end, 30.0, 5.0)?;
    let hydro: Vec<Sample> = res
        .cascade
        .outlet_hydrograph
        .iter()
        .map(|s| Sample {
            time_s: s.time_s,
            q_m2_s: s.outlet_unit_discharge_m2_s,
        })
        .collect();
    Ok(DvalRun {
        peak_m2_s: res.cascade.per_ofe_peak_unit_discharge_m2_s[0],
        time_to_peak_s: res.cascade.time_to_peak_s,
        max_courant: res.cascade.max_courant,
        runoff_coefficient: Some(res.per_ofe_runoff_coefficient[0]),
        hydrograph: hydro,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // Cited enhanced-WEPP peak unit discharges (m^2/s), digitized from
    // Papanicolaou (2018) supplemental Figure_4.xlsx `Enhanced_WEPP`
    // (sha256 2bf68787…d2fe8), physically-consistent columns per the S0
    // cut-point map. Provenance: published values, not vendored series.
    const CITED_ENHANCED_PEAK_CASE1: f64 = 9.451e-5; // Abban, col 11
    const CITED_ENHANCED_PEAK_CASE4: f64 = 8.132e-3; // Iwagaki, col 1

    // Case 1 (bare) reproduces the enhanced-WEPP steady-state MAGNITUDE at the
    // literature Ks (peak +7%; `NS_trace` 0.868, plateau-dominated), but is a
    // PARTIAL verdict: the rising-limb 10-90% rise time is ~40% slow (5000 s vs
    // 3580 s), so it fails the shape co-condition and the fit is Ks-sensitive.
    // This test pins the magnitude reproduction only — see
    // artifacts/execution-report.md for the rise-limb shape gap.
    #[test]
    fn case1_bare_reproduces_steady_magnitude() {
        let run = run_rain_case(&case1_bare()).expect("case1 runs");
        let ratio = run.peak_m2_s / CITED_ENHANCED_PEAK_CASE1;
        assert!(
            (0.9..=1.2).contains(&ratio),
            "Case1 peak {:.3e} vs cited enhanced {:.3e} (ratio {ratio:.2}) outside magnitude band",
            run.peak_m2_s,
            CITED_ENHANCED_PEAK_CASE1
        );
        assert!(run.max_courant <= 1.0 + 1.0e-9);
    }

    // Case 4 (Iwagaki) — NO rain: water is supplied laterally, so the
    // skin-term rainfall intensity is ZERO (see run_iwagaki). Under that
    // correct forcing, openWEPP does not cleanly reproduce enhanced-WEPP
    // (best `NS_trace` ~0.30 at k_o~200), but at that k_o the TIMING and rise
    // shape do reproduce (t_peak ~28 s vs 26 s; rise ~20.6 s vs 20.9 s) — the
    // earlier "solver-side ~5-6 s shock lag" was an ARTIFACT of feeding the
    // lateral rate into I, now withdrawn. The residual (peak ~20% low, moderate
    // NS) is operand-limited on the unspecified flume k_o. This test pins the
    // order-of-magnitude peak and that the timing is NOT grossly lagged.
    #[test]
    fn case4_iwagaki_timing_reproduces_operand_limited_peak() {
        let run = run_iwagaki(200.0).expect("iwagaki runs");
        let ratio = run.peak_m2_s / CITED_ENHANCED_PEAK_CASE4;
        assert!(
            (0.5..=2.0).contains(&ratio),
            "Case4 peak {:.3e} vs cited {:.3e} (ratio {ratio:.2}) not even order-of-magnitude",
            run.peak_m2_s,
            CITED_ENHANCED_PEAK_CASE4
        );
        // Sampled-hydrograph peak time (matches the offline harness metric; the
        // solver's internal `time_to_peak_s` disagrees by ~9 s for Iwagaki — a
        // shock-capture multi-modality noted in the execution report). Under the
        // corrected zero-intensity forcing this is close to the ref ~26 s.
        let sampled_t_peak = run
            .hydrograph
            .iter()
            .fold((0.0_f64, 0.0_f64), |(bt, bq), s| {
                if s.q_m2_s > bq {
                    (s.time_s, s.q_m2_s)
                } else {
                    (bt, bq)
                }
            })
            .0;
        assert!(
            (20.0..=34.0).contains(&sampled_t_peak),
            "Case4 sampled t_peak {sampled_t_peak:.1}s outside the near-reference band"
        );
        assert!(run.max_courant <= 1.0 + 1.0e-9);
    }
}
