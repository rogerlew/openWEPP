# Medium Gate Results

Evidence class: **Ran + Static**

Status: `PASS`

## Authoritative Gates

Coverage measurement was source-bound to commit
`69822725a696d94a63d53fe36cdb74e4a02b95ad`. The authoritative ordinary gates
were source-bound to closure commit `553647f0b2577f1ab286f89e50e791bdf2a30b46`
after the two semantically neutral, test-only Clippy corrections recorded in
`final-metrics.md`. That record also discloses the documentation-only dirty
state at measurement and the untracked evidence paths present during gates.

| Gate | Exit | Elapsed | Max RSS | Result |
| --- | ---: | ---: | ---: | --- |
| `cargo fmt --check` | 0 | `0:02.09` | 69,836 KB | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 | `0:01.77` | 186,176 KB | PASS |
| `cargo nextest run --workspace --profile quick` | 0 | `2:32.32` | 209,608 KB | PASS: 1,851/1,851; 28 skipped; three slow |
| `cargo nextest run --workspace --profile full` | 0 | `9:39.73` | 209,480 KB | PASS: 1,930/1,930; three skipped; four slow |
| `cargo deny check` | 0 | `0:01.40` | 77,292 KB | PASS |
| Exact campaign `markdown-doc lint` | 0 | `0:00.03` | 9,600 KB | PASS: 65 files; zero errors/warnings |
| `git diff --check` | 0 | `<0.1 s` | 15,360 KB | PASS |

The four slow full-profile tests were the three Iwagaki oracle/reconciliation
tests and
`snowdensity05e_melt_adjudication::coe_melt_snowbench_runs_both_models_as_diagnostic_only`.
No ordinary full-profile test failed. The final ordinary execution therefore
distinguishes the four instrumented-only failures in `final-metrics.md` from
product or test regressions.

The parent checkpoint also reports the final affected focused cohort passing
56/56. Module records preserve the focused command evidence; the authoritative
workspace quick and full results above provide tranche-level closure.

## Clippy Correction Record

The first all-target Clippy attempt found exactly two test-only lints:

- inefficient assignment of a cloned replacement string in
  `infile_climate_parser_contract.rs`; and
- the intentionally exhaustive 112-line M-07 error-priority matrix exceeding
  Clippy's 100-line test-function threshold.

The accepted correction used `clone_into` and added a narrow
`#[allow(clippy::too_many_lines)]` on that test only. The initial failure log
and timing remain at `final/gates/clippy-initial-fail.{log,time}`. The
authoritative clean-commit Clippy rerun passed.

## Documentation Scope And Durable Logs

The exact campaign Markdown command scans the roadmap, campaign catalog,
assessment, baseline, execution contract, all four tranche plans, and the full
campaign evidence tree. No separate active module-package directory exists:
the revised execution model uses compact `medium/modules/M-*.md` checkpoint
records, which are included through the evidence-tree path.

All authoritative logs and `/usr/bin/time -v` records are archived in
`medium/final/gates/`. The provisional quick run and initial Clippy failure are
retained with explicit filenames and are not substituted for the authoritative
clean-commit results.

Disposition: all required Medium tranche-final gates `PASS`; no gate is
deferred.
