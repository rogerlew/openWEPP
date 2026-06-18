#[allow(clippy::wildcard_imports)]
use super::*;
use crate::constants::{
    MOFE_HOURLY_CARRY_ARRAY_COUNT, MOFE_HOURLY_CARRY_ARRAYS_ENABLED_SYMBOL,
    MOFE_HOURLY_CURRENT_LATERAL_RUNOFF_ROOT, MOFE_HOURLY_CURRENT_SATURATION_RUNOFF_ROOT,
    MOFE_HOURLY_UPSTREAM_AREA_RATIO_SYMBOL, MOFE_HOURLY_UPSTREAM_LATERAL_RUNOFF_ROOT,
    MOFE_HOURLY_UPSTREAM_SATURATION_RUNOFF_ROOT, PHASE_COUNT, PL_RUNTIME_DAY_SYMBOL,
    PL_RUNTIME_YEAR_SYMBOL, PL_SCHEDULE_ROTATION_REPEATS_SYMBOL, PL_SCHEDULE_ROTATION_YEARS_SYMBOL,
    PL_SCHEDULE_SLOT_COUNT_SYMBOL, WB11_ZERO_THRESHOLD, WB12_SYMBOL_RUNOFF_CARRYOVER,
    WB12_SYMBOL_RUNON_INPUT, WB19_SYMBOL_DRAIN_DEPTH, WB19_SYMBOL_DRAIN_DIAMETER,
    WB19_SYMBOL_DRAIN_ENABLED, WB19_SYMBOL_DRAIN_SPACING, WB19_SYMBOL_LATERAL_DRAIN_LANE_SUBSTEPS,
    WB19_SYMBOL_LATERAL_SSH_ROOT, WB19_SYMBOL_LATERAL_WITHDRAWAL_ROOT,
    WB20_SYMBOL_FORWARD_SOLVER_LANE_ENABLED,
};

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
pub const MOFE_TRANSFER_HOUR_COUNT: usize = MOFE_HOURLY_CARRY_ARRAY_COUNT;

const MOFE_TRANSFER_INPUT_UPSTRMQ_SYMBOL: &str = "UpStrmQ";
const MOFE_TRANSFER_INPUT_SUBRIN_SYMBOL: &str = "SubRIn";

/// Same-day upstream water-transfer input for one OFE lane.
#[derive(Debug, Clone, PartialEq)]
pub struct TransferInput {
    pub source_ofe_id: Option<usize>,
    pub recipient_ofe_id: usize,
    pub area_ratio: f64,
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
            area_ratio: 1.0,
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
        self.as_downstream_input_with_area_ratio(1.0)
    }

    /// Convert a nonterminal transfer output into the next OFE's input with
    /// explicit upstream-area scaling.
    ///
    /// # Errors
    ///
    /// Returns `PerOfeDailyWaterBalanceError` when the output is terminal or
    /// names a non-adjacent recipient.
    pub fn as_downstream_input_with_area_ratio(
        &self,
        area_ratio: f64,
    ) -> Result<TransferInput, PerOfeDailyWaterBalanceError> {
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
            area_ratio,
            surface_carry: self.surface_carry,
            lateral_carry: self.lateral_carry,
            upstrmq: self.surface_carry.iter().sum::<f64>() * area_ratio,
            subrin: self.lateral_carry.iter().sum::<f64>() * area_ratio,
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

/// One OFE lane's starting surface for sequential same-day execution.
#[derive(Debug, Clone)]
pub struct OfeLaneExecutionInput {
    pub ofe_id: usize,
    pub upstream_area_ratio: f64,
    pub writeback_surface: HillslopeWritebackSurface,
    pub indexed_writeback_surface: Option<IndexedWritebackSurface>,
}

impl OfeLaneExecutionInput {
    #[must_use]
    pub fn new(ofe_id: usize, writeback_surface: HillslopeWritebackSurface) -> Self {
        Self {
            ofe_id,
            upstream_area_ratio: 1.0,
            writeback_surface,
            indexed_writeback_surface: None,
        }
    }

    #[must_use]
    pub fn with_upstream_area_ratio(
        ofe_id: usize,
        upstream_area_ratio: f64,
        writeback_surface: HillslopeWritebackSurface,
    ) -> Self {
        Self {
            ofe_id,
            upstream_area_ratio,
            writeback_surface,
            indexed_writeback_surface: None,
        }
    }

    #[must_use]
    pub fn with_indexed_writeback_surface(
        mut self,
        indexed_writeback_surface: IndexedWritebackSurface,
    ) -> Self {
        self.indexed_writeback_surface = Some(indexed_writeback_surface);
        self
    }
}

/// One OFE lane's dynamic state carried across scheduler days.
#[derive(Debug, Clone)]
pub struct OfeLanePersistentState {
    pub ofe_id: usize,
    pub upstream_area_ratio: f64,
    pub writeback_surface: HillslopeWritebackSurface,
    indexed_writeback_surface: Option<IndexedWritebackSurface>,
}

impl OfeLanePersistentState {
    #[must_use]
    pub fn new(ofe_id: usize, writeback_surface: HillslopeWritebackSurface) -> Self {
        Self {
            ofe_id,
            upstream_area_ratio: 1.0,
            writeback_surface,
            indexed_writeback_surface: None,
        }
    }

    #[must_use]
    pub fn with_upstream_area_ratio(
        ofe_id: usize,
        upstream_area_ratio: f64,
        writeback_surface: HillslopeWritebackSurface,
    ) -> Self {
        Self {
            ofe_id,
            upstream_area_ratio,
            writeback_surface,
            indexed_writeback_surface: None,
        }
    }

    pub fn activate_indexed_writeback_authority(
        &mut self,
        registry: &SymbolRegistry,
    ) -> Result<(), SymbolRegistryError> {
        self.indexed_writeback_surface = Some(self.indexed_surface_from_current_surface(registry)?);
        Ok(())
    }

    pub fn refresh_indexed_writeback_authority_if_active(
        &mut self,
        registry: &SymbolRegistry,
    ) -> Result<(), SymbolRegistryError> {
        if self.indexed_writeback_surface.is_some() {
            self.indexed_writeback_surface =
                Some(self.indexed_surface_from_current_surface(registry)?);
        }
        Ok(())
    }

    #[must_use]
    pub fn indexed_writeback_surface(&self) -> Option<&IndexedWritebackSurface> {
        self.indexed_writeback_surface.as_ref()
    }

    #[must_use]
    pub fn take_execution_input(&mut self) -> OfeLaneExecutionInput {
        OfeLaneExecutionInput {
            ofe_id: self.ofe_id,
            upstream_area_ratio: self.upstream_area_ratio,
            writeback_surface: std::mem::take(&mut self.writeback_surface),
            indexed_writeback_surface: self.indexed_writeback_surface.take(),
        }
    }

    #[must_use]
    fn to_execution_input(&self) -> OfeLaneExecutionInput {
        OfeLaneExecutionInput {
            ofe_id: self.ofe_id,
            upstream_area_ratio: self.upstream_area_ratio,
            writeback_surface: self.writeback_surface.clone(),
            indexed_writeback_surface: self.indexed_writeback_surface.clone(),
        }
    }

    fn update_from_report(&mut self, report: &OfeLaneExecutionReport) {
        self.writeback_surface = report.kernel_report.writeback_surface.clone();
        self.indexed_writeback_surface = None;
    }

    fn indexed_surface_from_current_surface(
        &self,
        registry: &SymbolRegistry,
    ) -> Result<IndexedWritebackSurface, SymbolRegistryError> {
        IndexedWritebackSurface::from_btreemap_surfaces(
            registry,
            &self.writeback_surface.state_surface,
            &self.writeback_surface.flux_surface,
        )
    }
}

/// OFE-keyed dynamic state sequence for repeated daily MOFE execution.
#[derive(Debug, Clone)]
pub struct OfeLanePersistentStateSequence {
    lane_states: Vec<OfeLanePersistentState>,
}

impl OfeLanePersistentStateSequence {
    pub fn new(lane_states: Vec<OfeLanePersistentState>) -> Result<Self, OfeLaneSequenceError> {
        validate_persistent_lane_states(&lane_states)?;

        Ok(Self { lane_states })
    }

    #[must_use]
    pub fn lane_states(&self) -> &[OfeLanePersistentState] {
        &self.lane_states
    }

    pub fn lane_states_mut(&mut self) -> &mut [OfeLanePersistentState] {
        &mut self.lane_states
    }

    #[must_use]
    pub fn lane_surface(&self, ofe_id: usize) -> Option<&HillslopeWritebackSurface> {
        self.lane_states
            .iter()
            .find(|lane_state| lane_state.ofe_id == ofe_id)
            .map(|lane_state| &lane_state.writeback_surface)
    }

    pub fn activate_indexed_writeback_authority(
        &mut self,
        registry: &SymbolRegistry,
    ) -> Result<(), SymbolRegistryError> {
        for lane_state in &mut self.lane_states {
            lane_state.activate_indexed_writeback_authority(registry)?;
        }
        Ok(())
    }

    pub fn refresh_indexed_writeback_authority(
        &mut self,
        registry: &SymbolRegistry,
    ) -> Result<(), SymbolRegistryError> {
        for lane_state in &mut self.lane_states {
            lane_state.refresh_indexed_writeback_authority_if_active(registry)?;
        }
        Ok(())
    }

    #[must_use]
    fn to_execution_inputs(&self) -> Vec<OfeLaneExecutionInput> {
        self.lane_states
            .iter()
            .map(OfeLanePersistentState::to_execution_input)
            .collect()
    }

    pub fn replace_from_report(
        &mut self,
        report: &OfeLaneSequenceExecutionReport,
    ) -> Result<(), OfeLaneSequenceError> {
        if self.lane_states.len() != report.lane_reports.len() {
            return Err(OfeLaneSequenceError::PersistentStateLaneCountMismatch {
                expected_lane_count: self.lane_states.len(),
                observed_lane_count: report.lane_reports.len(),
            });
        }

        for (state, lane_report) in self.lane_states.iter_mut().zip(report.lane_reports.iter()) {
            if state.ofe_id != lane_report.ofe_id {
                return Err(OfeLaneSequenceError::PersistentStateLaneMismatch {
                    expected_ofe_id: state.ofe_id,
                    observed_ofe_id: lane_report.ofe_id,
                });
            }

            state.update_from_report(lane_report);
        }

        Ok(())
    }
}

/// One OFE lane's scheduler result and explicit adjacent-transfer evidence.
#[derive(Debug, Clone)]
pub struct OfeLaneExecutionReport {
    pub ofe_id: usize,
    pub upstream_transfer_input: TransferInput,
    pub current_transfer_output: TransferOutput,
    pub kernel_report: HillslopeKernelExecutionReport,
}

/// Sequential OFE lane execution report for one simulation day.
#[derive(Debug, Clone)]
pub struct OfeLaneSequenceExecutionReport {
    pub lane_reports: Vec<OfeLaneExecutionReport>,
}

impl OfeLaneSequenceExecutionReport {
    #[must_use]
    pub fn lane_count(&self) -> usize {
        self.lane_reports.len()
    }
}

/// Fail-closed errors for M-E2 sequential OFE lane execution.
#[derive(Debug)]
pub enum OfeLaneSequenceError {
    InvalidLaneCount {
        lane_count: usize,
    },
    NonSequentialLaneOfeId {
        expected_ofe_id: usize,
        observed_ofe_id: usize,
    },
    Transfer(PerOfeDailyWaterBalanceError),
    InvalidTransferValue {
        ofe_id: usize,
        symbol: String,
        hour: Option<usize>,
        value: f64,
    },
    TransferDailySumMismatch {
        ofe_id: usize,
        symbol: &'static str,
        expected: f64,
        observed: f64,
    },
    PersistentStateLaneCountMismatch {
        expected_lane_count: usize,
        observed_lane_count: usize,
    },
    PersistentStateLaneMismatch {
        expected_ofe_id: usize,
        observed_ofe_id: usize,
    },
    LaneScheduler {
        ofe_id: usize,
        source: HillslopeSchedulerError,
    },
    IndexedSymbolRegistry {
        ofe_id: usize,
        source: SymbolRegistryError,
    },
    LaneExecutionFailed {
        ofe_id: usize,
        status: SimulationStatus,
    },
}

impl fmt::Display for OfeLaneSequenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLaneCount { lane_count } => {
                write!(
                    f,
                    "OFE lane sequence requires at least one lane, observed {lane_count}"
                )
            }
            Self::NonSequentialLaneOfeId {
                expected_ofe_id,
                observed_ofe_id,
            } => write!(
                f,
                "OFE lanes must execute in 1-based adjacent order; expected {expected_ofe_id}, observed {observed_ofe_id}"
            ),
            Self::Transfer(source) => write!(f, "OFE transfer validation failed: {source}"),
            Self::InvalidTransferValue {
                ofe_id,
                symbol,
                hour,
                value,
            } => write!(
                f,
                "OFE {ofe_id} transfer symbol {symbol} hour {hour:?} must be finite and non-negative, observed {value}"
            ),
            Self::TransferDailySumMismatch {
                ofe_id,
                symbol,
                expected,
                observed,
            } => write!(
                f,
                "OFE {ofe_id} transfer daily {symbol} mismatch: expected {expected}, observed {observed}"
            ),
            Self::PersistentStateLaneCountMismatch {
                expected_lane_count,
                observed_lane_count,
            } => write!(
                f,
                "persistent OFE lane state had {expected_lane_count} lanes but execution produced {observed_lane_count}"
            ),
            Self::PersistentStateLaneMismatch {
                expected_ofe_id,
                observed_ofe_id,
            } => write!(
                f,
                "persistent OFE lane state expected OFE {expected_ofe_id} but execution produced OFE {observed_ofe_id}"
            ),
            Self::LaneScheduler { ofe_id, source } => {
                write!(f, "OFE {ofe_id} lane scheduler failed: {source}")
            }
            Self::IndexedSymbolRegistry { ofe_id, source } => {
                write!(f, "OFE {ofe_id} indexed symbol update failed: {source}")
            }
            Self::LaneExecutionFailed { ofe_id, status } => write!(
                f,
                "OFE {ofe_id} lane execution did not complete successfully: {}",
                status.message_id()
            ),
        }
    }
}

impl Error for OfeLaneSequenceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Transfer(source) => Some(source),
            Self::LaneScheduler { source, .. } => Some(source),
            Self::IndexedSymbolRegistry { source, .. } => Some(source),
            _ => None,
        }
    }
}

impl From<PerOfeDailyWaterBalanceError> for OfeLaneSequenceError {
    fn from(value: PerOfeDailyWaterBalanceError) -> Self {
        Self::Transfer(value)
    }
}

fn validate_persistent_lane_states(
    lane_states: &[OfeLanePersistentState],
) -> Result<(), OfeLaneSequenceError> {
    if lane_states.is_empty() {
        return Err(OfeLaneSequenceError::InvalidLaneCount { lane_count: 0 });
    }

    for (index, lane_state) in lane_states.iter().enumerate() {
        let expected_ofe_id = index + 1;
        if lane_state.ofe_id != expected_ofe_id {
            return Err(OfeLaneSequenceError::NonSequentialLaneOfeId {
                expected_ofe_id,
                observed_ofe_id: lane_state.ofe_id,
            });
        }

        validate_positive_transfer_scalar(
            lane_state.ofe_id,
            "persistent upstream_area_ratio",
            lane_state.upstream_area_ratio,
        )?;
    }

    Ok(())
}

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
    SymbolRegistry(SymbolRegistryError),
}

impl fmt::Display for HillslopeSchedulerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Status(source) => write!(f, "status construction failed: {source}"),
            Self::Writeback(source) => write!(f, "writeback application failed: {source}"),
            Self::SymbolRegistry(source) => {
                write!(f, "indexed execution symbol registry failed: {source}")
            }
        }
    }
}

impl Error for HillslopeSchedulerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Status(source) => Some(source),
            Self::Writeback(source) => Some(source),
            Self::SymbolRegistry(source) => Some(source),
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

impl From<SymbolRegistryError> for HillslopeSchedulerError {
    fn from(value: SymbolRegistryError) -> Self {
        Self::SymbolRegistry(value)
    }
}

/// Deterministic hillslope scheduler.
#[derive(Debug, Clone)]
pub struct HillslopePhaseScheduler {
    graph: HillslopePhaseGraph,
}

#[derive(Debug, Clone, Copy)]
struct IndexedExecutionContext<'a> {
    symbol_registry: &'a SymbolRegistry,
    hot_symbol_tables: &'a HotSymbolTables,
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
        writeback_surface: HillslopeWritebackSurface,
    ) -> Result<HillslopeKernelExecutionReport, HillslopeSchedulerError>
    where
        K: HillslopeKernel,
    {
        self.execute_with_kernel_indexed(
            topology_report,
            kernel,
            writeback_surface,
            None,
            None,
            None,
        )
    }

    /// Execute deterministic hillslope scheduling with an optional indexed
    /// read mirror synchronized after each accepted logical writeback.
    #[allow(clippy::too_many_lines)]
    pub fn execute_with_kernel_indexed<K>(
        &self,
        topology_report: &TopologyValidationReport,
        kernel: &mut K,
        mut writeback_surface: HillslopeWritebackSurface,
        mut indexed_writeback_surface: Option<IndexedWritebackSurface>,
        symbol_registry: Option<&SymbolRegistry>,
        hot_symbol_tables: Option<&HotSymbolTables>,
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
                let decomposition_dispatch = match decomposition_phase_dispatch_for_state_indexed(
                    phase,
                    &writeback_surface.state_surface,
                    indexed_writeback_surface.as_ref(),
                    hot_symbol_tables,
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
                let growth_dispatch = match growth_phase_dispatch_for_state_indexed(
                    phase,
                    &writeback_surface.state_surface,
                    indexed_writeback_surface.as_ref(),
                    hot_symbol_tables,
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
                let request = HillslopeKernelRequest::with_transition_context_and_indexed(
                    phase.as_str(),
                    phase_class,
                    consumer_adapter,
                    decomposition_context,
                    growth_context,
                    &writeback_surface.state_surface,
                    &writeback_surface.flux_surface,
                    indexed_writeback_surface.as_ref(),
                    hot_symbol_tables,
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
            if let Some(indexed_writeback_surface) = indexed_writeback_surface.as_mut() {
                let Some(symbol_registry) = symbol_registry else {
                    deferred_error = Some(HillslopeSchedulerError::SymbolRegistry(
                        SymbolRegistryError::UnknownSymbol {
                            symbol: BoundarySymbol::from("indexed_execution.registry"),
                        },
                    ));
                    phase_reports.push(HillslopeKernelPhaseReport {
                        phase,
                        kernel_status,
                        decision_outcome: WritebackDecisionOutcome::Reject,
                        decision_status: deferred_error_status.clone(),
                        decision_violations: Vec::new(),
                        apply_result: None,
                    });
                    return deferred_error_status.clone();
                };
                if let Err(source) = indexed_writeback_surface
                    .apply_writeback_payload(symbol_registry, &response.writeback)
                {
                    deferred_error = Some(HillslopeSchedulerError::SymbolRegistry(source));
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
            }

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

    /// Execute one simulation day by running the existing phase graph once per
    /// OFE lane and carrying explicit same-day transfer arrays downstream.
    ///
    /// M-E2 intentionally stops at sequence wiring: it does not persist OFE
    /// dynamic state across days and does not flip public WAT publication.
    ///
    /// # Errors
    ///
    /// Returns `OfeLaneSequenceError` when lane ordering, transfer payloads,
    /// transfer array values, or a lane scheduler result is invalid.
    pub fn execute_ofe_sequence_with_kernel<K>(
        &self,
        topology_report: &TopologyValidationReport,
        kernel: &mut K,
        lane_inputs: Vec<OfeLaneExecutionInput>,
    ) -> Result<OfeLaneSequenceExecutionReport, OfeLaneSequenceError>
    where
        K: HillslopeKernel,
    {
        self.execute_ofe_sequence_with_kernel_internal(topology_report, kernel, lane_inputs, None)
    }

    pub fn execute_ofe_sequence_with_kernel_indexed<K>(
        &self,
        topology_report: &TopologyValidationReport,
        kernel: &mut K,
        lane_inputs: Vec<OfeLaneExecutionInput>,
        symbol_registry: &SymbolRegistry,
        hot_symbol_tables: &HotSymbolTables,
    ) -> Result<OfeLaneSequenceExecutionReport, OfeLaneSequenceError>
    where
        K: HillslopeKernel,
    {
        self.execute_ofe_sequence_with_kernel_internal(
            topology_report,
            kernel,
            lane_inputs,
            Some(IndexedExecutionContext {
                symbol_registry,
                hot_symbol_tables,
            }),
        )
    }

    fn execute_ofe_sequence_with_kernel_internal<K>(
        &self,
        topology_report: &TopologyValidationReport,
        kernel: &mut K,
        lane_inputs: Vec<OfeLaneExecutionInput>,
        indexed_context: Option<IndexedExecutionContext<'_>>,
    ) -> Result<OfeLaneSequenceExecutionReport, OfeLaneSequenceError>
    where
        K: HillslopeKernel,
    {
        if lane_inputs.is_empty() {
            return Err(OfeLaneSequenceError::InvalidLaneCount { lane_count: 0 });
        }

        let lane_count = lane_inputs.len();
        let mut lane_reports = Vec::with_capacity(lane_count);
        let mut next_transfer_input = TransferInput::zero_for_first_ofe();

        for (index, mut lane_input) in lane_inputs.into_iter().enumerate() {
            let expected_ofe_id = index + 1;
            if lane_input.ofe_id != expected_ofe_id {
                return Err(OfeLaneSequenceError::NonSequentialLaneOfeId {
                    expected_ofe_id,
                    observed_ofe_id: lane_input.ofe_id,
                });
            }

            next_transfer_input.area_ratio = lane_input.upstream_area_ratio;
            rescale_transfer_input_daily_totals(&mut next_transfer_input)?;
            apply_transfer_input_to_lane_surface(
                lane_input.ofe_id,
                &next_transfer_input,
                &mut lane_input.writeback_surface,
                lane_input.indexed_writeback_surface.as_mut(),
                indexed_context.map(|context| context.symbol_registry),
            )?;
            let upstream_transfer_input = next_transfer_input.clone();
            let kernel_report = match indexed_context {
                Some(indexed_context) => self.execute_with_kernel_indexed(
                    topology_report,
                    kernel,
                    lane_input.writeback_surface,
                    lane_input.indexed_writeback_surface,
                    Some(indexed_context.symbol_registry),
                    Some(indexed_context.hot_symbol_tables),
                ),
                None => {
                    self.execute_with_kernel(topology_report, kernel, lane_input.writeback_surface)
                }
            }
            .map_err(|source| OfeLaneSequenceError::LaneScheduler {
                ofe_id: lane_input.ofe_id,
                source,
            })?;

            if !kernel_report.scheduler_report.is_success() {
                return Err(OfeLaneSequenceError::LaneExecutionFailed {
                    ofe_id: lane_input.ofe_id,
                    status: kernel_report.scheduler_report.scheduler_status.clone(),
                });
            }

            let is_terminal = lane_input.ofe_id == lane_count;
            let current_transfer_output = extract_transfer_output_from_lane_surface(
                lane_input.ofe_id,
                is_terminal,
                &kernel_report.writeback_surface,
            )?;

            if !is_terminal {
                next_transfer_input = current_transfer_output
                    .as_downstream_input_with_area_ratio(lane_input.upstream_area_ratio)?;
            }

            lane_reports.push(OfeLaneExecutionReport {
                ofe_id: lane_input.ofe_id,
                upstream_transfer_input,
                current_transfer_output,
                kernel_report,
            });
        }

        Ok(OfeLaneSequenceExecutionReport { lane_reports })
    }

    pub fn execute_persistent_ofe_sequence_day_with_kernel<K>(
        &self,
        topology_report: &TopologyValidationReport,
        kernel: &mut K,
        lane_state: &mut OfeLanePersistentStateSequence,
    ) -> Result<OfeLaneSequenceExecutionReport, OfeLaneSequenceError>
    where
        K: HillslopeKernel,
    {
        validate_persistent_lane_states(lane_state.lane_states())?;
        let lane_inputs = lane_state.to_execution_inputs();
        let report = self.execute_ofe_sequence_with_kernel(topology_report, kernel, lane_inputs)?;

        lane_state.replace_from_report(&report)?;

        Ok(report)
    }
}

fn apply_transfer_input_to_lane_surface(
    ofe_id: usize,
    input: &TransferInput,
    writeback_surface: &mut HillslopeWritebackSurface,
    indexed_writeback_surface: Option<&mut IndexedWritebackSurface>,
    symbol_registry: Option<&SymbolRegistry>,
) -> Result<(), OfeLaneSequenceError> {
    validate_transfer_input_for_lane(ofe_id, input)?;
    let mut indexed_writeback_surface = indexed_writeback_surface;
    clear_current_transfer_arrays(
        ofe_id,
        writeback_surface,
        indexed_writeback_surface.as_deref_mut(),
        symbol_registry,
    )?;

    insert_transfer_state_symbol(
        ofe_id,
        writeback_surface,
        indexed_writeback_surface.as_deref_mut(),
        symbol_registry,
        BoundarySymbol::from(MOFE_HOURLY_CARRY_ARRAYS_ENABLED_SYMBOL),
        BoundaryValue::scalar(1.0),
    )?;
    insert_transfer_state_symbol(
        ofe_id,
        writeback_surface,
        indexed_writeback_surface.as_deref_mut(),
        symbol_registry,
        BoundarySymbol::from(MOFE_HOURLY_UPSTREAM_AREA_RATIO_SYMBOL),
        BoundaryValue::scalar(input.area_ratio),
    )?;
    insert_transfer_state_symbol(
        ofe_id,
        writeback_surface,
        indexed_writeback_surface.as_deref_mut(),
        symbol_registry,
        BoundarySymbol::from(MOFE_TRANSFER_INPUT_UPSTRMQ_SYMBOL),
        BoundaryValue::scalar(input.upstrmq),
    )?;
    insert_transfer_state_symbol(
        ofe_id,
        writeback_surface,
        indexed_writeback_surface.as_deref_mut(),
        symbol_registry,
        BoundarySymbol::from(MOFE_TRANSFER_INPUT_SUBRIN_SYMBOL),
        BoundaryValue::scalar(input.subrin),
    )?;
    let daily_transfer_total =
        validate_combined_transfer_total(ofe_id, input.upstrmq, input.subrin)?;
    insert_transfer_state_symbol(
        ofe_id,
        writeback_surface,
        indexed_writeback_surface.as_deref_mut(),
        symbol_registry,
        BoundarySymbol::from(WB12_SYMBOL_RUNON_INPUT),
        BoundaryValue::scalar(daily_transfer_total),
    )?;
    insert_transfer_flux_symbol(
        ofe_id,
        writeback_surface,
        indexed_writeback_surface.as_deref_mut(),
        symbol_registry,
        BoundarySymbol::from(WB12_SYMBOL_RUNOFF_CARRYOVER),
        BoundaryValue::scalar(daily_transfer_total),
    )?;

    for (index, value) in input.surface_carry.iter().copied().enumerate() {
        insert_transfer_state_symbol(
            ofe_id,
            writeback_surface,
            indexed_writeback_surface.as_deref_mut(),
            symbol_registry,
            mofe_hourly_symbol(MOFE_HOURLY_UPSTREAM_SATURATION_RUNOFF_ROOT, index + 1),
            BoundaryValue::scalar(value),
        )?;
    }
    for (index, value) in input.lateral_carry.iter().copied().enumerate() {
        insert_transfer_state_symbol(
            ofe_id,
            writeback_surface,
            indexed_writeback_surface.as_deref_mut(),
            symbol_registry,
            mofe_hourly_symbol(MOFE_HOURLY_UPSTREAM_LATERAL_RUNOFF_ROOT, index + 1),
            BoundaryValue::scalar(value),
        )?;
    }

    Ok(())
}

fn insert_transfer_state_symbol(
    ofe_id: usize,
    writeback_surface: &mut HillslopeWritebackSurface,
    indexed_writeback_surface: Option<&mut IndexedWritebackSurface>,
    symbol_registry: Option<&SymbolRegistry>,
    symbol: BoundarySymbol,
    value: BoundaryValue,
) -> Result<(), OfeLaneSequenceError> {
    if let Some(indexed_writeback_surface) = indexed_writeback_surface {
        let Some(symbol_registry) = symbol_registry else {
            return Err(OfeLaneSequenceError::IndexedSymbolRegistry {
                ofe_id,
                source: SymbolRegistryError::UnknownSymbol {
                    symbol: BoundarySymbol::from("indexed_execution.registry"),
                },
            });
        };
        indexed_writeback_surface
            .set_state_symbol(symbol_registry, &symbol, Some(value))
            .map_err(|source| OfeLaneSequenceError::IndexedSymbolRegistry { ofe_id, source })?;
    }
    writeback_surface.state_surface.insert(symbol, value);
    Ok(())
}

fn insert_transfer_flux_symbol(
    ofe_id: usize,
    writeback_surface: &mut HillslopeWritebackSurface,
    indexed_writeback_surface: Option<&mut IndexedWritebackSurface>,
    symbol_registry: Option<&SymbolRegistry>,
    symbol: BoundarySymbol,
    value: BoundaryValue,
) -> Result<(), OfeLaneSequenceError> {
    if let Some(indexed_writeback_surface) = indexed_writeback_surface {
        let Some(symbol_registry) = symbol_registry else {
            return Err(OfeLaneSequenceError::IndexedSymbolRegistry {
                ofe_id,
                source: SymbolRegistryError::UnknownSymbol {
                    symbol: BoundarySymbol::from("indexed_execution.registry"),
                },
            });
        };
        indexed_writeback_surface
            .set_flux_symbol(symbol_registry, &symbol, Some(value))
            .map_err(|source| OfeLaneSequenceError::IndexedSymbolRegistry { ofe_id, source })?;
    }
    writeback_surface.flux_surface.insert(symbol, value);
    Ok(())
}

fn validate_transfer_input_for_lane(
    ofe_id: usize,
    input: &TransferInput,
) -> Result<(), OfeLaneSequenceError> {
    if input.recipient_ofe_id != ofe_id {
        return Err(PerOfeDailyWaterBalanceError::TransferRecipientMismatch {
            ofe_id,
            recipient_ofe_id: input.recipient_ofe_id,
        }
        .into());
    }
    let expected_source = if ofe_id == 1 { None } else { Some(ofe_id - 1) };
    if input.source_ofe_id != expected_source {
        return Err(PerOfeDailyWaterBalanceError::TransferInputSourceMismatch {
            ofe_id,
            expected_source_ofe_id: expected_source,
            observed_source_ofe_id: input.source_ofe_id,
        }
        .into());
    }

    let surface_total = validate_transfer_array(
        ofe_id,
        MOFE_HOURLY_UPSTREAM_SATURATION_RUNOFF_ROOT,
        &input.surface_carry,
    )?;
    let lateral_total = validate_transfer_array(
        ofe_id,
        MOFE_HOURLY_UPSTREAM_LATERAL_RUNOFF_ROOT,
        &input.lateral_carry,
    )?;
    validate_positive_transfer_scalar(
        ofe_id,
        MOFE_HOURLY_UPSTREAM_AREA_RATIO_SYMBOL,
        input.area_ratio,
    )?;
    validate_transfer_scalar(ofe_id, MOFE_TRANSFER_INPUT_UPSTRMQ_SYMBOL, input.upstrmq)?;
    validate_transfer_scalar(ofe_id, MOFE_TRANSFER_INPUT_SUBRIN_SYMBOL, input.subrin)?;
    validate_transfer_daily_sum(
        ofe_id,
        MOFE_TRANSFER_INPUT_UPSTRMQ_SYMBOL,
        scaled_transfer_total(
            ofe_id,
            MOFE_TRANSFER_INPUT_UPSTRMQ_SYMBOL,
            surface_total,
            input.area_ratio,
        )?,
        input.upstrmq,
    )?;
    validate_transfer_daily_sum(
        ofe_id,
        MOFE_TRANSFER_INPUT_SUBRIN_SYMBOL,
        scaled_transfer_total(
            ofe_id,
            MOFE_TRANSFER_INPUT_SUBRIN_SYMBOL,
            lateral_total,
            input.area_ratio,
        )?,
        input.subrin,
    )?;

    Ok(())
}

fn rescale_transfer_input_daily_totals(
    input: &mut TransferInput,
) -> Result<(), OfeLaneSequenceError> {
    validate_positive_transfer_scalar(
        input.recipient_ofe_id,
        MOFE_HOURLY_UPSTREAM_AREA_RATIO_SYMBOL,
        input.area_ratio,
    )?;
    let surface_total: f64 = input.surface_carry.iter().sum();
    let lateral_total: f64 = input.lateral_carry.iter().sum();
    input.upstrmq = scaled_transfer_total(
        input.recipient_ofe_id,
        MOFE_TRANSFER_INPUT_UPSTRMQ_SYMBOL,
        surface_total,
        input.area_ratio,
    )?;
    input.subrin = scaled_transfer_total(
        input.recipient_ofe_id,
        MOFE_TRANSFER_INPUT_SUBRIN_SYMBOL,
        lateral_total,
        input.area_ratio,
    )?;
    Ok(())
}

fn extract_transfer_output_from_lane_surface(
    ofe_id: usize,
    is_terminal: bool,
    writeback_surface: &HillslopeWritebackSurface,
) -> Result<TransferOutput, OfeLaneSequenceError> {
    let surface_carry = read_transfer_array_from_state_surface(
        ofe_id,
        MOFE_HOURLY_CURRENT_SATURATION_RUNOFF_ROOT,
        writeback_surface,
    )?;
    let lateral_carry = read_transfer_array_from_state_surface(
        ofe_id,
        MOFE_HOURLY_CURRENT_LATERAL_RUNOFF_ROOT,
        writeback_surface,
    )?;
    let qofe = validate_transfer_array(
        ofe_id,
        MOFE_HOURLY_CURRENT_SATURATION_RUNOFF_ROOT,
        &surface_carry,
    )?;
    let lateral_export = validate_transfer_array(
        ofe_id,
        MOFE_HOURLY_CURRENT_LATERAL_RUNOFF_ROOT,
        &lateral_carry,
    )?;

    Ok(TransferOutput {
        source_ofe_id: ofe_id,
        recipient_ofe_id: if is_terminal { None } else { Some(ofe_id + 1) },
        surface_carry,
        lateral_carry,
        qofe,
        lateral_export,
    })
}

fn read_transfer_array_from_state_surface(
    ofe_id: usize,
    root: &str,
    writeback_surface: &HillslopeWritebackSurface,
) -> Result<[f64; MOFE_TRANSFER_HOUR_COUNT], OfeLaneSequenceError> {
    let mut values = [0.0; MOFE_TRANSFER_HOUR_COUNT];
    for (index, value) in values.iter_mut().enumerate() {
        let hour = index + 1;
        let symbol = mofe_hourly_symbol(root, hour);
        let Some(raw_value) = writeback_surface.state_surface.get(&symbol).copied() else {
            return Err(OfeLaneSequenceError::InvalidTransferValue {
                ofe_id,
                symbol: symbol.as_str().to_owned(),
                hour: Some(hour),
                value: f64::NAN,
            });
        };
        let scalar = raw_value.as_f64();
        validate_transfer_value(ofe_id, symbol.as_str(), Some(hour), scalar)?;
        *value = scalar;
    }
    Ok(values)
}

fn validate_transfer_array(
    ofe_id: usize,
    root: &str,
    values: &[f64; MOFE_TRANSFER_HOUR_COUNT],
) -> Result<f64, OfeLaneSequenceError> {
    let mut total = 0.0;
    for (index, value) in values.iter().copied().enumerate() {
        let hour = index + 1;
        let symbol = mofe_hourly_symbol(root, hour);
        validate_transfer_value(ofe_id, symbol.as_str(), Some(hour), value)?;
        total += value;
    }
    validate_transfer_value(ofe_id, root, None, total)?;
    Ok(total)
}

fn validate_transfer_scalar(
    ofe_id: usize,
    symbol: &'static str,
    value: f64,
) -> Result<(), OfeLaneSequenceError> {
    validate_transfer_value(ofe_id, symbol, None, value)
}

fn validate_positive_transfer_scalar(
    ofe_id: usize,
    symbol: &'static str,
    value: f64,
) -> Result<(), OfeLaneSequenceError> {
    if !value.is_finite() || value <= WB11_ZERO_THRESHOLD {
        return Err(OfeLaneSequenceError::InvalidTransferValue {
            ofe_id,
            symbol: symbol.to_owned(),
            hour: None,
            value,
        });
    }
    Ok(())
}

fn validate_transfer_value(
    ofe_id: usize,
    symbol: &str,
    hour: Option<usize>,
    value: f64,
) -> Result<(), OfeLaneSequenceError> {
    if !value.is_finite() || value < -WB11_ZERO_THRESHOLD {
        return Err(OfeLaneSequenceError::InvalidTransferValue {
            ofe_id,
            symbol: symbol.to_owned(),
            hour,
            value,
        });
    }
    Ok(())
}

fn validate_combined_transfer_total(
    ofe_id: usize,
    upstrmq: f64,
    subrin: f64,
) -> Result<f64, OfeLaneSequenceError> {
    let total = upstrmq + subrin;
    validate_transfer_value(ofe_id, WB12_SYMBOL_RUNOFF_CARRYOVER, None, total)?;
    Ok(total)
}

fn scaled_transfer_total(
    ofe_id: usize,
    symbol: &'static str,
    total: f64,
    area_ratio: f64,
) -> Result<f64, OfeLaneSequenceError> {
    let scaled = total * area_ratio;
    validate_transfer_value(ofe_id, symbol, None, scaled)?;
    Ok(scaled)
}

fn clear_current_transfer_arrays(
    ofe_id: usize,
    writeback_surface: &mut HillslopeWritebackSurface,
    mut indexed_writeback_surface: Option<&mut IndexedWritebackSurface>,
    symbol_registry: Option<&SymbolRegistry>,
) -> Result<(), OfeLaneSequenceError> {
    for hour in 1..=MOFE_TRANSFER_HOUR_COUNT {
        remove_transfer_state_symbol(
            ofe_id,
            writeback_surface,
            indexed_writeback_surface.as_deref_mut(),
            symbol_registry,
            &mofe_hourly_symbol(MOFE_HOURLY_CURRENT_SATURATION_RUNOFF_ROOT, hour),
        )?;
        remove_transfer_state_symbol(
            ofe_id,
            writeback_surface,
            indexed_writeback_surface.as_deref_mut(),
            symbol_registry,
            &mofe_hourly_symbol(MOFE_HOURLY_CURRENT_LATERAL_RUNOFF_ROOT, hour),
        )?;
    }
    Ok(())
}

fn remove_transfer_state_symbol(
    ofe_id: usize,
    writeback_surface: &mut HillslopeWritebackSurface,
    indexed_writeback_surface: Option<&mut IndexedWritebackSurface>,
    symbol_registry: Option<&SymbolRegistry>,
    symbol: &BoundarySymbol,
) -> Result<(), OfeLaneSequenceError> {
    if let Some(indexed_writeback_surface) = indexed_writeback_surface {
        let Some(symbol_registry) = symbol_registry else {
            return Err(OfeLaneSequenceError::IndexedSymbolRegistry {
                ofe_id,
                source: SymbolRegistryError::UnknownSymbol {
                    symbol: BoundarySymbol::from("indexed_execution.registry"),
                },
            });
        };
        indexed_writeback_surface
            .set_state_symbol(symbol_registry, symbol, None)
            .map_err(|source| OfeLaneSequenceError::IndexedSymbolRegistry { ofe_id, source })?;
    }
    writeback_surface.state_surface.remove(symbol);
    Ok(())
}

fn validate_transfer_daily_sum(
    ofe_id: usize,
    symbol: &'static str,
    expected: f64,
    observed: f64,
) -> Result<(), OfeLaneSequenceError> {
    if (expected - observed).abs() > WB11_ZERO_THRESHOLD {
        return Err(OfeLaneSequenceError::TransferDailySumMismatch {
            ofe_id,
            symbol,
            expected,
            observed,
        });
    }
    Ok(())
}

fn mofe_hourly_symbol(root: &str, hour: usize) -> BoundarySymbol {
    BoundarySymbol::from(format!("{root}_{hour:04}"))
}

impl Default for HillslopePhaseScheduler {
    fn default() -> Self {
        Self::canonical()
    }
}

/// Build resolve-once hot symbol id tables for Stage-4 indexed reads.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn build_hillslope_hot_symbol_tables(registry: &SymbolRegistry) -> HotSymbolTables {
    HotSymbolTables::from_registry(
        registry,
        &[
            MOFE_HOURLY_CARRY_ARRAYS_ENABLED_SYMBOL,
            MOFE_HOURLY_UPSTREAM_AREA_RATIO_SYMBOL,
            MOFE_TRANSFER_INPUT_UPSTRMQ_SYMBOL,
            MOFE_TRANSFER_INPUT_SUBRIN_SYMBOL,
            "wb12_runon_input",
            WB20_SYMBOL_FORWARD_SOLVER_LANE_ENABLED,
            WB19_SYMBOL_LATERAL_DRAIN_LANE_SUBSTEPS,
            WB19_SYMBOL_DRAIN_ENABLED,
            WB19_SYMBOL_DRAIN_DEPTH,
            WB19_SYMBOL_DRAIN_SPACING,
            WB19_SYMBOL_DRAIN_DIAMETER,
            PL_SCHEDULE_SLOT_COUNT_SYMBOL,
            PL_SCHEDULE_ROTATION_REPEATS_SYMBOL,
            PL_SCHEDULE_ROTATION_YEARS_SYMBOL,
            PL_RUNTIME_DAY_SYMBOL,
            PL_RUNTIME_YEAR_SYMBOL,
            "wb11_nsl",
            "nsl",
            "solwpv",
            "frost.runtime_thermal_conductivity_landuse_class_proxy",
        ],
        &[WB12_SYMBOL_RUNOFF_CARRYOVER],
        &[
            "timem",
            "intsty",
            "obmaxt",
            "obmint",
            "snow.hourly.depth_before_m",
            "snow.hourly.depth_available_m",
            "snow.hourly.density_before_kg_m3",
            "snow.hourly.depth_after_m",
            "snow.hourly.density_after_kg_m3",
            "snow.hourly.melt_m",
            "snow.hourly.melt_raw_m",
            "snow.hourly.melt_amelt_in",
            "snow.hourly.melt_bmelt_in",
            "snow.hourly.melt_cmelt_in",
            "snow.hourly.melt_dmelt_in",
            "snow.hourly.melt_hrtef_f",
            "snow.hourly.melt_hrdtf_f",
            "snow.hourly.melt_vwmph",
            "snow.hourly.melt_rainin",
            "snow.hourly.melt_wind_adjustment",
            "snow.hourly.melt_branch_active",
            "snow.hourly.rain_m",
            "snow.hourly.rain_retained_m",
            "snow.hourly.rain_released_m",
            "snow.hourly.snowfall_m",
            "winter.hourly.rad_mj_m2",
            "winter.hourly.air_temp_c",
            "winter.hourly.cloud_fraction",
            "winter.hourly.dewpoint_c",
            "winter.hourly.wind_m_s",
            "frost.hourly.qsrf_w_m2",
            "frost.hourly.quf_w_m2",
            "frost.hourly.ksrf_w_m_k",
            "frost.hourly.surface_temp_c",
            "frost.hourly.snow_depth_m",
            "frost.hourly.residue_depth_m",
            "frost.hourly.tilled_frozen_depth_m",
            "frost.hourly.untilled_frozen_depth_m",
            "frost.hourly.frzflg",
            MOFE_HOURLY_UPSTREAM_SATURATION_RUNOFF_ROOT,
            MOFE_HOURLY_UPSTREAM_LATERAL_RUNOFF_ROOT,
            MOFE_HOURLY_CURRENT_SATURATION_RUNOFF_ROOT,
            MOFE_HOURLY_CURRENT_LATERAL_RUNOFF_ROOT,
            "wb18_perc_theta",
            "wb18_perc_fc",
            "wb18_perc_ul",
            "wb18_perc_ssc",
            "wb18_perc_frzw",
            "wb18_perc_frozen_depth",
            "wb19_dg",
            "dg",
            "wb19_coca",
            "coca",
            "wb19_por",
            "por",
            "cpm",
            "wb19_thetfc",
            "thetfc",
            "wb19_thetdr",
            "thetdr",
            "wb19_bulk_density_kg_m3",
            WB19_SYMBOL_LATERAL_SSH_ROOT,
            WB19_SYMBOL_LATERAL_WITHDRAWAL_ROOT,
            "frost.runtime_nfine",
            "frost.runtime_fine_thickness_m",
            "frost.runtime_yst_m",
            "frost.runtime_nwfrzz_m",
        ],
        &["wb18_perc_pei"],
        &[
            "frost.runtime_fgfrst",
            "frost.runtime_slfsd_m",
            "frost.runtime_slsic_m",
            "frost.runtime_slsw_theta",
            "frost.runtime_sltime_s",
        ],
        &[],
    )
}
