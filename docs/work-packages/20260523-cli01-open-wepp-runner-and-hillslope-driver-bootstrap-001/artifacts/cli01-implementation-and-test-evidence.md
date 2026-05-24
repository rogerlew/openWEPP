# CLI01 Implementation and Test Evidence

Status: complete
Evidence mode: Static + Ran

## Static
- Added new crate and binaries:
  - `crates/openwepp-runner/src/lib.rs`
  - `crates/openwepp-runner/src/bin/open_wepp_runner.rs`
  - `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`
  - `crates/openwepp-runner/Cargo.toml`
- Added workspace wiring:
  - root `Cargo.toml` workspace member + dependency for `openwepp-runner`
  - `Cargo.lock` updates for new dependency graph
- Implemented CLI01 runner/hillslope surfaces:
  - explicit launcher argv boundary (`open_wepp_runner run-hillslope`)
  - release lint boundary (`open_wepp_runner release lint --release-dir <path>`)
  - blind run-directory sidecar discovery with strict/compat adapter policy
  - typed hard-fail behavior for missing required sidecars and required outputs
  - deterministic manifest emission (`openwepp-hillslope-run-manifest-v1`)
  - build/release sidecar write + validation (`<binary>.json`)
- Added CLI01 contract-derived and integration tests:
  - `tests/integration/cli01_runner_contract_derived_tests.rs`
  - `tests/integration/cli01_runner_hillslope_integration.rs`
  - root `Cargo.toml` test target entries for both integration files
- Added CLI01 fixtures:
  - `tests/fixtures/cli01/hillslope_run_dir/*`
  - `tests/fixtures/cli01/hillslope_run_dir_unknown/*`

## Ran
- `cargo fmt --check`
  - Result: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Result: pass.
- `cargo test --workspace`
  - Result: pass.
  - CLI01-specific evidence from run:
    - `tests/integration/cli01_runner_contract_derived_tests.rs`: 6 passed.
    - `tests/integration/cli01_runner_hillslope_integration.rs`: 5 passed.
- `cargo deny check`
  - Result: pass (`advisories ok, bans ok, licenses ok, sources ok`).
  - Note: `license-not-encountered` warnings were emitted for unmatched allowed
    licenses in `deny.toml`; these are warnings, not gate failures.
