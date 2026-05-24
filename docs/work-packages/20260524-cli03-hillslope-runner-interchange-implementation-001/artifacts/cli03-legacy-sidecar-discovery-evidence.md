# CLI03 Legacy Sidecar Discovery Evidence

Status: completed
Evidence mode: Static + Ran

## Static
Legacy-sidecar mode behavior is implemented and verified:
- CLI binaries now accept and forward `--legacy-sidecar-discovery`:
  - `crates/openwepp-runner/src/bin/open_wepp_runner.rs`
  - `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`
- runner execution path preserves legacy discovery authority when enabled:
  - `sidecar_discovery_mode = legacy-sidecar-discovery` in manifest,
  - discovered sidecars are authoritative,
  - `.run` sidecar override values are ignored for sidecar binding precedence.

Integration test coverage:
- `cli03_legacy_sidecar_discovery_mode_uses_legacy_sidecars_and_ignores_runfile_overrides`

## Ran
- Command:
  - `cargo test --test cli03_runner_contract_derived_tests`
- Observed:
  - pass (`9 passed; 0 failed`).
- Relevant legacy discovery test:
  - `cli03_legacy_sidecar_discovery_mode_uses_legacy_sidecars_and_ignores_runfile_overrides`
