# CQR33 Coverage Closure

Ran: before and after LCOV were generated from full workspace
`cargo llvm-cov --workspace --ignore-run-fail` runs.

## Target File Coverage

| Metric | Before | After |
|---|---:|---:|
| Functions found | 20 | 21 |
| Functions hit | 14 | 17 |
| Lines found | 450 | 453 |
| Lines hit | 330 | 412 |
| Function coverage | 70.000000000000% | 80.952380952381% |
| Line coverage | 73.333333333333% | 90.949227373068% |

Result: no target-file coverage regression. Line coverage improved by
`17.615894039735` percentage points and is above the ADR-0021 glue-tier `85%`
threshold.

## Characterization

Ran: `cargo test --test infile_watershed_structure_parser_contract --no-fail-fast`
after adding display/source characterization and after production refactor.

Result: exit `0`, `20` passed.
