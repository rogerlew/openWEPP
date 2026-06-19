# PERFDEEP02 - Hydrology Island Core (array-native Stage 1)

Status: complete 2026-06-19. Verdict: `NO-GO - performance blocked`.

Package type: **Performance architecture implementation** under
[ADR-0025](../../decisions/0025-array-native-hillslope-day-frame.md) and
[`docs/architecture/array-native-runtime-specification.md`](../../architecture/array-native-runtime-specification.md).

## Objective

Execute Stage 1 of the `PERFDEEP0N` series: migrate the hydrology island toward
`HillslopeDayFrame` authority, beginning with the carried Stage-0 real-surface
round-trip gate and then moving a real hydrology computation slice onto frame
reads/writes with exact identity evidence. The implementation reached functional
dense-slot coverage, but failed the H2637 endpoint gate and is therefore
fail-closed behind `OPENWEPP_PERFDEEP02_FRAME_ISLAND=1`.

## Entry Authority

- `docs/ROADMAP.md` authorizes `PERFDEEP02` as the next perf package after
  `PERFDEEP01`.
- `PERFDEEP01` disposition is `GO` with two carried conditions:
  1. close the real-surface, every-symbol seed/flush round-trip before any phase
     becomes frame-authoritative;
  2. inherit the slot-frame representation (`Vec<Option<BoundaryValue>>` keyed
     by `SymbolId`).

## In Scope

- Add current-scope evidence for the real H2637 prepared runtime surfaces:
  `HillslopeDayFrame` seed/flush must be `to_bits()` identical on every symbol
  before and after scheduler execution.
- Add focused in-repo coverage for the same frame round-trip mechanics without
  depending on `/tmp` perf fixtures.
- Attempt the first hydrology island core migration using the inherited slot
  frame and preserve exact identity against the logical path.
- Record whether Stage 1 can proceed to the complete hydrology island, or must
  hold at a named blocker.

## Out of Scope

- No science/numeric behavior changes.
- No output schema changes.
- No new writeback-only or materialization-retirement rung may be marked as
  Stage-1 completion.
- No typed-field promotion; the slot frame is the ratified representation.

## Intended Write Set

- `crates/openwepp-kernel-contract/src/lib_mod/core_types/00_symbol_registry_and_indexed_surfaces.rs`
- `crates/openwepp-kernel-contract/src/lib_mod/core_types/02_boundary_values_and_kernel_requests.rs`
- `crates/openwepp-hillslope-orchestrator/src/day_frame.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/day_frame.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/writeback.rs`
- `crates/openwepp-runner/src/hillslope/**`
- this package directory and `docs/work-packages/README.md`

## Deliverables

- `artifacts/perfdeep02-real-surface-roundtrip.md`
- `artifacts/perfdeep02-hydrology-island-migration.md`
- `artifacts/perfdeep02-gate-results.md`
- `artifacts/perfdeep02_disposition.md`
- focused tests proving frame round-trip mechanics for full symbol families.

## Exit Criteria

- Real H2637 prepared lane/day surfaces pass frame seed/flush `to_bits()`
  identity with zero mismatches, or the package is `HOLD` with the failing symbol
  and blocker recorded.
- Focused non-`/tmp` tests pass for full-family frame round-trip coverage.
- Any migrated hydrology slice has per-branch exact identity against the existing
  logical path.
- H2637 output identity and endpoint/RSS are recorded if production hot-path
  behavior changes. A production opt-in that exceeds the PERFDEEP01 endpoint is
  not acceptable for `GO`; leave it disabled by default and record the blocker.
- Required closure gates are run or truthfully marked `NOT RUN` with rationale.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
to read-only review and verification subagents for package artifacts, code
diffs, and gate evidence. Expected outputs are review and verification artifact
drafts under this package. Write access is bounded to this package's
`artifacts/` directory.
