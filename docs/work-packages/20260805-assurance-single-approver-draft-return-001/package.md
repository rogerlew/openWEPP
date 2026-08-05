# Assurance Single-Approver Semantics And Draft Return

Status: `scaffolded / authorized`

This ExecPlan is a living document governed by `docs/codex_exec_plans.md`.

## Purpose

Remove the ambiguity between governance prose and the v2 assurance event
model. The current publication contract requires exactly one scientific
approver and one reproduction/publication approver, with no co-approver
primitive. Therefore a
report lead, applicable material producer, or applicable build maintainer is
ineligible for the corresponding approval role; "not the sole approver" must
not imply that producer co-approval is representable.

Return the snow/frost flagship from `IN_REVIEW` to `DRAFT` through an explicit
typed lifecycle event. Preserve its immutable review-entry history, clear all
active review authority and roots required by the DRAFT projection, keep the
public report count at zero, and do not fabricate approval.

## User Authority

Direct instruction on 2026-08-05: tighten the policy/tooling language and keep
the assurance in draft.

## Implementation Intent

- Governance clarification plus typed lifecycle implementation.
- No science, result, model, runtime, fixture, default, public-output, or
  release change.
- The snow/frost report remains unpublished and returns to `DRAFT` before CoE
  cutover implementation resumes.

## Included Scope

- Replace ambiguous "cannot be the sole approver" language with explicit
  current-model ineligibility and a statement that co-approval would require a
  separately designed schema/event extension.
- Add one typed `return_to_draft` lifecycle event from `IN_REVIEW`.
- Make the active projection DRAFT-valid while retaining the immutable prior
  review-entry event and generation chain.
- Add focused lifecycle/publication/source contract tests.
- Apply the event to `snow-and-frozen-soil-process-evaluation` through the
  canonical CLI and regenerate governed identity/review projections.
- Reconcile ASSURE-06 status, roadmap, and catalog truthfully.

## Excluded Scope

- Co-approver schema or voting/quorum semantics.
- Approval, publication, release transfer, or deletion of historical events.
- Scientific manuscript/result changes except deterministic lifecycle wording
  required to state DRAFT status.
- Stage 3/CoE runtime work; that receives a separate package.

## Intended Write Set

- `docs/governance/scientific-assurance-v2-architecture.md`
- `docs/governance/scientific-assurance-v2-source-build-contract.md`
- `docs/governance/scientific-assurance-dossier-lifecycle.md`
- `docs/specifications/assurance-amendment-and-identity-workflow.md`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260805-assure06-snow-frost-flagship-refresh-001/**`
- `docs/work-packages/20260805-assurance-single-approver-draft-return-001/**`
- `crates/openwepp-assurance/src/v2/amendment.rs`
- `crates/openwepp-assurance/src/v2/identity.rs`
- `crates/openwepp-assurance/src/v2/lifecycle.rs`
- `crates/openwepp-assurance/src/v2/publication.rs`
- `assurance/v2/README.md`
- `assurance/v2/schemas/review-event.schema.json`
- `tests/integration/assurance_v2_amendment_contract.rs`
- `tests/integration/assurance_v2_lifecycle_contract.rs`
- `tests/integration/assurance_v2_source_contract.rs`
- `assurance/v2/identity.lock.json`
- `assurance/v2/transactions/**`
- `assurance/v2/reports/snow-and-frozen-soil-process-evaluation/report.yaml`
- `assurance/v2/reports/snow-and-frozen-soil-process-evaluation/review.lock.json`
- `assurance/v2/reports/snow-and-frozen-soil-process-evaluation/review-events/**`
- `usersum/assurance/review-drafts/snow-and-frozen-soil-process-evaluation/1.0.0/**`

Everything else is read-only. Generated files may change only through the
typed assurance CLI.

## Validation

- Focused lifecycle, amendment, publication, and real-source tests.
- DRAFT projection contains no active charge, roots, findings, approvals,
  producers, or independence claim while the immutable return event records
  why the prior review root ceased to be active.
- Named and all-report validation/plan pass; public report count remains zero.
- Formatting, warnings-denied Clippy, doctests, quick profile, immediate
  full-workspace correctness regression, documentation lint, American-English
  preview, path checks, and `git diff --check` pass.
- Dual independent Rust/governance review and dual terminal verification.

## Subagent Authorization

This package explicitly authorizes subagent spawning/delegation to one
read-only Rust correctness reviewer, one read-only assurance-governance/QA
reviewer, two read-only terminal verifiers, and one read-only
`comparator_suite_runner` for the required full-workspace correctness run. No
reviewer or runner may edit tracked files, create approval authority, or
publish.

## Progress

- [x] (2026-08-05) User identified the policy/tooling mismatch and directed
  explicit language plus DRAFT posture.
- [x] (2026-08-05) Confirmed the publication contract enforces one distinct
  human per approval role and the lifecycle had no return-to-draft transition.
- [x] (2026-08-05) Scaffolded this package before implementation edits.
- [ ] Implement and test policy/tooling reconciliation.
- [ ] Apply typed return-to-draft event and validate exact projections.
- [ ] Complete reviews, verification, and disposition.

## Decision Log

- Decision: align governance to implemented flat exclusion rather than imply
  unimplemented producer co-approval. Rationale: exact-one active approval
  events and distinct principals are current publication invariants.
  Date/Author: 2026-08-05 / Codex.
- Decision: preserve review history through a typed event rather than hand-edit
  the report back to DRAFT. Rationale: lifecycle history and generation
  identity are governed evidence. Date/Author: 2026-08-05 / Codex.

## Outcomes

Pending execution.
