use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use openwepp_hillslope_orchestrator::{
    HillslopeWritebackSurface, Wb11HydrologyKernel,
    runtime_inputs::build_hillslope_runtime_surface_from_soil,
};
use openwepp_input_contract::parsers::soil::{ParserMode, SoilParserOptions, parse_soil};
use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, HillslopeConsumerAdapter, HillslopeKernel,
    HillslopeKernelPhaseClass, HillslopeKernelRequest,
};
use serde::Deserialize;

const TOL: f64 = 1.0e-9;
const LEGACY_INPUT_DELTA_EPS_M: f64 = 0.001;

const VALID_9002: &str = include_str!("../fixtures/infile/soil/valid_9002.sol");
const VALID_7778: &str = include_str!("../fixtures/infile/soil/valid_7778.sol");

#[derive(Debug, Deserialize)]
struct RelaxExpectedRequired {
    pei_m: Option<f64>,
    d_m: Option<f64>,
    pei_positive: bool,
    d_non_negative: bool,
}

#[derive(Debug, Deserialize)]
struct RelaxFixtureRequired {
    case_id: String,
    theta_m: Option<f64>,
    fc_m: f64,
    ul_m: f64,
    ssc_m_s: f64,
    expected: RelaxExpectedRequired,
}

#[derive(Clone, Copy, Debug)]
struct IndependentLegacySeed {
    depth_mm: f64,
    bulk_density_g_cm3: f64,
    fc_measured: f64,
    wp_measured: f64,
    sand_pct: f64,
    clay_pct: f64,
    orgmat_pct: f64,
    cec_meq_100g: f64,
    rock_frag_pct: f64,
    fc_wp_rock_multiplier_policy: IndependentFcWpRockMultiplierPolicy,
}

#[derive(Clone, Copy, Debug)]
struct IndependentLegacyLayerExpanded {
    thickness_m: f64,
    bulk_density_kg_m3: f64,
    thetfc: f64,
    thetdr: f64,
    sand: f64,
    clay: f64,
    orgmat: f64,
    cec: f64,
    rfg: f64,
    fc_wp_rock_multiplier_policy: IndependentFcWpRockMultiplierPolicy,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum IndependentFcWpRockMultiplierPolicy {
    ApplyToMeasuredFcWp,
}

#[derive(Clone, Copy, Debug)]
struct IndependentLegacyCorrectedLayer {
    thickness_m: f64,
    thetfc: f64,
    thetdr: f64,
}

#[derive(Debug)]
struct IndependentFcWpAuthority {
    mapped_layers: Vec<(f64, f64)>,
    profile_fc_store_mm: f64,
    profile_wp_store_mm: f64,
}

fn repo_file(path: &str) -> String {
    let repo_root = env!("CARGO_MANIFEST_DIR");
    let full_path = Path::new(repo_root).join(path);
    fs::read_to_string(&full_path)
        .unwrap_or_else(|error| panic!("expected readable file {}: {error}", full_path.display()))
}

fn repo_json_fixture<T: for<'de> Deserialize<'de>>(path: &str) -> T {
    let text = repo_file(path);
    serde_json::from_str::<T>(&text)
        .unwrap_or_else(|error| panic!("failed to parse fixture {path} as JSON: {error}"))
}

#[test]
fn auth05_suite_metadata_removes_legacy_as_constitutive_authority() {
    let suite_fc =
        repo_file("docs/specifications/external-authority/suites/cas_l4_soil_fc_minus33_001.md");
    let suite_wp =
        repo_file("docs/specifications/external-authority/suites/cas_l4_soil_wp_minus1500_001.md");
    let suite_relax =
        repo_file("docs/specifications/external-authority/suites/cas_l4_watbal_relax_to_fc_001.md");
    let registry = repo_file("docs/specifications/external-authority/registry.yaml");

    assert!(
        !suite_fc.contains("EXT-SOIL-FC-LEGACY-001")
            && !suite_wp.contains("EXT-SOIL-WP-LEGACY-001")
            && !suite_relax.contains("EXT-WATBAL-PERC-LEGACY-001"),
        "AUTH05 suite docs must not retain legacy baseline citations as Level-4 constitutive authority"
    );
    assert!(
        suite_fc.contains("gate_lane: required")
            && suite_wp.contains("gate_lane: required")
            && suite_relax.contains("gate_lane: required")
            && suite_fc.contains("failure_class: hard-fail")
            && suite_wp.contains("failure_class: hard-fail")
            && suite_relax.contains("failure_class: hard-fail"),
        "AUTH05 must preserve required/hard-fail lane semantics for AUTH03 Level-4 suites"
    );
    assert!(
        registry.contains(
            "tests/integration/auth05_level4_constitutive_authority_hardening_contract.rs"
        ),
        "registry integration_test paths must route AUTH03 Level-4 suites through AUTH05 hardened gate target"
    );
}

#[test]
fn auth05_fc_wp_runtime_surface_matches_independent_authority_on_real_soils() {
    for (fixture_id, soil_text) in [("valid_9002", VALID_9002), ("valid_7778", VALID_7778)] {
        let soil = parse_soil(soil_text, strict_soil_parser_options())
            .unwrap_or_else(|error| panic!("{fixture_id} should parse: {error}"));
        let surface = build_hillslope_runtime_surface_from_soil(&soil)
            .unwrap_or_else(|error| panic!("{fixture_id} runtime surface should build: {error}"));
        let authority = independent_authority_from_soil(&soil, fixture_id);
        compare_surface_to_independent_authority(&surface, &authority).unwrap_or_else(|detail| {
            panic!("{fixture_id} runtime FC/WP surfaces must match independent authority: {detail}")
        });
    }
}

#[test]
fn auth05_fc_wp_authority_comparator_rejects_underdrained_perturbation() {
    let soil =
        parse_soil(VALID_9002, strict_soil_parser_options()).expect("9002 fixture should parse");
    let mut surface =
        build_hillslope_runtime_surface_from_soil(&soil).expect("runtime surface should build");
    let authority = independent_authority_from_soil(&soil, "valid_9002");
    compare_surface_to_independent_authority(&surface, &authority)
        .expect("baseline surface should satisfy independent authority");

    let baseline_thetfc = state_scalar(&surface, "thetfc_0001");
    surface.state_surface.insert(
        BoundarySymbol::from("thetfc_0001"),
        BoundaryValue::scalar(baseline_thetfc * 0.1),
    );
    let err = compare_surface_to_independent_authority(&surface, &authority)
        .expect_err("perturbed underdrained theta_fc surface must fail authority comparator");
    assert!(
        err.contains("thetfc_0001"),
        "expected mismatch detail to include the perturbed symbol; observed: {err}"
    );
}

#[test]
fn auth05_relax_to_fc_requires_explicit_positive_branch_assertions() {
    let near_fc: RelaxFixtureRequired = repo_json_fixture(
        "tests/fixtures/constitutive/cas_l4_watbal_relax_to_fc_001/near_fc_cutoff.json",
    );
    let above_fc: RelaxFixtureRequired = repo_json_fixture(
        "tests/fixtures/constitutive/cas_l4_watbal_relax_to_fc_001/above_fc_positive.json",
    );
    assert_eq!(near_fc.case_id, "relax_near_fc_cutoff");
    assert_eq!(above_fc.case_id, "relax_above_fc_positive");

    let mut kernel = Wb11HydrologyKernel;

    let near_response = kernel.run_hillslope_phase(&build_perc_request(&seeded_perc_state(
        near_fc.theta_m,
        near_fc.fc_m,
        near_fc.ul_m,
        near_fc.ssc_m_s,
    )));
    assert_eq!(
        near_response.status.message_id(),
        "HKERNEL-WB11-PERC-OK-001"
    );
    let near_pei = writeback_flux_value(&near_response, "wb18_perc_pei_0001");
    let near_d = writeback_flux_value(&near_response, "D");
    if let Some(expected_pei) = near_fc.expected.pei_m {
        assert!(
            (near_pei - expected_pei).abs() <= 1.0e-12,
            "near-FC pei mismatch: expected={expected_pei}, observed={near_pei}"
        );
    }
    if let Some(expected_d) = near_fc.expected.d_m {
        assert!(
            (near_d - expected_d).abs() <= 1.0e-12,
            "near-FC D mismatch: expected={expected_d}, observed={near_d}"
        );
    }
    assert_eq!(
        near_pei > 0.0,
        near_fc.expected.pei_positive,
        "near-FC pei positivity assertion must be explicit and enforced"
    );
    assert_eq!(
        near_d >= 0.0,
        near_fc.expected.d_non_negative,
        "near-FC D non-negative assertion must be explicit and enforced"
    );

    let above_response = kernel.run_hillslope_phase(&build_perc_request(&seeded_perc_state(
        above_fc.theta_m,
        above_fc.fc_m,
        above_fc.ul_m,
        above_fc.ssc_m_s,
    )));
    assert_eq!(
        above_response.status.message_id(),
        "HKERNEL-WB11-PERC-OK-001"
    );
    let above_pei = writeback_flux_value(&above_response, "wb18_perc_pei_0001");
    let above_d = writeback_flux_value(&above_response, "D");
    assert_eq!(
        above_pei > 0.0,
        above_fc.expected.pei_positive,
        "above-FC pei positivity assertion must be explicit and enforced"
    );
    assert_eq!(
        above_d >= 0.0,
        above_fc.expected.d_non_negative,
        "above-FC D non-negative assertion must be explicit and enforced"
    );
}

fn strict_soil_parser_options() -> SoilParserOptions {
    SoilParserOptions {
        mode: ParserMode::Strict,
        allow_legacy_aliases: false,
        expected_topology_count: None,
        topology_scope: None,
    }
}

fn independent_fc_wp_rock_multiplier_policy(
    _ofe: &openwepp_input_contract::parsers::soil::SoilOfe,
) -> IndependentFcWpRockMultiplierPolicy {
    // AUTH12 promotes measured-theta FC/WP ingest authority with paired runtime
    // cpm correction for disturbed-policy datvers (9002/9003/9005). The
    // independent comparator must mirror that closure posture.
    IndependentFcWpRockMultiplierPolicy::ApplyToMeasuredFcWp
}

fn independent_authority_from_soil(
    soil: &openwepp_input_contract::parsers::soil::SoilProfile,
    fixture_id: &str,
) -> IndependentFcWpAuthority {
    let ofe = soil
        .ofes
        .first()
        .unwrap_or_else(|| panic!("{fixture_id}: expected at least one OFE"));
    let fc_wp_policy = independent_fc_wp_rock_multiplier_policy(ofe);

    let mut seeds = Vec::with_capacity(ofe.layers.len());
    for (layer_position, layer) in ofe.layers.iter().enumerate() {
        let layer_index = layer_position + 1;
        let seed = IndependentLegacySeed {
            depth_mm: layer.depth_mm,
            bulk_density_g_cm3: layer.bulk_density_g_cm3.unwrap_or_else(|| {
                panic!("{fixture_id}: layer {layer_index} missing bulk_density_g_cm3")
            }),
            fc_measured: layer
                .fc_measured
                .unwrap_or_else(|| panic!("{fixture_id}: layer {layer_index} missing fc_measured")),
            wp_measured: layer
                .wp_measured
                .unwrap_or_else(|| panic!("{fixture_id}: layer {layer_index} missing wp_measured")),
            sand_pct: layer.sand_pct,
            clay_pct: layer.clay_pct,
            orgmat_pct: layer.orgmat_pct,
            cec_meq_100g: layer.cec_meq_100g,
            rock_frag_pct: layer.rock_frag_pct,
            fc_wp_rock_multiplier_policy: fc_wp_policy,
        };
        assert!(
            independent_seed_is_finite(seed),
            "{fixture_id}: layer {layer_index} contains non-finite normalization inputs"
        );
        seeds.push(seed);
    }
    let expanded = independent_expand_soil_layers_to_200mm(&seeds)
        .unwrap_or_else(|| panic!("{fixture_id}: normalized layer expansion failed"));
    let corrected = expanded
        .into_iter()
        .map(independent_correct_layer_moisture)
        .collect::<Option<Vec<_>>>()
        .unwrap_or_else(|| panic!("{fixture_id}: independent moisture correction failed"));

    let profile_fc_store_mm = corrected
        .iter()
        .map(|layer| layer.thetfc * layer.thickness_m * 1_000.0)
        .sum::<f64>();
    let profile_wp_store_mm = corrected
        .iter()
        .map(|layer| layer.thetdr * layer.thickness_m * 1_000.0)
        .sum::<f64>();

    let normalized_intervals = corrected_normalized_intervals(&corrected);
    let mapped_layers =
        map_corrected_layers_to_parser_layers(ofe, &normalized_intervals, fixture_id);

    IndependentFcWpAuthority {
        mapped_layers,
        profile_fc_store_mm,
        profile_wp_store_mm,
    }
}

type CorrectedNormalizedInterval = (f64, f64, f64, f64);

fn corrected_normalized_intervals(
    corrected_layers: &[IndependentLegacyCorrectedLayer],
) -> Vec<CorrectedNormalizedInterval> {
    let mut normalized_intervals = Vec::with_capacity(corrected_layers.len());
    let mut top_mm = 0.0_f64;
    for layer in corrected_layers {
        let bottom_mm = top_mm + (layer.thickness_m * 1_000.0);
        normalized_intervals.push((top_mm, bottom_mm, layer.thetfc, layer.thetdr));
        top_mm = bottom_mm;
    }
    normalized_intervals
}

fn map_corrected_layers_to_parser_layers(
    ofe: &openwepp_input_contract::parsers::soil::SoilOfe,
    normalized_intervals: &[CorrectedNormalizedInterval],
    fixture_id: &str,
) -> Vec<(f64, f64)> {
    let mut mapped = Vec::with_capacity(ofe.layers.len());
    let mut parser_top_mm = 0.0_f64;
    for (layer_position, layer) in ofe.layers.iter().enumerate() {
        let layer_index = layer_position + 1;
        let parser_bottom_mm = layer.depth_mm;
        let parser_thickness_mm = parser_bottom_mm - parser_top_mm;
        assert!(
            parser_thickness_mm > 0.0,
            "{fixture_id}: parser layer {layer_index} has non-positive thickness"
        );

        let mut weighted_thetfc = 0.0_f64;
        let mut weighted_thetdr = 0.0_f64;
        let mut covered_mm = 0.0_f64;
        for (normalized_top_mm, normalized_bottom_mm, thetfc, thetdr) in normalized_intervals {
            let overlap_top_mm = parser_top_mm.max(*normalized_top_mm);
            let overlap_bottom_mm = parser_bottom_mm.min(*normalized_bottom_mm);
            let overlap_mm = (overlap_bottom_mm - overlap_top_mm).max(0.0);
            if overlap_mm <= 0.0 {
                continue;
            }
            weighted_thetfc += *thetfc * overlap_mm;
            weighted_thetdr += *thetdr * overlap_mm;
            covered_mm += overlap_mm;
        }
        assert!(
            (covered_mm - parser_thickness_mm).abs() <= 1.0e-9,
            "{fixture_id}: parser layer {layer_index} overlap mapping incomplete (covered {covered_mm} of {parser_thickness_mm})"
        );
        mapped.push((weighted_thetfc / covered_mm, weighted_thetdr / covered_mm));
        parser_top_mm = parser_bottom_mm;
    }
    mapped
}

fn compare_surface_to_independent_authority(
    surface: &HillslopeWritebackSurface,
    authority: &IndependentFcWpAuthority,
) -> Result<(), String> {
    for (layer_position, (expected_thetfc, expected_thetdr)) in
        authority.mapped_layers.iter().enumerate()
    {
        let layer_index = layer_position + 1;
        let observed_thetfc = state_scalar(surface, &format!("thetfc_{layer_index:04}"));
        let observed_thetdr = state_scalar(surface, &format!("thetdr_{layer_index:04}"));
        if (observed_thetfc - expected_thetfc).abs() > TOL {
            return Err(format!(
                "thetfc_{layer_index:04} mismatch: observed={observed_thetfc}, expected={expected_thetfc}"
            ));
        }
        if (observed_thetdr - expected_thetdr).abs() > TOL {
            return Err(format!(
                "thetdr_{layer_index:04} mismatch: observed={observed_thetdr}, expected={expected_thetdr}"
            ));
        }
        if observed_thetdr > observed_thetfc {
            return Err(format!(
                "layer ordering violation at {layer_index}: thetdr={observed_thetdr} > thetfc={observed_thetfc}"
            ));
        }
    }

    let observed_profile_fc_store_mm = state_scalar(surface, "wb13_profile_fc_store_mm");
    let observed_profile_wp_store_mm = state_scalar(surface, "wb13_profile_wp_store_mm");
    if (observed_profile_fc_store_mm - authority.profile_fc_store_mm).abs() > TOL {
        return Err(format!(
            "wb13_profile_fc_store_mm mismatch: observed={observed_profile_fc_store_mm}, expected={}",
            authority.profile_fc_store_mm
        ));
    }
    if (observed_profile_wp_store_mm - authority.profile_wp_store_mm).abs() > TOL {
        return Err(format!(
            "wb13_profile_wp_store_mm mismatch: observed={observed_profile_wp_store_mm}, expected={}",
            authority.profile_wp_store_mm
        ));
    }
    Ok(())
}

fn state_scalar(surface: &HillslopeWritebackSurface, symbol: &str) -> f64 {
    surface
        .state_surface
        .get(&BoundarySymbol::from(symbol))
        .unwrap_or_else(|| panic!("missing state symbol {symbol}"))
        .as_f64()
}

fn independent_seed_is_finite(seed: IndependentLegacySeed) -> bool {
    seed.depth_mm.is_finite()
        && seed.bulk_density_g_cm3.is_finite()
        && seed.fc_measured.is_finite()
        && seed.wp_measured.is_finite()
        && seed.sand_pct.is_finite()
        && seed.clay_pct.is_finite()
        && seed.orgmat_pct.is_finite()
        && seed.cec_meq_100g.is_finite()
        && seed.rock_frag_pct.is_finite()
}

fn independent_expand_soil_layers_to_200mm(
    seeds: &[IndependentLegacySeed],
) -> Option<Vec<IndependentLegacyLayerExpanded>> {
    let cumulative_depths_mm = independent_cumulative_depths_mm(seeds)?;
    let source_layers = independent_source_layers_from_seed_depths(seeds, &cumulative_depths_mm)?;
    let total_depth_m = source_layers
        .iter()
        .map(|layer| layer.thickness_m)
        .sum::<f64>();
    let normalized_layer_count = independent_normalized_layer_count(total_depth_m)?;
    Some(independent_normalize_layers_to_200mm(
        &source_layers,
        normalized_layer_count,
    ))
}

fn independent_cumulative_depths_mm(seeds: &[IndependentLegacySeed]) -> Option<Vec<f64>> {
    if seeds.is_empty() {
        return None;
    }
    let mut cumulative_depths_mm = seeds.iter().map(|seed| seed.depth_mm).collect::<Vec<f64>>();
    let total_thickness_mm = *cumulative_depths_mm.last()?;
    if total_thickness_mm < 200.0 {
        let deficit = 200.0 - total_thickness_mm;
        if let Some(last_depth) = cumulative_depths_mm.last_mut() {
            *last_depth += deficit;
        }
    }
    if let Some(last_depth) = cumulative_depths_mm.last_mut() {
        *last_depth += 200.0;
    }
    for depth in &mut cumulative_depths_mm {
        if *depth > 1800.0 {
            *depth = 1800.0;
        }
    }
    Some(cumulative_depths_mm)
}

fn independent_source_layers_from_seed_depths(
    seeds: &[IndependentLegacySeed],
    cumulative_depths_mm: &[f64],
) -> Option<Vec<IndependentLegacyLayerExpanded>> {
    let mut source_layers = Vec::with_capacity(seeds.len());
    let mut previous_depth_m = 0.0_f64;
    for (seed, depth_mm) in seeds.iter().zip(cumulative_depths_mm.iter().copied()) {
        let mut bulk_density = seed.bulk_density_g_cm3;
        if bulk_density > 0.0 && bulk_density < 0.8 {
            bulk_density = 0.8;
        }
        if bulk_density > 2.0 {
            bulk_density = 2.0;
        }
        let mut orgmat_pct = seed.orgmat_pct;
        if orgmat_pct > 10.0 {
            orgmat_pct = 10.0;
        }
        let mut rock_frag_pct = seed.rock_frag_pct;
        if rock_frag_pct > 85.0 {
            rock_frag_pct = 85.0;
        }
        let depth_m = depth_mm * 0.001;
        let thickness_m = depth_m - previous_depth_m;
        previous_depth_m = depth_m;
        if thickness_m <= 0.0 {
            continue;
        }
        source_layers.push(IndependentLegacyLayerExpanded {
            thickness_m,
            bulk_density_kg_m3: bulk_density * 1000.0,
            thetfc: seed.fc_measured,
            thetdr: seed.wp_measured,
            sand: seed.sand_pct / 100.0,
            clay: seed.clay_pct / 100.0,
            orgmat: orgmat_pct / 100.0,
            cec: seed.cec_meq_100g,
            rfg: rock_frag_pct / 100.0,
            fc_wp_rock_multiplier_policy: seed.fc_wp_rock_multiplier_policy,
        });
    }
    if source_layers.is_empty() {
        return None;
    }
    Some(source_layers)
}

fn independent_normalized_layer_count(total_depth_m: f64) -> Option<usize> {
    let rounded_total_mm = (total_depth_m * 1000.0).round();
    if !rounded_total_mm.is_finite() || rounded_total_mm <= 0.0 {
        return None;
    }
    let mut remaining_mm = rounded_total_mm;
    let mut normalized_layer_count = 0usize;
    while remaining_mm >= 200.0 {
        normalized_layer_count = normalized_layer_count.checked_add(1)?;
        remaining_mm -= 200.0;
    }
    if normalized_layer_count == 0 {
        return None;
    }
    Some(normalized_layer_count)
}

fn independent_normalize_layers_to_200mm(
    source_layers: &[IndependentLegacyLayerExpanded],
    normalized_layer_count: usize,
) -> Vec<IndependentLegacyLayerExpanded> {
    let mut remaining_thicknesses = source_layers
        .iter()
        .map(|layer| layer.thickness_m)
        .collect::<Vec<f64>>();
    let mut source_index = 0usize;
    let mut normalized_layers = Vec::with_capacity(normalized_layer_count);
    for _ in 0..normalized_layer_count {
        let mut layer = IndependentLegacyLayerExpanded {
            thickness_m: 0.2,
            bulk_density_kg_m3: 0.0,
            thetfc: 0.0,
            thetdr: 0.0,
            sand: 0.0,
            clay: 0.0,
            orgmat: 0.0,
            cec: 0.0,
            rfg: 0.0,
            fc_wp_rock_multiplier_policy: source_layers.first().map_or(
                IndependentFcWpRockMultiplierPolicy::ApplyToMeasuredFcWp,
                |source| source.fc_wp_rock_multiplier_policy,
            ),
        };
        let mut remaining = 0.2;
        while remaining > 0.0 && source_index < source_layers.len() {
            let source_thickness = remaining_thicknesses[source_index];
            if source_thickness <= 0.0 {
                source_index += 1;
                continue;
            }
            if source_thickness <= remaining {
                let fraction = source_thickness / 0.2;
                independent_accumulate_weighted_layer(
                    &mut layer,
                    source_layers[source_index],
                    fraction,
                );
                remaining -= source_thickness;
                source_index += 1;
                if remaining.abs() <= LEGACY_INPUT_DELTA_EPS_M {
                    remaining = 0.0;
                }
            } else {
                let fraction = remaining / 0.2;
                independent_accumulate_weighted_layer(
                    &mut layer,
                    source_layers[source_index],
                    fraction,
                );
                remaining_thicknesses[source_index] -= remaining;
                if remaining_thicknesses[source_index].abs() <= LEGACY_INPUT_DELTA_EPS_M {
                    source_index += 1;
                }
                remaining = 0.0;
            }
        }
        normalized_layers.push(layer);
    }
    normalized_layers
}

fn independent_accumulate_weighted_layer(
    target: &mut IndependentLegacyLayerExpanded,
    source: IndependentLegacyLayerExpanded,
    weight: f64,
) {
    target.bulk_density_kg_m3 += source.bulk_density_kg_m3 * weight;
    target.thetfc += source.thetfc * weight;
    target.thetdr += source.thetdr * weight;
    target.sand += source.sand * weight;
    target.clay += source.clay * weight;
    target.orgmat += source.orgmat * weight;
    target.cec += source.cec * weight;
    target.rfg += source.rfg * weight;
}

fn independent_correct_layer_moisture(
    layer: IndependentLegacyLayerExpanded,
) -> Option<IndependentLegacyCorrectedLayer> {
    let dg = layer.thickness_m;
    if !dg.is_finite() || dg <= 0.0 {
        return None;
    }
    let solcon = independent_solcon(layer.clay, layer.cec, layer.orgmat, dg);
    let oca = 3.80 + 1.9 * layer.clay.powi(2) - (3.365 * layer.sand)
        + (12.6 * solcon * layer.clay)
        + (100.0 * layer.orgmat * (layer.sand / 2.0).powi(2));
    let coca = 1.0 - (oca / 100.0);
    if !coca.is_finite() || coca <= 0.0 || coca > 1.0 {
        return None;
    }
    let cpm = 1.0
        - ((layer.rfg * layer.bulk_density_kg_m3)
            / ((layer.rfg * layer.bulk_density_kg_m3) + 2650.0 * (1.0 - layer.rfg)));
    if !cpm.is_finite() || cpm <= 0.0 || cpm > 1.0 {
        return None;
    }
    let mut por = (2650.0 - layer.bulk_density_kg_m3) / 2650.0;
    por *= coca;
    if !por.is_finite() || por <= 0.0 {
        return None;
    }
    let apply_fc_wp_rock_multiplier = matches!(
        layer.fc_wp_rock_multiplier_policy,
        IndependentFcWpRockMultiplierPolicy::ApplyToMeasuredFcWp
    );
    let mut thetfc = layer.thetfc.max(0.0);
    let mut thetdr = layer.thetdr.max(0.0);
    if apply_fc_wp_rock_multiplier {
        thetfc *= cpm;
        thetdr *= cpm;
    }
    if !thetfc.is_finite() || !thetdr.is_finite() {
        return None;
    }
    if thetfc <= 0.0 || thetdr <= 0.0 {
        return None;
    }
    let log10 = 10_f64.ln();
    let t33 = 333.3_f64.ln() / log10;
    let t15 = 15_300.0_f64.ln() / log10;
    let s33 = thetfc.ln() / log10;
    let s15 = thetdr.ln() / log10;
    let slope = ((s15 - s33) / (t15 - t33)).abs();
    if !slope.is_finite() {
        return None;
    }
    let mut sm20c = 10.0_f64.powf(slope * (t15 - (10.0_f64.ln() / log10)) + s15);
    if apply_fc_wp_rock_multiplier {
        sm20c *= cpm;
    }
    if sm20c >= por {
        sm20c = por * 0.95;
    }
    if thetfc >= sm20c {
        let delta = thetfc - thetdr;
        thetfc = sm20c * 0.99;
        thetdr = (thetfc - delta).max(0.01);
        thetfc = thetfc.max(0.01);
    }
    if (thetfc / por) > 0.83 {
        let scale = thetfc / (por * 0.83);
        thetfc = por * 0.83;
        thetdr /= scale;
    }
    thetdr = thetdr.max(0.01);
    thetfc = thetfc.max(0.01);
    if !thetfc.is_finite() || !thetdr.is_finite() {
        return None;
    }
    if thetdr > thetfc || thetfc > por {
        return None;
    }
    Some(IndependentLegacyCorrectedLayer {
        thickness_m: dg,
        thetfc,
        thetdr,
    })
}

fn independent_solcon(clay: f64, cec: f64, orgmat: f64, dg: f64) -> f64 {
    if clay <= 0.0 {
        return 0.0;
    }
    let cecc = cec - orgmat * (142.0 + 170.0 * dg);
    (cecc / (100.0 * clay)).clamp(0.15, 0.65)
}

fn seeded_perc_state(
    theta_m: Option<f64>,
    fc_m: f64,
    ul_m: f64,
    ssc_m_s: f64,
) -> BTreeMap<BoundarySymbol, BoundaryValue> {
    let mut state = BTreeMap::new();
    state.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(1.0));
    if let Some(theta) = theta_m {
        state.insert(
            BoundarySymbol::from("wb18_perc_theta_0001"),
            BoundaryValue::scalar(theta),
        );
        state.insert(
            BoundarySymbol::from("wb11_soil_water"),
            BoundaryValue::scalar(theta),
        );
    } else {
        state.insert(
            BoundarySymbol::from("wb11_soil_water"),
            BoundaryValue::scalar(fc_m),
        );
    }
    state.insert(
        BoundarySymbol::from("wb18_perc_fc_0001"),
        BoundaryValue::scalar(fc_m),
    );
    state.insert(
        BoundarySymbol::from("wb18_perc_ul_0001"),
        BoundaryValue::scalar(ul_m),
    );
    state.insert(
        BoundarySymbol::from("wb18_perc_ssc_0001"),
        BoundaryValue::scalar(ssc_m_s),
    );
    state.insert(
        BoundarySymbol::from("thetdr_0001"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(BoundarySymbol::from("dg_0001"), BoundaryValue::scalar(1.0));
    state.insert(
        BoundarySymbol::from("wb11_field_capacity"),
        BoundaryValue::scalar(fc_m),
    );
    state.insert(
        BoundarySymbol::from("wb11_perc_fraction"),
        BoundaryValue::scalar(1.0),
    );
    state
}

fn build_perc_request(
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
) -> HillslopeKernelRequest<'_> {
    let flux_surface = Box::leak(Box::new(BTreeMap::new()));
    HillslopeKernelRequest::with_transition_context(
        "percolation_deep_seepage",
        HillslopeKernelPhaseClass::HydrologyPercolationDeepSeepage,
        HillslopeConsumerAdapter::Perc,
        None,
        None,
        state_surface,
        flux_surface,
    )
}

fn writeback_flux_value(
    response: &openwepp_kernel_contract::KernelRunResponse,
    symbol: &str,
) -> f64 {
    response
        .writeback
        .flux_updates
        .iter()
        .find(|field| field.symbol == BoundarySymbol::from(symbol))
        .unwrap_or_else(|| panic!("missing flux writeback symbol {symbol}"))
        .value
        .as_f64()
}
