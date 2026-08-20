# SC-COUPLEDTIME-001 V2 phase/sequence amendment review B

Date: 2026-08-20

Scope: independent authority review of the narrow V2 restart checkpoint-phase
and parent-sequence amendment. This review covers transaction identity, checked
increment, committed-outbox crash restoration, canonical schema admission, and
released restart V1 protection. Production Rust is outside this review.

## Verdict

**HOLD — one blocking authority/admission gap remains.**

The parent identity and sequence split is sound: the retained
`parent_transaction_id` continues to reconstruct from
`parent_transaction_sequence`; `ActiveParent` retains the same next sequence;
and `CommittedParent` requires a checked successor. This prevents restore from
deriving the committed parent under the successor sequence or consuming the
increment twice.

However, the committed phase does not require the durable publication row whose
creation defines the atomic parent commit. The schema permits an empty
`publication_outbox`, and `validate_restart` accepts that shape for
`CommittedParent` as long as the cursor and successor sequence are correct.
Such a checkpoint has consumed the persistent sequence but has no
`CommittedUndelivered` row to retry after a crash. That contradicts the
contract's requirement that owner installation, sequence increment, and durable
enqueue occur in one transaction.

## Finding

| ID | Severity | Finding | Required disposition |
| --- | --- | --- | --- |
| PHASE-B-001 | **BLOCKER** | `CommittedParent` admission checks only `cursor == parent_end` and `next_sequence == checked(parent_sequence + 1)`. It does not require exactly one durable outbox row for the retained committed parent. Conversely, the new schema commentary says only that committed phase requires a complete cursor and successor. Therefore deleting `/publication_outbox/0` from the committed baseline remains admissible, consuming the transaction sequence while losing the crash-retry publication row. The validator also does not reconstruct the row's `parent_receipt_id` or `publication_receipt_id`, so an unrelated well-formed row can satisfy presence if cardinality alone is added. | Freeze and enforce the phase invariant: `ActiveParent` has no durable outbox for this parent; `CommittedParent` has exactly one outbox row derived from this parent finalization, with the canonical parent-receipt identity reconstructed from the retained parent/interval/owner/slab/event chronology and the canonical publication-receipt identity reconstructed from that parent receipt, ordered records, and admitted initial outbox state/sequence. At minimum the phase amendment must specify exact committed cardinality and authenticated parent/publication joins. Add poisons for committed-without-outbox, committed-with-two-outboxes, foreign parent receipt, foreign publication receipt, and wrong outbox sequence. Rerun the semantic gate and this review. |

## Checks that pass

- **Retained identity:** PASS. `parent_transaction_sequence` remains the input
  to `parent-transaction`; the successor is a separate continuation field.
- **Checked increment:** PASS in the stated phase rule. A committed checkpoint
  at `u128::MAX` is rejected rather than wrapping.
- **No double increment:** PASS at the authority level. Restore is explicitly
  forbidden from deriving the retained parent ID from, or incrementing again
  from, `next_parent_transaction_sequence`.
- **Active-phase boundary:** PASS. `ActiveParent` requires equal current/next
  sequence and forbids a durable committed outbox.
- **Canonical closed shape:** PASS. Both new fields are required, additional
  properties remain forbidden, and the semantic validator parses both as
  canonical bounded `u128` strings.
- **Released restart V1:** PASS. `artifacts/restart-schema.json` is unchanged
  from `HEAD`; both copies hash to
  `71c6905d9913ad3a8baccdef3785256c32ea89cff52c757ca157e0438711a05d`.

## Release condition

Close `PHASE-B-001` in canonical contract prose, V2 schema semantic
requirements, executable validation, and alias-separating poisons. No
production implementation should consume this amendment before independent
verification passes.

## Final re-review after correction

Date: 2026-08-20

**PASS — `PHASE-B-001` is closed. Review B approves the narrow V2
phase/sequence amendment for independent verification.**

The corrected authority now makes the atomic boundary explicit:

- `ActiveParent` retains `next_parent_transaction_sequence ==
  parent_transaction_sequence`, has no durable outbox row, and may retain a
  pending publication buffer.
- `CommittedParent` is admitted only at the complete parent cursor, retains the
  sequence that derived its immutable parent identity, records the checked
  successor separately, contains exactly one durable outbox row, and contains
  no pending publication buffer.
- The committed row's `parent_receipt_id` is reconstructed under
  `parent-receipt-v2` from the retained parent/interval/owner identity and the
  complete ordered slab, event, and scheduled receipt chronology.
- Its `publication_receipt_id` is reconstructed under
  `publication-receipt-v2` from that parent receipt, ordered record IDs,
  committed-parent sequence, and the fixed `CommittedUndelivered` identity
  state. Later delivery-state and attempt-count changes cannot alter the
  publication identity.

Executable evidence independently rerun during this re-review:

- Semantic schema/poison gate: **60/60 expected outcomes**, including rejection
  of missing committed outbox, foreign parent receipt, foreign publication
  receipt, wrong outbox sequence, committed pending buffer, incomplete cursor,
  and active phase with committed outbox.
- Independent active -> commit -> crash -> restore fixture: **PASS**, digest
  `2e8995953d142b4c9dc41f0b617615eebe7dd146015b1d9cd13ea586fe042ab8`.
- Released restart V1 protection remains **PASS**, SHA-256
  `71c6905d9913ad3a8baccdef3785256c32ea89cff52c757ca157e0438711a05d`
  in both worktree and `HEAD`.

No production files were edited by this review.
