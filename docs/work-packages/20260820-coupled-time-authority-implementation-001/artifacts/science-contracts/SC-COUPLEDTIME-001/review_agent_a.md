# SC-COUPLEDTIME-001 authority review A

Reviewer scope: time/numerics and event semantics, including identity,
conversion, constraint, progress, replay, and contract-vector coverage.

Evidence class: **Static.** Inspected the canonical contract, model definition,
time/restart schemas, frozen vectors, independent reference model, supporting
constraint/event notes, and `tests/integration/coupled_time_authority_contract.rs`.
No validators, reference execution, or Rust tests were run. I did not read the
other authority review or verification artifacts.

## Findings

### A-001 — CRITICAL — rejected-attempt chronology cannot drive adaptive retry

Contract lines 190-209 define an attempt ordinal and an adopter-owned adaptive
proposal policy, but lines 205-209 require rejection to leave controller history
and ordinals byte-identical. Consequently a nonlinear rejection cannot record
the information needed to reduce the next proposal, and `attempt_ordinal` cannot
advance. For an otherwise identical retry its `AttemptId` at lines 192-196 is
therefore identical; for a reduced-support retry the attempt ordinal remains
semantically unused. This conflicts with the required reject/reduce/retry
chronology and leaves minimum-step exhaustion without a defined state machine.

Required correction: distinguish accepted physical chronology from provisional
retry-control state. Define which diagnostic retry ordinal and adopter proposal
history may advance after rejection, how they enter the next proposal and
attempt identity, why they cannot affect already accepted owner identity, and
whether they are intentionally discarded on restart at the last accepted
boundary. Add reject-then-reduce, repeated rejection, identical-proposal retry,
minimum-step exhaustion, and restart-after-rejection vectors.

### A-002 — CRITICAL — the event no-progress guard is vacuous

Contract lines 239-244 require each successful same-tick event to increment the
event ordinal and then count an ordinal change as sufficient deterministic
progress. Every accepted event therefore satisfies the stated progress test
even if it merely generates another materially identical event at the same tick.
Such a cycle reaches ordinal overflow rather than `ERR-CT-013`, so the authority
does not establish termination of zero-duration chronology.

Required correction: exclude bookkeeping identity/ordinal advancement from the
physical/event-queue progress measure. Define a well-founded progress rule over
owner/regime/custody state and the canonical pending-event multiset (or impose a
finite admitted transition graph/budget whose exhaustion is `ERR-CT-013`). Add a
multi-event cycle whose event IDs and ordinals differ while physical state and
pending semantics repeat.

### A-003 — HIGH — canonical parent/segment/slab/attempt/event identity is not
fully specified

Contract lines 102-107 and 192-196/230-232 express SHA-256 identities as
unframed prose tuples, with no domain separators, field encoding, length
framing, digest byte encoding, or schema/version inclusion. No canonical
`SegmentId` or accepted `SlabId` construction is defined, despite segment/slab
identity being restart and receipt authority. `ParentTransactionId` is called a
persistent identity that is “incremented” (lines 106-107, 286-288), but its wire
type and increment rule are absent; the restart schema instead admits only 32
hex characters, while other named SHA-256 identities use 64. This permits
independent conforming implementations to produce different identities and can
make restart/replay joins unsound.

Required correction: freeze domain-separated, length-framed canonical byte
preimages and wire widths for every identity and digest; define parent interval,
parent transaction, segment, accepted slab, attempt, and event lineage; reconcile
transaction sequence versus transaction identity; and add exact known-answer
identity vectors including boundary and ordinal cases.

### A-004 — HIGH — constraint compatibility and event precedence are not
machine-decidable authority

Contract lines 156-162 say equal-time constraints are compatible when their
semantics “can all be satisfied,” but do not define a compatibility table,
coalescing result, or which non-selected constraints remain binding. Lines
239-243 order events by an “explicit event-class precedence,” while lines
392-393 provide no closed event-class enumeration or canonical precedence
values. The frozen `two_events_same_tick` vector supplies names but no expected
ordered receipts/state, and there are no compatible/incompatible equal-time
constraint vectors. Thus two implementations can deterministically choose
different accepted boundaries or event order while each follows the prose.

Required correction: define the complete constraint compatibility/coalescing
matrix, persistence of coincident hard boundaries, a closed/versioned event
class precedence representation, and exact tie outputs. Bind both policies into
the model digest and populate positive and poison vectors.

### A-005 — HIGH — numeric conversion authority is underspecified at the hard
cases it claims to govern

Lines 171-188 name IEEE-754 operations, but do not define the parent-duration
comparison used to reject an event proposal at very large `u128` supports,
overflow behavior of `x_s * 1e9`, or a conversion algorithm for finite binary64
values whose rounded nanosecond magnitude exceeds the exactly representable
integer range. The model definition line 8 restates conversion as an ordinary
cast/divide expression rather than binding the contract's exact operation
sequence. The only duration vectors are exactly representable 1 s and 30 s;
there are no halfway ties, one-bit neighbors, sub-nanosecond proposals, boundary
quantization, large integer rounding, maximum tick, or overflow cases required
by contract lines 449-452.

Required correction: specify an exact finite-binary64-to-integer quantization
algorithm and comparison domain, including multiplication/integer overflow and
signed-zero behavior, and reconcile the model definition. Add independent
known-answer vectors at every named hard case, including values above `2^53`
nanoseconds and near `u128::MAX`.

### A-006 — HIGH — the frozen vector/reference/test set does not implement the
contract's release gate

Contract lines 444-477 require exact expected receipts/digests, duration bits,
constraint selection, owner bytes, atomic no-op proofs, restart equivalence,
serialization poison, and publication/reduction poisons. The current vector
file mostly contains labels and inputs. `reference_model.py` lines 63-74 execute
only three rejected partition cases; all event, replay, constraint, controller,
clock-advance, restart, atomicity, and output rejection entries are never
evaluated. The Rust contract test lines 36-49 checks IDs and a vector count, not
answers or semantics. This can pass while essentially all critical event and
constraint behavior is wrong.

Required correction: make every accepted and rejected vector carry exact
expected outputs/error precedence and before/after state digests; independently
evaluate every case; make the separately authored Rust comparison consume and
compare those results; and add all cases named by the canonical vector
obligations. Presence-only assertions are not contract-gate evidence.

### A-007 — MEDIUM — model-time origin is contradictory across authority files

Contract lines 113-118 define `tau=0` by a run-start calendar receipt and allow
provider-authoritative non-UTC calendars/offset policies. The model definition
instead fixes `model_time_origin` to `simulation_start_utc`. That narrows and can
misidentify runs whose calendar authority has no UTC mapping.

Required correction: use the contract's receipt-bound run-relative origin in
the model definition, or explicitly require and define a UTC instant for every
admitted calendar provider. Add identity vectors that distinguish equal ticks
under different calendar/forcing receipts.

## Recommendation

**HOLD.** Do not promote/index `SC-COUPLEDTIME-001` and do not begin production
Rust. A-001 and A-002 are direct chronology contradictions; A-003 through A-006
leave identity, arbitration, numeric conversion, and the purported independent
gate non-binding. Apply and disposition the corrections, regenerate the frozen
schemas/vectors/reference results, rerun every invalidated contract gate, then
submit the exact corrected authority checkpoint for independent verification.
