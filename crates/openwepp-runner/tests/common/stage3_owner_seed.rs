use std::fs;
use std::path::Path;
use std::thread;

const SEED_FILE: &str = "snow_stage3_v11_owner_seed.json";
const TWO_DAY_SEED: &[u8] = include_bytes!("../fixtures/snow_stage3_v11_owner_seed_two_day.json");

#[test]
fn checked_seed_and_explicit_runfile_binding_are_current() {
    let artifact: serde_json::Value =
        serde_json::from_slice(TWO_DAY_SEED).expect("checked Stage-3 owner seed should parse");
    assert_eq!(
        artifact.get("schema").and_then(serde_json::Value::as_str),
        Some("OPENWEPP_SNOW_STAGE3_V11_PRODUCTION_SEED_V1")
    );
    assert_eq!(
        artifact.get("version").and_then(serde_json::Value::as_u64),
        Some(1)
    );
    assert_eq!(
        artifact
            .pointer("/checkpoint/schema")
            .and_then(serde_json::Value::as_str),
        Some("OPENWEPP_DIRECT_V10_REAL_CONSUMER_CHECKPOINT_V1")
    );

    let runfile = render_runfile("[inputs]\nmanagement = \"case.man\"\n");
    assert!(runfile.contains("snow_stage3_v11_owner_seed = \"snow_stage3_v11_owner_seed.json\""));
    assert!(runfile.contains("management = \"case.man.yaml\""));
}

/// Install the repository-sealed Stage-3/V11 owner authority and the exact
/// native input topology that it authenticates. This is integration-fixture
/// construction, not a production default or missing-input fallback.
pub(crate) fn install(run_dir: &Path, runfile_payload: &str) -> String {
    install_matching_native_inputs(run_dir);
    fs::write(run_dir.join(SEED_FILE), TWO_DAY_SEED).expect("write explicit Stage-3 owner seed");

    render_runfile(runfile_payload)
}

fn render_runfile(runfile_payload: &str) -> String {
    runfile_payload
        .replace("H5.", "H83.")
        .replace(
            "management = \"case.man\"",
            "management = \"case.man.yaml\"",
        )
        .replace(
            "[inputs]\n",
            "[inputs]\nsnow_stage3_v11_owner_seed = \"snow_stage3_v11_owner_seed.json\"\n",
        )
}

pub(crate) fn with_large_stack<T: Send>(run: impl FnOnce() -> T + Send) -> T {
    thread::scope(|scope| {
        thread::Builder::new()
            .name("stage3-owner-seed-fixture".to_owned())
            .stack_size(64 * 1024 * 1024)
            .spawn_scoped(scope, run)
            .expect("spawn Stage-3 integration fixture thread")
            .join()
            .expect("Stage-3 integration fixture thread should not panic")
    })
}

fn install_matching_native_inputs(run_dir: &Path) {
    let native_fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/cancov_forest/marcell_conifer_mn");
    fs::copy(
        native_fixture.join("p8.man.yaml"),
        run_dir.join("case.man.yaml"),
    )
    .expect("install native forest management authority");
    fs::copy(
        native_fixture.join("pmetpara.txt"),
        run_dir.join("pmetpara.txt"),
    )
    .expect("install native forest PMET authority");

    replace_file(run_dir.join("case.slp"), |source| {
        source
            .replace("180.0 30.0", "180.0 10.0")
            .replace("3 60.0", "3 10.0")
    });
    replace_file(run_dir.join("case.cli"), |source| {
        source.replace("45.0 -120.0", "41.1 -120.0")
    });
    replace_file(run_dir.join("case.sol"), |source| {
        source.replace("CLAY_LOAM 2 ", "CLAY_LOAM 5 ").replace(
            "250 1.30 8.0 1.10 0.28 0.14 33 27 1.8 14 7 0.06 0.43 0.03 1.35 110 0.15 0.30\n",
            concat!(
                "250 1.30 8.0 1.10 0.28 0.14 33 27 1.8 14 7 0.06 0.43 0.03 1.35 110 0.15 0.30\n",
                "500 1.30 8.0 1.10 0.28 0.14 33 27 1.8 14 7 0.06 0.43 0.03 1.35 110 0.15 0.30\n",
                "750 1.30 8.0 1.10 0.28 0.14 33 27 1.8 14 7 0.06 0.43 0.03 1.35 110 0.15 0.30\n",
                "1000 1.30 8.0 1.10 0.28 0.14 33 27 1.8 14 7 0.06 0.43 0.03 1.35 110 0.15 0.30\n",
            ),
        )
    });
}

fn replace_file(path: impl AsRef<Path>, replace: impl FnOnce(String) -> String) {
    let path = path.as_ref();
    let source = fs::read_to_string(path).expect("read explicit Stage-3 fixture input");
    fs::write(path, replace(source)).expect("write explicit Stage-3 fixture input");
}
