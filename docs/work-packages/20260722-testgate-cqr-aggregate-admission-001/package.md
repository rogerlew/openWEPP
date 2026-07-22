# TESTGATE CQR Aggregate Admission Authority

Package ID: `20260722-testgate-cqr-aggregate-admission-001`

Queue ID: `TESTGATE-CQR-AGGREGATE-ADMISSION-01`

Status: `HOLD-PREREQUISITE-VALIDATOR`

## Objective

Close RTR-031 by establishing immutable aggregate closeout authority before the
final correction diff and by binding the CQR workflow to require that authority
before module execution. Then validate, dual-review, and durably close the
defect before one changed-head global TESTGATE qualification attempt.

## Failure

Static: the completed seven-package CQR ExecPlan changed 248 recovery paths
from the original recovery base, but its master plan was not a base-commit work
package with an immutable aggregate write set. The cheap final package audit
correctly rejected retroactive widening as `RETROACTIVE_WRITE_SET_WIDENING`.
No LIGHT or HEAVY gate started.

## Scope

In scope: CQR authoring/process/template controls for aggregate admission,
package-local evidence, recovery closeout evidence, and the completed master
ExecPlan record. Out of scope: production or science behavior, package-audit
weakening, retroactive authority, branch operations, deployment, workflow
dispatch, and direct HEAVY execution.

## Declared Write Set

- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/cqr-nightly-burndown-execplan.md`
- `docs/work-packages/templates/cqr-nightly-package.md`
- `docs/work-packages/20260721-cqr-testgate-recovery-closeout-execplan.md`
- `docs/work-packages/20260720-testgate-recovery-trust-001/**`
- `docs/work-packages/20260722-testgate-cqr-aggregate-admission-001/**`
- `docs/work-packages/README.md`

## Phase Plan

1. Commit this scaffold before the correction diff.
2. Add one normative aggregate-admission rule and matching process/template
   controls without weakening immutable base authority.
3. Run scoped documentation and package validation, obtain dual independent
   implementation review, and close RTR-031 durably at the exact correction
   commit.
4. Rebuild the release planner and delegate exactly one changed-head global
   qualification attempt through the recovery campaign comparator.

## Exit Criteria

- The package validates `READY` from its immutable scaffold base with zero
  unauthorized paths.
- CQR process guidance mechanically blocks module execution until an aggregate
  authority package exists when a master closeout will require one exact
  qualification diff.
- Dual independent implementation review passes and RTR-031 is durably closed.
- No unchanged expensive gate is rerun.

## Hold Legitimacy Audit

Static: review A proved the correction is prose-only and does not mechanically
validate the aggregate package path, status, base write set, coverage of planned
paths, or ordering before module implementation. An executable validator with
positive/negative tests is required. Those tool/test paths are outside this
package's immutable scaffold write set, and adding them now would reproduce the
retroactive-widening defect this package exists to prevent. The in-envelope
documentation route was implemented and reviewed but cannot close the defect.
`20260722-cqr-aggregate-admission-validator-001` owns the executable prerequisite.

## Security Impact Gate

- security_impact: high
- dedicated_security_review_required: no
- rationale: trust-bearing qualification admission receives dual independent
  review and the final global TESTGATE campaign gate.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent read-only implementation reviewers, one
comparator runner for the exact changed-head TESTGATE attempt, and two
independent read-only terminal verifiers. Expected outputs are package-local
review/verification evidence and retained external qualification artifacts.
Write access is read-only except for the comparator's ignored external evidence
root. No subagent may push, deploy, change branches, dispatch a workflow, or
rerun an unchanged expensive gate.
