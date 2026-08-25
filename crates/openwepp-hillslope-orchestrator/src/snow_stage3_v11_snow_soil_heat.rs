/// Sealed custody for the one OFE-ground lower boundary between the bottom
/// represented snow volume and the first ordered OFE soil-thermal node.
#[derive(Clone, Debug, PartialEq)]
pub struct SnowSoilHeatReceiptV1 {
    pub schema_version: u16,
    pub model_identity_sha256: Digest32,
    pub support: TimeSupport,
    pub support_duration_ns: u128,
    pub lane_id: u32,
    pub ofe_id: OfeId,
    pub ofe_ground_basis: bool,
    pub topology_identity_sha256: Digest32,
    pub configuration_identity_sha256: Digest32,
    pub beginning_snow_owner_identity_sha256: Digest32,
    pub beginning_soil_owner_identity_sha256: Digest32,
    pub bottom_snow_layer_id: u32,
    pub first_soil_layer_id: SoilLayerId,
    pub bottom_snow_half_thickness_m: f64,
    pub bottom_snow_conductivity_w_m_k: f64,
    pub top_soil_half_thickness_m: f64,
    pub top_soil_conductivity_w_m_k: f64,
    pub beginning_bottom_snow_temperature_k: f64,
    pub beginning_top_soil_temperature_k: f64,
    pub ending_bottom_snow_temperature_k: f64,
    pub ending_top_soil_temperature_k: f64,
    pub beginning_heat_flux_w_m2_ofe_ground: f64,
    pub ending_heat_flux_w_m2_ofe_ground: f64,
    pub accepted_heat_flux_w_m2_ofe_ground: f64,
    pub accepted_heat_j_m2_ofe_ground: f64,
    pub snow_candidate_heat_j_m2_ofe_ground: f64,
    pub soil_candidate_heat_j_m2_ofe_ground: f64,
    pub snow_candidate_ending_identity_sha256: Digest32,
    pub soil_candidate_ending_identity_sha256: Digest32,
    pub receipt_sha256: Digest32,
}

impl SnowSoilHeatReceiptV1 {
    pub fn seal(mut self) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        self.receipt_sha256 = snow_soil_heat_receipt_digest(&self)?;
        validate_snow_soil_heat_receipt(&self)?;
        Ok(self)
    }
}

/// Reconstruct the positive-downward Crank--Nicolson snow-to-soil heat flux.
#[allow(clippy::too_many_arguments)]
pub fn snow_soil_heat_w_m2_ofe_ground(
    bottom_snow_half_thickness_m: f64,
    bottom_snow_conductivity_w_m_k: f64,
    top_soil_half_thickness_m: f64,
    top_soil_conductivity_w_m_k: f64,
    beginning_bottom_snow_temperature_k: f64,
    beginning_top_soil_temperature_k: f64,
    ending_bottom_snow_temperature_k: f64,
    ending_top_soil_temperature_k: f64,
) -> Result<(f64, f64, f64), DirectSnowStage3V11AttachmentError> {
    let operands = [
        bottom_snow_half_thickness_m,
        bottom_snow_conductivity_w_m_k,
        top_soil_half_thickness_m,
        top_soil_conductivity_w_m_k,
        beginning_bottom_snow_temperature_k,
        beginning_top_soil_temperature_k,
        ending_bottom_snow_temperature_k,
        ending_top_soil_temperature_k,
    ];
    if operands.iter().any(|value| !value.is_finite())
        || bottom_snow_half_thickness_m <= 0.0
        || bottom_snow_conductivity_w_m_k <= 0.0
        || top_soil_half_thickness_m <= 0.0
        || top_soil_conductivity_w_m_k <= 0.0
        || beginning_bottom_snow_temperature_k <= 0.0
        || beginning_top_soil_temperature_k <= 0.0
        || ending_bottom_snow_temperature_k <= 0.0
        || ending_top_soil_temperature_k <= 0.0
    {
        return Err(DirectSnowStage3V11AttachmentError::SnowSoilHeat(
            "nonfinite or nonpositive physical operand",
        ));
    }
    let resistance = bottom_snow_half_thickness_m / bottom_snow_conductivity_w_m_k
        + top_soil_half_thickness_m / top_soil_conductivity_w_m_k;
    if !resistance.is_finite() || resistance <= 0.0 {
        return Err(DirectSnowStage3V11AttachmentError::SnowSoilHeat(
            "interface resistance",
        ));
    }
    let conductance = 1.0 / resistance;
    let beginning =
        conductance * (beginning_bottom_snow_temperature_k - beginning_top_soil_temperature_k);
    let ending = conductance * (ending_bottom_snow_temperature_k - ending_top_soil_temperature_k);
    let accepted = 0.5 * (beginning + ending);
    if !conductance.is_finite()
        || !beginning.is_finite()
        || !ending.is_finite()
        || !accepted.is_finite()
    {
        return Err(DirectSnowStage3V11AttachmentError::SnowSoilHeat(
            "reconstructed heat flux",
        ));
    }
    Ok((beginning, ending, accepted))
}

pub fn validate_snow_soil_heat_receipt(
    receipt: &SnowSoilHeatReceiptV1,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    let fail = |message| DirectSnowStage3V11AttachmentError::SnowSoilHeat(message);
    if receipt.schema_version != 1
        || !receipt.ofe_ground_basis
        || receipt.support_duration_ns != receipt.support.duration_ns()
        || receipt.model_identity_sha256 == Digest32::zero()
        || receipt.topology_identity_sha256 == Digest32::zero()
        || receipt.configuration_identity_sha256 == Digest32::zero()
        || receipt.beginning_snow_owner_identity_sha256 == Digest32::zero()
        || receipt.beginning_soil_owner_identity_sha256 == Digest32::zero()
        || receipt.snow_candidate_ending_identity_sha256 == Digest32::zero()
        || receipt.soil_candidate_ending_identity_sha256 == Digest32::zero()
        || receipt.receipt_sha256 == Digest32::zero()
        || receipt.receipt_sha256 != snow_soil_heat_receipt_digest(receipt)?
    {
        return Err(fail(
            "receipt identity, support, topology, node, basis, or seal",
        ));
    }
    let (beginning, ending, accepted) = snow_soil_heat_w_m2_ofe_ground(
        receipt.bottom_snow_half_thickness_m,
        receipt.bottom_snow_conductivity_w_m_k,
        receipt.top_soil_half_thickness_m,
        receipt.top_soil_conductivity_w_m_k,
        receipt.beginning_bottom_snow_temperature_k,
        receipt.beginning_top_soil_temperature_k,
        receipt.ending_bottom_snow_temperature_k,
        receipt.ending_top_soil_temperature_k,
    )?;
    #[allow(clippy::cast_precision_loss)]
    let duration_s = receipt.support_duration_ns as f64 / 1_000_000_000.0;
    let accepted_heat = accepted * duration_s;
    if receipt.beginning_heat_flux_w_m2_ofe_ground.to_bits() != beginning.to_bits()
        || receipt.ending_heat_flux_w_m2_ofe_ground.to_bits() != ending.to_bits()
        || receipt.accepted_heat_flux_w_m2_ofe_ground.to_bits() != accepted.to_bits()
        || !accepted_heat.is_finite()
        || receipt.accepted_heat_j_m2_ofe_ground.to_bits() != accepted_heat.to_bits()
        || receipt.snow_candidate_heat_j_m2_ofe_ground.to_bits() != (-accepted_heat).to_bits()
        || receipt.soil_candidate_heat_j_m2_ofe_ground.to_bits() != accepted_heat.to_bits()
    {
        return Err(fail(
            "endpoint, Crank--Nicolson, or support-energy reconstruction",
        ));
    }
    Ok(())
}

pub fn validate_snow_soil_heat_receipt_installed_join(
    receipt: &SnowSoilHeatReceiptV1,
    installed_first_soil_layer_id: &SoilLayerId,
    installed_snow_lane_identity_sha256: Digest32,
    installed_soil_ofe_identity_sha256: Digest32,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    validate_snow_soil_heat_receipt(receipt)?;
    if &receipt.first_soil_layer_id != installed_first_soil_layer_id
        || receipt.snow_candidate_ending_identity_sha256 != installed_snow_lane_identity_sha256
        || receipt.soil_candidate_ending_identity_sha256 != installed_soil_ofe_identity_sha256
    {
        return Err(DirectSnowStage3V11AttachmentError::SnowSoilHeat(
            "typed node or canonical installed candidate identity",
        ));
    }
    Ok(())
}

fn snow_soil_heat_receipt_digest(
    receipt: &SnowSoilHeatReceiptV1,
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    let schema = receipt.schema_version.to_be_bytes();
    let start = receipt.support.start_ns().get().to_be_bytes();
    let end = receipt.support.end_ns().get().to_be_bytes();
    let duration = receipt.support_duration_ns.to_be_bytes();
    let lane = receipt.lane_id.to_be_bytes();
    let basis = [u8::from(receipt.ofe_ground_basis)];
    let snow_layer = receipt.bottom_snow_layer_id.to_be_bytes();
    let numeric_bytes = snow_soil_heat_numeric_bytes(receipt);
    framed_sha256(
        "snow-soil-heat-receipt-v1",
        &[
            FramedField {
                tag: "schema",
                value: &schema,
            },
            FramedField {
                tag: "model",
                value: receipt.model_identity_sha256.as_bytes(),
            },
            FramedField {
                tag: "support_start_ns",
                value: &start,
            },
            FramedField {
                tag: "support_end_ns",
                value: &end,
            },
            FramedField {
                tag: "support_duration_ns",
                value: &duration,
            },
            FramedField {
                tag: "lane_id",
                value: &lane,
            },
            FramedField {
                tag: "ofe_id",
                value: receipt.ofe_id.as_str().as_bytes(),
            },
            FramedField {
                tag: "ofe_ground_basis",
                value: &basis,
            },
            FramedField {
                tag: "topology",
                value: receipt.topology_identity_sha256.as_bytes(),
            },
            FramedField {
                tag: "configuration",
                value: receipt.configuration_identity_sha256.as_bytes(),
            },
            FramedField {
                tag: "beginning_snow_owner",
                value: receipt.beginning_snow_owner_identity_sha256.as_bytes(),
            },
            FramedField {
                tag: "beginning_soil_owner",
                value: receipt.beginning_soil_owner_identity_sha256.as_bytes(),
            },
            FramedField {
                tag: "bottom_snow_layer",
                value: &snow_layer,
            },
            FramedField {
                tag: "first_soil_layer",
                value: receipt.first_soil_layer_id.as_str().as_bytes(),
            },
            FramedField {
                tag: "physical_operands_and_results",
                value: &numeric_bytes,
            },
            FramedField {
                tag: "snow_candidate_ending",
                value: receipt.snow_candidate_ending_identity_sha256.as_bytes(),
            },
            FramedField {
                tag: "soil_candidate_ending",
                value: receipt.soil_candidate_ending_identity_sha256.as_bytes(),
            },
        ],
    )
    .map_err(|_| DirectSnowStage3V11AttachmentError::SnowSoilHeat("canonical receipt framing"))
}

fn snow_soil_heat_numeric_bytes(receipt: &SnowSoilHeatReceiptV1) -> Vec<u8> {
    [
        receipt.bottom_snow_half_thickness_m,
        receipt.bottom_snow_conductivity_w_m_k,
        receipt.top_soil_half_thickness_m,
        receipt.top_soil_conductivity_w_m_k,
        receipt.beginning_bottom_snow_temperature_k,
        receipt.beginning_top_soil_temperature_k,
        receipt.ending_bottom_snow_temperature_k,
        receipt.ending_top_soil_temperature_k,
        receipt.beginning_heat_flux_w_m2_ofe_ground,
        receipt.ending_heat_flux_w_m2_ofe_ground,
        receipt.accepted_heat_flux_w_m2_ofe_ground,
        receipt.accepted_heat_j_m2_ofe_ground,
        receipt.snow_candidate_heat_j_m2_ofe_ground,
        receipt.soil_candidate_heat_j_m2_ofe_ground,
    ]
    .iter()
    .flat_map(|value| value.to_bits().to_be_bytes())
    .collect()
}
