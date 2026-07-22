# Eligibility Classification

Static historical selection: `replace_string` was conservatively classified
`E-PRODUCTION` while located at
`crates/openwepp-gate-planner/src/verifier_coverage_tests.rs`. Frozen baseline
source SHA-256 was
`50ef688d9c4003d3ca8c58146bf7426bf3ebf40ac2307e610819b6470b61fecf`.

Static terminal classification: `TEST-ONLY-NON-PRODUCTION`. The source is
reachable only within `#[cfg(test)] mod tests` and now resides at
`crates/openwepp-gate-planner/src/verifier/tests/coverage_tests.rs`, SHA-256
`0976cdf9e9559609dafca49310640e8b2c9e956d649b3a397790c991e2b3857d`.
Both canonical CRAP predicates exclude exact `/src/tests/` paths before ADR row
classification. Ran: the existing exact-filter/dedup unit passed 1/1 in 0.035s.

This is not an `R-*` retention or `X-*` denominator exception. It corrects a
test-only source-role/path mismatch. Conservative raw closure still passes:
the target and helpers all have measured 100% coverage and CRAP at most 5.
