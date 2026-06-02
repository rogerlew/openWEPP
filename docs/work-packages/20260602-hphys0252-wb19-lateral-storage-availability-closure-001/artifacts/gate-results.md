# Gate Results

Status: complete

Evidence mode: ran

Ran:

| Gate | Result | Evidence |
|---|---|---|
| `cargo test --test wb19_lateral_drainage_physics_kernel_contract hphys0252 -- --nocapture` pre-implementation | expected fail | `artifacts/gate-logs/pre_implementation_hphys0252_wb19.log` |
| `cargo test --test wb19_lateral_drainage_physics_kernel_contract hphys0252 -- --nocapture` post-implementation | pass `1/1` | `artifacts/gate-logs/post_implementation_hphys0252_wb19.log` |
| `cargo test --test wb19_lateral_drainage_physics_kernel_contract` | pass `12/12` | `artifacts/gate-logs/final_wb19_contract_full.log` |
| Full `H1..H39` runtime suite | pass `39/39` | `/tmp/hphys0252_20260602T195147Z/reports/hillslope_batch_status.tsv` |
| Full `H1..H39` semantic reports | completed `39/39`, semantic pass `0/39` | `/tmp/hphys0252_20260602T195147Z/reports/semantic_status.tsv` |
| `cargo fmt --check` | pass | `artifacts/gate-logs/cargo_fmt_check.log` |
| `cargo clippy --workspace --all-targets -- -D warnings` | pass | `artifacts/gate-logs/cargo_clippy_workspace.log` |
| `cargo test --workspace` | pass | `artifacts/gate-logs/cargo_test_workspace.log` |
| `cargo deny check` | pass with duplicate/unmatched-license warnings | `artifacts/gate-logs/cargo_deny_check.log` |
| `git diff --check` | pass | `artifacts/gate-logs/git_diff_check.log` |
| `bash tools/release/check_authority_suite_antievasion.sh` | pass | `artifacts/gate-logs/check_authority_suite_antievasion.log` |
| `cargo test --test auth11_required_suite_obligation_guards_contract` | pass `2/2` | `artifacts/gate-logs/auth11_required_suite_obligation_guards_contract.log` |
