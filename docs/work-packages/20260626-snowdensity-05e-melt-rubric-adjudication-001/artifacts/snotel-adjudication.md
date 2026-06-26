# SNOWDENSITY-05E CoE Melt Adjudication

- Schema: `snowdensity05e-coe-melt-adjudication-v1`
- Contract: `SC-SNOWFREEZE-001 INV-SNOWFREEZE-050 INV-SNOWFREEZE-052 INV-SNOWFREEZE-055`
- Runtime coupling: `diagnostic snowbench replay only; no production activation`
- No site constants: `True`
- Disposition: `PROMOTION-CANDIDATE`
- Beats diagnostic legacy: `True`

## Model Summary

| Model | Paired rows | Robust fail | Robust score | Robust counts |
|---|---:|---:|---:|---|
| `legacy_coe` | 13590 | 13 | 61 | `{"fail": 13, "marginal": 14, "pass": 7, "strong": 11, "unavailable": 15}` |
| `coe_shortwave_albedo_v1` | 13590 | 10 | 84 | `{"fail": 10, "marginal": 8, "pass": 5, "strong": 22, "unavailable": 15}` |

## Site Summary

| Model | Site | Robust fail | Robust score | Robust counts |
|---|---|---:|---:|---|
| `legacy_coe` | `snotel_mica_creek_st_joe_id` | 2 | 12 | `{"fail": 2, "marginal": 3, "pass": 3, "strong": 1, "unavailable": 3}` |
| `legacy_coe` | `snotel_paradise_wa` | 5 | 10 | `{"fail": 5, "pass": 2, "strong": 2, "unavailable": 3}` |
| `legacy_coe` | `snotel_css_lab_ca` | 2 | 13 | `{"fail": 2, "marginal": 4, "strong": 3, "unavailable": 3}` |
| `legacy_coe` | `snotel_snowbird_ut` | 2 | 12 | `{"fail": 2, "marginal": 4, "pass": 1, "strong": 2, "unavailable": 3}` |
| `legacy_coe` | `snotel_niwot_co` | 2 | 14 | `{"fail": 2, "marginal": 3, "pass": 1, "strong": 3, "unavailable": 3}` |
| `coe_shortwave_albedo_v1` | `snotel_mica_creek_st_joe_id` | 1 | 16 | `{"fail": 1, "marginal": 3, "pass": 2, "strong": 3, "unavailable": 3}` |
| `coe_shortwave_albedo_v1` | `snotel_paradise_wa` | 4 | 13 | `{"fail": 4, "marginal": 1, "strong": 4, "unavailable": 3}` |
| `coe_shortwave_albedo_v1` | `snotel_css_lab_ca` | 1 | 18 | `{"fail": 1, "marginal": 3, "strong": 5, "unavailable": 3}` |
| `coe_shortwave_albedo_v1` | `snotel_snowbird_ut` | 3 | 16 | `{"fail": 3, "marginal": 1, "strong": 5, "unavailable": 3}` |
| `coe_shortwave_albedo_v1` | `snotel_niwot_co` | 1 | 21 | `{"fail": 1, "pass": 3, "strong": 5, "unavailable": 3}` |

## H Comparator Context

| Model | Robust fail | Robust score | Robust counts |
|---|---:|---:|---|
| `legacy_as_built` | 9 | 84 | `{"fail": 9, "marginal": 8, "pass": 8, "strong": 20, "unavailable": 15}` |
| `openwepp_as_built` | 9 | 84 | `{"fail": 9, "marginal": 8, "pass": 8, "strong": 20, "unavailable": 15}` |
| `pysnobal` | 28 | 11 | `{"fail": 28, "marginal": 6, "pass": 1, "strong": 1, "unavailable": 24}` |

Disposition rule: the opt-in path is a promotion candidate only if it reduces forcing-robust failures relative to diagnostic legacy without lowering the forcing-robust ordinal score. Observation-only disagreement does not create an `OPENWEPP-DEFECTIVE` label under ADR-0017.
