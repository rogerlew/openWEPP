# Terminal bounded evidence V14 literal decision and custody authority

Status: `REVIEW CANDIDATE / SOURCE EDITS FORBIDDEN`.

Base/origin: `521986ae8467239268c9635f2b1e05fe06f0dc1d`. V3--V13 are frozen.
The exact source write set is the accepted V13 ten files plus
`src/v11_covered/carrier_phase.rs`, eleven Rust files total.

Findings incorporated: `CHILD1-TERM-EVIDENCE-016` in the companion census;
`CHILD1-TERM-EVIDENCE-017`, V13 conflated the live acceptance vector with
prospective effectivity operands; `CHILD1-TERM-EVIDENCE-018`, capture laziness
lacked a literal type interface; `CHILD1-TERM-EVIDENCE-019`, provider/floor
finalization lacked immutable exactly-once custody.

## Literal test-only DTO surface

All definitions below are `#[cfg(test)]`, `pub(crate)` only where cross-owner
access requires it, and derive `Clone, Debug, PartialEq`; integer/tag-only
types additionally derive `Eq`. `DiagnosticF64V14 { bits: u64 }` is constructed
only from finite `f64` and compares exact bits.

```rust
struct SupportEvidenceV14 { start_ns: u128, end_ns: u128 }
struct TerminalStateV14 { ice_bits: u64, liquid_bits: u64, cold_content_bits: u64 }
struct TerminalLedgerV14 {
    complete_energy_bits: u64, cold_energy_change_bits: u64,
    refrozen_bits: u64, deposition_bits: u64, sublimation_bits: u64,
    melt_bits: u64, unallocated_energy_bits: u64, shortwave_bits: u64,
    longwave_bits: u64, sensible_bits: u64, latent_bits: u64,
    advected_bits: u64, snow_soil_heat_bits: u64, external_liquid_bits: u64,
}
enum LiveDecisionComponentV14 { IceMass, LiquidMass, ColdContent, CompleteEnergy, UnallocatedEnergy }
struct LivePairComponentV14 {
    component: LiveDecisionComponentV14, coarse_bits: u64, refined_bits: u64,
    delta_bits: u64, denominator_bits: u64, scaled_bits: u64,
}
struct LivePairDecisionV14 {
    components: [LivePairComponentV14; 5], maximum_scaled: DiagnosticF64V14,
    first_bitwise_equal_winner: LiveDecisionComponentV14, rejected: bool,
}
struct CouplingIterationV14 {
    iteration: u32, beginning_joint_sha256: Digest32,
    carrier_ending_joint_sha256: Digest32,
    hydrology_complete_ending_joint_sha256: Digest32,
}
struct CouplingSelectionV14 { iterations: Vec<CouplingIterationV14>, selected_iteration: u32 }
enum PairPositionV14 { Coarse, Fine1, Fine2 }
struct SelectedTrialV14 {
    position: PairPositionV14, role: CoveredTerminalTrialRoleV1,
    attempt_ordinal: u32, support: SupportEvidenceV14,
    beginning_state: TerminalStateV14, ending_state: TerminalStateV14,
    beginning_joint_sha256: Digest32, carrier_ending_joint_sha256: Digest32,
    hydrology_complete_ending_joint_sha256: Digest32,
    ledger: TerminalLedgerV14, coupling: CouplingSelectionV14,
    carrier: CarrierPhaseProjectionV14,
}
struct RejectedPairV14 {
    epoch_ordinal: u32, pair_ordinal: u32,
    trials: [SelectedTrialV14; 3], decision: LivePairDecisionV14,
}
struct FloorAdmissionDraftV14 {
    proposed_duration: DiagnosticF64V14,
    required_half_duration: DiagnosticF64V14,
    minimum_duration: DiagnosticF64V14,
    outcome: TerminalPhysicalResultV14,
}
struct ProviderEpochFloorV14 {
    epoch_ordinal: u32, calls: Vec<ProviderObservationV14>,
    rejected_pairs: Vec<RejectedPairV14>, final_rejected_pair_ordinal: u32,
    call_count_through_final_pair: u64, call_count_at_floor: u64,
    floor: FloorAdmissionDraftV14,
}
```

The live component formula is exact for each ordered source pair `(coarse,
refined)`: `delta = refined - coarse`; `denominator = abs_tol + 1e-8 *
max(abs(coarse), abs(refined))`; `scaled = abs(delta) / denominator`. Mass
absolute tolerance is `1e-9`; energy absolute tolerance is `1e-6`. The maximum
is the exact left-associated `.max()` fold in enum order. Winner is the first
ordered component whose scaled bits equal maximum bits. No ledger component is
substituted for this live decision.

The refined ledger is reconstructed by the exact field order above using one
binary64 `half1 + half2` operation per field. Full physical/effectivity evidence
is the entire three states/ledgers/joints/carrier projections; V14 defines no
prospective reduced effectivity vector.

## Lazy type-level projection

The existing sealed trait is replaced literally by the following relevant
surface (pair/selection/admission methods use the DTOs above):

```rust
trait TerminalEvidenceMode<J>: terminal_evidence_sealed::Sealed {
    type State; type ProviderState; type ProviderProjection;
    fn new_state() -> Self::State;
    fn new_provider_state() -> Self::ProviderState;
    fn project_provider_success(
        request: &CoveredTerminalTrialRequestV1,
        result: &crate::v11_covered::CoveredCarrierPhaseResultV1,
    ) -> Self::ProviderProjection;
    fn record_provider_success(
        state: &mut Self::ProviderState,
        request: &CoveredTerminalTrialRequestV1,
        projection: Self::ProviderProjection,
    );
    fn record_provider_failure(state: &mut Self::ProviderState,
        request: &CoveredTerminalTrialRequestV1);
    fn record_pair(state: &mut Self::State, pair: RejectedPairV14);
    fn record_floor(state: &mut Self::State, floor: FloorAdmissionDraftV14);
    fn finish_provider_epoch(state: &mut Self::State,
        provider: Self::ProviderState);
}
```

`NoEvidence` assigns all three associated types to `()`; every method is an
empty direct function, and its projection method accepts references but does
not inspect either. `CaptureEvidence::ProviderProjection =
CarrierPhaseProjectionV14`; only that implementation invokes
`result.capture_projection_v14(request)`. That method is `#[cfg(test)]` and
owner-local in `carrier_phase.rs`. The generic caller invokes
`M::project_provider_success`; it performs no scan before this static dispatch.
There is no callback, runtime flag, feature, environment input, global,
thread-local, `catch_unwind`, public API or user-supplied closure.

## Immutable epoch/floor custody

Capture state holds `epoch_ordinal`, `pairs: Vec<RejectedPairV14>` and
`floor: Option<FloorAdmissionDraftV14>`. Provider state holds only ordered
provider calls. Pair/floor hooks append or install new values and never mutate
old records. After physical return, `finish_provider_epoch` consumes provider
state with `mem::take` of the current pairs and floor draft and appends one new
`ProviderEpochFloorV14`. Its constructor is infallible: missing/duplicate/faulty
evidence is retained and rejected only by the post-return validator. The record
sets both counts from the consumed call length and the captured final-pair call
boundary; no provider call is possible after the solver returned the floor.
Validation requires one final `REJECT_RETRY`, one following
`BelowCarrierDomain` draft, equal counts, proposed duration equal to the final
pair's next duration, half equal to proposed/2, and half below 0.6 seconds.

## Owner-local carrier projection

```rust
struct ReceiptIdentityV14 { collection_sha256: Digest32, ordered_count: u32 }
struct ProviderObservationV14 {
    provider_ordinal: u64, support: SupportEvidenceV14, lane_id: u32,
    live_role: CoveredTerminalTrialRoleV1, attempt_ordinal: u32,
    coupling_iteration: u32, beginning_joint_sha256: Digest32,
    carrier_ending_joint_sha256: Digest32,
    projection: CarrierPhaseProjectionV14,
    zero_terminal_ingress: [ZeroTerminalIngressV14; 3],
}
struct CarrierPhaseProjectionV14 {
    probe_child_sha256: Digest32, beginning_joint_sha256: Digest32,
    carrier_ending_joint_sha256: Digest32,
    precipitation_sets: ReceiptIdentityV14,
    lower_boundaries: ReceiptIdentityV14,
    carrier_source_receipts: ReceiptIdentityV14,
    covered_lse_states: ReceiptIdentityV14,
    soil_candidate_sha256: Digest32, soil_top_boundary_credit_sha256: Digest32,
    snow_soil_trial_receipt_sha256: Option<Digest32>,
    wb14_child_receipt_set_sha256: String,
    wb14_child_replay_sha256: Digest32,
}
```

Every `CoveredCarrierPhaseResultV1` field is dispositioned: `transition` maps
probe/joints/snow-soil receipt; `ending_candidates` is excluded because its
ending joint is already bound and mutable shadow/stage maps are not claims;
`precipitation_sets`, `complete_lower_boundaries`, `carrier_source_receipts`,
and `covered_lse_states` map to canonical ordered count+digest identities;
`carrier_envelope` is excluded except the three exact ingress searches below;
`soil_candidate` and `soil_top_boundary_credit` map to their canonical
owner-byte digests; child WB14 string and replay bytes map exactly; parent WB14
string/replay are excluded because probes cannot publish parent receipts.
No whole live result is cloned or serialized.

## Exact ingress and noninterference

```rust
enum TerminalIngressSurfaceV14 { HydrologySupply, Wb14Authorization, SurfaceLiquidIngress }
struct ZeroTerminalIngressV14 {
    surface: TerminalIngressSurfaceV14,
    searched_terminal_core_sha256: Digest32,
    searched_collection_sha256: Digest32,
    matching_count: u32, matching_mass_bits: u64,
}
enum TerminalPhysicalResultV14 { BelowCarrierDomain, UnexpectedError, UnexpectedSuccess }
struct ParentInvocationSnapshotV14 {
    parent: V11ParentTransaction,
    consumer: DirectV10RealConsumerShadow,
    clock: CoupledClockStateV1,
    stage3_by_lane: BTreeMap<u32, DirectSnowStage3PersistentState>,
    pending_terminal_parcels: BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
}
struct RejectedPrefixEvidenceV14 {
    before: ParentInvocationSnapshotV14, after: ParentInvocationSnapshotV14,
    physical: TerminalPhysicalResultV14, epoch: ProviderEpochFloorV14,
}
```

The owner-local searches are: hydrology `surface_ingress().receipts()` terminal
supply; WB14 the same ordered receipts filtered to `TerminalReceiver`
authorization/credit; surface-liquid `open_ingress_parcels()` filtered to
`TerminalReceiver`. Each uses the probe-child receipt as terminal-core identity
and canonical ordered collection digest. Valid records occur exactly in the
array order above with zero count and positive-zero mass bits. Isolated positive
controls inject one test-only projected matching item into only the named
witness; ordinary nonterminal precipitation remains nonmatching.

NoEvidence and CaptureEvidence run from independent clones of one exact typed
snapshot. The closed error projection matches only the nested
`Stage3(TerminalNumerics(BelowCarrierDomain))`; all other errors and success are
distinct tags. Validation requires equal result tags and exact `PartialEq`
before/after caller values, no staged candidate, accepted event, pending parcel
or publication/output mutation.

## Literal poison operations

Starting from `valid_rejected_prefix_v14()`, each named constructor clones once
and changes only: `omit_coarse` removes trial index 0; `duplicate_half1`
replaces index 2 by index 1; `swap_halves` swaps indices 1/2; `retry_as_half1`
changes trial 1 role; `discarded_iteration` increments selected iteration;
`wrong_beginning_joint` flips one bit of trial 0 beginning digest;
`break_half_joint` flips one bit of trial 2 beginning digest;
`change_component_bit` flips the low bit of component 0 refined bits;
`change_maximum` flips maximum low bit; `change_winner` selects the next enum;
`change_floor_duration` flips proposed-duration low bit;
`change_ingress_digest` flips hydrology collection digest low bit;
`change_ingress_count` sets WB14 count to one; `remove_provider` removes the
selected provider observation; `reorder_providers` swaps its two adjacent
coupling observations. Each has one test named
`v14_rejects_<constructor>` and one validator predicate expectation; all other
fields are asserted equal to the valid fixture.

Two fresh GO reviews authorize only this eleven-file diagnostic expansion.
Either HOLD stops before source edits. Matrix and final reviews remain barred.
