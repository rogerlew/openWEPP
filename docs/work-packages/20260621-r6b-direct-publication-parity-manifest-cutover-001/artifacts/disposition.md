# Disposition

Status: executed-hold.
Evidence mode: Static + Ran.

Final disposition:
`HOLD-R6B-DIRECT-PUBLICATION-TYPED-OPERAND-BRIDGE-ABSENT`.

R6B did not complete the R6 direct-publication cutover. It stopped at the first
required current-scope gate because the production candidate still builds a
skeleton direct run frame and captures zero/default publication operands.

Retained work:

- fail-closed diagnostic marker
  `R6B-DIRECT-PUBLICATION-TYPED-OPERANDS-ABSENT`;
- focused Rust tests asserting that marker appears for the internal and CLI
  cutover failures;
- package artifacts recording the concrete production bridge blocker.

Current-scope gates for anti-alias fixtures, independent reconstruction,
manifest cutover, output-family parity, default-disabled timing, and endpoint
RSS are blocked behind the missing typed operand bridge.
