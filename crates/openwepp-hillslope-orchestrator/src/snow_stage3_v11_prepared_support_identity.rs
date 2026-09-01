fn append_canonical_bytes(bytes: &mut Vec<u8>, value: &[u8]) {
    bytes.extend_from_slice(&(value.len() as u64).to_be_bytes());
    bytes.extend_from_slice(value);
}

fn append_canonical_str(bytes: &mut Vec<u8>, value: &str) {
    append_canonical_bytes(bytes, value.as_bytes());
}

fn append_canonical_f64(bytes: &mut Vec<u8>, value: f64) {
    bytes.extend_from_slice(&value.to_bits().to_be_bytes());
}

fn append_canonical_option_f64(bytes: &mut Vec<u8>, value: Option<f64>) {
    match value {
        Some(value) => {
            bytes.push(1);
            append_canonical_f64(bytes, value);
        }
        None => bytes.push(0),
    }
}

fn canonical_stage3_support_forcing_digest(
    forcing_by_lane: &BTreeMap<u32, DirectSnowStage3SupportInput>,
) -> Digest32 {
    let mut bytes = Vec::new();
    append_canonical_bytes(&mut bytes, b"OPENWEPP_STAGE3_SUPPORT_FORCING_V2");
    for (lane, support) in forcing_by_lane {
        bytes.extend_from_slice(&lane.to_be_bytes());
        append_canonical_f64(&mut bytes, support.duration_seconds);
        let forcing = support.forcing;
        for value in [
            forcing.active_precipitation_m,
            forcing.rain_m,
            forcing.snowfall_m,
            forcing.radiation_mj_m2,
            forcing.air_temperature_c,
            forcing.cloud_fraction,
            forcing.rain_fraction,
            forcing.snow_fraction,
        ] {
            append_canonical_f64(&mut bytes, value);
        }
        append_canonical_str(&mut bytes, forcing.phase_model.id());
        append_canonical_option_f64(&mut bytes, forcing.hydrometeor_temperature_c);
    }
    digest_bytes(&bytes)
}

fn canonical_stage3_configuration_digest(
    inputs_by_lane: &BTreeMap<u32, DirectActiveSnowPartitionInputs>,
) -> Digest32 {
    let mut bytes = Vec::new();
    append_canonical_bytes(&mut bytes, b"OPENWEPP_STAGE3_CONFIGURATION_V2");
    for (lane, input) in inputs_by_lane {
        bytes.extend_from_slice(&lane.to_be_bytes());
        for value in [
            input.hyetograph_rainfall_m,
            input.rst_c,
            input.newsnw_kg_m3,
            input.ssd_kg_m3,
            input.tmax_c,
            input.tmin_c,
            input.canopy_cover_fraction,
            input.wind_m_s,
            input.dewpoint_c,
            input.coe_boundary_depth_m,
            input.coe_boundary_density_kg_m3,
            input.coe_boundary_settle_day_count,
            input.underlying_surface_albedo,
        ] {
            append_canonical_f64(&mut bytes, value);
        }
        for id in [
            input.snow_melt_model.id(),
            input.snow_density_model.id(),
            input.stage3_liquid_routing_model.id(),
            input.surface_energy_options.longwave_model.id(),
            input.surface_energy_options.sublimation_model.id(),
        ] {
            append_canonical_str(&mut bytes, id);
        }
        if let Some(model) = input.snow_albedo_model {
            bytes.push(1);
            append_canonical_str(&mut bytes, model.id());
        } else {
            bytes.push(0);
        }
        if let Some(class) = input.sturm_climate_class {
            bytes.push(1);
            append_canonical_str(&mut bytes, class.id());
        } else {
            bytes.push(0);
        }
        append_canonical_option_f64(&mut bytes, input.sturm_day_of_year);
        let options = input.surface_energy_options;
        for value in [
            options.daily_solar_radiation_mj_m2,
            options.daily_extraterrestrial_radiation_mj_m2,
            options.atmospheric_pressure_pa,
            options.turbulent_geometry.air_temperature_height_m,
            options.turbulent_geometry.vapor_pressure_height_m,
            options.turbulent_geometry.wind_speed_height_m,
            options.turbulent_geometry.aerodynamic_roughness_length_m,
        ] {
            append_canonical_f64(&mut bytes, value);
        }
        bytes.push(u8::from(options.daylight));
        bytes.push(u8::from(options.complete_carrier_shadow));
    }
    digest_bytes(&bytes)
}

fn canonical_v11_forcing_digest(
    lse_forcing: &openwepp_land_surface_energy::LandSurfaceForcing,
    vegetation_forcing: &openwepp_vegetation::SnowFreeForcing,
) -> Digest32 {
    let mut bytes = Vec::new();
    append_canonical_bytes(&mut bytes, b"OPENWEPP_COVERED_V11_FORCING_V2");
    append_canonical_str(&mut bytes, lse_forcing.forcing_sha256.as_str());
    bytes.extend_from_slice(&lse_forcing.transaction_id.0.to_be_bytes());
    append_canonical_f64(&mut bytes, lse_forcing.interval_s);
    for value in [
        vegetation_forcing.air_temperature_k,
        vegetation_forcing.pressure_pa,
        vegetation_forcing.co2_pa,
        vegetation_forcing.vapor_pressure_deficit_kpa,
        vegetation_forcing.wind_m_s,
        vegetation_forcing.rain_kg_m2,
        vegetation_forcing.direct_par_w_m2,
        vegetation_forcing.diffuse_par_w_m2,
        vegetation_forcing.direct_nir_w_m2,
        vegetation_forcing.diffuse_nir_w_m2,
        vegetation_forcing.solar_zenith_cosine,
        vegetation_forcing.ground_albedo_vis,
        vegetation_forcing.ground_albedo_nir,
        vegetation_forcing.longwave_down_w_m2,
        vegetation_forcing.longwave_up_w_m2,
        vegetation_forcing.specific_humidity,
        vegetation_forcing.reference_height_m,
        vegetation_forcing.gsi,
    ] {
        append_canonical_f64(&mut bytes, value);
    }
    for layer in &vegetation_forcing.soil_layers {
        append_canonical_str(&mut bytes, layer.layer_id.as_str());
        for value in [
            layer.water_beginning_kg_m2,
            layer.matric_potential_mm,
            layer.hydraulic_conductivity_mm_s,
            layer.root_path_length_mm,
            layer.gravity_root_mm,
            layer.temperature_k,
        ] {
            append_canonical_f64(&mut bytes, value);
        }
        bytes.push(u8::from(layer.accessible));
        bytes.push(u8::from(layer.frozen));
    }
    digest_bytes(&bytes)
}

fn canonical_snow_surface_forcing_digest(
    by_destination: &BTreeMap<(OfeId, TileId), SealedStage3TileBoundaryForcingV1>,
) -> Digest32 {
    let mut bytes = Vec::new();
    append_canonical_bytes(&mut bytes, b"OPENWEPP_STAGE3_SNOW_SURFACE_SET_V1");
    for (destination, forcing) in by_destination {
        append_canonical_str(&mut bytes, destination.0.as_str());
        append_canonical_str(&mut bytes, destination.1.as_str());
        match forcing {
            SealedStage3TileBoundaryForcingV1::V11CanopyCovered(forcing) => {
                bytes.push(0);
                bytes.extend_from_slice(forcing.exposure_identity().as_bytes());
            }
            SealedStage3TileBoundaryForcingV1::OpenSnow(forcing) => {
                bytes.push(1);
                bytes.extend_from_slice(forcing.receipt_sha256.as_bytes());
            }
        }
    }
    digest_bytes(&bytes)
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PreparedStage3V11SupportIdentityV1 {
    destination_ofe_id: String,
    destination_tile_id: String,
    wb14_configuration_sha256: String,
    exposure_identity: Digest32,
    precipitation_parcels: Vec<SnowFreePrecipitationParcelReceipt>,
    solid_precipitation_parcels: Vec<SnowFreeSolidPrecipitationParcelReceipt>,
    forcing_receipt_digest: Digest32,
}

impl PreparedStage3V11SupportIdentityV1 {
    /// Project an open destination identity from one repository interval.
    /// Exposure identity is derived from the same raw-wind projection later
    /// sealed by provider binding; callers cannot supply an arbitrary digest.
    pub fn from_provider_open_interval(
        support: TimeSupport,
        interval: &SnowFreeHalfHourIntervalReceipt,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        let interval_start_ns = day_start_ns(interval.day_index)?
            .checked_add(
                u128::try_from(interval.start_s)
                    .map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Support(
                            "provider open interval start width",
                        )
                    })?
                    .checked_mul(1_000_000_000)
                    .ok_or(DirectSnowStage3V11AttachmentError::Support(
                        "provider open interval start overflow",
                    ))?,
            )
            .ok_or(DirectSnowStage3V11AttachmentError::Support(
                "provider open interval day overflow",
            ))?;
        if support.start_ns().get() != interval_start_ns
            || support.duration_ns() != STAGE3_V11_PARENT_SUPPORT_NS
        {
            return Err(DirectSnowStage3V11AttachmentError::Support(
                "provider open interval support",
            ));
        }
        let destination = (
            OfeId::try_new(interval.ofe_id.clone()).map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity("provider open interval OFE")
            })?,
            TileId::try_new(interval.tile_id.clone()).map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity("provider open interval tile")
            })?,
        );
        let exposure = SealedOpenSnowExposureReceiptV1::try_new(
            support,
            destination,
            parse_lower_hex_digest(&interval.interval_receipt_sha256)?,
            parse_lower_hex_digest(&interval.provider_definition_sha256)?,
            interval.wind_m_s,
            digest_bytes(b"OPENWEPP_STAGE3_RAW_WIND_IDENTITY_PROJECTION_V1"),
        )?;
        Ok(Self::new_with_phase_parcels(
            interval.ofe_id.clone(),
            interval.tile_id.clone(),
            interval.wb14_configuration_sha256.clone(),
            exposure.receipt_sha256,
            interval.precipitation_parcels.clone(),
            interval.solid_precipitation_parcels.clone(),
            parse_lower_hex_digest(&interval.interval_receipt_sha256)?,
        ))
    }

    /// Project a covered destination identity from the already sealed V11
    /// carrier capability and one repository interval.
    pub fn from_provider_covered_interval(
        interval: &SnowFreeHalfHourIntervalReceipt,
        forcing: &SealedCoveredCarrierForcing,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        Ok(Self::new_with_phase_parcels(
            interval.ofe_id.clone(),
            interval.tile_id.clone(),
            interval.wb14_configuration_sha256.clone(),
            forcing.exposure_identity(),
            interval.precipitation_parcels.clone(),
            interval.solid_precipitation_parcels.clone(),
            parse_lower_hex_digest(&interval.interval_receipt_sha256)?,
        ))
    }

    #[must_use]
    pub fn new(
        destination_ofe_id: String,
        destination_tile_id: String,
        wb14_configuration_sha256: String,
        exposure_identity: Digest32,
        precipitation_parcels: Vec<SnowFreePrecipitationParcelReceipt>,
        forcing_receipt_digest: Digest32,
    ) -> Self {
        Self {
            destination_ofe_id,
            destination_tile_id,
            wb14_configuration_sha256,
            exposure_identity,
            precipitation_parcels,
            solid_precipitation_parcels: Vec::new(),
            forcing_receipt_digest,
        }
    }

    #[must_use]
    pub fn new_with_phase_parcels(
        destination_ofe_id: String,
        destination_tile_id: String,
        wb14_configuration_sha256: String,
        exposure_identity: Digest32,
        precipitation_parcels: Vec<SnowFreePrecipitationParcelReceipt>,
        solid_precipitation_parcels: Vec<SnowFreeSolidPrecipitationParcelReceipt>,
        forcing_receipt_digest: Digest32,
    ) -> Self {
        Self {
            destination_ofe_id,
            destination_tile_id,
            wb14_configuration_sha256,
            exposure_identity,
            precipitation_parcels,
            solid_precipitation_parcels,
            forcing_receipt_digest,
        }
    }
}
