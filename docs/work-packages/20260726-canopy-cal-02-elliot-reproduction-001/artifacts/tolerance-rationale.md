# Tolerance Rationale

Evidence class: `Static pre-score declaration`

Tolerances were frozen before the five-arm result score.

| Target class | Tolerance | Rationale |
| --- | --- | --- |
| exact input, derived input, executable, or retained output identity | byte identity / SHA-256 equality | identity claim |
| daily plant/residue pool sum | absolute `1e-12 kg/m2` on parsed decimal values | independent arithmetic, not model agreement |
| report chart live biomass or residue stock | larger of `0.25 kg/m2` or 2% of target | plotted line thickness and axis reading resolution |
| report chart LAI | `0.15 m2/m2` | chart resolution; management cap remains exact input |
| report annual runoff or sediment table | larger of report rounding unit or 2% | report rows are rounded and historical Windows project is absent |
| report return-period table | larger of report rounding unit or 5% | plotted-position and rounded-table sensitivity |
| equilibrium | years 91--100 year-end range no larger than 2% of its mean | practical numerical equilibrium, not exact steady state |

These tolerances score reproduction only. They do not turn Bill-derived
assumptions into field authority, compensate a failed process result with a
hydrology result, or compare hillslope surface runoff with watershed/channel
flow.
