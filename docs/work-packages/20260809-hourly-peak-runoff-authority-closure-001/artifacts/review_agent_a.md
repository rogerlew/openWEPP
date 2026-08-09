# Independent Hydrology / Science Re-Review A

Status: `executed`

Reviewed identity: `949349e7055c5d19277eeb708401c4614a52cd77`

Evidence class:

- `Static`: exact-commit source, contracts, tests, package artifacts, and
  publication/interchange consumer inspection.
- `Ran`: `cargo nextest run --test erosion_single_ofe_p61_sediment --test
  erosion_multi_ofe_p102_chain --profile quick` at the reviewed identity;
  2 tests passed, 0 skipped, nextest run
  `848d6234-80b0-436a-8cb9-2a517c2c306e`.
- `Ran (package-recorded, not independently rerun in this review)`: the focused
  corrected gates listed in `artifacts/implementation-test-evidence.md`.

Verdict: `PASS`

## Findings

No remaining Critical, Major, or Minor hydrology/science finding.

## Prior Finding Resolution

### `REVIEW-A-CRITICAL-001` — resolved

Routed melt and inter-OFE runon are now producer-timed liquid supplies admitted
to WB14 before infiltration and depression-storage partitioning, not raw
post-partition runoff limbs. The runner adds routed melt to
`hourly_additional_supply_m`; the direct runtime adds runon to the same input,
and WB14 produces the only post-partition hourly excess ledger
(`crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:651-733,
1743-1769,1778-1896`). The melt vectors prove both zero-capacity in-hour runoff
and complete infiltration without a fabricated runoff limb
(`crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_dc01.rs:50-111`).

The peak and shared transfer weights consume only WB14 post-partition depths plus
WB19 saturation return. `closing_hourly_runoff_depths_m` independently sums
those depths; `ensure_hourly_runoff_source_closure` rejects mismatch against
daily `Q`; only after closure may normalized weights be derived
(`direct_runtime/runoff.rs:1498-1555,1572-1614`). The peak itself is calculated
directly from the closing hourly depths, not from `Q * normalized_weight`
(`direct_runtime/runoff.rs:1616-1650`). Material mismatch has a negative vector
at `direct_runtime_dc01.rs:256-276`.

`SC-WATBAL-001` version 167 accurately owns this post-partition boundary in
`q_hourly`, `INV-WATBAL-102/103`, and `TOL-WATBAL-009`
(`docs/specifications/science-contracts/contracts/SC-WATBAL-001.md:127,
296-297,873`). The tolerance is bounded to the documented 24 interval-ledger
arithmetic allowance; material mismatch hard-fails, and the within-bound
residual is applied once to an existing largest runoff bin rather than creating
a new source hour (`direct_runtime/runoff.rs:1432-1495`).

### `REVIEW-A-MAJOR-002` — resolved

The source-free positive-roundoff decision is now evaluated against the WB14
post-partition ledger after melt and runon have already entered WB14 exactly
once. Therefore a positive melt- or runon-backed runoff necessarily appears in
`wb14_hourly_excess_m` and is not source-free. The canonicalizer preserves every
positive WB14-backed value, while zero-source sub-tolerance arithmetic becomes
exact zero (`direct_runtime/runoff.rs:778-810,1409-1422`;
`direct_runtime_dc01.rs:173-202`). `TOL-WATBAL-009` separately reconciles the
independent daily/hourly accumulations without authorizing loss of a positive
hourly source.

### `REVIEW-A-MAJOR-003` — resolved

No uniform runon fallback remains. Positive surface runon with a zero hourly
surface shape and positive lateral runon with a zero lateral carry both return
typed `MissingDirectUpstream`; valid shapes retain their produced-hour weights
before entering WB14 (`direct_runtime/runoff.rs:694-733`;
`direct_runtime_dc01.rs:1-48`). This closes the multi-OFE source-custody defect.

## Physical And Consumer Acceptance

- WB19 saturation-return water is added only from
  `hourly_saturation_carry_m[hour]` and therefore remains in its produced hour;
  the saturation-only vector selects hour 11 exactly
  (`direct_runtime/runoff.rs:855-865,1572-1595`;
  `direct_runtime_dc01.rs:113-128`).
- The internal quantity is the maximum one-hour mean depth rate in `m s^-1`,
  with no instantaneous/subhourly claim. Rectangular-equivalent duration remains
  `Q / peak_depth_rate` and is not rainfall duration, hydrograph duration, or
  time to peak.
- Publication validates a positive area, adjusts only to the published runoff
  depth basis, and applies area once to obtain `m^3 s^-1`
  (`crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs:576-640`).
- `SC-INFILE-HBP-001` version 0.2.4 correctly defines minor-1 peak as
  `max(hourly_runoff_volume_m3)/3600`, requires hourly-volume sum to equal
  `runvol`, and binds duration to `runvol/peak` without another area multiplier
  (`docs/specifications/science-contracts/contracts/SC-INFILE-HBP-001.md:114-124,
  238-246`).
- Ran: the real single-OFE p61 consumer independently reconstructed hourly
  volume, pass-Parquet/HBP peak equality, and rectangular duration. The real
  routed multi-OFE p102 consumer independently reconstructed outlet `runvol`,
  maximum-hour HBP peak, and pass-Parquet peak from the routed outlet hourly
  series (`tests/integration/erosion_single_ofe_p61_sediment.rs:155-191`;
  `tests/integration/erosion_multi_ofe_p102_chain.rs:55-110`). Both tests passed
  in this re-review.
- Package claims remain hillslope-scale, non-calibrated, non-watershed-routing,
  and maximum-hourly-mean only. No observed-flow validation, legacy parity, or
  instantaneous-peak claim was found.

The prior closure blockers are resolved at the exact reviewed commit. The
hourly/daily mass ledger, source timing, units, area conversion, and real
consumer evidence support `PASS`.
