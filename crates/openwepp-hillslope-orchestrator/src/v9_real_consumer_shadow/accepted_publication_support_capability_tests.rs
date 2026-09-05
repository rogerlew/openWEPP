//! Contract-first expected-red evidence for SC-COUPLEDTIME v17.
//!
//! This module deliberately names the private validation-once capability,
//! live revision, and audit before their production implementation exists.

use std::collections::BTreeMap;

use openwepp_coupled_time::{
    AcceptedEventReceiptV1, ConstraintClass, CoupledClockStateV1, CoupledSlabCandidateV1, Digest32,
    EventClass, EventProposalV1, EventQueueV1, LedgerEntryV1, ModelTimeNs, OwnerState,
    ParentAuthorityV1, ParentIntervalId, ParentTransactionId, SegmentId, StepConstraintV1,
    TimeSupport, accept_slab, complete_owner_set_digest, digest_bytes, reduce_constraints,
};
use openwepp_kernel_contract::TransactionId;
use openwepp_land_surface_energy::OfeId;
use openwepp_vegetation::v11::{
    V11_COMPLETE_OWNER_MANIFEST, V11OwnerEnvelope, V11ParentTransaction,
    migrate_v10_runtime_to_v11, v11_vegetation_owner_envelope,
};

use crate::DirectOfeWb14Parameters;
use crate::land_surface_energy_shadow::EndpointFixture;
use crate::v11_vegetation_consumer::{
    DirectV11VegetationExecutor, accept_direct_v11_segment, execute_direct_v11_segment,
};

use super::{
    AcceptedPublicationHistoryLiveRevisionV1, AcceptedPublicationHistoryV1,
    AcceptedPublicationLiveRevisionPoisonV1, AcceptedPublicationSupportCapabilityAuditV1,
    DirectV9ShadowIntervalInput, DirectV10RealConsumerShadow, DirectV11RealConsumerStack,
    Stage3AcceptedPublicationSupportV1, ValidatedStage3AcceptedPublicationSupportV1,
    begin_accepted_publication_support_capability_audit_v1,
    force_full_scan_accepted_publication_history_v1,
    take_accepted_publication_support_capability_audit_v1,
};

const SUPPORT_END_NS: u128 = 1_800_000_000_000;

fn digest(seed: u8) -> Digest32 {
    Digest32::from_bytes([seed; 32])
}

fn interval(fixture: &EndpointFixture) -> DirectV9ShadowIntervalInput {
    let mut lse_forcing = fixture.forcing.clone();
    lse_forcing.transaction_id = TransactionId(41);
    lse_forcing.forcing_sha256 = lse_forcing
        .canonical_sha256()
        .expect("real LSE forcing seal");
    DirectV9ShadowIntervalInput {
        lse_forcing,
        vegetation_forcing: fixture.receipt.forcing().clone(),
        wb14_parameters: vec![DirectOfeWb14Parameters {
            ofe_id: OfeId::try_new("ofe-1").expect("real fixture OFE"),
            effective_conductivity_m_s: 1.0e-6,
            matric_potential_m: 0.1,
            infiltration_storage_capacity_m: 0.04,
        }],
    }
}

fn initial_v11_owners(
    shadow: &DirectV10RealConsumerShadow,
    vegetation: &openwepp_vegetation::v11::V11CoupledOwnedState,
) -> BTreeMap<String, V11OwnerEnvelope> {
    V11_COMPLETE_OWNER_MANIFEST
        .iter()
        .map(|owner_id| {
            let owner = match *owner_id {
                "vegetation" => {
                    v11_vegetation_owner_envelope(vegetation).expect("real vegetation owner")
                }
                "land_surface_energy" => V11OwnerEnvelope::try_new(
                    (*owner_id).to_owned(),
                    serde_json::to_vec(&shadow.inner.lse_state).expect("real LSE owner bytes"),
                )
                .expect("real LSE owner"),
                "soil_thermal" => V11OwnerEnvelope::try_new(
                    (*owner_id).to_owned(),
                    serde_json::to_vec(&shadow.inner.soil_thermal)
                        .expect("real soil-thermal owner bytes"),
                )
                .expect("real soil-thermal owner"),
                _ => {
                    V11OwnerEnvelope::try_new((*owner_id).to_owned(), owner_id.as_bytes().to_vec())
                        .expect("real complete-owner placeholder")
                }
            };
            ((*owner_id).to_owned(), owner)
        })
        .collect()
}

fn accepted_slab(
    owners: &[OwnerState],
    support_start_ns: u128,
    support_end_ns: u128,
    parent_ordinal: u128,
) -> (
    ParentTransactionId,
    openwepp_coupled_time::AcceptedSlabReceiptV1,
) {
    let support = TimeSupport::new(
        ModelTimeNs::new(support_start_ns),
        ModelTimeNs::new(support_end_ns),
    )
    .expect("real support");
    let beginning = complete_owner_set_digest(owners).expect("real owner-set digest");
    let interval = ParentIntervalId::derive(digest(1), digest(2), digest(3), support)
        .expect("real parent interval");
    let parent = ParentTransactionId::derive(digest(1), parent_ordinal, interval, beginning)
        .expect("real parent");
    let authority = ParentAuthorityV1::new(
        digest(1),
        digest(2),
        digest(3),
        parent_ordinal,
        support,
        beginning,
    )
    .expect("real parent authority");
    let participants = owners
        .iter()
        .map(|owner| owner.owner_id().to_owned())
        .collect::<Vec<_>>();
    let mut clock = CoupledClockStateV1::new(
        authority,
        owners.to_vec(),
        "snow-free".to_owned(),
        participants.clone(),
        digest(4),
        Vec::new(),
    )
    .expect("real coupled clock");
    let constraint = StepConstraintV1::new(
        parent,
        ModelTimeNs::new(support_start_ns),
        ModelTimeNs::new(support_end_ns),
        "vegetation".to_owned(),
        ConstraintClass::HardBoundary,
        digest(5),
        digest(2),
        digest(3),
    )
    .expect("real constraint");
    let reduced = reduce_constraints(
        &[constraint],
        parent,
        ModelTimeNs::new(support_start_ns),
        ModelTimeNs::new(support_end_ns),
        None,
    )
    .expect("real reduced constraint");
    let mut participant_bytes = Vec::new();
    for participant in &participants {
        participant_bytes.extend_from_slice(participant.as_bytes());
        participant_bytes.push(0);
    }
    let segment = SegmentId::derive(
        parent,
        0,
        support,
        digest_bytes(b"snow-free"),
        digest_bytes(&participant_bytes),
    )
    .expect("real segment");
    let joined = digest(6);
    let ledger = LedgerEntryV1::new(
        "vegetation".to_owned(),
        "owner".to_owned(),
        joined,
        joined,
        digest(7),
    )
    .expect("real ledger");
    let slab = CoupledSlabCandidateV1::new(
        &clock,
        segment,
        support,
        &reduced,
        owners.to_vec(),
        vec![ledger],
    )
    .expect("real slab");
    let receipt = accept_slab(&mut clock, slab).expect("real accepted slab");
    (parent, receipt)
}

fn execute_one_real_support(
    force_full_reference: bool,
) -> (
    DirectV10RealConsumerShadow,
    AcceptedPublicationHistoryLiveRevisionV1,
) {
    let (shadow, fixture) = super::tests::v10_shadow_fixture();
    let initial_revision = shadow.accepted_publication_history.live_revision_v1();
    let interval = interval(&fixture);
    let migrated =
        migrate_v10_runtime_to_v11(&shadow.vegetation_configuration, &shadow.vegetation_state)
            .expect("real V10-to-V11 migration");
    let owners = initial_v11_owners(&shadow, &migrated.state);
    let clock_owners = owners
        .values()
        .map(|owner| owner.to_owner_state().expect("real clock owner"))
        .collect::<Vec<_>>();
    let (parent_id, slab) = accepted_slab(&clock_owners, 0, SUPPORT_END_NS, 40);
    let mut parent = V11ParentTransaction::new_with_complete_owners(
        &migrated.configuration,
        &migrated.state,
        parent_id,
        ModelTimeNs::new(0),
        owners,
    )
    .expect("real V11 parent");
    let stack = DirectV11RealConsumerStack::new(&shadow, &interval, 0, 0);
    let mut executor = DirectV11VegetationExecutor { stack };
    let _forced_full_scan =
        force_full_reference.then(force_full_scan_accepted_publication_history_v1);
    let segment =
        execute_direct_v11_segment(&migrated.configuration, &parent, &slab, &mut executor)
            .expect("real V11 segment execution");
    accept_direct_v11_segment(
        &mut parent,
        &migrated.configuration,
        segment,
        &executor.stack.beginning,
    )
    .expect("real V11 segment acceptance");
    let ending = executor
        .stack
        .commit_selected_publication_and_take_staged_ending()
        .expect("real accepted publication ending");
    (ending, initial_revision)
}

fn execute_one_real_publication_day() -> DirectV10RealConsumerShadow {
    let (mut shadow, fixture) = super::tests::v10_shadow_fixture();
    let migrated =
        migrate_v10_runtime_to_v11(&shadow.vegetation_configuration, &shadow.vegetation_state)
            .expect("real publication-day initial migration");
    let mut owners = initial_v11_owners(&shadow, &migrated.state);
    for interval_index in 0..48 {
        let start_ns = u128::try_from(interval_index)
            .expect("publication interval width")
            .checked_mul(SUPPORT_END_NS)
            .expect("publication interval start");
        let end_ns = start_ns
            .checked_add(SUPPORT_END_NS)
            .expect("publication interval end");
        let parent_ordinal = 40_u128
            .checked_add(u128::try_from(interval_index).expect("publication ordinal width"))
            .expect("publication parent ordinal");
        let transaction_id = shadow
            .vegetation_state
            .0
            .last_transaction_id
            .checked_add(1)
            .expect("publication transaction id");
        let mut interval = interval(&fixture);
        interval.lse_forcing.transaction_id = TransactionId(transaction_id);
        interval.lse_forcing.forcing_sha256 = interval
            .lse_forcing
            .canonical_sha256()
            .expect("publication-day LSE forcing seal");
        let migrated =
            migrate_v10_runtime_to_v11(&shadow.vegetation_configuration, &shadow.vegetation_state)
                .expect("real publication-day migration");
        let clock_owners = owners
            .values()
            .map(|owner| owner.to_owner_state().expect("publication-day clock owner"))
            .collect::<Vec<_>>();
        let (parent_id, slab) = accepted_slab(&clock_owners, start_ns, end_ns, parent_ordinal);
        let mut parent = V11ParentTransaction::new_with_complete_owners(
            &migrated.configuration,
            &migrated.state,
            parent_id,
            ModelTimeNs::new(start_ns),
            owners,
        )
        .expect("real publication-day parent");
        let stack = DirectV11RealConsumerStack::new(&shadow, &interval, 0, interval_index);
        let mut executor = DirectV11VegetationExecutor { stack };
        let segment =
            execute_direct_v11_segment(&migrated.configuration, &parent, &slab, &mut executor)
                .expect("real publication-day segment");
        owners = segment.ending_resource_owners.clone();
        owners.insert(
            "vegetation".to_owned(),
            v11_vegetation_owner_envelope(&segment.ending_state)
                .expect("publication-day vegetation owner"),
        );
        accept_direct_v11_segment(
            &mut parent,
            &migrated.configuration,
            segment,
            &executor.stack.beginning,
        )
        .expect("real publication-day segment acceptance");
        shadow = executor
            .stack
            .commit_selected_publication_and_take_staged_ending()
            .expect("real publication-day accepted ending");
    }
    assert_eq!(shadow.accepted_publication_supports().len(), 48);
    shadow
}

fn assert_private_move_only_shape<T>() {
    assert!(
        std::mem::needs_drop::<T>(),
        "the owning capability must have a nontrivial move/drop lifecycle",
    );
    assert!(std::mem::size_of::<T>() > 0);
    assert!(std::any::type_name::<T>().ends_with("ValidatedStage3AcceptedPublicationSupportV1"));
}

fn one_real_support() -> Stage3AcceptedPublicationSupportV1 {
    let (ending, _) = execute_one_real_support(true);
    let mut supports = ending.accepted_publication_supports();
    assert_eq!(supports.len(), 1);
    supports.remove(0)
}

fn fully_validate_for_history(
    support: Stage3AcceptedPublicationSupportV1,
    history: &AcceptedPublicationHistoryV1,
) -> ValidatedStage3AcceptedPublicationSupportV1 {
    support.validate().expect("real support full validation");
    ValidatedStage3AcceptedPublicationSupportV1::mint(support, history.live_revision_v1())
}

fn accepted_event_pair() -> [AcceptedEventReceiptV1; 2] {
    let beginning = vec![
        OwnerState::new("event-owner".to_owned(), b"a".to_vec()).expect("event beginning owner"),
    ];
    let support =
        TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(10)).expect("event parent support");
    let authority = ParentAuthorityV1::new(
        digest(31),
        digest(32),
        digest(33),
        0,
        support,
        complete_owner_set_digest(&beginning).expect("event beginning digest"),
    )
    .expect("event parent authority");
    let mut clock = CoupledClockStateV1::new(
        authority,
        beginning,
        "event-regime".to_owned(),
        vec!["event-owner".to_owned()],
        digest(34),
        Vec::new(),
    )
    .expect("event clock");
    let proposal = |context, state: &[u8], ledger| {
        EventProposalV1::new(
            EventClass::OwnershipTransfer,
            "event-owner".to_owned(),
            digest(context),
            vec![
                OwnerState::new("event-owner".to_owned(), state.to_vec())
                    .expect("event ending owner"),
            ],
            vec!["event-owner".to_owned()],
            "event-regime".to_owned(),
            vec!["event-owner".to_owned()],
            vec![
                LedgerEntryV1::new(
                    "event-owner".to_owned(),
                    "owner-state".to_owned(),
                    digest(ledger),
                    digest(ledger),
                    digest(ledger.wrapping_add(1)),
                )
                .expect("event ledger"),
            ],
        )
        .expect("event proposal")
    };
    let mut queue = EventQueueV1::new(
        ModelTimeNs::new(0),
        vec![proposal(35, b"b", 36), proposal(37, b"c", 38)],
    )
    .expect("ordered event queue");
    let first = queue
        .apply_next(&mut clock)
        .expect("first event application")
        .expect("first accepted event");
    let second = queue
        .apply_next(&mut clock)
        .expect("second event application")
        .expect("second accepted event");
    [first, second]
}

#[test]
fn accepted_publication_support_capability_validates_once_and_appends_by_exact_live_tail() {
    assert_private_move_only_shape::<ValidatedStage3AcceptedPublicationSupportV1>();
    assert!(std::mem::size_of::<AcceptedPublicationHistoryLiveRevisionV1>() > 0);
    assert!(std::mem::size_of::<AcceptedPublicationSupportCapabilityAuditV1>() > 0);

    let (forced_full, _) = execute_one_real_support(true);
    begin_accepted_publication_support_capability_audit_v1();
    let (optimized, initial_revision) = execute_one_real_support(false);
    let audit = take_accepted_publication_support_capability_audit_v1();

    assert_eq!(
        optimized.accepted_publication_supports(),
        forced_full.accepted_publication_supports(),
        "the trusted append must preserve the real support payload bit-for-bit",
    );
    assert_eq!(
        optimized
            .canonical_owner_state_bytes()
            .expect("optimized complete-owner bytes"),
        forced_full
            .canonical_owner_state_bytes()
            .expect("forced-full complete-owner bytes"),
        "validation-once append must equal the independent full-validation reference",
    );
    assert_ne!(
        optimized.accepted_publication_history.live_revision_v1(),
        initial_revision,
        "a successful append must advance the exact live revision",
    );
    assert_eq!(
        audit,
        AcceptedPublicationSupportCapabilityAuditV1 {
            full_validation_attempt_count: 1,
            full_validation_success_count: 1,
            operand_seal_count: 1,
            receipt_seal_count: 1,
            capability_mint_count: 1,
            trusted_append_attempt_count: 1,
            live_revision_join_count: 1,
            chronology_owner_tail_join_count: 1,
            successful_append_count: 1,
            append_time_full_validation_count: 0,
            append_time_operand_reconstruction_count: 0,
            append_time_receipt_reconstruction_count: 0,
            append_time_serialization_count: 0,
            append_time_full_prefix_scan_count: 0,
            support_payload_clone_count: 0,
        },
    );
}

#[test]
fn accepted_publication_support_tail_revision_poison_matrix_is_exhaustive() {
    use AcceptedPublicationLiveRevisionPoisonV1 as Poison;

    let real = one_real_support();
    let poisons = [
        Poison::ForeignIncarnation,
        Poison::Sequence,
        Poison::CumulativeSupportCount,
        Poison::CumulativeEventCount,
        Poison::ResidentSupportCount,
        Poison::ResidentEventCount,
        Poison::LastDayIndex,
        Poison::LastIntervalIndex,
        Poison::LastSupport,
        Poison::LastParentTransaction,
        Poison::LastAcceptedSlab,
        Poison::TraversedEndingOwner,
        Poison::PendingPreSupportEvent,
        Poison::EventIdCount,
        Poison::CurrentEventOrdinal,
        Poison::SealedPrefixSupportCount,
        Poison::SealedPrefixEventCount,
        Poison::SealedPrefixAuthority,
        Poison::Wb14Checkpoint,
        Poison::LastWb14Replay,
        Poison::LastSupportReceipt,
        Poison::AggregateTail,
        Poison::SupportPayloadIdentity,
    ];
    for poison in poisons {
        let mut history = AcceptedPublicationHistoryV1::default();
        let before = history.clone();
        let before_revision = history.live_revision_v1();
        let capability =
            fully_validate_for_history(real.clone(), &history).poison_target_for_test(poison);
        assert!(
            history.push_validated_support(capability).is_err(),
            "{poison:?} must fail closed",
        );
        assert_eq!(history, before, "{poison:?} mutated retained bytes");
        assert_eq!(
            history.live_revision_v1(),
            before_revision,
            "{poison:?} mutated the live revision",
        );
        let successor = fully_validate_for_history(real.clone(), &history);
        history
            .push_validated_support(successor)
            .expect("freshly validated successor after consumed poison");
        assert_eq!(history.supports().len(), 1);
    }
}

#[test]
fn accepted_publication_support_capability_rejects_stale_foreign_replayed_and_restart_revisions_atomically()
 {
    let real = one_real_support();

    let original = AcceptedPublicationHistoryV1::default();
    let mut exact_clone = original.clone();
    assert_eq!(original.live_revision_v1(), exact_clone.live_revision_v1());
    exact_clone
        .push_validated_support(fully_validate_for_history(real.clone(), &original))
        .expect("exact in-process clone shares the identical live revision");
    assert!(original.supports().is_empty());
    assert_ne!(original.live_revision_v1(), exact_clone.live_revision_v1());

    let mut replay_history = AcceptedPublicationHistoryV1::default();
    let first = fully_validate_for_history(real.clone(), &replay_history);
    let replay = fully_validate_for_history(real.clone(), &replay_history);
    replay_history
        .push_validated_support(first)
        .expect("first exact capability");
    let accepted_before_replay = replay_history.clone();
    let revision_before_replay = replay_history.live_revision_v1();
    assert!(replay_history.push_validated_support(replay).is_err());
    assert_eq!(replay_history, accepted_before_replay);
    assert_eq!(replay_history.live_revision_v1(), revision_before_replay);

    let target = AcceptedPublicationHistoryV1::default();
    let foreign = fully_validate_for_history(real.clone(), &target);
    let mut independent = AcceptedPublicationHistoryV1::default();
    assert_ne!(target.live_revision_v1(), independent.live_revision_v1());
    assert!(independent.push_validated_support(foreign).is_err());
    assert!(independent.supports().is_empty());

    let mut event_history = AcceptedPublicationHistoryV1::default();
    let stale_after_event = fully_validate_for_history(real.clone(), &event_history);
    let [first_event, second_event] = accepted_event_pair();
    event_history
        .push_event_handoff(first_event)
        .expect("first authentic event append");
    let after_first_event = event_history.live_revision_v1();
    assert_eq!(after_first_event.cumulative_event_count, 1);
    assert_eq!(after_first_event.event_id_count, 1);
    assert_eq!(after_first_event.current_event_ordinal, Some(0));
    let before_stale_event_refusal = event_history.clone();
    assert!(
        event_history
            .push_validated_support(stale_after_event)
            .is_err()
    );
    assert_eq!(event_history, before_stale_event_refusal);
    assert_eq!(event_history.live_revision_v1(), after_first_event);
    event_history
        .push_event_handoff(second_event)
        .expect("second authentic event append");
    let after_second_event = event_history.live_revision_v1();
    assert_eq!(after_second_event.cumulative_event_count, 2);
    assert_eq!(after_second_event.event_id_count, 2);
    assert_eq!(after_second_event.current_event_ordinal, Some(1));
    assert_ne!(
        after_second_event.aggregate_tail_sha256,
        after_first_event.aggregate_tail_sha256
    );

    let mut replaced = AcceptedPublicationHistoryV1::default();
    let pre_replacement = fully_validate_for_history(real, &replaced);
    let old_revision = replaced.live_revision_v1();
    replaced
        .replace(Vec::new(), &[])
        .expect("empty untrusted/restart-equivalent replacement");
    assert_ne!(replaced.live_revision_v1(), old_revision);
    let replaced_before = replaced.clone();
    assert!(replaced.push_validated_support(pre_replacement).is_err());
    assert_eq!(replaced, replaced_before);
}

#[test]
fn accepted_publication_support_untrusted_restore_revalidates_every_support_and_never_restores_capability()
 {
    let real = one_real_support();
    let mut restored = AcceptedPublicationHistoryV1::default();
    let pre_restore_revision = restored.live_revision_v1();

    begin_accepted_publication_support_capability_audit_v1();
    restored
        .replace(vec![real.clone()], &[])
        .expect("one authentic untrusted support restore");
    let audit = take_accepted_publication_support_capability_audit_v1();
    assert_eq!(audit.full_validation_attempt_count, 1);
    assert_eq!(audit.full_validation_success_count, 1);
    assert_eq!(audit.operand_seal_count, 1);
    assert_eq!(audit.receipt_seal_count, 1);
    assert_eq!(audit.capability_mint_count, 0);
    assert_eq!(audit.trusted_append_attempt_count, 0);
    assert_eq!(audit.successful_append_count, 0);
    assert_ne!(restored.live_revision_v1(), pre_restore_revision);
    assert_eq!(restored.supports().len(), 1);

    let before_poison = restored.clone();
    let before_poison_revision = restored.live_revision_v1();
    let mut poisoned = real;
    poisoned.receipt_sha256 = Digest32::zero();
    begin_accepted_publication_support_capability_audit_v1();
    assert!(restored.replace(vec![poisoned], &[]).is_err());
    let poison_audit = take_accepted_publication_support_capability_audit_v1();
    assert_eq!(poison_audit.full_validation_attempt_count, 1);
    assert_eq!(poison_audit.full_validation_success_count, 0);
    assert_eq!(poison_audit.capability_mint_count, 0);
    assert_eq!(restored, before_poison);
    assert_eq!(restored.live_revision_v1(), before_poison_revision);
}

#[test]
fn accepted_publication_rotation_invalidates_pre_rotation_support_capability() {
    let append_support = one_real_support();
    let mut shadow = execute_one_real_publication_day();
    let pre_rotation =
        fully_validate_for_history(append_support, &shadow.accepted_publication_history);
    let evidence = shadow
        .seal_accepted_publication_day_evidence_v1(0)
        .expect("authentic complete-day evidence");
    shadow
        .rotate_accepted_publication_day_v1(&evidence)
        .expect("acknowledged authentic rotation");
    let rotated_before_refusal = shadow.accepted_publication_history.clone();
    let rotated_revision = shadow.accepted_publication_history.live_revision_v1();
    assert!(
        shadow
            .accepted_publication_history
            .push_validated_support(pre_rotation)
            .is_err(),
    );
    assert_eq!(shadow.accepted_publication_history, rotated_before_refusal);
    assert_eq!(
        shadow.accepted_publication_history.live_revision_v1(),
        rotated_revision,
    );
}

#[test]
fn accepted_publication_support_forbidden_append_work_audit_is_non_vacuous() {
    let support = one_real_support();
    let history = AcceptedPublicationHistoryV1::default();
    begin_accepted_publication_support_capability_audit_v1();
    {
        let _scope = super::accepted_publication_support_capability::
            enter_trusted_accepted_publication_append_audit_scope_v1();
        let cloned = support.clone();
        cloned
            .validate()
            .expect("scoped real support validation probe");
        let _wire = support.to_wire();
        history
            .validate_cached_tail_against_full_scan()
            .expect("scoped real full-prefix probe");
    }
    let audit = take_accepted_publication_support_capability_audit_v1();
    assert_eq!(audit.append_time_full_validation_count, 1);
    assert_eq!(audit.append_time_operand_reconstruction_count, 1);
    assert_eq!(audit.append_time_receipt_reconstruction_count, 1);
    assert_eq!(audit.append_time_serialization_count, 1);
    assert_eq!(audit.append_time_full_prefix_scan_count, 1);
    assert_eq!(audit.support_payload_clone_count, 1);
}

#[test]
fn accepted_publication_support_capability_is_private_move_only_non_wire() {
    let capability_source = include_str!("accepted_publication_support_capability.rs");
    let declaration = capability_source
        .split("pub(crate) struct ValidatedStage3AcceptedPublicationSupportV1")
        .nth(1)
        .expect("private capability declaration")
        .split("impl ValidatedStage3AcceptedPublicationSupportV1")
        .next()
        .expect("private capability body");
    assert!(!declaration.contains("derive("));
    assert!(!declaration.contains("Serialize"));
    assert!(!declaration.contains("Deserialize"));
    for forbidden_impl in [
        "impl Clone for ValidatedStage3AcceptedPublicationSupportV1",
        "impl serde::Serialize for ValidatedStage3AcceptedPublicationSupportV1",
        "impl Serialize for ValidatedStage3AcceptedPublicationSupportV1",
        "impl<'de> Deserialize<'de> for ValidatedStage3AcceptedPublicationSupportV1",
        "impl Deserialize for ValidatedStage3AcceptedPublicationSupportV1",
    ] {
        assert!(
            !capability_source.contains(forbidden_impl),
            "private capability contains forbidden implementation {forbidden_impl}",
        );
    }

    let retention_source = include_str!("../v9_real_consumer_shadow_publication_retention.rs");
    let trusted = retention_source
        .split("fn push_validated_support(")
        .nth(1)
        .expect("trusted consuming append")
        .split("fn push_event_handoff(")
        .next()
        .expect("trusted append body");
    for forbidden in [
        ".validate(",
        "operands_sha256(",
        "reconstructed_receipt_sha256(",
        "serde_json",
        "validate_cached_tail_against_full_scan",
        ".clone()",
    ] {
        assert!(
            !trusted.contains(forbidden),
            "trusted append contains forbidden operation {forbidden}",
        );
    }
    let installer = capability_source
        .split("pub(super) fn install_validated_support(")
        .nth(1)
        .expect("trusted installer")
        .split("#[cfg(test)]\n#[derive(Clone, Copy, Debug)]")
        .next()
        .expect("trusted installer body");
    for forbidden in [
        "support.validate(",
        "support.validate_semantics(",
        "operands_sha256(",
        "reconstructed_receipt_sha256(",
        "serde_json",
        "bincode",
        ".to_wire(",
        "support.clone()",
        "(*support).clone()",
        "clone_from(&support)",
    ] {
        assert!(
            !installer.contains(forbidden),
            "trusted installer contains forbidden operation {forbidden}",
        );
    }
    assert!(!retention_source.contains("fn push_support("));
}
