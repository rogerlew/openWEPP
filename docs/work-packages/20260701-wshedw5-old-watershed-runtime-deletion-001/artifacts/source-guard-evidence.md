# Source Guard Evidence

Status: `executed`

Evidence mode: `static + ran`

Focused source guard:

```text
cargo nextest run -p openwepp-runner --test watershed_cli_behavior_contract wshedw5_public_cli_uses_typed_network_and_publication_frames
Summary: 1 test run: 1 passed, 23 skipped
```

The guard scans the public CLI plus frame, dispatch, type, kernel-core,
included kernel helper/diagnostic/validation/routing files, direct kernel, and
runtime-input exports. It forbids:

- `WatershedWritebackSurface`
- `WatershedKernelExecutionReport`
- `WatershedKernelStepReport`
- `WatershedKernelRequest`
- `impl WatershedKernel for Ws10ChannelImpoundmentKernel`
- `execute_watershed_dispatch_with_kernel`
- `execute_watershed_dispatch_with_gate_and_kernel`
- `compatibility_writeback_surface`
- `harvest_compatibility_routing_report`
- `build_watershed_runtime_surface`
- `seed_watershed_runtime_surface`
- `WatershedClimateRuntime`

Independent verification agent also wrote:

- `artifacts/verification/production_old_surface_scan.log`
- `artifacts/verification/source_guard_scan.log`
- `artifacts/verification/wshedw5_typed_runtime_test.log`
- `artifacts/verification/watershed_cli_behavior_test.log`

Those logs passed for the pre-review focused checks; the parent reran the
expanded typed/source-guard gates after addressing review findings.
