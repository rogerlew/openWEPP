# Operand Lineage

Status: `frozen before result execution`

Evidence mode: `Static`

| Quantity | Units/time basis | Source/authority | Rejected alias or misuse |
|---|---|---|---|
| observed peak SWE | `m`, water-year maximum | normalized SNOTEL `WTEQ`; A4 diagnostic | modeled peak or PRCPSA |
| observed daily SWE change | `m d^-1` | consecutive normalized `WTEQ` | snow depth change or cumulative PREC |
| observed gauge increment | `m d^-1` | guarded difference of consecutive same-WY `PREC` | bridged gap, reset, or PRCPSA |
| fixture all-phase precipitation | `m d^-1` | checked-in `.cli` `prcp` | modeled snowfall |
| modeled snowfall | `m SWE d^-1` | sum of hourly `snowfall_swe_m` | `.cli` precipitation or reported depth |
| modeled retained rain | `m d^-1` | trace `rain_retained_m` | released rain or all rain |
| modeled pack loss | `m SWE d^-1` | trace `snowpack_swe_loss_m` | raw/signed/gross CoE melt |
| modeled daily storage | `m SWE` | trace runtime SWE before/after | layer liquid store or snow depth |
| current-input mass ceiling | `m` | initial modeled SWE plus all-phase fixture precipitation | achievable physical peak or calibrated forcing |
| storage-effective input | `m` | initial SWE plus modeled snowfall plus retained rain | all-phase input ceiling |
| PRCPSA | `in d^-1`, derived | AWDB snow-adjusted diagnostic | independent precipitation measurement |

The observed target and modeled inputs are not one conservation system. Their
ratios are magnitude and representativeness diagnostics. Exact modeled storage
closure is inherited from the hash-bound predecessor trace and independently
spot-checked here; it cannot validate the SNOTEL-to-hillslope observation
operator.
