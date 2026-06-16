# CQR25 Implementation and Test Evidence

Status: complete.

Static: production implementation files:

- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
- `crates/openwepp-runner/src/hillslope/intake_lane_setup/mod.rs`

Static: implementation approach:

- Removed the CQR target `#[allow(clippy::too_many_lines)]` suppression from
  `execute_hillslope_run`.
- Split the long runner function into private data carriers and phase helpers.
- Kept the public entry point as a short orchestration pipeline.
- Preserved single parse of the optional HPhys0245 trace config by carrying the
  parsed config into output writing.
- Kept the persistent-lane versus single-lane scheduler lifecycle calls in one
  branch site to satisfy the MOFE01 source-shape contract.

Ran: pre-refactor characterization:

```text
cargo test cli01_contract_conformance_hillslope_run_emits_required_outputs_and_manifest
cargo test cli03_fixture_run_emits_required_and_configured_optional_outputs_with_manifest_checksums
cargo test simimpl04_contract_requires_runner_kernel_execution_provenance_manifest
```

Ran: post-refactor focused tests:

```text
cargo test cli01_contract_conformance_hillslope_run_emits_required_outputs_and_manifest
cargo test cli03_fixture_run_emits_required_and_configured_optional_outputs_with_manifest_checksums
cargo test simimpl04_contract_requires_runner_kernel_execution_provenance_manifest
cargo test -p openwepp --test mofe01_per_ofe_state_contract mofe01_mi_multiofe_runner_lifecycle_is_mutually_exclusive_with_single_ofe_aggregate_path
```

Ran: final required gates passed:

```text
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
```
