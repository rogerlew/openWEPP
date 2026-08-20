# SC-COUPLEDTIME-001 V2 reduction/outbox amendment review B

Date: 2026-08-20

Scope: independent authority review of the narrow V2 persisted-reduction and
outbox-state/count coherence amendment. This review covers operand identity and
value persistence, nullable empty reductions, independent maximum/minimum/sum
reconstruction, zero-sentinel exclusion, and delivery attempt rules. Production
Rust was not reviewed or edited.

## Verdict

**HOLD — one numerical admission blocker remains.**

The amendment closes the structural restart gap: each reduction now persists
the ordered receipt/value pairs, requires their receipt projection to equal the
accepted operand ID list, authenticates every operand against accepted
chronology, reconstructs the declared operator, and represents an empty fold as
`null` rather than zero. Outbox state/count coherence is also stated and
enforced.

However, any 64-bit pattern is currently admitted as an operand value. The
semantic validator decodes NaN and infinity and passes them into Python
`max`, `min`, or addition. NaN comparison behavior is order-sensitive and not a
portable reduction definition; finite sums may also overflow to infinity.
Production reduction admission already rejects non-finite values, so a restart
wire admitting them would be semantically broader than uninterrupted runtime
chronology.

## Finding

| ID | Severity | Finding | Required disposition |
| --- | --- | --- | --- |
| RED-B-001 | **BLOCKER** | `accepted_operand_values[*].value_bits` and non-null `value_bits` accept every binary64 bit pattern. `validate_restart` decodes without `math.isfinite`, and independently reconstructed sum is not checked for a finite result. Consequently NaN, positive/negative infinity, or finite operands whose sum overflows may be admitted. This conflicts with production `DiagnosticReductionV1::fold_accepted`, which rejects non-finite operands, and prevents deterministic cross-language max/min/sum authority. | Freeze that every persisted reduction operand and every non-null reconstructed result must be finite binary64. Reject NaN, either infinity, and sum overflow typed. Enforce this in canonical V2 semantic requirements and the semantic validator. Add alias-separating poisons for NaN operand, infinity operand, and finite sum overflow; retain a finite signed-zero/order control if signed-zero bits are intended to remain significant. Rerun this review. |

## Checks that pass

- **Complete operand persistence:** PASS. Ordered
  `(accepted_receipt_id, value_bits)` pairs are present, and their ID projection
  must exactly equal `accepted_operand_receipt_ids`.
- **Accepted-only lineage:** PASS. Operand IDs must belong to accepted
  slab/event/scheduled receipt chronology; duplicate operand IDs are excluded by
  the existing ordered-unique rule.
- **Independent reconstruction:** PASS for finite ordinary inputs. Maximum,
  minimum, and ordered sum are independently recomputed from stored operand
  bits rather than trusted accumulator state.
- **Empty semantics:** PASS. Empty operands require `value_bits = null`; a
  binary64 zero bit pattern is rejected as an empty sentinel.
- **Outbox coherence:** PASS. `CommittedUndelivered` requires count zero;
  `DeliveredUnacknowledged` and `Acknowledged` require a positive count. The
  existing phase fixture preserves state/count through crash and increments on
  delivery/redelivery without changing publication identity.
- **Closed V2 shape:** PASS. `accepted_operand_values` is required and its
  element objects reject additional fields.
- **Released V1 protection:** PASS. The amendment remains confined to V2; the
  released `restart-schema.json` is unchanged.

## Evidence

The current declared semantic population passes **67/67** expected outcomes
(6 accepted, 61 rejected), result SHA-256
`af335957c98d22f25240fdce51199c81ba1bd32cfe46614d83a380f14b7e9e9d`
when hashed without the emitted newline. That population does not include the
non-finite cases above, so its self-consistency does not close `RED-B-001`.

## Release condition

Close `RED-B-001` in canonical prose, V2 schema semantic requirements,
executable validation, and decisive non-finite/overflow fixtures. No production
implementation should consume this amendment before independent verification
passes.

## Final re-review after correction

Date: 2026-08-20

**PASS — `RED-B-001` is closed. Review B approves the narrow V2
reduction/outbox-coherence amendment for independent verification.**

The corrected authority now requires every stored operand and reconstructed
non-null result to be finite binary64. Maximum and minimum scan persisted order
and retain the first operand on numeric equality, including signed-zero
equality. Sum is explicitly the persisted-order left fold beginning at positive
zero, with each intermediate checked for finiteness. NaN, either infinity, and
finite-operand overflow therefore fail closed and restart admission no longer
exceeds uninterrupted runtime reduction semantics.

Independent checks performed during this re-review:

- V2 schema parses as JSON and places `accepted_operand_values` inside the
  closed reduction `properties` object.
- Semantic schema population: **76/76 expected outcomes** (10 accepted, 66
  rejected), emitted result SHA-256
  `22123a7214fce5d70bb4fa951b62fa9832fb38163dc6b445068779a4ed843783`.
- Multi-operand minimum and sum controls pass; altered declared value, altered
  operand value, ID/value projection mismatch, NaN, infinity, and sum overflow
  fail.
- Empty reduction with `null` passes and the zero-sentinel alias fails.
- Independent signed-zero probes confirm first-on-equality bits for maximum
  (`-0.0` before `+0.0`) and minimum (`+0.0` before `-0.0`).
- `CommittedUndelivered` with nonzero attempts, and delivered/acknowledged with
  zero attempts, fail; positive delivered and acknowledged controls pass.
- Released restart V1 remains unchanged, and `git diff --check` passes.

No production files were edited by this review. No review finding remains
open.
