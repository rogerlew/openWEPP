# Gate Results

Status: completed

Evidence mode: Ran

Ran:
- `cargo fmt --check`: pass after applying `cargo fmt`.
- `cargo test --test wb18_percolation_physics_kernel_contract -- --nocapture`:
  pass (`15/15`).
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo test --workspace`: pass.
- `cargo deny check`: pass (`advisories ok, bans ok, licenses ok, sources ok`);
  pre-existing duplicate/unmatched-license warnings emitted by deny.
- `git diff --check`: pass.
- `cargo build -p openwepp-runner --bin openwepp-cli-hill`: pass.
- Full `H1..H39` runtime suite: pass (`39/39`).
- Full `H1..H39` semantic comparator report generation: pass (`39/39`).
- Post-closeout tracked plus untracked package whitespace checks: pass.

Raw log index:
- `artifacts/gate-logs/status.tsv`
- `artifacts/gate-logs/cargo_fmt_check.log`
- `artifacts/gate-logs/wb18_contract_test.log`
- `artifacts/gate-logs/cargo_clippy.log`
- `artifacts/gate-logs/cargo_test_workspace.log`
- `artifacts/gate-logs/cargo_deny_check.log`
- `artifacts/gate-logs/git_diff_check.log`
