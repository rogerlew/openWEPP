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

Ran: corrected final closure at exact clean head `68e9b747` passed 117/117
instrumented library tests. Restricted to production lines 1-1,743, the target
measures 1,324/1,378 lines (96.08%) and 1,886/2,104 regions (89.64%). All
111 production functions meet the 75% region floor; the minimum is 80.00%.

Evidence root: `/tmp/cqr-pre-heavy-final-region-ORwL2Q`; LLVM JSON SHA-256:
`6597a19e8010d47e4cd834364990804c2afb74c2377734772bec0cb202fbc614`.
