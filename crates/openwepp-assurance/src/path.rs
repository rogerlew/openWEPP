use std::fs;
use std::path::{Component, Path, PathBuf};

use crate::error::{AssuranceError, Result};

pub(crate) fn validate_relative(path: &Path, label: &str) -> Result<()> {
    if path.as_os_str().is_empty() || path.is_absolute() {
        return Err(AssuranceError::Invalid(format!(
            "{label} must be a nonempty repository-relative path: {}",
            path.display()
        )));
    }
    let mut canonical = String::new();
    for component in path.components() {
        let Component::Normal(segment) = component else {
            return Err(AssuranceError::Invalid(format!(
                "{label} contains forbidden traversal or prefix: {}",
                path.display()
            )));
        };
        let Some(segment) = segment.to_str() else {
            return Err(AssuranceError::Invalid(format!(
                "{label} is not portable UTF-8: {}",
                path.display()
            )));
        };
        if segment.is_empty()
            || !segment
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
        {
            return Err(AssuranceError::Invalid(format!(
                "{label} contains a nonportable path segment: {}",
                path.display()
            )));
        }
        if !canonical.is_empty() {
            canonical.push('/');
        }
        canonical.push_str(segment);
    }
    if path.to_str() != Some(canonical.as_str()) {
        return Err(AssuranceError::Invalid(format!(
            "{label} is not a canonical portable repository path: {}",
            path.display()
        )));
    }
    Ok(())
}

pub(crate) fn existing_file(root: &Path, relative: &Path, label: &str) -> Result<PathBuf> {
    validate_relative(relative, label)?;
    reject_symlinks_below(root, relative, label)?;
    let candidate = root.join(relative);
    let canonical = candidate
        .canonicalize()
        .map_err(|error| AssuranceError::io(&candidate, error))?;
    if !canonical.starts_with(root) || !canonical.is_file() {
        return Err(AssuranceError::Invalid(format!(
            "{label} escapes the repository or is not a file: {}",
            relative.display()
        )));
    }
    Ok(canonical)
}

pub(crate) fn safe_output(root: &Path, relative: &Path, label: &str) -> Result<PathBuf> {
    validate_relative(relative, label)?;
    let candidate = root.join(relative);
    reject_symlinks_below(root, relative, label)?;
    let existing = nearest_existing(&candidate)?;
    let canonical = existing
        .canonicalize()
        .map_err(|error| AssuranceError::io(&existing, error))?;
    if !canonical.starts_with(root) {
        return Err(AssuranceError::Invalid(format!(
            "{label} follows a symlink outside its output root: {}",
            relative.display()
        )));
    }
    Ok(candidate)
}

pub(crate) fn create_dir_all_no_symlinks(path: &Path, label: &str) -> Result<()> {
    if path.as_os_str().is_empty() {
        return Err(AssuranceError::Invalid(format!(
            "{label} must be a nonempty directory path"
        )));
    }
    let mut current = PathBuf::new();
    for component in path.components() {
        current.push(component.as_os_str());
        match fs::symlink_metadata(&current) {
            Ok(metadata) => validate_directory_component(&current, &metadata, label)?,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                match fs::create_dir(&current) {
                    Ok(()) => {}
                    Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                        let metadata = fs::symlink_metadata(&current)
                            .map_err(|error| AssuranceError::io(&current, error))?;
                        validate_directory_component(&current, &metadata, label)?;
                    }
                    Err(error) => return Err(AssuranceError::io(&current, error)),
                }
            }
            Err(error) => return Err(AssuranceError::io(&current, error)),
        }
    }
    Ok(())
}

pub(crate) fn validate_snapshot_id(value: &str) -> Result<()> {
    let valid = !value.is_empty()
        && !value.starts_with('.')
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'-' | b'_' | b'.')
        });
    if valid {
        Ok(())
    } else {
        Err(AssuranceError::Invalid(format!(
            "unsafe snapshot ID '{value}'; use lowercase ASCII letters, digits, '.', '_', or '-'"
        )))
    }
}

fn nearest_existing(path: &Path) -> Result<PathBuf> {
    let mut current = path.to_path_buf();
    loop {
        if current.exists() {
            return Ok(current);
        }
        if !current.pop() {
            return Err(AssuranceError::Invalid(format!(
                "output has no existing containment root: {}",
                path.display()
            )));
        }
    }
}

fn validate_directory_component(path: &Path, metadata: &fs::Metadata, label: &str) -> Result<()> {
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        Err(AssuranceError::Invalid(format!(
            "{label} contains a symlink or non-directory component: {}",
            path.display()
        )))
    } else {
        Ok(())
    }
}

fn reject_symlinks_below(root: &Path, relative: &Path, label: &str) -> Result<()> {
    let mut current = root.to_path_buf();
    for component in relative.components() {
        current.push(component.as_os_str());
        match fs::symlink_metadata(&current) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return Err(AssuranceError::Invalid(format!(
                    "{label} contains a symlink component: {}",
                    current.display()
                )));
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(AssuranceError::io(&current, error)),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::validate_relative;

    #[test]
    fn accepts_only_canonical_portable_relative_paths() {
        for value in [
            "assurance/catalog.yaml",
            "Cargo.toml",
            "usersum/A-1_file.md",
        ] {
            assert!(validate_relative(Path::new(value), "test path").is_ok());
        }
        for value in [
            "",
            "/etc/passwd",
            "../private",
            "assurance/./catalog.yaml",
            "assurance//catalog.yaml",
            "assurance/has space.yaml",
            "assurance/has`tick.yaml",
            "assurance/has[bracket].yaml",
            "assurance/has\\backslash.yaml",
            "assurance/café.yaml",
            "assurance/line\nbreak.yaml",
        ] {
            assert!(validate_relative(Path::new(value), "test path").is_err());
        }
    }
}
