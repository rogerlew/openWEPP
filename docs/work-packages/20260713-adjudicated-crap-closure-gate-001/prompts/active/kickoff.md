# Adjudicated CRAP Closure Gate Kickoff

Scope: local openWEPP repository quality-gate engineering; flat-file reads and
edits only; no external connectivity or external-system actions required.

Execution mode: `package-end-to-end`.

Phase plan: execute every phase in `package.md` sequentially through final
disposition.

Required reading:

- Core: `AGENTS.md`, `docs/work-packages/AGENTS.md`,
  `docs/standards/AGENTS.md`, `docs/standards/prompt-wording-guidance.md`,
  `docs/decisions/0021-module-coverage-closure-thresholds.md`,
  `docs/standards/code-quality-refactor-authoring-guide.md`, and
  `docs/standards/module-test-enhancement-authoring-guide.md`.
- Conditional: `tests/AGENTS.md` before editing the focused Python tests.
- On-demand:
  `docs/work-packages/cqr-pre-integration-campaign-execution-contract.md` and
  `docs/work-packages/cqr-pre-integration-campaign-evidence/low/**` for exact
  filter, adjudication, and terminal-census evidence.

Required-reading budget: `109321` local bytes, `OK`; map:
`artifacts/required-reading-map.md`.

Files: only the intended write set in `package.md`.

Task: implement the adjudication-aware CRAP closure gate end-to-end. Preserve
the strict threshold and the CQR campaign's raw/actionable distinction. Do not
create new exceptions or change Rust science behavior.

Subagent requirement: REQUIRED. Spawn `comparator_suite_runner` for heavy live
CRAP and closure commands; do not run those commands on the parent when the
runner is available. This prompt explicitly authorizes subagent
spawning/delegation to the runner and two independent reviewer/verifier roles
for the scopes, outputs, and bounded write sets stated in `package.md`.

Autonomy: execute all phases and update artifacts without requesting additional
direction unless a hard blocker is proven.

Outputs: implementation, focused tests, governance integration, exact campaign
reproduction, heavy-run evidence, two independent reviews, finding disposition,
and final package disposition.
