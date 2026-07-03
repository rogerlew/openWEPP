//! Wave-1 event/transport operand producer tests (erosion port
//! Increment-1b-A, SC-SED-001). Each producer is checked against the
//! hand-evaluated legacy equation on the McKenzie clay-loam texture: the
//! `forest_high_severity_clay_loam` fixture (`sand = 0.25`,
//! `clay = 0.30`, `silt = 0.45`, `orgmat = 0.05`, `ki = 1.5e6`,
//! `kr = 6e-5`, `shcrit = 0.5`).
#![allow(clippy::doc_markdown)]

use crate::{
    DirectRuntimeError, EROSION_PARTICLE_CLASS_COUNT, ErosionRillCoverInputs, ErosionShearSlopes,
    ErosionTextureInputs, erosion_detinr, erosion_effective_particle, erosion_falvel,
    erosion_interrill_delivery_ratio, erosion_particle_composition, erosion_rill_hydraulics,
    erosion_shield, erosion_transport_coefficients, erosion_trcoef, erosion_yalin,
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
fn detinr_is_zero_without_width_or_excess_duration() {
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
