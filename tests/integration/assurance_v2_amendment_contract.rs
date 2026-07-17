use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use openwepp_assurance::{
    V2AmendMode, amend_attribution, amend_attribution_at_generation, amend_lifecycle,
    amend_principal, amend_role, inspect_report, rebind_implementation, recover_amendment,
    sha256_bytes, verify_generation,
};

const GROUNDWATER: &str = "linear-groundwater-reservoir-recurrence";
const SNOW: &str = "snow-and-frozen-soil-process-evaluation";
const BASE_REF: &str = "15763d7f6d5d4125333d9b7583424c714f5f5ea4";

#[test]
fn inspect_exposes_layered_identity_without_mutation() {
    let root = repository_root();
    let before = fs::read(root.join("assurance/v2/identity.lock.json")).unwrap();
    let inspection = inspect_report(&root, SNOW).unwrap();
    assert_eq!(inspection.report_id, SNOW);
    for root in [
        inspection.science_root,
        inspection.communication_root,
        inspection.attribution_root,
        inspection.review_governance_root,
        inspection.content_review_subject_root,
    ] {
        assert_eq!(root.len(), 64);
    }
    assert_eq!(
        before,
        fs::read(root.join("assurance/v2/identity.lock.json")).unwrap()
    );
}

#[test]
fn attribution_check_is_read_only_and_apply_is_layer_proportional() {
    let fixture = fixture("assurance-amend-attribution");
    let before_tree = capture_tree(&fixture.path.join("assurance/v2"));
    let before = inspect_report(&fixture.path, SNOW).unwrap();
    let checked = amend_attribution(
        &fixture.path,
        "roger-lew",
        Some("Roger A. Lew"),
        None,
        V2AmendMode::Check,
    )
    .unwrap();
    assert!(checked.changed);
    assert_eq!(
        before_tree,
        capture_tree(&fixture.path.join("assurance/v2"))
    );

    let applied = amend_attribution(
        &fixture.path,
        "roger-lew",
        Some("Roger A. Lew"),
        None,
        V2AmendMode::Apply,
    )
    .unwrap();
    assert_eq!(checked, applied);
    assert_eq!(applied.impact_class, "metadata-fast");
    assert_eq!(applied.affected_reports, vec![SNOW]);
    let after = inspect_report(&fixture.path, SNOW).unwrap();
    assert_eq!(before.science_root, after.science_root);
    assert_eq!(before.communication_root, after.communication_root);
    assert_ne!(before.attribution_root, after.attribution_root);
    assert_eq!(
        before.content_review_subject_root,
        after.content_review_subject_root
    );
    assert!(
        fs::read_to_string(fixture.path.join("assurance/v2/principals.yaml"))
            .unwrap()
            .contains("display_name: Roger A. Lew")
    );

    let repeated = amend_attribution(
        &fixture.path,
        "roger-lew",
        Some("Roger A. Lew"),
        None,
        V2AmendMode::Apply,
    )
    .unwrap();
    assert!(!repeated.changed);
    assert!(repeated.gate_ids.is_empty());
}

#[test]
fn role_assignment_is_typed_receipted_and_idempotent() {
    let fixture = fixture("assurance-amend-role");
    let request = br"schema_version: 1
operation: role_assignment
principal_id: roger-lew
assignments:
  report_lead: true
  material_producer: false
  build_maintainer: false
attestation:
  authority: ASSURE-MAINT-01 integration contract
  statement: Assign the existing eligible report lead to this draft fixture.
  recorded_on: 2026-07-16
";
    let before = inspect_report(&fixture.path, GROUNDWATER).unwrap();
    let checked = amend_role(&fixture.path, GROUNDWATER, request, V2AmendMode::Check).unwrap();
    assert!(checked.changed);
    let applied = amend_role(&fixture.path, GROUNDWATER, request, V2AmendMode::Apply).unwrap();
    assert_eq!(checked, applied);
    let after = inspect_report(&fixture.path, GROUNDWATER).unwrap();
    assert_eq!(before.science_root, after.science_root);
    assert_ne!(before.review_governance_root, after.review_governance_root);
    assert_eq!(applied.impact_class, "governance-focused");
    assert!(
        applied
            .affected_paths
            .iter()
            .all(|path| path.starts_with("assurance/v2/"))
    );

    let repeated = amend_role(&fixture.path, GROUNDWATER, request, V2AmendMode::Apply).unwrap();
    assert!(!repeated.changed);
}

#[test]
fn principal_and_lifecycle_checks_recalculate_identical_candidates() {
    let fixture = fixture("assurance-amend-deterministic-candidates");
    let principal = br"schema_version: 1
principal_id: roger-lew
display_name: Roger Lew, deterministic fixture
affiliations: []
roles: null
identity_authority: null
identity_reference: null
attestation:
  authority: ASSURE-MAINT-01 integration contract
  statement: Exercise deterministic principal candidate calculation.
  recorded_on: 2026-07-16
";
    let first = amend_principal(&fixture.path, principal, V2AmendMode::Check).unwrap();
    let second = amend_principal(&fixture.path, principal, V2AmendMode::Check).unwrap();
    assert_eq!(first, second);

    let withdrawal = br"schema_version: 1
event_type: withdrawal
principal_id: roger-lew
decision: withdrawn
rationale: Exercise deterministic terminal lifecycle calculation.
recorded_on: 2026-07-16
authority_source: ASSURE-MAINT-01 integration contract
predecessor_event_ids: []
";
    let first = amend_lifecycle(&fixture.path, SNOW, withdrawal, V2AmendMode::Check).unwrap();
    let second = amend_lifecycle(&fixture.path, SNOW, withdrawal, V2AmendMode::Check).unwrap();
    assert_eq!(first, second);
    let applied = amend_lifecycle(&fixture.path, SNOW, withdrawal, V2AmendMode::Apply).unwrap();
    assert_eq!(first, applied);
    let inspection = inspect_report(&fixture.path, SNOW).unwrap();
    assert_eq!(inspection.lifecycle, "WITHDRAWN");
    assert!(inspection.approval_lock_root.is_none());
}

#[test]
fn supersession_is_terminal_and_requires_a_named_successor() {
    let fixture = fixture("assurance-amend-supersession");
    let missing = br"schema_version: 1
event_type: supersession
principal_id: roger-lew
decision: superseded
rationale: Negative supersession fixture.
recorded_on: 2026-07-16
authority_source: ASSURE-MAINT-01 integration contract
predecessor_event_ids: []
";
    assert!(
        amend_lifecycle(&fixture.path, SNOW, missing, V2AmendMode::Check)
            .unwrap_err()
            .to_string()
            .contains("superseding report")
    );
    let request = br"schema_version: 1
event_type: supersession
principal_id: roger-lew
decision: superseded
rationale: Exercise terminal supersession mechanics.
recorded_on: 2026-07-16
authority_source: ASSURE-MAINT-01 integration contract
predecessor_event_ids: []
superseding_report_id: future-snow-process-report
";
    amend_lifecycle(&fixture.path, SNOW, request, V2AmendMode::Apply).unwrap();
    let inspection = inspect_report(&fixture.path, SNOW).unwrap();
    assert_eq!(inspection.lifecycle, "SUPERSEDED");
    assert!(inspection.approval_lock_root.is_none());
}

#[test]
fn stale_optional_generation_rejects_check_and_apply_without_writes() {
    let fixture = fixture("assurance-amend-stale-generation");
    let before = capture_tree(&fixture.path.join("assurance/v2"));
    for mode in [V2AmendMode::Check, V2AmendMode::Apply] {
        let error = amend_attribution_at_generation(
            &fixture.path,
            "roger-lew",
            Some("Roger Lew, stale generation fixture"),
            None,
            mode,
            Some("not-the-current-generation"),
        )
        .expect_err("stale generation must fail closed");
        assert!(error.to_string().contains("stale generation"));
        assert_eq!(before, capture_tree(&fixture.path.join("assurance/v2")));
    }
}

#[test]
fn production_generation_chain_and_recovery_inspection_are_current() {
    let root = repository_root();
    let verified = verify_generation(&root, BASE_REF).unwrap();
    assert!(verified.contains("generation verification: PASS"));
    let recovery = recover_amendment(&root, openwepp_assurance::V2RecoveryAction::Inspect).unwrap();
    assert!(recovery.contains("pending_cleanup: false"));
}

#[test]
fn implementation_rebind_adopts_only_the_finite_contract_surface() {
    let fixture = fixture("assurance-implementation-rebind");
    let readme = fixture.path.join("assurance/v2/README.md");
    let mut bytes = fs::read(&readme).unwrap();
    bytes.extend_from_slice(b"\nImplementation contract fixture note.\n");
    fs::write(&readme, bytes).unwrap();

    let checked = rebind_implementation(&fixture.path, V2AmendMode::Check).unwrap();
    assert!(checked.changed);
    assert_eq!(checked.impact_class, "scientific-full");
    assert_eq!(checked.gate_ids, ["assurance-implementation-package-v1"]);
    assert!(checked.gate_argv.is_empty());
    assert!(
        checked
            .affected_paths
            .contains(&"assurance/v2/README.md".to_owned())
    );
    let applied = rebind_implementation(&fixture.path, V2AmendMode::Apply).unwrap();
    assert_eq!(checked.new_generation_id, applied.new_generation_id);
    let repeated = rebind_implementation(&fixture.path, V2AmendMode::Check).unwrap();
    assert!(!repeated.changed);
    let manuscript = fixture
        .path
        .join(format!("assurance/v2/reports/{GROUNDWATER}/manuscript.md"));
    let mut manuscript_bytes = fs::read(&manuscript).unwrap();
    manuscript_bytes.push(b'\n');
    fs::write(manuscript, manuscript_bytes).unwrap();
    assert!(
        rebind_implementation(&fixture.path, V2AmendMode::Check)
            .expect_err("noncontract drift must remain rejected")
            .to_string()
            .contains("generated identity member changed")
    );
}

#[test]
fn generated_identity_event_and_receipt_schemas_accept_current_artifacts() {
    let root = repository_root();
    assert_schema_accepts(
        &root.join("assurance/v2/schemas/identity-lock.schema.json"),
        &root.join("assurance/v2/identity.lock.json"),
    );
    for report in [GROUNDWATER, SNOW] {
        assert_schema_accepts(
            &root.join("assurance/v2/schemas/review-lock.schema.json"),
            &root.join(format!("assurance/v2/reports/{report}/review.lock.json")),
        );
        let events = root.join(format!("assurance/v2/reports/{report}/review-events"));
        if events.is_dir() {
            for event in fs::read_dir(events).unwrap() {
                assert_schema_accepts(
                    &root.join("assurance/v2/schemas/review-event.schema.json"),
                    &event.unwrap().path(),
                );
            }
        }
    }
    for receipt in fs::read_dir(root.join("assurance/v2/transactions")).unwrap() {
        assert_schema_accepts(
            &root.join("assurance/v2/schemas/transaction-receipt.schema.json"),
            &receipt.unwrap().path(),
        );
    }
}

#[test]
fn focused_runner_rejects_an_off_archive_receipt() {
    let root = repository_root();
    let generation: serde_json::Value =
        serde_json::from_slice(&fs::read(root.join("assurance/v2/identity.lock.json")).unwrap())
            .unwrap();
    let generation = generation["generation_id"].as_str().unwrap();
    let archived = fs::read_dir(root.join("assurance/v2/transactions"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .find(|path| {
            serde_json::from_slice::<serde_json::Value>(&fs::read(path).unwrap()).unwrap()
                ["new_generation_id"]
                .as_str()
                == Some(generation)
        })
        .unwrap();
    let scratch = Scratch::new("assurance-forged-receipt");
    let forged = scratch.path.join(archived.file_name().unwrap());
    fs::copy(archived, &forged).unwrap();
    let output = Command::new(".venv/bin/python")
        .args([
            "tools/local_ci/run_assurance_amendment.py",
            "--receipt",
            forged.to_str().unwrap(),
        ])
        .current_dir(&root)
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("regular direct member of the active transaction archive")
    );
}

#[test]
fn focused_runner_rejects_forged_receipt_matrix() {
    let (gate_root, mut gate) = focused_runner_fixture("runner-gate-mismatch");
    gate["gate_ids"] = serde_json::json!(["wrong-focused-gate-v1"]);
    assert_runner_rejected(
        &gate_root,
        &write_canonical_receipt(&gate_root.path, &gate),
        "gate ID does not match",
    );

    let (argv_root, mut argv) = focused_runner_fixture("runner-argv-mismatch");
    argv["gate_argv"] = serde_json::json!([["cargo", "nextest", "run", "--profile", "quick"]]);
    assert_runner_rejected(
        &argv_root,
        &write_canonical_receipt(&argv_root.path, &argv),
        "argv does not match",
    );

    let (escalation_root, mut escalation) = focused_runner_fixture("runner-escalation");
    escalation["gate_argv"] = serde_json::json!([["cargo", "nextest", "run", "--profile", "full"]]);
    assert_runner_rejected(
        &escalation_root,
        &write_canonical_receipt(&escalation_root.path, &escalation),
        "forbidden escalation token",
    );

    let (schema_root, mut schema) = focused_runner_fixture("runner-schema");
    schema["affected_paths"] = serde_json::json!(["assurance/v2/schemas/report.schema.json"]);
    assert_runner_rejected(
        &schema_root,
        &write_canonical_receipt(&schema_root.path, &schema),
        "schema changes require implementation-package gates",
    );

    let (noncurrent_root, mut noncurrent) = focused_runner_fixture("runner-noncurrent");
    noncurrent["new_generation_id"] = serde_json::Value::String("b".repeat(64));
    assert_runner_rejected(
        &noncurrent_root,
        &write_canonical_receipt(&noncurrent_root.path, &noncurrent),
        "does not terminate at the current generation",
    );

    let (duplicate_root, duplicate) = focused_runner_fixture("runner-duplicate");
    let selected = write_canonical_receipt(&duplicate_root.path, &duplicate);
    let mut second = duplicate.clone();
    second["operation"] = serde_json::Value::String("second-valid-transition".to_owned());
    write_canonical_receipt(&duplicate_root.path, &second);
    assert_runner_rejected(&duplicate_root, &selected, "unique archived transition");

    let (noncanonical_root, noncanonical) = focused_runner_fixture("runner-noncanonical");
    let mut bytes = serde_json::to_vec(&noncanonical).unwrap();
    bytes.push(b'\n');
    let noncanonical_path = receipt_path(&noncanonical_root.path, &bytes);
    fs::write(&noncanonical_path, bytes).unwrap();
    assert_runner_rejected(
        &noncanonical_root,
        &noncanonical_path,
        "not canonical pretty JSON",
    );

    let (symlink_root, symlinked) = focused_runner_fixture("runner-symlink");
    let outside = symlink_root.path.join("outside-receipt.json");
    fs::write(&outside, canonical_receipt_bytes(&symlinked)).unwrap();
    let symlink_path = symlink_root
        .path
        .join("assurance/v2/transactions/symlinked.json");
    std::os::unix::fs::symlink(outside, &symlink_path).unwrap();
    assert_runner_rejected(
        &symlink_root,
        &symlink_path,
        "regular direct member of the active transaction archive",
    );
}

#[test]
#[ignore = "ASSURE-MAINT-01 scaled-corpus fixture generator"]
fn generate_scaled_performance_fixture_when_requested() {
    let target = std::env::var_os("ASSURE_MAINT_SCALE_ROOT")
        .map(PathBuf::from)
        .expect("ASSURE_MAINT_SCALE_ROOT must name the disposable fixture");
    if target.exists() {
        fs::remove_dir_all(&target).unwrap();
    }
    fs::create_dir_all(&target).unwrap();
    openwepp_assurance::copy_v2_test_fixture(&repository_root(), &target).unwrap();
    let catalog_path = target.join("assurance/v2/catalog.yaml");
    let mut catalog: serde_yaml::Value =
        serde_yaml::from_slice(&fs::read(&catalog_path).unwrap()).unwrap();
    let source_entry = catalog["reports"]
        .as_sequence()
        .unwrap()
        .iter()
        .find(|entry| entry["id"].as_str() == Some(GROUNDWATER))
        .unwrap()
        .clone();
    for index in 0..98 {
        let report_id = format!("assure-maint-scaled-{index:03}");
        let source = target.join(format!("assurance/v2/reports/{GROUNDWATER}"));
        let destination = target.join(format!("assurance/v2/reports/{report_id}"));
        copy_tree(&source, &destination);
        let report_path = destination.join("report.yaml");
        let mut report: serde_yaml::Value =
            serde_yaml::from_slice(&fs::read(&report_path).unwrap()).unwrap();
        replace_yaml_strings(&mut report, GROUNDWATER, &report_id);
        fs::write(&report_path, serde_yaml::to_string(&report).unwrap()).unwrap();
        let mut entry = source_entry.clone();
        replace_yaml_strings(&mut entry, GROUNDWATER, &report_id);
        catalog["reports"].as_sequence_mut().unwrap().push(entry);
    }
    fs::write(&catalog_path, serde_yaml::to_string(&catalog).unwrap()).unwrap();
    fs::write(
        target.join("assurance/v2/scaled-corpus-32mib.bin"),
        vec![0xA5; 32 * 1024 * 1024],
    )
    .unwrap();
    openwepp_assurance::rebind_v2_test_fixture(&target).unwrap();
    let repository = openwepp_assurance::V2Repository::open(&target).unwrap();
    assert_eq!(repository.validate_all().unwrap().reports.len(), 100);
}

#[test]
#[ignore = "ASSURE-MAINT-01 current-corpus fixture generator"]
fn generate_current_performance_fixture_when_requested() {
    let target = std::env::var_os("ASSURE_MAINT_CURRENT_ROOT")
        .map(PathBuf::from)
        .expect("ASSURE_MAINT_CURRENT_ROOT must name the disposable fixture");
    if target.exists() {
        fs::remove_dir_all(&target).unwrap();
    }
    fs::create_dir_all(&target).unwrap();
    openwepp_assurance::copy_v2_test_fixture(&repository_root(), &target).unwrap();
    openwepp_assurance::rebind_v2_test_fixture(&target).unwrap();
    let repository = openwepp_assurance::V2Repository::open(&target).unwrap();
    assert_eq!(repository.validate_all().unwrap().reports.len(), 2);
}

fn assert_schema_accepts(schema_path: &Path, instance_path: &Path) {
    let schema: serde_json::Value =
        serde_json::from_slice(&fs::read(schema_path).unwrap()).unwrap();
    let instance: serde_json::Value =
        serde_json::from_slice(&fs::read(instance_path).unwrap()).unwrap();
    let validator = jsonschema::draft202012::new(&schema).unwrap();
    assert!(
        validator.is_valid(&instance),
        "{} rejected {}",
        schema_path.display(),
        instance_path.display()
    );
}

fn focused_runner_fixture(label: &str) -> (Scratch, serde_json::Value) {
    let scratch = Scratch::new(label);
    let archive = scratch.path.join("assurance/v2/transactions");
    fs::create_dir_all(&archive).unwrap();
    fs::write(
        scratch.path.join("assurance/v2/identity.lock.json"),
        format!("{{\"generation_id\":\"{}\"}}\n", "a".repeat(64)),
    )
    .unwrap();
    let receipt = serde_json::json!({
        "schema_version": 1,
        "operation": "attribution",
        "impact_class": "metadata-fast",
        "changed": true,
        "old_generation_id": "b".repeat(64),
        "new_generation_id": "a".repeat(64),
        "affected_reports": [GROUNDWATER],
        "affected_paths": ["assurance/v2/principals.yaml"],
        "invalidated_authority": [],
        "gate_ids": ["assurance-amendment-metadata-v1"],
        "gate_argv": [["cargo", "nextest", "run", "--workspace", "--profile", "assurance-amendment"]]
    });
    (scratch, receipt)
}

fn canonical_receipt_bytes(receipt: &serde_json::Value) -> Vec<u8> {
    let mut bytes = serde_json::to_vec_pretty(receipt).unwrap();
    bytes.push(b'\n');
    bytes
}

fn receipt_path(root: &Path, bytes: &[u8]) -> PathBuf {
    let mut identity = b"openwepp-assurance-amendment-receipt-v1\0".to_vec();
    identity.extend_from_slice(bytes);
    root.join(format!(
        "assurance/v2/transactions/{}.json",
        sha256_bytes(&identity)
    ))
}

fn write_canonical_receipt(root: &Path, receipt: &serde_json::Value) -> PathBuf {
    let bytes = canonical_receipt_bytes(receipt);
    let path = receipt_path(root, &bytes);
    fs::write(&path, bytes).unwrap();
    path
}

fn assert_runner_rejected(root: &Scratch, receipt: &Path, message: &str) {
    let output = Command::new(".venv/bin/python")
        .args([
            "tools/local_ci/run_assurance_amendment.py",
            "--receipt",
            receipt.to_str().unwrap(),
            "--root",
            root.path.to_str().unwrap(),
            "--allow-test-fixture",
        ])
        .current_dir(repository_root())
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains(message),
        "runner stderr did not contain {message:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn fixture(label: &str) -> Scratch {
    let target = Scratch::new(label);
    openwepp_assurance::copy_v2_test_fixture(&repository_root(), &target.path).unwrap();
    openwepp_assurance::rebind_v2_test_fixture(&target.path).unwrap();
    target
}

fn capture_tree(root: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    fn visit(root: &Path, current: &Path, output: &mut Vec<(PathBuf, Vec<u8>)>) {
        for entry in fs::read_dir(current).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_dir() {
                visit(root, &entry.path(), output);
            } else {
                output.push((
                    entry.path().strip_prefix(root).unwrap().to_path_buf(),
                    fs::read(entry.path()).unwrap(),
                ));
            }
        }
    }
    let mut output = Vec::new();
    visit(root, root, &mut output);
    output.sort_by(|left, right| left.0.cmp(&right.0));
    output
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn replace_yaml_strings(value: &mut serde_yaml::Value, old: &str, new: &str) {
    match value {
        serde_yaml::Value::String(text) => *text = text.replace(old, new),
        serde_yaml::Value::Sequence(values) => {
            for value in values {
                replace_yaml_strings(value, old, new);
            }
        }
        serde_yaml::Value::Mapping(values) => {
            for value in values.values_mut() {
                replace_yaml_strings(value, old, new);
            }
        }
        serde_yaml::Value::Tagged(tagged) => replace_yaml_strings(&mut tagged.value, old, new),
        serde_yaml::Value::Null | serde_yaml::Value::Bool(_) | serde_yaml::Value::Number(_) => {}
    }
}

fn copy_tree(source: &Path, target: &Path) {
    fs::create_dir_all(target).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let destination = target.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&entry.path(), &destination);
        } else {
            fs::copy(entry.path(), destination).unwrap();
        }
    }
}

struct Scratch {
    path: PathBuf,
}

impl Scratch {
    fn new(label: &str) -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let serial = COUNTER.fetch_add(1, Ordering::Relaxed);
        let path =
            std::env::temp_dir().join(format!("openwepp-{label}-{}-{serial}", std::process::id()));
        if path.exists() {
            fs::remove_dir_all(&path).unwrap();
        }
        fs::create_dir_all(&path).unwrap();
        Self { path }
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        if self.path.exists() {
            fs::remove_dir_all(&self.path).unwrap();
        }
    }
}
