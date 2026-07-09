use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use openwepp_hillslope_orchestrator::runtime_inputs::build_hillslope_pl_runtime_surfaces_from_management;
use openwepp_input_contract::parsers::management::{
    ParseMode, parse_management_document_from_path,
};
use openwepp_kernel_contract::{BoundarySymbol, BoundaryValue};
use openwepp_landuse_migrate::{
    ClassMap, MigrationAuthority, MigrationRequest, MigrationTarget, ReportFormat, migrate_path,
};

fn fixture_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/infile/management")
        .join(name)
}

fn temp_dir(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be valid")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!(
        "openwepp-landuse-migration-contract-{label}-{}-{nanos}",
        std::process::id()
    ));
    fs::create_dir_all(&dir).expect("temp dir should be created");
    dir
}

#[test]
fn migrated_legacy_cropland_yaml_is_consumed_by_real_runtime_projection() {
    let dir = temp_dir("runtime");
    let input = dir.join("field.man");
    fs::copy(fixture_path("canonical_cropland_nonzero_98_4.man"), &input)
        .expect("fixture should copy");

    let migration = migrate_path(&MigrationRequest {
        input: input.clone(),
        target: MigrationTarget::OwLanuse1,
        output: None,
        authority: MigrationAuthority {
            disturbed_class: Some("agriculture crops".to_string()),
            disturbed_class_map: ClassMap::default(),
        },
        dry_run: false,
        report: None,
        report_format: ReportFormat::Text,
    })
    .expect("migration should succeed");

    let output_path = migration
        .output_path
        .expect("non-dry migration should return output path");
    assert_eq!(output_path, dir.join("field.man.yaml"));

    let management = parse_management_document_from_path(&output_path, ParseMode::Strict)
        .expect("runtime management parser should read migrated YAML");
    let surfaces = build_hillslope_pl_runtime_surfaces_from_management(&management)
        .expect("runtime PL projection should consume migrated YAML");

    assert_scalar_close(
        scalar_at(
            &surfaces.pl_schedule_surface,
            &BoundarySymbol::from("ofe1_route_skin_friction_coefficient_ko"),
        ),
        480.0,
    );
    assert_scalar_close(
        scalar_at(
            &surfaces.pl_schedule_surface,
            &BoundarySymbol::from("ofe1_route_form_drag_coefficient"),
        ),
        0.25,
    );
    assert_scalar_close(
        scalar_at(
            &surfaces.pl_schedule_surface,
            &BoundarySymbol::from("ofe1_route_roughness_element_height_m"),
        ),
        0.01,
    );
    assert_scalar_close(
        scalar_at(
            &surfaces.pl_schedule_surface,
            &BoundarySymbol::from("ofe1_route_roughness_concentration"),
        ),
        0.05,
    );
    assert_scalar_close(
        scalar_at(
            &surfaces.pl_schedule_surface,
            &BoundarySymbol::from("ofe1_route_vegetation_drag_coefficient"),
        ),
        0.12,
    );
    assert_scalar_close(
        scalar_at(
            &surfaces.pl_schedule_surface,
            &BoundarySymbol::from("pl_schedule_slot_0001_crop_0001_route_roughness_concentration"),
        ),
        0.05,
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
