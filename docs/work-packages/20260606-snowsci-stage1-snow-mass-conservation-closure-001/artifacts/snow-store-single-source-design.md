# Snow Store Single-Source Design

Status: closed-with-follow-up-postreview

Evidence mode: Ran

## Milestone 1 Localization

Ran:

- Reproduced pre-fix `p7` failure with
  `target/release/openwepp-cli-hill --run-dir /wc1/runs/in/indispensable-presenter/wepp/runs --run-file /tmp/wbval05_j95_perc_20260606T000000Z/generated_runfiles/p7.toml --output-dir /tmp/snowsci_stage1_repro/p7 --policy compat`.
- Observed fail-closed status:
  `HKERNEL-WB14-RUNOFF-E-003`, `sim_day_index=95`,
  `calendar_year=1990`, `julian_day=95`.
- Temporary diagnostic build localized the mechanism at
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
  in `redistribute_daily_signed_snowmelt` / `runtime_swe_after_raw`.

Static:

- Day 95 enters WB14 with valid positive snow state, not negative carry:
  `runtime_swe=0.007376104224 m`, `depth=0.021074583496 m`,
  `density=350 kg/m3`, `settle=9`.
- The hourly snow loop has already removed the positive raw melt from the
  depth/density store. The separate SWE debit then computed
  `state_loss=0.013547261834 m` from `positive - negative`, while routed melt
  was only `0.001204946614 m`. That overdraw is the independent SWE ledger, not
  a snow physics-magnitude equation.

## Design

Stage-1 authority is `SC-SNOWFREEZE-001#INV-SNOWFREEZE-019` and
`SC-WATBAL-001#INV-WATBAL-059` as amended in this package.

The authoritative snow store is the post-hourly depth/density state. Runtime
`snow.runtime_swe` remains a derived/carry/publication value. Mixed signed melt
diagnostics keep negative raw melt in `snow.hourly.melt_raw_m`, but negative raw
melt cannot create a second SWE debit after the depth/density store has already
recorded pack loss.

Production correction:

- `redistribute_daily_signed_snowmelt` now routes the positive water-equivalent
  pack loss already applied to the depth/density store.
- Negative raw melt is preserved through diagnostics but is zeroed in routed
  hourly melt publication.
- `routed_melt_total_m` and `snowpack_state_loss_m` are equal for the snowpack
  component, so `S`, WB12 liquid forcing, WB13 `RM`, and WB13 `Snow-Water` share
  one storage-loss scalar.

Protected-boundary result:

- No CRM Eq. 3.7.x settling/density equations were changed.
- No melt-model magnitude equation was changed.
- No rain/snow partition or daily-temperature-in-hourly threshold was changed.
- The visible guard was not replaced with a silent clamp; the overdraw path was
  removed by making the routed/storage accounting single-sourced.
- SNOWSCI-S1 does supersede the prior openWEPP/wepp-forest
  negative-melt carry-state interpretation in `INV-SNOWFREEZE-019` for Stage-1
  accounting. Negative raw melt is diagnostic-only here; Stage 2 still owns
  physical ratification of any independent negative-melt pack/routing role.

## Common-Cause Check

Ran:

- After the correction, `p7`, `p11`, `p18`, and `p20` all reached WAT
  publication.
- A fresh isolated WBVAL04 runfile sweep produced WAT outputs for all `22`
  runfiles under `/tmp/snowsci_stage1_wbval06_after_20260607T021725Z/outputs`.
- WBVAL06 before/after residual recomputation used WBVAL04
  `closure_summary.csv`/`closure_ledger.csv` as the before source and the fresh
  after parquet outputs.

Result:

- The J-95 negative-SWE fail-closed mechanism is snow-sourced and in-envelope.
- On the `18` WBVAL04 status-valid emitters, max annual residual fell from
  `94.433070 mm` to `26.790809 mm`; mean prefix max residual fell from
  `58.644141 mm` to `22.461195 mm`.
- The broader WBVAL06 annual residual is not closed in this package: all
  after-run emitters remain above the `1.0 mm/year` tolerance, so WBVAL06 still
  owns residual attribution and any remaining term/unit audit.
