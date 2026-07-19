use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use openwepp_assurance::{Assurance, AssuranceError, BuildOptions, sha256_bytes};

const BASE_COMMIT: &str = "3352388465f8b288aed4636e8f9752ca6c1cceb9";
const PACKAGE_ROOT: &str = "docs/work-packages/20260714-assure03-v1-retirement-zero-report-001";

#[test]
fn public_builder_stays_zero_report_while_validation_admits_internal_v2() {
    let root = repository_root();
    let assurance = Assurance::open(&root).expect("load zero-report source");
    assurance.validate().expect("validate zero-report source");
    let plan = assurance.plan().expect("plan zero-report build");
    assert_eq!(plan.publication_state, "v1_retired_zero_reports");
    assert_eq!(
        plan.inputs.keys().cloned().collect::<BTreeSet<_>>(),
        BTreeSet::from([
            PathBuf::from("assurance/catalog.yaml"),
            PathBuf::from("assurance/templates/catalog.md"),
        ])
    );
    assert_eq!(
        plan.outputs.keys().cloned().collect::<BTreeSet<_>>(),
        BTreeSet::from([
            PathBuf::from("assurance/generated/wepppy-usersum.yaml"),
            PathBuf::from("usersum/assurance/README.md"),
        ])
    );
    assurance
        .check()
        .expect("tracked zero-report outputs current");

    let validation = openwepp_assurance::cli::run(["openwepp-assurance", "validate", "--all"])
        .expect("run public and internal-source validate CLI");
    assert!(validation.contains("validation: PASS"));
    assert!(validation.contains("public_reports: 0"));
    assert!(validation.contains("v2_reports_total: 2"));
    let rendered_plan = openwepp_assurance::cli::run(["openwepp-assurance", "plan", "--all"])
        .expect("run public plan CLI");
    assert!(rendered_plan.contains("publication_state: v1_retired_zero_reports"));
    assert!(rendered_plan.contains("reports: 0"));
    assert!(
        openwepp_assurance::cli::run(["openwepp-assurance", "validate", "--dossier", "retired"])
            .is_err()
    );
}

#[test]
fn one_and_repeated_builds_are_byte_deterministic() {
    let root = repository_root();
    let assurance = Assurance::open(&root).expect("load zero-report source");
    let first = Scratch::new("assure03-build-first");
    let second = Scratch::new("assure03-build-second");
    let first_result = assurance
        .build(&output_options(&first.path))
        .expect("first staged build");
    let second_result = assurance
        .build(&output_options(&second.path))
        .expect("second staged build");
    assert_eq!(first_result.outputs, second_result.outputs);
    for path in first_result.outputs.keys() {
        assert_eq!(
            fs::read(first.path.join(path)).expect("read first output"),
            fs::read(second.path.join(path)).expect("read second output"),
            "nondeterministic output {}",
            path.display()
        );
    }

    let catalog =
        fs::read(first.path.join("usersum/assurance/README.md")).expect("read staged catalog");
    assert_eq!(
        catalog,
        fs::read(root.join("assurance/templates/catalog.md")).expect("read source template")
    );
    let export = fs::read_to_string(first.path.join("assurance/generated/wepppy-usersum.yaml"))
        .expect("read staged export");
    assert!(export.contains("documents: []"));
    assert!(export.contains("vendoring_authorized: false"));
}

#[test]
fn nonempty_catalog_and_retired_routes_fail_closed() {
    let nonempty = transition_fixture("assure03-nonempty-catalog");
    replace_in(
        &nonempty.path.join("assurance/catalog.yaml"),
        "dossiers: []",
        "dossiers:\n  - dossier_id: retired-candidate",
    );
    assert!(matches!(
        Assurance::open(&nonempty.path),
        Err(AssuranceError::Invalid(message))
            if message.contains("must contain zero reports")
    ));

    let orphan = transition_fixture("assure03-retired-public-route");
    write_file(
        &orphan
            .path
            .join("usersum/assurance/dossiers/retired-candidate.md"),
        b"retired\n",
    );
    assert!(matches!(
        Assurance::open(&orphan.path),
        Err(AssuranceError::Invalid(message)) if message.contains("retired v1 assurance route")
    ));
}

#[test]
fn check_detects_byte_drift_and_extra_public_files() {
    let drift = transition_fixture("assure03-output-drift");
    let assurance = Assurance::open(&drift.path).expect("load drift fixture");
    assurance
        .build(&BuildOptions::default())
        .expect("build fixture outputs");
    fs::write(drift.path.join("usersum/assurance/README.md"), "stale\n")
        .expect("make catalog stale");
    assert!(matches!(
        assurance.check(),
        Err(AssuranceError::Drift(message)) if message.contains("stale")
    ));

    let extra = transition_fixture("assure03-extra-output");
    let assurance = Assurance::open(&extra.path).expect("load extra fixture");
    assurance
        .build(&BuildOptions::default())
        .expect("build extra fixture outputs");
    write_file(
        &extra.path.join("usersum/assurance/untracked.md"),
        b"not allowed\n",
    );
    assert!(matches!(
        assurance.check(),
        Err(AssuranceError::Drift(message)) if message.contains("only README.md")
    ));
}

#[test]
fn snapshot_is_zero_report_deterministic_and_immutable() {
    let fixture = transition_fixture("assure03-snapshot-source");
    let snapshot_root = Scratch::new("assure03-snapshots");
    let assurance = Assurance::open(&fixture.path).expect("load snapshot fixture");
    assurance
        .build(&BuildOptions::default())
        .expect("build tracked fixture outputs");
    let options = BuildOptions {
        output_root: None,
        snapshot: Some("260714assure03".to_owned()),
        snapshot_root: Some(snapshot_root.path.clone()),
    };
    let created = assurance.build(&options).expect("create snapshot");
    assert!(!created.snapshot_confirmed_existing);
    let manifest_path = created.snapshot_manifest.expect("snapshot manifest path");
    let manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(&manifest_path).expect("read snapshot manifest"))
            .expect("parse snapshot manifest");
    assert_eq!(manifest["publication_state"], "v1_retired_zero_reports");
    assert_eq!(manifest["report_count"], 0);
    assert_eq!(manifest["reports"].as_array().map(Vec::len), Some(0));
    assert_eq!(manifest["files"].as_array().map(Vec::len), Some(2));
    assert!(
        assurance
            .build(&options)
            .expect("confirm existing snapshot")
            .snapshot_confirmed_existing
    );

    let snap_catalog = snapshot_root
        .path
        .join("260714assure03/files/usersum/assurance/README.md");
    fs::write(&snap_catalog, "mutated\n").expect("mutate snapshot fixture");
    assert!(matches!(
        assurance.build(&options),
        Err(AssuranceError::SnapshotConflict(_))
    ));
    let invalid = BuildOptions {
        snapshot: Some("../escape".to_owned()),
        snapshot_root: Some(snapshot_root.path.clone()),
        ..BuildOptions::default()
    };
    assert!(matches!(
        assurance.build(&invalid),
        Err(AssuranceError::Invalid(_))
    ));
}

#[cfg(unix)]
#[test]
fn snapshot_target_and_descendant_symlinks_are_rejected() {
    use std::os::unix::fs::symlink;

    let fixture = transition_fixture("assure03-snapshot-symlink-source");
    let assurance = Assurance::open(&fixture.path).expect("load snapshot fixture");
    assurance
        .build(&BuildOptions::default())
        .expect("build tracked fixture outputs");

    let outside = Scratch::new("assure03-snapshot-symlink-outside");
    let outside_options = BuildOptions {
        output_root: None,
        snapshot: Some("linked".to_owned()),
        snapshot_root: Some(outside.path.clone()),
    };
    assurance
        .build(&outside_options)
        .expect("create complete outside snapshot");

    let snapshot_root = Scratch::new("assure03-snapshot-symlink-root");
    symlink(
        outside.path.join("linked"),
        snapshot_root.path.join("linked"),
    )
    .expect("create snapshot-ID symlink");
    let linked_options = BuildOptions {
        output_root: None,
        snapshot: Some("linked".to_owned()),
        snapshot_root: Some(snapshot_root.path.clone()),
    };
    assert!(matches!(
        assurance.build(&linked_options),
        Err(AssuranceError::Invalid(message)) if message.contains("snapshot target cannot be a symlink")
    ));

    let descendant_root = Scratch::new("assure03-snapshot-descendant-root");
    let descendant_options = BuildOptions {
        output_root: None,
        snapshot: Some("descendant".to_owned()),
        snapshot_root: Some(descendant_root.path.clone()),
    };
    assurance
        .build(&descendant_options)
        .expect("create descendant snapshot");
    let catalog = descendant_root
        .path
        .join("descendant/files/usersum/assurance/README.md");
    fs::remove_file(&catalog).expect("remove snapshot catalog");
    symlink(fixture.path.join("usersum/assurance/README.md"), &catalog)
        .expect("replace snapshot descendant with symlink");
    assert!(matches!(
        assurance.build(&descendant_options),
        Err(AssuranceError::Invalid(message)) if message.contains("cannot contain symlinks")
    ));
}

#[cfg(unix)]
#[test]
fn output_symlink_escape_is_rejected() {
    use std::os::unix::fs::symlink;

    let fixture = transition_fixture("assure03-symlink-source");
    let output = Scratch::new("assure03-symlink-output");
    let outside = Scratch::new("assure03-symlink-outside");
    fs::create_dir_all(output.path.join("usersum")).expect("create output parent");
    symlink(&outside.path, output.path.join("usersum/assurance")).expect("create output symlink");
    let assurance = Assurance::open(&fixture.path).expect("load symlink fixture");
    assert!(matches!(
        assurance.build(&output_options(&output.path)),
        Err(AssuranceError::Invalid(message)) if message.contains("escapes selected root")
    ));
}

#[cfg(unix)]
#[test]
fn public_output_and_snapshot_special_entries_are_rejected() {
    use std::os::unix::net::UnixListener;

    let fixture = transition_fixture("s");
    let assurance = Assurance::open(&fixture.path).expect("load special-entry fixture");
    assurance
        .build(&BuildOptions::default())
        .expect("build tracked fixture outputs");
    let public_socket = fixture.path.join("usersum/assurance/p");
    let public_listener = UnixListener::bind(&public_socket).expect("bind public socket");
    assert!(matches!(
        assurance.check(),
        Err(AssuranceError::Invalid(message)) if message.contains("unsupported filesystem entry")
    ));
    drop(public_listener);
    fs::remove_file(&public_socket).expect("remove public socket");

    let snapshot_root = Scratch::new("t");
    let options = BuildOptions {
        output_root: None,
        snapshot: Some("s".to_owned()),
        snapshot_root: Some(snapshot_root.path.clone()),
    };
    assurance.build(&options).expect("create clean snapshot");
    let snapshot_socket = snapshot_root.path.join("s/p");
    let snapshot_listener = UnixListener::bind(&snapshot_socket).expect("bind snapshot socket");
    assert!(matches!(
        assurance.build(&options),
        Err(AssuranceError::Invalid(message)) if message.contains("unsupported filesystem entry")
    ));
    drop(snapshot_listener);
}

#[test]
fn frozen_v1_manifest_recovers_every_recorded_blob() {
    let root = repository_root();
    let manifest_path = root
        .join(PACKAGE_ROOT)
        .join("artifacts/v1-content-manifest.tsv");
    let manifest = fs::read_to_string(&manifest_path).expect("read v1 content manifest");
    let mut rows = 0_usize;
    for line in manifest.lines().skip(1).filter(|line| !line.is_empty()) {
        let fields = line.split('\t').collect::<Vec<_>>();
        assert_eq!(fields.len(), 4, "malformed manifest row: {line}");
        let output = Command::new("git")
            .args(["show", &format!("{BASE_COMMIT}:{}", fields[0])])
            .current_dir(&root)
            .output()
            .expect("run git show for manifest row");
        assert!(output.status.success(), "cannot recover {}", fields[0]);
        assert_eq!(
            output.stdout.len(),
            fields[1].parse::<usize>().expect("size")
        );
        assert_eq!(
            sha256_bytes(&output.stdout),
            fields[2],
            "hash for {}",
            fields[0]
        );
        match fields[3] {
            "remove" => assert!(
                !root.join(fields[0]).exists(),
                "retired file remains: {}",
                fields[0]
            ),
            "preserve-or-revise" => assert!(
                root.join(fields[0]).is_file(),
                "preserved or revised file is absent: {}",
                fields[0]
            ),
            action => panic!("unknown manifest action '{action}' for {}", fields[0]),
        }
        rows += 1;
    }
    assert_eq!(rows, 51, "unexpected preservation inventory size");
}

#[test]
fn transition_preflight_separates_validation_from_release() {
    let root = repository_root();
    let script = root.join("tools/release/check_assurance_release_transition.sh");
    let fixture = transition_fixture("assure03-release-preflight");
    for relative in [
        "tools/release/check_assurance_release_transition.sh",
        "tools/release/run_release_candidate_gates.sh",
    ] {
        write_file(
            &fixture.path.join(relative),
            &fs::read(root.join(relative)).expect("read release script"),
        );
    }
    fs::write(
        fixture.path.join("assurance/V1_PUBLICATION_TRANSITION"),
        "blocked\n",
    )
    .expect("write transition marker");
    assert!(run_preflight(&script, "validate", &fixture.path));
    assert!(!run_preflight(&script, "release", &fixture.path));
    let rejected_release_dir = fixture.path.join("must-not-be-created");
    let aggregate = Command::new("bash")
        .arg(
            fixture
                .path
                .join("tools/release/run_release_candidate_gates.sh"),
        )
        .args(["--mode", "release", "--release-dir"])
        .arg(&rejected_release_dir)
        .output()
        .expect("run real aggregate release preflight");
    assert!(!aggregate.status.success());
    assert!(String::from_utf8_lossy(&aggregate.stderr).contains("blocked by"));
    assert!(!rejected_release_dir.exists());
    fs::remove_file(fixture.path.join("assurance/V1_PUBLICATION_TRANSITION"))
        .expect("remove transition marker");
    assert!(run_preflight(&script, "release", &fixture.path));

    let catalog_path = fixture.path.join("assurance/catalog.yaml");
    let clean_catalog = fs::read_to_string(&catalog_path).expect("read clean catalog");
    fs::write(
        &catalog_path,
        format!("{clean_catalog}dossiers:\n  - dossier_id: duplicate-key-evasion\n"),
    )
    .expect("write duplicate-key catalog");
    assert!(!run_preflight(&script, "release", &fixture.path));
    assert_aggregate_rejected_without_release_dir(
        &fixture.path,
        &fixture.path.join("must-not-admit-duplicate-catalog-key"),
        "exact typed v1-retired zero-report catalog bytes",
    );
    fs::write(&catalog_path, &clean_catalog).expect("restore clean catalog");

    replace_in(
        &fixture.path.join("assurance/catalog.yaml"),
        "dossiers: []",
        "dossiers:\n  - dossier_id: retired-candidate",
    );
    assert!(!run_preflight(&script, "release", &fixture.path));
    replace_in(
        &fixture.path.join("assurance/catalog.yaml"),
        "dossiers:\n  - dossier_id: retired-candidate",
        "dossiers: []",
    );
    write_file(
        &fixture.path.join("assurance/dossiers/retired.yaml"),
        b"retired\n",
    );
    assert!(!run_preflight(&script, "release", &fixture.path));
}

#[cfg(unix)]
#[test]
fn transition_preflight_rejects_symlink_evasions_before_release_directory() {
    use std::os::unix::fs::symlink;
    use std::os::unix::net::UnixListener;

    let root = repository_root();
    let script = root.join("tools/release/check_assurance_release_transition.sh");
    let fixture = transition_fixture("p");
    for relative in [
        "tools/release/check_assurance_release_transition.sh",
        "tools/release/run_release_candidate_gates.sh",
    ] {
        write_file(
            &fixture.path.join(relative),
            &fs::read(root.join(relative)).expect("read release script"),
        );
    }

    let marker = fixture.path.join("assurance/V1_PUBLICATION_TRANSITION");
    symlink("missing-transition-target", &marker).expect("create dangling marker symlink");
    assert!(!run_preflight(&script, "release", &fixture.path));
    assert_aggregate_rejected_without_release_dir(
        &fixture.path,
        &fixture.path.join("must-not-follow-marker-symlink"),
        "blocked by",
    );
    fs::remove_file(&marker).expect("remove dangling marker symlink");

    let retired = fixture.path.join("assurance/dossiers/retired-link");
    fs::create_dir_all(retired.parent().expect("retired route parent"))
        .expect("create retired route parent");
    symlink("missing-retired-target", &retired).expect("create retired route symlink");
    assert!(!run_preflight(&script, "release", &fixture.path));
    assert_aggregate_rejected_without_release_dir(
        &fixture.path,
        &fixture.path.join("must-not-follow-retired-symlink"),
        "retired v1 route",
    );
    fs::remove_file(&retired).expect("remove retired route symlink");

    let retired_socket = fixture.path.join("assurance/dossiers");
    fs::remove_dir(&retired_socket).expect("remove empty retired route directory");
    let listener = UnixListener::bind(&retired_socket).expect("bind retired-root socket");
    assert!(!run_preflight(&script, "release", &fixture.path));
    assert_aggregate_rejected_without_release_dir(
        &fixture.path,
        &fixture.path.join("must-not-admit-retired-root-socket"),
        "retired v1 route",
    );
    drop(listener);
    fs::remove_file(&retired_socket).expect("remove retired-root socket");

    let external = Scratch::new("assure03-external-catalog");
    let external_catalog = external.path.join("catalog.yaml");
    fs::copy(
        fixture.path.join("assurance/catalog.yaml"),
        &external_catalog,
    )
    .expect("copy external catalog");
    fs::remove_file(fixture.path.join("assurance/catalog.yaml")).expect("remove regular catalog");
    symlink(
        &external_catalog,
        fixture.path.join("assurance/catalog.yaml"),
    )
    .expect("create catalog symlink");
    assert!(!run_preflight(&script, "release", &fixture.path));
    assert_aggregate_rejected_without_release_dir(
        &fixture.path,
        &fixture.path.join("must-not-follow-catalog-symlink"),
        "non-symlink assurance/catalog.yaml",
    );
}

#[test]
fn workflow_uses_validation_only_for_ordinary_events() {
    let root = repository_root();
    let workflow = fs::read_to_string(root.join(".github/workflows/release-gates.yml"))
        .expect("read release workflow");
    assert!(workflow.contains("workspace-validation:"));
    assert!(workflow.contains("--mode validate"));
    assert!(workflow.contains("openwepp-validation-evidence-${{ github.run_id }}"));
    assert!(workflow.contains("release-assembly:"));
    assert!(workflow.contains("inputs.assemble_release"));
    assert!(workflow.contains("--mode release"));
    assert!(workflow.contains("openwepp-release-candidate-${{ github.run_id }}"));
    assert_eq!(
        workflow
            .matches("openwepp-release-candidate-${{ github.run_id }}")
            .count(),
        1
    );

    let release_start = workflow.find("  release-assembly:").expect("release job");
    let release_end = workflow[release_start..]
        .find("\n  stability-cohort:")
        .map(|offset| release_start + offset)
        .expect("release job end");
    let release_job = &workflow[release_start..release_end];
    assert!(release_job.contains("inputs.run_stability"));
    assert!(release_job.contains("needs.stability-cohort.result == 'success'"));
    assert!(release_job.contains("- stability-cohort"));
    let preflight = release_job
        .find("Release assurance transition preflight")
        .expect("release preflight step");
    let release_directory = release_job
        .find("release_dir=\"${RUNNER_TEMP}/openwepp_release_")
        .expect("candidate directory creation");
    assert!(preflight < release_directory);
    let candidate_upload = release_job
        .find("Upload explicit release candidate")
        .expect("candidate upload");
    let failed_upload = release_job
        .find("Upload failed release evidence")
        .expect("failure upload");
    assert!(candidate_upload < failed_upload);
    assert!(release_job[candidate_upload..failed_upload].contains("if: ${{ success() }}"));
    assert!(release_job[failed_upload..].contains("if: ${{ failure() }}"));
    assert!(
        release_job[failed_upload..]
            .contains("openwepp-release-failure-evidence-${{ github.run_id }}")
    );

    let script = fs::read_to_string(root.join("tools/release/run_release_candidate_gates.sh"))
        .expect("read aggregate gate script");
    let validation_exit = script
        .find("validation gate automation passed")
        .expect("validation exit");
    let binary_assembly = script
        .find("building release binaries")
        .expect("binary assembly");
    assert!(validation_exit < binary_assembly);
    assert!(script.contains("if [[ \"${MODE}\" == \"release\" ]]"));
}

#[test]
fn public_reader_path_preserves_snow_science_without_v1_routes() {
    let root = repository_root();
    for retired in [
        "usersum/assurance/application-context-worksheet.md",
        "usersum/assurance/dossiers/snow-snotel-swe-depth-density.md",
        "usersum/assurance/methods/snow-snotel-evaluation-v1.md",
    ] {
        assert!(
            !root.join(retired).exists(),
            "retired public route remains: {retired}"
        );
    }
    let catalog = fs::read_to_string(root.join("usersum/assurance/README.md"))
        .expect("read public zero-report catalog");
    assert!(catalog.contains("No scientific model-evaluation report has completed"));
    assert!(catalog.contains("does not mean that openWEPP processes lack"));
    assert!(!catalog.contains("CANDIDATE"));
    assert!(!catalog.contains("INSUFFICIENT_EVIDENCE"));

    let snow = fs::read_to_string(root.join("usersum/snow-frost-modeling-and-validation.md"))
        .expect("read snow/frost narrative");
    for retained_science in [
        "Multilayer snowpack",
        "SNOTEL",
        "Precipitation phase",
        "water equivalent",
        "soil-temperature",
    ] {
        assert!(
            snow.contains(retained_science),
            "missing snow science: {retained_science}"
        );
    }
    assert!(!snow.contains("assurance/dossiers/snow-snotel-swe-depth-density.md"));
    assert!(!snow.contains("assurance/methods/snow-snotel-evaluation-v1.md"));
    assert!(!snow.contains("assurance/application-context-worksheet.md"));
}

fn output_options(root: &Path) -> BuildOptions {
    BuildOptions {
        output_root: Some(root.to_path_buf()),
        ..BuildOptions::default()
    }
}

fn run_preflight(script: &Path, mode: &str, root: &Path) -> bool {
    Command::new("bash")
        .arg(script)
        .args(["--mode", mode, "--root"])
        .arg(root)
        .status()
        .expect("run release preflight")
        .success()
}

fn assert_aggregate_rejected_without_release_dir(
    fixture_root: &Path,
    release_dir: &Path,
    expected_error: &str,
) {
    let output = Command::new("bash")
        .arg(fixture_root.join("tools/release/run_release_candidate_gates.sh"))
        .args(["--mode", "release", "--release-dir"])
        .arg(release_dir)
        .output()
        .expect("run aggregate release preflight");
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains(expected_error),
        "missing aggregate rejection '{expected_error}': {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!release_dir.exists());
}

fn transition_fixture(label: &str) -> Scratch {
    let source = repository_root();
    let fixture = Scratch::new(label);
    for relative in [
        "assurance/catalog.yaml",
        "assurance/templates/catalog.md",
        "assurance/generated/wepppy-usersum.yaml",
        "usersum/assurance/README.md",
    ] {
        let bytes = fs::read(source.join(relative)).expect("read transition fixture source");
        write_file(&fixture.path.join(relative), &bytes);
    }
    fixture
}

fn write_file(path: &Path, bytes: &[u8]) {
    fs::create_dir_all(path.parent().expect("fixture file parent"))
        .expect("create fixture directory");
    fs::write(path, bytes).expect("write fixture file");
}

fn replace_in(path: &Path, old: &str, new: &str) {
    let text = fs::read_to_string(path).expect("read replacement target");
    assert!(text.contains(old), "replacement source missing: {old}");
    fs::write(path, text.replacen(old, new, 1)).expect("write replacement target");
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
            fs::remove_dir_all(&path).expect("remove stale scratch directory");
        }
        fs::create_dir_all(&path).expect("create scratch directory");
        Self { path }
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        if self.path.exists() {
            fs::remove_dir_all(&self.path).expect("remove scratch directory");
        }
    }
}
