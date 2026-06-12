# D3 Increment Da Energy Characterization

Status: executed-hold

Evidence mode: Static + Ran

Date: 2026-06-12

Comparator subagent: not used. The parent run executed local CLI/DuckDB
comparisons directly because the user reported GPT-5.3-Codex-Spark weekly quota
exhaustion.

## Scope

Increment Da was diagnostic. It did not land production physics. A temporary
env-gated stderr trace was added to the hourly frost loop, used for one p1 run,
then removed before rebuilding the release CLI and running the 43-prefix cohort.

Da also repaired the cohort closure audit surface. The C2 table reused
`flux_balance_mm == storage_delta_mm`, making `residual_mm` zero by
construction. The Da annual table recomputes the visible WAT flux balance
independently:

`RM + Irr - Interception - Q - Ep - Es - Er - Dp - latqcc - Tile`

and compares that against annual `delta(Total-Soil + frozwt)`, carrying each
year's initial storage from the previous year-end final storage for years 2-7.
Year 1 is recorded but not used as a gate because the independent annual ledger
does not have a pre-run initial storage state.

## Static Legacy Expectation

Legacy `frzng.for` recomputes the surface heat-flow resistance as the frozen
front advances within an hour:

- `frzng.for:235-240` loops until the 3600-second hour is exhausted and says
  the heat flux from the soil surface must be re-estimated when the frozen
  front moves down one or more fine layers.
- `frzng.for:287-305` advances `frzdp`, updates `qoutdm` by the newly frozen
  tilled/untilled path length divided by `kftill`/`kfutil`, and exits at soil
  thickness.
- `frzng.for:334-335` recomputes `qhtout = surtmp(hour) / (qoutdm + vardm)`
  before the next fine-layer step.
- `frostn.for:660-669` dispatches `frzng` for freeze and freeze plus
  bottom-melt arms, then calls `watdst(..., 2)`.

The openWEPP C2 loop computes the frozen-path resistance once at the start of
the hour, converts the resulting flux to one hour of energy, and lets
`freeze_fine_front` consume that energy across multiple fine layers. That skips
the legacy in-loop `qoutdm` growth.

## p1 Trace Result

Trace root:

`/tmp/fdhp01_increment_da_trace_20260612T043800Z`

Generated evidence:

- `fdhp01_increment_da_p1_hourly_energy_trace_20260612.csv`
- `fdhp01_increment_da_p1_energy_summary_20260612.json`

The trace emitted `20852` frost-active hourly rows. Of those, `4512` were
freeze-branch rows and `220` advanced the front.

Key hours:

| Case | Year/day/hour | Depth before -> after | Resistance before -> after-depth projection | qhtout before -> after-depth projection | qdry | Note |
|---|---:|---:|---:|---:|---:|---|
| First metre-scale advance | 1/1/2 | `0.000397484 -> 1.162927773 m` | `0.000227134 -> 0.572822749 m2 C/W` | `-35602.871 -> -14.117 W/m2` | `14.7 W/m2` | Uncapped; inferred water per depth `0.328972115` |
| Largest uncapped advance | 3/69/2 | `0.000573159 -> 1.642144897 m` | `0.000327519 -> 0.801021379 m2 C/W` | `-22776.653 -> -9.313 W/m2` | `14.7 W/m2` | Uncapped; inferred water per depth `0.149007236` |
| Largest advance | 5/83/2 | `0.000007686 -> 1.800000000 m` | `0.000004392 -> 0.876190476 m2 C/W` | `-1001876.220 -> -5.022 W/m2` | `14.7 W/m2` | Hits profile cap; inferred water value is not physical because energy remains after cap |

The first metre-scale advance is the decisive discriminator. The hour uses a
near-zero frozen-path resistance for the whole hour, generating `128117415.9
J/m2` of freeze energy and moving the front `1.162530288 m`. If the same
hour-end frozen path is included in resistance, the surface flux drops by a
factor of about `2522`. That is the legacy `qoutdm` feedback Da expected to
find.

## Candidate Discrimination

- Candidate (a), stale resistance growth: supported. Resistance does not grow
  with the within-hour frozen path, so `qhtout` stays artificially large while
  the front crosses many fine layers.
- Candidate (b), `qdry` too weak: not the primary onset cause. In the first
  metre-scale hour, `qdry` is `14.7 W/m2` against a stale-resistance
  `|qhtout|` of `35602.871 W/m2`.
- Candidate (c), latent term too cheap: not the root discriminator from p1.
  The first uncapped metre-scale advance has inferred water per depth
  `0.328972115 m3/m3`. Cap-hit rows have nonphysical inferred values because
  unused energy remains after the `1.8 m` profile cap.
- Candidate (d), W/m2-to-energy unit error: not supported. The trace energy is
  internally `W/m2 * 3600 s`; the pathological energy magnitude comes from the
  stale resistance, not an extra day/hour multiplier.

## Local Cohort Evidence

Production release binary SHA:

`0b3fed8561232d0de371a195e3f5a5609121ddd2253713b3fb0139add9ec8a4f`

Cohort root:

`/tmp/fdhp01_increment_da_cohort_20260612T044217Z`

| Gate | Result |
|---|---|
| Temporary trace removed before production cohort | Pass; no trace marker remains under `crates/` |
| Clean execution | Pass, `43/43` prefixes |
| WAT outputs | Pass, `43/43` |
| WAT row equality versus C2 | Pass, `43/43`; Da lands no production physics |
| Independent years 2-6 `Total-Soil + frozwt` closure | Pass at repaired WAT-surface numerical floor; max abs `1.3813070645629644e-07 mm` (`p11`, year 5) |
| p43 year-2 closure watch | Cleared as WAT-surface numerical floor, `-1.912025027195341e-08 mm` |
| p1/p20 closure spot checks, years 2-6 | p1 max abs `1.1812772982011666e-13 mm`; p20 max abs `6.650235917504688e-14 mm` |
| Profile-bound pinning | Still unpinned, `0/43`; minimum margin `5.54557792097421 mm` |
| `frozwt/frdp` scalar-signature rejection | Still pass, max correlation `0.9441102161636825`, median `0.8831449770567324` |
| D3 depth envelope | Still fail, mean max depth `1793.52198510966 mm` |
| D3 depth correlation | Still fail, median `-0.16722397856345997` |
| D3 frozen-duration acceptance | Still fail, median open-minus-legacy `111` days |
| Days above `200 mm` watch | Still fail/watch, median `815` days |

Generated cohort artifacts:

- `fdhp01_increment_da_execution_summary_20260612.json`
- `fdhp01_increment_da_run_status_20260612.tsv`
- `fdhp01_increment_da_annual_closure_residuals_20260612.csv`
- `fdhp01_increment_da_depth_metrics_20260612.csv`
- `fdhp01_increment_da_frozwt_frdp_ratio_20260612.csv`
- `fdhp01_increment_da_activation_summary_20260612.csv`
- `fdhp01_increment_da_c2_row_equality_20260612.json`

## Disposition

Da lands as `executed-hold`, not closure. It repairs the annual closure ledger
so future gates no longer use a tautological residual and clears the p43 watch
as WAT-publication numerical texture, not a storage leak.

The Db implementation target is now narrow: port the legacy `frzng` in-hour
front-advance loop so every fine-layer advance grows `qoutdm`/frozen-path
resistance before consuming more of the hour's freeze energy. Db should add a
red test that would fail the Da trace shape: a known p1-like profile must not
advance metres in one hour under the starting near-zero resistance, and
`qhtout` must collapse as the frozen path length grows.
