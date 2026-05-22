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

/// Maximum supported climate forcing series points for runtime symbol
/// projection.
pub const MAX_CLIMATE_FORCING_SERIES_POINTS: usize = 1_500;

/// Typed climate forcing symbol projection surface for `timem_*` and
/// `intsty_*` boundary aliases.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClimateForcingSymbolSurface {
    timem_symbols: Vec<BoundarySymbol>,
    intsty_symbols: Vec<BoundarySymbol>,
}

impl ClimateForcingSymbolSurface {
    /// Build canonical hillslope series symbols (`timem_XXXX`, `intsty_XXXX`).
    ///
    /// # Errors
    ///
    /// Returns `ClimateForcingSymbolSurfaceError` when point cardinality
    /// exceeds supported runtime bounds.
    pub fn hillslope(point_count: usize) -> Result<Self, ClimateForcingSymbolSurfaceError> {
        Self::build(None, point_count)
    }

    /// Build canonical watershed-hillslope scoped series symbols
    /// (`hs{id}_timem_XXXX`, `hs{id}_intsty_XXXX`).
    ///
    /// # Errors
    ///
    /// Returns `ClimateForcingSymbolSurfaceError` when point cardinality
    /// exceeds supported runtime bounds.
    pub fn watershed_hillslope(
        hillslope_id: u32,
        point_count: usize,
    ) -> Result<Self, ClimateForcingSymbolSurfaceError> {
        Self::build(Some(hillslope_id), point_count)
    }

    #[must_use]
    pub fn timem_symbols(&self) -> &[BoundarySymbol] {
        &self.timem_symbols
    }

    #[must_use]
    pub fn intsty_symbols(&self) -> &[BoundarySymbol] {
        &self.intsty_symbols
    }

    #[must_use]
    pub fn point_count(&self) -> usize {
        self.timem_symbols.len()
    }

    fn build(
        hillslope_id: Option<u32>,
        point_count: usize,
    ) -> Result<Self, ClimateForcingSymbolSurfaceError> {
        if point_count > MAX_CLIMATE_FORCING_SERIES_POINTS {
            return Err(ClimateForcingSymbolSurfaceError::PointCountOutOfRange {
                count: point_count,
                supported_max: MAX_CLIMATE_FORCING_SERIES_POINTS,
            });
        }

        let mut timem_symbols = Vec::with_capacity(point_count);
        let mut intsty_symbols = Vec::with_capacity(point_count);
        for index in 1..=point_count {
            timem_symbols.push(build_series_symbol(hillslope_id, "timem", index));
            intsty_symbols.push(build_series_symbol(hillslope_id, "intsty", index));
        }

        Ok(Self {
            timem_symbols,
            intsty_symbols,
        })
    }
}

/// Typed failure for climate forcing symbol-surface construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClimateForcingSymbolSurfaceError {
    PointCountOutOfRange { count: usize, supported_max: usize },
}

impl fmt::Display for ClimateForcingSymbolSurfaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PointCountOutOfRange {
                count,
                supported_max,
            } => write!(
                f,
                "climate forcing point count {count} exceeds supported maximum {supported_max}"
            ),
        }
    }
}

impl Error for ClimateForcingSymbolSurfaceError {}

fn build_series_symbol(
    hillslope_id: Option<u32>,
    series: &str,
    one_based_index: usize,
) -> BoundarySymbol {
    match hillslope_id {
        Some(id) => BoundarySymbol::from(format!("hs{id}_{series}_{one_based_index:04}")),
        None => BoundarySymbol::from(format!("{series}_{one_based_index:04}")),
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
///
/// Scheduler execution keeps state/flux ownership and lends immutable views to
/// kernels to avoid full-surface cloning in hot paths.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HillslopeKernelRequest<'a> {
    pub phase_name: &'a str,
    pub state_surface: &'a BTreeMap<BoundarySymbol, BoundaryValue>,
    pub flux_surface: &'a BTreeMap<BoundarySymbol, BoundaryValue>,
}

impl<'a> HillslopeKernelRequest<'a> {
    #[must_use]
    pub fn new(
        phase_name: &'a str,
        state_surface: &'a BTreeMap<BoundarySymbol, BoundaryValue>,
        flux_surface: &'a BTreeMap<BoundarySymbol, BoundaryValue>,
    ) -> Self {
        Self {
            phase_name,
            state_surface,
            flux_surface,
        }
    }
}

/// Watershed kernel invocation request.
///
/// State/flux surfaces are borrowed from orchestrator-owned writeback maps to
/// reduce scheduler hot-path allocation pressure.
#[derive(Debug, Clone, PartialEq)]
pub struct WatershedKernelRequest<'a> {
    pub node_kind: &'a str,
    pub node_id: u32,
    pub dependency_nodes: Vec<String>,
    pub contributor_hillslopes: &'a [u32],
    pub state_surface: &'a BTreeMap<BoundarySymbol, BoundaryValue>,
    pub flux_surface: &'a BTreeMap<BoundarySymbol, BoundaryValue>,
}

impl<'a> WatershedKernelRequest<'a> {
    #[must_use]
    pub fn new(
        node_kind: &'a str,
        node_id: u32,
        dependency_nodes: Vec<String>,
        contributor_hillslopes: &'a [u32],
        state_surface: &'a BTreeMap<BoundarySymbol, BoundaryValue>,
        flux_surface: &'a BTreeMap<BoundarySymbol, BoundaryValue>,
    ) -> Self {
        Self {
            node_kind,
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
    fn run_hillslope_phase(&mut self, request: &HillslopeKernelRequest<'_>) -> KernelRunResponse;
}

/// Watershed kernel trait boundary.
pub trait WatershedKernel {
    fn run_watershed_node(&mut self, request: &WatershedKernelRequest<'_>) -> KernelRunResponse;
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
}
