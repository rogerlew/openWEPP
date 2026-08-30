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
        let soil: serde_json::Value = serde_json::from_slice(
            &owners
                .get("soil_thermal")
                .ok_or(V11Error::LseSupportReceipt)?
                .state_bytes,
        )
        .map_err(V11Error::Schema)?;
        if lse
            .get("configuration_sha256")
            .and_then(serde_json::Value::as_str)
            != Some(self.configuration_sha256.as_str())
            || lse.get("state_sha256").and_then(serde_json::Value::as_str)
                != Some(self.beginning_state_sha256.as_str())
            || soil.get("state_sha256").and_then(serde_json::Value::as_str)
                != Some(self.beginning_soil_thermal_state_sha256.as_str())
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
