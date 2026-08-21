# Contract impact and preimplementation gate

Status: `ADJUDICATED — NO_CONTRACT_CHANGE_REQUIRED`.

Cadence: `NO_CONTRACT_CHANGE_REQUIRED`. `SC-COUPLEDTIME-001` v3 admits any
positive half-open common support, owns accepted cursor/event chronology, and
requires the same duration bits for all participants (sections 4–7).
`SC-SNOWFREEFORCING-001` v1 explicitly defines 48 ordered 1,800-second
supports covering the day (`INV-SFF-001`). The implementation change is an
extraction of the existing Stage 3 transition over that already admitted
support; it does not change equations, constants, operation order, or error
criteria. `SC-SNOWFREEZE-001` v136 already separates the default-off terminal
receiver chronology (`INV-SNOWFREEZE-102`) from the evaluation-only terminal
numerics (`INV-SNOWFREEZE-101`).

Snow-covered V11: current authority requires V11 state to advance in the
active coupled segment (`SC-COUPLEDTIME-001` consumer scope and
`SC-VEGETATION-001`/`SC-VEGETATIONTRANSACTION-001` owner boundaries). The
released V10/V11 real-consumer stack is reusable for typed owner transitions,
but its snow-free lower-surface boundary cannot be selected for a covered
segment. This is an implementation adapter within the admitted owner
boundary, not a new equation or authority claim.

Receiver topology: `SC-SNOWFREEZE-001#INV-SNOWFREEZE-102`,
`SC-SURFACELIQUID-001` v7, and `SC-LANDSURFACEENERGY-001` v7 require actual
receiver selection, exact-one custody, support validation, and rollback. They
do not limit the domain to one surface-liquid record. The attachment must use
the existing digest-bound OFE/tile topology and area fractions.

Restart: `SC-COUPLEDTIME-001` v3 section 8 requires additive versioning and
protects existing DirectV10 V1, coupled-time V2, and nested V11/direct owner
wires. The closure path will add an outer envelope containing immutable nested
payloads; no released wire is edited.

Therefore no prospective contract amendment is required before Rust edits.
The contract identity census and current index/assertion profile are recorded
in `required-reading-map.md`, `assurance-drift-census.md`, and
`stale-contract-assertion-census.md`. If implementation discovers a missing
binding for a new physical operand or owner boundary, edits stop and a narrow
contract cycle is opened before production-path continuation.
