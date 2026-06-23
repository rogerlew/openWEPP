# Profile And Blocker Ledger

Evidence class: Static plus Ran.

Status: executed-held.

## Profile Trigger

Direct default must be profiled if it misses `<=10x` legacy WEPP on H2637.

R7G did not produce a legitimate direct performance profile because direct
default failed closed before the H2637 hot loop. Profiling that failure path
would measure startup and error handling, not production direct runtime.

## Blocker Ledger

| ID | Status | Evidence | In-envelope? | Disposition |
|---|---|---|---|---|
| `HOLD-R7G-SURFACE-FREE-ACTIVE-SNOW-PARTITION-AUTHORITY-ABSENT` | accepted hold | Direct default and explicit direct H2637 both exit `1` at lane-1 active snow partition guard before timing. Static scan shows the existing snow partition helper calls `Wb11HydrologyKernel::compute_direct_snow_liquid_partition` with `HillslopeWritebackSurface` state/flux maps. | no | Scaffold a follow-up package to add typed direct active snow partition authority: parsed controls, persistent snowpack state, hourly winter forcing/partition inputs, direct state mutation, downstream operands for liquid input/storage/WAT/HBP, shadow projection, and active-snow fixtures. |

## Hot Function / Cost Classes

- Not run: direct endpoint did not reach the hot loop.
- Static cost class: active snow authority is still represented through
  compatibility surfaces for the only complete helper path.
- Protected boundary: wiring the map-backed helper into
  `DirectProductionDayInputBuilder` would reintroduce the hot-loop
  compatibility surface R7F removed.

## Iteration Notes

- Ran direct default candidate first; failed before output creation.
- Ran explicit direct production; failed on the same guard.
- Ran default-disabled compatibility and explicit rollback compatibility to
  preserve the same-binary matrix baseline.
- Focused R7 tests passed, including R7F source-scan tests proving production
  direct uses `DirectProductionDayInputBuilder` and its hot-loop body excludes
  runtime-surface reads.
