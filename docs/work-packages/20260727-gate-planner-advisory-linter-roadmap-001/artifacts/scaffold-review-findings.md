# Scaffold Review Findings

Evidence class: `Static`.

Three independent read-only reviews assessed the initial roadmap and Order-0
package scaffold. All three returned `HOLD` for scaffold closure only. None
placed CAL-04B, direct modeling, or manual validation on hold.

## Philosophy And Authority Review

- `PHIL-001` — fallback wording made nonblocking behavior conditional on an
  agent already knowing the requirements.
- `PHIL-002` — `tools/local_ci/gate-plan` preserved the rejected CI/gate
  identity.
- `PHIL-003` — `ERROR` findings and unspecified exit codes could become an
  indirect permission signal.
- `PHIL-004` — bounded repository-inspection subprocesses lacked a closed
  read-only allowlist.
- `PHIL-005` — “validate declarations” implied admission authority.
- `PHIL-006` — roadmap sequencing and stop-loss language did not name their
  linter-only scope.
- `PHIL-007` — requirements, proof, and disposition authority required sharper
  separation.

## Operator And Failure-Semantics Review

- `OPER-001` — the prospective redirect conflicted with historical TESTGATE
  authority still presented as current.
- `OPER-002` — the one-command contract did not define pre-edit, dirty-tree,
  and terminal observation bases.
- `OPER-003` — complete, partial, unavailable, finding, and exit semantics were
  not closed.
- `OPER-004` — the manual route was promised but not executable.
- `OPER-005` — finding severity, confidence, category, and action were mixed.
- `OPER-006` — the product name preserved the old conceptual model.
- `OPER-007` — value and deletion criteria lacked a cohort and numeric
  thresholds.

## Governance, Science, And Protected-Boundary Review

- `GOV-001` — `docs/ROADMAP.md` simultaneously made the redirect and TESTGATE
  authority prospective.
- `GOV-002` — Order 0 wrote operative guidance while Order 1 separately owned
  the same cutover.
- `GOV-003` — ADR supersession targets were not clause-specific.
- `SCI-001` — nonblocking work needed separation from truthful closure with an
  unmet governing requirement.
- `HARV-001` — the deletion plan did not bind the exact CAL freeze, verifier,
  open-once, crash, digest, lock, and negative-write invariants.
- `ROAD-001` — CAL sequencing could still be read as waiting on roadmap work.
- `SCI-002` — required reading omitted ADR-0042, science authority, and exact
  CAL protected-boundary sources.
- `PLAN-001` — the ExecPlan lacked `Surprises And Discoveries` and timestamped
  progress.
- `VALUE-001` — the value contract lacked numeric acceptance.

