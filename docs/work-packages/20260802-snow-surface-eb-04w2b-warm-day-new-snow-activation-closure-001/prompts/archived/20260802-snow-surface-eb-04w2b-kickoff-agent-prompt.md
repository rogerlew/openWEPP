# EB-04W2B Kickoff

Close defects `SNOW-SURFACE-EB-04W2B-D001` and `D002` end-to-end under the
Correction Authority Envelope in `package.md`.

Autonomy: execute the full contract-first plan through disposition without
requesting intervention unless an external authority boundary makes closure
impossible.

Required reading:

- Core: `/workdir/openWEPP/AGENTS.md`, `docs/codex_exec_plans.md`,
  `docs/work-packages/AGENTS.md`, `docs/work-packages/README.md`, `package.md`.
- Conditional: `docs/defect_closure_execplans.md`,
  `docs/specifications/science-contract-authoring-procedure.md`,
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`,
  `docs/specifications/science-contracts/index.md`.
- On-demand: `SC-SNOWFREEZE-001`, `SC-RUNOFFPART-001`, W2A evidence, pinned
  `winter.for`/`stmtim.for`, and affected source/tests.

Required-reading size disposition is recorded in
`artifacts/required-reading-map.md`.

Sequence: contracts, failing contract-derived tests, pre-implementation gate,
then production edits. No kernel edit may precede that gate.

Conservation/output acceptance: bind independent operands and units, reject
self-restating accumulation formulas and zero-output aliases, use differentiating
warm-snow/mixed/rain-only fixtures, reconstruct the real consumer, and prove
closure at the canonical tolerance.

No surrogate physics: do not add heuristic or provisional process equations.

Real consumer proof: prove direct runtime and snowbench use the corrected shared
API and that no wrapper/shadow path carries the claim.

HOLD legitimacy audit: name and prove the external boundary, record attempted
in-envelope routes, and explain why none can close. Investigation effort alone
is not a valid hold.

Subagent authorization: two independent Rust reviewers and two independent
terminal verifiers may be spawned; each may write only its named artifact.
