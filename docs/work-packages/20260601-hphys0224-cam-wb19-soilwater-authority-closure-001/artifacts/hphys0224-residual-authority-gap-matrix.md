# HPHYS0224 Residual Authority Gap Matrix

Status: completed  
Evidence mode: Static + Ran

## Gap Matrix

| Gap ID | Description | Status | Evidence |
| --- | --- | --- | --- |
| `HP224-GAP-001` | WB19 lateral/drainage soil-water subtraction used silent post-subtraction flooring (`max(0.0)`) instead of typed over-withdrawal domain failure. | closed | Static: `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs` and `03_kernel_support_00_support_helpers.rs` now enforce `q/Qdd <= wb11_soil_water_before` and explicit subtraction path via `wb19_apply_soil_water_withdrawal`. |
| `HP224-GAP-002` | No blocking A3 suite covered WB19 realized-withdrawal soil-water cap law. | closed | Static: added suite + registry + fixture lock/provenance (`cas_l4_subhyd_withdrawal_soilwater_cap_001`). Ran: `cargo test --test hphys0224_wb19_withdrawal_soilwater_cap_contract` (pass after implementation). |
| `HP224-GAP-003` | Open residual families (`Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`, `ProfileFCStore`) required post-change readjudication. | open | Ran: `/tmp/hphys0224_20260601T054337Z/parity/reports/hillslope_semantic_summary.json` unchanged vs `/tmp/hphys0223_20260531T201410Z/parity/reports/hillslope_semantic_summary.json` (all monitored deltas = 0). |
| `HP224-GAP-004` | Package required full workspace gate revalidation after kernel/suite changes. | closed | Ran: `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo deny check`. |
