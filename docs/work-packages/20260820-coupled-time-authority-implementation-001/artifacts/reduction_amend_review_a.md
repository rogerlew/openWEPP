# Independent V2 reduction/outbox-coherence amendment review A

Date: 2026-08-20
Scope: narrow authority review of persisted reduction operands and values,
nullable empty state, maximum/minimum/sum reconstruction, zero-sentinel
exclusion, and outbox state/attempt-count coherence. Production code was not
reviewed or edited.

## Verdict

**HOLD — corrections required before authority release.**

The amendment selects the right wire information and closes the former
ID-only/zero-sentinel ambiguity in principle. The machine schema currently
makes every reduction instance invalid, and the numerical reconstruction rules
and poison population are not yet sufficient for independent cross-language
implementation.

## Findings

| ID | Severity | Finding | Required disposition |
| --- | --- | --- | --- |
| RED-A-001 | **Blocker** | `accepted_operand_values` is listed in the reduction object's `required` array but is defined as a sibling of `properties`, not within `properties`. Because the reduction object also has `additionalProperties: false`, every document containing the required member is invalid under the canonical JSON Schema. The Python semantic validator bypasses JSON Schema and therefore reports accepted controls despite the wire rejecting them. | Move `accepted_operand_values` inside the reduction `properties` map. Run an actual Draft 2020-12 schema validator on the baseline and empty-null control, plus missing/member-shape poisons. Do not treat semantic-validator self-consistency as schema admission. |
| RED-A-002 | **High** | Bit-exact reduction numerics are under-specified. Any 64-bit pattern is admitted as an operand, including NaN and infinities. Python `max`/`min` behavior with NaN and signed-zero ties depends on operand order, while sum overflow/rounding and signed zero require an explicitly frozen sequential binary64 rule for a Rust implementation to reproduce exact bits. The contract only says “recomputes” maximum/minimum/sum. | Freeze admissible operand domain (recommended: finite binary64 only), signed-zero/tie policy for extrema, and exact ordered sum algorithm/rounding and overflow behavior. Reject noncanonical NaNs/nonfinite values as applicable. Add exact-bit vectors for negative-only values, `-0/+0`, equal extrema, rounding-sensitive ordered sum, and overflow/nonfinite poisons. |
| RED-A-003 | **High** | Executable coverage demonstrates only a one-operand maximum and an empty maximum. There is no accepted/rejected multi-operand maximum, minimum, or sum case; no reorder/duplicate/cardinality poison for the paired arrays; and no empty minimum/sum control. Consequently the claim of independent maximum/minimum/sum reconstruction is not release-tested. | Add alias-separating fixtures for all three operators with at least two unequal operands, reordered values, duplicate IDs, missing/extra paired rows, declared-value substitution, and empty-null controls for each operator. Require exact expected bits. |
| RED-A-004 | **High** | Outbox state/count semantics are implemented but only `CommittedUndelivered + nonzero` is poisoned. There is no `DeliveredUnacknowledged + zero` or `Acknowledged + zero` poison, no positive controls for both states, and no transition fixture proving crash preserves count, each delivery/redelivery increments once, and acknowledgement alone does not increment. Static `count >= 1` cannot prove transition deltas. | Add both impossible-pair poisons, positive state/count controls, and an independent transition KAT covering initial delivery, crash, redelivery, acknowledgement, crash-after-ack, stable receipt identity, and exact count deltas. |
| RED-A-005 | **Medium** | The semantic validator checks the operand-ID projection and verifies membership in the accepted set through the separate ID array, which is sound, but schema/contract text does not clearly make `accepted_operand_values` the canonical paired sequence and the ID-only array a redundant checked projection. Maintaining two authoritative arrays risks adopter divergence. | State explicitly that the paired operand sequence is canonical and the ID array is a required redundant projection (or remove the redundant array in this new V2 authority before release). Require identical length/order and reject any mismatch. |

## Positive evidence

- The semantic validator currently produces all 67 declared outcomes: six
  accepted controls and 61 rejected poisons.
- Empty operands reconstruct `null`; an empty zero bit pattern is rejected, so
  zero is no longer used as the empty sentinel by the semantic validator.
- Operand receipt membership is checked against accepted slab/event/scheduled
  receipts, and paired receipt projection must match the ID sequence.
- `CommittedUndelivered` requires count zero; other admitted states require a
  nonzero count in the semantic validator.
- Released restart V1 was not changed by this amendment.

## Release criterion

GO requires correction of the canonical schema, frozen cross-language numeric
semantics, executable maximum/minimum/sum and paired-array poisons, and complete
outbox state/count transition evidence. All gates must be rerun while preserving
released V1 bytes.

---

## Final re-review after corrections

Date: 2026-08-20

### Verdict

**PASS / GO for the narrow V2 reduction/outbox-coherence authority amendment.**

### Finding disposition

- **RED-A-001 closed.** `accepted_operand_values` is now correctly inside the
  reduction object's `properties` map while remaining required under
  `additionalProperties: false`. The JSON parses, the reduction subschema
  admits the canonical baseline under a Draft 2020-12 validator, and the
  semantic population passes.
- **RED-A-002 closed.** The contract freezes finite-only operands/results,
  persisted-order scanning, first-on-numeric-equality extrema behavior
  (including signed zero), and positive-zero ordered left-fold sum with typed
  failure on a nonfinite intermediate/result. The validator implements these
  rules and rejects NaN, infinity, and sum overflow.
- **RED-A-003 closed.** The population now exercises maximum reconstruction,
  multi-operand minimum, multi-operand sum, declared-value substitution,
  operand-value substitution, paired projection mismatch, nonfinite operands,
  overflow, empty-null acceptance, and empty-zero rejection. Canonical
  ordering/uniqueness of the ID projection is enforced by the existing ordered
  uniqueness check; exact paired order and cardinality are enforced by equality
  of the paired-ID projection.
- **RED-A-004 closed.** Impossible count pairs reject for all three outbox
  states, and positive delivered/acknowledged controls admit. Combined with the
  already approved phase lifecycle KAT and new frozen count-delta prose, this
  closes snapshot and transition coherence.
- **RED-A-005 closed.** Canonical prose identifies the ordered paired operands
  as retained reduction authority, while admission requires the redundant ID
  projection to match exactly and to name accepted receipts.

### Re-run evidence

- Semantic validator: **76/76 declared outcomes**, comprising 10 accepted
  controls and 66 rejected poisons; stdout SHA-256
  `22123a7214fce5d70bb4fa951b62fa9832fb38163dc6b445068779a4ed843783`.
- `restart-schema-v2.json`: valid JSON; corrected reduction subschema admits
  the canonical reduction instance under Draft 2020-12 validation.
- `git diff --check`: **PASS**.
- Released restart V1 remains outside this additive V2 amendment.

All RED-A-001 through RED-A-005 findings are closed. The amendment is ready for
the required second independent review, disposition, verification, and exact
authority checkpoint before production implementation resumes.
