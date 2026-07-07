# Implementation

Status: COMPLETE. Evidence mode: Static + Ran.

## Files Changed

- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs`
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`
- Package docs and artifacts under this work package.
- `docs/work-packages/README.md`

## Code Behavior

The public predicate `CellParameters::is_bare_skin_only()` now exposes the
same exact-active-addend absence rule used by the rev-4 direct branch
evaluator. It is `#[must_use]`.

`laned_active_route_lane` now interprets `hybrid_implicit: true` as a request:

- Build the active lane-day `CellParameters` from static Lane-D friction
  operands plus post-growth `leaf_area_index` and `canopy_height_m`.
- If `cell.is_bare_skin_only()`, route with `route_single_ofe_hybrid`.
- Otherwise route with `route_single_ofe`.
- Count requested, selected, and plain-fallback lane-days in
  `DirectLanedActiveDayBooks` and fold them into
  `DirectLanedActiveRunSummary`.

The runner manifest `laned_active` block now includes:

- `hybrid_implicit_requested_lane_days`
- `hybrid_implicit_selected_lane_days`
- `hybrid_implicit_plain_fallback_lane_days`

The existing `hybrid_implicit_stepping` boolean remains request-compatible for
existing harness checks: it reports whether the env request was set for that
active run, while the counters report actual selection/fallback.

## Tests Added

- `hybrid_request_selects_exact_bare_skin_lane_day`
- `hybrid_request_falls_back_to_plain_on_post_growth_vegetation`

The tests prove that:

- A bare active lane-day selects hybrid and increments selected counters.
- A vegetated post-growth lane-day falls back to active plain and increments
  fallback counters.
- Counters fold into the run summary after day closure.

## Default / Subsystem-Off Isolation

Static:

- The selection branch is reachable only when the active executor already
  passes `config.hybrid_implicit == true`.
- `config.hybrid_implicit` is set only from
  `OPENWEPP_LANED_ACTIVE_IMPLICIT=1` inside the active direct-publication
  builder.
- With `OPENWEPP_LANED_ACTIVE_IMPLICIT` unset, `hybrid_implicit_selected` is
  false and the original `route_single_ofe` path is used.
- With `OPENWEPP_LANED_ACTIVE` unset, the active path is not constructed and
  the new manifest fields are absent because the `laned_active` block is
  absent.

Ran:

- Active plain selected-cohort rerun emitted zero requested/selected/fallback
  lane-days in all plain manifests.
- Full workspace nextest passed.

## Line Count Governance

Current touched source line counts:

| File | Lines | Status |
|---|---:|---|
| `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs` | 2126 | Existing over-threshold file; one-line visibility/attribute change only. |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs` | 1045 | Within threshold. |
| `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` | 1308 | Existing over-threshold file; manifest struct field addition only. |
| `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs` | 1339 | Existing over-threshold file; manifest mapping field addition only. |

No new over-threshold Rust file was created.
