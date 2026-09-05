use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use jsonschema::Draft;
use serde_json::Value;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(relative: &str) -> String {
    fs::read_to_string(root().join(relative)).expect("authority file must exist")
}

fn approx(actual: &Value, expected: &Value, label: &str) {
    let actual = actual.as_f64().expect("actual numeric value");
    let expected = expected.as_f64().expect("expected numeric value");
    assert!(
        (actual - expected).abs() < 1e-12,
        "{label}: {actual} != {expected}"
    );
}

fn canonical_tick(value: &Value) -> bool {
    let Some(value) = value.as_str() else {
        return false;
    };
    value == "0"
        || (value.as_bytes().first() != Some(&b'0')
            && value.bytes().all(|byte| byte.is_ascii_digit())
            && value.parse::<u128>().is_ok())
}

fn support_receipts_match(event: &Value, phase: &str) -> bool {
    let participant_key = format!("{phase}_active_participant_set");
    let receipt_key = format!("{phase}_support_receipts");
    let receipt_id_key = format!("{phase}_support_receipt_ids");
    let participants = event[&participant_key]
        .as_array()
        .expect("participant array");
    let receipts = event[&receipt_key].as_array().expect("receipt array");
    let receipt_ids = event[&receipt_id_key].as_array().expect("receipt id array");
    let participant_set = participants
        .iter()
        .map(|item| item.as_str().expect("participant id"))
        .collect::<BTreeSet<_>>();
    let declared_receipt_ids = receipt_ids
        .iter()
        .map(|item| item.as_str().expect("receipt id"))
        .collect::<Vec<_>>();
    let actual_receipt_ids = receipts
        .iter()
        .map(|receipt| {
            receipt["support_receipt_id"]
                .as_str()
                .expect("support receipt id")
        })
        .collect::<Vec<_>>();
    let actual_participants = receipts
        .iter()
        .map(|receipt| receipt["participant_id"].as_str().unwrap_or_default())
        .collect::<BTreeSet<_>>();
    if receipts.len() != participants.len()
        || actual_receipt_ids.len() != BTreeSet::from_iter(actual_receipt_ids.iter()).len()
        || actual_participants != participant_set
        || declared_receipt_ids != actual_receipt_ids
    {
        return false;
    }
    receipts.iter().all(|receipt| {
        participant_set.contains(receipt["participant_id"].as_str().unwrap_or_default())
            && declared_receipt_ids
                .contains(&receipt["support_receipt_id"].as_str().unwrap_or_default())
    })
}

fn carrier_support_receipts_match(carrier: &Value) -> bool {
    let participants = carrier["active_participant_set"]
        .as_array()
        .expect("carrier participant array")
        .iter()
        .map(|item| item.as_str().expect("carrier participant id"))
        .collect::<BTreeSet<_>>();
    let receipts = carrier["support_receipts"]
        .as_array()
        .expect("carrier receipt array");
    let receipt_ids = receipts
        .iter()
        .map(|receipt| receipt["receipt_id"].as_str().expect("carrier receipt id"))
        .collect::<Vec<_>>();
    let receipt_participants = receipts
        .iter()
        .map(|receipt| receipt["participant_id"].as_str().unwrap_or_default())
        .collect::<BTreeSet<_>>();
    receipts.len() == participants.len()
        && receipt_ids.len() == BTreeSet::from_iter(receipt_ids.iter()).len()
        && receipt_participants == participants
        && receipts.iter().all(|receipt| {
            participants.contains(receipt["participant_id"].as_str().unwrap_or_default())
                && !receipt["receipt_id"]
                    .as_str()
                    .unwrap_or_default()
                    .is_empty()
        })
        && receipts
            .iter()
            .map(|receipt| receipt["participant_id"].as_str().unwrap_or_default())
            .collect::<BTreeSet<_>>()
            == participants
}

#[test]
fn canonical_contracts_bind_child_2c_authority_without_activation() {
    let contracts = [
        read("docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md"),
        read("docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md"),
        read("docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md"),
        read("docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md"),
        read("docs/specifications/science-contracts/contracts/SC-VEGETATIONTRANSACTION-001.md"),
    ];
    for required in [
        "common_minimum_support = max(minimum support of every active physical participant)",
        "proposed_event_tick",
        "accepted_event_tick",
        "EventBoundaryNoCandidate",
        "ERR-CT-021",
        "R_E = snow_error / epsilon_M",
        "base-10 strings representing unsigned",
        "LseSupportAdmissibilityReceiptV1",
        "dt >= 60000000000 ns",
        "One shared canopy-air node",
        "SNOWENERGY-E-WIND-001",
        "SNOWENERGY-E-REGIME-001",
        "canopy-intercepted snow",
        "complete-owner-only commit",
        "IMPLEMENTATION_MISSING",
    ] {
        assert!(
            contracts.iter().any(|contract| contract.contains(required)),
            "missing {required}"
        );
    }

    let package = read(
        "docs/work-packages/20260821-snow-stage3-shared-carrier-authority-closure-001/package.md",
    );
    assert!(package.contains("No production Rust is in the write set"));
    assert!(package.contains("dual science review"));
}

#[test]
fn current_snow_energy_preserves_v22_physical_custody_and_v17_precipitation() {
    let contract = read("docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md");
    let registry = read("docs/specifications/science-contracts/index.md");

    for required in [
        "status: approved",
        "maturity: active",
        "contract_version: 58",
        "last_reviewed: 2026-09-01",
        "INV-SNOWENERGY-042",
        "INV-SNOWENERGY-043",
        "INV-SNOWENERGY-044",
        "OBL-SNOWENERGY-C-018",
        "OBL-SNOWENERGY-C-019",
        "OBL-SNOWENERGY-C-020",
        "TOL-SNOWENERGY-003",
        "SCHEMA_UNDEFINED / IMPLEMENTATION_BLOCKED",
        "X_lane = sum_i(f_i * X_i)",
        "not divided by the covered fraction",
        "INV-SNOWENERGY-045",
        "INV-SNOWENERGY-046",
        "INV-SNOWENERGY-047",
        "OBL-SNOWENERGY-P-011",
        "OBL-SNOWENERGY-C-021",
        "SNOWENERGY-E-PRECIP-001",
        "complete empty set",
        "Raw rain and vegetation release cannot both be delivered",
        "Solid atmospheric precipitation bypasses vegetation",
        "identical accepted parcel identities",
        "sum_p(f_destination,p * X_p)",
        "SC-VEGETATION-001@28",
        "SC-LANDSURFACEENERGY-001",
        "INV-SNOWENERGY-048",
        "INV-SNOWENERGY-049",
        "INV-SNOWENERGY-050",
        "OBL-SNOWENERGY-P-012",
        "OBL-SNOWENERGY-C-022",
        "SNOWENERGY-E-SOIL-HEAT-001",
        "bottom represented Stage 3 snow thermal volume",
        "first ordered OFE soil-thermal node",
        "dz_sb/(2*lambda_sb)+dz_1/(2*lambda_1)",
        "bar(G_ss)=0.5*(G_ss,0+G_ss,1)",
        "SnowSoilHeatReceiptV1",
        "first-tile selection",
        "duplicated lane flux",
    ] {
        assert!(
            contract.contains(required),
            "missing in-review v17 precipitation authority: {required}"
        );
    }
    assert!(registry.contains(
        "| `SC-SNOWENERGY-001` | Snow-Surface Energy and Sub-Canopy Longwave Contract | `approved` | `active` |"
    ));
    assert!(
        registry.contains("non-versioned ADR-0044 amendments bind one canonical covered solver")
    );
    assert!(!registry.contains("v14 binds the default-off shared V11/Stage 3 carrier"));
    assert_eq!(contract.matches("| `INV-SNOWENERGY-041` |").count(), 1);
    assert_eq!(contract.matches("| `INV-SNOWENERGY-042` |").count(), 2);
    assert_eq!(contract.matches("| `INV-SNOWENERGY-043` |").count(), 3);
    assert_eq!(contract.matches("| `INV-SNOWENERGY-044` |").count(), 3);
    assert_eq!(
        contract
            .matches("| `OBL-SNOWENERGY-C-018` | OFE-ground lane-boundary consumer |")
            .count(),
        1
    );
    assert!(contract.contains("SNOWENERGY-V15-OFE-GROUND-LANE"));
    assert!(contract.contains("TOL-SNOWENERGY-002"));
    assert!(contract.contains("| `OBL-SNOWENERGY-C-018` | OFE-ground lane-boundary consumer |"));
    assert!(contract.contains("REF-SNOWENERGY-USER-OFE-GROUND-V15"));
    assert!(contract.contains("REF-SNOWENERGY-PRECIP-CUSTODY-V17"));
    assert!(contract.contains("REF-SNOWENERGY-SOIL-BOUNDARY-V18"));

    let lse_contract =
        read("docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md");
    for required in [
        "contract_version: 31",
        "INV-LANDSURFACEENERGY-124",
        "INV-LANDSURFACEENERGY-125",
        "INV-LANDSURFACEENERGY-126",
        "R_ss       = dz_sb/(2*lambda_sb) + dz_1/(2*lambda_1)",
        "SnowSoilHeatReceiptV1",
        "LSEB-E-044",
    ] {
        assert!(
            lse_contract.contains(required),
            "missing in-review LSE v8 snow-soil authority: {required}"
        );
    }

    let attachment = format!(
        "{}\n{}\n{}",
        read("crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_attachment.rs"),
        read("crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_precipitation.rs"),
        read("crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_snow_soil_heat.rs")
    );
    for required in [
        "Stage3PrecipitationPhaseParcelSetV1",
        "validate_precipitation_phase_parcel_set",
        "precipitation_advected_heat_j_m2_tile_ground",
        "DirectSnowStage3V11AttachmentError::Precipitation",
        "SnowSoilHeatReceiptV1",
        "validate_snow_soil_heat_receipt",
        "snow_soil_heat_w_m2_ofe_ground",
        "DirectSnowStage3V11AttachmentError::SnowSoilHeat",
    ] {
        assert!(
            attachment.contains(required),
            "missing v18 production source guard: {required}"
        );
    }

    let receipt_source =
        read("crates/openwepp-hillslope-orchestrator/src/snow_stage3_terminal_handoff.rs");
    let covered_source =
        read("crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow.rs");
    assert!(!receipt_source.contains("CoveredTileGround"));
    assert!(!covered_source.contains("CoveredTileGround"));
    assert!(!covered_source.contains("1.0 / fraction_sum"));
    assert!(receipt_source.contains("STAGE3_OFE_TILE_FRACTION_CLOSURE_TOLERANCE"));
    assert!(receipt_source.contains("Stage3TileBoundaryClassV1"));
    assert!(receipt_source.contains("LaneBoundaryTopologyExpectationV1"));
    assert!(receipt_source.contains("lane Stage-3 boundary topology authority join"));
    assert!(receipt_source.contains("LaneStage3BoundaryReceiptV1::try_new(poisoned_class"));
    assert!(receipt_source.contains("LaneStage3BoundaryReceiptV1::try_new(poisoned_model"));
    assert!(receipt_source.contains("lane_source_set_digests"));
}

#[test]
fn independent_reference_model_reconstructs_carrier_boundary_and_ledgers() {
    let artifact = root().join(
        "docs/work-packages/20260821-snow-stage3-shared-carrier-authority-closure-001/artifacts",
    );
    let source = fs::read_to_string(artifact.join("reference_model.py")).expect("reference source");
    assert!(!source.contains("import openwepp"));
    assert!(!source.contains("subprocess"));
    assert!(!source.contains("candidate_errors"));
    assert!(source.contains("terminal_errors"));
    assert!(source.contains("diagnostic_alias"));

    let output = Command::new("python3")
        .arg(artifact.join("reference_model.py"))
        .current_dir(root())
        .output()
        .expect("reference model runs");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let actual: Value = serde_json::from_slice(&output.stdout).expect("reference result JSON");
    assert_eq!(
        actual["schema"],
        "OPENWEPP_SNOW_STAGE3_SHARED_CARRIER_REFERENCE_RESULTS_V2"
    );
    let vectors: Value = serde_json::from_str(&read("docs/work-packages/20260821-snow-stage3-shared-carrier-authority-closure-001/artifacts/carrier-boundary-vectors.json")).expect("vector JSON");
    let mut expected = BTreeMap::new();
    for group in ["carrier_cases", "event_cases", "conservation_cases"] {
        for case in vectors[group].as_array().expect("case array") {
            expected.insert(case["id"].as_str().unwrap(), case);
        }
    }

    let results = actual["results"].as_array().expect("result array");
    assert_eq!(results.len(), expected.len());
    for result in results {
        let id = result["id"].as_str().expect("result id");
        let case = expected.get(id).expect("result has frozen case");
        assert_eq!(result["status"], case["status"], "status {id}");
        if case["status"] == "rejected" {
            assert_eq!(result["error"], case["expected"]["error"], "error {id}");
            assert_eq!(
                result["after_sha256"], case["expected"]["after_sha256"],
                "rollback {id}"
            );
            assert_eq!(
                result["after_owner_digest"], case["expected"]["after_owner_digest"],
                "owner rollback {id}"
            );
            assert!(
                !result["after_owner_digest"].as_str().unwrap().is_empty(),
                "owner rollback digest {id}"
            );
            assert_eq!(
                result["retry"],
                case["expected"]
                    .get("retry")
                    .cloned()
                    .unwrap_or(Value::Bool(false))
            );
            continue;
        }
        if case["id"].as_str().unwrap().contains("carrier")
            || case["id"] == "shared_node_two_surface"
        {
            for key in [
                "shared_air_temperature_k",
                "shared_air_specific_humidity",
                "snow_sensible_into_surface_w_m2",
                "snow_vapor_into_surface_kg_m2_s",
                "canopy_sensible_into_surface_w_m2",
                "canopy_vapor_into_surface_kg_m2_s",
                "reference_sensible_into_node_w_m2",
                "reference_vapor_into_node_kg_m2_s",
                "sky_view_fraction",
                "snow_longwave_net_w_m2",
                "snow_canopy_longwave_exchange_w_m2",
                "temperature_residual_w_m2",
                "vapor_residual_kg_m2_s",
                "snow_ice_end_kg_m2",
                "liquid_end_kg_m2",
                "vapor_net_kg_m2",
                "energy_closure_j_m2",
                "longwave_reciprocal_closure_j_m2",
            ] {
                approx(&result[key], &case["expected"][key], &format!("{id} {key}"));
            }
            assert_eq!(result["exposure_receipt_id"], "exposure-v1");
            assert_eq!(
                result["common_minimum_support_ns"], "600000000",
                "superseded v17 reference vector retains its historical authority"
            );
        } else if case["id"] == "independent_snow_liquid_vapor_energy_time_reconstruction" {
            for key in [
                "snow_ice_end_kg_m2",
                "liquid_end_kg_m2",
                "vapor_net_kg_m2",
                "energy_closure_j_m2",
            ] {
                approx(&result[key], &case["expected"][key], &format!("{id} {key}"));
            }
            assert_eq!(
                result["event_time_error_ns"],
                case["expected"]["event_time_error_ns"]
            );
        } else {
            assert_eq!(
                result["accepted_event_tick"], case["expected"]["accepted_event_tick"],
                "accepted tick {id}"
            );
            if let Some(expected_support) = case["expected"].get("pre_common_minimum_support") {
                assert_eq!(result["pre_common_minimum_support"], *expected_support);
            }
            if let Some(expected_support) = case["expected"].get("post_common_minimum_support") {
                assert_eq!(result["post_common_minimum_support"], *expected_support);
            }
            if let Some(expected_successor) = case["expected"].get("positive_physical_successor") {
                assert_eq!(result["positive_physical_successor"], *expected_successor);
            }
            if let Some(expected_rank) = case["expected"].get("tie_break_rank") {
                let actual_rank = result["tie_break_rank"].as_array().unwrap();
                let expected_rank = expected_rank.as_array().unwrap();
                assert_eq!(actual_rank[0], expected_rank[0], "time rank {id}");
                approx(&actual_rank[1], &expected_rank[1], &format!("score {id}"));
                assert_eq!(actual_rank[2], expected_rank[2], "tick rank {id}");
            }
            let evaluations = result["candidate_evaluations"]
                .as_array()
                .expect("candidate evaluations");
            let ticks = evaluations
                .iter()
                .map(|evaluation| evaluation["tick"].as_str().unwrap())
                .collect::<Vec<_>>();
            assert!(
                ticks.windows(2).all(|window| window[0] < window[1]),
                "candidate ordering {id}"
            );
            assert!(
                evaluations
                    .iter()
                    .all(|evaluation| canonical_tick(&evaluation["tick"])
                        && canonical_tick(&evaluation["event_time_error_ns"]))
            );
        }
    }
}

#[test]
fn receipt_schemas_and_fixtures_close_wire_and_custody_shape() {
    let artifact =
        "docs/work-packages/20260821-snow-stage3-shared-carrier-authority-closure-001/artifacts/";
    let carrier_schema: Value = serde_json::from_str(&read("docs/work-packages/20260821-snow-stage3-shared-carrier-authority-closure-001/artifacts/carrier-receipt-schema.json")).unwrap();
    let event_schema: Value = serde_json::from_str(&read("docs/work-packages/20260821-snow-stage3-shared-carrier-authority-closure-001/artifacts/event-boundary-receipt-schema.json")).unwrap();
    let fixtures: Value = serde_json::from_str(&read("docs/work-packages/20260821-snow-stage3-shared-carrier-authority-closure-001/artifacts/receipt-schema-fixtures.json")).unwrap();
    let carrier_validator = jsonschema::options()
        .with_draft(Draft::Draft202012)
        .build(&carrier_schema)
        .expect("carrier schema compiles");
    let event_validator = jsonschema::options()
        .with_draft(Draft::Draft202012)
        .build(&event_schema)
        .expect("event schema compiles");
    assert!(carrier_validator.is_valid(&fixtures["valid_carrier"]));
    assert!(event_validator.is_valid(&fixtures["valid_event"]));
    assert!(carrier_support_receipts_match(&fixtures["valid_carrier"]));
    let mut duplicate_carrier_receipt = fixtures["valid_carrier"].clone();
    duplicate_carrier_receipt["support_receipts"][1]["receipt_id"] =
        duplicate_carrier_receipt["support_receipts"][0]["receipt_id"].clone();
    assert!(carrier_validator.is_valid(&duplicate_carrier_receipt));
    assert!(!carrier_support_receipts_match(&duplicate_carrier_receipt));
    let mut omitted_carrier_participant = fixtures["valid_carrier"].clone();
    omitted_carrier_participant["support_receipts"][1]["participant_id"] =
        omitted_carrier_participant["support_receipts"][0]["participant_id"].clone();
    omitted_carrier_participant["support_receipts"][1]["receipt_id"] =
        Value::String("support-omitted-participant".to_string());
    assert!(carrier_validator.is_valid(&omitted_carrier_participant));
    assert!(!carrier_support_receipts_match(
        &omitted_carrier_participant
    ));
    let mut empty_carrier_receipt = fixtures["valid_carrier"].clone();
    empty_carrier_receipt["support_receipts"][0]["receipt_id"] = Value::String(String::new());
    assert!(!carrier_validator.is_valid(&empty_carrier_receipt));
    assert!(!carrier_support_receipts_match(&empty_carrier_receipt));
    let mut invalid_tick = fixtures["valid_event"].clone();
    invalid_tick["accepted_event_tick"] = Value::String("0001".to_string());
    assert!(!event_validator.is_valid(&invalid_tick));
    let mut forged_join = fixtures["valid_event"].clone();
    forged_join["pre_support_receipts"][0]["participant_id"] = Value::String("forged".to_string());
    assert!(event_validator.is_valid(&forged_join));
    assert!(!support_receipts_match(&forged_join, "pre"));
    let mut duplicate_receipt = fixtures["valid_event"].clone();
    duplicate_receipt["pre_support_receipts"][1]["support_receipt_id"] =
        duplicate_receipt["pre_support_receipts"][0]["support_receipt_id"].clone();
    assert!(event_validator.is_valid(&duplicate_receipt));
    assert!(!support_receipts_match(&duplicate_receipt, "pre"));
    let mut omitted_event_participant = fixtures["valid_event"].clone();
    omitted_event_participant["pre_support_receipts"][1]["participant_id"] =
        omitted_event_participant["pre_support_receipts"][0]["participant_id"].clone();
    omitted_event_participant["pre_support_receipts"][1]["support_receipt_id"] =
        Value::String("v11-pre-omitted-participant".to_string());
    omitted_event_participant["pre_support_receipt_ids"][1] =
        Value::String("v11-pre-omitted-participant".to_string());
    assert!(event_validator.is_valid(&omitted_event_participant));
    assert!(!support_receipts_match(&omitted_event_participant, "pre"));
    for schema in [&carrier_schema, &event_schema] {
        assert_eq!(schema["additionalProperties"], false);
        assert!(!schema["required"].as_array().unwrap().is_empty());
    }
    assert!(
        carrier_schema["required"]
            .as_array()
            .unwrap()
            .iter()
            .any(|key| key == "owner_map")
    );
    assert!(
        carrier_schema["required"]
            .as_array()
            .unwrap()
            .iter()
            .any(|key| key == "mass_ledger")
    );
    assert!(
        event_schema["required"]
            .as_array()
            .unwrap()
            .iter()
            .any(|key| key == "candidate_evaluations")
    );
    assert_eq!(
        event_schema["$defs"]["tick"]["pattern"],
        "^(0|[1-9][0-9]*)$"
    );
    assert!(
        fixtures["valid_carrier"]["exposure_receipt"]["provider_digest"]
            .as_str()
            .unwrap()
            .len()
            > 0
    );
    assert!(
        fixtures["valid_event"]["candidate_ticks"]
            .as_array()
            .unwrap()
            .windows(2)
            .all(|pair| pair[0].as_str() < pair[1].as_str())
    );
    assert!(canonical_tick(
        &fixtures["valid_event"]["accepted_event_tick"]
    ));
    assert!(!canonical_tick(
        &fixtures["invalid_event_leading_zero_tick"]["tick"]
    ));
    assert!(!canonical_tick(
        &fixtures["invalid_event_overflow_tick"]["tick"]
    ));
    let valid_event = &fixtures["valid_event"];
    let pre_participants = valid_event["pre_active_participant_set"]
        .as_array()
        .unwrap()
        .iter()
        .map(Value::as_str)
        .collect::<Vec<_>>();
    let pre_receipts = valid_event["pre_support_receipts"].as_array().unwrap();
    assert_eq!(pre_receipts.len(), pre_participants.len());
    assert!(
        pre_receipts
            .iter()
            .all(|receipt| pre_participants.contains(&receipt["participant_id"].as_str()))
    );
    let forged = &fixtures["invalid_event_support_join"];
    assert!(!pre_receipts.iter().any(|receipt| {
        receipt["participant_id"] == forged["participant_id"]
            && receipt["support_receipt_id"] == forged["support_receipt_id"]
    }));
    assert!(
        !fixtures["invalid_event_unsorted_ticks"]
            .as_array()
            .unwrap()
            .windows(2)
            .all(|pair| pair[0].as_str() < pair[1].as_str())
    );
    assert!(artifact.ends_with("artifacts/"));
}

#[test]
fn vector_population_covers_support_aliases_ties_and_wrong_regime_poison() {
    let vectors: Value = serde_json::from_str(&read("docs/work-packages/20260821-snow-stage3-shared-carrier-authority-closure-001/artifacts/carrier-boundary-vectors.json")).expect("vector JSON");
    let carrier_ids = vectors["carrier_cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|case| case["id"].as_str().unwrap())
        .collect::<Vec<_>>();
    for id in [
        "shared_node_two_surface",
        "raw_10m_wind_poison",
        "independent_canopy_air_poison",
        "wrong_regime_snow_flux_poison",
        "canopy_intercepted_snow_scope_poison",
    ] {
        assert!(carrier_ids.contains(&id), "missing carrier vector {id}");
    }
    let event_ids = vectors["event_cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|case| case["id"].as_str().unwrap())
        .collect::<Vec<_>>();
    for id in [
        "unequal_support_pre_event_coalescing",
        "unequal_support_post_event_coalescing",
        "deterministic_equal_displacement_lower_error",
        "deterministic_equal_error_earlier_tick",
        "out_of_window_candidate_is_ignored",
        "exact_active_minimum_support_acceptance",
        "no_candidate_retry_owner_noop",
        "one_nanosecond_structural_event",
        "pre_neighbor_support_violation",
        "post_neighbor_support_violation",
        "one_tick_below_support_rejection",
    ] {
        assert!(event_ids.contains(&id), "missing event vector {id}");
    }
}

#[test]
fn restart_and_rollback_vectors_prove_structured_identity_noop() {
    let vectors: Value = serde_json::from_str(&read("docs/work-packages/20260821-snow-stage3-shared-carrier-authority-closure-001/artifacts/restart-rollback-vectors.json")).unwrap();
    let artifact = root().join(
        "docs/work-packages/20260821-snow-stage3-shared-carrier-authority-closure-001/artifacts",
    );
    let oracle_output = Command::new("python3")
        .arg(artifact.join("reference_model.py"))
        .current_dir(root())
        .output()
        .expect("restart oracle runs");
    assert!(oracle_output.status.success());
    let oracle: Value = serde_json::from_slice(&oracle_output.stdout).expect("restart oracle JSON");
    for case in vectors["cases"].as_array().unwrap() {
        if case["id"].as_str().unwrap().starts_with("restart_") {
            assert_eq!(case["expected_equivalent"], true);
            let source_id = case["source_event_case"].as_str().unwrap();
            let event_result = oracle["results"]
                .as_array()
                .unwrap()
                .iter()
                .find(|result| result["id"] == source_id)
                .expect("source event result");
            assert_eq!(event_result["status"], "accepted");
            let restart_result = oracle["restart_results"]
                .as_array()
                .unwrap()
                .iter()
                .find(|result| result["id"] == case["id"])
                .expect("restart result");
            let expected_cursor = if case["id"] == "restart_before_event" {
                event_result["accepted_event_tick"].clone()
            } else {
                assert_eq!(
                    case["checkpoint"]["accepted_cursor"],
                    event_result["accepted_event_tick"]
                );
                case["resumed_transition"]["accepted_cursor"].clone()
            };
            assert_eq!(restart_result["accepted_cursor"], expected_cursor);
            assert_eq!(restart_result["equivalent"], true);
            assert_eq!(
                restart_result["resumed_owner_digest"],
                restart_result["uninterrupted_owner_digest"]
            );
            assert_eq!(
                restart_result["resumed_receipt_digest"],
                restart_result["uninterrupted_receipt_digest"]
            );
        } else {
            let rollback_result = oracle["restart_results"]
                .as_array()
                .unwrap()
                .iter()
                .find(|result| result["id"] == case["id"])
                .expect("rollback result");
            assert_eq!(case["expected_noop"], true);
            assert_eq!(rollback_result["no_op"], true);
            assert_eq!(
                rollback_result["beginning_owner_digest"],
                rollback_result["restored_owner_digest"]
            );
            assert_ne!(
                rollback_result["beginning_owner_digest"],
                rollback_result["staged_owner_digest"]
            );
        }
    }
}
