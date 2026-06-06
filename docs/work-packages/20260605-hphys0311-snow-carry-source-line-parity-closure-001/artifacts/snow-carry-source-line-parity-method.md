# Snow Carry Source-Line Parity Method

Status: complete

Evidence mode: ran

Static:

- Uses HPHYS0310 carry-divergence groups as the input population.
- Uses HPHYS0305 fixed-comparator observe-on logs for `H305_S_OUT` post-hour
  `snodpt`/`densgt`.
- Uses HPHYS0305 openWEPP traces at `post_wb13` for runtime and hourly snow
  state.
- Requires source-line citations before generating the ledger.

Ran:

- For day-1 groups, compared prior-year terminal fixed-comparator state against
  prior-year terminal openWEPP runtime state, then compared day-1 hour-1 carried
  states.
- For the H1 2013 settling group, compared previous/current hour paired
  depth-density states and recorded that fixed-observe precision and missing
  `wdayct` prevent production-edit authority.
