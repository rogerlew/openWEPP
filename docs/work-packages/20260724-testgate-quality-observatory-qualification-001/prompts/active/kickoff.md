# Execute TESTGATE Changed-Head Qualification

Scope: local repository workflow qualification plus authorized TESTGATE
dispatch on current `main`; heavy execution remains forest1-only.

Execution mode: package-end-to-end.

Required reading:

- Core: `AGENTS.md`, `docs/work-packages/AGENTS.md`, this package,
  `docs/work-packages/testgate-quality-observatory-roadmap.md`, ADR-0041, and
  `docs/standards/testing-and-gate-strategy.md`.
- Conditional: package-local authority maps and nearest instructions for every
  corrected path.
- On-demand: TESTGATE workflow, queue/recovery helpers, verifier, and
  predecessor failure evidence.

Required-reading budget: `147370` current local bytes, `OK`; map:
`artifacts/required-reading-map.md`. Recompute against the final Orders 1-5
head before qualification work.

Task: run cheap gates first and qualify changed-head TESTGATE without quality
execution. Iteratively diagnose and correct in-scope defects; retain every
failed attempt; do not rerun unchanged failures.

Subagent requirement: REQUIRED: spawn `comparator_suite_runner` for all heavy
workflow monitoring/evidence collection and do not execute heavy suites on the
parent model unless unavailable with command-level evidence. This prompt
explicitly authorizes subagent spawning/delegation to that runner, two
read-only result reviewers, and two read-only terminal verifiers; outputs are
compact metrics and log/artifact paths; write access is read-only. Only the
parent may commit, push, or dispatch.

Autonomy: execute through final disposition. Ignore defunct Omarchy queued
records; never cancel or wait for them.
