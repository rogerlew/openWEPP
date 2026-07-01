# Gates

| Gate | Status | Evidence |
| --- | --- | --- |
| CRAP before | PASS | See `crap-before.md`; 24 unique offender entries, duplicated to 48 rows by current `cargo crap` report shape. |
| CRAP after | PASS | See `crap-after.md`; final full-workspace LCOV + `cargo crap` found 0 row #4 entries above CRAP 30. |
| Secondary typed assertions | PASS | Added row #4 tests for runtime input error codes/display, annual-extension naming, perennial grazing projection, SIMIMPL28 forcing trigger/sunmap/partition branches; `cargo nextest run -p openwepp-hillslope-orchestrator` passed 118 tests. |
| H2637 identity | PASS | `openwepp-cli-hill` exited 0; HBP, loss, plot, WAT, and PASS outputs byte-identical against `/tmp/typed-direct-carrier-identity/base/output`; wall `1:06.99`, max RSS `79684 KiB`. |
| `cargo fmt --check` | PASS | Exited 0. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Exited 0. |
| `cargo nextest run --workspace --profile full` | PASS | `1229` tests passed, `1` skipped, `2` slow; elapsed `650.299s`. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| Authority anti-evasion | PASS | `PASS: authority suite anti-evasion checks passed.` |
| Required-suite obligation guard | PASS | `cargo nextest run --test auth11_required_suite_obligation_guards_contract`: 2 passed. |
| Line-count governance | PASS | Row #4 files: `1715`, `197`, `1005`, `1289`, `26` lines; all below the local 2000-line review threshold. |
| Docs and diff hygiene | PASS | `git diff --check` exited 0; `markdown-doc lint` and `markdown-doc validate` scanned 9 files with 0 findings. |
