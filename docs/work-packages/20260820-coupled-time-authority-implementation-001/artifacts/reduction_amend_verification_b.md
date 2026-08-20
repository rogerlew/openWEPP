# SC-COUPLEDTIME-001 V2 reduction amendment verification B

Date: 2026-08-20

Scope: final independent verification of the narrow V2 persisted-reduction and
outbox-state/count amendment. Production Rust was not edited.

## Verdict

**FAIL — the reduction gates pass, but an invalidated outbox lifecycle KAT
prevents verified closure of `RED-A-004`.**

## Independent results

| Gate | Result |
| --- | --- |
| V2 semantic/canonical population | PASS, 76/76 expected outcomes; 10 accepted and 66 rejected; emitted result SHA-256 `22123a7214fce5d70bb4fa951b62fa9832fb38163dc6b445068779a4ed843783` |
| Complete coupled-time reference oracle | PASS, 114/114 expected outcomes; 62 accepted and 52 rejected; emitted result SHA-256 `192a03003475f4b8e10326a91b837236f9b4e8cafd6b5335b2189b4f8ff69a7c`, exactly matching `expected_reference_results_sha256` |
| V2 schema JSON parse | PASS |
| Draft 2020-12 schema structural check | PASS via `Draft202012Validator.check_schema` |
| Released restart V1 protection | PASS; worktree and `HEAD` SHA-256 both `71c6905d9913ad3a8baccdef3785256c32ea89cff52c757ca157e0438711a05d` |
| Review B closure | PASS; `RED-B-001` has a final PASS disposition |
| Review A closure | **FAIL evidence dependency**; `RED-A-004` is marked closed partly by the independent lifecycle KAT, but that KAT no longer passes against the amended baseline |
| Patch hygiene | PASS; `git diff --check` returned success |

## Blocking verification finding

| ID | Severity | Finding | Required correction |
| --- | --- | --- | --- |
| RED-VERIFY-B-001 | **BLOCKER** | `phase_sequence_reference.py` consumes the shared semantic baseline. Adding canonical reduction state changed the fixture's evidence object and therefore its digest. The script now exits with `phase fixture mismatch: bedc538b51f9a766e81fc7fc6235ab784b13e5d68c256156ac269a1ab13cb85f` instead of its frozen expected digest. Review A explicitly relies on this KAT to close exact delivery/redelivery count deltas in `RED-A-004`; static state/count poisons alone do not prove those transitions. | Independently inspect the changed evidence, confirm the lifecycle semantics and receipt identities remain correct, then deliberately repin the KAT expected digest (or isolate the KAT from unrelated reduction baseline fields). Rerun the KAT and both final verification gates. Do not merely suppress the hash check. |

## Commands executed

```text
python3 artifacts/semantic_schema_validator.py \
  --poisons artifacts/semantic-schema-poisons.json
python3 artifacts/reference_model.py artifacts/coupled-time-vectors.json
python3 artifacts/phase_sequence_reference.py
python3 -c '... Draft202012Validator.check_schema(...) ...'
sha256sum artifacts/restart-schema.json
git show HEAD:artifacts/restart-schema.json | sha256sum
git diff --exit-code -- artifacts/restart-schema.json
git diff --check
```

The abbreviated `artifacts/...` paths are relative to
`docs/work-packages/20260820-coupled-time-authority-implementation-001`.

The amendment must not be released until `RED-VERIFY-B-001` is closed and this
verification is rerun.

## Final verification after deliberate KAT repin

Date: 2026-08-20

**PASS — `RED-VERIFY-B-001` is closed. The final reduction amendment is
verified for release to production implementation.**

The KAT change is limited to its frozen expected digest. Its transition logic
is unchanged. The digest changed because the canonical committed baseline now
contains the newly authenticated reduction operand/value state, which is
intentionally included in the KAT evidence snapshots.

Independent rerun results:

- Phase/outbox lifecycle KAT: **PASS**, deliberately repinned digest
  `bedc538b51f9a766e81fc7fc6235ab784b13e5d68c256156ac269a1ab13cb85f`.
- Semantic schema population: **76/76 PASS** (10 accepted, 66 rejected),
  emitted SHA-256
  `22123a7214fce5d70bb4fa951b62fa9832fb38163dc6b445068779a4ed843783`.
- Complete oracle: **114/114 PASS** (62 accepted, 52 rejected), emitted
  SHA-256
  `192a03003475f4b8e10326a91b837236f9b4e8cafd6b5335b2189b4f8ff69a7c`,
  matching the vector manifest.
- V2 JSON parse and Draft 2020-12 structural schema check: **PASS**.
- Released restart V1 protection: **PASS**, worktree and `HEAD` SHA-256
  `71c6905d9913ad3a8baccdef3785256c32ea89cff52c757ca157e0438711a05d`.
- Patch hygiene: **PASS**.

All `RED-A-001` through `RED-A-005`, `RED-B-001`, and
`RED-VERIFY-B-001` findings are closed. No verification finding remains open.
