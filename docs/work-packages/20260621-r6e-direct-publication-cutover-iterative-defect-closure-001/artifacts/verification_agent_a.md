# Verification A

Evidence mode: Static + Ran.

Status: focused verification complete.

Ran:

```bash
cargo test -p openwepp-runner \
  r6e_cutover_candidate_reaches_direct_input_binding_then_fails_hbp_parity \
  -- --nocapture
cargo test -p openwepp-runner \
  r6_direct_publication_cutover_cli_flag_reaches_direct_binding_then_fails_hbp_parity \
  --test r6_direct_publication_cutover_cli_contract \
  -- --nocapture
```

Result: PASS.

Verified:

- direct frame/executor/publication capture counters execute;
- direct compute/state/downstream/shadow counters are nonzero;
- skeleton-run and compatibility-edge counters remain zero;
- old input-binding marker is absent;
- current marker is `HOLD-R6E-HBP-DIRECT-PROCESS-PARITY-MISMATCH`;
- public output files are not written on cutover failure.
