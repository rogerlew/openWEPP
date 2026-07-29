use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use openwepp_management_schema::parse_management_yaml_from_path;
use sha2::{Digest, Sha256};

const CONTRACT_PLANT: &str = "docs/specifications/science-contracts/contracts/SC-PLANT-001.md";
const CONTRACT_RESIDUE: &str = "docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md";
const BASE_MANAGEMENT: &str =
    "tests/fixtures/infile/management/canonical_forest_nonzero_ow_lanuse_1.man.yaml";
const VEGETATION_BYTES: &[u8] = b"functional_class\nneedleleaf_evergreen\nbroadleaf_deciduous\n";
const VEGETATION_DIGEST: &str = "68abc74fbf291ea7b33122e8bf267563e0811dbaee7919d8e9a21327d60a4a5a";

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

fn unique_temp_dir(label: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock must follow epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("openwepp-{label}-{nonce}"));
    fs::create_dir_all(&path).expect("create isolated fixture directory");
    path
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn forcing_block(digest: &str, mode: &str, original_resolution: &str) -> String {
    format!(
        r#"    surface_litter_forcing:
      vegetation:
        functional_classes:
          - needleleaf_evergreen
          - broadleaf_deciduous
        authority:
          source_identity: fixture vegetation classification
          source_uri_or_path: vegetation.csv
          access_or_version_date: "2026-07-28"
          claim_anchor: complete file
          digest_algorithm: sha256
          source_digest: {VEGETATION_DIGEST}
      needle:
        status: complete
        payload:
          mode: {mode}
          support_start: "2020-01-01"
          support_end: "2020-12-31"
          calendar: proleptic_gregorian
          species_or_functional_type: needleleaf_evergreen
          included_material: fallen needles
          excluded_material: cones, bark, and woody material
          mass_basis:
            state: dry_to_constant_mass
            drying_temperature_c: 65.0
            constant_mass_criterion: fixture dry-mass definition
            horizontal_area_basis: true
            units: kg_dry_mass_m2_day
          spatial_support:
            site_or_plot: Forest_Management
            ofe_binding: 1
          authority:
            source_identity: prescribed needle fixture
            source_uri_or_path: needle.csv
            access_or_version_date: "2026-07-28"
            claim_anchor: complete file
            digest_algorithm: sha256
            source_digest: {digest}
          original_observation:
            support_start: "2020-01-01"
            support_end: "2020-12-31"
            resolution: {original_resolution}
            units: kg_dry_mass_m2_day
          executable_forcing:
            path: needle.csv
            digest_algorithm: sha256
            executable_digest: {digest}
      fine_woody:
        status: not_represented
"#
    )
}

fn install_management(
    directory: &Path,
    digest: &str,
    mode: &str,
    original_resolution: &str,
) -> PathBuf {
    let source = fs::read_to_string(repository_root().join(BASE_MANAGEMENT))
        .expect("read base management fixture");
    fs::write(directory.join("vegetation.csv"), VEGETATION_BYTES)
        .expect("write vegetation classification");
    let insertion = format!(
        "{}    cf: 5.0",
        forcing_block(digest, mode, original_resolution)
    );
    let management = source.replacen("    cf: 5.0", &insertion, 1);
    assert_ne!(management, source, "forcing block insertion must occur");
    let path = directory.join("forest.man.yaml");
    fs::write(&path, management).expect("write management fixture");
    path
}

fn rewrite_management(path: &Path, from: &str, to: &str) {
    let source = fs::read_to_string(path).expect("read generated management");
    let rewritten = source.replacen(from, to, 1);
    assert_ne!(rewritten, source, "management rewrite must match");
    fs::write(path, rewritten).expect("rewrite generated management");
}

#[test]
fn contracts_bind_external_source_and_parallel_residue_closure() {
    let root = repository_root();
    let plant = fs::read_to_string(root.join(CONTRACT_PLANT)).expect("read plant contract");
    let residue = fs::read_to_string(root.join(CONTRACT_RESIDUE)).expect("read residue contract");

    for required in [
        "INV-PLANT-039",
        "Q_litter(d) = L_leaf(d) + N_ext(d) + W_ext(d)",
        "AUTHORITY_MISSING / NOT_CALIBRATION_READY / NOT_ASSESSED",
    ] {
        assert!(
            plant.contains(required),
            "plant contract missing {required}"
        );
    }
    for required in [
        "INV-RESIDUE-022",
        "S_pre = S + Q + O_s",
        "I_pre = I + Q + O_i",
        "R_pre = R + Q + O_r",
    ] {
        assert!(
            residue.contains(required),
            "residue contract missing {required}"
        );
    }
}

#[test]
fn prescribed_identity_forcing_is_hash_verified_and_hydrated() {
    let directory = unique_temp_dir("litter-boundary-valid");
    let bytes = b"date,deposited_kg_m2\n2020-01-01,0.002\n2020-12-31,0\n";
    fs::write(directory.join("needle.csv"), bytes).expect("write forcing");
    let digest = sha256_hex(bytes);
    let path = install_management(&directory, &digest, "prescribed_scenario", "exact_daily");

    let document = parse_management_yaml_from_path(path).expect("valid forcing must parse");
    let yaml = serde_yaml::to_string(&document).expect("serialize hydrated document");
    assert!(yaml.contains("surface_litter_forcing"));
    assert!(yaml.contains("needle.csv"));
}

#[test]
fn wrong_digest_fails_before_forcing_publication() {
    let directory = unique_temp_dir("litter-boundary-digest");
    let bytes = b"date,deposited_kg_m2\n2020-01-01,0.002\n";
    fs::write(directory.join("needle.csv"), bytes).expect("write forcing");
    let path = install_management(
        &directory,
        &"0".repeat(64),
        "prescribed_scenario",
        "exact_daily",
    );

    let error = parse_management_yaml_from_path(path).expect_err("wrong digest must fail");
    assert!(error.to_string().contains("digest"));
}

#[test]
fn interval_observation_cannot_claim_measured_daily_forcing() {
    let directory = unique_temp_dir("litter-boundary-interval");
    let bytes = b"date,deposited_kg_m2\n2020-01-01,0.002\n";
    fs::write(directory.join("needle.csv"), bytes).expect("write forcing");
    let digest = sha256_hex(bytes);
    let path = install_management(&directory, &digest, "measured_daily", "interval");

    let error =
        parse_management_yaml_from_path(path).expect_err("interval-as-daily must fail closed");
    assert!(error.to_string().contains("exact_daily"));
}

#[test]
fn measured_daily_requires_an_explicit_record_for_every_supported_date() {
    let directory = unique_temp_dir("litter-boundary-exhaustive");
    let bytes = b"date,deposited_kg_m2\n2020-01-01,0.002\n";
    fs::write(directory.join("needle.csv"), bytes).expect("write forcing");
    let digest = sha256_hex(bytes);
    let path = install_management(&directory, &digest, "measured_daily", "exact_daily");

    let error = parse_management_yaml_from_path(path).expect_err("missing measured days must fail");
    assert!(error.to_string().contains("every supported date"));
}

#[test]
fn exhaustive_measured_daily_forcing_is_accepted() {
    let directory = unique_temp_dir("litter-boundary-measured-valid");
    let bytes = b"date,deposited_kg_m2\n2020-01-01,0.002\n2020-01-02,0\n";
    fs::write(directory.join("needle.csv"), bytes).expect("write forcing");
    let digest = sha256_hex(bytes);
    let path = install_management(&directory, &digest, "measured_daily", "exact_daily");
    rewrite_management(
        &path,
        "support_end: \"2020-12-31\"",
        "support_end: \"2020-01-02\"",
    );
    rewrite_management(
        &path,
        "support_end: \"2020-12-31\"",
        "support_end: \"2020-01-02\"",
    );

    parse_management_yaml_from_path(path).expect("exhaustive measured daily must parse");
}

#[test]
fn noncanonical_csv_bytes_fail_closed() {
    let directory = unique_temp_dir("litter-boundary-csv");
    let bytes = b"date,deposited_kg_m2\r\n2020-01-01,0.002\r\n";
    fs::write(directory.join("needle.csv"), bytes).expect("write forcing");
    let digest = sha256_hex(bytes);
    let path = install_management(&directory, &digest, "prescribed_scenario", "exact_daily");

    let error = parse_management_yaml_from_path(path).expect_err("CRLF forcing must fail");
    assert!(error.to_string().contains("LF"));
}

#[test]
fn unauthenticated_vegetation_classification_fails_closed() {
    let directory = unique_temp_dir("litter-boundary-vegetation-digest");
    let bytes = b"date,deposited_kg_m2\n2020-01-01,0.002\n";
    fs::write(directory.join("needle.csv"), bytes).expect("write forcing");
    let digest = sha256_hex(bytes);
    let path = install_management(&directory, &digest, "prescribed_scenario", "exact_daily");
    rewrite_management(&path, VEGETATION_DIGEST, &"0".repeat(64));

    let error = parse_management_yaml_from_path(path).expect_err("classification drift must fail");
    assert!(error.to_string().contains("digest"));
}

#[test]
fn needle_material_cannot_claim_a_broadleaf_class() {
    let directory = unique_temp_dir("litter-boundary-material");
    let bytes = b"date,deposited_kg_m2\n2020-01-01,0.002\n";
    fs::write(directory.join("needle.csv"), bytes).expect("write forcing");
    let digest = sha256_hex(bytes);
    let path = install_management(&directory, &digest, "prescribed_scenario", "exact_daily");
    rewrite_management(
        &path,
        "species_or_functional_type: needleleaf_evergreen",
        "species_or_functional_type: broadleaf_deciduous",
    );

    let error = parse_management_yaml_from_path(path).expect_err("material mismatch must fail");
    assert!(error.to_string().contains("declared tissue material"));
}

#[test]
fn identity_payload_requires_matching_original_support() {
    let directory = unique_temp_dir("litter-boundary-support");
    let bytes = b"date,deposited_kg_m2\n2020-01-01,0.002\n";
    fs::write(directory.join("needle.csv"), bytes).expect("write forcing");
    let digest = sha256_hex(bytes);
    let path = install_management(&directory, &digest, "prescribed_scenario", "exact_daily");
    rewrite_management(
        &path,
        "original_observation:\n            support_start: \"2020-01-01\"\n            support_end: \"2020-12-31\"",
        "original_observation:\n            support_start: \"2020-01-01\"\n            support_end: \"2020-12-30\"",
    );

    let error = parse_management_yaml_from_path(path).expect_err("support mismatch must fail");
    assert!(error.to_string().contains("support to match"));
}

#[test]
fn oven_dry_mass_requires_a_duration_or_constant_mass_criterion() {
    let directory = unique_temp_dir("litter-boundary-drying");
    let bytes = b"date,deposited_kg_m2\n2020-01-01,0.002\n";
    fs::write(directory.join("needle.csv"), bytes).expect("write forcing");
    let digest = sha256_hex(bytes);
    let path = install_management(&directory, &digest, "prescribed_scenario", "exact_daily");
    rewrite_management(
        &path,
        "state: dry_to_constant_mass\n            drying_temperature_c: 65.0\n            constant_mass_criterion: fixture dry-mass definition",
        "state: oven_dry\n            drying_temperature_c: 65.0",
    );

    let error = parse_management_yaml_from_path(path).expect_err("drying provenance must fail");
    assert!(error.to_string().contains("drying_duration_hours"));
}

#[test]
fn site_and_ofe_mismatches_fail_closed() {
    let bytes = b"date,deposited_kg_m2\n2020-01-01,0.002\n";
    for (label, from, to, expected) in [
        (
            "site",
            "site_or_plot: Forest_Management",
            "site_or_plot: Other_Site",
            "metadata.name",
        ),
        ("ofe", "ofe_binding: 1", "ofe_binding: 2", "declared OFE"),
    ] {
        let directory = unique_temp_dir(&format!("litter-boundary-{label}"));
        fs::write(directory.join("needle.csv"), bytes).expect("write forcing");
        let digest = sha256_hex(bytes);
        let path = install_management(&directory, &digest, "prescribed_scenario", "exact_daily");
        rewrite_management(&path, from, to);
        let error = parse_management_yaml_from_path(path).expect_err("binding mismatch must fail");
        assert!(error.to_string().contains(expected));
    }
}

#[test]
fn sidecar_path_escape_fails_closed() {
    let directory = unique_temp_dir("litter-boundary-path");
    let bytes = b"date,deposited_kg_m2\n2020-01-01,0.002\n";
    fs::write(directory.join("needle.csv"), bytes).expect("write forcing");
    let digest = sha256_hex(bytes);
    let path = install_management(&directory, &digest, "prescribed_scenario", "exact_daily");
    rewrite_management(
        &path,
        "source_uri_or_path: needle.csv",
        "source_uri_or_path: ../needle.csv",
    );
    rewrite_management(&path, "path: needle.csv", "path: ../needle.csv");

    let error = parse_management_yaml_from_path(path).expect_err("path escape must fail");
    assert!(error.to_string().contains("confined relative"));
}

#[test]
fn duplicate_dates_and_negative_mass_fail_closed() {
    for (label, bytes, expected) in [
        (
            "duplicate",
            &b"date,deposited_kg_m2\n2020-01-01,0.002\n2020-01-01,0.003\n"[..],
            "strictly increasing",
        ),
        (
            "negative",
            &b"date,deposited_kg_m2\n2020-01-01,-0.002\n"[..],
            "nonnegative",
        ),
    ] {
        let directory = unique_temp_dir(&format!("litter-boundary-{label}"));
        fs::write(directory.join("needle.csv"), bytes).expect("write forcing");
        let digest = sha256_hex(bytes);
        let path = install_management(&directory, &digest, "prescribed_scenario", "exact_daily");
        let error = parse_management_yaml_from_path(path).expect_err("invalid CSV must fail");
        assert!(error.to_string().contains(expected));
    }
}

#[test]
fn derived_payload_is_rejected_until_authority_is_admitted() {
    let directory = unique_temp_dir("litter-boundary-derived");
    let bytes = b"date,deposited_kg_m2\n2020-01-01,0.002\n";
    fs::write(directory.join("needle.csv"), bytes).expect("write forcing");
    let digest = sha256_hex(bytes);
    let path = install_management(&directory, &digest, "prescribed_scenario", "exact_daily");
    rewrite_management(
        &path,
        "      fine_woody:\n        status: not_represented",
        "          derivation:\n            identity: forbidden fixture\n            version: \"1\"\n            inputs:\n              - source\n            transformation_authority: none admitted\n      fine_woody:\n        status: not_represented",
    );

    let error = parse_management_yaml_from_path(path).expect_err("derived payload must fail");
    assert!(error.to_string().contains("identity-only"));
}

#[test]
fn not_represented_tissue_cannot_carry_a_numeric_payload() {
    let directory = unique_temp_dir("litter-boundary-not-represented");
    let bytes = b"date,deposited_kg_m2\n2020-01-01,0.002\n";
    fs::write(directory.join("needle.csv"), bytes).expect("write forcing");
    let digest = sha256_hex(bytes);
    let path = install_management(&directory, &digest, "prescribed_scenario", "exact_daily");
    rewrite_management(
        &path,
        "      needle:\n        status: complete",
        "      needle:\n        status: not_represented",
    );

    let error =
        parse_management_yaml_from_path(path).expect_err("numeric missing tissue must fail");
    assert!(
        error
            .to_string()
            .contains("must not carry a numeric payload")
    );
}
