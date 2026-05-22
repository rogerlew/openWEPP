# Comparator Tier Routing Metadata

Status: Draft (ARCH11)
Evidence: Ran + Static
Ran evidence:
- `cargo test --test comparator_tier_routing_metadata`
- `cargo test --workspace`

## Purpose

Define deterministic comparator confidence-tier routing metadata for reporting
outputs, aligned with ADR-0011 governance.

Implementation paths:
- `/home/workdir/openWEPP/crates/openwepp-comparator-metadata/src/lib.rs`
- `/home/workdir/openWEPP/crates/openwepp-summary-accumulator/src/lib.rs`

## Architecture Placement

- `openwepp-comparator-metadata` owns tier-routing policy, typed message IDs,
  and invalid-metadata error classes.
- `openwepp-summary-accumulator` attaches routed comparator metadata to each
  emitted summary rollup.
- Downstream comparator/reporting consumers receive explicit confidence-tier
  interpretation context with each rollup payload.

## Deterministic Tier Mapping

| surface class | confidence tier | route message id |
| --- | --- | --- |
| `single_ofe_daily_water_balance` | `higher_confidence` | `COMPMETA-HC-SINGLE-OFE-DAILY-001` |
| `hourly_water_balance` | `investigation` | `COMPMETA-I-HOURLY-001` |
| `watershed_water_balance` | `investigation` | `COMPMETA-I-WATERSHED-001` |

This mapping directly operationalizes ADR-0011:
- higher-confidence: single OFE + daily water-balance
- investigation-tier: hourly and watershed surfaces

## Invalid Metadata Policy (No Fallback)

Invalid routing metadata is rejected with typed errors; no default tier is
silently assigned.

Typed invalid-path message IDs:
- `COMPMETA-E-MISSING-OFE-COUNT`
- `COMPMETA-E-INVALID-OFE-COUNT`
- `COMPMETA-E-SINGLE-OFE-COUNT-MISMATCH`

## Summary Integration

`SummaryRollup` now carries:
- window + key + scalar totals
- `SimulationStatus` (ARCH03 semantics unchanged)
- `ComparatorTierRoutingMetadata`

ARCH10 daily/monthly/yearly/EOS rollup emission order and status message IDs are
preserved; ARCH11 only augments each rollup with explicit comparator-tier
metadata.
