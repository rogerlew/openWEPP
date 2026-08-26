# Line-count governance

Status: `PASS WITH DOCUMENTED WARN`.

Static final counts:

- `snow_terminal_phase_trajectory.rs`: 881 lines, below 2,000 WARN;
- `snow_stage3_v11_terminal_execution.rs`: 1,939 lines, below 2,000 WARN;
- `v9_real_consumer_shadow_wb14_tests.rs`: 2,680 lines, above 2,000 WARN and
  below 3,000 BLOCK.

The 96-line real-fixture adapter was placed in the existing test-only fixture
because it reuses its private complete-owner values. The WARN is accepted for
this rejected research checkpoint. Any reuse or extension must first split the
trajectory evidence into its own fixture module; no production implementation
may grow this file.
