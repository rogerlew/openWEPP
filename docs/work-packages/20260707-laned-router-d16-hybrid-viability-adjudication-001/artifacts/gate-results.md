# Gate Results

Status: COMPLETE. Evidence mode: Static + Ran.

| Gate | Status | Evidence |
|---|---|---|
| `git diff --check` | PASS | Ran after package edits; no whitespace errors. |
| Markdown/doc lint for touched docs | PASS | `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260707-laned-router-d16-hybrid-viability-adjudication-001` passed. |
| Claude review artifact lint | PASS | Operator-reported `markdown-doc lint` on `artifacts/review-claude.md`: 1 file, 0 errors, 0 warnings. |
| Selected-cohort evidence audit | PASS | Read current selected-cohort command log, summary JSON, and timing logs from the row-crop canhgt package. |
| `SC-OFEROUTE-002#INV-OFEHYB-008` audit | PASS-HOLD | Promotion remains held because fidelity/timing tolerances are not met. |
| Review finding disposition | PASS | `review-claude.md` GO accepted. CL-H1/CL-H2/CL-M1/CL-M2 constraints were added to `worker-handoff.md`; CL-L1 stale record line removed; CL-I1 left as watch item. |
| Contract/profile/BEI checks | N/A | No `SC-*` contract was touched. |
| Rust closure gates | N/A | No Rust implementation files were changed. Prior selected-cohort package already recorded the full Rust gates for the code fix. |
| `.rs` line-count governance | N/A | No `.rs` files were touched. |
