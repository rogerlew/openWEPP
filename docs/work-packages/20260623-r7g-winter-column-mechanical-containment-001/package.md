# R7G Winter-Column Mechanical Containment

Status: COMPLETE.

Package type: mechanical containment / architecture implementation scaffold
with validation-blocker remediation.

Roadmap item: W, "Mechanical containment before growth" under the
ADR-0026 winter-column snow/frost sequence.

Subagent authorization: this package explicitly authorizes spawning/delegating
to read-only reviewer and verifier subagents for architecture-boundary review,
mechanical-refactor review, no-compatibility scan review, line-count governance,
and final disposition. Subagents may not edit files. Findings must be
dispositioned in `artifacts/review-disposition.md` and
`artifacts/verification.md`.

## Objective

Create the winter-column module boundary required by ADR-0026 before any
additional snow/frost solver growth. The package completes the containment
step: ratified winter-column types exist outside `direct_runtime` phase
modules, direct frames can own the new state, and current production winter
solver ownership remains outside the new module until a later migration
package.

## Required Reading

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/decisions/0026-stateful-winter-column-sub-solver.md`
- `docs/architecture/coupled-frost-sub-solver-specification.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/ROADMAP.md`

## Included Scope

- Add a crate-level winter-column module outside `direct_runtime` phase modules.
- Define the ratified containment types:
  `DirectWinterColumnState`, `DirectSnowLaneState`, `DirectFrostLaneState`,
  `DirectWinterDayForcing`, and `DirectWinterDayOutcome`.
- Add typed snow/frost sub-state structures needed to carry the existing R7G
  state envelope without moving solver math.
- Add inert direct-frame ownership hooks so lane/day frames can carry
  `DirectWinterColumnState`.
- Add focused compile-time tests or source scans proving the boundary exists
  and contains no compatibility request/symbol surface authority.
- Remediate validation-only blockers discovered by the package-required closure
  gates when the fix is already contract-required and does not move winter
  solver ownership into the new module.
- Split the oversized direct-publication day-input helper into ordered include
  chunks when needed to preserve line-count governance and localize clippy
  containment.
- Update work-package evidence and catalog entries.

## Excluded Scope

- No snow or frost solver migration.
- No deletion of existing `DirectSnowCoupling*`, `DirectSnowRuntimeCarry`,
  `DirectFrostRuntimeCarry`, `DirectFrostRunoffSurface`,
  `DirectFrostLiquidPartition`, or R4G/R4A consumers.
- No output/parity/performance claim.
- No unvalidated physics, equations, units, thresholds, guards, or
  `SC-SNOWFREEZE-001` contract changes. The only accepted exception is the
  package-validation blocker fix for active-frost no-freeze hourly diagnostic
  publication required by existing FDHP/CLIM06 contract tests.
- No default direct activation or R7G closure claim.

## Intended Write Set

- `docs/work-packages/20260623-r7g-winter-column-mechanical-containment-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/winter_column.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs`
- focused tests under `crates/openwepp-hillslope-orchestrator/src/tests/**`
  if needed for boundary proof
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs`
  for the active-frost no-freeze diagnostic fast-path gate blocker exposed by
  `cargo test --workspace`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs`
  for final clippy closure
- `crates/openwepp-hillslope-orchestrator/src/tests.rs` for final clippy
  closure
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
  for final clippy closure
- `crates/openwepp-runner/src/hillslope/mod.rs` for final clippy containment
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers.rs`
  and split include chunks under
  `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/`
  for line-count and clippy containment
- `crates/openwepp-runner/src/hillslope/03_tests.rs` for source-inspection
  tests over the split helper

## Non-Goals / Anti-Drift Rule

This is a containment package. Do not opportunistically improve snow/frost
math, publication parity, R7G performance, or existing direct snow/frost
plumbing. Any movement of solver behavior or consumer cutover must be a
follow-up package.

## Exit Criteria

- `DirectWinterColumnState`, `DirectSnowLaneState`, `DirectFrostLaneState`,
  `DirectWinterDayForcing`, and `DirectWinterDayOutcome` exist in a module
  outside `direct_runtime`.
- Direct lane/day frames own or can carry `DirectWinterColumnState` without
  changing existing execution semantics.
- Source scan proves `winter_column.rs` does not reference
  `DirectFrostRunoffSurface`, `HillslopeKernelRequest`,
  `HillslopeWritebackSurface`, `BoundarySymbol`, `BoundaryValue`, WB13 rows, or
  map-backed symbol helpers.
- No existing direct-runtime production behavior is intentionally changed by
  winter-column containment. The only production behavior remediation is the
  separately recorded active-frost no-freeze hourly diagnostic publication fix
  required to satisfy an existing contract gate during package validation.
- Line-count governance is recorded for touched `.rs` files.
- Dual reviews and verification are completed and dispositioned.
- Required closure commands run and are recorded:
  `cargo fmt --check`;
  `cargo clippy --workspace --all-targets -- -D warnings`;
  `cargo test --workspace`;
  `cargo deny check`.

## Validation Notes

The package-required closure gates override ambient test-skip assumptions. If a
gate cannot run, record the exact command-level blocker and close as HOLD.

## Progress

- [x] Scaffold package.
- [x] Implement winter-column module and inert frame ownership hooks.
- [x] Add focused boundary tests/scans.
- [x] Run closure gates.
- [x] Complete review, verification, line-count, and worker-handoff artifacts.
- [x] Close package disposition and update work-package catalog.

## Completion Summary

Completed on 2026-06-23. The package added the crate-level winter-column
state/forcing/outcome boundary, reused the existing runtime-input hourly winter
forcing authority, boxed inert `DirectWinterColumnState` ownership into direct
lane/day frames, proved the new module has no compatibility symbol surface
authority, split the oversized direct-publication day-input helper into
sub-3000-line chunks, fixed the validation-exposed active-frost no-freeze
hourly diagnostic gap, and passed the required Rust closure gates.
