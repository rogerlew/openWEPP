# FDMC01 Frost Depth Characterization Ledger

Status: complete

Evidence mode: Ran.

## Scope

Substrate: `/wc1/runs/al/algebraic-radium/wepp/runs/` single-OFE cohort (`p1..p43`, `ksflag=1`).

Comparator role: legacy `wepp_260606_hill` is used to size the depth-model gap for Stage-2 promotion; it is not an acceptance oracle.

## Legacy Surface Feasibility

Legacy frost depth (`frdp`) is parseable from daily winter output (`unit 42`) when `daily winter` output is enabled in the legacy runfile (`outfil.for` + `winter.for` write path).

Run-time acquisition used in this package:
- For each prefix, run `wepp_260606_hill` with the `daily winter` toggle enabled.
- Output emitted to `H*.winter.dat`, containing hourly `frost depth` (`frdp*1000`) and `thaw depth` fields.

Frozen-water duration uses `H*.wat.dat` `frozwt`.

## Method

1. Ran legacy `wepp_260606_hill` for all 43 prefixes with winter output enabled (`H*.winter.dat` + `H*.wat.dat`).
2. Used openWEPP per-prefix WAT outputs from `/tmp/fq4_population/outputs/p*/H.wat.parquet`.
3. Reconstructed openWEPP frost-depth proxy series from climate (`p*.cli`) using current runtime logic:
   - `freeze_active = (tmin <= 0 C)`
   - `freeze_index = clamp(-mean_temp/6, 0, 1)`
   - `frdp = max(prior_frdp, 0.20*freeze_index)` when freeze-active
   - thaw branch otherwise with `thaw_index = clamp(mean_temp/6, 0, 1)`
4. Aligned depth series by simulation day index (legacy winter output is winter-window sparse; WAT chronology provides full-day index map).

Generated evidence files:
- `artifacts/frost_depth_characterization_metrics.csv`
- `artifacts/frost_depth_timeseries_pairs.csv`
- `artifacts/frost_depth_characterization_summary.json`

## Cohort Results (43 prefixes)

- Legacy max depth: min `240.0 mm`, max `503.2 mm`, mean `414.22 mm`.
- OpenWEPP proxy max depth: `200.0 mm` for all 43 prefixes.
- Max-depth delta (open - legacy): mean `-214.22 mm`, range `[-303.2, -40.0] mm`.
- Legacy prefixes exceeding open cap (`200 mm`): `43/43`.
- Open cap bind days: `847` days per prefix (`36,421` total).
- Depth shape agreement (winter-observed days):
  - mean MAE `123.81 mm`
  - mean RMSE `146.44 mm`
  - median correlation `0.133`
- Frozen-water duration (`frozwt > 0`, WAT daily):
  - legacy mean `759.37` days
  - openWEPP mean `1017.0` days
  - mean delta (open - legacy) `+257.63` days (range `+235..+289`)
- Onset/thaw edge timing (first/last nonzero depth day) aligns at series boundaries for all prefixes (`delta=0`), but in-window depth magnitude/duration diverges materially.

## Representative Prefix Extremes

Largest under-depth vs legacy:
- `p11`: legacy `503.2 mm`, open `200.0 mm`, delta `-303.2 mm`
- `p9`: legacy `460.0 mm`, open `200.0 mm`, delta `-260.0 mm`
- `p14`: legacy `460.0 mm`, open `200.0 mm`, delta `-260.0 mm`

Smallest under-depth in cohort:
- `p4`: legacy `240.0 mm`, open `200.0 mm`, delta `-40.0 mm`

## Materiality Note (bounded)

The frozen-soil conductivity bite is near-total when frost is active (already established in FQ-4). The observed depth/duration gap maps directly to bite duration and effective infiltration limitation time.

In this characterization set, openWEPP keeps frozen water active substantially longer (`+~258` days mean) while capping depth (`200 mm`) far below legacy heat-flow depth. This indicates a materially different frost forcing envelope for runoff generation windows. Full runoff magnitude quantification remains post-MOFE per roadmap staging.
