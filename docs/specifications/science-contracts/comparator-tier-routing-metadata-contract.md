# Comparator Tier Routing Metadata Contract

Status: Draft (ARCH11)
Evidence: Ran + Static
Ran evidence:
- `cargo test --test comparator_tier_routing_metadata`

## Purpose

Specify the typed comparator confidence-tier routing contract for
summary/reporting outputs.

Implementation paths:
- `/home/workdir/openWEPP/crates/openwepp-comparator-metadata/src/lib.rs`
- `/home/workdir/openWEPP/crates/openwepp-summary-accumulator/src/lib.rs`

## Contract Input Surface

### Routing request
`ComparatorTierRoutingRequest` fields:
- `surface_class: ComparatorSurfaceClass`
- `contributor_ofe_count: Option<u32>`

Surface classes:
- `SingleOfeDailyWaterBalance`
- `HourlyWaterBalance`
- `WatershedWaterBalance`

### Validation rules
- `contributor_ofe_count` must be present for `SingleOfeDailyWaterBalance`.
- `contributor_ofe_count` (when provided) must be `>= 1`.
- `SingleOfeDailyWaterBalance` requires `contributor_ofe_count == 1`.

## Contract Output Surface

`ComparatorTierRoutingMetadata` fields:
- `surface_class: ComparatorSurfaceClass`
- `confidence_tier: ComparatorConfidenceTier`
- `message_id: &'static str`

Confidence tiers:
- `HigherConfidence`
- `Investigation`

## Deterministic Mapping Table

| surface class | contributor_ofe_count constraint | confidence tier | message id |
| --- | --- | --- | --- |
| `SingleOfeDailyWaterBalance` | must be present and equal to `1` | `HigherConfidence` | `COMPMETA-HC-SINGLE-OFE-DAILY-001` |
| `HourlyWaterBalance` | optional; if provided must be `>= 1` | `Investigation` | `COMPMETA-I-HOURLY-001` |
| `WatershedWaterBalance` | optional; if provided must be `>= 1` | `Investigation` | `COMPMETA-I-WATERSHED-001` |

## Typed Invalid-Path Surface

`ComparatorTierRoutingError` variants:
- `MissingRequiredMetadata { field, message_id }`
- `InvalidContributorOfeCount { contributor_ofe_count, message_id }`
- `SingleOfeCountMismatch { contributor_ofe_count, message_id }`

Failure-class mapping:
- `MissingRequiredMetadata` -> `MissingRequiredMetadata`
- `InvalidContributorOfeCount` -> `InvalidMetadata`
- `SingleOfeCountMismatch` -> `InvalidMetadata`

Invalid-path message IDs:
- `COMPMETA-E-MISSING-OFE-COUNT`
- `COMPMETA-E-INVALID-OFE-COUNT`
- `COMPMETA-E-SINGLE-OFE-COUNT-MISMATCH`

## Summary Integration Contract

`SummaryRollup` carries routed `ComparatorTierRoutingMetadata` in addition to
ARCH10 rollup status/total fields.

`SummaryAccumulator::new(routing_request)` validates routing metadata up front;
invalid routing metadata returns typed `SummaryAccumulatorError::ComparatorMetadata`
with no fallback/default tier assignment.

## Invariants

- `INV-COMPMETA-001`: tier mapping is deterministic for each surface class.
- `INV-COMPMETA-002`: single OFE daily routes only when OFE count is exactly `1`.
- `INV-COMPMETA-003`: hourly/watershed surfaces always map to investigation tier.
- `INV-COMPMETA-004`: invalid routing metadata is explicit typed failure.
- `INV-COMPMETA-005`: ARCH10 summary window/status semantics remain unchanged by routing integration.
