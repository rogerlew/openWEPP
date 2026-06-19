use std::io::Write as _;

const PERFDEEP02_FRAME_ROUNDTRIP_PATH_ENV: &str = "OPENWEPP_PERFDEEP02_FRAME_ROUNDTRIP_PATH";

#[derive(Debug, Serialize)]
struct Perfdeep02FrameRoundtripRow {
    schema: &'static str,
    stage: &'static str,
    run_name: String,
    execution_lane: String,
    simulation_year: i32,
    sim_day_index: usize,
    calendar_year: i32,
    calendar_julian_day: u16,
    ofe_id: Option<usize>,
    state_symbol_count: usize,
    flux_symbol_count: usize,
    state_mismatch_count: usize,
    flux_mismatch_count: usize,
    first_mismatch_surface: Option<&'static str>,
    first_mismatch_symbol: Option<String>,
    first_mismatch_expected_bits: Option<u64>,
    first_mismatch_observed_bits: Option<u64>,
    first_mismatch_expected_unit: Option<&'static str>,
    first_mismatch_observed_unit: Option<&'static str>,
    mofe_upstream_saturation_present: usize,
    mofe_current_saturation_present: usize,
    mofe_upstream_lateral_present: usize,
    mofe_current_lateral_present: usize,
}

fn maybe_record_perfdeep02_frame_roundtrip(
    stage: &'static str,
    ofe_id: Option<usize>,
    surface: &HillslopeWritebackSurface,
    context: &SchedulerLifecycleContext<'_>,
) -> Result<(), HillslopeCliError> {
    let Some(path) = std::env::var_os(PERFDEEP02_FRAME_ROUNDTRIP_PATH_ENV).map(PathBuf::from)
    else {
        return Ok(());
    };

    let Some(symbol_registry) = context.symbol_registry else {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "perfdeep02_frame_roundtrip",
            detail: format!(
                "{SIMPIPE_GUARD_ID} frame roundtrip report requested without an active frame registry"
            ),
        });
    };
    let frame = HillslopeDayFrame::seed_from_writeback_surface(surface, symbol_registry, None)
        .map_err(|source| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "perfdeep02_frame_roundtrip",
            detail: source.to_string(),
        })?;
    let report = frame
        .assert_shadow_roundtrip_bits(&surface.state_surface, &surface.flux_surface)
        .map_err(|source| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "perfdeep02_frame_roundtrip",
            detail: source.to_string(),
        })?;
    let first_mismatch = report.first_mismatch.as_ref();
    let row = Perfdeep02FrameRoundtripRow {
        schema: "openwepp-perfdeep02-frame-roundtrip-v1",
        stage,
        run_name: context.run_name.to_string(),
        execution_lane: context.execution_lane.as_str().to_string(),
        simulation_year: context.simulation_year,
        sim_day_index: context.sim_day_index,
        calendar_year: context.calendar_day.year,
        calendar_julian_day: context.calendar_day.julian_day,
        ofe_id,
        state_symbol_count: report.state_symbol_count,
        flux_symbol_count: report.flux_symbol_count,
        state_mismatch_count: report.state_mismatch_count,
        flux_mismatch_count: report.flux_mismatch_count,
        first_mismatch_surface: first_mismatch.map(|mismatch| mismatch.surface),
        first_mismatch_symbol: first_mismatch.map(|mismatch| mismatch.symbol.as_str().to_string()),
        first_mismatch_expected_bits: first_mismatch.map(|mismatch| mismatch.expected_bits),
        first_mismatch_observed_bits: first_mismatch.map(|mismatch| mismatch.observed_bits),
        first_mismatch_expected_unit: first_mismatch.map(|mismatch| mismatch.expected_unit),
        first_mismatch_observed_unit: first_mismatch.map(|mismatch| mismatch.observed_unit),
        mofe_upstream_saturation_present: frame
            .mofe_hourly_upstream_saturation_runoff
            .iter()
            .filter(|value| value.is_some())
            .count(),
        mofe_current_saturation_present: frame
            .mofe_hourly_current_saturation_runoff
            .iter()
            .filter(|value| value.is_some())
            .count(),
        mofe_upstream_lateral_present: frame
            .mofe_hourly_upstream_lateral_runoff
            .iter()
            .filter(|value| value.is_some())
            .count(),
        mofe_current_lateral_present: frame
            .mofe_hourly_current_lateral_runoff
            .iter()
            .filter(|value| value.is_some())
            .count(),
    };
    let payload =
        serde_json::to_string(&row).map_err(|source| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "perfdeep02_frame_roundtrip",
            detail: format!("failed serializing frame roundtrip row: {source}"),
        })?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|source| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "perfdeep02_frame_roundtrip",
            detail: format!("failed creating report directory {}: {source}", parent.display()),
        })?;
    }
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|source| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "perfdeep02_frame_roundtrip",
            detail: format!("failed opening report {}: {source}", path.display()),
        })?;
    writeln!(file, "{payload}").map_err(|source| HillslopeCliError::RuntimeSurfaceFailure {
        surface: "perfdeep02_frame_roundtrip",
        detail: format!("failed writing report {}: {source}", path.display()),
    })
}
