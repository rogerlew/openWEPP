# SC-COUPLEDTIME-001 V2 phase/sequence amendment verification B

Date: 2026-08-20

Scope: final independent verification of the narrow V2 restart
checkpoint-phase and parent-sequence amendment. Production Rust was not edited.

## Verdict

**PASS.**

All findings from independent reviews A and B are explicitly closed. The
amendment is ready for production implementation.

## Independent results

| Gate | Result |
| --- | --- |
| V2 semantic/canonical population | PASS, 61/61 expected outcomes; 5 accepted and 56 rejected; emitted result SHA-256 `e164ebf018a4c1103694b1998ce3a68ada79b48a41ca8af270a8e8dc01d3655e` |
| Complete coupled-time reference oracle | PASS, 111/111 expected outcomes; 59 accepted and 52 rejected; emitted result SHA-256 `60825170c2457403c040dd60b8acc3aa9048aa3251e71d3a39fae039ebf365ec`, exactly matching `expected_reference_results_sha256` |
| Independent phase transition KAT | PASS; active -> atomic commit -> committed crash restore -> delivery -> acknowledgement -> next-parent derivation digest `0b5b9be20d22de5139dd5b19d2aeb4430af917640149b7e04baeeef74e479642` |
| Released restart V1 protection | PASS; worktree and `HEAD` SHA-256 both `71c6905d9913ad3a8baccdef3785256c32ea89cff52c757ca157e0438711a05d` |
| Review finding closure | PASS; `PH-A-001` through `PH-A-005` and `PHASE-B-001` have final PASS dispositions |
| Patch hygiene | PASS; `git diff --check` returned success |

Commands executed from `/workdir/openWEPP`:

```text
python3 artifacts/semantic_schema_validator.py \
  --poisons artifacts/semantic-schema-poisons.json
python3 artifacts/reference_model.py artifacts/coupled-time-vectors.json
python3 artifacts/phase_sequence_reference.py
sha256sum artifacts/restart-schema.json
git show HEAD:artifacts/restart-schema.json | sha256sum
git diff --exit-code -- artifacts/restart-schema.json
git diff --check
```

The abbreviated `artifacts/...` paths are relative to
`docs/work-packages/20260820-coupled-time-authority-implementation-001`.

## Authority verification

- The retained parent ID reconstructs only from
  `parent_transaction_sequence`; the continuation sequence is not permitted to
  rewrite committed identity.
- Active state retains equal current/next sequence and has no durable outbox.
- Atomic commit changes phase once, consumes the checked successor once,
  removes the staged publication buffer, and installs exactly one authenticated
  durable outbox row.
- Parent-receipt V2 authenticates ordered slab, event, and scheduled-once
  chronology. Publication-receipt V2 authenticates that parent receipt,
  ordered record IDs, retained outbox sequence, and immutable
  `CommittedUndelivered` identity state.
- Crash restoration retains both receipt identities and the already-consumed
  successor. Delivery state and attempt count may advance without changing the
  idempotency identity; acknowledgement cannot redeliver or consume another
  sequence.
- The isolated poisons reject torn or aliased phase shapes, including missing
  committed outbox, committed staged buffer, active durable outbox, wrong
  successor, incomplete cursor, foreign parent/publication receipts, and wrong
  outbox sequence.
- The released DirectV10 restart V1 wire remains byte-identical; all additions
  are confined to the separately versioned coupled-time V2 authority.

No verification finding remains open.
