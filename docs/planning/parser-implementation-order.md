# Parser Implementation Order Plan

Date: 2026-05-21
Status: Draft (INIMPL01)
Evidence mode: `Static`

## Decision Summary

openWEPP should implement parser surfaces in dependency-aware waves with
strict wave precedence: Tier-A hillslope core first, then hillslope sidecars,
then watershed core, then watershed sidecars.

Rationale:
- `[DIRECT]` Registry marks all `SC-INFILE-*` parser surfaces as `active`.
- `[DIRECT]` ARCH-01 comparator policy gives highest acceptance confidence to
  single OFE + daily water-balance paths.
- `[DIRECT]` Contract HOLD registers show meaningful compatibility/authority
  uncertainty in several watershed-sidecar surfaces; these should be sequenced
  after core parser readiness.

## Canonical Ordered Queue (1..19)

1. `SC-INFILE-SLOPE-001`
2. `SC-INFILE-SOIL-001`
3. `SC-INFILE-CLIMATE-001`
4. `SC-INFILE-MANAGEMENT-001`
5. `SC-INFILE-PMETPARA-001`
6. `SC-INFILE-IRRIGATION-DEPLETION-001`
7. `SC-INFILE-IRRIGATION-FIXEDDATE-001`
8. `SC-INFILE-FROST-001`
9. `SC-INFILE-SNOW-001`
10. `SC-INFILE-WEPPUI-001`
11. `SC-INFILE-WATERSHED-STRUCTURE-001`
12. `SC-INFILE-WATERSHED-CHANNEL-001`
13. `SC-INFILE-WATERSHED-IMPOUNDMENT-001`
14. `SC-INFILE-CHANINP-001`
15. `SC-INFILE-TC-001`
16. `SC-INFILE-GWCOEFF-001`
17. `SC-INFILE-PHOSPHORUS-001`
18. `SC-INFILE-TCR-001`
19. `SC-INFILE-LCWB-001`

Note:
- Ordered queue is score-informed and dependency-constrained.
- Wave precedence is authoritative; ranking is intra-wave only.

## Wave Plan

### Wave 1: Tier-A Core
- `SC-INFILE-SLOPE-001`
- `SC-INFILE-SOIL-001`
- `SC-INFILE-CLIMATE-001`
- `SC-INFILE-MANAGEMENT-001`

### Wave 2: Tier-A Extension Sidecars
- `SC-INFILE-WEPPUI-001`
- `SC-INFILE-PMETPARA-001`
- `SC-INFILE-SNOW-001`
- `SC-INFILE-FROST-001`
- `SC-INFILE-IRRIGATION-FIXEDDATE-001`
- `SC-INFILE-IRRIGATION-DEPLETION-001`

### Wave 3: Watershed Core
- `SC-INFILE-WATERSHED-STRUCTURE-001`
- `SC-INFILE-WATERSHED-CHANNEL-001`
- `SC-INFILE-WATERSHED-IMPOUNDMENT-001`

### Wave 4: Watershed Sidecar Extension
- `SC-INFILE-CHANINP-001`
- `SC-INFILE-TC-001`
- `SC-INFILE-GWCOEFF-001`
- `SC-INFILE-PHOSPHORUS-001`
- `SC-INFILE-TCR-001`
- `SC-INFILE-LCWB-001`

## Why This Order

1. It maximizes earliest confidence signal from Tier-A surfaces.
2. It unlocks the largest dependency fan-out first.
3. It defers highest governance uncertainty sidecars until after core parser
   framework behavior is proven.
4. It aligns parser rollout with explicit strict/compatibility policy closure
   gates per contract HOLD registers.

## Required Gate Checks Per Wave

1. Guard and invariant mapping completeness for touched parser surfaces.
2. Explicit strict vs compatibility-mode tests for every sidecar branch.
3. Typed error taxonomy coverage for malformed/missing/open-failure paths.
4. Observability traces for parser failures and compatibility warnings.
5. Wave disposition cannot be `GO` with unresolved high-severity correctness
   findings.

## Evidence Map

- `[DIRECT]` `docs/specifications/wepp-input-files/input-surface-registry.md`
- `[DIRECT]` `docs/specifications/science-contracts/contracts/SC-INFILE-*.md`
- `[DIRECT]` `docs/work-packages/20260520-arch01-subsystem-map-and-contract-spine/artifacts/comparator-confidence-tier-policy.md`
- `[DIRECT]` `docs/planning/wepp-input-file-parser-survey.md`
- `[INFERENCE]` Dependency fan-out and wave grouping from cross-file constraints,
  guard maps, and contract gap registers.
