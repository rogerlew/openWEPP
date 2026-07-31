# Line-Count Governance

Evidence class: `Static`

| Rust file | Lines | Disposition |
| --- | ---: | --- |
| `03_kernel_support_00_support_helpers.rs` | 812 | below warning threshold |
| `runoff_reconciliation.rs` | 2,321 | WARN; pre-existing cohesive include, below mandatory 3,000-line threshold |
| `00c_day_input_builder_impl.rs` | 2,377 | WARN; pre-existing cohesive include, below mandatory 3,000-line threshold |

EB-04C adds two authoritative branches, four diagnostics, and associated validation
helpers. It does not introduce a new responsibility into either warned file.
