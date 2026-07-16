# Groundwater Current-Tree Confirmation

Evidence class: Ran

Execution date: 2026-07-15 UTC

Source root: `773eb3c56f0afcbc7f605d49c9a09d391e8113a5`

The working tree contained documentation-only ASSURE-02 changes. No Rust file
was changed.

## Frozen-To-Intake Path Comparison

Command:

```bash
git diff --name-only \
  de520f1ff867ca5c65b1f82dfe32a19c213ae18c..\
773eb3c56f0afcbc7f605d49c9a09d391e8113a5 -- \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/groundwater.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs \
  crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs \
  crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs \
  crates/openwepp-runner/src/hillslope/03_tests.rs \
  crates/openwepp-runner/src/hillslope/04_direct_publication.rs \
  crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs \
  crates/openwepp-input-contract/src/parsers/hbp/types.rs \
  crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs \
  tests/integration/wshedw5_typed_watershed_runtime_contract.rs \
  tests/integration/laned_shadow_h2637.rs
```

Result: exit `0`; standard output empty. Therefore exactly these twelve
declared paths have no committed difference between the frozen assessed commit
and the intake commit. This is a static currency check, not a fresh integrated
release transfer.

## Focused Executable Confirmation

Command:

```bash
cargo nextest run --workspace --profile quick \
  -E 'test(/gwbaseflow|r6a_direct_hbp_writer_serializes_groundwater_payload_operands/)'
```

Exit: `0`

Nextest output:

```text
Nextest run ID 1f5af549-b12e-4ca2-afd0-2b4b48734fc8
Nextest profile: quick
Starting 7 tests across 170 binaries
Summary: 7 tests run: 7 passed, 1930 skipped
```

The selected tests were:

- `openwepp-hillslope-orchestrator`
  `gwbaseflow_exports_over_accepted_storage_fail_closed`;
- `openwepp-hillslope-orchestrator`
  `gwbaseflow_linear_reservoir_recurrence_uses_prior_day_exports`;
- `openwepp-hillslope-orchestrator`
  `gwbaseflow_mofe_recharge_aggregates_lane_deep_percolation`;
- integration target `wshedw5_typed_watershed_runtime_contract`
  `gwbaseflow_bftharea_suppresses_below_threshold_side_baseflow`;
- integration target `wshedw5_typed_watershed_runtime_contract`
  `gwbaseflow_generated_hbp_payload_without_gwcoeff_authority_fails_closed`;
- integration target `wshedw5_typed_watershed_runtime_contract`
  `gwbaseflow_lr_bf1_channel_branch_consumes_generated_hbp_not_cbase`; and
- `openwepp-runner`
  `r6a_direct_hbp_writer_serializes_groundwater_payload_operands`.

This focused confirmation does not replace a fresh H2637 run or the full
release-candidate suite.

## Independent Two-Day Floating-Point Reconstruction

ASSURE-02 independently applied the published operation order using IEEE-754
binary64 arithmetic. The observed values and their absolute differences from
the decimal test referents were:

```text
day 1: S=12.0, Qb=1.2000000000000002, Qs=0.6000000000000001
       absolute differences: 0.0, 2.220446049250313e-16,
       1.1102230246251565e-16 m3
day 2: S=14.200000000000001, Qb=1.4200000000000002,
       Qs=0.7100000000000001
       absolute differences: 1.7763568394002505e-15,
       2.220446049250313e-16, 1.1102230246251565e-16 m3
```

The maximum observed absolute residual was
`1.7763568394002505e-15 m3`, below the implementation-test allowance of
`1.0e-12 m3`. This calculation characterizes floating-point agreement for the
analytical recurrence vector; it is not environmental-model error.
