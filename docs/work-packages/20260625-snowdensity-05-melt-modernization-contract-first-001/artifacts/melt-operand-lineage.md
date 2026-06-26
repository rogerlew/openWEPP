# Melt Operand Lineage

Status: queued.
Evidence mode: not-run.

Fill before production edits.

| Operand | Units | Source authority | Runtime source | Authoritative or diagnostic | Rejected aliases |
|---|---|---|---|---|---|
| `hrmelt_raw` | `m` | `SC-SNOWFREEZE-001`, WEPP Ch. 3 | queued | authoritative | SWE delta, degree-day proxy |
| `amelt` | legacy inch-equivalent term | WEPP Ch. 3 + SNOWDENSITY-05 amendment | queued | authoritative term | tuned radiation forcing |
| `bmelt` | legacy inch-equivalent term | WEPP Ch. 3 + SNOWDENSITY-05 amendment | queued | authoritative term after sign/alias reconciliation | silent sign flip, double subtraction |
| `hrrad` / hourly shortwave | `MJ m^-2 h^-1` | `SC-CLIMATE-001#INV-CLIMATE-013` | queued | authoritative forcing | raw Langleys/day, fitted scalar |
| `snow_albedo` | fraction | Brock 2000 + SNOWDENSITY-05 amendment | queued | authoritative opt-in state | constant hidden default |
| `cancov` | fraction | WEPP Ch. 3 / plant state | queued | authoritative attenuation | density surrogate |
| `cmelt` | legacy inch-equivalent term | WEPP Ch. 3 + SNOWDENSITY-05 amendment | queued | authoritative term | degree-day melt factor |
| `wmelt` / routed melt | `m` | `INV-SNOWFREEZE-015/019/021/022` | queued | authoritative liquid forcing | raw precipitation, SWE delta |

Anti-tautology requirement: every test fixture must make at least one rejected
alias numerically different from the accepted operand path.
