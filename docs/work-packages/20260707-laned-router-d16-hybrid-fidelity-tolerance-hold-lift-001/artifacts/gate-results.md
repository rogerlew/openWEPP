# Gate Results

Status: EXECUTED-HOLD-COHORT-AUTHORITY. Evidence mode: Static + Ran.

| Gate | Status | Evidence |
|---|---|---|
| `git diff --check` | PASS | Ran after final verification artifacts; exit `0`. |
| Markdown/doc lint | PASS | Package path: 17 files, 0 errors/warnings. README path: 1 file, 0 errors/warnings. |
| owcmp manifest env preflight | PASS | All three suite manifests pass `tools/owcmp/owcmp env --manifest`; see `artifacts/owcmp-env-preflight.log`. |
| owcmp executable suite preflight | BLOCKED | All three manifests are `cohort-inventory`; `manifest run` refuses them as preflight declarations; see `artifacts/owcmp-manifest-run-preflight.log`. |
| Active-runnable cohort preflight | BLOCKED | Zero `routing_coefficients` matches across repo fixtures and external run roots; copied active runs all fail closed. |
| H2637 evidence reuse | PASS | D16 evidence reused; H2637 alone remains insufficient by D16 hold audit. |
| Contract/profile/BEI checks | NOT RUN | No contract edits yet. |
| Protected-output byte identity | NOT RUN | No selector/default edits yet. |
| Focused Lane-D / `ofe_routing` tests | NOT RUN | No code/contract edits yet. |
| `cargo fmt --check` | PASS | Ran after final verification artifacts; exit `0`. |
| Full Rust closure gates | NOT RUN | No Rust edits yet. |
| `.rs` line-count governance | PASS | `git diff --name-only -- '*.rs'` returned no files. |
