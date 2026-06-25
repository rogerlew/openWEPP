# Pre-Implementation Evidence

Status: complete

Evidence mode: mixed.

## Reproduction

Ran:

- Command:
  `.venv/bin/python tools/snowfreeze_observed/observed_harness.py compare --site site3_scan_mandan_nd --observations-dir tests/fixtures/snowfreeze_observed/observations --output-dir target/snowfreeze_observed_compare_site3_direct_prefail`
- Result: exit `1`; `comparison_report.json` emitted
  `verdict = HARNESS-SURFACE-MISMATCH`, `runtime =
  direct-production-executor`, `reason = openwepp-cli-hill failed with exit
  code 1`.
- Runtime stderr:
  `CLIHILL-E-011 runtime surface failure for r7c_direct_production_executor:
  HS-SIMPIPE-E-001 direct runtime day execution failed at lane 1 day 487:
  direct runtime field storage_reconciliation.frost_storage_projection_theta_m
  must be nonnegative`.

Ran:

- Command:
  `.venv/bin/python tools/snowfreeze_observed/observed_harness.py compare --site site4_ggd498_morris_mn --observations-dir tests/fixtures/snowfreeze_observed/observations --output-dir target/snowfreeze_observed_compare_site4_direct_prefail`
- Result: exit `1`; `comparison_report.json` emitted
  `verdict = HARNESS-SURFACE-MISMATCH`, `runtime =
  direct-production-executor`, `reason = openwepp-cli-hill failed with exit
  code 1`.
- Runtime stderr:
  `CLIHILL-E-011 runtime surface failure for r7c_direct_production_executor:
  HS-SIMPIPE-E-001 direct runtime day execution failed at lane 1 day 10727:
  direct runtime field storage_reconciliation.frost_storage_projection_theta_m
  must be nonnegative`.

## Mechanism

Static:

- `DirectDayFrame::run_r4b_storage_reconciliation_span` writes the explicit
  WB12 frost storage liquid delta into
  `storage_reconciliation_inputs.frost_liquid_delta_m`, computes aggregate
  storage, then calls
  `rebalance_r4b_explicit_frost_storage_projection(storage_reconciled_m)`.
- Current `rebalance_r4b_explicit_frost_storage_projection` computes
  `delta_m = storage_reconciled_m - aggregate_m` over all
  `layer_state_after_root_uptake` entries, but applies that entire delta to the
  first layer only.
- When the aggregate delta is negative and its magnitude is greater than the
  first layer's active `theta_m`, the first layer becomes negative even if
  deeper layers contain enough positive active liquid storage for the aggregate
  debit. The nonnegative guard then fails before the comparison harness can
  emit model/observation residuals for the site.

## Contract and Guard Posture

Static: the package does not authorize removal or loosening of
`storage_reconciliation.frost_storage_projection_*` guards. Expected correction
must preserve nonnegative finite per-layer state and fail closed for material
insufficient active storage.
