# CLI04 Contract Test Implementation Evidence

Status: completed (Phase B)
Evidence mode: Static + Ran

## Static
- Added dedicated CLI04 contract-derived integration target and registration:
  - `tests/integration/cli04_runner_wat_parquet_contract_derived_tests.rs`
  - root `Cargo.toml` `[[test]]` entry:
    `cli04_runner_wat_parquet_contract_derived_tests`
- Added root test-time parquet schema inspection dependencies:
  - `parquet`
  - `arrow-schema`
- Updated CLI03 contract test heading assertion for compatibility with
  CLI03/CLI04 combined heading text:
  - `tests/integration/cli03_runner_contract_derived_tests.rs`
- Implemented CLI04 Phase B contract-derived coverage for:
  - contract/spec text assertions (shared boundary target, metadata keys,
    dependency posture),
  - fixture execution assertion requiring `outputs.wat` to be valid parquet,
  - required WAT dataset metadata keys:
    `dataset_version`, `dataset_version_major`,
    `dataset_version_minor`, `schema_version`,
  - required WAT field metadata parity checks for `P` and
    `InterceptionStorage` (`units`, `description`).

## Ran
- Command:
  - `cargo test --test cli03_runner_contract_derived_tests`
- Observed:
  - pass (`9 passed; 0 failed`).

- Command:
  - `cargo test --test cli04_runner_wat_parquet_contract_derived_tests`
- Observed:
  - fail (`1 passed; 1 failed`).
- Failing test:
  - `cli04_fixture_run_emits_valid_wat_parquet_with_required_metadata_keys`
- Failure signature:
  - `Parquet error: Invalid Parquet file. Corrupt footer`
- Interpretation:
  - current `outputs.wat` emission is still placeholder payload text, not real
    parquet, and does not satisfy CLI04 contract-derived parity gates.
