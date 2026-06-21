# Contract Test Implementation Evidence

Evidence mode: Static + Ran.

Status: focused fail-closed tests updated.

R6E updated focused cutover tests to assert the corrected execution boundary:

- `r6e_cutover_candidate_reaches_direct_input_binding_then_fails_hbp_parity`;
- `r6_direct_publication_cutover_cli_flag_reaches_direct_binding_then_fails_hbp_parity`.

These tests confirm:

- the cutover path reports `R6-DIRECT-PUBLICATION-PARITY`;
- the cutover path reports
  `HOLD-R6E-HBP-DIRECT-PROCESS-PARITY-MISMATCH`;
- the prior input-binding marker is absent;
- direct run-frame, executor, and publication-capture counters execute;
- direct compute, state mutation, downstream operand, and shadow projection
  counters are nonzero;
- skeleton-run and compatibility-edge counters remain zero;
- no public outputs are written.

No output-family alias, unit, or parity acceptance tests were added because HBP
process parity is not yet closed.
