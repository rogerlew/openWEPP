use std::fs;

use openwepp_vegetation::carbon_nitrogen::{
    CnParameters, ElementPool, PhenologyMode, ReceiverClass, Tissue, TissuePool, advance_phenology,
    carbon_offer, finalize_growth, material_transfer,
};
use openwepp_vegetation::energy::{
    energy_residual, neutral_resistance, saturation_specific_humidity,
};
use openwepp_vegetation::interception::{InterceptionInput, liquid_interception};
use openwepp_vegetation::migration::{RhessysSource, migrate_definition_fields};
use openwepp_vegetation::photosynthesis::{FvcbInput, fvcb, medlyn};
use openwepp_vegetation::radiation::two_stream;
use openwepp_vegetation::{
    CoupledOwnedState, MODEL_BYTES, MODEL_SHA256, PhenologyPhase, VegetationConfiguration,
    load_model_definition,
};
use sha2::{Digest, Sha256};

fn expected() -> serde_json::Value {
    serde_json::from_slice(
        &fs::read("tests/fixtures/c3_woody_v1_expected_vectors.json").expect("vector fixture"),
    )
    .expect("valid vector fixture")
}

fn assert_fvcb_vector(
    input: FvcbInput,
    key: &str,
) -> openwepp_vegetation::photosynthesis::FvcbResult {
    let result = fvcb(input).expect("FvCB vector");
    let expected = expected();
    assert!((result.an - expected["photosynthesis"][key].as_f64().unwrap()).abs() < 1e-12);
    result
}

#[test]
fn public_candidate_is_v4_only_and_fail_closed_before_capped_pass() {
    let source = fs::read_to_string("crates/openwepp-vegetation/src/transaction.rs")
        .expect("transaction source");
    assert!(
        source
            .contains("V4 occupancy-local capped transaction routing is implementation-incomplete")
    );
    assert!(source.contains("BTreeMap<OccupancyId, OccupancyState>"));
    assert!(!source.contains("struct StratumState"));
    assert!(!source.contains("pub canopy_liquid: f64"));
    assert!(!source.contains("pub psi_root_mm: f64"));
    assert!(!source.contains("ledger_residuals: [0.0; 5]"));
    assert!(!source.contains("vapor_pressure_deficit_kpa *"));
    assert!(!source.contains("direct_par_w_m2 * 1e-9"));
}

#[test]
fn v4_configuration_state_and_migration_inputs_have_no_default_path() {
    for path in [
        "crates/openwepp-vegetation/src/config.rs",
        "crates/openwepp-vegetation/src/occupancy_state.rs",
        "crates/openwepp-vegetation/src/transaction.rs",
        "crates/openwepp-vegetation/src/migration.rs",
    ] {
        let source = fs::read_to_string(path).expect("V4 source");
        assert!(!source.contains("impl Default for VegetationConfiguration"));
        assert!(!source.contains("impl Default for OccupancyState"));
        assert!(!source.contains("impl Default for CoupledOwnedState"));
        assert!(!source.contains("impl Default for V1CoupledOwnedState"));
        assert!(!source.contains("impl Default for V3CoupledOwnedState"));
        assert!(!source.contains("impl Default for V3VegetationConfiguration"));
    }
}

#[test]
fn production_registry_is_byte_identical_to_authority() {
    let authority = fs::read("docs/work-packages/20260812-c3-woody-shared-state-authority-001/artifacts/openwepp_c3_woody_v4_definition.json")
        .expect("authority definition");
    assert_eq!(MODEL_BYTES, authority);
    assert_eq!(format!("{:x}", Sha256::digest(MODEL_BYTES)), MODEL_SHA256);
    load_model_definition().expect("digest-bound model");
}

#[test]
fn admitted_fvcb_and_medlyn_vectors_match_oracle() {
    assert_fvcb_vector(
        FvcbInput {
            ci_pa: 30.0,
            oi_pa: 20_265.0,
            gamma_pa: 4.275,
            kc_pa: 40.49,
            ko_pa: 27_840.0,
            vcmax: 60.0,
            jmax: 110.0,
            tp: 0.167 * 60.0,
            rd: 1.2,
            par_abs: 0.0,
        },
        "zero_an",
    );
    let saturated = assert_fvcb_vector(
        FvcbInput {
            ci_pa: 30.0,
            oi_pa: 20_265.0,
            gamma_pa: 4.275,
            kc_pa: 40.49,
            ko_pa: 27_840.0,
            vcmax: 70.0,
            jmax: 120.0,
            tp: 0.167 * 70.0,
            rd: 1.2,
            par_abs: 1600.0,
        },
        "saturated_an",
    );
    assert_fvcb_vector(
        FvcbInput {
            ci_pa: 8.0,
            oi_pa: 20_265.0,
            gamma_pa: 4.275,
            kc_pa: 40.49,
            ko_pa: 27_840.0,
            vcmax: 35.0,
            jmax: 160.0,
            tp: 0.167 * 35.0,
            rd: 1.0,
            par_abs: 800.0,
        },
        "rubisco_an",
    );
    assert_fvcb_vector(
        FvcbInput {
            ci_pa: 30.0,
            oi_pa: 20_265.0,
            gamma_pa: 4.275,
            kc_pa: 40.49,
            ko_pa: 27_840.0,
            vcmax: 100.0,
            jmax: 70.0,
            tp: 0.167 * 100.0,
            rd: 1.0,
            par_abs: 45.0,
        },
        "electron_an",
    );
    let gs = medlyn(saturated.an, 25.0, 3.5, 1.4, 39.0, 101_325.0, 1.0).expect("Medlyn");
    let expected = expected();
    assert!(
        (gs - expected["photosynthesis"]["medlyn_gs"]
            .as_f64()
            .expect("Medlyn vector"))
        .abs()
            < 1e-8
    );
    assert!(
        fvcb(FvcbInput {
            vcmax: f64::NAN,
            ..zero_input()
        })
        .is_err()
    );
    assert!(
        fvcb(FvcbInput {
            vcmax: 0.0,
            ..zero_input()
        })
        .is_err()
    );
}

fn zero_input() -> FvcbInput {
    FvcbInput {
        ci_pa: 30.0,
        oi_pa: 20_265.0,
        gamma_pa: 4.275,
        kc_pa: 40.49,
        ko_pa: 27_840.0,
        vcmax: 60.0,
        jmax: 110.0,
        tp: 0.167 * 60.0,
        rd: 1.2,
        par_abs: 0.0,
    }
}

#[test]
fn liquid_interception_matches_fixed_oracle_and_closes() {
    let result = liquid_interception(InterceptionInput {
        store0: 0.2,
        rain: 3.7,
        vapor_amount: 0.42,
        lai: 3.2,
        sai: 0.9,
        alpha_liq: 0.73,
        p_liq: 0.22,
        stemflow_fraction: 0.13,
        leaf_temperature_k: 295.0,
    })
    .expect("liquid interception");
    assert!((result.store1 - 0.482).abs() < 1e-12);
    assert!((result.drainage() - 1.997_516_728_800_741).abs() < 1e-12);
    assert!(result.closure_residual.abs() < 1e-12);
    assert!(
        liquid_interception(InterceptionInput {
            leaf_temperature_k: 268.0,
            ..InterceptionInput {
                store0: 0.2,
                rain: 3.7,
                vapor_amount: 0.42,
                lai: 3.2,
                sai: 0.9,
                alpha_liq: 0.73,
                p_liq: 0.22,
                stemflow_fraction: 0.13,
                leaf_temperature_k: 295.0
            }
        })
        .is_err()
    );
}

#[test]
fn two_stream_rejects_beer_lambert_poison() {
    let expected = expected();
    let result =
        two_stream(3.2, 0.68, 0.1, 0.08, 0.05, 0.14, 620.0, 90.0).expect("two-stream radiation");
    assert!(
        (result.absorbed
            - expected["radiation"]["absorbed"]
                .as_f64()
                .expect("radiation vector"))
        .abs()
            < 1e-7
    );
    let beer = 710.0 * (1.0_f64 - (-0.5_f64 * 3.2).exp());
    assert!((result.absorbed - beer).abs() > 1.0);
    assert!(result.closure_residual.abs() < 1e-9);
    let direct_closure = 620.0
        - result.absorbed_direct
        - result.reflected_direct
        - (1.0 - 0.14) * result.terminal_from_direct;
    let diffuse_closure = 90.0
        - result.absorbed_diffuse
        - result.reflected_diffuse
        - (1.0 - 0.14) * result.terminal_from_diffuse;
    assert!(direct_closure.abs() < 1e-9);
    assert!(diffuse_closure.abs() < 1e-9);
}

#[test]
fn carbon_and_dry_material_remain_distinct() {
    let transfer = material_transfer(
        Tissue::Leaf,
        ReceiverClass::Metabolic,
        0.00432,
        0.000_100_285_714_285_714_27,
        0.48,
    )
    .expect("material");
    assert!((transfer.dry_matter() - 0.009).abs() < 1e-14);
    assert!((transfer.carbon() - transfer.dry_matter()).abs() > 1e-6);
}

fn cn_vector_parameters() -> CnParameters {
    CnParameters {
        growth_respiration_ratio: 0.11,
        a1_froot_leaf: 0.8,
        a2_croot_stem: 0.25,
        a3_stem_leaf: 0.35,
        a4_livewood_fraction: 0.2,
        current_growth_fraction: 0.6,
        cn_leaf: 30.0,
        cn_leaf_litter: 45.0,
        cn_froot: 45.0,
        cn_livewood: 55.0,
        cn_deadwood: 450.0,
        drymatter_carbon_fraction: 0.48,
        xs_recovery_days: 30.0,
        leaf_lifetime_s: 3.0 * 365.0 * 86_400.0,
        froot_lifetime_s: 2.0 * 365.0 * 86_400.0,
        livewood_turnover_s: 5.0 * 365.0 * 86_400.0,
        mortality_rate_s1: 0.01 / (365.0 * 86_400.0),
        leaf_litter_fractions: [0.2, 0.3, 0.5],
        froot_litter_fractions: [0.25, 0.35, 0.4],
    }
}

fn empty_tissues() -> std::collections::BTreeMap<Tissue, TissuePool> {
    [
        Tissue::Leaf,
        Tissue::FineRoot,
        Tissue::LiveStem,
        Tissue::DeadStem,
        Tissue::LiveCoarseRoot,
        Tissue::DeadCoarseRoot,
    ]
    .into_iter()
    .map(|tissue| (tissue, TissuePool::default()))
    .collect()
}

#[test]
fn six_tissue_allocation_and_phenology_match_oracle_vectors() {
    let expected = expected();
    let parameters = cn_vector_parameters();
    let mut tissues = empty_tissues();
    let offer = carbon_offer(0.018, 0.006, -0.030, 0.004, 86_400.0, 30.0).expect("carbon offer");
    let mut internal_n = 0.00007;
    let growth = finalize_growth(
        &mut tissues,
        &offer,
        &mut internal_n,
        0.000_274_527_112_063_062_8,
        &parameters,
    )
    .expect("growth finalization");
    assert!(
        (growth.tissue_carbon[0]
            - expected["carbon_nitrogen"]["leaf_growth"]
                .as_f64()
                .expect("leaf vector"))
        .abs()
            < 1e-14
    );
    assert!(
        (growth.growth_respiration
            - expected["carbon_nitrogen"]["growth_respiration"]
                .as_f64()
                .expect("respiration vector"))
        .abs()
            < 1e-14
    );

    let mut phenology_tissues = empty_tissues();
    phenology_tissues
        .get_mut(&Tissue::Leaf)
        .expect("leaf")
        .transfer = ElementPool {
        carbon: 0.012,
        nitrogen: 0.0004,
    };
    let first = advance_phenology(
        &mut phenology_tissues,
        PhenologyMode::SeasonalDeciduous,
        PhenologyPhase::Dormant,
        3.0 * 86_400.0,
        0.0,
        0.35,
        0.65,
        86_400.0,
        0.60,
        0.30,
        3.0 * 86_400.0,
        3.0 * 86_400.0,
        &parameters,
    )
    .expect("onset day one");
    assert_eq!(first.phase, PhenologyPhase::Onset);
    assert!(
        (phenology_tissues[&Tissue::Leaf].display.carbon
            - expected["phenology"]["onset_day_1_display"]
                .as_f64()
                .expect("onset vector"))
        .abs()
            < 1e-14
    );
    let equality = advance_phenology(
        &mut empty_tissues(),
        PhenologyMode::SeasonalDeciduous,
        PhenologyPhase::Dormant,
        0.0,
        0.0,
        0.35,
        0.60,
        86_400.0,
        0.60,
        0.30,
        3.0 * 86_400.0,
        3.0 * 86_400.0,
        &parameters,
    )
    .expect("threshold equality");
    assert_eq!(equality.phase, PhenologyPhase::Dormant);
}

#[test]
fn schema_and_migration_fail_closed_without_defaults() {
    assert!(VegetationConfiguration::parse_strict(br"{}").is_err());
    assert!(VegetationConfiguration::parse_strict(br#"{"unknown":1}"#).is_err());
    let mut mutated = VegetationConfiguration::parse_strict(
        &fs::read("tests/fixtures/c3_woody_v4_diagnostic_configuration.json")
            .expect("V4 configuration fixture"),
    )
    .expect("V4 configuration shape");
    mutated.strata[0].stem_rho_vis += 0.01;
    assert!(mutated.validate().is_err());
    let source = RhessysSource {
        source_path: "synthetic.epc".into(),
        raw_bytes: "leaf_cn 28".into(),
        fields: std::collections::BTreeMap::from([("leaf_cn".into(), serde_json::json!(28.0))]),
    };
    let report = migrate_definition_fields(
        &source,
        &std::collections::BTreeMap::new(),
        &["cn_leaf".into(), "p50_leaf_mm".into()],
        &std::collections::BTreeMap::from([("leaf_cn".into(), "cn_leaf".into())]),
    );
    assert_eq!(report.unresolved_required_fields, vec!["p50_leaf_mm"]);
    assert!(report.canonical_configuration_sha256.is_none());
}

#[test]
fn energy_and_aerodynamic_domains_are_explicit() {
    assert!(neutral_resistance(30.0, 12.0, 1.0, 0.2, 2.4).expect("neutral") > 0.0);
    assert!(neutral_resistance(30.0, 12.0, 1.0, 0.2, 0.0).is_err());
    assert!(saturation_specific_humidity(296.0, 101_325.0).expect("qsat") > 0.0);
    assert!(
        energy_residual(100.0, 50.0, 0.0, 50.0)
            .expect("closure")
            .abs()
            < f64::EPSILON
    );
}

#[test]
fn historical_states_cannot_enter_the_v4_public_state_parser() {
    let config = VegetationConfiguration::parse_strict(
        &fs::read("tests/fixtures/c3_woody_v4_diagnostic_configuration.json")
            .expect("V4 configuration fixture"),
    )
    .expect("V4 configuration shape");
    for path in [
        "tests/fixtures/c3_woody_v1_diagnostic_state.json",
        "tests/fixtures/c3_woody_v3_diagnostic_state.json",
    ] {
        let result = CoupledOwnedState::parse_strict(
            &fs::read(path).expect("historical state fixture"),
            &config,
        );
        assert!(result.is_err(), "historical state was accepted: {path}");
    }
    assert!(
        VegetationConfiguration::parse_strict(
            &fs::read("tests/fixtures/c3_woody_v3_diagnostic_configuration.json")
                .expect("V3 configuration fixture"),
        )
        .is_err()
    );
}
