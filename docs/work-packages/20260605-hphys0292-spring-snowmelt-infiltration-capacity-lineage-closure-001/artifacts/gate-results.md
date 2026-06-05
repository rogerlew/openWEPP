# Gate Results

Status: executed
Evidence mode: Ran

Ran:

- `cargo fmt --check` — pass, log `/tmp/hphys0292_cargo_fmt_check.log`.
- `cargo clippy --workspace --all-targets -- -D warnings` — pass, log `/tmp/hphys0292_cargo_clippy.log`.
- `cargo test --workspace` — pass, log `/tmp/hphys0292_cargo_test_workspace.log`.
- `cargo deny check` — pass with existing license-not-encountered warnings, log `/tmp/hphys0292_cargo_deny.log`.
- `bash tools/release/check_authority_suite_antievasion.sh` — pass, log `/tmp/hphys0292_antievasion.log`.
- `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture` — pass, log `/tmp/hphys0292_auth11_guard.log`.

Focused gates:

- `cargo test --test hphys0292_spring_snowmelt_infiltration_capacity_contract -- --nocapture` — pass.
- `cargo test --test wb14_infiltration_hyetograph_kernel_contract -- --nocapture` — pass.
- `cargo test -p openwepp-runner hphys0245_trace -- --nocapture` — pass.
