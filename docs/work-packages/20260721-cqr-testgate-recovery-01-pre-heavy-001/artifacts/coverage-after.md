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

Ran: final non-regression measurement at exact head `3d6e8817` passed 109/109
instrumented tests. The target now measures 1,406/2,074 lines (67.79%),
2,401/3,378 regions (71.08%), and 152/235 functions (64.68%). Line and region
coverage exceed the source-identical pre-refactor checkpoint. This CQR package
claims characterization and non-regression, not standalone module-test-
enhancement closure at the ADR-0021 glue-tier floor.

Evidence root: `/tmp/cqr-pre-heavy-nonreg-rNRfRV`; summary SHA-256:
`d2d97592ebf4c3f5e82691f1010df561c21c10bfb0601afda45c85b3f3427163`.
