# Continuous-Daily Runtime Parity Evidence

Evidence mode: `Ran`
Status: `pass`

## Test Evidence
Executed targeted CLIM03 coverage tests:

1. `cargo test -p openwepp-hillslope-orchestrator runtime_inputs -- --nocapture`
- includes:
  - datver policy branches
  - pre-4 rejection
  - wc1 wet-event `timep` floor behavior
  - wc1 `stmdur` cap behavior

2. `cargo test -p openwepp-watershed-orchestrator runtime_inputs -- --nocapture`
- includes per-hillslope assignment variants of same CLIM03 behaviors.

3. `cargo test --test parser_runtime_seam_integration -- --nocapture`
- integration closure through orchestrator execution surfaces.
- wc1 fixture integration checks included.

## Runtime Behavior Checks
- `datver>=4.0` branch applies `ip *= 0.70`.
- `datver=0.0` branch preserves unscaled `ip`.
- `0.0<datver<4.0` rejected typed.
- `stmdur` cap enforced at `23.999 h` pre-conversion.
- non-breakpoint runtime publishes disaggregated `timem/intsty` event-shape symbols.
- watershed per-hillslope projection preserves same semantics with `hs{ID}_` prefix.

## Known Limits
- Breakpoint runtime branch parity remains out-of-scope for CLIM03 and is deferred to CLIM04.
