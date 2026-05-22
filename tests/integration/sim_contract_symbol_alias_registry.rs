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
fn canonical_wepp_registry_contains_sr04_slope_soil_alias_entries() {
    let registry = SymbolAliasRegistry::canonical_wepp_registry()
        .expect("canonical WEPP alias registry should construct");

    let xinput_aliases = registry
        .aliases_for_canonical("xinput")
        .expect("xinput canonical symbol should exist");
    assert_eq!(xinput_aliases, ["ofe{ofe}_xinput_{idx4}", "xinput_{idx4}"]);

    let nsl_aliases = registry
        .aliases_for_canonical("nsl")
        .expect("nsl canonical symbol should exist");
    assert_eq!(nsl_aliases, ["nsl", "ofe{ofe}_nsl"]);

    let ssc_aliases = registry
        .aliases_for_canonical("ssc")
        .expect("ssc canonical symbol should exist");
    assert_eq!(ssc_aliases, ["ofe{ofe}_ssc_{idx4}", "ssc", "ssc_{idx4}"]);
}

#[test]
fn canonical_wepp_registry_contains_pl04_schedule_growth_and_decomp_alias_entries() {
    let registry = SymbolAliasRegistry::canonical_wepp_registry()
        .expect("canonical WEPP alias registry should construct");

    let lanuse_aliases = registry
        .aliases_for_canonical("lanuse")
        .expect("lanuse canonical symbol should exist");
    assert_eq!(lanuse_aliases, ["lanuse", "ofe{ofe}_lanuse"]);

    let itype_aliases = registry
        .aliases_for_canonical("itype")
        .expect("itype canonical symbol should exist");
    assert_eq!(
        itype_aliases,
        ["itype", "itype_{idx4}", "ofe{ofe}_itype_{idx4}"]
    );

    let gday_aliases = registry
        .aliases_for_canonical("gday")
        .expect("gday canonical symbol should exist");
    assert_eq!(
        gday_aliases,
        ["gday", "gday_{idx4}", "ofe{ofe}_gday_{idx4}"]
    );

    let vdmt_aliases = registry
        .aliases_for_canonical("vdmt")
        .expect("vdmt canonical symbol should exist");
    assert_eq!(vdmt_aliases, ["ofe{ofe}_vdmt", "vdmt"]);

    let rmogt_aliases = registry
        .aliases_for_canonical("rmogt")
        .expect("rmogt canonical symbol should exist");
    assert_eq!(rmogt_aliases, ["ofe{ofe}_rmogt_{idx4}", "rmogt_{idx4}"]);

    let senvin_aliases = registry
        .aliases_for_canonical("senvin")
        .expect("senvin canonical symbol should exist");
    assert_eq!(senvin_aliases, ["ofe{ofe}_senvin", "senvin"]);
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
    assert_eq!(
        registry
            .canonical_for_boundary_alias("ofe2_xinput_0003")
            .expect("indexed slope alias should resolve"),
        "xinput"
    );
    assert_eq!(
        registry
            .canonical_for_boundary_alias("slpinp_0002")
            .expect("primary-OFE slope alias should resolve"),
        "slpinp"
    );
    assert_eq!(
        registry
            .canonical_for_boundary_alias("ofe5_scc_0002")
            .expect_err("typo alias must remain unresolved"),
        SymbolAliasRegistryError::BoundaryAliasNotFound {
            boundary_alias: "ofe5_scc_0002".to_string(),
        }
    );
    assert_eq!(
        registry
            .canonical_for_boundary_alias("ofe5_ssc_0002")
            .expect("indexed soil alias should resolve"),
        "ssc"
    );
}

#[test]
fn reverse_lookup_resolves_pl04_aliases_to_single_canonical_symbol() {
    let registry = SymbolAliasRegistry::canonical_wepp_registry()
        .expect("canonical WEPP alias registry should construct");

    assert_eq!(
        registry
            .canonical_for_boundary_alias("ofe3_lanuse")
            .expect("ofe-scoped schedule alias should resolve"),
        "lanuse"
    );
    assert_eq!(
        registry
            .canonical_for_boundary_alias("itype_0007")
            .expect("slot-indexed schedule alias should resolve"),
        "itype"
    );
    assert_eq!(
        registry
            .canonical_for_boundary_alias("ofe4_jdharv_0002")
            .expect("ofe + slot-indexed schedule alias should resolve"),
        "jdharv"
    );
    assert_eq!(
        registry
            .canonical_for_boundary_alias("ofe2_vdmx")
            .expect("ofe-scoped growth alias should resolve"),
        "vdmx"
    );
    assert_eq!(
        registry
            .canonical_for_boundary_alias("ofe5_rmogt_0003")
            .expect("ofe + slot-indexed decomposition alias should resolve"),
        "rmogt"
    );
    assert_eq!(
        registry
            .canonical_for_boundary_alias("fenvin_0002")
            .expect("slot-indexed decomposition alias should resolve"),
        "fenvin"
    );
    assert_eq!(
        registry
            .canonical_for_boundary_alias("ofe5_rmogt_002")
            .expect_err("invalid idx width must remain unresolved"),
        SymbolAliasRegistryError::BoundaryAliasNotFound {
            boundary_alias: "ofe5_rmogt_002".to_string(),
        }
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

#[test]
fn constructor_rejects_invalid_template_token() {
    let error =
        SymbolAliasRegistry::new([SymbolAliasEntry::new("xinput", "ofe{bad}_xinput_{idx4}")])
            .expect_err("unsupported template tokens must fail");

    assert_eq!(
        error,
        SymbolAliasRegistryError::InvalidBoundaryAliasTemplate {
            row: 1,
            canonical_symbol: "xinput".to_string(),
            boundary_alias: "ofe{bad}_xinput_{idx4}".to_string(),
            reason: "unsupported token {bad}".to_string(),
        }
    );
}

#[test]
fn constructor_rejects_ambiguous_template_alias_strings() {
    let error = SymbolAliasRegistry::new([
        SymbolAliasEntry::new("rmogt", "ofe{ofe}_shared_{idx4}"),
        SymbolAliasEntry::new("smrm", "ofe{ofe}_shared_{idx4}"),
    ])
    .expect_err("same template alias must not map to multiple canonical symbols");

    assert_eq!(
        error,
        SymbolAliasRegistryError::AmbiguousBoundaryAlias {
            boundary_alias: "ofe{ofe}_shared_{idx4}".to_string(),
            canonical_a: "rmogt".to_string(),
            canonical_b: "smrm".to_string(),
        }
    );
}

#[test]
fn reverse_lookup_reports_ambiguous_template_matches() {
    let registry = SymbolAliasRegistry::new([
        SymbolAliasEntry::new("alpha", "ofe{ofe}_shared_{idx4}"),
        SymbolAliasEntry::new("beta", "ofe1_shared_{idx4}"),
    ])
    .expect("template registry should construct");

    assert_eq!(
        registry
            .canonical_for_boundary_alias("ofe1_shared_0001")
            .expect_err("overlapping templates must produce a typed ambiguity"),
        SymbolAliasRegistryError::AmbiguousBoundaryAlias {
            boundary_alias: "ofe1_shared_0001".to_string(),
            canonical_a: "alpha".to_string(),
            canonical_b: "beta".to_string(),
        }
    );
}
