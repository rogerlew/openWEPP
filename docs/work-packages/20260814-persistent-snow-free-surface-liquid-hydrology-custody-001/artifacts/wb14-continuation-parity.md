# WB14 Continuation Parity

Evidence class: `Static + Ran`

The shadow continuation calls the same
`compute_green_ampt_interval_infiltration` transition used by the production
daily WB14 wrapper. It does not replay the day and does not carry a second
Green-Ampt transcription.

The focused vector executes 48 consecutive 1800-second intervals with retained
cumulative supply and infiltration, then compares the result with one unchanged
daily-wrapper call over the same 48-piece hyetograph and zero legacy depression
capacity.

Observed result:

- cumulative infiltration: bit-identical;
- legacy depression retention: exact zero;
- total excess: one-ULP grouping difference between 48 chronological additions
  and 24 hourly-bin additions, within the already admitted scale-aware depth
  closure rule;
- no solver or process tolerance was added or changed.

The earlier strict-bit excess assertion and its failure remain recorded in the
package gate history. A later widening of this comparison requires new
authority; this package does not create a generic portability tolerance.
