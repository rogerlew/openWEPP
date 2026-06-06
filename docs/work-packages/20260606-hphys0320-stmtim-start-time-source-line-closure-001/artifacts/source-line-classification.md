# Source-Line Classification

Status: complete

Evidence mode: Static

Static:

source_line_classification: `OPENWEPP-DEFECTIVE`

production_timing_edit_authorized: `true`

Baseline source proof:

- `/workdir/wepp-forest_260430_baseline/src/winter.for:206-235`
  - Random/breakpoint branches assign `wnttim`.
  - Breakpoint storm branch assigns `wnttim = stmstr`.
  - The subsequent minimum-hour guard normalizes any `wnttim .lt. 1.0` to
    `1.0`.
- `/workdir/wepp-forest_260430_baseline/src/stmtim.for:43-64`
  - Rounds `stmdur` to `wntdur`.
  - Adjusts end-of-day overflow.
  - Evaluates the active interval as `hour.ge.wnttim` and
    `hour.lt.(wnttim+wntdur)`.

OpenWEPP source proof:

- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`
  - Before HPHYS0320, breakpoint `stmstr` was passed through as `wnttim`
    without applying the legacy lower-bound normalization before
    `stmtim` active-interval evaluation.
  - HPHYS0320 adds `simimpl28_stmtim_start_time`, preserving finite-value
    failure and applying the baseline-authoritative `wnttim < 1.0` to `1.0`
    normalization.

Classification:

- Fixed baseline H1/H7/H39 at 2013 day 11 hour 11 records `wntdur = 11`,
  adjusted `wnttim = 1`, active interval `1`, snow branch `1`, and
  `hrsnow = 0.00074545 m`.
- Pre-fix OpenWEPP recorded `wnttim = 0`, active interval `0`, snow branch `0`,
  and `hrsnow = 0`.
- The source-line mismatch is an OpenWEPP omission in SIMIMPL28 timing
  projection, not a snow-producer, melt, or downstream water-balance defect.
