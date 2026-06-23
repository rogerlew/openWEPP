# Boundary Proof

Status: COMPLETE.

Static:

- New module: `crates/openwepp-hillslope-orchestrator/src/winter_column.rs`.
  It lives outside `direct_runtime` phase modules and contains typed
  winter-column state, forcing, and outcome structures only.
- Public module binding:
  `crates/openwepp-hillslope-orchestrator/src/lib.rs` declares
  `mod winter_column;` and exports the containment types.
- Direct runtime type visibility:
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs` imports
  `DirectWinterColumnState`.
- Frame ownership hooks:
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs`
  adds boxed `winter_column: Box<DirectWinterColumnState>` fields to
  `DirectLaneFrame` and `DirectDayFrame`, initializes them with
  `DirectWinterColumnState::zero()`, seeds day frames from lane frames, and
  commits day state back to lane state.

Type inventory:

- `DirectWinterColumnState`
- `DirectSnowLaneState`
- `DirectFrostLaneState`
- `DirectWinterDayForcing`
- `DirectWinterDayOutcome`
- `DirectWinterSnowOutcome`
- `DirectWinterFrostOutcome`
- `DirectWinterStorageOutcome`
- `DirectWinterPublicationOutcome`
- `DirectFrostLayerShadowState`
- `DirectFrostFineLayerState`

Forcing authority:

- `DirectWinterDayForcing` wraps the existing
  `runtime_inputs::DirectWinterHourlyForcing` and
  `DIRECT_WINTER_HOURLY_FORCING_COUNT` authority. This avoids creating a
  second hourly winter forcing definition in the new containment module.
- `DirectWinterDayOutcome` does not carry a `DirectWinterColumnState` snapshot;
  persistent end-of-day winter state remains the frame-owned
  `DirectWinterColumnState` authority.

Boundary scan:

Ran:

```bash
rg -n "DirectFrostRunoffSurface|HillslopeKernelRequest|HillslopeWritebackSurface|BoundarySymbol|BoundaryValue|WB13|BTreeMap|HashMap|Symbol" crates/openwepp-hillslope-orchestrator/src/winter_column.rs
```

Result: exit code 1 with no matches. `winter_column.rs` contains no
compatibility request surface, writeback surface, boundary symbol authority,
WB13 row authority, or map-backed symbol helper references.

Semantics:

- The containment state is inert at construction and is not yet consumed by
  solver or publication code.
- Existing `DirectSnowRuntimeCarry`, `DirectFrostRuntimeCarry`,
  `DirectFrostRunoffSurface`, and R4G/R4A consumers remain in place.
- No snow or frost solver math was moved into the new module.
- The only non-containment behavior change in this package is the documented
  active-frost no-freeze hourly diagnostic fast-path fix needed for an existing
  contract test; that fix does not make `winter_column.rs` a solver authority.
