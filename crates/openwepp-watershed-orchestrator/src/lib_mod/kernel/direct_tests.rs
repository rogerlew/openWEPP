#[cfg(test)]
mod direct_tests {
    use std::collections::BTreeSet;

    use openwepp_input_contract::parsers::{
        chaninp::{ChaninpParseOptions, parse_chaninp_from_str},
        slope::{SlopeParserOptions, parse_slope_str},
        watershed_channel::{WatershedChannelParseOptions, parse_watershed_channel_from_str},
        watershed_impoundment::{
            WatershedImpoundmentParseOptions, parse_watershed_impoundment_from_str,
        },
    };
    use openwepp_topology::parse_topology_fixture_str;

    use super::*;
    use crate::WatershedChannelSegmentPoint;

    const TYPED_TOPOLOGY: &str = r"
HILLSLOPES 2
CHANNELS 1
IMPOUNDMENTS 1
NODE CHANNEL 1 H 1 0 0 C 0 0 0 I 0 0 0
NODE IMPOUNDMENT 1 H 2 0 0 C 1 0 0 I 0 0 0
";
    const STRICT_VALID_CHANINP: &str =
        include_str!("../../../../../tests/fixtures/infile/chaninp/strict_valid.chaninp");
    const STRICT_VALID_SLOPE: &str =
        include_str!("../../../../../tests/fixtures/infile/slope/strict_valid_canonical.slp");
    const STRICT_VALID_WATERSHED_CHANNEL: &str = include_str!(
        "../../../../../tests/fixtures/infile/watershed_channel/strict_sidecar_required.chn"
    );
    const STRICT_VALID_WATERSHED_IMPOUNDMENT: &str = include_str!(
        "../../../../../tests/fixtures/infile/watershed_impoundment/strict_valid_minimal.imp"
    );

    fn test_channel_control() -> WatershedChannelControlRecord {
        WatershedChannelControlRecord {
            node_id: 7,
            ishape: 2,
            icntrl: 0,
            ienslp: 1,
            flgout: 0,
            chnz: 1.0,
            chnnbr: 0.04,
            chnn: 0.05,
            chnk: 0.001,
            chntcr: 2.0,
            chnedm: 0.25,
            chneds: 0.15,
            ctlslp: 0.02,
            ctlz: 1.4,
            ctln: 0.045,
            rating_curve: None,
            segment_points: vec![
                WatershedChannelSegmentPoint {
                    x_m: 0.0,
                    slope: 0.01,
                    depth_a_ft: 0.2,
                    depth_b_ft: 0.3,
                    width_a_ft: 2.0,
                    width_b_ft: 2.5,
                },
                WatershedChannelSegmentPoint {
                    x_m: 12.0,
                    slope: 0.015,
                    depth_a_ft: 0.25,
                    depth_b_ft: 0.35,
                    width_a_ft: 2.2,
                    width_b_ft: 2.8,
                },
            ],
            ws20_case12_enabled: true,
            ws21_case34_enabled: true,
            crfrac: vec![0.2, 0.3, 0.5],
        }
    }

    fn test_network_frame() -> WatershedNetworkFrame {
        let graph = parse_topology_fixture_str(TYPED_TOPOLOGY).expect("typed topology should parse");
        let valid_channel_element_ids = BTreeSet::from([4_i32, 5_i32]);
        let chaninp = parse_chaninp_from_str(
            STRICT_VALID_CHANINP,
            ChaninpParseOptions::strict(4, 2),
            &valid_channel_element_ids,
        )
        .expect("strict chan.inp fixture should parse");
        let slope = parse_slope_str(STRICT_VALID_SLOPE, SlopeParserOptions::strict())
            .expect("strict slope fixture should parse");
        let channel = parse_watershed_channel_from_str(
            STRICT_VALID_WATERSHED_CHANNEL,
            WatershedChannelParseOptions::default(),
        )
        .expect("strict watershed channel fixture should parse");
        let impoundment = parse_watershed_impoundment_from_str(
            STRICT_VALID_WATERSHED_IMPOUNDMENT,
            WatershedImpoundmentParseOptions::strict(),
        )
        .expect("strict watershed impoundment fixture should parse");

        WatershedNetworkFrame::from_parsed_inputs(
            graph,
            Some(chaninp),
            channel,
            slope,
            impoundment,
            3600.0,
            24.0,
        )
        .expect("typed watershed frame should build")
    }

    fn test_hillslope_contribution() -> HillslopeContribution {
        HillslopeContribution {
            hillslope_id: 3,
            area_m2: Some(12_000.0),
            peak_runoff_m3_s: 0.4,
            duration_seconds: 1800.0,
            generated_baseflow_m3: 0.0,
            groundwater_deep_seepage_m3: 0.0,
            total_detachment_kg: 12.0,
            total_deposition_kg: 2.0,
            sediment_concentration_kg_m3: vec![0.1, 0.2],
            particle_diameter_m: vec![0.0002, 0.0005],
            particle_flow_fraction: vec![0.25, 0.75],
            hourly_runoff_volume_m3: Vec::new(),
            hourly_sediment_mass_kg: Vec::new(),
        }
    }

    fn test_dispatch_step(
        dependency_nodes: Vec<TopologyNodeKey>,
        contributor_hillslopes: Vec<u32>,
    ) -> DispatchStep {
        DispatchStep {
            sequence_index: 0,
            node: TopologyNodeKey::new(TopologyNodeKind::Channel, 7),
            dependency_nodes,
            contributor_hillslopes,
            status: Ws10ChannelImpoundmentKernel::direct_ok_status(TopologyNodeKind::Channel),
        }
    }

    fn test_routed_channel_state(qsed_kg_s: f64) -> RoutedChannelState {
        RoutedChannelState {
            node_id: 4,
            runoff_volume_m3: 10.0,
            channel_inflow_m3: 10.0,
            channel_outflow_m3: 10.0,
            channel_storage_m3: 0.0,
            peak_discharge_m3_s: 0.5,
            duration_seconds: 20.0,
            channel_baseflow_m3: 0.0,
            channel_loss_m3: 0.0,
            groundwater_deep_seepage_m3: 0.0,
            sediment_yield_kg: qsed_kg_s,
            wave_state: None,
            sediment_state: RoutedChannelSedimentState {
                qsed_kg_s,
                transport_capacity_kg_s: qsed_kg_s * 2.0,
                particle_flow_fraction: vec![0.4, 0.6],
                particle_diameter_m: vec![0.0001, 0.0003],
                ..RoutedChannelSedimentState::default()
            },
        }
    }

    fn test_peak_partition() -> Ws20IncomingPeakPartition {
        Ws20IncomingPeakPartition {
            hillslope_peak_cms: 0.4,
            dependency_peak_cms: 0.0,
            hillslope_volume_m3: 25.0,
            dependency_volume_m3: 0.0,
            hillslope_duration_s: 200.0,
            dependency_duration_s: 0.0,
            hourly_resolved: false,
            hourly_sediment_inlet_kg: [0.0; 24],
        }
    }

    fn test_sediment_accumulator() -> Ws19SedimentAccumulator {
        let mut accumulator = Ws19SedimentAccumulator::default();
        let hillslope_payload = Ws18HillslopeSedimentPayload {
            mass_kg: 10.0,
            fractions: vec![0.25, 0.75],
            particle_diameters_m: vec![0.0002, 0.0005],
        };
        Ws10ChannelImpoundmentKernel::add_direct_sediment_payload_to_accumulator(
            &mut accumulator,
            &hillslope_payload,
            Ws19SedimentIngress::Hillslope,
            Ws10NodeClass::Channel,
        )
        .expect("hillslope payload should accumulate");
        let dependency_payload = Ws18HillslopeSedimentPayload {
            mass_kg: 6.0,
            fractions: vec![0.5, 0.5],
            particle_diameters_m: vec![0.0001, 0.0003],
        };
        Ws10ChannelImpoundmentKernel::add_direct_sediment_payload_to_accumulator(
            &mut accumulator,
            &dependency_payload,
            Ws19SedimentIngress::DependencyChannel,
            Ws10NodeClass::Channel,
        )
        .expect("dependency payload should accumulate");
        accumulator
    }

    fn test_sediment_routing_context(
        control: &WatershedChannelControlRecord,
    ) -> Ws19SedimentRoutingContext<'_> {
        let sediment_controls =
            Ws10ChannelImpoundmentKernel::read_direct_ws15_channel_sediment_controls(control)
                .expect("test channel controls should project");
        let nslpts =
            Ws10ChannelImpoundmentKernel::require_direct_ws17_channel_segment_scaffold(control)
                .expect("test channel profile should be valid");
        Ws19SedimentRoutingContext {
            control,
            node_class: Ws10NodeClass::Channel,
            event_duration: 200.0,
            qpo: 0.3,
            roughness: control.chnn,
            sediment_controls,
            nslpts,
            peak_partition: test_peak_partition(),
        }
    }

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() <= 1.0e-12,
            "{actual} differs from {expected}",
        );
    }

    fn test_channel_context(
        control: &WatershedChannelControlRecord,
        ipeak_branch: Ws11IpeakBranch,
    ) -> Ws11DirectChannelContext<'_> {
        let sediment_controls =
            Ws10ChannelImpoundmentKernel::read_direct_ws15_channel_sediment_controls(control)
                .expect("test channel controls should project");
        let nslpts =
            Ws10ChannelImpoundmentKernel::require_direct_ws17_channel_segment_scaffold(control)
                .expect("test channel profile should be valid");
        let channel_length = Ws10ChannelImpoundmentKernel::direct_ws11_channel_length(control)
            .expect("test channel length should be valid");
        Ws11DirectChannelContext {
            node_id: control.node_id,
            control,
            dtchr: 120.0,
            nchnum: 1.0,
            cbase: 0.0,
            ipeak_branch,
            roughness: control.chnn,
            control_slope: control.ctlslp,
            conductivity: control.chnk,
            sediment_controls,
            nslpts,
            channel_length,
            ishape: u32::try_from(control.ishape).expect("test shape should convert"),
        }
    }

    fn test_channel_hydrology(runvol_case: f64, incoming_peak: f64) -> Ws11DirectChannelHydrology {
        Ws11DirectChannelHydrology {
            peak_partition: test_peak_partition(),
            channel_baseflow: Ws10ChannelBaseflowPartition {
                peak_m3_s: 0.0,
                volume_m3: 0.0,
                deep_seepage_m3: 0.0,
            },
            incoming_peak,
            baseflow_peak: 0.0,
            available_peak: incoming_peak,
            routing_gain: 1.2,
            watdur: 200.0,
            runvol_case,
        }
    }

    #[test]
    fn direct_range_and_ipeak_branch_helpers_cover_guard_cases() {
        assert_eq!(
            Ws10ChannelImpoundmentKernel::direct_require_range(
                Ws10NodeClass::Channel,
                "finite",
                f64::NAN,
                None,
                None,
            )
            .expect_err("NaN should fail closed")
            .boundary_class(),
            BoundaryClass::NonFinite
        );
        assert_eq!(
            Ws10ChannelImpoundmentKernel::direct_require_range(
                Ws10NodeClass::Channel,
                "minimum",
                -1.0,
                Some(0.0),
                None,
            )
            .expect_err("below minimum should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );
        assert_eq!(
            Ws10ChannelImpoundmentKernel::direct_require_range(
                Ws10NodeClass::Channel,
                "maximum",
                2.0,
                None,
                Some(1.0),
            )
            .expect_err("above maximum should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );

        assert!(matches!(
            Ws10ChannelImpoundmentKernel::direct_ipeak_branch(1, Ws10NodeClass::Channel)
                .expect("ipeak 1 should select rational"),
            Ws11IpeakBranch::Rational
        ));
        assert!(matches!(
            Ws10ChannelImpoundmentKernel::direct_ipeak_branch(2, Ws10NodeClass::Channel)
                .expect("ipeak 2 should select CREAMS"),
            Ws11IpeakBranch::Creams
        ));
        assert!(matches!(
            Ws10ChannelImpoundmentKernel::direct_ipeak_branch(3, Ws10NodeClass::Channel)
                .expect("ipeak 3 should select kinematic"),
            Ws11IpeakBranch::KinematicWave
        ));
        assert!(matches!(
            Ws10ChannelImpoundmentKernel::direct_ipeak_branch(5, Ws10NodeClass::Channel)
                .expect("ipeak 5 should select variable Muskingum"),
            Ws11IpeakBranch::MuskingumCungeVariable
        ));
        assert!(matches!(
            Ws10ChannelImpoundmentKernel::direct_ipeak_branch(4, Ws10NodeClass::Channel)
                .expect("other positive ipeak should select Muskingum"),
            Ws11IpeakBranch::MuskingumCunge
        ));
        assert_eq!(
            Ws10ChannelImpoundmentKernel::direct_ipeak_branch(0, Ws10NodeClass::Channel)
                .expect_err("non-positive ipeak should fail closed")
                .boundary_class(),
            BoundaryClass::DomainViolation
        );
    }

    #[test]
    fn direct_ws11_runon_cases_are_characterized() {
        let zero_baseflow = Ws10ChannelBaseflowPartition {
            peak_m3_s: 0.0,
            volume_m3: 0.0,
            deep_seepage_m3: 0.0,
        };
        let zero_runon = Ws10ChannelImpoundmentKernel::compute_direct_channel_runon(
            Ws20IncomingPeakPartition {
                hillslope_volume_m3: 0.0,
                hillslope_duration_s: 10.0,
                ..test_peak_partition()
            },
            zero_baseflow,
            10.0,
        )
        .expect("zero incoming and zero channel runoff should select case 1");
        assert_close(zero_runon.runvol_case, 0.0);

        let channel_runon = Ws10ChannelImpoundmentKernel::compute_direct_channel_runon(
            Ws20IncomingPeakPartition {
                hillslope_volume_m3: 2.0,
                dependency_volume_m3: 3.0,
                hillslope_duration_s: 10.0,
                dependency_duration_s: 12.0,
                ..test_peak_partition()
            },
            Ws10ChannelBaseflowPartition {
                volume_m3: 4.0,
                ..zero_baseflow
            },
            10.0,
        )
        .expect("positive channel runoff should select case 2");
        assert_close(channel_runon.runvol_case, 9.0);

        let small_runon = Ws10ChannelImpoundmentKernel::compute_direct_channel_runon(
            Ws20IncomingPeakPartition {
                hillslope_volume_m3: 0.0005,
                dependency_volume_m3: 0.0,
                hillslope_duration_s: 10.0,
                ..test_peak_partition()
            },
            zero_baseflow,
            10.0,
        )
        .expect("small runon without channel runoff should select case 4");
        assert_close(small_runon.runvol_case, 0.0);

        let lateral_runon = Ws10ChannelImpoundmentKernel::compute_direct_channel_runon(
            Ws20IncomingPeakPartition {
                hillslope_volume_m3: 2.0,
                dependency_volume_m3: 0.0,
                hillslope_duration_s: 10.0,
                ..test_peak_partition()
            },
            zero_baseflow,
            10.0,
        )
        .expect("runon without channel runoff should select case 3");
        assert_close(lateral_runon.runvol_case, 2.0);
    }

    #[test]
    fn direct_ws11_runon_guards_fail_closed() {
        let zero_baseflow = Ws10ChannelBaseflowPartition {
            peak_m3_s: 0.0,
            volume_m3: 0.0,
            deep_seepage_m3: 0.0,
        };
        for invalid_partition in [
            Ws20IncomingPeakPartition {
                hillslope_volume_m3: -1.0,
                ..test_peak_partition()
            },
            Ws20IncomingPeakPartition {
                dependency_volume_m3: -1.0,
                ..test_peak_partition()
            },
            Ws20IncomingPeakPartition {
                hillslope_duration_s: -1.0,
                ..test_peak_partition()
            },
            Ws20IncomingPeakPartition {
                dependency_duration_s: -1.0,
                ..test_peak_partition()
            },
        ] {
            assert_eq!(
                Ws10ChannelImpoundmentKernel::compute_direct_channel_runon(
                    invalid_partition,
                    zero_baseflow,
                    10.0,
                )
                .expect_err("negative runon operand should fail closed")
                .boundary_class(),
                BoundaryClass::DomainViolation
            );
        }

        let error = Ws10ChannelImpoundmentKernel::compute_direct_channel_runon(
            Ws20IncomingPeakPartition {
                hillslope_volume_m3: 0.0,
                dependency_volume_m3: 0.0,
                hillslope_duration_s: 0.0,
                dependency_duration_s: 0.0,
                ..test_peak_partition()
            },
            zero_baseflow,
            0.0,
        )
        .expect_err("zero duration support should fail closed");
        assert_eq!(error.boundary_class(), BoundaryClass::DomainViolation);
    }

    #[test]
    fn direct_channel_peak_helpers_cover_branch_cases() {
        let control = test_channel_control();
        let zero_hydrology = test_channel_hydrology(0.0, 0.0);

        let rational_peak = Ws10ChannelImpoundmentKernel::compute_direct_channel_rational_peak(
            &test_channel_hydrology(20.0, 0.5),
        );
        assert!(rational_peak.qpo > 0.0);

        let creams_zero = Ws10ChannelImpoundmentKernel::compute_direct_channel_creams_peak(
            &test_channel_context(&control, Ws11IpeakBranch::Creams),
            &zero_hydrology,
        )
        .expect("CREAMS zero runoff should return zero peak");
        assert_close(creams_zero.qpo, 0.0);

        let creams_peak = Ws10ChannelImpoundmentKernel::compute_direct_channel_creams_peak(
            &test_channel_context(&control, Ws11IpeakBranch::Creams),
            &test_channel_hydrology(20.0, 0.5),
        )
        .expect("CREAMS positive runoff should route");
        assert!(creams_peak.qpo.is_finite() && creams_peak.qpo > 0.0);

        let mut invalid_creams_context =
            test_channel_context(&control, Ws11IpeakBranch::Creams);
        invalid_creams_context.conductivity = f64::INFINITY;
        assert_eq!(
            Ws10ChannelImpoundmentKernel::compute_direct_channel_creams_peak(
                &invalid_creams_context,
                &test_channel_hydrology(20.0, 0.5),
            )
            .expect_err("non-finite CREAMS attenuation should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );

        let mut invalid_gain = test_channel_hydrology(20.0, 0.5);
        invalid_gain.routing_gain = -1.0;
        assert_eq!(
            Ws10ChannelImpoundmentKernel::compute_direct_channel_creams_peak(
                &test_channel_context(&control, Ws11IpeakBranch::Creams),
                &invalid_gain,
            )
            .expect_err("non-finite CREAMS gain should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );

        for branch in [
            Ws11IpeakBranch::KinematicWave,
            Ws11IpeakBranch::MuskingumCunge,
            Ws11IpeakBranch::MuskingumCungeVariable,
        ] {
            let context = test_channel_context(&control, branch);
            let peak = match branch {
                Ws11IpeakBranch::KinematicWave => {
                    Ws10ChannelImpoundmentKernel::compute_direct_channel_kinematic_peak(
                        &context,
                        &zero_hydrology,
                    )
                }
                Ws11IpeakBranch::MuskingumCunge => {
                    Ws10ChannelImpoundmentKernel::compute_direct_channel_muskingum_peak(
                        &context,
                        &zero_hydrology,
                        None,
                    )
                }
                Ws11IpeakBranch::MuskingumCungeVariable => {
                    Ws10ChannelImpoundmentKernel::compute_direct_channel_variable_muskingum_peak(
                        &context,
                        &zero_hydrology,
                        None,
                    )
                }
                Ws11IpeakBranch::Rational | Ws11IpeakBranch::Creams => unreachable!(),
            }
            .expect("zero runoff wave branch should return zero peak");
            assert_close(peak.qpo, 0.0);
            assert!(peak.wave_state.is_none());
        }
    }

    #[test]
    fn direct_channel_peak_helpers_cover_zero_and_positive_wave_paths() {
        let control = test_channel_control();
        let zero_rational = Ws10ChannelImpoundmentKernel::compute_direct_channel_rational_peak(
            &test_channel_hydrology(0.0, 0.5),
        );
        assert_close(zero_rational.qpo, 0.0);

        let hydrology = test_channel_hydrology(20.0, 0.5);
        let kinematic = Ws10ChannelImpoundmentKernel::compute_direct_channel_kinematic_peak(
            &test_channel_context(&control, Ws11IpeakBranch::KinematicWave),
            &hydrology,
        )
        .expect("positive kinematic branch should route");
        assert!(kinematic.qpo.is_finite() && kinematic.qpo >= 0.0);
        assert!(kinematic.wave_state.is_some());

        let muskingum = Ws10ChannelImpoundmentKernel::compute_direct_channel_muskingum_peak(
            &test_channel_context(&control, Ws11IpeakBranch::MuskingumCunge),
            &hydrology,
            None,
        )
        .expect("positive Muskingum branch should route");
        assert!(muskingum.qpo.is_finite() && muskingum.qpo >= 0.0);
        assert!(muskingum.wave_state.is_some());

        let variable =
            Ws10ChannelImpoundmentKernel::compute_direct_channel_variable_muskingum_peak(
                &test_channel_context(&control, Ws11IpeakBranch::MuskingumCungeVariable),
                &hydrology,
                None,
            )
            .expect("positive variable Muskingum branch should route");
        assert!(variable.qpo.is_finite() && variable.qpo >= 0.0);
        assert!(variable.wave_state.is_some());
    }

    #[test]
    fn direct_channel_runoff_helpers_cover_branch_cases() {
        let control = test_channel_control();
        let rational_context = test_channel_context(&control, Ws11IpeakBranch::Rational);
        let zero_hydrology = test_channel_hydrology(0.0, 0.0);
        let rational_runoff = Ws10ChannelImpoundmentKernel::compute_direct_channel_runoff(
            &rational_context,
            &zero_hydrology,
            0.0,
        )
        .expect("zero rational runoff should project");
        assert_close(rational_runoff.roff, 0.0);
        assert_close(rational_runoff.durrof, 0.0);

        let wave_runoff = Ws10ChannelImpoundmentKernel::compute_direct_channel_runoff(
            &test_channel_context(&control, Ws11IpeakBranch::KinematicWave),
            &test_channel_hydrology(20.0, 0.5),
            0.1,
        )
        .expect("wave runoff should use qpo-duration product");
        assert!((wave_runoff.roff - 20.0).abs() <= 1.0e-12);
        assert!((wave_runoff.durrof - 200.0).abs() <= 1.0e-12);

        assert_eq!(
            Ws10ChannelImpoundmentKernel::compute_direct_channel_runoff(
                &rational_context,
                &test_channel_hydrology(f64::MAX, 0.5),
                1.0e-6,
            )
            .expect_err("non-finite duration from finite runoff volume should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );

        assert_eq!(
            Ws10ChannelImpoundmentKernel::compute_direct_channel_runoff(
                &test_channel_context(&control, Ws11IpeakBranch::KinematicWave),
                &test_channel_hydrology(20.0, 0.5),
                f64::INFINITY,
            )
            .expect_err("non-finite wave runoff should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );
    }

    #[test]
    fn direct_scalar_validation_helpers_cover_remaining_guard_paths() {
        assert_eq!(
            Ws10ChannelImpoundmentKernel::direct_checked_runon_volume(
                Ws10NodeClass::Channel,
                "runon_volume",
                f64::INFINITY,
                1.0,
            )
            .expect_err("non-finite runon volume should fail closed")
            .boundary_class(),
            BoundaryClass::NonFinite
        );
        assert_eq!(
            Ws10ChannelImpoundmentKernel::direct_checked_runon_volume(
                Ws10NodeClass::Channel,
                "runon_volume",
                -1.0,
                1.0,
            )
            .expect_err("negative runon volume should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );

        for (hillslope, dependency, expected) in [
            (
                Ws20RunonTotals {
                    peak_cms: f64::INFINITY,
                    ..Ws20RunonTotals::default()
                },
                Ws20RunonTotals::default(),
                BoundaryClass::NonFinite,
            ),
            (
                Ws20RunonTotals {
                    peak_cms: -2.0,
                    ..Ws20RunonTotals::default()
                },
                Ws20RunonTotals::default(),
                BoundaryClass::DomainViolation,
            ),
            (
                Ws20RunonTotals {
                    duration_s: f64::INFINITY,
                    ..Ws20RunonTotals::default()
                },
                Ws20RunonTotals::default(),
                BoundaryClass::NonFinite,
            ),
            (
                Ws20RunonTotals {
                    duration_s: -2.0,
                    ..Ws20RunonTotals::default()
                },
                Ws20RunonTotals {
                    duration_s: -1.0,
                    ..Ws20RunonTotals::default()
                },
                BoundaryClass::DomainViolation,
            ),
        ] {
            assert_eq!(
                Ws10ChannelImpoundmentKernel::validate_direct_incoming_runon_totals(
                    Ws10NodeClass::Channel,
                    hillslope,
                    dependency,
                )
                .expect_err("invalid incoming totals should fail closed")
                .boundary_class(),
                expected
            );
        }
    }

    #[test]
    fn direct_hourly_and_sediment_input_validation_helpers_fail_closed() {
        let mut contribution = test_hillslope_contribution();
        contribution.hourly_runoff_volume_m3 = vec![1.0; 24];
        contribution.hourly_sediment_mass_kg = vec![0.5; 24];

        let mut bad_runoff = contribution.clone();
        bad_runoff.hourly_runoff_volume_m3[3] = -1.0;
        assert_eq!(
            Ws10ChannelImpoundmentKernel::accumulate_direct_hillslope_hourly_runon(
                Ws10NodeClass::Channel,
                &bad_runoff,
                &mut [0.0; 24],
                &mut [0.0; 24],
            )
            .expect_err("negative hourly runoff volume should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );

        let mut bad_sediment = contribution;
        bad_sediment.hourly_sediment_mass_kg[3] = -1.0;
        assert_eq!(
            Ws10ChannelImpoundmentKernel::accumulate_direct_hillslope_hourly_runon(
                Ws10NodeClass::Channel,
                &bad_sediment,
                &mut [0.0; 24],
                &mut [0.0; 24],
            )
            .expect_err("negative hourly sediment mass should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );

        for (event_duration, qpo, roughness) in [(0.0, 0.1, 0.05), (10.0, -0.1, 0.05), (10.0, 0.1, 0.0)] {
            assert_eq!(
                Ws10ChannelImpoundmentKernel::validate_direct_sediment_routing_inputs(
                    Ws10NodeClass::Channel,
                    event_duration,
                    qpo,
                    roughness,
                )
                .expect_err("invalid sediment routing input should fail closed")
                .boundary_class(),
                BoundaryClass::DomainViolation
            );
        }
    }

    #[test]
    fn direct_impoundment_horizon_and_channel_scaffold_guards_are_characterized() {
        let horizon = Ws10ChannelImpoundmentKernel::direct_impoundment_integration_horizon(
            0.5, 7200.0,
        )
        .expect("positive incoming duration should set horizon");
        assert_close(horizon, 2.0);
        let fallback = Ws10ChannelImpoundmentKernel::direct_impoundment_integration_horizon(0.5, 0.0)
            .expect("zero incoming duration should fall back to deltat");
        assert_close(fallback, 0.5);
        for (deltat, incoming_duration) in [(0.5, -1.0), (0.0, 0.0)] {
            assert_eq!(
                Ws10ChannelImpoundmentKernel::direct_impoundment_integration_horizon(
                    deltat,
                    incoming_duration,
                )
                .expect_err("invalid impoundment horizon input should fail closed")
                .boundary_class(),
                BoundaryClass::DomainViolation
            );
        }

        let control = test_channel_control();
        assert_eq!(
            Ws10ChannelImpoundmentKernel::require_direct_ws17_channel_segment_scaffold(
                &WatershedChannelControlRecord {
                    segment_points: vec![control.segment_points[0]],
                    ..control.clone()
                },
            )
            .expect_err("single segment point should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );
        assert_eq!(
            Ws10ChannelImpoundmentKernel::require_direct_ws17_channel_segment_scaffold(
                &WatershedChannelControlRecord {
                    segment_points: vec![control.segment_points[1], control.segment_points[0]],
                    ..control.clone()
                },
            )
            .expect_err("descending segment x should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );
        assert_eq!(
            Ws10ChannelImpoundmentKernel::direct_ws11_channel_length(
                &WatershedChannelControlRecord {
                    segment_points: Vec::new(),
                    ..control.clone()
                },
            )
            .expect_err("empty segment scaffold should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );
        assert_eq!(
            Ws10ChannelImpoundmentKernel::direct_ws11_channel_length(
                &WatershedChannelControlRecord {
                    segment_points: vec![control.segment_points[0], control.segment_points[0]],
                    ..control.clone()
                },
            )
            .expect_err("zero channel length should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );
        for invalid_point in [
            WatershedChannelSegmentPoint {
                width_a_ft: 0.0,
                ..control.segment_points[0]
            },
            WatershedChannelSegmentPoint {
                width_b_ft: 0.0,
                ..control.segment_points[0]
            },
        ] {
            assert_eq!(
                Ws10ChannelImpoundmentKernel::require_direct_ws17_channel_segment_scaffold(
                    &WatershedChannelControlRecord {
                        segment_points: vec![invalid_point, control.segment_points[1]],
                        ..control.clone()
                    },
                )
                .expect_err("invalid segment width should fail closed")
                .boundary_class(),
                BoundaryClass::DomainViolation
            );
        }
    }

    #[test]
    fn direct_impoundment_outflow_helper_routes_valid_context() {
        let frame = test_network_frame();
        let control = frame
            .impoundment_controls
            .values()
            .next()
            .expect("test frame should carry an impoundment control");
        let context = Ws12DirectImpoundmentContext {
            node_id: control.node_id,
            control,
            stage_h: control.h,
            hfull: control.hfull,
            deltat: control.deltat,
            qinf: control.qinf,
        };
        let coefficients = Ws10ChannelImpoundmentKernel::direct_ws12_impoundment_coefficients(
            context.control,
        )
        .expect("test impoundment coefficients should project");
        let horizon = Ws10ChannelImpoundmentKernel::direct_impoundment_integration_horizon(
            context.deltat,
            120.0,
        )
        .expect("valid horizon should project");

        let outflow = Ws10ChannelImpoundmentKernel::route_direct_impoundment_outflow(
            &context,
            0.25,
            120.0,
            horizon,
            &coefficients,
        )
        .expect("valid impoundment context should route outflow");

        assert!(outflow.qo.is_finite() && outflow.qo >= 0.0);
        assert!(outflow.durout.is_finite() && outflow.durout >= 120.0);
        assert!(outflow.hnext.is_finite() && outflow.hnext >= 0.0);
        if let DirectWatershedKernelOutput::Impoundment(state) =
            Ws10ChannelImpoundmentKernel::direct_impoundment_output(&context, outflow)
        {
            assert_eq!(state.node_id, control.node_id);
            assert_close(state.outflow_rate_m3_s, outflow.qo);
        } else {
            unreachable!("direct impoundment output should publish impoundment state");
        }
    }

    #[test]
    fn direct_impoundment_outflow_helper_guards_terminal_values() {
        let frame = test_network_frame();
        let control = frame
            .impoundment_controls
            .values()
            .next()
            .expect("test frame should carry an impoundment control");
        let context = Ws12DirectImpoundmentContext {
            node_id: control.node_id,
            control,
            stage_h: control.h,
            hfull: control.hfull,
            deltat: control.deltat,
            qinf: control.qinf,
        };
        let coefficients = Ws10ChannelImpoundmentKernel::direct_ws12_impoundment_coefficients(
            context.control,
        )
        .expect("test impoundment coefficients should project");

        assert_eq!(
            Ws10ChannelImpoundmentKernel::route_direct_impoundment_outflow(
                &context,
                0.25,
                f64::INFINITY,
                1.0,
                &coefficients,
            )
            .expect_err("non-finite output duration should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );
        let negative_continuity = Ws12DirectImpoundmentContext {
            qinf: -10.0,
            ..context
        };
        assert_eq!(
            Ws10ChannelImpoundmentKernel::route_direct_impoundment_outflow(
                &negative_continuity,
                0.0,
                120.0,
                1.0,
                &coefficients,
            )
            .expect_err("negative continuity outflow should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );
    }

    #[test]
    fn direct_ws20_profile_guard_cases_are_characterized() {
        let control = test_channel_control();
        for invalid_point in [
            WatershedChannelSegmentPoint {
                x_m: -1.0,
                ..control.segment_points[0]
            },
            WatershedChannelSegmentPoint {
                slope: -1.0,
                ..control.segment_points[0]
            },
            WatershedChannelSegmentPoint {
                depth_a_ft: -1.0,
                ..control.segment_points[0]
            },
            WatershedChannelSegmentPoint {
                width_a_ft: 0.0,
                ..control.segment_points[0]
            },
        ] {
            let invalid = WatershedChannelControlRecord {
                segment_points: vec![invalid_point, control.segment_points[1]],
                ..control.clone()
            };
            assert_eq!(
                Ws10ChannelImpoundmentKernel::direct_ws20_channel_profile(&invalid, 2)
                    .expect_err("invalid WS20 segment point should fail closed")
                    .boundary_class(),
                BoundaryClass::DomainViolation
            );
        }
    }

    #[test]
    fn direct_active_sediment_and_publication_guards_are_characterized() {
        let empty_active = Ws10ChannelImpoundmentKernel::direct_active_sediment_classes(
            &Ws19SedimentAccumulator::default(),
            Ws10NodeClass::Channel,
        )
        .expect("zero accumulator should produce no active classes");
        assert!(empty_active.class_mass_kg.is_empty());

        let sparse_accumulator = Ws19SedimentAccumulator {
            incoming_sediment_mass_kg: 2.0,
            class_mass_kg: vec![0.0, 2.0],
            class_diameter_mass_m: vec![0.0, 0.001],
            top_class_mass_kg: Vec::new(),
            lateral_class_mass_kg: Vec::new(),
        };
        let sparse_active = Ws10ChannelImpoundmentKernel::direct_active_sediment_classes(
            &sparse_accumulator,
            Ws10NodeClass::Channel,
        )
        .expect("sparse positive accumulator should skip zero classes");
        assert_eq!(sparse_active.class_numbers, vec![2]);
        assert_eq!(sparse_active.top_class_mass_kg, vec![0.0]);
        assert_eq!(sparse_active.lateral_class_mass_kg, vec![0.0]);

        let invalid_active = Ws19SedimentAccumulator {
            class_diameter_mass_m: vec![0.0, 0.0],
            ..sparse_accumulator
        };
        assert_eq!(
            Ws10ChannelImpoundmentKernel::direct_active_sediment_classes(
                &invalid_active,
                Ws10NodeClass::Channel,
            )
            .expect_err("positive class mass requires positive weighted diameter")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );

        let (fractions, diameters) =
            Ws10ChannelImpoundmentKernel::direct_sediment_publication_particles(
                &[],
                &empty_active,
                Ws10NodeClass::Channel,
            )
            .expect("zero routed classes should publish empty particles");
        assert!(fractions.is_empty());
        assert!(diameters.is_empty());

        let active = Ws19ActiveSedimentClasses {
            class_mass_kg: vec![1.0, 2.0],
            top_class_mass_kg: vec![0.0, 0.0],
            lateral_class_mass_kg: vec![1.0, 2.0],
            particle_diameters_m: vec![0.0001, 0.0003],
            class_numbers: vec![1, 2],
        };
        let (fractions, diameters) =
            Ws10ChannelImpoundmentKernel::direct_sediment_publication_particles(
                &[0.0, 3.0],
                &active,
                Ws10NodeClass::Channel,
            )
            .expect("zero routed class should be skipped during publication");
        assert_eq!(fractions, vec![1.0]);
        assert_eq!(diameters, vec![0.0003]);

        assert_eq!(
            Ws10ChannelImpoundmentKernel::direct_sediment_publication_particles(
                &[f64::INFINITY],
                &Ws19ActiveSedimentClasses {
                    particle_diameters_m: vec![0.0001],
                    ..active
                },
                Ws10NodeClass::Channel,
            )
            .expect_err("non-finite publication fraction sum should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );
    }

    #[test]
    fn direct_dependency_sediment_payload_guards_cover_remaining_paths() {
        let mut invalid_qsed = test_routed_channel_state(-0.1);
        assert_eq!(
            Ws10ChannelImpoundmentKernel::read_direct_channel_sediment_payload(
                &invalid_qsed,
                Ws10NodeClass::Channel,
                100.0,
            )
            .expect_err("negative dependency qsed should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );

        invalid_qsed.sediment_state.qsed_kg_s = 0.2;
        assert_eq!(
            Ws10ChannelImpoundmentKernel::read_direct_channel_sediment_payload(
                &invalid_qsed,
                Ws10NodeClass::Channel,
                f64::INFINITY,
            )
            .expect_err("non-finite dependency mass should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );

        let mut invalid_fraction = test_routed_channel_state(0.2);
        invalid_fraction.sediment_state.particle_flow_fraction = vec![1.2];
        invalid_fraction.sediment_state.particle_diameter_m = vec![0.0001];
        assert_eq!(
            Ws10ChannelImpoundmentKernel::read_direct_channel_sediment_payload(
                &invalid_fraction,
                Ws10NodeClass::Channel,
                100.0,
            )
            .expect_err("dependency particle fractions are bounded")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );

        let mut invalid_diameter = test_routed_channel_state(0.2);
        invalid_diameter.sediment_state.particle_flow_fraction = vec![1.0];
        invalid_diameter.sediment_state.particle_diameter_m = vec![0.0];
        assert_eq!(
            Ws10ChannelImpoundmentKernel::read_direct_channel_sediment_payload(
                &invalid_diameter,
                Ws10NodeClass::Channel,
                100.0,
            )
            .expect_err("dependency particle diameters are positive")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );

        let mut zero_fraction_sum = test_routed_channel_state(0.2);
        zero_fraction_sum.sediment_state.particle_flow_fraction = vec![0.0, 0.0];
        zero_fraction_sum.sediment_state.particle_diameter_m = vec![0.0001, 0.0002];
        assert_eq!(
            Ws10ChannelImpoundmentKernel::read_direct_channel_sediment_payload(
                &zero_fraction_sum,
                Ws10NodeClass::Channel,
                100.0,
            )
            .expect_err("positive dependency sediment needs positive fraction support")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );
    }

    #[test]
    fn direct_dependency_peak_payload_covers_channel_paths() {
        let mut frame = test_network_frame();
        frame.routed_channels.insert(4, test_routed_channel_state(0.0));
        let channel_key = TopologyNodeKey::new(TopologyNodeKind::Channel, 4);

        let (peak, duration) = Ws10ChannelImpoundmentKernel::read_direct_dependency_peak_payload(
            &frame,
            Ws10NodeClass::Channel,
            channel_key,
        )
        .expect("valid routed channel dependency should project peak payload");
        assert_close(peak, 0.5);
        assert_close(duration, 20.0);

        let mut invalid_peak = test_routed_channel_state(0.0);
        invalid_peak.peak_discharge_m3_s = -1.0;
        frame.routed_channels.insert(4, invalid_peak);
        assert_eq!(
            Ws10ChannelImpoundmentKernel::read_direct_dependency_peak_payload(
                &frame,
                Ws10NodeClass::Channel,
                channel_key,
            )
            .expect_err("negative dependency channel peak should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );

        let mut invalid_duration = test_routed_channel_state(0.0);
        invalid_duration.duration_seconds = -1.0;
        frame.routed_channels.insert(4, invalid_duration);
        assert_eq!(
            Ws10ChannelImpoundmentKernel::read_direct_dependency_peak_payload(
                &frame,
                Ws10NodeClass::Channel,
                channel_key,
            )
            .expect_err("negative dependency channel duration should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );

        assert_eq!(
            Ws10ChannelImpoundmentKernel::read_direct_dependency_peak_payload(
                &frame,
                Ws10NodeClass::Channel,
                TopologyNodeKey::new(TopologyNodeKind::Channel, 99),
            )
            .expect_err("missing dependency channel should fail closed")
            .boundary_class(),
            BoundaryClass::MissingRequiredInput
        );
    }

    #[test]
    fn direct_dependency_peak_payload_covers_impoundment_and_kind_paths() {
        let mut frame = test_network_frame();
        let impoundment_key = TopologyNodeKey::new(TopologyNodeKind::Impoundment, 8);
        frame.routed_impoundments.insert(
            8,
            RoutedImpoundmentState {
                node_id: 8,
                outflow_volume_m3: 30.0,
                outflow_rate_m3_s: 0.25,
                duration_seconds: 120.0,
                hnext_m: 0.5,
            },
        );

        let (peak, duration) = Ws10ChannelImpoundmentKernel::read_direct_dependency_peak_payload(
            &frame,
            Ws10NodeClass::Channel,
            impoundment_key,
        )
        .expect("valid routed impoundment dependency should project peak payload");
        assert_close(peak, 0.25);
        assert_close(duration, 120.0);

        let mut invalid_peak = frame
            .routed_impoundments
            .get(&8)
            .expect("test impoundment should exist")
            .clone();
        invalid_peak.outflow_rate_m3_s = -1.0;
        frame.routed_impoundments.insert(8, invalid_peak);
        assert_eq!(
            Ws10ChannelImpoundmentKernel::read_direct_dependency_peak_payload(
                &frame,
                Ws10NodeClass::Channel,
                impoundment_key,
            )
            .expect_err("negative dependency impoundment peak should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );

        let mut invalid_duration = frame
            .routed_impoundments
            .get(&8)
            .expect("test impoundment should exist")
            .clone();
        invalid_duration.outflow_rate_m3_s = 0.25;
        invalid_duration.duration_seconds = -1.0;
        frame.routed_impoundments.insert(8, invalid_duration);
        assert_eq!(
            Ws10ChannelImpoundmentKernel::read_direct_dependency_peak_payload(
                &frame,
                Ws10NodeClass::Channel,
                impoundment_key,
            )
            .expect_err("negative dependency impoundment duration should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );

        assert_eq!(
            Ws10ChannelImpoundmentKernel::read_direct_dependency_peak_payload(
                &frame,
                Ws10NodeClass::Channel,
                TopologyNodeKey::new(TopologyNodeKind::Impoundment, 99),
            )
            .expect_err("missing dependency impoundment should fail closed")
            .boundary_class(),
            BoundaryClass::MissingRequiredInput
        );

        assert_eq!(
            Ws10ChannelImpoundmentKernel::read_direct_dependency_peak_payload(
                &frame,
                Ws10NodeClass::Channel,
                TopologyNodeKey::new(TopologyNodeKind::Hillslope, 1),
            )
            .expect_err("hillslope dependency kind should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );
    }

    #[test]
    fn direct_dependency_baseflow_and_contributor_area_helpers_cover_frame_paths() {
        let mut frame = test_network_frame();
        let mut contribution = test_hillslope_contribution();
        contribution.generated_baseflow_m3 = 3.0;
        contribution.groundwater_deep_seepage_m3 = 0.4;
        frame.add_hillslope_contribution(contribution);
        let mut routed = test_routed_channel_state(0.0);
        routed.channel_baseflow_m3 = 5.0;
        routed.groundwater_deep_seepage_m3 = 0.6;
        frame.routed_channels.insert(4, routed);
        let step = test_dispatch_step(
            vec![TopologyNodeKey::new(TopologyNodeKind::Channel, 4)],
            vec![3],
        );
        let input = DirectWatershedKernelInput {
            step: &step,
            frame: &frame,
        };

        let generated = Ws10ChannelImpoundmentKernel::generated_groundwater_from_step(
            &input,
            Ws10NodeClass::Channel,
        )
        .expect("valid generated groundwater payload should project");
        assert_close(generated.volume_m3, 3.0);
        assert_close(generated.deep_seepage_m3, 0.4);
        assert_close(
            Ws10ChannelImpoundmentKernel::contributor_area_ha(&input, Ws10NodeClass::Channel)
                .expect("valid contributor area should project"),
            1.2,
        );
        assert_close(
            Ws10ChannelImpoundmentKernel::dependency_channel_baseflow_m3(
                &input,
                Ws10NodeClass::Channel,
            )
            .expect("dependency baseflow should project"),
            5.0,
        );
        assert_close(
            Ws10ChannelImpoundmentKernel::dependency_channel_deep_seepage_m3(
                &input,
                Ws10NodeClass::Channel,
            )
            .expect("dependency deep seepage should project"),
            0.6,
        );

        let missing_step = test_dispatch_step(
            vec![TopologyNodeKey::new(TopologyNodeKind::Channel, 99)],
            vec![99],
        );
        let missing_input = DirectWatershedKernelInput {
            step: &missing_step,
            frame: &frame,
        };
        assert_eq!(
            Ws10ChannelImpoundmentKernel::generated_groundwater_from_step(
                &missing_input,
                Ws10NodeClass::Channel,
            )
            .expect_err("missing hillslope contribution should fail closed")
            .boundary_class(),
            BoundaryClass::MissingRequiredInput
        );
        assert_eq!(
            Ws10ChannelImpoundmentKernel::dependency_channel_baseflow_m3(
                &missing_input,
                Ws10NodeClass::Channel,
            )
            .expect_err("missing dependency channel state should fail closed")
            .boundary_class(),
            BoundaryClass::MissingRequiredInput
        );
    }

    #[test]
    fn direct_dependency_sediment_accumulation_covers_channel_paths() {
        let mut frame = test_network_frame();
        frame.routed_channels.insert(4, test_routed_channel_state(0.2));
        let step = test_dispatch_step(
            vec![
                TopologyNodeKey::new(TopologyNodeKind::Impoundment, 8),
                TopologyNodeKey::new(TopologyNodeKind::Channel, 4),
            ],
            Vec::new(),
        );
        let input = DirectWatershedKernelInput {
            step: &step,
            frame: &frame,
        };
        let mut accumulator = Ws19SedimentAccumulator::default();

        Ws10ChannelImpoundmentKernel::accumulate_direct_dependency_sediment(
            &input,
            Ws10NodeClass::Channel,
            100.0,
            &mut accumulator,
        )
        .expect("dependency channel sediment should accumulate");

        assert_close(accumulator.incoming_sediment_mass_kg, 20.0);
        assert_eq!(accumulator.class_mass_kg.len(), 2);
        assert!(accumulator.top_class_mass_kg.iter().all(|mass| *mass > 0.0));

        let missing_step = test_dispatch_step(
            vec![TopologyNodeKey::new(TopologyNodeKind::Channel, 99)],
            Vec::new(),
        );
        let missing_input = DirectWatershedKernelInput {
            step: &missing_step,
            frame: &frame,
        };
        assert_eq!(
            Ws10ChannelImpoundmentKernel::accumulate_direct_dependency_sediment(
                &missing_input,
                Ws10NodeClass::Channel,
                100.0,
                &mut Ws19SedimentAccumulator::default(),
            )
            .expect_err("missing dependency sediment channel should fail closed")
            .boundary_class(),
            BoundaryClass::MissingRequiredInput
        );
    }

    #[test]
    fn direct_contributor_groundwater_and_area_guards_fail_closed() {
        let mut frame = test_network_frame();
        let mut bad_baseflow = test_hillslope_contribution();
        bad_baseflow.generated_baseflow_m3 = -1.0;
        frame.add_hillslope_contribution(bad_baseflow);
        let step = test_dispatch_step(Vec::new(), vec![3]);
        let input = DirectWatershedKernelInput {
            step: &step,
            frame: &frame,
        };
        assert_eq!(
            Ws10ChannelImpoundmentKernel::generated_groundwater_from_step(
                &input,
                Ws10NodeClass::Channel,
            )
            .expect_err("negative generated baseflow should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );

        let mut frame = test_network_frame();
        let mut bad_seepage = test_hillslope_contribution();
        bad_seepage.groundwater_deep_seepage_m3 = -1.0;
        frame.add_hillslope_contribution(bad_seepage);
        let input = DirectWatershedKernelInput {
            step: &step,
            frame: &frame,
        };
        assert_eq!(
            Ws10ChannelImpoundmentKernel::generated_groundwater_from_step(
                &input,
                Ws10NodeClass::Channel,
            )
            .expect_err("negative deep seepage should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );

        let mut frame = test_network_frame();
        let mut missing_area = test_hillslope_contribution();
        missing_area.area_m2 = None;
        frame.add_hillslope_contribution(missing_area);
        let input = DirectWatershedKernelInput {
            step: &step,
            frame: &frame,
        };
        assert_eq!(
            Ws10ChannelImpoundmentKernel::contributor_area_ha(&input, Ws10NodeClass::Channel)
                .expect_err("missing contributor area should fail closed")
                .boundary_class(),
            BoundaryClass::MissingRequiredInput
        );

        let mut frame = test_network_frame();
        let mut bad_area = test_hillslope_contribution();
        bad_area.area_m2 = Some(0.0);
        frame.add_hillslope_contribution(bad_area);
        let input = DirectWatershedKernelInput {
            step: &step,
            frame: &frame,
        };
        assert_eq!(
            Ws10ChannelImpoundmentKernel::contributor_area_ha(&input, Ws10NodeClass::Channel)
                .expect_err("non-positive contributor area should fail closed")
                .boundary_class(),
            BoundaryClass::DomainViolation
        );
    }

    #[test]
    fn direct_hillslope_sediment_payload_guards_cover_remaining_paths() {
        let mut invalid_hillslope = test_hillslope_contribution();
        invalid_hillslope.total_detachment_kg = -1.0;
        assert_eq!(
            Ws10ChannelImpoundmentKernel::read_direct_hillslope_sediment_payload(
                &invalid_hillslope,
                Ws10NodeClass::Channel,
            )
            .expect_err("negative detachment should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );

        invalid_hillslope = test_hillslope_contribution();
        invalid_hillslope.total_deposition_kg = -1.0;
        assert_eq!(
            Ws10ChannelImpoundmentKernel::read_direct_hillslope_sediment_payload(
                &invalid_hillslope,
                Ws10NodeClass::Channel,
            )
            .expect_err("negative deposition should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );

        invalid_hillslope = test_hillslope_contribution();
        invalid_hillslope.sediment_concentration_kg_m3.clear();
        assert_eq!(
            Ws10ChannelImpoundmentKernel::read_direct_hillslope_sediment_payload(
                &invalid_hillslope,
                Ws10NodeClass::Channel,
            )
            .expect_err("empty particle class support should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );

        invalid_hillslope = test_hillslope_contribution();
        invalid_hillslope.particle_diameter_m.pop();
        assert_eq!(
            Ws10ChannelImpoundmentKernel::read_direct_hillslope_sediment_payload(
                &invalid_hillslope,
                Ws10NodeClass::Channel,
            )
            .expect_err("mismatched particle class support should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );

        invalid_hillslope = test_hillslope_contribution();
        invalid_hillslope.sediment_concentration_kg_m3[0] = -1.0;
        assert_eq!(
            Ws10ChannelImpoundmentKernel::read_direct_hillslope_sediment_payload(
                &invalid_hillslope,
                Ws10NodeClass::Channel,
            )
            .expect_err("negative concentration should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );

        invalid_hillslope = test_hillslope_contribution();
        invalid_hillslope.particle_diameter_m[0] = 0.0;
        assert_eq!(
            Ws10ChannelImpoundmentKernel::read_direct_hillslope_sediment_payload(
                &invalid_hillslope,
                Ws10NodeClass::Channel,
            )
            .expect_err("zero particle diameter should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );

        invalid_hillslope = test_hillslope_contribution();
        invalid_hillslope.particle_flow_fraction[0] = 1.2;
        assert_eq!(
            Ws10ChannelImpoundmentKernel::read_direct_hillslope_sediment_payload(
                &invalid_hillslope,
                Ws10NodeClass::Channel,
            )
            .expect_err("particle fraction above one should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );

        invalid_hillslope = test_hillslope_contribution();
        invalid_hillslope.hourly_sediment_mass_kg = vec![0.25; 24];
        invalid_hillslope.hourly_sediment_mass_kg[4] = -1.0;
        assert_eq!(
            Ws10ChannelImpoundmentKernel::read_direct_hillslope_sediment_payload(
                &invalid_hillslope,
                Ws10NodeClass::Channel,
            )
            .expect_err("negative hourly sediment mass should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );
    }

    #[test]
    fn direct_ws20_profile_projects_segment_points_and_guards_domains() {
        let control = test_channel_control();

        let profile = Ws10ChannelImpoundmentKernel::direct_ws20_channel_profile(&control, 2)
            .expect("valid profile should project");

        assert_eq!(profile.x_points_ft, vec![0.0, 12.0]);
        assert_eq!(profile.slopes, vec![0.01, 0.015]);
        assert_eq!(profile.width_b_points_ft, vec![2.5, 2.8]);

        let mut invalid = control;
        invalid.segment_points[1].width_b_ft = 0.0;
        let error = Ws10ChannelImpoundmentKernel::direct_ws20_channel_profile(&invalid, 2)
            .expect_err("zero width should fail closed");
        assert_eq!(error.boundary_class(), BoundaryClass::DomainViolation);
    }

    #[test]
    fn direct_ws20_crfrac_normalizes_selected_classes_and_guards_payloads() {
        let control = test_channel_control();

        let crfrac = Ws10ChannelImpoundmentKernel::direct_ws20_crfrac(&control, &[1, 3])
            .expect("valid class fractions should normalize");

        assert!((crfrac[0] - (0.2 / 0.7)).abs() <= 1.0e-12);
        assert!((crfrac[1] - (0.5 / 0.7)).abs() <= 1.0e-12);

        let missing = Ws10ChannelImpoundmentKernel::direct_ws20_crfrac(&control, &[4])
            .expect_err("missing class fraction should fail closed");
        assert_eq!(missing.boundary_class(), BoundaryClass::MissingRequiredInput);

        let mut zero_sum = control;
        zero_sum.crfrac = vec![0.0, 0.0, 0.0];
        let error = Ws10ChannelImpoundmentKernel::direct_ws20_crfrac(&zero_sum, &[1, 2])
            .expect_err("zero fraction support should fail closed");
        assert_eq!(error.boundary_class(), BoundaryClass::DomainViolation);
    }

    #[test]
    fn direct_sediment_accumulator_projects_ingress_and_active_classes() {
        let accumulator = test_sediment_accumulator();

        assert!((accumulator.incoming_sediment_mass_kg - 16.0).abs() <= 1.0e-12);
        assert_eq!(accumulator.class_mass_kg, vec![5.5, 10.5]);
        assert_eq!(accumulator.lateral_class_mass_kg, vec![2.5, 7.5]);
        assert_eq!(accumulator.top_class_mass_kg, vec![3.0, 3.0]);

        let active = Ws10ChannelImpoundmentKernel::direct_active_sediment_classes(
            &accumulator,
            Ws10NodeClass::Channel,
        )
        .expect("positive accumulator should produce active classes");
        assert_eq!(active.class_mass_kg, vec![5.5, 10.5]);
        assert_eq!(active.class_numbers, vec![1, 2]);
        assert!((active.particle_diameters_m[0] - (0.0008 / 5.5)).abs() <= 1.0e-12);
        assert!((active.particle_diameters_m[1] - (0.00465 / 10.5)).abs() <= 1.0e-12);

        let invalid_payload = Ws18HillslopeSedimentPayload {
            mass_kg: 1.0,
            fractions: vec![0.0, 0.0],
            particle_diameters_m: vec![0.0002, 0.0005],
        };
        let mut invalid = Ws19SedimentAccumulator::default();
        let error = Ws10ChannelImpoundmentKernel::add_direct_sediment_payload_to_accumulator(
            &mut invalid,
            &invalid_payload,
            Ws19SedimentIngress::DependencyChannel,
            Ws10NodeClass::Channel,
        )
        .expect_err("positive mass requires a positive dependency fraction sum");
        assert_eq!(error.boundary_class(), BoundaryClass::DomainViolation);
    }

    #[test]
    fn direct_sediment_capacity_additional_guard_paths_are_characterized() {
        let accumulator = test_sediment_accumulator();
        let control = test_channel_control();
        let context = test_sediment_routing_context(&control);

        let zero_qpo_context = Ws19SedimentRoutingContext { qpo: 0.0, ..context };
        assert_eq!(
            Ws10ChannelImpoundmentKernel::direct_terminal_sediment_hydraulics(&zero_qpo_context)
                .expect_err("zero discharge should fail when terminal flow width is zero")
                .boundary_class(),
            BoundaryClass::DomainViolation
        );

        let zero_duration_context = Ws19SedimentRoutingContext {
            event_duration: 0.0,
            ..context
        };
        assert_eq!(
            Ws10ChannelImpoundmentKernel::direct_sediment_transport_capacity(
                &zero_duration_context,
                &accumulator,
            )
            .expect_err("zero sediment duration should fail class load computation")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );

        let sparse_accumulator = Ws19SedimentAccumulator {
            incoming_sediment_mass_kg: 2.0,
            class_mass_kg: vec![0.0, 2.0],
            class_diameter_mass_m: vec![0.0, 0.001],
            top_class_mass_kg: Vec::new(),
            lateral_class_mass_kg: Vec::new(),
        };
        let tc = Ws10ChannelImpoundmentKernel::direct_sediment_transport_capacity(
            &context,
            &sparse_accumulator,
        )
        .expect("zero-mass capacity class should be skipped");
        assert!(tc.is_finite() && tc >= 0.0);

        let class_index_accumulator = Ws19SedimentAccumulator {
            incoming_sediment_mass_kg: 1.0,
            class_mass_kg: vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.0],
            class_diameter_mass_m: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0002],
            top_class_mass_kg: Vec::new(),
            lateral_class_mass_kg: Vec::new(),
        };
        assert_eq!(
            Ws10ChannelImpoundmentKernel::direct_sediment_transport_capacity(
                &context,
                &class_index_accumulator,
            )
            .expect_err("class index beyond default specific gravities should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );
    }

    #[test]
    fn direct_terminal_sediment_hydraulic_guards_are_characterized() {
        let control = test_channel_control();
        let context = test_sediment_routing_context(&control);
        let sediment_controls = context.sediment_controls;

        let missing_context = Ws19SedimentRoutingContext {
            nslpts: 3,
            ..context
        };
        assert_eq!(
            Ws10ChannelImpoundmentKernel::direct_terminal_sediment_hydraulics(&missing_context)
                .expect_err("missing terminal segment should fail closed")
                .boundary_class(),
            BoundaryClass::MissingRequiredInput
        );

        let mut low_slope = control.clone();
        low_slope.segment_points[1].slope = 0.0;
        let low_slope_context = Ws19SedimentRoutingContext {
            control: &low_slope,
            sediment_controls,
            ..context
        };
        assert_eq!(
            Ws10ChannelImpoundmentKernel::direct_terminal_sediment_hydraulics(&low_slope_context)
                .expect_err("low terminal slope should fail closed")
                .boundary_class(),
            BoundaryClass::DomainViolation
        );

        let mut negative_depth = control.clone();
        negative_depth.segment_points[1].depth_b_ft = -1.0;
        let negative_depth_context = Ws19SedimentRoutingContext {
            control: &negative_depth,
            sediment_controls,
            ..context
        };
        assert_eq!(
            Ws10ChannelImpoundmentKernel::direct_terminal_sediment_hydraulics(
                &negative_depth_context,
            )
            .expect_err("negative terminal depth should fail closed")
            .boundary_class(),
            BoundaryClass::DomainViolation
        );

        let bad_shape_context = Ws19SedimentRoutingContext {
            sediment_controls: Ws15ChannelSedimentControls {
                ishape: 4.0,
                ..sediment_controls
            },
            ..context
        };
        assert_eq!(
            Ws10ChannelImpoundmentKernel::direct_terminal_sediment_hydraulics(&bad_shape_context)
                .expect_err("unsupported channel shape should fail closed")
                .boundary_class(),
            BoundaryClass::DomainViolation
        );
    }

    #[test]
    fn direct_sediment_routing_helpers_cover_disabled_and_enabled_paths() {
        let accumulator = test_sediment_accumulator();
        let active = Ws10ChannelImpoundmentKernel::direct_active_sediment_classes(
            &accumulator,
            Ws10NodeClass::Channel,
        )
        .expect("positive accumulator should produce active classes");
        let mut control = test_channel_control();
        control.ws20_case12_enabled = false;
        control.ws21_case34_enabled = false;
        let context = test_sediment_routing_context(&control);

        let routed = Ws10ChannelImpoundmentKernel::route_direct_ws20_sediment_if_enabled(
            &context,
            &active,
            accumulator.incoming_sediment_mass_kg,
        )
        .expect("disabled WS20 branch should preserve active class masses");
        assert_eq!(routed.outgoing_class_mass_kg, active.class_mass_kg);
        assert!(routed.widb_points_ft.is_none());
        assert!(routed.wida_points_ft.is_none());

        let enabled_control = test_channel_control();
        let enabled_context = test_sediment_routing_context(&enabled_control);
        let routed_enabled = Ws10ChannelImpoundmentKernel::route_direct_ws20_sediment_if_enabled(
            &enabled_context,
            &active,
            accumulator.incoming_sediment_mass_kg,
        )
        .expect("enabled WS20 branch should route valid sediment classes");
        assert_eq!(routed_enabled.outgoing_class_mass_kg.len(), active.class_mass_kg.len());
        assert!(routed_enabled.widb_points_ft.is_some());
        assert!(routed_enabled.wida_points_ft.is_some());

        let mut no_crfrac_control = test_channel_control();
        no_crfrac_control.ws21_case34_enabled = false;
        let no_crfrac_context = test_sediment_routing_context(&no_crfrac_control);
        let routed_without_crfrac = Ws10ChannelImpoundmentKernel::route_direct_ws20_sediment_if_enabled(
            &no_crfrac_context,
            &active,
            accumulator.incoming_sediment_mass_kg,
        )
        .expect("enabled WS20 branch without WS21 should route without crfrac");
        assert_eq!(routed_without_crfrac.outgoing_class_mass_kg.len(), active.class_mass_kg.len());
        assert!(routed_without_crfrac.widb_points_ft.is_some());

        let (fractions, diameters) =
            Ws10ChannelImpoundmentKernel::direct_sediment_publication_particles(
                &routed.outgoing_class_mass_kg,
                &active,
                Ws10NodeClass::Channel,
            )
            .expect("positive routed mass should publish class fractions");
        assert!((fractions.iter().sum::<f64>() - 1.0).abs() <= 1.0e-12);
        assert_eq!(diameters.len(), 2);

        let mut hourly_partition = test_peak_partition();
        hourly_partition.hourly_resolved = true;
        hourly_partition.hourly_sediment_inlet_kg[2] = 1.0;
        hourly_partition.hourly_sediment_inlet_kg[3] = 2.0;
        let duration =
            Ws10ChannelImpoundmentKernel::direct_sediment_rate_duration_s(hourly_partition, 200.0);
        assert!((duration - 7200.0).abs() <= 1.0e-12);

        let mut zero_hourly_partition = test_peak_partition();
        zero_hourly_partition.hourly_resolved = true;
        let fallback_duration = Ws10ChannelImpoundmentKernel::direct_sediment_rate_duration_s(
            zero_hourly_partition,
            200.0,
        );
        assert_close(fallback_duration, 200.0);
    }

    #[test]
    fn direct_sediment_capacity_helpers_cover_terminal_and_guard_paths() {
        let accumulator = test_sediment_accumulator();
        let control = test_channel_control();
        let context = test_sediment_routing_context(&control);

        let tc =
            Ws10ChannelImpoundmentKernel::direct_sediment_transport_capacity(&context, &accumulator)
                .expect("terminal capacity should compute for valid controls");
        assert!(tc.is_finite() && tc >= 0.0);

        let invalid_qpo_context = Ws19SedimentRoutingContext {
            qpo: f64::INFINITY,
            ..context
        };
        let error = Ws10ChannelImpoundmentKernel::direct_sediment_transport_capacity(
            &invalid_qpo_context,
            &accumulator,
        )
        .expect_err("non-finite converted discharge should fail closed");
        assert_eq!(error.boundary_class(), BoundaryClass::DomainViolation);

        let mut invalid_accumulator = accumulator.clone();
        invalid_accumulator.class_diameter_mass_m[0] = 0.0;
        let error = Ws10ChannelImpoundmentKernel::direct_sediment_transport_capacity(
            &context,
            &invalid_accumulator,
        )
        .expect_err("positive class mass requires positive capacity diameter");
        assert_eq!(error.boundary_class(), BoundaryClass::DomainViolation);

        let sediment_controls = context.sediment_controls;
        let nslpts = context.nslpts;
        let mut invalid_control = control;
        invalid_control.segment_points[1].width_b_ft = 0.0;
        let invalid_context = Ws19SedimentRoutingContext {
            control: &invalid_control,
            node_class: Ws10NodeClass::Channel,
            event_duration: 200.0,
            qpo: 0.3,
            roughness: invalid_control.chnn,
            sediment_controls,
            nslpts,
            peak_partition: test_peak_partition(),
        };
        let error = Ws10ChannelImpoundmentKernel::direct_sediment_transport_capacity(
            &invalid_context,
            &accumulator,
        )
        .expect_err("invalid terminal width should fail closed");
        assert_eq!(error.boundary_class(), BoundaryClass::DomainViolation);
    }

    #[test]
    fn read_direct_channel_sediment_payload_covers_zero_valid_and_guard_paths() {
        let zero_payload = Ws10ChannelImpoundmentKernel::read_direct_channel_sediment_payload(
            &test_routed_channel_state(0.0),
            Ws10NodeClass::Channel,
            100.0,
        )
        .expect("zero qsed is a valid zero-payload dependency");
        assert!(zero_payload.mass_kg.abs() <= 1.0e-12);
        assert!(zero_payload.fractions.is_empty());

        let payload = Ws10ChannelImpoundmentKernel::read_direct_channel_sediment_payload(
            &test_routed_channel_state(0.2),
            Ws10NodeClass::Channel,
            100.0,
        )
        .expect("positive qsed with class support should project");
        assert!((payload.mass_kg - 20.0).abs() <= 1.0e-12);
        assert_eq!(payload.fractions, vec![0.4, 0.6]);
        assert_eq!(payload.particle_diameters_m, vec![0.0001, 0.0003]);

        let mut invalid = test_routed_channel_state(0.2);
        invalid.sediment_state.particle_flow_fraction.clear();
        let error = Ws10ChannelImpoundmentKernel::read_direct_channel_sediment_payload(
            &invalid,
            Ws10NodeClass::Channel,
            100.0,
        )
        .expect_err("positive qsed needs class support");
        assert_eq!(error.boundary_class(), BoundaryClass::DomainViolation);
    }

    #[test]
    fn read_direct_hillslope_sediment_payload_prefers_hourly_mass_authority() {
        let mut contribution = test_hillslope_contribution();
        contribution.hourly_sediment_mass_kg = vec![0.25; 24];

        let payload = Ws10ChannelImpoundmentKernel::read_direct_hillslope_sediment_payload(
            &contribution,
            Ws10NodeClass::Channel,
        )
        .expect("hourly sediment authority should project");

        assert!((payload.mass_kg - 6.0).abs() <= 1.0e-12);
        assert_eq!(payload.fractions, vec![0.25, 0.75]);

        let mut invalid = contribution;
        invalid.particle_flow_fraction = vec![0.0, 0.0];
        let error = Ws10ChannelImpoundmentKernel::read_direct_hillslope_sediment_payload(
            &invalid,
            Ws10NodeClass::Channel,
        )
        .expect_err("positive sediment mass requires positive fraction support");
        assert_eq!(error.boundary_class(), BoundaryClass::DomainViolation);
    }
}
