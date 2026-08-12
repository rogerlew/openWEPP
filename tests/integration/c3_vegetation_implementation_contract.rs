use std::fs;

use openwepp_biogeochemistry::{BiogeochemistryState, MineralLayer};
use openwepp_hillslope_orchestrator::vegetation_diagnostic::{
    DiagnosticEnergyState, DiagnosticOwnedState, DiagnosticWaterState, run_default_off_diagnostic,
    run_default_off_diagnostic_at_phase,
};
use openwepp_kernel_contract::SoilLayerId;
use openwepp_vegetation::carbon_nitrogen::{
    AllocationInput, CnParameters, ElementPool, PhenologyMode, ReceiverClass, Tissue, TissuePool,
    advance_phenology, allocate, carbon_offer, finalize_growth, material_transfer,
};
use openwepp_vegetation::energy::{
    energy_residual, neutral_resistance, saturation_specific_humidity,
};
use openwepp_vegetation::interception::{InterceptionInput, liquid_interception};
use openwepp_vegetation::migration::{RhessysSource, migrate};
use openwepp_vegetation::photosynthesis::{FvcbInput, fvcb, medlyn};
use openwepp_vegetation::radiation::two_stream;
use openwepp_vegetation::{
    CoupledOwnedState, FailurePoint, MODEL_BYTES, MODEL_SHA256, PhenologyPhase, SnowFreeForcing,
    SoilLayerForcing, VegetationConfiguration, load_model_definition,
};
use sha2::{Digest, Sha256};

fn expected() -> serde_json::Value {
    serde_json::from_slice(
        &fs::read("tests/fixtures/c3_woody_v1_expected_vectors.json").expect("vector fixture"),
    )
    .expect("valid vector fixture")
}

#[test]
fn public_candidate_orchestrates_admitted_modules_without_proxy_formulas() {
    let source = fs::read_to_string("crates/openwepp-vegetation/src/transaction.rs")
        .expect("transaction source");
    assert!(source.contains("prepare_stratum"));
    assert!(source.contains("solve_coupled"));
    assert!(source.contains("finalize_growth"));
    assert!(!source.contains("ImplementationIncomplete"));
    assert!(!source.contains("ledger_residuals: [0.0; 5]"));
    assert!(!source.contains("vapor_pressure_deficit_kpa *"));
    assert!(!source.contains("direct_par_w_m2 * 1e-9"));
}

#[test]
fn production_registry_is_byte_identical_to_authority() {
    let authority = fs::read("docs/work-packages/20260811-coupled-c3-forest-vegetation-model-stack-authority-001/artifacts/openwepp_c3_woody_v1_definition.json")
        .expect("authority definition");
    assert_eq!(MODEL_BYTES, authority);
    assert_eq!(format!("{:x}", Sha256::digest(MODEL_BYTES)), MODEL_SHA256);
    load_model_definition().expect("digest-bound model");
}

#[test]
fn admitted_fvcb_and_medlyn_vectors_match_oracle() {
    let expected = expected();
    let zero = fvcb(FvcbInput {
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
    })
    .expect("zero light");
    assert!(
        (zero.an
            - expected["photosynthesis"]["zero_an"]
                .as_f64()
                .expect("zero vector"))
        .abs()
            < 1e-12
    );
    let saturated = fvcb(FvcbInput {
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
    })
    .expect("saturated light");
    let rubisco = fvcb(FvcbInput {
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
    })
    .expect("Rubisco limitation");
    let electron = fvcb(FvcbInput {
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
    })
    .expect("electron limitation");
    let gs = medlyn(saturated.an, 25.0, 3.5, 1.4, 39.0, 101_325.0, 1.0).expect("Medlyn");
    assert!(
        (saturated.an
            - expected["photosynthesis"]["saturated_an"]
                .as_f64()
                .expect("saturated vector"))
        .abs()
            < 1e-12
    );
    assert!(
        (gs - expected["photosynthesis"]["medlyn_gs"]
            .as_f64()
            .expect("Medlyn vector"))
        .abs()
            < 1e-8
    );
    assert!(
        (rubisco.an
            - expected["photosynthesis"]["rubisco_an"]
                .as_f64()
                .expect("Rubisco vector"))
        .abs()
            < 1e-12
    );
    assert!(
        (electron.an
            - expected["photosynthesis"]["electron_an"]
                .as_f64()
                .expect("electron vector"))
        .abs()
            < 1e-12
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
    assert!((result.drainage - 1.997_516_728_800_741).abs() < 1e-12);
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
fn resource_caps_and_cn_dry_material_remain_distinct() {
    let allocation = allocate(AllocationInput {
        carbon_offer: 0.02,
        nitrogen_available: 0.0001,
        a1: 0.7,
        a2: 0.3,
        a3: 1.2,
        growth_resp_ratio: 0.25,
        cn_leaf: 28.0,
        cn_froot: 35.0,
        cn_wood: 120.0,
    })
    .expect("allocation");
    assert!(allocation.eta < 1.0 && allocation.nsc_end > 0.0);
    let transfer = material_transfer(
        Tissue::Leaf,
        ReceiverClass::Metabolic,
        0.00432,
        0.000_100_285_714_285_714_27,
        0.48,
    )
    .expect("material");
    assert!((transfer.dry_matter - 0.009).abs() < 1e-14);
    assert_ne!(transfer.carbon, transfer.dry_matter);
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
        0.0002745271120630628,
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
    assert!(VegetationConfiguration::parse_strict(br#"{}"#).is_err());
    assert!(VegetationConfiguration::parse_strict(br#"{"unknown":1}"#).is_err());
    let mut mutated: VegetationConfiguration = serde_json::from_slice(
        &fs::read("tests/fixtures/c3_woody_v1_diagnostic_configuration.json")
            .expect("configuration fixture"),
    )
    .expect("configuration JSON");
    mutated.strata[0].stem_rho_vis += 0.01;
    assert!(mutated.validate().is_err());
    let source = RhessysSource {
        source_path: "synthetic.epc".into(),
        raw_bytes: "leaf_cn 28".into(),
        fields: std::collections::BTreeMap::from([("leaf_cn".into(), serde_json::json!(28.0))]),
    };
    let report = migrate(
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
fn public_transaction_commits_all_owners_and_rolls_back_on_injected_failure() {
    let config = VegetationConfiguration::parse_strict(
        &fs::read("tests/fixtures/c3_woody_v1_diagnostic_configuration.json")
            .expect("configuration fixture"),
    )
    .expect("valid configuration");
    let vegetation = CoupledOwnedState::parse_strict(
        &fs::read("tests/fixtures/c3_woody_v1_diagnostic_state.json").expect("state fixture"),
    )
    .expect("valid state");
    let layer = SoilLayerId::try_new("soil-1").expect("layer identity");
    let beginning = DiagnosticOwnedState {
        vegetation,
        water: DiagnosticWaterState {
            liquid_kg_m2: std::collections::BTreeMap::from([(layer.clone(), 100.0)]),
            last_transaction_id: 0,
        },
        biogeochemistry: BiogeochemistryState {
            layers: std::collections::BTreeMap::from([(
                "soil-1".into(),
                MineralLayer {
                    ammonium_n: 0.01,
                    nitrate_n: 0.02,
                },
            )]),
            receivers: std::collections::BTreeMap::from([
                (
                    openwepp_kernel_contract::MaterialReceiverClass::Metabolic,
                    Default::default(),
                ),
                (
                    openwepp_kernel_contract::MaterialReceiverClass::Cellulose,
                    Default::default(),
                ),
                (
                    openwepp_kernel_contract::MaterialReceiverClass::Lignin,
                    Default::default(),
                ),
                (
                    openwepp_kernel_contract::MaterialReceiverClass::CoarseWoodyDebris,
                    Default::default(),
                ),
            ]),
            ..BiogeochemistryState::default()
        },
        energy: DiagnosticEnergyState::default(),
    };
    let forcing = SnowFreeForcing {
        air_temperature_k: 296.0,
        pressure_pa: 101_325.0,
        co2_pa: 40.0,
        vapor_pressure_deficit_kpa: 1.4,
        wind_m_s: 2.4,
        rain_kg_m2: 0.1,
        direct_par_w_m2: 620.0,
        diffuse_par_w_m2: 90.0,
        direct_nir_w_m2: 500.0,
        diffuse_nir_w_m2: 80.0,
        solar_zenith_cosine: 0.68,
        ground_albedo_vis: 0.14,
        ground_albedo_nir: 0.25,
        longwave_down_w_m2: 400.0,
        longwave_up_w_m2: 420.0,
        specific_humidity: 0.010,
        reference_height_m: 30.0,
        soil_layers: vec![SoilLayerForcing {
            layer_id: layer,
            water_beginning_kg_m2: 100.0,
            matric_potential_mm: -5_000.0,
            hydraulic_conductivity_mm_s: 0.000_017,
            root_path_length_mm: 1.0,
            gravity_root_mm: 980.0,
            temperature_k: 294.0,
            accessible: true,
            frozen: false,
        }],
        gsi: 0.8,
    };
    let model = load_model_definition().expect("model");
    let available = beginning.water.liquid_kg_m2.clone();
    let beginning_bytes = serde_json::to_vec(&beginning).expect("serialize beginning owners");
    for phase in [
        FailurePoint::Validation,
        FailurePoint::Radiation,
        FailurePoint::Interception,
        FailurePoint::PotentialCoupledSolve,
        FailurePoint::WaterAuthorization,
        FailurePoint::CappedResolve,
        FailurePoint::NitrogenRequest,
        FailurePoint::NitrogenAuthorization,
        FailurePoint::Allocation,
        FailurePoint::ReceiverConstruction,
        FailurePoint::ClosureValidation,
        FailurePoint::BeforeCommit,
        FailurePoint::OwnerValidation,
    ] {
        let mut rollback = beginning.clone();
        assert!(
            run_default_off_diagnostic_at_phase(
                &mut rollback,
                &model,
                &config,
                &forcing,
                &available,
                Some(phase),
            )
            .is_err(),
            "phase {phase:?}"
        );
        assert_eq!(
            serde_json::to_vec(&rollback).expect("serialize rollback owners"),
            beginning_bytes,
            "phase {phase:?}"
        );
    }
    let mut committed = beginning;
    let receipt =
        run_default_off_diagnostic(&mut committed, &model, &config, &forcing, &available, false)
            .expect("coupled transaction");
    assert_eq!(receipt.transaction_id.0, 1);
    assert_eq!(committed.vegetation.last_transaction_id, 1);
    assert_eq!(committed.water.last_transaction_id, 1);
    assert_eq!(committed.biogeochemistry.last_transaction_id, 1);
    assert_eq!(committed.energy.last_transaction_id, 1);

    let mut partial_config = config.clone();
    partial_config.topology_tiles[0].fraction = 0.5;
    partial_config
        .topology_tiles
        .push(openwepp_vegetation::TopologyTile {
            tile_id: "tile-empty".into(),
            fraction: 0.5,
        });
    partial_config.configuration_sha256 = partial_config
        .canonical_sha256()
        .expect("partial config digest");
    let mut partial_owned = committed;
    partial_owned.vegetation.configuration_sha256 = partial_config.configuration_sha256.clone();
    partial_owned.vegetation.state_sha256 = partial_owned
        .vegetation
        .canonical_sha256()
        .expect("partial state digest");
    let partial_available = partial_owned.water.liquid_kg_m2.clone();
    let mut partial_forcing = forcing.clone();
    partial_forcing.soil_layers[0].water_beginning_kg_m2 = *partial_available
        .values()
        .next()
        .expect("diagnostic water layer");
    let partial_result = run_default_off_diagnostic(
        &mut partial_owned,
        &model,
        &partial_config,
        &partial_forcing,
        &partial_available,
        false,
    );
    assert!(partial_result.is_err());

    let mut empty_config = config;
    empty_config.strata.clear();
    empty_config.configuration_sha256 = empty_config
        .canonical_sha256()
        .expect("empty config digest");
    let mut empty_owned = partial_owned;
    empty_owned.vegetation.strata.clear();
    empty_owned.vegetation.configuration_sha256 = empty_config.configuration_sha256.clone();
    empty_owned.vegetation.state_sha256 = empty_owned
        .vegetation
        .canonical_sha256()
        .expect("empty state digest");
    let empty_available = empty_owned.water.liquid_kg_m2.clone();
    let mut empty_forcing = forcing;
    empty_forcing.soil_layers[0].water_beginning_kg_m2 = *empty_available
        .values()
        .next()
        .expect("diagnostic water layer");
    let empty_receipt = run_default_off_diagnostic(
        &mut empty_owned,
        &model,
        &empty_config,
        &empty_forcing,
        &empty_available,
        false,
    )
    .expect("empty-stand transaction");
    assert_eq!(empty_receipt.transaction_id.0, 2);
    assert!(empty_owned.vegetation.strata.is_empty());
}
