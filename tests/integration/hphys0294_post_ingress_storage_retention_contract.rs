use std::fs;
use std::path::Path;

const SC_PERC: &str = "docs/specifications/science-contracts/contracts/SC-PERC-001.md";
const SC_WATBAL: &str = "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md";
const SC_SNOWFREEZE: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const SC_RUNOFFPART: &str = "docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md";
const KERNEL_SOURCE: &str =
    "crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs";

fn read_runner_hillslope_sources() -> String {
    let runner_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("crates/openwepp-runner/src/hillslope");
    let mut files: Vec<_> = fs::read_dir(&runner_dir)
        .expect("runner hillslope source directory should be readable")
        .map(|entry| {
            entry
                .expect("runner hillslope source entry should be readable")
                .path()
        })
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("rs"))
        .collect();
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
fn hphys0294_contracts_define_post_ingress_attribution_authority() {
    let perc = fs::read_to_string(SC_PERC).expect("percolation contract should be readable");
    let watbal = fs::read_to_string(SC_WATBAL).expect("water-balance contract should be readable");
    let snow = fs::read_to_string(SC_SNOWFREEZE).expect("snow contract should be readable");
    let runoff = fs::read_to_string(SC_RUNOFFPART).expect("runoff contract should be readable");

    assert!(
        perc.contains("INV-PERC-019")
            && perc.contains("post-ingress storage/percolation/lateral retention invariant")
            && perc.contains("WB19 lateral potential/target/realized/unrealized"),
        "SC-PERC must define HPHYS0294 WB18/WB19 attribution authority"
    );
    assert!(
        watbal.contains("INV-WATBAL-069")
            && watbal.contains("post-ingress storage/percolation/lateral attribution invariant")
            && watbal.contains("Mixed residual direction across H1/H7/H39"),
        "SC-WATBAL must define HPHYS0294 storage attribution authority"
    );
    assert!(
        snow.contains("INV-SNOWFREEZE-026") && runoff.contains("INV-RUNOFFPART-023"),
        "HPHYS0294 must remain anchored to HPHYS0293 snow exclusion and runoff exclusion authority"
    );
}

#[test]
fn hphys0294_runner_trace_preserves_storage_percolation_lateral_masks() {
    let runner = read_runner_hillslope_sources();

    for required_field in [
        "wb11_soil_water_m",
        "wb18_theta_sum_m",
        "wb18_recomputed_soil_water_m",
        "wb18_recomputed_minus_wb11_m",
        "wb11_minus_theta_sum_m",
        "wb18_pei_sum_m",
        "d_m",
        "pe_m",
        "wb13_dp_mm",
        "wb13_total_soil_mm",
        "wb13_soil_water_total_mm",
        "wb19_q_lateral_potential_m",
        "wb19_q_lateral_target_m",
        "wb19_lateral_capacity_tdv_m",
        "wb19_q_lateral_unrealized_m",
        "wb19_lateral_withdrawal_layers_m",
        "snow_runtime_swe_delta_m",
        "snow_routed_melt_m",
        "wb13_rm_mm",
        "wb13_snow_water_mm",
        "wb13_q_mm",
    ] {
        assert!(
            runner.contains(required_field),
            "runner trace must preserve HPHYS0294 attribution field {required_field}"
        );
    }
}

#[test]
fn hphys0294_kernel_preserves_wb18_identity_and_wb19_lateral_lineage() {
    let kernel = fs::read_to_string(KERNEL_SOURCE).expect("kernel source should be readable");

    assert!(
        kernel.contains("wb18_aggregate_soil_water_after_percolation")
            && kernel.contains("layer_soil_water = *layer_theta + thetdr * (dg - frozen_depth)")
            && kernel.contains("WB11_SYMBOL_PERC_LOSS_D")
            && kernel.contains("WB11_SYMBOL_PERC_RECHARGE_PE"),
        "WB18 must preserve aggregate watcon identity and D=Pe boundary publication"
    );
    for lateral_symbol in [
        "WB19_SYMBOL_LATERAL_POTENTIAL",
        "WB19_SYMBOL_LATERAL_TARGET",
        "WB19_SYMBOL_LATERAL_CAPACITY_TDV",
        "WB19_SYMBOL_LATERAL_UNREALIZED",
        "WB19_SYMBOL_LATERAL_WITHDRAWAL_ROOT",
    ] {
        assert!(
            kernel.contains(lateral_symbol),
            "WB19 lateral diagnostics must preserve {lateral_symbol}"
        );
    }
}
