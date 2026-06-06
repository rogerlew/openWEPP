# Worker Handoff

Status: complete

Evidence mode: ran

Static:

- HPHYS0312 executed the HPHYS0311 continuation and remains `HOLD`.
- Production physics edits remain unauthorized.

Ran:

- Broad validation passed.
- Added `SC-SNOWFREEZE-001#INV-SNOWFREEZE-037`.
- Added `SC-WATBAL-001#INV-WATBAL-085`.
- Added and ran `hphys0312_prior_year_terminal_snowpack_lineage_contract`.
- Classified `6` HPHYS0311 inherited terminal groups representing `57`
  HPHYS0309 rows:
  - `3` `settling-depth-update-hold`;
  - `3` `year-start-inherited-state-hold`.
- No group authorizes downstream compensation.

## Required Continuation

The next work package should split along the two HPHYS0312 routes:

- For the three `settling-depth-update-hold` rows, add full-precision baseline
  `wdayct`, pre/post `densgy`/`densgt`, and `setf` evidence at 2013 day 11
  hour 11, then reconstruct the `snowd.for:122-139` settling/depth equations
  against openWEPP.
- For the three `year-start-inherited-state-hold` rows, recurse one calendar
  year earlier and scan the 2014 terminal carry-state chain that feeds 2015 day
  1 hour 1.
- Keep branch-predicate, melt-term, WB13/WB17/WB18/WB19/WB12 compensation
  prohibited until source-line proof exists.
