# 20260520-infile01-author-sc-infile-climate-001

## Status
- `state`: active
- `date`: 2026-05-20
- `timezone`: UTC

## Objective
Author and disposition `SC-INFILE-CLIMATE-001` and its paired openWEPP-owned input
specification for surface `infile-climate-cli` (.cli).

## Why This Package Exists
The parser input-surface registry marks `infile-climate-cli` as `planned`.
openWEPP requires a canonical specification + parser contract pair for this
surface before implementation is accepted.

## Scope
### Included
- Author/update canonical specification:
  - `docs/specifications/wepp-input-files/specs/climate-file.spec.md`
- Author/update canonical parser contract:
  - `docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md`
- Define typed source/simulation data model, propagation mapping, and guard map.
- Define compatibility and unsupported behaviors for this surface.
- Run required dual-agent review, disposition, and dual-agent verification.
- Update parser input-surface registry disposition for `infile-climate-cli`.

### Explicitly Out of Scope
- Rust parser/runtime implementation changes.
- Contract authoring for other parser surfaces.
- Broad comparator campaigns.

## Deliverables
1. Canonical specification update:
   - `docs/specifications/wepp-input-files/specs/climate-file.spec.md`
2. Canonical parser contract update:
   - `docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md`
3. Review/disposition artifact bundle:
   - `artifacts/parser-contracts/SC-INFILE-CLIMATE-001/contract_ref.md`
   - `artifacts/parser-contracts/SC-INFILE-CLIMATE-001/review_agent_a.md`
   - `artifacts/parser-contracts/SC-INFILE-CLIMATE-001/review_agent_b.md`
   - `artifacts/parser-contracts/SC-INFILE-CLIMATE-001/disposition.md`
   - `artifacts/parser-contracts/SC-INFILE-CLIMATE-001/verification_agent_a.md`
   - `artifacts/parser-contracts/SC-INFILE-CLIMATE-001/verification_agent_b.md`
4. Artifact index update:
   - `artifacts/README.md`

## Dependencies
- `docs/specifications/wepp-input-file-parser-contract-authoring-procedure.md`
- `docs/specifications/wepp-input-files/parser-contract-requirements.md`
- `docs/specifications/wepp-input-files/input-surface-registry.md`
- `docs/specifications/wepp-input-files/README.md`
- `docs/specifications/science-contracts/README.md`
- `docs/planning/wepp-input-file-parser-survey.md`
- Static provenance sources:
  - `/home/workdir/wepp-forest`
  - `/workdir/wepppy`
  - `/workdir/wepppyo3`

## Phase Plan
### Phase 0 - Evidence Inventory
- Capture source file structure and version/datver applicability evidence.
- Inventory legacy symbol names and openWEPP boundary alias requirements.

### Phase 1 - Spec and Contract Drafting
- Draft/refresh `docs/specifications/wepp-input-files/specs/climate-file.spec.md`.
- Draft/refresh `SC-INFILE-CLIMATE-001` with required section set, propagation map,
  cross-file constraints, compatibility policy, and guard map.

### Phase 2 - Dual-Agent Review and Disposition
- Run independent reviewer A and reviewer B passes.
- Publish disposition with closure for every finding.

### Phase 3 - Dual-Agent Verification and Closeout
- Run verification A/B over fixed draft.
- Promote disposition to `GO`, `GO-WITH-AMENDMENTS`, or `HOLD`.

## Exit Criteria
- `docs/specifications/wepp-input-files/specs/climate-file.spec.md` exists and covers this surface.
- `SC-INFILE-CLIMATE-001` includes all required parser-contract sections.
- Dual independent review artifacts are complete.
- Disposition file closes or explicitly justifies every finding.
- Dual verification artifacts are complete with no unresolved high-severity
  findings.
- Registry entry for `infile-climate-cli` is updated consistently with disposition.

## Security Impact and Review Gate
- `security_impact`: `none`
- `dedicated_security_review_required`: `no`
- Rationale: docs/specification package only.
