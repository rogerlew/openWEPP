use openwepp_hillslope_orchestrator::runtime_inputs::build_hillslope_runtime_surface_from_soil;
use openwepp_input_contract::parsers::soil::{
    ParserMode, SoilParserOptions, TopologyScope, parse_soil,
};
use openwepp_kernel_contract::BoundarySymbol;

const EPS: f64 = 1.0e-9;
const VALID_9002: &str = include_str!("../fixtures/infile/soil/valid_9002.sol");

fn strict_soil_parser_options() -> SoilParserOptions {
    SoilParserOptions {
        mode: ParserMode::Strict,
        allow_legacy_aliases: false,
        expected_topology_count: Some(1),
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
