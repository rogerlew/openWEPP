# EROD18 Route Topology Implementation Report

Status: complete
Evidence mode: static+ran
Date: 2026-05-26

## Static
Implemented EROD18 runtime topology closure in:
- `crates/openwepp-hillslope-orchestrator/src/constants.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/02_guard_errors.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/erod14_wave2_multiofe_enrichment_kernel_contract.rs`

Key behavior implemented:
- Added EROD18 symbol constants for canonical route topology family.
- Added typed EROD18 guard variants and message IDs:
  - `HKERNEL-EROD18-ROUTE-E-001` missing required symbol
  - `HKERNEL-EROD18-ROUTE-E-002` non-finite symbol
  - `HKERNEL-EROD18-ROUTE-E-003` domain violation
- Added route topology publication path in hillslope closure diagnostics:
  - validates required ingress families,
  - publishes route seam state family (`mshear`, `xc1/xc2`, `du/dl`, `xdbeg/xdend`, `ldlast/lddend`, `ndep`).
- Added runner ingress projection for route-topology symbols on Wave-2 enabled runs.
- Updated runner provenance detection so EROD18 kernel success satisfies
  `erod14_wave2_kernel_status_seen` continuity checks.
- Added EROD18 integration tests for missing/non-finite/domain guard behavior.
- Enabled non-ignored seam test requiring core route publication family.

## Ran
- `cargo fmt --all`
- `cargo test -p openwepp --test erod14_wave2_multiofe_enrichment_kernel_contract`
- `cargo test -p openwepp --test erod14_wave2_multiofe_enrichment_kernel_contract -- --ignored --nocapture`
- `cargo test -p openwepp --test cli03_runner_contract_derived_tests cli03_mofe03 -- --nocapture`
