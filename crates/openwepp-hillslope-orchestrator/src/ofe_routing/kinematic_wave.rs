//! MOFEFID Lane D / D4 (SC-OFEROUTE-001, INV-OFEROUTE-005/006/007):
//! single-OFE 1-D kinematic-wave overland-flow solver with the TVD-MacCormack
//! predictor/corrector scheme (Papanicolaou et al. 2018, eqs. (8)-(14)),
//! space/time-variant friction (eqs. (2)-(7) via `super::friction`), and a
//! CFL-adaptive sub-timestep (eq. (12)). The solver was introduced
//! shadow-first; rev-27 active owner and rev-46 conditional default activation
//! now call it on coefficient-complete Lane D active runtime paths
//! (INV-OFEROUTE-010).
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
    GRAVITY_M_S2, KINEMATIC_VISCOSITY_M2_S, SKIN_REGIME_REYNOLDS_THRESHOLD, WAVE_FROUDE_THRESHOLD,
    chezy_from_friction, equivalent_friction_factor, form_resistance_abrahams, froude_number,
    reynolds_number, skin_rain_term, skin_resistance_with_rain_term, vegetation_length_scale,
    vegetation_momentum_absorption, vegetation_resistance_katul, wave_resistance_hu_abrahams,
};
use super::profile;

/// Depth-discharge exponent `m` (eq. A2): `q = alpha h^m`, `m = 1.5`.
pub const DEPTH_DISCHARGE_EXPONENT: f64 = 1.5;
/// Target Courant number for the CFL-adaptive sub-timestep (eq. 12: `Cr <= 1`).
/// Conservative default; the hard CFL ceiling is 1.0.
pub const CFL_TARGET: f64 = 0.9;
/// Minimum positive depth used to guard divisions; below this a cell is dry.
pub(super) const DRY_DEPTH_M: f64 = 1.0e-9;
const ALPHA_NEWTON_MAX_ITERS: usize = 8;
const ALPHA_NEWTON_REL_TOL: f64 = 1.0e-12;
const ALPHA_NEWTON_ABS_TOL_M2_S: f64 = 1.0e-18;

#[must_use]
fn depth_pow_3_2(flow_depth_m: f64) -> f64 {
    flow_depth_m * flow_depth_m.sqrt()
}

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
    pub(super) fn validate(&self) -> Result<(), RoutingError> {
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
            // dependence, so rev-47 evaluates alpha/q/celerity directly.
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

    fn vegetation_resistance_and_derivative(
        &self,
        flow_depth_m: f64,
    ) -> Result<(f64, f64), RoutingError> {
        if flow_depth_m <= 0.0
            || self.canopy_height_m <= 0.0
            || self.leaf_area_index <= 0.0
            || self.vegetation_drag_coefficient <= 0.0
        {
            return Ok((0.0, 0.0));
        }
        let l_c = vegetation_length_scale(
            self.vegetation_drag_coefficient,
            self.leaf_area_index,
            self.canopy_height_m,
        );
        if !l_c.is_finite() || l_c <= 0.0 {
            return Err(RoutingError::NonFiniteState);
        }
        let beta = vegetation_momentum_absorption(self.leaf_area_index, self.canopy_height_m);
        if !beta.is_finite() || beta <= 0.0 {
            return Err(RoutingError::NonFiniteState);
        }
        let two_beta_sq = 2.0 * beta * beta;
        let b = 1.0 / (two_beta_sq * l_c);
        if !b.is_finite() {
            return Err(RoutingError::NonFiniteState);
        }
        let z = b * flow_depth_m;
        if !z.is_finite() {
            return Err(RoutingError::NonFiniteState);
        }
        let expm1_z = z.exp_m1();
        if expm1_z <= 0.0 || !expm1_z.is_finite() {
            return Err(RoutingError::NonFiniteState);
        }
        let term_exp_neg = (-self.canopy_height_m / (two_beta_sq * l_c)).exp();
        if !term_exp_neg.is_finite() {
            return Err(RoutingError::NonFiniteState);
        }
        let radicand = 2.0 * beta * (l_c / flow_depth_m) * term_exp_neg * expm1_z;
        if radicand <= 0.0 || !radicand.is_finite() {
            return Err(RoutingError::NonFiniteState);
        }
        let resistance = radicand.sqrt();
        let d_log_radicand_dh = b * (expm1_z + 1.0) / expm1_z - 1.0 / flow_depth_m;
        let derivative = 0.5 * resistance * d_log_radicand_dh;
        if !resistance.is_finite() || !derivative.is_finite() {
            return Err(RoutingError::NonFiniteState);
        }
        Ok((resistance, derivative))
    }

    fn equivalent_friction_local(
        &self,
        flow_depth_m: f64,
        unit_discharge_m2_s: f64,
        skin_rain_term: f64,
    ) -> Result<LocalFriction, RoutingError> {
        if self.manning_n > 0.0 {
            let h = flow_depth_m.max(DRY_DEPTH_M);
            let f_eq = 8.0 * GRAVITY_M_S2 * self.manning_n * self.manning_n / h.cbrt();
            return Ok(LocalFriction {
                f_eq,
                df_dq: 0.0,
                df_dh: -(1.0 / 3.0) * f_eq / h,
            });
        }
        if flow_depth_m <= DRY_DEPTH_M || unit_discharge_m2_s <= 0.0 {
            return Ok(LocalFriction {
                f_eq: 0.0,
                df_dq: 0.0,
                df_dh: 0.0,
            });
        }
        let re = reynolds_number(unit_discharge_m2_s, KINEMATIC_VISCOSITY_M2_S);
        let skin = skin_resistance_with_rain_term(skin_rain_term, self.friction_coefficient_ko, re);
        let skin_dq = if re > SKIN_REGIME_REYNOLDS_THRESHOLD {
            -0.45 * skin / unit_discharge_m2_s
        } else if re > 0.0 {
            -skin / unit_discharge_m2_s
        } else {
            0.0
        };

        let form = form_resistance_abrahams(
            self.drag_coefficient,
            flow_depth_m,
            self.element_tip_height_m,
            self.roughness_concentration,
        );
        let form_dh = if form > 0.0 { form / flow_depth_m } else { 0.0 };

        let mut wave_flow_partial = 0.0;
        let mut wave_depth_partial = 0.0;
        let wave = if self.element_tip_height_m > 0.0 && flow_depth_m < self.element_tip_height_m {
            let fr = froude_number(unit_discharge_m2_s, flow_depth_m, GRAVITY_M_S2);
            let wave = wave_resistance_hu_abrahams(self.roughness_concentration, fr);
            if fr > WAVE_FROUDE_THRESHOLD {
                wave_flow_partial = -0.5 * wave / unit_discharge_m2_s;
                wave_depth_partial = 0.75 * wave / flow_depth_m;
            } else if fr > 0.0 {
                wave_flow_partial = wave / unit_discharge_m2_s;
                wave_depth_partial = -1.5 * wave / flow_depth_m;
            }
            wave
        } else {
            0.0
        };

        let (veg, veg_dh) = self.vegetation_resistance_and_derivative(flow_depth_m)?;
        let f_eq = equivalent_friction_factor(skin, form, wave, veg);
        let discharge_partial = skin_dq + wave_flow_partial;
        let depth_partial = form_dh + wave_depth_partial + veg_dh;
        if !f_eq.is_finite() || !discharge_partial.is_finite() || !depth_partial.is_finite() {
            return Err(RoutingError::NonFiniteState);
        }
        Ok(LocalFriction {
            f_eq,
            df_dq: discharge_partial,
            df_dh: depth_partial,
        })
    }

    fn skin_only_hydraulics(
        &self,
        flow_depth_m: f64,
        h_pow: f64,
        unit_discharge_m2_s: f64,
        skin_rain_term: f64,
    ) -> Option<Result<LocalHydraulics, RoutingError>> {
        let form_or_wave_active =
            self.element_tip_height_m > 0.0 && self.roughness_concentration > 0.0;
        let vegetation_active = self.vegetation_drag_coefficient > 0.0
            && self.leaf_area_index > 0.0
            && self.canopy_height_m > 0.0;
        if form_or_wave_active || vegetation_active {
            return None;
        }
        let skin_numerator = skin_rain_term + self.friction_coefficient_ko;
        if skin_numerator <= 0.0 {
            return Some(Ok(LocalHydraulics::zero()));
        }

        let q_laminar = 8.0 * GRAVITY_M_S2 * self.slope * flow_depth_m.powi(3)
            / (skin_numerator * KINEMATIC_VISCOSITY_M2_S);
        if q_laminar.is_finite()
            && q_laminar > 0.0
            && reynolds_number(q_laminar, KINEMATIC_VISCOSITY_M2_S)
                <= SKIN_REGIME_REYNOLDS_THRESHOLD
        {
            return Some(Ok(LocalHydraulics {
                alpha: q_laminar / h_pow,
                q: q_laminar,
                celerity: 3.0 * q_laminar / flow_depth_m,
            }));
        }

        let laminar_hydraulics = || {
            if q_laminar.is_finite() && q_laminar > 0.0 {
                Some(LocalHydraulics {
                    alpha: q_laminar / h_pow,
                    q: q_laminar,
                    celerity: 3.0 * q_laminar / flow_depth_m,
                })
            } else {
                None
            }
        };

        let hirsch_factor = 3.19 * KINEMATIC_VISCOSITY_M2_S.powf(0.45);
        let q_hirsch_base = (8.0 * GRAVITY_M_S2 * self.slope / hirsch_factor).sqrt() * h_pow;
        let q_hirsch = q_hirsch_base.powf(1.0 / 0.775);
        if q_hirsch.is_finite()
            && q_hirsch > 0.0
            && reynolds_number(q_hirsch, KINEMATIC_VISCOSITY_M2_S) > SKIN_REGIME_REYNOLDS_THRESHOLD
        {
            return Some(Ok(LocalHydraulics {
                alpha: q_hirsch / h_pow,
                q: q_hirsch,
                celerity: (DEPTH_DISCHARGE_EXPONENT / 0.775) * q_hirsch / flow_depth_m,
            }));
        }

        let hirsch_hydraulics = || {
            if q_hirsch.is_finite() && q_hirsch > 0.0 {
                Some(LocalHydraulics {
                    alpha: q_hirsch / h_pow,
                    q: q_hirsch,
                    celerity: (DEPTH_DISCHARGE_EXPONENT / 0.775) * q_hirsch / flow_depth_m,
                })
            } else {
                None
            }
        };

        let seed_re = reynolds_number(unit_discharge_m2_s.max(0.0), KINEMATIC_VISCOSITY_M2_S);
        let selected = if seed_re > SKIN_REGIME_REYNOLDS_THRESHOLD {
            hirsch_hydraulics()
        } else {
            laminar_hydraulics()
        };
        Some(selected.ok_or(RoutingError::NonFiniteState))
    }

    /// Kinematic coefficient, discharge, and celerity for the local
    /// equilibrium relation `q = sqrt(8 g S_o / f_eq(q,h)) h^1.5`.
    fn alpha_q_celerity(
        &self,
        flow_depth_m: f64,
        unit_discharge_m2_s: f64,
        skin_rain_term: f64,
    ) -> Result<LocalHydraulics, RoutingError> {
        if flow_depth_m <= DRY_DEPTH_M || self.slope <= 0.0 {
            return Ok(LocalHydraulics::zero());
        }
        if self.manning_n > 0.0 {
            let alpha = self.slope.sqrt() / self.manning_n * flow_depth_m.powf(1.0 / 6.0);
            let q = alpha * depth_pow_3_2(flow_depth_m);
            let celerity = (5.0 / 3.0) * q / flow_depth_m;
            return Ok(LocalHydraulics { alpha, q, celerity });
        }
        let h_pow = depth_pow_3_2(flow_depth_m);
        if let Some(local) =
            self.skin_only_hydraulics(flow_depth_m, h_pow, unit_discharge_m2_s, skin_rain_term)
        {
            return local;
        }
        self.additive_hydraulics(flow_depth_m, h_pow, unit_discharge_m2_s, skin_rain_term)
    }

    fn additive_hydraulics(
        &self,
        flow_depth_m: f64,
        h_pow: f64,
        unit_discharge_m2_s: f64,
        skin_rain_term: f64,
    ) -> Result<LocalHydraulics, RoutingError> {
        let slope_sqrt = self.slope.sqrt();
        let Some(q_est) = self.solve_additive_discharge(
            flow_depth_m,
            h_pow,
            unit_discharge_m2_s,
            skin_rain_term,
            slope_sqrt,
        )?
        else {
            return Ok(LocalHydraulics::zero());
        };
        let friction = self.equivalent_friction_local(flow_depth_m, q_est, skin_rain_term)?;
        if friction.f_eq <= 0.0 {
            return Ok(LocalHydraulics::zero());
        }
        let alpha = chezy_from_friction(friction.f_eq, GRAVITY_M_S2) * slope_sqrt;
        let q = alpha * h_pow;
        if !alpha.is_finite() || !q.is_finite() || q < 0.0 {
            return Err(RoutingError::NonFiniteState);
        }
        let rel_residual = (q_est - q).abs() / q.max(1.0e-12);
        let abs_residual = (q_est - q).abs();
        if rel_residual > ALPHA_NEWTON_REL_TOL && abs_residual > ALPHA_NEWTON_ABS_TOL_M2_S {
            return Err(RoutingError::NonFiniteState);
        }
        let frozen = DEPTH_DISCHARGE_EXPONENT * alpha * flow_depth_m.sqrt();
        let denom = 1.0 + 0.5 * q * friction.df_dq / friction.f_eq;
        let numer = DEPTH_DISCHARGE_EXPONENT / flow_depth_m - 0.5 * friction.df_dh / friction.f_eq;
        if !denom.is_finite() || !numer.is_finite() || denom <= 0.0 {
            return Err(RoutingError::NonFiniteState);
        }
        let celerity = (q * numer / denom).max(frozen);
        if !celerity.is_finite() || celerity < 0.0 {
            return Err(RoutingError::NonFiniteState);
        }
        Ok(LocalHydraulics { alpha, q, celerity })
    }

    fn solve_additive_discharge(
        &self,
        flow_depth_m: f64,
        h_pow: f64,
        unit_discharge_m2_s: f64,
        skin_rain_term: f64,
        slope_sqrt: f64,
    ) -> Result<Option<f64>, RoutingError> {
        let mut q_est = if unit_discharge_m2_s > 0.0 {
            unit_discharge_m2_s
        } else {
            (GRAVITY_M_S2 * self.slope).sqrt() * h_pow
        };
        if !q_est.is_finite() || q_est <= 0.0 {
            return Err(RoutingError::NonFiniteState);
        }
        for _ in 0..ALPHA_NEWTON_MAX_ITERS {
            let friction = self.equivalent_friction_local(flow_depth_m, q_est, skin_rain_term)?;
            if friction.f_eq <= 0.0 {
                return Ok(None);
            }
            let alpha = chezy_from_friction(friction.f_eq, GRAVITY_M_S2) * slope_sqrt;
            let q_model = alpha * h_pow;
            if !alpha.is_finite() || !q_model.is_finite() || q_model <= 0.0 {
                return Err(RoutingError::NonFiniteState);
            }
            let rel_residual = (q_est - q_model).abs() / q_model.max(1.0e-12);
            if rel_residual <= ALPHA_NEWTON_REL_TOL {
                q_est = q_model;
                break;
            }
            let denom = 1.0 + 0.5 * q_est * friction.df_dq / friction.f_eq;
            if !denom.is_finite() || denom.abs() <= 1.0e-12 {
                return Err(RoutingError::NonFiniteState);
            }
            let log_residual = (q_est / q_model).ln();
            if !log_residual.is_finite() {
                return Err(RoutingError::NonFiniteState);
            }
            let log_step = (-log_residual / denom).clamp(-2.0, 2.0);
            let q_new = q_est * log_step.exp();
            if !q_new.is_finite() || q_new <= 0.0 {
                return Err(RoutingError::NonFiniteState);
            }
            let step_rel = (q_new - q_est).abs() / q_new.max(1.0e-12);
            q_est = q_new;
            if step_rel <= ALPHA_NEWTON_REL_TOL {
                break;
            }
        }
        Ok(Some(q_est))
    }
}

#[derive(Debug, Clone, Copy)]
struct LocalFriction {
    f_eq: f64,
    df_dq: f64,
    df_dh: f64,
}

#[derive(Debug, Clone, Copy)]
struct LocalHydraulics {
    alpha: f64,
    q: f64,
    celerity: f64,
}

impl LocalHydraulics {
    fn zero() -> Self {
        Self {
            alpha: 0.0,
            q: 0.0,
            celerity: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct StepCelerity {
    max_celerity: f64,
    max_cell_index: usize,
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
    /// Mass injected by residual positivity clamps (`h.max(0)`), per unit
    /// width (m^2). Rev 41 caps stage face fluxes and scales final TVD fluxes
    /// before candidate depths go negative, so this should remain zero except
    /// for sub-roundoff dry-floor cleanup.
    pub positivity_clamp_m2: f64,
    /// Scheme-actual upstream injection. Post-rev-24 BOTH sweeps carry
    /// `q_up` at the top face, so this equals `inflow_m2` by construction
    /// (booked-equals-actual identity surface; pre-rev-24 it exposed the
    /// `0.5 (q_up + q_0)` mismatch).
    pub scheme_inflow_m2: f64,
    /// DISCRETE SCHEME's actual downstream boundary outflow,
    /// `0.5 (q_pred_stage_out + q_corr_stage_out) dt` per step. Post-rev-24
    /// this equals `outflow_m2` by construction and remains as the
    /// booked-equals-actual identity surface.
    pub scheme_outflow_m2: f64,
    /// D10B diagnostic retained as a literal telescoping check for the
    /// face-based TVD term. Post-rev-24 this remains at machine noise because
    /// boundary dissipative faces are zero and interior face contributions
    /// cancel exactly.
    pub tvd_boundary_leak_m2: f64,
}

impl MassBalance {
    /// Closure residual: `in + rain - out - storage_change`.
    #[must_use]
    pub fn residual_m2(&self) -> f64 {
        self.inflow_m2 + self.rainfall_excess_m2 - self.outflow_m2 - self.storage_change_m2
    }

    /// Clamp-adjusted conservation residual: `residual + clamp`. The clamp
    /// term is retained for residual sub-roundoff dry-floor cleanup; material
    /// positivity is handled by rev-41 conservative flux limiting before
    /// publication.
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
    /// Optional diagnostic step trace. This is populated only by explicit
    /// row-scoped active-router diagnostics; normal solver runs carry `None`.
    pub step_trace: Option<Vec<KinematicWaveStepTraceRecord>>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct KinematicWaveStageLimiterTrace {
    pub reductions: u32,
    pub max_reduction_m2_s: f64,
    pub face_index: usize,
    pub face_x_m: f64,
}

struct PredictorCorrectorStages {
    clamp_injected_m2: f64,
    pred_out_face: f64,
    corr_out_face: f64,
    predictor_limiter: KinematicWaveStageLimiterTrace,
    corrector_limiter: KinematicWaveStageLimiterTrace,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct KinematicWaveTvdTrace {
    pub scale: f64,
    pub max_abs_delta_m: f64,
    pub cell_index: usize,
    pub cell_center_x_m: f64,
    pub signed_delta_m: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct KinematicWaveStepTraceRecord {
    pub step_index: u64,
    pub t_start_s: f64,
    pub t_end_s: f64,
    pub dt_s: f64,
    pub max_courant: f64,
    pub max_courant_cell_index: usize,
    pub max_courant_cell_center_x_m: f64,
    pub q_up_m2_s: f64,
    pub source_m2: f64,
    pub upstream_inflow_m2: f64,
    pub outflow_m2: f64,
    pub storage_before_m2: f64,
    pub storage_after_m2: f64,
    pub clamp_injected_m2: f64,
    pub pred_out_face_m2_s: f64,
    pub corr_out_face_m2_s: f64,
    pub outlet_depth_m: f64,
    pub outlet_unit_discharge_m2_s: f64,
    pub predictor_limiter: KinematicWaveStageLimiterTrace,
    pub corrector_limiter: KinematicWaveStageLimiterTrace,
    pub tvd: KinematicWaveTvdTrace,
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
    face_flux: Vec<f64>,
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
            face_flux: vec![0.0; cell_count + 1],
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
    /// Rev 51: valid production stage faces are non-negative by construction,
    /// so their accumulated per-bin outflow is non-negative without borrowing
    /// from later bins. The exact-total forward redistribution remains only as
    /// a defensive invariant path for an invalid or independently injected
    /// negative recorder sample. A material terminal deficit is RETURNED to
    /// the caller, which fails closed rather than publishing a negative outlet
    /// bin; sub-noise deficits are folded into the last covered bin so the
    /// series sums exactly to the booked outflow.
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

    /// Evaluate `alpha` and the TRUE kinematic celerity for every cell at
    /// the pre-step state into the step scratch, returning the max
    /// wet-cell celerity for CFL dt selection (eq. 12, rev 24).
    ///
    /// Rev-47 computes `dq/dh` analytically from the selected implicit
    /// friction branch derivatives, with the frozen-alpha expression
    /// `1.5 alpha h^0.5` retained only as a lower bound.
    fn prepare_step_alpha(&mut self, skin_rain_term: f64) -> Result<StepCelerity, RoutingError> {
        let mut max_celerity = 0.0_f64;
        let mut max_cell_index = 0_usize;
        let mut evaluations = 0_u64;
        for (i, cell) in self.mesh.cells.iter().enumerate() {
            let h = self.depth_m[i];
            let local = cell.alpha_q_celerity(h, self.discharge_m2_s[i], skin_rain_term)?;
            self.scratch.alpha[i] = local.alpha;
            evaluations += 1;
            if h <= DRY_DEPTH_M {
                self.scratch.celerity[i] = 0.0;
                self.scratch.q0[i] = 0.0;
                continue;
            }
            self.scratch.q0[i] = local.q;
            self.scratch.celerity[i] = local.celerity;
            if local.celerity > max_celerity {
                max_celerity = local.celerity;
                max_cell_index = i;
            }
        }
        profile::count_alpha_evaluations(evaluations);
        Ok(StepCelerity {
            max_celerity,
            max_cell_index,
        })
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

    /// Rev 41 positivity limiter: cap each outgoing stage face by the water
    /// available in its upwind cell over this substep. The incoming face has
    /// already been limited, so the operation is conservative over the stage
    /// fluxes rather than a post-hoc mass injection.
    fn limit_stage_face_fluxes(
        depth_m: &[f64],
        source_m_s: &[f64],
        dt: f64,
        dx: f64,
        faces_m2_s: &mut [f64],
        trace_enabled: bool,
    ) -> Result<KinematicWaveStageLimiterTrace, RoutingError> {
        let mut trace = KinematicWaveStageLimiterTrace::default();
        for i in 0..depth_m.len() {
            let available_m2 = depth_m[i] * dx + faces_m2_s[i] * dt + source_m_s[i] * dx * dt;
            if !available_m2.is_finite() {
                return Err(RoutingError::NonFiniteState);
            }
            let max_out_m2_s = (available_m2 / dt).max(0.0);
            if faces_m2_s[i + 1] > max_out_m2_s {
                if trace_enabled {
                    let reduction = faces_m2_s[i + 1] - max_out_m2_s;
                    trace.reductions = trace.reductions.saturating_add(1);
                    if reduction > trace.max_reduction_m2_s {
                        trace.max_reduction_m2_s = reduction;
                        trace.face_index = i + 1;
                        #[allow(clippy::cast_precision_loss)]
                        {
                            trace.face_x_m = (i + 1) as f64 * dx;
                        }
                    }
                }
                faces_m2_s[i + 1] = max_out_m2_s;
            }
        }
        Ok(trace)
    }

    /// Rev 41 final-stage positivity limiter. Returns one uniform scale for
    /// the face-based TVD correction so the correction still telescopes exactly
    /// while preventing negative committed depths.
    fn tvd_positivity_scale(averaged_m: &[f64], tvd_delta_m: &[f64]) -> Result<f64, RoutingError> {
        if averaged_m.len() != tvd_delta_m.len() {
            return Err(RoutingError::DegenerateConfiguration);
        }
        let mut scale = 1.0_f64;
        for (averaged, tvd) in averaged_m.iter().zip(tvd_delta_m) {
            if !averaged.is_finite() || !tvd.is_finite() {
                return Err(RoutingError::NonFiniteState);
            }
            if *averaged < -DRY_DEPTH_M {
                return Err(RoutingError::NegativeDepth);
            }
            if *tvd < 0.0 {
                let local_scale = averaged.max(0.0) / (-tvd);
                if local_scale < scale {
                    scale = local_scale;
                }
            }
        }
        Ok(scale)
    }

    fn predictor_corrector_stages(
        &mut self,
        time_s: f64,
        dt: f64,
        q_up: f64,
        forcing: &Forcing<'_>,
        trace_enabled: bool,
    ) -> Result<PredictorCorrectorStages, RoutingError> {
        let n = self.mesh.cell_count();
        let dx = self.mesh.cell_length_m;
        let mut clamp_injected_m2 = 0.0_f64;
        for i in 0..n {
            let v_raw = (forcing.rainfall_excess_m_s)(i, time_s);
            if !is_valid_forcing(v_raw) {
                return Err(RoutingError::InvalidForcing);
            }
            self.scratch.v[i] = v_raw;
        }

        self.scratch.face_flux[0] = q_up;
        for face in 1..n {
            self.scratch.face_flux[face] = self.discharge_m2_s[face];
        }
        let raw_predictor_outlet_m2_s = if n >= 2 {
            2.0 * self.discharge_m2_s[n - 1] - self.discharge_m2_s[n - 2]
        } else {
            self.discharge_m2_s[0]
        };
        if !raw_predictor_outlet_m2_s.is_finite() {
            return Err(RoutingError::NonFiniteState);
        }
        self.scratch.face_flux[n] = raw_predictor_outlet_m2_s.max(0.0);
        let predictor_limiter = Self::limit_stage_face_fluxes(
            &self.depth_m,
            &self.scratch.v,
            dt,
            dx,
            &mut self.scratch.face_flux[..=n],
            trace_enabled,
        )?;
        for i in 0..n {
            let h_new = self.depth_m[i]
                - (dt / dx) * (self.scratch.face_flux[i + 1] - self.scratch.face_flux[i])
                + self.scratch.v[i] * dt;
            if !h_new.is_finite() {
                return Err(RoutingError::NonFiniteState);
            }
            if h_new < -DRY_DEPTH_M {
                return Err(RoutingError::NegativeDepth);
            }
            if h_new < 0.0 {
                clamp_injected_m2 += 0.5 * (-h_new) * dx;
            }
            self.scratch.h_pred[i] = h_new.max(0.0);
            self.scratch.q_pred[i] = self.scratch.alpha[i] * depth_pow_3_2(self.scratch.h_pred[i]);
        }
        let pred_out_face = self.scratch.face_flux[n];

        self.scratch.face_flux[0] = q_up;
        for face in 1..=n {
            self.scratch.face_flux[face] = self.scratch.q_pred[face - 1];
        }
        let corrector_limiter = Self::limit_stage_face_fluxes(
            &self.depth_m,
            &self.scratch.v,
            dt,
            dx,
            &mut self.scratch.face_flux[..=n],
            trace_enabled,
        )?;
        for i in 0..n {
            let h_new = self.depth_m[i]
                - (dt / dx) * (self.scratch.face_flux[i + 1] - self.scratch.face_flux[i])
                + self.scratch.v[i] * dt;
            if !h_new.is_finite() {
                return Err(RoutingError::NonFiniteState);
            }
            if h_new < -DRY_DEPTH_M {
                return Err(RoutingError::NegativeDepth);
            }
            if h_new < 0.0 {
                clamp_injected_m2 += 0.5 * (-h_new) * dx;
            }
            self.scratch.h_corr[i] = h_new.max(0.0);
        }
        let corr_out_face = self.scratch.face_flux[n];
        for i in 0..n {
            self.scratch.averaged[i] = 0.5 * (self.scratch.h_pred[i] + self.scratch.h_corr[i]);
        }
        Ok(PredictorCorrectorStages {
            clamp_injected_m2,
            pred_out_face,
            corr_out_face,
            predictor_limiter,
            corrector_limiter,
        })
    }

    fn apply_tvd_stage(
        &mut self,
        dt: f64,
        trace_enabled: bool,
    ) -> Result<(f64, f64, KinematicWaveTvdTrace), RoutingError> {
        let n = self.mesh.cell_count();
        let dx = self.mesh.cell_length_m;
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
        let mut tvd_leak_m2 = 0.0_f64;
        for i in 0..n {
            let d_hi = if i + 1 < n { self.scratch.gr[i] } else { 0.0 };
            let d_lo = if i >= 1 { self.scratch.gr[i - 1] } else { 0.0 };
            tvd_leak_m2 += (d_hi - d_lo) * dx;
        }
        for i in 0..n {
            let d_hi = if i + 1 < n { self.scratch.gr[i] } else { 0.0 };
            let d_lo = if i >= 1 { self.scratch.gr[i - 1] } else { 0.0 };
            self.scratch.h_next[i] = d_hi - d_lo;
        }
        let mut tvd_trace = KinematicWaveTvdTrace {
            scale: 1.0,
            ..KinematicWaveTvdTrace::default()
        };
        if trace_enabled {
            for i in 0..n {
                let tvd = self.scratch.h_next[i];
                let abs_tvd = tvd.abs();
                if abs_tvd > tvd_trace.max_abs_delta_m {
                    tvd_trace.max_abs_delta_m = abs_tvd;
                    tvd_trace.cell_index = i;
                    #[allow(clippy::cast_precision_loss)]
                    {
                        tvd_trace.cell_center_x_m = (i as f64 + 0.5) * dx;
                    }
                    tvd_trace.signed_delta_m = tvd;
                }
            }
        }
        let tvd_scale =
            Self::tvd_positivity_scale(&self.scratch.averaged[..n], &self.scratch.h_next[..n])?;
        if trace_enabled {
            tvd_trace.scale = tvd_scale;
        }
        let mut clamp_injected_m2 = 0.0_f64;
        for i in 0..n {
            let h_new = self.scratch.averaged[i] + tvd_scale * self.scratch.h_next[i];
            if !h_new.is_finite() {
                return Err(RoutingError::NonFiniteState);
            }
            if h_new < -DRY_DEPTH_M {
                return Err(RoutingError::NegativeDepth);
            }
            if h_new < 0.0 {
                clamp_injected_m2 += (-h_new) * dx;
            }
            self.scratch.h_next[i] = h_new.max(0.0);
        }
        Ok((tvd_leak_m2, clamp_injected_m2, tvd_trace))
    }

    /// Advance one TVD-MacCormack step of size `dt` (eqs. 8-14). Returns the
    /// outlet sample, or a `RoutingError` on a fail-closed condition.
    ///
    /// Preconditions owned by the run loop (D14 OPT-1): `scratch.alpha`
    /// holds the pre-step per-cell alpha values and the rainfall intensity
    /// was validated before they were computed.
    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    fn step(
        &mut self,
        step_index: u64,
        time_s: f64,
        dt: f64,
        step_max_courant: f64,
        step_max_courant_cell_index: usize,
        q_up: f64,
        forcing: &Forcing<'_>,
        mass: &mut MassBalance,
        trace_enabled: bool,
    ) -> Result<(f64, f64, f64, Option<KinematicWaveStepTraceRecord>), RoutingError> {
        let n = self.mesh.cell_count();
        let dx = self.mesh.cell_length_m;
        let storage_before_m2 = if trace_enabled {
            self.total_storage_m2()
        } else {
            0.0
        };
        let stages = self.predictor_corrector_stages(time_s, dt, q_up, forcing, trace_enabled)?;
        let mut clamp_injected_m2 = stages.clamp_injected_m2;

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
        let (tvd_leak_m2, tvd_clamp_injected_m2, tvd_trace) =
            self.apply_tvd_stage(dt, trace_enabled)?;
        clamp_injected_m2 += tvd_clamp_injected_m2;

        // Commit state and recompute discharge (eq. 14).
        for i in 0..n {
            self.depth_m[i] = self.scratch.h_next[i];
            self.discharge_m2_s[i] = self.scratch.alpha[i] * depth_pow_3_2(self.depth_m[i]);
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
        // Outflow: predictor and corrector stage boundary faces after any
        // rev-41 conservative positivity limiting; the committed average
        // discharges their mean.
        let outflow_actual_m2 = 0.5 * (stages.pred_out_face + stages.corr_out_face) * dt;
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

        let step_trace = trace_enabled.then(|| KinematicWaveStepTraceRecord {
            step_index,
            t_start_s: time_s,
            t_end_s: time_s + dt,
            dt_s: dt,
            max_courant: step_max_courant,
            max_courant_cell_index: step_max_courant_cell_index,
            max_courant_cell_center_x_m: {
                #[allow(clippy::cast_precision_loss)]
                {
                    (step_max_courant_cell_index as f64 + 0.5) * dx
                }
            },
            q_up_m2_s: q_up,
            source_m2: self.scratch.v.iter().sum::<f64>() * dx * dt,
            upstream_inflow_m2: q_up * dt,
            outflow_m2: outflow_actual_m2,
            storage_before_m2,
            storage_after_m2: self.total_storage_m2(),
            clamp_injected_m2,
            pred_out_face_m2_s: stages.pred_out_face,
            corr_out_face_m2_s: stages.corr_out_face,
            outlet_depth_m: self.depth_m[n - 1],
            outlet_unit_discharge_m2_s: self.discharge_m2_s[n - 1],
            predictor_limiter: stages.predictor_limiter,
            corrector_limiter: stages.corrector_limiter,
            tvd: tvd_trace,
        });

        Ok((
            self.discharge_m2_s[n - 1],
            self.depth_m[n - 1],
            outflow_actual_m2,
            step_trace,
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
    #[allow(clippy::too_many_lines)]
    pub fn run_with_options(
        &mut self,
        forcing: &Forcing<'_>,
        upstream_integral_m2: Option<&dyn Fn(f64, f64) -> f64>,
        forcing_breakpoints_s: &[f64],
        end_time_s: f64,
        sample_dt_s: f64,
        max_dt_s: f64,
    ) -> Result<RoutingResult, RoutingError> {
        self.run_with_options_and_step_trace(
            forcing,
            upstream_integral_m2,
            forcing_breakpoints_s,
            end_time_s,
            sample_dt_s,
            max_dt_s,
            false,
        )
    }

    fn validate_run_configuration(
        &self,
        end_time_s: f64,
        sample_dt_s: f64,
        max_dt_s: f64,
    ) -> Result<(), RoutingError> {
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
        Ok(())
    }

    fn upstream_flux_for_step(
        forcing: &Forcing<'_>,
        upstream_integral_m2: Option<&dyn Fn(f64, f64) -> f64>,
        t: f64,
        dt: f64,
    ) -> Result<f64, RoutingError> {
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
        Ok(q_up)
    }

    /// Diagnostic variant of `run_with_options` that can retain a per-step
    /// trace for one externally selected active lane-day.
    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    pub fn run_with_options_and_step_trace(
        &mut self,
        forcing: &Forcing<'_>,
        upstream_integral_m2: Option<&dyn Fn(f64, f64) -> f64>,
        forcing_breakpoints_s: &[f64],
        end_time_s: f64,
        sample_dt_s: f64,
        max_dt_s: f64,
        trace_steps: bool,
    ) -> Result<RoutingResult, RoutingError> {
        self.validate_run_configuration(end_time_s, sample_dt_s, max_dt_s)?;
        let dx = self.mesh.cell_length_m;
        let storage_initial = self.total_storage_m2();
        self.max_tv_increase_m = 0.0;
        let mut mass = MassBalance::default();
        let mut recorder = BinRecorder::new(sample_dt_s, end_time_s);
        let mut t = 0.0_f64;
        let mut peak = 0.0_f64;
        let mut time_to_peak = 0.0_f64;
        let mut max_courant = 0.0_f64;
        let mut step_trace = trace_steps.then(Vec::new);

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
            let step_celerity = match self.prepare_step_alpha(rain_term) {
                Ok(value) => value,
                Err(err) => {
                    profile::end_solver_cfl(cfl_span);
                    return Err(err);
                }
            };
            let max_celerity = step_celerity.max_celerity;
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
            let step_max_courant = if max_celerity <= 0.0 {
                0.0
            } else {
                max_celerity * dt / dx
            };
            let step_max_courant_cell_index = if step_max_courant > 0.0 {
                step_celerity.max_cell_index
            } else {
                0
            };
            if !step_max_courant.is_finite() || step_max_courant > 1.0 + 1.0e-9 {
                return Err(RoutingError::CflViolation);
            }
            if step_max_courant > max_courant {
                max_courant = step_max_courant;
            }
            profile::end_solver_cfl(cfl_span);

            // Upstream boundary flux for this step: exact interval mean
            // when the integral closure is provided (rev 24), else the
            // point sample. Physically non-negative; fails closed.
            let q_up = Self::upstream_flux_for_step(forcing, upstream_integral_m2, t, dt)?;

            let t_before = t;
            let step_span = profile::span_start();
            let (_q_cell_out, h_out, outflow_step_m2, trace_record) = self.step(
                guard_steps,
                t,
                dt,
                step_max_courant,
                step_max_courant_cell_index,
                q_up,
                forcing,
                &mut mass,
                trace_steps,
            )?;
            profile::end_solver_step(step_span);
            profile::count_solver_steps(1);
            t += dt;
            if let (Some(records), Some(record)) = (&mut step_trace, trace_record) {
                records.push(record);
            }

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
        if terminal_deficit_m2 < 0.0 {
            // Medium-1: a material terminal deficit means the outflow
            // series cannot be represented as a non-negative exact-total
            // bin series. Publishing a negative outlet bin is not an
            // option, so the public path fails closed.
            return Err(RoutingError::NegativeOutletBin);
        }
        Ok(RoutingResult {
            hydrograph,
            mass_balance: mass,
            peak_unit_discharge_m2_s: peak,
            time_to_peak_s: time_to_peak,
            max_courant,
            max_homogeneous_tv_increase_m2_s: self.max_tv_increase_m,
            outlet_bin_outflow_m2,
            outlet_bin_dt_s: sample_dt_s,
            outlet_bin_spans_s,
            step_trace,
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

    #[test]
    fn rev47_depth_power_uses_sqrt_identity() {
        for h in [0.0_f64, 1.0e-12, 1.0e-6, 0.0125, 1.0] {
            assert_eq!(depth_pow_3_2(h).to_bits(), (h * h.sqrt()).to_bits());
        }
    }

    #[test]
    fn rev47_dust_residual_floor_matches_contract() {
        assert_eq!(ALPHA_NEWTON_ABS_TOL_M2_S.to_bits(), 1.0e-18_f64.to_bits());
    }

    #[test]
    fn rev47_dry_and_zero_slope_cells_are_zero_local_hydraulics() {
        let dry = CellParameters::bare(0.01, 500.0)
            .alpha_q_celerity(0.0, 0.0, 0.0)
            .expect("dry local numerics");
        let zero_slope = CellParameters::bare(0.0, 500.0)
            .alpha_q_celerity(0.01, 0.0, 0.0)
            .expect("zero-slope local numerics");

        for local in [dry, zero_slope] {
            assert_eq!(local.alpha.to_bits(), 0.0_f64.to_bits());
            assert_eq!(local.q.to_bits(), 0.0_f64.to_bits());
            assert_eq!(local.celerity.to_bits(), 0.0_f64.to_bits());
        }
    }

    #[test]
    fn rev47_manning_celerity_uses_five_thirds_q_over_h() {
        let cell = CellParameters::manning(0.02, 0.009);
        let h = 0.015;
        let local = cell
            .alpha_q_celerity(h, 0.0, 0.0)
            .expect("manning local numerics");

        let q_expected = 0.02_f64.sqrt() / 0.009 * h.powf(5.0 / 3.0);
        assert!((local.q - q_expected).abs() / q_expected < 1.0e-13);
        assert!((local.celerity - (5.0 / 3.0) * local.q / h).abs() / local.celerity < 1.0e-13);
    }

    #[test]
    fn rev47_laminar_skin_celerity_uses_three_q_over_h() {
        let cell = CellParameters::bare(0.01, 500.0);
        let h = 1.0e-4;
        let local = cell
            .alpha_q_celerity(h, 0.0, 0.0)
            .expect("laminar local numerics");
        let re = super::reynolds_number(local.q, KINEMATIC_VISCOSITY_M2_S);

        assert!(re <= super::SKIN_REGIME_REYNOLDS_THRESHOLD, "Re {re}");
        assert!((local.celerity - 3.0 * local.q / h).abs() / local.celerity < 1.0e-12);
    }

    #[test]
    fn rev47_hirsch_skin_celerity_uses_exact_turbulent_pow() {
        let cell = CellParameters::bare(0.5, 500.0);
        let h = 0.10;
        let local = cell
            .alpha_q_celerity(h, 0.0, 0.0)
            .expect("hirsch local numerics");
        let re = super::reynolds_number(local.q, KINEMATIC_VISCOSITY_M2_S);
        let expected_multiplier = DEPTH_DISCHARGE_EXPONENT / (1.0 - 0.5 * 0.45);
        let hirsch_factor = 3.19 * KINEMATIC_VISCOSITY_M2_S.powf(0.45);
        let exact_q = ((8.0 * GRAVITY_M_S2 * cell.slope / hirsch_factor).sqrt() * depth_pow_3_2(h))
            .powf(1.0 / 0.775);

        assert!(re > super::SKIN_REGIME_REYNOLDS_THRESHOLD, "Re {re}");
        assert!((local.q - exact_q).abs() / exact_q < 1.0e-13);
        assert!(
            (local.celerity - expected_multiplier * local.q / h).abs() / local.celerity < 1.0e-10,
            "celerity {} q {} h {} multiplier {}",
            local.celerity,
            local.q,
            h,
            expected_multiplier
        );
    }

    #[test]
    fn rev47_pure_skin_branch_gap_uses_pre_step_branch_without_smoothing() {
        let cell = CellParameters::bare(0.001, 10.0);
        let h = 0.006_309_573_444_801_93;
        let h_pow = depth_pow_3_2(h);
        let laminar_q = 8.0 * GRAVITY_M_S2 * cell.slope * h.powi(3)
            / (cell.friction_coefficient_ko * KINEMATIC_VISCOSITY_M2_S);
        let hirsch_factor = 3.19 * KINEMATIC_VISCOSITY_M2_S.powf(0.45);
        let hirsch_q =
            ((8.0 * GRAVITY_M_S2 * cell.slope / hirsch_factor).sqrt() * h_pow).powf(1.0 / 0.775);
        let laminar_re = super::reynolds_number(laminar_q, KINEMATIC_VISCOSITY_M2_S);
        let hirsch_re = super::reynolds_number(hirsch_q, KINEMATIC_VISCOSITY_M2_S);

        assert!(
            laminar_re > super::SKIN_REGIME_REYNOLDS_THRESHOLD,
            "laminar root Re {laminar_re}"
        );
        assert!(
            hirsch_re <= super::SKIN_REGIME_REYNOLDS_THRESHOLD,
            "Hirsch root Re {hirsch_re}"
        );

        let low_seed = 0.5 * super::SKIN_REGIME_REYNOLDS_THRESHOLD * KINEMATIC_VISCOSITY_M2_S;
        let low_branch = cell
            .alpha_q_celerity(h, low_seed, 0.0)
            .expect("laminar fallback branch");
        assert!((low_branch.q - laminar_q).abs() / laminar_q < 1.0e-13);
        assert!(
            (low_branch.celerity - 3.0 * low_branch.q / h).abs() / low_branch.celerity < 1.0e-13
        );

        let high_seed = 2.0 * super::SKIN_REGIME_REYNOLDS_THRESHOLD * KINEMATIC_VISCOSITY_M2_S;
        let high_branch = cell
            .alpha_q_celerity(h, high_seed, 0.0)
            .expect("Hirsch fallback branch");
        let expected_multiplier = DEPTH_DISCHARGE_EXPONENT / (1.0 - 0.5 * 0.45);
        assert!((high_branch.q - hirsch_q).abs() / hirsch_q < 1.0e-13);
        assert!(
            (high_branch.celerity - expected_multiplier * high_branch.q / h).abs()
                / high_branch.celerity
                < 1.0e-13
        );
    }

    #[test]
    fn rev47_additive_menu_celerity_matches_small_finite_difference() {
        let cell = CellParameters {
            slope: 0.04,
            friction_coefficient_ko: 80.0,
            drag_coefficient: 1.1,
            element_tip_height_m: 0.08,
            roughness_concentration: 0.25,
            leaf_area_index: 1.3,
            canopy_height_m: 0.45,
            vegetation_drag_coefficient: 0.9,
            manning_n: 0.0,
        };
        let h = 0.025;
        let rain_term = skin_rain_term(20.0 / 3.6e6);
        let local = cell
            .alpha_q_celerity(h, 0.0, rain_term)
            .expect("additive local numerics");
        let dh = 1.0e-6;
        let q2 = cell
            .alpha_q_celerity(h + dh, local.q, rain_term)
            .expect("perturbed local numerics")
            .q;
        let finite_diff = (q2 - local.q) / dh;
        let frozen_floor = DEPTH_DISCHARGE_EXPONENT * local.alpha * h.sqrt();
        let expected_celerity = finite_diff.max(frozen_floor);

        assert!(
            (expected_celerity - local.celerity).abs() / local.celerity < 2.0e-4,
            "analytic {} finite_diff {} frozen {} q {}",
            local.celerity,
            finite_diff,
            frozen_floor,
            local.q
        );
    }

    #[test]
    fn public_equivalent_friction_covers_manning_and_component_menu() {
        let h = 0.02_f64;
        let q = 4.0e-4_f64;
        let rain = 2.0e-5_f64;
        let manning = CellParameters::manning(0.05, 0.009);
        let expected = 8.0 * GRAVITY_M_S2 * 0.009_f64 * 0.009_f64 / h.cbrt();
        assert_eq!(
            manning.equivalent_friction(h, q, rain).to_bits(),
            expected.to_bits()
        );

        let mut components = CellParameters::bare(0.05, 120.0);
        components.drag_coefficient = 1.2;
        components.element_tip_height_m = 0.03;
        components.roughness_concentration = 0.15;
        components.leaf_area_index = 2.0;
        components.canopy_height_m = 0.4;
        components.vegetation_drag_coefficient = 0.8;
        let rain_term = skin_rain_term(rain);
        assert_eq!(
            components.equivalent_friction(h, q, rain).to_bits(),
            components
                .equivalent_friction_with_rain_term(h, q, rain_term)
                .to_bits()
        );
        assert!(components.equivalent_friction(h, q, rain) > 0.0);

        let submerged = components.equivalent_friction(0.04, q, rain);
        assert!(submerged.is_finite() && submerged > 0.0);
    }

    #[test]
    fn conservation_residual_books_positivity_clamp_after_raw_residual() {
        let mass = MassBalance {
            inflow_m2: 2.0,
            rainfall_excess_m2: 3.0,
            outflow_m2: 1.25,
            storage_change_m2: 3.5,
            positivity_clamp_m2: 0.25,
            ..MassBalance::default()
        };
        assert_eq!(mass.residual_m2().to_bits(), 0.25_f64.to_bits());
        assert_eq!(mass.conservation_residual_m2().to_bits(), 0.5_f64.to_bits());
    }

    #[test]
    fn stage_face_limiter_records_largest_reduction_and_rejects_nonfinite_availability() {
        let depth = [0.10, 0.05];
        let source = [0.0, 0.0];
        let mut faces = [0.0, 0.30, 0.20];
        let trace = KinematicWaveSolver::limit_stage_face_fluxes(
            &depth, &source, 1.0, 1.0, &mut faces, true,
        )
        .expect("finite stage");
        assert_eq!(faces[0].to_bits(), 0.0_f64.to_bits());
        assert_eq!(faces[1].to_bits(), 0.10_f64.to_bits());
        assert!((faces[2] - 0.15).abs() <= f64::EPSILON);
        assert_eq!(trace.reductions, 2);
        assert_eq!(trace.face_index, 1);
        assert_eq!(trace.face_x_m.to_bits(), 1.0_f64.to_bits());
        assert!((trace.max_reduction_m2_s - 0.20).abs() <= f64::EPSILON);

        let mut nonfinite_faces = [f64::INFINITY, 0.0];
        assert!(matches!(
            KinematicWaveSolver::limit_stage_face_fluxes(
                &[0.0],
                &[0.0],
                1.0,
                1.0,
                &mut nonfinite_faces,
                false,
            ),
            Err(RoutingError::NonFiniteState)
        ));
    }

    #[test]
    fn rev47_active_vegetation_nonfinite_local_numerics_fail_closed() {
        let cell = CellParameters {
            slope: 0.01,
            friction_coefficient_ko: 100.0,
            drag_coefficient: 0.0,
            element_tip_height_m: 0.0,
            roughness_concentration: 0.0,
            leaf_area_index: 1.0,
            canopy_height_m: 1.0,
            vegetation_drag_coefficient: 1.0e308,
            manning_n: 0.0,
        };

        assert!(matches!(
            cell.alpha_q_celerity(1.0, 1.0e-6, 0.0),
            Err(RoutingError::NonFiniteState)
        ));
    }

    #[test]
    fn post_tier1_prepare_step_alpha_retains_scan_max_celerity() {
        let dx = 2.0;
        let cells = vec![
            CellParameters::manning(0.01, 0.03),
            CellParameters::manning(0.04, 0.03),
            CellParameters::manning(0.04, 0.03),
            CellParameters::manning(0.01, 0.03),
        ];
        let mesh = KinematicWaveMesh {
            cell_length_m: dx,
            cells,
        };
        let mut solver = KinematicWaveSolver::new(mesh);
        solver.depth_m = vec![0.02, 0.02, 0.02, 0.02];
        solver.discharge_m2_s = vec![0.0; 4];

        let summary = solver.prepare_step_alpha(0.0).expect("prepare alpha");
        let scan = solver
            .scratch
            .celerity
            .iter()
            .enumerate()
            .filter(|(i, _)| solver.depth_m[*i] > DRY_DEPTH_M)
            .fold((0_usize, 0.0_f64), |best, (i, celerity)| {
                if *celerity > best.1 {
                    (i, *celerity)
                } else {
                    best
                }
            });

        assert_eq!(summary.max_cell_index, scan.0);
        assert_eq!(summary.max_cell_index, 1);
        assert_eq!(summary.max_celerity.to_bits(), scan.1.to_bits());

        let dt = 0.5;
        let retained_courant = summary.max_celerity * dt / dx;
        let scan_courant = scan.1 * dt / dx;
        assert_eq!(retained_courant.to_bits(), scan_courant.to_bits());
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

        // conservation: all lateral input eventually leaves or is stored.
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
        // Secondary: the scheme conserves (INV-OFEROUTE-006), independent of
        // dt/slope.
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
    fn source_quiet_dry_front_outlet_flux_stays_nonnegative_and_conservative() {
        let params = CellParameters::bare(0.05, 100.0);
        let penultimate_depth_m = 1.0e-4;
        let outlet_depth_m = 1.0e-6;
        let penultimate_local = params
            .alpha_q_celerity(penultimate_depth_m, 0.0, 0.0)
            .expect("finite penultimate-cell local hydraulics");
        let outlet_local = params
            .alpha_q_celerity(outlet_depth_m, 0.0, 0.0)
            .expect("finite near-dry outlet local hydraulics");
        assert!(penultimate_local.q > 0.0);
        assert!(outlet_local.q > 0.0);
        let raw_predictor_outlet_m2_s = 2.0 * outlet_local.q - penultimate_local.q;
        assert!(raw_predictor_outlet_m2_s < 0.0);
        let mesh = KinematicWaveMesh::uniform(20.0, 2, params);
        let mut solver = KinematicWaveSolver::new(mesh);
        // SC-OFEROUTE-001 rev 51 / LANED-NOB-001: a wet penultimate cell
        // followed by a near-dry but positive outlet makes the raw predictor
        // donor extrapolation `2 q[n-1] - q[n-2]` negative. The one-way outlet
        // face must enforce its exact zero lower bound inside the conservative
        // update; it must not alias the positive committed outlet discharge or
        // depend on borrowing mass from a later outlet bin.
        solver.depth_m[0] = penultimate_depth_m;
        solver.discharge_m2_s[0] = penultimate_local.q;
        solver.depth_m[1] = outlet_depth_m;
        solver.discharge_m2_s[1] = outlet_local.q;
        let initial_storage_m2 = solver.total_storage_m2();

        let excess = |_i: usize, _t: f64| 0.0;
        let inflow = |_t: f64| 0.0;
        let intensity = |_t: f64| 0.0;
        let forcing = Forcing {
            rainfall_excess_m_s: &excess,
            upstream_inflow_m2_s: &inflow,
            rainfall_intensity_m_s: &intensity,
        };

        let result = solver
            .run_with_options_and_step_trace(&forcing, None, &[], 1.0, 1.0, 1.0, true)
            .expect("rev-51 source-quiet dry front must complete");
        let step_trace = result.step_trace.as_ref().expect("step trace retained");
        assert!(!step_trace.is_empty());
        assert_eq!(
            step_trace[0].pred_out_face_m2_s.to_bits(),
            0.0_f64.to_bits(),
            "raw-negative predictor must use the exact positive-zero boundary face"
        );
        assert_ne!(
            step_trace[0].pred_out_face_m2_s.to_bits(),
            outlet_local.q.to_bits(),
            "scheme face must not alias the positive committed outlet discharge"
        );
        assert!(step_trace.iter().all(|record| {
            record.pred_out_face_m2_s.is_finite()
                && record.pred_out_face_m2_s >= 0.0
                && record.corr_out_face_m2_s.is_finite()
                && record.corr_out_face_m2_s >= 0.0
        }));
        assert!(result.mass_balance.outflow_m2 >= 0.0);
        assert!(
            result
                .outlet_bin_outflow_m2
                .iter()
                .all(|value| *value >= 0.0)
        );
        assert!(
            result
                .hydrograph
                .iter()
                .all(|sample| sample.outlet_unit_discharge_m2_s >= 0.0)
        );
        let bin_sum_m2: f64 = result.outlet_bin_outflow_m2.iter().sum();
        assert_eq!(
            bin_sum_m2.to_bits(),
            result.mass_balance.outflow_m2.to_bits(),
            "outlet bins must equal the independently booked scheme outflow"
        );
        assert!(result.mass_balance.positivity_clamp_m2 <= 1.0e-18);

        // Anti-tautology: reconstruct storage change from committed cell
        // depths rather than reusing the mass ledger's storage-change field.
        let committed_storage_m2 = solver.total_storage_m2();
        let reconstructed_storage_change_m2 = committed_storage_m2 - initial_storage_m2;
        let reconstructed_residual_m2 = result.mass_balance.inflow_m2
            + result.mass_balance.rainfall_excess_m2
            + result.mass_balance.positivity_clamp_m2
            - result.mass_balance.outflow_m2
            - reconstructed_storage_change_m2;
        assert!(
            reconstructed_residual_m2.abs() <= 1.0e-15,
            "independent dry-front closure residual {reconstructed_residual_m2}"
        );
    }

    #[test]
    fn bin_recorder_retains_material_terminal_deficit_signal() {
        let mut recorder = BinRecorder::new(1.0, 1.0);
        recorder.record_step(0.0, 1.0, -1.0e-4, 0.0);
        let (_hydrograph, bins, _spans, terminal_deficit) = recorder.finish();

        assert_eq!(terminal_deficit.to_bits(), (-1.0e-4_f64).to_bits());
        assert!(bins.iter().all(|value| *value >= 0.0));
    }

    #[test]
    fn stage_flux_limiter_prevents_positive_clamp_injection() {
        let mesh = KinematicWaveMesh::uniform(3.0, 3, CellParameters::bare(0.05, 100.0));
        let mut solver = KinematicWaveSolver::new(mesh);
        solver.depth_m.copy_from_slice(&[1.0e-6, 1.0e-6, 1.0e-6]);
        solver
            .discharge_m2_s
            .copy_from_slice(&[0.0, 1.0e-3, 1.0e-3]);
        solver.scratch.alpha.fill(0.0);
        solver.scratch.celerity.fill(0.0);

        let excess = |_i: usize, _t: f64| 0.0;
        let inflow = |_t: f64| 0.0;
        let intensity = |_t: f64| 0.0;
        let forcing = Forcing {
            rainfall_excess_m_s: &excess,
            upstream_inflow_m2_s: &inflow,
            rainfall_intensity_m_s: &intensity,
        };
        let mut mass = MassBalance::default();
        let storage_initial = solver.total_storage_m2();

        solver
            .step(0, 0.0, 1.0, 0.0, 0, 0.0, &forcing, &mut mass, false)
            .expect("limited step");
        mass.storage_change_m2 = solver.total_storage_m2() - storage_initial;

        assert!(
            mass.positivity_clamp_m2 <= 1.0e-18,
            "roundoff-only clamp {}",
            mass.positivity_clamp_m2
        );
        assert!(
            solver.depth_m.iter().all(|h| *h >= 0.0),
            "limited stage must not publish negative depth"
        );
        assert!(
            mass.residual_m2().abs() <= 1.0e-15,
            "conservative limiter residual {}",
            mass.residual_m2()
        );
    }

    #[test]
    fn final_tvd_scaling_preserves_positivity_and_total() {
        let averaged = [0.10, 0.10, 0.10];
        // Telescoping TVD cell deltas: sum is zero, but full strength would
        // drive cell 0 negative.
        let tvd_delta = [-0.20, 0.05, 0.15];

        let scale =
            KinematicWaveSolver::tvd_positivity_scale(&averaged, &tvd_delta).expect("scale finite");
        assert!((scale - 0.5).abs() <= f64::EPSILON);

        let committed = [
            averaged[0] + scale * tvd_delta[0],
            averaged[1] + scale * tvd_delta[1],
            averaged[2] + scale * tvd_delta[2],
        ];
        assert!(
            committed.iter().all(|h| *h >= 0.0),
            "scaled TVD update must be non-negative: {committed:?}"
        );
        let before = averaged.iter().sum::<f64>();
        let after = committed.iter().sum::<f64>();
        assert!(
            (after - before).abs() <= f64::EPSILON,
            "scaled telescoping correction must preserve total: before {before}, after {after}"
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
        // Discretization-only (no material clamp mass at either resolution)
        // and machine-scale closure after the conservative stage limiter.
        assert_eq!(clamp_coarse.to_bits(), 0.0_f64.to_bits());
        assert_eq!(clamp_fine.to_bits(), 0.0_f64.to_bits());
        assert!(
            r_coarse < 1.0e-12 && r_fine < 1.0e-12,
            "residuals should be machine-scale: coarse {r_coarse}, fine {r_fine}"
        );
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
