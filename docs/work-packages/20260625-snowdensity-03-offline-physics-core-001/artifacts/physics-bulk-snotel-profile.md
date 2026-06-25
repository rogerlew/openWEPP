# SNOWDENSITY-03 Physics-Bulk SNOTEL Profile

- Schema: `snowdensity03-physics-bulk-snotel-profile-v1`
- Model: `physics_bulk`
- Contract: `SC-SNOWFREEZE-001 INV-SNOWFREEZE-050 INV-SNOWFREEZE-051 OBL-SNOWFREEZE-P-026`
- Runtime coupling: `none; offline snowbench candidate only`
- No site constants: `True`

| Site | Paired rows | Robust counts | All counts |
|---|---:|---|---|
| `snotel_mica_creek_st_joe_id` | 2540 | `{"fail": 5, "marginal": 2, "pass": 2, "unavailable": 3}` | `{"fail": 7, "marginal": 4, "pass": 2, "unavailable": 3}` |
| `snotel_paradise_wa` | 3170 | `{"fail": 5, "marginal": 3, "strong": 1, "unavailable": 3}` | `{"fail": 8, "marginal": 4, "strong": 1, "unavailable": 3}` |
| `snotel_css_lab_ca` | 1744 | `{"fail": 4, "marginal": 2, "pass": 1, "strong": 2, "unavailable": 3}` | `{"fail": 7, "marginal": 3, "pass": 1, "strong": 2, "unavailable": 3}` |
| `snotel_snowbird_ut` | 2754 | `{"fail": 5, "marginal": 3, "strong": 1, "unavailable": 3}` | `{"fail": 8, "marginal": 4, "strong": 1, "unavailable": 3}` |
| `snotel_niwot_co` | 3382 | `{"fail": 5, "marginal": 3, "strong": 1, "unavailable": 3}` | `{"fail": 8, "marginal": 4, "strong": 1, "unavailable": 3}` |

Disposition: profile evidence only. Failures remain `UNRESOLVED` under ADR-0017 until SNOWDENSITY-04 adjudicates whether in-envelope changes improve forcing-robust cells without site tuning.
