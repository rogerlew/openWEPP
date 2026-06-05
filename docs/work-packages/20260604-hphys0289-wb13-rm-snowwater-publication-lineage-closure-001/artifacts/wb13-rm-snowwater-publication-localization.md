# WB13 RM/Snow-Water Publication Localization

Status: complete
Evidence mode: Static/Ran

## Localized Defect

Static:

- Prior WB13 `RM` publication used raw precipitation plus SWE-delta proxy. That does not match pinned baseline `rain + wmelt + irrigation`.
- The hydrology kernel already computed a daily snow/runoff term for WB12/WB14 closure but did not publish it as a named routed-melt surface for WB13.
- WB13 `Snow-Water` already consumed `snow.runtime_swe`, matching baseline `snodpy * densg`, so HPHYS0289 did not alter the Snow-Water source.

## Correction

Static:

- Published `snow.routed_melt_m` from hydrology runoff reconciliation.
- WB13 `RM` now consumes `snow.routed_melt_m`, post-winter rain inference, and `Irr`.
- Non-finite/missing/negative routed melt fails closed through runtime-surface guard paths.

## Remaining Localization

Ran:

- Full suite shows `RM` fail count improved by 765 but `Snow-Water` did not move and `Q` did not move.
- Target traces show residual rows where explicit post-winter rain identity is still not available to WB13.
- Continuation should publish/consume the post-winter `rain(iplane)` surface from winter/contin lineage instead of inferring it from `prcp` and snow-state activity.
