# PL08 Single-OFE Daily Water-Balance Comparator Delta Report

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Tier-A scope is single-OFE daily water-balance (ADR-0011 + numerics policy).
- Baseline authority is ADR-0012 pinned worktree `/workdir/wepp-forest_260430_baseline` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.

Ran:
- Executed baseline/candidate hillslope replay for fixture `delicate_game_pw0` run `p5.run`.
- Compared `H5.wat.dat` and `H5.plot.dat` via `compare_wepp_raw_outputs.py` (`abs_tol=0`, `rel_tol=0`).

## Comparator Targets

- Fixture run path: `/workdir/wepp-forest_260430_baseline/tests/fixtures/delicate_game_pw0/runs/p5.run`
- Single-OFE evidence: `/workdir/wepp-forest_260430_baseline/tests/fixtures/delicate_game_pw0/runs/p5.slp` line 2 = `1`
- Baseline binary: `/workdir/wepp-forest_260430_baseline/release/wepp_260430_hill`
- Candidate surrogate binary: `/workdir/wepp-forest_260430_baseline/release/wepp_260319_hill`

## Delta Summary

| surface | strict_pass | status | key metrics |
|---|---|---|---|
| `H5.wat.dat` | `false` | `structure_diff` | `line_count_mismatch=5`, `numeric_arity_mismatch_lines=1096`, `numeric_values_compared=21802`, `numeric_values_exceeding_tolerance=5568`, `max_abs_diff=360.0`, `max_rel_diff=1.0` |
| `H5.plot.dat` | `true` | `identical` | `identical_count=1`, `changed_count=0`, `max_abs_any=0`, `max_rel_any=0` |

## First Divergence Anchor (`H5.wat.dat`)

- First text divergence line: `18`
  - baseline: `SoilWaterTotal=Full-profile soil water (mm)`
  - candidate: blank line
- Header schema then diverges with baseline-only columns:
  - `SoilWaterTotal`, `ProfileDepth`, `ProfilePorosityCap`, `ProfileFCStore`, `ProfileWPStore`

## Keyed Alignment Check (Shared Daily Fields)

Ran keyed alignment check by `(OFE,J,Y)` across shared 20-field daily rows in `H5.wat.dat`.

Results:
- matched keys: `1095`
- columns `1..20` mismatch count: `0` for every column
- this includes plant/residue-relevant fields:
  - `Ep` mismatch rows: `0`
  - `Es` mismatch rows: `0`
  - `Er` mismatch rows: `0`

Interpretation:
- Comparator flags `structure_diff` because the baseline emits 5 additional trailing daily columns.
- For shared daily fields, keyed values are identical in this surrogate lane.

## Evidence Artifacts

- `artifacts/h5_wat_comparator.json`
- `artifacts/h5_plot_comparator.json`
- Run workspace: `/tmp/pl08_tiera_cmp_20260522`
