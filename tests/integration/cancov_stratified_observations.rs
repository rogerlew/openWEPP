use std::fs;
use std::path::Path;

const ROOT: &str = "tests/fixtures/cancov_forest/observations";

#[test]
fn cancov_stratified_observation_manifest_installs_harvard_and_marcell_sources() {
    let manifest = read("tests/fixtures/cancov_forest/observations/manifest.json");
    assert!(manifest.contains("cancov-stratified-observations-manifest-v1"));
    assert!(manifest.contains("harvard_hf237"));
    assert!(manifest.contains("marcell_rds_2021_0016"));
    assert!(manifest.contains("HF237"));
    assert!(manifest.contains("RDS-2021-0016"));
    assert!(manifest.contains("\"row_count\": 2463"));
    assert!(manifest.contains("\"row_count\": 19114"));
    assert!(manifest.contains("\"row_count\": 706"));

    let harvard_provenance = read(format!("{ROOT}/provenance/harvard_hf237.json"));
    assert!(harvard_provenance.contains("https://doi.org/10.6073/pasta/"));
    assert!(harvard_provenance.contains("CC0-1.0"));
    assert!(harvard_provenance.contains("unbound because no pure Harvard hemlock"));

    let marcell_provenance = read(format!("{ROOT}/provenance/marcell_rds_2021_0016.json"));
    assert!(marcell_provenance.contains("https://doi.org/10.2737/RDS-2021-0016"));
    assert!(marcell_provenance.contains("S53 appears in the snow/SWE table"));
}

#[test]
fn cancov_stratified_observation_tables_bind_expected_model_strata() {
    let harvard = read(format!("{ROOT}/sites/harvard_hf237_strata.csv"));
    assert!(harvard.starts_with(
        "source_id,observation_site,observed_stratum,binding_status,model_fixture,date,"
    ));
    assert!(harvard.contains("open,bound,harvard_open_ma"));
    assert!(harvard.contains("hardwood,bound,harvard_deciduous_ma"));
    assert!(harvard.contains("hemlock,unbound_no_pure_conifer_fixture,"));
    assert_eq!(line_count(&harvard), 2464);

    let marcell = read(format!(
        "{ROOT}/sites/marcell_rds_2021_0016_stratum_means.csv"
    ));
    assert!(marcell.contains("conifer,bound,marcell_conifer_mn"));
    assert!(marcell.contains("deciduous,bound,marcell_deciduous_mn"));
    assert!(marcell.contains("open,bound,marcell_open_mn"));
    assert!(!marcell.contains("unknown"));
    assert_eq!(line_count(&marcell), 707);

    let points = read(format!("{ROOT}/sites/marcell_rds_2021_0016_points.csv"));
    assert!(points.contains("unknown,unbound_metadata_unmapped"));
    assert!(points.contains("metadata_snowcourse_unmapped"));
}

#[test]
fn cancov_stratified_observation_regeneration_tool_is_present() {
    let source = read("tools/snowfreeze_observed/cancov_stratified_observations.py");
    assert!(source.contains("HARVARD_DEPTH_URL"));
    assert!(source.contains("MARCELL_ZIP_URL"));
    assert!(source.contains("EXPECTED_HASHES"));
    assert!(source.contains("marcell_rds_2021_0016_stratum_means.csv"));
}

fn read(path: impl AsRef<Path>) -> String {
    fs::read_to_string(path.as_ref())
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.as_ref().display()))
}

fn line_count(text: &str) -> usize {
    text.lines().count()
}
