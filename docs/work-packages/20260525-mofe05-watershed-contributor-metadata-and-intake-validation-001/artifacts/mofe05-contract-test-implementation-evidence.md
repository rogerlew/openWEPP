# MOFE05 Contract-Test Implementation Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
Implemented MOFE05 contract-derived tests in:
- `tests/integration/mofe05_watershed_contributor_metadata_contract_authority_closure_contract.rs`
- `tests/integration/cli03_runner_contract_derived_tests.rs`
  - updated watershed source-surface assertions to include `manifest_file`.
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
  - added MOFE05 missing/malformed/mismatch/acceptance behavior vectors.
- `Cargo.toml`
  - registered `mofe05_watershed_contributor_metadata_contract_authority_closure_contract` integration target.

## Ran
- `cargo test -p openwepp --test mofe05_watershed_contributor_metadata_contract_authority_closure_contract -- --nocapture`
- `cargo test -p openwepp --test cli03_runner_contract_derived_tests cli03_watershed_cli_surface_uses_runfile_pattern_with_legacy_discovery_flag -- --nocapture`
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract mofe05 -- --nocapture`
