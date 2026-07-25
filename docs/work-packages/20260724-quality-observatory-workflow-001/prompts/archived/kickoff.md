# Execute Optional Forest1 Quality Observatory Workflow

Scope: local repository workflow/tooling engineering; flat-file reads/edits
only. Do not dispatch TESTGATE or QA during implementation.

Execution mode: package-end-to-end.

Required reading:

- Core: `AGENTS.md`, `docs/work-packages/AGENTS.md`, this package,
  `docs/work-packages/testgate-quality-observatory-roadmap.md`, ADR-0041, and
  `docs/standards/testing-and-gate-strategy.md`.
- Conditional: workflow and test instructions discovered for the declared
  write set; prompt wording guidance when prompts change.
- On-demand: current TESTGATE workflow, queue/recovery helpers, artifact
  packaging, and gate receipt schemas.

Required-reading budget: `148756` current local bytes, `OK`; map:
`artifacts/required-reading-map.md`. Recompute after Orders 1-3 land and before
implementation edits.

Task: implement and locally characterize the manual optional QA workflow,
typed TESTGATE-first deferral, and compact-artifact contract through final
disposition.

Subagent requirement: this prompt explicitly authorizes subagent
spawning/delegation to two read-only workflow/security reviewers and two
read-only verifiers; outputs are compact artifacts; write access is read-only.
No live or heavy run is selected in this package.

Autonomy: execute all phases without further user direction unless provider
behavior cannot be characterized locally.
