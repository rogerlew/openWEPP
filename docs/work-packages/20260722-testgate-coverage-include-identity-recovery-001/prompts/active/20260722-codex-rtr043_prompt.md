# RTR-043 Coverage Include Identity Recovery Prompt

Scope: local repository engineering; flat-file reads/edits only; no external
connectivity or system action.

Execution mode: package-end-to-end.

Required reading:

Core: root/crate/work-package instructions, DC-ExecPlan authority, testing/gate
strategy, ADR-0021, and the package.

Conditional: none.

On-demand: the two authorized Rust files.

Required-reading budget: WARN; the binding governance and two focused source
surfaces are necessary. Map: `artifacts/required-reading-map.md`.

Task: close RTR-043 end-to-end inside the declared envelope. Do not hold while
the direct path-backed test-module correction and focused validation remain
available.

Subagent requirement: no HEAVY gate. This prompt explicitly authorizes subagent
spawning/delegation to dual read-only reviewers and verifiers; outputs are the
package review/verification artifacts.

Autonomy: continue through correction, evidence, review, durable closure, and
verification without requesting intervention unless a declared boundary blocks.

Do not push, deploy, switch branches, manually dispatch TESTGATE, or rerun
unchanged expensive gates.
