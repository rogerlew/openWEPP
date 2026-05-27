//! Deterministic watershed dispatch scheduler graph for openWEPP.

pub mod runtime_inputs;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, KernelRunResponse, KernelWritebackApplyResult,
    KernelWritebackPayload, WatershedChannelFluxField, WatershedChannelStateField,
    WatershedImpoundmentFluxField, WatershedImpoundmentStateField, WatershedKernel,
    WatershedKernelRequest, WatershedProductionFluxSymbol, WatershedProductionStateSymbol,
    WritebackDecisionOutcome, WritebackError, WritebackField, apply_kernel_writeback,
    evaluate_kernel_writeback,
};
use openwepp_sim_contract::status::{
    BoundaryClass, SimulationPhase, SimulationStatus, StatusClassification, StatusError,
};
use openwepp_topology::{
    TopologyGraph, TopologyNodeKey, TopologyNodeKind, TopologyValidationError,
    TopologyValidationReport, validate_pre_execution_topology,
};

/// Success message id for a complete watershed dispatch schedule.
pub const MESSAGE_DISPATCH_OK: &str = "WATERSHED-DISPATCH-OK-001";
/// Failure message id when topology preconditions are not satisfied.
pub const MESSAGE_PRECONDITION_FAILED: &str = "WATERSHED-DISPATCH-E-PRECONDITION-TOPOLOGY";
/// Failure message id when a dispatch dependency is missing from the graph.
pub const MESSAGE_MISSING_DEPENDENCY: &str = "WATERSHED-DISPATCH-E-MISSING-DEPENDENCY";
/// Failure message id when channel/impoundment dependencies contain a cycle.
pub const MESSAGE_CYCLE_DETECTED: &str = "WATERSHED-DISPATCH-E-CYCLE-DETECTED";

/// Diagnostic code for watershed dispatch failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DispatchDiagnosticCode {
    TopologyPreconditionFailed,
    MissingDependency,
    DependencyCycleDetected,
}

impl DispatchDiagnosticCode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TopologyPreconditionFailed => "TOPOLOGY_PRECONDITION_FAILED",
            Self::MissingDependency => "MISSING_DEPENDENCY",
            Self::DependencyCycleDetected => "DEPENDENCY_CYCLE_DETECTED",
        }
    }
}

/// Typed diagnostic record emitted by scheduler execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DispatchDiagnostic {
    pub code: DispatchDiagnosticCode,
    pub message_id: String,
    pub detail: String,
}

impl DispatchDiagnostic {
    #[must_use]
    pub fn new(
        code: DispatchDiagnosticCode,
        message_id: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            code,
            message_id: message_id.into(),
            detail: detail.into(),
        }
    }
}

/// One deterministic scheduler step for watershed dispatch execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DispatchStep {
    pub sequence_index: usize,
    pub node: TopologyNodeKey,
    pub dependency_nodes: Vec<TopologyNodeKey>,
    pub contributor_hillslopes: Vec<u32>,
    pub status: SimulationStatus,
}

/// Full scheduler report with precondition and dispatch outcomes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WatershedDispatchReport {
    pub precondition_status: SimulationStatus,
    pub dispatch_status: SimulationStatus,
    pub steps: Vec<DispatchStep>,
    pub diagnostics: Vec<DispatchDiagnostic>,
}

impl WatershedDispatchReport {
    #[must_use]
    pub fn is_success(&self) -> bool {
        self.precondition_status.classification() != StatusClassification::Failure
            && self.dispatch_status.classification() != StatusClassification::Failure
    }
}

/// Mutable state/flux maps owned by the watershed orchestrator.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WatershedWritebackSurface {
    pub state_surface: BTreeMap<BoundarySymbol, BoundaryValue>,
    pub flux_surface: BTreeMap<BoundarySymbol, BoundaryValue>,
}

/// Per-step watershed kernel/writeback execution evidence.
#[derive(Debug, Clone, PartialEq)]
pub struct WatershedKernelStepReport {
    pub step: DispatchStep,
    pub kernel_status: SimulationStatus,
    pub decision_outcome: WritebackDecisionOutcome,
    pub decision_status: SimulationStatus,
    pub apply_result: Option<KernelWritebackApplyResult>,
}

/// Kernel-integrated watershed execution report.
#[derive(Debug, Clone, PartialEq)]
pub struct WatershedKernelExecutionReport {
    pub dispatch_report: WatershedDispatchReport,
    pub step_reports: Vec<WatershedKernelStepReport>,
    pub writeback_surface: WatershedWritebackSurface,
}

/// Error surface for watershed dispatch scheduler orchestration.
#[derive(Debug)]
pub enum WatershedDispatchError {
    Status(StatusError),
    TopologyValidation(TopologyValidationError),
    Writeback(WritebackError),
}

impl fmt::Display for WatershedDispatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Status(source) => write!(f, "failed constructing dispatch status: {source}"),
            Self::TopologyValidation(source) => write!(
                f,
                "failed constructing topology validation gate report: {source}"
            ),
            Self::Writeback(source) => {
                write!(f, "failed applying watershed kernel writeback: {source}")
            }
        }
    }
}

impl Error for WatershedDispatchError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Status(source) => Some(source),
            Self::TopologyValidation(source) => Some(source),
            Self::Writeback(source) => Some(source),
        }
    }
}

impl From<StatusError> for WatershedDispatchError {
    fn from(value: StatusError) -> Self {
        Self::Status(value)
    }
}

impl From<TopologyValidationError> for WatershedDispatchError {
    fn from(value: TopologyValidationError) -> Self {
        Self::TopologyValidation(value)
    }
}

impl From<WritebackError> for WatershedDispatchError {
    fn from(value: WritebackError) -> Self {
        Self::Writeback(value)
    }
}

const WS10_ZERO_THRESHOLD: f64 = 1.0e-12;
const WS11_IPEAK_INTEGER_TOLERANCE: f64 = 1.0e-9;
const WS12_IMPOUNDMENT_ERROR_SCALE: f64 = 1.0e-4;
const WS12_IMPOUNDMENT_RETRY_LIMIT: usize = 64;

const WS10_CHANNEL_GUARD_MISSING_SYMBOL: &str = "WKERNEL-WS10-CHANNEL-E-001";
const WS10_CHANNEL_GUARD_NON_FINITE: &str = "WKERNEL-WS10-CHANNEL-E-002";
const WS10_CHANNEL_GUARD_DOMAIN: &str = "WKERNEL-WS10-CHANNEL-E-003";

const WS10_IMPOUNDMENT_GUARD_MISSING_SYMBOL: &str = "WKERNEL-WS10-IMPOUNDMENT-E-001";
const WS10_IMPOUNDMENT_GUARD_NON_FINITE: &str = "WKERNEL-WS10-IMPOUNDMENT-E-002";
const WS10_IMPOUNDMENT_GUARD_DOMAIN: &str = "WKERNEL-WS10-IMPOUNDMENT-E-003";

const WS10_CHANNEL_OK_MESSAGE_ID: &str = "WKERNEL-WS10-CHANNEL-OK-001";
const WS10_IMPOUNDMENT_OK_MESSAGE_ID: &str = "WKERNEL-WS10-IMPOUNDMENT-OK-001";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ws10NodeClass {
    Channel,
    Impoundment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ws10GuardClass {
    MissingRequiredInput,
    NonFinite,
    DomainViolation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ws11IpeakBranch {
    Rational,
    Creams,
    KinematicWave,
    MuskingumCunge,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Ws11WaveRoutingState {
    q1: f64,
    qin: f64,
    qlat: f64,
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    c4: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Ws12ImpoundmentCoefficients {
    a: [f64; 15],
    b: [f64; 15],
    c: [f64; 15],
    d: [f64; 15],
    e: [f64; 15],
    ha: [f64; 15],
    a0: f64,
    a1: f64,
    a2: f64,
}

#[derive(Debug, Clone, PartialEq)]
struct Ws10GuardError {
    node_class: Ws10NodeClass,
    guard_class: Ws10GuardClass,
}

impl Ws10GuardError {
    #[must_use]
    fn message_id(&self) -> &'static str {
        match (self.node_class, self.guard_class) {
            (Ws10NodeClass::Channel, Ws10GuardClass::MissingRequiredInput) => {
                WS10_CHANNEL_GUARD_MISSING_SYMBOL
            }
            (Ws10NodeClass::Channel, Ws10GuardClass::NonFinite) => WS10_CHANNEL_GUARD_NON_FINITE,
            (Ws10NodeClass::Channel, Ws10GuardClass::DomainViolation) => WS10_CHANNEL_GUARD_DOMAIN,
            (Ws10NodeClass::Impoundment, Ws10GuardClass::MissingRequiredInput) => {
                WS10_IMPOUNDMENT_GUARD_MISSING_SYMBOL
            }
            (Ws10NodeClass::Impoundment, Ws10GuardClass::NonFinite) => {
                WS10_IMPOUNDMENT_GUARD_NON_FINITE
            }
            (Ws10NodeClass::Impoundment, Ws10GuardClass::DomainViolation) => {
                WS10_IMPOUNDMENT_GUARD_DOMAIN
            }
        }
    }

    #[must_use]
    const fn boundary_class(&self) -> BoundaryClass {
        match self.guard_class {
            Ws10GuardClass::MissingRequiredInput => BoundaryClass::MissingRequiredInput,
            Ws10GuardClass::NonFinite => BoundaryClass::NonFinite,
            Ws10GuardClass::DomainViolation => BoundaryClass::DomainViolation,
        }
    }
}

/// WS10 production watershed kernel for channel and impoundment execution.
#[derive(Debug, Default, Clone, Copy)]
pub struct Ws10ChannelImpoundmentKernel;

impl Ws10ChannelImpoundmentKernel {
    fn missing_required(
        node_class: Ws10NodeClass,
        symbol: impl Into<BoundarySymbol>,
    ) -> Ws10GuardError {
        let _ = symbol.into();
        Ws10GuardError {
            node_class,
            guard_class: Ws10GuardClass::MissingRequiredInput,
        }
    }

    fn non_finite(
        node_class: Ws10NodeClass,
        symbol: impl Into<BoundarySymbol>,
        value: f64,
    ) -> Ws10GuardError {
        let _ = symbol.into();
        let _ = value;
        Ws10GuardError {
            node_class,
            guard_class: Ws10GuardClass::NonFinite,
        }
    }

    fn domain_violation(
        node_class: Ws10NodeClass,
        symbol: impl Into<BoundarySymbol>,
        value: f64,
    ) -> Ws10GuardError {
        let _ = symbol.into();
        let _ = value;
        Ws10GuardError {
            node_class,
            guard_class: Ws10GuardClass::DomainViolation,
        }
    }

    fn require_state_scalar(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
        symbol: WatershedProductionStateSymbol,
    ) -> Result<f64, Ws10GuardError> {
        let key = BoundarySymbol::from(symbol);
        let Some(value) = request.state_surface.get(&key) else {
            return Err(Self::missing_required(node_class, key));
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Self::non_finite(node_class, key, scalar));
        }
        Ok(scalar)
    }

    fn require_impoundment_coefficient_scalar(
        request: &WatershedKernelRequest<'_>,
        node_id: u32,
        suffix: &'static str,
    ) -> Result<f64, Ws10GuardError> {
        let node_class = Ws10NodeClass::Impoundment;
        let key = BoundarySymbol::from(format!("ws10_impoundment_{node_id}_{suffix}"));
        let Some(value) = request.state_surface.get(&key) else {
            return Err(Self::missing_required(node_class, key));
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Self::non_finite(node_class, key, scalar));
        }
        Ok(scalar)
    }

    fn require_impoundment_function_coefficient_scalar(
        request: &WatershedKernelRequest<'_>,
        node_id: u32,
        family_index: usize,
        suffix: &'static str,
    ) -> Result<f64, Ws10GuardError> {
        let node_class = Ws10NodeClass::Impoundment;
        let key = BoundarySymbol::from(format!(
            "ws10_impoundment_{node_id}_f{family_index:02}_{suffix}"
        ));
        let Some(value) = request.state_surface.get(&key) else {
            return Err(Self::missing_required(node_class, key));
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Self::non_finite(node_class, key, scalar));
        }
        Ok(scalar)
    }

    fn require_flux_scalar(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
        symbol: WatershedProductionFluxSymbol,
    ) -> Result<f64, Ws10GuardError> {
        let key = BoundarySymbol::from(symbol);
        let Some(value) = request.flux_surface.get(&key) else {
            return Err(Self::missing_required(node_class, key));
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Self::non_finite(node_class, key, scalar));
        }
        Ok(scalar)
    }

    fn require_state_range(
        node_class: Ws10NodeClass,
        symbol: WatershedProductionStateSymbol,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Result<(), Ws10GuardError> {
        if let Some(minimum_value) = minimum
            && value < minimum_value
        {
            return Err(Self::domain_violation(node_class, symbol, value));
        }
        if let Some(maximum_value) = maximum
            && value > maximum_value
        {
            return Err(Self::domain_violation(node_class, symbol, value));
        }
        Ok(())
    }

    fn require_flux_range(
        node_class: Ws10NodeClass,
        symbol: WatershedProductionFluxSymbol,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Result<(), Ws10GuardError> {
        if let Some(minimum_value) = minimum
            && value < minimum_value
        {
            return Err(Self::domain_violation(node_class, symbol, value));
        }
        if let Some(maximum_value) = maximum
            && value > maximum_value
        {
            return Err(Self::domain_violation(node_class, symbol, value));
        }
        Ok(())
    }

    fn parse_dependency(
        node_class: Ws10NodeClass,
        dependency: &str,
    ) -> Result<(Ws10NodeClass, u32), Ws10GuardError> {
        let Some((kind, id_text)) = dependency.split_once(':') else {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("dependency_node"),
                -1.0,
            ));
        };
        let Ok(id) = id_text.parse::<u32>() else {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("dependency_node"),
                -1.0,
            ));
        };

        match kind {
            "channel" => Ok((Ws10NodeClass::Channel, id)),
            "impoundment" => Ok((Ws10NodeClass::Impoundment, id)),
            _ => Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("dependency_node"),
                -1.0,
            )),
        }
    }

    #[allow(clippy::too_many_lines)]
    fn impoundment_outflow_at_stage(
        node_class: Ws10NodeClass,
        stage: f64,
        coefficients: &Ws12ImpoundmentCoefficients,
    ) -> Result<f64, Ws10GuardError> {
        let mut q = [0.0_f64; 15];
        let htw = 0.0;

        // Drop spillway family (qo1..qo3)
        if stage > coefficients.ha[0] {
            q[0] = coefficients.b[0] * (stage - coefficients.ha[0]).powf(coefficients.c[0]);
        }
        if stage > coefficients.ha[1] {
            q[1] = coefficients.b[1] * (stage - coefficients.ha[1]).powf(coefficients.c[1]);
        }
        if stage > coefficients.ha[2] {
            let adjusted_head = if htw > coefficients.a[2] {
                stage - (coefficients.ha[2] + htw - coefficients.a[2])
            } else {
                stage - coefficients.ha[2]
            };
            if adjusted_head > 0.0 {
                q[2] = coefficients.b[2] * adjusted_head.powf(coefficients.c[2]);
            }
        }

        // Culvert #1 family (qo4..qo6)
        if stage > coefficients.ha[3] {
            if coefficients.b[3] <= WS10_ZERO_THRESHOLD {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("f04_b"),
                    coefficients.b[3],
                ));
            }
            let base = (stage - coefficients.ha[3]) / coefficients.b[3];
            if base > 0.0 {
                q[3] = coefficients.a[3] * base.powf(coefficients.c[3]);
            }
        }
        if stage > coefficients.ha[4] {
            if coefficients.b[4].abs() <= WS10_ZERO_THRESHOLD
                || coefficients.d[4].abs() <= WS10_ZERO_THRESHOLD
            {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("f05_bd"),
                    coefficients.b[4] + coefficients.d[4],
                ));
            }
            let base = (((stage - coefficients.ha[4]) / coefficients.b[4]) + coefficients.c[4])
                / coefficients.d[4];
            if base > 0.0 {
                q[4] = coefficients.a[4] * base.sqrt();
            }
        }
        if stage > coefficients.ha[5] {
            let adjusted_head = if htw > coefficients.a[5] {
                stage - (coefficients.ha[5] + htw - coefficients.a[5])
            } else {
                stage - coefficients.ha[5]
            };
            if adjusted_head > 0.0 {
                q[5] = coefficients.b[5] * adjusted_head.powf(coefficients.c[5]);
            }
        }

        // Culvert #2 family (qo7..qo9)
        if stage > coefficients.ha[6] {
            if coefficients.b[6] <= WS10_ZERO_THRESHOLD {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("f07_b"),
                    coefficients.b[6],
                ));
            }
            let base = (stage - coefficients.ha[6]) / coefficients.b[6];
            if base > 0.0 {
                q[6] = coefficients.a[6] * base.powf(coefficients.c[6]);
            }
        }
        if stage > coefficients.ha[7] {
            if coefficients.b[7].abs() <= WS10_ZERO_THRESHOLD
                || coefficients.d[7].abs() <= WS10_ZERO_THRESHOLD
            {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("f08_bd"),
                    coefficients.b[7] + coefficients.d[7],
                ));
            }
            let base = (((stage - coefficients.ha[7]) / coefficients.b[7]) + coefficients.c[7])
                / coefficients.d[7];
            if base > 0.0 {
                q[7] = coefficients.a[7] * base.sqrt();
            }
        }
        if stage > coefficients.ha[8] {
            let adjusted_head = if htw > coefficients.a[8] {
                stage - (coefficients.ha[8] + htw - coefficients.a[8])
            } else {
                stage - coefficients.ha[8]
            };
            if adjusted_head > 0.0 {
                q[8] = coefficients.b[8] * adjusted_head.powf(coefficients.c[8]);
            }
        }

        // Rockfill family (qo10)
        if stage > coefficients.ha[9] {
            if coefficients.b[9] <= WS10_ZERO_THRESHOLD {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("f10_b"),
                    coefficients.b[9],
                ));
            }
            let base = (stage - coefficients.ha[9]) / coefficients.b[9];
            if base > 0.0 {
                q[9] += coefficients.a[9] * base.powf(coefficients.c[9]);
            }
        }
        if stage > coefficients.e[9] {
            q[9] += coefficients.d[9] * (stage - coefficients.e[9]).powf(1.5);
        }

        // Emergency spillway family (qo11)
        if stage > coefficients.ha[10] {
            let depth = stage - coefficients.ha[10];
            let polynomial = coefficients.a[10]
                + coefficients.b[10] * depth
                + coefficients.c[10] * depth.powi(2)
                + coefficients.d[10] * depth.powi(3)
                + coefficients.e[10] * depth.powi(4);
            if polynomial.is_finite() && polynomial > 0.0 {
                q[10] = polynomial;
            }
        }

        // Filter fence family (qo12)
        if stage > coefficients.ha[11] {
            q[11] = coefficients.a[11] * (stage - coefficients.ha[11]);
            if stage > coefficients.d[11] {
                let overtopping_depth = stage - coefficients.d[11];
                q[11] += (coefficients.b[11] + coefficients.c[11] * overtopping_depth)
                    * overtopping_depth.powf(1.5);
            }
        }

        // Perforated riser family (qo13..qo15)
        if stage > coefficients.ha[12] {
            let depth = stage - coefficients.ha[12];
            if depth > 0.0 {
                let denominator = coefficients.b[12] + coefficients.c[12] / depth.powf(1.5);
                if denominator <= WS10_ZERO_THRESHOLD || !denominator.is_finite() {
                    return Err(Self::domain_violation(
                        node_class,
                        BoundarySymbol::from("f13_denominator"),
                        denominator,
                    ));
                }
                q[12] = coefficients.a[12] / denominator;
            }
        }
        if stage > coefficients.ha[13] {
            q[13] = coefficients.a[13] * (stage - coefficients.ha[13]).sqrt();
        }
        if stage > coefficients.ha[14] {
            q[14] = coefficients.b[14] * (stage - coefficients.ha[14]).powf(coefficients.c[14]);
        }

        let group_1 = q[0].min(q[1]).min(q[2]);
        let group_2 = q[3].min(q[4]).min(q[5]);
        let group_3 = q[6].min(q[7]).min(q[8]);
        let group_4 = q[12].min(q[13]).min(q[14]);
        let qo = group_1 + group_2 + group_3 + q[9] + q[10] + q[11] + group_4;

        if !qo.is_finite() {
            return Err(Self::non_finite(node_class, BoundarySymbol::from("qo"), qo));
        }
        if qo < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("qo"),
                qo,
            ));
        }
        Ok(qo)
    }

    fn impoundment_area_at_stage(
        node_class: Ws10NodeClass,
        stage: f64,
        coefficients: &Ws12ImpoundmentCoefficients,
    ) -> Result<f64, Ws10GuardError> {
        let area = coefficients.a0 + coefficients.a1 * stage.powf(coefficients.a2);
        if !area.is_finite() || area <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("area"),
                area,
            ));
        }
        Ok(area)
    }

    fn impoundment_continuity_rate(
        node_class: Ws10NodeClass,
        stage: f64,
        incoming_peak: f64,
        qinf: f64,
        coefficients: &Ws12ImpoundmentCoefficients,
    ) -> Result<f64, Ws10GuardError> {
        if !stage.is_finite() {
            return Err(Self::non_finite(
                node_class,
                BoundarySymbol::from("stage"),
                stage,
            ));
        }
        if stage < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("stage"),
                stage,
            ));
        }

        let area = Self::impoundment_area_at_stage(node_class, stage, coefficients)?;
        let qo = Self::impoundment_outflow_at_stage(node_class, stage, coefficients)?;
        let continuity_outflow = qo + qinf;
        if !continuity_outflow.is_finite() || continuity_outflow < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("continuity_outflow"),
                continuity_outflow,
            ));
        }

        let dhdt = (incoming_peak - continuity_outflow) / area;
        if !dhdt.is_finite() {
            return Err(Self::non_finite(
                node_class,
                BoundarySymbol::from("dhdt"),
                dhdt,
            ));
        }
        Ok(dhdt)
    }

    fn impoundment_rk4_step(
        node_class: Ws10NodeClass,
        stage: f64,
        dt: f64,
        incoming_peak: f64,
        qinf: f64,
        coefficients: &Ws12ImpoundmentCoefficients,
    ) -> Result<f64, Ws10GuardError> {
        if !dt.is_finite() || dt <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("dt"),
                dt,
            ));
        }

        let k1 = Self::impoundment_continuity_rate(
            node_class,
            stage,
            incoming_peak,
            qinf,
            coefficients,
        )?;
        let k2 = Self::impoundment_continuity_rate(
            node_class,
            stage + 0.5 * dt * k1,
            incoming_peak,
            qinf,
            coefficients,
        )?;
        let k3 = Self::impoundment_continuity_rate(
            node_class,
            stage + 0.5 * dt * k2,
            incoming_peak,
            qinf,
            coefficients,
        )?;
        let k4 = Self::impoundment_continuity_rate(
            node_class,
            stage + dt * k3,
            incoming_peak,
            qinf,
            coefficients,
        )?;

        let hnext = stage + (dt / 6.0) * (k1 + k4 + 2.0 * (k2 + k3));
        if !hnext.is_finite() {
            return Err(Self::non_finite(
                node_class,
                BoundarySymbol::from("hnext"),
                hnext,
            ));
        }
        Ok(hnext)
    }

    fn crosses_threshold(h_start: f64, h_end: f64, threshold: f64) -> bool {
        (h_start < threshold && h_end > threshold) || (h_start > threshold && h_end < threshold)
    }

    fn impoundment_crosses_regime_transition(
        h_start: f64,
        h_end: f64,
        coefficients: &Ws12ImpoundmentCoefficients,
    ) -> bool {
        coefficients
            .ha
            .iter()
            .copied()
            .any(|threshold| Self::crosses_threshold(h_start, h_end, threshold))
            || Self::crosses_threshold(h_start, h_end, coefficients.e[9])
            || Self::crosses_threshold(h_start, h_end, coefficients.d[11])
    }

    fn integrate_impoundment_stage_with_adaptive_retry(
        node_class: Ws10NodeClass,
        stage_h: f64,
        hfull: f64,
        deltat: f64,
        incoming_peak: f64,
        qinf: f64,
        coefficients: &Ws12ImpoundmentCoefficients,
    ) -> Result<(f64, f64), Ws10GuardError> {
        let mut dt = deltat;
        let mut retries = 0_usize;

        loop {
            if retries >= WS12_IMPOUNDMENT_RETRY_LIMIT {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("adaptive_retry"),
                    dt,
                ));
            }
            if !dt.is_finite() || dt <= WS10_ZERO_THRESHOLD {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("deltat"),
                    dt,
                ));
            }

            let half_dt = 0.5 * dt;
            if half_dt <= WS10_ZERO_THRESHOLD {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("deltat"),
                    dt,
                ));
            }

            let half_stage = Self::impoundment_rk4_step(
                node_class,
                stage_h,
                half_dt,
                incoming_peak,
                qinf,
                coefficients,
            )?;
            let two_half_stage = Self::impoundment_rk4_step(
                node_class,
                half_stage,
                half_dt,
                incoming_peak,
                qinf,
                coefficients,
            )?;
            let full_stage = Self::impoundment_rk4_step(
                node_class,
                stage_h,
                dt,
                incoming_peak,
                qinf,
                coefficients,
            )?;

            let stage_error = two_half_stage - full_stage;
            if !stage_error.is_finite() {
                return Err(Self::non_finite(
                    node_class,
                    BoundarySymbol::from("stage_error"),
                    stage_error,
                ));
            }
            let errmax = stage_error.abs() / WS12_IMPOUNDMENT_ERROR_SCALE;
            if !errmax.is_finite() {
                return Err(Self::non_finite(
                    node_class,
                    BoundarySymbol::from("errmax"),
                    errmax,
                ));
            }
            if errmax > 1.0 {
                dt = 0.9 * dt * errmax.powf(-0.25);
                retries += 1;
                continue;
            }

            let corrected_hnext = two_half_stage + (stage_error / 15.0);
            if !corrected_hnext.is_finite() {
                return Err(Self::non_finite(
                    node_class,
                    BoundarySymbol::from("hnext"),
                    corrected_hnext,
                ));
            }
            if !(0.0..=hfull).contains(&corrected_hnext) {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("hnext"),
                    corrected_hnext,
                ));
            }

            if Self::impoundment_crosses_regime_transition(stage_h, corrected_hnext, coefficients)
                && dt > WS10_ZERO_THRESHOLD * 2.0
            {
                dt *= 0.5;
                retries += 1;
                continue;
            }

            return Ok((corrected_hnext, dt));
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn route_impoundment_stage_over_duration(
        node_class: Ws10NodeClass,
        stage_h: f64,
        hfull: f64,
        deltat: f64,
        total_duration_hours: f64,
        incoming_peak: f64,
        qinf: f64,
        coefficients: &Ws12ImpoundmentCoefficients,
    ) -> Result<(f64, f64), Ws10GuardError> {
        if !total_duration_hours.is_finite() || total_duration_hours <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("total_duration_hours"),
                total_duration_hours,
            ));
        }

        let mut stage = stage_h;
        let mut remaining = total_duration_hours;
        let mut last_accepted_dt = deltat;
        let mut iterations = 0_usize;

        while remaining > WS10_ZERO_THRESHOLD {
            if iterations >= WS12_IMPOUNDMENT_RETRY_LIMIT {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("integration_iterations"),
                    remaining,
                ));
            }
            let step_trial_dt = deltat.min(remaining);
            let (step_hnext, accepted_dt) = Self::integrate_impoundment_stage_with_adaptive_retry(
                node_class,
                stage,
                hfull,
                step_trial_dt,
                incoming_peak,
                qinf,
                coefficients,
            )?;
            if !accepted_dt.is_finite() || accepted_dt <= WS10_ZERO_THRESHOLD {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("accepted_dt"),
                    accepted_dt,
                ));
            }
            if accepted_dt > remaining + WS10_ZERO_THRESHOLD {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("accepted_dt"),
                    accepted_dt,
                ));
            }

            stage = step_hnext;
            remaining -= accepted_dt;
            last_accepted_dt = accepted_dt;
            iterations += 1;
        }

        Ok((stage, last_accepted_dt))
    }

    fn read_hillslope_peak_payload(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
        hillslope_id: u32,
    ) -> Result<(f64, f64), Ws10GuardError> {
        let peak_symbol = WatershedProductionStateSymbol::HillslopeContributorPeak { hillslope_id };
        let dur_symbol =
            WatershedProductionStateSymbol::HillslopeContributorDuration { hillslope_id };

        let peak = Self::require_state_scalar(request, node_class, peak_symbol)?;
        let duration = Self::require_state_scalar(request, node_class, dur_symbol)?;

        Self::require_state_range(node_class, peak_symbol, peak, Some(0.0), None)?;
        Self::require_state_range(node_class, dur_symbol, duration, Some(0.0), None)?;

        Ok((peak, duration))
    }

    fn read_hillslope_sediment_payload(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
        hillslope_id: u32,
    ) -> Result<f64, Ws10GuardError> {
        let total_detachment_symbol =
            WatershedProductionStateSymbol::HillslopeContributorTotalDetachmentKg { hillslope_id };
        let total_deposition_symbol =
            WatershedProductionStateSymbol::HillslopeContributorTotalDepositionKg { hillslope_id };
        let class_count_symbol =
            WatershedProductionStateSymbol::HillslopeContributorParticleClassCount { hillslope_id };

        let total_detachment =
            Self::require_state_scalar(request, node_class, total_detachment_symbol)?;
        let total_deposition =
            Self::require_state_scalar(request, node_class, total_deposition_symbol)?;
        let class_count_value =
            Self::require_state_scalar(request, node_class, class_count_symbol)?;

        Self::require_state_range(
            node_class,
            total_detachment_symbol,
            total_detachment,
            Some(0.0),
            None,
        )?;
        Self::require_state_range(
            node_class,
            total_deposition_symbol,
            total_deposition,
            Some(0.0),
            None,
        )?;
        Self::require_state_range(
            node_class,
            class_count_symbol,
            class_count_value,
            Some(1.0),
            None,
        )?;

        let rounded_class_count = class_count_value.round();
        if (class_count_value - rounded_class_count).abs() > WS11_IPEAK_INTEGER_TOLERANCE {
            return Err(Self::domain_violation(
                node_class,
                class_count_symbol,
                class_count_value,
            ));
        }
        if rounded_class_count < 1.0 {
            return Err(Self::domain_violation(
                node_class,
                class_count_symbol,
                class_count_value,
            ));
        }
        let class_count = format!("{rounded_class_count:.0}")
            .parse::<usize>()
            .map_err(|_| {
                Self::domain_violation(node_class, class_count_symbol, class_count_value)
            })?;
        if class_count == 0 {
            return Err(Self::domain_violation(
                node_class,
                class_count_symbol,
                class_count_value,
            ));
        }

        for class_index in 1..=class_count {
            let concentration_symbol =
                WatershedProductionStateSymbol::HillslopeContributorSedimentConcentrationKgM3 {
                    hillslope_id,
                    class_index,
                };
            let fraction_symbol =
                WatershedProductionStateSymbol::HillslopeContributorParticleFlowFraction {
                    hillslope_id,
                    class_index,
                };

            let concentration =
                Self::require_state_scalar(request, node_class, concentration_symbol)?;
            let fraction = Self::require_state_scalar(request, node_class, fraction_symbol)?;

            Self::require_state_range(
                node_class,
                concentration_symbol,
                concentration,
                Some(0.0),
                None,
            )?;
            Self::require_state_range(node_class, fraction_symbol, fraction, Some(0.0), Some(1.0))?;
        }

        Ok((total_detachment - total_deposition).max(0.0))
    }

    fn read_dependency_peak_payload(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
        dependency_class: Ws10NodeClass,
        dependency_id: u32,
    ) -> Result<(f64, f64), Ws10GuardError> {
        let (peak_symbol, duration_symbol) = match dependency_class {
            Ws10NodeClass::Channel => (
                WatershedProductionStateSymbol::ChannelNode {
                    node_id: dependency_id,
                    field: WatershedChannelStateField::Qpo,
                },
                WatershedProductionStateSymbol::ChannelNode {
                    node_id: dependency_id,
                    field: WatershedChannelStateField::Durrof,
                },
            ),
            Ws10NodeClass::Impoundment => (
                WatershedProductionStateSymbol::ImpoundmentNode {
                    node_id: dependency_id,
                    field: WatershedImpoundmentStateField::Qo,
                },
                WatershedProductionStateSymbol::ImpoundmentNode {
                    node_id: dependency_id,
                    field: WatershedImpoundmentStateField::Durout,
                },
            ),
        };

        let peak = Self::require_state_scalar(request, node_class, peak_symbol)?;
        let duration = Self::require_state_scalar(request, node_class, duration_symbol)?;

        Self::require_state_range(node_class, peak_symbol, peak, Some(0.0), None)?;
        Self::require_state_range(node_class, duration_symbol, duration, Some(0.0), None)?;

        Ok((peak, duration))
    }

    fn assemble_incoming_peak_and_duration(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
    ) -> Result<(f64, f64), Ws10GuardError> {
        let mut incoming_peak = 0.0_f64;
        let mut incoming_duration = 0.0_f64;

        for &hillslope_id in request.contributor_hillslopes {
            let (peak, duration) =
                Self::read_hillslope_peak_payload(request, node_class, hillslope_id)?;
            let _ = Self::read_hillslope_sediment_payload(request, node_class, hillslope_id)?;
            incoming_peak += peak;
            incoming_duration = incoming_duration.max(duration);
        }

        for dependency in &request.dependency_nodes {
            let (dependency_class, dependency_id) = Self::parse_dependency(node_class, dependency)?;
            let (peak, duration) = Self::read_dependency_peak_payload(
                request,
                node_class,
                dependency_class,
                dependency_id,
            )?;
            incoming_peak += peak;
            incoming_duration = incoming_duration.max(duration);
        }

        if !incoming_peak.is_finite() {
            return Err(Self::non_finite(
                node_class,
                BoundarySymbol::from("incoming_peak"),
                incoming_peak,
            ));
        }
        if incoming_peak < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("incoming_peak"),
                incoming_peak,
            ));
        }
        if !incoming_duration.is_finite() {
            return Err(Self::non_finite(
                node_class,
                BoundarySymbol::from("incoming_duration"),
                incoming_duration,
            ));
        }
        if incoming_duration < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("incoming_duration"),
                incoming_duration,
            ));
        }

        Ok((incoming_peak, incoming_duration))
    }

    fn assemble_incoming_sediment_load_and_capacity(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
        event_duration: f64,
    ) -> Result<(f64, f64), Ws10GuardError> {
        if !event_duration.is_finite() || event_duration <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("event_duration"),
                event_duration,
            ));
        }

        let mut incoming_sediment_mass_kg = 0.0_f64;
        for &hillslope_id in request.contributor_hillslopes {
            incoming_sediment_mass_kg +=
                Self::read_hillslope_sediment_payload(request, node_class, hillslope_id)?;
        }
        if !incoming_sediment_mass_kg.is_finite() || incoming_sediment_mass_kg < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("incoming_sediment_mass_kg"),
                incoming_sediment_mass_kg,
            ));
        }

        let qsed = incoming_sediment_mass_kg / event_duration;
        if !qsed.is_finite() || qsed < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("qsed"),
                qsed,
            ));
        }

        let tc = qsed;
        if !tc.is_finite() || tc < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("tc"),
                tc,
            ));
        }

        Ok((qsed, tc))
    }

    fn require_ipeak_branch(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
    ) -> Result<Ws11IpeakBranch, Ws10GuardError> {
        let ipeak_symbol = WatershedProductionStateSymbol::Ipeak;
        let ipeak = Self::require_state_scalar(request, node_class, ipeak_symbol)?;
        Self::require_state_range(node_class, ipeak_symbol, ipeak, Some(1.0), None)?;

        let rounded_ipeak = ipeak.round();
        if (ipeak - rounded_ipeak).abs() > WS11_IPEAK_INTEGER_TOLERANCE {
            return Err(Self::domain_violation(node_class, ipeak_symbol, ipeak));
        }

        let branch = if (rounded_ipeak - 1.0).abs() <= WS11_IPEAK_INTEGER_TOLERANCE {
            Ws11IpeakBranch::Rational
        } else if (rounded_ipeak - 2.0).abs() <= WS11_IPEAK_INTEGER_TOLERANCE {
            Ws11IpeakBranch::Creams
        } else if (rounded_ipeak - 3.0).abs() <= WS11_IPEAK_INTEGER_TOLERANCE {
            Ws11IpeakBranch::KinematicWave
        } else {
            Ws11IpeakBranch::MuskingumCunge
        };

        Ok(branch)
    }

    fn channel_wave_state_symbol(node_id: u32, suffix: &'static str) -> BoundarySymbol {
        BoundarySymbol::from(format!("ws10_channel_{node_id}_{suffix}"))
    }

    fn require_non_negative_computed(
        node_class: Ws10NodeClass,
        symbol: impl Into<BoundarySymbol>,
        value: f64,
    ) -> Result<f64, Ws10GuardError> {
        let symbol = symbol.into();
        if !value.is_finite() {
            return Err(Self::non_finite(node_class, symbol, value));
        }
        if value < 0.0 {
            return Err(Self::domain_violation(node_class, symbol, value));
        }
        Ok(value)
    }

    #[allow(clippy::too_many_arguments)]
    fn compute_kinematic_wave_state(
        node_class: Ws10NodeClass,
        roughness: f64,
        conductivity: f64,
        nchnum: f64,
        routing_gain: f64,
        incoming_peak: f64,
        available_peak: f64,
        baseflow_peak: f64,
        dtchr: f64,
        event_duration: f64,
    ) -> Result<Ws11WaveRoutingState, Ws10GuardError> {
        let qin = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("qin"),
            available_peak,
        )?;
        let qin_previous = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("q1_previous"),
            incoming_peak,
        )?;
        let qlat = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("qlat"),
            baseflow_peak / event_duration,
        )?;

        let wave_storage = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("wave_storage"),
            1.0 + (roughness * dtchr) + (conductivity * nchnum),
        )?;
        if wave_storage <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("wave_storage"),
                wave_storage,
            ));
        }

        let c0 = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("c0"),
            1.0 / wave_storage,
        )?;
        let c1 = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("c1"),
            routing_gain / (1.0 + routing_gain),
        )?;
        let c2 = 0.0;
        let c3 =
            Self::require_non_negative_computed(node_class, BoundarySymbol::from("c3"), 1.0 - c1)?;
        let c4 = Self::require_non_negative_computed(node_class, BoundarySymbol::from("c4"), qlat)?;
        let q1 = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("q1"),
            ((c1 * qin) + (c3 * qin_previous) + c4) * c0,
        )?;

        Ok(Ws11WaveRoutingState {
            q1,
            qin,
            qlat,
            c0,
            c1,
            c2,
            c3,
            c4,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn compute_muskingum_cunge_state(
        node_class: Ws10NodeClass,
        roughness: f64,
        control_slope: f64,
        conductivity: f64,
        nchnum: f64,
        incoming_peak: f64,
        available_peak: f64,
        baseflow_peak: f64,
        dtchr: f64,
        event_duration: f64,
    ) -> Result<Ws11WaveRoutingState, Ws10GuardError> {
        let qin = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("qin"),
            available_peak,
        )?;
        let qin_previous = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("qin_previous"),
            incoming_peak,
        )?;
        let q1_previous = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("q1_previous"),
            incoming_peak,
        )?;
        let qlat = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("qlat"),
            baseflow_peak / event_duration,
        )?;

        let mc_translation = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("mc_translation"),
            1.0 + (conductivity * dtchr),
        )?;
        if mc_translation <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("mc_translation"),
                mc_translation,
            ));
        }

        let mc_storage = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("mc_storage"),
            1.0 + (roughness * dtchr) + (control_slope * nchnum),
        )?;
        if mc_storage <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("mc_storage"),
                mc_storage,
            ));
        }

        let denominator = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("mc_denominator"),
            mc_translation + mc_storage,
        )?;
        if denominator <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("mc_denominator"),
                denominator,
            ));
        }

        let c0 = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("c0"),
            1.0 / denominator,
        )?;
        let c1 = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("c1"),
            mc_translation * c0,
        )?;
        let c2 = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("c2"),
            0.5 * mc_storage * c0,
        )?;
        let c3 = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("c3"),
            1.0 - c1 - c2,
        )?;
        let c4 = Self::require_non_negative_computed(node_class, BoundarySymbol::from("c4"), qlat)?;
        let q1 = Self::require_non_negative_computed(
            node_class,
            BoundarySymbol::from("q1"),
            (c1 * qin) + (c2 * qin_previous) + (c3 * q1_previous) + c4,
        )?;

        Ok(Ws11WaveRoutingState {
            q1,
            qin,
            qlat,
            c0,
            c1,
            c2,
            c3,
            c4,
        })
    }

    #[allow(clippy::too_many_lines)]
    fn run_channel_node(
        request: &WatershedKernelRequest<'_>,
    ) -> Result<KernelRunResponse, Ws10GuardError> {
        let node_class = Ws10NodeClass::Channel;
        let dtchr_symbol = WatershedProductionStateSymbol::Dtchr;
        let dtchr = Self::require_state_scalar(request, node_class, dtchr_symbol)?;
        Self::require_state_range(
            node_class,
            dtchr_symbol,
            dtchr,
            Some(WS10_ZERO_THRESHOLD),
            None,
        )?;

        let nchnum_symbol = WatershedProductionStateSymbol::Nchnum;
        let nchnum = Self::require_state_scalar(request, node_class, nchnum_symbol)?;
        Self::require_state_range(
            node_class,
            nchnum_symbol,
            nchnum,
            Some(WS10_ZERO_THRESHOLD),
            None,
        )?;

        let cbase_symbol = WatershedProductionFluxSymbol::Cbase;
        let cbase = Self::require_flux_scalar(request, node_class, cbase_symbol)?;
        Self::require_flux_range(node_class, cbase_symbol, cbase, Some(0.0), None)?;
        let ipeak_branch = Self::require_ipeak_branch(request, node_class)?;

        let roughness_symbol = WatershedProductionStateSymbol::ChannelNode {
            node_id: request.node_id,
            field: WatershedChannelStateField::Chnn,
        };
        let slope_symbol = WatershedProductionStateSymbol::ChannelNode {
            node_id: request.node_id,
            field: WatershedChannelStateField::Ctlslp,
        };
        let conductivity_symbol = WatershedProductionStateSymbol::ChannelNode {
            node_id: request.node_id,
            field: WatershedChannelStateField::Chnk,
        };

        let roughness = Self::require_state_scalar(request, node_class, roughness_symbol)?;
        Self::require_state_range(
            node_class,
            roughness_symbol,
            roughness,
            Some(WS10_ZERO_THRESHOLD),
            None,
        )?;
        let control_slope = Self::require_state_scalar(request, node_class, slope_symbol)?;
        Self::require_state_range(node_class, slope_symbol, control_slope, Some(0.0), None)?;
        let conductivity = Self::require_state_scalar(request, node_class, conductivity_symbol)?;
        Self::require_state_range(
            node_class,
            conductivity_symbol,
            conductivity,
            Some(0.0),
            None,
        )?;

        let (incoming_peak, incoming_duration) =
            Self::assemble_incoming_peak_and_duration(request, node_class)?;

        let routing_gain = (1.0 + control_slope) / (1.0 + roughness);
        if !routing_gain.is_finite() || routing_gain <= 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("routing_gain"),
                routing_gain,
            ));
        }

        let baseflow_peak = cbase * nchnum * (1.0 + conductivity * dtchr);
        if !baseflow_peak.is_finite() || baseflow_peak < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("baseflow_peak"),
                baseflow_peak,
            ));
        }

        let available_peak = incoming_peak + baseflow_peak;
        if !available_peak.is_finite() || available_peak < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("available_peak"),
                available_peak,
            ));
        }

        let event_duration = incoming_duration.max(dtchr);
        if !event_duration.is_finite() || event_duration <= 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("event_duration"),
                event_duration,
            ));
        }

        let mut wave_state: Option<Ws11WaveRoutingState> = None;
        let qpo = if available_peak <= WS10_ZERO_THRESHOLD {
            0.0
        } else {
            match ipeak_branch {
                Ws11IpeakBranch::Rational => available_peak * routing_gain,
                Ws11IpeakBranch::Creams => {
                    let creams_attenuation = 1.0 + (conductivity * dtchr);
                    if !creams_attenuation.is_finite() || creams_attenuation <= 0.0 {
                        return Err(Self::domain_violation(
                            node_class,
                            BoundarySymbol::from("creams_attenuation"),
                            creams_attenuation,
                        ));
                    }

                    let creams_gain = (routing_gain / creams_attenuation).sqrt();
                    if !creams_gain.is_finite() || creams_gain <= 0.0 {
                        return Err(Self::domain_violation(
                            node_class,
                            BoundarySymbol::from("creams_gain"),
                            creams_gain,
                        ));
                    }

                    available_peak * creams_gain
                }
                Ws11IpeakBranch::KinematicWave => {
                    let state = Self::compute_kinematic_wave_state(
                        node_class,
                        roughness,
                        conductivity,
                        nchnum,
                        routing_gain,
                        incoming_peak,
                        available_peak,
                        baseflow_peak,
                        dtchr,
                        event_duration,
                    )?;
                    let q1 = state.q1;
                    wave_state = Some(state);
                    q1
                }
                Ws11IpeakBranch::MuskingumCunge => {
                    let state = Self::compute_muskingum_cunge_state(
                        node_class,
                        roughness,
                        control_slope,
                        conductivity,
                        nchnum,
                        incoming_peak,
                        available_peak,
                        baseflow_peak,
                        dtchr,
                        event_duration,
                    )?;
                    let q1 = state.q1;
                    wave_state = Some(state);
                    q1
                }
            }
        };

        if !qpo.is_finite() || qpo < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("qpo"),
                qpo,
            ));
        }

        let roff = if qpo <= WS10_ZERO_THRESHOLD {
            0.0
        } else {
            qpo * event_duration
        };
        if !roff.is_finite() || roff < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("roff"),
                roff,
            ));
        }

        let durrof = if qpo <= WS10_ZERO_THRESHOLD {
            0.0
        } else {
            roff / qpo
        };
        if !durrof.is_finite() || durrof < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("durrof"),
                durrof,
            ));
        }
        let (qsed, tc) = Self::assemble_incoming_sediment_load_and_capacity(
            request,
            node_class,
            event_duration,
        )?;

        let Ok(status) =
            SimulationStatus::ok(SimulationPhase::WatershedKernel, WS10_CHANNEL_OK_MESSAGE_ID)
        else {
            unreachable!("status message ids are non-empty WS10 constants")
        };

        let qpo_symbol = WatershedProductionStateSymbol::ChannelNode {
            node_id: request.node_id,
            field: WatershedChannelStateField::Qpo,
        };
        let durrof_symbol = WatershedProductionStateSymbol::ChannelNode {
            node_id: request.node_id,
            field: WatershedChannelStateField::Durrof,
        };
        let roff_symbol = WatershedProductionFluxSymbol::ChannelNode {
            node_id: request.node_id,
            field: WatershedChannelFluxField::Roff,
        };

        let mut state_updates = vec![
            WritebackField::bounded(qpo_symbol, qpo, Some(0.0), None),
            WritebackField::bounded(durrof_symbol, durrof, Some(0.0), None),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "qsed"),
                qsed,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "tc"),
                tc,
                Some(0.0),
                None,
            ),
        ];
        if let Some(state) = wave_state {
            let node_id = request.node_id;
            state_updates.push(WritebackField::bounded(
                Self::channel_wave_state_symbol(node_id, "q1"),
                state.q1,
                Some(0.0),
                None,
            ));
            state_updates.push(WritebackField::bounded(
                Self::channel_wave_state_symbol(node_id, "qin"),
                state.qin,
                Some(0.0),
                None,
            ));
            state_updates.push(WritebackField::bounded(
                Self::channel_wave_state_symbol(node_id, "qlat"),
                state.qlat,
                Some(0.0),
                None,
            ));
            state_updates.push(WritebackField::bounded(
                Self::channel_wave_state_symbol(node_id, "c0"),
                state.c0,
                Some(0.0),
                None,
            ));
            state_updates.push(WritebackField::bounded(
                Self::channel_wave_state_symbol(node_id, "c1"),
                state.c1,
                Some(0.0),
                None,
            ));
            state_updates.push(WritebackField::bounded(
                Self::channel_wave_state_symbol(node_id, "c2"),
                state.c2,
                Some(0.0),
                None,
            ));
            state_updates.push(WritebackField::bounded(
                Self::channel_wave_state_symbol(node_id, "c3"),
                state.c3,
                Some(0.0),
                None,
            ));
            state_updates.push(WritebackField::bounded(
                Self::channel_wave_state_symbol(node_id, "c4"),
                state.c4,
                Some(0.0),
                None,
            ));
        }

        let writeback = KernelWritebackPayload::with_updates(
            state_updates,
            vec![WritebackField::bounded(roff_symbol, roff, Some(0.0), None)],
        );

        Ok(KernelRunResponse::new(status, writeback))
    }

    #[allow(clippy::too_many_lines)]
    fn run_impoundment_node(
        request: &WatershedKernelRequest<'_>,
    ) -> Result<KernelRunResponse, Ws10GuardError> {
        let node_class = Ws10NodeClass::Impoundment;

        let h_symbol = WatershedProductionStateSymbol::ImpoundmentNode {
            node_id: request.node_id,
            field: WatershedImpoundmentStateField::H,
        };
        let hfull_symbol = WatershedProductionStateSymbol::ImpoundmentNode {
            node_id: request.node_id,
            field: WatershedImpoundmentStateField::Hfull,
        };
        let deltat_symbol = WatershedProductionStateSymbol::ImpoundmentNode {
            node_id: request.node_id,
            field: WatershedImpoundmentStateField::Deltat,
        };
        let qinf_symbol = WatershedProductionStateSymbol::ImpoundmentNode {
            node_id: request.node_id,
            field: WatershedImpoundmentStateField::Qinf,
        };

        let stage_h = Self::require_state_scalar(request, node_class, h_symbol)?;
        let hfull = Self::require_state_scalar(request, node_class, hfull_symbol)?;
        let deltat = Self::require_state_scalar(request, node_class, deltat_symbol)?;
        let qinf = Self::require_state_scalar(request, node_class, qinf_symbol)?;

        Self::require_state_range(node_class, h_symbol, stage_h, Some(0.0), None)?;
        Self::require_state_range(
            node_class,
            hfull_symbol,
            hfull,
            Some(WS10_ZERO_THRESHOLD),
            None,
        )?;
        if stage_h > hfull {
            return Err(Self::domain_violation(node_class, h_symbol, stage_h));
        }
        Self::require_state_range(
            node_class,
            deltat_symbol,
            deltat,
            Some(WS10_ZERO_THRESHOLD),
            None,
        )?;
        Self::require_state_range(node_class, qinf_symbol, qinf, Some(0.0), None)?;

        let (incoming_peak, incoming_duration) =
            Self::assemble_incoming_peak_and_duration(request, node_class)?;

        let mut family_a = [0.0_f64; 15];
        let mut family_b = [0.0_f64; 15];
        let mut family_c = [0.0_f64; 15];
        let mut family_d = [0.0_f64; 15];
        let mut family_e = [0.0_f64; 15];
        let mut family_head_threshold = [0.0_f64; 15];
        for family_index in 1..=15 {
            family_a[family_index - 1] = Self::require_impoundment_function_coefficient_scalar(
                request,
                request.node_id,
                family_index,
                "a",
            )?;
            family_b[family_index - 1] = Self::require_impoundment_function_coefficient_scalar(
                request,
                request.node_id,
                family_index,
                "b",
            )?;
            family_c[family_index - 1] = Self::require_impoundment_function_coefficient_scalar(
                request,
                request.node_id,
                family_index,
                "c",
            )?;
            family_d[family_index - 1] = Self::require_impoundment_function_coefficient_scalar(
                request,
                request.node_id,
                family_index,
                "d",
            )?;
            family_e[family_index - 1] = Self::require_impoundment_function_coefficient_scalar(
                request,
                request.node_id,
                family_index,
                "e",
            )?;
            family_head_threshold[family_index - 1] =
                Self::require_impoundment_function_coefficient_scalar(
                    request,
                    request.node_id,
                    family_index,
                    "ha",
                )?;
        }

        let a0 = Self::require_impoundment_coefficient_scalar(request, request.node_id, "a0")?;
        let a1 = Self::require_impoundment_coefficient_scalar(request, request.node_id, "a1")?;
        let a2 = Self::require_impoundment_coefficient_scalar(request, request.node_id, "a2")?;
        let _l0 = Self::require_impoundment_coefficient_scalar(request, request.node_id, "l0")?;
        let _l1 = Self::require_impoundment_coefficient_scalar(request, request.node_id, "l1")?;
        let _l2 = Self::require_impoundment_coefficient_scalar(request, request.node_id, "l2")?;

        let coefficients = Ws12ImpoundmentCoefficients {
            a: family_a,
            b: family_b,
            c: family_c,
            d: family_d,
            e: family_e,
            ha: family_head_threshold,
            a0,
            a1,
            a2,
        };

        let incoming_duration_hours = incoming_duration / 3600.0;
        if !incoming_duration_hours.is_finite() || incoming_duration_hours < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("incoming_duration"),
                incoming_duration,
            ));
        }
        let integration_horizon_hours = if incoming_duration_hours > WS10_ZERO_THRESHOLD {
            incoming_duration_hours
        } else {
            deltat
        };
        if !integration_horizon_hours.is_finite()
            || integration_horizon_hours <= WS10_ZERO_THRESHOLD
        {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("integration_horizon_hours"),
                integration_horizon_hours,
            ));
        }

        let (hnext, accepted_deltat) = Self::route_impoundment_stage_over_duration(
            node_class,
            stage_h,
            hfull,
            deltat,
            integration_horizon_hours,
            incoming_peak,
            qinf,
            &coefficients,
        )?;

        let qo = Self::impoundment_outflow_at_stage(node_class, hnext, &coefficients)?;
        let continuity_outflow = qo + qinf;
        if !continuity_outflow.is_finite() || continuity_outflow < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("continuity_outflow"),
                continuity_outflow,
            ));
        }

        let accepted_duration_seconds = accepted_deltat * 3600.0;
        if !accepted_duration_seconds.is_finite() || accepted_duration_seconds < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("accepted_duration_seconds"),
                accepted_duration_seconds,
            ));
        }

        let durout = incoming_duration.max(accepted_duration_seconds);
        if !durout.is_finite() || durout < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("durout"),
                durout,
            ));
        }

        let outflow_volume = qo * durout;
        if !outflow_volume.is_finite() || outflow_volume < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("outflow_volume"),
                outflow_volume,
            ));
        }

        let Ok(status) = SimulationStatus::ok(
            SimulationPhase::WatershedKernel,
            WS10_IMPOUNDMENT_OK_MESSAGE_ID,
        ) else {
            unreachable!("status message ids are non-empty WS10 constants")
        };

        let qo_symbol = WatershedProductionStateSymbol::ImpoundmentNode {
            node_id: request.node_id,
            field: WatershedImpoundmentStateField::Qo,
        };
        let durout_symbol = WatershedProductionStateSymbol::ImpoundmentNode {
            node_id: request.node_id,
            field: WatershedImpoundmentStateField::Durout,
        };
        let hnext_symbol = WatershedProductionStateSymbol::ImpoundmentNode {
            node_id: request.node_id,
            field: WatershedImpoundmentStateField::Hnext,
        };
        let outflow_symbol = WatershedProductionFluxSymbol::ImpoundmentNode {
            node_id: request.node_id,
            field: WatershedImpoundmentFluxField::OutflowVolume,
        };

        let writeback = KernelWritebackPayload::with_updates(
            vec![
                WritebackField::bounded(qo_symbol, qo, Some(0.0), None),
                WritebackField::bounded(durout_symbol, durout, Some(0.0), None),
                WritebackField::bounded(hnext_symbol, hnext, Some(0.0), Some(hfull)),
            ],
            vec![WritebackField::bounded(
                outflow_symbol,
                outflow_volume,
                Some(0.0),
                None,
            )],
        );

        Ok(KernelRunResponse::new(status, writeback))
    }

    fn status_from_guard_error(error: &Ws10GuardError) -> SimulationStatus {
        let Ok(status) = SimulationStatus::failure(
            SimulationPhase::WatershedKernel,
            true,
            false,
            error.boundary_class(),
            error.message_id(),
        ) else {
            unreachable!("status message ids are non-empty WS10 constants")
        };
        status
    }
}

impl WatershedKernel for Ws10ChannelImpoundmentKernel {
    fn run_watershed_node(&mut self, request: &WatershedKernelRequest<'_>) -> KernelRunResponse {
        let response = match request.node_kind {
            "channel" => Self::run_channel_node(request),
            "impoundment" => Self::run_impoundment_node(request),
            _ => Err(Self::domain_violation(
                Ws10NodeClass::Channel,
                BoundarySymbol::from("node_kind"),
                -1.0,
            )),
        };

        match response {
            Ok(response) => response,
            Err(error) => KernelRunResponse::new(
                Self::status_from_guard_error(&error),
                KernelWritebackPayload::empty(),
            ),
        }
    }
}

/// Schedule deterministic watershed dispatch order using an explicit topology
/// validation gate result as a hard precondition.
///
/// # Errors
///
/// Returns `WatershedDispatchError` when typed status construction fails.
pub fn schedule_watershed_dispatch(
    graph: &TopologyGraph,
    topology_validation: &TopologyValidationReport,
) -> Result<WatershedDispatchReport, WatershedDispatchError> {
    if !topology_validation.is_valid() {
        let dispatch_status = SimulationStatus::failure(
            SimulationPhase::WatershedKernel,
            true,
            false,
            BoundaryClass::TopologyInvalid,
            MESSAGE_PRECONDITION_FAILED,
        )?;

        let diagnostics = vec![DispatchDiagnostic::new(
            DispatchDiagnosticCode::TopologyPreconditionFailed,
            MESSAGE_PRECONDITION_FAILED,
            format!(
                "dispatch aborted: topology precondition failed with {} violation(s); gate_message_id={}",
                topology_validation.violations.len(),
                topology_validation.status.message_id()
            ),
        )];

        return Ok(WatershedDispatchReport {
            precondition_status: topology_validation.status.clone(),
            dispatch_status,
            steps: Vec::new(),
            diagnostics,
        });
    }

    match build_dispatch_steps(graph) {
        Ok(steps) => {
            let dispatch_status =
                SimulationStatus::ok(SimulationPhase::WatershedKernel, MESSAGE_DISPATCH_OK)?;

            Ok(WatershedDispatchReport {
                precondition_status: topology_validation.status.clone(),
                dispatch_status,
                steps,
                diagnostics: Vec::new(),
            })
        }
        Err(DispatchPlanError::MissingDependency { node, dependency }) => {
            let dispatch_status = SimulationStatus::failure(
                SimulationPhase::WatershedKernel,
                true,
                false,
                BoundaryClass::TopologyInvalid,
                MESSAGE_MISSING_DEPENDENCY,
            )?;

            let diagnostics = vec![DispatchDiagnostic::new(
                DispatchDiagnosticCode::MissingDependency,
                MESSAGE_MISSING_DEPENDENCY,
                format!(
                    "dispatch node {} depends on unresolved node {}",
                    format_node_key(node),
                    format_node_key(dependency)
                ),
            )];

            Ok(WatershedDispatchReport {
                precondition_status: topology_validation.status.clone(),
                dispatch_status,
                steps: Vec::new(),
                diagnostics,
            })
        }
        Err(DispatchPlanError::Status(source)) => Err(WatershedDispatchError::Status(source)),
        Err(DispatchPlanError::DependencyCycle { remaining_nodes }) => {
            let dispatch_status = SimulationStatus::failure(
                SimulationPhase::WatershedKernel,
                true,
                false,
                BoundaryClass::TopologyInvalid,
                MESSAGE_CYCLE_DETECTED,
            )?;

            let remaining_labels: Vec<String> = remaining_nodes
                .iter()
                .copied()
                .map(format_node_key)
                .collect();

            let diagnostics = vec![DispatchDiagnostic::new(
                DispatchDiagnosticCode::DependencyCycleDetected,
                MESSAGE_CYCLE_DETECTED,
                format!(
                    "dispatch dependency cycle detected; unresolved_nodes={}",
                    remaining_labels.join(",")
                ),
            )];

            Ok(WatershedDispatchReport {
                precondition_status: topology_validation.status.clone(),
                dispatch_status,
                steps: Vec::new(),
                diagnostics,
            })
        }
    }
}

/// Run the ARCH04 topology validation gate and execute deterministic dispatch
/// scheduling when preconditions pass.
///
/// # Errors
///
/// Returns `WatershedDispatchError` when topology validation or status
/// construction fails.
pub fn schedule_watershed_dispatch_with_gate(
    graph: &TopologyGraph,
) -> Result<WatershedDispatchReport, WatershedDispatchError> {
    let topology_validation = validate_pre_execution_topology(graph)?;
    schedule_watershed_dispatch(graph, &topology_validation)
}

/// Execute watershed dispatch scheduling and invoke watershed kernels through
/// the typed ARCH07 boundary.
///
/// Kernel writeback proposals are accepted/rejected/applied by orchestrator
/// policy. Kernel code never mutates orchestrator state directly.
///
/// # Errors
///
/// Returns `WatershedDispatchError` when scheduler/status construction fails or
/// when writeback apply surfaces return typed errors.
pub fn execute_watershed_dispatch_with_kernel<K>(
    graph: &TopologyGraph,
    topology_validation: &TopologyValidationReport,
    kernel: &mut K,
    mut writeback_surface: WatershedWritebackSurface,
) -> Result<WatershedKernelExecutionReport, WatershedDispatchError>
where
    K: WatershedKernel,
{
    let mut dispatch_report = schedule_watershed_dispatch(graph, topology_validation)?;

    if !dispatch_report.is_success() {
        return Ok(WatershedKernelExecutionReport {
            dispatch_report,
            step_reports: Vec::new(),
            writeback_surface,
        });
    }

    let mode_mismatch_status = SimulationStatus::failure(
        SimulationPhase::WatershedKernel,
        true,
        false,
        BoundaryClass::ModeMismatch,
        "WKERNEL-E-STATUS-PHASE-MISMATCH",
    )?;

    let mut step_reports = Vec::new();

    for step in dispatch_report.steps.iter().cloned() {
        let response = {
            let request = WatershedKernelRequest::new(
                step.node.kind.as_str(),
                step.node.id,
                step.dependency_nodes
                    .iter()
                    .map(|node| format_node_key(*node))
                    .collect::<Vec<String>>(),
                &step.contributor_hillslopes,
                &writeback_surface.state_surface,
                &writeback_surface.flux_surface,
            );

            kernel.run_watershed_node(&request)
        };
        let kernel_status = response.status.clone();

        if kernel_status.phase() != SimulationPhase::WatershedKernel {
            step_reports.push(WatershedKernelStepReport {
                step,
                kernel_status,
                decision_outcome: WritebackDecisionOutcome::Reject,
                decision_status: mode_mismatch_status.clone(),
                apply_result: None,
            });
            dispatch_report.dispatch_status = mode_mismatch_status.clone();
            break;
        }

        if kernel_status.classification() == StatusClassification::Failure {
            step_reports.push(WatershedKernelStepReport {
                step,
                kernel_status: kernel_status.clone(),
                decision_outcome: WritebackDecisionOutcome::Reject,
                decision_status: kernel_status.clone(),
                apply_result: None,
            });
            dispatch_report.dispatch_status = kernel_status;
            break;
        }

        let decision =
            evaluate_kernel_writeback(SimulationPhase::WatershedKernel, &response.writeback)?;
        if decision.outcome == WritebackDecisionOutcome::Reject {
            step_reports.push(WatershedKernelStepReport {
                step,
                kernel_status,
                decision_outcome: WritebackDecisionOutcome::Reject,
                decision_status: decision.status.clone(),
                apply_result: None,
            });
            dispatch_report.dispatch_status = decision.status;
            break;
        }

        let apply_result = apply_kernel_writeback(
            SimulationPhase::WatershedKernel,
            &decision,
            &response.writeback,
            &mut writeback_surface.state_surface,
            &mut writeback_surface.flux_surface,
        )?;

        step_reports.push(WatershedKernelStepReport {
            step,
            kernel_status: kernel_status.clone(),
            decision_outcome: apply_result.outcome,
            decision_status: apply_result.status.clone(),
            apply_result: Some(apply_result),
        });

        if kernel_status.classification() == StatusClassification::Advisory {
            dispatch_report.dispatch_status = kernel_status;
        }
    }

    Ok(WatershedKernelExecutionReport {
        dispatch_report,
        step_reports,
        writeback_surface,
    })
}

/// Execute topology validation gate + watershed dispatch + kernel writeback
/// protocol in one helper surface.
///
/// # Errors
///
/// Returns `WatershedDispatchError` when topology validation, dispatch status
/// construction, or writeback apply surfaces return typed errors.
pub fn execute_watershed_dispatch_with_gate_and_kernel<K>(
    graph: &TopologyGraph,
    kernel: &mut K,
    writeback_surface: WatershedWritebackSurface,
) -> Result<WatershedKernelExecutionReport, WatershedDispatchError>
where
    K: WatershedKernel,
{
    let topology_validation = validate_pre_execution_topology(graph)?;
    execute_watershed_dispatch_with_kernel(graph, &topology_validation, kernel, writeback_surface)
}

#[derive(Debug)]
enum DispatchPlanError {
    Status(StatusError),
    MissingDependency {
        node: TopologyNodeKey,
        dependency: TopologyNodeKey,
    },
    DependencyCycle {
        remaining_nodes: Vec<TopologyNodeKey>,
    },
}

fn build_dispatch_steps(graph: &TopologyGraph) -> Result<Vec<DispatchStep>, DispatchPlanError> {
    let dispatch_nodes: BTreeSet<TopologyNodeKey> = graph
        .nodes()
        .iter()
        .map(|node| node.key)
        .filter(|key| key.kind != TopologyNodeKind::Hillslope)
        .collect();

    let (dependencies, hillslope_contributors) = collect_dependency_maps(graph, &dispatch_nodes)?;
    let (mut indegree, dependents) = build_indegree_and_dependents(&dependencies, &dispatch_nodes);

    let mut ready: BTreeSet<TopologyNodeKey> = indegree
        .iter()
        .filter_map(|(node, count)| if *count == 0 { Some(*node) } else { None })
        .collect();

    let mut steps: Vec<DispatchStep> = Vec::new();

    while let Some(node) = ready.pop_first() {
        steps.push(build_dispatch_step(
            node,
            steps.len(),
            &dependencies,
            &hillslope_contributors,
        )?);

        if let Some(children) = dependents.get(&node) {
            for child in children {
                if let Some(count) = indegree.get_mut(child) {
                    *count -= 1;
                    if *count == 0 {
                        ready.insert(*child);
                    }
                }
            }
        }
    }

    if steps.len() != dispatch_nodes.len() {
        let remaining_nodes: Vec<TopologyNodeKey> = indegree
            .into_iter()
            .filter_map(|(node, count)| if count > 0 { Some(node) } else { None })
            .collect();

        return Err(DispatchPlanError::DependencyCycle { remaining_nodes });
    }

    Ok(steps)
}

type DependencyMap = BTreeMap<TopologyNodeKey, BTreeSet<TopologyNodeKey>>;
type HillslopeContributorMap = BTreeMap<TopologyNodeKey, BTreeSet<u32>>;
type IndegreeMap = BTreeMap<TopologyNodeKey, usize>;
type DependentMap = BTreeMap<TopologyNodeKey, BTreeSet<TopologyNodeKey>>;

fn collect_dependency_maps(
    graph: &TopologyGraph,
    dispatch_nodes: &BTreeSet<TopologyNodeKey>,
) -> Result<(DependencyMap, HillslopeContributorMap), DispatchPlanError> {
    let mut dependencies: DependencyMap = dispatch_nodes
        .iter()
        .copied()
        .map(|key| (key, BTreeSet::new()))
        .collect();
    let mut hillslope_contributors: HillslopeContributorMap = dispatch_nodes
        .iter()
        .copied()
        .map(|key| (key, BTreeSet::new()))
        .collect();

    for node in graph.nodes() {
        if node.key.kind == TopologyNodeKind::Hillslope {
            continue;
        }
        for (kind, _slot, contributor_id) in node.contributors.references() {
            if contributor_id == 0 {
                continue;
            }
            record_contributor(
                &mut dependencies,
                &mut hillslope_contributors,
                dispatch_nodes,
                node.key,
                kind,
                contributor_id,
            )?;
        }
    }

    Ok((dependencies, hillslope_contributors))
}

fn record_contributor(
    dependencies: &mut DependencyMap,
    hillslope_contributors: &mut HillslopeContributorMap,
    dispatch_nodes: &BTreeSet<TopologyNodeKey>,
    node: TopologyNodeKey,
    kind: TopologyNodeKind,
    contributor_id: u32,
) -> Result<(), DispatchPlanError> {
    match kind {
        TopologyNodeKind::Hillslope => {
            hillslope_contributors
                .entry(node)
                .or_default()
                .insert(contributor_id);
            Ok(())
        }
        TopologyNodeKind::Channel | TopologyNodeKind::Impoundment => {
            let dependency = TopologyNodeKey::new(kind, contributor_id);
            if !dispatch_nodes.contains(&dependency) {
                return Err(DispatchPlanError::MissingDependency { node, dependency });
            }

            dependencies.entry(node).or_default().insert(dependency);
            Ok(())
        }
    }
}

fn build_indegree_and_dependents(
    dependencies: &DependencyMap,
    dispatch_nodes: &BTreeSet<TopologyNodeKey>,
) -> (IndegreeMap, DependentMap) {
    let indegree: IndegreeMap = dependencies
        .iter()
        .map(|(node, parents)| (*node, parents.len()))
        .collect();
    let mut dependents: DependentMap = dispatch_nodes
        .iter()
        .copied()
        .map(|node| (node, BTreeSet::new()))
        .collect();

    for (node, parents) in dependencies {
        for parent in parents {
            dependents.entry(*parent).or_default().insert(*node);
        }
    }

    (indegree, dependents)
}

fn build_dispatch_step(
    node: TopologyNodeKey,
    sequence_index: usize,
    dependencies: &DependencyMap,
    hillslope_contributors: &HillslopeContributorMap,
) -> Result<DispatchStep, DispatchPlanError> {
    let parent_nodes: Vec<TopologyNodeKey> = dependencies
        .get(&node)
        .map(|parents| parents.iter().copied().collect())
        .unwrap_or_default();
    let hillslope_nodes: Vec<u32> = hillslope_contributors
        .get(&node)
        .map(|parents| parents.iter().copied().collect())
        .unwrap_or_default();
    let status = SimulationStatus::ok(
        SimulationPhase::WatershedKernel,
        format!(
            "WATERSHED-DISPATCH-STEP-{}-{}-OK",
            node_kind_message_token(node.kind),
            node.id
        ),
    )
    .map_err(DispatchPlanError::Status)?;

    Ok(DispatchStep {
        sequence_index,
        node,
        dependency_nodes: parent_nodes,
        contributor_hillslopes: hillslope_nodes,
        status,
    })
}

fn node_kind_message_token(kind: TopologyNodeKind) -> &'static str {
    match kind {
        TopologyNodeKind::Hillslope => "HILLSLOPE",
        TopologyNodeKind::Channel => "CHANNEL",
        TopologyNodeKind::Impoundment => "IMPOUNDMENT",
    }
}

fn format_node_key(key: TopologyNodeKey) -> String {
    format!("{}:{}", key.kind.as_str(), key.id)
}

#[cfg(test)]
mod tests {
    use openwepp_kernel_contract::{
        BoundarySymbol, BoundaryValue, KernelRunResponse, KernelWritebackPayload,
        WRITEBACK_REJECT_NON_FINITE_MESSAGE_ID, WatershedKernel, WatershedKernelRequest,
        WritebackDecisionOutcome, WritebackField,
    };
    use openwepp_sim_contract::status::{BoundaryClass, SimulationPhase, StatusClassification};
    use openwepp_topology::{
        ContributorTriplet, TopologyContributors, TopologyNode, validate_pre_execution_topology,
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
    fn execute_with_kernel_applies_writeback() {
        #[derive(Default)]
        struct NominalKernel {
            call_index: u32,
        }

        impl WatershedKernel for NominalKernel {
            fn run_watershed_node(
                &mut self,
                _request: &WatershedKernelRequest<'_>,
            ) -> KernelRunResponse {
                self.call_index += 1;
                let status = SimulationStatus::ok(
                    SimulationPhase::WatershedKernel,
                    format!("WKERNEL-STEP-OK-{}", self.call_index),
                )
                .expect("status should construct");
                let writeback = KernelWritebackPayload::with_updates(
                    vec![WritebackField::bounded(
                        "channel_storage",
                        f64::from(self.call_index),
                        Some(0.0),
                        Some(10_000.0),
                    )],
                    vec![WritebackField::bounded(
                        "discharge_total",
                        f64::from(self.call_index) * 0.5,
                        Some(0.0),
                        None,
                    )],
                );

                KernelRunResponse::new(status, writeback)
            }
        }

        let graph = TopologyGraph::new(
            2,
            1,
            0,
            vec![node(
                TopologyNodeKind::Channel,
                1,
                [1, 2, 0],
                [0, 0, 0],
                [0, 0, 0],
            )],
        );
        let topology_validation =
            validate_pre_execution_topology(&graph).expect("topology validation should construct");
        assert!(topology_validation.is_valid());

        let mut kernel = NominalKernel::default();
        let report = execute_watershed_dispatch_with_kernel(
            &graph,
            &topology_validation,
            &mut kernel,
            WatershedWritebackSurface::default(),
        )
        .expect("kernel execution should succeed");

        assert!(report.dispatch_report.is_success());
        assert_eq!(report.step_reports.len(), 1);
        assert_eq!(
            report.step_reports[0].decision_outcome,
            WritebackDecisionOutcome::Apply
        );
        assert_eq!(
            report
                .writeback_surface
                .state_surface
                .get(&BoundarySymbol::from("channel_storage"))
                .copied(),
            Some(BoundaryValue::from(1.0))
        );
        assert_eq!(
            report
                .writeback_surface
                .flux_surface
                .get(&BoundarySymbol::from("discharge_total"))
                .copied(),
            Some(BoundaryValue::from(0.5))
        );
    }

    #[test]
    fn execute_with_kernel_lends_stable_surface_references() {
        #[derive(Default)]
        struct PointerProbeKernel {
            call_index: u32,
            state_surface_ptrs: Vec<usize>,
            flux_surface_ptrs: Vec<usize>,
        }

        impl WatershedKernel for PointerProbeKernel {
            fn run_watershed_node(
                &mut self,
                request: &WatershedKernelRequest<'_>,
            ) -> KernelRunResponse {
                self.call_index += 1;
                self.state_surface_ptrs
                    .push(std::ptr::from_ref(request.state_surface) as usize);
                self.flux_surface_ptrs
                    .push(std::ptr::from_ref(request.flux_surface) as usize);
                let status = SimulationStatus::ok(
                    SimulationPhase::WatershedKernel,
                    format!("WKERNEL-STEP-POINTER-{}", self.call_index),
                )
                .expect("status should construct");

                KernelRunResponse::new(status, KernelWritebackPayload::empty())
            }
        }

        let graph = TopologyGraph::new(
            3,
            2,
            1,
            vec![
                node(
                    TopologyNodeKind::Channel,
                    1,
                    [1, 2, 0],
                    [0, 0, 0],
                    [0, 0, 0],
                ),
                node(
                    TopologyNodeKind::Channel,
                    2,
                    [3, 0, 0],
                    [1, 0, 0],
                    [0, 0, 0],
                ),
                node(
                    TopologyNodeKind::Impoundment,
                    1,
                    [0, 0, 0],
                    [2, 0, 0],
                    [0, 0, 0],
                ),
            ],
        );
        let topology_validation =
            validate_pre_execution_topology(&graph).expect("topology validation should construct");
        assert!(topology_validation.is_valid());

        let mut kernel = PointerProbeKernel::default();
        let report = execute_watershed_dispatch_with_kernel(
            &graph,
            &topology_validation,
            &mut kernel,
            WatershedWritebackSurface::default(),
        )
        .expect("kernel execution should succeed");

        assert!(report.dispatch_report.is_success());
        assert_eq!(report.step_reports.len(), 3);
        assert_eq!(kernel.state_surface_ptrs.len(), 3);
        assert_eq!(kernel.flux_surface_ptrs.len(), 3);
        assert!(
            kernel
                .state_surface_ptrs
                .windows(2)
                .all(|pair| pair[0] == pair[1]),
            "state surface reference should remain stable across dispatch calls"
        );
        assert!(
            kernel
                .flux_surface_ptrs
                .windows(2)
                .all(|pair| pair[0] == pair[1]),
            "flux surface reference should remain stable across dispatch calls"
        );
    }

    #[test]
    fn execute_with_kernel_rejects_non_finite_writeback() {
        struct RejectKernel;

        impl WatershedKernel for RejectKernel {
            fn run_watershed_node(
                &mut self,
                _request: &WatershedKernelRequest<'_>,
            ) -> KernelRunResponse {
                let status = SimulationStatus::ok(
                    SimulationPhase::WatershedKernel,
                    "WKERNEL-STEP-OK-REJECT",
                )
                .expect("status should construct");
                let writeback = KernelWritebackPayload::with_updates(
                    vec![WritebackField::unbounded("channel_storage", f64::INFINITY)],
                    Vec::new(),
                );
                KernelRunResponse::new(status, writeback)
            }
        }

        let graph = TopologyGraph::new(
            1,
            1,
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
        let mut kernel = RejectKernel;

        let report = execute_watershed_dispatch_with_kernel(
            &graph,
            &topology_validation,
            &mut kernel,
            WatershedWritebackSurface::default(),
        )
        .expect("execution should return typed report");

        assert!(!report.dispatch_report.is_success());
        assert_eq!(report.step_reports.len(), 1);
        assert_eq!(
            report.step_reports[0].decision_outcome,
            WritebackDecisionOutcome::Reject
        );
        assert_eq!(
            report.step_reports[0].decision_status.message_id(),
            WRITEBACK_REJECT_NON_FINITE_MESSAGE_ID
        );
        assert_eq!(
            report.dispatch_report.dispatch_status.message_id(),
            WRITEBACK_REJECT_NON_FINITE_MESSAGE_ID
        );
        assert!(
            report.writeback_surface.state_surface.is_empty(),
            "rejected payload must not mutate orchestrator writeback state"
        );
    }

    #[test]
    fn execute_with_kernel_rejects_status_phase_mismatch() {
        struct PhaseMismatchKernel;

        impl WatershedKernel for PhaseMismatchKernel {
            fn run_watershed_node(
                &mut self,
                _request: &WatershedKernelRequest<'_>,
            ) -> KernelRunResponse {
                let status = SimulationStatus::ok(
                    SimulationPhase::PreExecutionValidation,
                    "WKERNEL-STEP-INVALID-PHASE",
                )
                .expect("status should construct");
                KernelRunResponse::new(status, KernelWritebackPayload::empty())
            }
        }

        let graph = TopologyGraph::new(
            1,
            1,
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
        let mut kernel = PhaseMismatchKernel;

        let report = execute_watershed_dispatch_with_kernel(
            &graph,
            &topology_validation,
            &mut kernel,
            WatershedWritebackSurface::default(),
        )
        .expect("execution should return typed report");

        assert!(!report.dispatch_report.is_success());
        assert_eq!(
            report.dispatch_report.dispatch_status.boundary_class(),
            BoundaryClass::ModeMismatch
        );
        assert_eq!(report.step_reports.len(), 1);
        assert_eq!(
            report.step_reports[0].decision_outcome,
            WritebackDecisionOutcome::Reject
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
