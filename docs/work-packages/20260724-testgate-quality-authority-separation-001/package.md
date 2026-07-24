# TESTGATE And Quality Authority Separation

Package ID: `20260724-testgate-quality-authority-separation-001`

Status: `ACTIVE / ORDER-1`

## Objective

Adopt ADR-0041 and align canonical governance so TESTGATE remains the blocking
increment correctness gate while workspace coverage and adjudicated CRAP become
optional, observational, non-blocking quality evidence. This package changes
authority and documentation only; executable TESTGATE decoupling is Order 2.

## Included Scope

- ADR-0041, the decision index, testing/gate strategy, gate-policy governance,
  package governance, CQR guidance, and affected prompt/templates.
- Normative TESTGATE plan and receipt semantics for
  `DEFERRED_TO_QUALITY_CI`.
- Explicit retention of correctness gates and metric obligations for packages
  whose declared objective is test enhancement or CQR.
- Disposition of conflicting acceptance language in the active coverage
  reconstruction and science-gate proportionality packages.

## Excluded Scope

- Planner, executor, verifier, schema, or workflow implementation.
- QA workflow implementation.
- Coverage-profile merging.
- Live TESTGATE or QA dispatch.
- Changes to CRAP threshold math or symbol eligibility taxonomy.

## Declared Write Set

- `AGENTS.md`
- `docs/ROADMAP.md`
- `docs/decisions/README.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/decisions/0039-campaign-scoped-risk-based-testing-and-assurance-gates.md`
- `docs/decisions/0040-accelerated-testgate-cutover-on-trusted-self-hosted-runner.md`
- `docs/decisions/0041-separate-testgate-from-observational-quality-ci.md`
- `docs/standards/testing-and-gate-strategy.md`
- `docs/standards/README.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/standards/local-ci-gate-selection.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/module-test-enhancement-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/standards/rust-scientific-coding-standard.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/cqr-nightly-burndown-execplan.md`
- `docs/work-packages/templates/**`
- `docs/work-packages/20260724-cqr-testgate-coverage-reconstruction-001/**`
- `docs/work-packages/20260724-testgate-science-gate-proportionality-001/**`
- `docs/work-packages/20260720-testgate-workflow-qualify-001/**`
- `docs/work-packages/20260723-testgate-incompatible-recovery-receipt-001/**`
- `docs/work-packages/20260724-testgate-quality-authority-separation-001/**`
- `docs/work-packages/README.md`
- `tools/local_ci/README.md`
- `tools/release/README.md`

## Dependencies

- Completed roadmap authoring package.

## Phase Plan

1. Inventory every normative and executable closure binding for CRAP/coverage.
2. Draft ADR-0041 as a targeted supersession/amendment, preserving thresholds
   as observational/action thresholds and package-local CQR/test-enhancement
   acceptance.
3. Align standards, AGENTS, templates, operator docs, and conflicting active
   package obligations.
4. State campaign and release behavior explicitly so neither silently restores
   ordinary increment blocking.
5. Reconcile the exact terminal documentation diff, review, verify, and
   disposition.

## Exit Criteria

- ADR-0041 explicitly supersedes ADR-0021/ADR-0039 cadence and closure clauses
  while preserving thresholds, eligibility taxonomy, and package-local
  CQR/test-enhancement acceptance.
- ADR-0041 explicitly decides whether release qualification must consume a
  current QA observation and states that this does not restore ordinary
  increment blocking.
- Canonical queue guidance consistently ignores, never cancels or awaits,
  permanently queued retired Omarchy records.
- Correctness, authority, conservation, consumer, and selected science gates
  remain unchanged except where necessary to remove metric coupling.
- Campaign/release quality behavior is stated explicitly by ADR-0041 and cannot
  be inferred from obsolete ADR-0021 cadence text.
- Conflicting active-package acceptance is prospectively amended and
  historically failed receipts remain immutable.
- Documentation/path checks, dual review, dual verification, and
  security-impact review pass.

## Security Impact

No runtime security behavior changes. The adopted authority requires Order 2
to implement fail-closed typed receipt semantics before qualification.

## Delegation

Subagent authorization: this package explicitly authorizes spawning/delegating
to two read-only implementation reviewers and two read-only terminal verifiers
for authority consistency, policy/schema fail-closed behavior, and exact-diff
validation; expected outputs are package artifacts; write access is read-only.
