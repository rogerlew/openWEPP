# Worker Handoff

Status: `executed`

Evidence mode: `static + ran`

W5 deletion work is implemented. No handoff blocker remains in current scope.

If a future package touches watershed physics, start from the typed direct path:

- `WatershedNetworkFrame`
- `execute_watershed_dispatch_with_frame`
- `Ws10ChannelImpoundmentKernel::run_direct_watershed_node`
- `publish_typed_routing_report`

Do not resurrect the deleted request/writeback watershed runtime. Generic or
hillslope writeback infrastructure is outside W5 and remains separately owned.
