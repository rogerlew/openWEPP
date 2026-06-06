# Snow Carry/Depletion Lineage Method

Status: complete

Evidence mode: ran

Static:

- Reads HPHYS0308 branch-extra state-ordering ledger.
- Uses HPHYS0305 fixed-comparator observe-on logs for baseline
  `H305_S_OUT` post-hour `snodpt`/`densgt` and `H305_M_POST` routed melt/rain
  surfaces.
- Uses HPHYS0305 openWEPP traces at `post_wb13` for daily runtime and hourly
  snow depth/SWE surfaces.

Ran:

- Filters HPHYS0308 rows to `snow-state-carry-depletion-hold`.
- Compares fixed-comparator prior-day hour-24 depth to openWEPP
  `snow_runtime_depth_before_m` on the key day.
- Finds the first same-day zero after-hour snow depth for baseline and
  openWEPP and records openWEPP depletion lead hours.
- Keeps every row in `HOLD` unless source-line-owned production proof exists.
