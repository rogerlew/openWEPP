# R7G Typed Frost Solver Extraction

Status: COMPLETE.

Package type: architecture implementation / typed solver extraction.

Roadmap item: ADR-0026 winter-column snow/frost sequence, "Typed frost solver
extraction" follow-up after frost state skeleton and comparator seam.

Subagent authorization: none. This package is executed locally in the current
agent session; reviews and verification are recorded as independent local
passes.

## Objective

Extract the existing active-frost hourly solver behind a typed winter-column
boundary so production direct frost execution no longer computes the partition
through `DirectFrostRunoffSurface` or `HillslopeKernelRequest`. The package
must mutate `DirectWinterColumnState.frost` in place, emit the existing
`DirectFrostLiquidPartition` bridge only as the temporary downstream operand for
pre-step-6 consumers, and preserve the named comparator seam for parity
evidence.

## Required Reading

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/decisions/0026-stateful-winter-column-sub-solver.md`
- `docs/architecture/coupled-frost-sub-solver-specification.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/work-packages/20260623-r7g-winter-column-mechanical-containment-001/package.md`
- `docs/work-packages/20260623-r7g-snow-lane-migration-001/package.md`
- `docs/work-packages/20260624-r7g-frost-state-skeleton-comparator-seam-001/package.md`

## Included Scope

- Add typed active-frost solver input structures for controls, hourly winter
  forcing, prior snow/residue thermal context, soil-column layer state, and
  canonical prior `DirectFrostLaneState`.
- Refactor the existing active-frost hourly loop so the production compute path
  consumes typed inputs directly instead of reading a map-backed
  `HillslopeKernelRequest`.
- Add an in-place winter-column frost advance helper that mutates
  `DirectWinterColumnState.frost` from the typed solver outcome.
- Cut production direct publication day-input frost context to the typed solver
  and stop assigning a production `frost_runoff_surface`.
- Preserve `DirectFrostRunoffSurface` only as a named test/comparator adapter
  for parity evidence until the later consumer-cutover/deletion package.
- Validate typed output against the compatibility adapter on focused active and
  inactive/no-material fixtures under `SC-SNOWFREEZE-001` diagnostic
  tolerances.
- Add source-scan tests proving the production frost hot path no longer calls
  `DirectFrostRunoffSurface::compute_frost_liquid_partition`, constructs a
  frost surface, or reads `HillslopeKernelRequest`.
- Record package evidence, dual local reviews, dual verification, line-count
  governance, and worker handoff.

## Excluded Scope

- No deletion of `DirectFrostRunoffSurface`, `DirectFrostLiquidPartition`,
  direct-runtime `frost_liquid_partition`, or residual direct-runtime fallback
  fields. That is roadmap step 6.
- No HBP/WAT/PASS/loss/plot byte-parity claim, default activation claim,
  performance closure claim, or R7G closure claim.
- No snow physics formula changes, frost threshold retuning, or
  `SC-SNOWFREEZE-001` contract amendments unless execution exposes a missing
  or contradictory authority blocker.
- No consumer deletion beyond making the production runner/direct-publication
  day-input path use the typed solver.

## Intended Write Set

- `docs/work-packages/20260624-r7g-typed-frost-solver-extraction-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-hillslope-orchestrator/src/winter_column.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/**`
- focused orchestrator tests under
  `crates/openwepp-hillslope-orchestrator/src/tests/**`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/01_frost_and_layer_helpers.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/03_frost_comparator_seam.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`

## Non-Goals / Anti-Drift Rule

This package is not allowed to close by saying typed extraction "will happen
next." If a production path still computes active frost through
`DirectFrostRunoffSurface` or `HillslopeKernelRequest`, continue implementation
or close as `HOLD` with the exact blocker. Do not treat a comparator-only seam
as production extraction evidence.

## Exit Criteria

- Production direct frost day context constructs typed solver inputs from
  direct lane/day state, prior snow, residue, hourly winter forcing, and soil
  layers.
- The active-frost hourly solver has a typed compute entry point that does not
  construct or read `HillslopeKernelRequest`.
- `DirectWinterColumnState.frost` is mutated in place from typed solver output;
  fine/shadow carry remains distinct from coarse layer mutation.
- The production direct day input no longer assigns
  `day_input.frost_runoff_surface = Some(...)`.
- Source scans prove the production frost hot path does not call
  `DirectFrostRunoffSurface::compute_frost_liquid_partition`, build a
  `DirectFrostRunoffSurface`, or reference `HillslopeKernelRequest`.
- Comparator-seam tests prove typed-vs-adapter diagnostic parity for focused
  active and inactive/no-material fixtures.
- Focused runtime/publication tests still pass for frost partition application
  and prior-snow ordering.
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
- [x] Implement typed active-frost solver boundary.
- [x] Cut production direct frost day context to typed solver.
- [x] Add focused typed-vs-adapter and source-scan tests.
- [x] Run closure gates.
- [x] Complete review, verification, line-count, and worker-handoff artifacts.
- [x] Close package disposition and update work-package catalog.

## Final Disposition

`COMPLETE-R7G-TYPED-FROST-SOLVER-EXTRACTION`.

The production direct frost day context now builds typed frost inputs from
direct lane/day state, prior snow, residue, hourly winter forcing, and soil
layers, then computes the frost liquid partition through the typed
`Wb11HydrologyKernel` entry point. It no longer assigns a production
`DirectFrostRunoffSurface` to day input and no longer uses
`HillslopeKernelRequest` in the production frost hot path.

`DirectFrostRunoffSurface` remains only as the named comparator seam for
diagnostic parity fixtures and for the later consumer-cutover/deletion package.
Closure evidence is recorded in this package's artifacts, including typed
solver proof, comparator parity proof, line-count governance, dual local review,
worker handoff, and verification results.
