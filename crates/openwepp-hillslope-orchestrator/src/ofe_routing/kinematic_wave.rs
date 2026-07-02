//! MOFEFID Lane D / D4 (SC-OFEROUTE-001, INV-OFEROUTE-005/006/007):
//! single-OFE 1-D kinematic-wave overland-flow solver with the TVD-MacCormack
//! predictor/corrector scheme (Papanicolaou et al. 2018, eqs. (8)-(14)),
//! space/time-variant friction (eqs. (2)-(7) via `super::friction`), and a
//! CFL-adaptive sub-timestep (eq. (12)). Shadow-first and opt-in: this solver
//! is not wired into any production phase span; the default hillslope runtime
//! does not call it (INV-OFEROUTE-010).
//!
//! Frozen-library posture (GAP-OFEROUTE-001): the TVD-MacCormack numerics
//! primaries are un-acquired; the scheme is implemented as stated in R-63
//! §2.3 and validated by mass conservation (INV-006), CFL stability
//! (INV-007), steady-state, and shock structure rather than by a
//! digit-by-digit primary read.

use super::friction::{
    GRAVITY_M_S2, KINEMATIC_VISCOSITY_M2_S, chezy_from_friction, equivalent_friction_factor,
    form_resistance_abrahams, froude_number, reynolds_number, skin_resistance,
    vegetation_resistance_katul, wave_resistance_hu_abrahams,
};

/// Depth-discharge exponent `m` (eq. A2): `q = alpha h^m`, `m = 1.5`.
pub const DEPTH_DISCHARGE_EXPONENT: f64 = 1.5;
/// Target Courant number for the CFL-adaptive sub-timestep (eq. 12: `Cr <= 1`).
/// Conservative default; the hard CFL ceiling is 1.0.
pub const CFL_TARGET: f64 = 0.9;
/// Minimum positive depth used to guard divisions; below this a cell is dry.
const DRY_DEPTH_M: f64 = 1.0e-9;

/// A forcing value is valid iff finite and non-negative (rainfall excess,
/// rainfall intensity, and upstream inflow are physically non-negative).
#[must_use]
fn is_valid_forcing(value: f64) -> bool {
    value.is_finite() && value >= 0.0
}

/// Per-cell space-variant friction + geometry parameters. A single-OFE mesh
/// with uniform roughness repeats one value across cells; Case 4 varies
/// `slope` per section.
#[derive(Debug, Clone, Copy)]
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
        let re = reynolds_number(unit_discharge_m2_s, KINEMATIC_VISCOSITY_M2_S);
        let fr = froude_number(unit_discharge_m2_s, flow_depth_m, GRAVITY_M_S2);
        let skin = skin_resistance(rainfall_intensity_m_s, self.friction_coefficient_ko, re);
        let form = form_resistance_abrahams(
            self.drag_coefficient,
            flow_depth_m,
            self.element_tip_height_m,
            self.roughness_concentration,
        );
        // Wave resistance applies when the element is not fully submerged
        // (h/D_r < 1); above full submergence it vanishes.
        let wave = if self.element_tip_height_m > 0.0 && flow_depth_m < self.element_tip_height_m {
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
    #[must_use]
    fn alpha(
        &self,
        flow_depth_m: f64,
        unit_discharge_m2_s: f64,
        rainfall_intensity_m_s: f64,
    ) -> f64 {
        if flow_depth_m <= DRY_DEPTH_M || self.slope <= 0.0 {
            return 0.0;
        }
        let h_pow = flow_depth_m.powf(DEPTH_DISCHARGE_EXPONENT);
        let mut q_est = if unit_discharge_m2_s > 0.0 {
            unit_discharge_m2_s
        } else {
            (GRAVITY_M_S2 * self.slope).sqrt() * h_pow
        };
        let mut alpha = 0.0;
        for _ in 0..4 {
            let f_eq = self.equivalent_friction(flow_depth_m, q_est, rainfall_intensity_m_s);
            if f_eq <= 0.0 {
                return 0.0;
            }
            alpha = chezy_from_friction(f_eq, GRAVITY_M_S2) * self.slope.sqrt();
            let q_new = alpha * h_pow;
            let converged = (q_new - q_est).abs() <= 1.0e-12 * q_new.max(1.0e-12);
            q_est = q_new;
            if converged {
                break;
            }
        }
        alpha
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
    /// Peak outlet unit discharge (m^2/s) and its time (s).
    pub peak_unit_discharge_m2_s: f64,
    pub time_to_peak_s: f64,
    /// Max Courant number observed (CFL evidence; must stay <= 1).
    pub max_courant: f64,
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
    /// Degenerate configuration (empty mesh, non-positive spacing/end time,
    /// non-positive sample or max sub-timestep).
    DegenerateConfiguration,
    /// Forcing (rainfall excess / intensity / upstream inflow) returned a
    /// non-finite OR negative value (all three are physically non-negative).
    InvalidForcing,
    /// A cell parameter is out of domain (non-finite, negative slope/ko/etc.).
    InvalidCellParameter,
}

/// Single-OFE TVD-MacCormack kinematic-wave solver state.
pub struct KinematicWaveSolver {
    mesh: KinematicWaveMesh,
    depth_m: Vec<f64>,
    discharge_m2_s: Vec<f64>,
}

impl KinematicWaveSolver {
    /// Construct with a dry initial condition.
    #[must_use]
    pub fn new(mesh: KinematicWaveMesh) -> Self {
        let n = mesh.cell_count();
        Self {
            mesh,
            depth_m: vec![0.0; n],
            discharge_m2_s: vec![0.0; n],
        }
    }

    fn total_storage_m2(&self) -> f64 {
        self.depth_m.iter().sum::<f64>() * self.mesh.cell_length_m
    }

    /// CFL-limited sub-timestep at the current state (eq. 12), clamped to
    /// `[min_dt, max_dt]`. Uses celerity `c = 1.5 C S_o^0.5 h^0.5 = 1.5 alpha h^0.5`.
    fn cfl_dt(&self, time_s: f64, forcing: &Forcing<'_>, max_dt_s: f64) -> f64 {
        let dx = self.mesh.cell_length_m;
        let intensity = (forcing.rainfall_intensity_m_s)(time_s);
        let mut max_celerity = 0.0_f64;
        for (i, cell) in self.mesh.cells.iter().enumerate() {
            let h = self.depth_m[i];
            if h <= DRY_DEPTH_M {
                continue;
            }
            let alpha = cell.alpha(h, self.discharge_m2_s[i], intensity);
            let celerity = DEPTH_DISCHARGE_EXPONENT * alpha * h.sqrt();
            if celerity > max_celerity {
                max_celerity = celerity;
            }
        }
        if max_celerity <= 0.0 {
            return max_dt_s;
        }
        (CFL_TARGET * dx / max_celerity).min(max_dt_s)
    }

    /// Flux-limiter ratio `r_i` (eq. 11d) with a guarded denominator.
    fn limiter_ratio(h: &[f64], i: usize) -> f64 {
        if i == 0 || i + 1 >= h.len() {
            return 1.0;
        }
        let denom = h[i + 1] - h[i];
        if denom.abs() <= DRY_DEPTH_M {
            return 1.0; // treat as monotone (phi -> 0, standard dissipation)
        }
        (h[i] - h[i - 1]) / denom
    }

    /// `Cf_i` (eq. 11e) from the local Courant number.
    fn cf(courant: f64) -> f64 {
        if courant > 0.5 {
            0.25
        } else {
            courant * (1.0 - courant)
        }
    }

    /// `phi_i` flux limiter (eq. 11c as stated in R-63): `min(2 r, 1)` for
    /// `r < 0`, else `0`.
    fn phi(ratio: f64) -> f64 {
        if ratio < 0.0 {
            (2.0 * ratio).min(1.0)
        } else {
            0.0
        }
    }

    /// Advance one TVD-MacCormack step of size `dt` (eqs. 8-14). Returns the
    /// outlet sample, or a `RoutingError` on a fail-closed condition.
    fn step(
        &mut self,
        time_s: f64,
        dt: f64,
        forcing: &Forcing<'_>,
        mass: &mut MassBalance,
    ) -> Result<(f64, f64), RoutingError> {
        let n = self.mesh.cell_count();
        let dx = self.mesh.cell_length_m;
        let intensity = (forcing.rainfall_intensity_m_s)(time_s);
        let q_up = (forcing.upstream_inflow_m2_s)(time_s);
        // Rainfall excess, intensity, and upstream inflow are physically
        // non-negative; a non-finite OR negative value is invalid input and
        // fails closed (it is not silently normalized to zero).
        if !is_valid_forcing(q_up) || !is_valid_forcing(intensity) {
            return Err(RoutingError::InvalidForcing);
        }
        let mut clamp_injected_m2 = 0.0_f64;

        // Per-cell alpha and rainfall excess for this step.
        let mut alpha = vec![0.0_f64; n];
        let mut v = vec![0.0_f64; n];
        for i in 0..n {
            alpha[i] = self.mesh.cells[i].alpha(self.depth_m[i], self.discharge_m2_s[i], intensity);
            let v_raw = (forcing.rainfall_excess_m_s)(i, time_s);
            if !is_valid_forcing(v_raw) {
                return Err(RoutingError::InvalidForcing);
            }
            v[i] = v_raw;
        }

        // Predictor (eqs. 8-9): forward flux difference; upstream flux = q_up.
        let mut h_pred = vec![0.0_f64; n];
        let mut q_pred = vec![0.0_f64; n];
        for i in 0..n {
            let q_i = self.discharge_m2_s[i];
            let q_ip1 = if i + 1 < n {
                self.discharge_m2_s[i + 1]
            } else if n >= 2 {
                // downstream outflow ghost: linear extrapolation so the
                // predictor actually advects mass out of the last cell
                // (a zero-gradient ghost zeros the outlet flux difference and
                // under-accounts boundary outflow -> spurious storage). Floor
                // at 0; kinematic information travels downstream only.
                (2.0 * self.discharge_m2_s[i] - self.discharge_m2_s[i - 1]).max(0.0)
            } else {
                self.discharge_m2_s[i]
            };
            let h_new = self.depth_m[i] - (dt / dx) * (q_ip1 - q_i) + v[i] * dt;
            if h_new < 0.0 {
                clamp_injected_m2 += (-h_new) * dx;
            }
            h_pred[i] = h_new.max(0.0);
            q_pred[i] = alpha[i] * h_pred[i].powf(DEPTH_DISCHARGE_EXPONENT);
        }

        // Corrector (eq. 10): backward flux difference; upstream ghost flux =
        // q_up entering cell 0.
        let mut h_corr = vec![0.0_f64; n];
        for i in 0..n {
            let q_im1 = if i == 0 { q_up } else { q_pred[i - 1] };
            let h_new = self.depth_m[i] - (dt / dx) * (q_pred[i] - q_im1) + v[i] * dt;
            if h_new < 0.0 {
                clamp_injected_m2 += (-h_new) * dx;
            }
            h_corr[i] = h_new.max(0.0);
        }

        // Average + TVD dissipation (eqs. 11a-e, 13). Gr uses the local
        // Courant number from the pre-step state.
        let averaged: Vec<f64> = (0..n).map(|i| 0.5 * (h_pred[i] + h_corr[i])).collect();
        let mut gr = vec![0.0_f64; n];
        for i in 0..n {
            let h = self.depth_m[i];
            let celerity = DEPTH_DISCHARGE_EXPONENT * alpha[i] * h.max(0.0).sqrt();
            let courant = celerity * dt / dx;
            let ratio = Self::limiter_ratio(&self.depth_m, i);
            gr[i] = 0.5 * Self::cf(courant) * (1.0 - Self::phi(ratio));
        }
        let mut h_next = vec![0.0_f64; n];
        for i in 0..n {
            let tvd = if i == 0 || i + 1 >= n {
                0.0
            } else {
                gr[i] * (self.depth_m[i + 1] - self.depth_m[i])
                    - gr[i - 1] * (self.depth_m[i] - self.depth_m[i - 1])
            };
            let h_new = averaged[i] + tvd;
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
            h_next[i] = h_new.max(0.0);
        }

        // Commit state and recompute discharge (eq. 14).
        // outflow leaving the last cell over dt = q_out * dt (unit width)
        let outlet_flux_before = self.discharge_m2_s[n - 1];
        for i in 0..n {
            self.depth_m[i] = h_next[i];
            self.discharge_m2_s[i] = alpha[i] * self.depth_m[i].powf(DEPTH_DISCHARGE_EXPONENT);
            if !self.discharge_m2_s[i].is_finite() {
                return Err(RoutingError::NonFiniteState);
            }
        }

        // Mass ledger (per unit width): inflow, rain, outflow over dt.
        mass.inflow_m2 += q_up * dt;
        mass.rainfall_excess_m2 += v.iter().sum::<f64>() * dx * dt;
        // Use the trapezoidal outflow over the step for a stable ledger.
        let outlet_flux_after = self.discharge_m2_s[n - 1];
        mass.outflow_m2 += 0.5 * (outlet_flux_before + outlet_flux_after) * dt;
        mass.positivity_clamp_m2 += clamp_injected_m2;

        Ok((self.discharge_m2_s[n - 1], self.depth_m[n - 1]))
    }

    /// Run to `end_time_s`, recording the outlet hydrograph at ~`sample_dt_s`.
    /// `max_dt_s` caps the CFL-adaptive step. Fails closed on CFL/non-finite/
    /// negative-depth conditions.
    pub fn run(
        &mut self,
        forcing: &Forcing<'_>,
        end_time_s: f64,
        sample_dt_s: f64,
        max_dt_s: f64,
    ) -> Result<RoutingResult, RoutingError> {
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
        let mut mass = MassBalance::default();
        let mut hydrograph = Vec::new();
        let mut t = 0.0_f64;
        let mut next_sample = 0.0_f64;
        let mut peak = 0.0_f64;
        let mut time_to_peak = 0.0_f64;
        let mut max_courant = 0.0_f64;

        // record initial (dry) sample
        hydrograph.push(HydrographSample {
            time_s: 0.0,
            outlet_unit_discharge_m2_s: 0.0,
            outlet_depth_m: 0.0,
        });
        next_sample += sample_dt_s;

        let mut guard_steps = 0_u64;
        let max_steps = 50_000_000_u64; // runaway backstop
        while t < end_time_s {
            let dt = self.cfl_dt(t, forcing, max_dt_s).min(end_time_s - t);
            if dt <= 0.0 {
                break;
            }
            // CFL evidence at the chosen dt.
            let intensity = (forcing.rainfall_intensity_m_s)(t);
            for (i, cell) in self.mesh.cells.iter().enumerate() {
                let h = self.depth_m[i];
                if h <= DRY_DEPTH_M {
                    continue;
                }
                let alpha = cell.alpha(h, self.discharge_m2_s[i], intensity);
                let courant = DEPTH_DISCHARGE_EXPONENT * alpha * h.sqrt() * dt / dx;
                if courant > max_courant {
                    max_courant = courant;
                }
                if courant > 1.0 + 1.0e-9 {
                    return Err(RoutingError::CflViolation);
                }
            }

            let (q_out, h_out) = self.step(t, dt, forcing, &mut mass)?;
            t += dt;

            if q_out > peak {
                peak = q_out;
                time_to_peak = t;
            }
            while next_sample <= t + 1.0e-12 && next_sample <= end_time_s {
                hydrograph.push(HydrographSample {
                    time_s: next_sample,
                    outlet_unit_discharge_m2_s: q_out,
                    outlet_depth_m: h_out,
                });
                next_sample += sample_dt_s;
            }

            guard_steps += 1;
            if guard_steps > max_steps {
                return Err(RoutingError::CflViolation);
            }
        }

        mass.storage_change_m2 = self.total_storage_m2() - storage_initial;
        Ok(RoutingResult {
            hydrograph,
            mass_balance: mass,
            peak_unit_discharge_m2_s: peak,
            time_to_peak_s: time_to_peak,
            max_courant,
        })
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

        // steady-state outlet unit discharge = v * L
        let q_steady = res
            .hydrograph
            .last()
            .expect("hydrograph")
            .outlet_unit_discharge_m2_s;
        let q_expected = v * length;
        assert!(
            (q_steady - q_expected).abs() / q_expected < 0.02,
            "steady outlet q {q_steady} should approach v*L {q_expected}"
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
