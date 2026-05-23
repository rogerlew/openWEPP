# WS10 Watershed Kernel Authority and Guard Map

Status: `completed`
Evidence mode: `Static + Ran`

## Production Kernel Surfaces
- Kernel type: `Ws10ChannelImpoundmentKernel`
- Implementation file: `crates/openwepp-watershed-orchestrator/src/lib.rs`
- Core entrypoints:
  - `run_channel_node(...)`
  - `run_impoundment_node(...)`
  - `impl WatershedKernel for Ws10ChannelImpoundmentKernel`

## Runtime Projection Surfaces
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
  - `seed_watershed_runtime_surface_from_watershed_channel(...)`
  - `seed_watershed_runtime_surface_from_watershed_impoundment(...)`

## Channel Lane Guard Map
- Guard family:
  - `WKERNEL-WS10-CHANNEL-E-001` -> missing required input
  - `WKERNEL-WS10-CHANNEL-E-002` -> non-finite input
  - `WKERNEL-WS10-CHANNEL-E-003` -> domain/dependency violation
- Required symbol families:
  - global controls: `dtchr`, `nchnum`, `cbase`
  - per-node controls: `ws10_channel_{id}_chnn`, `ws10_channel_{id}_ctlslp`, `ws10_channel_{id}_chnk`
  - hillslope contributor payloads: `hs{ID}_peakro`, `hs{ID}_watdur`
  - upstream dependency payloads:
    - `ws10_channel_{id}_qpo`, `ws10_channel_{id}_durrof`
    - `ws10_impoundment_{id}_qo`, `ws10_impoundment_{id}_durout`
- Published outputs:
  - state: `ws10_channel_{id}_qpo`, `ws10_channel_{id}_durrof`
  - flux: `ws10_channel_{id}_roff`

## Impoundment Lane Guard Map
- Guard family:
  - `WKERNEL-WS10-IMPOUNDMENT-E-001` -> missing required input
  - `WKERNEL-WS10-IMPOUNDMENT-E-002` -> non-finite input
  - `WKERNEL-WS10-IMPOUNDMENT-E-003` -> domain/dependency violation
- Required symbol families:
  - per-node controls:
    - `ws10_impoundment_{id}_h`
    - `ws10_impoundment_{id}_hfull`
    - `ws10_impoundment_{id}_deltat`
    - `ws10_impoundment_{id}_qinf`
  - hillslope contributor payloads: `hs{ID}_peakro`, `hs{ID}_watdur`
  - upstream dependency payloads:
    - `ws10_channel_{id}_qpo`, `ws10_channel_{id}_durrof`
    - `ws10_impoundment_{id}_qo`, `ws10_impoundment_{id}_durout`
- Published outputs:
  - state:
    - `ws10_impoundment_{id}_qo`
    - `ws10_impoundment_{id}_durout`
    - `ws10_impoundment_{id}_hnext`
  - flux:
    - `ws10_impoundment_{id}_outflow_volume`

## Contract Authority Alignment
- `SC-ROUTE-001` WS10 addendum governs channel lane symbols, guards, and vectors.
- `SC-IMPOUND-001` WS10 addendum governs impoundment lane symbols, guards, and vectors.
- `SC-HYDRAULICS-001` WS10 addendum governs consumer-side coupling expectations.
- `SC-SYSTEM-001` WS10 addendum governs deterministic dependency publication and hard-fail posture.
