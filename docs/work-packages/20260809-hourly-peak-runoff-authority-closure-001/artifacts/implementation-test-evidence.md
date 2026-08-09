# Implementation And Test Evidence

Status: `corrected focused gates passed; terminal gates pending`

Evidence mode: `Ran`

- `cargo check --workspace --tests`: PASS on the corrected post-partition
  implementation.
- `cargo nextest run --test peak_hourly_authority_contract`: PASS, 4/4.
- `cargo nextest run -p openwepp-hillslope-orchestrator --profile quick
  --no-fail-fast`: PASS, 464/464 before the final two bounded-reconciliation
  vectors; the 15-test exact filter containing those vectors then passed 15/15.
- Focused saturation-only, melt-only, runon-only, missing-shape, equal-volume
  shape, infiltrating melt, bounded hourly/daily reconciliation, material
  mismatch, sub-tolerance positive runoff, zero-capacity excess, and
  public-area scaling tests: PASS.
- Real single-OFE `p61` and multi-OFE `p102` HBP/pass-Parquet consumers: PASS.
  Both independently reconstruct maximum-hour peak from `max(V_h)/3600`;
  `p102` proves the routed outlet series.
- Runner/output quick gate reached 221/222 before the generated-watershed
  fixture's stale dry-tail assumption failed. After making its single runoff
  day the HBP latest state, the failed case passed focused execution.
- Warnings-denied Clippy for the orchestrator, runner, and hillslope-output
  crates: PASS.
- Census harness compile and four provenance/schema tests: PASS, 4/4.
- `cargo fmt --all --check` and `git diff --check`: PASS.

The first native Topanga probe found source-free positive subtraction roundoff
at days 8/112. The final correction canonicalizes only `<=1e-12 m` positive
partition residuals whose authoritative WB14 hourly-excess sum is exact zero;
positive source-backed depths remain representable and missing material timing
still hard-fails. The rebuilt one-baseline/one-mutation probe then passed on the
initial implementation. Because review subsequently corrected raw melt/runon
source assembly, an exact corrected probe remains required before the full
cohort.
