# Line-Count Governance

Status: PASS before review.

Evidence mode: Ran on 2026-08-06.

| File | Baseline | Final | Disposition |
|---|---:|---:|---|
| `runoff_reconciliation.rs` | 3,177 | 1,131 | PASS |
| `stage3_solver.rs` | absent | 1,567 | PASS |
| `stage3_solver/evaluation.rs` | absent | 800 | PASS |
| runner `00c_day_input_builder_impl.rs` | 2,652 | 2,912 | WARN, below blocker |
| runner `00h_snow_stage3_evaluation_trace.rs` | absent | 143 | PASS |

The runner builder remains above the 2,000-line warning threshold, so the new
schema-v5 formatter was extracted into its own include rather than leaving the
builder above 3,000. No touched or new Rust file reaches the 3,000-line
blocking threshold.
