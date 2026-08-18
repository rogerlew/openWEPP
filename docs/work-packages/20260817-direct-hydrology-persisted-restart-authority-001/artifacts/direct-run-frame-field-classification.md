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

The complete `DirectV10RealConsumerCheckpointV1` additionally persists the
CP-GSI01 beginning/ending state and accepted daily receipt, forcing-provider
beginning/ending cursor, canonical V10 vegetation, LSE-V2, soil thermal, BGC,
scheduler day/interval, and an in-progress-day envelope. Transient V9 and
LSE-V1 projections are reconstructed after deserialization and must match the
canonical successor payload exactly before the checkpoint can be admitted.

No owner is replaced until every field, digest, topology, lineage, receipt,
and reconstructed projection has validated. Replacement is one non-fallible
assignment of the fully validated candidate envelope.
