# Active Prompt: CQR TESTGATE Gate-Planner CLI

Execution mode: package-end-to-end.

Execute `20260721-cqr-testgate-recovery-02-main-001` within the sole production
module `crates/openwepp-gate-planner/src/main.rs`. Preserve option admission,
transition order, ledger lifecycle, canonical JSON, error codes, and exit
semantics while reducing every eligible target row to CRAP `<= 30`.

Autonomy: characterize, refactor, measure, review, verify, and complete or hold
the package. This prompt explicitly authorizes subagent spawning/delegation to
independent review, verification, and comparator/closure-runner subagents for
eligibility, CQR metrics, selected gates, behavior-identity review, and terminal
verification. Expected outputs are package-local review, verification, compact
metric, and command-evidence artifacts. Write access is read-only unless a
subagent is explicitly assigned a bounded change within the declared package
write set. Do not run campaign-global TESTGATE; the master seven-package
ExecPlan owns it.
