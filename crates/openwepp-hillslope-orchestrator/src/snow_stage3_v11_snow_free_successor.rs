const STAGE3_SNOW_FREE_SUCCESSOR_SCHEMA_V1: u16 = 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct RealV11AcceptedSupportIdentityV1 {
    accepted_slab_sha256: Digest32,
    beginning_complete_owner_set_sha256: Digest32,
    ending_complete_owner_set_sha256: Digest32,
    beginning_snow_owner_sha256: Digest32,
    ending_snow_owner_sha256: Digest32,
}

fn stage3_snow_owner_sha256_v1(
    owners: &[OwnerState],
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    owners
        .iter()
        .find(|owner| owner.owner_id() == "snow")
        .map(OwnerState::state_digest)
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "snow-free successor snow owner",
        ))
}

fn stage3_pending_terminal_parcel_set_sha256_v1(
    parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    let canonical = parcels.iter().collect::<Vec<_>>();
    let bytes = serde_json::to_vec(&canonical).map_err(|_| {
        DirectSnowStage3V11AttachmentError::Identity(
            "snow-free successor pending parcel serialization",
        )
    })?;
    Ok(digest_bytes(&bytes))
}

/// Sealed execution-class evidence for a positive support advanced by the
/// real V11 snow-free successor after sequential Stage-3 state proves that no
/// lane is active. This is chronology evidence, not a model selector.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage3SnowFreeSuccessorReceiptV1 {
    pub schema_version: u16,
    pub day_index: usize,
    pub interval_index: usize,
    pub parent_transaction_id: ParentTransactionId,
    pub support: TimeSupport,
    pub successor_ordinal: u32,
    pub forcing_receipt_sha256: Digest32,
    pub support_forcing_sha256: Digest32,
    pub accepted_slab_sha256: Digest32,
    pub beginning_complete_owner_set_sha256: Digest32,
    pub ending_complete_owner_set_sha256: Digest32,
    pub beginning_snow_owner_sha256: Digest32,
    pub ending_snow_owner_sha256: Digest32,
    pub beginning_pending_terminal_parcel_set_sha256: Digest32,
    pub ending_pending_terminal_parcel_set_sha256: Digest32,
    pub receiver_pending: bool,
    pub receipt_sha256: Digest32,
}

impl Stage3SnowFreeSuccessorReceiptV1 {
    #[allow(clippy::too_many_arguments)]
    fn seal(
        prepared: &DirectSnowStage3V11PreparedSupport,
        day_index: usize,
        interval_index: usize,
        parent_transaction_id: ParentTransactionId,
        successor_ordinal: u32,
        forcing_receipt_sha256: Digest32,
        beginning_pending: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
        ending_pending: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
        accepted: RealV11AcceptedSupportIdentityV1,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        if prepared
            .support_forcing_by_lane
            .values()
            .any(|input| {
                input.forcing.snowfall_m.to_bits() != 0.0_f64.to_bits()
                    || input.forcing.snow_fraction.to_bits() != 0.0_f64.to_bits()
            })
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "snow-free successor received solid precipitation",
            ));
        }
        let receiver_pending = !beginning_pending.is_empty();
        if !ending_pending.is_empty() || (!receiver_pending && beginning_pending != ending_pending) {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "snow-free successor terminal parcel disposition",
            ));
        }
        let mut value = Self {
            schema_version: STAGE3_SNOW_FREE_SUCCESSOR_SCHEMA_V1,
            day_index,
            interval_index,
            parent_transaction_id,
            support: prepared.support,
            successor_ordinal,
            forcing_receipt_sha256,
            support_forcing_sha256: canonical_stage3_support_forcing_digest(
                &prepared.support_forcing_by_lane,
            ),
            accepted_slab_sha256: accepted.accepted_slab_sha256,
            beginning_complete_owner_set_sha256: accepted
                .beginning_complete_owner_set_sha256,
            ending_complete_owner_set_sha256: accepted.ending_complete_owner_set_sha256,
            beginning_snow_owner_sha256: accepted.beginning_snow_owner_sha256,
            ending_snow_owner_sha256: accepted.ending_snow_owner_sha256,
            beginning_pending_terminal_parcel_set_sha256:
                stage3_pending_terminal_parcel_set_sha256_v1(beginning_pending)?,
            ending_pending_terminal_parcel_set_sha256:
                stage3_pending_terminal_parcel_set_sha256_v1(ending_pending)?,
            receiver_pending,
            receipt_sha256: Digest32::zero(),
        };
        value.receipt_sha256 = value.reconstructed_digest()?;
        value.validate()?;
        Ok(value)
    }

    fn reconstructed_digest(&self) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
        let mut value = self.clone();
        value.receipt_sha256 = Digest32::zero();
        let bytes = serde_json::to_vec(&value).map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity(
                "snow-free successor receipt serialization",
            )
        })?;
        Ok(digest_bytes(&bytes))
    }

    fn validate(&self) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if self.schema_version != STAGE3_SNOW_FREE_SUCCESSOR_SCHEMA_V1
            || self.support.duration_ns() == 0
            || self.forcing_receipt_sha256 == Digest32::zero()
            || self.support_forcing_sha256 == Digest32::zero()
            || self.accepted_slab_sha256 == Digest32::zero()
            || self.beginning_complete_owner_set_sha256 == Digest32::zero()
            || self.ending_complete_owner_set_sha256 == Digest32::zero()
            || self.beginning_snow_owner_sha256 == Digest32::zero()
            || self.ending_snow_owner_sha256 == Digest32::zero()
            || self.receipt_sha256 != self.reconstructed_digest()?
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "snow-free successor receipt identity",
            ));
        }
        if self.receiver_pending
            == (self.beginning_pending_terminal_parcel_set_sha256
                == self.ending_pending_terminal_parcel_set_sha256)
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "snow-free successor receiver mode",
            ));
        }
        Ok(())
    }

    fn validate_against_publication(
        &self,
        support: &crate::v9_real_consumer_shadow::Stage3AcceptedPublicationSupportV1,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        self.validate()?;
        if support.day_index() != self.day_index
            || support.interval_index() != self.interval_index
            || support.parent_transaction_id() != self.parent_transaction_id
            || support.support() != self.support
            || support.accepted_slab_sha256() != self.accepted_slab_sha256
            || support.beginning_complete_owner_set_sha256()
                != self.beginning_complete_owner_set_sha256
            || support.ending_complete_owner_set_sha256()
                != self.ending_complete_owner_set_sha256
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "snow-free successor publication cross-join",
            ));
        }
        Ok(())
    }
}
