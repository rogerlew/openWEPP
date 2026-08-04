# Line-Count Governance

Status: `PASS with three WARN files`

Evidence mode: `Ran: wc -l + Clippy`

| Rust file | Before | After | Delta | Disposition |
|---|---:|---:|---:|---|
| `03_kernel_support_00_support_helpers.rs` | 874 | 931 | +57 | `PASS`, below 2000. |
| `infiltration_reconciliation.rs` | 2353 | 2392 | +39 | `WARN`, below 3000; diagnostic extraction keeps modified orchestration below Clippy's function-length gate. |
| `runoff_reconciliation.rs` | 2598 | 2632 | +34 | `WARN`, below 3000; additive projection only. |
| `00c_day_input_builder_impl.rs` | 2450 | 2575 | +125 | `WARN`, below 3000; new hourly helper isolates schema-v4 formatting and passes Clippy. |
| `00f_snow_accumulation_melt_trace.rs` | 111 | 176 | +65 | `PASS`, below 2000. |
| EB-04W integration target | 217 | 324 | +107 | `PASS`, below 2000. |

No touched Rust file reaches the 3000-line mandatory-refactor threshold.
`cargo clippy --workspace --all-targets -- -D warnings` passes; no new
`too_many_lines` allowance or lint suppression was introduced.
