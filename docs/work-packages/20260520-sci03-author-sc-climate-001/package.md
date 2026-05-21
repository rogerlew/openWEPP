# 20260520-sci03-author-sc-climate-001

## Status
- `state`: active
- `date`: 2026-05-20
- `timezone`: UTC

## Objective
Author and review `SC-CLIMATE-001` as a canonical openWEPP science contract for
weather/climate forcing generation and downstream forcing payload boundaries.

## Why This Package Exists
SCI-01 identified climate forcing as a top-priority contract domain. This
package executes the canonical science-contract workflow to establish
`SC-CLIMATE-001` with explicit invariants, guard mappings, and dual-agent
review/verification evidence.

## Scope
### Included
- Author canonical contract file:
  - `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- Define climate-domain invariants (`INV-CLIMATE-*`) with citation anchors.
- Define producer/consumer obligations and forcing-payload boundary
dispositions.
- Include required invariant-to-guard mapping and symbol alias mapping.
- Run required dual independent agent review and capture findings.
- Produce finding disposition and post-fix dual-agent verification records.
- Update canonical science-contract registry entry for `SC-CLIMATE-001`.

### Explicitly Out of Scope
- Rust kernel implementation changes.
- Contract authoring for non-climate domains.
- Broad comparator campaign execution.

## Deliverables
1. Canonical contract draft/update:
   - `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
2. Artifact bundle:
   - `artifacts/science-contracts/SC-CLIMATE-001/contract_ref.md`
   - `artifacts/science-contracts/SC-CLIMATE-001/review_agent_a.md`
   - `artifacts/science-contracts/SC-CLIMATE-001/review_agent_b.md`
   - `artifacts/science-contracts/SC-CLIMATE-001/disposition.md`
   - `artifacts/science-contracts/SC-CLIMATE-001/verification_agent_a.md`
   - `artifacts/science-contracts/SC-CLIMATE-001/verification_agent_b.md`
3. Package artifact index update:
   - `artifacts/README.md`

## Dependencies
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/README.md`
- `docs/specifications/science-contracts/index.md`
- `docs/work-packages/20260520-sci01-50201000-process-contract-mapping/artifacts/50201000-chapter-process-contract-map.md`
- `references/50201000/chap2.pdf` (primary)
- Coupled references: `chap3.pdf`, `chap4.pdf`, `chap5.pdf`, `chap12.pdf`

## Phase Plan
### Phase 0 — Contract Skeleton and Citation Inventory
- Build/update `SC-CLIMATE-001` skeleton with metadata and section scaffolding.
- Capture citation inventory for climate forcing and disaggregation boundaries.

### Phase 1 — Invariant, Guard, and Boundary Authoring
- Author `INV-CLIMATE-*` invariants and boundary dispositions.
- Author invariant-to-guard map and alias map.
- Author producer/consumer obligations and gap register.

### Phase 2 — Dual-Agent Review and Fix Pass
- Run independent review by Agent A and Agent B.
- Disposition findings and apply required amendments.

### Phase 3 — Dual-Agent Verification and Closeout
- Run Agent A/B verification on applied fixes.
- Finalize promotion-ready disposition status for this revision.

## Exit Criteria
- Canonical `SC-CLIMATE-001` exists and contains required draft sections.
- Two independent review artifacts are completed.
- Disposition file covers every reported finding.
- Two verification artifacts are completed with no unresolved high-severity
  findings.
- Registry entry for `SC-CLIMATE-001` is present and consistent with contract
  status/maturity.

## Security Impact and Review Gate
- `security_impact`: `none`
- `dedicated_security_review_required`: `no`
- Rationale: documentation/specification package only.
