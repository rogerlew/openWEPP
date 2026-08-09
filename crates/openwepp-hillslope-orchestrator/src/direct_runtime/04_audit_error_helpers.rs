#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectRuntimeAuditSnapshot {
    pub run_frame_constructions: u64,
    pub day_frame_constructions: u64,
    pub day_frame_commits: u64,
    pub executor_constructions: u64,
    pub skeleton_runs: u64,
    pub publication_capture_runs: u64,
    pub phase_view_constructions: u64,
    /// DC01: erod14 decreasing-flow qin clamps (INV-RUNOFFPART-030 hold).
    pub erod14_qin_clamped_events: u64,
    /// E.3 (SC-SED-001 INV-SED-016 (f)): Wave-1 hour quanta refused by the
    /// flux-consistency diagnostic and skipped (zero sediment contribution).
    pub wave1_flux_refused_quanta: u64,
    pub phase_span_runs: u64,
    pub direct_phase_entries: u64,
    pub direct_compute_operations: u64,
    pub direct_state_mutations: u64,
    pub downstream_operand_productions: u64,
    pub shadow_projections: u64,
    pub compatibility_edge_invocations: u64,
    pub ksatadj_effective_conductivity_evaluations: u64,
}

#[must_use]
pub fn direct_runtime_audit_snapshot() -> DirectRuntimeAuditSnapshot {
    DIRECT_AUDIT.snapshot()
}

pub fn reset_direct_runtime_audit_counters() {
    DIRECT_AUDIT.reset();
}

pub fn record_direct_runtime_compatibility_edge_invocation() {
    DIRECT_AUDIT.record_compatibility_edge_invocation();
}

pub fn record_direct_runtime_ksatadj_effective_conductivity_evaluation() {
    DIRECT_AUDIT.record_ksatadj_effective_conductivity_evaluation();
}

struct DirectRuntimeAuditCounters {
    run_frame_constructions: AtomicU64,
    day_frame_constructions: AtomicU64,
    day_frame_commits: AtomicU64,
    executor_constructions: AtomicU64,
    skeleton_runs: AtomicU64,
    publication_capture_runs: AtomicU64,
    phase_view_constructions: AtomicU64,
    erod14_qin_clamped_events: AtomicU64,
    wave1_flux_refused_quanta: AtomicU64,
    phase_span_runs: AtomicU64,
    direct_phase_entries: AtomicU64,
    direct_compute_operations: AtomicU64,
    direct_state_mutations: AtomicU64,
    downstream_operand_productions: AtomicU64,
    shadow_projections: AtomicU64,
    compatibility_edge_invocations: AtomicU64,
    ksatadj_effective_conductivity_evaluations: AtomicU64,
}

impl DirectRuntimeAuditCounters {
    const fn new() -> Self {
        Self {
            run_frame_constructions: AtomicU64::new(0),
            day_frame_constructions: AtomicU64::new(0),
            day_frame_commits: AtomicU64::new(0),
            executor_constructions: AtomicU64::new(0),
            skeleton_runs: AtomicU64::new(0),
            publication_capture_runs: AtomicU64::new(0),
            phase_view_constructions: AtomicU64::new(0),
            erod14_qin_clamped_events: AtomicU64::new(0),
            wave1_flux_refused_quanta: AtomicU64::new(0),
            phase_span_runs: AtomicU64::new(0),
            direct_phase_entries: AtomicU64::new(0),
            direct_compute_operations: AtomicU64::new(0),
            direct_state_mutations: AtomicU64::new(0),
            downstream_operand_productions: AtomicU64::new(0),
            shadow_projections: AtomicU64::new(0),
            compatibility_edge_invocations: AtomicU64::new(0),
            ksatadj_effective_conductivity_evaluations: AtomicU64::new(0),
        }
    }

    fn record_run_frame_construction(&self) {
        self.run_frame_constructions.fetch_add(1, Ordering::Relaxed);
    }

    fn record_day_frame_construction(&self) {
        self.day_frame_constructions.fetch_add(1, Ordering::Relaxed);
    }

    fn record_day_frame_commit(&self) {
        self.day_frame_commits.fetch_add(1, Ordering::Relaxed);
    }

    fn record_executor_construction(&self) {
        self.executor_constructions.fetch_add(1, Ordering::Relaxed);
    }

    fn record_skeleton_run(&self) {
        self.skeleton_runs.fetch_add(1, Ordering::Relaxed);
    }

    fn record_publication_capture_run(&self) {
        self.publication_capture_runs
            .fetch_add(1, Ordering::Relaxed);
    }

    fn record_phase_view_construction(&self) {
        self.phase_view_constructions
            .fetch_add(1, Ordering::Relaxed);
    }

    fn record_phase_span_run(&self) {
        self.phase_span_runs.fetch_add(1, Ordering::Relaxed);
    }

    fn record_direct_phase_entry(&self) {
        self.direct_phase_entries.fetch_add(1, Ordering::Relaxed);
    }

    fn record_direct_compute_operation(&self) {
        self.direct_compute_operations
            .fetch_add(1, Ordering::Relaxed);
    }

    fn record_direct_state_mutation(&self) {
        self.direct_state_mutations.fetch_add(1, Ordering::Relaxed);
    }

    fn record_wave1_flux_refused_quantum(&self) {
        self.wave1_flux_refused_quanta.fetch_add(1, Ordering::Relaxed);
    }

    fn record_downstream_operand_production(&self) {
        self.downstream_operand_productions
            .fetch_add(1, Ordering::Relaxed);
    }

    fn record_shadow_projection(&self) {
        self.shadow_projections.fetch_add(1, Ordering::Relaxed);
    }

    fn record_compatibility_edge_invocation(&self) {
        self.compatibility_edge_invocations
            .fetch_add(1, Ordering::Relaxed);
    }

    fn record_ksatadj_effective_conductivity_evaluation(&self) {
        self.ksatadj_effective_conductivity_evaluations
            .fetch_add(1, Ordering::Relaxed);
    }

    fn snapshot(&self) -> DirectRuntimeAuditSnapshot {
        DirectRuntimeAuditSnapshot {
            run_frame_constructions: self.run_frame_constructions.load(Ordering::Relaxed),
            day_frame_constructions: self.day_frame_constructions.load(Ordering::Relaxed),
            day_frame_commits: self.day_frame_commits.load(Ordering::Relaxed),
            executor_constructions: self.executor_constructions.load(Ordering::Relaxed),
            skeleton_runs: self.skeleton_runs.load(Ordering::Relaxed),
            publication_capture_runs: self.publication_capture_runs.load(Ordering::Relaxed),
            phase_view_constructions: self.phase_view_constructions.load(Ordering::Relaxed),
            erod14_qin_clamped_events: self
                .erod14_qin_clamped_events
                .load(std::sync::atomic::Ordering::Relaxed),
            wave1_flux_refused_quanta: self
                .wave1_flux_refused_quanta
                .load(std::sync::atomic::Ordering::Relaxed),
            phase_span_runs: self.phase_span_runs.load(Ordering::Relaxed),
            direct_phase_entries: self.direct_phase_entries.load(Ordering::Relaxed),
            direct_compute_operations: self.direct_compute_operations.load(Ordering::Relaxed),
            direct_state_mutations: self.direct_state_mutations.load(Ordering::Relaxed),
            downstream_operand_productions: self
                .downstream_operand_productions
                .load(Ordering::Relaxed),
            shadow_projections: self.shadow_projections.load(Ordering::Relaxed),
            compatibility_edge_invocations: self
                .compatibility_edge_invocations
                .load(Ordering::Relaxed),
            ksatadj_effective_conductivity_evaluations: self
                .ksatadj_effective_conductivity_evaluations
                .load(Ordering::Relaxed),
        }
    }

    fn reset(&self) {
        self.run_frame_constructions.store(0, Ordering::Relaxed);
        self.day_frame_constructions.store(0, Ordering::Relaxed);
        self.day_frame_commits.store(0, Ordering::Relaxed);
        self.executor_constructions.store(0, Ordering::Relaxed);
        self.skeleton_runs.store(0, Ordering::Relaxed);
        self.publication_capture_runs.store(0, Ordering::Relaxed);
        self.phase_view_constructions.store(0, Ordering::Relaxed);
        self.wave1_flux_refused_quanta.store(0, Ordering::Relaxed);
        self.phase_span_runs.store(0, Ordering::Relaxed);
        self.direct_phase_entries.store(0, Ordering::Relaxed);
        self.direct_compute_operations.store(0, Ordering::Relaxed);
        self.direct_state_mutations.store(0, Ordering::Relaxed);
        self.downstream_operand_productions
            .store(0, Ordering::Relaxed);
        self.shadow_projections.store(0, Ordering::Relaxed);
        self.compatibility_edge_invocations
            .store(0, Ordering::Relaxed);
        self.ksatadj_effective_conductivity_evaluations
            .store(0, Ordering::Relaxed);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DirectRuntimeError {
    InvalidLaneCount {
        lane_count: usize,
    },
    InvalidDayCount {
        day_count: usize,
    },
    LaneIdOverflow {
        lane_index: usize,
    },
    FrameLaneCountMismatch {
        identity_lane_count: usize,
        actual_lane_count: usize,
    },
    InvalidLaneTopology {
        lane_index: usize,
        lane_id: u32,
        upstream_lane_id: u32,
        downstream_lane_id: u32,
    },
    InvalidLaneOutletCount {
        outlet_count: usize,
    },
    CalendarDayCountMismatch {
        identity_day_count: usize,
        calendar_day_count: usize,
    },
    PublicationRowCountMismatch {
        expected_row_count: usize,
        actual_row_count: usize,
    },
    InvalidPublicationArea {
        lane_id: u32,
        area_m2: f64,
    },
    LaneIndexOutOfRange {
        lane_index: usize,
        lane_count: usize,
    },
    DayIndexOutOfRange {
        day_index: usize,
        day_count: usize,
    },
    MissingDirectUpstream {
        upstream: &'static str,
    },
    NonFiniteDirectValue {
        field: &'static str,
    },
    NegativeDirectValue {
        field: &'static str,
    },
    DirectDomainViolation {
        field: &'static str,
    },
    DirectKernelGuardFailure {
        phase: &'static str,
        detail: String,
    },
    HydrologyKernelGuard(Box<Wb11HydrologyKernelGuardError>),
    PublicationDayInputBuildFailure {
        detail: String,
    },
    PublicationSinkFailure {
        detail: String,
    },
    DirectClosureToleranceExceeded {
        field: &'static str,
    },
    SnowMassTransitionLedger(DirectSnowMassTransitionLedgerError),
    DirectDayExecutionFailure {
        lane_index: usize,
        day_index: usize,
        detail: String,
    },
}

impl fmt::Display for DirectRuntimeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_parts().fmt(formatter)
    }
}

impl DirectRuntimeError {
    fn display_parts(&self) -> DirectRuntimeErrorDisplay<'_> {
        use DirectRuntimeGuardDisplay as Guard;
        use DirectRuntimeIdentityDisplay as Identity;
        use DirectRuntimePublicationDisplay as Publication;

        match self {
            Self::InvalidLaneCount { lane_count } => Identity::InvalidLaneCount(*lane_count).into(),
            Self::InvalidDayCount { day_count } => Identity::InvalidDayCount(*day_count).into(),
            Self::LaneIdOverflow { lane_index } => Identity::LaneIdOverflow(*lane_index).into(),
            Self::FrameLaneCountMismatch {
                identity_lane_count,
                actual_lane_count,
            } => Identity::FrameLaneCountMismatch {
                identity_lane_count: *identity_lane_count,
                actual_lane_count: *actual_lane_count,
            }
            .into(),
            Self::InvalidLaneTopology {
                lane_index,
                lane_id,
                upstream_lane_id,
                downstream_lane_id,
            } => Identity::InvalidLaneTopology {
                lane_index: *lane_index,
                lane_id: *lane_id,
                upstream_lane_id: *upstream_lane_id,
                downstream_lane_id: *downstream_lane_id,
            }
            .into(),
            Self::InvalidLaneOutletCount { outlet_count } => {
                Identity::InvalidLaneOutletCount(*outlet_count).into()
            }
            Self::CalendarDayCountMismatch {
                identity_day_count,
                calendar_day_count,
            } => Publication::CalendarDayCountMismatch {
                identity_day_count: *identity_day_count,
                calendar_day_count: *calendar_day_count,
            }
            .into(),
            Self::PublicationRowCountMismatch {
                expected_row_count,
                actual_row_count,
            } => Publication::RowCountMismatch {
                expected_row_count: *expected_row_count,
                actual_row_count: *actual_row_count,
            }
            .into(),
            Self::InvalidPublicationArea { lane_id, area_m2 } => Publication::InvalidArea {
                lane_id: *lane_id,
                area_m2: *area_m2,
            }
            .into(),
            Self::LaneIndexOutOfRange {
                lane_index,
                lane_count,
            } => Publication::LaneIndexOutOfRange {
                lane_index: *lane_index,
                lane_count: *lane_count,
            }
            .into(),
            Self::DayIndexOutOfRange {
                day_index,
                day_count,
            } => Publication::DayIndexOutOfRange {
                day_index: *day_index,
                day_count: *day_count,
            }
            .into(),
            Self::MissingDirectUpstream { upstream } => Guard::MissingUpstream(upstream).into(),
            Self::NonFiniteDirectValue { field } => Guard::NonFinite(field).into(),
            Self::NegativeDirectValue { field } => Guard::Negative(field).into(),
            Self::DirectDomainViolation { field } => Guard::Domain(field).into(),
            Self::DirectKernelGuardFailure { phase, detail } => Guard::KernelFailure {
                phase,
                detail: detail.as_str(),
            }
            .into(),
            Self::HydrologyKernelGuard(source) => {
                DirectRuntimeErrorDisplay::HydrologyGuard(source.as_ref())
            }
            Self::PublicationDayInputBuildFailure { detail } => {
                Guard::DayInputBuildFailure(detail.as_str()).into()
            }
            Self::PublicationSinkFailure { detail } => Guard::SinkFailure(detail.as_str()).into(),
            Self::DirectClosureToleranceExceeded { field } => Guard::ClosureTolerance(field).into(),
            Self::SnowMassTransitionLedger(source) => Guard::SnowMassTransitionLedger(source).into(),
            Self::DirectDayExecutionFailure {
                lane_index,
                day_index,
                detail,
            } => Guard::DayExecutionFailure {
                lane_index: *lane_index,
                day_index: *day_index,
                detail: detail.as_str(),
            }
            .into(),
        }
    }
}

enum DirectRuntimeErrorDisplay<'a> {
    Identity(DirectRuntimeIdentityDisplay),
    Publication(DirectRuntimePublicationDisplay),
    Guard(DirectRuntimeGuardDisplay<'a>),
    HydrologyGuard(&'a Wb11HydrologyKernelGuardError),
}

impl From<DirectRuntimeIdentityDisplay> for DirectRuntimeErrorDisplay<'_> {
    fn from(display: DirectRuntimeIdentityDisplay) -> Self {
        Self::Identity(display)
    }
}

impl From<DirectRuntimePublicationDisplay> for DirectRuntimeErrorDisplay<'_> {
    fn from(display: DirectRuntimePublicationDisplay) -> Self {
        Self::Publication(display)
    }
}

impl<'a> From<DirectRuntimeGuardDisplay<'a>> for DirectRuntimeErrorDisplay<'a> {
    fn from(display: DirectRuntimeGuardDisplay<'a>) -> Self {
        Self::Guard(display)
    }
}

impl fmt::Display for DirectRuntimeErrorDisplay<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Identity(display) => display.fmt(formatter),
            Self::Publication(display) => display.fmt(formatter),
            Self::Guard(display) => display.fmt(formatter),
            Self::HydrologyGuard(display) => display.fmt(formatter),
        }
    }
}

enum DirectRuntimeIdentityDisplay {
    InvalidLaneCount(usize),
    InvalidDayCount(usize),
    LaneIdOverflow(usize),
    FrameLaneCountMismatch {
        identity_lane_count: usize,
        actual_lane_count: usize,
    },
    InvalidLaneTopology {
        lane_index: usize,
        lane_id: u32,
        upstream_lane_id: u32,
        downstream_lane_id: u32,
    },
    InvalidLaneOutletCount(usize),
}

impl fmt::Display for DirectRuntimeIdentityDisplay {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLaneCount(lane_count) => {
                write!(
                    formatter,
                    "direct runtime requires at least one lane, observed {lane_count}"
                )
            }
            Self::InvalidDayCount(day_count) => {
                write!(
                    formatter,
                    "direct runtime requires at least one day, observed {day_count}"
                )
            }
            Self::LaneIdOverflow(lane_index) => {
                write!(
                    formatter,
                    "direct runtime lane index {lane_index} cannot be represented as a u32 lane id"
                )
            }
            Self::FrameLaneCountMismatch {
                identity_lane_count,
                actual_lane_count,
            } => {
                write!(
                    formatter,
                    "direct runtime frame lane count {actual_lane_count} does not match identity lane count {identity_lane_count}"
                )
            }
            Self::InvalidLaneTopology {
                lane_index,
                lane_id,
                upstream_lane_id,
                downstream_lane_id,
            } => {
                write!(
                    formatter,
                    "direct runtime lane topology is invalid at index {lane_index}: lane {lane_id}, upstream {upstream_lane_id}, downstream {downstream_lane_id}"
                )
            }
            Self::InvalidLaneOutletCount(outlet_count) => {
                write!(
                    formatter,
                    "direct runtime requires exactly one lane outlet, observed {outlet_count}"
                )
            }
        }
    }
}

enum DirectRuntimePublicationDisplay {
    CalendarDayCountMismatch {
        identity_day_count: usize,
        calendar_day_count: usize,
    },
    RowCountMismatch {
        expected_row_count: usize,
        actual_row_count: usize,
    },
    InvalidArea {
        lane_id: u32,
        area_m2: f64,
    },
    LaneIndexOutOfRange {
        lane_index: usize,
        lane_count: usize,
    },
    DayIndexOutOfRange {
        day_index: usize,
        day_count: usize,
    },
}

impl fmt::Display for DirectRuntimePublicationDisplay {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CalendarDayCountMismatch {
                identity_day_count,
                calendar_day_count,
            } => {
                write!(
                    formatter,
                    "direct publication calendar day count {calendar_day_count} does not match identity day count {identity_day_count}"
                )
            }
            Self::RowCountMismatch {
                expected_row_count,
                actual_row_count,
            } => {
                write!(
                    formatter,
                    "direct publication row count {actual_row_count} does not match expected row count {expected_row_count}"
                )
            }
            Self::InvalidArea { lane_id, area_m2 } => {
                write!(
                    formatter,
                    "direct publication lane {lane_id} area must be finite and > 0.0 m^2, observed {area_m2}"
                )
            }
            Self::LaneIndexOutOfRange {
                lane_index,
                lane_count,
            } => {
                write!(
                    formatter,
                    "direct runtime lane index {lane_index} is outside lane count {lane_count}"
                )
            }
            Self::DayIndexOutOfRange {
                day_index,
                day_count,
            } => {
                write!(
                    formatter,
                    "direct runtime day index {day_index} is outside day count {day_count}"
                )
            }
        }
    }
}

enum DirectRuntimeGuardDisplay<'a> {
    MissingUpstream(&'static str),
    NonFinite(&'static str),
    Negative(&'static str),
    Domain(&'static str),
    KernelFailure {
        phase: &'static str,
        detail: &'a str,
    },
    DayInputBuildFailure(&'a str),
    SinkFailure(&'a str),
    ClosureTolerance(&'static str),
    SnowMassTransitionLedger(&'a DirectSnowMassTransitionLedgerError),
    DayExecutionFailure {
        lane_index: usize,
        day_index: usize,
        detail: &'a str,
    },
}

impl fmt::Display for DirectRuntimeGuardDisplay<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingUpstream(upstream) => {
                write!(
                    formatter,
                    "direct runtime upstream span {upstream} must execute before this span"
                )
            }
            Self::NonFinite(field) => {
                write!(formatter, "direct runtime field {field} must be finite")
            }
            Self::Negative(field) => {
                write!(
                    formatter,
                    "direct runtime field {field} must be nonnegative"
                )
            }
            Self::Domain(field) => {
                write!(
                    formatter,
                    "direct runtime field {field} violates its direct-domain constraints"
                )
            }
            Self::KernelFailure { phase, detail } => {
                write!(
                    formatter,
                    "direct runtime kernel guard failed in {phase}: {detail}"
                )
            }
            Self::DayInputBuildFailure(detail) => {
                write!(
                    formatter,
                    "direct publication day-input builder failed: {detail}"
                )
            }
            Self::SinkFailure(detail) => {
                write!(formatter, "direct publication sink failed: {detail}")
            }
            Self::ClosureTolerance(field) => {
                write!(
                    formatter,
                    "direct runtime field {field} exceeds declared closure tolerance"
                )
            }
            Self::SnowMassTransitionLedger(source) => {
                write!(formatter, "direct runtime snow mass-transition ledger validation failed: {source}")
            }
            Self::DayExecutionFailure {
                lane_index,
                day_index,
                detail,
            } => {
                write!(
                    formatter,
                    "direct runtime day execution failed at lane {} day {}: {detail}",
                    lane_index + 1,
                    day_index + 1
                )
            }
        }
    }
}

impl Error for DirectRuntimeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::SnowMassTransitionLedger(source) => Some(source),
            Self::HydrologyKernelGuard(source) => Some(source.as_ref()),
            _ => None,
        }
    }
}

fn validate_finite(field: &'static str, value: f64) -> Result<(), DirectRuntimeError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(DirectRuntimeError::NonFiniteDirectValue { field })
    }
}

fn validate_nonnegative_direct_m(
    field: &'static str,
    value: f64,
) -> Result<(), DirectRuntimeError> {
    validate_finite(field, value)?;
    if value >= 0.0 {
        Ok(())
    } else {
        Err(DirectRuntimeError::NegativeDirectValue { field })
    }
}

fn m_to_mm(value_m: f64) -> Result<f64, DirectRuntimeError> {
    validate_nonnegative_direct_m("publication.depth_m", value_m)?;
    let value_mm = value_m * 1_000.0;
    validate_finite("publication.depth_mm", value_mm)?;
    Ok(value_mm)
}

fn option_m_to_mm(value_m: Option<f64>) -> Result<Option<f64>, DirectRuntimeError> {
    value_m.map(m_to_mm).transpose()
}

fn publication_mm_to_volume_m3(
    field: &'static str,
    depth_mm: f64,
    area_m2: f64,
) -> Result<f64, DirectRuntimeError> {
    validate_nonnegative_direct_m("publication.depth_mm", depth_mm)?;
    validate_finite("publication.area_m2", area_m2)?;
    if area_m2 <= 0.0 {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "publication.area_m2",
        });
    }
    let volume_m3 = depth_mm * area_m2 / 1_000.0;
    validate_finite(field, volume_m3)?;
    Ok(volume_m3)
}

fn publication_volume_m3_to_mm(
    field: &'static str,
    volume_m3: f64,
    area_m2: f64,
) -> Result<f64, DirectRuntimeError> {
    validate_nonnegative_direct_m(field, volume_m3)?;
    validate_finite("publication.area_m2", area_m2)?;
    if area_m2 <= 0.0 {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "publication.area_m2",
        });
    }
    let depth_mm = volume_m3 * 1_000.0 / area_m2;
    validate_finite(field, depth_mm)?;
    Ok(depth_mm)
}

fn sum_nonnegative_direct_m(
    field: &'static str,
    values: &[f64; DIRECT_TRANSFER_HOUR_COUNT],
) -> Result<f64, DirectRuntimeError> {
    let mut total = 0.0;
    for value in values {
        validate_nonnegative_direct_m(field, *value)?;
        total += value;
        validate_finite(field, total)?;
    }
    Ok(total)
}

fn scaled_direct_transfer_total_m(
    field: &'static str,
    raw_total_m: f64,
    upstream_area_ratio: f64,
) -> Result<f64, DirectRuntimeError> {
    validate_nonnegative_direct_m(field, raw_total_m)?;
    validate_nonnegative_direct_m("transfer.upstream_area_ratio", upstream_area_ratio)?;
    if raw_total_m > WB11_ZERO_THRESHOLD && upstream_area_ratio <= WB11_ZERO_THRESHOLD {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "transfer.upstream_area_ratio",
        });
    }
    let scaled_total_m = raw_total_m * upstream_area_ratio;
    validate_finite(field, scaled_total_m)?;
    Ok(scaled_total_m)
}

fn sum_finite_direct_m(field: &'static str, values: &[f64]) -> Result<f64, DirectRuntimeError> {
    let mut total = 0.0;
    for value in values {
        total += value;
        validate_finite(field, total)?;
    }
    Ok(total)
}

#[cfg(test)]
mod cqr_direct_runtime_error_display_tests {
    use super::{
        DirectRuntimeAuditCounters, DirectRuntimeError, DirectSnowMassTransitionLedgerError,
    };

    fn assert_display(error: &DirectRuntimeError, expected: &str) {
        assert_eq!(error.to_string(), expected);
    }

    #[test]
    fn snow_mass_transition_error_remains_a_typed_direct_runtime_source() {
        let source = DirectSnowMassTransitionLedgerError::Negative {
            field: "snowpack_swe_loss_m",
        };
        let error = DirectRuntimeError::SnowMassTransitionLedger(source);
        assert!(error.to_string().contains("snowpack_swe_loss_m"));
        assert_eq!(
            std::error::Error::source(&error).map(ToString::to_string),
            Some(source.to_string())
        );
    }

    #[test]
    fn direct_runtime_error_display_preserves_identity_and_topology_diagnostics() {
        assert_display(
            &DirectRuntimeError::InvalidLaneCount { lane_count: 0 },
            "direct runtime requires at least one lane, observed 0",
        );
        assert_display(
            &DirectRuntimeError::InvalidDayCount { day_count: 0 },
            "direct runtime requires at least one day, observed 0",
        );
        assert_display(
            &DirectRuntimeError::LaneIdOverflow { lane_index: 42 },
            "direct runtime lane index 42 cannot be represented as a u32 lane id",
        );
        assert_display(
            &DirectRuntimeError::FrameLaneCountMismatch {
                identity_lane_count: 3,
                actual_lane_count: 2,
            },
            "direct runtime frame lane count 2 does not match identity lane count 3",
        );
        assert_display(
            &DirectRuntimeError::InvalidLaneTopology {
                lane_index: 4,
                lane_id: 7,
                upstream_lane_id: 6,
                downstream_lane_id: 0,
            },
            "direct runtime lane topology is invalid at index 4: lane 7, upstream 6, downstream 0",
        );
    }

    #[test]
    fn direct_runtime_error_display_preserves_frame_and_publication_diagnostics() {
        assert_display(
            &DirectRuntimeError::InvalidLaneOutletCount { outlet_count: 2 },
            "direct runtime requires exactly one lane outlet, observed 2",
        );
        assert_display(
            &DirectRuntimeError::CalendarDayCountMismatch {
                identity_day_count: 365,
                calendar_day_count: 364,
            },
            "direct publication calendar day count 364 does not match identity day count 365",
        );
        assert_display(
            &DirectRuntimeError::PublicationRowCountMismatch {
                expected_row_count: 8,
                actual_row_count: 7,
            },
            "direct publication row count 7 does not match expected row count 8",
        );
        assert_display(
            &DirectRuntimeError::InvalidPublicationArea {
                lane_id: 3,
                area_m2: 12.5,
            },
            "direct publication lane 3 area must be finite and > 0.0 m^2, observed 12.5",
        );
        assert_display(
            &DirectRuntimeError::LaneIndexOutOfRange {
                lane_index: 2,
                lane_count: 2,
            },
            "direct runtime lane index 2 is outside lane count 2",
        );
    }

    #[test]
    fn direct_runtime_error_display_preserves_index_and_domain_diagnostics() {
        assert_display(
            &DirectRuntimeError::DayIndexOutOfRange {
                day_index: 5,
                day_count: 5,
            },
            "direct runtime day index 5 is outside day count 5",
        );
        assert_display(
            &DirectRuntimeError::MissingDirectUpstream {
                upstream: "R5B normalization phase",
            },
            "direct runtime upstream span R5B normalization phase must execute before this span",
        );
        assert_display(
            &DirectRuntimeError::NonFiniteDirectValue {
                field: "forcing.precipitation_m",
            },
            "direct runtime field forcing.precipitation_m must be finite",
        );
        assert_display(
            &DirectRuntimeError::NegativeDirectValue {
                field: "transfer.surface_carry_m",
            },
            "direct runtime field transfer.surface_carry_m must be nonnegative",
        );
        assert_display(
            &DirectRuntimeError::DirectDomainViolation {
                field: "publication.area_m2",
            },
            "direct runtime field publication.area_m2 violates its direct-domain constraints",
        );
    }

    #[test]
    fn direct_runtime_error_display_preserves_guard_and_execution_diagnostics() {
        assert_display(
            &DirectRuntimeError::DirectKernelGuardFailure {
                phase: "groundwater_linear_reservoir",
                detail: "outflow exceeded storage".to_owned(),
            },
            "direct runtime kernel guard failed in groundwater_linear_reservoir: outflow exceeded storage",
        );
        assert_display(
            &DirectRuntimeError::PublicationDayInputBuildFailure {
                detail: "missing calendar row".to_owned(),
            },
            "direct publication day-input builder failed: missing calendar row",
        );
        assert_display(
            &DirectRuntimeError::PublicationSinkFailure {
                detail: "writer rejected row".to_owned(),
            },
            "direct publication sink failed: writer rejected row",
        );
        assert_display(
            &DirectRuntimeError::DirectClosureToleranceExceeded {
                field: "water_ledger.residual_m",
            },
            "direct runtime field water_ledger.residual_m exceeds declared closure tolerance",
        );
        assert_display(
            &DirectRuntimeError::DirectDayExecutionFailure {
                lane_index: 2,
                day_index: 3,
                detail: "upstream guard failed".to_owned(),
            },
            "direct runtime day execution failed at lane 3 day 4: upstream guard failed",
        );
    }

    #[test]
    fn direct_runtime_audit_counter_records_specialized_events_locally() {
        let audit = DirectRuntimeAuditCounters::new();
        audit.record_wave1_flux_refused_quantum();
        audit.record_compatibility_edge_invocation();
        audit.record_ksatadj_effective_conductivity_evaluation();

        let snapshot = audit.snapshot();
        assert_eq!(snapshot.wave1_flux_refused_quanta, 1);
        assert_eq!(snapshot.compatibility_edge_invocations, 1);
        assert_eq!(snapshot.ksatadj_effective_conductivity_evaluations, 1);

        audit.reset();
        let reset = audit.snapshot();
        assert_eq!(reset.wave1_flux_refused_quanta, 0);
        assert_eq!(reset.compatibility_edge_invocations, 0);
        assert_eq!(reset.ksatadj_effective_conductivity_evaluations, 0);
    }
}
