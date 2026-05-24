# CLI03 Output Crate Organization Evidence

Status: completed
Evidence mode: Static + Ran

## Static
Dedicated output crate is implemented and integrated:
- crate path:
  - `crates/openwepp-hillslope-output/`

Crate module organization:
- `src/contracts.rs`
  - typed hillslope output configuration (`pass`, `loss`, optional parquet outputs)
  - required/optional extension guards and typed contract errors
- `src/manifest.rs`
  - deterministic checksum map assembly
  - duplicate/empty guard enforcement
- `src/writers.rs`
  - required-output and optional-output path projection

Runner wiring to output crate is implemented:
- `crates/openwepp-runner/Cargo.toml` includes `openwepp-hillslope-output` dependency.
- `crates/openwepp-runner/src/lib.rs` imports and uses output crate APIs for:
  - output contract validation,
  - required/optional output path mapping,
  - manifest checksum map assembly.

Integration assertion for runner wiring:
- `cli03_runner_crate_wires_output_surface_dependency`

## Ran
- Command:
  - `cargo test -p openwepp-hillslope-output`
- Observed:
  - pass (`11 passed; 0 failed`).

- Command:
  - `cargo test --test cli03_runner_contract_derived_tests`
- Observed:
  - pass (`9 passed; 0 failed`).
