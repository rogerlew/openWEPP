fn qualification_day_delta_with_two_routes() -> SnowStage3V11QualificationDayDeltaV1 {
    let mut value = qualification_day_delta(0, qualification_endpoint(0, 201, false), 1);
    value.surface_receipt_occurrences = vec![
        qualification_surface_occurrence(digest(71), 0, 60, 0, digest(81)),
        qualification_surface_occurrence(digest(72), 60, 120, 0, digest(82)),
    ];
    value.surface_flow_by_route = BTreeMap::from([
        (
            SnowStage3V11QualifiedSurfaceRouteV1 {
                source_ofe_id: "ofe-1".to_owned(),
                destination_ofe_id: "ofe-2".to_owned(),
                kind: crate::direct_runtime::DirectSurfaceLiquidParcelKind::RawPrecipitation,
                disposition:
                    crate::direct_runtime::DirectSurfaceLiquidReceiptDisposition::RoutedRunoff,
            },
            SnowStage3V11QualifiedMassEnthalpyTotalV1 {
                mass_kg_m2_basis_ofe_ground: 1.25,
                enthalpy_j_m2_basis_ofe_ground: -2.5,
            },
        ),
        (
            SnowStage3V11QualifiedSurfaceRouteV1 {
                source_ofe_id: "ofe-2".to_owned(),
                destination_ofe_id: "outlet".to_owned(),
                kind: crate::direct_runtime::DirectSurfaceLiquidParcelKind::UpstreamRunon,
                disposition:
                    crate::direct_runtime::DirectSurfaceLiquidReceiptDisposition::OutletRunoff,
            },
            SnowStage3V11QualifiedMassEnthalpyTotalV1 {
                mass_kg_m2_basis_ofe_ground: 3.75,
                enthalpy_j_m2_basis_ofe_ground: 4.5,
            },
        ),
    ]);
    value.routed_runoff = SnowStage3V11QualifiedMassEnthalpyTotalV1 {
        mass_kg_m2_basis_ofe_ground: 1.25,
        enthalpy_j_m2_basis_ofe_ground: -2.5,
    };
    value.upstream_runon = SnowStage3V11QualifiedMassEnthalpyTotalV1 {
        mass_kg_m2_basis_ofe_ground: 3.75,
        enthalpy_j_m2_basis_ofe_ground: 4.5,
    };
    value.outlet_runoff = value.upstream_runon;
    value.receipt_sha256 = Digest32::zero();
    value.seal().expect("seal nonempty route delta")
}

#[test]
fn qualification_nonempty_route_wire_roundtrips_and_binds_every_entry() {
    let value = qualification_day_delta_with_two_routes();
    let bytes = serde_json::to_vec(&value).expect("serialize structured route entries");
    let restored: SnowStage3V11QualificationDayDeltaV1 =
        serde_json::from_slice(&bytes).expect("deserialize structured route entries");
    assert_eq!(restored, value);
    restored.validate().expect("roundtrip delta remains sealed");

    let accumulator = SnowStage3V11QualificationAccumulatorV1::reconstruct_from_days([&value])
        .expect("fold routed day");
    let accumulator_bytes = serde_json::to_vec(&accumulator).expect("serialize routed accumulator");
    let restored_accumulator: SnowStage3V11QualificationAccumulatorV1 =
        serde_json::from_slice(&accumulator_bytes).expect("deserialize routed accumulator");
    assert_eq!(restored_accumulator, accumulator);
    restored_accumulator
        .validate()
        .expect("roundtrip routed accumulator remains sealed");

    let mut snapshot = valid_snapshot();
    snapshot.surface_receipt_count = 2;
    snapshot.surface_receipt_root_sha256 = digest(83);
    snapshot.surface_flow_by_route = value.surface_flow_by_route.clone();
    snapshot.routed_runoff_mass_kg_m2 = 1.25;
    snapshot.routed_runoff_enthalpy_j_m2 = -2.5;
    snapshot.upstream_runon_mass_kg_m2 = 3.75;
    snapshot.upstream_runon_enthalpy_j_m2 = 4.5;
    snapshot.outlet_runoff_mass_kg_m2 = 3.75;
    snapshot.outlet_runoff_enthalpy_j_m2 = 4.5;
    snapshot.receipt_sha256 = snapshot
        .reconstructed_digest()
        .expect("seal routed snapshot");
    snapshot.validate().expect("validate routed snapshot");
    let snapshot_bytes = serde_json::to_vec(&snapshot).expect("serialize routed snapshot");
    let restored_snapshot: SnowStage3V11ProductionQualificationSnapshotV1 =
        serde_json::from_slice(&snapshot_bytes).expect("deserialize routed snapshot");
    assert_eq!(restored_snapshot, snapshot);
    restored_snapshot
        .validate()
        .expect("roundtrip routed snapshot remains sealed");

    let mut route_substitution = value.clone();
    let (route, total) = route_substitution
        .surface_flow_by_route
        .pop_first()
        .expect("first route");
    route_substitution.surface_flow_by_route.insert(
        SnowStage3V11QualifiedSurfaceRouteV1 {
            destination_ofe_id: "ofe-substituted".to_owned(),
            ..route
        },
        total,
    );
    route_substitution.receipt_sha256 = Digest32::zero();
    let route_substitution = route_substitution
        .seal()
        .expect("reseal exact route substitution");
    assert_ne!(
        route_substitution.receipt_sha256(),
        value.receipt_sha256(),
        "route key and its mass/enthalpy entry are digest operands",
    );
}

#[test]
fn qualification_nonempty_route_wire_order_duplicate_and_substitution_poisons_fail_closed() {
    let value = qualification_day_delta_with_two_routes();
    let canonical = serde_json::to_value(&value).expect("route wire value");

    let mut reordered = canonical.clone();
    reordered["surface_flow_by_route"]
        .as_array_mut()
        .expect("route entry sequence")
        .reverse();
    assert!(
        serde_json::from_value::<SnowStage3V11QualificationDayDeltaV1>(reordered).is_err(),
        "noncanonical route order must not be silently sorted",
    );

    let mut duplicated = canonical.clone();
    let entries = duplicated["surface_flow_by_route"]
        .as_array_mut()
        .expect("route entry sequence");
    entries.push(entries[0].clone());
    assert!(
        serde_json::from_value::<SnowStage3V11QualificationDayDeltaV1>(duplicated).is_err(),
        "duplicate structured route keys must fail at decoding",
    );

    let mut substituted = canonical;
    substituted["surface_flow_by_route"][0]["total"]["mass_kg_m2_basis_ofe_ground"] =
        serde_json::json!(99.0);
    let substituted: SnowStage3V11QualificationDayDeltaV1 =
        serde_json::from_value(substituted).expect("well-shaped substituted route entry");
    assert!(
        substituted.validate().is_err(),
        "mass substitution must invalidate totals and the canonical seal",
    );
}
