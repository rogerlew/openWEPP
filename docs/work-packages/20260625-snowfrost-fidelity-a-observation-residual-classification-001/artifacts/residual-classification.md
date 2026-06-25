# Residual Classification

Evidence mode: Ran.

- Schema: `snowfreeze-observed-residual-classification-v1`
- Measurement contract: `SC-SNOWFREEZE-001 INV-SNOWFREEZE-047`
- Snow-control tolerance: `TOL-SNOWFREEZE-009`
- Site count: `5`
- Defect-attribution eligible sites: `0`
- `OPENWEPP-DEFECTIVE` sites: `0`

## Summary

- Primary classifications: `{'INCONCLUSIVE': 2, 'SNOW-CONTROL-BLOCKED': 3}`
- Residual families: `{'snow-confounded': 3, 'snow-control-missing': 2}`
- Next action: Add a modeled snow-depth diagnostic and rerun SNOWFROST-FIDELITY-A classification before field residuals are attributed to frost physics. No Qwet, SFCC, frozen-K, or heat-flow tuning is authorized by these classifications.

## Site Classifications

| Site | Harness | Primary | Family | Matched | Frost residuals | Max abs residual m | Isotherm exceedances | Snow rows | Reason |
| --- | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| site1_sleepers_south_field_vt | UNRESOLVED | SNOW-CONTROL-BLOCKED | snow-confounded | 392 | 392 | 0.2641958258624707 | 0 | 384 | Observed snow depth exists but modeled snow depth is absent; TOL-SNOWFREEZE-009 cannot be evaluated. |
| site2_sleepers_w9_hardwood_vt | UNRESOLVED | SNOW-CONTROL-BLOCKED | snow-confounded | 200 | 200 | 0.3838127878666539 | 0 | 193 | Observed snow depth exists but modeled snow depth is absent; TOL-SNOWFREEZE-009 cannot be evaluated. |
| site3_scan_mandan_nd | UNRESOLVED | INCONCLUSIVE | snow-control-missing | 10643 | 0 | n/a | 3452 | 0 | No modeled snow-depth diagnostic is available, and this source does not provide paired snow-depth rows. |
| site4_ggd498_morris_mn | UNRESOLVED | SNOW-CONTROL-BLOCKED | snow-confounded | 83 | 83 | 0.990389751515789 | 0 | 232 | Observed snow depth exists but modeled snow depth is absent; TOL-SNOWFREEZE-009 cannot be evaluated. |
| site5_reynolds_creek_us_rls_id | UNRESOLVED | INCONCLUSIVE | snow-control-missing | 4356 | 0 | n/a | 104 | 0 | No modeled snow-depth diagnostic is available, and this source does not provide paired snow-depth rows. |

## Disposition

No site is eligible for frost-model defect attribution in this pass. The direct harness produces metric-bearing reports, but modeled snow depth is absent, so `TOL-SNOWFREEZE-009` cannot be evaluated. Current field residuals are evidence for the next diagnostic gate, not authority to tune heat flow, frozen conductivity, or migration heat.
