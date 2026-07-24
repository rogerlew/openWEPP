# Execute TESTGATE And Quality Authority Separation

Scope: local repository gate-policy engineering; flat-file reads/edits only;
no external connectivity or workflow dispatch is required.

Execution mode: package-end-to-end.

Required reading:

- Core: `AGENTS.md`, `docs/work-packages/AGENTS.md`, this package,
  `docs/work-packages/testgate-quality-observatory-roadmap.md`,
  `docs/decisions/0021-module-coverage-closure-thresholds.md`, and
  `docs/standards/testing-and-gate-strategy.md`.
- Conditional: `docs/standards/AGENTS.md` and
  `docs/standards/prompt-wording-guidance.md` for standards/prompts;
  nearest crate/test instructions for touched Rust/tests.
- On-demand: ADR-0039, ADR-0040, gate-policy schemas, planner implementation,
  TESTGATE workflow contracts, and CQR templates.

Required-reading budget: `170333` current local bytes, `OK`; map:
`artifacts/required-reading-map.md`. Recomputed after ADR-0041 and the aligned
authority edits existed.

Task: execute every package phase through disposition. Adopt and align the
normative authority and specify typed non-blocking deferral without weakening
correctness gates. Executable policy implementation and proof belong to Order
2.

Subagent requirement: this prompt explicitly authorizes subagent
spawning/delegation to two read-only reviewers and two read-only verifiers;
outputs are compact artifacts with exact paths and findings; write access is
read-only. No heavy closure run is selected by this scaffold; if terminal
planning selects one, spawn `comparator_suite_runner` and do not run it on the
parent model unless unavailable with recorded evidence.

Autonomy: execute end-to-end without requesting direction unless canonical
authority is contradictory after ADR-0041 is drafted.
