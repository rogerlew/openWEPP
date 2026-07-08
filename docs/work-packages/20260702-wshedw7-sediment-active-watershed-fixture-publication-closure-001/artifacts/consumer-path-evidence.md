# Consumer Path Evidence

Status: `passed`

Evidence mode: `Static:` source inspection plus `Ran:` focused test.

Public consumer path:

- `openwepp-cli-watershed`
- `WatershedRunPlan::execute_hillslope_jobs`
- `PassInventory::validate`
- `WatershedNetworkFrame`
- `execute_watershed_dispatch_with_frame`
- `WatershedNetworkFrame::publish_typed_routing_report`
- `write_typed_publication_parquet_outputs`

The W7R focused test proves the public path consumes the generated p102 HBP
payload and publishes nonzero sediment through public parquet outputs.

Detailed evidence is in `publication-consumer-proof.md`.
