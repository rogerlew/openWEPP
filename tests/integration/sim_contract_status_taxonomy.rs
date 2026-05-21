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
