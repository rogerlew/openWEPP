use std::fs;

use openwepp_vegetation::carbon_nitrogen::{
    AllocationInput, ReceiverClass, Tissue, allocate, material_transfer,
};
use openwepp_vegetation::energy::{
    energy_residual, neutral_resistance, saturation_specific_humidity,
};
use openwepp_vegetation::hydraulics::finalize_under_caps;
use openwepp_vegetation::interception::{InterceptionInput, liquid_interception};
use openwepp_vegetation::migration::{RhessysSource, migrate};
use openwepp_vegetation::photosynthesis::{FvcbInput, fvcb, medlyn};
use openwepp_vegetation::radiation::two_stream;
use openwepp_vegetation::{
    MODEL_BYTES, MODEL_SHA256, VegetationConfiguration, load_model_definition,
};
use sha2::{Digest, Sha256};

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
    assert!((zero.an + 1.2).abs() < 1e-12);
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
    let gs = medlyn(saturated.an, 25.0, 3.5, 1.4, 39.0, 101_325.0, 1.0).expect("Medlyn");
    assert!(gs > 25.0);
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
    let result =
        two_stream(3.2, 0.68, 0.1, 0.08, 0.05, 0.14, 620.0, 90.0).expect("two-stream radiation");
    assert!((result.absorbed - 631.455_094_216_157_8).abs() < 1e-7);
    let beer = 710.0 * (1.0_f64 - (-0.5_f64 * 3.2).exp());
    assert!((result.absorbed - beer).abs() > 1.0);
    assert!(result.closure_residual.abs() < 1e-9);
}

#[test]
fn resource_caps_and_cn_dry_material_remain_distinct() {
    assert_eq!(
        finalize_under_caps(&[2.0, 3.0], &[1.5, 0.5]).expect("caps"),
        vec![1.5, 0.5]
    );
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

#[test]
fn schema_and_migration_fail_closed_without_defaults() {
    assert!(VegetationConfiguration::parse_strict(br#"{}"#).is_err());
    assert!(VegetationConfiguration::parse_strict(br#"{"unknown":1}"#).is_err());
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
