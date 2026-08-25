#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Stage3PrecipitationPhaseV1 {
    Solid,
    Liquid,
}

impl Stage3PrecipitationPhaseV1 {
    const fn rank(self) -> u8 {
        match self {
            Self::Solid => 0,
            Self::Liquid => 1,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Stage3PrecipitationSourceV1 {
    AtmosphericGroundSnow,
    OpenRawRain,
    VegetationTerminalThroughfall,
    VegetationTerminalInitialDrainage,
    VegetationTerminalSecondDrainage,
    VegetationTerminalStemflow,
}

impl Stage3PrecipitationSourceV1 {
    const fn rank(self) -> u8 {
        match self {
            Self::AtmosphericGroundSnow => 0,
            Self::OpenRawRain => 1,
            Self::VegetationTerminalThroughfall => 2,
            Self::VegetationTerminalInitialDrainage => 3,
            Self::VegetationTerminalSecondDrainage => 4,
            Self::VegetationTerminalStemflow => 5,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stage3PrecipitationEnthalpyProviderV1 {
    Temperature {
        temperature_k: f64,
        reference_temperature_k: f64,
        specific_heat_j_kg_k: f64,
        provider_receipt_sha256: Digest32,
    },
    SpecificEnthalpy {
        specific_enthalpy_j_kg: f64,
        provider_receipt_sha256: Digest32,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Stage3PrecipitationDestinationV1 {
    pub topology_index: u32,
    pub ofe_id: OfeId,
    pub tile_id: TileId,
    pub fraction_of_ofe: f64,
    pub canopy_covered: bool,
    pub destination_identity_sha256: Digest32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Stage3PrecipitationPhaseParcelV1 {
    pub support: TimeSupport,
    pub lane_id: u32,
    pub destination_topology_index: u32,
    pub destination_ofe_id: OfeId,
    pub destination_tile_id: TileId,
    pub phase: Stage3PrecipitationPhaseV1,
    pub source: Stage3PrecipitationSourceV1,
    /// Producer-declared semantic order within one destination/phase/source
    /// route. Receipt hashes authenticate this identity but never order it.
    pub semantic_receipt_ordinal: u32,
    pub mass_kg_m2_tile_ground: f64,
    pub enthalpy_provider: Stage3PrecipitationEnthalpyProviderV1,
    pub source_identity_sha256: Digest32,
    pub producer_beginning_state_sha256: Digest32,
    pub receipt_sha256: Digest32,
}

impl Stage3PrecipitationPhaseParcelV1 {
    pub fn seal(mut self) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        self.receipt_sha256 = precipitation_phase_parcel_digest(&self);
        Ok(self)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Stage3PrecipitationPhaseParcelSetV1 {
    pub schema_version: u16,
    pub support: TimeSupport,
    pub lane_id: u32,
    pub ofe_id: OfeId,
    pub ofe_ground_basis: bool,
    pub beginning_snow_state_sha256: Digest32,
    pub topology_identity_sha256: Digest32,
    pub destinations: Vec<Stage3PrecipitationDestinationV1>,
    pub parcels: Vec<Stage3PrecipitationPhaseParcelV1>,
    pub receipt_sha256: Digest32,
}

impl Stage3PrecipitationPhaseParcelSetV1 {
    /// Seal a fully populated semantic record. Validation remains mandatory at
    /// the consumer because a digest authenticates bytes, not their meaning.
    pub fn seal(mut self) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        self.receipt_sha256 = precipitation_parcel_set_digest(&self)?;
        validate_precipitation_phase_parcel_set(&self)?;
        Ok(self)
    }
}

/// Reconstruct one parcel's precipitation-advection energy on its declared
/// tile-ground basis. A temperature provider must declare the complete
/// reference and heat-capacity operands; no implicit water/ice constant is
/// inserted at this custody boundary.
pub fn precipitation_advected_heat_j_m2_tile_ground(
    parcel: &Stage3PrecipitationPhaseParcelV1,
) -> Result<f64, DirectSnowStage3V11AttachmentError> {
    let specific_enthalpy = match parcel.enthalpy_provider {
        Stage3PrecipitationEnthalpyProviderV1::Temperature {
            temperature_k,
            reference_temperature_k,
            specific_heat_j_kg_k,
            provider_receipt_sha256,
        } => {
            if provider_receipt_sha256 == Digest32::zero()
                || !temperature_k.is_finite()
                || temperature_k <= 0.0
                || !reference_temperature_k.is_finite()
                || reference_temperature_k <= 0.0
                || !specific_heat_j_kg_k.is_finite()
                || specific_heat_j_kg_k <= 0.0
            {
                return Err(DirectSnowStage3V11AttachmentError::Precipitation(
                    "temperature enthalpy provider",
                ));
            }
            specific_heat_j_kg_k * (temperature_k - reference_temperature_k)
        }
        Stage3PrecipitationEnthalpyProviderV1::SpecificEnthalpy {
            specific_enthalpy_j_kg,
            provider_receipt_sha256,
        } => {
            if provider_receipt_sha256 == Digest32::zero() || !specific_enthalpy_j_kg.is_finite() {
                return Err(DirectSnowStage3V11AttachmentError::Precipitation(
                    "specific-enthalpy provider",
                ));
            }
            specific_enthalpy_j_kg
        }
    };
    if !parcel.mass_kg_m2_tile_ground.is_finite()
        || parcel.mass_kg_m2_tile_ground < 0.0
        || parcel.mass_kg_m2_tile_ground == 0.0 && parcel.mass_kg_m2_tile_ground.is_sign_negative()
    {
        return Err(DirectSnowStage3V11AttachmentError::Precipitation(
            "parcel mass",
        ));
    }
    let heat = parcel.mass_kg_m2_tile_ground * specific_enthalpy;
    if !heat.is_finite() {
        return Err(DirectSnowStage3V11AttachmentError::Precipitation(
            "parcel advected heat",
        ));
    }
    Ok(heat)
}

pub fn validate_precipitation_phase_parcel_set(
    set: &Stage3PrecipitationPhaseParcelSetV1,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    let fail = |message| DirectSnowStage3V11AttachmentError::Precipitation(message);
    if set.schema_version != 1
        || !set.ofe_ground_basis
        || set.beginning_snow_state_sha256 == Digest32::zero()
        || set.topology_identity_sha256 == Digest32::zero()
        || set.destinations.is_empty()
        || set.receipt_sha256 == Digest32::zero()
        || set.receipt_sha256 != precipitation_parcel_set_digest(set)?
    {
        return Err(fail("parcel-set identity or seal"));
    }
    let mut fraction_sum = 0.0;
    for (ordinal, destination) in set.destinations.iter().enumerate() {
        let expected_index = u32::try_from(ordinal).map_err(|_| fail("topology width"))?;
        if destination.topology_index != expected_index
            || destination.ofe_id != set.ofe_id
            || !destination.fraction_of_ofe.is_finite()
            || destination.fraction_of_ofe <= 0.0
            || destination.fraction_of_ofe > 1.0
            || destination.destination_identity_sha256 == Digest32::zero()
        {
            return Err(fail("destination topology"));
        }
        fraction_sum += destination.fraction_of_ofe;
    }
    if !fraction_sum.is_finite() || (fraction_sum - 1.0).abs() > 1.0e-12 {
        return Err(fail("OFE tile-fraction closure"));
    }
    for pair in set.parcels.windows(2) {
        if precipitation_parcel_key(&pair[0]) >= precipitation_parcel_key(&pair[1]) {
            return Err(fail("canonical parcel order or duplicate key"));
        }
    }
    for parcel in &set.parcels {
        let destination = set
            .destinations
            .get(parcel.destination_topology_index as usize)
            .ok_or_else(|| fail("parcel destination index"))?;
        if parcel.support != set.support
            || parcel.lane_id != set.lane_id
            || parcel.destination_ofe_id != set.ofe_id
            || parcel.destination_ofe_id != destination.ofe_id
            || parcel.destination_tile_id != destination.tile_id
            || parcel.source_identity_sha256 == Digest32::zero()
            || parcel.producer_beginning_state_sha256 == Digest32::zero()
            || parcel.receipt_sha256 == Digest32::zero()
            || parcel.receipt_sha256 != precipitation_phase_parcel_digest(parcel)
            || !parcel.mass_kg_m2_tile_ground.is_finite()
            || parcel.mass_kg_m2_tile_ground < 0.0
            || parcel.mass_kg_m2_tile_ground == 0.0
                && parcel.mass_kg_m2_tile_ground.is_sign_negative()
        {
            return Err(fail("parcel identity, support owner, or mass basis"));
        }
        let valid_source = matches!(
            (parcel.phase, parcel.source, destination.canopy_covered),
            (
                Stage3PrecipitationPhaseV1::Solid,
                Stage3PrecipitationSourceV1::AtmosphericGroundSnow,
                _,
            ) | (
                Stage3PrecipitationPhaseV1::Liquid,
                Stage3PrecipitationSourceV1::OpenRawRain,
                false,
            ) | (
                Stage3PrecipitationPhaseV1::Liquid,
                Stage3PrecipitationSourceV1::VegetationTerminalThroughfall
                    | Stage3PrecipitationSourceV1::VegetationTerminalInitialDrainage
                    | Stage3PrecipitationSourceV1::VegetationTerminalSecondDrainage
                    | Stage3PrecipitationSourceV1::VegetationTerminalStemflow,
                true,
            )
        );
        if !valid_source {
            return Err(fail("phase/source/destination exclusivity"));
        }
        precipitation_advected_heat_j_m2_tile_ground(parcel)?;
    }
    let (reconstructed_mass, _) = reconstruct_precipitation_mass_and_advected_heat(set)?;
    if !set.parcels.is_empty() && reconstructed_mass.to_bits() == 0.0_f64.to_bits() {
        return Err(fail("nonempty zero-precipitation parcel set"));
    }
    Ok(())
}

fn precipitation_phase_parcel_digest(parcel: &Stage3PrecipitationPhaseParcelV1) -> Digest32 {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"openwepp.stage3-precipitation-phase-parcel.v1\0");
    bytes.extend_from_slice(&parcel.support.start_ns().get().to_be_bytes());
    bytes.extend_from_slice(&parcel.support.end_ns().get().to_be_bytes());
    bytes.extend_from_slice(&parcel.lane_id.to_be_bytes());
    bytes.extend_from_slice(&parcel.destination_topology_index.to_be_bytes());
    bytes.extend_from_slice(parcel.destination_ofe_id.as_str().as_bytes());
    bytes.push(0);
    bytes.extend_from_slice(parcel.destination_tile_id.as_str().as_bytes());
    bytes.push(0);
    bytes.push(parcel.phase.rank());
    bytes.push(parcel.source.rank());
    bytes.extend_from_slice(&parcel.semantic_receipt_ordinal.to_be_bytes());
    bytes.extend_from_slice(&parcel.mass_kg_m2_tile_ground.to_bits().to_be_bytes());
    match parcel.enthalpy_provider {
        Stage3PrecipitationEnthalpyProviderV1::Temperature {
            temperature_k,
            reference_temperature_k,
            specific_heat_j_kg_k,
            provider_receipt_sha256,
        } => {
            bytes.push(0);
            bytes.extend_from_slice(&temperature_k.to_bits().to_be_bytes());
            bytes.extend_from_slice(&reference_temperature_k.to_bits().to_be_bytes());
            bytes.extend_from_slice(&specific_heat_j_kg_k.to_bits().to_be_bytes());
            bytes.extend_from_slice(provider_receipt_sha256.as_bytes());
        }
        Stage3PrecipitationEnthalpyProviderV1::SpecificEnthalpy {
            specific_enthalpy_j_kg,
            provider_receipt_sha256,
        } => {
            bytes.push(1);
            bytes.extend_from_slice(&specific_enthalpy_j_kg.to_bits().to_be_bytes());
            bytes.extend_from_slice(provider_receipt_sha256.as_bytes());
        }
    }
    bytes.extend_from_slice(parcel.source_identity_sha256.as_bytes());
    bytes.extend_from_slice(parcel.producer_beginning_state_sha256.as_bytes());
    digest_bytes(&bytes)
}

pub fn reconstruct_precipitation_mass_and_advected_heat(
    set: &Stage3PrecipitationPhaseParcelSetV1,
) -> Result<(f64, f64), DirectSnowStage3V11AttachmentError> {
    let mut mass = 0.0;
    let mut heat = 0.0;
    for parcel in &set.parcels {
        let fraction = set
            .destinations
            .get(parcel.destination_topology_index as usize)
            .ok_or(DirectSnowStage3V11AttachmentError::Precipitation(
                "advection destination index",
            ))?
            .fraction_of_ofe;
        mass += fraction * parcel.mass_kg_m2_tile_ground;
        heat += fraction * precipitation_advected_heat_j_m2_tile_ground(parcel)?;
    }
    if !mass.is_finite() || !heat.is_finite() {
        return Err(DirectSnowStage3V11AttachmentError::Precipitation(
            "lane precipitation reconstruction",
        ));
    }
    Ok((mass, heat))
}

fn precipitation_parcel_key(parcel: &Stage3PrecipitationPhaseParcelV1) -> (u32, u32, u8, u8, u32) {
    (
        parcel.lane_id,
        parcel.destination_topology_index,
        parcel.phase.rank(),
        parcel.source.rank(),
        parcel.semantic_receipt_ordinal,
    )
}

fn precipitation_parcel_set_digest(
    set: &Stage3PrecipitationPhaseParcelSetV1,
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"openwepp.stage3-precipitation-phase-parcel-set.v1\0");
    bytes.extend_from_slice(&set.schema_version.to_be_bytes());
    bytes.extend_from_slice(&set.support.start_ns().get().to_be_bytes());
    bytes.extend_from_slice(&set.support.end_ns().get().to_be_bytes());
    bytes.extend_from_slice(&set.lane_id.to_be_bytes());
    bytes.extend_from_slice(set.ofe_id.as_str().as_bytes());
    bytes.push(u8::from(set.ofe_ground_basis));
    bytes.extend_from_slice(set.beginning_snow_state_sha256.as_bytes());
    bytes.extend_from_slice(set.topology_identity_sha256.as_bytes());
    let destination_count = u64::try_from(set.destinations.len()).map_err(|_| {
        DirectSnowStage3V11AttachmentError::Precipitation("destination count width")
    })?;
    bytes.extend_from_slice(&destination_count.to_be_bytes());
    for destination in &set.destinations {
        bytes.extend_from_slice(&destination.topology_index.to_be_bytes());
        bytes.extend_from_slice(destination.ofe_id.as_str().as_bytes());
        bytes.push(0);
        bytes.extend_from_slice(destination.tile_id.as_str().as_bytes());
        bytes.push(0);
        bytes.extend_from_slice(&destination.fraction_of_ofe.to_bits().to_be_bytes());
        bytes.push(u8::from(destination.canopy_covered));
        bytes.extend_from_slice(destination.destination_identity_sha256.as_bytes());
    }
    let parcel_count = u64::try_from(set.parcels.len())
        .map_err(|_| DirectSnowStage3V11AttachmentError::Precipitation("parcel count width"))?;
    bytes.extend_from_slice(&parcel_count.to_be_bytes());
    for parcel in &set.parcels {
        bytes.extend_from_slice(&parcel.support.start_ns().get().to_be_bytes());
        bytes.extend_from_slice(&parcel.support.end_ns().get().to_be_bytes());
        bytes.extend_from_slice(&parcel.lane_id.to_be_bytes());
        bytes.extend_from_slice(&parcel.destination_topology_index.to_be_bytes());
        bytes.extend_from_slice(parcel.destination_ofe_id.as_str().as_bytes());
        bytes.push(0);
        bytes.extend_from_slice(parcel.destination_tile_id.as_str().as_bytes());
        bytes.push(0);
        bytes.push(parcel.phase.rank());
        bytes.push(parcel.source.rank());
        bytes.extend_from_slice(&parcel.semantic_receipt_ordinal.to_be_bytes());
        bytes.extend_from_slice(&parcel.mass_kg_m2_tile_ground.to_bits().to_be_bytes());
        match parcel.enthalpy_provider {
            Stage3PrecipitationEnthalpyProviderV1::Temperature {
                temperature_k,
                reference_temperature_k,
                specific_heat_j_kg_k,
                provider_receipt_sha256,
            } => {
                bytes.push(0);
                bytes.extend_from_slice(&temperature_k.to_bits().to_be_bytes());
                bytes.extend_from_slice(&reference_temperature_k.to_bits().to_be_bytes());
                bytes.extend_from_slice(&specific_heat_j_kg_k.to_bits().to_be_bytes());
                bytes.extend_from_slice(provider_receipt_sha256.as_bytes());
            }
            Stage3PrecipitationEnthalpyProviderV1::SpecificEnthalpy {
                specific_enthalpy_j_kg,
                provider_receipt_sha256,
            } => {
                bytes.push(1);
                bytes.extend_from_slice(&specific_enthalpy_j_kg.to_bits().to_be_bytes());
                bytes.extend_from_slice(provider_receipt_sha256.as_bytes());
            }
        }
        bytes.extend_from_slice(parcel.source_identity_sha256.as_bytes());
        bytes.extend_from_slice(parcel.producer_beginning_state_sha256.as_bytes());
        bytes.extend_from_slice(parcel.receipt_sha256.as_bytes());
    }
    Ok(digest_bytes(&bytes))
}
