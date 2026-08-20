# Review finding disposition

Status: contract-first review findings accepted / remediation in progress

Evidence mode: Static

## Contract-first science review

All findings are `accepted` and closure-blocking until rereview:

- `SCI-CRITICAL-001`: positive terminal-unallocated energy has no receiver
  disposition. The handoff must reject it with a typed error, not discard or
  credit it.
- `SCI-HIGH-002`: tail-only amendments must be integrated into canonical
  algorithm, branch/guard, invariant, unit/alias, tolerance, test-vector, and
  gap surfaces.
- `SCI-HIGH-003`: use absolute half-open wall support and define exact event-
  boundary rain/vapor ownership and cross-midnight endpoints.
- `SCI-HIGH-004`: serialize/resume the inside-substep adaptive trial or event-
  bracket state, not only accepted outer steps.
- `SCI-MEDIUM-005`: use one explicit terminal-liquid composition equation and
  source partition.
- `SCI-HIGH-006`: replace lexical-only tests with contradiction, equation,
  guard, support, restart, and poison assertions.

## Contract-first hydrology/ownership review

All findings are `accepted` and closure-blocking until rereview:

- `OWN-CRITICAL-001`: define partial-duration WB14 equations, forcing
  partition, wall-bin continuation, end-of-bin composition, and narrow
  supersession of exact-1800 rejection.
- `OWN-CRITICAL-002`: separate absolute wall/calendar support from sequential
  receiver transaction identity and specify cross-midnight advancement.
- `OWN-HIGH-003`: define a total cross-owner error precedence and preserve the
  causative error while reporting rollback-validation failure.
- `OWN-HIGH-004`: define restart stage/acceptance, schema/version/migration,
  bracket/trial state, resume order, and receipt-consumed marker.
- `OWN-HIGH-005`: explicitly include snow-side rain in the terminal parcel
  equation and exact debit/credit transition.
- `OWN-HIGH-006`: narrowly reconcile the existing frozen/thawing
  `AUTHORITY_MISSING` row with the actual receiver boundary; do not invent
  frozen-liquid physics.
- `OWN-MEDIUM-007`: strengthen tests beyond substring inventory.

Those initial findings were remediated and both reviewers returned GO. A later
consumer-depth rereview found a distinct omitted authority surface:

- `SCI-VEG-DURATION-001` — accepted, closure-blocking. `SC-VEGETATION-001`
  does not authorize `transaction_support_s < configuration.dt_s`, nor decide
  pre-event vegetation evolution. Final science verdict: HOLD.
- `OWN-VEG-DURATION-001` — accepted, closure-blocking. Receipts, state hashes,
  configuration identity, LSE joins, and restart cannot be rekeyed or advanced
  on a caller override. Final ownership verdict: HOLD.

Disposition: route both to campaign Child 2 because the pre-event decision
depends on the unresolved snow/canopy carrier. No production edit was made.
