use openwepp_hillslope_output::hillslope_wat::{InterchangeVersion, hillslope_wat_schema};
use openwepp_sim_contract::units::{
    BoundaryUnitEntry, BoundaryUnitRegistry, BoundaryUnitRegistryError, DimensionClass,
    DomainClass, TypedBoundaryRequirement, hphys0274_required_boundary_aliases,
};

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
        ("hillslope_wat.SoilWaterTotal", "mm", DimensionClass::Depth),
        ("hillslope_wat.Snow-Water", "mm", DimensionClass::Depth),
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
        "snow.runtime_depth_m",
        "snow.runtime_density_kg_m3",
        "snow.hourly.rain_m_0001",
        "snow.hourly.rain_retained_m_0001",
        "snow.hourly.snowfall_m_0001",
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
    let schema = hillslope_wat_schema(InterchangeVersion::default());

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
