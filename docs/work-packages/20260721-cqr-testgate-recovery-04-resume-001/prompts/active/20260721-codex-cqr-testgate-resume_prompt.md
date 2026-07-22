# Active Prompt: CQR TESTGATE Resume

Execution mode: package-end-to-end.

Execute `20260721-cqr-testgate-recovery-04-resume-001` within the sole production
module `crates/openwepp-gate-planner/src/resume.rs`. Preserve candidate
admission, attestation/checkpoint validation, error precedence, and recovery
behavior while reducing every eligible row to CRAP `<= 30`.

Autonomy: characterize, refactor, measure, review, verify, and complete or hold.
This prompt explicitly authorizes subagent spawning/delegation to independent
review, verification, and comparator/closure-runner subagents for eligibility,
metrics, gates, behavior review, and terminal verification. Outputs are
package-local evidence; write access is read-only unless explicitly assigned a
bounded declared-write-set change. Do not run campaign-global TESTGATE; the
master seven-package ExecPlan owns it.
