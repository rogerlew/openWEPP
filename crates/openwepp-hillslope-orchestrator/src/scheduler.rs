#[allow(clippy::wildcard_imports)]
use super::*;
use crate::constants::PHASE_COUNT;

/// Explicit scheduler dependency edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhaseDependency {
    pub phase: HillslopePhase,
    pub depends_on: HillslopePhase,
}

/// Deterministic dependency graph for hillslope phase ordering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HillslopePhaseGraph {
    dependencies: BTreeMap<HillslopePhase, Vec<HillslopePhase>>,
}

impl HillslopePhaseGraph {
    /// Build the canonical ARCH05 deterministic graph.
    #[must_use]
    pub fn canonical() -> Self {
        let mut dependencies: BTreeMap<HillslopePhase, Vec<HillslopePhase>> = BTreeMap::new();
        for phase in HillslopePhase::ORDERED {
            dependencies.insert(phase, Vec::new());
        }

        for edge in Self::canonical_dependencies() {
            dependencies
                .entry(edge.phase)
                .or_default()
                .push(edge.depends_on);
        }

        for deps in dependencies.values_mut() {
            deps.sort_by_key(|phase| phase.rank());
            deps.dedup();
        }

        Self { dependencies }
    }

    #[must_use]
    pub fn dependencies_for(&self, phase: HillslopePhase) -> &[HillslopePhase] {
        self.dependencies
            .get(&phase)
            .map_or(&[] as &[HillslopePhase], Vec::as_slice)
    }

    #[must_use]
    pub const fn canonical_order() -> [HillslopePhase; PHASE_COUNT] {
        HillslopePhase::ORDERED
    }

    #[must_use]
    pub fn dependency_edges(&self) -> Vec<PhaseDependency> {
        let mut edges = Vec::new();

        for phase in HillslopePhase::ORDERED {
            if let Some(deps) = self.dependencies.get(&phase) {
                for dependency in deps {
                    edges.push(PhaseDependency {
                        phase,
                        depends_on: *dependency,
                    });
                }
            }
        }

        edges
    }

    #[cfg(test)]
    #[must_use]
    pub(crate) fn from_dependencies_for_test(
        dependencies: BTreeMap<HillslopePhase, Vec<HillslopePhase>>,
    ) -> Self {
        Self { dependencies }
    }

    #[must_use]
    pub fn topological_order(&self) -> Option<Vec<HillslopePhase>> {
        let mut indegree: BTreeMap<HillslopePhase, usize> = BTreeMap::new();
        let mut adjacency: BTreeMap<HillslopePhase, BTreeSet<HillslopePhase>> = BTreeMap::new();

        for phase in HillslopePhase::ORDERED {
            indegree.insert(phase, 0);
            adjacency.insert(phase, BTreeSet::new());
        }

        for phase in HillslopePhase::ORDERED {
            for dependency in self.dependencies_for(phase) {
                let value = indegree.get_mut(&phase)?;
                *value += 1;

                adjacency.entry(*dependency).or_default().insert(phase);
            }
        }

        let mut ready: Vec<HillslopePhase> = HillslopePhase::ORDERED
            .iter()
            .copied()
            .filter(|phase| indegree.get(phase).copied().unwrap_or(0) == 0)
            .collect();
        let mut order = Vec::with_capacity(PHASE_COUNT);

        while !ready.is_empty() {
            ready.sort_by_key(|phase| phase.rank());
            let phase = ready.remove(0);
            order.push(phase);

            if let Some(neighbors) = adjacency.get(&phase) {
                for neighbor in neighbors {
                    let count = indegree.get_mut(neighbor)?;

                    if *count == 0 {
                        return None;
                    }

                    *count -= 1;
                    if *count == 0 {
                        ready.push(*neighbor);
                    }
                }
            }
        }

        if order.len() == PHASE_COUNT {
            Some(order)
        } else {
            None
        }
    }

    #[must_use]
    const fn canonical_dependencies() -> [PhaseDependency; PHASE_COUNT - 1] {
        [
            PhaseDependency {
                phase: HillslopePhase::StorageBounds,
                depends_on: HillslopePhase::Normalization,
            },
            PhaseDependency {
                phase: HillslopePhase::DecompositionTransition,
                depends_on: HillslopePhase::StorageBounds,
            },
            PhaseDependency {
                phase: HillslopePhase::ResiduePartitionTransition,
                depends_on: HillslopePhase::DecompositionTransition,
            },
            PhaseDependency {
                phase: HillslopePhase::AnnualGrowthTransition,
                depends_on: HillslopePhase::ResiduePartitionTransition,
            },
            PhaseDependency {
                phase: HillslopePhase::PerennialGrowthTransition,
                depends_on: HillslopePhase::AnnualGrowthTransition,
            },
            PhaseDependency {
                phase: HillslopePhase::PercolationDeepSeepage,
                depends_on: HillslopePhase::PerennialGrowthTransition,
            },
            PhaseDependency {
                phase: HillslopePhase::Evapotranspiration,
                depends_on: HillslopePhase::PercolationDeepSeepage,
            },
            PhaseDependency {
                phase: HillslopePhase::Drainage,
                depends_on: HillslopePhase::Evapotranspiration,
            },
            PhaseDependency {
                phase: HillslopePhase::LateralTransfer,
                depends_on: HillslopePhase::Drainage,
            },
            PhaseDependency {
                phase: HillslopePhase::PlantRootUptake,
                depends_on: HillslopePhase::LateralTransfer,
            },
            PhaseDependency {
                phase: HillslopePhase::RunoffReconciliation,
                depends_on: HillslopePhase::PlantRootUptake,
            },
            PhaseDependency {
                phase: HillslopePhase::StorageReconciliation,
                depends_on: HillslopePhase::RunoffReconciliation,
            },
            PhaseDependency {
                phase: HillslopePhase::ClosureDiagnostics,
                depends_on: HillslopePhase::StorageReconciliation,
            },
        ]
    }
}

impl Default for HillslopePhaseGraph {
    fn default() -> Self {
        Self::canonical()
    }
}

/// One executed phase and its typed status surface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HillslopePhaseOutcome {
    pub phase: HillslopePhase,
    pub status: SimulationStatus,
}

/// Coarse scheduler completion class for deterministic decision routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulerOutcomeClass {
    Completed,
    TopologyPreconditionFailed,
    PhaseFailure,
    SchedulerInvariantFailure,
}

/// Scheduler execution report.
#[derive(Debug, Clone)]
pub struct HillslopeSchedulerReport {
    pub outcome_class: SchedulerOutcomeClass,
    pub topology_precondition_status: SimulationStatus,
    pub scheduler_status: SimulationStatus,
    pub ordered_phases: Vec<HillslopePhase>,
    pub outcomes: Vec<HillslopePhaseOutcome>,
    pub precondition_violations: Vec<ClosureViolation>,
    pub halted_phase: Option<HillslopePhase>,
}

impl HillslopeSchedulerReport {
    #[must_use]
    pub fn is_success(&self) -> bool {
        self.outcome_class == SchedulerOutcomeClass::Completed
            && self.scheduler_status.classification() != StatusClassification::Failure
    }

    #[must_use]
    pub fn executed_phases(&self) -> Vec<HillslopePhase> {
        self.outcomes.iter().map(|outcome| outcome.phase).collect()
    }
}

/// Mutable state/flux maps owned by the hillslope orchestrator.
#[derive(Debug, Clone, Default)]
pub struct HillslopeWritebackSurface {
    pub state_surface: BTreeMap<BoundarySymbol, BoundaryValue>,
    pub flux_surface: BTreeMap<BoundarySymbol, BoundaryValue>,
}

/// Hourly slots carried between adjacent OFE lanes.
pub const MOFE_TRANSFER_HOUR_COUNT: usize = 24;

/// Same-day upstream water-transfer input for one OFE lane.
#[derive(Debug, Clone, PartialEq)]
pub struct TransferInput {
    pub source_ofe_id: Option<usize>,
    pub recipient_ofe_id: usize,
    pub surface_carry: [f64; MOFE_TRANSFER_HOUR_COUNT],
    pub lateral_carry: [f64; MOFE_TRANSFER_HOUR_COUNT],
    pub upstrmq: f64,
    pub subrin: f64,
}

impl TransferInput {
    #[must_use]
    pub const fn zero_for_first_ofe() -> Self {
        Self {
            source_ofe_id: None,
            recipient_ofe_id: 1,
            surface_carry: [0.0; MOFE_TRANSFER_HOUR_COUNT],
            lateral_carry: [0.0; MOFE_TRANSFER_HOUR_COUNT],
            upstrmq: 0.0,
            subrin: 0.0,
        }
    }
}

/// Same-day downstream water-transfer output from one OFE lane.
#[derive(Debug, Clone, PartialEq)]
pub struct TransferOutput {
    pub source_ofe_id: usize,
    pub recipient_ofe_id: Option<usize>,
    pub surface_carry: [f64; MOFE_TRANSFER_HOUR_COUNT],
    pub lateral_carry: [f64; MOFE_TRANSFER_HOUR_COUNT],
    pub qofe: f64,
    pub lateral_export: f64,
}

impl TransferOutput {
    #[must_use]
    pub const fn zero_for_terminal_ofe(source_ofe_id: usize) -> Self {
        Self {
            source_ofe_id,
            recipient_ofe_id: None,
            surface_carry: [0.0; MOFE_TRANSFER_HOUR_COUNT],
            lateral_carry: [0.0; MOFE_TRANSFER_HOUR_COUNT],
            qofe: 0.0,
            lateral_export: 0.0,
        }
    }

    /// Convert a nonterminal transfer output into the next OFE's input.
    ///
    /// # Errors
    ///
    /// Returns `PerOfeDailyWaterBalanceError` when the output is terminal or
    /// names a non-adjacent recipient.
    pub fn as_downstream_input(&self) -> Result<TransferInput, PerOfeDailyWaterBalanceError> {
        let Some(expected_recipient_ofe_id) = self.source_ofe_id.checked_add(1) else {
            return Err(PerOfeDailyWaterBalanceError::InvalidTransferSourceOfeId {
                source_ofe_id: self.source_ofe_id,
            });
        };
        let Some(recipient_ofe_id) = self.recipient_ofe_id else {
            return Err(
                PerOfeDailyWaterBalanceError::TransferOutputRecipientMismatch {
                    source_ofe_id: self.source_ofe_id,
                    expected_recipient_ofe_id: Some(expected_recipient_ofe_id),
                    observed_recipient_ofe_id: None,
                },
            );
        };
        if recipient_ofe_id != expected_recipient_ofe_id {
            return Err(
                PerOfeDailyWaterBalanceError::TransferOutputRecipientMismatch {
                    source_ofe_id: self.source_ofe_id,
                    expected_recipient_ofe_id: Some(expected_recipient_ofe_id),
                    observed_recipient_ofe_id: Some(recipient_ofe_id),
                },
            );
        }

        Ok(TransferInput {
            source_ofe_id: Some(self.source_ofe_id),
            recipient_ofe_id,
            surface_carry: self.surface_carry,
            lateral_carry: self.lateral_carry,
            upstrmq: self.surface_carry.iter().sum(),
            subrin: self.lateral_carry.iter().sum(),
        })
    }
}

/// One OFE-keyed daily water-balance shadow record.
#[derive(Debug, Clone)]
pub struct PerOfeDailyWaterBalanceRecord {
    pub ofe_id: usize,
    pub year: i32,
    pub julian_day: u16,
    pub post_day_state: HillslopeWritebackSurface,
    pub day_flux_surface: HillslopeWritebackSurface,
    pub upstream_transfer_input: TransferInput,
    pub current_transfer_output: TransferOutput,
}

impl PerOfeDailyWaterBalanceRecord {
    /// Construct an explicit OFE-keyed daily record.
    ///
    /// M-E1 does not populate these records in the runner path yet; this
    /// constructor exists so later increments can build records from real
    /// OFE-keyed state without using the legacy aggregate adapter below.
    pub fn new(
        ofe_id: usize,
        year: i32,
        julian_day: u16,
        post_day_state: HillslopeWritebackSurface,
        day_flux_surface: HillslopeWritebackSurface,
        upstream_transfer_input: TransferInput,
        current_transfer_output: TransferOutput,
    ) -> Result<Self, PerOfeDailyWaterBalanceError> {
        if ofe_id == 0 {
            return Err(PerOfeDailyWaterBalanceError::InvalidRecordOfeId { ofe_id });
        }

        Ok(Self {
            ofe_id,
            year,
            julian_day,
            post_day_state,
            day_flux_surface,
            upstream_transfer_input,
            current_transfer_output,
        })
    }

    /// Build the N=1 legacy aggregate adapter record.
    ///
    /// This deliberately has no OFE id parameter: aggregate WB13/WAT state is
    /// valid only as the single-OFE specialization, never as reconstructed
    /// downstream OFE state.
    pub fn from_legacy_single_ofe_aggregate_surface(
        year: i32,
        julian_day: u16,
        aggregate_surface: HillslopeWritebackSurface,
    ) -> Result<Self, PerOfeDailyWaterBalanceError> {
        Self::new(
            1,
            year,
            julian_day,
            aggregate_surface,
            HillslopeWritebackSurface::default(),
            TransferInput::zero_for_first_ofe(),
            TransferOutput::zero_for_terminal_ofe(1),
        )
    }
}

/// OFE-keyed daily water-balance shadow collection for staged MOFE migration.
#[derive(Debug, Clone)]
pub struct PerOfeDailyWaterBalanceCollection {
    simulation_day_index: usize,
    contributor_ofe_count: usize,
    records: Vec<PerOfeDailyWaterBalanceRecord>,
}

impl PerOfeDailyWaterBalanceCollection {
    /// Construct an empty daily per-OFE collection.
    ///
    /// # Errors
    ///
    /// Returns `PerOfeDailyWaterBalanceError` when the day index or
    /// contributor OFE count is outside the contract domain.
    pub fn new(
        simulation_day_index: usize,
        contributor_ofe_count: usize,
    ) -> Result<Self, PerOfeDailyWaterBalanceError> {
        if simulation_day_index == 0 {
            return Err(PerOfeDailyWaterBalanceError::InvalidSimulationDayIndex {
                simulation_day_index,
            });
        }
        if contributor_ofe_count == 0 {
            return Err(PerOfeDailyWaterBalanceError::InvalidContributorOfeCount {
                contributor_ofe_count,
            });
        }

        Ok(Self {
            simulation_day_index,
            contributor_ofe_count,
            records: Vec::with_capacity(contributor_ofe_count),
        })
    }

    pub fn push_record(
        &mut self,
        record: PerOfeDailyWaterBalanceRecord,
    ) -> Result<(), PerOfeDailyWaterBalanceError> {
        let expected_ofe_id = self.records.len() + 1;
        if self.records.len() >= self.contributor_ofe_count {
            return Err(PerOfeDailyWaterBalanceError::TooManyRecords {
                contributor_ofe_count: self.contributor_ofe_count,
            });
        }
        if record.ofe_id != expected_ofe_id {
            return Err(PerOfeDailyWaterBalanceError::NonSequentialOfeRecord {
                expected_ofe_id,
                observed_ofe_id: record.ofe_id,
            });
        }
        if record.upstream_transfer_input.recipient_ofe_id != record.ofe_id {
            return Err(PerOfeDailyWaterBalanceError::TransferRecipientMismatch {
                ofe_id: record.ofe_id,
                recipient_ofe_id: record.upstream_transfer_input.recipient_ofe_id,
            });
        }
        let expected_upstream_source = record.ofe_id.checked_sub(1).filter(|source| *source > 0);
        if record.upstream_transfer_input.source_ofe_id != expected_upstream_source {
            return Err(PerOfeDailyWaterBalanceError::TransferInputSourceMismatch {
                ofe_id: record.ofe_id,
                expected_source_ofe_id: expected_upstream_source,
                observed_source_ofe_id: record.upstream_transfer_input.source_ofe_id,
            });
        }
        if record.current_transfer_output.source_ofe_id != record.ofe_id {
            return Err(PerOfeDailyWaterBalanceError::TransferOutputSourceMismatch {
                ofe_id: record.ofe_id,
                source_ofe_id: record.current_transfer_output.source_ofe_id,
            });
        }
        let expected_downstream_recipient = if record.ofe_id == self.contributor_ofe_count {
            None
        } else {
            Some(record.ofe_id + 1)
        };
        if record.current_transfer_output.recipient_ofe_id != expected_downstream_recipient {
            return Err(
                PerOfeDailyWaterBalanceError::TransferOutputRecipientMismatch {
                    source_ofe_id: record.ofe_id,
                    expected_recipient_ofe_id: expected_downstream_recipient,
                    observed_recipient_ofe_id: record.current_transfer_output.recipient_ofe_id,
                },
            );
        }

        self.records.push(record);
        Ok(())
    }

    #[must_use]
    pub const fn simulation_day_index(&self) -> usize {
        self.simulation_day_index
    }

    #[must_use]
    pub const fn contributor_ofe_count(&self) -> usize {
        self.contributor_ofe_count
    }

    #[must_use]
    pub fn records(&self) -> &[PerOfeDailyWaterBalanceRecord] {
        &self.records
    }

    #[must_use]
    pub fn record_count(&self) -> usize {
        self.records.len()
    }

    /// Return the legacy scalar surface for the N=1 compatibility adapter.
    ///
    /// # Errors
    ///
    /// Returns `PerOfeDailyWaterBalanceError` for incomplete collections or
    /// for multi-OFE collections, whose aggregate derivation remains later
    /// M-E scope.
    pub fn aggregate_for_legacy_outer_consumers(
        &self,
    ) -> Result<HillslopeWritebackSurface, PerOfeDailyWaterBalanceError> {
        if self.contributor_ofe_count != 1 {
            return Err(
                PerOfeDailyWaterBalanceError::MultiOfeAggregateNotImplemented {
                    contributor_ofe_count: self.contributor_ofe_count,
                },
            );
        }
        let Some(record) = self.records.first() else {
            return Err(PerOfeDailyWaterBalanceError::IncompleteCollection {
                contributor_ofe_count: self.contributor_ofe_count,
                record_count: self.records.len(),
            });
        };

        Ok(record.post_day_state.clone())
    }
}

/// Construction and adapter errors for M-E per-OFE shadow state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PerOfeDailyWaterBalanceError {
    InvalidSimulationDayIndex {
        simulation_day_index: usize,
    },
    InvalidContributorOfeCount {
        contributor_ofe_count: usize,
    },
    InvalidRecordOfeId {
        ofe_id: usize,
    },
    InvalidTransferSourceOfeId {
        source_ofe_id: usize,
    },
    TooManyRecords {
        contributor_ofe_count: usize,
    },
    NonSequentialOfeRecord {
        expected_ofe_id: usize,
        observed_ofe_id: usize,
    },
    TransferRecipientMismatch {
        ofe_id: usize,
        recipient_ofe_id: usize,
    },
    TransferInputSourceMismatch {
        ofe_id: usize,
        expected_source_ofe_id: Option<usize>,
        observed_source_ofe_id: Option<usize>,
    },
    TransferOutputSourceMismatch {
        ofe_id: usize,
        source_ofe_id: usize,
    },
    TransferOutputRecipientMismatch {
        source_ofe_id: usize,
        expected_recipient_ofe_id: Option<usize>,
        observed_recipient_ofe_id: Option<usize>,
    },
    IncompleteCollection {
        contributor_ofe_count: usize,
        record_count: usize,
    },
    MultiOfeAggregateNotImplemented {
        contributor_ofe_count: usize,
    },
}

impl fmt::Display for PerOfeDailyWaterBalanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSimulationDayIndex {
                simulation_day_index,
            } => write!(
                f,
                "simulation_day_index must be >= 1, observed {simulation_day_index}"
            ),
            Self::InvalidContributorOfeCount {
                contributor_ofe_count,
            } => write!(
                f,
                "contributor_ofe_count must be >= 1, observed {contributor_ofe_count}"
            ),
            Self::InvalidRecordOfeId { ofe_id } => {
                write!(f, "per-OFE record id must be >= 1, observed {ofe_id}")
            }
            Self::InvalidTransferSourceOfeId { source_ofe_id } => write!(
                f,
                "transfer output source OFE id cannot be incremented, observed {source_ofe_id}"
            ),
            Self::TooManyRecords {
                contributor_ofe_count,
            } => write!(
                f,
                "cannot append more than {contributor_ofe_count} per-OFE records"
            ),
            Self::NonSequentialOfeRecord {
                expected_ofe_id,
                observed_ofe_id,
            } => write!(
                f,
                "per-OFE records must be appended in OFE order; expected {expected_ofe_id}, observed {observed_ofe_id}"
            ),
            Self::TransferRecipientMismatch {
                ofe_id,
                recipient_ofe_id,
            } => write!(
                f,
                "upstream transfer recipient {recipient_ofe_id} does not match record OFE {ofe_id}"
            ),
            Self::TransferInputSourceMismatch {
                ofe_id,
                expected_source_ofe_id,
                observed_source_ofe_id,
            } => write!(
                f,
                "upstream transfer source {observed_source_ofe_id:?} does not match expected source {expected_source_ofe_id:?} for record OFE {ofe_id}"
            ),
            Self::TransferOutputSourceMismatch {
                ofe_id,
                source_ofe_id,
            } => write!(
                f,
                "transfer output source {source_ofe_id} does not match record OFE {ofe_id}"
            ),
            Self::TransferOutputRecipientMismatch {
                source_ofe_id,
                expected_recipient_ofe_id,
                observed_recipient_ofe_id,
            } => write!(
                f,
                "transfer output from OFE {source_ofe_id} targets {observed_recipient_ofe_id:?}; expected {expected_recipient_ofe_id:?}"
            ),
            Self::IncompleteCollection {
                contributor_ofe_count,
                record_count,
            } => write!(
                f,
                "per-OFE collection has {record_count} records for {contributor_ofe_count} contributing OFEs"
            ),
            Self::MultiOfeAggregateNotImplemented {
                contributor_ofe_count,
            } => write!(
                f,
                "aggregate derivation from {contributor_ofe_count} per-OFE records is later M-E scope"
            ),
        }
    }
}

impl Error for PerOfeDailyWaterBalanceError {}

/// Per-phase kernel/writeback execution evidence.
#[derive(Debug, Clone)]
pub struct HillslopeKernelPhaseReport {
    pub phase: HillslopePhase,
    pub kernel_status: SimulationStatus,
    pub decision_outcome: WritebackDecisionOutcome,
    pub decision_status: SimulationStatus,
    pub decision_violations: Vec<ClosureViolation>,
    pub apply_result: Option<KernelWritebackApplyResult>,
}

/// Kernel-integrated hillslope execution report.
#[derive(Debug, Clone)]
pub struct HillslopeKernelExecutionReport {
    pub scheduler_report: HillslopeSchedulerReport,
    pub phase_reports: Vec<HillslopeKernelPhaseReport>,
    pub writeback_surface: HillslopeWritebackSurface,
}

/// Scheduler construction/operation error.
#[derive(Debug)]
pub enum HillslopeSchedulerError {
    Status(StatusError),
    Writeback(WritebackError),
}

impl fmt::Display for HillslopeSchedulerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Status(source) => write!(f, "status construction failed: {source}"),
            Self::Writeback(source) => write!(f, "writeback application failed: {source}"),
        }
    }
}

impl Error for HillslopeSchedulerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Status(source) => Some(source),
            Self::Writeback(source) => Some(source),
        }
    }
}

impl From<StatusError> for HillslopeSchedulerError {
    fn from(value: StatusError) -> Self {
        Self::Status(value)
    }
}

impl From<WritebackError> for HillslopeSchedulerError {
    fn from(value: WritebackError) -> Self {
        Self::Writeback(value)
    }
}

/// Deterministic hillslope scheduler.
#[derive(Debug, Clone)]
pub struct HillslopePhaseScheduler {
    graph: HillslopePhaseGraph,
}

impl HillslopePhaseScheduler {
    #[must_use]
    pub fn canonical() -> Self {
        Self {
            graph: HillslopePhaseGraph::canonical(),
        }
    }

    #[must_use]
    pub fn new(graph: HillslopePhaseGraph) -> Self {
        Self { graph }
    }

    #[must_use]
    pub fn graph(&self) -> &HillslopePhaseGraph {
        &self.graph
    }

    /// Build a nominal phase status for deterministic test/driver defaults.
    pub fn nominal_phase_status(phase: HillslopePhase) -> Result<SimulationStatus, StatusError> {
        SimulationStatus::ok(SimulationPhase::HillslopeKernel, phase.ok_message_id())
    }

    /// Execute deterministic phase scheduling with topology precondition gating.
    #[allow(clippy::too_many_lines)]
    pub fn execute_with<F>(
        &self,
        topology_report: &TopologyValidationReport,
        mut phase_executor: F,
    ) -> Result<HillslopeSchedulerReport, HillslopeSchedulerError>
    where
        F: FnMut(HillslopePhase) -> SimulationStatus,
    {
        if topology_report.status.classification() == StatusClassification::Failure {
            return Ok(HillslopeSchedulerReport {
                outcome_class: SchedulerOutcomeClass::TopologyPreconditionFailed,
                topology_precondition_status: topology_report.status.clone(),
                scheduler_status: topology_report.status.clone(),
                ordered_phases: Vec::new(),
                outcomes: Vec::new(),
                precondition_violations: topology_report.violations.clone(),
                halted_phase: None,
            });
        }

        if !topology_report.violations.is_empty() {
            let status = SimulationStatus::failure(
                SimulationPhase::PreExecutionValidation,
                true,
                false,
                BoundaryClass::TopologyInvalid,
                "HSCHED-E-TOPOLOGY-PRECONDITION",
            )?;

            return Ok(HillslopeSchedulerReport {
                outcome_class: SchedulerOutcomeClass::TopologyPreconditionFailed,
                topology_precondition_status: topology_report.status.clone(),
                scheduler_status: status,
                ordered_phases: Vec::new(),
                outcomes: Vec::new(),
                precondition_violations: topology_report.violations.clone(),
                halted_phase: None,
            });
        }

        let Some(order) = self.graph.topological_order() else {
            let status = SimulationStatus::failure(
                SimulationPhase::HillslopeKernel,
                true,
                false,
                BoundaryClass::ClosureViolation,
                "HSCHED-E-GRAPH-CYCLE",
            )?;

            return Ok(HillslopeSchedulerReport {
                outcome_class: SchedulerOutcomeClass::SchedulerInvariantFailure,
                topology_precondition_status: topology_report.status.clone(),
                scheduler_status: status,
                ordered_phases: Vec::new(),
                outcomes: Vec::new(),
                precondition_violations: Vec::new(),
                halted_phase: None,
            });
        };

        let mut outcomes = Vec::with_capacity(order.len());
        let mut completed: BTreeSet<HillslopePhase> = BTreeSet::new();

        for phase in order.clone() {
            let has_unsatisfied_dependency = self
                .graph
                .dependencies_for(phase)
                .iter()
                .any(|dependency| !completed.contains(dependency));

            if has_unsatisfied_dependency {
                let status = SimulationStatus::failure(
                    SimulationPhase::HillslopeKernel,
                    true,
                    false,
                    BoundaryClass::ClosureViolation,
                    "HSCHED-E-DEPENDENCY-CLOSURE",
                )?;

                return Ok(HillslopeSchedulerReport {
                    outcome_class: SchedulerOutcomeClass::SchedulerInvariantFailure,
                    topology_precondition_status: topology_report.status.clone(),
                    scheduler_status: status,
                    ordered_phases: order,
                    outcomes,
                    precondition_violations: Vec::new(),
                    halted_phase: Some(phase),
                });
            }

            let phase_status = phase_executor(phase);
            if phase_status.phase() != SimulationPhase::HillslopeKernel {
                let status = SimulationStatus::failure(
                    SimulationPhase::HillslopeKernel,
                    true,
                    false,
                    BoundaryClass::ModeMismatch,
                    "HSCHED-E-PHASE-STATUS-PHASE",
                )?;

                outcomes.push(HillslopePhaseOutcome {
                    phase,
                    status: status.clone(),
                });

                return Ok(HillslopeSchedulerReport {
                    outcome_class: SchedulerOutcomeClass::SchedulerInvariantFailure,
                    topology_precondition_status: topology_report.status.clone(),
                    scheduler_status: status,
                    ordered_phases: order,
                    outcomes,
                    precondition_violations: Vec::new(),
                    halted_phase: Some(phase),
                });
            }

            let is_failure = phase_status.classification() == StatusClassification::Failure;
            outcomes.push(HillslopePhaseOutcome {
                phase,
                status: phase_status.clone(),
            });
            completed.insert(phase);

            if is_failure {
                return Ok(HillslopeSchedulerReport {
                    outcome_class: SchedulerOutcomeClass::PhaseFailure,
                    topology_precondition_status: topology_report.status.clone(),
                    scheduler_status: phase_status,
                    ordered_phases: order,
                    outcomes,
                    precondition_violations: Vec::new(),
                    halted_phase: Some(phase),
                });
            }
        }

        let has_advisory = outcomes
            .iter()
            .any(|outcome| outcome.status.classification() == StatusClassification::Advisory);
        let scheduler_status = if has_advisory {
            SimulationStatus::advisory(
                SimulationPhase::HillslopeKernel,
                BoundaryClass::CapBinding,
                ClampClass::None,
                "HSCHED-W-ADVISORY",
            )?
        } else {
            SimulationStatus::ok(SimulationPhase::HillslopeKernel, "HSCHED-OK-001")?
        };

        Ok(HillslopeSchedulerReport {
            outcome_class: SchedulerOutcomeClass::Completed,
            topology_precondition_status: topology_report.status.clone(),
            scheduler_status,
            ordered_phases: order,
            outcomes,
            precondition_violations: Vec::new(),
            halted_phase: None,
        })
    }

    /// Execute deterministic hillslope scheduling against a typed kernel
    /// boundary with explicit writeback accept/reject/apply handling.
    ///
    /// Kernel outputs are pure proposals; orchestrator-owned writeback surfaces
    /// are the only mutable commit authority.
    #[allow(clippy::too_many_lines)]
    pub fn execute_with_kernel<K>(
        &self,
        topology_report: &TopologyValidationReport,
        kernel: &mut K,
        mut writeback_surface: HillslopeWritebackSurface,
    ) -> Result<HillslopeKernelExecutionReport, HillslopeSchedulerError>
    where
        K: HillslopeKernel,
    {
        let mode_mismatch_status = SimulationStatus::failure(
            SimulationPhase::HillslopeKernel,
            true,
            false,
            BoundaryClass::ModeMismatch,
            "HKERNEL-E-STATUS-PHASE-MISMATCH",
        )?;
        let deferred_error_status = SimulationStatus::failure(
            SimulationPhase::HillslopeKernel,
            true,
            false,
            BoundaryClass::ClosureViolation,
            "HKERNEL-E-WRITEBACK-INTERNAL",
        )?;

        let mut phase_reports = Vec::new();
        let mut deferred_error: Option<HillslopeSchedulerError> = None;

        let scheduler_report = self.execute_with(topology_report, |phase| {
            if deferred_error.is_some() {
                return deferred_error_status.clone();
            }

            let consumer_adapter = hillslope_consumer_adapter_for_phase(phase);
            let phase_class = hillslope_phase_class_for_phase(phase);
            let mut decomposition_context = None;
            let mut growth_context = None;

            if is_decomposition_phase(phase) {
                let decomposition_dispatch = match decomposition_phase_dispatch_for_state(
                    phase,
                    &writeback_surface.state_surface,
                ) {
                    Ok(value) => value,
                    Err(source) => {
                        let boundary_status = match SimulationStatus::failure(
                            SimulationPhase::HillslopeKernel,
                            true,
                            false,
                            source.boundary_class(),
                            source.code(),
                        ) {
                            Ok(status) => status,
                            Err(status_error) => {
                                deferred_error =
                                    Some(HillslopeSchedulerError::Status(status_error));
                                phase_reports.push(HillslopeKernelPhaseReport {
                                    phase,
                                    kernel_status: deferred_error_status.clone(),
                                    decision_outcome: WritebackDecisionOutcome::Reject,
                                    decision_status: deferred_error_status.clone(),
                                    decision_violations: Vec::new(),
                                    apply_result: None,
                                });
                                return deferred_error_status.clone();
                            }
                        };

                        phase_reports.push(HillslopeKernelPhaseReport {
                            phase,
                            kernel_status: boundary_status.clone(),
                            decision_outcome: WritebackDecisionOutcome::Reject,
                            decision_status: boundary_status.clone(),
                            decision_violations: Vec::new(),
                            apply_result: None,
                        });
                        return boundary_status;
                    }
                };

                if let DecompositionPhaseDispatch::Execute(context) = decomposition_dispatch {
                    decomposition_context = Some(context);
                }
            } else if is_growth_phase(phase) {
                let growth_dispatch = match growth_phase_dispatch_for_state(
                    phase,
                    &writeback_surface.state_surface,
                ) {
                    Ok(value) => value,
                    Err(source) => {
                        let boundary_status = match SimulationStatus::failure(
                            SimulationPhase::HillslopeKernel,
                            true,
                            false,
                            source.boundary_class(),
                            source.code(),
                        ) {
                            Ok(status) => status,
                            Err(status_error) => {
                                deferred_error =
                                    Some(HillslopeSchedulerError::Status(status_error));
                                phase_reports.push(HillslopeKernelPhaseReport {
                                    phase,
                                    kernel_status: deferred_error_status.clone(),
                                    decision_outcome: WritebackDecisionOutcome::Reject,
                                    decision_status: deferred_error_status.clone(),
                                    decision_violations: Vec::new(),
                                    apply_result: None,
                                });
                                return deferred_error_status.clone();
                            }
                        };

                        phase_reports.push(HillslopeKernelPhaseReport {
                            phase,
                            kernel_status: boundary_status.clone(),
                            decision_outcome: WritebackDecisionOutcome::Reject,
                            decision_status: boundary_status.clone(),
                            decision_violations: Vec::new(),
                            apply_result: None,
                        });
                        return boundary_status;
                    }
                };

                if let GrowthPhaseDispatch::Execute(context) = growth_dispatch {
                    growth_context = Some(context);
                }
            } else {
                if let Err(source) = hydrology_phase_dispatch_for_phase(phase, phase_class) {
                    let boundary_status = match SimulationStatus::failure(
                        SimulationPhase::HillslopeKernel,
                        true,
                        false,
                        source.boundary_class(),
                        source.code(),
                    ) {
                        Ok(status) => status,
                        Err(status_error) => {
                            deferred_error = Some(HillslopeSchedulerError::Status(status_error));
                            phase_reports.push(HillslopeKernelPhaseReport {
                                phase,
                                kernel_status: deferred_error_status.clone(),
                                decision_outcome: WritebackDecisionOutcome::Reject,
                                decision_status: deferred_error_status.clone(),
                                decision_violations: Vec::new(),
                                apply_result: None,
                            });
                            return deferred_error_status.clone();
                        }
                    };

                    phase_reports.push(HillslopeKernelPhaseReport {
                        phase,
                        kernel_status: boundary_status.clone(),
                        decision_outcome: WritebackDecisionOutcome::Reject,
                        decision_status: boundary_status.clone(),
                        decision_violations: Vec::new(),
                        apply_result: None,
                    });
                    return boundary_status;
                }

                if let Err(source) =
                    validate_hillslope_consumer_boundary(phase, &writeback_surface.state_surface)
                {
                    let boundary_status = match SimulationStatus::failure(
                        SimulationPhase::HillslopeKernel,
                        true,
                        false,
                        BoundaryClass::MissingRequiredInput,
                        source.code(),
                    ) {
                        Ok(status) => status,
                        Err(status_error) => {
                            deferred_error = Some(HillslopeSchedulerError::Status(status_error));
                            phase_reports.push(HillslopeKernelPhaseReport {
                                phase,
                                kernel_status: deferred_error_status.clone(),
                                decision_outcome: WritebackDecisionOutcome::Reject,
                                decision_status: deferred_error_status.clone(),
                                decision_violations: Vec::new(),
                                apply_result: None,
                            });
                            return deferred_error_status.clone();
                        }
                    };

                    phase_reports.push(HillslopeKernelPhaseReport {
                        phase,
                        kernel_status: boundary_status.clone(),
                        decision_outcome: WritebackDecisionOutcome::Reject,
                        decision_status: boundary_status.clone(),
                        decision_violations: Vec::new(),
                        apply_result: None,
                    });
                    return boundary_status;
                }
            }

            let response = {
                let request = HillslopeKernelRequest::with_transition_context(
                    phase.as_str(),
                    phase_class,
                    consumer_adapter,
                    decomposition_context,
                    growth_context,
                    &writeback_surface.state_surface,
                    &writeback_surface.flux_surface,
                );
                kernel.run_hillslope_phase(&request)
            };
            let kernel_status = response.status.clone();

            if kernel_status.phase() != SimulationPhase::HillslopeKernel {
                phase_reports.push(HillslopeKernelPhaseReport {
                    phase,
                    kernel_status,
                    decision_outcome: WritebackDecisionOutcome::Reject,
                    decision_status: mode_mismatch_status.clone(),
                    decision_violations: Vec::new(),
                    apply_result: None,
                });
                return mode_mismatch_status.clone();
            }

            if kernel_status.classification() == StatusClassification::Failure {
                phase_reports.push(HillslopeKernelPhaseReport {
                    phase,
                    kernel_status: kernel_status.clone(),
                    decision_outcome: WritebackDecisionOutcome::Reject,
                    decision_status: kernel_status.clone(),
                    decision_violations: Vec::new(),
                    apply_result: None,
                });
                return kernel_status;
            }

            let decision = match evaluate_kernel_writeback(
                SimulationPhase::HillslopeKernel,
                &response.writeback,
            ) {
                Ok(value) => value,
                Err(source) => {
                    deferred_error = Some(HillslopeSchedulerError::Status(source));
                    phase_reports.push(HillslopeKernelPhaseReport {
                        phase,
                        kernel_status,
                        decision_outcome: WritebackDecisionOutcome::Reject,
                        decision_status: deferred_error_status.clone(),
                        decision_violations: Vec::new(),
                        apply_result: None,
                    });
                    return deferred_error_status.clone();
                }
            };

            if decision.outcome == WritebackDecisionOutcome::Reject {
                phase_reports.push(HillslopeKernelPhaseReport {
                    phase,
                    kernel_status,
                    decision_outcome: WritebackDecisionOutcome::Reject,
                    decision_status: decision.status.clone(),
                    decision_violations: decision.violations.clone(),
                    apply_result: None,
                });
                return decision.status;
            }

            let apply_result = match apply_kernel_writeback(
                SimulationPhase::HillslopeKernel,
                &decision,
                &response.writeback,
                &mut writeback_surface.state_surface,
                &mut writeback_surface.flux_surface,
            ) {
                Ok(value) => value,
                Err(source) => {
                    deferred_error = Some(HillslopeSchedulerError::Writeback(source));
                    phase_reports.push(HillslopeKernelPhaseReport {
                        phase,
                        kernel_status,
                        decision_outcome: WritebackDecisionOutcome::Reject,
                        decision_status: deferred_error_status.clone(),
                        decision_violations: Vec::new(),
                        apply_result: None,
                    });
                    return deferred_error_status.clone();
                }
            };

            phase_reports.push(HillslopeKernelPhaseReport {
                phase,
                kernel_status: kernel_status.clone(),
                decision_outcome: apply_result.outcome,
                decision_status: apply_result.status.clone(),
                decision_violations: Vec::new(),
                apply_result: Some(apply_result),
            });

            kernel_status
        })?;

        if let Some(error) = deferred_error {
            return Err(error);
        }

        Ok(HillslopeKernelExecutionReport {
            scheduler_report,
            phase_reports,
            writeback_surface,
        })
    }
}

impl Default for HillslopePhaseScheduler {
    fn default() -> Self {
        Self::canonical()
    }
}
