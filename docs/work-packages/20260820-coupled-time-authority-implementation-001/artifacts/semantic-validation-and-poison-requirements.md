# Semantic Validation And Poison Requirements

Status: authority candidate

Evidence mode: Static

JSON Schema is only the structural gate. `OPENWEPP_COUPLED_TIME_SEMANTIC_VALIDATOR_V1`
must parse every decimal into checked `u128`, reject values above `u128::MAX`,
enforce support and cursor bounds, and prove every bounded collection is in
strict canonical UTF-8-byte or receipt-ID order with no duplicate. It must
decode every retained byte payload, reproduce its digest, canonically
reserialize the admitted document, and require byte equality.

For a slab, the owner-candidate keys must equal the complete owner set exactly.
Active owners require `ActiveCandidate`; inactive owners require
`InactiveByteCarry`, identical beginning/ending bytes, and no exchange. Every
candidate joins the common support/duration/beginning digest. Ledger IDs must be
unique, every referenced local ledger must exist, exchange-pair ledgers must
name both operands through their lineage digest, and all ledgers must pass.
Only after those proofs may the complete ending digest be reconstructed and
installed atomically. Event candidates use the same complete-set rule, with
`EventMutation` admitted only for declared mutated owners and zero derived
duration. Parent candidates must reference the exact ordered accepted child
receipts, closed global ledgers, complete owner set, and publication buffer.

Required poison population includes over-maximum `u128`; reversed/equal/outside
support; unordered, duplicate, oversized, omitted, or extra identities and
receipts; corrupt base64/digest; noncanonical serialization; missing or extra
owner candidate; wrong participant disposition; inactive mutation; wrong
beginning owner; mismatched support/duration; missing/duplicate/unreferenced or
failed ledger; partial installation; rejected receipt in reduction/publication;
altered run/calendar/forcing/model/policy/controller bytes; event/scheduled-once
replay; and altered restart state immediately before or after an event.

Outbox poisons include output before parent commit, outbox without its parent
receipt, committed owners without the atomically committed outbox row, changed
receipt/records on retry, acknowledgement before delivery, redelivery after
acknowledgement, duplicate sequence/receipt, and retained output after rollback.
Every poison must return its typed error without accepted-state mutation.
