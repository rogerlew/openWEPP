const DIRECT_V10_SOIL_THERMAL_RESIDENT_V2_SCHEMA: &str =
    "OPENWEPP_DIRECT_V10_SOIL_THERMAL_RESIDENT_V2";
const SOIL_THERMAL_OWNER_V2_TAG: &str = "OPENWEPP_SOIL_THERMAL_OWNER_V2";
const SOIL_THERMAL_OWNER_V2_SCHEMA_SHA256: &str =
    "7877f2a227b0fa98c0c92ae2fb7397744857555fc2f2f77d91a6de327ca88be4";
const EXACT_DYADIC_ENTHALPY_V1_DEFINITION_SHA256: &str =
    "7ceb6e80567a05625b0ac7c33fc8c48ac9a776bab8f9863e02e5a87696714014";
const SOIL_THERMAL_MINIMUM_SUPPORT_NS: u128 = 60_000_000_000;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct DirectV10SoilThermalResidentV2Wire {
    schema: String,
    owner: SoilThermalOwnerEnvelopeV2Wire,
    latest_credit_receipt_sha256: Option<String>,
    expected_operand_set_sha256: Option<String>,
    orchestrator_seal_sha256: Option<String>,
    receipt_free_seal_sha256: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct SoilThermalOwnerEnvelopeV2Wire {
    owner_tag: String,
    schema_sha256: String,
    exact_carry_definition_sha256: String,
    parent_v1_state_sha256: String,
    contract_version: u32,
    model_version: String,
    model_definition_sha256: String,
    run_id: String,
    transaction_id: u128,
    expected_predecessor_transaction_id: Option<u128>,
    support_start_ns: u128,
    support_end_ns: u128,
    receipt_chain_sha256: String,
    state: SoilThermalOwnedStateV2Wire,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct SoilThermalOwnedStateV2Wire {
    owner_id: String,
    configuration_sha256: String,
    state_sha256: String,
    last_accepted_transaction_id: Option<u128>,
    ofes: Vec<SoilThermalOfeStateV2Wire>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct SoilThermalOfeStateV2Wire {
    ofe_id: String,
    ordered_layers: Vec<SoilThermalLayerStateV2Wire>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct SoilThermalLayerStateV2Wire {
    layer_id: String,
    temperature_k: f64,
    enthalpy_hi_j_m2_ofe_ground: f64,
    enthalpy_carry: ExactDyadicEnthalpyWire,
    last_accepted_transaction_id: Option<u128>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct ExactDyadicEnthalpyWire {
    sign: i8,
    coefficient_hex: String,
    exponent2: i32,
}

#[derive(Serialize)]
struct SoilThermalStateDigestBodyV2<'a> {
    owner_tag: &'static str,
    schema_sha256: &'static str,
    exact_carry_definition_sha256: &'static str,
    owner_id: &'a str,
    configuration_sha256: &'a str,
    last_accepted_transaction_id: Option<u128>,
    ofes: &'a [SoilThermalOfeStateV2Wire],
}

fn cpython_json_exponents(bytes: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(bytes.len());
    let mut quoted = false;
    let mut escaped = false;
    let mut index = 0;
    while index < bytes.len() {
        let byte = bytes[index];
        if quoted {
            output.push(byte);
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                quoted = false;
            }
            index += 1;
            continue;
        }
        if byte == b'"' {
            quoted = true;
            output.push(byte);
            index += 1;
            continue;
        }
        if byte == b'e'
            && index + 3 < bytes.len()
            && matches!(bytes[index + 1], b'+' | b'-')
            && bytes[index + 2].is_ascii_digit()
            && !bytes[index + 3].is_ascii_digit()
        {
            output.extend_from_slice(&[byte, bytes[index + 1], b'0', bytes[index + 2]]);
            index += 3;
            continue;
        }
        output.push(byte);
        index += 1;
    }
    output
}

fn valid_exact_dyadic(value: &ExactDyadicEnthalpyWire) -> bool {
    match value.sign {
        0 => value.coefficient_hex == "0" && value.exponent2 == 0,
        -1 | 1 => {
            !value.coefficient_hex.is_empty()
                && value.coefficient_hex.as_bytes()[0] != b'0'
                && value
                    .coefficient_hex
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
                && value.coefficient_hex.as_bytes().last().is_some_and(|byte| {
                    matches!(byte, b'1' | b'3' | b'5' | b'7' | b'9' | b'b' | b'd' | b'f')
                })
        }
        _ => false,
    }
}

fn validate_native_v2_soil_owner(
    value: &DirectV10SoilThermalResidentV2Wire,
) -> Result<String, V11Error> {
    let owner = &value.owner;
    let state = &owner.state;
    let receipt_free = value.receipt_free_seal_sha256.is_some();
    let accepted = value.latest_credit_receipt_sha256.is_some()
        && value.expected_operand_set_sha256.is_some()
        && value.orchestrator_seal_sha256.is_some();
    if value.schema != DIRECT_V10_SOIL_THERMAL_RESIDENT_V2_SCHEMA
        || owner.owner_tag != SOIL_THERMAL_OWNER_V2_TAG
        || owner.schema_sha256 != SOIL_THERMAL_OWNER_V2_SCHEMA_SHA256
        || owner.exact_carry_definition_sha256 != EXACT_DYADIC_ENTHALPY_V1_DEFINITION_SHA256
        || owner.contract_version != 15
        || owner.run_id.trim().is_empty()
        || owner.model_version.trim().is_empty()
        || owner.transaction_id == 0
        || owner.support_start_ns >= owner.support_end_ns
        || owner.support_end_ns - owner.support_start_ns < SOIL_THERMAL_MINIMUM_SUPPORT_NS
        || !matches!(
            state.last_accepted_transaction_id,
            value if value == owner.expected_predecessor_transaction_id
                || value == Some(owner.transaction_id)
        )
        || receipt_free == accepted
        || (!receipt_free
            && (value.latest_credit_receipt_sha256.is_none()
                || value.expected_operand_set_sha256.is_none()
                || value.orchestrator_seal_sha256.is_none()))
        || ![
            owner.schema_sha256.as_str(),
            owner.exact_carry_definition_sha256.as_str(),
            owner.parent_v1_state_sha256.as_str(),
            owner.model_definition_sha256.as_str(),
            owner.receipt_chain_sha256.as_str(),
            state.configuration_sha256.as_str(),
            state.state_sha256.as_str(),
        ]
        .into_iter()
        .all(|digest| is_lower_hex(digest, 64))
        || value
            .latest_credit_receipt_sha256
            .iter()
            .chain(value.expected_operand_set_sha256.iter())
            .chain(value.orchestrator_seal_sha256.iter())
            .chain(value.receipt_free_seal_sha256.iter())
            .any(|digest| !is_lower_hex(digest, 64))
        || state.owner_id.trim().is_empty()
        || state.ofes.is_empty()
    {
        return Err(V11Error::LseSupportReceipt);
    }
    let mut ofes = BTreeSet::new();
    for ofe in &state.ofes {
        if ofe.ofe_id.trim().is_empty()
            || !ofes.insert(ofe.ofe_id.as_str())
            || ofe.ordered_layers.is_empty()
        {
            return Err(V11Error::LseSupportReceipt);
        }
        let mut layers = BTreeSet::new();
        for layer in &ofe.ordered_layers {
            if layer.layer_id.trim().is_empty()
                || !layers.insert(layer.layer_id.as_str())
                || !layer.temperature_k.is_finite()
                || !(200.0..=350.0).contains(&layer.temperature_k)
                || !layer.enthalpy_hi_j_m2_ofe_ground.is_finite()
                || !valid_exact_dyadic(&layer.enthalpy_carry)
                || layer.last_accepted_transaction_id != state.last_accepted_transaction_id
            {
                return Err(V11Error::LseSupportReceipt);
            }
        }
    }
    let digest_body = SoilThermalStateDigestBodyV2 {
        owner_tag: SOIL_THERMAL_OWNER_V2_TAG,
        schema_sha256: SOIL_THERMAL_OWNER_V2_SCHEMA_SHA256,
        exact_carry_definition_sha256: EXACT_DYADIC_ENTHALPY_V1_DEFINITION_SHA256,
        owner_id: &state.owner_id,
        configuration_sha256: &state.configuration_sha256,
        last_accepted_transaction_id: state.last_accepted_transaction_id,
        ofes: &state.ofes,
    };
    let bytes = serde_json::to_vec(&digest_body).map_err(V11Error::Schema)?;
    let digest = format!("{:x}", Sha256::digest(cpython_json_exponents(&bytes)));
    if digest != state.state_sha256 {
        return Err(V11Error::LseSupportReceipt);
    }
    Ok(state.state_sha256.clone())
}

fn validated_soil_beginning_state_sha256(bytes: &[u8]) -> Result<String, V11Error> {
    let value: serde_json::Value = serde_json::from_slice(bytes).map_err(V11Error::Schema)?;
    let object = value.as_object().ok_or(V11Error::LseSupportReceipt)?;
    let has_v2_schema = object.contains_key("schema");
    let has_v2_owner = object.contains_key("owner");
    let top_level = object
        .get("state_sha256")
        .and_then(serde_json::Value::as_str);
    if has_v2_schema || has_v2_owner {
        if !has_v2_schema || !has_v2_owner || top_level.is_some() {
            return Err(V11Error::LseSupportReceipt);
        }
        let native: DirectV10SoilThermalResidentV2Wire =
            serde_json::from_slice(bytes).map_err(V11Error::Schema)?;
        validate_native_v2_soil_owner(&native)
    } else {
        let digest = top_level.ok_or(V11Error::LseSupportReceipt)?;
        if !is_lower_hex(digest, 64) {
            return Err(V11Error::LseSupportReceipt);
        }
        Ok(digest.to_owned())
    }
}

impl V11LseSupportReceiptEnvelope {
    pub fn from_canonical_json(canonical_json: Vec<u8>) -> Result<Self, V11Error> {
        let value: serde_json::Value =
            serde_json::from_slice(&canonical_json).map_err(V11Error::Schema)?;
        let field = |name: &'static str| -> Result<String, V11Error> {
            value
                .get(name)
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned)
                .ok_or(V11Error::LseSupportReceipt)
        };
        let envelope = Self {
            parent_transaction_id: field("parent_transaction_id")?,
            segment_id: field("segment_id")?,
            accepted_slab_id: field("accepted_slab_id")?,
            slab_ordinal: field("slab_ordinal")?,
            support_start_ns: field("support_start_ns")?,
            support_end_ns: field("support_end_ns")?,
            requested_support_ns: field("requested_support_ns")?,
            duration_s_bits: field("duration_s_bits")?,
            configuration_sha256: field("configuration_sha256")?,
            beginning_state_sha256: field("beginning_state_sha256")?,
            beginning_soil_thermal_state_sha256: field("beginning_soil_thermal_state_sha256")?,
            receipt_sha256: field("receipt_sha256")?,
            canonical_bytes_sha256: format!("{:x}", Sha256::digest(&canonical_json)),
            canonical_json,
        };
        envelope.validate_closed_bytes()?;
        Ok(envelope)
    }

    fn validate_closed_bytes(&self) -> Result<(), V11Error> {
        if format!("{:x}", Sha256::digest(&self.canonical_json)) != self.canonical_bytes_sha256
            || !is_lower_hex(&self.receipt_sha256, 64)
            || !is_lower_hex(&self.canonical_bytes_sha256, 64)
            || !is_lower_hex(&self.configuration_sha256, 64)
            || !is_lower_hex(&self.beginning_state_sha256, 64)
            || !is_lower_hex(&self.beginning_soil_thermal_state_sha256, 64)
        {
            return Err(V11Error::LseSupportReceipt);
        }
        let wire: LseSupportReceiptWire =
            serde_json::from_slice(&self.canonical_json).map_err(V11Error::Schema)?;
        if serde_json::to_vec(&wire).map_err(V11Error::Schema)? != self.canonical_json
            || wire.receipt_sha256 != self.receipt_sha256
            || wire.model_version != "OPENWEPP_SNOW_FREE_LSE_V1"
            || wire.model_definition_sha256
                != "e1736b8c77d13d6fb12fb97a6f747e54eea877edf237817b6c6e8954cff8332f"
            || wire.tolerance_policy_sha256
                != format!(
                    "{:x}",
                    Sha256::digest(b"energy_absolute=1e-6;energy_relative=1e-10")
                )
            || wire.numerical_policy_sha256
                != format!(
                    "{:x}",
                    Sha256::digest(b"iterations=50;backtracking=0..20;strict-decrease")
                )
            || wire.minimum_support_ns != "60000000000"
        {
            return Err(V11Error::LseSupportReceipt);
        }
        let mut blank = wire;
        blank.receipt_sha256.clear();
        let mut preimage = b"OPENWEPP_LSE_SUPPORT_ADMISSION_V1\0".to_vec();
        preimage.extend(serde_json::to_vec(&blank).map_err(V11Error::Schema)?);
        if format!("{:x}", Sha256::digest(preimage)) != self.receipt_sha256 {
            return Err(V11Error::LseSupportReceipt);
        }
        let reconstructed = Self::from_json_without_recursion(&self.canonical_json)?;
        if reconstructed != *self {
            return Err(V11Error::LseSupportReceipt);
        }
        Ok(())
    }

    fn from_json_without_recursion(bytes: &[u8]) -> Result<Self, V11Error> {
        let value: serde_json::Value = serde_json::from_slice(bytes).map_err(V11Error::Schema)?;
        let field = |name: &'static str| -> Result<String, V11Error> {
            value
                .get(name)
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned)
                .ok_or(V11Error::LseSupportReceipt)
        };
        Ok(Self {
            parent_transaction_id: field("parent_transaction_id")?,
            segment_id: field("segment_id")?,
            accepted_slab_id: field("accepted_slab_id")?,
            slab_ordinal: field("slab_ordinal")?,
            support_start_ns: field("support_start_ns")?,
            support_end_ns: field("support_end_ns")?,
            requested_support_ns: field("requested_support_ns")?,
            duration_s_bits: field("duration_s_bits")?,
            configuration_sha256: field("configuration_sha256")?,
            beginning_state_sha256: field("beginning_state_sha256")?,
            beginning_soil_thermal_state_sha256: field("beginning_soil_thermal_state_sha256")?,
            receipt_sha256: field("receipt_sha256")?,
            canonical_bytes_sha256: format!("{:x}", Sha256::digest(bytes)),
            canonical_json: bytes.to_vec(),
        })
    }

    pub fn validate_join(&self, slab: &AcceptedSlabReceiptV1) -> Result<(), V11Error> {
        self.validate_closed_bytes()?;
        let support = slab.support();
        if self.parent_transaction_id != digest_hex(slab.parent_transaction_id().digest())
            || self.segment_id != digest_hex(slab.segment_id().digest())
            || self.accepted_slab_id != digest_hex(slab.slab_id().digest())
            || self.slab_ordinal != slab.slab_ordinal().to_string()
            || self.support_start_ns != support.start_ns().get().to_string()
            || self.support_end_ns != support.end_ns().get().to_string()
            || self.requested_support_ns != support.duration_ns().to_string()
            || self.duration_s_bits != format!("{:016x}", support.duration_s_bits())
        {
            return Err(V11Error::LseSupportReceipt);
        }
        Ok(())
    }

    fn validate_beginning_owners(
        &self,
        owners: &BTreeMap<String, V11OwnerEnvelope>,
    ) -> Result<(), V11Error> {
        let lse: serde_json::Value = serde_json::from_slice(
            &owners
                .get("land_surface_energy")
                .ok_or(V11Error::LseSupportReceipt)?
                .state_bytes,
        )
        .map_err(V11Error::Schema)?;
        let soil_state_sha256 = validated_soil_beginning_state_sha256(
            &owners
                .get("soil_thermal")
                .ok_or(V11Error::LseSupportReceipt)?
                .state_bytes,
        )?;
        if lse
            .get("configuration_sha256")
            .and_then(serde_json::Value::as_str)
            != Some(self.configuration_sha256.as_str())
            || lse.get("state_sha256").and_then(serde_json::Value::as_str)
                != Some(self.beginning_state_sha256.as_str())
            || soil_state_sha256 != self.beginning_soil_thermal_state_sha256
        {
            return Err(V11Error::LseSupportReceipt);
        }
        Ok(())
    }

    fn validate_checkpoint_join(
        &self,
        segment: &V11AcceptedSegmentCheckpoint,
    ) -> Result<(), V11Error> {
        self.validate_closed_bytes()?;
        if self.parent_transaction_id != digest_hex(segment.parent_transaction_id.digest())
            || self.segment_id != digest_hex(segment.segment_id.digest())
            || self.accepted_slab_id != digest_hex(segment.slab_id.digest())
            || self.slab_ordinal != segment.slab_ordinal.to_string()
            || self.support_start_ns != segment.support.start_ns().get().to_string()
            || self.support_end_ns != segment.support.end_ns().get().to_string()
            || self.requested_support_ns != segment.support.duration_ns().to_string()
            || self.duration_s_bits != format!("{:016x}", segment.duration_s_bits)
        {
            return Err(V11Error::LseSupportReceipt);
        }
        Ok(())
    }
}
