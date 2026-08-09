use crate::{
    DirectEvapotranspirationComputeInputs, DirectEvapotranspirationPmetInputs, DirectExecutorMode,
    DirectFrameExecutor, DirectHydrologyProjectionInputs, DirectInfiltrationDepressionInputs,
    DirectLanedActiveConfig, DirectLanedActiveLaneConfig, DirectLanedActiveMeshPolicy,
    DirectLanedActiveTraceDetailFilter, DirectLiquidInputInputs, DirectPercolationInputs,
    DirectPublicationCalendarDay, DirectPublicationDayInput, DirectPublicationRunMetadata,
    DirectRunFrame, DirectRunIdentity, DirectRuntimeError, DirectSubsurfaceComputeInputs,
    DirectSubsurfaceLayerInputs, DirectSubsurfaceLayerState, DirectWb14HyetographInterval,
    DirectWb14InfiltrationProducerInputs,
};

fn active_config(lane_count: usize, trace_enabled: bool) -> DirectLanedActiveConfig {
    DirectLanedActiveConfig {
        lanes: (0..lane_count)
            .map(|_| DirectLanedActiveLaneConfig {
                slplen_m: 10.0,
                width_m: 10.0,
                mean_gradient: 0.01,
                skin_friction_coefficient_ko: 500.0,
                form_drag_coefficient: 0.0,
                roughness_element_height_m: 0.0,
                roughness_concentration: 0.0,
                vegetation_drag_coefficient: 0.0,
                canopy_height_m: None,
            })
            .collect(),
        mesh_policy: DirectLanedActiveMeshPolicy::FixedCells { cells: 10 },
        max_dt_s: 300.0,
        trace_enabled,
        trace_detail_filter: None,
        step_trace_enabled: false,
    }
}

fn publication_day(theta_m: f64, wet: bool) -> DirectPublicationDayInput {
    let mut day = DirectPublicationDayInput::calendar_only(DirectPublicationCalendarDay {
        year: 2026,
        julian_day: 1,
        month: 1,
        day_of_month: 1,
        water_year: 2026,
    });
    let layer_inputs = DirectSubsurfaceLayerInputs {
        theta_m,
        field_capacity_m: 0.100,
        upper_limit_m: 0.500,
        conductivity_m_s: 1.0e-10,
        depth_m: 0.400,
        residual_theta: 0.0,
        frozen_depth_m: 0.0,
        frozen_water_m: 0.0,
        porosity: 0.5,
        field_capacity_theta: 0.25,
        coca: 1.0,
        lateral_conductivity_m_s: 1.0e-10,
    };
    day.precipitation_m = if wet { 0.050 } else { 0.0 };
    day.initial_soil_water_m = Some(theta_m);
    day.liquid_input_inputs = Some(DirectLiquidInputInputs {
        liquid_input_handoff_m: day.precipitation_m,
    });
    day.percolation_inputs = Some(DirectPercolationInputs {
        soil_water_initial_m: theta_m,
        reconcile_legacy_soil_water_from_layers: false,
        same_pass_infiltration_m: 0.0,
        same_pass_infiltration_lineage: false,
        tillage_depth_m: 0.0,
        lane_substeps: 24,
        restrictive_layer_enabled: false,
        restrictive_layer_conductivity_m_s: 0.0,
        restrictive_layer_thickness_m: 0.0,
        layers: vec![DirectSubsurfaceLayerState::from(layer_inputs.clone())],
    });
    day.subsurface_compute_inputs = Some(DirectSubsurfaceComputeInputs {
        avg_slope: if wet { 0.10 } else { 0.0 },
        slope_length_m: 1.0,
        lateral_anisotropy_ratio: 1.0,
        soil_depth_m: 0.400,
        solwpv_mode: 2006,
        mofe_hourly_carry_arrays_enabled: wet,
        lane_substeps: 24,
        drainage_capacity_m: 0.0,
        drain_enabled: false,
        drain_depth_m: 0.5,
        drain_spacing_m: 1.0,
        drain_diameter_m: 0.1,
        layers: vec![layer_inputs],
    });
    day.evapotranspiration_compute_inputs = Some(DirectEvapotranspirationComputeInputs {
        et_demand_m: 0.0,
        leaf_area_index: 0.0,
        canopy_height_m: 0.0,
        canopy_cover_fraction: 0.0,
        residue_interception_m: 0.0,
        same_pass_infiltration_m: 0.0,
        outside_water_depth_m: 0.0,
        root_depth_m: 0.0,
        plant_tolerance: 0.25,
        growth_context_required: false,
        stage_state: None,
        pmet: Some(DirectEvapotranspirationPmetInputs {
            soil_evaporation_m: 0.0,
            plant_transpiration_m: 0.0,
            soil_evaporation_storage_return_m: 0.0,
        }),
        pmet_compute: None,
    });
    day.hydrology_projection_inputs = Some(DirectHydrologyProjectionInputs {
        aggregate_storage_tolerance_m: 1.0e-12,
        snow_water_m: 0.0,
        frozen_soil_water_m: 0.0,
        frost_depth_m: 0.0,
        profile_depth_m: Some(0.400),
        profile_porosity_cap_m: Some(0.200),
        profile_field_capacity_m: Some(0.100),
        profile_wilting_point_m: Some(0.050),
    });
    day.infiltration_depression_inputs = Some(DirectInfiltrationDepressionInputs {
        cumulative_infiltration_handoff_m: 0.0,
        depression_storage_delta_handoff_m: 0.0,
        producer_inputs: Some(DirectWb14InfiltrationProducerInputs {
            hyetograph: vec![DirectWb14HyetographInterval {
                start_s: 0.0,
                end_s: 3_600.0,
                intensity_m_s: day.precipitation_m / 3_600.0,
            }],
            hourly_additional_supply_m: [0.0; 24],
            effective_conductivity_m_s: 1.0e-10,
            matric_potential_m: 0.0,
            storage_capacity_m: 0.0,
            depression_storage_capacity_m: 0.0,
        }),
    });
    day
}

fn metadata(name: &str) -> DirectPublicationRunMetadata {
    DirectPublicationRunMetadata {
        run_name: name.to_string(),
        runtime_selection: "laned-active".to_string(),
        output_policy: "test".to_string(),
    }
}

#[test]
fn cqr_laned_active_publication_rejects_non_linear_topology_before_input_build() {
    let identity = DirectRunIdentity::new(17, 501, 2, 1).expect("valid active identity");
    let mut frame = DirectRunFrame::skeleton(identity).expect("active frame");
    frame.laned_active = Some(Box::new(active_config(2, false)));
    frame.lanes[0].downstream_lane_id = 0;
    let mut input_builds = 0;
    let result = DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly)
        .run_publication_stream_with_interleaved_day_inputs_and_day_frames(
            &mut frame,
            metadata("cqr_active_bad_topology"),
            |_, _, _| {
                input_builds += 1;
                unreachable!("topology must fail before input construction")
            },
            |_, _| Ok(()),
        );
    assert!(matches!(
        result,
        Err(DirectRuntimeError::InvalidLaneTopology { lane_index: 0, .. })
    ));
    assert_eq!(input_builds, 0);
}

#[test]
fn cqr_laned_active_zero_source_publication_commits_summary_and_trace() {
    let identity = DirectRunIdentity::new(17, 501, 1, 1).expect("valid active identity");
    let mut frame = DirectRunFrame::skeleton(identity).expect("active frame");
    frame.lanes[0].area_m2 = 100.0;
    frame.laned_active = Some(Box::new(active_config(1, true)));
    let day_input = publication_day(0.700, false);
    let mut consumed = 0;
    let execution = DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly)
        .run_publication_stream_with_interleaved_day_inputs_and_day_frames(
            &mut frame,
            metadata("cqr_active_zero_source"),
            |_, _, _| Ok(day_input.clone()),
            |_, day_frame| {
                consumed += 1;
                assert_eq!(
                    day_frame.erosion_inputs.hydrograph_shape_authority,
                    crate::DirectErosionHydrographShapeAuthority::RoutedHydrograph
                );
                assert!(day_frame.laned_active_routing.is_some());
                Ok(())
            },
        )
        .expect("zero-source active stream");
    assert_eq!(execution.row_count, 1);
    assert_eq!(execution.report.day_frame_commit_count, 1);
    assert_eq!(consumed, 1);
    let summary = frame.laned_active_summary.as_ref().expect("active summary");
    assert_eq!(summary.days_seen, 1);
    assert_eq!(summary.days_routed, 0);
    assert_eq!(summary.trace_records.as_ref().map(Vec::len), Some(1));
}

#[test]
fn cqr_laned_active_positive_source_publication_routes_before_consumption() {
    let identity = DirectRunIdentity::new(17, 501, 1, 1).expect("valid active identity");
    let mut frame = DirectRunFrame::skeleton(identity).expect("active frame");
    frame.lanes[0].area_m2 = 100.0;
    let mut config = active_config(1, true);
    config.trace_detail_filter = Some(DirectLanedActiveTraceDetailFilter {
        day_index: 0,
        lane_index: 0,
    });
    config.step_trace_enabled = true;
    frame.laned_active = Some(Box::new(config));
    let day_input = publication_day(0.700, true);
    let execution = DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly)
        .run_publication_stream_with_interleaved_day_inputs_and_day_frames(
            &mut frame,
            metadata("cqr_active_positive_source"),
            |_, _, _| Ok(day_input.clone()),
            |_, day_frame| {
                let routing = day_frame
                    .laned_active_routing
                    .as_ref()
                    .expect("positive source routing evidence");
                assert!(routing.source_m3 > 0.0);
                assert!(
                    routing
                        .trace_detail
                        .as_ref()
                        .and_then(|detail| detail.step_trace.as_ref())
                        .is_some()
                );
                Ok(())
            },
        )
        .expect("positive-source active stream");
    assert_eq!(execution.row_count, 1);
    let summary = frame.laned_active_summary.as_ref().expect("active summary");
    assert_eq!(summary.days_seen, 1);
    assert_eq!(summary.days_routed, 1);
    assert!(summary.total_source_m3 > 0.0);
}
