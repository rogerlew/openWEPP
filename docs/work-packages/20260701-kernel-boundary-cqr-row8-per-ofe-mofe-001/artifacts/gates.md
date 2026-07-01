# Gates

| Gate | Status | Evidence |
| --- | --- | --- |
| CRAP before | PASS | See `crap-before.md`; 2 unique offender entries, duplicated to 4 rows by current `cargo crap` report shape. |
| CRAP after | PASS | See `crap-after.md`; full-workspace LCOV + `cargo crap` reported `0` row #8 owned functions above CRAP 30. |
| Secondary typed assertions | PASS | Added `cqr_row8` focused tests for trace filtering, percolation JSON-line serialization, and subsurface saturation JSON-line serialization. |
| H2637 identity | PASS | Release `openwepp-cli-hill` exited 0; wall `1:07.23`, max RSS `79420 KiB`; manifest selected `direct-production-executor`, `compatibility_edge_invocations=0`, and HBP/loss/plot/WAT/PASS are byte-identical to `/tmp/typed-direct-carrier-identity/base/output`. |
| `cargo fmt --check` | PASS | Exited 0. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Exited 0. |
| `cargo nextest run --workspace --profile full` | PASS | `1260` tests run, `1260` passed, `1` skipped, `1` slow. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| Authority anti-evasion | PASS | `PASS: authority suite anti-evasion checks passed.` |
| Required-suite obligation guard | PASS | `cargo nextest run --test auth11_required_suite_obligation_guards_contract`: `2` tests run, `2` passed. |
| Line-count governance | PASS | See `line-count-governance.md`; row #8 owned Rust files are below 3000 lines. |
| Markdown docs | PASS | `markdown-doc lint` and `markdown-doc validate` on row #8 package plus `docs/work-packages/README.md`: 10 files, 0 errors. |
