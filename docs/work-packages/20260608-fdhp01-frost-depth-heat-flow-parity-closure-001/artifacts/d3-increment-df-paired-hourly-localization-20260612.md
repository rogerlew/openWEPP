# D3 Increment Df — Paired Hourly Front/Flux Localization

Status: executed-hold

Evidence mode: Ran

Date: 2026-06-12

## Objective

Execute Increment Df from `d3-staged-increment-plan.md`: run a temporary
openWEPP hourly frost trace under the De forced-legacy-snow setup, pair it
hour-by-hour with legacy `H*.winter.dat`, and attribute the first material
remaining front/flux divergence before authoring another physics fix.

The comparator subagent was not used per user quota direction. All execution
and comparisons were run locally with the release CLI plus Pandas/PyArrow.

## Method

- Verified legacy winter output semantics against source before analysis:
  `winter.dat` `ground` is ground-drift snow (`gdrft`), not a ground or surface
  temperature column. In this pinned build ground drift is hard-set to zero in
  `winter.for`; the temperature-surface hypothesis therefore cannot be read
  directly from that column.
- Added a temporary env-gated diagnostic hook in
  `compute_active_frost_coupling`:
  - forced only frost-consumed snow depth/density from the Dd legacy snow CSV;
  - wrote hourly frost front, heat-flux, resistance, branch, residue, snow, and
    storage terms for `p1` and `p2`.
- Ran p1/p2 through the De forced-snow TOML runfile lane, rewriting absolute
  output paths into the Df temp root.
- Joined the last openWEPP trace row for each `(prefix, simulation_year,
  julian, hour)` to the corresponding legacy winter row.
- Removed the temporary hook and rebuilt release `openwepp-cli-hill` from clean
  source before updating package evidence.

## Evidence

- Temporary trace root:
  `/tmp/fdhp01_increment_df_trace2_20260612T175406Z`
- Legacy winter root:
  `/tmp/fdhp01_increment_dd_legacy_winter_20260612Tdd/output`
- Forced snow CSV:
  `/tmp/fdhp01_increment_dd_legacy_winter_20260612Tdd/legacy_snow_forcing_daily_hour24.csv`
- Compact artifacts:
  - `fdhp01_increment_df_localization_summary_20260612.json`
  - `fdhp01_increment_df_term_attribution_20260612.csv`
  - `fdhp01_increment_df_paired_hourly_excerpt_20260612.csv`

The raw p1/p2 hourly traces were not committed; they remain under `/tmp`.

## Result

Df localizes the controlled residual to the frost surface-resistance path, not
to snow, `Qdry`, or publication.

- Execution: p1/p2 clean, WAT outputs present.
- The first material divergence occurs before snow is involved:
  - p1 year 1 day 1 hour 2: legacy frost depth `5.0 mm`, openWEPP
    `42.057866709 mm` (`+37.057866709 mm`), open snow `0`, legacy snow `0`.
  - p2 year 1 day 1 hour 2: legacy `5.0 mm`, openWEPP `41.417581693 mm`
    (`+36.417581693 mm`), open snow `0`, legacy snow `0`.
- The openWEPP frost resistance seam is missing the legacy residue path:
  - open trace `residue_depth_m = 0.0` at the first divergence;
  - legacy `H*.winter.dat` reports `23.0 mm` residue depth on the same rows.
- The openWEPP shallow-front resistance also omits the legacy
  `frostn.for` minimum top-frozen conduction distance (`dpfsfl`, midpoint of
  the first fine layer). On p1 hour 2, open resistance is
  `0.000514691883 m2 C/W`; the conservative legacy estimate
  `(23 mm / Kres) + (5 mm / kftill)` is `0.462857142857 m2 C/W`, about
  `899x` larger. p2 is about `951x`.
- The freeze arm is surface-flux dominated at the first divergence:
  `signed_surface_flux = -15711.6 W/m2` and `lower_front_heat = +0.751 W/m2`
  for p1; p2 is `-16016.6 W/m2` and `+0.718 W/m2`.
- Rows with `open - legacy > 100 mm` are consistently on the same seam in the
  paired samples:
  - p1: `9078/9078` high-positive rows have zero open residue and positive
    legacy residue.
  - p2: `9947/9947` high-positive rows have zero open residue and positive
    legacy residue.

The trace also recorded two full 24-hour frost executions on many later days
(`1696` days for p1, `1706` for p2). This is a secondary watch item, but it is
not the first divergence: the first material split occurs on day 1 before any
duplicated daily frost trace.

## Disposition

Df is diagnostic-only and leaves production at the De boundary. The next
bounded implementation increment should be Dg: restore the legacy residue-depth
surface into frost heat resistance and apply the legacy shallow-front minimum
top-frozen conduction distance before re-running the De forced-snow
certification and native cohort.

Do not tune snow depth/density, kfactor, latent heat, WAT publication, or the
D2 storage surfaces for this residual.
