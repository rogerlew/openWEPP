# Operand Lineage

Status: executed-hold.
Evidence mode: Static + Ran.

## Lineage Table

| Family | Field | Units | Basis | Producer | Direct field | Legacy alias | Wrong aliases to reject | Metadata obligation | Reconstruction |
|---|---|---|---|---|---|---|---|---|---|
| HBP | `peakro` | m3/s | event peak | missing direct producer in current cutover | `DirectPublicationErosionOperands::peak_runoff_m3_s` or runoff fallback | runtime surface `peakro` | zero default, runoff volume, WB13 `q`/`qofe`, stale runtime surface | HBP schema unchanged | BLOCKED: no direct producer accepted |
| HBP | `watdur` | s | event duration | missing direct producer in current cutover | `DirectPublicationErosionOperands::runoff_duration_s` or runoff fallback | runtime surface `watdur` | zero default, day length, event count, stale runtime surface | HBP schema unchanged | BLOCKED: no direct producer accepted |
| HBP | sediment fields | kg and kg/m3 | event sediment | missing direct producer in current cutover | `DirectPublicationErosionOperands` | runtime sediment symbols | zero default, PASS zeros, adjacent diagnostics | HBP schema unchanged | BLOCKED: erosion authority is absent |
| WAT | precipitation/liquid/runoff/ET/subsurface/storage/profile/interception | mm, m2 | per OFE-day row | current cutover skeleton plus geometry only | `DirectPublicationDayRow` climate/liquid/runoff/evaporation/subsurface/storage/profile/interception | `Wb13DailyWaterBalanceRow` | zero default, runtime surface, stale WB13 row, diagnostic ledger | WAT Arrow schema/metadata unchanged | BLOCKED: fields map exists but source frame is skeleton |
| PASS | `runvol_m3`, `sbrunv_m3` | m3 | depth times OFE area | current cutover skeleton plus geometry only | `DirectPublicationRunoffOperands::runvol_m3`, `DirectPublicationSubsurfaceOperands::sbrunv_m3` | WB13 `q`/`qofe`/`latqcc` times area | wrong area denominator, local `q` vs outlet `qofe`, zero default | PASS Arrow schema/metadata unchanged | BLOCKED: volumes are zero/default because source depths are zero |
| PASS | peak/sediment fields | m3/s, kg, kg/m3 | outlet/event | missing direct producer in current cutover | `DirectPublicationErosionOperands` | compatibility PASS currently zeros; HBP runtime surface for HBP only | zero default, HBP aliases, stale runtime surface | PASS Arrow schema/metadata unchanged | BLOCKED: erosion authority is absent |
| loss | run/span/static fields | mixed | run summary | direct metadata plus calendar, sidecar inputs | `DirectRunPublicationFrame` metadata/identity/calendar | runfile/climate span/sidecar compatibility path | first-day precipitation alias, stale span, sidecar shortcut | JSON schema unchanged | BLOCKED behind skeleton publication frame |
| manifest | provenance/checksums/counters | mixed | run manifest | not wired in cutover mode | no production direct manifest projection | `HillslopeManifestPublication` compatibility provenance | text helper, output checksums from compatibility path, stale counters | manifest schema/checksum parity | BLOCKED: production writer remains compatibility-provenance based |

## Gate

FAIL. The table identifies the current direct fields and rejected aliases, but
the required authoritative direct producers are absent at the cutover boundary.
