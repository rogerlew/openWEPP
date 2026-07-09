# Coverage Closure

Status: `COMPLETE`

Characterization tests were materially expanded, so ADR-0021 coverage closure is
required.

Tier assignment:

- `science`: the target is a kernel-contract typed boundary surface used by
  runtime symbol projection.

Threshold status:

- Before: target CRAP rows reported low function coverage for the hillslope
  state/flux symbol conversion and irrigation field suffix mapping.
- First after measurement: delegated `/tmp/openwepp-cqr-nightly-01-after.lcov`
  and `/tmp/openwepp-cqr-nightly-01-after-crap.json` reported `0` target
  functions above CRAP `30`.
- Review B found first after measurement did not prove ADR-0021 line/region
  threshold status and sampled file line coverage below the science-tier
  threshold.
- Additional tests were added for the uncovered eligible rows and remaining
  source spans. Refreshed final2 evidence closes the science-tier threshold:
  - line coverage: `278 / 284 = 97.88732394366197%` from
    `/tmp/openwepp-cqr-nightly-01-final2.lcov`;
  - region coverage: `332 / 338 = 98.22485207100591%` from
    `/tmp/openwepp-cqr-nightly-01-final2-full.json`, de-duplicating duplicate
    monomorphized/source-span copies by taking the maximum hit count for each
    unique source region span;
  - CRAP replay: `0` deduplicated target rows above `30` and `0` target rows
    below cargo-crap coverage `75`.
- Package-local extraction log:
  `artifacts/logs/final-current-coverage-metrics.log`, SHA-256
  `d1d33852232ba2825fd0ba40eaad821eae219d2480eb15df42515be412a8c0ec`,
  `__EXIT_CODE__:0`. It records the LCOV `awk` extraction, full-JSON
  source-region `jq` de-duplication, and target CRAP-over-30 `jq` check.

Obligation-to-test binding:

- Existing ARCH22 typed boundary tests continue to bind representative
  hillslope and watershed symbol projection behavior.
- This package adds exhaustive hillslope state, hillslope flux, dynamic
  irrigation field suffix, climate forcing error display, and watershed
  impoundment field suffix coverage for the target CRAP rows and reviewed floor
  gaps.
- Additional final characterization binds climate forcing accessors, watershed
  channel state/flux suffixes, `Nchnum`, and watershed hillslope particle
  diameter projection.
- No `SC-*` contract text was changed; this is a behavior-preserving typed
  symbol-string CQR package.

Per-function region-floor disposition:

- PASS. Deduplicated full-JSON source-function regions show `0` functions below
  the ADR-0021 `75%` per-function floor and `0` functions below `90%`.
- The private helper `_ => unreachable!` arms are not counted as ADR-0021
  coverage exclusions. They remain documented caller-invariant guards behind the
  exhaustive public `From<HillslopeProductionStateSymbol>` route.
