# Owned File Manifest

Status: `terminal diff reconciled`

Base identity: `a65cc3973ddd04b07cad108fcb33d83a8c161abb`.

Implementation/contract/test identity:
`33831787b7029b28b0716c8458f08a11899db446`.

The terminal package diff contains exactly the paths below. Package-local
verification artifacts are included because the two verifiers append their
receipts after the closure-candidate commit.

## Production And Tests

```text
Cargo.toml
crates/openwepp-hillslope-orchestrator/src/constants.rs
crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs
crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs
crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs
crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs
crates/openwepp-hillslope-orchestrator/src/direct_runtime/04_audit_error_helpers.rs
crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion.rs
crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs
crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs
crates/openwepp-hillslope-orchestrator/src/direct_runtime/tests/erosion_hb01.rs
crates/openwepp-hillslope-orchestrator/src/hydrology/02_guard_errors.rs
crates/openwepp-hillslope-orchestrator/src/lib.rs
crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/cqr_laned_active_executor.rs
crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs
crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_dc01.rs
crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r3c_r4b.rs
crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r4il.rs
crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r7g_frost.rs
crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_wave1_continuity.rs
crates/openwepp-hillslope-output/src/hillslope_pass.rs
crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs
crates/openwepp-runner/src/hillslope/01_scheduler_and_trace.rs
crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs
crates/openwepp-runner/src/hillslope/03_tests.rs
crates/openwepp-runner/src/hillslope/04_direct_publication.rs
crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs
crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs
crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00b_ksatadj_authority_impl.rs
crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs
crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00d_authority_runtime_impl.rs
crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/01_frost_and_layer_helpers.rs
crates/openwepp-runner/src/hillslope/direct_seed_projections/01_wb12_wb16_wb19_projection.rs
crates/openwepp-runner/src/hillslope/laned_shadow.rs
crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs
tests/integration/cli03_runner_contract_derived_tests.rs
tests/integration/erod16_wave1_continuity_fixture_conservation.rs
tests/integration/erosion_multi_ofe_p102_chain.rs
tests/integration/erosion_single_ofe_p61_sediment.rs
tests/integration/laned_shadow_h2637.rs
tests/integration/peak_hourly_authority_contract.rs
```

## Authority And Lifecycle Documentation

```text
docs/backlog/20260807-canopy-peak-runoff-discontinuity.md
docs/backlog/TRACKER.md
docs/specifications/science-contracts/contracts/SC-INFILE-HBP-001.md
docs/specifications/science-contracts/contracts/SC-SED-001.md
docs/specifications/science-contracts/contracts/SC-WATBAL-001.md
docs/specifications/science-contracts/index.md
docs/work-packages/README.md
```

## Package Scaffold, Tools, And Narrative Evidence

```text
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/README.md
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/command-log.json
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/contract-implementation-evidence.md
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/contract-test-implementation-evidence.md
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/disposition.md
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/finding-disposition.md
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/gate-results.md
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/implementation-test-evidence.md
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/intent-plan.md
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/kernel-profile-compliance.md
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/line-count-governance.md
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/mutation-study.md
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/operand-lineage.md
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/owned-file-manifest.md
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/pre-implementation-contract-gate.md
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/required-reading-map.md
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/review_agent_a.md
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/review_agent_b.md
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/rust_code_review.md
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/rust_qa_review.md
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/summary.json
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/summary.md
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/verification_agent_a.md
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/verification_agent_b.md
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/worker-handoff.md
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/package.md
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/prompts/README.md
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/prompts/active/20260809-hourly-peak-runoff-authority-closure-001_kickoff_agent_prompt.md
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/prompts/active/README.md
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/prompts/archived/README.md
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/tools/test_topanga_openwepp_census.py
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/tools/topanga_openwepp_census.py
```

## Retained Execution Receipts

```text
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/cargo-test-workspace-doc.command-log.txt
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/cargo-test-workspace-doc.log
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/comparator-probe-build-v2.log
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/comparator-probe-build-v5.log
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/comparator-probe-build.log
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/comparator-probe-run.log
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/full-list.command-log.txt
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/full-list.log
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/full-list.sorted
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/full-vs-quick-summary.log
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/quick-list.command-log.txt
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/quick-list.log
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/quick-list.sorted
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/quick-only.identities
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/topanga-openwepp-census-full-v3-7820953.log
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/topanga-openwepp-census-full-v4-cargo-doc.log
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/topanga-openwepp-census-full-v4-nextest-full-clean-target-0d5fa08b2.log
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/topanga-openwepp-census-full-v4-nextest-full-clean-target-33831787b.log
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/topanga-openwepp-census-full-v4-nextest-full-clean-target-ff7c91846.log
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/topanga-openwepp-census-full-v4-nextest-full-clean-target.log
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/topanga-openwepp-census-full-v4-nextest-full-retry-threads2.log
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/topanga-openwepp-census-full-v4-nextest-full-threads4.log
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/topanga-openwepp-census-full-v4-nextest-full.log
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/topanga-openwepp-census-full-v4-nextest-quick-threads4.log
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/topanga-openwepp-census-full-v4-nextest-quick.log
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/topanga-openwepp-census-full-v4.log
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/topanga-openwepp-census-full-v5.log
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/topanga-openwepp-census-full.log
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/topanga-openwepp-census-probe-resume-v2.log
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/topanga-openwepp-census-probe-v2.log
docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/topanga-openwepp-census-run.log
```

External generated evidence remains outside Git at
`/home/workdir/openwepp-hourly-peak-topanga-census-20260809-v5`; its hashes and
paths are bound in `summary.json` and `command-log.json`.
