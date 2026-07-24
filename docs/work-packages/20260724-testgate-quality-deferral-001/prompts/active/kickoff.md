# Execute TESTGATE Quality Execution Deferral

Scope: local repository TESTGATE policy/planner engineering; flat-file
reads/edits only; do not dispatch workflows.

Execution mode: package-end-to-end.

Required reading:

- Core: `AGENTS.md`, `docs/work-packages/AGENTS.md`, this package, ADR-0041,
  `docs/work-packages/testgate-quality-observatory-roadmap.md`, and
  `docs/standards/testing-and-gate-strategy.md`.
- Conditional: crate/test instructions discovered for touched paths.
- On-demand: gate definitions/schemas, planner, executor, verifier, workflow
  contracts, and predecessor TESTGATE failure evidence.

Required-reading budget: `147236` current local bytes, `OK`; map:
`artifacts/required-reading-map.md`. Recompute after Order 1 lands and before
implementation edits.

Task: remove quality execution from ordinary TESTGATE, add independently
reconstructed `DEFERRED_TO_QUALITY_CI`, preserve every non-quality gate, and
execute the package through disposition.

Subagent requirement: this prompt explicitly authorizes subagent
spawning/delegation to two read-only implementation/security reviewers and two
read-only verifiers; outputs are compact package artifacts; write access is
read-only. If terminal planning selects heavy execution, spawn
`comparator_suite_runner` and do not run it on the parent model unless
unavailable with recorded evidence.

Autonomy: execute all phases without further direction unless ADR-0041 lacks a
machine-representable required field.
