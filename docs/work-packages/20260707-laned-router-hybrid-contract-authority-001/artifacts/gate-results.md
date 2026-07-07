# LANED-HYB-SC — Gate results

Status: **EXECUTED-COMPLETE** (2026-07-07). Evidence mode: **Ran** (all
commands invoked on the approved rev-2 tree).

| Gate | Command | Result |
|---|---|---|
| Markdown lint | `markdown-doc lint --path SC-OFEROUTE-002.md --path SC-OFEROUTE-001.md --path index.md --path docs/work-packages/README.md --path <this package>` | **PASS** — `15 files validated, 0 errors, 0 warnings` |
| BEI schema (new contract) | `python tools/check_sc_binding_exposure.py …/SC-OFEROUTE-002.md` | **PASS-DEFERRED** — 4 rows, 4 routed `science-review-follow-on` (the expected verdict for an unpromoted experimental subsystem) |
| BEI schema (parent) | `python tools/check_sc_binding_exposure.py …/SC-OFEROUTE-001.md` | **PASS-DEFERRED** — 7 rows, 6 routed `science-review-follow-on`, no schema failures |
| SC unit compliance (new) | `bash tools/release/check_sc_unit_compliance.sh --path …/SC-OFEROUTE-002.md` | **PASS** |
| SC unit compliance (parent) | same, `…/SC-OFEROUTE-001.md` | **PASS** |
| Verification Agent A | `verification_agent_a.md` | **PASS** — GO for approval lift from Agent A scope |
| Verification Agent B | `verification_agent_b.md` + `verification_agent_b_followup.md` | **PASS** — initial Low B-L1 hold fixed; follow-up GO for approval lift from Agent B scope |
| Diff whitespace | `git diff --check` | **PASS** |
| Cargo gates | — | **NOT RUN, out of scope** — no code touched in this package (docs-only change set) |

Consolidation-rule compliance is the DUAL REVIEW's surface (nothing
dropped / nothing smuggled / code-vs-contract fidelity), not a lint. Review,
disposition, and verification are archived under
`artifacts/science-contracts/SC-OFEROUTE-002/`.
