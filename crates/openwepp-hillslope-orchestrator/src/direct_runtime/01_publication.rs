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
    pub interception_m: f64,
    pub erosion_producer_required: bool,
    pub initial_soil_water_m: Option<f64>,
    pub storage_input_inputs: Option<DirectStorageInputInputs>,
    pub liquid_input_inputs: Option<DirectLiquidInputInputs>,
    pub percolation_inputs: Option<DirectPercolationInputs>,
    pub infiltration_depression_inputs: Option<DirectInfiltrationDepressionInputs>,
    pub peak_runoff_inputs: Option<DirectPeakRunoffInputs>,
    pub subsurface_compute_inputs: Option<DirectSubsurfaceComputeInputs>,
    pub decomposition_inputs: Option<DirectDecompositionInputs>,
    pub residue_partition_inputs: Option<DirectResiduePartitionInputs>,
    pub evapotranspiration_compute_inputs: Option<DirectEvapotranspirationComputeInputs>,
    pub annual_growth_inputs: Option<DirectGrowthInputs>,
    pub perennial_growth_inputs: Option<DirectGrowthInputs>,
    pub canopy_cover_fraction: Option<f64>,
    pub snow_coupling_inputs: Option<DirectSnowCouplingInputs>,
    pub hydrology_projection_inputs: Option<DirectHydrologyProjectionInputs>,
    pub erosion_inputs: Option<DirectErosionInputs>,
    pub frost_storage_liquid_delta_m: Option<f64>,
    pub winter_frost_compute_inputs: Option<crate::hydrology::DirectWinterFrostComputeInputs>,
    // Single-solve authority (WP-2): the day's frost partition outcome,
    // solved once by the runner authority from start-of-day lane state.
    pub winter_frost_outcome: Option<Box<crate::hydrology::DirectWinterFrostPartitionOutcome>>,
    pub frost_layer_carry_projection: Option<Vec<DirectFrostLayerCarryProjection>>,
    pub frost_runtime_carry: Option<DirectFrostRuntimeCarry>,
}

impl DirectPublicationDayInput {
    #[must_use]
    pub fn calendar_only(calendar: DirectPublicationCalendarDay) -> Self {
        Self {
            calendar,
            precipitation_m: 0.0,
            effective_temperature_c: 0.0,
            interception_m: 0.0,
            erosion_producer_required: false,
            initial_soil_water_m: None,
            storage_input_inputs: None,
            liquid_input_inputs: None,
            percolation_inputs: None,
            infiltration_depression_inputs: None,
            peak_runoff_inputs: None,
            subsurface_compute_inputs: None,
            decomposition_inputs: None,
            residue_partition_inputs: None,
            evapotranspiration_compute_inputs: None,
            annual_growth_inputs: None,
            perennial_growth_inputs: None,
            canopy_cover_fraction: None,
            snow_coupling_inputs: None,
            hydrology_projection_inputs: None,
            erosion_inputs: None,
            frost_storage_liquid_delta_m: None,
            winter_frost_compute_inputs: None,
            winter_frost_outcome: None,
            frost_layer_carry_projection: None,
            frost_runtime_carry: None,
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
        if layer.frozen_depth_m <= WB11_ZERO_THRESHOLD {
            return layer.theta_m;
        }
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
    pub water_temperature: DirectPublicationWaterTemperatureOperands,
    pub profile: DirectPublicationProfileOperands,
    pub interception: DirectPublicationInterceptionOperands,
    pub erosion: DirectPublicationErosionOperands,
}

impl DirectPublicationDayRow {
    fn from_day_frame(
        day_frame: &DirectDayFrame,
        day_input: &DirectPublicationDayInput,
        lane: &DirectLaneFrame,
    ) -> Result<Self, DirectRuntimeError> {
        validate_direct_publication_lane(lane)?;
        let sim_day_index = i32::try_from(day_frame.day_index + 1).map_err(|_| {
            DirectRuntimeError::DirectDomainViolation {
                field: "publication.sim_day_index",
            }
        })?;
        let runoff = direct_publication_runoff_operands(day_frame, lane)?;
        let subsurface_lateral_m = day_frame.hydrology_projection.lateral_flow_m;
        let interception_m = day_frame.interception_m;
        validate_nonnegative_direct_m("publication.interception_m", interception_m)?;
        let storage = direct_publication_storage_operands(day_frame)?;
        let water_temperature = direct_publication_water_temperature_operands(day_frame)?;
        let erosion = direct_publication_erosion_operands(day_frame, day_input)?;

        Ok(Self {
            run_id: day_frame.identity.run_id,
            hillslope_id: day_frame.identity.hillslope_id,
            lane_id: lane.lane_id,
            ofe_id: lane.lane_id,
            lane_index: day_frame.lane_index,
            day_index: day_frame.day_index,
            sim_day_index,
            calendar: day_input.calendar,
            area_m2: lane.area_m2,
            climate: DirectPublicationClimateOperands {
                precipitation_mm: m_to_mm(day_frame.normalization.precipitation_m)?,
            },
            liquid_input: DirectPublicationLiquidInputOperands {
                rm_mm: m_to_mm(day_frame.liquid_input.liquid_input_m + interception_m)?,
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
                sbrunv_m3: publication_mm_to_volume_m3(
                    "publication.subsurface.sbrunv_m3",
                    m_to_mm(subsurface_lateral_m)?,
                    lane.area_m2,
                )?,
            },
            transfer: DirectPublicationTransferOperands {
                upstream_surface_mm: m_to_mm(day_frame.normalization.surface_transfer_m)?,
                upstream_lateral_mm: m_to_mm(day_frame.normalization.lateral_transfer_m)?,
            },
            storage,
            water_temperature,
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
                interception_mm: m_to_mm(interception_m)?,
                interception_storage_mm: None,
            },
            erosion,
        })
    }
}

fn validate_direct_publication_lane(lane: &DirectLaneFrame) -> Result<(), DirectRuntimeError> {
    if !lane.area_m2.is_finite() || lane.area_m2 <= 0.0 {
        return Err(DirectRuntimeError::InvalidPublicationArea {
            lane_id: lane.lane_id,
            area_m2: lane.area_m2,
        });
    }
    Ok(())
}

fn direct_publication_runoff_operands(
    day_frame: &DirectDayFrame,
    lane: &DirectLaneFrame,
) -> Result<DirectPublicationRunoffOperands, DirectRuntimeError> {
    let q_publication_mm = day_frame.hydrology_projection.q_runoff_m
        * 1_000.0
        * lane.runoff_publication_efflen_m
        / lane.runoff_publication_cumulative_length_m;
    let q_publication_m = q_publication_mm / 1_000.0;
    validate_finite("publication.runoff.q_publication_m", q_publication_m)?;
    validate_nonnegative_direct_m("publication.runoff.q_publication_m", q_publication_m)?;
    // MOFEFID-B02 (INV-RUNOFFPART-032): the published QOFE column adopts the
    // post-wepp_260516 ecosystem convention QOFE == Q (cumulative-length
    // normalization). The per-OFE local-length basis is retained ONLY as the
    // internal runoff volume/peak basis so H.pass.runvol and peak stay
    // byte-invariant (the wepp_260516 fix's preserved property). On single-OFE
    // lanes cumulative_length == ofe_length so QOFE already equalled Q and this
    // is a no-op (single-OFE byte-identity).
    let runvol_basis_mm = day_frame.hydrology_projection.q_ofe_m
        * 1_000.0
        * lane.runoff_publication_efflen_m
        / lane.runoff_publication_ofe_length_m;
    let runvol_basis_m = runvol_basis_mm / 1_000.0;
    validate_finite("publication.runoff.runvol_basis_m", runvol_basis_m)?;
    validate_nonnegative_direct_m("publication.runoff.runvol_basis_m", runvol_basis_m)?;
    let qofe_publication_mm = q_publication_mm;
    let (peak_runoff_m3_s, runoff_duration_s) =
        direct_publication_peak_runoff_operands(day_frame, runvol_basis_m)?;
    Ok(DirectPublicationRunoffOperands {
        q_mm: q_publication_mm,
        qofe_mm: qofe_publication_mm,
        runvol_m3: publication_mm_to_volume_m3(
            "publication.runoff.runvol_m3",
            runvol_basis_mm,
            lane.area_m2,
        )?,
        peak_runoff_m3_s,
        runoff_duration_s,
    })
}

fn direct_publication_storage_operands(
    day_frame: &DirectDayFrame,
) -> Result<DirectPublicationStorageOperands, DirectRuntimeError> {
    let total_soil_publication_m = nonnegative_publication_storage_m(
        "publication.storage.total_soil_publication_m",
        day_frame.hydrology_projection.total_soil_m,
    )?;
    let soil_water_total_publication_m = nonnegative_publication_storage_m(
        "publication.storage.soil_water_total_publication_m",
        day_frame.hydrology_projection.soil_water_total_m,
    )?;
    let snow_depth_publication_m = nonnegative_publication_storage_m(
        "publication.storage.snow_depth_publication_m",
        day_frame.winter_column.snow.runtime_depth_m,
    )?;
    Ok(DirectPublicationStorageOperands {
        total_soil_mm: m_to_mm(total_soil_publication_m)?,
        soil_water_total_mm: m_to_mm(soil_water_total_publication_m)?,
        frozwt_mm: m_to_mm(day_frame.hydrology_projection.frozen_soil_water_m)?,
        frdp_mm: Some(m_to_mm(day_frame.hydrology_projection.frost_depth_m)?),
        snow_water_mm: m_to_mm(day_frame.hydrology_projection.snow_water_m)?,
        snow_depth_mm: m_to_mm(snow_depth_publication_m)?,
    })
}

fn direct_publication_water_temperature_operands(
    day_frame: &DirectDayFrame,
) -> Result<DirectPublicationWaterTemperatureOperands, DirectRuntimeError> {
    let meltwater_temperature_c = day_frame
        .snow_coupling_shadow_projection
        .as_ref()
        .and_then(|projection| projection.stage3_diagnostics.as_deref())
        .and_then(|diagnostics| diagnostics.meltwater_temperature_c)
        .map(openwepp_unit_boundary::TemperatureCelsius::as_celsius);
    if let Some(value) = meltwater_temperature_c {
        validate_finite(
            "publication.water_temperature.meltwater_temperature_c",
            value,
        )?;
        if value > WB11_ZERO_THRESHOLD {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "publication.water_temperature.meltwater_temperature_c",
            });
        }
    }
    Ok(DirectPublicationWaterTemperatureOperands {
        meltwater_temperature_c,
    })
}

fn nonnegative_publication_storage_m(
    field: &'static str,
    value: f64,
) -> Result<f64, DirectRuntimeError> {
    validate_finite(field, value)?;
    if value < -WB11_ZERO_THRESHOLD {
        return Err(DirectRuntimeError::DirectDomainViolation { field });
    }
    Ok(value.max(0.0))
}

fn direct_publication_erosion_operands(
    day_frame: &DirectDayFrame,
    day_input: &DirectPublicationDayInput,
) -> Result<DirectPublicationErosionOperands, DirectRuntimeError> {
    if let Some(erosion_projection) = day_frame.erosion_shadow_projection.as_ref() {
        if erosion_projection.publication_authority {
            return Ok(erosion_projection.publication);
        }
    }
    if day_input.erosion_producer_required {
        return Err(DirectRuntimeError::MissingDirectUpstream {
            upstream: "R7D5 direct Wave-1 sediment producer",
        });
    }
    Ok(DirectPublicationErosionOperands::zero_authority())
}

fn direct_publication_peak_runoff_operands(
    day_frame: &DirectDayFrame,
    q_runoff_m: f64,
) -> Result<(Option<f64>, Option<f64>), DirectRuntimeError> {
    validate_finite("publication.runoff.q_runoff_m", q_runoff_m)?;
    validate_nonnegative_direct_m("publication.runoff.q_runoff_m", q_runoff_m)?;
    if let Some(peak_runoff) = day_frame.peak_runoff_shadow_projection.as_ref() {
        return Ok((
            Some(peak_runoff.peak_runoff_m3_s),
            Some(peak_runoff.runoff_duration_s),
        ));
    }
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
    pub snow_depth_mm: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectPublicationWaterTemperatureOperands {
    pub meltwater_temperature_c: Option<f64>,
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
    /// ADR-0036 D2: the day's unit-normalized hourly runoff distribution
    /// (`REF-SED-DC01-SHAPE` weights). `Some` on hydrograph-resolved Wave-1
    /// lanes (the HBP writer forms `V_h = runvol · w_h`); `None` on lanes
    /// without the hourly surfaces (Wave-2 multi-OFE — minor-0 payloads).
    pub hourly_runoff_fraction: Option<[f64; 24]>,
    /// ADR-0036 D2: hour-integrated exported sediment mass (kg) on the
    /// same time base (`Σ = ` the day's exported mass; all-zero on
    /// non-routed days so the closure holds trivially).
    pub hourly_sediment_mass_kg: Option<[f64; 24]>,
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
            hourly_runoff_fraction: None,
            hourly_sediment_mass_kg: None,
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
            hourly_runoff_fraction: None,
            hourly_sediment_mass_kg: None,
        }
    }
}
