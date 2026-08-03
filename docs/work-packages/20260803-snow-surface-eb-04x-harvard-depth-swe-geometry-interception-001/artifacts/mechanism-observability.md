# Mechanism Observability

Status: bounded

Evidence mode: **Ran + Static**

| Operand / mechanism | Availability | Consequence |
|---|---|---|
| Ground snowfall accumulation | observed in retained traces | May compare paired common days; it is not intercepted-snow mass. |
| Ground pack SWE, depth, density | observed and algebraically closed | Supports model geometry and density-trajectory characterization. |
| Ground-pack sublimation | observed with latent identity | Supports only the implemented surface-pack process. |
| Longwave and latent energy | observed | Supports implemented component bookkeeping, subject to common-duration bounds. |
| Canopy snow load | `NOT_OBSERVED` | No canopy storage closure can be reconstructed. |
| Intercepted snowfall | `NOT_OBSERVED` | No interception magnitude or efficacy claim. |
| Canopy snow sublimation | `NOT_OBSERVED` | Ground sublimation cannot alias this operand. |
| Unloading / drip | `NOT_OBSERVED` | No timing or delivery attribution. |

B and L pair all `16,437` days. S pairs only 75 days because open continues
2,568 days beyond hardwood; LS pairs 29 days with four additional hardwood
days. S/LS totals are therefore common-prefix characterizations, not complete
seasonal factorial effects.

The B paired traces also differ slightly in total meteorological precipitation
(`hardwood - open = -0.0313 m`) and ground accumulation (`+0.1498 m`) over the
full record. The lanes are not an identical-forcing canopy manipulation, so the
residual mixes forcing/fixture and ground-snow response. It cannot identify
interception causally.
