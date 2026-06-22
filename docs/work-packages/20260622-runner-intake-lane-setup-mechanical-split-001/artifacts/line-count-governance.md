# Line-Count Governance

Evidence class: Static + Ran.

## Pre-Refactor Baseline

Ran at scaffold:

```text
wc -l crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs \
  crates/openwepp-runner/src/hillslope/04_direct_publication.rs \
  crates/openwepp-runner/src/hillslope/mod.rs
```

Result:

```text
  2997 crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs
  1919 crates/openwepp-runner/src/hillslope/04_direct_publication.rs
     9 crates/openwepp-runner/src/hillslope/mod.rs
  4925 total
```

`00_runner_intake_and_lane_setup.rs` was below the hard 3000-line block but
above the 2000-line warning threshold.

## Post-Refactor Touched Rust Files

Ran:

```text
wc -l \
  crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs \
  crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs \
  crates/openwepp-runner/src/hillslope/mod.rs \
  tests/integration/mofe01_per_ofe_state_contract.rs \
  tests/integration/mofe01_inter_ofe_route_contract.rs
```

Result:

```text
  1741 crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs
  1255 crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs
    10 crates/openwepp-runner/src/hillslope/mod.rs
   454 tests/integration/mofe01_per_ofe_state_contract.rs
   140 tests/integration/mofe01_inter_ofe_route_contract.rs
  3600 total
```

## Disposition

- `00_runner_intake_and_lane_setup.rs`: reduced from `2997` to `1741` lines;
  below WARN and below hard block.
- `05_runner_execution_and_outputs.rs`: `1255` lines; below WARN and below
  hard block.
- All other touched Rust files remain below WARN and hard block thresholds.
- No touched `.rs` file is at or above `3000` lines.
