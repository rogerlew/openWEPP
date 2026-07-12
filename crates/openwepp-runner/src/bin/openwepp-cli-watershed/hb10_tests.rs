use super::*;

#[test]
fn validator_families_are_characterized() {
    let path = Path::new("manifest.json");
    let empty = json!({});
    assert!(validate_manifest_schema(1, path, &empty).is_err());
    assert!(validate_manifest_publication_policy(1, path, &empty).is_err());
    assert!(validate_manifest_publication_area(1, path, &empty).is_err());
    assert!(validate_manifest_contributor_count(1, 2, path, &empty).is_err());
    assert!(manifest_mofe_hourly_carry_header(1, 2, path, &empty).is_err());
    assert!(validate_manifest_mofe_hourly_carry_required_arrays(1, path, &empty).is_err());
    assert!(validate_manifest_mofe_hourly_carry_totals(1, path, &empty).is_err());

    let valid = json!({
        "schema": HILLSLOPE_RUN_MANIFEST_SCHEMA_ID,
        "wb13_publication": {
            "publication_ofe_policy": MF_PUBLICATION_OFE_POLICY,
            "contributor_ofe_count": 2,
            "area_policy": MOFE04_PUBLICATION_AREA_POLICY,
            "publication_area_m2": 2.0,
            "storage_lineage_policy": MF_STORAGE_LINEAGE_POLICY,
            "per_ofe_state_policy": MF_PER_OFE_STATE_POLICY,
            "transfer_identity_status": MF_IDENTITY_STATUS,
            "per_element_identity_status": MF_IDENTITY_STATUS,
            "aggregate_identity_status": MF_IDENTITY_STATUS,
            "hillslope_total_identity_max_abs_mm": 0.0,
            "row_count": 2,
            "per_ofe_record_count": 2,
            "per_ofe_expected_record_count": 2,
            "per_ofe_internal_day_count": 1,
            "sim_day_index_monotonic": true,
            "first_row_key": {"ofe": 1},
            "last_row_key": {"ofe": 2}
        },
        "mofe_hourly_carry": {
            "policy": MOFE_HOURLY_CARRY_POLICY,
            "active": true,
            "substep_count": MOFE_HOURLY_CARRY_ARRAY_COUNT,
            "required_arrays": MOFE_HOURLY_REQUIRED_ARRAYS,
            "upstream_carry_total_m": 0.0,
            "current_carry_total_m": 0.0
        }
    });
    validate_manifest_schema(1, path, &valid).unwrap();
    validate_manifest_publication_policy(1, path, &valid).unwrap();
    assert_eq!(
        validate_manifest_publication_area(1, path, &valid)
            .unwrap()
            .to_bits(),
        2.0_f64.to_bits()
    );
    validate_manifest_contributor_count(1, 2, path, &valid).unwrap();
    validate_manifest_per_ofe_wb13_publication_metadata(1, 2, path, &valid).unwrap();
    validate_manifest_mofe_hourly_carry_metadata(1, 2, path, &valid).unwrap();

    for (pointer, value) in [
        ("/wb13_publication/publication_area_m2", json!(0.0)),
        (
            "/wb13_publication/hillslope_total_identity_max_abs_mm",
            json!(1.0),
        ),
        ("/mofe_hourly_carry/upstream_carry_total_m", json!(-1.0)),
        ("/mofe_hourly_carry/current_carry_total_m", json!(-1.0)),
    ] {
        let mut mutated = valid.clone();
        *mutated.pointer_mut(pointer).unwrap() = value;
        if pointer.ends_with("publication_area_m2") {
            assert!(validate_manifest_publication_area(1, path, &mutated).is_err());
        } else if pointer.contains("identity_max") {
            assert!(validate_manifest_hillslope_total_residual(1, path, &mutated).is_err());
        } else {
            assert!(validate_manifest_mofe_hourly_carry_totals(1, path, &mutated).is_err());
        }
    }
    let mut wrong_area = valid.clone();
    *wrong_area
        .pointer_mut("/wb13_publication/publication_area_m2")
        .unwrap() = json!("not-a-number");
    assert!(validate_manifest_publication_area(1, path, &wrong_area).is_err());
    let mut residual_boundary = valid.clone();
    *residual_boundary
        .pointer_mut("/wb13_publication/hillslope_total_identity_max_abs_mm")
        .unwrap() = json!(MI_HILLSLOPE_TOTAL_IDENTITY_TOLERANCE_MM);
    validate_manifest_hillslope_total_residual(1, path, &residual_boundary).unwrap();
    *residual_boundary
        .pointer_mut("/wb13_publication/hillslope_total_identity_max_abs_mm")
        .unwrap() = json!(MI_HILLSLOPE_TOTAL_IDENTITY_TOLERANCE_MM.next_up());
    assert!(validate_manifest_hillslope_total_residual(1, path, &residual_boundary).is_err());
}

#[test]
fn simple_boundaries_are_characterized() {
    let path = Path::new("run.toml");
    assert_eq!(
        hillslope_area_m2_from_native(path, "metric", 2.0)
            .unwrap()
            .to_bits(),
        2.0_f64.to_bits()
    );
    assert_eq!(
        hillslope_area_m2_from_native(path, "english", 2.0)
            .unwrap()
            .to_bits(),
        (2.0 * SQUARE_FEET_TO_SQUARE_METERS).to_bits()
    );
    assert!(hillslope_area_m2_from_native(path, "bad", 2.0).is_err());
    assert!(hillslope_area_m2_from_native(path, "metric", 0.0).is_err());
    let lookup = BTreeMap::new();
    assert!(resolve_structure_contributor_local_id(0, TopologyNodeKind::Channel, &lookup).is_ok());
    assert!(resolve_structure_contributor_local_id(4, TopologyNodeKind::Channel, &lookup).is_err());
    print_help();
}

#[test]
fn groundwater_authority_boundaries_are_characterized() {
    let fixture = |name: &str| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/infile/gwcoeff")
            .join(name)
    };
    let disabled =
        watershed_groundwater_authority_from_gwcoeff(None, SidecarPolicy::Compat).unwrap();
    assert!(!disabled.is_enabled());
    let missing = watershed_groundwater_authority_from_gwcoeff(
        Some(&fixture("does-not-exist.txt")),
        SidecarPolicy::Compat,
    )
    .unwrap();
    assert!(!missing.is_enabled());
    let enabled = watershed_groundwater_authority_from_gwcoeff(
        Some(&fixture("strict_valid_numeric_only.txt")),
        SidecarPolicy::Compat,
    )
    .unwrap();
    match enabled {
        WatershedGroundwaterRoutingAuthority::LinearReservoir {
            baseflow_threshold_area_ha,
        } => assert_eq!(baseflow_threshold_area_ha.to_bits(), 1.0_f64.to_bits()),
        WatershedGroundwaterRoutingAuthority::Disabled => panic!("expected enabled authority"),
    }
    for name in [
        "invalid_missing_line4.txt",
        "invalid_nonfinite_line3.txt",
        "invalid_negative_bftharea.txt",
        "invalid_non_numeric_line2.txt",
    ] {
        assert!(
            watershed_groundwater_authority_from_gwcoeff(
                Some(&fixture(name)),
                SidecarPolicy::Compat
            )
            .is_err(),
            "fixture {name} must fail"
        );
    }
    let path = Path::new("direct-gwcoeff");
    assert!(groundwater_authority_from_values(path, 1, None).is_err());
    assert!(groundwater_authority_from_values(path, 2, Some(1.0)).is_err());
    assert!(groundwater_authority_from_values(path, 1, Some(f64::NAN)).is_err());
}

#[test]
fn source_area_arithmetic_and_errors_are_characterized() {
    let root = std::env::temp_dir().join(format!("hb10-area-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    let runfile = root.join("run.toml");
    assert!(hillslope_area_m2_from_source_runfile(&root.join("missing.toml")).is_err());
    fs::write(&runfile, "not toml = [").unwrap();
    assert!(hillslope_area_m2_from_source_runfile(&runfile).is_err());
    fs::write(&runfile, "unit_system = 'metric'\n[inputs]\n").unwrap();
    assert!(hillslope_area_m2_from_source_runfile(&runfile).is_err());

    let slope = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/cancov_forest/hubbardbrook_deciduous_nh/p10.slp");
    for unit in ["metric", "english", "bad"] {
        fs::write(
            &runfile,
            format!(
                "unit_system = '{unit}'\n[inputs]\nslope = '{}'\n",
                slope.display()
            ),
        )
        .unwrap();
        let result = hillslope_area_m2_from_source_runfile(&runfile);
        if unit == "bad" {
            assert!(result.is_err());
        } else {
            let expected_native = 130.6_f64 * 627.0_f64;
            let expected = if unit == "metric" {
                expected_native
            } else {
                expected_native * SQUARE_FEET_TO_SQUARE_METERS
            };
            assert_eq!(result.unwrap().to_bits(), expected.to_bits());
        }
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn crfrac_input_priorities_are_characterized() {
    assert!(validate_routed_particle_class_count(5).is_ok());
    assert!(validate_routed_particle_class_count(6).is_err());
    assert!(parse_watershed_crfrac_soil(Path::new("missing.sol"), SidecarPolicy::Compat).is_err());
    let soil = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/cancov_forest/hubbardbrook_deciduous_nh/p10.sol");
    let profile = parse_watershed_crfrac_soil(&soil, SidecarPolicy::Compat).unwrap();
    let control = |node_id| WatershedChannelControlRecord {
        node_id,
        ishape: 1,
        icntrl: 0,
        ienslp: 0,
        flgout: 0,
        chnz: 0.0,
        chnnbr: 0.0,
        chnn: 0.0,
        chnk: 0.0,
        chntcr: 0.0,
        chnedm: 0.0,
        chneds: 0.0,
        ctlslp: 0.0,
        ctlz: 0.0,
        ctln: 0.0,
        rating_curve: None,
        segment_points: Vec::new(),
        ws20_case12_enabled: false,
        ws21_case34_enabled: false,
        crfrac: Vec::new(),
    };
    let mut controls = BTreeMap::from([(1, control(1))]);
    project_channel_crfrac_from_soil_profile(&mut controls, &profile).unwrap();
    let expected = [
        0.0117_f64,
        0.228_000_000_000_000_1,
        0.081,
        0.166_144_041_686_418_83,
        0.513_155_958_313_581_1,
    ];
    assert_eq!(controls[&1].crfrac.as_slice(), expected);
    assert_ne!(controls[&1].crfrac[0].to_bits(), 0.228_f64.to_bits());
    controls.insert(2, control(2));
    assert!(project_channel_crfrac_from_soil_profile(&mut controls, &profile).is_err());
    let mut no_layer = profile.clone();
    no_layer.ofes[0].layers.clear();
    let mut controls = BTreeMap::from([(1, control(1))]);
    assert!(project_channel_crfrac_from_soil_profile(&mut controls, &no_layer).is_err());
}
