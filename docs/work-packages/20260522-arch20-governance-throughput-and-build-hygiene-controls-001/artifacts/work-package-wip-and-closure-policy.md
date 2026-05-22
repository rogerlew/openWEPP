# Work-Package WIP and Closure Policy

Evidence mode: `Static`
Status: `complete`

## Source Finding Linkage

- [DIRECT] `docs/work-packages/20260522-arch14-claude-architecture-review-disposition-001/artifacts/disposition-register.md` (`CRF-008`: requires WIP limits and closure SLA policy).
- [DIRECT] `docs/work-packages/20260522-arch14-claude-architecture-review-disposition-001/artifacts/remediation-work-package-queue.md` (dependency-ordered remediation lanes).

## Policy Goals

- Control package churn and false closeout.
- Preserve correctness-over-completion.
- Keep high-severity remediation lanes moving with explicit ownership.

## Normative WIP Controls

1. `WIP-001`: A closure owner MUST NOT hold more than `2` active packages at
   once.
2. `WIP-002`: For the same dependency lane (same blocking `CRF-*` objective),
   only one package MAY be `active` at a time unless a parent orchestration
   package explicitly documents parallel worker ownership.
3. `WIP-003`: A package MUST define objective, write set, and exit criteria in
   `package.md` before status may move to `active`.
4. `WIP-004`: Re-opening a `complete` package MUST include a root-cause note,
   impacted files, and a replacement closure target date.

## Normative Closure Controls

1. `CLS-001`: A package MUST NOT be marked `complete` while any required
   artifact remains `Status: pending`.
2. `CLS-002`: A package MUST provide explicit disposition state (`GO`,
   `GO-WITH-AMENDMENTS`, or `HOLD`) with rationale.
3. `CLS-003`: If any required gate is skipped or fails, disposition MUST be
   `HOLD`.
4. `CLS-004`: All closure claims MUST include evidence posture labeling:
   `Static` and/or `Ran`.
5. `CLS-005`: A package MUST include an owned-file manifest that matches
   actual touched files.

## False-Closeout Prevention Rules

1. `FC-001`: "Artifacts drafted" is not sufficient for closure; required gates
   and verification artifacts MUST also be complete.
2. `FC-002`: "Code compiles locally" is not sufficient for code-touch package
   closure; workspace gates are mandatory per evidence/gate policy.
3. `FC-003`: If unresolved correctness gaps remain, disposition MUST stay
   `HOLD` even when schedule pressure exists.

## Closure SLA Expectations

1. `SLA-001`: Review artifacts (`review_agent_a.md`, `review_agent_b.md`) MUST
   be completed in the same day as disposition drafting.
2. `SLA-002`: Verification artifacts MUST be completed before disposition state
   can be considered final.
3. `SLA-003`: Queue snapshot updates for affected remediation lanes SHOULD be
   performed in the same change set as package closeout.

## Compliance Audit Checklist

- `package.md` defines objective, scope, exit criteria.
- Required artifacts exist and are non-pending.
- Gate results align with scope type (docs-only vs code-touch).
- Disposition state and evidence posture are explicit.
- Owned-file manifest matches actual changed files.
