# Protected Output Evidence

Status: `executed`

Evidence mode: `static + ran`

W5 deletes the retired watershed request/writeback runtime and does not change
the public watershed output schema. Output protection is carried by:

- typed publication consumer proof in
  `typed_frame_dispatch_records_and_publishes_direct_routed_state`;
- public CLI source guard proving the CLI still uses
  `publish_typed_routing_report` and `publication_frame_to_row_seed`;
- existing watershed CLI parquet/publication tests in
  `watershed_cli_behavior_contract`, included in the full workspace profile.

No contract-governed output delta was introduced. The focused typed publication
test verifies nonzero channel runoff and particulate pollutant publication from
typed frame state; full runner tests cover parquet emission and branch
publication continuity.
