fn validate_wb14_provider_bindings(
    receipts: &[crate::runtime_inputs::SnowFreeHalfHourDayReceipt],
    interval_index: usize,
    template: &DirectV9ShadowIntervalInput,
) -> Result<(), DirectV9RealConsumerError> {
    for receipt in receipts {
        let provider = &receipt.intervals[interval_index];
        let parameter = template
            .wb14_parameters
            .iter()
            .find(|value| value.ofe_id.as_str() == provider.ofe_id)
            .ok_or(DirectV9RealConsumerError::Identity(
                "repository WB14 OFE binding",
            ))?;
        if provider.wb14_configuration_sha256 != wb14_parameter_sha256(parameter) {
            return Err(DirectV9RealConsumerError::Identity(
                "repository WB14 configuration receipt",
            ));
        }
    }
    Ok(())
}

fn wb14_parameter_sha256(value: &DirectOfeWb14Parameters) -> String {
    let mut digest = Sha256::new();
    digest.update(value.ofe_id.as_str().as_bytes());
    for operand in [
        value.effective_conductivity_m_s,
        value.matric_potential_m,
        value.infiltration_storage_capacity_m,
    ] {
        digest.update(operand.to_bits().to_le_bytes());
    }
    format!("{:x}", digest.finalize())
}

#[cfg(any(
    feature = "restart-authority-evidence",
    feature = "persisted-restart-v1"
))]
#[must_use]
pub fn restart_authority_wb14_parameter_sha256(value: &DirectOfeWb14Parameters) -> String {
    wb14_parameter_sha256(value)
}

fn validate_global_provider_interval(
    receipts: &[crate::runtime_inputs::SnowFreeHalfHourDayReceipt],
    interval_index: usize,
    expected: &SnowFreeHalfHourIntervalReceipt,
) -> Result<(), DirectV9RealConsumerError> {
    let expected_values = provider_global_values(expected);
    for receipt in receipts {
        let candidate = &receipt.intervals[interval_index];
        if provider_global_values(candidate)
            .iter()
            .zip(expected_values)
            .any(|(left, right)| left.to_bits() != right.to_bits())
            || candidate.co2_pa.to_bits() != expected.co2_pa.to_bits()
            || candidate.reference_height_m.to_bits() != expected.reference_height_m.to_bits()
            || candidate.gsi.to_bits() != expected.gsi.to_bits()
            || candidate.gsi_receipt_sha256 != expected.gsi_receipt_sha256
        {
            return Err(DirectV9RealConsumerError::Unsupported(
                "repository global atmospheric forcing heterogeneity",
            ));
        }
    }
    Ok(())
}

fn provider_global_values(value: &SnowFreeHalfHourIntervalReceipt) -> [f64; 15] {
    [
        value.air_temperature_c,
        value.dew_point_c,
        value.wind_m_s,
        value.pressure_kpa,
        value.actual_vapor_pressure_kpa,
        value.specific_humidity_kg_kg,
        value.vpd_kpa,
        value.cloud_fraction,
        value.solar_zenith_cosine,
        value.global_horizontal_shortwave_w_m2,
        value.direct_visible_w_m2,
        value.diffuse_visible_w_m2,
        value.direct_nir_w_m2,
        value.diffuse_nir_w_m2,
        value.downward_longwave_w_m2,
    ]
}

fn project_lse_atmosphere(
    receipts: &[crate::runtime_inputs::SnowFreeHalfHourDayReceipt],
    interval_index: usize,
    atmospheric: &SnowFreeHalfHourIntervalReceipt,
    forcing: &mut LandSurfaceForcing,
) -> Result<(), DirectV9RealConsumerError> {
    forcing.air_temperature_k = celsius_to_kelvin(atmospheric.air_temperature_c);
    forcing.air_specific_humidity_kg_kg = atmospheric.specific_humidity_kg_kg;
    forcing.air_pressure_pa = kilopascals_to_pascals(atmospheric.pressure_kpa);
    forcing.reference_wind_m_s = atmospheric.wind_m_s;
    forcing.direct_vis_w_m2 = atmospheric.direct_visible_w_m2;
    forcing.diffuse_vis_w_m2 = atmospheric.diffuse_visible_w_m2;
    forcing.direct_nir_w_m2 = atmospheric.direct_nir_w_m2;
    forcing.diffuse_nir_w_m2 = atmospheric.diffuse_nir_w_m2;
    forcing.atmospheric_downward_longwave_w_m2 = atmospheric.downward_longwave_w_m2;
    forcing.precipitation_parcels.clear();
    for receipt in receipts {
        let source = &receipt.intervals[interval_index];
        for parcel in &source.precipitation_parcels {
            forcing.precipitation_parcels.push(project_lse_parcel(
                source,
                parcel,
                forcing.interval_s,
            )?);
        }
    }
    forcing.forcing_sha256 = Sha256Digest::try_new("0".repeat(64))?;
    forcing.forcing_sha256 = forcing.canonical_sha256()?;
    forcing.validate(forcing.transaction_id)?;
    Ok(())
}

fn project_lse_parcel(
    interval: &SnowFreeHalfHourIntervalReceipt,
    parcel: &SnowFreePrecipitationParcelReceipt,
    interval_s: f64,
) -> Result<LiquidParcel, DirectV9RealConsumerError> {
    let interval_start = f64::from(
        u32::try_from(interval.start_s)
            .map_err(|_| DirectV9RealConsumerError::Identity("provider interval support"))?,
    );
    let start_s = parcel.start_s - interval_start;
    let end_s = parcel.end_s - interval_start;
    if start_s < 0.0 || end_s > interval_s {
        return Err(DirectV9RealConsumerError::Identity(
            "provider parcel interval support",
        ));
    }
    let destination_ofe = OfeId::try_new(parcel.destination_ofe_id.clone())?;
    let destination_tile = TileId::try_new(parcel.destination_tile_id.clone())
        .map_err(|_| DirectV9RealConsumerError::Identity("provider parcel tile"))?;
    Ok(LiquidParcel {
        parcel_kind: LiquidParcelKind::Precipitation,
        parcel_id: ParcelId::try_new(format!(
            "{}:{}:{}",
            parcel.parcel_id, parcel.destination_ofe_id, parcel.destination_tile_id
        ))?,
        source_owner_id: ResourceOwnerId::try_new(parcel.source_owner_id.clone())
            .map_err(|_| DirectV9RealConsumerError::Identity("provider parcel owner"))?,
        source_ofe_id: destination_ofe.clone(),
        source_tile_id: destination_tile.clone(),
        destination_ofe_id: destination_ofe,
        destination_tile_id: destination_tile,
        start_s,
        end_s,
        amount_kg_m2_destination_tile_ground: parcel.mass_kg_m2,
        temperature_provider: LiquidTemperatureProvider::HarderPomeroyHourly,
        temperature_k: Some(parcel.temperature_k),
        specific_liquid_enthalpy_j_kg: Some(liquid_specific_enthalpy_j_kg(parcel.temperature_k)),
        source_state_sha256: Some(Sha256Digest::try_new(parcel.source_owner_id.clone())?),
    })
}

fn project_vegetation_atmosphere(
    provider: &SnowFreeHalfHourIntervalReceipt,
    forcing: &mut SnowFreeForcing,
) {
    forcing.air_temperature_k = celsius_to_kelvin(provider.air_temperature_c);
    forcing.pressure_pa = kilopascals_to_pascals(provider.pressure_kpa);
    forcing.co2_pa = provider.co2_pa;
    forcing.vapor_pressure_deficit_kpa = provider.vpd_kpa;
    forcing.wind_m_s = provider.wind_m_s;
    forcing.rain_kg_m2 = provider
        .precipitation_parcels
        .iter()
        .map(|parcel| parcel.mass_kg_m2)
        .fold(0.0, |sum, value| sum + value);
    forcing.direct_par_w_m2 = provider.direct_visible_w_m2;
    forcing.diffuse_par_w_m2 = provider.diffuse_visible_w_m2;
    forcing.direct_nir_w_m2 = provider.direct_nir_w_m2;
    forcing.diffuse_nir_w_m2 = provider.diffuse_nir_w_m2;
    forcing.solar_zenith_cosine = provider.solar_zenith_cosine;
    forcing.longwave_down_w_m2 = provider.downward_longwave_w_m2;
    forcing.specific_humidity = provider.specific_humidity_kg_kg;
    forcing.reference_height_m = provider.reference_height_m;
    forcing.gsi = provider.gsi;
}
