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
        "5.30\n1 0 0\nTEST STATION 1500\nDAY MON YEAR PRCP STMDUR TIMEP IP TMAX TMIN RAD VWIND WIND TDPT\n{latitude_degrees:.1} -120.0 1225.0 30 2001 1 CLIGEN 5.30 --seed 123\nMONTHLY MAX TEMP HEADER\n1 2 3 4 5 6 7 8 9 10 11 12\nMONTHLY MIN TEMP HEADER\n-5 -4 -3 -2 -1 0 1 2 3 4 5 6\nMONTHLY RAD HEADER\n100 101 102 103 104 105 106 107 108 109 110 111\nMONTHLY RAIN HEADER\n10 11 12 13 14 15 16 17 18 19 20 21\nDAILY HEADER\nDAILY UNITS\n",
    );
    let month_days = [31_usize, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
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
            "{day_of_month} {month} 2001 {precipitation_mm:.3} {storm_duration_h:.3} 0.25 {peak_intensity:.3} {tmax_c:.3} {tmin_c:.3} {radiation:.3} 3.0 180.0 {dewpoint_c:.3}"
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

#[test]
fn complete_season_climate_source_covers_one_exact_nonleap_calendar_year() {
    for profile in [
        CompleteSeasonClimateProfile::AccumulationPersistenceMeltout,
        CompleteSeasonClimateProfile::ReappearanceRoutingBgc,
    ] {
        let source = complete_season_climate_source(profile);
        let rows = source
            .lines()
            .skip_while(|line| *line != "DAILY UNITS")
            .skip(1)
            .collect::<Vec<_>>();
        assert_eq!(rows.len(), COMPLETE_SEASON_DAY_COUNT);
        assert!(rows[0].starts_with("1 1 2001 "));
        assert!(rows[COMPLETE_SEASON_DAY_COUNT - 1].starts_with("31 12 2001 "));
        let mut row_index = 0_usize;
        for (month_index, day_count) in [31_usize, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
            .into_iter()
            .enumerate()
        {
            for day in 1..=day_count {
                let mut fields = rows[row_index].split_whitespace();
                assert_eq!(
                    fields.next().and_then(|value| value.parse().ok()),
                    Some(day)
                );
                assert_eq!(
                    fields.next().and_then(|value| value.parse().ok()),
                    Some(month_index + 1),
                );
                assert_eq!(
                    fields.next().and_then(|value| value.parse().ok()),
                    Some(2001)
                );
                row_index += 1;
            }
        }
        assert_eq!(row_index, COMPLETE_SEASON_DAY_COUNT);
    }
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

#[test]
fn accepted_stage3_real_runner_routes_lane_d_and_publishes_summary() {
    const PROBE: &str = "OPENWEPP_STAGE3_LANED_REAL_CONSUMER_PROBE";
    if std::env::var_os(PROBE).is_none() {
        let test_name =
            "hillslope::tests::accepted_stage3_real_runner_routes_lane_d_and_publishes_summary";
        let status = std::process::Command::new(std::env::current_exe().expect("test executable"))
            .args(["--exact", test_name, "--nocapture"])
            .env(PROBE, "1")
            .env("RUST_MIN_STACK", "67108864")
            .env_remove("OPENWEPP_LANED_ACTIVE")
            .env_remove("OPENWEPP_LANED_ACTIVE_DISABLE")
            .env_remove("OPENWEPP_LANED_ACTIVE_IMPLICIT")
            .env_remove("OPENWEPP_LANED_SHADOW")
            .env_remove("OPENWEPP_LANED_SHADOW_PROFILE")
            .env_remove("OPENWEPP_LANED_ACTIVE_TRACE")
            .env_remove("OPENWEPP_LANED_ACTIVE_TRACE_DETAIL")
            .env_remove("OPENWEPP_LANED_ACTIVE_STEP_TRACE")
            .env_remove("OPENWEPP_LANED_ACTIVE_MAX_DT_S")
            .env_remove("OPENWEPP_LANED_ACTIVE_MESH_TARGET_DX_M")
            .status()
            .expect("real Stage-3 Lane-D probe process");
        assert!(
            status.success(),
            "authenticated default-eligibility Stage-3 Lane-D subprocess failed"
        );
        return;
    }

    let _execution_guard = runner_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    assert_authenticated_lane_d_production_seam_present();
    assert_real_stage3_lane_d_scale_and_publication_sensitivity();
    let run_dir =
        prepare_native_stage3_lane_d_scale_fixture("stage3_laned_real_consumer", &[100.0, 200.0]);
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
    .expect("author exact two-OFE Stage-3 owner seed");
    assert_authored_stage3_owner_cardinality(&request, &seed_path, 2);
    crate::hillslope::snow_stage3_v11_qualification_audit::begin();
    let report = execute_hillslope_run_with_runtime_policy(
        &request,
        &["openwepp-cli-hill".to_string()],
        HillslopeRuntimeSelectionPolicy::new(
            HillslopeRuntimeSelection::DirectProductionExecutor,
            HillslopeDefaultRuntimeActivation::default(),
        ),
    )
    .expect("accepted Stage-3 real Lane-D run");
    let stage3_audit = crate::hillslope::snow_stage3_v11_qualification_audit::take();
    let stage3 = stage3_audit
        .committed_snapshot
        .as_ref()
        .expect("accepted Stage-3 committed qualification snapshot");
    stage3
        .validate()
        .expect("accepted Stage-3 committed qualification evidence");
    assert!(
        stage3_audit
            .attachment_adoption
            .native_inactive_prefix_validation_count
            > 0,
        "the real represented-snow transition must validate an inactive prefix"
    );
    assert!(
        stage3_audit
            .attachment_adoption
            .native_inactive_prefix_receipt_counts
            .iter()
            .all(|count| *count > 0),
        "every real prefix validation must authenticate positive support"
    );
    assert_eq!(
        stage3_audit.attachment_adoption.successful_adoption_count, 1,
        "the actual validated outer-frame installation must adopt exactly once"
    );
    assert_eq!(
        stage3_audit
            .attachment_adoption
            .accepted_history_append_count,
        stage3.accepted_publication_support_count,
        "every accepted real-parent support must cross the actual history append"
    );
    assert_eq!(
        stage3_audit
            .attachment_adoption
            .appended_support_sha256
            .last(),
        stage3_audit
            .attachment_adoption
            .accepted_support_sha256
            .first(),
        "the installed parent must adopt the exact tail produced by accepted-history append"
    );
    assert_eq!(
        stage3_audit
            .attachment_adoption
            .accepted_support_sha256
            .len(),
        1,
        "the adopted frame must retain the exact accepted-history support"
    );
    assert_ne!(
        stage3_audit.attachment_adoption.accepted_support_sha256[0],
        openwepp_coupled_time::Digest32::zero(),
        "the adopted accepted-history support digest must be authenticated"
    );
    assert_eq!(stage3.committed_day_count, 1);
    assert_eq!(stage3.lanes.len(), 2);
    assert!(
        stage3.accepted_publication_support_count > 0,
        "the fixture must commit accepted Stage-3 publication supports before Lane D"
    );
    assert_eq!(
        stage3.routed_runoff_mass_kg_m2.to_bits(),
        0.0_f64.to_bits(),
        "active Lane D must receive local-only Stage-3 receipts without a pre-routed double feed"
    );

    let manifest: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(&report.manifest_path).expect("read manifest"),
    )
    .expect("parse manifest");
    let active = &manifest["execution_provenance"]["laned_active"];
    assert!(
        active.is_object(),
        "authenticated native routing authority must select the production Stage-3 -> Lane-D seam without an activation selector: {manifest}"
    );
    assert_eq!(active["days_seen"].as_u64(), Some(1));
    assert_eq!(active["days_routed"].as_u64(), Some(1));
    let source_m3 = positive_manifest_f64(active, "total_source_m3");
    let outlet_m3 = positive_manifest_f64(active, "total_routed_outlet_m3");
    let storage_m3 = nonnegative_manifest_f64(active, "total_end_window_storage_m3");
    let clamp_m3 = nonnegative_manifest_f64(active, "total_clamp_m3");
    assert_relative_close(
        source_m3 + clamp_m3,
        outlet_m3 + storage_m3,
        1.0e-9,
        "Lane-D run-level source/outlet/storage/clamp volume closure",
    );

    assert_stage3_lane_d_source_bins_and_route_order();

    let wat_path = run_dir.join("output/H83.wat.parquet");
    let published_ofes = read_wat_i16_column(&wat_path, "OFE");
    let published_runoff = read_wat_f64_column(&wat_path, "QOFE");
    assert_eq!(
        published_ofes,
        vec![1, 2],
        "final WAT publication must retain upstream-to-downstream OFE order"
    );
    assert_eq!(published_runoff.len(), 2);
    assert!(
        published_runoff.iter().all(|value| *value > 0.0),
        "final WAT rows must consume the positive routed day rather than a dry/default publication"
    );
    assert_routed_lane_d_books_reach_public_outputs(&run_dir, &report, &wat_path, 2);
    let _ = std::fs::remove_dir_all(run_dir);
}

#[test]
#[ignore = "release-profile comparator probe; run explicitly with --release --ignored --exact"]
fn stage3_laned_release_one_ofe_positive_baseline_profile() {
    assert!(
        !cfg!(debug_assertions),
        "this ignored comparator probe must execute under cargo test --release"
    );
    let _execution_guard = runner_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let fixture_started = std::time::Instant::now();
    let run_dir = prepare_native_stage3_lane_d_scale_fixture(
        "stage3_laned_release_one_ofe_positive_baseline",
        &[100.0],
    );
    let fixture_wall_us = release_probe_elapsed_us(fixture_started.elapsed());
    let request = HillslopeRunRequest {
        run_dir: run_dir.clone(),
        run_file: PathBuf::from("case.run"),
        output_dir: run_dir.join("output"),
        sidecar_policy: SidecarPolicy::Compat,
        legacy_sidecar_discovery: false,
        manifest_path: None,
    };

    let bootstrap_started = std::time::Instant::now();
    let seed_path = crate::hillslope::test_fixture_authority::author_stage3_v11_owner_seed_fixture(
        &request,
        crate::hillslope::test_fixture_authority::Stage3TestFixtureSeedProfile::CompleteOwner,
        crate::hillslope::test_fixture_authority::Stage3TestFixtureSeedBinding::ExplicitRunfile,
    )
    .expect("author exact one-OFE release-probe Stage-3 owner seed");
    assert_authored_stage3_owner_cardinality(&request, &seed_path, 1);
    let bootstrap_wall_us = release_probe_elapsed_us(bootstrap_started.elapsed());

    crate::hillslope::snow_stage3_v11_qualification_audit::begin();
    let telemetry_guard = openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::begin_adaptive_parent_telemetry_v1(
        openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::STAGE3_V11_PARENT_SUPPORT_COUNT,
        std::time::Duration::from_secs(3_600),
    )
    .expect("enable bounded Stage-3 owner-evaluation telemetry");
    let qualification_telemetry_guard = openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::begin_release_qualification_telemetry_v1()
        .expect("enable one-OFE release attribution telemetry");
    let run_started = std::time::Instant::now();
    let report = execute_hillslope_run_with_runtime_policy(
        &request,
        &["openwepp-cli-hill".to_string()],
        HillslopeRuntimeSelectionPolicy::new(
            HillslopeRuntimeSelection::DirectProductionExecutor,
            HillslopeDefaultRuntimeActivation::default(),
        ),
    )
    .expect("one-OFE positive Stage-3 -> Lane-D release probe");
    let run_wall_us = release_probe_elapsed_us(run_started.elapsed());
    let qualification_telemetry = openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::take_release_qualification_telemetry_v1()
        .expect("take one-OFE release attribution telemetry");
    drop(qualification_telemetry_guard);

    let validation_started = std::time::Instant::now();
    let stage3_audit = crate::hillslope::snow_stage3_v11_qualification_audit::take();
    let telemetry = openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::take_adaptive_parent_telemetry_v1();
    drop(telemetry_guard);
    let snapshot = stage3_audit
        .committed_snapshot
        .as_ref()
        .expect("release probe committed Stage-3 qualification snapshot");
    snapshot
        .validate()
        .expect("release probe committed Stage-3 qualification evidence");
    assert_eq!(snapshot.committed_day_count, 1);
    assert_eq!(snapshot.lanes.len(), 1);
    assert_eq!(
        stage3_audit
            .support_chronology_by_day
            .get(&0)
            .expect("release probe day-zero parent supports")
            .len(),
        openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::STAGE3_V11_PARENT_SUPPORT_COUNT,
    );

    let manifest: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(&report.manifest_path)
            .expect("read release-probe Lane-D manifest"),
    )
    .expect("parse release-probe Lane-D manifest");
    let active = &manifest["execution_provenance"]["laned_active"];
    assert_eq!(active["days_seen"].as_u64(), Some(1));
    assert_eq!(active["days_routed"].as_u64(), Some(1));

    let hbp_bytes = std::fs::read(&report.output_pass).expect("read release-probe routed HBP");
    let (hbp, latest_event) = parse_hbp_from_bytes_with_latest_event_payload(
        &hbp_bytes,
        &report.output_pass,
        HbpParseOptions {
            expected_hillslope_id: Some(83),
        },
    )
    .expect("release-probe routed HBP must parse");
    assert_eq!(hbp.nofe, 1);
    let event = latest_event.expect("release probe must publish a positive routed HBP event");
    let wat_path = run_dir.join("output/H83.wat.parquet");
    let evidence = RealLaneDPublicEvidence {
        ofe_count: 1,
        areas_m2: read_wat_f64_column(&wat_path, "Area"),
        source_depths_mm: read_wat5_hourly_sources(
            &run_dir.join("output/H83.wat-subhourly.parquet"),
            1,
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
    assert_real_lane_d_public_closure(&evidence, &[100.0]);
    let fixed_point_evaluation_count = telemetry
        .iter()
        .map(|row| row.fixed_point_evaluation_count)
        .sum::<u64>();
    let direct_trial_count = telemetry
        .iter()
        .map(|row| row.direct_trial_count)
        .sum::<u64>();
    let split_child_trial_count = telemetry
        .iter()
        .map(|row| row.split_child_trial_count)
        .sum::<u64>();
    let accepted_microstep_count = telemetry
        .iter()
        .map(|row| row.accepted_microstep_count)
        .sum::<u64>();
    let telemetry_elapsed_us = |project: fn(
        &openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::AdaptiveParentTelemetryV1,
    ) -> std::time::Duration| {
        release_probe_elapsed_us(telemetry.iter().map(project).sum())
    };
    let parent_phase_wall_us = serde_json::json!({
        "parent": telemetry_elapsed_us(|row| row.parent_elapsed),
        "covered_direct_trial": telemetry_elapsed_us(|row| row.covered_direct_trial_phase_elapsed),
        "covered_composed_trial": telemetry_elapsed_us(|row| row.covered_composed_trial_phase_elapsed),
        "terminal_direct_trial": telemetry_elapsed_us(|row| row.terminal_direct_trial_phase_elapsed),
        "terminal_composed_trial": telemetry_elapsed_us(|row| row.terminal_composed_trial_phase_elapsed),
        "provisional_projection": telemetry_elapsed_us(|row| row.provisional_envelope_projection_elapsed),
        "provisional_solver_ready": telemetry_elapsed_us(|row| row.provisional_envelope_solver_ready_elapsed),
        "provisional_physical": telemetry_elapsed_us(|row| row.provisional_envelope_physical_elapsed),
        "provisional_receipts": telemetry_elapsed_us(|row| row.provisional_envelope_receipts_elapsed),
        "provisional_owner": telemetry_elapsed_us(|row| row.provisional_envelope_owner_elapsed),
        "publication_append": telemetry_elapsed_us(|row| row.publication_append_elapsed),
        "publication_full_validation": telemetry_elapsed_us(|row| row.publication_full_validation_elapsed),
        "reuse_validation": telemetry_elapsed_us(|row| row.reuse_validation_elapsed),
    });
    let physical_phase_wall_us = serde_json::json!({
        "topology": telemetry_elapsed_us(|row| row.profile_detail.physical_topology_elapsed),
        "potential": telemetry_elapsed_us(|row| row.profile_detail.physical_potential_elapsed),
        "request": telemetry_elapsed_us(|row| row.profile_detail.physical_request_elapsed),
        "unified": telemetry_elapsed_us(|row| row.profile_detail.physical_unified_elapsed),
        "final_tile": telemetry_elapsed_us(|row| row.profile_detail.physical_final_tile_elapsed),
        "protocol": telemetry_elapsed_us(|row| row.profile_detail.physical_protocol_elapsed),
        "ingress": telemetry_elapsed_us(|row| row.profile_detail.physical_ingress_elapsed),
        "post": telemetry_elapsed_us(|row| row.profile_detail.physical_post_elapsed),
    });
    let unified_phase_wall_us = serde_json::json!({
        "preflight": telemetry_elapsed_us(|row| row.profile_detail.unified_preflight_elapsed),
        "authorization": telemetry_elapsed_us(|row| row.profile_detail.unified_authorization_elapsed),
        "entry_validation": telemetry_elapsed_us(|row| row.profile_detail.unified_entry_validation_elapsed),
        "protocol_validation": telemetry_elapsed_us(|row| row.profile_detail.unified_protocol_validation_elapsed),
        "candidate": telemetry_elapsed_us(|row| row.profile_detail.unified_candidate_elapsed),
        "candidate_soil": telemetry_elapsed_us(|row| row.profile_detail.candidate_soil_elapsed),
        "candidate_surface_resource": telemetry_elapsed_us(|row| row.profile_detail.candidate_surface_resource_elapsed),
        "candidate_surface_ingress": telemetry_elapsed_us(|row| row.profile_detail.candidate_surface_ingress_elapsed),
        "candidate_receivers": telemetry_elapsed_us(|row| row.profile_detail.candidate_receivers_elapsed),
        "candidate_validation": telemetry_elapsed_us(|row| row.profile_detail.candidate_validation_elapsed),
    });
    let finalization_phase_wall_us = serde_json::json!({
        "candidate": telemetry_elapsed_us(|row| row.profile_detail.finalization_candidate_elapsed),
        "sealed_source": telemetry_elapsed_us(|row| row.profile_detail.finalization_sealed_source_elapsed),
        "install": telemetry_elapsed_us(|row| row.profile_detail.finalization_install_elapsed),
        "identity_replay": telemetry_elapsed_us(|row| row.profile_detail.finalization_identity_replay_elapsed),
    });
    let imported_stack_phase_wall_us = serde_json::json!({
        "entry_validation": telemetry_elapsed_us(|row| row.profile_detail.imported_entry_validation_elapsed),
        "physical_candidate": telemetry_elapsed_us(|row| row.profile_detail.imported_physical_candidate_elapsed),
        "physical_setup": telemetry_elapsed_us(|row| row.profile_detail.imported_physical_setup_elapsed),
        "frozen_evaluation": telemetry_elapsed_us(|row| row.profile_detail.imported_frozen_evaluation_elapsed),
        "frozen_preparation": telemetry_elapsed_us(|row| row.profile_detail.imported_frozen_preparation_elapsed),
        "frozen_execute_accept": telemetry_elapsed_us(|row| row.profile_detail.imported_frozen_execute_accept_elapsed),
        "frozen_execution_setup": telemetry_elapsed_us(|row| row.profile_detail.imported_frozen_execution_setup_elapsed),
        "frozen_runtime": telemetry_elapsed_us(|row| row.profile_detail.imported_frozen_runtime_elapsed),
        "frozen_acceptance": telemetry_elapsed_us(|row| row.profile_detail.imported_frozen_acceptance_elapsed),
        "envelope_construction": telemetry_elapsed_us(|row| row.profile_detail.imported_envelope_construction_elapsed),
        "envelope_validation": telemetry_elapsed_us(|row| row.profile_detail.imported_envelope_validation_elapsed),
        "accepted_candidate": telemetry_elapsed_us(|row| row.profile_detail.imported_accepted_candidate_elapsed),
        "owner_publication": telemetry_elapsed_us(|row| row.profile_detail.imported_owner_publication_elapsed),
        "install": telemetry_elapsed_us(|row| row.profile_detail.imported_install_elapsed),
        "reuse_validation": telemetry_elapsed_us(|row| row.profile_detail.imported_reuse_validation_elapsed),
        "reuse_reseal": telemetry_elapsed_us(|row| row.profile_detail.imported_reuse_reseal_elapsed),
        "reuse_install": telemetry_elapsed_us(|row| row.profile_detail.imported_reuse_install_elapsed),
    });
    let terminal_candidate_phase_wall_us = serde_json::json!({
        "setup": telemetry_elapsed_us(|row| row.profile_detail.terminal_candidate_setup_elapsed),
        "provider_custody": telemetry_elapsed_us(|row| row.profile_detail.terminal_provider_custody_elapsed),
        "provider_projection": telemetry_elapsed_us(|row| row.profile_detail.terminal_provider_projection_elapsed),
        "provider_carrier": telemetry_elapsed_us(|row| row.profile_detail.terminal_provider_carrier_elapsed),
        "provider_retention": telemetry_elapsed_us(|row| row.profile_detail.terminal_provider_retention_elapsed),
        "result_finalization": telemetry_elapsed_us(|row| row.profile_detail.terminal_result_finalization_elapsed),
        "carrier_physical": telemetry_elapsed_us(|row| row.profile_detail.carrier_physical_phase_elapsed),
        "carrier_complete": telemetry_elapsed_us(|row| row.profile_detail.carrier_complete_phase_elapsed),
        "carrier_physical_setup": telemetry_elapsed_us(|row| row.profile_detail.carrier_physical_setup_elapsed),
        "carrier_physical_evidence": telemetry_elapsed_us(|row| row.profile_detail.carrier_physical_evidence_elapsed),
        "carrier_physical_completion": telemetry_elapsed_us(|row| row.profile_detail.carrier_physical_completion_elapsed),
        "carrier_complete_envelope": telemetry_elapsed_us(|row| row.profile_detail.carrier_complete_envelope_elapsed),
        "carrier_complete_adoption": telemetry_elapsed_us(|row| row.profile_detail.carrier_complete_adoption_elapsed),
        "carrier_complete_projection": telemetry_elapsed_us(|row| row.profile_detail.carrier_complete_projection_elapsed),
        "carrier_complete_owner": telemetry_elapsed_us(|row| row.profile_detail.carrier_complete_owner_elapsed),
        "carrier_owner_vegetation_validation": telemetry_elapsed_us(|row| row.profile_detail.carrier_owner_vegetation_validation_elapsed),
        "carrier_owner_hydrology_projection": telemetry_elapsed_us(|row| row.profile_detail.carrier_owner_hydrology_projection_elapsed),
        "carrier_owner_surface_canonical": telemetry_elapsed_us(|row| row.profile_detail.carrier_owner_surface_canonical_elapsed),
        "carrier_owner_vegetation_encoding": telemetry_elapsed_us(|row| row.profile_detail.carrier_owner_vegetation_encoding_elapsed),
        "carrier_owner_surface_encoding": telemetry_elapsed_us(|row| row.profile_detail.carrier_owner_surface_encoding_elapsed),
        "carrier_owner_soil_encoding": telemetry_elapsed_us(|row| row.profile_detail.carrier_owner_soil_encoding_elapsed),
        "carrier_owner_other_encoding": telemetry_elapsed_us(|row| row.profile_detail.carrier_owner_other_encoding_elapsed),
        "carrier_owner_joint_map": telemetry_elapsed_us(|row| row.profile_detail.carrier_owner_joint_map_elapsed),
        "carrier_owner_joint_seal": telemetry_elapsed_us(|row| row.profile_detail.carrier_owner_joint_seal_elapsed),
        "carrier_owner_soil_custody": telemetry_elapsed_us(|row| row.profile_detail.carrier_owner_soil_custody_elapsed),
        "carrier_owner_candidate_bytes": telemetry_elapsed_us(|row| row.profile_detail.carrier_owner_candidate_bytes_elapsed),
        "carrier_owner_ephemeral_assembly": telemetry_elapsed_us(|row| row.profile_detail.carrier_owner_ephemeral_assembly_elapsed),
    });
    let validation_wall_us = release_probe_elapsed_us(validation_started.elapsed());

    let record = serde_json::json!({
        "record": "stage3_laned_release_one_ofe_positive_baseline_v1",
        "fixture_wall_us": fixture_wall_us,
        "bootstrap_wall_us": bootstrap_wall_us,
        "run_wall_us": run_wall_us,
        "validation_wall_us": validation_wall_us,
        "rss_kib": release_probe_rss_kib(),
        "committed_day_count": snapshot.committed_day_count,
        "total_parent_support_count": snapshot.total_parent_support_count,
        "covered_parent_support_count": snapshot.adaptive_support_receipt_count,
        "snow_free_parent_support_count": snapshot.snow_free_parent_support_count,
        "accepted_publication_support_count": snapshot.accepted_publication_support_count,
        "fixed_point_evaluation_count": fixed_point_evaluation_count,
        "direct_trial_count": direct_trial_count,
        "split_child_trial_count": split_child_trial_count,
        "accepted_microstep_count": accepted_microstep_count,
        "parent_phase_wall_us": parent_phase_wall_us,
        "physical_phase_wall_us": physical_phase_wall_us,
        "unified_phase_wall_us": unified_phase_wall_us,
        "finalization_phase_wall_us": finalization_phase_wall_us,
        "imported_stack_phase_wall_us": imported_stack_phase_wall_us,
        "terminal_candidate_phase_wall_us": terminal_candidate_phase_wall_us,
        "native_vegetation_et_wall_us": release_probe_elapsed_us(qualification_telemetry.native_vegetation_et_elapsed),
        "stage3_lse_soil_wall_us": release_probe_elapsed_us(qualification_telemetry.stage3_lse_soil_elapsed),
        "lane_d_wall_us": release_probe_elapsed_us(qualification_telemetry.lane_d_elapsed),
        "remaining_runner_wall_us": release_probe_elapsed_us(qualification_telemetry.remaining_runner_elapsed),
        "qualification_telemetry_total_wall_us": release_probe_elapsed_us(qualification_telemetry.total_elapsed),
        "qualification_scopes_balanced": qualification_telemetry.scopes_balanced,
        "qualification_counters_complete": qualification_telemetry.counters_complete,
        "laned_days_seen": active["days_seen"].as_u64().expect("Lane-D days_seen"),
        "laned_days_routed": active["days_routed"].as_u64().expect("Lane-D days_routed"),
        "laned_source_m3": evidence.manifest_source_m3,
        "laned_outlet_m3": evidence.manifest_outlet_m3,
        "laned_end_window_storage_m3": evidence.manifest_storage_m3,
        "laned_clamp_m3": evidence.manifest_clamp_m3,
    });
    println!("STAGE3_LANED_RELEASE_PROBE {record}");
    let _ = std::fs::remove_dir_all(run_dir);
}

fn release_probe_elapsed_us(elapsed: std::time::Duration) -> u64 {
    u64::try_from(elapsed.as_micros()).unwrap_or(u64::MAX)
}

fn release_probe_rss_kib() -> Option<u64> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    status.lines().find_map(|line| {
        let value = line.strip_prefix("VmRSS:")?.split_whitespace().next()?;
        value.parse().ok()
    })
}

fn positive_stage3_lane_d_climate_source() -> String {
    let source = complete_season_climate_source_for_days(
        CompleteSeasonClimateProfile::ReappearanceRoutingBgc,
        1,
    );
    source
        .lines()
        .map(|line| {
            if line.starts_with("1 1 2001 ") {
                // Warm, high-intensity liquid precipitation: accepted Stage-3
                // liquid is material after canopy/soil custody, and the storm
                // timing supplies a non-uniform 24-hour routing source.
                "1 1 2001 50.000 2.000 0.250 25.000 18.000 9.000 260.000 3.0 180.0 7.000"
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

fn assert_authenticated_lane_d_production_seam_present() {
    assert_production_visibility_guard_antievasion();
    let module_source =
        std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/hillslope/mod.rs"))
            .expect("read runner hillslope module source");
    let module_tokens = rust_syntax_tokens(&module_source);
    assert_rust_item_is_production_visible(
        &module_tokens,
        "mod",
        "laned_active",
        "laned_active module",
    );

    let builder_source = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs",
    ))
    .expect("read real Lane-D authority builder source");
    let builder_tokens = rust_syntax_tokens(&builder_source);
    for (kind, name, label) in [
        (
            "enum",
            "DirectLanedActiveDefaultEligibility",
            "default-eligibility type",
        ),
        (
            "fn",
            "laned_active_default_eligibility",
            "default-eligibility consumer",
        ),
        ("fn", "laned_active_config", "Lane-D config builder"),
    ] {
        assert_rust_item_is_production_visible(&builder_tokens, kind, name, label);
    }
    for function in ["laned_active_default_eligibility", "laned_active_config"] {
        let body = rust_function_body_tokens(&builder_tokens, function)
            .unwrap_or_else(|| panic!("production Stage-3 -> Lane-D seam omits {function}"));
        assert_no_conditional_cfg(body, function);
    }

    let runner = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/hillslope/05_runner_execution_and_outputs.rs"),
    )
    .expect("read real runner production source");
    let runner_tokens = rust_syntax_tokens(&runner);
    assert_rust_item_is_production_visible(
        &runner_tokens,
        "fn",
        "configure_laned_active_execution",
        "configuration/call surface",
    );
    let direct_execution =
        rust_function_body_tokens(&runner_tokens, "execute_hillslope_direct_production_days")
            .expect("locate real direct-production runner");
    assert_no_conditional_cfg(
        rust_function_body_tokens(&runner_tokens, "configure_laned_active_execution")
            .expect("locate authenticated Lane-D configuration function"),
        "runner Lane-D configuration",
    );
    let builder_position = direct_execution
        .iter()
        .position(|token| token == "day_input_builder")
        .expect("real direct runner constructs the authenticated day-input authority");
    let configure_position = direct_execution
        .iter()
        .position(|token| token == "configure_laned_active_execution")
        .unwrap_or_else(|| {
            panic!(
                "production Stage-3 -> Lane-D seam is absent: real direct runner omits authenticated Lane-D configuration call"
            )
        });
    let configure_statement_start = direct_execution[..configure_position]
        .iter()
        .rposition(|token| matches!(token.as_str(), ";" | "{" | "}"))
        .map_or(0, |position| position + 1);
    assert_no_conditional_cfg(
        &direct_execution[configure_statement_start..=configure_position],
        "real-runner Lane-D configuration call",
    );
    assert!(
        !position_has_conditional_enclosing_block(direct_execution, configure_position),
        "production Stage-3 -> Lane-D seam is absent: real-runner Lane-D configuration call is enclosed by conditional compilation"
    );
    assert!(
        builder_position < configure_position,
        "production Stage-3 -> Lane-D seam is absent: authenticated configuration must follow construction of the day-input authority"
    );
    assert!(
        direct_execution
            .iter()
            .any(|token| token == "configure_laned_active_execution")
            && direct_execution
                .iter()
                .any(|token| token == "execute_direct_publication_stream"),
        "production Stage-3 -> Lane-D seam is absent: the real direct runner does not configure authenticated default Lane-D ownership before entering publication"
    );
}

fn assert_rust_item_is_production_visible(tokens: &[String], kind: &str, name: &str, label: &str) {
    let conditionally_compiled = rust_item_conditional_compilation(tokens, kind, name)
        .unwrap_or_else(|| panic!("production Stage-3 -> Lane-D seam omits {label}"));
    assert!(
        !conditionally_compiled,
        "production Stage-3 -> Lane-D seam is absent: {label} remains test-gated or conditionally compiled"
    );
}

fn rust_item_conditional_compilation(tokens: &[String], kind: &str, name: &str) -> Option<bool> {
    let item = tokens.windows(2).position(|pair| pair == [kind, name])?;
    let header = rust_item_header_start(tokens, item);
    let directly_conditional = immediately_preceding_attributes(tokens, header)
        .iter()
        .any(|attribute| is_conditional_cfg_attribute(attribute));
    Some(directly_conditional || position_has_conditional_enclosing_block(tokens, item))
}

fn rust_item_header_start(tokens: &[String], item: usize) -> usize {
    if item > 0 && tokens[item - 1] == "pub" {
        return item - 1;
    }
    if item == 0 || tokens[item - 1] != ")" {
        return item;
    }
    let mut depth = 1_usize;
    let mut cursor = item - 1;
    while cursor > 0 && depth > 0 {
        cursor -= 1;
        match tokens[cursor].as_str() {
            ")" => depth += 1,
            "(" => depth -= 1,
            _ => {}
        }
    }
    if depth == 0 && cursor > 0 && tokens[cursor - 1] == "pub" {
        cursor - 1
    } else {
        item
    }
}

fn immediately_preceding_attributes(tokens: &[String], mut before: usize) -> Vec<&[String]> {
    let mut attributes = Vec::new();
    while before > 0 && tokens[before - 1] == "]" {
        let mut depth = 1_usize;
        let mut start = before - 1;
        while start > 0 && depth > 0 {
            start -= 1;
            match tokens[start].as_str() {
                "]" => depth += 1,
                "[" => depth -= 1,
                _ => {}
            }
        }
        if depth != 0 || start == 0 || tokens[start - 1] != "#" {
            break;
        }
        attributes.push(&tokens[start + 1..before - 1]);
        before = start - 1;
    }
    attributes
}

fn is_conditional_cfg_attribute(attribute: &[String]) -> bool {
    matches!(
        attribute.first().map(String::as_str),
        Some("cfg" | "cfg_attr")
    )
}

fn assert_no_conditional_cfg(tokens: &[String], label: &str) {
    for index in 0..tokens.len() {
        if tokens[index] == "cfg" && tokens.get(index + 1).is_some_and(|token| token == "!") {
            balanced_token_end(tokens, index + 2, "(", ")")
                .unwrap_or_else(|| panic!("malformed cfg! expression in {label}"));
            panic!(
                "production Stage-3 -> Lane-D seam is absent: {label} uses a runtime cfg! condition"
            );
        }
        if tokens[index] == "#" && tokens.get(index + 1).is_some_and(|token| token == "[") {
            let end = balanced_token_end(tokens, index + 1, "[", "]")
                .unwrap_or_else(|| panic!("malformed attribute in {label}"));
            let attribute = &tokens[index + 2..end - 1];
            assert!(
                !is_conditional_cfg_attribute(attribute),
                "production Stage-3 -> Lane-D seam is absent: {label} contains a conditional cfg/cfg_attr attribute"
            );
        }
    }
}

fn position_has_conditional_enclosing_block(tokens: &[String], position: usize) -> bool {
    let mut stack = Vec::new();
    for (index, token) in tokens[..position].iter().enumerate() {
        match token.as_str() {
            "{" => stack.push(index),
            "}" => {
                stack.pop();
            }
            _ => {}
        }
    }
    stack.into_iter().any(|open| {
        let header_start = tokens[..open]
            .iter()
            .rposition(|token| matches!(token.as_str(), ";" | "{" | "}"))
            .map_or(0, |index| index + 1);
        syntax_tokens_contain_conditional_cfg(&tokens[header_start..open])
    })
}

fn rust_function_body_tokens<'a>(tokens: &'a [String], name: &str) -> Option<&'a [String]> {
    let function = tokens.windows(2).position(|pair| pair == ["fn", name])?;
    let open = tokens[function + 2..]
        .iter()
        .position(|token| token == "{")?
        + function
        + 2;
    let end = balanced_token_end(tokens, open, "{", "}")?;
    Some(&tokens[open + 1..end - 1])
}

fn balanced_token_end(tokens: &[String], start: usize, open: &str, close: &str) -> Option<usize> {
    if tokens.get(start)? != open {
        return None;
    }
    let mut depth = 0_usize;
    for (offset, token) in tokens[start..].iter().enumerate() {
        if token == open {
            depth += 1;
        } else if token == close {
            depth -= 1;
            if depth == 0 {
                return Some(start + offset + 1);
            }
        }
    }
    None
}

fn rust_syntax_tokens(source: &str) -> Vec<String> {
    let bytes = source.as_bytes();
    let mut tokens = Vec::new();
    let mut cursor = 0_usize;
    while cursor < bytes.len() {
        if bytes[cursor].is_ascii_whitespace() {
            cursor += 1;
        } else if bytes[cursor..].starts_with(b"//") {
            cursor += 2;
            while cursor < bytes.len() && bytes[cursor] != b'\n' {
                cursor += 1;
            }
        } else if bytes[cursor..].starts_with(b"/*") {
            cursor += 2;
            let mut depth = 1_usize;
            while cursor < bytes.len() && depth > 0 {
                if bytes[cursor..].starts_with(b"/*") {
                    depth += 1;
                    cursor += 2;
                } else if bytes[cursor..].starts_with(b"*/") {
                    depth -= 1;
                    cursor += 2;
                } else {
                    cursor += 1;
                }
            }
        } else if let Some(end) = rust_raw_string_end(bytes, cursor) {
            cursor = end;
        } else if let Some(end) = rust_quoted_literal_end(bytes, cursor) {
            cursor = end;
        } else if bytes[cursor].is_ascii_alphabetic() || bytes[cursor] == b'_' {
            let start = cursor;
            cursor += 1;
            while cursor < bytes.len()
                && (bytes[cursor].is_ascii_alphanumeric() || bytes[cursor] == b'_')
            {
                cursor += 1;
            }
            tokens.push(source[start..cursor].to_owned());
        } else {
            tokens.push(char::from(bytes[cursor]).to_string());
            cursor += 1;
        }
    }
    tokens
}

fn rust_raw_string_end(bytes: &[u8], start: usize) -> Option<usize> {
    let mut cursor = start;
    if bytes.get(cursor) == Some(&b'b') {
        cursor += 1;
    }
    if bytes.get(cursor) != Some(&b'r') {
        return None;
    }
    cursor += 1;
    let hash_start = cursor;
    while bytes.get(cursor) == Some(&b'#') {
        cursor += 1;
    }
    if bytes.get(cursor) != Some(&b'"') {
        return None;
    }
    let hashes = cursor - hash_start;
    cursor += 1;
    while cursor < bytes.len() {
        if bytes[cursor] == b'"'
            && bytes.get(cursor + 1..cursor + 1 + hashes)
                == Some(&bytes[hash_start..hash_start + hashes])
        {
            return Some(cursor + 1 + hashes);
        }
        cursor += 1;
    }
    Some(bytes.len())
}

fn rust_quoted_literal_end(bytes: &[u8], start: usize) -> Option<usize> {
    let (quote, content_start, byte_literal) = match (bytes.get(start), bytes.get(start + 1)) {
        (Some(b'b'), Some(quote @ (b'"' | b'\''))) => (*quote, start + 2, true),
        (Some(quote @ (b'"' | b'\'')), _) => (*quote, start + 1, false),
        _ => return None,
    };
    if quote == b'"' {
        let mut cursor = content_start;
        while cursor < bytes.len() {
            match bytes[cursor] {
                b'\\' => cursor = (cursor + 2).min(bytes.len()),
                b'"' => return Some(cursor + 1),
                _ => cursor += 1,
            }
        }
        return Some(bytes.len());
    }

    let mut cursor = content_start;
    if bytes.get(cursor) == Some(&b'\\') {
        cursor += 1;
        if bytes.get(cursor) == Some(&b'u') && !byte_literal {
            cursor += 1;
            if bytes.get(cursor) != Some(&b'{') {
                return None;
            }
            cursor += 1;
            while bytes.get(cursor).is_some_and(u8::is_ascii_hexdigit)
                || bytes.get(cursor) == Some(&b'_')
            {
                cursor += 1;
            }
            if bytes.get(cursor) != Some(&b'}') {
                return None;
            }
            cursor += 1;
        } else if bytes.get(cursor) == Some(&b'x') {
            let digits = bytes.get(cursor + 1..cursor + 3)?;
            if !digits.iter().all(u8::is_ascii_hexdigit) {
                return None;
            }
            cursor += 3;
        } else {
            cursor += 1;
        }
    } else {
        let width = std::str::from_utf8(&bytes[cursor..])
            .ok()?
            .chars()
            .next()?
            .len_utf8();
        cursor += width;
    }
    (bytes.get(cursor) == Some(&b'\'')).then_some(cursor + 1)
}

fn syntax_tokens_contain_conditional_cfg(tokens: &[String]) -> bool {
    for index in 0..tokens.len() {
        if tokens[index] == "cfg" && tokens.get(index + 1).is_some_and(|token| token == "!") {
            return true;
        }
        if tokens[index] == "#"
            && tokens.get(index + 1).is_some_and(|token| token == "[")
            && let Some(end) = balanced_token_end(tokens, index + 1, "[", "]")
            && is_conditional_cfg_attribute(&tokens[index + 2..end - 1])
        {
            return true;
        }
    }
    false
}

fn assert_production_visibility_guard_antievasion() {
    for source in [
        "#[cfg(debug_assertions)] pub(crate) mod laned_active;",
        "#[cfg(feature = \"test-route\")] pub(crate) mod laned_active;",
        "#[cfg(any(not(target_os = \"linux\"), all(unix, feature = \"route\")))] pub(crate) mod laned_active;",
        "#[cfg_attr(feature = \"route\", allow(dead_code))] pub(crate) mod laned_active;",
        "#[cfg(feature = \"route\")] pub(in crate::hillslope::routing) mod laned_active;",
        "fn required() { if cfg!(debug_assertions) { route(); } }",
        "fn required() { if cfg!(all(unix, feature = \"route\")) { route(); } }",
    ] {
        assert!(
            syntax_tokens_contain_conditional_cfg(&rust_syntax_tokens(source)),
            "conditional-compilation anti-evasion vector must be detected: {source}"
        );
    }
    for source in [
        "#[cfg(debug_assertions)] pub(crate) mod laned_active;",
        "#[cfg(feature = \"test-route\")] pub(crate) mod laned_active;",
        "#[cfg(any(not(target_os = \"linux\"), all(unix, feature = \"route\")))] pub(crate) mod laned_active;",
        "#[cfg_attr(feature = \"route\", allow(dead_code))] pub(crate) mod laned_active;",
        "#[cfg(feature = \"route\")] pub(in crate::hillslope::routing) mod laned_active;",
    ] {
        assert_eq!(
            rust_item_conditional_compilation(&rust_syntax_tokens(source), "mod", "laned_active",),
            Some(true),
            "required-item conditional gate must be detected: {source}"
        );
    }

    let gated_impl = rust_syntax_tokens(
        "#[cfg(feature = \"route\")] impl Builder { pub(crate) fn laned_active_config(&self) {} }",
    );
    assert_eq!(
        rust_item_conditional_compilation(&gated_impl, "fn", "laned_active_config"),
        Some(true),
        "an enclosing conditional impl must gate the required item"
    );

    let scoped_function = rust_syntax_tokens(
        "#[cfg(debug_assertions)] pub(in crate::hillslope::routing) fn laned_active_config() {}",
    );
    assert_eq!(
        rust_item_conditional_compilation(&scoped_function, "fn", "laned_active_config"),
        Some(true),
        "cfg before a multi-token scoped function visibility must be recovered"
    );
    for source in [
        "pub(in crate::hillslope::routing) fn laned_active_config() {}",
        "pub(super) fn laned_active_config() {}",
        "pub(self) fn laned_active_config() {}",
    ] {
        assert_eq!(
            rust_item_conditional_compilation(
                &rust_syntax_tokens(source),
                "fn",
                "laned_active_config",
            ),
            Some(false),
            "unconditional scoped visibility must not manufacture a cfg gate: {source}"
        );
    }

    let nested_module = rust_syntax_tokens(
        "#[cfg(feature = \"route\")] pub(in crate::hillslope) mod gated { pub(in crate::hillslope::routing) fn laned_active_config() {} }",
    );
    assert_eq!(
        rust_item_conditional_compilation(&nested_module, "fn", "laned_active_config"),
        Some(true),
        "a required item nested in a cfg-gated module must be rejected"
    );

    let conditional_call = rust_syntax_tokens(
        "fn required() { #[cfg(all(debug_assertions, feature = \"route\"))] if ready { configure_laned_active_execution(); } }",
    );
    let body = rust_function_body_tokens(&conditional_call, "required")
        .expect("conditional-call anti-evasion function body");
    let call = body
        .iter()
        .position(|token| token == "configure_laned_active_execution")
        .expect("conditional-call anti-evasion call");
    assert!(
        position_has_conditional_enclosing_block(body, call),
        "compound cfg on an enclosing call-edge block must be detected"
    );

    let literal_decoys = rust_syntax_tokens(
        r####"fn required() {
            let _ = '}';
            let _ = b'{';
            let _ = '\u{7d}';
            let _ = b'\x7b';
            let _ = "} cfg!(debug_assertions) {";
            let _ = b"} #[cfg(feature = \"route\")] {";
            let _ = r###"} cfg!(feature = "route") {"###;
            let _ = br##"} #[cfg(debug_assertions)] {"##;
            // } cfg!(debug_assertions) {
            /* { #[cfg(feature = "route")] } */
            route();
        }
        fn after() {}"####,
    );
    assert!(
        rust_function_body_tokens(&literal_decoys, "required").is_some(),
        "character/string literal braces must not disrupt balanced function extraction"
    );
    assert!(
        !syntax_tokens_contain_conditional_cfg(&literal_decoys),
        "conditional-looking character/string literal contents must remain opaque"
    );
}

#[derive(Debug)]
struct RealLaneDPublicEvidence {
    ofe_count: usize,
    areas_m2: Vec<f64>,
    source_depths_mm: Vec<[f64; 24]>,
    pass_runvol_m3: Vec<f64>,
    pass_peakro_m3_s: Vec<f64>,
    hbp_hourly_outlet_m3: Vec<f64>,
    hbp_peak_outlet_m3_s: f64,
    manifest_source_m3: f64,
    manifest_outlet_m3: f64,
    manifest_storage_m3: f64,
    manifest_clamp_m3: f64,
}

impl RealLaneDPublicEvidence {
    fn routed_publication_fingerprint(&self) -> Vec<u64> {
        self.pass_runvol_m3
            .iter()
            .chain(&self.pass_peakro_m3_s)
            .chain(&self.hbp_hourly_outlet_m3)
            .chain([
                &self.hbp_peak_outlet_m3_s,
                &self.manifest_outlet_m3,
                &self.manifest_storage_m3,
                &self.manifest_clamp_m3,
            ])
            .map(|value| value.to_bits())
            .collect()
    }
}

fn assert_real_stage3_lane_d_scale_and_publication_sensitivity() {
    for ofe_count in [1_usize, 9, 10, 19] {
        let areas_m2 = (1..=ofe_count)
            .map(|ofe| {
                let ofe = f64::from(u32::try_from(ofe).expect("qualification OFE index"));
                100.0 * ofe
            })
            .collect::<Vec<_>>();
        let baseline = execute_real_stage3_lane_d_scale_case(
            &format!("stage3_laned_scale_{ofe_count}_baseline"),
            &areas_m2,
        );
        assert_real_lane_d_public_closure(&baseline, &areas_m2);

        let factor_1_000_wrong_unit_areas = areas_m2
            .iter()
            .map(|area| area * 1_000.0)
            .collect::<Vec<_>>();
        let factor_1_000 = execute_real_stage3_lane_d_scale_case(
            &format!("stage3_laned_scale_{ofe_count}_factor_1000"),
            &factor_1_000_wrong_unit_areas,
        );
        assert_real_lane_d_public_closure(&factor_1_000, &factor_1_000_wrong_unit_areas);
        assert_relative_close(
            factor_1_000.manifest_source_m3,
            baseline.manifest_source_m3 * 1_000.0,
            1.0e-9,
            "real runner factor-1000 area-unit poison must propagate to Lane-D source books",
        );
        assert_ne!(
            baseline.routed_publication_fingerprint(),
            factor_1_000.routed_publication_fingerprint(),
            "real Lane-D HBP/PASS/manifest publication must be sensitive to a factor-1000 area-unit poison at {ofe_count} OFEs"
        );

        if ofe_count > 1 {
            let mut wrong_adjacent_area_association = areas_m2.clone();
            wrong_adjacent_area_association.rotate_left(1);
            assert!(
                matches!(
                    validate_fixed_routed_result_area_lineage(
                        &baseline,
                        &wrong_adjacent_area_association,
                    ),
                    Err(RoutedPublicationLineageError::WrongAreaAssociation { .. })
                ),
                "a fixed accepted routed result must reject current/adjacent OFE area reassociation at {ofe_count} OFEs"
            );
        }
    }
}

#[test]
fn exact_stage3_owner_seed_expands_single_downstream_bgc_binding_at_scale() {
    for ofe_count in [1_usize, 9, 10, 19] {
        let areas_m2 = (1..=ofe_count)
            .map(|ofe| 100.0 * ofe as f64)
            .collect::<Vec<_>>();
        let run_dir = prepare_native_stage3_lane_d_scale_fixture(
            &format!("stage3_exact_owner_topology_{ofe_count}"),
            &areas_m2,
        );
        let request = HillslopeRunRequest {
            run_dir: run_dir.clone(),
            run_file: PathBuf::from("case.run"),
            output_dir: run_dir.join("output"),
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: false,
            manifest_path: None,
        };
        let seed_path =
            crate::hillslope::test_fixture_authority::author_stage3_v11_owner_seed_fixture(
                &request,
                crate::hillslope::test_fixture_authority::Stage3TestFixtureSeedProfile::CompleteOwner,
                crate::hillslope::test_fixture_authority::Stage3TestFixtureSeedBinding::ExplicitRunfile,
            )
            .expect("author exact 1/9/10/19 live-topology Stage-3 owner seed");
        assert_authored_stage3_owner_cardinality(&request, &seed_path, ofe_count);
        std::fs::remove_dir_all(run_dir).expect("remove focused exact-owner fixture");
    }
}

#[test]
fn duplicate_configured_mapping_rejects_real_stage3_day_without_owner_or_clock_mutation() {
    let ofe_count = 10_usize;
    let areas_m2 = (1..=ofe_count)
        .map(|ofe| 100.0 * ofe as f64)
        .collect::<Vec<_>>();
    let run_dir = prepare_native_stage3_lane_d_scale_fixture(
        "stage3_duplicate_configured_mapping_poison",
        &areas_m2,
    );
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
    .expect("author normal explicit seed sidecar before installing poison");
    let inputs = load_hillslope_run_inputs(&request).expect("load poison inputs");
    let targets =
        resolve_hillslope_output_targets(&inputs.runfile).expect("resolve poison outputs");
    let sidecars =
        resolve_hillslope_sidecars(&request, &inputs, &targets).expect("resolve poison sidecars");
    let setup = build_static_hillslope_runtime_setup(
        &request,
        &inputs,
        &sidecars,
        HillslopeRuntimeSelection::DirectProductionExecutor,
    )
    .expect("build poison static setup");
    let HillslopeClimateExecutionState {
        per_ofe_lane_areas_m2,
        per_ofe_runoff_publication_geometries,
        lane_context,
        climate_span,
    } = setup.execution_state;
    assert_eq!(per_ofe_lane_areas_m2.len(), ofe_count);
    let climate_request =
        build_hillslope_climate_runtime_request(&inputs.climate).expect("poison climate request");
    let seed_authority = DirectProductionSeedAuthority::from_typed_inputs(
        &climate_request,
        &inputs,
        &sidecars,
        ofe_count,
        lane_context.lane,
    )
    .expect("poison typed seed authority");
    let mut frame = build_direct_production_run_frame(&DirectProductionRunFrameBuildInputs {
        output_hillslope_id: targets.output_hillslope_id,
        lane_areas_m2: &per_ofe_lane_areas_m2,
        runoff_publication_geometries: &per_ofe_runoff_publication_geometries,
        day_count: climate_span.days.len(),
        seed_authority: &seed_authority,
    })
    .expect("poison live frame");
    frame
        .configure_groundwater(
            direct_groundwater_authority_from_gwcoeff(&sidecars.gwcoeff)
                .expect("poison groundwater authority"),
        )
        .expect("poison groundwater configuration");
    let poison_bytes = crate::hillslope::snow_stage3_v11_production_seed::duplicate_configured_vegetation_mapping_test_seed_bytes(
        &frame,
        41.1,
    )
    .expect("author digest-valid duplicate configured-mapping poison bytes");
    std::fs::write(&seed_path, poison_bytes).expect("install poison in normal explicit sidecar");

    arm_stage3_rejected_day_rollback_audit();
    crate::hillslope::snow_stage3_v11_qualification_audit::begin();
    let error = execute_hillslope_run_with_runtime_policy(
        &request,
        &["openwepp-cli-hill".to_string()],
        HillslopeRuntimeSelectionPolicy::new(
            HillslopeRuntimeSelection::DirectProductionExecutor,
            HillslopeDefaultRuntimeActivation::default(),
        ),
    )
    .expect_err("duplicate configured mapping must reject the normal runner pipeline");
    let rejected_stage3_audit = crate::hillslope::snow_stage3_v11_qualification_audit::take();
    assert_eq!(
        rejected_stage3_audit
            .attachment_adoption
            .successful_adoption_count,
        0,
        "a rejected real runner transaction must not adopt an attachment"
    );
    assert!(
        rejected_stage3_audit
            .attachment_adoption
            .accepted_support_sha256
            .is_empty(),
        "a rejected real runner transaction must not expose accepted-history publication"
    );
    let error = error.to_string();
    assert!(
        error.contains(
            "VEG-E-TRANSACTION-001: resource receipt is invalid: V8 component/occupancy mapping is not bijective"
        ),
        "unexpected duplicate-mapping rejection: {error}"
    );
    let rollback = take_stage3_rejected_day_rollback_audit();
    assert!(
        rollback
            .error
            .contains("V8 component/occupancy mapping is not bijective")
    );
    assert!(
        rollback.frame_unchanged,
        "failed day must roll back the full frame"
    );
    assert!(
        rollback.parent_checkpoint_bytes_unchanged,
        "V11 beginning-parent checkpoint bytes must be unchanged"
    );
    assert!(
        rollback.coupled_clock_bytes_unchanged,
        "coupled-clock bytes must be unchanged"
    );
    std::fs::remove_dir_all(run_dir).expect("remove duplicate-mapping poison fixture");
}

fn execute_real_stage3_lane_d_scale_case(
    prefix: &str,
    areas_m2: &[f64],
) -> RealLaneDPublicEvidence {
    let run_dir = prepare_native_stage3_lane_d_scale_fixture(prefix, areas_m2);
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
    .expect("author exact live-topology Stage-3 owner seed");
    assert_authored_stage3_owner_cardinality(&request, &seed_path, areas_m2.len());
    let report = execute_hillslope_run_with_runtime_policy(
        &request,
        &["openwepp-cli-hill".to_string()],
        HillslopeRuntimeSelectionPolicy::new(
            HillslopeRuntimeSelection::DirectProductionExecutor,
            HillslopeDefaultRuntimeActivation::default(),
        ),
    )
    .expect("real Stage-3 Lane-D scale/sensitivity run");

    let manifest: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(&report.manifest_path).expect("read scale-case manifest"),
    )
    .expect("parse scale-case manifest");
    let active = &manifest["execution_provenance"]["laned_active"];
    assert_eq!(active["days_seen"].as_u64(), Some(1));
    assert_eq!(active["days_routed"].as_u64(), Some(1));

    let hbp_bytes = std::fs::read(&report.output_pass).expect("read scale-case routed HBP");
    let (hbp, latest_event) = parse_hbp_from_bytes_with_latest_event_payload(
        &hbp_bytes,
        &report.output_pass,
        HbpParseOptions {
            expected_hillslope_id: Some(83),
        },
    )
    .expect("scale-case routed public HBP must parse");
    assert_eq!(
        usize::try_from(hbp.nofe).expect("HBP OFE count fits usize"),
        areas_m2.len(),
        "HBP must retain full topology"
    );
    let event = latest_event.expect("positive scale case must publish routed HBP event");

    let wat_path = run_dir.join("output/H83.wat.parquet");
    let evidence = RealLaneDPublicEvidence {
        ofe_count: areas_m2.len(),
        areas_m2: read_wat_f64_column(&wat_path, "Area"),
        source_depths_mm: read_wat5_hourly_sources(
            &run_dir.join("output/H83.wat-subhourly.parquet"),
            areas_m2.len(),
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
    let _ = std::fs::remove_dir_all(run_dir);
    evidence
}

fn assert_authored_stage3_owner_cardinality(
    request: &HillslopeRunRequest,
    seed_path: &Path,
    expected_ofe_count: usize,
) {
    let inputs = load_hillslope_run_inputs(request).expect("load authored scale-case inputs");
    let targets = resolve_hillslope_output_targets(&inputs.runfile)
        .expect("resolve authored scale-case output targets");
    let sidecars = resolve_hillslope_sidecars(request, &inputs, &targets)
        .expect("resolve authored scale-case sidecars");
    let setup = build_static_hillslope_runtime_setup(
        request,
        &inputs,
        &sidecars,
        HillslopeRuntimeSelection::DirectProductionExecutor,
    )
    .expect("build authored scale-case static setup");
    let HillslopeClimateExecutionState {
        per_ofe_lane_areas_m2,
        per_ofe_runoff_publication_geometries,
        lane_context,
        climate_span,
    } = setup.execution_state;
    assert_eq!(per_ofe_lane_areas_m2.len(), expected_ofe_count);
    let climate_request =
        build_hillslope_climate_runtime_request(&inputs.climate).expect("scale climate request");
    let seed_authority = DirectProductionSeedAuthority::from_typed_inputs(
        &climate_request,
        &inputs,
        &sidecars,
        expected_ofe_count,
        lane_context.lane,
    )
    .expect("scale-case typed seed authority");
    let mut frame = build_direct_production_run_frame(&DirectProductionRunFrameBuildInputs {
        output_hillslope_id: targets.output_hillslope_id,
        lane_areas_m2: &per_ofe_lane_areas_m2,
        runoff_publication_geometries: &per_ofe_runoff_publication_geometries,
        day_count: climate_span.days.len(),
        seed_authority: &seed_authority,
    })
    .expect("scale-case live frame");
    frame
        .configure_groundwater(
            direct_groundwater_authority_from_gwcoeff(&sidecars.gwcoeff)
                .expect("scale-case groundwater authority"),
        )
        .expect("scale-case groundwater configuration");
    let seed = crate::hillslope::snow_stage3_v11_production_seed::DirectSnowStage3V11ProductionSeedV1::load_required(
        Some(seed_path),
    )
    .expect("load authored live-topology Stage-3 seed");
    let (vegetation_configuration, lse_configuration) = seed.test_fixture_vegetation_authorities();
    assert_eq!(lse_configuration.ofes.len(), expected_ofe_count);
    let expected_physical_ofes = (1..=expected_ofe_count)
        .map(|ofe| format!("ofe-{ofe}"))
        .collect::<Vec<_>>();
    assert_eq!(
        lse_configuration
            .ofes
            .iter()
            .map(|ofe| ofe.ofe_id.as_str())
            .collect::<Vec<_>>(),
        expected_physical_ofes,
        "sealed LSE authority must retain physical lane order, including decimal OFE identities"
    );
    if expected_ofe_count >= 10 {
        assert_eq!(lse_configuration.ofes[1].ofe_id.as_str(), "ofe-2");
        assert_eq!(lse_configuration.ofes[9].ofe_id.as_str(), "ofe-10");
    }

    let configured_occupancies = vegetation_configuration.expected_occupancies();
    assert!(
        !configured_occupancies.is_empty(),
        "complete-owner qualification seed must retain configured vegetation"
    );
    let configured_vegetation_tiles = configured_occupancies
        .iter()
        .map(|occupancy| occupancy.tile_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let downstream_ofe = lse_configuration
        .ofes
        .last()
        .expect("nonempty LSE topology")
        .ofe_id
        .as_str();
    let mut open_vegetation_ids = std::collections::BTreeSet::new();
    for (ofe_index, ofe) in lse_configuration.ofes.iter().enumerate() {
        for tile in &ofe.tiles {
            if configured_vegetation_tiles.contains(tile.vegetation_tile_id.as_str()) {
                assert_eq!(
                    ofe.ofe_id.as_str(),
                    downstream_ofe,
                    "only the downstream common BGC-bearing OFE may map configured vegetation"
                );
            } else if ofe_index + 1 < expected_ofe_count {
                assert!(
                    open_vegetation_ids.insert(tile.vegetation_tile_id.as_str()),
                    "every upstream open-ingress LSE tile needs a unique nonconfigured vegetation identity"
                );
            }
        }
    }
    for occupancy in &configured_occupancies {
        let bindings = lse_configuration
            .ofes
            .iter()
            .flat_map(|ofe| {
                ofe.tiles
                    .iter()
                    .filter(move |tile| tile.vegetation_tile_id == occupancy.tile_id)
                    .map(move |_| ofe.ofe_id.as_str())
            })
            .collect::<Vec<_>>();
        assert_eq!(
            bindings,
            vec![downstream_ofe],
            "each vegetation occupancy/BGC stratum must resolve through exactly one downstream LSE binding"
        );
    }
    seed.bootstrap(&mut frame)
        .expect("bootstrap authored live-topology Stage-3 owner");
    assert_eq!(frame.identity.lane_count, expected_ofe_count);
    assert_eq!(frame.lanes.len(), expected_ofe_count);
    let attachment = frame
        .snow_stage3_v11_attachment
        .as_ref()
        .expect("authored Stage-3 attachment installed before runner execution");
    assert_eq!(attachment.static_context.lane_ids.len(), expected_ofe_count);
    assert_eq!(
        attachment
            .static_context
            .surface_liquid_configuration
            .ofe_topology
            .len(),
        expected_ofe_count
    );
    assert_eq!(
        attachment
            .static_context
            .surface_liquid_configuration
            .ofe_bindings
            .len(),
        expected_ofe_count
    );
    assert_eq!(
        attachment.static_context.wb14_parameters.len(),
        expected_ofe_count
    );
    for record in &attachment
        .static_context
        .surface_liquid_configuration
        .records
    {
        let topology_index = attachment
            .static_context
            .surface_liquid_configuration
            .ofe_topology
            .iter()
            .position(|ofe| ofe == &record.key.ofe_id)
            .expect("surface store OFE belongs to authenticated topology");
        if topology_index + 1 < expected_ofe_count {
            assert_eq!(
                record.ground_ingress_mode,
                openwepp_hillslope_orchestrator::DirectGroundIngressMode::OpenRawPrecipitation,
                "every upstream OFE must use authoritative open raw-precipitation ingress"
            );
        }
    }
}

#[derive(Debug, PartialEq)]
enum RoutedPublicationLineageError {
    Cardinality {
        published: usize,
        associated: usize,
    },
    WrongAreaAssociation {
        ofe_index: usize,
        published_area_m2_bits: u64,
        associated_area_m2_bits: u64,
    },
}

fn validate_fixed_routed_result_area_lineage(
    evidence: &RealLaneDPublicEvidence,
    associated_areas_m2: &[f64],
) -> Result<(), RoutedPublicationLineageError> {
    if evidence.areas_m2.len() != associated_areas_m2.len() {
        return Err(RoutedPublicationLineageError::Cardinality {
            published: evidence.areas_m2.len(),
            associated: associated_areas_m2.len(),
        });
    }
    for (ofe_index, (published, associated)) in evidence
        .areas_m2
        .iter()
        .zip(associated_areas_m2)
        .enumerate()
    {
        if published.to_bits() != associated.to_bits() {
            return Err(RoutedPublicationLineageError::WrongAreaAssociation {
                ofe_index,
                published_area_m2_bits: published.to_bits(),
                associated_area_m2_bits: associated.to_bits(),
            });
        }
    }
    Ok(())
}

fn assert_real_lane_d_public_closure(
    evidence: &RealLaneDPublicEvidence,
    expected_areas_m2: &[f64],
) {
    const SOURCE_DEPTH_TOLERANCE_M: f64 = 1.0e-12;
    const PUBLICATION_REL_TOLERANCE: f64 = 1.0e-9;
    const HOUR_SECONDS: f64 = 3_600.0;

    assert_eq!(evidence.ofe_count, expected_areas_m2.len());
    assert_eq!(evidence.areas_m2, expected_areas_m2);
    assert_eq!(evidence.source_depths_mm.len(), evidence.ofe_count);
    assert_eq!(evidence.pass_runvol_m3.len(), 1, "PASS is outlet-only");
    assert_eq!(evidence.pass_peakro_m3_s.len(), 1, "PASS is outlet-only");
    assert_eq!(evidence.hbp_hourly_outlet_m3.len(), 24);
    validate_fixed_routed_result_area_lineage(evidence, expected_areas_m2)
        .expect("fixed routed result retains exact OFE/area association");

    let independently_reconstructed_source_m3 = evidence
        .source_depths_mm
        .iter()
        .zip(&evidence.areas_m2)
        .map(|(depths_mm, area_m2)| depths_mm.iter().sum::<f64>() / 1_000.0 * area_m2)
        .sum::<f64>();
    let source_tolerance_m3 = SOURCE_DEPTH_TOLERANCE_M * evidence.areas_m2.iter().sum::<f64>();
    assert!(
        (independently_reconstructed_source_m3 - evidence.manifest_source_m3).abs()
            <= source_tolerance_m3,
        "all-OFE WAT5 accepted-source reconstruction must close the Lane-D source book: reconstructed={independently_reconstructed_source_m3}, manifest={}, tolerance={source_tolerance_m3}",
        evidence.manifest_source_m3,
    );

    let wrong_unit_source_m3 = independently_reconstructed_source_m3 * 1_000.0;
    assert!(
        (wrong_unit_source_m3 - evidence.manifest_source_m3).abs() > source_tolerance_m3,
        "factor-1000 mm-as-m poison must not satisfy Lane-D source closure"
    );

    assert_relative_close(
        evidence.manifest_source_m3 + evidence.manifest_clamp_m3,
        evidence.manifest_outlet_m3 + evidence.manifest_storage_m3,
        PUBLICATION_REL_TOLERANCE,
        "Lane-D source + clamp = terminal routed outlet + end-window storage",
    );
    let terminal_pass_runvol_m3 = *evidence
        .pass_runvol_m3
        .last()
        .expect("terminal PASS volume");
    assert_relative_close(
        terminal_pass_runvol_m3,
        evidence.manifest_outlet_m3,
        PUBLICATION_REL_TOLERANCE,
        "terminal routed Lane-D outlet book -> PASS runvol",
    );
    let hbp_outlet_m3 = evidence.hbp_hourly_outlet_m3.iter().sum::<f64>();
    assert_relative_close(
        hbp_outlet_m3,
        evidence.manifest_outlet_m3,
        PUBLICATION_REL_TOLERANCE,
        "terminal routed Lane-D outlet book -> HBP hourly volume",
    );
    let reconstructed_peak_m3_s = evidence
        .hbp_hourly_outlet_m3
        .iter()
        .copied()
        .fold(0.0_f64, f64::max)
        / HOUR_SECONDS;
    assert_relative_close(
        reconstructed_peak_m3_s,
        evidence.hbp_peak_outlet_m3_s,
        PUBLICATION_REL_TOLERANCE,
        "routed HBP hourly maximum -> HBP peak",
    );
    assert_relative_close(
        reconstructed_peak_m3_s,
        *evidence
            .pass_peakro_m3_s
            .last()
            .expect("terminal PASS peak"),
        PUBLICATION_REL_TOLERANCE,
        "routed HBP hourly maximum -> terminal PASS peakro",
    );
}

fn prepare_native_stage3_lane_d_scale_fixture(prefix: &str, areas_m2: &[f64]) -> PathBuf {
    use std::fmt::Write as _;

    assert!(!areas_m2.is_empty());
    let run_dir = prepare_two_ofe_complete_season_fixture(prefix);
    let runfile_path = run_dir.join("case.run");
    let runfile = std::fs::read_to_string(&runfile_path)
        .expect("read scale-case runfile")
        .replace(
            "management = \"p102.man\"",
            "management = \"case.man.yaml\"",
        )
        .replace(
            "wat = \"output/H83.wat.parquet\"",
            concat!(
                "wat = \"output/H83.wat.parquet\"\n",
                "wat_subhourly = \"output/H83.wat-subhourly.parquet\"",
            ),
        );
    std::fs::write(&runfile_path, runfile).expect("write scale-case runfile");

    let native_management_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/cancov_forest/marcell_conifer_mn/p8.man.yaml");
    let native_management = std::fs::read_to_string(native_management_path)
        .expect("read native-forest management authority");
    let (native_prefix, _) = native_management
        .split_once("schedule:\n")
        .expect("native management schedule marker");
    let native_prefix = native_prefix
        .replacen("nofes: 1", &format!("nofes: {}", areas_m2.len()), 1)
        .replacen("total_years: 45", "total_years: 1", 1);
    let mut management = format!("{native_prefix}schedule:\n  ofe_initial_refs:\n");
    for _ in areas_m2 {
        management.push_str("  - 1\n");
    }
    management.push_str("  rotation_repeats: 1\n  rotation_years: 1\n  slots:\n");
    for ofe_index in 1..=areas_m2.len() {
        writeln!(
            management,
            "  - rotation_index: 1\n    year_in_rotation: 1\n    ofe_index: {ofe_index}\n    yearly_refs:\n    - 1"
        )
        .expect("author scale-case native schedule");
    }
    std::fs::write(run_dir.join("case.man.yaml"), management)
        .expect("write scale-case native management authority");

    let mut slope = format!("97.5\n{}\n", areas_m2.len());
    for (lane_index, area_m2) in areas_m2.iter().copied().enumerate() {
        assert!(area_m2.is_finite() && area_m2 > 0.0);
        let width_m = area_m2 / 10.0;
        let gradient = 0.02 + 0.001 * lane_index as f64;
        writeln!(
            slope,
            "180.0 {width_m:.12}\n3 10.0\n0.0 {gradient:.6} 0.5 {gradient:.6} 1.0 {gradient:.6}"
        )
        .expect("author scale-case slope");
    }
    std::fs::write(run_dir.join("p102.slp"), slope).expect("write scale-case slope topology");

    let two_ofe_soil = std::fs::read_to_string(run_dir.join("p102.sol"))
        .expect("read expanded two-OFE soil authority");
    let native_forest_soil = expand_two_ofe_soil_authority(&two_ofe_soil, areas_m2.len())
        .replace("'forest high sev fire'", "'forest'")
        .replace("'forest moderate sev fire'", "'forest'");
    std::fs::write(run_dir.join("p102.sol"), native_forest_soil)
        .expect("write scale-case per-OFE soil authority");
    std::fs::write(
        run_dir.join("p102.cli"),
        positive_stage3_lane_d_climate_source(),
    )
    .expect("write positive scale-case Stage-3 climate");
    run_dir
}

fn expand_two_ofe_soil_authority(source: &str, ofe_count: usize) -> String {
    let lines = source.lines().collect::<Vec<_>>();
    let counts_index = lines
        .iter()
        .position(|line| line.trim() == "2 0")
        .expect("two-OFE soil count line");
    let mut blocks = Vec::<String>::new();
    let mut block = String::new();
    for line in &lines[counts_index + 1..] {
        block.push_str(line);
        block.push('\n');
        if line.trim() == "1 10000.0 0.01" {
            blocks.push(std::mem::take(&mut block));
        }
    }
    assert_eq!(blocks.len(), 2, "two authoritative soil operand blocks");
    assert!(block.trim().is_empty(), "no unparsed soil authority tail");

    let mut expanded = lines[..counts_index].join("\n");
    expanded.push('\n');
    expanded.push_str(&format!("{ofe_count} 0\n"));
    for lane_index in 0..ofe_count {
        expanded.push_str(&blocks[lane_index % blocks.len()]);
    }
    expanded
}

fn positive_manifest_f64(parent: &serde_json::Value, field: &str) -> f64 {
    let value = nonnegative_manifest_f64(parent, field);
    assert!(value > 0.0, "manifest field {field} must be positive");
    value
}

fn nonnegative_manifest_f64(parent: &serde_json::Value, field: &str) -> f64 {
    let value = parent[field]
        .as_f64()
        .unwrap_or_else(|| panic!("manifest field {field} must be a finite number"));
    assert!(
        value.is_finite() && value >= 0.0,
        "manifest field {field} must be finite and non-negative, observed {value}"
    );
    value
}

fn assert_relative_close(left: f64, right: f64, tolerance: f64, label: &str) {
    let scale = left.abs().max(right.abs()).max(f64::MIN_POSITIVE);
    let relative = (left - right).abs() / scale;
    assert!(
        relative <= tolerance,
        "{label}: left={left}, right={right}, relative={relative}, tolerance={tolerance}"
    );
}

fn assert_stage3_lane_d_source_bins_and_route_order() {
    let orchestrator = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../openwepp-hillslope-orchestrator/src/direct_runtime");
    let seam_source = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../openwepp-hillslope-orchestrator/src/ofe_routing/seam.rs"),
    )
    .expect("read canonical Lane-D seam source");
    assert!(
        seam_source.contains("pub const SEAM_HOUR_BINS: usize = 24"),
        "the accepted Lane-D source must retain exactly 24 hourly bins"
    );

    let lane_source = std::fs::read_to_string(orchestrator.join("laned_active.rs"))
        .expect("read canonical Lane-D source builder");
    let source_body = lane_source
        .split("pub(crate) fn laned_active_lane_source(")
        .nth(1)
        .and_then(|tail| {
            tail.split("pub(crate) fn laned_active_assert_no_dc01_surface_feed")
                .next()
        })
        .expect("locate canonical accepted-source builder");
    assert!(source_body.contains("day_frame.wb14_hourly_excess_m"));
    assert!(source_body.contains("closing_hourly_runoff_depths_m("));
    assert!(source_body.contains("let (depths_m, total)"));
    assert!(source_body.contains("total.abs().max(q_runoff_m.abs())"));

    let runoff_source = std::fs::read_to_string(orchestrator.join("runoff.rs"))
        .expect("read canonical hourly runoff source assembler");
    let closing_body = runoff_source
        .split("pub(crate) fn closing_hourly_runoff_depths_m(")
        .nth(1)
        .and_then(|tail| {
            tail.split("fn assemble_closing_hourly_runoff_depths_m(")
                .next()
        })
        .expect("locate validated closing-hourly runoff helper");
    assert!(closing_body.contains("assemble_closing_hourly_runoff_depths_m("));
    let assembler_body = runoff_source
        .split("fn assemble_closing_hourly_runoff_depths_m(")
        .nth(1)
        .and_then(|tail| tail.split("fn ensure_hourly_runoff_source_closure(").next())
        .expect("locate canonical closing-hourly runoff assembler");
    assert!(assembler_body.contains("[0.0; DC01_HOUR_BIN_COUNT]"));
    assert!(assembler_body.contains("for hour in 0..DC01_HOUR_BIN_COUNT"));
    assert!(assembler_body.contains("wb14_hourly_excess_m[hour]"));
    assert!(assembler_body.contains("hourly_saturation_carry_m[hour]"));
    assert!(assembler_body.contains("total_m += hourly_runoff_m[hour]"));

    let executor = std::fs::read_to_string(orchestrator.join("03_executor.rs"))
        .expect("read canonical Lane-D executor");
    let route_body = executor
        .split("fn route_laned_active_day(")
        .nth(1)
        .and_then(|tail| tail.split("fn publish_laned_active_day(").next())
        .expect("locate canonical Lane-D topology loop");
    let loop_position = route_body
        .find("for lane_index in 0..lane_count")
        .expect("one route call loop over every OFE");
    let call_position = route_body[loop_position..]
        .find("laned_active::laned_active_route_lane(")
        .map(|offset| loop_position + offset)
        .expect("Lane-D route call inside topology loop");
    let handoff_read_position = route_body[call_position..]
        .find("upstream.as_ref()")
        .map(|offset| call_position + offset)
        .expect("adjacent upstream handoff consumed by route call");
    let handoff_write_position = route_body[handoff_read_position..]
        .find("upstream = Some(handoff)")
        .map(|offset| handoff_read_position + offset)
        .expect("route result installed for the adjacent downstream OFE");
    assert!(
        loop_position < call_position
            && call_position < handoff_read_position
            && handoff_read_position < handoff_write_position,
        "Lane D must route exactly once per lane in topology order and install each adjacent handoff"
    );

    let stream_body = executor
        .split("fn run_laned_active_publication_stream")
        .nth(1)
        .expect("locate canonical Lane-D publication stream");
    let prepare = stream_body
        .find("prepare_stage3_day(frame, day_index)")
        .expect("accepted Stage-3 day preparation");
    let source = stream_body
        .find("Self::laned_active_lane_sources")
        .expect("accepted hourly source construction");
    let route = stream_body
        .find("Self::route_laned_active_day")
        .expect("Lane-D route call");
    let publish = stream_body
        .find("Self::publish_laned_active_day")
        .expect("routed-result publication");
    assert!(
        prepare < source && source < route && route < publish,
        "accepted Stage-3 commit/source must precede Lane-D routing, which must precede final publication"
    );
}

fn assert_routed_lane_d_books_reach_public_outputs(
    run_dir: &Path,
    report: &HillslopeRunReport,
    wat_path: &Path,
    expected_ofe_count: usize,
) {
    const INV_OFEROUTE_014_SOURCE_DEPTH_TOLERANCE_M: f64 = 1.0e-12;
    const ROUTED_HYDROGRAPH_PUBLICATION_REL_TOLERANCE: f64 = 1.0e-9;
    const HOUR_SECONDS: f64 = 3_600.0;

    let hbp_bytes = std::fs::read(&report.output_pass).expect("read routed public HBP");
    let (hbp, latest_event) = parse_hbp_from_bytes_with_latest_event_payload(
        &hbp_bytes,
        &report.output_pass,
        HbpParseOptions {
            expected_hillslope_id: Some(83),
        },
    )
    .expect("routed public HBP must parse through the production consumer");
    assert_eq!(hbp.schema_minor, 1, "routed hourly HBP payload");
    assert_eq!(
        usize::from(hbp.nofe),
        expected_ofe_count,
        "exact routed HBP topology"
    );
    let event = latest_event.expect("positive routed day must publish an HBP event payload");
    assert_eq!(event.hourly_runoff_volume_m3.len(), 24);

    let pass_path = run_dir.join("output/H83.pass.parquet");
    let pass_runvol = read_wat_f64_column(&pass_path, "runvol");
    let pass_peakro = read_wat_f64_column(&pass_path, "peakro");
    assert_eq!(pass_runvol.len(), 1, "one outlet PASS row per routed day");
    assert_eq!(pass_peakro.len(), 1, "one outlet PASS peak per routed day");
    let terminal_runvol_m3 = *pass_runvol.last().expect("terminal PASS runvol");
    let terminal_peakro_m3_s = *pass_peakro.last().expect("terminal PASS peakro");
    assert!(
        terminal_runvol_m3 > 0.0 && terminal_peakro_m3_s > 0.0,
        "positive routed qualification must publish positive terminal PASS volume and peak"
    );

    let manifest: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(&report.manifest_path).expect("read routed manifest"),
    )
    .expect("parse routed manifest");
    let active = &manifest["execution_provenance"]["laned_active"];
    let manifest_source_m3 = positive_manifest_f64(active, "total_source_m3");
    let manifest_outlet_m3 = positive_manifest_f64(active, "total_routed_outlet_m3");
    let manifest_storage_m3 = nonnegative_manifest_f64(active, "total_end_window_storage_m3");
    let manifest_clamp_m3 = nonnegative_manifest_f64(active, "total_clamp_m3");

    let routed_hourly_sum_m3 = event.hourly_runoff_volume_m3.iter().sum::<f64>();
    assert_relative_close(
        routed_hourly_sum_m3,
        manifest_outlet_m3,
        ROUTED_HYDROGRAPH_PUBLICATION_REL_TOLERANCE,
        "SC-OFEROUTE-001 INV-OFEROUTE-014 terminal routed book -> HBP volume",
    );
    assert_relative_close(
        terminal_runvol_m3,
        manifest_outlet_m3,
        ROUTED_HYDROGRAPH_PUBLICATION_REL_TOLERANCE,
        "SC-OFEROUTE-001 INV-OFEROUTE-014 terminal routed book -> PASS runvol",
    );
    let reconstructed_peak_m3_s = event
        .hourly_runoff_volume_m3
        .iter()
        .copied()
        .fold(0.0_f64, f64::max)
        / HOUR_SECONDS;
    assert_relative_close(
        reconstructed_peak_m3_s,
        event.peak_runoff_m3_s,
        ROUTED_HYDROGRAPH_PUBLICATION_REL_TOLERANCE,
        "SC-WATBAL-001 INV-WATBAL-102/104 routed HBP hourly maximum -> HBP peak",
    );
    assert_relative_close(
        reconstructed_peak_m3_s,
        terminal_peakro_m3_s,
        ROUTED_HYDROGRAPH_PUBLICATION_REL_TOLERANCE,
        "SC-WATBAL-001 INV-WATBAL-102/104 routed HBP hourly maximum -> PASS peakro",
    );

    let source_depths_mm = read_wat5_hourly_sources(
        &run_dir.join("output/H83.wat-subhourly.parquet"),
        expected_ofe_count,
    );
    let areas_m2 = read_wat_f64_column(wat_path, "Area");
    let source_volume_m3 = source_depths_mm
        .iter()
        .zip(&areas_m2)
        .map(|(depths_mm, area_m2)| depths_mm.iter().sum::<f64>() / 1_000.0 * area_m2)
        .sum::<f64>();
    let source_volume_tolerance_m3 =
        INV_OFEROUTE_014_SOURCE_DEPTH_TOLERANCE_M * areas_m2.iter().sum::<f64>();
    assert!(
        (source_volume_m3 - manifest_source_m3).abs() <= source_volume_tolerance_m3,
        "INV-OFEROUTE-014 all-OFE WAT5 accepted 24-bin sources -> Lane-D source book: source={source_volume_m3}, manifest={manifest_source_m3}, tolerance={source_volume_tolerance_m3}"
    );
    assert_relative_close(
        manifest_source_m3 + manifest_clamp_m3,
        manifest_outlet_m3 + manifest_storage_m3,
        ROUTED_HYDROGRAPH_PUBLICATION_REL_TOLERANCE,
        "Lane-D all-OFE source + clamp = terminal routed outlet + end-window storage",
    );

    let source_hourly_m3 = std::array::from_fn::<_, 24, _>(|hour| {
        source_depths_mm
            .iter()
            .zip(&areas_m2)
            .map(|(depths_mm, area_m2)| depths_mm[hour] / 1_000.0 * area_m2)
            .sum::<f64>()
    });
    let source_weights = source_hourly_m3.map(|volume| volume / manifest_source_m3);
    let routed_weights = std::array::from_fn::<_, 24, _>(|hour| {
        event.hourly_runoff_volume_m3[hour] / terminal_runvol_m3
    });
    let maximum_shape_delta = source_weights
        .iter()
        .zip(routed_weights.iter())
        .map(|(source, routed)| (source - routed).abs())
        .fold(0.0_f64, f64::max);
    assert!(
        maximum_shape_delta > ROUTED_HYDROGRAPH_PUBLICATION_REL_TOLERANCE,
        "routed-vs-unrouted poison: public HBP/PASS must consume Lane-D routed weights, not the pre-route WB14/WAT5 source shape"
    );
}

fn reconstruct_sparse_wat5_hourly_sources(
    rows: impl IntoIterator<Item = (usize, usize, usize, f64)>,
    ofe_count: usize,
) -> Result<Vec<[f64; 24]>, String> {
    let mut hourly: Vec<[Option<f64>; 24]> = vec![[None; 24]; ofe_count];
    let mut previous_key = None;
    for (ofe, hour, subinterval, depth_mm) in rows {
        if !(1..=ofe_count).contains(&ofe) {
            return Err(format!("WAT5 OFE {ofe} is outside topology"));
        }
        if hour >= 24 || subinterval >= 24 * 12 || hour != subinterval / 12 {
            return Err(format!(
                "WAT5 noncanonical hour/bin pair ofe={ofe} hour={hour} bin={subinterval}"
            ));
        }
        if !depth_mm.is_finite() || depth_mm < 0.0 {
            return Err(format!(
                "WAT5 invalid hourly authority ofe={ofe} hour={hour} depth={depth_mm}"
            ));
        }
        let key = (ofe, subinterval);
        if previous_key.is_some_and(|previous| key <= previous) {
            return Err(format!(
                "WAT5 duplicate or noncanonical row order at ofe={ofe} bin={subinterval}"
            ));
        }
        previous_key = Some(key);

        let slot = &mut hourly[ofe - 1][hour];
        if let Some(existing) = slot {
            if existing.to_bits() != depth_mm.to_bits() {
                return Err(format!(
                    "WAT5 inconsistent repeated hourly authority at ofe={ofe} hour={hour}"
                ));
            }
        } else {
            *slot = Some(depth_mm);
        }
    }

    Ok(hourly
        .into_iter()
        .map(|lane| lane.map(|value| value.unwrap_or(0.0)))
        .collect())
}

fn read_wat5_hourly_sources(path: &Path, ofe_count: usize) -> Vec<[f64; 24]> {
    use arrow_array::{Float64Array, Int32Array};
    use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

    let mut rows = Vec::new();
    for batch in ParquetRecordBatchReaderBuilder::try_new(
        std::fs::File::open(path).expect("open public WAT5 source parquet"),
    )
    .expect("public WAT5 parquet metadata")
    .build()
    .expect("public WAT5 parquet reader")
    {
        let batch = batch.expect("public WAT5 record batch");
        let ofe_ids = batch
            .column_by_name("ofe_id")
            .expect("WAT5 ofe_id")
            .as_any()
            .downcast_ref::<Int32Array>()
            .expect("WAT5 ofe_id Int32");
        let hour_indices = batch
            .column_by_name("hour_index")
            .expect("WAT5 hour_index")
            .as_any()
            .downcast_ref::<Int32Array>()
            .expect("WAT5 hour_index Int32");
        let subinterval_indices = batch
            .column_by_name("subinterval_index")
            .expect("WAT5 subinterval_index")
            .as_any()
            .downcast_ref::<Int32Array>()
            .expect("WAT5 subinterval_index Int32");
        let depths = batch
            .column_by_name("hourly_authoritative_runoff_depth_mm")
            .expect("WAT5 hourly authoritative runoff")
            .as_any()
            .downcast_ref::<Float64Array>()
            .expect("WAT5 hourly authoritative runoff Float64");
        for row in 0..batch.num_rows() {
            let ofe = usize::try_from(ofe_ids.value(row)).expect("positive WAT5 OFE id");
            let hour = usize::try_from(hour_indices.value(row)).expect("non-negative WAT5 hour");
            let subinterval = usize::try_from(subinterval_indices.value(row))
                .expect("non-negative WAT5 subinterval");
            rows.push((ofe, hour, subinterval, depths.value(row)));
        }
    }
    reconstruct_sparse_wat5_hourly_sources(rows, ofe_count)
        .unwrap_or_else(|detail| panic!("canonical WAT5 sparse source reconstruction: {detail}"))
}

#[test]
fn wat5_sparse_and_dense_hourly_source_reconstruction_are_equivalent() {
    let depths = std::array::from_fn::<_, 24, _>(|hour| {
        if hour < 7 {
            f64::from(u32::try_from(hour + 1).expect("hour depth"))
        } else {
            0.0
        }
    });
    let dense = (0..24).map(|hour| (1, hour, hour * 12, depths[hour]));
    let sparse = (0..7).map(|hour| (1, hour, hour * 12, depths[hour]));

    let dense = reconstruct_sparse_wat5_hourly_sources(dense, 1).expect("dense WAT5 source");
    let sparse = reconstruct_sparse_wat5_hourly_sources(sparse, 1).expect("sparse WAT5 source");
    assert_eq!(
        sparse[0].map(f64::to_bits),
        dense[0].map(f64::to_bits),
        "contractually omitted hours must reconstruct as exact positive zero"
    );
}

#[test]
fn wat5_hourly_source_reconstruction_rejects_duplicate_or_noncanonical_rows() {
    let duplicate = [(1, 0, 0, 1.0), (1, 0, 0, 1.0)];
    assert!(
        reconstruct_sparse_wat5_hourly_sources(duplicate, 1).is_err(),
        "duplicate WAT5 row must be poison"
    );
    let reversed = [(1, 0, 1, 1.0), (1, 0, 0, 1.0)];
    assert!(
        reconstruct_sparse_wat5_hourly_sources(reversed, 1).is_err(),
        "noncanonical WAT5 row order must be poison"
    );
    let wrong_hour = [(1, 1, 0, 1.0)];
    assert!(
        reconstruct_sparse_wat5_hourly_sources(wrong_hour, 1).is_err(),
        "hour/bin relabeling must be poison"
    );
    let inconsistent_hour = [(1, 0, 0, 1.0), (1, 0, 1, 2.0)];
    assert!(
        reconstruct_sparse_wat5_hourly_sources(inconsistent_hour, 1).is_err(),
        "one emitted hour must retain bit-identical authority"
    );
}

fn read_wat_i16_column(path: &Path, name: &str) -> Vec<i16> {
    use arrow_array::Int16Array;
    use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

    ParquetRecordBatchReaderBuilder::try_new(
        std::fs::File::open(path).expect("open Lane-D WAT parquet"),
    )
    .expect("Lane-D WAT parquet metadata")
    .build()
    .expect("Lane-D WAT parquet reader")
    .flat_map(|batch| {
        let batch = batch.expect("Lane-D WAT record batch");
        let column = batch
            .column_by_name(name)
            .unwrap_or_else(|| panic!("Lane-D WAT column {name}"))
            .as_any()
            .downcast_ref::<Int16Array>()
            .unwrap_or_else(|| panic!("Lane-D WAT column {name} must be Int16"))
            .iter()
            .map(|value| value.expect("non-null Lane-D WAT value"))
            .collect::<Vec<_>>();
        column
    })
    .collect()
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
    let snow = read_wat_f64_column(&wat, "Snow-Water");
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

include!("stage3_runner_qualification/complete_season_reappearance.rs");
