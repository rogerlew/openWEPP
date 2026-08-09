use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Barrier};
use std::thread;

use openwepp_assurance::{
    AssuranceError, V2AmendMode, V2PublicationFault, V2PublicationOptions, V2PublicationResult,
    V2ReleaseIdentity, V2Repository, V2ReviewRoots, V2TrustDomain, amend_lifecycle, sha256_bytes,
    verify_v2_release_snapshot,
};

const REPORT_ID: &str = "linear-groundwater-reservoir-recurrence";
const SECOND_REPORT_ID: &str = "linear-groundwater-reservoir-recurrence-secondary";
const REPORT_PATH: &str =
    "assurance/v2/reports/linear-groundwater-reservoir-recurrence/report.yaml";
const CATALOG_PATH: &str = "assurance/v2/catalog.yaml";
const MANUSCRIPT_PATH: &str =
    "assurance/v2/reports/linear-groundwater-reservoir-recurrence/manuscript.md";
const SUPPLEMENT_PATH: &str =
    "assurance/v2/reports/linear-groundwater-reservoir-recurrence/supplement.md";
const TEST_BANNER: &str = "TEST ONLY — NOT SCIENTIFICALLY APPROVED";

#[test]
fn publication_api_has_separate_production_and_test_trust_domains() {
    let release = V2ReleaseIdentity::new(current_checkout_commit(), "openwepp-release-default-v1")
        .expect("valid release identity");
    let options = V2PublicationOptions::new(
        PathBuf::from("/tmp/assure04d-staging"),
        PathBuf::from("/tmp/assure04d-usersum"),
        PathBuf::from("/tmp/assure04d-snapshots"),
        release,
    );
    assert_eq!(
        options.release().configuration(),
        "openwepp-release-default-v1"
    );
    assert_eq!(V2TrustDomain::Production.to_string(), "production");
    assert_eq!(V2TrustDomain::TestOnly.to_string(), "test_only");

    let repository = V2Repository::open(repository_root()).expect("open v2 source");
    assert!(repository.publish_report(REPORT_ID, &options).is_err());
    assert!(
        repository
            .publish_test_fixture_report(REPORT_ID, &options)
            .is_err()
    );
}

#[test]
fn release_verifier_requires_real_artifacts_and_independent_identity() {
    let release = V2ReleaseIdentity::new(current_checkout_commit(), "openwepp-release-default-v1")
        .expect("valid release identity");
    assert!(
        verify_v2_release_snapshot(
            Path::new("/does/not/exist/snapshot"),
            Path::new("/does/not/exist/receipt.json"),
            &release,
        )
        .is_err()
    );
}

#[test]
fn cli_requires_explicit_publication_inputs() {
    let missing = openwepp_assurance::cli::run([
        "openwepp-assurance",
        "publish-test-fixture",
        "--report",
        REPORT_ID,
    ]);
    assert!(matches!(missing, Err(AssuranceError::Usage(_))));

    let production_verify = openwepp_assurance::cli::run([
        "openwepp-assurance",
        "verify-release",
        "--snapshot-dir",
        "/does/not/exist/snapshot",
        "--receipt",
        "/does/not/exist/receipt.json",
        "--release-commit",
        "ec396c458a5015c504011a75814ff13e274544a1",
        "--release-configuration",
        "openwepp-release-default-v1",
        "--all",
    ]);
    assert!(production_verify.is_err());
}

struct ApprovedFixture {
    source: Scratch,
    stage: Scratch,
    usersum: Scratch,
    snapshots: Scratch,
    release: V2ReleaseIdentity,
    domain: V2TrustDomain,
}

impl ApprovedFixture {
    fn options(&self) -> V2PublicationOptions {
        V2PublicationOptions::new(
            self.stage.path.clone(),
            self.usersum.path.clone(),
            self.snapshots.path.clone(),
            self.release.clone(),
        )
    }
}

fn approved_fixture(label: &str) -> ApprovedFixture {
    approved_fixture_in_domain(label, V2TrustDomain::TestOnly)
}

fn approved_fixture_in_domain(label: &str, domain: V2TrustDomain) -> ApprovedFixture {
    approved_fixture_with_suffix(label, domain, None)
}

fn approved_fixture_with_suffix(
    label: &str,
    domain: V2TrustDomain,
    manuscript_suffix: Option<&str>,
) -> ApprovedFixture {
    let source = source_fixture(&format!("{label}-source"));
    let stage = prepared_stage(&format!("{label}-stage"));
    let usersum = prepared_usersum(&format!("{label}-usersum"));
    let snapshots = Scratch::new(&format!("{label}-snapshots"));
    let release = V2ReleaseIdentity::new(current_checkout_commit(), "openwepp-release-default-v1")
        .expect("release identity");
    if let Some(suffix) = manuscript_suffix {
        let manuscript = source.path.join(MANUSCRIPT_PATH);
        let mut bytes = fs::read(&manuscript).unwrap();
        bytes.extend_from_slice(suffix.as_bytes());
        fs::write(manuscript, bytes).unwrap();
        openwepp_assurance::rebind_v2_test_fixture(&source.path).unwrap();
    }
    enter_synthetic_review(&source.path, domain);
    let repository = V2Repository::open(&source.path).expect("open in-review source");
    repository
        .build_report(REPORT_ID, &stage.path)
        .expect("build in-review subject");
    let review_roots = repository
        .review_roots(REPORT_ID, &stage.path)
        .expect("calculate subject and finding roots");
    bind_review_roots(&source.path, &review_roots);
    approve_synthetic_review(&source.path, &review_roots, &release);
    let repository = V2Repository::open(&source.path).expect("open approved source");
    repository
        .build_report(REPORT_ID, &stage.path)
        .expect("rebuild approved subject");
    let approved_roots = repository
        .review_roots(REPORT_ID, &stage.path)
        .expect("calculate approval and transfer roots");
    assert_eq!(approved_roots.subject_root, review_roots.subject_root);
    assert_eq!(
        approved_roots.finding_ledger_root,
        review_roots.finding_ledger_root
    );
    bind_approval_roots(&source.path, &approved_roots);
    V2Repository::open(&source.path)
        .expect("open root-bound source")
        .build_report(REPORT_ID, &stage.path)
        .expect("build exact approved staging");
    ApprovedFixture {
        source,
        stage,
        usersum,
        snapshots,
        release,
        domain,
    }
}

fn in_review_fixture(label: &str) -> ApprovedFixture {
    let source = source_fixture(&format!("{label}-source"));
    let stage = prepared_stage(&format!("{label}-stage"));
    let usersum = prepared_usersum(&format!("{label}-usersum"));
    let snapshots = Scratch::new(&format!("{label}-snapshots"));
    let release =
        V2ReleaseIdentity::new(current_checkout_commit(), "openwepp-release-default-v1").unwrap();
    enter_synthetic_review(&source.path, V2TrustDomain::TestOnly);
    let repository = V2Repository::open(&source.path).unwrap();
    repository.build_report(REPORT_ID, &stage.path).unwrap();
    let roots = repository.review_roots(REPORT_ID, &stage.path).unwrap();
    bind_review_roots(&source.path, &roots);
    V2Repository::open(&source.path)
        .unwrap()
        .build_report(REPORT_ID, &stage.path)
        .unwrap();
    ApprovedFixture {
        source,
        stage,
        usersum,
        snapshots,
        release,
        domain: V2TrustDomain::TestOnly,
    }
}

fn approved_two_report_fixture(label: &str) -> ApprovedFixture {
    approved_two_report_fixture_in_domain(label, V2TrustDomain::TestOnly)
}

fn approved_two_report_fixture_in_domain(label: &str, domain: V2TrustDomain) -> ApprovedFixture {
    let fixture = approved_fixture_in_domain(label, domain);
    let first_directory = fixture
        .source
        .path
        .join("assurance/v2/reports")
        .join(REPORT_ID);
    let second_directory = fixture
        .source
        .path
        .join("assurance/v2/reports")
        .join(SECOND_REPORT_ID);
    copy_tree(&first_directory, &second_directory);
    let second_report = second_directory.join("report.yaml");
    fs::copy(repository_root().join(REPORT_PATH), &second_report).unwrap();
    replace_all_in(&second_report, REPORT_ID, SECOND_REPORT_ID);
    if domain == V2TrustDomain::TestOnly {
        replace_in(
            &second_report,
            "trust_domain: production",
            "trust_domain: test_only",
        );
        replace_in(&second_report, "fixture_only: false", "fixture_only: true");
    }
    let second_lock = second_directory.join("review.lock.json");
    let production_draft_lock = repository_root()
        .join("assurance/v2/reports")
        .join(REPORT_ID)
        .join("review.lock.json");
    fs::copy(production_draft_lock, &second_lock).unwrap();
    replace_all_in(&second_lock, REPORT_ID, SECOND_REPORT_ID);
    let second_events = second_directory.join("review-events");
    if second_events.exists() {
        fs::remove_dir_all(second_events).unwrap();
    }
    append_catalog_report(
        &fixture.source.path,
        SECOND_REPORT_ID,
        &second_report,
        fixture.domain,
    );
    openwepp_assurance::rebind_v2_test_fixture(&fixture.source.path).unwrap();
    enter_synthetic_review_for(&fixture.source.path, SECOND_REPORT_ID);
    let repository = V2Repository::open(&fixture.source.path).unwrap();
    repository
        .build_report(SECOND_REPORT_ID, &fixture.stage.path)
        .unwrap();
    let second = repository
        .review_roots(SECOND_REPORT_ID, &fixture.stage.path)
        .unwrap();
    approve_synthetic_review_for(&fixture.source.path, SECOND_REPORT_ID, &fixture.release);
    let repository = V2Repository::open(&fixture.source.path).unwrap();
    repository
        .build_report(SECOND_REPORT_ID, &fixture.stage.path)
        .unwrap();
    let approved = repository
        .review_roots(SECOND_REPORT_ID, &fixture.stage.path)
        .unwrap();
    assert_eq!(second.subject_root, approved.subject_root);
    fixture
}

fn enter_synthetic_review(root: &Path, domain: V2TrustDomain) {
    fs::write(
        root.join("assurance/v2/principals.yaml"),
        synthetic_principals(domain),
    )
    .expect("write synthetic principals");
    refresh_catalog_identity(root, "assurance/v2/principals.yaml");

    if domain == V2TrustDomain::TestOnly {
        replace_all_in(
            &root.join(CATALOG_PATH),
            "trust_domain: production",
            "trust_domain: test_only",
        );
        replace_all_in(
            &root.join(CATALOG_PATH),
            "fixture_only: false",
            "fixture_only: true",
        );
        prepend_test_banner(&root.join(MANUSCRIPT_PATH));
        prepend_test_banner(&root.join(SUPPLEMENT_PATH));
        refresh_local_hash(root, MANUSCRIPT_PATH);
        refresh_local_hash(root, SUPPLEMENT_PATH);
    }

    let report = root.join(REPORT_PATH);
    if domain == V2TrustDomain::TestOnly {
        replace_in(
            &report,
            "trust_domain: production",
            "trust_domain: test_only",
        );
        replace_in(&report, "fixture_only: false", "fixture_only: true");
    }
    refresh_report_hash(root);
    enter_synthetic_review_for(root, REPORT_ID);
}

fn enter_synthetic_review_for(root: &Path, report_id: &str) {
    apply_lifecycle_for(
        root,
        report_id,
        r"schema_version: 1
event_type: review_entry
principal_id: test-report-lead
decision: entered_pending_review
rationale: Synthetic test-only review entry.
recorded_on: 2026-07-16
authority_source: ASSURE-04D synthetic fixture
predecessor_event_ids: []
review_charge: Independently assess scientific claims, reproducibility, reader communication, and publication mechanics.
build_maintainer_id: test-builder
material_producer_ids: [test-producer]
independence_assessment: Test principals are distinct and marked test-only; no real independence is claimed.
scientific_approver_id: test-scientist
",
    );
}

fn synthetic_principals(domain: V2TrustDomain) -> String {
    format!(
        r"schema_version: 2
trust_domain: {domain}
principals:
  - id: test-report-lead
    record_version: 1
    supersedes: null
    display_name: Test Report Lead
    affiliations: []
    kind: human
    identity_authority: ASSURE-04D synthetic fixture
    identity_reference: test-only/report-lead
    roles: [report_lead]
  - id: test-scientist
    record_version: 1
    supersedes: null
    display_name: Test Scientific Reviewer
    affiliations: []
    kind: human
    identity_authority: ASSURE-04D synthetic fixture
    identity_reference: test-only/scientific-reviewer
    roles: [scientific_approver]
  - id: test-reproducer
    record_version: 1
    supersedes: null
    display_name: Test Reproduction Reviewer
    affiliations: []
    kind: human
    identity_authority: ASSURE-04D synthetic fixture
    identity_reference: test-only/reproduction-reviewer
    roles: [reproduction_approver]
  - id: test-steward
    record_version: 1
    supersedes: null
    display_name: Test Assurance Steward
    affiliations: []
    kind: human
    identity_authority: ASSURE-04D synthetic fixture
    identity_reference: test-only/assurance-steward
    roles: [assurance_steward]
  - id: test-release-owner
    record_version: 1
    supersedes: null
    display_name: Test Release Owner
    affiliations: []
    kind: human
    identity_authority: ASSURE-04D synthetic fixture
    identity_reference: test-only/release-owner
    roles: [release_owner]
  - id: test-builder
    record_version: 1
    supersedes: null
    display_name: Test Build Maintainer
    affiliations: []
    kind: agent
    identity_authority: ASSURE-04D synthetic fixture
    identity_reference: test-only/build-maintainer
    roles: [build_maintainer]
  - id: test-producer
    record_version: 1
    supersedes: null
    display_name: Test Material Producer
    affiliations: []
    kind: agent
    identity_authority: ASSURE-04D synthetic fixture
    identity_reference: test-only/material-producer
    roles: [material_producer]
  - id: test-verifier
    record_version: 1
    supersedes: null
    display_name: Test Finding Verifier
    affiliations: []
    kind: agent
    identity_authority: ASSURE-04D synthetic fixture
    identity_reference: test-only/finding-verifier
    roles: [reviewer]
"
    )
}

fn bind_review_roots(_root: &Path, _roots: &V2ReviewRoots) {}

fn approve_synthetic_review(
    root: &Path,
    _review_roots: &V2ReviewRoots,
    release: &V2ReleaseIdentity,
) {
    approve_synthetic_review_for(root, REPORT_ID, release);
}

fn approve_synthetic_review_for(root: &Path, report_id: &str, release: &V2ReleaseIdentity) {
    apply_lifecycle_for(
        root,
        report_id,
        r"schema_version: 1
event_type: scientific_approval
principal_id: test-scientist
decision: approved
rationale: Synthetic scientific approval for publication mechanics testing only.
recorded_on: 2026-07-16
authority_source: ASSURE-04D synthetic fixture
predecessor_event_ids: []
competence_basis: Synthetic domain-science competence declaration for mechanics testing only.
independence_attestation: Synthetic reviewer is distinct from lead and producers.
",
    );
    let scientific = event_id_for(root, report_id, "scientific_approval");
    apply_lifecycle_for(
        root,
        report_id,
        &format!(
            "schema_version: 1\nevent_type: reproduction_approval\nprincipal_id: test-reproducer\ndecision: approved\nrationale: Synthetic reproduction approval for mechanics testing only.\nrecorded_on: 2026-07-16\nauthority_source: ASSURE-04D synthetic fixture\npredecessor_event_ids: [{scientific}]\ncompetence_basis: Synthetic reproduction competence declaration.\nindependence_attestation: Synthetic reproducer is distinct from lead, producers, and builder.\n"
        ),
    );
    let reproduction = event_id_for(root, report_id, "reproduction_approval");
    apply_lifecycle_for(
        root,
        report_id,
        &format!(
            "schema_version: 1\nevent_type: steward_approval\nprincipal_id: test-steward\ndecision: approved\nrationale: Synthetic steward approval for mechanics testing only.\nrecorded_on: 2026-07-16\nauthority_source: ASSURE-04D synthetic fixture\npredecessor_event_ids: [{scientific}, {reproduction}]\ncompetence_basis: Synthetic assurance-governance competence declaration.\nindependence_attestation: Synthetic steward is distinct from all predecessor approvers.\n"
        ),
    );
    let steward = event_id_for(root, report_id, "steward_approval");
    apply_lifecycle_for(
        root,
        report_id,
        &format!(
            "schema_version: 1\nevent_type: release_transfer\nprincipal_id: test-release-owner\ndecision: approved\nrationale: Synthetic release transfer for mechanics testing only.\nrecorded_on: 2026-07-16\nauthority_source: ASSURE-04D synthetic fixture\npredecessor_event_ids: [{steward}]\ntarget_release_commit: {}\ntarget_release_configuration: {}\nprior_realization: ASSURE-04C deterministic staging realization\ncandidate_realization: ASSURE-04D synthetic publication realization\nimpact_assessment: Publication mechanics only; no scientific conclusion changes are claimed.\nreproduction_disposition: Synthetic reproduction approval is present solely to exercise the fail-closed contract.\nsemantic_differences: [No scientific semantic difference; TEST ONLY banner added.]\nassurance_steward_id: test-steward\npublication_date: 2026-07-16\npublic_path: assurance/reports/{}/1.0.0/index.md\n",
            release.commit(),
            release.configuration(),
            report_id
        ),
    );
}

fn bind_approval_roots(_root: &Path, _roots: &V2ReviewRoots) {}

fn apply_lifecycle(root: &Path, request: &str) {
    apply_lifecycle_for(root, REPORT_ID, request);
}

fn apply_lifecycle_for(root: &Path, report_id: &str, request: &str) {
    amend_lifecycle(root, report_id, request.as_bytes(), V2AmendMode::Apply)
        .expect("apply synthetic immutable lifecycle event");
}

fn event_id(root: &Path, event_type: &str) -> String {
    event_id_for(root, REPORT_ID, event_type)
}

fn event_id_for(root: &Path, report_id: &str, event_type: &str) -> String {
    let lock: serde_json::Value = serde_json::from_slice(
        &fs::read(root.join(format!("assurance/v2/reports/{report_id}/review.lock.json"))).unwrap(),
    )
    .unwrap();
    lock["event_ids"]
        .as_array()
        .unwrap()
        .iter()
        .find_map(|id| {
            let id = id.as_str()?;
            let event: serde_json::Value = serde_json::from_slice(
                &fs::read(root.join(format!(
                    "assurance/v2/reports/{report_id}/review-events/{id}.json"
                )))
                .ok()?,
            )
            .ok()?;
            (event["event_type"] == event_type).then(|| id.to_owned())
        })
        .unwrap_or_else(|| panic!("missing event type {event_type}"))
}

fn mutate_review_event(root: &Path, event_type: &str, mutate: impl FnOnce(&mut serde_json::Value)) {
    let id = event_id(root, event_type);
    let path = root.join(format!(
        "assurance/v2/reports/{REPORT_ID}/review-events/{id}.json"
    ));
    let mut event: serde_json::Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    mutate(&mut event);
    fs::write(path, canonical_json_bytes(&event)).unwrap();
    openwepp_assurance::rebind_invalid_v2_test_fixture(root).unwrap();
}

#[test]
fn draft_subject_root_is_stable_but_cannot_publish() {
    let stage = prepared_stage("assure04d-draft-stage");
    let repository = V2Repository::open(repository_root()).expect("open v2 source");
    repository
        .build_report(REPORT_ID, &stage.path)
        .expect("assemble draft review subject");
    let first = repository
        .review_roots(REPORT_ID, &stage.path)
        .expect("calculate review subject");
    let second = repository
        .review_roots(REPORT_ID, &stage.path)
        .expect("repeat review subject");
    assert_eq!(first, second);
    assert_eq!(first.subject_root.len(), 64);
    assert_eq!(first.finding_ledger_root, None);
    assert_eq!(first.approval_lock_root, None);
    assert_eq!(first.release_transfer_root, None);
    let usersum = prepared_usersum("assure04d-draft-usersum");
    let snapshots = Scratch::new("assure04d-draft-snapshots");
    let prior_public = capture_tree(&usersum.path);
    let options = V2PublicationOptions::new(
        stage.path.clone(),
        usersum.path.clone(),
        snapshots.path.clone(),
        V2ReleaseIdentity::new(
            "ec396c458a5015c504011a75814ff13e274544a1",
            "openwepp-release-default-v1",
        )
        .unwrap(),
    );
    let error = repository
        .publish_report(REPORT_ID, &options)
        .expect_err("a DRAFT report must not publish");
    match error {
        AssuranceError::Invalid(message) => assert_eq!(
            message,
            format!("report '{REPORT_ID}' is DRAFT; publication requires APPROVED"),
            "DRAFT publication returned the wrong invalid-state error"
        ),
        other => panic!("DRAFT publication returned the wrong error type: {other:?}"),
    }
    assert_eq!(
        capture_tree(&usersum.path),
        prior_public,
        "DRAFT rejection must not mutate the public tree"
    );
    assert_eq!(
        fs::read_dir(&snapshots.path).unwrap().count(),
        0,
        "DRAFT rejection must not create a snapshot or receipt"
    );
}

#[test]
fn in_review_source_cannot_publish() {
    let fixture = in_review_fixture("assure04d-in-review");
    let result = V2Repository::open(&fixture.source.path)
        .unwrap()
        .publish_test_fixture_report(REPORT_ID, &fixture.options());
    assert!(matches!(
        result,
        Err(AssuranceError::Invalid(message)) if message.contains("IN_REVIEW")
    ));
    assert!(!fixture.usersum.path.join("assurance/catalog.json").exists());
}

#[test]
fn synthetic_approved_fixture_publishes_idempotently_and_release_rejects_it() {
    let fixture = approved_fixture("assure04d-positive");
    let repository = V2Repository::open(&fixture.source.path).expect("open root-bound source");
    let options = fixture.options();
    let first = repository
        .publish_test_fixture_report(REPORT_ID, &options)
        .expect("publish synthetic fixture");
    let repeated = repository
        .publish_test_fixture_report(REPORT_ID, &options)
        .expect("confirm repeated synthetic publication");
    assert_eq!(first, repeated);
    assert_eq!(first.report_ids, vec![REPORT_ID]);

    let public_report = fixture
        .usersum
        .path
        .join(format!("assurance/reports/{REPORT_ID}/1.0.0/index.md"));
    let staged_report = fixture.stage.path.join(format!(
        "usersum/assurance/reports/{REPORT_ID}/1.0.0/index.md"
    ));
    assert_eq!(
        fs::read(&public_report).unwrap(),
        fs::read(staged_report).unwrap()
    );
    for path in [
        fixture.usersum.path.join("assurance/README.md"),
        fixture.usersum.path.join("assurance/catalog.json"),
        public_report,
        first.snapshot_path.join("manifest.json"),
        first.receipt_path.clone(),
    ] {
        let text = fs::read_to_string(&path).expect("read visible test-only surface");
        assert!(
            text.contains(TEST_BANNER) || text.contains("\"trust_domain\":\"test_only\""),
            "synthetic marker missing from {}",
            path.display()
        );
    }
    assert!(matches!(
        verify_v2_release_snapshot(&first.snapshot_path, &first.receipt_path, &fixture.release),
        Err(AssuranceError::Invalid(message)) if message.contains("trust domain")
    ));

    let release_dir = fixture.snapshots.path.join("must-not-be-created");
    let preflight = Command::new("bash")
        .arg(repository_root().join("tools/release/run_release_candidate_gates.sh"))
        .args(["--mode", "release", "--release-dir"])
        .arg(&release_dir)
        .args(["--v2-assurance-snapshot"])
        .arg(&first.snapshot_path)
        .args(["--v2-assurance-receipt"])
        .arg(&first.receipt_path)
        .args(["--v2-assurance-release-commit", fixture.release.commit()])
        .args([
            "--v2-assurance-release-configuration",
            fixture.release.configuration(),
        ])
        .output()
        .expect("run actual release preflight consumer");
    assert!(!preflight.status.success());
    assert!(!release_dir.exists());
    assert!(
        String::from_utf8_lossy(&preflight.stderr).contains("trust domain"),
        "unexpected release rejection: {}",
        String::from_utf8_lossy(&preflight.stderr)
    );
    retain_publication_evidence_if_requested(&fixture, &first);

    let public_before = fs::read(fixture.usersum.path.join("assurance/catalog.json")).unwrap();
    fs::write(
        first
            .snapshot_path
            .join(format!("source/{REPORT_ID}/assurance/v2/principals.yaml")),
        b"mutated immutable snapshot\n",
    )
    .expect("mutate snapshot conflict fixture");
    assert!(matches!(
        repository.publish_test_fixture_report(REPORT_ID, &options),
        Err(AssuranceError::SnapshotConflict(_))
    ));
    assert_eq!(
        fs::read(fixture.usersum.path.join("assurance/catalog.json")).unwrap(),
        public_before
    );
}

#[test]
fn reconstructed_production_snapshot_passes_and_forged_roots_fail() {
    let fixture = approved_fixture_in_domain("assure04d-production", V2TrustDomain::Production);
    assert_eq!(fixture.domain, V2TrustDomain::Production);
    let repository = V2Repository::open(&fixture.source.path).unwrap();
    let published = repository
        .publish_report(REPORT_ID, &fixture.options())
        .expect("publish mechanically approved production fixture");
    let verified = verify_v2_release_snapshot(
        &published.snapshot_path,
        &published.receipt_path,
        &fixture.release,
    )
    .expect("reconstruct production authority from snapshot source");
    assert_eq!(verified.report_ids, vec![REPORT_ID]);

    let preflight = Command::new("bash")
        .arg(repository_root().join("tools/release/check_assurance_release_transition.sh"))
        .args(["--mode", "release", "--root"])
        .arg(repository_root())
        .args(["--v2-snapshot"])
        .arg(&published.snapshot_path)
        .args(["--v2-receipt"])
        .arg(&published.receipt_path)
        .args(["--release-commit", fixture.release.commit()])
        .args(["--release-configuration", fixture.release.configuration()])
        .output()
        .expect("run production release preflight");
    assert!(
        preflight.status.success(),
        "{}",
        String::from_utf8_lossy(&preflight.stderr)
    );
    let wrong_commit = run_release_preflight(
        &published.snapshot_path,
        &published.receipt_path,
        "ac396c458a5015c504011a75814ff13e274544a1",
        fixture.release.configuration(),
    );
    assert!(!wrong_commit.status.success());
    assert!(String::from_utf8_lossy(&wrong_commit.stderr).contains("checkout HEAD"));
    let wrong_configuration = run_release_preflight(
        &published.snapshot_path,
        &published.receipt_path,
        fixture.release.commit(),
        "different-build-configuration",
    );
    assert!(!wrong_configuration.status.success());
    assert!(
        String::from_utf8_lossy(&wrong_configuration.stderr).contains("actual build configuration")
    );

    assert_forged_receipts_fail(&fixture, &published);
}

fn assert_forged_receipts_fail(fixture: &ApprovedFixture, published: &V2PublicationResult) {
    let mut receipt: serde_json::Value =
        serde_json::from_slice(&fs::read(&published.receipt_path).unwrap()).unwrap();
    receipt["subject_roots"][REPORT_ID] = serde_json::Value::String("2".repeat(64));
    let forged_bytes = canonical_json_bytes(&receipt);
    let forged_id = sha256_bytes(&forged_bytes);
    let forged_path = fixture
        .snapshots
        .path
        .join("receipts")
        .join(format!("{forged_id}.json"));
    fs::write(&forged_path, forged_bytes).unwrap();
    assert!(matches!(
        verify_v2_release_snapshot(
            &published.snapshot_path,
            &forged_path,
            &fixture.release
        ),
        Err(AssuranceError::Drift(message)) if message.contains("roots")
    ));

    let mut wrong_builder: serde_json::Value =
        serde_json::from_slice(&fs::read(&published.receipt_path).unwrap()).unwrap();
    wrong_builder["builder_identity"] = serde_json::Value::String("unknown-builder".to_owned());
    let wrong_builder_bytes = canonical_json_bytes(&wrong_builder);
    let wrong_builder_id = sha256_bytes(&wrong_builder_bytes);
    let wrong_builder_path = fixture
        .snapshots
        .path
        .join("receipts")
        .join(format!("{wrong_builder_id}.json"));
    fs::write(&wrong_builder_path, wrong_builder_bytes).unwrap();
    assert!(matches!(
        verify_v2_release_snapshot(
            &published.snapshot_path,
            &wrong_builder_path,
            &fixture.release
        ),
        Err(AssuranceError::Invalid(message)) if message.contains("does not bind")
    ));

    #[cfg(unix)]
    {
        let linked = Scratch::new("assure04d-linked-receipt");
        let linked_receipt = linked.path.join(
            published
                .receipt_path
                .file_name()
                .expect("content-addressed receipt name"),
        );
        fs::hard_link(&published.receipt_path, &linked_receipt).unwrap();
        assert!(matches!(
            verify_v2_release_snapshot(
                &published.snapshot_path,
                &linked_receipt,
                &fixture.release
            ),
            Err(AssuranceError::Invalid(message)) if message.contains("multiply linked")
        ));
    }
}

#[test]
fn multi_report_production_snapshot_replays_complete_authority() {
    let fixture = approved_two_report_fixture_in_domain(
        "assure04d-production-two-report",
        V2TrustDomain::Production,
    );
    let published = V2Repository::open(&fixture.source.path)
        .unwrap()
        .publish_all(&fixture.options())
        .expect("publish two-report production snapshot");
    assert_eq!(
        published.report_ids,
        vec![REPORT_ID.to_owned(), SECOND_REPORT_ID.to_owned()]
    );
    let verified = verify_v2_release_snapshot(
        &published.snapshot_path,
        &published.receipt_path,
        &fixture.release,
    )
    .expect("replay both production report authorities");
    assert_eq!(verified.report_ids, published.report_ids);
}

#[test]
fn release_driver_persists_verified_v2_artifacts_and_discovery_sidecar() {
    let fixture = approved_fixture_in_domain(
        "assure04d-release-materialization",
        V2TrustDomain::Production,
    );
    let published = V2Repository::open(&fixture.source.path)
        .unwrap()
        .publish_report(REPORT_ID, &fixture.options())
        .unwrap();
    let release_dir = Scratch::new("assure04d-release-directory");
    let preflight = Command::new("bash")
        .arg(repository_root().join("tools/release/check_assurance_release_transition.sh"))
        .args(["--mode", "release", "--root"])
        .arg(repository_root())
        .arg("--v2-snapshot")
        .arg(&published.snapshot_path)
        .arg("--v2-receipt")
        .arg(&published.receipt_path)
        .args(["--release-commit", fixture.release.commit()])
        .args(["--release-configuration", fixture.release.configuration()])
        .status()
        .expect("run real release preflight");
    assert!(preflight.success());
    let materialized = Command::new("bash")
        .arg(repository_root().join("tools/release/materialize_assurance_v2_release.sh"))
        .arg(repository_root())
        .arg(&release_dir.path)
        .arg(&published.snapshot_path)
        .arg(&published.receipt_path)
        .arg(fixture.release.commit())
        .arg(fixture.release.configuration())
        .status()
        .expect("run real release materializer");
    assert!(materialized.success());

    let release_root = release_dir.path.join("assurance-v2");
    let copied_snapshot = release_root.join("snapshots").join(&published.snapshot_id);
    let copied_receipt = release_root
        .join("receipts")
        .join(format!("{}.json", published.receipt_id));
    verify_v2_release_snapshot(&copied_snapshot, &copied_receipt, &fixture.release).unwrap();
    assert_eq!(
        capture_tree(&published.snapshot_path),
        capture_tree(&copied_snapshot)
    );
    assert_eq!(
        fs::read(&published.receipt_path).unwrap(),
        fs::read(&copied_receipt).unwrap()
    );
    let sidecar: serde_json::Value = serde_json::from_slice(
        &fs::read(release_dir.path.join("assurance-v2-publication.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(sidecar["snapshot_id"], published.snapshot_id);
    assert_eq!(sidecar["receipt_id"], published.receipt_id);
    assert!(release_root.join("verification.txt").is_file());
    let checksums = Command::new("sha256sum")
        .args(["--check", "SHA256SUMS"])
        .current_dir(&release_root)
        .status()
        .expect("check materialized release hashes");
    assert!(checksums.success());

    let runner =
        fs::read_to_string(repository_root().join("tools/release/run_release_candidate_gates.sh"))
            .unwrap();
    assert!(runner.contains("materialize_assurance_v2_release.sh"));
}

#[test]
fn self_hashed_empty_production_container_is_not_release_authority() {
    let root = Scratch::new("assure04d-forged-empty");
    let release = V2ReleaseIdentity::new(
        "ec396c458a5015c504011a75814ff13e274544a1",
        "openwepp-release-default-v1",
    )
    .unwrap();
    let mut empty_public_identity = b"openwepp-assurance-public-tree-v1".to_vec();
    empty_public_identity.push(0);
    let public_tree = sha256_bytes(&empty_public_identity);
    let manifest = serde_json::json!({
        "domain": "openwepp-assurance-snapshot-v1",
        "files": [],
        "format": "openwepp-assurance-snapshot:1",
        "public_tree_sha256": public_tree,
        "release": {
            "commit": release.commit(),
            "configuration": release.configuration(),
        },
        "report_ids": [],
        "trust_domain": "production",
    });
    let manifest_bytes = canonical_json_bytes(&manifest);
    let snapshot_id = sha256_bytes(&manifest_bytes);
    let snapshot = root.path.join(&snapshot_id);
    fs::create_dir(&snapshot).unwrap();
    fs::write(snapshot.join("manifest.json"), manifest_bytes).unwrap();
    let receipt = serde_json::json!({
        "approval_lock_roots": {},
        "builder_identity": "openwepp-assurance-planner:1+openwepp-assurance-assembly:1+publication:1",
        "domain": "openwepp-assurance-receipt-v1",
        "finding_ledger_roots": {},
        "format": "openwepp-assurance-receipt:1",
        "public_tree_sha256": public_tree,
        "release": {
            "commit": release.commit(),
            "configuration": release.configuration(),
        },
        "release_transfer_roots": {},
        "report_ids": [],
        "snapshot_id": snapshot_id,
        "subject_roots": {},
        "trust_domain": "production",
    });
    let receipt_bytes = canonical_json_bytes(&receipt);
    let receipt_id = sha256_bytes(&receipt_bytes);
    let receipts = root.path.join("receipts");
    fs::create_dir(&receipts).unwrap();
    let receipt_path = receipts.join(format!("{receipt_id}.json"));
    fs::write(&receipt_path, receipt_bytes).unwrap();
    assert!(matches!(
        verify_v2_release_snapshot(&snapshot, &receipt_path, &release),
        Err(AssuranceError::Invalid(message)) if message.contains("at least one")
    ));
}

fn retain_publication_evidence_if_requested(
    fixture: &ApprovedFixture,
    result: &openwepp_assurance::V2PublicationResult,
) {
    let Some(destination) = std::env::var_os("OPENWEPP_ASSURE04D_RETAIN_ROOT") else {
        return;
    };
    let destination = PathBuf::from(destination);
    if destination.exists() {
        fs::remove_dir_all(&destination).expect("remove prior retained evidence");
    }
    copy_tree(
        &fixture.usersum.path.join("assurance"),
        &destination.join("public/assurance"),
    );
    copy_tree(
        &result.snapshot_path,
        &destination.join("snapshots").join(&result.snapshot_id),
    );
    let retained_receipt = destination
        .join("snapshots/receipts")
        .join(format!("{}.json", result.receipt_id));
    fs::create_dir_all(retained_receipt.parent().unwrap()).unwrap();
    fs::copy(&result.receipt_path, retained_receipt).expect("retain receipt bytes");
}

#[test]
fn simultaneous_publishers_serialize_and_converge_on_one_generation() {
    let fixture = approved_fixture("assure04d-concurrent");
    let barrier = Arc::new(Barrier::new(2));
    let mut publishers = Vec::new();
    for _ in 0..2 {
        let source = fixture.source.path.clone();
        let options = fixture.options();
        let barrier = Arc::clone(&barrier);
        publishers.push(thread::spawn(move || {
            let repository = V2Repository::open(&source).expect("open concurrent source");
            barrier.wait();
            repository
                .publish_test_fixture_report(REPORT_ID, &options)
                .expect("publish under advisory serialization")
        }));
    }
    let first = publishers.remove(0).join().expect("first publisher joined");
    let second = publishers
        .remove(0)
        .join()
        .expect("second publisher joined");
    assert_eq!(first, second);
    assert!(
        fixture
            .usersum
            .path
            .join("assurance/catalog.json")
            .is_file()
    );
    assert!(first.snapshot_path.join("manifest.json").is_file());
    assert!(first.receipt_path.is_file());
    assert!(fs::read_dir(&fixture.usersum.path).unwrap().all(|entry| {
        !entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .contains("prepare")
    }));
}

#[test]
fn concurrent_reader_observes_only_complete_old_or_new_report_bytes() {
    let fixture = approved_fixture("assure04d-reader");
    let repository = V2Repository::open(&fixture.source.path).unwrap();
    let first = repository
        .publish_test_fixture_report(REPORT_ID, &fixture.options())
        .unwrap();
    let report_path = fixture
        .usersum
        .path
        .join(format!("assurance/reports/{REPORT_ID}/1.0.0/index.md"));
    let old_bytes = fs::read(&report_path).unwrap();
    let second_fixture = approved_fixture_with_suffix(
        "assure04d-reader-second",
        V2TrustDomain::TestOnly,
        Some("\nDistinct second approved realization.\n"),
    );
    let staged_path = second_fixture.stage.path.join(format!(
        "usersum/assurance/reports/{REPORT_ID}/1.0.0/index.md"
    ));
    let new_bytes = fs::read(&staged_path).unwrap();
    assert_ne!(old_bytes, new_bytes);

    let usersum = fixture.usersum.path.clone();
    let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let saw_old = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let saw_new = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let reader_stop = Arc::clone(&stop);
    let reader_saw_old = Arc::clone(&saw_old);
    let reader_saw_new = Arc::clone(&saw_new);
    let reader_old = old_bytes.clone();
    let reader_new = new_bytes.clone();
    let reader = thread::spawn(move || {
        let report_path = usersum.join(format!("assurance/reports/{REPORT_ID}/1.0.0/index.md"));
        let mut reads = 0_u64;
        while !reader_stop.load(Ordering::Acquire) {
            let observed = fs::read(&report_path).unwrap();
            if observed == reader_old {
                reader_saw_old.store(true, Ordering::Release);
            } else if observed == reader_new {
                reader_saw_new.store(true, Ordering::Release);
            } else {
                panic!("reader observed a partial or mixed report byte stream");
            }
            reads += 1;
        }
        reads
    });
    for _ in 0..1_000_000 {
        if saw_old.load(Ordering::Acquire) {
            break;
        }
        thread::yield_now();
    }
    assert!(saw_old.load(Ordering::Acquire));
    let second_options = V2PublicationOptions::new(
        second_fixture.stage.path.clone(),
        fixture.usersum.path.clone(),
        fixture.snapshots.path.clone(),
        fixture.release.clone(),
    );
    let second = V2Repository::open(&second_fixture.source.path)
        .unwrap()
        .publish_test_fixture_report(REPORT_ID, &second_options)
        .unwrap();
    assert_ne!(first.snapshot_id, second.snapshot_id);
    for _ in 0..1_000_000 {
        if saw_new.load(Ordering::Acquire) {
            break;
        }
        thread::yield_now();
    }
    assert!(saw_new.load(Ordering::Acquire));
    stop.store(true, Ordering::Release);
    assert!(reader.join().unwrap() > 0);
    assert!(saw_old.load(Ordering::Acquire));
    assert!(saw_new.load(Ordering::Acquire));
}

#[test]
fn every_precommit_fault_boundary_preserves_the_prior_public_generation() {
    let fixture = approved_fixture("assure04d-fault-matrix");
    let repository = V2Repository::open(&fixture.source.path).unwrap();
    repository
        .publish_test_fixture_report(REPORT_ID, &fixture.options())
        .unwrap();
    let prior = capture_tree(&fixture.usersum.path.join("assurance"));
    for fault in [
        V2PublicationFault::AfterSnapshotInstall,
        V2PublicationFault::AfterReceiptInstall,
        V2PublicationFault::BeforePublicCommit,
    ] {
        let options = fixture.options().with_fault_injection_for_test(fault);
        assert!(
            repository
                .publish_test_fixture_report(REPORT_ID, &options)
                .is_err()
        );
        assert_eq!(capture_tree(&fixture.usersum.path.join("assurance")), prior);
    }
    repository
        .publish_test_fixture_report(REPORT_ID, &fixture.options())
        .expect("retry confirms orphaned immutable artifacts and commits");
    assert_eq!(capture_tree(&fixture.usersum.path.join("assurance")), prior);
}

#[test]
fn receipt_preparation_is_reused_only_when_bytes_match() {
    let oracle = approved_fixture("assure04d-receipt-oracle");
    let first = V2Repository::open(&oracle.source.path)
        .unwrap()
        .publish_test_fixture_report(REPORT_ID, &oracle.options())
        .unwrap();

    let fixture = approved_fixture("assure04d-receipt-retry");
    let preparation = fixture.snapshots.path.join(format!(
        "receipts/receipt.prepare-{}.json",
        &first.receipt_id[..16]
    ));
    fs::create_dir_all(preparation.parent().unwrap()).unwrap();
    fs::copy(&first.receipt_path, &preparation).unwrap();
    let retried = V2Repository::open(&fixture.source.path)
        .unwrap()
        .publish_test_fixture_report(REPORT_ID, &fixture.options())
        .expect("reuse exact transaction-owned receipt preparation");
    assert_eq!(first.snapshot_id, retried.snapshot_id);
    assert_eq!(first.receipt_id, retried.receipt_id);
    assert_eq!(first.public_tree_sha256, retried.public_tree_sha256);
    assert!(!preparation.exists());

    let conflict = approved_fixture("assure04d-receipt-conflict");
    let conflict_preparation = conflict.snapshots.path.join(format!(
        "receipts/receipt.prepare-{}.json",
        &first.receipt_id[..16]
    ));
    fs::create_dir_all(conflict_preparation.parent().unwrap()).unwrap();
    fs::write(&conflict_preparation, b"different receipt preparation\n").unwrap();
    assert!(matches!(
        V2Repository::open(&conflict.source.path)
            .unwrap()
            .publish_test_fixture_report(REPORT_ID, &conflict.options()),
        Err(AssuranceError::SnapshotConflict(message)) if message.contains("preparation")
    ));
    assert!(
        !conflict
            .usersum
            .path
            .join("assurance/catalog.json")
            .exists()
    );
}

#[test]
fn named_publication_preserves_receipted_peer_and_all_mode_converges() {
    let fixture = approved_two_report_fixture("assure04d-two-report");
    let repository = V2Repository::open(&fixture.source.path).unwrap();
    let first = repository
        .publish_test_fixture_report(REPORT_ID, &fixture.options())
        .expect("publish first named report");
    assert_eq!(first.report_ids, vec![REPORT_ID]);
    let first_path = fixture
        .usersum
        .path
        .join(format!("assurance/reports/{REPORT_ID}/1.0.0/index.md"));
    let first_bytes = fs::read(&first_path).unwrap();
    let combined = repository
        .publish_test_fixture_report(SECOND_REPORT_ID, &fixture.options())
        .expect("publish peer while preserving receipt-backed first report");
    assert_eq!(
        combined.report_ids,
        vec![REPORT_ID.to_owned(), SECOND_REPORT_ID.to_owned()]
    );
    assert_eq!(fs::read(&first_path).unwrap(), first_bytes);
    let catalog: serde_json::Value = serde_json::from_slice(
        &fs::read(fixture.usersum.path.join("assurance/catalog.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(catalog["reports"].as_array().unwrap().len(), 2);
    let all = repository
        .publish_all_test_fixtures(&fixture.options())
        .expect("all mode consumes the exact two-report staging set");
    assert_eq!(combined, all);
}

#[cfg(unix)]
#[test]
fn multiply_linked_staging_bytes_fail_before_publication() {
    let fixture = approved_fixture("assure04d-hardlink");
    let report_directory = fixture
        .stage
        .path
        .join(format!("usersum/assurance/reports/{REPORT_ID}/1.0.0"));
    fs::hard_link(
        report_directory.join("index.md"),
        report_directory.join("hard-linked-alias.md"),
    )
    .expect("create adversarial hard link");
    let result = V2Repository::open(&fixture.source.path)
        .unwrap()
        .publish_test_fixture_report(REPORT_ID, &fixture.options());
    assert!(matches!(
        result,
        Err(AssuranceError::Invalid(message)) if message.contains("multiply linked")
    ));
    assert!(!fixture.usersum.path.join("assurance/catalog.json").exists());
}

#[cfg(unix)]
#[test]
fn special_files_on_public_and_immutable_surfaces_fail_closed() {
    let public = approved_fixture("assure04d-public-fifo");
    let repository = V2Repository::open(&public.source.path).unwrap();
    repository
        .publish_test_fixture_report(REPORT_ID, &public.options())
        .unwrap();
    let fifo = public.usersum.path.join("assurance/unowned-fifo");
    assert!(
        Command::new("mkfifo")
            .arg(&fifo)
            .status()
            .unwrap()
            .success()
    );
    assert!(
        repository
            .publish_test_fixture_report(REPORT_ID, &public.options())
            .is_err()
    );

    let immutable =
        approved_fixture_in_domain("assure04d-immutable-fifo", V2TrustDomain::Production);
    let published = V2Repository::open(&immutable.source.path)
        .unwrap()
        .publish_report(REPORT_ID, &immutable.options())
        .unwrap();
    let fifo = published.snapshot_path.join("unowned-fifo");
    assert!(
        Command::new("mkfifo")
            .arg(&fifo)
            .status()
            .unwrap()
            .success()
    );
    assert!(
        verify_v2_release_snapshot(
            &published.snapshot_path,
            &published.receipt_path,
            &immutable.release,
        )
        .is_err()
    );
}

#[test]
fn named_publication_rejects_unreceipted_prior_catalog_entries() {
    let fixture = approved_fixture("assure04d-unreceipted-prior");
    let fake_directory = fixture
        .usersum
        .path
        .join("assurance/reports/unreceipted-report/9.9.9");
    fs::create_dir_all(&fake_directory).unwrap();
    fs::write(
        fake_directory.join("index.md"),
        "# Unreceipted report\n\nhillslope-hydrology-and-sediment-physics.md\n",
    )
    .unwrap();
    fs::write(fake_directory.join("supplement.md"), "# Supplement\n").unwrap();
    let fake_root = "1".repeat(64);
    let fake_catalog = format!(
        "{{\"format\":\"openwepp-assurance-public:1\",\"reports\":[{{\"approval_lock_root\":\"{fake_root}\",\"assessed_process\":\"Synthetic unreceipted process\",\"assessed_quantity\":\"None\",\"publication_date\":\"2026-07-16\",\"publication_state\":\"PUBLISHED\",\"realization\":\"Invalid prior fixture\",\"related_model_narrative\":\"usersum/hillslope-hydrology-and-sediment-physics.md\",\"release_transfer_root\":\"{fake_root}\",\"report_id\":\"unreceipted-report\",\"report_path\":\"assurance/reports/unreceipted-report/9.9.9/index.md\",\"scientific_question\":\"Can an unreceipted catalog entry survive?\",\"subject_root\":\"{fake_root}\",\"supplement_path\":\"assurance/reports/unreceipted-report/9.9.9/supplement.md\",\"title\":\"Unreceipted report\",\"version\":\"9.9.9\"}}],\"test_marker\":\"{TEST_BANNER}\",\"trust_domain\":\"test_only\"}}\n"
    );
    let catalog_path = fixture.usersum.path.join("assurance/catalog.json");
    fs::write(&catalog_path, &fake_catalog).unwrap();
    let result = V2Repository::open(&fixture.source.path)
        .unwrap()
        .publish_test_fixture_report(REPORT_ID, &fixture.options());
    assert!(matches!(
        result,
        Err(AssuranceError::Invalid(message)) if message.contains("receipt")
    ));
    assert_eq!(fs::read_to_string(catalog_path).unwrap(), fake_catalog);
}

#[test]
fn bootstrap_unowned_readme_fails_closed() {
    let readme = approved_fixture("assure04d-bootstrap-readme");
    fs::write(
        readme.usersum.path.join("assurance/README.md"),
        "# unowned bootstrap\n",
    )
    .unwrap();
    assert!(
        V2Repository::open(&readme.source.path)
            .unwrap()
            .publish_test_fixture_report(REPORT_ID, &readme.options())
            .is_err()
    );
    assert!(!readme.usersum.path.join("assurance/catalog.json").exists());
}

#[test]
fn bootstrap_empty_directory_fails_closed() {
    let empty = approved_fixture("assure04d-bootstrap-empty-dir");
    fs::create_dir_all(empty.usersum.path.join("assurance/reports/unknown/empty")).unwrap();
    assert!(
        V2Repository::open(&empty.source.path)
            .unwrap()
            .publish_test_fixture_report(REPORT_ID, &empty.options())
            .is_err()
    );
    assert!(!empty.usersum.path.join("assurance/catalog.json").exists());
}

#[test]
fn narrative_drift_fails_closed() {
    let narrative = approved_fixture("assure04d-narrative-drift");
    fs::write(
        narrative
            .usersum
            .path
            .join("hillslope-hydrology-and-sediment-physics.md"),
        "# drifted model narrative\n",
    )
    .unwrap();
    assert!(matches!(
        V2Repository::open(&narrative.source.path)
            .unwrap()
            .publish_test_fixture_report(REPORT_ID, &narrative.options()),
        Err(AssuranceError::Drift(message)) if message.contains("narrative")
    ));
    assert!(
        !narrative
            .usersum
            .path
            .join("assurance/catalog.json")
            .exists()
    );
}

#[cfg(unix)]
#[test]
fn staging_symlink_fails_closed() {
    use std::os::unix::fs::symlink;

    let staging = approved_fixture("assure04d-staging-symlink");
    symlink(
        "/etc/passwd",
        staging.stage.path.join(format!(
            "usersum/assurance/reports/{REPORT_ID}/1.0.0/unknown-link"
        )),
    )
    .unwrap();
    assert!(
        V2Repository::open(&staging.source.path)
            .unwrap()
            .publish_test_fixture_report(REPORT_ID, &staging.options())
            .is_err()
    );
    assert!(!staging.usersum.path.join("assurance/catalog.json").exists());
}

#[cfg(unix)]
#[test]
fn aliased_usersum_root_fails_closed() {
    use std::os::unix::fs::symlink;

    let aliased = approved_fixture("assure04d-usersum-alias");
    let alias_parent = Scratch::new("assure04d-alias-parent");
    let alias = alias_parent.path.join("usersum-link");
    symlink(&aliased.usersum.path, &alias).unwrap();
    let options = V2PublicationOptions::new(
        aliased.stage.path.clone(),
        alias,
        aliased.snapshots.path.clone(),
        aliased.release.clone(),
    );
    assert!(
        V2Repository::open(&aliased.source.path)
            .unwrap()
            .publish_test_fixture_report(REPORT_ID, &options)
            .is_err()
    );
    assert!(!aliased.usersum.path.join("assurance/catalog.json").exists());
}

#[cfg(unix)]
#[test]
fn staging_fifo_fails_closed() {
    let fifo = approved_fixture("assure04d-staging-fifo");
    let fifo_path = fifo.stage.path.join(format!(
        "usersum/assurance/reports/{REPORT_ID}/1.0.0/unknown-fifo"
    ));
    assert!(
        Command::new("mkfifo")
            .arg(&fifo_path)
            .status()
            .unwrap()
            .success()
    );
    assert!(
        V2Repository::open(&fifo.source.path)
            .unwrap()
            .publish_test_fixture_report(REPORT_ID, &fifo.options())
            .is_err()
    );
    assert!(!fifo.usersum.path.join("assurance/catalog.json").exists());
}

#[test]
fn canonical_public_path_and_real_markdown_narrative_link_are_mandatory() {
    let path_fixture = approved_fixture("assure04d-wrong-public-path");
    replace_in(
        &path_fixture.source.path.join(REPORT_PATH),
        &format!("  public_path: assurance/reports/{REPORT_ID}/1.0.0/index.md"),
        &format!("  public_path: assurance/reports/{REPORT_ID}/1.0.0/elsewhere.md"),
    );
    openwepp_assurance::rebind_invalid_v2_test_fixture(&path_fixture.source.path).unwrap();
    assert_unpublished(&path_fixture, "wrong public path");

    let link_fixture = approved_fixture("assure04d-missing-narrative-link");
    let staged_manuscript = link_fixture.stage.path.join(format!(
        "usersum/assurance/reports/{REPORT_ID}/1.0.0/index.md"
    ));
    replace_in(
        &staged_manuscript,
        "[model-science narrative](../../../../hillslope-hydrology-and-sediment-physics.md)",
        "model-science narrative",
    );
    assert_unpublished(&link_fixture, "missing narrative link");

    let fake_link = approved_fixture("assure04d-fake-narrative-link");
    let staged_manuscript = fake_link.stage.path.join(format!(
        "usersum/assurance/reports/{REPORT_ID}/1.0.0/index.md"
    ));
    replace_in(
        &staged_manuscript,
        "[model-science narrative](../../../../hillslope-hydrology-and-sediment-physics.md)",
        "<div>not a rendered link</div>",
    );
    assert_unpublished(&fake_link, "fake narrative link");
}

#[test]
fn stale_roots_open_findings_conflicts_and_release_mismatch_fail_before_publication() {
    let stale = approved_fixture("assure04d-stale-root");
    replace_in(
        &stale.source.path.join("assurance/v2/principals.yaml"),
        "display_name: Test Scientific Reviewer",
        "display_name: Changed Scientific Reviewer",
    );
    openwepp_assurance::rebind_invalid_v2_test_fixture(&stale.source.path).unwrap();
    let repository = V2Repository::open(&stale.source.path).unwrap();
    assert!(
        repository
            .publish_test_fixture_report(REPORT_ID, &stale.options())
            .is_err()
    );
    assert!(!stale.usersum.path.join("assurance/catalog.json").exists());

    let open = in_review_fixture("assure04d-open-finding");
    apply_lifecycle(
        &open.source.path,
        "schema_version: 1\nevent_type: finding\nprincipal_id: test-verifier\ndecision: open\nrationale: Synthetic open finding.\nrecorded_on: 2026-07-16\nauthority_source: test\npredecessor_event_ids: []\n",
    );
    let error = amend_lifecycle(
        &open.source.path,
        REPORT_ID,
        b"schema_version: 1\nevent_type: scientific_approval\nprincipal_id: test-scientist\ndecision: approved\nrationale: Must reject open finding.\nrecorded_on: 2026-07-16\nauthority_source: test\npredecessor_event_ids: []\ncompetence_basis: test\nindependence_attestation: test\n",
        V2AmendMode::Check,
    )
    .unwrap_err();
    assert!(error.to_string().contains("finding"));
}

#[test]
fn approval_conflicts_and_release_mismatch_fail_before_publication() {
    let duplicate = approved_fixture("assure04d-duplicate-approver");
    mutate_review_event(&duplicate.source.path, "reproduction_approval", |event| {
        event["principal_id"] = serde_json::Value::String("test-scientist".to_owned());
    });
    assert!(
        V2Repository::open(&duplicate.source.path)
            .unwrap()
            .publish_test_fixture_report(REPORT_ID, &duplicate.options())
            .is_err()
    );
    assert!(
        !duplicate
            .usersum
            .path
            .join("assurance/catalog.json")
            .exists()
    );

    let mismatch = approved_fixture("assure04d-release-mismatch");
    let wrong = V2PublicationOptions::new(
        mismatch.stage.path.clone(),
        mismatch.usersum.path.clone(),
        mismatch.snapshots.path.clone(),
        V2ReleaseIdentity::new(
            "ac396c458a5015c504011a75814ff13e274544a1",
            mismatch.release.configuration(),
        )
        .unwrap(),
    );
    assert!(
        V2Repository::open(&mismatch.source.path)
            .unwrap()
            .publish_test_fixture_report(REPORT_ID, &wrong)
            .is_err()
    );
    assert!(
        !mismatch
            .usersum
            .path
            .join("assurance/catalog.json")
            .exists()
    );

    let wrong_ledger = approved_fixture("assure04d-wrong-approval-ledger");
    mutate_review_event(&wrong_ledger.source.path, "scientific_approval", |event| {
        event["bound_roots"]["finding_ledger_root"] = serde_json::Value::String("2".repeat(64));
    });
    assert!(
        V2Repository::open(&wrong_ledger.source.path)
            .unwrap()
            .publish_test_fixture_report(REPORT_ID, &wrong_ledger.options())
            .is_err()
    );
    assert!(
        !wrong_ledger
            .usersum
            .path
            .join("assurance/catalog.json")
            .exists()
    );
}

#[test]
fn wrong_principal_kind_fails_closed() {
    let wrong_kind = approved_fixture("assure04d-wrong-principal-kind");
    replace_in(
        &wrong_kind.source.path.join("assurance/v2/principals.yaml"),
        "identity_reference: test-only/scientific-reviewer\n    roles: [scientific_approver]",
        "identity_reference: test-only/scientific-reviewer\n    roles: [draft_author]",
    );
    openwepp_assurance::rebind_invalid_v2_test_fixture(&wrong_kind.source.path).unwrap();
    assert_unpublished(&wrong_kind, "wrong principal kind");
}

#[test]
fn wrong_principal_role_fails_closed() {
    let wrong_role = approved_fixture("assure04d-wrong-principal-role");
    replace_in(
        &wrong_role.source.path.join("assurance/v2/principals.yaml"),
        "    roles: [scientific_approver]",
        "    roles: [draft_author]",
    );
    openwepp_assurance::rebind_invalid_v2_test_fixture(&wrong_role.source.path).unwrap();
    assert_unpublished(&wrong_role, "wrong principal role");
}

#[test]
fn wrong_principal_trust_domain_fails_closed() {
    let wrong_domain = approved_fixture("assure04d-wrong-principal-domain");
    replace_in(
        &wrong_domain
            .source
            .path
            .join("assurance/v2/principals.yaml"),
        "trust_domain: test_only",
        "trust_domain: production",
    );
    openwepp_assurance::rebind_invalid_v2_test_fixture(&wrong_domain.source.path).unwrap();
    assert_unpublished(&wrong_domain, "wrong principal trust domain");
}

#[test]
fn missing_competence_fails_closed() {
    let competence = in_review_fixture("assure04d-missing-competence");
    let error = amend_lifecycle(
        &competence.source.path,
        REPORT_ID,
        b"schema_version: 1\nevent_type: scientific_approval\nprincipal_id: test-scientist\ndecision: approved\nrationale: Missing competence negative.\nrecorded_on: 2026-07-16\nauthority_source: test\npredecessor_event_ids: []\ncompetence_basis: ''\nindependence_attestation: present\n",
        V2AmendMode::Check,
    )
    .unwrap_err();
    assert!(error.to_string().contains("competence"));
}

#[test]
fn missing_independence_fails_closed() {
    let independence = in_review_fixture("assure04d-missing-independence");
    let error = amend_lifecycle(
        &independence.source.path,
        REPORT_ID,
        b"schema_version: 1\nevent_type: scientific_approval\nprincipal_id: test-scientist\ndecision: approved\nrationale: Missing independence negative.\nrecorded_on: 2026-07-16\nauthority_source: test\npredecessor_event_ids: []\ncompetence_basis: present\nindependence_attestation: ''\n",
        V2AmendMode::Check,
    )
    .unwrap_err();
    assert!(error.to_string().contains("independence"));
}

#[test]
fn withdrawn_report_fails_closed() {
    let withdrawn = approved_fixture("assure04d-withdrawn");
    replace_in(
        &withdrawn.source.path.join(REPORT_PATH),
        "  withdrawn: false",
        "  withdrawn: true",
    );
    openwepp_assurance::rebind_invalid_v2_test_fixture(&withdrawn.source.path).unwrap();
    assert_unpublished_with_message(&withdrawn, "withdrawn publication", "withdrawn");
}

#[test]
fn superseded_report_fails_closed() {
    let superseded = approved_fixture("assure04d-superseded");
    replace_in(
        &superseded.source.path.join(REPORT_PATH),
        "  supersedes: null",
        "  supersedes: prior-assurance-realization",
    );
    openwepp_assurance::rebind_invalid_v2_test_fixture(&superseded.source.path).unwrap();
    assert_unpublished_with_message(&superseded, "superseded publication", "supersedes");
}

#[test]
fn missing_release_transfer_fails_closed() {
    let missing_transfer = approved_fixture("assure04d-missing-release-transfer");
    let transfer = event_id(&missing_transfer.source.path, "release_transfer");
    fs::remove_file(missing_transfer.source.path.join(format!(
        "assurance/v2/reports/{REPORT_ID}/review-events/{transfer}.json"
    )))
    .unwrap();
    openwepp_assurance::rebind_invalid_v2_test_fixture(&missing_transfer.source.path).unwrap();
    assert_unpublished(&missing_transfer, "missing release transfer");
}

#[test]
fn authority_bound_byte_negative_matrix_is_fail_closed() {
    let research = approved_fixture("assure04d-incomplete-research-object");
    fs::remove_file(
        research
            .source
            .path
            .join("docs/specifications/science-contracts/contracts/SC-GWBASEFLOW-001.md"),
    )
    .unwrap();
    assert_unpublished(&research, "missing research object");

    let schema = approved_fixture("assure04d-changed-schema");
    append_byte(
        &schema
            .source
            .path
            .join("assurance/v2/schemas/report.schema.json"),
    );
    assert_unpublished(&schema, "changed schema bytes");

    let catalog = approved_fixture("assure04d-changed-catalog");
    append_byte(&catalog.source.path.join(CATALOG_PATH));
    assert_unpublished(&catalog, "changed catalog bytes");

    let dependency = approved_fixture("assure04d-changed-dependency");
    append_byte(
        &dependency
            .source
            .path
            .join("docs/specifications/science-contracts/contracts/SC-GWBASEFLOW-001.md"),
    );
    assert_unpublished(&dependency, "changed dependency bytes");

    let generated = approved_fixture("assure04d-changed-generated-output");
    append_byte(&generated.stage.path.join(format!(
        "usersum/assurance/reports/{REPORT_ID}/1.0.0/index.md"
    )));
    assert_unpublished(&generated, "changed generated output bytes");
}

#[test]
fn all_mode_extra_staging_and_overlapping_roots_fail_without_public_mutation() {
    let fixture = approved_fixture("assure04d-confinement");
    let extra = fixture
        .stage
        .path
        .join("usersum/assurance/reports/.hidden-stale");
    fs::write(&extra, b"stale\n").expect("write hidden staging byte");
    let repository = V2Repository::open(&fixture.source.path).unwrap();
    assert!(matches!(
        repository.publish_all_test_fixtures(&fixture.options()),
        Err(AssuranceError::Drift(message)) if message.contains("staging")
    ));
    assert!(!fixture.usersum.path.join("assurance/catalog.json").exists());
    fs::remove_file(extra).unwrap();

    let overlap = V2PublicationOptions::new(
        fixture.stage.path.clone(),
        fixture.stage.path.clone(),
        fixture.snapshots.path.clone(),
        fixture.release.clone(),
    );
    assert!(matches!(
        repository.publish_test_fixture_report(REPORT_ID, &overlap),
        Err(AssuranceError::Invalid(message)) if message.contains("unrelated")
    ));
    assert!(!fixture.usersum.path.join("assurance/catalog.json").exists());
}

fn repository_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn current_checkout_commit() -> String {
    let output = Command::new("git")
        .args(["rev-parse", "--verify", "HEAD"])
        .current_dir(repository_root())
        .output()
        .expect("read selected checkout HEAD");
    assert!(output.status.success());
    String::from_utf8(output.stdout)
        .expect("Git object ID is UTF-8")
        .trim()
        .to_owned()
}

fn prepared_stage(label: &str) -> Scratch {
    let stage = Scratch::new(label);
    let relative = "usersum/hillslope-hydrology-and-sediment-physics.md";
    let target = stage.path.join(relative);
    fs::create_dir_all(target.parent().expect("narrative parent"))
        .expect("create narrative parent");
    fs::copy(repository_root().join(relative), target).expect("copy related model narrative");
    stage
}

fn prepared_usersum(label: &str) -> Scratch {
    let usersum = Scratch::new(label);
    for relative in [
        "hillslope-hydrology-and-sediment-physics.md",
        "assurance/README.md",
    ] {
        let source = repository_root().join("usersum").join(relative);
        let target = usersum.path.join(relative);
        fs::create_dir_all(target.parent().unwrap()).expect("create usersum parent");
        fs::copy(source, target).expect("copy usersum fixture byte");
    }
    usersum
}

fn source_fixture(label: &str) -> Scratch {
    let source = repository_root();
    let target = Scratch::new(label);
    openwepp_assurance::copy_v2_test_fixture(source, &target.path).unwrap();
    openwepp_assurance::retain_v2_test_report(&target.path, REPORT_ID).unwrap();
    for relative in [
        "usersum/hillslope-hydrology-and-sediment-physics.md",
        "docs/specifications/science-contracts/contracts/SC-GWBASEFLOW-001.md",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/groundwater.rs",
        "docs/work-packages/20260714-assure02-manuscript-first-assurance-architecture-001/artifacts/groundwater-current-tree-confirmation.md",
        "docs/work-packages/20260714-assure02-manuscript-first-assurance-architecture-001/artifacts/prototype-linear-groundwater-reservoir-evaluation.md",
        "docs/work-packages/20260709-laned-active-baseflow-export-closure-001/artifacts/consumer-path-proof.md",
        "docs/work-packages/20260708-groundwater-baseflow-laned-single-ofe-mofe-implementation-001/artifacts/consumer-path-proof.md",
        "docs/work-packages/20260716-assure05-first-production-v2-report-001/artifacts/realization-freeze.md",
        "docs/work-packages/20260716-assure05-first-production-v2-report-001/artifacts/study-protocol.md",
        "docs/work-packages/20260716-assure05-first-production-v2-report-001/prompts/archived/20260716-codex-execute-assure05_prompt.md",
        "docs/work-packages/20260713-integrated-validation-campaign-001/artifacts/final-conservation-and-consumer-evidence.md",
        "docs/work-packages/20260713-integrated-validation-campaign-001/artifacts/logs/final-reconstruction-arithmetic.log",
    ] {
        copy_file(source, &target.path, relative);
    }
    target
}

fn prepend_test_banner(path: &Path) {
    let text = fs::read_to_string(path).expect("read authored source");
    let end = text.find('\n').expect("markdown heading line");
    let updated = format!(
        "{}\n\n> **{TEST_BANNER}**{}",
        &text[..end],
        &text[end + 1..]
    );
    fs::write(path, updated).expect("write test-only banner");
}

fn refresh_local_hash(root: &Path, _relative: &str) {
    openwepp_assurance::rebind_v2_test_fixture(root).expect("rebind fixture identity");
}

fn refresh_catalog_identity(root: &Path, _relative: &str) {
    openwepp_assurance::rebind_v2_test_fixture(root).expect("rebind fixture identity");
}

fn refresh_report_hash(root: &Path) {
    openwepp_assurance::rebind_v2_test_fixture(root).expect("rebind fixture identity");
}

fn append_catalog_report(root: &Path, report_id: &str, report_path: &Path, domain: V2TrustDomain) {
    let relative = format!("assurance/v2/reports/{report_id}/report.yaml");
    assert!(report_path.is_file());
    let catalog = root.join(CATALOG_PATH);
    let mut text = fs::read_to_string(&catalog).unwrap();
    let fixture_only = domain == V2TrustDomain::TestOnly;
    write!(
        text,
        "- id: {report_id}\n  version: 1.0.0\n  title: Verification of openWEPP's Daily Linear Groundwater Reservoir\n  owner: openWEPP scientific assurance maintainers\n  trust_domain: {domain}\n  fixture_only: {fixture_only}\n  manifest_path: {relative}\n"
    )
    .expect("write catalog report entry");
    fs::write(catalog, text).unwrap();
}

fn assert_unpublished(fixture: &ApprovedFixture, case: &str) {
    let result = V2Repository::open(&fixture.source.path).and_then(|repository| {
        repository.publish_test_fixture_report(REPORT_ID, &fixture.options())
    });
    assert!(result.is_err(), "{case} unexpectedly published");
    assert!(
        !fixture.usersum.path.join("assurance/catalog.json").exists(),
        "{case} mutated the public destination"
    );
}

fn assert_unpublished_with_message(fixture: &ApprovedFixture, case: &str, needle: &str) {
    let error = V2Repository::open(&fixture.source.path)
        .and_then(|repository| {
            repository.publish_test_fixture_report(REPORT_ID, &fixture.options())
        })
        .expect_err(case);
    assert!(
        error.to_string().contains(needle),
        "{case} failed for the wrong reason: {error}"
    );
    assert!(
        !fixture.usersum.path.join("assurance/catalog.json").exists(),
        "{case} mutated the public destination"
    );
}

fn append_byte(path: &Path) {
    let mut bytes = fs::read(path).expect("read byte-mutation target");
    bytes.push(b'\n');
    fs::write(path, bytes).expect("write byte-mutation target");
}

fn replace_in(path: &Path, old: &str, new: &str) {
    let text = fs::read_to_string(path).expect("read replacement target");
    assert!(text.contains(old), "replacement source missing: {old}");
    fs::write(path, text.replacen(old, new, 1)).expect("write replacement target");
}

fn replace_all_in(path: &Path, old: &str, new: &str) {
    let text = fs::read_to_string(path).expect("read replacement target");
    assert!(text.contains(old), "replacement source missing: {old}");
    fs::write(path, text.replace(old, new)).expect("write replacement target");
}

fn canonical_json_bytes(value: &serde_json::Value) -> Vec<u8> {
    let mut bytes = serde_json::to_vec(&normalized_json(value.clone())).unwrap();
    bytes.push(b'\n');
    bytes
}

fn normalized_json(value: serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Object(values) => {
            let ordered = values
                .into_iter()
                .map(|(key, value)| (key, normalized_json(value)))
                .collect::<std::collections::BTreeMap<_, _>>();
            serde_json::to_value(ordered).unwrap()
        }
        serde_json::Value::Array(values) => {
            serde_json::Value::Array(values.into_iter().map(normalized_json).collect())
        }
        value => value,
    }
}

fn run_release_preflight(
    snapshot: &Path,
    receipt: &Path,
    commit: &str,
    configuration: &str,
) -> std::process::Output {
    Command::new("bash")
        .arg(repository_root().join("tools/release/check_assurance_release_transition.sh"))
        .args(["--mode", "release", "--root"])
        .arg(repository_root())
        .args(["--v2-snapshot"])
        .arg(snapshot)
        .args(["--v2-receipt"])
        .arg(receipt)
        .args(["--release-commit", commit])
        .args(["--release-configuration", configuration])
        .output()
        .expect("run release transition preflight")
}

fn copy_tree(source: &Path, target: &Path) {
    fs::create_dir_all(target).expect("create fixture tree");
    for entry in fs::read_dir(source).expect("read fixture source tree") {
        let entry = entry.expect("read fixture entry");
        let destination = target.join(entry.file_name());
        if entry.file_type().expect("fixture type").is_dir() {
            copy_tree(&entry.path(), &destination);
        } else {
            fs::copy(entry.path(), destination).expect("copy fixture entry");
        }
    }
}

fn copy_file(source_root: &Path, target_root: &Path, relative: &str) {
    let target = target_root.join(relative);
    fs::create_dir_all(target.parent().unwrap()).expect("create fixture parent");
    fs::copy(source_root.join(relative), target).expect("copy fixture file");
}

fn capture_tree(root: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    fn visit(root: &Path, path: &Path, files: &mut Vec<(PathBuf, Vec<u8>)>) {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_dir() {
                visit(root, &entry.path(), files);
            } else {
                files.push((
                    entry.path().strip_prefix(root).unwrap().to_path_buf(),
                    fs::read(entry.path()).unwrap(),
                ));
            }
        }
    }
    let mut files = Vec::new();
    visit(root, root, &mut files);
    files.sort_by(|left, right| left.0.cmp(&right.0));
    files
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
            fs::remove_dir_all(&path).expect("remove stale scratch");
        }
        fs::create_dir_all(&path).expect("create scratch");
        Self { path }
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        if self.path.exists() {
            fs::remove_dir_all(&self.path).expect("remove scratch");
        }
    }
}
