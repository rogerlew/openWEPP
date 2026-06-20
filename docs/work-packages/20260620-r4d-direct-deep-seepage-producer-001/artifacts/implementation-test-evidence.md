# R4D Implementation And Test Evidence

Status: complete.
Evidence mode: Static + Ran.

Implementation summary:

- Added `DIRECT_R4D_PHASE_SPAN_COUNT` and `DIRECT_R4D_DEEP_SEEPAGE_SPAN`.
- Added direct deep-seepage input, state, downstream operand, shadow projection,
  and span-report types.
- Extended `DirectDayFrame` with R4D input/state/downstream/shadow fields.
- Added `run_r4d_deep_seepage_span`, direct compute, validation, state
  mutation, downstream operand production, and shadow projection in
  `direct_runtime/storage.rs`.
- Updated the direct executor to run R4D after R4C and before R4A/R4B.
- Updated R4B validation so it requires R4C storage input, R4D deep seepage,
  and R4A runoff upstream shadows before reconciliation.
- Updated public exports and runner aggregate counter expectations.

Focused tests:

```text
cargo test -p openwepp-hillslope-orchestrator r4d_ -- --nocapture
```

Result: PASS, 2 tests passed.

```text
cargo test -p openwepp-hillslope-orchestrator r4b_ -- --nocapture
```

Result: PASS, 3 tests passed.

```text
cargo test -p openwepp-hillslope-orchestrator r2a_direct -- --nocapture
```

Result: PASS, 3 tests passed.

```text
cargo test -p openwepp-runner r2a_ -- --nocapture
```

Result: PASS, 2 tests passed.

Full gates:

```text
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
```

Result: PASS.

Anti-alias evidence:

- R4D focused test distinguishes the direct handoff from publication drainage,
  R4B `subsurface_loss_m`, ET, snow coupling, precipitation, runoff, R3B
  diagnostic residual, and storage closure residual.
- R4B focused test seeds `storage_reconciliation_inputs.deep_seepage_m` with a
  sentinel `9.0` and proves R4D overwrites it with `0.03125` before R4B
  reconciles storage.
