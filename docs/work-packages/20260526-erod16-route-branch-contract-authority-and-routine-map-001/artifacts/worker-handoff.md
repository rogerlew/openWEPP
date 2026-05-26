# Worker Handoff

Status: complete
Evidence mode: static+ran
Date: 2026-05-26

## Completed in EROD16
1. Canonicalized baseline hillslope `CONTIN -> ROUTE` routine lineage and
   `mshear` branch-family authority under `SC-SED-001`.
2. Added explicit cross-contract scope partitioning in `SC-ROUTE-001` so WS10
   watershed routing authority is distinct from hillslope `route.for` parity.
3. Corrected provenance classification: `rtpart.for` is growth/root
   partitioning (`grow.for` call chain), not routing.
4. Updated science-contract registry notes and review dates for SED/ROUTE.

## Required next package
- Start `EROD17` to implement contract-derived tests and pre-implementation
  gate evidence against the now-canonical route branch map.

## Ran
- `git diff -- docs/specifications/science-contracts/contracts/SC-SED-001.md docs/specifications/science-contracts/contracts/SC-ROUTE-001.md docs/specifications/science-contracts/index.md`
