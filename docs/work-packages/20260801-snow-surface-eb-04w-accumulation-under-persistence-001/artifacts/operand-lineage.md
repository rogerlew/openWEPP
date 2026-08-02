# Operand And Closure Lineage

Evidence mode: **Static + Ran**.

| Surface | Units | Exact producer/derivation | Required consumer interpretation |
|---|---|---|---|
| active precipitation total | `m` water depth | authoritative SIMIMPL28 pre-partition hourly operand, preserved before rain/snow classification | independent source total for phase-amount reconstruction; never recompute as the published rain/snow sum |
| hourly rain | `m` water depth | `DirectWinterHourlyForcing::rain_m` | phase input, before pack retention |
| hourly snowfall depth | `m` physical snow | `DirectWinterHourlyForcing::snowfall_m` | never alias to SWE |
| hourly snowfall SWE | `m` water equivalent | snowfall depth multiplied by the baseline `0.1` water-equivalent ratio | sums to daily `accumulation_m` |
| rain/snow fractions | dimensionless | selected SIMIMPL28 phase model | active precipitation closure only |
| hydrometeor temperature | `degC`, optional | Harder-Pomeroy solve when applicable | phase diagnostic; null under models without this state |
| `amelt` | `m` water equivalent | exact Chapter 3/CoE radiation contribution converted from legacy inches | empirical formula contribution, not pure shortwave energy |
| `bmelt` | `m` water equivalent | exact temperature/cloud contribution converted from legacy inches | mixed empirical contribution, not pure sensible heat |
| `cmelt` | `m` water equivalent | exact wind/dewpoint/canopy/temperature contribution converted from legacy inches | mixed empirical contribution, not pure turbulent/sensible heat |
| `dmelt` | `m` water equivalent | exact rain/temperature contribution converted from legacy inches | empirical rain-heat melt-depth contribution |
| uncapped melt | `m` water equivalent | conversion of the existing sum `amelt+bmelt+cmelt+dmelt` | preserves mutation arithmetic |
| cap adjustment | `m` water equivalent | applied raw melt minus uncapped melt | separate pack-availability boundary |
| modeled wind redistribution | `m` water equivalent | exactly zero; process inactive under `INV-SNOWFREEZE-008` | model status only; physical redistribution remains unknown |

Closure requirements reconstruct the preserved pre-partition active total from
hourly rain plus snowfall SWE and independently reconstruct both amounts from
that total and their phase fractions. Daily accumulation equals the hourly
snowfall-SWE sum; uncapped melt equals the four exact component terms; and
applied raw melt equals uncapped melt plus cap adjustment, each within
`1e-12 m`. Independent analysis must use produced operands and reject
snowfall-depth/SWE, total-melt, pure-energy, and residual-as-redistribution
aliases.
