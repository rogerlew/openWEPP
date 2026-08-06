# Line-Count Governance

Status: `PASS with WARN dispositions`.

Evidence mode: `Ran: wc -l at scaffold commit 30e843d4116411520cf9eeb7f08a3bf1ce853b78`.

Files at or above 2,000 lines require WARN disposition and split intent;
nonexempt files at or above 3,000 block closure.

| File | Baseline lines | Prospective disposition |
| --- | ---: | --- |
| `openwepp-meteorology/src/surface_energy.rs` | 1,472 | Below WARN; additive wrapper only. |
| orchestrator `03_kernel_support_00_support_helpers.rs` | 1,137 | Below WARN; companion types only. |
| orchestrator `runoff_reconciliation.rs` | 1,233 | Below WARN. |
| orchestrator `stage3_solver.rs` | 1,797 | Below WARN; remain below 2,000 or record WARN/split intent. |
| orchestrator `stage3_solver/evaluation.rs` | 882 | Below WARN; tuple capture implementation. |
| orchestrator `lib.rs` | 197 | Below WARN; additive exports only. |
| runner `00c_day_input_builder_impl.rs` | 2,923 | `WARN`; no net growth permitted. Move schema selection out so terminal count is below 2,923 and cannot reach 3,000. |
| runner `00h_snow_stage3_evaluation_trace.rs` | 708 | Below WARN; v5 remains unchanged. |

New schema-v6 formatting belongs in a new included module. Terminal evidence
records every touched/new Rust file and blocks closure at 3,000 lines.

Current continuity-corrected implementation counts:

| File | Current lines | Disposition |
| --- | ---: | --- |
| orchestrator `stage3_solver.rs` | 2,406 | `WARN`; below the 3,000-line closure block. Retain the already-declared next mechanical extraction intent. |
| orchestrator `stage3_solver/evaluation.rs` | 1,354 | Below WARN. |
| runner `00c_day_input_builder_impl.rs` | 2,917 | `WARN`; six lines below scaffold and below the 3,000-line closure block. |

Additional touched production Rust files remain below `2,000` lines; the
largest are `surface_energy.rs` at `1,823`, `runoff_reconciliation.rs` at
`1,378`, and `03_kernel_support_00_support_helpers.rs` at `1,287`. No touched
or added `.rs` file reaches the `3,000`-line closure block. The `stage3_solver`
WARN remains mechanical extraction debt and does not conceal a current
correctness or authority failure.
