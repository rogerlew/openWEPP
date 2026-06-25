# Legacy Snow Comparison

Evidence mode: Ran.

- Schema: `snowfreeze-legacy-snow-comparison-v1`
- Contract: `SC-SNOWFREEZE-001 INV-SNOWFREEZE-048`
- Legacy baseline: `/home/workdir/wepp-forest_260430_baseline`
- Runtime: `direct-production-executor`
- Site count: `5`
- Route counts: `{'BOTH-FAIL-LEGACY-CLOSER-FLAG': 2, 'BOTH-FAIL-OPENWEPP-CLOSER-FLAG': 1, 'NO-PAIRED-OBSERVED-SNOW-DEPTH': 2}`
- Legacy closer by mean absolute observed-depth residual: `['site1_sleepers_south_field_vt', 'site4_ggd498_morris_mn']`
- openWEPP closer by mean absolute observed-depth residual: `['site2_sleepers_w9_hardwood_vt']`
- Legacy correctness target: `False`

## Site Summary

| Site | Route | Obs pairs | openWEPP mean abs depth m | Legacy mean abs depth m | openWEPP failures | Legacy failures | Legacy better rows | openWEPP better rows | Depth delta mean abs m | SWE delta mean abs m |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| site1_sleepers_south_field_vt | BOTH-FAIL-LEGACY-CLOSER-FLAG | 384 | 0.41459 | 0.319415 | 322 | 296 | 249 | 131 | 0.0448288 | 0.013429 |
| site2_sleepers_w9_hardwood_vt | BOTH-FAIL-OPENWEPP-CLOSER-FLAG | 193 | 0.348239 | 0.371766 | 143 | 148 | 56 | 136 | 0.0107974 | 0.00293952 |
| site4_ggd498_morris_mn | BOTH-FAIL-LEGACY-CLOSER-FLAG | 83 | 0.0687163 | 0.0533434 | 28 | 23 | 27 | 18 | 0.00690632 | 0.0019001 |
| site3_scan_mandan_nd | NO-PAIRED-OBSERVED-SNOW-DEPTH | 0 | n/a | n/a | 0 | 0 | 0 | 0 | 0.0025994 | 0.000664768 |
| site5_reynolds_creek_us_rls_id | NO-PAIRED-OBSERVED-SNOW-DEPTH | 0 | n/a | n/a | 0 | 0 | 0 | 0 | 0.00452109 | 0.00122156 |

## Capture Lineage

- Legacy SWE is parsed from normal WAT `Snow-Water` rows.
- Legacy physical snow depth is parsed from dated daily-winter hour-24 rows produced by a temporary replay with the existing `.run` daily-winter answer changed from `No` to `Yes`.
- Legacy large graphics is also enabled; `treal(73)=snodpy*1000` and `treal(75)=densg` prove the same physical operands exist there, but that output is sparse for these hillslope fixtures and is not used for date-aligned observed comparisons.
- Legacy snow density is parsed from dated daily-winter hour-24 rows to support depth/SWE anti-alias review.
- Legacy agreement remains flag evidence under ADR-0017; observed physical snow depth and `INV-SNOWFREEZE-048` remain the correspondence authority.
