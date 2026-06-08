# 20260608-refactor016-openwepp-kernel-contract-lib-mechanical-modularization-001

## Status
- state: queued
- date: 2026-06-08
- timezone: UTC

## Objective
Mechanically modularize `crates/openwepp-kernel-contract/src/lib.rs` into
cohesive kernel-contract modules while preserving API behavior, typed contract
surface semantics, and existing test outcomes.

## Why This Package Exists
`crates/openwepp-kernel-contract/src/lib.rs` is currently a monolithic mixed-
concern file at 2044 lines, exceeding the `.rs` 2000+ warning threshold. This
package reduces maintenance and review risk by splitting concerns into coherent
modules without intended semantic drift.

## Scope
### Included
- Mechanical movement from `crates/openwepp-kernel-contract/src/lib.rs` into
  dedicated module files under `crates/openwepp-kernel-contract/src/lib_mod/`.
- Conversion of `lib.rs` into a thin facade/module wiring entrypoint.
- Validation and evidence updates demonstrating no intended behavior changes.

### Explicitly Out of Scope
- New process-physics logic or contract-authority behavior changes.
- Threshold/guard loosening or canonicalize-and-proceed handling.
- Public API changes unless explicitly declared and approved.

## Deliverables
1. Mechanical modularization implementation with preserved API and behavior:
   - `crates/openwepp-kernel-contract/src/lib.rs`
   - `crates/openwepp-kernel-contract/src/lib_mod/*.rs`
2. Test updates only when required by module-residency assumptions.
3. Work-package artifacts:
   - `artifacts/required-reading-map.md`
   - `artifacts/refactor016-modularization-plan-report.md`
   - `artifacts/refactor016-public-api-surface-parity-report.md`
   - `artifacts/refactor016-contract-implementation-evidence.md`
   - `artifacts/refactor016-contract-test-implementation-evidence.md`
   - `artifacts/refactor016-preimplementation-contract-gate.md`
   - `artifacts/refactor016-implementation-and-test-evidence.md`
   - `artifacts/refactor016-kernel-profile-compliance-checklist.md`
   - `artifacts/refactor016-line-count-governance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/refactor016_disposition.md`
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
- `/workdir/openWEPP/docs/prompt_templates/required-reading-map-template.md`
- `/workdir/openWEPP/docs/standards/kernel-work-package-preparation.md`
- `/workdir/openWEPP/crates/openwepp-kernel-contract/Cargo.toml`
- `/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs`

## Intended Write Set
- `docs/work-packages/20260608-refactor016-openwepp-kernel-contract-lib-mechanical-modularization-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-kernel-contract/src/lib.rs`
- `crates/openwepp-kernel-contract/src/lib_mod/*.rs`

## Phase Plan
### Phase A - Intake, Sizing, and Surface Freeze
- Capture pre-refactor symbol/API inventory and line-count baseline.
- Freeze split boundaries by kernel-contract concern.

### Phase B - Mechanical Extraction
- Move cohesive contract surfaces into dedicated module files.
- Preserve signatures, visibility, and exported surface behavior.

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
- `lib.rs` decomposition is complete with preserved API and behavior intent.
- `lib.rs` is reduced below 2000 lines.
- Required gates pass:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test -p openwepp-kernel-contract --tests`
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
