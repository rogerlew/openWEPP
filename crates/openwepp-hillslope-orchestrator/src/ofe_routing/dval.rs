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
    /// Sampled-hydrograph peak (D-val comparison metric).
    pub peak_m2_s: f64,
    /// Sampled-hydrograph peak time (D-val comparison metric).
    pub time_to_peak_s: f64,
    /// Solver sub-step peak diagnostic, when exposed by the underlying run.
    pub diagnostic_substep_peak_m2_s: Option<f64>,
    /// Solver sub-step peak-time diagnostic, when exposed by the underlying run.
    pub diagnostic_substep_time_to_peak_s: Option<f64>,
    pub max_courant: f64,
    /// Event runoff coefficient (Cases 1-3; `None` for the impermeable flume).
    pub runoff_coefficient: Option<f64>,
    /// D10B (rev 24): booked TVD boundary-leak mass (m^2 per unit width),
    /// exposed by the single-OFE Case-4 paths for the mass-neutrality gate.
    pub diagnostic_tvd_boundary_leak_m2: Option<f64>,
    /// D10B (rev 24): max homogeneous-step total-variation increase (m^2/s).
    pub diagnostic_max_homogeneous_tv_increase_m2_s: Option<f64>,
    /// Codex review High-2: the solver's booked lateral-source total
    /// (m^2 per unit width), exposed by the Case-4 paths so tests can
    /// prove the discrete source history matches the oracle configuration.
    pub diagnostic_rainfall_excess_m2: Option<f64>,
}

fn mm_h_to_m_s(v: f64) -> f64 {
    v / 3.6e6
}

fn sampled_peak(hydrograph: &[Sample]) -> (f64, f64) {
    hydrograph.iter().fold((0.0_f64, 0.0_f64), |best, s| {
        if s.q_m2_s > best.1 {
            (s.time_s, s.q_m2_s)
        } else {
            best
        }
    })
}

fn sampled_crossing(samples: &[Sample], target: f64) -> Option<f64> {
    for w in samples.windows(2) {
        let a = w[0];
        let b = w[1];
        if (a.q_m2_s <= target && b.q_m2_s >= target) || (a.q_m2_s >= target && b.q_m2_s <= target)
        {
            let dq = b.q_m2_s - a.q_m2_s;
            if dq.abs() <= f64::EPSILON {
                return Some(a.time_s);
            }
            let frac = (target - a.q_m2_s) / dq;
            return Some(a.time_s + frac * (b.time_s - a.time_s));
        }
    }
    None
}

/// Sampled-hydrograph 10-90% rising-limb duration for D-val diagnostics.
#[must_use]
pub fn sampled_rise_time_10_90(hydrograph: &[Sample]) -> Option<f64> {
    let (t_peak, q_peak) = sampled_peak(hydrograph);
    if q_peak <= 0.0 {
        return None;
    }
    let rising: Vec<Sample> = hydrograph
        .iter()
        .copied()
        .take_while(|s| s.time_s <= t_peak)
        .collect();
    if rising.len() < 2 {
        return None;
    }
    let t10 = sampled_crossing(&rising, 0.1 * q_peak)?;
    let t90 = sampled_crossing(&rising, 0.9 * q_peak)?;
    Some(t90 - t10)
}

/// Case 4 (Iwagaki 1955, shock): impermeable 24 m flume, three 8 m sections at
/// 2/1.5/1 %, per-section lateral inflow 0.108/0.0638/0.08 cm/s for 10 s.
/// Single mesh with per-cell slope. `k_o` is unspecified in the paper (operand
/// gap); the caller supplies it.
pub fn run_iwagaki(ko: f64) -> Result<DvalRun, RoutingError> {
    run_iwagaki_with_options(ko, 120, 1.0, 0.5)
}

/// Case 4 with explicit numerical controls for D8 diagnostics.
pub fn run_iwagaki_with_options(
    ko: f64,
    cell_count: usize,
    sample_dt_s: f64,
    max_dt_s: f64,
) -> Result<DvalRun, RoutingError> {
    run_iwagaki_cells(
        |slope| CellParameters::bare(slope, ko),
        cell_count,
        sample_dt_s,
        max_dt_s,
    )
}

/// Case 4 under the rev-24 acceptance configuration: the primary's own
/// Manning law (`n = 0.009`, m-s units) via the definitional
/// `f = 8 g n^2 / h^(1/3)` limb, so the solver runs the same PDE as the
/// `iwagaki_oracle` (SC-OFEROUTE-001 rev 24, `INV-OFEROUTE-011`).
pub fn run_iwagaki_manning(
    cell_count: usize,
    sample_dt_s: f64,
    max_dt_s: f64,
) -> Result<DvalRun, RoutingError> {
    run_iwagaki_cells(
        |slope| CellParameters::manning(slope, super::iwagaki_oracle::IWAGAKI_MANNING_N),
        cell_count,
        sample_dt_s,
        max_dt_s,
    )
}

fn iwagaki_domain_end(config: &super::iwagaki_oracle::OracleConfig) -> f64 {
    config.reaches.last().map_or(24.0, |reach| reach.x_end_m)
}

fn iwagaki_reach_slope(config: &super::iwagaki_oracle::OracleConfig, x: f64) -> f64 {
    let manning_n = super::iwagaki_oracle::IWAGAKI_MANNING_N;
    for reach in &config.reaches {
        if x < reach.x_end_m {
            return (reach.alpha * manning_n).powi(2);
        }
    }
    (config.reaches[config.reaches.len() - 1].alpha * manning_n).powi(2)
}

fn iwagaki_reach_supply(config: &super::iwagaki_oracle::OracleConfig, x: f64) -> f64 {
    for reach in &config.reaches {
        if x < reach.x_end_m {
            return reach.supply_m_s;
        }
    }
    0.0
}

fn build_iwagaki_mesh(
    config: &super::iwagaki_oracle::OracleConfig,
    cell_for_slope: impl Fn(f64) -> CellParameters,
    cell_count: usize,
) -> KinematicWaveMesh {
    let domain_end = iwagaki_domain_end(config);
    let n = cell_count;
    let dx = domain_end / f64::from(u32::try_from(n).unwrap_or(u32::MAX));
    let mut cells = Vec::with_capacity(n);
    for i in 0..n {
        let x = (f64::from(u32::try_from(i).unwrap_or(u32::MAX)) + 0.5) * dx;
        cells.push(cell_for_slope(iwagaki_reach_slope(config, x)));
    }
    KinematicWaveMesh {
        cell_length_m: dx,
        cells,
    }
}

fn run_iwagaki_cells(
    cell_for_slope: impl Fn(f64) -> CellParameters,
    cell_count: usize,
    sample_dt_s: f64,
    max_dt_s: f64,
) -> Result<DvalRun, RoutingError> {
    // Codex review High-2: the Iwagaki Case-4 configuration has ONE source
    // of truth — `iwagaki_oracle::OracleConfig::iwagaki_case4()`
    // (REF-OFEROUTE-SHOCK-IWAGAKI). Geometry, reach slopes, supplies,
    // cutoff, and window are all derived from it here so solver and oracle
    // cannot diverge; `alpha = sqrt(S)/n` inverts to `S = (alpha n)^2`
    // with the configuration's own `n`.
    let config = super::iwagaki_oracle::OracleConfig::iwagaki_case4();
    let mesh = build_iwagaki_mesh(&config, cell_for_slope, cell_count);
    let dx = mesh.cell_length_m;
    let mut solver = KinematicWaveSolver::new(mesh);
    // Exact source history (High-2): supply is ON for `t < supply_end_s`
    // (matching the oracle's cutoff test), and the run clips steps at the
    // cutoff breakpoint so no step straddles it.
    let supply_end = config.supply_end_s;
    let excess = |i: usize, t: f64| {
        if t >= supply_end {
            return 0.0;
        }
        let x = (f64::from(u32::try_from(i).unwrap_or(u32::MAX)) + 0.5) * dx;
        iwagaki_reach_supply(&config, x)
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
    let res = solver.run_with_options(
        &forcing,
        None,
        &[config.supply_end_s],
        config.end_time_s,
        sample_dt_s,
        max_dt_s,
    )?;
    let hydrograph: Vec<Sample> = res
        .hydrograph
        .iter()
        .map(|s| Sample {
            time_s: s.time_s,
            q_m2_s: s.outlet_unit_discharge_m2_s,
        })
        .collect();
    let (time_to_peak_s, peak_m2_s) = sampled_peak(&hydrograph);
    Ok(DvalRun {
        hydrograph,
        peak_m2_s,
        time_to_peak_s,
        diagnostic_substep_peak_m2_s: Some(res.peak_unit_discharge_m2_s),
        diagnostic_substep_time_to_peak_s: Some(res.time_to_peak_s),
        max_courant: res.max_courant,
        runoff_coefficient: None,
        diagnostic_tvd_boundary_leak_m2: Some(res.mass_balance.tvd_boundary_leak_m2),
        diagnostic_max_homogeneous_tv_increase_m2_s: Some(res.max_homogeneous_tv_increase_m2_s),
        diagnostic_rainfall_excess_m2: Some(res.mass_balance.rainfall_excess_m2),
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
    let (time_to_peak_s, peak_m2_s) = sampled_peak(&hydro);
    Ok(DvalRun {
        peak_m2_s,
        time_to_peak_s,
        diagnostic_substep_peak_m2_s: res
            .cascade
            .per_ofe_peak_unit_discharge_m2_s
            .first()
            .copied(),
        diagnostic_substep_time_to_peak_s: None,
        max_courant: res.cascade.max_courant,
        runoff_coefficient: Some(res.per_ofe_runoff_coefficient[0]),
        hydrograph: hydro,
        diagnostic_tvd_boundary_leak_m2: None,
        diagnostic_max_homogeneous_tv_increase_m2_s: None,
        diagnostic_rainfall_excess_m2: None,
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
    const CITED_ENHANCED_PEAK_CASE2: f64 = 1.06098e-4; // Jomaa, col 17
    const CITED_ENHANCED_PEAK_CASE3: f64 = 1.685_100_6e-4; // Neibling, col 8 (S0 caveat)
    const CITED_ENHANCED_PEAK_CASE4: f64 = 8.132e-3; // Iwagaki, col 1
    const CITED_ENHANCED_RISE_CASE1_S: f64 = 3_579.914_381_755_707_4; // derived scalar, Fig 4

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
    // skin-term rainfall intensity is ZERO (see run_iwagaki). Rev 24
    // (D10B): the `k_o = 200` configuration is a COMPARATOR-FLAG
    // diagnostic (the acceptance configuration is `run_iwagaki_manning`
    // with the primary's `n = 0.009` vs the characteristics oracle); this
    // test pins its diagnostic coherence, not acceptance.
    #[test]
    fn case4_iwagaki_ko_diagnostic_remains_coherent() {
        let run = run_iwagaki(200.0).expect("iwagaki runs");
        let ratio = run.peak_m2_s / CITED_ENHANCED_PEAK_CASE4;
        assert!(
            (0.5..=2.0).contains(&ratio),
            "Case4 peak {:.3e} vs cited {:.3e} (ratio {ratio:.2}) not even order-of-magnitude",
            run.peak_m2_s,
            CITED_ENHANCED_PEAK_CASE4
        );
        let substep_t = run.diagnostic_substep_time_to_peak_s.unwrap();
        assert!(
            (substep_t - run.time_to_peak_s).abs() <= 1.0,
            "sub-step and sampled t_peak must agree within one sample: substep {substep_t}, sampled {}",
            run.time_to_peak_s
        );
        assert!(run.max_courant <= 1.0 + 1.0e-9);
    }

    // Rev 26 (D10B review response): the `k_o = 200` configuration is a
    // COMPARATOR-FLAG DIAGNOSTIC whose `f = k_o/Re` law converges to
    // `q ∝ h^3` — a near-discontinuous shock whose bin-mean peak
    // measurably WOBBLES with grid registration (measured ±13% across
    // 120..960 cells at fixed sampling, no defect trend; the earlier
    // "resolution-stable <10%" pin was ratified from a measurement
    // confounded by mixed sample grids and the pre-High-2 straddle-mass
    // surplus). Resolution stability is contractually guaranteed — and
    // enforced — on the Manning ACCEPTANCE surface
    // (`case4_manning_solver_converges_to_iwagaki_oracle`); the durable
    // guards here are the law-like surfaces of the diagnostic:
    // conservation exactness, positivity, and CFL.
    #[test]
    fn case4_iwagaki_ko_diagnostic_conserves_and_stays_positive() {
        for cells in [120usize, 240] {
            let run = run_iwagaki_with_options(200.0, cells, 0.25, 0.125).expect("runs");
            let rain = run
                .diagnostic_rainfall_excess_m2
                .expect("case4 exposes the source-total diagnostic");
            assert!(rain > 0.0);
            assert!(
                run.hydrograph.iter().all(|s| s.q_m2_s >= 0.0),
                "diagnostic hydrograph must be non-negative (cells {cells})"
            );
            assert!(run.max_courant <= 1.0 + 1.0e-9, "CFL holds (cells {cells})");
            let leak = run
                .diagnostic_tvd_boundary_leak_m2
                .expect("case4 exposes the TVD leak diagnostic");
            assert!(
                leak.abs() / rain < 1.0e-12,
                "dissipation must stay mass-neutral (cells {cells}): {leak}"
            );
        }
    }

    #[test]
    fn case2_underprediction_is_ks_operand_limited() {
        let default = run_rain_case(&case2_isolated()).expect("case2 default runs");
        let mut tighter_ks = case2_isolated();
        tighter_ks.ks_mm_h = 10.0;
        let adjusted = run_rain_case(&tighter_ks).expect("case2 adjusted runs");

        let default_ratio = default.peak_m2_s / CITED_ENHANCED_PEAK_CASE2;
        let adjusted_ratio = adjusted.peak_m2_s / CITED_ENHANCED_PEAK_CASE2;
        assert!(
            (0.70..=0.80).contains(&default_ratio),
            "D7 default Case2 ratio should remain the operand-limited shortfall: {default_ratio}"
        );
        assert!(
            (0.90..=0.95).contains(&adjusted_ratio),
            "a plausible lower sandy/gravel Ks materially closes Case2 peak: {adjusted_ratio}"
        );
        assert!(
            adjusted_ratio > default_ratio + 0.15,
            "Case2 shortfall must be materially Ks-sensitive"
        );
    }

    #[test]
    fn case3_enhanced_peak_exceeds_recorded_rainfall_length_ceiling() {
        let c = case3_vegetation();
        let rainfall_length_ceiling = mm_h_to_m_s(c.intensity_mm_h) * c.length_m;
        assert!(
            CITED_ENHANCED_PEAK_CASE3 > rainfall_length_ceiling,
            "Case3 cited enhanced peak must remain flagged as an S0 magnitude caveat"
        );

        let mut impermeable = c;
        impermeable.ks_mm_h = 0.0;
        let run = run_rain_case(&impermeable).expect("case3 impermeable runs");
        assert!(
            run.peak_m2_s <= rainfall_length_ceiling * 1.10,
            "even impermeable openWEPP should remain bounded by the recorded rainfall-length ceiling"
        );
    }

    #[test]
    fn case1_rising_limb_lag_is_green_ampt_operand_limited() {
        let default = run_rain_case(&case1_bare()).expect("case1 default runs");
        let default_rise = sampled_rise_time_10_90(&default.hydrograph).expect("default rise");
        assert!(
            default_rise > CITED_ENHANCED_RISE_CASE1_S * 1.30,
            "D8-4 baseline lag should remain visible: default rise {default_rise}"
        );

        let mut impermeable = case1_bare();
        impermeable.ks_mm_h = 0.0;
        let routing_only = run_rain_case(&impermeable).expect("case1 impermeable runs");
        let routing_rise = sampled_rise_time_10_90(&routing_only.hydrograph).expect("routing rise");
        assert!(
            routing_rise < CITED_ENHANCED_RISE_CASE1_S * 0.10,
            "routing-only response is fast, so the slow D7 limb is not a routing-celerity defect"
        );
    }
}
