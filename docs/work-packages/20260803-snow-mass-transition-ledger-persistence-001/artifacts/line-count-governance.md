# Line-Count Governance

Status: `queued`

Evidence mode: `Static scaffold`

Scaffold counts:

| File | Lines | Initial disposition |
|---|---:|---|
| `03_kernel_support_00_support_helpers.rs` | 931 | below WARN |
| `infiltration_reconciliation.rs` | 2392 | WARN; no unreviewed growth |
| `runoff_reconciliation.rs` | 2632 | WARN; extract ledger/capture seam |
| `00c_day_input_builder_impl.rs` | 2575 | WARN; extract trace/capture seam |
| `00f_snow_accumulation_melt_trace.rs` | 176 | below WARN |

Record all touched `.rs` pre/post counts, new-module counts, symbol movement,
and cohesion disposition. No nonexempt file may reach 3000 lines.
