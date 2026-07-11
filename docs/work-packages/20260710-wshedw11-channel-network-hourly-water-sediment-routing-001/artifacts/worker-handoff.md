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
eleven contract-derived test vectors in `SC-ROUTE-001` v53
(`INV-ROUTE-015..020` plus the amended `INV-ROUTE-005(a)/(e)`). Resume through
WSHED-W11B using the
authority map in
`../../20260710-wshedw11a-channel-hourly-sediment-authority-001/artifacts/w11-handoff.md`.
The do-not-do constraints above remain in force and are extended by the
handoff's list (no event-scalar solve on activated channels, no new unit
constants, no bed store, no suspended pool, no HBP schema change).

## Implementation successor (2026-07-10)

Codex post-hoc review ratified `SC-ROUTE-001` v53 with eleven vectors and the
distinct wave-total `qlat(it)` / per-unit-length `qlat_eff(it)` binding.
Implementation resumes in
`../../20260710-wshedw11b-channel-interval-sediment-implementation-001/package.md`,
which owns Phase B through closure and both `GAP-ROUTE-014` terminal fixes. This
successor is now `EXECUTED-COMPLETE`; both terminal defects and the production
direct-consumer gap are closed. This held W11 package remains historical Phase-A
evidence and has no remaining implementation handoff.
