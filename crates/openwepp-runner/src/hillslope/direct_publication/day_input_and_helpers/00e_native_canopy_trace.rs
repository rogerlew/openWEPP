#[derive(Clone, Copy, Debug)]
struct NativeCanopyBuilderTrace {
    day_index: usize,
    lane_index: usize,
    year: i32,
    month: i32,
    day_of_month: i32,
    daily: openwepp_plant_phenology::ForestCanopyDailyResult,
    leaf_litter_input_kg_m2: f64,
    needle_litter_input_kg_m2: Option<f64>,
    fine_woody_litter_input_kg_m2: Option<f64>,
    needle_litter_status: &'static str,
    needle_litter_source_mode: &'static str,
    fine_woody_litter_status: &'static str,
    fine_woody_litter_source_mode: &'static str,
    litter_source_completeness: &'static str,
    #[cfg(test)]
    canopy: openwepp_plant_phenology::ForestCanopyRealization,
    snow_canopy_cover_fraction: f64,
    interception_inputs: DirectCanopyInterceptionInputs,
    #[cfg(test)]
    interception_state: openwepp_hillslope_orchestrator::DirectCanopyInterceptionState,
    #[cfg(test)]
    projected_surface_residue_kg_m2: f64,
    #[cfg(test)]
    projected_residue_depth_m: f64,
    #[cfg(test)]
    frost_residue_depth_m: Option<f64>,
    #[cfg(test)]
    frost_canopy_height_m: Option<f64>,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug)]
struct NativeCanopyConsumerTrace {
    builder: NativeCanopyBuilderTrace,
    growth_state_after: DirectGrowthStateSurface,
    et_leaf_area_index: f64,
    et_canopy_cover_fraction: f64,
    et_canopy_height_m: f64,
    erosion_canopy_height_m: Option<f64>,
    interception_m: f64,
    decomposition_litter_kg_m2: f64,
    decomposition_surface_residue_kg_m2: f64,
    decomposition_residue_depth_m: f64,
    frost_residue_depth_m_consumed: Option<f64>,
    frost_canopy_height_m_consumed: Option<f64>,
    erosion_canopy_cover_fraction: Option<f64>,
    laned_active_canopy_height_m_consumed: Option<f64>,
    laned_shadow_canopy_height_m_consumed: f64,
}

#[cfg(test)]
fn native_canopy_builder_traces() -> &'static std::sync::Mutex<Vec<NativeCanopyBuilderTrace>> {
    static TRACES: std::sync::OnceLock<std::sync::Mutex<Vec<NativeCanopyBuilderTrace>>> =
        std::sync::OnceLock::new();
    TRACES.get_or_init(|| std::sync::Mutex::new(Vec::new()))
}

#[cfg(test)]
fn native_canopy_consumer_traces() -> &'static std::sync::Mutex<Vec<NativeCanopyConsumerTrace>> {
    static TRACES: std::sync::OnceLock<std::sync::Mutex<Vec<NativeCanopyConsumerTrace>>> =
        std::sync::OnceLock::new();
    TRACES.get_or_init(|| std::sync::Mutex::new(Vec::new()))
}

#[cfg(test)]
fn reset_native_canopy_runtime_traces() {
    native_canopy_builder_traces()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .clear();
    native_canopy_consumer_traces()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .clear();
}

#[cfg(test)]
fn record_native_canopy_builder_trace(trace: &NativeCanopyBuilderTrace) {
    native_canopy_builder_traces()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .push(*trace);
}

#[cfg(test)]
fn record_native_canopy_consumer_trace(
    day_frame: &openwepp_hillslope_orchestrator::DirectDayFrame,
) {
    let builder = {
        let traces = native_canopy_builder_traces()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        traces
            .iter()
            .find(|trace| {
                trace.day_index == day_frame.day_index && trace.lane_index == day_frame.lane_index
            })
            .copied()
    };
    let Some(builder) = builder else {
        return;
    };
    let growth_state_after = if day_frame.perennial_growth_inputs.active_context.is_active() {
        day_frame.perennial_growth.state_after
    } else {
        day_frame.annual_growth.state_after
    };
    let laned_shadow_canopy_height_m_consumed = build_laned_shadow_lane_day_operands(
        day_frame.lane_index,
        day_frame.day_index,
        day_frame.wb14_hourly_rainfall_m,
        *day_frame
            .snow_coupling_downstream_operands
            .hourly_routed_melt_m,
        day_frame.evapotranspiration_compute_inputs.leaf_area_index,
        Some(day_frame.evapotranspiration_compute_inputs.canopy_height_m),
    )
    .expect("native canopy trace must pass the real Lane D shadow operand seam")
    .canopy_height_m;
    native_canopy_consumer_traces()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .push(NativeCanopyConsumerTrace {
            builder,
            growth_state_after,
            et_leaf_area_index: day_frame.evapotranspiration_compute_inputs.leaf_area_index,
            et_canopy_cover_fraction: day_frame
                .evapotranspiration_compute_inputs
                .canopy_cover_fraction,
            et_canopy_height_m: day_frame.evapotranspiration_compute_inputs.canopy_height_m,
            erosion_canopy_height_m: day_frame
                .erosion_daily_consumers
                .map(|consumers| consumers.canopy_height_m),
            interception_m: day_frame.interception_m,
            decomposition_litter_kg_m2: day_frame.decomposition_inputs.surface_litter_input_kg_m2,
            decomposition_surface_residue_kg_m2: day_frame.decomposition.surface_residue_kg_m2,
            decomposition_residue_depth_m: day_frame.decomposition.residue_depth_m,
            frost_residue_depth_m_consumed: day_frame
                .frost_daily_consumers
                .map(|consumers| consumers.residue_depth_m),
            frost_canopy_height_m_consumed: day_frame
                .frost_daily_consumers
                .map(|consumers| consumers.canopy_height_m),
            erosion_canopy_cover_fraction: day_frame
                .erosion_daily_consumers
                .map(|consumers| consumers.canopy_cover_fraction),
            laned_active_canopy_height_m_consumed: day_frame
                .laned_active_routing
                .as_ref()
                .and_then(|routing| routing.canopy_height_m_consumed),
            laned_shadow_canopy_height_m_consumed,
        });
}

#[cfg(test)]
fn take_native_canopy_consumer_traces() -> Vec<NativeCanopyConsumerTrace> {
    std::mem::take(
        &mut *native_canopy_consumer_traces()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner),
    )
}
