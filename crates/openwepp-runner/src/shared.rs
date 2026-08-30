use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;
use std::process::Command;

use sha2::{Digest, Sha256};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

pub(crate) fn utc_now_rfc3339() -> Result<String, String> {
    OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .map_err(|error| format!("failed formatting UTC timestamp: {error}"))
}

#[must_use]
pub(crate) fn git_source_commit_or_unknown() -> String {
    Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "unknown".to_string())
}

pub(crate) fn sha256_file_hex(path: &Path) -> Result<String, io::Error> {
    let mut reader = BufReader::new(File::open(path)?);
    let mut hasher = Sha256::new();
    let mut buffer = vec![0_u8; 64 * 1024].into_boxed_slice();
    loop {
        let read = reader.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

#[must_use]
pub(crate) fn path_has_extension_case_insensitive(path: &Path, extension: &str) -> bool {
    path.extension()
        .and_then(OsStr::to_str)
        .is_some_and(|ext| ext.eq_ignore_ascii_case(extension))
}

#[must_use]
pub(crate) fn file_name_string(path: &Path) -> String {
    path.file_name()
        .and_then(OsStr::to_str)
        .map_or_else(String::new, ToString::to_string)
}
