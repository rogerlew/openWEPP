# Process Span Contract

Status: complete.

Static:

| Span | Phase kinds | Inputs | Compute | Mutation | Downstream operands | Shadow |
|---|---|---|---|---|---|---|
| R4P/Q/Z projection closure | `StorageReconciliation -> ClosureDiagnostics` | final R4N layer state, R4B storage shadow, R4A/R4M/R4O/R4N/R4G direct shadows, publication comparison frame | aggregate storage recompute plus direct hydrology projection assembly | stores projection state on the direct day frame | direct hydrology projection operands | direct hydrology projection shadow |

The span is comparison-only. It must fail closed if required upstream direct
shadows are absent and must not read compatibility storage/request/writeback or
symbol surfaces.
