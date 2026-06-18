# REFACTOR022 Split Plan

Evidence class: Static + Ran.

## Strategy

Behavior-preserving mechanical split only. The existing implementation text was moved into
ordered section files by responsibility and wired with `include!` from the original parent
module path. For impl-boundary sections, a local `impl` wrapper was added around the moved
methods; the method bodies, attributes, ordering, symbol names, and arithmetic remained
unchanged.

Final mechanical parity was checked against `HEAD` after stripping only wrapper lines for
impl-boundary sections:

```text
REFACTOR022_SECTION_MOVE_PARITY_OK
```

Raw parity evidence: `/tmp/refactor022/artifacts/section-move-parity.txt`.

## Split Surfaces

### Watershed Routing

Parent: `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing.rs`

- `routing/00_ws15_ws18_scaffold_and_hydraulics.rs` - WS15 scaffold and WS18 hydraulics.
- `routing/01_ws22_ws23_ws26_detachment.rs` - WS22/WS23/WS26 detachment and closure helpers.
- `routing/02_ws20_segment_routing.rs` - WS20 segment routing and transport helpers.

The original local `#[allow(clippy::similar_names)]` for
`derive_ws15_channel_sediment_scaffold` was preserved in the moved section.

### Scheduler Seed And Runtime

Parent:
`crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs`

- `scheduler_seed_and_runtime/00_wb11_runtime_seed.rs` - WB11 runtime seed helpers.
- `scheduler_seed_and_runtime/01_wb12_wb16_wb19_seed.rs` - WB12/WB16/WB19 seed helpers.
- `scheduler_seed_and_runtime/02_mofe03_wave2_seed.rs` - MOFE03 wave-2 seed helpers.
- `scheduler_seed_and_runtime/03_scheduler_lifecycle.rs` - scheduler kernel lifecycle.

The original local `#[allow(clippy::too_many_lines)]` for
`execute_scheduler_kernel_lifecycle` was preserved in the moved section.

### Kernel Core Types

Parent: `crates/openwepp-kernel-contract/src/lib_mod/core_types.rs`

- `core_types/00_symbol_registry_and_indexed_surfaces.rs` - symbol registry and indexed
  read/writeback surfaces.
- `core_types/01_typed_symbol_surfaces.rs` - typed symbol surface support.
- `core_types/02_boundary_values_and_kernel_requests.rs` - boundary values, kernel request,
  and request context types.

### Lateral Drainage

Parent:
`crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage.rs`

- `hydrology_phase_lateral_drainage/00_lateral_transfer.rs` - lateral transfer inputs and
  response.
- `hydrology_phase_lateral_drainage/01_tile_drainage.rs` - tile drainage inputs and response.
- `hydrology_phase_lateral_drainage/02_ksat_adjustment.rs` - Ksat adjustment helpers.

## Non-Goals

No `SC-*` contract changes, no output schema changes, no numerical edits, no public API
changes, and no forced split of the deferred 2000-2500 line WARN tier.
