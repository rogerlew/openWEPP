# Temporal Operator Ledger.md

Status: authority candidate

Evidence mode: Static

| Class | Slab/event behavior | Retry behavior |
| --- | --- | --- |
| `AlgebraicRate` | recompute from staged state/support | discard |
| `SupportIntegral` | rate times exact derived slab seconds | discard |
| `SequentialStateTransition` | ending accepted state begins next slab | discard candidate |
| `ThresholdEvent` | localize; accept zero-duration transition once | failed event changes nothing |
| `ScheduledOnce` | receipt keyed to named parent/calendar boundary | receipt prevents replay |
| `DiagnosticReduction` | fold accepted slabs only | ignore rejected attempt |
