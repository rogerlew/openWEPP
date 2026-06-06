# Prior-Day Snow Carry Divergence Method

Status: complete

Evidence mode: ran

Static:

- Reads the executed HPHYS0309 carry/depletion ledger.
- Uses HPHYS0305 fixed-comparator observe-on logs for baseline `H305_S_OUT`
  post-hour `snodpt`/`densgt`, `H305_M_POST` post-winter routed melt/rain
  surfaces, and active `H305_T_*` term evidence where present.
- Uses HPHYS0305 openWEPP traces at `post_wb13` for daily runtime snow state
  and hourly before/after depth, density, melt, rain, and snowfall surfaces.

Ran:

- Groups all HPHYS0309 snow-state carry/depletion rows by
  hillslope/window/year.
- Scans paired fixed-comparator and openWEPP hourly after-depth from day 1
  through the first HPHYS0309 key day for that group.
- Records the first paired depth divergence above `0.0005 m`.
- Aggregates episode-level baseline and openWEPP snow flux/state lanes from
  first nonzero snow through the first key day.
- Keeps every group in `HOLD` unless source-line-owned production proof exists.
