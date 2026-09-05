---
contract_id: SC-COUPLEDTIME-001
title: Coupled Time Support, Event, and Atomic Chronology Contract
status: approved
maturity: active
owner: openWEPP maintainers + time/numerics + transaction/restart reviewers
contract_version: 17
producer_scope:
  - OPENWEPP_COUPLED_TIME_SUPPORT_V1
  - Coupled parent-interval coordinator and staged clock
consumer_scope:
  - Segmented-support vegetation V11
  - Snow, land-surface-energy, surface-liquid, Lane D, Richards, plant, soil-thermal, biogeochemistry, restart, and publication adopters
evidence_level: static+independent_oracle+contract_vectors
last_reviewed: 2026-09-03
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
| `dt_min_pre`, `dt_min_post` | `u128 ns` | admitted minimum positive physical support for the pre/post active participant set | `minimum_support_ns` |
| `epsilon_t` | `u128 ns` | event-time displacement tolerance | `event_time_tolerance_ns` |
| `epsilon_M`, `epsilon_L`, `epsilon_E` | typed mass/energy units | terminal snow-mass, liquid-mass, and energy tolerances | `snow_mass_tolerance_kg_m2`, `liquid_mass_tolerance_kg_m2`, `energy_tolerance_j_m2` |
| `R_E` | dimensionless binary64 | ordered combined normalized mass/energy error used only for candidate ranking | `combined_normalized_mass_energy_error` |

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
physical progress. There is one narrow receipt-custody successor: an
`OwnershipTransfer` whose exact mutation set is empty and whose ending owners,
regime, and participants are byte-identical may remove one independently
validated pending custody event when its canonically ordered ledger is
nonempty, balanced, and every entry has nonzero debit, matching nonzero credit,
and nonzero operand-lineage digests. That transition advances the event and
segment ordinals exactly once and persists exactly one accepted event receipt;
it changes no time, rate, owner, regime, or participant state. Its domain owner
must independently prove positive output, exact first-hop mass and enthalpy
closure, zero retained mass and enthalpy, and at least one complete runoff or
outlet receipt. Ordinals, IDs, or receipts without that typed custody authority
do not count as progress. Any other empty no-op, missing/empty/unbalanced/zero
ledger authority, wrong event class, or replay remains `ERR-CT-013`. The cycle
key is the framed hash of tick, complete-owner digest,
regime/participant digest, and sorted pending-event semantic digests. A
repeated key or more than 256 transitions at one tick fails `ERR-CT-013`.
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
the complete ordered accepted-slab receipt chronology; accepted event-transition
and scheduled-once receipts; diagnostic/peak
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

`OPENWEPP_SNOW_STAGE3_V11_RESTART_V3` may replace resident completed-day
detail only after the complete day has been validated, written as canonical
uncompressed bytes to transaction-private durable storage, and acknowledged by
the exact content-addressed archive record. The append-only archive manifest
seals day count/order, prior and resulting ordered/content roots, complete
parent and publication receipt identities, beginning/ending owner sets, clock
and parent sequence, and the bounded qualification delta. A resident prefix
seals the same terminal roots/counts and qualification fold; only the active
day and exact next-owner/event/WB14 tail remain resident. The next day is not
admitted while an archive acknowledgement is pending.

Archive acknowledgement is an atomic owner operation. Any write, sync,
content-digest, manifest-append, publication-rotation, prefix-fold, or final
owner/count/root failure leaves the full completed day resident and changes no
prefix, publication history, owner, clock, or public output. Whole-run failure
discards the transaction-private archive together with all private output
spools. Restore V3 requires the matching manifest and every content-addressed
record and rejects omission, truncation, duplication, reorder, substitution,
wrong prior/final root, wrong day count, or wrong final owner. Uncompacted and
archived continuation must produce identical physical owners, accepted
publication rows, WB14 materialization, archive roots, qualification fold, and
restart continuation.

`OPENWEPP_COUPLED_TIME_RESTART_V2` retains the complete canonical owner bytes,
controller checkpoint bytes, accepted slab, event, and scheduled-once receipts,
operand receipt lineage for reductions, complete pending publication record
bytes, and durable outbox rows—not digests alone. The separately versioned
`OPENWEPP_COUPLED_TIME_SEMANTIC_VALIDATOR_V1` is mandatory in addition to JSON
Schema: it enforces checked numeric ranges, relational support/cursor bounds,
canonical ordering/uniqueness/cardinality, byte-to-digest reconstruction,
receipt chronology and replay exclusion, accepted-only reduction/publication
lineage, outbox/parent joins, and canonical reserialization equality.

`OPENWEPP_COUPLED_TIME_RESTART_V1` remains byte-identical and does not support
mid-parent continuation requiring authenticated slab chronology; admission to
that workflow fails typed. Accepted slab receipts are not reconstructable from reduction or publication
state: either surface is optional and may contain only a subset of slabs. The
restart wire therefore persists each accepted slab receipt in slab-ordinal
order, including its support, segment and constraint lineage, beginning and
ending clock/owner-set digests, duration bits, owner-candidate-set digest, and
coupled-ledger digest. Admission rejects an omission, duplicate, reorder,
noncontiguous support, ordinal gap, cursor mismatch, or digest/lineage mismatch.
Parent finalization after restore consumes this authenticated chronology exactly
as uninterrupted execution does.

V2 restart distinguishes `ActiveParent` and `CommittedParent`. The
`parent_transaction_sequence` always remains the sequence used to derive the
retained `parent_transaction_id`. `next_parent_transaction_sequence` equals it
while active and equals its checked successor after the atomic parent commit.
Thus a committed crash checkpoint preserves both the immutable committed-parent
identity and the already-consumed persistent increment; restore must not derive
the retained parent ID from the next sequence or increment it again.

`CommittedParent` contains exactly one durable outbox row and no pending
publication buffer. Its `parent_receipt_id` is reconstructed with the
`parent-receipt-v2` domain over the ordered accepted slab, event, and
scheduled-once receipt IDs. Its `publication_receipt_id` is reconstructed with
`publication-receipt-v2` over that parent receipt, ordered output record IDs,
outbox sequence, and `CommittedUndelivered` identity state. The outbox sequence
equals the committed parent sequence. Delivery state and attempt count may
advance without changing publication identity. An `ActiveParent` has no durable
outbox row and may retain a pending buffer.

After committed restore, the only outbox transitions are
`CommittedUndelivered -> DeliveredUnacknowledged -> Acknowledged`; crash or
restart preserves the current state, redelivery is permitted only from
`DeliveredUnacknowledged`, and acknowledged rows cannot redeliver. These
operations preserve parent/publication receipt identity and never increment the
parent sequence. Beginning the next parent consumes exactly the persisted
`next_parent_transaction_sequence`; the committed checkpoint itself cannot be
committed again.

Each persisted diagnostic reduction retains ordered
`(accepted_receipt_id, value_bits)` operands, not IDs alone. Admission requires
the operand-ID projection to equal `accepted_operand_receipt_ids`, requires
every ID to name an accepted slab/event/scheduled receipt, and independently
recomputes the declared maximum/minimum/sum bits. An empty reduction has no
operands and `value_bits = null`; zero is never used as an empty sentinel.
Operands and results must be finite binary64 values. Maximum and minimum scan
the persisted order and retain the first operand on numeric equality (including
signed-zero equality). Sum is a left fold in persisted order beginning at
positive zero; any nonfinite intermediate or result fails typed.

Outbox delivery attempt counts are phase coherent: `CommittedUndelivered` has
count zero, while `DeliveredUnacknowledged` and `Acknowledged` have count at
least one. Crash/restart preserves both state and count; each actual delivery or
redelivery increments the count exactly once.

V2 scheduled-once receipts use the closed `scheduled-receipt-v2` framed
identity over parent transaction, operation ID, boundary ID, tick, and result
digest. Event ordinal is not an input: scheduled execution and event-transition
chronology are distinct namespaces. Restore reconstructs this identity and
rejects cross-parent, altered-operation, altered-boundary, altered-tick, or
altered-result substitutions.

`boundary_id` is the closed `scheduled-boundary-v2` framed identity over parent
transaction, operation ID, and exact integer tick. The canonical scheduled-once
execution key is `(parent_transaction_id, operation_id, boundary_id)`. Exactly
one accepted receipt may exist for that key regardless of result digest or
receipt ID; a second correctly framed receipt is replay and fails typed.
Different admitted boundary identities remain distinct scheduled executions.

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
| same-tick event no-progress cycle, including any empty mutation without the exact typed receipt-custody exception | atomic no-op | `ERR-CT-013 EventNoProgressCycle` |
| ledger/ending-owner mismatch | reject entire acceptance | `ERR-CT-014 AtomicAcceptance` |
| restart schema/digest/policy mismatch | reject restore | `ERR-CT-015 RestartMismatch` |
| rejected state appears in restart/reduction/output | reject | `ERR-CT-016 RejectedStateLeak` |
| scheduled-once duplicate | reject | `ERR-CT-017 ScheduledOnceReplay` |
| publication before commit/after rollback | reject and expose nothing | `ERR-CT-018 PublicationState` |
| unsupported authority tuple | reject configuration | `ERR-CT-019 UnsupportedAuthorityTuple` |
| malformed/noncanonical serialization | reject | `ERR-CT-020 Serialization` |
| no candidate satisfies both neighbor supports and all independently admitted event tolerances | atomic parent retry/no-op; no owner or chronology mutation | `ERR-CT-021 EventBoundaryNoCandidate` |
| archive record/manifest/prefix/durable acknowledgement or V3 archive-reader mismatch | reject without rotation or owner mutation | `ERR-CT-027 ArchiveMismatch` |
| canonical covered role, zero-based ordinal, or support differs from the closed role sequence | reject before physical execution or custody construction | `ERR-CT-010 SlabJoinMismatch` |
| canonical covered nonfinal state appears in owner/restart/output custody | reject the complete candidate without exposure | `ERR-CT-016 RejectedStateLeak` |
| any covered map publishes, enqueues, exposes, or installs before accepted composed-parent commit | reject and expose nothing | `ERR-CT-018 PublicationState` |
| trusted accepted-publication support capability is foreign, stale, replayed, partially joined, or differs from its owned support/tail identity | consume the capability and reject before history mutation | numeric precedence among `ERR-CT-003/004/010/014/015` |
| accepted-publication support capability appears in wire, archive, restart, or an untrusted reconstruction | reject; perform full independent support and chronology validation without restoring the capability | numeric precedence among `ERR-CT-015/020/027` |

`ERR-CT-022` through `ERR-CT-026` are reserved historical candidate
identifiers and are not active version-8 failure authority. Active error
precedence is the numeric order `ERR-CT-001` through `ERR-CT-021`, followed by
`ERR-CT-027`.
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
`NoncanonicalSerialization`, and `EventBoundaryNoCandidate`. More specific aliases inherit their enclosing
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
| INV-COUPLEDTIME-007 | Event transitions advance no time, integrate no rate, explicitly join owner digests, close transfer ledgers, advance one event and segment ordinal, and cannot replay. An exact-empty-mutation, unchanged-owner/regime/participant `OwnershipTransfer` counts as progress only with a nonempty balanced ledger whose debit, matching credit, and lineage digests are all nonzero; every ordinary empty no-op fails `ERR-CT-013`. | REF-CT-SNOW, REF-CT-PHYSICAL | `[DIRECT][Static]` | event receipt/ledger/replay and typed receipt-custody vectors | `ERR-CT-012/013/014` |
| INV-COUPLEDTIME-008 | The complete parent owner set is fixed; segment participants may change only through admitted segment/event authority; inactive owners are byte-identical except at admitted events. | REF-CT-SNOW, REF-CT-TRANSACTION | `[INFERENCE][Static]` | participant subset/owner digest | `ERR-CT-004/005/012` |
| INV-COUPLEDTIME-009 | Constraint reduction is deterministic by tick, class, owner, and digest; adopter controller physics remains outside clock authority. | REF-CT-PHYSICAL | `[INFERENCE][Static]` | reducer/policy digest vectors | `ERR-CT-006/008/015` |
| INV-COUPLEDTIME-010 | Restart contains every accepted temporal receipt and staged reduction/publication fact needed for equivalent continuation and no rejected iterate. | REF-CT-RESTART | `[DIRECT][Static]` | schema/roundtrip/poison tests | `ERR-CT-015/016/017` |
| INV-COUPLEDTIME-011 | Scheduled-once operations execute exactly once at their named boundary, independent of slab/attempt count. | REF-CT-PHYSICAL | `[INFERENCE][Static]` | receipt set | `ERR-CT-017` |
| INV-COUPLEDTIME-012 | Diagnostic reductions consume accepted values only and cannot affect physical identity or acceptance. | REF-CT-PHYSICAL | `[INFERENCE][Static]` | independent reconstruction/alias fixtures | `ERR-CT-016/018` |
| INV-COUPLEDTIME-013 | No staged publication is visible before one atomic parent commit; rollback exposes nothing. | REF-CT-TRANSACTION | `[DIRECT][Static]` | publication state machine | `ERR-CT-018` |
| INV-COUPLEDTIME-014 | Existing DirectV10 persisted-restart V1 bytes remain unchanged; this authority is additive/versioned. | REF-CT-RESTART | `[DIRECT][Static]` | exact legacy vector/manifest gate | governance `HOLD` / `ERR-CT-015` |
| INV-COUPLEDTIME-015 | Temporal operator classes determine retry, integration, sequencing, event, once-only, and reduction behavior without changing adopter equations. | REF-CT-PHYSICAL | `[INFERENCE][Static]` | operator ledger/profile tests | governance `HOLD` |
| INV-COUPLEDTIME-016 | `RichardsCoupledV1` requires `CoupledAdaptiveSupportV1`, persistent Lane D, signed top-face exchange, staged coupling, and atomic interval commit. | REF-CT-PACKAGE | `[DIRECT][Static]` | authority-tuple validator | `ERR-CT-019` |
| INV-COUPLEDTIME-017 | Active physical participants use the maximum of their admitted minimum supports; structural clock identity is not a constitutive support promise. | Child 2C support authority | `[INFERENCE][Static]` | support receipt validator | `ERR-CT-021` |
| INV-COUPLEDTIME-018 | Event boundary candidates satisfy both neighbor-side support predicates and all four independently admitted tolerances. | Child 2C event authority | `[INFERENCE][Static]` | candidate validator | `ERR-CT-021` |
| INV-COUPLEDTIME-019 | Proposed and accepted ticks, candidate digest, errors, and tie-break identity are retained and replay-authenticated. | Child 2C receipt authority | `[INFERENCE][Static]` | event receipt validator | `ERR-CT-012/015` |
| INV-COUPLEDTIME-020 | A no-candidate event is an atomic retry/failure; it cannot drop, freeze, scale, or execute a below-domain successor. | Child 2C rollback authority | `[INFERENCE][Static]` | rollback validator | `ERR-CT-021` |
| INV-COUPLEDTIME-021 | Stage-3 adaptive candidates use exact integer 60-second (`60_000_000_000 ns`) quanta, tile the parent exactly, evaluate direct plus composed paths from immutable beginnings, and install only the composed result. Stable ordinary supports accept steps substantially larger than one quantum. | owner-selected adaptive model | `[DIRECT][Static] + [INFERENCE][Static] + [DIRECT][Ran]`; exact-60 focused and canonical one-day replacement evidence passed 2026-08-28 | grid/controller/receipt guards plus package one-day evidence | `ERR-CT-001/010/016/021` |
| INV-COUPLEDTIME-022 | A floor operation has exactly one physical trial and a typed `FloorAccepted` or `FloorRejected` decision; no split child or sub-floor continuation is fabricated. | owner-selected adaptive model | `[DIRECT][Static]` | floor admission and provider-call guards | `ERR-CT-001/021` |
| INV-COUPLEDTIME-023 | Direct/composed comparison spans the complete prognostic owner set with dimension-specific tolerances and exact discrete topology, event, parcel, ordering, and schema predicates. | transaction atomicity and owner contracts | `[DIRECT][Static] + [INFERENCE][Static]` | comparison vector and owner-set validator | `ERR-CT-010/012/014/016` |
| INV-COUPLEDTIME-024 | The canonical receipt chain is acyclic: parent request, direct trial, split child 1, split child 2, comparison, accepted microstep, optional event group/parcel set, ending owner set, parent receipt. Every node binds exact support, attempt, beginning/ending owner digests, forcing, topology, configuration, ledgers, and decision. | canonical framed SHA-256 and transaction authority | `[DIRECT][Static]` | receipt replay/poison validation | `ERR-CT-002/003/010/012/015/017` |
| INV-COUPLEDTIME-026 | Completed-day detail rotates only after durable exact archive acknowledgement. The content-addressed manifest, bounded prefix/qualification fold, resident active-day tail, publication-history prefix, and materialized WB14 tail must reconstruct the uncompacted owner/publication chronology exactly; missing or substituted archive authority fails closed. | transaction, restart, and canonical receipt authority | `[DIRECT][Static]` | archive/prefix/rotation/restart equality and poison gates | `ERR-CT-015/018/020/027` |
| INV-COUPLEDTIME-028 | A slab candidate may carry a private, non-wire validation proof minted only after complete semantic validation. The proof binds one live clock incarnation and its exact accepted revision: parent, cursor, segment, ordinals, accepted receipt counts, scheduled-once count, committed posture, support/duration, owner-set and ledger digests, slab identity, and receipt identity. Acceptance may move the already-validated immutable payload after exact scalar/revision joins; mutation, stale/foreign clocks, independent reconstruction, or restart invalidates the proof. Exact in-process clock clones share an incarnation, while restart creates a fresh incarnation after full semantic validation. Proof material never enters physics, identities, receipts, publication, or durable wire. | transaction atomicity, restart, and canonical receipt authority | `[INFERENCE][Static]` | private proof constructor/consumer, live-incarnation and exact-revision joins, restart freshness and poison gates | `ERR-CT-003/004/005/010/014/015/017/018` |
| INV-COUPLEDTIME-029 | A fully validated snow-free provisional owner transaction may yield one private move-only physical-reuse proof. A final slab on the same live clock revision, parent, segment, ordinal, support, beginning owners, ledger and non-slab inputs may consume it only when the provisional/final difference is the ending-owner-derived accepted-slab identity. The consumer reseals final receipt-dependent identities without executing any constitutive, LSE, hydrology, BGC, soil, or vegetation physics again, then proves the final complete owner set byte-identical to the validated physical ending. The proof is single-use, non-wire, absent from restart/checkpoint/publication, and stale, foreign, mutated, replayed, or post-restart use rejects atomically. No physical replay fallback is authorized. | transaction atomicity, exact owner custody, restart, and canonical receipt authority | `[INFERENCE][Static]` | private physical-reuse typestate, live-revision and complete non-slab identity joins, exact final-owner comparison, restart freshness and poison gates | `ERR-CT-003/004/005/010/014/015/016/018` |
| INV-COUPLEDTIME-030 | Every canonical covered-map role and zero-based ordinal charges exactly one physical map with one exclusive private disposition. `Initial@0` yields only the first validated physical endpoint. Each later `FixedPointAdjudication@1` or contiguous `MultisecantAdjudication(n)@(n+1)`, `n=1..=N`, must validate exact custody before yielding a non-Clone, non-wire pending map. Outer candidate-versus-own-output nonclosure consumes that value into iteration history without error. After outer closure, dependent-output nonclosure against the preceding authentic map consumes it into typed adaptive rejection without history. Only full closure consumes that same pending value once as the `FinalAccepted` outcome and continues into one private complete owner envelope; a constructor failure cannot reinterpret it. Thus `M=N+2`, `0<=N<=5`, and `2<=M<=7`. No map is replayed, promoted from a completed nonfinal endpoint, or allowed to advance the accepted clock, publish, or mutate live accepted state; only accepted composed-parent commit publishes. | transaction atomicity, exact covered-map charge, and canonical solver authority | `[INFERENCE][Static]` | closed role dispatcher, custody-before-pending gate, non-Clone pending typestate with exclusive history/rejection/final dispositions, separate charge/physical-endpoint/disposition/final-constructor/envelope/parent-publication counters, role/ordinal and state-leak poisons | outer nonclosure -> history, no error; dependent-only nonclosure/role/ordinal/support -> `ERR-CT-010`; owner/identity/custody/disposition -> `ERR-CT-003/004/014`; rejected-state leak -> `ERR-CT-016`; publication before commit/after rollback -> `ERR-CT-018`; physical and constructor errors retain their typed variants; numeric precedence, complete rollback |
| INV-COUPLEDTIME-032 | A fully validated accepted-publication support may be owned by one private, move-only, non-Clone, non-wire capability minted only after one complete semantic, operand-seal, and receipt-seal pass. The capability binds the exact immutable support payload and receipt to one process-local accepted-publication-history incarnation and its complete O(1) live-tail revision. Trusted append consumes it, admits only the exact live-revision and cached chronology/owner-tail join, and then performs the existing byte-preserving WB14 compaction and Arc/COW install; it performs no second support validation, operand/receipt reconstruction, serialization, prefix scan, or payload clone. Every successful support/event append and history rotation/replacement advances the live revision; exact in-process clones may share only the identical incarnation and revision. Independent construction, restart, wire, archive, and every untrusted reconstruction carry no capability and retain full independent support validation plus full chronology reconstruction before a fresh process-local incarnation is established. | transaction atomicity, exact publication chronology, restart, archive, and canonical receipt authority | `[INFERENCE][Static]` | private capability constructor/consumer, exact payload/incarnation/live-revision/tail joins, mutation revision advance, full untrusted validator, non-wire source guard, exhaustive stale/foreign/replay/restart poisons and counters | existing constructor errors before mint; then numeric precedence `ERR-CT-002/003/004/010/014/015/018/020/027`; failed append consumes the capability and preserves history/publication bytes and live revision exactly |

`INV-COUPLEDTIME-025` is a reserved historical candidate identifier and is not
active version-8 invariant authority.

The 2026-08-27 owner amendment changes only the Stage-3 minimum positive
adaptive support and its exact tiling grid, from 600 ms to 60 seconds. It does
not change constraint ordering, direct/composed ownership, conservation or
custody, participant/owner topology, receipt content/order, event atomicity,
restart/rollback, or fail-closed behavior. All earlier floor-dependent vector
results, attempt counts, event ticks, traces, and performance evidence are
superseded and require fresh 60-second execution; no rerun is claimed here.

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
| OBL-COUPLEDTIME-008 | Child 2C event receipts bind canonical ticks, participant/support receipts, immutable terminal-state and candidate ledgers, tolerance policy, tie rank, owner custody, and atomic retry identity. | schema, oracle, restart, and transaction gates |
| OBL-COUPLEDTIME-009 | Multi-day Stage-3 execution proves bounded one-active-day residency, exact uncompacted-versus-archived equality, durable archive rollback, streamed archive reconstruction, V3 restart admission/poisons, and flat retained-memory growth. | runner consumer, archive reader, restart, poison, and resource gates |
| OBL-COUPLEDTIME-011 | Prove one full validation per slab-candidate revision and no acceptance-time reconstruction, owner/ledger rehash, serialization, or payload clone. Preserve candidate, receipt, clock, and restart bytes and identities. Prove exact-clone acceptance plus atomic rejection for foreign/stale clocks, changed segment or scheduled-once revision, prior slab acceptance, wrong lineage, and pre-restart proof use; a freshly validated post-restore candidate must succeed. | constructor/acceptance counters, source guard, exact-byte vectors, restart and rollback poisons |
| OBL-COUPLEDTIME-012 | Prove exactly one physical owner-stack execution for each accepted snow-free support while retaining two independently constructed coupled slab candidates, final accepted-slab identity, exact final complete-owner bytes, one final publication, and unchanged restart/wire output. Force every non-slab identity apart, reuse the proof twice, cross a restart boundary, and poison the retained physical ending; every case must reject with byte-identical rollback and a fresh post-restore execution. | provider/physics counters, exact direct-versus-reuse comparison, publication counters, exhaustive identity/reuse/restart poisons, source guards |
| OBL-COUPLEDTIME-013 | For a successful canonical covered solve, prove `2 <= M <= 7` charged maps beneath the unchanged maximum-eight ceiling, `M` validated physical endpoints, `M-1` nonfinal outcomes, one final disposition of the last pending map, one final-constructor attempt, and one completed private envelope. Prove exactly one history, typed-rejection, or final disposition for each pending value; physical failure before a pending value; dependent-only rejection without history/final construction; and constructor failure after final disposition without reinterpretation. Preserve supports, adaptive retry, rollback, and accepted chronology; every map and failed/direct/unselected candidate publishes zero; only accepted composed-parent commit publishes once. Prove exact differential physical-prefix equality and typed role, ordinal, identity, regime, topology, disposition, and one-ULP rejection without replay, promotion, or fallback. | role/ordinal and physical-map counters, pending/exclusive-disposition counters, final-attempt/envelope/parent-publication counters, forced-complete differential oracle, exhaustive poisons, rollback and source guards |
| OBL-COUPLEDTIME-015 | Prove constructor-to-trusted-append validation-once custody on the real publication path. One successful support requires exactly one full-validation attempt/success, one operand seal, one receipt seal, one capability mint, one trusted-append attempt, one live-revision join, one chronology/owner-tail join, and one successful append, with zero append-time full validations, operand/receipt reconstructions, serializations, full-prefix scans, or support-payload clones. Prove the same counts for the legitimate snow-free reseal path. Exhaustively poison every bound live-tail field and every stale, foreign, replayed, rotated, replaced, post-event, and pre-restart capability; each must be consumed, return the existing numeric-precedence typed error, leave history/publication bytes and revision unchanged, and permit a freshly validated successor. Wire/archive/restart and untrusted restore must independently revalidate every support and reconstruct the complete chronology, mint no capability during decoding, and establish a fresh incarnation only after success. | exact validation/seal/mint/join/append counters, expected-red capability API test, real routing/rotation/snow-free/adaptive tests, exhaustive tail-field poison matrix, source guards, wire/archive/restart roundtrip and rollback vectors |

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
  no-progress cycle; receipt-bearing exact-empty-mutation `OwnershipTransfer`
  with unchanged owners/regime/participants and one ordinal/receipt advance;
  empty, zero, unbalanced, missing-lineage, wrong-class, and replay poisons;
  restart immediately before/after;
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

Version-7 Stage-3 vectors additionally cover odd 60-second-quantum tiling,
direct-invalid/composed-valid retry, exact floor acceptance and rejection,
initial-proposal and growth-history invariance, attempt-ordinal invariance,
complete-owner substitution, same-tick event groups, later event boundaries,
receiver exact-once posture, adaptive-boundary restart, and byte-identical
uninterrupted/restarted parent receipts. A complete-season fixed-floor run is
not an admitted qualification oracle.

Expected outputs include canonical receipt/digest identities, exact ticks and
duration bits, selected constraints, accepted owner bytes, event/scheduled
receipts, reduction state, publication order, typed error and precedence, and
proof of atomic no-op on every rejected path.

## Gap register and promotability

| Gap ID | Gap | Promotability |
|---|---|---|
| GAP-CT-001 | Independent schemas, model definition, vectors, and reference calculator were pending authority admission. | `AUTHORITY_ADMITTED` by Child 2C contract evidence |
| GAP-CT-002 | Dual authority review, disposition, and dual verification were pending. | `AUTHORITY_RELEASED` after Child 2C dual review and verification |
| GAP-CT-003 | Production crate and orchestrator reference consumer are not implemented. | expected after authority release; blocks package completion, not contract review |
| GAP-CT-004 | V11 full-support compatibility and snow-covered carrier equations are Child 2B/2C. | explicitly out of scope; no V10/V11 efficacy claim |
| GAP-CT-005 | Richards equations/controller policy remain for `SC-RICHARDS-001`. | explicitly out of scope; tuple guard is current scope |

No comparator agreement can close these gaps. Promotion requires
`OBL-COUPLEDTIME-005`, complete guard/vector mapping, and no unresolved binding
contradiction.

## Binding Exposure Index

| Entry ID | Source | Status | Binding classification | Canonical binding IDs | Review gate | Notes |
|---|---|---|---|---|---|---|
| `BEI-CT-001` | coupled-time package Authority To Establish | `active` | `maps-to-existing-INV` | `INV-COUPLEDTIME-001, INV-COUPLEDTIME-002, INV-COUPLEDTIME-003, INV-COUPLEDTIME-004, INV-COUPLEDTIME-005, INV-COUPLEDTIME-006, INV-COUPLEDTIME-007, INV-COUPLEDTIME-008, INV-COUPLEDTIME-009, INV-COUPLEDTIME-010, INV-COUPLEDTIME-011, INV-COUPLEDTIME-012, INV-COUPLEDTIME-013, INV-COUPLEDTIME-014, INV-COUPLEDTIME-015, INV-COUPLEDTIME-016, INV-COUPLEDTIME-026, OBL-COUPLEDTIME-001, OBL-COUPLEDTIME-002, OBL-COUPLEDTIME-003, OBL-COUPLEDTIME-004, OBL-COUPLEDTIME-005, OBL-COUPLEDTIME-006, OBL-COUPLEDTIME-007, OBL-COUPLEDTIME-009` | `flagged-binding-addition` | Entire new authority receives the mandatory contract cycle. |
| `BEI-CT-002` | terminal snow HOLD timing findings | `active` | `maps-to-existing-INV` | `INV-COUPLEDTIME-002, INV-COUPLEDTIME-007, INV-COUPLEDTIME-008, INV-COUPLEDTIME-010` | `flagged-binding-addition` | Preserves event, participant, and restart residue. |
| `BEI-CT-003` | Richards assessment timing recommendation | `active` | `maps-to-existing-INV` | `INV-COUPLEDTIME-003, INV-COUPLEDTIME-009, INV-COUPLEDTIME-016, OBL-COUPLEDTIME-007` | `flagged-binding-addition` | Imports chronology only, not Richards numerics. |
| `BEI-CT-CHILD2C` | `docs/work-packages/20260821-snow-stage3-shared-carrier-authority-closure-001/` | `active` | `maps-to-existing-INV` | `INV-COUPLEDTIME-017, INV-COUPLEDTIME-018, INV-COUPLEDTIME-019, INV-COUPLEDTIME-020, OBL-COUPLEDTIME-008` | `flagged-binding-addition` | Active-participant support, canonical event receipt, deterministic coalescing, and atomic no-candidate retry. |
| `CT-VALIDATED-SLAB-ACCEPTANCE` | Stage 3 throughput-recovery validation-once handoff | `active` | `maps-to-existing-INV` | `INV-COUPLEDTIME-004, INV-COUPLEDTIME-005, INV-COUPLEDTIME-008, INV-COUPLEDTIME-010, INV-COUPLEDTIME-024, INV-COUPLEDTIME-028, OBL-COUPLEDTIME-011` | `flagged-binding-addition` | Private move-only proof removes duplicate trusted in-process validation without changing chronology, errors, wire, or physics. |
| `CT-SNOW-FREE-PHYSICAL-REUSE` | Stage 3 snow-free accepted-slab reseal | `active` | `maps-to-existing-INV` | `INV-COUPLEDTIME-004, INV-COUPLEDTIME-005, INV-COUPLEDTIME-008, INV-COUPLEDTIME-010, INV-COUPLEDTIME-024, INV-COUPLEDTIME-029, OBL-COUPLEDTIME-012` | `flagged-binding-addition` | A private single-use proof permits identity-only final resealing after one validated snow-free physical execution; final owner bytes, publication, restart, and rollback remain exact. |
| `CT-COVERED-NONFINAL-PHYSICAL-ONLY` | Stage 3 canonical covered-map role split | `active` | `maps-to-existing-INV` | `INV-COUPLEDTIME-004, INV-COUPLEDTIME-005, INV-COUPLEDTIME-013, INV-COUPLEDTIME-021, INV-COUPLEDTIME-024, INV-COUPLEDTIME-030, OBL-COUPLEDTIME-013` | `flagged-binding-addition` | Every charged map retains its canonical physical role, but only `FinalAccepted` constructs complete publishable transaction custody; nonfinal physical endpoints cannot advance time or be promoted. |
| `CT-NATIVE-INACTIVE-PREFIX-TRANSITION` | Stage 3 represented-snow to snow-free parent-local transition | `active` | `maps-to-existing-INV` | `INV-COUPLEDTIME-005, INV-COUPLEDTIME-006, INV-COUPLEDTIME-024, INV-COUPLEDTIME-031, OBL-COUPLEDTIME-014` | `flagged-binding-addition` | A complete accepted represented-snow prefix may advance only the WB14 parent-local support cursor; its coupled chronology remains exact and the first physical WB14 child is ordinal zero. |
| `CT-VALIDATED-PUBLICATION-SUPPORT-APPEND` | Stage 3 accepted-publication support validation-once handoff | `active` | `maps-to-existing-INV` | `INV-COUPLEDTIME-004, INV-COUPLEDTIME-005, INV-COUPLEDTIME-010, INV-COUPLEDTIME-013, INV-COUPLEDTIME-024, INV-COUPLEDTIME-026, INV-COUPLEDTIME-032, OBL-COUPLEDTIME-015` | `flagged-binding-addition` | A private capability moves one already-validated support into the exact same-live-revision history after O(1) tail joins; all untrusted, restart, wire, and archive paths still validate and reconstruct independently. |

## Child 2C shared-carrier and event-boundary amendment

This version imports the released V11/Restart V3 chronology and adds the
support-admissibility and event-boundary authority required by the Child 2C
shared snow--canopy carrier. It changes no tick identity, parent transaction
identity, restart V1/V2 bytes, or owner commit rule.

### Active physical support aggregation

For every positive-duration segment, the coordinator first forms the ordered
active physical participant set `A_g` and reads one admitted minimum support
from each participant. The common physical minimum is exactly:

```text
common_minimum_support = max(minimum support of every active physical participant)
```

The maximum is taken over the active set for the segment being proposed, not
over inactive owners or the complete owner set. A one-nanosecond structural
clock interval remains a valid identity and event location, but a positive
physical segment is rejected before owner execution when its duration is below
this common minimum. The segment receipt retains the participant list,
individual support receipts, and the derived maximum. No physical owner may
convert the structural interval into a constitutive one-nanosecond advance.

### Terminal event-boundary coalescing

For parent support `[a,b)`, a terminal event proposal `t*`, pre-event minimum
support `dt_min_pre`, and post-event minimum support `dt_min_post`, enumerate
integer tick candidates in the independently admitted event-time tolerance
window around `t*`. A candidate `t` is support-admissible exactly when:

```text
t-a == 0 or t-a >= dt_min_pre
b-t == 0 or b-t >= dt_min_post
```

The coordinator recomputes the terminal state and all event ledgers at each
candidate; it may accept only a candidate that also passes the independently
admitted event-time, snow-mass, liquid-mass, and energy tolerances. Candidate
selection is deterministic:

1. smallest absolute displacement from `t*`;
2. lowest combined normalized mass/energy error;
3. earliest tick.

The combined score is calculated in this fixed order:

```text
R_E = snow_error / epsilon_M
    + liquid_error / epsilon_L
    + energy_error / epsilon_E
```

Each term is zero when its tolerance and error are both exactly zero. A
zero-tolerance term admits only exact zero error; it is not replaced with a
unit denominator. Candidate errors are reconstructed from the immutable
terminal state and ledgers, never accepted from caller-supplied diagnostics.

The event receipt stores both `proposed_event_tick` and
`accepted_event_tick`, the candidate set digest, both neighboring support
values, each error, the tie-break rank, and the retry/rollback identity. When
no candidate passes, the event attempt fails with `ERR-CT-021
EventBoundaryNoCandidate`; all owners, chronology, reductions, and receipts
remain byte-identical and the declared parent retry policy is invoked. No
remainder is dropped, no snow-free state is frozen, no longer LSE result is
scaled, and no below-domain physical solve is attempted.

The accepted chronology is normative:

```text
solve terminal event proposal
-> enumerate admissible boundary candidates
-> select deterministically
-> recompute terminal snow state and ledgers at selected tick
-> accept zero-duration custody transition
-> execute successor regime only when support is nonzero and admissible
```

### Child 2C guards and receipt fields

| ID | Binding rule | Guard/failure |
|---|---|---|
| `INV-COUPLEDTIME-017` | Active physical participants use the maximum of their admitted minimum supports; structural clock identity is not a constitutive support promise. | support receipt validator / `ERR-CT-021` |
| `INV-COUPLEDTIME-018` | Event boundary candidates satisfy both neighbor-side support predicates and all four independently admitted tolerances. | candidate validator / `ERR-CT-021` |
| `INV-COUPLEDTIME-019` | Proposed and accepted ticks, candidate digest, errors, and tie-break identity are retained and replay-authenticated. | event receipt validator / `ERR-CT-012`, `ERR-CT-015` |
| `INV-COUPLEDTIME-020` | A no-candidate event is an atomic retry/failure; it cannot drop, freeze, scale, or execute a below-domain successor. | rollback validator / `ERR-CT-021` |

The closed `EventBoundaryCoalescingReceiptV1` fields are
`parent_transaction_id`, `segment_id`, `event_id`, `proposed_event_tick`,
`accepted_event_tick`, `parent_start_tick`, `parent_end_tick`,
`pre_common_minimum_support`, `post_common_minimum_support`,
`candidate_ticks`, `candidate_digest`, `event_time_error_ns`,
`snow_mass_error_kg_m2`, `liquid_mass_error_kg_m2`, `energy_error_j_m2`,
`tie_break_rank`, `retry_policy_digest`, `begin_owner_digest`,
`ending_owner_digest`, and `receipt_id`. Tick and support fields are canonical
base-10 strings representing unsigned 128-bit values. The candidate list is
strictly sorted and unique; its digest covers the ordered candidates,
participant/support receipts, immutable terminal-state digest, tolerances,
candidate evaluation ledgers, and retry policy. A receipt cannot be
reconstructed from the accepted tick alone.

### Child 2C test obligations

The contract-derived population must include unequal sequential supports in
both orders, an exact common minimum, a structural one-nanosecond interval,
both neighbor-side violations, deterministic tie and tie-poison cases,
proposed/accepted tick divergence, no-candidate retry, owner-preserving
rejection, restart before/after the event, and wrong-regime flux rejection.

## Canonical Covered Pending-Adjudication Map Amendment

`INV-COUPLEDTIME-030` makes every charged covered-map role a closed chronology
and single-use custody decision. `Initial@0` executes from the immutable
candidate beginning and returns the first private validated physical endpoint.
The next charge is `FixedPointAdjudication@1`. If iteration remains necessary,
later charges are contiguous `MultisecantAdjudication(n)@(n+1)` for
`n=1..=N`, `0<=N<=5`. Every post-initial charge returns one private, non-Clone,
non-wire pending map only after exact physical, identity, and discrete-custody
validation. It contains its exact candidate, own physical output, support,
role ordinal, custody, and identities. No such pending value is yet an
iteration endpoint or a publishable final envelope.

The pending value has exactly one disposition. Compare its candidate against
its own output under the unchanged outer tolerance. Outer nonclosure consumes the pending value into iteration
history and permits the next multisecant adjudication. Outer closure then
requires dependent-output stability against the preceding authentic map.
Dependent-only nonclosure consumes it into the canonical adaptive response or
typed floor failure without admitting the value to history. When both checks close, the
same pending value is consumed once as the `FinalAccepted` outcome and
continues from its already-executed physical prefix through complete custody
construction. A physical or constructor failure retains its existing typed
variant and cannot reinterpret the pending value. The rule is identical for
ordinary and native represented-snow regimes.

The exact chronology is `Initial@0`, `FixedPointAdjudication@1`, then zero or
more contiguous `MultisecantAdjudication(n)@(n+1)` roles until one adjudication
is consumed as `FinalAccepted`. Therefore `M=N+2`, `0<=N<=5`, and
`2<=M<=7`. `FinalAccepted` is a disposition of the last charged adjudication,
not an additional charged role. There is no skipped, repeated, replayed, or
renumbered map. This correction preserves the existing maximum-eight budget as
a conservative ceiling while requiring at most seven charges.

Only the successful final disposition constructs and validates the complete
vegetation, land-surface-energy, surface-liquid, soil, biogeochemistry, joint,
receipt, and owner envelope. The initial endpoint and history values have no
conversion into candidate, owner, install, restart, or publication surfaces.
The private final envelope is publishable only by the enclosing transaction;
no map performs enqueue, exposure, install, publication, or live-clock
mutation. Direct, rejected, and unselected adaptive candidates publish zero.
Only the selected composed parent publishes, exactly once, at atomic commit.

`OBL-COUPLEDTIME-013` requires exact accounting for charges, validated physical
endpoints, pending values, history dispositions, final dispositions,
dependent-only rejections, constructor attempts, completed envelopes,
map-level publication, and accepted-parent publication. Every successful solve
has `M` charges and `M` validated physical endpoints, `M-1` nonfinal outcomes
(the initial endpoint plus any history dispositions), exactly one final
disposition, one final-constructor attempt, and one completed private envelope.
A charged physical failure has no validated pending value or disposition. An
outer-nonclosed pending map has exactly one history disposition. A dependent-
only failure has one validated pending value and one rejection disposition but
no history or final disposition. A final-constructor failure has one final
disposition and constructor attempt but no completed envelope. Every map and
failure publishes zero and rolls back byte-identically; the accepted composed
parent alone publishes once. Exact physical-prefix parity against a test-only
forced-complete reference and the complete role, identity, regime, topology,
disposition, and one-ULP poison matrix remain mandatory.

### Profile integration

| Profile surface | Binding |
| --- | --- |
| algorithm step | Charge `Initial`, then ordered pending adjudication maps; consume each pending map exactly once into history, rejection, or final construction. |
| branch/guard | A non-Clone pending typestate prevents replay, cross-disposition reuse, live-clock mutation, or promotion of a completed nonfinal endpoint. |
| invariant guard map | `INV-COUPLEDTIME-030` -> canonical role/ordinal validator, custody-before-pending gate, pending typestate, exclusive disposition gates, exact counters, final-only transaction constructor, clock/publication/rollback guards; outer nonclosure -> history/no error, dependent-only nonclosure/role/ordinal/support -> `ERR-CT-010`, owner/identity/custody/disposition -> `ERR-CT-003/004/014`, rejected-state leak -> `ERR-CT-016`, publication violation -> `ERR-CT-018`, in numeric precedence. |
| test vector | `OBL-COUPLEDTIME-013`: two-map and iterative success, physical/history/dependent/constructor failure matrices, exact role/support chronology, forced-complete parity, exhaustive disposition poisons, zero map publication, parent-only publication, unpublishability, rollback. |
| binding exposure | `CT-COVERED-NONFINAL-PHYSICAL-ONLY`, active, `new-INV`, IDs `030/013`, dual review/verification. |

## Change log

| Date | Version | Change |
|---|---|---|
| 2026-09-03 | `17` | Bound one private move-only accepted-publication support capability to an exact process-local history incarnation and complete live-tail revision. Trusted in-process append retains only exact chronology/tail joins plus existing byte-preserving compaction/COW install; duplicate support validation, sealing, serialization, prefix scanning, and payload cloning are forbidden. Restart, wire, archive, and untrusted reconstruction retain full independent validation and establish fresh incarnation authority. |
| 2026-09-03 | `16` | Corrected the two-map authority to consume the second charged physical map through a private pending-adjudication typestate: history on outer nonclosure, typed adaptive rejection on dependent-only nonclosure, or final-envelope construction on full closure. `FinalAccepted` is a disposition, not a replayed charge; exact chronologies now require two through seven maps under the unchanged maximum-eight ceiling. |
| 2026-09-03 | `15` | Authorized the exact two-map stable covered chronology after initial outer closure, with independent final outer/dependent closure and no Predictor fallthrough on final failure. Tolerances, physics, the eight-map ceiling, adaptive response, rollback, restart, and publication authority remain unchanged. |
| 2026-09-03 | `14` | Bound the coupled-receipt-backed represented-snow inactive prefix consumed by the first exact snow-free WB14 child; no under-snow physics, receipt, accepted-clock change, or publication is added. |
| 2026-09-02 | `13` | Bound canonical covered-map role to a private nonfinal physical-only result or the one independently charged final-complete owner envelope. Map count, roles, supports, chronology, adaptive response, rollback, restart, and publication authority remain unchanged. |
| 2026-09-02 | `12` | Bound one private move-only snow-free physical-reuse proof to the exact live provisional/final slab relation. Only accepted-slab-dependent identities may be resealed; physical execution remains once, final complete owners remain byte-identical, and wire/restart/publication/rollback semantics are unchanged. |
| 2026-09-02 | `11` | Bound private revision-scoped validation-once slab acceptance. Constructor semantics, chronology, owners, ledgers, receipts, restart/publication bytes, error precedence, and physics are unchanged; only duplicate trusted in-process reconstruction is removed. |
| 2026-08-28 | `9` | Admitted one typed receipt-custody progress successor for exact-empty-mutation `OwnershipTransfer` events with unchanged owners/regime/participants and a nonempty balanced nonzero debit/credit/lineage ledger. The successor advances one event/segment ordinal and one receipt only; all ordinary empty no-ops remain `ERR-CT-013`. |
| 2026-08-28 | `8` | Added mandatory durable content-addressed completed-day archival, bounded resident prefix/qualification fold, publication/WB14 rotation, transaction-private runner spooling, and fail-closed V3 archive-reader/restart authority without changing physical equations or accepted chronology. |
| 2026-08-27 | `7` owner amendment | Replaced the provisional 600-ms Stage-3 floor with an exact 60-second (`60_000_000_000 ns`) temporal floor. Conservation, custody, phase, topology, receipt, rollback, and fail-closed obligations are unchanged; stable ordinary supports must accept substantially larger steps. Prior floor-dependent evidence is superseded and awaits rerun. |
| 2026-08-26 | `7` | Replaced terminal root/localization chronology with exact-grid adaptive compositional stepping, composed-result ownership, typed floor decisions, complete-owner/discrete comparison, and acyclic adaptive/event receipt custody; versions 4-6 remain rejected historical candidates. |
| 2026-08-20 | `1-rc1` | Authored complete coupled-time identity, event, participant, controller, restart, atomicity, and publication authority for independent review. |
| 2026-08-20 | `1-rc2` | Added complete accepted-slab receipt chronology to restart after implementation exposed that reductions/publications cannot reconstruct parent finalization. |
| 2026-08-20 | `2` | Preserved restart V1, released restart V2 slab/event chronology, and closed scheduled-once receipt identity without borrowing event ordinals. |
| 2026-08-20 | `3` | Bound Child 2C active-participant maximum support, deterministic event-boundary coalescing, typed no-candidate retry, and the proposal/accepted event receipt. |

## Canonical Covered-Solver Adaptive Response Amendment

`INV-COUPLEDTIME-027` — A Stage-3 covered-solver eight-map exhaustion is one
rejected adaptive attempt. Above the exact 60-second floor, the next exact
support invokes the same canonical covered solver from the immutable beginning
owner; no alternate, historical, or recovery solver may run. At the floor,
exhaustion returns a typed evaluation-budget/nonconvergence error. Continuous
direct/composed replay uses the adopter's `TOL-SNOWENERGY-007`; exact support,
owner, topology, event, parcel, ordering, schema, receipt chain, and rollback
rules in `INV-COUPLEDTIME-021`--`024` are unchanged.

`OBL-COUPLEDTIME-010` — Tests must poison an alternate-solver dispatch,
uncharged map, support mutation, floor split, and discrete receipt mutation,
and prove exact rollback plus same-solver smaller-support retry.

### Profile integration

| Profile surface | Binding |
| --- | --- |
| algorithm step | Reduce the rejected covered attempt to the next exact support, then re-enter the same solver from immutable owners. |
| branch/guard | Above floor: same-solver retry; exact floor: typed `ERR-CT-021`-class failure; alternate solver/floor split refuses. |
| invariant guard map | `INV-COUPLEDTIME-027` -> adaptive rejection, exact-support reducer, receipt/rollback validator. |
| test vector | `OBL-COUPLEDTIME-010`: alternate dispatch, uncharged call, support mutation, floor split, discrete poison. |
| binding exposure | `CT-STAGE3-CANONICAL-RETRY`, active, `new-INV`, IDs `027/010`, dual review/verification. |
| gap | `GAP-CT-003` remains open only for real production orchestration proof. |
| change log | 2026-09-01, contract 10: bound same-solver Stage-3 adaptive response and typed floor exhaustion. |

## Native Inactive-Prefix Parent-Local Chronology Amendment

`INV-COUPLEDTIME-031` — If an accepted parent begins with represented snow and
the exact next positive child is snow-free, the already accepted coupled slabs
before that child form one immutable inactive prefix. A typed proof must replay
the complete ordered slab and complete-owner chain from parent start through
the snow-free child start with no gap, overlap, omission, duplication, or
identity substitution. The accepted coupled clock, slab receipts, event
chronology, and complete owners are not advanced or rewritten by consuming the
proof.

The SurfaceLiquid/WB14 adopter may use that proof only to establish its
parent-local physical-support cursor at the same exact tick. This chronology-
only establishment performs no inactive-regime WB14 physics or receipt and
does not consume a physical-child ordinal; the first snow-free WB14 child is
ordinal zero. The proof is bound into adopter replay/restart and final parent
partition identity. Rejection preserves the complete accepted and candidate
owner sets, clocks, receipts, and publication buffers byte-for-byte.

`OBL-COUPLEDTIME-014` — Prove a real represented-snow prefix followed by one
snow-free child, first physical ordinal zero, complete coupled/adopter support
partition, uninterrupted versus split-restart identity, zero inactive physics
and publication, and exact rollback. Poison every prefix row identity/order/
support/slab/owner/marker, prefix endpoint, duplicate consumption, nonzero
first ordinal, and adopter-cursor advance without proof.

| Profile surface | Binding |
| --- | --- |
| algorithm step | Replay the complete accepted inactive prefix and grant one chronology-only adopter cursor establishment at its exact end. |
| branch/guard | The proof cannot change coupled accepted state, synthesize a physical receipt, or consume a physical ordinal. |
| invariant guard map | `INV-COUPLEDTIME-031` -> accepted-slab/owner replay, adopter proof digest, cursor/ordinal join, restart and rollback. |
| test vector | `OBL-COUPLEDTIME-014`: transition, support/identity poisons, ordinal zero, restart, zero work/publication, rollback. |
| binding exposure | `CT-NATIVE-INACTIVE-PREFIX-TRANSITION`, active, `new-INV`, IDs `031/014`, dual review/verification. |

## Validated Accepted-Publication Support Append Amendment

`INV-COUPLEDTIME-032` removes only the second full validation currently paid
when a newly constructed accepted-publication support enters its trusted
in-process history. `Stage3AcceptedPublicationSupportV1::try_new`, and the
legitimate snow-free identity-only reseal that produces the same support type,
must complete the existing semantic checks, operand reconstruction/seal, and
receipt reconstruction/seal exactly once. They accept the exact target
`AcceptedPublicationHistoryLiveRevisionV1`; only successful validation may
mint a `ValidatedStage3AcceptedPublicationSupportV1` that owns the immutable
support and binds its payload/receipt identity to that process-local history
incarnation and complete cached live-tail revision. Constructor failure returns
the existing typed error and mints nothing.

The capability is private, move-only, non-Clone, and has no Serialize,
Deserialize, wire, archive, restart, checkpoint, publication, or public API
representation. Trusted `push_validated_support` consumes it. Before mutation,
the consumer compares its exact history incarnation and every live-revision
field, then applies only the existing O(1) cached support chronology and
beginning/ending-owner tail join. On success it may perform the unchanged
byte-preserving WB14 replay compaction and Arc/COW handle installation. It must
not call `Stage3AcceptedPublicationSupportV1::validate`, reconstruct operand or
receipt hashes, serialize the support, scan the retained prefix, or clone the
support payload. A failed attempt consumes the capability and leaves the
history, publication state, and live revision byte-identical.

The live revision binds the history incarnation, accepted support/event counts,
last day and interval, last support, last parent transaction, last accepted
slab, traversed ending owner, pending pre-support event, event and event-ordinal
authority, sealed-prefix/rotation posture, WB14 tail/checkpoint identity, and
aggregate tail digest. Every successful support append, event append, or
history rotation/replacement advances or replaces that history value's
revision. Exact in-process history clones may share an incarnation only while
their revisions are identical; mutation makes a capability for the prior
revision stale against the mutated value. Independently constructed histories
and all restored histories receive a fresh incarnation after complete
independent validation.

Wire, archive, checkpoint, restart, and other untrusted reconstruction never
encode, decode, infer, or restore this capability. They continue to validate
every support independently and reconstruct the complete ordered chronology
and cached tail before admitting a fresh live history. This amendment changes
no support value, history order, compaction bytes, receipt, publication,
archive, restart, physics, or accepted chronology. It requires no amendment to
the land-surface-energy, surface-liquid, snow-energy, or vegetation-transaction
contracts: their existing immutable validated-handoff, publication, restart,
and custody authority is unchanged.

Error selection retains numeric precedence. Overflow is `ERR-CT-002`; foreign
history incarnation or support/payload identity is `ERR-CT-003`; owner-tail
handoff mismatch is `ERR-CT-004`; stale support/event/tail chronology is
`ERR-CT-010`; forged, replayed, partially joined, or already-consumed append is
`ERR-CT-014`; a pre-restart capability or invalid restored history is
`ERR-CT-015`; publication-state violation is `ERR-CT-018`; capability encoding
or noncanonical wire reconstruction is `ERR-CT-020`; and archive/rotation
authority mismatch is `ERR-CT-027`. Underlying support validation errors keep
their existing variants and precedence before capability minting.

`OBL-COUPLEDTIME-015` requires exhaustive counter and poison evidence. The
ordinary and snow-free success paths each record exactly one full-validation
attempt/success, operand seal, receipt seal, capability mint, trusted-append
attempt, live-revision join, chronology/owner-tail join, and successful append.
They record zero append-time full validations, operand/receipt reconstructions,
serializations, full-prefix scans, or support-payload clones. Untrusted restore
records one full validation per support and zero restored capabilities. Every
individual live-tail field, foreign incarnation, intervening support/event,
rotation/replacement, replay, and restart boundary receives a fail-closed,
byte-identical rollback vector, followed by successful freshly validated use.

### Profile integration

| Profile surface | Binding |
| --- | --- |
| algorithm step | Fully validate and seal one support against the exact target live revision, mint one owning capability, then consume it through `push_validated_support` for exact live-revision and cached chronology/owner-tail joins plus unchanged WB14 compaction/Arc-COW installation. |
| branch/guard | Constructor error mints nothing; trusted mismatch consumes the capability and mutates nothing; restart/wire/archive/untrusted input cannot carry a capability and enters the full validator/reconstructor. |
| invariant guard map | `INV-COUPLEDTIME-032` -> private non-Clone/non-wire capability, exact payload/incarnation/revision join, mutation revision advance, full untrusted validator, numeric-precedence error map, rollback and source guards. |
| test vector | `OBL-COUPLEDTIME-015`: validation-once hot success, snow-free reseal, stale/foreign/replay/restart/rotation poisons, exhaustive tail-field matrix, untrusted restore, non-wire/source guards, exact counters and byte-identical rollback. |
| binding exposure | `CT-VALIDATED-PUBLICATION-SUPPORT-APPEND`, active, `new-INV`, IDs `032/015`, dual review/verification. |
| change log | 2026-09-03, contract 17: private same-live-revision validated-support append; no wire, archive, restart, publication, chronology, or physics change. |
