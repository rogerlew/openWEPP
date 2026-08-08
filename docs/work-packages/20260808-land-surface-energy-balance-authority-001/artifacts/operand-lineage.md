# Operand Lineage

Status: complete for authority v1

Evidence mode: Static

| Operand | Unit/sign | Producer | Consumer | v1 disposition |
|---|---|---|---|---|
| `R_sw`, `R_lw` | `W m^-2`, into surface positive | future admitted radiation operator | LSE ledger | constitutive authority missing |
| `H`, `LE` | `W m^-2`, into surface positive | future turbulent/latent operator | LSE ledger | constitutive authority missing |
| `Q_p`, `Q_runon` | `W m^-2`, into surface positive | future liquid-advection operator sharing mass lineage | LSE ledger | authority missing |
| `Q_inf`, `Q_runoff` | `W m^-2`, outgoing magnitude | future liquid-advection operator sharing mass lineage | LSE ledger | authority missing |
| `G` | `W m^-2`, into surface positive | one shared surface/soil interface | surface `G`, soil/frost `-G` | authority/interface missing |
| `E_s,0/1` | `J m^-2` | future LSE state | future LSE state/closure | state authority missing |
| `m_p`, `m_runon` | `kg m^-2`, input positive | climate/hydrology | surface-water ledger | sealed handoff required |
| `m_evap` | `kg m^-2`, debit non-negative | `SC-EVAP-001` | surface-water and latent ledgers | exact-one shared identity required |
| `m_inf`, `m_runoff` | `kg m^-2`, debit non-negative | hydrology owners | surface-water ledger | no repartition by LSE |
| terminal liquid/energy/time | snow schema-v8 units | snow terminal solver | none | censored and rejected |

Energy closure is `delta E = dt * sum(fluxes)`. Water closure is
`delta M = inputs - debits`. Tests require distinct poison operands so omission
or duplication cannot hide behind equal numeric values.
