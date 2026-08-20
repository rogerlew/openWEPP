# SC-COUPLEDTIME-001 V2 scheduled amendment verification B

Date: 2026-08-20

Scope: final independent verification of the narrow V2 scheduled-once
authority amendment. Production Rust was not edited.

## Verdict

**PASS.**

Both independent review findings, `SCHED-A-001` and `SCHED-B-001`, are
explicitly closed by the final re-review sections in their respective review
artifacts. The approved authority, machine definition, V2 schema semantics,
validator, and executable fixtures agree on the parent-bound scheduled
boundary and once-only execution key. No open verification finding remains.

## Independent gate results

| Gate | Result |
|---|---|
| V2 semantic/canonical population | **PASS**, 52/52 expected outcomes; raw result SHA-256 `18ac6a50a08a86598344ad28ce5d77d6c8dfcc5e093d12dcfe9ca1355b6300db` |
| Complete coupled-time reference oracle | **PASS**, 109/109 expected outcomes; 57 accepted and 52 rejected; raw result SHA-256 `8a4e0728cd37a65873a44299df71bf6add7ef5b41d8ca454eae464bc9f6133e1`, exactly matching the vector manifest |
| Released restart V1 protection | **PASS**; worktree and checkpoint `30e82ab16` SHA-256 are both `71c6905d9913ad3a8baccdef3785256c32ea89cff52c757ca157e0438711a05d` |
| Patch hygiene | **PASS**; `git diff --check` returned success |
| Review disposition | **PASS**; final review A closes `SCHED-A-001`, and final review B closes `SCHED-B-001` |

Commands were run from `/workdir/openWEPP`:

```text
python3 docs/work-packages/20260820-coupled-time-authority-implementation-001/artifacts/reference_model.py \
  docs/work-packages/20260820-coupled-time-authority-implementation-001/artifacts/coupled-time-vectors.json
python3 docs/work-packages/20260820-coupled-time-authority-implementation-001/artifacts/semantic_schema_validator.py \
  --poisons docs/work-packages/20260820-coupled-time-authority-implementation-001/artifacts/semantic-schema-poisons.json
sha256sum docs/work-packages/20260820-coupled-time-authority-implementation-001/artifacts/restart-schema.json
git show 30e82ab16:docs/work-packages/20260820-coupled-time-authority-implementation-001/artifacts/restart-schema.json | sha256sum
git diff --exit-code 30e82ab16 -- \
  docs/work-packages/20260820-coupled-time-authority-implementation-001/artifacts/restart-schema.json
git diff --check
```

## Authority verification

- `scheduled-boundary-v2` is a closed framed identity over parent transaction,
  operation ID, and exact integer tick.
- `scheduled-receipt-v2` is a separate result-bearing identity over parent,
  operation, reconstructed boundary, tick, and result digest.
- The semantic validator reconstructs the boundary first, reconstructs the
  receipt second, and joins the receipt parent to the enclosing transaction.
- The once-only key is
  `(parent_transaction_id, operation_id, boundary_id)`, independent of result
  and receipt identity.
- The replay poison contains a second correctly reconstructed receipt for the
  same execution key with a different result and is rejected.
- The distinct operation/tick control has independently valid boundary and
  receipt IDs and is accepted.
- Scheduled receipt identity does not borrow event ordinal or event receipt
  namespace.
- The amendment is additive to V2 and leaves the released restart V1 wire
  byte-identical.

The final scheduled amendment is verified for promotion to the exact authority
checkpoint and subsequent production implementation.
