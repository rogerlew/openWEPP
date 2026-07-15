use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use openwepp_assurance::{Assurance, AssuranceError, BuildOptions, Selection};
use sha2::{Digest, Sha256};

const DOSSIER_ID: &str = "snow-snotel-swe-depth-density";
const AUTHORING_PATH: &str = "assurance/dossiers/snow-snotel-swe-depth-density/authoring.yaml";
const DOSSIER_PATH: &str = "assurance/dossiers/snow-snotel-swe-depth-density/dossier.yaml";
const REVIEW_PATH: &str = "assurance/dossiers/snow-snotel-swe-depth-density/review.yaml";
const NARRATIVE_PATH: &str = "usersum/snow-frost-modeling-and-validation.md";

#[test]
fn real_public_vertical_slice_is_current_and_navigable() {
    let root = repository_root();
    let assurance = Assurance::open(&root).expect("load real assurance catalog");
    assurance
        .validate(&Selection::All)
        .expect("validate real assurance catalog");
    assurance
        .check(&Selection::All)
        .expect("committed generated output must be current");
    assert_eq!(
        openwepp_assurance::cli::run(["openwepp-assurance", "validate", "--all"])
            .expect("run public CLI validate"),
        "validation: PASS\n"
    );

    let plan = assurance
        .plan(&Selection::Dossier(DOSSIER_ID.to_owned()))
        .expect("plan pilot dossier");
    assert_eq!(plan.outputs.len(), 5);
    assert_eq!(
        plan.review_implications.get(DOSSIER_ID).map(String::as_str),
        Some("scientific=pending,publication=pending")
    );
    for input in [
        "assurance/schemas/authoring.schema.json",
        AUTHORING_PATH,
        NARRATIVE_PATH,
    ] {
        assert!(
            plan.inputs.iter().any(|(path, _)| path == Path::new(input)),
            "plan omits {input}"
        );
    }
    for node in [
        "output:index",
        "output:worksheet",
        "output:export",
        "output:method:snow-snotel-swe-depth-density",
        "output:dossier:snow-snotel-swe-depth-density",
        "narrative:snow-snotel-swe-depth-density",
        "authoring:snow-snotel-swe-depth-density",
        "tool:identity",
        "schema:identity",
    ] {
        assert_eq!(
            plan.node_fingerprints.get(node).map(String::len),
            Some(64),
            "missing node fingerprint {node}"
        );
    }
    assert_eq!(plan.scientific_roots[DOSSIER_ID].len(), 64);
    assert_eq!(plan.source_roots[DOSSIER_ID].len(), 64);
    assert_eq!(plan.review_payloads.len(), 2);

    assert_generated_outputs(&root, &plan.outputs);
    assert_export_contract(&root);
    let dossier = fs::read_to_string(
        root.join("usersum/assurance/dossiers")
            .join(format!("{DOSSIER_ID}.md")),
    )
    .expect("read public dossier");
    assert!(dossier.contains("Aggregate software verification: `BLOCKED`"));
    assert!(dossier.contains("`default-phase-selector`"));
    assert!(dossier.contains("`numerical-solution-verification`"));
}

#[test]
fn targeted_and_all_builds_are_byte_deterministic() {
    let root = repository_root();
    let assurance = Assurance::open(&root).expect("load assurance catalog");
    let targeted_selection = Selection::Dossier(DOSSIER_ID.to_owned());
    let targeted_plan = assurance
        .plan(&targeted_selection)
        .expect("plan targeted build");
    let all_plan = assurance.plan(&Selection::All).expect("plan all build");
    assert_eq!(targeted_plan.inputs, all_plan.inputs);
    for required in [AUTHORING_PATH, NARRATIVE_PATH] {
        assert!(
            targeted_plan
                .inputs
                .iter()
                .any(|(path, _)| path == Path::new(required)),
            "targeted plan omits {required}"
        );
    }
    let first = Scratch::new("assurance-deterministic-a");
    let second = Scratch::new("assurance-deterministic-b");
    let targeted = assurance
        .build(&targeted_selection, &output_options(&first.path))
        .expect("targeted build");
    let all = assurance
        .build(&Selection::All, &output_options(&second.path))
        .expect("all build");
    assert_eq!(targeted.outputs, all.outputs);
    for path in targeted.outputs.keys() {
        assert_eq!(
            fs::read(first.path.join(path)).expect("read first output"),
            fs::read(second.path.join(path)).expect("read second output"),
            "non-deterministic output {}",
            path.display()
        );
    }

    let changed_template = copy_real_fixture("assurance-unrelated-template-fingerprint");
    let before = Assurance::open(&changed_template.path)
        .expect("load baseline template fixture")
        .plan(&Selection::All)
        .expect("plan baseline template fixture");
    append_to(
        &changed_template
            .path
            .join("assurance/templates/application-context-worksheet.md"),
        "\nBound template-change probe.\n",
    );
    let after = Assurance::open(&changed_template.path)
        .expect("load changed template fixture")
        .plan(&Selection::All)
        .expect("plan changed template fixture");
    assert_ne!(
        before.source_roots[DOSSIER_ID],
        after.source_roots[DOSSIER_ID]
    );
    assert_ne!(
        before.node_fingerprints[&format!("output:dossier:{DOSSIER_ID}")],
        after.node_fingerprints[&format!("output:dossier:{DOSSIER_ID}")]
    );
}

#[test]
fn drift_and_dual_review_locks_fail_closed() {
    let fixture = copy_real_fixture("assurance-review-lock");
    let assurance = Assurance::open(&fixture.path).expect("load copied fixture");
    assurance
        .build(&Selection::All, &BuildOptions::default())
        .expect("build copied fixture");
    fs::write(fixture.path.join("usersum/assurance/README.md"), "stale\n")
        .expect("make generated output stale");
    assert!(matches!(
        assurance.check(&Selection::All),
        Err(AssuranceError::Drift(_))
    ));

    set_lifecycle(&fixture.path, "published");
    approve_reviews(&fixture.path);
    Assurance::open(&fixture.path)
        .expect("load locked fixture")
        .validate(&Selection::All)
        .expect("matching scientific and publication locks");

    replace_in(
        &fixture.path.join(AUTHORING_PATH),
        "inventory the retained five-climate SNOTEL evidence",
        "review the retained five-climate SNOTEL evidence",
    );
    let invalidated = Assurance::open(&fixture.path).expect("load changed source-root fixture");
    assert!(matches!(
        invalidated.validate(&Selection::All),
        Err(AssuranceError::ReviewRequired(_))
    ));

    let payload = published_fixture("assurance-review-payload-change");
    replace_in(
        &payload.path.join(REVIEW_PATH),
        "Scientific disposition accepted.",
        "Scientific disposition amended.",
    );
    assert!(matches!(
        Assurance::open(&payload.path),
        Err(AssuranceError::Invalid(_))
    ));

    let moved_review = published_fixture("assurance-review-path-change");
    let moved_relative = "assurance/dossiers/snow-snotel-swe-depth-density/moved-review.yaml";
    fs::rename(
        moved_review.path.join(REVIEW_PATH),
        moved_review.path.join(moved_relative),
    )
    .expect("move approved review record");
    replace_in(
        &moved_review.path.join("assurance/catalog.yaml"),
        &format!("review: {REVIEW_PATH}"),
        &format!("review: {moved_relative}"),
    );
    let changed_path = Assurance::open(&moved_review.path).expect("load moved review fixture");
    assert!(matches!(
        changed_path.validate(&Selection::All),
        Err(AssuranceError::ReviewRequired(_))
    ));
}

#[test]
fn approval_payloads_bind_the_complete_ordered_history() {
    let edited = renewed_review_fixture("assurance-review-history-edit");
    replace_in(
        &edited.path.join(REVIEW_PATH),
        "Scientific disposition accepted.",
        "Scientific disposition historically amended.",
    );
    assert!(matches!(
        Assurance::open(&edited.path),
        Err(AssuranceError::Invalid(_))
    ));

    let removed = renewed_review_fixture("assurance-review-history-remove");
    mutate_review_approvals(&removed.path, |approvals| {
        approvals.remove(0);
    });
    assert!(matches!(
        Assurance::open(&removed.path),
        Err(AssuranceError::Invalid(_))
    ));

    let reordered = renewed_review_fixture("assurance-review-history-reorder");
    mutate_review_approvals(&reordered.path, |approvals| {
        approvals.swap(0, 1);
    });
    assert!(matches!(
        Assurance::open(&reordered.path),
        Err(AssuranceError::Invalid(_))
    ));
}

#[test]
fn self_review_and_unresolved_blockers_are_rejected() {
    let pending_authoring = copy_real_fixture("assurance-pending-authoring");
    set_lifecycle(&pending_authoring.path, "published");
    set_authoring_pending(&pending_authoring.path);
    assert!(matches!(
        Assurance::open(&pending_authoring.path)
            .expect("load pending authoring fixture")
            .validate(&Selection::All),
        Err(AssuranceError::ReviewRequired(_))
    ));

    let self_review = published_fixture("assurance-self-review");
    replace_in(
        &self_review.path.join(REVIEW_PATH),
        "Independent Scientific Reviewer",
        "OpenAI Codex",
    );
    assert!(matches!(
        Assurance::open(&self_review.path),
        Err(AssuranceError::Invalid(_))
    ));

    let unresolved = published_fixture("assurance-unresolved-review-finding");
    replace_in(
        &unresolved.path.join(REVIEW_PATH),
        "    findings: []",
        "    findings:\n      - finding_id: unresolved-scientific-blocker\n        severity: high\n        summary: A closure-blocking scientific issue remains.\n        disposition: follow_up\n        rationale: The issue requires evidence before approval.\n        closure_blocking: true\n        resolved: false",
    );
    assert!(matches!(
        Assurance::open(&unresolved.path),
        Err(AssuranceError::Invalid(_))
    ));
}

#[test]
fn historical_terminal_lifecycles_require_and_accept_current_locks() {
    for lifecycle in ["superseded", "withdrawn"] {
        let fixture = copy_real_fixture(&format!("assurance-{lifecycle}-lock"));
        set_lifecycle(&fixture.path, lifecycle);
        let unlocked = Assurance::open(&fixture.path).expect("load unlocked historical fixture");
        assert!(matches!(
            unlocked.validate(&Selection::All),
            Err(AssuranceError::ReviewRequired(_))
        ));
        approve_reviews(&fixture.path);
        Assurance::open(&fixture.path)
            .expect("load locked historical fixture")
            .validate(&Selection::All)
            .expect("accept matching historical locks");
    }
}

#[test]
fn export_status_maps_all_lifecycles_for_every_dossier_document() {
    for (lifecycle, expected_status) in [
        ("draft", "draft"),
        ("candidate", "active"),
        ("published", "active"),
        ("superseded", "deprecated"),
        ("withdrawn", "deprecated"),
    ] {
        let fixture = copy_real_fixture(&format!("assurance-export-{lifecycle}"));
        if lifecycle != "candidate" {
            set_lifecycle(&fixture.path, lifecycle);
        }
        if matches!(lifecycle, "published" | "superseded" | "withdrawn") {
            approve_reviews(&fixture.path);
        }
        let assurance = Assurance::open(&fixture.path).expect("load lifecycle fixture");
        assurance
            .build(&Selection::All, &BuildOptions::default())
            .expect("build lifecycle export");
        let export: serde_yaml::Value = serde_yaml::from_str(
            &fs::read_to_string(fixture.path.join("assurance/generated/wepppy-usersum.yaml"))
                .expect("read lifecycle export"),
        )
        .expect("parse lifecycle export");
        let records = export["documents"].as_sequence().expect("export records");
        let scoped = records
            .iter()
            .filter(|record| record["assurance_lifecycle"].as_str() == Some(lifecycle))
            .collect::<Vec<_>>();
        assert_eq!(scoped.len(), 3, "wrong scoped record count for {lifecycle}");
        assert!(
            scoped
                .iter()
                .all(|record| { record["status"].as_str() == Some(expected_status) })
        );
    }
}

#[test]
fn snapshots_bind_narratives_and_reject_drafts() {
    let fixture = copy_real_fixture("assurance-snapshot");
    let options = snapshot_options(&fixture.path, "release-20260714");
    let assurance = Assurance::open(&fixture.path).expect("load copied fixture");
    let created = assurance
        .build(&Selection::All, &options)
        .expect("create snapshot");
    assert!(!created.snapshot_confirmed_existing);
    let snapshot = read_snapshot(&created);
    assert_eq!(
        snapshot["tool_source_sha256"].as_str().map(str::len),
        Some(64)
    );
    assert_eq!(snapshot["catalog_sha256"].as_str().map(str::len), Some(64));
    let public_files = snapshot["public_files"]
        .as_array()
        .expect("snapshot public files");
    assert!(
        public_files
            .iter()
            .any(|entry| { entry["path"].as_str() == Some(NARRATIVE_PATH) })
    );
    assert!(snapshot["dossiers"][0]["scientific_root_sha256"].is_string());
    assert!(snapshot["dossiers"][0]["publication_root_sha256"].is_string());
    assert!(
        assurance
            .build(&Selection::All, &options)
            .expect("confirm snapshot")
            .snapshot_confirmed_existing
    );
    let unsafe_options = snapshot_options(&fixture.path, "../escape");
    assert!(matches!(
        assurance.build(&Selection::All, &unsafe_options),
        Err(AssuranceError::Invalid(_))
    ));
    assert!(matches!(
        assurance.build(&Selection::Dossier(DOSSIER_ID.to_owned()), &options),
        Err(AssuranceError::Usage(_))
    ));

    append_to(
        &fixture.path.join(NARRATIVE_PATH),
        "\nMaterially changed narrative.\n",
    );
    refresh_authoring(&fixture.path);
    let changed = Assurance::open(&fixture.path).expect("load changed narrative fixture");
    assert!(matches!(
        changed.build(&Selection::All, &options),
        Err(AssuranceError::SnapshotConflict(_))
    ));

    let draft = copy_real_fixture("assurance-draft-snapshot");
    set_lifecycle(&draft.path, "draft");
    let draft_assurance = Assurance::open(&draft.path).expect("load draft fixture");
    assert!(matches!(
        draft_assurance.build(
            &Selection::All,
            &snapshot_options(&draft.path, "draft-release")
        ),
        Err(AssuranceError::Invalid(_))
    ));
}

#[cfg(unix)]
#[test]
fn snapshots_reject_symlinked_file_roots() {
    use std::os::unix::fs::symlink;

    let fixture = copy_real_fixture("assurance-snapshot-files-symlink");
    let options = snapshot_options(&fixture.path, "symlink-files");
    let assurance = Assurance::open(&fixture.path).expect("load copied fixture");
    assurance
        .build(&Selection::All, &options)
        .expect("create baseline snapshot");
    let files = fixture.path.join("snapshots/symlink-files/files");
    let relocated = fixture.path.join("relocated-snapshot-files");
    fs::rename(&files, &relocated).expect("relocate snapshot files");
    symlink(&relocated, &files).expect("symlink snapshot files root");
    assert!(matches!(
        assurance.build(&Selection::All, &options),
        Err(AssuranceError::SnapshotConflict(_))
    ));
    fs::remove_file(&files).expect("remove files symlink");
    fs::rename(&relocated, &files).expect("restore snapshot files");

    let manifest = fixture.path.join("snapshots/symlink-files/manifest.json");
    let relocated_manifest = fixture.path.join("relocated-snapshot-manifest.json");
    fs::rename(&manifest, &relocated_manifest).expect("relocate snapshot manifest");
    symlink(&relocated_manifest, &manifest).expect("symlink snapshot manifest");
    assert!(matches!(
        assurance.build(&Selection::All, &options),
        Err(AssuranceError::SnapshotConflict(_))
    ));
    fs::remove_file(&manifest).expect("remove manifest symlink");
    fs::rename(&relocated_manifest, &manifest).expect("restore snapshot manifest");

    let public_file = files.join(NARRATIVE_PATH);
    let relocated_file = fixture.path.join("relocated-snapshot-narrative.md");
    fs::rename(&public_file, &relocated_file).expect("relocate snapshot descendant");
    symlink(&relocated_file, &public_file).expect("symlink snapshot descendant");
    assert!(matches!(
        assurance.build(&Selection::All, &options),
        Err(AssuranceError::SnapshotConflict(_))
    ));

    let real_root = fixture.path.join("real-snapshot-root");
    fs::create_dir(&real_root).expect("create real snapshot root");
    let linked_root = fixture.path.join("linked-snapshot-root");
    symlink(&real_root, &linked_root).expect("symlink snapshot root");
    let root_options = BuildOptions {
        output_root: Some(fixture.path.clone()),
        snapshot: Some("root-link".to_owned()),
        snapshot_root: Some(linked_root),
    };
    assert!(matches!(
        assurance.build(&Selection::All, &root_options),
        Err(AssuranceError::Invalid(_))
    ));

    let outside = Scratch::new("assurance-snapshot-outside-root");
    let ancestor_link = fixture.path.join("snapshot-ancestor-link");
    symlink(&outside.path, &ancestor_link).expect("symlink snapshot ancestor");
    let missing_descendant = outside.path.join("must-not-be-created");
    let ancestor_options = BuildOptions {
        output_root: Some(fixture.path.clone()),
        snapshot: Some("ancestor-link".to_owned()),
        snapshot_root: Some(ancestor_link.join("must-not-be-created/nested")),
    };
    assert!(matches!(
        assurance.build(&Selection::All, &ancestor_options),
        Err(AssuranceError::Invalid(_))
    ));
    assert!(
        !missing_descendant.exists(),
        "snapshot validation wrote through a symlink ancestor"
    );

    let collision_root = fixture.path.join("collision-snapshots");
    fs::create_dir(&collision_root).expect("create collision snapshot root");
    let collision = collision_root.join(format!(".collision-safe.tmp-{}-0", std::process::id()));
    fs::create_dir(&collision).expect("create unknown colliding staging directory");
    fs::write(collision.join("sentinel"), "owned elsewhere\n").expect("write collision sentinel");
    let collision_options = BuildOptions {
        output_root: Some(fixture.path.clone()),
        snapshot: Some("collision-safe".to_owned()),
        snapshot_root: Some(collision_root),
    };
    assurance
        .build(&Selection::All, &collision_options)
        .expect("retry after staging collision");
    assert_eq!(
        fs::read_to_string(collision.join("sentinel")).expect("read preserved collision sentinel"),
        "owned elsewhere\n"
    );
}

#[cfg(unix)]
#[test]
fn snapshots_reject_symlinked_id_target_without_writing_through() {
    use std::os::unix::fs::symlink;

    let fixture = copy_real_fixture("assurance-snapshot-id-target-symlink");
    let assurance = Assurance::open(&fixture.path).expect("load ID-target symlink fixture");
    let id_root = fixture.path.join("id-target-snapshots");
    fs::create_dir(&id_root).expect("create ID-target snapshot root");
    let relocated_id = fixture.path.join("relocated-snapshot-id");
    fs::create_dir(&relocated_id).expect("create relocated snapshot ID target");
    symlink(&relocated_id, id_root.join("symlink-id")).expect("symlink snapshot ID target");
    let options = BuildOptions {
        output_root: Some(fixture.path.clone()),
        snapshot: Some("symlink-id".to_owned()),
        snapshot_root: Some(id_root),
    };
    assert!(matches!(
        assurance.build(&Selection::All, &options),
        Err(AssuranceError::Invalid(_))
    ));
    assert_eq!(
        fs::read_dir(&relocated_id)
            .expect("read relocated snapshot ID target")
            .count(),
        0,
        "snapshot ID symlink target received an outside write"
    );
}

#[test]
fn schema_documents_are_size_limited_and_compiler_bound() {
    let mutated = copy_real_fixture("assurance-schema-mutation");
    append_to(
        &mutated.path.join("assurance/schemas/dossier.schema.json"),
        "\n",
    );
    assert!(matches!(
        Assurance::open(&mutated.path),
        Err(AssuranceError::Invalid(_))
    ));

    let changed_id = copy_real_fixture("assurance-schema-id-mutation");
    replace_in(
        &changed_id
            .path
            .join("assurance/schemas/dossier.schema.json"),
        "https://openwepp.org/assurance/schema/dossier-v1",
        "https://openwepp.org/assurance/schema/dossier-v2",
    );
    assert!(matches!(
        Assurance::open(&changed_id.path),
        Err(AssuranceError::Invalid(_))
    ));

    let changed_version = copy_real_fixture("assurance-schema-version-mutation");
    replace_in(
        &changed_version
            .path
            .join("assurance/schemas/dossier.schema.json"),
        r#""schema_version": {"const": 1}"#,
        r#""schema_version": {"const": 2}"#,
    );
    assert!(matches!(
        Assurance::open(&changed_version.path),
        Err(AssuranceError::Invalid(_))
    ));

    let oversized = copy_real_fixture("assurance-schema-oversized");
    fs::write(
        oversized.path.join("assurance/schemas/dossier.schema.json"),
        vec![b' '; 2 * 1024 * 1024 + 1],
    )
    .expect("write oversized schema");
    assert!(matches!(
        Assurance::open(&oversized.path),
        Err(AssuranceError::Invalid(_))
    ));
}

#[test]
fn public_links_and_secret_markers_fail_closed() {
    let link = copy_real_fixture("assurance-public-link-injection");
    append_to(
        &link.path.join(NARRATIVE_PATH),
        "\n[private operator page](/admin/private)\n",
    );
    refresh_authoring(&link.path);
    let assurance = Assurance::open(&link.path).expect("load linked fixture");
    assert!(matches!(
        assurance.validate(&Selection::All),
        Err(AssuranceError::Invalid(_))
    ));
    assert!(matches!(
        assurance.plan(&Selection::All),
        Err(AssuranceError::Invalid(_))
    ));
    assert!(matches!(
        assurance.build(&Selection::All, &BuildOptions::default()),
        Err(AssuranceError::Invalid(_))
    ));

    let secret = copy_real_fixture("assurance-public-secret-injection");
    replace_in(
        &secret.path.join(DOSSIER_PATH),
        "title: SNOTEL Snow Evidence Across Five Climates",
        "title: SNOTEL Snow Evidence ghp_exampletoken",
    );
    refresh_authoring(&secret.path);
    assert!(matches!(
        Assurance::open(&secret.path),
        Err(AssuranceError::Invalid(_))
    ));

    let raw_template_token = copy_real_fixture("assurance-raw-template-token-injection");
    append_to(
        &raw_template_token
            .path
            .join("assurance/dossiers/snow-snotel-swe-depth-density/interpretation.md"),
        "\n{{NARRATIVE_LINK}}\n",
    );
    refresh_authoring(&raw_template_token.path);
    let assurance =
        Assurance::open(&raw_template_token.path).expect("load raw template-token fixture");
    assert!(matches!(
        assurance.validate(&Selection::All),
        Err(AssuranceError::Invalid(_))
    ));

    for (label, injected) in [
        ("absolute-posix", "title: SNOTEL Snow Evidence /etc/shadow"),
        (
            "absolute-windows",
            r"title: SNOTEL Snow Evidence C:\Users\operator\secret.txt",
        ),
        ("markdown-scalar", "title: SNOTEL Snow Evidence [injected]"),
        ("template-token", "title: '{{NARRATIVE_LINK}}'"),
        (
            "generic-secret",
            "title: SNOTEL Snow Evidence sk-1234567890abcdefghijklmnopqrstuv",
        ),
    ] {
        let fixture = copy_real_fixture(&format!("assurance-public-{label}"));
        replace_in(
            &fixture.path.join(DOSSIER_PATH),
            "title: SNOTEL Snow Evidence Across Five Climates",
            injected,
        );
        refresh_authoring(&fixture.path);
        assert!(matches!(
            Assurance::open(&fixture.path),
            Err(AssuranceError::Invalid(_))
        ));
    }
}

#[test]
fn participant_fields_are_safe_in_nonapproved_review_states() {
    for state in ["pending", "rejected"] {
        let fixture = copy_real_fixture(&format!("assurance-review-participant-{state}"));
        let review_path = fixture.path.join(REVIEW_PATH);
        let mut review: serde_yaml::Value = serde_yaml::from_str(
            &fs::read_to_string(&review_path).expect("read participant review"),
        )
        .expect("parse participant review");
        let entry = &mut review["approvals"]
            .as_sequence_mut()
            .expect("review approvals")[0];
        entry["state"] = serde_yaml::Value::String(state.to_owned());
        entry["reviewers"] = serde_yaml::from_str(
            r#"- name: Pending Reviewer
  role: "[injected](outside.md)"
  expertise: Evidence review.
  independent_of_authors: false
  independence_basis: Independence not yet adjudicated.
"#,
        )
        .expect("parse unsafe participant");
        fs::write(
            &review_path,
            serde_yaml::to_string(&review).expect("serialize participant review"),
        )
        .expect("write participant review");
        assert!(matches!(
            Assurance::open(&fixture.path),
            Err(AssuranceError::Invalid(_))
        ));
    }

    let authoring = copy_real_fixture("assurance-authoring-pending-participant");
    let path = authoring.path.join(AUTHORING_PATH);
    let mut record: serde_yaml::Value = serde_yaml::from_str(
        &fs::read_to_string(&path).expect("read authoring participant record"),
    )
    .expect("parse authoring participant record");
    record["review"]["state"] = serde_yaml::Value::String("pending".to_owned());
    record["review"]["reviewer"]["role"] =
        serde_yaml::Value::String("`injected participant`".to_owned());
    fs::write(
        &path,
        serde_yaml::to_string(&record).expect("serialize authoring participant record"),
    )
    .expect("write authoring participant record");
    assert!(matches!(
        Assurance::open(&authoring.path),
        Err(AssuranceError::Invalid(_))
    ));
}

#[test]
fn malformed_sources_collisions_and_path_escapes_are_rejected() {
    let unknown = copy_real_fixture("assurance-unknown-field");
    append_to(
        &unknown.path.join("assurance/catalog.yaml"),
        "\ncommand: echo forbidden\n",
    );
    assert!(matches!(
        Assurance::open(&unknown.path),
        Err(AssuranceError::Parse { .. })
    ));

    let collision = copy_real_fixture("assurance-output-collision");
    replace_in(
        &collision.path.join("assurance/catalog.yaml"),
        "method: usersum/assurance/methods/snow-snotel-evaluation-v1.md",
        "method: usersum/assurance/dossiers/snow-snotel-swe-depth-density.md",
    );
    assert!(matches!(
        Assurance::open(&collision.path),
        Err(AssuranceError::Invalid(_))
    ));

    let traversal = copy_real_fixture("assurance-traversal");
    replace_in(
        &traversal.path.join("assurance/catalog.yaml"),
        "source: assurance/dossiers/snow-snotel-swe-depth-density/dossier.yaml",
        "source: ../outside.yaml",
    );
    assert!(matches!(
        Assurance::open(&traversal.path),
        Err(AssuranceError::Invalid(_))
    ));

    for (label, path) in [
        ("space", "assurance/dossiers/unsafe source.yaml"),
        ("backtick", "assurance/dossiers/unsafe`source.yaml"),
        ("bracket", "assurance/dossiers/unsafe[source].yaml"),
        ("backslash", r"assurance\dossiers\unsafe.yaml"),
        ("unicode", "assurance/dossiers/café.yaml"),
    ] {
        let fixture = copy_real_fixture(&format!("assurance-path-{label}"));
        replace_in(
            &fixture.path.join("assurance/catalog.yaml"),
            "source: assurance/dossiers/snow-snotel-swe-depth-density/dossier.yaml",
            &format!("source: {path}"),
        );
        assert!(matches!(
            Assurance::open(&fixture.path),
            Err(AssuranceError::Invalid(_))
        ));
    }

    #[cfg(unix)]
    rejects_symlink_escapes();
}

#[test]
fn authoring_only_inputs_are_planned_and_fingerprinted() {
    let fixture = copy_real_fixture("assurance-authoring-only-input");
    let relative = PathBuf::from("docs/authoring-only-input.txt");
    fs::create_dir_all(
        fixture
            .path
            .join(&relative)
            .parent()
            .expect("authoring input parent"),
    )
    .expect("create authoring input parent");
    fs::write(
        fixture.path.join(&relative),
        "bounded authoring-only input\n",
    )
    .expect("write authoring-only input");
    let authoring_path = fixture.path.join(AUTHORING_PATH);
    let mut record: serde_yaml::Value = serde_yaml::from_str(
        &fs::read_to_string(&authoring_path).expect("read authoring-only record"),
    )
    .expect("parse authoring-only record");
    let input: serde_yaml::Value = serde_yaml::from_str(&format!(
        "repository: openwepp\nrevision: current-candidate\npath: {}\nsha256: {}\navailability: tracked\nrole: Unique authoring-only dependency.\n",
        relative.display(),
        file_sha256(&fixture.path.join(&relative))
    ))
    .expect("parse authoring-only input");
    record["inputs"]
        .as_sequence_mut()
        .expect("authoring inputs")
        .push(input);
    fs::write(
        &authoring_path,
        serde_yaml::to_string(&record).expect("serialize authoring-only record"),
    )
    .expect("write authoring-only record");

    let assurance = Assurance::open(&fixture.path).expect("load authoring-only fixture");
    let plan = assurance
        .plan(&Selection::All)
        .expect("plan authoring-only fixture");
    assert!(plan.inputs.iter().any(|(path, _)| path == &relative));
    let node = assurance
        .graph()
        .nodes()
        .find(|node| node.path == relative)
        .expect("authoring-only graph node");
    assert_eq!(node.kind.label(), "authoring-input");
    assert_eq!(plan.node_fingerprints[&node.id].len(), 64);
}

#[test]
fn check_rejects_missing_and_undeclared_generated_outputs() {
    let fixture = copy_real_fixture("assurance-orphan-output");
    let assurance = Assurance::open(&fixture.path).expect("load orphan fixture");
    assurance
        .build(&Selection::All, &BuildOptions::default())
        .expect("build declared outputs");
    fs::remove_file(fixture.path.join("usersum/assurance/README.md"))
        .expect("remove declared generated output");
    assert!(matches!(
        assurance.check(&Selection::All),
        Err(AssuranceError::Drift(_))
    ));
    assurance
        .build(&Selection::All, &BuildOptions::default())
        .expect("restore declared outputs");
    fs::write(
        fixture.path.join("assurance/generated/orphan.yaml"),
        "undeclared: true\n",
    )
    .expect("write orphan output");
    assert!(matches!(
        assurance.check(&Selection::All),
        Err(AssuranceError::Drift(_))
    ));

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        use std::os::unix::net::UnixListener;

        fs::remove_file(fixture.path.join("assurance/generated/orphan.yaml"))
            .expect("remove orphan output");
        let outside = fixture.path.join("outside-generated-link.md");
        fs::write(&outside, "outside\n").expect("write symlink target");
        symlink(
            &outside,
            fixture.path.join("usersum/assurance/undeclared-link.md"),
        )
        .expect("create undeclared output symlink");
        assert!(matches!(
            assurance.check(&Selection::All),
            Err(AssuranceError::Invalid(_))
        ));
        fs::remove_file(fixture.path.join("usersum/assurance/undeclared-link.md"))
            .expect("remove undeclared output symlink");
        let socket_path = fixture.path.join("usersum/assurance/undeclared.sock");
        let _listener = UnixListener::bind(&socket_path).expect("create generated-root socket");
        assert!(matches!(
            assurance.check(&Selection::All),
            Err(AssuranceError::Invalid(_))
        ));
    }
}

#[test]
fn operations_reject_inputs_changed_after_open_before_snapshotting() {
    for (label, relative) in [
        ("narrative", NARRATIVE_PATH),
        ("template", "assurance/templates/dossier.md"),
    ] {
        let fixture = copy_real_fixture(&format!("assurance-open-drift-{label}"));
        let assurance = Assurance::open(&fixture.path).expect("open stable drift fixture");
        append_to(&fixture.path.join(relative), "\nchanged after open\n");
        let snapshot_root = fixture.path.join("must-not-snapshot");
        let options = BuildOptions {
            output_root: Some(fixture.path.clone()),
            snapshot: Some(format!("drift-{label}")),
            snapshot_root: Some(snapshot_root.clone()),
        };
        assert!(matches!(
            assurance.build(&Selection::All, &options),
            Err(AssuranceError::Drift(_))
        ));
        assert!(
            !snapshot_root.exists(),
            "input drift created a snapshot root for {label}"
        );
    }

    let removed_narrative = copy_real_fixture("assurance-open-drift-removed-narrative");
    let assurance =
        Assurance::open(&removed_narrative.path).expect("open removable narrative fixture");
    fs::remove_file(removed_narrative.path.join(NARRATIVE_PATH))
        .expect("remove narrative after open");
    let snapshot_root = removed_narrative
        .path
        .join("must-not-snapshot-removed-narrative");
    let options = BuildOptions {
        output_root: Some(removed_narrative.path.clone()),
        snapshot: Some("drift-removed-narrative".to_owned()),
        snapshot_root: Some(snapshot_root.clone()),
    };
    assert!(matches!(
        assurance.build(&Selection::All, &options),
        Err(AssuranceError::Drift(_))
    ));
    assert!(!snapshot_root.exists());

    let added_source = copy_real_fixture("assurance-open-drift-added-tool-source");
    let assurance = Assurance::open(&added_source.path).expect("open tool-source drift fixture");
    fs::write(
        added_source
            .path
            .join("crates/openwepp-assurance/src/added_after_open.rs"),
        "// added after Assurance::open\n",
    )
    .expect("add compiler input after open");
    let snapshot_root = added_source.path.join("must-not-snapshot-added-source");
    let options = BuildOptions {
        output_root: Some(added_source.path.clone()),
        snapshot: Some("drift-added-source".to_owned()),
        snapshot_root: Some(snapshot_root.clone()),
    };
    assert!(matches!(
        assurance.build(&Selection::All, &options),
        Err(AssuranceError::Drift(_))
    ));
    assert!(!snapshot_root.exists());
}

#[test]
fn production_builder_has_no_execution_network_or_agent_surface() {
    let root = repository_root();
    let source = read_rust_tree(&root.join("crates/openwepp-assurance/src"));
    for forbidden in [
        "Command::new",
        "std::process::Command",
        "TcpStream",
        "reqwest",
        "ureq",
        "nextest::",
        "agent.invoke",
    ] {
        assert!(
            !source.contains(forbidden),
            "normal builder contains forbidden execution surface {forbidden}"
        );
    }
    let manifest = fs::read_to_string(root.join("crates/openwepp-assurance/Cargo.toml"))
        .expect("read assurance Cargo manifest");
    for forbidden in ["reqwest", "tokio", "async-std", "minijinja"] {
        assert!(!manifest.contains(forbidden));
    }
    let graph_source = fs::read_to_string(root.join("crates/openwepp-assurance/src/graph.rs"))
        .expect("read dependency graph source");
    assert!(
        !graph_source.contains("fs::read") && !graph_source.contains("std::fs::read"),
        "dependency fingerprints must stream evidence assets"
    );

    let release = fs::read_to_string(root.join("tools/release/run_release_candidate_gates.sh"))
        .expect("read release candidate gate");
    for required in [
        "check_assurance_dossier_exports.sh",
        "openwepp-assurance -- build --all",
        "--snapshot \"${RELEASE_TAG}\"",
        "assurance-snapshot.sha256",
    ] {
        assert!(
            release.contains(required),
            "release consumer missing {required}"
        );
    }
}

fn assert_generated_outputs(root: &Path, outputs: &[PathBuf]) {
    for path in outputs {
        let text = fs::read_to_string(root.join(path)).expect("read generated output");
        if path.extension().and_then(|value| value.to_str()) == Some("md") {
            assert!(text.contains("Generated by openwepp-assurance; DO NOT EDIT"));
            assert_local_markdown_links_resolve(root, path, &text);
        }
        assert!(!text.contains("/home/") && !text.contains("/workdir/"));
    }
}

fn assert_export_contract(root: &Path) {
    let export: serde_yaml::Value = serde_yaml::from_str(
        &fs::read_to_string(root.join("assurance/generated/wepppy-usersum.yaml"))
            .expect("read export fragment"),
    )
    .expect("parse export fragment");
    assert_eq!(export["vendor_id"], "openwepp");
    let documents = export["documents"].as_sequence().expect("export documents");
    assert_eq!(documents.len(), 5);
    let mut doc_ids = BTreeSet::new();
    for document in documents {
        for field in [
            "doc_id",
            "source",
            "vendor_id",
            "rel_path",
            "title",
            "min_role",
            "category",
            "audience_tags",
            "status",
            "assurance_lifecycle",
            "nav_key",
        ] {
            assert!(
                !document[field].is_null(),
                "export document missing {field}"
            );
        }
        assert_eq!(document["source"], "vendor");
        assert_eq!(document["vendor_id"], "openwepp");
        assert_eq!(document["status"], "active");
        let id = document["doc_id"].as_str().expect("string doc_id");
        assert!(doc_ids.insert(id));
        let rel_path = document["rel_path"].as_str().expect("string rel_path");
        assert!(!rel_path.starts_with('/') && !rel_path.contains(".."));
    }
}

fn read_snapshot(result: &openwepp_assurance::BuildResult) -> serde_json::Value {
    serde_json::from_str(
        &fs::read_to_string(
            result
                .snapshot_manifest
                .as_ref()
                .expect("snapshot manifest path"),
        )
        .expect("read snapshot manifest"),
    )
    .expect("parse snapshot manifest")
}

fn published_fixture(label: &str) -> Scratch {
    let fixture = copy_real_fixture(label);
    set_lifecycle(&fixture.path, "published");
    approve_reviews(&fixture.path);
    fixture
}

fn renewed_review_fixture(label: &str) -> Scratch {
    let fixture = published_fixture(label);
    let path = fixture.path.join(REVIEW_PATH);
    let mut review: serde_yaml::Value =
        serde_yaml::from_str(&fs::read_to_string(&path).expect("read review history"))
            .expect("parse review history");
    let approvals = review["approvals"]
        .as_sequence_mut()
        .expect("review approval history");
    let mut scientific = approvals[0].clone();
    scientific["review_id"] = serde_yaml::Value::String("scientific-renewal-test".to_owned());
    scientific["payload_sha256"] = serde_yaml::Value::Null;
    scientific["disposition_summary"] =
        serde_yaml::Value::String("Renewed scientific disposition accepted.".to_owned());
    let mut publication = approvals[1].clone();
    publication["review_id"] = serde_yaml::Value::String("publication-renewal-test".to_owned());
    publication["payload_sha256"] = serde_yaml::Value::Null;
    publication["disposition_summary"] =
        serde_yaml::Value::String("Renewed publication disposition accepted.".to_owned());
    approvals.push(scientific);
    approvals.push(publication);
    fs::write(
        &path,
        serde_yaml::to_string(&review).expect("serialize renewed review history"),
    )
    .expect("write renewed review history");

    let plan = Assurance::open(&fixture.path)
        .expect("load renewed review fixture")
        .plan(&Selection::All)
        .expect("plan renewed review payloads");
    replace_in(
        &path,
        "payload_sha256: null",
        &format!(
            "payload_sha256: {}",
            plan.review_payloads[&format!("{DOSSIER_ID}:scientific-renewal-test")]
        ),
    );
    replace_in(
        &path,
        "payload_sha256: null",
        &format!(
            "payload_sha256: {}",
            plan.review_payloads[&format!("{DOSSIER_ID}:publication-renewal-test")]
        ),
    );
    Assurance::open(&fixture.path)
        .expect("load complete renewed review history")
        .validate(&Selection::All)
        .expect("validate complete renewed review history");
    fixture
}

fn mutate_review_approvals(root: &Path, mutation: impl FnOnce(&mut Vec<serde_yaml::Value>)) {
    let path = root.join(REVIEW_PATH);
    let mut review: serde_yaml::Value =
        serde_yaml::from_str(&fs::read_to_string(&path).expect("read review mutation target"))
            .expect("parse review mutation target");
    mutation(
        review["approvals"]
            .as_sequence_mut()
            .expect("review approvals"),
    );
    fs::write(
        path,
        serde_yaml::to_string(&review).expect("serialize mutated review"),
    )
    .expect("write mutated review");
}

fn set_lifecycle(root: &Path, lifecycle: &str) {
    replace_in(
        &root.join("assurance/catalog.yaml"),
        "lifecycle: candidate",
        &format!("lifecycle: {lifecycle}"),
    );
    replace_in(
        &root.join(DOSSIER_PATH),
        "lifecycle: candidate",
        &format!("lifecycle: {lifecycle}"),
    );
    refresh_authoring(root);
}

fn approve_reviews(root: &Path) {
    approve_authoring(root);
    let first = Assurance::open(root)
        .expect("load review fixture")
        .plan(&Selection::All)
        .expect("plan review roots");
    let scientific_root = &first.scientific_roots[DOSSIER_ID];
    let publication_root = &first.source_roots[DOSSIER_ID];
    let review = approved_review_source(scientific_root, publication_root);
    fs::write(root.join(REVIEW_PATH), review).expect("write root-bound reviews");

    let second = Assurance::open(root)
        .expect("load root-bound review fixture")
        .plan(&Selection::All)
        .expect("plan review payloads");
    let scientific = &second.review_payloads[&format!("{DOSSIER_ID}:scientific-approval-test")];
    let publication = &second.review_payloads[&format!("{DOSSIER_ID}:publication-approval-test")];
    let review_path = root.join(REVIEW_PATH);
    replace_in(
        &review_path,
        "payload_sha256: null",
        &format!("payload_sha256: {scientific}"),
    );
    replace_in(
        &review_path,
        "payload_sha256: null",
        &format!("payload_sha256: {publication}"),
    );
    Assurance::open(root)
        .expect("load approved review fixture")
        .validate(&Selection::All)
        .expect("validate approved review fixture");
}

fn approve_authoring(root: &Path) {
    let path = root.join(AUTHORING_PATH);
    let mut record: serde_yaml::Value =
        serde_yaml::from_str(&fs::read_to_string(&path).expect("read authoring record"))
            .expect("parse authoring record");
    let output_root = record["accepted_output_root_sha256"]
        .as_str()
        .expect("accepted output root")
        .to_owned();
    record["review"] = serde_yaml::from_str(&format!(
        r"state: approved
reviewer:
  name: Independent Authoring Reviewer
  role: Agent-assisted synthesis reviewer.
  expertise: Evidence inventory and scientific documentation review.
  independent_of_authors: true
  independence_basis: Did not produce the candidate synthesis.
review_date: 2026-07-14
findings: []
disposition: Agent-assisted outputs accepted for the test fixture.
approved_output_root_sha256: {output_root}
"
    ))
    .expect("parse authoring approval");
    fs::write(
        &path,
        serde_yaml::to_string(&record).expect("serialize authoring approval"),
    )
    .expect("write authoring approval");
}

fn set_authoring_pending(root: &Path) {
    let path = root.join(AUTHORING_PATH);
    let mut record: serde_yaml::Value =
        serde_yaml::from_str(&fs::read_to_string(&path).expect("read authoring record"))
            .expect("parse authoring record");
    record["review"] = serde_yaml::from_str(
        r"state: pending
reviewer: null
review_date: null
findings:
  - Independent review is pending for this test fixture.
disposition: Test fixture remains pending.
approved_output_root_sha256: null
",
    )
    .expect("parse pending authoring review");
    fs::write(
        &path,
        serde_yaml::to_string(&record).expect("serialize pending authoring review"),
    )
    .expect("write pending authoring review");
}

fn approved_review_source(scientific_root: &str, publication_root: &str) -> String {
    format!(
        r"schema_version: 1
dossier_id: {DOSSIER_ID}
dossier_version: 1.0.0
conclusion_authors:
  - name: OpenAI Codex
    role: Candidate conclusion author.
approvals:
  - review_id: scientific-approval-test
    scope: scientific
    state: approved
    reviewers:
      - name: Independent Scientific Reviewer
        role: Domain-science reviewer.
        expertise: Snow hydrology and evidence assessment.
        independent_of_authors: true
        independence_basis: Did not author the candidate conclusions.
    review_date: 2026-07-14
    reviewed_root_sha256: {scientific_root}
    payload_sha256: null
    disposition_summary: Scientific disposition accepted.
    findings: []
    residual_disagreements: []
  - review_id: publication-approval-test
    scope: publication
    state: approved
    reviewers:
      - name: Independent Publication Reviewer
        role: Publication-contract reviewer.
        expertise: Public documentation and provenance review.
        independent_of_authors: true
        independence_basis: Did not author the compiler or candidate content.
    review_date: 2026-07-14
    reviewed_root_sha256: {publication_root}
    payload_sha256: null
    disposition_summary: Publication disposition accepted.
    findings: []
    residual_disagreements: []
"
    )
}

fn refresh_authoring(root: &Path) {
    let path = root.join(AUTHORING_PATH);
    let mut record: serde_yaml::Value =
        serde_yaml::from_str(&fs::read_to_string(&path).expect("read authoring record"))
            .expect("parse authoring record");
    let outputs = record["accepted_outputs"]
        .as_sequence_mut()
        .expect("accepted outputs");
    let mut paths = Vec::new();
    for output in outputs {
        let relative = PathBuf::from(output["path"].as_str().expect("accepted output path"));
        output["sha256"] = serde_yaml::Value::String(file_sha256(&root.join(&relative)));
        paths.push(relative);
    }
    let output_root = named_file_root(root, &paths);
    record["accepted_output_root_sha256"] = serde_yaml::Value::String(output_root.clone());
    if record["review"]["state"].as_str() == Some("approved") {
        record["review"]["approved_output_root_sha256"] = serde_yaml::Value::String(output_root);
    }
    fs::write(
        &path,
        serde_yaml::to_string(&record).expect("serialize authoring record"),
    )
    .expect("write authoring record");
}

fn named_file_root(root: &Path, paths: &[PathBuf]) -> String {
    let mut ordered = paths.to_vec();
    ordered.sort();
    let mut hasher = Sha256::new();
    add_hash_field(&mut hasher, b"openwepp-assurance-agent-output-v1");
    for relative in ordered {
        add_hash_field(&mut hasher, relative.to_string_lossy().as_bytes());
        add_hash_field(
            &mut hasher,
            &fs::read(root.join(relative)).expect("read accepted output"),
        );
    }
    format!("{:x}", hasher.finalize())
}

fn file_sha256(path: &Path) -> String {
    format!(
        "{:x}",
        Sha256::digest(fs::read(path).expect("read hash input"))
    )
}

fn add_hash_field(hasher: &mut Sha256, bytes: &[u8]) {
    hasher.update((bytes.len() as u64).to_be_bytes());
    hasher.update(bytes);
}

#[cfg(unix)]
fn rejects_symlink_escapes() {
    use std::os::unix::fs::symlink;

    let fixture = copy_real_fixture("assurance-symlink");
    let outside = Scratch::new("assurance-symlink-outside");
    let method = fixture
        .path
        .join("assurance/methods/snow-snotel-evaluation-v1.yaml");
    fs::remove_file(&method).expect("remove copied method");
    let outside_method = outside.path.join("method.yaml");
    fs::write(&outside_method, "schema_version: 1\n").expect("write outside method");
    symlink(&outside_method, &method).expect("create input symlink");
    assert!(matches!(
        Assurance::open(&fixture.path),
        Err(AssuranceError::Invalid(_))
    ));

    let fixture = copy_real_fixture("assurance-output-symlink");
    let assurance = Assurance::open(&fixture.path).expect("load output symlink fixture");
    let output_root = fixture.path.join("staging");
    fs::create_dir(&output_root).expect("create staging root");
    symlink(&outside.path, output_root.join("usersum")).expect("create output symlink");
    assert!(matches!(
        assurance.build(&Selection::All, &output_options(&output_root)),
        Err(AssuranceError::Invalid(_))
    ));
}

fn copy_real_fixture(label: &str) -> Scratch {
    let source_root = repository_root();
    let source = Assurance::open(&source_root).expect("load source assurance");
    let plan = source.plan(&Selection::All).expect("plan fixture inputs");
    let fixture = Scratch::new(label);
    for (relative, _) in plan.inputs {
        copy_file(&source_root, &fixture.path, &relative);
    }
    fixture
}

fn copy_file(source_root: &Path, destination_root: &Path, relative: &Path) {
    let destination = destination_root.join(relative);
    fs::create_dir_all(destination.parent().expect("copied file parent"))
        .expect("create copied file parent");
    fs::copy(source_root.join(relative), destination).expect("copy fixture file");
}

fn replace_in(path: &Path, old: &str, new: &str) {
    let source = fs::read_to_string(path).expect("read replacement target");
    assert!(source.contains(old), "replacement marker missing: {old}");
    fs::write(path, source.replacen(old, new, 1)).expect("write replacement target");
}

fn append_to(path: &Path, addition: &str) {
    let mut source = fs::read_to_string(path).expect("read append target");
    source.push_str(addition);
    fs::write(path, source).expect("write append target");
}

fn output_options(root: &Path) -> BuildOptions {
    BuildOptions {
        output_root: Some(root.to_path_buf()),
        ..BuildOptions::default()
    }
}

fn snapshot_options(root: &Path, id: &str) -> BuildOptions {
    BuildOptions {
        output_root: Some(root.to_path_buf()),
        snapshot: Some(id.to_owned()),
        snapshot_root: Some(root.join("snapshots")),
    }
}

fn assert_local_markdown_links_resolve(root: &Path, relative: &Path, text: &str) {
    assert!(!text.contains("://"), "public output contains a URI");
    let parent = root
        .join(relative)
        .parent()
        .expect("generated document parent")
        .to_path_buf();
    for tail in text.split("](").skip(1) {
        let target = tail
            .split(')')
            .next()
            .expect("Markdown destination")
            .split('#')
            .next()
            .unwrap_or_default();
        if !target.is_empty() {
            assert!(
                parent.join(target).is_file(),
                "broken generated link in {}: {target}",
                relative.display()
            );
        }
    }
}

fn read_rust_tree(root: &Path) -> String {
    let mut paths = fs::read_dir(root)
        .expect("read Rust source directory")
        .map(|entry| entry.expect("read source entry").path())
        .collect::<Vec<_>>();
    paths.sort();
    paths
        .into_iter()
        .filter(|path| path.extension().and_then(|value| value.to_str()) == Some("rs"))
        .map(|path| fs::read_to_string(path).expect("read Rust source"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

static SCRATCH_COUNTER: AtomicU64 = AtomicU64::new(0);

struct Scratch {
    path: PathBuf,
}

impl Scratch {
    fn new(label: &str) -> Self {
        let sequence = SCRATCH_COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!("{label}-{}-{sequence}", std::process::id()));
        if path.exists() {
            fs::remove_dir_all(&path).expect("remove stale scratch directory");
        }
        fs::create_dir(&path).expect("create scratch directory");
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
