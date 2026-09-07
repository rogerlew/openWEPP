//! Test-only bridge to the canonical directory parser. No maintained aggregate.
use std::path::Path;
use std::process::Command;

pub(super) fn read(path: impl AsRef<Path>) -> std::io::Result<String> {
    let path = path.as_ref();
    // Dispatch contract entries through the canonical metadata parser, including
    // malformed format metadata. Noncontract source/docs remain raw.
    let is_contract = path
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.starts_with("SC-"))
        && path.extension() == Some(std::ffi::OsStr::new("md"));
    if !is_contract {
        return std::fs::read_to_string(path);
    }
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let output = Command::new(root.join(".venv/bin/python"))
        .arg(root.join("tools/sc_contract_directory.py"))
        .arg(path)
        .output()?;
    if !output.status.success() {
        return Err(std::io::Error::other(
            String::from_utf8_lossy(&output.stderr).into_owned(),
        ));
    }
    String::from_utf8(output.stdout).map_err(std::io::Error::other)
}
