# 20260607-refactor012-openwepp-watershed-orchestrator-runtime-inputs-mechanical-modularization-001

## Status
- state: complete
- date: 2026-06-07
- timezone: UTC

## Objective
Mechanically modularize `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
into cohesive runtime-input adaptation modules while preserving API behavior,
typed guard semantics, and existing contract/test outcomes.

## Why This Package Exists
`crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs` is currently a
monolithic mixed-concern file at 4330 lines, which exceeds `.rs` 3000+
governance and blocks closure without decomposition (absent an approved
generated/fixture exception). This package reduces review and maintenance risk
by splitting runtime-input adaptation concerns into coherent modules without
intended semantic drift.

## Scope
### Included
- Mechanical movement from
  `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs` into dedicated
  module files under
  `crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/`.
- Conversion of `runtime_inputs.rs` into a thin facade/module wiring entrypoint.
- Module-aware updates to layout-coupled tests only when required to preserve
  assertion intent.
- Validation and evidence updates demonstrating no intended behavior changes.

### Explicitly Out of Scope
- New process-physics logic or contract-authority behavior changes.
- Threshold/guard loosening or canonicalize-and-proceed behavior.
- Public API changes unless explicitly declared and approved.

## Deliverables
1. Mechanical modularization implementation with preserved API and behavior:
   - `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
   - `crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/*.rs`
2. Test updates only when required by module-residency assumptions.
3. Work-package artifacts:
   - `artifacts/refactor012-modularization-plan-report.md`
   - `artifacts/refactor012-public-api-surface-parity-report.md`
   - `artifacts/refactor012-contract-implementation-evidence.md`
   - `artifacts/refactor012-contract-test-implementation-evidence.md`
   - `artifacts/refactor012-preimplementation-contract-gate.md`
   - `artifacts/refactor012-implementation-and-test-evidence.md`
   - `artifacts/refactor012-kernel-profile-compliance-checklist.md`
   - `artifacts/refactor012-line-count-governance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/refactor012_disposition.md`
   - `artifacts/worker-handoff.md`
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Autonomous Execution Intent (Required)
This package is execution-ready and must proceed end-to-end through final
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
- `/workdir/openWEPP/crates/openwepp-watershed-orchestrator/Cargo.toml`
- `/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`

## Intended Write Set
- `docs/work-packages/20260607-refactor012-openwepp-watershed-orchestrator-runtime-inputs-mechanical-modularization-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/*.rs`

## Phase Plan
### Phase A - Intake, Sizing, and Surface Freeze
- Capture pre-refactor symbol/API inventory and line-count baseline.
- Freeze split boundaries by runtime-input adaptation concern.

### Phase B - Mechanical Extraction
- Move cohesive runtime-input adaptation sections into dedicated module files.
- Preserve signatures, visibility, and contract citation continuity.

### Phase C - Validation and Evidence
- Run required validation gates and record truthful outputs.
- Complete dual review and dual verification artifacts.

### Phase D - Disposition
- Publish final disposition, parity result, and residual-risk ownership.

## Contract-First Sequencing Requirement
Contract-first sequence remains mandatory for kernel-adjacent package posture:
1. canonical contract amendments,
2. contract-derived tests,
3. pre-implementation contract gate,
4. production edits.

For this package, no canonical contract amendments are expected because this
is mechanical decomposition only with no intended behavior changes. Artifacts
must explicitly record this determination before production edits.

## Exit Criteria
- `runtime_inputs.rs` decomposition is complete with preserved API and behavior
  intent.
- `runtime_inputs.rs` is reduced below 3000 lines.
- Required gates pass:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test -p openwepp-watershed-orchestrator --tests`
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
- rationale: internal Rust module organization refactor with no new external
  interface.
