use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use openwepp_meteorology::phase::hydrometeor_temperature_from_relative_humidity;
use openwepp_meteorology::psychrometrics::relative_humidity_from_dew_point;
use openwepp_unit_boundary::TemperatureCelsius;
use sha2::{Digest, Sha256};

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEFORCING-001.md";
const PACKAGE: &str = "docs/work-packages/20260817-snow-free-half-hour-forcing-authority-001";

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

fn text(path: &str) -> String {
    fs::read_to_string(root().join(path)).unwrap_or_else(|error| panic!("read {path}: {error}"))
}

fn sha256(path: &str) -> String {
    format!(
        "{:x}",
        Sha256::digest(fs::read(root().join(path)).expect("authority bytes"))
    )
}

#[test]
fn contract_binds_complete_operator_and_protected_boundaries() {
    let contract = text(CONTRACT);
    for marker in [
        "OPENWEPP_SNOW_FREE_HALF_HOUR_FORCING_V1",
        "INV-SFF-001",
        "INV-SFF-012",
        "WEISS_NORMAN_1985",
        "E_g,h = radmj * radcur_h / rpoth",
        "VPD=0.611 exp",
        "atmospheric-only",
        "zero-order parent-hour refinement",
        "typed unsupported",
    ] {
        assert!(
            contract.contains(marker),
            "missing contract marker {marker}"
        );
    }
    assert!(!contract.contains("production selector activation"));
}

#[test]
fn independent_calculator_regenerates_frozen_vectors_exactly() {
    let calculator = root().join(format!("{PACKAGE}/artifacts/reference_calculator.py"));
    let fixture = fs::read(root().join(format!(
        "{PACKAGE}/artifacts/openwepp_snow_free_half_hour_forcing_v1_vectors.json"
    )))
    .expect("frozen forcing vectors");
    let first = Command::new(root().join(".venv/bin/python"))
        .arg(&calculator)
        .current_dir(root())
        .output()
        .expect("execute independent forcing calculator");
    let second = Command::new(root().join(".venv/bin/python"))
        .arg(calculator)
        .current_dir(root())
        .output()
        .expect("repeat independent forcing calculator");
    assert!(
        first.status.success(),
        "first calculator stderr: {}",
        String::from_utf8_lossy(&first.stderr)
    );
    assert!(
        second.status.success(),
        "second calculator stderr: {}",
        String::from_utf8_lossy(&second.stderr)
    );
    assert_eq!(first.stdout, fixture);
    assert_eq!(second.stdout, fixture);
}

#[test]
fn definition_binds_all_authority_bytes_and_schema_validates_complete_receipt() {
    let artifacts = format!("{PACKAGE}/artifacts");
    let definition: serde_json::Value =
        serde_json::from_str(&text(&format!("{artifacts}/model-definition.json")))
            .expect("forcing model definition");
    for (field, path) in [
        ("contract_sha256", CONTRACT.to_string()),
        (
            "reference_calculator_sha256",
            format!("{artifacts}/reference_calculator.py"),
        ),
        (
            "vectors_sha256",
            format!("{artifacts}/openwepp_snow_free_half_hour_forcing_v1_vectors.json"),
        ),
        (
            "forcing_schema_sha256",
            format!("{artifacts}/forcing-schema.json"),
        ),
        (
            "receipt_provider_definition_sha256",
            format!("{artifacts}/receipt-provider-definition.json"),
        ),
    ] {
        assert_eq!(definition[field], sha256(&path), "stale {field}");
    }

    let schema: serde_json::Value =
        serde_json::from_str(&text(&format!("{artifacts}/forcing-schema.json")))
            .expect("forcing schema");
    let vectors: serde_json::Value = serde_json::from_str(&text(&format!(
        "{artifacts}/openwepp_snow_free_half_hour_forcing_v1_vectors.json"
    )))
    .expect("forcing vectors");
    jsonschema::draft202012::new(&schema)
        .expect("compile forcing schema")
        .validate(&vectors["complete_day_receipt"])
        .expect("complete receipt satisfies forcing schema");
    assert_eq!(
        vectors["complete_day_receipt"]["provider_definition_sha256"],
        definition["receipt_provider_definition_sha256"]
    );
    for (field, expected) in [
        (
            "precipitation_phase",
            "HARDER_POMEROY_HOURLY_RAIN_SNOW_FRACTIONS",
        ),
        (
            "phase_scalar_custody",
            "ACTIVE_DEPTH_EQUALS_RAIN_DEPTH_PLUS_SNOWFALL_DEPTH_DIVIDED_BY_TEN",
        ),
        (
            "solid_precipitation_custody",
            "SNOWFALL_DEPTH_TIMES_100_WITH_CP_ICE_2100_AND_TEMPERATURE_BOUNDED_AT_273_15_K",
        ),
        (
            "midnight_carry",
            "PHASE_SEPARATE_LIQUID_AND_SOLID_PARCELS_WITH_TRANSLATED_SUPPORT",
        ),
    ] {
        assert_eq!(definition[field], expected, "missing semantic {field}");
    }
}

#[test]
fn vector_inventory_covers_required_branch_families() {
    let vectors = text(&format!(
        "{PACKAGE}/artifacts/openwepp_snow_free_half_hour_forcing_v1_vectors.json"
    ));
    assert_named_inventory(&vectors);
    let value: serde_json::Value = serde_json::from_str(&vectors).expect("forcing vectors JSON");
    let receipt = &value["complete_day_receipt"];
    let intervals = receipt["intervals"].as_array().expect("48 intervals");
    assert_complete_receipt(receipt, intervals);
    let cases = value["cases"].as_array().expect("authority cases");
    assert_poison_and_carry_cases(cases);
    assert_repository_hydrometeor_parity(intervals);
}

fn assert_named_inventory(vectors: &str) {
    for name in [
        "dry_clear_summer",
        "dry_cloudy",
        "night",
        "dawn",
        "low_transmissivity",
        "low_pressure",
        "humid_positive_vpd",
        "dewpoint_equal_air",
        "dewpoint_above_air",
        "breakpoint_cross_half_hour_and_midnight",
        "mixed_phase_partition",
        "fallback_parent_hour_split",
        "support_continuity",
        "zero_wind",
        "missing_interval",
        "duplicate_interval",
        "mixed_provider_version",
        "heterogeneous_multi_ofe",
    ] {
        assert!(vectors.contains(name), "missing vector {name}");
    }
}

fn assert_complete_receipt(receipt: &serde_json::Value, intervals: &[serde_json::Value]) {
    assert_eq!(intervals.len(), 48);
    let energy: f64 = intervals
        .iter()
        .map(|interval| {
            interval["global_horizontal_shortwave_w_m2"]
                .as_f64()
                .expect("global shortwave")
                * 1800.0
                / 1_000_000.0
        })
        .sum();
    let admitted_daily_energy = receipt["daily_horizontal_energy_mj_m2"]
        .as_f64()
        .expect("daily horizontal energy");
    let closure_tolerance = 64.0 * f64::EPSILON * admitted_daily_energy.abs().max(1.0);
    assert!((energy - admitted_daily_energy).abs() <= closure_tolerance);
    let mut active_precipitation_m = 0.0;
    let mut parcel_mass_kg_m2 = 0.0;
    let mut saw_mixed_phase = false;
    for (index, interval) in intervals.iter().enumerate() {
        assert_eq!(interval["interval_index"].as_u64(), Some(index as u64));
        for field in [
            "provider_definition_sha256",
            "source_climate_sha256",
            "ofe_id",
            "tile_id",
            "transaction_id",
            "solar_zenith_cosine",
            "actual_vapor_pressure_kpa",
            "co2_pa",
            "gsi_receipt_sha256",
            "wb14_configuration_sha256",
            "active_precipitation_m",
            "rain_m",
            "snowfall_m",
            "rain_fraction",
            "snow_fraction",
            "precipitation_parcels",
            "solid_precipitation_parcels",
            "interval_receipt_sha256",
        ] {
            assert!(!interval[field].is_null(), "interval missing {field}");
        }
        let active = interval["active_precipitation_m"]
            .as_f64()
            .expect("active precipitation");
        let rain = interval["rain_m"].as_f64().expect("rain depth");
        let snowfall = interval["snowfall_m"].as_f64().expect("snowfall depth");
        let rain_fraction = interval["rain_fraction"].as_f64().expect("rain fraction");
        let snow_fraction = interval["snow_fraction"].as_f64().expect("snow fraction");
        let scale = active.abs().max(1.0);
        assert!((rain + snowfall / 10.0 - active).abs() <= 1.0e-12 * scale);
        active_precipitation_m += active;
        let liquid = interval["precipitation_parcels"]
            .as_array()
            .expect("liquid parcels");
        let solid = interval["solid_precipitation_parcels"]
            .as_array()
            .expect("solid parcels");
        if active == 0.0 {
            assert_eq!(rain_fraction, 0.0);
            assert_eq!(snow_fraction, 0.0);
            assert!(interval["hydrometeor_temperature_c"].is_null());
            assert!(liquid.is_empty() && solid.is_empty());
        } else {
            assert!((rain_fraction + snow_fraction - 1.0).abs() <= 1.0e-12);
            assert!((rain - active * rain_fraction).abs() <= 1.0e-12 * scale);
            assert!((snowfall / 10.0 - active * snow_fraction).abs() <= 1.0e-12 * scale);
            assert!(interval["hydrometeor_temperature_c"].as_f64().is_some());
            saw_mixed_phase |= !liquid.is_empty() && !solid.is_empty();
        }
        for parcel in liquid.iter().chain(solid) {
            parcel_mass_kg_m2 += parcel["mass_kg_m2"].as_f64().expect("parcel mass");
        }
        for parcel in solid {
            let mass = parcel["mass_kg_m2"].as_f64().expect("solid mass");
            let temperature_k = parcel["temperature_k"].as_f64().expect("solid temperature");
            let enthalpy = parcel["enthalpy_j_m2"].as_f64().expect("solid enthalpy");
            let expected = mass * 2_100.0 * (temperature_k - 273.15);
            assert!(temperature_k <= 273.15);
            assert!((enthalpy - expected).abs() <= 1.0e-12 * expected.abs().max(1.0));
        }
    }
    assert!(
        saw_mixed_phase,
        "frozen receipt must exercise typed mixed phase"
    );
    assert!((active_precipitation_m - 0.0054).abs() <= 1.0e-12);
    assert!(
        (parcel_mass_kg_m2 - active_precipitation_m * 1_000.0).abs()
            <= 1.0e-12 * parcel_mass_kg_m2.abs().max(1.0)
    );
    assert!(
        receipt["next_day_solid_precipitation_carry"]
            .as_array()
            .expect("day solid carry")
            .is_empty()
    );
}

fn authority_case<'a>(cases: &'a [serde_json::Value], name: &str) -> &'a serde_json::Value {
    cases
        .iter()
        .find(|case| case["name"] == name)
        .unwrap_or_else(|| panic!("missing case {name}"))
}

fn assert_poison_and_carry_cases(cases: &[serde_json::Value]) {
    let case = |name| authority_case(cases, name);
    assert_eq!(
        case("missing_interval")["reason"],
        "receipt:support_identity"
    );
    assert_eq!(
        case("duplicate_interval")["reason"],
        "receipt:support_identity"
    );
    assert_eq!(
        case("mixed_provider_version")["reason"],
        "receipt:provider_identity"
    );
    assert_eq!(
        case("one_bit_physical_operand")["reason"],
        "receipt:interval_digest"
    );
    assert_eq!(
        case("heterogeneous_multi_ofe")["reason"],
        "receipt:unsupported_global_atmospheric_heterogeneity"
    );
    let mixed = case("mixed_phase_partition");
    assert_eq!(mixed["status"], "accepted");
    assert!(mixed["rain_fraction"].as_f64().expect("rain fraction") > 0.0);
    assert!(mixed["snow_fraction"].as_f64().expect("snow fraction") > 0.0);
    assert_eq!(
        mixed["solid_precipitation_parcels"]
            .as_array()
            .expect("mixed solid parcels")
            .len(),
        1
    );
    assert_eq!(case("digest_operand_matrix")["changed_field_count"], 39);
    assert!(
        case("dewpoint_equal_air")["lse_atmospheric_receipt"]["downward_longwave_w_m2"]
            .as_f64()
            .is_some()
    );

    let carry_case = case("event_relative_midnight_carry");
    let carry = carry_case["next_day_precipitation_carry"]
        .as_array()
        .expect("midnight liquid carry");
    let solid_carry = carry_case["next_day_solid_precipitation_carry"]
        .as_array()
        .expect("midnight solid carry");
    assert_eq!(carry.len(), 1);
    assert_eq!(solid_carry.len(), 1);
    for field in [
        "parcel_id",
        "source_owner_id",
        "destination_ofe_id",
        "destination_tile_id",
        "start_s",
        "end_s",
        "mass_kg_m2",
        "temperature_k",
        "enthalpy_j_m2",
    ] {
        assert!(!carry[0][field].is_null(), "carry missing {field}");
        assert!(
            !solid_carry[0][field].is_null(),
            "solid carry missing {field}"
        );
    }
    let carry_mass = carry[0]["mass_kg_m2"].as_f64().expect("liquid carry mass")
        + solid_carry[0]["mass_kg_m2"]
            .as_f64()
            .expect("solid carry mass");
    let active_carry = carry_case["active_precipitation_m"]
        .as_f64()
        .expect("active carry depth");
    assert!((carry_mass - active_carry * 1_000.0).abs() <= 1.0e-12 * carry_mass);
}

fn assert_repository_hydrometeor_parity(intervals: &[serde_json::Value]) {
    let rainy_interval = intervals
        .iter()
        .find(|interval| {
            !interval["precipitation_parcels"]
                .as_array()
                .expect("precipitation parcels")
                .is_empty()
        })
        .expect("rainy interval");
    let parcel = &rainy_interval["precipitation_parcels"][0];
    let air_c = rainy_interval["air_temperature_c"].as_f64().expect("air");
    let dew_c = rainy_interval["dew_point_c"].as_f64().expect("dew point");
    let air = TemperatureCelsius::try_new(air_c).expect("air temperature");
    let dew = TemperatureCelsius::try_new(dew_c).expect("dew-point temperature");
    let relative_humidity = relative_humidity_from_dew_point(air, dew).expect("relative humidity");
    let repository_parent = hydrometeor_temperature_from_relative_humidity(air, relative_humidity)
        .expect("repository Harder-Pomeroy parent");
    let parcel_temperature_k = parcel["temperature_k"].as_f64().expect("hydrometeor");
    let interval_hydrometeor_c = rainy_interval["hydrometeor_temperature_c"]
        .as_f64()
        .expect("interval hydrometeor");
    assert_eq!(
        (interval_hydrometeor_c + 273.15).to_bits(),
        parcel_temperature_k.to_bits()
    );
    let repository_temperature_k = repository_parent.temperature.as_celsius() + 273.15;
    assert!(
        (parcel_temperature_k - repository_temperature_k).abs()
            <= 1.0e-12 * repository_temperature_k.abs().max(1.0)
    );
    let air_kelvin = air_c + 273.15;
    assert_ne!(
        parcel["temperature_k"]
            .as_f64()
            .expect("hydrometeor")
            .to_bits(),
        air_kelvin.to_bits()
    );
}
