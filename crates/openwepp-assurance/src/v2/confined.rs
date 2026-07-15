use std::path::{Component, Path};

use crate::{AssuranceError, Result};

pub(super) fn validate_relative(path: &Path) -> Result<()> {
    if path.as_os_str().is_empty()
        || path.is_absolute()
        || path
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(AssuranceError::Invalid(
            "source path must be a confined relative path".to_owned(),
        ));
    }
    Ok(())
}

pub(crate) fn read_regular_confined(root: &Path, relative: &Path) -> Result<Vec<u8>> {
    validate_relative(relative)?;
    read_regular_confined_platform(root, relative)
}

#[cfg(unix)]
fn read_regular_confined_platform(root: &Path, relative: &Path) -> Result<Vec<u8>> {
    use std::ffi::OsStr;

    let mut directory = open_root(root)?;
    let components = relative
        .components()
        .map(|component| match component {
            Component::Normal(name) => Ok(name),
            _ => Err(AssuranceError::Invalid(
                "source path must be a confined relative path".to_owned(),
            )),
        })
        .collect::<Result<Vec<&OsStr>>>()?;
    let (file_name, parents) = components.split_last().ok_or_else(|| {
        AssuranceError::Invalid("source path must be a confined relative path".to_owned())
    })?;
    for parent in parents {
        directory = open_directory_at(&directory, parent, relative)?;
    }
    let file = open_regular_at(&directory, file_name, relative)?;
    read_opened_regular(file, relative)
}

#[cfg(not(unix))]
fn read_regular_confined_platform(_root: &Path, _relative: &Path) -> Result<Vec<u8>> {
    Err(AssuranceError::Invalid(
        "descriptor-relative confined reads require Unix openat support".to_owned(),
    ))
}

#[cfg(unix)]
fn open_root(root: &Path) -> Result<std::fs::File> {
    use std::os::unix::fs::OpenOptionsExt as _;

    std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_CLOEXEC | libc::O_DIRECTORY | libc::O_NOFOLLOW)
        .open(root)
        .map_err(|error| AssuranceError::io(".", error))
}

#[cfg(unix)]
fn open_directory_at(
    directory: &std::fs::File,
    name: &std::ffi::OsStr,
    relative: &Path,
) -> Result<std::fs::File> {
    let flags =
        libc::O_RDONLY | libc::O_CLOEXEC | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_NONBLOCK;
    open_at(directory, name, flags).map_err(|error| component_error(error, relative, false))
}

#[cfg(unix)]
fn open_regular_at(
    directory: &std::fs::File,
    name: &std::ffi::OsStr,
    relative: &Path,
) -> Result<std::fs::File> {
    let flags = libc::O_RDONLY | libc::O_CLOEXEC | libc::O_NOFOLLOW | libc::O_NONBLOCK;
    open_at(directory, name, flags).map_err(|error| component_error(error, relative, true))
}

#[cfg(unix)]
fn open_at(
    directory: &std::fs::File,
    name: &std::ffi::OsStr,
    flags: libc::c_int,
) -> std::io::Result<std::fs::File> {
    use std::os::fd::{AsRawFd as _, FromRawFd as _};
    use std::os::unix::ffi::OsStrExt as _;

    let name = std::ffi::CString::new(name.as_bytes())
        .map_err(|_| std::io::Error::from(std::io::ErrorKind::InvalidInput))?;
    // SAFETY: `directory` owns a live descriptor, `name` is NUL-terminated and
    // has no interior NUL, no creation flag requires a mode argument, and a
    // successful descriptor is transferred exactly once into `File` below.
    let descriptor = unsafe { libc::openat(directory.as_raw_fd(), name.as_ptr(), flags) };
    if descriptor < 0 {
        return Err(std::io::Error::last_os_error());
    }
    // SAFETY: `openat` returned a new owned descriptor, and this is its sole
    // ownership transfer. `File` closes it exactly once.
    Ok(unsafe { std::fs::File::from_raw_fd(descriptor) })
}

#[cfg(unix)]
fn component_error(
    error: std::io::Error,
    relative: &Path,
    final_component: bool,
) -> AssuranceError {
    match error.raw_os_error() {
        Some(libc::ELOOP) => AssuranceError::Invalid(format!(
            "identified source cannot traverse a symlink: {}",
            relative.display()
        )),
        Some(libc::ENOTDIR) if !final_component => AssuranceError::Invalid(format!(
            "identified source parent is not a directory: {}",
            relative.display()
        )),
        Some(libc::ENXIO | libc::ENODEV) if final_component => AssuranceError::Invalid(format!(
            "identified source must be a regular file: {}",
            relative.display()
        )),
        _ => AssuranceError::io(relative, error),
    }
}

#[cfg(unix)]
fn read_opened_regular(mut file: std::fs::File, relative: &Path) -> Result<Vec<u8>> {
    use std::io::Read as _;

    let metadata = file
        .metadata()
        .map_err(|error| AssuranceError::io(relative, error))?;
    if !metadata.is_file() {
        return Err(AssuranceError::Invalid(format!(
            "identified source must be a regular file: {}",
            relative.display()
        )));
    }
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .map_err(|error| AssuranceError::io(relative, error))?;
    Ok(bytes)
}

#[cfg(all(test, unix))]
mod tests {
    use std::fs;
    use std::os::unix::fs::symlink;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::*;

    #[test]
    fn opened_directory_descriptor_survives_path_replacement() {
        let scratch = Scratch::new("directory-replacement");
        let relative = Path::new("source/value.txt");
        fs::create_dir(scratch.path.join("source")).expect("create source");
        fs::write(scratch.path.join(relative), b"inside").expect("write inside");
        fs::create_dir(scratch.path.join("outside")).expect("create outside");
        fs::write(scratch.path.join("outside/value.txt"), b"outside").expect("write outside");

        let root = open_root(&scratch.path).expect("open root descriptor");
        let source = open_directory_at(&root, std::ffi::OsStr::new("source"), relative)
            .expect("open source descriptor");
        fs::rename(
            scratch.path.join("source"),
            scratch.path.join("source-held"),
        )
        .expect("rename opened directory");
        symlink(scratch.path.join("outside"), scratch.path.join("source"))
            .expect("replace path with symlink");

        let opened = open_regular_at(&source, std::ffi::OsStr::new("value.txt"), relative)
            .expect("open through retained descriptor");
        assert_eq!(
            read_opened_regular(opened, relative).expect("read retained descriptor"),
            b"inside"
        );
        let error = read_regular_confined(&scratch.path, relative).unwrap_err();
        assert!(
            error.to_string().contains("symlink") || error.to_string().contains("not a directory")
        );
    }

    #[test]
    fn opened_file_descriptor_survives_final_path_replacement() {
        let scratch = Scratch::new("file-replacement");
        let relative = Path::new("source/value.txt");
        fs::create_dir(scratch.path.join("source")).expect("create source");
        fs::write(scratch.path.join(relative), b"inside").expect("write inside");
        fs::write(scratch.path.join("outside.txt"), b"outside").expect("write outside");

        let root = open_root(&scratch.path).expect("open root descriptor");
        let source = open_directory_at(&root, std::ffi::OsStr::new("source"), relative)
            .expect("open source descriptor");
        let opened = open_regular_at(&source, std::ffi::OsStr::new("value.txt"), relative)
            .expect("open value descriptor");
        fs::rename(
            scratch.path.join(relative),
            scratch.path.join("source/value-held.txt"),
        )
        .expect("rename opened file");
        symlink(
            scratch.path.join("outside.txt"),
            scratch.path.join(relative),
        )
        .expect("replace final path with symlink");

        assert_eq!(
            read_opened_regular(opened, relative).expect("read retained file"),
            b"inside"
        );
        assert!(
            read_regular_confined(&scratch.path, relative)
                .unwrap_err()
                .to_string()
                .contains("symlink")
        );
    }

    struct Scratch {
        path: PathBuf,
    }

    impl Scratch {
        fn new(label: &str) -> Self {
            static COUNTER: AtomicU64 = AtomicU64::new(0);
            let counter = COUNTER.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "openwepp-assurance-{label}-{}-{counter}",
                std::process::id()
            ));
            if path.exists() {
                fs::remove_dir_all(&path).expect("remove stale scratch");
            }
            fs::create_dir_all(&path).expect("create scratch");
            Self { path }
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            if self.path.exists() {
                fs::remove_dir_all(&self.path).expect("remove scratch");
            }
        }
    }
}
