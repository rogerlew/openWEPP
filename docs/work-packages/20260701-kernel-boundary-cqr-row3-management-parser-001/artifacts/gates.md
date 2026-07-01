# Gates

| Gate | Status | Evidence |
| --- | --- | --- |
| CRAP before | PASS | See `crap-before.md`; 1 unique offender entry, duplicated to 2 rows by current `cargo crap` report shape. |
| CRAP after | PASS | See `crap-after.md`; full-workspace LCOV + `cargo crap` reported `0` row #3 owned functions above CRAP 30. |
| Secondary typed assertions | PASS | Added `cqr_row3` focused tests for disabled drain projection, enabled geometry projection, dangling drain references, and zero geometry fail-closed behavior. |
| H2637 identity | PASS | Release `openwepp-cli-hill` exited 0; wall `1:06.75`, max RSS `80052 KiB`; manifest selected `direct-production-executor`, `compatibility_edge_invocations=0`, and HBP/loss/plot/WAT/PASS are byte-identical to `/tmp/typed-direct-carrier-identity/base/output`. |
| `cargo fmt --check` | PASS | Exited 0. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Exited 0 after replacing test-only exact float comparisons with a tolerance helper. |
| `cargo nextest run --workspace --profile full` | PASS | `1264` tests run, `1264` passed, `1` skipped, `2` slow. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| Authority anti-evasion | PASS | `PASS: authority suite anti-evasion checks passed.` |
| Required-suite obligation guard | PASS | `cargo nextest run --test auth11_required_suite_obligation_guards_contract`: `2` tests run, `2` passed. |
| Line-count governance | PASS | See `line-count-governance.md`; row #3 touched Rust file is below 3000 lines. |
| Markdown docs | PASS | `markdown-doc lint` and `markdown-doc validate` on row #3 package plus `docs/work-packages/README.md`: 10 files, 0 errors. |
