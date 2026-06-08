use std::fs;
use std::path::{Path, PathBuf};

const KERNEL_PHASE_SOURCE: &str =
    "crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs";

fn collect_runner_source_files(dir: &Path, files: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).expect("runner hillslope source entry should be readable") {
        let entry = entry.expect("runner hillslope source entry should be readable");
        let path = entry.path();
        if path.is_dir() {
            collect_runner_source_files(&path, files);
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
            files.push(path);
        }
    }
}

fn read_runner_hillslope_sources() -> String {
    let runner_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("crates/openwepp-runner/src/hillslope");
    let mut files: Vec<PathBuf> = Vec::new();

    collect_runner_source_files(&runner_dir, &mut files);
    files.sort();

    files
        .into_iter()
        .map(|path| {
            fs::read_to_string(&path).unwrap_or_else(|error| {
                panic!(
                    "runner source {} should be readable: {error}",
                    path.display()
                )
            })
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn hphys0289_contract_requires_wb13_rm_from_routed_wmelt_not_swe_delta_proxy() {
    let runner = read_runner_hillslope_sources();

    assert!(
        runner.contains(
            "require_runtime_flux_surface_scalar(runtime_surface, \"snow.routed_melt_m\")"
        ),
        "WB13 publication must consume an explicit routed wmelt producer flux"
    );
    assert!(
        !runner.contains("precipitation_m + runtime_swe_before_m - runtime_swe_m + irrigation_m"),
        "WB13 RM must not use raw precipitation plus SWE-delta proxy math"
    );
}

#[test]
fn hphys0289_contract_requires_kernel_to_publish_daily_routed_wmelt() {
    let kernel =
        fs::read_to_string(KERNEL_PHASE_SOURCE).expect("kernel phase source should be readable");

    assert!(
        kernel.contains("BoundarySymbol::from(\"snow.routed_melt_m\")"),
        "WB12/WB14 runoff reconciliation must publish daily routed wmelt for WB13"
    );
}
