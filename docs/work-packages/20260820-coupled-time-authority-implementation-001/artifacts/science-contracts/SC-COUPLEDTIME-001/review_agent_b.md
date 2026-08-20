# Authority Review B — Ownership, Restart, Wire, And Publication

Status: complete

Evidence mode: Static

Review identity: independent Phase-2A authority review B. I did not read the
other authority review. I inspected the canonical contract, package authority,
time/restart/model/error schemas, frozen vectors, independent reference model,
contract-derived Rust test, event/participant artifact, constraint artifact,
and publication/reduction lineage artifact. I did not execute validators.

## Findings

### B-001 — CRITICAL — restart wire omits authority-required continuation state

Contract lines 261-280 require run/forcing/calendar and model identities, next
segment/slab/event ordinals, last accepted step, the accepted complete owner
set, active regime/participants, boundary modes, constraint/controller policy
identities, and adopter-owned proposal history. `restart-schema.json` omits the
run, forcing, calendar, parent-interval, schema/model-definition, next-segment,
last-step, boundary-mode, constraint-policy, and controller-history fields. It
stores only an `owner_set_sha256`, not the accepted complete owner set whose
bytes are needed to resume. The schema also leaves event receipts, reduction
state, and publication records unconstrained (`array`/`object` with no canonical
item shape). Thus two materially different continuations can satisfy the same
released wire shape, and uninterrupted/restored byte equivalence cannot be
enforced.

Required disposition: expand and version the schema to bind every line 261-270
field with closed, bounded canonical sub-schemas; distinguish complete owner
state from its digest; add poison vectors for each omitted or altered field and
restart-before/after-event/scheduled/reduction/publication cases.

### B-002 — CRITICAL — identity hashing and parent transaction advancement are not canonical

Contract lines 102-107 and 192-196 describe `ParentIntervalId` and `AttemptId`
as SHA-256 over prose-level tuples, while lines 230-232 do the same for
`EventId`. No domain separators, field tags, length framing, canonical byte
encoding, digest encoding, or schema-version binding are specified. Different
field partitions can therefore hash the same concatenated bytes, and independent
implementations need not produce the same identity. `ParentTransactionId` is
said to “increment” once, but its type, initial value, overflow behavior, and
relationship to the 32-hex-character restart field are undefined. This blocks
transaction, replay, and restart identity authority.

Required disposition: define one canonical hash preimage/framing algorithm for
every identity and receipt, including authority/schema version and domain tag;
define the parent transaction counter/identifier wire and checked successor
operation; publish separating vectors for reordered, omitted, ambiguous-length,
wrong-version, and overflow preimages.

### B-003 — MAJOR — event no-progress protection is defeated by its own ordinal

Contract lines 239-244 say an event succeeds by incrementing `event_ordinal`,
then count a changed `event_ordinal` itself as deterministic progress. A
same-tick event generator can therefore recreate an equivalent pending event
forever while satisfying the stated progress test on every iteration; it will
reach ordinal overflow instead of the promised `ERR-CT-013` cycle failure.

Required disposition: exclude bookkeeping ordinals and receipt accumulation
from physical/event-queue progress, define a canonical same-tick transition
state/cycle key, and bound same-tick processing. Add a vector in which each
event creates the same semantic successor under a new ordinal.

### B-004 — MAJOR — publication lacks a crash-safe commit/delivery protocol

Contract lines 282-295 prohibit visibility before owner commit, but do not
define the identity and state machine for publication after commit. A crash
between complete-owner installation and external delivery can lose output; a
crash after delivery but before durable acknowledgement can duplicate it.
The restart schema contains only an untyped precommit `publication_buffer` and
does not represent committed-but-undelivered or delivered/acknowledged state.
Consequently the promised identical publication order across restart (lines
272-276) is not reconstructable.

Required disposition: define a parent/publication receipt identity and durable
outbox states, the atomic relationship between owner commit and outbox commit,
idempotent delivery/replay rules, and restart at every commit/delivery crash
boundary. Clarify whether “expose” means enqueue durably or complete external
delivery.

### B-005 — MAJOR — wire schemas do not enforce the declared integer/support authority

Contract lines 90-96 admit only `0..=u128::MAX` and lines 138-146 require
positive support. `time-wire-schema.json` accepts arbitrary 39-digit values,
including values above `u128::MAX`, and cannot establish `start_ns < end_ns`.
The restart schema repeats the same overbroad tick pattern. Active participants
are merely unique, not canonically ordered, despite the ordered-set authority;
receipt and publication arrays have no bounds or canonical ordering. The
contract's generic `ERR-CT-020 Serialization` therefore has no complete wire
predicate to enforce.

Required disposition: add semantic validation requirements beyond JSON Schema,
closed canonical collection schemas, ordering and size limits, exact u128-max
vectors, and malformed/noncanonical serialization vectors for every identity
collection.

### B-006 — CRITICAL — the vector/reference/test gate is presence-based, not authority evidence

The contract requires all poison populations at lines 444-477. The current JSON
contains only 8 accepted, 11 rejected, and 2 duration cases and omits, among
others, partial owner acceptance, mismatched duration/owner/participant joins,
rejection byte identity, scheduled-once replay, reduction/publication aliases,
publication rollback, legacy DirectV10 exact bytes, malformed serialization,
transaction advancement, and authority tuples. More importantly,
`reference_model.py` lines 63-74 evaluate only `empty_support`, `gap`, and
`overlap`; all other rejected cases are ignored. The Rust test lines 35-50 only
checks that selected IDs exist, lines 63-69 only search schema/package strings,
and no separately implemented expected-output comparison exists. A PASS can be
obtained with incorrect ownership, restart, event, and publication semantics.

Required disposition: make every mandatory vector executable with exact
expected identity/state/error/no-op output; independently compute expected
results in the Python model; compare them structurally from a separately
authored Rust test; and protect DirectV10 with actual baseline bytes/digests,
not a prose assertion.

### B-007 — MAJOR — atomic owner acceptance has no canonical candidate/ledger receipt shape

Contract lines 211-219 require candidate identities, exchanged-flux identities,
ending owner digests, and local/global ledgers, but neither a wire/schema nor a
canonical receipt field set defines those joins. Event transfer ledgers are
similarly prose-only at lines 221-237. Without a required per-owner candidate
map keyed over the fixed complete owner set, active/inactive disposition, and
ledger identity/units/tolerance binding, an implementation can claim atomic
acceptance while omitting an active owner or mutating an inactive owner.

Required disposition: freeze canonical slab/event/parent receipt schemas and
the exact owner/participant/ledger join algorithm; require cardinality and
complete-set proofs plus alias-separating candidates for omitted, duplicated,
inactive-mutated, partial-installed, and wrong-beginning-owner cases.

## Positive observations

- The contract correctly fixes the complete parent owner set while allowing an
  admitted active participant subset and zero-duration custody transitions.
- Rejected attempts are explicitly excluded from accepted chronology, restart,
  reductions, and publication.
- Existing DirectV10 restart bytes are expressly protected and coupled-time
  restart is declared additive/versioned.
- Controller physics is separated from clock arbitration, and publication is
  correctly intended to remain buffered until parent acceptance.

## Recommendation

**GO-WITH-AMENDMENTS.** The architecture is coherent, but B-001, B-002, and
B-006 prevent authority release and therefore prevent the Phase-2A checkpoint.
Production Rust must remain forbidden until every finding is dispositioned,
accepted corrections are applied, the complete executable vector/schema/profile
gate passes, and independent verification confirms the corrected exact
authority identity.
