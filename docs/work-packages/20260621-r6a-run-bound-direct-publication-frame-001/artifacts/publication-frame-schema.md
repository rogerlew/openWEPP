# Publication Frame Schema

Status: implemented.
Evidence mode: Static.

The frame must cover the promoted R6 ledger from
`docs/architecture/array-native-runtime-specification.md` section `5.2.1`.

Required frame groups:

| Group | Required operands |
|---|---|
| Identity/calendar | `wepp_id`, `ofe_id`, `sim_day_index`, `julian`, `month`, `day_of_month`, `water_year`. |
| Climate/liquid input | `precipitation_mm`, `rm_mm`, irrigation/liquid-input components required for anti-alias tests. |
| Runoff | `q_mm`, `qofe_mm`, `runvol_m3`, `peak_runoff_m3_s`, `runoff_duration_s`. |
| ET | `ep_mm`, `es_mm`, `er_mm`. |
| Subsurface | `dp_mm`, `latqcc_mm`, `tile_mm`, `sbrunv_m3`. |
| Transfer | `upstream_surface_mm`, `upstream_lateral_mm`. |
| Storage/snow/frost/profile | total soil, soil-water alias, frozen water, frost depth, snow water, profile depth, porosity cap, FC, WP. |
| Interception | interception flux and optional storage. |
| Erosion/HBP | detachment, deposition, sediment concentration classes, particle metadata. |
| loss JSON | run name, first/last day, climate/executed day counts, static sidecar fields. |
| manifest | input/output checksums, runtime selection, direct counters, warning IDs, output policy, schema/provenance metadata. |

Fields without current authoritative direct producers must be explicit optional
or absent-authority fields with tests that prevent silent defaulting. A missing
required direct producer is a typed error, not a compatibility fallback.

Implemented binding:

- `DirectRunPublicationFrame`
  - `identity: DirectRunIdentity`
  - `metadata: DirectPublicationRunMetadata`
  - `rows: Vec<DirectPublicationDayRow>`
- `DirectPublicationDayRow`
  - identity/calendar fields: `run_id`, `hillslope_id`, `lane_id`, `ofe_id`,
    `lane_index`, `day_index`, `sim_day_index`, `calendar`, `area_m2`;
  - `climate: DirectPublicationClimateOperands`;
  - `liquid_input: DirectPublicationLiquidInputOperands`;
  - `runoff: DirectPublicationRunoffOperands`;
  - `evaporation: DirectPublicationEvaporationOperands`;
  - `subsurface: DirectPublicationSubsurfaceOperands`;
  - `transfer: DirectPublicationTransferOperands`;
  - `storage: DirectPublicationStorageOperands`;
  - `profile: DirectPublicationProfileOperands`;
  - `interception: DirectPublicationInterceptionOperands`;
  - `erosion: DirectPublicationErosionOperands`.

Current absent-authority operands are explicit `Option` fields:

- HBP runoff duration, peak runoff, detachment, deposition, and sediment classes
  unless direct erosion/event producers fill them;
- frost depth;
- profile depth and porosity cap;
- interception storage.

The runner direct publication shadow mode builds this frame from parsed slope
OFE area and climate calendar plus typed direct day-frame state. Compatibility
WB13 rows/runtime surfaces are not constructor inputs.
