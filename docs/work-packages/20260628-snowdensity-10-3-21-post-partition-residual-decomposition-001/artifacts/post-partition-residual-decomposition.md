# SNOWDENSITY-10.3.21 Post-Partition Residual Decomposition

Evidence mode: Static diagnostic over reused real WAT/trace evidence.

- Disposition: `DIAGNOSTIC-COMPLETE-NO-PROMOTION-NO-FROST-DECISION`
- Current default robust fail/score: `15` / `179`
- Pre-partition activated robust fail/score: `17` / `172`
- Dominant failed signature: `seasonal_densification_trajectory`
- Dominant component class: `density`
- Site concentration: `diffuse_by_site`
- Under-persistence read: `under_persistence_tail_present_not_sole_constraint`
- Production/frost/default changes authorized: `false`

## Robust Fail Clusters

| Cluster | Fails | Direction | Mechanism signal | Read |
|---|---:|---|---|---|
| density_trajectory_diffuse | 9 | mixed_density_shape | irreducible_or_new_density_structure | Densification trajectory is the dominant residual signature and is diffuse across SNOTEL and cancov sites. Because Anderson/SNOBAL/CoE density variants have been exhausted, this does not point to another in-family density lever. |
| mountain_under_persistence_timing | 4 | under_persistence | wind_redistribution_or_forcing_representativeness | All robust timing failures are early modeled peak or meltout dates. This keeps an under-persistence tail, but it is not an over-persistence guardrail blocker and is concentrated in mountain SNOTEL timing cells. |
| cancov_depth_swe_slope_geometry | 2 | over_depth_per_swe | canopy_snow_interception_or_subcanopy_longwave | The depth-SWE slope failures are confined to the humid-New-England paired set. They are frost-relevant because snow depth insulates frost, but the open-stratum member keeps this from being a pure canopy-only diagnosis. |

## Robust Fail Rows

| Site | Climate | Cell | Component | Direction | Mechanism signal |
|---|---|---|---|---|---|
| harvard_hardwood | humid_new_england_hardwood | `seasonal_densification_trajectory` | density | density_shape_over_beta | irreducible_or_new_density_structure |
| marcell_conifer | laurentian_continental_conifer | `seasonal_densification_trajectory` | density | density_shape_over_beta | irreducible_or_new_density_structure |
| marcell_deciduous | laurentian_continental_deciduous | `seasonal_densification_trajectory` | density | density_shape_over_beta | irreducible_or_new_density_structure |
| marcell_open | laurentian_continental_open | `seasonal_densification_trajectory` | density | density_shape_over_beta | irreducible_or_new_density_structure |
| snotel_css_lab_ca | sierra_maritime | `seasonal_densification_trajectory` | density | density_shape_under_beta | irreducible_or_new_density_structure |
| snotel_mica_creek_st_joe_id | northern_rockies_intermountain | `seasonal_densification_trajectory` | density | density_shape_under_beta | irreducible_or_new_density_structure |
| snotel_niwot_co | front_range_continental | `seasonal_densification_trajectory` | density | density_shape_under_beta | irreducible_or_new_density_structure |
| snotel_paradise_wa | cascades_maritime | `seasonal_densification_trajectory` | density | density_shape_under_beta | irreducible_or_new_density_structure |
| snotel_snowbird_ut | wasatch_intermountain | `seasonal_densification_trajectory` | density | density_shape_under_beta | irreducible_or_new_density_structure |
| harvard_hardwood | humid_new_england_hardwood | `seasonal_depth_swe_slope` | depth_density_geometry | over_depth_per_swe | canopy_snow_interception_or_subcanopy_longwave |
| harvard_open | humid_new_england_open | `seasonal_depth_swe_slope` | depth_density_geometry | over_depth_per_swe | canopy_snow_interception_or_subcanopy_longwave |
| snotel_mica_creek_st_joe_id | northern_rockies_intermountain | `seasonal_ablation_meltout_date` | timing | under_persistence | wind_redistribution_or_forcing_representativeness |
| snotel_paradise_wa | cascades_maritime | `seasonal_ablation_meltout_date` | timing | under_persistence | wind_redistribution_or_forcing_representativeness |
| snotel_niwot_co | front_range_continental | `seasonal_peak_swe_date` | timing | under_persistence | wind_redistribution_or_forcing_representativeness |
| snotel_snowbird_ut | wasatch_intermountain | `seasonal_peak_swe_date` | timing | under_persistence | wind_redistribution_or_forcing_representativeness |

## Frost-Threshold Input

- Threshold input read: `MIXED-NO-SINGLE-GLOBAL-SNOW-LEVER: the residual is not zero, but it is mostly density-structure plus under-persistent mountain timing, with only a small mechanism-coherent cancov forest geometry cluster. This supports an operator frost-threshold decision rather than an automatic snow promotion or automatic frost unblock.`
- Decision made: `False`

Snow good-enough evidence:
- Current no-env default is 15/179 and beats the legacy flag profile 16/176.
- The SNOBAL/CoE/Anderson mechanism family has no remaining promoted candidate.
- Residual fails are diffuse by site; no site has more than two robust fails.
- No over-persistence timing tail remains after partition.

One-more-lever evidence:
- Humid-New-England depth-SWE slope failures point to a possible canopy/sub-canopy class.
- Mountain timing under-persistence points to wind redistribution or forcing/representativeness limits.
- Density trajectory failures dominate count, but explored density-family variants did not recover them.

Forcing-limited absolute SWE/depth magnitude cells are report-only under INV-SNOWFREEZE-050. Legacy and PySnobal profiles remain ADR-0017 flags, not targets.
