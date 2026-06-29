# Snow-Depth Fidelity Audit

Evidence mode: Ran.

- Schema: `snowfreeze-observed-snow-depth-audit-v1`
- Contract: `SC-SNOWFREEZE-001 INV-SNOWFREEZE-048`
- Site count: `5`
- Route counts: `{'INSUFFICIENT-PAIRED-SNOW-DATA': 2, 'SNOW-DEPTH-FIDELITY-ISSUE': 3}`
- Direction counts: `{'dominant-modeled-over-observed': 2, 'mixed': 1, 'no-paired-residuals': 2}`
- Next route: `snow-depth fidelity`
- Frost attribution authorized: `False`
- Qwet authorized: `False`

## Site Audit

| Site | Route | Direction | Pairs | Failures | Timing rescues | SWE alias better | Mean signed m | Median signed m | Max abs m | Reason |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| site1_sleepers_south_field_vt | SNOW-DEPTH-FIDELITY-ISSUE | dominant-modeled-over-observed | 384 | 218 | 25 | 188 | 0.15606590909970983 | 0.13661465153804614 | 0.7897087938117313 | Like-for-like depth evidence is present, aliases/timing do not explain the failures, and TOL-SNOWFREEZE-009 fails. |
| site2_sleepers_w9_hardwood_vt | SNOW-DEPTH-FIDELITY-ISSUE | mixed | 193 | 72 | 15 | 35 | 0.08137700789858286 | 0.07135798002990856 | 0.5825943749751785 | Like-for-like depth evidence is present, aliases/timing do not explain the failures, and TOL-SNOWFREEZE-009 fails. |
| site3_scan_mandan_nd | INSUFFICIENT-PAIRED-SNOW-DATA | no-paired-residuals | 0 | 0 | 0 | 0 | n/a | n/a | n/a | No observed snow-depth rows are available for this site. |
| site4_ggd498_morris_mn | SNOW-DEPTH-FIDELITY-ISSUE | dominant-modeled-over-observed | 83 | 20 | 3 | 35 | 0.04400065299290993 | 0.0 | 0.22323340797447816 | Like-for-like depth evidence is present, aliases/timing do not explain the failures, and TOL-SNOWFREEZE-009 fails. |
| site5_reynolds_creek_us_rls_id | INSUFFICIENT-PAIRED-SNOW-DATA | no-paired-residuals | 0 | 0 | 0 | 0 | n/a | n/a | n/a | No observed snow-depth rows are available for this site. |

## Disposition

Frost heat-flow, frozen-K, SFCC, impedance, and migration/fringe work remain unauthorized by these field residuals. The next authorized route is snow-depth fidelity unless correspondence blockers are reported above.
