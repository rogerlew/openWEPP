# Operand Lineage

Status: `PASS`

Evidence mode: **Static + Ran**.

| Operand | Units / scale | Source | Reconstruction role |
|---|---|---|---|
| precipitation multiplier | dimensionless, full record | frozen retained/new grid | daily source precipitation multiplied before execution |
| modeled seasonal peak SWE | m, water year | WAT `snow_water` | maximum daily modeled SWE |
| observed seasonal peak SWE | m, water year | frozen SNOTEL record | maximum daily observed SWE |
| peak ratio | dimensionless, median across years | modeled/observed peaks | magnitude objective and `[0.9,1.1]` band |
| chronology offset | days, median across years | WAT and SNOTEL | inherited peak-date or melt-out-date operator |
| effective input | m SWE through observed peak | trace ledger | initial SWE + snowfall SWE + retained rain |
| retained storage | m SWE on observed peak date | WAT and trace | modeled SWE remaining on that date |
| pre-peak modeled loss | m SWE | trace ledger | effective input minus storage and vapor transfer |
| CoE melt terms | m SWE | trace `amelt/bmelt/cmelt/dmelt/cap` | independent component/applied-melt closure |
| diagnostic closure | m SWE | independently reconstructed trace/WAT operands | two-sided acceptance against `1e-12 m` |

Niwot magnitude uses peak SWE; its chronology is the worse absolute result of
the depth-peak and SWE-peak operators. Other lanes use their sole frozen
operator. Retained and new cells are analyzed with the same code and operands.
