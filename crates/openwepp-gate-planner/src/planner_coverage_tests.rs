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
    }

    #[test]
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
    }
