use std::path::{Path, PathBuf};

#[allow(clippy::wildcard_imports)]
use super::*;

fn has_forbidden_pass_suffix(path_str: &str) -> bool {
    let lower = path_str.to_ascii_lowercase();
    lower.ends_with(".pass.hbp") || lower.ends_with(".pass.dat.hbp")
}

pub(super) fn resolve_path(
    input_path: &Path,
) -> Result<(PathBuf, HbpPathResolution, Vec<HbpWarning>), HbpParseError> {
    let raw = input_path.to_string_lossy();
    let lower = raw.to_ascii_lowercase();

    if has_forbidden_pass_suffix(&raw) {
        return Err(HbpParseError::InvalidProcessHbpName {
            input_path: input_path.to_path_buf(),
            reason: "use H*.hbp; rejecting H*.pass.hbp and H*.pass.dat.hbp".to_string(),
        });
    }

    if lower.ends_with(".hbp") {
        return Ok((
            input_path.to_path_buf(),
            HbpPathResolution::Direct,
            Vec::new(),
        ));
    }

    if lower.ends_with(".pass.dat") {
        return Err(HbpParseError::InvalidProcessHbpName {
            input_path: input_path.to_path_buf(),
            reason: "legacy .pass.dat naming is unsupported; use direct H*.hbp naming".to_string(),
        });
    }

    Err(HbpParseError::InvalidProcessHbpName {
        input_path: input_path.to_path_buf(),
        reason: "invalid process HBP name; use H*.hbp".to_string(),
    })
}
