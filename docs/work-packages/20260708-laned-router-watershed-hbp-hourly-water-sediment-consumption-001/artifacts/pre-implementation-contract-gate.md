# Pre-Implementation Contract Gate

Status: `EXECUTED`
Evidence mode: `Static` plus `Ran`

Pre-edit audit:

- Current `direct.rs` selected hourly resolution only when every hillslope
  contributor carried both arrays and the inlet had no dependency nodes.
- A mixed contributor set returned `false` from the complete-pair predicate and
  could reach the triangular fallback. That contradicted the M-T3 fail-closed
  package requirement.
- The HBP parser and run-level inventory already had structural/intake guards
  for malformed minor-1 payloads.

Contract-derived tests added:

- `mt3_hourly_pair_distribution_changes_channel_water_and_sediment_outputs`
  proves production frame dispatch consumes hourly water and sediment timing.
- `mt3_mixed_or_malformed_hourly_pair_fails_closed_before_routing_state`
  proves mixed and malformed hourly authority fail with
  `WKERNEL-WS10-CHANNEL-E-003` before routed channel state is written.

Ran:

- `cargo nextest run --test wshedw5_typed_watershed_runtime_contract`:
  16 tests passed.
