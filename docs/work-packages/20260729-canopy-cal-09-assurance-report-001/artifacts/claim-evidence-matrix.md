# Claim-Evidence Matrix

Status: `FROZEN`

| Claim family | Evidence class | Retained direction | Prospective manuscript language |
| --- | --- | --- | --- |
| Indicator, GSI21, and signed-latitude implementation | Software and mathematical verification | Favorable | Internally verified within tested domains |
| Daily allocation, litter transfer, annual closure, and no drift | Conservation and state verification | Favorable | Supported implementation behavior |
| Real post-phenology downstream consumption | Integration and consumer verification | Favorable | Supported production-path lineage |
| Hubbard temperate timing calibration | Empirical calibration | Favorable but partially identifiable | Site-calibrated; parameter uniqueness unsupported |
| Harvard temperate timing transfer | Independent evaluation | Adverse | Transferability contradicted for the tested holdout |
| Litter-source and decomposition operator | Calibration readiness | Mixed/data-limited | Mechanically ready; empirical decomposition and predictive sources limited |
| Predictive evergreen needle and fine-woody sources | Science authority | Missing | Not evaluated / authority missing |
| Within-site winter canopy ordering | Model-response characterization | Favorable | Bounded congruence, not empirical canopy validation |
| Snow and frost response across canopy gradients | Empirical/model-response evaluation | Mixed | Bounded with forcing and identity limitations |
| ET, runoff, and erosion consequences | Model response or unavailable output | Limited | Bounded or not evaluated; no calibration authority |
| Synthetic Northern/Southern phase behavior | Mathematical/integration verification | Favorable | Hemisphere mechanics supported |
| Bezà tropical dry-forest chronology | Independent evaluation | Adverse | Contradicted; ecosystem-model limitation |
| Elliot numerical targets | Legacy comparison | Adverse/not reproducible | Not reproducible and not correctness authority |
| Elliot staged evaluation method | Conceptual methodology | Retained | Useful study sequence, not numerical validation |

## Stable Claim Bindings

| ID | Quantity/domain | Method and result | Verdict | Placement and boundary |
| --- | --- | --- | --- | --- |
| `CANOPY-C01` | Daily GSI, foliage, LAI, cover, height, allocation, litter, residue, and real-consumer chronology | Contract-derived equations, closure/state reconstruction, and production consumer lineage | `SUPPORTED` | Manuscript §§2, 4; tested contracts and realization only |
| `CANOPY-C02` | Hubbard/Harvard seasonal timing, days and interval coverage | 9,261-member Hubbard search; 37-member frozen ensemble; Harvard scored once | Hubbard `BOUNDED`; Harvard `CONTRADICTED` | Manuscript §5, F1/F4, T2/T3; no general transfer |
| `CANOPY-C03` | Surface-residue source and rate, kg m^-2 and d^-1 | Complete synthetic daily recovery and five-pair terminal ridge | `BOUNDED` | Manuscript §6, F3/F5; readiness only, no empirical decay/source law |
| `CANOPY-C04` | Winter cover ordering and downstream model response at Marcell, Harvard, and Hubbard Brook | 261 prespecified runs; 37 members per forest lane | `BOUNDED` | Manuscript §7, F6; no canopy-amplitude or downstream accuracy claim |
| `CANOPY-C05` | Bezà 2024-2025 relative GCC/GSI transition chronology, days | Product-consistent same-direction seasonal crossings for `gcc_mean` and `gcc_90` | `CONTRADICTED` | Manuscript §8, F8; ecosystem-model limitation, replacement unknown |
| `CANOPY-C06` | Elliot biomass, residue, hydrology, sediment, and recurrence targets | Authorized reconstruction under frozen tolerances | `CONTRADICTED` / `NOT_REPRODUCIBLE` | Manuscript §9, supplement T5; legacy is not correctness authority |

The integrated T4 matrix retains predictive needle/fine-woody source and
erosion consequence output as `NOT_EVALUATED`. Harvard SWE remains excluded
rather than treated as zero or reinterpreted.
