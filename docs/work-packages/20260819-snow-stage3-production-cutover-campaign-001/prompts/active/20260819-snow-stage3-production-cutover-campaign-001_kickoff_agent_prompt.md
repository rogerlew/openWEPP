# Stage 3 production-cutover campaign coordinator

Scope: local repository campaign coordination; flat-file reads/edits only; no
external connectivity or deployment action.

Execution mode: package-end-to-end, with each child retaining its independent
package boundary and kickoff.

Read `AGENTS.md`, `docs/codex_exec_plans.md`,
`docs/work-packages/AGENTS.md`, `docs/work-packages/README.md`, and
`docs/work-packages/20260819-snow-stage3-production-cutover-campaign-001/package.md`.

Task: maintain the child-release matrix and execute the ordered campaign
without transferring acceptance between children. Start with
`20260819-snow-stage3-terminal-meltout-lse-handoff-implementation-001`.
Keep CoE production-authoritative until Child 4 changes ownership and retires
CoE generation atomically. Require final-candidate `ASSURE-06` independent
human review and approval between Children 3 and 4. Exclude canopy-intercepted
snow.

Subagent requirement: follow each child package. This prompt explicitly
authorizes subagent spawning/delegation to the science, ownership, Rust,
assurance, release, comparator, and terminal-verifier roles named by each
child; outputs are compact findings/metrics and artifact paths; tracked-file
write access remains with the primary executor unless a child grants a bounded
write set.

Autonomy: progress through every currently released child phase and update
campaign artifacts without requesting direction unless a declared hard blocker
is reached. Never represent a deferred, blocked, failed, or not-run child gate
as passed.
