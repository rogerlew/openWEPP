# No-Regression Gates

Evidence class: `Ran`.

| Gate | Status | Evidence |
|---|---|---|
| `cargo fmt --check` | PASS | Completed after code/docs finalization. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Completed after the default-selection changes; no warnings. |
| `cargo test --workspace` | PASS | Full workspace suite passed with the activated default and compatibility fallbacks. |
| Conservation/closure contract checks | PASS | Covered by the full workspace suite's water-balance, frost, snow, and publication closure tests under the activated default. |
| `cargo deny check` | PASS | Advisories, bans, licenses, and sources all passed. |
| `bash tools/release/check_authority_suite_antievasion.sh` | PASS | Authority-suite anti-evasion checks passed. |
| `cargo test --test auth11_required_suite_obligation_guards_contract` | PASS | Two required-suite obligation guard tests passed. |
| Runtime default direct + compatibility rollback | PASS | Full suite includes direct default manifest tests, explicit compatibility rollback tests, legacy sidecar-discovery fallback, and compatibility-pinned standalone runner contracts. |
| `markdown-doc lint` scoped to touched docs | PASS | Ten touched Markdown files validated with zero errors and zero warnings. |
