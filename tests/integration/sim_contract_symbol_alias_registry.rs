use openwepp_sim_contract::symbols::{
    SymbolAliasEntry, SymbolAliasRegistry, SymbolAliasRegistryError,
};

#[test]
fn canonical_wepp_registry_contains_arch03_baseline_aliases() {
    let registry = SymbolAliasRegistry::canonical_wepp_registry()
        .expect("canonical WEPP alias registry should construct");

    let runoff_aliases = registry
        .aliases_for_canonical("runoff")
        .expect("runoff canonical symbol should exist");
    assert_eq!(runoff_aliases, ["runoff_depth_m"]);

    let canonical = registry
        .canonical_for_boundary_alias("runoff_depth_m")
        .expect("runoff alias should resolve");
    assert_eq!(canonical, "runoff");
}

#[test]
fn reverse_lookup_resolves_each_boundary_alias_to_single_canonical_symbol() {
    let registry = SymbolAliasRegistry::canonical_wepp_registry()
        .expect("canonical WEPP alias registry should construct");

    assert_eq!(
        registry
            .canonical_for_boundary_alias("tile_drain_flow_m")
            .expect("tile drain alias should resolve"),
        "drainq"
    );
    assert_eq!(
        registry
            .canonical_for_boundary_alias("runoff_duration_s")
            .expect("runoff duration alias should resolve"),
        "watdur"
    );
}

#[test]
fn missing_canonical_or_boundary_alias_returns_typed_errors() {
    let registry = SymbolAliasRegistry::canonical_wepp_registry()
        .expect("canonical WEPP alias registry should construct");

    let canonical_error = registry
        .aliases_for_canonical("does_not_exist")
        .expect_err("unknown canonical symbol should fail");
    assert_eq!(
        canonical_error,
        SymbolAliasRegistryError::CanonicalSymbolNotFound {
            canonical_symbol: "does_not_exist".to_string(),
        }
    );

    let alias_error = registry
        .canonical_for_boundary_alias("unknown_alias")
        .expect_err("unknown boundary alias should fail");
    assert_eq!(
        alias_error,
        SymbolAliasRegistryError::BoundaryAliasNotFound {
            boundary_alias: "unknown_alias".to_string(),
        }
    );
}

#[test]
fn constructor_rejects_ambiguous_boundary_aliases() {
    let error = SymbolAliasRegistry::new([
        SymbolAliasEntry::new("runoff", "shared_alias"),
        SymbolAliasEntry::new("runvol", "shared_alias"),
    ])
    .expect_err("same boundary alias must not map to multiple canonical symbols");

    assert_eq!(
        error,
        SymbolAliasRegistryError::AmbiguousBoundaryAlias {
            boundary_alias: "shared_alias".to_string(),
            canonical_a: "runoff".to_string(),
            canonical_b: "runvol".to_string(),
        }
    );
}

#[test]
fn constructor_rejects_duplicate_rows() {
    let error = SymbolAliasRegistry::new([
        SymbolAliasEntry::new("runoff", "runoff_depth_m"),
        SymbolAliasEntry::new("runoff", "runoff_depth_m"),
    ])
    .expect_err("duplicate canonical-alias rows must fail");

    assert_eq!(
        error,
        SymbolAliasRegistryError::DuplicateAliasMapping {
            canonical_symbol: "runoff".to_string(),
            boundary_alias: "runoff_depth_m".to_string(),
        }
    );
}
