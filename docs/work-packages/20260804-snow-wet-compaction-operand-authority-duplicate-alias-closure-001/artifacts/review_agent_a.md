# Independent Review A

Evidence class: **Static + Ran**

Status: complete

## Findings

No blocking, high-, medium-, or low-severity Rust/science findings.

## Correctness Review

- The active-pack operand matches `INV-SNOWFREEZE-092` and
  `TOL-SNOWFREEZE-017`: production sums the positive part of each hourly raw
  generated-melt value with retained and released snow-contact rain, validates
  the result as finite and nonnegative under the typed symbol
  `snow.wet_compaction_liquid_input_m`, and computes it before signed-melt
  redistribution and released-rain routing. See
  `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs:2041-2048`
  and `:2133-2152`, against
  `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md:343`
  and `:1316`.
- Rain contact uses the interval-start pack snapshot. The active-hour path
  captures `state_before`/`depth_before_m` before melt and liquid processing,
  and releases rain as snow-contact rain only when that starting depth exceeds
  the zero threshold. A mixed zero-pack onset therefore excludes same-interval
  raw rain, while the inactive path supplies an exact zero operand. See
  `infiltration_reconciliation.rs:1283-1324` and
  `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs:597-603`.
- The new value remains private in `SnowCouplingOutcome` and is handed directly
  to the existing density input without a routed-melt or state-loss argument.
  The bulk and multilayer consumers each validate the input and convert metres
  water equivalent to `kg m^-2` exactly once with the existing water-density
  constant. See
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:162-183`,
  `runoff_reconciliation.rs:2303-2310`, and
  `crates/openwepp-hillslope-orchestrator/src/hydrology/09_snow_density.rs:666-693,800-852`.
  No density equation, coefficient, cap, selector, default, or public
  WAT/HBP/PASS schema changed.
- The CoE-bound offline lane now emits gross positive generated melt from the
  hourly applied-melt diagnostics and requires independent generated-melt,
  retained-rain, and released-rain columns. Missing, negative, non-finite, or
  non-finite-sum inputs fail closed before the real density replay consumes the
  value. See
  `crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs:742-810,861-890`
  and
  `crates/openwepp-runner/src/hillslope/snowbench_coe_density.rs:317-390,442-477,507-526`.
- The three-term contract identity appears once in production and once at the
  offline serialization/replay boundary. Retaining this small mirror is
  justified: it is an independently parsed reconstruction across a crate and
  artifact seam, not a second mutable production algorithm, and its purpose is
  to detect precisely the routed/state-loss aliases being closed. It is bound
  by focused reconstruction and alias-rejection tests. This does not meet the
  substantial-duplication threshold and presents no silent science-divergence
  finding.
- The eight-test integration target exercises deterministic fixture custody,
  the private handoff, interval-start mixed-onset exclusion, inactive zero,
  alias discrimination, and real bulk and multilayer consumers with mass,
  density, layer-SWE, and layer-depth closure. See
  `tests/integration/snow_wet_compaction_operand_authority.rs:80-423`.
- Fresh release execution reports `PASS` with maximum operand reconstruction
  `8.353e-17 m`, upstream mass delta `2.443e-15 m`, density-process closure
  `2.274e-13 kg m^-3`, Stage-3 closure residual `3e-17 m`, layer-SWE residual
  `4.441e-16 m`, and layer-depth residual `8.882e-16 m`. The observed maximum
  Stage-3 disposition delta (`0.00236299 m`) is correctly classified separately
  as a density-mediated routing/store/refreeze response, not folded into or
  used to waive the upstream mass-invariance gate. The scaled Snowbird lane is
  explicitly `DEVELOPMENT_ONLY` and does not support calibration, forcing
  truth, default, or transferability claims.

## Validation Evidence

- Ran: focused wet-compaction integration `8/8`; offline source-column tests
  `2/2`; production helper `1/1`; focused density replay `3/3`; focused
  SNOWDENSITY-10.3.1A `3/3`.
- Ran: low-concurrency quick profile `2181/2181`; frost profile `358/358`; full
  workspace correctness `2270/2270`; workspace doctests; `cargo fmt --check`;
  workspace/all-target Clippy with `-D warnings`; `cargo deny check`; assurance
  validation; deterministic Snowbird materializer check; authority-suite
  anti-evasion guard; and AUTH11 obligation guards all passed.
- The earlier high-contention quick attempt timed out in two assurance tests.
  It is retained as a failed receipt and is adequately reconciled by the clean
  low-concurrency quick and full-workspace executions, where the same tests
  completed within their limits.

## Residual Risk and Missing Tests

- The materiality tool's acceptance-threshold guard is currently reinforced by
  source-marker coverage. A future hardening increment should execute negative
  cases that inject each non-finite or out-of-tolerance metric and prove the
  tool rejects publication; this is nonblocking because the fresh receipt is
  hash-bound, reconstructs the operands independently, and passes every limit.
- Historical internal CoE boundary CSVs lacking the three authoritative source
  columns now fail closed. Any future archival replay will need an explicit,
  versioned migration rather than a fallback alias.
- No empirical calibration was authorized. The materiality evidence establishes
  arithmetic reachability, anti-alias behavior, and conservation, not broad
  physical calibration or transferability.
- The two touched reconciliation modules remain in the repository's line-count
  warning band (`2579` and `2723` lines). This is maintainability debt, not a
  correctness blocker for the narrow operand closure.

## Review Disposition

**GO.** The reviewed implementation closes the duplicate wet-compaction alias,
preserves the science contract and protected mass/public boundaries, reaches
both real density consumers, and has sufficient comparator-sensitive and
full-workspace validation. No blocker remains from the primary Rust/scientific
correctness review.
