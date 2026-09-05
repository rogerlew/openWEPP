#![allow(dead_code)] // Retained historical V2 poison fixtures stay decodable but cannot encode V3 custody.

use std::collections::BTreeMap;

use openwepp_coupled_time::{ModelTimeNs, TimeSupport, digest_bytes};
use openwepp_hillslope_orchestrator::{
    DirectActiveSnowPartitionInputs, DirectOfeWb14Parameters, DirectSnowHourlyForcing,
    DirectSnowLayerState, DirectSnowStage3PersistentState, DirectSnowStage3SupportInput,
    DirectSnowSurfaceEnergyOptions, DirectSnowTerminalEventRequest, SnowDensityModel,
    SnowMeltModel, SnowStage3LiquidRoutingModel, SnowSurfaceSublimationModel, Wb11HydrologyKernel,
    snow_stage3_terminal_handoff::SealedCoveredCarrierForcing,
    snow_stage3_v11_attachment::{
        DirectSnowStage3V11DestinationCapabilityV1, DirectSnowStage3V11DualRegimeSupportInputsV1,
        DirectSnowStage3V11InterruptionPostureV2, DirectSnowStage3V11PreparedSupport,
        DirectSnowStage3V11ProductionConfigurationV1, DirectSnowStage3V11ShadowAttachment,
        PreparedStage3V11DayV1, PreparedStage3V11SupportIdentityV1,
    },
    v9_real_consumer_shadow::DirectV11SnowCoveredSegmentInput,
};
use openwepp_kernel_contract::TileId;
use openwepp_land_surface_energy::OfeId;

use super::*;
use crate::{
    DirectHydrologyRestartV1, ExpectedDirectHydrologyRestartContext, checkpoint_identities_v1,
    restart_authority_adaptive_cross_midnight_carry_fixture,
    restart_authority_adaptive_prepared_day_fixture,
    restart_authority_adaptive_snow_prepared_day_fixture, restart_authority_owner_fixture,
};

include!("snow_stage3_v11_v3_tests.rs");

#[test]
fn restored_direct_v10_owner_executes_only_a_fresh_snow_free_stack() {
    use openwepp_coupled_time::{
        ConstraintClass, CoupledClockStateV1, CoupledSlabCandidateV1, LedgerEntryV1,
        ParentAuthorityV1, ParentIntervalId, SegmentId, StepConstraintV1, accept_slab,
        complete_owner_set_digest, reduce_constraints,
    };
    use openwepp_hillslope_orchestrator::{
        DirectWb14CoupledChildBindingV1,
        v9_real_consumer_shadow::{
            DirectV9ShadowIntervalInput, restart_authority_execute_fresh_snow_free_segment_v1,
            restart_authority_v11_parent_owner_envelopes_v1,
        },
    };
    use openwepp_kernel_contract::TransactionId;
    use openwepp_vegetation::v11::{V11ParentTransaction, migrate_v10_runtime_to_v11};

    const END_NS: u128 = 900_000_000_000;
    let fixture = restart_authority_owner_fixture();
    let (run, topology) = checkpoint_identities_v1(
        &fixture.committed,
        fixture.runtime.shadow.root_zone_hydraulic_configuration(),
    )
    .unwrap();
    let context = ExpectedRestartStaticContext {
        run_identity_sha256: &run,
        topology_sha256: &topology,
        vegetation_configuration: fixture
            .runtime
            .shadow
            .restart_authority_vegetation_configuration(),
        vegetation_owner_id: fixture
            .runtime
            .shadow
            .restart_authority_vegetation_owner_id(),
        soil_thermal_owner_id: &fixture
            .runtime
            .shadow
            .restart_authority_soil_thermal()
            .unwrap()
            .owner_id,
        soil_thermal_configuration_sha256: &fixture
            .runtime
            .shadow
            .restart_authority_soil_thermal()
            .unwrap()
            .configuration_sha256,
        lse_configuration: fixture.runtime.shadow.restart_authority_lse_configuration(),
        surface_liquid_configuration: fixture
            .runtime
            .shadow
            .restart_authority_surface_configuration(),
        gsi_configuration: fixture.runtime.shadow.gsi_owner_configuration(),
        forcing_static_configuration: fixture.runtime.shadow.provider_static_configuration(),
        root_zone_hydraulic_configuration: fixture
            .runtime
            .shadow
            .root_zone_hydraulic_configuration(),
        phase_plan: &fixture
            .runtime
            .shadow
            .restart_authority_hydrology_frame()
            .phase_plan,
        phase_plan_sha256: &fixture.phase_plan_sha256,
        day_inputs: &fixture.day_inputs,
        day_input_digests: &fixture.day_input_digests,
    };
    let mut checkpoint = DirectV10RealConsumerCheckpointV1 {
        schema: "OPENWEPP_DIRECT_V10_REAL_CONSUMER_CHECKPOINT_V1".into(),
        version: 1,
        run_identity_sha256: run.clone(),
        topology_sha256: topology.clone(),
        phase: DirectV10CheckpointPhaseV1::BetweenDays {
            next_day_index: WireDayIndex(0),
            accepted_interval_count: AcceptedIntervalCount::try_new(0).unwrap(),
            committed: fixture.committed.clone(),
        },
        payload_sha256: Sha256Hex::try_new("0".repeat(64)).unwrap(),
    };
    checkpoint.seal().unwrap();
    let isolated =
        admit_checkpoint_v1(&to_canonical_bytes(&checkpoint).unwrap(), &context).unwrap();
    let restored = DirectV10RestartHost::from_isolated(isolated, &context).unwrap();
    let restored_shadow = restored.shadow();

    let migrated = migrate_v10_runtime_to_v11(
        restored_shadow.restart_authority_vegetation_configuration(),
        restored_shadow.vegetation_state(),
    )
    .unwrap();
    let owners = restart_authority_v11_parent_owner_envelopes_v1(restored_shadow).unwrap();
    let owner_states = owners
        .values()
        .map(|owner| owner.to_owner_state().unwrap())
        .collect::<Vec<_>>();
    let owner_digest = complete_owner_set_digest(&owner_states).unwrap();
    let support = TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(END_NS)).unwrap();
    let interval_id = ParentIntervalId::derive(
        digest_bytes(b"restart-run"),
        digest_bytes(b"restart-forcing"),
        digest_bytes(b"restart-policy"),
        support,
    )
    .unwrap();
    let parent_id = openwepp_coupled_time::ParentTransactionId::derive(
        digest_bytes(b"restart-run"),
        40,
        interval_id,
        owner_digest,
    )
    .unwrap();
    let authority = ParentAuthorityV1::new(
        digest_bytes(b"restart-run"),
        digest_bytes(b"restart-forcing"),
        digest_bytes(b"restart-policy"),
        40,
        support,
        owner_digest,
    )
    .unwrap();
    let participants = owners.keys().cloned().collect::<Vec<_>>();
    let mut clock = CoupledClockStateV1::new(
        authority,
        owner_states.clone(),
        "restored-snow-free".to_owned(),
        participants.clone(),
        digest_bytes(b"restart-clock"),
        Vec::new(),
    )
    .unwrap();
    let constraint = StepConstraintV1::new(
        parent_id,
        ModelTimeNs::new(0),
        ModelTimeNs::new(END_NS),
        "vegetation".to_owned(),
        ConstraintClass::HardBoundary,
        digest_bytes(b"restart-constraint"),
        digest_bytes(b"restart-forcing"),
        digest_bytes(b"restart-policy"),
    )
    .unwrap();
    let reduction = reduce_constraints(
        &[constraint],
        parent_id,
        ModelTimeNs::new(0),
        ModelTimeNs::new(END_NS),
        None,
    )
    .unwrap();
    let mut participant_bytes = Vec::new();
    for participant in &participants {
        participant_bytes.extend_from_slice(participant.as_bytes());
        participant_bytes.push(0);
    }
    let segment = SegmentId::derive(
        parent_id,
        0,
        support,
        digest_bytes(b"restored-snow-free"),
        digest_bytes(&participant_bytes),
    )
    .unwrap();
    let joined = digest_bytes(b"restart-ledger");
    let ledger = LedgerEntryV1::new(
        "vegetation".to_owned(),
        "owner".to_owned(),
        joined,
        joined,
        digest_bytes(b"restart-lineage"),
    )
    .unwrap();
    let slab = CoupledSlabCandidateV1::new(
        &clock,
        segment,
        support,
        &reduction,
        owner_states,
        vec![ledger],
    )
    .unwrap();
    let receipt = accept_slab(&mut clock, slab).unwrap();
    let parent = V11ParentTransaction::new_with_complete_owners(
        &migrated.configuration,
        &migrated.state,
        parent_id,
        ModelTimeNs::new(0),
        owners,
    )
    .unwrap();

    let mut lse_forcing = fixture.runtime.endpoint.forcing.clone();
    lse_forcing.interval_s = 900.0;
    lse_forcing.transaction_id = TransactionId(41);
    lse_forcing.precipitation_parcels.clear();
    lse_forcing.runon_parcels.clear();
    lse_forcing.forcing_sha256 = lse_forcing.canonical_sha256().unwrap();
    let interval = DirectV9ShadowIntervalInput {
        lse_forcing,
        vegetation_forcing: fixture.runtime.endpoint.receipt.forcing().clone(),
        wb14_parameters: vec![DirectOfeWb14Parameters {
            ofe_id: OfeId::try_new("ofe-1").unwrap(),
            effective_conductivity_m_s: 1.0e-6,
            matric_potential_m: 0.1,
            infiltration_storage_capacity_m: 0.04,
        }],
    };
    let binding = DirectWb14CoupledChildBindingV1 {
        proposed_upper_bound_s_bits: 1_800.0_f64.to_bits(),
        coupled_parent_transaction_sha256: *parent_id.digest().as_bytes(),
        accepted_slab_sha256: *receipt.slab_id().digest().as_bytes(),
        parent_beginning_complete_owner_set_sha256: *owner_digest.as_bytes(),
        parent_support_start_ns: 0,
        parent_support_end_ns: 1_800_000_000_000,
        child_support_start_ns: 0,
        child_support_end_ns: END_NS,
    };
    let (candidate, ending, audit) = restart_authority_execute_fresh_snow_free_segment_v1(
        restored_shadow,
        &interval,
        0,
        0,
        false,
        binding,
        &migrated.configuration,
        &parent,
        &receipt,
    )
    .unwrap();
    assert_eq!(candidate.accepted_slab_receipt, receipt);
    assert_eq!(candidate.ending_resource_owners.len(), 7);
    assert_eq!(ending.restart_authority_accepted_interval_count(), 0);
    assert_eq!(audit.physical_execution_count, 1);
    assert_eq!(audit.identity_reseal_count, 0);
    assert_eq!(audit.final_publication_append_count, 0);
    assert_eq!(audit.outer_accepted_publication_count, 0);
}

fn snow_free_state(lane_id: u32) -> DirectSnowStage3PersistentState {
    DirectSnowStage3PersistentState {
        schema_version: 1,
        terminal_event_model: None,
        fingerprint: 0,
        lane_id,
        next_interval_index: 0,
        layers: Vec::new(),
        detached_retained_liquid_kg_m2: 0.0,
        initial_ice_kg_m2: 0.0,
        initial_retained_liquid_kg_m2: 0.0,
        cumulative_snowfall_kg_m2: 0.0,
        cumulative_external_liquid_kg_m2: 0.0,
        cumulative_deposition_kg_m2: 0.0,
        cumulative_sublimation_kg_m2: 0.0,
        cumulative_melt_kg_m2: 0.0,
        cumulative_unresolved_liquid_kg_m2: 0.0,
        cumulative_complete_energy_j_m2: 0.0,
        cumulative_cold_energy_change_j_m2: 0.0,
        cumulative_terminal_unallocated_energy_j_m2: 0.0,
    }
}

#[derive(Clone, Copy)]
enum AdaptiveRestartFixtureMode {
    Terminal,
    PositiveSupportLiquidCustody,
    Reappearance,
    CrossMidnight,
}

fn assert_last_v11_candidate_authority_equal(
    restored: Option<&openwepp_vegetation::V11ParentCandidate>,
    uninterrupted: Option<&openwepp_vegetation::V11ParentCandidate>,
    label: &str,
) {
    match (restored, uninterrupted) {
        (None, None) => {}
        (Some(restored), Some(uninterrupted)) => {
            assert!(
                restored.parent_transaction_id == uninterrupted.parent_transaction_id,
                "{label} parent"
            );
            assert!(
                restored.beginning_state_sha256 == uninterrupted.beginning_state_sha256,
                "{label} beginning state"
            );
            assert!(
                restored.ending_state == uninterrupted.ending_state,
                "{label} ending state"
            );
            assert!(
                restored.accepted_segment_checkpoints == uninterrupted.accepted_segment_checkpoints,
                "{label} accepted receipts"
            );
            assert!(
                restored.cumulative_debits == uninterrupted.cumulative_debits,
                "{label} cumulative debits"
            );
            assert!(
                restored.material_transfers == uninterrupted.material_transfers,
                "{label} material transfers"
            );
            assert!(
                restored.beginning_complete_owners == uninterrupted.beginning_complete_owners,
                "{label} beginning owners"
            );
            assert!(
                restored.ending_complete_owners == uninterrupted.ending_complete_owners,
                "{label} ending owners"
            );
        }
        _ => panic!("{label} posture"),
    }
}

fn assert_serialized_authority_equal<T: Serialize>(restored: &T, uninterrupted: &T, label: &str) {
    let restored = serde_json::to_vec(restored).unwrap();
    let uninterrupted = serde_json::to_vec(uninterrupted).unwrap();
    assert!(
        restored == uninterrupted,
        "{label}: restored_sha256={:?}/{}B uninterrupted_sha256={:?}/{}B",
        sha256_hex(&restored).unwrap(),
        restored.len(),
        sha256_hex(&uninterrupted).unwrap(),
        uninterrupted.len(),
    );
}

fn assert_pending_candidate_authority_equal(
    restored: &DirectSnowStage3V11ParentCandidate,
    uninterrupted: &DirectSnowStage3V11ParentCandidate,
    phase_plan_sha256: &Sha256Hex,
    day_input_digests: &[Sha256Hex],
) {
    let restored_state = project_committed_mode(
        &restored.ending_state,
        phase_plan_sha256,
        day_input_digests,
        true,
    )
    .unwrap();
    let uninterrupted_state = project_committed_mode(
        &uninterrupted.ending_state,
        phase_plan_sha256,
        day_input_digests,
        true,
    )
    .unwrap();
    assert_serialized_authority_equal(
        &restored_state.stage3_by_lane,
        &uninterrupted_state.stage3_by_lane,
        "pending candidate Stage-3 lanes",
    );
    assert_serialized_authority_equal(
        &restored_state.real_consumer,
        &uninterrupted_state.real_consumer,
        "pending candidate real-consumer owner state",
    );
    assert_serialized_authority_equal(
        &(
            restored_state.real_consumer_next_day_index,
            restored_state.real_consumer_accepted_interval_count,
            restored_state.real_consumer_provider_cursor_configuration_bound,
        ),
        &(
            uninterrupted_state.real_consumer_next_day_index,
            uninterrupted_state.real_consumer_accepted_interval_count,
            uninterrupted_state.real_consumer_provider_cursor_configuration_bound,
        ),
        "pending candidate real-consumer scheduler",
    );
    assert_serialized_authority_equal(
        &(
            &restored_state.real_consumer_wb14_parent_canonical_base64,
            &restored_state.real_consumer_wb14_parent_sha256,
        ),
        &(
            &uninterrupted_state.real_consumer_wb14_parent_canonical_base64,
            &uninterrupted_state.real_consumer_wb14_parent_sha256,
        ),
        "pending candidate WB14 parent working state",
    );
    assert_serialized_authority_equal(
        &(
            &restored_state.accepted_publication_supports_canonical_base64,
            &restored_state.accepted_publication_supports_sha256,
        ),
        &(
            &uninterrupted_state.accepted_publication_supports_canonical_base64,
            &uninterrupted_state.accepted_publication_supports_sha256,
        ),
        "pending candidate publication authority",
    );
    assert_serialized_authority_equal(
        &restored_state.v11_parent_checkpoint,
        &uninterrupted_state.v11_parent_checkpoint,
        "pending candidate V11 parent checkpoint",
    );
    assert_serialized_authority_equal(
        &(
            &restored_state.coupled_clock_canonical_base64,
            &restored_state.coupled_clock_sha256,
        ),
        &(
            &uninterrupted_state.coupled_clock_canonical_base64,
            &uninterrupted_state.coupled_clock_sha256,
        ),
        "pending candidate coupled clock",
    );
    assert_serialized_authority_equal(
        &(
            restored_state.next_parent_sequence,
            restored_state.has_last_v11_parent_candidate,
        ),
        &(
            uninterrupted_state.next_parent_sequence,
            uninterrupted_state.has_last_v11_parent_candidate,
        ),
        "pending candidate parent sequence/posture",
    );
    assert_serialized_authority_equal(
        &(
            &restored_state.receipt_state_canonical_base64,
            &restored_state.receipt_state_sha256,
        ),
        &(
            &uninterrupted_state.receipt_state_canonical_base64,
            &uninterrupted_state.receipt_state_sha256,
        ),
        "pending candidate receipt authority",
    );
    assert_last_v11_candidate_authority_equal(
        restored.ending_state.last_v11_parent_candidate.as_ref(),
        uninterrupted
            .ending_state
            .last_v11_parent_candidate
            .as_ref(),
        "pending candidate last V11 candidate authority",
    );
    // Raw parent-receipt `PartialEq` additionally observes the non-Serde
    // evaluator candidates in `V11ParentTransaction::accepted_segments`
    // and `V11ParentCandidate::accepted_segments`. Restart deliberately
    // restores their sealed checkpoint authority instead. The canonical
    // receipt-state bytes, V11 checkpoint, candidate checkpoint authority,
    // owners, event chronology, and custody receipts are asserted above.
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AcceptedPublicationAuthorityTestWireV2 {
    schema_version: u32,
    supports: Vec<serde_json::Value>,
    event_handoffs: Vec<serde_json::Value>,
    traversed_ending_complete_owner_set_sha256: Option<Digest32>,
    receipt_sha256: Digest32,
}

fn install_resealed_publication_authority_poison(
    state: &mut DirectSnowStage3V11CommittedRestartV2,
    mut wire: AcceptedPublicationAuthorityTestWireV2,
) {
    let authority_bytes = serde_json::to_vec(&(&wire.supports, &wire.event_handoffs)).unwrap();
    let mut preimage = b"OPENWEPP_ACCEPTED_PUBLICATION_AUTHORITY_RESTART_V2\0".to_vec();
    preimage.extend_from_slice(&authority_bytes);
    wire.receipt_sha256 = digest_bytes(&preimage);
    let bytes = serde_json::to_vec(&wire).unwrap();
    state.accepted_publication_supports_sha256 = sha256_hex(&bytes).unwrap();
    state.accepted_publication_supports_canonical_base64 = STANDARD.encode(bytes);
}

fn assert_event_handoff_restart_poisons(
    projected: &DirectSnowStage3V11AttachmentRestartV2,
    context: &ExpectedSnowStage3V11RestartContext<'_>,
) {
    let current = projected
        .in_progress_execution
        .as_deref()
        .unwrap()
        .support_current
        .as_ref();
    let bytes = STANDARD
        .decode(&current.accepted_publication_supports_canonical_base64)
        .unwrap();
    let wire: AcceptedPublicationAuthorityTestWireV2 = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(wire.schema_version, 2);
    assert!(!wire.supports.is_empty());
    assert!(!wire.event_handoffs.is_empty());

    let mut omission = projected.clone();
    let omission_current = omission
        .in_progress_execution
        .as_deref_mut()
        .unwrap()
        .support_current
        .as_mut();
    let mut omission_wire = wire.clone();
    omission_wire.event_handoffs.clear();
    omission_wire.traversed_ending_complete_owner_set_sha256 = omission_wire
        .supports
        .last()
        .and_then(|support| support.get("ending_complete_owner_set_sha256"))
        .cloned()
        .map(serde_json::from_value)
        .transpose()
        .unwrap();
    install_resealed_publication_authority_poison(omission_current, omission_wire);
    omission.seal().unwrap();
    assert!(omission.restore(context).is_err());

    let mut substitution = projected.clone();
    let substitution_current = substitution
        .in_progress_execution
        .as_deref_mut()
        .unwrap()
        .support_current
        .as_mut();
    let mut substitution_wire = wire.clone();
    substitution_wire.event_handoffs[0]["event_context_digest"] =
        serde_json::Value::String("11".repeat(32));
    install_resealed_publication_authority_poison(substitution_current, substitution_wire);
    substitution.seal().unwrap();
    assert!(substitution.restore(context).is_err());

    let mut order = projected.clone();
    let order_current = order
        .in_progress_execution
        .as_deref_mut()
        .unwrap()
        .support_current
        .as_mut();
    let mut order_wire = wire;
    order_wire.event_handoffs[0]["ordinal"] = serde_json::Value::from(1_u32);
    install_resealed_publication_authority_poison(order_current, order_wire);
    order.seal().unwrap();
    assert!(order.restore(context).is_err());
}

fn install_resealed_coupled_clock_poison(
    value: &mut DirectSnowStage3V11AttachmentRestartV2,
    clock: serde_json::Value,
) {
    let bytes = serde_json::to_vec(&clock).unwrap();
    let current = value
        .in_progress_execution
        .as_deref_mut()
        .unwrap()
        .support_current
        .as_mut();
    current.coupled_clock_sha256 = sha256_hex(&bytes).unwrap();
    current.coupled_clock_canonical_base64 = STANDARD.encode(bytes);
    value.seal().unwrap();
}

fn assert_after_reappearance_deferred_publication_poisons(
    projected: &DirectSnowStage3V11AttachmentRestartV2,
    context: &ExpectedSnowStage3V11RestartContext<'_>,
) {
    let current = projected
        .in_progress_execution
        .as_deref()
        .unwrap()
        .support_current
        .as_ref();
    let bytes = STANDARD
        .decode(&current.coupled_clock_canonical_base64)
        .unwrap();
    let clock: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let events = clock["accepted_event_receipts"].as_array().unwrap();
    assert_eq!(events.len(), 1);

    let mut omission = projected.clone();
    let mut omission_clock = clock.clone();
    omission_clock["accepted_event_receipts"]
        .as_array_mut()
        .unwrap()
        .clear();
    install_resealed_coupled_clock_poison(&mut omission, omission_clock);
    assert!(omission.restore(context).is_err());

    let mut substitution = projected.clone();
    let mut substitution_clock = clock.clone();
    substitution_clock["accepted_event_receipts"][0]["begin_owner_set_sha256"] =
        serde_json::Value::String("11".repeat(32));
    install_resealed_coupled_clock_poison(&mut substitution, substitution_clock);
    assert!(substitution.restore(context).is_err());

    let mut order = projected.clone();
    let mut order_clock = clock.clone();
    order_clock["accepted_event_receipts"][0]["event_ordinal"] = serde_json::Value::from(1_u32);
    install_resealed_coupled_clock_poison(&mut order, order_clock);
    assert!(order.restore(context).is_err());

    let mut cross_parent = projected.clone();
    let mut cross_parent_clock = clock;
    cross_parent_clock["accepted_event_receipts"][0]["parent_transaction_id"] =
        serde_json::Value::String("22".repeat(32));
    install_resealed_coupled_clock_poison(&mut cross_parent, cross_parent_clock);
    assert!(cross_parent.restore(context).is_err());
}

fn assert_after_receiver_successor_poisons(
    projected: &DirectSnowStage3V11AttachmentRestartV2,
    context: &ExpectedSnowStage3V11RestartContext<'_>,
) {
    let mut wrong_posture = projected.clone();
    let metadata = STANDARD
        .decode(
            &wrong_posture
                .in_progress_execution
                .as_deref()
                .unwrap()
                .metadata_canonical_base64,
        )
        .unwrap();
    let mut metadata: serde_json::Value = serde_json::from_slice(&metadata).unwrap();
    metadata["posture"] = serde_json::Value::String("before_terminal_receiver".into());
    install_resealed_metadata_poison(&mut wrong_posture, metadata);
    assert!(wrong_posture.restore(context).is_err());

    let mut missing_event = projected.clone();
    let current = missing_event
        .in_progress_execution
        .as_deref_mut()
        .unwrap()
        .support_current
        .as_mut();
    let clock = STANDARD
        .decode(&current.coupled_clock_canonical_base64)
        .unwrap();
    let mut clock: serde_json::Value = serde_json::from_slice(&clock).unwrap();
    let events = clock["accepted_event_receipts"].as_array_mut().unwrap();
    assert!(events.pop().is_some());
    let clock = serde_json::to_vec(&clock).unwrap();
    current.coupled_clock_sha256 = sha256_hex(&clock).unwrap();
    current.coupled_clock_canonical_base64 = STANDARD.encode(clock);
    missing_event.seal().unwrap();
    assert!(missing_event.restore(context).is_err());

    let exact_surface = projected
        .in_progress_execution
        .as_deref()
        .unwrap()
        .support_current
        .real_consumer
        .scientific
        .direct_hydrology
        .surface_liquid_owned_state
        .as_deref()
        .unwrap()
        .restore()
        .unwrap();
    let vegetation_transaction = projected
        .in_progress_execution
        .as_deref()
        .unwrap()
        .support_current
        .real_consumer
        .scientific
        .vegetation_v10
        .last_transaction_id
        .to_u128();
    let plus_two_transaction =
        openwepp_kernel_contract::TransactionId(vegetation_transaction.checked_add(2).unwrap());
    let plus_two_surface = exact_surface
        .restart_authority_with_admission_lineage(
            context.real_consumer_context.surface_liquid_configuration,
            plus_two_transaction,
        )
        .unwrap();
    let mut plus_two = projected.clone();
    plus_two
        .in_progress_execution
        .as_deref_mut()
        .unwrap()
        .support_current
        .real_consumer
        .scientific
        .direct_hydrology
        .surface_liquid_owned_state = Some(Box::new(
        crate::DirectSurfaceLiquidOwnedStateRestartV1::project(&plus_two_surface).unwrap(),
    ));
    plus_two.seal().unwrap();
    assert!(plus_two.restore(context).is_err());

    let mut mixed_record = projected.clone();
    let surface = mixed_record
        .in_progress_execution
        .as_deref_mut()
        .unwrap()
        .support_current
        .real_consumer
        .scientific
        .direct_hydrology
        .surface_liquid_owned_state
        .as_deref_mut()
        .unwrap();
    assert!(surface.records.len() > 1);
    surface.records[0].last_accepted_transaction_id =
        Some(HexU128::from_u128(vegetation_transaction));
    mixed_record.seal().unwrap();
    assert!(mixed_record.restore(context).is_err());
}

fn install_resealed_metadata_poison(
    value: &mut DirectSnowStage3V11AttachmentRestartV2,
    metadata: serde_json::Value,
) {
    let bytes = serde_json::to_vec(&metadata).unwrap();
    let in_progress = value.in_progress_execution.as_deref_mut().unwrap();
    in_progress.metadata_sha256 = sha256_hex(&bytes).unwrap();
    in_progress.metadata_canonical_base64 = STANDARD.encode(bytes);
    value.seal().unwrap();
}

fn assert_in_progress_owner_restart_poisons(
    projected: &DirectSnowStage3V11AttachmentRestartV2,
    context: &ExpectedSnowStage3V11RestartContext<'_>,
) {
    let mut cursor_omission: serde_json::Value =
        serde_json::from_slice(&projected.to_canonical_bytes().unwrap()).unwrap();
    cursor_omission["in_progress_execution"]["support_current"]
        .as_object_mut()
        .unwrap()
        .remove("real_consumer_provider_cursor_configuration_bound");
    assert!(
        DirectSnowStage3V11AttachmentRestartV2::from_canonical_bytes(
            &serde_json::to_vec(&cursor_omission).unwrap(),
            context,
        )
        .is_err()
    );

    let mut cursor_substitution = projected.clone();
    let cursor_current = cursor_substitution
        .in_progress_execution
        .as_deref_mut()
        .unwrap()
        .support_current
        .as_mut();
    cursor_current.real_consumer_provider_cursor_configuration_bound =
        !cursor_current.real_consumer_provider_cursor_configuration_bound;
    cursor_substitution.seal().unwrap();
    assert!(cursor_substitution.restore(context).is_err());

    let mut wb14_omission = projected.clone();
    let omission_current = wb14_omission
        .in_progress_execution
        .as_deref_mut()
        .unwrap()
        .support_current
        .as_mut();
    assert!(
        omission_current
            .real_consumer_wb14_parent_canonical_base64
            .take()
            .is_some()
    );
    omission_current.real_consumer_wb14_parent_sha256 = None;
    wb14_omission.seal().unwrap();
    assert!(wb14_omission.restore(context).is_err());

    let mut wb14_substitution = projected.clone();
    let substitution_current = wb14_substitution
        .in_progress_execution
        .as_deref_mut()
        .unwrap()
        .support_current
        .as_mut();
    let mut substitution_bytes = STANDARD
        .decode(
            substitution_current
                .real_consumer_wb14_parent_canonical_base64
                .as_deref()
                .unwrap(),
        )
        .unwrap();
    let last = substitution_bytes.last_mut().unwrap();
    *last ^= 1;
    substitution_current.real_consumer_wb14_parent_sha256 =
        Some(sha256_hex(&substitution_bytes).unwrap());
    substitution_current.real_consumer_wb14_parent_canonical_base64 =
        Some(STANDARD.encode(substitution_bytes));
    wb14_substitution.seal().unwrap();
    assert!(wb14_substitution.restore(context).is_err());

    let mut wb14_order = projected.clone();
    let order_current = wb14_order
        .in_progress_execution
        .as_deref_mut()
        .unwrap()
        .support_current
        .as_mut();
    let canonical = STANDARD
        .decode(
            order_current
                .real_consumer_wb14_parent_canonical_base64
                .as_deref()
                .unwrap(),
        )
        .unwrap();
    let value: serde_json::Value = serde_json::from_slice(&canonical).unwrap();
    let object = value.as_object().unwrap();
    let reordered = format!(
        "{{{}}}",
        object
            .iter()
            .rev()
            .map(|(key, value)| format!(
                "{}:{}",
                serde_json::to_string(key).unwrap(),
                serde_json::to_string(value).unwrap()
            ))
            .collect::<Vec<_>>()
            .join(",")
    )
    .into_bytes();
    assert_ne!(reordered, canonical);
    order_current.real_consumer_wb14_parent_sha256 = Some(sha256_hex(&reordered).unwrap());
    order_current.real_consumer_wb14_parent_canonical_base64 = Some(STANDARD.encode(reordered));
    wb14_order.seal().unwrap();
    assert!(wb14_order.restore(context).is_err());

    let metadata_bytes = STANDARD
        .decode(
            &projected
                .in_progress_execution
                .as_deref()
                .unwrap()
                .metadata_canonical_base64,
        )
        .unwrap();
    let metadata: serde_json::Value = serde_json::from_slice(&metadata_bytes).unwrap();
    let rows = metadata["support_owner_joins"][0]["destination_receipts"]
        .as_array()
        .unwrap();
    assert!(rows.len() >= 2);

    let mut destination_omission = projected.clone();
    let mut omission_metadata = metadata.clone();
    omission_metadata["support_owner_joins"][0]["destination_receipts"]
        .as_array_mut()
        .unwrap()
        .remove(0);
    install_resealed_metadata_poison(&mut destination_omission, omission_metadata);
    assert!(destination_omission.restore(context).is_err());

    let mut destination_substitution = projected.clone();
    let mut substitution_metadata = metadata.clone();
    substitution_metadata["support_owner_joins"][0]["destination_receipts"][0]["tile_id"] =
        serde_json::Value::String("substituted-tile".into());
    install_resealed_metadata_poison(&mut destination_substitution, substitution_metadata);
    assert!(destination_substitution.restore(context).is_err());

    let mut destination_order = projected.clone();
    let mut order_metadata = metadata;
    order_metadata["support_owner_joins"][0]["destination_receipts"]
        .as_array_mut()
        .unwrap()
        .reverse();
    install_resealed_metadata_poison(&mut destination_order, order_metadata);
    assert!(destination_order.restore(context).is_err());
}

fn active_snow_inputs(mode: AdaptiveRestartFixtureMode) -> DirectActiveSnowPartitionInputs {
    // Match the authoritative mixed open/covered WB14 fixture. Its small,
    // cold pack is known to remain inside the complete-owner constitutive
    // domain while still exercising adaptive subdivision.
    let mut layer = DirectSnowLayerState::new(0.005, 0.05, 100.0, 12.0);
    layer.temperature_c = -8.0;
    layer.cold_content_j_m2 = 0.005 * 1_000.0 * 2_100.0 * 8.0;
    let mut inputs = DirectActiveSnowPartitionInputs {
        hyetograph_rainfall_m: 0.0,
        rst_c: 0.0,
        newsnw_kg_m3: 100.0,
        ssd_kg_m3: 522.0,
        runtime_swe_m: 0.005,
        runtime_depth_m: 0.05,
        runtime_density_kg_m3: 100.0,
        runtime_settle_day_count: 12.0,
        liquid_water_retained_m: 0.0,
        tmax_c: -3.0,
        tmin_c: -7.0,
        canopy_cover_fraction: 0.0,
        wind_m_s: 3.0,
        dewpoint_c: -15.0,
        snow_melt_model: SnowMeltModel::AdaptiveCompositionalStage3V1,
        snow_density_model: SnowDensityModel::PhysicsBulkDensityCompactionV1,
        stage3_liquid_routing_model: SnowStage3LiquidRoutingModel::LayeredThermalLiquidV1,
        surface_energy_options: DirectSnowSurfaceEnergyOptions::default(),
        sturm_climate_class: None,
        sturm_day_of_year: None,
        coe_boundary_depth_m: 0.05,
        coe_boundary_density_kg_m3: 100.0,
        coe_boundary_settle_day_count: 12.0,
        snow_albedo_model: None,
        snow_albedo_state: None,
        snow_layers: vec![layer],
        underlying_surface_albedo: 0.2,
        hourly: [DirectSnowHourlyForcing::zero(); 24],
    };
    inputs.surface_energy_options.sublimation_model =
        SnowSurfaceSublimationModel::NeutralBulkStage3V1;
    match mode {
        AdaptiveRestartFixtureMode::CrossMidnight => {}
        AdaptiveRestartFixtureMode::Terminal => {
            inputs.runtime_swe_m = 0.001_2;
            inputs.runtime_depth_m = 0.012;
            inputs.runtime_density_kg_m3 = 100.0;
            inputs.snow_layers[0].mass_swe_m = 0.001_2;
            inputs.snow_layers[0].thickness_m = 0.012;
            inputs.snow_layers[0].density_kg_m3 = 100.0;
            inputs.snow_layers[0].temperature_c = 0.0;
            inputs.snow_layers[0].cold_content_j_m2 = 0.0;
        }
        AdaptiveRestartFixtureMode::PositiveSupportLiquidCustody => {
            // Keep a warm, nonterminal pack resident while a positive
            // support produces liquid. This gives restart V3 a genuine
            // support-liquid custody supplemental to poison; the terminal
            // fixture's liquid is correctly handled by its terminal
            // receiver and must not be double-installed here.
            inputs.runtime_swe_m = 0.01;
            inputs.runtime_depth_m = 0.04;
            inputs.runtime_density_kg_m3 = 250.0;
            inputs.snow_layers[0].mass_swe_m = 0.01;
            inputs.snow_layers[0].thickness_m = 0.04;
            inputs.snow_layers[0].density_kg_m3 = 250.0;
            inputs.snow_layers[0].temperature_c = 0.0;
            inputs.snow_layers[0].cold_content_j_m2 = 0.0;
            inputs.tmax_c = 3.0;
            inputs.tmin_c = 1.0;
        }
        AdaptiveRestartFixtureMode::Reappearance => {
            inputs.runtime_swe_m = 0.0;
            inputs.runtime_depth_m = 0.0;
            inputs.runtime_density_kg_m3 = 0.0;
            inputs.snow_layers.clear();
        }
    }
    inputs
}

fn adaptive_snow_state(
    lane_id: u32,
    mode: AdaptiveRestartFixtureMode,
) -> DirectSnowStage3PersistentState {
    let inputs = active_snow_inputs(mode);
    match mode {
        AdaptiveRestartFixtureMode::Terminal => {
            Wb11HydrologyKernel::initialize_stage3_persistent_state_with_terminal_event(
                lane_id,
                inputs.snow_layers,
                DirectSnowTerminalEventRequest::ENTHALPY_EVENT_V1,
            )
        }
        AdaptiveRestartFixtureMode::PositiveSupportLiquidCustody
        | AdaptiveRestartFixtureMode::Reappearance
        | AdaptiveRestartFixtureMode::CrossMidnight => {
            Wb11HydrologyKernel::initialize_stage3_persistent_state(lane_id, inputs.snow_layers)
        }
    }
    .unwrap()
}

fn adaptive_prepared_day(
    fixture: &crate::RestartAuthorityPreparedDayFixture,
    lane_id: u32,
    mode: AdaptiveRestartFixtureMode,
) -> openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::ValidatedPreparedStage3V11DayV1 {
    let covered_tiles = fixture
        .owners
        .runtime
        .shadow
        .vegetation_configuration()
        .strata
        .iter()
        .flat_map(|stratum| stratum.tile_ids.iter().cloned())
        .collect::<std::collections::BTreeSet<_>>();
    let supports = fixture
        .template
        .intervals
        .iter()
        .cloned()
        .enumerate()
        .map(|(index, mut snow_free)| {
            let provider = &fixture.prepared.forcing_receipts().receipts()[0].intervals[index];
            snow_free.lse_forcing.air_temperature_k = provider.air_temperature_c + 273.15;
            snow_free.lse_forcing.air_specific_humidity_kg_kg = provider.specific_humidity_kg_kg;
            snow_free.lse_forcing.air_pressure_pa = provider.pressure_kpa * 1_000.0;
            snow_free.lse_forcing.reference_wind_m_s = provider.wind_m_s;
            snow_free.lse_forcing.direct_vis_w_m2 = provider.direct_visible_w_m2;
            snow_free.lse_forcing.diffuse_vis_w_m2 = provider.diffuse_visible_w_m2;
            snow_free.lse_forcing.direct_nir_w_m2 = provider.direct_nir_w_m2;
            snow_free.lse_forcing.diffuse_nir_w_m2 = provider.diffuse_nir_w_m2;
            snow_free.lse_forcing.atmospheric_downward_longwave_w_m2 =
                provider.downward_longwave_w_m2;
            snow_free.lse_forcing.precipitation_parcels.clear();
            snow_free.lse_forcing.runon_parcels.clear();
            snow_free.lse_forcing.snow_present_at_beginning = false;
            snow_free.lse_forcing.snow_present_at_end = false;
            snow_free.lse_forcing.snow_terminal_payload_present = false;
            snow_free.lse_forcing.forcing_sha256 =
                snow_free.lse_forcing.canonical_sha256().unwrap();
            snow_free.vegetation_forcing.air_temperature_k =
                snow_free.lse_forcing.air_temperature_k;
            snow_free.vegetation_forcing.pressure_pa = snow_free.lse_forcing.air_pressure_pa;
            snow_free.vegetation_forcing.wind_m_s = provider.wind_m_s;
            snow_free.vegetation_forcing.specific_humidity = provider.specific_humidity_kg_kg;
            snow_free.vegetation_forcing.direct_par_w_m2 = provider.direct_visible_w_m2;
            snow_free.vegetation_forcing.diffuse_par_w_m2 = provider.diffuse_visible_w_m2;
            snow_free.vegetation_forcing.direct_nir_w_m2 = provider.direct_nir_w_m2;
            snow_free.vegetation_forcing.diffuse_nir_w_m2 = provider.diffuse_nir_w_m2;
            snow_free.vegetation_forcing.longwave_down_w_m2 = provider.downward_longwave_w_m2;
            snow_free.vegetation_forcing.gsi = provider.gsi;
            let mut covered_lse = snow_free.lse_forcing.clone();
            covered_lse.snow_present_at_end = true;
            covered_lse.forcing_sha256 = covered_lse.canonical_sha256().unwrap();
            let covered = DirectV11SnowCoveredSegmentInput::try_new(
                covered_lse,
                snow_free.vegetation_forcing.clone(),
                snow_free.wb14_parameters.clone(),
            )
            .unwrap();
            let support = TimeSupport::new(
                ModelTimeNs::new(index as u128 * 1_800_000_000_000),
                ModelTimeNs::new((index as u128 + 1) * 1_800_000_000_000),
            )
            .unwrap();
            let identities = fixture
                .prepared
                .forcing_receipts()
                .receipts()
                .iter()
                .map(|day| {
                    let interval = &day.intervals[index];
                    let tile = TileId::try_new(interval.tile_id.clone()).unwrap();
                    if covered_tiles.contains(&tile) {
                        let forcing = SealedCoveredCarrierForcing::try_from_repository_interval(
                            interval, 1.2, 1_005.0, 0.5,
                        )
                        .unwrap();
                        PreparedStage3V11SupportIdentityV1::from_provider_covered_interval(
                            interval, &forcing,
                        )
                        .unwrap()
                    } else {
                        PreparedStage3V11SupportIdentityV1::from_provider_open_interval(
                            support, interval,
                        )
                        .unwrap()
                    }
                })
                .collect();
            let destination_capabilities = fixture
                .prepared
                .forcing_receipts()
                .receipts()
                .iter()
                .map(|day| {
                    let interval = &day.intervals[index];
                    (
                        (
                            OfeId::try_new(interval.ofe_id.clone()).unwrap(),
                            TileId::try_new(interval.tile_id.clone()).unwrap(),
                        ),
                        if covered_tiles
                            .contains(&TileId::try_new(interval.tile_id.clone()).unwrap())
                        {
                            DirectSnowStage3V11DestinationCapabilityV1::CanopyCovered(
                                SealedCoveredCarrierForcing::try_from_repository_interval(
                                    interval, 1.2, 1_005.0, 0.5,
                                )
                                .unwrap(),
                            )
                        } else {
                            DirectSnowStage3V11DestinationCapabilityV1::OpenProviderProjection
                        },
                    )
                })
                .collect();
            let mut inputs = active_snow_inputs(mode);
            inputs.wind_m_s = provider.wind_m_s;
            inputs.dewpoint_c = provider.dew_point_c;
            inputs.surface_energy_options.atmospheric_pressure_pa = provider.pressure_kpa * 1_000.0;
            let prepared = DirectSnowStage3V11PreparedSupport::from_dual_regime_production_inputs(
                support,
                DirectSnowStage3V11DualRegimeSupportInputsV1 {
                    snow_inputs_by_lane: BTreeMap::from([(lane_id, inputs)]),
                    support_forcing_by_lane: BTreeMap::from([(
                        lane_id,
                        DirectSnowStage3SupportInput {
                            forcing: DirectSnowHourlyForcing {
                                active_precipitation_m: provider.active_precipitation_m,
                                rain_m: provider.rain_m,
                                snowfall_m: provider.snowfall_m,
                                air_temperature_c: provider.air_temperature_c,
                                cloud_fraction: provider.cloud_fraction,
                                rain_fraction: provider.rain_fraction,
                                snow_fraction: provider.snow_fraction,
                                hydrometeor_temperature_c: provider.hydrometeor_temperature_c,
                                radiation_mj_m2: match mode {
                                    AdaptiveRestartFixtureMode::Terminal => 1_000.0,
                                    AdaptiveRestartFixtureMode::PositiveSupportLiquidCustody => 1.0,
                                    AdaptiveRestartFixtureMode::Reappearance
                                    | AdaptiveRestartFixtureMode::CrossMidnight => 0.0,
                                },
                                ..DirectSnowHourlyForcing::zero()
                            },
                            duration_seconds: 1_800.0,
                        },
                    )]),
                    snow_free_v11_interval: snow_free,
                    snow_surface_v11_interval: covered,
                    support_identity_by_lane: BTreeMap::from([(lane_id, identities)]),
                    destination_capabilities,
                    hard_boundaries: Vec::new(),
                },
            )
            .unwrap();
            if matches!(mode, AdaptiveRestartFixtureMode::Terminal) && index == 0 {
                prepared
                    .restart_authority_with_open_snow_shortwave_multiplier_for_fixture(8.041_1)
                    .unwrap()
            } else {
                prepared
            }
        })
        .collect();
    PreparedStage3V11DayV1::bind_production_provider_day(&fixture.prepared, 0, supports).unwrap()
}

fn exercise_adaptive_restart_posture(
    mode: AdaptiveRestartFixtureMode,
    posture: DirectSnowStage3V11InterruptionPostureV2,
) {
    std::thread::Builder::new()
        .name(format!("stage3-v11-restart-{posture:?}"))
        .stack_size(64 * 1024 * 1024)
        .spawn(move || exercise_adaptive_restart_posture_inner(mode, posture))
        .unwrap()
        .join()
        .unwrap();
}

fn exercise_adaptive_restart_posture_inner(
    mode: AdaptiveRestartFixtureMode,
    posture: DirectSnowStage3V11InterruptionPostureV2,
) {
    let mut fixture = match mode {
        AdaptiveRestartFixtureMode::Terminal => restart_authority_adaptive_prepared_day_fixture(),
        AdaptiveRestartFixtureMode::PositiveSupportLiquidCustody => {
            restart_authority_adaptive_prepared_day_fixture()
        }
        AdaptiveRestartFixtureMode::Reappearance => {
            restart_authority_adaptive_snow_prepared_day_fixture()
        }
        AdaptiveRestartFixtureMode::CrossMidnight => {
            restart_authority_adaptive_cross_midnight_carry_fixture()
        }
    };
    // Every restart posture starts from the same fully resealed complete
    // owner authority. Cross-midnight precipitation/cursor carry remains
    // independently bound by the provider receipts; it is not a reason to
    // retain the source fixture's stale soil-thermal nested digests.
    openwepp_hillslope_orchestrator::v9_real_consumer_shadow::restart_authority_equilibrate_complete_owner_fixture(
            &mut fixture.owners.runtime.shadow,
        )
        .unwrap();
    if matches!(
        mode,
        AdaptiveRestartFixtureMode::PositiveSupportLiquidCustody
    ) {
        // A staged day ends at next_day_index=1. Give the isolated restart
        // host a real day-1 continuation instead of weakening its exact
        // next-day/day-count admission guard.
        install_nonzero_archive_continuation_context(&mut fixture);
    }
    if matches!(mode, AdaptiveRestartFixtureMode::CrossMidnight) {
        assert!(
            fixture
                .forcing_receipts
                .iter()
                .any(|receipt| !receipt.next_day_precipitation_carry.is_empty())
        );
    }
    let surface_configuration = fixture
        .owners
        .runtime
        .shadow
        .restart_authority_surface_configuration()
        .clone();
    let lane_id = surface_configuration.ofe_bindings[0].production_lane_id;
    let prepared = adaptive_prepared_day(&fixture, lane_id, mode);
    let production_configuration = DirectSnowStage3V11ProductionConfigurationV1 {
        run_identity: digest_bytes(b"stage3-v11-adaptive-restart-run"),
        topology_identity: digest_bytes(b"stage3-v11-adaptive-restart-topology"),
        calendar_receipt: digest_bytes(b"stage3-v11-adaptive-restart-calendar"),
        controller_policy: digest_bytes(b"stage3-v11-adaptive-restart-controller"),
        surface_liquid_configuration: surface_configuration,
        wb14_parameters: vec![DirectOfeWb14Parameters {
            ofe_id: OfeId::try_new("ofe-1").unwrap(),
            effective_conductivity_m_s: 1.0e-6,
            matric_potential_m: 0.1,
            infiltration_storage_capacity_m: 0.04,
        }],
    };
    let attachment = DirectSnowStage3V11ShadowAttachment::new_production(
        production_configuration,
        BTreeMap::from([(lane_id, adaptive_snow_state(lane_id, mode))]),
        fixture.owners.runtime.shadow.clone(),
    )
    .unwrap();
    let commit_target = attachment.clone();
    let mut interrupted = attachment.clone();
    if matches!(
        mode,
        AdaptiveRestartFixtureMode::PositiveSupportLiquidCustody
    ) {
        interrupted.stage_prepared_day(&prepared).unwrap();
    } else {
        assert!(
            !interrupted
                .stage_prepared_day_until_posture_v2(&prepared, posture)
                .unwrap()
        );
        assert_eq!(
            interrupted
                .restart_authority_in_progress_execution_v2()
                .unwrap()
                .posture(),
            posture
        );
        if matches!(
            posture,
            DirectSnowStage3V11InterruptionPostureV2::AdaptiveMicrostepBoundary
        ) {
            let current = interrupted
                .restart_authority_in_progress_execution_v2()
                .unwrap()
                .support_current()
                .unwrap();
            let parent = current
                .real_consumer
                .restart_authority_wb14_parent_canonical_bytes()
                .unwrap();
            let represented_snow = current.stage3_by_lane.values().any(|state| {
                openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::restart_authority_stage3_has_represented_ice_v2(state)
            });
            let parent_complete = current.coupled_clock.accepted_until()
                == current.coupled_clock.parent_support().end_ns();
            if !represented_snow {
                assert_eq!(
                    parent.is_some(),
                    !parent_complete,
                    "snow-free adaptive boundary WB14 parent follows final posture"
                );
            }
        }
    }
    let (run, topology) = checkpoint_identities_v1(
        &fixture.owners.committed,
        fixture
            .owners
            .runtime
            .shadow
            .root_zone_hydraulic_configuration(),
    )
    .unwrap();
    let real_consumer_context = ExpectedRestartStaticContext {
        run_identity_sha256: &run,
        topology_sha256: &topology,
        vegetation_configuration: fixture
            .owners
            .runtime
            .shadow
            .restart_authority_vegetation_configuration(),
        vegetation_owner_id: fixture
            .owners
            .runtime
            .shadow
            .restart_authority_vegetation_owner_id(),
        soil_thermal_owner_id: &fixture
            .owners
            .runtime
            .shadow
            .restart_authority_soil_thermal()
            .expect("V1 Stage-3 fixture soil resident")
            .owner_id,
        soil_thermal_configuration_sha256: &fixture
            .owners
            .runtime
            .shadow
            .restart_authority_soil_thermal()
            .expect("V1 Stage-3 fixture soil resident")
            .configuration_sha256,
        lse_configuration: fixture
            .owners
            .runtime
            .shadow
            .restart_authority_lse_configuration(),
        surface_liquid_configuration: fixture
            .owners
            .runtime
            .shadow
            .restart_authority_surface_configuration(),
        gsi_configuration: fixture.owners.runtime.shadow.gsi_owner_configuration(),
        forcing_static_configuration: fixture
            .owners
            .runtime
            .shadow
            .provider_static_configuration(),
        root_zone_hydraulic_configuration: fixture
            .owners
            .runtime
            .shadow
            .root_zone_hydraulic_configuration(),
        phase_plan: &fixture
            .owners
            .runtime
            .shadow
            .restart_authority_hydrology_frame()
            .phase_plan,
        phase_plan_sha256: &fixture.owners.phase_plan_sha256,
        day_inputs: &fixture.owners.day_inputs,
        day_input_digests: &fixture.owners.day_input_digests,
    };
    let restart_context = ExpectedSnowStage3V11RestartContext {
        static_context: &interrupted.static_context,
        real_consumer_context: &real_consumer_context,
    };
    if matches!(
        mode,
        AdaptiveRestartFixtureMode::PositiveSupportLiquidCustody
    ) {
        let (restored, _) = restore_empty_archive_v3(
            &interrupted,
            &restart_context,
            &fixture.owners.phase_plan_sha256,
            &fixture.owners.day_input_digests,
        );
        assert!(
            restored.static_context == interrupted.static_context,
            "custody fixture restored static context"
        );
        assert!(
            restored.committed == interrupted.committed,
            "custody fixture restored committed state"
        );
        assert_pending_candidate_authority_equal(
            restored.restart_authority_pending_candidate().unwrap(),
            interrupted.restart_authority_pending_candidate().unwrap(),
            &fixture.owners.phase_plan_sha256,
            &fixture.owners.day_input_digests,
        );
        assert!(
            project_empty_archive_v3_bytes(
                &restored,
                &fixture.owners.phase_plan_sha256,
                &fixture.owners.day_input_digests,
            ) == project_empty_archive_v3_bytes(
                &interrupted,
                &fixture.owners.phase_plan_sha256,
                &fixture.owners.day_input_digests,
            ),
            "custody fixture canonical V3 bytes"
        );
        assert_empty_archive_v3_support_liquid_custody_poisons(
            &interrupted,
            &restart_context,
            &fixture.owners.phase_plan_sha256,
            &fixture.owners.day_input_digests,
        );
        return;
    }
    if matches!(mode, AdaptiveRestartFixtureMode::Terminal)
        && matches!(
            posture,
            DirectSnowStage3V11InterruptionPostureV2::AfterTerminalReceiver
        )
    {
        assert_empty_archive_v3_terminal_liquid_custody_poisons(
            &interrupted,
            &restart_context,
            &fixture.owners.phase_plan_sha256,
            &fixture.owners.day_input_digests,
        );
    }
    if matches!(mode, AdaptiveRestartFixtureMode::Terminal)
        && matches!(
            posture,
            DirectSnowStage3V11InterruptionPostureV2::AdaptiveMicrostepBoundary
        )
    {
        assert_nonzero_archive_v3_stable_fixture();
    }
    let (restored_v3, _bytes_v3) = restore_empty_archive_v3(
        &interrupted,
        &restart_context,
        &fixture.owners.phase_plan_sha256,
        &fixture.owners.day_input_digests,
    );
    if let Ok(projected_v2) = DirectSnowStage3V11AttachmentRestartV2::project(
        &interrupted,
        &fixture.owners.phase_plan_sha256,
        &fixture.owners.day_input_digests,
    ) {
        let bytes_v2 = projected_v2.to_canonical_bytes().unwrap();
        let decoded_v2 = DirectSnowStage3V11AttachmentRestartV2::from_canonical_bytes(
            &bytes_v2,
            &restart_context,
        )
        .unwrap();
        decoded_v2.restore(&restart_context).unwrap();
    }
    let mut restored = restored_v3;
    assert!(
        restored.static_context == interrupted.static_context,
        "restored static context"
    );
    assert!(
        restored.committed == interrupted.committed,
        "restored committed state"
    );
    assert!(
        restored.restart_authority_pending_candidate()
            == interrupted.restart_authority_pending_candidate(),
        "restored pending state"
    );
    let restored_in_progress = restored
        .restart_authority_in_progress_execution_v2()
        .unwrap();
    let interrupted_in_progress = interrupted
        .restart_authority_in_progress_execution_v2()
        .unwrap();
    let restored_day = restored_in_progress.day_candidate();
    let interrupted_day = interrupted_in_progress.day_candidate();
    assert!(
        restored_day.stage3_by_lane == interrupted_day.stage3_by_lane,
        "restored in-progress day Stage-3 state"
    );
    let restored_day_consumer = &restored_day.real_consumer;
    let interrupted_day_consumer = &interrupted_day.real_consumer;
    assert!(
        restored_day_consumer.vegetation_state() == interrupted_day_consumer.vegetation_state(),
        "restored in-progress day vegetation"
    );
    assert!(
        restored_day_consumer.lse_state() == interrupted_day_consumer.lse_state(),
        "restored in-progress day LSE"
    );
    assert!(
        restored_day_consumer.restart_authority_hydrology_frame()
            == interrupted_day_consumer.restart_authority_hydrology_frame(),
        "restored in-progress day hydrology"
    );
    assert!(
        restored_day_consumer
            .restart_authority_soil_thermal()
            .expect("restored V1 day soil resident")
            == interrupted_day_consumer
                .restart_authority_soil_thermal()
                .expect("interrupted V1 day soil resident"),
        "restored in-progress day soil"
    );
    assert!(
        restored_day_consumer.restart_authority_biogeochemistry()
            == interrupted_day_consumer.restart_authority_biogeochemistry(),
        "restored in-progress day BGC"
    );
    assert!(
        restored_day_consumer.gsi_state() == interrupted_day_consumer.gsi_state()
            && restored_day_consumer.provider_cursor()
                == interrupted_day_consumer.provider_cursor()
            && restored_day_consumer.v11_next_day_index()
                == interrupted_day_consumer.v11_next_day_index()
            && restored_day_consumer.restart_authority_accepted_interval_count()
                == interrupted_day_consumer.restart_authority_accepted_interval_count(),
        "restored in-progress day scheduler owners"
    );
    assert_eq!(
        restored_day_consumer
            .restart_authority_wb14_parent_canonical_bytes()
            .unwrap(),
        interrupted_day_consumer
            .restart_authority_wb14_parent_canonical_bytes()
            .unwrap(),
        "restored in-progress day WB14 parent"
    );
    assert_eq!(
        restored_day_consumer
            .restart_authority_accepted_publication_supports_canonical_bytes()
            .unwrap(),
        interrupted_day_consumer
            .restart_authority_accepted_publication_supports_canonical_bytes()
            .unwrap(),
        "restored in-progress day publication authority"
    );
    assert!(
        restored_day_consumer
            .restart_authority_vegetation_configuration()
            .configuration_sha256
            == interrupted_day_consumer
                .restart_authority_vegetation_configuration()
                .configuration_sha256
            && restored_day_consumer.restart_authority_lse_configuration()
                == interrupted_day_consumer.restart_authority_lse_configuration()
            && restored_day_consumer.restart_authority_surface_configuration()
                == interrupted_day_consumer.restart_authority_surface_configuration()
            && restored_day_consumer.gsi_owner_configuration()
                == interrupted_day_consumer.gsi_owner_configuration()
            && restored_day_consumer.provider_static_configuration()
                == interrupted_day_consumer.provider_static_configuration()
            && restored_day_consumer.root_zone_hydraulic_configuration()
                == interrupted_day_consumer.root_zone_hydraulic_configuration(),
        "restored in-progress day canonical configurations"
    );
    assert_eq!(
        restored_day.v11_parent_state.checkpoint(),
        interrupted_day.v11_parent_state.checkpoint(),
        "restored in-progress day V11 parent"
    );
    assert!(
        restored_day.coupled_clock == interrupted_day.coupled_clock,
        "restored in-progress day clock"
    );
    assert!(
        restored_day.next_parent_sequence == interrupted_day.next_parent_sequence,
        "restored in-progress day sequence"
    );
    assert_last_v11_candidate_authority_equal(
        restored_day.last_v11_parent_candidate.as_ref(),
        interrupted_day.last_v11_parent_candidate.as_ref(),
        "restored in-progress day last V11 candidate authority",
    );
    assert!(
        restored_day.terminal_parcels == interrupted_day.terminal_parcels
            && restored_day.receipt_chain == interrupted_day.receipt_chain,
        "restored in-progress day receipts"
    );
    let restored_current = restored_in_progress.support_current().unwrap();
    let interrupted_current = interrupted_in_progress.support_current().unwrap();
    assert!(
        restored_current.stage3_by_lane == interrupted_current.stage3_by_lane,
        "restored support Stage-3 state"
    );
    let restored_consumer = &restored_current.real_consumer;
    let interrupted_consumer = &interrupted_current.real_consumer;
    assert!(
        restored_consumer.vegetation_state() == interrupted_consumer.vegetation_state(),
        "restored support vegetation"
    );
    assert!(
        restored_consumer.lse_state() == interrupted_consumer.lse_state(),
        "restored support LSE"
    );
    assert!(
        restored_consumer.restart_authority_hydrology_frame()
            == interrupted_consumer.restart_authority_hydrology_frame(),
        "restored support hydrology"
    );
    assert!(
        restored_consumer
            .restart_authority_soil_thermal()
            .expect("restored V1 support soil resident")
            == interrupted_consumer
                .restart_authority_soil_thermal()
                .expect("interrupted V1 support soil resident"),
        "restored support soil"
    );
    assert!(
        restored_consumer.restart_authority_biogeochemistry()
            == interrupted_consumer.restart_authority_biogeochemistry(),
        "restored support BGC"
    );
    assert!(
        restored_consumer.gsi_state() == interrupted_consumer.gsi_state()
            && restored_consumer.provider_cursor() == interrupted_consumer.provider_cursor()
            && restored_consumer.v11_next_day_index() == interrupted_consumer.v11_next_day_index()
            && restored_consumer.restart_authority_accepted_interval_count()
                == interrupted_consumer.restart_authority_accepted_interval_count(),
        "restored support scheduler owners"
    );
    assert_eq!(
        restored_consumer
            .restart_authority_wb14_parent_canonical_bytes()
            .unwrap(),
        interrupted_consumer
            .restart_authority_wb14_parent_canonical_bytes()
            .unwrap(),
        "restored support WB14 parent"
    );
    assert_eq!(
        restored_consumer
            .restart_authority_accepted_publication_supports_canonical_bytes()
            .unwrap(),
        interrupted_consumer
            .restart_authority_accepted_publication_supports_canonical_bytes()
            .unwrap(),
        "restored support publication authority"
    );
    assert_eq!(
        restored_current.v11_parent_state.checkpoint(),
        interrupted_current.v11_parent_state.checkpoint(),
        "restored support V11 parent checkpoint"
    );
    assert!(
        restored_current.coupled_clock == interrupted_current.coupled_clock,
        "restored support coupled clock"
    );
    assert_last_v11_candidate_authority_equal(
        restored_current.last_v11_parent_candidate.as_ref(),
        interrupted_current.last_v11_parent_candidate.as_ref(),
        "restored support last V11 candidate authority",
    );
    assert!(
        restored_current.next_parent_sequence == interrupted_current.next_parent_sequence
            && restored_current.terminal_parcels == interrupted_current.terminal_parcels
            && restored_current.receipt_chain == interrupted_current.receipt_chain,
        "restored support remaining state"
    );
    assert_eq!(
        restart_authority_encode_in_progress_metadata_base_v3(restored_in_progress).unwrap(),
        restart_authority_encode_in_progress_metadata_base_v3(interrupted_in_progress).unwrap(),
        "restored in-progress metadata"
    );
    if matches!(
        posture,
        DirectSnowStage3V11InterruptionPostureV2::AdaptiveMicrostepBoundary
    ) {
        let rollback = restored.clone();
        let wrong_fixture = restart_authority_adaptive_snow_prepared_day_fixture();
        let wrong_prepared = adaptive_prepared_day(
            &wrong_fixture,
            lane_id,
            AdaptiveRestartFixtureMode::Reappearance,
        );
        assert!(
            restored
                .finish_in_progress_prepared_day_v2(&wrong_prepared)
                .is_err()
        );
        assert_eq!(restored, rollback);
    }
    let mut uninterrupted = attachment;
    uninterrupted.stage_prepared_day(&prepared).unwrap();
    restored
        .finish_in_progress_prepared_day_v2(&prepared)
        .unwrap();
    let restored_candidate = restored.restart_authority_pending_candidate().unwrap();
    let uninterrupted_candidate = uninterrupted.restart_authority_pending_candidate().unwrap();
    assert_pending_candidate_authority_equal(
        restored_candidate,
        uninterrupted_candidate,
        &fixture.owners.phase_plan_sha256,
        &fixture.owners.day_input_digests,
    );
    assert_eq!(
        project_empty_archive_v3_bytes(
            &restored,
            &fixture.owners.phase_plan_sha256,
            &fixture.owners.day_input_digests,
        ),
        project_empty_archive_v3_bytes(
            &uninterrupted,
            &fixture.owners.phase_plan_sha256,
            &fixture.owners.day_input_digests,
        ),
        "V3 posture replay must preserve archive root/count and active outputs",
    );
    let mut restored_committed = commit_target.clone();
    restored_committed
        .install_candidate(restored_candidate.clone())
        .unwrap();
    let mut uninterrupted_committed = commit_target;
    uninterrupted_committed
        .install_candidate(uninterrupted_candidate.clone())
        .unwrap();
    assert_eq!(
        project_empty_archive_v3_bytes(
            &restored_committed,
            &fixture.owners.phase_plan_sha256,
            &fixture.owners.day_input_digests,
        ),
        project_empty_archive_v3_bytes(
            &uninterrupted_committed,
            &fixture.owners.phase_plan_sha256,
            &fixture.owners.day_input_digests,
        ),
        "committed ending owners and receipts must be byte-identical",
    );
}

#[test]
fn adaptive_microstep_boundary_round_trips_and_resumes_byte_identically() {
    exercise_adaptive_restart_posture(
        AdaptiveRestartFixtureMode::Terminal,
        DirectSnowStage3V11InterruptionPostureV2::AdaptiveMicrostepBoundary,
    );
    exercise_adaptive_restart_posture(
        AdaptiveRestartFixtureMode::PositiveSupportLiquidCustody,
        DirectSnowStage3V11InterruptionPostureV2::AdaptiveMicrostepBoundary,
    );
}

#[test]
fn before_terminal_event_round_trips_and_resumes_byte_identically() {
    exercise_adaptive_restart_posture(
        AdaptiveRestartFixtureMode::Terminal,
        DirectSnowStage3V11InterruptionPostureV2::BeforeTerminalEvent,
    );
}

#[test]
fn after_terminal_event_round_trips_and_resumes_byte_identically() {
    exercise_adaptive_restart_posture(
        AdaptiveRestartFixtureMode::Terminal,
        DirectSnowStage3V11InterruptionPostureV2::AfterTerminalEvent,
    );
}

#[test]
fn before_terminal_receiver_round_trips_and_resumes_byte_identically() {
    exercise_adaptive_restart_posture(
        AdaptiveRestartFixtureMode::Terminal,
        DirectSnowStage3V11InterruptionPostureV2::BeforeTerminalReceiver,
    );
}

#[test]
fn after_terminal_receiver_round_trips_and_resumes_byte_identically() {
    exercise_adaptive_restart_posture(
        AdaptiveRestartFixtureMode::Terminal,
        DirectSnowStage3V11InterruptionPostureV2::AfterTerminalReceiver,
    );
}

#[test]
fn before_snow_reappearance_round_trips_and_resumes_byte_identically() {
    exercise_adaptive_restart_posture(
        AdaptiveRestartFixtureMode::Reappearance,
        DirectSnowStage3V11InterruptionPostureV2::BeforeSnowReappearance,
    );
}

#[test]
fn after_snow_reappearance_round_trips_and_resumes_byte_identically() {
    exercise_adaptive_restart_posture(
        AdaptiveRestartFixtureMode::Reappearance,
        DirectSnowStage3V11InterruptionPostureV2::AfterSnowReappearance,
    );
}

#[test]
fn cross_midnight_owner_and_receipt_state_is_byte_identical_after_restart() {
    exercise_adaptive_restart_posture(
        AdaptiveRestartFixtureMode::CrossMidnight,
        DirectSnowStage3V11InterruptionPostureV2::AdaptiveMicrostepBoundary,
    );
}

#[test]
fn production_attachment_round_trips_canonical_v2_and_rejects_poison() {
    let fixture = restart_authority_owner_fixture();
    let surface_configuration = fixture
        .runtime
        .shadow
        .restart_authority_surface_configuration()
        .clone();
    let stage3_by_lane = surface_configuration
        .ofe_bindings
        .iter()
        .map(|binding| {
            (
                binding.production_lane_id,
                snow_free_state(binding.production_lane_id),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let production_configuration = DirectSnowStage3V11ProductionConfigurationV1 {
        run_identity: digest_bytes(b"stage3-v11-restart-fixture-run"),
        topology_identity: digest_bytes(b"stage3-v11-restart-fixture-topology"),
        calendar_receipt: digest_bytes(b"stage3-v11-restart-fixture-calendar"),
        controller_policy: digest_bytes(b"stage3-v11-restart-fixture-controller"),
        surface_liquid_configuration: surface_configuration,
        wb14_parameters: vec![DirectOfeWb14Parameters {
            ofe_id: OfeId::try_new("ofe-1").unwrap(),
            effective_conductivity_m_s: 1.0e-6,
            matric_potential_m: 0.1,
            infiltration_storage_capacity_m: 0.04,
        }],
    };
    let (run, topology) = checkpoint_identities_v1(
        &fixture.committed,
        fixture.runtime.shadow.root_zone_hydraulic_configuration(),
    )
    .unwrap();
    let real_consumer_context = ExpectedRestartStaticContext {
        run_identity_sha256: &run,
        topology_sha256: &topology,
        vegetation_configuration: fixture
            .runtime
            .shadow
            .restart_authority_vegetation_configuration(),
        vegetation_owner_id: fixture
            .runtime
            .shadow
            .restart_authority_vegetation_owner_id(),
        soil_thermal_owner_id: &fixture
            .runtime
            .shadow
            .restart_authority_soil_thermal()
            .expect("V1 Stage-3 fixture soil resident")
            .owner_id,
        soil_thermal_configuration_sha256: &fixture
            .runtime
            .shadow
            .restart_authority_soil_thermal()
            .expect("V1 Stage-3 fixture soil resident")
            .configuration_sha256,
        lse_configuration: fixture.runtime.shadow.restart_authority_lse_configuration(),
        surface_liquid_configuration: fixture
            .runtime
            .shadow
            .restart_authority_surface_configuration(),
        gsi_configuration: fixture.runtime.shadow.gsi_owner_configuration(),
        forcing_static_configuration: fixture.runtime.shadow.provider_static_configuration(),
        root_zone_hydraulic_configuration: fixture
            .runtime
            .shadow
            .root_zone_hydraulic_configuration(),
        phase_plan: &fixture
            .runtime
            .shadow
            .restart_authority_hydrology_frame()
            .phase_plan,
        phase_plan_sha256: &fixture.phase_plan_sha256,
        day_inputs: &fixture.day_inputs,
        day_input_digests: &fixture.day_input_digests,
    };
    let mut consumer_checkpoint = DirectV10RealConsumerCheckpointV1 {
        schema: "OPENWEPP_DIRECT_V10_REAL_CONSUMER_CHECKPOINT_V1".into(),
        version: 1,
        run_identity_sha256: run.clone(),
        topology_sha256: topology.clone(),
        phase: DirectV10CheckpointPhaseV1::BetweenDays {
            next_day_index: WireDayIndex(0),
            accepted_interval_count: AcceptedIntervalCount::try_new(0).unwrap(),
            committed: fixture.committed.clone(),
        },
        payload_sha256: Sha256Hex::try_new("0".repeat(64)).unwrap(),
    };
    consumer_checkpoint.seal().unwrap();
    let isolated = admit_checkpoint_v1(
        &to_canonical_bytes(&consumer_checkpoint).unwrap(),
        &real_consumer_context,
    )
    .unwrap();
    let consumer_host =
        DirectV10RestartHost::from_isolated(isolated, &real_consumer_context).unwrap();
    let attachment = DirectSnowStage3V11ShadowAttachment::new_production(
        production_configuration,
        stage3_by_lane,
        consumer_host.shadow().clone(),
    )
    .unwrap();
    let context = ExpectedSnowStage3V11RestartContext {
        static_context: &attachment.static_context,
        real_consumer_context: &real_consumer_context,
    };
    assert_empty_archive_v3_production_roundtrip(
        &attachment,
        &context,
        &fixture.phase_plan_sha256,
        &fixture.day_input_digests,
    );

    let mut production_frame = fixture
        .runtime
        .shadow
        .restart_authority_hydrology_frame()
        .clone();
    production_frame
        .configure_snow_stage3_v11_attachment(
            attachment.static_context.clone(),
            attachment.committed.clone(),
        )
        .unwrap();
    let projected_frame = DirectHydrologyRestartV1::project(
        &production_frame,
        fixture.phase_plan_sha256.clone(),
        &fixture.day_input_digests,
    )
    .unwrap();
    let hydrology_context = ExpectedDirectHydrologyRestartContext {
        phase_plan: &production_frame.phase_plan,
        phase_plan_sha256: &fixture.phase_plan_sha256,
        day_inputs: &fixture.day_inputs,
        day_input_digests: &fixture.day_input_digests,
        surface_liquid_configuration: fixture
            .runtime
            .shadow
            .restart_authority_surface_configuration(),
    };
    assert!(projected_frame.restore(&hydrology_context).is_err());
    let restored_frame = projected_frame
        .restore_with_stage3_v11(&hydrology_context, &context)
        .unwrap();
    assert!(restored_frame.snow_stage3_v11_attachment.is_some());
    assert_eq!(
        DirectHydrologyRestartV1::project(
            &restored_frame,
            fixture.phase_plan_sha256.clone(),
            &fixture.day_input_digests,
        )
        .unwrap(),
        projected_frame
    );

    let projected = DirectSnowStage3V11AttachmentRestartV2::project(
        &attachment,
        &fixture.phase_plan_sha256,
        &fixture.day_input_digests,
    )
    .unwrap();
    let bytes = projected.to_canonical_bytes().unwrap();
    assert!(
        !bytes
            .windows(b"ordinary_physical_reuse_seed".len())
            .any(|window| window == b"ordinary_physical_reuse_seed")
            && !bytes
                .windows(b"physical_authority_sha256".len())
                .any(|window| window == b"physical_authority_sha256"),
        "transient physical reuse seed must not leak into restart bytes"
    );
    let mut seed_leak_poison: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    seed_leak_poison["ordinary_physical_reuse_seed"] = serde_json::json!({
        "physical_authority_sha256": "00".repeat(32)
    });
    assert!(
        DirectSnowStage3V11AttachmentRestartV2::from_canonical_bytes(
            &serde_json::to_vec(&seed_leak_poison).unwrap(),
            &context,
        )
        .is_err()
    );
    let decoded =
        DirectSnowStage3V11AttachmentRestartV2::from_canonical_bytes(&bytes, &context).unwrap();
    let restored = decoded.restore(&context).unwrap();
    assert_eq!(restored.static_context, attachment.static_context);
    assert_eq!(restored.committed, attachment.committed);
    assert!(restored.restart_authority_pending_candidate().is_none());
    let reprojected = DirectSnowStage3V11AttachmentRestartV2::project(
        &restored,
        &fixture.phase_plan_sha256,
        &fixture.day_input_digests,
    )
    .unwrap();
    assert_eq!(reprojected, projected);
    assert_eq!(reprojected.to_canonical_bytes().unwrap(), bytes);

    let mut publication_poison = projected.clone();
    publication_poison
        .committed
        .accepted_publication_supports_canonical_base64
        .push('A');
    publication_poison.seal().unwrap();
    assert!(publication_poison.restore(&context).is_err());

    let mut payload_poison = projected.clone();
    payload_poison.payload_sha256 = Sha256Hex::try_new("1".repeat(64)).unwrap();
    assert!(payload_poison.restore(&context).is_err());

    let mut static_poison = projected.clone();
    static_poison.static_context_sha256 = Sha256Hex::try_new("2".repeat(64)).unwrap();
    static_poison.seal().unwrap();
    assert!(static_poison.restore(&context).is_err());

    let mut substituted_static_context = attachment.static_context.clone();
    substituted_static_context
        .vegetation_configuration
        .configuration_sha256 = "3".repeat(64);
    let substituted_context = ExpectedSnowStage3V11RestartContext {
        static_context: &substituted_static_context,
        real_consumer_context: &real_consumer_context,
    };
    assert!(projected.restore(&substituted_context).is_err());

    let mut state_receipt_poison = projected.clone();
    state_receipt_poison
        .committed
        .v11_parent_checkpoint
        .staged_state
        .state_sha256 = "4".repeat(64);
    state_receipt_poison.seal().unwrap();
    assert!(state_receipt_poison.restore(&context).is_err());

    let mut blob_poison = projected;
    blob_poison
        .committed
        .receipt_state_canonical_base64
        .push('A');
    blob_poison.seal().unwrap();
    assert!(blob_poison.restore(&context).is_err());

    let mut noncanonical = bytes;
    noncanonical.push(b' ');
    assert!(
        DirectSnowStage3V11AttachmentRestartV2::from_canonical_bytes(&noncanonical, &context,)
            .is_err()
    );
}
