# Verification Agent B

Status: complete.
Evidence mode: Ran.

Verification focus: package boundary, default-disabled regression, and protected
identity.

Ran:

- direct-runtime forbidden-token scan
  - PASS, no compatibility storage/request/writeback tokens found.
- scheduler no-diff check
  - PASS, no diff in
    `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`.
- release build for `openwepp-cli-hill`
  - PASS, elapsed `57.96 s`, RSS `1088060 KB`.
- default-disabled H2637 benchmark
  - PASS: `644.01 / 646.84 / 643.66 s`, median `644.01 s`.
- protected output identity
  - PASS for HBP, loss, WAT, plot checksums.
- PASS parquet row/schema equivalence
  - PASS: `left_minus_right = 0`, `right_minus_left = 0`, rows
    `12419 / 12419`, schema 17 columns.

Result: PASS.
