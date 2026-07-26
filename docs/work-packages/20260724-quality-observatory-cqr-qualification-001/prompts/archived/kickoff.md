# Execute Quality Observatory And CQR Qualification

Scope: local repository qualification plus an initial authorized QA dispatch
and one changed-head attempt per in-scope correction on current `main`; heavy
execution remains forest1-only.

Execution mode: package-end-to-end.

Required reading:

- Core: `AGENTS.md`, `docs/work-packages/AGENTS.md`, this package,
  `docs/work-packages/testgate-quality-observatory-roadmap.md`, ADR-0041, the
  adopted QA report contract, and CQR Nightly ExecPlan.
- Conditional: nearest instructions for every corrected path.
- On-demand: QA workflow, occupancy helper, report validator, artifact
  publisher, CQR selector, and TESTGATE qualification evidence.

Required-reading budget: `64305` current local bytes, `OK`; map:
`artifacts/required-reading-map.md`. Recompute against the final Order-6
qualified head before QA dispatch.

Task: qualify QA after TESTGATE, verify merged snowbench-aware quality evidence,
and prove exact-report CQR intake without recollection. Retain and correct
in-scope failures; never rerun an unchanged failed attempt.

Subagent requirement: REQUIRED: spawn `comparator_suite_runner` for heavy
workflow monitoring/evidence collection. This prompt explicitly authorizes
subagent spawning/delegation to that runner, two read-only result reviewers,
and two read-only verifiers; outputs are compact metrics and log/artifact
paths; write access is read-only. Only the parent may commit, push, or dispatch.

Autonomy: execute through roadmap closeout. Ignore defunct Omarchy records;
never cancel or wait for them.
