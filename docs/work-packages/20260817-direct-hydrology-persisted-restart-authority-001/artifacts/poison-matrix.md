# Restart V1 poison matrix

Status: draft; executable poison coverage pending

| Poison | Typed category | Mutation permitted |
|---|---|---|
| missing / extra / reordered / duplicate member | `missing_field` / `extra_field` / `reordered_field` / `duplicate_field` | no |
| malformed or noncanonical JSON / binary64 | `noncanonical_bytes` | no |
| schema or version substitution | `schema` / `unsupported_version` | no |
| payload bit flip | `payload_digest` | no |
| run, lane, OFE, tile, layer-map substitution | `run_identity` / `topology_identity` | no |
| configuration or owner digest substitution | `configuration_identity` / `owner_identity` | no |
| transaction gap, replay, or substitution | `transaction_lineage` | no |
| wrong day or interval | `scheduler_position` | no |
| cursor rewind, skip, or midnight mismatch | `provider_cursor` | no |
| wrong daily GSI receipt or ending state | `gsi_receipt` | no |
| unequal lane GSI receipts | `heterogeneous_lane_gsi_receipt` | no |
| forcing receipt missing, extra, reordered, duplicated, corrupt | `forcing_receipt_cardinality` / `forcing_receipt_order` / `forcing_receipt_digest` | no |
| transient V9 differs from V10 projection | `v10_v9_projection` | no |
| transient LSE-V1 differs from LSE-V2 projection | `lse_v2_v1_projection` | no |
| invalid owner domain | `owner_validation` | no |
| `laned_active` present | `unsupported_laned_active` | no |

Every row must be tested against an isolated candidate before release. The live
owner bytes must remain identical until every row has passed and final
replacement is non-fallible.
