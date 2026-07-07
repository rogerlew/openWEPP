# LANED-HYB-SC — Gate results

Status: **EXECUTED** (2026-07-07). Evidence mode: **Ran** (all commands
invoked this session on the authored tree).

| Gate | Command | Result |
|---|---|---|
| Markdown lint | `markdown-doc lint --path SC-OFEROUTE-002.md --path SC-OFEROUTE-001.md --path index.md --path <this package>` | **PASS** — `4 files validated, 0 errors, 0 warnings` |
| BEI schema (new contract) | `python tools/check_sc_binding_exposure.py …/SC-OFEROUTE-002.md` | **PASS-DEFERRED** — 4 rows, 4 routed `science-review-follow-on` (the expected verdict for an unpromoted experimental subsystem) |
| BEI schema (parent) | `python tools/check_sc_binding_exposure.py …/SC-OFEROUTE-001.md` | **PASS-DEFERRED** — 7 rows (pointer row replaces the full hybrid row), no schema failures |
| SC unit compliance (new) | `bash tools/release/check_sc_unit_compliance.sh --path …/SC-OFEROUTE-002.md` | **PASS** (after fixing two schema findings at authoring: `Units` column header; alias-map 4-column schema) |
| SC unit compliance (parent) | same, `…/SC-OFEROUTE-001.md` | **PASS** |
| Cargo gates | — | **NOT RUN, out of scope** — no code touched in this package (docs-only change set) |

Consolidation-rule compliance is the DUAL REVIEW's surface (nothing
dropped / nothing smuggled / code-vs-contract fidelity), not a lint —
review prompt at
`prompts/active/20260707-codex-review-sc-oferoute-002_prompt.md`.
