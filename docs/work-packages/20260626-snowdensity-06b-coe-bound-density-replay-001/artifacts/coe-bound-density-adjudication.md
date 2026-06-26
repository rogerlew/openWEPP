# SNOWDENSITY-06B CoE-Bound Density Adjudication

- Schema: `snowdensity06b-coe-bound-density-adjudication-v1`
- Disposition: `PROMOTION-CANDIDATE`
- Best model: `coe_bound_density_compaction_v1_coe_shortwave_albedo_v1`
- Best boundary: `coe_shortwave_albedo_v1`
- Beats openWEPP as-built: `True`
- Beats legacy as-built: `True`
- Runtime coupling: `none; offline CoE-bound density replay only`
- No site constants: `True`

## Comparator Summary

| Model | Robust fail | Robust score | Density fail | Density score | Robust counts |
|---|---:|---:|---:|---:|---|
| `openwepp_as_built` | 9 | 84 | 9 | 16 | `{"fail": 9, "marginal": 8, "pass": 8, "strong": 20, "unavailable": 15}` |
| `legacy_as_built` | 9 | 84 | 9 | 16 | `{"fail": 9, "marginal": 8, "pass": 8, "strong": 20, "unavailable": 15}` |
| `pysnobal` | 28 | 11 | 12 | 4 | `{"fail": 28, "marginal": 6, "pass": 1, "strong": 1, "unavailable": 24}` |

## Candidate Summary

| Boundary | Model | Robust fail | Robust score | Density fail | Density score | Max CoE SWE residual | Max unbounded SWE residual | vs openWEPP | vs legacy |
|---|---|---:|---:|---:|---:|---:|---:|---|---|
| `legacy_coe` | `coe_bound_density_compaction_v1_legacy_coe` | 5 | 107 | 5 | 40 | 4.441e-16 | 1.285e-01 | `{"better": 13, "equal": 30, "unpaired": 0, "worse": 2}` | `{"better": 14, "equal": 28, "unpaired": 0, "worse": 3}` |
| `coe_shortwave_albedo_v1` | `coe_bound_density_compaction_v1_coe_shortwave_albedo_v1` | 5 | 110 | 5 | 41 | 4.441e-16 | 1.285e-01 | `{"better": 14, "equal": 30, "unpaired": 0, "worse": 1}` | `{"better": 15, "equal": 28, "unpaired": 0, "worse": 2}` |

Disposition rule: a 06B candidate must improve whole-rubric and density-cell profiles against both openWEPP as-built and legacy as-built while preserving CoE SWE identity. Failure is non-promotion evidence, not authorization to retune melt, canopy, albedo, radiation, or site constants.
