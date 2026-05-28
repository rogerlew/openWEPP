# hillstab01-stability-report

Status: complete  
Evidence mode: Ran

## Run Provenance
- openWEPP binary:
  `/home/workdir/openWEPP/target/release/openwepp-cli-hill`
- 1166 cohort source:
  `/workdir/wepp-forest/docs/work-packages/20260503-wb05b-forest-hillslope-closure-sweep/artifacts/audits/_meta/defect_seeds.csv`
- release-gate watchlist source:
  `/workdir/wepp-forest/docs/ablation/hillslope_watchlist.csv`
- structured output:
  `artifacts/hillstab01-stability-results.json`
- temporary run workspace:
  `/tmp/hillstab01`

## Cohort Summary
- `wb05b_1166`
  - total: 1166
  - passed: 0
  - failed: 1166
  - mean elapsed: 0.034589 s
  - median elapsed: 0.004806 s
  - max elapsed: 0.990579 s
- `release_gate_watchlist`
  - total: 19
  - passed: 0
  - failed: 19
  - mean elapsed: 0.112523 s
  - median elapsed: 0.041443 s
  - max elapsed: 0.485711 s

## Dominant Failure Families
### 1166 cohort
- `830` cases: `CLIHILL-E-010|SOL-E-006`
  - soil parser variant arity mismatch (legacy `.sol` forms incompatible with
    current parser expectations).
- `137` cases:
  `CLIHILL-E-011|HS-SIMPIPE-E-001|HKERNEL-WB16-PEAK-E-003`
- `93` cases: `CLIHILL-E-010|MAN-E-009`
  - management parser reference domain violations.
- `38` cases:
  `CLIHILL-E-011|HS-SIMPIPE-E-001|HKERNEL-EROD14-WAVE2-E-003`

### Release-gate watchlist
- `13` cases: `CLIHILL-E-010|SOL-E-006`
- `3` cases:
  `CLIHILL-E-011|HS-SIMPIPE-E-001|HKERNEL-WB16-PEAK-E-003`
- `2` cases:
  `CLIHILL-E-011|HS-SIMPIPE-E-001|HKERNEL-EROD14-WAVE2-E-003`
- `1` case: `CLIHILL-E-010` (slope endpoint constraint failure)

## Verdict
- Stability gate result: **HOLD**.
- Broad cohorts are currently non-runnable/guard-failing under current
  `openwepp-cli-hill` parser/kernel guard surfaces for these legacy inputs.
