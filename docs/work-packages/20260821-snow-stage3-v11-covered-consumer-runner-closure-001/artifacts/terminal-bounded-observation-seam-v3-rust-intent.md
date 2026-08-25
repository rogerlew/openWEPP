# Terminal bounded observation seam V3 literal Rust intent

Status: `CANDIDATE / NO SOURCE AUTHORITY`

The declarations below are normative, not pseudocode. All are `pub(crate)` in
the existing private runoff module. DTOs and `CaptureEvidence` are `cfg(test)`.
Production wrappers instantiate `NoEvidence`; only borrowed hooks are built.

```rust
mod terminal_evidence_sealed { pub trait Sealed {} }
pub(crate) trait TerminalEvidenceMode<J>: terminal_evidence_sealed::Sealed {
    type State;
    fn new_state() -> Self::State;
    fn carrier(_: &mut Self::State, _: CarrierHook<'_>) {}
    fn iteration(_: &mut Self::State, _: IterationHook<'_, J>) {}
    fn selection(_: &mut Self::State, _: SelectionHook<'_>) {}
    fn selected(_: &mut Self::State, _: SelectedHook<'_, J>) {}
    fn pair(_: &mut Self::State, _: PairHook<'_, J>) {}
    fn admission(_: &mut Self::State, _: AdmissionHook) {}
}
pub(crate) enum NoEvidence {}
impl terminal_evidence_sealed::Sealed for NoEvidence {}
impl<J> TerminalEvidenceMode<J> for NoEvidence {
    type State = ();
    #[inline(always)] fn new_state() {}
}

pub(crate) struct CarrierHook<'a> {
    pub event_ordinal: u64,
    pub provider_call_ordinal: u64,
    pub arena_ordinal: usize,
    pub request: &'a CoveredTerminalTrialRequestV1,
    pub child: &'a CoveredProbeChildIdentityV1,
    pub result: &'a crate::v11_covered::carrier_phase::CoveredCarrierPhaseResultV1,
}
pub(crate) struct IterationHook<'a, J> {
    pub event_ordinal: u64, pub carrier_ordinal: usize,
    pub iteration_ordinal: usize, pub flux: &'a TerminalFluxIntegral,
    pub preview: &'a TerminalState,
    pub incoming: Option<&'a CoveredTerminalEndingSnowHintV1>,
    pub outgoing: &'a CoveredTerminalEndingSnowHintV1,
    pub component_within: [bool; 4], pub converged: bool,
    pub beginning_joint: &'a J, pub ending_joint: &'a J,
}
pub(crate) struct SelectionHook<'a> {
    pub event_ordinal: u64, pub first_iteration: usize,
    pub iteration_count: usize, pub selected_iteration: usize,
    pub selected_carrier: usize, pub selected_converged: bool,
    pub request: &'a CoveredTerminalTrialRequestV1,
}
pub(crate) struct SelectedHook<'a, J> {
    pub event_ordinal: u64, pub selected_ordinal: usize,
    pub position: PairPosition, pub request: &'a CoveredTerminalTrialRequestV1,
    pub beginning: &'a TerminalState, pub ending: &'a TerminalState,
    pub ledger: &'a TerminalLedger, pub selection_ordinal: usize,
    pub beginning_joint: &'a J, pub ending_joint: &'a J,
}
pub(crate) struct PairHook<'a, J> {
    pub event_ordinal: u64, pub pair_ordinal: usize,
    pub coarse_selected: usize, pub fine_1_selected: usize,
    pub fine_2_selected: usize,
    pub components: &'a [PairComponentHook; 5],
    pub maximum_scaled: f64, pub winner: PairComponent,
    pub decision: PairDecision, pub current_duration_s: f64,
    pub proposed_next_s: Option<f64>, pub resulting_joint: &'a J,
}
#[derive(Clone, Copy)] pub(crate) struct PairComponentHook {
    pub component: PairComponent, pub coarse: f64, pub refined: f64,
    pub delta: f64, pub abs_tol: f64, pub rel_tol: f64,
    pub denominator: f64, pub scaled: f64,
}
#[derive(Clone, Copy)] pub(crate) struct AdmissionHook {
    pub event_ordinal: u64, pub chronology_ordinal: u64,
    pub start_s: f64, pub proposed_duration_s: f64,
    pub required_half_duration_s: f64, pub minimum_duration_s: f64,
    pub outcome: SnowTerminalNumericsFailure,
    pub provider_calls_before: u64, pub provider_calls_after: u64,
}
```

The live joint parameter at this chain is
`Option<CoveredTerminalJointTrialStateV1>`; `CaptureEvidence` implements exactly
`TerminalEvidenceMode<Option<CoveredTerminalJointTrialStateV1>>`.

```rust
#[cfg(test)] #[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct DiagnosticF64 { pub bits: u64, pub finite: bool }
#[cfg(test)] #[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SupportEvidence { pub start_ns: u128, pub end_ns: u128 }
#[cfg(test)] #[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct DigestEvidence(pub [u8; 32]);
#[cfg(test)] #[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PairPosition { Coarse, Fine1, Fine2 }
#[cfg(test)] #[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ProviderRole { Full, Half1, Half2, Retry, BracketLower, BracketUpper, Root }
#[cfg(test)] #[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PairComponent { Ice, Liquid, Cold, CompleteEnergy, UnallocatedEnergy }
#[cfg(test)] #[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PairDecision { Accept, RejectRetry }

#[cfg(test)] #[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TerminalStateEvidence { pub ice: DiagnosticF64, pub liquid: DiagnosticF64, pub cold: DiagnosticF64 }
#[cfg(test)] #[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TerminalLedgerEvidence {
 pub complete_energy: DiagnosticF64, pub cold_energy_change: DiagnosticF64,
 pub refrozen: DiagnosticF64, pub deposition: DiagnosticF64,
 pub sublimation: DiagnosticF64, pub melt: DiagnosticF64,
 pub unallocated_energy: DiagnosticF64, pub shortwave: DiagnosticF64,
 pub longwave: DiagnosticF64, pub sensible: DiagnosticF64,
 pub latent: DiagnosticF64, pub advected: DiagnosticF64,
 pub snow_soil_heat: DiagnosticF64, pub external_liquid: DiagnosticF64,
}
#[cfg(test)] #[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct JointEvidence {
 pub source_owner_set: DigestEvidence, pub lane_id: u32,
 pub source_snow_owner: DigestEvidence, pub interval_index: u64,
 pub support: SupportEvidence, pub accepted_predecessors: Vec<DigestEvidence>,
 pub vegetation: Vec<u8>, pub snow: Vec<u8>, pub land_surface_energy: Vec<u8>,
 pub hydrology: Vec<u8>, pub bgc: Vec<u8>, pub soil_thermal: Vec<u8>,
 pub surface_liquid: Vec<u8>, pub receipt: DigestEvidence,
}
#[cfg(test)] #[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct IngressParcelEvidence { pub kind: u8, pub parcel_id: Vec<u8>, pub mass: DiagnosticF64 }
#[cfg(test)] #[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Wb14ReceiptEvidence { pub kind: u8, pub disposition: u8, pub parcel_id: Vec<u8>, pub mass: DiagnosticF64 }
#[cfg(test)] #[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CarrierEvidence {
 pub event_ordinal: u64, pub provider_call_ordinal: u64, pub arena_ordinal: usize,
 pub support: SupportEvidence, pub role: ProviderRole, pub attempt: u32,
 pub coupling: u32, pub beginning_joint: JointEvidence,
 pub ending_joint: JointEvidence, pub open_ingress: Vec<IngressParcelEvidence>,
 pub wb14_receipts: Vec<Wb14ReceiptEvidence>,
}
#[cfg(test)] #[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct IterationEvidence {
 pub event_ordinal: u64, pub carrier_ordinal: usize, pub ordinal: usize,
 pub flux: TerminalLedgerEvidence, pub preview: TerminalStateEvidence,
 pub incoming_hint: Option<[DiagnosticF64; 4]>, pub outgoing_hint: [DiagnosticF64; 4],
 pub component_within: [bool; 4], pub converged: bool,
 pub beginning_joint: Option<JointEvidence>, pub ending_joint: Option<JointEvidence>,
}
#[cfg(test)] #[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SelectionEvidence {
 pub event_ordinal: u64, pub first_iteration: usize, pub iteration_count: usize,
 pub selected_iteration: usize, pub selected_carrier: usize,
 pub selected_converged: bool, pub support: SupportEvidence,
 pub role: ProviderRole, pub attempt: u32,
}
#[cfg(test)] #[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SelectedTrialEvidence {
 pub event_ordinal: u64, pub ordinal: usize, pub position: PairPosition,
 pub support: SupportEvidence, pub role: ProviderRole, pub attempt: u32,
 pub beginning: TerminalStateEvidence, pub ending: TerminalStateEvidence,
 pub ledger: TerminalLedgerEvidence, pub selection_ordinal: usize,
 pub beginning_joint: Option<JointEvidence>, pub ending_joint: Option<JointEvidence>,
}
#[cfg(test)] #[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PairComponentEvidence {
 pub component: PairComponent, pub coarse: DiagnosticF64, pub refined: DiagnosticF64,
 pub delta: DiagnosticF64, pub abs_tol: DiagnosticF64, pub rel_tol: DiagnosticF64,
 pub denominator: DiagnosticF64, pub scaled: DiagnosticF64,
}
#[cfg(test)] #[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PairDecisionEvidence {
 pub event_ordinal: u64, pub chronology_ordinal: u64, pub pair_ordinal: usize,
 pub coarse_selected: usize, pub fine_1_selected: usize, pub fine_2_selected: usize,
 pub components: [PairComponentEvidence; 5], pub maximum_scaled: DiagnosticF64,
 pub winner: PairComponent, pub decision: PairDecision,
 pub current_duration: DiagnosticF64, pub proposed_next: Option<DiagnosticF64>,
}
#[cfg(test)] #[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TrialAdmissionEvidence {
 pub event_ordinal: u64, pub chronology_ordinal: u64, pub support: SupportEvidence,
 pub proposed_duration: DiagnosticF64, pub required_half: DiagnosticF64,
 pub minimum: DiagnosticF64, pub outcome: u8,
 pub provider_calls_before: u64, pub provider_calls_after: u64,
}
#[cfg(test)] #[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ZeroIngressEvidence {
 pub source: u8, pub inspected: usize, pub matches: Vec<usize>,
 pub hydrology_external_liquid: Option<DiagnosticF64>,
}
```

Source tags are constants: hydrology `0`, WB14 `1`, surface `2`; live parcel
kind tags follow the source enum discriminant order and `TerminalReceiver` is
`7`. WB14 scans `CarrierEvidence.wb14_receipts`; surface scans
`CarrierEvidence.open_ingress`; hydrology scans every selected ledger's exact
rain-derived external-liquid scalar. The focused fixture requires all those
scalars to positive-zero bits and both exact-kind match vectors empty.

```rust
#[cfg(test)] #[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct LayerEvidence {
 pub mass_swe_m: DiagnosticF64, pub thickness_m: DiagnosticF64,
 pub density_kg_m3: DiagnosticF64, pub settle_day_count: DiagnosticF64,
 pub temperature_c: DiagnosticF64, pub liquid_water_m: DiagnosticF64,
 pub cold_content_j_m2: DiagnosticF64, pub refrozen_liquid_m: DiagnosticF64,
}
#[cfg(test)] #[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PersistentStateEvidence {
 pub schema_version: u16, pub terminal_event_model: Option<u8>,
 pub fingerprint: u64, pub lane_id: u32, pub next_interval_index: u64,
 pub layers: Vec<LayerEvidence>, pub detached_retained_liquid: DiagnosticF64,
 pub initial_ice: DiagnosticF64, pub initial_retained_liquid: DiagnosticF64,
 pub cumulative_snowfall: DiagnosticF64,
 pub cumulative_external_liquid: DiagnosticF64,
 pub cumulative_deposition: DiagnosticF64, pub cumulative_sublimation: DiagnosticF64,
 pub cumulative_melt: DiagnosticF64, pub cumulative_unresolved_liquid: DiagnosticF64,
 pub cumulative_complete_energy: DiagnosticF64,
 pub cumulative_cold_energy_change: DiagnosticF64,
 pub cumulative_terminal_unallocated_energy: DiagnosticF64,
}
#[cfg(test)] #[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ClockEvidence {
 pub run_identity: DigestEvidence, pub calendar_receipt: DigestEvidence,
 pub forcing_receipt: DigestEvidence, pub parent_transaction_sequence: u128,
 pub committed: bool, pub begin_owner_set: DigestEvidence,
 pub begin_clock: DigestEvidence, pub accepted_clock: DigestEvidence,
 pub parent_interval_id: Vec<u8>, pub parent_transaction_id: Vec<u8>,
 pub parent_support: SupportEvidence, pub accepted_until_ns: u128,
 pub segment_ordinal: u32, pub slab_ordinal: u32, pub event_ordinal: u32,
 pub last_accepted_step_ns: Option<u128>, pub complete_owners: Vec<Vec<u8>>,
 pub active_regime_id: Vec<u8>, pub active_segment_start_ns: u128,
 pub active_segment_end_ns: u128, pub active_segment_id: Vec<u8>,
 pub active_participants: Vec<Vec<u8>>, pub accepted_slabs: Vec<Vec<u8>>,
 pub accepted_events: Vec<Vec<u8>>, pub scheduled_once: Vec<Vec<u8>>,
 pub controller_policy: DigestEvidence, pub controller_checkpoint: Vec<u8>,
}
#[cfg(test)] #[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PendingParcelEvidence {
 pub support: SupportEvidence, pub source_lane_id: u32,
 pub parent_transaction_id: DigestEvidence, pub event_ordinal: u32,
 pub proposal_core: DigestEvidence, pub event_result: DigestEvidence,
 pub receiver_topology: DigestEvidence, pub destination_ofe_id: Vec<u8>,
 pub destination_tile_id: Vec<u8>, pub destination_fraction: DiagnosticF64,
 pub mass: DiagnosticF64, pub temperature: DiagnosticF64,
 pub specific_enthalpy: DiagnosticF64, pub posture: u8,
 pub parcel_digest: DigestEvidence,
}
#[cfg(test)] #[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct NoninterferenceSnapshot {
 pub vegetation: Vec<u8>, pub snow: Vec<u8>, pub land_surface_energy: Vec<u8>,
 pub hydrology: Vec<u8>, pub bgc: Vec<u8>, pub soil_thermal: Vec<u8>,
 pub surface_liquid: Vec<u8>, pub joint: Option<JointEvidence>,
 pub clock: ClockEvidence, pub provider_calls: u64,
 pub candidate_joint_digests: Vec<DigestEvidence>,
 pub carrier_key_digests: Vec<DigestEvidence>,
 pub pending_terminal_parcels: Vec<PendingParcelEvidence>,
 pub stage3_by_lane: Vec<(u32, PersistentStateEvidence)>,
}
```

Nested clock identities and receipts use their exact type-associated canonical
preimage/replay byte functions, never debug text or whole-clock serialization.
No wildcard map or live type is retained. Snapshot validation compares every
declared leaf recursively and returns the first static field path.

`RejectedPrefixEvidence` has an explicit constructor and fields `carriers:
Vec<CarrierEvidence>`, `iterations`, `selections`, `selected_trials`, `pairs`,
`admissions`, `zero_ingress: [ZeroIngressEvidence; 3]`, `before`, and `after`.
It does not derive `Default`.
