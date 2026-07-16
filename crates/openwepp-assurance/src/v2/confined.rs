use std::collections::BTreeSet;
use std::ffi::{OsStr, OsString};
use std::path::{Component, Path, PathBuf};

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

/// An opened directory capability used for race-safe staging operations.
pub(super) struct ConfinedDirectory {
    #[cfg(unix)]
    pub(super) directory: std::fs::File,
}

impl ConfinedDirectory {
    /// Opens an absolute directory path without following symlinks, creating
    /// missing components descriptor-relatively when requested.
    pub(super) fn open_ambient(path: &Path, create: bool) -> Result<Self> {
        if !path.is_absolute() {
            return Err(AssuranceError::Invalid(
                "staging root must resolve to an absolute path".to_owned(),
            ));
        }
        open_ambient_platform(path, create)
    }

    pub(super) fn create_dir_all(&self, relative: &Path) -> Result<()> {
        validate_relative(relative)?;
        create_dir_all_platform(self, relative).map(|_| ())
    }

    pub(super) fn ensure_directory(&self, relative: &Path) -> Result<()> {
        validate_relative(relative)?;
        open_directory_path_platform(self, relative).map(|_| ())
    }

    pub(super) fn directory_exists(&self, relative: &Path) -> Result<bool> {
        validate_relative(relative)?;
        directory_exists_platform(self, relative)
    }

    pub(super) fn remove_directory_if_exists(&self, relative: &Path) -> Result<bool> {
        validate_relative(relative)?;
        remove_directory_if_exists_platform(self, relative)
    }

    pub(super) fn remove_regular_if_exists(&self, relative: &Path) -> Result<bool> {
        validate_relative(relative)?;
        remove_regular_if_exists_platform(self, relative)
    }

    pub(super) fn write_new(&self, relative: &Path, bytes: &[u8]) -> Result<()> {
        validate_relative(relative)?;
        write_new_platform(self, relative, bytes)
    }

    pub(super) fn read_regular(&self, relative: &Path) -> Result<Vec<u8>> {
        validate_relative(relative)?;
        read_regular_platform(self, relative)
    }

    pub(super) fn rename(&self, source: &Path, target: &Path) -> Result<()> {
        validate_relative(source)?;
        validate_relative(target)?;
        rename_platform(self, source, target)
    }

    pub(super) fn rename_noreplace(&self, source: &Path, target: &Path) -> Result<()> {
        validate_relative(source)?;
        validate_relative(target)?;
        rename_noreplace_platform(self, source, target)
    }

    pub(super) fn exchange(&self, left: &Path, right: &Path) -> Result<()> {
        validate_relative(left)?;
        validate_relative(right)?;
        exchange_platform(self, left, right)
    }

    pub(super) fn lock_exclusive(&self, display: &Path) -> Result<()> {
        lock_exclusive_platform(self, display)
    }

    pub(super) fn collect_regular_files(&self, relative: &Path) -> Result<BTreeSet<PathBuf>> {
        validate_relative(relative)?;
        collect_regular_files_platform(self, relative)
    }

    pub(super) fn collect_all_regular_files(&self) -> Result<BTreeSet<PathBuf>> {
        collect_all_regular_files_platform(self)
    }

    pub(super) fn collect_directories(&self, relative: &Path) -> Result<BTreeSet<PathBuf>> {
        validate_relative(relative)?;
        collect_directories_platform(self, relative)
    }

    pub(super) fn sync_tree(&self, relative: &Path) -> Result<()> {
        validate_relative(relative)?;
        sync_tree_platform(self, relative)
    }

    /// Returns whether this held directory is the same directory as, or
    /// contains, another held directory. The traversal follows directory
    /// mount points through opened descriptors, so bind-mounted aliases do
    /// not evade the relationship check merely by using an unrelated ambient
    /// pathname.
    pub(super) fn contains_directory(&self, other: &Self) -> Result<bool> {
        contains_directory_platform(self, other)
    }

    /// Confirms that an ambient pathname still identifies this held directory
    /// capability without following symlinks.
    pub(super) fn verify_ambient_identity(&self, path: &Path) -> Result<()> {
        let reopened = Self::open_ambient(path, false)?;
        verify_same_directory_platform(self, &reopened, path)
    }
}

#[cfg(unix)]
fn verify_same_directory_platform(
    expected: &ConfinedDirectory,
    observed: &ConfinedDirectory,
    path: &Path,
) -> Result<()> {
    use std::os::unix::fs::MetadataExt as _;

    let expected = expected
        .directory
        .metadata()
        .map_err(|error| AssuranceError::io(path, error))?;
    let observed = observed
        .directory
        .metadata()
        .map_err(|error| AssuranceError::io(path, error))?;
    if expected.dev() == observed.dev() && expected.ino() == observed.ino() {
        Ok(())
    } else {
        Err(AssuranceError::Drift(format!(
            "staging root identity changed during operation: {}",
            path.display()
        )))
    }
}

#[cfg(unix)]
fn contains_directory_platform(
    root: &ConfinedDirectory,
    other: &ConfinedDirectory,
) -> Result<bool> {
    use std::os::unix::fs::MetadataExt as _;

    let target = other
        .directory
        .metadata()
        .map_err(|error| AssuranceError::io(Path::new("held-directory"), error))?;
    let target = (target.dev(), target.ino());
    let mut visited = BTreeSet::new();
    contains_directory_identity(&root.directory, target, &mut visited, Path::new("."))
}

#[cfg(unix)]
fn contains_directory_identity(
    directory: &std::fs::File,
    target: (u64, u64),
    visited: &mut BTreeSet<(u64, u64)>,
    display: &Path,
) -> Result<bool> {
    use std::os::unix::fs::MetadataExt as _;

    let metadata = directory
        .metadata()
        .map_err(|error| AssuranceError::io(display, error))?;
    let identity = (metadata.dev(), metadata.ino());
    if identity == target {
        return Ok(true);
    }
    if !visited.insert(identity) {
        return Ok(false);
    }
    for name in directory_entries(directory, display)? {
        let path = display.join(&name);
        let stat = stat_at(directory, &name)
            .map_err(|error| AssuranceError::io(&path, error))?
            .ok_or_else(|| {
                AssuranceError::Drift(format!(
                    "directory entry disappeared during root ancestry check: {}",
                    path.display()
                ))
            })?;
        if file_kind(stat.st_mode) != libc::S_IFDIR {
            continue;
        }
        let child = open_directory_at_io(directory, &name)
            .map_err(|error| component_error(error, &path, false))?;
        if contains_directory_identity(&child, target, visited, &path)? {
            return Ok(true);
        }
    }
    Ok(false)
}

#[cfg(not(unix))]
fn contains_directory_platform(
    _root: &ConfinedDirectory,
    _other: &ConfinedDirectory,
) -> Result<bool> {
    Err(AssuranceError::Invalid(
        "descriptor-relative confined staging requires Unix openat support".to_owned(),
    ))
}

#[cfg(not(unix))]
fn verify_same_directory_platform(
    _expected: &ConfinedDirectory,
    _observed: &ConfinedDirectory,
    _path: &Path,
) -> Result<()> {
    Err(AssuranceError::Invalid(
        "descriptor-relative confined staging requires Unix openat support".to_owned(),
    ))
}

#[cfg(unix)]
fn open_ambient_platform(path: &Path, create: bool) -> Result<ConfinedDirectory> {
    if !path.is_absolute() {
        return Err(AssuranceError::Invalid(format!(
            "confined ambient path must be absolute: {}",
            path.display()
        )));
    }
    let mut directory = open_root(Path::new("/"))?;
    for component in path.components() {
        let Some(name) = ambient_component_name(component, path)? else {
            continue;
        };
        directory = open_or_create_ambient_child(&directory, name, path, create)?;
    }
    Ok(ConfinedDirectory { directory })
}

#[cfg(unix)]
fn ambient_component_name<'a>(
    component: Component<'a>,
    path: &Path,
) -> Result<Option<&'a std::ffi::OsStr>> {
    match component {
        Component::RootDir => Ok(None),
        Component::Normal(name) => Ok(Some(name)),
        Component::CurDir | Component::ParentDir | Component::Prefix(_) => {
            Err(AssuranceError::Invalid(format!(
                "confined ambient path must be normalized: {}",
                path.display()
            )))
        }
    }
}

#[cfg(unix)]
fn open_or_create_ambient_child(
    parent: &std::fs::File,
    name: &std::ffi::OsStr,
    path: &Path,
    create: bool,
) -> Result<std::fs::File> {
    match open_directory_at_io(parent, name) {
        Ok(next) => Ok(next),
        Err(error) if create && error.kind() == std::io::ErrorKind::NotFound => {
            mkdir_at(parent, name).map_err(|source| AssuranceError::io(path, source))?;
            parent
                .sync_all()
                .map_err(|source| AssuranceError::io(path, source))?;
            open_directory_at_io(parent, name).map_err(|source| AssuranceError::io(path, source))
        }
        Err(error) => Err(component_error(error, path, false)),
    }
}

#[cfg(not(unix))]
fn open_ambient_platform(_path: &Path, _create: bool) -> Result<ConfinedDirectory> {
    Err(AssuranceError::Invalid(
        "descriptor-relative confined staging requires Unix openat support".to_owned(),
    ))
}

#[cfg(unix)]
fn create_dir_all_platform(root: &ConfinedDirectory, relative: &Path) -> Result<std::fs::File> {
    let mut directory = root
        .directory
        .try_clone()
        .map_err(|error| AssuranceError::io(relative, error))?;
    for component in relative.components() {
        let Component::Normal(name) = component else {
            return Err(AssuranceError::Invalid(
                "staging output path is not confined".to_owned(),
            ));
        };
        match open_directory_at_io(&directory, name) {
            Ok(next) => directory = next,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                match mkdir_at(&directory, name) {
                    Ok(()) => {}
                    Err(source) if source.kind() == std::io::ErrorKind::AlreadyExists => {}
                    Err(source) => return Err(AssuranceError::io(relative, source)),
                }
                directory = open_directory_at_io(&directory, name)
                    .map_err(|source| component_error(source, relative, false))?;
            }
            Err(error) => return Err(component_error(error, relative, false)),
        }
    }
    Ok(directory)
}

#[cfg(not(unix))]
fn create_dir_all_platform(_root: &ConfinedDirectory, _relative: &Path) -> Result<std::fs::File> {
    Err(AssuranceError::Invalid(
        "descriptor-relative confined staging requires Unix openat support".to_owned(),
    ))
}

#[cfg(unix)]
fn open_directory_path_platform(
    root: &ConfinedDirectory,
    relative: &Path,
) -> Result<std::fs::File> {
    let mut directory = root
        .directory
        .try_clone()
        .map_err(|error| AssuranceError::io(relative, error))?;
    for component in relative.components() {
        let Component::Normal(name) = component else {
            return Err(AssuranceError::Invalid(
                "staging output path is not confined".to_owned(),
            ));
        };
        directory = open_directory_at_io(&directory, name)
            .map_err(|error| component_error(error, relative, false))?;
    }
    Ok(directory)
}

#[cfg(not(unix))]
fn open_directory_path_platform(
    _root: &ConfinedDirectory,
    _relative: &Path,
) -> Result<std::fs::File> {
    Err(AssuranceError::Invalid(
        "descriptor-relative confined staging requires Unix openat support".to_owned(),
    ))
}

#[cfg(unix)]
fn directory_exists_platform(root: &ConfinedDirectory, relative: &Path) -> Result<bool> {
    let (parent, name) = open_parent(root, relative)?;
    match stat_at(&parent, name).map_err(|error| AssuranceError::io(relative, error))? {
        None => Ok(false),
        Some(stat) if file_kind(stat.st_mode) == libc::S_IFDIR => Ok(true),
        Some(_) => Err(AssuranceError::Invalid(format!(
            "staged output target is not a real directory: {}",
            relative.display()
        ))),
    }
}

#[cfg(not(unix))]
fn directory_exists_platform(_root: &ConfinedDirectory, _relative: &Path) -> Result<bool> {
    Err(AssuranceError::Invalid(
        "descriptor-relative confined staging requires Unix openat support".to_owned(),
    ))
}

#[cfg(unix)]
fn remove_directory_if_exists_platform(root: &ConfinedDirectory, relative: &Path) -> Result<bool> {
    let (parent, name) = open_parent(root, relative)?;
    let Some(stat) = stat_at(&parent, name).map_err(|error| AssuranceError::io(relative, error))?
    else {
        return Ok(false);
    };
    if file_kind(stat.st_mode) != libc::S_IFDIR {
        return Err(AssuranceError::Invalid(format!(
            "staging replacement path is not a real directory: {}",
            relative.display()
        )));
    }
    let directory = open_directory_at_io(&parent, name)
        .map_err(|error| component_error(error, relative, false))?;
    remove_directory_contents(&directory, relative)?;
    unlink_at(&parent, name, libc::AT_REMOVEDIR)
        .map_err(|error| AssuranceError::io(relative, error))?;
    Ok(true)
}

#[cfg(not(unix))]
fn remove_directory_if_exists_platform(
    _root: &ConfinedDirectory,
    _relative: &Path,
) -> Result<bool> {
    Err(AssuranceError::Invalid(
        "descriptor-relative confined staging requires Unix openat support".to_owned(),
    ))
}

#[cfg(unix)]
fn remove_regular_if_exists_platform(root: &ConfinedDirectory, relative: &Path) -> Result<bool> {
    let (parent, name) = open_parent(root, relative)?;
    let Some(stat) = stat_at(&parent, name).map_err(|error| AssuranceError::io(relative, error))?
    else {
        return Ok(false);
    };
    if file_kind(stat.st_mode) != libc::S_IFREG {
        return Err(AssuranceError::Invalid(format!(
            "staging removal target is not a regular file: {}",
            relative.display()
        )));
    }
    unlink_at(&parent, name, 0).map_err(|error| AssuranceError::io(relative, error))?;
    Ok(true)
}

#[cfg(not(unix))]
fn remove_regular_if_exists_platform(_root: &ConfinedDirectory, _relative: &Path) -> Result<bool> {
    Err(AssuranceError::Invalid(
        "descriptor-relative confined staging requires Unix openat support".to_owned(),
    ))
}

#[cfg(unix)]
fn write_new_platform(root: &ConfinedDirectory, relative: &Path, bytes: &[u8]) -> Result<()> {
    use std::io::Write as _;

    let parent_path = relative
        .parent()
        .ok_or_else(|| AssuranceError::Invalid("staging output file has no parent".to_owned()))?;
    let parent = if parent_path.as_os_str().is_empty() {
        root.directory
            .try_clone()
            .map_err(|error| AssuranceError::io(relative, error))?
    } else {
        create_dir_all_platform(root, parent_path)?
    };
    let name = relative
        .file_name()
        .ok_or_else(|| AssuranceError::Invalid("staging output file has no name".to_owned()))?;
    let flags = libc::O_WRONLY
        | libc::O_CREAT
        | libc::O_EXCL
        | libc::O_CLOEXEC
        | libc::O_NOFOLLOW
        | libc::O_NONBLOCK;
    let mut file = open_at_with_mode(&parent, name, flags, 0o644)
        .map_err(|error| component_error(error, relative, true))?;
    let metadata = file
        .metadata()
        .map_err(|error| AssuranceError::io(relative, error))?;
    if !metadata.is_file() {
        return Err(AssuranceError::Invalid(format!(
            "staging output target must be a regular file: {}",
            relative.display()
        )));
    }
    file.write_all(bytes)
        .map_err(|error| AssuranceError::io(relative, error))?;
    file.sync_all()
        .map_err(|error| AssuranceError::io(relative, error))
}

#[cfg(not(unix))]
fn write_new_platform(_root: &ConfinedDirectory, _relative: &Path, _bytes: &[u8]) -> Result<()> {
    Err(AssuranceError::Invalid(
        "descriptor-relative confined staging requires Unix openat support".to_owned(),
    ))
}

#[cfg(unix)]
fn read_regular_platform(root: &ConfinedDirectory, relative: &Path) -> Result<Vec<u8>> {
    let (parent, name) = open_parent(root, relative)?;
    let file = open_regular_at(&parent, name, relative)?;
    read_opened_regular(file, relative)
}

#[cfg(not(unix))]
fn read_regular_platform(_root: &ConfinedDirectory, _relative: &Path) -> Result<Vec<u8>> {
    Err(AssuranceError::Invalid(
        "descriptor-relative confined staging requires Unix openat support".to_owned(),
    ))
}

#[cfg(unix)]
fn rename_platform(root: &ConfinedDirectory, source: &Path, target: &Path) -> Result<()> {
    if source.parent() != target.parent() {
        return Err(AssuranceError::Invalid(
            "staging rename must remain within one parent directory".to_owned(),
        ));
    }
    let (parent, source_name) = open_parent(root, source)?;
    let target_name = target
        .file_name()
        .ok_or_else(|| AssuranceError::Invalid("staging rename target has no name".to_owned()))?;
    rename_at(&parent, source_name, target_name).map_err(|error| AssuranceError::io(source, error))
}

#[cfg(not(unix))]
fn rename_platform(_root: &ConfinedDirectory, _source: &Path, _target: &Path) -> Result<()> {
    Err(AssuranceError::Invalid(
        "descriptor-relative confined staging requires Unix openat support".to_owned(),
    ))
}

#[cfg(target_os = "linux")]
fn rename_noreplace_platform(root: &ConfinedDirectory, source: &Path, target: &Path) -> Result<()> {
    renameat2_platform(root, source, target, libc::RENAME_NOREPLACE)
}

#[cfg(not(target_os = "linux"))]
fn rename_noreplace_platform(
    _root: &ConfinedDirectory,
    _source: &Path,
    _target: &Path,
) -> Result<()> {
    Err(AssuranceError::Invalid(
        "assurance publication requires Linux renameat2 no-replace support".to_owned(),
    ))
}

#[cfg(target_os = "linux")]
fn exchange_platform(root: &ConfinedDirectory, left: &Path, right: &Path) -> Result<()> {
    renameat2_platform(root, left, right, libc::RENAME_EXCHANGE)
}

#[cfg(not(target_os = "linux"))]
fn exchange_platform(_root: &ConfinedDirectory, _left: &Path, _right: &Path) -> Result<()> {
    Err(AssuranceError::Invalid(
        "assurance publication requires Linux renameat2 exchange support".to_owned(),
    ))
}

#[cfg(target_os = "linux")]
fn renameat2_platform(
    root: &ConfinedDirectory,
    source: &Path,
    target: &Path,
    flags: libc::c_uint,
) -> Result<()> {
    use std::os::fd::AsRawFd as _;
    use std::os::unix::ffi::OsStrExt as _;

    if source.parent() != target.parent() {
        return Err(AssuranceError::Invalid(
            "publication rename must remain within one parent directory".to_owned(),
        ));
    }
    let (parent, source_name) = open_parent(root, source)?;
    let target_name = target
        .file_name()
        .ok_or_else(|| AssuranceError::Invalid("publication target has no name".to_owned()))?;
    let source = std::ffi::CString::new(source_name.as_bytes())
        .map_err(|_| AssuranceError::Invalid("publication source contains NUL".to_owned()))?;
    let target = std::ffi::CString::new(target_name.as_bytes())
        .map_err(|_| AssuranceError::Invalid("publication target contains NUL".to_owned()))?;
    // SAFETY: both names are confined single components, the retained parent
    // descriptor is live, and `flags` is one supported renameat2 operation.
    let result = unsafe {
        libc::renameat2(
            parent.as_raw_fd(),
            source.as_ptr(),
            parent.as_raw_fd(),
            target.as_ptr(),
            flags,
        )
    };
    if result == 0 {
        Ok(())
    } else {
        Err(AssuranceError::io(
            Path::new(source_name),
            std::io::Error::last_os_error(),
        ))
    }
}

#[cfg(unix)]
fn lock_exclusive_platform(root: &ConfinedDirectory, display: &Path) -> Result<()> {
    use std::os::fd::AsRawFd as _;

    // SAFETY: the directory descriptor is live for the duration of the held
    // `ConfinedDirectory`; `flock` changes only its advisory lock state.
    let result = unsafe { libc::flock(root.directory.as_raw_fd(), libc::LOCK_EX) };
    if result == 0 {
        Ok(())
    } else {
        Err(AssuranceError::io(display, std::io::Error::last_os_error()))
    }
}

#[cfg(not(unix))]
fn lock_exclusive_platform(_root: &ConfinedDirectory, _display: &Path) -> Result<()> {
    Err(AssuranceError::Invalid(
        "assurance publication requires Unix advisory locks".to_owned(),
    ))
}

#[cfg(unix)]
fn collect_regular_files_platform(
    root: &ConfinedDirectory,
    relative: &Path,
) -> Result<BTreeSet<PathBuf>> {
    let directory = open_directory_path_platform(root, relative)?;
    let mut files = BTreeSet::new();
    collect_directory_files(&directory, Path::new(""), relative, &mut files)?;
    Ok(files)
}

#[cfg(unix)]
fn collect_all_regular_files_platform(root: &ConfinedDirectory) -> Result<BTreeSet<PathBuf>> {
    let mut files = BTreeSet::new();
    collect_directory_files(&root.directory, Path::new(""), Path::new("."), &mut files)?;
    Ok(files)
}

#[cfg(unix)]
fn collect_directories_platform(
    root: &ConfinedDirectory,
    relative: &Path,
) -> Result<BTreeSet<PathBuf>> {
    let directory = open_directory_path_platform(root, relative)?;
    let mut directories = BTreeSet::new();
    collect_directory_paths(&directory, Path::new(""), relative, &mut directories)?;
    Ok(directories)
}

#[cfg(not(unix))]
fn collect_directories_platform(
    _root: &ConfinedDirectory,
    _relative: &Path,
) -> Result<BTreeSet<PathBuf>> {
    Err(AssuranceError::Invalid(
        "descriptor-relative confined staging requires Unix openat support".to_owned(),
    ))
}

#[cfg(unix)]
fn sync_tree_platform(root: &ConfinedDirectory, relative: &Path) -> Result<()> {
    let directory = open_directory_path_platform(root, relative)?;
    sync_directory_tree(&directory, relative)
}

#[cfg(not(unix))]
fn sync_tree_platform(_root: &ConfinedDirectory, _relative: &Path) -> Result<()> {
    Err(AssuranceError::Invalid(
        "descriptor-relative confined staging requires Unix openat support".to_owned(),
    ))
}

#[cfg(not(unix))]
fn collect_all_regular_files_platform(_root: &ConfinedDirectory) -> Result<BTreeSet<PathBuf>> {
    Err(AssuranceError::Invalid(
        "descriptor-relative confined staging requires Unix openat support".to_owned(),
    ))
}

#[cfg(not(unix))]
fn collect_regular_files_platform(
    _root: &ConfinedDirectory,
    _relative: &Path,
) -> Result<BTreeSet<PathBuf>> {
    Err(AssuranceError::Invalid(
        "descriptor-relative confined staging requires Unix openat support".to_owned(),
    ))
}

#[cfg(unix)]
fn open_parent<'a>(
    root: &ConfinedDirectory,
    relative: &'a Path,
) -> Result<(std::fs::File, &'a OsStr)> {
    let name = relative
        .file_name()
        .ok_or_else(|| AssuranceError::Invalid("staging path has no final component".to_owned()))?;
    let parent_path = relative
        .parent()
        .ok_or_else(|| AssuranceError::Invalid("staging path has no parent".to_owned()))?;
    let parent = if parent_path.as_os_str().is_empty() {
        root.directory
            .try_clone()
            .map_err(|error| AssuranceError::io(relative, error))?
    } else {
        open_directory_path_platform(root, parent_path)?
    };
    Ok((parent, name))
}

#[cfg(unix)]
fn remove_directory_contents(directory: &std::fs::File, display: &Path) -> Result<()> {
    for name in directory_entries(directory, display)? {
        let path = display.join(&name);
        let stat = stat_at(directory, &name)
            .map_err(|error| AssuranceError::io(&path, error))?
            .ok_or_else(|| {
                AssuranceError::Drift(format!("staging entry disappeared: {}", path.display()))
            })?;
        match file_kind(stat.st_mode) {
            libc::S_IFREG => {
                unlink_at(directory, &name, 0).map_err(|error| AssuranceError::io(&path, error))?;
            }
            libc::S_IFDIR => {
                let child = open_directory_at_io(directory, &name)
                    .map_err(|error| component_error(error, &path, false))?;
                remove_directory_contents(&child, &path)?;
                unlink_at(directory, &name, libc::AT_REMOVEDIR)
                    .map_err(|error| AssuranceError::io(&path, error))?;
            }
            _ => {
                return Err(AssuranceError::Invalid(format!(
                    "staging tree contains a symlink or special file: {}",
                    path.display()
                )));
            }
        }
    }
    Ok(())
}

#[cfg(unix)]
fn collect_directory_files(
    directory: &std::fs::File,
    relative: &Path,
    display: &Path,
    files: &mut BTreeSet<PathBuf>,
) -> Result<()> {
    for name in directory_entries(directory, display)? {
        let path = display.join(&name);
        let relative_path = relative.join(&name);
        let stat = stat_at(directory, &name)
            .map_err(|error| AssuranceError::io(&path, error))?
            .ok_or_else(|| {
                AssuranceError::Drift(format!("staging entry disappeared: {}", path.display()))
            })?;
        match file_kind(stat.st_mode) {
            libc::S_IFREG => {
                if stat.st_nlink != 1 {
                    return Err(AssuranceError::Invalid(format!(
                        "staging tree contains a multiply linked regular file: {}",
                        path.display()
                    )));
                }
                files.insert(relative_path);
            }
            libc::S_IFDIR => {
                let child = open_directory_at_io(directory, &name)
                    .map_err(|error| component_error(error, &path, false))?;
                collect_directory_files(&child, &relative_path, &path, files)?;
            }
            _ => {
                return Err(AssuranceError::Invalid(format!(
                    "staging tree contains a symlink or special file: {}",
                    path.display()
                )));
            }
        }
    }
    Ok(())
}

#[cfg(unix)]
fn collect_directory_paths(
    directory: &std::fs::File,
    relative: &Path,
    display: &Path,
    directories: &mut BTreeSet<PathBuf>,
) -> Result<()> {
    for name in directory_entries(directory, display)? {
        let path = display.join(&name);
        let relative_path = relative.join(&name);
        let stat = stat_at(directory, &name)
            .map_err(|error| AssuranceError::io(&path, error))?
            .ok_or_else(|| {
                AssuranceError::Drift(format!("staging entry disappeared: {}", path.display()))
            })?;
        match file_kind(stat.st_mode) {
            libc::S_IFREG => {}
            libc::S_IFDIR => {
                directories.insert(relative_path.clone());
                let child = open_directory_at_io(directory, &name)
                    .map_err(|error| component_error(error, &path, false))?;
                collect_directory_paths(&child, &relative_path, &path, directories)?;
            }
            _ => {
                return Err(AssuranceError::Invalid(format!(
                    "staging tree contains a symlink or special file: {}",
                    path.display()
                )));
            }
        }
    }
    Ok(())
}

#[cfg(unix)]
fn sync_directory_tree(directory: &std::fs::File, display: &Path) -> Result<()> {
    for name in directory_entries(directory, display)? {
        let path = display.join(&name);
        let stat = stat_at(directory, &name)
            .map_err(|error| AssuranceError::io(&path, error))?
            .ok_or_else(|| {
                AssuranceError::Drift(format!("staging entry disappeared: {}", path.display()))
            })?;
        if file_kind(stat.st_mode) == libc::S_IFDIR {
            let child = open_directory_at_io(directory, &name)
                .map_err(|error| component_error(error, &path, false))?;
            sync_directory_tree(&child, &path)?;
        }
    }
    directory
        .sync_all()
        .map_err(|error| AssuranceError::io(display, error))
}

#[cfg(unix)]
fn directory_entries(directory: &std::fs::File, display: &Path) -> Result<Vec<OsString>> {
    use std::os::fd::AsRawFd as _;
    use std::os::unix::ffi::OsStrExt as _;

    // SAFETY: `fcntl` duplicates a live directory descriptor; successful
    // ownership is transferred to `fdopendir` immediately below.
    let duplicate = unsafe { libc::fcntl(directory.as_raw_fd(), libc::F_DUPFD_CLOEXEC, 0) };
    if duplicate < 0 {
        return Err(AssuranceError::io(display, std::io::Error::last_os_error()));
    }
    // SAFETY: `duplicate` is a fresh owned directory descriptor. On success,
    // `DIR` owns and closes it; on failure we close it explicitly.
    let stream = unsafe { libc::fdopendir(duplicate) };
    if stream.is_null() {
        // SAFETY: `fdopendir` failed and did not consume `duplicate`.
        unsafe { libc::close(duplicate) };
        return Err(AssuranceError::io(display, std::io::Error::last_os_error()));
    }
    let mut names = Vec::new();
    loop {
        set_errno_zero();
        // SAFETY: `stream` remains live until `closedir` below; `readdir`
        // returns a borrowed entry valid until the next call.
        let entry = unsafe { libc::readdir(stream) };
        if entry.is_null() {
            let error = std::io::Error::last_os_error();
            // SAFETY: `stream` is a live `DIR` owned by this function.
            unsafe { libc::closedir(stream) };
            if error.raw_os_error() == Some(0) {
                break;
            }
            return Err(AssuranceError::io(display, error));
        }
        // SAFETY: POSIX guarantees `d_name` is NUL-terminated for this entry.
        let bytes = unsafe { std::ffi::CStr::from_ptr((*entry).d_name.as_ptr()) }.to_bytes();
        if bytes != b"." && bytes != b".." {
            names.push(OsStr::from_bytes(bytes).to_owned());
        }
    }
    names.sort();
    Ok(names)
}

#[cfg(target_os = "linux")]
fn set_errno_zero() {
    // SAFETY: `__errno_location` returns this thread's writable errno slot.
    unsafe { *libc::__errno_location() = 0 };
}

#[cfg(all(unix, not(target_os = "linux")))]
fn set_errno_zero() {
    // Other Unix targets are not currently supported by the descriptor
    // staging implementation.
}

#[cfg(unix)]
fn file_kind(mode: libc::mode_t) -> libc::mode_t {
    mode & libc::S_IFMT
}

#[cfg(unix)]
fn stat_at(directory: &std::fs::File, name: &OsStr) -> std::io::Result<Option<libc::stat>> {
    use std::mem::MaybeUninit;
    use std::os::fd::AsRawFd as _;

    let name = c_string(name)?;
    let mut stat = MaybeUninit::<libc::stat>::uninit();
    // SAFETY: descriptors and pointers are live; `stat` is initialized by a
    // successful `fstatat` before `assume_init`.
    let result = unsafe {
        libc::fstatat(
            directory.as_raw_fd(),
            name.as_ptr(),
            stat.as_mut_ptr(),
            libc::AT_SYMLINK_NOFOLLOW,
        )
    };
    if result == 0 {
        // SAFETY: successful `fstatat` initialized `stat`.
        return Ok(Some(unsafe { stat.assume_init() }));
    }
    let error = std::io::Error::last_os_error();
    if error.kind() == std::io::ErrorKind::NotFound {
        Ok(None)
    } else {
        Err(error)
    }
}

#[cfg(unix)]
fn mkdir_at(directory: &std::fs::File, name: &OsStr) -> std::io::Result<()> {
    use std::os::fd::AsRawFd as _;

    let name = c_string(name)?;
    // SAFETY: descriptor and NUL-terminated name are valid; mode is explicit.
    let result = unsafe { libc::mkdirat(directory.as_raw_fd(), name.as_ptr(), 0o755) };
    if result == 0 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error())
    }
}

#[cfg(unix)]
fn unlink_at(directory: &std::fs::File, name: &OsStr, flags: libc::c_int) -> std::io::Result<()> {
    use std::os::fd::AsRawFd as _;

    let name = c_string(name)?;
    // SAFETY: descriptor, name, and `unlinkat` flags are valid.
    let result = unsafe { libc::unlinkat(directory.as_raw_fd(), name.as_ptr(), flags) };
    if result == 0 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error())
    }
}

#[cfg(unix)]
fn rename_at(directory: &std::fs::File, source: &OsStr, target: &OsStr) -> std::io::Result<()> {
    use std::os::fd::AsRawFd as _;

    let source = c_string(source)?;
    let target = c_string(target)?;
    // SAFETY: both names and the retained parent descriptor are valid.
    let result = unsafe {
        libc::renameat(
            directory.as_raw_fd(),
            source.as_ptr(),
            directory.as_raw_fd(),
            target.as_ptr(),
        )
    };
    if result == 0 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error())
    }
}

#[cfg(unix)]
fn open_directory_at_io(directory: &std::fs::File, name: &OsStr) -> std::io::Result<std::fs::File> {
    let flags =
        libc::O_RDONLY | libc::O_CLOEXEC | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_NONBLOCK;
    open_at(directory, name, flags)
}

#[cfg(unix)]
fn open_at_with_mode(
    directory: &std::fs::File,
    name: &OsStr,
    flags: libc::c_int,
    mode: libc::mode_t,
) -> std::io::Result<std::fs::File> {
    use std::os::fd::{AsRawFd as _, FromRawFd as _};

    let name = c_string(name)?;
    // SAFETY: directory and name are valid; `O_CREAT` is paired with `mode`;
    // successful ownership transfers exactly once into `File`.
    let descriptor = unsafe { libc::openat(directory.as_raw_fd(), name.as_ptr(), flags, mode) };
    if descriptor < 0 {
        return Err(std::io::Error::last_os_error());
    }
    // SAFETY: `openat` returned a fresh owned descriptor.
    Ok(unsafe { std::fs::File::from_raw_fd(descriptor) })
}

#[cfg(unix)]
fn c_string(name: &OsStr) -> std::io::Result<std::ffi::CString> {
    use std::os::unix::ffi::OsStrExt as _;

    std::ffi::CString::new(name.as_bytes())
        .map_err(|_| std::io::Error::from(std::io::ErrorKind::InvalidInput))
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
    use std::os::unix::fs::MetadataExt as _;

    let metadata = file
        .metadata()
        .map_err(|error| AssuranceError::io(relative, error))?;
    if !metadata.is_file() {
        return Err(AssuranceError::Invalid(format!(
            "identified source must be a regular file: {}",
            relative.display()
        )));
    }
    if metadata.nlink() != 1 {
        return Err(AssuranceError::Invalid(format!(
            "identified source must not be multiply linked: {}",
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

    #[test]
    fn ambient_identity_detects_path_replacement() {
        let scratch = Scratch::new("ambient-identity");
        let staging = scratch.path.join("staging");
        let held = scratch.path.join("staging-held");
        let outside = scratch.path.join("outside");
        fs::create_dir(&staging).expect("create staging");
        fs::create_dir(&outside).expect("create outside");
        let directory =
            ConfinedDirectory::open_ambient(&staging, false).expect("open staging capability");
        directory
            .verify_ambient_identity(&staging)
            .expect("unchanged path retains identity");

        fs::rename(&staging, &held).expect("rename staging pathname");
        symlink(&outside, &staging).expect("replace staging path with symlink");
        assert!(directory.verify_ambient_identity(&staging).is_err());
    }

    #[test]
    fn ambient_open_creates_missing_components_and_rejects_unconfined_paths() {
        let scratch = Scratch::new("ambient-create");
        let created = scratch.path.join("one/two");
        ConfinedDirectory::open_ambient(&created, true).expect("create held ambient path");
        assert!(created.is_dir());
        assert!(ConfinedDirectory::open_ambient(Path::new("relative"), false).is_err());
        assert!(
            ConfinedDirectory::open_ambient(&scratch.path.join("one/../escape"), false).is_err()
        );
    }

    #[test]
    fn held_relationship_walks_descriptor_subtrees() {
        let scratch = Scratch::new("held-ancestry");
        let root_path = scratch.path.join("root");
        let child_path = root_path.join("nested/child");
        fs::create_dir_all(&child_path).expect("create nested directory");
        let root = ConfinedDirectory::open_ambient(&root_path, false).expect("open root");
        let child = ConfinedDirectory::open_ambient(&child_path, false).expect("open child");
        assert!(root.contains_directory(&child).expect("walk root"));
        assert!(!child.contains_directory(&root).expect("walk child"));
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
