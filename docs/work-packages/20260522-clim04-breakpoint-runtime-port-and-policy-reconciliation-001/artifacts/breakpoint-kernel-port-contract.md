# Breakpoint Kernel Port Contract (CLIM04)

Evidence mode: `Static`
Status: `implemented`

## Scope
Port legacy breakpoint (`ibrkpt=1`) runtime event-shape behavior from `/workdir/wepp-forest_260430_baseline/src/brkpt.for` into openWEPP runtime seam adapters:
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`

## Ported Semantics
For each breakpoint day:
1. Capture storm start hour: `stmstr = breakpoints[0].timem`.
2. Convert time/depth units:
- `timem_elapsed_s(i) = (timem_h(i) - stmstr) * 3600`
- `pptcum_m(i) = pptcum_mm(i) * 0.001`
3. Build interval intensities for `i=1..nbrkpt-1`:
- `drain_m = pptcum_m(i) - pptcum_m(i-1)`
- `dtime_s = timem_elapsed_s(i) - timem_elapsed_s(i-1)`
- strict default policy requires `dtime_s > 0` for all intervals
- `intsty(i-1) = 0` when `drain_m == 0`, else `drain_m / dtime_s`
4. Event summary:
- `stmdur_s = sum(dtime_s)`
- `mxint_m_s = max(intsty(0..nbrkpt-2))`
- terminal `intsty(nbrkpt-1) = 0`
- `prcp_m = pptcum_m(nbrkpt-1)`

## Runtime Symbol Projection
Breakpoint branch now exports:
- `stmstr`, `prcp`, `stmdur`, `mxint`, `nbrkpt`, `timem_####`, `intsty_####`, and met fields.
- Watershed projection exports equivalent `hs{ID}_...` prefixed symbols.

## Policy Alignment
- Parser strict breakpoint cardinality policy is now `nbrkpt <= 1500`.
- Runtime seam continues hard-fail behavior on non-monotone breakpoint times and negative drains.

## Compatibility Controls
- `allow_breakpoint_cardinality_override` (parser compatibility mode): allows `nbrkpt > 1500` for controlled investigations.
- `allow_legacy_zero_drain_non_positive_dtime` (parser compatibility mode): disabled by default; explicitly opt-in for legacy zero-drain timing acceptance.
- Strict mode remains default, with no silent fallback behavior.
