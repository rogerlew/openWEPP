#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]

//! Kernel invocation and writeback contract boundaries for openWEPP.

pub mod lib_mod;
pub use lib_mod::*;

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use openwepp_sim_contract::status::{
        BoundaryClass, SimulationPhase, SimulationStatus, StatusClassification,
    };
    use openwepp_unit_boundary::{FlowRateCubicMetersPerSecond, StorageVolumeCubicMeters};

    use super::*;

    #[test]
    fn accepts_finite_domain_valid_payload() {
        let payload = KernelWritebackPayload::with_updates(
            vec![WritebackField::bounded("st", 10.0, Some(0.0), None)],
            vec![WritebackField::unbounded("runoff", 1.5)],
        );

        let decision = evaluate_kernel_writeback(SimulationPhase::HillslopeKernel, &payload)
            .expect("decision should construct");

        assert_eq!(decision.outcome, WritebackDecisionOutcome::Accept);
        assert_eq!(
            decision.status.classification(),
            StatusClassification::Nominal
        );
        assert!(decision.violations.is_empty());
    }

    #[test]
    fn accepts_unit_boundary_typed_values() {
        let storage = StorageVolumeCubicMeters::try_new(12.0).expect("storage should construct");
        let flow = FlowRateCubicMetersPerSecond::try_new(0.25).expect("flow should construct");
        let payload = KernelWritebackPayload::with_updates(
            vec![WritebackField::bounded("st", storage, Some(0.0), None)],
            vec![WritebackField::bounded("qout", flow, Some(0.0), None)],
        );

        let decision = evaluate_kernel_writeback(SimulationPhase::HillslopeKernel, &payload)
            .expect("decision should construct");

        assert_eq!(decision.outcome, WritebackDecisionOutcome::Accept);
        assert!(decision.violations.is_empty());
    }

    #[test]
    fn rejects_non_finite_payload_with_typed_status() {
        let payload = KernelWritebackPayload::with_updates(
            vec![WritebackField::unbounded("st", f64::NAN)],
            Vec::new(),
        );

        let decision = evaluate_kernel_writeback(SimulationPhase::HillslopeKernel, &payload)
            .expect("decision should construct");

        assert_eq!(decision.outcome, WritebackDecisionOutcome::Reject);
        assert_eq!(
            decision.status.classification(),
            StatusClassification::Failure
        );
        assert_eq!(
            decision.status.message_id(),
            WRITEBACK_REJECT_NON_FINITE_MESSAGE_ID
        );
        assert_eq!(decision.violations.len(), 1);
    }

    #[test]
    fn apply_requires_accept_outcome() {
        let payload = KernelWritebackPayload::empty();
        let reject_decision = KernelWritebackDecision {
            outcome: WritebackDecisionOutcome::Reject,
            status: SimulationStatus::domain_failure(
                SimulationPhase::WatershedKernel,
                BoundaryClass::DomainViolation,
                WRITEBACK_REJECT_DOMAIN_MESSAGE_ID,
            )
            .expect("status should construct"),
            violations: Vec::new(),
        };
        let mut state = BTreeMap::new();
        let mut flux = BTreeMap::new();

        let error = apply_kernel_writeback(
            SimulationPhase::WatershedKernel,
            &reject_decision,
            &payload,
            &mut state,
            &mut flux,
        )
        .expect_err("reject decision should not apply");

        assert!(matches!(
            error,
            WritebackError::DecisionNotAccept {
                outcome: WritebackDecisionOutcome::Reject
            }
        ));
    }

    #[test]
    fn climate_forcing_symbol_surface_hillslope_uses_canonical_aliases() {
        let surface = ClimateForcingSymbolSurface::hillslope(3)
            .expect("hillslope symbol surface should build");

        assert_eq!(surface.point_count(), 3);
        assert_eq!(surface.timem_symbols()[0].as_str(), "timem_0001");
        assert_eq!(surface.timem_symbols()[2].as_str(), "timem_0003");
        assert_eq!(surface.intsty_symbols()[0].as_str(), "intsty_0001");
        assert_eq!(surface.intsty_symbols()[2].as_str(), "intsty_0003");
    }

    #[test]
    fn climate_forcing_symbol_surface_watershed_scope_uses_canonical_aliases() {
        let surface = ClimateForcingSymbolSurface::watershed_hillslope(42, 2)
            .expect("watershed symbol surface should build");

        assert_eq!(surface.point_count(), 2);
        assert_eq!(surface.timem_symbols()[0].as_str(), "hs42_timem_0001");
        assert_eq!(surface.timem_symbols()[1].as_str(), "hs42_timem_0002");
        assert_eq!(surface.intsty_symbols()[0].as_str(), "hs42_intsty_0001");
        assert_eq!(surface.intsty_symbols()[1].as_str(), "hs42_intsty_0002");
    }

    #[test]
    fn climate_forcing_symbol_surface_rejects_unsupported_point_count() {
        let error = ClimateForcingSymbolSurface::hillslope(MAX_CLIMATE_FORCING_SERIES_POINTS + 1)
            .expect_err("point count above supported maximum should fail");

        assert!(matches!(
            error,
            ClimateForcingSymbolSurfaceError::PointCountOutOfRange {
                count,
                supported_max
            } if count == MAX_CLIMATE_FORCING_SERIES_POINTS + 1
                && supported_max == MAX_CLIMATE_FORCING_SERIES_POINTS
        ));
    }

    #[test]
    fn phase_class_growth_predicate_matches_contract() {
        assert!(!HillslopeKernelPhaseClass::Hydrology.is_growth_transition());
        assert!(!HillslopeKernelPhaseClass::HydrologyEvapotranspiration.is_growth_transition());
        assert!(!HillslopeKernelPhaseClass::HydrologyPercolationDeepSeepage.is_growth_transition());
        assert!(!HillslopeKernelPhaseClass::HydrologyLateralTransfer.is_growth_transition());
        assert!(!HillslopeKernelPhaseClass::HydrologyDrainage.is_growth_transition());
        assert!(!HillslopeKernelPhaseClass::HydrologyPlantRootUptake.is_growth_transition());
        assert!(!HillslopeKernelPhaseClass::HydrologyRunoffReconciliation.is_growth_transition());
        assert!(!HillslopeKernelPhaseClass::HydrologyStorageReconciliation.is_growth_transition());
        assert!(!HillslopeKernelPhaseClass::HydrologyPeakRunoff.is_growth_transition());
        assert!(!HillslopeKernelPhaseClass::DecompositionTransition.is_growth_transition());
        assert!(!HillslopeKernelPhaseClass::ResiduePartitionTransition.is_growth_transition());
        assert!(HillslopeKernelPhaseClass::GrowthAnnualTransition.is_growth_transition());
        assert!(HillslopeKernelPhaseClass::GrowthPerennialTransition.is_growth_transition());
    }

    #[test]
    fn phase_class_decomposition_predicate_matches_contract() {
        assert!(!HillslopeKernelPhaseClass::Hydrology.is_decomposition_transition());
        assert!(
            !HillslopeKernelPhaseClass::HydrologyEvapotranspiration.is_decomposition_transition()
        );
        assert!(
            !HillslopeKernelPhaseClass::HydrologyPercolationDeepSeepage
                .is_decomposition_transition()
        );
        assert!(!HillslopeKernelPhaseClass::HydrologyLateralTransfer.is_decomposition_transition());
        assert!(!HillslopeKernelPhaseClass::HydrologyDrainage.is_decomposition_transition());
        assert!(!HillslopeKernelPhaseClass::HydrologyPlantRootUptake.is_decomposition_transition());
        assert!(
            !HillslopeKernelPhaseClass::HydrologyRunoffReconciliation.is_decomposition_transition()
        );
        assert!(
            !HillslopeKernelPhaseClass::HydrologyStorageReconciliation
                .is_decomposition_transition()
        );
        assert!(!HillslopeKernelPhaseClass::HydrologyPeakRunoff.is_decomposition_transition());
        assert!(HillslopeKernelPhaseClass::DecompositionTransition.is_decomposition_transition());
        assert!(
            HillslopeKernelPhaseClass::ResiduePartitionTransition.is_decomposition_transition()
        );
        assert!(!HillslopeKernelPhaseClass::GrowthAnnualTransition.is_decomposition_transition());
        assert!(
            !HillslopeKernelPhaseClass::GrowthPerennialTransition.is_decomposition_transition()
        );
    }

    #[test]
    fn phase_class_hydrology_predicate_matches_contract() {
        assert!(HillslopeKernelPhaseClass::Hydrology.is_hydrology_phase());
        assert!(HillslopeKernelPhaseClass::HydrologyEvapotranspiration.is_hydrology_phase());
        assert!(HillslopeKernelPhaseClass::HydrologyPercolationDeepSeepage.is_hydrology_phase());
        assert!(HillslopeKernelPhaseClass::HydrologyLateralTransfer.is_hydrology_phase());
        assert!(HillslopeKernelPhaseClass::HydrologyDrainage.is_hydrology_phase());
        assert!(HillslopeKernelPhaseClass::HydrologyPlantRootUptake.is_hydrology_phase());
        assert!(HillslopeKernelPhaseClass::HydrologyRunoffReconciliation.is_hydrology_phase());
        assert!(HillslopeKernelPhaseClass::HydrologyStorageReconciliation.is_hydrology_phase());
        assert!(HillslopeKernelPhaseClass::HydrologyPeakRunoff.is_hydrology_phase());
        assert!(!HillslopeKernelPhaseClass::DecompositionTransition.is_hydrology_phase());
        assert!(!HillslopeKernelPhaseClass::ResiduePartitionTransition.is_hydrology_phase());
        assert!(!HillslopeKernelPhaseClass::GrowthAnnualTransition.is_hydrology_phase());
        assert!(!HillslopeKernelPhaseClass::GrowthPerennialTransition.is_hydrology_phase());
    }

    #[test]
    fn request_with_growth_context_preserves_typed_phase_metadata() {
        let state_surface = BTreeMap::new();
        let flux_surface = BTreeMap::new();
        let growth_context =
            HillslopeGrowthKernelContext::new(HillslopeGrowthManagementClass::Perennial, 1.0, 1.0);

        let request = HillslopeKernelRequest::with_phase_context(
            "perennial_growth_transition",
            HillslopeKernelPhaseClass::GrowthPerennialTransition,
            HillslopeConsumerAdapter::Growth,
            Some(growth_context),
            &state_surface,
            &flux_surface,
        );

        assert_eq!(
            request.phase_class,
            HillslopeKernelPhaseClass::GrowthPerennialTransition
        );
        assert_eq!(request.consumer_adapter, HillslopeConsumerAdapter::Growth);
        assert_eq!(request.decomposition_context, None);
        assert_eq!(request.growth_context, Some(growth_context));
    }

    #[test]
    fn request_with_decomposition_context_preserves_typed_phase_metadata() {
        let state_surface = BTreeMap::new();
        let flux_surface = BTreeMap::new();
        let decomposition_context = HillslopeDecompositionKernelContext::new(
            HillslopeDecompositionManagementClass::AnnualOrFallow,
            1.0,
            1.0,
        );

        let request = HillslopeKernelRequest::with_transition_context(
            "decomposition_transition",
            HillslopeKernelPhaseClass::DecompositionTransition,
            HillslopeConsumerAdapter::Decomposition,
            Some(decomposition_context),
            None,
            &state_surface,
            &flux_surface,
        );

        assert_eq!(
            request.phase_class,
            HillslopeKernelPhaseClass::DecompositionTransition
        );
        assert_eq!(
            request.consumer_adapter,
            HillslopeConsumerAdapter::Decomposition
        );
        assert_eq!(request.decomposition_context, Some(decomposition_context));
        assert_eq!(request.growth_context, None);
    }

    #[test]
    fn decomposition_context_can_carry_typed_transition_payload() {
        let payload = HillslopeDecompositionTransitionPayload {
            active_slot_index: 1,
            active_crop_slot_index: 1,
            runtime_day_of_year: 200,
            iresd_seed: 3.0,
            sumrtm_seed: 2.5,
            sumsrm_seed: 1.5,
            control: HillslopeDecompositionTransitionControl::Annual(
                HillslopeAnnualDecompositionControl {
                    resmgt: 1,
                    jdherb: 200,
                    jdburn: 0,
                    jdslge: 0,
                    jdcut: 0,
                    jdmove: 0,
                    fbrnag: 0.0,
                    fbrnog: 0.0,
                    frcut: 0.0,
                    frmove: 0.0,
                    active_action: HillslopeAnnualDecompositionAction::Herbicide,
                },
            ),
        };
        let context = HillslopeDecompositionKernelContext::new(
            HillslopeDecompositionManagementClass::AnnualOrFallow,
            1.0,
            1.0,
        )
        .with_transition_payload(payload);

        assert_eq!(context.transition_payload, Some(payload));
    }

    #[test]
    fn growth_context_can_carry_typed_transition_payload() {
        let payload = HillslopeGrowthTransitionPayload {
            active_slot_index: 1,
            active_crop_slot_index: 1,
            runtime_day_of_year: 200,
            state_before: HillslopeGrowthStateSurface {
                sumgdd: 800.0,
                vdmt: 2.4,
                cancov: 0.65,
                lai: 2.1,
                rtmass: 1.0,
                rtd: 0.35,
                hia: 0.45,
            },
            state_after: HillslopeGrowthStateSurface {
                sumgdd: 0.0,
                vdmt: 0.0,
                cancov: 0.0,
                lai: 0.0,
                rtmass: 0.0,
                rtd: 0.0,
                hia: 0.0,
            },
            control: HillslopeGrowthTransitionControl::Annual(HillslopeAnnualGrowthControl {
                jdharv: 240,
                jdplt: 120,
                rw: 1.3,
                active_action: HillslopeAnnualGrowthAction::HarvestReset,
            }),
        };
        let context = HillslopeGrowthKernelContext::new(
            HillslopeGrowthManagementClass::AnnualOrFallow,
            1.0,
            1.0,
        )
        .with_transition_payload(payload);

        assert_eq!(context.transition_payload, Some(payload));
    }
}
