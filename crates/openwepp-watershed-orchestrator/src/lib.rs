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
                    let wave_storage = 1.0 + (roughness * dtchr) + (conductivity * nchnum);
                    if !wave_storage.is_finite() || wave_storage <= 0.0 {
                        return Err(Self::domain_violation(
                            node_class,
                            BoundarySymbol::from("wave_storage"),
                            wave_storage,
                        ));
                    }

                    available_peak / wave_storage
                }
                Ws11IpeakBranch::MuskingumCunge => {
                    let mc_translation = 1.0 + (conductivity * dtchr);
                    if !mc_translation.is_finite() || mc_translation <= 0.0 {
                        return Err(Self::domain_violation(
                            node_class,
                            BoundarySymbol::from("mc_translation"),
                            mc_translation,
                        ));
                    }

                    let mc_storage = 1.0 + (roughness * dtchr) + (control_slope * nchnum);
                    if !mc_storage.is_finite() || mc_storage <= 0.0 {
                        return Err(Self::domain_violation(
                            node_class,
                            BoundarySymbol::from("mc_storage"),
                            mc_storage,
                        ));
                    }

                    available_peak * (mc_translation / mc_storage)
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

        let writeback = KernelWritebackPayload::with_updates(
            vec![
                WritebackField::bounded(qpo_symbol, qpo, Some(0.0), None),
                WritebackField::bounded(durrof_symbol, durrof, Some(0.0), None),
            ],
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
        let coef_a = Self::require_impoundment_coefficient_scalar(request, request.node_id, "a")?;
        let coef_b = Self::require_impoundment_coefficient_scalar(request, request.node_id, "b")?;
        let coef_c = Self::require_impoundment_coefficient_scalar(request, request.node_id, "c")?;
        let coef_d = Self::require_impoundment_coefficient_scalar(request, request.node_id, "d")?;
        let coef_e = Self::require_impoundment_coefficient_scalar(request, request.node_id, "e")?;
        let ha = Self::require_impoundment_coefficient_scalar(request, request.node_id, "ha")?;
        let ht = Self::require_impoundment_coefficient_scalar(request, request.node_id, "ht")?;
        let hlm = Self::require_impoundment_coefficient_scalar(request, request.node_id, "hlm")?;
        let a0 = Self::require_impoundment_coefficient_scalar(request, request.node_id, "a0")?;
        let a1 = Self::require_impoundment_coefficient_scalar(request, request.node_id, "a1")?;
        let a2 = Self::require_impoundment_coefficient_scalar(request, request.node_id, "a2")?;
        let _l0 = Self::require_impoundment_coefficient_scalar(request, request.node_id, "l0")?;
        let _l1 = Self::require_impoundment_coefficient_scalar(request, request.node_id, "l1")?;
        let _l2 = Self::require_impoundment_coefficient_scalar(request, request.node_id, "l2")?;

        let area = a0 + a1 * stage_h.powf(a2);
        if !area.is_finite() || area <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("area"),
                area,
            ));
        }

        let drop_spillway_q = if stage_h > ha {
            coef_a * (stage_h - ha).powf(coef_b)
        } else {
            0.0
        };
        let culvert_q = if stage_h > ht {
            coef_c * (stage_h - ht).powf(coef_d)
        } else {
            0.0
        };
        let riser_q = if stage_h > hlm {
            coef_e * (stage_h - hlm)
        } else {
            0.0
        };

        let qo = drop_spillway_q + culvert_q + riser_q;
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

        let continuity_outflow = qo + qinf;
        if !continuity_outflow.is_finite() || continuity_outflow < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("continuity_outflow"),
                continuity_outflow,
            ));
        }

        let hnext = stage_h + (deltat * (incoming_peak - continuity_outflow) / area);
        if !hnext.is_finite() {
            return Err(Self::non_finite(
                node_class,
                BoundarySymbol::from("hnext"),
                hnext,
            ));
        }
        if !(0.0..=hfull).contains(&hnext) {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("hnext"),
                hnext,
            ));
        }

        let durout = incoming_duration.max(deltat);
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
