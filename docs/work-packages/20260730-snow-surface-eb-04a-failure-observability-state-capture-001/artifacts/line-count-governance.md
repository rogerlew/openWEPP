# Line-Count Governance

Static:

| Rust file | Lines | Disposition |
| --- | ---: | --- |
| `02_guard_errors.rs` | 725 | below warning threshold |
| `03_kernel_support_00_support_helpers.rs` | 804 | below warning threshold |
| `runoff_reconciliation.rs` | 2,229 | WARN; pre-existing cohesive included module, below mandatory 3,000-line refactor threshold |
| `00c_day_input_builder_impl.rs` | 2,369 | WARN; pre-existing cohesive included module, below mandatory 3,000-line refactor threshold |

04A adds bounded typed error and diagnostic fields; it does not materially
expand either warned module’s responsibility.
