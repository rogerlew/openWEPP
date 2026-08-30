#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnowStage3V11QualificationSurfaceReceiptOccurrenceV1 {
    pub accepted_support_receipt_sha256: Digest32,
    pub accepted_support_start_ns: u128,
    pub accepted_support_end_ns: u128,
    pub accepted_support_interval_index: usize,
    pub source_receipt_ordinal: usize,
    pub source_receipt_sha256: Digest32,
    pub receipt_sha256: Digest32,
}

impl SnowStage3V11QualificationSurfaceReceiptOccurrenceV1 {
    fn custody_identity_sha256(&self) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
        framed_sha256(
            "snow-stage3-v11-qualified-surface-receipt-custody-v1",
            &[
                FramedField {
                    tag: "accepted_support_receipt_sha256",
                    value: self.accepted_support_receipt_sha256.as_bytes(),
                },
                FramedField {
                    tag: "source_receipt_sha256",
                    value: self.source_receipt_sha256.as_bytes(),
                },
            ],
        )
        .map_err(|_| qualification_error("qualification surface custody framing"))
    }

    fn reconstructed_receipt_sha256(&self) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
        let custody = self.custody_identity_sha256()?;
        let interval = u64::try_from(self.accepted_support_interval_index)
            .map_err(|_| qualification_error("qualification surface interval width"))?
            .to_be_bytes();
        let ordinal = u64::try_from(self.source_receipt_ordinal)
            .map_err(|_| qualification_error("qualification surface receipt ordinal width"))?
            .to_be_bytes();
        framed_sha256(
            "snow-stage3-v11-qualified-surface-receipt-occurrence-v1",
            &[
                FramedField {
                    tag: "custody_identity_sha256",
                    value: custody.as_bytes(),
                },
                FramedField {
                    tag: "accepted_support_start_ns",
                    value: &self.accepted_support_start_ns.to_be_bytes(),
                },
                FramedField {
                    tag: "accepted_support_end_ns",
                    value: &self.accepted_support_end_ns.to_be_bytes(),
                },
                FramedField {
                    tag: "accepted_support_interval_index",
                    value: &interval,
                },
                FramedField {
                    tag: "source_receipt_ordinal",
                    value: &ordinal,
                },
            ],
        )
        .map_err(|_| qualification_error("qualification surface occurrence framing"))
    }

    fn try_new(
        accepted_support_receipt_sha256: Digest32,
        accepted_support_start_ns: u128,
        accepted_support_end_ns: u128,
        accepted_support_interval_index: usize,
        source_receipt_ordinal: usize,
        source_receipt_sha256: Digest32,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        let mut value = Self {
            accepted_support_receipt_sha256,
            accepted_support_start_ns,
            accepted_support_end_ns,
            accepted_support_interval_index,
            source_receipt_ordinal,
            source_receipt_sha256,
            receipt_sha256: Digest32::zero(),
        };
        value.receipt_sha256 = value.reconstructed_receipt_sha256()?;
        value.validate()?;
        Ok(value)
    }

    fn validate(&self) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if self.accepted_support_receipt_sha256 == Digest32::zero()
            || self.source_receipt_sha256 == Digest32::zero()
            || self.receipt_sha256 == Digest32::zero()
            || self.accepted_support_start_ns >= self.accepted_support_end_ns
            || self.receipt_sha256 != self.reconstructed_receipt_sha256()?
        {
            return Err(qualification_record_identity_error(
                "surface_receipt_occurrences",
                "invalid occurrence",
                self.receipt_sha256,
                0,
                None,
                Some(self.accepted_support_receipt_sha256),
                Some(self.source_receipt_sha256),
            ));
        }
        Ok(())
    }
}

fn qualification_record_identity_error(
    vector: &'static str,
    failure: &'static str,
    digest: Digest32,
    first_index: usize,
    duplicate_index: Option<usize>,
    source_support_receipt_sha256: Option<Digest32>,
    source_receipt_sha256: Option<Digest32>,
) -> DirectSnowStage3V11AttachmentError {
    DirectSnowStage3V11AttachmentError::QualificationOrderedRecordIdentity {
        vector,
        failure,
        digest,
        first_index,
        duplicate_index,
        source_support_receipt_sha256,
        source_receipt_sha256,
    }
}

fn validate_qualification_digest_records(
    vector: &'static str,
    records: &[Digest32],
    records_are_supports: bool,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    let mut first_by_digest = BTreeMap::new();
    for (index, digest) in records.iter().copied().enumerate() {
        let support = records_are_supports.then_some(digest);
        let receipt = (!records_are_supports).then_some(digest);
        if digest == Digest32::zero() {
            return Err(qualification_record_identity_error(
                vector, "zero", digest, index, None, support, receipt,
            ));
        }
        if let Some(first_index) = first_by_digest.insert(digest, index) {
            return Err(qualification_record_identity_error(
                vector,
                "duplicate",
                digest,
                first_index,
                Some(index),
                support,
                receipt,
            ));
        }
    }
    Ok(())
}

fn validate_qualification_surface_receipt_occurrences(
    records: &[SnowStage3V11QualificationSurfaceReceiptOccurrenceV1],
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    let mut first_by_custody = BTreeMap::new();
    let mut previous: Option<&SnowStage3V11QualificationSurfaceReceiptOccurrenceV1> = None;
    for (index, record) in records.iter().enumerate() {
        if let Err(error) = record.validate() {
            return match error {
                DirectSnowStage3V11AttachmentError::QualificationOrderedRecordIdentity {
                    failure,
                    digest,
                    source_support_receipt_sha256,
                    source_receipt_sha256,
                    ..
                } => Err(qualification_record_identity_error(
                    "surface_receipt_occurrences",
                    failure,
                    digest,
                    index,
                    None,
                    source_support_receipt_sha256,
                    source_receipt_sha256,
                )),
                other => Err(other),
            };
        }
        let custody = record.custody_identity_sha256()?;
        if let Some(first_index) = first_by_custody.insert(custody, index) {
            return Err(qualification_record_identity_error(
                "surface_receipt_occurrences",
                "duplicate custody",
                custody,
                first_index,
                Some(index),
                Some(record.accepted_support_receipt_sha256),
                Some(record.source_receipt_sha256),
            ));
        }
        if let Some(prior) = previous {
            let same_support = record.accepted_support_receipt_sha256
                == prior.accepted_support_receipt_sha256
                && record.accepted_support_start_ns == prior.accepted_support_start_ns
                && record.accepted_support_end_ns == prior.accepted_support_end_ns
                && record.accepted_support_interval_index == prior.accepted_support_interval_index;
            let ordered = if same_support {
                prior.source_receipt_ordinal.checked_add(1) == Some(record.source_receipt_ordinal)
            } else {
                record.accepted_support_start_ns >= prior.accepted_support_end_ns
                    && record.source_receipt_ordinal == 0
            };
            if !ordered {
                return Err(qualification_record_identity_error(
                    "surface_receipt_occurrences",
                    "noncanonical order",
                    record.receipt_sha256,
                    index - 1,
                    Some(index),
                    Some(record.accepted_support_receipt_sha256),
                    Some(record.source_receipt_sha256),
                ));
            }
        } else if record.source_receipt_ordinal != 0 {
            return Err(qualification_record_identity_error(
                "surface_receipt_occurrences",
                "nonzero first ordinal",
                record.receipt_sha256,
                index,
                None,
                Some(record.accepted_support_receipt_sha256),
                Some(record.source_receipt_sha256),
            ));
        }
        previous = Some(record);
    }
    Ok(())
}
