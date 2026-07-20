# Owned File Manifest

Evidence mode: `Static`

Status: `PASS — exact 57-path diff is inside the 60-path authorization`

Base: `a749ed7a0e8b05667661d560612b4b54b90695ac`

The final name-status diff contains exactly these paths:

| Status | Path |
|---|---|
| M | `.config/nextest.toml` |
| M | `Cargo.lock` |
| M | `crates/openwepp-gate-planner/src/planner.rs` |
| M | `crates/openwepp-gate-planner/src/verifier.rs` |
| M | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs` |
| M | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs` |
| M | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion.rs` |
| M | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs` |
| M | `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/05_projection_helpers.rs` |
| M | `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs` |
| M | `crates/openwepp-input-contract/src/parsers/management.rs` |
| M | `crates/openwepp-landuse-migrate/src/convert.rs` |
| M | `crates/openwepp-management-schema/src/lib.rs` |
| M | `crates/openwepp-plant-phenology/src/lib.rs` |
| A | `crates/openwepp-plant-phenology/tests/native_canopy_contract.rs` |
| M | `crates/openwepp-runner/Cargo.toml` |
| M | `crates/openwepp-runner/src/hillslope/03_tests.rs` |
| M | `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs` |
| M | `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs` |
| M | `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs` |
| M | `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00d_authority_runtime_impl.rs` |
| M | `crates/openwepp-runner/src/hillslope/tests03/direct_publication_source_guards.rs` |
| M | `docs/ROADMAP.md` |
| M | `docs/backlog/20260626-frost-daylength-canopy-decline-hemisphere-robust.md` |
| M | `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-YAML-001.md` |
| M | `docs/specifications/science-contracts/contracts/SC-PLANT-001.md` |
| M | `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md` |
| M | `docs/specifications/science-contracts/index.md` |
| M | `docs/work-packages/20260719-canopy-phenology-native-integration-001/artifacts/conservation-audit.md` |
| M | `docs/work-packages/20260719-canopy-phenology-native-integration-001/artifacts/consumer-path-proof.md` |
| M | `docs/work-packages/20260719-canopy-phenology-native-integration-001/artifacts/contract-implementation-evidence.md` |
| M | `docs/work-packages/20260719-canopy-phenology-native-integration-001/artifacts/contract-test-implementation-evidence.md` |
| M | `docs/work-packages/20260719-canopy-phenology-native-integration-001/artifacts/disposition.md` |
| M | `docs/work-packages/20260719-canopy-phenology-native-integration-001/artifacts/gate-results.md` |
| M | `docs/work-packages/20260719-canopy-phenology-native-integration-001/artifacts/implementation-and-test-evidence.md` |
| M | `docs/work-packages/20260719-canopy-phenology-native-integration-001/artifacts/intent-plan.md` |
| M | `docs/work-packages/20260719-canopy-phenology-native-integration-001/artifacts/kernel-profile-compliance.md` |
| M | `docs/work-packages/20260719-canopy-phenology-native-integration-001/artifacts/line-count-governance.md` |
| M | `docs/work-packages/20260719-canopy-phenology-native-integration-001/artifacts/operand-lineage.md` |
| M | `docs/work-packages/20260719-canopy-phenology-native-integration-001/artifacts/owned-file-manifest.md` |
| M | `docs/work-packages/20260719-canopy-phenology-native-integration-001/artifacts/pre-implementation-contract-gate.md` |
| M | `docs/work-packages/20260719-canopy-phenology-native-integration-001/artifacts/review-disposition.md` |
| M | `docs/work-packages/20260719-canopy-phenology-native-integration-001/artifacts/science-contracts/SC-INFILE-MANAGEMENT-YAML-001/contract_ref.md` |
| M | `docs/work-packages/20260719-canopy-phenology-native-integration-001/artifacts/science-contracts/SC-PLANT-001/contract_ref.md` |
| M | `docs/work-packages/20260719-canopy-phenology-native-integration-001/artifacts/science-contracts/SC-RESIDUE-001/contract_ref.md` |
| M | `docs/work-packages/20260719-canopy-phenology-native-integration-001/artifacts/verification_agent_a.md` |
| M | `docs/work-packages/20260719-canopy-phenology-native-integration-001/artifacts/verification_agent_b.md` |
| M | `docs/work-packages/20260719-canopy-phenology-native-integration-001/artifacts/worker-handoff.md` |
| M | `docs/work-packages/20260719-canopy-phenology-native-integration-001/package.md` |
| M | `docs/work-packages/README.md` |
| M | `gate-policy/v1/gate-definitions.json` |
| M | `gate-policy/v1/impact-map.json` |
| M | `tests/fixtures/infile/management/canonical_forest_nonzero_ow_lanuse_1.man.yaml` |
| M | `tests/integration/infile_management_parser_contract.rs` |
| M | `tests/integration/infile_management_yaml_contract.rs` |
| M | `tests/integration/testgate_align_authority_contract.rs` |
| M | `tests/integration/testgate_ci_executor_contract.rs` |

The 60-path prospective authorization also names
`direct_runtime/storage.rs`, `review_agent_a.md`, and `review_agent_b.md`; they
remain unchanged and are absent from the exact diff. No changed path is outside
the declared write set or authenticated manifest.
