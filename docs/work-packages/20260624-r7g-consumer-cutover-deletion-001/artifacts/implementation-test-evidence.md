# Implementation And Test Evidence

Status: COMPLETE.

Ran:
- Implemented typed hydrology outcome rename/addition:
  - `DirectWinterFrostComputeInputs`
  - `DirectWinterFrostPartitionOutcome`
  - `compute_direct_winter_frost_partition`
- Removed production bridge fields from direct-publication/day-frame surfaces:
  - `frost_runoff_surface`
  - `frost_liquid_partition`
- Cut R4A over to typed winter frost:
  - `run_r4a_runoff_partition_span_with_winter_frost` takes the current day
    typed payload by reference.
  - R4A computes from latest layer state plus `winter_column.frost`.
  - Active outcomes mutate `winter_column.frost` and the temporary carry mirror.
  - Inactive/no-material outcomes remain inert while preserving typed shadow and
    fine-layer evidence.
- Preserved frame-size budget:
  - Initial full-suite run failed because storing the compute payload on
    `DirectDayFrame` grew the frame to `12440 > 12288`.
  - Fixed by keeping the payload in day-input scope and borrowing it during R4A;
    focused layout test reports `DirectDayFrame=11464`.
- Updated tests:
  - Orchestrator R7G frost tests cover typed active/inactive outcomes and R4A
    winter-column mutation/commit.
  - Runner R7G source-scan tests prove direct production emits typed frost
    compute inputs and deletes the bridge.
  - Mechanical split moved R3C/R4B tests to
    `direct_runtime_r3c_r4b.rs` to satisfy line-count governance.

Focused checks:
- `cargo test -p openwepp-hillslope-orchestrator r7b_constructor_type_size_layout_is_bounded -- --nocapture`: passed.
- `cargo test -p openwepp-hillslope-orchestrator r7g_ -- --nocapture`: passed.
- `cargo test -p openwepp-runner r7g_direct_production -- --nocapture`: passed.
- `cargo test -p openwepp-hillslope-orchestrator direct_runtime_r3c_r4b -- --nocapture`: passed.
