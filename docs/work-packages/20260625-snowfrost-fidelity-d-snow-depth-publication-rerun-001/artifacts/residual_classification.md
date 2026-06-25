# Residual Classification

Evidence mode: Ran.

- Schema: `snowfreeze-observed-residual-classification-v1`
- Measurement contract: `SC-SNOWFREEZE-001 INV-SNOWFREEZE-047`
- Snow-control tolerance: `TOL-SNOWFREEZE-009`
- Site count: `5`
- Defect-attribution eligible sites: `0`
- `OPENWEPP-DEFECTIVE` sites: `0`

## Summary

- Primary classifications: `{'INCONCLUSIVE': 2, 'SNOW-CONTROL-FAILED': 3}`
- Residual families: `{'snow-confounded': 3, 'snow-control-unavailable': 2}`
- Next action: Where snow control passes, use SNOWFROST-FIDELITY-B/C evidence to adjudicate heat-flow versus frozen-K mechanisms. Where snow control fails or lacks paired rows, resolve snow-depth publication/physics before attributing frost residuals. No Qwet, SFCC, frozen-K, or heat-flow tuning is authorized by these classifications.

## Site Classifications

| Site | Harness | Primary | Family | Matched | Frost residuals | Max abs residual m | Isotherm exceedances | Snow pairs | Snow failures | Max snow residual m | Reason |
| --- | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| site1_sleepers_south_field_vt | UNRESOLVED | SNOW-CONTROL-FAILED | snow-confounded | 392 | 392 | 0.2641958258624707 | 0 | 384 | 322 | 1.596821792509187 | Modeled snow depth is available but fails paired TOL-SNOWFREEZE-009 snow-depth control; frost residuals remain snow-confounded. |
| site2_sleepers_w9_hardwood_vt | UNRESOLVED | SNOW-CONTROL-FAILED | snow-confounded | 200 | 200 | 0.3838127878666539 | 0 | 193 | 143 | 1.059919954616471 | Modeled snow depth is available but fails paired TOL-SNOWFREEZE-009 snow-depth control; frost residuals remain snow-confounded. |
| site3_scan_mandan_nd | UNRESOLVED | INCONCLUSIVE | snow-control-unavailable | 10643 | 0 | n/a | 3452 | 0 | 0 | n/a | Modeled snow depth is available, but this source has no paired observed snow-depth rows for TOL-SNOWFREEZE-009. |
| site4_ggd498_morris_mn | UNRESOLVED | SNOW-CONTROL-FAILED | snow-confounded | 83 | 83 | 0.990389751515789 | 0 | 83 | 28 | 0.392372927299844 | Modeled snow depth is available but fails paired TOL-SNOWFREEZE-009 snow-depth control; frost residuals remain snow-confounded. |
| site5_reynolds_creek_us_rls_id | UNRESOLVED | INCONCLUSIVE | snow-control-unavailable | 4356 | 0 | n/a | 104 | 0 | 0 | n/a | Modeled snow depth is available, but this source has no paired observed snow-depth rows for TOL-SNOWFREEZE-009. |

## Disposition

Defect attribution remains gated by paired snow-depth control. A site can move to heat-flow or frozen-K mechanism discrimination only after `TOL-SNOWFREEZE-009` passes; missing, unmatched, or failed snow-control rows remain snow-confounded and do not authorize heat flow, frozen conductivity, or migration heat tuning.
