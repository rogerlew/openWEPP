# 20260608-refactor019-openwepp-hillslope-orchestrator-hydrology-phase-storage-erosion-mechanical-modularization-001

## Status
- state: queued
- date: 2026-06-08
- timezone: UTC

## Objective
Mechanically modularize
`crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion.rs`
into cohesive modules while preserving public API intent, guard semantics, and all
consumer-facing behavior.

## Why This Package Exists
`crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion.rs`
is a 2110-line mixed-concern implementation, which exceeds the `.rs` 2000+ warning
threshold for required structural refactors. The function cluster spans storage
reconciliation orchestration, flux/state validation, solver-bridge behavior, and
helper utilities. A focused refactor lowers review risk and enables domain-local
maintenance without intended semantic changes.

## Scope
### Included
- Move `hydrology_phase_storage_erosion.rs` to a thin facade entrypoint under
  `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/mod.rs`.
- Split cohesive concern groups into additional files under
  `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/`.
- Preserve public method signatures, visibility, `#[allow]`/`#[cfg]` attributes,
  and observable side effects.
- Validation and evidence updates demonstrating no intended behavioral drift.

### Explicitly Out of Scope
- New hydrology algorithms or formula changes.
- Contract-authority changes or guard loosening.
- Changes outside the declared `hydrology_phase_storage_erosion` seam and target
  write set.
- New semantic cleanup outside the modularization objective.

## Deliverables
1. Mechanical modularization implementation with preserved API intent:
   - `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/mod.rs`
   - `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/*.rs`
2. Work-package artifacts:
   - `artifacts/required-reading-map.md`
   - `artifacts/refactor019-modularization-plan-report.md`
   - `artifacts/refactor019-public-api-surface-parity-report.md`
   - `artifacts/refactor019-contract-implementation-evidence.md`
   - `artifacts/refactor019-contract-test-implementation-evidence.md`
   - `artifacts/refactor019-preimplementation-contract-gate.md`
   - `artifacts/refactor019-implementation-and-test-evidence.md`
   - `artifacts/refactor019-kernel-profile-compliance-checklist.md`
   - `artifacts/refactor019-line-count-governance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/refactor019_disposition.md`
   - `artifacts/worker-handoff.md`
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Autonomous Execution Intent (Required)
This package is execution-ready and must proceed end-to-end through final
closure without user intervention unless hard-blocked.

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
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/Cargo.toml`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/hydrology/mod.rs`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion.rs`

## Intended Write Set
- `docs/work-packages/20260608-refactor019-openwepp-hillslope-orchestrator-hydrology-phase-storage-erosion-mechanical-modularization-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/mod.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/*.rs`

## Phase Plan
### Phase A - Intake, Sizing, and Surface Freeze
- Capture pre-refactor line count and symbol inventory.
- Confirm module seam and call-site assumptions in `src/hydrology/mod.rs` and
  `kernel_phases_mod/mod.rs`.
- Freeze module split boundaries.

### Phase B - Mechanical Extraction
- Convert `hydrology_phase_storage_erosion.rs` into a thin facade module.
- Extract cohesive helpers to dedicated files under `kernel_phases_mod/`.
- Preserve all public and shared-private symbols required by existing imports.

### Phase C - Validation and Evidence
- Run required validation gates and record outputs.
- Capture post-refactor public API parity and line-count posture.
- Complete dual review and dual verification artifacts.

### Phase D - Disposition
- Publish final disposition and residual-risk ownership.

## Contract-First Sequencing Requirement
Kernel-adjacent work remains contract-first by process posture:
1. canonical contract amendments,
2. contract-derived tests,
3. pre-implementation contract gate,
4. production edits.

This mechanical package does not change contracts or rules; artifacts must
explicitly record this determination before edits.

## Exit Criteria
- `hydrology_phase_storage_erosion.rs` split into `kernel_phases_mod/mod.rs` plus
  supporting files while preserving module intent.
- No behavior changes intended or introduced by this package.
- All required gates pass:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test -p openwepp-hillslope-orchestrator --tests`
  4. `cargo test --workspace`
  5. `cargo deny check`
- Required artifacts are complete with truthful `Static`/`Ran` evidence.
- Review findings are fully dispositioned and line-count governance is documented.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: internal Rust module organization refactor in the hydrology orchestrator;
  no external interface changes and no new trust boundaries.
