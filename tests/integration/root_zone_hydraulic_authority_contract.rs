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

fn capacity_one_bit_limit(capacity: f64) -> f64 {
    f64::from_bits(capacity.to_bits() + 1)
}

fn digest(value: &Value) -> String {
    let mut bytes = serde_json::to_vec(value).unwrap();
    bytes.push(b'\n');
    format!("{:x}", Sha256::digest(bytes))
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
