use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn repo_file(path: &str) -> String {
    let repo_root = env!("CARGO_MANIFEST_DIR");
    let full_path = Path::new(repo_root).join(path);
    fs::read_to_string(&full_path)
        .unwrap_or_else(|error| panic!("expected readable file {}: {error}", full_path.display()))
}

fn repo_path(path: &str) -> PathBuf {
    let repo_root = env!("CARGO_MANIFEST_DIR");
    Path::new(repo_root).join(path)
}

fn parse_lock_entries(lock_path: &Path) -> Vec<(String, String)> {
    let content = fs::read_to_string(lock_path).unwrap_or_else(|error| {
        panic!(
            "expected readable lock file {}: {error}",
            lock_path.display()
        )
    });
    let mut entries = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let mut fields = trimmed.split_whitespace();
        let digest = fields
            .next()
            .unwrap_or_else(|| panic!("malformed lock entry in {}: {line}", lock_path.display()));
        let fixture = fields
            .next()
            .unwrap_or_else(|| panic!("missing fixture path in {}: {line}", lock_path.display()));
        assert!(
            digest.len() == 64
                && digest
                    .chars()
                    .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()),
            "lock digest must be 64-char lowercase hex in {}: {}",
            lock_path.display(),
            digest
        );
        entries.push((digest.to_string(), fixture.to_string()));
    }
    entries
}

fn assert_sha256_lock_passes(fixture_root: &Path) {
    let status = Command::new("sha256sum")
        .arg("--check")
        .arg("--strict")
        .arg("fixtures.sha256")
        .current_dir(fixture_root)
        .status()
        .unwrap_or_else(|error| {
            panic!(
                "failed to run sha256sum in {}: {error}",
                fixture_root.display()
            )
        });
    assert!(
        status.success(),
        "fixtures.sha256 check failed in {}",
        fixture_root.display()
    );
}

#[test]
fn auth06_schema_requires_fixture_hash_and_source_provenance_fields() {
    let schema = repo_file("docs/specifications/external-authority/suite-schema.md");
    let template = repo_file("docs/specifications/external-authority/suite-template.md");
    let authority_model = repo_file("docs/specifications/correctness-authority-model.md");

    assert!(
        schema.contains("| `hash` | string (`sha256`) | yes |")
            && schema.contains("| `source_repo` | string | yes |")
            && schema.contains("| `source_commit` | string | yes |")
            && schema.contains("| `source_path` | string | yes |")
            && schema.contains("| `source_sha256` | string (`sha256`) | yes |")
            && schema.contains("| `transform_note` | string | yes |"),
        "suite schema must require fixture hash and source provenance fields"
    );
    assert!(
        schema.contains("fixtures.sha256") && schema.contains("fixtures.provenance.yaml"),
        "suite schema must require lockfile and provenance sidecars"
    );
    assert!(
        template.contains("source_repo:")
            && template.contains("source_commit:")
            && template.contains("source_path:")
            && template.contains("source_sha256:")
            && template.contains("transform_note:"),
        "suite template must include fixture provenance keys"
    );
    assert!(
        authority_model.contains("Fixture integrity metadata per fixture"),
        "correctness authority model must include fixture-integrity metadata requirements"
    );
}

#[test]
fn auth06_active_level4_and_level3_suites_publish_fixture_hashes_and_provenance_sidecars() {
    let registry = repo_file("docs/specifications/external-authority/registry.yaml");
    let suite_docs = [
        "docs/specifications/external-authority/suites/cas_l4_soil_fc_minus33_001.md",
        "docs/specifications/external-authority/suites/cas_l4_soil_wp_minus1500_001.md",
        "docs/specifications/external-authority/suites/cas_l4_watbal_relax_to_fc_001.md",
        "docs/specifications/external-authority/suites/cas_l4_soil_fc_direct_theta_minus33_cohort_001.md",
        "docs/specifications/external-authority/suites/cas_l4_subhyd_withdrawal_soilwater_cap_001.md",
        "docs/specifications/external-authority/suites/cas_l3_subhyd_solwpv_fcdep_branch_001.md",
    ];
    let fixture_roots = [
        "tests/fixtures/constitutive/cas_l4_soil_fc_minus33_001",
        "tests/fixtures/constitutive/cas_l4_soil_wp_minus1500_001",
        "tests/fixtures/constitutive/cas_l4_watbal_relax_to_fc_001",
        "tests/fixtures/constitutive/cas_l4_soil_fc_direct_theta_minus33_cohort_001",
        "tests/fixtures/constitutive/cas_l4_subhyd_withdrawal_soilwater_cap_001",
        "tests/fixtures/constitutive/cas_l3_subhyd_solwpv_fcdep_branch_001",
    ];

    for fixture_root in fixture_roots {
        assert!(
            registry.contains(&format!("fixture_root: {fixture_root}"))
                && registry.contains(&format!("fixture_lock: {fixture_root}/fixtures.sha256"))
                && registry.contains(&format!(
                    "fixture_provenance: {fixture_root}/fixtures.provenance.yaml"
                )),
            "registry must declare fixture root, lock, and provenance sidecar for active suite {fixture_root}"
        );

        let fixture_root_path = repo_path(fixture_root);
        let lock_path = fixture_root_path.join("fixtures.sha256");
        let provenance_path = fixture_root_path.join("fixtures.provenance.yaml");
        assert!(
            lock_path.exists(),
            "missing fixture lock file {}",
            lock_path.display()
        );
        assert!(
            provenance_path.exists(),
            "missing fixture provenance file {}",
            provenance_path.display()
        );

        assert_sha256_lock_passes(&fixture_root_path);
        let provenance_text = fs::read_to_string(&provenance_path).unwrap_or_else(|error| {
            panic!(
                "expected readable provenance file {}: {error}",
                provenance_path.display()
            )
        });
        for (digest, fixture_file) in parse_lock_entries(&lock_path) {
            assert!(
                provenance_text.contains(&format!("- path: {fixture_file}")),
                "provenance file {} missing fixture path entry {}",
                provenance_path.display(),
                fixture_file
            );
            assert!(
                provenance_text.contains(&format!("sha256: {digest}")),
                "provenance file {} missing sha256 entry {}",
                provenance_path.display(),
                digest
            );
        }
        for required_key in [
            "source_repo:",
            "source_commit:",
            "source_path:",
            "source_sha256:",
            "transform_note:",
        ] {
            assert!(
                provenance_text.contains(required_key),
                "provenance file {} missing required key {}",
                provenance_path.display(),
                required_key
            );
        }
    }

    for suite_doc in suite_docs {
        let suite_text = repo_file(suite_doc);
        assert!(
            suite_text.contains("hash:")
                && suite_text.contains("source_repo:")
                && suite_text.contains("source_commit:")
                && suite_text.contains("source_path:")
                && suite_text.contains("source_sha256:")
                && suite_text.contains("transform_note:"),
            "suite doc {suite_doc} must include fixture hash and provenance fields"
        );
    }
}

#[test]
fn auth06_tamper_detection_rejects_modified_lock_hash() {
    let fixture_root = repo_path("tests/fixtures/constitutive/cas_l4_soil_fc_minus33_001");
    let lock_path = fixture_root.join("fixtures.sha256");
    let lock_text = fs::read_to_string(&lock_path).unwrap_or_else(|error| {
        panic!(
            "expected readable lock file {}: {error}",
            lock_path.display()
        )
    });
    let mut lines = lock_text.lines();
    let first_line = lines.next().expect("fixtures.sha256 must not be empty");
    let mut first_fields = first_line.split_whitespace();
    let digest = first_fields
        .next()
        .expect("first lock line must contain digest");
    let fixture_file = first_fields
        .next()
        .expect("first lock line must contain fixture path");
    let leading = if digest.starts_with('0') { '1' } else { '0' };
    let tampered_digest = format!("{leading}{}", &digest[1..]);

    let mut tampered_content = format!("{tampered_digest}  {fixture_file}\n");
    for line in lines {
        tampered_content.push_str(line);
        tampered_content.push('\n');
    }

    let temp_name = format!("fixtures.auth06.bad.{}.sha256", std::process::id());
    let temp_path = fixture_root.join(&temp_name);
    fs::write(&temp_path, tampered_content).unwrap_or_else(|error| {
        panic!(
            "failed to write tampered lock fixture {}: {error}",
            temp_path.display()
        )
    });

    let status = Command::new("sha256sum")
        .arg("--check")
        .arg("--strict")
        .arg(&temp_name)
        .current_dir(&fixture_root)
        .status()
        .unwrap_or_else(|error| {
            panic!(
                "failed to run tamper-detection check in {}: {error}",
                fixture_root.display()
            )
        });

    let _ = fs::remove_file(&temp_path);
    assert!(
        !status.success(),
        "tampered lock hash must fail sha256 verification"
    );
}

#[test]
fn auth06_release_gate_script_enforces_fixture_integrity_before_lane_execution() {
    let script = repo_file("tools/release/run_release_candidate_gates.sh");
    let release_readme = repo_file("tools/release/README.md");
    let release_runbook = repo_file("docs/governance/openwepp-release-procedure-draft.md");

    assert!(
        script.contains("active_suite_fixture_roots")
            && script.contains("run_authority_fixture_integrity_gate")
            && script.contains("sha256sum --check --strict fixtures.sha256")
            && script.contains("fixtures.provenance.yaml"),
        "release gate script must enforce fixture lock and provenance checks"
    );
    assert!(
        script.contains("fixture_integrity_enforced: true")
            && script.contains("run_authority_fixture_integrity_gate")
            && script.contains("evaluating authority-suite lanes"),
        "release gate report and control flow must include fixture integrity gate"
    );
    assert!(
        release_readme.contains("fixtures.sha256")
            && release_readme.contains("fixtures.provenance.yaml"),
        "release tooling README must document fixture integrity enforcement"
    );
    assert!(
        release_runbook.contains(
            "Before lane execution, release-gate automation must verify fixture integrity"
        ),
        "release runbook must capture fixture integrity gate policy"
    );
}
