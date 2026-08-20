# SC-COUPLEDTIME-001 V2 scheduled-receipt amendment review B

Date: 2026-08-20

Scope: independent authority review of the corrected narrow V2
scheduled-receipt amendment. This review covers the dedicated parent-bound
identity domain, semantic reconstruction, once-only execution-key semantics,
the recomputed-ID duplicate poison, the distinct-boundary accepted control,
and released V1 protection. Production Rust was not reviewed or edited.

## Verdict

**HOLD — one authority-definition blocker remains.**

The executable correction closes the replay hole in the semantic validator:
it rejects a second, individually authentic receipt with the same
`(parent_transaction_id, operation_id, boundary_id)` execution key even when
the result and receipt ID differ. Both new fixtures have independently
recomputed, correct `scheduled-receipt-v2` IDs. The duplicate-key fixture is
rejected and the distinct-boundary control is accepted. The dedicated receipt
domain, parent join, and V1 protection also pass.

However, the canonical contract and V2 schema commentary do not define that
execution key or require uniqueness by it. The validator therefore introduces
an unapproved semantic rule that cannot be derived from the current authority
text. The contract currently defines only the receipt-ID preimage and field
substitution rejection. The schema still describes receipt-ID
ordering/uniqueness and generic "no duplicate" behavior, neither of which
specifies once-only uniqueness independently of result-bearing receipt
identity.

## Finding

| ID | Severity | Finding | Required correction |
|---|---|---|---|
| SCHED-B-001 | **BLOCKER** | The semantic validator defines the scheduled execution key as `(parent_transaction_id, operation_id, boundary_id)`, but neither `SC-COUPLEDTIME-001` nor `restart-schema-v2.json` freezes that rule. Receipt identity includes `tick_ns` and `result_sha256`, so receipt-ID uniqueness alone is not once-only authority. The executable policy is correct but currently stronger than its canonical specification. The authority also does not state whether `tick_ns` is derived from the named boundary or merely authenticated/admitted separately. | Add canonical prose defining the parent-bound scheduled execution key as `(parent_transaction_id, operation_id, boundary_id)`, requiring at most one accepted receipt per key regardless of tick/result/receipt ID, and mapping duplicates to `ERR-CT-017 ScheduledOnceReplay`. State the admission relationship between `boundary_id` and `tick_ns` (for example, the adopter's admitted boundary definition supplies the tick). Mirror this rule in the V2 schema semantic requirements. Then rerun this narrow review; the existing recomputed-ID poison and distinct-boundary control are adequate for this issue. |

## Evidence

- Dedicated identity domain: **PASS**. `model-definition.json` freezes
  `scheduled-receipt-v2` over parent transaction, operation, boundary, tick,
  and result, independently of event receipt domains.
- Parent binding and reconstruction: **PASS**. V2 requires the parent field,
  joins it to the enclosing parent, and reconstructs the complete framed ID.
- Recomputed-ID duplicate poison: **PASS**. Its frozen ID
  `f23b8e52...7f077` independently recomputes exactly and it is rejected by
  execution-key uniqueness.
- Distinct-boundary control: **PASS**. Its frozen ID
  `ec8dd727...1df8` independently recomputes exactly and the validator accepts
  it.
- Semantic population: **PASS**, 52/52 declared outcomes.
- JSON parsing and diff hygiene: **PASS**.
- Released V1 protection: **PASS**. `restart-schema.json` has no diff from the
  released authority checkpoint `30e82ab16`.

## Release condition

Freeze the already implemented execution-key and boundary/tick rule in the
canonical contract and V2 schema semantic requirements. No fixture or
algorithmic redesign is otherwise required by this review.

## Final re-review — 2026-08-20

**PASS — SCHED-B-001 is closed and review B approves the narrow V2
scheduled-receipt amendment for implementation.**

The canonical contract now freezes `scheduled-boundary-v2` over
`parent_transaction_id`, `operation_id`, and exact integer `tick_ns`. It also
defines the scheduled execution key as
`(parent_transaction_id, operation_id, boundary_id)`, permits exactly one
accepted receipt for that key independently of result and receipt identity,
and classifies a second correctly framed receipt as typed replay. The V2 schema
semantic requirements mirror boundary reconstruction and execution-key
uniqueness.

The semantic validator independently reconstructs the boundary before the
receipt, joins the receipt parent to the enclosing transaction, enforces the
canonical execution key, and reconstructs the complete
`scheduled-receipt-v2` identity. Independent digest reconstruction confirmed:

- baseline `daily` at tick 5: boundary and receipt IDs exact;
- replay fixture `daily` at tick 5 with a different result: boundary and
  receipt IDs exact, rejected because the execution key is already accepted;
- control `hourly` at tick 4: distinct exact boundary and receipt IDs,
  accepted.

Focused evidence passes: semantic population **52/52**, JSON inputs parse,
`git diff --check` is clean, and released `restart-schema.json` V1 remains
byte-identical to checkpoint `30e82ab16`. No open review-B finding remains.
