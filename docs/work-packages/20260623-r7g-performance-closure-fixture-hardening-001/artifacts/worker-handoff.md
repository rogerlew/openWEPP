# Worker Handoff

Status: executed-held.

## Final Disposition

`HOLD-R7G-SURFACE-FREE-ACTIVE-SNOW-PARTITION-AUTHORITY-ABSENT`.

## Next Action

Close defect
`HOLD-R7G-SURFACE-FREE-ACTIVE-SNOW-PARTITION-AUTHORITY-ABSENT`.

First implementation action: scaffold and execute a typed active-snow direct
runtime package that adds surface-free snow partition authority for production
direct mode. The package must include:

- typed parsed snow controls and persistent lane snowpack state;
- hourly winter forcing/rain-snow partition inputs without
  `HillslopeWritebackSurface`;
- direct snow partition compute, state mutation, downstream operands, and
  shadow projection;
- WAT/HBP/PASS/loss/manifest publication operand projection for `RM`,
  `Snow-Water`, snow coupling, and related storage fields;
- active-snow fixture that currently trips R7G lane 1;
- source scans proving `DirectProductionDayInputBuilder` does not call the
  existing map-backed `direct_publication_snow_liquid_partition`;
- a split of `day_input_and_helpers.rs` before adding substantial Rust code.

After the snow package, rerun R7G. If the next blocker is active frost, close
it with the same typed-state pattern rather than wrapping compatibility
surfaces.
