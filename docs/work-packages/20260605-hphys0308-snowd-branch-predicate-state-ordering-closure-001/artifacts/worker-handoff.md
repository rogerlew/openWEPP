# Worker Handoff

Status: complete

Evidence mode: ran

Static:

- HPHYS0308 executed the HPHYS0307 continuation and remains `HOLD`.
- Production physics edits remain unauthorized.

Ran:

- Added `SC-WATBAL-001#INV-WATBAL-081`.
- Added and ran `hphys0308_snowd_branch_state_ordering_contract`.
- Classified `59` branch-extra keys:
  - `58` `snow-state-carry-depletion-hold`;
  - `1` `baseline-branch-instrumentation-hold`.
- No row authorizes downstream compensation.

## Required Continuation

The next work package should focus on snow-state carry/depletion lineage before
any melt-term magnitude or branch-predicate edit. Scope:

- Instrument or reconstruct baseline/openWEPP day/hour snowpack carry state
  around the `58` baseline-extra keys where openWEPP snow depth is already zero.
- Compare fixed-baseline pre-hour `snodpy`/`densg`/`wdayct` carry state and
  post-hour `snodpt`/`densgt` against openWEPP runtime
  `snow_runtime_depth_before_m`, `snow_hourly_depth_before_m`, and
  `snow_hourly_depth_after_m`.
- Determine whether the carry/depletion divergence is caused by earlier
  meltout timing, retained-liquid density handling, negative-melt correction,
  or state publication/carry ordering.
- Keep the single H7 first-2013 openWEPP-extra key as baseline branch
  instrumentation scope unless the carry-state package explains it.
- Keep WB13/WB17/WB18/WB19/WB12 compensation prohibited.
