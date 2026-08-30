use std::collections::BTreeSet;
use std::fs::{self, File, OpenOptions};
#[cfg(test)]
use std::io::Read;
use std::io::{self, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use flate2::Compression;
#[cfg(test)]
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use openwepp_coupled_time::{Digest32, digest_bytes};
use sha2::{Digest as _, Sha256};

const RECORD_STREAM_MAGIC: &[u8; 31] = b"OPENWEPP_TX_RECORD_SPOOL_V2\0\0\0\0";
#[cfg(test)]
const LEGACY_RECORD_STREAM_MAGIC_V1: &[u8; 31] = b"OPENWEPP_TX_RECORD_SPOOL_V1\0\0\0\0";
const RECORD_MARKER: u8 = 1;
const FOOTER_MARKER: u8 = 255;

#[derive(Debug)]
struct PrivateSpoolEntry {
    private_path: PathBuf,
    staged_path: PathBuf,
}

/// Unpublished execution storage one tier behind the output transaction's
/// ordinary staged names. Promotion creates no public path and succeeds only
/// when every private file is complete.
#[derive(Debug)]
pub(super) struct RunnerTransactionPrivateSpool {
    entries: Vec<PrivateSpoolEntry>,
}

impl RunnerTransactionPrivateSpool {
    pub(super) fn new(paths: impl IntoIterator<Item = (PathBuf, PathBuf)>) -> io::Result<Self> {
        let entries = paths
            .into_iter()
            .map(|(private_path, staged_path)| PrivateSpoolEntry {
                private_path,
                staged_path,
            })
            .collect::<Vec<_>>();
        let mut identities = BTreeSet::new();
        for entry in &entries {
            if entry.private_path == entry.staged_path
                || !identities.insert(entry.private_path.clone())
                || !identities.insert(entry.staged_path.clone())
                || entry.private_path.exists()
                || entry.staged_path.exists()
            {
                return Err(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    "transaction-private spool path collision",
                ));
            }
        }
        Ok(Self { entries })
    }

    pub(super) fn promote_complete(&self) -> io::Result<()> {
        if self
            .entries
            .iter()
            .any(|entry| !entry.private_path.is_file() || entry.staged_path.exists())
        {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "transaction-private spool is incomplete or staging is occupied",
            ));
        }
        let mut promoted = Vec::with_capacity(self.entries.len());
        for entry in &self.entries {
            if let Err(source) = fs::hard_link(&entry.private_path, &entry.staged_path) {
                remove_paths(&promoted);
                return Err(source);
            }
            promoted.push(entry.staged_path.clone());
        }
        for entry in &self.entries {
            if let Err(source) = fs::remove_file(&entry.private_path) {
                remove_paths(&promoted);
                return Err(source);
            }
        }
        Ok(())
    }

    pub(super) fn discard(&self) {
        for entry in &self.entries {
            remove_file_if_present(&entry.private_path);
            remove_file_if_present(&entry.staged_path);
        }
    }
}

impl Drop for RunnerTransactionPrivateSpool {
    fn drop(&mut self) {
        self.discard();
    }
}

fn remove_paths(paths: &[PathBuf]) {
    for path in paths {
        remove_file_if_present(path);
    }
}

fn remove_file_if_present(path: &Path) {
    let _ = fs::remove_file(path);
}

/// Length-framed canonical record stream used for detailed Stage-3 daily
/// evidence. The writer retains only a count and ordered root in memory.
pub(super) struct CanonicalRecordSpoolWriter {
    file: File,
    record_count: u64,
    ordered_root_sha256: Digest32,
    canonical_uncompressed_bytes: u64,
    stored_record_bytes: u64,
    finished: bool,
    #[cfg(test)]
    force_sync_failure_once: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct CanonicalRecordSpoolSeal {
    pub(super) record_count: u64,
    pub(super) ordered_root_sha256: Digest32,
    pub(super) canonical_uncompressed_bytes: u64,
    pub(super) stored_record_bytes: u64,
}

impl CanonicalRecordSpoolWriter {
    pub(super) fn create(path: &Path) -> io::Result<Self> {
        let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
        file.write_all(RECORD_STREAM_MAGIC)?;
        Ok(Self {
            file,
            record_count: 0,
            ordered_root_sha256: Digest32::zero(),
            canonical_uncompressed_bytes: 0,
            stored_record_bytes: 0,
            finished: false,
            #[cfg(test)]
            force_sync_failure_once: false,
        })
    }

    /// Append one complete frame and make it durable before returning its
    /// content digest. On any write/sync failure the file is truncated back to
    /// the exact preceding boundary and count/root remain unchanged.
    #[cfg(test)]
    pub(super) fn append_durable(&mut self, canonical_bytes: &[u8]) -> io::Result<Digest32> {
        let length = u64::try_from(canonical_bytes.len()).map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidInput, "canonical spool record width")
        })?;
        let record_sha256 = digest_bytes(canonical_bytes);
        self.append_durable_stream(length, record_sha256, |writer| {
            writer.write_all(canonical_bytes)
        })
    }

    /// Stream one canonical record directly through its digest/count verifier
    /// and deterministic compressor. The expected uncompressed seal is
    /// supplied by the producer's independent staging pass.
    pub(super) fn append_durable_stream(
        &mut self,
        length: u64,
        record_sha256: Digest32,
        write_canonical: impl FnOnce(&mut dyn Write) -> io::Result<()>,
    ) -> io::Result<Digest32> {
        if self.finished || length == 0 || record_sha256 == Digest32::zero() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "canonical spool record is empty or writer is finished",
            ));
        }
        let next_root = digest_bytes(
            &[
                b"OPENWEPP_TX_RECORD_SPOOL_ORDER_V1\0".as_slice(),
                self.ordered_root_sha256.as_bytes().as_slice(),
                self.record_count.to_be_bytes().as_slice(),
                length.to_be_bytes().as_slice(),
                record_sha256.as_bytes().as_slice(),
            ]
            .concat(),
        );
        let beginning_length = self.file.stream_position()?;
        let write_result = (|| -> io::Result<u64> {
            self.file.write_all(&[RECORD_MARKER])?;
            self.file.write_all(&length.to_be_bytes())?;
            self.file.write_all(&0_u64.to_be_bytes())?;
            self.file.write_all(record_sha256.as_bytes())?;
            let compressed_start = self.file.stream_position()?;
            let (actual_sha256, actual_length) = {
                let encoder = ZlibEncoder::new(&mut self.file, Compression::new(6));
                let mut canonical = CanonicalCompressionWriter::new(encoder);
                write_canonical(&mut canonical)?;
                canonical.finish()?
            };
            if actual_length != length || actual_sha256 != record_sha256 {
                return Err(invalid_spool("canonical spool streamed record seal"));
            }
            let ending_position = self.file.stream_position()?;
            let compressed_length = ending_position
                .checked_sub(compressed_start)
                .ok_or_else(|| invalid_spool("compressed record position"))?;
            self.file.seek(SeekFrom::Start(
                beginning_length
                    .checked_add(1 + 8)
                    .ok_or_else(|| invalid_spool("compressed length header position"))?,
            ))?;
            self.file.write_all(&compressed_length.to_be_bytes())?;
            self.file.seek(SeekFrom::Start(ending_position))?;
            self.file.flush()?;
            #[cfg(test)]
            if self.force_sync_failure_once {
                self.force_sync_failure_once = false;
                return Err(io::Error::other("forced canonical spool sync failure"));
            }
            self.file.sync_data()?;
            Ok(compressed_length)
        })();
        let compressed_length = match write_result {
            Ok(compressed_length) => compressed_length,
            Err(source) => {
                let rollback = self
                    .file
                    .set_len(beginning_length)
                    .and_then(|()| self.file.seek(SeekFrom::Start(beginning_length)).map(drop))
                    .and_then(|()| self.file.sync_data());
                return match rollback {
                    Ok(()) => Err(source),
                    Err(rollback_source) => Err(io::Error::other(format!(
                        "canonical spool append failed: {source}; truncation rollback failed: {rollback_source}"
                    ))),
                };
            }
        };
        self.record_count = self.record_count.checked_add(1).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "canonical spool count overflow")
        })?;
        self.ordered_root_sha256 = next_root;
        self.canonical_uncompressed_bytes =
            self.canonical_uncompressed_bytes
                .checked_add(length)
                .ok_or_else(|| invalid_spool("canonical spool uncompressed byte count"))?;
        self.stored_record_bytes = self
            .stored_record_bytes
            .checked_add(compressed_length)
            .ok_or_else(|| invalid_spool("canonical spool stored byte count"))?;
        Ok(record_sha256)
    }

    #[cfg(test)]
    fn force_sync_failure_once(&mut self) {
        self.force_sync_failure_once = true;
    }

    #[cfg(test)]
    const fn current_seal(&self) -> CanonicalRecordSpoolSeal {
        CanonicalRecordSpoolSeal {
            record_count: self.record_count,
            ordered_root_sha256: self.ordered_root_sha256,
            canonical_uncompressed_bytes: self.canonical_uncompressed_bytes,
            stored_record_bytes: self.stored_record_bytes,
        }
    }

    pub(super) fn finish(mut self) -> io::Result<CanonicalRecordSpoolSeal> {
        self.file.write_all(&[FOOTER_MARKER])?;
        self.file.write_all(&self.record_count.to_be_bytes())?;
        self.file.write_all(self.ordered_root_sha256.as_bytes())?;
        self.file.sync_all()?;
        self.finished = true;
        Ok(CanonicalRecordSpoolSeal {
            record_count: self.record_count,
            ordered_root_sha256: self.ordered_root_sha256,
            canonical_uncompressed_bytes: self.canonical_uncompressed_bytes,
            stored_record_bytes: self.stored_record_bytes,
        })
    }
}

struct CanonicalCompressionWriter<W: Write> {
    encoder: ZlibEncoder<W>,
    digest: Sha256,
    byte_count: u64,
}

impl<W: Write> CanonicalCompressionWriter<W> {
    fn new(encoder: ZlibEncoder<W>) -> Self {
        Self {
            encoder,
            digest: Sha256::new(),
            byte_count: 0,
        }
    }

    fn finish(self) -> io::Result<(Digest32, u64)> {
        let Self {
            encoder,
            digest,
            byte_count,
        } = self;
        encoder.finish()?;
        Ok((Digest32::from_bytes(digest.finalize().into()), byte_count))
    }
}

impl<W: Write> Write for CanonicalCompressionWriter<W> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let written = self.encoder.write(bytes)?;
        let length = u64::try_from(written)
            .map_err(|_| invalid_spool("canonical spool streamed write width"))?;
        self.byte_count = self
            .byte_count
            .checked_add(length)
            .ok_or_else(|| invalid_spool("canonical spool streamed byte count"))?;
        self.digest.update(&bytes[..written]);
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.encoder.flush()
    }
}

#[cfg(test)]
fn reconstruct_canonical_record_spool(
    path: &Path,
) -> io::Result<(Vec<Vec<u8>>, CanonicalRecordSpoolSeal)> {
    let mut records = Vec::new();
    let seal = visit_canonical_record_spool(path, |record| {
        records.push(record.to_vec());
        Ok(())
    })?;
    Ok((records, seal))
}

/// Verify and visit one canonical record at a time. The compressed archive is
/// never materialized as a whole, and each record is released after the
/// visitor returns.
#[cfg(test)]
fn visit_canonical_record_spool(
    path: &Path,
    mut visit: impl FnMut(&[u8]) -> io::Result<()>,
) -> io::Result<CanonicalRecordSpoolSeal> {
    let mut file = File::open(path)?;
    let mut magic = [0_u8; RECORD_STREAM_MAGIC.len()];
    file.read_exact(&mut magic)?;
    let compressed_records = if magic == *RECORD_STREAM_MAGIC {
        true
    } else if magic == *LEGACY_RECORD_STREAM_MAGIC_V1 {
        false
    } else {
        return Err(invalid_spool("canonical spool magic"));
    };
    let mut record_count = 0_u64;
    let mut root = Digest32::zero();
    let mut canonical_uncompressed_bytes = 0_u64;
    let mut stored_record_bytes = 0_u64;
    loop {
        let mut marker = [0_u8; 1];
        file.read_exact(&mut marker)?;
        match marker[0] {
            RECORD_MARKER => {
                let length = read_u64(&mut file)?;
                let compressed_length = compressed_records
                    .then(|| read_u64(&mut file))
                    .transpose()?;
                let mut sealed_digest = [0_u8; 32];
                file.read_exact(&mut sealed_digest)?;
                let record_length = usize::try_from(length)
                    .map_err(|_| invalid_spool("canonical spool record width"))?;
                let (record, stored_length) = if let Some(compressed_length) = compressed_length {
                    if compressed_length == 0 {
                        return Err(invalid_spool("canonical spool compressed record width"));
                    }
                    let mut record = Vec::with_capacity(record_length);
                    let compressed = (&mut file).take(compressed_length);
                    let mut decoder = ZlibDecoder::new(compressed);
                    decoder.read_to_end(&mut record)?;
                    let compressed = decoder.into_inner();
                    if compressed.limit() != 0 || record.len() != record_length {
                        return Err(invalid_spool("canonical spool compressed record closure"));
                    }
                    (record, compressed_length)
                } else {
                    let mut record = vec![0_u8; record_length];
                    file.read_exact(&mut record)?;
                    (record, length)
                };
                let record_sha256 = digest_bytes(&record);
                if record_sha256 != Digest32::from_bytes(sealed_digest) {
                    return Err(invalid_spool("canonical spool record digest"));
                }
                root = digest_bytes(
                    &[
                        b"OPENWEPP_TX_RECORD_SPOOL_ORDER_V1\0".as_slice(),
                        root.as_bytes().as_slice(),
                        record_count.to_be_bytes().as_slice(),
                        length.to_be_bytes().as_slice(),
                        record_sha256.as_bytes().as_slice(),
                    ]
                    .concat(),
                );
                canonical_uncompressed_bytes = canonical_uncompressed_bytes
                    .checked_add(length)
                    .ok_or_else(|| invalid_spool("canonical spool uncompressed byte count"))?;
                stored_record_bytes = stored_record_bytes
                    .checked_add(stored_length)
                    .ok_or_else(|| invalid_spool("canonical spool stored byte count"))?;
                visit(&record)?;
                record_count = record_count
                    .checked_add(1)
                    .ok_or_else(|| invalid_spool("canonical spool record count"))?;
            }
            FOOTER_MARKER => {
                let sealed_record_count = read_u64(&mut file)?;
                let mut footer_root = [0_u8; 32];
                file.read_exact(&mut footer_root)?;
                let mut trailing = [0_u8; 1];
                if file.read(&mut trailing)? != 0
                    || sealed_record_count != record_count
                    || Digest32::from_bytes(footer_root) != root
                {
                    return Err(invalid_spool("canonical spool footer"));
                }
                return Ok(CanonicalRecordSpoolSeal {
                    record_count,
                    ordered_root_sha256: root,
                    canonical_uncompressed_bytes,
                    stored_record_bytes,
                });
            }
            _ => return Err(invalid_spool("canonical spool marker")),
        }
    }
}

#[cfg(test)]
fn read_u64(reader: &mut impl Read) -> io::Result<u64> {
    let mut bytes = [0_u8; 8];
    reader.read_exact(&mut bytes)?;
    Ok(u64::from_be_bytes(bytes))
}

fn invalid_spool(detail: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, detail)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn directory(label: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("test clock")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "openwepp-runner-spool-{label}-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir(&path).expect("create test directory");
        path
    }

    fn write_legacy_v1_spool(path: &Path, records: &[&[u8]]) -> CanonicalRecordSpoolSeal {
        let mut file = File::create(path).expect("create legacy spool");
        file.write_all(LEGACY_RECORD_STREAM_MAGIC_V1)
            .expect("write legacy magic");
        let mut root = Digest32::zero();
        let mut canonical_uncompressed_bytes = 0_u64;
        for (index, record) in records.iter().enumerate() {
            let length = u64::try_from(record.len()).expect("legacy record length");
            let digest = digest_bytes(record);
            root = digest_bytes(
                &[
                    b"OPENWEPP_TX_RECORD_SPOOL_ORDER_V1\0".as_slice(),
                    root.as_bytes().as_slice(),
                    u64::try_from(index)
                        .expect("legacy record index")
                        .to_be_bytes()
                        .as_slice(),
                    length.to_be_bytes().as_slice(),
                    digest.as_bytes().as_slice(),
                ]
                .concat(),
            );
            file.write_all(&[RECORD_MARKER])
                .expect("write legacy marker");
            file.write_all(&length.to_be_bytes())
                .expect("write legacy length");
            file.write_all(digest.as_bytes())
                .expect("write legacy digest");
            file.write_all(record).expect("write legacy record");
            canonical_uncompressed_bytes += length;
        }
        file.write_all(&[FOOTER_MARKER])
            .expect("write legacy footer marker");
        file.write_all(
            &u64::try_from(records.len())
                .expect("legacy record count")
                .to_be_bytes(),
        )
        .expect("write legacy record count");
        file.write_all(root.as_bytes())
            .expect("write legacy footer root");
        file.sync_all().expect("sync legacy spool");
        CanonicalRecordSpoolSeal {
            record_count: u64::try_from(records.len()).expect("legacy record count"),
            ordered_root_sha256: root,
            canonical_uncompressed_bytes,
            stored_record_bytes: canonical_uncompressed_bytes,
        }
    }

    #[test]
    fn canonical_record_stream_reconstructs_exact_bytes_and_ordered_seal() {
        let root = directory("reconstruct");
        let path = root.join("evidence.spool");
        let records = [b"day-zero".as_slice(), b"day-one-different".as_slice()];
        let mut writer = CanonicalRecordSpoolWriter::create(&path).expect("create spool");
        for record in records {
            writer.append_durable(record).expect("append record");
        }
        let written = writer.finish().expect("finish spool");
        let (reconstructed, read) =
            reconstruct_canonical_record_spool(&path).expect("reconstruct spool");
        assert_eq!(written, read);
        assert_eq!(reconstructed, records.map(<[u8]>::to_vec));
        fs::remove_dir_all(root).expect("remove test directory");
    }

    #[test]
    fn legacy_v1_stream_reconstructs_exactly_and_rejects_payload_substitution() {
        let root = directory("legacy-v1");
        let path = root.join("evidence.spool");
        let records = [b"legacy-day-zero".as_slice(), b"legacy-day-one".as_slice()];
        let expected = write_legacy_v1_spool(&path, &records);
        let (reconstructed, seal) =
            reconstruct_canonical_record_spool(&path).expect("reconstruct legacy spool");
        assert_eq!(reconstructed, records.map(<[u8]>::to_vec));
        assert_eq!(seal, expected);

        let payload_offset = u64::try_from(LEGACY_RECORD_STREAM_MAGIC_V1.len())
            .expect("legacy magic length")
            + 1
            + 8
            + 32;
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path)
            .expect("open legacy spool");
        file.seek(SeekFrom::Start(payload_offset + 2))
            .expect("seek legacy payload");
        file.write_all(b"X").expect("substitute legacy payload");
        file.sync_all().expect("sync legacy substitution");
        assert!(reconstruct_canonical_record_spool(&path).is_err());
        drop(file);

        write_legacy_v1_spool(&path, &records);
        let truncated_length = fs::metadata(&path).expect("legacy metadata").len() - 5;
        OpenOptions::new()
            .write(true)
            .open(&path)
            .expect("open legacy spool for truncation")
            .set_len(truncated_length)
            .expect("truncate legacy spool");
        assert!(reconstruct_canonical_record_spool(&path).is_err());
        fs::remove_dir_all(root).expect("remove test directory");
    }

    #[test]
    fn canonical_record_stream_compresses_without_changing_canonical_identity() {
        let root = directory("compressed-identity");
        let path = root.join("evidence.spool");
        let record = vec![b'x'; 2 * 1024 * 1024];
        let expected_digest = digest_bytes(&record);
        let mut writer = CanonicalRecordSpoolWriter::create(&path).expect("create spool");
        assert_eq!(
            writer.append_durable(&record).expect("append record"),
            expected_digest
        );
        let written = writer.finish().expect("finish spool");
        assert_eq!(
            written.canonical_uncompressed_bytes,
            u64::try_from(record.len()).expect("record length")
        );
        assert!(
            written
                .stored_record_bytes
                .checked_mul(10)
                .is_some_and(|scaled| scaled < written.canonical_uncompressed_bytes),
            "repetitive canonical evidence must compress below ten percent"
        );
        let (reconstructed, read) =
            reconstruct_canonical_record_spool(&path).expect("reconstruct compressed spool");
        assert_eq!(written, read);
        assert_eq!(reconstructed, vec![record]);
        fs::remove_dir_all(root).expect("remove test directory");
    }

    #[test]
    fn streamed_record_seal_substitution_rolls_back_before_acknowledgement() {
        let root = directory("streamed-seal-substitution");
        let path = root.join("evidence.spool");
        let record = b"canonical-streamed-day";
        let length = u64::try_from(record.len()).expect("record length");
        let digest = digest_bytes(record);
        let mut writer = CanonicalRecordSpoolWriter::create(&path).expect("create spool");
        let beginning = writer.current_seal();
        let beginning_length = fs::metadata(&path).expect("spool metadata").len();

        writer
            .append_durable_stream(length + 1, digest, |output| output.write_all(record))
            .expect_err("length substitution must reject");
        assert_eq!(writer.current_seal(), beginning);
        assert_eq!(
            fs::metadata(&path).expect("rolled-back metadata").len(),
            beginning_length
        );

        writer
            .append_durable_stream(length, digest_bytes(b"substituted"), |output| {
                output.write_all(record)
            })
            .expect_err("digest substitution must reject");
        assert_eq!(writer.current_seal(), beginning);
        assert_eq!(
            fs::metadata(&path).expect("rolled-back metadata").len(),
            beginning_length
        );

        assert_eq!(
            writer
                .append_durable_stream(length, digest, |output| output.write_all(record))
                .expect("exact streamed retry"),
            digest
        );
        let seal = writer.finish().expect("finish spool");
        let (records, reconstructed) =
            reconstruct_canonical_record_spool(&path).expect("reconstruct streamed spool");
        assert_eq!(seal, reconstructed);
        assert_eq!(records, vec![record.to_vec()]);
        fs::remove_dir_all(root).expect("remove test directory");
    }

    #[test]
    fn compressed_record_substitution_rejects_before_canonical_reconstruction() {
        let root = directory("compressed-substitution");
        let path = root.join("evidence.spool");
        let mut writer = CanonicalRecordSpoolWriter::create(&path).expect("create spool");
        writer
            .append_durable(&vec![b'y'; 64 * 1024])
            .expect("append record");
        writer.finish().expect("finish spool");

        let payload_offset =
            u64::try_from(RECORD_STREAM_MAGIC.len()).expect("magic length") + 1 + 8 + 8 + 32;
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path)
            .expect("open spool");
        file.seek(SeekFrom::Start(payload_offset + 2))
            .expect("seek compressed payload");
        let mut byte = [0_u8; 1];
        file.read_exact(&mut byte).expect("read compressed byte");
        byte[0] ^= 0x40;
        file.seek(SeekFrom::Start(payload_offset + 2))
            .expect("seek substituted payload");
        file.write_all(&byte).expect("write substituted byte");
        file.sync_all().expect("sync substituted spool");
        assert!(reconstruct_canonical_record_spool(&path).is_err());
        fs::remove_dir_all(root).expect("remove test directory");
    }

    #[test]
    fn durable_append_sync_failure_retains_preceding_count_root_and_file_boundary() {
        let root = directory("durable-failure");
        let path = root.join("evidence.spool");
        let mut writer = CanonicalRecordSpoolWriter::create(&path).expect("create spool");
        writer
            .append_durable(b"day-zero")
            .expect("durable day zero");
        let before = writer.current_seal();
        let before_len = fs::metadata(&path).expect("spool metadata").len();
        writer.force_sync_failure_once();
        writer
            .append_durable(b"day-one-must-not-be-acknowledged")
            .expect_err("forced sync failure");
        assert_eq!(writer.current_seal(), before);
        assert_eq!(
            fs::metadata(&path).expect("rolled-back metadata").len(),
            before_len
        );
        writer
            .append_durable(b"day-one-retry")
            .expect("durable retry");
        let seal = writer.finish().expect("finish spool");
        let (records, reconstructed) =
            reconstruct_canonical_record_spool(&path).expect("reconstruct durable spool");
        assert_eq!(seal, reconstructed);
        assert_eq!(
            records,
            vec![b"day-zero".to_vec(), b"day-one-retry".to_vec()]
        );
        fs::remove_dir_all(root).expect("remove test directory");
    }

    #[test]
    fn private_spool_promotes_complete_set_without_public_paths() {
        let root = directory("promote");
        let private_a = root.join("a.private");
        let private_b = root.join("b.private");
        let staged_a = root.join("a.stage");
        let staged_b = root.join("b.stage");
        let spool = RunnerTransactionPrivateSpool::new([
            (private_a.clone(), staged_a.clone()),
            (private_b.clone(), staged_b.clone()),
        ])
        .expect("private spool");
        fs::write(&private_a, b"a").expect("write a");
        fs::write(&private_b, b"b").expect("write b");
        spool.promote_complete().expect("promote complete spool");
        assert_eq!(fs::read(&staged_a).expect("read a"), b"a");
        assert_eq!(fs::read(&staged_b).expect("read b"), b"b");
        assert!(!private_a.exists() && !private_b.exists());
        drop(spool);
        assert!(!staged_a.exists() && !staged_b.exists());
        fs::remove_dir_all(root).expect("remove test directory");
    }

    #[test]
    fn incomplete_private_spool_rolls_back_every_promoted_name() {
        let root = directory("rollback");
        let private_a = root.join("a.private");
        let private_b = root.join("b.private");
        let staged_a = root.join("a.stage");
        let staged_b = root.join("b.stage");
        let spool = RunnerTransactionPrivateSpool::new([
            (private_a.clone(), staged_a.clone()),
            (private_b, staged_b.clone()),
        ])
        .expect("private spool");
        fs::write(&private_a, b"a").expect("write only first record");
        spool
            .promote_complete()
            .expect_err("incomplete spool must reject");
        assert!(!staged_a.exists() && !staged_b.exists());
        drop(spool);
        assert!(!private_a.exists());
        fs::remove_dir_all(root).expect("remove test directory");
    }
}
