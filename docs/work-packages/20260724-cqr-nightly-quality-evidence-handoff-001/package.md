# CQR Nightly Quality Evidence Handoff

Package ID: `20260724-cqr-nightly-quality-evidence-handoff-001`

Status: `ACTIVE / CLOSEOUT / ORDER-5`

## Pre-Implementation Intent

Risk: `CRITICAL`.

This package consumes untrusted quality artifacts and controls whether CQR may
reuse evidence or recollect it. Implementation is limited to one local
fail-closed intake/selection tool, its focused contract tests, CQR operator
documentation/templates, and package evidence. The operator supplies:

1. the exact 11-file quality-observatory publication directory;
2. its separately retained complete control receipt containing the admission
   object; and
3. the expected `quality_evidence_id`.

The intake tool independently invokes the adopted Order-3 verifier, requires
the evidence subject to equal current repository HEAD/tree/source identities,
reconstructs the raw/adjudicated/actionable CRAP partitions from exact rows and
the current registry, and ranks production modules from reconstructed
actionable rows. It does not execute collection commands.

Recollection authorization is a separate command. It requires an explicit
operator CQR directive and a canonical retained intake receipt with disposition
`STALE` or `INVALID`; `CURRENT`, missing evidence, locator failure alone, or an
untyped reason cannot authorize reacquisition.

Selected increment gates are Python compilation and tool self-test, focused
intake/selection/recollection integration contracts, existing aggregate
admission contracts, Rustfmt, warnings-denied Clippy for touched Rust tests,
documentation lint, diff/write-set/prompt/line-count reconciliation, two
independent read-only reviews, and two independent read-only terminal
verifications. No quality collection, CQR batch, live workflow, or heavy gate
is selected.

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
- `docs/standards/testing-and-gate-strategy.md` (Order-5 CQR intake hold lift
  only)
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/cqr-nightly-burndown-execplan.md`
- `docs/work-packages/templates/cqr-nightly-*`
- `docs/work-packages/20260724-cqr-nightly-quality-evidence-handoff-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/testgate-quality-observatory-roadmap.md` (Order-5
  closeout routing only)
- `Cargo.toml` (exact Order-5 integration-test registration only)
- `gate-policy/v1/impact-map.json` (testing-strategy `policy_sha256`
  synchronization only)
- `tools/local_ci/**`
- `tools/release/**`
- `tests/integration/cqr_*`

## Dependencies

- Orders 3 and 4 complete with a stable report schema and workflow identity.
- Inherited baseline correction: Order 4 updated the canonical testing
  strategy during closeout without refreshing the impact-map
  `policy_sha256`. Order 5 must restore that exact digest binding before its
  selected existing TESTGATE contract can pass; no matcher or gate semantics
  may change.

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
