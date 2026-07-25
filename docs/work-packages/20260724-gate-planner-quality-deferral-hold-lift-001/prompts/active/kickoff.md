# Kickoff: Gate-Planner Quality-Deferral Hold Lift

Scope: local repository engineering; flat-file reads/edits only in
`/home/workdir/openWEPP`; no external connectivity or system mutation.

Execution mode: package-end-to-end.

Phase plan: execute every phase in `package.md` through disposition.

Required reading:

- Core: `AGENTS.md`, `docs/work-packages/AGENTS.md`, this package,
  `artifacts/required-reading-map.md`,
  `20260724-quality-observatory-merged-coverage-001/artifacts/heavy-attempt-03-gate-planner-blocker.md`,
  `20260724-testgate-quality-deferral-001/package.md`, ADR-0041, and the
  selected testing/gate-strategy sections.
- Conditional: `crates/AGENTS.md` when editing the gate-planner crate.
- On-demand: exact gate-policy schemas and executor/planner/pre-heavy/verifier
  fixture modules named by the seven retained failures.

Required-reading budget: recorded in
`artifacts/required-reading-map.md`; disposition `OK`.

Task: close defect `QOBS-HOLD-LIFT-01` end-to-end, then lift and execute
`20260724-quality-observatory-merged-coverage-001`.

Constraints: preserve `DEFERRED_TO_QUALITY_CI`; do not restore retired quality
nodes or weaken schema, committed-checkout, source-mutation, receipt, or
verifier fail-closed behavior. Do not HOLD while source reading, implementation,
or validation remains possible inside the declared envelope.

Subagent requirement: REQUIRED. This prompt explicitly authorizes subagent
spawning/delegation to two read-only implementation/security reviewers,
`comparator_suite_runner` for all heavy/full-workspace and quality-observatory
runs, and two read-only terminal verifiers. Outputs: compact findings, metrics,
evidence IDs, and log paths. Write access: read-only.

Autonomy: execute both packages end-to-end without requesting user direction
unless a declared hard boundary is proven.

Outputs: maintain all living-plan sections, review/disposition artifacts,
hold-lift evidence, terminal verification, and exact final reconciliation.
