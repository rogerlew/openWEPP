# 20260525-refactor004-openwepp-hillslope-orchestrator-runtime-inputs-mechanical-modularization-001

## Status
- state: package-complete
- date: 2026-05-25
- timezone: UTC

## Objective
Mechanically modularize
`crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs` into cohesive
module files while preserving public API behavior, typed guard semantics, and
existing contract/test outcomes.

## Why This Package Exists
`runtime_inputs.rs` has grown into a single very large mixed-concern file
(~5.9k lines) spanning parser-to-runtime projection for management, soil,
slope, climate, snow/frost controls, irrigation schedules, SIMIMPL28 hourly
winter forcing synthesis, and extensive module-level tests. This package
reduces maintenance risk and review friction by splitting the file into explicit
source boundaries with no intended semantic drift.

## Scope
### Included
- Mechanical code movement from
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs` into
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/*.rs`.
- Conversion from single-file runtime-inputs module to
  `runtime_inputs/mod.rs` plus section files.
- Validation and evidence updates proving no behavioral drift.

### Explicitly Out of Scope
- New process-physics logic or parser semantics changes.
- New runtime symbol families or output schema updates.
- Contract authority rewrites unrelated to modularization.

## Deliverables
1. Mechanical modularization implementation with preserved API surface:
   - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/mod.rs`
   - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/*.rs`
   - removal of `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
2. Work-package evidence artifacts:
   - `artifacts/refactor004-modularization-plan-report.md`
   - `artifacts/refactor004-public-api-surface-parity-report.md`
   - `artifacts/refactor004-contract-implementation-evidence.md`
   - `artifacts/refactor004-contract-test-implementation-evidence.md`
   - `artifacts/refactor004-preimplementation-contract-gate.md`
   - `artifacts/refactor004-implementation-and-test-evidence.md`
   - `artifacts/refactor004-kernel-profile-compliance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/refactor004_disposition.md`
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
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/Cargo.toml`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs`
- `/workdir/openWEPP/tests/integration/clim05_snow_runtime_kernel_contract.rs`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`

## Intended Write Set
- `docs/work-packages/20260525-refactor004-openwepp-hillslope-orchestrator-runtime-inputs-mechanical-modularization-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/mod.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/*.rs`

## Phase Plan
### Phase A - Intake and API Surface Freeze
- Capture current exported symbols and module wiring.
- Freeze section boundaries and migration order.

### Phase B - Mechanical Module Extraction
- Create target runtime-inputs section files.
- Move code blocks into files with no intended behavior change.
- Preserve visibility and API surface.

### Phase C - Validation Surface Check
- Confirm no integration tests depend on single-file residency.
- Update layout-coupled paths only where required by file relocation.

### Phase D - Validation and Evidence
- Run required gates and record outputs.
- Complete governance artifacts and dual review/verification.

### Phase E - Disposition
- Publish final disposition with API parity and residual risk notes.

## Contract-First Sequencing Requirement
Contract sequencing is applicable as a gate posture for kernel-affecting work:
1. contract amendments,
2. contract-derived tests,
3. pre-implementation contract gate,
4. production code edits.

For this package, no canonical contract amendments are required because the
change is mechanical source modularization only with no intended behavior
change. Evidence artifacts must still record this explicitly before code edits.

## Exit Criteria
- Runtime-inputs module split is complete with stable public API surface.
- No intentional runtime semantic changes introduced.
- Required gates pass:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test -p openwepp-hillslope-orchestrator`
  4. `cargo test --workspace`
  5. `cargo deny check`
- Required artifacts are complete with truthful `Static`/`Ran` evidence.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: internal Rust module organization refactor without new external
  interfaces.
