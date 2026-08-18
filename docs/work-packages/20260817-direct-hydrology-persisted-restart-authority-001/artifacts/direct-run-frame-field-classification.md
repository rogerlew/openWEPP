# DirectRunFrame persistence classification

Status: `executing / contract-first inventory`

The restart format is a versioned DTO and never a serialized Rust frame.

| `DirectRunFrame` field | Classification | Restart obligation |
|---|---|---|
| `identity` | immutable configuration reference | persist run/day/lane identity and exact topology receipt |
| `lanes` | persisted continuation state | project every lane water/soil/transfer continuation field in canonical lane order |
| `phase_plan` | immutable configuration reference | bind configuration digest; reconstruct from admitted run configuration |
| `publication` | publication-only | never restore as owner state; reproduce outputs from continued execution |
| `lane_transfer_ledger` | persisted continuation state | persist exact ordered transfer custody |
| `lane_transfer_downstream_operands` | persisted continuation state | persist exact accepted downstream operands |
| `lane_transfer_shadow_projection` | deterministically reconstructed cache | rebuild and exact-check from transfer ledger and topology |
| `groundwater` | persisted continuation state | persist complete enabled/disabled owner state |
| `surface_liquid_shadow` | persisted continuation state | persist typed owned state and configuration receipt |
| `laned_active` | unsupported | V10 shadow remains default-off and rejects active routing before mutation |
| `laned_active_summary` | ephemeral diagnostic | absent for the supported domain; never restored as owner state |

## `DirectLaneFrame` field classification

| Field | Classification | Restart obligation |
|---|---|---|
| `lane_id`, `upstream_lane_id`, `downstream_lane_id` | immutable topology | bind ordered topology and reconstruct/check exact values |
| `upstream_area_ratio`, `area_m2` | immutable topology/configuration | bind exact binary64 bits |
| five `runoff_publication_*` scalars | immutable publication configuration | bind configuration digest; reconstruct, do not persist as mutable state |
| `water` | persisted continuation owner | explicit complete water-state DTO |
| `transfer` | persisted continuation owner | explicit complete transfer-buffer DTO |
| `publication` | publication-only | exclude and reconstruct empty scratch |
| `erosion_downstream_operands` | persisted continuation owner | explicit erosion operand DTO |
| `erosion_inflow_intake` | persisted continuation owner | preserve optional intake and identity |
| `subsurface_layers` | persisted continuation owner | ordered complete layer DTOs with topology/layer identity |
| `evapotranspiration_stage_state` | persisted continuation owner | preserve exact optional stage state |
| `plant_growth_state` | persisted continuation owner | preserve complete growth state |
| `plant_water_stress` | persisted continuation owner | preserve exact binary64 bits |
| `winter_column` | persisted continuation owner | preserve even though the V10 consumer rejects snow-active execution; snow-free state must remain exact |
| `snow_runtime_carry` | persisted continuation owner | preserve optional carry; reject snow-active handoff outside supported domain |
| `frost_runtime_carry` | persisted continuation owner | preserve optional carry and layer order |
| `erosion_runtime_carry` | persisted continuation owner | preserve complete day-to-day erosion carry |
| `day_inputs` | immutable future input/configuration | bind ordered canonical digest and reconstruct from admitted run input; never persist as mutable owner |

`surface_liquid_shadow` inside the hydrology frame is the sole surface-liquid
state representation. The checkpoint's surface-liquid configuration binds and
validates that embedded owner; it does not carry a second state copy.

The complete `DirectV10RealConsumerCheckpointV1` additionally persists the
CP-GSI01 beginning/ending state and accepted daily receipt, forcing-provider
beginning/ending cursor, canonical V10 vegetation, LSE-V2, soil thermal, BGC,
scheduler day/interval, and an in-progress-day envelope. Transient V9 and
LSE-V1 projections are reconstructed after deserialization and must match the
canonical successor payload exactly before the checkpoint can be admitted.

No owner is replaced until every field, digest, topology, lineage, receipt,
and reconstructed projection has validated. Replacement is one non-fallible
assignment of the fully validated candidate envelope.
