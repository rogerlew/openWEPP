//! MOFEFID Lane D (SC-OFEROUTE-001, ADR-0033 ratified): space/time-variant
//! overland-flow friction-factor kernels from Papanicolaou et al. (2018),
//! WRR 54, eqs. (2)-(7). Pure functions over typed SI inputs; shadow-first
//! (not wired into any phase span). Opt-in subsystem — the default hillslope
//! runtime does not call these.
//!
//! Primary-source provenance under the frozen reference library
//! (2026-07-01): the eq. (2)-(3) constants (Shen & Li 1973; Hirsch 1996) and
//! the eq. (4) applicability bounds (Abrahams 1998 discussion) are cited
//! SECONDARY via R-63 (Papanicolaou 2018) per the campaign's citation rule;
//! eq. (4) form (Lawrence 1997, R-77), eq. (5) wave (Hu & Abrahams 2006,
//! R-72), and eq. (6) vegetation (Katul et al. 2011, R-78) are primary in
//! hand. The local SI unit convention for the Shen & Li intensity term is
//! confirmed against R-63's stated equation by D8 low-`k_o` regression tests;
//! primary-source coefficient provenance remains frozen-library.

/// Kinematic viscosity of water at ~15 C (m^2 s^-1). Papanicolaou states the
/// `Re = q / nu` unit relation; the exact representative value remains
/// frozen-library provenance.
pub const KINEMATIC_VISCOSITY_M2_S: f64 = 1.14e-6;
/// Gravitational acceleration (m s^-2).
pub const GRAVITY_M_S2: f64 = 9.81;

/// Reynolds number `Re = q / nu` (eq. context): unit discharge over
/// kinematic viscosity. `q` in m^2 s^-1 (unit-width discharge).
#[must_use]
pub fn reynolds_number(unit_discharge_m2_s: f64, kinematic_viscosity_m2_s: f64) -> f64 {
    unit_discharge_m2_s / kinematic_viscosity_m2_s
}

/// Froude number `Fr = q / sqrt(g h^3)` (eq. (5) context). `h` flow depth (m).
#[must_use]
pub fn froude_number(unit_discharge_m2_s: f64, flow_depth_m: f64, gravity_m_s2: f64) -> f64 {
    if flow_depth_m <= 0.0 {
        return 0.0;
    }
    unit_discharge_m2_s / (gravity_m_s2 * flow_depth_m.powi(3)).sqrt()
}

/// Skin (grain + raindrop) resistance, laminar regime `Re < 1000`
/// (Papanicolaou eq. (2); Shen & Li 1973 via R-63):
/// `f_s = (3393 * I^0.407 + k_o) / Re`, with rainfall intensity `I` in m s^-1.
/// Active routing callers validate `I >= 0` before this pure equation helper is
/// called; invalid negative values are not normalized here.
#[must_use]
pub fn skin_resistance_shen_li(
    rainfall_intensity_m_s: f64,
    friction_coefficient_ko: f64,
    reynolds_number: f64,
) -> f64 {
    if reynolds_number <= 0.0 {
        return 0.0;
    }
    (3393.0 * rainfall_intensity_m_s.powf(0.407) + friction_coefficient_ko) / reynolds_number
}

/// Skin resistance, turbulent regime `Re > 1000` (eq. (3); Hirsch 1996 via
/// R-63): `f_s = 3.19 / Re^0.45`.
#[must_use]
pub fn skin_resistance_hirsch(reynolds_number: f64) -> f64 {
    if reynolds_number <= 0.0 {
        return 0.0;
    }
    3.19 / reynolds_number.powf(0.45)
}

/// Regime threshold between eq. (2) and eq. (3). Papanicolaou states the two
/// laboratory-derived branches; the crossover is at `Re ~ 1000` (the paper's
/// stated regime split). Below/at the threshold uses Shen & Li, above uses
/// Hirsch.
pub const SKIN_REGIME_REYNOLDS_THRESHOLD: f64 = 1000.0;

/// Skin resistance with regime dispatch by Reynolds number.
#[must_use]
pub fn skin_resistance(
    rainfall_intensity_m_s: f64,
    friction_coefficient_ko: f64,
    reynolds_number: f64,
) -> f64 {
    if reynolds_number > SKIN_REGIME_REYNOLDS_THRESHOLD {
        skin_resistance_hirsch(reynolds_number)
    } else {
        skin_resistance_shen_li(
            rainfall_intensity_m_s,
            friction_coefficient_ko,
            reynolds_number,
        )
    }
}

/// Precomputed Shen & Li rainfall numerator term `3393 * I^0.407` (the
/// rain-dependent part of eq. (2)). D14 OPT-3: `I` is constant within one
/// solver step, so the hot path computes this once per step and reuses it
/// across cells and fixed-point iterations. Bit-identical to the inline
/// evaluation in `skin_resistance_shen_li` (same operations in the same
/// order; see `skin_rain_term_dispatch_is_bit_identical`).
#[must_use]
pub fn skin_rain_term(rainfall_intensity_m_s: f64) -> f64 {
    3393.0 * rainfall_intensity_m_s.powf(0.407)
}

/// Skin resistance with regime dispatch, consuming a precomputed
/// `skin_rain_term`. Must remain bit-identical to `skin_resistance`
/// (unit-enforced).
#[must_use]
pub fn skin_resistance_with_rain_term(
    skin_rain_term: f64,
    friction_coefficient_ko: f64,
    reynolds_number: f64,
) -> f64 {
    if reynolds_number > SKIN_REGIME_REYNOLDS_THRESHOLD {
        skin_resistance_hirsch(reynolds_number)
    } else if reynolds_number <= 0.0 {
        0.0
    } else {
        (skin_rain_term + friction_coefficient_ko) / reynolds_number
    }
}

/// Form (isolated-roughness-element) resistance (eq. (4); Abrahams 1998 /
/// Lawrence 1997, R-77): `f_f = (16/pi) * C_d * (h / D_r) * lambda`, where
/// `D_r` is the element tip height (m) and `lambda` the roughness
/// concentration (0-1). Physical only while the flow is submerged relative to
/// the element (`h/D_r <= 1` introduces wave resistance, eq. (5)).
#[must_use]
pub fn form_resistance_abrahams(
    drag_coefficient: f64,
    flow_depth_m: f64,
    element_tip_height_m: f64,
    roughness_concentration: f64,
) -> f64 {
    if element_tip_height_m <= 0.0 {
        return 0.0;
    }
    (16.0 / std::f64::consts::PI)
        * drag_coefficient
        * (flow_depth_m / element_tip_height_m)
        * roughness_concentration
}

/// Froude threshold below which wave resistance ramps linearly to its
/// `Fr = 0.5` value (Papanicolaou eq. (5); Abrahams & Parsons 1994).
pub const WAVE_FROUDE_THRESHOLD: f64 = 0.5;

/// Wave resistance (eq. (5); Hu & Abrahams 2006, R-72): for `Fr > 0.5`,
/// `f_w = 3.32 * lambda / Fr^0.5`. For `Fr <= 0.5`, effects increase
/// proportionally from 0 to the `Fr = 0.5` maximum (linear ramp in Fr).
#[must_use]
pub fn wave_resistance_hu_abrahams(roughness_concentration: f64, froude_number: f64) -> f64 {
    if froude_number <= 0.0 {
        return 0.0;
    }
    let value_at_threshold = 3.32 * roughness_concentration / WAVE_FROUDE_THRESHOLD.sqrt();
    if froude_number > WAVE_FROUDE_THRESHOLD {
        3.32 * roughness_concentration / froude_number.sqrt()
    } else {
        // Linear ramp 0 -> value_at_threshold across Fr in (0, 0.5].
        value_at_threshold * (froude_number / WAVE_FROUDE_THRESHOLD)
    }
}

/// Momentum absorption coefficient `beta = min(0.135 * sqrt(LAI / h_c), 0.33)`
/// (Papanicolaou eq. (6) definition; Katul et al. 2011, R-78).
#[must_use]
pub fn vegetation_momentum_absorption(leaf_area_index: f64, canopy_height_m: f64) -> f64 {
    if canopy_height_m <= 0.0 {
        return 0.33;
    }
    (0.135 * (leaf_area_index / canopy_height_m).sqrt()).min(0.33)
}

/// Vegetation adjustment length scale `L_c = (C_d * LAI / h_c)^-1` (eq. (6)).
#[must_use]
pub fn vegetation_length_scale(
    drag_coefficient: f64,
    leaf_area_index: f64,
    canopy_height_m: f64,
) -> f64 {
    let denom = drag_coefficient * leaf_area_index / canopy_height_m;
    if denom <= 0.0 {
        return 0.0;
    }
    1.0 / denom
}

/// Vegetation (canopy) resistance (eq. (6); Katul et al. 2011, R-78 /
/// Thompson et al. 2011, R-80):
/// `f_veg = sqrt( 2*beta*(L_c/h) * exp(-h_c/(2*beta^2*L_c)) * (-1 + exp((1/(2*beta^2))*(h/L_c))) )`.
#[must_use]
pub fn vegetation_resistance_katul(
    drag_coefficient: f64,
    leaf_area_index: f64,
    canopy_height_m: f64,
    flow_depth_m: f64,
) -> f64 {
    if flow_depth_m <= 0.0 || canopy_height_m <= 0.0 || leaf_area_index <= 0.0 {
        return 0.0;
    }
    let beta = vegetation_momentum_absorption(leaf_area_index, canopy_height_m);
    let l_c = vegetation_length_scale(drag_coefficient, leaf_area_index, canopy_height_m);
    if l_c <= 0.0 || beta <= 0.0 {
        return 0.0;
    }
    let two_beta_sq = 2.0 * beta * beta;
    let term_exp_neg = (-canopy_height_m / (two_beta_sq * l_c)).exp();
    let term_exp_pos = ((1.0 / two_beta_sq) * (flow_depth_m / l_c)).exp();
    let radicand = 2.0 * beta * (l_c / flow_depth_m) * term_exp_neg * (-1.0 + term_exp_pos);
    if radicand <= 0.0 {
        return 0.0;
    }
    radicand.sqrt()
}

/// Additive equivalent friction factor (eq. (7)):
/// `f_eq = f_s + f_f + f_w + f_veg`.
#[must_use]
pub fn equivalent_friction_factor(skin: f64, form: f64, wave: f64, vegetation: f64) -> f64 {
    skin + form + wave + vegetation
}

/// Chezy coefficient from the equivalent Darcy-Weisbach friction factor
/// (Appendix A): `C = sqrt(8 g / f_eq)`.
#[must_use]
pub fn chezy_from_friction(equivalent_friction_factor: f64, gravity_m_s2: f64) -> f64 {
    if equivalent_friction_factor <= 0.0 {
        return 0.0;
    }
    (8.0 * gravity_m_s2 / equivalent_friction_factor).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() <= tol
    }

    #[test]
    fn skin_regime_dispatch_and_continuity() {
        // Shen & Li below threshold, Hirsch above; both positive and decreasing in Re.
        let ko = 500.0;
        let i = 60.0 / 3.6e6; // 60 mm/h -> m/s (Case 1)
        let re_lam = 500.0;
        let re_turb = 2000.0;
        let f_lam = skin_resistance(i, ko, re_lam);
        let f_turb = skin_resistance(i, ko, re_turb);
        assert!(f_lam > 0.0 && f_turb > 0.0);
        // laminar branch decreases with Re
        assert!(skin_resistance_shen_li(i, ko, 600.0) < skin_resistance_shen_li(i, ko, 500.0));
        // hand value: Shen & Li at Re=500, I=60mm/h
        let expected = (3393.0 * i.powf(0.407) + ko) / 500.0;
        assert!(approx(f_lam, expected, 1e-12));
        // Hirsch at Re=2000
        assert!(approx(f_turb, 3.19 / 2000.0_f64.powf(0.45), 1e-12));
    }

    #[test]
    fn shen_li_low_ko_vector_pins_si_rainfall_intensity() {
        // D8-1: exercise a rain-driven, low-k_o regime where the I term is not
        // hidden by k_o. R-63 states I in m/s; passing 100 mm/h as the raw
        // number would inflate the numerator by >400x and fail this test.
        let i_m_s = 100.0 / 3.6e6;
        let ko = 1.0;
        let re = 100.0;
        let got = skin_resistance_shen_li(i_m_s, ko, re);
        let expected = (3393.0 * i_m_s.powf(0.407) + ko) / re;
        assert!(approx(got, expected, 1e-14));

        let wrong_mmh = (3393.0 * 100.0_f64.powf(0.407) + ko) / re;
        assert!(
            (wrong_mmh / got) > 400.0,
            "unit regression must distinguish m/s from raw mm/h: got {got}, wrong {wrong_mmh}"
        );
    }

    #[test]
    fn shen_li_negative_intensity_is_not_silently_zeroed() {
        // Active callers fail closed before negative forcing reaches the pure
        // equation helper. The helper itself must not normalize a negative
        // rainfall intensity to zero because that would mask an invalid
        // forcing path.
        assert!(skin_resistance_shen_li(-1.0e-6, 1.0, 100.0).is_nan());
    }

    #[test]
    fn form_resistance_matches_equation_case2() {
        // Case 2 params: C_d=1.0, D_r=0.06 m, lambda=0.2; pick h=0.03 (h/D_r=0.5).
        let f = form_resistance_abrahams(1.0, 0.03, 0.06, 0.2);
        let expected = (16.0 / std::f64::consts::PI) * 1.0 * (0.03 / 0.06) * 0.2;
        assert!(approx(f, expected, 1e-12));
        assert!(f > 0.0);
        // increases with submergence h/D_r
        assert!(form_resistance_abrahams(1.0, 0.06, 0.06, 0.2) > f);
    }

    #[test]
    fn wave_resistance_ramps_below_half_and_decays_above() {
        let lambda = 0.2;
        let at_half = wave_resistance_hu_abrahams(lambda, 0.5);
        let expected_half = 3.32 * lambda / 0.5_f64.sqrt();
        assert!(approx(at_half, expected_half, 1e-12));
        // below 0.5: linear ramp -> half the value at Fr=0.25
        assert!(approx(
            wave_resistance_hu_abrahams(lambda, 0.25),
            expected_half * 0.5,
            1e-12
        ));
        // above 0.5: decays with Fr
        assert!(wave_resistance_hu_abrahams(lambda, 1.0) < at_half);
        assert!(approx(
            wave_resistance_hu_abrahams(lambda, 1.0),
            3.32 * lambda / 1.0,
            1e-12
        ));
    }

    #[test]
    fn vegetation_beta_is_capped_and_resistance_positive() {
        // Case 3 params: LAI=1, h_c=0.1 m, C_d=1.0.
        let beta = vegetation_momentum_absorption(1.0, 0.1);
        assert!(beta <= 0.33 && beta > 0.0);
        // cap engages for large LAI/h_c
        assert!(approx(
            vegetation_momentum_absorption(100.0, 0.1),
            0.33,
            1e-12
        ));
        let f = vegetation_resistance_katul(1.0, 1.0, 0.1, 0.02);
        assert!(
            f > 0.0,
            "vegetation resistance should be positive for submerged canopy flow"
        );
        // deeper flow (more canopy interaction) increases resistance
        assert!(vegetation_resistance_katul(1.0, 1.0, 0.1, 0.05) > f);
    }

    #[test]
    fn additive_equivalent_and_chezy() {
        let f_eq = equivalent_friction_factor(0.5, 0.3, 0.2, 0.4);
        assert!(approx(f_eq, 1.4, 1e-12));
        let c = chezy_from_friction(f_eq, GRAVITY_M_S2);
        assert!(approx(c, (8.0 * GRAVITY_M_S2 / 1.4).sqrt(), 1e-12));
        // higher friction -> lower Chezy conveyance
        assert!(chezy_from_friction(2.0, GRAVITY_M_S2) < c);
    }

    // D14 OPT-3 bit-identity contract: the precomputed-rain-term dispatch
    // must be indistinguishable from the canonical `skin_resistance` at the
    // bit level for every regime branch, including NaN propagation for
    // negative intensity (guarded upstream in active callers).
    #[test]
    fn skin_rain_term_dispatch_is_bit_identical() {
        let intensities = [
            0.0,
            1.0e-9,
            60.0 / 3.6e6,
            100.0 / 3.6e6,
            0.5,
            -1.0e-6, // NaN path: both forms must produce NaN identically
        ];
        let kos = [0.0, 0.25, 1.0, 500.0, 10_000.0];
        let res = [
            f64::NAN,
            -1.0,
            0.0,
            1.0e-6,
            1.0,
            500.0,
            999.9,
            1000.0,
            1000.1,
            2.0e6,
        ];
        for &i in &intensities {
            let term = skin_rain_term(i);
            for &ko in &kos {
                for &re in &res {
                    let canonical = skin_resistance(i, ko, re);
                    let hoisted = skin_resistance_with_rain_term(term, ko, re);
                    assert_eq!(
                        canonical.to_bits(),
                        hoisted.to_bits(),
                        "bit mismatch at I={i}, ko={ko}, Re={re}: canonical {canonical} vs hoisted {hoisted}"
                    );
                }
            }
        }
    }

    #[test]
    fn degenerate_inputs_return_zero_not_nan() {
        let z = 0.0_f64.to_bits();
        assert_eq!(reynolds_number(0.0, KINEMATIC_VISCOSITY_M2_S).to_bits(), z);
        assert_eq!(froude_number(0.1, 0.0, GRAVITY_M_S2).to_bits(), z);
        assert_eq!(skin_resistance(0.0, 500.0, 0.0).to_bits(), z);
        assert_eq!(form_resistance_abrahams(1.0, 0.03, 0.0, 0.2).to_bits(), z);
        assert_eq!(wave_resistance_hu_abrahams(0.2, 0.0).to_bits(), z);
        assert_eq!(vegetation_resistance_katul(1.0, 1.0, 0.1, 0.0).to_bits(), z);
    }
}
