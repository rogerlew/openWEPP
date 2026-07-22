# Obligation-to-Test Map

| Obligation | Existing evidence | Package treatment |
| --- | --- | --- |
| exact option/key admission and duplicate/cardinality rejection | `options_require_complete_unique_key_value_pairs`; `command_dispatch_fails_closed_for_unknown_or_incomplete_requests` | preserve match arms and fail-closed usage error |
| standalone HEAVY cannot accept caller-synthesized READY input | `standalone_heavy_rejects_an_unauthenticated_ready_document` | unchanged |
| transition inputs/outputs are validated before execution | `transition_rejects_missing_or_colliding_outputs_before_execution` | preserve first operation and error precedence |
| LIGHT persistence precedes durable LIGHT closure and audit construction | Attempt 15 LIGHT receipt and READY audit | extract whole ordered block only |
| non-READY audit returns without HEAVY | direct branch/static review plus sealed audit result schema | preserve branch and JSON fields verbatim |
| HEAVY STARTED precedes validation/execution; CLOSED or FAILED follows | Attempt 15 durable receipt/ledger lifecycle | extract transaction helpers without reordering calls |
| invalid LIGHT/ledger combinations produce a sealed failure audit | existing `pre_heavy` fallback tests and main wrapper match | preserve four-way selection and represented error code |
| canonical output and exit semantics | binary `emit`/confined-output unit tests plus Attempt 15 receipt | JSON keys/values and `persist_*` calls unchanged |

Static: this is glue/control-flow code, not a science-contract module; no
process-physics Test-Vector Obligation family applies.
