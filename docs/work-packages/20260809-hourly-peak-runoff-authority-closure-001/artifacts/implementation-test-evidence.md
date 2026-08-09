# Implementation And Test Evidence

Status: `focused gates passed; terminal gates pending`

Evidence mode: `Ran`

- `cargo check --workspace --tests`: PASS.
- `cargo nextest run --test peak_hourly_authority_contract`: PASS, 4/4.
- `cargo nextest run -p openwepp-hillslope-orchestrator`: PASS, 457/457
  before the final source-informed roundoff tests; all subsequently added peak
  tests passed focused execution.
- Focused saturation-only, melt-only, runon-only, missing-shape, equal-volume
  shape, sub-tolerance positive runoff, zero-capacity excess, and public-area
  scaling tests: PASS.
- `cargo fmt --all --check` and `git diff --check`: PASS.

The first native Topanga probe found source-free positive subtraction roundoff
at days 8/112. The final correction canonicalizes only `<=1e-12 m` positive
partition residuals whose authoritative WB14 hourly-excess sum is exact zero;
positive source-backed depths remain representable and missing material timing
still hard-fails. The rebuilt one-baseline/one-mutation probe then passed.
