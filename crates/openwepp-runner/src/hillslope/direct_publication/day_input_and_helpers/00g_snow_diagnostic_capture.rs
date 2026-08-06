#[derive(Clone, Debug, PartialEq, Eq)]
struct DirectSnowDiagnosticCaptureRequest {
    capture: openwepp_hillslope_orchestrator::DirectSnowDiagnosticCapture,
    selected_path: Option<std::ffi::OsString>,
}

#[derive(Clone, Copy)]
struct DirectSnowTraceRowContext<'a> {
    day_index: usize,
    lane_index: usize,
    hyetograph_rainfall_m: f64,
    snow_lane_state: &'a openwepp_hillslope_orchestrator::DirectSnowLaneState,
    snow_melt_model: openwepp_hillslope_orchestrator::SnowMeltModel,
    snow_phase_model: openwepp_hillslope_orchestrator::SnowPhasePartitionModel,
    snow_liquid: &'a openwepp_hillslope_orchestrator::DirectSnowLiquidPartition,
    stage3_evaluation:
        Option<&'a openwepp_hillslope_orchestrator::DirectSnowStage3EvaluationDiagnostics>,
}

impl DirectSnowDiagnosticCaptureRequest {
    fn from_values(
        path: Option<std::ffi::OsString>,
        day_filter: Option<usize>,
        lane_filter: Option<usize>,
        day_index: usize,
        lane_index: usize,
    ) -> Self {
        let selected_path = path.filter(|path| {
            !path.is_empty()
                && day_filter.is_none_or(|filter| filter == day_index)
                && lane_filter.is_none_or(|filter| filter == lane_index)
        });
        let capture = if selected_path.is_some() {
            openwepp_hillslope_orchestrator::DirectSnowDiagnosticCapture::Verbose
        } else {
            openwepp_hillslope_orchestrator::DirectSnowDiagnosticCapture::Disabled
        };
        Self {
            capture,
            selected_path,
        }
    }

    fn resolve(day_index: usize, lane_index: usize) -> Self {
        Self::from_values(
            std::env::var_os("OPENWEPP_R7H_SNOW_TRACE_PATH"),
            direct_production_trace_env_usize("OPENWEPP_R7H_SNOW_TRACE_DAY_INDEX"),
            direct_production_trace_env_usize("OPENWEPP_R7H_SNOW_TRACE_LANE_INDEX"),
            day_index,
            lane_index,
        )
    }
}

fn selected_snow_verbose_diagnostics<'a>(
    request: &DirectSnowDiagnosticCaptureRequest,
    snow_liquid: &'a openwepp_hillslope_orchestrator::DirectSnowLiquidPartition,
) -> Result<Option<&'a openwepp_hillslope_orchestrator::DirectSnowVerboseDiagnostics>, HillslopeCliError>
{
    if request.selected_path.is_none() {
        return Ok(None);
    }
    snow_liquid
        .verbose_diagnostics
        .as_deref()
        .map(Some)
        .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_snow_trace",
            detail: format!(
                "{SIMOUT_GUARD_ID} selected direct production snow trace row is missing its verbose diagnostic payload"
            ),
        })
}

#[cfg(test)]
mod snow_diagnostic_capture_tests {
    use super::*;

    #[test]
    fn absent_empty_and_filtered_requests_disable_capture() {
        for request in [
            DirectSnowDiagnosticCaptureRequest::from_values(None, None, None, 7, 3),
            DirectSnowDiagnosticCaptureRequest::from_values(
                Some(std::ffi::OsString::new()),
                None,
                None,
                7,
                3,
            ),
            DirectSnowDiagnosticCaptureRequest::from_values(
                Some(std::ffi::OsString::from("trace.jsonl")),
                Some(8),
                None,
                7,
                3,
            ),
            DirectSnowDiagnosticCaptureRequest::from_values(
                Some(std::ffi::OsString::from("trace.jsonl")),
                None,
                Some(4),
                7,
                3,
            ),
        ] {
            assert_eq!(
                request.capture,
                openwepp_hillslope_orchestrator::DirectSnowDiagnosticCapture::Disabled
            );
            assert!(request.selected_path.is_none());
        }
    }

    #[test]
    fn selected_request_requires_verbose_payload_before_writer_io() {
        let request = DirectSnowDiagnosticCaptureRequest::from_values(
            Some(std::ffi::OsString::from("trace.jsonl")),
            Some(7),
            Some(3),
            7,
            3,
        );
        let partition = inactive_direct_snow_liquid_partition(
            openwepp_hillslope_orchestrator::SnowDensityModel::LegacyWepp,
            0.0,
            &openwepp_hillslope_orchestrator::DirectSnowLaneState::zero(),
            openwepp_hillslope_orchestrator::DirectSnowDiagnosticCapture::Disabled,
        );
        let error = selected_snow_verbose_diagnostics(&request, &partition)
            .expect_err("selected writer must reject a missing verbose payload");
        assert!(matches!(
            error,
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_production_snow_trace",
                ..
            }
        ));
    }
}
