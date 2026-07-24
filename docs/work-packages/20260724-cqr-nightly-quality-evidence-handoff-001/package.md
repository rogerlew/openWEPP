# CQR Nightly Quality Evidence Handoff

Package ID: `20260724-cqr-nightly-quality-evidence-handoff-001`

Status: `QUEUED / ORDER-5`

## Objective

Make operator-directed CQR Nightly consume the exact current quality-observatory
report for target selection without recollecting workspace coverage unless that
evidence is stale, malformed, incomplete, or identity-incompatible.

## Included Scope

- QA report locator/identity input for CQR Nightly.
- Report verification, staleness rules, actionable module ranking, and retained
  selection provenance.
- Explicit fallback to fresh collection only after a typed stale/invalid
  disposition.
- CQR ExecPlan, templates, admission tooling, tests, and operator docs.

## Excluded Scope

- Automatic CQR execution after every QA run.
- Module refactors or science changes.
- Treating observational QA debt as ordinary package closure failure.
- Reusing a report across different source, policy, registry, profile, or
  toolchain identities.

## Declared Write Set

- `docs/ROADMAP.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/cqr-nightly-burndown-execplan.md`
- `docs/work-packages/templates/cqr-nightly-*`
- `docs/work-packages/20260724-cqr-nightly-quality-evidence-handoff-001/**`
- `docs/work-packages/README.md`
- `tools/local_ci/**`
- `tools/release/**`
- `tests/integration/cqr_*`

## Dependencies

- Orders 3 and 4 complete with a stable report schema and workflow identity.

## Phase Plan

1. Define exact current/stale/invalid report predicates.
2. Add operator input and fail-closed report verification.
3. Derive raw and actionable CQR target selection from verified evidence.
4. Permit recollection only after recording why the supplied evidence cannot
   be consumed.
5. Update ExecPlan/templates and prove exact aggregate/module admission.
6. Reconcile, review, verify, and disposition.

## Exit Criteria

- A valid exact-head QA report selects the same actionable modules as direct
  parsing of its retained CRAP data and launches no coverage collection.
- Selection independently reconstructs registry adjudication, filtering,
  deduplication, and ranking from compact raw/adjudicated/actionable row data;
  report summary counts alone are insufficient.
- Head, source manifest, registry, policy, workflow, toolchain, profile set, or
  artifact digest mismatch is typed `STALE` or `INVALID` and cannot select
  targets.
- Recollection requires an explicit operator CQR directive and a retained
  stale/invalid reason; absence of a report is not silently called current.
- Existing one-module packages, aggregate admission, dual selection reviews,
  and CQR package-local gates remain binding.
- CQR docs state that QA is deferred/operator-directed and non-blocking for
  ordinary science closure.
- Focused parser/selection/admission tests, documentation checks, dual review,
  and dual verification pass.

## Security Impact

Report content is untrusted input until every bound digest and identity is
verified. Locator text is never authority.

## Delegation

Subagent authorization: this package explicitly authorizes spawning/delegating
to two read-only evidence/selection reviewers and two read-only terminal
verifiers; expected outputs are compact package artifacts; write access is
read-only.
