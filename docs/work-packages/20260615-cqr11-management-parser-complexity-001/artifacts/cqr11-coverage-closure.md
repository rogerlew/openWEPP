# CQR11 Coverage Closure

Status: complete-with-warnings.

Static: coverage closure is scoped to the CQR11 target-file and focused
perennial parser branches.

Ran: before LCOV recorded target-file coverage of `608/1068` lines and `32/42`
functions.

Ran: after LCOV recorded target-file coverage of `749/1114` lines and `40/49`
functions.

Coverage comparison:

| Metric | Before | After | Delta |
| --- | ---: | ---: | ---: |
| Covered lines | 608 | 749 | +141 |
| Instrumented lines | 1068 | 1114 | +46 |
| Covered functions | 32 | 40 | +8 |
| Instrumented functions | 42 | 49 | +7 |

Ran: focused characterization command passed before production refactor:

```console
cargo test --test infile_management_parser_contract perennial -- --nocapture
```

Result: exit `0`, `9` passed.

Ran: the same focused command passed after production refactor and formatting:
exit `0`, `9` passed.

WARN: target-file line coverage remains below the science-tier threshold from
`docs/decisions/0021-module-coverage-closure-thresholds.md`; this package does
not claim target-file coverage closure beyond the scoped CQR target.
