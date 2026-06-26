# SNOWDENSITY-05G CoE Melt Adjudication

- Schema: `snowdensity05g-coe-melt-adjudication-v1`
- Contract: `SC-SNOWFREEZE-001 INV-SNOWFREEZE-050 INV-SNOWFREEZE-052 INV-SNOWFREEZE-055 INV-SNOWFREEZE-057`
- Runtime coupling: `diagnostic snowbench replay only; no production activation`
- No site constants: `True`
- Disposition: `NON-PROMOTION`
- Beats diagnostic legacy: `False`

## Model Summary

| Model | Paired rows | Robust fail | Robust score | Robust counts |
|---|---:|---:|---:|---|
| `legacy_coe` | 13590 | 9 | 84 | `{"fail": 9, "marginal": 8, "pass": 8, "strong": 20, "unavailable": 15}` |
| `coe_shortwave_albedo_v1` | 13590 | 9 | 86 | `{"fail": 9, "marginal": 8, "pass": 6, "strong": 22, "unavailable": 15}` |

## Site Summary

| Model | Site | Robust fail | Robust score | Robust counts |
|---|---|---:|---:|---|
| `legacy_coe` | `snotel_mica_creek_st_joe_id` | 1 | 15 | `{"fail": 1, "marginal": 3, "pass": 3, "strong": 2, "unavailable": 3}` |
| `legacy_coe` | `snotel_paradise_wa` | 4 | 15 | `{"fail": 4, "strong": 5, "unavailable": 3}` |
| `legacy_coe` | `snotel_css_lab_ca` | 1 | 17 | `{"fail": 1, "marginal": 3, "pass": 1, "strong": 4, "unavailable": 3}` |
| `legacy_coe` | `snotel_snowbird_ut` | 2 | 16 | `{"fail": 2, "marginal": 2, "pass": 1, "strong": 4, "unavailable": 3}` |
| `legacy_coe` | `snotel_niwot_co` | 1 | 21 | `{"fail": 1, "pass": 3, "strong": 5, "unavailable": 3}` |
| `coe_shortwave_albedo_v1` | `snotel_mica_creek_st_joe_id` | 1 | 15 | `{"fail": 1, "marginal": 3, "pass": 3, "strong": 2, "unavailable": 3}` |
| `coe_shortwave_albedo_v1` | `snotel_paradise_wa` | 4 | 15 | `{"fail": 4, "strong": 5, "unavailable": 3}` |
| `coe_shortwave_albedo_v1` | `snotel_css_lab_ca` | 1 | 18 | `{"fail": 1, "marginal": 3, "strong": 5, "unavailable": 3}` |
| `coe_shortwave_albedo_v1` | `snotel_snowbird_ut` | 2 | 17 | `{"fail": 2, "marginal": 2, "strong": 5, "unavailable": 3}` |
| `coe_shortwave_albedo_v1` | `snotel_niwot_co` | 1 | 21 | `{"fail": 1, "pass": 3, "strong": 5, "unavailable": 3}` |

## H Comparator Context

| Model | Robust fail | Robust score | Robust counts |
|---|---:|---:|---|
| `legacy_as_built` | 9 | 84 | `{"fail": 9, "marginal": 8, "pass": 8, "strong": 20, "unavailable": 15}` |
| `openwepp_as_built` | 9 | 84 | `{"fail": 9, "marginal": 8, "pass": 8, "strong": 20, "unavailable": 15}` |
| `pysnobal` | 28 | 11 | `{"fail": 28, "marginal": 6, "pass": 1, "strong": 1, "unavailable": 24}` |

Disposition rule: the opt-in path is a promotion candidate only if it reduces forcing-robust failures relative to diagnostic legacy without lowering the forcing-robust ordinal score. Observation-only disagreement does not create an `OPENWEPP-DEFECTIVE` label under ADR-0017.
