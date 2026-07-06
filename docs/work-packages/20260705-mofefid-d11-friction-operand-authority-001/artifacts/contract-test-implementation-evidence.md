# Contract-Test Implementation Evidence

Status: blocked-by-source-authority
Evidence mode: Static

D11 did not author production tests because the pre-implementation gate held
before builder implementation. Adding tests that assert a fabricated `k_o`,
`C_d`, `D_r`, or `lambda` mapping would encode surrogate physics.

Required tests for the hold-lift package are now named in `SC-OFEROUTE-001`
rev 19:

- Per-operand source/default/fail-closed tests for `I`, `k_o`, `C_d`, `D_r`,
  `lambda`, `LAI`, and `h_c`.
- A friction-sensitive real-consumer test proving the Lane D candidate consumes
  builder-produced operands rather than the old hardcoded path.
- A negative source guard for all-lane `k_o=500` / `I=0` acceptance claims.

Existing tests remain relevant but insufficient:

- `ofe_routing::friction` unit tests prove equation behavior.
- `tests/integration/laned_shadow_h2637.rs` proves the shadow consumes live
  publication water rows and preserves protected outputs. It does not prove
  friction operand lineage.
