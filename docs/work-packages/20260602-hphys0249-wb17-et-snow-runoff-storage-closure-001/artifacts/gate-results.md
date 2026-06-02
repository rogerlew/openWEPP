# Gate Results

Status: complete

Evidence mode: ran

Ran:

| Gate | Result | Evidence |
|---|---|---|
| `cargo fmt --check` | passed | `artifacts/gate-logs/cargo_fmt_check.log` |
| `git diff --check` | passed | `artifacts/gate-logs/git_diff_check.log` |
| `cargo clippy --workspace --all-targets -- -D warnings` | passed | `artifacts/gate-logs/cargo_clippy_workspace_all_targets.log` |
| `cargo test --workspace` | passed | `artifacts/gate-logs/cargo_test_workspace.log` |
| `cargo deny check` | passed with existing warnings | `artifacts/gate-logs/cargo_deny_check.log` |
| `bash tools/release/check_authority_suite_antievasion.sh` | passed | `artifacts/gate-logs/check_authority_suite_antievasion.log` |
| `cargo test --test auth11_required_suite_obligation_guards_contract` | passed | `artifacts/gate-logs/auth11_required_suite_obligation_guards_contract.log` |
| `cargo test --test wb17_et_physics_kernel_contract -- --nocapture` | passed `9/9` | `artifacts/gate-logs/final_wb17_contract_test.log` |
| `wctl doc-lint --path <package>` | passed, `0` files validated | `artifacts/gate-logs/doc_lint_package.log` |

Notes:

- `cargo deny check` emitted duplicate-crate and unmatched-license-allowance
  warnings but completed with `advisories ok, bans ok, licenses ok, sources ok`.
- Earlier final-gate attempts exposed stale fixture expectations after adding
  the post-WB19 `PlantRootUptake` phase; those were corrected and the final
  workspace run passed.
