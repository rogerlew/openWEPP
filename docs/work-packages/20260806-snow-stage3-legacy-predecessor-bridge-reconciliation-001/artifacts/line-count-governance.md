# Line-Count Governance

Status: `prospectively frozen`.

Evidence mode: `Static`.

| Potential write | Baseline lines | Disposition |
| --- | ---: | --- |
| `00c_day_input_builder_impl.rs` | 2,917 | `WARN`; only 82 lines below blocking threshold; do not add diagnostics here |
| `00_builders_and_authority.rs` | 2,871 | `WARN`; only 128 lines below blocking threshold; do not add diagnostics here |
| `stage3_solver.rs` | 2,406 | `WARN`; keep any conditional observability extracted |
| `evaluation.rs` | 1,354 | below WARN threshold; preferred conditional evaluation-owned surface |

No Rust change is currently planned. Recount every terminally touched Rust file;
all 2,000+ files require explicit WARN disposition and any nonexempt file at
3,000+ blocks closure.
