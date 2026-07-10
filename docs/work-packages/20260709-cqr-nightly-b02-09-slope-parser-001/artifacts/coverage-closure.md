# ADR-0021 Coverage Closure

Tier: glue.

Status: PASS.

Evidence:

- Final focused LCOV: `/tmp/openwepp-cqr-b02-t09-focused2.lcov`
- Final CRAP JSON: `/tmp/openwepp-cqr-b02-t09-focused2-crap.json`
- Final LLVM export:
  `/tmp/openwepp-cqr-b02-t09-final3-llvm-export.json`
- Lines: 628/677, 92.7622%.
- Regions: 668/728, 91.7582%.
- Functions: 39/44, 88.6364%.
- CRAP rows above 30: 0.

Per-function floor:

- Original high-CRAP functions exceed the 75% floor:
  `slope_parser_error_message` 100% and `parse_slope_str` 95.6522%.
- Extracted parser-orchestration helpers are covered by public
  `parse_slope_str` and `parse_slope_file` characterization.
- `prefer_shared_geometry_when_trailing_tokens` has cargo-crap function
  coverage 57.1429% and CRAP 6.9679. The covered branch is the compatibility
  shared-form preference used by public parser input; the uncovered early-return
  guard combinations retain the same pre-refactor no-op behavior.
- `verify_cross_ofe_boundary_continuity` has cargo-crap function coverage
  66.6667% and CRAP 5.9259. Public tests cover strict cross-OFE mismatch and
  compatibility permissive behavior; uncovered invariant arms require internally
  malformed parsed `SlopeOfe` values after prior parser guards have already
  constructed non-empty point vectors.
- These low-function-coverage helpers are reviewed coverage-floor exceptions;
  module-level glue-tier line/region floors and the CRAP bound pass.
