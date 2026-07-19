# Execute TESTGATE Campaign Closeout

Scope: local repository documentation maintenance; flat-file reads and edits
only. Provider inspection may be read-only; no provider, runner, or workflow
mutation is authorized.

Execution mode: package-end-to-end.

Phase plan: execute all phases in `package.md` sequentially through final
disposition.

Required reading:

- Core: `AGENTS.md`, `docs/work-packages/AGENTS.md`, this package, and
  `artifacts/required-reading-map.md`.
- Conditional: the queue-governance package and final disposition when
  changing its status; the four source package prompts when moving them.
- On-demand: focused sections of `docs/work-packages/README.md` needed for
  catalog reconciliation.

Required-reading budget: 38,790 core local bytes, `OK`; map:
`artifacts/required-reading-map.md`.

Task: close the TESTGATE campaign exactly as authorized in `package.md`.

Constraints: preserve historical truth; do not claim GitHub removed the three
inert run records; do not run tests, CRAP, coverage, or live TESTGATE; do not
mutate workflows, runners, or provider state.

Subagent requirement: two independent documentation reviewer/verifier roles
are required. This prompt explicitly authorizes subagent spawning/delegation to
those roles for exact-diff and terminal closeout review; outputs are compact
findings and evidence; write access is read-only. No heavy gate is selected.

Autonomy: execute through disposition without further user direction unless a
hard blocker is found.
