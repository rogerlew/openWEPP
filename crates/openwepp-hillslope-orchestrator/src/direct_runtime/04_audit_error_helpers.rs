#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectRuntimeAuditSnapshot {
    pub run_frame_constructions: u64,
    pub day_frame_constructions: u64,
    pub day_frame_commits: u64,
    pub executor_constructions: u64,
    pub skeleton_runs: u64,
    pub publication_capture_runs: u64,
    pub phase_view_constructions: u64,
    pub phase_span_runs: u64,
    pub direct_phase_entries: u64,
    pub direct_compute_operations: u64,
    pub direct_state_mutations: u64,
    pub downstream_operand_productions: u64,
    pub shadow_projections: u64,
    pub compatibility_edge_invocations: u64,
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

struct DirectRuntimeAuditCounters {
    run_frame_constructions: AtomicU64,
    day_frame_constructions: AtomicU64,
    day_frame_commits: AtomicU64,
    executor_constructions: AtomicU64,
    skeleton_runs: AtomicU64,
    publication_capture_runs: AtomicU64,
    phase_view_constructions: AtomicU64,
    phase_span_runs: AtomicU64,
    direct_phase_entries: AtomicU64,
    direct_compute_operations: AtomicU64,
    direct_state_mutations: AtomicU64,
    downstream_operand_productions: AtomicU64,
    shadow_projections: AtomicU64,
    compatibility_edge_invocations: AtomicU64,
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
            phase_span_runs: AtomicU64::new(0),
            direct_phase_entries: AtomicU64::new(0),
            direct_compute_operations: AtomicU64::new(0),
            direct_state_mutations: AtomicU64::new(0),
            downstream_operand_productions: AtomicU64::new(0),
            shadow_projections: AtomicU64::new(0),
            compatibility_edge_invocations: AtomicU64::new(0),
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

    fn snapshot(&self) -> DirectRuntimeAuditSnapshot {
        DirectRuntimeAuditSnapshot {
            run_frame_constructions: self.run_frame_constructions.load(Ordering::Relaxed),
            day_frame_constructions: self.day_frame_constructions.load(Ordering::Relaxed),
            day_frame_commits: self.day_frame_commits.load(Ordering::Relaxed),
            executor_constructions: self.executor_constructions.load(Ordering::Relaxed),
            skeleton_runs: self.skeleton_runs.load(Ordering::Relaxed),
            publication_capture_runs: self.publication_capture_runs.load(Ordering::Relaxed),
            phase_view_constructions: self.phase_view_constructions.load(Ordering::Relaxed),
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
        self.phase_span_runs.store(0, Ordering::Relaxed);
        self.direct_phase_entries.store(0, Ordering::Relaxed);
        self.direct_compute_operations.store(0, Ordering::Relaxed);
        self.direct_state_mutations.store(0, Ordering::Relaxed);
        self.downstream_operand_productions
            .store(0, Ordering::Relaxed);
        self.shadow_projections.store(0, Ordering::Relaxed);
        self.compatibility_edge_invocations
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
    PublicationDayInputBuildFailure {
        detail: String,
    },
    DirectClosureToleranceExceeded {
        field: &'static str,
    },
    DirectDayExecutionFailure {
        lane_index: usize,
        day_index: usize,
        detail: String,
    },
}

impl fmt::Display for DirectRuntimeError {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLaneCount { lane_count } => {
                write!(
                    formatter,
                    "direct runtime requires at least one lane, observed {lane_count}"
                )
            }
            Self::InvalidDayCount { day_count } => {
                write!(
                    formatter,
                    "direct runtime requires at least one day, observed {day_count}"
                )
            }
            Self::LaneIdOverflow { lane_index } => {
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
            Self::InvalidLaneOutletCount { outlet_count } => {
                write!(
                    formatter,
                    "direct runtime requires exactly one lane outlet, observed {outlet_count}"
                )
            }
            Self::CalendarDayCountMismatch {
                identity_day_count,
                calendar_day_count,
            } => {
                write!(
                    formatter,
                    "direct publication calendar day count {calendar_day_count} does not match identity day count {identity_day_count}"
                )
            }
            Self::PublicationRowCountMismatch {
                expected_row_count,
                actual_row_count,
            } => {
                write!(
                    formatter,
                    "direct publication row count {actual_row_count} does not match expected row count {expected_row_count}"
                )
            }
            Self::InvalidPublicationArea { lane_id, area_m2 } => {
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
            Self::MissingDirectUpstream { upstream } => {
                write!(
                    formatter,
                    "direct runtime upstream span {upstream} must execute before this span"
                )
            }
            Self::NonFiniteDirectValue { field } => {
                write!(formatter, "direct runtime field {field} must be finite")
            }
            Self::NegativeDirectValue { field } => {
                write!(
                    formatter,
                    "direct runtime field {field} must be nonnegative"
                )
            }
            Self::DirectDomainViolation { field } => {
                write!(
                    formatter,
                    "direct runtime field {field} violates its direct-domain constraints"
                )
            }
            Self::DirectKernelGuardFailure { phase, detail } => {
                write!(
                    formatter,
                    "direct runtime kernel guard failed in {phase}: {detail}"
                )
            }
            Self::PublicationDayInputBuildFailure { detail } => {
                write!(
                    formatter,
                    "direct publication day-input builder failed: {detail}"
                )
            }
            Self::DirectClosureToleranceExceeded { field } => {
                write!(
                    formatter,
                    "direct runtime field {field} exceeds declared closure tolerance"
                )
            }
            Self::DirectDayExecutionFailure {
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

impl Error for DirectRuntimeError {}

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
