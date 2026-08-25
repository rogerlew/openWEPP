#[allow(clippy::wildcard_imports)]
use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Stage3BottomVolumeProjectionV1 {
    pub(crate) temperature_k: f64,
    pub(crate) thickness_m: f64,
    pub(crate) thermal_conductivity_w_m_k: f64,
    pub(crate) beginning_stage3_state_sha256: openwepp_coupled_time::Digest32,
}

impl Wb11HydrologyKernel {
    /// Project the complete terminal-domain control volume at the soil
    /// boundary. The terminal evaluator collapses the full represented column,
    /// so its surface and bottom are the same canonical volume.
    pub(crate) fn project_stage3_terminal_bottom_volume_v1(
        state: &DirectSnowStage3PersistentState,
        atmospheric_pressure_pa: f64,
    ) -> Result<Stage3BottomVolumeProjectionV1, DirectSnowStage3EvaluationError> {
        Self::validate_stage3_persistent_state(state)?;
        if !stage3_is_terminal_event_domain(state) {
            return Err(Self::stage3_domain_error(
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                "snow.stage3_bottom_volume_terminal_domain",
                stage3_total_represented_ice_swe_m(state),
                Some(0.0),
                Some(STAGE3_MINIMUM_RESOLVED_THERMAL_MASS_SWE_M),
            )
            .into());
        }
        let layers = state
            .layers
            .iter()
            .copied()
            .filter(|layer| snow_density_layer_has_resolved_mass(layer.mass_swe_m))
            .collect::<Vec<_>>();
        let cold_content = layers
            .iter()
            .map(Self::stage3_layer_cold_content_j_m2)
            .collect::<Vec<_>>();
        let volume = Self::stage3_control_volume_state(
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
            &layers,
            &cold_content,
            atmospheric_pressure_pa,
        )?;
        let temperature_c = Self::stage3_temperature_from_cold_content_values(
            volume.mass_swe_m,
            volume.cold_content_j_m2,
        );
        Self::stage3_temperature(
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
            temperature_c,
        )?;
        Ok(Stage3BottomVolumeProjectionV1 {
            temperature_k: temperature_c + 273.15,
            thickness_m: volume.depth_m,
            thermal_conductivity_w_m_k: volume.conductivity_w_m_k,
            beginning_stage3_state_sha256: openwepp_coupled_time::digest_bytes(
                &Self::serialize_stage3_persistent_state(state)?,
            ),
        })
    }

    /// Project the bottom represented thermal volume of a resolved Stage-3 column.
    pub(crate) fn project_stage3_bottom_volume_v1(
        state: &DirectSnowStage3PersistentState,
        atmospheric_pressure_pa: f64,
    ) -> Result<Stage3BottomVolumeProjectionV1, DirectSnowStage3EvaluationError> {
        Self::validate_stage3_persistent_state(state)?;
        let mut layers = state
            .layers
            .iter()
            .copied()
            .filter(|layer| snow_density_layer_has_resolved_mass(layer.mass_swe_m))
            .collect::<Vec<_>>();
        if layers.is_empty() || !stage3_is_resolved_thermal_domain(state) {
            return Err(Self::stage3_domain_error(
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                "snow.stage3_bottom_volume_resolved_domain",
                stage3_total_represented_ice_swe_m(state),
                Some(STAGE3_MINIMUM_RESOLVED_THERMAL_MASS_SWE_M),
                None,
            )
            .into());
        }
        let mut cold_content_by_layer = layers
            .iter()
            .map(Self::stage3_layer_cold_content_j_m2)
            .collect::<Vec<_>>();
        let active_layer_count =
            Self::prepare_stage3_sequential_control_volume(&mut layers, &mut cold_content_by_layer);
        let bottom_start = if active_layer_count < layers.len() {
            active_layer_count
        } else {
            0
        };
        let bottom_layers = &layers[bottom_start..];
        let bottom_cold_content = &cold_content_by_layer[bottom_start..];
        let bottom = Self::stage3_control_volume_state(
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
            bottom_layers,
            bottom_cold_content,
            atmospheric_pressure_pa,
        )?;
        let temperature_c = Self::stage3_temperature_from_cold_content_values(
            bottom.mass_swe_m,
            bottom.cold_content_j_m2,
        );
        Self::stage3_temperature(
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
            temperature_c,
        )?;
        Ok(Stage3BottomVolumeProjectionV1 {
            temperature_k: temperature_c + 273.15,
            thickness_m: bottom.depth_m,
            thermal_conductivity_w_m_k: bottom.conductivity_w_m_k,
            beginning_stage3_state_sha256: openwepp_coupled_time::digest_bytes(
                &Self::serialize_stage3_persistent_state(state)?,
            ),
        })
    }

    /// Project the canonical current Stage-3 active thermal surface state.
    pub fn project_stage3_surface_state_v1(
        state: &DirectSnowStage3PersistentState,
    ) -> Result<Stage3SurfaceStateV1, DirectSnowStage3EvaluationError> {
        Self::validate_stage3_persistent_state(state)?;
        let mut layers = state
            .layers
            .iter()
            .copied()
            .filter(|layer| snow_density_layer_has_resolved_mass(layer.mass_swe_m))
            .collect::<Vec<_>>();
        if layers.is_empty() || !stage3_is_resolved_thermal_domain(state) {
            return Err(Self::stage3_domain_error(
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                "snow.stage3_surface_state_resolved_domain",
                stage3_total_represented_ice_swe_m(state),
                Some(STAGE3_MINIMUM_RESOLVED_THERMAL_MASS_SWE_M),
                None,
            )
            .into());
        }
        let mut cold_content_by_layer = layers
            .iter()
            .map(Self::stage3_layer_cold_content_j_m2)
            .collect::<Vec<_>>();
        let active_layer_count =
            Self::prepare_stage3_sequential_control_volume(&mut layers, &mut cold_content_by_layer);
        let active_mass_swe_m = layers[..active_layer_count]
            .iter()
            .map(|layer| layer.mass_swe_m)
            .sum::<f64>();
        let active_depth_m = layers[..active_layer_count]
            .iter()
            .map(|layer| layer.thickness_m)
            .sum::<f64>();
        let active_cold_content_j_m2 = cold_content_by_layer[..active_layer_count]
            .iter()
            .sum::<f64>();
        let surface_temperature_c = Self::stage3_temperature_from_cold_content_values(
            active_mass_swe_m,
            active_cold_content_j_m2,
        );
        let temperature = Self::stage3_temperature(
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
            surface_temperature_c,
        )?;
        let latent_heat_j_kg = latent_heat_for_surface_temperature(temperature)
            .map_err(|_| {
                Self::stage3_domain_error(
                    HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                    "snow.stage3_surface_state_latent_heat",
                    surface_temperature_c,
                    Some(-273.15),
                    Some(0.0),
                )
            })?
            .as_joules_per_kilogram();
        let beginning_stage3_state_sha256 =
            openwepp_coupled_time::digest_bytes(&Self::serialize_stage3_persistent_state(state)?);
        let mut partition_bytes = Vec::new();
        partition_bytes.extend_from_slice(b"OPENWEPP_STAGE3_ACTIVE_LOWER_PARTITION_V1");
        partition_bytes.extend_from_slice(&(active_layer_count as u64).to_le_bytes());
        for (index, layer) in layers.iter().enumerate() {
            partition_bytes.extend_from_slice(&(index as u64).to_le_bytes());
            partition_bytes.extend_from_slice(&layer.mass_swe_m.to_bits().to_le_bytes());
            partition_bytes.extend_from_slice(&layer.thickness_m.to_bits().to_le_bytes());
            partition_bytes.extend_from_slice(&layer.density_kg_m3.to_bits().to_le_bytes());
            partition_bytes
                .extend_from_slice(&cold_content_by_layer[index].to_bits().to_le_bytes());
        }
        Ok(Stage3SurfaceStateV1 {
            active_mass_kg_m2: active_mass_swe_m * STAGE3_RHO_WATER_KG_M3,
            active_depth_m,
            active_cold_content_j_m2,
            surface_temperature_k: surface_temperature_c + 273.15,
            latent_heat_j_kg,
            selected_substep_seconds: Self::stage3_substep_seconds(&layers, active_layer_count)
                .min(1_800.0),
            active_lower_partition_sha256: openwepp_coupled_time::digest_bytes(&partition_bytes),
            beginning_stage3_state_sha256,
        })
    }

    /// Project the complete one-volume surface admitted only by the terminal
    /// enthalpy-event schema. This is deliberately separate from the resolved
    /// thermal projection so the ordinary Stage-3 domain remains fail-closed.
    pub fn project_stage3_terminal_surface_state_v1(
        state: &DirectSnowStage3PersistentState,
    ) -> Result<Stage3SurfaceStateV1, DirectSnowStage3EvaluationError> {
        Self::validate_stage3_persistent_state(state)?;
        let layers = state
            .layers
            .iter()
            .copied()
            .filter(|layer| snow_density_layer_has_resolved_mass(layer.mass_swe_m))
            .collect::<Vec<_>>();
        if !stage3_is_terminal_event_domain(state) {
            return Err(Self::stage3_domain_error(
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                "snow.stage3_surface_state_terminal_domain",
                stage3_total_represented_ice_swe_m(state),
                Some(0.0),
                Some(STAGE3_MINIMUM_RESOLVED_THERMAL_MASS_SWE_M),
            )
            .into());
        }
        let active_mass_swe_m = layers.iter().map(|layer| layer.mass_swe_m).sum::<f64>();
        let active_depth_m = layers.iter().map(|layer| layer.thickness_m).sum::<f64>();
        let active_cold_content_j_m2 = layers
            .iter()
            .map(Self::stage3_layer_cold_content_j_m2)
            .sum::<f64>();
        let surface_temperature_c = Self::stage3_temperature_from_cold_content_values(
            active_mass_swe_m,
            active_cold_content_j_m2,
        );
        let temperature = Self::stage3_temperature(
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
            surface_temperature_c,
        )?;
        let latent_heat_j_kg = latent_heat_for_surface_temperature(temperature)
            .map_err(|_| {
                Self::stage3_domain_error(
                    HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                    "snow.stage3_terminal_surface_state_latent_heat",
                    surface_temperature_c,
                    Some(-273.15),
                    Some(0.0),
                )
            })?
            .as_joules_per_kilogram();
        let beginning_stage3_state_sha256 =
            openwepp_coupled_time::digest_bytes(&Self::serialize_stage3_persistent_state(state)?);
        let mut partition_bytes = Vec::new();
        partition_bytes.extend_from_slice(b"OPENWEPP_STAGE3_TERMINAL_COMPLETE_VOLUME_V1");
        for (index, layer) in layers.iter().enumerate() {
            partition_bytes.extend_from_slice(&(index as u64).to_le_bytes());
            partition_bytes.extend_from_slice(&layer.mass_swe_m.to_bits().to_le_bytes());
            partition_bytes.extend_from_slice(&layer.thickness_m.to_bits().to_le_bytes());
            partition_bytes.extend_from_slice(&layer.density_kg_m3.to_bits().to_le_bytes());
            partition_bytes.extend_from_slice(
                &Self::stage3_layer_cold_content_j_m2(layer)
                    .to_bits()
                    .to_le_bytes(),
            );
        }
        Ok(Stage3SurfaceStateV1 {
            active_mass_kg_m2: active_mass_swe_m * STAGE3_RHO_WATER_KG_M3,
            active_depth_m,
            active_cold_content_j_m2,
            surface_temperature_k: surface_temperature_c + 273.15,
            latent_heat_j_kg,
            selected_substep_seconds: STAGE3_SMALL_TIMESTEP_SECONDS,
            active_lower_partition_sha256: openwepp_coupled_time::digest_bytes(&partition_bytes),
            beginning_stage3_state_sha256,
        })
    }

    pub fn initialize_stage3_persistent_state(
        lane_id: u32,
        layers: Vec<DirectSnowLayerState>,
    ) -> Result<DirectSnowStage3PersistentState, DirectSnowStage3EvaluationError> {
        Self::initialize_stage3_persistent_state_with_retained_liquid(lane_id, layers, 0.0)
    }

    pub fn initialize_stage3_persistent_state_with_terminal_event(
        lane_id: u32,
        layers: Vec<DirectSnowLayerState>,
        request: DirectSnowTerminalEventRequest,
    ) -> Result<DirectSnowStage3PersistentState, DirectSnowStage3EvaluationError> {
        Self::initialize_stage3_persistent_state_with_retained_liquid_and_terminal_event(
            lane_id, layers, 0.0, request,
        )
    }

    pub fn initialize_stage3_persistent_state_with_retained_liquid_and_terminal_event(
        lane_id: u32,
        layers: Vec<DirectSnowLayerState>,
        detached_retained_liquid_kg_m2: f64,
        request: DirectSnowTerminalEventRequest,
    ) -> Result<DirectSnowStage3PersistentState, DirectSnowStage3EvaluationError> {
        let mut state = Self::initialize_stage3_persistent_state_with_retained_liquid(
            lane_id,
            layers,
            detached_retained_liquid_kg_m2,
        )?;
        state.schema_version = 2;
        state.terminal_event_model = Some(request.model);
        state.fingerprint = Self::stage3_persistent_state_fingerprint(&state);
        Self::validate_stage3_persistent_state(&state)?;
        Ok(state)
    }

    pub fn initialize_stage3_persistent_state_with_retained_liquid(
        lane_id: u32,
        layers: Vec<DirectSnowLayerState>,
        detached_retained_liquid_kg_m2: f64,
    ) -> Result<DirectSnowStage3PersistentState, DirectSnowStage3EvaluationError> {
        let initial_ice_kg_m2 = Self::stage3_total_ice_mass_swe_m(&layers) * STAGE3_RHO_WATER_KG_M3;
        let initial_retained_liquid_kg_m2 = layers
            .iter()
            .map(|layer| layer.liquid_water_m * STAGE3_RHO_WATER_KG_M3)
            .sum::<f64>()
            + detached_retained_liquid_kg_m2;
        let mut state = DirectSnowStage3PersistentState {
            schema_version: 1,
            terminal_event_model: None,
            fingerprint: 0,
            lane_id,
            next_interval_index: 0,
            layers,
            detached_retained_liquid_kg_m2,
            initial_ice_kg_m2,
            initial_retained_liquid_kg_m2,
            cumulative_snowfall_kg_m2: 0.0,
            cumulative_external_liquid_kg_m2: 0.0,
            cumulative_deposition_kg_m2: 0.0,
            cumulative_sublimation_kg_m2: 0.0,
            cumulative_melt_kg_m2: 0.0,
            cumulative_unresolved_liquid_kg_m2: 0.0,
            cumulative_complete_energy_j_m2: 0.0,
            cumulative_cold_energy_change_j_m2: 0.0,
            cumulative_terminal_unallocated_energy_j_m2: 0.0,
        };
        state.fingerprint = Self::stage3_persistent_state_fingerprint(&state);
        Self::validate_stage3_persistent_state(&state)?;
        Ok(state)
    }

    pub fn restore_stage3_persistent_state(
        snapshot: DirectSnowStage3PersistentState,
        lane_id: u32,
        next_interval_index: u64,
    ) -> Result<DirectSnowStage3PersistentState, DirectSnowStage3EvaluationError> {
        Self::validate_stage3_persistent_state(&snapshot)?;
        if snapshot.lane_id != lane_id || snapshot.next_interval_index != next_interval_index {
            return Err(Self::stage3_domain_error(
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                "snow.stage3_persistent_restore_identity_or_order",
                1.0,
                Some(0.0),
                Some(0.0),
            )
            .into());
        }
        Ok(snapshot)
    }

    pub fn serialize_stage3_persistent_state(
        state: &DirectSnowStage3PersistentState,
    ) -> Result<Vec<u8>, DirectSnowStage3EvaluationError> {
        Self::validate_stage3_persistent_state(state)?;
        serde_json::to_vec(state).map_err(|_| {
            Self::stage3_domain_error(
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                "snow.stage3_persistent_snapshot_serialize",
                1.0,
                Some(0.0),
                Some(0.0),
            )
            .into()
        })
    }

    pub fn restore_stage3_persistent_state_json(
        snapshot: &[u8],
        lane_id: u32,
        next_interval_index: u64,
    ) -> Result<DirectSnowStage3PersistentState, DirectSnowStage3EvaluationError> {
        let state: DirectSnowStage3PersistentState =
            serde_json::from_slice(snapshot).map_err(|_| {
                Self::stage3_domain_error(
                    HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                    "snow.stage3_persistent_snapshot_deserialize",
                    1.0,
                    Some(0.0),
                    Some(0.0),
                )
            })?;
        Self::restore_stage3_persistent_state(state, lane_id, next_interval_index)
    }
}
