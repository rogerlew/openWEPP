# Verification Agent A — SC-INFILE-TC-001

Evidence: Static

## Per-Finding Closure Verification

| finding_id | source | severity | disposition_decision | verifier_verdict | evidence (contract file:line) | notes |
| --- | --- | --- | --- | --- | --- | --- |
| `TC-A-001` | review_agent_a | high | `amended_closed` | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md:68`, `:73`, `:84`, `:166` | Strict non-ENOENT open-failure path is typed error/no normalized emission; missing/collapsed compatibility branches remain explicit. |
| `TC-A-002` | review_agent_a | medium | `amended_closed` | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md:74`, `:90`, `:134`, `:169` | Watershed-only applicability is now driven by explicit `run_context` model/propagation and guard linkage. |
| `TC-A-003` | review_agent_a | medium | `amended_closed` | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md:71`, `:72`, `:87`, `:88`, `:157`, `:171` | Content-insensitive warning trigger surfaces are explicit and mapped to compatibility warning semantics. |
| `TC-B-001` | review_agent_b | high | `amended_closed` | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md:84`, `:166` | `luntc` propagation now explicitly includes strict open-error guard path (`G-TC-003`) and no strict fault masking remains. |
| `TC-B-002` | review_agent_b | medium | `amended_closed` | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md:72`, `:88`, `:144`, `:157`, `:171` | Field-level warning trigger/export distinction for ignored body content is present and executable. |

## Package Verdict

`PASS-WITH-NOTES`

All accepted/amended A/B findings verify as closed against the updated canonical contract.

## Remaining High-Severity Open Findings

None.

## Notes

Non-finding governance HOLD gaps remain in contract gap register (`TC-GAP-001..003`) at `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md:184` onward.
