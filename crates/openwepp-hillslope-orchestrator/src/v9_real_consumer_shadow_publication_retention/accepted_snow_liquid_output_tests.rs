use super::*;

fn output(mass: f64, sensible_enthalpy: f64) -> Stage3AcceptedSnowLiquidOutputV1 {
    let mut value = Stage3AcceptedSnowLiquidOutputV1 {
        support: TimeSupport::new(
            openwepp_coupled_time::ModelTimeNs::new(0),
            openwepp_coupled_time::ModelTimeNs::new(60_000_000_000),
        )
        .expect("positive support"),
        lane_id: 1,
        ofe_id: OfeId::try_new("ofe-1".to_owned()).expect("OFE"),
        topology_sha256: Digest32::from_bytes([1; 32]),
        beginning_snow_owner_sha256: Digest32::from_bytes([2; 32]),
        ending_snow_owner_sha256: Digest32::from_bytes([3; 32]),
        source_receipts_sha256: [
            Digest32::from_bytes([4; 32]),
            Digest32::from_bytes([5; 32]),
            Digest32::from_bytes([6; 32]),
            Digest32::from_bytes([7; 32]),
            Digest32::from_bytes([8; 32]),
            Digest32::from_bytes([9; 32]),
        ],
        mass_kg_m2_ofe_ground: mass,
        sensible_enthalpy_j_m2_ofe_ground: sensible_enthalpy,
        destinations: vec![
            v11_covered::physical_outcome_ledger::Stage3DestinationLiquidOutcomeV1 {
                ofe_id: OfeId::try_new("ofe-1".to_owned()).expect("OFE"),
                tile_id: openwepp_kernel_contract::TileId::try_new("tile-1").expect("tile"),
                tile_fraction: 1.0,
                mass_kg_m2_tile_ground: mass,
                sensible_enthalpy_j_m2_tile_ground: sensible_enthalpy,
            },
        ],
        refreeze_kg_m2_ofe_ground: 0.0,
        physical_ledger_receipt_sha256: Digest32::from_bytes([10; 32]),
        receipt_sha256: Digest32::zero(),
    };
    value.receipt_sha256 = value.digest().expect("output digest");
    value
}

#[test]
fn output_zero_posture_and_six_source_thermodynamics_are_fail_closed() {
    output(0.0, 0.0).validate().expect("exact zero output");

    let zero_with_enthalpy = output(0.0, 1.0);
    assert!(zero_with_enthalpy.validate().is_err());

    let mut duplicate_source = output(1.0e-8, 0.0);
    duplicate_source.source_receipts_sha256[5] = duplicate_source.source_receipts_sha256[4];
    duplicate_source.receipt_sha256 = duplicate_source.digest().expect("duplicate digest");
    assert!(duplicate_source.validate().is_err());

    let too_hot = output(1.0, 4_218.0 * (350.0 - 273.15) + 1.0);
    assert!(too_hot.trial_thermodynamics_require_refinement());
    assert!(too_hot.validate().is_err());

    let admitted = output(1.0, 0.0);
    assert!(!admitted.trial_thermodynamics_require_refinement());

    let mut refreeze = output(0.0, 0.0);
    refreeze.refreeze_kg_m2_ofe_ground = 1.0e-12;
    refreeze.receipt_sha256 = refreeze.digest().expect("refreeze digest");
    refreeze
        .validate()
        .expect("nonnegative physical-ledger refreeze");

    let mut sign_poison = refreeze.clone();
    sign_poison.refreeze_kg_m2_ofe_ground = -f64::MIN_POSITIVE;
    sign_poison.receipt_sha256 = sign_poison.digest().expect("sign-poison digest");
    assert!(sign_poison.validate().is_err());

    let mut omission_poison = refreeze;
    omission_poison.refreeze_kg_m2_ofe_ground = 0.0;
    assert!(
        omission_poison.validate().is_err(),
        "receipt seal binds refreeze"
    );
}
