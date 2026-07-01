# Gates

| Gate | Status | Evidence |
| --- | --- | --- |
| CRAP before | PASS | See `crap-before.md`; row #2 production scope has `0` functions above CRAP 30. |
| CRAP after | PASS | See `crap-after.md`; full-workspace LCOV + `cargo crap` reported `0` row #2 owned production functions above CRAP 30. |
| Secondary typed assertions | PASS | Added focused climate parser/runtime tests for non-breakpoint direct forcing, breakpoint direct forcing, datver-0 override behavior, itemp runtime rejection, and direct-day out-of-range errors. |
| H2637 identity | PASS | Release `openwepp-cli-hill` exited 0; wall `1:07.75`, max RSS `77720 KiB`; manifest selected `direct-production-executor`, `compatibility_edge_invocations=0`, and HBP/loss/plot/WAT/PASS are byte-identical to `/tmp/typed-direct-carrier-identity/base/output`. |
| `cargo fmt --check` | PASS | Exited 0. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Exited 0. |
| `cargo nextest run --workspace --profile full` | PASS | `1272` tests run, `1272` passed, `1` skipped, `1` slow. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| Authority anti-evasion | PASS | `PASS: authority suite anti-evasion checks passed.` |
| Required-suite obligation guard | PASS | `cargo nextest run --test auth11_required_suite_obligation_guards_contract`: `2` tests run, `2` passed. |
| Line-count governance | PASS | See `line-count-governance.md`; row #2 production Rust file is below 3000 lines. |
| Markdown docs | PASS | `markdown-doc lint`, `markdown-doc validate`, and `git diff --check` passed after final artifact edits. |
