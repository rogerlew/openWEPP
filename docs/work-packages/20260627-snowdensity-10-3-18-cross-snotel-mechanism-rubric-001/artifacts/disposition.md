# Disposition

Ran: SNOWDENSITY-10.3.18 closes as
`DIAGNOSTIC-COMPLETE-NO-PROMOTION-DECISION`.

The diagnostic applied the `INV-SNOWFREEZE-050` rubric across the five SNOTEL
SWE/depth/density climates and five bound `cancov_forest` SWE/depth/density
strata. It scored eight model profiles: five current direct-runtime profiles,
two archival rejected candidates as explicit unavailable profiles, and PySnobal
as an H SNOTEL flag profile.

Main read:

- The activated bundle remains mixed: `17` robust failures, robust score `172`.
- `harder_pomeroy_partition` is the highest-ranked supported current next lever
  in this matrix (`15` robust failures, score `179`, `9` better and `2` worse
  robust cells vs activated). This is an investigation rank only; prior
  10.3.5c paired non-SNOTEL snow-control regression remains binding against
  promotion.
- The 10.3.17 shallow-pack guard remains non-promoted (`0` aggregate robust-score
  delta and `0` robust-fail delta vs activated).
- The 10.3.16 sublimation candidate is worse in this cross-corpus rubric
  (`20` robust failures, score `153`).
- Humid-New-England cancov residuals are not representative of the mountain
  SNOTEL activated-bundle fail signature set (`fail_cell_jaccard = 0.2`).
- PySnobal remains a weak flag profile in the available SNOTEL H lane (`28`
  robust failures, score `11`) and is not a target.

No production/default/cap/schema/fixture/frost change was made. No
promotion/activation decision was made.
