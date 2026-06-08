# 20260608-refactor015-openwepp-hillslope-orchestrator-hydrology-kernel-phases-mechanical-modularization-001

## Status
- state: complete-with-external-blocker
- date: 2026-06-08
- timezone: UTC
- decision: complete-with-external-blocker

## Objective
Mechanically modularize `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
into cohesive hydrology kernel-phase modules while preserving API behavior,
typed guard semantics, and existing contract/test outcomes.

## Why This Package Exists
`crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
is currently a monolithic mixed-concern file at 6996 lines, which exceeds `.rs`
3000+ governance and blocks closure without decomposition (absent an approved
 generated/fixture exception). This package reduces review and maintenance risk
by splitting kernel-phase concerns into coherent modules without intended
semantic drift.

## Scope
### Included
- Mechanical movement from
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
  into dedicated module files under
  `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/`.
- Conversion of `03_kernel_support_01_kernel_phases.rs` into a thin
  facade/module wiring entrypoint.
- Validation and evidence updates demonstrating no intended behavior changes.

### Explicitly Out of Scope
- New process-physics logic or contract-authority behavior changes.
- Threshold/guard loosening or canonicalize-and-proceed handling.
- Public API changes unless explicitly declared and approved.

## Deliverables
1. Mechanical modularization implementation with preserved API and behavior:
   - `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
   - `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/*.rs`
2. Test updates only when required by module-residency assumptions.
3. Work-package artifacts:
   - `artifacts/refactor015-modularization-plan-report.md`
   - `artifacts/refactor015-public-api-surface-parity-report.md`
   - `artifacts/refactor015-contract-implementation-evidence.md`
   - `artifacts/refactor015-contract-test-implementation-evidence.md`
   - `artifacts/refactor015-preimplementation-contract-gate.md`
   - `artifacts/refactor015-implementation-and-test-evidence.md`
   - `artifacts/refactor015-kernel-profile-compliance-checklist.md`
   - `artifacts/refactor015-line-count-governance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/refactor015_disposition.md`
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
- `/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/standards/mechanical-refactor-authoring-guide.md`
- `/workdir/openWEPP/docs/prompt_templates/mechanical-refactor-kickoff-template.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/Cargo.toml`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`

## Intended Write Set
- `docs/work-packages/20260608-refactor015-openwepp-hillslope-orchestrator-hydrology-kernel-phases-mechanical-modularization-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/*.rs`

## Phase Plan
### Phase A - Intake, Sizing, and Surface Freeze
- Captured pre-refactor line count and function inventory.
- Froze split boundaries by hydrology phase concern.

### Phase B - Mechanical Extraction
- Moved cohesive kernel-phase sections into dedicated module files.
- Preserved function signatures, visibility, and dispatch sites.

### Phase C - Validation and Evidence
- Ran required validation gates and recorded outcomes.
- Completed dual review and dual verification artifacts.

### Phase D - Disposition
- Published final disposition.
- Recorded package hold condition due unrelated workspace-level failure.

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
- `03_kernel_support_01_kernel_phases.rs` decomposition is complete with
  preserved API and behavior intent.
- `03_kernel_support_01_kernel_phases.rs` is reduced below 3000 lines.
- Required gates are executed:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test -p openwepp-hillslope-orchestrator --tests`
  4. `cargo test --workspace`
  5. `cargo deny check`
- `cargo test --workspace` currently fails on unrelated `hphys0225` legacy
  contract fixture expectation (`HPHYS0225`), so closeout status is
  `complete-with-external-blocker` pending follow-on fix in that package.
- Required artifacts are complete with truthful `Static`/`Ran` evidence.
- Review findings are fully dispositioned and line-count governance is
  documented.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: internal Rust module organization refactor with no new external
  interface.
