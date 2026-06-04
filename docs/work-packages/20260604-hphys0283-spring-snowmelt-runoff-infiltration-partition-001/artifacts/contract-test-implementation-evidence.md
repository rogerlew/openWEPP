# Contract-Test Implementation Evidence

Status: complete
Evidence mode: Static + Ran

## Static: Test Added

- Added `tests/integration/hphys0283_snowmelt_infiltration_partition_contract.rs`.
- Registered it in `Cargo.toml` as `hphys0283_snowmelt_infiltration_partition_contract`.

## Ran: Red/Green Evidence

- Pre-production gate: the new test failed before the production runoff/infiltration fix because `wb12_infiltration` remained zero for a snowmelt-only event.
- Post-production gate:
  - `cargo test --test hphys0283_snowmelt_infiltration_partition_contract -- --nocapture`
  - Result: `1 passed; 0 failed`.

## Assertions Covered

- Positive controlled snowmelt is generated.
- Snowmelt is offered to same-pass infiltration.
- Sufficient-capacity snowmelt event does not become runoff.
- WB18 layer storage and aggregate `wb11_soil_water` increase from infiltrated melt.
