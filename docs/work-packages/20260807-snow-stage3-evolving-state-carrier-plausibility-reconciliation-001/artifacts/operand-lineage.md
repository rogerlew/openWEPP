# Operand Lineage Freeze

Static: frozen before new attribution execution.

| Operand family | Units / sign | Support and basis | Authority / use |
| --- | --- | --- | --- |
| shortwave, longwave, sensible, latent, advected | `W m^-2`, positive toward snow | tuple duration; integrate before WY reduction | schema-v6 primitives; reconstruct independently |
| complete external energy | `J m^-2`, positive toward snow | exact tuple duration, per unit snow-surface area | independent five-term sum |
| raw vapor opportunity | `kg m^-2`, positive deposition | tuple duration; S/F/Q diagnostic | never actual S/F debit |
| bounded deposition/sublimation | `kg m^-2`, non-negative | sequential Q tuple only | independently reconstruct as `max(raw,0)` / `min(max(-raw,0),active_ice_before)`; producer fields are check-only |
| melt | `kg m^-2`, non-negative | sequential Q tuple only | distinct phase change, never vapor |
| total ice endpoints | `kg m^-2` | before/after same Q tuple and area | independent solid-mass closure |
| active/lower cold change and export | `J m^-2` | Q tuple control volume | independent cold-content closure |
| wind | `m s^-1` | daily CLI value repeated hourly | exposure unresolved; exact consumed value |
| `z_T,z_q,z_u,z_0,aero,d,L` | `m` | tuple geometry | virtual geometry; bare thermal `z_0` rejected |
| friction/exchange velocities | `m s^-1` | tuple state | reconstruct from logs/corrections |
| temperature / humidity gradient | `degC`, `K`, `kg kg^-1` | tuple before/after | evolution attribution |
| support/censoring | `s`, counts, ratio | common-prefix and all-evaluated | `0.05` materiality threshold |

Every per-water-year ledger is formed before median reduction. Expected values
must differ from raw vapor, melt, liquid, adjacent term, and all-evaluated
aliases.
