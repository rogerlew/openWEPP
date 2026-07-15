use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use crate::error::{AssuranceError, Result};

#[must_use]
pub fn sha256_bytes(bytes: &[u8]) -> String {
    hex_digest(Sha256::digest(bytes).as_slice())
}

/// Computes a streaming SHA-256 identity for a file.
///
/// # Errors
///
/// Returns an error when the file cannot be opened or read.
pub fn sha256_file(path: &Path) -> Result<String> {
    let mut file = File::open(path).map_err(|error| AssuranceError::io(path, error))?;
    let expected = file
        .metadata()
        .map_err(|error| AssuranceError::io(path, error))?
        .len();
    let mut hasher = Sha256::new();
    let mut buffer = vec![0_u8; 64 * 1024].into_boxed_slice();
    let mut observed = 0_u64;
    loop {
        let count = file
            .read(&mut buffer)
            .map_err(|error| AssuranceError::io(path, error))?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
        observed += count as u64;
    }
    if observed != expected {
        return Err(AssuranceError::Drift(format!(
            "file changed while hashing: {}",
            path.display()
        )));
    }
    Ok(hex_digest(hasher.finalize().as_slice()))
}

pub(crate) fn hash_named_files(root: &Path, paths: &[PathBuf], domain: &str) -> Result<String> {
    let mut ordered = paths.to_vec();
    ordered.sort();
    let mut hasher = Sha256::new();
    add_field(&mut hasher, domain.as_bytes());
    for relative in ordered {
        let display = relative.to_string_lossy();
        add_field(&mut hasher, display.as_bytes());
        let absolute = root.join(&relative);
        add_file_field(&mut hasher, &absolute)?;
    }
    Ok(hex_digest(hasher.finalize().as_slice()))
}

fn add_file_field(hasher: &mut Sha256, path: &Path) -> Result<()> {
    let mut file = File::open(path).map_err(|error| AssuranceError::io(path, error))?;
    let expected = file
        .metadata()
        .map_err(|error| AssuranceError::io(path, error))?
        .len();
    hasher.update(expected.to_be_bytes());
    let mut observed = 0_u64;
    let mut buffer = vec![0_u8; 64 * 1024].into_boxed_slice();
    loop {
        let count = file
            .read(&mut buffer)
            .map_err(|error| AssuranceError::io(path, error))?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
        observed += count as u64;
    }
    if observed != expected {
        return Err(AssuranceError::Drift(format!(
            "file changed while hashing named inputs: {}",
            path.display()
        )));
    }
    Ok(())
}

fn add_field(hasher: &mut Sha256, bytes: &[u8]) {
    hasher.update((bytes.len() as u64).to_be_bytes());
    hasher.update(bytes);
}

fn hex_digest(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}
