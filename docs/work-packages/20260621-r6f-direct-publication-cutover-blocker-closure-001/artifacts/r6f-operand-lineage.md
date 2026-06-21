# R6F Operand Lineage

Status: executed-held.

Compatibility WB13 rows, compatibility runtime surfaces, writeback payloads,
stale logical state, and skeleton publication capture remain forbidden as
direct authority.

## Publication Operand Map

| Output family | Field/key | Direct operand | Direct producer | Direct consumer/writer | Authority | Units/basis | Compatibility alias rejected by |
|---|---|---|---|---|---|---|---|
| HBP | `peakro` | `publication.runoff.peak_runoff_m3_s` | R6F helper from typed direct `q_runoff_m` and WB16 near-zero constants for near-zero runoff only | `build_hbp_output_from_direct_publication` | R6 ledger; WB16 near-zero constants | `m^3/s` | Current near-zero fixture HBP byte identity test; nonzero peak-runoff fixture remains required. |
| HBP | `watdur` | `publication.runoff.runoff_duration_s` | R6F helper from typed direct `q_runoff_m` and WB16 near-zero constants for near-zero runoff only | `build_hbp_output_from_direct_publication` | R6 ledger; WB16 near-zero constants | seconds | Current near-zero fixture HBP byte identity test; distinct event-duration fixture remains required. |
| WAT | `P`, `RM`, `Q`, `QOFE` | `publication.climate.precipitation_mm`, `publication.liquid_input.rm_mm`, `publication.runoff.q_mm`, `publication.runoff.qofe_mm` | Existing direct climate/liquid/runoff path after R6F unit fix | direct WAT row builder | R6 ledger | mm | R6F HBP/WAT reduction test asserts parity for these fields. |
| WAT | `wepp_id`, `year` | direct publication identity/schema projection | Missing production schema identity binding | direct WAT row builder | R6 metadata ledger; `SC-SYSTEM-001` replay/publication lineage | identity / simulation year | R6F WAT reduction test asserts mismatch; R6G must bind. |
| WAT | `Es` | `publication.evaporation.es_mm` | Direct runtime can accept `DirectEvapotranspirationComputeInputs`; production runner does not yet bind parsed ET inputs | direct WAT row builder | R6 ledger; `SC-EVAP-001` | mm | R6F direct-runtime typed-input test rejects zero/default when supplied. |
| WAT | `Total-Soil`, `SoilWaterTotal` | `publication.storage.total_soil_mm`, `publication.storage.soil_water_total_mm` | Direct runtime can carry layer state and project storage; production runner does not yet seed typed layer state from parsed inputs | direct WAT row builder | R6 ledger; `SC-SYSTEM-001` WB13 storage lineage | mm | R6F direct-runtime typed-input/carry test. |
| WAT | `ProfileDepth`, `ProfilePorosityCap`, `ProfileFCStore`, `ProfileWPStore` | `publication.profile.*` | R6F added profile-depth/porosity projection; production runner does not yet bind parsed profile inputs | direct WAT row builder | R6 ledger; `SC-SYSTEM-001` profile publication lineage | mm | R6F direct-runtime typed-input test. |
| PASS | `runvol`, `sbrunv`, `peakro`, sediment fields | direct publication runoff/subsurface/erosion operands | Not reached after WAT hold | direct PASS row builder | R6 ledger | mixed | Pending R6G/R6 continuation. |
| Loss | JSON fields | direct publication metadata/static inputs | Not accepted as final cutover because WAT blocks | direct loss JSON builder | R6 ledger | JSON | Pending R6 continuation. |
| Manifest | checksums/provenance/counters | direct publication manifest projection | Not reached after WAT hold | manifest writer | R6 ledger | JSON/checksum | Pending R6 continuation. |

## Missing Producers

| Producer | Needed for | Current R6F state | Follow-on |
|---|---|---|---|
| Parsed-input direct WAT identity binding | `wepp_id`, simulation `year` | direct row uses run identity/calendar year; compatibility WAT schema uses `wepp_id=1` and simulation year. | R6G. |
| Parsed-input direct ET producer | `Es` and future `Ep`/`Er` parity | direct runtime accepts `DirectEvapotranspirationComputeInputs`; runner only supplies climate/calendar. | R6G under `SC-EVAP-001`. |
| Parsed-input direct storage/layer producer | `Total-Soil`, `SoilWaterTotal` | lane layer carry exists; production initial layer state is absent. | R6G under `SC-SYSTEM-001`. |
| Parsed-input direct profile producer | profile optional WAT fields | projection fields exist; production profile inputs absent. | R6G under `SC-SYSTEM-001`. |

## Alias Rejection

R6F explicitly rejected using compatibility WB13 rows or runtime surfaces as a
shortcut for the WAT fields. Section 5.2.1 of the array-native runtime
specification says those structures are not valid direct sources for R6 cutover.
