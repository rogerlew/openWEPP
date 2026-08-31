struct EmptyStage3ArchiveReaderV3;

impl crate::Stage3CommittedDayArchiveReaderV3 for EmptyStage3ArchiveReaderV3 {
    fn read_canonical_uncompressed(&self, _content_sha256: Digest32) -> Option<Vec<u8>> {
        None
    }
}

#[derive(Default)]
struct InMemoryStage3ArchiveReaderV3 {
    content: BTreeMap<Digest32, Vec<u8>>,
}

impl crate::Stage3CommittedDayArchiveReaderV3 for InMemoryStage3ArchiveReaderV3 {
    fn read_canonical_uncompressed(&self, content_sha256: Digest32) -> Option<Vec<u8>> {
        self.content.get(&content_sha256).cloned()
    }
}

fn nonzero_archive_profile_inputs(
    lane: &openwepp_hillslope_orchestrator::DirectLaneFrame,
) -> (
    openwepp_hillslope_orchestrator::DirectHydrologyProjectionInputs,
    openwepp_hillslope_orchestrator::DirectSubsurfaceComputeInputs,
) {
    let profile_depth_m = lane
        .subsurface_layers
        .iter()
        .map(|layer| layer.depth_m)
        .sum::<f64>();
    let projection = openwepp_hillslope_orchestrator::DirectHydrologyProjectionInputs {
        aggregate_storage_tolerance_m: 1.0e-9,
        profile_depth_m: Some(profile_depth_m),
        profile_porosity_cap_m: Some(
            lane.subsurface_layers
                .iter()
                .map(|layer| layer.porosity * layer.depth_m)
                .sum(),
        ),
        profile_field_capacity_m: Some(
            lane.subsurface_layers
                .iter()
                .map(|layer| layer.field_capacity_m)
                .sum(),
        ),
        profile_wilting_point_m: Some(
            lane.subsurface_layers
                .iter()
                .map(|layer| layer.residual_theta * layer.depth_m)
                .sum(),
        ),
        ..openwepp_hillslope_orchestrator::DirectHydrologyProjectionInputs::zero()
    };
    let mut subsurface =
        openwepp_hillslope_orchestrator::DirectSubsurfaceComputeInputs::neutral();
    subsurface.soil_depth_m = profile_depth_m;
    subsurface.layers = lane
        .subsurface_layers
        .iter()
        .cloned()
        .map(Into::into)
        .collect();
    (projection, subsurface)
}

fn install_nonzero_archive_continuation_context(
    fixture: &mut crate::RestartAuthorityPreparedDayFixture,
) {
    fixture.owners.day_inputs = fixture
        .owners
        .runtime
        .shadow
        .restart_authority_hydrology_frame()
        .lanes
        .iter()
        .map(|lane| {
            let (hydrology_projection_inputs, subsurface_compute_inputs) =
                nonzero_archive_profile_inputs(lane);
            let mut day = openwepp_hillslope_orchestrator::DirectDayConstructorInputs::zero();
            day.forcing.precipitation_m = 0.0;
            day.forcing.effective_temperature_c = 0.0;
            day.hydrology_projection_inputs = hydrology_projection_inputs;
            day.subsurface_compute_inputs = subsurface_compute_inputs;
            vec![day.clone(), day]
        })
        .collect();
    fixture.owners.day_input_digests = fixture
        .owners
        .day_inputs
        .iter()
        .map(|inputs| {
            crate::hydrology_restart::canonical_operand_sha256(
                "DirectDayConstructorInputsV1",
                inputs,
            )
            .unwrap()
        })
        .collect();
    let continuation_inputs = fixture.owners.day_inputs.clone();
    fixture
        .owners
        .runtime
        .shadow
        .restart_authority_install_hydrology_continuation_inputs_v3(&continuation_inputs)
        .unwrap();
    fixture.owners.committed.scientific.direct_hydrology = DirectHydrologyRestartV1::project(
        fixture
            .owners
            .runtime
            .shadow
            .restart_authority_hydrology_frame(),
        fixture.owners.phase_plan_sha256.clone(),
        &fixture.owners.day_input_digests,
    )
    .unwrap();
}

fn nonzero_archive_publication_inputs(
    fixture: &crate::RestartAuthorityPreparedDayFixture,
) -> Vec<openwepp_hillslope_orchestrator::DirectPublicationDayInput> {
    let initial_frame = fixture
        .owners
        .runtime
        .shadow
        .restart_authority_hydrology_frame();
    initial_frame
        .lanes
        .iter()
        .enumerate()
        .map(|(lane_index, lane)| {
            let mut input =
                openwepp_hillslope_orchestrator::DirectPublicationDayInput::calendar_only(
                    openwepp_hillslope_orchestrator::DirectPublicationCalendarDay {
                        year: 2000,
                        julian_day: 172,
                        month: 6,
                        day_of_month: 20,
                        water_year: 2000,
                    },
                );
            input.precipitation_m = 0.0;
            input.effective_temperature_c = 0.0;
            input.initial_soil_water_m = Some(initial_frame.lanes[lane_index].water.soil_water_m);
            let (projection, subsurface) = nonzero_archive_profile_inputs(lane);
            input.hydrology_projection_inputs = Some(projection);
            input.subsurface_compute_inputs = Some(subsurface);
            input
        })
        .collect()
}

fn assert_nonzero_archive_v3_stable_fixture() {
    let mut fixture = restart_authority_adaptive_prepared_day_fixture();
    openwepp_hillslope_orchestrator::v9_real_consumer_shadow::restart_authority_equilibrate_complete_owner_fixture(
        &mut fixture.owners.runtime.shadow,
    )
    .unwrap();
    let surface_configuration = fixture
        .owners
        .runtime
        .shadow
        .restart_authority_surface_configuration()
        .clone();
    let lane_id = surface_configuration.ofe_bindings[0].production_lane_id;
    let prepared = adaptive_prepared_day(
        &fixture,
        lane_id,
        AdaptiveRestartFixtureMode::Reappearance,
    );
    let attachment = DirectSnowStage3V11ShadowAttachment::new_production(
        DirectSnowStage3V11ProductionConfigurationV1 {
            run_identity: digest_bytes(b"stage3-v11-nonzero-archive-run"),
            topology_identity: digest_bytes(b"stage3-v11-nonzero-archive-topology"),
            calendar_receipt: digest_bytes(b"stage3-v11-nonzero-archive-calendar"),
            controller_policy: digest_bytes(b"stage3-v11-nonzero-archive-controller"),
            surface_liquid_configuration: surface_configuration,
            wb14_parameters: vec![DirectOfeWb14Parameters {
                ofe_id: OfeId::try_new("ofe-1").unwrap(),
                effective_conductivity_m_s: 1.0e-6,
                matric_potential_m: 0.1,
                infiltration_storage_capacity_m: 0.04,
            }],
        },
        BTreeMap::from([(
            lane_id,
            adaptive_snow_state(lane_id, AdaptiveRestartFixtureMode::Reappearance),
        )]),
        fixture.owners.runtime.shadow.clone(),
    )
    .unwrap();

    install_nonzero_archive_continuation_context(&mut fixture);
    assert_eq!(
        fixture
            .owners
            .runtime
            .shadow
            .restart_authority_hydrology_frame()
            .identity
            .day_count,
        2,
    );
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
            .expect("V1 V3 fixture soil resident")
            .owner_id,
        soil_thermal_configuration_sha256: &fixture
            .owners
            .runtime
            .shadow
            .restart_authority_soil_thermal()
            .expect("V1 V3 fixture soil resident")
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
    let static_context = attachment.static_context.clone();
    let restart_context = ExpectedSnowStage3V11RestartContext {
        static_context: &static_context,
        real_consumer_context: &real_consumer_context,
    };
    let publication_inputs = nonzero_archive_publication_inputs(&fixture);
    assert_nonzero_archive_v3_roundtrip_and_reader_poisons(
        attachment,
        &prepared,
        &publication_inputs,
        &restart_context,
        &fixture.owners.phase_plan_sha256,
        &fixture.owners.day_input_digests,
    );
}

fn empty_archive_manifest_v3(
    value: &DirectSnowStage3V11ShadowAttachment,
) -> openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::Stage3CommittedDayArchiveManifestV1
{
    openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::Stage3CommittedDayArchiveManifestV1::empty(
        value.static_context.run_identity,
        value.static_context.topology_identity,
    )
    .unwrap()
}

fn restore_empty_archive_v3(
    value: &DirectSnowStage3V11ShadowAttachment,
    context: &ExpectedSnowStage3V11RestartContext<'_>,
    phase_plan_sha256: &Sha256Hex,
    day_input_digests: &[Sha256Hex],
) -> (DirectSnowStage3V11ShadowAttachment, Vec<u8>) {
    let manifest = empty_archive_manifest_v3(value);
    let reader = EmptyStage3ArchiveReaderV3;
    let archive = crate::ExpectedStage3CommittedDayArchiveV3 {
        manifest: &manifest,
        reader: &reader,
    };
    let projected = crate::DirectSnowStage3V11AttachmentRestartV3::project(
        value,
        phase_plan_sha256,
        day_input_digests,
        &archive,
    )
    .unwrap();
    assert_eq!(projected.archive_record_count, 0);
    assert_eq!(
        projected.archive_content_root_sha256,
        manifest.archive_content_root_sha256
    );
    let bytes = projected.to_canonical_bytes().unwrap();
    let restored = crate::DirectSnowStage3V11AttachmentRestartV3::from_canonical_bytes(
        &bytes, context, &archive,
    )
    .unwrap()
    .restore(context, &archive)
    .unwrap();
    (restored, bytes)
}

fn project_empty_archive_v3_bytes(
    value: &DirectSnowStage3V11ShadowAttachment,
    phase_plan_sha256: &Sha256Hex,
    day_input_digests: &[Sha256Hex],
) -> Vec<u8> {
    let manifest = empty_archive_manifest_v3(value);
    let reader = EmptyStage3ArchiveReaderV3;
    crate::DirectSnowStage3V11AttachmentRestartV3::project(
        value,
        phase_plan_sha256,
        day_input_digests,
        &crate::ExpectedStage3CommittedDayArchiveV3 {
            manifest: &manifest,
            reader: &reader,
        },
    )
    .unwrap()
    .to_canonical_bytes()
    .unwrap()
}

fn assert_empty_archive_v3_support_liquid_custody_poisons(
    value: &DirectSnowStage3V11ShadowAttachment,
    context: &ExpectedSnowStage3V11RestartContext<'_>,
    phase_plan_sha256: &Sha256Hex,
    day_input_digests: &[Sha256Hex],
) {
    use openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::RestartAuthoritySupportLiquidCustodyPoisonV3::{
        LseBeginningSubstitution, LseEndingSubstitution, RunoffDispositionSubstitution,
        RunoffRouteTopologySubstitution,
    };

    let manifest = empty_archive_manifest_v3(value);
    let reader = EmptyStage3ArchiveReaderV3;
    let archive = crate::ExpectedStage3CommittedDayArchiveV3 {
        manifest: &manifest,
        reader: &reader,
    };
    let projected = crate::DirectSnowStage3V11AttachmentRestartV3::project(
        value,
        phase_plan_sha256,
        day_input_digests,
        &archive,
    )
    .unwrap();
    for poison in [
        LseBeginningSubstitution,
        LseEndingSubstitution,
        RunoffRouteTopologySubstitution,
        RunoffDispositionSubstitution,
    ] {
        let poisoned = projected
            .restart_authority_with_support_liquid_custody_poison_v3(poison)
            .unwrap();
        let error = poisoned
            .restore(context, &archive)
            .expect_err("resealed V3 custody substitution must reject");
        if matches!(
            poison,
            RunoffRouteTopologySubstitution | RunoffDispositionSubstitution
        ) {
            assert!(
                error
                    .to_string()
                    .contains("support-liquid custody V2 event/receipt-set ledger join"),
                "route/disposition poison must fail the causal event ledger join: {error}",
            );
        }
    }
}

fn assert_empty_archive_v3_terminal_liquid_custody_poisons(
    value: &DirectSnowStage3V11ShadowAttachment,
    context: &ExpectedSnowStage3V11RestartContext<'_>,
    phase_plan_sha256: &Sha256Hex,
    day_input_digests: &[Sha256Hex],
) {
    use openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::RestartAuthorityTerminalLiquidCustodyPoisonV3::{
        LseBeginningSubstitution, LseEndingSubstitution, RunoffDispositionSubstitution,
        RunoffRouteTopologySubstitution,
    };

    let manifest = empty_archive_manifest_v3(value);
    let reader = EmptyStage3ArchiveReaderV3;
    let archive = crate::ExpectedStage3CommittedDayArchiveV3 {
        manifest: &manifest,
        reader: &reader,
    };
    let projected = crate::DirectSnowStage3V11AttachmentRestartV3::project(
        value,
        phase_plan_sha256,
        day_input_digests,
        &archive,
    )
    .unwrap();
    for poison in [
        LseBeginningSubstitution,
        LseEndingSubstitution,
        RunoffRouteTopologySubstitution,
        RunoffDispositionSubstitution,
    ] {
        let poisoned = projected
            .restart_authority_with_terminal_liquid_custody_poison_v3(poison)
            .unwrap();
        poisoned
            .restore(context, &archive)
            .expect_err("resealed terminal V3 custody substitution must reject");
    }
}

fn assert_empty_archive_v3_production_roundtrip(
    attachment: &DirectSnowStage3V11ShadowAttachment,
    context: &ExpectedSnowStage3V11RestartContext<'_>,
    phase_plan_sha256: &Sha256Hex,
    day_input_digests: &[Sha256Hex],
) {
    let (restored, bytes) =
        restore_empty_archive_v3(attachment, context, phase_plan_sha256, day_input_digests);
    assert_eq!(&restored, attachment);
    let reader = EmptyStage3ArchiveReaderV3;
    let substituted_manifest = openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::Stage3CommittedDayArchiveManifestV1::empty(
        digest_bytes(b"stage3-v11-restart-substituted-run"),
        attachment.static_context.topology_identity,
    )
    .unwrap();
    assert!(
        crate::DirectSnowStage3V11AttachmentRestartV3::from_canonical_bytes(
            &bytes,
            context,
            &crate::ExpectedStage3CommittedDayArchiveV3 {
                manifest: &substituted_manifest,
                reader: &reader,
            },
        )
        .is_err(),
        "archive manifest static/root substitution must reject",
    );
}

fn assert_nonzero_archive_v3_roundtrip_and_reader_poisons(
    mut value: DirectSnowStage3V11ShadowAttachment,
    prepared: &openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::ValidatedPreparedStage3V11DayV1,
    publication_inputs: &[openwepp_hillslope_orchestrator::DirectPublicationDayInput],
    context: &ExpectedSnowStage3V11RestartContext<'_>,
    phase_plan_sha256: &Sha256Hex,
    day_input_digests: &[Sha256Hex],
) {
    assert_eq!(
        context.real_consumer_context.day_inputs.len(),
        day_input_digests.len(),
    );
    for (inputs, expected) in context
        .real_consumer_context
        .day_inputs
        .iter()
        .zip(day_input_digests)
    {
        assert_eq!(
            crate::hydrology_restart::canonical_operand_sha256(
                "DirectDayConstructorInputsV1",
                inputs,
            )
            .unwrap(),
            *expected,
            "continuation constructor vector must match its exact canonical context seal",
        );
    }
    let empty_publication_rotation = openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::restart_authority_encode_publication_rotation_state_v3(&value)
        .unwrap();
    value.stage_prepared_day(prepared).unwrap();
    value
        .restart_authority_complete_commit_and_stage_archive_day_v3(publication_inputs)
        .unwrap();
    let pending = value
        .pending_committed_day_evidence_v1()
        .expect("nonzero archive staging evidence");
    let entry = pending.entry().clone();
    let mut canonical = Vec::new();
    value
        .write_pending_committed_day_evidence_v1(&mut canonical)
        .unwrap();
    assert_eq!(digest_bytes(&canonical), entry.content_sha256);
    let mut manifest = empty_archive_manifest_v3(&value);
    manifest.append(entry.clone()).unwrap();
    value
        .acknowledge_committed_day_archive_v1(entry.record_sha256)
        .unwrap();
    assert_eq!(value.committed.real_consumer.v11_next_day_index(), 1);
    value
        .restart_authority_install_hydrology_continuation_inputs_v3(
            context.real_consumer_context.day_inputs,
        )
        .unwrap();
    assert_eq!(value.archived_receipt_prefix_v1().archived_day_count, 1);
    assert!(value.committed.receipt_chain.is_empty());

    let reader = InMemoryStage3ArchiveReaderV3 {
        content: BTreeMap::from([(entry.content_sha256, canonical.clone())]),
    };
    let archive = crate::ExpectedStage3CommittedDayArchiveV3 {
        manifest: &manifest,
        reader: &reader,
    };
    let projected = crate::DirectSnowStage3V11AttachmentRestartV3::project(
        &value,
        phase_plan_sha256,
        day_input_digests,
        &archive,
    )
    .unwrap();
    assert_eq!(projected.archive_record_count, 1);
    assert_eq!(
        projected.archive_content_root_sha256,
        manifest.archive_content_root_sha256,
    );
    let empty_rotation_substitution = projected
        .restart_authority_with_publication_rotation_substitution_v3(
            &empty_publication_rotation,
        )
        .unwrap();
    assert!(
        empty_rotation_substitution.restore(context, &archive).is_err(),
        "empty resident publication history cannot substitute for a nonzero V3 sealed prefix",
    );
    let bytes = projected.to_canonical_bytes().unwrap();
    let restored = crate::DirectSnowStage3V11AttachmentRestartV3::from_canonical_bytes(
        &bytes, context, &archive,
    )
    .unwrap()
    .restore(context, &archive)
    .unwrap();
    assert_eq!(restored.static_context, value.static_context);
    assert_eq!(restored.committed.stage3_by_lane, value.committed.stage3_by_lane);
    assert_eq!(
        restored.committed.v11_parent_state.checkpoint(),
        value.committed.v11_parent_state.checkpoint(),
    );
    assert_eq!(restored.committed.coupled_clock, value.committed.coupled_clock);
    assert_eq!(
        restored.committed.next_parent_sequence,
        value.committed.next_parent_sequence,
    );
    assert_eq!(restored.committed.terminal_parcels, value.committed.terminal_parcels);
    assert_eq!(restored.committed.receipt_chain, value.committed.receipt_chain);
    assert_eq!(
        restored.archived_receipt_prefix_v1(),
        value.archived_receipt_prefix_v1(),
    );
    assert_eq!(
        crate::DirectSnowStage3V11AttachmentRestartV3::project(
            &restored,
            phase_plan_sha256,
            day_input_digests,
            &archive,
        )
        .unwrap()
        .to_canonical_bytes()
        .unwrap(),
        bytes,
    );

    let missing_reader = EmptyStage3ArchiveReaderV3;
    assert!(
        projected
            .restore(
                context,
                &crate::ExpectedStage3CommittedDayArchiveV3 {
                    manifest: &manifest,
                    reader: &missing_reader,
                },
            )
            .is_err(),
        "missing nonzero archive content must reject",
    );
    let mut truncated = canonical.clone();
    truncated.pop().expect("nonempty archive content");
    let truncated_reader = InMemoryStage3ArchiveReaderV3 {
        content: BTreeMap::from([(entry.content_sha256, truncated)]),
    };
    assert!(
        projected
            .restore(
                context,
                &crate::ExpectedStage3CommittedDayArchiveV3 {
                    manifest: &manifest,
                    reader: &truncated_reader,
                },
            )
            .is_err(),
        "truncated nonzero archive content must reject",
    );
    let mut substituted = canonical;
    let last = substituted.last_mut().expect("nonempty archive content");
    *last ^= 1;
    let substituted_reader = InMemoryStage3ArchiveReaderV3 {
        content: BTreeMap::from([(entry.content_sha256, substituted)]),
    };
    assert!(
        projected
            .restore(
                context,
                &crate::ExpectedStage3CommittedDayArchiveV3 {
                    manifest: &manifest,
                    reader: &substituted_reader,
                },
            )
            .is_err(),
        "same-length substituted nonzero archive content must reject",
    );

    let wrong_root = empty_archive_manifest_v3(&value);
    assert!(
        projected
            .restore(
                context,
                &crate::ExpectedStage3CommittedDayArchiveV3 {
                    manifest: &wrong_root,
                    reader: &reader,
                },
            )
            .is_err(),
        "wrong-root archive manifest must reject",
    );
    let mut duplicated = manifest.clone();
    duplicated.entries.push(entry.clone());
    duplicated.committed_day_count += 1;
    assert!(
        projected
            .restore(
                context,
                &crate::ExpectedStage3CommittedDayArchiveV3 {
                    manifest: &duplicated,
                    reader: &reader,
                },
            )
            .is_err(),
        "duplicated archive manifest entry must reject",
    );
    let mut reordered = duplicated;
    reordered.entries[1].day_index = 1;
    reordered.entries.reverse();
    assert!(
        projected
            .restore(
                context,
                &crate::ExpectedStage3CommittedDayArchiveV3 {
                    manifest: &reordered,
                    reader: &reader,
                },
            )
            .is_err(),
        "reordered archive manifest entries must reject",
    );
}
