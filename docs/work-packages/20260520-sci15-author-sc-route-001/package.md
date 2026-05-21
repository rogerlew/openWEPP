# 20260520-sci15-author-sc-route-001

## Status
- `state`: active
- `date`: 2026-05-20
- `timezone`: UTC

## Objective
Author and review `SC-ROUTE-001` as a canonical openWEPP science contract for
process-based channel/watershed routing, channel hydrology/erosion behavior, and hillslope handoff boundaries.

## Why This Package Exists
SCI-01 identified watershed routing and channel hydrology as a core process domain in the contract
spine. This package executes the canonical science-contract workflow to
establish `SC-ROUTE-001` with explicit invariants, guard mappings, symbol alias
mapping, and dual-agent review/verification evidence.

## Scope
### Included
- Author canonical contract file:
  - `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- Define watershed routing and channel hydrology-domain invariants (`INV-ROUTE-*`) with
  citation anchors.
- Define producer/consumer obligations and watershed routing and channel hydrology boundary
  dispositions.
- Include required invariant-to-guard mapping and symbol alias mapping.
- Run required dual independent agent review and capture findings.
- Produce finding disposition and post-fix dual-agent verification records.
- Update canonical science-contract registry entry for `SC-ROUTE-001`.

### Explicitly Out of Scope
- Rust kernel implementation changes.
- Contract authoring for non-watershed routing and channel hydrology domains.
- Broad comparator campaign execution.

## Deliverables
1. Canonical contract draft/update:
   - `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
2. Artifact bundle:
   - `artifacts/science-contracts/SC-ROUTE-001/contract_ref.md`
   - `artifacts/science-contracts/SC-ROUTE-001/review_agent_a.md`
   - `artifacts/science-contracts/SC-ROUTE-001/review_agent_b.md`
   - `artifacts/science-contracts/SC-ROUTE-001/disposition.md`
   - `artifacts/science-contracts/SC-ROUTE-001/verification_agent_a.md`
   - `artifacts/science-contracts/SC-ROUTE-001/verification_agent_b.md`
3. Package artifact index update:
   - `artifacts/README.md`

## Dependencies
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/README.md`
- `docs/specifications/science-contracts/index.md`
- `docs/work-packages/20260520-sci01-50201000-process-contract-mapping/artifacts/50201000-chapter-process-contract-map.md`
- `references/50201000/chap13.pdf` (primary)
- Coupled references: `chap1.pdf`, `chap11.pdf`, `chap6.pdf`, `chap14.pdf`

## Phase Plan
### Phase 0 - Contract Skeleton and Citation Inventory
- Build/update `SC-ROUTE-001` skeleton with metadata and section scaffolding.
- Capture citation inventory for watershed routing and channel hydrology boundary constraints.

### Phase 1 - Invariant, Guard, and Boundary Authoring
- Author `INV-ROUTE-*` invariants and boundary dispositions.
- Author invariant-to-guard map and alias map.
- Author producer/consumer obligations and gap register.

### Phase 2 - Dual-Agent Review and Fix Pass
- Run independent review by Agent A and Agent B.
- Disposition findings and apply required amendments.

### Phase 3 - Dual-Agent Verification and Closeout
- Run Agent A/B verification on applied fixes.
- Finalize promotion-ready disposition status for this revision.

## Exit Criteria
- Canonical `SC-ROUTE-001` exists and contains required draft sections.
- Two independent review artifacts are completed.
- Disposition file covers every reported finding.
- Two verification artifacts are completed with no unresolved high-severity
  findings.
- Registry entry for `SC-ROUTE-001` is present and consistent with contract
  status/maturity.

## Security Impact and Review Gate
- `security_impact`: `none`
- `dedicated_security_review_required`: `no`
- Rationale: documentation/specification package only.
