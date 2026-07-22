# Coverage Checkpoint After First Extraction

Ran: the delegated target-only LLVM measurement reports `pre_heavy.rs` at
1,124/1,686 lines (66.67%) on source SHA
`e946b005cdbb92ee71fb46e3bbe0b05e449f500db36b575bc08ece88135dc037`.
This is a progress checkpoint, not coverage closure: all eligible production
functions remain subject to the ADR-0021 glue-tier coverage threshold and
per-function floor before package completion.

Ran: final CRAP-zero measurement at `f1774586` reports 1,168/1,991 lines
(58.66%) and 118/230 functions (51.30%). CRAP is closed, but coverage
non-regression remains open because helper extraction increased the denominator
without increasing executed lines. Additional direct characterization is
required before package review.
