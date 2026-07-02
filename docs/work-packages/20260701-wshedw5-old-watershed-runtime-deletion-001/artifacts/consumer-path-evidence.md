# Consumer Path Evidence

Status: `executed`

Evidence mode: `static + ran`

Typed consumer path:

1. `openwepp-cli-watershed` builds `WatershedNetworkFrame::from_parsed_inputs`.
2. The CLI adds `HillslopeContribution` records from generated or reused HBP
   pass inventory.
3. The public route calls `execute_watershed_dispatch_with_frame`.
4. Direct dispatch invokes `Ws10ChannelImpoundmentKernel::run_direct_watershed_node`
   over typed frame records.
5. Channel and impoundment outputs are recorded as `RoutedChannelState` and
   `RoutedImpoundmentState`.
6. The output path consumes `publish_typed_routing_report` and converts the
   typed publication frame into watershed output row seeds.

Real-consumer proof:

- `typed_frame_dispatch_records_and_publishes_direct_routed_state` proves routed
  typed state is recorded and then consumed by typed publication.
- `wshedw5_public_cli_uses_typed_network_and_publication_frames` proves the
  public CLI contains the typed handoff markers and does not contain old
  symbol-map surface markers.
- Existing runner output tests in `watershed_cli_behavior_contract` continue to
  exercise parquet publication and are included in the full workspace profile.

Negative proof:

- Production source scan has no old watershed request/writeback runtime symbols.
- The only remaining `WatershedKernel` string matches are
  `SimulationPhase::WatershedKernel`, the live status phase.
