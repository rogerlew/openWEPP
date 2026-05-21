# 20260520-sci13-author-sc-sed-001

## Status
- `state`: active
- `date`: 2026-05-20
- `timezone`: UTC

## Objective
Author and review `SC-SED-001` as a canonical openWEPP science contract for
process-based sediment continuity, detachment/deposition, and transport-capacity boundary behavior.

## Why This Package Exists
SCI-01 identified hillslope erosion as a core process domain in the contract
spine. This package executes the canonical science-contract workflow to
establish `SC-SED-001` with explicit invariants, guard mappings, symbol alias
mapping, and dual-agent review/verification evidence.

## Scope
### Included
- Author canonical contract file:
  - `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- Define hillslope erosion-domain invariants (`INV-SED-*`) with
  citation anchors.
- Define producer/consumer obligations and hillslope erosion boundary
  dispositions.
- Include required invariant-to-guard mapping and symbol alias mapping.
- Run required dual independent agent review and capture findings.
- Produce finding disposition and post-fix dual-agent verification records.
- Update canonical science-contract registry entry for `SC-SED-001`.

### Explicitly Out of Scope
- Rust kernel implementation changes.
- Contract authoring for non-hillslope erosion domains.
- Broad comparator campaign execution.

## Deliverables
1. Canonical contract draft/update:
   - `docs/specifications/science-contracts/contracts/SC-SED-001.md`
2. Artifact bundle:
   - `artifacts/science-contracts/SC-SED-001/contract_ref.md`
   - `artifacts/science-contracts/SC-SED-001/review_agent_a.md`
   - `artifacts/science-contracts/SC-SED-001/review_agent_b.md`
   - `artifacts/science-contracts/SC-SED-001/disposition.md`
   - `artifacts/science-contracts/SC-SED-001/verification_agent_a.md`
   - `artifacts/science-contracts/SC-SED-001/verification_agent_b.md`
3. Package artifact index update:
   - `artifacts/README.md`

## Dependencies
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/README.md`
- `docs/specifications/science-contracts/index.md`
- `docs/work-packages/20260520-sci01-50201000-process-contract-mapping/artifacts/50201000-chapter-process-contract-map.md`
- `references/50201000/chap11.pdf` (primary)
- Coupled references: `chap10.pdf`, `chap7.pdf`, `chap4.pdf`, `chap13.pdf`

## Phase Plan
### Phase 0 - Contract Skeleton and Citation Inventory
- Build/update `SC-SED-001` skeleton with metadata and section scaffolding.
- Capture citation inventory for hillslope erosion boundary constraints.

### Phase 1 - Invariant, Guard, and Boundary Authoring
- Author `INV-SED-*` invariants and boundary dispositions.
- Author invariant-to-guard map and alias map.
- Author producer/consumer obligations and gap register.

### Phase 2 - Dual-Agent Review and Fix Pass
- Run independent review by Agent A and Agent B.
- Disposition findings and apply required amendments.

### Phase 3 - Dual-Agent Verification and Closeout
- Run Agent A/B verification on applied fixes.
- Finalize promotion-ready disposition status for this revision.

## Exit Criteria
- Canonical `SC-SED-001` exists and contains required draft sections.
- Two independent review artifacts are completed.
- Disposition file covers every reported finding.
- Two verification artifacts are completed with no unresolved high-severity
  findings.
- Registry entry for `SC-SED-001` is present and consistent with contract
  status/maturity.

## Security Impact and Review Gate
- `security_impact`: `none`
- `dedicated_security_review_required`: `no`
- Rationale: documentation/specification package only.
