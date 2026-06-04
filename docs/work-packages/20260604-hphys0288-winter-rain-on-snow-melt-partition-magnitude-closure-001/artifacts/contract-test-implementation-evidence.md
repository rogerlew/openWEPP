# Contract Test Implementation Evidence

Status: complete
Evidence mode: Static + Ran

Static:
- Added `tests/integration/hphys0288_winter_rain_snowmelt_partition_contract.rs`.
- Registered the test in `Cargo.toml`.
- The vector initializes a `0.100 m`, `340 kg m^-3` snowpack and one `0.003 m` rain-on-snow hour.
- Contract expectation: `0.001 m` is retained to reach `350 kg m^-3`; `0.002 m` residual is published through final `snow.hourly.melt_m_0001`/`wmelt` and offered to WB12 infiltration; signed `S` remains `-0.001 m` for retained snow-storage gain.

Ran:
- `cargo test --test hphys0288_winter_rain_snowmelt_partition_contract -- --nocapture`
- Result before production edits: failed as expected.
- Failure: `released rain-on-snow must be routed through final hrmlt/wmelt; observed 0`.
