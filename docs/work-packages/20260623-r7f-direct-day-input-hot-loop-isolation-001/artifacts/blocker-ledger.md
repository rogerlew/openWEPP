# R7F Blocker Ledger

Status: complete.

## Ledger Protocol

Each blocker entry records defect ID, evidence, mechanism, owner surface,
classification, correction, validation, and next gate. Classifications are
`in-envelope`, `out-of-envelope`, `invalid-input`, `authority-missing`, or
`evidence-unavailable`.

## Iteration 1 - Starting Hot Compatibility Edge

- Defect ID:
  `HOLD-R7F-DIRECT-DAY-INPUT-BUILDER-COMPATIBILITY-SURFACE-HOT-EDGE`
- Evidence:
  - Static:
    `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
    constructs `DirectPublicationDayInputBuilder` in
    `execute_hillslope_direct_production_days` and passes it to
    `run_publication_capture_with_interleaved_day_inputs`.
  - Static:
    `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers.rs`
    holds `seed_surfaces: Vec<HillslopeWritebackSurface>` and
    `DirectPublicationClimateContextState` surface state.
  - Static: the builder clones/merges runtime surfaces through
    `seed_surface`, `climate_context_surface`, `advance_climate_context`, and
    `merge_runtime_surfaces`.
  - Static: R7E/R7H made the builder count as
    `record_direct_runtime_compatibility_edge_invocation()` during production
    direct.
- Mechanism: direct scheduler entry is bypassed, but hot day/OFE input
  construction remains compatibility-surface based.
- Owner surface: runner production direct execution setup and direct
  publication day-input helpers.
- Classification: in-envelope.
- Correction: complete. `execute_hillslope_direct_production_days` now
  instantiates `DirectProductionDayInputBuilder`, not
  `DirectPublicationDayInputBuilder`.
- Validation:
  - Ran: `cargo test -p openwepp-runner r7 -- --nocapture`.
  - Static: `r7c_direct_production_source_excludes_compatibility_entrypoints`
    and `r7f_production_direct_uses_typed_day_input_builder`.
  - Static:
    `r7f_typed_day_input_hot_loop_excludes_runtime_surface_reads`.
  - Runtime: explicit production direct and default-activated production direct
    manifests assert `/direct_runtime_counters/compatibility_edge_invocations =
    0`.
- Next gate: complete.

## Iteration 2 - Setup Authority Missing WB11 Day-Zero Overlay

- Defect ID:
  `R7F-SETUP-AUTHORITY-MISSING-DAY-ZERO-WB11-SEED`.
- Evidence:
  - Ran:
    `cargo test -p openwepp-runner r7c_direct_production_executor_reports_interleaved_day_input_compatibility_edges -- --nocapture`
    failed while constructing production direct authority with missing
    `wb18_perc_theta_0001`.
- Mechanism: the raw lane seed surface did not yet carry the WB11 day-zero
  layer projection required by direct percolation/subsurface authority.
- Owner surface: production direct setup authority extraction.
- Classification: in-envelope.
- Correction: `DirectProductionDayInputBuilder::new` now derives setup-time
  lane authority from `direct_publication_day_zero_seed_surface`, matching the
  direct run-frame constructor, while its hot-loop `build` method remains
  typed and surface-free.
- Validation:
  - Ran:
    `cargo test -p openwepp-runner r7c_direct_production_executor_reports_no_day_input_compatibility_edges -- --nocapture`
    passed after correction.
- Next gate: complete.

## Iteration 3 - Inert Frost Option Misclassified As Active Frost

- Defect ID:
  `R7F-FROST-OPTION-BIT-MISCLASSIFIED-AS-MATERIAL-ACTIVE-FROST`.
- Evidence:
  - Ran: focused R7C production direct fixture failed fail-closed on
    `R7F typed production day-input path does not yet have surface-free active
    frost partition authority for lane 1`.
- Mechanism: the guard treated `frost.options.wintRed` as material active
  frost even when runtime frost depth and frozen-water carry were zero.
- Owner surface: `DirectProductionSnowFrostAuthority`.
- Classification: in-envelope.
- Correction: keep fail-closed behavior for non-zero frost depth or frozen
  water, but do not block an inert option bit with no material frost carry.
- Validation:
  - Ran: full focused R7 suite passed.
  - Ran: focused R6 suite passed.
- Next gate: complete.
