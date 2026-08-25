# Terminal bounded observation-seam exact private Rust intent

Status: `FROZEN CANDIDATE / NO SOURCE IMPLEMENTATION AUTHORITY`

All types below are crate-private. Rich DTOs and `CaptureEvidence` are compiled
only under `cfg(test)`. Field order shown is declaration order.

## Sealed mode and return boundary

```rust
mod terminal_evidence_private {
    pub trait Sealed {}
}

pub(crate) trait TerminalEvidenceMode: terminal_evidence_private::Sealed {
    type State: Default;
    fn carrier(state: &mut Self::State, value: TerminalCarrierHook<'_>);
    fn iteration(state: &mut Self::State, value: CouplingIterationHook<'_>);
    fn selection(state: &mut Self::State, value: CouplingSelectionHook<'_>);
    fn selected_trial(state: &mut Self::State, value: SelectedTrialHook<'_>);
    fn pair(state: &mut Self::State, value: PairHook<'_>);
    fn admission(state: &mut Self::State, value: AdmissionHook);
}

pub(crate) enum NoEvidence {}
impl terminal_evidence_private::Sealed for NoEvidence {}
impl TerminalEvidenceMode for NoEvidence {
    type State = ();
    #[inline(always)] fn carrier(_: &mut (), _: TerminalCarrierHook<'_>) {}
    #[inline(always)] fn iteration(_: &mut (), _: CouplingIterationHook<'_>) {}
    #[inline(always)] fn selection(_: &mut (), _: CouplingSelectionHook<'_>) {}
    #[inline(always)] fn selected_trial(_: &mut (), _: SelectedTrialHook<'_>) {}
    #[inline(always)] fn pair(_: &mut (), _: PairHook<'_>) {}
    #[inline(always)] fn admission(_: &mut (), _: AdmissionHook) {}
}

#[cfg(test)] pub(crate) enum CaptureEvidence {}
#[cfg(test)] impl terminal_evidence_private::Sealed for CaptureEvidence {}
#[cfg(test)] impl TerminalEvidenceMode for CaptureEvidence {
    type State = RejectedPrefixEvidence;
    // Each hook performs only Clone/Copy and Vec::push/BTreeMap insertion.
    // It returns (), does no hashing/validation/I/O and cannot replace the
    // physical Result. OOM retains ordinary Rust abort behavior in tests.
}
```

Every currently callable wrapper retains its exact signature and calls a new
private generic core with `NoEvidence`. The generic core returns
`(Result<T, DirectSnowStage3EvaluationError>, M::State)`. The wrapper returns
only `.0`. A `cfg(test)` caller invokes the same core with `CaptureEvidence` and
retains both tuple members. This is the only way evidence survives the expected
`BelowCarrierDomain` error. Validation, digesting, serialization, assertions
and file I/O occur after the tuple has returned to the unit test.

There is no callback supplied by a caller, runtime flag, Cargo feature,
environment read, global, thread-local, mutex, channel, panic interception or
`catch_unwind`. `NoEvidence::State` is `()`, constructs no collection and is
allocation-free. All hooks inline to empty functions in the production
monomorphization.

## Exact DTO declarations

```rust
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct DiagnosticF64 { pub bits: u64, pub semantic_finite: bool }

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SupportEvidence { pub start_ns: u128, pub end_ns: u128 }

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)] pub(crate) enum PairPosition { Coarse=0, Fine1=1, Fine2=2 }

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)] pub(crate) enum LiveProviderRole {
    Full=0, Half1=1, Half2=2, Retry=3, BracketLower=4, BracketUpper=5, Root=6
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CarrierPhaseKey {
    pub prefix: Digest32, pub support: SupportEvidence,
    pub role: LiveProviderRole, pub attempt: u32, pub coupling: u32,
    pub beginning_joint: Digest32, pub carrier_ending_joint: Digest32,
    pub provider_call: u64, pub arena_index: u64,
}

#[cfg(test)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CarrierPhaseEvidence {
    pub key: CarrierPhaseKey,
    pub request: CoveredTerminalTrialRequestV1,
    pub child: CoveredProbeChildIdentityV1,
    pub ending_joint: Digest32,
    pub trial_snow_soil_receipt: Option<TerminalSnowSoilTrialReceiptV1>,
    pub precipitation_receipts: BTreeMap<u32, Digest32>,
    pub carrier_envelope_transaction: TransactionId,
    pub lower_boundary_destinations: Vec<(OfeId, TileId)>,
    pub carrier_source_destinations: Vec<(OfeId, TileId)>,
    pub covered_lse_destinations: Vec<(OfeId, TileId)>,
    pub soil_candidate_configuration: String,
    pub soil_top_boundary_credit: SoilThermalTopBoundaryCreditV1,
    pub wb14_child_receipt_set_sha256: String,
    pub wb14_child_replay_bytes: Vec<u8>,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct TerminalStateEvidence {
    pub ice_kg_m2: DiagnosticF64, pub liquid_kg_m2: DiagnosticF64,
    pub cold_content_j_m2: DiagnosticF64,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct TerminalLedgerEvidence {
    pub complete_energy_j_m2: DiagnosticF64,
    pub cold_energy_change_j_m2: DiagnosticF64,
    pub refrozen_kg_m2: DiagnosticF64, pub deposition_kg_m2: DiagnosticF64,
    pub sublimation_kg_m2: DiagnosticF64, pub melt_kg_m2: DiagnosticF64,
    pub unallocated_energy_j_m2: DiagnosticF64,
    pub shortwave_energy_j_m2: DiagnosticF64,
    pub longwave_energy_j_m2: DiagnosticF64,
    pub sensible_energy_j_m2: DiagnosticF64,
    pub latent_energy_j_m2: DiagnosticF64,
    pub advected_energy_j_m2: DiagnosticF64,
    pub snow_soil_heat_energy_j_m2: DiagnosticF64,
    pub external_liquid_kg_m2: DiagnosticF64,
}

#[cfg(test)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CouplingIterationEvidence {
    pub key: CarrierPhaseKey, pub flux: TerminalFluxIntegral,
    pub preview: TerminalStateEvidence,
    pub incoming_hint: Option<CoveredTerminalEndingSnowHintV1>,
    pub outgoing_hint: CoveredTerminalEndingSnowHintV1,
    pub component_within: [bool; 4], pub combined_converged: bool,
}

#[cfg(test)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CouplingSelectionEvidence {
    pub prefix: Digest32, pub support: SupportEvidence,
    pub role: LiveProviderRole, pub attempt: u32,
    pub ordered_iteration_keys: Vec<CarrierPhaseKey>,
    pub selected_iteration_key: CarrierPhaseKey,
    pub selected_carrier_key: CarrierPhaseKey,
    pub selected_coupling: u32, pub selected_converged: bool,
}

#[cfg(test)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SelectedTerminalTrialEvidence {
    pub prefix: Digest32, pub position: PairPosition,
    pub role: LiveProviderRole, pub attempt: u32, pub support: SupportEvidence,
    pub beginning: TerminalStateEvidence, pub ending: TerminalStateEvidence,
    pub ledger: TerminalLedgerEvidence,
    pub selection: CouplingSelectionEvidence,
    pub hydrology_complete_ending_joint: CoveredTerminalJointTrialStateV1,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)] pub(crate) enum PairComponent { Ice=0, Liquid=1, Cold=2,
    CompleteEnergy=3, UnallocatedEnergy=4 }

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PairComponentErrorEvidence {
    pub component: PairComponent, pub coarse: DiagnosticF64,
    pub refined: DiagnosticF64, pub delta: DiagnosticF64,
    pub abs_tol: DiagnosticF64, pub rel_tol: DiagnosticF64,
    pub denominator: DiagnosticF64, pub scaled: DiagnosticF64,
}

#[cfg(test)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PairDecisionEvidence {
    pub prefix: Digest32, pub pair_ordinal: u32,
    pub coarse: SelectedTerminalTrialEvidence,
    pub fine_1: SelectedTerminalTrialEvidence,
    pub fine_2: SelectedTerminalTrialEvidence,
    pub components: [PairComponentErrorEvidence; 5],
    pub maximum_scaled: DiagnosticF64, pub diagnostic_winner: PairComponent,
    pub decision: PairDecision, pub current_duration: DiagnosticF64,
    pub proposed_next: Option<DiagnosticF64>,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)] pub(crate) enum AdmissionDecision {
    Admit=0, BelowCarrierDomain=1, DomainOrNonFinite=2
}

#[cfg(test)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TrialAdmissionEvidence {
    pub prefix: Digest32, pub ordinal: u32, pub proposed: SupportEvidence,
    pub proposed_duration: DiagnosticF64,
    pub required_half_duration: DiagnosticF64,
    pub minimum_carrier_duration: DiagnosticF64,
    pub decision: AdmissionDecision,
    pub outcome: Option<SnowTerminalNumericsFailure>,
    pub provider_calls_before: u64, pub provider_calls_after: u64,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)] pub(crate) enum ZeroIngressSource {
    HydrologyTerminalLiquidSupply=0, Wb14TerminalLiquidCredit=1,
    SurfaceLiquidTerminalIngress=2
}

#[cfg(test)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ZeroIngressEvidence {
    pub source: ZeroIngressSource, pub searched_carrier_keys: Vec<CarrierPhaseKey>,
    pub observed: Vec<DiagnosticF64>,
}

#[cfg(test)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct NoninterferenceSnapshot {
    pub owner_bytes: BTreeMap<String, Vec<u8>>,
    pub joint: CoveredTerminalJointTrialStateV1,
    pub clock: CoupledClockStateV1,
    pub provider_calls: u64,
    pub candidates_by_joint: BTreeMap<Digest32, CoveredCarrierEphemeralCandidatesV1>,
    pub carrier_phase_keys: Vec<CarrierPhaseKey>,
    pub pending_terminal_parcels: BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    pub stage3_by_lane: BTreeMap<u32, DirectSnowStage3PersistentState>,
}

#[cfg(test)]
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct RejectedPrefixEvidence {
    pub carriers: Vec<CarrierPhaseEvidence>,
    pub iterations: Vec<CouplingIterationEvidence>,
    pub selections: Vec<CouplingSelectionEvidence>,
    pub selected_trials: Vec<SelectedTerminalTrialEvidence>,
    pub pairs: Vec<PairDecisionEvidence>,
    pub admissions: Vec<TrialAdmissionEvidence>,
    pub zero_ingress: [ZeroIngressEvidence; 3],
    pub before: Option<NoninterferenceSnapshot>,
    pub after: Option<NoninterferenceSnapshot>,
}
```

`PairDecision` is private `Accept | RejectRetry`. `DiagnosticF64::from(f64)`
sets both fields from the same value. No DTO is serialized during capture.

## Exact hook locations and chronology

1. `snow_stage3_v11_terminal_execution.rs::evaluate_covered_terminal_candidate_v1`
   creates state and `before`, increments `provider_calls` immediately before
   each provider invocation, and calls `M::carrier` only after
   `execute_covered_carrier_phase_v1` returned `Ok` and before moving its values.
2. `evaluation.rs` calls `M::iteration` after a physical carrier transition,
   flux, preview and convergence tuple all exist; `M::selection` occurs once
   immediately before returning the selected tuple. Ordered iteration keys are
   the actual loop push order; the selected key is its last converged member.
3. `terminal_event.rs` calls `M::selected_trial` only after
   `join_hydrology_ending` returns the complete joint. It records COARSE for the
   full/retry result and FINE_1/FINE_2 separately before constructing refined.
4. The five component errors are constructed from the exact local `full` and
   `refined` values immediately before the existing `error > 1.0` branch.
   `M::pair` is called once with `RejectRetry` immediately before `continue`,
   or `Accept` immediately before adoption/event localization.
5. The `dt < 2.0 * MINIMUM_COVERED_CARRIER_SECONDS` branch calls
   `M::admission` before returning `BelowCarrierDomain`, with
   `proposed_duration=dt`, `required_half_duration=0.5*dt`, minimum `0.6`, and
   provider counts sampled on both sides without a provider call.
6. The outer candidate function always captures `after` after its physical
   `Result` is retained. The `cfg(test)` caller then constructs the three zero
   searches over captured carrier keys: hydrology supply from the provider
   transition's terminal-supply input slot, WB14 credit from the carrier
   envelope surface-ingress terminal-credit slot, and surface-liquid ingress
   from the corresponding ingress parcel class. Each search retains every
   observed amount, not only matches.

The v11 owner projection names these three exact accessors in the implementation
diff before use. If any named slot/accessor does not already exist in the seven-
file write set, review returns HOLD; it may not synthesize zero from absence.
