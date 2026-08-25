# Terminal bounded observation seam V6 literal Rust corrections

Status: `CANDIDATE / NO SOURCE AUTHORITY`

```rust
// CarrierHook.result final exact path:
pub result: &'a crate::v9_real_consumer_shadow::CoveredCarrierPhaseResultV1,

// TerminalEvidenceMode additions:
fn before_clock(_: &mut Self::State, _: &openwepp_coupled_time::CoupledClockStateV1) {}
fn after_clock(_: &mut Self::State, _: &openwepp_coupled_time::CoupledClockStateV1) {}
```

`NoEvidence` overrides both additions as empty `#[inline(always)]` methods.
`CaptureEvidence::before_clock` sets `state.before_clock = Some(value.clone())`;
`after_clock` sets `state.after_clock = Some(value.clone())`.

The generic core owns `let mut evidence = M::new_state()`, calls
`M::before_clock(&mut evidence, beginning_clock)` immediately before the
physical call, calls `M::after_clock(&mut evidence, beginning_clock)`
immediately after the physical `Result` is obtained, and returns
`(physical_result, evidence)`. `beginning_clock` is the exact existing immutable
borrow; no `physical_clock` binding is claimed and no mutation is possible
through this path. The cfg(test) caller receives `CaptureState`, performs live
Eq and serialization after tuple return. Generic code accesses state only
through trait methods.

No other inherited declaration or validator changes.
