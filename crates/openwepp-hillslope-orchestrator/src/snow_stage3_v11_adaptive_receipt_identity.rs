const STAGE3_ADAPTIVE_MINIMUM_STEP_NS: u128 = STAGE3_V11_ADAPTIVE_MINIMUM_SUPPORT_NS;

fn adaptive_receipt_identity_error(reason: &'static str) -> DirectSnowStage3V11AttachmentError {
    DirectSnowStage3V11AttachmentError::Identity(reason)
}

fn require_adaptive_digest(
    value: Digest32,
    reason: &'static str,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    if value == Digest32::zero() {
        return Err(adaptive_receipt_identity_error(reason));
    }
    Ok(())
}

fn adaptive_framed_sha256(
    domain: &str,
    fields: Vec<(&'static str, Vec<u8>)>,
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    let framed = fields
        .iter()
        .map(|(tag, value)| FramedField {
            tag,
            value: value.as_slice(),
        })
        .collect::<Vec<_>>();
    framed_sha256(domain, &framed).map_err(DirectSnowStage3V11AttachmentError::from)
}
