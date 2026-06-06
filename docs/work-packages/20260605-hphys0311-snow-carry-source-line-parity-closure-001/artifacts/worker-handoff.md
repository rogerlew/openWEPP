# Worker Handoff

Status: complete

Evidence mode: ran

Static:

- HPHYS0311 executed the HPHYS0310 continuation and remains `HOLD`.
- Production physics edits remain unauthorized.
- Post-review dual review findings were dispositioned.
- Broad validation passed.

Ran:

- Added `SC-SNOWFREEZE-001#INV-SNOWFREEZE-036`.
- Added `SC-WATBAL-001#INV-WATBAL-084`.
- Added and ran `hphys0311_snow_carry_source_line_parity_contract`.
- Classified `7` HPHYS0310 groups representing `58` HPHYS0309 rows:
  - `6` `prior-year-terminal-state-hold`;
  - `1` `fixed-observe-precision-hold`.
- No group authorizes downstream compensation.
- Dual verification completed after review repairs.

## Required Continuation

The next work package should localize the inherited prior-year terminal
snowpack state deltas. Scope:

- For the six `prior-year-terminal-state-hold` groups, scan backward within the
  prior calendar year from terminal day hour 24 to the first material paired
  snowpack divergence.
- Compare fixed-comparator `snowd.for` depth/density transitions against
  openWEPP hourly depth/density and daily runtime SWE/depth/density surfaces at
  the newly found divergence.
- If the H1 2013 `fixed-observe-precision-hold` row remains material, add or
  require full-precision baseline carry-state evidence including `wdayct`
  before any settling equation edit.
- Keep branch-predicate, same-hour melt-term, WB13/WB17/WB18/WB19/WB12
  compensation prohibited until source-line proof exists.
