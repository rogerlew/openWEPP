# SNOWDENSITY-08 Snow/Frost Gate Rerun

Evidence class: Static + Ran.

- Disposition: `COMPLETE-08-SNOTEL-CLEARED-FROST-ATTRIBUTION-BLOCKED`
- SNOTEL density gate cleared: `True`
- Non-SNOTEL coupled opt-in WAT path: `False`
- Frost attribution authorized: `False`
- Blocker: `NON-SNOTEL-COUPLED-OPT-IN-WAT-PATH-ABSENT`
- Next route: Build an authorized diagnostic coupled opt-in WAT/publication path for non-SNOTEL frost fixtures, or keep frost attribution blocked while snow-depth control fails on the default path.

## SNOTEL Summary

| Model | Boundary | Robust fail | Robust score | Density fail | Density score | CoE SWE residual |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| `coe_bound_density_compaction_v1_legacy_coe` | `legacy_coe` | 5 | 107 | 5 | 40 | 4.441e-16 |
| `coe_bound_density_compaction_v1_coe_shortwave_albedo_v1` | `coe_shortwave_albedo_v1` | 5 | 110 | 5 | 41 | 4.441e-16 |

## Non-SNOTEL Summary

- Runtime: `direct-production-executor`
- Snow-control counts: `{'MODELED_SNOW_DEPTH_DIAGNOSTIC_PRESENT_NO_PAIRED_OBSERVED_SNOW': 2, 'SNOW_CONTROL_FAILED': 3}`
- Robust counts: `{'fail': 9, 'marginal': 7, 'pass': 4, 'strong': 20, 'unavailable': 45}`

| Site | Snow control | Robust counts |
| --- | --- | --- |
| `site1_sleepers_south_field_vt` | `SNOW_CONTROL_FAILED` | `{'fail': 2, 'marginal': 1, 'pass': 2, 'strong': 5, 'unavailable': 7}` |
| `site2_sleepers_w9_hardwood_vt` | `SNOW_CONTROL_FAILED` | `{'fail': 1, 'marginal': 2, 'pass': 1, 'strong': 6, 'unavailable': 7}` |
| `site4_ggd498_morris_mn` | `SNOW_CONTROL_FAILED` | `{'fail': 2, 'marginal': 2, 'pass': 1, 'strong': 5, 'unavailable': 7}` |
| `site3_scan_mandan_nd` | `MODELED_SNOW_DEPTH_DIAGNOSTIC_PRESENT_NO_PAIRED_OBSERVED_SNOW` | `{'fail': 3, 'marginal': 1, 'strong': 1, 'unavailable': 12}` |
| `site5_reynolds_creek_us_rls_id` | `MODELED_SNOW_DEPTH_DIAGNOSTIC_PRESENT_NO_PAIRED_OBSERVED_SNOW` | `{'fail': 1, 'marginal': 1, 'strong': 3, 'unavailable': 12}` |

## Disposition

SNOTEL density evidence remains promotion-candidate evidence for the opt-in lineage, but frost attribution stays blocked. The current non-SNOTEL WAT rerun is still the default `legacy_wepp` density path, and the repository intentionally has no parser/runfile/CLI selector that can produce a coupled opt-in frost-site WAT surface. Per `INV-SNOWFREEZE-061`, offline snow-only depth cannot be substituted for a coupled frost run.
