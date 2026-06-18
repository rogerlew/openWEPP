# PERFARRAY02 Review A

Evidence: Static + Ran.

## Findings

1. `Accepted`: The scoped array request/accessor seam is behaviorally safe but does
   not meet the performance target. H2637 array-native measured `817.810 us/OFE-day`,
   above the `386 us/OFE-day` <=10x budget.
2. `Accepted`: Boundary conversion remains expensive: `1685.023 us/OFE-day` for seed,
   export, and reindex. This is reported separately and is not hidden in the floor.
3. `Accepted`: The pilot still converts a logical kernel writeback payload to
   `ArrayWritebackPayload`. This is in scope for PERFARRAY02 but is visible in perf and
   contributes to the missed floor.

## Checks

Static:

- default path passes `None` for array state;
- array pilot passes empty logical maps to the kernel request;
- logical apply is mutually exclusive with array apply for the piloted phase;
- pass parquet byte churn is handled by row-level comparison.

Ran:

- OFE5 identity passed;
- H2637 identity passed;
- `cargo clippy --workspace --all-targets -- -D warnings` passed;
- `cargo test --workspace` passed;
- `cargo deny check` passed.

Disposition: NO-GO is the correct outcome. Do not ratify ADR-0023 from this pilot.
