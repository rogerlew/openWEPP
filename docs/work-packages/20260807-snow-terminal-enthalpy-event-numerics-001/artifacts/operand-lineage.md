# Operand Lineage

Status: frozen before production edits

Evidence mode: Static

| Operand | Units / basis | Authority | Status |
|---|---|---|---|
| shallow-pack ice mass | `kg m^-2`, event-local | snow state / SC-SNOWENERGY | diagnostic clone |
| shallow-pack cold content | `J m^-2`, relative to 0 C ice | SC-SNOWENERGY active volume | diagnostic clone |
| integrated snow energy | `J m^-2` over accepted step | complete Stage 3 carrier | diagnostic input |
| bounded vapor exchange | `kg m^-2` over accepted step | existing bounded Stage 3 transfer | diagnostic transfer |
| fusion melt | `kg m^-2` over accepted step | `Q/L_f`, ice-available bounded | diagnostic transfer |
| exhaustion event time | `s` from interval start | contract event solve | diagnostic endpoint |
| terminal liquid handoff | `kg m^-2` at event | ice debit plus retained liquid | censored handoff |
| terminal unallocated energy | `J m^-2` at event | post-exhaustion excess | censored handoff |

No receiving-surface, soil, frost, runoff, or evaporation operand exists in this
package. Producer residuals are never acceptance operands.
