# CQR32 Coverage Closure

Ran: before and after LCOV were generated from full workspace
`cargo llvm-cov --workspace --ignore-run-fail` runs.

## Target File Coverage

| Metric | Before | After |
|---|---:|---:|
| Functions found | 33 | 34 |
| Functions hit | 28 | 31 |
| Lines found | 577 | 580 |
| Lines hit | 400 | 470 |
| Function coverage | 84.848484848485% | 91.176470588235% |
| Line coverage | 69.324090121317% | 81.034482758621% |

Result: no target-file coverage regression. Line coverage improved by
`11.710392637304` percentage points. Focused target/helper display coverage
reached `100%` for `ClimateParseError::fmt`,
`ClimateParseError::write_display`, and `ClimateParseError::source`.

Warn: target-file line coverage remains below the ADR-0021 glue-tier `85%`
threshold. This CQR package is scoped to the ranked `ClimateParseError::fmt`
CRAP row, not full module test-enhancement closure.

## Characterization

Ran: `cargo test --test infile_climate_parser_contract --no-fail-fast`
after adding display/source characterization and after production refactor.

Result: exit `0`, `21` passed.
