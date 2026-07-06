# HOLD Legitimacy Audit

Status: executed-hold
Evidence mode: Static + Ran

Boundary named: `EXECUTED-HOLD-SOURCE-AUTHORITY` for
`SC-OFEROUTE-001#GAP-OFEROUTE-005`.

Evidence proving boundary:

- Case 4 was rerun after D8 and remains poor/resolution-sensitive
  (`NS_trace=0.262677`, peak ratio `0.836598`, sampled `t_peak=37.0 s` at the
  baseline resolution).
- Garcia-Navarro 1992 and Mingham 2001 are now primary-in-hand, but they do
  not directly specify the reduced Papanicolaou KWE + lateral-source + sampled
  OFE handoff implementation.
- Iwagaki 1955 primary names Manning `n=0.009`; D-val Case 4 uses `k_o`, and
  D10 has no authority to tune or default that operand.
- H2637 reproduces the production-shaped diagnostic class with aggregate
  router conservation residual `0.1047607953` and maximum per-day residual
  `0.6110480464`.

In-envelope correction routes considered:

- Source-shaped limiter branch reconciliation: tested locally and rejected
  after Case 4 worsened and six focused `ofe_routing` tests failed.
- Simple `k_o` scan: rejected as tuning without D11 friction authority.
- Implicit `alpha` iteration increase: D8 already considered and rejected
  because it changed steady/cascade conservation tests.

Why D10 cannot close now: the authority gate fails before a production
correction can be made. The package can reproduce the defect and improve the
harness, but it cannot select a limiter/boundary/friction mapping without
surrogate numerics.

First actionable follow-on: author a source-authority reconciliation package
that binds limiter/CFL/dissipation, lateral-source and OFE handoff/boundary
treatment, and Iwagaki Manning-`n` to Lane D friction operands with named Case-4
tolerances before production solver/cascade edits.
