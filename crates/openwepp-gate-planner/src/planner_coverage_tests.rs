    use super::{
        InventoryProvider, PlanRequest, Planner, PlanningStage, authority_suite_inventory,
        prepare_reconstruction_workspace, reconcile_intent_terminal, reconcile_semantics, select,
        validate_request,
    };
    use crate::canonical::canonical_bytes;
    use crate::error::Result;
    use crate::policy::{GateDefinition, PolicyBundle};
    use crate::repository::{CargoGraph, ObservedChange, ObservedSource};

    struct FixedInventory;

    #[test]
    fn authority_inventory_enumerates_required_level_four_suites() {
        let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let inventory = authority_suite_inventory(&repo).expect("authority suite inventory");
        assert_eq!(inventory.len(), 9);
        assert!(inventory.iter().all(|suite| suite.starts_with("cas_l4_")));
    }

    impl InventoryProvider for FixedInventory {
        fn inventory(
            &self,
            _repo: &std::path::Path,
            definition: &GateDefinition,
            target: &str,
        ) -> Result<Vec<String>> {
            Ok(vec![format!("{}:{target}", definition.gate_definition_id)])
        }
    }

    fn assert_measurement_only_global_risk(plan: &serde_json::Value) {
        assert_eq!(plan["risk"]["class"], "CRITICAL");
        assert!(
            plan["risk"]["reason_codes"]
                .as_array()
                .expect("reason codes")
                .contains(&serde_json::json!(
                    "MEASUREMENT_ONLY_PACKAGE_REQUIRES_GLOBAL_QUALITY"
                ))
        );
        assert_eq!(plan["quality_scope"]["mode"], "GLOBAL");
    }

    #[cfg(unix)]
    #[test]
    fn reconstruction_workspace_rejects_symlink_escape() {
        use std::os::unix::fs::symlink;

        let root = std::env::temp_dir().join(format!(
            "openwepp-reconstruction-symlink-{}",
            std::process::id()
        ));
        let escape = std::env::temp_dir().join(format!(
            "openwepp-reconstruction-escape-{}",
            std::process::id()
        ));
        std::fs::create_dir(&root).expect("create selected root");
        std::fs::create_dir(&escape).expect("create escape root");
        let workspace = root.join("reconstruction");
        symlink(&escape, &workspace).expect("create workspace symlink");
        let error = prepare_reconstruction_workspace(&workspace)
            .expect_err("workspace symlink must fail closed");
        assert_eq!(error.code, "GATE-RECONSTRUCTION-WORKSPACE");
        std::fs::remove_file(&workspace).expect("remove workspace symlink");

        std::fs::create_dir(&workspace).expect("create plain workspace");
        symlink(&escape, workspace.join("cargo-target")).expect("create child symlink");
        let error = prepare_reconstruction_workspace(&workspace)
            .expect_err("child symlink must fail closed");
        assert_eq!(error.code, "GATE-RECONSTRUCTION-WORKSPACE");

        std::fs::remove_dir_all(&root).expect("remove precise selected root");
        std::fs::remove_dir_all(&escape).expect("remove precise escape root");
    }

    #[test]
    fn reconstruction_workspace_creates_owned_children_and_rejects_plain_files() {
        let fixture = crate::executor::tests::TempDirectory::new("planner-workspace-objects");
        let parent_file = fixture.path().join("parent-file");
        std::fs::write(&parent_file, b"not a directory").expect("parent file");
        assert_eq!(
            prepare_reconstruction_workspace(&parent_file.join("workspace"))
                .expect_err("plain-file parent must fail")
                .code,
            "GATE-RECONSTRUCTION-WORKSPACE"
        );

        let workspace_file = fixture.path().join("workspace-file");
        std::fs::write(&workspace_file, b"not a directory").expect("workspace file");
        assert_eq!(
            prepare_reconstruction_workspace(&workspace_file)
                .expect_err("plain-file workspace must fail")
                .code,
            "GATE-RECONSTRUCTION-WORKSPACE"
        );

        let workspace = fixture.path().join("created-workspace");
        let canonical = prepare_reconstruction_workspace(&workspace)
            .expect("missing workspace and owned children are created");
        for child in ["cargo-target", "graph-snapshots", "inventory-snapshots"] {
            assert!(canonical.join(child).is_dir(), "missing owned child {child}");
        }
        std::fs::remove_dir_all(canonical.join("cargo-target")).expect("remove owned child");
        std::fs::write(canonical.join("cargo-target"), b"not a directory")
            .expect("replace child with file");
        assert_eq!(
            prepare_reconstruction_workspace(&canonical)
                .expect_err("plain-file child must fail")
                .code,
            "GATE-RECONSTRUCTION-WORKSPACE"
        );
    }

    #[test]
    fn gate_policy_change_is_deterministic_and_critical() {
        let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let head = std::process::Command::new("git")
            .args(["rev-parse", "HEAD"])
            .current_dir(&repo)
            .output()
            .expect("git rev-parse");
        let head = String::from_utf8(head.stdout)
            .expect("UTF-8 head")
            .trim()
            .to_owned();
        let request = PlanRequest {
            stage: PlanningStage::Intent,
            predecessor_intent_plan_id: None,
            boundary: "INCREMENT".to_owned(),
            campaign_id: Some("TESTGATE-PLAN-01".to_owned()),
            combined_quality_proof_id: None,
            authorized_paths: vec!["gate-policy/v1/impact-map.json".to_owned()],
            package_authority_chain_id: "aa".repeat(32),
            intent_package_path: "docs/work-packages/fixture/package.md".to_owned(),
            source: ObservedSource {
                base_commit: head,
                head_commit: None,
                dirty_tree_digest: Some("11".repeat(32)),
                index_digest: Some("22".repeat(32)),
                worktree_digest: Some("33".repeat(32)),
                untracked_digest: Some("44".repeat(32)),
                changes: vec![ObservedChange {
                    path: "gate-policy/v1/impact-map.json".to_owned(),
                    change_kind: "MODIFY".to_owned(),
                    object_kind: "REGULAR".to_owned(),
                    old_mode: Some("100644".to_owned()),
                    new_mode: Some("100644".to_owned()),
                }],
            },
        };
        let planner = Planner::new(FixedInventory);
        let first = planner.build(&repo, &request).expect("first plan");
        assert_eq!(
            canonical_bytes(&first).expect("first canonicalization"),
            canonical_bytes(&first).expect("second canonicalization")
        );
        assert_eq!(first["risk"]["class"], "CRITICAL");
        assert_eq!(first["quality_scope"]["mode"], "GLOBAL");
        assert_eq!(first["quality_scope"]["completeness"], "ESCALATED_GLOBAL");
        assert!(
            first["nodes"]
                .as_array()
                .expect("nodes")
                .iter()
                .all(|node| { node["gate_definition_id"] != "documentation-lint-v1" })
        );
        assert_eq!(
            first["nodes"]
                .as_array()
                .expect("nodes")
                .iter()
                .filter(|node| node["gate_definition_id"] == "adjudicated-crap-v1")
                .count(),
            1
        );
        assert!(!first["nodes"].as_array().expect("nodes").is_empty());
        let ids = first["nodes"]
            .as_array()
            .expect("nodes")
            .iter()
            .map(|node| node["node_id"].as_str().expect("node ID"))
            .collect::<std::collections::BTreeSet<_>>();
        assert!(!ids.is_empty());
    }

    #[test]
    fn science_surfaces_select_explicit_a1_and_gate_planner_is_critical() {
        let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let policy = PolicyBundle::load(&repo).expect("policy bundle");
        let graph = CargoGraph::load_current(&repo).expect("Cargo graph");
        for (path, expected_gate) in [
            (
                "crates/openwepp-hillslope-orchestrator/src/direct_runtime/groundwater.rs",
                "hard-invariant-groundwater-v1",
            ),
            (
                "crates/openwepp-meteorology/src/phase.rs",
                "hard-invariant-snow-phase-v1",
            ),
        ] {
            let selection = select(
                &policy,
                &graph,
                &[ObservedChange {
                    path: path.to_owned(),
                    change_kind: "MODIFY".to_owned(),
                    object_kind: "REGULAR".to_owned(),
                    old_mode: Some("100644".to_owned()),
                    new_mode: Some("100644".to_owned()),
                }],
            );
            assert_eq!(selection.risk.as_str(), "CRITICAL", "{path}");
            assert!(selection.unmapped.is_empty(), "{path}");
            assert!(
                selection
                    .explicit_definitions
                    .iter()
                    .any(|definition| definition == expected_gate),
                "{path}"
            );
        }

        let selection = select(
            &policy,
            &graph,
            &[ObservedChange {
                path: "crates/openwepp-gate-planner/src/verifier.rs".to_owned(),
                change_kind: "MODIFY".to_owned(),
                object_kind: "REGULAR".to_owned(),
                old_mode: Some("100644".to_owned()),
                new_mode: Some("100644".to_owned()),
            }],
        );
        assert_eq!(selection.risk.as_str(), "CRITICAL");
        assert!(selection.unmapped.is_empty());
    }

    #[test]
    fn nonproduction_cargo_targets_require_global_quality() {
        let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let policy = PolicyBundle::load(&repo).expect("policy bundle");
        let metadata = br#"{"packages":[{"id":"test-only 0.1","name":"test-only","manifest_path":"/repo/crates/test-only/Cargo.toml","features":{},"targets":[{"kind":["test"],"src_path":"/repo/crates/test-only/src/contract.rs"}]},{"id":"out-of-tree 0.1","name":"out-of-tree","manifest_path":"/repo/crates/out-of-tree/Cargo.toml","features":{},"targets":[{"kind":["lib"],"src_path":"/repo/shared/lib.rs"}]}],"workspace_members":["test-only 0.1","out-of-tree 0.1"],"resolve":{"nodes":[{"id":"test-only 0.1","deps":[]},{"id":"out-of-tree 0.1","deps":[]}]}}"#;
        let graph = CargoGraph::from_metadata(metadata, std::path::Path::new("/repo"))
            .expect("Cargo graph");
        for path in [
            "crates/test-only/src/contract.rs",
            "crates/out-of-tree/src/lib.rs",
        ] {
            let selection = select(
                &policy,
                &graph,
                &[ObservedChange {
                    path: path.to_owned(),
                    change_kind: "MODIFY".to_owned(),
                    object_kind: "REGULAR".to_owned(),
                    old_mode: Some("100644".to_owned()),
                    new_mode: Some("100644".to_owned()),
                }],
            );
            assert_eq!(selection.risk.as_str(), "CRITICAL", "{path}");
            assert!(selection.reason_codes.iter().any(|reason| {
                reason == "MEASUREMENT_ONLY_PACKAGE_REQUIRES_GLOBAL_QUALITY"
            }));
            assert!(selection.quality_packages.is_empty());
        }
    }

    #[test]
    fn assurance_reconciliation_allows_declared_to_exact_but_not_watch_removal() {
        let impact = |kind: &str, watches: serde_json::Value| {
            serde_json::json!({
                "report_id": "report",
                "registry_generation": 1,
                "registry_sha256": "a".repeat(64),
                "source_root": "b".repeat(64),
                "assessed_realization_root": "c".repeat(64),
                "campaign_id": "campaign",
                "requested_action": "ASSESS",
                "watch_generation": 1,
                "impact_state": "OPEN_ASSESSMENT",
                "resolution_authority": {"principal_id": "lead", "role_id": "report_lead", "role_record_sha256": "d".repeat(64)},
                "assessed_realization_integrity": "CURRENT",
                "campaign_impact_disposition": "IMPACT_PENDING",
                "campaign_transfer_request": "NOT_REQUESTED",
                "campaign_transfer_currency": "BLOCKED",
                "release_transfer_request": "NOT_REQUESTED",
                "release_transfer_currency": "BLOCKED",
                "lifecycle_boundaries": ["CAMPAIGN_CLOSURE"],
                "mapping_complete": true,
                "matching_watch_ids": watches,
                "changed_object": {"path": "crates/example/src/lib.rs", "change_kind": kind}
            })
        };
        let intent = serde_json::json!({
            "changed_objects": [{"path": "crates/example/src/lib.rs"}],
            "assurance_impacts": [impact("DECLARED", serde_json::json!(["package", "path"]))]
        });
        let terminal = serde_json::json!({
            "changed_objects": [{"path": "crates/example/src/lib.rs"}],
            "assurance_impacts": [impact("MODIFY", serde_json::json!(["package", "path"]))]
        });
        let actual = std::collections::BTreeSet::from(["crates/example/src/lib.rs".to_owned()]);
        crate::assurance::reconcile_assurance_impacts(&intent, &terminal, &actual)
            .expect("exact terminal impact");

        let weakened = serde_json::json!({
            "changed_objects": [{"path": "crates/example/src/lib.rs"}],
            "assurance_impacts": [impact("MODIFY", serde_json::json!(["package"]))]
        });
        assert!(
            crate::assurance::reconcile_assurance_impacts(&intent, &weakened, &actual).is_err()
        );

        let untouched = serde_json::json!({"changed_objects": [], "assurance_impacts": []});
        crate::assurance::reconcile_assurance_impacts(
            &intent,
            &untouched,
            &std::collections::BTreeSet::new(),
        )
        .expect("an authorized but untouched path creates no terminal impact");
    }

    #[test]
    fn retained_shadow_replay_has_no_selection_misses() {
        let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let policy = PolicyBundle::load(&repo).expect("policy bundle");
        let graph = CargoGraph::load_current(&repo).expect("Cargo graph");
        let fixture = std::fs::read(
            repo.join("gate-policy/v1/fixtures/replay/retained-selection-cases.json"),
        )
        .expect("replay fixture");
        let fixture: serde_json::Value = serde_json::from_slice(&fixture).expect("replay JSON");
        for case in fixture["cases"].as_array().expect("replay cases") {
            let changes = case["paths"]
                .as_array()
                .expect("paths")
                .iter()
                .map(|path| ObservedChange {
                    path: path.as_str().expect("path").to_owned(),
                    change_kind: "MODIFY".to_owned(),
                    object_kind: "REGULAR".to_owned(),
                    old_mode: Some("100644".to_owned()),
                    new_mode: Some("100644".to_owned()),
                })
                .collect::<Vec<_>>();
            let selection = select(&policy, &graph, &changes);
            assert_eq!(
                selection.risk.as_str(),
                case["expected_risk"],
                "{}",
                case["case_id"]
            );
            assert_eq!(
                selection.unmapped.len(),
                usize::try_from(case["expected_unmapped"].as_u64().expect("unmapped"))
                    .expect("usize unmapped")
            );
            for package in case["must_affect"].as_array().expect("must affect") {
                assert!(
                    selection
                        .affected_packages
                        .iter()
                        .any(|selected| selected == package.as_str().expect("package")),
                    "{} missed {package}",
                    case["case_id"]
                );
            }
        }
    }

    #[test]
    fn ordinary_documentation_is_editorial_and_mapped() {
        let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let policy = PolicyBundle::load(&repo).expect("policy bundle");
        let graph = CargoGraph::load_current(&repo).expect("Cargo graph");
        let changes = [ObservedChange {
            path: "docs/example/operator-note.md".to_owned(),
            change_kind: "MODIFY".to_owned(),
            object_kind: "REGULAR".to_owned(),
            old_mode: Some("100644".to_owned()),
            new_mode: Some("100644".to_owned()),
        }];
        let selection = select(&policy, &graph, &changes);
        assert_eq!(selection.risk.as_str(), "EDITORIAL");
        assert!(selection.unmapped.is_empty());
        assert_eq!(
            selection.documentation_paths,
            ["docs/example/operator-note.md"]
        );
        assert!(
            selection
                .reason_codes
                .iter()
                .any(|reason| reason == "DOCUMENTATION_ONLY")
        );
    }

    #[test]
    fn normative_documentation_is_unknown_critical_without_policy_mapping() {
        let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let policy = PolicyBundle::load(&repo).expect("policy bundle");
        let graph = CargoGraph::load_current(&repo).expect("Cargo graph");
        for path in [
            "docs/specifications/science-contracts/contracts/SC-EXAMPLE-999.md",
            "docs/decisions/9999-example.md",
            "docs/contracts/example.md",
            "docs/architecture/example.md",
            "docs/numerics/example.md",
            "docs/codex_exec_plans.md",
            "docs/defect_closure_execplans.md",
            "crates/example/AGENTS.md",
        ] {
            let changes = [ObservedChange {
                path: path.to_owned(),
                change_kind: "MODIFY".to_owned(),
                object_kind: "REGULAR".to_owned(),
                old_mode: Some("100644".to_owned()),
                new_mode: Some("100644".to_owned()),
            }];
            let selection = select(&policy, &graph, &changes);
            assert_eq!(selection.risk.as_str(), "CRITICAL", "{path}");
            assert_eq!(selection.unmapped.len(), 1, "{path}");
            assert!(
                selection
                    .reason_codes
                    .iter()
                    .any(|reason| reason == "UNKNOWN_INPUT"),
                "{path}"
            );
        }
    }

    #[test]
    fn prospective_work_package_changes_do_not_force_critical_selection() {
        let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let policy = PolicyBundle::load(&repo).expect("policy bundle");
        let graph = CargoGraph::load_current(&repo).expect("Cargo graph");
        let changes = [ObservedChange {
            path: "docs/work-packages/20260718-example-001/package.md".to_owned(),
            change_kind: "MODIFY".to_owned(),
            object_kind: "REGULAR".to_owned(),
            old_mode: Some("100644".to_owned()),
            new_mode: Some("100644".to_owned()),
        }];
        let selection = select(&policy, &graph, &changes);
        assert_eq!(selection.risk.as_str(), "EDITORIAL");
        assert!(selection.unmapped.is_empty());
    }

    #[test]
    fn planner_rejects_non_increment_lifecycle_boundaries() {
        let request = PlanRequest {
            stage: PlanningStage::Intent,
            predecessor_intent_plan_id: None,
            boundary: "RELEASE".to_owned(),
            campaign_id: None,
            combined_quality_proof_id: None,
            authorized_paths: vec!["README.md".to_owned()],
            package_authority_chain_id: "aa".repeat(32),
            intent_package_path: "docs/work-packages/fixture/package.md".to_owned(),
            source: ObservedSource {
                base_commit: "1".repeat(40),
                head_commit: None,
                dirty_tree_digest: Some("11".repeat(32)),
                index_digest: Some("22".repeat(32)),
                worktree_digest: Some("33".repeat(32)),
                untracked_digest: Some("44".repeat(32)),
                changes: Vec::new(),
            },
        };
        let error = validate_request(&request).expect_err("release must fail closed");
        assert_eq!(error.code, "GATE-PLAN-BOUNDARY");
    }

    #[test]
    fn ordinary_rust_plan_builds_unique_per_package_outputs() {
        let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let head = std::process::Command::new("git")
            .args(["rev-parse", "HEAD"])
            .current_dir(&repo)
            .output()
            .expect("git rev-parse");
        let head = String::from_utf8(head.stdout)
            .expect("UTF-8 head")
            .trim()
            .to_owned();
        let path = "crates/openwepp-comparator-metadata/src/lib.rs";
        let plan = Planner::new(FixedInventory)
            .build(
                &repo,
                &PlanRequest {
                    stage: PlanningStage::Intent,
                    predecessor_intent_plan_id: None,
                    boundary: "INCREMENT".to_owned(),
                    campaign_id: Some("TESTGATE-PLAN-01".to_owned()),
                    combined_quality_proof_id: None,
                    authorized_paths: vec![path.to_owned()],
                    package_authority_chain_id: "aa".repeat(32),
                    intent_package_path: "docs/work-packages/fixture/package.md".to_owned(),
                    source: ObservedSource {
                        base_commit: head,
                        head_commit: None,
                        dirty_tree_digest: Some("11".repeat(32)),
                        index_digest: Some("22".repeat(32)),
                        worktree_digest: Some("33".repeat(32)),
                        untracked_digest: Some("44".repeat(32)),
                        changes: vec![ObservedChange {
                            path: path.to_owned(),
                            change_kind: "MODIFY".to_owned(),
                            object_kind: "REGULAR".to_owned(),
                            old_mode: Some("100644".to_owned()),
                            new_mode: Some("100644".to_owned()),
                        }],
                    },
                },
            )
            .expect("ordinary Rust plan");
        let package_nodes = plan["nodes"]
            .as_array()
            .expect("nodes")
            .iter()
            .filter(|node| {
                matches!(
                    node["gate_definition_id"].as_str(),
                    Some("cargo-package-clippy-v1" | "cargo-package-nextest-v1")
                )
            })
            .collect::<Vec<_>>();
        assert!(package_nodes.len() >= 4);
        let outputs = package_nodes
            .iter()
            .flat_map(|node| node["output_paths"].as_array().expect("outputs"))
            .map(|path| path.as_str().expect("output path"))
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(outputs.len(), package_nodes.len());
        assert_eq!(
            plan["quality_scope"]["mode"],
            "AFFECTED",
            "scope={} affected={} nextest_targets={}",
            plan["quality_scope"],
            plan["affected_packages"],
            serde_json::Value::Array(
                plan["nodes"]
                    .as_array()
                    .expect("nodes")
                    .iter()
                    .filter(|node| node["gate_definition_id"] == "cargo-package-nextest-v1")
                    .map(|node| node["target"].clone())
                    .collect()
            )
        );
        assert_eq!(plan["quality_scope"]["completeness"], "COMPLETE");
        let affected_nodes = plan["nodes"]
            .as_array()
            .expect("nodes")
            .iter()
            .filter(|node| node["gate_definition_id"] == "affected-adjudicated-crap-v1")
            .collect::<Vec<_>>();
        assert_eq!(affected_nodes.len(), 1);
        let arguments = affected_nodes[0]["arguments"]
            .as_array()
            .expect("affected arguments");
        for package in plan["quality_scope"]["production_packages"]
            .as_array()
            .expect("production packages")
        {
            assert!(arguments.contains(package));
        }
        assert!(!arguments.contains(&serde_json::json!("openwepp")));
    }

    #[test]
    fn root_measurement_change_escalates_to_global_crap() {
        let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let head = std::process::Command::new("git")
            .args(["rev-parse", "HEAD"])
            .current_dir(&repo)
            .output()
            .expect("git rev-parse");
        let head = String::from_utf8(head.stdout)
            .expect("UTF-8 head")
            .trim()
            .to_owned();
        let path = "tests/integration/testgate_ci_executor_contract.rs";
        let plan = Planner::new(FixedInventory)
            .build(
                &repo,
                &PlanRequest {
                    stage: PlanningStage::Intent,
                    predecessor_intent_plan_id: None,
                    boundary: "INCREMENT".to_owned(),
                    campaign_id: Some("TESTGATE-ROOT-MEASUREMENT-01".to_owned()),
                    combined_quality_proof_id: None,
                    authorized_paths: vec![path.to_owned()],
                    package_authority_chain_id: "aa".repeat(32),
                    intent_package_path: "docs/work-packages/fixture/package.md".to_owned(),
                    source: ObservedSource {
                        base_commit: head,
                        head_commit: None,
                        dirty_tree_digest: Some("11".repeat(32)),
                        index_digest: Some("22".repeat(32)),
                        worktree_digest: Some("33".repeat(32)),
                        untracked_digest: Some("44".repeat(32)),
                        changes: vec![ObservedChange {
                            path: path.to_owned(),
                            change_kind: "MODIFY".to_owned(),
                            object_kind: "REGULAR".to_owned(),
                            old_mode: Some("100644".to_owned()),
                            new_mode: Some("100644".to_owned()),
                        }],
                    },
                },
            )
            .expect("root measurement plan");

        assert_measurement_only_global_risk(&plan);
        let definitions = plan["nodes"]
            .as_array()
            .expect("nodes")
            .iter()
            .filter_map(|node| node["gate_definition_id"].as_str())
            .collect::<std::collections::BTreeSet<_>>();
        assert!(definitions.contains("adjudicated-crap-v1"));
        assert!(!definitions.contains("affected-adjudicated-crap-v1"));
    }

    #[test]
    #[cfg_attr(not(coverage), ignore = "development-only: repeatedly constructs complete repository plans")]
    #[allow(clippy::too_many_lines)]
    fn terminal_reconciliation_reports_added_paths_and_escalation() {
        let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let head = std::process::Command::new("git")
            .args(["rev-parse", "HEAD"])
            .current_dir(&repo)
            .output()
            .expect("git rev-parse");
        let head = String::from_utf8(head.stdout)
            .expect("UTF-8 head")
            .trim()
            .to_owned();
        let source = |changes| ObservedSource {
            base_commit: head.clone(),
            head_commit: None,
            dirty_tree_digest: Some("11".repeat(32)),
            index_digest: Some("22".repeat(32)),
            worktree_digest: Some("33".repeat(32)),
            untracked_digest: Some("44".repeat(32)),
            changes,
        };
        let planner = Planner::new(FixedInventory);
        let intent = planner
            .build(
                &repo,
                &PlanRequest {
                    stage: PlanningStage::Intent,
                    predecessor_intent_plan_id: None,
                    boundary: "INCREMENT".to_owned(),
                    campaign_id: Some("TESTGATE-PLAN-01".to_owned()),
                    combined_quality_proof_id: None,
                    authorized_paths: vec!["Cargo.lock".to_owned()],
                    package_authority_chain_id: "aa".repeat(32),
                    intent_package_path: "docs/work-packages/fixture/package.md".to_owned(),
                    source: source(Vec::new()),
                },
            )
            .expect("intent");
        let terminal = planner
            .build(
                &repo,
                &PlanRequest {
                    stage: PlanningStage::Terminal,
                    predecessor_intent_plan_id: intent["plan_id"].as_str().map(str::to_owned),
                    boundary: "INCREMENT".to_owned(),
                    campaign_id: Some("TESTGATE-PLAN-01".to_owned()),
                    combined_quality_proof_id: None,
                    authorized_paths: vec!["Cargo.lock".to_owned()],
                    package_authority_chain_id: "aa".repeat(32),
                    intent_package_path: "docs/work-packages/fixture/package.md".to_owned(),
                    source: source(vec![ObservedChange {
                        path: "Cargo.lock".to_owned(),
                        change_kind: "MODIFY".to_owned(),
                        object_kind: "REGULAR".to_owned(),
                        old_mode: Some("100644".to_owned()),
                        new_mode: Some("100644".to_owned()),
                    }]),
                },
            )
            .expect("terminal");
        let reconciliation = reconcile_semantics(&intent, &terminal).expect("reconciliation");
        assert_eq!(reconciliation.added_paths, ["Cargo.lock"]);
        assert!(!reconciliation.risk_escalated);

        let mut forged = terminal.clone();
        forged["plan_id"] = serde_json::json!("f".repeat(64));
        assert!(reconcile_intent_terminal(&repo, &intent, &forged).is_err());

        let reidentify = |plan: &mut serde_json::Value| {
            plan["plan_id"] = serde_json::json!(super::derive_plan_id(plan).expect("plan ID"));
            plan["execution_key"] =
                serde_json::json!(super::derive_execution_key(plan).expect("execution key"));
        };
        let mut intent_with_path = terminal.clone();
        intent_with_path["planning_stage"] = serde_json::json!("INTENT");
        intent_with_path["predecessor_intent_plan_id"] = serde_json::Value::Null;
        reidentify(&mut intent_with_path);
        let mut removed_path = intent_with_path.clone();
        removed_path["planning_stage"] = serde_json::json!("TERMINAL");
        removed_path["predecessor_intent_plan_id"] = intent_with_path["plan_id"].clone();
        removed_path["changed_objects"] = serde_json::json!([]);
        reidentify(&mut removed_path);
        let removed = reconcile_semantics(&intent_with_path, &removed_path)
            .expect("authorized path may remain untouched at terminal");
        assert_eq!(removed.removed_paths, ["Cargo.lock"]);

        let mut downgraded = intent_with_path.clone();
        downgraded["planning_stage"] = serde_json::json!("TERMINAL");
        downgraded["predecessor_intent_plan_id"] = intent_with_path["plan_id"].clone();
        downgraded["risk"]["class"] = serde_json::json!("EDITORIAL");
        reidentify(&mut downgraded);
        assert!(reconcile_intent_terminal(&repo, &intent_with_path, &downgraded).is_err());

        let mut removed_gate = intent_with_path.clone();
        removed_gate["planning_stage"] = serde_json::json!("TERMINAL");
        removed_gate["predecessor_intent_plan_id"] = intent_with_path["plan_id"].clone();
        removed_gate["nodes"].as_array_mut().expect("nodes").pop();
        reidentify(&mut removed_gate);
        assert!(reconcile_intent_terminal(&repo, &intent_with_path, &removed_gate).is_err());

        let mut weakened = intent_with_path.clone();
        weakened["planning_stage"] = serde_json::json!("TERMINAL");
        weakened["predecessor_intent_plan_id"] = intent_with_path["plan_id"].clone();
        let last = weakened["nodes"]
            .as_array_mut()
            .expect("nodes")
            .last_mut()
            .expect("node");
        last["acceptance"]["expected"] = serde_json::json!(9);
        last["node_id"] =
            serde_json::json!(crate::canonical::derived_id(last, "node_id").expect("node ID"));
        reidentify(&mut weakened);
        assert!(reconcile_intent_terminal(&repo, &intent_with_path, &weakened).is_err());

        let semantic_error = |intent: &serde_json::Value,
                              terminal: &serde_json::Value,
                              code: &str| {
            assert_eq!(
                reconcile_semantics(intent, terminal)
                    .expect_err("semantic guard must fail")
                    .code,
                code
            );
        };
        let semantic_terminal = removed_path.clone();

        let mut invalid_intent_stage = intent_with_path.clone();
        invalid_intent_stage["planning_stage"] = serde_json::json!("TERMINAL");
        semantic_error(
            &invalid_intent_stage,
            &semantic_terminal,
            "GATE-PLAN-RECONCILIATION",
        );
        let mut invalid_terminal_stage = semantic_terminal.clone();
        invalid_terminal_stage["planning_stage"] = serde_json::json!("INTENT");
        semantic_error(
            &intent_with_path,
            &invalid_terminal_stage,
            "GATE-PLAN-RECONCILIATION",
        );
        let mut invalid_predecessor = semantic_terminal.clone();
        invalid_predecessor["predecessor_intent_plan_id"] = serde_json::json!("wrong");
        semantic_error(
            &intent_with_path,
            &invalid_predecessor,
            "GATE-PLAN-RECONCILIATION",
        );

        let mut malformed_changes = semantic_terminal.clone();
        malformed_changes["changed_objects"] = serde_json::Value::Null;
        semantic_error(
            &intent_with_path,
            &malformed_changes,
            "GATE-PLAN-CHANGES",
        );
        let mut missing_changed_path = semantic_terminal.clone();
        missing_changed_path["changed_objects"] = serde_json::json!([{}]);
        semantic_error(
            &intent_with_path,
            &missing_changed_path,
            "GATE-PLAN-CHANGE-PATH",
        );

        let mut mismatched_authorization = semantic_terminal.clone();
        mismatched_authorization["authorized_paths"] = serde_json::json!(["Cargo.toml"]);
        semantic_error(
            &intent_with_path,
            &mismatched_authorization,
            "GATE-TERMINAL-UNAUTHORIZED-PATH",
        );
        let mut unauthorized_actual = semantic_terminal.clone();
        unauthorized_actual["changed_objects"] = serde_json::json!([{
            "path": "Cargo.toml"
        }]);
        semantic_error(
            &intent_with_path,
            &unauthorized_actual,
            "GATE-TERMINAL-UNAUTHORIZED-PATH",
        );
        let mut mismatched_proof = semantic_terminal.clone();
        mismatched_proof["combined_quality"]["requested_proof_id"] =
            serde_json::json!("different-proof");
        semantic_error(
            &intent_with_path,
            &mismatched_proof,
            "GATE-TERMINAL-UNAUTHORIZED-PATH",
        );

        let mut invalid_intent_risk = intent_with_path.clone();
        invalid_intent_risk["risk"]["class"] = serde_json::json!("UNKNOWN");
        semantic_error(&invalid_intent_risk, &semantic_terminal, "GATE-PLAN-RISK");
        let mut invalid_terminal_risk = semantic_terminal.clone();
        invalid_terminal_risk["risk"]["class"] = serde_json::json!("UNKNOWN");
        semantic_error(&intent_with_path, &invalid_terminal_risk, "GATE-PLAN-RISK");

        let mut malformed_affected = semantic_terminal.clone();
        malformed_affected["affected_packages"] = serde_json::Value::Null;
        semantic_error(
            &intent_with_path,
            &malformed_affected,
            "GATE-PLAN-SHAPE",
        );
        let mut removed_affected_intent = intent_with_path.clone();
        removed_affected_intent["affected_packages"] = serde_json::json!(["required-package"]);
        semantic_error(
            &removed_affected_intent,
            &semantic_terminal,
            "GATE-TERMINAL-OBLIGATION-REMOVED",
        );
        let mut malformed_reverse = semantic_terminal.clone();
        malformed_reverse["reverse_dependencies"] = serde_json::json!([1]);
        semantic_error(
            &intent_with_path,
            &malformed_reverse,
            "GATE-PLAN-SHAPE",
        );
        let mut removed_reverse_intent = intent_with_path.clone();
        removed_reverse_intent["reverse_dependencies"] = serde_json::json!(["required-package"]);
        semantic_error(
            &removed_reverse_intent,
            &semantic_terminal,
            "GATE-TERMINAL-OBLIGATION-REMOVED",
        );

        let mut malformed_impact_edges = semantic_terminal.clone();
        malformed_impact_edges["impact_edges"] = serde_json::Value::Null;
        semantic_error(
            &intent_with_path,
            &malformed_impact_edges,
            "GATE-PLAN-SHAPE",
        );
        let mut required_impact_intent = intent_with_path.clone();
        required_impact_intent["impact_edges"] =
            serde_json::json!([{"entry_id": "required-edge"}]);
        semantic_error(
            &required_impact_intent,
            &semantic_terminal,
            "GATE-TERMINAL-OBLIGATION-REMOVED",
        );
        let mut deferred = semantic_terminal.clone();
        deferred["deferred_obligations"] = serde_json::json!([{"owner": "later"}]);
        semantic_error(
            &intent_with_path,
            &deferred,
            "GATE-TERMINAL-RETROACTIVE-DEFERRAL",
        );
    }

    fn node_fixture(definition: &str, target: &str, id: &str) -> serde_json::Value {
        serde_json::json!({
            "node_id": id,
            "gate_definition_id": definition,
            "target": target,
            "arguments": ["check"],
            "prerequisites": [],
            "expected_inventory": {
                "mode": "EXACT",
                "minimum_count": 1,
                "ids": ["inventory"]
            },
            "acceptance": {"kind": "EXIT_CODE", "expected": 0}
        })
    }

    #[test]
    fn terminal_node_superset_accepts_stronger_gates_and_rejects_weakening() {
        let package_clippy = node_fixture("cargo-package-clippy-v1", "fixture", "package");
        let workspace_clippy = node_fixture("workspace-clippy-v1", "workspace", "workspace");
        let intent = serde_json::json!({"nodes": [package_clippy.clone()]});
        let stronger = serde_json::json!({"nodes": [workspace_clippy]});
        super::require_node_superset(&intent, &stronger).expect("workspace gate supersedes package");

        let removed = serde_json::json!({"nodes": []});
        let error = super::require_node_superset(&intent, &removed)
            .expect_err("unrecognized removal must fail");
        assert_eq!(error.code, "GATE-TERMINAL-OBLIGATION-REMOVED");

        let terminal = serde_json::json!({"nodes": [package_clippy.clone()]});
        super::require_node_superset(&intent, &terminal).expect("identical node");

        let mut weakened = package_clippy;
        weakened["arguments"] = serde_json::json!(["check", "--weakened"]);
        let weakened = serde_json::json!({"nodes": [weakened]});
        let error = super::require_node_superset(&intent, &weakened)
            .expect_err("semantic weakening must fail");
        assert_eq!(error.code, "GATE-TERMINAL-NODE-WEAKENED");

        let nextest = node_fixture("cargo-package-nextest-v1", "fixture", "nextest");
        let full = node_fixture("workspace-full-nextest-v1", "workspace", "full");
        super::require_node_superset(
            &serde_json::json!({"nodes": [nextest]}),
            &serde_json::json!({"nodes": [full]}),
        )
        .expect("full Nextest supersedes package Nextest");
    }

    #[test]
    fn risk_and_node_argument_guards_cover_all_typed_outcomes() {
        for (risk, rank) in [
            ("EDITORIAL", 0),
            ("BOUNDED_COMPONENT", 1),
            ("INTEGRATED_DOMAIN", 2),
            ("CRITICAL", 3),
        ] {
            assert_eq!(super::risk_rank(Some(risk)).expect("known risk"), rank);
        }
        assert_eq!(
            super::risk_rank(None).expect_err("missing risk").code,
            "GATE-PLAN-RISK"
        );
        assert_eq!(
            super::risk_rank(Some("UNKNOWN"))
                .expect_err("unknown risk")
                .code,
            "GATE-PLAN-RISK"
        );

        let node = serde_json::json!({
            "arguments": ["nextest", "--package", "one", "--package", "two"]
        });
        assert_eq!(
            super::node_argument_values(&node, "--package").expect("package arguments"),
            ["one", "two"]
        );
        assert!(super::node_argument_values(&node, "--test").is_err());
        assert!(super::node_argument_values(&serde_json::json!({"arguments": [1]}), "--package")
            .is_err());
        assert!(super::node_argument_values(
            &serde_json::json!({"arguments": ["--package"]}),
            "--package"
        )
        .is_err());
    }

    #[test]
    fn node_semantic_and_graph_guards_fail_closed_on_malformed_inputs() {
        let node = node_fixture("fixture-light-v1", "workspace", "placeholder");
        assert_eq!(
            super::require_node_semantics("fixture", &serde_json::Value::Null, &node)
                .expect_err("non-object intent node must fail")
                .code,
            "GATE-PLAN-SHAPE"
        );
        assert_eq!(
            super::require_node_semantics("fixture", &node, &serde_json::Value::Null)
                .expect_err("non-object terminal node must fail")
                .code,
            "GATE-PLAN-SHAPE"
        );

        assert_eq!(
            super::verify_node_graph(&[serde_json::json!({})])
                .expect_err("missing node ID must fail")
                .code,
            "GATE-NODE-SHAPE"
        );

        let identified = |mut value: serde_json::Value| {
            value["node_id"] = serde_json::Value::Null;
            value["node_id"] = serde_json::json!(
                crate::canonical::derived_id(&value, "node_id").expect("derived node ID")
            );
            value
        };
        let first = identified(serde_json::json!({
            "node_id": null,
            "gate_definition_id": "fixture-light-v1",
            "target": "first",
            "prerequisites": [],
            "output_paths": ["target/first.json"]
        }));
        assert_eq!(
            super::verify_node_graph(&[first.clone(), first.clone()])
                .expect_err("duplicate node ID must fail")
                .code,
            "GATE-NODE-IDENTITY"
        );

        let dangling = identified(serde_json::json!({
            "node_id": null,
            "gate_definition_id": "fixture-light-v1",
            "target": "dangling",
            "prerequisites": ["missing"],
            "output_paths": ["target/dangling.json"]
        }));
        assert_eq!(
            super::verify_node_graph(&[dangling])
                .expect_err("unseen prerequisite must fail")
                .code,
            "GATE-NODE-DAG"
        );

        let second = identified(serde_json::json!({
            "node_id": null,
            "gate_definition_id": "fixture-light-v1",
            "target": "second",
            "prerequisites": [],
            "output_paths": ["target/first.json"]
        }));
        assert_eq!(
            super::verify_node_graph(&[first, second])
                .expect_err("duplicate output must fail")
                .code,
            "GATE-NODE-OUTPUT-DUPLICATE"
        );
    }

    #[cfg(unix)]
    #[test]
    fn dirty_manifest_roots_replace_remove_and_add_owned_objects() {
        use std::os::unix::fs::symlink;

        let repo = crate::executor::tests::TempDirectory::new("planner-dirty-manifest");
        let run = |arguments: &[&str]| {
            let output = std::process::Command::new("git")
                .args(arguments)
                .current_dir(repo.path())
                .output()
                .expect("run fixture Git");
            assert!(
                output.status.success(),
                "git {arguments:?}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            String::from_utf8(output.stdout)
                .expect("UTF-8 Git output")
                .trim()
                .to_owned()
        };
        run(&["init", "-q"]);
        for (path, content) in [
            ("Cargo.toml", "[workspace]\nresolver = \"3\"\n"),
            ("tools/obsolete.sh", "#!/bin/sh\nexit 0\n"),
            ("docs/standards/fixture.md", "# Fixture\n"),
            ("assurance/existing.txt", "existing\n"),
        ] {
            let path = repo.path().join(path);
            std::fs::create_dir_all(path.parent().expect("fixture parent"))
                .expect("fixture directory");
            std::fs::write(path, content).expect("fixture file");
        }
        run(&["add", "."]);
        run(&[
            "-c",
            "user.name=Codex Test",
            "-c",
            "user.email=codex@example.invalid",
            "commit",
            "-q",
            "-m",
            "manifest base",
        ]);
        let head = run(&["rev-parse", "HEAD"]);
        let baseline = super::manifest_roots(repo.path(), &head, false)
            .expect("committed manifest roots");

        std::fs::remove_file(repo.path().join("tools/obsolete.sh"))
            .expect("remove included file");
        let deleted = super::manifest_roots(repo.path(), &head, true)
            .expect("manifest after included deletion");
        assert_ne!(deleted["execution_root"], baseline["execution_root"]);
        assert_eq!(deleted["assurance_root"], baseline["assurance_root"]);
        assert_eq!(deleted["authority_root"], baseline["authority_root"]);
        assert_eq!(
            deleted["documentation_root"],
            baseline["documentation_root"]
        );

        std::fs::write(repo.path().join("notes.txt"), "excluded\n")
            .expect("add excluded file");
        let excluded = super::manifest_roots(repo.path(), &head, true)
            .expect("manifest after excluded addition");
        assert_eq!(excluded, deleted);

        symlink("existing.txt", repo.path().join("assurance/new-link"))
            .expect("add included symlink");
        let linked = super::manifest_roots(repo.path(), &head, true)
            .expect("manifest after included symlink addition");
        assert_eq!(linked["execution_root"], excluded["execution_root"]);
        assert_ne!(linked["assurance_root"], excluded["assurance_root"]);
        assert_eq!(linked["authority_root"], excluded["authority_root"]);
        assert_eq!(
            linked["documentation_root"],
            excluded["documentation_root"]
        );

        std::fs::write(
            repo.path().join("Cargo.toml"),
            "[workspace]\nresolver = \"2\"\n",
        )
        .expect("modify included regular file");
        let modified = super::manifest_roots(repo.path(), &head, true)
            .expect("manifest after included regular modification");
        assert_ne!(modified["execution_root"], linked["execution_root"]);
        assert_eq!(modified["assurance_root"], linked["assurance_root"]);
        assert_eq!(modified["authority_root"], linked["authority_root"]);
        assert_eq!(
            modified["documentation_root"],
            linked["documentation_root"]
        );

        assert_eq!(
            super::manifest_roots(repo.path(), &head, false)
                .expect("dirty files excluded from committed roots"),
            baseline
        );
    }

    #[cfg(unix)]
    #[test]
    fn manifest_identity_and_git_guards_cover_object_kinds_and_failures() {
        use std::os::unix::ffi::OsStringExt;
        use std::os::unix::fs::symlink;

        let fixture = crate::executor::tests::TempDirectory::new("planner-manifest-identity");
        std::fs::write(fixture.path().join("regular"), b"content").expect("regular file");
        std::fs::create_dir(fixture.path().join("directory")).expect("directory");
        symlink("regular", fixture.path().join("link")).expect("symlink");
        symlink(
            std::ffi::OsString::from_vec(vec![0xff]),
            fixture.path().join("non-utf8-link"),
        )
        .expect("non-UTF8 symlink");

        let regular = super::manifest_object_identity(fixture.path(), "regular")
            .expect("regular identity");
        assert_eq!(regular.0, "REGULAR");
        assert_eq!(regular.1, Some("100644"));
        assert!(regular.2.is_some());
        assert_eq!(
            super::manifest_object_identity(fixture.path(), "missing")
                .expect("missing identity"),
            ("MISSING", None, None)
        );
        let link = super::manifest_object_identity(fixture.path(), "link")
            .expect("symlink identity");
        assert_eq!(link.0, "SYMLINK");
        assert_eq!(link.1, Some("120000"));
        assert!(link.2.is_some());
        assert_eq!(
            super::manifest_object_identity(fixture.path(), "directory")
                .expect_err("directory must fail")
                .code,
            "GATE-MANIFEST-OBJECT"
        );
        assert_eq!(
            super::manifest_object_identity(fixture.path(), "non-utf8-link")
                .expect_err("non-UTF8 symlink target must fail")
                .code,
            "GATE-MANIFEST-SYMLINK-NONUTF8"
        );

        let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        assert!(!super::git_bytes(&repo, &["rev-parse", "HEAD"], "TEST-GIT")
            .expect("valid Git query")
            .is_empty());
        assert_eq!(
            super::git_bytes(&repo, &["rev-parse", "not-a-revision"], "TEST-GIT")
                .expect_err("invalid revision")
                .code,
            "TEST-GIT"
        );
        assert!(
            super::git_blob_batch(&repo, std::iter::empty::<&str>())
                .expect("empty blob batch")
                .is_empty()
        );

        assert!(
            super::tracked_tree_entry(b"100644 blob deadbeef\texcluded.txt")
                .expect("excluded tree entry")
                .is_none()
        );
        for entry in [
            &b"missing-tab"[..],
            &b"100644 tree deadbeef\tCargo.toml"[..],
            &b"100644 blob\tCargo.toml"[..],
            &b"100644 blob deadbeef extra\tCargo.toml"[..],
            &b"100644 blob deadbeef\tCargo.toml\xff"[..],
        ] {
            assert!(
                super::tracked_tree_entry(entry).is_err(),
                "malformed tree entry must fail: {entry:?}"
            );
        }

        for (bytes, count) in [
            (&b""[..], 1),
            (&b"oid blob invalid\n"[..], 1),
            (&b"oid blob 3\nab"[..], 1),
            (&b"oid blob 1\na\ntrailing"[..], 1),
        ] {
            assert_eq!(
                super::parse_blob_batch(bytes, count)
                    .expect_err("malformed batch")
                    .code,
                "GATE-MANIFEST-BLOB"
            );
        }
    }

    #[test]
    fn inventory_dispatch_covers_command_package_and_unsupported_sources() {
        let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let policy = PolicyBundle::load(&repo).expect("policy bundle");
        let command = policy
            .definition("documentation-lint-v1")
            .expect("command definition")
            .clone();
        let node = serde_json::json!({"arguments": ["markdown-doc", "lint"]});
        let direct = super::inventory_for_definition(&repo, &node, &command, "workspace", None)
            .expect("command inventory");
        assert_eq!(direct.len(), 1);

        let confined = super::ConfinedNextestInventory {
            cargo_target: repo.join("target/planner-confined-inventory-test"),
        };
        assert_eq!(
            confined
                .inventory(&repo, &command, "workspace")
                .expect("confined command inventory"),
            direct
        );

        let mut unsupported = command.clone();
        unsupported.inventory_source = "UNSUPPORTED".to_owned();
        assert_eq!(
            super::inventory_for_definition(&repo, &node, &unsupported, "workspace", None)
                .expect_err("unsupported inventory")
                .code,
            "GATE-INVENTORY-SOURCE"
        );
        assert_eq!(
            confined
                .inventory(&repo, &unsupported, "workspace")
                .expect_err("unsupported confined inventory")
                .code,
            "GATE-INVENTORY-SOURCE"
        );

        let mut packages = command;
        packages.inventory_source = "NEXTEST_PACKAGES".to_owned();
        assert!(super::package_inventories(&repo, &node, &packages, None).is_err());

        let target = crate::executor::tests::TempDirectory::new("planner-package-inventory");
        let package_node = serde_json::json!({
            "arguments": ["nextest", "--package", "openwepp-gate-planner"]
        });
        let package_inventory = super::package_inventories(
            &repo,
            &package_node,
            &packages,
            Some(&target.path().join("cargo-target")),
        )
        .expect("one package inventory");
        assert!(!package_inventory.is_empty());

        let mut authority = packages.clone();
        authority.inventory_source = "AUTHORITY_SUITES".to_owned();
        assert_eq!(
            super::inventory_for_definition(&repo, &node, &authority, "workspace", None)
                .expect("authority inventory")
                .len(),
            9
        );

        let mut nextest = packages;
        nextest.inventory_source = "NEXTEST_PACKAGE".to_owned();
        let empty = crate::executor::tests::TempDirectory::new("planner-empty-inventory");
        assert_eq!(
            super::inventory_for_definition(
                empty.path(),
                &node,
                &nextest,
                "missing",
                Some(&target.path().join("cargo-target")),
            )
            .expect_err("missing manifest must fail")
            .code,
            "GATE-NEXTEST-LIST"
        );
        assert_eq!(
            super::ConfinedNextestInventory {
                cargo_target: target.path().join("cargo-target"),
            }
            .inventory(empty.path(), &nextest, "missing")
            .expect_err("confined missing manifest must fail")
            .code,
            "GATE-NEXTEST-LIST"
        );
    }

    #[test]
    fn terminal_reconciliation_accepts_canonical_isolated_reconstruction() {
        use crate::executor::tests::{execution_fixture, gate_definition};

        let documentation =
            gate_definition("documentation-lint-v1", &["./tools/pass.sh"], &[]);
        let component = gate_definition("fixture-light-v1", &["./tools/pass.sh"], &[]);
        let (repo, mut terminal) = execution_fixture(
            "planner-reconciliation-reconstruction",
            &[documentation, component],
        );
        let mut intent = terminal.clone();
        intent["planning_stage"] = serde_json::json!("INTENT");
        intent["predecessor_intent_plan_id"] = serde_json::Value::Null;
        intent["plan_id"] =
            serde_json::json!(super::derive_plan_id(&intent).expect("intent plan ID"));
        intent["execution_key"] =
            serde_json::json!(super::derive_execution_key(&intent).expect("intent key"));
        terminal["predecessor_intent_plan_id"] = intent["plan_id"].clone();
        terminal["plan_id"] =
            serde_json::json!(super::derive_plan_id(&terminal).expect("terminal plan ID"));
        terminal["execution_key"] =
            serde_json::json!(super::derive_execution_key(&terminal).expect("terminal key"));

        let reconciliation = reconcile_intent_terminal(repo.path(), &intent, &terminal)
            .expect("canonical terminal reconstruction");
        assert!(reconciliation.added_paths.is_empty());
        assert!(reconciliation.removed_paths.is_empty());
        assert!(!reconciliation.risk_escalated);
    }

    fn run_graph_fixture_git(repo: &std::path::Path, arguments: &[&str]) -> String {
        let output = std::process::Command::new("git")
            .args(arguments)
            .current_dir(repo)
            .output()
            .expect("run fixture Git");
        assert!(
            output.status.success(),
            "git {arguments:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout)
            .expect("UTF-8 Git output")
            .trim()
            .to_owned()
    }

    fn commit_graph_fixture(repo: &std::path::Path, message: &str) -> String {
        run_graph_fixture_git(repo, &["add", "."]);
        run_graph_fixture_git(
            repo,
            &[
                "-c",
                "user.name=Codex Test",
                "-c",
                "user.email=codex@example.invalid",
                "commit",
                "-q",
                "-m",
                message,
            ],
        );
        run_graph_fixture_git(repo, &["rev-parse", "HEAD"])
    }

    fn write_graph_fixture_package(repo: &std::path::Path, name: &str) {
        let root = repo.join(name);
        std::fs::create_dir_all(root.join("src")).expect("package source directory");
        std::fs::write(
            root.join("Cargo.toml"),
            format!(
                "[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nexecutor-fixture = {{ path = \"..\" }}\n"
            ),
        )
        .expect("package manifest");
        std::fs::write(root.join("src/lib.rs"), "pub fn uses_root() {}\n")
            .expect("package source");
    }

    fn write_graph_fixture_workspace(
        repo: &std::path::Path,
        member: &str,
        expectation: &str,
    ) {
        std::fs::write(
            repo.join("Cargo.toml"),
            format!(
                "[package]\nname = \"executor-fixture\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[workspace]\nmembers = [\"{member}\"]\nresolver = \"3\"\n"
            ),
        )
        .expect(expectation);
    }

    fn generate_graph_fixture_lockfile(repo: &std::path::Path, expectation: &str) {
        let lock = std::process::Command::new("cargo")
            .args(["generate-lockfile", "--offline"])
            .current_dir(repo)
            .output()
            .expect(expectation);
        assert!(lock.status.success());
    }

    fn graph_union_request(base: String, head: String) -> PlanRequest {
        PlanRequest {
            stage: PlanningStage::Intent,
            predecessor_intent_plan_id: None,
            boundary: "INCREMENT".to_owned(),
            campaign_id: Some("TESTGATE-PLAN-01".to_owned()),
            combined_quality_proof_id: None,
            authorized_paths: vec!["src/lib.rs".to_owned()],
            package_authority_chain_id: "aa".repeat(32),
            intent_package_path: "docs/work-packages/fixture/package.md".to_owned(),
            source: ObservedSource {
                base_commit: base,
                head_commit: Some(head),
                dirty_tree_digest: None,
                index_digest: None,
                worktree_digest: None,
                untracked_digest: None,
                changes: vec![ObservedChange {
                    path: "src/lib.rs".to_owned(),
                    change_kind: "MODIFY".to_owned(),
                    object_kind: "REGULAR".to_owned(),
                    old_mode: Some("100644".to_owned()),
                    new_mode: Some("100644".to_owned()),
                }],
            },
        }
    }

    fn assert_exact_graph_selection(
        plan: &serde_json::Value,
        request: &PlanRequest,
        context: &serde_json::Value,
    ) {
        assert_eq!(plan["source"]["base_commit"], request.source.base_commit);
        assert_eq!(
            plan["source"]["head_commit"],
            serde_json::json!(request.source.head_commit)
        );
        assert_eq!(&plan["execution_context"], context);
        assert_measurement_only_global_risk(plan);
        assert_eq!(
            plan["affected_packages"],
            serde_json::json!(["consumer", "executor-fixture", "legacy"])
        );
        assert_eq!(
            plan["reverse_dependencies"],
            serde_json::json!(["consumer", "legacy"])
        );
        let package_targets = plan["nodes"]
            .as_array()
            .expect("nodes")
            .iter()
            .filter(|node| node["gate_definition_id"] == "cargo-package-nextest-v1")
            .map(|node| node["target"].as_str().expect("target"))
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(
            package_targets,
            std::collections::BTreeSet::from(["consumer", "executor-fixture", "legacy"])
        );
    }

    fn assert_invalid_base_precedes_invalid_head(
        repo: &std::path::Path,
        workspace: &std::path::Path,
        context: &serde_json::Value,
        request: PlanRequest,
    ) {
        let mut invalid = request;
        invalid.source.base_commit = "missing-base-revision".to_owned();
        invalid.source.head_commit = Some("missing-head-revision".to_owned());
        let error = Planner::new(FixedInventory)
            .build_with_workspace_and_context(
                repo,
                &invalid,
                Some(workspace),
                Some(context),
            )
            .expect_err("invalid base must fail before invalid head");
        assert_eq!(error.code, "GATE-GIT-COMMAND");
        assert!(error.message.contains("missing-base-revision"));
        assert!(!error.message.contains("missing-head-revision"));
    }

    #[test]
    fn workspace_and_bound_context_plan_build_preserves_exact_graph_selection() {
        use crate::executor::tests::{execution_fixture, gate_definition};

        let mut package_definition =
            gate_definition("cargo-package-nextest-v1", &["./tools/pass.sh", "{package}"], &[]);
        package_definition["target_template"] = serde_json::json!("CARGO_PACKAGE");
        package_definition["risk_classes"] = serde_json::json!(["BOUNDED_COMPONENT"]);
        package_definition["output_paths"] =
            serde_json::json!(["target/e2e/{package}-result.json"]);
        let component = gate_definition("fixture-light-v1", &["./tools/pass.sh"], &[]);
        let (repo, _) = execution_fixture(
            "planner-graph-union",
            &[package_definition, component],
        );

        write_graph_fixture_package(repo.path(), "legacy");
        write_graph_fixture_workspace(repo.path(), "legacy", "base workspace manifest");
        generate_graph_fixture_lockfile(repo.path(), "base lockfile");
        let base = commit_graph_fixture(repo.path(), "base graph with legacy dependent");

        std::fs::remove_dir_all(repo.path().join("legacy")).expect("remove legacy package");
        write_graph_fixture_package(repo.path(), "consumer");
        write_graph_fixture_workspace(repo.path(), "consumer", "head workspace manifest");
        generate_graph_fixture_lockfile(repo.path(), "head lockfile");
        let head = commit_graph_fixture(repo.path(), "head graph with consumer dependent");

        let request = graph_union_request(base, head);
        let fixture = crate::executor::tests::TempDirectory::new("planner-workspace-build");
        let workspace = fixture.path().join("workspace");
        std::fs::create_dir(&workspace).expect("workspace");
        let context = super::current_execution_context(repo.path()).expect("execution context");
        let plan = Planner::new(FixedInventory)
            .build_with_workspace_and_context(
                repo.path(),
                &request,
                Some(&workspace),
                Some(&context),
            )
            .expect("workspace-bound plan");
        assert_exact_graph_selection(&plan, &request, &context);
        assert_invalid_base_precedes_invalid_head(repo.path(), &workspace, &context, request);
    }
