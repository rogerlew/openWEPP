# EROD19 Route Migration Implementation Report

Status: complete
Evidence mode: static+ran
Date: 2026-05-26

## Static
Implemented EROD19 runtime migration in:
- `crates/openwepp-hillslope-orchestrator/src/constants.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/erod14_wave2_multiofe_enrichment_kernel_contract.rs`

Key behavior:
- Added route near-zero threshold constant `EROD19_QOSTAR_NEAR_ZERO_THRESHOLD = 0.0011`.
- Replaced EROD18 placeholder route publication with EROD19 branch migration path:
  - `xcrit`-equivalent `mshear` classification (`1..5`),
  - `depc` and `depend`-style `xdend` solve,
  - upper-boundary `dl` branch based on `qostar` threshold,
  - branch-family publication of `mshear`, `xc1/xc2`, `du/dl`, `xdbeg/xdend`, `ndep`, `ldlast/lddend`.
- Added success status `HKERNEL-EROD19-ROUTE-OK-001` in peak-runoff route coupling path.
- Updated runner provenance continuity detection to recognize `EROD19-ROUTE` status.
- Activated EROD17 route vectors (removed `#[ignore]`) and tuned vectors to avoid upstream guard-confounded failures while exercising route branches.

## Ran
- `cargo fmt --all`
- `cargo clippy -p openwepp-hillslope-orchestrator -p openwepp-runner --all-targets -- -D warnings`
- `cargo test -p openwepp --test erod14_wave2_multiofe_enrichment_kernel_contract`
- `cargo test -p openwepp --test cli03_runner_contract_derived_tests cli03_mofe03 -- --nocapture`
