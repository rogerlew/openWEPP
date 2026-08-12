# Pre-implementation Contract Gate

Status: `PASS / V1 evidence preserved / V2 Milestone 0 complete`

Evidence mode: `Ran + Static`

Ran from `/home/workdir/openWEPP` before production edits:

- `sha256sum .../openwepp_c3_woody_v1_definition.json` -> `003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157`.
- `.venv/bin/python .../reference_calculator.py` -> completed successfully with `all_pass: true` and emitted the admitted vector set.
- `bash tools/release/check_science_contract_admission.sh --base-ref 06f7d8041f7d957a803a52db87fb5957461f84df --worktree` -> `A0_ADMITTED contracts=44 science_surfaces=0`.

Static inspection confirms `SC-VEGETATION-001@5`, `SC-BIOGEOCHEM-001@1`, the equation/parameter/ownership/numerical ledgers, and the model JSON agree on E01--E22, strict complete caller state, typed unsupported branches, and request/authorization/final-use/atomic-commit ordering. No authority contradiction was found. Production implementation may begin.

## 2026-08-12 V2 Intake

The historical V1 gate above remains immutable evidence. Stage-A authority
commit `817b082d01d194cde61b1cf284bd85e40e44afc9` released V2 digest
`38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3`.
V2 authority, digest, unit, oracle, anti-evasion, AUTH11, and A0 checks pass.
Production remediation may resume while the heterogeneous public path remains
fail-closed until the complete V2 transaction passes.
