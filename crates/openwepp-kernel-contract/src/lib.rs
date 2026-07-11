#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]

//! Kernel invocation and writeback contract boundaries for openWEPP.

pub mod lib_mod;
pub use lib_mod::*;

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use openwepp_sim_contract::closure::ClosureViolationKind;
    use openwepp_sim_contract::status::{
        BoundaryClass, SimulationPhase, SimulationStatus, StatusClassification, StatusError,
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
        assert_eq!(decision.status.message_id(), WRITEBACK_ACCEPT_MESSAGE_ID);
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
    fn writeback_errors_preserve_display_sources_and_conversions() {
        let status_error = WritebackError::from(StatusError::MessageIdEmpty);
        assert_eq!(
            status_error.to_string(),
            "failed constructing writeback status: message_id must not be empty"
        );
        assert!(std::error::Error::source(&status_error).is_some());

        let decision_error = WritebackError::DecisionNotAccept {
            outcome: WritebackDecisionOutcome::Reject,
        };
        assert_eq!(
            decision_error.to_string(),
            "cannot apply writeback for non-accept outcome: Reject"
        );
        assert!(std::error::Error::source(&decision_error).is_none());

        let registry_error = WritebackError::from(SymbolRegistryError::UnknownSymbol {
            symbol: BoundarySymbol::from("unknown"),
        });
        assert_eq!(
            registry_error.to_string(),
            "indexed writeback registry error: symbol unknown is not present in the frozen registry"
        );
        assert!(std::error::Error::source(&registry_error).is_some());
    }

    #[test]
    fn indexed_writeback_evaluation_accepts_inclusive_boundaries() {
        let registry = SymbolRegistry::from_symbols(["state", "flux"])
            .expect("writeback symbols should register");
        let state_id = registry
            .id_of(&BoundarySymbol::from("state"))
            .expect("state id should resolve");
        let flux_id = registry
            .id_of(&BoundarySymbol::from("flux"))
            .expect("flux id should resolve");

        let accepted = IndexedKernelWritebackPayload::with_updates(
            vec![IndexedWritebackField::bounded(
                state_id,
                BoundaryValue::from(0.0),
                Some(0.0),
                Some(2.0),
            )],
            vec![IndexedWritebackField::bounded(
                flux_id,
                BoundaryValue::from(2.0),
                Some(0.0),
                Some(2.0),
            )],
        );
        let accepted_decision =
            evaluate_indexed_kernel_writeback(SimulationPhase::HillslopeKernel, &accepted)
                .expect("accepted indexed decision should construct");
        assert_eq!(accepted_decision.outcome, WritebackDecisionOutcome::Accept);
        assert_eq!(
            accepted_decision.status.message_id(),
            WRITEBACK_ACCEPT_MESSAGE_ID
        );
        assert!(accepted_decision.violations.is_empty());
    }

    #[test]
    fn indexed_writeback_evaluation_reports_ordered_domain_violations() {
        let registry = SymbolRegistry::from_symbols(["state", "flux"])
            .expect("writeback symbols should register");
        let state_id = registry
            .id_of(&BoundarySymbol::from("state"))
            .expect("state id should resolve");
        let flux_id = registry
            .id_of(&BoundarySymbol::from("flux"))
            .expect("flux id should resolve");

        let domain_invalid = IndexedKernelWritebackPayload::with_updates(
            vec![
                IndexedWritebackField::bounded(
                    state_id,
                    BoundaryValue::from(3.0),
                    Some(0.0),
                    Some(2.0),
                ),
                IndexedWritebackField::bounded(
                    state_id,
                    BoundaryValue::from(-1.0),
                    Some(0.0),
                    None,
                ),
                IndexedWritebackField::bounded(
                    state_id,
                    BoundaryValue::from(1.0),
                    Some(2.0),
                    Some(0.0),
                ),
            ],
            vec![IndexedWritebackField::bounded(
                flux_id,
                BoundaryValue::from(3.0),
                None,
                Some(2.0),
            )],
        );
        let domain_decision =
            evaluate_indexed_kernel_writeback(SimulationPhase::WatershedKernel, &domain_invalid)
                .expect("domain-reject decision should construct");
        assert_eq!(domain_decision.outcome, WritebackDecisionOutcome::Reject);
        assert_eq!(
            domain_decision.status.message_id(),
            WRITEBACK_REJECT_DOMAIN_MESSAGE_ID
        );
        assert_eq!(domain_decision.violations.len(), 4);
        assert_eq!(
            domain_decision
                .violations
                .iter()
                .map(|violation| (
                    violation.check_id.as_str(),
                    violation.message_id.as_str(),
                    violation.kind,
                ))
                .collect::<Vec<_>>(),
            vec![
                (
                    "INV-WRITEBACK-002",
                    WRITEBACK_REJECT_DOMAIN_MESSAGE_ID,
                    ClosureViolationKind::DomainRange,
                ),
                (
                    "INV-WRITEBACK-003",
                    WRITEBACK_REJECT_DOMAIN_MESSAGE_ID,
                    ClosureViolationKind::DomainLowerBound,
                ),
                (
                    "INV-WRITEBACK-002",
                    "CLOSURE-PRIMITIVE-INVALID-BOUNDS",
                    ClosureViolationKind::DomainRange,
                ),
                (
                    "INV-WRITEBACK-004",
                    WRITEBACK_REJECT_DOMAIN_MESSAGE_ID,
                    ClosureViolationKind::DomainUpperBound,
                ),
            ]
        );
    }

    #[test]
    fn indexed_writeback_evaluation_prioritizes_nonfinite_failures() {
        let registry =
            SymbolRegistry::from_symbols(["state"]).expect("writeback symbol should register");
        let state_id = registry
            .id_of(&BoundarySymbol::from("state"))
            .expect("state id should resolve");
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let nonfinite = IndexedKernelWritebackPayload::with_updates(
                vec![IndexedWritebackField::bounded(
                    state_id,
                    BoundaryValue::from(value),
                    Some(0.0),
                    Some(1.0),
                )],
                Vec::new(),
            );
            let decision =
                evaluate_indexed_kernel_writeback(SimulationPhase::HillslopeKernel, &nonfinite)
                    .expect("non-finite reject decision should construct");
            assert_eq!(decision.outcome, WritebackDecisionOutcome::Reject);
            assert_eq!(
                decision.status.message_id(),
                WRITEBACK_REJECT_NON_FINITE_MESSAGE_ID
            );
            assert_eq!(decision.violations.len(), 2);
            assert_eq!(
                decision
                    .violations
                    .iter()
                    .map(|violation| (violation.check_id.as_str(), violation.kind))
                    .collect::<Vec<_>>(),
                vec![
                    ("INV-WRITEBACK-001", ClosureViolationKind::NonFinite),
                    ("INV-WRITEBACK-002", ClosureViolationKind::DomainRange),
                ]
            );
        }
    }

    #[test]
    fn indexed_writeback_application_sorts_and_updates_both_surfaces() {
        let registry = SymbolRegistry::from_symbols(["state_z", "state_a", "flux_q"])
            .expect("writeback symbols should register");
        let upper_state_id = registry
            .id_of(&BoundarySymbol::from("state_z"))
            .expect("state_z id should resolve");
        let lower_state_id = registry
            .id_of(&BoundarySymbol::from("state_a"))
            .expect("state_a id should resolve");
        let flux_q_id = registry
            .id_of(&BoundarySymbol::from("flux_q"))
            .expect("flux_q id should resolve");
        let payload = IndexedKernelWritebackPayload::with_updates(
            vec![
                IndexedWritebackField::unbounded(upper_state_id, BoundaryValue::from(9.0)),
                IndexedWritebackField::unbounded(lower_state_id, BoundaryValue::from(1.0)),
            ],
            vec![IndexedWritebackField::unbounded(
                flux_q_id,
                BoundaryValue::from(4.0),
            )],
        );
        let decision =
            evaluate_indexed_kernel_writeback(SimulationPhase::WatershedKernel, &payload)
                .expect("indexed decision should construct");
        let mut state = BTreeMap::new();
        let mut flux = BTreeMap::new();
        let mut indexed = IndexedWritebackSurface::from_btreemap_surfaces(&registry, &state, &flux)
            .expect("empty surfaces should index");

        let result = apply_indexed_kernel_writeback(
            SimulationPhase::WatershedKernel,
            &decision,
            &payload,
            &mut indexed,
            &registry,
            &mut state,
            &mut flux,
        )
        .expect("accepted indexed writeback should apply");

        assert_eq!(result.outcome, WritebackDecisionOutcome::Apply);
        assert_eq!(result.status.message_id(), WRITEBACK_APPLY_MESSAGE_ID);
        assert_eq!(
            result.applied_state_symbols,
            vec![
                BoundarySymbol::from("state_a"),
                BoundarySymbol::from("state_z")
            ]
        );
        assert_eq!(
            result.applied_flux_symbols,
            vec![BoundarySymbol::from("flux_q")]
        );
        assert_eq!(
            indexed.state_value(lower_state_id),
            Some(BoundaryValue::from(1.0))
        );
        assert_eq!(
            indexed.state_value(upper_state_id),
            Some(BoundaryValue::from(9.0))
        );
        assert_eq!(
            indexed.flux_value(flux_q_id),
            Some(BoundaryValue::from(4.0))
        );
        assert_eq!(
            state[&BoundarySymbol::from("state_a")],
            BoundaryValue::from(1.0)
        );
        assert_eq!(
            state[&BoundarySymbol::from("state_z")],
            BoundaryValue::from(9.0)
        );
        assert_eq!(
            flux[&BoundarySymbol::from("flux_q")],
            BoundaryValue::from(4.0)
        );
    }

    #[test]
    fn indexed_writeback_resolves_all_ids_before_any_mutation() {
        let full_registry =
            SymbolRegistry::from_symbols(["known", "unknown"]).expect("full registry should build");
        let small_registry =
            SymbolRegistry::from_symbols(["known"]).expect("small registry should build");
        let known_id = full_registry
            .id_of(&BoundarySymbol::from("known"))
            .expect("known id should resolve");
        let unknown_id = full_registry
            .id_of(&BoundarySymbol::from("unknown"))
            .expect("unknown id should resolve in full registry");
        let payload = IndexedKernelWritebackPayload::with_updates(
            vec![IndexedWritebackField::unbounded(
                known_id,
                BoundaryValue::from(1.0),
            )],
            vec![IndexedWritebackField::unbounded(
                unknown_id,
                BoundaryValue::from(2.0),
            )],
        );
        let decision =
            evaluate_indexed_kernel_writeback(SimulationPhase::WatershedKernel, &payload)
                .expect("indexed decision should construct");
        let mut state = BTreeMap::new();
        let mut flux = BTreeMap::new();
        let mut indexed =
            IndexedWritebackSurface::from_btreemap_surfaces(&small_registry, &state, &flux)
                .expect("empty surfaces should index");

        let error = apply_indexed_kernel_writeback(
            SimulationPhase::WatershedKernel,
            &decision,
            &payload,
            &mut indexed,
            &small_registry,
            &mut state,
            &mut flux,
        )
        .expect_err("unknown id should reject before mutation");

        assert!(matches!(
            error,
            WritebackError::SymbolRegistry(SymbolRegistryError::UnknownSymbolId { id })
                if id == unknown_id
        ));
        assert!(state.is_empty());
        assert!(flux.is_empty());
        assert!(indexed.state_surface().is_empty());
        assert!(indexed.flux_surface().is_empty());
    }

    #[test]
    fn indexed_writeback_application_requires_accept_outcome() {
        let registry =
            SymbolRegistry::from_symbols(["known"]).expect("writeback symbol should register");
        let payload = IndexedKernelWritebackPayload::empty();
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
        let mut indexed = IndexedWritebackSurface::from_btreemap_surfaces(&registry, &state, &flux)
            .expect("empty surfaces should index");

        let error = apply_indexed_kernel_writeback(
            SimulationPhase::WatershedKernel,
            &reject_decision,
            &payload,
            &mut indexed,
            &registry,
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
    fn logical_writeback_evaluation_covers_every_domain_bound_shape() {
        let payload = KernelWritebackPayload::with_updates(
            vec![
                WritebackField::bounded("range", 3.0, Some(0.0), Some(2.0)),
                WritebackField::bounded("minimum", -1.0, Some(0.0), None),
            ],
            vec![
                WritebackField::bounded("maximum", 3.0, None, Some(2.0)),
                WritebackField::unbounded("valid", 1.0),
            ],
        );

        let decision = evaluate_kernel_writeback(SimulationPhase::HillslopeKernel, &payload)
            .expect("domain-reject decision should construct");

        assert_eq!(decision.outcome, WritebackDecisionOutcome::Reject);
        assert_eq!(
            decision.status.message_id(),
            WRITEBACK_REJECT_DOMAIN_MESSAGE_ID
        );
        assert_eq!(decision.violations.len(), 3);
        assert_eq!(
            decision
                .violations
                .iter()
                .map(|violation| (violation.check_id.as_str(), violation.kind))
                .collect::<Vec<_>>(),
            vec![
                ("INV-WRITEBACK-002", ClosureViolationKind::DomainRange),
                ("INV-WRITEBACK-003", ClosureViolationKind::DomainLowerBound,),
                ("INV-WRITEBACK-004", ClosureViolationKind::DomainUpperBound,),
            ]
        );
    }

    #[test]
    fn logical_writeback_application_sorts_and_updates_both_surfaces() {
        let payload = KernelWritebackPayload::with_updates(
            vec![
                WritebackField::unbounded("state_z", 9.0),
                WritebackField::unbounded("state_a", 1.0),
            ],
            vec![
                WritebackField::unbounded("flux_z", 8.0),
                WritebackField::unbounded("flux_a", 4.0),
            ],
        );
        let decision = evaluate_kernel_writeback(SimulationPhase::WatershedKernel, &payload)
            .expect("accepted decision should construct");
        let mut state = BTreeMap::new();
        let mut flux = BTreeMap::new();

        let result = apply_kernel_writeback(
            SimulationPhase::WatershedKernel,
            &decision,
            &payload,
            &mut state,
            &mut flux,
        )
        .expect("accepted writeback should apply");

        assert_eq!(result.outcome, WritebackDecisionOutcome::Apply);
        assert_eq!(result.status.message_id(), WRITEBACK_APPLY_MESSAGE_ID);
        assert_eq!(
            result.applied_state_symbols,
            vec![
                BoundarySymbol::from("state_a"),
                BoundarySymbol::from("state_z")
            ]
        );
        assert_eq!(
            result.applied_flux_symbols,
            vec![
                BoundarySymbol::from("flux_a"),
                BoundarySymbol::from("flux_z")
            ]
        );
        assert_eq!(
            state[&BoundarySymbol::from("state_a")],
            BoundaryValue::from(1.0)
        );
        assert_eq!(
            state[&BoundarySymbol::from("state_z")],
            BoundaryValue::from(9.0)
        );
        assert_eq!(
            flux[&BoundarySymbol::from("flux_a")],
            BoundaryValue::from(4.0)
        );
        assert_eq!(
            flux[&BoundarySymbol::from("flux_z")],
            BoundaryValue::from(8.0)
        );
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
    fn symbol_registry_assigns_ids_in_sorted_symbol_order() {
        let registry = SymbolRegistry::from_symbols(["zeta", "alpha", "beta", "alpha"])
            .expect("registry should build from duplicate symbols");

        let assigned = registry
            .iter()
            .map(|(id, symbol)| (id.as_u32(), symbol.as_str().to_owned()))
            .collect::<Vec<_>>();

        assert_eq!(
            assigned,
            vec![
                (0, "alpha".to_owned()),
                (1, "beta".to_owned()),
                (2, "zeta".to_owned()),
            ]
        );
        assert_eq!(
            registry
                .id_of(&BoundarySymbol::from("beta"))
                .expect("beta should be registered")
                .as_u32(),
            1
        );
    }

    #[test]
    fn symbol_registry_export_surface_matches_btreemap_order_after_sort() {
        let mut surface = BTreeMap::new();
        surface.insert(BoundarySymbol::from("q"), BoundaryValue::from(4.0));
        surface.insert(BoundarySymbol::from("alpha"), BoundaryValue::from(2.0));
        surface.insert(BoundarySymbol::from("storage"), BoundaryValue::from(6.0));
        let registry =
            SymbolRegistry::from_symbols(["storage", "q", "alpha"]).expect("registry should build");

        let exported = registry
            .export_surface_in_id_order(&surface)
            .expect("registered surface should export");
        let exported_symbols = exported
            .iter()
            .map(|(_, symbol, value)| (symbol.as_str().to_owned(), value.as_f64()))
            .collect::<Vec<_>>();
        let btree_symbols = surface
            .iter()
            .map(|(symbol, value)| (symbol.as_str().to_owned(), value.as_f64()))
            .collect::<Vec<_>>();

        assert_eq!(exported_symbols, btree_symbols);
    }

    #[test]
    fn indexed_surface_round_trips_in_sorted_symbol_order() {
        let mut surface = BTreeMap::new();
        surface.insert(BoundarySymbol::from("q"), BoundaryValue::from(4.0));
        surface.insert(BoundarySymbol::from("alpha"), BoundaryValue::from(2.0));
        surface.insert(BoundarySymbol::from("storage"), BoundaryValue::from(6.0));
        let registry =
            SymbolRegistry::from_symbols(["storage", "q", "alpha"]).expect("registry should build");

        let indexed =
            IndexedSurface::from_btreemap(&registry, &surface).expect("surface should index");
        let exported = indexed
            .export_btreemap(&registry)
            .expect("surface should export");

        assert_eq!(exported, surface);
        let exported_symbols = indexed
            .entries()
            .iter()
            .map(|(id, value)| {
                (
                    registry
                        .symbol(*id)
                        .expect("id should resolve")
                        .as_str()
                        .to_owned(),
                    value.as_f64(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            exported_symbols,
            vec![
                ("alpha".to_owned(), 2.0),
                ("q".to_owned(), 4.0),
                ("storage".to_owned(), 6.0),
            ]
        );

        let q_id = registry
            .id_of(&BoundarySymbol::from("q"))
            .expect("q should be registered");
        assert_eq!(indexed.get(q_id), Some(BoundaryValue::from(4.0)));
    }

    #[test]
    fn indexed_surface_rejects_unknown_btreemap_symbol() {
        let mut surface = BTreeMap::new();
        surface.insert(BoundarySymbol::from("known"), BoundaryValue::from(1.0));
        surface.insert(BoundarySymbol::from("unknown"), BoundaryValue::from(2.0));
        let registry = SymbolRegistry::from_symbols(["known"]).expect("registry should build");

        let error = IndexedSurface::from_btreemap(&registry, &surface)
            .expect_err("unknown symbol should fail");

        assert!(matches!(
            error,
            SymbolRegistryError::UnknownSymbol { symbol } if symbol.as_str() == "unknown"
        ));
    }

    #[test]
    fn indexed_writeback_surface_round_trips_state_and_flux() {
        let mut state_surface = BTreeMap::new();
        state_surface.insert(BoundarySymbol::from("s"), BoundaryValue::from(1.0));
        state_surface.insert(BoundarySymbol::from("q"), BoundaryValue::from(2.0));
        let mut flux_surface = BTreeMap::new();
        flux_surface.insert(BoundarySymbol::from("ET"), BoundaryValue::from(3.0));
        let registry =
            SymbolRegistry::from_symbols(["s", "q", "ET"]).expect("registry should build");

        let indexed = IndexedWritebackSurface::from_btreemap_surfaces(
            &registry,
            &state_surface,
            &flux_surface,
        )
        .expect("writeback surface should index");
        let (exported_state, exported_flux) = indexed
            .export_btreemap_surfaces(&registry)
            .expect("writeback surface should export");

        assert_eq!(exported_state, state_surface);
        assert_eq!(exported_flux, flux_surface);
    }

    #[test]
    fn symbol_registry_audit_records_post_freeze_unknowns() {
        let registry = SymbolRegistry::from_symbols(["known"]).expect("registry should build");
        begin_symbol_registry_audit(registry).expect("audit should begin");

        let _known = BoundarySymbol::from("known");
        let _unknown = BoundarySymbol::from("late_unknown");

        let report = finish_symbol_registry_audit().expect("audit report should exist");
        assert_eq!(report.registry_symbol_count(), 1);
        assert_eq!(report.constructed_symbol_count(), 2);
        assert_eq!(
            report
                .unknown_symbols()
                .iter()
                .map(BoundarySymbol::as_str)
                .collect::<Vec<_>>(),
            vec!["late_unknown"]
        );
        assert!(!report.is_complete());
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
    fn indexed_request_without_dense_slots_keeps_dense_surface_absent() {
        let mut state_surface = BTreeMap::new();
        let mut flux_surface = BTreeMap::new();
        let state_symbol = BoundarySymbol::from("perfdeep07_hot_state");
        let flux_symbol = BoundarySymbol::from("perfdeep07_hot_flux");
        state_surface.insert(state_symbol.clone(), BoundaryValue::scalar(4.25));
        flux_surface.insert(flux_symbol.clone(), BoundaryValue::scalar(1.5));
        let registry = SymbolRegistry::from_symbols([state_symbol.clone(), flux_symbol])
            .expect("registry should build");
        let indexed_surface = IndexedWritebackSurface::from_btreemap_surfaces(
            &registry,
            &state_surface,
            &flux_surface,
        )
        .expect("indexed surface should build");
        let state_id = registry
            .id_of(&state_symbol)
            .expect("state symbol should be registered");
        let indexed_state_symbol = IndexedBoundarySymbol::new(state_symbol, state_id);
        let request = HillslopeKernelRequest::with_transition_context_and_indexed(
            "perfdeep07_default_indexed",
            HillslopeKernelPhaseClass::Hydrology,
            HillslopeConsumerAdapter::Runoff,
            None,
            None,
            &state_surface,
            &flux_surface,
            Some(&indexed_surface),
            None,
        );

        assert!(request.has_indexed_state_surface());
        assert!(!request.has_dense_state_surface());
        assert_eq!(
            request.indexed_state_value(&indexed_state_symbol),
            Some(BoundaryValue::scalar(4.25))
        );
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

    #[test]
    fn hot_symbol_tables_scan_series_and_grid_symbols_once() {
        let registry = SymbolRegistry::from_symbols([
            "timem_0001",
            "timem_0002",
            "intsty_0001",
            "frost.runtime_fgfrst_0002_0003",
            "mofe_hourly_carry_arrays_enabled",
        ])
        .expect("registry should build");

        let tables = HotSymbolTables::from_registry(
            &registry,
            &["mofe_hourly_carry_arrays_enabled"],
            &[],
            &["timem", "intsty"],
            &[],
            &["frost.runtime_fgfrst"],
            &[],
        );

        assert_eq!(
            tables
                .state_scalar("mofe_hourly_carry_arrays_enabled")
                .expect("scalar id should resolve")
                .symbol
                .as_str(),
            "mofe_hourly_carry_arrays_enabled"
        );
        assert_eq!(
            tables
                .state_series_symbol("timem", 2)
                .expect("series id should resolve")
                .symbol
                .as_str(),
            "timem_0002"
        );
        assert_eq!(
            tables
                .state_grid_symbol("frost.runtime_fgfrst", 2, 3)
                .expect("grid id should resolve")
                .symbol
                .as_str(),
            "frost.runtime_fgfrst_0002_0003"
        );
        assert!(tables.state_series_symbol("timem", 3).is_none());
    }

    #[test]
    fn hot_symbol_tables_scan_pl_dispatch_symbols_once() {
        let registry = SymbolRegistry::from_symbols([
            "pl_schedule_slot_0002_ofe_index",
            "pl_schedule_slot_0002_crop_0003_imngmt",
            "pl_growth_slot_0002_crop_0003_jdplt",
            "pl_decomp_slot_0002_crop_0003_resmgt",
            "pl_decomp_slot_0002_crop_0003_gday_0001",
        ])
        .expect("registry should build");

        let tables = HotSymbolTables::from_registry(&registry, &[], &[], &[], &[], &[], &[]);

        assert_eq!(
            tables
                .pl_schedule_slot_state_symbol("ofe_index", 2)
                .expect("schedule slot symbol should resolve")
                .symbol
                .as_str(),
            "pl_schedule_slot_0002_ofe_index"
        );
        assert_eq!(
            tables
                .pl_schedule_slot_crop_state_symbol("imngmt", 2, 3)
                .expect("schedule crop symbol should resolve")
                .symbol
                .as_str(),
            "pl_schedule_slot_0002_crop_0003_imngmt"
        );
        assert_eq!(
            tables
                .pl_growth_slot_crop_state_symbol("jdplt", 2, 3)
                .expect("growth crop symbol should resolve")
                .symbol
                .as_str(),
            "pl_growth_slot_0002_crop_0003_jdplt"
        );
        assert_eq!(
            tables
                .pl_decomp_slot_crop_state_symbol("resmgt", 2, 3)
                .expect("decomp crop symbol should resolve")
                .symbol
                .as_str(),
            "pl_decomp_slot_0002_crop_0003_resmgt"
        );
        assert!(
            tables
                .pl_decomp_slot_crop_state_symbol("gday", 2, 3)
                .is_none()
        );
        assert_eq!(
            tables
                .pl_decomp_slot_crop_indexed_state_symbol("gday", 2, 3, 1)
                .expect("indexed decomp crop symbol should resolve")
                .symbol
                .as_str(),
            "pl_decomp_slot_0002_crop_0003_gday_0001"
        );
    }

    #[test]
    fn indexed_surface_set_updates_in_id_order() {
        let registry =
            SymbolRegistry::from_symbols(["a", "b", "c"]).expect("registry should build");
        let mut surface = IndexedSurface::from_btreemap(
            &registry,
            &BTreeMap::from([
                (BoundarySymbol::from("a"), BoundaryValue::scalar(1.0)),
                (BoundarySymbol::from("c"), BoundaryValue::scalar(3.0)),
            ]),
        )
        .expect("indexed surface should build");

        let b_id = registry
            .id_of(&BoundarySymbol::from("b"))
            .expect("b id should resolve");
        let a_id = registry
            .id_of(&BoundarySymbol::from("a"))
            .expect("a id should resolve");

        surface.set(b_id, Some(BoundaryValue::scalar(2.0)));
        surface.set(a_id, None);

        let exported = surface
            .export_btreemap(&registry)
            .expect("export should succeed");
        assert_eq!(exported.len(), 2);
        let b_value = exported
            .get(&BoundarySymbol::from("b"))
            .expect("b should be present")
            .as_f64();
        assert!((b_value - 2.0).abs() <= f64::EPSILON);
        assert!(!exported.contains_key(&BoundarySymbol::from("a")));
    }
}
