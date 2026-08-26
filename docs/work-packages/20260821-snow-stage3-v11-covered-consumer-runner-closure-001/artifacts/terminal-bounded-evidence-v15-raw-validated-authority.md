# Terminal bounded evidence V15 raw-to-validated custody authority

Status: `REVIEW CANDIDATE / SOURCE EDITS FORBIDDEN`.

Base/origin: `22dd588ab0519e96de22e4cc90ee4d2990923663`.
V3--V14 remain frozen. Last qualified physical implementation is
`43cc9bbea2fbf5fe6ab6596cee4162de75cef999`; `BelowCarrierDomain` remains
authoritative.

The exact source write set remains the V14 eleven files: the ten V12 Rust files
listed by the V14 census plus `src/v11_covered/carrier_phase.rs`. No public API,
workspace Cargo, temporal operator, Batch V2, event acceptance, receiver,
restart, runner or cutover change is authorized.

## Primitive and complete physical records

All V15 evidence types are `#[cfg(test)]`. `DiagnosticF64V15 { bits: u64 }`
accepts only finite values and retains exact binary64 bits.
`SupportEvidenceV15 { start_ns: u128, end_ns: u128 }` and
`DestinationKeyV15 { ofe_id: OfeId, tile_id: TileId }` use native typed keys.

```rust
struct TerminalStateV15 { ice_bits: u64, liquid_bits: u64, cold_content_bits: u64 }
struct TerminalLedgerV15 {
    complete_energy_bits: u64, cold_energy_change_bits: u64,
    refrozen_bits: u64, deposition_bits: u64, sublimation_bits: u64,
    melt_bits: u64, unallocated_energy_bits: u64, shortwave_bits: u64,
    longwave_bits: u64, sensible_bits: u64, latent_bits: u64,
    advected_bits: u64, snow_soil_heat_bits: u64, external_liquid_bits: u64,
}
enum PairPositionV15 { Coarse, Fine1, Fine2 }
enum LiveDecisionComponentV15 { IceMass, LiquidMass, ColdContent, CompleteEnergy, UnallocatedEnergy }
struct LivePairComponentDraftV15 {
    component: LiveDecisionComponentV15, coarse_bits: u64, refined_bits: u64,
    delta_bits: u64, denominator_bits: u64, scaled_bits: u64,
}
struct LivePairDecisionDraftV15 {
    components: Vec<LivePairComponentDraftV15>, maximum_scaled_bits: u64,
    winner: LiveDecisionComponentV15, rejected: bool,
}
struct LivePairDecisionV15 {
    components: [LivePairComponentDraftV15; 5], maximum_scaled_bits: u64,
    winner: LiveDecisionComponentV15, rejected: bool,
}
```

The validated decision requires component order exactly ice, liquid, cold,
complete energy, unallocated energy. For each: `delta = refined - coarse`;
`denominator = abs_tol + 1e-8 * max(abs(coarse), abs(refined))`; `scaled =
abs(delta)/denominator`; mass `abs_tol=1e-9`, energy `abs_tol=1e-6`. Maximum is
the live left-associated `.max()` fold and winner is the first ordered matching
scaled bit pattern. Ledger/effectivity operands never replace this array.

## Provider and coupling correlation

```rust
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ProviderCallKeyV15 {
    epoch_ordinal: u32, support: SupportEvidenceV15, lane_id: u32,
    role: CoveredTerminalTrialRoleV1, attempt_ordinal: u32,
    coupling_iteration: u32, beginning_joint_sha256: Digest32,
}
enum ProviderOutcomeV15 { Success, Failure }
struct ProviderObservationV15 {
    provider_ordinal: u64, key: ProviderCallKeyV15,
    outcome: ProviderOutcomeV15,
    projection: Option<CarrierPhaseProjectionV15>,
}
struct EndingSnowHintV15 {
    ice_bits: u64, liquid_bits: u64, cold_content_bits: u64,
    surface_temperature_bits: u64,
}
struct CouplingComponentV15 {
    previous_bits: u64, next_bits: u64, absolute_difference_bits: u64,
    tolerance_bits: u64, within: bool,
}
struct CouplingIterationDraftV15 {
    key: ProviderCallKeyV15, incoming_hint: Option<EndingSnowHintV15>,
    outgoing_hint: EndingSnowHintV15,
    components: [CouplingComponentV15; 4], live_converged: bool,
}
enum CouplingSelectionReasonV15 { FourComponentConvergenceBreak, IterationLoopExhausted }
struct CouplingSelectionDraftV15 {
    ordered_call_keys: Vec<ProviderCallKeyV15>,
    selected_call_key: ProviderCallKeyV15, selected_iteration: u32,
    reason: CouplingSelectionReasonV15,
    post_loop_three_component_check: bool,
}
struct CouplingIterationV15 {
    draft: CouplingIterationDraftV15, provider_ordinal: u64,
}
struct CouplingSelectionV15 {
    iterations: Vec<CouplingIterationV15>, selected_provider_ordinal: u64,
    selected_iteration: u32, reason: CouplingSelectionReasonV15,
    post_loop_three_component_check: bool,
}
```

The component array order is ice, liquid, cold content, surface temperature;
tolerances are respectively `1e-9`, `1e-9`, `1e-6`, `1e-9`. Every iteration
key joins exactly one successful provider observation; every provider key joins
exactly one iteration. Ordered keys and provider ordinals must be bijective and
strictly increasing. Selected key must be a member and match selected iteration.
Matrix-quality validation additionally requires `FourComponentConvergenceBreak`
and `live_converged=true`. Exhausted selection is faithfully retained but is
inadmissible evidence.

`CHILD1-TERM-COUPLING-020` is characterized by a test-only provider fixture
whose outgoing surface-temperature hint alternates by `2e-9` while the other
three components repeat. It executes all 32 iterations, records
`IterationLoopExhausted`, and observes whether the existing post-loop
three-component check accepts. This test records behavior only; it cannot alter
the loop or physical result. Confirmation opens a separate correction authority.

## Raw and validated pair/epoch custody

```rust
struct SelectedTrialDraftV15 {
    position: PairPositionV15, role: CoveredTerminalTrialRoleV1,
    attempt_ordinal: u32, support: SupportEvidenceV15,
    proposed_next_duration_bits: u64,
    beginning_state: TerminalStateV15, ending_state: TerminalStateV15,
    beginning_joint_sha256: Digest32, carrier_ending_joint_sha256: Digest32,
    hydrology_complete_ending_joint_sha256: Digest32,
    ledger: TerminalLedgerV15, coupling: CouplingSelectionDraftV15,
}
struct SelectedTrialV15 {
    position: PairPositionV15, role: CoveredTerminalTrialRoleV1,
    attempt_ordinal: u32, support: SupportEvidenceV15,
    proposed_next_duration_bits: u64,
    beginning_state: TerminalStateV15, ending_state: TerminalStateV15,
    beginning_joint_sha256: Digest32, carrier_ending_joint_sha256: Digest32,
    hydrology_complete_ending_joint_sha256: Digest32,
    ledger: TerminalLedgerV15, coupling: CouplingSelectionV15,
}
struct RejectedPairDraftV15 {
    epoch_ordinal: u32, pair_ordinal: u32,
    trials: Vec<SelectedTrialDraftV15>, decision: LivePairDecisionDraftV15,
}
struct ValidatedRejectedPairV15 {
    epoch_ordinal: u32, pair_ordinal: u32,
    trials: [SelectedTrialV15; 3], decision: LivePairDecisionV15,
}
struct FloorAdmissionDraftV15 {
    proposed_duration_bits: u64, required_half_duration_bits: u64,
    minimum_duration_bits: u64, outcome: TerminalPhysicalResultV15,
}
struct ProviderEpochDraftV15 {
    epoch_ordinal: u32, calls: Vec<ProviderObservationV15>,
    iterations: Vec<CouplingIterationDraftV15>,
    selections: Vec<CouplingSelectionDraftV15>,
    pairs: Vec<RejectedPairDraftV15>,
    floor_drafts: Vec<FloorAdmissionDraftV15>,
}
struct ValidatedProviderEpochFloorV15 {
    epoch_ordinal: u32, calls: Vec<ProviderObservationV15>,
    pairs: Vec<ValidatedRejectedPairV15>, final_pair_index: usize,
    floor: FloorAdmissionDraftV15, call_count_through_final_pair: u64,
    call_count_at_floor: u64,
}
```

`TryFrom<RejectedPairDraftV15>` requires length three; exactly one Coarse,
Fine1 and Fine2; coarse role Full or Retry; Fine1 Half1; Fine2 Half2; and no
duplicate `(support,attempt,position)` key. It sorts only into fixed position
order after validating that raw order was Coarse/Fine1/Fine2. Half2 beginning
state and joint equal Half1 ending; refined ledger is the exact ordered
binary64 Half1+Half2 field sum.

The infallible physical hooks append raw calls, iterations, selections, pairs
and floor drafts only. `finish_provider_epoch` consumes these vectors into a
new raw epoch without overwriting any entry. Post-return `TryFrom` requires one
floor draft, at least one rejected pair, exactly one final Retry pair, and
validates all joins. `call_count_through_final_pair` is one plus the maximum
provider ordinal across every coupling iteration of all three final-pair
trials; `call_count_at_floor` is calls.len(); equality proves no floor call.
Floor proposed duration equals final pair proposed-next, required half is one
binary64 division by 2, half is below 0.6, and outcome is BelowCarrierDomain.

## Literal lazy interface

```rust
trait TerminalEvidenceMode<J>: terminal_evidence_sealed::Sealed {
    type State; type ProviderState; type ProviderProjection;
    fn new_state() -> Self::State;
    fn new_provider_state() -> Self::ProviderState;
    fn project_provider_success(
        request: &CoveredTerminalTrialRequestV1,
        result: &crate::v9_real_consumer_shadow::CoveredCarrierPhaseResultV1,
    ) -> Self::ProviderProjection;
    fn record_provider_success(state: &mut Self::ProviderState,
        request: &CoveredTerminalTrialRequestV1,
        projection: Self::ProviderProjection);
    fn record_provider_failure(state: &mut Self::ProviderState,
        request: &CoveredTerminalTrialRequestV1);
    fn record_iteration(state: &mut Self::State, value: CouplingIterationDraftV15);
    fn record_selection(state: &mut Self::State, value: CouplingSelectionDraftV15);
    fn record_selected_trial(state: &mut Self::State, value: SelectedTrialDraftV15);
    fn record_pair(state: &mut Self::State, value: RejectedPairDraftV15);
    fn record_floor(state: &mut Self::State, value: FloorAdmissionDraftV15);
    fn finish_provider_epoch(state: &mut Self::State, provider: Self::ProviderState);
}
```

NoEvidence uses `State=()`, `ProviderState=()`, `ProviderProjection=()` and
empty direct bodies. The generic provider calls `M::project_provider_success`
immediately on success, before any receipt access. Only CaptureEvidence calls
the `#[cfg(test)]` owner-local `capture_projection_v15(request)` method in
`carrier_phase.rs`. A source guard rejects `carrier_envelope`, `receipts()` or
`open_ingress_parcels()` between provider result and the static trait call.

## Purpose-built carrier projection and exact 13-field disposition

```rust
struct PrecipitationSetProjectionV15 { lane_id: u32, receipt_sha256: Digest32 }
struct LowerBoundaryProjectionV15 { destination: DestinationKeyV15,
    snow_temperature_bits: u64, snow_soil_heat_flux_bits: u64 }
struct CarrierSourceProjectionV15 { destination: DestinationKeyV15,
    diagnostic_sha256: Digest32 }
struct CoveredLseProjectionV15 { destination: DestinationKeyV15,
    canopy_air_temperature_bits: u64, canopy_air_humidity_bits: u64,
    snow_temperature_bits: u64, snow_sensible_bits: u64, snow_vapor_bits: u64,
    snow_latent_bits: u64, snow_net_longwave_bits: u64 }
struct SoilTopCreditProjectionV15 { lane_id: u32, ofe_id: OfeId,
    first_layer_id: SoilLayerId, beginning_state_sha256: Sha256Digest,
    support_start_ns: i64, support_end_ns: i64,
    accepted_positive_downward_bits: u64, credit_bits: u64,
    snow_soil_receipt_sha256: Sha256Digest }
struct CarrierPhaseProjectionV15 {
    beginning_joint_sha256: Digest32, carrier_ending_joint_sha256: Digest32,
    probe_child_sha256: Digest32,
    snow_soil_trial_receipt_sha256: Option<Sha256Digest>,
    precipitation_sets: Vec<PrecipitationSetProjectionV15>,
    lower_boundaries: Vec<LowerBoundaryProjectionV15>,
    carrier_sources: Vec<CarrierSourceProjectionV15>,
    covered_lse: Vec<CoveredLseProjectionV15>,
    soil_top_credit: SoilTopCreditProjectionV15,
    wb14_child_receipt_set_sha256: String,
    wb14_child_replay_sha256: Digest32,
    zero_terminal_ingress: [ZeroTerminalIngressV15; 3],
}
```

Disposition of live result fields: transition -> joints/probe/snow-soil;
ending_candidates -> ending joint only, with shadow/stage maps excluded as
non-claim mutable candidates; precipitation_sets -> existing set receipts;
carrier_envelope -> only WB14/surface witness accessors; lower_boundaries ->
typed destination and exact temperature/heat primitives; carrier_source_receipts
-> existing diagnostic_sha256; covered_lse_states -> named primitive subset;
soil_candidate -> excluded because candidate storage is not used by the V15
claims and no canonical digest exists; soil_top_boundary_credit -> every field
listed above except redundant owner/configuration identity, which is bound by
beginning joint and probe; WB14 child string and replay -> existing string plus
`digest_bytes(replay)`; parent WB14 string/replay -> excluded because probe
cannot publish a parent receipt. Collections preserve BTreeMap key order or
source Vec order; no invented collection digest or whole-result serialization.

## Independent ingress and observable noninterference

```rust
enum TerminalIngressSurfaceV15 { HydrologySupply, Wb14Authorization, SurfaceLiquidIngress }
enum IngressCollectionIdentityV15 {
    PrecipitationSetReceipts(Vec<Digest32>),
    Wb14Child { receipt_set_sha256: String, replay_sha256: Digest32 },
    SurfaceInput { ordered_parcel_receipt_sha256: Vec<Digest32> },
}
struct ZeroTerminalIngressV15 {
    surface: TerminalIngressSurfaceV15, searched_terminal_identity: Digest32,
    source_collection_identity: IngressCollectionIdentityV15,
    matching_count: u32, matching_mass_bits: u64,
}
enum TerminalPhysicalResultV15 { BelowCarrierDomain, UnexpectedError, UnexpectedSuccess }
struct CallerInvocationSnapshotV15 {
    parent: V11ParentTransaction, consumer: DirectV10RealConsumerShadow,
    clock: CoupledClockStateV1,
    retained_stage3_by_lane: BTreeMap<u32, DirectSnowStage3PersistentState>,
    retained_terminal_parcels: BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
}
```

Hydrology supply uses selected trial `ledger.external_liquid` and the exact
Stage3 precipitation set receipts, matching only parcels whose existing source
identity equals the searched terminal probe/core. WB14 authorization parses the
ordered child replay through its existing validator and matches only
TerminalReceiver kind/source. Surface input searches pre-WB14
`surface_ingress().open_ingress_parcels()` for TerminalReceiver. Each isolated
test-only projection poison adds one match only to its named source; the other
two DTOs must remain byte-for-byte equal. Ordinary nonterminal precipitation
is a positive nonmatch control.

NoEvidence and CaptureEvidence run independently from clones of one caller
snapshot; they require the same closed result tag and exact caller-owned values.
Internal dropped locals are not claimed observable. Source/control-flow guards
prove no installation before Err and NoEvidence never references the owner
projection; production and test configurations must compile.

## Raw one-field poisons

Each constructor starts from `valid_epoch_draft_v15`, clones once and mutates
only the named raw field: remove coarse; duplicate Half1; swap halves; change
Fine1 role to Retry; select a nonconverged iteration; change selection reason
to Exhausted; remove a provider call; duplicate a provider key; flip beginning
joint; break Half2/half1 joint; flip one decision refined bit; flip maximum;
change winner; remove floor; duplicate floor; flip floor proposed duration;
flip hydrology match count; flip WB14 replay identity; flip surface parcel
identity; reorder provider calls. Each has a named `v15_rejects_*` validator
test and an all-other-fields-equal assertion. Validated fixed arrays are never
mutated to manufacture malformed cardinality.

Two independent GO reviews authorize only this diagnostic evidence expansion.
Either HOLD stops before source edits. The estimator matrix remains downstream.
