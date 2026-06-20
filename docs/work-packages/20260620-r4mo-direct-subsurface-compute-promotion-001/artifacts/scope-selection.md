# Scope Selection

Status: pre-implementation.

Static: R4M/O is a direct-compute promotion, not a handoff package. It owns
request-free direct WB18/WB19 compute from typed layer vectors and leaves public
publication authoritative in compatibility mode.

Static: Included compute:

- WB18 percolation layer routing, same-pass infiltration distribution,
  per-layer `Pe_i`, bottom-layer `D`/`Pe`, scalar/layer storage reconciliation,
  and deep-percolation roundoff canonicalization.
- WB19 drainage and lateral withdrawal over direct layer state, including
  drainage-before-lateral ordering for hourly tail coverage, realized
  withdrawal caps, `q`, `Qdd`, `Qd`, carry arrays, and layer diagnostics.

Static: Excluded compute:

- WB17 ET/root uptake; this remains R4N.
- Public `Dp`, `latqcc`, `Qd`, WB13/WAT/PASS/loss/schema cutover; this remains
  R4P/Q/Z and R6.
- Scheduler/default activation changes.

Pre-implementation disposition: proceed with production Rust edits only inside
the intended write set.
