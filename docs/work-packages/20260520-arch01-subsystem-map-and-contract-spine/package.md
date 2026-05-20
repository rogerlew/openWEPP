# 20260520-arch01-subsystem-map-and-contract-spine

## Status
- `state`: closed
- `date`: 2026-05-20
- `timezone`: UTC
- `closed_utc`: 2026-05-20 23:25 UTC
- `outcome`: `GO_ARCHITECTURE_DISCOVERY_COMPLETE`

## Objective
Establish a production-usable architecture baseline for openWEPP by mapping
subsystems, state surfaces, and top-down science-contract authority before
major kernel implementation.

## Why This Package Exists
openWEPP adopted architecture-first, top-down contract governance in
ADR-0011. This package turns that policy into executable architecture assets so
implementation work can proceed without waiting for full legacy
re-kernelization.

## Scope
### Included
- Subsystem inventory for hillslope, watershed, replay, contracts, and outputs.
- Dependency map across subsystems and data boundaries.
- State-surface catalog with producer/consumer and unit ownership.
- Legacy `.run` + `.txt` sidecar backward-compatibility bridge definition for
  initial adoption.
- Invariant catalog sourced from:
  - `references/50201000`
  - peer-reviewed literature
  - physical/common-sense constraints
  - static legacy code inspection (secondary evidence)
- Comparator confidence-tier policy draft (single OFE/daily high confidence;
  hourly/watershed investigation signal).
- Architecture decision summary suitable to promote into a follow-on ADR.

### Explicitly Out of Scope
- Broad kernel implementation.
- Major source mutations in `src/` crates (none exist yet).
- Re-running large cohort comparisons.
- Full replacement cutover to schema-only `.run` ingestion.

## Deliverables
1. `artifacts/subsystem-inventory.md`
2. `artifacts/subsystem-dependency-map.md`
3. `artifacts/state-surface-catalog.csv`
4. `artifacts/invariant-catalog.md`
5. `artifacts/reference-citation-matrix.md`
6. `artifacts/comparator-confidence-tier-policy.md`
7. `artifacts/architecture-decision-summary.md`
8. `artifacts/legacy-run-sidecar-compatibility-bridge.md`
9. `artifacts/arch01_disposition.md`

## Phase Plan
### Phase 0 — Docs-Only Audit / Inventory
- Build subsystem inventory and dependency map.
- Build initial state-surface catalog.
- Build citation matrix with evidence tags (`[DIRECT]`, `[INFERENCE]`).

### Phase 1 — Architecture Decision
- Produce architecture decision summary.
- Confirm module boundaries and contract ownership.
- Confirm comparator tier policy and investigation routing.

### Phase 2 — Single Mechanism Pilot
- Land one narrow mechanism only:
  - scaffold a non-invasive contract/invariant check surface for a single
    high-confidence domain (single OFE + daily water balance), or
  - if coding is deferred, produce a code-ready implementation blueprint with
    explicit file/module targets and acceptance checks.

### Phase 3 — Closeout
- Record disposition.
- Queue follow-on subsystem implementation work packages with clear ownership.

## Dependencies
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/specifications/README.md`
- `docs/contracts/README.md`
- `docs/architecture/README.md`
- `references/README.md`
- Legacy static analysis surface: `/home/workdir/wepp-forest` (read-only)

## Exit Criteria
- All listed artifacts are populated and internally consistent.
- Subsystem boundaries and state ownership are explicit and reviewable.
- Comparator confidence tiers are documented with concrete triage actions.
- A follow-on implementation sequence is defined from this architecture spine.

## Security Impact and Review Gate
- `security_impact`: `none`
- `dedicated_security_review_required`: `no`
- Rationale: architecture and documentation package; no runtime attack-surface
  changes.
