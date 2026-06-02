# HPHYS0246 Pre Implementation Contract Gate

Status: completed
Evidence mode: Ran

## Command
- `cargo test -p openwepp-hillslope-orchestrator hphys0246_wb18 -- --nocapture`

## Result
- Exit code: `101`
- Expected failure: yes

## Observed Target Failures
- Residual-storage preservation failed:
  - observed `wb11_soil_water = 0.30000000000000004`
  - expected baseline aggregate `0.3430000000000001`
- Missing residual input did not fail closed:
  - observed status `HKERNEL-WB11-PERC-OK-001`
  - expected status `HKERNEL-WB11-PERC-E-001`

## Interpretation
- The gate demonstrated the pre-HPHYS0246 WB18 implementation was publishing
  aggregate soil water from `Σtheta` and did not require residual/dead-water
  symbols.
