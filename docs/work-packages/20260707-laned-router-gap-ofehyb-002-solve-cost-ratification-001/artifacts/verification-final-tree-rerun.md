# Final-Tree Verification Rerun

Status: PASS. Evidence mode: Ran.

This artifact records the post-review-fix rerun after the three additional
regression tests were added. It supersedes the earlier subagent final-gate test
counts for Rust test freshness.

| Gate | Result | Evidence |
|---|---|---|
| `cargo fmt --check` | PASS | Ran with clippy batch after moving the test-only gravity import. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Finished dev profile; no warnings. |
| Focused Lane-D / `ofe_routing` | PASS | `cargo test -p openwepp-hillslope-orchestrator ofe_routing -- --nocapture`: 96 tests selected; `95 passed`, `1 ignored`, `247 filtered`, `151.43 s`. Includes `case4_hybrid_manning_ladder_meets_iwagaki_oracle`, `bare_skin_direct_equilibrium_does_not_authorize_invalid_raw_operands`, `bare_skin_direct_equilibrium_composed_edge_cases_close_cell_residuals`, and `implicit_step_rejects_invalid_inactive_raw_operands_before_direct_path`. |
| Full workspace nextest | PASS | `cargo nextest run --workspace --profile full`: `1438` tests run, `1438 passed`, `4 skipped`, `584.742 s`. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| Authority anti-evasion | PASS | `bash tools/release/check_authority_suite_antievasion.sh`: PASS. |
| Required-suite guard test | PASS | `cargo nextest run --test auth11_required_suite_obligation_guards_contract`: `2 passed`, `0 skipped`. |
| Final doc/diff lint | PASS | `git diff --check`; `markdown-doc lint --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md --path docs/work-packages/README.md --path docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001`: `20 files validated`, `0 errors`, `0 warnings`. |

Freshness note: the earlier subagent artifact
`artifacts/verification-final-gates.md` remains as the original 14-command
batch record. This rerun is the closure authority for gates affected by the
post-review code and documentation edits.
