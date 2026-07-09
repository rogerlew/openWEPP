# Coverage Closure

Evidence label: Static/Ran.

Status: `HOLD-NOT-MET`

ADR-0021 tier: `science`, because the target owns WS20/WS21 channel sediment
routing math, typed guards, and contract-bound output/diagnostic behavior.

Closure rule:

- If characterization tests are added or materially changed, record line and
  region coverage status for the target, per-function 75% region-floor status or
  disposition, and obligation-to-test binding before completion.
- Because LCOV does not provide region coverage, record available line coverage
  plus a branch-sensitive CRAP surrogate and explicit per-function disposition.

Current focused status:

- Baseline LCOV: `LF:934`, `LH:0`, `0.0%`.
- Focused after LCOV: `LF:1364`, `LH:817`, `59.89736070381232%`.
- Science-tier 90% line threshold: `NOT MET`.
- Region coverage: `NOT AVAILABLE` from LCOV.
- Provisional CRAP closure before rollback: max target CRAP `30.0`, rows above
  `30`: `0`.
- Obligation-to-test binding: no new `SC-*` obligation was introduced. Tests
  bind private helper behavior and typed guard classes to existing WS20/WS21
  route/sediment behavior under `SC-ROUTE-001` and `SC-SED-001`.

Review disposition:

- Accepted review finding: ADR-0021 binds packages that add or materially change
  tests, and this science-tier target did not meet the `>=90%` line/region
  threshold or per-function region-floor evidence.
- Accepted review finding: provisional characterization did not cover key
  refactored case34/case4 paths.
- This package therefore closes in local hold, not completion. The provisional
  implementation and tests were rolled back; no CRAP closure is claimed for the
  current tree.
