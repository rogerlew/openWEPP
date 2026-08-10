# Finding Disposition

Status: `complete`

All implementation-review findings were corrected before terminal closure.

- The initially proposed raw melt/runon append was rejected. Routed melt and
  runon now enter WB14's producer-timed liquid supply once and receive their
  ordinary infiltration/depression opportunity.
- The daily same-pass snow infiltration reconstruction and its post-WB14
  earliest-bin debit were removed. WB14 is the sole infiltration and hourly
  residual authority; a real limited-capacity pure-melt regression proves the
  residual remains in its producer hour.
- A partially retained positive frost residual can no longer be
  tolerance-cleared or redistributed. Exact-zero runoff clears the series;
  every positive residual requires producer-timed custody or hard-fails.
- WB16 typed diagnostics now retain the actual rejected value and meaningful
  bounds instead of placeholder values.
- Historical `ealpha`/APPMTH provenance is marked closed and superseded;
  retained manifest fields are fixed to `false` and
  `retired_not_applicable` for schema lineage only.
- HBP output now carries the selected event row's calendar identity, and the
  real p61/p102 consumers join by simulation year and Julian day before
  independently reconstructing hourly peak flow.
- Census resume receipts now bind the plan, binary, all primary and discoverable
  sidecar inputs, expected rows, and complete calendar before reuse.
- The erosion contract's stale analytical peak, public `m3/s` forcing unit,
  obsolete APPMTH diagnostics, and uniform-shape fallback authority were
  removed. `SC-SED-001` rev63 now consumes the internal maximum-hour `m/s`
  operand, applies area only at publication, and uses a seconds-dimensional
  absolute-seconds `TOL-SED-009` custody check rather than sediment tolerance
  `TOL-SED-001` or a dimensionally invalid relative expression.
- The H2637 routing-seam fixture warms temperature only. Its ignored evidence
  counters were reconciled to the resulting source-complete behavior: all 731
  days route and zero use a uniform shape. All ten cases, including both
  ignored proofs, pass.

Terminal reviews bind exact implementation/contract/test commit
`33831787b7029b28b0716c8458f08a11899db446`. Both science reviewers, the Rust
correctness reviewer, and Rust QA returned PASS with no remaining blocking
findings.
