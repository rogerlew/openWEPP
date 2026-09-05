/// Snow operand presented to the shared covered carrier engine.
///
/// Persistent execution uses the canonical Stage-3 lane map. Terminal trials
/// retain that map only as lineage and replace the target lane's physical
/// bottom/surface operands with the aggregate one-volume trial state.
#[derive(Clone, Debug)]
pub(crate) enum CoveredSnowBoundaryStateV1 {
    TerminalTrial {
        lane_id: u32,
        ice_kg_m2: f64,
        liquid_kg_m2: f64,
        cold_content_j_m2: f64,
        surface_temperature_k: f64,
        depth_m: f64,
        density_kg_m3: f64,
    },
    BatchTerminalTrial {
        lanes: BTreeMap<u32, CoveredTerminalLaneTrialStateV2>,
    },
}

impl CoveredSnowBoundaryStateV1 {
    fn lane_states(
        &self,
        request: &CoveredTerminalTrialRequestV1,
    ) -> BTreeMap<u32, CoveredTerminalLaneTrialStateV2> {
        match self {
            Self::TerminalTrial {
                lane_id,
                ice_kg_m2,
                liquid_kg_m2,
                cold_content_j_m2,
                surface_temperature_k,
                depth_m,
                density_kg_m3,
            } => BTreeMap::from([(
                *lane_id,
                CoveredTerminalLaneTrialStateV2 {
                    lane_id: *lane_id,
                    schema_version: request.beginning_stage3_state.schema_version,
                    terminal_event_model: request.beginning_stage3_state.terminal_event_model,
                    next_interval_index: request.beginning_stage3_state.next_interval_index,
                    snow_density_model: crate::SnowDensityModel::LegacyWepp,
                    ice_kg_m2: *ice_kg_m2,
                    liquid_kg_m2: *liquid_kg_m2,
                    cold_content_j_m2: *cold_content_j_m2,
                    surface_temperature_c: *surface_temperature_k - 273.15,
                    snow_depth_m: *depth_m,
                    snow_density_kg_m3: *density_kg_m3,
                    layer_density_kg_m3: request
                        .beginning_stage3_state
                        .layers
                        .iter()
                        .map(|layer| layer.density_kg_m3)
                        .collect(),
                    layer_settle_day_count: request
                        .beginning_stage3_state
                        .layers
                        .iter()
                        .map(|layer| layer.settle_day_count)
                        .collect(),
                    represented_layers: request.beginning_stage3_state.layers.clone(),
                    resolved_beginning: crate::hydrology::stage3_is_resolved_thermal_domain(
                        &request.beginning_stage3_state,
                    ),
                    candidate_event_tick: None,
                },
            )]),
            Self::BatchTerminalTrial { lanes } => lanes.clone(),
        }
    }

    fn apply_to_boundary_sets(
        &self,
        bindings: &[crate::direct_runtime::DirectSurfaceLiquidOfeBinding],
        covered_boundaries: &mut BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        open_boundaries: &mut BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
    ) -> Result<(), DirectV11RealConsumerError> {
        self.apply_to_boundary_sets_with_topology(
            bindings,
            covered_boundaries,
            open_boundaries,
            true,
        )
    }

    fn apply_to_boundary_sets_with_topology(
        &self,
        bindings: &[crate::direct_runtime::DirectSurfaceLiquidOfeBinding],
        covered_boundaries: &mut BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        open_boundaries: &mut BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        require_every_lane: bool,
    ) -> Result<(), DirectV11RealConsumerError> {
        let lanes = match self {
            Self::TerminalTrial {
                lane_id,
                ice_kg_m2,
                liquid_kg_m2,
                cold_content_j_m2,
                surface_temperature_k,
                depth_m,
                density_kg_m3,
            } => BTreeMap::from([(
                *lane_id,
                CoveredTerminalLaneTrialStateV2 {
                    lane_id: *lane_id,
                    schema_version: 1,
                    terminal_event_model: None,
                    next_interval_index: 0,
                    snow_density_model: crate::SnowDensityModel::LegacyWepp,
                    ice_kg_m2: *ice_kg_m2,
                    liquid_kg_m2: *liquid_kg_m2,
                    cold_content_j_m2: *cold_content_j_m2,
                    surface_temperature_c: *surface_temperature_k - 273.15,
                    snow_depth_m: *depth_m,
                    snow_density_kg_m3: *density_kg_m3,
                    layer_density_kg_m3: vec![*density_kg_m3],
                    layer_settle_day_count: vec![0.0],
                    represented_layers: Vec::new(),
                    resolved_beginning: false,
                    candidate_event_tick: None,
                },
            )]),
            Self::BatchTerminalTrial { lanes } => lanes.clone(),
        };
        for (lane_id, lane) in lanes {
            let surface_temperature_k = lane.surface_temperature_c + 273.15;
            if lane.lane_id != lane_id
                || !surface_temperature_k.is_finite()
                || surface_temperature_k <= 0.0
                || !lane.ice_kg_m2.is_finite()
                || lane.ice_kg_m2 < 0.0
                || !lane.liquid_kg_m2.is_finite()
                || lane.liquid_kg_m2 < 0.0
                || !lane.cold_content_j_m2.is_finite()
                || lane.cold_content_j_m2 < 0.0
                || !lane.snow_depth_m.is_finite()
                || lane.snow_depth_m < 0.0
                || !lane.snow_density_kg_m3.is_finite()
                || lane.snow_density_kg_m3 <= 0.0
                || (lane.ice_kg_m2 - lane.snow_density_kg_m3 * lane.snow_depth_m).abs() > 1.0e-9
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered terminal trial snow boundary state",
                ));
            }
            let surface_temperature = TemperatureCelsius::try_new(lane.surface_temperature_c)
                .map_err(|_| {
                    DirectV11RealConsumerError::Identity(
                        "covered terminal trial snow boundary temperature",
                    )
                })?;
            let latent_heat_j_kg =
                openwepp_meteorology::surface_energy::latent_heat_for_surface_temperature(
                    surface_temperature,
                )
                .map_err(|_| {
                    DirectV11RealConsumerError::Identity(
                        "covered terminal trial snow boundary latent heat",
                    )
                })?
                .as_joules_per_kilogram();
            let mut matched = false;
            for boundaries in [&mut *covered_boundaries, &mut *open_boundaries] {
                for (destination, boundary) in boundaries {
                    if bindings.iter().any(|binding| {
                        binding.ofe_id == destination.0 && binding.production_lane_id == lane_id
                    }) {
                        matched = true;
                        boundary.snow_temperature_k = surface_temperature_k;
                        boundary.latent_heat_j_kg = latent_heat_j_kg;
                    }
                }
            }
            if require_every_lane && !matched {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered terminal trial lane topology",
                ));
            }
        }
        Ok(())
    }

    fn project_trial_stage3_states(
        &self,
        request: &CoveredTerminalTrialRequestV1,
        beginning: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    ) -> Result<BTreeMap<u32, DirectSnowStage3PersistentState>, DirectV11RealConsumerError> {
        let mut projected = beginning.clone();
        for (lane_id, lane) in self.lane_states(request) {
            let state = projected
                .get_mut(&lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered terminal trial Stage-3 lane",
                ))?;
            state.layers = lane.represented_layers.clone();
            if lane.layer_density_kg_m3.len() != state.layers.len()
                || lane.layer_settle_day_count.len() != state.layers.len()
                || lane
                    .layer_density_kg_m3
                    .iter()
                    .any(|density| !density.is_finite() || *density <= 0.0)
                || lane
                    .layer_settle_day_count
                    .iter()
                    .any(|settle| !settle.is_finite() || *settle < 0.0)
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered terminal trial represented-layer coordinates",
                ));
            }
            if lane.ice_kg_m2 > 0.0 {
                let beginning_ice_m = state
                    .layers
                    .iter()
                    .map(|layer| layer.mass_swe_m)
                    .sum::<f64>();
                if beginning_ice_m <= 0.0 || state.layers.is_empty() {
                    return Err(DirectV11RealConsumerError::Identity(
                        "covered terminal trial represented-layer ice topology",
                    ));
                }
                let beginning_liquid_m = state
                    .layers
                    .iter()
                    .map(|layer| layer.liquid_water_m)
                    .sum::<f64>();
                let beginning_cold = state
                    .layers
                    .iter()
                    .map(|layer| layer.cold_content_j_m2)
                    .sum::<f64>();
                let target_ice_m = lane.ice_kg_m2 / 1_000.0;
                let target_liquid_m = lane.liquid_kg_m2 / 1_000.0;
                for (index, layer) in state.layers.iter_mut().enumerate() {
                    let ice_fraction = layer.mass_swe_m / beginning_ice_m;
                    let liquid_fraction = if beginning_liquid_m > 0.0 {
                        layer.liquid_water_m / beginning_liquid_m
                    } else if index == 0 {
                        1.0
                    } else {
                        0.0
                    };
                    let cold_fraction = if beginning_cold > 0.0 {
                        layer.cold_content_j_m2 / beginning_cold
                    } else if index == 0 {
                        1.0
                    } else {
                        0.0
                    };
                    layer.mass_swe_m = target_ice_m * ice_fraction;
                    layer.density_kg_m3 = lane.layer_density_kg_m3[index];
                    layer.thickness_m = layer.mass_swe_m * 1_000.0 / layer.density_kg_m3;
                    layer.settle_day_count = lane.layer_settle_day_count[index];
                    layer.temperature_c = lane.surface_temperature_c;
                    layer.liquid_water_m = target_liquid_m * liquid_fraction;
                    layer.cold_content_j_m2 = lane.cold_content_j_m2 * cold_fraction;
                }
            } else {
                state.layers.clear();
            }
            state.detached_retained_liquid_kg_m2 = if lane.ice_kg_m2 > 0.0 {
                0.0
            } else {
                lane.liquid_kg_m2
            };
            state.fingerprint = Wb11HydrologyKernel::stage3_persistent_state_fingerprint(state);
            Wb11HydrologyKernel::validate_stage3_persistent_state(state).map_err(|_| {
                DirectV11RealConsumerError::Identity(
                    "covered terminal trial projected Stage-3 state",
                )
            })?;
        }
        Ok(projected)
    }
}
