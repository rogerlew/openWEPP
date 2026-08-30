#[derive(Debug, Error)]
pub enum DirectSnowStage3V11AttachmentError {
    #[error("Stage-3/V11 attachment identity failure: {0}")]
    Identity(&'static str),
    #[error(
        "Stage-3/V11 qualification ordered {vector} record {failure}: digest={digest:?}, first_index={first_index}, duplicate_index={duplicate_index:?}, source_support_receipt={source_support_receipt_sha256:?}, source_receipt={source_receipt_sha256:?}"
    )]
    QualificationOrderedRecordIdentity {
        vector: &'static str,
        failure: &'static str,
        digest: Digest32,
        first_index: usize,
        duplicate_index: Option<usize>,
        source_support_receipt_sha256: Option<Digest32>,
        source_receipt_sha256: Option<Digest32>,
    },
    #[error("Stage-3/V11 attachment support failure: {0}")]
    Support(&'static str),
    #[error("Stage-3/V11 attachment terminal candidate failure: {0}")]
    Terminal(&'static str),
    #[error("Stage-3/V11 adaptive candidate requires refinement: {0}")]
    AdaptiveRefinement(&'static str),
    #[error(
        "Stage-3/V11 adaptive {phase} trial failure at {start_ns}..{end_ns} ns ({duration_ns} ns): {source}"
    )]
    AdaptiveTrial {
        phase: &'static str,
        start_ns: u128,
        end_ns: u128,
        duration_ns: u128,
        #[source]
        source: Box<DirectSnowStage3V11AttachmentError>,
    },
    #[error("Stage-3/V11 diagnostic completed-parent telemetry stop")]
    AdaptiveTelemetryStop,
    #[error("SNOWENERGY-E-PRECIP-001: {0}")]
    Precipitation(&'static str),
    #[error("SNOWENERGY-E-SOIL-HEAT-001: {0}")]
    SnowSoilHeat(&'static str),
    #[error(transparent)]
    Stage3(#[from] crate::hydrology::DirectSnowStage3EvaluationError),
    #[error(transparent)]
    CoupledTime(#[from] openwepp_coupled_time::CoupledTimeError),
    #[error(transparent)]
    Owner(#[from] DirectV11RealConsumerError),
    #[error(transparent)]
    V11(#[from] openwepp_vegetation::v11::V11ExecutionError<DirectV11RealConsumerError>),
    #[error(transparent)]
    V11Authority(#[from] openwepp_vegetation::v11::V11Error),
    #[error(transparent)]
    ForcingProvider(#[from] SnowFreeHalfHourForcingError),
    #[error(transparent)]
    SnowBoundary(#[from] SnowStage3HandoffError),
}
