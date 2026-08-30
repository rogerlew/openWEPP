#![cfg(feature = "test-fixture-authority")]

const MANIFEST: &str = include_str!("../Cargo.toml");
const HILLSLOPE_MODULE: &str = include_str!("../src/hillslope/mod.rs");
const RUNNER_EXECUTION: &str = include_str!("../src/hillslope/05_runner_execution_and_outputs.rs");
const PRODUCTION_SEED: &str = include_str!("../src/hillslope/snow_stage3_v11_production_seed.rs");

use std::path::{Path, PathBuf};

use openwepp_runner::{
    HillslopeRunRequest, SidecarPolicy, Stage3TestFixtureSeedBinding, Stage3TestFixtureSeedProfile,
    author_stage3_v11_owner_seed_fixture,
};

#[test]
fn test_fixture_authority_is_nondefault_and_cannot_admit_a_missing_seed() {
    assert!(MANIFEST.contains("default = []"));
    assert!(
        MANIFEST.contains("test-fixture-authority = [\"openwepp-persisted-restart-v1/fixtures\"]")
    );
    assert!(HILLSLOPE_MODULE.contains("#[cfg(any(test, feature = \"test-fixture-authority\"))]"));
    assert!(HILLSLOPE_MODULE.contains("mod test_fixture_authority;"));

    let production_branch = RUNNER_EXECUTION
        .split("#[cfg(not(test))]")
        .nth(1)
        .expect("runner retains a distinct non-test seed admission branch")
        .split("#[cfg(test)]")
        .next()
        .expect("production seed branch precedes the test-only branch");
    assert!(production_branch.contains("DirectSnowStage3V11ProductionSeedV1::load_required"));
    assert!(!production_branch.contains("load_required_or_explicit_test"));
    assert!(PRODUCTION_SEED.contains("no fixture/default owner seed is admitted"));
}

#[test]
fn exact_live_frame_authority_binds_single_and_two_lane_fixture_artifacts() {
    for (fixture, run_file, profile, expected_roots) in [
        (
            "cli01/hillslope_run_dir",
            "case.run",
            Stage3TestFixtureSeedProfile::AdaptiveNoStrataOwner,
            &[][..],
        ),
        (
            "cli01/hillslope_run_dir",
            "case.run",
            Stage3TestFixtureSeedProfile::CompleteOwner,
            &["thermal-1", "thermal-2"][..],
        ),
        (
            "watershed/p102-sediment-active/runs",
            "H1.source.run",
            Stage3TestFixtureSeedProfile::AdaptiveNoStrataOwner,
            &[][..],
        ),
    ] {
        let source = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures")
            .join(fixture);
        let run_dir = std::env::temp_dir().join(format!(
            "openwepp-stage3-fixture-authority-{}-{}",
            std::process::id(),
            run_file
        ));
        let _ = std::fs::remove_dir_all(&run_dir);
        copy_fixture(&source, &run_dir);
        let request = HillslopeRunRequest {
            run_dir: run_dir.clone(),
            run_file: PathBuf::from(run_file),
            output_dir: run_dir.join("output"),
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: false,
            manifest_path: None,
        };
        let seed_path = author_stage3_v11_owner_seed_fixture(
            &request,
            profile,
            Stage3TestFixtureSeedBinding::ExplicitRunfile,
        )
        .expect("prepared live fixture should author exact owner authority");
        assert!(seed_path.is_file());
        assert!(std::fs::metadata(&seed_path).expect("seed metadata").len() > 0);
        let seed: serde_json::Value = serde_json::from_slice(
            &std::fs::read(&seed_path).expect("read exact owner-seed fixture"),
        )
        .expect("owner-seed fixture JSON");
        let strata = seed["vegetation_configuration"]["strata"]
            .as_array()
            .expect("vegetation strata array");
        if expected_roots.is_empty() {
            assert!(strata.is_empty(), "adaptive fixture must remain no-strata");
        } else {
            for stratum in strata {
                let roots = stratum["root_layers"].as_array().expect("root layer array");
                assert_eq!(
                    roots
                        .iter()
                        .map(|root| root["layer_id"].as_str().expect("root layer identity"))
                        .collect::<Vec<_>>(),
                    expected_roots
                );
                assert_eq!(
                    roots
                        .iter()
                        .map(|root| root["root_fraction"].as_f64().expect("root fraction"))
                        .collect::<Vec<_>>(),
                    [0.62, 0.38]
                        .into_iter()
                        .chain(std::iter::repeat_n(
                            0.0,
                            expected_roots.len().saturating_sub(2),
                        ))
                        .collect::<Vec<_>>(),
                    "root authority must preserve the checked nonzero distribution exactly"
                );
            }
        }
        let runfile = std::fs::read_to_string(run_dir.join(run_file)).expect("bound runfile");
        let seed_name = seed_path
            .file_name()
            .and_then(std::ffi::OsStr::to_str)
            .expect("UTF-8 seed name");
        assert!(runfile.contains("snow_stage3_v11_owner_seed"));
        assert!(runfile.contains(seed_name));
        std::fs::remove_dir_all(run_dir).expect("remove fixture authority directory");
    }
}

fn copy_fixture(source: &Path, destination: &Path) {
    std::fs::create_dir_all(destination).expect("fixture destination");
    for entry in std::fs::read_dir(source).expect("fixture directory") {
        let entry = entry.expect("fixture entry");
        let target = destination.join(entry.file_name());
        if entry.path().is_dir() {
            copy_fixture(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), target).expect("copy fixture file");
        }
    }
}
