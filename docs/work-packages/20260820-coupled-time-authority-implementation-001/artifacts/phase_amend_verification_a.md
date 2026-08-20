# Independent V2 phase/sequence amendment verification A

Date: 2026-08-20
Scope: final verification of the narrow `ActiveParent` / `CommittedParent`
restart phase and parent-sequence amendment. Production Rust is outside this
verification.

## Verdict

**PASS / GO.**

The canonical authority, additive V2 wire, independent semantic validator,
poison population, and independently framed transition/crash KAT agree. All
PH-A review findings are closed and the released V1 wire remains byte-identical.

## Result-bearing gates

| Gate | Result |
| --- | --- |
| V2 semantic validator and poison population | **PASS — 61/61 declared outcomes**: 5 accepted controls and 56 rejected poisons; stdout SHA-256 `e164ebf018a4c1103694b1998ce3a68ada79b48a41ca8af270a8e8dc01d3655e` |
| Complete independent coupled-time oracle | **PASS — 111/111 exact expected results**: 59 accepted and 52 rejected; stdout SHA-256 `60825170c2457403c040dd60b8acc3aa9048aa3251e71d3a39fae039ebf365ec`, exactly matching `expected_reference_results_sha256` |
| Independent phase/sequence state-machine KAT | **PASS** — `0b5b9be20d22de5139dd5b19d2aeb4430af917640149b7e04baeeef74e479642` |
| Released coupled-time restart V1 versus checkpoint `30e82ab16` | **PASS — byte-identical**; SHA-256 `71c6905d9913ad3a8baccdef3785256c32ea89cff52c757ca157e0438711a05d` |
| Diff hygiene | **PASS** — `git diff --check` |

Commands rerun from `/workdir/openWEPP`:

```text
pkg=docs/work-packages/20260820-coupled-time-authority-implementation-001
python3 "$pkg/artifacts/reference_model.py" "$pkg/artifacts/coupled-time-vectors.json"
python3 "$pkg/artifacts/semantic_schema_validator.py" \
  --poisons "$pkg/artifacts/semantic-schema-poisons.json"
python3 "$pkg/artifacts/phase_sequence_reference.py"
git diff --exit-code 30e82ab16 -- "$pkg/artifacts/restart-schema.json"
git diff --check
```

## Finding disposition verification

| Finding | Final disposition |
| --- | --- |
| PH-A-001 — committed checkpoint admitted without its durable outbox | **Closed.** `CommittedParent` requires exactly one row. The validator independently reconstructs and joins the parent receipt, publication receipt, and retained sequence. Missing/foreign/wrong-sequence rows reject. |
| PH-A-002 — committed checkpoint retained staged publication | **Closed.** Committed state requires an empty pending buffer; active state forbids a durable outbox. Both directions have executable poisons. |
| PH-A-003 — confounded phase/sequence poisons | **Closed.** `active_phase_outbox_isolated` changes phase and next sequence together, isolating the retained durable outbox as the sole violation. The committed successor, incomplete cursor, missing outbox, receipt substitution, sequence substitution, and pending-buffer cases also reject. |
| PH-A-004 — no executed active/commit/crash/replay chronology | **Closed.** `phase_sequence_reference.py` independently reconstructs both framed durable receipt IDs, derives canonical active state, executes and exact-compares the atomic committed state, canonical-round-trips crash restore, preserves identity through delivery/acknowledgement, rejects recommit and acknowledged redelivery, and derives a distinct next-parent identity from sequence 42. |
| PH-A-005 — phase monotonicity and legal post-commit operations unstated | **Closed.** Canonical prose freezes the monotone delivery lifecycle, crash preservation, redelivery boundary, acknowledgement terminality, identity invariance, no reincrement/recommit, and exact persisted-successor consumption. |

The final review artifact explicitly records PH-A-001 through PH-A-005 as
closed. No finding is deferred to production implementation.

## Authority conclusion

The retained parent remains identified by its consumed sequence while the next
sequence advances exactly once inside the atomic durable commit. Crash restore
cannot reinterpret the retained parent under the successor, repeat
finalization, lose the committed publication row, or redeliver an acknowledged
row. The additive V2 amendment is ready for its exact authority checkpoint and
subsequent production implementation.
