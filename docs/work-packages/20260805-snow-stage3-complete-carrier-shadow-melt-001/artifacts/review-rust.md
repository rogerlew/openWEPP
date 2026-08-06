# Rust Correctness Review

Evidence class: Static + Ran on corrected source commit `2d035638`.

Ran: focused runtime and contract tests passed `32/32`.

Corrected high findings:

- snowfall advected heat used geometric snow depth as water mass and was
  tenfold; and
- the shadow used `333,550 J kg^-1` instead of contract-bound
  `333,600 J kg^-1` fusion heat.

The frozen Snowbird consumer was rebuilt and rerun after both corrections.

Retained HOLD findings:

- the emitted shadow residual closes a pre-vapor-debit allocation identity but
  omits proportional cold content exported with sublimated mass;
- the consumer trace omits shadow-specific per-term flux and available-ice
  operands needed for independent exact-one reconstruction;
- `runoff_reconciliation.rs` remains `3,177` lines; and
- Monin-Obukhov failures are fail-closed but mapped to an overbroad shadow
  turbulent-flux symbol, losing the primitive's typed cause.

Confirmed: the corrected `/3600 s` conversion belongs only to the hourly
carrier-rate boundary; authoritative snow mass consumes hourly totals once.
The shadow mutates cloned state and does not alter authoritative SWE, layers,
or linked liquid ledgers.

Verdict: implementation result may be dispositioned only as an executed HOLD.
It does not satisfy full linked energy closure, independent carrier lineage,
or production cutover gates.
