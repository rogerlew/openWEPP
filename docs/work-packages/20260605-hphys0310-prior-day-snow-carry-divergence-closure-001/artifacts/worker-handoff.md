# Worker Handoff

Status: complete

Evidence mode: ran

Static:

- HPHYS0310 executed the HPHYS0309 continuation and remains `HOLD`.
- Production physics edits remain unauthorized.

Ran:

- Added `SC-SNOWFREEZE-001#INV-SNOWFREEZE-035`.
- Added `SC-WATBAL-001#INV-WATBAL-083`.
- Added and ran `hphys0310_prior_day_snow_carry_divergence_contract`.
- Classified `7` affected hillslope/window/year groups representing `58`
  HPHYS0309 rows:
  - `6` `initial-carry-state-projection-hold`;
  - `1` `density-settling-carry-state-hold`.
- No group authorizes downstream compensation.

## Required Continuation

The next work package should focus on source-line-level accumulation, density,
and depth-update parity for the first divergent carry-state hours. Scope:

- Compare fixed-comparator `snowd.for` accumulation/settling/density update
  lines against openWEPP hourly snow state update code for the seven HPHYS0310
  first-divergence groups.
- For the six day-1 onset groups, determine whether the divergence is caused
  by initial carried snowpack state projection, density-to-depth conversion,
  settle-day count projection, or snowfall/drift initialization.
- For the H1 2013 day-11 h11 group, compare the pre/post density and depth
  update path over the preceding hours to identify the first source-owned
  settling/depth equation divergence.
- Keep branch-predicate, same-hour melt-term, WB13/WB17/WB18/WB19/WB12
  compensation prohibited until source-line proof exists.
