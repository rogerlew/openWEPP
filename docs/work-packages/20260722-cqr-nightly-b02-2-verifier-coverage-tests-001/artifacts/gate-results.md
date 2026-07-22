# Gate Results

| Gate | Status | Evidence |
| --- | --- | --- |
| Aggregate admission | PASS | Retained JSON; scaffold `1a42f2dc`. |
| Direct characterization | PASS | 1/1; run `37ce16ae`. |
| Real consumer | PASS | 1/1; run `e3ce5d89`, 209.652s. |
| Focused full-source LLVM coverage | PASS | Exact terminal source/FNDA; SHA `3f77c88b...`. |
| Focused cargo-crap | PASS | Target 5, helpers 2; SHA `df93664b...`. |
| `cargo fmt --all -- --check` | PASS | Exit 0. |
| Planner all-target warnings-denied Clippy | PASS | Exit 0. |
| Production-filter unit | PASS | 1 passed in 0.035s. |
| `git diff --check` | PASS | Exit 0. |
| Line-count governance | PASS | 490 lines. |
| Batch TESTGATE | DEFERRED-BY-PLAN | Master ExecPlan owns one changed-head run after both modules. |

Ran: one initial exact test used the wrong module path and ran zero tests; one
consumer filter expression also ran zero tests. Neither is claimed as evidence.
Corrected exact/direct and consumer commands passed as recorded. Default-filter
LCOV omitted test sources; the supported full-source reporting control produced
the passing retained evidence. No HEAVY gate was run by the parent.
