use super::*;

fn remove_growth_symbol(
    projection: &mut openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces,
    root: &str,
) {
    projection
        .pl_growth_surface
        .remove(&BoundarySymbol::from(direct_growth_slot_crop_symbol(
            1, 1, root,
        )));
}

#[test]
fn cqr_growth_crop_authority_binds_every_required_scalar_error_arm() {
    for root in [
        "btemp", "otemp", "gddmax", "dlai", "dropfc", "decfct", "spriod", "bb", "bbb", "hmax",
        "beinp", "extnct", "hi", "xmxlai", "rsr", "rtmmax", "rdmax",
    ] {
        let mut projection = cqr_row7_growth_projection();
        remove_growth_symbol(&mut projection, root);
        let error = direct_production_typed_growth_crop_authority(&projection, 1, 1)
            .expect_err("every growth scalar is required");
        assert!(
            error.to_string().contains(root),
            "wrong error for {root}: {error}"
        );
    }
    for root in ["oratea", "orater"] {
        let mut projection = cqr_row7_growth_projection();
        projection
            .pl_decomp_surface
            .remove(&BoundarySymbol::from(direct_decomp_slot_crop_symbol(
                1, 1, root,
            )));
        let error = direct_production_typed_growth_crop_authority(&projection, 1, 1)
            .expect_err("every decomposition scalar is required");
        assert!(
            error.to_string().contains(root),
            "wrong error for {root}: {error}"
        );
    }
}

#[test]
fn cqr_growth_crop_authority_preserves_schedule_precedence_and_integral_guards() {
    for (surface, root) in [
        ("schedule", "imngmt"),
        ("growth", "imngmt"),
        ("growth", "jdplt"),
        ("growth", "jdharv"),
        ("growth", "jdstop"),
        ("growth", "mgtopt"),
    ] {
        let mut projection = cqr_row7_growth_projection();
        let symbol = if surface == "schedule" {
            direct_growth_schedule_slot_crop_symbol(1, 1, root)
        } else {
            direct_growth_slot_crop_symbol(1, 1, root)
        };
        if surface == "schedule" {
            projection
                .pl_schedule_surface
                .remove(&BoundarySymbol::from(symbol));
        } else {
            projection
                .pl_growth_surface
                .remove(&BoundarySymbol::from(symbol));
        }
        assert!(
            direct_production_typed_growth_crop_authority(&projection, 1, 1).is_err(),
            "missing {surface} {root} must fail"
        );
    }

    let mut fractional = cqr_row7_growth_projection();
    cqr_row7_insert_projection_scalar(
        &mut fractional.pl_growth_surface,
        direct_growth_slot_crop_symbol(1, 1, "jdharv"),
        100.5,
    );
    assert!(direct_production_typed_growth_crop_authority(&fractional, 1, 1).is_err());

    let mut mixed = cqr_row7_growth_projection();
    cqr_row7_insert_projection_scalar(
        &mut mixed.pl_growth_surface,
        direct_growth_slot_crop_symbol(1, 1, "imngmt"),
        1.0,
    );
    let crop = direct_production_typed_growth_crop_authority(&mixed, 1, 1)
        .expect("schedule authority controls perennial date branch");
    assert_eq!(crop.schedule_imngmt, 2);
    assert_eq!(crop.imngmt, 1);
    assert_eq!(crop.jdplt, 0);
    assert_eq!(crop.jdstop, 200);
}

#[test]
fn cqr_growth_crop_authority_rejects_nan_required_scalar() {
    let mut nonfinite_scalar = cqr_row7_growth_projection();
    cqr_row7_insert_projection_scalar(
        &mut nonfinite_scalar.pl_growth_surface,
        direct_growth_slot_crop_symbol(1, 1, "bbb"),
        f64::NAN,
    );
    let scalar_error = direct_production_typed_growth_crop_authority(&nonfinite_scalar, 1, 1)
        .expect_err("NaN canopy coefficient must fail");
    assert!(
        scalar_error.to_string().contains("bbb"),
        "NaN scalar error must retain bbb identity: {scalar_error}"
    );
}

#[test]
fn cqr_growth_crop_authority_rejects_infinite_integral_date() {
    let mut nonfinite_date = cqr_row7_growth_projection();
    cqr_row7_insert_projection_scalar(
        &mut nonfinite_date.pl_growth_surface,
        direct_growth_slot_crop_symbol(1, 1, "jdharv"),
        f64::INFINITY,
    );
    let date_error = direct_production_typed_growth_crop_authority(&nonfinite_date, 1, 1)
        .expect_err("infinite harvest date must fail");
    assert!(
        date_error.to_string().contains("jdharv"),
        "infinite integral error must retain jdharv identity: {date_error}"
    );
}
