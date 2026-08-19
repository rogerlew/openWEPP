use std::{fs, path::Path};

use openwepp_persisted_restart_v1::{
    DirectV10CheckpointPhaseV1, DirectV10RealConsumerCheckpointV1, from_canonical_bytes,
};

#[test]
fn production_checkpoint_contains_complete_typed_direct_hydrology_owner() {
    let bytes = fs::read(Path::new(
        "docs/work-packages/20260817-direct-hydrology-persisted-restart-authority-001/artifacts/checkpoint-in-progress-vector.json",
    ))
    .unwrap();
    let checkpoint: DirectV10RealConsumerCheckpointV1 = from_canonical_bytes(&bytes).unwrap();
    let DirectV10CheckpointPhaseV1::InProgressDay {
        committed_day_beginning,
        staged_scientific,
        ..
    } = checkpoint.phase
    else {
        panic!("released interval-24 vector must be in progress")
    };
    assert!(
        !committed_day_beginning
            .scientific
            .direct_hydrology
            .lanes
            .is_empty()
    );
    assert!(!staged_scientific.direct_hydrology.lanes.is_empty());
    assert_eq!(
        committed_day_beginning
            .scientific
            .direct_hydrology
            .lane_count,
        staged_scientific.direct_hydrology.lane_count
    );
}
