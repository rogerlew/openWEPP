#[test]
fn v44_private_trial_defers_only_aggregate_weighted_ofe_closure() {
    let posture = covered_phase_consistent_carrier_closure_posture_v1(
        CoveredPhaseConsistentPhysicalEvaluationKindV1::PrivateTrial,
    );
    assert_eq!(
        posture,
        CoveredPhaseConsistentCarrierClosurePostureV1::UncommittedPrivateLseExchange
    );
    assert!(posture.carrier_is_provisional());
    assert!(!posture.requires_strict_weighted_ofe_closure());
    assert!(
        !CoveredPhaseConsistentPhysicalEvaluationKindV1::PrivateTrial
            .requires_authentic_receipts()
    );
}

fn v44_exchange_boundary(net_longwave_w_m2: f64, digest_byte: u8) -> Stage3SnowCoveredLowerBoundary {
    let digest = Sha256Digest::try_new(format!("{digest_byte:02x}").repeat(32)).expect("digest");
    Stage3SnowCoveredLowerBoundary {
        snow_temperature_k: 273.15,
        latent_heat_j_kg: 2_834_000.0,
        sensible_to_canopy_air_w_m2: 0.0,
        vapor_to_canopy_air_kg_m2_s: 0.0,
        net_longwave_w_m2,
        shortwave_absorbed_w_m2: 0.0,
        precipitation_advection_w_m2: 0.0,
        carrier_receipt_id: digest.clone(),
        snow_vis_albedo: 0.8,
        snow_nir_albedo: 0.8,
        stage3_albedo_state_sha256: digest.clone(),
        forcing_receipt_sha256: digest,
        optical_receipt_sha256: None,
        reciprocal_longwave_receipt_sha256: None,
        final_canopy_boundary_receipt_sha256: None,
    }
}

fn v44_exchange_key() -> (OfeId, TileId) {
    (
        OfeId::try_new("v44-ofe").expect("OFE"),
        TileId::try_new("v44-covered").expect("tile"),
    )
}

#[test]
fn v44_receipt_probe_and_replay_require_strict_weighted_ofe_closure() {
    let stale = BTreeMap::from([(v44_exchange_key(), v44_exchange_boundary(0.0, 0x44))]);
    let corrected = BTreeMap::from([(
        v44_exchange_key(),
        v44_exchange_boundary(7.058_344_714_996_6, 0x45),
    )]);
    for kind in [
        CoveredPhaseConsistentPhysicalEvaluationKindV1::ReceiptStabilizationProbe,
        CoveredPhaseConsistentPhysicalEvaluationKindV1::ReceiptStabilizationReplay,
    ] {
        let posture = covered_phase_consistent_carrier_closure_posture_v1(kind);
        assert_eq!(
            posture,
            CoveredPhaseConsistentCarrierClosurePostureV1::StrictAuthenticWeightedOfe
        );
        assert!(!posture.carrier_is_provisional());
        assert!(posture.requires_strict_weighted_ofe_closure());
        assert!(kind.requires_authentic_receipts());
        assert_eq!(
            covered_phase_consistent_carrier_input_exchange_v1(
                kind,
                &stale,
                Some(&corrected),
            )
            .expect("strict corrected exchange"),
            corrected
        );
        assert_eq!(
            covered_phase_consistent_carrier_input_exchange_v1(kind, &stale, None),
            Err(PhaseConsistentCoupledSolveErrorV1::Structure)
        );
    }
}
