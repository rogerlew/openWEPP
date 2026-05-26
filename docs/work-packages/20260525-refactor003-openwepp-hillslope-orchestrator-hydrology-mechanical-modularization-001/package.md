# 20260525-refactor003-openwepp-hillslope-orchestrator-hydrology-mechanical-modularization-001

## Status
- state: package-complete
- date: 2026-05-25
- timezone: UTC

## Objective
Mechanically modularize
`crates/openwepp-hillslope-orchestrator/src/hydrology.rs` into cohesive module
files while preserving public API behavior, typed guard semantics, and existing
contract/test outcomes.

## Why This Package Exists
`openwepp-hillslope-orchestrator/src/hydrology.rs` has grown into a single very
large mixed-concern file (~9.2k lines) spanning PL dispatch resolution,
hydrology routing, WB11 guard surfaces, runtime solver helpers, growth/decomp
state extraction, and decomposition equation payload assembly. This package
reduces maintenance risk by splitting that file into explicit file boundaries
with no intentional behavioral drift.

## Scope
### Included
- Mechanical code movement from
  `crates/openwepp-hillslope-orchestrator/src/hydrology.rs` into
  `crates/openwepp-hillslope-orchestrator/src/hydrology/*.rs`.
- Conversion from single-file hydrology module to `hydrology/mod.rs` plus
  section files.
- Validation and evidence updates proving no behavioral drift.

### Explicitly Out of Scope
- New process-physics logic or scheduler behavior changes.
- New parser semantics, output schema changes, or contract authority rewrites.
- Runner/watershed feature expansion unrelated to modularization.

## Deliverables
1. Mechanical modularization implementation with preserved API surface:
   - `crates/openwepp-hillslope-orchestrator/src/hydrology/mod.rs`
   - `crates/openwepp-hillslope-orchestrator/src/hydrology/*.rs`
   - removal of `crates/openwepp-hillslope-orchestrator/src/hydrology.rs`
2. Work-package evidence artifacts:
   - `artifacts/refactor003-modularization-plan-report.md`
   - `artifacts/refactor003-public-api-surface-parity-report.md`
   - `artifacts/refactor003-contract-implementation-evidence.md`
   - `artifacts/refactor003-contract-test-implementation-evidence.md`
   - `artifacts/refactor003-preimplementation-contract-gate.md`
   - `artifacts/refactor003-implementation-and-test-evidence.md`
   - `artifacts/refactor003-kernel-profile-compliance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/refactor003_disposition.md`
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
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/hydrology.rs`
- `/workdir/openWEPP/tests/integration/arch22_typed_state_surface_contract.rs`
- `/workdir/openWEPP/tests/integration/hillslope_consumer_boundary_integration.rs`
- `/workdir/openWEPP/tests/integration/kernel_writeback_contract.rs`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`

## Intended Write Set
- `docs/work-packages/20260525-refactor003-openwepp-hillslope-orchestrator-hydrology-mechanical-modularization-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/mod.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/*.rs`

## Phase Plan
### Phase A - Intake and API Surface Freeze
- Capture current exported symbols and module wiring.
- Freeze section boundaries and migration order.

### Phase B - Mechanical Module Extraction
- Create target hydrology section files.
- Move code blocks into files with no intended behavior change.
- Preserve visibility and API surface.

### Phase C - Validation Surface Check
- Confirm no integration tests depend on single `hydrology.rs` residency.
- Update layout-coupled assertions only if required.

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
- Hydrology module split is complete with stable public API surface.
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
