# 20260607-refactor006-openwepp-runner-hillslope-mod-mechanical-modularization-001

## Status
- state: package-complete
- date: 2026-06-08
- timezone: UTC
- decision: GO

## Objective
Mechanically modularize `crates/openwepp-runner/src/hillslope/mod.rs` into
cohesive module files while preserving public API behavior, typed guard
semantics, and existing contract/test outcomes.

## Why This Package Exists
`crates/openwepp-runner/src/hillslope/mod.rs` currently holds mixed runner
concerns in a single large file (~11.2k lines), including runtime projection,
hourly/daily trace publication, WB13 output emission, and test-coupled source
surfaces. This package reduces maintenance and review friction by splitting the
file into explicit module boundaries without behavioral or contract drift.

## Scope
### Included
- Mechanical code movement from
  `crates/openwepp-runner/src/hillslope/mod.rs` into new
  `crates/openwepp-runner/src/hillslope/*.rs` modules.
- `crates/openwepp-runner/src/hillslope/mod.rs` conversion to module
  declarations and re-exports that preserve the current API/runtime boundary.
- Test updates for layout-coupled assertions that currently require key logic
  text to remain in one monolithic file.
- Validation and evidence updates proving no behavioral drift.

### Explicitly Out of Scope
- New process-physics logic or scheduler behavior changes.
- New parser semantics, output schema changes, or contract authority rewrites.
- Watershed or climate feature expansion unrelated to mechanical modularization.

## Deliverables
1. Mechanical modularization implementation with preserved API surface:
   - `crates/openwepp-runner/src/hillslope/mod.rs`
   - `crates/openwepp-runner/src/hillslope/*.rs`
2. Test updates for modularized source layout assumptions:
   - `tests/integration/cli03_runner_contract_derived_tests.rs`
   - `tests/integration/hphys*_contract.rs` files that assert monolithic
     `hillslope/mod.rs` residency
3. Work-package evidence artifacts:
   - `artifacts/refactor006-modularization-plan-report.md`
   - `artifacts/refactor006-public-api-surface-parity-report.md`
   - `artifacts/refactor006-contract-implementation-evidence.md`
   - `artifacts/refactor006-contract-test-implementation-evidence.md`
   - `artifacts/refactor006-preimplementation-contract-gate.md`
   - `artifacts/refactor006-implementation-and-test-evidence.md`
   - `artifacts/refactor006-kernel-profile-compliance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/refactor006_disposition.md`
   - `artifacts/worker-handoff.md`
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Autonomous Execution Intent (Required)
This package executed end-to-end through disposition without user intervention.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:` sections.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/crates/openwepp-runner/Cargo.toml`
- `/workdir/openWEPP/crates/openwepp-runner/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-runner/src/hillslope/mod.rs`
- `/workdir/openWEPP/crates/openwepp-runner/src/bin/open_wepp_runner.rs`
- `/workdir/openWEPP/crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`
- `/workdir/openWEPP/tests/integration/cli01_runner_hillslope_integration.rs`
- `/workdir/openWEPP/tests/integration/cli03_runner_contract_derived_tests.rs`
- `/workdir/openWEPP/tests/integration/cli04_runner_wat_parquet_contract_derived_tests.rs`
- `/workdir/openWEPP/tests/integration/hparity02_profile_capacity_parity_contract.rs`
- `/workdir/openWEPP/tests/integration/hphys0289_wb13_rm_snowwater_publication_contract.rs`
- `/workdir/openWEPP/tests/integration/hphys0290_post_winter_rain_publication_contract.rs`
- `/workdir/openWEPP/tests/integration/hphys0291_snow_publication_lifecycle_contract.rs`
- `/workdir/openWEPP/tests/integration/hphys0293_winter_melt_timing_contract.rs`
- `/workdir/openWEPP/tests/integration/hphys0294_post_ingress_storage_retention_contract.rs`
- `/workdir/openWEPP/tests/integration/hphys0295_cumulative_storage_budget_contract.rs`
- `/workdir/openWEPP/tests/integration/hphys0296_snow_rm_acceptance_authority_contract.rs`
- `/workdir/openWEPP/tests/integration/hphys0299_hourly_snow_partition_unit_provenance_contract.rs`
- `/workdir/openWEPP/tests/integration/hphys0305_paired_melt_term_state_contract.rs`
- `/workdir/openWEPP/tests/integration/hphys0318_stmtim_control_surface_instrumentation_contract.rs`
- `/workdir/openWEPP/docs/contracts/openwepp-runner-contract.md`
- `/workdir/openWEPP/docs/contracts/openwepp-hillslope-runfile-contract.md`
- `/workdir/openWEPP/docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`

## Intended Write Set
- `docs/work-packages/20260607-refactor006-openwepp-runner-hillslope-mod-mechanical-modularization-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-runner/src/hillslope/*.rs`
- `tests/integration/cli03_runner_contract_derived_tests.rs`
- `tests/integration/hparity02_profile_capacity_parity_contract.rs`
- `tests/integration/hphys0289_wb13_rm_snowwater_publication_contract.rs`
- `tests/integration/hphys0290_post_winter_rain_publication_contract.rs`
- `tests/integration/hphys0291_snow_publication_lifecycle_contract.rs`
- `tests/integration/hphys0293_winter_melt_timing_contract.rs`
- `tests/integration/hphys0294_post_ingress_storage_retention_contract.rs`
- `tests/integration/hphys0295_cumulative_storage_budget_contract.rs`
- `tests/integration/hphys0296_snow_rm_acceptance_authority_contract.rs`
- `tests/integration/hphys0299_hourly_snow_partition_unit_provenance_contract.rs`
- `tests/integration/hphys0305_paired_melt_term_state_contract.rs`
- `tests/integration/hphys0318_stmtim_control_surface_instrumentation_contract.rs`

## Phase Plan
### Phase A - Intake and API Surface Freeze
- Capture current exported symbols and known external callsites.
- Freeze module boundaries and extraction order.

### Phase B - Mechanical Module Extraction
- Create target `hillslope/*.rs` module files.
- Move code blocks into modules with no intended behavior change.
- Preserve visibility and API surface through re-exports.

### Phase C - Layout-Coupled Test Update
- Update brittle source-residency assertions to behavior/API oriented checks
  or module-tree aware checks.

### Phase D - Validation and Evidence
- Run required validation gates and record truthful outputs.
- Complete governance artifacts and dual review/verification.

### Phase E - Disposition
- Publish final disposition with API parity and residual risk notes.

## Contract-First Sequencing Requirement
Contract sequencing is applicable as a gate posture for kernel-adjacent runtime
projection work:
1. contract amendments,
2. contract-derived tests,
3. pre-implementation contract gate,
4. production code edits.

For this package, no canonical contract amendments are expected because the
change is mechanical source modularization only with no intended behavior
change. Evidence artifacts must explicitly record this before code edits.

## Exit Criteria
- `hillslope/mod.rs` decomposition is complete with stable public API surface.
- Layout-coupled tests are updated and passing.
- Required gates pass:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test -p openwepp-runner --tests`
  4. `cargo test --workspace`
  5. `cargo deny check`
- Required artifacts are complete with truthful `Static`/`Ran` evidence.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: internal Rust module organization refactor and test-surface
  maintenance without new external interfaces.
