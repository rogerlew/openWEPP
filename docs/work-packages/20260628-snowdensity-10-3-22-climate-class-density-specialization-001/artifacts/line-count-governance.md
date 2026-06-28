# Line-Count Governance

Evidence mode: `Static`

Touched implementation/test files were checked after the 10.3.22 rerun edits:

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs` | 750 | OK |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/09_snow_density.rs` | 821 | OK |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs` | 1010 | OK |
| `crates/openwepp-hillslope-orchestrator/src/lib.rs` | 208 | OK |
| `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs` | 2753 | WARN: below 3000-line refactor threshold; no package-scope split performed. |
| `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs` | 593 | OK |
| `crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs` | 923 | OK |
| `tests/integration/snowdensity10_3_22_climate_class_density_specialization.rs` | 312 | OK |

No touched non-generated Rust file is at or above the 3000-line required
refactor threshold. The one warning-band file was already a direct-production
authority builder; this package kept its changes narrowly scoped to passing
Sturm class operands into the existing direct snow runtime surface.
