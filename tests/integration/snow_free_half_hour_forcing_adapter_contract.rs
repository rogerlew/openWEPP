use openwepp_hillslope_orchestrator::runtime_inputs::{
    DirectGsiDailyReceiptV1, DirectGsiOwnerConfigurationV1, SnowFreeHalfHourDestination,
    SnowFreeHalfHourForcingError, SnowFreeHalfHourProviderConfiguration,
    SnowFreeHalfHourProviderCursor, SnowFreeHalfHourStaticConfiguration,
    ValidatedSnowFreeHalfHourForcingReceipts, build_hillslope_climate_runtime_request,
};
use openwepp_input_contract::parsers::climate::{ParserMode, parse_climate_from_str};
use openwepp_meteorology::snow_free_forcing::{
    atmospheric_longwave_dilley_unsworth, fao56_station_pressure_kpa, weiss_norman_partition,
};
use openwepp_plant_phenology::{GsiDailyForcing, GsiDate, GsiParameters, GsiState};

const PACKAGE: &str =
    "docs/work-packages/20260817-snow-free-half-hour-forcing-authority-001/artifacts";

#[test]
fn closure_eligible_v10_api_has_no_raw_or_defaulted_execution_escape() {
    let forcing_source = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/crates/openwepp-hillslope-orchestrator/src/runtime_inputs/09_snow_free_half_hour_forcing.rs"
    ));
    assert!(!forcing_source.contains("pub fn snow_free_half_hour_forcing_receipts("));
    assert!(!forcing_source.contains("pub fn prepare_snow_free_gsi_day("));

    let consumer_source = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow.rs"
    ));
    let v10_start = consumer_source
        .find("impl DirectV10RealConsumerShadow")
        .expect("V10 implementation");
    let v10_end = consumer_source[v10_start..]
        .find("impl DirectV9RealConsumerShadow")
        .map(|offset| v10_start + offset)
        .expect("following V9 implementation");
    let v10_api = &consumer_source[v10_start..v10_end];
    assert!(!v10_api.contains("pub fn execute_day("));
    assert!(!v10_api.contains("pub fn project_repository_forcing_receipts("));
    assert!(!v10_api.contains("GsiParameters::generalized()"));
    assert!(!v10_api.contains("GsiState::new()"));
    assert!(!v10_api.contains("SnowFreeHalfHourProviderCursor::default()"));
    assert!(v10_api.contains("pub fn execute_prepared_gsi_day("));
}

/// Integration-test-only adapter retaining legacy forcing-focused assertions
/// after the production legacy entry point became crate-private.
trait LegacySnowFreeProjectionTestOnly {
    fn snow_free_half_hour_forcing_receipts(
        &self,
        day_index: usize,
        configuration: &SnowFreeHalfHourProviderConfiguration,
        cursor: &SnowFreeHalfHourProviderCursor,
    ) -> Result<ValidatedSnowFreeHalfHourForcingReceipts, SnowFreeHalfHourForcingError>;
}

impl LegacySnowFreeProjectionTestOnly
    for openwepp_hillslope_orchestrator::runtime_inputs::HillslopeClimateRuntimeRequest
{
    fn snow_free_half_hour_forcing_receipts(
        &self,
        day_index: usize,
        configuration: &SnowFreeHalfHourProviderConfiguration,
        cursor: &SnowFreeHalfHourProviderCursor,
    ) -> Result<ValidatedSnowFreeHalfHourForcingReceipts, SnowFreeHalfHourForcingError> {
        if !(0.0..=1.0).contains(&configuration.gsi)
            || configuration.gsi_receipt_sha256.len() != 64
            || !configuration
                .gsi_receipt_sha256
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        {
            return Err(SnowFreeHalfHourForcingError::Identity(
                "provider configuration",
            ));
        }
        let owner = DirectGsiOwnerConfigurationV1::try_new(
            "legacy-test-only-gsi-owner".into(),
            GsiParameters::generalized(),
            41.1,
        )?;
        let static_configuration = SnowFreeHalfHourStaticConfiguration {
            run_id: configuration.run_id.clone(),
            co2_pa: configuration.co2_pa,
            reference_height_m: configuration.reference_height_m,
            gsi_owner_configuration_sha256: owner.configuration_sha256.clone(),
            destinations: configuration.destinations.clone(),
        };
        Ok(self
            .prepare_snow_free_gsi_day_from_repository(
                day_index,
                &static_configuration,
                &owner,
                &GsiState::new(),
                cursor,
            )?
            .forcing_receipts()
            .clone())
    }
}

#[test]
fn direct_gsi_daily_receipt_reconstructs_cp_gsi01_state_and_rejects_poison() {
    let beginning = GsiState::new();
    let forcing = GsiDailyForcing {
        minimum_temperature_c: 4.0,
        vapor_pressure_deficit_pa: 800.0,
        latitude_degrees: 41.1,
        date: GsiDate {
            year: 2000,
            ordinal_day: 172,
        },
    };
    let owner = DirectGsiOwnerConfigurationV1::try_new(
        "gsi-owner".into(),
        GsiParameters::generalized(),
        41.1,
    )
    .expect("GSI owner");
    let (receipt, ending) = DirectGsiDailyReceiptV1::prepare_owned(
        &beginning,
        &owner,
        "test-run",
        0,
        &"a".repeat(64),
        forcing,
    )
    .expect("accepted CP-GSI01 daily receipt");
    receipt.validate().expect("reconstructed receipt");
    assert_eq!(receipt.ending_state.history_oldest_first, ending.history());
    assert_eq!(receipt.result.sample_count, 1);

    let mut poison = receipt;
    poison.result.growing_season_index =
        f64::from_bits(poison.result.growing_season_index.to_bits() + 1);
    assert!(matches!(
        poison.validate(),
        Err(SnowFreeHalfHourForcingError::Identity("daily GSI receipt"))
    ));
}

#[test]
fn static_provider_cursor_accepts_changing_daily_gsi_and_rejects_wrong_owner() {
    let climate_source = format!(
        "{}{}",
        warm_non_breakpoint_climate(),
        "21 6 2000 0.0 0.0 0.0 0.0 29.0 23.0 430.0 2.5 180.0 20.0\n"
    );
    let climate =
        parse_climate_from_str(&climate_source, ParserMode::Strict).expect("two-day climate");
    let request = build_hillslope_climate_runtime_request(&climate).expect("runtime request");
    let parameters = GsiParameters::generalized();
    let owner = DirectGsiOwnerConfigurationV1::try_new("gsi-owner".into(), parameters, 41.1)
        .expect("GSI owner");
    let legacy = provider_configuration();
    let configuration = SnowFreeHalfHourStaticConfiguration {
        run_id: legacy.run_id,
        co2_pa: legacy.co2_pa,
        reference_height_m: legacy.reference_height_m,
        gsi_owner_configuration_sha256: owner.configuration_sha256.clone(),
        destinations: legacy.destinations,
    };
    let mut gsi = GsiState::new();
    let mut cursor = SnowFreeHalfHourProviderCursor::default();
    for day_index in 0..2 {
        let prepared = request
            .prepare_snow_free_gsi_day_from_repository(
                day_index,
                &configuration,
                &owner,
                &gsi,
                &cursor,
            )
            .expect("static plus daily provider projection");
        prepared
            .commit(&mut gsi, &mut cursor)
            .expect("owner commit");
    }

    let mut wrong = configuration;
    wrong.gsi_owner_configuration_sha256 = "f".repeat(64);
    assert!(matches!(
        request.prepare_snow_free_gsi_day_from_repository(2, &wrong, &owner, &gsi, &cursor),
        Err(SnowFreeHalfHourForcingError::Identity(
            "repository GSI owner join"
        ))
    ));
}

#[test]
fn prepared_gsi_provider_day_commits_both_owners_or_neither() {
    let climate = parse_climate_from_str(&warm_non_breakpoint_climate(), ParserMode::Strict)
        .expect("climate");
    let request = build_hillslope_climate_runtime_request(&climate).expect("runtime request");
    let parameters = GsiParameters::generalized();
    let owner = DirectGsiOwnerConfigurationV1::try_new("gsi-owner".into(), parameters, 41.1)
        .expect("GSI owner");
    let legacy = provider_configuration();
    let configuration = SnowFreeHalfHourStaticConfiguration {
        run_id: legacy.run_id,
        co2_pa: legacy.co2_pa,
        reference_height_m: legacy.reference_height_m,
        gsi_owner_configuration_sha256: owner.configuration_sha256.clone(),
        destinations: legacy.destinations,
    };
    let beginning_gsi = GsiState::new();
    let beginning_cursor = SnowFreeHalfHourProviderCursor::default();
    let prepared = request
        .prepare_snow_free_gsi_day_from_repository(
            0,
            &configuration,
            &owner,
            &beginning_gsi,
            &beginning_cursor,
        )
        .expect("prepared atomic owners");
    let mut wrong_gsi = GsiState::new();
    let forcing = GsiDailyForcing {
        minimum_temperature_c: 22.0,
        vapor_pressure_deficit_pa: 800.0,
        latitude_degrees: 41.1,
        date: GsiDate {
            year: 2000,
            ordinal_day: 172,
        },
    };
    wrong_gsi
        .advance(parameters, forcing)
        .expect("different beginning owner");
    let wrong_before = wrong_gsi.clone();
    let mut cursor = beginning_cursor.clone();
    let cursor_before = cursor.clone();
    assert!(matches!(
        prepared.clone().commit(&mut wrong_gsi, &mut cursor),
        Err(SnowFreeHalfHourForcingError::Identity(
            "GSI/provider atomic commit beginning"
        ))
    ));
    assert_eq!(wrong_gsi, wrong_before);
    assert_eq!(cursor, cursor_before);

    let mut gsi = beginning_gsi;
    prepared
        .commit(&mut gsi, &mut cursor)
        .expect("atomic commit");
    assert_eq!(gsi.sample_count(), 1);
    assert_ne!(cursor, beginning_cursor);
}

fn warm_breakpoint_climate(wind_m_s: f64, dew_point_c: f64) -> String {
    format!(
        "5.30\n1 1 0\nTEST STATION 1500\nDAY MON YEAR NBRKPT TMAX TMIN RAD VWIND WIND TDPT\n41.1 -120.0 1225.0 30 2000 1\nMONTHLY MAX TEMP HEADER\n1 2 3 4 5 6 7 8 9 10 11 12\nMONTHLY MIN TEMP HEADER\n-5 -4 -3 -2 -1 0 1 2 3 4 5 6\nMONTHLY RAD HEADER\n100 101 102 103 104 105 106 107 108 109 110 111\nMONTHLY RAIN HEADER\n10 11 12 13 14 15 16 17 18 19 20 21\nDAILY HEADER\nDAILY UNITS\n20 6 2000 3 28.0 22.0 420.0 {wind_m_s} 180.0 {dew_point_c}\n13.25 0.0\n13.75 3.6\n14.00 5.4\n"
    )
}

fn warm_non_breakpoint_climate() -> String {
    "5.30\n1 0 0\nTEST STATION 1500\nDAY MON YEAR PRCP STMDUR TIMEP IP TMAX TMIN RAD VWIND WIND TDPT\n41.1 -120.0 1225.0 30 2000 1 CLIGEN 5.30 --seed 123\nMONTHLY MAX TEMP HEADER\n1 2 3 4 5 6 7 8 9 10 11 12\nMONTHLY MIN TEMP HEADER\n-5 -4 -3 -2 -1 0 1 2 3 4 5 6\nMONTHLY RAD HEADER\n100 101 102 103 104 105 106 107 108 109 110 111\nMONTHLY RAIN HEADER\n10 11 12 13 14 15 16 17 18 19 20 21\nDAILY HEADER\nDAILY UNITS\n20 6 2000 5.4 1.5 0.5 3.6 28.0 22.0 420.0 2.5 180.0 20.0\n".to_string()
}

fn midnight_breakpoint_climate() -> String {
    warm_breakpoint_climate(2.5, 20.0).replace(
        "13.25 0.0\n13.75 3.6\n14.00 5.4",
        "23.50 0.0\n24.00 3.6\n24.50 7.2",
    )
}

fn supersaturated_non_breakpoint_climate() -> String {
    warm_non_breakpoint_climate().replace(
        "5.4 1.5 0.5 3.6 28.0 22.0 420.0 2.5 180.0 20.0",
        "0.0 0.0 0.0 0.0 5.0 5.0 420.0 2.5 180.0 6.0",
    )
}

fn cold_midnight_breakpoint_climate() -> String {
    midnight_breakpoint_climate()
        .replace(
            "3 28.0 22.0 420.0 2.5 180.0 20",
            "3 -5.0 -7.0 420.0 2.5 180.0 -8.0",
        )
        .replace(
            "23.50 0.0\n24.00 3.6\n24.50 7.2",
            "24.00 0.0\n24.25 3.6\n24.50 7.2",
        )
}

fn two_day_midnight_carry_climate() -> String {
    let first = midnight_breakpoint_climate().replace(
        "41.1 -120.0 1225.0 30 2000 1",
        "41.1 -120.0 1225.0 30 2000 2",
    );
    format!("{first}21 6 2000 0 28.0 22.0 420.0 2.5 180.0 20.0\n")
}

fn provider_configuration() -> SnowFreeHalfHourProviderConfiguration {
    SnowFreeHalfHourProviderConfiguration {
        run_id: "adapter-contract-run".to_string(),
        co2_pa: 42.0,
        reference_height_m: 2.0,
        gsi: 0.75,
        gsi_receipt_sha256: "c".repeat(64),
        destinations: vec![
            SnowFreeHalfHourDestination {
                ofe_id: "ofe-1".to_string(),
                tile_id: "forest-1".to_string(),
                wb14_configuration_sha256: "d".repeat(64),
            },
            SnowFreeHalfHourDestination {
                ofe_id: "ofe-2".to_string(),
                tile_id: "open-1".to_string(),
                wb14_configuration_sha256: "e".repeat(64),
            },
        ],
    }
}

fn request(
    source: &str,
) -> openwepp_hillslope_orchestrator::runtime_inputs::HillslopeClimateRuntimeRequest {
    let climate = parse_climate_from_str(source, ParserMode::SnowFreeHalfHourProvider)
        .expect("explicit snow-free provider climate");
    build_hillslope_climate_runtime_request(&climate).expect("hillslope climate request")
}

#[test]
fn actual_breakpoint_climate_projects_complete_digest_bound_receipts() {
    let receipts = request(&warm_breakpoint_climate(2.5, 20.0))
        .snow_free_half_hour_forcing_receipts(
            0,
            &provider_configuration(),
            &SnowFreeHalfHourProviderCursor::default(),
        )
        .expect("snow-free receipt projection");
    assert_eq!(receipts.len(), 2);
    for receipt in receipts.receipts() {
        receipt.validate().expect("receipt closure");
        assert_eq!(receipt.intervals.len(), 48);
        assert_eq!(
            receipt.provider_definition_sha256,
            "4658de9f7590897633ffbfe0facedd52b5c9b9754f7d829f25869ef2c592f153"
        );
        assert!(
            (receipt.daily_horizontal_energy_mj_m2 - 17.5728).abs()
                <= 64.0 * f64::EPSILON * 17.5728
        );
        let rain = receipt
            .intervals
            .iter()
            .flat_map(|interval| &interval.precipitation_parcels)
            .map(|parcel| parcel.mass_kg_m2)
            .sum::<f64>();
        assert_eq!(rain.to_bits(), 5.4_f64.to_bits());
        let late_rain = receipt
            .intervals
            .iter()
            .find(|interval| {
                interval.interval_index > 0 && !interval.precipitation_parcels.is_empty()
            })
            .expect("non-midnight rain receipt");
        assert_eq!(
            late_rain.precipitation_parcels[0].start_s.to_bits(),
            (f64::from(u32::try_from(late_rain.interval_index).expect("bounded interval"))
                * 1_800.0)
                .to_bits()
        );
        for pair in receipt.intervals.chunks_exact(2) {
            assert_eq!(
                pair[0].air_temperature_c.to_bits(),
                pair[1].air_temperature_c.to_bits()
            );
            assert_eq!(
                pair[0].global_horizontal_shortwave_w_m2.to_bits(),
                pair[1].global_horizontal_shortwave_w_m2.to_bits()
            );
        }
    }
    for index in 0..48 {
        assert_eq!(
            receipts[0].intervals[index]
                .global_horizontal_shortwave_w_m2
                .to_bits(),
            receipts[1].intervals[index]
                .global_horizontal_shortwave_w_m2
                .to_bits()
        );
    }
    let first_left = receipts[0]
        .intervals
        .iter()
        .flat_map(|interval| &interval.precipitation_parcels)
        .next()
        .expect("left precipitation parcel");
    let first_right = receipts[1]
        .intervals
        .iter()
        .flat_map(|interval| &interval.precipitation_parcels)
        .next()
        .expect("right precipitation parcel");
    assert_eq!(first_left.parcel_id, first_right.parcel_id);
    assert_ne!(
        first_left.destination_tile_id,
        first_right.destination_tile_id
    );

    let schema: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(format!("{PACKAGE}/forcing-schema.json")).expect("forcing schema"),
    )
    .expect("schema JSON");
    jsonschema::draft202012::new(&schema)
        .expect("compile forcing schema")
        .validate(&serde_json::to_value(&receipts[0]).expect("receipt JSON"))
        .expect("production receipt satisfies authority schema");
}

#[test]
fn provider_rejects_unsupported_domains_and_digest_mutation() {
    let zero_wind = request(&warm_breakpoint_climate(0.0, 20.0))
        .snow_free_half_hour_forcing_receipts(
            0,
            &provider_configuration(),
            &SnowFreeHalfHourProviderCursor::default(),
        )
        .expect_err("zero wind must reject");
    assert_eq!(
        zero_wind,
        SnowFreeHalfHourForcingError::Unsupported("nonpositive wind")
    );

    let saturated = request(&supersaturated_non_breakpoint_climate())
        .snow_free_half_hour_forcing_receipts(
            0,
            &provider_configuration(),
            &SnowFreeHalfHourProviderCursor::default(),
        )
        .expect_err("closure owner rejects negative daily GSI VPD");
    assert_eq!(
        saturated,
        SnowFreeHalfHourForcingError::Unsupported("GSI daily VPD")
    );

    let mut duplicate = provider_configuration();
    duplicate
        .destinations
        .push(duplicate.destinations[0].clone());
    assert_eq!(
        request(&warm_breakpoint_climate(2.5, 20.0))
            .snow_free_half_hour_forcing_receipts(
                0,
                &duplicate,
                &SnowFreeHalfHourProviderCursor::default()
            )
            .expect_err("duplicate destination must reject"),
        SnowFreeHalfHourForcingError::Identity("destination configuration")
    );

    let mut invalid_gsi = provider_configuration();
    invalid_gsi.gsi = 2.0;
    assert_eq!(
        request(&warm_breakpoint_climate(2.5, 20.0))
            .snow_free_half_hour_forcing_receipts(
                0,
                &invalid_gsi,
                &SnowFreeHalfHourProviderCursor::default()
            )
            .expect_err("out-of-domain GSI must reject"),
        SnowFreeHalfHourForcingError::Identity("provider configuration")
    );
    let mut invalid_digest = provider_configuration();
    invalid_digest.gsi_receipt_sha256 = "z".repeat(64);
    assert_eq!(
        request(&warm_breakpoint_climate(2.5, 20.0))
            .snow_free_half_hour_forcing_receipts(
                0,
                &invalid_digest,
                &SnowFreeHalfHourProviderCursor::default()
            )
            .expect_err("nonhex digest must reject"),
        SnowFreeHalfHourForcingError::Identity("provider configuration")
    );

    assert_eq!(
        request(&cold_midnight_breakpoint_climate())
            .snow_free_half_hour_forcing_receipts(
                0,
                &provider_configuration(),
                &SnowFreeHalfHourProviderCursor::default()
            )
            .expect_err("cold carry cannot be relabeled liquid"),
        SnowFreeHalfHourForcingError::Unsupported("snow or mixed precipitation carry")
    );

    let mut receipt = request(&warm_breakpoint_climate(2.5, 20.0))
        .snow_free_half_hour_forcing_receipts(
            0,
            &provider_configuration(),
            &SnowFreeHalfHourProviderCursor::default(),
        )
        .expect("valid receipt")
        .receipts()[0]
        .clone();
    receipt.intervals[47].downward_longwave_w_m2 =
        f64::from_bits(receipt.intervals[47].downward_longwave_w_m2.to_bits() + 1);
    assert_eq!(
        receipt.validate().expect_err("one-bit poison must reject"),
        SnowFreeHalfHourForcingError::Identity("interval receipt")
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn parent_fallback_midnight_carry_and_authority_primitives_are_executable() {
    let fallback = request(&warm_non_breakpoint_climate())
        .snow_free_half_hour_forcing_receipts(
            0,
            &provider_configuration(),
            &SnowFreeHalfHourProviderCursor::default(),
        )
        .expect("parent-hour fallback")
        .receipts()[0]
        .clone();
    let rainy_pair = fallback
        .intervals
        .chunks_exact(2)
        .find(|pair| !pair[0].precipitation_parcels.is_empty())
        .expect("rainy parent hour");
    assert_eq!(
        rainy_pair[0].precipitation_parcels[0].mass_kg_m2.to_bits(),
        rainy_pair[1].precipitation_parcels[0].mass_kg_m2.to_bits()
    );
    let changed_duration = request(&warm_non_breakpoint_climate().replace("5.4 1.5", "5.4 1.6"))
        .snow_free_half_hour_forcing_receipts(
            0,
            &provider_configuration(),
            &SnowFreeHalfHourProviderCursor::default(),
        )
        .expect("changed storm duration")
        .receipts()[0]
        .clone();
    assert_ne!(
        fallback.source_climate_sha256,
        changed_duration.source_climate_sha256
    );

    let carry = request(&midnight_breakpoint_climate())
        .snow_free_half_hour_forcing_receipts(
            0,
            &provider_configuration(),
            &SnowFreeHalfHourProviderCursor::default(),
        )
        .expect("midnight carry")
        .receipts()[0]
        .clone()
        .next_day_precipitation_carry;
    assert_eq!(carry.len(), 1);
    assert_eq!(carry[0].start_s.to_bits(), 0.0_f64.to_bits());
    assert_eq!(carry[0].end_s.to_bits(), 1_800.0_f64.to_bits());
    assert!((carry[0].mass_kg_m2 - 3.6).abs() <= 4.0 * f64::EPSILON);

    let sequential_request = request(&two_day_midnight_carry_climate());
    let owner = DirectGsiOwnerConfigurationV1::try_new(
        "carry-gsi-owner".into(),
        GsiParameters::generalized(),
        41.1,
    )
    .expect("carry GSI owner");
    let legacy_configuration = provider_configuration();
    let static_configuration = SnowFreeHalfHourStaticConfiguration {
        run_id: legacy_configuration.run_id,
        co2_pa: legacy_configuration.co2_pa,
        reference_height_m: legacy_configuration.reference_height_m,
        gsi_owner_configuration_sha256: owner.configuration_sha256.clone(),
        destinations: legacy_configuration.destinations,
    };
    let mut gsi = GsiState::new();
    let mut cursor = SnowFreeHalfHourProviderCursor::default();
    let beginning_cursor = serde_json::to_vec(&cursor).expect("beginning cursor bytes");
    let first_day = sequential_request
        .prepare_snow_free_gsi_day_from_repository(0, &static_configuration, &owner, &gsi, &cursor)
        .expect("first cursor day");
    let carried_source = first_day.forcing_receipts()[0].next_day_precipitation_carry[0]
        .source_owner_id
        .clone();
    assert_eq!(
        serde_json::to_vec(&cursor).expect("uncommitted cursor bytes"),
        beginning_cursor,
        "receipt preparation must not advance provider custody"
    );
    first_day
        .commit(&mut gsi, &mut cursor)
        .expect("commit first day");
    let cursor_bytes = cursor.to_json_bytes().expect("persisted cursor");
    cursor = SnowFreeHalfHourProviderCursor::restore_json(&cursor_bytes, &static_configuration, 1)
        .expect("validated cursor restart");
    let second_day = sequential_request
        .prepare_snow_free_gsi_day_from_repository(1, &static_configuration, &owner, &gsi, &cursor)
        .expect("second cursor day");
    assert!(
        second_day.forcing_receipts()[0].intervals[0]
            .precipitation_parcels
            .iter()
            .any(|parcel| parcel.source_owner_id == carried_source)
    );

    let vectors: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(format!(
            "{PACKAGE}/openwepp_snow_free_half_hour_forcing_v1_vectors.json"
        ))
        .expect("authority vectors"),
    )
    .expect("authority vector JSON");
    let clear = vectors["cases"]
        .as_array()
        .expect("cases")
        .iter()
        .find(|case| case["name"] == "dry_clear_summer")
        .expect("clear vector");
    let pressure = fao56_station_pressure_kpa(250.0).expect("pressure");
    assert_eq!(
        pressure.to_bits(),
        clear["pressure_kpa"]
            .as_f64()
            .expect("vector pressure")
            .to_bits()
    );
    let partition = weiss_norman_partition(700.0, 0.82, pressure).expect("shortwave");
    assert_eq!(
        partition.direct_visible_w_m2.to_bits(),
        clear["shortwave"]["direct_visible_w_m2"]
            .as_f64()
            .expect("direct visible")
            .to_bits()
    );
    let longwave = atmospheric_longwave_dilley_unsworth(
        28.0 + 273.15,
        clear["humidity"]["actual_vapor_pressure_kpa"]
            .as_f64()
            .expect("vapor pressure"),
        0.05,
    )
    .expect("longwave");
    assert_eq!(
        longwave.to_bits(),
        clear["downward_longwave_w_m2"]
            .as_f64()
            .expect("vector longwave")
            .to_bits()
    );
}
