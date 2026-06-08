# SCSTRUCT03 Disposition

Evidence mode: Static + Ran lint
Status: in-progress stop-boundary after batch 1

## Disposition

SCSTRUCT03 completed the lowest-risk ProfileFC/WP batch. Five historical/superseded lineage sections were relocated to the provenance sidecar, with BEI rows mapped to existing core binding IDs and no binding additions.

The package stops before the snow/melt-term comparator batch because those rows require science/operator authority decisions under ADR-0017. No forced calls were made.

## Protected boundaries

- No kernel/runtime code edited.
- No `INV-*` / `OBL-*` binding IDs removed, weakened, or added.
- No comparator re-tiering.
- Remaining science-review rows stay in the core contract.

## Batch 2 update

Completed snow/melt comparator arc relocation with binding residue mapped to current WATBAL governance invariants. Strict lint remains PASS-DEFERRED because later batches remain science-review routed.
