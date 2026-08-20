# SC-COUPLEDTIME-001 V2 scheduled amendment verification A

Date: 2026-08-20

Scope: final independent verification of the narrow scheduled-once V2
authority amendment. Production Rust was not edited.

## Verdict

**PASS.**

The final amendment closes the only review finding, `SCHED-A-001`, and is ready
for the production implementation phase. The final section of
`scheduled_amend_review_a.md` explicitly dispositions that finding as closed.

## Independent results

| Gate | Result |
|---|---|
| V2 semantic/canonical population | PASS, 52/52 expected outcomes; 5 accepted and 47 rejected; result SHA-256 `18ac6a50a08a86598344ad28ce5d77d6c8dfcc5e093d12dcfe9ca1355b6300db` |
| Complete coupled-time reference oracle | PASS, 109/109 expected outcomes; 57 accepted and 52 rejected; result SHA-256 `8a4e0728cd37a65873a44299df71bf6add7ef5b41d8ca454eae464bc9f6133e1` |
| Released restart V1 protection | PASS; worktree and `HEAD` SHA-256 both `71c6905d9913ad3a8baccdef3785256c32ea89cff52c757ca157e0438711a05d` |
| Scheduled/event namespace separation | PASS; no scheduled domain, wire, or reconstruction surface uses `event_ordinal` |
| Patch hygiene | PASS; `git diff --check` returned success |

Commands executed from `/workdir/openWEPP`:

```text
python3 artifacts/reference_model.py artifacts/coupled-time-vectors.json
python3 artifacts/semantic_schema_validator.py \
  --poisons artifacts/semantic-schema-poisons.json
git diff --exit-code -- artifacts/restart-schema.json
git diff --check
```

The abbreviated `artifacts/...` paths above are relative to
`docs/work-packages/20260820-coupled-time-authority-implementation-001`.

## Authority verification

- `scheduled-boundary-v2` is a dedicated closed framed domain over parent
  transaction, operation ID, and exact integer tick.
- `scheduled-receipt-v2` is a separate result-bearing framed domain over parent,
  operation, boundary, tick, and result digest.
- Restore reconstructs both identities and checks the receipt parent against the
  enclosing parent transaction.
- Once-only custody uses the separate execution key
  `(parent_transaction_id, operation_id, boundary_id)`, not receipt-ID
  uniqueness and not event chronology.
- The decisive replay poison supplies a second correctly reconstructed receipt
  with the same execution key and a different result, and is rejected.
- The distinct operation/tick boundary control supplies independently valid
  boundary and receipt identities and is accepted.
- The amendment is additive to V2; the released V1 schema remains unchanged.

No verification finding remains open.
