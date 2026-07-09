# Source Map

Status: `EXECUTED-COMPLETE`
Evidence: `Static`

## Producer Surfaces

- Channel routing kernel:
  `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs`
  - `runvol_case` becomes typed `channel_inflow_m3`.
  - `roff` remains routed runoff/outflow and becomes typed `channel_outflow_m3`.
  - Current direct lane publishes explicit `0.0` storage/loss operands until
    separate channel storage/transmission-loss physics owns nonzero terms.
- Routed channel state:
  `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs`

## Publication Surfaces

- Typed watershed publication frame:
  `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs`
  - Sums routed channel `channel_*_m3` operands over dispatched channels.
- Public parquet writer:
  `crates/openwepp-watershed-output/src/writers.rs`
  - `chanwb` `Inflow (m^3)` reads typed channel inflow, while generic `value`
    remains watershed runoff for existing dynamic-value schemas.
  - `Balance (m^3)` is reconstructed as
    `Inflow - Outflow - Loss - Storage`.

## Tests

- Typed routing/publication integration:
  `tests/integration/wshedw5_typed_watershed_runtime_contract.rs`
- Writer projection tests:
  `crates/openwepp-watershed-output/src/writers.rs`
