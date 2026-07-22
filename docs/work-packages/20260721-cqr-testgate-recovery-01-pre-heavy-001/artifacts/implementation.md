# Implementation

Status: implementation and corrected metric complete; terminal verification
pending.

Static: the first extraction is private and preserves the original evaluation
order and error codes. It replaces the ordered failure-token ladder with an
ordered rule table, isolates LIGHT result checks, execution identity field
checks, per-node stage ordering, and combined-DAG predicates. No policy,
schema, public API, error precedence, or output bytes were intentionally
changed.

Ran: the focused `pre_heavy` inventory passed 19/19 after this extraction (run
ID `bd87c218-85b7-4379-af70-177eec5d0f16`, 15.710 seconds). Formatting passed.
The scoped Clippy invocation was started after that test; this client did not
return its terminal status, so it is not claimed as passing evidence.

Static: a second private extraction isolates executor-binary and claim binding,
then exact node-shape and disposable reconstruction checks. Call order and the
original typed errors remain unchanged.

Ran: the focused inventory passed 19/19 in 16.876 seconds (run ID
`8f3c1bd4-18f0-4a40-b9a4-3a97d99559fb`).

Static: the durable-defect scan now separates exact ledger parsing/status
folding from ordered OPEN rejection. It retains later-record-wins semantics and
the first lexicographic OPEN-defect error.

Ran: the focused inventory passed 20/20 in 16.658 seconds (run ID
`31649409-1f30-4002-8c95-6442c4ad97c5`).

Static: the remaining original hot spots were split into ordered private
helpers for ten-check assembly, audit binding/check-set validation, diff/line/
prompt prerequisites, LIGHT checkpoint artifacts, and stage-receipt binding.
No gate ordering, canonical field, error code, public API, or policy input was
changed. No new lint suppression was added.

Ran: focused Clippy with warnings denied passed. The focused `pre_heavy` unit
inventory passed 20/20 in 15.282 seconds (run ID
`baf52c7c-5b93-497f-8b2a-34499f11150c`).

Static: direct characterization now binds the public audit-construction and
execution-admission wrappers plus exact committed-plan reconstruction. The
tests live in the authorized test-only child module and do not expand the sole
production-module boundary.

Ran: exact head `68e9b747` passed 117/117 instrumented library tests. The
production target passes the 85% line/region aggregate, every one of 111
production functions passes the 75% region floor, and no CRAP row exceeds 30.
