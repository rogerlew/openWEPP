# No-Premature-Stop Audit

Evidence class: Static/Ran.

Audit items must be answered before final disposition:

- A red timing result is not a boundary until attributed to named mechanisms.
- A parity mismatch is not a boundary until reduced to output family, row/key,
  field, producer, consumer, and authority.
- A new fail-closed marker starts the next iteration when it is in-envelope.
- Missing comparison, profiling, source-scan, fixture, or reconstruction helper
  support is not a boundary when helper changes are in the write set.
- An in-envelope hot-loop cost must be corrected or rejected with concrete
  safety/authority evidence before HOLD.
- An in-envelope protected-output residual must be corrected or rejected with
  concrete authority evidence before HOLD.
- R7H cannot be complete while direct endpoint timing is red, parity is red,
  no-compatibility proof is unrun, reconstruction is missing, or rollback is
  unproven.

Current answers:

- Timing was red and in-envelope. It was profiled to a named mechanism:
  `require_shadow_fine_state_domains` formatted indexed fine-layer
  `BoundarySymbol`s on the valid typed direct path. The fix removed valid-path
  formatting and direct timing is now green (`61.40 s` default-candidate,
  `64.19 s` explicit direct, budget `<=91.2 s`).
- Compatibility and rollback were rerun on current code. They are stable:
  default compatibility and explicit rollback checksums match exactly for
  HBP/WAT/PASS/loss/plot; WAT row-difference is `0/0`.
- Direct and direct-default were rerun on current code. They are stable against
  each other and both report `compatibility_edge_invocations=0`.
- Protected parity is red and was reduced:
  direct-vs-compatibility HBP/WAT/PASS differ, loss/plot match, WAT first
  material divergence occurs on Julian day 6 in frost/water state fields, and
  PASS deltas are hydrology/runoff fields only.
- Operator closure supersedes the former hold. The remaining frost split is not
  an R7H premature-stop defect because compatibility frost is not validated to
  frost-depth magnitude. It is reclassified under reopened
  `GAP-SNOWFREEZE-002`.
- R7H is closed `OPT-IN`, not default-activated. Protected parity remains red
  for frost-influenced fields by acknowledged contract delta; compatibility,
  rollback, and shadow paths remain intact.
