# Implementation Test Evidence

Ran: complete.

## Implementation Summary

R5C added direct-runtime ownership for:

- `DecompositionTransition`, implemented in
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/decomposition.rs`
  with typed active context, action selector, PL17 seed-pool equation update,
  event-domain guards, state mutation, downstream operands, and shadow
  projection.
- `ResiduePartitionTransition`, implemented in the same module with typed
  partition inputs, decomposition downstream consumption, state mutation,
  downstream operands, and shadow projection.

Executor wiring now runs both spans after R5B `StorageBounds` and before the R4
hydrology spans. Lifecycle status now reports only `AnnualGrowthTransition` and
`PerennialGrowthTransition` as hold phases.

## Focused Tests

Ran:

```text
cargo test -p openwepp-hillslope-orchestrator r5c_ -- --nocapture
```

Result: PASS, 5 tests.

Covered:

- annual/fallow cut branch direct compute, state mutation, downstream operands,
  shadow projection, and anti-alias assertions;
- perennial grazing branch with zero-decay constants;
- missing R5B upstream failure;
- missing/ambiguous active context failure;
- invalid action/fraction/pool guards;
- residue partition downstream consumption, projection, anti-alias assertions,
  and missing-upstream/invalid-input guards.

Ran:

```text
cargo test -p openwepp-hillslope-orchestrator direct_runtime -- --nocapture
```

Result: PASS, 50 tests.

Ran:

```text
cargo test -p openwepp-runner r2a_ -- --nocapture
```

Result: PASS, 2 tests. Default compatibility fixture recorded zero direct
runtime counters; explicit opt-in direct skeleton fixture recorded updated R5C
span/entry counts and one production compatibility-edge handoff.
