# Coverage After

Evidence label: Static/Ran.

Status: `FOCUSED-PASS`; full-workspace after-metrics delegated to
`comparator_suite_runner`.

Focused artifact: `/tmp/openwepp-cqr-nightly-03-snowbench-focused.lcov`

Focused target LCOV:

- `LF:487`
- `LH:426`
- Line coverage: `426 / 487 = 87.47433264887063%`

ADR-0021 glue-tier line threshold: `>= 85%` - `PASS`.

LCOV does not include region coverage. Focused closure uses LCOV line coverage
plus cargo-crap per-function coverage/CRAP as the measurable branch-sensitive
surrogate for this CLI glue package; full-workspace after-metrics are delegated
to `comparator_suite_runner`.
