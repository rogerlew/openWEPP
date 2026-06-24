# R7G Snow Lane Migration

Status: COMPLETE.

Package type: architecture implementation / state authority migration.

Roadmap item: ADR-0026 winter-column snow/frost sequence, "Snow lane
migration" follow-up after mechanical containment.

Subagent authorization: this package explicitly authorizes
spawning/delegating to read-only reviewer and verifier subagents for
snow-state authority review, consumer-path review, no-compatibility scan
review, line-count governance, and final disposition. Subagents may not edit
files. Findings must be dispositioned in `artifacts/review-disposition.md` and
`artifacts/verification.md`.

## Objective

Move direct snow lane-state authority from `DirectSnowRuntimeCarry` to
`DirectWinterColumnState.snow` for the production direct snow/frost path. The
package must make the winter-column snow state the persistent lane/day source
for prior snowpack, same-day snow mutation, and downstream frost forcing while
preserving existing validated behavior.

## Required Reading

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/decisions/0026-stateful-winter-column-sub-solver.md`
- `docs/architecture/coupled-frost-sub-solver-specification.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/work-packages/20260623-r7g-winter-column-mechanical-containment-001/package.md`
- `docs/work-packages/20260623-r7g-winter-column-mechanical-containment-001/artifacts/worker-handoff.md`

## Included Scope

- Add typed helpers that convert existing seed/coupling snow values into
  `DirectSnowLaneState` without moving snow physics math.
- Seed direct lane/day frames from `DirectWinterColumnState.snow`.
- Make R4G snow coupling mutate `DirectWinterColumnState.snow` as the
  canonical persistent state.
- Keep `DirectSnowRuntimeCarry` only as a temporary compatibility mirror for
  still-unmigrated frame surfaces; it must not be the production direct
  snow/frost consumer authority after this package.
- Make direct publication snow/frost helpers read prior snowpack from
  `lane.winter_column.snow`.
- Preserve the legacy ordering invariant: same-day frost forcing sees prior
  snowpack, not same-day snow partition output.
- Add or update focused tests/source scans proving the authority direction and
  prior-snow invariant.
- Update package evidence and work-package catalog entries.

## Excluded Scope

- No frost lane-state migration or deletion of `DirectFrostRuntimeCarry`.
- No deletion of `DirectSnowRuntimeCarry` from public direct-runtime frame
  structs unless all residual consumers are proven migrated in this package.
- No R7G output parity, default activation, performance, or protected-output
  closure claim.
- No snow/frost physics formula changes, threshold changes, or
  `SC-SNOWFREEZE-001` contract changes.
- No publication cutover claim beyond the direct publication helper reading
  winter-column snow state for this package's direct path.

## Intended Write Set

- `docs/work-packages/20260623-r7g-snow-lane-migration-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-hillslope-orchestrator/src/winter_column.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs`
- focused direct-runtime tests under
  `crates/openwepp-hillslope-orchestrator/src/tests/**`
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`

## Non-Goals / Anti-Drift Rule

This is not a frost-subsolver completion package and not a full R7G closure
package. If validation exposes frost parity, output parity, or performance
defects outside the snow-lane authority envelope, close in HOLD with a
defect-shaped follow-up rather than expanding the package into unbounded R7G
work.

## Exit Criteria

- `DirectSnowLaneState` carries seeded runtime snow state into direct lane/day
  frames before production direct execution.
- R4G snow coupling writes same-day runtime snow output back into
  `DirectWinterColumnState.snow` before lane commit.
- Direct publication snow partition and frost forcing read prior snowpack from
  `lane.winter_column.snow`, not `lane.snow_runtime_carry`.
- Source scan proves
  `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
  no longer contains `lane.snow_runtime_carry`,
  `current_snow_runtime_carry`, `initial_snow_runtime_carry`, or
  `snow_runtime_carry.map_or`.
- Focused tests prove constructor seeding, R4G state mutation, lane commit, and
  frost-prior-snow ordering against the winter-column snow state.
- Line-count governance is recorded for touched `.rs` files.
- Dual reviews and verification are completed and dispositioned.
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
- [x] Implement snow lane-state authority migration.
- [x] Add focused tests/source scans.
- [x] Run closure gates.
- [x] Complete review, verification, line-count, and worker-handoff artifacts.
- [x] Close package disposition and update work-package catalog.

## Completion Summary

Completed on 2026-06-23. `DirectWinterColumnState.snow` is now the direct
production lane/day authority for seeded snow state, R4G same-day snow state
mutation, and direct publication snow/frost prior-snow reads.
`DirectSnowRuntimeCarry` remains only as a temporary direct-runtime
compatibility mirror for residual frame surfaces. Focused runtime tests,
source-scan guards, scoped Markdown lint, `cargo fmt --check`,
`cargo clippy --workspace --all-targets -- -D warnings`,
`cargo test --workspace`, `cargo deny check`, and `git diff --check` passed.
