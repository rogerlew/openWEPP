# Execute TESTGATE-ASSURE-01

Scope: local repository assurance-planner task; flat-file reads/edits and local
tool execution only; no external connectivity, publication, or provider
changes.

Execution mode: package-end-to-end (default).

Phase plan: execute every phase in `package.md` sequentially through truthful
disposition.

Required reading:

- Core: root `AGENTS.md`, `docs/work-packages/AGENTS.md`, ADR-0039,
  `docs/standards/testing-and-gate-strategy.md` sections 8, 11, and 13, this
  package, and `artifacts/required-reading-map.md`.
- Conditional: `crates/AGENTS.md` for Rust edits and `tests/AGENTS.md` for test
  edits.
- On-demand: assurance v2 governance, catalog/schema, report dependency paths,
  and adjacent planner/verifier/schema source.

Files: use only the declared write set in `package.md`.

Task: add registry-wide exact and semantic assurance impact discovery,
deterministic campaign-head impact identities, and fail-closed transfer
currency without changing reports or historical evidence.

Constraints: typed errors; deterministic offline matching; complete registry
enumeration; no agent materiality decisions; no report rebuild, lifecycle
change, approval, public export, CI cutover, or gate reduction.

Subagent requirement: REQUIRED for the terminal heavy closure set. This prompt
explicitly authorizes subagent spawning/delegation to one closure runner, two
independent reviewers, and two terminal verifiers for the scopes in
`package.md`; outputs are compact metrics/findings and log paths; write access
is read-only except generated build/coverage output for the closure runner.

Autonomy: execute package phases end-to-end without requesting user direction
unless a hard external boundary is proven.

Outputs: keep package progress and artifacts current and record the final
disposition without overclaiming scientific refresh, publication, or cutover.
