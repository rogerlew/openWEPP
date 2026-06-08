# 20260607-refactor007-openwepp-runner-hillslope-01-scheduler-and-trace-mechanical-modularization-001

## Status
- state: complete
- date: 2026-06-07
- timezone: UTC

Disposition date: 2026-06-08 UTC
Disposition decision: GO

## Objective
Mechanically modularize `crates/openwepp-runner/src/hillslope/01_scheduler_and_trace.rs`
into cohesive scheduler/trace modules while preserving behavior, public API
surface, typed guard semantics, and existing contract/test outcomes.

## Why This Package Exists
`crates/openwepp-runner/src/hillslope/01_scheduler_and_trace.rs` is currently a
single mixed-concern file at 3156 lines, which exceeds the required-refactor
threshold for `.rs` files and increases review/maintenance risk. This package
creates explicit internal seams for scheduler setup, runtime seeding, and trace
publication logic without introducing intended runtime semantic drift.

## Scope
### Included
- Mechanical code movement from
  `crates/openwepp-runner/src/hillslope/01_scheduler_and_trace.rs` into new
  module files under a scheduler/trace subtree.
- Conversion of `01_scheduler_and_trace.rs` into a thin wiring/facade module
  that preserves consumed symbol visibility.
- Layout-coupled test updates only when needed to keep contract assertions
  resilient to file decomposition.
- Validation and evidence updates proving no intentional behavior changes.

### Explicitly Out of Scope
- New process-physics logic, scheduler semantics, or climate behavior changes.
- Guard loosening, fallback wrappers, or canonicalize-and-proceed handling.
- Contract authority rewrites not required for mechanical decomposition.

## Deliverables
1. Mechanical modularization implementation with preserved behavior and API
   parity:
   - `crates/openwepp-runner/src/hillslope/01_scheduler_and_trace.rs`
   - `crates/openwepp-runner/src/hillslope/scheduler_trace/*.rs`
2. Test updates for any layout-coupled source-residency assertions (only when
   required).
3. Work-package artifacts:
   - `artifacts/refactor007-modularization-plan-report.md`
   - `artifacts/refactor007-public-api-surface-parity-report.md`
   - `artifacts/refactor007-contract-implementation-evidence.md`
   - `artifacts/refactor007-contract-test-implementation-evidence.md`
   - `artifacts/refactor007-preimplementation-contract-gate.md`
   - `artifacts/refactor007-implementation-and-test-evidence.md`
   - `artifacts/refactor007-kernel-profile-compliance-checklist.md`
   - `artifacts/refactor007-line-count-governance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/refactor007_disposition.md`
   - `artifacts/worker-handoff.md`
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Autonomous Execution Intent (Required)
This package is execution-ready and must proceed end-to-end through
disposition without user intervention unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:` sections.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/standards/mechanical-refactor-authoring-guide.md`
- `/workdir/openWEPP/docs/prompt_templates/mechanical-refactor-kickoff-template.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/crates/openwepp-runner/Cargo.toml`
- `/workdir/openWEPP/crates/openwepp-runner/src/hillslope/mod.rs`
- `/workdir/openWEPP/crates/openwepp-runner/src/hillslope/01_scheduler_and_trace.rs`
- `/workdir/openWEPP/crates/openwepp-runner/src/bin/open_wepp_runner.rs`
- `/workdir/openWEPP/crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`
- `/workdir/openWEPP/tests/integration/cli03_runner_contract_derived_tests.rs`
- `/workdir/openWEPP/tests/integration/hphys0318_stmtim_control_surface_instrumentation_contract.rs`

## Intended Write Set
- `docs/work-packages/20260607-refactor007-openwepp-runner-hillslope-01-scheduler-and-trace-mechanical-modularization-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-runner/src/hillslope/01_scheduler_and_trace.rs`
- `crates/openwepp-runner/src/hillslope/scheduler_trace/*.rs`
- `tests/integration/cli03_runner_contract_derived_tests.rs` (if required)
- additional `tests/integration/*.rs` only when layout-coupled assertions
  require module-aware updates

## Phase Plan
### Phase A - Intake, Sizing, and API Freeze
- Capture pre-refactor symbol inventory and callsites.
- Capture current line-count baseline and split target plan.

### Phase B - Mechanical Extraction by Concern
- Extract cohesive scheduler/trace seams into dedicated modules.
- Preserve signatures/visibility and typed guard behavior.

### Phase C - Test Surface Stabilization
- Update only brittle layout-coupled tests to module-aware behavior checks.

### Phase D - Validation and Evidence
- Run required validation gates and record truthful outputs.
- Complete dual review and dual verification artifacts.

### Phase E - Disposition
- Publish final disposition, parity conclusion, and residual risk ownership.

## Contract-First Sequencing Requirement
Contract-first sequence remains mandatory for kernel-adjacent package posture:
1. canonical contract amendments,
2. contract-derived tests,
3. pre-implementation contract gate,
4. production edits.

For this package, no canonical contract amendments are expected because the
change is mechanical decomposition only with no intended behavior changes.
Artifacts must explicitly record this determination before production edits.

## Exit Criteria
- Scheduler/trace decomposition is complete with preserved API surface.
- `01_scheduler_and_trace.rs` is reduced below 3000 lines.
- Required gates pass:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test -p openwepp-runner --tests`
  4. `cargo test --workspace`
  5. `cargo deny check`
- Required artifacts are complete with truthful `Static`/`Ran` evidence.
- Review findings are fully dispositioned and line-count governance is
  documented.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: internal Rust module organization refactor with no new external
  interface.
