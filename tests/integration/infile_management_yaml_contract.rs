use std::fs;
use std::path::{Path, PathBuf};

use openwepp_hillslope_orchestrator::runtime_inputs::build_hillslope_pl_runtime_surfaces_from_management;
use openwepp_input_contract::parsers::management::{
    ParseMode, PlantScenarioData, YearlyAnnualExtension, YearlyCroplandBranch, YearlyScenarioData,
    parse_management_document_from_path,
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
    let phenology = plant
        .phenology
        .expect("native YAML must carry explicit forest phenology authority");
    assert_scalar_close(phenology.summer_foliar_biomass_kg_m2, 0.2);
    assert_scalar_close(phenology.evergreen_fraction, 0.2);

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
    for (root, expected) in [
        ("forest_phenology_model", 1.0),
        ("forest_summer_foliar_biomass_kg_m2", 0.2),
        ("forest_evergreen_fraction", 0.2),
        ("forest_structural_canopy_cover_fraction", 0.2),
        ("forest_structural_biomass_kg_m2", 0.1),
        ("forest_minimum_temperature_inactive_c", -2.0),
        ("forest_minimum_temperature_unconstrained_c", 5.0),
        ("forest_vapor_pressure_deficit_unconstrained_pa", 900.0),
        ("forest_vapor_pressure_deficit_inactive_pa", 4_100.0),
        ("forest_photoperiod_inactive_hours", 10.0),
        ("forest_photoperiod_unconstrained_hours", 11.0),
    ] {
        assert_scalar_close(
            scalar_at(
                &surfaces.pl_growth_surface,
                &BoundarySymbol::from(format!("pl_growth_slot_0001_crop_0001_{root}")),
            ),
            expected,
        );
    }
}

#[test]
fn native_forest_yaml_without_phenology_fails_closed() {
    let source = fs::read_to_string(fixture_path(
        "canonical_forest_nonzero_ow_lanuse_1.man.yaml",
    ))
    .expect("fixture should be readable");
    let start = source.find("    phenology:\n").expect("phenology block");
    let end = source[start..]
        .find("    cf: 5.0\n")
        .map(|offset| start + offset)
        .expect("field after phenology block");
    let without = format!("{}{}", &source[..start], &source[end..]);
    let path = std::env::temp_dir().join(format!(
        "openwepp-native-forest-missing-phenology-{}.man.yaml",
        std::process::id()
    ));
    fs::write(&path, without).expect("temporary YAML fixture should be writable");
    let result = parse_management_document_from_path(&path, ParseMode::Strict);
    fs::remove_file(&path).ok();
    assert!(
        result.is_err(),
        "missing phenology authority must fail closed"
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

#[test]
fn native_cropland_yaml_preserves_annual_residue_extensions() {
    let path = std::env::temp_dir().join(format!(
        "openwepp-management-yaml-annual-extensions-{}.man.yaml",
        std::process::id()
    ));
    fs::write(&path, NATIVE_CROPLAND_ANNUAL_EXTENSIONS_YAML)
        .expect("temporary YAML fixture should be writable");

    let management = parse_management_document_from_path(&path, ParseMode::Strict)
        .expect("native cropland YAML with annual extensions should parse");
    fs::remove_file(&path).ok();

    let extensions: Vec<_> = management
        .registries
        .yearlies
        .iter()
        .map(|yearly| {
            let YearlyScenarioData::Cropland(cropland) = &yearly.data else {
                panic!("expected native cropland yearly scenario");
            };
            let YearlyCroplandBranch::AnnualOrFallow(annual) = &cropland.branch else {
                panic!("expected annual/fallow branch");
            };
            annual.extension.clone()
        })
        .collect();

    assert_eq!(
        extensions,
        vec![
            Some(YearlyAnnualExtension::Herbicide { jdherb: 201 }),
            Some(YearlyAnnualExtension::Burn {
                jdburn: 202,
                fbmag: 0.3,
                fbrnog: 0.4,
            }),
            Some(YearlyAnnualExtension::Silage { jdslge: 203 }),
            Some(YearlyAnnualExtension::Cut {
                jdcut: 204,
                frcut: 0.5,
            }),
            Some(YearlyAnnualExtension::Remove {
                jdmove: 205,
                frmove: 0.6,
            }),
        ]
    );
    assert_eq!(management.registries.operations.len(), 1);
    assert_eq!(management.registries.surfaces.len(), 1);
    assert_eq!(management.registries.contours.len(), 1);
    assert_eq!(management.registries.drains.len(), 1);
}

const NATIVE_CROPLAND_ANNUAL_EXTENSIONS_YAML: &str = r"
format: openwepp-management-yaml
schema_version: 1
datver: ow-lanuse-1
topology:
  nofes: 1
  total_years: 1
metadata:
  name: Native_Cropland_Management
  description: [d1, d2, d3]
plants:
  - landuse: native_cropland
    name: Native_Cropland
    description: [d1, d2, d3]
    crunit: WeppWillSet
    canopy_line: [3.6, 3.0, 35.00196, 10.0, 2.3, 55.0, 0.0, 0.30404, 0.65, 0.051]
    growth_line: [0.85, 0.98, 0.65, 0.99, 0.0, 1700.0, 0.5, 2.60099]
    mfocod: 2
    residue_line: [0.0065, 0.0065, 25.0, 0.25, 0.219, 1.51995, 0.25, 0.0, 30.0, 0.0]
    terminal_line: [0.0, 3.5, 0.0]
    routing_coefficients:
      k_o: 650.0
      form_c_d: 0.8
      d_r_m: 0.03
      lambda: 0.1
      vegetation_c_d: 0.4
      authority:
        source: fixture
        version: 2026-07-09
        checksum: fixture
        disturbed_class: native_cropland
initial_conditions:
  - landuse: native_cropland
    name: Native_Initial
    description: [d1, d2, d3]
    base_line: [1.1, 0.0, 200.0, 92.0, 0.0, 0.9]
    iresd: 1
    imngmt: 1
    residue_line: [500.12601, 0.02, 0.9, 0.02, 0.0]
    rtyp: 1
    thaw_line: [0.0, 0.0, 0.1, 0.2, 0.0254]
    terminal_line: [0.50003, 0.19997]
operations:
  - landuse: native_cropland
    name: Native_Operation
    description: [d1, d2, d3]
    mfo1: 0.25
    mfo2: 0.15
    numof: 0
    pcode: 4
    effect_line: [0.025, 0.75, 0.25, 0.15, 0.012, 0.15, 0.0]
surface_effects:
  - landuse: native_cropland
    name: Native_Surface
    description: [d1, d2, d3]
    ntill: 1
    operations:
      - mdate: 130
        op_ref: 1
        tildep: 0.051
        typtil: 2
contours:
  - name: Native_Contour
    description: [d1, d2, d3]
    cntslp: 0.1
    rdghgt: 0.2
    rowlen: 30.0
    rowspc: 0.762
    contours_perm: 1
drains:
  - name: Native_Drain
    description: [d1, d2, d3]
    ddrain: 1.0
    drainc: 2.0
    drdiam: 3.0
    sdrain: 4.0
yearly_scenarios:
  - landuse: native_cropland
    name: Herbicide
    description: [d1, d2, d3]
    itype: 1
    tilseq: 1
    conset: 1
    drset: 1
    imngmt: 1
    branch:
      type: annual_or_fallow
      jdharv: 288
      jdplt: 130
      rw: 0.762
      resmgt: 1
      extension: { type: herbicide, jdherb: 201 }
  - landuse: native_cropland
    name: Burn
    description: [d1, d2, d3]
    itype: 1
    tilseq: 1
    conset: 1
    drset: 1
    imngmt: 1
    branch:
      type: annual_or_fallow
      jdharv: 288
      jdplt: 130
      rw: 0.762
      resmgt: 2
      extension: { type: burn, jdburn: 202, fbmag: 0.3, fbrnog: 0.4 }
  - landuse: native_cropland
    name: Silage
    description: [d1, d2, d3]
    itype: 1
    tilseq: 1
    conset: 1
    drset: 1
    imngmt: 1
    branch:
      type: annual_or_fallow
      jdharv: 288
      jdplt: 130
      rw: 0.762
      resmgt: 3
      extension: { type: silage, jdslge: 203 }
  - landuse: native_cropland
    name: Cut
    description: [d1, d2, d3]
    itype: 1
    tilseq: 1
    conset: 1
    drset: 1
    imngmt: 1
    branch:
      type: annual_or_fallow
      jdharv: 288
      jdplt: 130
      rw: 0.762
      resmgt: 4
      extension: { type: cut, jdcut: 204, frcut: 0.5 }
  - landuse: native_cropland
    name: Remove
    description: [d1, d2, d3]
    itype: 1
    tilseq: 1
    conset: 1
    drset: 1
    imngmt: 1
    branch:
      type: annual_or_fallow
      jdharv: 288
      jdplt: 130
      rw: 0.762
      resmgt: 5
      extension: { type: remove, jdmove: 205, frmove: 0.6 }
schedule:
  ofe_initial_refs:
    - 1
  rotation_repeats: 1
  rotation_years: 1
  slots:
    - rotation_index: 1
      year_in_rotation: 1
      ofe_index: 1
      yearly_refs: [1, 2, 3, 4, 5]
";
