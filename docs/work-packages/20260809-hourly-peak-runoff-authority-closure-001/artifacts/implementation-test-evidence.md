# Implementation And Test Evidence

Status: `PASS`

Evidence mode: `Ran`

- `cargo check --workspace --tests`: PASS on the corrected post-partition
  implementation.
- `cargo nextest run --test peak_hourly_authority_contract`: PASS, 4/4.
  The guard now also binds `SC-SED-001` rev63, rejects the retired analytical
  peak and cross-dimensional duration tolerance, and requires the internal
  depth-rate/public-area boundary.
- Named erosion duration-custody guard: PASS, 1/1. The behavioral vector tests
  residuals below and above the absolute `1.001e-9 s` threshold at expected
  durations of 0.25, 10, and 80,000 seconds; the source-level contract guard
  binds the same named live constant.
- Owning orchestrator full crate suite: PASS, 472/472 after the terminal
  pure-melt regression.
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
- Census harness provenance/schema tests: PASS, 6/6.
- `cargo fmt --all --check` and `git diff --check`: PASS.
- `cargo nextest run --test laned_shadow_h2637 --run-ignored all`: PASS,
  10/10 including both long-running ignored proofs. The frost-free seam
  isolates routing without altering precipitation/routing inputs; all 731
  warmed days route and none use a uniform shape.

The first native Topanga probe found source-free positive subtraction roundoff
at days 8/112. The final correction clears a residual only when reconciled
partition runoff is exact zero; positive source-backed depths remain
representable, and positive frost-adjusted runoff without a producer-timed
shape hard-fails. The exact corrected probe and complete 1,088-trial cohort
both passed. See `mutation-study.md` for the complete-cohort provenance and
metrics.

## Reopened ADR-0036 Closure Evidence

Evidence mode: `Ran + retained identity evidence`

The semantic ADR/source-guard increment is `669269ee4`; review/editorial
reconciliation is `a8a96498`. Exact-source evidence at `a8a96498` passed:

- full workspace 2,346/2,346, 46 slow, 33 skipped, 8,193.187 seconds, run ID
  `64cd5e97-d253-4da1-a3cf-3c4e16f83d22`;
- peak authority 4/4;
- required-suite obligation guard 3/3;
- workspace doctests, `cargo deny check`, format, and authority anti-evasion.

The retained quick/full inventories contain 2,297/2,346 identities and zero
quick-only identities. The admitted full receipt therefore covers every
quick-selected test. All interrupted/setup attempts in `reopen-gate-runlog.txt`
and `reopen-gate-manifest.tsv` are non-admitted.

The reopened delta changes no runtime, canonical SC-* contract, serialization,
release binary, or Topanga input. The admitted runtime/cohort evidence remains
bound to `33831787b` and is reused, not reported as newly executed.
