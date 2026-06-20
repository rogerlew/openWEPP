# R4B Implementation And Test Evidence

Status: complete.
Evidence mode: Ran.

Implementation summary:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
  - added `DIRECT_R4B_PHASE_SPAN_COUNT`;
  - added `DIRECT_R4B_STORAGE_RECONCILIATION_SPAN`;
  - added direct storage input, state, downstream operand, shadow projection,
    and span-report types;
  - added `DirectDayFrame::run_r4b_storage_reconciliation_span`;
  - added direct WB12 storage reconciliation compute and validation;
  - added typed errors for missing R4A upstream and direct closure tolerance;
  - ran R4B after R4A and before the R3B diagnostic ledger in the skeleton
    executor.
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
  - exported the R4B constants and public direct storage types.
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`
  - updated aggregate direct-span counters for R3A/R3B/R3C/R4A/R4B;
  - added exact R4B storage identity, shadow projection, downstream operand,
    state mutation, and anti-alias coverage;
  - added invalid input coverage.
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
  - updated opt-in direct-runtime counter assertions to include R4B;
  - preserved default-disabled zero-counter assertions.

Direct equation implemented:

```text
storage_reconciled_m =
  storage_initial_m
  + precip_input_m
  + snow_coupling_m
  - q_runoff_m
  - evapotranspiration_m
  - deep_seepage_m
  - subsurface_loss_m
```

Guard mapping:

- `storage_initial_m`, `precip_input_m`, `q_runoff_m`,
  `evapotranspiration_m`, `deep_seepage_m`, `subsurface_loss_m`,
  `closure_tolerance_m`, and `storage_reconciled_m` must be finite and
  nonnegative.
- `snow_coupling_m` and `closure_residual_m` must be finite.
- closure residual must be within the declared tolerance.
- R4A must have executed first; otherwise R4B returns
  `MissingDirectUpstream { upstream: "R4A runoff partition" }`.

Focused tests:

- `cargo test -p openwepp-hillslope-orchestrator r4b_ -- --nocapture`
  - passed, 2 tests.

Runner counter tests:

- `cargo test -p openwepp-runner r2a_ -- --nocapture`
  - passed, 2 tests.
- `cargo test -p openwepp-hillslope-orchestrator r2a_direct -- --nocapture`
  - passed, 3 tests.

Anti-alias vectors covered:

- omitted `S`;
- wrong `Q` sign;
- omitted ET, `D`, or `Qd`;
- publication-runoff alias;
- R3B diagnostic-ledger alias.

Boundary statement:

R4B is no-publication, no-default, and no-scheduler. The direct runtime remains
shadow-only, and production compatibility output paths remain authoritative.
