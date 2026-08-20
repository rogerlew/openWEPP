# Independent V2 reduction/outbox amendment verification A

Date: 2026-08-20
Scope: final verification of the additive V2 reduction operand/value and outbox
state/count authority amendment. Production Rust is outside this verification.

## Verdict

**PASS / GO.**

The canonical V2 schema, semantic admission rules, independent oracle, review
dispositions, and released-wire protection agree. No review-A or review-B
finding remains open.

## Result-bearing gates

| Gate | Result |
| --- | --- |
| Semantic validator and poison population | **PASS — 76/76**: 10 accepted controls and 66 rejected poisons; stdout SHA-256 `22123a7214fce5d70bb4fa951b62fa9832fb38163dc6b445068779a4ed843783` |
| Complete independent coupled-time oracle | **PASS — 114/114**: 62 accepted and 52 rejected; stdout SHA-256 `192a03003475f4b8e10326a91b837236f9b4e8cafd6b5335b2189b4f8ff69a7c`, exactly matching the vector manifest |
| V2 JSON schema parse | **PASS** — `jq empty` |
| Reduction Draft 2020-12 subschema against canonical baseline | **PASS — zero validation errors** |
| Released restart V1 versus authority checkpoint `30e82ab16` | **PASS — byte-identical**; SHA-256 `71c6905d9913ad3a8baccdef3785256c32ea89cff52c757ca157e0438711a05d` |
| Diff hygiene | **PASS** — `git diff --check` |

Commands were rerun from `/workdir/openWEPP` using the package path
`docs/work-packages/20260820-coupled-time-authority-implementation-001`.

## Finding closure

| Finding | Verification disposition |
| --- | --- |
| RED-A-001 — malformed closed reduction schema | **Closed.** `accepted_operand_values` is a required member inside `properties`; the canonical instance validates under the reduction subschema. |
| RED-A-002 / RED-B-001 — nonfinite and ambiguous bit-exact numerics | **Closed.** Finite-only operands/results, persisted-order extrema with first-on-equality, and positive-zero ordered left-fold sum are authoritative and enforced. NaN, infinity, and overflow reject. |
| RED-A-003 — insufficient operator/projection/empty coverage | **Closed.** Maximum, multi-operand minimum and sum, value substitution, projection mismatch, nonfinite/overflow, nullable empty, and zero alias are executable. Exact paired projection plus ordered uniqueness closes reorder/duplicate/cardinality substitution. |
| RED-A-004 — outbox state/count coherence unproven | **Closed.** All impossible state/count classes reject and positive delivered/acknowledged controls admit; the previously approved lifecycle KAT supplies transition/crash evidence. |
| RED-A-005 — competing ID and paired-value authority | **Closed.** Ordered pairs are retained authority; the ID list is a mandatory exact redundant projection joined to accepted receipts. |

Review A records RED-A-001 through RED-A-005 closed with **PASS / GO**. Review
B records RED-B-001 closed with **PASS** and states no finding remains open.

## Authority conclusion

V2 restart now preserves enough accepted operand information to reconstruct
diagnostic reductions independently without inventing a zero for empty state.
Outbox snapshots exclude impossible delivery-state/count pairs. The amendment
is ready for its second independent verification, exact disposition, and
authority checkpoint before production implementation resumes.
