# Operand Lineage

Status: `PASS`

Evidence mode: **Static + Ran**.

| Reported operand | Units / scale | Direct source | Reconstruction |
|---|---|---|---|
| precipitation multiplier | dimensionless, whole climate record | prospectively frozen grid | daily source `prcp` multiplied before execution |
| modeled seasonal peak SWE | m, water year | WAT `snow_water` | maximum daily SWE within paired observation year |
| observed seasonal peak SWE | m, water year | inherited SNOTEL record | maximum daily observed SWE within frozen operator window |
| peak-SWE ratio | dimensionless, median across water years | modeled and observed peaks | median of modeled/observed seasonal peak ratios |
| chronology offset | days, median across water years | modeled WAT plus observed SNOTEL | inherited peak-date or melt-out-date operator |
| effective input | m SWE, through observed peak date | snow trace | initial SWE + realized snowfall SWE + retained rain |
| retained storage | m SWE, observed peak date | WAT `snow_water` | modeled SWE sampled on observed SWE-peak date |
| pre-peak loss | m SWE, through observed peak date | trace storage ledger | effective input minus retained modeled SWE |
| CoE melt terms | m SWE | trace `amelt`, `bmelt`, `cmelt`, `dmelt`, cap | summed independently and checked against applied melt |
| sublimation | m SWE | snow trace | summed sublimation output; zero in baseline-B cells |
| mass closure | m SWE | independently reconstructed ledger | input minus storage and diagnosed loss operands |

Niwot retains two observation operators. Its magnitude objective uses the
peak-SWE ratio, while its chronology objective is the worse absolute offset of
the peak-depth and peak-SWE operators. Other lanes use their sole inherited
operator. No result-aware operator substitution occurred.
