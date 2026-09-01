//! Typed, default-off Stage-3/V11 parent attachment.
//!
//! This boundary owns the constitutive Stage-3 support cadence and terminal
//! event projection.  It deliberately accepts a prepared forcing capability
//! rather than an event request or live carrier receipt.  The legacy
//! caller-built handoff remains test-only in `direct_runtime::snow_stage3_shadow`.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[path = "snow_stage3_v11_profile.rs"]
mod profile;
pub use profile::AdaptiveParentProfileDetailV1;

use openwepp_coupled_time::{
    AcceptedEventReceiptV1, ConstraintClass, CoupledClockStateV1, CoupledSlabCandidateV1, Digest32,
    EventClass, EventProposalV1, EventQueueV1, FramedField, LedgerEntryV1, ModelTimeNs, OwnerState,
    ParentAuthorityV1, ParentIntervalId, ParentTransactionId, StepConstraintV1, TimeSupport,
    accept_slab, complete_owner_set_digest, digest_bytes, framed_sha256, quantize_seconds_to_tick,
    reduce_constraints,
};
use openwepp_kernel_contract::{SoilLayerId, TileId};
use openwepp_land_surface_energy::OfeId;
use openwepp_meteorology::psychrometrics::saturation_vapor_pressure_water_kpa;
use openwepp_meteorology::snow_free_forcing::{celsius_to_kelvin, kilopascals_to_pascals};
use openwepp_unit_boundary::TemperatureCelsius;
use openwepp_vegetation::v11::{
    V11OwnerEnvelope, V11ParentCandidate, V11ParentTransaction, VegetationConfigurationV11,
    migrate_v10_runtime_to_v11, v11_vegetation_owner_envelope,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::hydrology::{
    CoveredProbeChildIdentityV1, CoveredTerminalExecutionMode, CoveredTerminalJointTrialStateV1,
    DirectActiveSnowPartitionInputs, DirectSnowHourlyForcing, DirectSnowStage3EvaluationError,
    DirectSnowStage3PersistentDayResult, DirectSnowStage3PersistentState,
    DirectSnowStage3SupportInput, DirectSnowTerminalEventRequest, DirectSnowTerminalEventResult,
    JointTrialAuthorityV1, ProbeChildAuthorityV1, Wb11HydrologyKernel, stage3_has_represented_ice,
    stage3_is_resolved_thermal_domain, stage3_is_terminal_event_domain,
};
use crate::runtime_inputs::{
    HillslopeClimateRuntimeRequest, PreparedSnowFreeGsiDayV1, SnowFreeHalfHourForcingError,
    SnowFreeHalfHourIntervalReceipt, SnowFreeHalfHourProviderCursor,
    SnowFreePrecipitationParcelReceipt, SnowFreeSolidPrecipitationParcelReceipt, direct_gsi_state,
};
use crate::snow_stage3_open_boundary::{
    FinalStage3TileBoundaryReceiptV1, SealedOpenSnowExposureReceiptV1,
    SealedOpenSnowTileForcingInputsV1, SealedOpenSnowTileForcingV1,
    SealedStage3TileBoundaryForcingV1,
};
use crate::snow_stage3_terminal_handoff::{
    LaneStage3BoundaryReceiptV1, SealedCoveredCarrierForcing, SnowStage3HandoffError,
};
use crate::v9_real_consumer_shadow::DirectV10RealConsumerShadow;
use crate::v9_real_consumer_shadow::{
    CoveredParentOwnerJoinReceiptV1, CoveredPhysicalCustodyJoinInputs, DirectV9ShadowIntervalInput,
    DirectV11RealConsumerError, DirectV11RealConsumerStack, DirectV11SnowCoveredRealConsumerStack,
    DirectV11SnowCoveredSegmentInput, DirectV11SnowCoveredStackInputs,
    Stage3AcceptedSnowLiquidOutputV1,
};
pub use crate::v9_real_consumer_shadow::{
    Stage3PhysicalOutcomeClosureAuditV1, begin_stage3_physical_outcome_closure_audit_v1,
    take_stage3_physical_outcome_closure_audit_v1,
};
use crate::v11_vegetation_consumer::{accept_direct_v11_segment, execute_direct_v11_segment};
use crate::winter_column::DirectSnowLayerState;
use crate::{DirectSurfaceLiquidConfiguration, DirectSurfaceLiquidConfigurationRecord};

pub const STAGE3_V11_PARENT_SUPPORT_NS: u128 = 1_800_000_000_000;
pub const STAGE3_V11_PARENT_SUPPORT_COUNT: usize = 48;
pub const STAGE3_V11_DAY_NS: u128 = 86_400_000_000_000;
pub const STAGE3_V11_ADAPTIVE_MINIMUM_SUPPORT_NS: u128 = 60_000_000_000;
/// Maximum accepted final receipt-reseal roundoff. This is three orders of
/// magnitude below the unchanged Stage-3 energy-ledger closure threshold.
pub const STAGE3_V11_SNOW_SOIL_RECEIPT_RESEAL_ROUNDOFF_J_M2: f64 = 1.0e-9;
pub const STAGE3_V11_SNOW_SOIL_RECEIPT_RESEAL_ROUNDOFF_TEMPERATURE_K: f64 = 1.0e-8;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CoveredFixedPointIterationAuditV1 {
    pub support: TimeSupport,
    pub completed_iterations: usize,
    pub converged: bool,
    pub limit_detail: Option<CoveredFixedPointLimitDetailV1>,
    pub receipt_reseal_max_abs_residual_bits: u64,
    pub receipt_reseal_max_abs_temperature_residual_bits: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoveredFixedPointLimitStageV1 {
    Picard,
    Finalization,
    ReceiptReplay,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CoveredFixedPointLimitDetailV1 {
    pub stage: CoveredFixedPointLimitStageV1,
    pub lse_converged: bool,
    pub stage3_converged: bool,
    pub soil_converged: bool,
    pub boundary_converged: bool,
    pub stage3_first_difference: Option<(u32, &'static str, u64, u64, u64, u64)>,
}

/// One bounded, diagnostic-only observation of the covered fixed-point map.
///
/// Delta fields are stored as IEEE-754 bits so this evidence remains exactly
/// comparable without making floating-point values part of owner state. A
/// normalized delta at or below one satisfies the corresponding dimensional
/// norm. The booleans remain authoritative for exact-field predicates, while
/// infinity denotes a topology or other non-numeric mismatch.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CoveredFixedPointLimiterSampleV1 {
    pub support: TimeSupport,
    pub iteration: usize,
    pub stage: CoveredFixedPointLimitStageV1,
    pub lse_converged: bool,
    pub stage3_converged: bool,
    pub soil_converged: bool,
    pub boundary_converged: bool,
    pub lse_max_normalized_delta_bits: u64,
    pub stage3_max_normalized_delta_bits: u64,
    pub soil_enthalpy_max_normalized_delta_bits: u64,
    pub soil_temperature_max_normalized_delta_bits: u64,
    pub boundary_max_normalized_delta_bits: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoveredFixedPointLimiterAuditV1 {
    pub total_sample_count: u64,
    pub dropped_sample_count: u64,
    pub retained_tail: Vec<CoveredFixedPointLimiterSampleV1>,
    pub peak_lse_normalized_delta_bits: u64,
    pub peak_stage3_normalized_delta_bits: u64,
    pub peak_soil_enthalpy_normalized_delta_bits: u64,
    pub peak_soil_temperature_normalized_delta_bits: u64,
    pub peak_boundary_normalized_delta_bits: u64,
}

const COVERED_FIXED_POINT_LIMITER_AUDIT_CAPACITY: usize = 384;

struct CoveredFixedPointLimiterAuditStateV1 {
    total_sample_count: u64,
    dropped_sample_count: u64,
    retained_tail: VecDeque<CoveredFixedPointLimiterSampleV1>,
    peak_lse_normalized_delta_bits: u64,
    peak_stage3_normalized_delta_bits: u64,
    peak_soil_enthalpy_normalized_delta_bits: u64,
    peak_soil_temperature_normalized_delta_bits: u64,
    peak_boundary_normalized_delta_bits: u64,
}

impl Default for CoveredFixedPointLimiterAuditStateV1 {
    fn default() -> Self {
        Self {
            total_sample_count: 0,
            dropped_sample_count: 0,
            retained_tail: VecDeque::with_capacity(COVERED_FIXED_POINT_LIMITER_AUDIT_CAPACITY),
            peak_lse_normalized_delta_bits: 0.0_f64.to_bits(),
            peak_stage3_normalized_delta_bits: 0.0_f64.to_bits(),
            peak_soil_enthalpy_normalized_delta_bits: 0.0_f64.to_bits(),
            peak_soil_temperature_normalized_delta_bits: 0.0_f64.to_bits(),
            peak_boundary_normalized_delta_bits: 0.0_f64.to_bits(),
        }
    }
}

std::thread_local! {
    static COVERED_FIXED_POINT_ITERATION_AUDIT_ENABLED: std::cell::Cell<bool> = const {
        std::cell::Cell::new(false)
    };
    static COVERED_FIXED_POINT_ITERATION_AUDIT: std::cell::RefCell<
        Vec<CoveredFixedPointIterationAuditV1>,
    > = const { std::cell::RefCell::new(Vec::new()) };
    static COVERED_FIXED_POINT_LIMIT_DETAIL: std::cell::Cell<
        Option<CoveredFixedPointLimitDetailV1>
    > = const { std::cell::Cell::new(None) };
    static COVERED_RECEIPT_RESEAL_MAX_ABS_RESIDUAL_BITS: std::cell::Cell<u64> = const {
        std::cell::Cell::new(0.0_f64.to_bits())
    };
    static COVERED_RECEIPT_RESEAL_MAX_ABS_TEMPERATURE_RESIDUAL_BITS: std::cell::Cell<u64> = const {
        std::cell::Cell::new(0.0_f64.to_bits())
    };
    static COVERED_FIXED_POINT_LIMITER_AUDIT: std::cell::RefCell<
        Option<CoveredFixedPointLimiterAuditStateV1>
    > = const { std::cell::RefCell::new(None) };
}

pub struct CoveredFixedPointIterationAuditGuardV1 {
    _not_send: std::marker::PhantomData<std::rc::Rc<()>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdaptiveParentTelemetryV1 {
    pub parent_ordinal: usize,
    pub support: TimeSupport,
    pub direct_trial_count: u64,
    pub split_child_trial_count: u64,
    pub accepted_microstep_count: u64,
    pub rejected_candidate_count: u64,
    pub owner_join_count: usize,
    pub event_group_count: usize,
    pub terminal_parcel_count: usize,
    pub publication_support_count: usize,
    pub publication_event_count: usize,
    pub adaptive_receipt_bytes: Option<usize>,
    pub coupled_receipt_inline_bytes: usize,
    pub retained_complete_owner_bytes: Option<usize>,
    pub accepted_width_histogram: Vec<(u128, u64)>,
    pub phase_rejection_count: u64,
    pub event_rejection_count: u64,
    pub phase_and_event_rejection_count: u64,
    pub other_rejection_count: u64,
    pub covered_direct_trial_phase_count: u64,
    pub covered_direct_trial_phase_elapsed: std::time::Duration,
    pub covered_composed_trial_phase_count: u64,
    pub covered_composed_trial_phase_elapsed: std::time::Duration,
    pub terminal_direct_trial_phase_count: u64,
    pub terminal_direct_trial_phase_elapsed: std::time::Duration,
    pub terminal_composed_trial_phase_count: u64,
    pub terminal_composed_trial_phase_elapsed: std::time::Duration,
    pub fixed_point_evaluation_count: u64,
    pub fixed_point_iteration_total: u64,
    pub fixed_point_iteration_maximum: usize,
    pub fixed_point_operand_elapsed: std::time::Duration,
    pub fixed_point_envelope_elapsed: std::time::Duration,
    pub provisional_envelope_projection_elapsed: std::time::Duration,
    pub provisional_envelope_solver_ready_elapsed: std::time::Duration,
    pub provisional_envelope_physical_elapsed: std::time::Duration,
    pub provisional_envelope_receipts_elapsed: std::time::Duration,
    pub provisional_envelope_owner_elapsed: std::time::Duration,
    pub profile_detail: AdaptiveParentProfileDetailV1,
    pub fixed_point_stage3_elapsed: std::time::Duration,
    pub fixed_point_soil_elapsed: std::time::Duration,
    pub fixed_point_finalization_elapsed: std::time::Duration,
    pub publication_append_count: u64,
    pub publication_append_elapsed: std::time::Duration,
    pub publication_cow_count: u64,
    pub publication_full_validation_count: u64,
    pub publication_full_validation_elapsed: std::time::Duration,
    pub reuse_validation_count: u64,
    pub reuse_validation_elapsed: std::time::Duration,
    pub reuse_hit_count: u64,
    pub reuse_fallback_count: u64,
    pub covered_child_memo_hit_count: u64,
    pub covered_child_memo_fallback_count: u64,
    pub covered_child_memo_direct_hit_count: u64,
    pub covered_child_memo_composed_hit_count: u64,
    pub parent_elapsed: std::time::Duration,
    pub cumulative_elapsed: std::time::Duration,
}

#[derive(Default)]
struct AdaptiveParentTelemetryAccumulatorV1 {
    covered_direct_trial_phase_count: u64,
    covered_direct_trial_phase_elapsed: std::time::Duration,
    covered_composed_trial_phase_count: u64,
    covered_composed_trial_phase_elapsed: std::time::Duration,
    terminal_direct_trial_phase_count: u64,
    terminal_direct_trial_phase_elapsed: std::time::Duration,
    terminal_composed_trial_phase_count: u64,
    terminal_composed_trial_phase_elapsed: std::time::Duration,
    fixed_point_evaluation_count: u64,
    fixed_point_iteration_total: u64,
    fixed_point_iteration_maximum: usize,
    fixed_point_operand_elapsed: std::time::Duration,
    fixed_point_envelope_elapsed: std::time::Duration,
    provisional_envelope_projection_elapsed: std::time::Duration,
    provisional_envelope_solver_ready_elapsed: std::time::Duration,
    provisional_envelope_physical_elapsed: std::time::Duration,
    provisional_envelope_receipts_elapsed: std::time::Duration,
    provisional_envelope_owner_elapsed: std::time::Duration,
    profile_detail: AdaptiveParentProfileDetailV1,
    fixed_point_stage3_elapsed: std::time::Duration,
    fixed_point_soil_elapsed: std::time::Duration,
    fixed_point_finalization_elapsed: std::time::Duration,
    publication_append_count: u64,
    publication_append_elapsed: std::time::Duration,
    publication_cow_count: u64,
    publication_full_validation_count: u64,
    publication_full_validation_elapsed: std::time::Duration,
    reuse_validation_count: u64,
    reuse_validation_elapsed: std::time::Duration,
    reuse_hit_count: u64,
    reuse_fallback_count: u64,
    covered_child_memo_hit_count: u64,
    covered_child_memo_fallback_count: u64,
    covered_child_memo_direct_hit_count: u64,
    covered_child_memo_composed_hit_count: u64,
    phase_rejection_count: u64,
    event_rejection_count: u64,
    phase_and_event_rejection_count: u64,
    other_rejection_count: u64,
}

struct AdaptiveParentTelemetryStateV1 {
    started: std::time::Instant,
    maximum_completed_parents: usize,
    maximum_elapsed: std::time::Duration,
    rows: Vec<AdaptiveParentTelemetryV1>,
    accumulator: AdaptiveParentTelemetryAccumulatorV1,
}

std::thread_local! {
    static ADAPTIVE_PARENT_TELEMETRY: std::cell::RefCell<Option<AdaptiveParentTelemetryStateV1>> = const {
        std::cell::RefCell::new(None)
    };
}

pub struct AdaptiveParentTelemetryGuardV1 {
    _not_send: std::marker::PhantomData<std::rc::Rc<()>>,
}

impl Drop for AdaptiveParentTelemetryGuardV1 {
    fn drop(&mut self) {
        ADAPTIVE_PARENT_TELEMETRY.with(|state| *state.borrow_mut() = None);
    }
}

/// Enables result-blind, diagnostic-only completed-parent telemetry. The stop
/// decision uses only completed-parent count and wall time; neither telemetry
/// nor the stop enters owner identity, receipts, controller decisions, or wire.
pub fn begin_adaptive_parent_telemetry_v1(
    maximum_completed_parents: usize,
    maximum_elapsed: std::time::Duration,
) -> Result<AdaptiveParentTelemetryGuardV1, DirectSnowStage3V11AttachmentError> {
    if maximum_completed_parents == 0 || maximum_elapsed.is_zero() {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "adaptive parent telemetry bound",
        ));
    }
    ADAPTIVE_PARENT_TELEMETRY.with(|state| {
        *state.borrow_mut() = Some(AdaptiveParentTelemetryStateV1 {
            started: std::time::Instant::now(),
            maximum_completed_parents,
            maximum_elapsed,
            rows: Vec::new(),
            accumulator: AdaptiveParentTelemetryAccumulatorV1::default(),
        });
    });
    Ok(AdaptiveParentTelemetryGuardV1 {
        _not_send: std::marker::PhantomData,
    })
}

pub fn take_adaptive_parent_telemetry_v1() -> Vec<AdaptiveParentTelemetryV1> {
    ADAPTIVE_PARENT_TELEMETRY.with(|state| {
        state
            .borrow_mut()
            .take()
            .map_or_else(Vec::new, |state| state.rows)
    })
}

pub(crate) fn record_adaptive_parent_telemetry_v1(mut row: AdaptiveParentTelemetryV1) -> bool {
    ADAPTIVE_PARENT_TELEMETRY.with(|state| {
        let mut state = state.borrow_mut();
        let Some(state) = state.as_mut() else {
            return false;
        };
        let accumulator = std::mem::take(&mut state.accumulator);
        row.covered_direct_trial_phase_count = accumulator.covered_direct_trial_phase_count;
        row.covered_direct_trial_phase_elapsed = accumulator.covered_direct_trial_phase_elapsed;
        row.covered_composed_trial_phase_count = accumulator.covered_composed_trial_phase_count;
        row.covered_composed_trial_phase_elapsed = accumulator.covered_composed_trial_phase_elapsed;
        row.terminal_direct_trial_phase_count = accumulator.terminal_direct_trial_phase_count;
        row.terminal_direct_trial_phase_elapsed = accumulator.terminal_direct_trial_phase_elapsed;
        row.terminal_composed_trial_phase_count = accumulator.terminal_composed_trial_phase_count;
        row.terminal_composed_trial_phase_elapsed =
            accumulator.terminal_composed_trial_phase_elapsed;
        row.fixed_point_evaluation_count = accumulator.fixed_point_evaluation_count;
        row.fixed_point_iteration_total = accumulator.fixed_point_iteration_total;
        row.fixed_point_iteration_maximum = accumulator.fixed_point_iteration_maximum;
        row.fixed_point_operand_elapsed = accumulator.fixed_point_operand_elapsed;
        row.fixed_point_envelope_elapsed = accumulator.fixed_point_envelope_elapsed;
        row.provisional_envelope_projection_elapsed =
            accumulator.provisional_envelope_projection_elapsed;
        row.provisional_envelope_solver_ready_elapsed =
            accumulator.provisional_envelope_solver_ready_elapsed;
        row.provisional_envelope_physical_elapsed =
            accumulator.provisional_envelope_physical_elapsed;
        row.provisional_envelope_receipts_elapsed =
            accumulator.provisional_envelope_receipts_elapsed;
        row.provisional_envelope_owner_elapsed = accumulator.provisional_envelope_owner_elapsed;
        row.profile_detail = accumulator.profile_detail;
        row.fixed_point_stage3_elapsed = accumulator.fixed_point_stage3_elapsed;
        row.fixed_point_soil_elapsed = accumulator.fixed_point_soil_elapsed;
        row.fixed_point_finalization_elapsed = accumulator.fixed_point_finalization_elapsed;
        row.publication_append_count = accumulator.publication_append_count;
        row.publication_append_elapsed = accumulator.publication_append_elapsed;
        row.publication_cow_count = accumulator.publication_cow_count;
        row.publication_full_validation_count = accumulator.publication_full_validation_count;
        row.publication_full_validation_elapsed = accumulator.publication_full_validation_elapsed;
        row.reuse_validation_count = accumulator.reuse_validation_count;
        row.reuse_validation_elapsed = accumulator.reuse_validation_elapsed;
        row.reuse_hit_count = accumulator.reuse_hit_count;
        row.reuse_fallback_count = accumulator.reuse_fallback_count;
        row.covered_child_memo_hit_count = accumulator.covered_child_memo_hit_count;
        row.covered_child_memo_fallback_count = accumulator.covered_child_memo_fallback_count;
        row.covered_child_memo_direct_hit_count = accumulator.covered_child_memo_direct_hit_count;
        row.covered_child_memo_composed_hit_count =
            accumulator.covered_child_memo_composed_hit_count;
        row.phase_rejection_count = accumulator.phase_rejection_count;
        row.event_rejection_count = accumulator.event_rejection_count;
        row.phase_and_event_rejection_count = accumulator.phase_and_event_rejection_count;
        row.other_rejection_count = accumulator.other_rejection_count;
        row.cumulative_elapsed = state.started.elapsed();
        let elapsed_stop = row.cumulative_elapsed >= state.maximum_elapsed;
        state.rows.push(row);
        state.rows.len() >= state.maximum_completed_parents || elapsed_stop
    })
}

pub(crate) fn record_adaptive_parent_rejection_v1(phase: bool, event: bool) {
    ADAPTIVE_PARENT_TELEMETRY.with(|state| {
        if let Some(state) = state.borrow_mut().as_mut() {
            state.accumulator.phase_rejection_count = state
                .accumulator
                .phase_rejection_count
                .saturating_add(u64::from(phase));
            state.accumulator.event_rejection_count = state
                .accumulator
                .event_rejection_count
                .saturating_add(u64::from(event));
            state.accumulator.phase_and_event_rejection_count = state
                .accumulator
                .phase_and_event_rejection_count
                .saturating_add(u64::from(phase && event));
            state.accumulator.other_rejection_count = state
                .accumulator
                .other_rejection_count
                .saturating_add(u64::from(!phase && !event));
        }
    });
}

pub(crate) fn record_adaptive_parent_trial_phase_v1(
    phase: &'static str,
    elapsed: std::time::Duration,
) {
    ADAPTIVE_PARENT_TELEMETRY.with(|state| {
        let mut state = state.borrow_mut();
        let Some(state) = state.as_mut() else { return };
        let (count, total) = match phase {
            "covered direct" => (
                &mut state.accumulator.covered_direct_trial_phase_count,
                &mut state.accumulator.covered_direct_trial_phase_elapsed,
            ),
            "covered composed" => (
                &mut state.accumulator.covered_composed_trial_phase_count,
                &mut state.accumulator.covered_composed_trial_phase_elapsed,
            ),
            "terminal direct" => (
                &mut state.accumulator.terminal_direct_trial_phase_count,
                &mut state.accumulator.terminal_direct_trial_phase_elapsed,
            ),
            "terminal composed" => (
                &mut state.accumulator.terminal_composed_trial_phase_count,
                &mut state.accumulator.terminal_composed_trial_phase_elapsed,
            ),
            _ => return,
        };
        *count = count.saturating_add(1);
        *total = total.saturating_add(elapsed);
    });
}

pub(crate) fn record_adaptive_parent_fixed_point_v1(completed_iterations: usize) {
    ADAPTIVE_PARENT_TELEMETRY.with(|state| {
        if let Some(state) = state.borrow_mut().as_mut() {
            state.accumulator.fixed_point_evaluation_count = state
                .accumulator
                .fixed_point_evaluation_count
                .saturating_add(1);
            state.accumulator.fixed_point_iteration_total = state
                .accumulator
                .fixed_point_iteration_total
                .saturating_add(u64::try_from(completed_iterations).unwrap_or(u64::MAX));
            state.accumulator.fixed_point_iteration_maximum = state
                .accumulator
                .fixed_point_iteration_maximum
                .max(completed_iterations);
        }
    });
}

pub(crate) fn record_adaptive_parent_fixed_point_phase_v1(
    phase: &'static str,
    started: Option<std::time::Instant>,
) {
    let Some(started) = started else { return };
    ADAPTIVE_PARENT_TELEMETRY.with(|state| {
        let mut state = state.borrow_mut();
        let Some(state) = state.as_mut() else { return };
        let destination = match phase {
            "operands" => &mut state.accumulator.fixed_point_operand_elapsed,
            "envelope" => &mut state.accumulator.fixed_point_envelope_elapsed,
            "stage3" => &mut state.accumulator.fixed_point_stage3_elapsed,
            "soil" => &mut state.accumulator.fixed_point_soil_elapsed,
            "finalization" => &mut state.accumulator.fixed_point_finalization_elapsed,
            _ => return,
        };
        *destination = destination.saturating_add(started.elapsed());
    });
}

pub(crate) fn begin_adaptive_parent_fixed_point_phase_v1() -> Option<std::time::Instant> {
    adaptive_parent_telemetry_enabled_v1().then(std::time::Instant::now)
}

pub(crate) fn record_adaptive_parent_provisional_envelope_phase_v1(
    phase: &'static str,
    started: Option<std::time::Instant>,
) {
    let Some(started) = started else { return };
    ADAPTIVE_PARENT_TELEMETRY.with(|state| {
        let mut state = state.borrow_mut();
        let Some(state) = state.as_mut() else { return };
        let destination = match phase {
            "projection" => &mut state.accumulator.provisional_envelope_projection_elapsed,
            "solver ready" => &mut state.accumulator.provisional_envelope_solver_ready_elapsed,
            "physical" => &mut state.accumulator.provisional_envelope_physical_elapsed,
            "receipts" => &mut state.accumulator.provisional_envelope_receipts_elapsed,
            "owner" => &mut state.accumulator.provisional_envelope_owner_elapsed,
            _ => return,
        };
        *destination = destination.saturating_add(started.elapsed());
    });
}

/// Records diagnostic-only nested attribution inside the physical carrier and
/// converged-candidate replay. This telemetry is active only under the explicit
/// thread-local parent guard and never enters owner state, receipts, restart,
/// publication, or controller decisions.
pub(crate) fn record_adaptive_parent_profile_detail_v1(
    phase: &'static str,
    started: Option<std::time::Instant>,
) {
    let Some(started) = started else { return };
    ADAPTIVE_PARENT_TELEMETRY.with(|state| {
        let mut state = state.borrow_mut();
        let Some(state) = state.as_mut() else { return };
        state
            .accumulator
            .profile_detail
            .record(phase, started.elapsed());
    });
}

pub(crate) fn record_adaptive_parent_publication_append_v1(
    elapsed: std::time::Duration,
    copied_on_write: bool,
) {
    ADAPTIVE_PARENT_TELEMETRY.with(|state| {
        if let Some(state) = state.borrow_mut().as_mut() {
            state.accumulator.publication_append_count =
                state.accumulator.publication_append_count.saturating_add(1);
            state.accumulator.publication_append_elapsed = state
                .accumulator
                .publication_append_elapsed
                .saturating_add(elapsed);
            state.accumulator.publication_cow_count = state
                .accumulator
                .publication_cow_count
                .saturating_add(u64::from(copied_on_write));
        }
    });
}

pub(crate) fn record_adaptive_parent_publication_validation_v1(elapsed: std::time::Duration) {
    ADAPTIVE_PARENT_TELEMETRY.with(|state| {
        if let Some(state) = state.borrow_mut().as_mut() {
            state.accumulator.publication_full_validation_count = state
                .accumulator
                .publication_full_validation_count
                .saturating_add(1);
            state.accumulator.publication_full_validation_elapsed = state
                .accumulator
                .publication_full_validation_elapsed
                .saturating_add(elapsed);
        }
    });
}

pub(crate) fn record_adaptive_parent_reuse_validation_v1(elapsed: std::time::Duration, hit: bool) {
    ADAPTIVE_PARENT_TELEMETRY.with(|state| {
        if let Some(state) = state.borrow_mut().as_mut() {
            state.accumulator.reuse_validation_count =
                state.accumulator.reuse_validation_count.saturating_add(1);
            state.accumulator.reuse_validation_elapsed = state
                .accumulator
                .reuse_validation_elapsed
                .saturating_add(elapsed);
            if hit {
                state.accumulator.reuse_hit_count =
                    state.accumulator.reuse_hit_count.saturating_add(1);
            } else {
                state.accumulator.reuse_fallback_count =
                    state.accumulator.reuse_fallback_count.saturating_add(1);
            }
        }
    });
}

pub(crate) fn record_adaptive_parent_covered_child_memo_v1(hit: bool, composed: bool) {
    ADAPTIVE_PARENT_TELEMETRY.with(|state| {
        if let Some(state) = state.borrow_mut().as_mut() {
            state.accumulator.covered_child_memo_hit_count = state
                .accumulator
                .covered_child_memo_hit_count
                .saturating_add(u64::from(hit));
            state.accumulator.covered_child_memo_fallback_count = state
                .accumulator
                .covered_child_memo_fallback_count
                .saturating_add(u64::from(!hit));
            state.accumulator.covered_child_memo_direct_hit_count = state
                .accumulator
                .covered_child_memo_direct_hit_count
                .saturating_add(u64::from(hit && !composed));
            state.accumulator.covered_child_memo_composed_hit_count = state
                .accumulator
                .covered_child_memo_composed_hit_count
                .saturating_add(u64::from(hit && composed));
        }
    });
}

pub(crate) fn adaptive_parent_telemetry_enabled_v1() -> bool {
    ADAPTIVE_PARENT_TELEMETRY.with(|state| state.borrow().is_some())
}

impl Drop for CoveredFixedPointIterationAuditGuardV1 {
    fn drop(&mut self) {
        COVERED_FIXED_POINT_ITERATION_AUDIT_ENABLED.with(|enabled| enabled.set(false));
        COVERED_FIXED_POINT_LIMITER_AUDIT.with(|audit| {
            audit.borrow_mut().take();
        });
    }
}

/// Enables a diagnostic-only iteration audit. Audit state never participates
/// in owner identity, receipts, controller decisions, or persisted bytes.
pub fn begin_covered_fixed_point_iteration_audit_v1() -> CoveredFixedPointIterationAuditGuardV1 {
    COVERED_FIXED_POINT_ITERATION_AUDIT.with(|audit| audit.borrow_mut().clear());
    COVERED_FIXED_POINT_LIMITER_AUDIT.with(|audit| {
        audit
            .borrow_mut()
            .replace(CoveredFixedPointLimiterAuditStateV1::default())
    });
    COVERED_FIXED_POINT_LIMIT_DETAIL.with(|detail| detail.set(None));
    COVERED_RECEIPT_RESEAL_MAX_ABS_RESIDUAL_BITS.with(|maximum| maximum.set(0.0_f64.to_bits()));
    COVERED_RECEIPT_RESEAL_MAX_ABS_TEMPERATURE_RESIDUAL_BITS
        .with(|maximum| maximum.set(0.0_f64.to_bits()));
    COVERED_FIXED_POINT_ITERATION_AUDIT_ENABLED.with(|enabled| enabled.set(true));
    CoveredFixedPointIterationAuditGuardV1 {
        _not_send: std::marker::PhantomData,
    }
}

/// Disables and returns the diagnostic-only iteration audit.
pub fn take_covered_fixed_point_iteration_audit_v1() -> Vec<CoveredFixedPointIterationAuditV1> {
    COVERED_FIXED_POINT_ITERATION_AUDIT_ENABLED.with(|enabled| enabled.set(false));
    COVERED_FIXED_POINT_LIMIT_DETAIL.with(|detail| detail.set(None));
    COVERED_RECEIPT_RESEAL_MAX_ABS_RESIDUAL_BITS.with(|maximum| maximum.set(0.0_f64.to_bits()));
    COVERED_RECEIPT_RESEAL_MAX_ABS_TEMPERATURE_RESIDUAL_BITS
        .with(|maximum| maximum.set(0.0_f64.to_bits()));
    COVERED_FIXED_POINT_ITERATION_AUDIT.with(|audit| std::mem::take(&mut *audit.borrow_mut()))
}

/// Returns the bounded fixed-point limiter tail accumulated by the explicit
/// iteration-audit guard. This diagnostic state is thread-local and has no
/// serialization, restart, publication, or controller path.
pub fn take_covered_fixed_point_limiter_audit_v1() -> CoveredFixedPointLimiterAuditV1 {
    let state = COVERED_FIXED_POINT_LIMITER_AUDIT
        .with(|audit| audit.borrow_mut().take())
        .unwrap_or_default();
    CoveredFixedPointLimiterAuditV1 {
        total_sample_count: state.total_sample_count,
        dropped_sample_count: state.dropped_sample_count,
        retained_tail: state.retained_tail.into_iter().collect(),
        peak_lse_normalized_delta_bits: state.peak_lse_normalized_delta_bits,
        peak_stage3_normalized_delta_bits: state.peak_stage3_normalized_delta_bits,
        peak_soil_enthalpy_normalized_delta_bits: state.peak_soil_enthalpy_normalized_delta_bits,
        peak_soil_temperature_normalized_delta_bits: state
            .peak_soil_temperature_normalized_delta_bits,
        peak_boundary_normalized_delta_bits: state.peak_boundary_normalized_delta_bits,
    }
}

pub(crate) fn covered_fixed_point_limiter_audit_enabled_v1() -> bool {
    COVERED_FIXED_POINT_LIMITER_AUDIT.with(|audit| audit.borrow().is_some())
}

pub(crate) fn record_covered_fixed_point_limiter_sample_v1(
    sample: CoveredFixedPointLimiterSampleV1,
) {
    fn update_peak(peak_bits: &mut u64, candidate_bits: u64) {
        let candidate = f64::from_bits(candidate_bits);
        if !candidate.is_nan() && candidate > f64::from_bits(*peak_bits) {
            *peak_bits = candidate_bits;
        }
    }

    COVERED_FIXED_POINT_LIMITER_AUDIT.with(|audit| {
        let mut audit = audit.borrow_mut();
        let Some(state) = audit.as_mut() else { return };
        state.total_sample_count = state.total_sample_count.saturating_add(1);
        update_peak(
            &mut state.peak_lse_normalized_delta_bits,
            sample.lse_max_normalized_delta_bits,
        );
        update_peak(
            &mut state.peak_stage3_normalized_delta_bits,
            sample.stage3_max_normalized_delta_bits,
        );
        update_peak(
            &mut state.peak_soil_enthalpy_normalized_delta_bits,
            sample.soil_enthalpy_max_normalized_delta_bits,
        );
        update_peak(
            &mut state.peak_soil_temperature_normalized_delta_bits,
            sample.soil_temperature_max_normalized_delta_bits,
        );
        update_peak(
            &mut state.peak_boundary_normalized_delta_bits,
            sample.boundary_max_normalized_delta_bits,
        );
        if state.retained_tail.len() == COVERED_FIXED_POINT_LIMITER_AUDIT_CAPACITY {
            state.retained_tail.pop_front();
            state.dropped_sample_count = state.dropped_sample_count.saturating_add(1);
        }
        state.retained_tail.push_back(sample);
    });
}

pub(crate) fn record_covered_receipt_reseal_roundoff_v1(
    residual_j_m2: f64,
    temperature_residual_k: f64,
) {
    if !residual_j_m2.is_finite()
        || residual_j_m2 < 0.0
        || !temperature_residual_k.is_finite()
        || temperature_residual_k < 0.0
    {
        return;
    }
    COVERED_FIXED_POINT_ITERATION_AUDIT_ENABLED.with(|enabled| {
        if enabled.get() {
            COVERED_RECEIPT_RESEAL_MAX_ABS_RESIDUAL_BITS.with(|maximum| {
                let previous = f64::from_bits(maximum.get());
                if residual_j_m2 > previous {
                    maximum.set(residual_j_m2.to_bits());
                }
            });
            COVERED_RECEIPT_RESEAL_MAX_ABS_TEMPERATURE_RESIDUAL_BITS.with(|maximum| {
                let previous = f64::from_bits(maximum.get());
                if temperature_residual_k > previous {
                    maximum.set(temperature_residual_k.to_bits());
                }
            });
        }
    });
}

pub(crate) fn record_covered_fixed_point_limit_detail_v1(detail: CoveredFixedPointLimitDetailV1) {
    COVERED_FIXED_POINT_ITERATION_AUDIT_ENABLED.with(|enabled| {
        if enabled.get() {
            COVERED_FIXED_POINT_LIMIT_DETAIL.with(|pending| pending.set(Some(detail)));
        }
    });
}

pub(crate) fn record_covered_fixed_point_iteration_audit_v1(
    support: TimeSupport,
    completed_iterations: usize,
    converged: bool,
) {
    record_adaptive_parent_fixed_point_v1(completed_iterations);
    let limit_detail = COVERED_FIXED_POINT_LIMIT_DETAIL.with(|detail| detail.take());
    let receipt_reseal_max_abs_residual_bits = COVERED_RECEIPT_RESEAL_MAX_ABS_RESIDUAL_BITS
        .with(|maximum| maximum.replace(0.0_f64.to_bits()));
    let receipt_reseal_max_abs_temperature_residual_bits =
        COVERED_RECEIPT_RESEAL_MAX_ABS_TEMPERATURE_RESIDUAL_BITS
            .with(|maximum| maximum.replace(0.0_f64.to_bits()));
    COVERED_FIXED_POINT_ITERATION_AUDIT_ENABLED.with(|enabled| {
        if enabled.get() {
            COVERED_FIXED_POINT_ITERATION_AUDIT.with(|audit| {
                audit.borrow_mut().push(CoveredFixedPointIterationAuditV1 {
                    support,
                    completed_iterations,
                    converged,
                    limit_detail,
                    receipt_reseal_max_abs_residual_bits,
                    receipt_reseal_max_abs_temperature_residual_bits,
                });
            });
        }
    });
}

fn digest32_lower_hex(value: Digest32) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(64);
    for byte in value.as_bytes() {
        output.push(char::from(HEX[usize::from(byte >> 4)]));
        output.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    output
}

include!("snow_stage3_v11_attachment_error.rs");

/// Static configuration and topology identity.  There is intentionally no
/// event day, lane, elapsed time, live surface receipt, or ending owner here.
#[derive(Clone, Debug, PartialEq)]
pub struct DirectSnowStage3V11StaticContext {
    pub run_identity: Digest32,
    pub topology_identity: Digest32,
    pub parent_duration_ns: u128,
    pub minimum_support_ns: u128,
    pub calendar_receipt: Digest32,
    pub controller_policy: Digest32,
    pub parent_sequence: u128,
    pub lane_ids: Vec<u32>,
    pub vegetation_configuration: VegetationConfigurationV11,
    pub surface_liquid_configuration: DirectSurfaceLiquidConfiguration,
    pub wb14_parameters: Vec<crate::DirectOfeWb14Parameters>,
}

/// Static production authority needed to install Stage 3 on a live direct
/// run. The initializer derives the V11 beginning state, complete owner
/// envelopes, lane set, coupled clock, and parent transaction internally;
/// callers cannot supply or splice those owner graphs.
#[derive(Clone, Debug, PartialEq)]
pub struct DirectSnowStage3V11ProductionConfigurationV1 {
    pub run_identity: Digest32,
    pub topology_identity: Digest32,
    pub calendar_receipt: Digest32,
    pub controller_policy: Digest32,
    pub surface_liquid_configuration: DirectSurfaceLiquidConfiguration,
    pub wb14_parameters: Vec<crate::DirectOfeWb14Parameters>,
}

impl DirectSnowStage3V11StaticContext {
    pub fn validate(&self) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if self.parent_duration_ns != STAGE3_V11_PARENT_SUPPORT_NS
            || self.minimum_support_ns == 0
            || self.minimum_support_ns > self.parent_duration_ns
            || self.lane_ids.is_empty()
            || self.lane_ids.windows(2).any(|pair| pair[0] >= pair[1])
            || self.surface_liquid_configuration.records.is_empty()
            || self.wb14_parameters.is_empty()
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "static parent, lane, receiver, or WB14 configuration",
            ));
        }
        self.vegetation_configuration
            .validate()
            .map_err(|_| DirectSnowStage3V11AttachmentError::Identity("V11 configuration"))?;
        validate_receiver_topology(&self.surface_liquid_configuration.records)
    }
}

/// One sealed 1,800-second support for every Stage-3 lane.  The snow inputs
/// are the actual Stage-3 owner operands for this support; no daily result is
/// accepted as a substitute.
#[derive(Clone, Debug, PartialEq)]
pub struct DirectSnowStage3V11PreparedSupport {
    support: TimeSupport,
    snow_inputs_by_lane: BTreeMap<u32, DirectActiveSnowPartitionInputs>,
    support_forcing_by_lane: BTreeMap<u32, DirectSnowStage3SupportInput>,
    /// Sealed lower-boundary/atmospheric input for the actual V11 owner.
    /// It contains no event request, carrier operand, or ending owner.
    v11_interval: DirectV9ShadowIntervalInput,
    /// Sealed Child-2C carrier inputs, keyed by production lane. A missing
    /// entry means this support is snow-free and must remain on the existing
    /// snow-free adopter.
    /// Complete active snow-surface forcing topology, keyed by physical
    /// destination. Empty means the support is snow-free.
    snow_surface_forcing_by_destination:
        BTreeMap<(OfeId, TileId), SealedStage3TileBoundaryForcingV1>,
    open_snow_destination_requests: BTreeSet<(OfeId, TileId)>,
    #[cfg(feature = "restart-authority-evidence")]
    restart_fixture_open_snow_shortwave_multiplier: Option<f64>,
    atmospheric_receipt_by_destination: BTreeMap<(OfeId, TileId), Stage3ParentAtmosphericReceiptV1>,
    /// Covered V11 projection. It is a separate type from the snow-free
    /// interval so regime selection is explicit at the sealed-support seam.
    covered_v11_interval: Option<DirectV11SnowCoveredSegmentInput>,
    /// Provider-owned destination and receipt identity. The physical
    /// precipitation parcel remains sealed input; it is not a terminal parcel
    /// and cannot contain an ending owner or event time.
    support_identity_by_lane: BTreeMap<u32, Vec<PreparedStage3V11SupportIdentityV1>>,
    hard_boundaries: Vec<ModelTimeNs>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Stage3SnowSurfaceRegime {
    SnowFree,
    OpenSnowOnly,
    CanopyCoveredOrMixed,
}

/// Static destination capability for a production support. This describes
/// canopy/open topology only; it is not the snow regime for the support.
#[derive(Clone, Debug, PartialEq)]
pub enum DirectSnowStage3V11DestinationCapabilityV1 {
    OpenProviderProjection,
    CanopyCovered(SealedCoveredCarrierForcing),
}

/// Physical inputs for one JIT production support. The attachment owns
/// construction of the sealed prepared-support graph and receives this same
/// dual-regime shape whether the sequential beginning state is snow-free,
/// covered, terminal, or about to reappear.
#[derive(Clone, Debug, PartialEq)]
pub struct DirectSnowStage3V11DualRegimeSupportInputsV1 {
    pub snow_inputs_by_lane: BTreeMap<u32, DirectActiveSnowPartitionInputs>,
    pub support_forcing_by_lane: BTreeMap<u32, DirectSnowStage3SupportInput>,
    pub snow_free_v11_interval: DirectV9ShadowIntervalInput,
    pub snow_surface_v11_interval: DirectV11SnowCoveredSegmentInput,
    pub support_identity_by_lane: BTreeMap<u32, Vec<PreparedStage3V11SupportIdentityV1>>,
    pub destination_capabilities:
        BTreeMap<(OfeId, TileId), DirectSnowStage3V11DestinationCapabilityV1>,
    pub hard_boundaries: Vec<ModelTimeNs>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Stage3LaneLifecycleV1 {
    SnowFree,
    ResolvedSnow,
    TerminalPending,
    SolidPrecipitationPending,
}

include!("snow_stage3_v11_precipitation.rs");
include!("snow_stage3_v11_snow_soil_heat.rs");
include!("snow_stage3_v11_terminal_chronology.rs");
include!("snow_stage3_v11_terminal_receiver_custody.rs");
include!("stage3_parent_atmosphere.rs");

pub(crate) fn stage3_lane_lifecycle(
    state: &DirectSnowStage3PersistentState,
    snowfall_m: f64,
) -> Stage3LaneLifecycleV1 {
    if stage3_is_resolved_thermal_domain(state) {
        return Stage3LaneLifecycleV1::ResolvedSnow;
    }
    let has_terminal_liquid = state.detached_retained_liquid_kg_m2 > 0.0
        || state
            .layers
            .iter()
            .any(|layer| layer.liquid_water_m > 0.0 || layer.refrozen_liquid_m > 0.0);
    if stage3_has_represented_ice(state) || has_terminal_liquid {
        return Stage3LaneLifecycleV1::TerminalPending;
    }
    if snowfall_m > 0.0 {
        return Stage3LaneLifecycleV1::SolidPrecipitationPending;
    }
    Stage3LaneLifecycleV1::SnowFree
}

fn terminal_domain_can_cross_parent_support(
    state: &DirectSnowStage3PersistentState,
    has_pending_terminal_parcels: bool,
) -> bool {
    stage3_is_terminal_event_domain(state) && !has_pending_terminal_parcels
}

fn debit_solid_reappearance_phase_v1(
    forcing: &mut DirectSnowHourlyForcing,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    let active_precipitation_m = forcing.active_precipitation_m;
    let rain_m = forcing.rain_m;
    let snowfall_m = forcing.snowfall_m;
    let rain_fraction = forcing.rain_fraction;
    let snow_fraction = forcing.snow_fraction;
    let solid_water_equivalent_m = snowfall_m * 0.1;
    let precipitation_scale = active_precipitation_m.abs().max(1.0);
    if [
        active_precipitation_m,
        rain_m,
        snowfall_m,
        rain_fraction,
        snow_fraction,
        solid_water_equivalent_m,
    ]
    .iter()
    .any(|value| !value.is_finite())
        || active_precipitation_m <= 0.0
        || rain_m < 0.0
        || solid_water_equivalent_m <= 0.0
        || !(0.0..=1.0).contains(&rain_fraction)
        || !(0.0..=1.0).contains(&snow_fraction)
        || (active_precipitation_m - solid_water_equivalent_m - rain_m).abs()
            > 1.0e-12 * precipitation_scale
        || (rain_fraction + snow_fraction - 1.0).abs() > 1.0e-12
        || (rain_m - active_precipitation_m * rain_fraction).abs() > 1.0e-12 * precipitation_scale
        || (solid_water_equivalent_m - active_precipitation_m * snow_fraction).abs()
            > 1.0e-12 * precipitation_scale
        || forcing
            .hydrometeor_temperature_c
            .is_some_and(|value| !value.is_finite())
    {
        return Err(DirectSnowStage3V11AttachmentError::Precipitation(
            "solid reappearance phase debit",
        ));
    }

    // The solid parcel has already entered the newly accepted snow owner at
    // the zero-duration reappearance event.  The following positive support
    // receives only the still-unconsumed liquid forcing.  Reconstruct every
    // phase field together so the successor cannot retain a stale solid
    // fraction after the solid mass itself was debited.
    forcing.active_precipitation_m = rain_m;
    forcing.snowfall_m = 0.0;
    forcing.snow_fraction = 0.0;
    if rain_m > 0.0 {
        forcing.rain_fraction = 1.0;
    } else {
        forcing.rain_fraction = 0.0;
        forcing.hydrometeor_temperature_c = None;
    }
    Ok(())
}

impl DirectSnowStage3V11PreparedSupport {
    fn bind_live_owner_transaction(
        &mut self,
        transaction_id: openwepp_kernel_contract::TransactionId,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        self.v11_interval.lse_forcing.transaction_id = transaction_id;
        self.v11_interval.lse_forcing.forcing_sha256 = self
            .v11_interval
            .lse_forcing
            .canonical_sha256()
            .map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity(
                    "live snow-free forcing transaction digest",
                )
            })?;
        if let Some(covered) = &mut self.covered_v11_interval {
            covered.lse_forcing.transaction_id = transaction_id;
            covered.lse_forcing.forcing_sha256 =
                covered.lse_forcing.canonical_sha256().map_err(|_| {
                    DirectSnowStage3V11AttachmentError::Identity(
                        "live covered forcing transaction digest",
                    )
                })?;
        }
        Ok(())
    }

    /// Build the sealed production support graph from a state-independent
    /// dual-regime capability. The absolute support is scheduler-owned;
    /// callers cannot substitute chronology embedded in the input payload.
    pub fn from_dual_regime_production_inputs(
        support: TimeSupport,
        inputs: DirectSnowStage3V11DualRegimeSupportInputsV1,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        let mut prepared = Self::try_new(
            support,
            inputs.snow_inputs_by_lane,
            inputs.support_forcing_by_lane,
            inputs.snow_free_v11_interval,
            inputs.support_identity_by_lane,
        )?
        .with_covered_v11_interval(inputs.snow_surface_v11_interval);
        for (destination, capability) in inputs.destination_capabilities {
            prepared = match capability {
                DirectSnowStage3V11DestinationCapabilityV1::OpenProviderProjection => {
                    prepared.with_provider_open_snow_destination(destination)?
                }
                DirectSnowStage3V11DestinationCapabilityV1::CanopyCovered(forcing) => {
                    prepared.with_covered_tile_forcing(destination, forcing)
                }
            };
        }
        if !inputs.hard_boundaries.is_empty() {
            prepared = prepared.with_hard_boundaries(inputs.hard_boundaries)?;
        }
        Ok(prepared)
    }

    /// Construct an unsealed support draft. Provider/GSI identity is admitted
    /// only when `PreparedStage3V11DayV1::bind_provider_day` consumes this
    /// draft and returns the opaque validated day capability.
    pub fn try_new(
        support: TimeSupport,
        snow_inputs_by_lane: BTreeMap<u32, DirectActiveSnowPartitionInputs>,
        support_forcing_by_lane: BTreeMap<u32, DirectSnowStage3SupportInput>,
        v11_interval: DirectV9ShadowIntervalInput,
        support_identity_by_lane: BTreeMap<u32, Vec<PreparedStage3V11SupportIdentityV1>>,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        validate_parent_support_duration(support.duration_ns())?;
        Self::try_new_after_parent_support_validation(
            support,
            snow_inputs_by_lane,
            support_forcing_by_lane,
            v11_interval,
            support_identity_by_lane,
        )
    }

    #[cfg(test)]
    pub(crate) fn try_new_for_short_production_test(
        support: TimeSupport,
        snow_inputs_by_lane: BTreeMap<u32, DirectActiveSnowPartitionInputs>,
        support_forcing_by_lane: BTreeMap<u32, DirectSnowStage3SupportInput>,
        v11_interval: DirectV9ShadowIntervalInput,
        support_identity_by_lane: BTreeMap<u32, Vec<PreparedStage3V11SupportIdentityV1>>,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        if support.duration_ns() == 0
            || support.duration_ns() % STAGE3_V11_ADAPTIVE_MINIMUM_SUPPORT_NS != 0
        {
            return Err(DirectSnowStage3V11AttachmentError::Support(
                "short production test support is outside the 60-second grid",
            ));
        }
        Self::try_new_after_parent_support_validation(
            support,
            snow_inputs_by_lane,
            support_forcing_by_lane,
            v11_interval,
            support_identity_by_lane,
        )
    }

    fn try_new_after_parent_support_validation(
        support: TimeSupport,
        snow_inputs_by_lane: BTreeMap<u32, DirectActiveSnowPartitionInputs>,
        support_forcing_by_lane: BTreeMap<u32, DirectSnowStage3SupportInput>,
        v11_interval: DirectV9ShadowIntervalInput,
        support_identity_by_lane: BTreeMap<u32, Vec<PreparedStage3V11SupportIdentityV1>>,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        let lane_ids = snow_inputs_by_lane.keys().copied().collect::<BTreeSet<_>>();
        if lane_ids.is_empty()
            || lane_ids != support_forcing_by_lane.keys().copied().collect()
            || lane_ids != support_identity_by_lane.keys().copied().collect()
            || support_identity_by_lane.values().any(Vec::is_empty)
            || support_identity_by_lane.values().any(|identities| {
                identities.windows(2).any(|pair| {
                    (
                        pair[0].destination_ofe_id.as_str(),
                        pair[0].destination_tile_id.as_str(),
                    ) >= (
                        pair[1].destination_ofe_id.as_str(),
                        pair[1].destination_tile_id.as_str(),
                    )
                })
            })
        {
            return Err(DirectSnowStage3V11AttachmentError::Support(
                "support draft lane and destination identity",
            ));
        }
        Ok(Self {
            support,
            snow_inputs_by_lane,
            support_forcing_by_lane,
            v11_interval,
            snow_surface_forcing_by_destination: BTreeMap::new(),
            open_snow_destination_requests: BTreeSet::new(),
            #[cfg(feature = "restart-authority-evidence")]
            restart_fixture_open_snow_shortwave_multiplier: None,
            atmospheric_receipt_by_destination: BTreeMap::new(),
            covered_v11_interval: None,
            support_identity_by_lane,
            hard_boundaries: Vec::new(),
        })
    }

    /// Add exact coupled-time event/restart/output boundaries that may
    /// truncate a Stage-3 cadence proposal without creating a zero-duration
    /// physics child.
    pub fn with_hard_boundaries(
        mut self,
        mut boundaries: Vec<ModelTimeNs>,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        boundaries.sort_unstable();
        boundaries.dedup();
        if boundaries.iter().any(|boundary| {
            *boundary <= self.support.start_ns() || *boundary >= self.support.end_ns()
        }) {
            return Err(DirectSnowStage3V11AttachmentError::Support(
                "coupled hard boundary outside parent interior",
            ));
        }
        self.hard_boundaries = boundaries;
        Ok(self)
    }

    /// Attach a covered forcing to one typed physical destination.
    #[must_use]
    pub fn with_covered_tile_forcing(
        mut self,
        destination: (OfeId, TileId),
        forcing: SealedCoveredCarrierForcing,
    ) -> Self {
        self.snow_surface_forcing_by_destination.insert(
            destination,
            SealedStage3TileBoundaryForcingV1::V11CanopyCovered(forcing),
        );
        self
    }

    #[cfg(test)]
    #[must_use]
    pub(crate) fn with_sealed_open_tile_forcing(
        mut self,
        destination: (OfeId, TileId),
        forcing: SealedOpenSnowTileForcingV1,
    ) -> Self {
        self.snow_surface_forcing_by_destination.insert(
            destination,
            SealedStage3TileBoundaryForcingV1::OpenSnow(forcing),
        );
        self
    }

    /// Derive and seal one open-snow destination from the prepared provider
    /// projection. Callers identify the retained raw-wind provider and the
    /// admitted identity projection; all meteorological scalars come from the
    /// interval that is subsequently joined to the provider day.
    pub fn with_provider_open_snow_destination(
        mut self,
        destination: (OfeId, TileId),
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        self.support_identity_by_lane
            .values()
            .flatten()
            .find(|identity| {
                identity.destination_ofe_id == destination.0.as_str()
                    && identity.destination_tile_id == destination.1.as_str()
            })
            .ok_or(DirectSnowStage3V11AttachmentError::Support(
                "open-snow destination provider identity",
            ))?;
        self.open_snow_destination_requests.insert(destination);
        Ok(self)
    }

    /// Fixture-only perturbation of the sealed open-snow shortwave bands.
    /// Provider identity, atmosphere, exposure, and all covered forcing remain
    /// unchanged; the multiplier is applied exactly once while the open-snow
    /// forcing receipt is sealed during provider-day admission.
    #[cfg(feature = "restart-authority-evidence")]
    pub fn restart_authority_with_open_snow_shortwave_multiplier_for_fixture(
        mut self,
        multiplier: f64,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        if !multiplier.is_finite() || multiplier <= 0.0 {
            return Err(DirectSnowStage3V11AttachmentError::Support(
                "restart fixture open-snow shortwave multiplier",
            ));
        }
        self.restart_fixture_open_snow_shortwave_multiplier = Some(multiplier);
        Ok(self)
    }

    fn bind_provider_atmosphere(
        &mut self,
        provider_destinations: &BTreeMap<(String, String), &SnowFreeHalfHourIntervalReceipt>,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        self.atmospheric_receipt_by_destination.clear();
        for (destination, provider) in provider_destinations {
            let typed_destination = (
                OfeId::try_new(destination.0.clone()).map_err(|_| {
                    DirectSnowStage3V11AttachmentError::Identity("provider atmosphere OFE")
                })?,
                TileId::try_new(destination.1.clone()).map_err(|_| {
                    DirectSnowStage3V11AttachmentError::Identity("provider atmosphere tile")
                })?,
            );
            let atmosphere =
                Stage3ParentAtmosphericReceiptV1::from_provider(self.support, provider)?;
            self.validate_atmospheric_projections(provider, &atmosphere)?;
            self.atmospheric_receipt_by_destination
                .insert(typed_destination, atmosphere);
        }
        let requests = self.open_snow_destination_requests.clone();
        for destination in requests {
            let atmosphere = self
                .atmospheric_receipt_by_destination
                .get(&destination)
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "open-snow provider atmosphere destination",
                ))?;
            let provider = provider_destinations
                .get(&(
                    destination.0.as_str().to_owned(),
                    destination.1.as_str().to_owned(),
                ))
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "open-snow provider destination",
                ))?;
            let source_wind_provider_sha256 =
                parse_lower_hex_digest(&provider.provider_definition_sha256)?;
            let projection_model_definition_sha256 =
                digest_bytes(b"OPENWEPP_STAGE3_RAW_WIND_IDENTITY_PROJECTION_V1");
            let exposure = SealedOpenSnowExposureReceiptV1::try_new(
                self.support,
                destination.clone(),
                atmosphere.provider_interval_receipt_sha256,
                source_wind_provider_sha256,
                atmosphere.raw_wind_m_s,
                projection_model_definition_sha256,
            )?;
            #[cfg(feature = "restart-authority-evidence")]
            let shortwave_multiplier = self
                .restart_fixture_open_snow_shortwave_multiplier
                .unwrap_or(1.0);
            #[cfg(not(feature = "restart-authority-evidence"))]
            let shortwave_multiplier = 1.0;
            let open = SealedOpenSnowTileForcingV1::try_new(SealedOpenSnowTileForcingInputsV1 {
                support: self.support,
                destination: destination.clone(),
                forcing_receipt_sha256: atmosphere.provider_interval_receipt_sha256,
                exposure: exposure.clone(),
                reference_temperature_k: atmosphere.air_temperature_k,
                reference_specific_humidity_kg_kg: atmosphere.specific_humidity_kg_kg,
                air_pressure_pa: atmosphere.air_pressure_pa,
                atmospheric_downward_longwave_w_m2: atmosphere.downward_longwave_w_m2,
                direct_vis_w_m2: atmosphere.direct_vis_w_m2 * shortwave_multiplier,
                diffuse_vis_w_m2: atmosphere.diffuse_vis_w_m2 * shortwave_multiplier,
                direct_nir_w_m2: atmosphere.direct_nir_w_m2 * shortwave_multiplier,
                diffuse_nir_w_m2: atmosphere.diffuse_nir_w_m2 * shortwave_multiplier,
                rain_m: provider.rain_m,
                snowfall_m: provider.snowfall_m,
                precipitation_parcel_count: provider.precipitation_parcels.len()
                    + provider.solid_precipitation_parcels.len(),
            })?;
            let identity = self
                .support_identity_by_lane
                .values_mut()
                .flatten()
                .find(|identity| {
                    identity.destination_ofe_id == destination.0.as_str()
                        && identity.destination_tile_id == destination.1.as_str()
                })
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "open-snow provider identity update",
                ))?;
            identity.exposure_identity = exposure.receipt_sha256;
            self.snow_surface_forcing_by_destination.insert(
                destination,
                SealedStage3TileBoundaryForcingV1::OpenSnow(open),
            );
        }
        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    fn validate_atmospheric_projections(
        &self,
        provider: &SnowFreeHalfHourIntervalReceipt,
        atmosphere: &Stage3ParentAtmosphericReceiptV1,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        fn same(lhs: f64, rhs: f64) -> bool {
            lhs.to_bits() == rhs.to_bits()
        }
        let validate_lse = |forcing: &openwepp_land_surface_energy::LandSurfaceForcing| {
            same(forcing.air_temperature_k, atmosphere.air_temperature_k)
                && same(
                    forcing.air_specific_humidity_kg_kg,
                    atmosphere.specific_humidity_kg_kg,
                )
                && same(forcing.air_pressure_pa, atmosphere.air_pressure_pa)
                && same(forcing.reference_wind_m_s, atmosphere.raw_wind_m_s)
                && same(forcing.direct_vis_w_m2, atmosphere.direct_vis_w_m2)
                && same(forcing.diffuse_vis_w_m2, atmosphere.diffuse_vis_w_m2)
                && same(forcing.direct_nir_w_m2, atmosphere.direct_nir_w_m2)
                && same(forcing.diffuse_nir_w_m2, atmosphere.diffuse_nir_w_m2)
                && same(
                    forcing.atmospheric_downward_longwave_w_m2,
                    atmosphere.downward_longwave_w_m2,
                )
        };
        if !validate_lse(&self.v11_interval.lse_forcing)
            || self
                .covered_v11_interval
                .as_ref()
                .is_some_and(|covered| !validate_lse(&covered.lse_forcing))
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "base/covered V11 provider atmosphere projection",
            ));
        }
        let base_vegetation = &self.v11_interval.vegetation_forcing;
        if !same(
            base_vegetation.air_temperature_k,
            atmosphere.air_temperature_k,
        ) || !same(base_vegetation.pressure_pa, atmosphere.air_pressure_pa)
            || !same(base_vegetation.wind_m_s, atmosphere.raw_wind_m_s)
            || !same(
                base_vegetation.specific_humidity,
                atmosphere.specific_humidity_kg_kg,
            )
            || !same(base_vegetation.direct_par_w_m2, atmosphere.direct_vis_w_m2)
            || !same(
                base_vegetation.diffuse_par_w_m2,
                atmosphere.diffuse_vis_w_m2,
            )
            || !same(base_vegetation.direct_nir_w_m2, atmosphere.direct_nir_w_m2)
            || !same(
                base_vegetation.diffuse_nir_w_m2,
                atmosphere.diffuse_nir_w_m2,
            )
            || !same(
                base_vegetation.longwave_down_w_m2,
                atmosphere.downward_longwave_w_m2,
            )
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "base V11 vegetation provider atmosphere projection",
            ));
        }
        if let Some(covered) = &self.covered_v11_interval {
            let vegetation = &covered.vegetation_forcing;
            if !same(vegetation.air_temperature_k, atmosphere.air_temperature_k)
                || !same(vegetation.pressure_pa, atmosphere.air_pressure_pa)
                || !same(vegetation.wind_m_s, atmosphere.raw_wind_m_s)
                || !same(
                    vegetation.specific_humidity,
                    atmosphere.specific_humidity_kg_kg,
                )
                || !same(vegetation.direct_par_w_m2, atmosphere.direct_vis_w_m2)
                || !same(vegetation.diffuse_par_w_m2, atmosphere.diffuse_vis_w_m2)
                || !same(vegetation.direct_nir_w_m2, atmosphere.direct_nir_w_m2)
                || !same(vegetation.diffuse_nir_w_m2, atmosphere.diffuse_nir_w_m2)
                || !same(
                    vegetation.longwave_down_w_m2,
                    atmosphere.downward_longwave_w_m2,
                )
            {
                return Err(DirectSnowStage3V11AttachmentError::Identity(
                    "covered V11 vegetation provider atmosphere projection",
                ));
            }
        }
        let dewpoint = TemperatureCelsius::try_new(provider.dew_point_c).map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity("provider dewpoint domain")
        })?;
        let dewpoint_vapor_pa = kilopascals_to_pascals(
            saturation_vapor_pressure_water_kpa(dewpoint)
                .map_err(|_| {
                    DirectSnowStage3V11AttachmentError::Identity(
                        "provider dewpoint vapor projection",
                    )
                })?
                .as_kilopascals(),
        );
        if !same(dewpoint_vapor_pa, atmosphere.actual_vapor_pressure_pa) {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "specific-humidity/dewpoint provider contradiction",
            ));
        }
        for (lane_id, inputs) in &self.snow_inputs_by_lane {
            let support_forcing = self.support_forcing_by_lane.get(lane_id).ok_or(
                DirectSnowStage3V11AttachmentError::Identity("Stage-3 atmosphere lane"),
            )?;
            if !same(inputs.wind_m_s, atmosphere.raw_wind_m_s)
                || !same(inputs.dewpoint_c, provider.dew_point_c)
                || !same(
                    inputs.surface_energy_options.atmospheric_pressure_pa,
                    atmosphere.air_pressure_pa,
                )
                || !same(
                    support_forcing.forcing.air_temperature_c,
                    provider.air_temperature_c,
                )
                || !same(
                    support_forcing.forcing.active_precipitation_m,
                    provider.active_precipitation_m,
                )
                || !same(support_forcing.forcing.rain_m, provider.rain_m)
                || !same(support_forcing.forcing.snowfall_m, provider.snowfall_m)
                || !same(
                    support_forcing.forcing.rain_fraction,
                    provider.rain_fraction,
                )
                || !same(
                    support_forcing.forcing.snow_fraction,
                    provider.snow_fraction,
                )
                || support_forcing
                    .forcing
                    .hydrometeor_temperature_c
                    .map(f64::to_bits)
                    != provider.hydrometeor_temperature_c.map(f64::to_bits)
            {
                return Err(DirectSnowStage3V11AttachmentError::Identity(
                    "Stage-3/open-forcing atmosphere projection",
                ));
            }
        }
        for (destination, forcing) in &self.snow_surface_forcing_by_destination {
            let matches_destination = destination.0.as_str() == provider.ofe_id
                && destination.1.as_str() == provider.tile_id;
            if !matches_destination {
                continue;
            }
            match forcing {
                SealedStage3TileBoundaryForcingV1::V11CanopyCovered(covered) => {
                    if !same(
                        covered.reference_temperature_k,
                        atmosphere.air_temperature_k,
                    ) || !same(
                        covered.reference_specific_humidity,
                        atmosphere.specific_humidity_kg_kg,
                    ) || !same(
                        covered.atmospheric_longwave_w_m2,
                        atmosphere.downward_longwave_w_m2,
                    ) || !same(covered.exposure.wind_m_s, atmosphere.raw_wind_m_s)
                        || covered.exposure.provider_digest != provider.provider_definition_sha256
                    {
                        return Err(DirectSnowStage3V11AttachmentError::Identity(
                            "covered carrier provider atmosphere projection",
                        ));
                    }
                }
                SealedStage3TileBoundaryForcingV1::OpenSnow(_) => {
                    return Err(DirectSnowStage3V11AttachmentError::Identity(
                        "open-snow forcing must be sealed during provider binding",
                    ));
                }
            }
        }
        Ok(())
    }

    /// Attach the distinct covered V11 atmospheric projection to this support.
    #[must_use]
    pub fn with_covered_v11_interval(mut self, interval: DirectV11SnowCoveredSegmentInput) -> Self {
        self.covered_v11_interval = Some(interval);
        self
    }

    #[must_use]
    pub const fn support(&self) -> TimeSupport {
        self.support
    }

    #[must_use]
    pub fn snow_surface_forcing_by_destination(
        &self,
    ) -> &BTreeMap<(OfeId, TileId), SealedStage3TileBoundaryForcingV1> {
        &self.snow_surface_forcing_by_destination
    }

    #[must_use]
    pub fn atmospheric_receipt_by_destination(
        &self,
    ) -> &BTreeMap<(OfeId, TileId), Stage3ParentAtmosphericReceiptV1> {
        &self.atmospheric_receipt_by_destination
    }

    fn coupled_subslab(
        &self,
        support: TimeSupport,
        child_ordinal: u32,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        if support.start_ns() < self.support.start_ns() || support.end_ns() > self.support.end_ns()
        {
            return Err(DirectSnowStage3V11AttachmentError::Support(
                "coupled subslab outside prepared parent support",
            ));
        }
        let duration_seconds = f64::from_bits(support.duration_s_bits());
        let parent_duration_seconds = f64::from_bits(self.support.duration_s_bits());
        let support_fraction = duration_seconds / parent_duration_seconds;
        let child_offset_ns = support.start_ns().get() - self.support.start_ns().get();
        let child_end_offset_ns = support.end_ns().get() - self.support.start_ns().get();
        let seconds_to_parent_ns = |seconds: f64| {
            let nanos = seconds * 1_000_000_000.0;
            let rounded = nanos.round();
            if !nanos.is_finite()
                || nanos < 0.0
                || (nanos - rounded).abs() > 0.500_001
                || rounded > u128::MAX as f64
            {
                return Err(DirectSnowStage3V11AttachmentError::Support(
                    "liquid parcel timestamp is outside exact nanosecond chronology",
                ));
            }
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            Ok(rounded as u128)
        };
        let partition_parcels = |parcels: &[openwepp_land_surface_energy::LiquidParcel]| {
            parcels
                .iter()
                .filter_map(|parcel| {
                    let result = (|| {
                        let parcel_start_ns = seconds_to_parent_ns(parcel.start_s)?;
                        let parcel_end_ns = seconds_to_parent_ns(parcel.end_s)?;
                        let overlap_start_ns = parcel_start_ns.max(child_offset_ns);
                        let overlap_end_ns = parcel_end_ns.min(child_end_offset_ns);
                        if overlap_end_ns <= overlap_start_ns {
                            return Ok(None);
                        }
                        let parcel_duration_ns = parcel_end_ns.checked_sub(parcel_start_ns).ok_or(
                            DirectSnowStage3V11AttachmentError::Support(
                                "liquid parcel timestamp order",
                            ),
                        )?;
                        if parcel_duration_ns == 0 {
                            return Err(DirectSnowStage3V11AttachmentError::Support(
                                "zero-duration liquid parcel",
                            ));
                        }
                        let mut child = parcel.clone();
                        let overlap_duration_ns = overlap_end_ns - overlap_start_ns;
                        child.start_s =
                            (overlap_start_ns - child_offset_ns) as f64 / 1_000_000_000.0;
                        child.end_s = (overlap_end_ns - child_offset_ns) as f64 / 1_000_000_000.0;
                        child.amount_kg_m2_destination_tile_ground *=
                            overlap_duration_ns as f64 / parcel_duration_ns as f64;
                        Ok(Some(child))
                    })();
                    match result {
                        Ok(Some(child)) => Some(Ok(child)),
                        Ok(None) => None,
                        Err(error) => Some(Err(error)),
                    }
                })
                .collect::<Result<Vec<_>, DirectSnowStage3V11AttachmentError>>()
        };
        let partition_vegetation_rain =
            |forcing: &openwepp_land_surface_energy::LandSurfaceForcing,
             parent_rain: f64,
             partitioned: &[openwepp_land_surface_energy::LiquidParcel]| {
                if parent_rain.to_bits() == 0.0_f64.to_bits() {
                    return Ok(0.0);
                }
                let mut projected = partitioned
                    .iter()
                    .filter(|child| {
                        child.parcel_kind
                            == openwepp_land_surface_energy::LiquidParcelKind::Precipitation
                            && forcing.precipitation_parcels.iter().any(|parent| {
                                parent.parcel_kind
                                    == openwepp_land_surface_energy::LiquidParcelKind::Precipitation
                                    && parent.parcel_id == child.parcel_id
                                    && parent.amount_kg_m2_destination_tile_ground.to_bits()
                                        == parent_rain.to_bits()
                            })
                    })
                    .map(|parcel| parcel.amount_kg_m2_destination_tile_ground);
                let value = projected.next().unwrap_or(0.0);
                if projected.any(|candidate| candidate.to_bits() != value.to_bits()) {
                    return Err(DirectSnowStage3V11AttachmentError::Identity(
                        "coupled subslab covered precipitation path",
                    ));
                }
                Ok(value)
            };
        let segment_interval = |input: &DirectV9ShadowIntervalInput| {
            let mut value = input.clone();
            // All unpublished adaptive trials remain children of the same
            // immutable V11 parent transaction. The coupled child ordinal is
            // bound by its support and receipt; it must not manufacture a new
            // persistent vegetation/LSE transaction identity.
            let _ = child_ordinal;
            value.lse_forcing.interval_s = duration_seconds;
            value.lse_forcing.precipitation_parcels =
                partition_parcels(&input.lse_forcing.precipitation_parcels)?;
            value.lse_forcing.runon_parcels = partition_parcels(&input.lse_forcing.runon_parcels)?;
            value.vegetation_forcing.rain_kg_m2 = partition_vegetation_rain(
                &input.lse_forcing,
                input.vegetation_forcing.rain_kg_m2,
                &value.lse_forcing.precipitation_parcels,
            )?;
            value.lse_forcing.forcing_sha256 =
                value.lse_forcing.canonical_sha256().map_err(|_| {
                    DirectSnowStage3V11AttachmentError::Identity(
                        "coupled subslab LSE forcing digest",
                    )
                })?;
            Ok::<_, DirectSnowStage3V11AttachmentError>(value)
        };
        let v11_interval = segment_interval(&self.v11_interval)?;
        let covered_v11_interval = self
            .covered_v11_interval
            .as_ref()
            .map(|input| {
                let mut lse_forcing = input.lse_forcing.clone();
                let mut vegetation_forcing = input.vegetation_forcing.clone();
                lse_forcing.interval_s = duration_seconds;
                lse_forcing.precipitation_parcels =
                    partition_parcels(&input.lse_forcing.precipitation_parcels).map_err(|_| {
                        DirectV11RealConsumerError::Identity(
                            "coupled subslab covered precipitation timestamp",
                        )
                    })?;
                lse_forcing.runon_parcels = partition_parcels(&input.lse_forcing.runon_parcels)
                    .map_err(|_| {
                        DirectV11RealConsumerError::Identity(
                            "coupled subslab covered runon timestamp",
                        )
                    })?;
                vegetation_forcing.rain_kg_m2 = partition_vegetation_rain(
                    &input.lse_forcing,
                    input.vegetation_forcing.rain_kg_m2,
                    &lse_forcing.precipitation_parcels,
                )
                .map_err(|_| {
                    DirectV11RealConsumerError::Identity(
                        "coupled subslab covered precipitation path",
                    )
                })?;
                lse_forcing.forcing_sha256 = lse_forcing.canonical_sha256().map_err(|_| {
                    DirectV11RealConsumerError::Identity(
                        "coupled subslab covered LSE forcing digest",
                    )
                })?;
                DirectV11SnowCoveredSegmentInput::try_new(
                    lse_forcing,
                    vegetation_forcing,
                    input.wb14_parameters.clone(),
                )
            })
            .transpose()?;
        let support_forcing_by_lane = self
            .support_forcing_by_lane
            .iter()
            .map(|(lane_id, forcing)| {
                let mut child_forcing = forcing.forcing;
                let support_fraction = duration_seconds / parent_duration_seconds;
                child_forcing.active_precipitation_m *= support_fraction;
                child_forcing.rain_m *= support_fraction;
                child_forcing.snowfall_m *= support_fraction;
                child_forcing.radiation_mj_m2 *= support_fraction;
                (
                    *lane_id,
                    DirectSnowStage3SupportInput {
                        forcing: child_forcing,
                        duration_seconds,
                    },
                )
            })
            .collect();
        let snow_surface_forcing_by_destination = self
            .snow_surface_forcing_by_destination
            .iter()
            .map(|(destination, forcing)| {
                let projected = match forcing {
                    SealedStage3TileBoundaryForcingV1::V11CanopyCovered(value) => {
                        SealedStage3TileBoundaryForcingV1::V11CanopyCovered(value.clone())
                    }
                    SealedStage3TileBoundaryForcingV1::OpenSnow(value) => {
                        let exposure = SealedOpenSnowExposureReceiptV1::try_new(
                            support,
                            destination.clone(),
                            value.exposure.source_forcing_receipt_sha256,
                            value.exposure.source_wind_provider_sha256,
                            value.exposure.raw_or_projected_wind_m_s,
                            value.exposure.projection_model_definition_sha256,
                        )?;
                        SealedStage3TileBoundaryForcingV1::OpenSnow(
                            SealedOpenSnowTileForcingV1::try_new(
                                SealedOpenSnowTileForcingInputsV1 {
                                    support,
                                    destination: destination.clone(),
                                    forcing_receipt_sha256: value.forcing_receipt_sha256,
                                    exposure,
                                    reference_temperature_k: value.reference_temperature_k,
                                    reference_specific_humidity_kg_kg: value
                                        .reference_specific_humidity_kg_kg,
                                    air_pressure_pa: value.air_pressure_pa,
                                    atmospheric_downward_longwave_w_m2: value
                                        .atmospheric_downward_longwave_w_m2,
                                    direct_vis_w_m2: value.direct_vis_w_m2,
                                    diffuse_vis_w_m2: value.diffuse_vis_w_m2,
                                    direct_nir_w_m2: value.direct_nir_w_m2,
                                    diffuse_nir_w_m2: value.diffuse_nir_w_m2,
                                    rain_m: value.rain_m * support_fraction,
                                    snowfall_m: value.snowfall_m * support_fraction,
                                    precipitation_parcel_count: value.precipitation_parcel_count,
                                },
                            )?,
                        )
                    }
                };
                Ok((destination.clone(), projected))
            })
            .collect::<Result<BTreeMap<_, _>, DirectSnowStage3V11AttachmentError>>()?;
        Ok(Self {
            support,
            snow_inputs_by_lane: self.snow_inputs_by_lane.clone(),
            support_forcing_by_lane,
            v11_interval,
            snow_surface_forcing_by_destination,
            open_snow_destination_requests: self.open_snow_destination_requests.clone(),
            #[cfg(feature = "restart-authority-evidence")]
            restart_fixture_open_snow_shortwave_multiplier: self
                .restart_fixture_open_snow_shortwave_multiplier,
            atmospheric_receipt_by_destination: self.atmospheric_receipt_by_destination.clone(),
            covered_v11_interval,
            support_identity_by_lane: self.support_identity_by_lane.clone(),
            hard_boundaries: self.hard_boundaries.clone(),
        })
    }

    fn after_solid_reappearance_debit(
        &self,
        lanes: &BTreeSet<u32>,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        let mut debited = self.clone();
        for lane in lanes {
            let forcing = debited.support_forcing_by_lane.get_mut(lane).ok_or(
                DirectSnowStage3V11AttachmentError::Identity("solid reappearance forcing lane"),
            )?;
            debit_solid_reappearance_phase_v1(&mut forcing.forcing)?;
        }
        Ok(debited)
    }

    fn retain_active_snow_lanes(
        mut self,
        active_lanes: &BTreeSet<u32>,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        let active_destinations = self
            .support_identity_by_lane
            .iter()
            .filter(|(lane, _)| active_lanes.contains(lane))
            .flat_map(|(_, identities)| identities)
            .map(|identity| {
                Ok((
                    OfeId::try_new(identity.destination_ofe_id.clone()).map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Identity(
                            "terminal successor OFE identity",
                        )
                    })?,
                    TileId::try_new(identity.destination_tile_id.clone()).map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Identity(
                            "terminal successor tile identity",
                        )
                    })?,
                ))
            })
            .collect::<Result<BTreeSet<_>, DirectSnowStage3V11AttachmentError>>()?;
        self.snow_surface_forcing_by_destination
            .retain(|destination, _| active_destinations.contains(destination));
        self.open_snow_destination_requests
            .retain(|destination| active_destinations.contains(destination));
        Ok(self)
    }

    fn snow_free_successor(mut self) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        // The snow-free successor terminal V4 owner remains the authoritative
        // hydrology state; this projection only removes snow-boundary forcing.
        // In particular, it must not silently replace or mutate parcel custody:
        // snow-free successor changed pending terminal V4 custody is an error
        // checked by the parent transaction before this successor is installed.
        self.snow_surface_forcing_by_destination.clear();
        self.open_snow_destination_requests.clear();
        self.atmospheric_receipt_by_destination.clear();
        self.covered_v11_interval = None;
        self.v11_interval.lse_forcing.snow_present_at_beginning = false;
        self.v11_interval.lse_forcing.snow_present_at_end = false;
        self.v11_interval.lse_forcing.snow_terminal_payload_present = false;
        self.v11_interval.lse_forcing.forcing_sha256 = self
            .v11_interval
            .lse_forcing
            .canonical_sha256()
            .map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity(
                    "terminal successor LSE forcing digest",
                )
            })?;
        Ok(self)
    }

    fn with_terminal_receiver_parcels(
        mut self,
        pending: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        let interval_s = f64::from_bits(self.support.duration_s_bits());
        if pending.is_empty() || !interval_s.is_finite() || interval_s <= 0.0 {
            return Err(DirectSnowStage3V11AttachmentError::Terminal(
                "terminal receiver requires a positive support and parcel set",
            ));
        }
        for (digest, parcel) in pending {
            if parcel.posture != DirectSnowStage3V11TerminalParcelPosture::ProducedUnconsumed
                || parcel.parcel_digest != *digest
                || parcel.support.end_ns() != self.support.start_ns()
            {
                return Err(DirectSnowStage3V11AttachmentError::Terminal(
                    "terminal receiver predecessor parcel identity",
                ));
            }
            for (topology_index, destination) in parcel.receiver_destinations.iter().enumerate() {
                self.v11_interval.lse_forcing.precipitation_parcels.push(
                    openwepp_land_surface_energy::LiquidParcel {
                    parcel_kind:
                        openwepp_land_surface_energy::LiquidParcelKind::SnowTerminalReceiver,
                    parcel_id: openwepp_land_surface_energy::ParcelId::try_new(format!(
                        "snow-terminal-{}-{topology_index}",
                        digest32_lower_hex(*digest),
                    ))
                    .map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Terminal(
                            "terminal receiver parcel identity",
                        )
                    })?,
                    source_owner_id: openwepp_kernel_contract::ResourceOwnerId::try_new("snow")
                        .map_err(|_| {
                            DirectSnowStage3V11AttachmentError::Terminal(
                                "terminal receiver source owner",
                            )
                        })?,
                    source_ofe_id: OfeId::try_new(destination.destination_ofe_id.clone()).map_err(
                        |_| {
                            DirectSnowStage3V11AttachmentError::Terminal(
                                "terminal receiver source OFE",
                            )
                        },
                    )?,
                    source_tile_id: TileId::try_new(destination.destination_tile_id.clone()).map_err(
                        |_| {
                            DirectSnowStage3V11AttachmentError::Terminal(
                                "terminal receiver source tile",
                            )
                        },
                    )?,
                    destination_ofe_id: OfeId::try_new(destination.destination_ofe_id.clone()).map_err(
                        |_| {
                            DirectSnowStage3V11AttachmentError::Terminal(
                                "terminal receiver destination OFE",
                            )
                        },
                    )?,
                    destination_tile_id: TileId::try_new(destination.destination_tile_id.clone())
                        .map_err(|_| {
                            DirectSnowStage3V11AttachmentError::Terminal(
                                "terminal receiver destination tile",
                            )
                        })?,
                    start_s: 0.0,
                    end_s: interval_s,
                    amount_kg_m2_destination_tile_ground: parcel.mass_kg_m2_tile_ground,
                    temperature_provider:
                        openwepp_land_surface_energy::LiquidTemperatureProvider::AcceptedSnowTerminalParcel,
                    temperature_k: Some(parcel.temperature_k),
                    specific_liquid_enthalpy_j_kg: Some(
                        parcel.specific_liquid_enthalpy_j_kg,
                    ),
                    source_state_sha256: Some(
                        openwepp_land_surface_energy::Sha256Digest::try_new(
                            digest32_lower_hex(parcel.event_result_digest),
                        )
                        .map_err(|_| {
                            DirectSnowStage3V11AttachmentError::Terminal(
                                "terminal receiver source state",
                            )
                        })?,
                    ),
                    },
                );
            }
        }
        self.v11_interval.lse_forcing.forcing_sha256 = self
            .v11_interval
            .lse_forcing
            .canonical_sha256()
            .map_err(|_| {
                DirectSnowStage3V11AttachmentError::Terminal("terminal receiver forcing digest")
            })?;
        Ok(self)
    }

    #[cfg(test)]
    pub(crate) fn poison_base_air_temperature(&mut self) {
        self.v11_interval.lse_forcing.air_temperature_k = f64::from_bits(
            self.v11_interval
                .lse_forcing
                .air_temperature_k
                .to_bits()
                .wrapping_add(1),
        );
    }

    #[cfg(test)]
    pub(crate) fn poison_base_wind(&mut self) {
        self.v11_interval.lse_forcing.reference_wind_m_s = f64::from_bits(
            self.v11_interval
                .lse_forcing
                .reference_wind_m_s
                .to_bits()
                .wrapping_add(1),
        );
    }

    #[cfg(test)]
    pub(crate) fn poison_covered_atmosphere(&mut self, wind: bool) {
        let mut covered = DirectV11SnowCoveredSegmentInput::from_snow_free(&self.v11_interval);
        if wind {
            covered.lse_forcing.reference_wind_m_s = f64::from_bits(
                covered
                    .lse_forcing
                    .reference_wind_m_s
                    .to_bits()
                    .wrapping_add(1),
            );
        } else {
            covered.lse_forcing.air_temperature_k = f64::from_bits(
                covered
                    .lse_forcing
                    .air_temperature_k
                    .to_bits()
                    .wrapping_add(1),
            );
        }
        self.covered_v11_interval = Some(covered);
    }

    #[cfg(test)]
    pub(crate) fn poison_stage3_pressure(&mut self) {
        if let Some(inputs) = self.snow_inputs_by_lane.values_mut().next() {
            inputs.surface_energy_options.atmospheric_pressure_pa = f64::from_bits(
                inputs
                    .surface_energy_options
                    .atmospheric_pressure_pa
                    .to_bits()
                    .wrapping_add(1),
            );
        }
    }

    #[cfg(test)]
    pub(crate) fn poison_stage3_dewpoint(&mut self) {
        if let Some(inputs) = self.snow_inputs_by_lane.values_mut().next() {
            inputs.dewpoint_c = f64::from_bits(inputs.dewpoint_c.to_bits().wrapping_add(1));
        }
    }

    fn has_snow_surface_forcing(&self) -> bool {
        !self.snow_surface_forcing_by_destination.is_empty()
    }

    #[must_use]
    pub fn snow_surface_regime(&self) -> Stage3SnowSurfaceRegime {
        if self.snow_surface_forcing_by_destination.is_empty() {
            Stage3SnowSurfaceRegime::SnowFree
        } else if self
            .snow_surface_forcing_by_destination
            .values()
            .all(|forcing| matches!(forcing, SealedStage3TileBoundaryForcingV1::OpenSnow(_)))
        {
            Stage3SnowSurfaceRegime::OpenSnowOnly
        } else {
            Stage3SnowSurfaceRegime::CanopyCoveredOrMixed
        }
    }

    fn validate_explicit_snow_surface_set(&self) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if self.snow_surface_forcing_by_destination.is_empty() {
            return Ok(());
        }
        let expected = self
            .support_identity_by_lane
            .values()
            .flatten()
            .map(|identity| {
                Ok((
                    OfeId::try_new(identity.destination_ofe_id.clone()).map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Support("snow-surface OFE identity")
                    })?,
                    TileId::try_new(identity.destination_tile_id.clone()).map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Support("snow-surface tile identity")
                    })?,
                ))
            })
            .collect::<Result<BTreeSet<_>, DirectSnowStage3V11AttachmentError>>()?;
        if !self
            .snow_surface_forcing_by_destination
            .keys()
            .all(|destination| expected.contains(destination))
        {
            return Err(DirectSnowStage3V11AttachmentError::Support(
                "snow-surface destination outside configured topology",
            ));
        }
        for identities in self.support_identity_by_lane.values() {
            for identity in identities {
                let destination = (
                    OfeId::try_new(identity.destination_ofe_id.clone()).map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Support("snow-surface OFE identity")
                    })?,
                    TileId::try_new(identity.destination_tile_id.clone()).map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Support("snow-surface tile identity")
                    })?,
                );
                let Some(physical) = self.snow_surface_forcing_by_destination.get(&destination)
                else {
                    continue;
                };
                let exposure_identity = match physical {
                    SealedStage3TileBoundaryForcingV1::V11CanopyCovered(forcing) => {
                        forcing.exposure_identity()
                    }
                    SealedStage3TileBoundaryForcingV1::OpenSnow(forcing) => {
                        forcing.validate().map_err(|_| {
                            DirectSnowStage3V11AttachmentError::Support("sealed open-snow forcing")
                        })?;
                        if forcing.forcing_receipt_sha256 != identity.forcing_receipt_digest {
                            return Err(DirectSnowStage3V11AttachmentError::Support(
                                "open-snow/provider forcing receipt join",
                            ));
                        }
                        forcing.exposure.receipt_sha256
                    }
                };
                if identity.exposure_identity != exposure_identity {
                    return Err(DirectSnowStage3V11AttachmentError::Support(
                        "support exposure identity/physical receipt join",
                    ));
                }
            }
        }
        self.validate_precipitation_custody()?;
        Ok(())
    }

    fn validate_precipitation_custody(&self) -> Result<(), DirectSnowStage3V11AttachmentError> {
        for support in self.support_forcing_by_lane.values() {
            let forcing = support.forcing;
            if !forcing.active_precipitation_m.is_finite()
                || !forcing.rain_m.is_finite()
                || !forcing.snowfall_m.is_finite()
                || forcing.active_precipitation_m < 0.0
                || forcing.rain_m < 0.0
                || forcing.snowfall_m < 0.0
                || (forcing.active_precipitation_m - (forcing.rain_m + forcing.snowfall_m * 0.1))
                    .abs()
                    > 1.0e-12
            {
                return Err(DirectSnowStage3V11AttachmentError::Precipitation(
                    "Stage-3 support precipitation phase closure",
                ));
            }
        }
        Ok(())
    }

    fn state_derived_active_snow_lanes(
        &self,
        beginning: &BTreeMap<u32, DirectSnowStage3PersistentState>,
        pending_terminal_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    ) -> Result<BTreeSet<u32>, DirectSnowStage3V11AttachmentError> {
        let mut represented = BTreeMap::<u32, BTreeSet<(OfeId, TileId)>>::new();
        for (lane_id, identities) in &self.support_identity_by_lane {
            for identity in identities {
                let destination = (
                    OfeId::try_new(identity.destination_ofe_id.clone()).map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Support("snow regime OFE")
                    })?,
                    TileId::try_new(identity.destination_tile_id.clone()).map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Support("snow regime tile")
                    })?,
                );
                if self
                    .snow_surface_forcing_by_destination
                    .contains_key(&destination)
                {
                    represented.entry(*lane_id).or_default().insert(destination);
                }
            }
        }
        let mut active = BTreeSet::new();
        for (lane_id, state) in beginning {
            let forcing = self.support_forcing_by_lane.get(lane_id).ok_or(
                DirectSnowStage3V11AttachmentError::Support("snow regime lane forcing"),
            )?;
            let lifecycle = stage3_lane_lifecycle(state, forcing.forcing.snowfall_m);
            let expected = self
                .support_identity_by_lane
                .get(lane_id)
                .ok_or(DirectSnowStage3V11AttachmentError::Support(
                    "snow regime lane topology",
                ))?
                .iter()
                .map(|identity| {
                    Ok((
                        OfeId::try_new(identity.destination_ofe_id.clone()).map_err(|_| {
                            DirectSnowStage3V11AttachmentError::Support("snow regime OFE")
                        })?,
                        TileId::try_new(identity.destination_tile_id.clone()).map_err(|_| {
                            DirectSnowStage3V11AttachmentError::Support("snow regime tile")
                        })?,
                    ))
                })
                .collect::<Result<BTreeSet<_>, DirectSnowStage3V11AttachmentError>>()?;
            let actual = represented.get(lane_id).cloned().unwrap_or_default();
            match lifecycle {
                Stage3LaneLifecycleV1::ResolvedSnow => {
                    active.insert(*lane_id);
                    if actual != expected {
                        return Err(DirectSnowStage3V11AttachmentError::Support(
                            "active snow lane requires complete destination boundary set",
                        ));
                    }
                }
                Stage3LaneLifecycleV1::SnowFree => {
                    // A production-prepared support carries both the
                    // snow-free adopter and the complete snow-surface
                    // capability. Presence here is capability, not a claim
                    // that Stage 3 owns the current surface. Sequential live
                    // state remains the only regime selector.
                    if !actual.is_empty() && actual != expected {
                        return Err(DirectSnowStage3V11AttachmentError::Support(
                            "snow-free lane has an incomplete future snow-surface capability",
                        ));
                    }
                }
                Stage3LaneLifecycleV1::TerminalPending => {
                    if !terminal_domain_can_cross_parent_support(
                        state,
                        !pending_terminal_parcels.is_empty(),
                    ) {
                        return Err(DirectSnowStage3V11AttachmentError::Terminal(
                            "Stage-3 lane requires terminal disposition",
                        ));
                    }
                    active.insert(*lane_id);
                    if actual != expected {
                        return Err(DirectSnowStage3V11AttachmentError::Support(
                            "terminal-domain lane requires complete destination boundary set",
                        ));
                    }
                }
                Stage3LaneLifecycleV1::SolidPrecipitationPending => {
                    active.insert(*lane_id);
                    if actual != expected {
                        return Err(DirectSnowStage3V11AttachmentError::Support(
                            "solid-precipitation reappearance requires complete destination boundary set",
                        ));
                    }
                }
            }
        }
        Ok(active)
    }

    fn validate_dual_regime_capability(&self) -> Result<(), DirectSnowStage3V11AttachmentError> {
        let expected = self
            .support_identity_by_lane
            .values()
            .flatten()
            .map(|identity| {
                Ok((
                    OfeId::try_new(identity.destination_ofe_id.clone()).map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Support(
                            "dual-regime capability OFE identity",
                        )
                    })?,
                    TileId::try_new(identity.destination_tile_id.clone()).map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Support(
                            "dual-regime capability tile identity",
                        )
                    })?,
                ))
            })
            .collect::<Result<BTreeSet<_>, DirectSnowStage3V11AttachmentError>>()?;
        let actual = self
            .snow_surface_forcing_by_destination
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>();
        if self.covered_v11_interval.is_none() || actual != expected {
            return Err(DirectSnowStage3V11AttachmentError::Support(
                "production support requires sealed snow-free and complete snow-surface capabilities",
            ));
        }
        Ok(())
    }

    fn forcing_projections(&self) -> (Digest32, Digest32, Digest32, Digest32) {
        let stage3_support_forcing_sha256 =
            canonical_stage3_support_forcing_digest(&self.support_forcing_by_lane);
        let stage3_configuration_sha256 =
            canonical_stage3_configuration_digest(&self.snow_inputs_by_lane);
        let covered_v11_forcing_sha256 =
            if let Some(covered_interval) = self.covered_v11_interval.as_ref() {
                canonical_v11_forcing_digest(
                    &covered_interval.lse_forcing,
                    &covered_interval.vegetation_forcing,
                )
            } else {
                canonical_v11_forcing_digest(
                    &self.v11_interval.lse_forcing,
                    &self.v11_interval.vegetation_forcing,
                )
            };
        let carrier_configuration_sha256 =
            canonical_snow_surface_forcing_digest(&self.snow_surface_forcing_by_destination);
        (
            stage3_support_forcing_sha256,
            stage3_configuration_sha256,
            covered_v11_forcing_sha256,
            carrier_configuration_sha256,
        )
    }
}

include!("snow_stage3_v11_prepared_support_identity.rs");

#[derive(Clone, Debug, PartialEq)]
pub struct PreparedStage3V11DayV1 {
    day_index: usize,
    accepted_gsi_receipt: Digest32,
    beginning_provider_cursor: SnowFreeHalfHourProviderCursor,
    ending_provider_cursor: SnowFreeHalfHourProviderCursor,
    supports: Vec<DirectSnowStage3V11PreparedSupport>,
}

/// Opaque provider/GSI-joined capability accepted by the closure path.
#[derive(Clone, Debug)]
pub struct ValidatedPreparedStage3V11DayV1 {
    inner: PreparedStage3V11DayV1,
    provider_day: PreparedSnowFreeGsiDayV1,
}

pub type DirectSnowStage3V11PreparedDay = ValidatedPreparedStage3V11DayV1;
pub type PreparedStage3V11SupportV1 = DirectSnowStage3V11PreparedSupport;

impl PreparedStage3V11DayV1 {
    /// Bind the production day capability. Unlike the lower-level
    /// constructor used by focused owner tests, every one of the 48 supports
    /// must carry both the snow-free adopter and a complete snow-surface
    /// capability. The runner therefore never predicts whether live state at
    /// a future support will be snow-free, persistent snow, meltout, or solid-
    /// precipitation reappearance.
    pub fn bind_production_provider_day(
        provider: &PreparedSnowFreeGsiDayV1,
        day_index: usize,
        supports: Vec<DirectSnowStage3V11PreparedSupport>,
    ) -> Result<ValidatedPreparedStage3V11DayV1, DirectSnowStage3V11AttachmentError> {
        let prepared = Self::bind_provider_day(provider, day_index, supports)?;
        for support in &prepared.inner.supports {
            support.validate_dual_regime_capability()?;
        }
        Ok(prepared)
    }

    /// Bind runner-built support operands to the already validated repository
    /// provider day. This is the only constructor that admits provider/GSI
    /// identity into the sealed 48-support capability.
    #[allow(clippy::too_many_lines)]
    pub fn bind_provider_day(
        provider: &PreparedSnowFreeGsiDayV1,
        day_index: usize,
        mut supports: Vec<DirectSnowStage3V11PreparedSupport>,
    ) -> Result<ValidatedPreparedStage3V11DayV1, DirectSnowStage3V11AttachmentError> {
        if supports.len() != STAGE3_V11_PARENT_SUPPORT_COUNT {
            return Err(DirectSnowStage3V11AttachmentError::Support(
                "provider binding requires exactly 48 supports",
            ));
        }
        provider.gsi_receipt().validate()?;
        if provider.gsi_receipt().day_index
            != u64::try_from(day_index).map_err(|_| {
                DirectSnowStage3V11AttachmentError::Support("provider day index width")
            })?
        {
            return Err(DirectSnowStage3V11AttachmentError::Support(
                "provider day index",
            ));
        }
        let accepted_gsi_receipt = provider.gsi_receipt_digest()?;
        let day_start_ns = day_start_ns(day_index)?;
        for (support_index, support) in supports.iter_mut().enumerate() {
            let provider_destinations = provider
                .forcing_receipts()
                .receipts()
                .iter()
                .filter(|day| day.day_index == day_index)
                .map(|day| {
                    let interval = day.intervals.get(support_index).ok_or(
                        DirectSnowStage3V11AttachmentError::Support(
                            "provider interval cardinality",
                        ),
                    )?;
                    Ok((
                        (interval.ofe_id.clone(), interval.tile_id.clone()),
                        interval,
                    ))
                })
                .collect::<Result<BTreeMap<_, _>, DirectSnowStage3V11AttachmentError>>()?;
            if provider_destinations.is_empty() {
                return Err(DirectSnowStage3V11AttachmentError::Support(
                    "provider day destination set",
                ));
            }
            support.bind_provider_atmosphere(&provider_destinations)?;
            support.validate_explicit_snow_surface_set()?;
            let mut support_destinations = BTreeSet::new();
            for identity in support
                .support_identity_by_lane
                .values()
                .flat_map(|identities| identities.iter())
            {
                if identity.exposure_identity == Digest32::zero() {
                    return Err(DirectSnowStage3V11AttachmentError::Support(
                        "missing sealed exposure identity",
                    ));
                }
                if !support_destinations.insert((
                    identity.destination_ofe_id.clone(),
                    identity.destination_tile_id.clone(),
                )) {
                    return Err(DirectSnowStage3V11AttachmentError::Support(
                        "duplicate support destination identity",
                    ));
                }
            }
            if support_destinations != provider_destinations.keys().cloned().collect() {
                return Err(DirectSnowStage3V11AttachmentError::Support(
                    "support/provider destination set",
                ));
            }
            for identity in support
                .support_identity_by_lane
                .values()
                .flat_map(|identities| identities.iter())
            {
                let interval = provider_destinations
                    .get(&(
                        identity.destination_ofe_id.clone(),
                        identity.destination_tile_id.clone(),
                    ))
                    .ok_or(DirectSnowStage3V11AttachmentError::Support(
                        "support/provider destination interval join",
                    ))?;
                let receipt_digest = parse_lower_hex_digest(&interval.interval_receipt_sha256)?;
                let interval_start_ns = day_start_ns
                    .checked_add(
                        u128::try_from(interval.start_s)
                            .map_err(|_| {
                                DirectSnowStage3V11AttachmentError::Support(
                                    "provider interval start width",
                                )
                            })?
                            .checked_mul(1_000_000_000)
                            .ok_or(DirectSnowStage3V11AttachmentError::Support(
                                "provider interval start overflow",
                            ))?,
                    )
                    .ok_or(DirectSnowStage3V11AttachmentError::Support(
                        "provider interval start day overflow",
                    ))?;
                let interval_end_ns = day_start_ns
                    .checked_add(
                        u128::try_from(interval.end_s)
                            .map_err(|_| {
                                DirectSnowStage3V11AttachmentError::Support(
                                    "provider interval end width",
                                )
                            })?
                            .checked_mul(1_000_000_000)
                            .ok_or(DirectSnowStage3V11AttachmentError::Support(
                                "provider interval end overflow",
                            ))?,
                    )
                    .ok_or(DirectSnowStage3V11AttachmentError::Support(
                        "provider interval end day overflow",
                    ))?;
                if identity.forcing_receipt_digest != receipt_digest
                    || interval.gsi_receipt_sha256 != provider.gsi_receipt().receipt_sha256
                    || interval.wb14_configuration_sha256 != identity.wb14_configuration_sha256
                    || interval.precipitation_parcels != identity.precipitation_parcels
                    || interval.solid_precipitation_parcels != identity.solid_precipitation_parcels
                    || support.support.start_ns().get() != interval_start_ns
                    || support.support.end_ns().get() != interval_end_ns
                {
                    return Err(DirectSnowStage3V11AttachmentError::Support(
                        "sealed provider support operands",
                    ));
                }
            }
        }
        Ok(ValidatedPreparedStage3V11DayV1 {
            inner: Self {
                day_index,
                accepted_gsi_receipt,
                beginning_provider_cursor: provider.forcing_receipts().beginning_cursor().clone(),
                ending_provider_cursor: provider.forcing_receipts().ending_cursor().clone(),
                supports,
            },
            provider_day: provider.clone(),
        })
    }

    fn validate(
        &self,
        context: &DirectSnowStage3V11StaticContext,
        expected_start_ns: u128,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if self.supports.len() != STAGE3_V11_PARENT_SUPPORT_COUNT {
            return Err(DirectSnowStage3V11AttachmentError::Support(
                "a prepared day requires exactly 48 parent supports",
            ));
        }
        let expected_lanes = context.lane_ids.iter().copied().collect::<BTreeSet<_>>();
        let mut cursor = expected_start_ns;
        for support in &self.supports {
            if support.support.start_ns().get() != cursor
                || support.support.duration_ns() != context.parent_duration_ns
                || support
                    .snow_inputs_by_lane
                    .keys()
                    .copied()
                    .collect::<BTreeSet<_>>()
                    != expected_lanes
                || support
                    .support_forcing_by_lane
                    .keys()
                    .copied()
                    .collect::<BTreeSet<_>>()
                    != expected_lanes
                || support
                    .support_identity_by_lane
                    .keys()
                    .copied()
                    .collect::<BTreeSet<_>>()
                    != expected_lanes
                || support.support_identity_by_lane.values().any(Vec::is_empty)
            {
                return Err(DirectSnowStage3V11AttachmentError::Support(
                    "support chronology or lane forcing identity",
                ));
            }
            support.validate_explicit_snow_surface_set()?;
            if (support.snow_surface_regime() != Stage3SnowSurfaceRegime::SnowFree)
                != support.covered_v11_interval.is_some()
            {
                return Err(DirectSnowStage3V11AttachmentError::Support(
                    "snow-surface support requires persistent-snow V11 projection",
                ));
            }
            cursor = support.support.end_ns().get();
        }
        Ok(())
    }

    fn validate_provider_join(
        &self,
        expected_beginning_cursor: &SnowFreeHalfHourProviderCursor,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if &self.beginning_provider_cursor != expected_beginning_cursor
            || self.beginning_provider_cursor == self.ending_provider_cursor
        {
            return Err(DirectSnowStage3V11AttachmentError::Support(
                "prepared day provider cursor join",
            ));
        }
        Ok(())
    }
}

impl ValidatedPreparedStage3V11DayV1 {
    #[must_use]
    pub const fn day_index(&self) -> usize {
        self.inner.day_index
    }

    #[must_use]
    pub const fn accepted_gsi_receipt(&self) -> Digest32 {
        self.inner.accepted_gsi_receipt
    }

    #[must_use]
    pub fn supports(&self) -> &[DirectSnowStage3V11PreparedSupport] {
        &self.inner.supports
    }

    #[must_use]
    pub const fn beginning_provider_cursor(&self) -> &SnowFreeHalfHourProviderCursor {
        &self.inner.beginning_provider_cursor
    }

    #[must_use]
    pub const fn ending_provider_cursor(&self) -> &SnowFreeHalfHourProviderCursor {
        &self.inner.ending_provider_cursor
    }

    fn validate(
        &self,
        context: &DirectSnowStage3V11StaticContext,
        expected_start_ns: u128,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        self.inner.validate(context, expected_start_ns)?;
        self.validate_lane_destination_bindings(context)
    }

    fn validate_lane_destination_bindings(
        &self,
        context: &DirectSnowStage3V11StaticContext,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        let provider_destinations_by_ofe = self
            .provider_day
            .forcing_receipts()
            .receipts()
            .iter()
            .flat_map(|day| day.intervals.iter())
            .fold(
                BTreeMap::<String, BTreeSet<(String, String)>>::new(),
                |mut destinations, interval| {
                    destinations
                        .entry(interval.ofe_id.clone())
                        .or_default()
                        .insert((interval.ofe_id.clone(), interval.tile_id.clone()));
                    destinations
                },
            );
        for support in &self.inner.supports {
            for (lane_id, identities) in &support.support_identity_by_lane {
                let binding = context
                    .surface_liquid_configuration
                    .ofe_bindings
                    .iter()
                    .find(|binding| binding.production_lane_id == *lane_id)
                    .ok_or(DirectSnowStage3V11AttachmentError::Support(
                        "support lane surface-liquid binding",
                    ))?;
                let expected = provider_destinations_by_ofe
                    .get(binding.ofe_id.as_str())
                    .ok_or(DirectSnowStage3V11AttachmentError::Support(
                        "support lane provider OFE destinations",
                    ))?;
                validate_lane_destination_set(binding.ofe_id.as_str(), identities, expected)?;
            }
        }
        Ok(())
    }

    fn validate_provider_join(
        &self,
        expected_beginning_cursor: &SnowFreeHalfHourProviderCursor,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        self.inner.validate_provider_join(expected_beginning_cursor)
    }

    fn into_provider_day(self) -> PreparedSnowFreeGsiDayV1 {
        self.provider_day
    }
}

include!("snow_stage3_v11_snow_free_successor.rs");
include!("snow_stage3_v11_attachment_receipts.rs");
include!("snow_stage3_v11_adaptive_identity.rs");
#[derive(Clone, Debug, PartialEq)]
pub struct DirectSnowStage3V11CommittedState {
    pub stage3_by_lane: BTreeMap<u32, DirectSnowStage3PersistentState>,
    pub real_consumer: DirectV10RealConsumerShadow,
    pub v11_parent_state: V11ParentTransaction,
    pub coupled_clock: CoupledClockStateV1,
    pub next_parent_sequence: u128,
    pub last_v11_parent_candidate: Option<V11ParentCandidate>,
    pub terminal_parcels: BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    pub receipt_chain: Vec<DirectSnowStage3V11ParentReceipt>,
    pub snow_enthalpy_material_owner:
        Option<crate::snow_stage3_v11_snow_enthalpy_carry::AuthenticatedCoveredSnowMaterialOwnerV1>,
    pub snow_enthalpy_material_owner_chronology:
        Vec<crate::snow_stage3_v11_snow_enthalpy_carry::AuthenticatedCoveredSnowMaterialOwnerV1>,
}

impl DirectSnowStage3V11CommittedState {
    fn validate_snow_enthalpy_material_resident_v1(
        &self,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        match &self.snow_enthalpy_material_owner {
            None if self.snow_enthalpy_material_owner_chronology.is_empty() => Ok(()),
            None => Err(DirectSnowStage3V11AttachmentError::Identity(
                "V56 snow material chronology without current owner",
            )),
            Some(current) => {
                current.validate().map_err(|_| {
                    DirectSnowStage3V11AttachmentError::Identity(
                        "V56 current compound snow material owner",
                    )
                })?;
                if current.base_material_owner() != &self.stage3_by_lane
                    || self.snow_enthalpy_material_owner_chronology.last() != Some(current)
                {
                    return Err(DirectSnowStage3V11AttachmentError::Identity(
                        "V56 current compound owner/base/chronology join",
                    ));
                }
                for pair in self.snow_enthalpy_material_owner_chronology.windows(2) {
                    pair[1]
                        .receipt()
                        .validate_successor_of(&pair[0])
                        .map_err(|_| {
                            DirectSnowStage3V11AttachmentError::Identity(
                                "V56 compound snow material successor chronology",
                            )
                        })?;
                }
                Ok(())
            }
        }
    }

    pub fn snow_enthalpy_material_owner_v1(
        &self,
    ) -> Option<&crate::snow_stage3_v11_snow_enthalpy_carry::AuthenticatedCoveredSnowMaterialOwnerV1>
    {
        self.snow_enthalpy_material_owner.as_ref()
    }

    pub fn snow_enthalpy_material_owner_chronology_v1(
        &self,
    ) -> &[crate::snow_stage3_v11_snow_enthalpy_carry::AuthenticatedCoveredSnowMaterialOwnerV1]
    {
        &self.snow_enthalpy_material_owner_chronology
    }

    pub fn install_snow_enthalpy_material_owner_v1(
        &mut self,
        owner: crate::snow_stage3_v11_snow_enthalpy_carry::AuthenticatedCoveredSnowMaterialOwnerV1,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        owner.validate().map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity(
                "V56 accepted compound snow material owner",
            )
        })?;
        if owner.base_material_owner() != &self.stage3_by_lane {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "V56 accepted compound owner/base material join",
            ));
        }
        if let Some(predecessor) = self.snow_enthalpy_material_owner.as_ref() {
            owner
                .receipt()
                .validate_successor_of(predecessor)
                .map_err(|_| {
                    DirectSnowStage3V11AttachmentError::Identity(
                        "V56 accepted compound owner predecessor",
                    )
                })?;
        } else if owner.receipt().predecessor_transaction_id().is_some()
            || owner.receipt().predecessor_receipt_chain_sha256() != Digest32::zero()
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "V56 initial compound owner has foreign predecessor",
            ));
        }
        let mut chronology = self.snow_enthalpy_material_owner_chronology.clone();
        chronology.push(owner.clone());
        let previous_owner = self.snow_enthalpy_material_owner.replace(owner);
        let previous_chronology = std::mem::replace(
            &mut self.snow_enthalpy_material_owner_chronology,
            chronology,
        );
        if let Err(error) = self.validate_snow_enthalpy_material_resident_v1() {
            self.snow_enthalpy_material_owner = previous_owner;
            self.snow_enthalpy_material_owner_chronology = previous_chronology;
            return Err(error);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SnowStage3V11SnowEnthalpyMaterialResidentV1 {
    pub current_owner:
        Option<crate::snow_stage3_v11_snow_enthalpy_carry::AuthenticatedCoveredSnowMaterialOwnerV1>,
    pub accepted_owner_chronology:
        Vec<crate::snow_stage3_v11_snow_enthalpy_carry::AuthenticatedCoveredSnowMaterialOwnerV1>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SnowStage3V11SnowEnthalpyMaterialResidentSetV1 {
    pub committed: SnowStage3V11SnowEnthalpyMaterialResidentV1,
    pub pending_candidate: Option<SnowStage3V11SnowEnthalpyMaterialResidentV1>,
    pub in_progress_day_candidate: Option<SnowStage3V11SnowEnthalpyMaterialResidentV1>,
    pub in_progress_support_current: Option<SnowStage3V11SnowEnthalpyMaterialResidentV1>,
}

fn snow_enthalpy_material_resident_from_committed_v1(
    state: &DirectSnowStage3V11CommittedState,
) -> Result<SnowStage3V11SnowEnthalpyMaterialResidentV1, DirectSnowStage3V11AttachmentError> {
    state.validate_snow_enthalpy_material_resident_v1()?;
    Ok(SnowStage3V11SnowEnthalpyMaterialResidentV1 {
        current_owner: state.snow_enthalpy_material_owner.clone(),
        accepted_owner_chronology: state.snow_enthalpy_material_owner_chronology.clone(),
    })
}

fn install_snow_enthalpy_material_resident_into_committed_v1(
    state: &mut DirectSnowStage3V11CommittedState,
    resident: SnowStage3V11SnowEnthalpyMaterialResidentV1,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    let previous_owner = std::mem::replace(
        &mut state.snow_enthalpy_material_owner,
        resident.current_owner,
    );
    let previous_chronology = std::mem::replace(
        &mut state.snow_enthalpy_material_owner_chronology,
        resident.accepted_owner_chronology,
    );
    if let Err(error) = state.validate_snow_enthalpy_material_resident_v1() {
        state.snow_enthalpy_material_owner = previous_owner;
        state.snow_enthalpy_material_owner_chronology = previous_chronology;
        return Err(error);
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq)]
pub struct DirectSnowStage3V11ParentCandidate {
    pub ending_state: DirectSnowStage3V11CommittedState,
    pub parent_receipt: DirectSnowStage3V11ParentReceipt,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DirectSnowStage3V11ShadowAttachment {
    pub static_context: DirectSnowStage3V11StaticContext,
    pub committed: DirectSnowStage3V11CommittedState,
    archived_receipt_prefix: Stage3ArchivedReceiptPrefixV1,
    pending_committed_day_evidence: Option<Stage3PendingCommittedDayEvidenceV1>,
    pending_candidate: Option<DirectSnowStage3V11ParentCandidate>,
    pending_publication_day: Option<crate::direct_runtime::Stage3AcceptedPublicationDayV1>,
    committed_publication_day: Option<crate::direct_runtime::Stage3AcceptedPublicationDayV1>,
    in_progress_execution: Option<Box<DirectSnowStage3V11InProgressExecutionV2>>,
    failure_injection: Option<Stage3V11FailureInjection>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Stage3V11FailureInjection {
    SubslabAccepted(usize),
    OutcomeLedgerBuilt(usize),
    PrecipitationReceiptRejected(usize),
    SnowSoilHeatReceiptRejected(usize),
    ParentEndTerminalReceiverCompleted,
    FinalOwnerJoinCompleted,
}

include!("snow_stage3_v11_attachment_runtime.rs");
include!("snow_stage3_v11_production_qualification.rs");
include!("snow_stage3_v11_committed_day_archive.rs");

fn validate_prepared_day_against_committed_provider(
    committed: &DirectSnowStage3V11CommittedState,
    prepared: &ValidatedPreparedStage3V11DayV1,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    let owner_destinations = committed
        .real_consumer
        .provider_static_configuration()
        .destinations
        .iter()
        .map(|destination| (destination.ofe_id.clone(), destination.tile_id.clone()))
        .collect::<BTreeSet<_>>();
    let prepared_destinations = prepared
        .supports()
        .first()
        .into_iter()
        .flat_map(|support| {
            support
                .support_identity_by_lane
                .values()
                .flat_map(|identities| identities.iter())
                .map(|identity| {
                    (
                        identity.destination_ofe_id.clone(),
                        identity.destination_tile_id.clone(),
                    )
                })
        })
        .collect::<BTreeSet<_>>();
    if prepared_destinations != owner_destinations {
        return Err(DirectSnowStage3V11AttachmentError::Support(
            "prepared/committed provider destination topology",
        ));
    }
    committed
        .real_consumer
        .provider_cursor()
        .validate_for_configuration(
            committed.real_consumer.provider_static_configuration(),
            prepared.day_index(),
        )?;
    let beginning_gsi_state = direct_gsi_state(committed.real_consumer.gsi_state())?;
    let prepared_gsi_receipt = prepared.provider_day.gsi_receipt();
    if prepared_gsi_receipt.configuration_sha256
        != committed
            .real_consumer
            .gsi_owner_configuration()
            .configuration_sha256
        || prepared_gsi_receipt.run_id
            != committed
                .real_consumer
                .provider_static_configuration()
                .run_id
        || prepared_gsi_receipt.beginning_state != beginning_gsi_state
    {
        return Err(DirectSnowStage3V11AttachmentError::Support(
            "prepared beginning GSI owner state",
        ));
    }
    prepared.validate_provider_join(committed.real_consumer.provider_cursor())
}

fn begin_v11_parent_for_support(
    context: &DirectSnowStage3V11StaticContext,
    committed: &DirectSnowStage3V11CommittedState,
    prepared: &DirectSnowStage3V11PreparedSupport,
    forcing_receipt: Digest32,
    parent_sequence: u128,
) -> Result<(V11ParentTransaction, CoupledClockStateV1), DirectSnowStage3V11AttachmentError> {
    let beginning_state = committed.last_v11_parent_candidate.as_ref().map_or_else(
        || committed.v11_parent_state.beginning_state(),
        |candidate| &candidate.ending_state,
    );
    let beginning_owners = committed.coupled_clock.owners().to_vec();
    let beginning_owner_digest = complete_owner_set_digest(&beginning_owners)?;
    let authority = ParentAuthorityV1::new(
        context.run_identity,
        context.calendar_receipt,
        forcing_receipt,
        parent_sequence,
        prepared.support,
        beginning_owner_digest,
    )?;
    let participants = committed.coupled_clock.active_participants().to_vec();
    let clock = CoupledClockStateV1::new(
        authority,
        beginning_owners.clone(),
        "snow-stage3-v11".to_owned(),
        participants,
        context.controller_policy,
        Vec::new(),
    )?;
    let parent = V11ParentTransaction::new_with_complete_owners(
        &context.vegetation_configuration,
        beginning_state,
        clock.parent_transaction_id(),
        prepared.support.start_ns(),
        owner_envelopes_from_states(&beginning_owners)?,
    )?;
    Ok((parent, clock))
}

include!("snow_stage3_v11_forcing_digest.rs");

include!("snow_stage3_v11_real_parent_execution.rs");
include!("snow_stage3_v11_adaptive_execution.rs");

include!("snow_stage3_v11_restart.rs");
include!("snow_stage3_v11_terminal_execution.rs");
#[cfg(test)]
mod fixed_point_limiter_audit_tests {
    use super::*;

    #[test]
    fn limiter_audit_is_explicit_thread_local_and_retains_a_bounded_tail() {
        let support = TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(60_000_000_000))
            .expect("limiter audit support");
        let _guard = begin_covered_fixed_point_iteration_audit_v1();
        for iteration in 1..=(COVERED_FIXED_POINT_LIMITER_AUDIT_CAPACITY + 3) {
            let delta = iteration as f64;
            record_covered_fixed_point_limiter_sample_v1(CoveredFixedPointLimiterSampleV1 {
                support,
                iteration,
                stage: CoveredFixedPointLimitStageV1::Picard,
                lse_converged: false,
                stage3_converged: false,
                soil_converged: false,
                boundary_converged: false,
                lse_max_normalized_delta_bits: delta.to_bits(),
                stage3_max_normalized_delta_bits: delta.to_bits(),
                soil_enthalpy_max_normalized_delta_bits: delta.to_bits(),
                soil_temperature_max_normalized_delta_bits: delta.to_bits(),
                boundary_max_normalized_delta_bits: delta.to_bits(),
            });
        }

        let audit = take_covered_fixed_point_limiter_audit_v1();
        assert_eq!(
            audit.total_sample_count,
            (COVERED_FIXED_POINT_LIMITER_AUDIT_CAPACITY + 3) as u64
        );
        assert_eq!(audit.dropped_sample_count, 3);
        assert_eq!(
            audit.retained_tail.len(),
            COVERED_FIXED_POINT_LIMITER_AUDIT_CAPACITY
        );
        assert_eq!(
            audit.retained_tail.first().map(|row| row.iteration),
            Some(4)
        );
        assert_eq!(
            audit.retained_tail.last().map(|row| row.iteration),
            Some(COVERED_FIXED_POINT_LIMITER_AUDIT_CAPACITY + 3)
        );
        assert_eq!(
            f64::from_bits(audit.peak_lse_normalized_delta_bits),
            (COVERED_FIXED_POINT_LIMITER_AUDIT_CAPACITY + 3) as f64
        );

        record_covered_fixed_point_limiter_sample_v1(CoveredFixedPointLimiterSampleV1 {
            support,
            iteration: usize::MAX,
            stage: CoveredFixedPointLimitStageV1::Picard,
            lse_converged: true,
            stage3_converged: true,
            soil_converged: true,
            boundary_converged: true,
            lse_max_normalized_delta_bits: 0.0_f64.to_bits(),
            stage3_max_normalized_delta_bits: 0.0_f64.to_bits(),
            soil_enthalpy_max_normalized_delta_bits: 0.0_f64.to_bits(),
            soil_temperature_max_normalized_delta_bits: 0.0_f64.to_bits(),
            boundary_max_normalized_delta_bits: 0.0_f64.to_bits(),
        });
        assert_eq!(
            take_covered_fixed_point_limiter_audit_v1().total_sample_count,
            0
        );
    }
}
#[cfg(test)]
include!("snow_stage3_v11_attachment_tests.rs");
#[cfg(test)]
#[path = "snow_stage3_v11_parent_chronology_tests.rs"]
mod parent_chronology_tests;
