# Gates

| Gate | Status | Evidence |
| --- | --- | --- |
| CRAP before | PASS | See `crap-before.md`; 17 unique offender entries, duplicated to 34 rows by current `cargo crap` report shape. |
| CRAP after | PASS | Full workspace LCOV + `cargo crap` refreshed to `/tmp/openwepp-crap-row7-after-final.json`; row #7 owned offender count above 30 is `0`. |
| Secondary typed assertions | PASS | Added stable typed assertions for retained publication-frame guards, snow/frost insulation helpers, snow selector parsing, Sturm climate normals, growth/residue projection, Priestley-Taylor demand, no-final-frost rebalance, frost carry projection, WB11 frozen-depth refresh, and WB16 equivalent-plane alpha. |
| H2637 identity | PASS | Release `openwepp-cli-hill` exited 0; wall `1:07.39`, max RSS `79588 KiB`; manifest selected `direct-production-executor`, `compatibility_edge_invocations=0`, and HBP/loss/plot/WAT/PASS are byte-identical to `/tmp/typed-direct-carrier-identity/base/output`. |
| `cargo fmt --check` | PASS | Exited 0 after final clippy cleanup. |
| `git diff --check` | PASS | Exited 0 after final clippy cleanup. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Exited 0. |
| `cargo nextest run --workspace --profile full` | PASS | `1239` tests run: `1239` passed, `1` skipped; two slow tests. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| Authority anti-evasion | PASS | `bash tools/release/check_authority_suite_antievasion.sh`: `PASS: authority suite anti-evasion checks passed.` |
| Required-suite obligation guard | PASS | `cargo nextest run --test auth11_required_suite_obligation_guards_contract`: `2` tests passed. |
| Markdown docs | PASS | `markdown-doc lint` and `markdown-doc validate` on row #7 package plus `docs/work-packages/README.md`: 9 files, 0 errors. |
| Line-count governance | PASS-WARN | See `line-count-governance.md`; existing `00_builders_and_authority.rs` remains above 3000 lines under a row-scoped exception with owner and sunset. |
