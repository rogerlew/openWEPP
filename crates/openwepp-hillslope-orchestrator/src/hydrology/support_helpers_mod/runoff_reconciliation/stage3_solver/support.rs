#[allow(clippy::wildcard_imports)]
use super::*;

impl Wb11HydrologyKernel {
    #[allow(clippy::too_many_lines)]
    pub fn evaluate_stage3_persistent_day(
        inputs: &DirectActiveSnowPartitionInputs,
        state: &DirectSnowStage3PersistentState,
        lane_id: u32,
        interval_index: u64,
    ) -> Result<DirectSnowStage3PersistentDayResult, DirectSnowStage3EvaluationError> {
        if state.schema_version != 1 {
            return Err(Self::stage3_domain_error(
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                "snow.stage3_persistent_request_state_mismatch",
                f64::from(state.schema_version),
                Some(1.0),
                Some(1.0),
            )
            .into());
        }
        let supports = inputs
            .hourly
            .iter()
            .copied()
            .map(|forcing| DirectSnowStage3SupportInput {
                forcing,
                duration_seconds: STAGE3_SECONDS_PER_HOUR,
            })
            .collect::<Vec<_>>();
        Self::evaluate_stage3_persistent_day_internal(
            inputs,
            state,
            lane_id,
            interval_index,
            &supports,
            None,
        )
    }

    pub fn evaluate_stage3_persistent_day_with_terminal_event(
        inputs: &DirectActiveSnowPartitionInputs,
        state: &DirectSnowStage3PersistentState,
        lane_id: u32,
        interval_index: u64,
        request: DirectSnowTerminalEventRequest,
    ) -> Result<DirectSnowStage3PersistentDayResult, DirectSnowStage3EvaluationError> {
        if state.schema_version != 2 || state.terminal_event_model != Some(request.model) {
            return Err(Self::stage3_domain_error(
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                "snow.stage3_persistent_request_state_mismatch",
                f64::from(state.schema_version),
                Some(2.0),
                Some(2.0),
            )
            .into());
        }
        Self::evaluate_stage3_persistent_day_internal(
            inputs,
            state,
            lane_id,
            interval_index,
            &inputs
                .hourly
                .iter()
                .copied()
                .map(|forcing| DirectSnowStage3SupportInput {
                    forcing,
                    duration_seconds: STAGE3_SECONDS_PER_HOUR,
                })
                .collect::<Vec<_>>(),
            Some(request),
        )
    }

    /// Evaluate one actual coupled-time support with the same sequential and
    /// adaptive terminal equations used by the day wrapper. The support is
    /// not expanded, duplicated, scaled, or interpolated.
    pub fn evaluate_stage3_persistent_support(
        inputs: &DirectActiveSnowPartitionInputs,
        state: &DirectSnowStage3PersistentState,
        lane_id: u32,
        interval_index: u64,
        support: DirectSnowStage3SupportInput,
        request: DirectSnowTerminalEventRequest,
    ) -> Result<DirectSnowStage3PersistentDayResult, DirectSnowStage3EvaluationError> {
        if !support.duration_seconds.is_finite() || support.duration_seconds <= 0.0 {
            return Err(Self::stage3_domain_error(
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                "snow.stage3_support_duration_seconds",
                support.duration_seconds,
                Some(f64::MIN_POSITIVE),
                None,
            )
            .into());
        }
        if state.schema_version != 2 || state.terminal_event_model != Some(request.model) {
            return Err(Self::stage3_domain_error(
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                "snow.stage3_persistent_request_state_mismatch",
                f64::from(state.schema_version),
                Some(2.0),
                Some(2.0),
            )
            .into());
        }
        Self::evaluate_stage3_persistent_day_internal(
            inputs,
            state,
            lane_id,
            interval_index,
            &[support],
            Some(request),
        )
    }

    /// Evaluate one admitted coupled-time support without enabling terminal
    /// event localization. This is the ordinary 1,800-second Stage-3
    /// transition used before a terminal request is introduced by the parent
    /// controller.
    pub fn evaluate_stage3_persistent_support_without_terminal_event(
        inputs: &DirectActiveSnowPartitionInputs,
        state: &DirectSnowStage3PersistentState,
        lane_id: u32,
        interval_index: u64,
        support: DirectSnowStage3SupportInput,
    ) -> Result<DirectSnowStage3PersistentDayResult, DirectSnowStage3EvaluationError> {
        if !support.duration_seconds.is_finite() || support.duration_seconds <= 0.0 {
            return Err(Self::stage3_domain_error(
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                "snow.stage3_support_duration_seconds",
                support.duration_seconds,
                Some(f64::MIN_POSITIVE),
                None,
            )
            .into());
        }
        Self::validate_stage3_persistent_state(state)?;
        if state.lane_id != lane_id || state.next_interval_index != interval_index {
            return Err(Self::stage3_domain_error(
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                "snow.stage3_persistent_identity_or_order",
                1.0,
                Some(0.0),
                Some(0.0),
            )
            .into());
        }
        Self::evaluate_stage3_persistent_day_internal(
            inputs,
            state,
            lane_id,
            interval_index,
            &[support],
            None,
        )
    }
}
