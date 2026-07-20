# Rust Line-Count Baseline

Evidence mode: Ran on 2026-07-20 with `wc -l` before implementation.

| File | Lines | Disposition |
| --- | ---: | --- |
| `crates/openwepp-gate-planner/src/planner.rs` | 2,975 | `WARN`; do not add audit implementation here beyond thin integration. |
| `crates/openwepp-gate-planner/src/executor.rs` | 2,611 | `WARN`; keep new lifecycle behavior in cohesive modules. |
| `crates/openwepp-gate-planner/src/verifier.rs` | 2,554 | `WARN`; keep audit verification in a cohesive module. |

All other planner Rust files are below 2,000 lines at scaffold time. No file is
at the 3,000-line refactor threshold. Execution must record decomposition
rationale for every touched warning file and must refactor before closure if a
nonexempt file reaches 3,000 lines.
