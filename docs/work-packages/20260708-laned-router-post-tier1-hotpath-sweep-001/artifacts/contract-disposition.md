# Contract Disposition

Status: `COMPLETE`
Evidence mode: Static.

Pre-code decision:

- The accepted implementation targets are behavior-preserving execution-order
  cleanup inside the existing explicit TVD-MacCormack solver.
- No `SC-OFEROUTE-001` amendment is required for reusing an already-computed
  maximum celerity to produce the same maximum Courant evidence after `dt`
  clipping.
- No `SC-OFEROUTE-001` amendment is required for delaying additive-path
  `slope.sqrt()` until after the pure-skin branch has returned, because the
  mathematical expression used by every branch remains unchanged.
- Any `Re^0.45` approximation, scheme specialization, tolerance change, or
  mesh-policy change is outside this package unless separately ratified.

Final disposition:

- No `SC-OFEROUTE-001` edit was made.
- The implementation preserves rev-47 equations, branch thresholds, CFL
  target, closure tolerances, mesh policy, and active/default selection policy.
- The `Re^0.45` approximation remains on the Tier1 hold path.
