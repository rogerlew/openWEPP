# Operand Lineage

Status: implemented.
Evidence mode: Static.

Implemented lineage:

| Operand | Units/basis | Direct producer | Direct field | Output consumer | Wrong aliases to reject | Reconstruction gate |
|---|---|---|---|---|---|---|
| identity/calendar | run/lane/day ids and calendar | parsed run dimensions, slope OFE index, climate span | `DirectPublicationDayRow` identity fields and `calendar` | HBP, WAT, PASS, loss, manifest direct consumers | compatibility row ids, output row count, first-day shortcut | row count and calendar fixture. |
| `P` | mm | direct normalization from `DirectDayFrame` | `climate.precipitation_mm` | WAT/loss direct consumers | `RM`, irrigation, snowmelt | direct projection consumer test plus future climate alias fixture. |
| `RM` | mm | direct liquid input | `liquid_input.rm_mm` | WAT direct consumer | `P`, irrigation-only | direct projection consumer test. |
| `Q`, `QOFE`, `runvol` | mm and m3 | direct hydrology projection | `runoff.q_mm`, `runoff.qofe_mm`, `runoff.runvol_m3` | WAT/PASS direct consumers | `publication.runoff_m`, row area shortcuts, compatibility `Q` | `r6a_publication_capture_records_run_bound_rows_without_publication_alias`; direct projection consumer test. |
| `Ep`, `Es`, `Er` | mm | direct hydrology projection ET components | `evaporation.*` | WAT direct consumer | total ET, seed branch flags | direct projection consumer test. |
| `Dp`, `latqcc`, `Tile`, `sbrunv` | mm and m3 | direct hydrology projection/subsurface | `subsurface.*` | WAT/PASS direct consumers | `Qd`, lateral state, runoff volume | direct projection consumer test. |
| `UpStrmQ`, `SubRIn` | mm | direct normalization transfer inputs | `transfer.*` | WAT direct consumer | transfer arrays, downstream output | direct projection consumer test. |
| storage/snow/profile | mm | direct hydrology projection and optional profile inputs | `storage.*`, `profile.*` | WAT direct consumer | stale logical `watcon`, layer shortcuts | direct projection consumer test; broader reconstruction pending. |
| interception | mm | direct publication frame absent/storage field | `interception.*` | WAT direct consumer | storage as flux | direct projection consumer test for accepted fields. |
| HBP erosion/event | event scalars/classes | absent-authority direct erosion/event producers | `erosion.*` options | HBP/PASS direct consumers | daily runoff volume, zero compatibility surface | direct projection consumer test for supplied operands; full erosion producer pending. |
| loss metadata | text scalars | frame metadata and first/last direct rows | `metadata`, first/last rows | loss direct consumer | compatibility optional-output payload | direct projection consumer test. |
| manifest metadata | text scalars | frame metadata and row counts | `metadata`, identity counts | manifest direct consumer | compatibility row counts as direct counters | direct projection consumer test. |

No operand may list `SimulationOwnedWb13Row`, `HillslopeWritebackSurface`,
`BoundarySymbol`, `BoundaryValue`, or `KernelWritebackPayload` as a direct
source. Those structures may appear only in compatibility or shadow-comparison
paths.
