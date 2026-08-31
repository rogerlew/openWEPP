/// Run the real CLI pipeline against a one-day fixture whose explicit
/// topology matches the repository-owned complete scientific test owner.
/// Production still requires the versioned sidecar; this path is scoped by
/// a test-only thread-local admission guard.
fn execute_explicit_stage3_fixture_run(prefix: &str) -> (HillslopeRunReport, PathBuf) {
    let _execution_guard = runner_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let temp_run_dir = prepare_explicit_stage3_fixture_dir(prefix, true);
    let output_dir = temp_run_dir.join("output");

    let report =
        crate::hillslope::snow_stage3_v11_production_seed::with_explicit_test_owner_seed(|| {
            execute_hillslope_run_with_runtime_policy(
                &HillslopeRunRequest {
                    run_dir: temp_run_dir.clone(),
                    run_file: PathBuf::from("case.run"),
                    output_dir,
                    sidecar_policy: SidecarPolicy::Compat,
                    legacy_sidecar_discovery: false,
                    manifest_path: None,
                },
                &["openwepp-cli-hill".to_string()],
                HillslopeRuntimeSelectionPolicy::new(
                    HillslopeRuntimeSelection::DirectProductionExecutor,
                    HillslopeDefaultRuntimeActivation::default(),
                ),
            )
            .expect("explicit Stage-3 fixture run should complete")
        });
    (report, temp_run_dir)
}

fn prepare_explicit_stage3_fixture_dir(prefix: &str, one_day: bool) -> PathBuf {
    let source_fixture_dir = fixture_path("hillslope_run_dir");
    let temp_run_dir = copy_fixture_to_temp(&source_fixture_dir, prefix);

    let run_path = temp_run_dir.join("case.run");
    let runfile = std::fs::read_to_string(&run_path)
        .expect("read explicit Stage-3 fixture runfile")
        .replace("H5.", "H83.")
        .replace(
            "management = \"case.man\"",
            "management = \"case.man.yaml\"",
        );
    std::fs::write(&run_path, runfile).expect("write explicit Stage-3 fixture runfile");

    let native_fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/cancov_forest/marcell_conifer_mn");
    let native_management = native_fixture.join("p8.man.yaml");
    std::fs::copy(native_management, temp_run_dir.join("case.man.yaml"))
        .expect("install explicit native-forest Stage-3 management authority");
    std::fs::copy(
        native_fixture.join("pmetpara.txt"),
        temp_run_dir.join("pmetpara.txt"),
    )
    .expect("install explicit native-forest PMET authority");

    let slope_path = temp_run_dir.join("case.slp");
    let slope = std::fs::read_to_string(&slope_path)
        .expect("read explicit Stage-3 fixture slope")
        .replace("180.0 30.0", "180.0 10.0")
        .replace("3 60.0", "3 10.0");
    std::fs::write(&slope_path, slope).expect("write explicit Stage-3 fixture slope");

    let climate_path = temp_run_dir.join("case.cli");
    let climate_source = std::fs::read_to_string(&climate_path)
        .expect("read explicit Stage-3 fixture climate")
        .replace("45.0 -120.0", "41.1 -120.0");
    let climate = climate_source
        .lines()
        .filter(|line| !one_day || !line.starts_with("2 1 2000 "))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    std::fs::write(&climate_path, climate).expect("write explicit Stage-3 fixture climate");

    let soil_path = temp_run_dir.join("case.sol");
    let soil = std::fs::read_to_string(&soil_path)
        .expect("read explicit Stage-3 fixture soil")
        .replace("CLAY_LOAM 2 ", "CLAY_LOAM 5 ")
        .replace(
            "250 1.30 8.0 1.10 0.28 0.14 33 27 1.8 14 7 0.06 0.43 0.03 1.35 110 0.15 0.30\n",
            concat!(
                "250 1.30 8.0 1.10 0.28 0.14 33 27 1.8 14 7 0.06 0.43 0.03 1.35 110 0.15 0.30\n",
                "500 1.30 8.0 1.10 0.28 0.14 33 27 1.8 14 7 0.06 0.43 0.03 1.35 110 0.15 0.30\n",
                "750 1.30 8.0 1.10 0.28 0.14 33 27 1.8 14 7 0.06 0.43 0.03 1.35 110 0.15 0.30\n",
                "1000 1.30 8.0 1.10 0.28 0.14 33 27 1.8 14 7 0.06 0.43 0.03 1.35 110 0.15 0.30\n",
            ),
        );
    std::fs::write(&soil_path, soil).expect("write explicit Stage-3 fixture soil");

    temp_run_dir
}

#[test]
fn explicit_stage3_runner_fixture_bootstraps_before_day_execution() {
    let _execution_guard = runner_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let temp_run_dir = prepare_explicit_stage3_fixture_dir("explicit_stage3_bootstrap_only", true);
    let request = HillslopeRunRequest {
        run_dir: temp_run_dir.clone(),
        run_file: PathBuf::from("case.run"),
        output_dir: temp_run_dir.join("output"),
        sidecar_policy: SidecarPolicy::Compat,
        legacy_sidecar_discovery: false,
        manifest_path: None,
    };
    let inputs = load_hillslope_run_inputs(&request).expect("explicit fixture inputs");
    let targets = resolve_hillslope_output_targets(&inputs.runfile).expect("output targets");
    let sidecars = resolve_hillslope_sidecars(&request, &inputs, &targets).expect("sidecars");
    let setup = build_static_hillslope_runtime_setup(
        &request,
        &inputs,
        &sidecars,
        HillslopeRuntimeSelection::DirectProductionExecutor,
    )
    .expect("explicit static setup");
    let HillslopeClimateExecutionState {
        per_ofe_lane_areas_m2,
        per_ofe_runoff_publication_geometries,
        lane_context,
        climate_span,
    } = setup.execution_state;
    let climate_request =
        build_hillslope_climate_runtime_request(&inputs.climate).expect("climate request");
    let seed_authority = DirectProductionSeedAuthority::from_typed_inputs(
        &climate_request,
        &inputs,
        &sidecars,
        per_ofe_lane_areas_m2.len(),
        lane_context.lane,
    )
    .expect("explicit production authority");
    let mut frame = build_direct_production_run_frame(&DirectProductionRunFrameBuildInputs {
        output_hillslope_id: targets.output_hillslope_id,
        lane_areas_m2: &per_ofe_lane_areas_m2,
        runoff_publication_geometries: &per_ofe_runoff_publication_geometries,
        day_count: climate_span.days.len(),
        seed_authority: &seed_authority,
    })
    .expect("explicit production frame");
    frame
        .configure_groundwater(
            direct_groundwater_authority_from_gwcoeff(&sidecars.gwcoeff)
                .expect("groundwater authority"),
        )
        .expect("groundwater configuration");
    assert_eq!(frame.lanes[0].subsurface_layers.len(), 6);
    let publication_builder =
        DirectProductionDayInputBuilder::new(&climate_request, &climate_span, &seed_authority)
            .expect("Stage-3 publication input builder");
    let profile_input = publication_builder
        .build_stage3_v11_publication_input(&frame, 0, 0)
        .expect("Stage-3 parsed-soil publication profile");
    let profile = profile_input
        .hydrology_projection_inputs
        .expect("complete Stage-3 WB13 profile authority");
    assert!(profile.profile_depth_m.is_some());
    assert!(profile.profile_porosity_cap_m.is_some());
    assert!(profile.profile_field_capacity_m.is_some());
    assert!(profile.profile_wilting_point_m.is_some());
    assert_eq!(
        profile_input
            .subsurface_compute_inputs
            .expect("profile soil-configuration cross-join")
            .layers
            .len(),
        frame.lanes[0].subsurface_layers.len(),
    );
    let seed =
        crate::hillslope::snow_stage3_v11_production_seed::with_explicit_test_owner_seed(|| {
            crate::hillslope::snow_stage3_v11_production_seed::load_required_or_explicit_test(
                None, &frame,
            )
            .expect("explicit complete owner seed")
        });

    seed.bootstrap(&mut frame)
        .expect("explicit parsed fixture bootstrap");
    assert_eq!(frame.identity.run_id, 83);
    assert_eq!(frame.identity.day_count, 1);
    assert!(frame.snow_stage3_v11_attachment.is_some());
    let _ = std::fs::remove_dir_all(temp_run_dir);
}

const COMPLETE_SEASON_DAY_COUNT: usize = 365;
const COMPLETE_SEASON_COLD_RAMP_DAY_COUNT: usize = 30;

#[derive(Clone, Copy)]
enum CompleteSeasonClimateProfile {
    AccumulationPersistenceMeltout,
    ReappearanceRoutingBgc,
}

fn complete_season_climate_source(profile: CompleteSeasonClimateProfile) -> String {
    complete_season_climate_source_for_days(profile, COMPLETE_SEASON_DAY_COUNT)
}

fn complete_season_climate_source_for_days(
    profile: CompleteSeasonClimateProfile,
    day_count: usize,
) -> String {
    use std::fmt::Write as _;

    assert!((1..=COMPLETE_SEASON_DAY_COUNT).contains(&day_count));

    let latitude_degrees = match profile {
        CompleteSeasonClimateProfile::AccumulationPersistenceMeltout => 84.7,
        CompleteSeasonClimateProfile::ReappearanceRoutingBgc => 41.1,
    };
    let mut source = format!(
        "5.30\n1 0 0\nTEST STATION 1500\nDAY MON YEAR PRCP STMDUR TIMEP IP TMAX TMIN RAD VWIND WIND TDPT\n{latitude_degrees:.1} -120.0 1225.0 30 2000 1 CLIGEN 5.30 --seed 123\nMONTHLY MAX TEMP HEADER\n1 2 3 4 5 6 7 8 9 10 11 12\nMONTHLY MIN TEMP HEADER\n-5 -4 -3 -2 -1 0 1 2 3 4 5 6\nMONTHLY RAD HEADER\n100 101 102 103 104 105 106 107 108 109 110 111\nMONTHLY RAIN HEADER\n10 11 12 13 14 15 16 17 18 19 20 21\nDAILY HEADER\nDAILY UNITS\n",
    );
    let month_days = [31_usize, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut month = 1_usize;
    let mut day_of_month = 1_usize;
    for day_index in 0..day_count {
        let (precipitation_mm, tmax_c, tmin_c, radiation, dewpoint_c) = match profile {
            CompleteSeasonClimateProfile::AccumulationPersistenceMeltout => {
                if day_index < COMPLETE_SEASON_COLD_RAMP_DAY_COUNT {
                    // The sealed fresh owner is an authenticated dry 273.15 K
                    // equilibrium predecessor. Enter winter continuously from
                    // the already-qualified warm forcing instead of inventing
                    // an incompatible subfreezing genesis state.
                    let cold_fraction = complete_season_cold_ramp_fraction(day_index);
                    let tmax_c = 12.0 - 16.0 * cold_fraction;
                    let tmin_c = 2.0 - 13.0 * cold_fraction;
                    (0.0, tmax_c, tmin_c, 0.0, tmin_c - 1.0)
                } else if matches!(day_index, 35 | 70 | 100) {
                    (35.0, -3.0, -9.0, 0.0, -11.0)
                } else if day_index < 130 {
                    (0.0, -4.0, -11.0, 0.0, -12.0)
                } else if day_index < 200 {
                    (0.0, 14.0, 4.0, 0.0, 1.0)
                } else {
                    (0.0, 20.0, 8.0, 0.0, 5.0)
                }
            }
            CompleteSeasonClimateProfile::ReappearanceRoutingBgc => {
                if day_index < COMPLETE_SEASON_COLD_RAMP_DAY_COUNT {
                    let cold_fraction = complete_season_cold_ramp_fraction(day_index);
                    let tmax_c = 12.0 - 16.0 * cold_fraction;
                    let tmin_c = 2.0 - 13.0 * cold_fraction;
                    (
                        0.0,
                        tmax_c,
                        tmin_c,
                        200.0 - 110.0 * cold_fraction,
                        tmin_c - 1.0,
                    )
                } else if day_index == 35 {
                    (45.0, -4.0, -10.0, 70.0, -12.0)
                } else if day_index < 120 {
                    (0.0, -3.0, -9.0, 100.0, -11.0)
                } else if day_index < 190 {
                    (0.0, 16.0, 5.0, 450.0, 2.0)
                } else if day_index == 220 {
                    (30.0, -5.0, -12.0, 60.0, -13.0)
                } else if day_index == 275 {
                    (80.0, 18.0, 9.0, 260.0, 7.0)
                } else {
                    (0.0, 15.0, 5.0, 340.0, 2.0)
                }
            }
        };
        let storm_duration_h = if precipitation_mm > 0.0 { 2.0 } else { 0.0 };
        let peak_intensity = if precipitation_mm > 0.0 {
            precipitation_mm / storm_duration_h
        } else {
            0.0
        };
        writeln!(
            source,
            "{day_of_month} {month} 2000 {precipitation_mm:.3} {storm_duration_h:.3} 0.25 {peak_intensity:.3} {tmax_c:.3} {tmin_c:.3} {radiation:.3} 3.0 180.0 {dewpoint_c:.3}"
        )
        .expect("writing climate rows to a String cannot fail");
        day_of_month += 1;
        if day_of_month > month_days[month - 1] {
            month += 1;
            day_of_month = 1;
        }
    }
    source
}

fn complete_season_cold_ramp_fraction(day_index: usize) -> f64 {
    let numerator = u32::try_from(day_index).expect("cold-ramp day index fits u32");
    let denominator = u32::try_from(COMPLETE_SEASON_COLD_RAMP_DAY_COUNT - 1)
        .expect("cold-ramp day count fits u32");
    f64::from(numerator) / f64::from(denominator)
}

#[allow(clippy::too_many_lines)]
fn run_stage3_archive_residency_progression(day_count: usize, prefix: &str) {
    let _execution_guard = runner_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let run_dir = prepare_one_ofe_complete_season_fixture(
        prefix,
        CompleteSeasonClimateProfile::AccumulationPersistenceMeltout,
    );
    std::fs::write(
        run_dir.join("case.cli"),
        complete_season_climate_source_for_days(
            CompleteSeasonClimateProfile::AccumulationPersistenceMeltout,
            day_count,
        ),
    )
    .expect("write bounded-residency climate");

    crate::hillslope::snow_stage3_v11_qualification_audit::begin();
    let report =
        crate::hillslope::snow_stage3_v11_production_seed::with_explicit_adaptive_test_owner_seed(
            || {
                execute_hillslope_run_with_runtime_policy(
                    &HillslopeRunRequest {
                        run_dir: run_dir.clone(),
                        run_file: PathBuf::from("case.run"),
                        output_dir: run_dir.join("output"),
                        sidecar_policy: SidecarPolicy::Compat,
                        legacy_sidecar_discovery: false,
                        manifest_path: None,
                    },
                    &["openwepp-cli-hill".to_string()],
                    HillslopeRuntimeSelectionPolicy::new(
                        HillslopeRuntimeSelection::DirectProductionExecutor,
                        HillslopeDefaultRuntimeActivation::default(),
                    ),
                )
                .expect("bounded archive residency runner")
            },
        );
    let audit = crate::hillslope::snow_stage3_v11_qualification_audit::take();
    let snapshot = audit
        .committed_snapshot
        .as_ref()
        .expect("sealed bounded qualification snapshot");
    snapshot
        .validate()
        .expect("valid bounded qualification snapshot");
    assert_eq!(snapshot.committed_day_count, day_count);
    assert_eq!(snapshot.next_day_index, day_count);
    assert_eq!(snapshot.total_parent_support_count, (day_count * 48) as u64);

    let manifest: serde_json::Value = serde_json::from_slice(
        &std::fs::read(&report.manifest_path).expect("read bounded manifest"),
    )
    .expect("parse bounded manifest");
    let archive = &manifest["stage3_evidence_archive"];
    assert_eq!(archive["record_count"].as_u64(), Some(day_count as u64));
    assert_eq!(
        archive["archived_day_count"].as_u64(),
        Some(day_count as u64)
    );
    assert_eq!(
        archive["archive_manifest"]["committed_day_count"].as_u64(),
        Some(day_count as u64)
    );
    let canonical_uncompressed_bytes = archive["canonical_uncompressed_bytes"]
        .as_u64()
        .expect("canonical uncompressed archive byte count");
    let stored_record_bytes = archive["stored_record_bytes"]
        .as_u64()
        .expect("compressed archive record byte count");
    assert!(canonical_uncompressed_bytes > 0);
    assert!(
        stored_record_bytes
            .checked_mul(4)
            .is_some_and(|scaled| scaled <= canonical_uncompressed_bytes),
        "compressed archive must stay at or below 25% of canonical evidence bytes"
    );
    let archive_path = archive["output_path"]
        .as_str()
        .map(PathBuf::from)
        .expect("archive output path");
    assert!(archive_path.is_file(), "durably promoted archive spool");
    let archive_bytes = std::fs::metadata(&archive_path)
        .expect("archive metadata")
        .len();
    let expected_archive_bytes =
        31_u64 + (day_count as u64) * (1 + 8 + 8 + 32) + stored_record_bytes + (1 + 8 + 32);
    assert_eq!(archive_bytes, expected_archive_bytes);
    let peak_rss_kib = std::fs::read_to_string("/proc/self/status")
        .expect("read process status for archive residency gate")
        .lines()
        .find_map(|line| {
            line.strip_prefix("VmHWM:")
                .and_then(|value| value.split_whitespace().next())
                .and_then(|value| value.parse::<u64>().ok())
        })
        .expect("VmHWM process status field");
    assert!(
        peak_rss_kib < 1_048_576,
        "archive-backed progression must remain below 1 GiB peak RSS"
    );
    eprintln!(
        "STAGE3_ARCHIVE_RESIDENCY_CHECKPOINT days={day_count} archived={} parents={} canonical_bytes={canonical_uncompressed_bytes} stored_record_bytes={stored_record_bytes} archive_bytes={archive_bytes} peak_rss_kib={peak_rss_kib}",
        snapshot.committed_day_count, snapshot.total_parent_support_count,
    );
    let _ = std::fs::remove_dir_all(run_dir);
}

#[test]
#[ignore = "real two-day archive/accumulator/spool residency qualification"]
fn cqr_stage3_two_day_archive_residency_progression() {
    run_stage3_archive_residency_progression(2, "stage3_two_day_archive_residency");
}

#[test]
#[ignore = "real sixty-day archive/accumulator/spool residency qualification"]
fn cqr_stage3_sixty_day_archive_residency_progression() {
    run_stage3_archive_residency_progression(60, "stage3_sixty_day_archive_residency");
}

#[test]
fn complete_season_climate_has_authenticated_warm_genesis_and_gradual_cold_onset() {
    for profile in [
        CompleteSeasonClimateProfile::AccumulationPersistenceMeltout,
        CompleteSeasonClimateProfile::ReappearanceRoutingBgc,
    ] {
        let source = complete_season_climate_source(profile);
        let daily = source
            .lines()
            .skip_while(|line| *line != "DAILY UNITS")
            .skip(1)
            .map(|line| {
                line.split_whitespace()
                    .map(str::parse::<f64>)
                    .collect::<Result<Vec<_>, _>>()
                    .expect("numeric complete-season climate row")
            })
            .collect::<Vec<_>>();
        assert_eq!(daily.len(), COMPLETE_SEASON_DAY_COUNT);
        assert_eq!(
            daily[0][3].to_bits(),
            0.0_f64.to_bits(),
            "genesis predecessor must be dry"
        );
        assert_eq!((daily[0][7], daily[0][8]), (12.0, 2.0));
        assert!(
            daily[..COMPLETE_SEASON_COLD_RAMP_DAY_COUNT]
                .windows(2)
                .all(|pair| pair[1][7] <= pair[0][7]
                    && pair[1][8] <= pair[0][8]
                    && pair[0][7] - pair[1][7] < 0.6
                    && pair[0][8] - pair[1][8] < 0.5)
        );
        assert!(daily.iter().filter(|row| row[7] < 0.0).count() >= 90);
        assert!(
            daily
                .iter()
                .skip(COMPLETE_SEASON_COLD_RAMP_DAY_COUNT)
                .any(|row| row[3] > 0.0 && row[7] < 0.0 && row[8] < 0.0)
        );
    }
}

fn prepare_one_ofe_complete_season_fixture(
    prefix: &str,
    profile: CompleteSeasonClimateProfile,
) -> PathBuf {
    let run_dir = prepare_explicit_stage3_fixture_dir(prefix, false);
    std::fs::write(
        run_dir.join("case.cli"),
        complete_season_climate_source(profile),
    )
    .expect("write complete-season climate");
    run_dir
}

fn prepare_two_ofe_complete_season_fixture(prefix: &str) -> PathBuf {
    let source =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/erosion_multi_ofe_p102");
    let run_dir = copy_fixture_to_temp(&source, prefix);

    let runfile = std::fs::read_to_string(run_dir.join("p102.run"))
        .expect("read two-OFE runfile")
        .replace("H102.", "H83.")
        .replace(
            "pass_parquet = \"output/H83.pass.parquet\"",
            concat!(
                "pass_parquet = \"output/H83.pass.parquet\"\n",
                "wat = \"output/H83.wat.parquet\"\n",
                "plot = \"output/H83.plot.parquet\"",
            ),
        );
    std::fs::write(run_dir.join("case.run"), runfile).expect("write two-OFE runfile");
    std::fs::write(
        run_dir.join("p102.cli"),
        complete_season_climate_source(CompleteSeasonClimateProfile::ReappearanceRoutingBgc),
    )
    .expect("write two-OFE season climate");
    let management_source = std::fs::read_to_string(run_dir.join("p102.man"))
        .expect("read two-OFE management")
        .replace("50 # sim_years", "1 # sim_years");
    let (management_prefix, _) = management_source
        .split_once("50 # number of years in a single rotation")
        .expect("two-OFE fixture rotation-year marker");
    let management = format!(
        "{management_prefix}{}",
        concat!(
            "1 # number of years in a single rotation\n",
            "   1 \t# plants/year; <Year: 1 - OFE: 1>  (nycrop)\n",
            "      1 \t# yearly index <Year 1>\n",
            "   1 \t# plants/year; <Year: 1 - OFE: 2>  (nycrop)\n",
            "      2 \t# yearly index <OFE2_Year 1>\n",
        )
    );
    std::fs::write(run_dir.join("p102.man"), management).expect("write two-OFE management");
    std::fs::write(
        run_dir.join("p102.slp"),
        concat!(
            "97.5\n2\n",
            "180.0 10.0\n3 10.0\n0.0 0.0200 0.6 0.0800 1.0 0.0600\n",
            "180.0 10.0\n3 20.0\n0.0 0.0600 0.5 0.0400 1.0 0.0300\n",
        ),
    )
    .expect("write exact two-OFE slope topology");

    let soil = std::fs::read_to_string(run_dir.join("p102.sol"))
        .expect("read two-OFE soil")
        .replace("'GR-L'\t 4\t", "'GR-L'\t 5\t");
    let mut expanded = String::new();
    for line in soil.lines() {
        expanded.push_str(line);
        expanded.push('\n');
        if line.trim_start().starts_with("400.0\t") {
            expanded.push_str(&line.replacen("400.0", "1000.0", 1));
            expanded.push('\n');
        }
    }
    std::fs::write(run_dir.join("p102.sol"), expanded).expect("write six-layer two-OFE soil");
    run_dir
}

#[test]
fn explicit_two_ofe_stage3_runner_fixture_bootstraps_exact_topology() {
    let _execution_guard = runner_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let run_dir = prepare_two_ofe_complete_season_fixture("explicit_stage3_two_ofe_bootstrap");
    let request = HillslopeRunRequest {
        run_dir: run_dir.clone(),
        run_file: PathBuf::from("case.run"),
        output_dir: run_dir.join("output"),
        sidecar_policy: SidecarPolicy::Compat,
        legacy_sidecar_discovery: false,
        manifest_path: None,
    };
    let inputs = load_hillslope_run_inputs(&request).expect("two-OFE fixture inputs");
    let targets = resolve_hillslope_output_targets(&inputs.runfile).expect("two-OFE targets");
    let sidecars = resolve_hillslope_sidecars(&request, &inputs, &targets).expect("sidecars");
    let setup = build_static_hillslope_runtime_setup(
        &request,
        &inputs,
        &sidecars,
        HillslopeRuntimeSelection::DirectProductionExecutor,
    )
    .expect("two-OFE static setup");
    let HillslopeClimateExecutionState {
        per_ofe_lane_areas_m2,
        per_ofe_runoff_publication_geometries,
        lane_context,
        climate_span,
    } = setup.execution_state;
    assert_eq!(per_ofe_lane_areas_m2, vec![100.0, 200.0]);
    assert_eq!(climate_span.days.len(), COMPLETE_SEASON_DAY_COUNT);
    let climate_request =
        build_hillslope_climate_runtime_request(&inputs.climate).expect("climate request");
    let seed_authority = DirectProductionSeedAuthority::from_typed_inputs(
        &climate_request,
        &inputs,
        &sidecars,
        2,
        lane_context.lane,
    )
    .expect("two-OFE seed authority");
    let mut frame = build_direct_production_run_frame(&DirectProductionRunFrameBuildInputs {
        output_hillslope_id: targets.output_hillslope_id,
        lane_areas_m2: &per_ofe_lane_areas_m2,
        runoff_publication_geometries: &per_ofe_runoff_publication_geometries,
        day_count: climate_span.days.len(),
        seed_authority: &seed_authority,
    })
    .expect("two-OFE production frame");
    frame
        .configure_groundwater(
            direct_groundwater_authority_from_gwcoeff(&sidecars.gwcoeff)
                .expect("groundwater authority"),
        )
        .expect("groundwater configuration");
    assert_eq!(
        frame
            .lanes
            .iter()
            .map(|lane| (lane.lane_id, lane.area_m2, lane.subsurface_layers.len()))
            .collect::<Vec<_>>(),
        vec![(1, 100.0, 6), (2, 200.0, 6)]
    );
    let seed =
        crate::hillslope::snow_stage3_v11_production_seed::with_explicit_two_ofe_test_owner_seed(
            || {
                crate::hillslope::snow_stage3_v11_production_seed::load_required_or_explicit_test(
                    None, &frame,
                )
                .expect("two-OFE complete owner seed")
            },
        );
    seed.bootstrap(&mut frame)
        .expect("two-OFE parsed fixture bootstrap");
    assert_eq!(frame.identity.run_id, 83);
    assert_eq!(frame.identity.lane_count, 2);
    assert_eq!(frame.identity.day_count, COMPLETE_SEASON_DAY_COUNT);
    assert!(frame.snow_stage3_v11_attachment.is_some());
    let _ = std::fs::remove_dir_all(run_dir);
}

fn assert_complete_season_jit_supports(
    audit: &crate::hillslope::snow_stage3_v11_qualification_audit::RunnerStage3V11QualificationAuditV1,
) {
    use openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::{
        STAGE3_V11_DAY_NS, STAGE3_V11_PARENT_SUPPORT_COUNT, STAGE3_V11_PARENT_SUPPORT_NS,
    };

    assert_eq!(
        audit.support_chronology_by_day.len(),
        COMPLETE_SEASON_DAY_COUNT
    );
    for day_index in 0..COMPLETE_SEASON_DAY_COUNT {
        let supports = audit
            .support_chronology_by_day
            .get(&day_index)
            .expect("every season day is prepared exactly once");
        assert_eq!(supports.len(), STAGE3_V11_PARENT_SUPPORT_COUNT);
        let day_start = (day_index as u128) * STAGE3_V11_DAY_NS;
        assert_eq!(supports[0].start_ns().get(), day_start);
        assert_eq!(supports[47].end_ns().get(), day_start + STAGE3_V11_DAY_NS);
        assert!(supports.windows(2).all(|pair| {
            pair[0].end_ns() == pair[1].start_ns()
                && pair[0].duration_ns() == STAGE3_V11_PARENT_SUPPORT_NS
        }));
    }
}

fn read_wat_f64_column(path: &Path, name: &str) -> Vec<f64> {
    use arrow_array::Float64Array;
    use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

    ParquetRecordBatchReaderBuilder::try_new(
        std::fs::File::open(path).expect("open season WAT parquet"),
    )
    .expect("season WAT parquet metadata")
    .build()
    .expect("season WAT parquet reader")
    .flat_map(|batch| {
        let batch = batch.expect("valid season WAT batch");
        let column_index = batch.schema().index_of(name).expect("named WAT column");
        batch
            .column(column_index)
            .as_any()
            .downcast_ref::<Float64Array>()
            .expect("f64 WAT column")
            .values()
            .to_vec()
    })
    .collect()
}

#[test]
#[ignore = "complete-season adaptive qualification; run only after the short chronology gate releases"]
fn complete_season_adaptive_accumulation_persistent_snow_and_meltout_runner_fixture() {
    let _execution_guard = runner_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let run_dir = prepare_one_ofe_complete_season_fixture(
        "stage3_complete_season_accumulation_meltout",
        CompleteSeasonClimateProfile::AccumulationPersistenceMeltout,
    );
    let started = std::time::Instant::now();
    crate::hillslope::snow_stage3_v11_qualification_audit::begin();
    let report =
        crate::hillslope::snow_stage3_v11_production_seed::with_explicit_adaptive_test_owner_seed(
            || {
                execute_hillslope_run_with_runtime_policy(
                    &HillslopeRunRequest {
                        run_dir: run_dir.clone(),
                        run_file: PathBuf::from("case.run"),
                        output_dir: run_dir.join("output"),
                        sidecar_policy: SidecarPolicy::Compat,
                        legacy_sidecar_discovery: false,
                        manifest_path: None,
                    },
                    &["openwepp-cli-hill".to_string()],
                    HillslopeRuntimeSelectionPolicy::new(
                        HillslopeRuntimeSelection::DirectProductionExecutor,
                        HillslopeDefaultRuntimeActivation::default(),
                    ),
                )
                .expect("complete-season adaptive runner")
            },
        );
    let elapsed = started.elapsed();
    let audit = crate::hillslope::snow_stage3_v11_qualification_audit::take();
    assert_complete_season_jit_supports(&audit);
    let snapshot = audit
        .committed_snapshot
        .as_ref()
        .expect("sealed complete-season qualification snapshot");
    snapshot.validate().expect("sealed qualification evidence");
    assert_eq!(snapshot.next_day_index, COMPLETE_SEASON_DAY_COUNT);
    assert_eq!(snapshot.committed_day_count, COMPLETE_SEASON_DAY_COUNT);
    assert_eq!(
        snapshot.total_parent_support_count,
        (COMPLETE_SEASON_DAY_COUNT * 48) as u64
    );
    assert_eq!(
        snapshot.adaptive_support_receipt_count + snapshot.snow_free_parent_support_count,
        snapshot.total_parent_support_count
    );
    assert!(snapshot.snow_free_successor_receipt_count > 0);
    assert!(snapshot.lanes[0].cumulative_snowfall_kg_m2 > 0.0);
    assert!(snapshot.lanes[0].cumulative_melt_kg_m2 > 0.0);
    assert!(
        snapshot.terminal_event_count > 0,
        "sealed meltout event receipt"
    );
    assert!(!snapshot.soil_thermal_ofes[0].ordered_layers.is_empty());
    assert!(
        report
            .optional_outputs
            .iter()
            .any(|path| path.ends_with("H83.wat.parquet")),
        "the real runner must retain the complete-season WAT publication"
    );

    let wat = run_dir.join("output/H83.wat.parquet");
    let snow = read_wat_f64_column(&wat, "snow_water");
    let frost = read_wat_f64_column(&wat, "frozwt");
    assert_eq!(snow.len(), COMPLETE_SEASON_DAY_COUNT);
    let first_snow = snow
        .iter()
        .position(|value| *value > 0.0)
        .expect("accumulation");
    assert!(
        snow[first_snow..]
            .windows(20)
            .any(|window| window.iter().all(|value| *value > 0.0)),
        "snow must persist across representative cold supports"
    );
    assert!(
        snow.iter().skip(first_snow + 1).any(|value| *value == 0.0),
        "the accumulated seasonal snow must later melt out"
    );
    assert!(
        frost.iter().any(|value| *value > 0.0),
        "seasonal frost interaction"
    );
    eprintln!(
        "STAGE3_COMPLETE_SEASON_QUALIFICATION profile=accumulation-persistence-meltout days={} supports={} elapsed={elapsed:?}",
        COMPLETE_SEASON_DAY_COUNT,
        COMPLETE_SEASON_DAY_COUNT * 48,
    );
    let _ = std::fs::remove_dir_all(run_dir);
}

#[test]
#[ignore = "complete-season adaptive qualification; run only after the short chronology gate releases"]
#[allow(clippy::too_many_lines)]
fn complete_season_adaptive_reappearance_routed_water_and_bgc_runner_fixture() {
    let _execution_guard = runner_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let run_dir =
        prepare_two_ofe_complete_season_fixture("stage3_complete_season_reappearance_routing_bgc");
    let started = std::time::Instant::now();
    crate::hillslope::snow_stage3_v11_qualification_audit::begin();
    let report =
        crate::hillslope::snow_stage3_v11_production_seed::with_explicit_two_ofe_test_owner_seed(
            || {
                execute_hillslope_run_with_runtime_policy(
                    &HillslopeRunRequest {
                        run_dir: run_dir.clone(),
                        run_file: PathBuf::from("case.run"),
                        output_dir: run_dir.join("output"),
                        sidecar_policy: SidecarPolicy::Compat,
                        legacy_sidecar_discovery: false,
                        manifest_path: None,
                    },
                    &["openwepp-cli-hill".to_string()],
                    HillslopeRuntimeSelectionPolicy::new(
                        HillslopeRuntimeSelection::DirectProductionExecutor,
                        HillslopeDefaultRuntimeActivation::default(),
                    ),
                )
                .expect("two-OFE complete-season adaptive runner")
            },
        );
    let elapsed = started.elapsed();
    let audit = crate::hillslope::snow_stage3_v11_qualification_audit::take();
    assert_complete_season_jit_supports(&audit);
    let snapshot = audit
        .committed_snapshot
        .as_ref()
        .expect("sealed two-OFE season qualification snapshot");
    snapshot.validate().expect("sealed qualification evidence");
    assert_eq!(snapshot.next_day_index, COMPLETE_SEASON_DAY_COUNT);
    assert_eq!(snapshot.lanes.len(), 2);
    assert_eq!(snapshot.soil_thermal_ofes.len(), 2);
    assert!(snapshot.terminal_event_count > 0);
    assert!(
        snapshot.lanes.iter().all(|lane| {
            lane.cumulative_snowfall_kg_m2 > 0.0 && lane.cumulative_melt_kg_m2 > 0.0
        })
    );
    assert!(snapshot.routed_runoff_mass_kg_m2 > 0.0);
    assert!(snapshot.upstream_runon_mass_kg_m2 > 0.0);
    assert!(
        snapshot
            .surface_flow_by_route
            .keys()
            .any(|route| { route.source_ofe_id == "ofe-1" && route.destination_ofe_id == "ofe-2" })
    );
    let bgc_delta = snapshot.biogeochemistry_delta;
    let bgc_beginning = snapshot.beginning_biogeochemistry;
    let bgc_ending = snapshot.ending_biogeochemistry;
    for (beginning, delta, ending) in [
        (
            bgc_beginning.ammonium_n,
            bgc_delta.ammonium_n,
            bgc_ending.ammonium_n,
        ),
        (
            bgc_beginning.nitrate_n,
            bgc_delta.nitrate_n,
            bgc_ending.nitrate_n,
        ),
        (
            bgc_beginning.receiver_carbon,
            bgc_delta.receiver_carbon,
            bgc_ending.receiver_carbon,
        ),
        (
            bgc_beginning.receiver_nitrogen,
            bgc_delta.receiver_nitrogen,
            bgc_ending.receiver_nitrogen,
        ),
        (
            bgc_beginning.receiver_dry_matter,
            bgc_delta.receiver_dry_matter,
            bgc_ending.receiver_dry_matter,
        ),
    ] {
        assert!((beginning + delta - ending).abs() <= 1.0e-12);
    }
    assert!(
        bgc_delta.ammonium_n != 0.0
            || bgc_delta.nitrate_n != 0.0
            || bgc_delta.receiver_carbon != 0.0
            || bgc_delta.receiver_nitrogen != 0.0
            || bgc_delta.receiver_dry_matter != 0.0,
        "the real season must advance a positive BGC follower operand"
    );
    assert!(snapshot.ending_biogeochemistry_last_transaction_id > 0);
    let beginning_owner = openwepp_persisted_restart_v1::restart_authority_two_ofe_owner_fixture();
    let beginning_soil = beginning_owner
        .runtime
        .shadow
        .restart_authority_soil_thermal()
        .expect("qualification fixture retains the explicit V1 soil owner");
    assert!(
        snapshot
            .soil_thermal_ofes
            .iter()
            .zip(&beginning_soil.ofes)
            .any(|(ending_ofe, beginning_ofe)| {
                ending_ofe
                    .ordered_layers
                    .iter()
                    .zip(&beginning_ofe.ordered_layers)
                    .any(|(ending, beginning)| {
                        ending.temperature_k.to_bits() != beginning.temperature_k.to_bits()
                            || ending.enthalpy_j_m2.to_bits()
                                != beginning.enthalpy_j_m2_ofe_ground.to_bits()
                    })
            })
    );
    assert!(
        report
            .optional_outputs
            .iter()
            .any(|path| path.ends_with("H83.wat.parquet"))
    );

    let wat = run_dir.join("output/H83.wat.parquet");
    let snow = read_wat_f64_column(&wat, "snow_water");
    let frost = read_wat_f64_column(&wat, "frozwt");
    let routed = read_wat_f64_column(&wat, "qofe");
    assert_eq!(snow.len(), 2 * COMPLETE_SEASON_DAY_COUNT);
    let snow_days = snow
        .chunks_exact(2)
        .map(|lanes| lanes.iter().copied().fold(0.0_f64, f64::max))
        .collect::<Vec<_>>();
    let first_snow = snow_days
        .iter()
        .position(|value| *value > 0.0)
        .expect("initial accumulation");
    let first_meltout = snow_days
        .iter()
        .enumerate()
        .skip(first_snow + 1)
        .find_map(|(day, value)| (*value == 0.0).then_some(day))
        .expect("first meltout");
    assert!(
        snow_days
            .iter()
            .skip(first_meltout + 1)
            .any(|value| *value > 0.0),
        "later solid precipitation must reappear through the same owner"
    );
    assert!(
        frost.iter().any(|value| *value > 0.0),
        "frost/soil season evidence"
    );
    assert!(
        routed.iter().any(|value| *value > 0.0),
        "two-OFE routed water evidence"
    );
    eprintln!(
        "STAGE3_COMPLETE_SEASON_QUALIFICATION profile=reappearance-routing-bgc days={} lanes=2 supports={} elapsed={elapsed:?}",
        COMPLETE_SEASON_DAY_COUNT,
        COMPLETE_SEASON_DAY_COUNT * 48,
    );
    let _ = std::fs::remove_dir_all(run_dir);
}
