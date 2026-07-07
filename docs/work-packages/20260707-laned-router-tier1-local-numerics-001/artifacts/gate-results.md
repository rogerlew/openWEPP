# Gate Results

Status: PASS-SUPERSEDED. Evidence mode: Static + Ran.

| Gate | Result | Evidence |
|---|---|---|
| Supersession legitimacy | PASS | Package status points to GAP-OFEHYB-002; superseding package is complete on `main` at `75b339c9`. |
| Canonical contract state | PASS | `SC-OFEROUTE-002` rev 4 records `GAP-OFEHYB-002` as `RESOLVED-FOR-H2637-SOLVE-COST`. |
| Baseline/profile | PASS-BY-SUPERSESSION | GAP-OFEHYB-002 baseline: H2637 active hybrid `38.39 s` user, `151435969` map evaluations. |
| Implementation | PASS-BY-SUPERSESSION | GAP-OFEHYB-002 exact bare skin-only branch evaluator; no code changes here. |
| Timing/fidelity | PASS-BY-SUPERSESSION | GAP-OFEHYB-002: H2637 user `33.37 s`, map evaluations `0`, numeric dust audited. |
| Review | PASS | `artifacts/review-supersession.md`. |
| Verification | PASS | `artifacts/verification-supersession.md`. |
| `git diff --check` | PASS | Ran for this docs-only closure. |
| Markdown/doc lint | PASS | `markdown-doc lint --path docs/work-packages/20260707-laned-router-tier1-local-numerics-001 --path docs/work-packages/README.md`: `13 files validated`, `0 errors`, `0 warnings`. |
| Rust gates | NOT APPLICABLE | No Rust, contract, fixture, or runtime surface changed in this package execution. Superseding Rust gates are recorded in GAP-OFEHYB-002. |
| `.rs` line-count governance | NOT APPLICABLE | No Rust files touched here; superseding line-count governance is in GAP-OFEHYB-002. |
