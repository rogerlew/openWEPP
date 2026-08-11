use std::fs;
use std::path::PathBuf;

fn read(path: &str) -> String {
    fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path))
        .expect("source must be readable")
}

fn rust_files_below(path: &str) -> Vec<PathBuf> {
    fn collect(directory: &std::path::Path, files: &mut Vec<PathBuf>) {
        for entry in fs::read_dir(directory).expect("production directory must be readable") {
            let path = entry.expect("production entry").path();
            if path.is_dir() {
                collect(&path, files);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                files.push(path);
            }
        }
    }

    let mut files = Vec::new();
    collect(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path),
        &mut files,
    );
    files
}

#[test]
fn wat5_observation_is_not_an_hbp_input() {
    let publication = read("crates/openwepp-runner/src/hillslope/04_direct_publication.rs");
    let wat5_start = publication
        .find("fn observe_subhourly_generation")
        .expect("WAT5 observer");
    let wat5_end = publication[wat5_start..]
        .find("\n    fn ")
        .map_or(publication.len(), |offset| wat5_start + offset);
    let wat5_observer = &publication[wat5_start..wat5_end];
    assert!(!wat5_observer.contains("hbp_"));

    let hbp_start = publication
        .find("fn build_hbp_output_from_direct_publication_summary")
        .expect("HBP builder");
    let hbp_end = publication[hbp_start..]
        .find("\nfn assemble_hbp_event_sediment_surfaces")
        .map_or(publication.len(), |offset| hbp_start + offset);
    let hbp_builder = &publication[hbp_start..hbp_end];
    assert!(!hbp_builder.contains("wat5"));
    assert!(!hbp_builder.contains("wat_subhourly"));
}

#[test]
fn runtime_executes_wat5_after_peak_and_erosion_authority() {
    let executor = read("crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs");
    let peak = executor
        .find("run_r7d6_peak_runoff_span")
        .expect("peak authority span");
    let wat5 = executor
        .find("run_wat5_subhourly_generation")
        .expect("WAT5 diagnostic span");
    assert!(peak < wat5, "WAT5 must not feed peak or erosion execution");
}

#[test]
fn watershed_channel_and_ofe_routing_do_not_read_wat5() {
    let mut paths = rust_files_below("crates/openwepp-hillslope-orchestrator/src/ofe_routing");
    paths.extend(rust_files_below(
        "crates/openwepp-watershed-orchestrator/src",
    ));
    paths.extend(rust_files_below("crates/openwepp-watershed-output/src"));
    paths.extend([
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("crates/openwepp-runner/src/watershed_supervisor.rs"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("crates/openwepp-runner/src/watershed_wat.rs"),
    ]);
    for path in paths {
        let source = fs::read_to_string(&path).expect("routing source must be readable");
        for forbidden in ["wat_subhourly", "DirectFiveMinuteGeneration", "WAT5"] {
            assert!(
                !source.contains(forbidden),
                "routing source {} reads forbidden WAT5 marker {forbidden}",
                path.display()
            );
        }
    }
}
