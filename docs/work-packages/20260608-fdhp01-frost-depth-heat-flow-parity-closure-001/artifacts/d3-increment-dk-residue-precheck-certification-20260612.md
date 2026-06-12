# D3 Increment Dk - Residue Pre-Check and Certification

Status: complete

Evidence mode: Static + Ran

Date: 2026-06-12

Comparator subagent: not used; user explicitly requested local comparisons
because GPT-5.3-Codex-Spark weekly quota was exhausted.

## Objective

Execute Route A from `d3-staged-increment-plan.md`: perform one bounded
residue-producer pre-check, then certify FDHP01 against the package acceptance
criteria and ADR-0017.

## Residue Pre-Check

Ran:

- Local Python comparison against the self-contained Dd pinned-baseline winter
  runs at `/tmp/fdhp01_increment_dd_legacy_winter_20260612Tdd`.
- Generated:
  - `fdhp01_increment_dk_residue_precheck_20260612.csv`
  - `fdhp01_increment_dk_residue_precheck_summary_20260612.json`

Static:

- Current openWEPP management projection reads `inrcov_seed` from
  `initial_data.base_line[5]`, not `base_line[1]`, and publishes the initial
  legacy `res_dp`-lineage value as `frost.runtime_residue_depth_m` in
  `01_management.rs`.
- Runner intake propagates `frost.runtime_residue_depth_m` into the frost
  state surface before the hourly frost coupling consumes it.
- Legacy authority: `infile.for` reads `rtyp`, `rspac`, and `width`;
  `init1.for` computes `wght1`, `rigrm`, `rilrm`, and `rmogt`; `res_dp.for`
  converts `rmogt` to `resdep`; `winter.for` writes the `resdep*1000` winter
  column before the hourly frost loop.

Result:

- OpenWEPP's current static initial projection matches the legacy first winter
  `resdep` row for all `43/43` prefixes within winter-output rounding.
  Maximum absolute difference is `0.0358593951535795 mm`; mismatch count above
  `0.1 mm` is `0`.
- The lower-residue outlier subgroup (`p1`, `p2`, `p3`, `p20`, `p21`, `p32`)
  is not undervalued by openWEPP. Each is `23.025850929940454 mm` in the
  current projection versus `23.0 mm` in the legacy frost-active residue
  group, a `0.025850929940453682 mm` rounding difference.
- Applying the legacy `infile.for` rill-width defaults does not change any of
  the 43 current initial projections, because the affected management classes
  have equal interrill/rill cover at the tested seam.
- The real residue discrepancy is lifecycle/dynamic: legacy frost consumes
  daily `resdep` recomputed from the evolving residue/decomposition state,
  while the current frost seam consumes a static initial
  `frost.runtime_residue_depth_m`. That is a residue/decomposition producer
  surface boundary, not a bounded Dk frost-seam projection fix.

Disposition:

- No production code change was made.
- No forced-snow rerun was needed for Dk because the pre-check found no
  in-envelope frost projection defect and changed no runtime behavior.
- Dynamic residue/decomposition exposure is recorded as a defect-shaped
  handoff, not fixed in FDHP01.

## Certification Basis

The certification consumes the latest clean Dj native and forced-snow cohorts
because Dk landed no code:

- Native root: `/tmp/fdhp01_increment_dj_native_cohort_20260612T205827Z`.
- Forced-snow root:
  `/tmp/fdhp01_increment_dj_forced_snow_cohort_20260612T205827Z`.
- Compact certification summary:
  `fdhp01_increment_dk_certification_summary_20260612.json`.

Ran:

- Dj native cohort: `43/43` clean, years 2-6 independent
  `Total-Soil + frozwt` closure max abs
  `6.17207992173463e-07 mm`, `0/43` profile-bound pinned, mean max depth
  `506.7933035417255 mm`, median depth correlation `0.7630792145889135`,
  median frozen-duration residual `+72 days`.
- Dj forced-snow cohort: `43/43` clean, years 2-6 closure max abs
  `5.09157033201646e-07 mm`, `0/43` profile-bound pinned, mean max depth
  `501.3624240499244 mm`, median max `492.3588252690888 mm`, median depth
  correlation `0.7635554124345166`, median frozen-duration residual
  `+61 days`.
- Frost activation is non-regressed: all 43 forced-snow prefixes have
  `frdp > 0` days and matching nonzero `frozwt` days.

ADR-0017 judgement:

- The legacy `240..503.2 mm` cohort envelope is a flag/reference, not a
  millimetre target.
- FDHP01's original defect was a freeze-index proxy: `200 mm` cap, median
  depth correlation `0.13`, and `+258` frozen-day duration residual.
- Current Dk/Dj evidence has retired the proxy cap, removed profile-bound
  pinning, preserved the repaired D2 additive storage identity, moved cohort
  mean/median maximum depth into the physical heat-flow range, raised median
  depth correlation to about `0.76`, and collapsed the forced-snow duration
  residual to `+61 days`.
- The stable `13/43` forced-snow upper-envelope outlier set is characterized,
  not a reason to keep grinding under the protected comparator-tuning
  boundary.

## Handoffs

- F4 snow density/depth-split magnitude review, priced by the native-vs-forced
  gap.
- `p2` individual attribution: ratio `2.18`, outside the subgroup patterns.
- Dynamic residue/decomposition lifecycle surface: expose or port daily
  `resdep` when the residue/decomposition contract owns that producer.
- Residual roughly 25% subgroup depth deltas recorded as characterized model
  difference, reopenable on new source-line evidence.

## Disposition

FDHP01 is complete at the declared single-OFE boundary. The closed scope is the
frost state machine, frozen-water exchange/publication, and frost heat-flow
energetics for single-OFE. MOFE is unblocked as the next ROADMAP mechanism.
