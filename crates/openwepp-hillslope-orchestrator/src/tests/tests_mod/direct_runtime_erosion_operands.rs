//! Wave-1 event/transport operand producer tests (erosion port
//! Increment-1b-A, SC-SED-001). Each producer is checked against the
//! hand-evaluated legacy equation on the McKenzie clay-loam texture: the
//! `forest_high_severity_clay_loam` fixture (`sand = 0.25`,
//! `clay = 0.30`, `silt = 0.45`, `orgmat = 0.05`, `ki = 1.5e6`,
//! `kr = 6e-5`, `shcrit = 0.5`).
#![allow(clippy::doc_markdown)]

use crate::{
    DirectRuntimeError, EROSION_PARTICLE_CLASS_COUNT, ErosionExcessInterval,
    ErosionRillCoverInputs, ErosionShearSlopes, ErosionTextureInputs, erosion_detinr,
    erosion_effective_intensity, erosion_effective_particle, erosion_falvel,
    erosion_interrill_delivery_ratio, erosion_particle_composition, erosion_rill_hydraulics,
    erosion_shield, erosion_transport_coefficients, erosion_trcoef, erosion_yalin,
    erosion_yalin_with_class_shares,
};

const ACCGAV: f64 = 9.807;
const KINVIS: f64 = 1.0e-6;

fn clay_loam_texture() -> ErosionTextureInputs {
    ErosionTextureInputs {
        sand: 0.25,
        clay: 0.30,
        silt: 0.45,
        orgmat: 0.05,
    }
}

#[test]
fn falvel_stokes_branch_matches_closed_form_for_small_particles() {
    // Primary clay: dia = 2e-6 m, spg = 2.60. rtsid = (spg-1)*g*dia^3/
    // kinvis^2 * 8/6 = 1.6*9.807*(8e-18)/1e-12*1.3333 ~ 1.67e-4 < 0.024,
    // so falvel = dia^2*(spg-1)*g/(kinvis*18) exactly.
    let dia = 2.0e-6;
    let spg = 2.60;
    let expected = dia * dia * (spg - 1.0) * ACCGAV / (KINVIS * 18.0);
    let observed = erosion_falvel(spg, dia);
    assert!(
        (observed - expected).abs() <= 1.0e-15,
        "Stokes-branch fall velocity must match the closed form: \
         expected {expected}, observed {observed}"
    );
}

#[test]
fn falvel_table_branch_is_positive_and_monotone_in_diameter() {
    // Larger particles use the drag-table interpolation; fall velocity
    // must be finite, positive, and increase with diameter.
    let small = erosion_falvel(2.65, 5.0e-5);
    let large = erosion_falvel(2.65, 2.0e-4);
    assert!(small > 0.0 && small.is_finite());
    assert!(large > small, "coarser sand must settle faster");
}

#[test]
fn particle_composition_sums_to_unity_and_orders_diameters() {
    let classes = erosion_particle_composition(&clay_loam_texture())
        .expect("clay-loam composition must resolve");
    assert_eq!(classes.len(), EROSION_PARTICLE_CLASS_COUNT);
    let frac_sum: f64 = classes.iter().map(|c| c.frac).sum();
    assert!(
        (frac_sum - 1.0).abs() < 1.0e-6,
        "class fractions must sum to 1, observed {frac_sum}"
    );
    // Class 1 (primary clay) = 0.26*clay (`prtcmp.for:134`).
    assert!((classes[0].frac - 0.26 * 0.30).abs() < 1.0e-9);
    // Class 1 diameter is the finest, class 4 (large aggregate) coarsest.
    assert!(classes[0].dia_m < classes[1].dia_m);
    assert!(classes[3].dia_m >= classes[2].dia_m);
    for class in &classes {
        assert!(class.frac >= 0.0 && class.fall_m_s > 0.0);
    }
}

#[test]
fn particle_composition_fails_closed_on_invalid_texture() {
    let bad = ErosionTextureInputs {
        sand: 1.5,
        clay: 0.3,
        silt: -0.8,
        orgmat: 0.0,
    };
    assert!(matches!(
        erosion_particle_composition(&bad),
        Err(DirectRuntimeError::DirectDomainViolation { .. })
    ));
}

#[test]
fn hb02_d_f_h_texture_mass_fractions_fail_closed() {
    for (field, value) in [
        ("sand", f64::NAN),
        ("sand", f64::INFINITY),
        ("sand", f64::NEG_INFINITY),
        ("clay", f64::NAN),
        ("clay", f64::INFINITY),
        ("clay", f64::NEG_INFINITY),
        ("silt", f64::NAN),
        ("silt", f64::INFINITY),
        ("silt", f64::NEG_INFINITY),
        ("orgmat", f64::NAN),
        ("orgmat", f64::INFINITY),
        ("orgmat", f64::NEG_INFINITY),
    ] {
        let mut texture = clay_loam_texture();
        match field {
            "sand" => texture.sand = value,
            "clay" => texture.clay = value,
            "silt" => texture.silt = value,
            "orgmat" => texture.orgmat = value,
            _ => unreachable!(),
        }
        assert!(
            erosion_particle_composition(&texture).is_err(),
            "{field}={value} must fail closed"
        );
    }

    for field in ["sand", "clay", "silt", "orgmat"] {
        for value in [-0.1, 1.1] {
            let mut texture = clay_loam_texture();
            match field {
                "sand" => texture.sand = value,
                "clay" => texture.clay = value,
                "silt" => texture.silt = value,
                "orgmat" => texture.orgmat = value,
                _ => unreachable!(),
            }
            assert_eq!(
                erosion_particle_composition(&texture),
                Err(DirectRuntimeError::DirectDomainViolation {
                    field: "erosion.prtcmp.texture"
                })
            );
        }
    }
}

#[test]
fn hb02_b_texture_individual_zero_and_one_boundaries_are_admitted() {
    for field in ["sand", "clay", "silt", "orgmat"] {
        for value in [0.0, 1.0] {
            let mut texture = clay_loam_texture();
            match field {
                "sand" => texture.sand = value,
                "clay" => texture.clay = value,
                "silt" => texture.silt = value,
                "orgmat" => texture.orgmat = value,
                _ => unreachable!(),
            }
            let classes = erosion_particle_composition(&texture)
                .unwrap_or_else(|error| panic!("{field}={value} boundary failed: {error}"));
            assert!(classes.iter().all(|class| class.frac.is_finite()));
        }
    }
}

#[test]
fn hb02_b_c_clay_band_boundaries_preserve_legacy_diameters_and_class_order() {
    let cases = [
        (0.0, 0.030e-3, 0.300e-3),
        (0.15, 0.030e-3, 0.300e-3),
        (0.150_000_001, 0.030e-3, 0.300_000_002e-3),
        (0.25, 0.030e-3, 0.500e-3),
        (
            0.250_000_001,
            (0.20 * 0.000_000_001 + 0.030) / 1000.0,
            0.500_000_002e-3,
        ),
        (0.50, 0.080e-3, 1.000e-3),
        (0.60, 0.100e-3, 1.200e-3),
        (1.0, 0.180e-3, 2.000e-3),
    ];
    for (clay, expected_small_aggregate_dia, expected_large_aggregate_dia) in cases {
        let classes = erosion_particle_composition(&ErosionTextureInputs {
            clay,
            ..clay_loam_texture()
        })
        .expect("clay-band boundary must resolve");
        assert!((classes[2].dia_m - expected_small_aggregate_dia).abs() <= 1.0e-15);
        assert!((classes[3].dia_m - expected_large_aggregate_dia).abs() <= 1.0e-15);
        assert_eq!(
            classes.map(|class| class.spg),
            [2.60, 2.65, 1.80, 1.60, 2.65]
        );
    }
}

#[test]
fn hb02_a_g_independent_mass_mineralogy_fall_and_ssa_reconstruction() {
    let texture = clay_loam_texture();
    let classes = erosion_particle_composition(&texture).expect("composition must resolve");
    let fraction_sum: f64 = classes.iter().map(|class| class.frac).sum();
    let clay_mass: f64 = classes.iter().map(|class| class.frac * class.frcly).sum();
    let silt_mass: f64 = classes.iter().map(|class| class.frac * class.frslt).sum();
    let sand_mass: f64 = classes.iter().map(|class| class.frac * class.frsnd).sum();
    assert!((fraction_sum - 1.0).abs() <= 1.0e-12);
    assert!((clay_mass - texture.clay).abs() <= 1.0e-12);
    assert!((silt_mass - texture.silt).abs() <= 1.0e-12);
    assert!((sand_mass - texture.sand).abs() <= 1.0e-12);
    for class in classes {
        assert!(class.dia_m > 0.0 && class.fall_m_s > 0.0);
        assert!((class.fall_m_s - erosion_falvel(class.spg, class.dia_m)).abs() <= 1.0e-15);
    }
    let soil_ssa: f64 = classes
        .iter()
        .map(|class| class.frac * 6.0 / (class.spg * class.dia_m))
        .sum();
    assert!(soil_ssa.is_finite() && soil_ssa > 0.0);

    let (transport, shares) =
        erosion_yalin_with_class_shares(2.0, &classes, texture.sand).expect("transport consumer");
    assert!(transport > 0.0);
    assert!((shares.iter().sum::<f64>() - 1.0).abs() <= 1.0e-12);
}

#[test]
fn effective_particle_is_the_three_class_log_mean() {
    let classes =
        erosion_particle_composition(&clay_loam_texture()).expect("composition must resolve");
    let (diaeff, spgeff) =
        erosion_effective_particle(&classes).expect("effective particle must resolve");
    // Hand log-mean of the three finest classes.
    let mut ld = 0.0;
    let mut ls = 0.0;
    let mut sf = 0.0;
    for class in classes.iter().take(3) {
        ld += class.frac * class.dia_m.ln();
        ls += class.frac * class.spg.ln();
        sf += class.frac;
    }
    assert!((diaeff - (ld / sf).exp()).abs() < 1.0e-15);
    assert!((spgeff - (ls / sf).exp()).abs() < 1.0e-15);
    // Effective diameter sits between the finest and coarsest of the three.
    assert!(diaeff > classes[0].dia_m && diaeff < classes[2].dia_m);
}

#[test]
fn shield_interpolates_within_and_extrapolates_outside_the_table() {
    // Exactly on a tabulated point (reyn = 4 -> y = 0.04).
    assert!((erosion_shield(4.0) - 0.04).abs() < 1.0e-9);
    // Below and above the table stay positive and finite.
    assert!(erosion_shield(0.5) > 0.0 && erosion_shield(0.5).is_finite());
    assert!(erosion_shield(5000.0) > 0.0 && erosion_shield(5000.0).is_finite());
}

#[test]
fn yalin_is_nonnegative_and_applies_the_sandy_adjustment() {
    let classes =
        erosion_particle_composition(&clay_loam_texture()).expect("composition must resolve");
    // Clay loam (sand 0.25 < 0.5): no sandy adjustment.
    let tc_loam = erosion_yalin(2.0, &classes, 0.25).expect("yalin must resolve");
    assert!(tc_loam >= 0.0 && tc_loam.is_finite());

    // A sandy soil (sand 0.8 > 0.5) applies the adjtc reduction: same
    // class shapes but the sandy branch multiplies by adjtc < 1.
    let sandy_classes = erosion_particle_composition(&ErosionTextureInputs {
        sand: 0.8,
        clay: 0.1,
        silt: 0.1,
        orgmat: 0.02,
    })
    .expect("sandy composition must resolve");
    let tc_sandy_raw = erosion_yalin(2.0, &sandy_classes, 0.4).expect("yalin raw");
    let tc_sandy_adj = erosion_yalin(2.0, &sandy_classes, 0.8).expect("yalin adjusted");
    let adjtc = (0.3 + 0.7 * (-12.52_f64 * (0.8 - 0.5)).exp()).max(0.30);
    assert!(
        (tc_sandy_adj - tc_sandy_raw * adjtc).abs() < 1.0e-9 * tc_sandy_raw.max(1.0),
        "sandy adjustment must scale total transport by adjtc"
    );
}

#[test]
fn yalin_and_trcoef_fail_closed_on_nonpositive_shear() {
    let classes =
        erosion_particle_composition(&clay_loam_texture()).expect("composition must resolve");
    assert!(matches!(
        erosion_yalin(0.0, &classes, 0.25),
        Err(DirectRuntimeError::DirectDomainViolation { .. })
    ));
    assert!(matches!(
        erosion_trcoef(-1.0, &classes, 0.25),
        Err(DirectRuntimeError::DirectDomainViolation { .. })
    ));
}

#[test]
fn transport_coefficients_derive_ktrato_and_tcend() {
    let classes =
        erosion_particle_composition(&clay_loam_texture()).expect("composition must resolve");
    let shrsol = 2.0;
    let shrend = 2.5;
    let coeffs = erosion_transport_coefficients(shrsol, shrend, &classes, 0.25)
        .expect("transport coefficients must resolve");
    let kt = erosion_trcoef(shrsol, &classes, 0.25).expect("kt");
    let kt2 = erosion_trcoef(0.5 * (shrend + shrsol), &classes, 0.25).expect("kt2");
    assert!((coeffs.kt - kt).abs() < 1.0e-12);
    assert!((coeffs.kt2 - kt2).abs() < 1.0e-12);
    assert!((coeffs.ktrato - kt2 / kt).abs() < 1.0e-12);
    assert!((coeffs.tcend_kg_s_m - (kt * shrsol.powf(1.5)).max(1.0e-10)).abs() < 1.0e-12);
    assert!(coeffs.ktrato > 0.0 && coeffs.tcend_kg_s_m > 0.0);
}

#[test]
fn rill_hydraulics_grows_gilley_width_and_floors_shear() {
    let cover = ErosionRillCoverInputs {
        rilcov: 0.0,
        canhgt_m: 0.0,
        hmax_m: 0.0,
        flivmx: 0.0,
    };
    let slopes = ErosionShearSlopes {
        cnslp: 0.43,
        slpend: 0.30,
    };
    // qshear = qout*rspace with qout = peakro*efflen.
    let qshear = 1.0e-5 * 200.0 * 1.0;
    let hydraulics = erosion_rill_hydraulics(qshear, &slopes, &cover, 0.0, 1.0)
        .expect("rill hydraulics must resolve");
    // Gilley width from a zero seed = 1.13*qshear^0.303, capped at rspace.
    let gilley = (1.13 * qshear.powf(0.303)).min(1.0);
    assert!((hydraulics.width_m - gilley).abs() < 1.0e-9);
    assert!(hydraulics.shrsol_pa >= 0.000_001);
    assert!(hydraulics.shrend_pa >= 0.000_001);
    // Steeper average gradient -> larger average-slope shear than end.
    assert!(hydraulics.shrsol_pa > hydraulics.shrend_pa);
}

#[test]
fn rill_width_is_capped_at_rill_spacing() {
    let cover = ErosionRillCoverInputs {
        rilcov: 0.0,
        canhgt_m: 0.0,
        hmax_m: 0.0,
        flivmx: 0.0,
    };
    let slopes = ErosionShearSlopes {
        cnslp: 0.1,
        slpend: 0.1,
    };
    // Large discharge would grow the Gilley width past a narrow spacing.
    let hydraulics = erosion_rill_hydraulics(5.0, &slopes, &cover, 0.0, 0.15)
        .expect("rill hydraulics must resolve");
    assert!(hydraulics.width_m <= 0.15 + 1.0e-12);
}

#[test]
fn interrill_delivery_ratio_branches_on_lanuse() {
    let classes =
        erosion_particle_composition(&clay_loam_texture()).expect("composition must resolve");
    // Non-cropland delivers everything.
    let noncrop = erosion_interrill_delivery_ratio(false, 0.01, &classes)
        .expect("non-cropland delivery must resolve");
    assert!((noncrop - 1.0).abs() < 1.0e-12);
    // Cropland delivery is a bounded fraction in [0, 1].
    let crop = erosion_interrill_delivery_ratio(true, 0.01, &classes)
        .expect("cropland delivery must resolve");
    assert!((0.0..=1.0).contains(&crop));
    // High random roughness (rrc >= ~0.0496) drives rif -> 0 -> intdr 0.
    let smooth = erosion_interrill_delivery_ratio(true, 0.06, &classes)
        .expect("high-roughness delivery must resolve");
    assert!(smooth.abs() < 1.0e-12);
}

#[test]
fn detinr_is_zero_only_for_exact_zero_width_or_duration() {
    // Legacy exact-zero branches: no rill area or no excess period.
    assert_eq!(
        erosion_detinr(1.5e6, 1.0, 1.0e-5, 0.02, 1000.0, 1.0, 1.0, 0.0)
            .expect("zero width -> zero detinr"),
        0.0
    );
    assert_eq!(
        erosion_detinr(1.5e6, 1.0, 1.0e-5, 0.02, 0.0, 1.0, 1.0, 0.2)
            .expect("zero effdrr -> zero detinr"),
        0.0
    );
}

#[test]
fn detinr_fails_closed_on_nan_and_negative_inputs() {
    // NaN in any argument is a typed error, never a silent zero.
    assert!(matches!(
        erosion_detinr(1.5e6, 1.0, 1.0e-5, 0.02, f64::NAN, 1.0, 1.0, 0.05),
        Err(DirectRuntimeError::NonFiniteDirectValue { .. })
    ));
    assert!(matches!(
        erosion_detinr(1.5e6, 1.0, 1.0e-5, 0.02, 1000.0, 1.0, 1.0, f64::NAN),
        Err(DirectRuntimeError::NonFiniteDirectValue { .. })
    ));
    // Negative width/duration is an invalid domain, distinct from the
    // legacy exact-zero branch (which would return 0.0).
    assert!(matches!(
        erosion_detinr(1.5e6, 1.0, 1.0e-5, 0.02, 1000.0, 1.0, 1.0, -0.05),
        Err(DirectRuntimeError::DirectDomainViolation { .. })
    ));
    assert!(matches!(
        erosion_detinr(1.5e6, 1.0, 1.0e-5, 0.02, -1000.0, 1.0, 1.0, 0.05),
        Err(DirectRuntimeError::DirectDomainViolation { .. })
    ));
    assert!(matches!(
        erosion_detinr(-1.5e6, 1.0, 1.0e-5, 0.02, 1000.0, 1.0, 1.0, 0.05),
        Err(DirectRuntimeError::DirectDomainViolation { .. })
    ));
}

#[test]
fn rill_hydraulics_fails_closed_on_nan_and_negative_inputs() {
    let classes_cover = ErosionRillCoverInputs {
        rilcov: 0.0,
        canhgt_m: 0.0,
        hmax_m: 0.0,
        flivmx: 0.0,
    };
    let slopes = ErosionShearSlopes {
        cnslp: 0.3,
        slpend: 0.3,
    };
    // NaN cover input is a typed error, not a silent zero-cover friction.
    let nan_cover = ErosionRillCoverInputs {
        rilcov: f64::NAN,
        ..classes_cover
    };
    assert!(matches!(
        erosion_rill_hydraulics(0.01, &slopes, &nan_cover, 0.0, 1.0),
        Err(DirectRuntimeError::NonFiniteDirectValue { .. })
    ));
    // Negative cover is an invalid domain.
    let negative_cover = ErosionRillCoverInputs {
        rilcov: -0.1,
        ..classes_cover
    };
    assert!(matches!(
        erosion_rill_hydraulics(0.01, &slopes, &negative_cover, 0.0, 1.0),
        Err(DirectRuntimeError::DirectDomainViolation { .. })
    ));
    // NaN discharge and negative seed width fail closed.
    assert!(matches!(
        erosion_rill_hydraulics(f64::NAN, &slopes, &classes_cover, 0.0, 1.0),
        Err(DirectRuntimeError::NonFiniteDirectValue { .. })
    ));
    assert!(matches!(
        erosion_rill_hydraulics(0.01, &slopes, &classes_cover, -0.1, 1.0),
        Err(DirectRuntimeError::DirectDomainViolation { .. })
    ));
}

#[test]
fn detinr_matches_the_legacy_product_form() {
    // detinr = ki*kiadjf*effint*(runoff/effdrr)*intdr*rspace/width.
    let (ki, kiadjf, effint, runoff, effdrr, intdr, rspace, width) =
        (1.5e6, 1.0, 1.0e-5, 0.02, 1000.0, 0.8, 1.0, 0.05);
    let expected = ki * kiadjf * effint * (runoff / effdrr) * intdr * rspace / width;
    let observed = erosion_detinr(ki, kiadjf, effint, runoff, effdrr, intdr, rspace, width)
        .expect("detinr must resolve");
    assert!((observed - expected).abs() < 1.0e-6 * expected.abs());
    assert!(observed > 0.0);
}

fn excess_interval(
    duration_s: f64,
    rainfall_intensity_m_s: f64,
    excess_m: f64,
    snowmelt_active: bool,
) -> ErosionExcessInterval {
    ErosionExcessInterval {
        duration_s,
        rainfall_intensity_m_s,
        excess_m,
        snowmelt_active,
    }
}

#[test]
fn effint_effdrr_are_the_excess_weighted_sumint_over_durre() {
    // Three intervals; only the middle two produce excess. effdrr = Σ dur
    // over excess intervals; effint = Σ dur·rate / effdrr (rainfall rate,
    // not excess rate).
    let intervals = [
        excess_interval(600.0, 1.0e-5, 0.0, false), // no excess: excluded
        excess_interval(600.0, 2.0e-5, 0.004, false),
        excess_interval(600.0, 3.0e-5, 0.006, false),
    ];
    let result = erosion_effective_intensity(&intervals).expect("effint must resolve");
    assert!((result.effdrr_s - 1200.0).abs() < 1.0e-9);
    let expected_effint = (600.0 * 2.0e-5 + 600.0 * 3.0e-5) / 1200.0;
    assert!((result.effint_m_s - expected_effint).abs() < 1.0e-15);
    // The faithful effint uses rainfall intensity, so it differs from the
    // mean excess rate (the earlier approximation): mean excess rate here
    // is (0.004+0.006)/1200 = 8.33e-6, well below effint = 2.5e-5.
    assert!(result.effint_m_s > 2.0e-5);
}

#[test]
fn effint_excludes_snowmelt_intervals_from_sumint_but_not_durre() {
    // A snowmelt-driven excess interval contributes to durre (it is an
    // excess period) but not to sumint (rainfall intensity is not the
    // driver): reid.for `if (smrate.le.0.0)`.
    let intervals = [
        excess_interval(600.0, 2.0e-5, 0.004, false),
        excess_interval(600.0, 5.0e-5, 0.010, true), // snowmelt: durre only
    ];
    let result = erosion_effective_intensity(&intervals).expect("effint must resolve");
    assert!((result.effdrr_s - 1200.0).abs() < 1.0e-9);
    // sumint counts only the rain interval; the snowmelt interval's
    // intensity is excluded.
    let expected_effint = (600.0 * 2.0e-5) / 1200.0;
    assert!((result.effint_m_s - expected_effint).abs() < 1.0e-15);
}

#[test]
fn effint_effdrr_are_zero_without_any_excess_period() {
    let intervals = [
        excess_interval(600.0, 1.0e-5, 0.0, false),
        excess_interval(600.0, 1.0e-5, 0.0, false),
    ];
    let result = erosion_effective_intensity(&intervals).expect("no-excess day is inert");
    assert_eq!(result.effdrr_s, 0.0);
    assert_eq!(result.effint_m_s, 0.0);
}

#[test]
fn effint_fails_closed_on_nan_and_negative_inputs() {
    assert!(matches!(
        erosion_effective_intensity(&[excess_interval(600.0, f64::NAN, 0.004, false)]),
        Err(DirectRuntimeError::NonFiniteDirectValue { .. })
    ));
    assert!(matches!(
        erosion_effective_intensity(&[excess_interval(-600.0, 1.0e-5, 0.004, false)]),
        Err(DirectRuntimeError::DirectDomainViolation { .. })
    ));
    assert!(matches!(
        erosion_effective_intensity(&[excess_interval(600.0, 1.0e-5, -0.004, false)]),
        Err(DirectRuntimeError::DirectDomainViolation { .. })
    ));
}

#[test]
fn prtcmp_per_class_mineralogy_matches_legacy_assignments() {
    // E.4 (`prtcmp.for:100-106` + `:208-286`) on the clay-loam texture
    // (clay 0.30, silt 0.45, sand 0.25, orgmat 0.05):
    //   ratiom = orgmat/clay = 1/6
    //   c1 = pure clay, frorg = ratiom
    //   c2 = pure silt, frorg = 0 (clay > 0)
    //   c3: frcly = clay/(clay+silt) = 0.4, frslt = 0.6,
    //       frorg = 0.4·ratiom
    //   c4: back-out from the converged fracs with [0,1] clamps,
    //       frorg = frcly4·ratiom
    //   c5 = pure sand, frorg = 0
    let classes =
        erosion_particle_composition(&clay_loam_texture()).expect("clay-loam composition");
    let ratiom = 0.05 / 0.30;

    assert!((classes[0].frcly - 1.0).abs() < 1.0e-12);
    assert!((classes[0].frorg - ratiom).abs() < 1.0e-12);
    assert!((classes[1].frslt - 1.0).abs() < 1.0e-12);
    assert!(classes[1].frorg.abs() < 1.0e-12);
    assert!((classes[2].frcly - 0.4).abs() < 1.0e-12);
    assert!((classes[2].frslt - 0.6).abs() < 1.0e-12);
    assert!((classes[2].frorg - 0.4 * ratiom).abs() < 1.0e-12);
    // class 4 back-out identities against the published fractions
    let frac = [
        classes[0].frac,
        classes[1].frac,
        classes[2].frac,
        classes[3].frac,
        classes[4].frac,
    ];
    let expected_frcly4 = ((0.30 - frac[0] - 0.4 * frac[2]) / frac[3]).clamp(0.0, 1.0);
    let expected_frslt4 = ((0.45 - frac[1] - 0.6 * frac[2]) / frac[3]).clamp(0.0, 1.0);
    let expected_frsnd4 = ((0.25 - frac[4]) / frac[3]).clamp(0.0, 1.0);
    assert!((classes[3].frcly - expected_frcly4).abs() < 1.0e-12);
    assert!((classes[3].frslt - expected_frslt4).abs() < 1.0e-12);
    assert!((classes[3].frsnd - expected_frsnd4).abs() < 1.0e-12);
    assert!((classes[3].frorg - expected_frcly4 * ratiom).abs() < 1.0e-12);
    assert!((classes[4].frsnd - 1.0).abs() < 1.0e-12);
    assert!(classes[4].frorg.abs() < 1.0e-12);

    // Whole-soil closure: Σ_i frac_i·frcly_i ≈ clay (and silt) — the
    // mineralogy re-composes the texture (the class-4 clamps make this
    // approximate only when a clamp fires; it must not here).
    let clay_back: f64 = classes.iter().map(|c| c.frac * c.frcly).sum();
    let silt_back: f64 = classes.iter().map(|c| c.frac * c.frslt).sum();
    assert!(
        (clay_back - 0.30).abs() < 1.0e-6 && (silt_back - 0.45).abs() < 1.0e-6,
        "class mineralogy must re-compose the surface texture \
         (clay {clay_back}, silt {silt_back})"
    );
}

#[test]
fn yalin_class_shares_sum_to_one_and_survive_the_sandy_adjustment() {
    // `yalin.for:150-160`: tcf1 are the per-class shares of the
    // pre-adjustment transport; the sandy adjustment scales the total
    // and redistributes ws proportionally, so the shares are invariant.
    let classes =
        erosion_particle_composition(&clay_loam_texture()).expect("clay-loam composition");
    let (total, shares) =
        erosion_yalin_with_class_shares(2.0, &classes, 0.25).expect("yalin with shares");
    assert!(total > 0.0);
    let share_sum: f64 = shares.iter().sum();
    assert!(
        (share_sum - 1.0).abs() < 1.0e-12,
        "tcf1 must be a unit-sum share vector (Σ = {share_sum})"
    );
    // Sandy invariance: same classes, sand > 0.5 forces the adjustment;
    // the total drops, the shares must not move.
    let (sandy_total, sandy_shares) =
        erosion_yalin_with_class_shares(2.0, &classes, 0.80).expect("sandy yalin");
    assert!(sandy_total < total);
    for (share, sandy_share) in shares.iter().zip(sandy_shares.iter()) {
        assert!(
            (share - sandy_share).abs() < 1.0e-12,
            "the sandy adjustment must not change the class shares"
        );
    }
    // And the scalar entry point stays consistent with the pair form.
    let scalar = erosion_yalin(2.0, &classes, 0.25).expect("scalar yalin");
    assert!((scalar - total).abs() < 1.0e-15);
}
