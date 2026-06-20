use std::error::Error;
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

pub const DIRECT_TRANSFER_HOUR_COUNT: usize = 24;
pub const DIRECT_PHASE_COUNT: usize = 14;

static DIRECT_AUDIT: DirectRuntimeAuditCounters = DirectRuntimeAuditCounters::new();

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum DirectExecutorMode {
    #[default]
    Noop,
    ShadowOnly,
}

impl DirectExecutorMode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Noop => "noop",
            Self::ShadowOnly => "shadow-only",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectPhaseKind {
    Normalization,
    StorageBounds,
    DecompositionTransition,
    ResiduePartitionTransition,
    AnnualGrowthTransition,
    PerennialGrowthTransition,
    PercolationDeepSeepage,
    Evapotranspiration,
    Drainage,
    LateralTransfer,
    PlantRootUptake,
    RunoffReconciliation,
    StorageReconciliation,
    ClosureDiagnostics,
}

impl DirectPhaseKind {
    pub const ORDERED: [Self; DIRECT_PHASE_COUNT] = [
        Self::Normalization,
        Self::StorageBounds,
        Self::DecompositionTransition,
        Self::ResiduePartitionTransition,
        Self::AnnualGrowthTransition,
        Self::PerennialGrowthTransition,
        Self::PercolationDeepSeepage,
        Self::Evapotranspiration,
        Self::Drainage,
        Self::LateralTransfer,
        Self::PlantRootUptake,
        Self::RunoffReconciliation,
        Self::StorageReconciliation,
        Self::ClosureDiagnostics,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectRunIdentity {
    pub run_id: u64,
    pub hillslope_id: u32,
    pub lane_count: usize,
    pub day_count: usize,
}

impl DirectRunIdentity {
    pub fn new(
        run_id: u64,
        hillslope_id: u32,
        lane_count: usize,
        day_count: usize,
    ) -> Result<Self, DirectRuntimeError> {
        if lane_count == 0 {
            return Err(DirectRuntimeError::InvalidLaneCount { lane_count });
        }
        if day_count == 0 {
            return Err(DirectRuntimeError::InvalidDayCount { day_count });
        }

        Ok(Self {
            run_id,
            hillslope_id,
            lane_count,
            day_count,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectRunFrame {
    pub identity: DirectRunIdentity,
    pub lanes: Vec<DirectLaneFrame>,
    pub phase_plan: DirectPhasePlan,
    pub publication: DirectPublicationFrame,
}

impl DirectRunFrame {
    pub fn skeleton(identity: DirectRunIdentity) -> Result<Self, DirectRuntimeError> {
        DIRECT_AUDIT.record_run_frame_construction();
        let lanes = (0..identity.lane_count)
            .map(|lane_index| DirectLaneFrame::skeleton(lane_index, identity.lane_count))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            identity,
            lanes,
            phase_plan: DirectPhasePlan::default(),
            publication: DirectPublicationFrame::empty(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectLaneFrame {
    pub lane_id: u32,
    pub upstream_lane_id: u32,
    pub downstream_lane_id: u32,
    pub upstream_area_ratio: f64,
    pub area_m2: f64,
    pub water: DirectWaterState,
    pub transfer: DirectTransferBuffers,
}

impl DirectLaneFrame {
    fn skeleton(lane_index: usize, lane_count: usize) -> Result<Self, DirectRuntimeError> {
        let lane_id = u32::try_from(lane_index + 1)
            .map_err(|_| DirectRuntimeError::LaneIdOverflow { lane_index })?;
        let upstream_lane_id = lane_id.saturating_sub(1);
        let downstream_lane_id = if lane_index + 1 == lane_count {
            0
        } else {
            lane_id + 1
        };

        Ok(Self {
            lane_id,
            upstream_lane_id,
            downstream_lane_id,
            upstream_area_ratio: 1.0,
            area_m2: 0.0,
            water: DirectWaterState::zero(),
            transfer: DirectTransferBuffers::zero(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectDayFrame {
    pub identity: DirectRunIdentity,
    pub lane_index: usize,
    pub day_index: usize,
    pub forcing: DirectDayForcing,
    pub water: DirectWaterState,
    pub transfer: DirectTransferBuffers,
    pub publication: DirectPublicationFrame,
}

impl DirectDayFrame {
    pub fn seed(
        identity: DirectRunIdentity,
        lane_index: usize,
        day_index: usize,
    ) -> Result<Self, DirectRuntimeError> {
        if lane_index >= identity.lane_count {
            return Err(DirectRuntimeError::LaneIndexOutOfRange {
                lane_index,
                lane_count: identity.lane_count,
            });
        }
        if day_index >= identity.day_count {
            return Err(DirectRuntimeError::DayIndexOutOfRange {
                day_index,
                day_count: identity.day_count,
            });
        }

        DIRECT_AUDIT.record_day_frame_construction();

        Ok(Self {
            identity,
            lane_index,
            day_index,
            forcing: DirectDayForcing::zero(),
            water: DirectWaterState::zero(),
            transfer: DirectTransferBuffers::zero(),
            publication: DirectPublicationFrame::empty(),
        })
    }

    pub fn phase_view(&mut self, phase: DirectPhaseKind) -> DirectPhaseView<'_> {
        DIRECT_AUDIT.record_phase_view_construction();
        DirectPhaseView {
            phase,
            water: &mut self.water,
            transfer: &mut self.transfer,
            publication: &mut self.publication,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct DirectPhaseView<'day> {
    phase: DirectPhaseKind,
    water: &'day mut DirectWaterState,
    transfer: &'day mut DirectTransferBuffers,
    publication: &'day mut DirectPublicationFrame,
}

impl DirectPhaseView<'_> {
    #[must_use]
    pub const fn phase(&self) -> DirectPhaseKind {
        self.phase
    }

    #[must_use]
    pub fn water_state(&self) -> &DirectWaterState {
        self.water
    }

    #[must_use]
    pub fn transfer_buffers(&self) -> &DirectTransferBuffers {
        self.transfer
    }

    #[must_use]
    pub fn publication_frame(&self) -> &DirectPublicationFrame {
        self.publication
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectPublicationFrame {
    pub runoff_m: f64,
    pub infiltration_m: f64,
    pub evapotranspiration_m: f64,
    pub drainage_m: f64,
    pub lateral_flow_m: f64,
}

impl DirectPublicationFrame {
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            runoff_m: 0.0,
            infiltration_m: 0.0,
            evapotranspiration_m: 0.0,
            drainage_m: 0.0,
            lateral_flow_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectPhasePlan {
    phases: [DirectPhaseKind; DIRECT_PHASE_COUNT],
}

impl DirectPhasePlan {
    #[must_use]
    pub const fn phases(&self) -> &[DirectPhaseKind; DIRECT_PHASE_COUNT] {
        &self.phases
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        DIRECT_PHASE_COUNT
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        false
    }
}

impl Default for DirectPhasePlan {
    fn default() -> Self {
        Self {
            phases: DirectPhaseKind::ORDERED,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectWaterState {
    pub soil_water_m: f64,
    pub infiltration_m: f64,
    pub runoff_m: f64,
    pub evapotranspiration_m: f64,
    pub drainage_m: f64,
    pub lateral_flow_m: f64,
}

impl DirectWaterState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            soil_water_m: 0.0,
            infiltration_m: 0.0,
            runoff_m: 0.0,
            evapotranspiration_m: 0.0,
            drainage_m: 0.0,
            lateral_flow_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectDayForcing {
    pub precipitation_m: f64,
    pub effective_temperature_c: f64,
}

impl DirectDayForcing {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            precipitation_m: 0.0,
            effective_temperature_c: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectTransferBuffers {
    pub surface_carry_m: [f64; DIRECT_TRANSFER_HOUR_COUNT],
    pub lateral_carry_m: [f64; DIRECT_TRANSFER_HOUR_COUNT],
    pub upstream_flow_m: f64,
    pub subsurface_input_m: f64,
}

impl DirectTransferBuffers {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            surface_carry_m: [0.0; DIRECT_TRANSFER_HOUR_COUNT],
            lateral_carry_m: [0.0; DIRECT_TRANSFER_HOUR_COUNT],
            upstream_flow_m: 0.0,
            subsurface_input_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectExecutionReport {
    pub mode: DirectExecutorMode,
    pub lane_count: usize,
    pub day_count: usize,
    pub planned_phase_count: usize,
    pub phase_view_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectFrameExecutor {
    mode: DirectExecutorMode,
}

impl DirectFrameExecutor {
    #[must_use]
    pub fn new(mode: DirectExecutorMode) -> Self {
        DIRECT_AUDIT.record_executor_construction();
        Self { mode }
    }

    #[must_use]
    pub const fn mode(&self) -> DirectExecutorMode {
        self.mode
    }

    pub fn run_skeleton(
        &self,
        frame: &mut DirectRunFrame,
    ) -> Result<DirectExecutionReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_skeleton_run();
        let mut phase_view_count = 0_u64;
        for lane_index in 0..frame.lanes.len() {
            let mut day_frame = DirectDayFrame::seed(frame.identity, lane_index, 0)?;
            for phase in frame.phase_plan.phases() {
                let view = day_frame.phase_view(*phase);
                let _phase = view.phase();
                phase_view_count += 1;
            }
        }

        Ok(DirectExecutionReport {
            mode: self.mode,
            lane_count: frame.lanes.len(),
            day_count: frame.identity.day_count,
            planned_phase_count: frame.phase_plan.len(),
            phase_view_count,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectRuntimeAuditSnapshot {
    pub run_frame_constructions: u64,
    pub day_frame_constructions: u64,
    pub executor_constructions: u64,
    pub skeleton_runs: u64,
    pub phase_view_constructions: u64,
}

#[must_use]
pub fn direct_runtime_audit_snapshot() -> DirectRuntimeAuditSnapshot {
    DIRECT_AUDIT.snapshot()
}

pub fn reset_direct_runtime_audit_counters() {
    DIRECT_AUDIT.reset();
}

struct DirectRuntimeAuditCounters {
    run_frame_constructions: AtomicU64,
    day_frame_constructions: AtomicU64,
    executor_constructions: AtomicU64,
    skeleton_runs: AtomicU64,
    phase_view_constructions: AtomicU64,
}

impl DirectRuntimeAuditCounters {
    const fn new() -> Self {
        Self {
            run_frame_constructions: AtomicU64::new(0),
            day_frame_constructions: AtomicU64::new(0),
            executor_constructions: AtomicU64::new(0),
            skeleton_runs: AtomicU64::new(0),
            phase_view_constructions: AtomicU64::new(0),
        }
    }

    fn record_run_frame_construction(&self) {
        self.run_frame_constructions.fetch_add(1, Ordering::Relaxed);
    }

    fn record_day_frame_construction(&self) {
        self.day_frame_constructions.fetch_add(1, Ordering::Relaxed);
    }

    fn record_executor_construction(&self) {
        self.executor_constructions.fetch_add(1, Ordering::Relaxed);
    }

    fn record_skeleton_run(&self) {
        self.skeleton_runs.fetch_add(1, Ordering::Relaxed);
    }

    fn record_phase_view_construction(&self) {
        self.phase_view_constructions
            .fetch_add(1, Ordering::Relaxed);
    }

    fn snapshot(&self) -> DirectRuntimeAuditSnapshot {
        DirectRuntimeAuditSnapshot {
            run_frame_constructions: self.run_frame_constructions.load(Ordering::Relaxed),
            day_frame_constructions: self.day_frame_constructions.load(Ordering::Relaxed),
            executor_constructions: self.executor_constructions.load(Ordering::Relaxed),
            skeleton_runs: self.skeleton_runs.load(Ordering::Relaxed),
            phase_view_constructions: self.phase_view_constructions.load(Ordering::Relaxed),
        }
    }

    fn reset(&self) {
        self.run_frame_constructions.store(0, Ordering::Relaxed);
        self.day_frame_constructions.store(0, Ordering::Relaxed);
        self.executor_constructions.store(0, Ordering::Relaxed);
        self.skeleton_runs.store(0, Ordering::Relaxed);
        self.phase_view_constructions.store(0, Ordering::Relaxed);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    LaneIndexOutOfRange {
        lane_index: usize,
        lane_count: usize,
    },
    DayIndexOutOfRange {
        day_index: usize,
        day_count: usize,
    },
}

impl fmt::Display for DirectRuntimeError {
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

impl Error for DirectRuntimeError {}
