use openwepp_sim_contract::status::{
    BoundaryClass, ClampClass, SimulationPhase, SimulationStatus, StatusClassification,
    StatusError, StatusSeverity,
};

#[test]
fn nominal_status_is_classified_as_ok() {
    let status = SimulationStatus::ok(SimulationPhase::HillslopeKernel, "WB-OK-001")
        .expect("nominal status should construct");

    assert!(status.ok_flag());
    assert!(status.finite_ok());
    assert!(status.domain_ok());
    assert_eq!(status.boundary_class(), BoundaryClass::Ok);
    assert_eq!(status.clamp_class(), ClampClass::None);
    assert_eq!(status.classification(), StatusClassification::Nominal);
    assert_eq!(status.severity(), StatusSeverity::Ok);
}

#[test]
fn clamp_status_is_classified_as_advisory() {
    let status = SimulationStatus::advisory(
        SimulationPhase::HillslopeKernel,
        BoundaryClass::Ok,
        ClampClass::QcapSoftLimit,
        "WB-W-CLAMP-001",
    )
    .expect("advisory clamp status should construct");

    assert_eq!(status.classification(), StatusClassification::Advisory);
    assert_eq!(status.severity(), StatusSeverity::Warning);
    assert_eq!(status.clamp_class(), ClampClass::QcapSoftLimit);
}

#[test]
fn non_finite_status_is_classified_as_failure() {
    let status = SimulationStatus::non_finite_failure(
        SimulationPhase::WatershedKernel,
        "ROUTE-E-NON-FINITE-001",
    )
    .expect("non-finite failure should construct");

    assert!(!status.ok_flag());
    assert!(!status.finite_ok());
    assert!(!status.domain_ok());
    assert_eq!(status.boundary_class(), BoundaryClass::NonFinite);
    assert_eq!(status.classification(), StatusClassification::Failure);
    assert_eq!(status.severity(), StatusSeverity::Error);
}

#[test]
fn empty_message_id_is_rejected() {
    let error = SimulationStatus::ok(SimulationPhase::PreExecutionValidation, "   ")
        .expect_err("empty message ids must fail");

    assert_eq!(error, StatusError::MessageIdEmpty);
}

#[test]
fn advisory_constructor_rejects_failure_boundary_class() {
    let error = SimulationStatus::advisory(
        SimulationPhase::WatershedKernel,
        BoundaryClass::ClosureViolation,
        ClampClass::None,
        "ROUTE-W-INVALID-001",
    )
    .expect_err("advisory constructor should reject failure boundary classes");

    assert_eq!(
        error,
        StatusError::AdvisoryBoundaryMustNotBeFailure {
            boundary_class: BoundaryClass::ClosureViolation,
        }
    );
}

#[test]
fn boundary_advisory_class_without_clamp_is_warning() {
    let status = SimulationStatus::advisory(
        SimulationPhase::HillslopeKernel,
        BoundaryClass::Dry,
        ClampClass::None,
        "WB-W-DRY-001",
    )
    .expect("dry advisory should construct");

    assert_eq!(status.classification(), StatusClassification::Advisory);
    assert_eq!(status.severity(), StatusSeverity::Warning);
}

#[test]
fn simulation_phase_strings_are_exhaustive_and_stable() {
    let phases = [
        (
            SimulationPhase::PreExecutionValidation,
            "pre_execution_validation",
        ),
        (SimulationPhase::HillslopeKernel, "hillslope_kernel"),
        (SimulationPhase::WatershedKernel, "watershed_kernel"),
        (SimulationPhase::SummaryAccumulator, "summary_accumulator"),
        (
            SimulationPhase::CompatibilityAdapter,
            "compatibility_adapter",
        ),
    ];
    for (phase, expected) in phases {
        assert_eq!(phase.as_str(), expected);
    }
}

#[test]
fn boundary_class_strings_mappings_and_advisory_construction_are_exhaustive() {
    let boundary_classes = [
        (BoundaryClass::Ok, "OK", StatusClassification::Nominal),
        (BoundaryClass::Dry, "DRY", StatusClassification::Advisory),
        (
            BoundaryClass::Saturated,
            "SATURATED",
            StatusClassification::Advisory,
        ),
        (
            BoundaryClass::NegativeInput,
            "NEGATIVE_INPUT",
            StatusClassification::Failure,
        ),
        (
            BoundaryClass::ZeroGeometry,
            "ZERO_GEOMETRY",
            StatusClassification::Failure,
        ),
        (
            BoundaryClass::ModeMismatch,
            "MODE_MISMATCH",
            StatusClassification::Failure,
        ),
        (
            BoundaryClass::CapBinding,
            "CAP_BINDING",
            StatusClassification::Advisory,
        ),
        (
            BoundaryClass::TopologyInvalid,
            "TOPOLOGY_INVALID",
            StatusClassification::Failure,
        ),
        (
            BoundaryClass::ClosureViolation,
            "CLOSURE_VIOLATION",
            StatusClassification::Failure,
        ),
        (
            BoundaryClass::DomainViolation,
            "DOMAIN_VIOLATION",
            StatusClassification::Failure,
        ),
        (
            BoundaryClass::NonFinite,
            "NON_FINITE",
            StatusClassification::Failure,
        ),
        (
            BoundaryClass::MissingRequiredInput,
            "MISSING_REQUIRED_INPUT",
            StatusClassification::Failure,
        ),
    ];
    for (boundary_class, expected_string, expected_classification) in boundary_classes {
        assert_eq!(boundary_class.as_str(), expected_string);
        assert_eq!(boundary_class.classification(), expected_classification);

        let advisory = SimulationStatus::advisory(
            SimulationPhase::HillslopeKernel,
            boundary_class,
            ClampClass::None,
            "STATUS-TAXONOMY-001",
        );
        if expected_classification == StatusClassification::Failure {
            assert_eq!(
                advisory,
                Err(StatusError::AdvisoryBoundaryMustNotBeFailure { boundary_class })
            );
        } else {
            let status =
                advisory.expect("nominal and advisory classes must construct advisory status");
            assert_eq!(status.boundary_class(), boundary_class);
            assert_eq!(status.classification(), expected_classification);
        }
    }
}

#[test]
fn clamp_strings_and_classification_severity_mappings_are_exhaustive() {
    let clamp_classes = [
        (ClampClass::None, "NONE", false),
        (ClampClass::LowerBoundClamp, "LOWER_BOUND_CLAMP", true),
        (ClampClass::UpperBoundClamp, "UPPER_BOUND_CLAMP", true),
        (ClampClass::QcapSoftLimit, "QCAP_SOFT_LIMIT", true),
        (ClampClass::ProfileShortfall, "PROFILE_SHORTFALL", true),
    ];
    for (clamp_class, expected_string, expected_is_clamped) in clamp_classes {
        assert_eq!(clamp_class.as_str(), expected_string);
        assert_eq!(clamp_class.is_clamped(), expected_is_clamped);
    }

    let severities = [
        (StatusClassification::Nominal, StatusSeverity::Ok),
        (StatusClassification::Advisory, StatusSeverity::Warning),
        (StatusClassification::Failure, StatusSeverity::Error),
    ];
    for (classification, expected_severity) in severities {
        assert_eq!(classification.severity(), expected_severity);
    }
}

#[test]
fn status_construction_and_classification_precedence_are_stable() {
    let explicit_failure = SimulationStatus::new(
        SimulationPhase::SummaryAccumulator,
        true,
        true,
        true,
        BoundaryClass::TopologyInvalid,
        ClampClass::ProfileShortfall,
        "SUMMARY-E-TOPOLOGY-001",
    )
    .expect("nonempty explicit status should construct");
    assert_eq!(
        explicit_failure.phase(),
        SimulationPhase::SummaryAccumulator
    );
    assert!(explicit_failure.ok_flag());
    assert!(explicit_failure.finite_ok());
    assert!(explicit_failure.domain_ok());
    assert_eq!(
        explicit_failure.boundary_class(),
        BoundaryClass::TopologyInvalid
    );
    assert_eq!(explicit_failure.clamp_class(), ClampClass::ProfileShortfall);
    assert_eq!(explicit_failure.message_id(), "SUMMARY-E-TOPOLOGY-001");
    assert_eq!(
        explicit_failure.classification(),
        StatusClassification::Failure
    );
    assert_eq!(explicit_failure.severity(), StatusSeverity::Error);

    let status_flags = [
        (false, true, true, "ok"),
        (true, false, true, "finite"),
        (true, true, false, "domain"),
    ];
    for (ok, finite_ok, domain_ok, field) in status_flags {
        let status = SimulationStatus::new(
            SimulationPhase::CompatibilityAdapter,
            ok,
            finite_ok,
            domain_ok,
            BoundaryClass::Ok,
            ClampClass::None,
            "COMPAT-E-STATUS-001",
        )
        .expect("nonempty explicit status should construct");
        assert_eq!(
            status.classification(),
            StatusClassification::Failure,
            "{field}"
        );
        assert_eq!(status.severity(), StatusSeverity::Error, "{field}");
    }

    let failure = SimulationStatus::failure(
        SimulationPhase::WatershedKernel,
        true,
        false,
        BoundaryClass::DomainViolation,
        "ROUTE-E-DOMAIN-001",
    )
    .expect("failure status should construct");
    assert!(!failure.ok_flag());
    assert!(failure.finite_ok());
    assert!(!failure.domain_ok());
    assert_eq!(failure.clamp_class(), ClampClass::None);
    assert_eq!(failure.classification(), StatusClassification::Failure);

    let domain_failure = SimulationStatus::domain_failure(
        SimulationPhase::PreExecutionValidation,
        BoundaryClass::NegativeInput,
        "INPUT-E-NEGATIVE-001",
    )
    .expect("domain failure should construct");
    assert_eq!(
        domain_failure.phase(),
        SimulationPhase::PreExecutionValidation
    );
    assert!(!domain_failure.ok_flag());
    assert!(domain_failure.finite_ok());
    assert!(!domain_failure.domain_ok());
    assert_eq!(
        domain_failure.boundary_class(),
        BoundaryClass::NegativeInput
    );
    assert_eq!(domain_failure.message_id(), "INPUT-E-NEGATIVE-001");
    assert_eq!(
        domain_failure.classification(),
        StatusClassification::Failure
    );
}

#[test]
fn status_error_display_strings_are_stable() {
    assert_eq!(
        StatusError::MessageIdEmpty.to_string(),
        "message_id must not be empty"
    );
    assert_eq!(
        StatusError::AdvisoryBoundaryMustNotBeFailure {
            boundary_class: BoundaryClass::MissingRequiredInput,
        }
        .to_string(),
        "advisory status cannot use failure boundary class MISSING_REQUIRED_INPUT"
    );
}
