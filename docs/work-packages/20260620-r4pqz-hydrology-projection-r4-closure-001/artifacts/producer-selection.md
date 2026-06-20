# Producer Selection

Status: complete.

Static: R4P/Q/Z is a shadow-only projection/closure producer. It does not
replace public WB13/WAT/PASS/loss outputs. The selected producer is a final
direct hydrology projection span after R4B storage reconciliation and before
R3B diagnostic ledger accounting.

Decision:

- Add `direct_runtime/projection.rs`.
- Recompute aggregate storage from the final direct layer vector produced by
  R4N root uptake.
- Build direct hydrology projection operands from existing direct shadows:
  R4A runoff, R4M percolation, R4O subsurface, R4N ET/root uptake, R4G snow
  coupling, R4B storage reconciliation, and R3C transfer/carry evidence.
- Keep public output projection/cutover deferred to R6.
