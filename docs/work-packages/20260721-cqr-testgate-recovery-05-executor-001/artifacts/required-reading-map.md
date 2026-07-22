# Required Reading Map

Static: all applicable required reading was completed before or during package
execution. The 347,441-byte budget is REQUIRES-JUSTIFICATION because this
package changes trust-bearing execution/receipt control flow and must read the
full gate standard and target source.

| Path | Tier | Bytes | Rationale | Applicability trigger | Status |
| --- | --- | ---: | --- | --- | --- |
| `AGENTS.md` | core | 11,317 | repository invariants | all work | READ |
| `crates/AGENTS.md` | core | 5,165 | Rust crate rules | production Rust | READ |
| `docs/work-packages/AGENTS.md` | core | 22,837 | package closure governance | package execution | READ |
| `docs/work-packages/20260721-cqr-testgate-recovery-05-executor-001/package.md` | core | 2,962 | exact scope and gates | this package | READ |
| `docs/work-packages/cqr-nightly-burndown-execplan.md` | core | 14,531 | per-module CQR procedure | CQR execution | READ |
| `docs/standards/mechanical-refactor-authoring-guide.md` | core | 10,501 | behavior-preserving mechanics | helper extraction | READ |
| `docs/standards/code-quality-refactor-authoring-guide.md` | core | 11,450 | CRAP-specific requirements | complexity reduction | READ |
| `docs/decisions/0021-module-coverage-closure-thresholds.md` | core | 15,949 | coverage/floor/CRAP authority | characterization changes | READ |
| `docs/standards/AGENTS.md` | conditional | 3,652 | standards maintenance rules | standards referenced | READ |
| `docs/standards/testing-and-gate-strategy.md` | conditional | 94,688 | gate lifecycle and reuse | metric failure/correction | READ |
| `docs/standards/prompt-wording-guidance.md` | conditional | 10,221 | active prompt contract | prompt maintenance | READ |
| `docs/work-packages/20260721-cqr-testgate-recovery-closeout-execplan.md` | conditional | 4,254 | master queue boundary | rank handoff | READ |
| `crates/openwepp-gate-planner/src/executor.rs` | on-demand | 112,186 | target implementation | production review/edit | READ |
| `crates/openwepp-gate-planner/src/executor_coverage_tests.rs` | on-demand | 27,728 | behavior and coverage oracle | characterization/review | READ |

Static: `tools/agents/find-agents` resolved `AGENTS.md` plus
`crates/AGENTS.md` for both Rust paths and `AGENTS.md` plus
`docs/work-packages/AGENTS.md` for package artifacts.
