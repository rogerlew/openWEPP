# Final Disposition

Status: `EXECUTED-COMPLETE`

WSHED-W9 is executed complete.

## Closed Boundary

- Valid HBP latest-day `NO_EVENT` and `SUBEVENT` payloads are typed as
  `HbpLatestEventState::NoEvent`, not collapsed into absent optional runoff
  payloads.
- Watershed pass inventory requires a latest HBP state and validates runoff
  `EventPayload` vectors only when the state is actually an event.
- Watershed routing-input construction maps valid no-event state to explicit
  zero surface runoff/sediment fields and preserves parsed baseflow/deep-seepage
  volumes.
- A later no-event day after an earlier event day routes the no-event state; the
  earlier runoff payload is not reused.
- Malformed no-event groundwater payloads fail closed before watershed
  publication.

## Final Status

`EXECUTED-COMPLETE` on 2026-07-09.
