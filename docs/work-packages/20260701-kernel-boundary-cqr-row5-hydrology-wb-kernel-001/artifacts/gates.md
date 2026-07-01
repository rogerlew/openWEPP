# Gates

| Gate | Status | Evidence |
| --- | --- | --- |
| CRAP before | PASS | See `crap-before.md`; 11 unique offender entries, duplicated to 22 rows by current `cargo crap` report shape. |
| CRAP after | PASS | See `crap-after.md`; full-workspace LCOV + `cargo crap` reported `0` row #5 owned functions above CRAP 30. |
| Secondary typed assertions | PASS | Added `cqr_row5` focused tests for hydrology guard error code/display coverage, snow albedo display variants, snow-density mass-boundary behavior, R7G JSON escaping, frozen-soil k-factor resolution, snow-density guard mapping, SIMIMPL29 melt branches, and active-snow coupling edge branches. |
| H2637 identity | PASS | Release `openwepp-cli-hill` exited 0; wall `1:08.04`, max RSS `79916 KiB`; manifest selected `direct-production-executor`, `compatibility_edge_invocations=0`, and HBP/loss/plot/WAT/PASS are byte-identical to `/tmp/typed-direct-carrier-identity/base/output`. |
| `cargo fmt --check` | PASS | Exited 0. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Exited 0. |
| `cargo nextest run --workspace --profile full` | PASS | `1254` tests run, `1254` passed, `1` skipped. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| Authority anti-evasion | PASS | `PASS: authority suite anti-evasion checks passed.` |
| Required-suite obligation guard | PASS | `cargo nextest run --test auth11_required_suite_obligation_guards_contract`: `2` tests run, `2` passed. |
| Line-count governance | PASS | See `line-count-governance.md`; all row #5 touched Rust files are below 3000 lines. |
| Markdown docs | PASS | `markdown-doc lint` and `markdown-doc validate` on row #5 package plus `docs/work-packages/README.md`: 10 files, 0 errors. |
