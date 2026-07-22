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

Ran: final closure measurement at exact clean head `b1096a78` passed 116/116
instrumented library tests in 452.40 seconds. The target measures 2,206/2,291
lines (96.29%), 3,653/3,896 regions (93.76%), and 218/243 functions (89.71%).
No target CRAP entry is below the 75% per-function coverage floor.

Evidence root: `/tmp/cqr-pre-heavy-final-b1096a78-sPEqMT`. Source SHA-256:
`b8ed9863410ab9695b0820f4959ec6cd03509c3b64ea8ed7ab991d8c88ca0be3`;
LCOV SHA-256:
`9ed27757cba8749a3b26f7e253ef644c859108c0baff9a2ccc7a6cb4c4bce902`.
