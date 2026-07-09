# Source Map

Status: `EXECUTED`
Evidence mode: `Static` plus focused `Ran` tests.

| Surface | Current binding | M-T3 disposition |
| --- | --- | --- |
| Active HBP producer | `crates/openwepp-runner/src/hillslope/04_direct_publication.rs` `build_hbp_output_from_direct_publication_summary` / `assemble_hbp_event_sediment_surfaces` | Minor-1 EVENT rows write `hourly_runoff_volume_m3[24] = runoff.runvol_m3 * hourly_runoff_fraction[h]` and `hourly_sediment_mass_kg[24]` from the hourly erosion surface. |
| HBP parser/intake | `crates/openwepp-input-contract/src/parsers/hbp/payload_validator.rs` | Parser requires the minor-1 arrays as a 24-slot finite non-negative pair; malformed count or element violations fail with `HBP-E-015`. |
| Run-level inventory | `crates/openwepp-runner/src/watershed_supervisor.rs` `validate_latest_event_vectors` | Inventory validates class vector cardinality and the sediment-side hourly telescoping identity before routing. |
| Watershed supervisor handoff | `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` pass inventory loop | `HbpLatestEventState::EventPayload` clones both hourly arrays into `HillslopeContribution`; no report, sidecar, or text fallback supplies the arrays. |
| Typed frame | `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs` `HillslopeContribution` | Carries `hourly_runoff_volume_m3` and `hourly_sediment_mass_kg` with the parsed contribution. |
| Water consumer | `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs` `assemble_direct_incoming_peak_partition` | Complete all-hourly inlets use `max_h(sum V_h)/3600`, exact hourly volume sum, and active-hour span. |
| Sediment consumer | `direct.rs` `read_direct_hillslope_sediment_payload` and `assemble_direct_incoming_sediment_load_and_capacity` | Uses `sum S_h` as sediment mass and the superposed `S_h` active-hour span as the quasi-steady `qsed` time base. |
| Fail-closed guard | `direct.rs` `direct_hillslope_hourly_authority` | New M-T3 guard rejects partial, malformed, mixed hourly/non-hourly contributors and hourly contributors with dependency nodes that lack channel-hourly surfaces. |
| CLI consumer proof | `crates/openwepp-runner/tests/mt3_hbp_hourly_consumer_contract.rs` | Writes schema-1.1 HBP fixtures, runs `openwepp-cli-watershed`, and proves equal daily totals with different hourly distributions change `ebe_pw0` consumer surfaces. |
| Publication/output | `WatershedPublicationFrame` through watershed output writer | The closure proof names routed channel state surfaces: `peak_discharge_m3_s`, `channel_inflow_m3`, and `sediment_state.qsed_kg_s`. |

Old-path check:

- All-no-hourly contributor inlets still use the contract-authorized
  Eq. [13.4.1]-[13.4.2] triangular fallback.
- Partial/mixed hourly inputs no longer silently reach that fallback.
- The active hourly closure claim is carried only by the HBP parser ->
  pass inventory -> `HillslopeContribution` -> WS10 direct kernel path.
