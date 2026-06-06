# Worker Handoff

Status: complete

Evidence mode: ran

Static:

- HPHYS0309 executed the HPHYS0308 continuation and remains `HOLD`.
- Production physics edits remain unauthorized.

Ran:

- Added `SC-SNOWFREEZE-001#INV-SNOWFREEZE-034`.
- Added `SC-WATBAL-001#INV-WATBAL-082`.
- Added and ran `hphys0309_snow_carry_depletion_lineage_contract`.
- Classified `58` HPHYS0308 snow-state carry/depletion rows:
  - `45` `pre-day-carry-deficit-hold`;
  - `13` `prior-day-openwepp-meltout-hold`.
- No row authorizes downstream compensation.

## Required Continuation

The next work package should focus on the prior-day/day-start snowpack carry
state that precedes the HPHYS0308 branch-extra keys. Scope:

- For each affected H1/H7/H39 window, reconstruct the first day/hour where
  openWEPP carry state materially diverges from fixed-comparator
  `snodpt`/`densgt`.
- Compare daily start/end depth, SWE, density, raw melt, redistributed melt,
  retained/released rain, snowfall, and corrected negative-melt state-loss
  terms across the preceding snow episode.
- Determine whether the carry deficit is caused by accumulation/settling,
  corrected negative-melt state loss, retained-liquid density handling,
  same-day raw melt magnitude, or runtime state publication/carry ordering.
- Keep the single H7 first-2013 openWEPP-extra key as baseline branch
  instrumentation scope unless prior carry-state evidence explains it.
- Keep WB13/WB17/WB18/WB19/WB12 compensation prohibited.
