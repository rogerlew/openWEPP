mod restart_adaptive_trial_grid_tests {
    use super::*;

    const MINIMUM_SUPPORT_NS: u128 = 60_000_000_000;
    const PARENT_END_NS: u128 = 1_800_000_000_000;

    fn parent_support() -> TimeSupport {
        TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(PARENT_END_NS)).unwrap()
    }

    #[test]
    fn exact_floor_count_and_cursor_cross_join_rejects_poisons() {
        let parent = parent_support();
        let cursor = ModelTimeNs::new(600_000_000_000);
        validate_restart_adaptive_trial_grid_v2(
            DirectSnowStage3V11InterruptionPostureV2::AdaptiveMicrostepBoundary,
            20,
            MINIMUM_SUPPORT_NS,
            parent,
            cursor,
        )
        .unwrap();

        // Count poison.
        assert!(
            validate_restart_adaptive_trial_grid_v2(
                DirectSnowStage3V11InterruptionPostureV2::AdaptiveMicrostepBoundary,
                0,
                MINIMUM_SUPPORT_NS,
                parent,
                cursor,
            )
            .is_err()
        );
        // Minimum-support substitution poison: the previous 600-ms authority
        // cannot be interpreted as a count on the exact 60-second grid.
        assert!(
            validate_restart_adaptive_trial_grid_v2(
                DirectSnowStage3V11InterruptionPostureV2::AdaptiveMicrostepBoundary,
                20,
                600_000_000,
                parent,
                cursor,
            )
            .is_err()
        );
        // Cursor divisibility poison.
        assert!(
            validate_restart_adaptive_trial_grid_v2(
                DirectSnowStage3V11InterruptionPostureV2::AdaptiveMicrostepBoundary,
                20,
                MINIMUM_SUPPORT_NS,
                parent,
                ModelTimeNs::new(cursor.get() + 1),
            )
            .is_err()
        );
        // Proposal range poison: 21 quanta do not fit the 20-quanta parent
        // remainder at this cursor.
        assert!(
            validate_restart_adaptive_trial_grid_v2(
                DirectSnowStage3V11InterruptionPostureV2::AdaptiveMicrostepBoundary,
                21,
                MINIMUM_SUPPORT_NS,
                parent,
                cursor,
            )
            .is_err()
        );
        // Parent range poison remains invalid even for a posture retaining the
        // just-executed proposal rather than a next proposal.
        assert!(
            validate_restart_adaptive_trial_grid_v2(
                DirectSnowStage3V11InterruptionPostureV2::AfterTerminalEvent,
                31,
                MINIMUM_SUPPORT_NS,
                parent,
                cursor,
            )
            .is_err()
        );
    }

    #[test]
    fn endpoint_event_posture_retains_nonzero_completed_trial_count() {
        validate_restart_adaptive_trial_grid_v2(
            DirectSnowStage3V11InterruptionPostureV2::AfterTerminalReceiver,
            30,
            MINIMUM_SUPPORT_NS,
            parent_support(),
            ModelTimeNs::new(PARENT_END_NS),
        )
        .unwrap();
    }
}
