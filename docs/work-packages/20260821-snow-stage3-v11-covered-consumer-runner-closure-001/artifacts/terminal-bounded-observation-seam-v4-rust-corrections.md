# Terminal bounded observation seam V4 normative Rust corrections

Status: `CANDIDATE / NO SOURCE AUTHORITY`

## Production-compiling borrowed surface

The following non-public, non-`cfg(test)` hook operands replace V3 hook enum
references and the inaccessible `TerminalLedger` reference:

```rust
#[derive(Clone, Copy)] pub(crate) enum HookPairPosition { Coarse, Fine1, Fine2 }
#[derive(Clone, Copy)] pub(crate) enum HookPairComponent { Ice, Liquid, Cold, CompleteEnergy, UnallocatedEnergy }
#[derive(Clone, Copy)] pub(crate) enum HookPairDecision { Accept, RejectRetry }
#[derive(Clone, Copy)] pub(crate) enum HookProviderOutcome { ReturnedOk, ReturnedErr }
#[derive(Clone, Copy)] pub(crate) struct TerminalLedgerHook {
 pub complete_energy: f64, pub cold_energy_change: f64, pub refrozen: f64,
 pub deposition: f64, pub sublimation: f64, pub melt: f64,
 pub unallocated_energy: f64, pub shortwave: f64, pub longwave: f64,
 pub sensible: f64, pub latent: f64, pub advected: f64,
 pub snow_soil_heat: f64, pub external_liquid: f64,
}
#[derive(Clone, Copy)] pub(crate) struct TerminalFluxHook {
 pub complete_energy: f64, pub vapor_mass_exchange: f64, pub shortwave: f64,
 pub longwave: f64, pub sensible: f64, pub latent: f64, pub advected: f64,
 pub snow_soil_heat: f64, pub external_liquid: f64,
}
pub(crate) struct ProviderCallHook<'a> {
 pub event_ordinal: u64, pub chronology_ordinal: u64, pub call_ordinal: u64,
 pub request: &'a CoveredTerminalTrialRequestV1, pub outcome: HookProviderOutcome,
}
```

`IterationHook.flux` is `TerminalFluxHook` by value. `SelectedHook.ledger` is
`TerminalLedgerHook` by value. `SelectedHook.position`, `PairHook.winner` and
`PairHook.decision` use the corresponding Hook enums. Capture maps them to the
test-only DTO enums. No visibility of `TerminalLedger` changes.

`TerminalEvidenceMode<J>` adds `fn provider_call(&mut State,
ProviderCallHook<'_>)`. `NoEvidence` explicitly overrides `carrier`,
`provider_call`, `iteration`, `selection`, `selected`, `pair`, and `admission`;
every override is `#[inline(always)]` with an empty body. Its state remains
`()`. Hook creation copies only scalars/enums/references.

## Exact DTO replacements

```rust
#[cfg(test)] #[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TerminalFluxEvidence {
 pub complete_energy: DiagnosticF64, pub vapor_mass_exchange: DiagnosticF64,
 pub shortwave: DiagnosticF64, pub longwave: DiagnosticF64,
 pub sensible: DiagnosticF64, pub latent: DiagnosticF64,
 pub advected: DiagnosticF64, pub snow_soil_heat: DiagnosticF64,
 pub external_liquid: DiagnosticF64,
}
#[cfg(test)] #[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ProviderCallEvidence {
 pub event_ordinal: u64, pub chronology_ordinal: u64, pub call_ordinal: u64,
 pub support: SupportEvidence, pub role: ProviderRole, pub attempt: u32,
 pub outcome: u8,
}
```

`IterationEvidence.flux` is replaced by `TerminalFluxEvidence`.
`PairDecisionEvidence` adds `resulting_joint: Option<JointEvidence>`.
`RejectedPrefixEvidence` adds `provider_calls: Vec<ProviderCallEvidence>`.
Every closure entry emits exactly one provider-call record after its physical
`Result` returns and before `?` propagation; outcome tags are ok `0`, error `1`.
Carrier evidence exists exactly for the ok subset.

## Opaque coupled-clock snapshot

`CoupledClockStateV1` is owned by another crate; its private fields are not
pretended accessible. V3 `ClockEvidence` is replaced by:

```rust
#[cfg(test)] #[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ClockEvidence { pub canonical_json_utf8: Vec<u8> }
```

`CoupledClockStateV1` already derives `Serialize` and `Eq`. CaptureEvidence
clones the before/after clock only in its test state. After the physical result
returns, `serde_json::to_vec` produces the complete named clock location for
the DTO; failure is a test failure and cannot replace the physical result.
The validator both compares the live clones with `Eq` and compares serialized
bytes. This is one explicitly named state location, not a wildcard owner map,
and no inaccessible field conversion is claimed. Seven physical owners,
Stage-3 lanes and pending parcels remain individually projected exactly as V3.
