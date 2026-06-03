use openwepp_hillslope_orchestrator::runtime_inputs::build_hillslope_runtime_surface_from_soil;
use openwepp_input_contract::parsers::soil::{
    ParserMode, SoilParserOptions, TopologyScope, parse_soil,
};
use openwepp_kernel_contract::BoundarySymbol;

const EPS: f64 = 1.0e-9;
const VALID_9002: &str = include_str!("../fixtures/infile/soil/valid_9002.sol");
const ASYMMETRIC_MOFE_9002: &str = "\
9002
Disturbed multi-OFE soil profile
3 1
SOIL_A SILT_LOAM 2 0.20 0.55 900000 0.005 4.2 10.5
1 forest silt_loam 0.20 0.001
100 1.25 15.0 1.20 0.30 0.15 35 25 2.0 15 5 0.05 0.45 0.02 1.40 120 0.16 0.31
250 1.30 8.0 1.10 0.28 0.14 33 27 1.8 14 7 0.06 0.43 0.03 1.35 110 0.15 0.30
SOIL_B CLAY_LOAM 2 0.20 0.55 900000 0.005 4.2 10.5
1 forest clay_loam 0.20 0.001
150 1.35 5.0 1.00 0.42 0.22 20 40 1.5 18 3 0.08 0.52 0.04 1.28 90 0.21 0.39
450 1.40 2.0 0.90 0.40 0.21 18 44 1.2 19 4 0.09 0.50 0.05 1.25 80 0.20 0.38
SOIL_C SANDY_LOAM 3 0.20 0.55 900000 0.005 4.2 10.5
1 forest sandy_loam 0.20 0.001
100 1.20 25.0 1.30 0.24 0.10 55 15 1.0 10 2 0.03 0.38 0.01 1.45 150 0.12 0.25
300 1.25 18.0 1.20 0.23 0.09 58 14 0.9 9 2 0.03 0.37 0.01 1.42 140 0.11 0.24
700 1.30 12.0 1.10 0.22 0.08 60 12 0.8 8 1 0.02 0.36 0.01 1.40 130 0.10 0.23
1 500 0.8
";

fn strict_soil_parser_options() -> SoilParserOptions {
    SoilParserOptions {
        mode: ParserMode::Strict,
        allow_legacy_aliases: false,
        expected_topology_count: Some(1),
        topology_scope: Some(TopologyScope::Hillslope),
    }
}

fn mofe_soil_parser_options(expected_topology_count: usize) -> SoilParserOptions {
    SoilParserOptions {
        mode: ParserMode::Strict,
        allow_legacy_aliases: false,
        expected_topology_count: Some(expected_topology_count),
        topology_scope: Some(TopologyScope::Hillslope),
    }
}

fn scalar(
    surface: &openwepp_hillslope_orchestrator::HillslopeWritebackSurface,
    symbol: &str,
) -> f64 {
    surface
        .state_surface
        .get(&BoundarySymbol::from(symbol))
        .unwrap_or_else(|| panic!("{symbol} should be present"))
        .as_f64()
}

fn rounded_usize(value: f64, symbol: &str) -> usize {
    assert!(value.is_finite(), "{symbol} must be finite");
    assert!(
        (value - value.round()).abs() <= EPS,
        "{symbol} must be integral, observed {value}"
    );
    format!("{:.0}", value.round())
        .parse::<usize>()
        .unwrap_or_else(|error| panic!("{symbol} must parse as usize from {value}: {error}"))
}

#[test]
fn hphys0255_contract_authority_sections_exist() {
    let watbal =
        include_str!("../../docs/specifications/science-contracts/contracts/SC-WATBAL-001.md");
    let soil = include_str!("../../docs/specifications/science-contracts/contracts/SC-SOIL-001.md");
    let system =
        include_str!("../../docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md");

    assert!(
        watbal.contains("INV-WATBAL-042")
            && watbal.contains("HPHYS0255 MOFE Storage Projection Addendum"),
        "SC-WATBAL-001 must carry HPHYS0255 MOFE storage projection authority"
    );
    assert!(
        soil.contains("INV-SOIL-016") && soil.contains("HPHYS0255 MOFE Storage-Scope Addendum"),
        "SC-SOIL-001 must carry HPHYS0255 MOFE storage-scope authority"
    );
    assert!(
        system.contains("INV-SYSTEM-029")
            && system.contains("storage_lineage_policy")
            && system.contains("single-runtime-wb11-state"),
        "SC-SYSTEM-001 must carry HPHYS0255 storage-lineage provenance authority"
    );
}

#[test]
fn hphys0255_mofe_seed_projection_separates_scoped_ofe_soil_from_active_wb11_state() {
    let soil = parse_soil(ASYMMETRIC_MOFE_9002, mofe_soil_parser_options(3))
        .expect("asymmetric three-OFE soil fixture should parse");
    assert_eq!(soil.ofes.len(), 3, "fixture must exercise MOFE input");

    let surface = build_hillslope_runtime_surface_from_soil(&soil)
        .expect("runtime surface should build from asymmetric MOFE soil");

    assert_eq!(rounded_usize(scalar(&surface, "ntemp"), "ntemp"), 3);
    assert_eq!(rounded_usize(scalar(&surface, "ofe1_nsl"), "ofe1_nsl"), 2);
    assert_eq!(rounded_usize(scalar(&surface, "ofe2_nsl"), "ofe2_nsl"), 2);
    assert_eq!(rounded_usize(scalar(&surface, "ofe3_nsl"), "ofe3_nsl"), 3);

    let profile_depth_mm = scalar(&surface, "wb13_profile_depth_mm");
    let wb11_nsl = rounded_usize(scalar(&surface, "wb11_nsl"), "wb11_nsl");
    let expected_wb11_nsl = rounded_usize(profile_depth_mm / 200.0, "profile_depth_mm / 200");
    assert_eq!(
        wb11_nsl, expected_wb11_nsl,
        "active WB11 hydrology state must use normalized primary storage grid"
    );

    let mut wb11_depth_sum_mm = 0.0_f64;
    for layer_index in 1..=wb11_nsl {
        wb11_depth_sum_mm += scalar(&surface, &format!("wb19_dg_{layer_index:04}")) * 1_000.0;
    }
    assert!(
        (wb11_depth_sum_mm - profile_depth_mm).abs() <= EPS,
        "active WB11 hydrology aliases must reconcile to WB13 profile depth"
    );

    assert!(
        (scalar(&surface, "ofe2_solthk") * 1_000.0 - 450.0).abs() <= EPS,
        "OFE2 parser/corrected-layer provenance must remain scoped"
    );
    assert!(
        (scalar(&surface, "ofe3_solthk") * 1_000.0 - 700.0).abs() <= EPS,
        "OFE3 parser/corrected-layer provenance must remain scoped"
    );
    assert!(
        (scalar(&surface, "ofe2_dg_0002") - 0.300).abs() <= EPS,
        "OFE2 layer thickness must remain independent of normalized WB11 aliases"
    );
    assert!(
        (scalar(&surface, "wb19_dg_0002") - 0.200).abs() <= EPS,
        "WB11 hydrology aliases must not be overwritten by later OFE rows"
    );
    assert!(
        !surface
            .state_surface
            .contains_key(&BoundarySymbol::from("ofe2_wb19_dg_0001")),
        "OFE-qualified parser diagnostics must not masquerade as dynamic per-OFE WB19 state"
    );
}

#[test]
fn hphys0254_contract_authority_sections_exist() {
    let watbal =
        include_str!("../../docs/specifications/science-contracts/contracts/SC-WATBAL-001.md");
    let soil = include_str!("../../docs/specifications/science-contracts/contracts/SC-SOIL-001.md");

    assert!(
        watbal.contains("INV-WATBAL-041")
            && watbal.contains("HPHYS0254 WB11 Initial-Storage Projection Addendum"),
        "SC-WATBAL-001 must carry HPHYS0254 WB11 initial-storage projection authority"
    );
    assert!(
        soil.contains("INV-SOIL-015")
            && soil.contains("HPHYS0254 WB11 Normalized Seed-Grid Addendum"),
        "SC-SOIL-001 must carry HPHYS0254 normalized seed-grid authority"
    );
}

#[test]
fn hphys0254_primary_wb11_seed_grid_spans_normalized_profile_depth() {
    let soil = parse_soil(VALID_9002, strict_soil_parser_options())
        .expect("9002 soil fixture should parse");
    let parser_profile_depth_mm = soil
        .ofes
        .first()
        .and_then(|ofe| ofe.layers.last())
        .expect("fixture must include a primary OFE layer")
        .depth_mm;

    let surface = build_hillslope_runtime_surface_from_soil(&soil)
        .expect("runtime surface should build from parsed soil");

    let profile_depth_mm = scalar(&surface, "wb13_profile_depth_mm");
    assert!(
        profile_depth_mm > parser_profile_depth_mm,
        "fixture must expose normalized-tail depth for the contract vector"
    );

    let nsl = rounded_usize(scalar(&surface, "wb11_nsl"), "wb11_nsl");
    let expected_nsl = rounded_usize(profile_depth_mm / 200.0, "profile_depth_mm / 200");
    assert_eq!(
        nsl, expected_nsl,
        "primary WB11 nsl must follow normalized 200 mm profile grid"
    );

    let mut depth_sum_mm = 0.0_f64;
    let mut seeded_soil_water_mm = 0.0_f64;
    let sat = scalar(&surface, "sat");
    assert!((0.0..=1.0).contains(&sat), "sat must be bounded");

    for layer_index in 1..=nsl {
        let dg = scalar(&surface, &format!("wb19_dg_{layer_index:04}"));
        let solthk = scalar(&surface, &format!("wb19_solthk_{layer_index:04}"));
        let por = scalar(&surface, &format!("wb19_por_{layer_index:04}"));
        let cpm = scalar(&surface, &format!("cpm_{layer_index:04}"));
        let coca = scalar(&surface, &format!("wb19_coca_{layer_index:04}"));
        let thetfc = scalar(&surface, &format!("wb19_thetfc_{layer_index:04}"));
        let thetdr = scalar(&surface, &format!("wb19_thetdr_{layer_index:04}"));
        let ssc = scalar(&surface, &format!("ssc_{layer_index:04}"));

        assert!(
            (dg - 0.2).abs() <= EPS,
            "dg_{layer_index:04} must be 200 mm"
        );
        depth_sum_mm += dg * 1000.0;
        assert!(
            (solthk * 1000.0 - depth_sum_mm).abs() <= EPS,
            "solthk_{layer_index:04} must match normalized cumulative depth"
        );
        assert!(
            por.is_finite() && por >= thetfc && thetfc >= thetdr && thetdr >= 0.0,
            "layer {layer_index} must satisfy por >= thetfc >= thetdr >= 0"
        );
        assert!(cpm.is_finite() && cpm > 0.0 && cpm <= 1.0);
        assert!(coca.is_finite() && coca > 0.0 && coca <= 1.0);
        assert!(ssc.is_finite() && ssc > 0.0);

        seeded_soil_water_mm += sat * por * cpm * dg * 1000.0;
    }

    assert!(
        (depth_sum_mm - profile_depth_mm).abs() <= EPS,
        "primary WB11 layer depth sum must reconcile to wb13_profile_depth_mm"
    );
    assert!(
        (scalar(&surface, "solthk") * 1000.0 - profile_depth_mm).abs() <= EPS,
        "primary solthk must expose normalized profile depth"
    );
    assert!(
        seeded_soil_water_mm.is_finite() && seeded_soil_water_mm > 0.0,
        "baseline st/soilw seed equivalent must be finite and positive"
    );
}
