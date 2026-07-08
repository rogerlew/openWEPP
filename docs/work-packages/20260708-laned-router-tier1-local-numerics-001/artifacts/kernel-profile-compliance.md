# Kernel Profile Compliance

Status: `EXECUTED`

Compliance check:

- Contract-first: yes, `SC-OFEROUTE-001` rev 47 before production code.
- Typed errors: yes, local numeric invalid states return `RoutingError`.
- No surrogate physics: yes, formulas are algebraic rewrites of the bound
  friction/KWE relation.
- No silent fallback: yes; branch-gap selection is explicit contract behavior,
  active vegetation non-finite math fails closed, and unratified `Re^0.45`
  approximation is not implemented.
- Conservation evidence: D10B reconciliation suite and H2637 active day closure
  pass.
- Comparator posture: D10B/Iwagaki oracle remains the acceptance surface;
  bit identity is not claimed.
- Final gates: `cargo fmt --check`, clippy, full nextest, and
  `cargo deny check` pass.
