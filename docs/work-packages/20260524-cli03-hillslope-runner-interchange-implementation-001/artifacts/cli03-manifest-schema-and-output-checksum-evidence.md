# CLI03 Manifest Schema And Output Checksum Evidence

Status: completed
Evidence mode: Static + Ran

## Static
Manifest behavior is implemented and validated as follows:
- schema id remains:
  - `openwepp-hillslope-run-manifest-v1`
- manifest output checksum assembly now delegates to output crate boundary:
  - `openwepp-hillslope-output::manifest::assemble_output_checksums`
- output checksum coverage includes:
  - required pass/loss outputs,
  - configured optional parquet outputs.
- output checksum ordering is deterministic via key-sorted `BTreeMap` assembly.

Supporting code paths:
- `crates/openwepp-runner/src/lib.rs`
- `crates/openwepp-hillslope-output/src/manifest.rs`

Contract tests covering deterministic ordering and guard behavior:
- `manifest_checksum_assembly_sorts_paths_deterministically`
- `manifest_checksum_assembly_rejects_duplicate_paths`
- `manifest_checksum_assembly_rejects_empty_checksum`
- `manifest_checksum_assembly_rejects_empty_output_path`

## Ran
- Command:
  - `cargo test -p openwepp-hillslope-output`
- Observed:
  - pass (`11 passed; 0 failed`).

- Command:
  - `cargo test --test cli03_runner_contract_derived_tests`
- Observed:
  - pass (`9 passed; 0 failed`).
- Relevant manifest fixture assertion:
  - `cli03_fixture_run_emits_required_and_configured_optional_outputs_with_manifest_checksums`
