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
const WS15_CRSH_FROM_CHNTCR_SCALE: f64 = 0.021;
const WS15_DEPTH_FROM_METERS_TO_FEET: f64 = 3.281;
const WS18_LBS_PER_KG: f64 = 2.2064;
const WS18_CFS_PER_CMS: f64 = 35.31984;
const WS18_MIN_CHANNEL_SLOPE: f64 = 0.00006;
const WS18_COVSH: f64 = 1000.0;
const WS18_AGRAV: f64 = 32.2;
const WS18_MSDH2O: f64 = 1.94;
const WS18_WTDH2O: f64 = 62.4;
const WS18_KNVIS: f64 = 1.05e-05;
const WS18_YALCON: f64 = 0.635;
const WS18_DEFAULT_CRSPG: [f64; 5] = [2.60, 2.65, 1.80, 1.60, 2.65];
const WS20_FALVEL_CDRE: [f64; 9] = [
    -3.0 * std::f64::consts::LN_10,
    -2.0 * std::f64::consts::LN_10,
    -std::f64::consts::LN_10,
    0.0,
    std::f64::consts::LN_10,
    2.0 * std::f64::consts::LN_10,
    3.0 * std::f64::consts::LN_10,
    4.0 * std::f64::consts::LN_10,
    5.0 * std::f64::consts::LN_10,
];
const WS20_FALVEL_CDRE2: [f64; 9] = [
    -4.50986, -1.51413, 0.78846, 3.12676, 6.04025, 9.30565, 13.08154, 17.50439, 22.29188,
];
const WS22_DCAP_WTDSOI: f64 = 96.0;
const WS22_DCAP_MIN_SLOPE: f64 = 0.00001;
const WS22_DCAP_MAXE: f64 = 1000.0;
const WS22_DCAP_XXCF: [f64; 17] = [
    0.0, 0.02, 0.04, 0.06, 0.08, 0.10, 0.12, 0.14, 0.16, 0.18, 0.20, 0.22, 0.24, 0.26, 0.28, 0.30,
    0.32,
];
const WS22_DCAP_FFXCF: [f64; 17] = [
    1000.0, 33.872, 12.571, 7.3030, 5.1102, 3.9575, 3.2659, 2.8419, 2.5040, 2.2818, 2.1194, 1.9997,
    1.9118, 1.8489, 1.8068, 1.7829, 1.7758,
];

const WS18_SHIELD_REYNOLDS: [f64; 8] = [1.0, 2.0, 4.0, 8.0, 12.0, 100.0, 400.0, 1000.0];
const WS18_SHIELD_VALUES: [f64; 8] = [0.0772, 0.0579, 0.04, 0.035, 0.034, 0.045, 0.055, 0.057];

const WS18_HYDCHN_XLC: [f64; 16] = [
    0.0, 0.01, 0.02, 0.04, 0.06, 0.08, 0.1, 0.12, 0.14, 0.16, 0.18, 0.2, 0.22, 0.24, 0.26, 0.28,
];
const WS18_HYDCHN_FGLC: [f64; 16] = [
    100_000.0, 32.91, 15.487, 7.307, 4.849, 3.713, 3.075, 2.676, 2.408, 2.222, 2.089, 1.994, 1.928,
    1.884, 1.858, 1.84866,
];
const WS18_HYDCHN_XXB: [f64; 27] = [
    0.0, 0.01, 0.02, 0.04, 0.06, 0.08, 0.1, 0.12, 0.14, 0.16, 0.18, 0.2, 0.22, 0.24, 0.26, 0.28,
    0.3, 0.32, 0.34, 0.36, 0.38, 0.4, 0.42, 0.44, 0.46, 0.48, 0.5,
];
const WS18_HYDCHN_FHXB: [f64; 27] = [
    0.0, 0.000_474, 0.00154, 0.00509, 0.0104, 0.0177, 0.0269, 0.0384, 0.0524, 0.0693, 0.0897,
    0.114, 0.1432, 0.1782, 0.2207, 0.2724, 0.3361, 0.4159, 0.5176, 0.6506, 0.8307, 1.0858, 1.4722,
    2.1212, 3.4264, 7.3566, 10000.0,
];

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

#[derive(Debug, Clone, Copy, PartialEq)]
struct Ws15ChannelSedimentControls {
    ishape: f64,
    ctlz: f64,
    chnz: f64,
    chnnbr: f64,
    chntcr: f64,
    chnedm: f64,
    chneds: f64,
}

#[derive(Debug, Clone, PartialEq)]
struct Ws18HillslopeSedimentPayload {
    mass_kg: f64,
    fractions: Vec<f64>,
    particle_diameters_m: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq)]
struct Ws19ChannelSedimentPublication {
    qsed: f64,
    tc: f64,
    particle_flow_fractions: Vec<f64>,
    particle_diameters_m: Vec<f64>,
    ws20_case1_segments: u32,
    ws20_case2_segments: u32,
    ws24_case2_detach_segments: u32,
    ws20_detachment_unmigrated_segments: u32,
    ws21_case3_segments: u32,
    ws21_case4_segments: u32,
    ws21_enddet_segments: u32,
    ws21_detach_unmigrated_segments: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Ws20IncomingPeakPartition {
    hillslope_peak_cms: f64,
    dependency_peak_cms: f64,
    duration_s: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[allow(clippy::struct_field_names)]
struct Ws20SegmentRoutingDiagnostics {
    case1_segments: u32,
    case2_segments: u32,
    ws24_case2_detach_segments: u32,
    detachment_unmigrated_segments: u32,
    case3_segments: u32,
    case4_segments: u32,
    enddet_segments: u32,
    ws21_detach_unmigrated_segments: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Ws27EnddetBracketProgress {
    used_xdbig_rebracket: bool,
    used_midpoint_rebracket: bool,
    iteration_count: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Ws15ChannelSedimentScaffold {
    chz: f64,
    nbarch: f64,
    crsh: f64,
    depmid: f64,
    depsid: f64,
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

    fn require_channel_control_scalar(
        request: &WatershedKernelRequest<'_>,
        node_id: u32,
        suffix: &'static str,
    ) -> Result<f64, Ws10GuardError> {
        let node_class = Ws10NodeClass::Channel;
        let key = BoundarySymbol::from(format!("ws10_channel_{node_id}_{suffix}"));
        let Some(value) = request.state_surface.get(&key) else {
            return Err(Self::missing_required(node_class, key));
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Self::non_finite(node_class, key, scalar));
        }
        Ok(scalar)
    }

    fn require_channel_state_symbol_scalar(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
        symbol: BoundarySymbol,
    ) -> Result<f64, Ws10GuardError> {
        let Some(value) = request.state_surface.get(&symbol) else {
            return Err(Self::missing_required(node_class, symbol));
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Self::non_finite(node_class, symbol, scalar));
        }
        Ok(scalar)
    }

    fn require_channel_control_range(
        node_class: Ws10NodeClass,
        symbol: BoundarySymbol,
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

    #[allow(clippy::similar_names)]
    fn read_ws15_channel_sediment_controls(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
    ) -> Result<Ws15ChannelSedimentControls, Ws10GuardError> {
        let node_id = request.node_id;

        let ishape_symbol = BoundarySymbol::from(format!("ws10_channel_{node_id}_ishape"));
        let ienslp_symbol = BoundarySymbol::from(format!("ws10_channel_{node_id}_ienslp"));
        let chnz_symbol = BoundarySymbol::from(format!("ws10_channel_{node_id}_chnz"));
        let chnnbr_symbol = BoundarySymbol::from(format!("ws10_channel_{node_id}_chnnbr"));
        let chntcr_symbol = BoundarySymbol::from(format!("ws10_channel_{node_id}_chntcr"));
        let chnedm_symbol = BoundarySymbol::from(format!("ws10_channel_{node_id}_chnedm"));
        let chneds_symbol = BoundarySymbol::from(format!("ws10_channel_{node_id}_chneds"));
        let ctlz_symbol = BoundarySymbol::from(format!("ws10_channel_{node_id}_ctlz"));
        let ctln_symbol = BoundarySymbol::from(format!("ws10_channel_{node_id}_ctln"));

        let ishape = Self::require_channel_control_scalar(request, node_id, "ishape")?;
        let ienslp = Self::require_channel_control_scalar(request, node_id, "ienslp")?;
        let chnz = Self::require_channel_control_scalar(request, node_id, "chnz")?;
        let chnnbr = Self::require_channel_control_scalar(request, node_id, "chnnbr")?;
        let chntcr = Self::require_channel_control_scalar(request, node_id, "chntcr")?;
        let chnedm = Self::require_channel_control_scalar(request, node_id, "chnedm")?;
        let chneds = Self::require_channel_control_scalar(request, node_id, "chneds")?;
        let ctlz = Self::require_channel_control_scalar(request, node_id, "ctlz")?;
        let ctln = Self::require_channel_control_scalar(request, node_id, "ctln")?;

        Self::require_channel_control_range(
            node_class,
            ishape_symbol,
            ishape,
            Some(1.0),
            Some(2.0),
        )?;
        Self::require_channel_control_range(
            node_class,
            ienslp_symbol,
            ienslp,
            Some(1.0),
            Some(2.0),
        )?;
        Self::require_channel_control_range(node_class, chnz_symbol, chnz, Some(0.0), None)?;
        Self::require_channel_control_range(
            node_class,
            chnnbr_symbol,
            chnnbr,
            Some(WS10_ZERO_THRESHOLD),
            None,
        )?;
        Self::require_channel_control_range(node_class, chntcr_symbol, chntcr, Some(0.0), None)?;
        Self::require_channel_control_range(node_class, chnedm_symbol, chnedm, Some(0.0), None)?;
        Self::require_channel_control_range(node_class, chneds_symbol, chneds, Some(0.0), None)?;
        Self::require_channel_control_range(
            node_class,
            ctlz_symbol,
            ctlz,
            Some(WS10_ZERO_THRESHOLD),
            None,
        )?;
        Self::require_channel_control_range(
            node_class,
            ctln_symbol,
            ctln,
            Some(WS10_ZERO_THRESHOLD),
            None,
        )?;

        let ishape_rounded = ishape.round();
        if (ishape - ishape_rounded).abs() > WS11_IPEAK_INTEGER_TOLERANCE {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from(format!("ws10_channel_{node_id}_ishape")),
                ishape,
            ));
        }

        let ienslp_rounded = ienslp.round();
        if (ienslp - ienslp_rounded).abs() > WS11_IPEAK_INTEGER_TOLERANCE {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from(format!("ws10_channel_{node_id}_ienslp")),
                ienslp,
            ));
        }

        Ok(Ws15ChannelSedimentControls {
            ishape,
            ctlz,
            chnz,
            chnnbr,
            chntcr,
            chnedm,
            chneds,
        })
    }

    #[allow(clippy::too_many_lines, clippy::similar_names)]
    fn require_ws17_channel_segment_scaffold(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
    ) -> Result<usize, Ws10GuardError> {
        let node_id = request.node_id;
        let nslpts_symbol = BoundarySymbol::from(format!("ws10_channel_{node_id}_nslpts"));
        let nslpts_raw =
            Self::require_channel_state_symbol_scalar(request, node_class, nslpts_symbol.clone())?;
        Self::require_channel_control_range(
            node_class,
            nslpts_symbol.clone(),
            nslpts_raw,
            Some(2.0),
            None,
        )?;

        let nslpts_rounded = nslpts_raw.round();
        if (nslpts_raw - nslpts_rounded).abs() > WS11_IPEAK_INTEGER_TOLERANCE {
            return Err(Self::domain_violation(
                node_class,
                nslpts_symbol,
                nslpts_raw,
            ));
        }
        if nslpts_rounded > f64::from(u32::MAX) {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from(format!("ws10_channel_{node_id}_nslpts")),
                nslpts_raw,
            ));
        }
        let nslpts_u32 = format!("{nslpts_rounded:.0}").parse::<u32>().map_err(|_| {
            Self::domain_violation(
                node_class,
                BoundarySymbol::from(format!("ws10_channel_{node_id}_nslpts")),
                nslpts_raw,
            )
        })?;
        let nslpts = usize::try_from(nslpts_u32).map_err(|_| {
            Self::domain_violation(
                node_class,
                BoundarySymbol::from(format!("ws10_channel_{node_id}_nslpts")),
                nslpts_raw,
            )
        })?;

        let mut previous_x: Option<f64> = None;
        for point_number in 1..=nslpts {
            let x_symbol =
                BoundarySymbol::from(format!("ws10_channel_{node_id}_x_{point_number:04}"));
            let slope_symbol =
                BoundarySymbol::from(format!("ws10_channel_{node_id}_slope_{point_number:04}"));
            let depth_a_symbol =
                BoundarySymbol::from(format!("ws10_channel_{node_id}_depa_{point_number:04}"));
            let depth_b_symbol =
                BoundarySymbol::from(format!("ws10_channel_{node_id}_depb_{point_number:04}"));
            let width_a_symbol =
                BoundarySymbol::from(format!("ws10_channel_{node_id}_wida_{point_number:04}"));
            let width_b_symbol =
                BoundarySymbol::from(format!("ws10_channel_{node_id}_widb_{point_number:04}"));

            let x =
                Self::require_channel_state_symbol_scalar(request, node_class, x_symbol.clone())?;
            let slope = Self::require_channel_state_symbol_scalar(
                request,
                node_class,
                slope_symbol.clone(),
            )?;
            let depth_a = Self::require_channel_state_symbol_scalar(
                request,
                node_class,
                depth_a_symbol.clone(),
            )?;
            let depth_b = Self::require_channel_state_symbol_scalar(
                request,
                node_class,
                depth_b_symbol.clone(),
            )?;
            let width_a = Self::require_channel_state_symbol_scalar(
                request,
                node_class,
                width_a_symbol.clone(),
            )?;
            let width_b = Self::require_channel_state_symbol_scalar(
                request,
                node_class,
                width_b_symbol.clone(),
            )?;

            Self::require_channel_control_range(node_class, x_symbol.clone(), x, Some(0.0), None)?;
            if let Some(previous) = previous_x
                && x + WS10_ZERO_THRESHOLD < previous
            {
                return Err(Self::domain_violation(node_class, x_symbol, x));
            }
            Self::require_channel_control_range(node_class, slope_symbol, slope, Some(0.0), None)?;
            Self::require_channel_control_range(
                node_class,
                depth_a_symbol,
                depth_a,
                Some(0.0),
                None,
            )?;
            Self::require_channel_control_range(
                node_class,
                depth_b_symbol,
                depth_b,
                Some(0.0),
                None,
            )?;
            Self::require_channel_control_range(
                node_class,
                width_a_symbol,
                width_a,
                Some(WS10_ZERO_THRESHOLD),
                None,
            )?;
            Self::require_channel_control_range(
                node_class,
                width_b_symbol,
                width_b,
                Some(WS10_ZERO_THRESHOLD),
                None,
            )?;

            previous_x = Some(x);
        }

        Ok(nslpts)
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

    #[allow(clippy::too_many_lines)]
    fn read_hillslope_sediment_payload(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
        hillslope_id: u32,
    ) -> Result<Ws18HillslopeSedimentPayload, Ws10GuardError> {
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

        let mut fractions = Vec::with_capacity(class_count);
        let mut particle_diameters_m = Vec::with_capacity(class_count);
        let mut fraction_sum = 0.0_f64;

        for class_index in 1..=class_count {
            let concentration_symbol =
                WatershedProductionStateSymbol::HillslopeContributorSedimentConcentrationKgM3 {
                    hillslope_id,
                    class_index,
                };
            let particle_diameter_symbol =
                WatershedProductionStateSymbol::HillslopeContributorParticleDiameterMeters {
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
            let particle_diameter =
                Self::require_state_scalar(request, node_class, particle_diameter_symbol)?;
            let fraction = Self::require_state_scalar(request, node_class, fraction_symbol)?;

            Self::require_state_range(
                node_class,
                concentration_symbol,
                concentration,
                Some(0.0),
                None,
            )?;
            Self::require_state_range(
                node_class,
                particle_diameter_symbol,
                particle_diameter,
                Some(WS10_ZERO_THRESHOLD),
                None,
            )?;
            Self::require_state_range(node_class, fraction_symbol, fraction, Some(0.0), Some(1.0))?;
            fractions.push(fraction);
            particle_diameters_m.push(particle_diameter);
            fraction_sum += fraction;
        }

        if fraction_sum <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                class_count_symbol,
                class_count_value,
            ));
        }

        Ok(Ws18HillslopeSedimentPayload {
            mass_kg: (total_detachment - total_deposition).max(0.0),
            fractions,
            particle_diameters_m,
        })
    }

    #[allow(clippy::too_many_lines)]
    fn read_channel_sediment_payload(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
        channel_id: u32,
        event_duration: f64,
    ) -> Result<Ws18HillslopeSedimentPayload, Ws10GuardError> {
        let qsed_symbol = Self::channel_wave_state_symbol(channel_id, "qsed");
        let qsed =
            Self::require_channel_state_symbol_scalar(request, node_class, qsed_symbol.clone())?;
        Self::require_channel_control_range(node_class, qsed_symbol, qsed, Some(0.0), None)?;

        if qsed <= WS10_ZERO_THRESHOLD {
            return Ok(Ws18HillslopeSedimentPayload {
                mass_kg: 0.0,
                fractions: Vec::new(),
                particle_diameters_m: Vec::new(),
            });
        }

        let mass_kg = qsed * event_duration;
        if !mass_kg.is_finite() || mass_kg < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from(format!("ws10_channel_{channel_id}_incoming_mass_kg")),
                mass_kg,
            ));
        }

        let class_count_symbol =
            Self::channel_wave_state_symbol(channel_id, "particle_class_count");
        let class_count_value = Self::require_channel_state_symbol_scalar(
            request,
            node_class,
            class_count_symbol.clone(),
        )?;
        Self::require_channel_control_range(
            node_class,
            class_count_symbol.clone(),
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
                Self::domain_violation(node_class, class_count_symbol.clone(), class_count_value)
            })?;
        if class_count == 0 {
            return Err(Self::domain_violation(
                node_class,
                class_count_symbol,
                class_count_value,
            ));
        }

        let mut fractions = Vec::with_capacity(class_count);
        let mut particle_diameters_m = Vec::with_capacity(class_count);
        let mut fraction_sum = 0.0_f64;
        for class_index in 1..=class_count {
            let fraction_symbol = Self::channel_wave_state_symbol(
                channel_id,
                &format!("particle_flow_fraction_{class_index:04}"),
            );
            let particle_diameter_symbol = Self::channel_wave_state_symbol(
                channel_id,
                &format!("particle_diameter_m_{class_index:04}"),
            );

            let fraction = Self::require_channel_state_symbol_scalar(
                request,
                node_class,
                fraction_symbol.clone(),
            )?;
            let particle_diameter = Self::require_channel_state_symbol_scalar(
                request,
                node_class,
                particle_diameter_symbol.clone(),
            )?;

            Self::require_channel_control_range(
                node_class,
                fraction_symbol,
                fraction,
                Some(0.0),
                Some(1.0),
            )?;
            Self::require_channel_control_range(
                node_class,
                particle_diameter_symbol,
                particle_diameter,
                Some(WS10_ZERO_THRESHOLD),
                None,
            )?;

            fractions.push(fraction);
            particle_diameters_m.push(particle_diameter);
            fraction_sum += fraction;
        }

        if fraction_sum <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                Self::channel_wave_state_symbol(channel_id, "particle_flow_fraction_sum"),
                fraction_sum,
            ));
        }

        Ok(Ws18HillslopeSedimentPayload {
            mass_kg,
            fractions,
            particle_diameters_m,
        })
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

    fn assemble_incoming_peak_partition(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
    ) -> Result<Ws20IncomingPeakPartition, Ws10GuardError> {
        let mut hillslope_peak = 0.0_f64;
        let mut dependency_peak = 0.0_f64;
        let mut incoming_duration = 0.0_f64;

        for &hillslope_id in request.contributor_hillslopes {
            let (peak, duration) =
                Self::read_hillslope_peak_payload(request, node_class, hillslope_id)?;
            let _ = Self::read_hillslope_sediment_payload(request, node_class, hillslope_id)?;
            hillslope_peak += peak;
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
            dependency_peak += peak;
            incoming_duration = incoming_duration.max(duration);
        }

        let incoming_peak = hillslope_peak + dependency_peak;

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

        Ok(Ws20IncomingPeakPartition {
            hillslope_peak_cms: hillslope_peak,
            dependency_peak_cms: dependency_peak,
            duration_s: incoming_duration,
        })
    }

    fn assemble_incoming_peak_and_duration(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
    ) -> Result<(f64, f64), Ws10GuardError> {
        let partition = Self::assemble_incoming_peak_partition(request, node_class)?;
        Ok((
            partition.hillslope_peak_cms + partition.dependency_peak_cms,
            partition.duration_s,
        ))
    }

    #[allow(clippy::similar_names)]
    fn derive_ws15_channel_sediment_scaffold(
        node_class: Ws10NodeClass,
        node_id: u32,
        controls: Ws15ChannelSedimentControls,
    ) -> Result<Ws15ChannelSedimentScaffold, Ws10GuardError> {
        let crsh = controls.chntcr * WS15_CRSH_FROM_CHNTCR_SCALE;
        let depmid = controls.chnedm * WS15_DEPTH_FROM_METERS_TO_FEET;
        let depsid = controls.chneds * WS15_DEPTH_FROM_METERS_TO_FEET;

        for (suffix, value) in [
            ("chz", controls.chnz),
            ("nbarch", controls.chnnbr),
            ("crsh", crsh),
            ("depmid", depmid),
            ("depsid", depsid),
        ] {
            if !value.is_finite() {
                return Err(Self::non_finite(
                    node_class,
                    BoundarySymbol::from(format!("ws10_channel_{node_id}_{suffix}")),
                    value,
                ));
            }
            if value < 0.0 {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from(format!("ws10_channel_{node_id}_{suffix}")),
                    value,
                ));
            }
        }

        Ok(Ws15ChannelSedimentScaffold {
            chz: controls.chnz,
            nbarch: controls.chnnbr,
            crsh,
            depmid,
            depsid,
        })
    }

    fn ws18_linear_interpolate(x1: f64, y1: f64, x2: f64, y2: f64, x: f64) -> f64 {
        let denominator = x2 - x1;
        if denominator.abs() <= WS10_ZERO_THRESHOLD {
            0.5 * (y1 + y2)
        } else {
            y1 + ((y2 - y1) * (x - x1) / denominator)
        }
    }

    fn ws18_inverse_interpolate(
        xs: &[f64],
        ys: &[f64],
        given: f64,
        increasing: bool,
    ) -> Option<f64> {
        if xs.len() != ys.len() || xs.len() < 2 {
            return None;
        }

        for index in 1..xs.len() {
            let y0 = ys[index - 1];
            let y1 = ys[index];
            let in_range = if increasing {
                given >= y0 && given <= y1
            } else {
                given <= y0 && given >= y1
            };
            if in_range {
                return Some(Self::ws18_linear_interpolate(
                    y0,
                    xs[index - 1],
                    y1,
                    xs[index],
                    given,
                ));
            }
        }

        None
    }

    fn ws18_shield_parameter(reyn: f64) -> f64 {
        if reyn <= WS10_ZERO_THRESHOLD {
            return WS18_SHIELD_VALUES[0];
        }

        let reynolds = reyn.ln();
        if reyn < WS18_SHIELD_REYNOLDS[0] {
            let i = 1;
            let slope = (WS18_SHIELD_VALUES[i].ln() - WS18_SHIELD_VALUES[i - 1].ln())
                / (WS18_SHIELD_REYNOLDS[i].ln() - WS18_SHIELD_REYNOLDS[i - 1].ln());
            let ycr =
                WS18_SHIELD_VALUES[0].ln() - (slope * (WS18_SHIELD_REYNOLDS[0].ln() - reynolds));
            return ycr.exp();
        }

        if reyn > WS18_SHIELD_REYNOLDS[WS18_SHIELD_REYNOLDS.len() - 1] {
            let i = WS18_SHIELD_REYNOLDS.len() - 1;
            let slope = (WS18_SHIELD_VALUES[i].ln() - WS18_SHIELD_VALUES[i - 1].ln())
                / (WS18_SHIELD_REYNOLDS[i].ln() - WS18_SHIELD_REYNOLDS[i - 1].ln());
            let ycr = WS18_SHIELD_VALUES[i] + (slope * (reynolds - WS18_SHIELD_REYNOLDS[i].ln()));
            return ycr.exp();
        }

        for i in 1..WS18_SHIELD_REYNOLDS.len() {
            if reyn >= WS18_SHIELD_REYNOLDS[i - 1] && reyn <= WS18_SHIELD_REYNOLDS[i] {
                let slope = (WS18_SHIELD_VALUES[i].ln() - WS18_SHIELD_VALUES[i - 1].ln())
                    / (WS18_SHIELD_REYNOLDS[i].ln() - WS18_SHIELD_REYNOLDS[i - 1].ln());
                let ycr = WS18_SHIELD_VALUES[i - 1].ln()
                    + (slope * (reynolds - WS18_SHIELD_REYNOLDS[i - 1].ln()));
                return ycr.exp();
            }
        }

        WS18_SHIELD_VALUES[WS18_SHIELD_VALUES.len() - 1]
    }

    #[allow(
        clippy::many_single_char_names,
        clippy::too_many_arguments,
        clippy::too_many_lines
    )]
    fn ws18_hydchn(
        node_class: Ws10NodeClass,
        flagc: i32,
        q_cfs: f64,
        sf: f64,
        c1: f64,
        z: f64,
        wb: f64,
        n: f64,
        crsh: f64,
        nbarch: f64,
    ) -> Result<(f64, f64), Ws10GuardError> {
        if q_cfs <= WS10_ZERO_THRESHOLD {
            return Ok((0.0, 0.0));
        }
        if sf <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws18_hydchn_sf"),
                sf,
            ));
        }
        if n <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws18_hydchn_n"),
                n,
            ));
        }
        if nbarch <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws18_hydchn_nbarch"),
                nbarch,
            ));
        }
        if crsh <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws18_hydchn_crsh"),
                crsh,
            ));
        }

        let mut flag = flagc;
        let mut n_total = n;
        for _ in 0..8 {
            let ap = (q_cfs * n_total / (1.49 * sf.sqrt())).powf(0.375);
            let (w, a, nt) = if flag == 2 {
                if wb <= WS10_ZERO_THRESHOLD {
                    if c1 <= WS10_ZERO_THRESHOLD {
                        return Err(Self::domain_violation(
                            node_class,
                            BoundarySymbol::from("ws18_hydchn_c1"),
                            c1,
                        ));
                    }
                    let y = ap / c1.powf(0.375);
                    let w = 2.0 * y * z;
                    let a = z * y * y;
                    (w, a, n_total)
                } else {
                    let w = wb;
                    let hxb = (ap / w).powf(8.0 / 3.0);
                    let xb = if hxb <= 0.114 {
                        let mut xbo = 0.2_f64;
                        let mut xbn = xbo;
                        for _ in 0..32 {
                            let core = ((1.0 - (2.0 * xbo)) * hxb).max(0.0);
                            xbn = core.powf(0.6);
                            if xbn.abs() <= WS10_ZERO_THRESHOLD {
                                xbn = 1.0e-10;
                            }
                            let dif = ((xbn - xbo) / xbn).abs();
                            if dif <= 0.001 {
                                break;
                            }
                            xbo = xbn;
                        }
                        xbn
                    } else {
                        Self::ws18_inverse_interpolate(
                            &WS18_HYDCHN_XXB,
                            &WS18_HYDCHN_FHXB,
                            hxb.min(9999.99),
                            true,
                        )
                        .unwrap_or(WS18_HYDCHN_XXB[WS18_HYDCHN_XXB.len() - 1])
                    };
                    let denominator = (1.0 - (2.0 * xb)).max(WS10_ZERO_THRESHOLD);
                    let y = w * xb / denominator;
                    let a = y * w;
                    (w, a, n_total)
                }
            } else if flag >= 3 {
                let ap_natural = (q_cfs * nbarch / (1.49 * sf.sqrt())).powf(0.375);
                let glc = ap_natural * WS18_WTDH2O * sf / crsh;
                if glc <= 1.84866 {
                    if wb <= WS10_ZERO_THRESHOLD {
                        flag = 1;
                        continue;
                    }
                    flag = 2;
                    continue;
                }
                let lc = Self::ws18_inverse_interpolate(
                    &WS18_HYDCHN_XLC,
                    &WS18_HYDCHN_FGLC,
                    glc.min(99_999.999),
                    false,
                )
                .unwrap_or(WS18_HYDCHN_XLC[WS18_HYDCHN_XLC.len() - 1]);
                let rstar = (-0.34707 * (0.5 - lc).powi(3)) - (0.54213 * (0.5 - lc).powi(2))
                    + (0.66383 * (0.5 - lc));
                if rstar <= WS10_ZERO_THRESHOLD {
                    return Err(Self::domain_violation(
                        node_class,
                        BoundarySymbol::from("ws18_hydchn_rstar"),
                        rstar,
                    ));
                }
                let w = (ap_natural / rstar.powf(0.625)) * (0.73 - (1.46 * lc));
                if w <= WS10_ZERO_THRESHOLD {
                    return Err(Self::domain_violation(
                        node_class,
                        BoundarySymbol::from("ws18_hydchn_w"),
                        w,
                    ));
                }
                let hxb = (ap_natural / w).powf(8.0 / 3.0);
                let xb = Self::ws18_inverse_interpolate(
                    &WS18_HYDCHN_XXB,
                    &WS18_HYDCHN_FHXB,
                    hxb.min(9999.99),
                    true,
                )
                .unwrap_or(WS18_HYDCHN_XXB[WS18_HYDCHN_XXB.len() - 1]);
                let denominator = (1.0 - (2.0 * xb)).max(WS10_ZERO_THRESHOLD);
                let y = w * xb / denominator;
                let a = y * w;
                (w, a, nbarch)
            } else {
                if c1 <= WS10_ZERO_THRESHOLD {
                    return Err(Self::domain_violation(
                        node_class,
                        BoundarySymbol::from("ws18_hydchn_c1"),
                        c1,
                    ));
                }
                let y = ap / c1.powf(0.375);
                let w = 2.0 * y * z;
                let a = z * y * y;
                (w, a, n_total)
            };

            let wetted_area = a.max(1.0e-10);
            let velocity = q_cfs / wetted_area;
            let rsh = (velocity * nbarch / (1.49 * sf.sqrt())).powf(1.5);
            let rcov = (velocity * (nt - nbarch) / (1.49 * sf.sqrt())).powf(1.5);
            let effsh = WS18_WTDH2O * rsh * sf;
            let mulsh = WS18_WTDH2O * rcov * sf;
            if mulsh < WS18_COVSH {
                return Ok((w, effsh.max(0.0)));
            }

            n_total = nbarch;
        }

        Err(Self::domain_violation(
            node_class,
            BoundarySymbol::from("ws18_hydchn_iteration_limit"),
            f64::from(flag),
        ))
    }

    #[allow(clippy::similar_names, clippy::too_many_lines)]
    fn ws18_trncap(effsh: f64, qs: &[f64], crdia_ft: &[f64], crspg: &[f64]) -> Vec<f64> {
        let class_count = qs.len();
        if class_count == 0 || effsh <= 0.0 {
            return vec![0.0; class_count];
        }

        let vstar = (effsh / WS18_MSDH2O).sqrt();
        let coef_base = vstar * WS18_AGRAV * WS18_MSDH2O;

        let mut coef = vec![0.0_f64; class_count];
        let mut delta = vec![0.0_f64; class_count];
        let mut p = vec![0.0_f64; class_count];
        let mut dltrat = vec![0.0_f64; class_count];
        let mut ws = vec![0.0_f64; class_count];
        let mut qs_local = vec![0.0_f64; class_count];

        for k in 0..class_count {
            coef[k] = coef_base * crdia_ft[k] * crspg[k];
            qs_local[k] = qs[k].max(1.0e-31);
        }

        let mut t = 0.0_f64;
        for k in 0..class_count {
            let reyn = vstar * crdia_ft[k] / WS18_KNVIS;
            let ycrit = Self::ws18_shield_parameter(reyn.max(1.0e-12));
            let mut delta_k =
                (vstar * vstar / ((crspg[k] - 1.0) * WS18_AGRAV * crdia_ft[k] * ycrit)) - 1.0;
            if delta_k <= 0.0 || !delta_k.is_finite() {
                delta_k = 0.0;
                p[k] = 0.0;
            } else {
                let sigma = delta_k * 2.45 * crspg[k].powf(-0.4) * ycrit.sqrt();
                if sigma <= WS10_ZERO_THRESHOLD {
                    p[k] = 0.0;
                } else {
                    p[k] = WS18_YALCON * delta_k * (1.0 - ((1.0 / sigma) * (1.0 + sigma).ln()));
                }
            }
            delta[k] = delta_k;
            t += delta_k;
        }

        if t == 0.0 {
            t = 1000.0;
        }

        for k in 0..class_count {
            dltrat[k] = delta[k] / t;
            ws[k] = p[k] * dltrat[k] * coef[k];
        }

        let mut mycount = 0_u32;
        loop {
            let mut flagd1 = 0_usize;
            let mut flagd2 = 0_usize;
            let mut flagd3 = 0_usize;
            let mut wsqrat = vec![0.0_f64; class_count];

            for k in 0..class_count {
                if qs_local[k] > 0.0 {
                    wsqrat[k] = ws[k] / qs_local[k];
                    if wsqrat[k] > 1.0 {
                        flagd3 += 1;
                    }
                    if wsqrat[k] >= 1.0 {
                        flagd1 += 1;
                    }
                    if wsqrat[k] <= 1.0 {
                        flagd2 += 1;
                    }
                }
            }

            if flagd2 == class_count || flagd3 == class_count {
                return ws;
            }

            if flagd3 != class_count {
                mycount += 1;
                if mycount > 20 || flagd1 == class_count {
                    let mut smdrat = 0.0_f64;
                    for k in 0..class_count {
                        let denominator = coef[k] * p[k];
                        if denominator > WS10_ZERO_THRESHOLD {
                            smdrat += qs_local[k] / denominator;
                        }
                    }
                    let a = if smdrat > WS10_ZERO_THRESHOLD {
                        let mut scale = 1.0 / smdrat;
                        if scale > 0.999_99 && scale < 1.000_009_9 {
                            scale = 1.0;
                        }
                        scale
                    } else {
                        1.0
                    };

                    return qs_local.iter().map(|value| a * value).collect();
                }
            }

            let mut smdrqt = 0.0_f64;
            let mut smdrat = 0.0_f64;

            for k in 0..class_count {
                let ratio = if qs_local[k] > 0.0 {
                    ws[k] / qs_local[k]
                } else {
                    0.0
                };
                if ratio >= 1.0 {
                    let denominator = coef[k] * p[k];
                    if denominator > WS10_ZERO_THRESHOLD {
                        smdrqt += qs_local[k] / denominator;
                    }
                    ws[k] = qs_local[k];
                } else {
                    smdrat += dltrat[k];
                }
            }

            let excap = 1.0 - smdrqt;
            let smdrat_guard = if smdrat.abs() <= WS10_ZERO_THRESHOLD {
                1_000_000.0
            } else {
                smdrat
            };
            for k in 0..class_count {
                let ratio = if qs_local[k] > 0.0 {
                    ws[k] / qs_local[k]
                } else {
                    0.0
                };
                if ratio < 1.0 {
                    ws[k] = dltrat[k] / smdrat_guard * excap * p[k] * coef[k];
                }
            }
        }
    }

    fn ws20_fall_velocity_ft_s(specific_gravity: f64, particle_diameter_ft: f64) -> f64 {
        if particle_diameter_ft <= WS10_ZERO_THRESHOLD {
            return 0.0;
        }

        let rtsid = ((specific_gravity - 1.0) * WS18_AGRAV * particle_diameter_ft.powi(3)
            / WS18_KNVIS.powi(2))
            * (8.0 / 6.0);
        if rtsid >= 0.024 {
            let rtsid_ln = rtsid.ln();
            for index in 1..WS20_FALVEL_CDRE2.len() {
                if WS20_FALVEL_CDRE2[index] > rtsid_ln {
                    let x0 = WS20_FALVEL_CDRE2[index - 1];
                    let x1 = WS20_FALVEL_CDRE2[index];
                    let y0 = WS20_FALVEL_CDRE[index - 1];
                    let y1 = WS20_FALVEL_CDRE[index];
                    let reynolds_log = y0 + (((rtsid_ln - x0) / (x1 - x0)) * (y1 - y0));
                    return reynolds_log.exp() * WS18_KNVIS / particle_diameter_ft;
                }
            }

            return WS20_FALVEL_CDRE[WS20_FALVEL_CDRE.len() - 1].exp() * WS18_KNVIS
                / particle_diameter_ft;
        }

        (particle_diameter_ft.powi(2) * (specific_gravity - 1.0) * WS18_AGRAV) / (WS18_KNVIS * 18.0)
    }

    fn ws22_require_crfrac_vector(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
        class_numbers: &[usize],
    ) -> Result<Vec<f64>, Ws10GuardError> {
        let mut crfrac = Vec::with_capacity(class_numbers.len());
        for class_number in class_numbers {
            let symbol = BoundarySymbol::from(format!(
                "ws10_channel_{}_crfrac_{:04}",
                request.node_id, class_number
            ));
            let value =
                Self::require_channel_state_symbol_scalar(request, node_class, symbol.clone())?;
            Self::require_channel_control_range(node_class, symbol, value, Some(0.0), Some(1.0))?;
            crfrac.push(value);
        }

        let sum = crfrac.iter().copied().sum::<f64>();
        if !sum.is_finite() || sum <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from(format!("ws10_channel_{}_crfrac_sum", request.node_id)),
                sum,
            ));
        }
        for value in &mut crfrac {
            *value /= sum;
        }
        Ok(crfrac)
    }

    fn ws22_table_column2_to_column1(
        col1: &[f64],
        col2: &[f64],
        given: f64,
        column2_increasing: bool,
    ) -> Option<f64> {
        if col1.len() != col2.len() || col1.len() < 2 {
            return None;
        }

        for index in 1..col1.len() {
            let left = col2[index - 1];
            let right = col2[index];
            let in_range = if column2_increasing {
                given >= left && given <= right
            } else {
                given <= left && given >= right
            };
            if in_range {
                return Some(Self::ws18_linear_interpolate(
                    left,
                    col1[index - 1],
                    right,
                    col1[index],
                    given,
                ));
            }
        }

        None
    }

    fn ws22_shdist(x: f64) -> f64 {
        if x >= 0.02 {
            return (0.12692
                - (0.51634 * x.ln())
                - (0.40825 * x.ln().powi(2))
                - (0.03442 * x.ln().powi(3)))
            .exp();
        }
        0.13 * x / 0.02
    }

    #[allow(
        clippy::too_many_arguments,
        clippy::too_many_lines,
        clippy::similar_names
    )]
    fn ws26_dcap(
        node_class: Ws10NodeClass,
        flagm: i32,
        q_cfs: f64,
        sf: f64,
        c1: f64,
        z: f64,
        effsh: f64,
        depsid: f64,
        depmid_input: f64,
        werod_input: f64,
        wflow: f64,
        roughness: f64,
        crsh: f64,
        excess: f64,
        tb: f64,
        flagt: i32,
        chnk: f64,
        nbarch: f64,
        maxe: f64,
        crfrac: &[f64],
    ) -> Result<Vec<f64>, Ws10GuardError> {
        let mut df = vec![0.0; crfrac.len()];
        let mut depmid = depmid_input;
        let mut werod = werod_input;
        if effsh <= crsh {
            return Ok(df);
        }

        let mut timpot = 0.0_f64;
        let mut timsh = tb * (1.0 - (crsh / effsh));
        let mut di = 0.0_f64;

        if depmid > WS10_ZERO_THRESHOLD {
            if flagt == 3 {
                werod = wflow;
            } else {
                let (wtmp, _) = Self::ws18_hydchn(
                    node_class,
                    4,
                    q_cfs,
                    sf.max(WS22_DCAP_MIN_SLOPE),
                    c1,
                    z,
                    wflow,
                    roughness,
                    crsh,
                    nbarch,
                )?;
                werod = wtmp;
            }

            let difsh = effsh - crsh;
            if difsh <= 0.0 {
                return Ok(df);
            }

            di = excess * chnk * difsh;
            if di <= WS10_ZERO_THRESHOLD {
                return Ok(df);
            }

            timpot = depmid * WS22_DCAP_WTDSOI / di;
            if timpot >= timsh {
                let mut dct = di * timsh * werod / (tb * wflow);
                if flagm != 1 && dct >= maxe {
                    di *= maxe / dct;
                    dct = maxe;
                }
                for class_offset in 0..crfrac.len() {
                    df[class_offset] = dct * crfrac[class_offset];
                }
                depmid -= di * timsh / WS22_DCAP_WTDSOI;
                if depmid < 0.005 {
                    depmid = 0.0;
                }
                let _ = depmid;
                return Ok(df);
            }
        }

        let timex = timsh - timpot;
        let ab = q_cfs * roughness / (1.49 * sf.max(WS22_DCAP_MIN_SLOPE).sqrt());

        if werod <= WS10_ZERO_THRESHOLD {
            let (wtmp, _) = Self::ws18_hydchn(
                node_class,
                4,
                q_cfs,
                sf.max(WS22_DCAP_MIN_SLOPE),
                c1,
                z,
                wflow,
                roughness,
                crsh,
                nbarch,
            )?;
            werod = wtmp;
        }

        let hxb = ab / werod.powf(8.0 / 3.0);
        let Some(xb) = Self::ws22_table_column2_to_column1(
            &WS18_HYDCHN_XXB,
            &WS18_HYDCHN_FHXB,
            hxb.min(9999.99),
            true,
        ) else {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws22_dcap_hxb"),
                hxb,
            ));
        };

        let difsh = effsh * Self::ws22_shdist(xb) - crsh;
        if difsh <= 0.0 {
            if depmid <= 0.0 {
                return Ok(df);
            }
            timsh = timpot;
            if di <= WS10_ZERO_THRESHOLD {
                return Ok(df);
            }
            let mut dct = di * timsh * werod / (tb * wflow);
            if flagm != 1 && dct >= maxe {
                dct = maxe;
            }
            for class_offset in 0..crfrac.len() {
                df[class_offset] = dct * crfrac[class_offset];
            }
            return Ok(df);
        }

        let dwdti = excess * 2.0 * chnk * difsh / WS22_DCAP_WTDSOI;
        let ad = ab.powf(0.375) * WS18_WTDH2O * sf.max(WS22_DCAP_MIN_SLOPE) / crsh;
        if ad <= WS22_DCAP_FFXCF[WS22_DCAP_FFXCF.len() - 1] {
            return Ok(df);
        }

        let Some(xcf) = Self::ws22_table_column2_to_column1(
            &WS22_DCAP_XXCF,
            &WS22_DCAP_FFXCF,
            ad.min(999.999),
            false,
        ) else {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws22_dcap_ad"),
                ad,
            ));
        };

        if xcf <= WS10_ZERO_THRESHOLD || (1.0 - (2.0 * xcf)) <= WS10_ZERO_THRESHOLD {
            return Ok(df);
        }
        let wfin_core = xcf * (1.0 - (2.0 * xcf)) / xcf.powf(8.0 / 3.0);
        if !wfin_core.is_finite() || wfin_core <= WS10_ZERO_THRESHOLD {
            return Ok(df);
        }
        let wfin = ab.powf(0.375) * wfin_core.powf(0.375);
        if wfin <= werod {
            return Ok(df);
        }

        let tstar = timex * dwdti / (wfin - werod);
        let wstar = (1.0 - (-1.0176 * tstar).exp()) / 1.0176;
        let we = wstar * (wfin - werod) + werod;
        let mut eros = (we - werod) * depsid + depmid * werod;
        let mut dct = eros * WS22_DCAP_WTDSOI / (tb * wflow);
        if flagm != 1 && dct >= maxe {
            dct = maxe;
            eros = dct * tb * wflow / WS22_DCAP_WTDSOI;
            let _ = eros;
        }

        for class_offset in 0..crfrac.len() {
            df[class_offset] = dct * crfrac[class_offset];
        }
        Ok(df)
    }

    #[allow(
        clippy::too_many_arguments,
        clippy::many_single_char_names,
        clippy::too_many_lines,
        clippy::similar_names
    )]
    fn ws23_detach_case4_iterative_closure(
        node_class: Ws10NodeClass,
        ql_cfs: f64,
        sfl: f64,
        c1: f64,
        z: f64,
        effshl: f64,
        depsid_ft: f64,
        depmid_ft: f64,
        wfl_ft: f64,
        roughness: f64,
        crsh: f64,
        tb_s: f64,
        flagc: i32,
        chnk: f64,
        nbarch: f64,
        crfrac: &[f64],
        gstu_lbs_s: &[f64],
        dlat_lbs_s_ft: &[f64],
        du_lbs_s_ft: &[f64],
        dx_ft: f64,
        crdia_ft: &[f64],
        crspg: &[f64],
    ) -> Result<Vec<f64>, Ws10GuardError> {
        let class_count = gstu_lbs_s.len();
        if class_count == 0
            || dlat_lbs_s_ft.len() != class_count
            || du_lbs_s_ft.len() != class_count
            || crdia_ft.len() != class_count
            || crspg.len() != class_count
            || crfrac.len() != class_count
        {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws23_detach_class_cardinality"),
                f64::from(u32::try_from(class_count).unwrap_or(u32::MAX)),
            ));
        }

        if dx_ft <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws23_detach_dx_ft"),
                dx_ft,
            ));
        }

        let mut excess = 1.0_f64;
        let mut df_lbs_s_ft2 = Self::ws26_dcap(
            node_class,
            1,
            ql_cfs,
            sfl,
            c1,
            z,
            effshl,
            depsid_ft,
            depmid_ft,
            wfl_ft,
            wfl_ft,
            roughness,
            crsh,
            excess,
            tb_s,
            flagc,
            chnk,
            nbarch,
            WS22_DCAP_MAXE,
            crfrac,
        )?;

        let mut dl_lbs_s_ft = vec![0.0_f64; class_count];
        let mut potld_lbs_s_ft = vec![0.0_f64; class_count];
        let mut nt3 = 0_usize;
        for class_offset in 0..class_count {
            dl_lbs_s_ft[class_offset] = df_lbs_s_ft2[class_offset] * wfl_ft;
            potld_lbs_s_ft[class_offset] = (gstu_lbs_s[class_offset]
                + (dlat_lbs_s_ft[class_offset] * dx_ft)
                + ((dl_lbs_s_ft[class_offset] + du_lbs_s_ft[class_offset]) * dx_ft / 2.0))
                / wfl_ft;
            if dl_lbs_s_ft[class_offset].abs() <= WS10_ZERO_THRESHOLD
                && potld_lbs_s_ft[class_offset].abs() <= WS10_ZERO_THRESHOLD
            {
                nt3 += 1;
            }
        }

        let mut tcl_lbs_s_ft = vec![0.0_f64; class_count];
        if nt3 < class_count {
            tcl_lbs_s_ft = Self::ws18_trncap(effshl, &potld_lbs_s_ft, crdia_ft, crspg);
        }

        let nt2 = tcl_lbs_s_ft
            .iter()
            .zip(&potld_lbs_s_ft)
            .filter(|(tcl, potld)| **tcl >= **potld)
            .count();
        if nt2 == class_count || nt3 == class_count {
            let mut next_gstu_lbs_s = vec![0.0_f64; class_count];
            for class_offset in 0..class_count {
                next_gstu_lbs_s[class_offset] = potld_lbs_s_ft[class_offset] * wfl_ft;
            }
            return Ok(next_gstu_lbs_s);
        }

        let mut sumtcl = tcl_lbs_s_ft.iter().sum::<f64>();
        let mut sumpld = potld_lbs_s_ft.iter().sum::<f64>();
        if !sumtcl.is_finite() || !sumpld.is_finite() || sumpld.abs() <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws23_detach_sumpld"),
                sumpld,
            ));
        }
        excess = sumtcl / sumpld;
        let mut excold = excess;

        for _ in 0..20 {
            if excess < 0.0 {
                excess = 0.0;
            }

            df_lbs_s_ft2 = Self::ws26_dcap(
                node_class,
                2,
                ql_cfs,
                sfl,
                c1,
                z,
                effshl,
                depsid_ft,
                depmid_ft,
                wfl_ft,
                wfl_ft,
                roughness,
                crsh,
                excess,
                tb_s,
                flagc,
                chnk,
                nbarch,
                WS22_DCAP_MAXE,
                crfrac,
            )?;

            for class_offset in 0..class_count {
                dl_lbs_s_ft[class_offset] = df_lbs_s_ft2[class_offset] * wfl_ft;
                potld_lbs_s_ft[class_offset] = (gstu_lbs_s[class_offset]
                    + (dlat_lbs_s_ft[class_offset] * dx_ft)
                    + ((dl_lbs_s_ft[class_offset] + du_lbs_s_ft[class_offset]) * dx_ft / 2.0))
                    / wfl_ft;
            }
            tcl_lbs_s_ft = Self::ws18_trncap(effshl, &potld_lbs_s_ft, crdia_ft, crspg);

            let mut sumdf = 0.0_f64;
            let mut sumexd = 0.0_f64;
            sumtcl = 0.0;
            sumpld = 0.0;
            for class_offset in 0..class_count {
                sumtcl += tcl_lbs_s_ft[class_offset];
                sumpld += potld_lbs_s_ft[class_offset];
                let exdet = (((tcl_lbs_s_ft[class_offset] * wfl_ft)
                    - gstu_lbs_s[class_offset]
                    - (dlat_lbs_s_ft[class_offset] * dx_ft))
                    * (2.0 / dx_ft)
                    - du_lbs_s_ft[class_offset])
                    / wfl_ft;
                sumexd += exdet;
                sumdf += df_lbs_s_ft2[class_offset];
            }

            if !sumtcl.is_finite() || !sumpld.is_finite() {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("ws23_detach_sumtc_sumpl"),
                    sumtcl,
                ));
            }

            if sumtcl.abs() > WS10_ZERO_THRESHOLD && ((sumtcl - sumpld) / sumtcl).abs() < 0.01 {
                break;
            }

            let mut ratex = if sumdf.abs() > 1.0e-8 {
                sumexd / sumdf
            } else {
                sumtcl / sumpld
            };
            if !ratex.is_finite() || ratex <= 0.0 {
                ratex = sumtcl / sumpld;
            }
            excess = excold * ratex;
            excold = excess;
        }

        let mut next_gstu_lbs_s = vec![0.0_f64; class_count];
        for class_offset in 0..class_count {
            let next_flux = tcl_lbs_s_ft[class_offset] * wfl_ft;
            if !next_flux.is_finite() || next_flux < 0.0 {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("ws23_detach_next_flux"),
                    next_flux,
                ));
            }
            next_gstu_lbs_s[class_offset] = next_flux;
        }
        Ok(next_gstu_lbs_s)
    }

    #[allow(clippy::too_many_arguments, clippy::similar_names)]
    fn ws24_case12_detach_transition_closure(
        node_class: Ws10NodeClass,
        ql_cfs: f64,
        sfl: f64,
        c1: f64,
        z: f64,
        effshl: f64,
        depsid_ft: f64,
        depmid_ft: f64,
        wfl_ft: f64,
        roughness: f64,
        crsh: f64,
        tb_s: f64,
        flagc: i32,
        chnk: f64,
        nbarch: f64,
        crfrac: &[f64],
        gstde_lbs_s: &[f64],
        dlat_lbs_s_ft: &[f64],
        dx_ft_remaining: f64,
        crdia_ft: &[f64],
        crspg: &[f64],
    ) -> Result<Vec<f64>, Ws10GuardError> {
        if dx_ft_remaining <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws24_case12_dx_remaining"),
                dx_ft_remaining,
            ));
        }

        let zero_du_lbs_s_ft = vec![0.0_f64; gstde_lbs_s.len()];
        Self::ws23_detach_case4_iterative_closure(
            node_class,
            ql_cfs,
            sfl,
            c1,
            z,
            effshl,
            depsid_ft,
            depmid_ft,
            wfl_ft,
            roughness,
            crsh,
            tb_s,
            flagc,
            chnk,
            nbarch,
            crfrac,
            gstde_lbs_s,
            dlat_lbs_s_ft,
            &zero_du_lbs_s_ft,
            dx_ft_remaining,
            crdia_ft,
            crspg,
        )
    }

    #[allow(
        clippy::too_many_arguments,
        clippy::many_single_char_names,
        clippy::similar_names
    )]
    fn ws27_case4_enddet_bracket_closure(
        x_upper_ft: f64,
        x_lower_ft: f64,
        wfl_ft: f64,
        dx_ft: f64,
        gstu_lbs_s: &[f64],
        dlat_lbs_s_ft: &[f64],
        du_lbs_s_ft: &[f64],
        potld_case4_lbs_s_ft: &mut [f64],
        tcl_case4_lbs_s_ft: &mut [f64],
        mut trncap: impl FnMut(&[f64]) -> Vec<f64>,
    ) -> Ws27EnddetBracketProgress {
        let class_count = potld_case4_lbs_s_ft.len();
        let mut progress = Ws27EnddetBracketProgress::default();
        let mut xdsmal_ft = x_upper_ft;
        let mut xdbig_ft = x_lower_ft;
        let mut xdbmin_ft = x_lower_ft;
        let mut ndep = 0_u8;
        let mut recompute_xdbeg = true;

        loop {
            if recompute_xdbeg {
                let mut xdbeg_ft = vec![x_lower_ft; class_count];
                for class_offset in 0..class_count {
                    if potld_case4_lbs_s_ft[class_offset] > tcl_case4_lbs_s_ft[class_offset]
                        && du_lbs_s_ft[class_offset].abs() > WS10_ZERO_THRESHOLD
                    {
                        xdbeg_ft[class_offset] = ((2.0
                            * ((tcl_case4_lbs_s_ft[class_offset] * wfl_ft)
                                - gstu_lbs_s[class_offset]
                                - (dlat_lbs_s_ft[class_offset] * dx_ft)))
                            / du_lbs_s_ft[class_offset])
                            + x_upper_ft;
                    }
                }

                xdbmin_ft = xdbeg_ft.iter().copied().fold(x_lower_ft, f64::min);
                if xdbmin_ft <= xdsmal_ft {
                    xdbmin_ft = xdsmal_ft;
                }
            }

            for class_offset in 0..class_count {
                potld_case4_lbs_s_ft[class_offset] = (gstu_lbs_s[class_offset]
                    + (dlat_lbs_s_ft[class_offset] * dx_ft)
                    + (du_lbs_s_ft[class_offset] * (xdbmin_ft - x_upper_ft) / 2.0))
                    / wfl_ft;
            }
            tcl_case4_lbs_s_ft.copy_from_slice(&trncap(potld_case4_lbs_s_ft));

            ndep = ndep.saturating_add(1);
            progress.iteration_count = ndep;
            if ndep == 4 {
                break;
            }

            let mut nt = 0_usize;
            let mut sumtc = 0.0_f64;
            let mut sumpl = 0.0_f64;
            for class_offset in 0..class_count {
                sumtc += tcl_case4_lbs_s_ft[class_offset];
                sumpl += potld_case4_lbs_s_ft[class_offset];
                if tcl_case4_lbs_s_ft[class_offset] <= potld_case4_lbs_s_ft[class_offset] {
                    nt += 1;
                }
            }

            if sumtc.abs() > WS10_ZERO_THRESHOLD && ((sumtc - sumpl) / sumtc).abs() < 0.01 {
                break;
            }

            if nt < class_count {
                xdsmal_ft = xdbmin_ft;
                xdbmin_ft = 0.5 * (xdsmal_ft + xdbig_ft);
                recompute_xdbeg = false;
                progress.used_midpoint_rebracket = true;
            } else {
                xdbig_ft = xdbmin_ft;
                recompute_xdbeg = true;
                progress.used_xdbig_rebracket = true;
            }
        }

        progress
    }

    #[allow(
        clippy::too_many_arguments,
        clippy::many_single_char_names,
        clippy::too_many_lines,
        clippy::similar_names
    )]
    fn ws20_route_case12_segment_family(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
        ws21_case34_enabled: bool,
        event_duration: f64,
        qpo: f64,
        roughness: f64,
        sediment_controls: Ws15ChannelSedimentControls,
        nslpts: usize,
        peak_partition: Ws20IncomingPeakPartition,
        top_class_mass_kg: &[f64],
        lateral_class_mass_kg: &[f64],
        class_diameters_m: &[f64],
        class_numbers: &[usize],
    ) -> Result<(Vec<f64>, Ws20SegmentRoutingDiagnostics), Ws10GuardError> {
        if class_diameters_m.is_empty() {
            return Ok((Vec::new(), Ws20SegmentRoutingDiagnostics::default()));
        }

        let class_count = class_diameters_m.len();
        if top_class_mass_kg.len() != class_count
            || lateral_class_mass_kg.len() != class_count
            || class_numbers.len() != class_count
        {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws20_class_cardinality"),
                f64::from(u32::try_from(class_count).unwrap_or(u32::MAX)),
            ));
        }

        let node_id = request.node_id;
        let mut x_points_ft = Vec::with_capacity(nslpts);
        let mut slopes = Vec::with_capacity(nslpts);
        let mut widths_ft = Vec::with_capacity(nslpts);
        for point_number in 1..=nslpts {
            let x_symbol =
                BoundarySymbol::from(format!("ws10_channel_{node_id}_x_{point_number:04}"));
            let slope_symbol =
                BoundarySymbol::from(format!("ws10_channel_{node_id}_slope_{point_number:04}"));
            let width_symbol =
                BoundarySymbol::from(format!("ws10_channel_{node_id}_widb_{point_number:04}"));

            let x_ft =
                Self::require_channel_state_symbol_scalar(request, node_class, x_symbol.clone())?;
            let slope = Self::require_channel_state_symbol_scalar(
                request,
                node_class,
                slope_symbol.clone(),
            )?;
            let width_ft = Self::require_channel_state_symbol_scalar(
                request,
                node_class,
                width_symbol.clone(),
            )?;

            Self::require_channel_control_range(node_class, x_symbol, x_ft, Some(0.0), None)?;
            Self::require_channel_control_range(node_class, slope_symbol, slope, Some(0.0), None)?;
            Self::require_channel_control_range(
                node_class,
                width_symbol,
                width_ft,
                Some(WS10_ZERO_THRESHOLD),
                None,
            )?;

            x_points_ft.push(x_ft);
            slopes.push(slope.max(WS18_MIN_CHANNEL_SLOPE));
            widths_ft.push(width_ft);
        }

        let Some(&leff_ft) = x_points_ft.last() else {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws20_effective_length_ft"),
                0.0,
            ));
        };
        if leff_ft <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws20_effective_length_ft"),
                leff_ft,
            ));
        }

        let q_cfs = qpo * WS18_CFS_PER_CMS;
        if !q_cfs.is_finite() || q_cfs < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws20_q_cfs"),
                q_cfs,
            ));
        }

        let peak_sum_cms = peak_partition.hillslope_peak_cms + peak_partition.dependency_peak_cms;
        if !peak_sum_cms.is_finite() || peak_sum_cms < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws20_peak_sum_cms"),
                peak_sum_cms,
            ));
        }
        let top_fraction = if peak_sum_cms > WS10_ZERO_THRESHOLD {
            peak_partition.dependency_peak_cms / peak_sum_cms
        } else {
            0.0
        };
        if !top_fraction.is_finite() || !(0.0..=1.0).contains(&top_fraction) {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws20_top_fraction"),
                top_fraction,
            ));
        }

        let qu_top_cfs = q_cfs * top_fraction;
        let qlat_cfs_per_ft = (q_cfs - qu_top_cfs) / leff_ft;
        if !qlat_cfs_per_ft.is_finite() || qlat_cfs_per_ft < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws20_qlat_cfs_per_ft"),
                qlat_cfs_per_ft,
            ));
        }

        let mut gstu_lbs_s = vec![0.0_f64; class_count];
        let mut dlat_lbs_s_ft = vec![0.0_f64; class_count];
        let mut crdia_ft = vec![0.0_f64; class_count];
        let mut crspg = vec![0.0_f64; class_count];
        let mut fall_ft_s = vec![0.0_f64; class_count];
        for class_offset in 0..class_count {
            let class_number = class_numbers[class_offset];
            let specific_gravity = WS18_DEFAULT_CRSPG
                .get(class_number.saturating_sub(1))
                .copied()
                .ok_or_else(|| {
                    Self::domain_violation(
                        node_class,
                        BoundarySymbol::from(format!("ws20_particle_class_{class_number:04}")),
                        f64::from(u32::try_from(class_number).unwrap_or(u32::MAX)),
                    )
                })?;

            let top_flux = top_class_mass_kg[class_offset] * WS18_LBS_PER_KG / event_duration;
            let lateral_flux =
                lateral_class_mass_kg[class_offset] * WS18_LBS_PER_KG / event_duration;
            if !top_flux.is_finite() || top_flux < 0.0 {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from(format!("ws20_top_flux_{class_number:04}")),
                    top_flux,
                ));
            }
            if !lateral_flux.is_finite() || lateral_flux < 0.0 {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from(format!("ws20_lateral_flux_{class_number:04}")),
                    lateral_flux,
                ));
            }

            gstu_lbs_s[class_offset] = top_flux;
            dlat_lbs_s_ft[class_offset] = lateral_flux / leff_ft;
            crdia_ft[class_offset] =
                class_diameters_m[class_offset] * WS15_DEPTH_FROM_METERS_TO_FEET;
            crspg[class_offset] = specific_gravity;
            fall_ft_s[class_offset] =
                Self::ws20_fall_velocity_ft_s(specific_gravity, crdia_ft[class_offset]);
        }

        let flagc = if (sediment_controls.ishape - 2.0).abs() <= WS11_IPEAK_INTEGER_TOLERANCE {
            2
        } else {
            1
        };
        let crsh = sediment_controls.chntcr * WS15_CRSH_FROM_CHNTCR_SCALE;
        let chnk_symbol = BoundarySymbol::from(format!("ws10_channel_{node_id}_chnk"));
        let chnk =
            Self::require_channel_state_symbol_scalar(request, node_class, chnk_symbol.clone())?;
        Self::require_channel_control_range(node_class, chnk_symbol, chnk, Some(0.0), None)?;

        let mut diagnostics = Ws20SegmentRoutingDiagnostics::default();
        for segment_index in 1..nslpts {
            let x_upper_ft = x_points_ft[segment_index - 1];
            let x_lower_ft = x_points_ft[segment_index];
            let dx_ft = x_lower_ft - x_upper_ft;
            if dx_ft <= WS10_ZERO_THRESHOLD {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("ws20_dx_ft"),
                    dx_ft,
                ));
            }

            let qu_cfs = qu_top_cfs + (qlat_cfs_per_ft * x_upper_ft);
            let ql_cfs = qu_top_cfs + (qlat_cfs_per_ft * x_lower_ft);
            if !qu_cfs.is_finite() || qu_cfs < 0.0 {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("ws20_qu_cfs"),
                    qu_cfs,
                ));
            }
            if !ql_cfs.is_finite() || ql_cfs < 0.0 {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("ws20_ql_cfs"),
                    ql_cfs,
                ));
            }

            let (mut wfu_ft, mut effshu) = Self::ws18_hydchn(
                node_class,
                flagc,
                qu_cfs,
                slopes[segment_index - 1],
                sediment_controls.ctlz,
                sediment_controls.chnz,
                widths_ft[segment_index - 1],
                roughness,
                crsh,
                sediment_controls.chnnbr,
            )?;
            let (mut wfl_ft, mut effshl) = Self::ws18_hydchn(
                node_class,
                flagc,
                ql_cfs,
                slopes[segment_index],
                sediment_controls.ctlz,
                sediment_controls.chnz,
                widths_ft[segment_index],
                roughness,
                crsh,
                sediment_controls.chnnbr,
            )?;

            if wfu_ft <= WS10_ZERO_THRESHOLD && qu_cfs <= WS10_ZERO_THRESHOLD {
                wfu_ft = widths_ft[segment_index - 1];
                effshu = 0.0;
            }
            if wfl_ft <= WS10_ZERO_THRESHOLD && ql_cfs <= WS10_ZERO_THRESHOLD {
                wfl_ft = widths_ft[segment_index];
                effshl = 0.0;
            }
            if wfu_ft <= WS10_ZERO_THRESHOLD || wfl_ft <= WS10_ZERO_THRESHOLD {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("ws20_width_ft"),
                    wfu_ft.min(wfl_ft),
                ));
            }

            let gsu_lbs_s_ft: Vec<f64> = gstu_lbs_s.iter().map(|flux| flux / wfu_ft).collect();
            let tcu_lbs_s_ft = Self::ws18_trncap(effshu, &gsu_lbs_s_ft, &crdia_ft, &crspg);

            let mut potld_lbs_s_ft = vec![0.0_f64; class_count];
            for class_offset in 0..class_count {
                potld_lbs_s_ft[class_offset] =
                    (gstu_lbs_s[class_offset] + (dlat_lbs_s_ft[class_offset] * dx_ft)) / wfl_ft;
            }
            let tcl_lbs_s_ft = Self::ws18_trncap(effshl, &potld_lbs_s_ft, &crdia_ft, &crspg);

            let mut dtcdx_lbs_s_ft2 = vec![0.0_f64; class_count];
            for class_offset in 0..class_count {
                dtcdx_lbs_s_ft2[class_offset] = ((tcl_lbs_s_ft[class_offset] * wfl_ft)
                    - (tcu_lbs_s_ft[class_offset] * wfu_ft))
                    / dx_ft;
            }

            let wfa_ft = 0.5 * (wfl_ft + wfu_ft);
            let qtemp_cfs_per_ft = if qlat_cfs_per_ft > WS10_ZERO_THRESHOLD {
                qlat_cfs_per_ft
            } else {
                0.0
            };
            let phi: Vec<f64> = if qtemp_cfs_per_ft > 0.0 {
                fall_ft_s
                    .iter()
                    .map(|fall| fall * wfa_ft / qtemp_cfs_per_ft)
                    .collect()
            } else {
                vec![0.0; class_count]
            };

            let mut excess = 1.0_f64;
            for class_offset in 0..class_count {
                if tcu_lbs_s_ft[class_offset] <= 1.0e-8 {
                    excess = 0.0;
                    break;
                }
                excess =
                    excess.min(1.0 - (gsu_lbs_s_ft[class_offset] / tcu_lbs_s_ft[class_offset]));
            }

            if excess > 0.0 {
                if !ws21_case34_enabled {
                    diagnostics.detachment_unmigrated_segments =
                        diagnostics.detachment_unmigrated_segments.saturating_add(1);
                    for class_offset in 0..class_count {
                        gstu_lbs_s[class_offset] += dlat_lbs_s_ft[class_offset] * dx_ft;
                    }
                    continue;
                }

                let crfrac = Self::ws22_require_crfrac_vector(request, node_class, class_numbers)?;
                let depmid_ft = sediment_controls.chnedm * WS15_DEPTH_FROM_METERS_TO_FEET;
                let depsid_ft = sediment_controls.chneds * WS15_DEPTH_FROM_METERS_TO_FEET;
                let tb_s = 2.0 * event_duration;
                let dcap_df_lbs_s_ft2 = Self::ws26_dcap(
                    node_class,
                    1,
                    qu_cfs,
                    slopes[segment_index - 1].max(WS22_DCAP_MIN_SLOPE),
                    sediment_controls.ctlz,
                    sediment_controls.chnz,
                    effshu,
                    depsid_ft,
                    depmid_ft,
                    wfu_ft,
                    wfu_ft,
                    roughness,
                    crsh,
                    excess,
                    tb_s,
                    flagc,
                    chnk,
                    sediment_controls.chnnbr,
                    WS22_DCAP_MAXE,
                    &crfrac,
                )?;

                let mut du_lbs_s_ft = vec![0.0_f64; class_count];
                for class_offset in 0..class_count {
                    du_lbs_s_ft[class_offset] = dcap_df_lbs_s_ft2[class_offset] * wfu_ft;
                }

                let case3_segment = tcl_lbs_s_ft
                    .iter()
                    .zip(&potld_lbs_s_ft)
                    .all(|(tcl, potld)| *tcl <= *potld);

                if case3_segment {
                    diagnostics.case3_segments = diagnostics.case3_segments.saturating_add(1);

                    let mut xdbeg_ft = vec![x_upper_ft; class_count];
                    let nz = du_lbs_s_ft
                        .iter()
                        .filter(|value| **value > WS10_ZERO_THRESHOLD)
                        .count();
                    let nk = gsu_lbs_s_ft
                        .iter()
                        .zip(&tcu_lbs_s_ft)
                        .filter(|(gsu, tcu)| (**gsu - **tcu).abs() <= WS10_ZERO_THRESHOLD)
                        .count();
                    let all_detaching = nz == class_count && nk == class_count;

                    for class_offset in 0..class_count {
                        if tcl_lbs_s_ft[class_offset] < potld_lbs_s_ft[class_offset] {
                            let denxdb = if all_detaching {
                                (2.0 * dlat_lbs_s_ft[class_offset]) + du_lbs_s_ft[class_offset]
                            } else {
                                (du_lbs_s_ft[class_offset] / 2.0) + dlat_lbs_s_ft[class_offset]
                                    - dtcdx_lbs_s_ft2[class_offset]
                            };

                            if denxdb.is_finite() && denxdb.abs() > WS10_ZERO_THRESHOLD {
                                xdbeg_ft[class_offset] = if all_detaching {
                                    ((dx_ft * du_lbs_s_ft[class_offset]) / denxdb) + x_upper_ft
                                } else {
                                    (((tcu_lbs_s_ft[class_offset] * wfu_ft)
                                        - gstu_lbs_s[class_offset])
                                        / denxdb)
                                        + x_upper_ft
                                };
                            }
                        }
                    }

                    let mut next_gstu_lbs_s = vec![0.0_f64; class_count];
                    let mut segment_invalid = false;
                    for class_offset in 0..class_count {
                        let next_flux =
                            if potld_lbs_s_ft[class_offset] <= tcl_lbs_s_ft[class_offset] {
                                potld_lbs_s_ft[class_offset] * wfl_ft
                            } else {
                                let xrat = if x_lower_ft.abs() <= WS10_ZERO_THRESHOLD {
                                    0.0
                                } else {
                                    xdbeg_ft[class_offset] / x_lower_ft
                                };

                                let dl_lbs_s_ft2 = if qlat_cfs_per_ft > WS10_ZERO_THRESHOLD {
                                    let denphi = 1.0 + phi[class_offset];
                                    if denphi.abs() <= WS10_ZERO_THRESHOLD || !denphi.is_finite() {
                                        0.0
                                    } else {
                                        (phi[class_offset] / denphi)
                                            * (dtcdx_lbs_s_ft2[class_offset]
                                                - dlat_lbs_s_ft[class_offset])
                                            * (1.0 - xrat.powf(1.0 + phi[class_offset]))
                                    }
                                } else {
                                    dtcdx_lbs_s_ft2[class_offset]
                                };

                                let dengsl = phi[class_offset] * wfl_ft;
                                let gsl_lbs_s_ft =
                                    if dengsl.abs() <= WS10_ZERO_THRESHOLD || !dengsl.is_finite() {
                                        tcl_lbs_s_ft[class_offset]
                                    } else {
                                        tcl_lbs_s_ft[class_offset]
                                            - (dl_lbs_s_ft2 * x_lower_ft / dengsl)
                                    };

                                gsl_lbs_s_ft * wfl_ft
                            };

                        if !next_flux.is_finite() || next_flux < 0.0 {
                            segment_invalid = true;
                            break;
                        }
                        next_gstu_lbs_s[class_offset] = next_flux;
                    }

                    if segment_invalid {
                        diagnostics.ws21_detach_unmigrated_segments = diagnostics
                            .ws21_detach_unmigrated_segments
                            .saturating_add(1);
                        diagnostics.detachment_unmigrated_segments =
                            diagnostics.detachment_unmigrated_segments.saturating_add(1);
                        for class_offset in 0..class_count {
                            gstu_lbs_s[class_offset] += dlat_lbs_s_ft[class_offset] * dx_ft;
                        }
                        continue;
                    }

                    gstu_lbs_s = next_gstu_lbs_s;
                    continue;
                }

                diagnostics.case4_segments = diagnostics.case4_segments.saturating_add(1);

                let mut potld_case4_lbs_s_ft = vec![0.0_f64; class_count];
                for class_offset in 0..class_count {
                    potld_case4_lbs_s_ft[class_offset] = (gstu_lbs_s[class_offset]
                        + (dlat_lbs_s_ft[class_offset] * dx_ft)
                        + (du_lbs_s_ft[class_offset] * dx_ft / 2.0))
                        / wfl_ft;
                }

                let mut tcl_case4_lbs_s_ft =
                    Self::ws18_trncap(effshl, &potld_case4_lbs_s_ft, &crdia_ft, &crspg);
                let nt_case4 = tcl_case4_lbs_s_ft
                    .iter()
                    .zip(&potld_case4_lbs_s_ft)
                    .filter(|(tcl, potld)| **tcl <= **potld)
                    .count();

                if nt_case4 < class_count {
                    gstu_lbs_s = Self::ws23_detach_case4_iterative_closure(
                        node_class,
                        ql_cfs,
                        slopes[segment_index].max(WS22_DCAP_MIN_SLOPE),
                        sediment_controls.ctlz,
                        sediment_controls.chnz,
                        effshl,
                        depsid_ft,
                        depmid_ft,
                        wfl_ft,
                        roughness,
                        crsh,
                        tb_s,
                        flagc,
                        chnk,
                        sediment_controls.chnnbr,
                        &crfrac,
                        &gstu_lbs_s,
                        &dlat_lbs_s_ft,
                        &du_lbs_s_ft,
                        dx_ft,
                        &crdia_ft,
                        &crspg,
                    )?;
                    continue;
                }

                diagnostics.enddet_segments = diagnostics.enddet_segments.saturating_add(1);
                let _ = Self::ws27_case4_enddet_bracket_closure(
                    x_upper_ft,
                    x_lower_ft,
                    wfl_ft,
                    dx_ft,
                    &gstu_lbs_s,
                    &dlat_lbs_s_ft,
                    &du_lbs_s_ft,
                    &mut potld_case4_lbs_s_ft,
                    &mut tcl_case4_lbs_s_ft,
                    |potld| Self::ws18_trncap(effshl, potld, &crdia_ft, &crspg),
                );

                let mut next_gstu_lbs_s = vec![0.0_f64; class_count];
                let mut segment_invalid = false;
                for class_offset in 0..class_count {
                    let next_flux = tcl_case4_lbs_s_ft[class_offset] * wfl_ft;
                    if !next_flux.is_finite() || next_flux < 0.0 {
                        segment_invalid = true;
                        break;
                    }
                    next_gstu_lbs_s[class_offset] = next_flux;
                }

                if segment_invalid {
                    diagnostics.ws21_detach_unmigrated_segments = diagnostics
                        .ws21_detach_unmigrated_segments
                        .saturating_add(1);
                    diagnostics.detachment_unmigrated_segments =
                        diagnostics.detachment_unmigrated_segments.saturating_add(1);
                    for class_offset in 0..class_count {
                        gstu_lbs_s[class_offset] += dlat_lbs_s_ft[class_offset] * dx_ft;
                    }
                    continue;
                }

                gstu_lbs_s = next_gstu_lbs_s;
                continue;
            }

            let mut saw_case1 = false;
            let mut saw_case2 = false;
            let mut next_gstu_lbs_s = vec![0.0_f64; class_count];
            let mut xde_ft = vec![x_lower_ft; class_count];
            let mut gstde_lbs_s = vec![0.0_f64; class_count];
            let mut case12_nz = 0_usize;
            let mut segment_route_invalid = false;
            for class_offset in 0..class_count {
                let xrat = if x_lower_ft > WS10_ZERO_THRESHOLD {
                    x_upper_ft / x_lower_ft
                } else {
                    0.0
                };
                let du_lbs_s_ft2 = if qu_cfs > 1.0e-8 {
                    let candidate = (fall_ft_s[class_offset] * wfu_ft / qu_cfs)
                        * ((tcu_lbs_s_ft[class_offset] * wfu_ft) - gstu_lbs_s[class_offset]);
                    candidate.min(0.0)
                } else if segment_index == 1
                    && qu_cfs < 0.001
                    && dtcdx_lbs_s_ft2[class_offset] < dlat_lbs_s_ft[class_offset]
                {
                    let phi_k = phi[class_offset];
                    if phi_k > WS10_ZERO_THRESHOLD {
                        (phi_k / (1.0 + phi_k))
                            * (dtcdx_lbs_s_ft2[class_offset] - dlat_lbs_s_ft[class_offset])
                    } else {
                        0.0
                    }
                } else {
                    0.0
                };

                let expon = 1.0 + phi[class_offset];
                let mut dl_lbs_s_ft2 = if qlat_cfs_per_ft > WS10_ZERO_THRESHOLD {
                    let phi_k = phi[class_offset];
                    let numerator =
                        phi_k * (dtcdx_lbs_s_ft2[class_offset] - dlat_lbs_s_ft[class_offset]);
                    (numerator / (1.0 + phi_k)) * (1.0 - xrat.powf(expon))
                } else {
                    dtcdx_lbs_s_ft2[class_offset]
                };
                dl_lbs_s_ft2 += du_lbs_s_ft2 * xrat.powf(expon);

                let next_flux = if dl_lbs_s_ft2 <= 0.0 {
                    saw_case1 = true;
                    case12_nz = case12_nz.saturating_add(1);
                    let phi_k = phi[class_offset];
                    let gsl = if phi_k > WS10_ZERO_THRESHOLD {
                        tcl_lbs_s_ft[class_offset] - ((dl_lbs_s_ft2 * x_lower_ft / phi_k) / wfl_ft)
                    } else {
                        0.0
                    };
                    xde_ft[class_offset] = x_lower_ft;
                    gstde_lbs_s[class_offset] = gsl * wfl_ft;
                    gsl * wfl_ft
                } else {
                    saw_case2 = true;
                    let xde_value_ft = if du_lbs_s_ft2.abs() <= WS10_ZERO_THRESHOLD {
                        x_upper_ft
                    } else if qlat_cfs_per_ft > WS10_ZERO_THRESHOLD {
                        let den = dtcdx_lbs_s_ft2[class_offset] - dlat_lbs_s_ft[class_offset];
                        if den.abs() <= WS10_ZERO_THRESHOLD
                            || phi[class_offset] <= WS10_ZERO_THRESHOLD
                        {
                            x_upper_ft
                        } else {
                            let core = (1.0
                                - (((1.0 + phi[class_offset]) / phi[class_offset])
                                    * (du_lbs_s_ft2 / den)))
                                .abs();
                            x_upper_ft * core.powf(1.0 / (1.0 + phi[class_offset]))
                        }
                    } else if dtcdx_lbs_s_ft2[class_offset].abs() <= WS10_ZERO_THRESHOLD {
                        x_upper_ft
                    } else {
                        x_upper_ft * (1.0 - (du_lbs_s_ft2 / dtcdx_lbs_s_ft2[class_offset]))
                    };

                    let gstde_value_lbs_s = if du_lbs_s_ft2.abs() <= WS10_ZERO_THRESHOLD {
                        gstu_lbs_s[class_offset]
                    } else {
                        (dtcdx_lbs_s_ft2[class_offset] * (xde_value_ft - x_upper_ft))
                            + (tcu_lbs_s_ft[class_offset] * wfu_ft)
                    };
                    let gsl_lbs_s_ft = if (xde_value_ft - x_lower_ft).abs() > WS10_ZERO_THRESHOLD {
                        (gstde_value_lbs_s
                            + (dlat_lbs_s_ft[class_offset] * (x_lower_ft - xde_value_ft)))
                            / wfl_ft
                    } else {
                        tcl_lbs_s_ft[class_offset]
                    };
                    xde_ft[class_offset] = xde_value_ft;
                    gstde_lbs_s[class_offset] = gstde_value_lbs_s;

                    gsl_lbs_s_ft * wfl_ft
                };

                if !next_flux.is_finite() || next_flux < 0.0 {
                    segment_route_invalid = true;
                    break;
                }
                next_gstu_lbs_s[class_offset] = next_flux;
            }

            if segment_route_invalid {
                diagnostics.detachment_unmigrated_segments =
                    diagnostics.detachment_unmigrated_segments.saturating_add(1);
                for class_offset in 0..class_count {
                    gstu_lbs_s[class_offset] += dlat_lbs_s_ft[class_offset] * dx_ft;
                }
                continue;
            }

            if ws21_case34_enabled && saw_case2 && case12_nz < class_count {
                let xdemax_ft = xde_ft.iter().copied().fold(x_upper_ft, f64::max);
                if xdemax_ft + WS10_ZERO_THRESHOLD < x_lower_ft {
                    let dx_remaining_ft = x_lower_ft - xdemax_ft;
                    let mut gstde_transition_lbs_s = gstde_lbs_s.clone();
                    for class_offset in 0..class_count {
                        gstde_transition_lbs_s[class_offset] +=
                            dlat_lbs_s_ft[class_offset] * (xdemax_ft - xde_ft[class_offset]);
                    }

                    let crfrac =
                        Self::ws22_require_crfrac_vector(request, node_class, class_numbers)?;
                    let depmid_ft = sediment_controls.chnedm * WS15_DEPTH_FROM_METERS_TO_FEET;
                    let depsid_ft = sediment_controls.chneds * WS15_DEPTH_FROM_METERS_TO_FEET;
                    let tb_s = 2.0 * event_duration;

                    gstu_lbs_s = Self::ws24_case12_detach_transition_closure(
                        node_class,
                        ql_cfs,
                        slopes[segment_index].max(WS22_DCAP_MIN_SLOPE),
                        sediment_controls.ctlz,
                        sediment_controls.chnz,
                        effshl,
                        depsid_ft,
                        depmid_ft,
                        wfl_ft,
                        roughness,
                        crsh,
                        tb_s,
                        flagc,
                        chnk,
                        sediment_controls.chnnbr,
                        &crfrac,
                        &gstde_transition_lbs_s,
                        &dlat_lbs_s_ft,
                        dx_remaining_ft,
                        &crdia_ft,
                        &crspg,
                    )?;
                    diagnostics.ws24_case2_detach_segments =
                        diagnostics.ws24_case2_detach_segments.saturating_add(1);
                    if saw_case1 {
                        diagnostics.case1_segments = diagnostics.case1_segments.saturating_add(1);
                    }
                    if saw_case2 {
                        diagnostics.case2_segments = diagnostics.case2_segments.saturating_add(1);
                    }
                    continue;
                }
            }

            if saw_case1 {
                diagnostics.case1_segments = diagnostics.case1_segments.saturating_add(1);
            }
            if saw_case2 {
                diagnostics.case2_segments = diagnostics.case2_segments.saturating_add(1);
            }
            gstu_lbs_s = next_gstu_lbs_s;
        }

        let mut outgoing_class_mass_kg = vec![0.0_f64; class_count];
        for class_offset in 0..class_count {
            let class_number = class_numbers[class_offset];
            let mass_kg = gstu_lbs_s[class_offset] * event_duration / WS18_LBS_PER_KG;
            if !mass_kg.is_finite() || mass_kg < 0.0 {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from(format!("ws20_outgoing_mass_kg_{class_number:04}")),
                    mass_kg,
                ));
            }
            outgoing_class_mass_kg[class_offset] = mass_kg;
        }

        Ok((outgoing_class_mass_kg, diagnostics))
    }

    fn read_channel_opt_in_toggle(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
        suffix: &'static str,
    ) -> Result<bool, Ws10GuardError> {
        let toggle_symbol = Self::channel_wave_state_symbol(request.node_id, suffix);
        let Some(value) = request.state_surface.get(&toggle_symbol) else {
            return Ok(false);
        };

        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Self::non_finite(node_class, toggle_symbol, scalar));
        }
        if scalar.abs() <= WS11_IPEAK_INTEGER_TOLERANCE {
            return Ok(false);
        }
        if (scalar - 1.0).abs() <= WS11_IPEAK_INTEGER_TOLERANCE {
            return Ok(true);
        }

        Err(Self::domain_violation(node_class, toggle_symbol, scalar))
    }

    #[allow(
        clippy::similar_names,
        clippy::too_many_lines,
        clippy::too_many_arguments
    )]
    fn assemble_incoming_sediment_load_and_capacity(
        request: &WatershedKernelRequest<'_>,
        node_class: Ws10NodeClass,
        event_duration: f64,
        qpo: f64,
        roughness: f64,
        sediment_controls: Ws15ChannelSedimentControls,
        nslpts: usize,
        peak_partition: Ws20IncomingPeakPartition,
    ) -> Result<Ws19ChannelSedimentPublication, Ws10GuardError> {
        if !event_duration.is_finite() || event_duration <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("event_duration"),
                event_duration,
            ));
        }
        if !qpo.is_finite() || qpo < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("qpo"),
                qpo,
            ));
        }
        if !roughness.is_finite() || roughness <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("roughness"),
                roughness,
            ));
        }

        let mut incoming_sediment_mass_kg = 0.0_f64;
        let mut class_mass_kg: Vec<f64> = Vec::new();
        let mut class_diameter_mass_m: Vec<f64> = Vec::new();
        let mut top_class_mass_kg: Vec<f64> = Vec::new();
        let mut lateral_class_mass_kg: Vec<f64> = Vec::new();
        for &hillslope_id in request.contributor_hillslopes {
            let payload = Self::read_hillslope_sediment_payload(request, node_class, hillslope_id)?;
            incoming_sediment_mass_kg += payload.mass_kg;
            let fraction_sum = payload.fractions.iter().sum::<f64>();
            if !fraction_sum.is_finite() || fraction_sum <= WS10_ZERO_THRESHOLD {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from(format!("hs{hillslope_id}_particle_flow_fraction_sum")),
                    fraction_sum,
                ));
            }

            for class_offset in 0..payload.fractions.len() {
                if class_mass_kg.len() <= class_offset {
                    class_mass_kg.resize(class_offset + 1, 0.0);
                    class_diameter_mass_m.resize(class_offset + 1, 0.0);
                    top_class_mass_kg.resize(class_offset + 1, 0.0);
                    lateral_class_mass_kg.resize(class_offset + 1, 0.0);
                }

                let normalized_fraction = payload.fractions[class_offset] / fraction_sum;
                let class_mass = payload.mass_kg * normalized_fraction;
                class_mass_kg[class_offset] += class_mass;
                class_diameter_mass_m[class_offset] +=
                    class_mass * payload.particle_diameters_m[class_offset];
                lateral_class_mass_kg[class_offset] += class_mass;
            }
        }

        for dependency in &request.dependency_nodes {
            let (dependency_class, dependency_id) = Self::parse_dependency(node_class, dependency)?;
            if dependency_class != Ws10NodeClass::Channel {
                continue;
            }

            let payload = Self::read_channel_sediment_payload(
                request,
                node_class,
                dependency_id,
                event_duration,
            )?;
            incoming_sediment_mass_kg += payload.mass_kg;
            if payload.mass_kg <= WS10_ZERO_THRESHOLD {
                continue;
            }

            let fraction_sum = payload.fractions.iter().sum::<f64>();
            if !fraction_sum.is_finite() || fraction_sum <= WS10_ZERO_THRESHOLD {
                return Err(Self::domain_violation(
                    node_class,
                    Self::channel_wave_state_symbol(dependency_id, "particle_flow_fraction_sum"),
                    fraction_sum,
                ));
            }

            for class_offset in 0..payload.fractions.len() {
                if class_mass_kg.len() <= class_offset {
                    class_mass_kg.resize(class_offset + 1, 0.0);
                    class_diameter_mass_m.resize(class_offset + 1, 0.0);
                    top_class_mass_kg.resize(class_offset + 1, 0.0);
                    lateral_class_mass_kg.resize(class_offset + 1, 0.0);
                }

                let normalized_fraction = payload.fractions[class_offset] / fraction_sum;
                let class_mass = payload.mass_kg * normalized_fraction;
                class_mass_kg[class_offset] += class_mass;
                class_diameter_mass_m[class_offset] +=
                    class_mass * payload.particle_diameters_m[class_offset];
                top_class_mass_kg[class_offset] += class_mass;
            }
        }

        if !incoming_sediment_mass_kg.is_finite() || incoming_sediment_mass_kg < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("incoming_sediment_mass_kg"),
                incoming_sediment_mass_kg,
            ));
        }

        let class_mass_total = class_mass_kg.iter().copied().sum::<f64>();
        let mut active_class_mass_kg = Vec::new();
        let mut active_top_class_mass_kg = Vec::new();
        let mut active_lateral_class_mass_kg = Vec::new();
        let mut active_particle_diameters_m = Vec::new();
        let mut active_class_numbers = Vec::new();
        if class_mass_total > WS10_ZERO_THRESHOLD {
            for class_offset in 0..class_mass_kg.len() {
                let class_mass = class_mass_kg[class_offset];
                if class_mass <= WS10_ZERO_THRESHOLD {
                    continue;
                }

                let class_diameter_m = class_diameter_mass_m[class_offset] / class_mass;
                if !class_diameter_m.is_finite() || class_diameter_m <= WS10_ZERO_THRESHOLD {
                    return Err(Self::domain_violation(
                        node_class,
                        BoundarySymbol::from(format!(
                            "ws19_class_diameter_m_{:04}",
                            class_offset + 1
                        )),
                        class_diameter_m,
                    ));
                }

                active_class_mass_kg.push(class_mass);
                active_top_class_mass_kg.push(*top_class_mass_kg.get(class_offset).unwrap_or(&0.0));
                active_lateral_class_mass_kg
                    .push(*lateral_class_mass_kg.get(class_offset).unwrap_or(&0.0));
                active_particle_diameters_m.push(class_diameter_m);
                active_class_numbers.push(class_offset + 1);
            }
        }

        let mut outgoing_class_mass_kg = active_class_mass_kg.clone();
        let mut ws20_diagnostics = Ws20SegmentRoutingDiagnostics::default();
        let ws20_case12_enabled =
            Self::read_channel_opt_in_toggle(request, node_class, "ws20_case12_enable")?;
        let ws21_case34_opt_in =
            Self::read_channel_opt_in_toggle(request, node_class, "ws21_case34_enable")?;
        let ws21_case34_enabled = ws20_case12_enabled || ws21_case34_opt_in;

        if ws20_case12_enabled
            && qpo > WS10_ZERO_THRESHOLD
            && incoming_sediment_mass_kg > WS10_ZERO_THRESHOLD
            && !active_class_mass_kg.is_empty()
        {
            let (routed_masses, diagnostics) = Self::ws20_route_case12_segment_family(
                request,
                node_class,
                ws21_case34_enabled,
                event_duration,
                qpo,
                roughness,
                sediment_controls,
                nslpts,
                peak_partition,
                &active_top_class_mass_kg,
                &active_lateral_class_mass_kg,
                &active_particle_diameters_m,
                &active_class_numbers,
            )?;
            outgoing_class_mass_kg = routed_masses;
            ws20_diagnostics = diagnostics;
        }

        let qsed = outgoing_class_mass_kg.iter().copied().sum::<f64>() / event_duration;
        if !qsed.is_finite() || qsed < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("qsed"),
                qsed,
            ));
        }

        let mut particle_flow_fractions = Vec::new();
        let mut particle_diameters_m = Vec::new();
        let routed_class_total = outgoing_class_mass_kg.iter().copied().sum::<f64>();
        if routed_class_total > WS10_ZERO_THRESHOLD {
            for class_offset in 0..outgoing_class_mass_kg.len() {
                let class_mass = outgoing_class_mass_kg[class_offset];
                if class_mass <= WS10_ZERO_THRESHOLD {
                    continue;
                }
                particle_flow_fractions.push(class_mass / routed_class_total);
                particle_diameters_m.push(active_particle_diameters_m[class_offset]);
            }

            let published_fraction_sum = particle_flow_fractions.iter().copied().sum::<f64>();
            if !published_fraction_sum.is_finite() || published_fraction_sum <= WS10_ZERO_THRESHOLD
            {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from("ws19_published_fraction_sum"),
                    published_fraction_sum,
                ));
            }
            for fraction in &mut particle_flow_fractions {
                *fraction /= published_fraction_sum;
            }
        }

        if qpo <= WS10_ZERO_THRESHOLD || incoming_sediment_mass_kg <= WS10_ZERO_THRESHOLD {
            return Ok(Ws19ChannelSedimentPublication {
                qsed,
                tc: 0.0,
                particle_flow_fractions,
                particle_diameters_m,
                ws20_case1_segments: ws20_diagnostics.case1_segments,
                ws20_case2_segments: ws20_diagnostics.case2_segments,
                ws24_case2_detach_segments: ws20_diagnostics.ws24_case2_detach_segments,
                ws20_detachment_unmigrated_segments: ws20_diagnostics
                    .detachment_unmigrated_segments,
                ws21_case3_segments: ws20_diagnostics.case3_segments,
                ws21_case4_segments: ws20_diagnostics.case4_segments,
                ws21_enddet_segments: ws20_diagnostics.enddet_segments,
                ws21_detach_unmigrated_segments: ws20_diagnostics.ws21_detach_unmigrated_segments,
            });
        }

        let node_id = request.node_id;
        let slope_symbol =
            BoundarySymbol::from(format!("ws10_channel_{node_id}_slope_{nslpts:04}"));
        let width_symbol = BoundarySymbol::from(format!("ws10_channel_{node_id}_widb_{nslpts:04}"));

        let terminal_slope =
            Self::require_channel_state_symbol_scalar(request, node_class, slope_symbol.clone())?;
        let terminal_width_ft =
            Self::require_channel_state_symbol_scalar(request, node_class, width_symbol.clone())?;
        Self::require_channel_control_range(
            node_class,
            slope_symbol,
            terminal_slope,
            Some(WS18_MIN_CHANNEL_SLOPE),
            None,
        )?;
        Self::require_channel_control_range(
            node_class,
            width_symbol,
            terminal_width_ft,
            Some(WS10_ZERO_THRESHOLD),
            None,
        )?;

        let q_cfs = qpo * WS18_CFS_PER_CMS;
        if !q_cfs.is_finite() || q_cfs < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws18_q_cfs"),
                q_cfs,
            ));
        }

        let flagc = if (sediment_controls.ishape - 2.0).abs() <= WS11_IPEAK_INTEGER_TOLERANCE {
            2
        } else {
            1
        };
        let c1 = sediment_controls.ctlz;
        let sf = terminal_slope;
        let crsh = sediment_controls.chntcr * WS15_CRSH_FROM_CHNTCR_SCALE;
        let (flow_width_ft, effsh) = Self::ws18_hydchn(
            node_class,
            flagc,
            q_cfs,
            sf,
            c1,
            sediment_controls.chnz,
            terminal_width_ft,
            roughness,
            crsh,
            sediment_controls.chnnbr,
        )?;
        if flow_width_ft <= WS10_ZERO_THRESHOLD {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("ws18_flow_width_ft"),
                flow_width_ft,
            ));
        }

        let mut qs = Vec::new();
        let mut crdia_ft = Vec::new();
        let mut crspg = Vec::new();
        for class_offset in 0..class_mass_kg.len() {
            let class_mass = class_mass_kg[class_offset];
            if class_mass <= WS10_ZERO_THRESHOLD {
                continue;
            }
            let class_diameter_m = class_diameter_mass_m[class_offset] / class_mass;
            if !class_diameter_m.is_finite() || class_diameter_m <= WS10_ZERO_THRESHOLD {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from(format!("ws18_class_diameter_m_{:04}", class_offset + 1)),
                    class_diameter_m,
                ));
            }
            let class_load_lbs_per_s = class_mass * WS18_LBS_PER_KG / event_duration;
            if !class_load_lbs_per_s.is_finite() || class_load_lbs_per_s < 0.0 {
                return Err(Self::domain_violation(
                    node_class,
                    BoundarySymbol::from(format!("ws18_class_load_lbs_s_{:04}", class_offset + 1)),
                    class_load_lbs_per_s,
                ));
            }

            qs.push(class_load_lbs_per_s / flow_width_ft);
            crdia_ft.push(class_diameter_m * WS15_DEPTH_FROM_METERS_TO_FEET);
            let specific_gravity =
                WS18_DEFAULT_CRSPG
                    .get(class_offset)
                    .copied()
                    .ok_or_else(|| {
                        let class_index_u32 = u32::try_from(class_offset + 1).unwrap_or(u32::MAX);
                        Self::domain_violation(
                            node_class,
                            BoundarySymbol::from(format!(
                                "ws18_particle_class_index_{:04}",
                                class_offset + 1
                            )),
                            f64::from(class_index_u32),
                        )
                    })?;
            crspg.push(specific_gravity);
        }

        let tc_per_width = Self::ws18_trncap(effsh, &qs, &crdia_ft, &crspg);
        let tc_lbs_per_s = tc_per_width.iter().copied().sum::<f64>() * flow_width_ft;
        let tc = tc_lbs_per_s / WS18_LBS_PER_KG;
        if !tc.is_finite() || tc < 0.0 {
            return Err(Self::domain_violation(
                node_class,
                BoundarySymbol::from("tc"),
                tc,
            ));
        }

        Ok(Ws19ChannelSedimentPublication {
            qsed,
            tc,
            particle_flow_fractions,
            particle_diameters_m,
            ws20_case1_segments: ws20_diagnostics.case1_segments,
            ws20_case2_segments: ws20_diagnostics.case2_segments,
            ws24_case2_detach_segments: ws20_diagnostics.ws24_case2_detach_segments,
            ws20_detachment_unmigrated_segments: ws20_diagnostics.detachment_unmigrated_segments,
            ws21_case3_segments: ws20_diagnostics.case3_segments,
            ws21_case4_segments: ws20_diagnostics.case4_segments,
            ws21_enddet_segments: ws20_diagnostics.enddet_segments,
            ws21_detach_unmigrated_segments: ws20_diagnostics.ws21_detach_unmigrated_segments,
        })
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

    fn channel_wave_state_symbol(node_id: u32, suffix: &str) -> BoundarySymbol {
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
        let sediment_controls = Self::read_ws15_channel_sediment_controls(request, node_class)?;
        let nslpts = Self::require_ws17_channel_segment_scaffold(request, node_class)?;
        let sediment_scaffold = Self::derive_ws15_channel_sediment_scaffold(
            node_class,
            request.node_id,
            sediment_controls,
        )?;

        let peak_partition = Self::assemble_incoming_peak_partition(request, node_class)?;
        let incoming_peak = peak_partition.hillslope_peak_cms + peak_partition.dependency_peak_cms;
        let incoming_duration = peak_partition.duration_s;

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
        let sediment_publication = Self::assemble_incoming_sediment_load_and_capacity(
            request,
            node_class,
            event_duration,
            qpo,
            roughness,
            sediment_controls,
            nslpts,
            peak_partition,
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
                sediment_publication.qsed,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "tc"),
                sediment_publication.tc,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "ws20_case1_segment_count"),
                f64::from(sediment_publication.ws20_case1_segments),
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "ws20_case2_segment_count"),
                f64::from(sediment_publication.ws20_case2_segments),
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "ws24_case2_detach_segment_count"),
                f64::from(sediment_publication.ws24_case2_detach_segments),
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(
                    request.node_id,
                    "ws20_detachment_unmigrated_segment_count",
                ),
                f64::from(sediment_publication.ws20_detachment_unmigrated_segments),
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "ws21_case3_segment_count"),
                f64::from(sediment_publication.ws21_case3_segments),
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "ws21_case4_segment_count"),
                f64::from(sediment_publication.ws21_case4_segments),
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "ws21_enddet_segment_count"),
                f64::from(sediment_publication.ws21_enddet_segments),
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(
                    request.node_id,
                    "ws21_detach_unmigrated_segment_count",
                ),
                f64::from(sediment_publication.ws21_detach_unmigrated_segments),
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "chz"),
                sediment_scaffold.chz,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "nbarch"),
                sediment_scaffold.nbarch,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "crsh"),
                sediment_scaffold.crsh,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "depmid"),
                sediment_scaffold.depmid,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                Self::channel_wave_state_symbol(request.node_id, "depsid"),
                sediment_scaffold.depsid,
                Some(0.0),
                None,
            ),
        ];
        let class_count_scalar = f64::from(
            u32::try_from(sediment_publication.particle_flow_fractions.len()).unwrap_or(u32::MAX),
        );
        state_updates.push(WritebackField::bounded(
            Self::channel_wave_state_symbol(request.node_id, "particle_class_count"),
            class_count_scalar,
            Some(0.0),
            None,
        ));
        for (class_index, (fraction, diameter)) in sediment_publication
            .particle_flow_fractions
            .iter()
            .zip(sediment_publication.particle_diameters_m.iter())
            .enumerate()
        {
            let class = class_index + 1;
            state_updates.push(WritebackField::bounded(
                Self::channel_wave_state_symbol(
                    request.node_id,
                    &format!("particle_flow_fraction_{class:04}"),
                ),
                *fraction,
                Some(0.0),
                Some(1.0),
            ));
            state_updates.push(WritebackField::bounded(
                Self::channel_wave_state_symbol(
                    request.node_id,
                    &format!("particle_diameter_m_{class:04}"),
                ),
                *diameter,
                Some(WS10_ZERO_THRESHOLD),
                None,
            ));
        }
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

        let sum_flagm1 = df_flagm1.iter().sum::<f64>();
        let sum_flagm2 = df_flagm2.iter().sum::<f64>();
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
