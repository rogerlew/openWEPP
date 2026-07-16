use std::fmt;
use std::io;
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, AssuranceError>;

#[derive(Debug)]
pub enum AssuranceError {
    Usage(String),
    Io {
        path: PathBuf,
        source: io::Error,
    },
    Parse {
        path: PathBuf,
        message: String,
    },
    Invalid(String),
    Drift(String),
    SnapshotConflict(String),
    Recovery {
        primary: Box<Self>,
        recovery: Box<Self>,
    },
    CommittedCleanup {
        path: PathBuf,
        receipt_json: String,
        source: Box<Self>,
    },
}

impl AssuranceError {
    #[must_use]
    pub const fn exit_code(&self) -> i32 {
        match self {
            Self::Usage(_) => 2,
            Self::Drift(_) | Self::SnapshotConflict(_) => 4,
            Self::Io { .. }
            | Self::Parse { .. }
            | Self::Invalid(_)
            | Self::Recovery { .. }
            | Self::CommittedCleanup { .. } => 1,
        }
    }

    pub fn io(path: impl Into<PathBuf>, source: io::Error) -> Self {
        Self::Io {
            path: path.into(),
            source,
        }
    }
}

impl fmt::Display for AssuranceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usage(message) | Self::Invalid(message) | Self::Drift(message) => {
                formatter.write_str(message)
            }
            Self::SnapshotConflict(message) => write!(formatter, "snapshot conflict: {message}"),
            Self::Recovery { primary, recovery } => {
                write!(
                    formatter,
                    "{primary}; transaction recovery also failed: {recovery}"
                )
            }
            Self::CommittedCleanup {
                path,
                receipt_json,
                source,
            } => write!(
                formatter,
                "normalization committed and validated, but old-generation cleanup failed at {}: {source}; committed receipt:\n{receipt_json}",
                path.display(),
            ),
            Self::Io { path, source } => write!(formatter, "{}: {source}", path.display()),
            Self::Parse { path, message } => write!(formatter, "{}: {message}", path.display()),
        }
    }
}

impl std::error::Error for AssuranceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::Recovery { primary, .. } => Some(primary.as_ref()),
            Self::CommittedCleanup { source, .. } => Some(source.as_ref()),
            _ => None,
        }
    }
}
