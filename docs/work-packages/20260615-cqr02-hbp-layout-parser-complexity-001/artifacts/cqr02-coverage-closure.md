# Coverage Closure

Status: complete
Evidence mode: Ran

Raw reports:

- Before: `coverage_before_summary.json`, `lcov_before.info`
- After: `coverage_after_summary.json`, `lcov_after.info`

Target summary from raw workspace exports:

| State | Functions | Lines | Regions |
| --- | ---: | ---: | ---: |
| Before export | 3/84 = 3.571428571428571% | 596/838 = 71.1217183770883% | 728/1119 = 65.05808757819482% |
| After refactor | 59/66 = 89.39393939393939% | 893/936 = 95.40598290598292% | 1031/1132 = 91.07773851590106% |

Additional pre-refactor characterization was added after the first full before export and before production source edits. Focused pre-refactor coverage for `layout_parser.rs` reached 87.82816229116943% line and 77.03306523681859% region before decomposition. After decomposition, the full workspace report satisfies glue-tier line and region thresholds.
