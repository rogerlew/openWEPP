# Contract Implementation Evidence

Status: complete

Evidence mode: ran

Static:

- Added `SC-SNOWFREEZE-001#INV-SNOWFREEZE-035`.
- Added `SC-WATBAL-001#INV-WATBAL-083`.
- Updated contract revision histories to `SC-SNOWFREEZE-001` version `39` and
  `SC-WATBAL-001` version `132`.

Ran:

- `cargo test --test hphys0310_prior_day_snow_carry_divergence_contract -- --nocapture`
  passed.
