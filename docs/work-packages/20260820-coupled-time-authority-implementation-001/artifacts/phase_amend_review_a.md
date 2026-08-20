# Independent restart phase/sequence amendment review A

Date: 2026-08-20
Scope: narrow V2 authority amendment for `ActiveParent` versus
`CommittedParent`, retained parent sequence/identity, successor sequence,
semantic poisons, and crash replay. Production Rust was neither reviewed nor
edited.

## Verdict

**HOLD / amendments required before authority release.**

The central sequence rule is correct: the retained parent transaction ID stays
bound to `parent_transaction_sequence`, an active checkpoint retains that
sequence as its next sequence, and a committed checkpoint retains the checked
successor without deriving a new identity or incrementing again. The schema and
validator represent those fields. The amendment does not yet close the durable
phase boundary or prove crash replay independently.

## Findings

| ID | Severity | Finding | Required disposition |
| --- | --- | --- | --- |
| PH-A-001 | Blocker | `CommittedParent` does not require the durable outbox produced by the atomic parent commit. The semantic validator accepts a committed checkpoint with `publication_outbox=[]`. This contradicts the contract definition that atomic owner installation and durable `CommittedUndelivered` enqueue are one commit. Such a checkpoint cannot distinguish a legitimate zero-record commit from a torn owner/sequence-only write and can lose publication after crash. | Define the committed durable aggregate completely. Require exactly the outbox row joined to the retained committed parent receipt (including the zero-record case), or define an equally authenticated no-publication receipt. Reconstruct its parent/publication identities. Add a committed-with-missing-outbox poison and a crash-after-atomic-write restore/redelivery case. |
| PH-A-002 | High | Phase isolation is incomplete. The validator accepts `CommittedParent` while `pending_publication_buffer` remains nonempty. A committed checkpoint should contain the durable outbox representation, not retain a second staged/precommit publication surface that can be replayed, duplicated, or rolled back inconsistently after restore. | Freeze phase-specific field invariants: active may have a pending buffer and no durable outbox; committed must have no pending buffer and must have its authenticated durable commit/outbox aggregate. Add nonempty-pending-buffer-in-committed and durable-outbox-in-active poisons built so each violates only the intended rule. |
| PH-A-003 | High | The new poisons do not independently establish the phase/sequence rules. `active_phase_with_committed_outbox` changes only the phase on a committed baseline, so it is rejected both because the outbox exists **and** because `next_parent_transaction_sequence` remains the committed successor. There is no poison for an active next sequence that is a successor, a committed next sequence that skips by more than one, retained parent ID incorrectly derived from the next sequence, maximum-sequence active versus committed behavior, missing committed outbox, or committed staged buffer. | Add isolated fixtures for every rule. In particular, construct a valid active baseline (`next == retained`, empty outbox) before adding an outbox; separately poison active `next`, committed `next`, retained ID derivation, overflow, committed outbox absence, and committed pending-buffer presence. |
| PH-A-004 | High | No executable reference vector models crash/restart at the phase boundary. The two added vectors are scheduled-once identity KATs and do not exercise `ActiveParent`, `CommittedParent`, retained identity, increment-once, or no-redelivery-after-acknowledgement. The semantic validator only admits/rejects static documents; it cannot prove that continuation consumes the correct sequence once or avoids replay. | Add an uninterrupted/restored transition vector: crash before commit restores active and commits sequence `n` once; crash after commit restores retained parent ID at `n` with next sequence `n+1` and does not recommit; `CommittedUndelivered` redelivers with the same key; `Acknowledged` does not redeliver; the next parent is derived exactly once from `n+1`. Compare terminal identity, outbox state, and publication order. |
| PH-A-005 | Medium | The contract prose says how the two sequence fields relate, but it does not explicitly state the phase transition is monotone and irreversible for the retained checkpoint, nor that `CommittedParent` restore is publication recovery/next-parent initiation rather than permission to rerun finalization. | State the legal transition `ActiveParent -> CommittedParent` occurs only inside the atomic durable commit, never in reverse; restore of committed state forbids slab/event/scheduled acceptance and parent finalization for the retained parent. Only delivery/acknowledgement and deterministic creation of the next parent are legal. |

## Positive checks

- `restart-schema-v2.json` adds closed required fields for phase, retained
  sequence, and next sequence.
- The validator reconstructs `parent_transaction_id` from the retained sequence,
  not the next sequence.
- Checked successor overflow is rejected for committed state.
- A committed incomplete cursor is rejected.
- The released V1 schema is byte-identical to checkpoint `30e82ab16`; SHA-256 is
  `71c6905d9913ad3a8baccdef3785256c32ea89cff52c757ca157e0438711a05d`.
- The current semantic poison runner passes its declared population and
  `git diff --check` is clean. These checks do not close the missing cases above.

## Reproduction evidence

Direct calls to the independent semantic validator against mutations of its
valid committed baseline produced:

```text
committed_empty_outbox     ACCEPTED
committed_pending_buffer   ACCEPTED
active_end_no_outbox       ACCEPTED
```

The third result is a legitimate precommit-at-parent-end representation when
`next == retained`; it demonstrates why phase is distinct from cursor. The
first two results expose the unclosed committed durable boundary.

## Release criterion

GO requires disposition and correction of PH-A-001 through PH-A-005, rerun of
the schema/semantic/vector gates, an independent verification of the new crash
transition vectors, and continued byte identity of released V1.

---

## Final re-review after corrections

Date: 2026-08-20

### Verdict

**HOLD — durable joins are corrected, but executable crash/sequence proof and
isolated poison coverage remain incomplete.**

### Closed findings

- **PH-A-001 closed.** `CommittedParent` now requires exactly one durable
  outbox row. The validator reconstructs `parent-receipt-v2` from retained
  parent/interval/owner and ordered slab/event/scheduled chronology, then
  reconstructs `publication-receipt-v2` from that parent receipt, ordered
  records, retained sequence, and the immutable `CommittedUndelivered` identity
  state. Missing or foreign rows fail.
- **PH-A-002 closed.** Committed state rejects a nonempty pending buffer; active
  state rejects a durable outbox. The direct loopholes reproduced in the first
  review are no longer admitted.

### Remaining findings

| ID | Severity | Status and evidence | Required correction |
| --- | --- | --- | --- |
| PH-A-003 | High | **Open.** The population is now 60 cases and adds missing-outbox, foreign parent/publication receipt, wrong outbox sequence, and committed pending-buffer poisons. However, `active_phase_with_committed_outbox` remains confounded: changing only phase on the committed baseline leaves both an outbox and `next == retained + 1`, so it does not independently prove either active rule. There is still no isolated active-successor poison, committed skip-successor poison, two-outbox poison, retained-parent-ID-from-next-sequence poison, or active-maximum/committed-overflow pair. | Build a valid active baseline (`phase=ActiveParent`, `next=retained`, empty outbox) and mutate one operand per case. Add each named phase/sequence/cardinality poison independently. |
| PH-A-004 | Blocker | **Open.** `phase_sequence_reference.py` is independent but is not an active-to-commit-to-crash execution. It creates three literal dictionaries, JSON-copies the committed dictionary, and checks integers/cardinality. It constructs no parent ID, parent receipt, publication receipt, durable aggregate, delivery state, acknowledgement, or next-parent ID; it does not call the semantic validator or round-trip the canonical V2 restart document. Consequently the fixture would still pass if commit and restore production/reference algorithms incremented twice, changed receipt identity, lost record order, replayed finalization, or redelivered an acknowledged row. It is a pinned illustration, not the requested crash-replay acceptance fixture. | Implement the fixture as state transitions over a canonical active V2 document: derive retained identity; atomically finalize into the authenticated committed aggregate; serialize/restore through V2 admission; prove finalization cannot replay; redeliver `CommittedUndelivered` with the same key; persist/restore `Acknowledged` and prove no redelivery; derive the next parent once from `n+1`; compare identities, records, and sequence with uninterrupted execution. |
| PH-A-005 | Medium | **Open.** The added prose defines field relationships and committed joins, but still does not explicitly make the retained phase transition monotone/irreversible or enumerate legal post-restore operations. | State that only atomic durable commit performs `ActiveParent -> CommittedParent`; reversal and retained-parent slab/event/scheduled acceptance or refinalization are forbidden. Committed restoration permits only delivery/acknowledgement recovery and deterministic next-parent creation. |

### Re-run evidence

- Semantic validator: **60/60 declared cases produced their expected status**.
- Phase illustration hash: **PASS**, producing
  `2e8995953d142b4c9dc41f0b617615eebe7dd146015b1d9cd13ea586fe042ab8`.
- Released V1 remains protected; no V1 concern was introduced by these
  corrections.

The first two authority defects are materially fixed. Release still requires a
real transition/crash fixture and isolated sequence poisons so the amendment is
not accepted on static self-consistency alone.

---

## Second final re-review

Date: 2026-08-20

### Verdict

**HOLD — poison isolation and lifecycle prose are closed; the executable
transition fixture still does not implement its claimed active-to-commit and
replay checks.**

### Newly closed

- **PH-A-003 closed for the active/outbox ambiguity.** The new
  `active_phase_outbox_isolated` case sets `ActiveParent` and
  `next_parent_transaction_sequence == parent_transaction_sequence` together,
  leaving only the durable outbox as the violation. The semantic population is
  now 61/61 at declared outcomes.
- **PH-A-005 closed.** The contract now freezes the post-commit delivery,
  redelivery, acknowledgement, identity-invariance, no-reincrement, and
  next-parent sequence rules. It also states that a committed checkpoint cannot
  be committed again.

### Remaining blocker

**PH-A-004 remains open.** The replacement fixture is stronger because it
loads the canonical committed V2 baseline and canonicalizes/restores its JSON,
then preserves the receipt tuple while changing delivery state. It still does
not execute the claimed chronology:

- it begins with a literal already-`CommittedParent` baseline; it never creates
  a valid `ActiveParent` document or performs the atomic transition;
- it never reconstructs or validates `parent_receipt_id` or
  `publication_receipt_id`; it merely remembers and compares the supplied
  strings;
- it does not admit the restored/delivered/acknowledged documents through the
  semantic validator;
- it changes `DeliveredUnacknowledged` to `Acknowledged` directly and does not
  attempt and reject redelivery from acknowledged state;
- it copies `next_parent_transaction_sequence` as an integer but constructs no
  next parent transaction identity and therefore cannot prove the successor is
  consumed exactly once.

The KAT
`b45fe1053e07d91a33f70f2f2e105f8a4c03eb631aaa8c9e3ef13b5cb6594e69`
passes, but it pins these assignments rather than an independently executed
authority state machine.

### Required final correction

Build a valid active V2 document from the canonical baseline by removing the
outbox, setting `ActiveParent` and `next == retained`, and placing the intended
records in the pending buffer. Validate it. A separately implemented transition
must reconstruct the parent/publication receipts, atomically produce the
committed document, and validate it. Then round-trip/validate each crash state,
enforce legal delivery transitions, explicitly reject acknowledged redelivery
and recommit, and derive the next parent identity from the persisted successor.
Compare this restored path with an uninterrupted path. Until that executable
proof exists, static contract text and snapshot validation do not establish
crash replay safety.

V1 remains byte-identical (`git diff 30e82ab16 -- restart-schema.json` is empty),
and `git diff --check` passes.

---

## Terminal re-review

Date: 2026-08-20

### Verdict

**PASS / GO for the narrow V2 phase-and-sequence authority amendment.**

PH-A-004 is closed. The independent reference now:

- reconstructs `parent-receipt-v2` and `publication-receipt-v2` from their
  closed framed operands and matches the frozen committed row;
- derives a canonical `ActiveParent` snapshot from the frozen V2 fixture,
  executes the single atomic phase/sequence/outbox transition, and exact-compares
  the result with the frozen `CommittedParent` checkpoint;
- canonical-round-trips the committed crash checkpoint;
- exercises delivery and acknowledgement while proving retained parent,
  publication, and outbox-sequence identity invariance;
- explicitly rejects recommit and acknowledged redelivery;
- consumes persisted successor sequence 42 to derive a distinct next-parent
  transaction identity.

The pinned evidence hash is
`0b5b9be20d22de5139dd5b19d2aeb4430af917640149b7e04baeeef74e479642`.
The semantic poison population remains 61/61 at declared outcomes,
`git diff --check` passes, and released restart V1 remains byte-identical to
checkpoint `30e82ab16`.

All PH-A-001 through PH-A-005 findings are closed. Production implementation
may resume after the second independent review/disposition and exact authority
checkpoint required by the package procedure.
