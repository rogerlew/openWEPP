# Worker Handoff

Status: `EXECUTED-HOLD-MISSING-CHANNEL-HOURLY-SEDIMENT-SEQUENCING-AUTHORITY`

Evidence mode: `Static` plus targeted `Ran` commands.

Dual independent reviews are dispositioned and dual verification passed. No
implemented W11 consumer path was accepted because the pre-implementation
authority gate is blocked.

## Current Handoff

No implementation was accepted. Existing M-T3 behavior remains:

- minor-1 `V_h/S_h` reaches a leaf channel;
- the channel reduces water to peak/volume/span and sediment to mass/active
  span;
- hourly contributors plus dependency nodes fail closed.

Water-series authority is ready for `ipeak` 3-5 using baseline `wshchr` grid,
state, and dependency semantics. The unresolved blocker is per-interval channel
sediment sequencing and geometry/profile/bed carry.

First action: close `WSHED-W11-HOLD-001` by executing
`20260710-wshedw11a-channel-hourly-sediment-authority-001`, then resume W11 at
Phase B. Do not start a water-only runtime or weaken the dependency guard.

## Hold lifted (2026-07-10)

`WSHED-W11-HOLD-001` is closed: WSHED-W11A completed
`EXECUTED-COMPLETE-AUTHORITY` the same day, ratifying per-interval channel
sediment sequencing, geometry/state carry, closure, guards, tolerances, and
ten contract-derived test vectors in `SC-ROUTE-001` v51 (`INV-ROUTE-015..020`
plus the amended `INV-ROUTE-005(a)/(e)`). Resume W11 at Phase B using the
authority map in
`../20260710-wshedw11a-channel-hourly-sediment-authority-001/artifacts/w11-handoff.md`.
The do-not-do constraints above remain in force and are extended by the
handoff's list (no event-scalar solve on activated channels, no new unit
constants, no bed store, no suspended pool, no HBP schema change).
