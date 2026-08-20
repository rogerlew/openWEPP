---
contract_id: SC-COUPLEDTIME-001
title: Coupled Time Support, Event, and Atomic Chronology Contract
status: approved
maturity: active
owner: openWEPP maintainers + time/numerics + transaction/restart reviewers
contract_version: 1
producer_scope:
  - OPENWEPP_COUPLED_TIME_SUPPORT_V1
  - Coupled parent-interval coordinator and staged clock
consumer_scope:
  - Segmented-support vegetation V11
  - Snow, land-surface-energy, surface-liquid, Lane D, Richards, plant, soil-thermal, biogeochemistry, restart, and publication adopters
evidence_level: static+independent_oracle+contract_vectors
last_reviewed: 2026-08-20
supersedes: []
superseded_by: []
---

# SC-COUPLEDTIME-001 Coupled Time Support, Event, and Atomic Chronology Contract

Status: `approved`

Maturity: `active`

Authority identity: `OPENWEPP_COUPLED_TIME_SUPPORT_V1`

Evidence mode: `Static + independent oracle and executable contract vectors`

## Purpose and scientific scope

This contract gives modeled time exactly one staged owner inside a sealed parent
forcing interval. It defines exact support identity, regime segmentation,
common accepted slabs, provisional attempts, zero-duration event transitions,
deterministic constraint arbitration, restart chronology, atomic owner-set
acceptance, scheduled-once custody, diagnostic reductions, and delayed parent
publication.

It governs chronology and transaction mechanics, not constitutive equations.
Vegetation V10 remains immutable. Vegetation V11, snow/carrier science,
Richards, Lane D, root hydraulics, thermal, and biogeochemical equations remain
owned by their adopter contracts. An adopter may request a maximum step and
retain its controller history, but may not advance accepted time privately.

The five distinct time concepts are:

1. **nominal forcing interval**: immutable external support of one forcing
   receipt;
2. **physical regime segment**: a maximal admitted support with one regime and
   active-participant identity;
3. **accepted coupled slab**: one common positive-duration state advance;
4. **numerical attempt**: provisional work that advances neither accepted state
   nor chronology until atomic acceptance; and
5. **event instant**: a zero-duration boundary at which admitted state/custody
   changes may occur without rate integration.

## Authority anchors

| ID | Source | Binding use | Evidence |
|---|---|---|---|
| REF-CT-PHYSICAL | Half-open interval algebra; conservation; monotone chronology; transactional atomicity | No gap/overlap, no time creation, accepted-only reductions, atomic owner installation | `[INFERENCE][Static]` |
| REF-CT-IEEE754 | IEEE 754 binary64 round-to-nearest, ties-to-even | Tick/seconds conversion and event-proposal quantization | `[DIRECT][Static]` |
| REF-CT-TRANSACTION | `SC-VEGETATIONTRANSACTION-001` | Beginning/ending owner joins, rollback, one parent commit | `[DIRECT][Static]` |
| REF-CT-SNOW | `SC-SNOWFREEZE-001`, `SC-SNOWENERGY-001`, `SC-SURFACELIQUID-001` | Terminal-event split and exact-one liquid/state custody transfer | `[DIRECT][Static]` |
| REF-CT-LSE | `SC-LANDSURFACEENERGY-001` | Pre/post-event receiver chronology and support-sensitive fluxes | `[DIRECT][Static]` |
| REF-CT-FORCING | `SC-SNOWFREEFORCING-001` | Sealed forcing-receipt support and day/calendar lineage | `[DIRECT][Static]` |
| REF-CT-RESTART | Released DirectV10 persisted-restart V1 authority and vectors | Additive versioned restart; existing bytes protected | `[DIRECT][Static]` |
| REF-CT-PACKAGE | `docs/work-packages/20260820-coupled-time-authority-implementation-001/package.md` | Current-scope architectural decisions and required poison vectors | `[DIRECT][Static]` |

## Variables and units

| Symbol | Units | Meaning | Boundary/API name |
|---|---|---|---|
| `tau` | `u128 ns` | run-relative model-time tick | `ModelTimeNs` |
| `tau_0` | `u128 ns` | run origin, exactly zero at admitted run start | `model_time_origin_ns` |
| `S=[a,b)` | `u128 ns` pair | positive-duration half-open support | `TimeSupport { start_ns, end_ns }` |
| `Delta_tau` | `u128 ns` | exact duration `b-a` | `duration_ns` |
| `Delta_t` | binary64 `s` | derived common numerical operand | `duration_s_bits` |
| `P` | identity | sealed parent interval | `ParentIntervalId` |
| `X` | identity | persistent parent transaction | `ParentTransactionId` |
| `n_X` | `u128` | persistent parent transaction sequence | `parent_transaction_sequence` |
| `g` | `u32` | segment ordinal | `segment_ordinal` |
| `k` | `u32` | accepted slab ordinal within parent | `slab_ordinal` |
| `r` | `u32` | diagnostic attempt ordinal at accepted cursor | `attempt_ordinal` |
| `e` | `u32` | accepted event ordinal within parent | `event_ordinal` |
| `O` | ordered identity set | complete parent owner set | `complete_owner_set` |
| `A_g` | ordered identity subset | active participants for segment `g` | `active_participant_set` |
| `C` | typed proposal | step constraint | `StepConstraintV1` |
| `D_policy` | SHA-256 | adopter controller definition identity | `controller_policy_sha256` |

The wire representation for every tick is an unsigned 128-bit integer encoded
as a canonical base-10 string with no sign, leading zero (except `"0"`),
fraction, exponent, or whitespace. The admitted range is `0..=u128::MAX` ns;
every addition, subtraction, ordinal increment, and conversion checks overflow.
The maximum representable run-relative duration is `u128::MAX ns` (about
`1.078e22` Julian years); no calendar claim beyond the calendar provider's own
domain follows from that wire capacity.

## Algorithm state surfaces

### Required immutable parent inputs

- run identity and run-start calendar receipt;
- sealed forcing receipt and exact parent support;
- `ParentIntervalId = SHA256(run identity, forcing receipt identity, parent
  support, calendar/day mapping)`;
- persistent `ParentTransactionId`, incremented exactly once only when the
  complete parent transaction commits;
- ordered complete owner identities and beginning owner digests;
- admitted initial regime/segment and active participant set;
- controller policy identity/digest for each proposing adopter; and
- canonical schema/model-definition identities.

Calendar mapping is receipt-based, not inferred from ticks: the run-start
receipt binds calendar system, timezone/offset policy, civil date/day identity,
and forcing chronology. `tau=0` is that receipt's exact run-start instant.
Each parent receipt binds its calendar/day/forcing mapping. Leap-day, no-leap,
or other calendars are provider authority; coupled time checks identity and
contiguity only.

### Canonical framed identity hashing

Every SHA-256 identity and receipt uses
`OPENWEPP_CANONICAL_FRAMED_SHA256_V1`. Its preimage is ASCII `OPENWEPP\0`,
big-endian `u16(1)`, a `u16`-length-framed UTF-8 domain tag, then each field in
contract order as `u16(tag length) || tag || u32(value length) || value`.
Integers are fixed-width unsigned big-endian (`u32` ordinals and `u128`
ticks/sequences), SHA-256 values are 32 raw bytes, strings are NFC UTF-8, and
ordered collections are `u32(count)` plus individually `u32`-length-framed
members. Optional fields use an explicit one-byte presence tag. Omission,
delimiter concatenation, JSON/debug bytes, and hexadecimal digest text are
forbidden. Closed domain tags are `parent-interval`, `parent-transaction`,
`segment`, `accepted-slab`, `attempt`, `event`, `constraint`, `owner-set`,
`event-receipt`, `slab-receipt`, `parent-receipt`, and
`publication-receipt`.

The V1 field lists are closed and ordered:

| Domain | Ordered fields |
|---|---|
| parent-interval | run_id, calendar_receipt, forcing_receipt, start_ns, end_ns |
| parent-transaction | run_id, sequence, parent_interval_id, begin_owner_set |
| segment | parent_transaction_id, ordinal, start_ns, end_ns, regime_id, participant_set |
| accepted-slab | parent_transaction_id, slab_ordinal, segment_id, start_ns, end_ns, duration_bits, begin_owner_set, end_owner_set, constraint_digest, ledger_digest |
| attempt | parent_transaction_id, accepted_cursor_ns, slab_ordinal, attempt_ordinal, start_ns, end_ns, constraint_digest, begin_owner_set |
| event | parent_transaction_id, tick_ns, event_class, event_ordinal, source_owner_id, event_context |
| event-receipt | event_id, tick_ns, ordinal, begin_owner_set, end_owner_set, ledger_digest |
| slab-receipt | accepted_slab_id, begin_clock, end_clock, owner_set, ledger_digest |
| parent-receipt | parent_transaction_id, parent_interval_id, begin_owner_set, end_owner_set, ordered_slab_receipts, ordered_event_receipts |
| publication-receipt | parent_receipt_id, ordered_output_records, outbox_state |

The vector model definition freezes each field's scalar type. A tag reorder,
field omission, extra field, wrong scalar width, or wrong domain/version is a
different preimage and cannot be admitted as the named V1 identity.

`n_X` starts at zero and is checked `u128`. `ParentTransactionId` binds run ID,
`n_X`, parent interval ID, and beginning complete-owner digest. Parent commit
computes checked `n_X+1`; slabs/events do not change it. `SegmentId` binds the
transaction, ordinal, support, regime, and participant digest. `AcceptedSlabId`
binds the transaction, slab ordinal, segment ID, support, duration bits,
beginning/ending owner digests, constraint digest, and ledger digest. Wrong
version, field order/partition, or sequence overflow fails closed.

### Staged mutable state

`CoupledClockStateV1` contains parent identities/support, accepted cursor,
segment/slab/event ordinals, last accepted step, accepted event and
scheduled-once receipts, active regime/participants, accepted complete-owner
digest, diagnostic-reduction state, parent publication buffer, and controller
policy/digest plus adopter-owned serialized proposal history. Attempt work is a
separate candidate rooted at this accepted state.

### Outputs

The authority emits accepted slab receipts, accepted event-transition receipts,
one complete parent candidate, and after atomic parent commit one publication
batch. It does not mutate an adopter's constitutive state except by installing
the validated owner candidates supplied for the common slab/event.

## Algorithm specification

### 1. Construct and validate the parent

Require `parent.start < parent.end`; canonical identities/digests; a unique,
strictly ordered complete owner set; and beginning owner digests for every
owner. Set `accepted_until=parent.start`, `segment_ordinal=0`,
`slab_ordinal=0`, `event_ordinal=0`, and leave the persistent transaction ID
unchanged. The first segment begins at parent start. A segment records its
regime identity, support boundary known so far, and unique ordered active set,
which must be a subset of the complete owner set.

### 2. Collect and reduce constraints

Constraints are typed as `HardBoundary`, `EventBoundary`, `OutputBoundary`,
`RestartBoundary`, or `AdaptiveUpperBound`. Each binds parent/cursor, proposed
end tick, source-owner identity, class, and canonical constraint digest.
Reject an end behind the cursor or after parent end. A proposal at the cursor
is legal only when it names a pending admitted event transition.

Choose the earliest end tick. At that tick use class precedence in the written
order above, then lexicographically smallest canonical source-owner identity,
then lexicographically smallest constraint digest. Equal-time constraints are
compatible only when they share byte-identical parent ID, accepted cursor,
calendar receipt, and forcing receipt, and every non-adaptive fact has the same
canonical compatibility-group digest. Adaptive bounds do not conflict with a
hard fact and remain in the ordered receipt. Coincident event transitions also
require the precedence-sorted ending owner digest of each event to equal the
next event's beginning owner digest. Any mismatch fails `ERR-CT-008` rather
than being silently ordered. The selected end may not cross parent, segment, event,
output, or restart boundaries.

Coupled time owns this reduction and the accepted cursor. Adopters own proposal
algorithms, constants, tolerances, convergence history, and policy definition.
The reference halving controller is demonstration-only and is not Richards or
other adopter science authority.

### 3. Derive the common numerical duration

For selected `[a,b)`, compute checked `Delta_tau=b-a`. Convert once as

```text
Delta_t = roundTiesToEven(binary64(Delta_tau) / 1_000_000_000.0)
```

using the implementation's specified IEEE-754 correctly rounded unsigned-
integer-to-binary64 conversion and binary64 division. Store `Delta_t.to_bits()`
in the slab proposal and give those exact bits to every active participant; an
owner may not independently reconvert duration.

For a binary64 event proposal `x_s`, inspect its bits. Reject NaN, infinity, and
negative sign except that either signed zero normalizes to zero. Decode the
finite value exactly as integer significand `m` times `2^e`; form the exact
rational nanoseconds `m * 1_000_000_000 * 2^e` with checked unbounded-integer
intermediates, then round the rational to the nearest integer with ties to even.
Reject if the rounded magnitude exceeds `u128::MAX`, checked addition to parent
start overflows, or the resulting tick exceeds parent end. Thus no binary64
multiplication overflow or inexact binary64 parent-duration comparison controls
admission. Exact halfway ties choose the even tick. A proposal quantized to a
support boundary is that boundary, not one bit inside either neighbor; a
zero-duration slab requires an admitted event.

### 4. Attempt a common positive-duration slab

Create `AttemptId = framed-SHA256(parent transaction identity, accepted cursor,
slab ordinal, attempt ordinal, proposed support, constraint digest, beginning
complete-owner digest)`. Attempt IDs are diagnostic: their number or bytes may
not enter physical equations, accepted owner identity, accepted reduction, or
parent transaction identity.

Every active participant begins from the same accepted complete-owner set and
consumes exactly the proposal support and stored `Delta_t` bits. Inactive owners
are carried byte-identically. Internal solver iterations are allowed; hidden
physical subcycling is allowed only when no state exchange occurs during it and
the adopter returns independently reconstructable integrated boundary fluxes
for the common slab.

On rejection, discard all owner candidates. Accepted owners, clock, accepted
controller checkpoint, accepted ordinals, receipts, ledgers, diagnostics, and
publication buffer remain byte-identical. A separate ephemeral
`RetryControlV1` may increment `attempt_ordinal`, record typed rejection and
adopter proposal history, and reduce the next proposal. It is rooted at the
unchanged accepted-state digest, enters only the next constraint/attempt ID,
and cannot alter accepted owner/slab/parent identity. Identical-proposal retry
requires a changed retry-control digest; repetition and minimum-step limits fail
typed. Restart intentionally discards post-boundary retry control and resumes
fresh from the accepted controller checkpoint.

### 5. Accept a slab atomically

Validate common parent, beginning owner-set digest, support, duration bits,
participant set, owner candidate identities, exchanged-flux identities,
ending owner-set digest, and all local/global ledgers. Install all active owner
candidates together, carry inactive owners byte-identically, advance the cursor
exactly to `b`, and increment `slab_ordinal` exactly once. Accepted slabs never
increment the persistent parent transaction ID. Partial installation is
impossible.

The canonical candidate/receipt/ledger wire is
`OPENWEPP_COUPLED_TIME_RECEIPT_CANDIDATE_LEDGER_V1`. Its complete-owner map has
exactly one entry per fixed parent owner. Active entries are candidates;
inactive entries are byte-identical carries. Event mutation is admitted only
for the declared event mutation set. Owner keys, participants, receipt IDs, and
ledger IDs are strictly canonically ordered and unique. Every local ledger
reference resolves; exchanged-flux, tolerance-policy, units, and operand-
lineage digests join before the ending complete-owner digest is reconstructed.
The same closed envelope defines slab/event/parent candidates and receipts and
the publication receipt. Missing, duplicated, wrong-beginning, inactive-
mutated, failed-ledger, or partially installed entries fail atomically.

### 6. Apply a zero-duration event transition

An admitted event may occur at parent start, an accepted slab end, or parent
end:

```text
accepted slab [a,b) -> event transition at b -> accepted slab [b,c)
```

`EventId = framed-SHA256(parent transaction identity, event tick, event class,
event precedence, event ordinal, beginning complete-owner digest, event-context
digest)`. An event integrates no rate and advances no time. It may atomically
mutate admitted owners, transfer conserved custody, close an independent event
ledger, terminate the current segment, create the successor segment/regime,
and select its active participant set. Beginning and ending owner digests are
explicit. On success increment `event_ordinal` once and persist its receipt;
on failure change nothing.

Event class is closed/versioned: `OwnershipTransfer=0`,
`BoundaryModeTransition=1`, `RegimeTransition=2`, `ScheduledBoundary=3`, and
`DiagnosticMarker=4`. Same-tick events execute by ascending numeric class, then
source-owner identity, then event-context digest. Duplicate identity or
incompatible equal-precedence transitions fail. After each event, either owner
state, regime/participant identity, or the pending-event multiset must make
physical progress; ordinals, IDs, and receipts do not count. The cycle key is
the framed hash of tick, complete-owner digest, regime/participant digest, and
sorted pending-event semantic digests. A repeated key or more than 256
transitions at one tick fails `ERR-CT-013`.
Accepted event receipts prevent replay, including after restart.

The canonical reference chronology is: segment 0 has owners A+B active and C
unchanged; an event transfers custody B-to-C; segment 1 has A+C active and B
retained in its terminal state. The complete parent owner set remains A+B+C.

### 7. Execute temporal operator classes

| Class | Required chronology |
|---|---|
| `AlgebraicRate` | Recompute from current staged state/forcing for each accepted slab. |
| `SupportIntegral` | Integrate an admitted rate over the exact common slab only. |
| `SequentialStateTransition` | Ending state of accepted slab `k` is beginning state of `k+1`. |
| `ThresholdEvent` | Localize/quantize an event and execute the zero-duration transition protocol. |
| `ScheduledOnce` | Execute once at its named calendar/parent boundary and persist a receipt; never once per retry/slab. |
| `DiagnosticReduction` | Fold accepted slabs/events only; never affect physical acceptance. |

### 8. Restart

Persist only the last accepted boundary: parent/run/forcing/calendar identities;
parent support and accepted cursor; next segment/slab/event ordinals; last
accepted step; accepted complete-owner set; active regime and participants;
accepted event-transition and scheduled-once receipts; diagnostic/peak
reduction state; buffered parent publication; boundary modes; constraint and
controller policy identities/digests; and the adopter-owned controller history
required for the next proposal. Rejected iterates and attempt physical state are
forbidden.

Restore validates schema/model/policy/digests and resumes a fresh attempt at
the same next boundary. Restart immediately after an event cannot replay it;
restart immediately before it must execute it once. Uninterrupted and restored
execution must produce identical accepted receipts, owner bytes, reductions,
publication order, and terminal parent identity.

Coupled-time restart is additive and versioned. Existing DirectV10 persisted-
restart V1 schema, vectors, manifest, and bytes remain byte-identical. Any
change to an existing wire requires separate authority amendment.

`OPENWEPP_COUPLED_TIME_RESTART_V1` retains the complete canonical owner bytes,
controller checkpoint bytes, accepted event and scheduled-once receipts,
operand receipt lineage for reductions, complete pending publication record
bytes, and durable outbox rows—not digests alone. The separately versioned
`OPENWEPP_COUPLED_TIME_SEMANTIC_VALIDATOR_V1` is mandatory in addition to JSON
Schema: it enforces checked numeric ranges, relational support/cursor bounds,
canonical ordering/uniqueness/cardinality, byte-to-digest reconstruction,
receipt chronology and replay exclusion, accepted-only reduction/publication
lineage, outbox/parent joins, and canonical reserialization equality.

### 9. Parent finalization and publication

When the cursor equals parent end, all required events/scheduled operations are
receipted, segments/slabs exactly cover required positive support, and all owner
and conservation ledgers close, construct one parent candidate. Increment the
persistent parent transaction exactly once and atomically install the complete
owner set once. In that same durable transaction create a framed
`PublicationReceiptId` binding the parent receipt, ordered output records,
units/support/source lineage, and outbox sequence, then install a durable row in
`CommittedUndelivered`. This durable enqueue is the meaning of **expose**;
direct external delivery is forbidden. Delivery is idempotent by receipt ID and
transitions `CommittedUndelivered -> DeliveredUnacknowledged -> Acknowledged`.
After a crash, undelivered rows retry, delivered rows may redeliver with the
same idempotency key, and acknowledged rows never redeliver.

Diagnostic maxima use accepted-slab values only. Publication operand lineage
records exact support, units, source owner, accepted receipt, reduction order,
and publication state. A parent rollback removes its entire staged publication
buffer. Volume divided by nominal duration, rejected attempts, pre-restart-only
or post-restart-only subsets, duplicate scheduled values, and precommit buffers
are prohibited aliases for authoritative parent reductions.

### Constraint coincidence matrix

Every coincident constraint remains in the selected-boundary receipt.
`HardBoundary`, `EventBoundary`, `OutputBoundary`, and `RestartBoundary`
mutually coalesce at one tick. Multiple `AdaptiveUpperBound` constraints
coalesce as an ordered source/digest list; an adaptive bound coalesces with any
hard class and cannot move it. Multiple event boundaries coalesce only when
their sorted event semantic digests form a deterministic queue under the V1
event precedence; incompatible custody preconditions conflict. Different
calendar, forcing, or parent identities at one tick always conflict. The model
digest binds this matrix, both precedence tables, and the 256-event budget.

## Branch and guard table

| Trigger | Branch/action | Typed failure |
|---|---|---|
| invalid/overflowing tick, support, or ordinal | no chronology mutation | `ERR-CT-001 InvalidTimeIdentity` / `ERR-CT-002 ArithmeticOverflow` |
| malformed parent/run/forcing/calendar join | reject parent/restore | `ERR-CT-003 ParentIdentityMismatch` |
| owner set duplicate/order/digest mismatch | reject | `ERR-CT-004 OwnerSetMismatch` |
| segment/participant invalid or uncovered support | reject | `ERR-CT-005 SegmentCoverage` |
| constraint behind cursor/past parent | reject | `ERR-CT-006 ConstraintOutOfBounds` |
| zero step without admitted event | reject | `ERR-CT-007 ZeroStepWithoutEvent` |
| incompatible equal-time constraints | reject | `ERR-CT-008 ConstraintConflict` |
| minimum step exhausted | reject without acceptance | `ERR-CT-009 MinimumStepExhausted` |
| owner support/duration/beginning join differs | reject entire attempt | `ERR-CT-010 SlabJoinMismatch` |
| owner attempts direct clock advancement | reject entire attempt | `ERR-CT-011 UnauthorizedClockAdvance` |
| event failure/replay/duplicate | atomic no-op | `ERR-CT-012 EventTransition` |
| same-tick event no-progress cycle | atomic no-op | `ERR-CT-013 EventNoProgressCycle` |
| ledger/ending-owner mismatch | reject entire acceptance | `ERR-CT-014 AtomicAcceptance` |
| restart schema/digest/policy mismatch | reject restore | `ERR-CT-015 RestartMismatch` |
| rejected state appears in restart/reduction/output | reject | `ERR-CT-016 RejectedStateLeak` |
| scheduled-once duplicate | reject | `ERR-CT-017 ScheduledOnceReplay` |
| publication before commit/after rollback | reject and expose nothing | `ERR-CT-018 PublicationState` |
| unsupported authority tuple | reject configuration | `ERR-CT-019 UnsupportedAuthorityTuple` |
| malformed/noncanonical serialization | reject | `ERR-CT-020 Serialization` |

Error precedence is the numeric order `ERR-CT-001` through `ERR-CT-020`.
Within a class, choose the earliest canonical owner/constraint/event identity.
Validation must not depend on hash-map iteration order.

The required boundary/API error variants are stable aliases within those
families: `InvalidTimeIdentity`, `ArithmeticOverflow`,
`ParentIdentityMismatch`, `OwnerSetMismatch`, `SegmentCoverage`,
`ConstraintBehindAcceptedTime`, `ConstraintPastParentEnd`,
`ZeroStepWithoutEventTransition`, `ConflictingEqualTimeConstraints`,
`MinimumStepExhaustion`, `SlabSupportMismatch`, `DurationBitsMismatch`,
`ParticipantSetMismatch`, `UnauthorizedClockAdvance`, `EventFailure`,
`EventReplay`, `EventNoProgressCycle`, `AtomicAcceptanceFailure`,
`RestartSchemaMismatch`, `ControllerPolicyMismatch`, `RejectedStateLeak`,
`ScheduledOnceReplay`, `PublicationBeforeParentCommit`,
`PublicationRetainedAfterRollback`, `UnsupportedAuthorityTuple`, and
`NoncanonicalSerialization`. More specific aliases inherit their enclosing
`ERR-CT-*` position and are ordered lexicographically when multiple aliases in
one family are simultaneously true.

## Invariants and invariant guard map

| Invariant ID | Statement | Authority | Evidence | Guard | Failure posture |
|---|---|---|---|---|---|
| INV-COUPLEDTIME-001 | Integer `u128` nanoseconds own time identity; binary64 seconds are derived once and shared bit-identically. | REF-CT-PHYSICAL, REF-CT-IEEE754 | `[INFERENCE][Static]` | wire/conversion/vector guards | `ERR-CT-001/002/010` |
| INV-COUPLEDTIME-002 | Supports are positive, half-open, ordered, gap-free, nonoverlapping, and exactly cover their admitted parent/segment. | REF-CT-PHYSICAL | `[INFERENCE][Static]` | constructor/coverage gate | `ERR-CT-001/005` |
| INV-COUPLEDTIME-003 | One staged clock exclusively owns accepted time; no physical owner advances it. | REF-CT-PHYSICAL | `[INFERENCE][Static]` | capability/API and poison test | `ERR-CT-011` |
| INV-COUPLEDTIME-004 | Rejected attempts consume no accepted chronology or physical identity and leave all accepted/staged state byte-identical. | REF-CT-TRANSACTION | `[DIRECT][Static]` | rollback digest comparison | `ERR-CT-016` |
| INV-COUPLEDTIME-005 | An accepted slab begins from one owner set, uses one support, closes ledgers, installs all participants atomically, and advances exactly once. | REF-CT-TRANSACTION, REF-CT-PHYSICAL | `[DIRECT][Static]` | slab-candidate join/commit | `ERR-CT-010/014` |
| INV-COUPLEDTIME-006 | The parent transaction ID increments once at complete parent commit; slab/event attempts never increment it. | REF-CT-TRANSACTION | `[DIRECT][Static]` | transaction receipt checks | `ERR-CT-003/014` |
| INV-COUPLEDTIME-007 | Event transitions advance no time, integrate no rate, explicitly join owner digests, close transfer ledgers, advance one event ordinal, and cannot replay. | REF-CT-SNOW, REF-CT-PHYSICAL | `[DIRECT][Static]` | event receipt/ledger/replay set | `ERR-CT-012/013/014` |
| INV-COUPLEDTIME-008 | The complete parent owner set is fixed; segment participants may change only through admitted segment/event authority; inactive owners are byte-identical except at admitted events. | REF-CT-SNOW, REF-CT-TRANSACTION | `[INFERENCE][Static]` | participant subset/owner digest | `ERR-CT-004/005/012` |
| INV-COUPLEDTIME-009 | Constraint reduction is deterministic by tick, class, owner, and digest; adopter controller physics remains outside clock authority. | REF-CT-PHYSICAL | `[INFERENCE][Static]` | reducer/policy digest vectors | `ERR-CT-006/008/015` |
| INV-COUPLEDTIME-010 | Restart contains every accepted temporal receipt and staged reduction/publication fact needed for equivalent continuation and no rejected iterate. | REF-CT-RESTART | `[DIRECT][Static]` | schema/roundtrip/poison tests | `ERR-CT-015/016/017` |
| INV-COUPLEDTIME-011 | Scheduled-once operations execute exactly once at their named boundary, independent of slab/attempt count. | REF-CT-PHYSICAL | `[INFERENCE][Static]` | receipt set | `ERR-CT-017` |
| INV-COUPLEDTIME-012 | Diagnostic reductions consume accepted values only and cannot affect physical identity or acceptance. | REF-CT-PHYSICAL | `[INFERENCE][Static]` | independent reconstruction/alias fixtures | `ERR-CT-016/018` |
| INV-COUPLEDTIME-013 | No staged publication is visible before one atomic parent commit; rollback exposes nothing. | REF-CT-TRANSACTION | `[DIRECT][Static]` | publication state machine | `ERR-CT-018` |
| INV-COUPLEDTIME-014 | Existing DirectV10 persisted-restart V1 bytes remain unchanged; this authority is additive/versioned. | REF-CT-RESTART | `[DIRECT][Static]` | exact legacy vector/manifest gate | governance `HOLD` / `ERR-CT-015` |
| INV-COUPLEDTIME-015 | Temporal operator classes determine retry, integration, sequencing, event, once-only, and reduction behavior without changing adopter equations. | REF-CT-PHYSICAL | `[INFERENCE][Static]` | operator ledger/profile tests | governance `HOLD` |
| INV-COUPLEDTIME-016 | `RichardsCoupledV1` requires `CoupledAdaptiveSupportV1`, persistent Lane D, signed top-face exchange, staged coupling, and atomic interval commit. | REF-CT-PACKAGE | `[DIRECT][Static]` | authority-tuple validator | `ERR-CT-019` |

## Canonical obligations

| Obligation ID | Requirement | Enforcement |
|---|---|---|
| OBL-COUPLEDTIME-001 | Canonical schema/model/vector identities bind origin, wire width, conversions, precedence, and every lineage field. | schema/profile/vector gates |
| OBL-COUPLEDTIME-002 | Independent reference model consumes frozen vectors only, imports no Rust, and calls no Rust binary for expected values. | source audit plus separately authored comparison test |
| OBL-COUPLEDTIME-003 | Reference consumer proves A+B/C, B-to-C event, A+C/B-terminal chronology with rejection, retry, restart, and atomic publication. | orchestrator integration test |
| OBL-COUPLEDTIME-004 | Operand lineage and wrong-answer fixtures independently reconstruct reductions and publication order; self-consistency is insufficient. | publication/reduction acceptance gate |
| OBL-COUPLEDTIME-005 | Dual independent authority review, disposition, correction, and dual verification pass before production Rust. | contract promotion gate |
| OBL-COUPLEDTIME-006 | V10 remains immutable; full-support V10/V11 equivalence belongs to Child 2B. | exact-diff/write-set audit |
| OBL-COUPLEDTIME-007 | Richards equations and controller policy remain outside this contract; Richards adoption must import this clock authority. | boundary and tuple audit |

## Symbol alias map

| Canonical symbol | Boundary/API name | Scope | Units check | Owner contract |
|---|---|---|---|---|
| `tau` | `ModelTimeNs` / `*_ns` | wire/runtime | exact nanoseconds | SC-COUPLEDTIME-001 |
| `S=[a,b)` | `TimeSupport` | wire/runtime | endpoints in ns | SC-COUPLEDTIME-001 |
| `Delta_t` | `duration_s_bits` / derived `f64` | kernel boundary | named ns-to-s conversion | SC-COUPLEDTIME-001 |
| `P` | `parent_interval_id` | receipt | identity, no units | SC-COUPLEDTIME-001 |
| `X` | `parent_transaction_id` | transaction/restart | identity, no units | SC-VEGETATIONTRANSACTION-001 + this contract |
| `O`, `A_g` | `complete_owner_set`, `active_participant_set` | runtime/restart | identity sets | SC-COUPLEDTIME-001 |
| `D_policy` | `controller_policy_sha256` | restart | digest, no units | adopter + SC-COUPLEDTIME-001 join |

## Constants and parameters

| Constant/parameter | Value/domain | Provenance | Custody |
|---|---|---|---|
| tick unit | exactly `1 ns` | model-definition choice | coupled-time authority |
| wire width | unsigned 128 bit | model-definition choice | coupled-time authority |
| seconds divisor | exact integer `1_000_000_000 ns/s`, represented as binary64 `1e9` for derived division | SI conversion | coupled-time authority |
| constraint precedence | Hard, Event, Output, Restart, Adaptive | architectural authority | coupled-time model definition |
| event precedence | explicit typed value, owner, context digest | architectural authority | event/adopter contract joined here |
| controller tolerances/minimum step | no universal value | adopter authority | adopter policy digest |

No controller constant, physical threshold, or empirical timestep is defined by
this contract.

## Unit-governance map

| Symbol | Declared units | Boundary registry entry | Conversion helper | Scalar exception | Publication metadata |
|---|---|---|---|---|---|
| `tau`, `S`, `Delta_tau` | integer ns | new coupled-time schema | none | `u128` is identity type, not physical scalar approximation | support start/end ns required |
| `Delta_t` | binary64 s | slab receipt bits | `duration_ns_to_f64_seconds_v1` | derived scalar admitted only at numerical kernel seam | exact source support and bits required |
| event proposal | binary64 s relative to parent | constraint/event schema | `event_seconds_to_tick_ties_even_v1` | proposal only; never canonical identity | quantized tick and source required |
| rates/integrals | adopter units | adopter registry | adopter-owned | none granted here | lineage must name support and units |

## Tolerance and numeric notes

Support, ordering, identity, ordinal, digest, replay, and coverage checks are
exact; no epsilon applies. Tick arithmetic is checked. Event quantization is
round-to-nearest, ties-to-even as specified above. The derived duration is
compared by exact binary64 bits across participants. Conservation tolerances,
when any, belong to the transferring adopter contract and must be digest-bound;
the timing authority merely requires their independently reconstructed ledger
to pass. Signed zero event proposals normalize to exact zero only after domain
validation; NaN and infinity always fail.

## Calibration and identifiability posture

`CALIBRATION_NOT_APPLICABLE`: this is an architectural/numerical chronology
contract with no fitted process parameter or empirical observation operator.

`science_implementation_status=AUTHORITY_DRAFT`;
`calibration_evidence_status=NOT_APPLICABLE`;
`identifiability_status=NOT_APPLICABLE`.

| Readiness obligation | Disposition | Evidence path/rationale |
|---|---|---|
| typed/enumerable parameter surface | PASS | constants/parameters and adopter boundary above |
| observation operator with units and scale | NOT_APPLICABLE | no empirical output claim |
| deterministic candidate execution | BLOCKED | production implementation follows authority release |
| objective reconstruction | NOT_APPLICABLE | no calibration objective |
| sensitivity analysis | NOT_APPLICABLE | no fitted parameters |
| identifiability/confounding analysis | NOT_APPLICABLE | no fitted parameters |
| boundary, saturation, and failure reporting | PASS | exact boundary rules and ERR-CT family |
| equifinality/uncertainty retention | NOT_APPLICABLE | no parameter inference |
| synthetic recovery | NOT_APPLICABLE | state-machine vectors replace calibration recovery |
| additional-data inventory | NOT_APPLICABLE | no empirical sufficiency claim |

The blocked deterministic implementation row is a current package phase gate,
not a calibration hold and cannot be reported as empirical validation.

## Test-vector obligations

The canonical vector population must distinguish correct answers from these
poisons:

- exact parent/segment/slab coverage; gaps, overlaps, reversed/zero support,
  ordinal and `u128` overflow;
- integer-to-binary64 duration bits, halfway event ties, one-bit neighbors, and
  quantization at start/interior/end boundaries;
- event at parent start, inside, and end; two same-tick events; event failure;
  no-progress cycle; restart immediately before/after; replay poison;
- A+B active/C unchanged, B-to-C event, then A+C active/B terminal;
- constraint behind cursor, past end, zero step without event, incompatible
  equal-time constraints, deterministic compatible ties, and minimum-step
  exhaustion;
- rejected/retried attempts with byte-identical accepted clock, owners,
  controller, ledgers, scheduled receipts, reduction, and publication buffer;
- mismatched support/duration bits/beginning digest/participant set, direct owner
  clock advance, partial owner acceptance, and ledger failure;
- uninterrupted versus mid-parent restart, policy/digest mismatch, malformed
  wire, rejected-iterate poison, event replay, scheduled-once replay, and exact
  legacy DirectV10 restart V1 byte protection;
- maximum over accepted plus rejected attempts, parent volume divided by nominal
  duration, pre-restart-only maximum, post-restart-only maximum, duplicate
  scheduled output, publication before final owner acceptance, and publication
  retained after rollback; and
- authority tuples: legacy hydrology with legacy fixed schedule admitted;
  `RichardsCoupledV1 + LegacyFixedSchedule`, whole-day nonpersistent Lane D, or
  legacy R4L mutation rejected.

Expected outputs include canonical receipt/digest identities, exact ticks and
duration bits, selected constraints, accepted owner bytes, event/scheduled
receipts, reduction state, publication order, typed error and precedence, and
proof of atomic no-op on every rejected path.

## Gap register and promotability

| Gap ID | Gap | Promotability |
|---|---|---|
| GAP-CT-001 | Independent schemas, model definition, vectors, and reference calculator are not yet admitted. | `HOLD` before authority release |
| GAP-CT-002 | Dual authority review, disposition, and dual verification are pending. | `HOLD` before registry promotion to approved/active |
| GAP-CT-003 | Production crate and orchestrator reference consumer are not implemented. | expected after authority release; blocks package completion, not contract review |
| GAP-CT-004 | V11 full-support compatibility and snow-covered carrier equations are Child 2B/2C. | explicitly out of scope; no V10/V11 efficacy claim |
| GAP-CT-005 | Richards equations/controller policy remain for `SC-RICHARDS-001`. | explicitly out of scope; tuple guard is current scope |

No comparator agreement can close these gaps. Promotion requires
`OBL-COUPLEDTIME-005`, complete guard/vector mapping, and no unresolved binding
contradiction.

## Binding Exposure Index

| Entry ID | Source | Status | Binding classification | Canonical binding IDs | Review gate | Notes |
|---|---|---|---|---|---|---|
| `BEI-CT-001` | coupled-time package Authority To Establish | `active` | `maps-to-existing-INV` | `INV-COUPLEDTIME-001, INV-COUPLEDTIME-002, INV-COUPLEDTIME-003, INV-COUPLEDTIME-004, INV-COUPLEDTIME-005, INV-COUPLEDTIME-006, INV-COUPLEDTIME-007, INV-COUPLEDTIME-008, INV-COUPLEDTIME-009, INV-COUPLEDTIME-010, INV-COUPLEDTIME-011, INV-COUPLEDTIME-012, INV-COUPLEDTIME-013, INV-COUPLEDTIME-014, INV-COUPLEDTIME-015, INV-COUPLEDTIME-016, OBL-COUPLEDTIME-001, OBL-COUPLEDTIME-002, OBL-COUPLEDTIME-003, OBL-COUPLEDTIME-004, OBL-COUPLEDTIME-005, OBL-COUPLEDTIME-006, OBL-COUPLEDTIME-007` | `flagged-binding-addition` | Entire new authority receives the mandatory contract cycle. |
| `BEI-CT-002` | terminal snow HOLD timing findings | `active` | `maps-to-existing-INV` | `INV-COUPLEDTIME-002, INV-COUPLEDTIME-007, INV-COUPLEDTIME-008, INV-COUPLEDTIME-010` | `flagged-binding-addition` | Preserves event, participant, and restart residue. |
| `BEI-CT-003` | Richards assessment timing recommendation | `active` | `maps-to-existing-INV` | `INV-COUPLEDTIME-003, INV-COUPLEDTIME-009, INV-COUPLEDTIME-016, OBL-COUPLEDTIME-007` | `flagged-binding-addition` | Imports chronology only, not Richards numerics. |

## Change log

| Date | Version | Change |
|---|---|---|
| 2026-08-20 | `1-rc1` | Authored complete coupled-time identity, event, participant, controller, restart, atomicity, and publication authority for independent review. |
