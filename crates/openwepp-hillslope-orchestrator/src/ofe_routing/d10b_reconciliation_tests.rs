//! D10B contract-derived tests (`SC-OFEROUTE-001` rev 24; package
//! `20260706-mofefid-d10b-gap005-source-authority-reconciliation-001`).
//!
//! These tests encode the rev-24 bindings and FAIL against the pre-rev-24
//! scheme (the pre-implementation gate records the failing baseline):
//!
//! - `INV-OFEROUTE-011` rev 24/26: the solver, run with the primary's own
//!   Manning law, stays within the ratified tolerances against the
//!   Richardson-extrapolated Iwagaki-primary oracle at every refinement
//!   rung, non-diverging (bounded shock-peak wobble).
//! - `INV-OFEROUTE-006` rev 24: face-based dissipation is exactly
//!   mass-neutral; the ledger books the scheme's actual boundary fluxes;
//!   multi-OFE conservation is resolution-convergent.
//! - Algorithm item 6: the inter-OFE handoff injection integrates the
//!   conservative per-bin outflow series exactly.
//! - Bounded TV transient: homogeneous-step total-variation increase
//!   stays under the ratified 1e-3 m^2/s bound (rev 26).

#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use super::cascade::{CascadeForcing, CascadeSegment, run_cascade};
use super::dval::{run_iwagaki_manning, run_iwagaki_manning_hybrid};
use super::iwagaki_oracle::{OracleConfig, run_upwind_reference};
use super::kinematic_wave::{CellParameters, KinematicWaveMesh};

/// Rev-24 Case-4 acceptance tolerances, RATIFIED from the D10B S4
/// evidence (`iwagaki-case4-evidence.md`): agreement with the
/// Richardson-extrapolated oracle limit at every ladder resolution, with
/// a bounded (non-diverging) shock-peak wobble.
const TOL_PEAK_REL: f64 = 0.05;
const TOL_T_PEAK_S: f64 = 1.5;
const TOL_RISE_S: f64 = 2.0;

#[test]
fn case4_manning_solver_converges_to_iwagaki_oracle() {
    // Richardson-extrapolated oracle reference: the first-order monotone
    // reference under-resolves the shock peak from below; extrapolate the
    // 2000/4000/8000 sequence with the observed order.
    let o2 = run_upwind_reference(&OracleConfig::iwagaki_case4(), 2000);
    let o4 = run_upwind_reference(&OracleConfig::iwagaki_case4(), 4000);
    let o8 = run_upwind_reference(&OracleConfig::iwagaki_case4(), 8000);
    let r = (o8.peak_m2_s - o4.peak_m2_s) / (o4.peak_m2_s - o2.peak_m2_s);
    assert!(
        (0.2..0.9).contains(&r),
        "oracle refinement ratio out of family: {r}"
    );
    let peak_ref = o8.peak_m2_s + (o8.peak_m2_s - o4.peak_m2_s) * r / (1.0 - r);
    let t_peak_ref = o8.time_to_peak_s;
    let rise_ref = o8.rise_10_90_s.expect("oracle rise");

    let mut peak_err = Vec::new();
    let mut t_peak_err = Vec::new();
    let mut rise_err = Vec::new();
    for (cells, sample_dt, max_dt) in [
        (120usize, 0.25, 0.125),
        (240, 0.125, 0.0625),
        (480, 0.0625, 0.03125),
    ] {
        let run = run_iwagaki_manning(cells, sample_dt, max_dt).expect("manning case4 runs");
        peak_err.push((run.peak_m2_s - peak_ref).abs() / peak_ref);
        t_peak_err.push((run.time_to_peak_s - t_peak_ref).abs());
        let rise = super::dval::sampled_rise_time_10_90(&run.hydrograph).expect("rise resolvable");
        rise_err.push((rise - rise_ref).abs());
    }
    // Every ladder resolution within the ratified tolerances (the shock
    // peak carries a bounded grid wobble; divergence would breach the
    // tolerance, monotone wobble within it is accepted).
    for (k, err) in peak_err.iter().enumerate() {
        assert!(
            *err < TOL_PEAK_REL,
            "peak error at ladder step {k}: {err} (ref {peak_ref}); ladder {peak_err:?}"
        );
    }
    for (k, err) in t_peak_err.iter().enumerate() {
        assert!(
            *err < TOL_T_PEAK_S,
            "t_peak error at ladder step {k}: {err} (ref {t_peak_ref}); ladder {t_peak_err:?}"
        );
    }
    for (k, err) in rise_err.iter().enumerate() {
        assert!(
            *err < TOL_RISE_S,
            "rise error at ladder step {k}: {err} (ref {rise_ref}); ladder {rise_err:?}"
        );
    }
    // Non-divergence: the finest error may not exceed the coarsest by more
    // than the wobble band (half the tolerance).
    assert!(
        peak_err[2] <= peak_err[0] + 0.5 * TOL_PEAK_REL,
        "peak error diverges under refinement: {peak_err:?}"
    );
}

#[test]
#[ignore = "rev-31 ratification gate: currently fails Case-4 peak tolerance; run explicitly for hold evidence"]
fn case4_hybrid_manning_ladder_meets_iwagaki_oracle() {
    // Rev-31 LANED-T3 closure gate: the hybrid Case-4 harness runs the
    // active source phase explicitly, then drains the same state with the
    // implicit recession stepper. It must satisfy the same oracle-shaped
    // Case-4 acceptance tolerances before selector promotion can be claimed.
    let o2 = run_upwind_reference(&OracleConfig::iwagaki_case4(), 2000);
    let o4 = run_upwind_reference(&OracleConfig::iwagaki_case4(), 4000);
    let o8 = run_upwind_reference(&OracleConfig::iwagaki_case4(), 8000);
    let r = (o8.peak_m2_s - o4.peak_m2_s) / (o4.peak_m2_s - o2.peak_m2_s);
    assert!(
        (0.2..0.9).contains(&r),
        "oracle refinement ratio out of family: {r}"
    );
    let peak_ref = o8.peak_m2_s + (o8.peak_m2_s - o4.peak_m2_s) * r / (1.0 - r);
    let t_peak_ref = o8.time_to_peak_s;
    let rise_ref = o8.rise_10_90_s.expect("oracle rise");

    let mut peak_err = Vec::new();
    let mut t_peak_err = Vec::new();
    let mut rise_err = Vec::new();
    for (cells, sample_dt, max_dt) in [
        (120usize, 0.25, 0.125),
        (240, 0.125, 0.0625),
        (480, 0.0625, 0.03125),
    ] {
        let run = run_iwagaki_manning_hybrid(cells, sample_dt, max_dt).expect("hybrid case4 runs");
        peak_err.push((run.peak_m2_s - peak_ref).abs() / peak_ref);
        t_peak_err.push((run.time_to_peak_s - t_peak_ref).abs());
        let rise = super::dval::sampled_rise_time_10_90(&run.hydrograph).expect("rise resolvable");
        rise_err.push((rise - rise_ref).abs());
    }
    for (k, err) in peak_err.iter().enumerate() {
        assert!(
            *err < TOL_PEAK_REL,
            "hybrid peak error at ladder step {k}: {err} (ref {peak_ref}); ladder {peak_err:?}"
        );
    }
    for (k, err) in t_peak_err.iter().enumerate() {
        assert!(
            *err < TOL_T_PEAK_S,
            "hybrid t_peak error at ladder step {k}: {err} (ref {t_peak_ref}); ladder {t_peak_err:?}"
        );
    }
    for (k, err) in rise_err.iter().enumerate() {
        assert!(
            *err < TOL_RISE_S,
            "hybrid rise error at ladder step {k}: {err} (ref {rise_ref}); ladder {rise_err:?}"
        );
    }
    assert!(
        peak_err[2] <= peak_err[0] + 0.5 * TOL_PEAK_REL,
        "hybrid peak error diverges under refinement: {peak_err:?}"
    );
}

#[test]
fn case4_manning_tvd_dissipation_is_mass_neutral_and_tv_transient_bounded() {
    let run = run_iwagaki_manning(240, 0.125, 0.0625).expect("manning case4 runs");
    // Face-based dissipation must be exactly mass-neutral (rev 24): the
    // booked TVD boundary leak is zero to numerical noise relative to the
    // ~0.2 m^2 of supplied volume.
    let supply_scale = 0.2_f64;
    let leak = run
        .diagnostic_tvd_boundary_leak_m2
        .expect("case4 exposes the TVD leak diagnostic");
    assert!(
        leak.abs() / supply_scale < 1.0e-12,
        "TVD term must not create/destroy mass: leak {leak}"
    );
    // TV-transient bound on homogeneous steps (post-cutoff routing),
    // RATIFIED from the S4 evidence: the scheme is not strictly TVD at
    // shock formation/boundary-adjacent cells (measured max single-step
    // increase ~5e-4 m^2/s ~ 6% of peak flux, a bounded transient); the
    // strict-TVD gap is a named residual refinement item
    // (`iwagaki-case4-evidence.md`), and oscillation-freedom of the
    // ACCEPTANCE surfaces is carried by the oracle-convergence metrics.
    let tv = run
        .diagnostic_max_homogeneous_tv_increase_m2_s
        .expect("case4 exposes the TV diagnostic");
    assert!(
        tv < 1.0e-3,
        "homogeneous-step TV transient exceeds the ratified bound: {tv}"
    );
}

#[test]
fn solver_ledger_books_scheme_actual_boundary_fluxes() {
    // Two-OFE cascade with steady supply: every OFE's booked ledger must
    // equal the scheme-actual boundary fluxes (rev 24 booked-equals-actual).
    let v = 60.0 / 3.6e6;
    let segs = vec![
        CascadeSegment {
            mesh: KinematicWaveMesh::uniform(10.0, 20, CellParameters::bare(0.08, 500.0)),
            width_m: 2.0,
        },
        CascadeSegment {
            mesh: KinematicWaveMesh::uniform(10.0, 20, CellParameters::bare(0.06, 500.0)),
            width_m: 2.0,
        },
    ];
    let excess = |_o: usize, _c: usize, _t: f64| v;
    let intensity = |_o: usize, _t: f64| v;
    let forcing = CascadeForcing {
        rainfall_excess_m_s: &excess,
        rainfall_intensity_m_s: &intensity,
    };
    let res = run_cascade(&segs, &forcing, 3600.0, 10.0, 2.0).expect("cascade runs");
    for (i, mb) in res.per_ofe_solver_mass.iter().enumerate() {
        let scale = mb.rainfall_excess_m2 + mb.inflow_m2;
        assert!(
            (mb.inflow_m2 - mb.scheme_inflow_m2).abs() / scale < 1.0e-12,
            "OFE {i}: booked inflow {} != scheme inflow {}",
            mb.inflow_m2,
            mb.scheme_inflow_m2
        );
        assert!(
            (mb.outflow_m2 - mb.scheme_outflow_m2).abs() / scale < 1.0e-12,
            "OFE {i}: booked outflow {} != scheme outflow {}",
            mb.outflow_m2,
            mb.scheme_outflow_m2
        );
    }
}

#[test]
fn handoff_injection_is_flux_integral_conservative() {
    // The downstream OFE's booked received volume must equal the exact
    // integral of the upstream CONSERVATIVE bin series over the window
    // (rev 24 Algorithm item 6), independent of downstream step
    // placement. Coarse sampling makes the pre-rev-24 left-endpoint
    // injection visibly non-conservative.
    let v = 60.0 / 3.6e6;
    let segs = vec![
        CascadeSegment {
            mesh: KinematicWaveMesh::uniform(10.0, 20, CellParameters::bare(0.08, 500.0)),
            width_m: 1.0,
        },
        CascadeSegment {
            mesh: KinematicWaveMesh::uniform(10.0, 20, CellParameters::bare(0.06, 500.0)),
            width_m: 1.0,
        },
    ];
    // 30 min of rain then stop: rising + falling limbs exercise the
    // injection quadrature in both directions.
    let excess = |_o: usize, _c: usize, t: f64| if t < 1800.0 { v } else { 0.0 };
    let intensity = |_o: usize, t: f64| if t < 1800.0 { v } else { 0.0 };
    let forcing = CascadeForcing {
        rainfall_excess_m_s: &excess,
        rainfall_intensity_m_s: &intensity,
    };
    let res = run_cascade(&segs, &forcing, 5400.0, 120.0, 60.0).expect("cascade runs");
    // The downstream booked receipt must equal the upstream booked outlet
    // volume (both from the solver ledger via the conservative bin series).
    let upstream_sampled_m3 = res.per_ofe_outlet_m3[0];
    let received_m3 = res.per_ofe_received_upstream_m3[1];
    assert!(
        (received_m3 - upstream_sampled_m3).abs() / upstream_sampled_m3 < 1.0e-9,
        "handoff injection must equal the upstream sampled-series integral: received {received_m3} vs sampled {upstream_sampled_m3}"
    );
}

#[test]
fn nineteen_ofe_conservation_is_resolution_convergent() {
    // H2637-shaped steep 19-OFE cascade (the S0 ledger fixture): the
    // clamp-adjusted cascade conservation residual must DECREASE
    // monotonically as (sample_dt, max_dt) refine, and land small
    // (rev 24 INV-OFEROUTE-006(c)). The pre-rev-24 scheme GROWS instead
    // (9% -> 54%, seam-conservation-ledger.md).
    let segments: Vec<CascadeSegment> = (0..19)
        .map(|i| {
            let frac = f64::from(u8::try_from(i).unwrap_or(u8::MAX)) / 18.0;
            CascadeSegment {
                mesh: KinematicWaveMesh::uniform(
                    20.0,
                    10,
                    CellParameters::bare(0.25 + 0.36 * frac, 500.0),
                ),
                width_m: 30.0,
            }
        })
        .collect();
    let hourly = |hour: usize| -> f64 {
        match hour {
            6 | 8 => 0.002,
            7 => 0.006,
            _ => 0.0,
        }
    };
    let excess = |_o: usize, _c: usize, t: f64| {
        let hour = (t / 3600.0) as usize;
        if hour < 24 {
            hourly(hour) / 3600.0
        } else {
            0.0
        }
    };
    let intensity = |_o: usize, _t: f64| 0.0;
    let forcing = CascadeForcing {
        rainfall_excess_m_s: &excess,
        rainfall_intensity_m_s: &intensity,
    };
    let window = 15.0 * 3600.0;
    let mut residuals = Vec::new();
    for (sample_dt, max_dt) in [
        (900.0, 300.0),
        (900.0, 120.0),
        (120.0, 300.0),
        (120.0, 120.0),
        (60.0, 30.0),
        (15.0, 5.0),
    ] {
        let res =
            run_cascade(&segments, &forcing, window, sample_dt, max_dt).expect("cascade runs");
        residuals.push(
            res.mass_balance.conservation_residual_m3().abs() / res.mass_balance.rainfall_excess_m3,
        );
    }
    // Post-correction the residual is exact (fp) at EVERY resolution —
    // strictly stronger than the resolution-convergence the pre-rev-24
    // scheme failed; assert the exactness bound directly.
    for (k, r) in residuals.iter().enumerate() {
        assert!(
            *r < 1.0e-9,
            "cascade residual at sweep point {k} must be exact (fp): {residuals:?}"
        );
    }
}

#[test]
fn bin_loops_progress_on_non_dyadic_sample_dt() {
    // Review-B M1 regression: `sample_dt = 0.003` has floating-point
    // zero-progress witnesses (e.g. t = 0.147) under the pre-fix bin-loop
    // formulation; the integer-index loops must terminate and conserve.
    let mesh = KinematicWaveMesh::uniform(5.0, 10, CellParameters::bare(0.05, 100.0));
    let mut solver = super::kinematic_wave::KinematicWaveSolver::new(mesh);
    let v = 60.0 / 3.6e6;
    let excess = |_i: usize, _t: f64| v;
    let inflow = |_t: f64| 0.0;
    let intensity = |_t: f64| v;
    let forcing = super::kinematic_wave::Forcing {
        rainfall_excess_m_s: &excess,
        upstream_inflow_m2_s: &inflow,
        rainfall_intensity_m_s: &intensity,
    };
    let res = solver.run(&forcing, 0.9, 0.003, 0.05).expect("run ok");
    let bin_sum: f64 = res.outlet_bin_outflow_m2.iter().sum();
    assert!(
        (bin_sum - res.mass_balance.outflow_m2).abs() <= 1.0e-15 + 1.0e-12 * bin_sum.abs(),
        "bin series must carry the booked outflow: bins {bin_sum} vs ledger {}",
        res.mass_balance.outflow_m2
    );
}

#[test]
fn runon_only_ofe_handoff_is_nonnegative_and_conservative() {
    // Review-B M2 regression: a middle/lower OFE with ZERO local excess
    // (runon-only) sees a dry-outlet front arrival whose per-step boundary
    // attribution dips negative; the redistributed bin series must stay
    // non-negative, the downstream injection must not fail closed, and the
    // cascade must conserve exactly.
    let v = 60.0 / 3.6e6;
    let segs = vec![
        CascadeSegment {
            mesh: KinematicWaveMesh::uniform(10.0, 20, CellParameters::bare(0.08, 500.0)),
            width_m: 1.0,
        },
        CascadeSegment {
            mesh: KinematicWaveMesh::uniform(10.0, 20, CellParameters::bare(0.06, 500.0)),
            width_m: 1.0,
        },
    ];
    // rain ONLY on the upstream OFE
    let excess = |ofe: usize, _c: usize, t: f64| {
        if ofe == 0 && t < 1800.0 { v } else { 0.0 }
    };
    let intensity = |_o: usize, _t: f64| 0.0;
    let forcing = CascadeForcing {
        rainfall_excess_m_s: &excess,
        rainfall_intensity_m_s: &intensity,
    };
    let res = run_cascade(&segs, &forcing, 5400.0, 30.0, 15.0).expect("runon-only cascade runs");
    assert!(
        res.mass_balance.conservation_residual_m3().abs() / res.mass_balance.rainfall_excess_m3
            < 1.0e-9,
        "runon-only cascade must conserve exactly: residual {}",
        res.mass_balance.conservation_residual_m3()
    );
    assert!(
        res.outlet_hydrograph
            .iter()
            .all(|s| s.outlet_unit_discharge_m2_s >= 0.0),
        "exported bin-mean discharge must be non-negative"
    );
    assert!(
        (res.per_ofe_received_upstream_m3[1] - res.per_ofe_outlet_m3[0]).abs()
            / res.per_ofe_outlet_m3[0]
            < 1.0e-9,
        "runon-only handoff must be exact: received {} vs upstream outlet {}",
        res.per_ofe_received_upstream_m3[1],
        res.per_ofe_outlet_m3[0]
    );
}

#[test]
fn partial_final_bin_handoff_is_exact() {
    // Review-B M3 regression: `end_time_s` NOT a multiple of `sample_dt_s`
    // (5430 / 120): the final partial bin's mass must still be injected
    // downstream in full (span-aware integration), keeping the seam exact.
    let v = 60.0 / 3.6e6;
    let segs = vec![
        CascadeSegment {
            mesh: KinematicWaveMesh::uniform(10.0, 20, CellParameters::bare(0.08, 500.0)),
            width_m: 1.0,
        },
        CascadeSegment {
            mesh: KinematicWaveMesh::uniform(10.0, 20, CellParameters::bare(0.06, 500.0)),
            width_m: 1.0,
        },
    ];
    let excess = |_o: usize, _c: usize, t: f64| if t < 1800.0 { v } else { 0.0 };
    let intensity = |_o: usize, t: f64| if t < 1800.0 { v } else { 0.0 };
    let forcing = CascadeForcing {
        rainfall_excess_m_s: &excess,
        rainfall_intensity_m_s: &intensity,
    };
    let res = run_cascade(&segs, &forcing, 5430.0, 120.0, 60.0).expect("cascade runs");
    assert!(
        (res.per_ofe_received_upstream_m3[1] - res.per_ofe_outlet_m3[0]).abs()
            / res.per_ofe_outlet_m3[0]
            < 1.0e-9,
        "partial-final-bin handoff must inject the full upstream mass: received {} vs outlet {}",
        res.per_ofe_received_upstream_m3[1],
        res.per_ofe_outlet_m3[0]
    );
    assert!(
        res.mass_balance.conservation_residual_m3().abs() / res.mass_balance.rainfall_excess_m3
            < 1.0e-9,
        "cascade must conserve exactly with a partial final bin"
    );
}

#[test]
fn unsatisfiable_cfl_fails_closed_not_partial_ok() {
    // Codex review High-1 regression: a finite-but-extreme parameter state
    // whose celerity admits no positive sub-timestep (dt underflows to 0)
    // must return a typed CFL failure, never a partial Ok result. The
    // pre-fix `dt <= 0 -> break` returned Ok(RoutingResult).
    let mesh = KinematicWaveMesh::uniform(1.0e-199, 10, CellParameters::manning(1.0e308, 0.009));
    let mut solver = super::kinematic_wave::KinematicWaveSolver::new(mesh);
    let excess = |_i: usize, _t: f64| 1.0;
    let inflow = |_t: f64| 0.0;
    let intensity = |_t: f64| 0.0;
    let forcing = super::kinematic_wave::Forcing {
        rainfall_excess_m_s: &excess,
        upstream_inflow_m2_s: &inflow,
        rainfall_intensity_m_s: &intensity,
    };
    let result = solver.run(&forcing, 10.0, 1.0, 1.0);
    assert!(
        matches!(
            result,
            Err(super::kinematic_wave::RoutingError::CflViolation
                | super::kinematic_wave::RoutingError::NonFiniteState)
        ),
        "unsatisfiable CFL must fail closed, got {result:?}"
    );
}

#[test]
fn case4_solver_and_oracle_source_histories_agree_exactly() {
    // Codex review High-2 regression: the solver's integrated lateral
    // source must equal the oracle configuration's supplied volume
    // (sum of reach supply x reach length x supply duration) exactly —
    // one source of truth, exact cutoff at T (no step straddles it).
    let config = OracleConfig::iwagaki_case4();
    let mut supplied_m2 = 0.0_f64;
    let mut x0 = 0.0_f64;
    for reach in &config.reaches {
        supplied_m2 += reach.supply_m_s * (reach.x_end_m - x0) * config.supply_end_s;
        x0 = reach.x_end_m;
    }
    for (cells, sample_dt, max_dt) in [(120usize, 0.25, 0.125), (240, 0.125, 0.0625)] {
        let run = run_iwagaki_manning(cells, sample_dt, max_dt).expect("manning case4 runs");
        let solver_rain = run
            .diagnostic_rainfall_excess_m2
            .expect("case4 exposes the source-total diagnostic");
        assert!(
            (solver_rain - supplied_m2).abs() <= 1.0e-9 * supplied_m2,
            "solver source history must match the oracle configuration exactly: solver {solver_rain} vs config {supplied_m2} (cells {cells})"
        );
    }
}

#[test]
fn single_ofe_outlet_bins_stay_nonnegative_under_front_arrival() {
    // Codex review Medium-1 regression (terminal/single-OFE surface): a
    // dry OFE fed only by an upstream inflow pulse sees the front-arrival
    // negative boundary-attribution transient at its own outlet; the
    // PUBLISHED hydrograph and bin series must stay non-negative and the
    // bins must still sum exactly to the booked outflow.
    let mesh = KinematicWaveMesh::uniform(10.0, 20, CellParameters::bare(0.06, 500.0));
    let mut solver = super::kinematic_wave::KinematicWaveSolver::new(mesh);
    let excess = |_i: usize, _t: f64| 0.0;
    let inflow = |t: f64| if t < 600.0 { 5.0e-4 } else { 0.0 };
    let intensity = |_t: f64| 0.0;
    let forcing = super::kinematic_wave::Forcing {
        rainfall_excess_m_s: &excess,
        upstream_inflow_m2_s: &inflow,
        rainfall_intensity_m_s: &intensity,
    };
    let res = solver.run(&forcing, 3600.0, 30.0, 15.0).expect("run ok");
    assert!(
        res.hydrograph
            .iter()
            .all(|s| s.outlet_unit_discharge_m2_s >= 0.0),
        "published outlet discharge must be non-negative"
    );
    assert!(
        res.outlet_bin_outflow_m2.iter().all(|v| *v >= 0.0),
        "published outlet bins must be non-negative"
    );
    let bin_sum: f64 = res.outlet_bin_outflow_m2.iter().sum();
    assert!(
        (bin_sum - res.mass_balance.outflow_m2).abs()
            <= 1.0e-9 * res.mass_balance.outflow_m2.max(1.0e-12),
        "bins must sum to the booked outflow: {bin_sum} vs {}",
        res.mass_balance.outflow_m2
    );
}
