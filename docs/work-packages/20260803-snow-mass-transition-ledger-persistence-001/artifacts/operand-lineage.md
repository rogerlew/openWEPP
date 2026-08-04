# Operand Lineage

Status: `queued / required before production edits`

Evidence mode: `Static scaffold`

Record one row per compact-ledger and shared-link operand:

| Ledger | Operand | Units | Time/sign basis | Exact producer | Authority role | Downstream consumer | Rejected aliases |
|---|---|---|---|---|---|---|---|
| upstream solid-to-liquid | TBD | TBD | TBD | TBD | TBD | TBD | TBD |
| exact linked handoff | TBD | TBD | TBD | TBD | TBD | TBD | TBD |
| downstream liquid disposition | `incoming_liquid_m` | `m` SWE | daily, nonnegative | TBD | authoritative outcome / compact ledger | TBD | top-level CoE melt aliases |
| downstream liquid disposition | `routed_liquid_m` | `m` SWE | daily, nonnegative | TBD | authoritative outcome / compact ledger | TBD | top-level routed melt |
| downstream liquid disposition | `retained_liquid_delta_m` | `m` SWE | daily, signed | TBD | compact ledger | TBD | CoE retained store or omission |
| downstream liquid disposition | `refrozen_liquid_m` | `m` SWE | daily, nonnegative | TBD | authoritative outcome / compact ledger | TBD | double refreeze |
| downstream liquid disposition | `liquid_closure_residual_m` | `m` SWE | daily, signed | TBD | guard/diagnostic | independent parser | trusting producer residual |

Freeze both accepted identities and deliberately separated fixture values
before production edits. Do not infer an operand from a neighboring output.
