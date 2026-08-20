# Restart And Rollback Evidence

Status: implementation candidate complete

Evidence mode: Static + Ran

The reference chronology serializes additive `CoupledTimeRestartV1` mid-parent,
restores its accepted clock, active regime/participants, accepted event and
scheduled receipts, reduction state, and staged outbox, then completes with the
same owner/receipt/maximum result. A ledger-rejected attempt compares the full
clock/reduction/outbox candidate byte-for-byte before reduced retry. Event replay
and premature publication fail without state change.

DirectV10 V1 production restart source and released schema/vector/manifest bytes
were not edited; authority gates recomputed their live protected hashes.
