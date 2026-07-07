//! T3-I1 (`SC-OFEROUTE-001` rev-28 DRAFT; WP
//! `20260706-laned-router-t3-hybrid-implicit-stepping-001`): the IMPLICIT
//! kinematic-wave stepper for smooth (recession/homogeneous) phases.
//!
//! Backward-Euler in time + first-order upwind in space. Kinematic-wave
//! information flows strictly downstream, so the implicit system is
//! lower-triangular: a single downstream march with ONE scalar safeguarded
//! solve per cell — no matrix, `O(n)` per step, unconditionally stable and
//! unconditionally positivity-preserving (`F(0) ≤ 0` for non-negative
//! inputs, `F' ≥ 1`), with a machine-exact mass ledger BY CONSTRUCTION (the
//! discrete cell equation IS the mass balance; the flux terms telescope).
//!
//! SOLVER-TIER ONLY in this increment: nothing routes through this module in
//! production; the hybrid integration (switching rule, opt-in selector,
//! contract ratification) is T3-I2. Accuracy on smooth phases is adjudicated
//! by the dt-refinement ladder against the explicit reference (design:
//! `artifacts/i0-scheme-design.md`).

use super::friction::{KINEMATIC_VISCOSITY_M2_S, SKIN_REGIME_REYNOLDS_THRESHOLD};
use super::kinematic_wave::{DRY_DEPTH_M, KinematicWaveMesh, RoutingError};
use super::profile;

/// The skin-regime crossover discharge `Q_c = Re_c · ν` (m²/s). The
/// INV-OFEROUTE-002 dispatch makes the friction law DISCONTINUOUS here, so
/// the equilibrium rating `q_eq(h)` is Z-SHAPED: in an overlap depth band
/// BOTH a laminar-branch and a turbulent-branch equilibrium exist, and the
/// fixed-point basins split exactly at `Q_c` (the map is increasing with an
/// upward jump at `Q_c`; each concave branch attracts its own side). The
/// implicit solve therefore evaluates a DETERMINISTIC single-valued rating
/// per branch via basin-split seeding (`Q_c·1e-3` / `Q_c·1e3`) and prefers
/// the LOW (laminar) branch, falling back to the HIGH branch when the cell
/// equation's root falls in the low rating's jump (rev-28 draft rule; the
/// explicit scheme's frozen 4-iteration cap masks this same structure).
const CROSSOVER_Q_M2_S: f64 = SKIN_REGIME_REYNOLDS_THRESHOLD * KINEMATIC_VISCOSITY_M2_S;

/// Equilibrium fixed-point tolerance (relative) for `q_eq(h)` inside the
/// implicit solve. The laminar limb contracts at ~0.5/iteration, so the
/// 80-iteration budget covers this from any seed.
pub const IMPLICIT_EQ_TOL_REL: f64 = 1.0e-13;
/// Equilibrium fixed-point iteration budget.
pub const IMPLICIT_EQ_MAX_ITER: usize = 80;
/// Outer per-cell solve: residual tolerance relative to the cell's
/// right-hand side (mass units, m of depth).
pub const IMPLICIT_NEWTON_TOL_REL: f64 = 1.0e-13;
/// Outer per-cell solve iteration budget (safeguarded bisection/secant on a
/// maintained bracket; the bracket halves even when secant stalls).
pub const IMPLICIT_NEWTON_MAX_ITER: usize = 128;

/// One implicit step's booked ledger (per unit width, m²) and solve
/// diagnostics. Booked = actual by construction.
#[derive(Debug, Clone, Copy, Default)]
pub struct ImplicitStepOutcome {
    /// Upstream boundary mass injected, `q_up · dt`.
    pub inflow_m2: f64,
    /// Outlet boundary mass discharged, `q_eq(h_out^{n+1}) · dt`.
    pub outflow_m2: f64,
    /// Source mass injected, `Σ v_i · dx · dt`.
    pub rainfall_excess_m2: f64,
    /// Committed storage change, `Σ (h_i^{n+1} − h_i^n) · dx`.
    pub storage_change_m2: f64,
    /// Outlet unit discharge at the new time level (m²/s).
    pub outlet_unit_discharge_m2_s: f64,
    /// Outlet depth at the new time level (m).
    pub outlet_depth_m: f64,
    /// Outer-solve iterations summed over cells (work diagnostic).
    pub solve_iterations: u64,
}

impl ImplicitStepOutcome {
    /// The step's conservation residual: zero by construction up to the
    /// per-cell solve tolerances (no clamp term exists in this scheme).
    #[must_use]
    pub fn residual_m2(&self) -> f64 {
        self.inflow_m2 + self.rainfall_excess_m2 - self.outflow_m2 - self.storage_change_m2
    }
}

/// Advance the mesh state one implicit step of size `dt_s`.
///
/// `depth_m` is the per-cell state (upstream → downstream, matching the
/// explicit solver's layout) and is committed in place on success.
/// `q_up_m2_s` is the interval-mean upstream boundary inflow (0 under the
/// strict T3 switching rule; the solve handles positive inflow unchanged —
/// the aggressive-rule extension changes only the caller's predicate).
/// `v_m_s` is the per-cell source rate (0 on recession; kept general so the
/// stepper's physics vectors can exercise sources directly).
pub fn implicit_step(
    mesh: &KinematicWaveMesh,
    depth_m: &mut [f64],
    q_up_m2_s: f64,
    v_m_s: &[f64],
    skin_rain_term: f64,
    dt_s: f64,
) -> Result<ImplicitStepOutcome, RoutingError> {
    implicit_step_with_discharges(mesh, depth_m, None, q_up_m2_s, v_m_s, skin_rain_term, dt_s)
}

/// `implicit_step` variant that also writes the per-cell converged
/// equilibrium discharges (the T3-I2 hybrid exit-seam state, design §2.2).
pub fn implicit_step_with_discharges(
    mesh: &KinematicWaveMesh,
    depth_m: &mut [f64],
    mut discharge_out_m2_s: Option<&mut [f64]>,
    q_up_m2_s: f64,
    v_m_s: &[f64],
    skin_rain_term: f64,
    dt_s: f64,
) -> Result<ImplicitStepOutcome, RoutingError> {
    let n = mesh.cell_count();
    let dx = mesh.cell_length_m;
    if n == 0
        || depth_m.len() != n
        || v_m_s.len() != n
        || !dx.is_finite()
        || dx <= 0.0
        || !dt_s.is_finite()
        || dt_s <= 0.0
        || discharge_out_m2_s
            .as_ref()
            .is_some_and(|out| out.len() != n)
    {
        return Err(RoutingError::DegenerateConfiguration);
    }
    if !q_up_m2_s.is_finite() || q_up_m2_s < 0.0 {
        return Err(RoutingError::InvalidForcing);
    }
    let dt_over_dx = dt_s / dx;
    let mut outcome = ImplicitStepOutcome {
        inflow_m2: q_up_m2_s * dt_s,
        ..ImplicitStepOutcome::default()
    };

    let mut q_in = q_up_m2_s;
    let mut q_out_last = 0.0_f64;
    for i in 0..n {
        let cell = &mesh.cells[i];
        cell.validate()?;
        let h_old = depth_m[i];
        let v = v_m_s[i];
        if !is_valid_state(h_old) || !is_valid_state(v) {
            return Err(RoutingError::InvalidForcing);
        }
        // rhs = h^n + (dt/dx)·q_in + v·dt  (all non-negative), and the root
        // of F(h) = h + (dt/dx)·q_eq(h) − rhs lies in [0, rhs]:
        //   F(0) = −rhs ≤ 0,  F(rhs) = (dt/dx)·q_eq(rhs) ≥ 0,  F' ≥ 1.
        let rhs = h_old + dt_over_dx * q_in + v * dt_s;
        if !rhs.is_finite() {
            return Err(RoutingError::NonFiniteState);
        }
        // Warm start: recession depths move slowly, so the old state is an
        // excellent bracket-interior start (the RATING itself is seeded
        // deterministically per branch — determinism over warmth there).
        let warm_q_seed = if i == 0 { None } else { Some(q_in) };
        let (h_new, q_new, iterations) = solve_cell(
            cell,
            rhs,
            dt_over_dx,
            skin_rain_term,
            h_old.min(rhs),
            warm_q_seed,
        )?;
        outcome.solve_iterations += iterations;
        outcome.rainfall_excess_m2 += v * dt_s * dx;
        outcome.storage_change_m2 += (h_new - h_old) * dx;
        depth_m[i] = h_new;
        if let Some(out) = discharge_out_m2_s.as_deref_mut() {
            out[i] = q_new;
        }
        q_in = q_new;
        q_out_last = q_new;
    }
    outcome.outflow_m2 = q_out_last * dt_s;
    outcome.outlet_unit_discharge_m2_s = q_out_last;
    outcome.outlet_depth_m = depth_m[n - 1];
    // Exact-ledger-by-construction is the scheme's design property; a
    // material residual therefore IS a solve failure, and the step fails
    // closed rather than committing a leaking state (the committed depths
    // are already written, but the caller aborts on Err — no partial
    // acceptance path exists).
    // The guard is RELATIVE with an ABSOLUTE noise floor of ONE
    // DRY-THRESHOLD DEPTH OVER THE WHOLE MESH (`DRY_DEPTH_M · L`): a step
    // whose entire ledger sits below the solver family's own dry floor is
    // numerical dust and cannot meaningfully violate conservation (observed
    // on H2637: −6e-23 residual on a 5e-14 m² step, −2e-19 on 2e-10 m² —
    // relative wiggle just over 1e-9 with zero physical mass at stake).
    #[allow(clippy::cast_precision_loss)]
    let dust_floor_m2 = DRY_DEPTH_M * dx * n as f64;
    let scale = outcome
        .inflow_m2
        .abs()
        .max(outcome.rainfall_excess_m2.abs())
        .max(outcome.outflow_m2.abs())
        .max(outcome.storage_change_m2.abs())
        .max(dust_floor_m2);
    if outcome.residual_m2().abs() > 1.0e-9 * scale {
        return Err(RoutingError::ImplicitSolveNonConvergence);
    }
    Ok(outcome)
}

#[inline]
fn is_valid_state(value: f64) -> bool {
    value.is_finite() && value >= 0.0
}

/// One branch-solve outcome: a genuine branch ROOT, or a bracket collapse
/// onto the rating's JUMP carrying the Filippov (filled-jump) candidate.
enum CellSolve {
    Root(f64, f64, u64),
    /// Bracket collapse onto the rating's jump (iterations spent) — the
    /// caller resolves it via the other branch or fails closed (rev 29:
    /// the filled-jump commit is removed, so no state payload is carried).
    Jump(u64),
}

/// Deterministic branch selection for the Z-shaped rating.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RatingBranch {
    /// Laminar-basin seeding (`Q_c·1e-3`): the LOW branch of the rating.
    Low,
    /// Turbulent-basin seeding (`Q_c·1e3`): the HIGH branch.
    High,
}

/// Converged equilibrium discharge on the requested rating branch —
/// a pure function of `(cell, h, rain, branch)` (fail closed on fixed-point
/// non-convergence). Where only one basin exists, both branches return it.
fn equilibrium_q(
    cell: &super::kinematic_wave::CellParameters,
    h: f64,
    skin_rain_term: f64,
    branch: RatingBranch,
    warm_q_seed: Option<f64>,
) -> Result<f64, RoutingError> {
    let seed = warm_q_seed
        .filter(|q| branch_accepts_seed(*q, branch))
        .unwrap_or_else(|| cold_branch_seed(branch));
    cell.equilibrium_discharge_converged(
        h,
        skin_rain_term,
        seed,
        IMPLICIT_EQ_TOL_REL,
        IMPLICIT_EQ_MAX_ITER,
    )
    .ok_or(RoutingError::ImplicitSolveNonConvergence)
}

#[inline]
fn cold_branch_seed(branch: RatingBranch) -> f64 {
    match branch {
        RatingBranch::Low => CROSSOVER_Q_M2_S * 1.0e-3,
        RatingBranch::High => CROSSOVER_Q_M2_S * 1.0e3,
    }
}

#[inline]
fn branch_accepts_seed(q_seed: f64, branch: RatingBranch) -> bool {
    q_seed.is_finite()
        && q_seed > 0.0
        && match branch {
            RatingBranch::Low => q_seed < CROSSOVER_Q_M2_S,
            RatingBranch::High => q_seed > CROSSOVER_Q_M2_S,
        }
}

/// Solve `h + (dt/dx)·q_eq(h) = rhs` with the LOW-branch rating, falling
/// back to the HIGH branch when the root falls in the low rating's jump
/// (the Z-shaped-rating rule; module doc). Every returned `(h, q)` pair is
/// VALIDATED against the cell equation — no exit path may return an
/// unvalidated pair (that inconsistency class is exactly what the exact
/// ledger exists to forbid).
fn solve_cell(
    cell: &super::kinematic_wave::CellParameters,
    rhs: f64,
    dt_over_dx: f64,
    skin_rain_term: f64,
    h_start: f64,
    warm_q_seed: Option<f64>,
) -> Result<(f64, f64, u64), RoutingError> {
    // Degenerate dry cell: nothing to hold or move.
    if rhs <= 0.0 {
        return Ok((0.0, 0.0, 0));
    }
    // Branch chain (rev-29 form, T3-H1 review): LOW root, else HIGH root.
    // A BOTH-JUMP outcome FAILS CLOSED: for this rating geometry it is
    // provably unreachable for genuine physics — each branch rating has
    // exactly one UPWARD jump (LOW at the laminar-basin edge `h_b`, HIGH at
    // the turbulent-basin edge `h_a`, with `h_a < h_b`), and the cell line
    // `q = (rhs − h)/(Δt/Δx)` is strictly DECREASING, so crossing the LOW
    // jump requires `line(h_b) > Q_c` while crossing the HIGH jump requires
    // `line(h_a) < Q_c` — contradicting `line(h_a) > line(h_b)`. Whenever
    // the LOW rating jumps over the root, the HIGH rating (turbulent above
    // `h_a`, laminar below) must host a genuine root; a double collapse can
    // therefore only be a solve failure (tolerance miss, fixed-point
    // pathology) and MUST NOT be masked by a mass-exact filled-jump commit
    // (the earlier Filippov arm is REMOVED on exactly these grounds).
    match solve_cell_on_branch(
        cell,
        rhs,
        dt_over_dx,
        skin_rain_term,
        h_start,
        RatingBranch::Low,
        warm_q_seed,
    )? {
        CellSolve::Root(h, q, iterations) => Ok((h, q, iterations)),
        CellSolve::Jump(low_iterations) => {
            match solve_cell_on_branch(
                cell,
                rhs,
                dt_over_dx,
                skin_rain_term,
                h_start,
                RatingBranch::High,
                warm_q_seed,
            )? {
                CellSolve::Root(h, q, iterations) => Ok((h, q, low_iterations + iterations)),
                CellSolve::Jump(_) => Err(RoutingError::ImplicitSolveNonConvergence),
            }
        }
    }
}

fn solve_cell_on_branch(
    cell: &super::kinematic_wave::CellParameters,
    rhs: f64,
    dt_over_dx: f64,
    skin_rain_term: f64,
    h_start: f64,
    branch: RatingBranch,
    warm_q_seed: Option<f64>,
) -> Result<CellSolve, RoutingError> {
    let tol = IMPLICIT_NEWTON_TOL_REL * rhs.max(1.0e-18);
    let f_at = |h: f64| -> Result<(f64, f64), RoutingError> {
        profile::count_implicit_branch_evaluations(1);
        let q = equilibrium_q(cell, h, skin_rain_term, branch, warm_q_seed)?;
        Ok((h + dt_over_dx * q - rhs, q))
    };

    let mut lo = 0.0_f64;
    let mut hi = rhs;
    let (f_hi, q_hi) = f_at(hi)?;
    if f_hi.abs() <= tol {
        // Zero-conveyance cell (q_eq(rhs) ≈ 0): the root is h = rhs.
        return Ok(CellSolve::Root(hi, q_hi, 1));
    }
    let mut f_lo = -rhs; // F(0) = −rhs (q_eq(0) = 0)
    let mut f_hi = f_hi;
    // Secant start from the warm interior point when it is usable.
    let mut x = if h_start > 0.0 && h_start < rhs {
        h_start
    } else {
        0.5 * rhs
    };
    let mut iterations = 0_u64;
    for iteration in 0..IMPLICIT_NEWTON_MAX_ITER {
        iterations += 1;
        let (f_x, q_x) = f_at(x)?;
        if f_x.abs() <= tol {
            return Ok(CellSolve::Root(x, q_x, iterations));
        }
        if f_x > 0.0 {
            hi = x;
            f_hi = f_x;
        } else {
            lo = x;
            f_lo = f_x;
        }
        // GUARANTEED-PROGRESS rule: pure false position stalls one-sided on
        // the convex rating (`q ~ h³` on the laminar limb — the far
        // endpoint barely moves and an interior clamp alone shrinks the
        // bracket only geometrically-slowly). Interleave TRUE bisection on
        // every other iteration so the bracket provably halves at least
        // every two iterations (2^-64 over the 128 budget).
        let denom = f_hi - f_lo;
        let secant = if denom.abs() > 0.0 {
            hi - f_hi * (hi - lo) / denom
        } else {
            0.5 * (lo + hi)
        };
        let interior_lo = lo + 0.01 * (hi - lo);
        let interior_hi = hi - 0.01 * (hi - lo);
        let use_secant = iteration % 2 == 0
            && secant.is_finite()
            && secant > interior_lo
            && secant < interior_hi;
        x = if use_secant { secant } else { 0.5 * (lo + hi) };
        if (hi - lo) <= f64::EPSILON * rhs {
            // FILIPPOV (filled-jump) closure, rev-28 rule: the bracket
            // collapsed with `f_lo < -tol < tol < f_hi` — the root falls in
            // the Z-rating's JUMP at this depth (the required discharge
            // lies between the two branch equilibria, so no branch hosts
            // it). The set-valued rating's weak solution commits the
            // collapse depth with the discharge the CELL MASS EQUATION
            // demands: `q = (rhs − h)/(Δt/Δx)` — exact ledger by
            // construction, `q` inside the jump's convex hull
            // (`q_low < q < q_high` follows from the bracketing signs),
            // and deterministic (the collapse point is unique).
            return Ok(CellSolve::Jump(iterations));
        }
    }
    Err(RoutingError::ImplicitSolveNonConvergence)
}

#[cfg(test)]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
mod tests {
    use super::super::friction::GRAVITY_M_S2;
    use super::super::kinematic_wave::{
        CellParameters, Forcing, KinematicWaveMesh, KinematicWaveSolver,
    };
    use super::*;

    const BIN_S: f64 = 900.0;

    fn bare_mesh(length_m: f64, cells: usize, slope: f64, ko: f64) -> KinematicWaveMesh {
        KinematicWaveMesh::uniform(length_m, cells, CellParameters::bare(slope, ko))
    }

    /// Build the steady-state profile for uniform source rate `v` by
    /// inverting the converged equilibrium (bisection per cell on `q_eq`):
    /// at steady state `q(x) = v·x`, evaluated at cell OUTLET faces (the
    /// upwind scheme's discrete steady state satisfies
    /// `q_i = q_in,i + v·dx` exactly).
    fn steady_profile(mesh: &KinematicWaveMesh, v: f64, rain_term: f64) -> Vec<f64> {
        let dx = mesh.cell_length_m;
        let mut depths = vec![0.0_f64; mesh.cell_count()];
        let mut q = 0.0_f64;
        for (i, cell) in mesh.cells.iter().enumerate() {
            q += v * dx;
            // invert q_eq(h) = q by bisection
            let mut lo = 0.0_f64;
            let mut hi = 1.0_f64;
            while cell
                .equilibrium_discharge_converged(hi, rain_term, q, 1.0e-13, 80)
                .expect("fixed point")
                < q
            {
                hi *= 2.0;
            }
            for _ in 0..200 {
                let mid = 0.5 * (lo + hi);
                let q_mid = cell
                    .equilibrium_discharge_converged(mid, rain_term, q, 1.0e-13, 80)
                    .expect("fixed point");
                if q_mid < q {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            depths[i] = 0.5 * (lo + hi);
        }
        depths
    }

    /// T3-M2/QA-M2 (review-required direct vector): the LOW→HIGH branch
    /// chain on a constructed jump-crossing cell. For `bare(0.05, 500)`
    /// the laminar-basin edge is `h_b ≈ 5.49e-3 m` (where
    /// `q_lam = 6884·h³` reaches `Q_c = 1.14e-3`); with `Δt/Δx = 450` and
    /// `rhs = h_b + 450·1.5e-3` the cell line crosses the LOW rating's
    /// jump (required `q ≈ 1.5e-3 > Q_c` at `h_b`), so the LOW solve MUST
    /// report `Jump` and the chain MUST recover the genuine TURBULENT root
    /// from the HIGH branch (per the rev-29 monotonicity argument a real
    /// root always exists on the other branch; a both-jump outcome fails
    /// closed).
    #[test]
    fn low_jump_recovers_high_branch_root_and_never_commits_filippov() {
        let cell = CellParameters::bare(0.05, 500.0);
        let dt_over_dx = 450.0_f64;
        let rhs = 5.49e-3 + dt_over_dx * 1.5e-3;
        // The LOW branch alone reports the jump.
        match solve_cell_on_branch(&cell, rhs, dt_over_dx, 0.0, 4.0e-3, RatingBranch::Low, None)
            .expect("low-branch solve runs")
        {
            CellSolve::Jump(_) => {}
            CellSolve::Root(h, q, _) => {
                panic!("expected LOW jump for the constructed line, got root h={h} q={q}")
            }
        }
        // The chain recovers a genuine HIGH-branch (turbulent) root.
        let (h, q, _) =
            solve_cell(&cell, rhs, dt_over_dx, 0.0, 4.0e-3, None).expect("chain resolves");
        // Mass identity holds exactly.
        assert!(
            (h + dt_over_dx * q - rhs).abs() <= 1.0e-10 * rhs,
            "cell equation must hold: h={h} q={q}"
        );
        // The committed q IS the converged HIGH-branch equilibrium at h —
        // i.e. a real root, not a filled-jump artifact.
        let q_eq_high = cell
            .equilibrium_discharge_converged(h, 0.0, CROSSOVER_Q_M2_S * 1.0e3, 1.0e-13, 80)
            .expect("high equilibrium");
        assert!(
            (q - q_eq_high).abs() <= 1.0e-9 * q_eq_high,
            "committed q {q} must equal the high-branch equilibrium {q_eq_high}"
        );
        assert!(
            q > CROSSOVER_Q_M2_S,
            "the recovered root must be on the turbulent side: q={q}"
        );
    }

    #[test]
    fn branch_warm_seed_preserves_solution_and_reduces_or_matches_map_work() {
        use super::super::profile;

        let _flag_guard = profile::test_flag_guard();
        let mut cell = CellParameters::bare(0.055, 500.0);
        cell.drag_coefficient = 1.0;
        cell.element_tip_height_m = 0.05;
        cell.roughness_concentration = 0.2;
        let rhs = 0.028_f64;
        let dt_over_dx = 180.0_f64;
        let h_start = 0.018_f64;

        profile::set_enabled(true);
        let _ = profile::snapshot_and_reset();
        let (h_cold, q_cold, _) =
            solve_cell(&cell, rhs, dt_over_dx, 0.0, h_start, None).expect("cold solve");
        let cold_profile = profile::snapshot_and_reset();

        let (h_warm, q_warm, _) =
            solve_cell(&cell, rhs, dt_over_dx, 0.0, h_start, Some(q_cold)).expect("warm solve");
        let warm_profile = profile::snapshot_and_reset();
        profile::set_enabled(false);

        assert!(
            (h_cold + dt_over_dx * q_cold - rhs).abs() <= 1.0e-10 * rhs,
            "cold residual"
        );
        assert!(
            (h_warm + dt_over_dx * q_warm - rhs).abs() <= 1.0e-10 * rhs,
            "warm residual"
        );
        assert!(
            (h_warm - h_cold).abs() <= 1.0e-10 * h_cold.max(1.0e-18),
            "warm seed must preserve the branch root h: cold={h_cold} warm={h_warm}"
        );
        assert!(
            (q_warm - q_cold).abs() <= 1.0e-10 * q_cold.max(1.0e-18),
            "warm seed must preserve the branch root q: cold={q_cold} warm={q_warm}"
        );
        assert!(
            warm_profile.implicit_equilibrium_map_evaluations
                <= cold_profile.implicit_equilibrium_map_evaluations,
            "warm map evals {} should not exceed cold {}",
            warm_profile.implicit_equilibrium_map_evaluations,
            cold_profile.implicit_equilibrium_map_evaluations
        );
        assert!(
            cold_profile.implicit_equilibrium_map_evaluations > 0,
            "non-bare fixture must exercise the generic fixed-point evaluator"
        );
        assert!(
            warm_profile.implicit_branch_evaluations > 0
                && cold_profile.implicit_branch_evaluations > 0,
            "branch evaluations must be counted"
        );
    }

    #[test]
    fn bare_skin_direct_equilibrium_avoids_fixed_point_map_work() {
        use super::super::profile;

        let _flag_guard = profile::test_flag_guard();
        let cell = CellParameters::bare(0.055, 500.0);
        let rhs = 0.028_f64;
        let dt_over_dx = 180.0_f64;
        let h_start = 0.018_f64;

        profile::set_enabled(true);
        let _ = profile::snapshot_and_reset();
        let (h, q, _) = solve_cell(&cell, rhs, dt_over_dx, 0.0, h_start, None).expect("bare solve");
        let snapshot = profile::snapshot_and_reset();
        profile::set_enabled(false);

        assert!(
            (h + dt_over_dx * q - rhs).abs() <= 1.0e-10 * rhs,
            "cell residual"
        );
        assert!(
            snapshot.implicit_branch_evaluations > 0,
            "outer branch residuals must still be counted"
        );
        assert_eq!(
            snapshot.implicit_equilibrium_map_evaluations, 0,
            "bare skin-only direct evaluator must not spend fixed-point map work"
        );
    }

    #[test]
    fn bare_skin_direct_equilibrium_composed_edge_cases_close_cell_residuals() {
        use super::super::profile;

        let _flag_guard = profile::test_flag_guard();
        let crossover_q = CROSSOVER_Q_M2_S;
        let laminar_edge_h =
            (crossover_q * 500.0 * KINEMATIC_VISCOSITY_M2_S / (8.0 * GRAVITY_M_S2 * 0.05)).cbrt();
        let cases = [
            (
                "rain-term",
                CellParameters::bare(0.05, 500.0),
                0.028,
                180.0,
                0.018,
                25.0,
            ),
            (
                "zero-ko",
                CellParameters::bare(0.05, 0.0),
                0.006,
                120.0,
                0.004,
                0.0,
            ),
            (
                "near-crossover",
                CellParameters::bare(0.05, 500.0),
                laminar_edge_h,
                240.0,
                laminar_edge_h * (1.0 - 1.0e-6),
                0.0,
            ),
        ];

        for (name, cell, rhs, dt_over_dx, h_start, rain_term) in cases {
            profile::set_enabled(true);
            let _ = profile::snapshot_and_reset();
            let (h, q, _) =
                solve_cell(&cell, rhs, dt_over_dx, rain_term, h_start, None).expect(name);
            let snapshot = profile::snapshot_and_reset();
            profile::set_enabled(false);

            assert!(
                (h + dt_over_dx * q - rhs).abs() <= 1.0e-10 * rhs.max(1.0e-18),
                "{name} cell residual"
            );
            assert!(
                snapshot.implicit_branch_evaluations > 0,
                "{name} branch residuals must be counted"
            );
            assert_eq!(
                snapshot.implicit_equilibrium_map_evaluations, 0,
                "{name} bare direct path must not execute fixed-point map work"
            );
        }
    }

    #[test]
    fn implicit_step_rejects_invalid_inactive_raw_operands_before_direct_path() {
        let mut invalid = CellParameters::bare(0.05, 500.0);
        invalid.roughness_concentration = 0.0;
        invalid.element_tip_height_m = f64::NAN;
        let mesh = KinematicWaveMesh::uniform(10.0, 2, invalid);
        let mut depths = vec![0.002_f64; 2];
        let source = vec![0.0_f64; 2];

        let err = implicit_step(&mesh, &mut depths, 0.0, &source, 0.0, 60.0)
            .expect_err("invalid inactive raw operand must fail closed");
        assert!(matches!(err, RoutingError::InvalidCellParameter));
    }

    #[test]
    fn branch_warm_seed_acceptance_is_basin_locked() {
        assert!(branch_accepts_seed(
            CROSSOVER_Q_M2_S * 0.5,
            RatingBranch::Low
        ));
        assert!(!branch_accepts_seed(
            CROSSOVER_Q_M2_S * 1.5,
            RatingBranch::Low
        ));
        assert!(branch_accepts_seed(
            CROSSOVER_Q_M2_S * 1.5,
            RatingBranch::High
        ));
        assert!(!branch_accepts_seed(
            CROSSOVER_Q_M2_S * 0.5,
            RatingBranch::High
        ));
        assert!(!branch_accepts_seed(f64::NAN, RatingBranch::Low));
        assert!(!branch_accepts_seed(0.0, RatingBranch::High));
    }

    /// T3-L1 (review-required accumulation vector): repeated dust-scale
    /// implicit steps on a short mesh must not accumulate a material
    /// ledger leak behind the dust-floor guard — the cumulative absolute
    /// residual over 10,000 steps stays far below one dry-threshold depth
    /// over the mesh.
    #[test]
    fn dust_scale_steps_do_not_accumulate_a_material_leak() {
        let mesh = bare_mesh(6.0, 3, 0.05, 500.0);
        let mut depths = vec![5.0e-10_f64; 3]; // sub-dry dust everywhere
        let zero = vec![0.0_f64; 3];
        let mut cumulative_residual = 0.0_f64;
        for _ in 0..10_000 {
            let outcome =
                implicit_step(&mesh, &mut depths, 0.0, &zero, 0.0, 900.0).expect("dust step");
            cumulative_residual += outcome.residual_m2().abs();
            assert!(depths.iter().all(|h| *h >= 0.0));
        }
        let dust_floor_m2 = 1.0e-9 * 6.0; // DRY_DEPTH_M * mesh length
        assert!(
            cumulative_residual < 0.01 * dust_floor_m2,
            "accumulated dust residual {cumulative_residual} must stay far below the mesh dust floor {dust_floor_m2}"
        );
    }

    #[test]
    fn implicit_step_ledger_is_exact_and_positive() {
        let mesh = bare_mesh(80.0, 10, 0.05, 500.0);
        let mut depths = vec![0.004_f64; 10];
        depths[3] = 0.02; // an uneven profile
        let v = vec![0.0_f64; 10];
        let before: f64 = depths.iter().sum::<f64>() * mesh.cell_length_m;
        let outcome =
            implicit_step(&mesh, &mut depths, 0.0, &v, 0.0, 900.0).expect("implicit step");
        assert!(depths.iter().all(|h| *h >= 0.0), "unconditional positivity");
        let after: f64 = depths.iter().sum::<f64>() * mesh.cell_length_m;
        // Booked storage matches the committed state exactly.
        assert!(((after - before) - outcome.storage_change_m2).abs() < 1.0e-15);
        // Conservation residual at solve-tolerance scale (no clamp term).
        let scale = before.max(outcome.outflow_m2);
        assert!(
            outcome.residual_m2().abs() <= 1.0e-11 * scale.max(1.0e-12),
            "residual {} vs scale {scale}",
            outcome.residual_m2()
        );
    }

    #[test]
    fn implicit_step_books_upstream_inflow_exactly() {
        let mesh = bare_mesh(40.0, 8, 0.06, 500.0);
        let mut depths = vec![0.001_f64; 8];
        let v = vec![0.0_f64; 8];
        let q_up = 2.5e-5;
        let before: f64 = depths.iter().sum::<f64>() * mesh.cell_length_m;
        let outcome =
            implicit_step(&mesh, &mut depths, q_up, &v, 0.0, 300.0).expect("implicit step");
        assert!((outcome.inflow_m2 - q_up * 300.0).abs() < 1.0e-18);
        let after: f64 = depths.iter().sum::<f64>() * mesh.cell_length_m;
        assert!(
            (outcome.inflow_m2 - outcome.outflow_m2 - (after - before)).abs()
                <= 1.0e-11 * outcome.inflow_m2.max(before),
        );
    }

    #[test]
    fn steady_state_is_a_fixed_point_of_the_implicit_step() {
        let mesh = bare_mesh(80.0, 10, 0.05, 500.0);
        let v = 60.0 / 3.6e6; // 60 mm/h
        let rain_term = 0.0;
        let steady = steady_profile(&mesh, v, rain_term);
        let mut depths = steady.clone();
        let sources = vec![v; 10];
        let outcome = implicit_step(&mesh, &mut depths, 0.0, &sources, rain_term, 900.0)
            .expect("implicit step");
        for (after, before) in depths.iter().zip(steady.iter()) {
            assert!(
                (after - before).abs() <= 1.0e-6 * before.max(1.0e-9),
                "steady profile must be a fixed point: {before} -> {after}"
            );
        }
        // Outlet discharges the full steady supply v·L.
        let q_expected = v * 80.0;
        assert!(
            (outcome.outlet_unit_discharge_m2_s - q_expected).abs() <= 1.0e-6 * q_expected,
            "steady outlet {} vs v*L {q_expected}",
            outcome.outlet_unit_discharge_m2_s
        );
    }

    /// The I1 acceptance ladder: recession from the steady profile, zero
    /// source. Reference = the EXPLICIT solver at fine dt (the D10B-validated
    /// scheme); the implicit outlet mass per 900 s bin must converge toward
    /// the reference as the implicit dt refines (first-order), and the
    /// TOTAL drained mass must agree to conservation-level accuracy at every
    /// dt (both schemes carry exact ledgers, so total-mass agreement is a
    /// storage-evolution fidelity check, not bookkeeping).
    #[test]
    #[ignore = "T3-I1 diagnostic: prints the full ladder + mesh-refinement probe"]
    fn recession_ladder_diagnostic() {
        for cells in [10_usize, 20, 40] {
            let mesh = bare_mesh(80.0, cells, 0.05, 500.0);
            let v = 60.0 / 3.6e6;
            let window_s = 6.0 * 3600.0;
            let bins = (window_s / BIN_S) as usize;
            let spin_up_s = 6.0 * 3600.0;
            let excess_on = move |_c: usize, _t: f64| v;
            let excess_off = |_c: usize, _t: f64| 0.0;
            let upstream = |_t: f64| 0.0;
            let intensity = |_t: f64| 0.0;
            let mut rs = KinematicWaveSolver::new(mesh.clone());
            let _ = rs
                .run(
                    &Forcing {
                        rainfall_excess_m_s: &excess_on,
                        upstream_inflow_m2_s: &upstream,
                        rainfall_intensity_m_s: &intensity,
                    },
                    spin_up_s,
                    BIN_S,
                    2.0,
                )
                .expect("spin-up");
            let initial_state = rs.depth_state().to_vec();
            let s0: f64 = initial_state.iter().sum::<f64>() * mesh.cell_length_m;
            let reference = rs
                .run(
                    &Forcing {
                        rainfall_excess_m_s: &excess_off,
                        upstream_inflow_m2_s: &upstream,
                        rainfall_intensity_m_s: &intensity,
                    },
                    window_s,
                    BIN_S,
                    2.0,
                )
                .expect("reference recession");
            let ref_bins: Vec<f64> = reference.outlet_bin_outflow_m2[..bins].to_vec();
            let ref_total: f64 = ref_bins.iter().sum();
            println!("cells={cells} S0={s0:.6} explicit_drained={ref_total:.6}");
            for dt in [900.0_f64, 300.0, 100.0, 30.0, 10.0] {
                let mut depths = initial_state.clone();
                let zero = vec![0.0_f64; cells];
                let steps_per_bin = (BIN_S / dt) as usize;
                let mut bins_m2 = vec![0.0_f64; bins];
                let mut iters = 0_u64;
                let mut resid_abs = 0.0_f64;
                let mut resid_max = 0.0_f64;
                let mut failed = false;
                'outer: for slot in &mut bins_m2 {
                    for _ in 0..steps_per_bin {
                        match implicit_step(&mesh, &mut depths, 0.0, &zero, 0.0, dt) {
                            Ok(o) => {
                                *slot += o.outflow_m2;
                                iters += o.solve_iterations;
                                let r = o.residual_m2().abs();
                                resid_abs += r;
                                if r > resid_max {
                                    resid_max = r;
                                }
                            }
                            Err(error) => {
                                println!("  dt={dt}: STEP FAILED {error:?}");
                                failed = true;
                                break 'outer;
                            }
                        }
                    }
                }
                if failed {
                    continue;
                }
                let total: f64 = bins_m2.iter().sum();
                let l1: f64 = bins_m2
                    .iter()
                    .zip(ref_bins.iter())
                    .map(|(a, b)| (a - b).abs())
                    .sum::<f64>()
                    / ref_total;
                println!(
                    "  dt={dt:>5}: drained={total:.6} l1_vs_explicit={l1:.4} iters={iters} resid_sum={resid_abs:.3e} resid_max={resid_max:.3e}"
                );
            }
        }
    }

    #[test]
    fn recession_converges_to_the_explicit_reference_under_dt_refinement() {
        let mesh = bare_mesh(80.0, 10, 0.05, 500.0);
        let v = 60.0 / 3.6e6;
        let rain_term = 0.0;
        let window_s = 6.0 * 3600.0;
        let bins = (window_s / BIN_S) as usize;

        // Explicit reference with IDENTICAL initial state: spin the explicit
        // solver up under the steady source, capture its committed state at
        // cutoff (the T3 hybrid handoff seam), then continue the SAME solver
        // through the recession window as the reference — successive runs
        // continue from the committed state.
        let spin_up_s = 6.0 * 3600.0;
        let excess_on = move |_cell: usize, _t: f64| v;
        let excess_off = |_cell: usize, _t: f64| 0.0;
        let upstream = |_t: f64| 0.0;
        let intensity = |_t: f64| 0.0;
        let mut reference_solver = KinematicWaveSolver::new(mesh.clone());
        let _spin_up = reference_solver
            .run(
                &Forcing {
                    rainfall_excess_m_s: &excess_on,
                    upstream_inflow_m2_s: &upstream,
                    rainfall_intensity_m_s: &intensity,
                },
                spin_up_s,
                BIN_S,
                2.0,
            )
            .expect("explicit spin-up run");
        let initial_state = reference_solver.depth_state().to_vec();
        let reference = reference_solver
            .run(
                &Forcing {
                    rainfall_excess_m_s: &excess_off,
                    upstream_inflow_m2_s: &upstream,
                    rainfall_intensity_m_s: &intensity,
                },
                window_s,
                BIN_S,
                2.0,
            )
            .expect("explicit reference recession");
        let reference_bins: Vec<f64> = reference.outlet_bin_outflow_m2[..bins].to_vec();
        let reference_total: f64 = reference_bins.iter().sum();

        // Implicit runs at three dt rungs from the IDENTICAL initial state.
        let mut errors = Vec::new();
        for dt in [900.0_f64, 300.0, 100.0] {
            let mut depths = initial_state.clone();
            let zero = vec![0.0_f64; 10];
            let steps_per_bin = (BIN_S / dt) as usize;
            let mut bins_m2 = vec![0.0_f64; bins];
            for (bin, slot) in bins_m2.iter_mut().enumerate() {
                for _ in 0..steps_per_bin {
                    let outcome = implicit_step(&mesh, &mut depths, 0.0, &zero, rain_term, dt)
                        .expect("implicit step");
                    *slot += outcome.outflow_m2;
                }
                let _ = bin;
            }
            let implicit_total: f64 = bins_m2.iter().sum();
            // Per-bin L1 error against the explicit reference, normalized by
            // the reference's drained mass.
            let l1: f64 = bins_m2
                .iter()
                .zip(reference_bins.iter())
                .map(|(a, b)| (a - b).abs())
                .sum();
            errors.push(l1 / reference_total.max(1.0e-18));
            // Total drained mass agreement (storage-evolution fidelity):
            // both schemes are conservative, so totals differ only through
            // end-state storage differences; require them close at every dt.
            assert!(
                (implicit_total - reference_total).abs() / reference_total < 0.05,
                "dt {dt}: drained totals {implicit_total} vs {reference_total}"
            );
        }
        // First-order refinement: error decreases monotonically and the
        // finest rung is materially better than the coarsest.
        assert!(
            errors[1] < errors[0] && errors[2] < errors[1],
            "per-bin errors must decrease under dt refinement: {errors:?}"
        );
        assert!(
            errors[2] < 0.6 * errors[0],
            "finest rung must materially improve on coarsest: {errors:?}"
        );
        // Measured coarse-dt scale (regression bound, NOT the ratified
        // tolerance — that is I2 scope with the Case-4/H2637 evidence): the
        // dt=900 per-bin L1 redistribution on this fixture measures ~0.43 of
        // drained mass (backward-Euler front-loads the recession); dt=300
        // measures ~0.21. Guard against regressions beyond the measured
        // scale.
        assert!(
            errors[0] < 0.6,
            "dt=900 per-bin L1 error regressed beyond the measured scale: {errors:?}"
        );
    }
}
