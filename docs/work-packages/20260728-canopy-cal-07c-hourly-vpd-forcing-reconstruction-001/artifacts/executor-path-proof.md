# CAL-07C Executor Path Proof

Evidence class: `Ran`

## Path map

| Layer | Evidence | Disposition |
| --- | --- | --- |
| Source producer | `artifacts/admission-table.csv` and `inputs/forcing.csv` | Alerce rows carry admitted hourly-product daily-mean `vpd_pa`; Beza rows carry unchanged CAL-07 daily-summary `vpd_pa`. |
| Package-local executor schema | `tools/executor/src/main.rs` `read_forcing` | Reads `vpd_pa` from forcing column 9 and `vpd_source` from column 10; fails on nonfinite or negative admitted VPD. |
| GSI handoff | `tools/executor/src/main.rs` `GsiDailyForcing` construction | Passes `vapor_pressure_deficit_pa: day.vpd` directly into `ForestCanopyState::advance`. |
| Output surface | `artifacts/daily-kernel-output.csv` | Publishes `vpd_pa` and `vpd_source` for every member/site/day row. |
| Independent validator | `tools/validate.py` | Reconstructs source VPD from retained POWER JSON and proves output VPD equals admitted forcing with max residual `0.000e+00 Pa`. |

## Negative proof for old path

CAL-07's original failure path computed Alerce VPD from frozen daily
`Tmax/Tmin/Tdew` under OBL-PLANT-P-013. CAL-07C does not use that rejected
daily-summary operand for Alerce. The validator proves the three rejected
negative dates are exactly `2022-07-22`, `2022-09-15`, and `2025-09-09`, and
that the consumed CAL-07C Alerce operand is the nonnegative hourly-product
daily mean.

Beza remains intentionally unchanged and continues to use the daily-summary
operator. No production runner, science contract, fixture, or CAL-07/CAL-07B
artifact was modified.

## Consumer-path boundary

The package-local executor is not a production cutover claim. The focused
consumer check
`cargo test -p openwepp-runner --lib native_forest_yaml_executes_through_the_direct_production_consumer -- --nocapture`
passed and supports the existing direct-production ordering evidence, not a
new CAL-07C production forcing operator.
