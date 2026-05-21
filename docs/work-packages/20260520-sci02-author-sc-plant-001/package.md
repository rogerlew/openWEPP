# 20260520-sci02-author-sc-plant-001

## Status
- `state`: complete
- `date`: 2026-05-20
- `timezone`: UTC

## Objective
Author and review `SC-PLANT-001` as a canonical openWEPP science contract for
plant growth process behavior and hydrology/erosion coupling boundaries.

## Why This Package Exists
SCI-01 established the chapter-to-contract mapping and identified
`SC-PLANT-001` as a required domain contract. The repository now has a canonical
science-contract location and mandatory dual-agent review workflow; this package
executes that workflow for the first plant-domain contract.

## Scope
### Included
- Author canonical contract file:
  - `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- Define plant-domain invariants (`INV-PLANT-*`) with citation anchors.
- Define producer/consumer obligations and boundary dispositions for plant
  state surfaces.
- Run required dual independent agent review and capture findings.
- Produce finding disposition and post-fix dual-agent verification records.
- Update canonical science-contract registry entry for `SC-PLANT-001`.

### Explicitly Out of Scope
- Rust kernel implementation changes.
- Contract authoring for non-plant domains.
- Broad comparator campaign execution.

## Deliverables
1. Canonical contract draft/update:
   - `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
2. Artifact bundle:
   - `artifacts/science-contracts/SC-PLANT-001/contract_ref.md`
   - `artifacts/science-contracts/SC-PLANT-001/review_agent_a.md`
   - `artifacts/science-contracts/SC-PLANT-001/review_agent_b.md`
   - `artifacts/science-contracts/SC-PLANT-001/disposition.md`
   - `artifacts/science-contracts/SC-PLANT-001/verification_agent_a.md`
   - `artifacts/science-contracts/SC-PLANT-001/verification_agent_b.md`
3. Package artifact index update:
   - `artifacts/README.md`

## Dependencies
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/README.md`
- `docs/specifications/science-contracts/index.md`
- `docs/work-packages/20260520-sci01-50201000-process-contract-mapping/artifacts/50201000-chapter-process-contract-map.md`
- `references/50201000/chap8.pdf` (primary)
- Coupled references: `chap5.pdf`, `chap7.pdf`, `chap9.pdf`, `chap11.pdf`

## Phase Plan
### Phase 0 — Contract Skeleton and Citation Inventory
- Build/update `SC-PLANT-001` skeleton with metadata and section scaffolding.
- Capture citation inventory for plant growth and coupling boundaries.

### Phase 1 — Invariant and Boundary Authoring
- Author `INV-PLANT-*` invariants and boundary dispositions.
- Author producer/consumer obligations and gap register.

### Phase 2 — Dual-Agent Review and Fix Pass
- Run independent review by Agent A and Agent B.
- Disposition findings and apply required amendments.

### Phase 3 — Dual-Agent Verification and Closeout
- Run Agent A/B verification on applied fixes.
- Finalize promotion-ready disposition status for this revision.

## Exit Criteria
- Canonical `SC-PLANT-001` exists and contains required draft sections.
- Two independent review artifacts are completed.
- Disposition file covers every reported finding.
- Two verification artifacts are completed with no unresolved high-severity
  findings.
- Registry entry for `SC-PLANT-001` is present and consistent with contract
  status/maturity.

## Security Impact and Review Gate
- `security_impact`: `none`
- `dedicated_security_review_required`: `no`
- Rationale: documentation/specification package only.
