# Consumer Path Evidence

Status: `passed-for-hold`

Evidence mode: `Static:` source inspection plus `Ran:` public CLI probe.

## Public Consumer Path

- Entrypoint: `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- Run plan: `WatershedRunPlan`
- Worker fanout: `WatershedRunPlan::execute_hillslope_jobs`
- Pass handoff: `PassInventory::validate`
- Typed routing frame: `WatershedNetworkFrame`
- Dispatch: `execute_watershed_dispatch_with_frame`
- Typed publication: `WatershedNetworkFrame::publish_typed_routing_report`
- Output writer: `write_typed_publication_parquet_outputs`

Static source inspection confirms the public CLI routes through typed frame and
typed publication. The W7 production edit preserves that path.

`/tmp/wshedw7_probe_carn/out` proves the public CLI can complete after the
path-resolution fix. It does not close W7 because the fixture sediment signal is
zero.
