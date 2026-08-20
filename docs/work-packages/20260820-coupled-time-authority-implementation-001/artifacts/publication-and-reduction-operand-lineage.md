# Publication And Reduction Operand Lineage.md

Status: authority candidate

Evidence mode: Static

| Operand | Units | Support/source | Admission |
| --- | --- | --- | --- |
| slab diagnostic | contract-declared scalar | exact accepted slab + owner | accepted slabs only |
| peak | same as diagnostic | maximum over all accepted slabs across restart | parent candidate only |
| scheduled output | contract-declared | exact named boundary receipt | exact-once |
| publication buffer | typed records | accepted chronology | invisible before commit |

## Independent reconstruction procedure

The reconstructor consumes only canonical accepted slab/event/scheduled
receipts plus the parent candidate; it may not consume runtime accumulator
state. It sorts accepted operands by `(support.end_ns, slab_ordinal,
source_owner_id, accepted_receipt_id)`, verifies each receipt belongs to this
parent and is not rejected, then folds the declared operator using exact stored
binary64 bits and declared units. Across restart, concatenate the ordered
pre-checkpoint receipt prefix with the post-checkpoint suffix and compare the
result byte-for-byte with uninterrupted execution. Reconstruct the ordered
publication records independently, hash them, and require that digest in both
the parent and publication receipts.

Parent owner installation and creation of exactly one outbox row in
`CommittedUndelivered` are one durable atomic action. This enqueue is exposure;
direct delivery before it is forbidden. Delivery uses
`publication_receipt_id` as its stable idempotency key and permits only:

```text
CommittedUndelivered -> DeliveredUnacknowledged -> Acknowledged
```

A crash before the atomic commit leaves neither owners nor outbox. A crash
after commit retries the same undelivered row. A crash after delivery but before
acknowledgement may redeliver only with the same key. Acknowledged rows never
redeliver. Restart must preserve row sequence, state, complete record bytes,
delivery-attempt count, and receipt identities; delivery-attempt bookkeeping
does not affect physics or publication identity.

## Alias-separating acceptance fixtures

Rejected aliases are: maximum over accepted plus rejected attempts; parent
volume divided by nominal duration; pre-restart-only or post-restart-only peak;
duplicate scheduled output; precommit publication; publication retained after
parent rollback; recomputed record order; changed receipt on redelivery;
acknowledgement-before-delivery; and redelivery after acknowledgement.

Acceptance requires the separately written reconstruction above, exact
uninterrupted/restored reduction and publication-order equality, real magnitude
and conservation closure, and crash-boundary proofs for before owner/outbox
commit, after commit, after delivery, and after acknowledgement. Runtime
self-consistency alone is not acceptance.
