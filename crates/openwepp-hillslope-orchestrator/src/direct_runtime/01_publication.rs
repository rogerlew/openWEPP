#[derive(Debug, Clone, PartialEq)]
pub struct DirectPublicationFrame {
    pub runoff_m: f64,
    pub infiltration_m: f64,
    pub evapotranspiration_m: f64,
    pub drainage_m: f64,
    pub lateral_flow_m: f64,
}

impl DirectPublicationFrame {
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            runoff_m: 0.0,
            infiltration_m: 0.0,
            evapotranspiration_m: 0.0,
            drainage_m: 0.0,
            lateral_flow_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectPublicationCalendarDay {
    pub year: i32,
    pub julian_day: u16,
    pub month: i8,
    pub day_of_month: i8,
    pub water_year: i16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectPublicationDayInput {
    pub calendar: DirectPublicationCalendarDay,
    pub precipitation_m: f64,
    pub effective_temperature_c: f64,
    pub initial_soil_water_m: Option<f64>,
    pub storage_input_inputs: Option<DirectStorageInputInputs>,
    pub liquid_input_inputs: Option<DirectLiquidInputInputs>,
    pub percolation_inputs: Option<DirectPercolationInputs>,
    pub infiltration_depression_inputs: Option<DirectInfiltrationDepressionInputs>,
    pub subsurface_compute_inputs: Option<DirectSubsurfaceComputeInputs>,
    pub evapotranspiration_compute_inputs: Option<DirectEvapotranspirationComputeInputs>,
    pub hydrology_projection_inputs: Option<DirectHydrologyProjectionInputs>,
    pub frost_layer_carry_projection: Option<Vec<DirectFrostLayerCarryProjection>>,
}

impl DirectPublicationDayInput {
    #[must_use]
    pub fn calendar_only(calendar: DirectPublicationCalendarDay) -> Self {
        Self {
            calendar,
            precipitation_m: 0.0,
            effective_temperature_c: 0.0,
            initial_soil_water_m: None,
            storage_input_inputs: None,
            liquid_input_inputs: None,
            percolation_inputs: None,
            infiltration_depression_inputs: None,
            subsurface_compute_inputs: None,
            evapotranspiration_compute_inputs: None,
            hydrology_projection_inputs: None,
            frost_layer_carry_projection: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectFrostLayerCarryProjection {
    pub layer_index: usize,
    pub fine_layer_count: usize,
    pub fine_layer_thickness_m: f64,
}

impl DirectFrostLayerCarryProjection {
    fn validate_for_layer(
        self,
        expected_layer_index: usize,
        layer: &DirectSubsurfaceLayerState,
    ) -> Result<(), DirectRuntimeError> {
        if self.layer_index != expected_layer_index
            || self.fine_layer_count == 0
            || !self.fine_layer_thickness_m.is_finite()
            || self.fine_layer_thickness_m <= WB11_ZERO_THRESHOLD
            || !layer.depth_m.is_finite()
            || layer.depth_m <= WB11_ZERO_THRESHOLD
            || self.fine_layer_thickness_m > layer.depth_m + WB11_ZERO_THRESHOLD
            || !layer.theta_m.is_finite()
            || !layer.residual_theta.is_finite()
            || !layer.frozen_depth_m.is_finite()
            || layer.frozen_depth_m < -WB11_ZERO_THRESHOLD
            || layer.frozen_depth_m > layer.depth_m + WB11_ZERO_THRESHOLD
        {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "frost_layer_carry_projection",
            });
        }
        Ok(())
    }

    fn projected_theta_m(self, layer: &DirectSubsurfaceLayerState) -> f64 {
        let mut remaining_frozen_depth_m = layer.frozen_depth_m.max(0.0);
        let unfrozen_depth_m = (layer.depth_m - layer.frozen_depth_m).max(0.0);
        let slsw_theta = if unfrozen_depth_m > WB11_ZERO_THRESHOLD {
            layer.residual_theta + layer.theta_m / unfrozen_depth_m
        } else {
            layer.residual_theta
        };
        let active_liquid_theta = (slsw_theta - layer.residual_theta).max(0.0);
        let mut active_liquid_m = 0.0_f64;
        for _ in 0..self.fine_layer_count {
            let slfsd_m = remaining_frozen_depth_m
                .min(self.fine_layer_thickness_m)
                .max(0.0);
            remaining_frozen_depth_m = (remaining_frozen_depth_m - slfsd_m).max(0.0);
            let fine_unfrozen_depth_m = (self.fine_layer_thickness_m - slfsd_m).max(0.0);
            active_liquid_m += active_liquid_theta * fine_unfrozen_depth_m;
        }
        active_liquid_m
    }
}

fn apply_direct_frost_carry_projection(
    layers: &mut [DirectSubsurfaceLayerState],
    projection: Option<&[DirectFrostLayerCarryProjection]>,
) -> Result<(), DirectRuntimeError> {
    let Some(projection) = projection else {
        return Ok(());
    };
    if projection.len() != layers.len() {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "frost_layer_carry_projection.layer_count",
        });
    }
    for (layer_offset, layer) in layers.iter_mut().enumerate() {
        let layer_index = layer_offset + 1;
        let projection = projection
            .iter()
            .find(|projection| projection.layer_index == layer_index)
            .ok_or(DirectRuntimeError::DirectDomainViolation {
                field: "frost_layer_carry_projection.layer_index",
            })?;
        projection.validate_for_layer(layer_index, layer)?;
        layer.theta_m = projection.projected_theta_m(layer);
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectPublicationRunMetadata {
    pub run_name: String,
    pub runtime_selection: String,
    pub output_policy: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectRunPublicationFrame {
    pub identity: DirectRunIdentity,
    pub metadata: DirectPublicationRunMetadata,
    pub rows: Vec<DirectPublicationDayRow>,
}

impl DirectRunPublicationFrame {
    fn new(
        identity: DirectRunIdentity,
        metadata: DirectPublicationRunMetadata,
        expected_row_count: usize,
    ) -> Self {
        Self {
            identity,
            metadata,
            rows: Vec::with_capacity(expected_row_count),
        }
    }

    fn push_day_row(
        &mut self,
        day_frame: &DirectDayFrame,
        calendar: DirectPublicationCalendarDay,
        lane: &DirectLaneFrame,
    ) -> Result<(), DirectRuntimeError> {
        self.rows.push(DirectPublicationDayRow::from_day_frame(
            day_frame, calendar, lane,
        )?);
        Ok(())
    }

    fn validate_complete(&self) -> Result<(), DirectRuntimeError> {
        let expected_row_count = self
            .identity
            .lane_count
            .checked_mul(self.identity.day_count)
            .ok_or(DirectRuntimeError::DirectDomainViolation {
                field: "publication.expected_row_count",
            })?;
        if self.rows.len() != expected_row_count {
            return Err(DirectRuntimeError::PublicationRowCountMismatch {
                expected_row_count,
                actual_row_count: self.rows.len(),
            });
        }
        Ok(())
    }

    #[must_use]
    pub fn rows(&self) -> &[DirectPublicationDayRow] {
        &self.rows
    }

    #[must_use]
    pub fn first_day(&self) -> Option<&DirectPublicationDayRow> {
        self.rows.first()
    }

    #[must_use]
    pub fn last_day(&self) -> Option<&DirectPublicationDayRow> {
        self.rows.last()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectPublicationDayRow {
    pub run_id: u64,
    pub hillslope_id: u32,
    pub lane_id: u32,
    pub ofe_id: u32,
    pub lane_index: usize,
    pub day_index: usize,
    pub sim_day_index: i32,
    pub calendar: DirectPublicationCalendarDay,
    pub area_m2: f64,
    pub climate: DirectPublicationClimateOperands,
    pub liquid_input: DirectPublicationLiquidInputOperands,
    pub runoff: DirectPublicationRunoffOperands,
    pub evaporation: DirectPublicationEvaporationOperands,
    pub subsurface: DirectPublicationSubsurfaceOperands,
    pub transfer: DirectPublicationTransferOperands,
    pub storage: DirectPublicationStorageOperands,
    pub profile: DirectPublicationProfileOperands,
    pub interception: DirectPublicationInterceptionOperands,
    pub erosion: DirectPublicationErosionOperands,
}

impl DirectPublicationDayRow {
    fn from_day_frame(
        day_frame: &DirectDayFrame,
        calendar: DirectPublicationCalendarDay,
        lane: &DirectLaneFrame,
    ) -> Result<Self, DirectRuntimeError> {
        if !lane.area_m2.is_finite() || lane.area_m2 <= 0.0 {
            return Err(DirectRuntimeError::InvalidPublicationArea {
                lane_id: lane.lane_id,
                area_m2: lane.area_m2,
            });
        }
        let sim_day_index = i32::try_from(day_frame.day_index + 1).map_err(|_| {
            DirectRuntimeError::DirectDomainViolation {
                field: "publication.sim_day_index",
            }
        })?;
        let (peak_runoff_m3_s, runoff_duration_s) =
            direct_publication_peak_runoff_operands(day_frame.hydrology_projection.q_runoff_m)?;
        let runoff = DirectPublicationRunoffOperands {
            q_mm: m_to_mm(day_frame.hydrology_projection.q_runoff_m)?,
            qofe_mm: m_to_mm(day_frame.hydrology_projection.q_ofe_m)?,
            runvol_m3: depth_to_volume_m3(
                "publication.runoff.runvol_m3",
                day_frame.hydrology_projection.q_ofe_m,
                lane.area_m2,
            )?,
            peak_runoff_m3_s,
            runoff_duration_s,
        };
        let subsurface_lateral_m = day_frame.hydrology_projection.lateral_flow_m;

        Ok(Self {
            run_id: day_frame.identity.run_id,
            hillslope_id: day_frame.identity.hillslope_id,
            lane_id: lane.lane_id,
            ofe_id: lane.lane_id,
            lane_index: day_frame.lane_index,
            day_index: day_frame.day_index,
            sim_day_index,
            calendar,
            area_m2: lane.area_m2,
            climate: DirectPublicationClimateOperands {
                precipitation_mm: m_to_mm(day_frame.normalization.precipitation_m)?,
            },
            liquid_input: DirectPublicationLiquidInputOperands {
                rm_mm: m_to_mm(day_frame.liquid_input.liquid_input_m)?,
                irrigation_mm: 0.0,
            },
            runoff,
            evaporation: DirectPublicationEvaporationOperands {
                ep_mm: m_to_mm(day_frame.hydrology_projection.plant_transpiration_m)?,
                es_mm: m_to_mm(day_frame.hydrology_projection.soil_evaporation_m)?,
                er_mm: m_to_mm(day_frame.hydrology_projection.residue_evaporation_m)?,
                total_evapotranspiration_mm: m_to_mm(
                    day_frame.hydrology_projection.evapotranspiration_m,
                )?,
            },
            subsurface: DirectPublicationSubsurfaceOperands {
                dp_mm: m_to_mm(day_frame.hydrology_projection.deep_percolation_m)?,
                latqcc_mm: m_to_mm(subsurface_lateral_m)?,
                tile_mm: m_to_mm(day_frame.hydrology_projection.tile_drainage_m)?,
                sbrunv_m3: depth_to_volume_m3(
                    "publication.subsurface.sbrunv_m3",
                    subsurface_lateral_m,
                    lane.area_m2,
                )?,
            },
            transfer: DirectPublicationTransferOperands {
                upstream_surface_mm: m_to_mm(day_frame.normalization.upstream_flow_m)?,
                upstream_lateral_mm: m_to_mm(day_frame.normalization.subsurface_input_m)?,
            },
            storage: DirectPublicationStorageOperands {
                total_soil_mm: m_to_mm(day_frame.hydrology_projection.total_soil_m)?,
                soil_water_total_mm: m_to_mm(day_frame.hydrology_projection.soil_water_total_m)?,
                frozwt_mm: m_to_mm(day_frame.hydrology_projection.frozen_soil_water_m)?,
                frdp_mm: None,
                snow_water_mm: m_to_mm(day_frame.hydrology_projection.snow_water_m)?,
            },
            profile: DirectPublicationProfileOperands {
                depth_mm: option_m_to_mm(day_frame.hydrology_projection.profile_depth_m)?,
                porosity_cap_mm: option_m_to_mm(
                    day_frame.hydrology_projection.profile_porosity_cap_m,
                )?,
                fc_store_mm: option_m_to_mm(
                    day_frame.hydrology_projection.profile_field_capacity_m,
                )?,
                wp_store_mm: option_m_to_mm(
                    day_frame.hydrology_projection.profile_wilting_point_m,
                )?,
            },
            interception: DirectPublicationInterceptionOperands {
                interception_mm: 0.0,
                interception_storage_mm: None,
            },
            erosion: DirectPublicationErosionOperands::zero_authority(),
        })
    }
}

fn direct_publication_peak_runoff_operands(
    q_runoff_m: f64,
) -> Result<(Option<f64>, Option<f64>), DirectRuntimeError> {
    validate_finite("publication.runoff.q_runoff_m", q_runoff_m)?;
    validate_nonnegative_direct_m("publication.runoff.q_runoff_m", q_runoff_m)?;
    if q_runoff_m < WB16_RUNOFF_NEAR_ZERO_THRESHOLD {
        return Ok((Some(WB16_PEAKRO_FLOOR), Some(0.0)));
    }
    Ok((None, None))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectPublicationClimateOperands {
    pub precipitation_mm: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectPublicationLiquidInputOperands {
    pub rm_mm: f64,
    pub irrigation_mm: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectPublicationRunoffOperands {
    pub q_mm: f64,
    pub qofe_mm: f64,
    pub runvol_m3: f64,
    pub peak_runoff_m3_s: Option<f64>,
    pub runoff_duration_s: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectPublicationEvaporationOperands {
    pub ep_mm: f64,
    pub es_mm: f64,
    pub er_mm: f64,
    pub total_evapotranspiration_mm: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectPublicationSubsurfaceOperands {
    pub dp_mm: f64,
    pub latqcc_mm: f64,
    pub tile_mm: f64,
    pub sbrunv_m3: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectPublicationTransferOperands {
    pub upstream_surface_mm: f64,
    pub upstream_lateral_mm: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectPublicationStorageOperands {
    pub total_soil_mm: f64,
    pub soil_water_total_mm: f64,
    pub frozwt_mm: f64,
    pub frdp_mm: Option<f64>,
    pub snow_water_mm: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectPublicationProfileOperands {
    pub depth_mm: Option<f64>,
    pub porosity_cap_mm: Option<f64>,
    pub fc_store_mm: Option<f64>,
    pub wp_store_mm: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectPublicationInterceptionOperands {
    pub interception_mm: f64,
    pub interception_storage_mm: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectPublicationErosionOperands {
    pub peak_runoff_m3_s: Option<f64>,
    pub runoff_duration_s: Option<f64>,
    pub total_detachment_kg: Option<f64>,
    pub total_deposition_kg: Option<f64>,
    pub hbp_total_detachment_kg: Option<f64>,
    pub hbp_total_deposition_kg: Option<f64>,
    pub hbp_sediment_concentration_kg_m3: Option<f64>,
    pub sediment_concentration_kg_m3: Option<[f64; 5]>,
}

impl DirectPublicationErosionOperands {
    #[must_use]
    pub const fn absent_authority() -> Self {
        Self {
            peak_runoff_m3_s: None,
            runoff_duration_s: None,
            total_detachment_kg: None,
            total_deposition_kg: None,
            hbp_total_detachment_kg: None,
            hbp_total_deposition_kg: None,
            hbp_sediment_concentration_kg_m3: None,
            sediment_concentration_kg_m3: None,
        }
    }

    #[must_use]
    pub const fn zero_authority() -> Self {
        Self {
            peak_runoff_m3_s: Some(0.0),
            runoff_duration_s: Some(0.0),
            total_detachment_kg: Some(0.0),
            total_deposition_kg: Some(0.0),
            hbp_total_detachment_kg: Some(0.0),
            hbp_total_deposition_kg: Some(0.0),
            hbp_sediment_concentration_kg_m3: Some(0.0),
            sediment_concentration_kg_m3: Some([0.0; 5]),
        }
    }
}
