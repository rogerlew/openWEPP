#[allow(clippy::too_many_lines)]
fn install_authenticated_litter_forcing(run_dir: &Path) {
    const NEEDLE_CSV: &str =
        "date,deposited_kg_m2\n2000-01-01,0.002\n2000-01-02,0.004\n";
    const NEEDLE_DIGEST: &str =
        "ca265fed6835ac2edd0ef88dcf306999820cf8ffbaec83c6da7428a71c5b1e1a";
    const WOOD_CSV: &str =
        "date,deposited_kg_m2\n2000-01-01,0.003\n2000-01-02,0.005\n";
    const WOOD_DIGEST: &str =
        "b3a5cef1814e4b2729e7e2ee534f2144b50dce03504e973d080830fdfd1ce23c";
    const VEGETATION_CSV: &str = "functional_class\nneedleleaf_evergreen\n";
    const VEGETATION_DIGEST: &str =
        "5c99ff73f10e2ae51e4b37a4e5a4690c9b8273a4b1f5170c077ec7884441a3bc";
    let management_path = run_dir.join("case.man.yaml");
    let source =
        fs::read_to_string(&management_path).expect("native forest YAML should be readable");
    let block = format!(
        r#"    surface_litter_forcing:
      vegetation:
        functional_classes:
          - needleleaf_evergreen
        authority:
          source_identity: authenticated runtime fixture classification
          source_uri_or_path: vegetation.csv
          access_or_version_date: "2026-07-28"
          claim_anchor: complete file
          digest_algorithm: sha256
          source_digest: {VEGETATION_DIGEST}
      needle:
        status: complete
        payload:
          mode: prescribed_scenario
          support_start: "2000-01-01"
          support_end: "2000-01-02"
          calendar: proleptic_gregorian
          species_or_functional_type: needleleaf_evergreen
          included_material: fallen needles
          excluded_material: cones, bark, and woody material
          mass_basis:
            state: dry_to_constant_mass
            drying_temperature_c: 65.0
            constant_mass_criterion: fixture dry-mass definition
            horizontal_area_basis: true
            units: kg_dry_mass_m2_day
          spatial_support:
            site_or_plot: Forest_Management
            ofe_binding: 1
          authority:
            source_identity: prescribed needle fixture
            source_uri_or_path: needle.csv
            access_or_version_date: "2026-07-28"
            claim_anchor: complete file
            digest_algorithm: sha256
            source_digest: {NEEDLE_DIGEST}
          original_observation:
            support_start: "2000-01-01"
            support_end: "2000-01-02"
            resolution: exact_daily
            units: kg_dry_mass_m2_day
          executable_forcing:
            path: needle.csv
            digest_algorithm: sha256
            executable_digest: {NEEDLE_DIGEST}
      fine_woody:
        status: complete
        payload:
          mode: prescribed_scenario
          support_start: "2000-01-01"
          support_end: "2000-01-02"
          calendar: proleptic_gregorian
          species_or_functional_type: needleleaf_evergreen
          included_material: fallen twigs and branches
          excluded_material: needles, cones, and stems above 10 mm
          mass_basis:
            state: dry_to_constant_mass
            drying_temperature_c: 65.0
            constant_mass_criterion: fixture dry-mass definition
            horizontal_area_basis: true
            units: kg_dry_mass_m2_day
          spatial_support:
            site_or_plot: Forest_Management
            ofe_binding: 1
          authority:
            source_identity: prescribed fine-woody fixture
            source_uri_or_path: fine-woody.csv
            access_or_version_date: "2026-07-28"
            claim_anchor: complete file
            digest_algorithm: sha256
            source_digest: {WOOD_DIGEST}
          original_observation:
            support_start: "2000-01-01"
            support_end: "2000-01-02"
            resolution: exact_daily
            units: kg_dry_mass_m2_day
          executable_forcing:
            path: fine-woody.csv
            digest_algorithm: sha256
            executable_digest: {WOOD_DIGEST}
          maximum_diameter_mm: 10.0
          bark_treatment: included
"#
    );
    let management = source.replacen("    cf: 5.0", &format!("{block}    cf: 5.0"), 1);
    assert_ne!(management, source, "forcing block insertion must occur");
    fs::write(management_path, management).expect("authenticated management fixture writes");
    fs::write(run_dir.join("needle.csv"), NEEDLE_CSV).expect("needle forcing writes");
    fs::write(run_dir.join("fine-woody.csv"), WOOD_CSV).expect("fine-woody forcing writes");
    fs::write(run_dir.join("vegetation.csv"), VEGETATION_CSV)
        .expect("vegetation classification writes");
}
