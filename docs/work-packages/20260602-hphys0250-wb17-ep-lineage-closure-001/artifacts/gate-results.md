# Gate Results

Status: complete

Evidence mode: ran

| Gate | Result | Log |
|---|---|---|
| `cargo test -p openwepp-hillslope-orchestrator hphys0250_ -- --nocapture` | pass | `gate-logs/post_impl_hphys0250_orchestrator_tests.log` |
| `cargo test -p openwepp-hillslope-orchestrator management_runtime_projection_assimilates_initial_perennial_live_canopy -- --nocapture` | pass | `gate-logs/post_impl_initial_canopy_projection_test.log` |
| `cargo test -p openwepp-runner hphys0250_ -- --nocapture` | pass | `gate-logs/post_impl_hphys0250_runner_tests.log` |
| `cargo test -p openwepp-runner hphys0245_trace -- --nocapture` | pass | `gate-logs/post_impl_hphys0245_trace_tests.log` |
| `cargo test -p openwepp-runner --lib -- --nocapture` | pass | `gate-logs/post_impl_openwepp_runner_lib_tests.log` |
| `cargo test --test wb17_et_physics_kernel_contract -- --nocapture` | pass | `gate-logs/post_impl_wb17_et_contract_tests.log` |
| `cargo fmt --check` | pass | `gate-logs/cargo_fmt_check.log` |
| `cargo clippy --workspace --all-targets -- -D warnings` | pass | `gate-logs/cargo_clippy_workspace_all_targets.log` |
| `cargo test --workspace` | pass | `gate-logs/cargo_test_workspace.log` |
| `cargo deny check` | pass with existing warnings | `gate-logs/cargo_deny_check.log` |
| `bash tools/release/check_authority_suite_antievasion.sh` | pass | `gate-logs/check_authority_suite_antievasion.log` |
| `cargo test --test auth11_required_suite_obligation_guards_contract` | pass | `gate-logs/auth11_required_suite_obligation_guards_contract.log` |
| `git diff --check` | pass | `gate-logs/git_diff_check.log` |
| Full `H1..H39` runtime | pass `39/39` | runtime root in `latest-run-root.txt` |
| Full `H1..H39` semantic comparator | complete `39/39`, semantic pass `0/39` | runtime root in `latest-run-root.txt` |
| `cargo fmt --check` follow-up | pass | `gate-logs/followup_cargo_fmt_check.log` |
| `cargo test -p openwepp-hillslope-orchestrator management_runtime_projection_preserves_wepppy_corn_no_till_growth_coefficients -- --nocapture` | pass `1/1` | `gate-logs/followup_wepppy_corn_growth_projection_test.log` |
| `git diff --check` follow-up | pass | `gate-logs/followup_git_diff_check.log` |
