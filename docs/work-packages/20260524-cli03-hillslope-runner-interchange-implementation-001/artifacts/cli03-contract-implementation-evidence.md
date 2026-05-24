# CLI03 Contract Implementation Evidence

Status: completed
Evidence mode: Static + Ran

## Static
- Implemented CLI03 runner/interchange behavior at the openWEPP Rust boundary:
  - `crates/openwepp-runner/src/lib.rs`
  - `crates/openwepp-runner/src/bin/open_wepp_runner.rs`
  - `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`
  - `crates/openwepp-runner/Cargo.toml`
- Contract-required `.run` execution guards are implemented in
  `parse_runfile_execution_config`:
  - schema must equal `openwepp-hillslope-runfile-v1`,
  - `run_name` must be non-empty,
  - `unit_system` must equal `metric`,
  - required input file bindings (`soil`, `management`, `slope`, `climate`)
    must resolve to readable files,
  - required output bindings (`outputs.pass`, `outputs.loss`) and optional
    output bindings (`wat`, `soil`, `plot`, `ebe`, `element`) are resolved and
    validated.
- Output contract validation/writer/checksum responsibilities are delegated to
  dedicated crate `crates/openwepp-hillslope-output/`:
  - `contracts.rs` (`OHOUT-E-001`, `OHOUT-E-002`),
  - `writers.rs`,
  - `manifest.rs` checksum assembly.
- Legacy sidecar precedence contract is implemented:
  - when `legacy_sidecar_discovery=true`, discovered sidecars are
    authoritative and `.run` sidecar overrides are ignored for binding
    authority;
  - when `legacy_sidecar_discovery=false`, `.run` sidecar controls are applied
    through explicit optional bindings.
- Runner manifest records sidecar discovery mode and deterministic checksum
  maps through `assemble_output_checksums`.
- CLI runner binaries now parse/forward `--legacy-sidecar-discovery`.
- Python wrapper boundary alignment is implemented in
  `open_wepp_runner/open_wepp_runner.py`:
  - TOML runfile emission for `openwepp-hillslope-runfile-v1`,
  - required output verification from runfile `outputs.pass`/`outputs.loss`,
  - explicit rejection of legacy ASCII pass-family runfile generation
    (`OPEN_RUNNER-E-026`).

## Ran
- `cargo test -p openwepp-hillslope-output`
  - pass (`11 passed; 0 failed`).
- `cargo test --test cli03_runner_contract_derived_tests`
  - pass (`9 passed; 0 failed`).
- `cargo test --test cli01_runner_hillslope_integration -- --test-threads=1`
  - pass (`5 passed; 0 failed`).
- `cargo build -p openwepp-runner --bins`
  - pass.
- `PYTHONPATH=. pytest -q tests/python/test_open_wepp_runner_api.py`
  - pass (`5 passed`).
- Full closeout gates executed in one chain:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace --quiet`
  - `cargo deny check`
  - all pass; `cargo deny check` reports non-fatal
    `license-not-encountered` allowlist warnings.
