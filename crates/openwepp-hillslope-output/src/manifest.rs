use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputChecksumEntry {
    pub output_path: String,
    pub sha256: String,
}

impl OutputChecksumEntry {
    #[must_use]
    pub fn new(output_path: impl Into<String>, sha256: impl Into<String>) -> Self {
        Self {
            output_path: output_path.into(),
            sha256: sha256.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManifestAssemblyError {
    EmptyOutputPath,
    EmptyChecksum { output_path: String },
    DuplicateOutputPath { output_path: String },
}

impl ManifestAssemblyError {
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::EmptyOutputPath => "OHMAN-E-001",
            Self::EmptyChecksum { .. } => "OHMAN-E-002",
            Self::DuplicateOutputPath { .. } => "OHMAN-E-003",
        }
    }
}

impl fmt::Display for ManifestAssemblyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyOutputPath => write!(f, "{} output path cannot be empty", self.code()),
            Self::EmptyChecksum { output_path } => write!(
                f,
                "{} output checksum cannot be empty for {}",
                self.code(),
                output_path
            ),
            Self::DuplicateOutputPath { output_path } => {
                write!(f, "{} duplicate output path {}", self.code(), output_path)
            }
        }
    }
}

impl std::error::Error for ManifestAssemblyError {}

pub fn assemble_output_checksums(
    entries: &[OutputChecksumEntry],
) -> Result<BTreeMap<String, String>, ManifestAssemblyError> {
    let mut checksums = BTreeMap::new();

    for entry in entries {
        if entry.output_path.trim().is_empty() {
            return Err(ManifestAssemblyError::EmptyOutputPath);
        }
        if entry.sha256.trim().is_empty() {
            return Err(ManifestAssemblyError::EmptyChecksum {
                output_path: entry.output_path.clone(),
            });
        }

        if checksums
            .insert(entry.output_path.clone(), entry.sha256.clone())
            .is_some()
        {
            return Err(ManifestAssemblyError::DuplicateOutputPath {
                output_path: entry.output_path.clone(),
            });
        }
    }

    Ok(checksums)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_checksum_assembly_sorts_paths_deterministically() {
        let entries = vec![
            OutputChecksumEntry::new("output/H1.loss.json", "b"),
            OutputChecksumEntry::new("output/H1.pass.hbp", "a"),
            OutputChecksumEntry::new("output/H1.wat.parquet", "c"),
        ];

        let checksums = assemble_output_checksums(&entries).expect("assembly should succeed");
        let ordered_paths: Vec<&str> = checksums.keys().map(String::as_str).collect();

        assert_eq!(
            ordered_paths,
            vec![
                "output/H1.loss.json",
                "output/H1.pass.hbp",
                "output/H1.wat.parquet",
            ]
        );
    }

    #[test]
    fn manifest_checksum_assembly_rejects_duplicate_paths() {
        let entries = vec![
            OutputChecksumEntry::new("output/H1.pass.hbp", "a"),
            OutputChecksumEntry::new("output/H1.pass.hbp", "b"),
        ];

        let error = assemble_output_checksums(&entries).expect_err("duplicate should fail");
        assert_eq!(error.code(), "OHMAN-E-003");
        assert!(matches!(
            error,
            ManifestAssemblyError::DuplicateOutputPath { .. }
        ));
    }

    #[test]
    fn manifest_checksum_assembly_rejects_empty_checksum() {
        let entries = vec![OutputChecksumEntry::new("output/H1.pass.hbp", "")];

        let error = assemble_output_checksums(&entries).expect_err("empty checksum should fail");
        assert_eq!(error.code(), "OHMAN-E-002");
        assert!(matches!(error, ManifestAssemblyError::EmptyChecksum { .. }));
    }

    #[test]
    fn manifest_checksum_assembly_rejects_empty_output_path() {
        let entries = vec![OutputChecksumEntry::new("", "abcd")];

        let error = assemble_output_checksums(&entries).expect_err("empty path should fail");
        assert_eq!(error.code(), "OHMAN-E-001");
        assert!(matches!(error, ManifestAssemblyError::EmptyOutputPath));
    }
}
