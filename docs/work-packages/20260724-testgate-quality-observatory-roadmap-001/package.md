# TESTGATE And Quality Observatory Roadmap Authoring

Package ID: `20260724-testgate-quality-observatory-roadmap-001`

Status: `EXECUTED-COMPLETE-DOCUMENTATION`

## Objective

Author and independently review the roadmap and prospective work-package
scaffolds that separate TESTGATE correctness qualification from optional
coverage/CRAP observation.

## Rationale

The failed TESTGATE trajectory proved that global CRAP coupled a slow,
profile-incomplete quality measurement to an otherwise successful correctness
gate. The remedy spans policy, workflow, measurement, CQR intake, and live
qualification and therefore requires multiple packages with explicit
dependencies.

## Included Scope

- Roadmap, package catalog, and prospective queue entries.
- Seven implementation/qualification package scaffolds.
- Independent architecture, workflow, and measurement/CQR review.
- Explicit disposition of every review finding.

## Excluded Scope

- Production, workflow, policy, or test implementation.
- TESTGATE or QA dispatch.
- Reclassification of an existing failed receipt before ADR adoption.

## Declared Write Set

- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/work-packages/testgate-quality-observatory-roadmap.md`
- `docs/work-packages/20260724-testgate-quality-observatory-roadmap-001/**`
- `docs/work-packages/20260724-testgate-quality-authority-separation-001/**`
- `docs/work-packages/20260724-testgate-quality-deferral-001/**`
- `docs/work-packages/20260724-quality-observatory-workflow-001/**`
- `docs/work-packages/20260724-quality-observatory-merged-coverage-001/**`
- `docs/work-packages/20260724-cqr-nightly-quality-evidence-handoff-001/**`
- `docs/work-packages/20260724-testgate-quality-observatory-qualification-001/**`
- `docs/work-packages/20260724-quality-observatory-cqr-qualification-001/**`

## Dependencies

- User direction on 2026-07-24.
- `docs/work-packages/20260724-cqr-testgate-coverage-reconstruction-001/`
  trajectory evidence.

## Phase Plan

1. Read applicable package and standards governance and record the map.
2. Audit current ADR, TESTGATE, QA/coverage, and CQR authority.
3. Scaffold the roadmap and seven bounded packages.
4. Obtain three independent read-only reviews.
5. Disposition every finding and amend the scaffolds.
6. Run documentation/path validation and close this authoring package.

## Exit Criteria

- Every prospective package has measurable scope, dependencies, write set,
  acceptance, active kickoff prompt, artifacts directory, and explicit
  delegation authority.
- The roadmap names live functional qualification of both workflows.
- Three independent reviews exist and every finding is `accepted`, `rejected`,
  `deferred`, or `follow-up`.
- Documentation lint and path checks pass.
- Security-impact gate: `NOT APPLICABLE`; this package edits documentation
  only and grants no runtime authority before ADR adoption.

## Delegation

Subagent authorization: this package explicitly authorizes spawning/delegating
to three read-only review subagents for authority/package-boundary review,
workflow-validation review, and merged-coverage/CQR review; expected outputs
are concise findings with exact paths and acceptance recommendations; write
access is read-only.
