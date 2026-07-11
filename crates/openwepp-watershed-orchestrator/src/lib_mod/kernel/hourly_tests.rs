#[cfg(test)]
mod hourly_tests {
    #![allow(clippy::too_many_lines)]
    use super::*;
    use super::direct_tests::{
        test_channel_control, test_hillslope_contribution, test_network_frame,
    };

    const EPS: f64 = 1.0e-12;

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() <= EPS,
            "actual={actual:.16e} expected={expected:.16e}"
        );
    }

    fn independent_rectangular_manning_area_m2(
        discharge_m3_s: f64,
        width_m: f64,
        roughness: f64,
        slope: f64,
    ) -> f64 {
        if discharge_m3_s <= 0.0 {
            return 0.0;
        }
        let capacity = |depth_m: f64| {
            let area_m2 = width_m * depth_m;
            let hydraulic_radius_m = area_m2 / (width_m + (2.0 * depth_m));
            area_m2 * hydraulic_radius_m.powf(2.0 / 3.0) * slope.sqrt() / roughness
        };
        let mut lower_m = 0.0;
        let mut upper_m = 1.0;
        while capacity(upper_m) < discharge_m3_s {
            upper_m *= 2.0;
        }
        for _ in 0..200 {
            let midpoint_m = 0.5 * (lower_m + upper_m);
            if capacity(midpoint_m) < discharge_m3_s {
                lower_m = midpoint_m;
            } else {
                upper_m = midpoint_m;
            }
        }
        width_m * 0.5 * (lower_m + upper_m)
    }

    fn geometry(depth: f64, width: f64) -> Ws11IntervalGeometry {
        Ws11IntervalGeometry {
            depth_a_points_ft: vec![depth, depth],
            depth_b_points_ft: vec![depth, depth],
            width_a_points_ft: vec![width, width],
            width_b_points_ft: vec![width, width],
            eroded_width_a_points_ft: vec![width, width],
            eroded_width_b_points_ft: vec![width, width],
        }
    }

    fn run_test_channel(
        frame: &WatershedNetworkFrame,
        node_id: u32,
        contributor_hillslopes: Vec<u32>,
        dependency_nodes: Vec<TopologyNodeKey>,
    ) -> RoutedChannelState {
        let step = DispatchStep {
            sequence_index: 0,
            node: TopologyNodeKey::new(TopologyNodeKind::Channel, node_id),
            dependency_nodes,
            contributor_hillslopes,
            status: Ws10ChannelImpoundmentKernel::direct_ok_status(TopologyNodeKind::Channel),
        };
        match Ws10ChannelImpoundmentKernel::run_direct_channel_node(
            &DirectWatershedKernelInput { step: &step, frame },
        )
        .unwrap_or_else(|error| panic!("channel {node_id} should route: {error:?}"))
        {
            DirectWatershedKernelOutput::Channel(state) => *state,
            DirectWatershedKernelOutput::Impoundment(_) => panic!("expected channel state"),
        }
    }

    #[test]
    fn wshedw11b_vector01_single_interval_uses_pinned_operands() {
        let mut hourly = [0.0_f64; 24];
        hourly[7] = 12.0;
        let projected = Ws10ChannelImpoundmentKernel::ws11_project_hourly_totals(
            &hourly, 3600.0, 24,
        )
        .expect("covering water grid should project");
        assert_close(projected[7], 12.0);
        assert_close(projected.iter().sum::<f64>(), 12.0);

        let operands = Ws10ChannelImpoundmentKernel::ws11_interval_hydraulic_operands(
            3.0, 1.0, 0.5, 100.0,
        )
        .expect("pinned profile should assemble");
        assert_close(operands.qe_m3_s, 3.0);
        assert_close(operands.qt_m3_s, 1.0);
        assert_close(operands.qlat_total_m3_s, 0.5);
        assert_close(operands.leff_ft, 300.0 * WS15_DEPTH_FROM_METERS_TO_FEET);
        assert_close(operands.qlat_eff_cfs_per_ft, operands.qe_cfs / operands.leff_ft);

        Ws10ChannelImpoundmentKernel::ws11_validate_interval_clock(3600.0, 3600.0, 3600.0)
            .expect("t_exp and t_norm both use dtchr");
        let ledger = Ws11IntervalMassLedger {
            inlet_kg: vec![3.0],
            lateral_kg: vec![9.0],
            detached_kg: vec![2.0],
            egress_kg: vec![10.0],
            deposited_kg: vec![4.0],
        };
        Ws10ChannelImpoundmentKernel::ws11_validate_interval_mass_closure(&ledger)
            .expect("single interval should close");
    }

    #[test]
    fn wshedw11b_vector02_interval_day_and_projection_close() {
        let mut hourly = [0.0_f64; 24];
        hourly[0] = 1.25;
        hourly[1] = 2.75;
        let projected = Ws10ChannelImpoundmentKernel::ws11_project_hourly_totals(
            &hourly, 900.0, 96,
        )
        .expect("quarter-hour grid should project");
        assert_close(projected.iter().sum::<f64>(), 4.0);
        for value in &projected[0..4] {
            assert_close(*value, 0.3125);
        }
        for value in &projected[4..8] {
            assert_close(*value, 0.6875);
        }

        let ledgers = vec![
            Ws11IntervalMassLedger {
                inlet_kg: vec![0.0],
                lateral_kg: vec![1.0],
                detached_kg: vec![0.5],
                egress_kg: vec![1.2],
                deposited_kg: vec![0.3],
            },
            Ws11IntervalMassLedger {
                inlet_kg: vec![1.2],
                lateral_kg: vec![3.0],
                detached_kg: vec![0.0],
                egress_kg: vec![3.5],
                deposited_kg: vec![0.7],
            },
        ];
        Ws10ChannelImpoundmentKernel::ws11_validate_daily_mass_closure(&ledgers, &[4.0])
            .expect("interval, daily, and projection closures should pass");
    }

    #[test]
    fn wshedw11b_vector03_geometry_carries_to_lower_flow_interval() {
        let start = geometry(0.5, 2.0);
        let widened = geometry(0.0, 2.5);
        let carried = Ws10ChannelImpoundmentKernel::ws11_advance_interval_geometry(
            &start, &widened, false,
        )
        .expect("first mutation should advance");
        let held = Ws10ChannelImpoundmentKernel::ws11_advance_interval_geometry(
            &carried,
            &carried,
            false,
        )
        .expect("lower flow should consume and retain carried geometry");
        assert_eq!(held, widened);
    }

    #[test]
    fn wshedw11b_vector04_widening_clock_widens_then_holds() {
        let current = geometry(0.0, 2.0);
        let widened = geometry(0.0, 2.2);
        let first = Ws10ChannelImpoundmentKernel::ws11_advance_interval_geometry(
            &current, &widened, false,
        )
        .expect("Wf above current should widen");
        let second = Ws10ChannelImpoundmentKernel::ws11_advance_interval_geometry(
            &first, &first, false,
        )
        .expect("Wf at current should hold");
        assert_eq!(first, second);
    }

    #[test]
    fn wshedw11b_vector05_zero_flow_deposits_everything() {
        let start = geometry(0.4, 2.0);
        let interval = Ws10ChannelImpoundmentKernel::ws11_zero_flow_interval(
            &[1.0, 2.0],
            &start,
        )
        .expect("zero-flow terminal should close");
        assert_eq!(interval.geometry_end, start);
        assert_eq!(interval.egress_kg, vec![0.0, 0.0]);
        assert_eq!(interval.deposited_kg, vec![1.0, 2.0]);
        assert_eq!(interval.detached_kg, vec![0.0, 0.0]);
    }

    #[test]
    fn wshedw11b_vector06_water_storage_has_zero_sediment_storage() {
        let disposition = Ws10ChannelImpoundmentKernel::ws11_grid_end_disposition(7.5)
            .expect("nonzero water storage is valid");
        assert_close(disposition.water_storage_m3, 7.5);
        assert_close(disposition.suspended_sediment_storage_kg, 0.0);

        let error = Ws10ChannelImpoundmentKernel::ws11_grid_end_disposition(-0.25)
            .expect_err("WSHED-W11D material negative hydraulic storage must fail typed");
        assert_eq!(error.message_id(), "WKERNEL-WS10-CHANNEL-E-003");
    }

    #[test]
    fn wshedw11b_vector07_only_geometry_crosses_day_boundary() {
        let end = geometry(0.1, 3.0);
        let next = Ws10ChannelImpoundmentKernel::ws11_cross_day_state(&end);
        assert_eq!(next.geometry, end);
        assert!(next.suspended_class_mass_kg.is_empty());
    }

    #[test]
    fn wshedw11b_vector08_primary_tillage_reseeds_only_shape_three() {
        let carried = geometry(0.0, 3.0);
        let input = geometry(0.5, 2.0);
        assert_eq!(
            Ws10ChannelImpoundmentKernel::ws11_apply_tillage_reseed(
                &carried, &input, 3, true,
            ),
            input
        );
        assert_eq!(
            Ws10ChannelImpoundmentKernel::ws11_apply_tillage_reseed(
                &carried, &input, 2, true,
            ),
            carried
        );
        assert_eq!(
            Ws10ChannelImpoundmentKernel::ws11_apply_tillage_reseed(
                &carried, &input, 3, false,
            ),
            carried
        );
    }

    #[test]
    fn wshedw11b_vector09_invalid_interval_states_fail_closed() {
        assert!(Ws10ChannelImpoundmentKernel::ws11_validate_interval_grid(1000.0, 86).is_err());
        assert!(Ws10ChannelImpoundmentKernel::ws11_validate_interval_clock(7200.0, 3600.0, 3600.0).is_err());
        assert!(Ws10ChannelImpoundmentKernel::ws11_validate_no_suspended_carry(&[0.1]).is_err());
        assert!(Ws10ChannelImpoundmentKernel::ws11_validate_active_lane_operand_mode(false).is_err());

        let current = geometry(0.2, 3.0);
        let reset = geometry(0.5, 2.0);
        assert!(Ws10ChannelImpoundmentKernel::ws11_advance_interval_geometry(
            &current, &reset, false,
        )
        .is_err());
    }

    #[test]
    fn wshedw11b_vector10_contact_budget_and_geometry_mass_are_constructive() {
        let budget = Ws10ChannelImpoundmentKernel::ws11_partition_contact_budget(
            3600.0, 900.0,
        )
        .expect("contact inside interval should partition");
        assert_close(budget.timpot_s, 900.0);
        assert_close(budget.timex_s, 2700.0);
        assert_close(budget.timpot_s + budget.timex_s, 3600.0);

        let mass = Ws10ChannelImpoundmentKernel::ws11_geometry_detachment_mass(
            2.0, 0.5, 0.25, 1.0, 9.6,
        )
        .expect("eroded geometry should produce mass");
        assert_close(mass, 12.0);
    }

    #[test]
    fn wshedw11b_vector11_hydraulic_profile_rejects_all_aliases() {
        let pinned = Ws10ChannelImpoundmentKernel::ws11_interval_hydraulic_operands(
            3.0, 1.0, 0.5, 100.0,
        )
        .expect("pinned profile should assemble");
        let event_peak_fraction = pinned.qe_cfs * 0.4 / pinned.leff_ft;
        let qin_anchored = pinned.qt_cfs / pinned.leff_ft;
        let raw_total = pinned.qlat_total_cfs;
        let total_over_lc = pinned.qlat_total_cfs / pinned.channel_length_ft;

        assert!((pinned.qt_m3_s + pinned.qlat_total_m3_s - pinned.qe_m3_s).abs() > EPS);
        assert!((pinned.qlat_eff_cfs_per_ft - event_peak_fraction).abs() > EPS);
        assert!((pinned.qlat_eff_cfs_per_ft - qin_anchored).abs() > EPS);
        assert!((pinned.qlat_eff_cfs_per_ft - raw_total).abs() > EPS);
        assert!((pinned.qlat_eff_cfs_per_ft - total_over_lc).abs() > EPS);
        assert!((total_over_lc - pinned.qe_cfs / pinned.leff_ft).abs() > EPS);
    }

    #[test]
    fn wshedw11b_two_channel_direct_consumer_reads_same_grid_class_egress() {
        let mut frame = test_network_frame();
        frame.routing_globals.ipeak = 3;
        frame.routing_globals.dtchr_seconds = 3600.0;
        frame.routing_globals.ntchr = 24.0;
        frame.routing_globals.cbase = 0.0;
        frame.routing_globals.nchnum = 0.0;

        let mut upstream_control = test_channel_control();
        upstream_control.node_id = 7;
        let mut downstream_control = upstream_control.clone();
        downstream_control.node_id = 8;
        frame.channel_controls.insert(7, upstream_control);
        frame.channel_controls.insert(8, downstream_control);

        let mut upstream_hillslope = test_hillslope_contribution();
        upstream_hillslope.hillslope_id = 3;
        upstream_hillslope.particle_diameter_m = vec![0.0002, 0.0005];
        upstream_hillslope.particle_flow_fraction = vec![0.25, 0.75];
        upstream_hillslope.hourly_runoff_volume_m3 = vec![0.0; 24];
        upstream_hillslope.hourly_sediment_mass_kg = vec![0.0; 24];
        upstream_hillslope.hourly_runoff_volume_m3[4] = 720.0;
        upstream_hillslope.hourly_sediment_mass_kg[4] = 4.0;
        frame.add_hillslope_contribution(upstream_hillslope);

        let upstream_step = DispatchStep {
            sequence_index: 0,
            node: TopologyNodeKey::new(TopologyNodeKind::Channel, 7),
            dependency_nodes: Vec::new(),
            contributor_hillslopes: vec![3],
            status: Ws10ChannelImpoundmentKernel::direct_ok_status(TopologyNodeKind::Channel),
        };
        let upstream = match Ws10ChannelImpoundmentKernel::run_direct_channel_node(
            &DirectWatershedKernelInput {
                step: &upstream_step,
                frame: &frame,
            },
        )
        .expect("upstream interval lane should route")
        {
            DirectWatershedKernelOutput::Channel(state) => *state,
            DirectWatershedKernelOutput::Impoundment(_) => panic!("expected channel"),
        };
        let upstream_interval = upstream
            .interval_sediment_state
            .as_ref()
            .expect("upstream must publish interval sediment");
        assert_eq!(upstream_interval.intervals.len(), 24);
        assert!(upstream_interval.daily_egress_kg.iter().sum::<f64>() > 0.0);
        frame.record_routed_channel_state(upstream);

        let mut downstream_hillslope = test_hillslope_contribution();
        downstream_hillslope.hillslope_id = 4;
        downstream_hillslope.particle_diameter_m = vec![0.0002, 0.0005];
        downstream_hillslope.particle_flow_fraction = vec![0.25, 0.75];
        downstream_hillslope.hourly_runoff_volume_m3 = vec![0.0; 24];
        downstream_hillslope.hourly_sediment_mass_kg = vec![0.0; 24];
        downstream_hillslope.hourly_runoff_volume_m3[9] = 360.0;
        downstream_hillslope.hourly_sediment_mass_kg[9] = 2.0;
        frame.add_hillslope_contribution(downstream_hillslope);

        let downstream_step = DispatchStep {
            sequence_index: 1,
            node: TopologyNodeKey::new(TopologyNodeKind::Channel, 8),
            dependency_nodes: vec![TopologyNodeKey::new(TopologyNodeKind::Channel, 7)],
            contributor_hillslopes: vec![4],
            status: Ws10ChannelImpoundmentKernel::direct_ok_status(TopologyNodeKind::Channel),
        };
        let downstream = match Ws10ChannelImpoundmentKernel::run_direct_channel_node(
            &DirectWatershedKernelInput {
                step: &downstream_step,
                frame: &frame,
            },
        )
        .expect("downstream interval lane should consume upstream state")
        {
            DirectWatershedKernelOutput::Channel(state) => *state,
            DirectWatershedKernelOutput::Impoundment(_) => panic!("expected channel"),
        };
        let upstream_water = frame.routed_channels[&7]
            .interval_water_state
            .as_ref()
            .expect("upstream water state");
        let downstream_water = downstream
            .interval_water_state
            .as_ref()
            .expect("downstream water state");
        assert_eq!(downstream_water.qin_m3_s, upstream_water.q1_m3_s);

        let downstream_sediment = downstream
            .interval_sediment_state
            .as_ref()
            .expect("downstream sediment state");
        for interval in 0..24 {
            assert_eq!(
                downstream_sediment.intervals[interval].inlet_kg,
                frame.routed_channels[&7]
                    .interval_sediment_state
                    .as_ref()
                    .expect("upstream sediment state")
                    .intervals[interval]
                    .egress_kg
            );
        }

        let upstream_state = &frame.routed_channels[&7];
        for state in [upstream_state, &downstream] {
            let water = state
                .interval_water_state
                .as_ref()
                .expect("interval water state");
            let independently_reconstructed_storage = water
                .qin_m3_s
                .iter()
                .zip(&water.qlat_total_m3_s)
                .zip(&water.q1_m3_s)
                .map(|((qin, qlat), q1)| (qin + qlat - q1) * water.dtchr_seconds)
                .sum::<f64>();
            assert_close(
                independently_reconstructed_storage,
                water.storage_change_m3.iter().sum::<f64>(),
            );
            assert_close(
                state.channel_inflow_m3 - state.channel_outflow_m3,
                state.channel_storage_m3,
            );
            let sediment = state
                .interval_sediment_state
                .as_ref()
                .expect("interval sediment state");
            for ledger in &sediment.intervals {
                for class in 0..ledger.egress_kg.len() {
                    assert_close(
                        ledger.inlet_kg[class]
                            + ledger.lateral_kg[class]
                            + ledger.detached_kg[class],
                        ledger.egress_kg[class] + ledger.deposited_kg[class],
                    );
                }
            }
            assert_close(
                state.sediment_yield_kg,
                sediment.daily_egress_kg.iter().sum::<f64>(),
            );
        }

        let upstream_sediment = upstream_state
            .interval_sediment_state
            .as_ref()
            .expect("upstream sediment state");
        let external_lateral = upstream_sediment.daily_lateral_kg.iter().sum::<f64>()
            + downstream_sediment.daily_lateral_kg.iter().sum::<f64>();
        let detached = upstream_sediment.daily_detached_kg.iter().sum::<f64>()
            + downstream_sediment.daily_detached_kg.iter().sum::<f64>();
        let deposited = upstream_sediment.daily_deposited_kg.iter().sum::<f64>()
            + downstream_sediment.daily_deposited_kg.iter().sum::<f64>();
        let terminal_egress = downstream_sediment.daily_egress_kg.iter().sum::<f64>();
        assert_close(external_lateral + detached, deposited + terminal_egress);
        assert!(terminal_egress > 0.0);
    }

    #[test]
    fn wshedw11d_wave_branches_publish_or_reject_inadmissible_mc_grids() {
        for ipeak in [3, 4, 5, 6] {
            let mut frame = test_network_frame();
            frame.routing_globals.ipeak = ipeak;
            frame.routing_globals.dtchr_seconds = 3600.0;
            frame.routing_globals.ntchr = 24.0;
            frame.routing_globals.cbase = 0.0;
            frame.routing_globals.nchnum = 0.0;
            let mut control = test_channel_control();
            control.node_id = 7;
            frame.channel_controls.insert(7, control);
            let mut contribution = test_hillslope_contribution();
            contribution.hillslope_id = 3;
            contribution.hourly_runoff_volume_m3 = vec![0.0; 24];
            contribution.hourly_sediment_mass_kg = vec![0.0; 24];
            contribution.hourly_runoff_volume_m3[6] = 720.0;
            if ipeak == 5 {
                contribution.hourly_runoff_volume_m3.fill(720.0);
            }
            contribution.hourly_sediment_mass_kg[6] = 4.0;
            frame.add_hillslope_contribution(contribution);
            let step = DispatchStep {
                sequence_index: 0,
                node: TopologyNodeKey::new(TopologyNodeKind::Channel, 7),
                dependency_nodes: Vec::new(),
                contributor_hillslopes: vec![3],
                status: Ws10ChannelImpoundmentKernel::direct_ok_status(
                    TopologyNodeKind::Channel,
                ),
            };
            let output = Ws10ChannelImpoundmentKernel::run_direct_channel_node(
                &DirectWatershedKernelInput {
                    step: &step,
                    frame: &frame,
                },
            );
            if ipeak != 3 {
                let Err(error) = output else {
                    panic!("W11C MC grids are numerically inadmissible");
                };
                assert_eq!(error.message_id(), "WKERNEL-WS10-CHANNEL-E-003");
                continue;
            }
            let output = output.expect("ipeak=3 KW grid should route");
            let DirectWatershedKernelOutput::Channel(state) = output else {
                panic!("ipeak=3 should publish channel state");
            };
            let water = state.interval_water_state.expect("wave grid");
            assert_eq!(water.q1_m3_s.len(), 24);
            let interval_count = f64::from(
                u32::try_from(water.q1_m3_s.len()).expect("test interval count fits u32"),
            );
            assert_close(water.dtchr_seconds * interval_count, 86_400.0);
            assert!(state.interval_sediment_state.is_some());
        }
    }

    #[test]
    fn wshedw11d_kw_terminal_storage_uses_every_spatial_node() {
        let mut frame = test_network_frame();
        frame.routing_globals.ipeak = 3;
        frame.routing_globals.dtchr_seconds = 1.0;
        frame.routing_globals.ntchr = 86_400.0;
        let mut control = test_channel_control();
        control.node_id = 7;
        control.segment_points[1].x_m = 12_000.0;
        frame.channel_controls.insert(7, control);
        let step = DispatchStep {
            sequence_index: 0,
            node: TopologyNodeKey::new(TopologyNodeKind::Channel, 7),
            dependency_nodes: Vec::new(),
            contributor_hillslopes: Vec::new(),
            status: Ws10ChannelImpoundmentKernel::direct_ok_status(TopologyNodeKind::Channel),
        };
        let input = DirectWatershedKernelInput {
            step: &step,
            frame: &frame,
        };
        let context = Ws10ChannelImpoundmentKernel::read_direct_channel_context(&input)
            .expect("synthetic KW context");
        let (routed, storage_m3) =
            Ws10ChannelImpoundmentKernel::ws11_route_baseline_wave_series(
                &context,
                &[0.0, 0.0],
                &[1.0, 1.0],
                None,
            )
            .expect("steady KW profile should route");

        // The 12-km reach forces the pinned 101-segment cap. A constant
        // one-cubic-metre-per-second lateral source preserves the fresh
        // linear steady profile q(is)=is/101 at every routed terminal.
        assert_eq!(routed.q1_m3_s.len(), 2);
        assert_close(routed.q1_m3_s[0], 1.0);
        assert_close(routed.q1_m3_s[1], 1.0);
        let width_m = context.control.segment_points[0].width_a_ft
            / WS15_DEPTH_FROM_METERS_TO_FEET;
        let expected_area_sum_m2 = (0..=101)
            .map(|segment| {
                independent_rectangular_manning_area_m2(
                    f64::from(segment) / 101.0,
                    width_m,
                    context.roughness,
                    context.control.segment_points[0].slope,
                )
            })
            .sum::<f64>();
        let expected_storage_m3 = expected_area_sum_m2 / 102.0 * context.channel_length;
        let boundary_mean_storage_m3 = 0.5
            * independent_rectangular_manning_area_m2(
                1.0,
                width_m,
                context.roughness,
                context.control.segment_points[0].slope,
            )
            * context.channel_length;
        let unrestricted_flux_residual_m3 = routed.storage_change_m3.iter().sum::<f64>();
        assert!((storage_m3 - expected_storage_m3).abs() <= 1.0e-8);
        assert!(
            (storage_m3 - boundary_mean_storage_m3).abs() > 1.0,
            "spatial mean must anti-alias the MC boundary mean"
        );
        assert!(
            (storage_m3 - unrestricted_flux_residual_m3).abs() > 1.0,
            "hydraulic storage must anti-alias the interval flux residual"
        );
        let boundary_ratio = storage_m3 / boundary_mean_storage_m3;
        assert!(boundary_ratio.is_finite());
        assert!((boundary_ratio - 1.0).abs() > 1.0e-3);
    }

    #[test]
    fn wshedw11d_fresh_storage_and_daily_volume_reconstruct_independently() {
        let mut frame = test_network_frame();
        frame.routing_globals.ipeak = 3;
        frame.routing_globals.dtchr_seconds = 3_600.0;
        frame.routing_globals.ntchr = 24.0;
        frame.routing_globals.cbase = 0.0;
        frame.routing_globals.nchnum = 0.0;
        let control = test_channel_control();
        frame.channel_controls.insert(7, control.clone());
        let mut contribution = test_hillslope_contribution();
        contribution.hourly_runoff_volume_m3 = vec![3_600.0; 24];
        contribution.hourly_sediment_mass_kg = vec![0.0; 24];
        frame.add_hillslope_contribution(contribution);

        let state = run_test_channel(&frame, 7, vec![3], Vec::new());
        let water = state
            .interval_water_state
            .as_ref()
            .expect("KW water state should publish");
        let width_m = control.segment_points[0].width_a_ft / WS15_DEPTH_FROM_METERS_TO_FEET;
        let area_at_one_m2 = independent_rectangular_manning_area_m2(
            1.0,
            width_m,
            control.chnn,
            control.segment_points[0].slope,
        );
        let channel_length_m = control.segment_points[1].x_m;
        let expected_initial_storage_m3 = 0.5 * area_at_one_m2 * channel_length_m;
        let terminal_q1_m3_s = *water.q1_m3_s.last().expect("covering terminal grid");
        let expected_final_storage_m3 = 0.5
            * independent_rectangular_manning_area_m2(
                terminal_q1_m3_s,
                width_m,
                control.chnn,
                control.segment_points[0].slope,
            )
            * channel_length_m;
        let external_inflow_m3 = 24.0 * 3_600.0;
        let expected_outlet_m3 =
            external_inflow_m3 + expected_initial_storage_m3 - expected_final_storage_m3;

        assert!((water.initial_storage_m3 - expected_initial_storage_m3).abs() <= 1.0e-9);
        assert!((water.final_storage_m3 - expected_final_storage_m3).abs() <= 1.0e-9);
        assert!((state.channel_outflow_m3 - expected_outlet_m3).abs() <= 1.0e-9);
        assert!((state.channel_inflow_m3
            - (external_inflow_m3 + expected_initial_storage_m3))
            .abs()
            <= 1.0e-9);
        assert!((state.channel_storage_m3 - expected_final_storage_m3).abs() <= 1.0e-9);
        assert!(
            (water.storage_change_m3.iter().sum::<f64>() - expected_final_storage_m3).abs()
                > 1.0,
            "the unrestricted flux residual must not alias hydraulic storage"
        );
    }

    #[test]
    fn wshedw11d_last_projected_slot_reaches_last_terminal_at_both_timesteps() {
        for (dtchr_seconds, ntchr) in [(3_600.0, 24_usize), (600.0, 144_usize)] {
            let mut frame = test_network_frame();
            frame.routing_globals.ipeak = 3;
            frame.routing_globals.dtchr_seconds = dtchr_seconds;
            frame.routing_globals.ntchr =
                f64::from(u32::try_from(ntchr).expect("test grid fits u32"));
            let mut control = test_channel_control();
            control.node_id = 7;
            frame.channel_controls.insert(7, control);
            let step = DispatchStep {
                sequence_index: 0,
                node: TopologyNodeKey::new(TopologyNodeKind::Channel, 7),
                dependency_nodes: Vec::new(),
                contributor_hillslopes: Vec::new(),
                status: Ws10ChannelImpoundmentKernel::direct_ok_status(
                    TopologyNodeKind::Channel,
                ),
            };
            let input = DirectWatershedKernelInput {
                step: &step,
                frame: &frame,
            };
            let context = Ws10ChannelImpoundmentKernel::read_direct_channel_context(&input)
                .expect("synthetic KW context");
            let qin_m3_s = vec![0.0; ntchr];
            let mut qlat_m3_s = vec![0.0; ntchr];
            qlat_m3_s[ntchr - 1] = 1.0;
            let (routed, final_storage_m3) =
                Ws10ChannelImpoundmentKernel::ws11_route_baseline_wave_series(
                    &context,
                    &qin_m3_s,
                    &qlat_m3_s,
                    None,
                )
                .expect("final-slot KW pulse should route");

            assert_eq!(routed.q1_m3_s.len(), ntchr);
            assert!(
                routed.q1_m3_s[..ntchr - 1]
                    .iter()
                    .all(|value| value.abs() <= EPS)
            );
            assert!(
                routed.q1_m3_s[ntchr - 1] > 0.0,
                "dtchr={dtchr_seconds} final forcing slot must reach terminal ntchr"
            );
            assert!(final_storage_m3 > 0.0);
        }
    }

    #[test]
    fn wshedw11b_wave_reference_flow_keeps_kw_and_mc_branches_distinct() {
        let qtmax = 4.0;
        let kw = Ws10ChannelImpoundmentKernel::ws11_wave_reference_flow(
            Ws11IpeakBranch::KinematicWave,
            qtmax,
        )
        .expect("KW qref");
        let static_mc = Ws10ChannelImpoundmentKernel::ws11_wave_reference_flow(
            Ws11IpeakBranch::MuskingumCunge,
            qtmax,
        )
        .expect("static MC qref");
        let dynamic_mc = Ws10ChannelImpoundmentKernel::ws11_wave_reference_flow(
            Ws11IpeakBranch::MuskingumCungeVariable,
            qtmax,
        )
        .expect("dynamic MC qref");
        assert_close(kw, 4.0);
        assert_close(static_mc, 2.0);
        assert_close(dynamic_mc, 2.0);
    }

    #[test]
    fn wshedw11d_mc_coefficients_enforce_convex_passive_recurrence() {
        let mut frame = test_network_frame();
        frame.routing_globals.ipeak = 4;
        frame.routing_globals.dtchr_seconds = 200.0;
        frame.routing_globals.ntchr = 432.0;
        let mut control = test_channel_control();
        control.node_id = 7;
        frame.channel_controls.insert(7, control);
        let step = DispatchStep {
            sequence_index: 0,
            node: TopologyNodeKey::new(TopologyNodeKind::Channel, 7),
            dependency_nodes: Vec::new(),
            contributor_hillslopes: Vec::new(),
            status: Ws10ChannelImpoundmentKernel::direct_ok_status(TopologyNodeKind::Channel),
        };
        let input = DirectWatershedKernelInput {
            step: &step,
            frame: &frame,
        };
        let mut context = Ws10ChannelImpoundmentKernel::read_direct_channel_context(&input)
            .expect("synthetic MC context");

        let error = Ws10ChannelImpoundmentKernel::ws11_route_muskingum_segment(
            &context, 1.0, 10.0, 5.0, 0.01, 100.0, 0.0, 2.0, 1.0, 1.0,
        )
        .expect_err("dt=200 produces a materially negative c3");
        assert_eq!(error.message_id(), "WKERNEL-WS10-CHANNEL-E-003");

        context.dtchr = 100.0;
        let (q1, coefficients) =
            Ws10ChannelImpoundmentKernel::ws11_route_muskingum_segment(
                &context, 1.0, 10.0, 5.0, 0.01, 100.0, 0.0, 2.0, 1.0, 1.0,
            )
            .expect("dt=100 gives a convex MC recurrence");
        assert_close(coefficients[1], 0.2);
        assert_close(coefficients[2], 0.6);
        assert_close(coefficients[3], 0.2);
        assert_close(coefficients[1] + coefficients[2] + coefficients[3], 1.0);
        assert_close(q1, 1.2);
        assert!(q1 <= 2.0 + EPS, "passive route cannot amplify its source maximum");
    }

    #[test]
    fn wshedw11d_admissible_static_and_dynamic_mc_execute_full_route() {
        let mut branch_outputs = Vec::new();
        for ipeak in [4, 5] {
            let mut frame = test_network_frame();
            frame.routing_globals.ipeak = ipeak;
            frame.routing_globals.dtchr_seconds = 60.0;
            frame.routing_globals.ntchr = 1_440.0;
            frame.routing_globals.cbase = 0.0;
            frame.routing_globals.nchnum = 0.0;
            let mut control = test_channel_control();
            control.node_id = 7;
            control.segment_points[1].x_m = 100.0;
            frame.channel_controls.insert(7, control);
            let mut contribution = test_hillslope_contribution();
            contribution.hourly_runoff_volume_m3 = vec![3_600.0; 24];
            contribution.hourly_runoff_volume_m3[6] = 3_960.0;
            contribution.hourly_sediment_mass_kg = vec![0.0; 24];
            frame.add_hillslope_contribution(contribution);

            let state = run_test_channel(&frame, 7, vec![3], Vec::new());
            let water = state
                .interval_water_state
                .as_ref()
                .expect("admissible MC should publish its full interval grid");
            assert_eq!(water.q1_m3_s.len(), 1_440);
            assert!(water.q1_m3_s.iter().all(|value| value.is_finite() && *value >= 0.0));
            assert!(state.peak_discharge_m3_s > 0.0);
            assert!(
                state.peak_discharge_m3_s <= 1.1 + 1.0e-12,
                "admissible passive MC route cannot amplify the inlet maximum"
            );
            assert_close(
                state.channel_inflow_m3,
                state.channel_outflow_m3 + state.channel_storage_m3,
            );
            let representative = state.wave_state.expect("MC coefficients should publish");
            assert!(representative.c1 >= -EPS);
            assert!(representative.c2 >= -EPS);
            assert!(representative.c3 >= -EPS);
            assert_close(representative.c1 + representative.c2 + representative.c3, 1.0);
            branch_outputs.push((representative, water.q1_m3_s.clone()));
        }
        let coefficient_delta = (branch_outputs[0].0.c1 - branch_outputs[1].0.c1).abs()
            + (branch_outputs[0].0.c2 - branch_outputs[1].0.c2).abs()
            + (branch_outputs[0].0.c3 - branch_outputs[1].0.c3).abs();
        let hydrograph_delta = branch_outputs[0]
            .1
            .iter()
            .zip(&branch_outputs[1].1)
            .map(|(static_q1, dynamic_q1)| (static_q1 - dynamic_q1).abs())
            .fold(0.0_f64, f64::max);
        assert!(coefficient_delta > 1.0e-9, "dynamic coefficients must refresh");
        assert!(hydrograph_delta > 1.0e-9, "matched static/dynamic routes must diverge");
    }

    #[test]
    fn wshedw11b_wave_epsilon_floor_is_an_outlet_boundary_only() {
        let material_negative =
            Ws10ChannelImpoundmentKernel::ws11_wave_outlet_discharge(-0.1).expect("finite outlet");
        let sub_epsilon = Ws10ChannelImpoundmentKernel::ws11_wave_outlet_discharge(0.5e-8)
            .expect("finite outlet");
        let retained = Ws10ChannelImpoundmentKernel::ws11_wave_outlet_discharge(2.0e-8)
            .expect("finite outlet");
        assert_close(material_negative, 0.0);
        assert_close(sub_epsilon, 0.0);
        assert_close(retained, 2.0e-8);

        // The segment helper itself returns finite raw spatial state. The
        // caller invokes the outlet boundary helper only for `nseg`.
        let source = include_str!("hourly.rs");
        let segment_start = source
            .find("fn ws11_route_muskingum_segment")
            .expect("MC segment helper");
        let segment_end = source[segment_start..]
            .find("fn ws11_local_channel_baseflow")
            .map(|offset| segment_start + offset)
            .expect("next helper after MC segment");
        assert!(!source[segment_start..segment_end].contains("ws11_wave_outlet_discharge"));
        assert!(source[..segment_start]
            .contains("ws11_wave_outlet_discharge(current_spatial[nseg])"));

        let gate_start = source
            .find("let mc_update_active")
            .expect("MC qmaxi/qlavg gate");
        let gate_end = source[gate_start..]
            .find("let mut outlet_coefficients")
            .map(|offset| gate_start + offset)
            .expect("segment update after MC gate");
        let gate = &source[gate_start..gate_end];
        for operand in [
            "previous_qin",
            "previous_spatial[nseg]",
            "qin_m3_s[interval]",
            "qlat_per_m",
        ] {
            assert!(gate.contains(operand), "missing MC zero-gate operand {operand}");
        }
        assert!(gate.contains("q1_m3_s.push(0.0)"));
        assert!(gate.contains("previous_spatial = current_spatial"));
        assert!(gate.contains("continue"));
    }

    #[test]
    fn wshedw11b_production_water_route_keeps_qin_and_qlat_distinct() {
        let mut frame = test_network_frame();
        frame.routing_globals.ipeak = 3;
        frame.routing_globals.dtchr_seconds = 3600.0;
        frame.routing_globals.ntchr = 24.0;
        frame.routing_globals.cbase = 0.0;
        frame.routing_globals.nchnum = 0.0;
        let control = test_channel_control();
        frame.channel_controls.insert(7, control.clone());
        let mut contribution = test_hillslope_contribution();
        contribution.hillslope_id = 3;
        contribution.hourly_runoff_volume_m3 = vec![0.0; 24];
        contribution.hourly_sediment_mass_kg = vec![0.0; 24];
        contribution.hourly_runoff_volume_m3[6] = 720.0;
        frame.add_hillslope_contribution(contribution);

        let state = run_test_channel(&frame, 7, vec![3], Vec::new());
        let water = state.interval_water_state.expect("active water state");
        assert_close(water.qin_m3_s[6], 0.0);
        assert_close(water.qlat_total_m3_s[6], 0.2);
        // Pinned `mofapp=1` routes the adjacent-state average after dividing
        // the total lateral rate by reach length. With a dry one-segment
        // front this is `0.5 * (0 + 0.2) / 12 * 12 = 0.1 m3/s`.
        assert_close(water.q1_m3_s[6], 0.1);
        assert!((water.q1_m3_s[6] - water.qlat_total_m3_s[6]).abs() > 1.0e-6);
        assert!(water.q1_m3_s[7] > 0.0);

        let sediment = state
            .interval_sediment_state
            .expect("active sediment state");
        let interval = &sediment.intervals[6];
        let hydraulic = interval.hydraulic.expect("flowing profile operands");
        assert_close(hydraulic.qe_m3_s, water.q1_m3_s[6]);
        assert_close(hydraulic.qt_m3_s, water.qin_m3_s[6]);
        assert_close(hydraulic.qlat_total_m3_s, water.qlat_total_m3_s[6]);
        assert_close(
            hydraulic.qlat_eff_cfs_per_ft,
            hydraulic.qe_m3_s * WS18_CFS_PER_CMS / hydraulic.leff_ft,
        );
        let raw_total_cfs = hydraulic.qlat_total_m3_s * WS18_CFS_PER_CMS;
        let total_over_lc = raw_total_cfs
            / (12.0 * WS15_DEPTH_FROM_METERS_TO_FEET);
        assert!((hydraulic.qlat_eff_cfs_per_ft - raw_total_cfs).abs() > EPS);
        assert!((hydraulic.qlat_eff_cfs_per_ft - total_over_lc).abs() > EPS);
        assert!(interval.max_effective_shear_lb_ft2.is_finite());
        assert!(interval.max_effective_shear_lb_ft2 >= 0.0);
        assert_eq!(
            interval.outlet_transport_capacity_kg_s.len(),
            sediment.particle_diameter_m.len()
        );
        assert!(interval
            .outlet_transport_capacity_kg_s
            .iter()
            .all(|value| value.is_finite() && *value >= 0.0));
    }

    #[test]
    fn wshedw11b_vector01_production_owner_matches_one_pinned_core_call() {
        let mut frame = test_network_frame();
        frame.routing_globals.ipeak = 3;
        frame.routing_globals.dtchr_seconds = 3600.0;
        frame.routing_globals.ntchr = 24.0;
        frame.routing_globals.cbase = 0.0;
        frame.routing_globals.nchnum = 0.0;
        let control = test_channel_control();
        frame.channel_controls.insert(7, control.clone());
        let mut contribution = test_hillslope_contribution();
        contribution.hillslope_id = 3;
        contribution.hourly_runoff_volume_m3 = vec![0.0; 24];
        contribution.hourly_sediment_mass_kg = vec![0.0; 24];
        contribution.hourly_sediment_mass_kg[6] = 4.0;
        frame.add_hillslope_contribution(contribution);
        let step = DispatchStep {
            sequence_index: 0,
            node: TopologyNodeKey::new(TopologyNodeKind::Channel, 7),
            dependency_nodes: Vec::new(),
            contributor_hillslopes: vec![3],
            status: Ws10ChannelImpoundmentKernel::direct_ok_status(TopologyNodeKind::Channel),
        };
        let input = DirectWatershedKernelInput {
            step: &step,
            frame: &frame,
        };
        let context = Ws10ChannelImpoundmentKernel::read_direct_channel_context(&input)
            .expect("channel context");
        let mut q1_m3_s = vec![0.0; 24];
        let mut qin_m3_s = vec![0.0; 24];
        let mut qlat_total_m3_s = vec![0.0; 24];
        q1_m3_s[6] = 3.0;
        qin_m3_s[6] = 1.0;
        qlat_total_m3_s[6] = 0.5;
        let water = RoutedChannelIntervalWaterState {
            dtchr_seconds: 3600.0,
            initial_storage_m3: 0.0,
            final_storage_m3: 0.0,
            storage_change_m3: qin_m3_s
                .iter()
                .zip(&qlat_total_m3_s)
                .zip(&q1_m3_s)
                .map(|((qin, qlat), q1)| (qin + qlat - q1) * 3600.0)
                .collect(),
            qin_m3_s,
            qlat_total_m3_s,
            q1_m3_s,
        };
        let routed = Ws10ChannelImpoundmentKernel::ws11_route_interval_sediment(
            &input, &context, &water, 24,
        )
        .expect("one-interval production owner");
        let ledger = &routed.intervals[6];
        let operands = Ws10ChannelImpoundmentKernel::ws11_interval_hydraulic_operands(
            3.0,
            1.0,
            0.5,
            context.channel_length,
        )
        .expect("pinned operands");
        let mut profile = Ws10ChannelImpoundmentKernel::direct_ws20_channel_profile(
            &control,
            context.nslpts,
        )
        .expect("profile");
        for x in &mut profile.x_points_ft {
            *x *= WS15_DEPTH_FROM_METERS_TO_FEET;
        }
        let original_length_ft = *profile.x_points_ft.last().expect("profile length");
        for x in &mut profile.x_points_ft {
            *x = *x / original_length_ft * operands.leff_ft;
        }
        let class_numbers = (1..=routed.particle_diameter_m.len()).collect::<Vec<_>>();
        let crfrac = Ws10ChannelImpoundmentKernel::direct_ws20_crfrac(
            &control,
            &class_numbers,
        )
        .expect("channel fractions");
        let core = Ws10ChannelImpoundmentKernel::ws20_route_case12_segment_family_core(
            7,
            Ws10NodeClass::Channel,
            true,
            3600.0,
            3.0,
            context.roughness,
            context.sediment_controls,
            context.nslpts,
            Ws20IncomingPeakPartition {
                hillslope_peak_cms: 0.0,
                dependency_peak_cms: 0.0,
                hillslope_volume_m3: 0.0,
                dependency_volume_m3: 0.0,
                hillslope_duration_s: 0.0,
                dependency_duration_s: 0.0,
                hourly_resolved: true,
                hourly_sediment_inlet_kg: [0.0; 24],
            },
            Some(operands),
            Some(3600.0),
            3600.0,
            &ledger.inlet_kg,
            &ledger.lateral_kg,
            &routed.particle_diameter_m,
            &class_numbers,
            profile,
            control.chnk,
            Some(&crfrac),
        )
        .expect("single pinned core call");
        assert_eq!(ledger.egress_kg, core.routed_class_masses_kg);
        assert_eq!(ledger.detached_kg, core.detached_class_masses_kg);
        assert_eq!(ledger.deposited_kg, core.deposited_class_masses_kg);
        assert_close(
            ledger.max_effective_shear_lb_ft2,
            core.max_effective_shear_lb_ft2,
        );
        assert_eq!(
            ledger.outlet_transport_capacity_kg_s,
            core.outlet_transport_capacity_kg_s
        );
    }

    #[test]
    fn wshedw11d_kinematic_wave_advances_first_interval_from_prior_q1() {
        let mut frame = test_network_frame();
        frame.routing_globals.ipeak = 3;
        frame.routing_globals.dtchr_seconds = 3600.0;
        frame.routing_globals.ntchr = 24.0;
        frame.routing_globals.cbase = 0.0;
        frame.routing_globals.nchnum = 0.0;
        let control = test_channel_control();
        frame.channel_controls.insert(7, control.clone());
        let mut contribution = test_hillslope_contribution();
        contribution.hourly_runoff_volume_m3 = vec![0.0; 24];
        contribution.hourly_sediment_mass_kg = vec![0.0; 24];
        contribution.hourly_runoff_volume_m3[23] = 720.0;
        frame.add_hillslope_contribution(contribution.clone());
        let first = run_test_channel(&frame, 7, vec![3], Vec::new());
        let prior_q1 = *first
            .interval_water_state
            .as_ref()
            .expect("first day water")
            .q1_m3_s
            .last()
            .expect("covering grid");
        let prior_storage_m3 = first
            .interval_water_state
            .as_ref()
            .expect("first day water")
            .final_storage_m3;
        assert!(prior_q1 > 0.0);
        assert!(prior_storage_m3 > 0.0);
        frame.record_routed_channel_state(first);
        contribution.hourly_runoff_volume_m3.fill(0.0);
        contribution.hourly_runoff_volume_m3[0] = 7_200.0;
        frame.add_hillslope_contribution(contribution);
        let second = run_test_channel(&frame, 7, vec![3], Vec::new());
        let next_q1 = second
            .interval_water_state
            .as_ref()
            .expect("second day water")
            .q1_m3_s[0];
        let second_water = second
            .interval_water_state
            .as_ref()
            .expect("second day water");
        // Pinned `wshchr.for:307,397-448` seeds the time-zero boundary from
        // yesterday and then advances `it=1`; the first published terminal
        // must not alias the seed when today's first forcing differs.
        assert!(
            next_q1 > prior_q1,
            "first interval must advance from the prior seed under new forcing: next={next_q1:.16e} prior={prior_q1:.16e}"
        );
        assert_close(second_water.initial_storage_m3, prior_storage_m3);
        assert_close(
            prior_storage_m3 + 7_200.0,
            second.channel_outflow_m3 + second_water.final_storage_m3,
        );
    }

    #[test]
    fn wshedw11d_zero_peak_retains_available_carried_storage() {
        let mut frame = test_network_frame();
        frame.routing_globals.ipeak = 3;
        frame.routing_globals.dtchr_seconds = 3600.0;
        frame.routing_globals.ntchr = 24.0;
        frame.routing_globals.cbase = 0.0;
        frame.routing_globals.nchnum = 0.0;
        frame.channel_controls.insert(7, test_channel_control());
        let mut contribution = test_hillslope_contribution();
        contribution.hourly_runoff_volume_m3 = vec![0.0; 24];
        contribution.hourly_sediment_mass_kg = vec![0.0; 24];
        frame.add_hillslope_contribution(contribution);

        let mut prior = run_test_channel(&frame, 7, vec![3], Vec::new());
        let water = prior
            .interval_water_state
            .as_mut()
            .expect("wave state should exist");
        water.q1_m3_s.fill(0.0);
        water.qin_m3_s.fill(0.0);
        water.qlat_total_m3_s.fill(0.0);
        water.final_storage_m3 = 7.5;
        prior.channel_storage_m3 = 7.5;
        frame.record_routed_channel_state(prior);

        let routed = run_test_channel(&frame, 7, vec![3], Vec::new());
        let water = routed
            .interval_water_state
            .as_ref()
            .expect("wave state should exist");
        assert_close(routed.peak_discharge_m3_s, 0.0);
        assert_close(routed.channel_outflow_m3, 0.0);
        assert_close(routed.channel_inflow_m3, 7.5);
        assert_close(routed.channel_storage_m3, 7.5);
        assert_close(water.initial_storage_m3, 7.5);
        assert_close(water.final_storage_m3, 7.5);
    }

    #[test]
    fn wshedw11b_prior_day_wave_seed_rejects_nonfinite_and_negative_state() {
        let mut frame = test_network_frame();
        frame.routing_globals.ipeak = 3;
        frame.routing_globals.dtchr_seconds = 3600.0;
        frame.routing_globals.ntchr = 24.0;
        let control = test_channel_control();
        frame.channel_controls.insert(7, control);
        let mut contribution = test_hillslope_contribution();
        contribution.hourly_runoff_volume_m3 = vec![0.0; 24];
        contribution.hourly_sediment_mass_kg = vec![0.0; 24];
        frame.add_hillslope_contribution(contribution);
        let seed = run_test_channel(&frame, 7, vec![3], Vec::new());
        frame.record_routed_channel_state(seed);
        let step = DispatchStep {
            sequence_index: 0,
            node: TopologyNodeKey::new(TopologyNodeKind::Channel, 7),
            dependency_nodes: Vec::new(),
            contributor_hillslopes: vec![3],
            status: Ws10ChannelImpoundmentKernel::direct_ok_status(TopologyNodeKind::Channel),
        };

        let mut nonfinite = frame.clone();
        *nonfinite
            .routed_channels
            .get_mut(&7)
            .and_then(|state| state.interval_water_state.as_mut())
            .and_then(|water| water.q1_m3_s.last_mut())
            .expect("terminal q1") = f64::NAN;
        let nonfinite_result = Ws10ChannelImpoundmentKernel::run_direct_channel_node(
            &DirectWatershedKernelInput {
                step: &step,
                frame: &nonfinite,
            },
        );
        let Err(nonfinite_error) = nonfinite_result else {
            panic!("nonfinite prior q1 must fail");
        };
        assert_eq!(nonfinite_error.message_id(), WS10_CHANNEL_GUARD_NON_FINITE);

        let mut negative = frame;
        *negative
            .routed_channels
            .get_mut(&7)
            .and_then(|state| state.interval_water_state.as_mut())
            .and_then(|water| water.qin_m3_s.last_mut())
            .expect("terminal qin") = -0.1;
        let negative_result = Ws10ChannelImpoundmentKernel::run_direct_channel_node(
            &DirectWatershedKernelInput {
                step: &step,
                frame: &negative,
            },
        );
        let Err(negative_error) = negative_result else {
            panic!("negative prior qin must fail");
        };
        assert_eq!(negative_error.message_id(), WS10_CHANNEL_GUARD_DOMAIN);
    }

    #[test]
    fn wshedw11b_production_baseflow_is_external_once_across_two_channels() {
        let mut frame = test_network_frame();
        frame.routing_globals.ipeak = 3;
        frame.routing_globals.dtchr_seconds = 3600.0;
        frame.routing_globals.ntchr = 24.0;
        frame
            .configure_groundwater_baseflow_routing(
                WatershedGroundwaterRoutingAuthority::linear_reservoir(0.0)
                    .expect("zero threshold is valid"),
            );
        let mut upstream_control = test_channel_control();
        upstream_control.node_id = 7;
        let mut downstream_control = upstream_control.clone();
        downstream_control.node_id = 8;
        frame.channel_controls.insert(7, upstream_control);
        frame.channel_controls.insert(8, downstream_control);

        let mut upstream_hillslope = test_hillslope_contribution();
        upstream_hillslope.hillslope_id = 3;
        upstream_hillslope.generated_baseflow_m3 = 864.0;
        upstream_hillslope.hourly_runoff_volume_m3 = vec![0.0; 24];
        upstream_hillslope.hourly_sediment_mass_kg = vec![0.0; 24];
        frame.add_hillslope_contribution(upstream_hillslope);
        let upstream = run_test_channel(&frame, 7, vec![3], Vec::new());
        frame.record_routed_channel_state(upstream);

        let mut downstream_hillslope = test_hillslope_contribution();
        downstream_hillslope.hillslope_id = 4;
        downstream_hillslope.generated_baseflow_m3 = 0.0;
        downstream_hillslope.hourly_runoff_volume_m3 = vec![0.0; 24];
        downstream_hillslope.hourly_sediment_mass_kg = vec![0.0; 24];
        frame.add_hillslope_contribution(downstream_hillslope);
        let downstream = run_test_channel(
            &frame,
            8,
            vec![4],
            vec![TopologyNodeKey::new(TopologyNodeKind::Channel, 7)],
        );
        let upstream_water = frame.routed_channels[&7]
            .interval_water_state
            .as_ref()
            .expect("upstream water");
        let downstream_water = downstream
            .interval_water_state
            .as_ref()
            .expect("downstream water");
        assert_eq!(downstream_water.qin_m3_s, upstream_water.q1_m3_s);
        assert!(downstream_water
            .qlat_total_m3_s
            .iter()
            .all(|value| value.abs() <= EPS));
        let external_water_m3 = 864.0;
        let initial_storage_m3 = upstream_water.initial_storage_m3
            + downstream_water.initial_storage_m3;
        let final_storage_m3 = upstream_water.final_storage_m3
            + downstream_water.final_storage_m3;
        assert_close(
            external_water_m3 + initial_storage_m3,
            downstream.channel_outflow_m3 + final_storage_m3,
        );
        assert_close(
            frame.routed_channels[&7].channel_storage_m3,
            upstream_water.final_storage_m3,
        );
        assert_close(
            downstream.channel_storage_m3,
            downstream_water.final_storage_m3,
        );
    }

    #[test]
    fn wshedw11b_production_geometry_carries_and_primary_tillage_reseeds() {
        let mut frame = test_network_frame();
        frame.routing_globals.ipeak = 3;
        frame.routing_globals.dtchr_seconds = 3600.0;
        frame.routing_globals.ntchr = 24.0;
        frame.routing_globals.cbase = 0.0;
        frame.routing_globals.nchnum = 0.0;
        let mut control = test_channel_control();
        control.ishape = 3;
        control.chntcr = 1.0e-9;
        control.chnk = 0.5;
        frame.channel_controls.insert(7, control.clone());
        let mut contribution = test_hillslope_contribution();
        contribution.hillslope_id = 3;
        contribution.hourly_runoff_volume_m3 = vec![0.0; 24];
        contribution.hourly_sediment_mass_kg = vec![0.0; 24];
        contribution.hourly_runoff_volume_m3[4] = 360_000.0;
        frame.add_hillslope_contribution(contribution.clone());

        let first = run_test_channel(&frame, 7, vec![3], Vec::new());
        let first_sediment = first
            .interval_sediment_state
            .as_ref()
            .expect("first-day geometry");
        assert_ne!(first_sediment.geometry_end, first_sediment.geometry_start);
        assert!(
            first_sediment.daily_detached_kg.iter().sum::<f64>() > 0.0,
            "geometry mutation must publish constructive detachment: {:?}",
            first_sediment.daily_detached_kg
        );

        frame.record_routed_channel_state(first.clone());
        contribution.hourly_runoff_volume_m3.fill(0.0);
        frame.add_hillslope_contribution(contribution.clone());
        frame.set_channel_tillage_day_state(7, ChannelTillageDayState::NoPrimaryTillage);
        let carried = run_test_channel(&frame, 7, vec![3], Vec::new());
        let carried_sediment = carried
            .interval_sediment_state
            .as_ref()
            .expect("carried geometry");
        assert_eq!(carried_sediment.geometry_start, first_sediment.geometry_end);
        assert_eq!(carried_sediment.geometry_end, first_sediment.geometry_end);

        frame.set_channel_tillage_day_state(7, ChannelTillageDayState::PrimaryTillage);
        let reseeded = run_test_channel(&frame, 7, vec![3], Vec::new());
        let reseeded_geometry = &reseeded
            .interval_sediment_state
            .as_ref()
            .expect("reseeded geometry")
            .geometry_start;
        assert_eq!(
            reseeded_geometry.depth_a_points_ft,
            control
                .segment_points
                .iter()
                .map(|point| point.depth_a_ft)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            reseeded_geometry.width_a_points_ft,
            control
                .segment_points
                .iter()
                .map(|point| point.width_a_ft)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn wshedw11b_partial_dependency_interval_state_fails_closed() {
        let mut frame = test_network_frame();
        frame.routing_globals.ipeak = 3;
        frame.routing_globals.dtchr_seconds = 3600.0;
        frame.routing_globals.ntchr = 24.0;
        let mut control = test_channel_control();
        control.node_id = 7;
        frame.channel_controls.insert(7, control.clone());
        let mut contribution = test_hillslope_contribution();
        contribution.hourly_runoff_volume_m3 = vec![0.0; 24];
        contribution.hourly_sediment_mass_kg = vec![0.0; 24];
        frame.add_hillslope_contribution(contribution);
        let mut partial = run_test_channel(&frame, 7, vec![3], Vec::new());
        partial.interval_sediment_state = None;
        frame.record_routed_channel_state(partial);
        control.node_id = 8;
        frame.channel_controls.insert(8, control);
        let step = DispatchStep {
            sequence_index: 1,
            node: TopologyNodeKey::new(TopologyNodeKind::Channel, 8),
            dependency_nodes: vec![TopologyNodeKey::new(TopologyNodeKind::Channel, 7)],
            contributor_hillslopes: Vec::new(),
            status: Ws10ChannelImpoundmentKernel::direct_ok_status(TopologyNodeKind::Channel),
        };
        let result = Ws10ChannelImpoundmentKernel::run_direct_channel_node(
            &DirectWatershedKernelInput { step: &step, frame: &frame },
        );
        let Err(error) = result else {
            panic!("partial interval dependency must fail");
        };
        assert_eq!(error.message_id(), WS10_CHANNEL_GUARD_DOMAIN);
        assert_eq!(
            error.symbol,
            BoundarySymbol::from("ws11_interval_partial_dependency")
        );
    }
}
