# 20260525-refactor001-openwepp-runner-lib-mechanical-modularization-001

## Status
- state: queued
- date: 2026-05-25
- timezone: UTC

## Objective
Mechanically modularize `crates/openwepp-runner/src/lib.rs` into cohesive
module files while preserving public API behavior, guard semantics, and
existing contract/test outcomes.

## Why This Package Exists
`openwepp-runner/src/lib.rs` has grown into a single large mixed-concern file
(~4.2k lines) spanning release metadata, runner launch logic, hillslope runfile
intake, sidecar handling, runtime seeding, scheduler execution, WB13
publication, and fixture-style internal tests. This package reduces maintenance
risk by splitting the file into explicit module boundaries without
behavioral/contract drift.

## Scope
### Included
- Mechanical code movement from `crates/openwepp-runner/src/lib.rs` into new
  `src/*.rs` and `src/hillslope/*.rs` modules.
- `lib.rs` conversion to module declarations and public re-exports preserving
  current external API surface.
- Test updates to remove brittle single-file assertions that require all
  implementation text to reside in `src/lib.rs`.
- Validation and evidence updates proving no behavioral drift.

### Explicitly Out of Scope
- New process-physics logic or scheduler behavior changes.
- New parser semantics, output schema changes, or contract authority rewrites.
- Watershed CLI feature expansion unrelated to modularization.

## Deliverables
1. Mechanical modularization implementation with preserved API surface:
   - `crates/openwepp-runner/src/lib.rs`
   - `crates/openwepp-runner/src/*.rs`
   - `crates/openwepp-runner/src/hillslope/*.rs`
2. Test updates for modularized source layout assumptions:
   - `tests/integration/cli03_runner_contract_derived_tests.rs` (and any other
     layout-coupled tests as required)
3. Work-package evidence artifacts:
   - `artifacts/refactor001-modularization-plan-report.md`
   - `artifacts/refactor001-public-api-surface-parity-report.md`
   - `artifacts/refactor001-contract-implementation-evidence.md`
   - `artifacts/refactor001-contract-test-implementation-evidence.md`
   - `artifacts/refactor001-preimplementation-contract-gate.md`
   - `artifacts/refactor001-implementation-and-test-evidence.md`
   - `artifacts/refactor001-kernel-profile-compliance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/refactor001_disposition.md`
   - `artifacts/worker-handoff.md`
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Autonomous Execution Intent (Required)
This package is execution-ready and must proceed end-to-end without user
intervention unless hard-blocked by contradictory canonical requirements or
unresolvable environment failures.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:` sections.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/crates/openwepp-runner/Cargo.toml`
- `/workdir/openWEPP/crates/openwepp-runner/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-runner/src/bin/open_wepp_runner.rs`
- `/workdir/openWEPP/crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`
- `/workdir/openWEPP/crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `/workdir/openWEPP/tests/integration/cli01_runner_hillslope_integration.rs`
- `/workdir/openWEPP/tests/integration/cli03_runner_contract_derived_tests.rs`
- `/workdir/openWEPP/tests/integration/cli04_runner_wat_parquet_contract_derived_tests.rs`
- `/workdir/openWEPP/tests/integration/pl14s_tier_a_candidate_emission_and_replay_contract.rs`
- `/workdir/openWEPP/crates/openwepp-runner/tests/`
- `/workdir/openWEPP/docs/contracts/openwepp-runner-contract.md`
- `/workdir/openWEPP/docs/contracts/openwepp-hillslope-runfile-contract.md`
- `/workdir/openWEPP/docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md`

## Intended Write Set
- `docs/work-packages/20260525-refactor001-openwepp-runner-lib-mechanical-modularization-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-runner/src/lib.rs`
- `crates/openwepp-runner/src/*.rs`
- `crates/openwepp-runner/src/hillslope/*.rs`
- `tests/integration/cli03_runner_contract_derived_tests.rs`
- additional `crates/openwepp-runner/tests/*.rs` or `tests/integration/*.rs`
  only when required for layout-coupled assertion updates

## Phase Plan
### Phase A - Intake and API Surface Freeze
- Capture current exported symbols and known external callsites.
- Freeze modularization boundaries and migration order.

### Phase B - Mechanical Module Extraction
- Create target module files.
- Move code blocks into modules with no intended behavior change.
- Preserve visibility and re-export surface.

### Phase C - Test Surface Update
- Update layout-coupled tests to assert contract/API behavior rather than
  single-file text residency.

### Phase D - Validation and Evidence
- Run required gates and record outputs.
- Complete governance artifacts and dual review/verification.

### Phase E - Disposition
- Publish final disposition with API parity and residual risk notes.

## Exit Criteria
- `openwepp-runner` modularization is complete with stable public API surface.
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
- Rationale: internal Rust module organization refactor and test-surface
  maintenance without new external interfaces.
