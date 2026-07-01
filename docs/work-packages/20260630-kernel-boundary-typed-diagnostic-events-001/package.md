# Kernel-Boundary Typed Diagnostic Events

Status: EXECUTED-HOLD-TRACE-CONSUMER-PAYLOAD-SOURCES-MISSING

Package id: `20260630-kernel-boundary-typed-diagnostic-events-001`

## Objective

Execute kernel-boundary step 2 follow-on #1: move TRACE-class diagnostic and
trace consumers off symbol-map carrier types into typed event payloads emitted
by the direct runtime, preserving output identity and changing no physics.

## Authority

- [Array-native runtime specification](../../architecture/array-native-runtime-specification.md)
  §8.2 step 3 orders diagnostics/trace migration before hydrology
  phase-boundary migration.
- `20260630-kernel-boundary-survivor-inventory-001/` classifies the remaining
  TRACE surface and names typed diagnostic/event payloads as follow-on #1.
- [ADR-0025](../../decisions/0025-array-native-hillslope-day-frame.md) remains
  the typed array-native runtime authority.
- [ADR-0031](../../decisions/0031-delete-compatibility-runtime-single-authority-terminal.md)
  authorizes terminal single-authority deletion after typed consumers exist.

## Scope

In scope:

- Define authority-bearing typed diagnostic event payloads in the direct
  runtime.
- Repoint direct-runtime trace writers that already have typed frame state to
  consume those payloads instead of ad hoc local references.
- Scan the TRACE-class carrier-reference count for the requested survivor files.
- Determine whether the package can honestly close the requested TRACE-family
  migration gate.

Out of scope:

- No hydrology phase-boundary typing.
- No scheduler, day-frame, carrier-type, or compatibility-support deletion.
- No output schema change.
- No physics change.
- No public runtime selector change.

## Implementation

This package added typed direct-runtime diagnostic event payloads for existing
direct trace writers:

- `DirectRunoffRebalanceTraceEvent`
- `DirectEvapotranspirationTraceEvent`
- `DirectPercolationTraceEvent`
- `DirectSubsurfaceSaturationTraceEvent`

The direct trace writers now consume those event payloads rather than pulling
fields directly from local runtime state at the writer boundary. The payloads
are constructed from `DirectDayFrame` and other direct typed state; they do not
wrap `HillslopeKernelRequest`, `HillslopeWritebackSurface`, or symbol lookup.

## Discovery

The requested TRACE files do not yet have typed event sources that can replace
their symbol-carrier reads:

- `frost_entry.rs` is mixed ownership. The R7G direct frost trace writer already
  takes typed frost contexts; the remaining `HillslopeKernelRequest`
  references are active-frost kernel-boundary readers, not a thin trace sink.
- `hphys_trace.rs` builds rows by scanning scheduler-era
  `HillslopeWritebackSurface` values. A correct migration requires a typed
  HPHYS publication/event source from direct runtime state.
- `indexed_shadow_surface.rs` and `symbol_registry_audit.rs` audit symbol-map
  shadows and registries. They should be replaced by typed source-scan/audit
  guards or deleted after carrier removal, not wrapped.
- `perfdeep02_frame_roundtrip.rs` exists to round-trip
  `HillslopeWritebackSurface` through `HillslopeDayFrame`. It is deletion-ready
  after typed diagnostics and tests replace it, but converting it to a typed
  payload without changing purpose would be a false migration.

## Gates

| Gate | Status | Evidence |
| --- | --- | --- |
| Direct typed event payloads are authority-bearing typed state | PASS | [Implementation log](artifacts/implementation-log.md) |
| TRACE-class carrier-reference count drops | FAIL | [Progress scan](artifacts/progress-scan.md) |
| Protected output identity | NOT RUN | Not run because the package holds before consumer migration |
| Trace output identity | NOT RUN | Not run because no requested TRACE-family consumer was repointed |
| Full workspace gates | NOT RUN | Not run because the current progress gate failed |
| Focused compile after code changes | PASS | [Gate evidence](artifacts/gates.md) |

## Disposition

Result: `EXECUTED-HOLD-TRACE-CONSUMER-PAYLOAD-SOURCES-MISSING`.

The package lands a useful typed direct-runtime event-payload foundation, but it
does not claim completion of kernel-boundary step 2. The current hold blocker is
not a failing Rust build; it is a missing source-of-truth seam for the requested
TRACE families. The next executable slice should first add typed direct
publication/diagnostic event sources for HPHYS and scheduler-era trace rows, or
split the mixed `frost_entry.rs` kernel-boundary work into the phase-family
typing package.
