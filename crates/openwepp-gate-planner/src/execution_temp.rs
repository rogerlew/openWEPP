//! Short, isolated process temporary directories for path-sensitive tools.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::error::{ErrorClass, GatePolicyError, Result};

static SEQUENCE: AtomicU64 = AtomicU64::new(0);

pub(crate) fn with_process_temp<T>(run: impl FnOnce(&Path) -> Result<T>) -> Result<T> {
    let temporary = ProcessTemp::create()?;
    let result = run(&temporary.path);
    temporary.close()?;
    result
}

struct ProcessTemp {
    path: PathBuf,
}

impl ProcessTemp {
    fn create() -> Result<Self> {
        #[cfg(unix)]
        let selected_base = PathBuf::from("/tmp");
        #[cfg(not(unix))]
        let selected_base = std::env::temp_dir();
        let base = fs::canonicalize(&selected_base)
            .map_err(|error| temp_error("GATE-EXEC-TMPDIR-BASE", error.to_string()))?;
        for _ in 0..1_024 {
            let sequence = SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let path = base.join(format!("owg-{}-{sequence:x}", std::process::id()));
            #[cfg(unix)]
            if path.as_os_str().as_encoded_bytes().len() > 40 {
                return Err(temp_error(
                    "GATE-EXEC-TMPDIR-PATH-BUDGET",
                    path.display().to_string(),
                ));
            }
            let mut builder = fs::DirBuilder::new();
            #[cfg(unix)]
            {
                use std::os::unix::fs::DirBuilderExt;
                builder.mode(0o700);
            }
            match builder.create(&path) {
                Ok(()) => return Ok(Self { path }),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(error) => {
                    return Err(temp_error("GATE-EXEC-TMPDIR-CREATE", error.to_string()));
                }
            }
        }
        Err(temp_error(
            "GATE-EXEC-TMPDIR-COLLISION",
            "failed to reserve a unique short process temporary directory",
        ))
    }

    fn close(self) -> Result<()> {
        fs::remove_dir_all(&self.path)
            .map_err(|error| temp_error("GATE-EXEC-TMPDIR-CLEANUP", error.to_string()))
    }
}

fn temp_error(code: &'static str, message: impl Into<String>) -> GatePolicyError {
    GatePolicyError::new(ErrorClass::Io, code, message)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::with_process_temp;
    use crate::error::{ErrorClass, GatePolicyError};

    #[test]
    fn process_temp_is_short_unique_and_cleaned_after_every_outcome() {
        let mut successful = None;
        with_process_temp(|path| {
            successful = Some(path.to_owned());
            fs::write(path.join("marker"), b"temporary").expect("write marker");
            #[cfg(unix)]
            assert!(path.as_os_str().as_encoded_bytes().len() <= 40);
            Ok(())
        })
        .expect("successful process temporary directory");
        assert!(!successful.expect("successful path").exists());

        let mut failed = None;
        let error = with_process_temp(|path| {
            failed = Some(path.to_owned());
            Err::<(), _>(GatePolicyError::new(
                ErrorClass::Execution,
                "GATE-EXEC-INJECTED",
                "injected process failure",
            ))
        })
        .expect_err("injected failure");
        assert_eq!(error.code, "GATE-EXEC-INJECTED");
        assert!(!failed.expect("failed path").exists());
    }
}
