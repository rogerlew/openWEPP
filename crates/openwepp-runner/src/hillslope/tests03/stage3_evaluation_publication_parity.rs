#[test]
fn stage3_v11_publication_uses_only_the_atomic_support_stream() {
    let runner = include_str!("../05_runner_execution_and_outputs.rs");
    let execution = runner
        .split("fn execute_direct_publication_stream(")
        .nth(1)
        .and_then(|tail| tail.split("\nfn ").next())
        .expect("direct production execution body");

    for required in [
        "run_atomic_publication_stream_with_stage3_day_preparation_and_committed_day_archive",
        "build_stage3_v11_support_inputs",
        "build_stage3_v11_publication_input",
        "stream_sink.observe_row(row)",
    ] {
        assert!(
            execution.contains(required),
            "Stage3 V11 production must retain the atomic support-stream seam: {required}"
        );
    }

    for retired in [
        "compute_direct_snow_liquid_partition_from_typed",
        "compute_direct_snow_liquid_partition_with_evaluation",
        "compute_direct_snow_liquid_partition_with_capture_and_reconciliation",
        ".snow_liquid_partition(",
        "SnowStage3EvaluationOperator",
        "PersistentAccumulationShadowV1",
        "stage3_persistent",
    ] {
        assert!(
            !execution.contains(retired),
            "Stage3 V11 production must not re-enter retired day evaluation/persistence: {retired}"
        );
    }
}
