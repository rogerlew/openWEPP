const REPRESENTATIVE_LONG_RUN_RECIPE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/work-packages/",
    "20260901-stage3-native-vegetation-laned-watershed-throughput-recovery-001/",
    "artifacts/representative-10ofe-100year-workload.json",
);
const REPRESENTATIVE_LONG_RUN_OFE_COUNT: usize = 10;
const REPRESENTATIVE_LONG_RUN_YEAR_DAYS: usize = 365;
const REPRESENTATIVE_LONG_RUN_CENTURY_DAYS: usize = 36_525;
const REPRESENTATIVE_PUBLICATION_REL_TOLERANCE: f64 = 1.0e-9;

fn representative_long_run_recipe() -> serde_json::Value {
    serde_json::from_slice(
        &std::fs::read(REPRESENTATIVE_LONG_RUN_RECIPE)
            .expect("read representative long-run workload recipe"),
    )
    .expect("parse representative long-run workload recipe")
}

fn representative_long_run_areas_m2() -> Vec<f64> {
    representative_long_run_recipe()["ofe_areas_m2"]
        .as_array()
        .expect("representative OFE areas")
        .iter()
        .map(|value| value.as_f64().expect("finite representative OFE area"))
        .collect()
}

fn representative_is_gregorian_leap_year(year: u32) -> bool {
    year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400))
}

fn representative_month_days(year: u32) -> [u8; 12] {
    [
        31,
        if representative_is_gregorian_leap_year(year) {
            29
        } else {
            28
        },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ]
}

fn representative_long_run_climate_source(start_year: u32, year_count: usize) -> String {
    use std::fmt::Write as _;

    assert!(year_count > 0);
    let template = complete_season_climate_source_for_days(
        CompleteSeasonClimateProfile::ReappearanceRoutingBgc,
        REPRESENTATIVE_LONG_RUN_YEAR_DAYS,
    );
    let marker = "DAILY UNITS";
    let mut source = template
        .lines()
        .take_while(|line| *line != marker)
        .chain(std::iter::once(marker))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    source = source.replacen(
        "30 2000 1 CLIGEN",
        &format!("30 {start_year} {year_count} CLIGEN"),
        1,
    );
    let payloads = template
        .lines()
        .skip_while(|line| *line != marker)
        .skip(1)
        .map(|line| {
            line.split_whitespace()
                .skip(3)
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>();
    assert_eq!(payloads.len(), REPRESENTATIVE_LONG_RUN_YEAR_DAYS);

    for year_offset in 0..year_count {
        let year = start_year
            .checked_add(u32::try_from(year_offset).expect("year offset fits u32"))
            .expect("representative year range");
        let mut template_day_index = 0_usize;
        for (month_index, day_count) in representative_month_days(year).iter().enumerate() {
            let month = month_index + 1;
            for day in 1..=usize::from(*day_count) {
                let leap_day = month == 2 && day == 29;
                let payload_index = if leap_day {
                    template_day_index
                        .checked_sub(1)
                        .expect("February 29 follows February 28")
                } else {
                    template_day_index
                };
                writeln!(source, "{day} {month} {year} {}", payloads[payload_index])
                    .expect("writing climate rows to a String cannot fail");
                if !leap_day {
                    template_day_index += 1;
                }
            }
        }
        assert_eq!(template_day_index, REPRESENTATIVE_LONG_RUN_YEAR_DAYS);
    }
    source
}

fn representative_daily_rows(source: &str) -> Vec<&str> {
    source
        .lines()
        .skip_while(|line| *line != "DAILY UNITS")
        .skip(1)
        .collect()
}

fn prepare_representative_long_run_fixture(
    prefix: &str,
    start_year: u32,
    year_count: usize,
) -> PathBuf {
    use std::fmt::Write as _;

    let areas_m2 = representative_long_run_areas_m2();
    let run_dir = prepare_native_stage3_lane_d_scale_fixture(prefix, &areas_m2);
    std::fs::write(
        run_dir.join("p102.cli"),
        representative_long_run_climate_source(start_year, year_count),
    )
    .expect("write representative long-run climate");
    let management_path = run_dir.join("case.man.yaml");
    let one_year_management =
        std::fs::read_to_string(&management_path).expect("read representative native management");
    let (management_prefix, _) = one_year_management
        .split_once("schedule:\n")
        .expect("representative native schedule marker");
    let mut management =
        management_prefix.replacen("total_years: 1", &format!("total_years: {year_count}"), 1);
    management.push_str("schedule:\n  ofe_initial_refs:\n");
    for _ in 0..REPRESENTATIVE_LONG_RUN_OFE_COUNT {
        management.push_str("  - 1\n");
    }
    writeln!(management, "  rotation_repeats: {year_count}")
        .expect("write representative rotation repeats");
    management.push_str("  rotation_years: 1\n  slots:\n");
    for rotation_index in 1..=year_count {
        for ofe_index in 1..=REPRESENTATIVE_LONG_RUN_OFE_COUNT {
            writeln!(
                management,
                "  - rotation_index: {rotation_index}\n    year_in_rotation: 1\n    ofe_index: {ofe_index}\n    yearly_refs:\n    - 1"
            )
            .expect("write explicit representative management slot");
        }
    }
    assert_eq!(
        management.matches("    ofe_index:").count(),
        year_count * REPRESENTATIVE_LONG_RUN_OFE_COUNT,
        "one explicit repeated native rotation slot per year and OFE",
    );
    std::fs::write(management_path, management)
        .expect("write repeated representative native management");
    run_dir
}

#[test]
fn representative_ten_ofe_century_recipe_has_exact_calendar_and_forcing_policy() {
    use sha2::{Digest as _, Sha256};

    let recipe = representative_long_run_recipe();
    assert_eq!(
        recipe["ofe_count"].as_u64(),
        Some(REPRESENTATIVE_LONG_RUN_OFE_COUNT as u64),
    );
    let areas_m2 = representative_long_run_areas_m2();
    assert_eq!(areas_m2.len(), REPRESENTATIVE_LONG_RUN_OFE_COUNT);
    assert!(areas_m2.iter().all(|area| area.is_finite() && *area > 0.0));

    let year = representative_long_run_climate_source(2001, 1);
    let year_rows = representative_daily_rows(&year);
    assert_eq!(year_rows.len(), REPRESENTATIVE_LONG_RUN_YEAR_DAYS);
    assert!(year_rows[0].starts_with("1 1 2001 "));
    assert!(year_rows[364].starts_with("31 12 2001 "));

    let century = representative_long_run_climate_source(2000, 100);
    let century_rows = representative_daily_rows(&century);
    assert_eq!(century_rows.len(), REPRESENTATIVE_LONG_RUN_CENTURY_DAYS);
    assert!(century_rows[0].starts_with("1 1 2000 "));
    assert!(century_rows[36_524].starts_with("31 12 2099 "));
    assert_eq!(
        century_rows
            .iter()
            .filter(|line| {
                let mut fields = line.split_whitespace();
                fields.next() == Some("29") && fields.next() == Some("2")
            })
            .count(),
        25,
    );
    assert_eq!(
        recipe["century"]["expected_day_count"].as_u64(),
        Some(REPRESENTATIVE_LONG_RUN_CENTURY_DAYS as u64),
    );
    eprintln!(
        "STAGE3_REPRESENTATIVE_WORKLOAD_RECIPE century_sha256={:x} rows={} ofes={}",
        Sha256::digest(century.as_bytes()),
        century_rows.len(),
        areas_m2.len(),
    );
}

#[test]
fn representative_ten_ofe_century_fixture_parses_exact_days_topology_and_owner() {
    let _execution_guard = runner_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let run_dir =
        prepare_representative_long_run_fixture("stage3_representative_century_intake", 2000, 100);
    let request = HillslopeRunRequest {
        run_dir: run_dir.clone(),
        run_file: PathBuf::from("case.run"),
        output_dir: run_dir.join("output"),
        sidecar_policy: SidecarPolicy::Compat,
        legacy_sidecar_discovery: false,
        manifest_path: None,
    };
    let inputs = load_hillslope_run_inputs(&request).expect("representative century inputs");
    let targets = resolve_hillslope_output_targets(&inputs.runfile)
        .expect("representative century output targets");
    let sidecars = resolve_hillslope_sidecars(&request, &inputs, &targets)
        .expect("representative century sidecars");
    let setup = build_static_hillslope_runtime_setup(
        &request,
        &inputs,
        &sidecars,
        HillslopeRuntimeSelection::DirectProductionExecutor,
    )
    .expect("representative century static setup");
    let HillslopeClimateExecutionState {
        per_ofe_lane_areas_m2,
        per_ofe_runoff_publication_geometries,
        lane_context,
        climate_span,
    } = setup.execution_state;
    assert_eq!(per_ofe_lane_areas_m2, representative_long_run_areas_m2());
    assert_eq!(
        climate_span.days.len(),
        REPRESENTATIVE_LONG_RUN_CENTURY_DAYS
    );
    assert_eq!(
        per_ofe_runoff_publication_geometries.len(),
        REPRESENTATIVE_LONG_RUN_OFE_COUNT
    );
    let climate_request =
        build_hillslope_climate_runtime_request(&inputs.climate).expect("century climate request");
    let seed_authority = DirectProductionSeedAuthority::from_typed_inputs(
        &climate_request,
        &inputs,
        &sidecars,
        REPRESENTATIVE_LONG_RUN_OFE_COUNT,
        lane_context.lane,
    )
    .expect("representative century typed seed authority");
    let frame = build_direct_production_run_frame(&DirectProductionRunFrameBuildInputs {
        output_hillslope_id: targets.output_hillslope_id,
        lane_areas_m2: &per_ofe_lane_areas_m2,
        runoff_publication_geometries: &per_ofe_runoff_publication_geometries,
        day_count: climate_span.days.len(),
        seed_authority: &seed_authority,
    })
    .expect("representative century production frame");
    assert_eq!(frame.identity.lane_count, REPRESENTATIVE_LONG_RUN_OFE_COUNT);
    assert_eq!(
        frame.identity.day_count,
        REPRESENTATIVE_LONG_RUN_CENTURY_DAYS
    );
    let seed_path = crate::hillslope::test_fixture_authority::author_stage3_v11_owner_seed_fixture(
        &request,
        crate::hillslope::test_fixture_authority::Stage3TestFixtureSeedProfile::CompleteOwner,
        crate::hillslope::test_fixture_authority::Stage3TestFixtureSeedBinding::ExplicitRunfile,
    )
    .expect("author exact representative century Stage-3 owner seed");
    assert_authored_stage3_owner_cardinality(
        &request,
        &seed_path,
        REPRESENTATIVE_LONG_RUN_OFE_COUNT,
    );
    std::fs::remove_dir_all(run_dir).expect("remove representative century intake fixture");
}

fn run_representative_long_run(
    prefix: &str,
    start_year: u32,
    year_count: usize,
    expected_day_count: usize,
) {
    let _execution_guard = runner_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let run_dir = prepare_representative_long_run_fixture(prefix, start_year, year_count);
    let request = HillslopeRunRequest {
        run_dir: run_dir.clone(),
        run_file: PathBuf::from("case.run"),
        output_dir: run_dir.join("output"),
        sidecar_policy: SidecarPolicy::Compat,
        legacy_sidecar_discovery: false,
        manifest_path: None,
    };
    let seed_path = crate::hillslope::test_fixture_authority::author_stage3_v11_owner_seed_fixture(
        &request,
        crate::hillslope::test_fixture_authority::Stage3TestFixtureSeedProfile::CompleteOwner,
        crate::hillslope::test_fixture_authority::Stage3TestFixtureSeedBinding::ExplicitRunfile,
    )
    .expect("author exact representative Stage-3 owner seed");
    assert_authored_stage3_owner_cardinality(
        &request,
        &seed_path,
        REPRESENTATIVE_LONG_RUN_OFE_COUNT,
    );
    reset_direct_runtime_audit_counters();
    let started = std::time::Instant::now();
    let report = execute_hillslope_run_with_runtime_policy(
        &request,
        &["openwepp-cli-hill".to_string()],
        HillslopeRuntimeSelectionPolicy::new(
            HillslopeRuntimeSelection::DirectProductionExecutor,
            HillslopeDefaultRuntimeActivation::default(),
        ),
    )
    .expect("representative Stage-3/native/Lane-D run");
    let elapsed = started.elapsed();
    let manifest: serde_json::Value = serde_json::from_slice(
        &std::fs::read(&report.manifest_path).expect("read representative manifest"),
    )
    .expect("parse representative manifest");
    let archive = &manifest["stage3_evidence_archive"];
    assert_eq!(
        archive["record_count"].as_u64(),
        Some(expected_day_count as u64)
    );
    assert_eq!(
        archive["archived_day_count"].as_u64(),
        Some(expected_day_count as u64),
    );
    let active = &manifest["execution_provenance"]["laned_active"];
    assert_eq!(
        active["days_seen"].as_u64(),
        Some(expected_day_count as u64)
    );
    assert_eq!(
        active["days_routed"].as_u64(),
        Some(expected_day_count as u64)
    );
    assert_relative_close(
        positive_manifest_f64(active, "total_source_m3")
            + nonnegative_manifest_f64(active, "total_clamp_m3"),
        positive_manifest_f64(active, "total_routed_outlet_m3")
            + nonnegative_manifest_f64(active, "total_end_window_storage_m3"),
        REPRESENTATIVE_PUBLICATION_REL_TOLERANCE,
        "representative Lane-D source plus clamp equals outlet plus storage",
    );
    let ofe_days = expected_day_count * REPRESENTATIVE_LONG_RUN_OFE_COUNT;
    println!(
        "STAGE3_REPRESENTATIVE_LONG_RUN {}",
        serde_json::json!({
            "start_year": start_year,
            "year_count": year_count,
            "day_count": expected_day_count,
            "ofe_count": REPRESENTATIVE_LONG_RUN_OFE_COUNT,
            "ofe_days": ofe_days,
            "wall_us": release_probe_elapsed_us(elapsed),
            "wall_us_per_ofe_day": elapsed.as_secs_f64() * 1_000_000.0
                / release_scale_usize_f64(ofe_days),
            "rss_kib": release_probe_rss_kib(),
            "archive_record_count": archive["record_count"],
            "archive_stored_record_bytes": archive["stored_record_bytes"],
        }),
    );
    std::fs::remove_dir_all(run_dir).expect("remove representative long-run fixture");
}

#[test]
#[ignore = "release-only representative 10-OFE complete-year qualification"]
fn representative_ten_ofe_complete_year_real_runner() {
    run_representative_long_run(
        "stage3_representative_ten_ofe_year",
        2001,
        1,
        REPRESENTATIVE_LONG_RUN_YEAR_DAYS,
    );
}

#[test]
#[ignore = "release-only representative 10-OFE 100-year qualification"]
fn representative_ten_ofe_hundred_year_real_runner() {
    run_representative_long_run(
        "stage3_representative_ten_ofe_century",
        2000,
        100,
        REPRESENTATIVE_LONG_RUN_CENTURY_DAYS,
    );
}

const RELEASE_SCALE_WARMUP_BATCHES: usize = 5;
const RELEASE_SCALE_MEASURED_BATCHES: usize = 30;
const RELEASE_SCALE_MIN_BATCH_NS: u128 = 250_000_000;
const RELEASE_SCALE_BOOTSTRAP_REPLICATES: usize = 10_000;
const RELEASE_SCALE_BOOTSTRAP_SEED: u64 = 0x4f57_4550_505f_5333;
const RELEASE_SCALE_RSS_RETURN_BASE_TOLERANCE_KIB: u64 = 8 * 1024;
const RELEASE_SCALE_RSS_RETURN_PER_OFE_TOLERANCE_KIB: u64 = 1024;
const RELEASE_SCALE_CHILD_OFE_ENV: &str = "OPENWEPP_STAGE3_RELEASE_SCALE_CHILD_OFE";

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct ReleaseScaleCountersV1 {
    parent_supports: u64,
    covered_supports: u64,
    snow_free_supports: u64,
    accepted_publications: u64,
    legacy_unwired_fixed_point_evaluations: u64,
    direct_trials: u64,
    split_child_trials: u64,
    accepted_microsteps: u64,
    rejected_candidates: u64,
    phase_rejections: u64,
    event_rejections: u64,
    phase_and_event_rejections: u64,
    other_rejections: u64,
}

#[derive(Debug)]
struct ReleaseScaleIterationV1 {
    wall_ns: u128,
    cpu_ticks: Option<u64>,
    input_sha256: String,
    input_file_sha256: std::collections::BTreeMap<String, String>,
    result_sha256: String,
    result_file_sha256: std::collections::BTreeMap<String, String>,
    rss_before_kib: Option<u64>,
    rss_after_kib: Option<u64>,
    active_rss_peak_kib: Option<u64>,
    active_rss_sample_count: u64,
    peak_rss_kib: Option<u64>,
    counters: ReleaseScaleCountersV1,
    accepted_width_histogram_ns: std::collections::BTreeMap<u128, u64>,
    parent_telemetry: Vec<serde_json::Value>,
    available_attribution_ns: std::collections::BTreeMap<&'static str, u128>,
    qualification_telemetry:
        openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::ReleaseQualificationTelemetryV1,
    lane_d_evidence: serde_json::Value,
    closure_evidence: serde_json::Value,
}

struct ReleaseScaleActiveRssSamplerV1 {
    stop: std::sync::Arc<std::sync::atomic::AtomicBool>,
    peak_kib: std::sync::Arc<std::sync::atomic::AtomicU64>,
    sample_count: std::sync::Arc<std::sync::atomic::AtomicU64>,
    handle: std::thread::JoinHandle<()>,
}

impl ReleaseScaleActiveRssSamplerV1 {
    fn start() -> Self {
        use std::sync::atomic::Ordering;

        let stop = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let peak_kib = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
        let sample_count = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
        let thread_stop = std::sync::Arc::clone(&stop);
        let thread_peak = std::sync::Arc::clone(&peak_kib);
        let thread_count = std::sync::Arc::clone(&sample_count);
        let handle = std::thread::spawn(move || {
            while !thread_stop.load(Ordering::Acquire) {
                if let Some(rss_kib) = release_scale_current_rss_kib() {
                    thread_peak.fetch_max(rss_kib, Ordering::AcqRel);
                    thread_count.fetch_add(1, Ordering::AcqRel);
                }
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
        });
        Self {
            stop,
            peak_kib,
            sample_count,
            handle,
        }
    }

    fn finish(self) -> (Option<u64>, u64) {
        use std::sync::atomic::Ordering;

        if let Some(rss_kib) = release_scale_current_rss_kib() {
            self.peak_kib.fetch_max(rss_kib, Ordering::AcqRel);
            self.sample_count.fetch_add(1, Ordering::AcqRel);
        }
        self.stop.store(true, Ordering::Release);
        self.handle.join().expect("active RSS sampler thread");
        let sample_count = self.sample_count.load(Ordering::Acquire);
        let peak_kib = self.peak_kib.load(Ordering::Acquire);
        ((sample_count > 0).then_some(peak_kib), sample_count)
    }
}

#[derive(Clone, Copy)]
enum ReleaseScaleStatistic {
    Median,
    P95,
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    reason = "the probability is checked and qualification batches contain exactly 30 values"
)]
fn release_scale_quantile(mut values: Vec<f64>, probability: f64) -> f64 {
    assert!(!values.is_empty());
    assert!((0.0..=1.0).contains(&probability));
    values.sort_by(f64::total_cmp);
    let position = probability * (values.len() - 1) as f64;
    let lower = position.floor() as usize;
    let upper = position.ceil() as usize;
    values[lower] + (values[upper] - values[lower]) * (position - lower as f64)
}

fn release_scale_statistic(values: &[f64], statistic: ReleaseScaleStatistic) -> f64 {
    release_scale_quantile(
        values.to_vec(),
        match statistic {
            ReleaseScaleStatistic::Median => 0.5,
            ReleaseScaleStatistic::P95 => 0.95,
        },
    )
}

fn release_scale_splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut value = *state;
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

fn release_scale_bootstrap_ci(
    values: &[f64],
    statistic: ReleaseScaleStatistic,
    seed: u64,
    replicates: usize,
) -> (f64, f64) {
    assert!(!values.is_empty());
    assert!(replicates > 0);
    let mut state = seed;
    let mut estimates = Vec::with_capacity(replicates);
    for _ in 0..replicates {
        let sample = (0..values.len())
            .map(|_| {
                let bound = u64::try_from(values.len()).expect("bootstrap sample length fits u64");
                let index = usize::try_from(release_scale_splitmix64(&mut state) % bound)
                    .expect("bootstrap index is below the source sample length");
                values[index]
            })
            .collect::<Vec<_>>();
        estimates.push(release_scale_statistic(&sample, statistic));
    }
    (
        release_scale_quantile(estimates.clone(), 0.025),
        release_scale_quantile(estimates, 0.975),
    )
}

fn release_scale_sha256_file(path: &Path) -> String {
    use sha2::{Digest as _, Sha256};
    format!(
        "{:x}",
        Sha256::digest(std::fs::read(path).expect("read qualification hash input"))
    )
}

fn release_scale_tree_hashes(root: &Path) -> (String, std::collections::BTreeMap<String, String>) {
    use sha2::{Digest as _, Sha256};
    fn collect(root: &Path, dir: &Path, paths: &mut Vec<PathBuf>) {
        let mut entries = std::fs::read_dir(dir)
            .expect("read qualification input directory")
            .map(|entry| entry.expect("read qualification input entry").path())
            .collect::<Vec<_>>();
        entries.sort();
        for path in entries {
            if path.file_name().is_some_and(|name| name == "output") {
                continue;
            }
            if path.is_dir() {
                collect(root, &path, paths);
            } else {
                paths.push(
                    path.strip_prefix(root)
                        .expect("input below root")
                        .to_path_buf(),
                );
            }
        }
    }
    let mut paths = Vec::new();
    collect(root, root, &mut paths);
    let mut digest = Sha256::new();
    let mut file_hashes = std::collections::BTreeMap::new();
    for relative in paths {
        let bytes = std::fs::read(root.join(&relative)).expect("read qualification input file");
        digest.update(relative.to_string_lossy().as_bytes());
        digest.update([0]);
        digest.update((bytes.len() as u64).to_le_bytes());
        digest.update(&bytes);
        file_hashes.insert(
            relative.to_string_lossy().into_owned(),
            format!("{:x}", Sha256::digest(bytes)),
        );
    }
    (format!("{:x}", digest.finalize()), file_hashes)
}

fn release_scale_source_identity() -> serde_json::Value {
    use sha2::{Digest as _, Sha256};
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let head = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(&root)
        .output()
        .expect("run git rev-parse for qualification");
    assert!(
        head.status.success(),
        "qualification requires a Git source identity"
    );
    let listed = std::process::Command::new("git")
        .args([
            "ls-files",
            "--cached",
            "--others",
            "--exclude-standard",
            "-z",
        ])
        .current_dir(&root)
        .output()
        .expect("list exact qualification source tree");
    assert!(
        listed.status.success(),
        "qualification requires an exact source listing"
    );
    let mut paths = listed
        .stdout
        .split(|byte| *byte == 0)
        .filter(|path| !path.is_empty())
        .map(|path| PathBuf::from(std::str::from_utf8(path).expect("UTF-8 repository path")))
        .collect::<Vec<_>>();
    paths.sort();
    let mut digest = Sha256::new();
    for relative in paths {
        digest.update(relative.to_string_lossy().as_bytes());
        digest.update([0]);
        match std::fs::read(root.join(&relative)) {
            Ok(bytes) => {
                digest.update((bytes.len() as u64).to_le_bytes());
                digest.update(bytes);
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => digest.update(b"DELETED"),
            Err(error) => panic!("read qualification source {}: {error}", relative.display()),
        }
    }
    serde_json::json!({
        "git_head": String::from_utf8(head.stdout).expect("ASCII Git head").trim(),
        "working_tree_sha256": format!("{:x}", digest.finalize()),
    })
}

fn release_scale_process_cpu_ticks() -> Option<u64> {
    let stat = std::fs::read_to_string("/proc/self/stat").ok()?;
    release_scale_process_cpu_ticks_from_stat(&stat)
}

fn release_scale_process_cpu_ticks_from_stat(stat: &str) -> Option<u64> {
    let tail = stat.rsplit_once(") ")?.1;
    let fields = tail.split_whitespace().collect::<Vec<_>>();
    let user: u64 = fields.get(11)?.parse().ok()?;
    let system: u64 = fields.get(12)?.parse().ok()?;
    user.checked_add(system)
}

fn release_scale_clock_tick_hz() -> Option<u64> {
    let output = std::process::Command::new("getconf")
        .arg("CLK_TCK")
        .output()
        .ok()?;
    output.status.success().then_some(())?;
    String::from_utf8(output.stdout).ok()?.trim().parse().ok()
}

fn release_scale_status_kib(status: &str, field: &str) -> Option<u64> {
    status.lines().find_map(|line| {
        let value = line.strip_prefix(field)?.split_whitespace().next()?;
        value.parse().ok()
    })
}

fn release_scale_current_rss_kib() -> Option<u64> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    release_scale_status_kib(&status, "VmRSS:")
}

fn release_scale_peak_rss_kib() -> Option<u64> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    release_scale_status_kib(&status, "VmHWM:")
}

fn release_scale_cpu_affinity() -> Option<String> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    status.lines().find_map(|line| {
        line.strip_prefix("Cpus_allowed_list:")
            .map(str::trim)
            .map(str::to_owned)
    })
}

fn release_scale_cpu_affinity_count(affinity: &str) -> Option<usize> {
    affinity.split(',').try_fold(0_usize, |total, component| {
        let (start, end) = component
            .split_once('-')
            .map_or((component, component), |range| range);
        let start: usize = start.parse().ok()?;
        let end: usize = end.parse().ok()?;
        end.checked_sub(start)
            .and_then(|width| width.checked_add(1))
            .and_then(|count| total.checked_add(count))
    })
}

fn release_scale_usize_f64(value: usize) -> f64 {
    f64::from(u32::try_from(value).expect("qualification count fits u32"))
}

fn release_scale_duration_ns_f64(nanoseconds: u128) -> f64 {
    std::time::Duration::from_nanos(
        u64::try_from(nanoseconds).expect("qualification batch duration fits u64 nanoseconds"),
    )
    .as_secs_f64()
        * 1.0e9
}

fn release_scale_cpu_ns_f64(ticks: u64, hz: u64) -> f64 {
    assert!(hz > 0);
    let whole = std::time::Duration::from_secs(ticks / hz).as_secs_f64() * 1.0e9;
    let remainder = f64::from(u32::try_from(ticks % hz).expect("remainder is below CLK_TCK"));
    let hz = f64::from(u32::try_from(hz).expect("CLK_TCK fits u32"));
    whole + remainder * 1.0e9 / hz
}

fn release_scale_rss_return_tolerance_kib(ofe_count: usize) -> u64 {
    RELEASE_SCALE_RSS_RETURN_BASE_TOLERANCE_KIB
        + RELEASE_SCALE_RSS_RETURN_PER_OFE_TOLERANCE_KIB
            * u64::try_from(ofe_count).expect("qualification OFE count fits u64")
}

fn release_scale_rss_returned(
    before_kib: Option<u64>,
    after_kib: Option<u64>,
    tolerance_kib: u64,
) -> Option<bool> {
    Some(after_kib? <= before_kib?.checked_add(tolerance_kib)?)
}

fn release_scale_hard_peak_live_rss_kib(
    sampled_active_peak_kib: Option<u64>,
    isolated_process_hwm_kib: Option<u64>,
) -> Option<u64> {
    Some(sampled_active_peak_kib?.max(isolated_process_hwm_kib?))
}

fn release_scale_parent_telemetry(
    telemetry: &[openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::AdaptiveParentTelemetryV1],
) -> (
    Vec<serde_json::Value>,
    std::collections::BTreeMap<u128, u64>,
    std::collections::BTreeMap<&'static str, u128>,
) {
    let mut widths = std::collections::BTreeMap::<u128, u64>::new();
    let mut attribution = std::collections::BTreeMap::<&'static str, u128>::new();
    let mut rows = Vec::with_capacity(telemetry.len());
    let mut add_attribution = |name: &'static str, elapsed: std::time::Duration| {
        let destination = attribution.entry(name).or_default();
        *destination = destination.saturating_add(elapsed.as_nanos());
    };
    for row in telemetry {
        for (width_ns, count) in &row.accepted_width_histogram {
            let destination = widths.entry(*width_ns).or_default();
            *destination = destination.saturating_add(*count);
        }
        add_attribution("parent_total", row.parent_elapsed);
        add_attribution(
            "covered_direct_trials",
            row.covered_direct_trial_phase_elapsed,
        );
        add_attribution(
            "covered_composed_trials",
            row.covered_composed_trial_phase_elapsed,
        );
        add_attribution(
            "terminal_direct_trials",
            row.terminal_direct_trial_phase_elapsed,
        );
        add_attribution(
            "terminal_composed_trials",
            row.terminal_composed_trial_phase_elapsed,
        );
        add_attribution("covered_map_operands", row.fixed_point_operand_elapsed);
        add_attribution("covered_map_envelope", row.fixed_point_envelope_elapsed);
        add_attribution("covered_stage3", row.fixed_point_stage3_elapsed);
        add_attribution("covered_soil", row.fixed_point_soil_elapsed);
        add_attribution("covered_finalization", row.fixed_point_finalization_elapsed);
        add_attribution("publication_append", row.publication_append_elapsed);
        add_attribution(
            "publication_full_validation",
            row.publication_full_validation_elapsed,
        );
        add_attribution("reuse_validation", row.reuse_validation_elapsed);
        rows.push(serde_json::json!({
            "parent_ordinal": row.parent_ordinal,
            "support_start_ns": row.support.start_ns().get(),
            "support_end_ns": row.support.end_ns().get(),
            "direct_trial_count": row.direct_trial_count,
            "split_child_trial_count": row.split_child_trial_count,
            "accepted_microstep_count": row.accepted_microstep_count,
            "rejected_candidate_count": row.rejected_candidate_count,
            "rejection_categories": {
                "phase": row.phase_rejection_count,
                "event": row.event_rejection_count,
                "phase_and_event": row.phase_and_event_rejection_count,
                "other": row.other_rejection_count,
            },
            "accepted_width_histogram_ns": row.accepted_width_histogram,
            "physical_map_evaluation_count": serde_json::Value::Null,
            "physical_map_evaluation_status": "not duplicated in overlapping parent telemetry; authentic per-accepted-support records are emitted in accepted_covered_physical_map_records",
            "adaptive_receipt_bytes": row.adaptive_receipt_bytes,
            "owner_join_count": row.owner_join_count,
            "event_group_count": row.event_group_count,
            "terminal_parcel_count": row.terminal_parcel_count,
            "publication_support_count": row.publication_support_count,
            "publication_event_count": row.publication_event_count,
        }));
    }
    (rows, widths, attribution)
}

fn release_scale_qualification_profile(
    telemetry: &openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::ReleaseQualificationTelemetryV1,
) -> serde_json::Value {
    let exclusive_sum = telemetry
        .native_vegetation_et_elapsed
        .saturating_add(telemetry.stage3_lse_soil_elapsed)
        .saturating_add(telemetry.lane_d_elapsed)
        .saturating_add(telemetry.remaining_runner_elapsed);
    serde_json::json!({
        "complete": telemetry.scopes_balanced && telemetry.counters_complete && exclusive_sum == telemetry.total_elapsed,
        "total_ns": release_scale_duration_ns_u64(telemetry.total_elapsed),
        "native_vegetation_et_successor_envelope_ns": release_scale_duration_ns_u64(telemetry.native_vegetation_et_elapsed),
        "stage3_day_preparation_remainder_envelope_ns": release_scale_duration_ns_u64(telemetry.stage3_lse_soil_elapsed),
        "lane_d_ns": release_scale_duration_ns_u64(telemetry.lane_d_elapsed),
        "remaining_runner_ns": release_scale_duration_ns_u64(telemetry.remaining_runner_elapsed),
        "exclusive_sum_ns": release_scale_duration_ns_u64(exclusive_sum),
        "exclusive_sum_matches_total": exclusive_sum == telemetry.total_elapsed,
        "scopes_balanced": telemetry.scopes_balanced,
        "counters_complete": telemetry.counters_complete,
        "stage3_scope_entry_count": telemetry.stage3_scope_entry_count,
        "native_vegetation_et_scope_entry_count": telemetry.native_vegetation_et_scope_entry_count,
        "lane_d_scope_entry_count": telemetry.lane_d_scope_entry_count,
        "attribution_semantics": {
            "native_vegetation_et": "whole snow-free native vegetation/ET successor transaction envelope; conservative upper bound, not isolated component time",
            "stage3_lse_soil": "remaining Stage3 day-preparation/LSE/soil transaction envelope after nested scopes; conservative upper bound, not isolated component time",
        },
    })
}

fn release_scale_duration_ns_u64(duration: std::time::Duration) -> u64 {
    u64::try_from(duration.as_nanos()).expect("qualification duration fits u64 nanoseconds")
}

fn release_scale_accepted_covered_map_records(
    telemetry: &openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::ReleaseQualificationTelemetryV1,
) -> Vec<serde_json::Value> {
    telemetry
        .accepted_covered_maps
        .iter()
        .enumerate()
        .map(|(accepted_support_ordinal, record)| {
            serde_json::json!({
                "accepted_support_ordinal": accepted_support_ordinal,
                "support_start_ns": record.support.start_ns().get(),
                "support_end_ns": record.support.end_ns().get(),
                "support_width_ns": record.support.duration_ns(),
                "physical_map_evaluation_count": record.physical_map_evaluation_count,
            })
        })
        .collect()
}

fn release_scale_canonical_covered_width_histogram_ns(
    telemetry: &openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::ReleaseQualificationTelemetryV1,
) -> std::collections::BTreeMap<u128, u64> {
    let mut histogram = std::collections::BTreeMap::<u128, u64>::new();
    for record in &telemetry.accepted_covered_maps {
        let destination = histogram.entry(record.support.duration_ns()).or_default();
        *destination = destination
            .checked_add(1)
            .expect("covered-map support histogram count");
    }
    histogram
}

#[allow(
    clippy::too_many_lines,
    reason = "one helper keeps the timed runner transaction and its complete post-run custody audit adjacent"
)]
fn release_scale_run_iteration(ofe_count: usize, label: &str) -> ReleaseScaleIterationV1 {
    let rss_before_kib = release_scale_current_rss_kib();
    let active_rss_sampler = ReleaseScaleActiveRssSamplerV1::start();
    let areas_m2 = (1..=ofe_count)
        .map(|ofe| 100.0 * release_scale_usize_f64(ofe))
        .collect::<Vec<_>>();
    let run_dir = prepare_native_stage3_lane_d_scale_fixture(label, &areas_m2);
    let request = HillslopeRunRequest {
        run_dir: run_dir.clone(),
        run_file: PathBuf::from("case.run"),
        output_dir: run_dir.join("output"),
        sidecar_policy: SidecarPolicy::Compat,
        legacy_sidecar_discovery: false,
        manifest_path: None,
    };
    let seed_path = crate::hillslope::test_fixture_authority::author_stage3_v11_owner_seed_fixture(
        &request,
        crate::hillslope::test_fixture_authority::Stage3TestFixtureSeedProfile::CompleteOwner,
        crate::hillslope::test_fixture_authority::Stage3TestFixtureSeedBinding::ExplicitRunfile,
    )
    .expect("author release qualification Stage-3 owner seed");
    assert_authored_stage3_owner_cardinality(&request, &seed_path, ofe_count);
    let (input_sha256, input_file_sha256) = release_scale_tree_hashes(&run_dir);
    reset_direct_runtime_audit_counters();
    crate::hillslope::snow_stage3_v11_qualification_audit::begin();
    let adaptive_telemetry_guard = openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::begin_adaptive_parent_telemetry_v1(
        openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::STAGE3_V11_PARENT_SUPPORT_COUNT
            * ofe_count,
        std::time::Duration::from_secs(3_600),
    )
    .expect("enable bounded qualification telemetry");
    let cpu_before = release_scale_process_cpu_ticks();
    let qualification_telemetry_guard = openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::begin_release_qualification_telemetry_v1()
        .expect("enable release qualification telemetry");
    let report = execute_hillslope_run_with_runtime_policy(
        &request,
        &["openwepp-cli-hill".to_string()],
        HillslopeRuntimeSelectionPolicy::new(
            HillslopeRuntimeSelection::DirectProductionExecutor,
            HillslopeDefaultRuntimeActivation::default(),
        ),
    )
    .expect("release qualification iteration must complete");
    let qualification_telemetry = openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::take_release_qualification_telemetry_v1()
        .expect("take release qualification telemetry immediately after the runner wall scope");
    let wall_ns = qualification_telemetry.total_elapsed.as_nanos();
    drop(qualification_telemetry_guard);
    let cpu_after = release_scale_process_cpu_ticks();
    let (active_rss_peak_kib, active_rss_sample_count) = active_rss_sampler.finish();
    let peak_rss_kib = release_scale_peak_rss_kib();
    let audit = crate::hillslope::snow_stage3_v11_qualification_audit::take();
    let telemetry = openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::take_adaptive_parent_telemetry_v1();
    drop(adaptive_telemetry_guard);
    let snapshot = audit
        .committed_snapshot
        .as_ref()
        .expect("qualification committed Stage-3 snapshot");
    snapshot
        .validate()
        .expect("qualification snapshot validation");
    assert_eq!(snapshot.committed_day_count, 1);
    assert_eq!(snapshot.lanes.len(), ofe_count);
    assert_eq!(
        audit.support_chronology_by_day.get(&0).expect("day-zero chronology").len(),
        openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::STAGE3_V11_PARENT_SUPPORT_COUNT,
    );
    let manifest: serde_json::Value = serde_json::from_slice(
        &std::fs::read(&report.manifest_path).expect("read qualification manifest"),
    )
    .expect("parse qualification manifest");
    let active = &manifest["execution_provenance"]["laned_active"];
    assert_eq!(active["days_seen"].as_u64(), Some(1));
    assert_eq!(active["days_routed"].as_u64(), Some(1));
    let routed_day_count = active["days_routed"]
        .as_u64()
        .expect("Lane-D routed day count");
    let hbp_bytes = std::fs::read(&report.output_pass).expect("read qualification HBP");
    let (hbp, latest_event) = parse_hbp_from_bytes_with_latest_event_payload(
        &hbp_bytes,
        &report.output_pass,
        HbpParseOptions {
            expected_hillslope_id: Some(83),
        },
    )
    .expect("parse qualification HBP");
    assert_eq!(usize::from(hbp.nofe), ofe_count);
    let event = latest_event.expect("qualification must publish a routed event");
    let wat_path = run_dir.join("output/H83.wat.parquet");
    let published_ofe_order = read_wat_i16_column(&wat_path, "OFE");
    assert_eq!(
        published_ofe_order,
        (1..=ofe_count)
            .map(|ofe| i16::try_from(ofe).expect("qualification OFE index fits i16"))
            .collect::<Vec<_>>(),
    );
    let evidence = RealLaneDPublicEvidence {
        ofe_count,
        areas_m2: read_wat_f64_column(&wat_path, "Area"),
        source_depths_mm: read_wat5_hourly_sources(
            &run_dir.join("output/H83.wat-subhourly.parquet"),
            ofe_count,
        ),
        pass_runvol_m3: read_wat_f64_column(&run_dir.join("output/H83.pass.parquet"), "runvol"),
        pass_peakro_m3_s: read_wat_f64_column(&run_dir.join("output/H83.pass.parquet"), "peakro"),
        hbp_hourly_outlet_m3: event.hourly_runoff_volume_m3,
        hbp_peak_outlet_m3_s: event.peak_runoff_m3_s,
        manifest_source_m3: positive_manifest_f64(active, "total_source_m3"),
        manifest_outlet_m3: positive_manifest_f64(active, "total_routed_outlet_m3"),
        manifest_storage_m3: nonnegative_manifest_f64(active, "total_end_window_storage_m3"),
        manifest_clamp_m3: nonnegative_manifest_f64(active, "total_clamp_m3"),
    };
    assert_real_lane_d_public_closure(&evidence, &areas_m2);
    assert_routed_lane_d_books_reach_public_outputs(
        &run_dir,
        &report,
        &wat_path,
        ofe_count,
    );
    let (result_sha256, result_file_sha256) =
        release_scale_scientific_output_hashes(&request.output_dir);
    let reconstructed_source_m3 = evidence
        .source_depths_mm
        .iter()
        .zip(&evidence.areas_m2)
        .map(|(depths_mm, area_m2)| depths_mm.iter().sum::<f64>() / 1_000.0 * area_m2)
        .sum::<f64>();
    let hbp_outlet_m3 = evidence.hbp_hourly_outlet_m3.iter().sum::<f64>();
    let reconstructed_peak_m3_s = evidence
        .hbp_hourly_outlet_m3
        .iter()
        .copied()
        .fold(0.0_f64, f64::max)
        / 3_600.0;
    let pass_outlet_m3 = *evidence
        .pass_runvol_m3
        .last()
        .expect("terminal PASS volume");
    let pass_peak_m3_s = *evidence
        .pass_peakro_m3_s
        .last()
        .expect("terminal PASS peak");
    let closure_evidence = serde_json::json!({
        "reconstructed_source_m3": reconstructed_source_m3,
        "manifest_source_m3": evidence.manifest_source_m3,
        "source_residual_m3": reconstructed_source_m3 - evidence.manifest_source_m3,
        "manifest_clamp_m3": evidence.manifest_clamp_m3,
        "manifest_outlet_m3": evidence.manifest_outlet_m3,
        "manifest_end_window_storage_m3": evidence.manifest_storage_m3,
        "source_plus_clamp_minus_outlet_minus_storage_m3": evidence.manifest_source_m3
            + evidence.manifest_clamp_m3 - evidence.manifest_outlet_m3
            - evidence.manifest_storage_m3,
        "hbp_outlet_m3": hbp_outlet_m3,
        "hbp_minus_manifest_outlet_m3": hbp_outlet_m3 - evidence.manifest_outlet_m3,
        "pass_outlet_m3": pass_outlet_m3,
        "pass_minus_manifest_outlet_m3": pass_outlet_m3 - evidence.manifest_outlet_m3,
        "reconstructed_peak_m3_s": reconstructed_peak_m3_s,
        "hbp_peak_m3_s": evidence.hbp_peak_outlet_m3_s,
        "pass_peak_m3_s": pass_peak_m3_s,
        "hbp_peak_residual_m3_s": reconstructed_peak_m3_s - evidence.hbp_peak_outlet_m3_s,
        "pass_peak_residual_m3_s": reconstructed_peak_m3_s - pass_peak_m3_s,
    });
    let internal_route_call_order = qualification_telemetry
        .lane_d_route_calls
        .iter()
        .map(|call| call.lane_index + 1)
        .collect::<Vec<_>>();
    let internal_route_calls = qualification_telemetry
        .lane_d_route_calls
        .iter()
        .map(|call| {
            serde_json::json!({
                "call_ordinal": call.call_ordinal,
                "day_index": call.day_index,
                "lane_index_zero_based": call.lane_index,
                "ofe_index_one_based": call.lane_index + 1,
            })
        })
        .collect::<Vec<_>>();
    let mut upstream_route_active = false;
    let expected_internal_route_call_order_from_sources = evidence
        .source_depths_mm
        .iter()
        .enumerate()
        .filter_map(|(lane_index, source_depths_mm)| {
            let local_source_positive = source_depths_mm.iter().any(|depth_mm| *depth_mm > 0.0);
            let route_call_required = local_source_positive || upstream_route_active;
            upstream_route_active |= route_call_required;
            route_call_required.then_some(lane_index + 1)
        })
        .collect::<Vec<_>>();
    let internal_route_calls_required_from_source_posture =
        !expected_internal_route_call_order_from_sources.is_empty();
    let lane_d_evidence = serde_json::json!({
        "routed_day_count": routed_day_count,
        "published_upstream_to_downstream_ofe_order": published_ofe_order,
        "internal_per_ofe_route_call_count": qualification_telemetry.lane_d_route_calls.len(),
        "internal_per_ofe_route_call_order": internal_route_call_order,
        "internal_per_ofe_route_calls": internal_route_calls,
        "expected_internal_route_call_count_from_source_posture": expected_internal_route_call_order_from_sources.len(),
        "expected_internal_route_call_order_from_source_posture": expected_internal_route_call_order_from_sources,
        "internal_route_calls_required_from_source_posture": internal_route_calls_required_from_source_posture,
        "internal_per_ofe_route_call_count_and_order_status": if qualification_telemetry.counters_complete {
            "measured from successful internal Lane-D route calls"
        } else {
            "incomplete: release qualification counter overflowed"
        },
    });
    let (parent_telemetry, accepted_width_histogram_ns, available_attribution_ns) =
        release_scale_parent_telemetry(&telemetry);
    let rejected_candidates = telemetry
        .iter()
        .map(|row| row.rejected_candidate_count)
        .sum::<u64>();
    let phase_rejections = telemetry.iter().map(|row| row.phase_rejection_count).sum();
    let event_rejections = telemetry.iter().map(|row| row.event_rejection_count).sum();
    let phase_and_event_rejections = telemetry
        .iter()
        .map(|row| row.phase_and_event_rejection_count)
        .sum();
    let other_rejections = telemetry.iter().map(|row| row.other_rejection_count).sum();
    assert_eq!(
        phase_rejections + event_rejections - phase_and_event_rejections + other_rejections,
        rejected_candidates,
        "rejection categories must reconcile to the observed candidate count"
    );
    let counters = ReleaseScaleCountersV1 {
        parent_supports: snapshot.total_parent_support_count,
        covered_supports: snapshot.adaptive_support_receipt_count,
        snow_free_supports: snapshot.snow_free_parent_support_count,
        accepted_publications: snapshot.accepted_publication_support_count,
        legacy_unwired_fixed_point_evaluations: telemetry
            .iter()
            .map(|row| row.fixed_point_evaluation_count)
            .sum(),
        direct_trials: telemetry.iter().map(|row| row.direct_trial_count).sum(),
        split_child_trials: telemetry
            .iter()
            .map(|row| row.split_child_trial_count)
            .sum(),
        accepted_microsteps: telemetry
            .iter()
            .map(|row| row.accepted_microstep_count)
            .sum(),
        rejected_candidates,
        phase_rejections,
        event_rejections,
        phase_and_event_rejections,
        other_rejections,
    };
    std::fs::remove_dir_all(run_dir).expect("remove release qualification fixture");
    drop(evidence);
    drop(hbp_bytes);
    drop(manifest);
    drop(audit);
    drop(telemetry);
    drop(report);
    let rss_after_kib = release_scale_current_rss_kib();
    ReleaseScaleIterationV1 {
        wall_ns,
        cpu_ticks: cpu_before
            .zip(cpu_after)
            .and_then(|(before, after)| after.checked_sub(before)),
        input_sha256,
        input_file_sha256,
        result_sha256,
        result_file_sha256,
        rss_before_kib,
        rss_after_kib,
        active_rss_peak_kib,
        active_rss_sample_count,
        peak_rss_kib,
        counters,
        accepted_width_histogram_ns,
        parent_telemetry,
        available_attribution_ns,
        qualification_telemetry,
        lane_d_evidence,
        closure_evidence,
    }
}

fn release_scale_iteration_comparability_identity(
    iteration: &ReleaseScaleIterationV1,
) -> serde_json::Value {
    let physical_map_evaluations = iteration
        .qualification_telemetry
        .accepted_covered_maps
        .iter()
        .map(|record| record.physical_map_evaluation_count)
        .collect::<Vec<_>>();
    serde_json::json!({
        "input_sha256": iteration.input_sha256,
        "input_files_sha256": iteration.input_file_sha256,
        "result_sha256": iteration.result_sha256,
        "result_files_sha256": iteration.result_file_sha256,
        "counters_per_run": iteration.counters,
        "accepted_width_histogram_ns_per_run": iteration.accepted_width_histogram_ns,
        "accepted_covered_physical_map_records_per_run": release_scale_accepted_covered_map_records(&iteration.qualification_telemetry),
        "physical_map_evaluations_per_covered_support": physical_map_evaluations,
        "canonical_covered_width_histogram_ns_per_run": release_scale_canonical_covered_width_histogram_ns(&iteration.qualification_telemetry),
        "release_qualification_counter_identity": {
            "scopes_balanced": iteration.qualification_telemetry.scopes_balanced,
            "counters_complete": iteration.qualification_telemetry.counters_complete,
            "stage3_scope_entry_count": iteration.qualification_telemetry.stage3_scope_entry_count,
            "native_vegetation_et_scope_entry_count": iteration.qualification_telemetry.native_vegetation_et_scope_entry_count,
            "lane_d_scope_entry_count": iteration.qualification_telemetry.lane_d_scope_entry_count,
            "required_scope_observation_complete": release_scale_required_scope_observation_complete(iteration),
        },
        "lane_d_result_identity": iteration.lane_d_evidence,
        "closure_result_identity": iteration.closure_evidence,
    })
}

fn release_scale_required_scope_observation_complete(iteration: &ReleaseScaleIterationV1) -> bool {
    let telemetry = &iteration.qualification_telemetry;
    telemetry.scopes_balanced
        && telemetry.counters_complete
        && release_scale_required_scope_counts_complete(
            telemetry.stage3_scope_entry_count,
            telemetry.native_vegetation_et_scope_entry_count,
            telemetry.lane_d_scope_entry_count,
            iteration.counters.snow_free_supports,
            iteration.lane_d_evidence["internal_route_calls_required_from_source_posture"]
                .as_bool()
                .unwrap_or(true),
        )
}

fn release_scale_required_scope_counts_complete(
    stage3_scope_entry_count: u64,
    native_scope_entry_count: u64,
    lane_d_scope_entry_count: u64,
    snow_free_support_count: u64,
    lane_d_route_calls_required: bool,
) -> bool {
    stage3_scope_entry_count > 0
        && (snow_free_support_count == 0 || native_scope_entry_count > 0)
        && (!lane_d_route_calls_required || lane_d_scope_entry_count > 0)
}

fn release_scale_batch_exclusive_profile(
    iterations: &[ReleaseScaleIterationV1],
) -> serde_json::Value {
    let mut total_ns = 0_u64;
    let mut native_vegetation_et_ns = 0_u64;
    let mut stage3_lse_soil_ns = 0_u64;
    let mut lane_d_ns = 0_u64;
    let mut remaining_runner_ns = 0_u64;
    let mut complete = !iterations.is_empty();
    for iteration in iterations {
        let telemetry = &iteration.qualification_telemetry;
        let profile = release_scale_qualification_profile(telemetry);
        complete &= telemetry.scopes_balanced
            && telemetry.counters_complete
            && release_scale_required_scope_observation_complete(iteration)
            && profile["exclusive_sum_matches_total"].as_bool() == Some(true);
        total_ns = total_ns
            .checked_add(release_scale_duration_ns_u64(telemetry.total_elapsed))
            .expect("batch qualification total duration");
        native_vegetation_et_ns = native_vegetation_et_ns
            .checked_add(release_scale_duration_ns_u64(
                telemetry.native_vegetation_et_elapsed,
            ))
            .expect("batch native vegetation duration");
        stage3_lse_soil_ns = stage3_lse_soil_ns
            .checked_add(release_scale_duration_ns_u64(
                telemetry.stage3_lse_soil_elapsed,
            ))
            .expect("batch Stage-3 duration");
        lane_d_ns = lane_d_ns
            .checked_add(release_scale_duration_ns_u64(telemetry.lane_d_elapsed))
            .expect("batch Lane-D duration");
        remaining_runner_ns = remaining_runner_ns
            .checked_add(release_scale_duration_ns_u64(
                telemetry.remaining_runner_elapsed,
            ))
            .expect("batch remaining-runner duration");
    }
    let exclusive_sum_ns = native_vegetation_et_ns
        .checked_add(stage3_lse_soil_ns)
        .and_then(|sum| sum.checked_add(lane_d_ns))
        .and_then(|sum| sum.checked_add(remaining_runner_ns))
        .expect("batch exclusive profile sum");
    complete &= total_ns > 0 && exclusive_sum_ns == total_ns;
    serde_json::json!({
        "complete": complete,
        "total_ns": total_ns,
        "native_vegetation_et_successor_envelope_ns": native_vegetation_et_ns,
        "stage3_day_preparation_remainder_envelope_ns": stage3_lse_soil_ns,
        "lane_d_ns": lane_d_ns,
        "remaining_runner_ns": remaining_runner_ns,
        "exclusive_sum_ns": exclusive_sum_ns,
        "exclusive_sum_matches_total": exclusive_sum_ns == total_ns,
    })
}

#[allow(
    clippy::too_many_lines,
    reason = "the stable batch JSON schema remains adjacent to its raw iteration aggregation"
)]
fn release_scale_run_batch(
    ofe_count: usize,
    phase: &str,
    batch_index: usize,
    clock_tick_hz: Option<u64>,
) -> serde_json::Value {
    let mut iterations = Vec::new();
    let mut wall_ns = 0_u128;
    while wall_ns < RELEASE_SCALE_MIN_BATCH_NS {
        let iteration = release_scale_run_iteration(
            ofe_count,
            &format!(
                "stage3_release_scale_{ofe_count}_{phase}_{batch_index}_{}",
                iterations.len()
            ),
        );
        wall_ns = wall_ns
            .checked_add(iteration.wall_ns)
            .expect("batch wall duration");
        iterations.push(iteration);
    }
    let input_sha256 = iterations[0].input_sha256.clone();
    let input_file_sha256 = iterations[0].input_file_sha256.clone();
    let counters = iterations[0].counters.clone();
    let accepted_width_histogram_ns = iterations[0].accepted_width_histogram_ns.clone();
    let comparability_identity = release_scale_iteration_comparability_identity(&iterations[0]);
    assert!(
        iterations
            .iter()
            .all(|row| row.input_sha256 == input_sha256)
    );
    assert!(
        iterations
            .iter()
            .all(|row| row.input_file_sha256 == input_file_sha256)
    );
    assert!(iterations.iter().all(|row| row.counters == counters));
    assert!(
        iterations
            .iter()
            .all(|row| row.accepted_width_histogram_ns == accepted_width_histogram_ns)
    );
    assert!(iterations.iter().all(|row| {
        release_scale_iteration_comparability_identity(row) == comparability_identity
    }));
    let cpu_ticks = iterations
        .iter()
        .map(|row| row.cpu_ticks)
        .try_fold(0_u64, |sum, value| sum.checked_add(value?));
    let repetitions = iterations.len();
    let repetitions_f64 = release_scale_usize_f64(repetitions);
    let ofe_days_f64 = release_scale_usize_f64(
        repetitions
            .checked_mul(ofe_count)
            .expect("qualification batch OFE-days"),
    );
    let wall_ns_f64 = release_scale_duration_ns_f64(wall_ns);
    let cpu_ns = cpu_ticks
        .zip(clock_tick_hz)
        .map(|(ticks, hz)| release_scale_cpu_ns_f64(ticks, hz));
    let rss_return_tolerance_kib = release_scale_rss_return_tolerance_kib(ofe_count);
    let raw_iterations = iterations
        .iter()
        .enumerate()
        .map(|(iteration_index, row)| {
            serde_json::json!({
                "iteration_index": iteration_index,
                "wall_ns": row.wall_ns,
                "cpu_ticks": row.cpu_ticks,
                "rss_before_kib": row.rss_before_kib,
                "rss_after_cleanup_kib": row.rss_after_kib,
                "rss_return_tolerance_kib": rss_return_tolerance_kib,
                "rss_returned_within_tolerance": release_scale_rss_returned(
                    row.rss_before_kib,
                    row.rss_after_kib,
                    rss_return_tolerance_kib,
                ),
                "sampled_active_fixture_and_run_peak_rss_kib": row.active_rss_peak_kib,
                "active_rss_sample_count": row.active_rss_sample_count,
                "process_lifetime_peak_rss_kib_supplemental": row.peak_rss_kib,
                "hard_peak_live_rss_kib": release_scale_hard_peak_live_rss_kib(
                    row.active_rss_peak_kib,
                    row.peak_rss_kib,
                ),
                "parent_telemetry": row.parent_telemetry,
                "available_overlapping_attribution_ns": row.available_attribution_ns,
                "exclusive_subsystem_attribution": release_scale_qualification_profile(&row.qualification_telemetry),
                "accepted_covered_physical_map_records": release_scale_accepted_covered_map_records(&row.qualification_telemetry),
                "canonical_covered_width_histogram_ns": release_scale_canonical_covered_width_histogram_ns(&row.qualification_telemetry),
                "lane_d": row.lane_d_evidence,
                "closure": row.closure_evidence,
            })
        })
        .collect::<Vec<_>>();
    let rss_all_iterations_returned = iterations.iter().all(|row| {
        release_scale_rss_returned(
            row.rss_before_kib,
            row.rss_after_kib,
            rss_return_tolerance_kib,
        ) == Some(true)
    });
    let hard_peak_live_rss_samples = iterations
        .iter()
        .map(|row| release_scale_hard_peak_live_rss_kib(row.active_rss_peak_kib, row.peak_rss_kib))
        .collect::<Option<Vec<_>>>();
    let hard_peak_live_rss_measurement_complete = hard_peak_live_rss_samples.is_some();
    let hard_peak_live_rss_kib = hard_peak_live_rss_samples
        .as_ref()
        .and_then(|samples| samples.iter().copied().max());
    let exclusive_subsystem_attribution = release_scale_batch_exclusive_profile(&iterations);
    let mut attribution_ns_per_run = std::collections::BTreeMap::<&str, f64>::new();
    for row in &iterations {
        for (name, elapsed_ns) in &row.available_attribution_ns {
            *attribution_ns_per_run.entry(name).or_default() +=
                release_scale_duration_ns_f64(*elapsed_ns) / repetitions_f64;
        }
    }
    let mut batch = serde_json::json!({
        "record": "stage3_release_scale_batch_v1",
        "phase": phase,
        "batch_index": batch_index,
        "ofe_count": ofe_count,
        "repetitions": repetitions,
        "completed": true,
        "wall_ns": wall_ns,
        "wall_ns_per_run": wall_ns_f64 / repetitions_f64,
        "wall_ns_per_ofe_day": wall_ns_f64 / ofe_days_f64,
        "cpu_ticks": cpu_ticks,
        "cpu_ns": cpu_ns,
        "cpu_ns_per_run": cpu_ns.map(|value| value / repetitions_f64),
        "cpu_ns_per_ofe_day": cpu_ns.map(|value| value / ofe_days_f64),
        "input_sha256": input_sha256,
        "input_files_sha256": input_file_sha256,
        "result_sha256": comparability_identity["result_sha256"],
        "result_files_sha256": comparability_identity["result_files_sha256"],
    });
    release_scale_extend_object(
        &mut batch,
        &serde_json::json!({
            "rss_before_kib": iterations.iter().filter_map(|row| row.rss_before_kib).min(),
            "rss_after_kib": iterations.iter().filter_map(|row| row.rss_after_kib).max(),
            "peak_rss_kib": iterations.iter().filter_map(|row| row.peak_rss_kib).max(),
            "hard_peak_live_rss_measurement_complete": hard_peak_live_rss_measurement_complete,
            "hard_peak_live_rss_kib": hard_peak_live_rss_kib,
            "counters_per_run": counters,
            "adaptive_support_receipts_per_run": counters.covered_supports,
            "adaptive_request_attempt_count": serde_json::Value::Null,
            "adaptive_request_attempt_count_status": "unavailable; adaptive support receipts and direct/split candidate trials are reported without inventing a request counter",
            "accepted_width_histogram_ns_per_run": accepted_width_histogram_ns,
            "canonical_covered_width_histogram_ns_per_run": comparability_identity["canonical_covered_width_histogram_ns_per_run"],
            "accepted_covered_physical_map_records_per_run": comparability_identity["accepted_covered_physical_map_records_per_run"],
            "release_qualification_counter_identity": comparability_identity["release_qualification_counter_identity"],
            "rejected_support_count": serde_json::Value::Null,
            "rejected_support_count_status": "unavailable; rejected candidate attempts are reported without relabeling them as supports",
            "physical_map_evaluations_per_covered_support": comparability_identity["physical_map_evaluations_per_covered_support"],
            "accepted_covered_physical_map_record_semantics": "exactly one record per accepted ordinary covered support; cardinality must equal counters_per_run.covered_supports",
            "physical_map_evaluation_status": if release_scale_physical_map_identity(&comparability_identity).is_some() {
                "measured from authentic canonical maps in selected/admitted covered trials; cardinality and scope identity checked"
            } else {
                "incomplete: canonical map counters, required scopes, or accepted-support cardinality are missing or inconsistent"
            },
            "exclusive_subsystem_attribution": exclusive_subsystem_attribution,
            "subsystem_attribution_status": if exclusive_subsystem_attribution["complete"] == true {
                "complete: nested exclusive monotonic wall-time scopes"
            } else {
                "incomplete: release qualification timing scopes were unbalanced or non-reconciling"
            },
            "available_overlapping_attribution_ns_per_run": attribution_ns_per_run,
            "lane_d_per_run": iterations.first().map(|row| row.lane_d_evidence.clone()),
            "closure_operands_and_residuals_per_run": iterations.first().map(|row| row.closure_evidence.clone()),
            "comparability_identity": comparability_identity,
            "rss_return_tolerance_kib": rss_return_tolerance_kib,
            "rss_all_iterations_returned": rss_all_iterations_returned,
            "raw_iterations": raw_iterations,
        }),
    );
    batch
}

fn release_scale_batch_comparability_identity(
    ofe_count: usize,
    samples: &[serde_json::Value],
) -> Result<serde_json::Value, String> {
    let first = samples
        .first()
        .ok_or_else(|| "measured surface has no batches".to_owned())?;
    let expected = first
        .get("comparability_identity")
        .ok_or_else(|| "first batch lacks comparability identity".to_owned())?;
    for (batch_index, sample) in samples.iter().enumerate() {
        if sample["phase"] != "measured"
            || sample["completed"] != true
            || sample["ofe_count"].as_u64()
                != Some(u64::try_from(ofe_count).map_err(|error| error.to_string())?)
        {
            return Err(format!(
                "batch {batch_index} has incompatible phase/completion/OFE identity"
            ));
        }
        let observed = serde_json::json!({
            "input_sha256": sample["input_sha256"],
            "input_files_sha256": sample["input_files_sha256"],
            "result_sha256": sample["result_sha256"],
            "result_files_sha256": sample["result_files_sha256"],
            "counters_per_run": sample["counters_per_run"],
            "accepted_width_histogram_ns_per_run": sample["accepted_width_histogram_ns_per_run"],
            "accepted_covered_physical_map_records_per_run": sample["accepted_covered_physical_map_records_per_run"],
            "physical_map_evaluations_per_covered_support": sample["physical_map_evaluations_per_covered_support"],
            "canonical_covered_width_histogram_ns_per_run": sample["canonical_covered_width_histogram_ns_per_run"],
            "release_qualification_counter_identity": sample["release_qualification_counter_identity"],
            "lane_d_result_identity": sample["lane_d_per_run"],
            "closure_result_identity": sample["closure_operands_and_residuals_per_run"],
        });
        if &observed != expected {
            return Err(format!(
                "batch {batch_index} comparability identity does not match its raw evidence"
            ));
        }
        if sample.get("comparability_identity") != Some(expected) {
            return Err(format!(
                "batch {batch_index} differs in input, result, counter, width, Lane-D, or closure identity"
            ));
        }
    }
    Ok(expected.clone())
}

fn release_scale_memory_summary(samples: &[serde_json::Value]) -> serde_json::Value {
    let current_rss_kib = samples
        .iter()
        .flat_map(|row| {
            [
                row["rss_before_kib"].as_u64(),
                row["rss_after_kib"].as_u64(),
            ]
        })
        .flatten()
        .collect::<Vec<_>>();
    let hard_peak_complete = samples.iter().all(|row| {
        row["hard_peak_live_rss_measurement_complete"].as_bool() == Some(true)
            && row["hard_peak_live_rss_kib"].as_u64().is_some()
    });
    serde_json::json!({
        "max_current_rss_before_or_after_cleanup_kib": current_rss_kib.into_iter().max(),
        "rss_all_iterations_returned": samples.iter().all(|row| row["rss_all_iterations_returned"].as_bool() == Some(true)),
        "hard_peak_live_rss_measurement_complete": hard_peak_complete,
        "hard_peak_live_rss_kib": hard_peak_complete.then(|| samples.iter().filter_map(|row| row["hard_peak_live_rss_kib"].as_u64()).max()).flatten(),
        "process_lifetime_peak_rss_kib_supplemental": samples.iter().filter_map(|row| row["peak_rss_kib"].as_u64()).max(),
    })
}

fn release_scale_subsystem_attribution_summary(samples: &[serde_json::Value]) -> serde_json::Value {
    let rows = samples
        .iter()
        .map(|sample| {
            let profile = sample.get("exclusive_subsystem_attribution")?;
            Some([
                profile.get("total_ns")?.as_u64()?,
                profile
                    .get("native_vegetation_et_successor_envelope_ns")?
                    .as_u64()?,
                profile
                    .get("stage3_day_preparation_remainder_envelope_ns")?
                    .as_u64()?,
                profile.get("lane_d_ns")?.as_u64()?,
                profile.get("remaining_runner_ns")?.as_u64()?,
                u64::from(profile.get("complete")?.as_bool()?),
            ])
        })
        .collect::<Option<Vec<_>>>();
    let Some(rows) = rows else {
        return serde_json::json!({
            "subsystem_attribution_complete": false,
            "subsystem_attribution_status": "incomplete: exclusive timing batch fields are unavailable",
            "native_vegetation_fraction": serde_json::Value::Null,
            "stage3_lse_soil_fraction": serde_json::Value::Null,
            "lane_d_fraction": serde_json::Value::Null,
            "remaining_runner_fraction": serde_json::Value::Null,
        });
    };
    let mut sums = [0_u64; 5];
    let mut complete = !rows.is_empty();
    for row in rows {
        complete &= row[5] == 1;
        complete &= row[0]
            == row[1]
                .checked_add(row[2])
                .and_then(|sum| sum.checked_add(row[3]))
                .and_then(|sum| sum.checked_add(row[4]))
                .unwrap_or(u64::MAX);
        for (sum, value) in sums.iter_mut().zip(row) {
            let Some(next) = sum.checked_add(value) else {
                complete = false;
                continue;
            };
            *sum = next;
        }
    }
    complete &= sums[0] > 0
        && sums[0]
            == sums[1]
                .checked_add(sums[2])
                .and_then(|sum| sum.checked_add(sums[3]))
                .and_then(|sum| sum.checked_add(sums[4]))
                .unwrap_or(u64::MAX);
    let fraction = |value: u64| {
        complete.then(|| {
            std::time::Duration::from_nanos(value).as_secs_f64()
                / std::time::Duration::from_nanos(sums[0]).as_secs_f64()
        })
    };
    serde_json::json!({
        "subsystem_attribution_complete": complete,
        "subsystem_attribution_status": if complete {
            "complete: exact nested exclusive monotonic wall-time profile"
        } else {
            "incomplete: exclusive timing batches are missing, unbalanced, or do not sum exactly"
        },
        "exclusive_subsystem_attribution_total_ns": sums[0],
        "native_vegetation_et_successor_envelope_ns": sums[1],
        "stage3_day_preparation_remainder_envelope_ns": sums[2],
        "lane_d_ns": sums[3],
        "remaining_runner_ns": sums[4],
        "native_vegetation_fraction": fraction(sums[1]),
        "stage3_lse_soil_fraction": fraction(sums[2]),
        "lane_d_fraction": fraction(sums[3]),
        "remaining_runner_fraction": fraction(sums[4]),
        "subsystem_fraction_semantics": "native vegetation/ET and Stage3 LSE/soil fractions are conservative transaction-envelope upper bounds, not isolated component timings",
    })
}

fn release_scale_lane_d_internal_identity(
    comparability_identity: &serde_json::Value,
) -> Option<(u64, serde_json::Value)> {
    if !release_scale_counter_identity_complete(comparability_identity) {
        return None;
    }
    let lane = &comparability_identity["lane_d_result_identity"];
    let count = lane["internal_per_ofe_route_call_count"].as_u64()?;
    let order = lane["internal_per_ofe_route_call_order"].as_array()?;
    let events = lane["internal_per_ofe_route_calls"].as_array()?;
    let expected_order =
        lane["expected_internal_route_call_order_from_source_posture"].as_array()?;
    let expected_count = lane["expected_internal_route_call_count_from_source_posture"].as_u64()?;
    let calls_required = lane["internal_route_calls_required_from_source_posture"].as_bool()?;
    if count != u64::try_from(events.len()).ok()? || order.len() != events.len() {
        return None;
    }
    if count != expected_count
        || order != expected_order
        || calls_required != !expected_order.is_empty()
    {
        return None;
    }
    for (ordinal, (event, order_entry)) in events.iter().zip(order).enumerate() {
        if event["call_ordinal"].as_u64()? != u64::try_from(ordinal).ok()?
            || event["day_index"].as_u64()? != 0
            || event["ofe_index_one_based"].as_u64()? != order_entry.as_u64()?
        {
            return None;
        }
    }
    Some((count, serde_json::Value::Array(order.clone())))
}

fn release_scale_counter_identity_complete(comparability_identity: &serde_json::Value) -> bool {
    let identity = &comparability_identity["release_qualification_counter_identity"];
    identity["counters_complete"].as_bool() == Some(true)
        && identity["scopes_balanced"].as_bool() == Some(true)
        && identity["required_scope_observation_complete"].as_bool() == Some(true)
}

fn release_scale_physical_map_identity(
    comparability_identity: &serde_json::Value,
) -> Option<serde_json::Value> {
    if !release_scale_counter_identity_complete(comparability_identity) {
        return None;
    }
    let counts =
        comparability_identity["physical_map_evaluations_per_covered_support"].as_array()?;
    let records =
        comparability_identity["accepted_covered_physical_map_records_per_run"].as_array()?;
    let canonical_covered_support_count = comparability_identity
        ["canonical_covered_width_histogram_ns_per_run"]
        .as_object()?
        .values()
        .try_fold(0_usize, |total, count| {
            total.checked_add(usize::try_from(count.as_u64()?).ok()?)
        })?;
    if counts.len() != records.len() || records.len() != canonical_covered_support_count {
        return None;
    }
    let mut histogram = std::collections::BTreeMap::<String, u64>::new();
    for (count, record) in counts.iter().zip(records) {
        if count.as_u64()? == 0
            || count.as_u64()? != record["physical_map_evaluation_count"].as_u64()?
        {
            return None;
        }
        let width = record["support_width_ns"].as_u64()?.to_string();
        *histogram.entry(width).or_default() += 1;
    }
    if serde_json::to_value(histogram).ok()?
        != comparability_identity["canonical_covered_width_histogram_ns_per_run"]
    {
        return None;
    }
    Some(serde_json::Value::Array(counts.clone()))
}

fn release_scale_evidence_summary(
    samples: &[serde_json::Value],
    comparability_identity: &serde_json::Value,
) -> serde_json::Value {
    let physical_maps = release_scale_physical_map_identity(comparability_identity);
    let lane_d = release_scale_lane_d_internal_identity(comparability_identity);
    let mut summary = serde_json::json!({
        "measured_batches_comparable": true,
        "comparability_identity": comparability_identity,
        "input_sha256": comparability_identity["input_sha256"],
        "input_files_sha256": comparability_identity["input_files_sha256"],
        "result_sha256": comparability_identity["result_sha256"],
        "result_files_sha256": comparability_identity["result_files_sha256"],
        "counters_per_run": comparability_identity["counters_per_run"],
        "accepted_width_histogram_ns_per_run": comparability_identity["accepted_width_histogram_ns_per_run"],
        "rejected_candidates_per_run": comparability_identity["counters_per_run"]["rejected_candidates"],
        "rejected_candidate_batch_samples_per_run": samples.iter().map(|row| row["counters_per_run"]["rejected_candidates"].clone()).collect::<Vec<_>>(),
        "rejected_support_count": serde_json::Value::Null,
        "rejected_support_count_status": "unavailable; rejected candidate attempts are not relabeled as rejected supports",
        "adaptive_support_receipts_per_run": comparability_identity["counters_per_run"]["covered_supports"],
        "adaptive_request_attempt_count": serde_json::Value::Null,
        "adaptive_request_attempt_count_status": "unavailable",
        "accepted_covered_physical_map_records_per_run": comparability_identity["accepted_covered_physical_map_records_per_run"],
        "canonical_covered_width_histogram_ns_per_run": comparability_identity["canonical_covered_width_histogram_ns_per_run"],
        "accepted_covered_physical_map_record_semantics": "exactly one record per accepted ordinary canonical-covered support; cardinality equals canonical_covered_width_histogram_ns_per_run total, not the broader adaptive-support receipt count",
        "physical_map_evaluations_per_covered_support": physical_maps,
        "physical_map_evaluation_status": match physical_maps.as_ref().and_then(serde_json::Value::as_array) {
            Some(values) if values.is_empty() => "not_applicable: surface accepted no ordinary canonical-covered support; terminal adaptive trial counts remain reported",
            Some(_) => "measured and identity-checked",
            None => "incomplete: canonical accepted-map records are missing or inconsistent",
        },
        "available_overlapping_attribution_batch_samples_ns_per_run": samples.iter().map(|row| row["available_overlapping_attribution_ns_per_run"].clone()).collect::<Vec<_>>(),
        "exclusive_subsystem_attribution_batch_samples": samples.iter().map(|row| row["exclusive_subsystem_attribution"].clone()).collect::<Vec<_>>(),
        "lane_d_internal_route_call_count": lane_d.as_ref().map(|evidence| evidence.0),
        "lane_d_internal_route_call_order": lane_d.as_ref().map(|evidence| evidence.1.clone()),
        "lane_d_internal_route_calls_status": if lane_d.is_some() { "measured and ordered event evidence identity-checked" } else { "incomplete: ordered successful internal route-call events are missing or inconsistent" },
        "lane_d_batch_samples": samples.iter().map(|row| row["lane_d_per_run"].clone()).collect::<Vec<_>>(),
        "closure_batch_samples": samples.iter().map(|row| row["closure_operands_and_residuals_per_run"].clone()).collect::<Vec<_>>(),
    });
    release_scale_extend_object(
        &mut summary,
        &release_scale_subsystem_attribution_summary(samples),
    );
    summary
}

fn release_scale_extend_object(target: &mut serde_json::Value, source: &serde_json::Value) {
    target
        .as_object_mut()
        .expect("summary target is an object")
        .extend(
            source
                .as_object()
                .expect("summary section is an object")
                .clone(),
        );
}

fn release_scale_summary(ofe_count: usize, samples: &[serde_json::Value]) -> serde_json::Value {
    let comparability_identity = release_scale_batch_comparability_identity(ofe_count, samples)
        .expect("all measured batches must have identical comparability evidence");
    let ofe_count_f64 = release_scale_usize_f64(ofe_count);
    let ofe_seed_component =
        u64::try_from(ofe_count).expect("qualification OFE count fits bootstrap seed u64");
    let wall = samples
        .iter()
        .map(|row| row["wall_ns_per_run"].as_f64().expect("wall sample"))
        .collect::<Vec<_>>();
    let cpu = samples
        .iter()
        .filter_map(|row| row["cpu_ns_per_run"].as_f64())
        .collect::<Vec<_>>();
    let median = release_scale_statistic(&wall, ReleaseScaleStatistic::Median);
    let p95 = release_scale_statistic(&wall, ReleaseScaleStatistic::P95);
    let median_ci = release_scale_bootstrap_ci(
        &wall,
        ReleaseScaleStatistic::Median,
        RELEASE_SCALE_BOOTSTRAP_SEED ^ ofe_seed_component,
        RELEASE_SCALE_BOOTSTRAP_REPLICATES,
    );
    let p95_ci = release_scale_bootstrap_ci(
        &wall,
        ReleaseScaleStatistic::P95,
        RELEASE_SCALE_BOOTSTRAP_SEED ^ 0x9500 ^ ofe_seed_component,
        RELEASE_SCALE_BOOTSTRAP_REPLICATES,
    );
    let cpu_median =
        (!cpu.is_empty()).then(|| release_scale_statistic(&cpu, ReleaseScaleStatistic::Median));
    let cpu_p95 =
        (!cpu.is_empty()).then(|| release_scale_statistic(&cpu, ReleaseScaleStatistic::P95));
    let cpu_median_ci = (!cpu.is_empty()).then(|| {
        release_scale_bootstrap_ci(
            &cpu,
            ReleaseScaleStatistic::Median,
            RELEASE_SCALE_BOOTSTRAP_SEED ^ 0xc000 ^ ofe_seed_component,
            RELEASE_SCALE_BOOTSTRAP_REPLICATES,
        )
    });
    let cpu_p95_ci = (!cpu.is_empty()).then(|| {
        release_scale_bootstrap_ci(
            &cpu,
            ReleaseScaleStatistic::P95,
            RELEASE_SCALE_BOOTSTRAP_SEED ^ 0xc950 ^ ofe_seed_component,
            RELEASE_SCALE_BOOTSTRAP_REPLICATES,
        )
    });
    let mut summary = serde_json::json!({
        "record": "stage3_release_scale_summary_v1",
        "ofe_count": ofe_count,
        "measured_batch_count": samples.len(),
        "wall_ns_per_run_median": median,
        "wall_ns_per_run_p95": p95,
        "wall_median_bootstrap_95_ci_ns": [median_ci.0, median_ci.1],
        "wall_p95_bootstrap_95_ci_ns": [p95_ci.0, p95_ci.1],
        "wall_ns_per_ofe_day_median": median / ofe_count_f64,
        "wall_ns_per_ofe_day_p95": p95 / ofe_count_f64,
        "wall_median_bootstrap_95_ci_ns_per_ofe_day": [median_ci.0 / ofe_count_f64, median_ci.1 / ofe_count_f64],
        "wall_p95_bootstrap_95_ci_ns_per_ofe_day": [p95_ci.0 / ofe_count_f64, p95_ci.1 / ofe_count_f64],
        "bootstrap_seeds": {
            "wall_median": RELEASE_SCALE_BOOTSTRAP_SEED ^ ofe_seed_component,
            "wall_p95": RELEASE_SCALE_BOOTSTRAP_SEED ^ 0x9500 ^ ofe_seed_component,
            "cpu_median": RELEASE_SCALE_BOOTSTRAP_SEED ^ 0xc000 ^ ofe_seed_component,
            "cpu_p95": RELEASE_SCALE_BOOTSTRAP_SEED ^ 0xc950 ^ ofe_seed_component,
        },
        "cpu_ns_per_run_median": cpu_median,
        "cpu_ns_per_run_p95": cpu_p95,
        "cpu_median_bootstrap_95_ci_ns": cpu_median_ci.map(|ci| [ci.0, ci.1]),
        "cpu_p95_bootstrap_95_ci_ns": cpu_p95_ci.map(|ci| [ci.0, ci.1]),
        "cpu_ns_per_ofe_day_median": cpu_median.map(|value| value / ofe_count_f64),
        "cpu_ns_per_ofe_day_p95": cpu_p95.map(|value| value / ofe_count_f64),
    });
    release_scale_extend_object(&mut summary, &release_scale_memory_summary(samples));
    release_scale_extend_object(
        &mut summary,
        &release_scale_evidence_summary(samples, &comparability_identity),
    );
    summary
}

#[derive(Clone, Debug)]
struct ReleaseScaleDecisionSurfaceV1 {
    ofe_count: usize,
    wall_ns_per_run_median: Option<f64>,
    wall_ns_per_ofe_day_median: Option<f64>,
    cpu_ns_per_run_median: Option<f64>,
    cpu_ns_per_ofe_day_median: Option<f64>,
    hard_peak_live_rss_kib: Option<u64>,
    all_iterations_returned_rss: Option<bool>,
    physical_map_evaluations: Option<Vec<u64>>,
    subsystem_attribution_complete: bool,
    native_vegetation_fraction: Option<f64>,
    stage3_lse_soil_fraction: Option<f64>,
    lane_d_fraction: Option<f64>,
    remaining_runner_fraction: Option<f64>,
    lane_d_internal_route_call_count: Option<u64>,
    lane_d_internal_route_call_order: Option<Vec<usize>>,
}

fn release_scale_physical_map_distribution(values: &[u64]) -> Option<(f64, f64, u64)> {
    if values.is_empty() {
        return None;
    }
    let values_f64 = values
        .iter()
        .map(|value| {
            f64::from(u32::try_from(*value).expect("physical-map count fits qualification u32"))
        })
        .collect::<Vec<_>>();
    Some((
        release_scale_statistic(&values_f64, ReleaseScaleStatistic::Median),
        release_scale_statistic(&values_f64, ReleaseScaleStatistic::P95),
        *values.iter().max()?,
    ))
}

#[allow(
    clippy::too_many_lines,
    reason = "one fail-closed decision function keeps every declared PASS prerequisite visible"
)]
fn release_scale_decision(surfaces: &[ReleaseScaleDecisionSurfaceV1]) -> serde_json::Value {
    let mut checks = std::collections::BTreeMap::<String, bool>::new();
    let mut metrics = std::collections::BTreeMap::<String, serde_json::Value>::new();
    let mut unavailable = Vec::<String>::new();
    let find = |ofe_count| {
        surfaces
            .iter()
            .find(|surface| surface.ofe_count == ofe_count)
    };
    let one = find(1);
    let ten = find(10);
    let nineteen = find(19);
    for ofe_count in [1_usize, 10, 19] {
        if find(ofe_count).is_none() {
            unavailable.push(format!("{ofe_count}-OFE surface"));
        }
    }
    if let (Some(one), Some(ten), Some(nineteen)) = (one, ten, nineteen) {
        let ratio = |numerator: Option<f64>, denominator: Option<f64>| {
            numerator
                .zip(denominator)
                .and_then(|(numerator, denominator)| {
                    (numerator.is_finite() && denominator.is_finite() && denominator > 0.0)
                        .then_some(numerator / denominator)
                })
        };
        match ratio(ten.wall_ns_per_run_median, one.wall_ns_per_run_median) {
            Some(value) => {
                metrics.insert("t10_over_t1".to_owned(), serde_json::json!(value));
                checks.insert("t10_over_t1_le_12".to_owned(), value <= 12.0);
            }
            None => unavailable.push("T10/T1 wall ratio".to_owned()),
        }
        match ratio(nineteen.wall_ns_per_run_median, ten.wall_ns_per_run_median) {
            Some(value) => {
                metrics.insert("t19_over_t10".to_owned(), serde_json::json!(value));
                checks.insert("t19_over_t10_le_2_2".to_owned(), value <= 2.2);
            }
            None => unavailable.push("T19/T10 wall ratio".to_owned()),
        }
        for (name, value, limit) in [
            (
                "ten_ofe_cpu_ns_per_day_le_5000000",
                ten.cpu_ns_per_run_median,
                5_000_000.0,
            ),
            (
                "ten_ofe_wall_ns_per_day_le_5500000",
                ten.wall_ns_per_run_median,
                5_500_000.0,
            ),
            (
                "nineteen_ofe_cpu_ns_per_ofe_day_le_500000",
                nineteen.cpu_ns_per_ofe_day_median,
                500_000.0,
            ),
            (
                "nineteen_ofe_wall_ns_per_ofe_day_le_550000",
                nineteen.wall_ns_per_ofe_day_median,
                550_000.0,
            ),
        ] {
            match value {
                Some(value) if value.is_finite() => {
                    checks.insert(name.to_owned(), value <= limit);
                }
                _ => unavailable.push(name.to_owned()),
            }
        }
        for surface in [one, ten, nineteen] {
            let ofe_count =
                u64::try_from(surface.ofe_count).expect("qualification OFE count fits u64");
            let memory_limit_kib = (128_u64 + 16 * ofe_count) * 1024;
            match surface.hard_peak_live_rss_kib {
                Some(value) => {
                    checks.insert(
                        format!("{}ofe_hard_peak_live_rss_within_kib", surface.ofe_count),
                        value <= memory_limit_kib,
                    );
                }
                None => unavailable.push(format!("{}-OFE hard peak live RSS", surface.ofe_count)),
            }
            match surface.all_iterations_returned_rss {
                Some(value) => {
                    checks.insert(format!("{}ofe_rss_returned", surface.ofe_count), value);
                }
                None => unavailable.push(format!("{}-OFE RSS return", surface.ofe_count)),
            }
            match surface.physical_map_evaluations.as_deref() {
                Some([]) => {}
                Some(values) => match release_scale_physical_map_distribution(values) {
                Some((median, p95, maximum)) => {
                    metrics.insert(
                        format!("{}ofe_physical_map_distribution", surface.ofe_count),
                        serde_json::json!({
                            "median": median,
                            "p95": p95,
                            "max": maximum,
                            "raw": surface.physical_map_evaluations,
                        }),
                    );
                    checks.insert(
                        format!("{}ofe_physical_map_median_le_2", surface.ofe_count),
                        median <= 2.0,
                    );
                    checks.insert(
                        format!("{}ofe_physical_map_p95_le_4", surface.ofe_count),
                        p95 <= 4.0,
                    );
                    checks.insert(
                        format!("{}ofe_physical_map_max_le_8", surface.ofe_count),
                        maximum <= 8,
                    );
                }
                None => unavailable.push(format!(
                    "{}-OFE canonical physical-map distribution",
                    surface.ofe_count
                )),
                },
                None => unavailable.push(format!(
                    "{}-OFE canonical physical-map distribution",
                    surface.ofe_count
                )),
            }
            if surface.subsystem_attribution_complete {
                checks.insert(
                    format!("{}ofe_subsystem_attribution_complete", surface.ofe_count),
                    true,
                );
                for (label, value, limit) in [
                    (
                        "native_vegetation_fraction_le_0_35",
                        surface.native_vegetation_fraction,
                        0.35,
                    ),
                    (
                        "stage3_lse_soil_fraction_le_0_40",
                        surface.stage3_lse_soil_fraction,
                        0.40,
                    ),
                    ("lane_d_fraction_le_0_20", surface.lane_d_fraction, 0.20),
                    (
                        "remaining_runner_fraction_le_0_20",
                        surface.remaining_runner_fraction,
                        0.20,
                    ),
                ] {
                    match value {
                        Some(value) if value.is_finite() && value >= 0.0 => {
                            metrics.insert(
                                format!("{}ofe_{label}", surface.ofe_count),
                                serde_json::json!(value),
                            );
                            checks.insert(
                                format!("{}ofe_{label}", surface.ofe_count),
                                value <= limit,
                            );
                        }
                        _ => unavailable.push(format!("{}-OFE {label}", surface.ofe_count)),
                    }
                }
            } else {
                unavailable.push(format!(
                    "{}-OFE complete subsystem attribution",
                    surface.ofe_count
                ));
            }
            match (
                surface.lane_d_internal_route_call_count,
                surface.lane_d_internal_route_call_order.as_deref(),
            ) {
                (Some(call_count), Some(call_order)) => {
                    let expected_order = (1..=surface.ofe_count).collect::<Vec<_>>();
                    checks.insert(
                        format!(
                            "{}ofe_lane_d_internal_route_call_count_and_order_verified",
                            surface.ofe_count
                        ),
                        call_count
                            == u64::try_from(call_order.len())
                                .expect("Lane-D route-call order length fits u64")
                            && call_order == expected_order,
                    );
                }
                _ => unavailable.push(format!(
                    "{}-OFE internal Lane-D route-call count/order",
                    surface.ofe_count
                )),
            }
        }
    }
    let any_budget_failed = checks.values().any(|passed| !passed);
    let status = if any_budget_failed {
        "FAIL"
    } else if unavailable.is_empty() {
        "PASS"
    } else {
        "INCOMPLETE"
    };
    serde_json::json!({
        "status": status,
        "metrics": metrics,
        "budget_checks": checks,
        "unavailable_required_measurements": unavailable,
        "one_ofe_absolute_budget_status": "not_applicable: full mixed-regime runner day is not a pure snow-free subsystem measurement",
    })
}

fn release_scale_protocol_schema_valid(record: &serde_json::Value) -> bool {
    let Some(record) = record.as_object() else {
        return false;
    };
    [
        "record",
        "phase",
        "batch_index",
        "ofe_count",
        "repetitions",
        "completed",
        "wall_ns",
        "cpu_ticks",
        "input_sha256",
        "input_files_sha256",
        "result_sha256",
        "result_files_sha256",
        "rss_return_tolerance_kib",
        "rss_all_iterations_returned",
        "hard_peak_live_rss_measurement_complete",
        "hard_peak_live_rss_kib",
        "raw_iterations",
        "counters_per_run",
        "adaptive_support_receipts_per_run",
        "adaptive_request_attempt_count",
        "accepted_width_histogram_ns_per_run",
        "canonical_covered_width_histogram_ns_per_run",
        "accepted_covered_physical_map_records_per_run",
        "release_qualification_counter_identity",
        "rejected_support_count",
        "physical_map_evaluations_per_covered_support",
        "exclusive_subsystem_attribution",
        "subsystem_attribution_status",
        "available_overlapping_attribution_ns_per_run",
        "lane_d_per_run",
        "closure_operands_and_residuals_per_run",
        "comparability_identity",
    ]
    .iter()
    .all(|key| record.contains_key(*key))
        && record
            .get("release_qualification_counter_identity")
            .is_some_and(|identity| {
                [
                    "scopes_balanced",
                    "counters_complete",
                    "required_scope_observation_complete",
                    "stage3_scope_entry_count",
                    "native_vegetation_et_scope_entry_count",
                    "lane_d_scope_entry_count",
                ]
                .iter()
                .all(|key| identity.get(*key).is_some())
            })
        && record
            .get("exclusive_subsystem_attribution")
            .is_some_and(release_scale_exclusive_profile_schema_valid)
        && record
            .get("raw_iterations")
            .and_then(serde_json::Value::as_array)
            .is_some_and(|iterations| {
                !iterations.is_empty()
                    && iterations.iter().all(|iteration| {
                        [
                            "iteration_index",
                            "wall_ns",
                            "cpu_ticks",
                            "rss_before_kib",
                            "rss_after_cleanup_kib",
                            "rss_return_tolerance_kib",
                            "rss_returned_within_tolerance",
                            "sampled_active_fixture_and_run_peak_rss_kib",
                            "active_rss_sample_count",
                            "process_lifetime_peak_rss_kib_supplemental",
                            "hard_peak_live_rss_kib",
                            "parent_telemetry",
                            "available_overlapping_attribution_ns",
                            "exclusive_subsystem_attribution",
                            "accepted_covered_physical_map_records",
                            "canonical_covered_width_histogram_ns",
                            "lane_d",
                            "closure",
                        ]
                        .iter()
                        .all(|key| iteration.get(*key).is_some())
                            && iteration
                                .get("exclusive_subsystem_attribution")
                                .is_some_and(release_scale_exclusive_profile_schema_valid)
                    })
            })
}

fn release_scale_exclusive_profile_schema_valid(profile: &serde_json::Value) -> bool {
    [
        "complete",
        "total_ns",
        "native_vegetation_et_successor_envelope_ns",
        "stage3_day_preparation_remainder_envelope_ns",
        "lane_d_ns",
        "remaining_runner_ns",
        "exclusive_sum_ns",
        "exclusive_sum_matches_total",
    ]
    .iter()
    .all(|key| profile.get(*key).is_some())
}

fn release_scale_surface_from_summary(
    summary: &serde_json::Value,
) -> ReleaseScaleDecisionSurfaceV1 {
    let physical_map_evaluations = summary["physical_map_evaluations_per_covered_support"]
        .as_array()
        .map(|values| {
            values
                .iter()
                .map(|value| value.as_u64().expect("physical-map sample is u64"))
                .collect()
        });
    ReleaseScaleDecisionSurfaceV1 {
        ofe_count: usize::try_from(summary["ofe_count"].as_u64().expect("summary OFE count"))
            .expect("summary OFE count fits usize"),
        wall_ns_per_run_median: summary["wall_ns_per_run_median"].as_f64(),
        wall_ns_per_ofe_day_median: summary["wall_ns_per_ofe_day_median"].as_f64(),
        cpu_ns_per_run_median: summary["cpu_ns_per_run_median"].as_f64(),
        cpu_ns_per_ofe_day_median: summary["cpu_ns_per_ofe_day_median"].as_f64(),
        hard_peak_live_rss_kib: summary["hard_peak_live_rss_kib"].as_u64(),
        all_iterations_returned_rss: summary["rss_all_iterations_returned"].as_bool(),
        physical_map_evaluations,
        subsystem_attribution_complete: summary["subsystem_attribution_complete"]
            .as_bool()
            .unwrap_or(false),
        native_vegetation_fraction: summary["native_vegetation_fraction"].as_f64(),
        stage3_lse_soil_fraction: summary["stage3_lse_soil_fraction"].as_f64(),
        lane_d_fraction: summary["lane_d_fraction"].as_f64(),
        remaining_runner_fraction: summary["remaining_runner_fraction"].as_f64(),
        lane_d_internal_route_call_count: summary["lane_d_internal_route_call_count"].as_u64(),
        lane_d_internal_route_call_order: summary["lane_d_internal_route_call_order"]
            .as_array()
            .map(|order| {
                order
                    .iter()
                    .map(|value| {
                        usize::try_from(value.as_u64().expect("Lane-D route call is u64"))
                            .expect("Lane-D route call fits usize")
                    })
                    .collect()
            }),
    }
}

fn release_scale_test_surface(ofe_count: usize) -> ReleaseScaleDecisionSurfaceV1 {
    let ofe_count_f64 = release_scale_usize_f64(ofe_count);
    ReleaseScaleDecisionSurfaceV1 {
        ofe_count,
        wall_ns_per_run_median: Some(200_000.0 * ofe_count_f64),
        wall_ns_per_ofe_day_median: Some(200_000.0),
        cpu_ns_per_run_median: Some(180_000.0 * ofe_count_f64),
        cpu_ns_per_ofe_day_median: Some(180_000.0),
        hard_peak_live_rss_kib: Some(100 * 1024),
        all_iterations_returned_rss: Some(true),
        physical_map_evaluations: Some(vec![1, 2, 2, 3, 4]),
        subsystem_attribution_complete: true,
        native_vegetation_fraction: Some(0.30),
        stage3_lse_soil_fraction: Some(0.35),
        lane_d_fraction: Some(0.15),
        remaining_runner_fraction: Some(0.20),
        lane_d_internal_route_call_count: Some(
            u64::try_from(ofe_count).expect("test OFE count fits u64"),
        ),
        lane_d_internal_route_call_order: Some((1..=ofe_count).collect()),
    }
}

#[test]
fn release_scale_statistics_and_protocol_are_deterministic() {
    let values = vec![1.0, 2.0, 3.0, 4.0];
    assert!(
        (release_scale_statistic(&values, ReleaseScaleStatistic::Median) - 2.5).abs()
            < f64::EPSILON
    );
    assert!((release_scale_statistic(&values, ReleaseScaleStatistic::P95) - 3.85).abs() < 1.0e-12);
    let first = release_scale_bootstrap_ci(&values, ReleaseScaleStatistic::Median, 42, 1_000);
    let second = release_scale_bootstrap_ci(&values, ReleaseScaleStatistic::Median, 42, 1_000);
    assert_eq!(first, second);
    assert!(first.0 <= 2.5 && first.1 >= 2.5);
    const {
        assert!(RELEASE_SCALE_WARMUP_BATCHES == 5);
        assert!(RELEASE_SCALE_MEASURED_BATCHES >= 30);
        assert!(RELEASE_SCALE_MIN_BATCH_NS >= 250_000_000);
    }
}

#[test]
fn release_scale_linux_process_parsers_are_exact() {
    let stat = "123 (name with space) R 1 2 3 4 5 6 7 8 9 10 130 170 15";
    assert_eq!(release_scale_process_cpu_ticks_from_stat(stat), Some(300));
    assert_eq!(release_scale_process_cpu_ticks_from_stat("malformed"), None);
    assert_eq!(release_scale_cpu_affinity_count("7"), Some(1));
    assert_eq!(release_scale_cpu_affinity_count("2-4,8,10-11"), Some(6));
    assert_eq!(release_scale_cpu_affinity_count("4-2"), None);
    let status = "Name:\tqualification\nVmHWM:\t  45678 kB\nVmRSS:\t  12345 kB\n";
    assert_eq!(release_scale_status_kib(status, "VmRSS:"), Some(12_345));
    assert_eq!(release_scale_status_kib(status, "VmHWM:"), Some(45_678));
    assert_eq!(release_scale_status_kib(status, "VmPeak:"), None);
}

#[test]
fn release_scale_final_decision_prevents_false_passes() {
    let valid = [
        release_scale_test_surface(1),
        release_scale_test_surface(10),
        release_scale_test_surface(19),
    ];
    let decision = release_scale_decision(&valid);
    assert_eq!(decision["status"], "PASS");
    assert!(
        decision["budget_checks"]
            .as_object()
            .expect("budget checks")
            .values()
            .all(|value| value.as_bool() == Some(true))
    );
    assert_eq!(
        decision["budget_checks"]
            .as_object()
            .expect("budget checks")
            .len(),
        39,
        "PASS requires every declared scaling, absolute, memory, physical-map, attribution, and Lane-D check",
    );
    for required in [
        "ten_ofe_cpu_ns_per_day_le_5000000",
        "ten_ofe_wall_ns_per_day_le_5500000",
        "nineteen_ofe_cpu_ns_per_ofe_day_le_500000",
        "nineteen_ofe_wall_ns_per_ofe_day_le_550000",
        "19ofe_hard_peak_live_rss_within_kib",
        "19ofe_physical_map_median_le_2",
        "19ofe_physical_map_p95_le_4",
        "19ofe_physical_map_max_le_8",
    ] {
        assert_eq!(decision["budget_checks"][required], true, "{required}");
    }

    let mut missing_physical = valid.clone();
    missing_physical[2].physical_map_evaluations = None;
    assert_eq!(
        release_scale_decision(&missing_physical)["status"],
        "INCOMPLETE"
    );

    let mut missing_attribution = valid.clone();
    missing_attribution[1].subsystem_attribution_complete = false;
    assert_eq!(
        release_scale_decision(&missing_attribution)["status"],
        "INCOMPLETE"
    );

    let mut missing_lane_calls = valid.clone();
    missing_lane_calls[0].lane_d_internal_route_call_order = None;
    assert_eq!(
        release_scale_decision(&missing_lane_calls)["status"],
        "INCOMPLETE"
    );

    let mut false_lane_order = valid.clone();
    false_lane_order[2].lane_d_internal_route_call_order = Some(vec![1, 3, 2]);
    false_lane_order[2].lane_d_internal_route_call_count = Some(3);
    assert_eq!(release_scale_decision(&false_lane_order)["status"], "FAIL");

    let mut missing_active_peak = valid.clone();
    missing_active_peak[1].hard_peak_live_rss_kib = None;
    assert_eq!(
        release_scale_decision(&missing_active_peak)["status"],
        "INCOMPLETE"
    );

    let mut over_wall_budget = valid.clone();
    over_wall_budget[1].wall_ns_per_run_median = Some(5_500_001.0);
    assert_eq!(release_scale_decision(&over_wall_budget)["status"], "FAIL");

    let mut retained_allocation = valid;
    retained_allocation[0].all_iterations_returned_rss = Some(false);
    assert_eq!(
        release_scale_decision(&retained_allocation)["status"],
        "FAIL"
    );

    let mut freed_over_budget_temporary = [
        release_scale_test_surface(1),
        release_scale_test_surface(10),
        release_scale_test_surface(19),
    ];
    freed_over_budget_temporary[0].hard_peak_live_rss_kib = Some((128 + 16) * 1024 + 1);
    freed_over_budget_temporary[0].all_iterations_returned_rss = Some(true);
    assert_eq!(
        release_scale_decision(&freed_over_budget_temporary)["status"],
        "FAIL",
        "post-cleanup return cannot hide an over-budget active peak"
    );
}

#[test]
fn release_scale_physical_map_distribution_quantiles_are_exact() {
    let (median, p95, maximum) =
        release_scale_physical_map_distribution(&[1, 2, 3, 4, 8]).expect("distribution");
    assert!((median - 3.0).abs() < f64::EPSILON);
    assert!((p95 - 7.2).abs() < 1.0e-12);
    assert_eq!(maximum, 8);
    assert_eq!(release_scale_physical_map_distribution(&[]), None);
}

#[test]
fn release_scale_rss_return_boundary_is_inclusive_and_missing_fails_closed() {
    let tolerance = release_scale_rss_return_tolerance_kib(10);
    assert_eq!(tolerance, 18 * 1024);
    assert_eq!(
        release_scale_rss_returned(Some(100_000), Some(100_000 + tolerance), tolerance),
        Some(true)
    );
    assert_eq!(
        release_scale_rss_returned(Some(100_000), Some(100_001 + tolerance), tolerance),
        Some(false)
    );
    assert_eq!(
        release_scale_rss_returned(None, Some(100_000), tolerance),
        None
    );
    assert_eq!(
        release_scale_rss_returned(Some(100_000), None, tolerance),
        None
    );
    assert_eq!(
        release_scale_hard_peak_live_rss_kib(Some(150_000), Some(175_000)),
        Some(175_000),
        "isolated-process high-water mark closes gaps between active samples"
    );
    assert_eq!(
        release_scale_hard_peak_live_rss_kib(Some(180_000), Some(175_000)),
        Some(180_000)
    );
    assert_eq!(
        release_scale_hard_peak_live_rss_kib(None, Some(175_000)),
        None
    );
    assert_eq!(
        release_scale_hard_peak_live_rss_kib(Some(175_000), None),
        None
    );
}

#[test]
fn release_scale_cross_batch_comparability_is_exact() {
    let identity = serde_json::json!({
        "input_sha256": "abc",
        "input_files_sha256": {"case.run": "def"},
        "result_sha256": "123",
        "result_files_sha256": {"H83.pass.parquet": "456"},
        "counters_per_run": {"covered_supports": 1, "rejected_candidates": 0},
        "accepted_width_histogram_ns_per_run": {"60000000000": 4},
        "accepted_covered_physical_map_records_per_run": [{
            "accepted_support_ordinal": 0,
            "support_start_ns": 0,
            "support_end_ns": 60_000_000_000_u64,
            "support_width_ns": 60_000_000_000_u64,
            "physical_map_evaluation_count": 3,
        }],
        "physical_map_evaluations_per_covered_support": [3],
        "canonical_covered_width_histogram_ns_per_run": {"60000000000": 1},
        "release_qualification_counter_identity": {
            "scopes_balanced": true,
            "counters_complete": true,
            "required_scope_observation_complete": true,
            "stage3_scope_entry_count": 1,
            "native_vegetation_et_scope_entry_count": 1,
            "lane_d_scope_entry_count": 1,
        },
        "lane_d_result_identity": {
            "routed_day_count": 1,
            "internal_per_ofe_route_call_count": 1,
            "internal_per_ofe_route_call_order": [1],
            "internal_per_ofe_route_calls": [{"call_ordinal": 0, "day_index": 0, "ofe_index_one_based": 1}],
            "expected_internal_route_call_count_from_source_posture": 1,
            "expected_internal_route_call_order_from_source_posture": [1],
            "internal_route_calls_required_from_source_posture": true,
        },
        "closure_result_identity": {"source_m3": 1.0, "residual_m3": 0.0},
    });
    let sample = |batch_index| {
        serde_json::json!({
            "phase": "measured",
            "completed": true,
            "ofe_count": 10,
            "batch_index": batch_index,
            "input_sha256": identity["input_sha256"],
            "input_files_sha256": identity["input_files_sha256"],
            "result_sha256": identity["result_sha256"],
            "result_files_sha256": identity["result_files_sha256"],
            "counters_per_run": identity["counters_per_run"],
            "accepted_width_histogram_ns_per_run": identity["accepted_width_histogram_ns_per_run"],
            "accepted_covered_physical_map_records_per_run": identity["accepted_covered_physical_map_records_per_run"],
            "physical_map_evaluations_per_covered_support": identity["physical_map_evaluations_per_covered_support"],
            "canonical_covered_width_histogram_ns_per_run": identity["canonical_covered_width_histogram_ns_per_run"],
            "release_qualification_counter_identity": identity["release_qualification_counter_identity"],
            "exclusive_subsystem_attribution": {
                "complete": true,
                "total_ns": 100,
                "native_vegetation_et_successor_envelope_ns": 30,
                "stage3_day_preparation_remainder_envelope_ns": 35,
                "lane_d_ns": 15,
                "remaining_runner_ns": 20,
            },
            "lane_d_per_run": identity["lane_d_result_identity"],
            "closure_operands_and_residuals_per_run": identity["closure_result_identity"],
            "comparability_identity": identity,
        })
    };
    let mut samples = vec![sample(0), sample(1)];
    let retained = release_scale_batch_comparability_identity(10, &samples)
        .expect("identical batches are comparable");
    assert_eq!(retained["input_sha256"], "abc");
    assert_eq!(retained["input_files_sha256"]["case.run"], "def");
    let summary_evidence = release_scale_evidence_summary(&samples, &retained);
    assert_eq!(summary_evidence["input_sha256"], "abc");
    assert_eq!(summary_evidence["input_files_sha256"]["case.run"], "def");
    assert_eq!(summary_evidence["result_sha256"], "123");
    assert_eq!(summary_evidence["measured_batches_comparable"], true);

    samples[1]["comparability_identity"]["counters_per_run"]["covered_supports"] =
        serde_json::json!(5);
    assert!(release_scale_batch_comparability_identity(10, &samples).is_err());
}

#[test]
fn release_scale_real_telemetry_identity_prevents_placeholder_passes() {
    let identity = serde_json::json!({
        "release_qualification_counter_identity": {
            "scopes_balanced": true,
            "counters_complete": true,
            "required_scope_observation_complete": true,
        },
        "counters_per_run": {"covered_supports": 1},
        "accepted_covered_physical_map_records_per_run": [{
            "support_width_ns": 60,
            "physical_map_evaluation_count": 3,
        }],
        "physical_map_evaluations_per_covered_support": [3],
        "canonical_covered_width_histogram_ns_per_run": {"60": 1},
        "lane_d_result_identity": {
            "internal_per_ofe_route_call_count": 2,
            "internal_per_ofe_route_call_order": [1, 2],
            "internal_per_ofe_route_calls": [
                {"call_ordinal": 0, "day_index": 0, "ofe_index_one_based": 1},
                {"call_ordinal": 1, "day_index": 0, "ofe_index_one_based": 2},
            ],
            "expected_internal_route_call_count_from_source_posture": 2,
            "expected_internal_route_call_order_from_source_posture": [1, 2],
            "internal_route_calls_required_from_source_posture": true,
        },
    });
    assert_eq!(
        release_scale_physical_map_identity(&identity),
        Some(serde_json::json!([3]))
    );
    assert_eq!(
        release_scale_lane_d_internal_identity(&identity),
        Some((2, serde_json::json!([1, 2])))
    );

    let mut inconsistent_maps = identity.clone();
    inconsistent_maps["physical_map_evaluations_per_covered_support"][0] = serde_json::json!(2);
    assert_eq!(
        release_scale_physical_map_identity(&inconsistent_maps),
        None
    );

    let mut zero_map = identity.clone();
    zero_map["physical_map_evaluations_per_covered_support"][0] = serde_json::json!(0);
    zero_map["accepted_covered_physical_map_records_per_run"][0]["physical_map_evaluation_count"] =
        serde_json::json!(0);
    assert_eq!(release_scale_physical_map_identity(&zero_map), None);

    let mut subset_maps = identity.clone();
    subset_maps["canonical_covered_width_histogram_ns_per_run"] = serde_json::json!({"60": 2});
    assert_eq!(release_scale_physical_map_identity(&subset_maps), None);

    let terminal_only = serde_json::json!({
        "release_qualification_counter_identity": {
            "scopes_balanced": true,
            "counters_complete": true,
            "required_scope_observation_complete": true,
        },
        "counters_per_run": {"covered_supports": 4},
        "accepted_covered_physical_map_records_per_run": [],
        "physical_map_evaluations_per_covered_support": [],
        "canonical_covered_width_histogram_ns_per_run": {},
    });
    assert_eq!(
        release_scale_physical_map_identity(&terminal_only),
        Some(serde_json::json!([])),
        "terminal adaptive supports are not mislabeled as ordinary canonical-covered supports",
    );

    let mut missing_required_scope = identity.clone();
    missing_required_scope["release_qualification_counter_identity"]["required_scope_observation_complete"] =
        serde_json::json!(false);
    assert_eq!(
        release_scale_physical_map_identity(&missing_required_scope),
        None
    );
    assert_eq!(
        release_scale_lane_d_internal_identity(&missing_required_scope),
        None
    );

    let mut availability_only = identity;
    availability_only["lane_d_result_identity"] = serde_json::json!({"available": true});
    assert_eq!(
        release_scale_lane_d_internal_identity(&availability_only),
        None
    );

    let zero_source = serde_json::json!({
        "release_qualification_counter_identity": {
            "scopes_balanced": true,
            "counters_complete": true,
            "required_scope_observation_complete": true,
        },
        "lane_d_result_identity": {
            "internal_per_ofe_route_call_count": 0,
            "internal_per_ofe_route_call_order": [],
            "internal_per_ofe_route_calls": [],
            "expected_internal_route_call_count_from_source_posture": 0,
            "expected_internal_route_call_order_from_source_posture": [],
            "internal_route_calls_required_from_source_posture": false,
        },
    });
    assert_eq!(
        release_scale_lane_d_internal_identity(&zero_source),
        Some((0, serde_json::json!([])))
    );

    let complete = [serde_json::json!({
        "exclusive_subsystem_attribution": {
            "complete": true,
            "total_ns": 100,
            "native_vegetation_et_successor_envelope_ns": 30,
            "stage3_day_preparation_remainder_envelope_ns": 35,
            "lane_d_ns": 15,
            "remaining_runner_ns": 20,
        },
    })];
    let attribution = release_scale_subsystem_attribution_summary(&complete);
    assert_eq!(attribution["subsystem_attribution_complete"], true);
    assert_eq!(attribution["remaining_runner_fraction"], 0.2);
    let mut unbalanced = complete;
    unbalanced[0]["exclusive_subsystem_attribution"]["remaining_runner_ns"] = serde_json::json!(21);
    assert_eq!(
        release_scale_subsystem_attribution_summary(&unbalanced)["subsystem_attribution_complete"],
        false
    );
}

#[test]
fn release_scale_required_scope_counts_fail_closed_but_allow_zero_source_lane_d() {
    assert!(release_scale_required_scope_counts_complete(
        1, 1, 0, 1, false
    ));
    assert!(!release_scale_required_scope_counts_complete(
        0, 1, 1, 1, true
    ));
    assert!(!release_scale_required_scope_counts_complete(
        1, 0, 1, 1, true
    ));
    assert!(!release_scale_required_scope_counts_complete(
        1, 1, 0, 1, true
    ));
    assert!(release_scale_required_scope_counts_complete(
        1, 0, 0, 0, false
    ));
}

#[test]
fn release_scale_protocol_schema_rejects_missing_required_fields() {
    let raw_iteration = serde_json::json!({
        "iteration_index": 0,
        "wall_ns": 250_000_000_u64,
        "cpu_ticks": 25,
        "input_sha256": "abc",
        "input_files_sha256": {"case.run": "def"},
        "result_sha256": "123",
        "result_files_sha256": {"H83.pass.parquet": "456"},
        "rss_before_kib": 100,
        "rss_after_cleanup_kib": 101,
        "rss_return_tolerance_kib": 9_216,
        "rss_returned_within_tolerance": true,
        "sampled_active_fixture_and_run_peak_rss_kib": 110,
        "active_rss_sample_count": 10,
        "process_lifetime_peak_rss_kib_supplemental": 120,
        "hard_peak_live_rss_kib": 120,
        "parent_telemetry": [],
        "available_overlapping_attribution_ns": {},
        "exclusive_subsystem_attribution": {
            "complete": true,
            "total_ns": 100,
            "native_vegetation_et_successor_envelope_ns": 30,
            "stage3_day_preparation_remainder_envelope_ns": 35,
            "lane_d_ns": 15,
            "remaining_runner_ns": 20,
            "exclusive_sum_ns": 100,
            "exclusive_sum_matches_total": true,
        },
        "accepted_covered_physical_map_records": [],
        "canonical_covered_width_histogram_ns": {},
        "lane_d": {},
        "closure": {},
    });
    let mut record = serde_json::json!({
        "record": "stage3_release_scale_batch_v1",
        "phase": "measured",
        "batch_index": 0,
        "ofe_count": 1,
        "repetitions": 1,
        "completed": true,
        "wall_ns": 250_000_000_u64,
        "cpu_ticks": 25,
        "input_sha256": "abc",
        "input_files_sha256": {"case.run": "def"},
        "result_sha256": "123",
        "result_files_sha256": {"H83.pass.parquet": "456"},
        "rss_return_tolerance_kib": 9_216,
        "rss_all_iterations_returned": true,
        "hard_peak_live_rss_measurement_complete": true,
        "hard_peak_live_rss_kib": 120,
        "raw_iterations": [raw_iteration],
        "counters_per_run": {},
        "adaptive_support_receipts_per_run": 4,
        "adaptive_request_attempt_count": null,
        "accepted_width_histogram_ns_per_run": {},
        "canonical_covered_width_histogram_ns_per_run": {},
        "accepted_covered_physical_map_records_per_run": [],
        "release_qualification_counter_identity": {
            "scopes_balanced": true,
            "counters_complete": true,
            "required_scope_observation_complete": true,
            "stage3_scope_entry_count": 1,
            "native_vegetation_et_scope_entry_count": 1,
            "lane_d_scope_entry_count": 1,
        },
        "rejected_support_count": null,
        "physical_map_evaluations_per_covered_support": null,
        "exclusive_subsystem_attribution": {
            "complete": true,
            "total_ns": 100,
            "native_vegetation_et_successor_envelope_ns": 30,
            "stage3_day_preparation_remainder_envelope_ns": 35,
            "lane_d_ns": 15,
            "remaining_runner_ns": 20,
            "exclusive_sum_ns": 100,
            "exclusive_sum_matches_total": true,
        },
        "subsystem_attribution_status": "incomplete",
        "available_overlapping_attribution_ns_per_run": {},
        "lane_d_per_run": {},
        "closure_operands_and_residuals_per_run": {},
        "comparability_identity": {},
    });
    assert!(release_scale_protocol_schema_valid(&record));
    record
        .as_object_mut()
        .expect("protocol object")
        .remove("physical_map_evaluations_per_covered_support");
    assert!(!release_scale_protocol_schema_valid(&record));
}

#[test]
#[ignore = "release-only 1/10/19-OFE qualification: five warm-ups plus 30 measured >=250ms batches per surface"]
#[allow(
    clippy::assertions_on_constants,
    clippy::too_many_lines,
    reason = "the release-mode guard and complete protocol remain visible in the explicit ignored qualification entry point"
)]
fn stage3_laned_release_qualification_matrix_1_10_19_ofe() {
    assert!(
        !cfg!(debug_assertions),
        "qualification must execute under cargo test --release"
    );
    let affinity = release_scale_cpu_affinity();
    let affinity_count = affinity
        .as_deref()
        .and_then(release_scale_cpu_affinity_count);
    assert_eq!(
        affinity_count,
        Some(1),
        "release qualification must be launched on one logical CPU (for example with taskset -c <cpu>)"
    );
    let clock_tick_hz = release_scale_clock_tick_hz();
    assert!(
        clock_tick_hz.is_some(),
        "Linux qualification requires a reliable CLK_TCK conversion for process CPU time"
    );

    if let Some(child_ofe) = std::env::var_os(RELEASE_SCALE_CHILD_OFE_ENV) {
        let ofe_count = child_ofe
            .to_str()
            .expect("child OFE count is UTF-8")
            .parse::<usize>()
            .expect("child OFE count is numeric");
        assert!(
            [1_usize, 10, 19].contains(&ofe_count),
            "child OFE surface is declared"
        );
        let _execution_guard = runner_execution_lock()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        for batch_index in 0..RELEASE_SCALE_WARMUP_BATCHES {
            let sample = release_scale_run_batch(ofe_count, "warmup", batch_index, clock_tick_hz);
            assert!(release_scale_protocol_schema_valid(&sample));
            println!("STAGE3_RELEASE_QUALIFICATION {sample}");
        }
        let mut measured = Vec::with_capacity(RELEASE_SCALE_MEASURED_BATCHES);
        for batch_index in 0..RELEASE_SCALE_MEASURED_BATCHES {
            let sample = release_scale_run_batch(ofe_count, "measured", batch_index, clock_tick_hz);
            assert!(release_scale_protocol_schema_valid(&sample));
            println!("STAGE3_RELEASE_QUALIFICATION {sample}");
            measured.push(sample);
        }
        let mut summary = release_scale_summary(ofe_count, &measured);
        summary["surface_process_isolated"] = serde_json::json!(true);
        println!("STAGE3_RELEASE_QUALIFICATION {summary}");
        println!("STAGE3_RELEASE_SURFACE_RESULT {summary}");
        return;
    }

    let executable = std::env::current_exe().expect("qualification executable path");
    let header = serde_json::json!({
        "record": "stage3_release_scale_protocol_v1",
        "source": release_scale_source_identity(),
        "binary_path": executable,
        "binary_sha256": release_scale_sha256_file(&executable),
        "cpu_affinity": affinity,
        "cpu_affinity_logical_count": affinity_count,
        "process_cpu_method": "linux_proc_self_stat_utime_plus_stime_ticks",
        "clock_tick_hz": clock_tick_hz,
        "warmup_batches": RELEASE_SCALE_WARMUP_BATCHES,
        "measured_batches": RELEASE_SCALE_MEASURED_BATCHES,
        "minimum_batch_ns": RELEASE_SCALE_MIN_BATCH_NS,
        "bootstrap": {
            "method": "ordinary_nonparametric_batch_resampling_with_replacement_type7_percentiles",
            "replicates": RELEASE_SCALE_BOOTSTRAP_REPLICATES,
            "base_seed_u64": RELEASE_SCALE_BOOTSTRAP_SEED,
            "surface_seed_derivation": "base XOR statistic_tag XOR OFE_count; tags wall_median=0, wall_p95=0x9500, cpu_median=0xc000, cpu_p95=0xc950",
            "confidence": 0.95,
        },
        "counter_overhead": {
            "posture": "included_unsubtracted",
            "qualification_audit": "enabled",
            "adaptive_parent_telemetry": "enabled",
            "accepted_canonical_map_and_lane_d_call_counters": "enabled",
            "nested_exclusive_subsystem_profiler": "enabled",
            "one_millisecond_active_rss_sampler": "enabled_and_included_in_process_cpu",
            "disabled_counter_arm": false,
        },
        "wall_clock_method": "thread-local nested exclusive profiler using std::time::Instant monotonic elapsed time; total_elapsed is the reported batch wall sample",
        "timed_scope": "completed_execute_hillslope_run_with_runtime_policy_only; exclusive profiler begins immediately before and is taken immediately after that wall scope",
        "subsystem_attribution_semantics": {
            "native_vegetation_et_fraction": "whole snow-free native vegetation/ET successor transaction envelope; conservative upper bound, not isolated component time",
            "stage3_lse_soil_fraction": "remaining Stage3 day-preparation/LSE/soil transaction envelope after nested scopes; conservative upper bound, not isolated component time",
            "lane_d_fraction": "exclusive Lane-D routed-day envelope",
            "remaining_runner_fraction": "exclusive remainder of the timed runner transaction",
        },
        "surface_isolation": "one fresh child test process per OFE surface",
        "rss_method": {
            "current": "linux_proc_self_status_VmRSS before fixture, sampled every 1ms across fixture/run, and after cleanup per iteration",
            "return_tolerance_kib": "8192 + 1024 * OFE_count",
            "hard_peak": "max(sampled active VmRSS, isolated-process VmHWM); both required",
            "process_lifetime_peak": "linux_proc_self_status_VmHWM supplements active sampling to cover freed transients",
        },
    });
    println!("STAGE3_RELEASE_QUALIFICATION {header}");
    let mut summaries = Vec::new();
    for ofe_count in [1_usize, 10, 19] {
        let output = std::process::Command::new(&executable)
            .args([
                "hillslope::tests::stage3_laned_release_qualification_matrix_1_10_19_ofe",
                "--ignored",
                "--exact",
                "--nocapture",
                "--test-threads=1",
            ])
            .env(RELEASE_SCALE_CHILD_OFE_ENV, ofe_count.to_string())
            .output()
            .expect("launch isolated OFE qualification surface");
        let stdout = String::from_utf8(output.stdout).expect("child stdout is UTF-8");
        let stderr = String::from_utf8(output.stderr).expect("child stderr is UTF-8");
        print!("{stdout}");
        eprint!("{stderr}");
        assert!(
            output.status.success(),
            "isolated {ofe_count}-OFE qualification surface failed"
        );
        let encoded = stdout
            .lines()
            .filter_map(|line| line.strip_prefix("STAGE3_RELEASE_SURFACE_RESULT "))
            .collect::<Vec<_>>();
        assert_eq!(
            encoded.len(),
            1,
            "isolated surface must emit exactly one summary marker"
        );
        let summary: serde_json::Value =
            serde_json::from_str(encoded[0]).expect("parse isolated surface summary");
        assert_eq!(
            summary["ofe_count"].as_u64(),
            Some(u64::try_from(ofe_count).expect("OFE count fits u64"))
        );
        assert_eq!(summary["measured_batch_count"].as_u64(), Some(30));
        assert_eq!(summary["surface_process_isolated"], true);
        summaries.push(summary);
    }
    let decision_surfaces = summaries
        .iter()
        .map(release_scale_surface_from_summary)
        .collect::<Vec<_>>();
    let decision = release_scale_decision(&decision_surfaces);
    let status = decision["status"]
        .as_str()
        .expect("qualification decision status")
        .to_owned();
    let final_record = serde_json::json!({
        "record": "stage3_release_scale_final_v1",
        "completed": true,
        "surfaces": summaries,
        "decision": decision,
    });
    println!("STAGE3_RELEASE_QUALIFICATION {final_record}");
    assert_eq!(
        status, "PASS",
        "release qualification cannot pass when a budget fails or required telemetry is unavailable"
    );
}
