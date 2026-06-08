use std::fs;
use std::path::{Path, PathBuf};

const SC_WATBAL: &str = "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md";
const SC_EVAP: &str = "docs/specifications/science-contracts/contracts/SC-EVAP-001.md";
const SC_PERC: &str = "docs/specifications/science-contracts/contracts/SC-PERC-001.md";
const SC_SUBHYD: &str = "docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md";

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
fn hphys0295_contracts_define_cumulative_storage_budget_authority() {
    let watbal = fs::read_to_string(SC_WATBAL).expect("watbal contract should be readable");
    let evap = fs::read_to_string(SC_EVAP).expect("evap contract should be readable");
    let perc = fs::read_to_string(SC_PERC).expect("perc contract should be readable");
    let subhyd = fs::read_to_string(SC_SUBHYD).expect("subhyd contract should be readable");

    assert!(
        watbal.contains("INV-WATBAL-070")
            && watbal.contains("cumulative storage-budget ownership invariant")
            && watbal.contains("Production edits to WB17, WB18, WB19, or WB13 are invalid"),
        "SC-WATBAL must define HPHYS0295 cumulative storage-budget ownership authority"
    );
    assert!(
        evap.contains("INV-EVAP-027")
            && evap.contains("cumulative storage-budget ET ownership invariant")
            && evap.contains("A same-day `Ep`/`Es` residual is not sufficient"),
        "SC-EVAP must define HPHYS0295 ET ownership gate"
    );
    assert!(
        perc.contains("INV-PERC-019") && subhyd.contains("INV-SUBHYD-031"),
        "HPHYS0295 must remain anchored to post-ingress percolation and lateral authority"
    );
}

#[test]
fn hphys0295_runner_trace_preserves_cumulative_budget_surfaces() {
    let runner = read_runner_hillslope_sources();

    for required_field in [
        "wb13_total_soil_mm",
        "wb13_soil_water_total_mm",
        "wb13_dp_mm",
        "wb13_q_mm",
        "wb13_rm_mm",
        "wb13_snow_water_mm",
        "ep_m",
        "etp_m",
        "ui_m",
        "wb17_ui_layers_m",
        "ws",
        "pmet_es_m",
        "pmet_ep_m",
        "d_m",
        "pe_m",
        "wb18_recomputed_minus_wb11_m",
        "wb18_pei_sum_m",
        "wb19_q_lateral_target_m",
        "wb19_q_lateral_unrealized_m",
        "wb19_lateral_withdrawal_layers_m",
        "q_m",
        "qdd_m",
        "qd_m",
        "snow_runtime_swe_delta_m",
        "snow_routed_melt_m",
        "snow_post_winter_rain_m",
    ] {
        assert!(
            runner.contains(required_field),
            "runner trace must preserve HPHYS0295 budget surface {required_field}"
        );
    }
}

#[test]
fn hphys0295_contract_forbids_same_day_delta_compensation() {
    let watbal = fs::read_to_string(SC_WATBAL).expect("watbal contract should be readable");
    let evap = fs::read_to_string(SC_EVAP).expect("evap contract should be readable");

    assert!(
        watbal.contains("candidate/baseline storage deltas, `Ep`, `Es`, `Er`, `D`, `latqcc`")
            && watbal.contains("excluded snow-producer masks"),
        "WATBAL budget must require candidate/baseline deltas and snow-mask separation"
    );
    assert!(
        evap.contains("same-day `Ep`/`Es` residual is not sufficient")
            && evap.contains("cumulative storage-accounting proves ET extraction magnitude"),
        "EVAP budget must reject same-day ET compensation without cumulative proof"
    );
}
