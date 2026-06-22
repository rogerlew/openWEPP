# Seed Authority Inventory

Status: executed-held.

## Current Sources

- Before R7D2, production direct built `DirectLaneConstructorInputs` from
  topology/area only and `DirectPublicationDayInputBuilder` cloned one
  aggregate `HillslopeWritebackSurface` for every lane.
- After R7D2, production direct selects per-lane seed surfaces from
  `OfeLanePersistentStateSequence` when available, seeds constructor water,
  layers, and ET stage state from the selected lane, and derives direct profile
  projection inputs from that same lane seed authority.
- `DirectPublicationDayInputBuilder::new_with_seed_surfaces` derives a
  `DirectHydrologyProjectionInputs` vector indexed to the seed-surface vector.
- Multi-OFE direct production now fails closed when no lane-indexed persistent
  lane state exists.

## Forbidden Sources

- `execution.wb13_rows` and compatibility public-output builders.
- Compatibility scheduler results as direct production authority.
- Aggregate runtime surfaces as multi-OFE direct seed authority.
- Compatibility trace/runtime-surface `wb12_infiltration` and
  `wb12_depression_storage_delta` observations as substitutes for direct WB14
  producer output.
- Static area-weighted storage or runoff synthesis without lane-local dynamic
  hydrology state.

## Target Direct Authority

- Completed in this package: lane-indexed seed/profile authority for direct
  constructor and day-input production.
- Still missing: baseline-authoritative direct WB14/R4K producer authority for
  hyetograph infiltration, depression storage, same-pass WB12 handoff, WB18
  layer ingress, ET same-pass infiltration, and R4A runoff partition.
- Next package target: convert parsed lane climate/soil/management/snow/frost
  operands into typed direct WB14/R4K inputs and compute those producer outputs
  inside direct runtime before R4A.
