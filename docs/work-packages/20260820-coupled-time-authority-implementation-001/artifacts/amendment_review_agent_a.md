# SC-COUPLEDTIME-001 restart amendment review A

Reviewer role: independent time/numerics/chronology authority review
Scope: `1-rc2` amendment adding `accepted_slab_receipts`; canonical restart
schema; semantic validator; poison population
Production Rust: not reviewed or edited
Date: 2026-08-20

## Verdict

**HOLD — authority amendment is directionally necessary, but its executable
admission evidence does not yet authenticate accepted chronology.**

Persisting the complete ordered slab-receipt sequence is the correct resolution
of the implementation-discovered contradiction: reductions and publication are
optional projections and cannot reconstruct all accepted slabs. The current
wire shape contains the principal operands needed for parent finalization.
However, the mandatory semantic validator presently checks only ordinal/support
coverage and digest syntax. It accepts mutually inconsistent duration,
identity, clock, owner, segment, constraint, candidate, and ledger lineage.
That contradicts the amendment's explicit fail-closed admission claim.

## Read-only evidence

Commands executed from `/workdir/openWEPP`:

```text
python3 docs/work-packages/20260820-coupled-time-authority-implementation-001/artifacts/semantic_schema_validator.py \
  --poisons docs/work-packages/20260820-coupled-time-authority-implementation-001/artifacts/semantic-schema-poisons.json
```

Result: the runner reported every declared expectation satisfied (36 results:
4 canonical-serialization cases plus 32 document cases). This establishes that
the runner is executable, not that the missing semantic relationships are
covered.

Independent IEEE-754 reconstruction:

```text
5 ns / 1_000_000_000 = 5e-9 -> binary64 bits 3e35798ee2308c3a
1 ns / 1_000_000_000 = 1e-9 -> binary64 bits 3e112e0be826d695
```

The poison baseline declares support `[0,5)` ns but stores
`duration_bits=3e112e0be826d695`. `valid_restart` is nevertheless accepted.
This directly falsifies the required integer-support-to-common-duration join.

## Findings

| ID | Severity | Finding | Required disposition |
|---|---:|---|---|
| AMEND-A-001 | BLOCKER | The validator checks `duration_bits` only as 16 lowercase hex digits. It does not derive the binary64 value from `support.end_ns - support.start_ns` using the contract's round-to-nearest-ties-to-even conversion. The baseline itself is a demonstrated false acceptance: a 5 ns slab carries the 1 ns bits. | Correct the baseline; derive and compare exact duration bits during semantic admission; add wrong-duration poisons for ordinary, rounding-boundary, large-`u128`, and exact-halfway cases already governed by the conversion authority. |
| AMEND-A-002 | BLOCKER | Receipt and accepted-slab identities are unauthenticated. `receipt_id` and `accepted_slab_id` are accepted if merely shaped like SHA-256. The validator never reconstructs either closed framed identity from the parent transaction, ordinal, segment, support, duration, owner, constraint, clock, and ledger operands. Thus arbitrary replacement of either ID is admitted and parent finalization after restore is not equivalent to uninterrupted finalization. | Freeze/reconcile the exact receipt preimages, recompute both IDs on admission, and add single-bit poisons for each identity and every bound operand. If an operand digest represents canonical retained bytes, persist those bytes or another independently reconstructable authenticated object rather than trusting a free digest. |
| AMEND-A-003 | BLOCKER | Cross-receipt chronology is incomplete. Support and ordinals are contiguous, but `end_clock_sha256[k] == begin_clock_sha256[k+1]` and `end_owner_set_sha256[k] == begin_owner_set_sha256[k+1]` are not checked; the first receipt is not joined to the parent beginning state and the final receipt is not joined to the restored cursor/current owner set. Event receipts at a shared tick are also not interleaved with slab owner/clock transitions. A sequence can cover time while describing disconnected state histories. | Specify the slab/event merge order at each boundary and validate the complete clock/owner chain from parent beginning through slabs/events to the restored boundary. Add two-slab, pre-event, post-event, same-tick-event, broken-clock-link, broken-owner-link, wrong-first-link, and wrong-terminal-link fixtures. |
| AMEND-A-004 | HIGH | Segment lineage cannot be authenticated for completed segments from this restart wire. Each slab stores only `segment_id`; only the currently active segment retains ordinal, regime, support, and participants needed to reconstruct a `SegmentId`. A past segment ID is therefore an opaque caller-supplied digest. `next_segment_ordinal` also is not related to receipt segment transitions. | Persist a canonical ordered segment receipt/definition history, or make each slab receipt retain the complete segment identity operands and define repeated-segment consistency. Validate segment ordering, coverage, participant membership, event-authorized transitions, active-segment terminal join, and `next_segment_ordinal`. Add a multi-segment A+B -> event -> A+C restart fixture and segment-ID/ordinal/regime/participant poisons. |
| AMEND-A-005 | HIGH | The prose says admission rejects duplicate and reordered slab receipts, but the current check derives ordinal from array position and support continuity only. A duplicated receipt with edited ordinal/support and arbitrary replacement digests remains admissible because IDs and lineage are not reconstructed. The schema metadata also says generically that “receipt arrays are strictly receipt-id ordered,” conflicting with the amendment's slab-ordinal order. | State one canonical order per receipt array. For slabs, require ordinal order and uniqueness of receipt ID, accepted-slab ID, and chronological identity; remove the contradictory generic receipt-ID ordering statement or scope it to the other arrays. Add duplicate/reorder cases with at least two valid slabs. |
| AMEND-A-006 | HIGH | The poison population covers omission, ordinal gap, support gap, cursor mismatch, and malformed digest text only. It does not exercise well-formed-but-wrong receipt IDs/digests, duration mismatch, two-receipt reorder/duplicate, clock/owner chain breaks, wrong parent/segment joins, constraint/candidate/ledger substitution, event interleaving, or uninterrupted/restored terminal parent-receipt equality. Consequently the passing gate is not an anti-tautology acceptance suite for the amendment's claims. | Add alias-separating semantic poisons and at least one positive multi-slab/multi-segment event vector. Independently reconstruct the terminal ordered parent receipt from uninterrupted and restored paths and require exact equality. |

## Authority consistency notes

- Adding accepted slab receipts is necessary and is within the coupled-time
  restart authority; it does not import adopter-specific controller physics.
- Complete positive-duration support coverage from parent start through
  `accepted_until` is appropriate. Zero-duration events must be merged into the
  state chronology without appearing as slabs.
- `maxItems: 65536` is a defensible bounded wire limit, but the runtime must
  fail typed before exceeding it; silent loss or compaction of accepted receipt
  chronology would violate exact restoration.
- The existing DirectV10 restart protection is retained in prose. This review
  did not observe or authorize any change to that released wire.

## Release condition

Amendment review A can pass after all six findings are dispositioned, the
semantic validator reconstructs rather than trusts the accepted chronology,
the expanded positive/poison population passes, and an independent gate proves
uninterrupted versus restored terminal parent identity and ordered receipt
equality across a slab-event-segment boundary.

---

## V2 re-review — 2026-08-20

Candidate: `OPENWEPP_COUPLED_TIME_RESTART_V2` with released V1 restored
byte-identically.

### Verdict

**HOLD — materially improved, but the central slab-event-slab chronology is
still not admissible and the beginning chronology remains unanchored.**

The correction closes AMEND-A-001's demonstrated duration false acceptance,
reconstructs the V2 slab/event IDs, adds several well-formed poisons, preserves
V1 exactly, and provides an independently written terminal finalization
calculation. Those are substantive corrections. Two blockers remain before the
amendment can be released.

### Read-only gates

```text
semantic_schema_validator.py --poisons semantic-schema-poisons.json
  PASS: 41/41 declared semantic/canonical cases

restart_finalization_reference.py
  PASS: parent_receipt_id=c17f03bc...a287
        publication_receipt_id=8b217e51...36c

cmp restart-schema.json \
    <(git show 30e82ab16:.../restart-schema.json)
  PASS: released V1 is byte-identical to the authority checkpoint
```

### Finding disposition from the first review

| Prior finding | Re-review disposition |
|---|---|
| AMEND-A-001 | **CLOSED for the exercised range.** The 5 ns baseline now carries `3e35798ee2308c3a`; admission derives duration from integer support and rejects the well-formed 1 ns alias. Boundary/large-`u128` population remains desirable but is not the present blocker. |
| AMEND-A-002 | **PARTIAL.** Slab, slab-receipt, event, and event-receipt identities are reconstructed. Parent transaction and segment anchors are still not reconstructed; see V2-A-001/V2-A-002. |
| AMEND-A-003 | **OPEN/BLOCKER.** Consecutive slab links and a terminal slab-to-event link are checked, but the general merged chronology is not; see V2-A-001. |
| AMEND-A-004 | **OPEN/HIGH through the parent/segment anchor defect.** Treating a segment digest as immutable lineage is only sufficient if its parent/root identity is authenticated and event-authorized segment changes are joined. See V2-A-002. |
| AMEND-A-005 | **PARTIAL.** Framed identities prevent simple duplicate substitution, but there is still no two-slab/reorder positive/negative population and the validator cannot admit the required event-mediated transition between two slabs. |
| AMEND-A-006 | **PARTIAL.** Well-formed duration/parent/owner/event poisons and a separate finalization calculator were added. The positive fixture remains one slab followed by a terminal event; it does not exercise the required slab-event-slab restored chronology. |

### Remaining findings

| ID | Severity | Finding | Required disposition |
|---|---:|---|---|
| V2-A-001 | BLOCKER | The validator processes and chains **all slabs first**, then processes **all events** starting from the last slab. This admits only events after the final accepted slab. For the defining terminal transition `[a,b) slab -> event at b -> [b,c) slab`, the slab loop requires the post-event slab's beginning owner/clock digest to equal the pre-event slab's ending digest, while the later event loop requires the event to begin from the final post-event slab. A real ownership-changing event between slabs therefore cannot validate. Sorting the separate event array does not merge it into chronology by `(tick, precedence, ordinal)`. | Validate a single deterministic boundary chronology: slab ending at tick `b`, then all admitted same-tick events in precedence/ordinal order, then slab beginning at `b`. Join owner and clock digests at every edge. Add a positive two-segment A+B slab -> B-to-C event -> A+C slab restart and poisons for event-before/after misplacement, missing event, duplicate same-tick event, wrong precedence, post-event slab using pre-event owner/clock, and restart immediately on both sides. |
| V2-A-002 | BLOCKER | The first slab's beginning owner/clock and its segment ID are not joined to an authenticated parent root. `parent_transaction_id` is syntax-checked but never reconstructed from `run_identity_sha256`, sequence, parent interval ID, and beginning owner set, despite the frozen `parent-transaction` preimage. `expected_begin_owner` and `expected_begin_clock` start as `None`, so arbitrary first values are accepted if descendant IDs are recomputed. Similarly, completed `segment_id` values remain opaque and `next_segment_ordinal` is not related to authenticated segment transitions. The independent finalization script then uses the unanchored first slab owner digest as authoritative `begin_owner_set`. | Persist the necessary beginning parent/clock and segment-definition operands (or an independently authenticated canonical parent/segment receipt), reconstruct `parent_transaction_id` and every `segment_id`, and join the first slab to that root. Derive/validate `next_segment_ordinal`, active segment, and terminal clock state. Add well-formed recomputed-tree poisons that alter the first beginning owner, first clock, parent sequence/run/interval, completed segment regime/participants/support, and next segment ordinal. |

### Finalization evidence limitation

`restart_finalization_reference.py` is genuinely separate from Rust and
reconstructs frozen receipt IDs rather than merely echoing them. Its current
fixture is nevertheless one slab plus an event at the cursor, with empty
publication records. It proves deterministic hashing for that narrow terminal
shape; it does not prove uninterrupted/restored equality across the required
slab-event-slab boundary. That broader fixture is the release condition named
in the first review and remains unmet.

### V2 release condition

Review A passes when V2-A-001 and V2-A-002 are corrected and the focused gates
include a positive, independently reconstructed two-segment chronology with an
interior ownership event. No production implementation should resume before
that authority checkpoint.

---

## Final V2 re-review — 2026-08-20

### Verdict

**PASS — amendment review A approves `OPENWEPP_COUPLED_TIME_RESTART_V2` for the
authority release checkpoint.**

The final correction closes V2-A-001 and V2-A-002. The V2 semantic boundary now
reconstructs the parent interval and parent transaction from their closed
preimages, reconstructs the active segment identity, anchors the first accepted
owner/clock to explicit parent-begin fields, reconstructs slab/event receipt
identities, validates exact duration bits, cursor coverage, last accepted step,
and next slab/event/segment ordinals, and evaluates slab endings and events as
one deterministic tick-ordered action chronology. An interior event at tick
`b` is therefore joined after the slab ending at `b` and before a later slab
ending after `b`.

The legacy `slab-receipt` identity domain is restored, so this additive restart
amendment no longer changes the released 108-case V1 oracle surface. V2-specific
receipt domains remain separately named.

### Final read-only gate evidence

| Gate | Result |
|---|---|
| Complete reference oracle | **PASS — 108/108 cases** |
| V2 semantic/canonical poison population | **PASS — 47/47 cases** |
| Independent merged chronology fixture | **PASS —** `6b131695fda7f600344dc7c706f63e8c1cf86ef41ab72afd5583b8b76ff25971` |
| Independent restored finalization KAT | **PASS —** parent `90627286...fda3`, publication `5faa32af...4602` |
| Released restart V1 versus authority checkpoint `30e82ab16` | **PASS — byte-identical** |

Commands included:

```text
python3 artifacts/reference_model.py artifacts/coupled-time-vectors.json
python3 artifacts/semantic_schema_validator.py \
  --poisons artifacts/semantic-schema-poisons.json
python3 artifacts/merged_chronology_reference.py
python3 artifacts/restart_finalization_reference.py
cmp artifacts/restart-schema.json \
  <(git show 30e82ab16:.../restart-schema.json)
```

### Closure of remaining findings

| Finding | Final disposition |
|---|---|
| V2-A-001 | **CLOSED.** The validator constructs one action sequence ordered by tick, slab-before-event precedence at a shared ending tick, then event ordinal. Beginning and ending owner/clock digests chain through that merged sequence. The independent positive fixture executes `[0,5)` A+B, B-to-C at tick 5, then `[5,10)` A+C and freezes the terminal chronology digest. |
| V2-A-002 | **CLOSED.** Explicit beginning owner/clock fields anchor the sequence; parent interval and transaction IDs are reconstructed from run/calendar/forcing/support/sequence/beginning owner operands; active segment identity and next segment ordinal are reconstructed/validated; next slab/event ordinals and last accepted step are joined to accepted chronology. Well-formed parent, transaction, initial-clock, active-segment, and ordinal poisons fail admission. |
| AMEND-A-001 through AMEND-A-006 | **CLOSED for authority release.** Exact duration, framed identities, merged chronology, root joins, order/ordinal constraints, well-formed poisons, V1 protection, and independent restored finalization evidence are now executable. |

### Implementation boundary retained

This PASS releases the amended authority; it does not approve the paused Rust
implementation. Production restart admission must implement the same V2 joins,
must create slab/segment/event receipts only through admitted authority paths,
and must be re-reviewed against the final implementation and reference consumer.
Any production bypass or divergence remains a package-level blocker.
