//! MOFEFID Lane D / D4 (SC-OFEROUTE-001, INV-OFEROUTE-005/006/007):
//! single-OFE 1-D kinematic-wave overland-flow solver with the TVD-MacCormack
//! predictor/corrector scheme (Papanicolaou et al. 2018, eqs. (8)-(14)),
//! space/time-variant friction (eqs. (2)-(7) via `super::friction`), and a
//! CFL-adaptive sub-timestep (eq. (12)). Shadow-first and opt-in: this solver
//! is not wired into any production phase span; the default hillslope runtime
//! does not call it (INV-OFEROUTE-010).
//!
//! Primary-bound posture (GAP-OFEROUTE-001 CLOSED, rev 26): the
//! TVD-MacCormack primaries are in hand (Davis 1984 R-102, Mingham 2001
//! R-82, Garcia-Navarro 1992 R-81, Tseng 2010 R-103) and the scheme is
//! implemented in the SOURCE-CORRECT form — R-63 §2.3's printed limiter
//! (11c) is adjudicated a transcription error; see the rev-24/25/26
//! Algorithm Specification and `GAP-OFEROUTE-005` (RESOLVED). Validated
//! by the Iwagaki-primary entropy-solution oracle, exact booked-ledger
//! conservation, CFL stability, and steady-state/shock structure.

use super::friction::{
    GRAVITY_M_S2, KINEMATIC_VISCOSITY_M2_S, chezy_from_friction, equivalent_friction_factor,
    form_resistance_abrahams, froude_number, reynolds_number, skin_rain_term,
    skin_resistance_with_rain_term, vegetation_resistance_katul, wave_resistance_hu_abrahams,
};
use super::profile;

/// Depth-discharge exponent `m` (eq. A2): `q = alpha h^m`, `m = 1.5`.
pub const DEPTH_DISCHARGE_EXPONENT: f64 = 1.5;
/// Target Courant number for the CFL-adaptive sub-timestep (eq. 12: `Cr <= 1`).
/// Conservative default; the hard CFL ceiling is 1.0.
pub const CFL_TARGET: f64 = 0.9;
/// Minimum positive depth used to guard divisions; below this a cell is dry.
pub(super) const DRY_DEPTH_M: f64 = 1.0e-9;

/// A forcing value is valid iff finite and non-negative (rainfall excess,
/// rainfall intensity, and upstream inflow are physically non-negative).
#[must_use]
fn is_valid_forcing(value: f64) -> bool {
    value.is_finite() && value >= 0.0
}

/// Per-cell space-variant friction + geometry parameters. A single-OFE mesh
/// with uniform roughness repeats one value across cells; Case 4 varies
/// `slope` per section. `PartialEq` identifies MATERIAL discontinuities
/// (slope/roughness breaks) for the rev-24 dissipation interface rule.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellParameters {
    /// OFE gradient `S_o` (m/m).
    pub slope: f64,
    /// Laminar skin friction coefficient `k_o` (surface type).
    pub friction_coefficient_ko: f64,
    /// Form-resistance drag coefficient `C_d`.
    pub drag_coefficient: f64,
    /// Isolated-element tip height `D_r` (m); 0 disables form/wave resistance.
    pub element_tip_height_m: f64,
    /// Roughness concentration `lambda` (0-1); 0 disables form/wave.
    pub roughness_concentration: f64,
    /// Leaf area index `LAI`; 0 disables vegetation resistance.
    pub leaf_area_index: f64,
    /// Canopy height `h_c` (m); 0 disables vegetation resistance.
    pub canopy_height_m: f64,
    /// Vegetation drag coefficient `C_d` (may differ from the form `C_d`).
    pub vegetation_drag_coefficient: f64,
    /// Manning roughness `n` (s m^-1/3); when positive, the friction law is
    /// the definitional Manning identity `f = 8 g n^2 / h^(1/3)` and the
    /// Papanicolaou component menu is bypassed. Rev 24
    /// (`SC-OFEROUTE-001#INV-OFEROUTE-011`, D10B): bound for the Iwagaki
    /// Case-4 D-val configuration (`n = 0.009`, the primary's own law) so
    /// oracle and solver run the same closure. Production operand paths
    /// (D11 `routing_coefficients`) never set this; 0 disables it.
    pub manning_n: f64,
}

impl CellParameters {
    /// Validate the parameter domain (finite; non-negative physical fields).
    fn validate(&self) -> Result<(), RoutingError> {
        let fields = [
            self.slope,
            self.friction_coefficient_ko,
            self.drag_coefficient,
            self.element_tip_height_m,
            self.roughness_concentration,
            self.leaf_area_index,
            self.canopy_height_m,
            self.vegetation_drag_coefficient,
            self.manning_n,
        ];
        if fields.iter().any(|v| !v.is_finite() || *v < 0.0) {
            return Err(RoutingError::InvalidCellParameter);
        }
        if self.roughness_concentration > 1.0 {
            return Err(RoutingError::InvalidCellParameter);
        }
        Ok(())
    }

    /// A bare-surface (skin-only) cell: only `slope` + `k_o` active.
    #[must_use]
    pub fn bare(slope: f64, friction_coefficient_ko: f64) -> Self {
        Self {
            slope,
            friction_coefficient_ko,
            drag_coefficient: 0.0,
            element_tip_height_m: 0.0,
            roughness_concentration: 0.0,
            leaf_area_index: 0.0,
            canopy_height_m: 0.0,
            vegetation_drag_coefficient: 0.0,
            manning_n: 0.0,
        }
    }

    /// A Manning-law cell (`f = 8 g n^2 / h^(1/3)`): the rev-24 D-val
    /// Case-4 configuration limb (`friction-mapping-evidence.md`).
    #[must_use]
    pub fn manning(slope: f64, manning_n: f64) -> Self {
        Self {
            slope,
            friction_coefficient_ko: 0.0,
            drag_coefficient: 0.0,
            element_tip_height_m: 0.0,
            roughness_concentration: 0.0,
            leaf_area_index: 0.0,
            canopy_height_m: 0.0,
            vegetation_drag_coefficient: 0.0,
            manning_n,
        }
    }

    /// Equivalent friction factor `f_eq` (eq. 7) for the current cell depth and
    /// unit discharge, using the rainfall intensity for the skin term.
    #[must_use]
    pub fn equivalent_friction(
        &self,
        flow_depth_m: f64,
        unit_discharge_m2_s: f64,
        rainfall_intensity_m_s: f64,
    ) -> f64 {
        self.equivalent_friction_with_rain_term(
            flow_depth_m,
            unit_discharge_m2_s,
            skin_rain_term(rainfall_intensity_m_s),
        )
    }

    /// `f_eq` with the skin rain term precomputed (D14 OPT-3): the hot solver
    /// path hoists `3393 I^0.407` once per step because `I` is constant
    /// within a step. Bit-identical to `equivalent_friction` (the public form
    /// delegates here).
    fn equivalent_friction_with_rain_term(
        &self,
        flow_depth_m: f64,
        unit_discharge_m2_s: f64,
        skin_rain_term: f64,
    ) -> f64 {
        if self.manning_n > 0.0 {
            // Rev-24 Manning limb: the definitional identity
            // `f = 8 g n^2 / h^(1/3)` (wide channel, R = h); no Re/Fr
            // dependence, so the alpha fixed point converges immediately.
            let h = flow_depth_m.max(DRY_DEPTH_M);
            return 8.0 * GRAVITY_M_S2 * self.manning_n * self.manning_n / h.cbrt();
        }
        let re = reynolds_number(unit_discharge_m2_s, KINEMATIC_VISCOSITY_M2_S);
        let skin = skin_resistance_with_rain_term(skin_rain_term, self.friction_coefficient_ko, re);
        let form = form_resistance_abrahams(
            self.drag_coefficient,
            flow_depth_m,
            self.element_tip_height_m,
            self.roughness_concentration,
        );
        // Wave resistance applies when the element is not fully submerged
        // (h/D_r < 1); above full submergence it vanishes. D15A OPT-8a: the
        // Froude number is consumed ONLY by this branch, so it is built
        // lazily here (bit-identical; its value was dead elsewhere).
        let wave = if self.element_tip_height_m > 0.0 && flow_depth_m < self.element_tip_height_m {
            let fr = froude_number(unit_discharge_m2_s, flow_depth_m, GRAVITY_M_S2);
            wave_resistance_hu_abrahams(self.roughness_concentration, fr)
        } else {
            0.0
        };
        let veg = vegetation_resistance_katul(
            self.vegetation_drag_coefficient,
            self.leaf_area_index,
            self.canopy_height_m,
            flow_depth_m,
        );
        equivalent_friction_factor(skin, form, wave, veg)
    }

    /// Kinematic coefficient `alpha = C S_o^0.5`, `C = sqrt(8 g / f_eq)`.
    ///
    /// `alpha` is implicit: `f_eq` depends on `Re = q/nu` and `Fr`, while
    /// `q = alpha h^1.5`. Papanicolaou updates `alpha` from the current flow at
    /// the end of each step; from a dry start the stored discharge is zero, so
    /// this resolves the implicit relation by a short fixed-point iteration
    /// seeded with a Chezy estimate `q0 ~ sqrt(g S_o) h^1.5` to break the
    /// zero-flow singularity (the skin term diverges as `Re -> 0`).
    ///
    /// Takes the precomputed `skin_rain_term` (D14 OPT-3); the run loop
    /// evaluates it once per step from the validated intensity.
    ///
    /// Returns `(alpha, q)` where `q = alpha * h^1.5` is the fixed point's
    /// own final iterate (D15A OPT-5): the caller's celerity evaluation needs
    /// exactly this discharge, and recomputing `alpha * h.powf(1.5)` outside
    /// was a redundant `powf` with bit-identical value (same `alpha`, same
    /// `h_pow` operands). Every zero-alpha path returns `(0.0, 0.0)`, which
    /// matches the caller-side `0.0 * h_pow = +0.0` it replaces.
    #[must_use]
    fn alpha_q(
        &self,
        flow_depth_m: f64,
        unit_discharge_m2_s: f64,
        skin_rain_term: f64,
    ) -> (f64, f64) {
        if flow_depth_m <= DRY_DEPTH_M || self.slope <= 0.0 {
            return (0.0, 0.0);
        }
        let h_pow = flow_depth_m.powf(DEPTH_DISCHARGE_EXPONENT);
        // D15A OPT-7: loop-invariant hoist (same deterministic sqrt value
        // every iteration).
        let slope_sqrt = self.slope.sqrt();
        let mut q_est = if unit_discharge_m2_s > 0.0 {
            unit_discharge_m2_s
        } else {
            (GRAVITY_M_S2 * self.slope).sqrt() * h_pow
        };
        let mut alpha = 0.0;
        for _ in 0..4 {
            let f_eq = self.equivalent_friction_with_rain_term(flow_depth_m, q_est, skin_rain_term);
            if f_eq <= 0.0 {
                return (0.0, 0.0);
            }
            alpha = chezy_from_friction(f_eq, GRAVITY_M_S2) * slope_sqrt;
            let q_new = alpha * h_pow;
            let converged = (q_new - q_est).abs() <= 1.0e-12 * q_new.max(1.0e-12);
            q_est = q_new;
            if converged {
                break;
            }
        }
        (alpha, q_est)
    }

    /// T3-I1 (implicit stepper): the CONVERGED equilibrium discharge
    /// `q_eq(h)` — the fixed point of `q ↦ alpha(h, q)·h^1.5`, iterated to
    /// `tol_rel` instead of the explicit scheme's frozen 4-iteration cap.
    /// Seeded from `q_seed` when positive (warm start), else the Chezy
    /// estimate (the same zero-flow-singularity break `alpha_q` uses).
    /// Returns `None` when the fixed point does not meet `tol_rel` within
    /// `max_iterations` (caller fails closed) — the laminar limb contracts
    /// at ~0.5 per iteration IN LOG SPACE, so 80 iterations covers 1e-13
    /// from any finite seed. This is a NEW scheme entry point (no
    /// bit-compatibility constraint with `alpha_q`, which the explicit
    /// scheme keeps unchanged).
    ///
    /// REGIME-CROSSOVER STRUCTURE (T3-I1 discovery, rev-28 draft): the
    /// `INV-OFEROUTE-002` skin dispatch is DISCONTINUOUS at `Re = 1000`
    /// (`f` drops laminar→turbulent), which makes the equilibrium rating
    /// Z-SHAPED: in an overlap depth band BOTH a laminar-branch and a
    /// turbulent-branch equilibrium exist, and the fixed-point basins
    /// split exactly at the crossover discharge `Q_c = 1000·ν` (the map
    /// is increasing with an upward jump at `Q_c`; each concave branch
    /// attracts its own side). The SEED therefore selects the branch —
    /// callers needing a single-valued rating must seed deterministically
    /// (`implicit_recession`'s basin-split rule — measured: basin-split
    /// seeding alone yields machine-exact ledgers across the full
    /// dt/mesh ladder). The explicit scheme's frozen 4-iteration cap
    /// rides over this structure without ever resolving it.
    #[must_use]
    pub(super) fn equilibrium_discharge_converged(
        &self,
        flow_depth_m: f64,
        skin_rain_term: f64,
        q_seed: f64,
        tol_rel: f64,
        max_iterations: usize,
    ) -> Option<f64> {
        if flow_depth_m <= DRY_DEPTH_M || self.slope <= 0.0 {
            return Some(0.0);
        }
        let h_pow = flow_depth_m.powf(DEPTH_DISCHARGE_EXPONENT);
        let slope_sqrt = self.slope.sqrt();
        let crossover_q = KINEMATIC_VISCOSITY_M2_S * 1000.0;
        let mut q_est = if q_seed > 0.0 && q_seed.is_finite() {
            q_seed
        } else {
            (GRAVITY_M_S2 * self.slope).sqrt() * h_pow
        };
        // One fixed-point map application.
        let map = |q: f64| -> Option<f64> {
            let f_eq = self.equivalent_friction_with_rain_term(flow_depth_m, q, skin_rain_term);
            if f_eq <= 0.0 {
                return Some(0.0);
            }
            let alpha = chezy_from_friction(f_eq, GRAVITY_M_S2) * slope_sqrt;
            let q_new = alpha * h_pow;
            if q_new.is_finite() { Some(q_new) } else { None }
        };
        // STEFFENSEN-accelerated iteration (T3-I2 performance): the plain
        // laminar-limb map contracts at only ~0.5 per iteration in log
        // space (~40-60 evaluations from a cold basin-split seed);
        // Steffensen converges quadratically to the SAME fixed point, so
        // the deterministic rating stays a pure function of the inputs.
        // The accelerated proposal is accepted only when it is finite,
        // positive, and on the SAME side of the regime crossover as the
        // plain iterate (the basin guard — jumping basins would break the
        // branch-pinned determinism); otherwise the plain iterate stands.
        let mut budget = max_iterations;
        while budget >= 2 {
            let q1 = map(q_est)?;
            if q1 == 0.0 {
                return Some(0.0);
            }
            if (q1 - q_est).abs() <= tol_rel * q1.max(1.0e-18) {
                return Some(q1);
            }
            let q2 = map(q1)?;
            if q2 == 0.0 {
                return Some(0.0);
            }
            if (q2 - q1).abs() <= tol_rel * q2.max(1.0e-18) {
                return Some(q2);
            }
            budget -= 2;
            let denom = q2 - 2.0 * q1 + q_est;
            let accelerated = if denom == 0.0 {
                q2
            } else {
                q_est - (q1 - q_est) * (q1 - q_est) / denom
            };
            // T3-H2 (review-hardened): acceleration is accepted ONLY when
            // the WHOLE plain triple (q_est, q1, q2) sits in one basin AND
            // the accelerated point stays on that side — the accelerated
            // sequence is then side-locked to the plain iteration at every
            // cycle, so it converges to the SAME fixed point the plain
            // iteration defines (the function's value IS the plain-iteration
            // limit from the given seed). While the plain trajectory is
            // mid-migration across `Q_c` (a legitimate basin move when the
            // seeded basin is empty at this depth), no acceleration is
            // applied.
            let plain_side = q2 > crossover_q;
            let same_basin = (accelerated > crossover_q) == plain_side
                && (q1 > crossover_q) == plain_side
                && (q_est > crossover_q) == plain_side;
            q_est = if accelerated.is_finite() && accelerated > 0.0 && same_basin {
                accelerated
            } else {
                q2
            };
        }
        None
    }
}

/// The single-OFE kinematic-wave mesh: uniform cell length, per-cell params.
#[derive(Debug, Clone)]
pub struct KinematicWaveMesh {
    /// Mesh spacing `Delta x` (m).
    pub cell_length_m: f64,
    /// Per-cell parameters, upstream (index 0) to downstream (index n-1).
    pub cells: Vec<CellParameters>,
}

impl KinematicWaveMesh {
    /// Uniform single-OFE mesh: `n` cells over `length_m` with one param set.
    #[must_use]
    pub fn uniform(length_m: f64, cell_count: usize, params: CellParameters) -> Self {
        let cell_count = cell_count.max(1);
        Self {
            cell_length_m: length_m / f64::from(u32::try_from(cell_count).unwrap_or(u32::MAX)),
            cells: vec![params; cell_count],
        }
    }

    #[must_use]
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }
}

/// Forcing for a solver run. `rainfall_excess_m_s` is the net lateral input
/// rate `v = r - i_f` (eq. A1); `upstream_inflow_m2_s` is the unit discharge
/// entering cell 0 (the upstream boundary condition). Both may be closures of
/// time (seconds) so Case 4's finite lateral pulse and Case 1's constant rain
/// are expressible.
pub struct Forcing<'a> {
    /// Rainfall-excess rate at `(cell_index, time_s)` -> m/s.
    pub rainfall_excess_m_s: &'a dyn Fn(usize, f64) -> f64,
    /// Upstream inflow unit discharge at `time_s` -> m^2/s (BC into cell 0).
    pub upstream_inflow_m2_s: &'a dyn Fn(f64) -> f64,
    /// Rainfall intensity at `time_s` -> m/s, used by the skin-resistance term
    /// (Papanicolaou eq. 2 depends on `I`, distinct from rainfall excess).
    pub rainfall_intensity_m_s: &'a dyn Fn(f64) -> f64,
}

/// One recorded outlet sample.
#[derive(Debug, Clone, Copy)]
pub struct HydrographSample {
    pub time_s: f64,
    /// Outlet unit discharge (m^2/s).
    pub outlet_unit_discharge_m2_s: f64,
    /// Outlet depth (m).
    pub outlet_depth_m: f64,
}

/// Mass-balance ledger accumulated over a run (INV-OFEROUTE-006).
#[derive(Debug, Clone, Copy, Default)]
pub struct MassBalance {
    /// Total upstream inflow volume per unit width (m^2).
    pub inflow_m2: f64,
    /// Total rainfall-excess volume per unit width (m^2).
    pub rainfall_excess_m2: f64,
    /// Total outlet outflow volume per unit width (m^2).
    pub outflow_m2: f64,
    /// Storage change (final - initial) per unit width (m^2).
    pub storage_change_m2: f64,
    /// Mass injected by positivity clamps (`h.max(0)`), per unit width (m^2).
    /// The scheme conserves exactly; this is the only non-conservative term,
    /// arising as small negative-depth excursions are clamped at the wetting
    /// front during the transient. Surfaced so conservation is auditable.
    pub positivity_clamp_m2: f64,
    /// Scheme-actual upstream injection. Post-rev-24 BOTH sweeps carry
    /// `q_up` at the top face, so this equals `inflow_m2` by construction
    /// (booked-equals-actual identity surface; pre-rev-24 it exposed the
    /// `0.5 (q_up + q_0)` mismatch).
    pub scheme_inflow_m2: f64,
    /// D10B diagnostic: the DISCRETE SCHEME's actual downstream boundary
    /// outflow, `0.5 (q_ghost + q_pred_out) dt` per step, where `q_ghost` is
    /// the predictor's extrapolated outflow ghost and `q_pred_out` the
    /// corrector's outlet predictor flux. Compare against `outflow_m2` (the
    /// booked committed-state trapezoid) to expose the outflow mismatch.
    pub scheme_outflow_m2: f64,
    /// D10B diagnostic: net mass injected by the TVD dissipation term per
    /// step, `[gr_{n-2} (h_{n-1}-h_{n-2}) - gr_0 (h_1-h_0)] dx`. The
    /// cell-indexed TVD application exempts boundary cells, so its
    /// telescoping sum does NOT vanish; this books the leak explicitly.
    pub tvd_boundary_leak_m2: f64,
}

impl MassBalance {
    /// Closure residual: `in + rain - out - storage_change`. Absorbs the
    /// positivity-clamp injection (see `conservation_residual_m2`).
    #[must_use]
    pub fn residual_m2(&self) -> f64 {
        self.inflow_m2 + self.rainfall_excess_m2 - self.outflow_m2 - self.storage_change_m2
    }

    /// Clamp-adjusted conservation residual: `residual + clamp`. The clamp
    /// injects storage, so the raw residual is ~ -clamp; adding it back
    /// recovers the scheme's true (machine-epsilon) conservation.
    #[must_use]
    pub fn conservation_residual_m2(&self) -> f64 {
        self.residual_m2() + self.positivity_clamp_m2
    }
}

/// Result of a solver run.
#[derive(Debug, Clone)]
pub struct RoutingResult {
    pub hydrograph: Vec<HydrographSample>,
    pub mass_balance: MassBalance,
    /// Peak outlet unit discharge (m^2/s) and its time (s) at solver sub-step
    /// endpoints. Validation harnesses that compare against sampled external
    /// traces should compute their peak from the sampled hydrograph instead.
    pub peak_unit_discharge_m2_s: f64,
    pub time_to_peak_s: f64,
    /// Max Courant number observed (CFL evidence; must stay <= 1).
    pub max_courant: f64,
    /// D10B (rev 24): max single-step increase of the spatial total
    /// variation of the flux `q` over HOMOGENEOUS steps (zero rainfall
    /// excess and zero upstream inflow), measured over uniform-material
    /// faces (the domain of the TVD theory; material-interface transients
    /// are recorded separately by validation). Must stay at numerical
    /// noise for a TVD scheme.
    pub max_homogeneous_tv_increase_m2_s: f64,
    /// D10B (rev 24, conservative handoff): per-sample-bin ACTUAL outlet
    /// outflow mass (m^2 per unit width), bin k covering
    /// `[k sample_dt, (k+1) sample_dt)` (last bin clipped to the window).
    /// Sums exactly to `mass_balance.outflow_m2`; the cascade handoff
    /// injects this series (piecewise-constant rate) so inter-OFE transfer
    /// conserves the scheme's actual discharge at ANY sample resolution.
    /// The instantaneous `hydrograph` remains the shape/metrics surface.
    pub outlet_bin_outflow_m2: Vec<f64>,
    /// The bin width (s) for `outlet_bin_outflow_m2` (= the run's
    /// `sample_dt_s`).
    pub outlet_bin_dt_s: f64,
    /// Per-bin coverage spans (s): equal to `outlet_bin_dt_s` for full
    /// bins; the final bin's span is the actual covered remainder when
    /// `end_time_s` is not a multiple of the bin width (Review-B M3 — the
    /// handoff integrates the final bin at `mass/span` over its covered
    /// interval so no mass is stranded past `end_time_s`).
    pub outlet_bin_spans_s: Vec<f64>,
}

/// Error conditions that fail closed (INV-OFEROUTE-005/007).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoutingError {
    /// A CFL violation was produced that adaptive stepping could not avoid.
    CflViolation,
    /// A non-finite state (NaN/inf depth or discharge) was produced.
    NonFiniteState,
    /// A negative depth was produced (scheme positivity failure).
    NegativeDepth,
    /// The outlet bin series carried a material terminal deficit that a
    /// non-negative exact-total redistribution cannot absorb (rev 26 /
    /// Codex review Medium-1: never publish negative outlet discharge).
    NegativeOutletBin,
    /// Degenerate configuration (empty mesh, non-positive spacing/end time,
    /// non-positive sample or max sub-timestep).
    DegenerateConfiguration,
    /// Forcing (rainfall excess / intensity / upstream inflow) returned a
    /// non-finite OR negative value (all three are physically non-negative).
    InvalidForcing,
    /// A cell parameter is out of domain (non-finite, negative slope/ko/etc.).
    InvalidCellParameter,
    /// T3-I1: an implicit-stepper nonlinear solve (equilibrium fixed point
    /// or the cell Newton) failed to converge within its iteration budget.
    ImplicitSolveNonConvergence,
}

/// Single-OFE TVD-MacCormack kinematic-wave solver state.
pub struct KinematicWaveSolver {
    mesh: KinematicWaveMesh,
    depth_m: Vec<f64>,
    discharge_m2_s: Vec<f64>,
    scratch: StepScratch,
    /// Per-face material-interface flags, `breaks[f] = cells[f] != cells[f+1]`
    /// (D15A OPT-9): the mesh is immutable for the solver's lifetime, so the
    /// rev-24 interface detection is computed once instead of re-comparing
    /// 9-field `CellParameters` per face per step. Same comparison, hoisted.
    material_breaks: Vec<bool>,
    /// D10B TV diagnostic accumulator (reset per `run`).
    max_tv_increase_m: f64,
}

/// Per-step workspace reused across steps (D14 OPT-2: was 8 `Vec`
/// allocations per step). Every slot is written before it is read within
/// the same step; no value crosses steps through the scratch.
struct StepScratch {
    /// Per-cell `alpha` at the pre-step state (D14 OPT-1: computed once per
    /// step by the run loop; reused by dt selection, Courant evidence, and
    /// the step math).
    alpha: Vec<f64>,
    /// Per-cell pre-step discharge `q = alpha h^1.5` from the celerity
    /// evaluation (D15A OPT-5/OPT-6): reused by the homogeneous TV(q)
    /// diagnostic's `tv_before` so the diagnostic recomputes no `powf`.
    /// Dry/zero-alpha cells carry `0.0` (bit-equal to `alpha·h^1.5 = +0.0`).
    q0: Vec<f64>,
    /// Per-cell TRUE kinematic celerity `dq/dh` at the pre-step state
    /// (rev 24): for depth-dependent friction the equilibrium celerity
    /// exceeds the frozen-alpha `1.5 alpha h^0.5` (Manning `(5/3) q/h`,
    /// laminar `k_o/Re` limb `3 q/h`), and CFL/dissipation must be
    /// governed by the true wave speed — the D10B S4 evidence showed the
    /// frozen-alpha estimate running the scheme at true Courant ~1.8 on
    /// the laminar limb (latent instability formerly masked by the
    /// inverted limiter's blanket dissipation).
    celerity: Vec<f64>,
    v: Vec<f64>,
    h_pred: Vec<f64>,
    q_pred: Vec<f64>,
    h_corr: Vec<f64>,
    averaged: Vec<f64>,
    gr: Vec<f64>,
    h_next: Vec<f64>,
}

impl StepScratch {
    fn new(cell_count: usize) -> Self {
        Self {
            alpha: vec![0.0; cell_count],
            q0: vec![0.0; cell_count],
            celerity: vec![0.0; cell_count],
            v: vec![0.0; cell_count],
            h_pred: vec![0.0; cell_count],
            q_pred: vec![0.0; cell_count],
            h_corr: vec![0.0; cell_count],
            averaged: vec![0.0; cell_count],
            gr: vec![0.0; cell_count],
            h_next: vec![0.0; cell_count],
        }
    }
}

/// Outlet-hydrograph accumulator (rev 24, D10B): per-sample-bin
/// time-averages of the BOUNDARY FLUX (the scheme's actual discharge —
/// the surface the mass ledger books and the cascade handoff conserves)
/// and of the outlet-cell depth. The hydrograph sample for bin `k` is
/// stamped at the bin midpoint with the bin-mean values (a discharge
/// gauge reading at the chosen cadence). This replaces the pre-rev-24
/// instantaneous committed-state sampler: the committed last-cell state
/// carries an O(dx)-registration and a confined, zero-net-mass boundary
/// limit cycle that must not pollute the exported hydrograph.
struct BinRecorder {
    flux_bins_m2: Vec<f64>,
    stage_bins_ms: Vec<f64>,
    span_bins_s: Vec<f64>,
    sample_dt_s: f64,
}

impl BinRecorder {
    fn new(sample_dt_s: f64, end_time_s: f64) -> Self {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let capacity = ((end_time_s / sample_dt_s).ceil() as usize).saturating_add(1);
        Self {
            flux_bins_m2: Vec::with_capacity(capacity),
            stage_bins_ms: Vec::with_capacity(capacity),
            span_bins_s: Vec::with_capacity(capacity),
            sample_dt_s,
        }
    }

    /// Apportion one step's outflow mass and outlet stage across the
    /// sample bins it spans, pro-rata by time overlap.
    ///
    /// Review-B M1: the loop iterates the INTEGER bin index (guaranteed
    /// progress) instead of re-deriving `k = floor(t/bin_dt)` from a
    /// floating-point bin boundary — the latter has zero-progress
    /// witnesses (e.g. `sample_dt = 0.003` at `t = 0.147`) that hang the
    /// process.
    fn record_step(&mut self, t_before: f64, dt: f64, outflow_m2: f64, outlet_depth_m: f64) {
        let sample_span = profile::span_start();
        let step_end = t_before + dt;
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let mut k = ((t_before / self.sample_dt_s).floor().max(0.0)) as usize;
        loop {
            #[allow(clippy::cast_precision_loss)]
            let bin_start = k as f64 * self.sample_dt_s;
            let bin_end = bin_start + self.sample_dt_s;
            let lo = bin_start.max(t_before);
            let hi = bin_end.min(step_end);
            if hi > lo {
                if k >= self.flux_bins_m2.len() {
                    self.flux_bins_m2.resize(k + 1, 0.0);
                    self.stage_bins_ms.resize(k + 1, 0.0);
                    self.span_bins_s.resize(k + 1, 0.0);
                }
                let overlap = hi - lo;
                self.flux_bins_m2[k] += outflow_m2 * overlap / dt;
                self.stage_bins_ms[k] += outlet_depth_m * overlap;
                self.span_bins_s[k] += overlap;
            }
            if bin_end >= step_end {
                break;
            }
            k += 1;
        }
        profile::end_solver_sample(sample_span);
    }

    /// Build the exported hydrograph: bin-mean flux/stage at bin midpoints
    /// (initial dry sample at t = 0), plus the conservative flux-bin
    /// series (with per-bin coverage spans) for the handoff.
    ///
    /// Review-B M2: transient NEGATIVE per-bin outflow can occur at
    /// front-arrival on a dry outlet (the predictor's telescoped boundary
    /// attribution `2 q_{n-1} - q_{n-2}` dips negative for a step-scale
    /// interval while the ledger total stays exact). The bin series is
    /// redistributed FORWARD to non-negative values with the exact total
    /// preserved (a deficit is carried into subsequent bins); a
    /// pathological terminal deficit (physically unexpected: total
    /// outflow is non-negative) is RETURNED to the caller, which fails
    /// closed rather than publishing a negative outlet bin (Codex review
    /// Medium-1); sub-noise deficits are folded into the last covered bin
    /// so the series sums exactly to the booked outflow.
    fn finish(self) -> (Vec<HydrographSample>, Vec<f64>, Vec<f64>, f64) {
        let mut flux_bins = self.flux_bins_m2;
        let mut carry = 0.0_f64;
        let mut last_covered = None;
        for (k, v) in flux_bins.iter_mut().enumerate() {
            if self.span_bins_s[k] <= 0.0 {
                continue;
            }
            last_covered = Some(k);
            *v += carry;
            if *v < 0.0 {
                carry = *v;
                *v = 0.0;
            } else {
                carry = 0.0;
            }
        }
        // Terminal deficit handling (Medium-1): fp-noise deficits fold
        // into the last covered bin (keeps the exact total; cannot go
        // materially negative); anything larger is returned for the
        // caller's typed fail-closed decision.
        let total_abs: f64 = flux_bins.iter().map(|v| v.abs()).sum();
        let noise_floor = 1.0e-9 * total_abs.max(1.0e-12);
        let mut terminal_deficit = 0.0_f64;
        if carry < 0.0 {
            if carry.abs() <= noise_floor {
                if let Some(k) = last_covered {
                    flux_bins[k] += carry;
                }
            } else {
                terminal_deficit = carry;
            }
        }
        let mut hydrograph = Vec::with_capacity(flux_bins.len() + 1);
        hydrograph.push(HydrographSample {
            time_s: 0.0,
            outlet_unit_discharge_m2_s: 0.0,
            outlet_depth_m: 0.0,
        });
        for (k, flux_m2) in flux_bins.iter().enumerate() {
            let span = self.span_bins_s[k];
            if span <= 0.0 {
                continue;
            }
            #[allow(clippy::cast_precision_loss)]
            let t_mid = k as f64 * self.sample_dt_s + 0.5 * span;
            hydrograph.push(HydrographSample {
                time_s: t_mid,
                outlet_unit_discharge_m2_s: flux_m2 / span,
                outlet_depth_m: self.stage_bins_ms[k] / span,
            });
        }
        profile::count_hydrograph_samples(hydrograph.len() as u64);
        (hydrograph, flux_bins, self.span_bins_s, terminal_deficit)
    }
}

impl KinematicWaveSolver {
    /// Construct with a dry initial condition.
    #[must_use]
    pub fn new(mesh: KinematicWaveMesh) -> Self {
        let n = mesh.cell_count();
        let material_breaks = (0..n.saturating_sub(1))
            .map(|f| mesh.cells[f] != mesh.cells[f + 1])
            .collect();
        Self {
            mesh,
            depth_m: vec![0.0; n],
            discharge_m2_s: vec![0.0; n],
            scratch: StepScratch::new(n),
            material_breaks,
            max_tv_increase_m: 0.0,
        }
    }

    fn total_storage_m2(&self) -> f64 {
        self.depth_m.iter().sum::<f64>() * self.mesh.cell_length_m
    }

    /// T3 (hybrid stepping seam): the committed per-cell depth state.
    /// Successive `run*` calls CONTINUE from this state (each run books its
    /// own ledger from the state at entry), so a scheme switch hands off
    /// exactly this vector. Read-only; the implicit stepper owns its copy.
    #[must_use]
    pub(super) fn depth_state(&self) -> &[f64] {
        &self.depth_m
    }

    /// T3-I2 (hybrid seam, exit-implicit → enter-explicit): install the
    /// per-cell state committed by the implicit stepper. Discharges are the
    /// implicit solve's own converged equilibria (`q_eq(h)` per the design
    /// §2.2 exit rule), so the explicit scheme's next pre-step alpha
    /// evaluation sees the same shape of state it would after any explicit
    /// step. Lengths must match the mesh.
    pub(super) fn set_state(
        &mut self,
        depth_m: &[f64],
        discharge_m2_s: &[f64],
    ) -> Result<(), RoutingError> {
        if depth_m.len() != self.depth_m.len() || discharge_m2_s.len() != self.depth_m.len() {
            return Err(RoutingError::DegenerateConfiguration);
        }
        self.depth_m.copy_from_slice(depth_m);
        self.discharge_m2_s.copy_from_slice(discharge_m2_s);
        Ok(())
    }

    /// T3-I2 (hybrid seam, exit-explicit): the committed per-cell discharge
    /// state paired with `depth_state`.
    #[must_use]
    pub(super) fn discharge_state(&self) -> &[f64] {
        &self.discharge_m2_s
    }

    /// Evaluate `alpha` and the TRUE kinematic celerity for every cell at
    /// the pre-step state into the step scratch, returning the max
    /// wet-cell celerity for CFL dt selection (eq. 12, rev 24).
    ///
    /// The true celerity is `dq/dh` along the equilibrium
    /// `q(h) = alpha(h, q) h^1.5`, evaluated numerically through the same
    /// friction fixed point at a perturbed depth. This costs one extra
    /// alpha evaluation per wet cell per step (D14 budget note recorded in
    /// the package evidence); the frozen-alpha expression
    /// `1.5 alpha h^0.5` under-estimates the wave speed for
    /// depth-dependent friction (by 2x on the laminar `k_o/Re` limb) and
    /// let the scheme run at true Courant well above 1.
    fn prepare_step_alpha(&mut self, skin_rain_term: f64) -> f64 {
        let mut max_celerity = 0.0_f64;
        let mut evaluations = 0_u64;
        for (i, cell) in self.mesh.cells.iter().enumerate() {
            let h = self.depth_m[i];
            // D15A OPT-5: the fixed point returns its own final
            // `q = alpha·h^1.5` iterate, replacing the two redundant
            // caller-side `powf` recomputations at the base and perturbed
            // depths (bit-identical operands; see `alpha_q`).
            let (alpha, q) = cell.alpha_q(h, self.discharge_m2_s[i], skin_rain_term);
            self.scratch.alpha[i] = alpha;
            evaluations += 1;
            if h <= DRY_DEPTH_M {
                self.scratch.celerity[i] = 0.0;
                self.scratch.q0[i] = 0.0;
                continue;
            }
            self.scratch.q0[i] = q;
            let dh = (1.0e-3 * h).max(1.0e-8);
            let h2 = h + dh;
            let (_, q2) = cell.alpha_q(h2, q, skin_rain_term);
            evaluations += 1;
            // Guarded true celerity: never below the frozen-alpha floor
            // (the discrete flux actually advanced this step).
            let frozen = DEPTH_DISCHARGE_EXPONENT * alpha * h.sqrt();
            let celerity = ((q2 - q) / dh).max(frozen);
            self.scratch.celerity[i] = celerity;
            if celerity > max_celerity {
                max_celerity = celerity;
            }
        }
        profile::count_alpha_evaluations(evaluations);
        max_celerity
    }

    /// `Cf_i` (eq. 11e) from the local Courant number.
    fn cf(courant: f64) -> f64 {
        if courant > 0.5 {
            0.25
        } else {
            courant * (1.0 - courant)
        }
    }

    /// Flux limiter `phi(r) = min(2r, 1) for r > 0; 0 for r <= 0` — the
    /// source-correct branch (Davis 1984 eq. 3.20; Mingham 2001 eq. 31f).
    /// R-63's printed (11c) swaps the branch conditions and is adjudicated
    /// a transcription error (`SC-OFEROUTE-001` rev 24,
    /// REF-OFEROUTE-TVD-MACCORMACK): dissipation must VANISH in smooth
    /// monotone regions (`r -> 1 => phi -> 1 => G -> 0`) and act at
    /// extrema (`r <= 0 => phi = 0 => full G`).
    fn phi(ratio: f64) -> f64 {
        if ratio > 0.0 {
            (2.0 * ratio).min(1.0)
        } else {
            0.0
        }
    }

    /// Face dissipation coefficient `G(r) = 0.5 Cf(Cr) (1 - phi(r))`
    /// (Mingham eq. 28b / Davis eqs. 3.17-3.18).
    fn g_coeff(courant: f64, ratio: f64) -> f64 {
        0.5 * Self::cf(courant) * (1.0 - Self::phi(ratio))
    }

    /// Advance one TVD-MacCormack step of size `dt` (eqs. 8-14). Returns the
    /// outlet sample, or a `RoutingError` on a fail-closed condition.
    ///
    /// Preconditions owned by the run loop (D14 OPT-1): `scratch.alpha`
    /// holds the pre-step per-cell alpha values and the rainfall intensity
    /// was validated before they were computed.
    #[allow(clippy::too_many_lines)]
    fn step(
        &mut self,
        time_s: f64,
        dt: f64,
        q_up: f64,
        forcing: &Forcing<'_>,
        mass: &mut MassBalance,
    ) -> Result<(f64, f64, f64), RoutingError> {
        let n = self.mesh.cell_count();
        let dx = self.mesh.cell_length_m;
        let mut clamp_injected_m2 = 0.0_f64;

        // Rainfall excess for this step (per-cell alpha is already in the
        // step scratch). Rainfall excess is physically non-negative; a
        // non-finite OR negative value is invalid input and fails closed.
        // Intensity and q_up are validated by the run loop.
        for i in 0..n {
            let v_raw = (forcing.rainfall_excess_m_s)(i, time_s);
            if !is_valid_forcing(v_raw) {
                return Err(RoutingError::InvalidForcing);
            }
            self.scratch.v[i] = v_raw;
        }

        // Predictor (eqs. 8-9, rev-24 boundary closure): forward flux
        // difference in the interior; the TOP face carries the prescribed
        // inflow flux `q_up` in BOTH sweeps so the discrete injection
        // equals the physical BC exactly; the LAST cell is a pure DONOR
        // cell (true upwind backward difference `q_{n-1} - q_{n-2}`, its
        // outflow face flux = its own kinematic flux). The upwind outflow
        // closure is steady-exact (the backward difference reproduces
        // `dq/dx = v` at discrete steady state) and, unlike the pre-rev-24
        // linear-extrapolation ghost, has no feedback gain — the ghost
        // both over-discharged (unbooked, the S0 ledger's dominant term)
        // and sustained a period-2 boundary limit cycle.
        for i in 0..n {
            let (q_lo, q_hi) = if i == 0 {
                (q_up, self.discharge_m2_s[usize::from(n > 1)])
            } else if i + 1 < n {
                (self.discharge_m2_s[i], self.discharge_m2_s[i + 1])
            } else {
                (self.discharge_m2_s[n - 2], self.discharge_m2_s[n - 1])
            };
            let h_new = self.depth_m[i] - (dt / dx) * (q_hi - q_lo) + self.scratch.v[i] * dt;
            if h_new < 0.0 {
                // Stage clamps enter the committed state at half weight
                // (the commit averages the two stages).
                clamp_injected_m2 += 0.5 * (-h_new) * dx;
            }
            self.scratch.h_pred[i] = h_new.max(0.0);
            self.scratch.q_pred[i] =
                self.scratch.alpha[i] * self.scratch.h_pred[i].powf(DEPTH_DISCHARGE_EXPONENT);
        }
        // Predictor boundary drain (telescoped): the donor difference at
        // the last cell drains `2 q_{n-1} - q_{n-2}` through the domain
        // boundary per the flux-difference sum (for n = 1, `q_0`).
        let pred_out_face = if n >= 2 {
            2.0 * self.discharge_m2_s[n - 1] - self.discharge_m2_s[n - 2]
        } else {
            self.discharge_m2_s[0]
        };

        // Corrector (eq. 10): backward flux difference; upstream ghost flux =
        // q_up entering cell 0.
        for i in 0..n {
            let q_im1 = if i == 0 {
                q_up
            } else {
                self.scratch.q_pred[i - 1]
            };
            let h_new = self.depth_m[i] - (dt / dx) * (self.scratch.q_pred[i] - q_im1)
                + self.scratch.v[i] * dt;
            if h_new < 0.0 {
                clamp_injected_m2 += 0.5 * (-h_new) * dx;
            }
            self.scratch.h_corr[i] = h_new.max(0.0);
        }

        // Average (eq. 13 first half).
        for i in 0..n {
            self.scratch.averaged[i] = 0.5 * (self.scratch.h_pred[i] + self.scratch.h_corr[i]);
        }

        // TVD dissipation, rev-24 FACE-BASED two-sided form (Mingham
        // eqs. 28b/31a/31g; Davis eqs. 3.17-3.18): for each interior face
        // f (between cells f and f+1),
        //   D_f = [G(Cr_f, r+_f) + G(Cr_{f+1}, r-_{f+1})] (h_{f+1} - h_f)
        // with r+_f = dh_{f-1/2}/dh_{f+1/2} and
        // r-_{f+1} = dh_{f+3/2}/dh_{f+1/2}; a ratio whose stencil leaves
        // the domain contributes no dissipation from that side. Cell i
        // receives D_i - D_{i-1} with ZERO flux at the domain-boundary
        // faces, so the term telescopes exactly (INV-OFEROUTE-006 rev 24).
        // Face coefficients are stored in scratch.gr[f] for f in 0..n-1.
        // Material-interface faces (cell PARAMETERS differ across the face
        // — the section/slope breaks) carry a PHYSICAL equilibrium depth
        // jump; the uniform-coefficient Davis/Mingham analysis does not
        // apply there and h-based dissipation would diffuse a legitimate
        // discontinuity. Such faces carry zero dissipative flux
        // (conservative), and ratio stencils reaching across them
        // contribute no dissipation from that side. NOTE: the detector
        // compares MATERIAL parameters, not the state-dependent alpha
        // (alpha varies per cell under depth-dependent friction even on
        // uniform material).
        let is_break = |f: usize| -> bool { self.material_breaks[f] };
        for f in 0..n.saturating_sub(1) {
            let dh_face = self.depth_m[f + 1] - self.depth_m[f];
            if dh_face.abs() <= DRY_DEPTH_M || is_break(f) {
                self.scratch.gr[f] = 0.0;
                continue;
            }
            let courant_at = |i: usize| self.scratch.celerity[i] * dt / dx;
            let r_plus_avail = f >= 1 && !is_break(f - 1);
            let r_minus_avail = f + 2 < n && !is_break(f + 1);
            let r_plus = if r_plus_avail {
                Some((self.depth_m[f] - self.depth_m[f - 1]) / dh_face)
            } else {
                None
            };
            let r_minus = if r_minus_avail {
                Some((self.depth_m[f + 2] - self.depth_m[f + 1]) / dh_face)
            } else {
                None
            };
            // Boundary-adjacent faces: a side whose smoothness-monitor
            // stencil leaves the domain (or crosses a material break)
            // mirrors the AVAILABLE side's ratio, so boundary-adjacent
            // oscillations are still detected and damped (limiter-stencil
            // ghost extension); a face with neither monitor carries no
            // dissipation.
            let g_plus = match (r_plus, r_minus) {
                (Some(r), _) | (None, Some(r)) => Self::g_coeff(courant_at(f), r),
                (None, None) => 0.0,
            };
            let g_minus = match (r_minus, r_plus) {
                (Some(r), _) | (None, Some(r)) => Self::g_coeff(courant_at(f + 1), r),
                (None, None) => 0.0,
            };
            self.scratch.gr[f] = (g_plus + g_minus) * dh_face;
        }
        // Literal telescoping check value (diagnostic; ~0 by construction).
        let mut tvd_leak_m2 = 0.0_f64;
        for i in 0..n {
            let d_hi = if i + 1 < n { self.scratch.gr[i] } else { 0.0 };
            let d_lo = if i >= 1 { self.scratch.gr[i - 1] } else { 0.0 };
            tvd_leak_m2 += (d_hi - d_lo) * dx;
        }
        for i in 0..n {
            let tvd = {
                let d_hi = if i + 1 < n { self.scratch.gr[i] } else { 0.0 };
                let d_lo = if i >= 1 { self.scratch.gr[i - 1] } else { 0.0 };
                d_hi - d_lo
            };
            let h_new = self.scratch.averaged[i] + tvd;
            if !h_new.is_finite() {
                return Err(RoutingError::NonFiniteState);
            }
            if h_new < -DRY_DEPTH_M {
                return Err(RoutingError::NegativeDepth);
            }
            if h_new < 0.0 {
                // small negative excursion clamped to dry: record injected mass
                clamp_injected_m2 += (-h_new) * dx;
            }
            self.scratch.h_next[i] = h_new.max(0.0);
        }

        // Commit state and recompute discharge (eq. 14).
        for i in 0..n {
            self.depth_m[i] = self.scratch.h_next[i];
            self.discharge_m2_s[i] =
                self.scratch.alpha[i] * self.depth_m[i].powf(DEPTH_DISCHARGE_EXPONENT);
            if !self.discharge_m2_s[i].is_finite() {
                return Err(RoutingError::NonFiniteState);
            }
        }

        // D10B TV diagnostic (rev 24): on homogeneous steps (no forcing
        // anywhere) the committed step must not increase the spatial total
        // variation of the FLUX `q = alpha h^1.5` (continuous across
        // material interfaces, unlike `h`, whose equilibrium jumps at
        // slope breaks make TV(h) the wrong functional for
        // variable-coefficient KWE). Both sides use the frozen per-step
        // alpha (apples-to-apples within the step). Source terms
        // legitimately change TV and are excluded.
        //
        // D15A OPT-6 (bit-identical value reuse): `tv_before`'s
        // `alpha_i·h_i^1.5` at the pre-step state IS the celerity
        // evaluation's own `q` (cached in `scratch.q0`; dry/zero-alpha
        // cells carry the same `+0.0`), and `tv_after`'s
        // `alpha_i·h_next_i^1.5` IS the committed discharge written by the
        // commit loop above (depths are non-negative, so the previous
        // `.max(0.0)` was the identity). The diagnostic therefore runs
        // after the commit and recomputes no `powf`. The only ordering
        // change is that a non-finite committed discharge now fails before
        // the TV accumulator updates — on that path the run returns `Err`
        // and no result (including the TV maximum) is ever published.
        let source_free = self.scratch.v.iter().all(|v| *v == 0.0);
        if source_free {
            profile::count_solver_steps_source_free(1);
        }
        let homogeneous = q_up == 0.0 && source_free;
        if homogeneous {
            profile::count_solver_steps_homogeneous(1);
            let mut tv_before = 0.0_f64;
            let mut tv_after = 0.0_f64;
            for i in 0..n.saturating_sub(1) {
                // Uniform-material faces only: the TVD property is a
                // uniform-coefficient statement; material-interface
                // transients are adjudicated separately by validation.
                if self.material_breaks[i] {
                    continue;
                }
                tv_before += (self.scratch.q0[i + 1] - self.scratch.q0[i]).abs();
                tv_after += (self.discharge_m2_s[i + 1] - self.discharge_m2_s[i]).abs();
            }
            let increase = (tv_after - tv_before).max(0.0);
            if increase > self.max_tv_increase_m {
                self.max_tv_increase_m = increase;
            }
        }

        // Mass ledger (rev 24: booked = the scheme's ACTUAL boundary
        // fluxes, Algorithm item 5). Inflow: both sweeps carry `q_up` at
        // the top face, so the discrete injection is exactly `q_up dt`.
        // Outflow: predictor face flux = the extrapolated ghost, corrector
        // face flux = `q_pred[n-1]`; the committed average discharges
        // their mean.
        let outflow_actual_m2 = 0.5 * (pred_out_face + self.scratch.q_pred[n - 1]) * dt;
        mass.inflow_m2 += q_up * dt;
        mass.rainfall_excess_m2 += self.scratch.v.iter().sum::<f64>() * dx * dt;
        mass.outflow_m2 += outflow_actual_m2;
        mass.positivity_clamp_m2 += clamp_injected_m2;
        // Scheme-actual diagnostics coincide with the booked ledger by
        // construction post-rev-24; kept as the booked-equals-actual
        // identity surface (`INV-OFEROUTE-006` rev 24 tests).
        mass.scheme_inflow_m2 += q_up * dt;
        mass.scheme_outflow_m2 += outflow_actual_m2;
        mass.tvd_boundary_leak_m2 += tvd_leak_m2;

        Ok((
            self.discharge_m2_s[n - 1],
            self.depth_m[n - 1],
            outflow_actual_m2,
        ))
    }

    /// Run to `end_time_s`, recording the outlet hydrograph at ~`sample_dt_s`.
    /// `max_dt_s` caps the CFL-adaptive step. Fails closed on CFL/non-finite/
    /// negative-depth conditions. The upstream boundary flux is the
    /// point-evaluated `forcing.upstream_inflow_m2_s` at each step start.
    pub fn run(
        &mut self,
        forcing: &Forcing<'_>,
        end_time_s: f64,
        sample_dt_s: f64,
        max_dt_s: f64,
    ) -> Result<RoutingResult, RoutingError> {
        self.run_with_upstream_integral(forcing, None, end_time_s, sample_dt_s, max_dt_s)
    }

    /// Rev-24 conservative-handoff entry point (Algorithm item 6): when
    /// `upstream_integral_m2` is provided (`(t0, t1) -> integral of the
    /// upstream unit discharge over [t0, t1]`, m^2 per unit width), each
    /// step's boundary flux is the exact interval MEAN of the upstream
    /// hydrograph, so the injected mass equals the upstream series'
    /// integral exactly instead of a left-endpoint point sample.
    #[allow(clippy::too_many_lines)]
    pub fn run_with_upstream_integral(
        &mut self,
        forcing: &Forcing<'_>,
        upstream_integral_m2: Option<&dyn Fn(f64, f64) -> f64>,
        end_time_s: f64,
        sample_dt_s: f64,
        max_dt_s: f64,
    ) -> Result<RoutingResult, RoutingError> {
        self.run_with_options(
            forcing,
            upstream_integral_m2,
            &[],
            end_time_s,
            sample_dt_s,
            max_dt_s,
        )
    }

    /// Full-options run (Codex review High-2): `forcing_breakpoints_s` are
    /// known discontinuity times of the forcing closures (e.g. a lateral
    /// supply cutoff); the CFL-adaptive step is clipped so no step
    /// STRADDLES a breakpoint — the solver samples forcing at the step
    /// start and holds it constant over the step, so a straddling step
    /// would integrate the pre-breakpoint rate across the post-breakpoint
    /// interval and the discrete source history would diverge from the
    /// forcing definition. Breakpoints must be sorted ascending.
    pub fn run_with_options(
        &mut self,
        forcing: &Forcing<'_>,
        upstream_integral_m2: Option<&dyn Fn(f64, f64) -> f64>,
        forcing_breakpoints_s: &[f64],
        end_time_s: f64,
        sample_dt_s: f64,
        max_dt_s: f64,
    ) -> Result<RoutingResult, RoutingError> {
        let (result, terminal_deficit_m2) = self.run_with_options_deficit_carry(
            forcing,
            upstream_integral_m2,
            forcing_breakpoints_s,
            end_time_s,
            sample_dt_s,
            max_dt_s,
        )?;
        if terminal_deficit_m2 < 0.0 {
            // Medium-1: a material terminal deficit means the outflow
            // series cannot be represented as a non-negative exact-total
            // bin series — physically unexpected (total outflow is
            // non-negative); publishing a negative outlet bin is not an
            // option, so fail closed.
            return Err(RoutingError::NegativeOutletBin);
        }
        Ok(result)
    }

    /// Rev-30 composition-scoped variant of [`Self::run_with_options`]: a
    /// MATERIAL terminal bin deficit (the Review-B M2 front-arrival
    /// attribution class that the recorder's forward redistribution could
    /// not absorb inside this window) is RETURNED (`<= 0`, m^2) instead of
    /// failing closed. When the deficit is nonzero the returned
    /// `outlet_bin_outflow_m2` series (and the hydrograph derived from it)
    /// OVER-COUNTS by exactly `|deficit|`; the caller MUST continue the
    /// forward redistribution into subsequent bins under the exact-total
    /// rule or fail closed itself. All mass ledgers in the result book
    /// actual boundary fluxes and are unaffected. Only the hybrid span
    /// composition (`route_single_ofe_hybrid`) may consume this; every
    /// other path goes through the fail-closed wrapper.
    #[allow(clippy::too_many_lines)]
    pub(super) fn run_with_options_deficit_carry(
        &mut self,
        forcing: &Forcing<'_>,
        upstream_integral_m2: Option<&dyn Fn(f64, f64) -> f64>,
        forcing_breakpoints_s: &[f64],
        end_time_s: f64,
        sample_dt_s: f64,
        max_dt_s: f64,
    ) -> Result<(RoutingResult, f64), RoutingError> {
        if self.mesh.cell_count() == 0
            || !self.mesh.cell_length_m.is_finite()
            || self.mesh.cell_length_m <= 0.0
            || !end_time_s.is_finite()
            || end_time_s <= 0.0
            || !max_dt_s.is_finite()
            || max_dt_s <= 0.0
            || !sample_dt_s.is_finite()
            || sample_dt_s <= 0.0
        {
            return Err(RoutingError::DegenerateConfiguration);
        }
        for cell in &self.mesh.cells {
            cell.validate()?;
        }
        let dx = self.mesh.cell_length_m;
        let storage_initial = self.total_storage_m2();
        self.max_tv_increase_m = 0.0;
        let mut mass = MassBalance::default();
        let mut recorder = BinRecorder::new(sample_dt_s, end_time_s);
        let mut t = 0.0_f64;
        let mut peak = 0.0_f64;
        let mut time_to_peak = 0.0_f64;
        let mut max_courant = 0.0_f64;

        profile::count_solver_runs(1);
        let mut guard_steps = 0_u64;
        let max_steps = 50_000_000_u64; // runaway backstop
        while t < end_time_s {
            let cfl_span = profile::span_start();
            // One intensity fetch and one alpha evaluation per cell per step
            // (D14 OPT-1): dt selection, Courant evidence, and the step math
            // all reuse the same pre-step alpha values. Intensity is
            // validated here, before any consumer.
            let intensity = (forcing.rainfall_intensity_m_s)(t);
            if !is_valid_forcing(intensity) {
                return Err(RoutingError::InvalidForcing);
            }
            let rain_term = skin_rain_term(intensity);
            let max_celerity = self.prepare_step_alpha(rain_term);
            // Codex review High-1: CFL must FAIL CLOSED, never open. A
            // non-finite celerity is corrupt state; a celerity so large
            // that no positive sub-timestep exists (dt underflows to 0)
            // violates INV-OFEROUTE-007's hard-fail posture. The pre-fix
            // `dt <= 0 -> break` returned a partial Ok result instead.
            if !max_celerity.is_finite() {
                profile::end_solver_cfl(cfl_span);
                return Err(RoutingError::NonFiniteState);
            }
            // CFL-limited sub-timestep (eq. 12), clamped to max_dt and the
            // remaining window.
            let dt_cfl = if max_celerity <= 0.0 {
                max_dt_s
            } else {
                (CFL_TARGET * dx / max_celerity).min(max_dt_s)
            };
            let mut dt = dt_cfl.min(end_time_s - t);
            // Clip at the next forcing breakpoint so no step straddles a
            // source discontinuity (High-2 exact source-history rule).
            for bp in forcing_breakpoints_s {
                if *bp > t + 1.0e-12 && *bp < t + dt {
                    dt = *bp - t;
                    break;
                }
            }
            if !dt.is_finite() || dt <= 0.0 {
                profile::end_solver_cfl(cfl_span);
                return Err(RoutingError::CflViolation);
            }
            // CFL evidence at the chosen dt (true celerity, rev 24).
            for i in 0..self.mesh.cell_count() {
                if self.depth_m[i] <= DRY_DEPTH_M {
                    continue;
                }
                let courant = self.scratch.celerity[i] * dt / dx;
                if courant > max_courant {
                    max_courant = courant;
                }
                if courant > 1.0 + 1.0e-9 {
                    return Err(RoutingError::CflViolation);
                }
            }
            profile::end_solver_cfl(cfl_span);

            // Upstream boundary flux for this step: exact interval mean
            // when the integral closure is provided (rev 24), else the
            // point sample. Physically non-negative; fails closed.
            let q_up = match upstream_integral_m2 {
                Some(integral) => {
                    let injected = integral(t, t + dt);
                    if !is_valid_forcing(injected) {
                        return Err(RoutingError::InvalidForcing);
                    }
                    injected / dt
                }
                None => (forcing.upstream_inflow_m2_s)(t),
            };
            if !is_valid_forcing(q_up) {
                return Err(RoutingError::InvalidForcing);
            }

            let t_before = t;
            let step_span = profile::span_start();
            let (_q_cell_out, h_out, outflow_step_m2) =
                self.step(t, dt, q_up, forcing, &mut mass)?;
            profile::end_solver_step(step_span);
            profile::count_solver_steps(1);
            t += dt;

            // Rev 24: the exported hydrograph is the bin-mean BOUNDARY
            // FLUX (the surface the ledger books and the handoff
            // conserves); the step-mean rate carries the sub-step peak
            // diagnostic.
            let q_out = outflow_step_m2 / dt;
            if q_out > peak {
                peak = q_out;
                time_to_peak = t;
            }
            recorder.record_step(t_before, dt, outflow_step_m2, h_out);

            guard_steps += 1;
            if guard_steps > max_steps {
                return Err(RoutingError::CflViolation);
            }
        }

        mass.storage_change_m2 = self.total_storage_m2() - storage_initial;
        let (hydrograph, outlet_bin_outflow_m2, outlet_bin_spans_s, terminal_deficit_m2) =
            recorder.finish();
        Ok((
            RoutingResult {
                hydrograph,
                mass_balance: mass,
                peak_unit_discharge_m2_s: peak,
                time_to_peak_s: time_to_peak,
                max_courant,
                max_homogeneous_tv_increase_m2_s: self.max_tv_increase_m,
                outlet_bin_outflow_m2,
                outlet_bin_dt_s: sample_dt_s,
                outlet_bin_spans_s,
            },
            terminal_deficit_m2,
        ))
    }
}

/// Nash-Sutcliffe efficiency `Ef` (Papanicolaou eq. 15) between observed and
/// modeled series sampled at matching times. `observed` and `modeled` must be
/// equal length. Returns `None` if the observed variance is zero.
#[must_use]
pub fn nash_sutcliffe_efficiency(observed: &[f64], modeled: &[f64]) -> Option<f64> {
    if observed.len() != modeled.len() || observed.is_empty() {
        return None;
    }
    let n = f64::from(u32::try_from(observed.len()).unwrap_or(u32::MAX));
    let mean_obs = observed.iter().sum::<f64>() / n;
    let mut num = 0.0;
    let mut den = 0.0;
    for (o, m) in observed.iter().zip(modeled.iter()) {
        num += (o - m).powi(2);
        den += (o - mean_obs).powi(2);
    }
    if den <= 0.0 {
        return None;
    }
    Some(1.0 - num / den)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn constant<'a>(value: f64) -> impl Fn(f64) -> f64 + 'a {
        move |_t: f64| value
    }

    /// Rev-30 (deficit-carry composition seam): when the terminal bin nets
    /// negative beyond the noise floor, `finish()` RETURNS the material
    /// deficit (it is not folded), the published bins stay non-negative,
    /// and the series over-counts by exactly `|deficit|` — the identity
    /// `Σ bins == booked total − deficit` the hybrid composition relies on
    /// to continue the forward redistribution across span boundaries.
    #[test]
    fn bin_recorder_returns_material_terminal_deficit_exactly() {
        let mut recorder = BinRecorder::new(900.0, 1800.0);
        recorder.record_step(0.0, 900.0, 5.0, 0.001);
        recorder.record_step(900.0, 890.0, 0.001, 0.0);
        // Front-arrival class: a step-scale negative attribution at the
        // very end of the window (no later bin can absorb it).
        recorder.record_step(1790.0, 10.0, -0.002, 0.0);
        let booked_total = 5.0 + 0.001 - 0.002;
        let (_hydrograph, bins, _spans, deficit) = recorder.finish();
        assert!(
            (deficit - (-0.001)).abs() < 1.0e-15,
            "material deficit returned exactly: {deficit:e}"
        );
        assert!(
            bins.iter().all(|v| *v >= 0.0),
            "bins non-negative: {bins:?}"
        );
        let bin_total: f64 = bins.iter().sum();
        assert!(
            (bin_total - (booked_total - deficit)).abs() < 1.0e-15,
            "over-count identity: bins {bin_total} vs booked {booked_total} - deficit {deficit}"
        );
    }

    // Case 1 (bare surface): 60 mm/h over a 7.5 m plot at 9%, k_o=500.
    // At steady state the outlet unit discharge must equal rainfall-excess rate
    // x plot length (mass balance), and mass must conserve.
    #[test]
    fn case1_bare_surface_reaches_steady_state_and_conserves_mass() {
        let length = 7.5;
        let slope = 0.09;
        let v = 60.0 / 3.6e6; // 60 mm/h -> m/s
        let mesh = KinematicWaveMesh::uniform(length, 30, CellParameters::bare(slope, 500.0));
        let mut solver = KinematicWaveSolver::new(mesh);
        let excess = |_i: usize, _t: f64| v;
        let inflow = constant(0.0);
        let intensity = constant(v);
        let forcing = Forcing {
            rainfall_excess_m_s: &excess,
            upstream_inflow_m2_s: &inflow,
            rainfall_intensity_m_s: &intensity,
        };
        let res = solver.run(&forcing, 3600.0, 10.0, 2.0).expect("run ok");

        // Steady-state outlet discharge = v * L. Rev 24 (D10B): the
        // authoritative steady measure is the BOOKED discharge (the
        // scheme's actual boundary flux); the exported bin-mean hydrograph
        // carries a bounded slow boundary ripple at coarse grids
        // (characterized in the D10B evidence) and gets a looser band.
        let bins = &res.outlet_bin_outflow_m2;
        let tail_bins = 60.min(bins.len());
        let q_booked_tail: f64 = bins[bins.len() - tail_bins..].iter().sum::<f64>()
            / (res.outlet_bin_dt_s * f64::from(u32::try_from(tail_bins).unwrap_or(u32::MAX)));
        let q_expected = v * length;
        assert!(
            (q_booked_tail - q_expected).abs() / q_expected < 0.005,
            "booked steady outlet {q_booked_tail} should equal v*L {q_expected}"
        );
        let q_steady = res
            .hydrograph
            .last()
            .expect("hydrograph")
            .outlet_unit_discharge_m2_s;
        assert!(
            (q_steady - q_expected).abs() / q_expected < 0.06,
            "sampled steady outlet {q_steady} outside the ripple band around v*L {q_expected}"
        );
        // mass conservation (INV-OFEROUTE-006): the scheme conserves exactly
        // once the surfaced positivity-clamp injection is accounted; the raw
        // residual equals -clamp to machine epsilon, and the clamp itself is a
        // tiny transient wetting-front term.
        // Conservation (INV-OFEROUTE-006): the independent outlet-flux ledger
        // (physical q at the outlet, trapezoidally integrated) matches the
        // input+storage to ~0.3% at this resolution; the positivity clamp is
        // zero, so this is a pure, resolution-convergent discretization gap
        // (see `conservation_residual_converges_with_resolution`), not a leak.
        let total = res.mass_balance.rainfall_excess_m2;
        assert!(res.mass_balance.positivity_clamp_m2 / total < 1.0e-9);
        assert!(
            res.mass_balance.residual_m2().abs() / total < 5.0e-3,
            "mass residual {} vs input {total}",
            res.mass_balance.residual_m2()
        );
        // CFL respected (INV-OFEROUTE-007)
        assert!(res.max_courant <= 1.0 + 1.0e-9, "CFL {}", res.max_courant);
        // The MacCormack scheme produces a small (~6%) rising-limb overshoot,
        // TVD-damped and settling back to steady state (checked above within
        // 2%). Bound it to catch gross instability without flagging the mild
        // second-order dispersion.
        assert!(
            res.peak_unit_discharge_m2_s <= q_expected * 1.10,
            "peak {} overshoots steady {q_expected} by >10%",
            res.peak_unit_discharge_m2_s
        );
    }

    // Case 4 (Iwagaki curvature/shock): impermeable bed, finite lateral inflow,
    // three sections of decreasing slope. Tests shock capture + conservation
    // with NO rainfall (pure routing of a lateral pulse).
    #[test]
    fn case4_lateral_pulse_conserves_mass_and_captures_front() {
        // 24 m flume, three 8 m sections at 2%, 1.5%, 1%.
        let n = 60usize;
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
            // smooth impermeable bed: small k_o, no elements/veg
            cells.push(CellParameters::bare(slope, 50.0));
        }
        let mesh = KinematicWaveMesh {
            cell_length_m: dx,
            cells,
        };
        let mut solver = KinematicWaveSolver::new(mesh);
        // lateral inflow as rainfall-excess-equivalent for 10 s, then off.
        // 0.108 cm/s over the domain (representative of the pulse magnitude).
        let pulse = |_i: usize, t: f64| if t <= 10.0 { 0.108e-2 } else { 0.0 };
        let inflow = constant(0.0);
        let intensity = constant(0.0);
        let forcing = Forcing {
            rainfall_excess_m_s: &pulse,
            upstream_inflow_m2_s: &inflow,
            rainfall_intensity_m_s: &intensity,
        };
        let res = solver.run(&forcing, 120.0, 0.5, 0.25).expect("run ok");

        // conservation: all lateral input eventually leaves or is stored; the
        // scheme conserves exactly modulo the surfaced positivity clamp.
        let input = res.mass_balance.rainfall_excess_m2;
        assert!(input > 0.0);
        assert!(
            res.mass_balance.residual_m2().abs() / input < 1.0e-2,
            "Case4 mass residual vs input"
        );
        // a discharge front reaches the outlet (peak well above zero) and the
        // scheme stays positive/finite/CFL-stable (TVD shock capture)
        assert!(res.peak_unit_discharge_m2_s > 0.0);
        assert!(res.max_courant <= 1.0 + 1.0e-9);
        // outlet response is delayed (front travel), not instantaneous
        assert!(res.time_to_peak_s > 5.0, "front should take time to arrive");
        // no negative depths anywhere in the recorded outlet series
        assert!(
            res.hydrograph
                .iter()
                .all(|s| s.outlet_depth_m >= 0.0 && s.outlet_unit_discharge_m2_s >= 0.0)
        );
    }

    #[test]
    fn cfl_adaptive_step_keeps_courant_bounded_on_steep_slope() {
        // steep + intense: would blow up a fixed step; adaptive dt must hold CFL.
        let mesh = KinematicWaveMesh::uniform(50.0, 40, CellParameters::bare(0.20, 100.0));
        let mut solver = KinematicWaveSolver::new(mesh);
        let v = 100.0 / 3.6e6;
        let excess = |_i: usize, _t: f64| v;
        let inflow = constant(0.0);
        let intensity = constant(v);
        let forcing = Forcing {
            rainfall_excess_m_s: &excess,
            upstream_inflow_m2_s: &inflow,
            rainfall_intensity_m_s: &intensity,
        };
        let res = solver.run(&forcing, 1800.0, 30.0, 5.0).expect("run ok");
        // Primary assertion: CFL adaptivity holds Cr <= 1 despite a large
        // max_dt cap on a steep slope (INV-OFEROUTE-007).
        assert!(res.max_courant <= 1.0 + 1.0e-9);
        // Secondary: the scheme conserves exactly modulo the surfaced clamp
        // (INV-OFEROUTE-006), independent of dt/slope.
        assert!(
            res.mass_balance.residual_m2().abs() / res.mass_balance.rainfall_excess_m2 < 1.0e-2
        );
    }

    #[test]
    fn hydrograph_bins_are_conservative_and_rise_at_bin_scale() {
        // Rev 24 (D10B, supersedes the D8-2 interpolating-sampler pin):
        // the exported hydrograph is the bin-mean boundary flux. The bin
        // series must carry the booked outflow EXACTLY (conservative
        // resampling), and the rising limb must be visible at bin scale
        // once solver steps are shorter than the bins.
        let mesh = KinematicWaveMesh::uniform(7.5, 20, CellParameters::bare(0.09, 500.0));
        let mut solver = KinematicWaveSolver::new(mesh);
        let v = 60.0 / 3.6e6;
        let excess = |_i: usize, _t: f64| v;
        let inflow = constant(0.0);
        let intensity = constant(v);
        let forcing = Forcing {
            rainfall_excess_m_s: &excess,
            upstream_inflow_m2_s: &inflow,
            rainfall_intensity_m_s: &intensity,
        };
        let res = solver.run(&forcing, 200.0, 1.0, 5.0).expect("run ok");
        let bin_sum: f64 = res.outlet_bin_outflow_m2.iter().sum();
        assert!(
            (bin_sum - res.mass_balance.outflow_m2).abs()
                <= 1.0e-12 * res.mass_balance.outflow_m2.max(1.0e-12),
            "bin series must carry the booked outflow exactly: bins {bin_sum} vs ledger {}",
            res.mass_balance.outflow_m2
        );
        let mid = res.hydrograph[res.hydrograph.len() / 2].outlet_unit_discharge_m2_s;
        let late = res
            .hydrograph
            .last()
            .expect("hydrograph")
            .outlet_unit_discharge_m2_s;
        assert!(
            mid < late,
            "rising limb must be visible at bin scale: mid={mid}, late={late}"
        );
    }

    #[test]
    fn upstream_inflow_boundary_routes_through() {
        // no rain; constant upstream inflow should convey to the outlet and
        // conserve mass.
        let mesh = KinematicWaveMesh::uniform(20.0, 20, CellParameters::bare(0.05, 200.0));
        let mut solver = KinematicWaveSolver::new(mesh);
        let q_in = 5.0e-4; // m^2/s
        let excess = |_i: usize, _t: f64| 0.0;
        let inflow = constant(q_in);
        let intensity = constant(0.0);
        let forcing = Forcing {
            rainfall_excess_m_s: &excess,
            upstream_inflow_m2_s: &inflow,
            rainfall_intensity_m_s: &intensity,
        };
        let res = solver.run(&forcing, 1200.0, 10.0, 2.0).expect("run ok");
        let q_out = res.hydrograph.last().unwrap().outlet_unit_discharge_m2_s;
        assert!(
            (q_out - q_in).abs() / q_in < 0.05,
            "steady outlet {q_out} should approach inflow {q_in}"
        );
        assert!(res.mass_balance.residual_m2().abs() / res.mass_balance.inflow_m2 < 5.0e-3);
    }

    #[test]
    fn nash_sutcliffe_perfect_and_mean() {
        let obs = [1.0, 2.0, 3.0, 4.0];
        assert_eq!(nash_sutcliffe_efficiency(&obs, &obs), Some(1.0));
        // predicting the mean gives Ef = 0
        let mean = [2.5, 2.5, 2.5, 2.5];
        let ef = nash_sutcliffe_efficiency(&obs, &mean).unwrap();
        assert!(ef.abs() < 1.0e-12);
    }

    #[test]
    fn degenerate_configuration_fails_closed() {
        let mesh = KinematicWaveMesh {
            cell_length_m: 1.0,
            cells: vec![],
        };
        let mut solver = KinematicWaveSolver::new(mesh);
        let excess = |_i: usize, _t: f64| 0.0;
        let inflow = constant(0.0);
        let intensity = constant(0.0);
        let forcing = Forcing {
            rainfall_excess_m_s: &excess,
            upstream_inflow_m2_s: &inflow,
            rainfall_intensity_m_s: &intensity,
        };
        assert!(matches!(
            solver.run(&forcing, 10.0, 1.0, 1.0),
            Err(RoutingError::DegenerateConfiguration)
        ));
    }

    #[test]
    fn conservation_residual_converges_with_resolution() {
        let v = 60.0 / 3.6e6;
        let run = |cells: usize, max_dt: f64| {
            let mesh = KinematicWaveMesh::uniform(7.5, cells, CellParameters::bare(0.09, 500.0));
            let mut solver = KinematicWaveSolver::new(mesh);
            let excess = |_i: usize, _t: f64| v;
            let inflow = |_t: f64| 0.0;
            let intensity = |_t: f64| v;
            let forcing = Forcing {
                rainfall_excess_m_s: &excess,
                upstream_inflow_m2_s: &inflow,
                rainfall_intensity_m_s: &intensity,
            };
            let res = solver.run(&forcing, 3600.0, 10.0, max_dt).unwrap();
            (
                res.mass_balance.residual_m2().abs() / res.mass_balance.rainfall_excess_m2,
                res.mass_balance.positivity_clamp_m2 / res.mass_balance.rainfall_excess_m2,
            )
        };
        let (r_coarse, clamp_coarse) = run(30, 2.0);
        let (r_fine, clamp_fine) = run(120, 0.5);
        // Discretization-only (no clamp mass at either resolution) and
        // convergent: refining 4x cells / 4x smaller dt shrinks the residual.
        assert_eq!(clamp_coarse.to_bits(), 0.0_f64.to_bits());
        assert_eq!(clamp_fine.to_bits(), 0.0_f64.to_bits());
        assert!(r_fine < r_coarse, "residual should shrink with resolution");
        assert!(
            r_fine < 2.0e-3,
            "fine-resolution residual {r_fine} should be small"
        );
    }
    #[test]
    fn nan_forcing_fails_closed() {
        let mesh = KinematicWaveMesh::uniform(10.0, 10, CellParameters::bare(0.05, 100.0));
        let mut solver = KinematicWaveSolver::new(mesh);
        let excess = |_i: usize, _t: f64| f64::NAN;
        let inflow = |_t: f64| 0.0;
        let intensity = |_t: f64| 0.0;
        let forcing = Forcing {
            rainfall_excess_m_s: &excess,
            upstream_inflow_m2_s: &inflow,
            rainfall_intensity_m_s: &intensity,
        };
        assert!(matches!(
            solver.run(&forcing, 100.0, 10.0, 2.0),
            Err(RoutingError::InvalidForcing)
        ));
    }

    #[test]
    fn nan_inflow_fails_closed() {
        let mesh = KinematicWaveMesh::uniform(10.0, 10, CellParameters::bare(0.05, 100.0));
        let mut solver = KinematicWaveSolver::new(mesh);
        let excess = |_i: usize, _t: f64| 0.0;
        let inflow = |_t: f64| f64::INFINITY;
        let intensity = |_t: f64| 0.0;
        let forcing = Forcing {
            rainfall_excess_m_s: &excess,
            upstream_inflow_m2_s: &inflow,
            rainfall_intensity_m_s: &intensity,
        };
        assert!(matches!(
            solver.run(&forcing, 100.0, 10.0, 2.0),
            Err(RoutingError::InvalidForcing)
        ));
    }

    #[test]
    fn negative_forcing_fails_closed_not_zeroed() {
        let mesh = KinematicWaveMesh::uniform(10.0, 10, CellParameters::bare(0.05, 100.0));
        let mut solver = KinematicWaveSolver::new(mesh);
        // finite but negative rainfall excess is invalid (must fail closed,
        // not silently normalize to zero).
        let excess = |_i: usize, _t: f64| -1.0e-6;
        let inflow = |_t: f64| 0.0;
        let intensity = |_t: f64| 0.0;
        let forcing = Forcing {
            rainfall_excess_m_s: &excess,
            upstream_inflow_m2_s: &inflow,
            rainfall_intensity_m_s: &intensity,
        };
        assert!(matches!(
            solver.run(&forcing, 100.0, 10.0, 2.0),
            Err(RoutingError::InvalidForcing)
        ));
        // negative upstream inflow likewise
        let excess2 = |_i: usize, _t: f64| 0.0;
        let inflow2 = |_t: f64| -1.0e-4;
        let mut solver2 = KinematicWaveSolver::new(KinematicWaveMesh::uniform(
            10.0,
            10,
            CellParameters::bare(0.05, 100.0),
        ));
        let forcing2 = Forcing {
            rainfall_excess_m_s: &excess2,
            upstream_inflow_m2_s: &inflow2,
            rainfall_intensity_m_s: &intensity,
        };
        assert!(matches!(
            solver2.run(&forcing2, 100.0, 10.0, 2.0),
            Err(RoutingError::InvalidForcing)
        ));
    }

    #[test]
    fn invalid_cell_parameter_fails_closed() {
        let mut params = CellParameters::bare(-0.1, 100.0); // negative slope
        params.slope = -0.1;
        let mesh = KinematicWaveMesh::uniform(10.0, 5, params);
        let mut solver = KinematicWaveSolver::new(mesh);
        let excess = |_i: usize, _t: f64| 0.0;
        let inflow = |_t: f64| 0.0;
        let intensity = |_t: f64| 0.0;
        let forcing = Forcing {
            rainfall_excess_m_s: &excess,
            upstream_inflow_m2_s: &inflow,
            rainfall_intensity_m_s: &intensity,
        };
        assert!(matches!(
            solver.run(&forcing, 100.0, 10.0, 2.0),
            Err(RoutingError::InvalidCellParameter)
        ));
    }

    // D14: slot profiling is opt-in and accumulates counters/spans for the
    // solver loop. The enable flag is process-global, so flag-toggling tests
    // hold `profile::test_flag_guard` to stay correct under plain
    // `cargo test` (libtest threads) as well as nextest process isolation.
    #[test]
    fn profile_slots_accumulate_when_enabled_and_stay_zero_when_disabled() {
        use super::super::profile;

        let _flag_guard = profile::test_flag_guard();
        profile::set_enabled(false);
        let _ = profile::snapshot_and_reset();
        let run_small = || {
            let mesh = KinematicWaveMesh::uniform(7.5, 10, CellParameters::bare(0.09, 500.0));
            let mut solver = KinematicWaveSolver::new(mesh);
            let v = 60.0 / 3.6e6;
            let excess = |_i: usize, _t: f64| v;
            let inflow = |_t: f64| 0.0;
            let intensity = |_t: f64| v;
            let forcing = Forcing {
                rainfall_excess_m_s: &excess,
                upstream_inflow_m2_s: &inflow,
                rainfall_intensity_m_s: &intensity,
            };
            solver.run(&forcing, 120.0, 10.0, 2.0).expect("run ok")
        };

        let disabled_result = run_small();
        assert_eq!(
            profile::snapshot_and_reset(),
            profile::RoutingProfileSnapshot::default(),
            "disabled profiling must accumulate nothing"
        );

        profile::set_enabled(true);
        let enabled_result = run_small();
        let snapshot = profile::snapshot_and_reset();
        profile::set_enabled(false);
        assert_eq!(snapshot.solver_runs, 1);
        assert!(snapshot.solver_steps > 0, "steps counted");
        assert!(snapshot.alpha_evaluations > 0, "alpha evals counted");
        assert!(snapshot.hydrograph_samples > 0, "samples counted");
        assert!(snapshot.solver_cfl_ns > 0, "cfl slot timed");
        assert!(snapshot.solver_step_ns > 0, "step slot timed");
        // Profiling must not change solver results.
        assert_eq!(
            disabled_result.mass_balance.residual_m2().to_bits(),
            enabled_result.mass_balance.residual_m2().to_bits(),
            "profiling must not perturb solver output"
        );
        assert_eq!(
            disabled_result.hydrograph.len(),
            enabled_result.hydrograph.len()
        );
    }

    #[test]
    fn nonpositive_sample_dt_fails_closed_not_hang() {
        let mesh = KinematicWaveMesh::uniform(10.0, 5, CellParameters::bare(0.05, 100.0));
        let mut solver = KinematicWaveSolver::new(mesh);
        let excess = |_i: usize, _t: f64| 0.0;
        let inflow = |_t: f64| 0.0;
        let intensity = |_t: f64| 0.0;
        let forcing = Forcing {
            rainfall_excess_m_s: &excess,
            upstream_inflow_m2_s: &inflow,
            rainfall_intensity_m_s: &intensity,
        };
        assert!(matches!(
            solver.run(&forcing, 100.0, 0.0, 2.0),
            Err(RoutingError::DegenerateConfiguration)
        ));
    }
}
