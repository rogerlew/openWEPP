# Execute TESTGATE-CI-01 Four-Blocker Hold Lift

Scope: local repository gate-engineering task; flat-file reads/edits and local
tool execution only; no external connectivity or provider changes.

Execution mode: package-end-to-end (default).

Phase plan: execute every phase in `package.md` sequentially through truthful
disposition.

Required reading:

- Core: root `AGENTS.md`, `docs/work-packages/AGENTS.md`,
  `docs/standards/testing-and-gate-strategy.md` sections 8-10, 12, and 14-19,
  this package, and `artifacts/required-reading-map.md`.
- Conditional: `crates/AGENTS.md` for Rust edits and `tests/AGENTS.md` for test
  edits.
- On-demand: prior package review/disposition artifacts and adjacent schema,
  verifier, workflow, and release-adapter source.

Required-reading budget: 173,604 bytes for the conservative full core and prior
review set, `WARN`; the authority map identifies section-scoped reading used to
avoid irrelevant context.

Files: use only the declared write set in `package.md`.

Task: close all four accepted blockers end-to-end. Preserve nonpass receipts,
execute the adversarial matrix, bind complete terminal-plan affected quality or
escalate globally, and keep subprocess evidence external.

Constraints: typed errors; no silent defaults; no shell evaluation of plan
content; no gate reduction; no protected-context or release-runner changes.

Subagent requirement: REQUIRED for the terminal heavy closure set. This prompt
explicitly authorizes subagent spawning/delegation to one closure runner, two
independent reviewers, and two terminal verifiers for the scopes in
`package.md`; outputs are compact metrics/findings and log paths; write access
is read-only except generated build/coverage output for the closure runner.

Autonomy: execute package phases end-to-end without requesting user direction
unless a hard external boundary is proven.

Outputs: keep package progress and artifacts current and record the final
disposition without overclaiming cutover.
