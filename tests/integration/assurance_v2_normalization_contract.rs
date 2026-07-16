use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use openwepp_assurance::{
    AssuranceError, V2NormalizationMode, V2NormalizationOptions, V2Repository, sha256_bytes,
};

const REPORT_ID: &str = "linear-groundwater-reservoir-recurrence";
const REPORT_PATH: &str =
    "assurance/v2/reports/linear-groundwater-reservoir-recurrence/report.yaml";
const MANUSCRIPT_PATH: &str =
    "assurance/v2/reports/linear-groundwater-reservoir-recurrence/manuscript.md";
const PACKET_PATH: &str = "assurance/v2/reports/linear-groundwater-reservoir-recurrence/evidence/agent-assistance-packet.json";
const CATALOG_PATH: &str = "assurance/v2/catalog.yaml";
const OUTPUT_BASE: &str = "usersum/assurance/reports/linear-groundwater-reservoir-recurrence/1.0.0";

#[test]
fn current_report_is_american_english_and_check_is_read_only() {
    let root = repository_root();
    let before = collect_files(&root.join("assurance/v2"));
    let receipt = V2Repository::open(&root)
        .unwrap()
        .normalize_report(
            REPORT_ID,
            &normalization_options(V2NormalizationMode::Check),
        )
        .expect("current production DRAFT is normalized");
    assert!(!receipt.changed);
    assert!(receipt.changes.is_empty());
    assert_eq!(before, collect_files(&root.join("assurance/v2")));
}

#[test]
fn check_detects_converter_diff_without_writing() {
    let fixture = british_fixture("assurance-normalize-check");
    let before = collect_files(&fixture.path.join("assurance/v2"));
    let error = V2Repository::open(&fixture.path)
        .unwrap()
        .normalize_report(
            REPORT_ID,
            &normalization_options(V2NormalizationMode::Check),
        )
        .expect_err("British spelling must fail the read-only gate");
    assert!(matches!(error, AssuranceError::Drift(_)));
    assert!(error.to_string().contains("rerun with --apply"));
    assert_eq!(before, collect_files(&fixture.path.join("assurance/v2")));
    assert!(!fixture.path.join("assurance/.v2.normalize.next").exists());
}

#[test]
fn apply_rebinds_complete_graph_builds_and_is_idempotent() {
    let fixture = british_fixture("assurance-normalize-apply");
    let reference = repository_root();
    let modes_before = collect_modes(&fixture.path.join("assurance/v2"));
    let receipt = V2Repository::open(&fixture.path)
        .unwrap()
        .normalize_report(
            REPORT_ID,
            &normalization_options(V2NormalizationMode::Apply),
        )
        .expect("apply normalization transaction");
    assert!(receipt.changed);
    assert_eq!(receipt.changes.len(), 4);
    assert_eq!(
        fs::read(fixture.path.join(MANUSCRIPT_PATH)).unwrap(),
        fs::read(reference.join(MANUSCRIPT_PATH)).unwrap()
    );
    assert_eq!(
        fs::read(fixture.path.join(PACKET_PATH)).unwrap(),
        fs::read(reference.join(PACKET_PATH)).unwrap()
    );
    assert_eq!(
        fs::read(fixture.path.join(REPORT_PATH)).unwrap(),
        fs::read(reference.join(REPORT_PATH)).unwrap()
    );
    assert_eq!(
        fs::read(fixture.path.join(CATALOG_PATH)).unwrap(),
        fs::read(reference.join(CATALOG_PATH)).unwrap()
    );
    assert_eq!(
        modes_before,
        collect_modes(&fixture.path.join("assurance/v2"))
    );

    let repository = V2Repository::open(&fixture.path).expect("reopen normalized source");
    repository
        .validate_report(REPORT_ID)
        .expect("validate rebound source");
    let stage = prepared_stage("assurance-normalize-stage");
    repository
        .build_report(REPORT_ID, &stage.path)
        .expect("build normalized source");
    repository
        .check_report(REPORT_ID, &stage.path)
        .expect("check normalized source");
    assert!(stage.path.join(OUTPUT_BASE).join("index.md").is_file());

    let repeated = repository
        .normalize_report(
            REPORT_ID,
            &normalization_options(V2NormalizationMode::Apply),
        )
        .expect("repeat apply is a no-op");
    assert!(!repeated.changed);
    assert!(repeated.changes.is_empty());
    assert_eq!(
        repeated.old_source_root_sha256,
        repeated.new_source_root_sha256
    );
    assert!(!fixture.path.join("assurance/.v2.normalize.next").exists());
}

#[test]
fn equivalent_inputs_emit_identical_receipts() {
    let left = british_fixture("assurance-normalize-receipt-left");
    let right = british_fixture("assurance-normalize-receipt-right");
    let left_receipt = V2Repository::open(&left.path)
        .unwrap()
        .normalize_report(
            REPORT_ID,
            &normalization_options(V2NormalizationMode::Apply),
        )
        .unwrap()
        .render_json()
        .unwrap();
    let right_receipt = V2Repository::open(&right.path)
        .unwrap()
        .normalize_report(
            REPORT_ID,
            &normalization_options(V2NormalizationMode::Apply),
        )
        .unwrap()
        .render_json()
        .unwrap();
    assert_eq!(left_receipt, right_receipt);
}

#[test]
fn lifecycle_review_and_packet_boundaries_fail_before_writing() {
    let authorized = fixture("assurance-normalize-review-authorized");
    replace_in_report(
        &authorized.path,
        "  provenance_complete: false\n  review_entry_authorized: false",
        "  provenance_complete: true\n  review_entry_authorized: true",
    );
    let authorized_before = collect_files(&authorized.path.join("assurance/v2"));
    let error = V2Repository::open(&authorized.path)
        .unwrap()
        .normalize_report(
            REPORT_ID,
            &normalization_options(V2NormalizationMode::Check),
        )
        .expect_err("review-authorized DRAFT must reject normalization");
    assert!(error.to_string().contains("authorized review entry"));
    assert_eq!(
        authorized_before,
        collect_files(&authorized.path.join("assurance/v2"))
    );

    let non_draft = fixture("assurance-normalize-non-draft");
    replace_in_report(&non_draft.path, "lifecycle: DRAFT", "lifecycle: IN_REVIEW");
    let non_draft_before = collect_files(&non_draft.path.join("assurance/v2"));
    V2Repository::open(&non_draft.path)
        .unwrap()
        .normalize_report(
            REPORT_ID,
            &normalization_options(V2NormalizationMode::Check),
        )
        .expect_err("non-DRAFT source must reject normalization");
    assert_eq!(
        non_draft_before,
        collect_files(&non_draft.path.join("assurance/v2"))
    );

    let stale_packet = fixture("assurance-normalize-stale-packet");
    let old_packet = sha256_bytes(&fs::read(stale_packet.path.join(PACKET_PATH)).unwrap());
    let manuscript = sha256_bytes(&fs::read(stale_packet.path.join(MANUSCRIPT_PATH)).unwrap());
    replace_all_digest(
        &stale_packet.path.join(PACKET_PATH),
        &manuscript,
        &"0".repeat(64),
    );
    let new_packet = sha256_bytes(&fs::read(stale_packet.path.join(PACKET_PATH)).unwrap());
    replace_all_digest(
        &stale_packet.path.join(REPORT_PATH),
        &old_packet,
        &new_packet,
    );
    refresh_report_hash(&stale_packet.path);
    let packet_before = collect_files(&stale_packet.path.join("assurance/v2"));
    V2Repository::open(&stale_packet.path)
        .unwrap()
        .normalize_report(
            REPORT_ID,
            &normalization_options(V2NormalizationMode::Check),
        )
        .expect_err("internally stale packet must fail even when prose is normalized");
    assert_eq!(
        packet_before,
        collect_files(&stale_packet.path.join("assurance/v2"))
    );
}

fn normalization_options(mode: V2NormalizationMode) -> V2NormalizationOptions {
    V2NormalizationOptions::new("en-US", mode)
}

fn replace_in_report(root: &Path, old: &str, new: &str) {
    let path = root.join(REPORT_PATH);
    let text = fs::read_to_string(&path).unwrap();
    assert!(text.contains(old));
    fs::write(path, text.replacen(old, new, 1)).unwrap();
    refresh_report_hash(root);
}

fn refresh_report_hash(root: &Path) {
    let reference = repository_root();
    let old = sha256_bytes(&fs::read(reference.join(REPORT_PATH)).unwrap());
    let new = sha256_bytes(&fs::read(root.join(REPORT_PATH)).unwrap());
    let catalog = root.join(CATALOG_PATH);
    let text = fs::read_to_string(&catalog).unwrap();
    let current = text
        .lines()
        .find_map(|line| line.trim().strip_prefix("manifest_sha256: "))
        .expect("catalog report digest");
    assert!(current == old || current.len() == 64);
    fs::write(catalog, text.replacen(current, &new, 1)).unwrap();
}

fn british_fixture(label: &str) -> Scratch {
    let target = fixture(label);
    let manuscript = target.path.join(MANUSCRIPT_PATH);
    let original = fs::read_to_string(&manuscript).unwrap();
    assert!(original.contains("Storage is in cubic meters."));
    fs::write(
        &manuscript,
        original.replacen(
            "Storage is in cubic meters.",
            "Storage is in cubic metres.",
            1,
        ),
    )
    .unwrap();

    let reference = repository_root();
    let old_manuscript = sha256_bytes(&fs::read(reference.join(MANUSCRIPT_PATH)).unwrap());
    let new_manuscript = sha256_bytes(&fs::read(&manuscript).unwrap());
    replace_all_digest(
        &target.path.join(PACKET_PATH),
        &old_manuscript,
        &new_manuscript,
    );
    let old_packet = sha256_bytes(&fs::read(reference.join(PACKET_PATH)).unwrap());
    let new_packet = sha256_bytes(&fs::read(target.path.join(PACKET_PATH)).unwrap());
    replace_all_digest(
        &target.path.join(REPORT_PATH),
        &old_manuscript,
        &new_manuscript,
    );
    replace_all_digest(&target.path.join(REPORT_PATH), &old_packet, &new_packet);
    let old_report = sha256_bytes(&fs::read(reference.join(REPORT_PATH)).unwrap());
    let new_report = sha256_bytes(&fs::read(target.path.join(REPORT_PATH)).unwrap());
    replace_all_digest(&target.path.join(CATALOG_PATH), &old_report, &new_report);
    V2Repository::open(&target.path)
        .unwrap()
        .validate_report(REPORT_ID)
        .expect("British-spelling fixture is current before normalization");
    target
}

fn replace_all_digest(path: &Path, old: &str, new: &str) {
    let text = fs::read_to_string(path).unwrap();
    assert!(text.contains(old), "missing digest in {}", path.display());
    fs::write(path, text.replace(old, new)).unwrap();
}

fn prepared_stage(label: &str) -> Scratch {
    let stage = Scratch::new(label);
    copy_file(
        &repository_root(),
        &stage.path,
        "usersum/hillslope-hydrology-and-sediment-physics.md",
    );
    stage
}

fn fixture(label: &str) -> Scratch {
    let source = repository_root();
    let target = Scratch::new(label);
    copy_tree(
        &source.join("assurance/v2"),
        &target.path.join("assurance/v2"),
    );
    for relative in [
        "assurance/catalog.yaml",
        "assurance/templates/catalog.md",
        "assurance/generated/wepppy-usersum.yaml",
        "usersum/assurance/README.md",
        "usersum/hillslope-hydrology-and-sediment-physics.md",
        "docs/specifications/science-contracts/contracts/SC-GWBASEFLOW-001.md",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/groundwater.rs",
        "docs/work-packages/20260716-assure05-first-production-v2-report-001/artifacts/study-protocol.md",
        "docs/work-packages/20260716-assure05-first-production-v2-report-001/artifacts/realization-freeze.md",
        "docs/work-packages/20260716-assure05-first-production-v2-report-001/prompts/archived/20260716-codex-execute-assure05_prompt.md",
        "docs/work-packages/20260709-laned-active-baseflow-export-closure-001/artifacts/consumer-path-proof.md",
        "docs/work-packages/20260708-groundwater-baseflow-laned-single-ofe-mofe-implementation-001/artifacts/consumer-path-proof.md",
    ] {
        copy_file(&source, &target.path, relative);
    }
    target
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

fn copy_file(source_root: &Path, target_root: &Path, relative: &str) {
    let target = target_root.join(relative);
    fs::create_dir_all(target.parent().unwrap()).unwrap();
    fs::copy(source_root.join(relative), target).unwrap();
}

fn collect_files(root: &Path) -> std::collections::BTreeMap<PathBuf, Vec<u8>> {
    let mut files = std::collections::BTreeMap::new();
    collect_files_into(root, root, &mut files);
    files
}

fn collect_modes(root: &Path) -> std::collections::BTreeMap<PathBuf, u32> {
    use std::os::unix::fs::PermissionsExt as _;

    let mut modes = std::collections::BTreeMap::new();
    collect_modes_into(root, root, &mut modes);
    modes.insert(
        PathBuf::from("."),
        fs::metadata(root).unwrap().permissions().mode() & 0o7777,
    );
    modes
}

fn collect_modes_into(
    root: &Path,
    directory: &Path,
    modes: &mut std::collections::BTreeMap<PathBuf, u32>,
) {
    use std::os::unix::fs::PermissionsExt as _;

    for entry in fs::read_dir(directory).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        modes.insert(
            path.strip_prefix(root).unwrap().to_path_buf(),
            fs::metadata(&path).unwrap().permissions().mode() & 0o7777,
        );
        if entry.file_type().unwrap().is_dir() {
            collect_modes_into(root, &path, modes);
        }
    }
}

fn collect_files_into(
    root: &Path,
    directory: &Path,
    files: &mut std::collections::BTreeMap<PathBuf, Vec<u8>>,
) {
    for entry in fs::read_dir(directory).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            collect_files_into(root, &entry.path(), files);
        } else {
            files.insert(
                entry.path().strip_prefix(root).unwrap().to_path_buf(),
                fs::read(entry.path()).unwrap(),
            );
        }
    }
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

struct Scratch {
    path: PathBuf,
}

impl Scratch {
    fn new(label: &str) -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let counter = COUNTER.fetch_add(1, Ordering::Relaxed);
        let path =
            std::env::temp_dir().join(format!("openwepp-{label}-{}-{counter}", std::process::id()));
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
