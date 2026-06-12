# D3 Increment Db Freeze-Resistance Evidence

Status: executed-hold

Evidence mode: Static + Ran

Date: 2026-06-12

## Scope

Increment Db implements the narrow target localized by Da: freeze-active
`frzng` must grow the surface heat path inside an hour as the fine-layer front
advances. It does not change thaw ownership, storage publication, capacity
routing, `frozwt`, or unit conversion.

Static authority:

- `SC-SNOWFREEZE-001` v63 adds the Db binding under `INV-SNOWFREEZE-006`.
- Legacy `frzng.for:235-240`, `frzng.for:287-305`, and
  `frzng.for:334-335` recompute `qoutdm`/`qhtout` inside the 3600-second
  freeze loop after front advance.
- The Da p1 trace showed the openWEPP pre-Db defect directly: year 1 day 1
  hour 2 advanced `0.000397484 -> 1.162927773 m` while retaining start-hour
  resistance `0.000227134 m2 C/W`.

## Implementation

- Added shared `frost_surface_heat_path` calculation for snow, residue,
  tilled frozen soil, and untilled frozen soil resistance.
- Split fine-front freezing into a single-step primitive so one in-hour loop
  cannot spend a stale start-hour flux across many fine layers.
- Added `freeze_fine_front_with_resistance_feedback`, which recomputes
  surface resistance and `Qsrf` after each fine-layer advance and decrements
  the remaining hour by the energy actually consumed at the current flux.
- Kept C1b/C2 capacity and overflow surfaces intact; overflow still routes
  through `watpdg`/`watbtm`, and the layered frozen store remains the only WAT
  `frozwt` source.

## Red Test

`fdhp01_db_freeze_front_recomputes_resistance_within_hour` was authored before
the production fix. On the pre-Db runtime path it failed as intended:

```text
one cold hour must not spend start-hour thin-front resistance across the profile; advanced 0.1996 m
```

After the production change, the focused test passes and verifies that a cold
thin-front hour still advances frost but remains bounded, with the implied
surface flux materially reduced after the frozen path grows.

## Cohort Evidence

Comparator subagent was not used per user quota direction. The parent model
ran local CLI/DuckDB comparisons against the `algebraic-radium` 43-prefix
single-OFE cohort.

Run root:

- `/tmp/fdhp01_increment_db_cohort_20260612T051524Z`

Generated package reports:

- `fdhp01_increment_db_execution_summary_20260612.json`
- `fdhp01_increment_db_run_status_20260612.tsv`
- `fdhp01_increment_db_annual_closure_residuals_20260612.csv`
- `fdhp01_increment_db_depth_metrics_20260612.csv`
- `fdhp01_increment_db_frozwt_frdp_ratio_20260612.csv`
- `fdhp01_increment_db_activation_summary_20260612.csv`

Key results:

- Cohort execution: `43/43` clean exits, `43/43` WAT outputs.
- D2 independent WAT ledger, years 2-6: max abs residual
  `1.9976620946327017e-07 mm` (`p11`, year 6), with p1/p20 spot checks still
  at `~1e-13 mm` and p43 year 2 at `-5.3290705182007514e-14 mm`.
- Profile-bound pinning: `0/43` prefixes pinned; minimum margin to the
  profile bound `1356.009607259958 mm`.
- Maximum depth envelope: all `43/43` prefixes fall inside the legacy
  `240..503.2 mm` envelope; mean max `409.16220799389805 mm`, median max
  `407.3294069097544 mm`, range `385.7920154626782..443.990392740042 mm`.
- `frozwt/frdp` scalar signature remains broken: per-prefix correlation range
  `0.7585445427147528..0.8677993973935473`, median
  `0.7716116121810137`.

## Validation Gates

Ran:

- `cargo fmt --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `cargo deny check` passed.
- `bash tools/release/check_authority_suite_antievasion.sh` passed.
- `cargo test --test auth11_required_suite_obligation_guards_contract --
  --nocapture` passed, `2` tests.
- `git diff --check` passed.
- `wctl doc-lint --path docs` passed, `1220 files validated, 0 errors,
  0 warnings`.

## Hold

Db resolves the stale in-hour resistance defect and moves frost depth into the
physical legacy maximum-depth envelope, but FDHP01 remains open:

- Depth correlation is still below the package acceptance target: median
  `-0.05296014769462692`, range
  `-0.07638011292463864..0.18461818718568115`.
- Frozen duration now under-persists severely: median open-minus-legacy
  `-452` days, mean `-455.3953488372093`, range `-505..-408`.
- Median days above `200 mm` is `92`, a large improvement from Da's `815`, but
  the duration/correlation gate is still red.

Disposition: landed at `executed-hold`. The next D3 work should target the
freeze/thaw seasonal persistence and timing behavior under the fine-layer
state, not publication, scalar capacity loosening, or comparator tuning.
