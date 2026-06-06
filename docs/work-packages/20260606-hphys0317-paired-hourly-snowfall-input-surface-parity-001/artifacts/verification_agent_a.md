# Verification Agent A

Status: complete

Evidence mode: Static + Ran

Static:

- `SC-CLIMATE-001#INV-CLIMATE-015`,
  `SC-SNOWFREEZE-001#INV-SNOWFREEZE-043`, and
  `SC-WATBAL-001#INV-WATBAL-091` are registered.
- The paired input-surface ledger preserves all `57` carried rows and assigns
  the missing paired controlling surface evidence to HPHYS0318.

Ran:

- `cargo test --test hphys0317_hourly_snowfall_input_surface_parity_contract -- --nocapture`

Final verification: PASS
