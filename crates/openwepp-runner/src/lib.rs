#![forbid(unsafe_code)]
#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

mod api;
mod constants;
mod errors;
mod hillslope;
mod launch;
mod policy;
mod release;
mod role;
mod shared;
mod watershed_wat;

pub use api::{HillslopeRunReport, HillslopeRunRequest, ReleaseLintReport, RunnerLaunchRequest};
pub use constants::*;
pub use errors::{HillslopeCliError, ReleaseLintError, ReleaseMetadataError, RunnerError};
pub use hillslope::execute_hillslope_run;
pub use launch::{build_hillslope_argv, launch_hillslope};
pub use policy::SidecarPolicy;
pub use release::{
    lint_release_directory, validate_release_sidecar, write_release_sidecar_for_binary,
};
pub use role::BinaryRole;
pub use watershed_wat::{WatershedWatPublicationError, build_watershed_daily_rows_from_wat};
