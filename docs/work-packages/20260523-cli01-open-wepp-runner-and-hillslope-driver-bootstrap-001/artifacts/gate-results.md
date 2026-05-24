# Gate Results

Status: pass
Evidence mode: Ran

## Ran
Required validation gates (package exit criteria):

1. `cargo fmt --check`
- Result: pass.

2. `cargo clippy --workspace --all-targets -- -D warnings`
- Result: pass.

3. `cargo test --workspace`
- Result: pass.
- CLI01-focused tests:
  - `cli01_runner_contract_derived_tests`: pass (6 tests).
  - `cli01_runner_hillslope_integration`: pass (5 tests).

4. `cargo deny check`
- Result: pass (`advisories ok, bans ok, licenses ok, sources ok`).
- Non-blocking warnings observed: `license-not-encountered` for unmatched
  allowlist entries in `deny.toml`.

Additional runtime gate evidence:
- `open_wepp_runner run-hillslope ... --policy strict`: pass, required outputs present.
- `open_wepp_runner release lint --release-dir /tmp/cli01_release_lint_uQLT9H`: pass.
