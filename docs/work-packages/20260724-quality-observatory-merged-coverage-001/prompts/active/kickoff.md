# Execute Quality Observatory Merged Coverage

Scope: local repository coverage/quality tooling engineering; flat-file
reads/edits only; no workflow dispatch is required for implementation.

Execution mode: package-end-to-end.

Required reading:

- Core: `AGENTS.md`, `docs/work-packages/AGENTS.md`, this package,
  `docs/work-packages/testgate-quality-observatory-roadmap.md`, ADR-0041,
  ADR-0021 as amended, and the coverage/CRAP section of
  `docs/standards/testing-and-gate-strategy.md`.
- Conditional: `tests/AGENTS.md` and crate instructions when tests are edited.
- On-demand: Nextest profiles, LLVM coverage tooling, adjudicated CRAP tooling,
  report schemas, and snowbench manual-science fixtures.

Required-reading budget: `164826` current local bytes, `OK`; map:
`artifacts/required-reading-map.md`. Recompute after Orders 1-2 land and before
implementation edits.

Task: implement identity-safe `full` plus `science-manual` coverage collection,
merge, snowbench proof, and merged-LCOV CRAP reporting through disposition.

Subagent requirement: REQUIRED for any heavy instrumented collection: spawn
`comparator_suite_runner`; do not run the heavy collection on the parent model
unless unavailable with command-level evidence. This prompt explicitly
authorizes subagent spawning/delegation to the heavy runner, two read-only
measurement reviewers, and two read-only verifiers; outputs are compact
metrics and log/artifact paths; write access is read-only.

Autonomy: execute all phases and correct in-scope measurement defects without
requesting operator direction.
