#[derive(Clone, Debug, PartialEq)]
pub struct Stage3ParentAtmosphericReceiptV1 {
    pub support: TimeSupport,
    pub provider_interval_receipt_sha256: Digest32,
    pub air_temperature_k: f64,
    pub actual_vapor_pressure_pa: f64,
    pub specific_humidity_kg_kg: f64,
    pub air_pressure_pa: f64,
    pub raw_wind_m_s: f64,
    pub direct_vis_w_m2: f64,
    pub diffuse_vis_w_m2: f64,
    pub direct_nir_w_m2: f64,
    pub diffuse_nir_w_m2: f64,
    pub downward_longwave_w_m2: f64,
    pub receipt_sha256: Digest32,
}

impl Stage3ParentAtmosphericReceiptV1 {
    fn from_provider(
        support: TimeSupport,
        provider: &SnowFreeHalfHourIntervalReceipt,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        let provider_interval_receipt_sha256 =
            parse_lower_hex_digest(&provider.interval_receipt_sha256)?;
        let mut value = Self {
            support,
            provider_interval_receipt_sha256,
            air_temperature_k: celsius_to_kelvin(provider.air_temperature_c),
            actual_vapor_pressure_pa: kilopascals_to_pascals(provider.actual_vapor_pressure_kpa),
            specific_humidity_kg_kg: provider.specific_humidity_kg_kg,
            air_pressure_pa: kilopascals_to_pascals(provider.pressure_kpa),
            raw_wind_m_s: provider.wind_m_s,
            direct_vis_w_m2: provider.direct_visible_w_m2,
            diffuse_vis_w_m2: provider.diffuse_visible_w_m2,
            direct_nir_w_m2: provider.direct_nir_w_m2,
            diffuse_nir_w_m2: provider.diffuse_nir_w_m2,
            downward_longwave_w_m2: provider.downward_longwave_w_m2,
            receipt_sha256: Digest32::zero(),
        };
        value.receipt_sha256 = value.reconstructed_digest();
        value.validate()?;
        Ok(value)
    }

    fn reconstructed_digest(&self) -> Digest32 {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"OPENWEPP_STAGE3_PARENT_ATMOSPHERE_V1\0");
        bytes.extend_from_slice(&self.support.start_ns().get().to_be_bytes());
        bytes.extend_from_slice(&self.support.end_ns().get().to_be_bytes());
        bytes.extend_from_slice(self.provider_interval_receipt_sha256.as_bytes());
        for value in [
            self.air_temperature_k,
            self.actual_vapor_pressure_pa,
            self.specific_humidity_kg_kg,
            self.air_pressure_pa,
            self.raw_wind_m_s,
            self.direct_vis_w_m2,
            self.diffuse_vis_w_m2,
            self.direct_nir_w_m2,
            self.diffuse_nir_w_m2,
            self.downward_longwave_w_m2,
        ] {
            bytes.extend_from_slice(&value.to_bits().to_be_bytes());
        }
        digest_bytes(&bytes)
    }

    fn validate(&self) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if self.provider_interval_receipt_sha256 == Digest32::zero()
            || self.receipt_sha256 != self.reconstructed_digest()
            || !self.air_temperature_k.is_finite()
            || self.air_temperature_k <= 0.0
            || !self.actual_vapor_pressure_pa.is_finite()
            || self.actual_vapor_pressure_pa < 0.0
            || !self.specific_humidity_kg_kg.is_finite()
            || !(0.0..=1.0).contains(&self.specific_humidity_kg_kg)
            || !self.air_pressure_pa.is_finite()
            || self.air_pressure_pa <= self.actual_vapor_pressure_pa
            || !self.raw_wind_m_s.is_finite()
            || self.raw_wind_m_s <= 0.0
            || [
                self.direct_vis_w_m2,
                self.diffuse_vis_w_m2,
                self.direct_nir_w_m2,
                self.diffuse_nir_w_m2,
                self.downward_longwave_w_m2,
            ]
            .iter()
            .any(|value| !value.is_finite() || *value < 0.0)
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "provider atmospheric receipt",
            ));
        }
        Ok(())
    }
}
