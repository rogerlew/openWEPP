# Verification Agent A — SC-INFILE-WEPPUI-001

Evidence: Static

## Per-Finding Closure Verification

| finding_id | source | severity | disposition_decision | verifier_verdict | evidence (contract file:line) | notes |
| --- | --- | --- | --- | --- | --- | --- |
| `WUI-A-001` | review_agent_a | high | `amended_closed` | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md:71`, `:75`, `:122`, `:173` | Strict non-ENOENT open failure is typed-error only and no longer collapses into normalized `ui_run=0`. Compatibility collapse path is explicitly modeled via `open_result`. |
| `WUI-A-002` | review_agent_a | high | `amended_closed` | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md:70`, `:79`, `:85`, `:94`, `:145`, `:146` | Requested/effective/divergence surfaces are explicitly modeled, propagated, and exported. |
| `WUI-A-003` | review_agent_a | medium | `amended_closed` | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md:77`, `:92`, `:109`, `:137`, `:170` | Deterministic multi-soil reduction rule (`solwpv_reduced_min`) and guard linkage are codified. |
| `WUI-B-001` | review_agent_b | high | `amended_closed` | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md:71`, `:75`, `:90`, `:122`, `:173` | Same strict IO-collapse inconsistency verified closed with explicit open-branch provenance and typed strict failure path. |
| `WUI-B-002` | review_agent_b | medium | `amended_closed` | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md:70`, `:79`, `:94`, `:145`, `:174` | Requested-vs-effective divergence observability is now executable and guard-linked. |

## Package Verdict

`PASS-WITH-NOTES`

All accepted/amended A/B findings verify as closed against the updated canonical contract.

## Remaining High-Severity Open Findings

None.

## Notes

Non-finding governance HOLD gaps remain in contract gap register (`WEPPUI-GAP-001..002`) at `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md:187` onward.
