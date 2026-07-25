# Execute CQR Nightly Quality Evidence Handoff

Scope: local repository CQR planning/tooling engineering; flat-file reads/edits
only; do not launch a CQR batch or external workflow.

Execution mode: package-end-to-end.

Required reading:

- Core: `AGENTS.md`, `docs/work-packages/AGENTS.md`, this package,
  `docs/work-packages/cqr-nightly-burndown-execplan.md`,
  `docs/work-packages/testgate-quality-observatory-roadmap.md`, and the adopted
  QA report contract.
- Conditional: code-quality and mechanical-refactor authoring guides when
  templates change; test instructions for touched integration tests.
- On-demand: CQR templates, aggregate-admission tooling, CRAP registry, and QA
  report verifier.

Required-reading budget: `62432` current local bytes, `OK`; map:
`artifacts/required-reading-map.md`. Recompute after the QA report contract
lands and before implementation edits.

Task: implement exact QA evidence intake, typed currency validation, no-
recollection selection, explicit stale/invalid fallback, and aligned CQR
documentation through final disposition.

Subagent requirement: this prompt explicitly authorizes subagent
spawning/delegation to two read-only evidence/selection reviewers and two
read-only verifiers; outputs are compact package artifacts; write access is
read-only. No heavy run is selected.

Autonomy: execute all phases without operator intervention unless the adopted
QA report contract lacks a required identity field.
