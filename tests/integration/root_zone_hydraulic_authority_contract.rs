use serde_json::Value;
use std::{fs, path::PathBuf, process::Command};

const PACKAGE: &str = "docs/work-packages/20260819-root-zone-hydraulic-owner-authority-001";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn text(path: &str) -> String {
    fs::read_to_string(root().join(path)).unwrap()
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
    for vector in accepted {
        for value in vector["expected"].as_object().unwrap().values() {
            let hex = value.as_str().unwrap();
            assert_eq!(hex.len(), 16);
            assert!(hex.bytes().all(|b| b.is_ascii_hexdigit()));
        }
    }
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
