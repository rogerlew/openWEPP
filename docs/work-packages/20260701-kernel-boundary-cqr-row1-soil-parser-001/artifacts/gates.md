# Gates

| Gate | Status | Evidence |
| --- | --- | --- |
| CRAP before | PASS | See `crap-before.md`; row #1 production scope has `0` functions above CRAP 30. |
| CRAP after | PASS | See `crap-after.md`; full-workspace LCOV + `cargo crap` reported `0` row #1 owned production functions above CRAP 30. |
| Secondary typed assertions | PASS | Restored 9002 policy/measured FC-WP, typed corrected theta stores, and harmonic vertical `ssc` assertions in `infile_soil_parser_contract.rs`. |
| H2637 identity | PASS | Release `openwepp-cli-hill` exited 0; wall `1:06.89`, max RSS `77756 KiB`; manifest selected `direct-production-executor`, `compatibility_edge_invocations=0`, and HBP/loss/plot/WAT/PASS are byte-identical to `/tmp/typed-direct-carrier-identity/base/output`. |
| `cargo fmt --check` | PASS | Exited 0. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Exited 0. |
| `cargo nextest run --workspace --profile full` | PASS | `1267` tests run, `1267` passed, `1` skipped, `2` slow. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| Authority anti-evasion | PASS | `PASS: authority suite anti-evasion checks passed.` |
| Required-suite obligation guard | PASS | `cargo nextest run --test auth11_required_suite_obligation_guards_contract`: `2` tests run, `2` passed. |
| Line-count governance | PASS | See `line-count-governance.md`; row #1 production Rust file is below 3000 lines. |
| Markdown docs | PASS | `markdown-doc lint`, `markdown-doc validate`, and `git diff --check` passed after final artifact edits. |
