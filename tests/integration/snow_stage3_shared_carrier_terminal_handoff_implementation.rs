use std::collections::BTreeMap;

use openwepp_coupled_time::{CoupledTimeError, ModelTimeNs};
use openwepp_hillslope_orchestrator::snow_stage3_terminal_handoff::{
    COMPLETE_OWNER_MANIFEST, CanopyLongwaveComponent, CarrierSurface, CompleteOwnerSet,
    ParticipantSupportReceipt, SealedExposureReceipt, SegmentPhase, SharedCarrierInput,
    SnowCarrierLedgerInput, SnowFreeContinuationInput, SnowStage3HandoffRuntime,
    SnowStage3OwnerExecutionReceipt, SnowStage3TerminalHandoffRequest, TerminalEventInput,
    TerminalStateRates, evaluate_shared_carrier, locate_terminal_event,
};
use openwepp_persisted_restart_v1::SnowStage3HandoffRestartV1;

fn tick(value: u128) -> ModelTimeNs {
    ModelTimeNs::new(value)
}

fn support(participant_id: &str, receipt_id: &str, minimum: u128) -> ParticipantSupportReceipt {
    ParticipantSupportReceipt {
        participant_id: participant_id.to_string(),
        support_receipt_id: receipt_id.to_string(),
        minimum_support_ns: tick(minimum),
    }
}

fn carrier() -> SharedCarrierInput {
    SharedCarrierInput {
        phase: SegmentPhase::SnowCovered,
        rho_air_kg_m3: 1.2,
        cp_air_j_kg_k: 1005.0,
        reference: CarrierSurface {
            temperature_k: 280.0,
            specific_humidity: 0.002,
            heat_conductance_m_s: 0.1,
            vapor_conductance_m_s: 0.1,
        },
        canopy: CarrierSurface {
            temperature_k: 285.0,
            specific_humidity: 0.004,
            heat_conductance_m_s: 0.05,
            vapor_conductance_m_s: 0.05,
        },
        snow: CarrierSurface {
            temperature_k: 270.0,
            specific_humidity: 0.001,
            heat_conductance_m_s: 0.05,
            vapor_conductance_m_s: 0.05,
        },
        canopy_longwave_components: vec![
            CanopyLongwaveComponent {
                temperature_k: 285.0,
                emissive_area_weight: 0.7,
            },
            CanopyLongwaveComponent {
                temperature_k: 275.0,
                emissive_area_weight: 0.3,
            },
        ],
        exposure: SealedExposureReceipt {
            receipt_id: "exposure-v1".to_string(),
            provider: "sealed-stage3-exposure".to_string(),
            provider_digest: "exposure-provider-digest".to_string(),
            source: "sealed-exposure-v1".to_string(),
            wind_m_s: 3.0,
            transfer_height_m: 5.0,
            roughness_m: 0.005,
        },
        active_participants: vec![
            "shared-carrier".to_string(),
            "stage3-snow".to_string(),
            "v11-canopy".to_string(),
        ],
        support_receipts: vec![
            support("shared-carrier", "support-carrier-v1", 60_000_000_000),
            support("stage3-snow", "support-stage3-v1", 60_000_000_000),
            support("v11-canopy", "support-v11-v1", 60_000_000_000),
        ],
        atmospheric_longwave_w_m2: 280.0,
        effective_canopy_cover: 0.5,
        canopy_intercepted_snow: false,
        ledger: SnowCarrierLedgerInput {
            duration_s: 3600.0,
            snow_ice_start_kg_m2: 10.0,
            solid_precipitation_kg_m2: 0.1,
            melt_kg_m2: 0.03,
            sublimation_kg_m2: 0.02,
            deposition_kg_m2: 0.01,
            liquid_start_kg_m2: 0.5,
            rain_kg_m2: 0.2,
            refreeze_kg_m2: 0.01,
            liquid_runoff_kg_m2: 0.1,
            energy_start_j_m2: 1000.0,
            external_energy_j_m2: 5000.0,
            canopy_energy_j_m2: -1000.0,
            snow_energy_j_m2: 3000.0,
            energy_end_j_m2: 8000.0,
            canopy_snow_longwave_exchange_j_m2: -139_473.340_214_138_1,
            snow_canopy_longwave_exchange_j_m2: 139_473.340_214_138_1,
        },
    }
}

fn event(end: u128) -> TerminalEventInput {
    TerminalEventInput {
        parent_identity: "parent-child2c-test".to_string(),
        segment_identity: "segment-stage3-test".to_string(),
        event_ordinal: 1,
        parent_start_tick: tick(0),
        parent_end_tick: tick(end),
        proposed_event_tick: tick(end),
        candidate_ticks: vec![tick(end)],
        pre_active_participants: vec![
            support("shared-carrier", "support-carrier-v1", end),
            support("stage3-snow", "support-stage3-v1", end),
            support("v11-canopy", "support-v11-v1", end),
        ],
        post_active_participants: Vec::new(),
        event_time_tolerance_ns: tick(0),
        snow_mass_tolerance_kg_m2: 0.0,
        liquid_mass_tolerance_kg_m2: 0.0,
        energy_tolerance_j_m2: 0.0,
        terminal_state: TerminalStateRates {
            snow_start_kg_m2: 10.0,
            snow_rate_kg_m2_s: 0.0,
            snow_target_kg_m2: 10.0,
            liquid_start_kg_m2: 0.5,
            liquid_rate_kg_m2_s: 0.0,
            liquid_target_kg_m2: 0.5,
            energy_start_j_m2: 0.0,
            energy_rate_j_m2_s: 0.0,
            energy_target_j_m2: 0.0,
        },
    }
}

fn owners(seed: u8) -> CompleteOwnerSet {
    let values = COMPLETE_OWNER_MANIFEST
        .into_iter()
        .map(|name| {
            (
                name.to_string(),
                vec![
                    seed,
                    u8::try_from(name.len()).expect("test owner id length"),
                ],
            )
        })
        .collect::<BTreeMap<_, _>>();
    CompleteOwnerSet::new(values).expect("test owner manifest")
}

fn owner_execution(owners: CompleteOwnerSet) -> SnowStage3OwnerExecutionReceipt {
    SnowStage3OwnerExecutionReceipt::from_owner_set("test-owner-envelope", owners)
        .expect("owner execution receipt")
}

#[test]
fn shared_carrier_reconstructs_authority_vector_and_rejects_proxies() {
    let result = evaluate_shared_carrier(&carrier()).expect("carrier vector");
    assert!((result.shared_air_temperature_k - 278.75).abs() < 1.0e-12);
    assert!((result.shared_air_specific_humidity - 0.00225).abs() < 1.0e-15);
    assert!((result.snow_sensible_into_surface_w_m2 - 527.625).abs() < 1.0e-12);
    assert!((result.snow_vapor_into_surface_kg_m2_s - 0.000_075).abs() < 1.0e-15);
    assert!((result.sky_view_fraction - 0.329_876_977_693_223_55).abs() < 1.0e-15);
    assert!((result.snow_longwave_net_w_m2 - 31.700_728_751_306_826).abs() < 1.0e-12);
    assert!((result.snow_ice_end_kg_m2 - 10.06).abs() < 1.0e-12);
    assert!((result.liquid_end_kg_m2 - 0.62).abs() < 1.0e-12);

    let mut raw_wind = carrier();
    raw_wind.exposure.source = "raw-10m".to_string();
    assert!(evaluate_shared_carrier(&raw_wind).is_err());
    let mut wrong_geometry = carrier();
    wrong_geometry.exposure.transfer_height_m = 4.0;
    assert!(evaluate_shared_carrier(&wrong_geometry).is_err());
    let mut wrong_longwave = carrier();
    wrong_longwave.ledger.snow_canopy_longwave_exchange_j_m2 = 0.0;
    assert!(evaluate_shared_carrier(&wrong_longwave).is_err());
    let mut full_cover = carrier();
    full_cover.effective_canopy_cover = 1.0;
    assert!(evaluate_shared_carrier(&full_cover).is_err());
    let mut two_nodes = carrier();
    two_nodes
        .active_participants
        .push("independent-canopy-air".to_string());
    assert!(evaluate_shared_carrier(&two_nodes).is_err());
    let mut intercepted = carrier();
    intercepted.canopy_intercepted_snow = true;
    assert!(evaluate_shared_carrier(&intercepted).is_err());
}

#[test]
fn event_selection_uses_support_tolerance_and_atomic_no_candidate() {
    let mut request = event(200_000_000_000);
    request.proposed_event_tick = tick(110_000_000_000);
    request.candidate_ticks = vec![
        tick(100_000_000_000),
        tick(110_000_000_000),
        tick(120_000_000_000),
    ];
    request.pre_active_participants = vec![
        support("stage3", "stage3-pre", 60_000_000_000),
        support("v11", "v11-pre", 120_000_000_000),
    ];
    request.post_active_participants = vec![support("v11", "v11-post", 60_000_000_000)];
    request.event_time_tolerance_ns = tick(10_000_000_000);
    request.snow_mass_tolerance_kg_m2 = 0.001;
    request.liquid_mass_tolerance_kg_m2 = 0.001;
    request.energy_tolerance_j_m2 = 10.0;
    request.terminal_state = TerminalStateRates {
        snow_start_kg_m2: 10.0,
        snow_rate_kg_m2_s: -0.01,
        snow_target_kg_m2: 8.8,
        liquid_start_kg_m2: 0.5,
        liquid_rate_kg_m2_s: 0.001,
        liquid_target_kg_m2: 0.62,
        energy_start_j_m2: 0.0,
        energy_rate_j_m2_s: 1.0,
        energy_target_j_m2: 120.0,
    };
    let selected = locate_terminal_event(&request).expect("support-admissible event");
    assert_eq!(selected.accepted_event_tick, Some(tick(120_000_000_000)));
    assert_eq!(
        selected.pre_common_minimum_support_ns,
        tick(120_000_000_000)
    );

    let mut no_candidate = event(120_000_000_000);
    no_candidate.proposed_event_tick = tick(50_000_000_000);
    no_candidate.candidate_ticks = vec![tick(50_000_000_000)];
    no_candidate.pre_active_participants = vec![support("stage3", "stage3", 60_000_000_000)];
    no_candidate.post_active_participants = vec![support("lse", "lse", 60_000_000_000)];
    let error = locate_terminal_event(&no_candidate).expect_err("retry boundary");
    assert_eq!(
        error.to_string(),
        "ERR-CT-021 event boundary has no admissible candidate"
    );
    assert!(matches!(error, openwepp_hillslope_orchestrator::snow_stage3_terminal_handoff::SnowStage3HandoffError::CoupledTime(CoupledTimeError::EventBoundaryNoCandidate)));
}

#[test]
fn handoff_stages_complete_owners_commits_once_and_round_trips_restart() {
    let beginning = owners(1);
    let ending = owners(2);
    let first_owner_execution = owner_execution(ending.clone());
    let mut runtime = SnowStage3HandoffRuntime::new(tick(0), beginning.clone()).expect("runtime");
    let request = SnowStage3TerminalHandoffRequest {
        carrier: carrier(),
        event: event(60_000_000_000),
        beginning_owners: beginning.clone(),
        ending_owners: ending,
        owner_execution: first_owner_execution,
        retained_liquid_kg_m2: 0.7,
        snow_support_rain_kg_m2: 0.2,
        terminal_melt_kg_m2: 0.5,
        terminal_refreeze_kg_m2: 0.1,
        continuation: SnowFreeContinuationInput {
            duration_ns: tick(0),
            terminal_liquid_kg_m2: 1.3,
            post_event_contains_snow_operands: false,
        },
    };
    let mut failure_runtime =
        SnowStage3HandoffRuntime::new(tick(0), beginning.clone()).expect("failure runtime");
    let mut invalid_request = request.clone();
    invalid_request
        .continuation
        .post_event_contains_snow_operands = true;
    let failure_before = failure_runtime
        .committed_owner_digest()
        .expect("failure digest");
    assert!(failure_runtime.stage(invalid_request).is_err());
    assert_eq!(failure_runtime.accepted_cursor_ns(), tick(0));
    assert_eq!(
        failure_runtime
            .committed_owner_digest()
            .expect("failure after digest"),
        failure_before
    );
    let before = runtime.committed_owner_digest().expect("before digest");
    runtime.stage(request).expect("stage without mutation");
    assert_eq!(
        runtime.committed_owner_digest().expect("staged digest"),
        before
    );
    let receipt = runtime.commit_pending().expect("commit");
    assert_eq!(receipt.continuation_duration_ns, tick(0));
    assert_eq!(receipt.parent_identity, "parent-child2c-test");
    assert_eq!(receipt.segment_identity, "segment-stage3-test");
    assert_eq!(receipt.accepted_tie_rank, 1);
    assert_eq!(runtime.accepted_cursor_ns(), tick(60_000_000_000));
    assert_eq!(runtime.accepted_event_ordinal(), 1);
    assert_eq!(runtime.receipt_chain().len(), 1);
    assert_eq!(runtime.receipt_history(), std::slice::from_ref(&receipt));

    let second_ending = owners(3);
    let mut second_event = event(120_000_000_000);
    second_event.segment_identity = "segment-stage3-test-2".to_string();
    second_event.event_ordinal = 2;
    second_event.parent_start_tick = tick(60_000_000_000);
    second_event.proposed_event_tick = tick(120_000_000_000);
    second_event.candidate_ticks = vec![tick(120_000_000_000)];
    second_event.pre_active_participants = carrier().support_receipts;
    second_event.post_active_participants = Vec::new();
    let second_request = SnowStage3TerminalHandoffRequest {
        carrier: carrier(),
        event: second_event,
        beginning_owners: runtime.committed_owners().clone(),
        ending_owners: second_ending.clone(),
        owner_execution: owner_execution(second_ending),
        retained_liquid_kg_m2: 0.7,
        snow_support_rain_kg_m2: 0.2,
        terminal_melt_kg_m2: 0.5,
        terminal_refreeze_kg_m2: 0.1,
        continuation: SnowFreeContinuationInput {
            duration_ns: tick(0),
            terminal_liquid_kg_m2: 1.3,
            post_event_contains_snow_operands: false,
        },
    };
    runtime
        .stage(second_request)
        .expect("second terminal stage");
    runtime.commit_pending().expect("second terminal commit");
    assert_eq!(runtime.accepted_event_ordinal(), 2);
    assert_eq!(runtime.receipt_chain().len(), 2);

    let checkpoint = SnowStage3HandoffRestartV1::project(&runtime).expect("checkpoint");
    let bytes = checkpoint
        .to_canonical_json()
        .expect("canonical checkpoint");
    let restored = SnowStage3HandoffRestartV1::admit(&bytes).expect("admit checkpoint");
    assert_eq!(restored, runtime);
    let mut poisoned = bytes;
    let poison_index = poisoned.len() - 2;
    poisoned[poison_index] ^= 1;
    assert!(SnowStage3HandoffRestartV1::admit(&poisoned).is_err());
}

#[test]
fn handoff_rejects_unjoined_carrier_and_successor_identity() {
    let beginning = owners(11);
    let ending = owners(12);
    let mut runtime = SnowStage3HandoffRuntime::new(tick(0), beginning.clone()).expect("runtime");
    let mut request = SnowStage3TerminalHandoffRequest {
        carrier: carrier(),
        event: event(180_000_000_000),
        beginning_owners: beginning,
        ending_owners: ending.clone(),
        owner_execution: owner_execution(ending),
        retained_liquid_kg_m2: 0.7,
        snow_support_rain_kg_m2: 0.2,
        terminal_melt_kg_m2: 0.5,
        terminal_refreeze_kg_m2: 0.1,
        continuation: SnowFreeContinuationInput {
            duration_ns: tick(180_000_000_000),
            terminal_liquid_kg_m2: 1.3,
            post_event_contains_snow_operands: false,
        },
    };
    request.event.pre_active_participants[0].participant_id = "wrong-owner".to_string();
    assert!(runtime.stage(request.clone()).is_err());

    let mut no_successor = request;
    no_successor.event.pre_active_participants = carrier()
        .active_participants
        .iter()
        .map(|participant| support(participant, &format!("{participant}-pre"), 60_000_000_000))
        .collect();
    no_successor.event.post_active_participants.clear();
    assert!(runtime.stage(no_successor).is_err());

    let mut subminimum_successor = SnowStage3TerminalHandoffRequest {
        carrier: carrier(),
        event: event(180_000_000_000),
        beginning_owners: owners(11),
        ending_owners: owners(12),
        owner_execution: owner_execution(owners(12)),
        retained_liquid_kg_m2: 0.7,
        snow_support_rain_kg_m2: 0.2,
        terminal_melt_kg_m2: 0.5,
        terminal_refreeze_kg_m2: 0.1,
        continuation: SnowFreeContinuationInput {
            duration_ns: tick(180_000_000_000),
            terminal_liquid_kg_m2: 1.3,
            post_event_contains_snow_operands: false,
        },
    };
    subminimum_successor.event.pre_active_participants = carrier().support_receipts;
    subminimum_successor.event.post_active_participants =
        vec![support("v11-canopy", "short-post", 1)];
    assert!(runtime.stage(subminimum_successor).is_err());
}

#[test]
fn complete_owner_set_rejects_empty_state_payloads() {
    let mut incomplete = owners(21).owners;
    incomplete.insert("vegetation".to_string(), Vec::new());
    assert!(CompleteOwnerSet::new(incomplete).is_err());
}

#[test]
fn direct_stage3_runtime_commits_staged_handoff() {
    let beginning = owners(3);
    let mut runtime = SnowStage3HandoffRuntime::new(tick(0), beginning.clone()).expect("runtime");
    let ending = owners(4);
    let request = SnowStage3TerminalHandoffRequest {
        carrier: carrier(),
        event: event(60_000_000_000),
        beginning_owners: beginning,
        ending_owners: ending.clone(),
        owner_execution: owner_execution(ending),
        retained_liquid_kg_m2: 0.7,
        snow_support_rain_kg_m2: 0.2,
        terminal_melt_kg_m2: 0.5,
        terminal_refreeze_kg_m2: 0.1,
        continuation: SnowFreeContinuationInput {
            duration_ns: tick(0),
            terminal_liquid_kg_m2: 1.3,
            post_event_contains_snow_operands: false,
        },
    };
    runtime.stage(request).expect("stage handoff candidate");
    runtime.commit_pending().expect("commit handoff candidate");
    assert_eq!(runtime.accepted_cursor_ns(), tick(60_000_000_000));
    assert_eq!(runtime.receipt_chain().len(), 1);
}
