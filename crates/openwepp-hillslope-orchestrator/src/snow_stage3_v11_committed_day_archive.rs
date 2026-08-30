const STAGE3_COMMITTED_DAY_ARCHIVE_SCHEMA_V1: u16 = 1;

fn archive_error(reason: &'static str) -> DirectSnowStage3V11AttachmentError {
    DirectSnowStage3V11AttachmentError::Identity(reason)
}

fn archive_genesis_root(domain: &'static str, run: Digest32, topology: Digest32) -> Digest32 {
    digest_bytes(
        &[
            domain.as_bytes(),
            run.as_bytes().as_slice(),
            topology.as_bytes().as_slice(),
        ]
        .concat(),
    )
}

#[derive(Default)]
pub(crate) struct ArchiveDigestCountWriter {
    digest: sha2::Sha256,
    byte_count: u64,
}

impl std::io::Write for ArchiveDigestCountWriter {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        use sha2::Digest as _;

        let length = u64::try_from(bytes.len())
            .map_err(|_| std::io::Error::other("archive byte length width"))?;
        self.byte_count = self
            .byte_count
            .checked_add(length)
            .ok_or_else(|| std::io::Error::other("archive byte count overflow"))?;
        self.digest.update(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl ArchiveDigestCountWriter {
    pub(crate) fn finish(self) -> (Digest32, u64) {
        use sha2::Digest as _;

        (Digest32::from_bytes(self.digest.finalize().into()), self.byte_count)
    }
}

fn archive_write_bytes<W, F>(
    output: &mut W,
    length: u64,
    write_bytes: F,
) -> Result<(), DirectSnowStage3V11AttachmentError>
where
    W: std::io::Write + ?Sized,
    F: FnOnce(&mut W) -> Result<(), DirectSnowStage3V11AttachmentError>,
{
    output
        .write_all(&length.to_be_bytes())
        .map_err(|_| archive_error("archive canonical write"))?;
    write_bytes(output).map_err(|_| archive_error("archive canonical field serialization"))
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage3CommittedDayArchiveEntryV1 {
    pub schema_version: u16,
    pub day_index: usize,
    pub previous_ordered_day_chain_sha256: Digest32,
    pub resulting_ordered_day_chain_sha256: Digest32,
    pub previous_archive_content_root_sha256: Digest32,
    pub resulting_archive_content_root_sha256: Digest32,
    pub content_sha256: Digest32,
    pub canonical_uncompressed_len: u64,
    pub parent_receipt_sha256: Digest32,
    pub publication_evidence_sha256: Digest32,
    pub committed_publication_receipt_sha256: Digest32,
    pub beginning_owner_set_sha256: Digest32,
    pub ending_owner_set_sha256: Digest32,
    pub ending_accepted_until_ns: u128,
    pub ending_next_parent_sequence: u128,
    pub qualification_day_delta_sha256: Digest32,
    pub record_sha256: Digest32,
}

impl Stage3CommittedDayArchiveEntryV1 {
    fn reconstructed_record_sha256(
        &self,
    ) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
        let day_index = u64::try_from(self.day_index)
            .map_err(|_| archive_error("archive day index width"))?
            .to_be_bytes();
        framed_sha256(
            "stage3-committed-day-archive-record-v1",
            &[
                FramedField {
                    tag: "schema_version",
                    value: &self.schema_version.to_be_bytes(),
                },
                FramedField {
                    tag: "day_index",
                    value: &day_index,
                },
                FramedField {
                    tag: "previous_ordered_day_chain_sha256",
                    value: self.previous_ordered_day_chain_sha256.as_bytes(),
                },
                FramedField {
                    tag: "previous_archive_content_root_sha256",
                    value: self.previous_archive_content_root_sha256.as_bytes(),
                },
                FramedField {
                    tag: "content_sha256",
                    value: self.content_sha256.as_bytes(),
                },
                FramedField {
                    tag: "canonical_uncompressed_len",
                    value: &self.canonical_uncompressed_len.to_be_bytes(),
                },
                FramedField {
                    tag: "parent_receipt_sha256",
                    value: self.parent_receipt_sha256.as_bytes(),
                },
                FramedField {
                    tag: "publication_evidence_sha256",
                    value: self.publication_evidence_sha256.as_bytes(),
                },
                FramedField {
                    tag: "committed_publication_receipt_sha256",
                    value: self.committed_publication_receipt_sha256.as_bytes(),
                },
                FramedField {
                    tag: "beginning_owner_set_sha256",
                    value: self.beginning_owner_set_sha256.as_bytes(),
                },
                FramedField {
                    tag: "ending_owner_set_sha256",
                    value: self.ending_owner_set_sha256.as_bytes(),
                },
                FramedField {
                    tag: "ending_accepted_until_ns",
                    value: &self.ending_accepted_until_ns.to_be_bytes(),
                },
                FramedField {
                    tag: "ending_next_parent_sequence",
                    value: &self.ending_next_parent_sequence.to_be_bytes(),
                },
                FramedField {
                    tag: "qualification_day_delta_sha256",
                    value: self.qualification_day_delta_sha256.as_bytes(),
                },
            ],
        )
        .map_err(|_| archive_error("archive record framing"))
    }

    fn reconstructed_ordered_day_chain_sha256(
        &self,
    ) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
        let day_index = u64::try_from(self.day_index)
            .map_err(|_| archive_error("archive day index width"))?
            .to_be_bytes();
        framed_sha256(
            "stage3-committed-day-ordered-chain-v1",
            &[
                FramedField {
                    tag: "previous_ordered_day_chain_sha256",
                    value: self.previous_ordered_day_chain_sha256.as_bytes(),
                },
                FramedField {
                    tag: "day_index",
                    value: &day_index,
                },
                FramedField {
                    tag: "record_sha256",
                    value: self.record_sha256.as_bytes(),
                },
            ],
        )
        .map_err(|_| archive_error("archive ordered-chain framing"))
    }

    fn reconstructed_archive_content_root_sha256(
        &self,
    ) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
        let day_index = u64::try_from(self.day_index)
            .map_err(|_| archive_error("archive day index width"))?
            .to_be_bytes();
        framed_sha256(
            "stage3-committed-day-content-root-v1",
            &[
                FramedField {
                    tag: "previous_archive_content_root_sha256",
                    value: self.previous_archive_content_root_sha256.as_bytes(),
                },
                FramedField {
                    tag: "day_index",
                    value: &day_index,
                },
                FramedField {
                    tag: "content_sha256",
                    value: self.content_sha256.as_bytes(),
                },
            ],
        )
        .map_err(|_| archive_error("archive content-root framing"))
    }

    pub fn validate(&self) -> Result<(), DirectSnowStage3V11AttachmentError> {
        let expected_end = u128::try_from(self.day_index)
            .ok()
            .and_then(|day| day.checked_add(1))
            .and_then(|day| day.checked_mul(STAGE3_V11_DAY_NS));
        if self.schema_version != STAGE3_COMMITTED_DAY_ARCHIVE_SCHEMA_V1
            || self.content_sha256 == Digest32::zero()
            || self.canonical_uncompressed_len == 0
            || self.parent_receipt_sha256 == Digest32::zero()
            || self.publication_evidence_sha256 == Digest32::zero()
            || self.committed_publication_receipt_sha256 == Digest32::zero()
            || self.beginning_owner_set_sha256 == Digest32::zero()
            || self.ending_owner_set_sha256 == Digest32::zero()
            || self.qualification_day_delta_sha256 == Digest32::zero()
            || expected_end != Some(self.ending_accepted_until_ns)
            || self.record_sha256 != self.reconstructed_record_sha256()?
            || self.resulting_ordered_day_chain_sha256
                != self.reconstructed_ordered_day_chain_sha256()?
            || self.resulting_archive_content_root_sha256
                != self.reconstructed_archive_content_root_sha256()?
        {
            return Err(archive_error("committed-day archive entry seal"));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage3CommittedDayArchiveManifestV1 {
    pub schema_version: u16,
    pub run_identity: Digest32,
    pub topology_identity: Digest32,
    pub committed_day_count: usize,
    pub entries: Vec<Stage3CommittedDayArchiveEntryV1>,
    pub ordered_day_chain_sha256: Digest32,
    pub archive_content_root_sha256: Digest32,
    pub receipt_sha256: Digest32,
}

impl Stage3CommittedDayArchiveManifestV1 {
    pub fn empty(
        run_identity: Digest32,
        topology_identity: Digest32,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        if run_identity == Digest32::zero() || topology_identity == Digest32::zero() {
            return Err(archive_error("archive manifest static identity"));
        }
        let mut value = Self {
            schema_version: STAGE3_COMMITTED_DAY_ARCHIVE_SCHEMA_V1,
            run_identity,
            topology_identity,
            committed_day_count: 0,
            entries: Vec::new(),
            ordered_day_chain_sha256: archive_genesis_root(
                "OPENWEPP_STAGE3_COMMITTED_DAY_ORDERED_CHAIN_GENESIS_V1\0",
                run_identity,
                topology_identity,
            ),
            archive_content_root_sha256: archive_genesis_root(
                "OPENWEPP_STAGE3_COMMITTED_DAY_CONTENT_ROOT_GENESIS_V1\0",
                run_identity,
                topology_identity,
            ),
            receipt_sha256: Digest32::zero(),
        };
        value.receipt_sha256 = value.reconstructed_digest()?;
        value.validate()?;
        Ok(value)
    }

    fn reconstructed_digest(&self) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
        let mut projection = self.clone();
        projection.receipt_sha256 = Digest32::zero();
        let bytes = serde_json::to_vec(&projection)
            .map_err(|_| archive_error("archive manifest serialization"))?;
        Ok(digest_bytes(
            &[
                b"OPENWEPP_STAGE3_COMMITTED_DAY_ARCHIVE_MANIFEST_V1\0".as_slice(),
                bytes.as_slice(),
            ]
            .concat(),
        ))
    }

    pub fn validate(&self) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if self.schema_version != STAGE3_COMMITTED_DAY_ARCHIVE_SCHEMA_V1
            || self.run_identity == Digest32::zero()
            || self.topology_identity == Digest32::zero()
            || self.committed_day_count != self.entries.len()
            || self.receipt_sha256 != self.reconstructed_digest()?
        {
            return Err(archive_error("archive manifest seal or static identity"));
        }
        let mut ordered = archive_genesis_root(
            "OPENWEPP_STAGE3_COMMITTED_DAY_ORDERED_CHAIN_GENESIS_V1\0",
            self.run_identity,
            self.topology_identity,
        );
        let mut content = archive_genesis_root(
            "OPENWEPP_STAGE3_COMMITTED_DAY_CONTENT_ROOT_GENESIS_V1\0",
            self.run_identity,
            self.topology_identity,
        );
        let mut previous_ending_owner = None;
        for (day_index, entry) in self.entries.iter().enumerate() {
            entry.validate()?;
            if entry.day_index != day_index
                || entry.previous_ordered_day_chain_sha256 != ordered
                || entry.previous_archive_content_root_sha256 != content
                || previous_ending_owner
                    .is_some_and(|owner| owner != entry.beginning_owner_set_sha256)
            {
                return Err(archive_error("archive manifest day order or prior root"));
            }
            ordered = entry.resulting_ordered_day_chain_sha256;
            content = entry.resulting_archive_content_root_sha256;
            previous_ending_owner = Some(entry.ending_owner_set_sha256);
        }
        if ordered != self.ordered_day_chain_sha256
            || content != self.archive_content_root_sha256
        {
            return Err(archive_error("archive manifest terminal roots"));
        }
        Ok(())
    }

    pub fn append(
        &mut self,
        entry: Stage3CommittedDayArchiveEntryV1,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        self.validate()?;
        entry.validate()?;
        if entry.day_index != self.entries.len()
            || entry.previous_ordered_day_chain_sha256 != self.ordered_day_chain_sha256
            || entry.previous_archive_content_root_sha256 != self.archive_content_root_sha256
        {
            return Err(archive_error("archive manifest append order or prior root"));
        }
        let mut candidate = self.clone();
        candidate.ordered_day_chain_sha256 = entry.resulting_ordered_day_chain_sha256;
        candidate.archive_content_root_sha256 = entry.resulting_archive_content_root_sha256;
        candidate.entries.push(entry);
        candidate.committed_day_count = candidate.entries.len();
        candidate.receipt_sha256 = candidate.reconstructed_digest()?;
        candidate.validate()?;
        *self = candidate;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage3ArchivedReceiptPrefixV1 {
    pub schema_version: u16,
    pub run_identity: Digest32,
    pub topology_identity: Digest32,
    pub archived_day_count: usize,
    pub total_parent_support_count: u64,
    pub first_day_index: Option<usize>,
    pub last_day_index: Option<usize>,
    pub ordered_day_chain_sha256: Digest32,
    pub last_day_record_sha256: Digest32,
    pub last_parent_receipt_sha256: Digest32,
    pub ending_owner_set_sha256: Option<Digest32>,
    pub accepted_until_ns: u128,
    pub next_parent_sequence: u128,
    pub archive_content_root_sha256: Digest32,
    pub qualification_accumulator: SnowStage3V11QualificationAccumulatorV1,
    pub receipt_sha256: Digest32,
}

impl Stage3ArchivedReceiptPrefixV1 {
    fn empty(
        run_identity: Digest32,
        topology_identity: Digest32,
        next_parent_sequence: u128,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        if run_identity == Digest32::zero() || topology_identity == Digest32::zero() {
            return Err(archive_error("archive prefix static identity"));
        }
        let mut value = Self {
            schema_version: STAGE3_COMMITTED_DAY_ARCHIVE_SCHEMA_V1,
            run_identity,
            topology_identity,
            archived_day_count: 0,
            total_parent_support_count: 0,
            first_day_index: None,
            last_day_index: None,
            ordered_day_chain_sha256: archive_genesis_root(
                "OPENWEPP_STAGE3_COMMITTED_DAY_ORDERED_CHAIN_GENESIS_V1\0",
                run_identity,
                topology_identity,
            ),
            last_day_record_sha256: Digest32::zero(),
            last_parent_receipt_sha256: Digest32::zero(),
            ending_owner_set_sha256: None,
            accepted_until_ns: 0,
            next_parent_sequence,
            archive_content_root_sha256: archive_genesis_root(
                "OPENWEPP_STAGE3_COMMITTED_DAY_CONTENT_ROOT_GENESIS_V1\0",
                run_identity,
                topology_identity,
            ),
            qualification_accumulator: SnowStage3V11QualificationAccumulatorV1::default(),
            receipt_sha256: Digest32::zero(),
        };
        value.receipt_sha256 = value.reconstructed_digest()?;
        value.validate()?;
        Ok(value)
    }

    fn reconstructed_digest(&self) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
        let mut projection = self.clone();
        projection.receipt_sha256 = Digest32::zero();
        let bytes = serde_json::to_vec(&projection)
            .map_err(|_| archive_error("archive prefix serialization"))?;
        Ok(digest_bytes(
            &[
                b"OPENWEPP_STAGE3_ARCHIVED_RECEIPT_PREFIX_V1\0".as_slice(),
                bytes.as_slice(),
            ]
            .concat(),
        ))
    }

    pub fn validate(&self) -> Result<(), DirectSnowStage3V11AttachmentError> {
        self.qualification_accumulator.validate()?;
        let empty = self.archived_day_count == 0;
        if self.schema_version != STAGE3_COMMITTED_DAY_ARCHIVE_SCHEMA_V1
            || self.run_identity == Digest32::zero()
            || self.topology_identity == Digest32::zero()
            || self.total_parent_support_count
                != self.qualification_accumulator.total_parent_support_count
            || self.archived_day_count != self.qualification_accumulator.committed_day_count
            || empty != self.first_day_index.is_none()
            || empty != self.last_day_index.is_none()
            || empty != self.ending_owner_set_sha256.is_none()
            || empty != (self.last_day_record_sha256 == Digest32::zero())
            || empty != (self.last_parent_receipt_sha256 == Digest32::zero())
            || self.receipt_sha256 != self.reconstructed_digest()?
        {
            return Err(archive_error("archive prefix seal or cardinality"));
        }
        if empty {
            if self.total_parent_support_count != 0 || self.accepted_until_ns != 0 {
                return Err(archive_error("archive empty prefix state"));
            }
        } else if self.first_day_index != Some(0)
            || self.last_day_index != self.archived_day_count.checked_sub(1)
            || self.accepted_until_ns
                != (self.archived_day_count as u128) * STAGE3_V11_DAY_NS
            || self.ending_owner_set_sha256
                != self
                    .qualification_accumulator
                    .ending_owner
                    .as_ref()
                    .map(|ending| ending.coupled_owner_set_sha256)
        {
            return Err(archive_error("archive prefix chronology or endpoint"));
        }
        Ok(())
    }

    fn append_day(
        &mut self,
        entry: &Stage3CommittedDayArchiveEntryV1,
        day_delta: &SnowStage3V11QualificationDayDeltaV1,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        self.validate()?;
        entry.validate()?;
        day_delta.validate()?;
        if entry.day_index != self.archived_day_count
            || day_delta.day_index != self.archived_day_count
            || entry.previous_ordered_day_chain_sha256 != self.ordered_day_chain_sha256
            || entry.previous_archive_content_root_sha256 != self.archive_content_root_sha256
            || entry.qualification_day_delta_sha256 != day_delta.receipt_sha256()
            || self
                .ending_owner_set_sha256
                .is_some_and(|ending| ending != entry.beginning_owner_set_sha256)
            || entry.ending_next_parent_sequence
                != self
                    .next_parent_sequence
                    .checked_add(STAGE3_V11_PARENT_SUPPORT_COUNT as u128)
                    .ok_or_else(|| archive_error("archive parent sequence overflow"))?
        {
            return Err(archive_error("archive prefix day join"));
        }
        let mut candidate = self.clone();
        candidate.qualification_accumulator.fold_day(day_delta)?;
        candidate.archived_day_count += 1;
        candidate.total_parent_support_count = candidate
            .total_parent_support_count
            .checked_add(day_delta.total_parent_support_count)
            .ok_or_else(|| archive_error("archive support count overflow"))?;
        candidate.first_day_index.get_or_insert(entry.day_index);
        candidate.last_day_index = Some(entry.day_index);
        candidate.ordered_day_chain_sha256 = entry.resulting_ordered_day_chain_sha256;
        candidate.last_day_record_sha256 = entry.record_sha256;
        candidate.last_parent_receipt_sha256 = entry.parent_receipt_sha256;
        candidate.ending_owner_set_sha256 = Some(entry.ending_owner_set_sha256);
        candidate.accepted_until_ns = entry.ending_accepted_until_ns;
        candidate.next_parent_sequence = entry.ending_next_parent_sequence;
        candidate.archive_content_root_sha256 = entry.resulting_archive_content_root_sha256;
        candidate.receipt_sha256 = candidate.reconstructed_digest()?;
        candidate.validate()?;
        *self = candidate;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Stage3PendingCommittedDayEvidenceV1 {
    entry: Stage3CommittedDayArchiveEntryV1,
    parent_receipt_canonical_len: u64,
    day_delta_canonical_len: u64,
    day_delta: SnowStage3V11QualificationDayDeltaV1,
    publication_evidence:
        crate::v9_real_consumer_shadow::Stage3RotatedPublicationDayEvidenceV1,
}

impl Stage3PendingCommittedDayEvidenceV1 {
    #[must_use]
    pub const fn entry(&self) -> &Stage3CommittedDayArchiveEntryV1 {
        &self.entry
    }

    fn write_canonical_uncompressed(
        &self,
        static_context: &DirectSnowStage3V11StaticContext,
        prefix: &Stage3ArchivedReceiptPrefixV1,
        parent_receipt: &DirectSnowStage3V11ParentReceipt,
        publication_day: &crate::direct_runtime::Stage3AcceptedPublicationDayV1,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        write_committed_day_canonical_v1(
            static_context,
            prefix,
            parent_receipt,
            publication_day,
            &self.publication_evidence,
            &self.day_delta,
            self.parent_receipt_canonical_len,
            self.day_delta_canonical_len,
            writer,
        )
    }

    fn try_new(
        static_context: &DirectSnowStage3V11StaticContext,
        prefix: &Stage3ArchivedReceiptPrefixV1,
        parent_receipt: &DirectSnowStage3V11ParentReceipt,
        publication_day: &crate::direct_runtime::Stage3AcceptedPublicationDayV1,
        publication_evidence: crate::v9_real_consumer_shadow::Stage3RotatedPublicationDayEvidenceV1,
        day_delta: SnowStage3V11QualificationDayDeltaV1,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        prefix.validate()?;
        day_delta.validate()?;
        let day_index = parent_receipt.day_index;
        if day_index != prefix.archived_day_count
            || publication_day.day_index() != day_index
            || publication_evidence.day_index != day_index
            || day_delta.day_index != day_index
            || publication_evidence.beginning_owner_set_sha256
                != publication_day.beginning_complete_owner_set_sha256()
            || publication_evidence.ending_owner_set_sha256
                != publication_day.ending_complete_owner_set_sha256()
            || publication_evidence.ending_owner_set_sha256
                != parent_receipt.ending_coupled_owner_set_sha256
            || day_delta.beginning_owner.coupled_owner_set_sha256
                != publication_evidence.beginning_owner_set_sha256
            || day_delta.ending_owner.coupled_owner_set_sha256
                != publication_evidence.ending_owner_set_sha256
        {
            return Err(archive_error("archive day owner/publication join"));
        }
        let mut parent_receipt_measure = ArchiveDigestCountWriter::default();
        write_stage3_v11_parent_receipt_canonical_v3(
            parent_receipt,
            &mut parent_receipt_measure,
        )?;
        let (parent_receipt_sha256, parent_receipt_canonical_len) =
            parent_receipt_measure.finish();
        let mut day_delta_measure = ArchiveDigestCountWriter::default();
        serde_json::to_writer(&mut day_delta_measure, &day_delta)
            .map_err(|_| archive_error("archive qualification delta serialization"))?;
        let (_, day_delta_canonical_len) = day_delta_measure.finish();
        let mut canonical_measure = ArchiveDigestCountWriter::default();
        write_committed_day_canonical_v1(
            static_context,
            prefix,
            parent_receipt,
            publication_day,
            &publication_evidence,
            &day_delta,
            parent_receipt_canonical_len,
            day_delta_canonical_len,
            &mut canonical_measure,
        )?;
        let (content_sha256, canonical_uncompressed_len) = canonical_measure.finish();
        let mut entry = Stage3CommittedDayArchiveEntryV1 {
            schema_version: STAGE3_COMMITTED_DAY_ARCHIVE_SCHEMA_V1,
            day_index,
            previous_ordered_day_chain_sha256: prefix.ordered_day_chain_sha256,
            resulting_ordered_day_chain_sha256: Digest32::zero(),
            previous_archive_content_root_sha256: prefix.archive_content_root_sha256,
            resulting_archive_content_root_sha256: Digest32::zero(),
            content_sha256,
            canonical_uncompressed_len,
            parent_receipt_sha256,
            publication_evidence_sha256: publication_evidence.canonical_uncompressed_sha256,
            committed_publication_receipt_sha256: publication_day.receipt_sha256(),
            beginning_owner_set_sha256: publication_evidence.beginning_owner_set_sha256,
            ending_owner_set_sha256: publication_evidence.ending_owner_set_sha256,
            ending_accepted_until_ns: parent_receipt.ending_coupled_accepted_until_ns.get(),
            ending_next_parent_sequence: parent_receipt.ending_next_parent_sequence,
            qualification_day_delta_sha256: day_delta.receipt_sha256(),
            record_sha256: Digest32::zero(),
        };
        entry.record_sha256 = entry.reconstructed_record_sha256()?;
        entry.resulting_ordered_day_chain_sha256 =
            entry.reconstructed_ordered_day_chain_sha256()?;
        entry.resulting_archive_content_root_sha256 =
            entry.reconstructed_archive_content_root_sha256()?;
        entry.validate()?;
        Ok(Self {
            entry,
            parent_receipt_canonical_len,
            day_delta_canonical_len,
            day_delta,
            publication_evidence,
        })
    }
}

#[allow(clippy::too_many_arguments)]
fn write_committed_day_canonical_v1(
    static_context: &DirectSnowStage3V11StaticContext,
    prefix: &Stage3ArchivedReceiptPrefixV1,
    parent_receipt: &DirectSnowStage3V11ParentReceipt,
    publication_day: &crate::direct_runtime::Stage3AcceptedPublicationDayV1,
    publication_evidence: &crate::v9_real_consumer_shadow::Stage3RotatedPublicationDayEvidenceV1,
    day_delta: &SnowStage3V11QualificationDayDeltaV1,
    parent_receipt_canonical_len: u64,
    day_delta_canonical_len: u64,
    writer: &mut (impl std::io::Write + ?Sized),
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    let day_index = u64::try_from(parent_receipt.day_index)
        .map_err(|_| archive_error("archive day index width"))?;
    writer
        .write_all(b"OPENWEPP_STAGE3_COMMITTED_DAY_EVIDENCE_V1\0")
        .and_then(|()| writer.write_all(&STAGE3_COMMITTED_DAY_ARCHIVE_SCHEMA_V1.to_be_bytes()))
        .and_then(|()| writer.write_all(static_context.run_identity.as_bytes()))
        .and_then(|()| writer.write_all(static_context.topology_identity.as_bytes()))
        .and_then(|()| writer.write_all(&day_index.to_be_bytes()))
        .and_then(|()| writer.write_all(prefix.archive_content_root_sha256.as_bytes()))
        .map_err(|_| archive_error("archive canonical header write"))?;
    archive_write_bytes(writer, parent_receipt_canonical_len, |output| {
        write_stage3_v11_parent_receipt_canonical_v3(parent_receipt, output)
    })?;
    let publication_length = u64::try_from(publication_evidence.canonical_support_event_bytes.len())
        .map_err(|_| archive_error("archive publication evidence length width"))?;
    archive_write_bytes(writer, publication_length, |output| {
        output
            .write_all(&publication_evidence.canonical_support_event_bytes)
            .map_err(|_| archive_error("archive publication evidence write"))
    })?;
    writer
        .write_all(publication_day.receipt_sha256().as_bytes())
        .and_then(|()| {
            writer.write_all(
                publication_day
                    .ordered_support_receipt_set_sha256()
                    .as_bytes(),
            )
        })
        .and_then(|()| writer.write_all(publication_evidence.beginning_owner_set_sha256.as_bytes()))
        .and_then(|()| writer.write_all(publication_evidence.ending_owner_set_sha256.as_bytes()))
        .and_then(|()| {
            writer.write_all(
                &parent_receipt
                    .ending_coupled_accepted_until_ns
                    .get()
                    .to_be_bytes(),
            )
        })
        .and_then(|()| writer.write_all(&parent_receipt.ending_next_parent_sequence.to_be_bytes()))
        .map_err(|_| archive_error("archive canonical endpoint write"))?;
    archive_write_bytes(writer, day_delta_canonical_len, |output| {
        serde_json::to_writer(output, day_delta)
            .map_err(|_| archive_error("archive qualification delta serialization"))
    })?;
    Ok(())
}

#[cfg(test)]
mod committed_day_archive_tests {
    use super::*;

    fn digest(byte: u8) -> Digest32 {
        Digest32::from_bytes([byte; 32])
    }

    fn entry(
        day_index: usize,
        previous_ordered: Digest32,
        previous_content: Digest32,
        beginning_owner: Digest32,
        byte: u8,
    ) -> Stage3CommittedDayArchiveEntryV1 {
        let mut value = Stage3CommittedDayArchiveEntryV1 {
            schema_version: STAGE3_COMMITTED_DAY_ARCHIVE_SCHEMA_V1,
            day_index,
            previous_ordered_day_chain_sha256: previous_ordered,
            resulting_ordered_day_chain_sha256: Digest32::zero(),
            previous_archive_content_root_sha256: previous_content,
            resulting_archive_content_root_sha256: Digest32::zero(),
            content_sha256: digest(byte),
            canonical_uncompressed_len: 4_096,
            parent_receipt_sha256: digest(byte.wrapping_add(1)),
            publication_evidence_sha256: digest(byte.wrapping_add(2)),
            committed_publication_receipt_sha256: digest(byte.wrapping_add(3)),
            beginning_owner_set_sha256: beginning_owner,
            ending_owner_set_sha256: digest(byte.wrapping_add(4)),
            ending_accepted_until_ns: (day_index as u128 + 1) * STAGE3_V11_DAY_NS,
            ending_next_parent_sequence: (day_index as u128 + 1)
                * STAGE3_V11_PARENT_SUPPORT_COUNT as u128,
            qualification_day_delta_sha256: digest(byte.wrapping_add(5)),
            record_sha256: Digest32::zero(),
        };
        value.record_sha256 = value.reconstructed_record_sha256().expect("record seal");
        value.resulting_ordered_day_chain_sha256 = value
            .reconstructed_ordered_day_chain_sha256()
            .expect("ordered chain");
        value.resulting_archive_content_root_sha256 = value
            .reconstructed_archive_content_root_sha256()
            .expect("content root");
        value.validate().expect("valid archive entry");
        value
    }

    fn endpoint(
        accepted_until_ns: u128,
        identity: u8,
        require_complete: bool,
    ) -> SnowStage3V11QualificationOwnerEndpointV1 {
        SnowStage3V11QualificationOwnerEndpointV1 {
            complete_owner_sha256: require_complete.then(|| digest(identity)),
            coupled_owner_set_sha256: digest(identity.wrapping_add(1)),
            accepted_until_ns,
            soil_thermal_owner_sha256: Some(digest(identity.wrapping_add(2))),
            biogeochemistry_owner_sha256: Some(digest(identity.wrapping_add(3))),
        }
    }

    fn day_delta(
        day_index: usize,
        beginning: SnowStage3V11QualificationOwnerEndpointV1,
        ending_identity: u8,
    ) -> SnowStage3V11QualificationDayDeltaV1 {
        SnowStage3V11QualificationDayDeltaV1 {
            schema_version: STAGE3_V11_QUALIFICATION_ACCUMULATOR_SCHEMA_V1,
            day_index,
            total_parent_support_count: STAGE3_V11_PARENT_SUPPORT_COUNT as u64,
            adaptive_support_receipt_count: 0,
            snow_free_successor_receipt_count: STAGE3_V11_PARENT_SUPPORT_COUNT as u64,
            snow_free_parent_support_count: STAGE3_V11_PARENT_SUPPORT_COUNT as u64,
            terminal_event_count: 0,
            publication_event_count: 0,
            accepted_publication_support_count: STAGE3_V11_PARENT_SUPPORT_COUNT as u64,
            ammonium_resource_use_n: 0.0,
            nitrate_resource_use_n: 0.0,
            material_transfers: SnowStage3V11QualifiedBgcInventoryV1::default(),
            accepted_support_receipt_sha256s: (0..STAGE3_V11_PARENT_SUPPORT_COUNT)
                .map(|offset| digest(50_u8.wrapping_add(offset as u8)))
                .collect(),
            surface_receipt_occurrences: Vec::new(),
            event_receipt_sha256s: Vec::new(),
            surface_flow_by_route: BTreeMap::new(),
            routed_runoff: SnowStage3V11QualifiedMassEnthalpyTotalV1::default(),
            upstream_runon: SnowStage3V11QualifiedMassEnthalpyTotalV1::default(),
            outlet_runoff: SnowStage3V11QualifiedMassEnthalpyTotalV1::default(),
            beginning_owner: beginning,
            ending_owner: endpoint(
                (day_index as u128 + 1) * STAGE3_V11_DAY_NS,
                ending_identity,
                true,
            ),
            receipt_sha256: Digest32::zero(),
        }
        .seal()
        .expect("sealed day delta")
    }

    #[test]
    fn committed_day_manifest_is_append_only_and_rejects_archive_poisons() {
        let mut manifest = Stage3CommittedDayArchiveManifestV1::empty(digest(1), digest(2))
            .expect("empty manifest");
        let day0 = entry(
            0,
            manifest.ordered_day_chain_sha256,
            manifest.archive_content_root_sha256,
            digest(10),
            20,
        );
        manifest.append(day0.clone()).expect("append day zero");
        let day1 = entry(
            1,
            manifest.ordered_day_chain_sha256,
            manifest.archive_content_root_sha256,
            day0.ending_owner_set_sha256,
            30,
        );
        manifest.append(day1.clone()).expect("append day one");
        manifest.validate().expect("two-day manifest");

        let mut omission = manifest.clone();
        omission.entries.remove(0);
        assert!(omission.validate().is_err());

        let mut duplicate = manifest.clone();
        duplicate.entries[1] = duplicate.entries[0].clone();
        assert!(duplicate.validate().is_err());

        let mut order = manifest.clone();
        order.entries.swap(0, 1);
        assert!(order.validate().is_err());

        let mut truncation = manifest.clone();
        truncation.entries.pop();
        assert!(truncation.validate().is_err());

        let mut wrong_day_count = manifest.clone();
        wrong_day_count.committed_day_count += 1;
        wrong_day_count.receipt_sha256 = wrong_day_count
            .reconstructed_digest()
            .expect("day-count poison reseal");
        assert!(wrong_day_count.validate().is_err());

        let mut content = manifest.clone();
        content.entries[0].content_sha256 = digest(99);
        assert!(content.validate().is_err());

        let mut prior_root = manifest.clone();
        prior_root.entries[1].previous_archive_content_root_sha256 = digest(98);
        assert!(prior_root.validate().is_err());

        let mut final_owner = manifest.clone();
        final_owner.entries[0].ending_owner_set_sha256 = digest(96);
        final_owner.entries[0].record_sha256 = final_owner.entries[0]
            .reconstructed_record_sha256()
            .expect("final-owner poison record");
        final_owner.entries[0].resulting_ordered_day_chain_sha256 = final_owner.entries[0]
            .reconstructed_ordered_day_chain_sha256()
            .expect("final-owner poison chain");
        final_owner.entries[1].previous_ordered_day_chain_sha256 =
            final_owner.entries[0].resulting_ordered_day_chain_sha256;
        final_owner.entries[1].record_sha256 = final_owner.entries[1]
            .reconstructed_record_sha256()
            .expect("final-owner successor record");
        final_owner.entries[1].resulting_ordered_day_chain_sha256 = final_owner.entries[1]
            .reconstructed_ordered_day_chain_sha256()
            .expect("final-owner successor chain");
        final_owner.ordered_day_chain_sha256 =
            final_owner.entries[1].resulting_ordered_day_chain_sha256;
        final_owner.receipt_sha256 = final_owner
            .reconstructed_digest()
            .expect("final-owner manifest reseal");
        assert!(final_owner.validate().is_err());

        let mut terminal_root = manifest;
        terminal_root.archive_content_root_sha256 = digest(97);
        assert!(terminal_root.validate().is_err());
    }

    #[test]
    fn archived_prefix_folds_exact_days_and_rejects_owner_delta_and_sequence_poisons() {
        let run = digest(1);
        let topology = digest(2);
        let mut prefix = Stage3ArchivedReceiptPrefixV1::empty(run, topology, 0)
            .expect("empty archive prefix");
        let delta = day_delta(0, endpoint(0, 10, false), 20);
        let mut day = entry(
            0,
            prefix.ordered_day_chain_sha256,
            prefix.archive_content_root_sha256,
            delta.beginning_owner.coupled_owner_set_sha256,
            30,
        );
        day.ending_owner_set_sha256 = delta.ending_owner.coupled_owner_set_sha256;
        day.ending_next_parent_sequence = STAGE3_V11_PARENT_SUPPORT_COUNT as u128;
        day.qualification_day_delta_sha256 = delta.receipt_sha256();
        day.record_sha256 = day.reconstructed_record_sha256().expect("record reseal");
        day.resulting_ordered_day_chain_sha256 = day
            .reconstructed_ordered_day_chain_sha256()
            .expect("ordered reseal");
        day.resulting_archive_content_root_sha256 = day
            .reconstructed_archive_content_root_sha256()
            .expect("content reseal");
        prefix.append_day(&day, &delta).expect("fold day zero");
        prefix.validate().expect("sealed one-day prefix");
        assert_eq!(prefix.archived_day_count, 1);
        assert_eq!(prefix.total_parent_support_count, 48);

        let mut wrong_prefix_count = prefix.clone();
        wrong_prefix_count.archived_day_count += 1;
        wrong_prefix_count.receipt_sha256 = wrong_prefix_count
            .reconstructed_digest()
            .expect("prefix count poison reseal");
        assert!(wrong_prefix_count.validate().is_err());

        let mut wrong_final_owner = prefix.clone();
        wrong_final_owner.ending_owner_set_sha256 = Some(digest(89));
        wrong_final_owner.receipt_sha256 = wrong_final_owner
            .reconstructed_digest()
            .expect("prefix owner poison reseal");
        assert!(wrong_final_owner.validate().is_err());

        let mut wrong_delta = delta.clone();
        wrong_delta.day_index = 1;
        assert!(prefix.append_day(&day, &wrong_delta).is_err());

        let mut wrong_owner = day.clone();
        wrong_owner.beginning_owner_set_sha256 = digest(90);
        wrong_owner.record_sha256 = wrong_owner
            .reconstructed_record_sha256()
            .expect("owner poison reseal");
        wrong_owner.resulting_ordered_day_chain_sha256 = wrong_owner
            .reconstructed_ordered_day_chain_sha256()
            .expect("owner poison ordered root");
        assert!(prefix.append_day(&wrong_owner, &delta).is_err());

        let mut wrong_sequence = day;
        wrong_sequence.ending_next_parent_sequence += 1;
        wrong_sequence.record_sha256 = wrong_sequence
            .reconstructed_record_sha256()
            .expect("sequence poison reseal");
        wrong_sequence.resulting_ordered_day_chain_sha256 = wrong_sequence
            .reconstructed_ordered_day_chain_sha256()
            .expect("sequence poison ordered root");
        assert!(prefix.append_day(&wrong_sequence, &delta).is_err());
    }
}
