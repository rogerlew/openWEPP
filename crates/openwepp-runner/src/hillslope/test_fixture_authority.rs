//! Explicit Stage-3 owner-seed authoring for integration fixtures.
//!
//! The checked H83 fixture authenticates one exact two-day, one-lane frame and
//! cannot represent tests that alter run identity, topology, climate span, or
//! parsed soil authority. This non-default feature authors a sealed artifact
//! from the already-prepared live test frame. Execution still consumes that
//! artifact through the ordinary explicit runfile or legacy-sidecar boundary.

use std::fs;
use std::path::PathBuf;

use crate::{HillslopeCliError, HillslopeRunRequest, HillslopeRuntimeSelection};

use super::{
    DirectProductionRunFrameBuildInputs, DirectProductionSeedAuthority,
    HillslopeClimateExecutionState, build_direct_production_run_frame,
    build_hillslope_climate_runtime_request, build_static_hillslope_runtime_setup,
    direct_groundwater_authority_from_gwcoeff, direct_production_runtime_error,
    load_hillslope_run_inputs, resolve_hillslope_output_targets, resolve_hillslope_sidecars,
};

const LEGACY_SEED_FILE: &str = "snow_stage3_v11_owner_seed.json";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(test, allow(unreachable_pub))]
pub enum Stage3TestFixtureSeedProfile {
    CompleteOwner,
    AdaptiveNoStrataOwner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(test, allow(dead_code, unreachable_pub))]
pub enum Stage3TestFixtureSeedBinding {
    ExplicitRunfile,
    LegacyDiscovery,
}

/// Author and bind an exact owner seed for a prepared integration fixture.
///
/// This API exists only under the non-default `test-fixture-authority` feature.
/// It never participates in execution or missing-input admission: callers must
/// write the returned artifact through one of the same boundaries production
/// resolves before invoking the runner.
#[cfg_attr(test, allow(unreachable_pub))]
pub fn author_stage3_v11_owner_seed_fixture(
    request: &HillslopeRunRequest,
    profile: Stage3TestFixtureSeedProfile,
    binding: Stage3TestFixtureSeedBinding,
) -> Result<PathBuf, HillslopeCliError> {
    let inputs = load_hillslope_run_inputs(request)?;
    let targets = resolve_hillslope_output_targets(&inputs.runfile)?;
    let sidecars = resolve_hillslope_sidecars(request, &inputs, &targets)?;
    let setup = build_static_hillslope_runtime_setup(
        request,
        &inputs,
        &sidecars,
        HillslopeRuntimeSelection::DirectProductionExecutor,
    )?;
    let HillslopeClimateExecutionState {
        per_ofe_lane_areas_m2,
        per_ofe_runoff_publication_geometries,
        lane_context,
        climate_span,
    } = setup.execution_state;
    let climate_request =
        build_hillslope_climate_runtime_request(&inputs.climate).map_err(|error| {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "test_fixture_authority",
                detail: error.to_string(),
            }
        })?;
    let seed_authority = DirectProductionSeedAuthority::from_typed_inputs(
        &climate_request,
        &inputs,
        &sidecars,
        per_ofe_lane_areas_m2.len(),
        lane_context.lane,
    )?;
    let mut frame = build_direct_production_run_frame(&DirectProductionRunFrameBuildInputs {
        output_hillslope_id: targets.output_hillslope_id,
        lane_areas_m2: &per_ofe_lane_areas_m2,
        runoff_publication_geometries: &per_ofe_runoff_publication_geometries,
        day_count: climate_span.days.len(),
        seed_authority: &seed_authority,
    })?;
    frame
        .configure_groundwater(direct_groundwater_authority_from_gwcoeff(
            &sidecars.gwcoeff,
        )?)
        .map_err(|source| direct_production_runtime_error(&source))?;

    let bytes = super::snow_stage3_v11_production_seed::author_explicit_test_seed_bytes(
        &frame,
        profile == Stage3TestFixtureSeedProfile::AdaptiveNoStrataOwner,
        inputs.climate.metadata.deglat,
    )?;
    let seed_file = match binding {
        Stage3TestFixtureSeedBinding::ExplicitRunfile => format!(
            "{}.snow_stage3_v11_owner_seed.json",
            request
                .run_file
                .file_name()
                .and_then(std::ffi::OsStr::to_str)
                .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "test_fixture_authority",
                    detail: "test runfile name is not valid UTF-8".to_owned(),
                })?
        ),
        Stage3TestFixtureSeedBinding::LegacyDiscovery => LEGACY_SEED_FILE.to_owned(),
    };
    let seed_path = request.run_dir.join(&seed_file);
    fs::write(&seed_path, bytes).map_err(|source| HillslopeCliError::Io {
        path: seed_path.clone(),
        source,
    })?;

    match binding {
        Stage3TestFixtureSeedBinding::ExplicitRunfile => {
            bind_seed_in_runfile(&inputs.run_file_path, &seed_file)?;
        }
        Stage3TestFixtureSeedBinding::LegacyDiscovery => {
            if !request.legacy_sidecar_discovery {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "test_fixture_authority",
                    detail: "legacy seed binding requires legacy_sidecar_discovery=true".to_owned(),
                });
            }
        }
    }

    Ok(seed_path)
}

fn bind_seed_in_runfile(
    run_file_path: &std::path::Path,
    seed_file: &str,
) -> Result<(), HillslopeCliError> {
    let source = fs::read_to_string(run_file_path).map_err(|source| HillslopeCliError::Io {
        path: run_file_path.to_path_buf(),
        source,
    })?;
    let mut runfile: toml::Value =
        toml::from_str(&source).map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "test_fixture_authority",
            detail: format!("test runfile cannot bind explicit Stage-3 owner seed: {error}"),
        })?;
    let inputs = runfile
        .get_mut("inputs")
        .and_then(toml::Value::as_table_mut)
        .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "test_fixture_authority",
            detail: "test runfile omits the [inputs] table".to_owned(),
        })?;
    inputs.insert(
        "snow_stage3_v11_owner_seed".to_owned(),
        toml::Value::String(seed_file.to_owned()),
    );
    let rendered =
        toml::to_string(&runfile).map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "test_fixture_authority",
            detail: format!("test runfile seed binding cannot serialize: {error}"),
        })?;
    fs::write(run_file_path, rendered).map_err(|source| HillslopeCliError::Io {
        path: run_file_path.to_path_buf(),
        source,
    })
}
