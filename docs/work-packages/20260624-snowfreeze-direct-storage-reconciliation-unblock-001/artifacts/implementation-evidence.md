# Implementation Evidence

Status: complete

Evidence mode: Static + Ran.

## Production Change

Static:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs`
  previously computed an aggregate R4B projection delta and applied the entire
  delta to `layer_state_after_root_uptake[0].theta_m`.
- The correction keeps the existing aggregate calculation and typed field names,
  but routes nonzero deltas through
  `r4b_apply_explicit_frost_storage_projection_delta`.
- Positive deltas retain the prior behavior of adding to the first layer.
- Negative deltas first validate that the active layer `theta_m` pool can cover
  the debit. Only then do they debit layers top-to-bottom. This preserves the
  existing fail-closed behavior for material active-storage deficits and avoids
  partially mutating state on insufficient storage.
- Residual storage terms are not debited as active liquid. If the reconciled
  aggregate would require drawing residual storage, the path still returns
  `NegativeDirectValue {
  field: "storage_reconciliation.frost_storage_projection_theta_m" }`.
- Shadow projection update remains tied to the corrected
  `layer_state_after_root_uptake` vector and reconciled aggregate
  `soil_water_after_m`.

## Regression Tests

Static:

- `r4b_explicit_frost_storage_rebalance_debits_multiple_layers` constructs an
  R4B-valid day with active layer `theta_m = [0.010, 0.050]` and an explicit
  frost storage debit of `0.030 m`. Pre-fix, this fails because the first layer
  alone is debited. Post-fix, the first layer is zeroed, the second layer holds
  `0.030 m`, and the R4N shadow projection matches the corrected layer vector.
- `r4b_explicit_frost_storage_rebalance_rejects_insufficient_active_theta`
  constructs a case where aggregate storage is nonnegative only because
  residual storage exists, while active `theta_m` is insufficient for the debit.
  The corrected path fails closed with the same nonnegative storage projection
  field instead of drawing residual storage as active liquid. Review hardening
  added snapshots proving the insufficient-storage path leaves both the layer
  projection and R4N shadow projection unchanged after the typed error.

Ran:

- `cargo test -p openwepp-hillslope-orchestrator r4b_explicit_frost_storage -- --nocapture`
  passed: `2 passed; 0 failed`.

## Adjacent Hygiene

Ran:

- `cargo clippy --workspace --all-targets -- -D warnings` initially failed on
  the adjacent observed-harness contract test helper
  `tests/integration/snowfreeze_observed_frost_depth_contract.rs` with
  `clippy::map_unwrap_or`.

Static:

- The package write set was amended to allow a mechanical snowfreeze harness
  test lint cleanup. The helper now uses `Option::map_or_else` with unchanged
  panic behavior and no harness semantics change.
