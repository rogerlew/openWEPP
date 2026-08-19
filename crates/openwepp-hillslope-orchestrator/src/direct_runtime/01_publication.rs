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
    pub wat5_subhourly_requested: bool,
    pub initial_soil_water_m: Option<f64>,
    pub storage_input_inputs: Option<DirectStorageInputInputs>,
    pub liquid_input_inputs: Option<DirectLiquidInputInputs>,
    pub percolation_inputs: Option<DirectPercolationInputs>,
    pub infiltration_depression_inputs: Option<DirectInfiltrationDepressionInputs>,
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
            wat5_subhourly_requested: false,
            initial_soil_water_m: None,
            storage_input_inputs: None,
            liquid_input_inputs: None,
            percolation_inputs: None,
            infiltration_depression_inputs: None,
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
#[cfg_attr(any(feature = "restart-authority-evidence", feature = "persisted-restart-v1"), derive(serde::Serialize))]
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
    /// DC01 unit-normalized hourly weights of THIS lane's OWN surface
    /// runoff (`INV-RUNOFFPART-031` M2 distribution recomputed at
    /// publication over the lane's own `wb14_hourly_excess` +
    /// `ui_SCrunf`-lineage carry — NOT `lane.transfer`, which carries the
    /// downstream INFLOW distribution). In-memory only (the parquet sink
    /// maps named columns); consumed by the Lane D seam shadow to
    /// reconstruct the routed source series (`weights[h] × runvol/area`,
    /// deliberately NOT published `QOFE`, which aliases cumulative `Q`) per
    /// the ADR-0036 weights-times-total hourly-flow authority.
    pub dc01_surface_hourly_weights: [f64; DIRECT_TRANSFER_HOUR_COUNT],
}

/// The lane's OWN surface-runoff hourly weights for the seam shadow:
/// the DC01 M2 distribution over the lane's own post-partition WB14 runoff
/// plus saturation return, against its own runoff total. Routed melt and
/// runon already passed through WB14. Positive runoff without timing fails
/// closed.
fn direct_publication_own_surface_hourly_weights(
    day_frame: &DirectDayFrame,
) -> Result<[f64; DIRECT_TRANSFER_HOUR_COUNT], DirectRuntimeError> {
    let runoff = day_frame.runoff_shadow_projection.as_ref().ok_or(
        DirectRuntimeError::MissingDirectUpstream {
            upstream: "R4A runoff partition producer (publication weights)",
        },
    )?;
    let subsurface = day_frame
        .subsurface_compute_shadow_projection
        .as_ref()
        .ok_or(DirectRuntimeError::MissingDirectUpstream {
            upstream: "R4O subsurface compute producer (publication weights)",
        })?;
    crate::direct_runtime::runoff::dc01_surface_runoff_hourly_weights(
        runoff.q_runoff_m,
        &day_frame.wb14_hourly_excess_m,
        &subsurface.hourly_saturation_carry_m,
    )
}

struct DirectPublicationPrecomputedOperands {
    runoff: DirectPublicationRunoffOperands,
    subsurface_lateral_m: f64,
    interception_m: f64,
    storage: DirectPublicationStorageOperands,
    water_temperature: DirectPublicationWaterTemperatureOperands,
    erosion: DirectPublicationErosionOperands,
}

fn direct_publication_precomputed_operands(
    day_frame: &DirectDayFrame,
    day_input: &DirectPublicationDayInput,
    lane: &DirectLaneFrame,
) -> Result<DirectPublicationPrecomputedOperands, DirectRuntimeError> {
    let runoff = direct_publication_runoff_operands(day_frame, lane)?;
    let subsurface_lateral_m = day_frame.hydrology_projection.lateral_flow_m;
    let interception_m = day_frame.interception_m;
    validate_nonnegative_direct_m("publication.interception_m", interception_m)?;
    let storage = direct_publication_storage_operands(day_frame)?;
    let water_temperature = direct_publication_water_temperature_operands(day_frame)?;
    let erosion = direct_publication_erosion_operands(day_frame, day_input)?;
    Ok(DirectPublicationPrecomputedOperands {
        runoff,
        subsurface_lateral_m,
        interception_m,
        storage,
        water_temperature,
        erosion,
    })
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
        let precomputed = direct_publication_precomputed_operands(day_frame, day_input, lane)?;
        let terminal_groundwater_output =
            direct_publication_terminal_groundwater_output(day_frame, lane);
        let groundwater_baseflow_mm = publication_volume_m3_to_mm(
            "publication.subsurface.groundwater_baseflow_mm",
            terminal_groundwater_output.baseflow_m3,
            lane.area_m2,
        )?;
        let groundwater_deep_seepage_mm = publication_volume_m3_to_mm(
            "publication.subsurface.groundwater_deep_seepage_mm",
            terminal_groundwater_output.deep_seepage_m3,
            lane.area_m2,
        )?;

        Ok(Self {
            run_id: day_frame.identity.run_id,
            hillslope_id: day_frame.identity.hillslope_id,
            dc01_surface_hourly_weights: direct_publication_own_surface_hourly_weights(day_frame)?,
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
                rm_mm: m_to_mm(day_frame.liquid_input.liquid_input_m + precomputed.interception_m)?,
                irrigation_mm: 0.0,
            },
            runoff: precomputed.runoff,
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
                latqcc_mm: m_to_mm(precomputed.subsurface_lateral_m)?,
                tile_mm: m_to_mm(day_frame.hydrology_projection.tile_drainage_m)?,
                sbrunv_m3: publication_mm_to_volume_m3(
                    "publication.subsurface.sbrunv_m3",
                    m_to_mm(precomputed.subsurface_lateral_m)?,
                    lane.area_m2,
                )?,
                groundwater_baseflow_mm,
                groundwater_baseflow_m3: terminal_groundwater_output.baseflow_m3,
                groundwater_deep_seepage_mm,
                groundwater_deep_seepage_m3: terminal_groundwater_output.deep_seepage_m3,
            },
            transfer: DirectPublicationTransferOperands {
                upstream_surface_mm: m_to_mm(day_frame.normalization.surface_transfer_m)?,
                upstream_lateral_mm: m_to_mm(day_frame.normalization.lateral_transfer_m)?,
            },
            storage: precomputed.storage,
            water_temperature: precomputed.water_temperature,
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
                interception_mm: m_to_mm(precomputed.interception_m)?,
                interception_storage_mm: None,
            },
            erosion: precomputed.erosion,
        })
    }
}

fn direct_publication_terminal_groundwater_output(
    day_frame: &DirectDayFrame,
    lane: &DirectLaneFrame,
) -> DirectGroundwaterDayOutput {
    if lane.downstream_lane_id == 0 {
        day_frame.groundwater_output
    } else {
        DirectGroundwaterDayOutput::zero()
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
    let q_publication_mm =
        day_frame.hydrology_projection.q_runoff_m * 1_000.0 * lane.runoff_publication_efflen_m
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
    let runvol_basis_mm =
        day_frame.hydrology_projection.q_ofe_m * 1_000.0 * lane.runoff_publication_efflen_m
            / lane.runoff_publication_ofe_length_m;
    let runvol_basis_m = runvol_basis_mm / 1_000.0;
    validate_finite("publication.runoff.runvol_basis_m", runvol_basis_m)?;
    validate_nonnegative_direct_m("publication.runoff.runvol_basis_m", runvol_basis_m)?;
    let qofe_publication_mm = q_publication_mm;
    let (peak_runoff_m3_s, runoff_duration_s) =
        direct_publication_peak_runoff_operands(day_frame, runvol_basis_m, lane.area_m2)?;
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

const WB13_STORAGE_ALIAS_TOLERANCE_M: f64 = 1.0e-9;

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
    if (soil_water_total_publication_m - total_soil_publication_m).abs()
        > WB13_STORAGE_ALIAS_TOLERANCE_M
    {
        return Err(DirectRuntimeError::DirectClosureToleranceExceeded {
            field: "publication.storage.soil_water_total_alias",
        });
    }
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
        .and_then(|projection| {
            projection
                .mass_transition_ledgers
                .stage3_outcome()
                .meltwater_temperature_c
        })
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
    area_m2: f64,
) -> Result<(Option<f64>, Option<f64>), DirectRuntimeError> {
    validate_finite("publication.runoff.q_runoff_m", q_runoff_m)?;
    validate_nonnegative_direct_m("publication.runoff.q_runoff_m", q_runoff_m)?;
    validate_finite("publication.runoff.area_m2", area_m2)?;
    if area_m2 <= 0.0 {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "publication.runoff.area_m2",
        });
    }
    if let Some(peak_runoff) = day_frame.peak_runoff_shadow_projection.as_ref() {
        validate_finite(
            "publication.runoff.shadow_q_runoff_m",
            peak_runoff.q_runoff_m,
        )?;
        validate_nonnegative_direct_m(
            "publication.runoff.shadow_q_runoff_m",
            peak_runoff.q_runoff_m,
        )?;
        if peak_runoff.q_runoff_m == 0.0 {
            if q_runoff_m > 0.0 {
                return Err(DirectRuntimeError::DirectClosureToleranceExceeded {
                    field: "publication.runoff.peak_runoff_basis_m",
                });
            }
            return Ok((Some(0.0), Some(0.0)));
        }
        if q_runoff_m == 0.0 {
            return Err(DirectRuntimeError::DirectClosureToleranceExceeded {
                field: "publication.runoff.peak_runoff_basis_m",
            });
        }
        validate_finite(
            "publication.runoff.peak_runoff_rate_m_s",
            peak_runoff.peak_runoff_rate_m_s,
        )?;
        if peak_runoff.peak_runoff_rate_m_s <= 0.0 {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "publication.runoff.peak_runoff_rate_m_s",
            });
        }
        validate_finite(
            "publication.runoff.runoff_duration_s",
            peak_runoff.runoff_duration_s,
        )?;
        if peak_runoff.runoff_duration_s <= 0.0 {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "publication.runoff.runoff_duration_s",
            });
        }
        let peak_runoff_rate_m_s = peak_runoff.peak_runoff_rate_m_s * q_runoff_m
            / peak_runoff.q_runoff_m;
        validate_finite(
            "publication.runoff.basis_adjusted_peak_runoff_rate_m_s",
            peak_runoff_rate_m_s,
        )?;
        let peak_runoff_m3_s = peak_runoff_rate_m_s * area_m2;
        validate_finite("publication.runoff.peak_runoff_m3_s", peak_runoff_m3_s)?;
        return Ok((
            Some(peak_runoff_m3_s),
            Some(peak_runoff.runoff_duration_s),
        ));
    }
    if q_runoff_m == 0.0 {
        return Ok((Some(0.0), Some(0.0)));
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
    pub groundwater_baseflow_mm: f64,
    pub groundwater_baseflow_m3: f64,
    pub groundwater_deep_seepage_mm: f64,
    pub groundwater_deep_seepage_m3: f64,
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
    pub peak_runoff_rate_m_s: Option<f64>,
    pub runoff_duration_s: Option<f64>,
    pub total_detachment_kg: Option<f64>,
    pub total_deposition_kg: Option<f64>,
    pub hbp_total_detachment_kg: Option<f64>,
    pub hbp_total_deposition_kg: Option<f64>,
    pub hbp_sediment_concentration_kg_m3: Option<f64>,
    pub sediment_concentration_kg_m3: Option<[f64; 5]>,
    /// ADR-0036 D2: the day's unit-normalized hourly runoff distribution
    /// from the selected water-shape authority. `Some` on hydrograph-
    /// resolved Wave-1 lanes (the HBP writer forms `V_h = runvol · w_h`);
    /// `None` on lanes without the hourly surfaces.
    pub hourly_runoff_fraction: Option<[f64; 24]>,
    /// ADR-0036 D2: hour-integrated exported sediment mass (kg) on the
    /// same time base (`Σ = ` the day's exported mass; all-zero on
    /// non-routed days so the closure holds trivially).
    pub hourly_sediment_mass_kg: Option<[f64; 24]>,
    /// E.4: the specific-surface-area enrichment ratio (`enrich.for`
    /// `enrato`, exit-lane day diagnostic).
    pub enrichment_ratio: Option<f64>,
}

impl DirectPublicationErosionOperands {
    #[must_use]
    pub const fn absent_authority() -> Self {
        Self {
            peak_runoff_rate_m_s: None,
            runoff_duration_s: None,
            total_detachment_kg: None,
            total_deposition_kg: None,
            hbp_total_detachment_kg: None,
            hbp_total_deposition_kg: None,
            hbp_sediment_concentration_kg_m3: None,
            sediment_concentration_kg_m3: None,
            hourly_runoff_fraction: None,
            hourly_sediment_mass_kg: None,
            enrichment_ratio: None,
        }
    }

    #[must_use]
    pub const fn zero_authority() -> Self {
        Self {
            peak_runoff_rate_m_s: Some(0.0),
            runoff_duration_s: Some(0.0),
            total_detachment_kg: Some(0.0),
            total_deposition_kg: Some(0.0),
            hbp_total_detachment_kg: Some(0.0),
            hbp_total_deposition_kg: Some(0.0),
            hbp_sediment_concentration_kg_m3: Some(0.0),
            sediment_concentration_kg_m3: Some([0.0; 5]),
            hourly_runoff_fraction: None,
            hourly_sediment_mass_kg: None,
            enrichment_ratio: None,
        }
    }
}

#[cfg(test)]
mod cqr_publication_tests {
    use super::*;

    fn identity(lane_count: usize, day_count: usize) -> DirectRunIdentity {
        DirectRunIdentity::new(101, 501, lane_count, day_count)
            .expect("valid publication characterization identity")
    }

    fn calendar() -> DirectPublicationCalendarDay {
        DirectPublicationCalendarDay {
            year: 2026,
            julian_day: 1,
            month: 1,
            day_of_month: 1,
            water_year: 2026,
        }
    }

    #[test]
    fn cqr_publication_input_and_frame_accessors_cover_empty_and_complete_states() {
        let input = DirectPublicationDayInput::calendar_only(calendar());
        assert_eq!(input.precipitation_m.to_bits(), 0.0_f64.to_bits());
        assert!(input.initial_soil_water_m.is_none());
        assert!(input.frost_runtime_carry.is_none());
        assert_eq!(
            DirectPublicationFrame::empty().runoff_m.to_bits(),
            0.0_f64.to_bits()
        );

        let metadata = DirectPublicationRunMetadata {
            run_name: "cqr_publication".to_string(),
            runtime_selection: "direct".to_string(),
            output_policy: "test".to_string(),
        };
        let empty = DirectRunPublicationFrame::new(identity(1, 1), metadata.clone(), 1);
        assert!(matches!(
            empty.validate_complete(),
            Err(DirectRuntimeError::PublicationRowCountMismatch {
                expected_row_count: 1,
                actual_row_count: 0
            })
        ));
        assert!(empty.rows().is_empty());
        assert!(empty.first_day().is_none());
        assert!(empty.last_day().is_none());

        assert_eq!(empty.metadata, metadata);
    }

    #[test]
    fn cqr_frost_projection_covers_absent_valid_and_invalid_domains() {
        let mut layers = vec![DirectSubsurfaceLayerState::neutral()];
        apply_direct_frost_carry_projection(&mut layers, None).expect("absent projection");
        let original_theta = layers[0].theta_m;
        let valid = DirectFrostLayerCarryProjection {
            layer_index: 1,
            fine_layer_count: 1,
            fine_layer_thickness_m: layers[0].depth_m,
        };
        apply_direct_frost_carry_projection(&mut layers, Some(&[valid]))
            .expect("unfrozen valid projection");
        assert_eq!(layers[0].theta_m.to_bits(), original_theta.to_bits());

        layers[0].frozen_depth_m = layers[0].depth_m / 2.0;
        layers[0].theta_m = 0.1;
        apply_direct_frost_carry_projection(&mut layers, Some(&[valid]))
            .expect("partially frozen valid projection");
        assert!(layers[0].theta_m >= 0.0);

        layers[0].frozen_depth_m = layers[0].depth_m;
        layers[0].theta_m = 0.1;
        apply_direct_frost_carry_projection(&mut layers, Some(&[valid]))
            .expect("fully frozen projection uses residual theta branch");
        assert!(layers[0].theta_m.abs() < f64::EPSILON);

        let mut two_layers = vec![DirectSubsurfaceLayerState::neutral(); 2];
        assert!(matches!(
            apply_direct_frost_carry_projection(&mut two_layers, Some(&[valid])),
            Err(DirectRuntimeError::DirectDomainViolation {
                field: "frost_layer_carry_projection.layer_count"
            })
        ));
        assert!(matches!(
            apply_direct_frost_carry_projection(
                &mut two_layers,
                Some(&[
                    valid,
                    DirectFrostLayerCarryProjection {
                        layer_index: 3,
                        ..valid
                    },
                ])
            ),
            Err(DirectRuntimeError::DirectDomainViolation {
                field: "frost_layer_carry_projection.layer_index"
            })
        ));
        assert!(
            DirectFrostLayerCarryProjection {
                fine_layer_count: 0,
                ..valid
            }
            .validate_for_layer(1, &DirectSubsurfaceLayerState::neutral())
            .is_err()
        );
    }

    #[test]
    fn cqr_publication_helpers_cover_guards_and_optional_authority_branches() {
        let run_identity = identity(1, 1);
        let mut frame = DirectRunFrame::skeleton(run_identity).expect("publication frame");
        frame.lanes[0].area_m2 = 100.0;
        let lane = &frame.lanes[0];
        validate_direct_publication_lane(lane).expect("valid publication lane");
        let mut invalid_lane = lane.clone();
        invalid_lane.area_m2 = 0.0;
        assert!(matches!(
            validate_direct_publication_lane(&invalid_lane),
            Err(DirectRuntimeError::InvalidPublicationArea { .. })
        ));

        assert_eq!(
            nonnegative_publication_storage_m("test", -WB11_ZERO_THRESHOLD / 2.0)
                .expect("roundoff storage")
                .to_bits(),
            0.0_f64.to_bits()
        );
        assert!(nonnegative_publication_storage_m("test", -1.0).is_err());
        assert!(nonnegative_publication_storage_m("test", f64::NAN).is_err());

        let mut day = DirectDayFrame::seed(run_identity, 0, 0).expect("publication day");
        let input = DirectPublicationDayInput::calendar_only(calendar());
        let zero_erosion =
            direct_publication_erosion_operands(&day, &input).expect("optional erosion authority");
        assert_eq!(
            zero_erosion,
            DirectPublicationErosionOperands::zero_authority()
        );
        let mut required = input.clone();
        required.erosion_producer_required = true;
        assert!(matches!(
            direct_publication_erosion_operands(&day, &required),
            Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "R7D5 direct Wave-1 sediment producer"
            })
        ));
        day.erosion_shadow_projection = Some(DirectErosionShadowProjection {
            lane_index: 0,
            day_index: 0,
            wave1_active: false,
            publication_authority: true,
            publication: DirectPublicationErosionOperands::absent_authority(),
        });
        assert_eq!(
            direct_publication_erosion_operands(&day, &required)
                .expect("authoritative erosion projection"),
            DirectPublicationErosionOperands::absent_authority()
        );

        direct_publication_storage_operands(&day).expect("zero storage operands");
        direct_publication_water_temperature_operands(&day).expect("absent melt temperature");
        assert!(matches!(
            direct_publication_own_surface_hourly_weights(&day),
            Err(DirectRuntimeError::MissingDirectUpstream { .. })
        ));

        day.groundwater_output = DirectGroundwaterDayOutput {
            enabled: true,
            recharge_m3: 1.0,
            storage_before_m3: 4.0,
            storage_after_m3: 4.0,
            storage_delta_m3: 0.0,
            baseflow_m3: 2.0,
            deep_seepage_m3: 3.0,
            baseflow_threshold_area_ha: None,
        };
        assert!(
            (direct_publication_terminal_groundwater_output(&day, lane).baseflow_m3 - 2.0).abs()
                < f64::EPSILON
        );
        let mut upstream_lane = lane.clone();
        upstream_lane.downstream_lane_id = 2;
        assert!(
            direct_publication_terminal_groundwater_output(&day, &upstream_lane)
                .baseflow_m3
                .abs()
                < f64::EPSILON
        );
    }

    #[test]
    fn publication_peak_scales_area_once_and_guards_its_boundary() {
        let run_identity = identity(1, 1);
        let mut day = DirectDayFrame::seed(run_identity, 0, 0).expect("publication day");
        assert_eq!(
            direct_publication_peak_runoff_operands(&day, 0.0, 1.0).expect("dry peak"),
            (Some(0.0), Some(0.0))
        );
        assert_eq!(
            direct_publication_peak_runoff_operands(&day, 0.01, 1.0)
                .expect("missing wet peak"),
            (None, None)
        );
        day.peak_runoff_shadow_projection = Some(DirectPeakRunoffShadowProjection {
            lane_index: 0,
            day_index: 0,
            q_runoff_m: 0.02,
            peak_runoff_rate_m_s: 0.002 / 3_600.0,
            runoff_duration_s: 36_000.0,
            peak_hour_index: Some(4),
            method_branch: 5.0,
            tstar: 0.0,
            qpstar: 0.0,
            vstar: 0.0,
        });
        let (peak_100_m3_s, duration_100_s) =
            direct_publication_peak_runoff_operands(&day, 0.01, 100.0)
                .expect("basis-adjusted public peak");
        let (peak_200_m3_s, duration_200_s) =
            direct_publication_peak_runoff_operands(&day, 0.01, 200.0)
                .expect("area-scaled public peak");
        assert!(
            (peak_100_m3_s.expect("public peak") - (0.002 / 3_600.0) * 0.5 * 100.0)
                .abs()
                < 1.0e-15
        );
        assert_eq!(
            peak_200_m3_s.expect("public peak").to_bits(),
            (2.0 * peak_100_m3_s.expect("public peak")).to_bits()
        );
        assert_eq!(duration_100_s, duration_200_s);
        for invalid_area_m2 in [-1.0, f64::NAN, f64::INFINITY] {
            assert!(direct_publication_peak_runoff_operands(&day, 0.01, invalid_area_m2).is_err());
        }
        assert!(direct_publication_peak_runoff_operands(&day, f64::NAN, 1.0).is_err());
        day.peak_runoff_shadow_projection
            .as_mut()
            .expect("peak shadow")
            .q_runoff_m = 0.0;
        assert!(matches!(
            direct_publication_peak_runoff_operands(&day, 0.01, 1.0),
            Err(DirectRuntimeError::DirectClosureToleranceExceeded {
                field: "publication.runoff.peak_runoff_basis_m"
            })
        ));
        day.peak_runoff_shadow_projection
            .as_mut()
            .expect("peak shadow")
            .q_runoff_m = 0.02;
        assert!(matches!(
            direct_publication_peak_runoff_operands(&day, 0.0, 1.0),
            Err(DirectRuntimeError::DirectClosureToleranceExceeded {
                field: "publication.runoff.peak_runoff_basis_m"
            })
        ));
    }

    fn snow_ledgers_with_temperature(
        value_c: f64,
    ) -> Result<DirectSnowMassTransitionLedgers, DirectSnowMassTransitionLedgerError> {
        let stage3_outcome = DirectSnowStage3Outcome {
            enabled: true,
            meltwater_temperature_c: Some(
            openwepp_unit_boundary::TemperatureCelsius::try_new(value_c)
                .expect("finite snow temperature"),
            ),
            sublimation_m: 0.0,
        };
        DirectSnowMassTransitionLedgers::try_from_parts(
            DirectSnowSolidToLiquidLedger {
                snowpack_swe_loss_m: 0.01,
                liquid_handoff_m: 0.01,
                ..DirectSnowSolidToLiquidLedger::default()
            },
            DirectSnowLiquidDispositionLedger {
                incoming_liquid_m: 0.01,
                routed_liquid_m: 0.01,
                ..DirectSnowLiquidDispositionLedger::default()
            },
            stage3_outcome,
        )
    }

    fn snow_projection_with_temperature(value_c: f64) -> DirectSnowCouplingShadowProjection {
        DirectSnowCouplingShadowProjection {
            lane_index: 0,
            day_index: 0,
            snow_coupling_m: 0.0,
            active_snow_coupling: true,
            mass_transition_ledgers: snow_ledgers_with_temperature(value_c)
                .expect("nonpositive linked snow mass-transition fixture should validate"),
            sublimation_m: 0.0,
            post_winter_rain_m: 0.0,
            runtime_swe_after_m: 0.0,
            runtime_depth_after_m: 0.0,
            runtime_density_after_kg_m3: 0.0,
            runtime_settle_day_count_after: 0.0,
            coe_boundary_depth_after_m: 0.0,
            coe_boundary_density_after_kg_m3: 0.0,
            coe_boundary_settle_day_count_after: 0.0,
            snow_albedo_state_after: None,
        }
    }

    #[test]
    fn cqr_water_temperature_accepts_nonpositive_and_ledger_rejects_positive_meltwater() {
        let run_identity = identity(1, 1);
        let mut day = DirectDayFrame::seed(run_identity, 0, 0).expect("temperature day");
        day.snow_coupling_shadow_projection =
            Some(Box::new(snow_projection_with_temperature(-0.25)));
        let operands = direct_publication_water_temperature_operands(&day)
            .expect("nonpositive meltwater temperature");
        assert_eq!(operands.meltwater_temperature_c, Some(-0.25));

        assert_eq!(
            snow_ledgers_with_temperature(0.25)
                .expect_err("positive meltwater must fail at the durable ledger boundary"),
            DirectSnowMassTransitionLedgerError::Stage3Outcome
        );
    }

    #[test]
    fn cqr_storage_operands_fail_closed_in_operand_order() {
        let run_identity = identity(1, 1);
        let mut day = DirectDayFrame::seed(run_identity, 0, 0).expect("storage day");

        day.hydrology_projection.total_soil_m = -1.0;
        day.hydrology_projection.soil_water_total_m = -2.0;
        day.winter_column.snow.runtime_depth_m = -3.0;
        assert!(matches!(
            direct_publication_storage_operands(&day),
            Err(DirectRuntimeError::DirectDomainViolation {
                field: "publication.storage.total_soil_publication_m"
            })
        ));

        day.hydrology_projection.total_soil_m = 0.1;
        assert!(matches!(
            direct_publication_storage_operands(&day),
            Err(DirectRuntimeError::DirectDomainViolation {
                field: "publication.storage.soil_water_total_publication_m"
            })
        ));

        day.hydrology_projection.soil_water_total_m = 0.1;
        assert!(matches!(
            direct_publication_storage_operands(&day),
            Err(DirectRuntimeError::DirectDomainViolation {
                field: "publication.storage.snow_depth_publication_m"
            })
        ));

        day.winter_column.snow.runtime_depth_m = 0.0;
        day.hydrology_projection.frozen_soil_water_m = -0.1;
        assert!(matches!(
            direct_publication_storage_operands(&day),
            Err(DirectRuntimeError::NegativeDirectValue {
                field: "publication.depth_m"
            })
        ));

        day.hydrology_projection.frozen_soil_water_m = 0.0;
        direct_publication_storage_operands(&day).expect("valid storage operands after guards");
    }

    fn seed_single_hour_runoff_and_peak(day: &mut DirectDayFrame, runoff_m: f64) {
        day.runoff_shadow_projection
            .as_mut()
            .expect("runoff producer")
            .q_runoff_m = runoff_m;
        day.wb14_hourly_excess_m[0] = runoff_m;
        let peak = day
            .peak_runoff_shadow_projection
            .as_mut()
            .expect("peak producer");
        peak.q_runoff_m = runoff_m;
        peak.peak_runoff_rate_m_s = runoff_m / 3_600.0;
        peak.runoff_duration_s = 3_600.0;
        peak.peak_hour_index = Some(0);
    }

    #[test]
    fn cqr_day_row_reconstructs_distinct_operands_and_rejects_storage_alias_mismatch() {
        let run_identity = identity(1, 1);
        let mut frame = DirectRunFrame::skeleton(run_identity).expect("row reconstruction frame");
        frame.lanes[0].area_m2 = 200.0;
        frame.lanes[0].runoff_publication_efflen_m = 2.0;
        frame.lanes[0].runoff_publication_cumulative_length_m = 4.0;
        frame.lanes[0].runoff_publication_ofe_length_m = 1.0;
        let lane = frame.lanes[0].clone();
        let input = DirectPublicationDayInput::calendar_only(calendar());
        let mut reconstructed = None;
        DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly)
            .run_publication_stream_with_interleaved_day_inputs_and_day_frames(
                &mut frame,
                DirectPublicationRunMetadata {
                    run_name: "cqr_row_reconstruction".to_string(),
                    runtime_selection: "direct".to_string(),
                    output_policy: "test".to_string(),
                },
                |_, _, _| Ok(input.clone()),
                |_, produced_day| {
                    let mut day = produced_day.clone();
                    day.normalization.precipitation_m = 0.011;
                    day.normalization.surface_transfer_m = 0.009;
                    day.normalization.lateral_transfer_m = 0.010;
                    day.liquid_input.liquid_input_m = 0.013;
                    day.interception_m = 0.002;
                    day.hydrology_projection.q_runoff_m = 0.004;
                    day.hydrology_projection.q_ofe_m = 0.006;
                    day.hydrology_projection.plant_transpiration_m = 0.003;
                    day.hydrology_projection.soil_evaporation_m = 0.005;
                    day.hydrology_projection.residue_evaporation_m = 0.007;
                    day.hydrology_projection.evapotranspiration_m = 0.015;
                    day.hydrology_projection.deep_percolation_m = 0.008;
                    day.hydrology_projection.lateral_flow_m = 0.007;
                    day.hydrology_projection.tile_drainage_m = 0.006;
                    day.hydrology_projection.total_soil_m = 0.021;
                    day.hydrology_projection.soil_water_total_m = 0.019;
                    day.hydrology_projection.frozen_soil_water_m = 0.001;
                    day.hydrology_projection.frost_depth_m = 0.012;
                    day.hydrology_projection.snow_water_m = 0.014;
                    day.winter_column.snow.runtime_depth_m = 0.016;
                    day.groundwater_output = DirectGroundwaterDayOutput {
                        enabled: true,
                        recharge_m3: 1.0,
                        storage_before_m3: 10.0,
                        storage_after_m3: 8.0,
                        storage_delta_m3: -2.0,
                        baseflow_m3: 3.0,
                        deep_seepage_m3: 5.0,
                        baseflow_threshold_area_ha: Some(0.01),
                    };
                    seed_single_hour_runoff_and_peak(&mut day, 0.004);
                    let error = DirectPublicationDayRow::from_day_frame(&day, &input, &lane)
                        .expect_err("distinct WB13 storage aliases must be rejected");
                    assert!(matches!(
                        error,
                        DirectRuntimeError::DirectClosureToleranceExceeded {
                            field: "publication.storage.soil_water_total_alias"
                        }
                    ));
                    day.hydrology_projection.soil_water_total_m =
                        day.hydrology_projection.total_soil_m;
                    reconstructed = Some(
                        DirectPublicationDayRow::from_day_frame(&day, &input, &lane)
                            .expect("canonical equal storage aliases should be accepted"),
                    );
                    Ok(())
                },
            )
            .expect("producer stream for row reconstruction");
        let row = reconstructed.expect("reconstructed row");
        assert!((row.climate.precipitation_mm - 11.0).abs() < f64::EPSILON);
        assert!((row.liquid_input.rm_mm - 15.0).abs() < f64::EPSILON);
        assert!((row.runoff.q_mm - 2.0).abs() < f64::EPSILON);
        assert!((row.runoff.qofe_mm - 2.0).abs() < f64::EPSILON);
        assert!((row.runoff.runvol_m3 - 2.4).abs() < 1.0e-12);
        assert!((row.subsurface.latqcc_mm - 7.0).abs() < f64::EPSILON);
        assert!((row.subsurface.sbrunv_m3 - 1.4).abs() < 1.0e-12);
        assert!((row.subsurface.groundwater_baseflow_mm - 15.0).abs() < f64::EPSILON);
        assert!((row.subsurface.groundwater_deep_seepage_mm - 25.0).abs() < f64::EPSILON);
        assert!((row.storage.total_soil_mm - 21.0).abs() < f64::EPSILON);
        assert!((row.storage.soil_water_total_mm - 21.0).abs() < f64::EPSILON);
        assert!((row.storage.snow_depth_mm - 16.0).abs() < f64::EPSILON);
        let rejected_q_volume_m3 = row.runoff.q_mm * 0.001 * row.area_m2;
        assert!((row.runoff.runvol_m3 - rejected_q_volume_m3).abs() > 1.0);
        assert!((row.subsurface.groundwater_baseflow_m3 - row.subsurface.sbrunv_m3).abs() > 1.0);
        assert_eq!(
            row.storage.total_soil_mm.to_bits(),
            row.storage.soil_water_total_mm.to_bits()
        );
    }
}
