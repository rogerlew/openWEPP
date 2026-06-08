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

## Batch 3 update

WB13/WB16/WB12 batch stopped at a narrower HOLD. WB12 and WB16 need flagged binding exposure/promotion before relocation; WB13 output-surface authority remains mapped-but-core-resident because schema/order/guard/test-vector obligations are active.

## Batch 4 update

CLIM05 rows were mapped to `INV-WATBAL-013` and retained in core. CLIM06/WB14/WB15/IRRIG10 rows stopped at narrower promotion/mapping gates because their live coupling obligations are not fully exposed by current invariant rows.

## Batch 5 update

Mapped 14 WB19/WB18 hourly/handoff rows to existing invariants and retained them in core. HPHYS0224/0225/0226/0227 remain narrower HOLDs for cap/constitutive/water-yield binding exposure.

## Batch 6 update

Mapped SIMIMPL03/14/15/16/18/21 rows to existing invariants and retained them in core. EROD12/MOFE04/ARCH22/EROD13/EROD14 remain narrower HOLDs for exact cross-domain binding exposure.
