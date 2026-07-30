use std::fs::{self, FileTimes};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime};

use openwepp_assurance::{V2PlanState, V2Repository, sha256_bytes};

const REPORT_ID: &str = "linear-groundwater-reservoir-recurrence";
const SNOW_REPORT_ID: &str = "snow-and-frozen-soil-process-evaluation";
const SECOND_REPORT_ID: &str = "second-groundwater-report";
const REPORT_DIR: &str = "assurance/v2/reports/linear-groundwater-reservoir-recurrence";
const REPORT_PATH: &str =
    "assurance/v2/reports/linear-groundwater-reservoir-recurrence/report.yaml";
const MANUSCRIPT_PATH: &str =
    "assurance/v2/reports/linear-groundwater-reservoir-recurrence/manuscript.md";
const RESULT_PATH: &str =
    "assurance/v2/reports/linear-groundwater-reservoir-recurrence/results/two-day-recurrence.json";
const CONTRACT_PATH: &str = "docs/specifications/science-contracts/contracts/SC-GWBASEFLOW-001.md";

#[test]
fn current_one_and_all_plans_are_equivalent_stable_and_cli_consumed() {
    let root = repository_root();
    let repository = V2Repository::open(&root).expect("open v2 repository");
    assert!(repository.plan_report("missing-report").is_err());
    let named = repository
        .plan_report(REPORT_ID)
        .expect("plan named report");
    let all = repository.plan_all().expect("plan all reports");

    assert_eq!(named.reports, vec![report(&all, REPORT_ID).clone()]);
    assert_eq!(named.selected_report_count, 1);
    assert_eq!(named.total_report_count, 3);
    assert_eq!(all.selected_report_count, 3);
    assert_eq!(all.total_report_count, 3);
    assert_eq!(all.public_report_count, 0);
    let groundwater = report(&all, REPORT_ID);
    assert_eq!(groundwater.state, V2PlanState::Current);
    assert!(
        groundwater
            .nodes
            .iter()
            .all(|node| node.state == V2PlanState::Current)
    );
    assert_dependency_first(groundwater);
    let snow = report(&all, SNOW_REPORT_ID);
    assert_eq!(snow.state, V2PlanState::Current);
    assert!(
        snow.nodes
            .iter()
            .all(|node| node.state == V2PlanState::Current)
    );
    assert_dependency_first(snow);

    let human = all.render();
    let json = all.render_json().expect("render JSON plan");
    assert_eq!(human, repository.plan_all().expect("repeat plan").render());
    assert_eq!(
        json,
        repository
            .plan_all()
            .expect("repeat JSON")
            .render_json()
            .unwrap()
    );
    let decoded: serde_json::Value = serde_json::from_str(&json).expect("parse plan JSON");
    assert_eq!(decoded["publication_state"], "v1_retired_zero_reports");
    assert_eq!(decoded["reports"][0]["state"], "current");
    assert_eq!(decoded["reports"][1]["state"], "current");

    let cli_human =
        openwepp_assurance::cli::run(["openwepp-assurance", "plan", "--report", REPORT_ID])
            .expect("real named CLI plan");
    let cli_json =
        openwepp_assurance::cli::run(["openwepp-assurance", "plan", "--all", "--format", "json"])
            .expect("real all CLI JSON plan");
    assert_eq!(cli_human, named.render());
    assert_eq!(cli_json, json);
}

#[test]
fn content_changes_select_expected_transitive_consumers() {
    for (label, path, node_id) in [
        ("narrative", MANUSCRIPT_PATH, "GW-MANUSCRIPT"),
        ("result", RESULT_PATH, "GW-RESULT-TWO-DAY"),
        ("contract", CONTRACT_PATH, "GW-DEP-SCIENCE-CONTRACT"),
    ] {
        let fixture = fixture(&format!("assure04b-stale-{label}"));
        append_bytes(&fixture.path.join(path), b"\nchanged for planner test\n");
        let plan = V2Repository::open(&fixture.path)
            .expect("open stale fixture")
            .plan_report(REPORT_ID)
            .expect("plan stale fixture");
        let report = report(&plan, REPORT_ID);
        assert_eq!(node(report, node_id).state, V2PlanState::Stale, "{label}");
        assert_eq!(report.state, V2PlanState::Selected, "{label}");
        assert_eq!(
            node(report, &format!("report:{REPORT_ID}")).state,
            V2PlanState::Selected
        );
    }
}

#[test]
fn manifest_method_figure_review_and_software_changes_select_the_report() {
    for (label, old, new) in [
        (
            "method",
            "description: Apply the published daily recurrence",
            "description: Independently apply the published daily recurrence",
        ),
        (
            "figure",
            "caption: Independent binary64 daily storage",
            "caption: Rechecked independent binary64 daily storage",
        ),
        (
            "review",
            "title: Scientific and publication review state",
            "title: Updated scientific and publication review state",
        ),
        (
            "software",
            "git:01ed70550a4e371e99afe35c4bdd4d9b667e812c",
            "git:0000000000000000000000000000000000000000",
        ),
    ] {
        let fixture = fixture(&format!("assure04b-manifest-{label}"));
        replace_in(&fixture.path.join(REPORT_PATH), old, new);
        let plan = V2Repository::open(&fixture.path)
            .expect("open changed-manifest fixture")
            .plan_report(REPORT_ID)
            .expect("plan changed manifest");
        let report = report(&plan, REPORT_ID);
        assert_eq!(
            node(report, &format!("source:manifest:{REPORT_ID}")).state,
            V2PlanState::Stale,
            "{label}"
        );
        assert_eq!(report.state, V2PlanState::Selected, "{label}");
    }
}

#[test]
fn unavailable_declared_content_blocks_consumers_with_relative_reasons() {
    let fixture = fixture("assure04b-blocked");
    fs::remove_file(fixture.path.join(MANUSCRIPT_PATH)).expect("remove manuscript");
    let plan = V2Repository::open(&fixture.path)
        .expect("open blocked fixture")
        .plan_report(REPORT_ID)
        .expect("blocked content remains explainable");
    let report = report(&plan, REPORT_ID);
    let manuscript = node(report, "GW-MANUSCRIPT");
    assert_eq!(manuscript.state, V2PlanState::Blocked);
    assert!(manuscript.reason.contains(MANUSCRIPT_PATH));
    assert!(
        !manuscript
            .reason
            .contains(fixture.path.to_string_lossy().as_ref())
    );
    assert_eq!(report.state, V2PlanState::Blocked);
}

#[test]
fn stale_consumer_cannot_mask_a_blocked_prerequisite() {
    let fixture = fixture("assure04b-stale-and-blocked");
    append_bytes(&fixture.path.join(MANUSCRIPT_PATH), b"\nstale manuscript\n");
    fs::remove_file(fixture.path.join(RESULT_PATH)).expect("remove result prerequisite");
    let plan = V2Repository::open(&fixture.path)
        .expect("open stale-and-blocked fixture")
        .plan_report(REPORT_ID)
        .expect("plan stale-and-blocked fixture");
    let report = report(&plan, REPORT_ID);
    assert_eq!(
        node(report, "GW-RESULT-TWO-DAY").state,
        V2PlanState::Blocked
    );
    assert_eq!(node(report, "GW-MANUSCRIPT").state, V2PlanState::Blocked);
    assert_eq!(report.state, V2PlanState::Blocked);
}

#[test]
fn unavailable_or_unparseable_selected_manifest_produces_a_bounded_blocked_plan() {
    let missing = fixture("assure04b-missing-manifest");
    fs::remove_file(missing.path.join(REPORT_PATH)).expect("remove manifest");
    assert_blocked_manifest(&missing);

    let malformed = fixture("assure04b-malformed-manifest");
    fs::write(malformed.path.join(REPORT_PATH), b"not: [valid\n")
        .expect("write malformed manifest");
    assert_blocked_manifest(&malformed);
}

#[test]
fn mtime_only_change_does_not_change_plan_bytes() {
    let fixture = fixture("assure04b-mtime");
    let before = V2Repository::open(&fixture.path)
        .expect("open before mtime")
        .plan_all()
        .expect("plan before mtime")
        .render_json()
        .expect("render before mtime");
    let file = fs::OpenOptions::new()
        .write(true)
        .open(fixture.path.join(MANUSCRIPT_PATH))
        .expect("open manuscript for timestamp");
    file.set_times(FileTimes::new().set_modified(SystemTime::now() + Duration::from_secs(3_600)))
        .expect("set modification time");
    let after = V2Repository::open(&fixture.path)
        .expect("open after mtime")
        .plan_all()
        .expect("plan after mtime")
        .render_json()
        .expect("render after mtime");
    assert_eq!(before, after);
}

#[test]
fn named_selection_isolated_and_all_plan_does_not_select_unrelated_report() {
    let fixture = fixture("assure04b-two-reports");
    add_second_report(&fixture.path);
    append_bytes(
        &fixture.path.join(MANUSCRIPT_PATH),
        b"\nfirst report changed\n",
    );

    let repository = V2Repository::open(&fixture.path).expect("open two-report fixture");
    let all = repository.plan_all().expect("plan both reports");
    assert_eq!(report(&all, REPORT_ID).state, V2PlanState::Selected);
    assert_eq!(report(&all, SECOND_REPORT_ID).state, V2PlanState::Current);
    let named = repository
        .plan_report(SECOND_REPORT_ID)
        .expect("plan second report");
    assert_eq!(named.reports, vec![report(&all, SECOND_REPORT_ID).clone()]);

    fs::write(fixture.path.join(REPORT_PATH), b"not: [valid\n").expect("corrupt unselected report");
    let isolated = V2Repository::open(&fixture.path)
        .expect("catalog remains loadable")
        .plan_report(SECOND_REPORT_ID)
        .expect("unselected malformed report is not traversed");
    assert_eq!(
        report(&isolated, SECOND_REPORT_ID).state,
        V2PlanState::Current
    );
}

#[test]
fn plan_format_is_plan_only_and_assembly_requires_staging() {
    for command in ["validate", "build", "check"] {
        let error = openwepp_assurance::cli::run([
            "openwepp-assurance",
            command,
            "--all",
            "--format",
            "json",
        ])
        .expect_err("format must be plan-only");
        assert!(error.to_string().contains("--format is plan-only"));
    }
    for command in ["build", "check"] {
        let error =
            openwepp_assurance::cli::run(["openwepp-assurance", command, "--report", REPORT_ID])
                .expect_err("report assembly requires explicit staging");
        assert!(error.to_string().contains("--staging-root"));
    }

    let build_root = Scratch::new("assure04b-cli-build");
    let build = openwepp_assurance::cli::run(vec![
        std::ffi::OsString::from("openwepp-assurance"),
        std::ffi::OsString::from("build"),
        std::ffi::OsString::from("--all"),
        std::ffi::OsString::from("--output-root"),
        build_root.path.as_os_str().to_owned(),
    ])
    .expect("zero-public build remains available through the real CLI");
    assert!(build.starts_with("build: PASS\nreports: 0\n"));

    let check = openwepp_assurance::cli::run(["openwepp-assurance", "check", "--all"])
        .expect("zero-public check remains available through the real CLI");
    assert!(check.starts_with("check: PASS\nreports: 0\n"));
}

#[test]
fn malformed_authority_remains_an_explicit_blocker() {
    let unsafe_path = fixture("assure04b-unsafe-path");
    replace_in(
        &unsafe_path.path.join(REPORT_PATH),
        "path: docs/specifications/science-contracts/contracts/SC-GWBASEFLOW-001.md",
        "path: ../outside.md",
    );
    openwepp_assurance::rebind_invalid_v2_test_fixture(&unsafe_path.path)
        .expect("rebind deliberately invalid authority fixture");
    let plan = V2Repository::open(&unsafe_path.path)
        .expect("open unsafe-path catalog")
        .plan_report(REPORT_ID)
        .expect("planner must expose malformed authority as blocked");
    let report = report(&plan, REPORT_ID);
    assert_eq!(report.state, V2PlanState::Blocked);
    assert!(
        node(report, &format!("source:manifest:{REPORT_ID}"))
            .reason
            .contains("confined relative path")
    );

    let malformed_digest = fixture("assure04b-malformed-digest");
    append_bytes(
        &malformed_digest
            .path
            .join("assurance/v2/identity.lock.json"),
        b"\n",
    );
    assert!(
        V2Repository::open(&malformed_digest.path)
            .unwrap_err()
            .to_string()
            .contains("canonical JSON")
    );

    let invalid_result = fixture("assure04b-invalid-current-result");
    let result_path = invalid_result.path.join(RESULT_PATH);
    fs::write(&result_path, b"{").expect("write invalid result");
    rebind_catalog_manifest(&invalid_result.path);
    assert!(
        V2Repository::open(&invalid_result.path)
            .expect("open current invalid-result catalog")
            .plan_report(REPORT_ID)
            .unwrap_err()
            .to_string()
            .contains(RESULT_PATH)
    );
}

fn report<'a>(
    plan: &'a openwepp_assurance::V2Plan,
    id: &str,
) -> &'a openwepp_assurance::V2ReportPlan {
    plan.reports
        .iter()
        .find(|report| report.id == id)
        .expect("report in plan")
}

fn assert_blocked_manifest(fixture: &Scratch) {
    let plan = V2Repository::open(&fixture.path)
        .expect("open selected blocked-manifest fixture")
        .plan_report(REPORT_ID)
        .expect("manifest failure remains explainable");
    let report = report(&plan, REPORT_ID);
    assert_eq!(report.state, V2PlanState::Blocked);
    assert_eq!(
        node(report, &format!("source:manifest:{REPORT_ID}")).state,
        V2PlanState::Blocked
    );
    assert!(
        !plan
            .render()
            .contains(fixture.path.to_string_lossy().as_ref())
    );
}

fn node<'a>(
    report: &'a openwepp_assurance::V2ReportPlan,
    id: &str,
) -> &'a openwepp_assurance::V2PlanNode {
    report
        .nodes
        .iter()
        .find(|node| node.id == id)
        .expect("node in plan")
}

fn assert_dependency_first(report: &openwepp_assurance::V2ReportPlan) {
    let positions = report
        .nodes
        .iter()
        .enumerate()
        .map(|(index, node)| (&node.id, index))
        .collect::<std::collections::BTreeMap<_, _>>();
    for planned in &report.nodes {
        for dependency in &planned.dependencies {
            assert!(positions[dependency] < positions[&planned.id]);
        }
    }
}

fn add_second_report(root: &Path) {
    let first = root.join(REPORT_DIR);
    let second_rel = format!("assurance/v2/reports/{SECOND_REPORT_ID}");
    let second = root.join(&second_rel);
    copy_tree(&first, &second);

    let mut manifest = fs::read_to_string(second.join("report.yaml")).expect("read second report");
    manifest = manifest
        .replace(REPORT_ID, SECOND_REPORT_ID)
        .replace("GW-", "SECOND-");
    for result in [
        "two-day-recurrence.json",
        "h2637-ledger.json",
        "assure05-path-currency.json",
        "assure05-focused-tests.json",
    ] {
        let path = second.join("results").join(result);
        let before = fs::read(&path).expect("read copied result");
        let old_digest = sha256_bytes(&before);
        let updated = String::from_utf8(before)
            .expect("result UTF-8")
            .replace("GW-", "SECOND-");
        fs::write(&path, updated.as_bytes()).expect("write second result");
        manifest = manifest.replace(&old_digest, &sha256_bytes(updated.as_bytes()));
    }
    fs::write(second.join("report.yaml"), manifest.as_bytes()).expect("write second manifest");
    let catalog_path = root.join("assurance/v2/catalog.yaml");
    let mut catalog: serde_yaml::Value =
        serde_yaml::from_slice(&fs::read(&catalog_path).expect("read catalog"))
            .expect("parse catalog");
    catalog["reports"]
        .as_sequence_mut()
        .expect("catalog reports")
        .push(serde_yaml::from_str(&format!(
            "id: {SECOND_REPORT_ID}\nversion: 1.0.0\ntitle: Verification of openWEPP's Daily Linear Groundwater Reservoir\nowner: openWEPP scientific assurance maintainers\ntrust_domain: production\nfixture_only: false\nmanifest_path: {second_rel}/report.yaml\n"
        )).expect("second catalog entry"));
    fs::write(
        catalog_path,
        serde_yaml::to_string(&catalog).expect("render catalog"),
    )
    .expect("write two-report catalog");
    openwepp_assurance::rebind_v2_test_fixture(root).expect("rebind two-report fixture");
}

fn rebind_catalog_manifest(root: &Path) {
    openwepp_assurance::rebind_v2_test_fixture(root).expect("rebind fixture identity");
}

fn append_bytes(path: &Path, suffix: &[u8]) {
    let mut bytes = fs::read(path).expect("read append target");
    bytes.extend_from_slice(suffix);
    fs::write(path, bytes).expect("append test bytes");
}

fn replace_in(path: &Path, old: &str, new: &str) {
    let text = fs::read_to_string(path).expect("read replacement target");
    assert!(text.contains(old), "replacement source missing: {old}");
    fs::write(path, text.replacen(old, new, 1)).expect("write replacement target");
}

fn fixture(label: &str) -> Scratch {
    let source = repository_root();
    let target = Scratch::new(label);
    openwepp_assurance::copy_v2_test_fixture(&source, &target.path).unwrap();
    openwepp_assurance::retain_v2_test_report(&target.path, REPORT_ID).unwrap();
    for relative in [
        "assurance/catalog.yaml",
        "assurance/templates/catalog.md",
        "assurance/generated/wepppy-usersum.yaml",
        "usersum/assurance/README.md",
        "usersum/hillslope-hydrology-and-sediment-physics.md",
        CONTRACT_PATH,
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
    fs::create_dir_all(target.parent().expect("fixture parent")).expect("create fixture parent");
    fs::copy(source_root.join(relative), target).expect("copy fixture file");
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
