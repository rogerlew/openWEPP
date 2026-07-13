use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use openwepp_landuse_migrate::cli::help_text;
use openwepp_management_schema as management_yaml;

const LEGACY_CROPLAND_98_4: &str = r"98.4
#
#
#
#

1 # number of OFE's
1 # (total) years in simulation

#######################
# Plant Section       #
#######################

1  # Number of plant scenarios


Corn
High production level-125 bu/acre for Jefferson Iowa
J. M. Laflen, Feb 28, 1998
Cutting height 1 foot,  non-fragile residue,  30 inch rows
1  #landuse
WeppWillSet
3.60000 3.00000 35.00196 10.00000 2.30000 55.00000 0.00000 0.30404 0.65000 0.05100
0.85000 0.98000 0.65000 0.99000 0.00000 1700.00000 0.50000 2.60099
2  # mfo - <non fragile>
0.00650 0.00650 25.00000 0.25000 0.21900 1.51995 0.25000 0.00000 30 0.00000
0.00000 3.50000 0.00000

#######################
# Operation Section   #
#######################

1  # Number of operation scenarios


PLNTFC
`Planter, no-till with fluted coulter'
(from WEPP distribution database)

1  #landuse
0.2500 0.1500 0
4 # pcode - other
0.0250 0.7500 0.2500 0.1500 0.0120 0.1500 0.0000



###############################
# Initial Conditions Section  #
###############################

1  # Number of initial scenarios


Default
Default corn initial conditions set - continuous corn - spring/summer tillage only
90 percent cover, approximately 200 days since last tillage
500 mm of rain since last tillage in summer prior
1  #landuse
1.10000 0.00000 200 92 0.00000 0.90000
1 # iresd  <Corn>
1 # mang annual
500.12601 0.02000 0.90000 0.02000 0.00000
1  # rtyp - temporary
0.00000 0.00000 0.10000 0.20000 0.02540
0.50003 0.19997




############################
# Surface Effects Section  #
############################

1  # Number of Surface Effects Scenarios


#
#   Surface Effects Scenario 1 of 1
#
Year 1
From WEPP database
Your name, phone

1  # landuse  - cropland
1 # ntill - number of operations
  130  # mdate  --- 5 / 10
  1  # op --- PLNTFC
      0.051  # depth
      2  # type


#######################
# Contouring Section  #
#######################

0  # Number of contour scenarios


#######################
# Drainage Section    #
#######################

0  # Number of drainage scenarios


#######################
# Yearly Section      #
#######################

1  # looper; number of Yearly Scenarios
#
# Yearly scenario 1 of 1
#
Year 1



1  # landuse <cropland>
1  # plant growth scenario
1  # surface effect scenario
0  # contour scenario
0  # drainage scenario
1 # management <annual>
   288  # harvest date --- 10 / 15
   130  # planting date --- 5 /10
   0.7620  # row width
   6   # residue man - <none>


#######################
# Management Section  #
#######################

Manage
description 1
description 2
description 3
1   # number of OFE's
    1   # initial condition index
1  # rotation repeats
1  # years in rotation

#
# Rotation 1: year 1 to 1
#

   1	#  <plants/yr 1> - OFE: 1>
      1	# year index
";

fn temp_dir(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time must be after epoch")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!(
        "openwepp-landuse-migrate-{label}-{}-{nanos}",
        std::process::id()
    ));
    fs::create_dir_all(&dir).expect("temp dir should be created");
    dir
}

fn write_legacy_fixture(dir: &Path, name: &str) -> PathBuf {
    let path = dir.join(name);
    fs::write(&path, LEGACY_CROPLAND_98_4).expect("fixture should write");
    path
}

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_openwepp-landuse-migrate"))
}

#[test]
fn args_for_legacy_reports_required_disturbed_class() {
    let dir = temp_dir("args");
    let input = write_legacy_fixture(&dir, "field.man");
    let output = bin()
        .arg(&input)
        .args(["--args-for-migration-to", "ow-lanuse-1", "--format", "json"])
        .output()
        .expect("cli should execute");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("--disturbed-class <class> or --disturbed-class-map <path>"));
    assert!(stdout.contains("openwepp-management-yaml"));
}

#[test]
fn validate_legacy_without_class_fails_closed() {
    let dir = temp_dir("validate-missing-class");
    let input = write_legacy_fixture(&dir, "field.man");
    let output = bin()
        .arg(&input)
        .args(["--validate", "--to", "ow-lanuse-1"])
        .output()
        .expect("cli should execute");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("stderr should be utf8");
    assert!(stderr.contains("missing disturbed-class authority"));
    assert!(stderr.contains("--args-for-migration-to ow-lanuse-1"));
}

#[test]
fn legacy_global_class_writes_default_man_yaml() {
    let dir = temp_dir("migrate-global");
    let input = write_legacy_fixture(&dir, "field.man");
    let output = bin()
        .arg(&input)
        .args([
            "--to",
            "ow-lanuse-1",
            "--disturbed-class",
            "agriculture crops",
        ])
        .output()
        .expect("cli should execute");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let yaml_path = dir.join("field.man.yaml");
    let document = management_yaml::parse_management_yaml_from_path(&yaml_path)
        .expect("migrated YAML should validate");
    assert_eq!(document.datver, "ow-lanuse-1");
    let management_yaml::PlantScenario::NativeCropland {
        routing_coefficients: Some(routing),
        ..
    } = &document.plants[0]
    else {
        panic!("expected native cropland with route coefficients");
    };
    assert_close(routing.k_o, 480.0);
    assert_eq!(routing.authority.disturbed_class, "agriculture crops");
}

#[test]
fn dry_run_writes_report_but_no_output_yaml() {
    let dir = temp_dir("dry-run");
    let input = write_legacy_fixture(&dir, "field.man");
    let report = dir.join("report.json");
    let output = bin()
        .arg(&input)
        .args([
            "--to",
            "ow-lanuse-1",
            "--disturbed-class",
            "agriculture crops",
            "--dry-run",
            "--format",
            "json",
            "--report",
        ])
        .arg(&report)
        .args(["--report-format", "json"])
        .output()
        .expect("cli should execute");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!dir.join("field.man.yaml").exists());
    assert!(report.exists());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("\"dry_run\": true"));
}

#[test]
fn overwrite_without_force_fails_closed() {
    let dir = temp_dir("overwrite");
    let input = write_legacy_fixture(&dir, "field.man");
    let output_path = dir.join("field.man.yaml");
    fs::write(&output_path, "existing").expect("existing output should write");
    let output = bin()
        .arg(&input)
        .args([
            "--to",
            "ow-lanuse-1",
            "--disturbed-class",
            "agriculture crops",
        ])
        .output()
        .expect("cli should execute");

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("already exists"));
    assert_eq!(
        fs::read_to_string(output_path).expect("existing output should remain"),
        "existing"
    );
}

#[test]
fn producer_rejects_non_lowercase_yaml_outputs() {
    for extension in ["YAML", "yml", "YML"] {
        let dir = temp_dir(&format!("bad-extension-{extension}"));
        let input = write_legacy_fixture(&dir, "field.man");
        let output = bin()
            .arg(&input)
            .args([
                "--to",
                "ow-lanuse-1",
                "--disturbed-class",
                "agriculture crops",
                "--output",
            ])
            .arg(dir.join(format!("field.{extension}")))
            .output()
            .expect("cli should execute");

        assert!(
            !output.status.success(),
            "extension {extension} should fail"
        );
        assert!(String::from_utf8_lossy(&output.stderr).contains("lowercase .yaml"));
    }
}

#[test]
fn partial_class_map_fails_closed() {
    let dir = temp_dir("partial-map");
    let input = write_legacy_fixture(&dir, "field.man");
    let map_path = dir.join("partial.json");
    fs::write(
        &map_path,
        r#"{"plant_index":{"2":{"disturbed_class":"agriculture crops"}}}"#,
    )
    .expect("map should write");
    let output = bin()
        .arg(&input)
        .args(["--validate", "--to", "ow-lanuse-1", "--disturbed-class-map"])
        .arg(&map_path)
        .output()
        .expect("cli should execute");

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("partial class map"));
}

#[test]
fn conflicting_args_file_and_class_map_fails_closed() {
    let dir = temp_dir("conflict-map");
    let input = write_legacy_fixture(&dir, "field.man");
    let args_path = dir.join("args.json");
    fs::write(
        &args_path,
        r#"{"disturbed_class_map":{"plant_index":{"1":{"disturbed_class":"agriculture crops"}}}}"#,
    )
    .expect("args file should write");
    let map_path = dir.join("map.json");
    fs::write(
        &map_path,
        r#"{"plant_index":{"1":{"disturbed_class":"bare"}}}"#,
    )
    .expect("map should write");
    let output = bin()
        .arg(&input)
        .args(["--validate", "--to", "ow-lanuse-1", "--args-file"])
        .arg(&args_path)
        .arg("--disturbed-class-map")
        .arg(&map_path)
        .output()
        .expect("cli should execute");

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("conflicting disturbed classes"));
}

#[test]
fn unknown_disturbed_class_fails_closed() {
    let dir = temp_dir("unknown-class");
    let input = write_legacy_fixture(&dir, "field.man");
    let output = bin()
        .arg(&input)
        .args([
            "--validate",
            "--to",
            "ow-lanuse-1",
            "--disturbed-class",
            "not a class",
        ])
        .output()
        .expect("cli should execute");

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("unknown disturbed class"));
}

#[test]
fn unsupported_legacy_rangeland_fails_closed() {
    let dir = temp_dir("legacy-rangeland");
    let input = dir.join("range.man");
    fs::write(
        &input,
        LEGACY_CROPLAND_98_4.replace("1  #landuse", "2  #landuse"),
    )
    .expect("fixture should write");
    let output = bin()
        .arg(&input)
        .args([
            "--to",
            "ow-lanuse-1",
            "--disturbed-class",
            "agriculture crops",
        ])
        .output()
        .expect("cli should execute");

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("unsupported landuse"));
}

#[test]
fn flat_ow_lanuse_1_with_coefficients_migrates_without_disturbed_class() {
    let dir = temp_dir("native-flat");
    let input = dir.join("native.man");
    fs::write(&input, native_flat_fixture()).expect("native flat fixture should write");
    let output_path = dir.join("native.man.yaml");
    let output = bin()
        .arg(&input)
        .args(["--to", "latest", "--output"])
        .arg(&output_path)
        .output()
        .expect("cli should execute");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let document = management_yaml::parse_management_yaml_from_path(&output_path)
        .expect("native flat migration output should validate");
    let management_yaml::PlantScenario::NativeCropland {
        routing_coefficients: Some(routing),
        ..
    } = &document.plants[0]
    else {
        panic!("expected native cropland with route coefficients");
    };
    assert_close(routing.k_o, 480.0);
    assert_eq!(
        routing.authority.source,
        "flat-ow-lanuse-1-routing_coefficients"
    );
}

#[test]
fn native_yaml_latest_passthrough_preserves_coefficients() {
    let dir = temp_dir("native-yaml-latest");
    let input = write_legacy_fixture(&dir, "field.man");
    let first_output = bin()
        .arg(&input)
        .args([
            "--to",
            "ow-lanuse-1",
            "--disturbed-class",
            "agriculture crops",
        ])
        .output()
        .expect("initial migration should execute");
    assert!(
        first_output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&first_output.stderr)
    );

    let source_yaml = dir.join("field.man.yaml");
    let latest_yaml = dir.join("latest.yaml");
    let pass_output = bin()
        .arg(&source_yaml)
        .args(["--to", "latest", "--output"])
        .arg(&latest_yaml)
        .output()
        .expect("latest pass-through should execute");
    assert!(
        pass_output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&pass_output.stderr)
    );

    let document = management_yaml::parse_management_yaml_from_path(&latest_yaml)
        .expect("latest YAML should validate");
    let management_yaml::PlantScenario::NativeCropland {
        routing_coefficients: Some(routing),
        ..
    } = &document.plants[0]
    else {
        panic!("expected native cropland with route coefficients");
    };
    assert_close(routing.k_o, 480.0);
    assert_eq!(routing.authority.disturbed_class, "agriculture crops");
}

#[test]
fn m13_empty_and_help_flags_print_the_exact_help_contract() {
    for args in [vec![], vec!["--help"], vec!["-h"]] {
        let output = bin().args(args).output().expect("CLI help should execute");
        assert!(output.status.success());
        assert_eq!(
            String::from_utf8(output.stdout).expect("help should be UTF-8"),
            help_text(),
        );
        assert!(output.stderr.is_empty());
    }
}

#[test]
fn m13_every_value_option_fails_at_the_missing_value() {
    for flag in [
        "--args-for-migration-to",
        "--to",
        "--output",
        "--disturbed-class",
        "--disturbed-class-map",
        "--args-file",
        "--report",
        "--report-format",
        "--format",
    ] {
        let output = bin()
            .arg("input.man")
            .arg(flag)
            .output()
            .expect("missing-value CLI should execute");
        assert!(!output.status.success(), "{flag} should fail");
        assert_eq!(
            String::from_utf8(output.stderr).expect("error should be UTF-8"),
            format!("LANDUSE-MIGRATE-E-016: missing value for {flag}\n"),
        );
        assert!(output.stdout.is_empty());
    }
}

#[test]
fn m13_discovery_rejects_each_incompatible_mode_before_source_work() {
    let dir = temp_dir("m13-discovery-conflicts");
    let input = write_legacy_fixture(&dir, "field.man");
    let cases: Vec<Vec<String>> = vec![
        vec!["--validate".to_string()],
        vec!["--to".to_string(), "latest".to_string()],
        vec![
            "--output".to_string(),
            dir.join("out.yaml").display().to_string(),
        ],
    ];
    for incompatible in cases {
        let output = bin()
            .arg(&input)
            .args(["--args-for-migration-to", "latest"])
            .args(&incompatible)
            .output()
            .expect("discovery-conflict CLI should execute");
        assert!(!output.status.success());
        assert_eq!(
            String::from_utf8(output.stderr).expect("error should be UTF-8"),
            "LANDUSE-MIGRATE-E-016: --args-for-migration-to cannot be combined with --validate, --to, or --output\n",
        );
    }
}

#[test]
fn m13_args_file_target_inheritance_runs_real_dry_migration() {
    let dir = temp_dir("m13-target-inheritance");
    let input = write_legacy_fixture(&dir, "field.man");
    let args_path = dir.join("args.json");
    fs::write(
        &args_path,
        r#"{"target":"latest","disturbed_class":"agriculture crops"}"#,
    )
    .expect("args file should write");
    let output = bin()
        .arg(&input)
        .args(["--args-file", args_path.to_str().unwrap(), "--dry-run"])
        .args(["--format", "json"])
        .output()
        .expect("inherited-target CLI should execute");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("stdout should be a JSON report");
    assert_eq!(report["target_datver"], "ow-lanuse-1");
    assert_eq!(report["dry_run"], true);
    assert!(!dir.join("field.man.yaml").exists());
}

#[test]
fn m13_validation_writes_requested_report_and_formats_stdout_independently() {
    let dir = temp_dir("m13-validation-report");
    let input = write_legacy_fixture(&dir, "field.man");
    let report_path = dir.join("validation.json");
    let output = bin()
        .arg(&input)
        .args([
            "--validate",
            "--to",
            "latest",
            "--disturbed-class",
            "agriculture crops",
            "--report",
            report_path.to_str().unwrap(),
            "--report-format",
            "json",
            "--format",
            "toml",
        ])
        .output()
        .expect("validation CLI should execute");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("stdout should be UTF-8");
    assert!(stdout.contains("valid = true"));
    assert!(stdout.contains("target_datver = \"ow-lanuse-1\""));
    let report: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&report_path).expect("validation report should exist"),
    )
    .expect("validation report should be JSON");
    assert_eq!(report["valid"], true);
    assert_eq!(report["target_datver"], "ow-lanuse-1");
}

#[test]
fn m13_validation_target_defaults_only_for_yaml_and_migration_requires_target() {
    let yaml_input = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
        "../../tests/fixtures/infile/management/canonical_forest_nonzero_ow_lanuse_1.man.yaml",
    );
    let yaml_output = bin()
        .arg(&yaml_input)
        .args(["--validate", "--format", "json"])
        .output()
        .expect("native-YAML validation should execute");
    assert!(
        yaml_output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&yaml_output.stderr)
    );
    let yaml_report: serde_json::Value =
        serde_json::from_slice(&yaml_output.stdout).expect("validation should emit JSON");
    assert_eq!(yaml_report["target_datver"], "ow-lanuse-1");

    let dir = temp_dir("m13-missing-target");
    let input = write_legacy_fixture(&dir, "field.man");
    let flat_validation = bin()
        .arg(&input)
        .arg("--validate")
        .output()
        .expect("flat validation should execute");
    assert_eq!(
        String::from_utf8(flat_validation.stderr).expect("error should be UTF-8"),
        "LANDUSE-MIGRATE-E-016: flat-source --validate requires --to <target>\n",
    );

    let migration = bin()
        .arg(&input)
        .output()
        .expect("missing-target migration should execute");
    assert_eq!(
        String::from_utf8(migration.stderr).expect("error should be UTF-8"),
        "LANDUSE-MIGRATE-E-016: missing --to <target>; use --validate for validation-only mode\n",
    );
}

#[test]
fn m13_parser_preserves_unknown_multiple_and_missing_input_priority() {
    let unknown = bin()
        .args(["input.man", "--unknown", "--help"])
        .output()
        .expect("unknown-option CLI should execute");
    assert_eq!(
        String::from_utf8(unknown.stderr).expect("error should be UTF-8"),
        "LANDUSE-MIGRATE-E-016: unrecognized argument --unknown\n",
    );

    let multiple = bin()
        .args(["first.man", "second.man"])
        .output()
        .expect("multiple-input CLI should execute");
    assert_eq!(
        String::from_utf8(multiple.stderr).expect("error should be UTF-8"),
        "LANDUSE-MIGRATE-E-016: multiple input paths supplied; unexpected second.man\n",
    );

    let missing = bin()
        .arg("--dry-run")
        .output()
        .expect("missing-input CLI should execute");
    assert_eq!(
        String::from_utf8(missing.stderr).expect("error should be UTF-8"),
        "LANDUSE-MIGRATE-E-016: missing input path\n",
    );
}

fn native_flat_fixture() -> String {
    LEGACY_CROPLAND_98_4
        .replacen("98.4", "ow-lanuse-1", 1)
        .replace("1  #landuse", "4  #landuse")
        .replace("1  # landuse  - cropland", "4  # landuse  - native cropland")
        .replace("1  # landuse <cropland>", "4  # landuse <native cropland>")
        .replace(
            "0.00000 3.50000 0.00000\n\n#######################\n# Operation Section",
            "0.00000 3.50000 0.00000\nrouting_coefficients\n480.0 0.25 0.010 0.050 0.12\n\n#######################\n# Operation Section",
        )
}

fn assert_close(observed: f64, expected: f64) {
    assert!(
        (observed - expected).abs() <= 1.0e-12,
        "observed {observed}, expected {expected}"
    );
}
