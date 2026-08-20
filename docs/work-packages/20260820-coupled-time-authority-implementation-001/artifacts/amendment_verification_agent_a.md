# Coupled-time restart amendment verification A

Verifier role: independent time/numerics/chronology authority verification
Authority: `SC-COUPLEDTIME-001` and `OPENWEPP_COUPLED_TIME_RESTART_V2`
Date: 2026-08-20
Production Rust: not edited or approved by this verification

## Verdict

**PASS.** The amended authority, V2 wire, semantic admission population,
independent chronology/finalization references, dual-review record, and V1
protection are mutually consistent at the exact candidate identified below.
All amendment-review findings have explicit accepted dispositions and passing
correction evidence. The amended authority may receive its exact release
checkpoint; production implementation remains a later review boundary.

## Exact candidate identity

| Identity | Value |
|---|---|
| Current commit | `42f88d644cf4f3c86bae0b9ae066505684699704` |
| Current commit tree | `741778f5c08e245ff7325be9885b1025a6a4f142` |
| Binary worktree diff SHA-256 (`git diff --binary HEAD`) | `087c213014a1c3c0aa3a8b3000a9c5f30404822402765f884bd3540758e470f8` |
| Authority-surface manifest SHA-256 | `0eb53dab56bd531c6dc4744ae0a8948d7db0583ae91dca71f68c4695fc5ce8d4` |

The authority-surface manifest is the SHA-256 of 50 sorted lines of
`git-hash-object path`, covering the canonical coupled-time contract, science-
contract index, and every regular file directly under this package's
`artifacts/` directory. It includes tracked modifications and untracked
amendment artifacts. The hashes identify the candidate immediately before this
verification report was added; the report cannot include itself without a
self-referential hash.

## Executed gates

| Gate | Result |
|---|---|
| Complete coupled-time reference oracle | **PASS — 108/108** |
| V2 semantic and canonical poison population | **PASS — 47/47** |
| Independent A+B slab -> B-to-C event -> A+C slab chronology | **PASS —** `6b131695fda7f600344dc7c706f63e8c1cf86ef41ab72afd5583b8b76ff25971` |
| Independent restored parent finalization KAT | **PASS —** parent `90627286f5cc4b6e341f0162323606013f0c0d8f58b2dd17615459befd6cfda3` |
| Independent restored publication KAT | **PASS —** publication `5faa32af248f6d4badbb0d6b65cf075d18b25f3eaedd23a2d49e53f6ff574602` |
| Released coupled-time restart V1 versus checkpoint `30e82ab16` | **PASS — byte-identical** |
| Released DirectV10 checkpoint vector/schema/manifest versus `30e82ab16` | **PASS — byte-identical** |
| Strict Binding Exposure Index | **PASS — 3/3 rows consolidated** |
| Science-contract unit compliance | **PASS — no findings** |
| V2 schema and poison JSON parse | **PASS** |
| `git diff --check` | **PASS** |

Principal commands:

```text
python3 artifacts/reference_model.py artifacts/coupled-time-vectors.json
python3 artifacts/semantic_schema_validator.py \
  --poisons artifacts/semantic-schema-poisons.json
python3 artifacts/merged_chronology_reference.py
python3 artifacts/restart_finalization_reference.py
cmp artifacts/restart-schema.json \
  <(git show 30e82ab16:artifacts/restart-schema.json)
python3 tools/check_sc_binding_exposure.py --strict \
  docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md
python3 tools/release/check_sc_unit_compliance.py --path \
  docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md
git diff --check
```

## Finding audit

| Finding family | Verification |
|---|---|
| V1 was mutated rather than extended | **CLOSED.** V1 schema and the three protected DirectV10 artifacts are byte-identical to the released checkpoint. V2 is additive and separately named. |
| Wrong or unchecked duration bits | **CLOSED.** Support-derived bits are reconstructed exactly; well-formed wrong bits fail; large and rounding-sensitive conversions remain in the complete oracle. |
| Slab/event/receipt IDs were syntax-only | **CLOSED.** Parent interval/transaction, slab, event, and V2 receipt identities are reconstructed from frozen framed operands. The legacy domain remains intact. |
| First owner/clock and parent roots were unauthenticated | **CLOSED.** Explicit begin digests seed the merged chain, and parent interval/transaction IDs are reconstructed from run/calendar/forcing/support/sequence/begin-owner roots. |
| Slabs were processed before all events | **CLOSED.** Admission merges actions by tick, applies slab-end before a same-tick event, orders coincident events deterministically, and chains owner/clock digests through the interior transition. |
| Cursor, last step, and next ordinals were unjoined | **CLOSED.** Cursor coverage, final step, next slab/event/segment ordinals, active segment identity, and terminal owner set are validated. |
| No positive interior event/restored finalization evidence | **CLOSED.** Separately authored fixtures freeze the A+B -> B-to-C -> A+C chronology and restored parent/publication identities. |
| Well-formed semantic aliases were absent | **CLOSED.** The 47-case population includes well-formed parent, transaction, beginning-clock, active-segment, ordinal, duration, owner, and event substitutions in addition to malformed/omission cases. |
| Review findings were not fully dispositioned | **CLOSED.** `amendment_disposition.md` explicitly maps the initial A/B findings and final `RB1`–`RB3`, `V2-A-001`, and `V2-A-002` to accepted corrections; both final reviews report PASS and no finding is waived. |

## Scope note

`check_science_contract_admission.sh --base-ref 30e82ab16 --worktree` also was
run. It stops on the concurrently modified production orchestrator consumer
because that implementation file does not yet carry a current SC binding. That
file is outside this contract-only amendment candidate and is already within
the paused production implementation/re-review scope. It does not invalidate
the amendment authority PASS, but it must be resolved before terminal package
admission or production approval.

## Verification conclusion

No unresolved authority contradiction, review finding, V1 compatibility
regression, or executable amendment-gate failure remains. Verification A is
**PASS** at the candidate hashes above.
