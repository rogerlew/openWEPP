# Terminal bounded observation seam V5 literal Rust corrections

Status: `CANDIDATE / NO SOURCE AUTHORITY`

The following replacements are normative:

```rust
// CarrierHook.result exact type replacement:
pub result: &'a crate::v11_covered::CoveredCarrierPhaseResultV1,

// PairComponentHook.component exact type replacement:
pub component: HookPairComponent,

// TerminalEvidenceMode addition:
fn provider_call(_: &mut Self::State, _: ProviderCallHook<'_>) {}

#[cfg(test)]
pub(crate) struct CaptureState {
    pub evidence: RejectedPrefixEvidence,
    pub before_clock: Option<openwepp_coupled_time::CoupledClockStateV1>,
    pub after_clock: Option<openwepp_coupled_time::CoupledClockStateV1>,
}
```

`CaptureEvidence::State = CaptureState`. `new_state` explicitly constructs
empty evidence and both clock options as `None`. The owning wrapper sets
`before_clock = Some(beginning_clock.clone())` before physical execution and
`after_clock = Some(physical_clock.clone())` after it. Only CaptureEvidence
executes those assignments; NoEvidence neither clones nor allocates. After the
physical tuple returns, the test takes both values, checks live `Eq`, then
serializes both into the existing `ClockEvidence` DTO.

The complete NoEvidence implementation is literally:

```rust
impl<J> TerminalEvidenceMode<J> for NoEvidence {
    type State = ();
    #[inline(always)] fn new_state() {}
    #[inline(always)] fn carrier(_: &mut (), _: CarrierHook<'_>) {}
    #[inline(always)] fn provider_call(_: &mut (), _: ProviderCallHook<'_>) {}
    #[inline(always)] fn iteration(_: &mut (), _: IterationHook<'_, J>) {}
    #[inline(always)] fn selection(_: &mut (), _: SelectionHook<'_>) {}
    #[inline(always)] fn selected(_: &mut (), _: SelectedHook<'_, J>) {}
    #[inline(always)] fn pair(_: &mut (), _: PairHook<'_, J>) {}
    #[inline(always)] fn admission(_: &mut (), _: AdmissionHook) {}
}
```

No other V3/V4 declaration or validator changes.
