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
- Next action: Where snow control passes, use SNOWFROST-FIDELITY-B/C evidence to adjudicate heat-flow versus frozen-K mechanisms. Where snow control fails or lacks paired rows, apply SC-SNOWFREEZE-001 INV-SNOWFREEZE-048 snow-depth correspondence and anti-alias checks before attributing frost residuals. No Qwet, SFCC, frozen-K, or heat-flow tuning is authorized by these classifications.

## Site Classifications

| Site | Harness | Primary | Family | Matched | Frost residuals | Max abs residual m | Isotherm exceedances | Snow pairs | Snow failures | Mean signed snow m | Over | Under | Timing rescues | SWE alias better | Max snow residual m | Reason |
| --- | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| site1_sleepers_south_field_vt | UNRESOLVED | SNOW-CONTROL-FAILED | snow-confounded | 392 | 392 | 0.24686706611591228 | 0 | 384 | 218 | 0.1560690183510679 | 315 | 56 | 25 | 188 | 0.7897087938117313 | Modeled snow depth is available but fails paired TOL-SNOWFREEZE-009 snow-depth control; frost residuals remain snow-confounded. |
| site2_sleepers_w9_hardwood_vt | UNRESOLVED | SNOW-CONTROL-FAILED | snow-confounded | 200 | 200 | 0.390313712420475 | 0 | 193 | 72 | 0.08137700789858286 | 134 | 54 | 15 | 35 | 0.5825943749751785 | Modeled snow depth is available but fails paired TOL-SNOWFREEZE-009 snow-depth control; frost residuals remain snow-confounded. |
| site3_scan_mandan_nd | UNRESOLVED | INCONCLUSIVE | snow-control-unavailable | 10643 | 0 | n/a | 3658 | 0 | 0 | n/a | 0 | 0 | 0 | 0 | n/a | Modeled snow depth is available, but this source has no paired observed snow-depth rows for TOL-SNOWFREEZE-009. |
| site4_ggd498_morris_mn | UNRESOLVED | SNOW-CONTROL-FAILED | snow-confounded | 83 | 83 | 0.7868649477045688 | 0 | 83 | 20 | 0.04400065299290993 | 39 | 3 | 3 | 35 | 0.22323340797447816 | Modeled snow depth is available but fails paired TOL-SNOWFREEZE-009 snow-depth control; frost residuals remain snow-confounded. |
| site5_reynolds_creek_us_rls_id | UNRESOLVED | INCONCLUSIVE | snow-control-unavailable | 4356 | 0 | n/a | 125 | 0 | 0 | n/a | 0 | 0 | 0 | 0 | n/a | Modeled snow depth is available, but this source has no paired observed snow-depth rows for TOL-SNOWFREEZE-009. |

## Disposition

Defect attribution remains gated by paired snow-depth control. A site can move to heat-flow or frozen-K mechanism discrimination only after `TOL-SNOWFREEZE-009` passes; missing, unmatched, or failed snow-control rows remain snow-confounded and do not authorize heat flow, frozen conductivity, or migration heat tuning. Failed snow-control rows must first pass `INV-SNOWFREEZE-048` correspondence and anti-alias adjudication.
