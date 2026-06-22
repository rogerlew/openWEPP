# Parity Evidence

Status: executed-held.

## Focused Fixture

- `cargo test -p openwepp-hillslope-orchestrator r4k_wb14_producer -- --nocapture`
  passed.
- `cargo test -p openwepp-hillslope-orchestrator
  r4l_sums_direct_hourly_saturation_carry_when_r4o_has_run -- --nocapture`
  passed.
- `cargo test -p openwepp-hillslope-orchestrator
  r4l_rejects_conflicting_saturation_handoff_when_hourly_carry_exists -- --nocapture`
  passed.

## H2637

- Same-binary default compatibility endpoint:
  `elapsed=637.63 rss_kb=227352`, exit 0.
- Same-binary direct production endpoint:
  `elapsed=192.90 rss_kb=643724`, exit 0.
- Direct manifest counter evidence:
  `compatibility_edge_invocations=0`, `day_frame_constructions=235961`,
  `day_frame_commits=235961`.
- `H2637.loss.json` byte parity: pass.
- `H2637.wat.parquet` row/schema shape: `235961` rows, `34` columns on both
  default and direct.
- `H2637.pass.parquet` row/schema shape: `12419` rows, `17` columns on both
  default and direct.

## Residuals

- HBP byte parity: fail.
- Manifest byte parity: fail; expected mode/provenance differences exist, but
  output checksum parity also fails because WAT/PASS/HBP differ.
- WAT value parity: fail. Maximum absolute deltas include:
  `Q=72.25917534435557`, `Dp=29.488664492756087`,
  `UpStrmQ=1316.4645543910933`, `SubRIn=89.89968491102034`,
  `latqcc=89.43602693828204`, `Total-Soil=511.2314284277249`,
  `Snow-Water=1089.901859742928`, `QOFE=1325.0132475165253`, and
  `Interception=0.891459703930619`.
- PASS value parity: fail. First row `runvol` default
  `107.13682236123434` vs direct `0.0`; max absolute
  `runvol=14402.354720112891`; max absolute
  `sbrunv=465.62385852543673`.
- Dominant blocker: direct MOFE hourly carry totals are zero in the direct
  manifest while default reports `0.2205447764353141` for both current and
  upstream carry totals. Downstream direct lanes therefore miss same-day
  `ui_SCrunf`/`ui_LfCrf` copy-forward into `UpStrmQ`/`SubRIn`.
