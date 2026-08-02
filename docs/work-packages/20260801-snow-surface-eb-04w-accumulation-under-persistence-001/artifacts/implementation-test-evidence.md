# Implementation And Test Evidence

Evidence mode: **Ran**.

| Surface | Result |
|---|---|
| EB-04W, EB-04V, and EB-03 focused integration targets | `15/15` passed |
| phase dry/active semantic unit test | `1/1` passed; 426 filtered |
| retained per-day canopy CoE snowbench replay | `1/1` passed after phase-adapter correction |
| non-100 kg/m³ snowbench phase/runtime regression | `1/1` passed; exact runtime snowfall-depth-to-SWE operand closes and replay remains executable |
| affected-crate all-target Clippy with warnings denied | passed |
| Rustfmt check | passed |
| unit registry | `21/21` passed |
| assurance source-adoption render check and catalog validation | passed |
| `cargo deny check` | passed |
| Markdown | recorded terminal package lint: 34 files with zero findings; closure re-lint: 41 files with zero findings; edited roadmap/catalog files pass individually; full docs inventory retains 15 pre-existing broken-link errors outside the write set |
| exact release cohort | 16/16 cells and 5/5 operators completed under final binary `b50dd71c…` |
| exact retained-output comparison | all 245,456 WAT rows and every prior v2 field across 245,456 trace rows matched exactly |
| quick workspace profile | `2143/2143` passed; 37 skipped; 55 slow; run `c2dcae3b-93b8-42b8-8522-8f3cd2d64384`; `2208.697 s`; exit `0` |
| frost workspace profile | `341/341` passed; 1,893 skipped; 1 slow; run `48827898-ee6e-44ca-8312-30ba4d457835`; `529.018 s`; exit `0` |
| full workspace profile | `2192/2192` passed; 30 skipped; 31 slow; run `803132f9-663e-439c-9055-7a921d45bc49`; `2216.531 s`; exit `0` |

The first terminal frost attempt identified an actual compatibility defect: the
snowbench CoE adapter populated rain/snow depths but left newly required phase
fractions at their dry default. The correction derives fractions from those
already classified hourly water operands, changes no modeled water input, and
the focused replay passes. Failed pre-fix terminal logs are diagnostic history,
not reused as terminal evidence.

Quick, frost, and full exact-head suite commands, run identifiers, counts,
durations, exits, and logs are retained in `terminal-suite-summary.md` and
`summary.json`.
