# Gate Results

Status: complete
Evidence mode: ran
Date: 2026-05-27

## Ran
1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test -p openwepp-hillslope-orchestrator`
4. `cargo test --workspace`
5. `cargo deny check`

## Result
- `cargo fmt --check`: pass (`cargo_fmt_check.exit_code=0`)
- `cargo clippy --workspace --all-targets -- -D warnings`: pass (`cargo_clippy_workspace.exit_code=0`)
- `cargo test -p openwepp-hillslope-orchestrator`: pass (`cargo_test_hillslope_orchestrator.exit_code=0`)
- `cargo test --workspace`: pass (`cargo_test_workspace.exit_code=0`)
- `cargo deny check`: pass (`cargo_deny.exit_code=0`; warnings only)

Warning capture (`cargo deny check`):
- duplicate lock entries:
  - `hashbrown`
  - `twox-hash`
- unmatched license allowlist entries:
  - `ISC`
  - `Unicode-DFS-2016`

## Evidence bundle
- `artifacts/gates-20260527T004800Z/`
