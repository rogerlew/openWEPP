# Line-Count Governance

Status: `complete / WARN accepted`

Evidence mode: `Static`

Static: terminal production-file line counts are:

| File | Before | After | Disposition |
|---|---:|---:|---|
| `03_kernel_support_00_support_helpers.rs` | 616 | 728 | OK |
| `infiltration_reconciliation.rs` | 2,033 | 2,074 | WARN, pre-existing 2,000+ file |
| `runoff_reconciliation.rs` | 1,232 | 1,544 | OK |
| `06_simimpl28_hourly_forcing.rs` | 1,289 | 1,319 | OK |
| `error.rs` | 79 | 100 | OK |
| `surface_energy.rs` | 1,191 | 1,391 | OK |
| `00_builders_and_authority.rs` | 2,844 | 2,848 | WARN, pre-existing 2,000+ file |
| `00a_snow_frost_authority_impl.rs` | 685 | 696 | OK |
| `00c_day_input_builder_impl.rs` | 2,067 | 2,121 | WARN, pre-existing 2,000+ file |
| `snowbench_coe_melt.rs` | 1,123 | 1,125 | OK |

No touched Rust file reaches the mandatory-refactor `3,000`-line threshold.
The three warnings are accepted because the EB-03 additions are narrow typed
carrier/parser changes within existing file boundaries; no new cross-domain
responsibility was added to those files.
