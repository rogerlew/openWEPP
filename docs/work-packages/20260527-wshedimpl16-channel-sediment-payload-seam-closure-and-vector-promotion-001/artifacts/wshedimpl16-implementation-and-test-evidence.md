# WSHEDIMPL16 Implementation and Test Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- Production/runtime seam edits:
  - `crates/openwepp-input-contract/src/parsers/hbp.rs`
    - Preserved `particle_diameter_m` metadata in parse result and latest-event
      payload surfaces.
    - Added finite/positive parse-domain validation for particle diameters.
  - `crates/openwepp-kernel-contract/src/lib.rs`
    - Added typed watershed symbol:
      `HillslopeContributorParticleDiameterMeters`.
  - `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
    - Projected contributor particle-diameter payload symbols into runtime
      state surface.
  - `crates/openwepp-watershed-orchestrator/src/lib.rs`
    - Extended WS10 contributor sediment guard path to require finite positive
      class-indexed particle diameters.
  - `crates/openwepp-sim-contract/src/symbols.rs`
    - Added canonical alias templates for `particle_diameter_m`.

## Ran
- `cargo fmt --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `cargo deny check` passed with non-failing duplicate/license-not-encountered
  warnings already present in policy output.
