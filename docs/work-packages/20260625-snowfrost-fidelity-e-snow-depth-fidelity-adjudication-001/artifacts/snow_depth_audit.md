# Snow-Depth Fidelity Audit

Evidence mode: Ran.

- Schema: `snowfreeze-observed-snow-depth-audit-v1`
- Contract: `SC-SNOWFREEZE-001 INV-SNOWFREEZE-048`
- Site count: `5`
- Route counts: `{'INSUFFICIENT-PAIRED-SNOW-DATA': 2, 'SNOW-DEPTH-FIDELITY-ISSUE': 3}`
- Direction counts: `{'dominant-modeled-over-observed': 3, 'no-paired-residuals': 2}`
- Next route: `snow-depth fidelity`
- Frost attribution authorized: `False`
- Qwet authorized: `False`

## Site Audit

| Site | Route | Direction | Pairs | Failures | Timing rescues | SWE alias better | Mean signed m | Median signed m | Max abs m | Reason |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| site1_sleepers_south_field_vt | SNOW-DEPTH-FIDELITY-ISSUE | dominant-modeled-over-observed | 384 | 322 | 4 | 298 | 0.41081660940626946 | 0.36666675782049574 | 1.596821792509187 | Like-for-like depth evidence is present, aliases/timing do not explain the failures, and TOL-SNOWFREEZE-009 fails. |
| site2_sleepers_w9_hardwood_vt | SNOW-DEPTH-FIDELITY-ISSUE | dominant-modeled-over-observed | 193 | 143 | 5 | 109 | 0.32327623539008665 | 0.29041936401336554 | 1.059919954616471 | Like-for-like depth evidence is present, aliases/timing do not explain the failures, and TOL-SNOWFREEZE-009 fails. |
| site3_scan_mandan_nd | INSUFFICIENT-PAIRED-SNOW-DATA | no-paired-residuals | 0 | 0 | 0 | 0 | n/a | n/a | n/a | No observed snow-depth rows are available for this site. |
| site4_ggd498_morris_mn | SNOW-DEPTH-FIDELITY-ISSUE | dominant-modeled-over-observed | 83 | 28 | 2 | 39 | 0.0672051635094675 | 0.0 | 0.392372927299844 | Like-for-like depth evidence is present, aliases/timing do not explain the failures, and TOL-SNOWFREEZE-009 fails. |
| site5_reynolds_creek_us_rls_id | INSUFFICIENT-PAIRED-SNOW-DATA | no-paired-residuals | 0 | 0 | 0 | 0 | n/a | n/a | n/a | No observed snow-depth rows are available for this site. |

## Disposition

Frost heat-flow, frozen-K, SFCC, impedance, and migration/fringe work remain unauthorized by these field residuals. The next authorized route is snow-depth fidelity unless correspondence blockers are reported above.
