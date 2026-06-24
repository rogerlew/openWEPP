# R7G Consumer Cutover And Deletion

Status: COMPLETE.

Package type: architecture implementation / consumer cutover / deletion.

Roadmap item: ADR-0026 winter-column snow/frost sequence, "Consumer cutover
and deletion" follow-up after typed frost solver extraction.

Subagent authorization: none. This package is executed locally in the current
agent session; reviews and verification are recorded as independent local
passes.

## Objective

Cut the remaining direct-runtime and direct-publication consumers from the
temporary frost runoff-surface/liquid-partition bridge to typed winter-column
state and outcomes, then delete the production bridge fields and API. R4A
runoff reconciliation must finish frost from typed winter-column state using the
latest post-ET/subsurface layers, not from `DirectFrostRunoffSurface`,
`HillslopeKernelRequest`, or `DirectFrostLiquidPartition` handoff fields.

## Required Reading

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `crates/AGENTS.md`
- `docs/decisions/0026-stateful-winter-column-sub-solver.md`
- `docs/architecture/coupled-frost-sub-solver-specification.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/work-packages/20260624-r7g-typed-frost-solver-extraction-001/package.md`

## Included Scope

- Add a typed post-hydrology frost partition/outcome path that consumes
  existing `DirectWinterColumnState.frost`, typed frost compute inputs, and the
  latest R4A layer state.
- Cut `DirectDayFrame::reconcile_r4a_frost_runtime` from
  `DirectFrostLiquidPartition`/`DirectFrostRunoffSurface` fields to typed
  winter-column frost state/outcome.
- Remove production direct-day/publication-day fields for
  `frost_runoff_surface` and `frost_liquid_partition`.
- Remove production builder assignment of frost runoff surfaces and liquid
  partition bridge handoff.
- Delete or de-export `DirectFrostRunoffSurface` and the map-backed
  `Wb11HydrologyKernel::compute_direct_frost_liquid_partition` production API.
- Replace source-scan tests so they prove deletion, not only isolation.
- Preserve physics and contract behavior; this package changes the runtime
  boundary, not the frost equations.
- Record required evidence, reviews, line-count governance, and verification.

## Excluded Scope

- No R7G terminal protected-output parity, default activation, release-readiness,
  or H2637 performance claim.
- No snow physics formula changes, frost threshold retuning, or
  `SC-SNOWFREEZE-001` amendment unless a contradictory authority blocker is
  exposed.
- No deletion of unrelated snow carry or compatibility scheduler surfaces.
- No compatibility rollback behavior changes outside the direct production
  winter-column consumer path.

## Intended Write Set

- `docs/work-packages/20260624-r7g-consumer-cutover-deletion-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-hillslope-orchestrator/src/winter_column.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/**`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**`
- focused orchestrator tests under
  `crates/openwepp-hillslope-orchestrator/src/tests/**`
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/**`

## Non-Goals / Anti-Drift Rule

This package is not allowed to close by retaining a bridge field and saying
consumers "can switch later." The closure target is consumer cutover plus
deletion. If a current-scope production consumer still reads
`DirectFrostRunoffSurface`, `DirectFrostLiquidPartition`, or their
`frost_*` handoff fields, continue implementation or close as `HOLD` with the
exact blocker.

## Exit Criteria

- R4A frost reconciliation computes/applies frost from typed winter-column
  state/outcome and the latest post-ET/subsurface layer state.
- Production direct-day and publication-day inputs have no
  `frost_runoff_surface` or `frost_liquid_partition` fields.
- Production direct-publication builders do not construct
  `DirectFrostRunoffSurface`, call
  `compute_direct_frost_liquid_partition`, or assign frost bridge fields.
- `DirectFrostRunoffSurface` is deleted or de-exported from production crates.
- `DirectFrostLiquidPartition` is no longer a production handoff field; any
  retained typed outcome has a winter-column name and is sourced from typed
  inputs/state.
- Source scans prove no production direct-runtime/direct-publication consumer
  reads the deleted bridge.
- Focused tests cover active/inactive typed frost outcome application,
  no-material fine/shadow carry preservation, and production source deletion.
- Line-count governance is recorded for touched `.rs` files.
- Dual local reviews and verification are completed and dispositioned.
- Required closure commands run and are recorded:
  `cargo fmt --check`;
  `cargo clippy --workspace --all-targets -- -D warnings`;
  `cargo test --workspace`;
  `cargo deny check`.

## Progress

- [x] Scaffold package.
- [x] Map remaining producers/consumers and deletion boundary.
- [x] Implement typed consumer cutover.
- [x] Remove bridge fields/API and update tests.
- [x] Run closure gates.
- [x] Complete review, verification, line-count, and worker-handoff artifacts.
- [x] Close package disposition and update work-package catalog.

## Final Disposition

Completed. R4A frost reconciliation now consumes typed winter-frost compute
inputs and winter-column frost state through direct runtime execution context,
not the deleted production bridge. Production direct-day/publication-day fields
for `frost_runoff_surface` and `frost_liquid_partition` are removed, the
map-backed direct frost-liquid-partition API is deleted from production, and
direct-publication builders hand typed winter-frost compute inputs to R4A.

Closure evidence is recorded under `artifacts/`. Final required gates passed:
`cargo fmt --check`; `cargo clippy --workspace --all-targets -- -D warnings`;
`cargo test --workspace`; `cargo deny check`; `git diff --check`.
