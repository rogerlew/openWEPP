# CLI03 Fixture Simulation Run And Interchange Output Evidence

Status: completed
Evidence mode: Static + Ran

## Static
Fixture integration evidence implemented in:
- `tests/integration/cli03_runner_contract_derived_tests.rs`

Fixture run assertions now verify:
- successful run with schema-versioned `.run` input,
- required output emission at configured `.run` paths:
  - `outputs.pass` (`.hbp`),
  - `outputs.loss` (`.json`),
- configured optional output emission at configured parquet paths,
- run manifest includes required schema id and output path coverage.

Primary fixture test:
- `cli03_fixture_run_emits_required_and_configured_optional_outputs_with_manifest_checksums`

## Ran
- Command:
  - `cargo test --test cli03_runner_contract_derived_tests`
- Observed:
  - pass (`9 passed; 0 failed`).
- Relevant passing fixture test:
  - `cli03_fixture_run_emits_required_and_configured_optional_outputs_with_manifest_checksums`
