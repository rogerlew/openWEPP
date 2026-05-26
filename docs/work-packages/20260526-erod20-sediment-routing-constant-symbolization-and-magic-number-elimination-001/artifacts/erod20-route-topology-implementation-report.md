# EROD20 Constant Symbolization Report

Status: complete
Evidence mode: static+ran
Date: 2026-05-26

## Static
Implemented EROD20 symbolization in:
- `crates/openwepp-hillslope-orchestrator/src/constants.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs`

Added constants:
- EROD14 family: case bounds, vector-capacity counts, attenuation floor, enrichment offset.
- EROD19 family: depc epsilon, shear floor, uniform sentinel, depend solver steps/tolerance/iterations, and `taucn` fallback scale.

Rewired call sites in EROD14/EROD19 production paths to remove direct literals.

## Ran
- `cargo fmt --all`
- `cargo clippy -p openwepp-hillslope-orchestrator -p openwepp-runner --all-targets -- -D warnings`
- `cargo test -p openwepp --test erod14_wave2_multiofe_enrichment_kernel_contract`
- `cargo test -p openwepp --test cli03_runner_contract_derived_tests cli03_mofe03 -- --nocapture`
