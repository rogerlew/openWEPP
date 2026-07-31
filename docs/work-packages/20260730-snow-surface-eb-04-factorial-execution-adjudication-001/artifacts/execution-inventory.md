# Execution Inventory

Status: `complete`

Evidence mode: `Ran`

The original fixed round executed 48 real direct-production cells with four
isolated workers. The operator attests that no failed cell was retried; the
durable attempt ledger and rerun sentinel were added retrospectively.

| Cell | Completed lanes | Typed failures | Scientific score |
| --- | ---: | ---: | --- |
| B | 12/12 | 0 | Available for 10 independent-validation lanes |
| L | 10/12 | 2 | Incomplete; not comparable across the fixed population |
| S | 2/12 | 10 | Incomplete; not comparable across the fixed population |
| LS | 0/12 | 12 | Unavailable |

Twenty-two failures carry
`snow.stage3_effective_snow_conductivity_w_m_k` through the typed
`HKERNEL-WB14-RUNOFF-E-003` path. `harvard_open/S` and `marcell_open/LS` carry
`prior_layers.thickness_m` reconciliation failures. Failure day indices range
from 13 to 12,517.
The complete lane/cell records, hashes, commands, partial trace counts,
physical audits, observation rubric outputs where available, and decision are
in [`factorial-results.json`](factorial-results.json).
