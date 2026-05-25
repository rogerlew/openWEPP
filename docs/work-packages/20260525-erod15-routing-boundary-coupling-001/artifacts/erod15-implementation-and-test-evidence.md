# Erod15 implementation and test evidence

Status: complete
Evidence mode: mixed

## Static
- Production implementation changes:
  - `crates/openwepp-hillslope-orchestrator/src/lib.rs`: emits Wave-3 HBP
    payload symbols from EROD14 Wave-2 path.
  - `crates/openwepp-kernel-contract/src/lib.rs`: adds typed watershed
    contributor sediment payload symbols.
  - `crates/openwepp-watershed-orchestrator/src/lib.rs`: validates contributor
    Wave-3 payload completeness/domain before WS10 routing proceeds.
  - `crates/openwepp-sim-contract/src/symbols.rs`: adds alias registry
    mappings for Wave-3 payload families.
  - `crates/openwepp-input-contract/src/parsers/hbp.rs`: adds typed extraction
    of latest EVENT payload fields required by watershed contributor handoff.
  - `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`: adds
    `openwepp-cli-watershed` executable path, seeds contributor runtime symbols
    from HBP payload extraction, and resolves relative outputs against
    `--output-dir`.
  - `crates/openwepp-watershed-output/`: adds watershed output contract +
    writer surface and enforces typed hard-fail guard for placeholder emission
    (`OWSOUT-E-004`).
  - `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`: adds
    behavior-level CLI execution tests for WS10 domain guard and writer guard
    paths.

## Ran
- `cargo test -p openwepp-watershed-output` -> PASS.
- `cargo test --test cli03_runner_contract_derived_tests` -> PASS.
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract` -> PASS.
- `cargo clippy -p openwepp-watershed-output -p openwepp-runner --bin openwepp-cli-watershed -- -D warnings` -> PASS.
- `cargo fmt --check` -> PASS.
- `cargo clippy --workspace --all-targets -- -D warnings` -> PASS.
- `cargo test --workspace` -> PASS.
- `cargo deny check` -> PASS (warnings only; no failing policy classes).
