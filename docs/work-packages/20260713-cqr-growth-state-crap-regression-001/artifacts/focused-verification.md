# Focused Verification

Evidence mode: Ran.

Terminal-source focused results on 2026-07-13:

| Command | Result |
| --- | --- |
| `cargo fmt --check` | PASS |
| `cargo nextest run -p openwepp-hillslope-orchestrator --lib intval_zero_perennial_root_cap_uses_saturated_branch_before_division` | PASS, 1/1 |
| `cargo nextest run -p openwepp-hillslope-orchestrator --lib` | PASS, 405/405, 3 slow, 0 skipped |
| `git diff --check` | PASS |

An intermediate substring invocation using
`direct_runtime::growth::tests` selected zero tests and exited 4. It is not
claimed as verification and was superseded by the complete 405-test crate
library run above.

The full crate run includes the row-6 growth characterization tests, the exact
zero-cap `INV-PLANT-027` vector, and the annual/perennial direct-runtime tests.
Fresh full-workspace CRAP and closure results remain delegated to the package's
runner.
