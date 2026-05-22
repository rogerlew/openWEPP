#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]

//! Kernel invocation and writeback contract boundaries for openWEPP.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use openwepp_sim_contract::closure::{
    ClosureViolation, ClosureViolationKind, check_finite, check_max, check_min, check_range,
};
use openwepp_sim_contract::status::{
    BoundaryClass, SimulationPhase, SimulationStatus, StatusError,
};
use openwepp_unit_boundary::{
    FlowRateCubicMetersPerSecond, ProcessRateMillimetersPerHour, RunoffDepthMillimeters,
    StorageVolumeCubicMeters, SurfaceAreaSquareMeters,
};

/// Message id emitted when writeback payload evaluation accepts all fields.
pub const WRITEBACK_ACCEPT_MESSAGE_ID: &str = "KWRITEBACK-ACCEPT-001";
/// Message id emitted when accepted writeback is applied by orchestrator.
pub const WRITEBACK_APPLY_MESSAGE_ID: &str = "KWRITEBACK-APPLY-001";
/// Message id emitted when writeback is rejected for non-finite values.
pub const WRITEBACK_REJECT_NON_FINITE_MESSAGE_ID: &str = "KWRITEBACK-E-NON-FINITE";
/// Message id emitted when writeback is rejected for domain/range violations.
pub const WRITEBACK_REJECT_DOMAIN_MESSAGE_ID: &str = "KWRITEBACK-E-DOMAIN-VIOLATION";

/// Type-safe state/flux symbol key for kernel seam maps.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BoundarySymbol(String);

impl BoundarySymbol {
    #[must_use]
    pub fn new(symbol: impl Into<String>) -> Self {
        Self(symbol.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for BoundarySymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for BoundarySymbol {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for BoundarySymbol {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

/// Unit-aware scalar value for kernel seam maps.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoundaryValue {
    Scalar(f64),
    RunoffDepthMillimeters(RunoffDepthMillimeters),
    FlowRateCubicMetersPerSecond(FlowRateCubicMetersPerSecond),
    StorageVolumeCubicMeters(StorageVolumeCubicMeters),
    ProcessRateMillimetersPerHour(ProcessRateMillimetersPerHour),
    SurfaceAreaSquareMeters(SurfaceAreaSquareMeters),
}

impl BoundaryValue {
    #[must_use]
    pub const fn scalar(value: f64) -> Self {
        Self::Scalar(value)
    }

    #[must_use]
    pub fn as_f64(self) -> f64 {
        match self {
            Self::Scalar(value) => value,
            Self::RunoffDepthMillimeters(value) => value.as_millimeters(),
            Self::FlowRateCubicMetersPerSecond(value) => value.as_cubic_meters_per_second(),
            Self::StorageVolumeCubicMeters(value) => value.as_cubic_meters(),
            Self::ProcessRateMillimetersPerHour(value) => value.as_millimeters_per_hour(),
            Self::SurfaceAreaSquareMeters(value) => value.as_square_meters(),
        }
    }

    #[must_use]
    pub const fn unit_label(self) -> &'static str {
        match self {
            Self::Scalar(_) => "scalar",
            Self::RunoffDepthMillimeters(_) => "mm",
            Self::FlowRateCubicMetersPerSecond(_) => "m3/s",
            Self::StorageVolumeCubicMeters(_) => "m3",
            Self::ProcessRateMillimetersPerHour(_) => "mm/hr",
            Self::SurfaceAreaSquareMeters(_) => "m2",
        }
    }
}

impl From<f64> for BoundaryValue {
    fn from(value: f64) -> Self {
        Self::Scalar(value)
    }
}

impl From<RunoffDepthMillimeters> for BoundaryValue {
    fn from(value: RunoffDepthMillimeters) -> Self {
        Self::RunoffDepthMillimeters(value)
    }
}

impl From<FlowRateCubicMetersPerSecond> for BoundaryValue {
    fn from(value: FlowRateCubicMetersPerSecond) -> Self {
        Self::FlowRateCubicMetersPerSecond(value)
    }
}

impl From<StorageVolumeCubicMeters> for BoundaryValue {
    fn from(value: StorageVolumeCubicMeters) -> Self {
        Self::StorageVolumeCubicMeters(value)
    }
}

impl From<ProcessRateMillimetersPerHour> for BoundaryValue {
    fn from(value: ProcessRateMillimetersPerHour) -> Self {
        Self::ProcessRateMillimetersPerHour(value)
    }
}

impl From<SurfaceAreaSquareMeters> for BoundaryValue {
    fn from(value: SurfaceAreaSquareMeters) -> Self {
        Self::SurfaceAreaSquareMeters(value)
    }
}

/// Outcome class for orchestrator writeback decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WritebackDecisionOutcome {
    Accept,
    Reject,
    Apply,
}

/// One scalar writeback field proposed by a kernel.
#[derive(Debug, Clone, PartialEq)]
pub struct WritebackField {
    pub symbol: BoundarySymbol,
    pub value: BoundaryValue,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
}

impl WritebackField {
    #[must_use]
    pub fn unbounded(symbol: impl Into<BoundarySymbol>, value: impl Into<BoundaryValue>) -> Self {
        Self {
            symbol: symbol.into(),
            value: value.into(),
            minimum: None,
            maximum: None,
        }
    }

    #[must_use]
    pub fn bounded(
        symbol: impl Into<BoundarySymbol>,
        value: impl Into<BoundaryValue>,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            value: value.into(),
            minimum,
            maximum,
        }
    }
}

/// Kernel-proposed writeback payload.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct KernelWritebackPayload {
    pub state_updates: Vec<WritebackField>,
    pub flux_updates: Vec<WritebackField>,
}

impl KernelWritebackPayload {
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_updates(
        state_updates: Vec<WritebackField>,
        flux_updates: Vec<WritebackField>,
    ) -> Self {
        Self {
            state_updates,
            flux_updates,
        }
    }
}

/// Kernel response surface for hillslope and watershed invocations.
#[derive(Debug, Clone, PartialEq)]
pub struct KernelRunResponse {
    pub status: SimulationStatus,
    pub writeback: KernelWritebackPayload,
}

impl KernelRunResponse {
    #[must_use]
    pub const fn new(status: SimulationStatus, writeback: KernelWritebackPayload) -> Self {
        Self { status, writeback }
    }
}

/// Hillslope kernel invocation request.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct HillslopeKernelRequest {
    pub phase_name: String,
    pub state_surface: BTreeMap<BoundarySymbol, BoundaryValue>,
    pub flux_surface: BTreeMap<BoundarySymbol, BoundaryValue>,
}

impl HillslopeKernelRequest {
    #[must_use]
    pub fn new(
        phase_name: impl Into<String>,
        state_surface: BTreeMap<BoundarySymbol, BoundaryValue>,
        flux_surface: BTreeMap<BoundarySymbol, BoundaryValue>,
    ) -> Self {
        Self {
            phase_name: phase_name.into(),
            state_surface,
            flux_surface,
        }
    }
}

/// Watershed kernel invocation request.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct WatershedKernelRequest {
    pub node_kind: String,
    pub node_id: u32,
    pub dependency_nodes: Vec<String>,
    pub contributor_hillslopes: Vec<u32>,
    pub state_surface: BTreeMap<BoundarySymbol, BoundaryValue>,
    pub flux_surface: BTreeMap<BoundarySymbol, BoundaryValue>,
}

impl WatershedKernelRequest {
    #[must_use]
    pub fn new(
        node_kind: impl Into<String>,
        node_id: u32,
        dependency_nodes: Vec<String>,
        contributor_hillslopes: Vec<u32>,
        state_surface: BTreeMap<BoundarySymbol, BoundaryValue>,
        flux_surface: BTreeMap<BoundarySymbol, BoundaryValue>,
    ) -> Self {
        Self {
            node_kind: node_kind.into(),
            node_id,
            dependency_nodes,
            contributor_hillslopes,
            state_surface,
            flux_surface,
        }
    }
}

/// Hillslope kernel trait boundary.
pub trait HillslopeKernel {
    fn run_hillslope_phase(&mut self, request: &HillslopeKernelRequest) -> KernelRunResponse;
}

/// Watershed kernel trait boundary.
pub trait WatershedKernel {
    fn run_watershed_node(&mut self, request: &WatershedKernelRequest) -> KernelRunResponse;
}

/// Outcome surface for writeback evaluation.
#[derive(Debug, Clone, PartialEq)]
pub struct KernelWritebackDecision {
    pub outcome: WritebackDecisionOutcome,
    pub status: SimulationStatus,
    pub violations: Vec<ClosureViolation>,
}

/// Outcome surface for accepted writeback application.
#[derive(Debug, Clone, PartialEq)]
pub struct KernelWritebackApplyResult {
    pub outcome: WritebackDecisionOutcome,
    pub status: SimulationStatus,
    pub applied_state_symbols: Vec<BoundarySymbol>,
    pub applied_flux_symbols: Vec<BoundarySymbol>,
}

/// Writeback-application errors.
#[derive(Debug)]
pub enum WritebackError {
    Status(StatusError),
    DecisionNotAccept { outcome: WritebackDecisionOutcome },
}

impl fmt::Display for WritebackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Status(source) => write!(f, "failed constructing writeback status: {source}"),
            Self::DecisionNotAccept { outcome } => {
                write!(
                    f,
                    "cannot apply writeback for non-accept outcome: {outcome:?}"
                )
            }
        }
    }
}

impl Error for WritebackError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Status(source) => Some(source),
            Self::DecisionNotAccept { .. } => None,
        }
    }
}

impl From<StatusError> for WritebackError {
    fn from(value: StatusError) -> Self {
        Self::Status(value)
    }
}

/// Evaluate a kernel writeback payload into deterministic accept/reject status.
pub fn evaluate_kernel_writeback(
    phase: SimulationPhase,
    payload: &KernelWritebackPayload,
) -> Result<KernelWritebackDecision, StatusError> {
    let mut violations = Vec::new();

    for field in &payload.state_updates {
        collect_field_violations("state", field, &mut violations);
    }

    for field in &payload.flux_updates {
        collect_field_violations("flux", field, &mut violations);
    }

    if violations.is_empty() {
        let status = SimulationStatus::ok(phase, WRITEBACK_ACCEPT_MESSAGE_ID)?;
        Ok(KernelWritebackDecision {
            outcome: WritebackDecisionOutcome::Accept,
            status,
            violations,
        })
    } else {
        let has_non_finite = violations
            .iter()
            .any(|violation| violation.kind == ClosureViolationKind::NonFinite);

        let status = if has_non_finite {
            SimulationStatus::non_finite_failure(phase, WRITEBACK_REJECT_NON_FINITE_MESSAGE_ID)?
        } else {
            SimulationStatus::domain_failure(
                phase,
                BoundaryClass::DomainViolation,
                WRITEBACK_REJECT_DOMAIN_MESSAGE_ID,
            )?
        };

        Ok(KernelWritebackDecision {
            outcome: WritebackDecisionOutcome::Reject,
            status,
            violations,
        })
    }
}

/// Apply an accepted writeback payload to orchestrator-owned state/flux maps.
pub fn apply_kernel_writeback(
    phase: SimulationPhase,
    decision: &KernelWritebackDecision,
    payload: &KernelWritebackPayload,
    state_surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    flux_surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
) -> Result<KernelWritebackApplyResult, WritebackError> {
    if decision.outcome != WritebackDecisionOutcome::Accept {
        return Err(WritebackError::DecisionNotAccept {
            outcome: decision.outcome,
        });
    }

    let mut state_updates: Vec<&WritebackField> = payload.state_updates.iter().collect();
    state_updates.sort_by_key(|field| field.symbol.as_str());

    let mut flux_updates: Vec<&WritebackField> = payload.flux_updates.iter().collect();
    flux_updates.sort_by_key(|field| field.symbol.as_str());

    for field in &state_updates {
        state_surface.insert(field.symbol.clone(), field.value);
    }

    for field in &flux_updates {
        flux_surface.insert(field.symbol.clone(), field.value);
    }

    let status = SimulationStatus::ok(phase, WRITEBACK_APPLY_MESSAGE_ID)?;

    Ok(KernelWritebackApplyResult {
        outcome: WritebackDecisionOutcome::Apply,
        status,
        applied_state_symbols: state_updates
            .iter()
            .map(|field| field.symbol.clone())
            .collect(),
        applied_flux_symbols: flux_updates
            .iter()
            .map(|field| field.symbol.clone())
            .collect(),
    })
}

fn collect_field_violations(
    scope: &str,
    field: &WritebackField,
    output: &mut Vec<ClosureViolation>,
) {
    let subject = format!("{scope}:{}[{}]", field.symbol, field.value.unit_label());
    let value = field.value.as_f64();

    collect_check(
        check_finite(
            "INV-WRITEBACK-001",
            WRITEBACK_REJECT_NON_FINITE_MESSAGE_ID,
            subject.as_str(),
            value,
        ),
        output,
    );

    match (field.minimum, field.maximum) {
        (Some(minimum), Some(maximum)) => collect_check(
            check_range(
                "INV-WRITEBACK-002",
                WRITEBACK_REJECT_DOMAIN_MESSAGE_ID,
                subject.as_str(),
                value,
                minimum,
                maximum,
            ),
            output,
        ),
        (Some(minimum), None) => collect_check(
            check_min(
                "INV-WRITEBACK-003",
                WRITEBACK_REJECT_DOMAIN_MESSAGE_ID,
                subject.as_str(),
                value,
                minimum,
            ),
            output,
        ),
        (None, Some(maximum)) => collect_check(
            check_max(
                "INV-WRITEBACK-004",
                WRITEBACK_REJECT_DOMAIN_MESSAGE_ID,
                subject.as_str(),
                value,
                maximum,
            ),
            output,
        ),
        (None, None) => {}
    }
}

fn collect_check(result: Result<(), Box<ClosureViolation>>, output: &mut Vec<ClosureViolation>) {
    if let Err(violation) = result {
        output.push(*violation);
    }
}

#[cfg(test)]
mod tests {
    use openwepp_sim_contract::status::{SimulationPhase, StatusClassification};

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
}
