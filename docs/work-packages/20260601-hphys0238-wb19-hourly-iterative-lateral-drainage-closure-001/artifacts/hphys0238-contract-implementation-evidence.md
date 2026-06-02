# HPHYS0238 Contract Implementation Evidence

Status: completed  
Evidence mode: Static

## Contract Amendments

1. `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
   - `contract_version: 21`
   - Added `INV-SUBHYD-020` to require WB19 hourly lane iterative execution
     semantics for lateral/drainage closure.
   - Added guard-map linkage for `INV-SUBHYD-020`.
   - Added `HPHYS0238 WB19 Hourly Iterative Lateral/Drainage Addendum`.

2. `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
   - `contract_version: 67`
   - Added required WB19 state symbol:
     `wb19_lateral_drain_lane_substeps`.
   - Added `INV-WATBAL-030` for WB19 hourly iterative lane execution.
   - Added guard-map linkage for `INV-WATBAL-030`.
   - Added `HPHYS0238 WB19 Hourly Iterative Lateral/Drainage Addendum`.

## Measure Mapping

- `MEASURE-HP238-001`, `MEASURE-HP238-002`, `MEASURE-HP238-003`:
  authority and symbol lineage encoded in canonical `SC-*` contracts.
