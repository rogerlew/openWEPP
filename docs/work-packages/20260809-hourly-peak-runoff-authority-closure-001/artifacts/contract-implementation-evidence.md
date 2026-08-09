# Contract Implementation Evidence

Status: `complete`

Evidence mode: `Static`

`SC-WATBAL-001` v167 adds `INV-WATBAL-102..104` and replaces the WB16
rainfall-envelope/APPMTH production rules with a closing post-partition hourly
peak, in-hour surface-return custody, exact dry-zero behavior, depth-rate
internals, exactly-once public area conversion, and rectangular-equivalent
duration. `TOL-WATBAL-009` declares the bounded 24-interval arithmetic
reconciliation and prohibits using it to absorb missing sources or timing.

Routed melt and runon are producer-timed hourly liquid supply admitted through
WB14 exactly once. They are not appended as raw post-partition runoff limbs;
normalized hourly weights are derived only after the depth ledger closes.

The lifecycle registry is current at `2026-08-09`.
