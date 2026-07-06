# Implementation Test Evidence

Status: **COMPLETE** (Static + Ran).

## Runtime Changes

- Added `DirectErosionHydrographShapeAuthority` with default
  `Dc01SourceShape` and explicit `RoutedHydrograph` candidate mode.
- Added boxed `DirectErosionInputs.routed_hydrograph_runoff_fraction` so the
  default/off day-constructor size bound remains intact.
- `r7d8_surface_hourly_weights` now selects DC01 source weights or the routed
  hydrograph candidate by authority.
- Candidate validation:
  - missing candidate => `MissingDirectUpstream`;
  - non-finite/negative weights => typed value error;
  - positive runoff with `sum(w_h) != 1` beyond `1.0e-9` =>
    `DirectClosureToleranceExceeded`;
  - no runoff with nonzero sum beyond `1.0e-12` =>
    `DirectClosureToleranceExceeded`.

## Focused Commands

Ran:
`cargo test -p openwepp-hillslope-orchestrator wave1_span_routed_hydrograph_shape -- --nocapture`

Result before boxing: pass, 3 passed.

Ran after boxing the optional routed shape:
`cargo test -p openwepp-hillslope-orchestrator wave1_span_routed_hydrograph_shape -- --nocapture`

Result: pass, 3 passed.

Ran:
`cargo test -p openwepp-hillslope-orchestrator direct_runtime_wave1_continuity -- --nocapture`

Result: pass, 28 passed.

Ran:
`cargo check --workspace`

Result: pass.

Ran after initial full-nextest size regression:
`cargo test -p openwepp-hillslope-orchestrator r7b_constructor_type_size_layout_is_bounded -- --nocapture`

Result: pass, `DirectDayConstructorInputs=4088` (bound `<= 4096`).

Ran:
`cargo test -p openwepp --test laned_shadow_h2637 h2637_native_shadow_classifies_uniform_shape_after_d12 -- --ignored --nocapture`

Result: pass, 1 passed, finished in `325.24s` on the final boxed-shape code.

## Gate Linkage

The full package gate suite is recorded in
`artifacts/gate-results.md`. The initial full-nextest size-bound failure was
accepted, fixed by boxing the optional routed shape, and verified by the
focused size test plus the final full-nextest pass.
