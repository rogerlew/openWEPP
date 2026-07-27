# Acceptance Matrix

Status: `SCAFFOLD REVIEW REQUIRED`

Evidence class: `Static + Ran`

| ID | Obligation | Evidence |
|---|---|---|
| AC-01 | Exact canonical failure retained | ledger package `canonical-execution.md` |
| AC-02 | Function-scoped disposition only | exact implementation diff |
| AC-03 | Cohesive lifecycle rationale | adjacent source comment and review |
| AC-04 | Test behavior unchanged | focused assurance target |
| AC-05 | Workspace Clippy restored | strict workspace Clippy |
| AC-06 | Full regression restored | full Nextest and doc tests |
| AC-07 | Independent closure | dual reviews, verifiers, canonical receipt |
| AC-08 | Findings explicitly dispositioned | `review-findings.md` |
| AC-09 | Worker/final handoff complete | `worker-handoff.md` and `final-disposition.md` |
| AC-10 | Rust line count governed | `line-count-disposition.md`; dual recount |
| AC-11 | Exact terminal diff reconciled | declared-base-to-head path/diff evidence |
| AC-12 | Independent reports retained | scaffold/implementation/terminal/receipt A+B artifacts |
