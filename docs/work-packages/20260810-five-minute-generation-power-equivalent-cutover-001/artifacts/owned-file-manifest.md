# Owned File Manifest

Status: `reopened implementation increment reconciled — 36/36 paths owned before terminal evidence refresh`

Evidence mode: `Static + Ran`

The prior terminal package-owned set contained 87 changed or added paths. The
separate predecessor/backlog closure diff is intentionally excluded and is
owned by package `20260809-hourly-peak-runoff-authority-closure-001`.

## Reopened 2026-08-11 increment (36)

Production, tests, authority, and user contracts:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/subhourly_generation.rs`
- `crates/openwepp-hillslope-output/src/hillslope_wat_subhourly.rs`
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `crates/openwepp-runner/src/hillslope/04_direct_publication.rs`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-runner/src/hillslope/output_transaction.rs`
- `crates/openwepp-runner/src/hillslope/tests03/cqr_laned_active_outputs.rs`
- `crates/openwepp-runner/src/hillslope/tests03/simimpl.rs`
- `crates/openwepp-runner/src/hillslope/tests03/wat5_output_transaction.rs`
- `crates/openwepp-sim-contract/src/units_mod/output_catalog.rs`
- `docs/contracts/openwepp-hillslope-runfile-contract.md`
- `docs/contracts/openwepp-runner-contract.md`
- `docs/specifications/science-contracts/contracts/SC-OUTPUT-WAT5-001.md`
- `docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md`
- `tests/integration/advisory_linter_authority_contract.rs`
- `tests/integration/subhourly_generation_contract.rs`
- `tests/integration/subhourly_generation_properties.rs`
- `tests/integration/subhourly_water_output_roundtrip.rs`
- `tools/release/authority-policy/impact-map.json`

Lifecycle and evidence:

- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/package.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/adoption-disposition.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/disposition.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/finding-disposition.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/five-minute-water-closure.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/gate-results.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/implementation-test-evidence.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/line-count-governance.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/operand-lineage.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/owned-file-manifest.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/performance-and-output-size.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/real-consumer-proof.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/required-reading-map.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/source-completeness.md`

The reopened write set is confined to the accepted corrections: transactional
publication, schema-v2 water closure, writer-boundary validation, declarative
run-file provenance, performance evidence, and their direct authority/test
bindings. It does not modify erosion adoption or open Topanga outcomes.

## Production and serialized surface (18)

- `Cargo.toml`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/subhourly_generation.rs`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-output/src/contracts.rs`
- `crates/openwepp-hillslope-output/src/hillslope_wat_subhourly.rs`
- `crates/openwepp-hillslope-output/src/lib.rs`
- `crates/openwepp-hillslope-output/src/writers.rs`
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `crates/openwepp-runner/src/hillslope/04_direct_publication.rs`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- `crates/openwepp-runner/src/hillslope/intake_lane_setup/runfile_helpers.rs`
- `crates/openwepp-sim-contract/src/units_mod/output_catalog.rs`

## Contract authority and admission tooling (6)

- `docs/specifications/science-contracts/contracts/SC-OUTPUT-WAT5-001.md`
- `docs/specifications/science-contracts/index.md`
- `tools/release/authority-policy/README.md`
- `tools/release/authority-policy/gate-definitions.json`
- `tools/release/authority-policy/impact-map.json`
- `tools/release/check_science_contract_admission.sh`

## Repository integration tests (6)

- `tests/integration/advisory_linter_authority_contract.rs`
- `tests/integration/hbp_subhourly_exclusion_contract.rs`
- `tests/integration/sim_contract_boundary_unit_registry.rs`
- `tests/integration/subhourly_generation_contract.rs`
- `tests/integration/subhourly_generation_properties.rs`
- `tests/integration/subhourly_water_output_roundtrip.rs`

## Work-package lifecycle, evidence, and tools (56)

- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/package.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/adoption-criteria-preregistration.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/adoption-disposition.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/baseline-output-hash-manifest.json`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/baseline-state-identity.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/calibration-readiness-matrix.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/constitutive-response-study.json`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/constitutive-response-study.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/current-consumer-map.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/cutover-contract-evidence.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/diagnostic-contract-evidence.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/diagnostic-contract-test-evidence.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/disposition.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/exponent-authority.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/feasibility-protocol.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/finding-disposition.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/five-minute-water-closure.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/gate-results.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/hold-legitimacy-audit.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/implementation-test-evidence.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/intent-and-base.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/kernel-profile-compliance.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/line-count-governance.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/no-coupling-byte-identity.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/old-path-negative-proof.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/operand-lineage.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/owned-file-manifest.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/performance-and-output-size.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/pre-cutover-contract-gate.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/pre-implementation-diagnostic-contract-gate.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/prerequisite-authority-gate.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/real-consumer-proof.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/reduction-selection.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/required-reading-map.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/resolution-and-phase-sensitivity.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/review_agent_a.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/review_agent_b.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/rust_code_review.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/rust_qa_review.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/source-completeness.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/terminal-auth11.log`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/terminal-doctests-post-a0.log`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/terminal-doctests.log`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/terminal-full-nextest-post-a0.log`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/terminal-full-nextest.log`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/topanga-cutover-study.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/topanga-cutover-summary.json`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/topanga-diagnostic-study.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/topanga-diagnostic-summary.json`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/topanga-plan-identity.json`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/verification_agent_a.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/verification_agent_b.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/tools/README.md`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/tools/feasibility_study.py`
- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/tools/test_feasibility_study.py`

## Shared catalog (1)

- `docs/work-packages/README.md` (the WAT5 lifecycle row; predecessor-owned
  catalog hunks are reconciled by the predecessor package)

The executable-semantic changes are the optional WAT5 ledger/output, its
runner wiring, tests, unit registry, and direct-admission corrections. Those
corrections include multi-token contract IDs, tracked/untracked `--worktree`
admission with a complete authority-input fingerprint, validation of every
atomic authority on shared paths, 17 WAT5 impact-map bindings, the WAT5 A1
definition, and their contract regression tests. The prior public serialized
surface was `openwepp-hillslope-wat-subhourly-v1.0`; the reopened correction
supersedes it with storage-aware `openwepp-hillslope-wat-subhourly-v2.0`. HBP and routing files are not changed;
the new exclusion test proves those consumers do not read WAT5. Green-Ampt
equations and coefficients, WB16 peak authority, and the production erosion
selector are unchanged.

Sorted terminal `git status --porcelain` paths, after excluding the separately
owned predecessor/backlog closure paths, matched this manifest 87-for-87. The
full working-source gate covers production/test/contract semantics. Later
lifecycle-only verifier and catalog edits are rechecked with lightweight
documentation, JSON, admission, and whitespace gates.
