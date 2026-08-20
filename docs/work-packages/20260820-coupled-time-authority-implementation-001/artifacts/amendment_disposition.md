# Coupled-time restart amendment disposition

Authority under review: `SC-COUPLEDTIME-001` version 2 and
`OPENWEPP_COUPLED_TIME_RESTART_V2`.

The first amendment candidate was rejected by both reviewers. No production
implementation resumed while those findings were open.

| Finding | Disposition |
| --- | --- |
| A/B: mutation of approved restart V1 | Accepted. Restored `restart-schema.json` byte-for-byte from the released checkpoint and introduced separately versioned `restart-schema-v2.json`. V1 cannot enter authenticated mid-parent continuation. |
| A: wrong duration bits accepted | Accepted. V2 semantic admission independently reconstructs binary64 seconds from integer support and compares exact bits. Added a well-formed wrong-bits poison. |
| A/B: receipt and slab IDs syntax-only | Accepted. V2 reconstructs both framed identities using the closed model-definition field sequence. |
| A/B: parent/cursor/owner/clock joins absent | Accepted. V2 joins parent transaction, ordinal, contiguous support, exact cursor, terminal accepted owner set, consecutive slab owner/clock digests, and slab-to-event owner/clock chronology. Event and event-receipt framed identities are independently reconstructed. |
| A/B: well-formed wrong values missing | Accepted. Added valid-shape wrong duration, parent, and terminal-owner poisons in addition to malformed and omission cases. |
| A/B: segment/event chronology and non-tautological finalization | Accepted. Segment identity remains an authenticated immutable lineage identifier; past participants are not needed to continue or finalize and cannot be substituted without changing the slab identity/receipt. V2 adds parent-bound authenticated event receipts and chains the slab boundary through event owner/clock digests. `restart_finalization_reference.py` independently reconstructs frozen parent/publication receipt IDs from restored receipt chronology. Production work remains paused until re-review. |

No finding is waived.

## Final re-review findings

| Finding | Disposition |
| --- | --- |
| `RB1`, `V2-A-001` — slabs processed before all events | Accepted. Semantic admission now merges accepted slabs and event transitions by tick with slab-end before same-tick event precedence, then checks one owner/clock chain. The independent A+B → B-to-C → A+C fixture proves the positive interior-event chronology. |
| `RB2` — last-step and segment cursor metadata unjoined | Accepted. Admission reconstructs the active segment ID, checks parent bounds, validates `last_accepted_step_ns` from the final slab support, and joins next slab/event/segment ordinals. Dedicated wrong-value poisons fail closed. |
| `RB3` — legacy oracle invalidated | Accepted. The released `slab-receipt` identity domain was restored unchanged; V2 uses the additive `slab-receipt-v2` domain. The complete 108-case oracle passes. |
| `V2-A-002` — first owner/clock/parent/segment roots unauthenticated | Accepted. V2 persists beginning owner-set and clock digests, reconstructs parent interval and transaction IDs from run/calendar/forcing/sequence roots, reconstructs the active segment ID, and begins the merged chain at those roots. Well-formed substitution poisons reject each join. |

Both final independent reviews report PASS. No production Rust is approved by
this disposition; implementation remains paused through dual amendment
verification and the exact authority checkpoint.
