use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use openwepp_assurance::{AssuranceError, V2Repository};

const REPORT_ID: &str = "linear-groundwater-reservoir-recurrence";
const CANOPY_REPORT_ID: &str = "native-forest-canopy-phenology-evaluation";
const REPORT_PATH: &str =
    "assurance/v2/reports/linear-groundwater-reservoir-recurrence/report.yaml";
const RESULT_PATH: &str =
    "assurance/v2/reports/linear-groundwater-reservoir-recurrence/results/two-day-recurrence.json";
const OUTPUT_BASE: &str = "usersum/assurance/reports/linear-groundwater-reservoir-recurrence/1.0.0";

#[test]
fn real_named_and_all_builds_are_deterministic_equivalent_and_checkable() {
    let root = repository_root();
    let repository = V2Repository::open(&root).expect("open v2 repository");
    let named_stage = prepared_stage("assure04c-named-stage");
    let all_stage = prepared_stage("assure04c-all-stage");

    let named = repository
        .build_report(REPORT_ID, &named_stage.path)
        .expect("build named report");
    let first_bytes = collect_files(&named_stage.path.join(OUTPUT_BASE));
    let repeated = repository
        .build_report(REPORT_ID, &named_stage.path)
        .expect("repeat named build");
    assert_eq!(named, repeated);
    assert_eq!(
        first_bytes,
        collect_files(&named_stage.path.join(OUTPUT_BASE))
    );

    let all = repository
        .build_all(&all_stage.path)
        .expect("build all reports");
    assert!(all.outputs.len() > named.outputs.len());
    for (path, digest) in &named.outputs {
        assert_eq!(all.outputs.get(path), Some(digest));
    }
    assert_eq!(
        first_bytes,
        collect_files(&all_stage.path.join(OUTPUT_BASE))
    );
    assert_eq!(
        repository
            .check_report(REPORT_ID, &named_stage.path)
            .expect("check named report"),
        named
    );
    assert_eq!(
        repository
            .check_all(&all_stage.path)
            .expect("check all reports"),
        all
    );

    let report = read_text(&named_stage.path.join(OUTPUT_BASE).join("index.md"));
    let supplement = read_text(&named_stage.path.join(OUTPUT_BASE).join("supplement.md"));
    assert!(report.starts_with("# Verification of openWEPP"));
    assert!(report.contains("*Version 1.0 draft — 2026-07-16*"));
    assert!(report.contains("## Plain-Language Summary"));
    assert!(report.contains("1.78e-15"));
    assert!(report.contains("1.78e-15 m3"));
    assert!(report.contains("## Revision Log"));
    assert!(report.contains("technical supplement"));
    assert!(supplement.contains("main report"));
    assert!(!report.contains("{{"));
    assert!(!supplement.contains("{{"));
    assert!(!report.contains(root.to_string_lossy().as_ref()));
    assert!(!supplement.contains(root.to_string_lossy().as_ref()));

    assert_literal_segments_preserved(
        &read_text(
            &root
                .join("assurance/v2/reports/linear-groundwater-reservoir-recurrence/manuscript.md"),
        ),
        &report,
    );
    assert_literal_segments_preserved(
        &read_text(
            &root
                .join("assurance/v2/reports/linear-groundwater-reservoir-recurrence/supplement.md"),
        ),
        &supplement,
    );
}

#[test]
fn canopy_named_and_all_builds_are_byte_equivalent_and_complete() {
    let root = repository_root();
    let repository = V2Repository::open(&root).expect("open repository");
    let named_stage = prepared_stage("assure-maint02-canopy-named");
    let all_stage = prepared_stage("assure-maint02-canopy-all");
    let named = repository
        .build_report(CANOPY_REPORT_ID, &named_stage.path)
        .expect("build named canopy report");
    repository
        .build_all(&all_stage.path)
        .expect("build complete catalog");
    let relative = PathBuf::from(format!(
        "usersum/assurance/reports/{CANOPY_REPORT_ID}/1.0.0"
    ));
    assert_eq!(
        collect_files(&named_stage.path.join(&relative)),
        collect_files(&all_stage.path.join(&relative))
    );
    assert_eq!(
        named
            .outputs
            .iter()
            .filter(|(path, _)| {
                path.extension().is_some_and(|value| value == "svg")
                    && path
                        .components()
                        .any(|component| component.as_os_str() == "figures")
            })
            .count(),
        9
    );
    for required in ["index.md", "supplement.md", "build-manifest.json"] {
        assert!(named_stage.path.join(&relative).join(required).is_file());
    }
    let research_objects = named_stage.path.join(&relative).join("research-objects");
    let mut retained_svg_count = 0;
    for entry in fs::read_dir(research_objects).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|extension| extension.to_str()) != Some("svg") {
            continue;
        }
        retained_svg_count += 1;
        let svg = read_text(&path);
        assert!(svg.contains("role=\"img\""));
        assert!(svg.contains("<title>"));
        assert!(svg.contains("<desc>"));
    }
    assert_eq!(retained_svg_count, 8);
    assert_eq!(
        repository
            .check_report(CANOPY_REPORT_ID, &named_stage.path)
            .expect("check named canopy report"),
        named
    );
}

#[test]
fn rendered_tables_figures_references_objects_and_links_are_real_consumers() {
    let root = repository_root();
    let repository = V2Repository::open(&root).expect("open v2 repository");
    let stage = prepared_stage("assure04c-rendered-consumer");
    repository
        .build_report(REPORT_ID, &stage.path)
        .expect("build rendered consumer");

    let report_path = stage.path.join(OUTPUT_BASE).join("index.md");
    let report = read_text(&report_path);
    assert!(report.contains("| Day | Recharge (`m3`)"));
    assert!(report.contains("| First day | 2.0 |"));
    assert!(report.contains("| Second day | 4.0 |"));
    assert!(report.contains("https://doi.org/10.13031/2013.42691"));
    assert!(report.contains("figures/GW-FIGURE-TWO-DAY.svg"));
    assert!(report.contains("research-objects/two-day-recurrence.json"));
    assert!(report.contains("research-objects/SC-GWBASEFLOW-001.md"));
    assert!(report.contains("../../../../hillslope-hydrology-and-sediment-physics.md"));

    for figure in ["GW-FIGURE-TWO-DAY", "GW-FIGURE-H2637"] {
        let svg = read_text(
            &stage
                .path
                .join(OUTPUT_BASE)
                .join("figures")
                .join(format!("{figure}.svg")),
        );
        assert!(svg.contains("role=\"img\""));
        assert!(svg.contains("<title>"));
        assert!(svg.contains("<desc>"));
        assert!(svg.contains("pattern"));
        assert!(!svg.contains(root.to_string_lossy().as_ref()));
    }

    for (object, source_relative) in [
        ("two-day-recurrence.json", "results/two-day-recurrence.json"),
        ("h2637-ledger.json", "results/h2637-ledger.json"),
        (
            "assure05-path-currency.json",
            "results/assure05-path-currency.json",
        ),
        (
            "assure05-focused-tests.json",
            "results/assure05-focused-tests.json",
        ),
        (
            "two-day-recurrence-input.json",
            "inputs/two-day-recurrence-input.json",
        ),
        (
            "reproduce_groundwater_report.py",
            "procedures/reproduce_groundwater_report.py",
        ),
        (
            "assure05-production-evidence.json",
            "evidence/assure05-production-evidence.json",
        ),
        (
            "agent-assistance-packet.json",
            "evidence/agent-assistance-packet.json",
        ),
        ("manifest.json", "evidence/h2637/manifest.json"),
        ("H2637.hbp", "evidence/h2637/H2637.hbp"),
        ("H2637.pass.parquet", "evidence/h2637/H2637.pass.parquet"),
    ] {
        assert_staged_research_object(&root, &stage.path, object, source_relative);
    }
    assert_eq!(
        fs::read(
            stage
                .path
                .join(OUTPUT_BASE)
                .join("research-objects/SC-GWBASEFLOW-001.md")
        )
        .unwrap(),
        fs::read(root.join("docs/specifications/science-contracts/contracts/SC-GWBASEFLOW-001.md"))
            .unwrap()
    );
    assert_eq!(
        fs::read(
            stage
                .path
                .join(OUTPUT_BASE)
                .join("research-objects/20260716-codex-execute-assure05_prompt.md")
        )
        .unwrap(),
        fs::read(root.join("docs/work-packages/20260716-assure05-first-production-v2-report-001/prompts/archived/20260716-codex-execute-assure05_prompt.md"))
            .unwrap()
    );
    assert_local_links_resolve(&stage.path, &report_path);
}

fn assert_staged_research_object(root: &Path, stage: &Path, object: &str, source_relative: &str) {
    let staged = stage
        .join(OUTPUT_BASE)
        .join("research-objects")
        .join(object);
    let source = root
        .join("assurance/v2/reports/linear-groundwater-reservoir-recurrence")
        .join(source_relative);
    if object == "agent-assistance-packet.json" {
        let mut staged_packet: serde_json::Value =
            serde_json::from_slice(&fs::read(staged).unwrap()).unwrap();
        let governance = staged_packet
            .as_object_mut()
            .unwrap()
            .remove("current_governance")
            .expect("generated current governance");
        let source_packet: serde_json::Value =
            serde_json::from_slice(&fs::read(source).unwrap()).unwrap();
        assert_eq!(staged_packet, source_packet);
        assert_eq!(governance["generated"], true);
        assert_eq!(governance["lifecycle"], "DRAFT");
        assert_eq!(governance["scientific_approval_complete"], false);
    } else {
        assert_eq!(fs::read(staged).unwrap(), fs::read(source).unwrap());
    }
}

#[test]
fn stale_missing_unit_precision_orphan_and_figure_drift_fail_closed() {
    let stale = fixture("assure04c-stale");
    let stale_stage = prepared_stage("assure04c-stale-stage");
    V2Repository::open(&stale.path)
        .unwrap()
        .build_report(REPORT_ID, &stale_stage.path)
        .expect("build baseline staging");
    let baseline = collect_files(&stale_stage.path.join(OUTPUT_BASE));
    append_bytes(&stale.path.join(RESULT_PATH), b"\n");
    let error = V2Repository::open(&stale.path)
        .unwrap()
        .build_report(REPORT_ID, &stale_stage.path)
        .expect_err("stale result blocks build");
    assert!(error.to_string().contains("not current"));
    assert_eq!(baseline, collect_files(&stale_stage.path.join(OUTPUT_BASE)));

    let unit = fixture("assure04c-unit");
    mutate_report(
        &unit.path,
        "  unit_id: m3\n  transform: identity\n  display: fixed:1",
        "  unit_id: d_inv\n  transform: identity\n  display: fixed:1",
    );
    assert_build_rejected(&unit.path, "unit");

    let orphan = fixture("assure04c-orphan");
    let manuscript = orphan
        .path
        .join("assurance/v2/reports/linear-groundwater-reservoir-recurrence/manuscript.md");
    let manuscript_text = fs::read_to_string(&manuscript).expect("read orphan manuscript");
    let orphan_binding = "{{quantity:GW-VALUE-MAX-RESIDUAL-SUMMARY}}";
    assert_eq!(manuscript_text.matches(orphan_binding).count(), 2);
    fs::write(
        &manuscript,
        manuscript_text.replace(orphan_binding, "1.78e-15"),
    )
    .expect("write orphan manuscript");
    refresh_local_hash(&orphan.path, manuscript.strip_prefix(&orphan.path).unwrap());
    refresh_report_hash(&orphan.path);
    assert_build_rejected(&orphan.path, "unused value binding");

    let precision = fixture("assure04c-precision");
    let precision_stage = prepared_stage("assure04c-precision-stage");
    V2Repository::open(&precision.path)
        .unwrap()
        .build_report(REPORT_ID, &precision_stage.path)
        .expect("build precision baseline");
    mutate_report(
        &precision.path,
        "  display: scientific:2",
        "  display: scientific:3",
    );
    let error = V2Repository::open(&precision.path)
        .unwrap()
        .check_report(REPORT_ID, &precision_stage.path)
        .expect_err("changed precision makes staging stale");
    assert!(matches!(error, AssuranceError::Drift(_)));

    let figure_stage = prepared_stage("assure04c-figure-stage");
    let repository = V2Repository::open(repository_root()).unwrap();
    repository
        .build_report(REPORT_ID, &figure_stage.path)
        .expect("build figure baseline");
    fs::remove_file(
        figure_stage
            .path
            .join(OUTPUT_BASE)
            .join("figures/GW-FIGURE-H2637.svg"),
    )
    .expect("remove staged figure");
    assert!(matches!(
        repository
            .check_report(REPORT_ID, &figure_stage.path)
            .unwrap_err(),
        AssuranceError::Drift(_)
    ));
}

#[test]
fn malformed_duplicate_unsafe_link_and_inaccessible_figure_fail_closed() {
    let malformed = fixture("assure04c-malformed-directive");
    mutate_manuscript(
        &malformed.path,
        "{{quantity:GW-VALUE-MAX-RESIDUAL-SUMMARY}}",
        "{{expression:GW-VALUE-MAX-RESIDUAL-SUMMARY}}",
    );
    assert_build_rejected(&malformed.path, "unknown assembly directive");

    let duplicate = fixture("assure04c-duplicate-figure");
    mutate_manuscript(
        &duplicate.path,
        "{{figure:GW-FIGURE-TWO-DAY}}",
        "{{figure:GW-FIGURE-TWO-DAY}}\n\n{{figure:GW-FIGURE-TWO-DAY}}",
    );
    assert_build_rejected(&duplicate.path, "rendered more than once");

    let unsafe_link = fixture("assure04c-unsafe-link");
    mutate_manuscript(
        &unsafe_link.path,
        "{{link:supplement|technical supplement}}",
        "[technical supplement](/tmp/escape)",
    );
    assert_build_rejected(&unsafe_link.path, "typed link directives");

    let bare_link = fixture("assure04c-bare-link");
    mutate_manuscript(
        &bare_link.path,
        "Expected hashes are currency checks",
        "See https://attacker.invalid. Expected hashes are currency checks",
    );
    assert_build_rejected(&bare_link.path, "autolinks must use typed link directives");

    let raw_quantity = fixture("assure04c-raw-quantity");
    mutate_manuscript(
        &raw_quantity.path,
        "Expected hashes are currency checks",
        "An unbound claim says 999.0 m3. Expected hashes are currency checks",
    );
    assert_build_rejected(&raw_quantity.path, "must use a typed quantity directive");

    let missing_link = fixture("assure04c-missing-link");
    mutate_manuscript(
        &missing_link.path,
        "{{link:usersum:hillslope-hydrology-and-sediment-physics.md|model-science narrative}}",
        "{{link:usersum:missing-reader-document.md|model-science narrative}}",
    );
    assert_build_rejected(&missing_link.path, "missing-reader-document.md");

    let zero_figure = fixture("assure04c-zero-figure");
    let result = zero_figure.path.join(RESULT_PATH);
    replace_in(
        &result,
        "\"value\": 1.7763568394002505e-15",
        "\"value\": 0.0",
    );
    refresh_local_hash(&zero_figure.path, Path::new(RESULT_PATH));
    assert_build_rejected(&zero_figure.path, "positive absolute value bindings");
}

#[test]
fn manifest_markdown_metadata_is_escaped_without_creating_external_links() {
    let markdown_fixture = fixture("assure04c-markdown-injection");
    mutate_report(
        &markdown_fixture.path,
        "Maximum binary64-versus-decimal arithmetic residual compared with the separate Rust assertion allowance for the two-day analytical vector.",
        "Evidence [external] <script>",
    );
    let stage = prepared_stage("assure04c-markdown-injection-stage");
    V2Repository::open(&markdown_fixture.path)
        .unwrap()
        .build_report(REPORT_ID, &stage.path)
        .expect("typed metadata must render safely");
    let report = read_text(&stage.path.join(OUTPUT_BASE).join("index.md"));
    assert!(report.contains("\\[external\\] &lt;script&gt;"));
    assert!(!report.contains("<script>"));
    assert!(report.contains("&lt;script&gt;"));

    let external = fixture("assure04c-metadata-external-link");
    mutate_report(
        &external.path,
        "Maximum binary64-versus-decimal arithmetic residual compared with the separate Rust assertion allowance for the two-day analytical vector.",
        "Evidence https://attacker.invalid",
    );
    assert_build_rejected(&external.path, "cannot introduce an external link");
}

#[test]
fn staging_is_sandboxed_exact_and_named_build_preserves_unrelated_reports() {
    let root = repository_root();
    let repository = V2Repository::open(&root).expect("open v2 repository");
    for protected in [root.clone(), root.join("usersum"), root.join("assurance")] {
        let error = repository
            .build_report(REPORT_ID, protected)
            .expect_err("protected staging root rejected");
        assert!(
            error.to_string().contains("staging root"),
            "unexpected protected-root error: {error}"
        );
    }
    let rejected_new = root.join("assurance/assure04c-rejected-new-stage");
    assert!(!rejected_new.exists());
    let error = repository
        .build_report(REPORT_ID, &rejected_new)
        .expect_err("nonexistent protected staging root rejected before creation");
    assert!(error.to_string().contains("staging root"));
    assert!(!rejected_new.exists());

    let stage = prepared_stage("assure04c-isolation");
    let unrelated = stage
        .path
        .join("usersum/assurance/reports/unrelated/0.1.0/sentinel.txt");
    fs::create_dir_all(unrelated.parent().unwrap()).unwrap();
    fs::write(&unrelated, b"keep me").unwrap();
    let temporary = stage
        .path
        .join(format!("usersum/assurance/reports/.{REPORT_ID}.next"));
    fs::create_dir_all(&temporary).unwrap();
    fs::write(temporary.join("stale-partial.txt"), b"remove me").unwrap();
    repository
        .build_report(REPORT_ID, &stage.path)
        .expect("build selected report");
    assert_eq!(fs::read(&unrelated).unwrap(), b"keep me");
    assert!(!temporary.exists());

    let selected_before_failure = collect_files(&stage.path.join(OUTPUT_BASE));
    fs::write(&temporary, b"not a directory").unwrap();
    let error = repository
        .build_report(REPORT_ID, &stage.path)
        .expect_err("special replacement path rejected");
    assert!(error.to_string().contains("not a real directory"));
    assert_eq!(
        selected_before_failure,
        collect_files(&stage.path.join(OUTPUT_BASE))
    );
    fs::remove_file(&temporary).unwrap();

    let backup = stage
        .path
        .join(format!("usersum/assurance/reports/.{REPORT_ID}.previous"));
    fs::create_dir_all(&backup).unwrap();
    fs::write(backup.join("sentinel.txt"), b"do not replace").unwrap();
    let error = repository
        .build_report(REPORT_ID, &stage.path)
        .expect_err("pre-existing backup blocks replacement");
    assert!(error.to_string().contains("backup already exists"));
    assert_eq!(
        selected_before_failure,
        collect_files(&stage.path.join(OUTPUT_BASE))
    );
    assert_eq!(
        fs::read(backup.join("sentinel.txt")).unwrap(),
        b"do not replace"
    );
    fs::remove_dir_all(&backup).unwrap();

    let extra = stage.path.join(OUTPUT_BASE).join("extra.txt");
    fs::write(&extra, b"not declared").unwrap();
    let error = repository
        .check_report(REPORT_ID, &stage.path)
        .expect_err("extra staged file rejected");
    assert!(matches!(error, AssuranceError::Drift(_)));
    assert_eq!(fs::read(&unrelated).unwrap(), b"keep me");

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;

        let symlink_stage = Scratch::new("assure04c-symlink");
        let outside = Scratch::new("assure04c-symlink-outside");
        symlink(&outside.path, symlink_stage.path.join("usersum")).unwrap();
        let error = repository
            .build_report(REPORT_ID, &symlink_stage.path)
            .unwrap_err()
            .to_string();
        assert!(error.contains("symlink") || error.contains("not a directory"));
        assert!(collect_files(&outside.path).is_empty());
    }
}

#[test]
fn real_cli_selects_v2_staging_without_weakening_zero_public_operations() {
    let stage = prepared_stage("assure04c-cli");
    let build = openwepp_assurance::cli::run(vec![
        std::ffi::OsString::from("openwepp-assurance"),
        std::ffi::OsString::from("build"),
        std::ffi::OsString::from("--report"),
        std::ffi::OsString::from(REPORT_ID),
        std::ffi::OsString::from("--staging-root"),
        stage.path.as_os_str().to_owned(),
    ])
    .expect("build report through real CLI");
    assert!(build.starts_with("build: PASS\nreports: 1\n"));
    let check = openwepp_assurance::cli::run(vec![
        std::ffi::OsString::from("openwepp-assurance"),
        std::ffi::OsString::from("check"),
        std::ffi::OsString::from("--report"),
        std::ffi::OsString::from(REPORT_ID),
        std::ffi::OsString::from("--staging-root"),
        stage.path.as_os_str().to_owned(),
    ])
    .expect("check named report through real CLI");
    assert_eq!(build, check.replacen("check:", "build:", 1));

    let all_stage = prepared_stage("assure04c-cli-all");
    let all_build = openwepp_assurance::cli::run(vec![
        std::ffi::OsString::from("openwepp-assurance"),
        std::ffi::OsString::from("build"),
        std::ffi::OsString::from("--all"),
        std::ffi::OsString::from("--staging-root"),
        all_stage.path.as_os_str().to_owned(),
    ])
    .expect("build all reports through real CLI");
    assert!(all_build.starts_with("build: PASS\nreports: 3\n"));
    let all_check = openwepp_assurance::cli::run(vec![
        std::ffi::OsString::from("openwepp-assurance"),
        std::ffi::OsString::from("check"),
        std::ffi::OsString::from("--all"),
        std::ffi::OsString::from("--staging-root"),
        all_stage.path.as_os_str().to_owned(),
    ])
    .expect("check all reports through real CLI");
    assert_eq!(all_build, all_check.replacen("check:", "build:", 1));

    let zero = openwepp_assurance::cli::run(["openwepp-assurance", "check", "--all"])
        .expect("zero-public check remains available");
    assert!(zero.starts_with("check: PASS\nreports: 0\n"));
    let no_stage =
        openwepp_assurance::cli::run(["openwepp-assurance", "build", "--report", REPORT_ID])
            .unwrap_err();
    assert!(no_stage.to_string().contains("--staging-root"));
    let mixed = openwepp_assurance::cli::run(vec![
        std::ffi::OsString::from("openwepp-assurance"),
        std::ffi::OsString::from("build"),
        std::ffi::OsString::from("--all"),
        std::ffi::OsString::from("--staging-root"),
        stage.path.as_os_str().to_owned(),
        std::ffi::OsString::from("--snapshot"),
        std::ffi::OsString::from("forbidden"),
        std::ffi::OsString::from("--snapshot-root"),
        stage.path.as_os_str().to_owned(),
    ])
    .unwrap_err();
    assert!(mixed.to_string().contains("staging"));
}

#[test]
fn mtime_changes_do_not_affect_assembly_bytes() {
    use std::fs::FileTimes;
    use std::time::{Duration, SystemTime};

    let fixture = fixture("assure04c-mtime");
    let before_stage = prepared_stage("assure04c-mtime-before");
    let after_stage = prepared_stage("assure04c-mtime-after");
    let repository = V2Repository::open(&fixture.path).unwrap();
    repository
        .build_all(&before_stage.path)
        .expect("build before mtime change");
    let manuscript = fixture
        .path
        .join("assurance/v2/reports/linear-groundwater-reservoir-recurrence/manuscript.md");
    fs::OpenOptions::new()
        .write(true)
        .open(manuscript)
        .unwrap()
        .set_times(FileTimes::new().set_modified(SystemTime::now() + Duration::from_secs(7200)))
        .unwrap();
    V2Repository::open(&fixture.path)
        .unwrap()
        .build_all(&after_stage.path)
        .expect("build after mtime change");
    assert_eq!(
        collect_files(&before_stage.path.join(OUTPUT_BASE)),
        collect_files(&after_stage.path.join(OUTPUT_BASE))
    );
}

fn assert_build_rejected(root: &Path, expected: &str) {
    let stage = prepared_stage("assure04c-rejected");
    let error = V2Repository::open(root)
        .and_then(|repository| repository.build_report(REPORT_ID, &stage.path))
        .expect_err("assembly fixture must fail closed");
    assert!(
        error.to_string().contains(expected),
        "expected '{expected}', observed '{error}'"
    );
}

fn assert_local_links_resolve(stage_root: &Path, markdown_path: &Path) {
    let text = read_text(markdown_path);
    for destination in markdown_destinations(&text) {
        if destination.starts_with("https://") {
            continue;
        }
        let target = markdown_path.parent().unwrap().join(destination);
        assert!(
            target.is_file(),
            "unresolved rendered link: {}",
            target.display()
        );
        assert!(target.starts_with(stage_root));
    }
}

fn markdown_destinations(text: &str) -> Vec<&str> {
    let mut destinations = Vec::new();
    let mut remaining = text;
    while let Some(open) = remaining.find("](") {
        let after = &remaining[open + 2..];
        let Some(close) = after.find(')') else {
            break;
        };
        destinations.push(&after[..close]);
        remaining = &after[close + 1..];
    }
    destinations
}

fn prepared_stage(label: &str) -> Scratch {
    let stage = Scratch::new(label);
    for relative in [
        "usersum/hillslope-hydrology-and-sediment-physics.md",
        "usersum/openwepp-canopy-phenology.md",
    ] {
        let source = repository_root().join(relative);
        let target = stage.path.join(relative);
        fs::create_dir_all(target.parent().unwrap()).unwrap();
        fs::copy(source, target).unwrap();
    }
    stage
}

fn mutate_report(root: &Path, old: &str, new: &str) {
    replace_in(&root.join(REPORT_PATH), old, new);
    refresh_report_hash(root);
}

fn mutate_manuscript(root: &Path, old: &str, new: &str) {
    let relative =
        Path::new("assurance/v2/reports/linear-groundwater-reservoir-recurrence/manuscript.md");
    replace_in(&root.join(relative), old, new);
    refresh_local_hash(root, relative);
    refresh_report_hash(root);
}

fn assert_literal_segments_preserved(source: &str, rendered: &str) {
    let mut source_cursor = 0;
    let mut rendered_cursor = 0;
    while let Some(start_offset) = source[source_cursor..].find("{{") {
        let start = source_cursor + start_offset;
        let literal = &source[source_cursor..start];
        let observed = rendered[rendered_cursor..]
            .find(literal)
            .expect("authored literal segment must remain in rendered order");
        rendered_cursor += observed + literal.len();
        let directive_end = source[start + 2..]
            .find("}}")
            .expect("source directive must terminate")
            + start
            + 4;
        source_cursor = directive_end;
    }
    let literal = &source[source_cursor..];
    assert!(
        rendered[rendered_cursor..].ends_with(literal),
        "final authored literal segment must be byte-identical"
    );
}

fn refresh_report_hash(root: &Path) {
    openwepp_assurance::rebind_v2_test_fixture(root).expect("rebind fixture identity");
}

fn refresh_local_hash(root: &Path, _relative: &Path) {
    openwepp_assurance::rebind_v2_test_fixture(root).expect("rebind fixture identity");
}

fn replace_in(path: &Path, old: &str, new: &str) {
    let text = fs::read_to_string(path).unwrap();
    assert!(text.contains(old), "replacement source missing: {old}");
    fs::write(path, text.replacen(old, new, 1)).unwrap();
}

fn append_bytes(path: &Path, suffix: &[u8]) {
    let mut bytes = fs::read(path).unwrap();
    bytes.extend_from_slice(suffix);
    fs::write(path, bytes).unwrap();
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

fn copy_file(source_root: &Path, target_root: &Path, relative: &str) {
    let target = target_root.join(relative);
    fs::create_dir_all(target.parent().unwrap()).unwrap();
    fs::copy(source_root.join(relative), target).unwrap();
}

fn collect_files(root: &Path) -> std::collections::BTreeMap<PathBuf, Vec<u8>> {
    let mut files = std::collections::BTreeMap::new();
    if root.exists() {
        collect_files_into(root, root, &mut files);
    }
    files
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

fn read_text(path: &Path) -> String {
    fs::read_to_string(path).unwrap()
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
