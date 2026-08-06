# Line-Count Governance

Status: PASS for the extraction increment.

Evidence mode: Ran on 2026-08-06.

| File | Baseline | Extracted | Disposition |
|---|---:|---:|---|
| `runoff_reconciliation.rs` | 3,177 | 1,091 | PASS |
| `stage3_solver.rs` | absent | 1,477 | PASS |
| `stage3_solver/evaluation.rs` | absent | 623 | PASS |

No touched or new Rust file reaches the 2,000-line warning threshold or the
3,000-line blocking threshold after extraction.
