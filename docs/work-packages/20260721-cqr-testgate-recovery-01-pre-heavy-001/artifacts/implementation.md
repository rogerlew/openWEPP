# Implementation

Status: in progress.

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
