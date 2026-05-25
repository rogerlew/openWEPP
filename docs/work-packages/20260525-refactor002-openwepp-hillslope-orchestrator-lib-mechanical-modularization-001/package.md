# 20260525-refactor002-openwepp-hillslope-orchestrator-lib-mechanical-modularization-001

## Status
- state: queued
- date: 2026-05-25
- timezone: UTC

## Objective
Mechanically modularize `crates/openwepp-hillslope-orchestrator/src/lib.rs`
into cohesive module files while preserving public API behavior, typed guard
semantics, and existing contract/test outcomes.

## Why This Package Exists
`openwepp-hillslope-orchestrator/src/lib.rs` has grown into a single large
mixed-concern file (~13.4k lines) spanning phase graphing, consumer boundary
validation, WB11 hydrology kernel execution, scheduler orchestration, and
extensive crate-local tests. This package reduces maintenance risk by splitting
that file into explicit module boundaries with no intentional behavioral drift.

## Scope
### Included
- Mechanical code movement from
  `crates/openwepp-hillslope-orchestrator/src/lib.rs` into new `src/*.rs`
  modules.
- `lib.rs` conversion to module declarations and public re-exports preserving
  current external API surface.
- Test updates to remove brittle single-file assertions that require all
  implementation text to reside in `src/lib.rs`.
- Validation and evidence updates proving no behavioral drift.

### Explicitly Out of Scope
- New process-physics logic or scheduler behavior changes.
- New parser semantics, output schema changes, or contract authority rewrites.
- Runner/watershed feature expansion unrelated to modularization.

## Deliverables
1. Mechanical modularization implementation with preserved API surface:
   - `crates/openwepp-hillslope-orchestrator/src/lib.rs`
   - `crates/openwepp-hillslope-orchestrator/src/*.rs`
2. Test updates for modularized source layout assumptions:
   - `tests/integration/arch22_typed_state_surface_contract.rs`
   - additional layout-coupled tests only when required
3. Work-package evidence artifacts:
   - `artifacts/refactor002-modularization-plan-report.md`
   - `artifacts/refactor002-public-api-surface-parity-report.md`
   - `artifacts/refactor002-contract-implementation-evidence.md`
   - `artifacts/refactor002-contract-test-implementation-evidence.md`
   - `artifacts/refactor002-preimplementation-contract-gate.md`
   - `artifacts/refactor002-implementation-and-test-evidence.md`
   - `artifacts/refactor002-kernel-profile-compliance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/refactor002_disposition.md`
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
- `/workdir/openWEPP/tests/integration/arch22_typed_state_surface_contract.rs`
- `/workdir/openWEPP/tests/integration/hillslope_consumer_boundary_integration.rs`
- `/workdir/openWEPP/tests/integration/kernel_writeback_contract.rs`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`

## Intended Write Set
- `docs/work-packages/20260525-refactor002-openwepp-hillslope-orchestrator-lib-mechanical-modularization-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/*.rs`
- `tests/integration/arch22_typed_state_surface_contract.rs`
- additional `tests/integration/*.rs` only when required for
  layout-coupled assertion updates

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
- `openwepp-hillslope-orchestrator` modularization is complete with stable
  public API surface.
- Layout-coupled tests are updated and passing.
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
- Rationale: internal Rust module organization refactor and test-surface
  maintenance without new external interfaces.
