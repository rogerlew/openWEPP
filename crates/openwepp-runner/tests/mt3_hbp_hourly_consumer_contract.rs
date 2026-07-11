#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::too_many_arguments,
    clippy::unreadable_literal
)]

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::Command;

use openwepp_input_contract::parsers::chaninp::{
    ChaninpParseOptions, ChaninpParseOutcome, parse_chaninp_from_path,
};
use openwepp_input_contract::parsers::hbp::{
    HbpParseOptions, parse_hbp_from_path_with_latest_event_payload,
};
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::{Row, RowAccessor};

const MAGIC: &[u8; 8] = b"WFPHBP01";
const FOOTER_MAGIC: &[u8; 8] = b"ENDHBP01";
const SUPPORTED_MAJOR_V1: u16 = 1;
const SUPPORTED_MINOR_V1: u16 = 1;
const DIM_SCALAR: u8 = 0;
const DIM_NOFE: u8 = 1;
const DIM_NOFE_LAYERS: u8 = 2;
const SCALE_INV_I64: f64 = 1.0e9;

const REQUIRED_STATE_IDS: &[u16] = &[
    1, 2, 3, 4, 5, 6, 7, 100, 101, 102, 103, 104, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209,
    210, 300, 900, 901,
];

#[test]
fn mt3_watershed_cli_hbp_hourly_pair_reaches_channel_consumer() {
    let mut spike_runoff = [0.0_f64; 24];
    spike_runoff[10] = 7_200.0;
    let mut spike_sediment = [0.0_f64; 24];
    spike_sediment[10] = 240.0;

    let mut spread_runoff = [0.0_f64; 24];
    let mut spread_sediment = [0.0_f64; 24];
    for hour in 8..12 {
        spread_runoff[hour] = 1_800.0;
        spread_sediment[hour] = 60.0;
    }

    let spike = run_hourly_fixture("mt3_cli_spike", spike_runoff, spike_sediment);
    let spread = run_hourly_fixture("mt3_cli_spread", spread_runoff, spread_sediment);

    assert_relative_close(
        spike.hbp_hourly_runoff_sum_m3,
        spread.hbp_hourly_runoff_sum_m3,
        1.0e-9,
        "HBP hourly runoff total",
    );
    assert_relative_close(
        spike.hbp_hourly_sediment_sum_kg,
        spread.hbp_hourly_sediment_sum_kg,
        1.0e-9,
        "HBP hourly sediment total",
    );
    assert!(spike.ebe_runoff_volume_m3 > 0.0);
    assert!(spread.ebe_runoff_volume_m3 > 0.0);
    assert_eq!(spike.ebe_element_id, 1);
    assert_eq!(spread.ebe_element_id, 1);
    assert!(
        (spike.ebe_peak_runoff_m3_s - spread.ebe_peak_runoff_m3_s).abs() > 1.0e-9,
        "identical scalar HBP fields must not erase hourly water timing"
    );
    assert_relative_close(
        spike.ebe_sediment_yield_kg,
        spike.hbp_hourly_sediment_sum_kg,
        1.0e-9,
        "single-channel spike sediment closure",
    );
    assert_relative_close(
        spread.ebe_sediment_yield_kg,
        spread.hbp_hourly_sediment_sum_kg,
        1.0e-9,
        "single-channel spread sediment closure",
    );
}

#[test]
fn wshedw11d_cli_accepts_three_record_zero_count_chaninp_without_defaulting() {
    let mut runoff = [0.0_f64; 24];
    runoff[6] = 7_200.0;
    let mut sediment = [0.0_f64; 24];
    sediment[6] = 240.0;
    let run_grid = |prefix: &str, chaninp: &str| {
        let run_dir = build_watershed_fixture_dir(prefix);
        let channel_controls = fs::read_to_string(run_dir.join("pw0.chn"))
            .expect("channel controls should be readable")
            .replacen("\n4\n", "\n3\n", 1);
        fs::write(run_dir.join("pw0.chn"), channel_controls)
            .expect("KW channel controls should be writable");
        fs::write(run_dir.join("chan.inp"), chaninp)
            .expect("chan.inp grid control should be writable");
        write_hourly_hbp_fixture(
            run_dir.join("H1.hbp"),
            1,
            2.0,
            3_600.0,
            240.0,
            0.0,
            runoff,
            sediment,
        );
        write_watershed_runfile(&run_dir, &[1]);
        let output_dir = run_dir.join("out");
        let output = run_watershed_cli(&run_dir, &output_dir);
        assert!(
            output.status.success(),
            "CLI should accept {prefix} chan.inp: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let ebe_row = read_first_parquet_row(&output_dir.join("interchange/ebe_pw0.parquet"));
        let channel_row = read_first_parquet_row(&output_dir.join("interchange/chanwb.parquet"));
        (
            run_dir,
            [
                row_f64_value(&ebe_row, "peak_runoff"),
                row_f64_value(&ebe_row, "runoff_volume"),
                row_f64_value(&channel_row, "Storage (m^3)"),
                row_f64_value(&channel_row, "Balance (m^3)"),
            ],
        )
    };

    let (run_dir, zero_count_600) = run_grid("w11d_chaninp_zero_600", "3 600\n0.0\n0\n");

    let parsed = parse_chaninp_from_path(
        run_dir.join("chan.inp"),
        ChaninpParseOptions::compatibility(3, 1),
        &BTreeSet::from([2]),
    )
    .expect("canonical zero-count sidecar should parse");
    assert_eq!(parsed.parse_outcome, ChaninpParseOutcome::ParsedBranch);
    assert!(parsed.warnings.is_empty());
    let options = parsed.options.expect("wave sidecar should expose options");
    assert_eq!(options.dtchr_norm_s, 600);
    assert_eq!(options.ntchr, 144);
    assert_eq!(options.nchnum_norm, 0);
    assert!(options.ichnum_norm.is_empty());
    assert!(!options.chan_output_enabled);

    let (_, positive_count_600) = run_grid("w11d_chaninp_positive_600", "3 600\n0.0\n1\n2\n");
    let (_, positive_count_60) = run_grid("w11d_chaninp_positive_60", "3 60\n0.0\n1\n2\n");
    for (field, zero, control) in [
        ("peak", zero_count_600[0], positive_count_600[0]),
        ("volume", zero_count_600[1], positive_count_600[1]),
        ("storage", zero_count_600[2], positive_count_600[2]),
        ("balance", zero_count_600[3], positive_count_600[3]),
    ] {
        assert_relative_close(zero, control, 1.0e-12, field);
    }
    assert!(
        (zero_count_600[0] - positive_count_60[0]).abs() > 1.0e-6
            || (zero_count_600[2] - positive_count_60[2]).abs() > 1.0e-6,
        "parsed zero-count dtchr=600 must not alias the 60-second compatibility default"
    );
}

#[test]
fn wshedw11b_two_channel_cli_consumes_same_grid_sediment_egress() {
    let mut spike_runoff = [0.0_f64; 24];
    spike_runoff[23] = 7_200.0;
    let mut spike_sediment = [0.0_f64; 24];
    spike_sediment[23] = 240.0;

    let mut spread_runoff = [0.0_f64; 24];
    let mut spread_sediment = [0.0_f64; 24];
    for hour in 20..24 {
        spread_runoff[hour] = 1_800.0;
        spread_sediment[hour] = 60.0;
    }

    let spike = run_two_channel_hourly_fixture("w11b_cli_spike", spike_runoff, spike_sediment);
    let spread = run_two_channel_hourly_fixture("w11b_cli_spread", spread_runoff, spread_sediment);
    assert_relative_close(
        spike.hbp_hourly_runoff_sum_m3,
        spread.hbp_hourly_runoff_sum_m3,
        1.0e-9,
        "two-channel HBP hourly runoff total",
    );
    assert_relative_close(
        spike.hbp_hourly_sediment_sum_kg,
        spread.hbp_hourly_sediment_sum_kg,
        1.0e-9,
        "two-channel HBP hourly sediment total",
    );
    assert!(spike.ebe_runoff_volume_m3 > 0.0);
    assert!(spread.ebe_runoff_volume_m3 > 0.0);
    assert!(spike.ebe_sediment_yield_kg > 0.0);
    assert!(spread.ebe_sediment_yield_kg > 0.0);
    assert_eq!(spike.ebe_element_id, 2);
    assert_eq!(spread.ebe_element_id, 2);
    for (label, output) in [("spike", &spike), ("spread", &spread)] {
        assert!(
            output.ebe_runoff_volume_m3 <= output.hbp_hourly_runoff_sum_m3 + 1.0e-9,
            "{label} terminal outflow cannot exceed external hourly runoff"
        );
        assert!(
            output.channel_storage_m3 >= 0.0,
            "{label} first-day residual channel storage must be non-negative"
        );
        assert_relative_close(
            output.ebe_runoff_volume_m3 + output.channel_storage_m3,
            output.hbp_hourly_runoff_sum_m3,
            1.0e-9,
            "{label} serialized-input routed water ledger",
        );
        assert_relative_close(
            output.channel_balance_m3,
            0.0,
            1.0e-9,
            "{label} published channel water balance",
        );
    }
    assert!(
        (spike.ebe_peak_runoff_m3_s - spread.ebe_peak_runoff_m3_s).abs() > 1.0e-9,
        "downstream peak must retain upstream timing sensitivity"
    );
    assert_relative_close(
        spike.ebe_sediment_yield_kg,
        spike.hbp_hourly_sediment_sum_kg,
        1.0e-9,
        "spike downstream sediment closure",
    );
    assert_relative_close(
        spread.ebe_sediment_yield_kg,
        spread.hbp_hourly_sediment_sum_kg,
        1.0e-9,
        "spread downstream sediment closure",
    );
}

#[test]
fn wshedw11d_creams_serial_publication_uses_terminal_extensive_outputs() {
    let scenario = sanity_scenarios()
        .into_iter()
        .find(|scenario| scenario.name == "early_spike")
        .expect("early-spike contract vector");
    let output =
        run_two_channel_sanity_fixture("w11d_creams_terminal_publication", &scenario, 2, 3_600);

    assert_eq!(
        output.ebe_element_id, 2,
        "channel 2 is the topology terminal"
    );
    assert_scaled_close(
        output.ebe_runoff_volume_m3,
        output.hbp_hourly_runoff_sum_m3,
        1.0e-9,
        "terminal CREAMS outlet volume",
    );
    assert_scaled_close(
        output.ebe_sediment_yield_kg,
        output.hbp_hourly_sediment_sum_kg,
        1.0e-9,
        "terminal CREAMS sediment mass",
    );
}

#[test]
fn wshedw11d_release_cli_rejects_inadmissible_mc_grids_typed() {
    for (ipeak, dtchr_seconds) in [(4, 3_600), (4, 600), (5, 3_600), (5, 600)] {
        for scenario in sanity_scenarios() {
            let prefix = format!("w11d_mc_i{ipeak}_dt{dtchr_seconds}_{}", scenario.name);
            let result = try_run_two_channel_fixture(
                &prefix,
                &scenario,
                ipeak,
                dtchr_seconds,
                scenario.input_peak_m3_s(),
                scenario.scalar_duration_seconds(),
                scenario.hourly_sediment_mass_kg.iter().sum(),
            );
            if scenario.name == "zero" {
                let output = result.expect("zero MC control executes no unstable recurrence");
                assert!(output.ebe_runoff_volume_m3.abs() <= 1.0e-12);
                assert!(output.ebe_peak_runoff_m3_s.abs() <= 1.0e-12);
                continue;
            }
            let error = result.expect_err("active W11C MC grids must fail before publication");
            assert!(
                error.contains("WKERNEL-WS10-CHANNEL-E-003"),
                "MC grid rejection must preserve typed guard identity: {error}"
            );
        }
    }
}

#[test]
fn wshedw11d_release_cli_accepts_admissible_static_and_dynamic_mc_grid() {
    let mut scenario = SanityScenario {
        name: "admissible_mc",
        hourly_runoff_volume_m3: [3_600.0; 24],
        hourly_sediment_mass_kg: [0.0; 24],
    };
    scenario.hourly_runoff_volume_m3[6] = 3_960.0;
    for ipeak in [4, 5] {
        let output =
            run_admitted_mc_fixture(&format!("w11d_admitted_mc_i{ipeak}"), &scenario, ipeak);
        assert!(output.ebe_peak_runoff_m3_s.is_finite());
        assert!(output.ebe_peak_runoff_m3_s > 0.0);
        assert!(output.ebe_peak_runoff_m3_s <= 1.1 + 1.0e-12);
        assert!(output.ebe_runoff_volume_m3.is_finite());
        assert!(output.channel_storage_m3.is_finite());
        assert_scaled_close(
            output.channel_balance_m3,
            0.0,
            1.0e-9,
            "admitted MC channel balance",
        );
    }
}

#[test]
fn wshedw11c_hourly_routing_sanity_matrix() {
    let scenarios = sanity_scenarios();
    let mut observations = Vec::new();
    for ipeak in [3, 2] {
        let timesteps: &[u32] = if ipeak == 2 { &[3_600] } else { &[3_600, 600] };
        for dtchr_seconds in timesteps {
            for scenario in &scenarios {
                let prefix = format!("w11c_i{ipeak}_dt{dtchr_seconds}_{}", scenario.name);
                let output =
                    run_two_channel_sanity_fixture(&prefix, scenario, ipeak, *dtchr_seconds);
                let observation = SanityObservation {
                    ipeak,
                    dtchr_seconds: *dtchr_seconds,
                    scenario: scenario.name,
                    input_peak_m3_s: scenario.input_peak_m3_s(),
                    output,
                };
                assert_sanity_observation(&observation);
                print_sanity_observation(&observation);
                observations.push(observation);
            }
        }
    }

    let creams_early = find_sanity_observation(&observations, 2, 3_600, "early_spike");
    let creams_late = find_sanity_observation(&observations, 2, 3_600, "late_spike");
    assert_scaled_close(
        creams_early.output.ebe_runoff_volume_m3,
        creams_late.output.ebe_runoff_volume_m3,
        1.0e-9,
        "ipeak=2 shifted-pulse runoff identity",
    );
    assert_scaled_close(
        creams_early.output.ebe_peak_runoff_m3_s,
        creams_late.output.ebe_peak_runoff_m3_s,
        1.0e-9,
        "ipeak=2 shifted-pulse peak identity",
    );

    for ipeak in [3] {
        for dtchr_seconds in [3_600, 600] {
            let spike = find_sanity_observation(&observations, ipeak, dtchr_seconds, "early_spike");
            let spread =
                find_sanity_observation(&observations, ipeak, dtchr_seconds, "early_spread");
            let late = find_sanity_observation(&observations, ipeak, dtchr_seconds, "late_spike");
            if spread.output.ebe_peak_runoff_m3_s >= spike.output.ebe_peak_runoff_m3_s {
                println!(
                    "W11C_FINDING kind=spread_peak_not_lower ipeak={ipeak} dtchr={dtchr_seconds} spike_peak={:.15} spread_peak={:.15}",
                    spike.output.ebe_peak_runoff_m3_s, spread.output.ebe_peak_runoff_m3_s,
                );
            }
            if late.output.channel_storage_m3 <= spike.output.channel_storage_m3 {
                println!(
                    "W11C_FINDING kind=late_storage_not_higher ipeak={ipeak} dtchr={dtchr_seconds} early_storage={:.15} late_storage={:.15}",
                    spike.output.channel_storage_m3, late.output.channel_storage_m3,
                );
            }
        }
    }

    for ipeak in [3] {
        for scenario in ["early_spike", "early_spread", "uniform", "late_spike"] {
            let hourly = find_sanity_observation(&observations, ipeak, 3_600, scenario);
            let subhourly = find_sanity_observation(&observations, ipeak, 600, scenario);
            println!(
                "W11C_TIMESTEP ipeak={ipeak} scenario={scenario} peak_dt3600={:.15} peak_dt600={:.15} peak_delta={:.15} storage_dt3600={:.15} storage_dt600={:.15} storage_delta={:.15}",
                hourly.output.ebe_peak_runoff_m3_s,
                subhourly.output.ebe_peak_runoff_m3_s,
                subhourly.output.ebe_peak_runoff_m3_s - hourly.output.ebe_peak_runoff_m3_s,
                hourly.output.channel_storage_m3,
                subhourly.output.channel_storage_m3,
                subhourly.output.channel_storage_m3 - hourly.output.channel_storage_m3,
            );
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct SanityScenario {
    name: &'static str,
    hourly_runoff_volume_m3: [f64; 24],
    hourly_sediment_mass_kg: [f64; 24],
}

impl SanityScenario {
    fn input_peak_m3_s(self) -> f64 {
        self.hourly_runoff_volume_m3
            .into_iter()
            .fold(0.0_f64, f64::max)
            / 3_600.0
    }

    fn scalar_duration_seconds(self) -> f64 {
        let input_peak = self.input_peak_m3_s();
        if input_peak > 0.0 {
            self.hourly_runoff_volume_m3.iter().sum::<f64>() / input_peak
        } else {
            0.0
        }
    }
}

fn sanity_scenarios() -> [SanityScenario; 5] {
    let zero = SanityScenario {
        name: "zero",
        hourly_runoff_volume_m3: [0.0; 24],
        hourly_sediment_mass_kg: [0.0; 24],
    };

    let mut early_spike = zero;
    early_spike.name = "early_spike";
    early_spike.hourly_runoff_volume_m3[6] = 7_200.0;
    early_spike.hourly_sediment_mass_kg[6] = 240.0;

    let mut early_spread = zero;
    early_spread.name = "early_spread";
    for hour in 5..9 {
        early_spread.hourly_runoff_volume_m3[hour] = 1_800.0;
        early_spread.hourly_sediment_mass_kg[hour] = 60.0;
    }

    let uniform = SanityScenario {
        name: "uniform",
        hourly_runoff_volume_m3: [300.0; 24],
        hourly_sediment_mass_kg: [10.0; 24],
    };

    let mut late_spike = zero;
    late_spike.name = "late_spike";
    late_spike.hourly_runoff_volume_m3[23] = 7_200.0;
    late_spike.hourly_sediment_mass_kg[23] = 240.0;

    [zero, early_spike, early_spread, uniform, late_spike]
}

#[derive(Debug, Clone)]
struct SanityObservation {
    ipeak: i32,
    dtchr_seconds: u32,
    scenario: &'static str,
    input_peak_m3_s: f64,
    output: HourlyFixtureOutput,
}

#[allow(clippy::float_cmp, clippy::too_many_lines)]
fn assert_sanity_observation(observation: &SanityObservation) {
    let output = &observation.output;
    for (label, value) in [
        ("input runoff", output.hbp_hourly_runoff_sum_m3),
        ("input sediment", output.hbp_hourly_sediment_sum_kg),
        ("outlet peak", output.ebe_peak_runoff_m3_s),
        ("outlet runoff", output.ebe_runoff_volume_m3),
        ("outlet sediment", output.ebe_sediment_yield_kg),
        ("ending storage", output.channel_storage_m3),
        ("channel balance", output.channel_balance_m3),
    ] {
        assert!(
            value.is_finite(),
            "ipeak={} dtchr={} scenario={} {label} must be finite, observed {value}",
            observation.ipeak,
            observation.dtchr_seconds,
            observation.scenario,
        );
    }
    for (label, value) in [
        ("outlet peak", output.ebe_peak_runoff_m3_s),
        ("outlet runoff", output.ebe_runoff_volume_m3),
        ("outlet sediment", output.ebe_sediment_yield_kg),
    ] {
        assert!(
            value >= 0.0,
            "ipeak={} dtchr={} scenario={} {label} must be nonnegative, observed {value}",
            observation.ipeak,
            observation.dtchr_seconds,
            observation.scenario,
        );
    }
    assert_eq!(output.ebe_element_id, 2, "public event channel is terminal");
    assert!(
        output.channel_storage_m3 >= -1.0e-9,
        "hydraulic end storage must be nonnegative"
    );
    assert!(
        output.ebe_runoff_volume_m3 <= output.hbp_hourly_runoff_sum_m3 + 1.0e-9,
        "terminal volume cannot exceed the only external water input"
    );
    let authorized_initial_storage_m3 =
        output.ebe_runoff_volume_m3 + output.channel_storage_m3 - output.hbp_hourly_runoff_sum_m3;
    assert!(
        authorized_initial_storage_m3 >= -1.0e-9,
        "terminal volume plus final storage cannot fall below external input without a sink"
    );
    if observation.scenario != "uniform" {
        assert_scaled_close(
            authorized_initial_storage_m3,
            0.0,
            1.0e-9,
            "fresh zero-flow initial storage",
        );
    }
    assert_scaled_close(
        output.channel_balance_m3,
        0.0,
        1.0e-9,
        "published channel balance",
    );
    assert!(
        output.ebe_sediment_yield_kg <= output.hbp_hourly_sediment_sum_kg + 1.0e-9,
        "terminal sediment cannot exceed external sediment without channel detachment: input={}, outlet={}",
        output.hbp_hourly_sediment_sum_kg,
        output.ebe_sediment_yield_kg,
    );
    if observation.ipeak == 2 {
        assert_scaled_close(
            output.ebe_sediment_yield_kg,
            output.hbp_hourly_sediment_sum_kg,
            1.0e-9,
            "CREAMS terminal sediment mass",
        );
    }
    let sediment_residual = output.hbp_hourly_sediment_sum_kg - output.ebe_sediment_yield_kg;
    if sediment_residual.abs() > 1.0e-9 {
        println!(
            "W11C_FINDING kind=sediment_publication_delta ipeak={} dtchr={} scenario={} input_kg={:.15} outlet_kg={:.15} input_minus_output_kg={:.15}",
            observation.ipeak,
            observation.dtchr_seconds,
            observation.scenario,
            output.hbp_hourly_sediment_sum_kg,
            output.ebe_sediment_yield_kg,
            sediment_residual,
        );
    }

    if observation.scenario == "zero" {
        assert_eq!(output.ebe_peak_runoff_m3_s, 0.0);
        assert_eq!(output.ebe_runoff_volume_m3, 0.0);
        assert_eq!(output.ebe_sediment_yield_kg, 0.0);
        assert_eq!(output.channel_storage_m3, 0.0);
    }
}

fn print_sanity_observation(observation: &SanityObservation) {
    let output = &observation.output;
    let peak_ratio = if observation.input_peak_m3_s > 0.0 {
        output.ebe_peak_runoff_m3_s / observation.input_peak_m3_s
    } else {
        0.0
    };
    let water_residual =
        output.hbp_hourly_runoff_sum_m3 - output.ebe_runoff_volume_m3 - output.channel_storage_m3;
    let sediment_residual = output.hbp_hourly_sediment_sum_kg - output.ebe_sediment_yield_kg;
    println!(
        "W11C_RESULT ipeak={} dtchr={} scenario={} input_m3={:.15} input_peak_m3_s={:.15} outlet_m3={:.15} storage_m3={:.15} peak_m3_s={:.15} peak_ratio={:.15} water_residual_m3={:.15} input_sediment_kg={:.15} outlet_sediment_kg={:.15} sediment_residual_kg={:.15} channel_balance_m3={:.15}",
        observation.ipeak,
        observation.dtchr_seconds,
        observation.scenario,
        output.hbp_hourly_runoff_sum_m3,
        observation.input_peak_m3_s,
        output.ebe_runoff_volume_m3,
        output.channel_storage_m3,
        output.ebe_peak_runoff_m3_s,
        peak_ratio,
        water_residual,
        output.hbp_hourly_sediment_sum_kg,
        output.ebe_sediment_yield_kg,
        sediment_residual,
        output.channel_balance_m3,
    );
}

fn find_sanity_observation<'a>(
    observations: &'a [SanityObservation],
    ipeak: i32,
    dtchr_seconds: u32,
    scenario: &str,
) -> &'a SanityObservation {
    observations
        .iter()
        .find(|observation| {
            observation.ipeak == ipeak
                && observation.dtchr_seconds == dtchr_seconds
                && observation.scenario == scenario
        })
        .unwrap_or_else(|| {
            panic!(
                "missing sanity observation ipeak={ipeak} dtchr={dtchr_seconds} scenario={scenario}"
            )
        })
}

fn assert_scaled_close(observed: f64, expected: f64, relative_tolerance: f64, label: &str) {
    let delta = (observed - expected).abs();
    let tolerance = relative_tolerance * expected.abs().max(1.0);
    assert!(
        delta <= tolerance,
        "{label} mismatch: expected {expected}, observed {observed}, delta {delta}, tolerance {tolerance}"
    );
}

#[derive(Debug, Clone)]
struct HourlyFixtureOutput {
    hbp_hourly_runoff_sum_m3: f64,
    hbp_hourly_sediment_sum_kg: f64,
    ebe_peak_runoff_m3_s: f64,
    ebe_runoff_volume_m3: f64,
    ebe_sediment_yield_kg: f64,
    ebe_element_id: i64,
    channel_storage_m3: f64,
    channel_balance_m3: f64,
}

fn run_hourly_fixture(
    prefix: &str,
    hourly_runoff_volume_m3: [f64; 24],
    hourly_sediment_mass_kg: [f64; 24],
) -> HourlyFixtureOutput {
    let run_dir = build_watershed_fixture_dir(prefix);
    let channel_controls = fs::read_to_string(run_dir.join("pw0.chn"))
        .expect("channel controls should be readable")
        .replacen("\n4\n", "\n3\n", 1);
    fs::write(run_dir.join("pw0.chn"), channel_controls)
        .expect("KW channel controls should be writable");
    write_hourly_hbp_fixture(
        run_dir.join("H1.hbp"),
        1,
        2.0,
        3_600.0,
        240.0,
        0.0,
        hourly_runoff_volume_m3,
        hourly_sediment_mass_kg,
    );
    write_watershed_runfile(&run_dir, &[1]);

    let (hbp, latest_event_payload) = parse_hbp_from_path_with_latest_event_payload(
        run_dir.join("H1.hbp"),
        HbpParseOptions {
            expected_hillslope_id: Some(1),
        },
    )
    .expect("schema-1.1 HBP fixture should parse");
    let payload = latest_event_payload.expect("schema-1.1 fixture should contain EVENT payload");
    assert_eq!(hbp.schema_major, 1);
    assert_eq!(hbp.schema_minor, 1);
    assert_eq!(payload.hourly_runoff_volume_m3.len(), 24);
    assert_eq!(payload.hourly_sediment_mass_kg.len(), 24);

    let output_dir = run_dir.join("out");
    let output = run_watershed_cli(&run_dir, &output_dir);
    assert!(
        output.status.success(),
        "watershed CLI should consume schema-1.1 HBP hourly pair; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );

    let ebe_row = read_first_parquet_row(&output_dir.join("interchange/ebe_pw0.parquet"));
    let channel_row = read_first_parquet_row(&output_dir.join("interchange/chanwb.parquet"));
    HourlyFixtureOutput {
        hbp_hourly_runoff_sum_m3: payload.hourly_runoff_volume_m3.iter().sum(),
        hbp_hourly_sediment_sum_kg: payload.hourly_sediment_mass_kg.iter().sum(),
        ebe_peak_runoff_m3_s: row_f64_value(&ebe_row, "peak_runoff"),
        ebe_runoff_volume_m3: row_f64_value(&ebe_row, "runoff_volume"),
        ebe_sediment_yield_kg: row_f64_value(&ebe_row, "sediment_yield"),
        ebe_element_id: row_i64_value(&ebe_row, "element_id"),
        channel_storage_m3: row_f64_value(&channel_row, "Storage (m^3)"),
        channel_balance_m3: row_f64_value(&channel_row, "Balance (m^3)"),
    }
}

fn run_admitted_mc_fixture(
    prefix: &str,
    scenario: &SanityScenario,
    ipeak: i32,
) -> HourlyFixtureOutput {
    let run_dir = build_watershed_fixture_dir(prefix);
    let channel_controls = format!(
        concat!(
            "99.1\n1\n{}\n1.500000\n",
            "channel 1 comment a\nchannel 1 comment b\nchannel 1 comment c\n",
            "2\n1\n1\n0\n1.0 0.04\n0.05 0.001 2.0 0.25 0.15\n0.02 1.4 0.045\n",
        ),
        ipeak,
    );
    fs::write(run_dir.join("pw0.chn"), channel_controls)
        .expect("MC channel controls should be writable");
    fs::write(
        run_dir.join("pw0.slp"),
        concat!(
            "# W11D admitted MC geometry\n97.5\n2\n",
            "100.0 0.6096\n3 60.0\n0.0 0.0100 0.6 0.0100 1.0 0.0100\n",
            "100.0 0.6096\n3 40.0\n0.0 0.0100 0.5 0.0100 1.0 0.0100\n",
        ),
    )
    .expect("admitted MC slope fixture should be writable");
    fs::write(run_dir.join("chan.inp"), "3 60\n0.0\n1\n3\n")
        .expect("admitted MC chan.inp should be writable");
    write_hourly_hbp_fixture(
        run_dir.join("H1.hbp"),
        1,
        scenario.input_peak_m3_s(),
        scenario.scalar_duration_seconds(),
        0.0,
        0.0,
        scenario.hourly_runoff_volume_m3,
        scenario.hourly_sediment_mass_kg,
    );
    write_watershed_runfile(&run_dir, &[1]);
    let output_dir = run_dir.join("out");
    let output = run_watershed_cli(&run_dir, &output_dir);
    assert!(
        output.status.success(),
        "admitted ipeak={ipeak} MC CLI route should succeed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let ebe_row = read_first_parquet_row(&output_dir.join("interchange/ebe_pw0.parquet"));
    let channel_row = read_first_parquet_row(&output_dir.join("interchange/chanwb.parquet"));
    HourlyFixtureOutput {
        hbp_hourly_runoff_sum_m3: scenario.hourly_runoff_volume_m3.iter().sum(),
        hbp_hourly_sediment_sum_kg: scenario.hourly_sediment_mass_kg.iter().sum(),
        ebe_peak_runoff_m3_s: row_f64_value(&ebe_row, "peak_runoff"),
        ebe_runoff_volume_m3: row_f64_value(&ebe_row, "runoff_volume"),
        ebe_sediment_yield_kg: row_f64_value(&ebe_row, "sediment_yield"),
        ebe_element_id: row_i64_value(&ebe_row, "element_id"),
        channel_storage_m3: row_f64_value(&channel_row, "Storage (m^3)"),
        channel_balance_m3: row_f64_value(&channel_row, "Balance (m^3)"),
    }
}

fn run_two_channel_hourly_fixture(
    prefix: &str,
    hourly_runoff_volume_m3: [f64; 24],
    hourly_sediment_mass_kg: [f64; 24],
) -> HourlyFixtureOutput {
    let scenario = SanityScenario {
        name: "w11b_protected",
        hourly_runoff_volume_m3,
        hourly_sediment_mass_kg,
    };
    run_two_channel_fixture(prefix, &scenario, 3, 600, 2.0, 3_600.0, 240.0)
}

fn run_two_channel_sanity_fixture(
    prefix: &str,
    scenario: &SanityScenario,
    ipeak: i32,
    dtchr_seconds: u32,
) -> HourlyFixtureOutput {
    run_two_channel_fixture(
        prefix,
        scenario,
        ipeak,
        dtchr_seconds,
        scenario.input_peak_m3_s(),
        scenario.scalar_duration_seconds(),
        scenario.hourly_sediment_mass_kg.iter().sum(),
    )
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn run_two_channel_fixture(
    prefix: &str,
    scenario: &SanityScenario,
    ipeak: i32,
    dtchr_seconds: u32,
    scalar_peak_runoff_m3_s: f64,
    scalar_duration_seconds: f64,
    total_detachment_kg: f64,
) -> HourlyFixtureOutput {
    try_run_two_channel_fixture(
        prefix,
        scenario,
        ipeak,
        dtchr_seconds,
        scalar_peak_runoff_m3_s,
        scalar_duration_seconds,
        total_detachment_kg,
    )
    .unwrap_or_else(|error| panic!("two-channel watershed CLI fixture should route: {error}"))
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn try_run_two_channel_fixture(
    prefix: &str,
    scenario: &SanityScenario,
    ipeak: i32,
    dtchr_seconds: u32,
    scalar_peak_runoff_m3_s: f64,
    scalar_duration_seconds: f64,
    total_detachment_kg: f64,
) -> Result<HourlyFixtureOutput, String> {
    let run_dir = build_watershed_fixture_dir(prefix);
    fs::write(
        run_dir.join("pw0.str"),
        "94.301\n2 1 0 0 0 0 0 0 0 0\n2 2 0 0 3 0 0 0 0 0\n",
    )
    .expect("two-channel structure should be writable");
    let channel_controls = format!(
        concat!(
            "99.1\n2\n{}\n1.500000\n",
            "channel 1 comment a\nchannel 1 comment b\nchannel 1 comment c\n",
            "1\n1\n1\n0\n19.99 0.03\n0.04 0.000001 19.0 900.0 0.0001\n0.02 4.0 0.04\n",
            "channel 2 comment a\nchannel 2 comment b\nchannel 2 comment c\n",
            "1\n1\n1\n0\n19.99 0.03\n0.04 0.000001 19.0 900.0 0.0001\n0.02 4.0 0.04\n",
        ),
        ipeak,
    );
    fs::write(run_dir.join("pw0.chn"), channel_controls)
        .expect("two-channel controls should be writable");
    copy_fixture_file(
        &repo_root().join("tests/fixtures/infile/slope/strict_valid_canonical.slp"),
        &run_dir.join("pw0.slp"),
    );
    fs::write(
        run_dir.join("chan.inp"),
        format!("3 {dtchr_seconds}\n0.0\n2\n3 4\n"),
    )
    .expect("two-channel chan.inp should be writable");
    if ipeak > 2 {
        let parsed = parse_chaninp_from_path(
            run_dir.join("chan.inp"),
            ChaninpParseOptions::compatibility(ipeak, 2),
            &BTreeSet::from([3, 4]),
        )
        .expect("W11C chan.inp should parse");
        assert_eq!(parsed.parse_outcome, ChaninpParseOutcome::ParsedBranch);
        assert!(
            parsed.warnings.is_empty(),
            "canonical W11C chan.inp should parse without warnings: {:?}",
            parsed.warnings
        );
        let options = parsed.options.expect("wave branch should expose options");
        assert_eq!(
            options.dtchr_norm_s,
            i32::try_from(dtchr_seconds).expect("W11C timestep should fit i32"),
        );
        assert_eq!(
            options.ntchr,
            i32::try_from(86_400 / dtchr_seconds).expect("W11C interval count should fit i32"),
        );
        assert_eq!(options.nchnum_norm, 2);
        assert_eq!(options.ichnum_norm, [3, 4]);
    }

    let creams_zero_no_event = ipeak == 2 && scenario.name == "zero";
    if creams_zero_no_event {
        write_no_event_hbp_fixture(run_dir.join("H1.hbp"), 1);
        write_no_event_hbp_fixture(run_dir.join("H2.hbp"), 2);
    } else {
        write_hourly_hbp_fixture(
            run_dir.join("H1.hbp"),
            1,
            scalar_peak_runoff_m3_s,
            scalar_duration_seconds,
            total_detachment_kg,
            0.0,
            scenario.hourly_runoff_volume_m3,
            scenario.hourly_sediment_mass_kg,
        );
        if ipeak == 2 {
            write_no_event_hbp_fixture(run_dir.join("H2.hbp"), 2);
        } else {
            write_hourly_hbp_fixture(
                run_dir.join("H2.hbp"),
                2,
                0.0,
                0.0,
                0.0,
                0.0,
                [0.0; 24],
                [0.0; 24],
            );
        }
    }
    write_watershed_runfile(&run_dir, &[1, 2]);

    let h1_totals = parsed_hbp_event_totals(&run_dir.join("H1.hbp"), 1);
    let h2_totals = parsed_hbp_event_totals(&run_dir.join("H2.hbp"), 2);
    if creams_zero_no_event {
        assert!(
            h1_totals.is_none() && h2_totals.is_none(),
            "CREAMS zero control should serialize two NOEVENT payloads"
        );
    } else {
        assert!(h1_totals.is_some(), "source HBP should contain an event");
        if ipeak == 2 {
            assert!(
                h2_totals.is_none(),
                "CREAMS downstream HBP should be NOEVENT"
            );
        } else {
            assert!(
                h2_totals.is_some(),
                "wave downstream HBP should contain an event"
            );
        }
    }
    let (hbp_hourly_runoff_sum_m3, hbp_hourly_sediment_sum_kg) =
        [h1_totals, h2_totals].into_iter().flatten().fold(
            (0.0, 0.0),
            |(runoff, sediment), (next_runoff, next_sediment)| {
                (runoff + next_runoff, sediment + next_sediment)
            },
        );
    let output_dir = run_dir.join("out");
    let output = run_watershed_cli(&run_dir, &output_dir);
    if !output.status.success() {
        return Err(format!(
            "fixture='{prefix}' ipeak={ipeak} dtchr={dtchr_seconds}: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let ebe_row = read_first_parquet_row(&output_dir.join("interchange/ebe_pw0.parquet"));
    let channel_row = read_first_parquet_row(&output_dir.join("interchange/chanwb.parquet"));
    Ok(HourlyFixtureOutput {
        hbp_hourly_runoff_sum_m3,
        hbp_hourly_sediment_sum_kg,
        ebe_peak_runoff_m3_s: row_f64_value(&ebe_row, "peak_runoff"),
        ebe_runoff_volume_m3: row_f64_value(&ebe_row, "runoff_volume"),
        ebe_sediment_yield_kg: row_f64_value(&ebe_row, "sediment_yield"),
        ebe_element_id: row_i64_value(&ebe_row, "element_id"),
        channel_storage_m3: row_f64_value(&channel_row, "Storage (m^3)"),
        channel_balance_m3: row_f64_value(&channel_row, "Balance (m^3)"),
    })
}

fn parsed_hbp_event_totals(path: &Path, expected_hillslope_id: u32) -> Option<(f64, f64)> {
    let (hbp, payload) = parse_hbp_from_path_with_latest_event_payload(
        path,
        HbpParseOptions {
            expected_hillslope_id: Some(expected_hillslope_id),
        },
    )
    .expect("serialized two-channel HBP should parse");
    assert_eq!(hbp.schema_major, 1);
    payload.map(|event| {
        assert_eq!(event.hourly_runoff_volume_m3.len(), 24);
        assert_eq!(event.hourly_sediment_mass_kg.len(), 24);
        (
            event.hourly_runoff_volume_m3.iter().sum(),
            event.hourly_sediment_mass_kg.iter().sum(),
        )
    })
}

fn build_watershed_fixture_dir(prefix: &str) -> PathBuf {
    let destination = unique_temp_dir(prefix);
    fs::create_dir_all(&destination).expect("fixture directory should be creatable");

    copy_fixture_file(
        &repo_root().join("tests/fixtures/infile/watershed_structure/strict_valid_two_rows.str"),
        &destination.join("pw0.str"),
    );
    copy_fixture_file(
        &repo_root().join("tests/fixtures/infile/watershed_channel/strict_sidecar_required.chn"),
        &destination.join("pw0.chn"),
    );
    copy_fixture_file(
        &repo_root().join("tests/fixtures/infile/watershed_impoundment/strict_valid_minimal.imp"),
        &destination.join("pw0.imp"),
    );
    copy_fixture_file(
        &repo_root().join("tests/fixtures/cli01/hillslope_run_dir/case.man"),
        &destination.join("pw0.man"),
    );
    copy_fixture_file(
        &repo_root().join("tests/fixtures/cli01/hillslope_run_dir/case.slp"),
        &destination.join("pw0.slp"),
    );
    copy_fixture_file(
        &repo_root().join("tests/fixtures/cli01/hillslope_run_dir/case.cli"),
        &destination.join("pw0.cli"),
    );
    copy_fixture_file(
        &repo_root().join("tests/fixtures/cli01/hillslope_run_dir/case.sol"),
        &destination.join("pw0.sol"),
    );
    fs::write(destination.join("pw0.str"), "94.301\n2 1 0 0 0 0 0 0 0 0\n")
        .expect("channel-only structure fixture should be writable");
    fs::write(destination.join("chan.inp"), "3 600\n0.000001\n1\n2\n")
        .expect("channel-only chan.inp fixture should be writable");

    destination
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("unix epoch should be before now")
        .as_nanos();
    std::env::temp_dir().join(format!("{prefix}_{timestamp}"))
}

fn copy_fixture_file(source: &Path, destination: &Path) {
    fs::copy(source, destination).unwrap_or_else(|error| {
        panic!(
            "fixture copy should succeed ({} -> {}): {error}",
            source.display(),
            destination.display()
        )
    });
}

fn write_watershed_runfile(run_dir: &Path, hillslope_ids: &[u32]) {
    let mut runfile_payload = String::from(
        r#"
schema = "openwepp-watershed-runfile-v1"
run_name = "mt3-hbp-hourly-consumer-contract"
unit_system = "metric"

[inputs]
pw0_str = "pw0.str"
pw0_chn = "pw0.chn"
pw0_imp = "pw0.imp"
pw0_man = "pw0.man"
pw0_slp = "pw0.slp"
pw0_cli = "pw0.cli"
pw0_sol = "pw0.sol"
chaninp = "chan.inp"

[inputs.applicability]
chapter13_small_watershed_intent = true
allow_partial_area_response = false
allow_headcutting = false
allow_bank_sloughing = false
allow_perennial_streams = false
"#,
    );

    for hillslope_id in hillslope_ids {
        write!(
            &mut runfile_payload,
            r#"
[[inputs.hillslopes_block]]
hillslope_id = {hillslope_id}
pass_file = "H{hillslope_id}.hbp"
use_existing_pass_file = true
"#
        )
        .expect("watershed runfile block should format");
    }

    runfile_payload.push_str(
        r#"
[outputs]
ebe_pw0 = "interchange/ebe_pw0.parquet"
chan_out = "interchange/chan.out.parquet"
chanwb = "interchange/chanwb.parquet"
chnwb = "interchange/chnwb.parquet"
soil_pw0 = "interchange/soil_pw0.parquet"
totalwatsed3 = "interchange/totalwatsed3.parquet"
loss_hill = "interchange/loss_pw0.hill.parquet"
loss_chn = "interchange/loss_pw0.chn.parquet"
loss_out = "interchange/loss_pw0.out.parquet"
loss_class_data = "interchange/loss_pw0.class_data.parquet"
loss_all_years_hill = "interchange/loss_pw0.all_years.hill.parquet"
loss_all_years_chn = "interchange/loss_pw0.all_years.chn.parquet"
loss_all_years_out = "interchange/loss_pw0.all_years.out.parquet"
loss_all_years_class_data = "interchange/loss_pw0.class_data.parquet"
"#,
    );
    fs::write(run_dir.join("case.run"), runfile_payload).expect("runfile should be writable");
}

fn run_watershed_cli(run_dir: &Path, output_dir: &Path) -> std::process::Output {
    let binary = std::env::var_os("OPENWEPP_W11C_WATERSHED_CLI").map_or_else(
        || PathBuf::from(env!("CARGO_BIN_EXE_openwepp-cli-watershed")),
        |configured| {
            let configured = PathBuf::from(configured);
            if configured.is_absolute() {
                configured
            } else {
                repo_root().join(configured)
            }
        },
    );
    Command::new(binary)
        .current_dir(Path::new(env!("CARGO_MANIFEST_DIR")))
        .arg("--run-dir")
        .arg(run_dir)
        .arg("--run-file")
        .arg("case.run")
        .arg("--output-dir")
        .arg(output_dir)
        .arg("--policy")
        .arg("compat")
        .output()
        .expect("watershed CLI should launch")
}

fn read_first_parquet_row(path: &Path) -> Row {
    let file = File::open(path).unwrap_or_else(|error| {
        panic!(
            "parquet output should be readable ({}): {error}",
            path.display()
        )
    });
    let reader = SerializedFileReader::new(file).unwrap_or_else(|error| {
        panic!("parquet output should parse ({}): {error}", path.display())
    });
    let mut rows = reader.get_row_iter(None).unwrap_or_else(|error| {
        panic!(
            "parquet row iterator should open ({}): {error}",
            path.display()
        )
    });
    rows.next()
        .unwrap_or_else(|| panic!("expected at least one row in {}", path.display()))
        .unwrap_or_else(|error| panic!("first parquet row should decode: {error}"))
}

fn row_index(row: &Row, column_name: &str) -> usize {
    row.get_column_iter()
        .enumerate()
        .find(|(_, (name, _))| name.as_str() == column_name)
        .map_or_else(
            || panic!("missing required parquet column '{column_name}'"),
            |(index, _)| index,
        )
}

fn row_f64_value(row: &Row, column_name: &str) -> f64 {
    let index = row_index(row, column_name);
    if let Ok(value) = row.get_double(index) {
        return value;
    }
    if let Ok(value) = row.get_float(index) {
        return f64::from(value);
    }
    if let Ok(value) = row.get_int(index) {
        return f64::from(value);
    }
    if let Ok(value) = row.get_short(index) {
        return f64::from(value);
    }
    if let Ok(value) = row.get_long(index) {
        return value as f64;
    }
    panic!("column '{column_name}' does not decode as numeric");
}

fn row_i64_value(row: &Row, column_name: &str) -> i64 {
    let index = row_index(row, column_name);
    if let Ok(value) = row.get_long(index) {
        return value;
    }
    if let Ok(value) = row.get_int(index) {
        return i64::from(value);
    }
    if let Ok(value) = row.get_short(index) {
        return i64::from(value);
    }
    panic!("column '{column_name}' does not decode as an integer");
}

fn assert_relative_close(observed: f64, expected: f64, tolerance: f64, label: &str) {
    let delta = (observed - expected).abs();
    assert!(
        delta <= tolerance,
        "{label} mismatch: expected {expected}, observed {observed}, delta {delta}"
    );
}

fn write_hourly_hbp_fixture(
    path: PathBuf,
    hillslope_id: u32,
    scalar_peak_runoff_m3_s: f64,
    scalar_duration_seconds: f64,
    total_detachment_kg: f64,
    total_deposition_kg: f64,
    hourly_runoff_volume_m3: [f64; 24],
    hourly_sediment_mass_kg: [f64; 24],
) {
    let bytes = build_schema1_1_event_fixture(
        hillslope_id,
        scalar_peak_runoff_m3_s,
        scalar_duration_seconds,
        total_detachment_kg,
        total_deposition_kg,
        hourly_runoff_volume_m3,
        hourly_sediment_mass_kg,
    );
    fs::write(path, bytes).expect("HBP fixture should be writable");
}

fn write_no_event_hbp_fixture(path: PathBuf, hillslope_id: u32) {
    let bytes = build_schema1_1_no_event_fixture(hillslope_id);
    fs::write(path, bytes).expect("NOEVENT HBP fixture should be writable");
}

fn build_schema1_1_no_event_fixture(hillslope_id: u32) -> Vec<u8> {
    let mut file = append_common_prefix(hillslope_id);
    let mut payload = Vec::new();
    put_u32(&mut payload, 1);
    put_i32(&mut payload, 2004);
    put_u16(&mut payload, 1);
    put_u8(&mut payload, 0);
    put_u16(&mut payload, 0);
    put_u16(&mut payload, REQUIRED_STATE_IDS.len() as u16);
    put_i64(&mut payload, 0);
    put_i64(&mut payload, 0);
    for state_id in REQUIRED_STATE_IDS {
        payload.extend_from_slice(&build_state_entry(*state_id));
    }
    let payload_crc = crc32c(&payload);

    let directory_start = file.len();
    let directory_len = 4 + 27;
    let payload_offset = directory_start + directory_len;
    let mut directory = Vec::new();
    put_u32(&mut directory, 1);
    put_u32(&mut directory, 1);
    put_i32(&mut directory, 2004);
    put_u16(&mut directory, 1);
    put_u8(&mut directory, 0);
    put_u64(&mut directory, payload_offset as u64);
    put_u32(&mut directory, payload.len() as u32);
    put_u32(&mut directory, payload_crc);

    file.extend_from_slice(&directory);
    file.extend_from_slice(&payload);
    let directory_crc = crc32c(&directory);
    put_u32(&mut file, directory_crc);
    let file_crc_pos = file.len();
    put_u32(&mut file, 0);
    put_u32(&mut file, 1);
    file.extend_from_slice(FOOTER_MAGIC);
    let file_crc = crc32c(&file);
    put_u32_at(&mut file, file_crc_pos, file_crc);
    file
}

fn build_schema1_1_event_fixture(
    hillslope_id: u32,
    peak_runoff_m3_s: f64,
    duration_seconds: f64,
    total_detachment_kg: f64,
    total_deposition_kg: f64,
    hourly_runoff_volume_m3: [f64; 24],
    hourly_sediment_mass_kg: [f64; 24],
) -> Vec<u8> {
    let mut file = append_common_prefix(hillslope_id);
    let payload = build_event_payload(
        duration_seconds,
        peak_runoff_m3_s,
        total_detachment_kg,
        total_deposition_kg,
        hourly_runoff_volume_m3,
        hourly_sediment_mass_kg,
    );
    let payload_crc = crc32c(&payload);

    let directory_start = file.len();
    let directory_len = 4 + 27;
    let payload_offset = directory_start + directory_len;
    let mut directory = Vec::new();
    put_u32(&mut directory, 1);
    put_u32(&mut directory, 1);
    put_i32(&mut directory, 2004);
    put_u16(&mut directory, 1);
    put_u8(&mut directory, 2);
    put_u64(&mut directory, payload_offset as u64);
    put_u32(&mut directory, payload.len() as u32);
    put_u32(&mut directory, payload_crc);

    file.extend_from_slice(&directory);
    file.extend_from_slice(&payload);

    let directory_crc = crc32c(&directory);
    put_u32(&mut file, directory_crc);
    let file_crc_pos = file.len();
    put_u32(&mut file, 0);
    put_u32(&mut file, 1);
    file.extend_from_slice(FOOTER_MAGIC);
    let file_crc = crc32c(&file);
    put_u32_at(&mut file, file_crc_pos, file_crc);
    file
}

fn append_common_prefix(hillslope_id: u32) -> Vec<u8> {
    let mut file = Vec::new();
    let mut header = Vec::new();
    header.extend_from_slice(MAGIC);
    put_u16(&mut header, SUPPORTED_MAJOR_V1);
    put_u16(&mut header, SUPPORTED_MINOR_V1);
    put_u8(&mut header, 1);
    let header_bytes_pos = header.len();
    put_u32(&mut header, 0);
    header.extend_from_slice(&[0u8; 32]);
    put_u8(&mut header, 1);
    put_string(&mut header, "openwepp-mt3-test");
    put_string(&mut header, "mt3-hourly-consumer");
    put_string(&mut header, "2026-07-09T00:00:00Z");
    put_string(&mut header, "metric-v1");
    header.extend_from_slice(&[0u8; 32]);
    let header_crc_pos = header.len();
    put_u32(&mut header, 0);
    let header_bytes = header.len() as u32;
    put_u32_at(&mut header, header_bytes_pos, header_bytes);
    let header_crc = crc32c(&header);
    put_u32_at(&mut header, header_crc_pos, header_crc);
    file.extend_from_slice(&header);

    let npart = 1_u16;
    let nofe = 1_u16;
    let max_layers = 1_u16;

    put_u32(&mut file, hillslope_id);
    put_u32(&mut file, 1);
    put_i32(&mut file, 2004);
    put_u16(&mut file, npart);
    put_u16(&mut file, nofe);
    put_u16(&mut file, max_layers);
    put_string(&mut file, "gregorian");
    put_u16(&mut file, 1);
    put_u8(&mut file, 1);

    put_string(&mut file, "p1.cli");
    put_i64(&mut file, 0);
    put_u32(&mut file, u32::from(npart));
    put_f64(&mut file, 0.001);
    put_f64(&mut file, 0.0);
    put_f64(&mut file, 0.0);
    put_f64(&mut file, 0.0);
    put_f64(&mut file, 0.0);

    put_u32(&mut file, 1);
    put_u32(&mut file, 1);
    put_i32(&mut file, 2004);
    put_u16(&mut file, 1);
    put_u16(&mut file, 1);
    put_u16(&mut file, 1);
    put_u8(&mut file, 0);

    put_u32(&mut file, REQUIRED_STATE_IDS.len() as u32);
    for state_id in REQUIRED_STATE_IDS {
        let (required_flag, representation_class, unit_class, rank, dims_kind) =
            expected_state_schema(*state_id).expect("required state schema should exist");
        put_u16(&mut file, *state_id);
        put_u8(&mut file, required_flag);
        put_u8(&mut file, representation_class);
        put_u16(&mut file, unit_class);
        put_u8(&mut file, rank);
        put_u8(&mut file, dims_kind);
        put_string(&mut file, &format!("state_{state_id}"));
    }

    file
}

fn build_event_payload(
    duration_seconds: f64,
    peak_runoff_m3_s: f64,
    total_detachment_kg: f64,
    total_deposition_kg: f64,
    hourly_runoff_volume_m3: [f64; 24],
    hourly_sediment_mass_kg: [f64; 24],
) -> Vec<u8> {
    let mut payload = Vec::new();
    put_u32(&mut payload, 1);
    put_i32(&mut payload, 2004);
    put_u16(&mut payload, 1);
    put_u8(&mut payload, 2);
    put_u16(&mut payload, SUPPORTED_MINOR_V1);
    put_u16(&mut payload, REQUIRED_STATE_IDS.len() as u16);
    put_f64(&mut payload, duration_seconds);
    put_f64(&mut payload, 0.5);
    put_f64(&mut payload, 0.8);
    for _ in 0..6 {
        put_i64(&mut payload, 0);
    }
    put_f64(&mut payload, peak_runoff_m3_s);
    put_i64(&mut payload, scaled_i64(total_detachment_kg));
    put_i64(&mut payload, scaled_i64(total_deposition_kg));
    put_u32(&mut payload, 1);
    put_f64(&mut payload, 0.25);
    put_u32(&mut payload, 1);
    put_f64(&mut payload, 1.0);
    put_u32(&mut payload, 24);
    for volume_m3 in hourly_runoff_volume_m3 {
        put_f64(&mut payload, volume_m3);
    }
    put_u32(&mut payload, 24);
    for sediment_kg in hourly_sediment_mass_kg {
        put_f64(&mut payload, sediment_kg);
    }
    put_i64(&mut payload, 0);
    put_i64(&mut payload, 0);

    for state_id in REQUIRED_STATE_IDS {
        payload.extend_from_slice(&build_state_entry(*state_id));
    }

    payload
}

fn expected_state_schema(state_id: u16) -> Option<(u8, u8, u16, u8, u8)> {
    match state_id {
        1 => Some((1, 1, 1, 1, DIM_NOFE)),
        2..=5 | 100..=102 | 210 | 900 | 901 => Some((1, 1, 2, 2, DIM_NOFE_LAYERS)),
        6 | 7 => Some((1, 2, 3, 2, DIM_NOFE_LAYERS)),
        103 | 104 | 200 | 202..=209 => Some((1, 1, 2, 1, DIM_NOFE)),
        201 => Some((1, 2, 4, 1, DIM_NOFE)),
        300 => Some((1, 1, 5, 0, DIM_SCALAR)),
        _ => None,
    }
}

fn build_state_entry(state_id: u16) -> Vec<u8> {
    let nofe = 1_u32;
    let max_layers = 1_u32;
    let (required_flag, representation_class, unit_class, rank, dims_kind) =
        expected_state_schema(state_id).expect("required state schema should exist");
    let dims = state_dims(dims_kind, nofe, max_layers);
    assert_eq!(dims.len(), usize::from(rank));

    let mut entry = Vec::new();
    put_u8(&mut entry, required_flag);
    put_u8(&mut entry, representation_class);
    put_u16(&mut entry, unit_class);
    put_u8(&mut entry, rank);
    for dim in &dims {
        put_u32(&mut entry, *dim);
    }

    let value_count = dims.iter().copied().product::<u32>().max(1) as usize;
    match representation_class {
        1 => {
            for _ in 0..value_count {
                put_i64(&mut entry, 0);
            }
        }
        2 => {
            for _ in 0..value_count {
                put_f64(&mut entry, 0.0);
            }
        }
        _ => panic!("unsupported representation class"),
    }

    let mut out = Vec::new();
    put_u16(&mut out, state_id);
    put_u32(&mut out, entry.len() as u32);
    out.extend_from_slice(&entry);
    out
}

fn state_dims(dims_kind: u8, nofe: u32, max_layers: u32) -> Vec<u32> {
    match dims_kind {
        DIM_SCALAR => vec![],
        DIM_NOFE => vec![nofe],
        DIM_NOFE_LAYERS => vec![nofe, max_layers],
        _ => panic!("unknown dims_kind {dims_kind}"),
    }
}

fn put_u8(buf: &mut Vec<u8>, value: u8) {
    buf.push(value);
}

fn put_u16(buf: &mut Vec<u8>, value: u16) {
    buf.extend_from_slice(&value.to_le_bytes());
}

fn put_u32(buf: &mut Vec<u8>, value: u32) {
    buf.extend_from_slice(&value.to_le_bytes());
}

fn put_i32(buf: &mut Vec<u8>, value: i32) {
    buf.extend_from_slice(&value.to_le_bytes());
}

fn put_u64(buf: &mut Vec<u8>, value: u64) {
    buf.extend_from_slice(&value.to_le_bytes());
}

fn put_i64(buf: &mut Vec<u8>, value: i64) {
    buf.extend_from_slice(&value.to_le_bytes());
}

fn put_f64(buf: &mut Vec<u8>, value: f64) {
    buf.extend_from_slice(&value.to_le_bytes());
}

fn put_string(buf: &mut Vec<u8>, value: &str) {
    put_u32(buf, value.len() as u32);
    buf.extend_from_slice(value.as_bytes());
}

fn put_u32_at(buf: &mut [u8], offset: usize, value: u32) {
    buf[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn scaled_i64(value: f64) -> i64 {
    let scaled = value * SCALE_INV_I64;
    assert!(scaled.is_finite());
    assert!(scaled >= i64::MIN as f64 && scaled <= i64::MAX as f64);
    scaled.round() as i64
}

fn crc32c(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for value in data {
        crc ^= u32::from(*value);
        for _ in 0..8 {
            if crc & 1 == 1 {
                crc = (crc >> 1) ^ 0x82F63B78;
            } else {
                crc >>= 1;
            }
            crc &= 0xFFFF_FFFF;
        }
    }
    crc ^ 0xFFFF_FFFF
}
