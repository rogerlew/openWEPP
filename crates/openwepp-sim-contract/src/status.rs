use std::error::Error;
use std::fmt;

/// High-level simulation lifecycle phase for status reporting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SimulationPhase {
    PreExecutionValidation,
    HillslopeKernel,
    WatershedKernel,
    SummaryAccumulator,
    CompatibilityAdapter,
}

impl SimulationPhase {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreExecutionValidation => "pre_execution_validation",
            Self::HillslopeKernel => "hillslope_kernel",
            Self::WatershedKernel => "watershed_kernel",
            Self::SummaryAccumulator => "summary_accumulator",
            Self::CompatibilityAdapter => "compatibility_adapter",
        }
    }
}

/// Coarse status classification for deterministic gate behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatusClassification {
    Nominal,
    Advisory,
    Failure,
}

/// Severity mapped from [`StatusClassification`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatusSeverity {
    Ok,
    Warning,
    Error,
}

impl StatusClassification {
    #[must_use]
    pub const fn severity(self) -> StatusSeverity {
        match self {
            Self::Nominal => StatusSeverity::Ok,
            Self::Advisory => StatusSeverity::Warning,
            Self::Failure => StatusSeverity::Error,
        }
    }
}

/// Boundary class aligned with legacy WEPP kernel status semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoundaryClass {
    Ok,
    Dry,
    Saturated,
    NegativeInput,
    ZeroGeometry,
    ModeMismatch,
    CapBinding,
    TopologyInvalid,
    ClosureViolation,
    DomainViolation,
    NonFinite,
    MissingRequiredInput,
}

#[derive(Debug, Clone, Copy)]
struct BoundaryClassDefinition {
    label: &'static str,
    classification: StatusClassification,
}

impl BoundaryClass {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.definition().label
    }

    #[must_use]
    pub const fn classification(self) -> StatusClassification {
        self.definition().classification
    }

    const fn definition(self) -> BoundaryClassDefinition {
        match self {
            Self::Ok => BoundaryClassDefinition {
                label: "OK",
                classification: StatusClassification::Nominal,
            },
            Self::Dry => BoundaryClassDefinition {
                label: "DRY",
                classification: StatusClassification::Advisory,
            },
            Self::Saturated => BoundaryClassDefinition {
                label: "SATURATED",
                classification: StatusClassification::Advisory,
            },
            Self::NegativeInput => BoundaryClassDefinition {
                label: "NEGATIVE_INPUT",
                classification: StatusClassification::Failure,
            },
            Self::ZeroGeometry => BoundaryClassDefinition {
                label: "ZERO_GEOMETRY",
                classification: StatusClassification::Failure,
            },
            Self::ModeMismatch => BoundaryClassDefinition {
                label: "MODE_MISMATCH",
                classification: StatusClassification::Failure,
            },
            Self::CapBinding => BoundaryClassDefinition {
                label: "CAP_BINDING",
                classification: StatusClassification::Advisory,
            },
            Self::TopologyInvalid => BoundaryClassDefinition {
                label: "TOPOLOGY_INVALID",
                classification: StatusClassification::Failure,
            },
            Self::ClosureViolation => BoundaryClassDefinition {
                label: "CLOSURE_VIOLATION",
                classification: StatusClassification::Failure,
            },
            Self::DomainViolation => BoundaryClassDefinition {
                label: "DOMAIN_VIOLATION",
                classification: StatusClassification::Failure,
            },
            Self::NonFinite => BoundaryClassDefinition {
                label: "NON_FINITE",
                classification: StatusClassification::Failure,
            },
            Self::MissingRequiredInput => BoundaryClassDefinition {
                label: "MISSING_REQUIRED_INPUT",
                classification: StatusClassification::Failure,
            },
        }
    }
}

/// Clamp class aligned with legacy WEPP kernel clamp semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClampClass {
    None,
    LowerBoundClamp,
    UpperBoundClamp,
    QcapSoftLimit,
    ProfileShortfall,
}

impl ClampClass {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "NONE",
            Self::LowerBoundClamp => "LOWER_BOUND_CLAMP",
            Self::UpperBoundClamp => "UPPER_BOUND_CLAMP",
            Self::QcapSoftLimit => "QCAP_SOFT_LIMIT",
            Self::ProfileShortfall => "PROFILE_SHORTFALL",
        }
    }

    #[must_use]
    pub const fn is_clamped(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// Typed simulation status used by kernel and orchestrator boundaries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationStatus {
    phase: SimulationPhase,
    ok: bool,
    finite_ok: bool,
    domain_ok: bool,
    boundary_class: BoundaryClass,
    clamp_class: ClampClass,
    message_id: String,
}

/// Status construction and classification errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatusError {
    MessageIdEmpty,
    AdvisoryBoundaryMustNotBeFailure { boundary_class: BoundaryClass },
}

impl fmt::Display for StatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MessageIdEmpty => f.write_str("message_id must not be empty"),
            Self::AdvisoryBoundaryMustNotBeFailure { boundary_class } => {
                write!(
                    f,
                    "advisory status cannot use failure boundary class {}",
                    boundary_class.as_str()
                )
            }
        }
    }
}

impl Error for StatusError {}

impl SimulationStatus {
    /// Construct a status record with explicit fields.
    ///
    /// # Errors
    ///
    /// Returns `StatusError::MessageIdEmpty` when `message_id` is empty.
    pub fn new(
        phase: SimulationPhase,
        ok: bool,
        finite_ok: bool,
        domain_ok: bool,
        boundary_class: BoundaryClass,
        clamp_class: ClampClass,
        message_id: impl Into<String>,
    ) -> Result<Self, StatusError> {
        let message_id = message_id.into();
        if message_id.trim().is_empty() {
            return Err(StatusError::MessageIdEmpty);
        }

        Ok(Self {
            phase,
            ok,
            finite_ok,
            domain_ok,
            boundary_class,
            clamp_class,
            message_id,
        })
    }

    /// Construct a nominal status.
    ///
    /// # Errors
    ///
    /// Returns `StatusError::MessageIdEmpty` when `message_id` is empty.
    pub fn ok(phase: SimulationPhase, message_id: impl Into<String>) -> Result<Self, StatusError> {
        Self::new(
            phase,
            true,
            true,
            true,
            BoundaryClass::Ok,
            ClampClass::None,
            message_id,
        )
    }

    /// Construct an advisory status with explicit boundary and clamp classes.
    ///
    /// # Errors
    ///
    /// Returns `StatusError::MessageIdEmpty` when `message_id` is empty.
    /// Returns `StatusError::AdvisoryBoundaryMustNotBeFailure` when
    /// `boundary_class` is a failure class.
    pub fn advisory(
        phase: SimulationPhase,
        boundary_class: BoundaryClass,
        clamp_class: ClampClass,
        message_id: impl Into<String>,
    ) -> Result<Self, StatusError> {
        if boundary_class.classification() == StatusClassification::Failure {
            return Err(StatusError::AdvisoryBoundaryMustNotBeFailure { boundary_class });
        }

        Self::new(
            phase,
            true,
            true,
            true,
            boundary_class,
            clamp_class,
            message_id,
        )
    }

    /// Construct an explicit failure status.
    ///
    /// # Errors
    ///
    /// Returns `StatusError::MessageIdEmpty` when `message_id` is empty.
    pub fn failure(
        phase: SimulationPhase,
        finite_ok: bool,
        domain_ok: bool,
        boundary_class: BoundaryClass,
        message_id: impl Into<String>,
    ) -> Result<Self, StatusError> {
        Self::new(
            phase,
            false,
            finite_ok,
            domain_ok,
            boundary_class,
            ClampClass::None,
            message_id,
        )
    }

    /// Construct a non-finite failure status.
    ///
    /// # Errors
    ///
    /// Returns `StatusError::MessageIdEmpty` when `message_id` is empty.
    pub fn non_finite_failure(
        phase: SimulationPhase,
        message_id: impl Into<String>,
    ) -> Result<Self, StatusError> {
        Self::failure(phase, false, false, BoundaryClass::NonFinite, message_id)
    }

    /// Construct a domain failure status.
    ///
    /// # Errors
    ///
    /// Returns `StatusError::MessageIdEmpty` when `message_id` is empty.
    pub fn domain_failure(
        phase: SimulationPhase,
        boundary_class: BoundaryClass,
        message_id: impl Into<String>,
    ) -> Result<Self, StatusError> {
        Self::failure(phase, true, false, boundary_class, message_id)
    }

    #[must_use]
    pub const fn phase(&self) -> SimulationPhase {
        self.phase
    }

    #[must_use]
    pub const fn ok_flag(&self) -> bool {
        self.ok
    }

    #[must_use]
    pub const fn finite_ok(&self) -> bool {
        self.finite_ok
    }

    #[must_use]
    pub const fn domain_ok(&self) -> bool {
        self.domain_ok
    }

    #[must_use]
    pub const fn boundary_class(&self) -> BoundaryClass {
        self.boundary_class
    }

    #[must_use]
    pub const fn clamp_class(&self) -> ClampClass {
        self.clamp_class
    }

    #[must_use]
    pub fn message_id(&self) -> &str {
        self.message_id.as_str()
    }

    #[must_use]
    pub const fn classification(&self) -> StatusClassification {
        if !self.ok || !self.finite_ok || !self.domain_ok {
            return StatusClassification::Failure;
        }

        match self.boundary_class.classification() {
            StatusClassification::Failure => StatusClassification::Failure,
            StatusClassification::Advisory => StatusClassification::Advisory,
            StatusClassification::Nominal => {
                if self.clamp_class.is_clamped() {
                    StatusClassification::Advisory
                } else {
                    StatusClassification::Nominal
                }
            }
        }
    }

    #[must_use]
    pub const fn severity(&self) -> StatusSeverity {
        self.classification().severity()
    }
}
