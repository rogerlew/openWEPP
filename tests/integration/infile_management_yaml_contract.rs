use std::path::{Path, PathBuf};

use openwepp_hillslope_orchestrator::runtime_inputs::build_hillslope_pl_runtime_surfaces_from_management;
use openwepp_input_contract::parsers::management::{
    ParseMode, PlantScenarioData, parse_management_document_from_path,
};
use openwepp_kernel_contract::{BoundarySymbol, BoundaryValue};

fn fixture_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/infile/management")
        .join(name)
}

#[test]
fn canonical_management_yaml_projects_route_coefficients_to_pl_surfaces() {
    let management = parse_management_document_from_path(
        fixture_path("canonical_forest_nonzero_ow_lanuse_1.man.yaml"),
        ParseMode::Strict,
    )
    .expect("canonical management YAML should parse");

    assert_eq!(management.datver, "ow-lanuse-1");
    let PlantScenarioData::Forest(plant) = &management.registries.plants[0].data else {
        panic!("expected native forest plant");
    };
    assert!(
        plant.routing.is_some(),
        "YAML parser must carry typed route coefficients into ManagementParseOutput"
    );

    let surfaces = build_hillslope_pl_runtime_surfaces_from_management(&management)
        .expect("YAML-derived management should project to PL surfaces");

    assert_scalar_close(
        scalar_at(
            &surfaces.pl_schedule_surface,
            &BoundarySymbol::from("ofe1_route_skin_friction_coefficient_ko"),
        ),
        500.0,
    );
    assert_scalar_close(
        scalar_at(
            &surfaces.pl_schedule_surface,
            &BoundarySymbol::from("ofe1_route_form_drag_coefficient"),
        ),
        1.25,
    );
    assert_scalar_close(
        scalar_at(
            &surfaces.pl_schedule_surface,
            &BoundarySymbol::from("ofe1_route_roughness_element_height_m"),
        ),
        0.06,
    );
    assert_scalar_close(
        scalar_at(
            &surfaces.pl_schedule_surface,
            &BoundarySymbol::from("ofe1_route_roughness_concentration"),
        ),
        0.2,
    );
    assert_scalar_close(
        scalar_at(
            &surfaces.pl_schedule_surface,
            &BoundarySymbol::from("ofe1_route_vegetation_drag_coefficient"),
        ),
        0.7,
    );
    assert_scalar_close(
        scalar_at(
            &surfaces.pl_schedule_surface,
            &BoundarySymbol::from(
                "pl_schedule_slot_0001_crop_0001_route_skin_friction_coefficient_ko",
            ),
        ),
        500.0,
    );
}

fn scalar_at(
    surface: &std::collections::BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &BoundarySymbol,
) -> f64 {
    surface
        .get(symbol)
        .unwrap_or_else(|| panic!("missing symbol {symbol}"))
        .as_f64()
}

fn assert_scalar_close(observed: f64, expected: f64) {
    assert!(
        (observed - expected).abs() <= 1.0e-12,
        "observed {observed}, expected {expected}"
    );
}
