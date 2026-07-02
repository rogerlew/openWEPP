//! Deterministic watershed dispatch scheduler for openWEPP.

pub mod runtime_inputs;

mod lib_mod;

pub use lib_mod::{
    DispatchDiagnostic, DispatchDiagnosticCode, DispatchStep, HillslopeContribution,
    MESSAGE_CYCLE_DETECTED, MESSAGE_DISPATCH_OK, MESSAGE_MISSING_DEPENDENCY,
    MESSAGE_PRECONDITION_FAILED, RoutedChannelSedimentState, RoutedChannelState,
    RoutedChannelWaveState, RoutedImpoundmentState, WatershedChannelControlRecord,
    WatershedChannelRatingCurveControl, WatershedChannelSegmentPoint, WatershedDispatchError,
    WatershedDispatchReport, WatershedFrameExecutionReport, WatershedFrameStepReport,
    WatershedImpoundmentControlRecord, WatershedNetworkFrame, WatershedNetworkFrameError,
    WatershedPublicationFrame, WatershedRoutingGlobals, Ws10ChannelImpoundmentKernel,
    execute_watershed_dispatch_with_frame, schedule_watershed_dispatch,
    schedule_watershed_dispatch_with_gate,
};

#[cfg(test)]
mod tests {
    use super::lib_mod::kernel::{WS22_DCAP_MAXE, Ws10NodeClass};
    use openwepp_sim_contract::status::{
        BoundaryClass, SimulationPhase, SimulationStatus, StatusClassification,
    };
    use openwepp_topology::{
        ContributorTriplet, TopologyContributors, TopologyGraph, TopologyNode, TopologyNodeKey,
        TopologyNodeKind, TopologyValidationReport, validate_pre_execution_topology,
    };

    use super::*;

    #[test]
    fn schedules_dispatch_in_deterministic_dependency_order() {
        let graph = TopologyGraph::new(
            4,
            3,
            2,
            vec![
                node(
                    TopologyNodeKind::Channel,
                    1,
                    [1, 0, 0],
                    [0, 0, 0],
                    [0, 0, 0],
                ),
                node(
                    TopologyNodeKind::Impoundment,
                    1,
                    [2, 0, 0],
                    [0, 0, 0],
                    [0, 0, 0],
                ),
                node(
                    TopologyNodeKind::Channel,
                    2,
                    [0, 0, 0],
                    [1, 0, 0],
                    [1, 0, 0],
                ),
                node(
                    TopologyNodeKind::Impoundment,
                    2,
                    [0, 0, 0],
                    [1, 0, 0],
                    [0, 0, 0],
                ),
                node(
                    TopologyNodeKind::Channel,
                    3,
                    [3, 0, 0],
                    [2, 0, 0],
                    [2, 0, 0],
                ),
            ],
        );

        let topology_validation =
            validate_pre_execution_topology(&graph).expect("topology validation should construct");
        assert!(topology_validation.is_valid());

        let report =
            schedule_watershed_dispatch(&graph, &topology_validation).expect("schedule should run");

        assert!(report.is_success());
        assert_eq!(
            report.dispatch_status.classification(),
            StatusClassification::Nominal
        );
        assert!(report.diagnostics.is_empty());

        let observed_order: Vec<TopologyNodeKey> =
            report.steps.iter().map(|step| step.node).collect();
        let expected_order = vec![
            key(TopologyNodeKind::Channel, 1),
            key(TopologyNodeKind::Impoundment, 1),
            key(TopologyNodeKind::Channel, 2),
            key(TopologyNodeKind::Impoundment, 2),
            key(TopologyNodeKind::Channel, 3),
        ];

        assert_eq!(observed_order, expected_order);

        let channel_two = &report.steps[2];
        assert_eq!(
            channel_two.dependency_nodes,
            vec![
                key(TopologyNodeKind::Channel, 1),
                key(TopologyNodeKind::Impoundment, 1),
            ]
        );

        let impoundment_two = &report.steps[3];
        assert_eq!(
            impoundment_two.dependency_nodes,
            vec![key(TopologyNodeKind::Channel, 1)]
        );

        for step in &report.steps {
            assert_eq!(step.status.phase(), SimulationPhase::WatershedKernel);
            assert_eq!(step.status.boundary_class(), BoundaryClass::Ok);
            assert_eq!(step.status.classification(), StatusClassification::Nominal);
        }
    }

    #[test]
    fn blocks_dispatch_when_topology_precondition_fails() {
        let graph = TopologyGraph::new(
            1,
            2,
            0,
            vec![node(
                TopologyNodeKind::Channel,
                1,
                [1, 0, 0],
                [0, 0, 0],
                [0, 0, 0],
            )],
        );

        let topology_validation =
            validate_pre_execution_topology(&graph).expect("topology validation should construct");
        assert!(!topology_validation.is_valid());

        let report =
            schedule_watershed_dispatch(&graph, &topology_validation).expect("schedule should run");

        assert!(!report.is_success());
        assert!(report.steps.is_empty());
        assert_eq!(
            report.precondition_status.classification(),
            StatusClassification::Failure
        );
        assert_eq!(
            report.dispatch_status.classification(),
            StatusClassification::Failure
        );
        assert_eq!(
            report.dispatch_status.boundary_class(),
            BoundaryClass::TopologyInvalid
        );
        assert_eq!(
            report.dispatch_status.message_id(),
            MESSAGE_PRECONDITION_FAILED
        );
        assert_eq!(report.diagnostics.len(), 1);
        assert_eq!(
            report.diagnostics[0].code,
            DispatchDiagnosticCode::TopologyPreconditionFailed
        );
    }

    #[test]
    fn classifies_cycle_as_typed_failure_class() {
        let graph = TopologyGraph::new(
            1,
            2,
            0,
            vec![
                node(
                    TopologyNodeKind::Channel,
                    1,
                    [1, 0, 0],
                    [2, 0, 0],
                    [0, 0, 0],
                ),
                node(
                    TopologyNodeKind::Channel,
                    2,
                    [1, 0, 0],
                    [1, 0, 0],
                    [0, 0, 0],
                ),
            ],
        );

        let forged_valid = TopologyValidationReport {
            status: SimulationStatus::ok(
                SimulationPhase::PreExecutionValidation,
                "TOPOLOGY-OK-001",
            )
            .expect("status should construct"),
            violations: Vec::new(),
        };

        let report =
            schedule_watershed_dispatch(&graph, &forged_valid).expect("schedule should run");

        assert!(!report.is_success());
        assert!(report.steps.is_empty());
        assert_eq!(
            report.dispatch_status.classification(),
            StatusClassification::Failure
        );
        assert_eq!(
            report.dispatch_status.boundary_class(),
            BoundaryClass::TopologyInvalid
        );
        assert_eq!(report.dispatch_status.message_id(), MESSAGE_CYCLE_DETECTED);
        assert_eq!(report.diagnostics.len(), 1);
        assert_eq!(
            report.diagnostics[0].code,
            DispatchDiagnosticCode::DependencyCycleDetected
        );
    }

    #[test]
    fn classifies_missing_dependency_as_typed_failure_class() {
        let graph = TopologyGraph::new(
            1,
            1,
            0,
            vec![node(
                TopologyNodeKind::Channel,
                1,
                [1, 0, 0],
                [2, 0, 0],
                [0, 0, 0],
            )],
        );

        let forged_valid = TopologyValidationReport {
            status: SimulationStatus::ok(
                SimulationPhase::PreExecutionValidation,
                "TOPOLOGY-OK-001",
            )
            .expect("status should construct"),
            violations: Vec::new(),
        };

        let report =
            schedule_watershed_dispatch(&graph, &forged_valid).expect("schedule should run");

        assert!(!report.is_success());
        assert_eq!(
            report.dispatch_status.classification(),
            StatusClassification::Failure
        );
        assert_eq!(
            report.dispatch_status.boundary_class(),
            BoundaryClass::TopologyInvalid
        );
        assert_eq!(
            report.dispatch_status.message_id(),
            MESSAGE_MISSING_DEPENDENCY
        );
        assert_eq!(report.diagnostics.len(), 1);
        assert_eq!(
            report.diagnostics[0].code,
            DispatchDiagnosticCode::MissingDependency
        );
    }

    #[test]
    fn wshedimpl26_dcap_flagm2_caps_detachment_rate_at_maxe() {
        let crfrac = vec![0.2, 0.3, 0.5];
        let common = (
            Ws10NodeClass::Channel,
            10.0,
            0.05,
            0.03,
            20.0,
            120.0,
            1.0,
            20_000.0,
            1.0,
            1.0,
            0.05,
            0.0,
            1.0,
            100.0,
            3,
            100.0,
            0.04,
            WS22_DCAP_MAXE,
        );

        let df_flagm1 = Ws10ChannelImpoundmentKernel::ws26_dcap(
            common.0, 1, common.1, common.2, common.3, common.4, common.5, common.6, common.7,
            common.8, common.9, common.10, common.11, common.12, common.13, common.14, common.15,
            common.16, common.17, &crfrac,
        )
        .expect("flagm1 detachment capacity should evaluate");
        let df_flagm2 = Ws10ChannelImpoundmentKernel::ws26_dcap(
            common.0, 2, common.1, common.2, common.3, common.4, common.5, common.6, common.7,
            common.8, common.9, common.10, common.11, common.12, common.13, common.14, common.15,
            common.16, common.17, &crfrac,
        )
        .expect("flagm2 detachment capacity should evaluate");

        let sum_flagm1 = df_flagm1.df_lbs_s_ft2.iter().sum::<f64>();
        let sum_flagm2 = df_flagm2.df_lbs_s_ft2.iter().sum::<f64>();
        assert!(
            sum_flagm1 > WS22_DCAP_MAXE,
            "expected uncapped flagm1 detachment > maxe, got {sum_flagm1}"
        );
        assert!(
            (sum_flagm2 - WS22_DCAP_MAXE).abs() <= 1e-9,
            "expected flagm2 detachment capped at maxe, got {sum_flagm2}"
        );
    }

    #[test]
    fn wshedimpl27_enddet_helper_exercises_xdbig_and_midpoint_rebracketing() {
        let class_count = 2;
        let mut potld_case4_lbs_s_ft = vec![1.0; class_count];
        let mut tcl_case4_lbs_s_ft = vec![0.1; class_count];
        let mut trncap_call = 0_u8;
        let progress = Ws10ChannelImpoundmentKernel::ws27_case4_enddet_bracket_closure(
            0.0,
            10.0,
            1.0,
            10.0,
            &[1.0, 1.0],
            &[0.0, 0.0],
            &[2.0, 2.0],
            &mut potld_case4_lbs_s_ft,
            &mut tcl_case4_lbs_s_ft,
            |potld| {
                trncap_call = trncap_call.saturating_add(1);
                match trncap_call {
                    1 => vec![0.1, 0.1],
                    2 => vec![100.0, 100.0],
                    _ => potld.to_vec(),
                }
            },
        );

        assert!(progress.used_xdbig_rebracket);
        assert!(progress.used_midpoint_rebracket);
        assert!(
            progress.iteration_count >= 3,
            "expected >=3 iterations to cover xdbig + midpoint branches, got {}",
            progress.iteration_count
        );
    }

    fn key(kind: TopologyNodeKind, id: u32) -> TopologyNodeKey {
        TopologyNodeKey::new(kind, id)
    }

    fn node(
        kind: TopologyNodeKind,
        id: u32,
        hillslope: [u32; 3],
        channels: [u32; 3],
        impoundments: [u32; 3],
    ) -> TopologyNode {
        let contributors = TopologyContributors::new(
            ContributorTriplet::new(hillslope[0], hillslope[1], hillslope[2]),
            ContributorTriplet::new(channels[0], channels[1], channels[2]),
            ContributorTriplet::new(impoundments[0], impoundments[1], impoundments[2]),
        );

        TopologyNode::new(key(kind, id), contributors)
    }
}
