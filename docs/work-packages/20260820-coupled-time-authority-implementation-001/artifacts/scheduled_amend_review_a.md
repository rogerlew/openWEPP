# SC-COUPLEDTIME-001 V2 scheduled-receipt amendment review A

Date: 2026-08-20

Scope: the narrow authority amendment that adds the
`scheduled-receipt-v2` framed domain, adds `parent_transaction_id` to the V2
restart wire, reconstructs the receipt in the semantic validator, and adds
scheduled-receipt poisons. Production Rust was not edited.

## Verdict

**HOLD — one authority blocker remains.**

The amendment correctly creates a dedicated framed domain with the ordered
preimage
`parent_transaction_id, operation_id, boundary_id, tick_ns, result_sha256`.
The V2 wire carries the parent field and the semantic validator reconstructs
the digest using that exact domain and field order. No event ordinal is used by
the scheduled identity or its reconstruction. The released
`restart-schema.json` V1 file has no worktree diff, so the amendment is additive
to V2.

The focused semantic population currently reports 50/50 expected outcomes,
and `git diff --check` passes. Those results do not close the blocker below.

## Finding

| ID | Severity | Finding | Required correction |
|---|---|---|---|
| SCHED-A-001 | **BLOCKER** | Scheduled-once replay is still admitted when the second execution changes `result_sha256` (or tick) and recomputes its valid receipt ID. The validator requires only receipt-ID ordering/uniqueness. Because `result_sha256` and `tick_ns` are receipt-ID operands, two receipts for the same `(parent_transaction_id, operation_id, boundary_id)` can have distinct valid IDs and coexist. This violates `ScheduledOnce`/`INV-COUPLEDTIME-011`; receipt identity authenticates an execution result but is not itself the once-only execution key. The new operation/result poisons mutate fields without recomputing the receipt tree, so they prove digest reconstruction but do not expose this replay. | Define the canonical scheduled-execution key independently of result identity—at minimum the parent-bound `(operation_id, boundary_id)` pair, with the contract stating whether tick is derived from/admitted by the boundary. Require uniqueness by that key on restore and at runtime. Add a well-formed poison containing two correctly reconstructed scheduled receipts for the same execution key with different result digest and/or tick; it must fail `ERR-CT-017 ScheduledOnceReplay`. Add an accepted control with two distinct admitted operation/boundary keys. |

## Checks

- Dedicated domain: **PASS**. `model-definition.json` names
  `scheduled-receipt-v2`, separate from event and event-receipt domains.
- Parent binding: **PASS** for receipt authentication. The V2 schema requires
  `parent_transaction_id`, and validation rejects a cross-parent receipt and
  reconstructs the framed digest.
- No event-ordinal borrowing: **PASS**. Neither the domain definition, wire
  shape, contract prose, nor reconstruction uses `event_ordinal`.
- V1 immutability: **PASS**. `git diff --exit-code -- artifacts/restart-schema.json`
  returns success.
- Semantic reconstruction: **PASS**, subject to SCHED-A-001. All individual
  scheduled receipt fields are authenticated.
- Poison strength: **PARTIAL**. Cross-parent and field substitution are covered,
  but no recomputed-tree scheduled replay poison exists.

## Release condition

Close SCHED-A-001, rerun the semantic/schema gates, and re-review the corrected
V2 scheduled-once execution-key semantics before production implementation
resumes.

## Final re-review — 2026-08-20

**PASS — the V2 scheduled-receipt amendment is approved.**

SCHED-A-001 is closed. The authority now freezes a separate
`scheduled-boundary-v2` identity over parent transaction, operation ID, and
exact integer tick, and defines the once-only execution key as
`(parent_transaction_id, operation_id, boundary_id)`. The semantic validator
reconstructs both the boundary identity and the result-bearing
`scheduled-receipt-v2` identity before admission, then independently enforces
execution-key uniqueness.

The corrected population includes the decisive well-formed tests:

- a second receipt with the same execution key, a different result digest, and
  a correctly recomputed receipt ID is rejected;
- a different operation/tick with its own correctly reconstructed boundary and
  receipt identities is accepted.

The semantic run passes all 52 expected cases. The scheduled domains contain
no event ordinal, `restart-schema.json` V1 remains byte-untouched in the
worktree, and `git diff --check` passes. No scheduled-amendment finding remains
open.
