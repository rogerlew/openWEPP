# R7G Frost State Skeleton and Comparator Seam

Status: COMPLETE.

Package type: architecture implementation / state authority migration.

Roadmap item: ADR-0026 winter-column snow/frost sequence, "Frost state
skeleton and comparator seam" follow-up after snow lane migration.

Subagent authorization: none. This package is executed locally in the current
agent session; reviews and verification are recorded as independent local
passes.

## Objective

Make `DirectWinterColumnState.frost` the canonical persistent skeleton for
direct frost lane/day state, while isolating the remaining
`DirectFrostRunoffSurface` compatibility bridge behind named comparator-seam
helpers. The package completes the skeleton/seam layer only; it must not claim
typed frost solver extraction or final deletion of the bridge.

## Required Reading

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/decisions/0026-stateful-winter-column-sub-solver.md`
- `docs/architecture/coupled-frost-sub-solver-specification.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/work-packages/20260623-r7g-winter-column-mechanical-containment-001/package.md`
- `docs/work-packages/20260623-r7g-snow-lane-migration-001/package.md`

## Included Scope

- Add typed mirror conversions between `DirectFrostRuntimeCarry` and
  `DirectFrostLaneState`, including front/thaw scalars, fine layers, layer
  shadows, no-material carry, liquid/frozen exchange ledger, `watpdg`, and
  `watbtm`.
- Seed direct lane/day frames from `DirectWinterColumnState.frost`, falling
  back to legacy `DirectFrostRuntimeCarry` only to populate the skeleton when a
  preexisting constructor uses the old mirror.
- Make R4A frost reconciliation mutate `DirectWinterColumnState.frost` as the
  same-day state authority, and regenerate `DirectFrostRuntimeCarry` only as a
  temporary mirror for residual frame surfaces.
- Make direct publication frost day context and runtime-surface overlay read
  prior frost state from `lane.winter_column.frost` rather than
  `lane.frost_runtime_carry`.
- Isolate remaining `DirectFrostRunoffSurface` construction/seeding behind
  named comparator-seam helpers and add source-scan evidence proving the main
  builder body no longer owns direct surface construction.
- Add focused tests/source scans proving constructor seeding, lane commit,
  R4A mutation direction, publication read authority, and named seam
  isolation.
- Update package evidence and work-package catalog entries.

## Excluded Scope

- No typed frost solver extraction from `DirectFrostRunoffSurface`.
- No deletion of `DirectFrostRuntimeCarry`,
  `DirectFrostRunoffSurface`, or `DirectFrostLiquidPartition`.
- No output/parity/performance claim, default activation claim, or R7G closure
  claim.
- No snow/frost physics formula changes, threshold changes, or
  `SC-SNOWFREEZE-001` contract changes.
- No publication cutover claim beyond direct publication reading prior frost
  state from `DirectWinterColumnState.frost`.

## Intended Write Set

- `docs/work-packages/20260624-r7g-frost-state-skeleton-comparator-seam-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-hillslope-orchestrator/src/winter_column.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs`
- focused direct-runtime tests under
  `crates/openwepp-hillslope-orchestrator/src/tests/**`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/01_frost_and_layer_helpers.rs`
- optional new include chunk under
  `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`

## Non-Goals / Anti-Drift Rule

This package is not allowed to close the final frost subsolver cutover by
narrative. If execution proves the production path still requires the
`DirectFrostRunoffSurface` bridge, record that as the next typed frost solver
blocker rather than claiming deletion or R7G completion.

## Exit Criteria

- `DirectFrostLaneState` has complete carry mirror support for all current
  `DirectFrostRuntimeCarry` fields, fine layers, layer shadows, exchange
  ledger fields, `watpdg`, and `watbtm`.
- Direct lane/day construction and commit preserve frost through
  `DirectWinterColumnState.frost`; the legacy carry is regenerated from winter
  state as a temporary mirror only.
- R4A frost reconciliation writes partition results into
  `DirectWinterColumnState.frost` before downstream commit.
- Direct publication frost context and runtime-surface overlay read prior
  frost from `lane.winter_column.frost`, not from `lane.frost_runtime_carry`.
- Source scan proves
  `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
  does not construct `DirectFrostRunoffSurface` directly and does not read
  `lane.frost_runtime_carry`.
- Remaining `DirectFrostRunoffSurface` production use is isolated to named
  comparator-seam helper functions with a documented typed-solver follow-up
  blocker.
- Focused tests prove constructor seeding, lane commit, same-day R4A frost
  mutation direction, publication read authority, and comparator-seam source
  isolation.
- Line-count governance is recorded for touched `.rs` files.
- Dual local reviews and verification are completed and dispositioned.
- Required closure commands run and are recorded:
  `cargo fmt --check`;
  `cargo clippy --workspace --all-targets -- -D warnings`;
  `cargo test --workspace`;
  `cargo deny check`.

## Validation Notes

Package-required closure gates override ambient test-skip assumptions. If a
gate cannot run, record the exact command-level blocker and close as HOLD.

## Progress

- [x] Scaffold package.
- [x] Implement frost lane-state authority skeleton.
- [x] Isolate named comparator seam.
- [x] Add focused tests/source scans.
- [x] Run closure gates.
- [x] Complete review, verification, line-count, and worker-handoff artifacts.
- [x] Close package disposition and update work-package catalog.

## Completion Summary

Completed on 2026-06-24. `DirectWinterColumnState.frost` is now the direct
lane/day skeleton authority for constructor seeding, R4A same-day frost
mutation, lane commit, direct publication prior-frost reads, and runtime
surface overlay. `DirectFrostRuntimeCarry` remains as a temporary mirror
derived from the winter-column state. Remaining `DirectFrostRunoffSurface`
construction is isolated to `03_frost_comparator_seam.rs`; typed frost solver
extraction remains the required follow-up before bridge deletion or R7G
closure.
