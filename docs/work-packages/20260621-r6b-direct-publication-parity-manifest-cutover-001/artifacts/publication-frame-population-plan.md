# Publication Frame Population Plan

Status: executed-hold.
Evidence mode: Static + Ran.

## Handoff Item 1

Populate `DirectRunPublicationFrame` from parity-grade typed direct run operands
instead of skeleton/zero direct state.

## Source Inventory

Static:

- `build_direct_publication_artifacts` creates `DirectRunFrame::skeleton`.
- `seed_direct_publication_lane_geometry` fills only area/slope geometry.
- `DirectDayFrame::seed` initializes direct forcing, hydrology projection,
  publication, and phase inputs to neutral values.
- `DirectPublicationDayRow::from_day_frame` maps those neutral values into
  public HBP/WAT/PASS/loss projection fields.
- `DirectPublicationErosionOperands::absent_authority` leaves peak, duration,
  detachment, deposition, and sediment concentration absent.

## Retained Guard

Ran: R6B added and tested
`R6B-DIRECT-PUBLICATION-TYPED-OPERANDS-ABSENT`, emitted when the cutover fails
while all direct publication operands are zero or absent.

## Gate

FAIL. There is no current production bridge that populates the cutover
`DirectRunFrame` from authoritative typed direct operands before publication
capture. R6B cannot safely implement anti-alias fixtures, reconstruction, or
manifest cutover until this bridge exists.
