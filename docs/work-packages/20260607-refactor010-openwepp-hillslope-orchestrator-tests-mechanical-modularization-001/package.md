# 20260607-refactor010-openwepp-hillslope-orchestrator-tests-mechanical-modularization-001

## Status
- state: queued
- date: 2026-06-07
- timezone: UTC

## Objective
Mechanically modularize `crates/openwepp-hillslope-orchestrator/src/tests.rs`
into cohesive test modules while preserving test intent, typed guard
expectations, and existing contract/test outcomes.

## Why This Package Exists
`crates/openwepp-hillslope-orchestrator/src/tests.rs` is currently a
single mixed-concern test file at 3460 lines, which exceeds the required
refactor threshold for `.rs` files. This package reduces review/maintenance
risk by splitting tests by concern without changing production behavior or
weakening assertions.

## Scope
### Included
- Mechanical movement of tests from
  `crates/openwepp-hillslope-orchestrator/src/tests.rs` into dedicated test
  module files under a `tests_mod/` subtree.
- Conversion of `src/tests.rs` into a thin module wiring entrypoint for split
  test files.
- Module-aware updates to layout-coupled assertions only when required to
  preserve contract fidelity.
- Validation and evidence updates proving no intended behavior changes.

### Explicitly Out of Scope
- New production process-physics logic or scheduler behavior changes.
- Guard loosening, fallback wrappers, or canonicalize-and-proceed handling.
- Contract authority rewrites not required for mechanical decomposition.

## Deliverables
1. Mechanical modularization implementation with preserved behavior expectations:
   - `crates/openwepp-hillslope-orchestrator/src/tests.rs`
   - `crates/openwepp-hillslope-orchestrator/src/tests_mod/*.rs`
2. Test updates for layout-coupled assertions only when required.
3. Work-package artifacts:
   - `artifacts/refactor010-modularization-plan-report.md`
   - `artifacts/refactor010-public-api-surface-parity-report.md`
   - `artifacts/refactor010-contract-implementation-evidence.md`
   - `artifacts/refactor010-contract-test-implementation-evidence.md`
   - `artifacts/refactor010-preimplementation-contract-gate.md`
   - `artifacts/refactor010-implementation-and-test-evidence.md`
   - `artifacts/refactor010-kernel-profile-compliance-checklist.md`
   - `artifacts/refactor010-line-count-governance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/refactor010_disposition.md`
   - `artifacts/worker-handoff.md`
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Autonomous Execution Intent (Required)
This package is execution-ready and must proceed end-to-end through
final disposition without user intervention unless hard-blocked.

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
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/Cargo.toml`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/tests.rs`

## Intended Write Set
- `docs/work-packages/20260607-refactor010-openwepp-hillslope-orchestrator-tests-mechanical-modularization-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-hillslope-orchestrator/src/tests.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests_mod/*.rs`

## Phase Plan
### Phase A - Intake, Sizing, and Test Surface Freeze
- Capture pre-refactor symbol/test inventory and line-count baseline.
- Freeze split boundaries by test concern.

### Phase B - Mechanical Test Extraction
- Move tests into cohesive module files with no intended behavior changes.
- Keep helper visibility and test utilities stable.

### Phase C - Validation and Evidence
- Run required validation gates and record truthful outputs.
- Complete dual review and dual verification artifacts.

### Phase D - Disposition
- Publish final disposition, parity conclusion, and residual-risk ownership.

## Contract-First Sequencing Requirement
Contract-first sequence remains mandatory for kernel-adjacent package posture:
1. canonical contract amendments,
2. contract-derived tests,
3. pre-implementation contract gate,
4. production edits.

For this package, no canonical contract amendments are expected because the
change is mechanical test decomposition only with no intended behavior changes.
Artifacts must explicitly record this determination before production edits.

## Exit Criteria
- Test decomposition is complete with preserved test intent and guard coverage.
- `tests.rs` is reduced below 3000 lines.
- Required gates pass:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test -p openwepp-hillslope-orchestrator --tests`
  4. `cargo test --workspace`
  5. `cargo deny check`
- Gate commands above are mandatory execution requirements; omission is only
  permitted when a hard blocker is recorded with command-level evidence.
- Required artifacts are complete with truthful `Static`/`Ran` evidence.
- Review findings are fully dispositioned and line-count governance is
  documented.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: internal test module organization refactor with no new external
  interface.
