# Execute ASSURE-MAINT-01

Scope: local repository assurance-tooling task; flat-file reads, edits, and
local deterministic commands only; no external connectivity or external-system
mutation.

Execution mode: package-end-to-end.

Phase plan: execute every milestone in `package.md` sequentially through final
disposition. Do not stop after scaffolding, partial migration, or a fast-path
prototype while current-scope gates remain executable.

Required reading:

- Core: `AGENTS.md`, `docs/work-packages/AGENTS.md`, `crates/AGENTS.md`,
  `tests/AGENTS.md`, `docs/standards/AGENTS.md`, this package, its
  `artifacts/required-reading-map.md`,
  `docs/specifications/assurance-amendment-and-identity-workflow.md`,
  `docs/governance/scientific-assurance-v2-source-build-contract.md`,
  `docs/governance/scientific-assurance-dossier-lifecycle.md`,
  `docs/standards/scientific-model-evaluation-report.md`,
  `docs/standards/local-ci-gate-selection.md`,
  `docs/decisions/0038-manuscript-first-scientific-assurance-publication.md`,
  and `assurance/v2/README.md`.
- Conditional: `docs/codex_exec_plans.md` when changing package scope or
  milestones.
- On demand: existing ASSURE-04B/04C/04D and assurance-editorial package
  artifacts for planner, transaction, publication, timing, and CRAP evidence.

Required-reading budget: 156,239 bytes, `OK`; map:
`artifacts/required-reading-map.md`. Preserve its tiering and record any added
core reading before edits.

Files: only the declared write set in `package.md` plus ignored build/test
outputs. Amend the write set before any necessary source edit outside it.

Task: implement the mechanical amendment and generated-identity architecture
end to end. Remove manually maintained derived hashes from authored v2 sources,
introduce the acyclic identity/event graph, implement typed amendment,
lifecycle, migration, and recovery transactions plus the focused receipt
runner, migrate both reports without scientific change, and prove the
no-package/no-agent workflow and timing contract.

Constraints: preserve fail-closed confinement, atomic exchange, rollback,
committed-cleanup, lifecycle independence, review approval, zero-public-report,
and publication boundaries. Do not implement a generic adopt/sync/bless
command. Do not use agents or heuristics to classify arbitrary scientific
meaning. Do not implement arbitrary copyedit or scientific-patch fast paths. No
model science, kernel, result, release, export, vendoring, or
WEPPcloud change.

Subagent requirement: REQUIRED. This prompt explicitly authorizes subagent
spawning/delegation to two read-only implementation reviewers, one heavy-gate
runner, and two read-only terminal verifiers for the scopes and artifacts named
in `package.md`. The parent must delegate full workspace Nextest, deny, and
fresh adjudicated CRAP to the heavy-gate runner and must not duplicate those
runs unless unavailability is recorded with command-level evidence.

Autonomy: execute through disposition without requesting user direction unless
a hard authority or external dependency blocker is proven. Every current-scope
gate requires current evidence; do not defer it to another package.

Outputs: maintain every required artifact, record timings and identities
truthfully, disposition all findings, and leave the package closed only when
the fast amendment workflow is both safe and demonstrably fast.
