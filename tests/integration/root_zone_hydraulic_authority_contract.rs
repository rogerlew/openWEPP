use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{fs, path::PathBuf, process::Command};

const PACKAGE: &str = "docs/work-packages/20260819-root-zone-hydraulic-owner-authority-001";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn text(path: &str) -> String {
    fs::read_to_string(root().join(path)).unwrap()
}

fn f(hex: &Value) -> f64 {
    f64::from_bits(u64::from_str_radix(hex.as_str().unwrap(), 16).unwrap())
}

fn hx(value: f64) -> String {
    format!("{:016x}", value.to_bits())
}

fn rust_libm_expected(input: &Value) -> serde_json::Map<String, Value> {
    let liquid = f(&input["liquid_m"]);
    let thickness = f(&input["thickness_m"]);
    let porosity = f(&input["porosity"]);
    let ksat = f(&input["ksat_m_s"]);
    let psi_sat = f(&input["psi_sat_mm"]);
    let b = f(&input["b"]);
    let top = f(&input["top_m"]);
    let lateral = f(&input["lateral_m"]);
    let dxroot = f(&input["dxroot_m"]);
    assert!(liquid <= capacity_one_bit_limit(porosity * thickness));
    let mut theta = liquid / thickness;
    let mut raw = theta / porosity;
    if theta == 0.0 {
        theta = 0.0;
    }
    if raw == 0.0 {
        raw = 0.0;
    }
    let mut saturation = raw.clamp(0.0, 1.0);
    if saturation == 0.0 {
        saturation = 0.0;
    }
    let s_psi = saturation.max(0.01);
    let psi = (psi_sat * libm::pow(s_psi, -b)).max(-1.0e8);
    let exponent = 2.0 * b + 3.0;
    let conductivity = ksat.min(ksat * libm::pow(saturation, exponent));
    let node = top + 0.5 * thickness;
    [
        ("theta_liq", theta),
        ("relative_saturation_raw", raw),
        ("relative_saturation", saturation),
        ("retention_saturation", s_psi),
        ("matric_potential_mm", psi),
        ("conductivity_exponent", exponent),
        ("current_conductivity_m_s", conductivity),
        ("soil_conductivity_mm_s", 1000.0 * conductivity),
        ("layer_node_depth_m", node),
        ("gravity_root_mm", -1000.0 * node),
        ("root_path_length_mm", 1000.0 * (node + lateral)),
        ("soil_root_interface_distance_m", dxroot),
    ]
    .into_iter()
    .map(|(k, v)| (k.into(), Value::String(hx(v))))
    .collect()
}

fn authority_evaluate(
    input: &Value,
    frozen: bool,
    path_present: bool,
) -> Result<serde_json::Map<String, Value>, &'static str> {
    if !path_present {
        return Err("ConfigurationIdentity");
    }
    if frozen {
        return Err("FrozenRootedLayerUnsupported");
    }
    let fields = [
        "liquid_m",
        "thickness_m",
        "porosity",
        "ksat_m_s",
        "psi_sat_mm",
        "b",
        "top_m",
        "lateral_m",
        "dxroot_m",
    ];
    if fields.iter().any(|name| input.get(name).is_none()) {
        return Err("ConfigurationIdentity");
    }
    let liquid = f(&input["liquid_m"]);
    let thickness = f(&input["thickness_m"]);
    let porosity = f(&input["porosity"]);
    let ksat = f(&input["ksat_m_s"]);
    let psi_sat = f(&input["psi_sat_mm"]);
    let b = f(&input["b"]);
    let top = f(&input["top_m"]);
    let lateral = f(&input["lateral_m"]);
    let dxroot = f(&input["dxroot_m"]);
    if ![
        liquid, thickness, porosity, ksat, psi_sat, b, top, lateral, dxroot,
    ]
    .iter()
    .all(|v| v.is_finite())
        || liquid < 0.0
        || thickness <= 0.0
        || !(0.0 < porosity && porosity <= 1.0)
        || ksat <= 0.0
        || psi_sat >= 0.0
        || b <= 0.0
        || top < 0.0
        || lateral < 0.0
        || dxroot <= 0.0
    {
        return Err("Domain");
    }
    if liquid > capacity_one_bit_limit(porosity * thickness) {
        return Err("WaterAbovePoreCapacity");
    }
    Ok(rust_libm_expected(input))
}

fn capacity_one_bit_limit(capacity: f64) -> f64 {
    f64::from_bits(capacity.to_bits() + 1)
}

fn digest(value: &Value) -> String {
    let mut bytes = serde_json::to_vec(value).unwrap();
    bytes.push(b'\n');
    format!("{:x}", Sha256::digest(bytes))
}

fn validate_configuration(mut value: Value, expected: &Value) -> Result<Value, &'static str> {
    let found = value["configuration_sha256"]
        .as_str()
        .ok_or("ConfigurationIdentity")?
        .to_owned();
    value["configuration_sha256"] = Value::String(String::new());
    if digest(&value) != found {
        return Err("ConfigurationDigest");
    }
    value["configuration_sha256"] = Value::String(found);
    for field in [
        "schema_version",
        "model_definition_sha256",
        "owner_id",
        "hydrology_configuration_sha256",
        "vegetation_configuration_sha256",
        "lse_configuration_sha256",
    ] {
        if value[field] != expected[field] {
            return Err("ConfigurationIdentity");
        }
    }
    let layers = value["ordered_layers"]
        .as_array()
        .ok_or("ConfigurationIdentity")?;
    if layers.is_empty() {
        return Err("ConfigurationIdentity");
    }
    let keys = layers
        .iter()
        .map(|v| {
            (
                v["ofe_id"].as_str(),
                v["production_lane_index"].as_u64(),
                v["production_lane_id"].as_str(),
                v["layer_id"].as_str(),
            )
        })
        .collect::<Vec<_>>();
    if keys.windows(2).any(|w| w[0] >= w[1]) {
        return Err("ConfigurationIdentity");
    }
    if keys
        != vec![
            (Some("ofe-1"), Some(0), Some("lane-1"), Some("layer-1")),
            (Some("ofe-1"), Some(0), Some("lane-1"), Some("layer-2")),
        ]
    {
        return Err("ConfigurationIdentity");
    }
    let strata = value["ordered_stratum_geometry"]
        .as_array()
        .ok_or("ConfigurationIdentity")?;
    let ids = strata
        .iter()
        .map(|v| v["stratum_id"].as_str())
        .collect::<Vec<_>>();
    if ids != vec![Some("stratum-1"), Some("stratum-2")] {
        return Err("ConfigurationIdentity");
    }
    if strata
        .iter()
        .any(|v| v.get("root_tissue_lateral_path_m").is_none())
    {
        return Err("ConfigurationIdentity");
    }
    if strata.iter().any(|v| {
        v["root_tissue_lateral_path_m"]
            .as_f64()
            .is_none_or(|x| !x.is_finite() || x < 0.0)
    }) {
        return Err("Domain");
    }
    Ok(value)
}

#[allow(clippy::too_many_lines)]
fn validate_receipt(
    mut receipt: Value,
    configuration: &Value,
    source: &Value,
) -> Result<Value, &'static str> {
    let found = receipt["receipt_sha256"]
        .as_str()
        .ok_or("ReceiptDigest")?
        .to_owned();
    receipt["receipt_sha256"] = Value::String(String::new());
    if digest(&receipt) != found {
        return Err("ReceiptDigest");
    }
    receipt["receipt_sha256"] = Value::String(found);
    for field in [
        "transaction_id",
        "day_index",
        "interval_index",
        "owner_id",
        "model_definition_sha256",
        "configuration_sha256",
        "hydrology_beginning_state_sha256",
        "vegetation_configuration_sha256",
        "lse_configuration_sha256",
    ] {
        if receipt[field] != source[field] {
            return Err("OwnerJoin");
        }
    }
    if receipt["configuration_sha256"] != configuration["configuration_sha256"]
        || receipt["model_definition_sha256"] != configuration["model_definition_sha256"]
        || receipt["vegetation_configuration_sha256"]
            != configuration["vegetation_configuration_sha256"]
        || receipt["lse_configuration_sha256"] != configuration["lse_configuration_sha256"]
    {
        return Err("OwnerJoin");
    }
    if receipt["frozen"] == true {
        return Err("FrozenRootedLayerUnsupported");
    }
    let configured_layers = configuration["ordered_layers"]
        .as_array()
        .ok_or("ConfigurationIdentity")?;
    let source_layers = source["layers"].as_array().ok_or("OwnerJoin")?;
    let source_keys = source_layers
        .iter()
        .map(|v| {
            (
                &v["ofe_id"],
                &v["production_lane_index"],
                &v["production_lane_id"],
                &v["layer_id"],
            )
        })
        .collect::<Vec<_>>();
    let configured_keys = configured_layers
        .iter()
        .map(|v| {
            (
                &v["ofe_id"],
                &v["production_lane_index"],
                &v["production_lane_id"],
                &v["layer_id"],
            )
        })
        .collect::<Vec<_>>();
    if source_keys != configured_keys {
        return Err("OwnerJoin");
    }
    let layer_index = configured_layers
        .iter()
        .position(|v| {
            receipt["ofe_id"] == v["ofe_id"]
                && receipt["production_lane_index"] == v["production_lane_index"]
                && receipt["production_lane_id"] == v["production_lane_id"]
                && receipt["layer_id"] == v["layer_id"]
        })
        .ok_or("OwnerJoin")?;
    let layer = &configured_layers[layer_index];
    let source_layer = &source_layers[layer_index];
    for field in [
        "occupancy_id",
        "stratum_id",
        "ofe_id",
        "production_lane_index",
        "production_lane_id",
        "layer_id",
        "liquid_water_depth_m",
        "layer_thickness_m",
        "porosity",
        "saturated_conductivity_m_s",
        "soil_root_interface_distance_m",
        "accessible",
        "frozen",
    ] {
        if receipt[field] != source_layer[field] {
            return Err("OwnerJoin");
        }
    }
    let geometry = configuration["ordered_stratum_geometry"]
        .as_array()
        .ok_or("ConfigurationIdentity")?
        .iter()
        .find(|v| receipt["stratum_id"] == v["stratum_id"])
        .ok_or("OwnerJoin")?;
    if receipt["ofe_id"] != layer["ofe_id"]
        || receipt["production_lane_index"] != layer["production_lane_index"]
        || receipt["production_lane_id"] != layer["production_lane_id"]
        || receipt["layer_id"] != layer["layer_id"]
        || receipt["stratum_id"] != geometry["stratum_id"]
        || f(&receipt["root_tissue_lateral_path_m"]).to_bits()
            != geometry["root_tissue_lateral_path_m"]
                .as_f64()
                .unwrap()
                .to_bits()
    {
        return Err("OwnerJoin");
    }
    let top_m = source_layers[..layer_index]
        .iter()
        .filter(|v| {
            v["ofe_id"] == receipt["ofe_id"]
                && v["production_lane_index"] == receipt["production_lane_index"]
                && v["production_lane_id"] == receipt["production_lane_id"]
        })
        .map(|v| f(&v["layer_thickness_m"]))
        .sum::<f64>();
    let input = serde_json::json!({
        "liquid_m": receipt["liquid_water_depth_m"], "thickness_m": receipt["layer_thickness_m"],
        "porosity": receipt["porosity"], "ksat_m_s": receipt["saturated_conductivity_m_s"],
        "psi_sat_mm": hx(layer["saturated_matric_potential_mm"].as_f64().unwrap()),
        "b": hx(layer["clapp_hornberger_b"].as_f64().unwrap()), "top_m": hx(top_m),
        "lateral_m": receipt["root_tissue_lateral_path_m"],
        "dxroot_m": receipt["soil_root_interface_distance_m"]
    });
    let reconstructed = authority_evaluate(&input, false, true)?;
    for (receipt_field, expected_field) in [
        ("relative_saturation", "relative_saturation"),
        ("matric_potential_mm", "matric_potential_mm"),
        ("soil_conductivity_mm_s", "soil_conductivity_mm_s"),
        ("layer_node_depth_m", "layer_node_depth_m"),
        ("gravity_root_mm", "gravity_root_mm"),
        ("root_path_length_mm", "root_path_length_mm"),
    ] {
        if receipt[receipt_field] != reconstructed[expected_field] {
            return Err("ReceiptScientificMismatch");
        }
    }
    Ok(receipt)
}

#[test]
fn candidate_contract_binds_selected_constitutive_authority_and_forbids_aliases() {
    let contract =
        text("docs/specifications/science-contracts/contracts/SC-ROOTZONEHYDRAULICS-001.md");
    for required in [
        "OPENWEPP_ROOT_ZONE_HYDRAULIC_OWNER_V1",
        "root_tissue_lateral_path_m",
        "S = min(1.0, max(0.0, S_raw))",
        "conductivity_exponent = 2.0 * B + 3.0",
        "libm 0.2.16",
        "FrozenRootedLayerUnsupported",
        "WB14 suction/K never enter the owner",
    ] {
        assert!(contract.contains(required), "missing {required}");
    }
    assert!(contract.contains("There is no default"));
    assert!(contract.contains("they are never aliased"));
}

#[test]
fn generated_vectors_are_exact_bit_complete_and_separate_z3_dxroot_and_gravity() {
    let raw = text(&format!(
        "{PACKAGE}/artifacts/root-zone-hydraulic-vectors.json"
    ));
    let vectors: Value = serde_json::from_str(&raw).unwrap();
    let accepted = vectors["accepted"].as_array().unwrap();
    for name in [
        "exact_saturation",
        "one_bit_above_pore_capacity_roundoff",
        "exact_zero_live_water",
        "different_ofe_parameters",
        "same_dxroot_different_z3",
        "same_z3_different_dxroot",
    ] {
        assert!(accepted.iter().any(|v| v["name"] == name), "missing {name}");
    }
    let mut mismatches = Vec::new();
    for vector in accepted {
        for value in vector["expected"].as_object().unwrap().values() {
            let hex = value.as_str().unwrap();
            assert_eq!(hex.len(), 16);
            assert!(hex.bytes().all(|b| b.is_ascii_hexdigit()));
        }
        let rust = rust_libm_expected(&vector["inputs"]);
        if vector["expected"].as_object().unwrap() != &rust {
            mismatches.push((vector["name"].clone(), rust));
        }
    }
    assert!(mismatches.is_empty(), "libm mismatches: {mismatches:#?}");
    let same_dx = accepted
        .iter()
        .find(|v| v["name"] == "same_dxroot_different_z3")
        .unwrap();
    assert_ne!(
        same_dx["expected"]["root_path_length_mm"],
        accepted[0]["expected"]["root_path_length_mm"]
    );
    assert_eq!(
        same_dx["expected"]["soil_root_interface_distance_m"],
        accepted[0]["expected"]["soil_root_interface_distance_m"]
    );
    let intermediate = &accepted[0];
    let expected = intermediate["expected"].as_object().unwrap();
    let input = &intermediate["inputs"];
    let s = f(&expected["relative_saturation"]);
    let ksat = f(&input["ksat_m_s"]);
    assert_ne!(
        expected["current_conductivity_m_s"],
        Value::String(hx(ksat))
    );
    assert_ne!(
        expected["current_conductivity_m_s"],
        Value::String(hx(ksat * libm::pow(s, 2.0 * f(&input["b"]) + 2.0)))
    );
    assert_ne!(
        expected["root_path_length_mm"],
        Value::String(hx(1000.0 * f(&input["dxroot_m"])))
    );
    assert_ne!(expected["root_path_length_mm"], expected["gravity_root_mm"]);
    let lower = accepted
        .iter()
        .find(|v| v["name"] == "exact_s_psi_lower_clamp")
        .unwrap();
    let lower_expected = lower["expected"].as_object().unwrap();
    assert_ne!(
        lower_expected["current_conductivity_m_s"],
        Value::String(hx(f(&lower["inputs"]["ksat_m_s"])
            * libm::pow(
                f(&lower_expected["retention_saturation"]),
                f(&lower_expected["conductivity_exponent"]),
            )))
    );

    let rejected = vectors["rejected"].as_array().unwrap();
    let pore = rejected
        .iter()
        .find(|v| v["name"] == "material_pore_capacity_violation")
        .unwrap();
    let i = &pore["inputs"];
    assert!(f(&i["liquid_m"]) > capacity_one_bit_limit(f(&i["porosity"]) * f(&i["thickness_m"])));
    assert_eq!(pore["error"], "WaterAbovePoreCapacity");
}

#[test]
fn configuration_schema_and_digest_are_canonical_and_closed() {
    let mut configuration: Value = serde_json::from_str(&text(&format!(
        "{PACKAGE}/artifacts/configuration-vector.json"
    )))
    .unwrap();
    let expected = configuration["configuration_sha256"]
        .as_str()
        .unwrap()
        .to_owned();
    configuration["configuration_sha256"] = Value::String(String::new());
    assert_eq!(digest(&configuration), expected);
    let schema = text(&format!("{PACKAGE}/artifacts/configuration-schema.json"));
    for required in [
        "additionalProperties",
        "exclusiveMaximum",
        "exclusiveMinimum",
        "root_tissue_lateral_path_m",
        "^[0-9a-f]{64}$",
    ] {
        assert!(schema.contains(required), "schema misses {required}");
    }
}

#[test]
fn every_rejected_vector_executes_its_typed_guard() {
    let vectors: Value = serde_json::from_str(&text(&format!(
        "{PACKAGE}/artifacts/root-zone-hydraulic-vectors.json"
    )))
    .unwrap();
    let base = vectors["accepted"][0]["inputs"].clone();
    for rejected in vectors["rejected"].as_array().unwrap() {
        let name = rejected["name"].as_str().unwrap();
        let mut input = base.clone();
        let (frozen, path_present) = match name {
            "material_pore_capacity_violation" => {
                input = rejected["inputs"].clone();
                (false, true)
            }
            "frozen_rooted_layer" => (true, true),
            "missing_root_tissue_path" => (false, false),
            "positive_psi_sat" => {
                input["psi_sat_mm"] = rejected["inputs"]["psi_sat_mm"].clone();
                (false, true)
            }
            "zero_b" => {
                input["b"] = rejected["inputs"]["b"].clone();
                (false, true)
            }
            other => panic!("unmapped rejected vector {other}"),
        };
        assert_eq!(
            authority_evaluate(&input, frozen, path_present),
            Err(rejected["error"].as_str().unwrap()),
            "{name}"
        );
    }
}

#[test]
#[allow(clippy::too_many_lines)]
fn configuration_and_receipt_joins_digests_order_and_poisons_execute_atomically() {
    let configuration: Value = serde_json::from_str(&text(&format!(
        "{PACKAGE}/artifacts/configuration-vector.json"
    )))
    .unwrap();
    let expected_configuration: Value = serde_json::from_str(&text(&format!(
        "{PACKAGE}/artifacts/expected-static-context-vector.json"
    )))
    .unwrap();
    let configuration = validate_configuration(configuration, &expected_configuration).unwrap();
    let receipt: Value =
        serde_json::from_str(&text(&format!("{PACKAGE}/artifacts/receipt-vector.json"))).unwrap();
    let second_receipt: Value = serde_json::from_str(&text(&format!(
        "{PACKAGE}/artifacts/receipt-second-layer-vector.json"
    )))
    .unwrap();
    let source: Value = serde_json::from_str(&text(&format!(
        "{PACKAGE}/artifacts/source-owner-vector.json"
    )))
    .unwrap();
    validate_receipt(receipt.clone(), &configuration, &source).unwrap();
    validate_receipt(second_receipt.clone(), &configuration, &source).unwrap();
    let live = serde_json::to_vec(&receipt).unwrap();
    let check = |mutated: Value, expected| {
        assert_eq!(
            validate_receipt(mutated, &configuration, &source),
            Err(expected)
        );
        assert_eq!(serde_json::to_vec(&receipt).unwrap(), live);
    };
    let mut bad = receipt.clone();
    bad["receipt_sha256"] = Value::String("0".repeat(64));
    check(bad, "ReceiptDigest");
    let mut bad = receipt.clone();
    bad["layer_id"] = Value::String("layer-x".into());
    bad["receipt_sha256"] = Value::String(String::new());
    bad["receipt_sha256"] = Value::String(digest(&bad));
    check(bad, "OwnerJoin");
    let mut bad = receipt.clone();
    bad["stratum_id"] = Value::String("stratum-x".into());
    bad["receipt_sha256"] = Value::String(String::new());
    bad["receipt_sha256"] = Value::String(digest(&bad));
    check(bad, "OwnerJoin");
    let mut bad = receipt.clone();
    bad["gravity_root_mm"] = Value::String(hx(f(&receipt["gravity_root_mm"]).abs()));
    bad["receipt_sha256"] = Value::String(String::new());
    bad["receipt_sha256"] = Value::String(digest(&bad));
    check(bad, "ReceiptScientificMismatch");
    let mut bad = receipt.clone();
    bad["frozen"] = Value::Bool(true);
    bad["receipt_sha256"] = Value::String(String::new());
    bad["receipt_sha256"] = Value::String(digest(&bad));
    check(bad, "FrozenRootedLayerUnsupported");

    for field in [
        "hydrology_beginning_state_sha256",
        "model_definition_sha256",
        "vegetation_configuration_sha256",
        "lse_configuration_sha256",
    ] {
        let mut bad = receipt.clone();
        bad[field] = Value::String("9".repeat(64));
        bad["receipt_sha256"] = Value::String(String::new());
        bad["receipt_sha256"] = Value::String(digest(&bad));
        check(bad, "OwnerJoin");
    }
    for field in [
        "transaction_id",
        "day_index",
        "interval_index",
        "owner_id",
        "occupancy_id",
        "ofe_id",
        "production_lane_index",
        "production_lane_id",
    ] {
        let mut bad = receipt.clone();
        bad[field] = match field {
            "day_index" | "interval_index" | "production_lane_index" => Value::from(9),
            _ => Value::String("wrong".into()),
        };
        bad["receipt_sha256"] = Value::String(String::new());
        bad["receipt_sha256"] = Value::String(digest(&bad));
        check(bad, "OwnerJoin");
    }
    for field in [
        "relative_saturation",
        "matric_potential_mm",
        "soil_conductivity_mm_s",
        "layer_node_depth_m",
        "root_tissue_lateral_path_m",
        "root_path_length_mm",
        "soil_root_interface_distance_m",
    ] {
        let mut bad = receipt.clone();
        bad[field] = Value::String(hx(42.0));
        bad["receipt_sha256"] = Value::String(String::new());
        bad["receipt_sha256"] = Value::String(digest(&bad));
        let expected = if matches!(
            field,
            "root_tissue_lateral_path_m" | "soil_root_interface_distance_m"
        ) {
            "OwnerJoin"
        } else {
            "ReceiptScientificMismatch"
        };
        check(bad, expected);
    }

    let mut bad_config = configuration.clone();
    bad_config["ordered_layers"] = Value::Array(vec![
        configuration["ordered_layers"][0].clone(),
        configuration["ordered_layers"][0].clone(),
    ]);
    bad_config["configuration_sha256"] = Value::String(String::new());
    bad_config["configuration_sha256"] = Value::String(digest(&bad_config));
    assert_eq!(
        validate_configuration(bad_config, &expected_configuration),
        Err("ConfigurationIdentity")
    );
    let mut reordered_config = configuration.clone();
    reordered_config["ordered_layers"]
        .as_array_mut()
        .unwrap()
        .reverse();
    reordered_config["configuration_sha256"] = Value::String(String::new());
    reordered_config["configuration_sha256"] = Value::String(digest(&reordered_config));
    assert_eq!(
        validate_configuration(reordered_config, &expected_configuration),
        Err("ConfigurationIdentity")
    );
    for field in [
        "model_definition_sha256",
        "owner_id",
        "hydrology_configuration_sha256",
        "vegetation_configuration_sha256",
        "lse_configuration_sha256",
    ] {
        let mut bad = configuration.clone();
        bad[field] = Value::String(if field == "owner_id" {
            "wrong".into()
        } else {
            "9".repeat(64)
        });
        bad["configuration_sha256"] = Value::String(String::new());
        bad["configuration_sha256"] = Value::String(digest(&bad));
        assert_eq!(
            validate_configuration(bad, &expected_configuration),
            Err("ConfigurationIdentity"),
            "{field}"
        );
    }
    let mut missing = configuration.clone();
    missing["ordered_stratum_geometry"][0]
        .as_object_mut()
        .unwrap()
        .remove("root_tissue_lateral_path_m");
    missing["configuration_sha256"] = Value::String(String::new());
    missing["configuration_sha256"] = Value::String(digest(&missing));
    assert_eq!(
        validate_configuration(missing, &expected_configuration),
        Err("ConfigurationIdentity")
    );
    let mut reordered_source = source.clone();
    reordered_source["layers"].as_array_mut().unwrap().reverse();
    assert_eq!(
        validate_receipt(second_receipt.clone(), &configuration, &reordered_source),
        Err("OwnerJoin")
    );
    let mut wrong_predecessor = source.clone();
    wrong_predecessor["layers"][0]["layer_thickness_m"] = Value::String(hx(0.25));
    assert_eq!(
        validate_receipt(second_receipt, &configuration, &wrong_predecessor),
        Err("ReceiptScientificMismatch")
    );
}

#[test]
fn generator_is_byte_reproducible_and_poison_inventory_is_complete() {
    let artifacts = root().join(PACKAGE).join("artifacts");
    let names = [
        "model-definition.json",
        "root-zone-hydraulic-vectors.json",
        "configuration-schema.json",
        "runtime-descriptor.json",
        "equation-and-operation-order.md",
        "test-vector-ledger.md",
        "poison-matrix.md",
        "reference-calculator.py",
        "configuration-vector.json",
        "artifact-manifest.json",
        "receipt-schema.json",
        "receipt-vector.json",
        "receipt-second-layer-vector.json",
        "source-owner-vector.json",
        "expected-static-context-vector.json",
    ];
    let before = names
        .iter()
        .map(|name| fs::read(artifacts.join(name)).unwrap())
        .collect::<Vec<_>>();
    let status = Command::new(root().join(".venv/bin/python"))
        .arg(
            root()
                .join(PACKAGE)
                .join("tools/generate_authority_artifacts.py"),
        )
        .status()
        .unwrap();
    assert!(status.success());
    for (name, expected) in names.iter().zip(before) {
        assert_eq!(fs::read(artifacts.join(name)).unwrap(), expected, "{name}");
    }
    let poison = text(&format!("{PACKAGE}/artifacts/poison-matrix.md"));
    for required in [
        "WB14 suction substitution",
        "Ksat used directly as current K",
        "S_psi used for K",
        "missing root-tissue path",
        "root path aliased to dxroot",
        "caller-created receipt",
    ] {
        assert!(poison.contains(required), "missing poison {required}");
    }
}

#[test]
fn independent_calculator_schemas_and_manifest_execute() {
    let status = Command::new(root().join(".venv/bin/python"))
        .arg(
            root()
                .join(PACKAGE)
                .join("tools/validate_authority_artifacts.py"),
        )
        .status()
        .unwrap();
    assert!(status.success());
}
