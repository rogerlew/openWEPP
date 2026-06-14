use std::collections::BTreeSet;

use openwepp_hillslope_output::hillslope_wat::{InterchangeVersion, hillslope_wat_schema};
use openwepp_sim_contract::units::{
    BoundaryUnitEntry, BoundaryUnitRegistry, BoundaryUnitRegistryError, DimensionClass,
    DomainClass, OutputUnitAuthority, OutputUnitEntry, OutputUnitRegistry, OutputUnitRegistryError,
    TypedBoundaryRequirement, hphys0274_required_boundary_aliases,
};
use openwepp_watershed_output::writers::watershed_interchange_schemas;

#[test]
fn canonical_registry_contains_hydrology_et_percolation_publication_units() {
    let registry = BoundaryUnitRegistry::canonical_registry()
        .expect("canonical unit registry should construct");

    for (alias, unit, dimension) in [
        ("hillslope_wat.P", "mm", DimensionClass::Depth),
        ("hillslope_wat.RM", "mm", DimensionClass::Depth),
        ("hillslope_wat.Q", "mm", DimensionClass::Depth),
        ("hillslope_wat.Ep", "mm", DimensionClass::Depth),
        ("hillslope_wat.Es", "mm", DimensionClass::Depth),
        ("hillslope_wat.Er", "mm", DimensionClass::Depth),
        ("hillslope_wat.Dp", "mm", DimensionClass::Depth),
        ("hillslope_wat.latqcc", "mm", DimensionClass::Depth),
        ("hillslope_wat.Total-Soil", "mm", DimensionClass::Depth),
        ("hillslope_wat.frdp", "mm", DimensionClass::Depth),
        ("hillslope_wat.SoilWaterTotal", "mm", DimensionClass::Depth),
        ("hillslope_wat.Snow-Water", "mm", DimensionClass::Depth),
        ("hillslope_wat.Interception", "mm", DimensionClass::Depth),
        ("hillslope_wat.Area", "m^2", DimensionClass::Area),
    ] {
        let entry = registry
            .entry_for_boundary_alias(alias)
            .expect("publication alias should resolve");
        assert_eq!(entry.unit_label(), unit, "{alias} unit mismatch");
        assert_eq!(
            entry.dimension_class(),
            dimension,
            "{alias} dimension mismatch"
        );
    }

    for alias in ["wind", "hs21_wind"] {
        let entry = registry
            .entry_for_boundary_alias(alias)
            .unwrap_or_else(|_| panic!("direction alias {alias} should resolve"));
        assert_eq!(
            entry.domain_class(),
            DomainClass::DirectionDegrees,
            "{alias} direction domain"
        );
    }
}

#[test]
fn canonical_registry_keeps_prcp_meters_distinct_from_p_publication_mm() {
    let registry = BoundaryUnitRegistry::canonical_registry()
        .expect("canonical unit registry should construct");

    let prcp = registry
        .entry_for_boundary_alias("prcp")
        .expect("runtime prcp alias should resolve");
    assert_eq!(prcp.canonical_symbol(), "prcp");
    assert_eq!(prcp.unit_label(), "m");

    let publication = registry
        .entry_for_boundary_alias("hillslope_wat.P")
        .expect("publication P alias should resolve");
    assert_eq!(publication.canonical_symbol(), "P");
    assert_eq!(publication.unit_label(), "mm");
}

#[test]
fn hphys0290_registry_declares_post_winter_rain_flux_metadata() {
    let registry = BoundaryUnitRegistry::canonical_registry()
        .expect("canonical unit registry should construct");

    let entry = registry
        .entry_for_boundary_alias("snow.post_winter_rain_m")
        .expect("post-winter rain alias should resolve");
    assert_eq!(entry.canonical_symbol(), "snow_post_winter_rain");
    assert_eq!(entry.unit_label(), "m");
    assert_eq!(entry.dimension_class(), DimensionClass::Depth);
    assert_eq!(entry.domain_class(), DomainClass::NonNegativeFinite);
    assert_eq!(
        entry.typed_boundary(),
        TypedBoundaryRequirement::TypedRequired
    );
    assert_eq!(entry.contract_id(), "SC-SNOWFREEZE-001");
    assert_eq!(entry.invariant_id(), "SC-SNOWFREEZE-001#INV-SNOWFREEZE-023");
}

#[test]
#[allow(clippy::too_many_lines)]
fn canonical_registry_resolves_climate_soil_and_snow_runtime_aliases() {
    let registry = BoundaryUnitRegistry::canonical_registry()
        .expect("canonical unit registry should construct");

    for (alias, unit, dimension) in [
        ("rad", "Ly d^-1", DimensionClass::RadiationDaily),
        ("stmdur", "s", DimensionClass::Time),
        ("stmstr", "h", DimensionClass::Time),
        ("timem_0001", "s", DimensionClass::Time),
        ("wind", "deg", DimensionClass::Direction),
        ("vwind", "m s^-1", DimensionClass::WindSpeed),
        ("hs21_prcp", "m", DimensionClass::Depth),
        ("hs21_rad", "Ly d^-1", DimensionClass::RadiationDaily),
        ("hs21_tmax", "degC", DimensionClass::Temperature),
        ("hs21_tmin", "degC", DimensionClass::Temperature),
        ("hs21_tdpt", "degC", DimensionClass::Temperature),
        ("hs21_vwind", "m s^-1", DimensionClass::WindSpeed),
        ("hs21_wind", "deg", DimensionClass::Direction),
        ("hs21_stmdur", "s", DimensionClass::Time),
        ("hs21_stmstr", "h", DimensionClass::Time),
        ("hs21_timem_0001", "s", DimensionClass::Time),
        ("hs21_mxint", "m s^-1", DimensionClass::Rate),
        ("hs21_avrint", "m s^-1", DimensionClass::Rate),
        ("hs21_intsty_0001", "m s^-1", DimensionClass::Rate),
        (
            "winter.hourly.rad_mj_m2_0001",
            "MJ m^-2 h^-1",
            DimensionClass::RadiationHourly,
        ),
        (
            "winter.hourly.air_temp_c_0024",
            "degC",
            DimensionClass::Temperature,
        ),
        (
            "winter.hourly.wind_m_s_0007",
            "m s^-1",
            DimensionClass::WindSpeed,
        ),
        (
            "winter.hourly.cloud_fraction_0012",
            "dimensionless",
            DimensionClass::Fraction,
        ),
        ("snow.runtime_swe", "m", DimensionClass::Depth),
        ("snow.post_winter_rain_m", "m", DimensionClass::Depth),
        ("snow.runtime_depth_m", "m", DimensionClass::Depth),
        (
            "snow.runtime_density_kg_m3",
            "kg m^-3",
            DimensionClass::Density,
        ),
        ("snow.hourly.rain_m_0011", "m", DimensionClass::Depth),
        (
            "snow.hourly.rain_retained_m_0011",
            "m",
            DimensionClass::Depth,
        ),
        ("snow.hourly.stmtim.rain_m_0011", "m", DimensionClass::Depth),
        (
            "snow.hourly.stmtim.stmdur_s_0011",
            "s",
            DimensionClass::Time,
        ),
        (
            "snow.hourly.stmtim.wntdur_h_0011",
            "h",
            DimensionClass::Time,
        ),
        (
            "snow.hourly.stmtim.wnttim_h_0011",
            "h",
            DimensionClass::Time,
        ),
        (
            "snow.hourly.stmtim.hrtemp_c_0011",
            "degC",
            DimensionClass::Temperature,
        ),
        (
            "snow.hourly.stmtim.rst_c_0011",
            "degC",
            DimensionClass::Temperature,
        ),
        (
            "snow.hourly.stmtim.hrrain_m_0011",
            "m",
            DimensionClass::Depth,
        ),
        (
            "snow.hourly.stmtim.hrsnow_m_0011",
            "m",
            DimensionClass::Depth,
        ),
        (
            "snow.hourly.stmtim.active_interval_0011",
            "dimensionless",
            DimensionClass::Fraction,
        ),
        (
            "snow.hourly.stmtim.rain_branch_0011",
            "dimensionless",
            DimensionClass::Fraction,
        ),
        (
            "snow.hourly.stmtim.snow_branch_0011",
            "dimensionless",
            DimensionClass::Fraction,
        ),
        (
            "snow.hourly.depth_before_m_0011",
            "m",
            DimensionClass::Depth,
        ),
        (
            "snow.hourly.depth_available_m_0011",
            "m",
            DimensionClass::Depth,
        ),
        ("snow.hourly.depth_after_m_0011", "m", DimensionClass::Depth),
        (
            "snow.hourly.density_before_kg_m3_0011",
            "kg m^-3",
            DimensionClass::Density,
        ),
        (
            "snow.hourly.density_after_kg_m3_0011",
            "kg m^-3",
            DimensionClass::Density,
        ),
        ("snow.hourly.melt_m_0011", "m", DimensionClass::Depth),
        ("snow.hourly.melt_raw_m_0011", "m", DimensionClass::Depth),
        (
            "snow.hourly.melt_branch_active_0011",
            "dimensionless",
            DimensionClass::Fraction,
        ),
        (
            "snow.hourly.melt_amelt_in_0011",
            "in",
            DimensionClass::Depth,
        ),
        ("dg_0001", "m", DimensionClass::Depth),
        ("ofe2_dg_0001", "m", DimensionClass::Depth),
        (
            "thetdr_0001",
            "m^3 m^-3",
            DimensionClass::VolumetricWaterContent,
        ),
        (
            "ofe3_thetfc_0002",
            "m^3 m^-3",
            DimensionClass::VolumetricWaterContent,
        ),
        ("ssc_0001", "m s^-1", DimensionClass::HydraulicConductivity),
        ("wb13_profile_depth_mm", "mm", DimensionClass::Depth),
        ("wb13_profile_porosity_cap_mm", "mm", DimensionClass::Depth),
        ("wb13_profile_fc_store_mm", "mm", DimensionClass::Depth),
        ("wb13_profile_fc_tail_mm", "mm", DimensionClass::Depth),
        ("wb13_profile_wp_store_mm", "mm", DimensionClass::Depth),
    ] {
        let entry = registry
            .entry_for_boundary_alias(alias)
            .expect("runtime alias should resolve");
        assert_eq!(entry.unit_label(), unit, "{alias} unit mismatch");
        assert_eq!(
            entry.dimension_class(),
            dimension,
            "{alias} dimension mismatch"
        );
    }
}

#[test]
fn hphys0275_registry_marks_only_migrated_aliases_typed_required() {
    let registry = BoundaryUnitRegistry::canonical_registry()
        .expect("canonical unit registry should construct");

    for alias in [
        "prcp",
        "rad",
        "tmax",
        "tmin",
        "tdpt",
        "vwind",
        "stmdur",
        "stmstr",
        "timem_0001",
        "mxint",
        "avrint",
        "intsty_0001",
        "winter.hourly.rad_mj_m2_0001",
        "winter.hourly.air_temp_c_0001",
        "winter.hourly.dewpoint_c_0001",
        "winter.hourly.wind_m_s_0001",
        "winter.hourly.cloud_fraction_0001",
        "snow.runtime_swe",
        "snow.post_winter_rain_m",
        "snow.runtime_depth_m",
        "snow.runtime_density_kg_m3",
        "snow.hourly.rain_m_0001",
        "snow.hourly.rain_retained_m_0001",
        "snow.hourly.snowfall_m_0001",
        "snow.hourly.stmtim.rain_m_0001",
        "snow.hourly.stmtim.stmdur_s_0001",
        "snow.hourly.stmtim.wntdur_h_0001",
        "snow.hourly.stmtim.wnttim_h_0001",
        "snow.hourly.stmtim.hrtemp_c_0001",
        "snow.hourly.stmtim.rst_c_0001",
        "snow.hourly.stmtim.hrrain_m_0001",
        "snow.hourly.stmtim.hrsnow_m_0001",
        "snow.hourly.stmtim.active_interval_0001",
        "snow.hourly.stmtim.rain_branch_0001",
        "snow.hourly.stmtim.snow_branch_0001",
        "wind",
        "hs21_prcp",
        "hs21_rad",
        "hs21_tmax",
        "hs21_tmin",
        "hs21_tdpt",
        "hs21_vwind",
        "hs21_wind",
        "hs21_stmdur",
        "hs21_stmstr",
        "hs21_timem_0001",
        "hs21_mxint",
        "hs21_avrint",
        "hs21_intsty_0001",
        "snow.hourly.depth_before_m_0001",
        "snow.hourly.depth_available_m_0001",
        "snow.hourly.depth_after_m_0001",
        "snow.hourly.density_before_kg_m3_0001",
        "snow.hourly.density_after_kg_m3_0001",
        "snow.hourly.melt_m_0001",
        "snow.hourly.melt_branch_active_0001",
    ] {
        let entry = registry
            .entry_for_boundary_alias(alias)
            .unwrap_or_else(|_| panic!("migrated alias {alias} should resolve"));
        assert_eq!(
            entry.typed_boundary(),
            TypedBoundaryRequirement::TypedRequired,
            "{alias} typed posture"
        );
    }

    for alias in [
        "snow.hourly.melt_raw_m_0001",
        "snow.hourly.melt_amelt_in_0001",
        "snow.hourly.melt_bmelt_in_0001",
        "snow.hourly.melt_cmelt_in_0001",
        "snow.hourly.melt_dmelt_in_0001",
    ] {
        let entry = registry
            .entry_for_boundary_alias(alias)
            .unwrap_or_else(|_| panic!("follow-up alias {alias} should resolve"));
        assert_eq!(
            entry.typed_boundary(),
            TypedBoundaryRequirement::FollowUpRequired,
            "{alias} typed posture"
        );
    }
}

#[test]
fn canonical_registry_gate_rejects_missing_required_aliases() {
    let registry = BoundaryUnitRegistry::canonical_registry()
        .expect("canonical unit registry should construct");

    registry
        .require_boundary_aliases(hphys0274_required_boundary_aliases().iter().copied())
        .expect("registered required aliases should pass");

    assert_eq!(
        registry
            .require_boundary_aliases(["unknown.depth_m"])
            .expect_err("missing required alias should be rejected"),
        BoundaryUnitRegistryError::RequiredBoundaryAliasMissing {
            boundary_alias: "unknown.depth_m".to_string(),
        }
    );
}

#[test]
fn registry_units_cover_hillslope_wat_schema_metadata() {
    let registry = BoundaryUnitRegistry::canonical_registry()
        .expect("canonical unit registry should construct");
    let schema = hillslope_wat_schema(InterchangeVersion::default())
        .expect("hillslope WAT schema should construct");

    for field in schema.fields() {
        let Some(unit) = field.metadata().get("units") else {
            continue;
        };
        let alias = format!("hillslope_wat.{}", field.name());
        let entry = registry
            .entry_for_boundary_alias(alias.as_str())
            .expect("WAT field with unit metadata should resolve in registry");
        assert_eq!(
            entry.unit_label(),
            unit,
            "{} schema unit must match registry",
            field.name()
        );
    }
}

#[test]
fn hphys0278_output_unit_registry_covers_output_schema_unit_metadata() {
    let registry = OutputUnitRegistry::canonical_registry()
        .expect("canonical output unit registry should construct");
    let mut seen_schema_unit_columns = BTreeSet::new();
    let mut seen_dynamic_unit_columns = BTreeSet::new();
    let mut schemas = Vec::new();
    schemas.push((
        "hillslope_wat",
        hillslope_wat_schema(InterchangeVersion::default())
            .expect("hillslope WAT schema should construct"),
    ));
    schemas.extend(
        watershed_interchange_schemas().expect("watershed interchange schemas should construct"),
    );

    for (schema_id, schema) in schemas {
        for field in schema.fields() {
            let Some(unit) = field.metadata().get("units") else {
                if field
                    .metadata()
                    .get("unit_source")
                    .is_some_and(|unit_source| unit_source == "units")
                {
                    seen_dynamic_unit_columns.insert((schema_id.to_string(), field.name().clone()));
                }
                continue;
            };
            seen_schema_unit_columns.insert((schema_id.to_string(), field.name().clone()));
            let entry = registry
                .entry_for_output_column(schema_id, field.name())
                .unwrap_or_else(|error| {
                    panic!(
                        "{schema_id}.{} missing output unit registry entry: {error}",
                        field.name()
                    )
                });
            assert_eq!(
                entry.unit_label(),
                unit,
                "{schema_id}.{} schema unit must match output registry",
                field.name()
            );
        }
    }

    for entry in registry.entries() {
        if entry.unit_label() == "row_field:units" {
            assert!(
                seen_dynamic_unit_columns.contains(&(
                    entry.schema_id().to_string(),
                    entry.column_name().to_string()
                )),
                "{}.{} dynamic output registry row must correspond to schema unit_source metadata",
                entry.schema_id(),
                entry.column_name()
            );
            continue;
        }
        assert!(
            seen_schema_unit_columns.contains(&(
                entry.schema_id().to_string(),
                entry.column_name().to_string()
            )),
            "{}.{} output registry row must correspond to schema unit metadata",
            entry.schema_id(),
            entry.column_name()
        );
    }
}

#[test]
fn hphys0278_dynamic_row_level_output_units_are_registry_governed() {
    let registry = OutputUnitRegistry::canonical_registry()
        .expect("canonical output unit registry should construct");
    let schemas = watershed_interchange_schemas().expect("watershed schemas should construct");

    for schema_id in ["watershed_loss_all_years_out", "watershed_loss_average_out"] {
        let schema = schemas
            .iter()
            .find(|(candidate, _)| *candidate == schema_id)
            .map_or_else(
                || panic!("{schema_id} schema should exist"),
                |(_, schema)| schema,
            );
        let value_field = schema
            .field_with_name("value")
            .expect("dynamic output schema should include value column");
        assert_eq!(
            value_field
                .metadata()
                .get("unit_source")
                .map(String::as_str),
            Some("units"),
            "{schema_id}.value should declare row-level unit source"
        );
        assert!(
            schema.field_with_name("units").is_ok(),
            "{schema_id} should include sibling units column"
        );
        let entry = registry
            .entry_for_output_column(schema_id, "value")
            .expect("dynamic value column should resolve in output unit registry");
        assert_eq!(entry.unit_label(), "row_field:units");
    }
}

#[test]
fn wshed01_totalwatsed3_runoff_unit_lineage_is_pass_volume_publication() {
    let registry = OutputUnitRegistry::canonical_registry()
        .expect("canonical output unit registry should construct");
    let entry = registry
        .entry_for_output_column("watershed_totalwatsed3", "Runoff")
        .expect("totalwatsed3 Runoff output unit should resolve");

    assert_eq!(entry.unit_label(), "mm");
    match entry.authority() {
        OutputUnitAuthority::PublicationOnly {
            rationale,
            contract_id,
            invariant_id,
        } => {
            assert!(
                rationale.contains("PASS runoff volume"),
                "Runoff lineage rationale should name PASS volume, observed {rationale}"
            );
            assert_eq!(contract_id, "SC-WATBAL-001");
            assert_eq!(invariant_id, "SC-WATBAL-001#INV-WATBAL-054");
        }
        OutputUnitAuthority::BoundaryRegistry { boundary_alias } => {
            panic!("totalwatsed3 Runoff must not be boundary-backed by {boundary_alias}");
        }
    }
}

#[test]
fn hphys0278_output_registry_rejects_stale_boundary_units_and_unexplained_publication_units() {
    let stale_boundary_unit = OutputUnitRegistry::new([OutputUnitEntry::boundary_registry(
        "test_schema",
        "P",
        "m",
        "hillslope_wat.P",
    )])
    .expect_err("boundary-backed output units must match boundary registry units");
    assert_eq!(
        stale_boundary_unit,
        OutputUnitRegistryError::BoundaryUnitMismatch {
            row: 1,
            schema_id: "test_schema".into(),
            column_name: "P".into(),
            boundary_alias: "hillslope_wat.P".into(),
            output_unit: "m".into(),
            boundary_unit: "mm".into(),
        }
    );

    let unexplained_publication_unit =
        OutputUnitRegistry::new([OutputUnitEntry::publication_only(
            "test_schema",
            "custom_column",
            "kg",
            "",
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        )])
        .expect_err("publication-only output units require rationale");
    assert_eq!(
        unexplained_publication_unit,
        OutputUnitRegistryError::EmptyPublicationOnlyRationale {
            row: 1,
            schema_id: "test_schema".into(),
            column_name: "custom_column".into(),
        }
    );

    let unexplained_contract = OutputUnitRegistry::new([OutputUnitEntry::publication_only(
        "test_schema",
        "custom_column",
        "kg",
        "test publication-only unit",
        "",
        "SC-SYSTEM-001#INV-SYSTEM-001",
    )])
    .expect_err("publication-only output units require contract authority");
    assert_eq!(
        unexplained_contract,
        OutputUnitRegistryError::EmptyPublicationOnlyContract {
            row: 1,
            schema_id: "test_schema".into(),
            column_name: "custom_column".into(),
        }
    );

    let unexplained_invariant = OutputUnitRegistry::new([OutputUnitEntry::publication_only(
        "test_schema",
        "custom_column",
        "kg",
        "test publication-only unit",
        "SC-SYSTEM-001",
        "",
    )])
    .expect_err("publication-only output units require invariant authority");
    assert_eq!(
        unexplained_invariant,
        OutputUnitRegistryError::EmptyPublicationOnlyContract {
            row: 1,
            schema_id: "test_schema".into(),
            column_name: "custom_column".into(),
        }
    );
}

#[test]
fn registry_rejects_missing_dimensional_units_and_missing_scalar_exception_reasons() {
    let missing_unit = BoundaryUnitRegistry::new([test_entry(
        "bad_depth",
        &["bad.depth"],
        "dimensionless",
        DimensionClass::Depth,
        DomainClass::NonNegativeFinite,
        TypedBoundaryRequirement::FollowUpRequired,
        None,
    )])
    .expect_err("dimensional symbols cannot use dimensionless unit labels");
    assert_eq!(
        missing_unit,
        BoundaryUnitRegistryError::DimensionalSymbolMissingUnit {
            row: 1,
            canonical_symbol: "bad_depth".to_string(),
        }
    );

    let missing_exception = BoundaryUnitRegistry::new([test_entry(
        "bad_scalar",
        &["bad.scalar"],
        "dimensionless",
        DimensionClass::Fraction,
        DomainClass::UnitInterval,
        TypedBoundaryRequirement::ScalarException,
        None,
    )])
    .expect_err("scalar exceptions require a reason");
    assert_eq!(
        missing_exception,
        BoundaryUnitRegistryError::EmptyScalarException {
            row: 1,
            canonical_symbol: "bad_scalar".to_string(),
        }
    );
}

#[test]
fn registry_rejects_duplicate_and_ambiguous_aliases() {
    let duplicate = BoundaryUnitRegistry::new([test_entry(
        "depth",
        &["depth_m", "depth_m"],
        "m",
        DimensionClass::Depth,
        DomainClass::NonNegativeFinite,
        TypedBoundaryRequirement::FollowUpRequired,
        None,
    )])
    .expect_err("duplicate aliases in a canonical row should be rejected");
    assert_eq!(
        duplicate,
        BoundaryUnitRegistryError::DuplicateAliasMapping {
            canonical_symbol: "depth".to_string(),
            boundary_alias: "depth_m".to_string(),
        }
    );

    let ambiguous = BoundaryUnitRegistry::new([
        test_entry(
            "depth_a",
            &["depth_m"],
            "m",
            DimensionClass::Depth,
            DomainClass::NonNegativeFinite,
            TypedBoundaryRequirement::FollowUpRequired,
            None,
        ),
        test_entry(
            "depth_b",
            &["depth_m"],
            "m",
            DimensionClass::Depth,
            DomainClass::NonNegativeFinite,
            TypedBoundaryRequirement::FollowUpRequired,
            None,
        ),
    ])
    .expect_err("one alias cannot map to two canonical symbols");
    assert_eq!(
        ambiguous,
        BoundaryUnitRegistryError::AmbiguousBoundaryAlias {
            boundary_alias: "depth_m".to_string(),
            canonical_a: "depth_a".to_string(),
            canonical_b: "depth_b".to_string(),
        }
    );
}

#[test]
fn registry_rejects_bad_templates_and_ambiguous_template_matches() {
    let bad_template = BoundaryUnitRegistry::new([test_entry(
        "bad_template",
        &["depth_{day}"],
        "m",
        DimensionClass::Depth,
        DomainClass::NonNegativeFinite,
        TypedBoundaryRequirement::FollowUpRequired,
        None,
    )])
    .expect_err("unsupported brace template token should be rejected");
    assert!(matches!(
        bad_template,
        BoundaryUnitRegistryError::InvalidBoundaryAliasTemplate { .. }
    ));

    let ambiguous = BoundaryUnitRegistry::new([
        test_entry(
            "depth_a",
            &["depth_{idx4}"],
            "m",
            DimensionClass::Depth,
            DomainClass::NonNegativeFinite,
            TypedBoundaryRequirement::FollowUpRequired,
            None,
        ),
        test_entry(
            "depth_b",
            &["depth_000{ofe}"],
            "m",
            DimensionClass::Depth,
            DomainClass::NonNegativeFinite,
            TypedBoundaryRequirement::FollowUpRequired,
            None,
        ),
    ])
    .expect("overlapping templates should construct until concrete lookup");
    assert_eq!(
        ambiguous
            .entry_for_boundary_alias("depth_0001")
            .expect_err("overlapping templates should fail concrete lookup"),
        BoundaryUnitRegistryError::AmbiguousBoundaryAlias {
            boundary_alias: "depth_0001".to_string(),
            canonical_a: "depth_a".to_string(),
            canonical_b: "depth_b".to_string(),
        }
    );
}

#[test]
fn registry_rejects_duplicate_publication_aliases() {
    let duplicate = BoundaryUnitRegistry::new([
        test_entry_with_publication(
            "depth_a",
            &["depth_a"],
            "m",
            DimensionClass::Depth,
            DomainClass::NonNegativeFinite,
            &["hillslope_wat.Depth:mm"],
        ),
        test_entry_with_publication(
            "depth_b",
            &["depth_b"],
            "mm",
            DimensionClass::Depth,
            DomainClass::NonNegativeFinite,
            &["hillslope_wat.Depth:mm"],
        ),
    ])
    .expect_err("one publication alias cannot be owned by two canonical rows");
    assert_eq!(
        duplicate,
        BoundaryUnitRegistryError::AmbiguousPublicationAlias {
            publication_alias: "hillslope_wat.Depth:mm".to_string(),
            canonical_a: "depth_a".to_string(),
            canonical_b: "depth_b".to_string(),
        }
    );
}

fn test_entry(
    canonical_symbol: &'static str,
    boundary_aliases: &'static [&'static str],
    unit_label: &'static str,
    dimension_class: DimensionClass,
    domain_class: DomainClass,
    typed_boundary: TypedBoundaryRequirement,
    scalar_exception: Option<&'static str>,
) -> BoundaryUnitEntry {
    BoundaryUnitEntry::new(
        canonical_symbol,
        boundary_aliases,
        unit_label,
        dimension_class,
        domain_class,
        "test-producer",
        "test-consumer",
        "SC-TEST-001",
        "SC-TEST-001#INV-TEST-001",
        typed_boundary,
        scalar_exception,
        &[],
    )
}

fn test_entry_with_publication(
    canonical_symbol: &'static str,
    boundary_aliases: &'static [&'static str],
    unit_label: &'static str,
    dimension_class: DimensionClass,
    domain_class: DomainClass,
    publication_aliases: &'static [&'static str],
) -> BoundaryUnitEntry {
    BoundaryUnitEntry::new(
        canonical_symbol,
        boundary_aliases,
        unit_label,
        dimension_class,
        domain_class,
        "test-producer",
        "test-consumer",
        "SC-TEST-001",
        "SC-TEST-001#INV-TEST-001",
        TypedBoundaryRequirement::FollowUpRequired,
        None,
        publication_aliases,
    )
}
