fn canonical_parent_forcing_digest(
    day_index: usize,
    interval_index: usize,
    accepted_gsi_receipt: Digest32,
    support: &DirectSnowStage3V11PreparedSupport,
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    let v11_forcing_receipt = support.covered_v11_interval.as_ref().map_or(
        support.v11_interval.lse_forcing.forcing_sha256.as_str(),
        |interval| interval.lse_forcing.forcing_sha256.as_str(),
    );
    let base = canonical_parent_forcing_digest_from_parts(
        day_index,
        interval_index,
        accepted_gsi_receipt,
        support.support,
        v11_forcing_receipt,
        &support.support_identity_by_lane,
    )?;
    let (
        stage3_support_forcing_sha256,
        stage3_configuration_sha256,
        covered_v11_forcing_sha256,
        carrier_configuration_sha256,
    ) = support.forcing_projections();
    let mut bytes = Vec::with_capacity(32 + 4 * 32);
    bytes.extend_from_slice(b"OPENWEPP_STAGE3_V11_PARENT_FORCING_COVERED_V1\0");
    bytes.extend_from_slice(base.as_bytes());
    for projection in [
        stage3_support_forcing_sha256,
        stage3_configuration_sha256,
        covered_v11_forcing_sha256,
        carrier_configuration_sha256,
    ] {
        bytes.extend_from_slice(projection.as_bytes());
    }
    Ok(digest_bytes(&bytes))
}

fn canonical_parent_forcing_digest_from_parts(
    day_index: usize,
    interval_index: usize,
    accepted_gsi_receipt: Digest32,
    support: TimeSupport,
    v11_forcing_receipt: &str,
    support_identity_by_lane: &BTreeMap<u32, Vec<PreparedStage3V11SupportIdentityV1>>,
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"OPENWEPP_STAGE3_V11_PARENT_FORCING_V1\0");
    bytes.extend_from_slice(
        &u128::try_from(day_index)
            .map_err(|_| DirectSnowStage3V11AttachmentError::Support("day index width"))?
            .to_be_bytes(),
    );
    bytes.extend_from_slice(
        &u128::try_from(interval_index)
            .map_err(|_| DirectSnowStage3V11AttachmentError::Support("interval index width"))?
            .to_be_bytes(),
    );
    bytes.extend_from_slice(&support.start_ns().get().to_be_bytes());
    bytes.extend_from_slice(&support.end_ns().get().to_be_bytes());
    bytes.extend_from_slice(accepted_gsi_receipt.as_bytes());
    append_framed_bytes(&mut bytes, v11_forcing_receipt.as_bytes());
    for (lane_id, identities) in support_identity_by_lane {
        bytes.extend_from_slice(&u32::to_be_bytes(*lane_id));
        bytes.extend_from_slice(
            &u64::try_from(identities.len())
                .map_err(|_| {
                    DirectSnowStage3V11AttachmentError::Support("support destination count width")
                })?
                .to_be_bytes(),
        );
        for identity in identities {
            append_framed_bytes(&mut bytes, identity.destination_ofe_id.as_bytes());
            append_framed_bytes(&mut bytes, identity.destination_tile_id.as_bytes());
            append_framed_bytes(&mut bytes, identity.wb14_configuration_sha256.as_bytes());
            bytes.extend_from_slice(identity.exposure_identity.as_bytes());
            bytes.extend_from_slice(identity.forcing_receipt_digest.as_bytes());
            bytes.extend_from_slice(
                &u64::try_from(identity.precipitation_parcels.len())
                    .map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Support(
                            "precipitation parcel count width",
                        )
                    })?
                    .to_be_bytes(),
            );
            for parcel in &identity.precipitation_parcels {
                append_framed_bytes(&mut bytes, parcel.parcel_id.as_bytes());
                append_framed_bytes(&mut bytes, parcel.source_owner_id.as_bytes());
                append_framed_bytes(&mut bytes, parcel.destination_ofe_id.as_bytes());
                append_framed_bytes(&mut bytes, parcel.destination_tile_id.as_bytes());
                bytes.extend_from_slice(&parcel.start_s.to_bits().to_be_bytes());
                bytes.extend_from_slice(&parcel.end_s.to_bits().to_be_bytes());
                bytes.extend_from_slice(&parcel.mass_kg_m2.to_bits().to_be_bytes());
                bytes.extend_from_slice(&parcel.temperature_k.to_bits().to_be_bytes());
                bytes.extend_from_slice(&parcel.enthalpy_j_m2.to_bits().to_be_bytes());
            }
            bytes.extend_from_slice(
                &u64::try_from(identity.solid_precipitation_parcels.len())
                    .map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Support(
                            "solid precipitation parcel count width",
                        )
                    })?
                    .to_be_bytes(),
            );
            for parcel in &identity.solid_precipitation_parcels {
                append_framed_bytes(&mut bytes, parcel.parcel_id.as_bytes());
                append_framed_bytes(&mut bytes, parcel.source_owner_id.as_bytes());
                append_framed_bytes(&mut bytes, parcel.destination_ofe_id.as_bytes());
                append_framed_bytes(&mut bytes, parcel.destination_tile_id.as_bytes());
                bytes.extend_from_slice(&parcel.start_s.to_bits().to_be_bytes());
                bytes.extend_from_slice(&parcel.end_s.to_bits().to_be_bytes());
                bytes.extend_from_slice(&parcel.mass_kg_m2.to_bits().to_be_bytes());
                bytes.extend_from_slice(&parcel.temperature_k.to_bits().to_be_bytes());
                bytes.extend_from_slice(&parcel.enthalpy_j_m2.to_bits().to_be_bytes());
            }
        }
    }
    Ok(digest_bytes(&bytes))
}

fn append_framed_bytes(bytes: &mut Vec<u8>, value: &[u8]) {
    bytes.extend_from_slice(&(value.len() as u64).to_be_bytes());
    bytes.extend_from_slice(value);
}
