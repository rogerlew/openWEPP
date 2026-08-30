use super::*;

#[test]
fn live_six_layer_seed_adds_only_exact_zero_synthetic_bgc_inventory() {
    let fixture = openwepp_persisted_restart_v1::restart_authority_prepared_day_fixture();
    let authority_bgc = fixture
        .owners
        .runtime
        .shadow
        .restart_authority_biogeochemistry();
    let mut frame = fixture
        .owners
        .runtime
        .shadow
        .restart_authority_hydrology_frame()
        .clone();
    for lane in &mut frame.lanes {
        let fallback = lane
            .subsurface_layers
            .last()
            .cloned()
            .expect("fixture lane has a subsurface layer");
        lane.subsurface_layers.resize(6, fallback);
        lane.water.soil_water_m = lane
            .subsurface_layers
            .iter()
            .map(|layer| layer.theta_m)
            .sum();
    }
    frame.surface_liquid_shadow = None;
    let seed = explicit_repository_test_seed(&frame, Some(41.1))
        .expect("live six-layer explicit owner seed");
    let committed = seed.day_zero_committed().expect("day-zero committed owner");
    let projected = committed
        .scientific
        .biogeochemistry
        .restore()
        .expect("projected BGC owner");
    let root_ids = seed
        .artifact
        .vegetation_configuration
        .strata
        .iter()
        .flat_map(|stratum| &stratum.root_layers)
        .map(|root| root.layer_id.as_str())
        .collect::<BTreeSet<_>>();
    assert!(
        root_ids
            .iter()
            .all(|layer_id| projected.layers.contains_key(*layer_id))
    );
    for layer_id in ["soil-dry", "soil-frozen"] {
        let layer = projected
            .layers
            .get(layer_id)
            .expect("synthetic root has explicit BGC inventory");
        assert_eq!(layer.ammonium_n.to_bits(), 0.0_f64.to_bits());
        assert_eq!(layer.nitrate_n.to_bits(), 0.0_f64.to_bits());
    }
    for (source, destination) in [
        ("soil-1", "thermal-1"),
        ("soil-2", "thermal-2"),
        ("soil-dry", "soil-1"),
        ("soil-frozen", "soil-2"),
    ] {
        let source = authority_bgc
            .layers
            .get(source)
            .expect("fixture source mineral inventory");
        let destination = projected
            .layers
            .get(destination)
            .expect("projected live mineral inventory");
        assert_eq!(
            destination.ammonium_n.to_bits(),
            source.ammonium_n.to_bits()
        );
        assert_eq!(destination.nitrate_n.to_bits(), source.nitrate_n.to_bits());
    }
}

#[test]
fn fixture_zero_bgc_synthesis_rejects_positive_negative_zero_and_unproven_roots() {
    let fixture = openwepp_persisted_restart_v1::restart_authority_prepared_day_fixture();
    let authority = &fixture.owners.runtime.shadow;
    let mut vegetation = authority
        .restart_authority_vegetation_configuration()
        .clone();
    for stratum in &mut vegetation.strata {
        let mut synthetic = stratum
            .root_layers
            .last()
            .cloned()
            .expect("fixture stratum has roots");
        synthetic.layer_id = SoilLayerId::try_new("synthetic-zero-root").expect("layer ID");
        synthetic.root_fraction = 0.0;
        synthetic.mineral_n_root_fraction = 0.0;
        stratum.root_layers.push(synthetic);
    }
    let synthetic_ids = BTreeSet::from(["synthetic-zero-root".to_owned()]);
    let original = authority.restart_authority_biogeochemistry().clone();
    let mut admitted = original.clone();
    synthesize_fixture_zero_bgc_inventory_for_synthetic_roots(
        &mut admitted.layers,
        &vegetation,
        &synthetic_ids,
    )
    .expect("proven exact-zero synthetic inventory");
    assert_eq!(
        admitted.layers["synthetic-zero-root"].ammonium_n.to_bits(),
        0.0_f64.to_bits()
    );
    assert_eq!(
        admitted.layers["synthetic-zero-root"].nitrate_n.to_bits(),
        0.0_f64.to_bits()
    );
    for (layer_id, layer) in &original.layers {
        assert_eq!(&admitted.layers[layer_id], layer);
    }

    let mut positive = vegetation.clone();
    positive.strata[0]
        .root_layers
        .last_mut()
        .expect("synthetic root")
        .mineral_n_root_fraction = f64::MIN_POSITIVE;
    let positive_error = synthesize_fixture_zero_bgc_inventory_for_synthetic_roots(
        &mut original.clone().layers,
        &positive,
        &synthetic_ids,
    )
    .expect_err("positive missing root inventory must fail");
    assert!(
        positive_error
            .to_string()
            .contains("not an exact-zero synthetic root")
    );

    let mut negative_zero = vegetation.clone();
    negative_zero.strata[0]
        .root_layers
        .last_mut()
        .expect("synthetic root")
        .root_fraction = -0.0;
    assert!(
        synthesize_fixture_zero_bgc_inventory_for_synthetic_roots(
            &mut original.clone().layers,
            &negative_zero,
            &synthetic_ids,
        )
        .expect_err("negative-zero missing root inventory must fail")
        .to_string()
        .contains("not an exact-zero synthetic root")
    );

    assert!(
        synthesize_fixture_zero_bgc_inventory_for_synthetic_roots(
            &mut original.clone().layers,
            &vegetation,
            &BTreeSet::new(),
        )
        .expect_err("unproven missing root inventory must fail")
        .to_string()
        .contains("not an exact-zero synthetic root")
    );
}

#[test]
fn production_seed_envelope_rejects_missing_bgc_root_identity() {
    let fixture = openwepp_persisted_restart_v1::restart_authority_prepared_day_fixture();
    let mut frame = fixture
        .owners
        .runtime
        .shadow
        .restart_authority_hydrology_frame()
        .clone();
    frame.surface_liquid_shadow = None;
    let mut seed = explicit_repository_test_seed(&frame, None).expect("explicit owner seed");
    for stratum in &mut seed.artifact.vegetation_configuration.strata {
        let mut missing = stratum
            .root_layers
            .last()
            .cloned()
            .expect("fixture stratum has roots");
        missing.layer_id = SoilLayerId::try_new("missing-bgc-root").expect("layer ID");
        missing.root_fraction = 0.0;
        missing.mineral_n_root_fraction = 0.0;
        stratum.root_layers.push(missing);
    }
    seed.artifact.vegetation_configuration.configuration_sha256 = seed
        .artifact
        .vegetation_configuration
        .canonical_sha256()
        .expect("reseal vegetation configuration");
    let error = seed
        .validate_envelope()
        .expect_err("missing BGC root identity must reject before bootstrap");
    assert!(
        error
            .to_string()
            .contains("vegetation roots omit exact BGC inventory identities")
    );
}
