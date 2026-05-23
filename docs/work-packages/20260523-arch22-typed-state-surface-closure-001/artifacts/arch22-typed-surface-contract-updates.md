# ARCH22 Typed-Surface Contract Updates

Status: `completed`
Evidence mode: `Static`

## Canonical Contract Amendments Implemented
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
  - version `11 -> 12`
  - added `ARCH22 Typed Production-Surface Addendum` for covered
    hydrology/plant coupling consumption via `HillslopeProductionStateSymbol`
    and `HillslopeProductionFluxSymbol`.
- `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md`
  - version `8 -> 9`
  - added ARCH22 typed addendum for residue-coupled consumption via typed
    hillslope production symbols.
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - version `14 -> 15`
  - added ARCH22 typed addendum requiring covered WB11/WB12/WB14/WB15/WB16
    interfaces to consume typed hillslope symbols.
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
  - version `10 -> 11`
  - added ARCH22 typed addendum for covered runoff-partition state/flux
    access and guard-family continuity.
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
  - version `4 -> 5`
  - added ARCH22 typed addendum for WS10 routing interfaces via
    `WatershedProductionStateSymbol` and `WatershedProductionFluxSymbol` with
    typed node/hillslope builders.
- `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
  - version `4 -> 5`
  - added ARCH22 typed addendum for covered hydraulics-coupled interfaces and
    removal of raw-string accessor signatures.
- `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
  - version `3 -> 4`
  - added ARCH22 typed addendum for WS10 impoundment node-scoped families via
    typed symbols.
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
  - version `6 -> 7`
  - added ARCH22 typed addendum requiring typed symbol families for covered
    hillslope/watershed integration interfaces.

## Registry and Alias Authority Updates
- `docs/specifications/science-contracts/index.md`
  - updated notes for all ARCH22-amended contracts to reflect typed production
    surface authority.
- `docs/specifications/science-contracts/symbol-alias-registry.md`
  - added `ARCH22 Typed-Symbol Closure Note` linking canonical alias authority
    to typed symbol family consumption in production kernels.

## Sequencing Statement
- Contract authority updates were completed before production ARCH22 migration
  code edits.
