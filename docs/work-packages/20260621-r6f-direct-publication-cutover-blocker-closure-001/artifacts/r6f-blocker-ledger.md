# R6F Blocker Ledger

Status: executed-held.

## Active Iteration Ledger

| # | First seen | Marker | Output family | Smallest reduced unit | Direct operand | Producer/consumer | Authority | In envelope? | Correction attempted | Validation result | State |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | R6E | `HOLD-R6E-HBP-DIRECT-PROCESS-PARITY-MISMATCH` | HBP | near-zero `peakro` and `watdur` latest-event fields in equal-length HBP streams | `publication.runoff.peak_runoff_m3_s`, `publication.runoff.runoff_duration_s` | `DirectPublicationDayRow::from_day_frame` -> HBP builder | Array-native R6 ledger; WB16 near-zero runoff constants | Yes for inherited near-zero fixture | Added `direct_publication_peak_runoff_operands` using typed direct `q_runoff_m` and WB16 thresholds. | HBP payload and byte identity passed on current fixture. | Fixture closed |
| 2 | R6F | `HOLD-R6F-WAT-DIRECT-PROCESS-PRODUCER-AUTHORITY-GAP` | WAT | First-row fields `wepp_id`, `year`, `Es`, `Total-Soil`, `SoilWaterTotal`, `ProfileDepth`, `ProfilePorosityCap`, `ProfileFCStore`, `ProfileWPStore` | direct publication identity, `publication.evaporation.es_mm`, `publication.storage.total_soil_mm`, `publication.storage.soil_water_total_mm`, `publication.profile.*` | Direct runtime can now accept typed process inputs and carry layers; production runner still supplies only climate/calendar. | R6 ledger section 5.2.1; `SC-EVAP-001`; `SC-SYSTEM-001` publication lineage. | Boundary for R6F after structural direct-runtime correction; implementation producer scaffolded as R6G. | Added optional typed process input slots, persistent layer carry, profile-depth/porosity projection fields, climate unit correction, and tests. Rejected WB13/runtime-surface wrapper. | Focused tests pass; cutover remains fail-closed at stable R6F marker. | Held |

## Blocker Reduction Notes

### 1. HBP Near-Zero Runoff Event Operands

Command:

`cargo test -p openwepp-runner r6f_cutover_candidate_hbp_identity_exposes_wat_producer_gap -- --nocapture`

Result: passed after correction. Direct and compatibility HBP bytes are equal
for the inherited near-zero runoff fixture. Nonzero peak-runoff/event-duration
fixture coverage remains required before full R6 HBP closure.

### 2. WAT Direct Process Producer Authority

Command:

`cargo test -p openwepp-runner r6f_cutover_candidate_hbp_identity_exposes_wat_producer_gap -- --nocapture`

Result: passed as blocker-reduction evidence. The test asserts direct and
compatibility WAT rows agree on `P`, `RM`, `Q`, and `QOFE`, then asserts the
reduced field set still differs.

The remaining producer cannot be created by reading compatibility WB13 rows or
runtime surfaces. R6F added the required direct-runtime receiving surface:

- `DirectPublicationDayInput.initial_soil_water_m`
- `DirectPublicationDayInput.percolation_inputs`
- `DirectPublicationDayInput.subsurface_compute_inputs`
- `DirectPublicationDayInput.evapotranspiration_compute_inputs`
- `DirectPublicationDayInput.hydrology_projection_inputs`
- lane-carried `DirectSubsurfaceLayerState`
- hydrology projection `profile_depth_m` and `profile_porosity_cap_m`

R6G must bind those from parsed typed inputs and direct state under canonical
authority.

## Closed Blockers

| # | Closed date | Closing commit | Evidence |
|---|---|---|---|
| 1 | 2026-06-21 | Pending commit | Inherited near-zero fixture only: `r6f-hbp-byte-diff.md`; focused HBP identity test. |
