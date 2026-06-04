use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn unique_temp_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("openwepp_{name}_{nanos}"))
}

fn run_lint(path: &Path) -> std::process::Output {
    Command::new("python3")
        .arg(repo_root().join("tools/release/check_sc_unit_compliance.py"))
        .arg("--path")
        .arg(path)
        .output()
        .expect("SC unit compliance lint should execute")
}

fn run_lint_with_registry(path: &Path, registry_source: &Path) -> std::process::Output {
    Command::new("python3")
        .arg(repo_root().join("tools/release/check_sc_unit_compliance.py"))
        .arg("--path")
        .arg(path)
        .arg("--registry-source")
        .arg(registry_source)
        .output()
        .expect("SC unit compliance lint should execute")
}

#[test]
fn hphys0279_sc_unit_lint_accepts_compliant_contract_fixture() {
    let dir = unique_temp_dir("hphys0279_good");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let contract = dir.join("SC-TEST-001.md");
    fs::write(
        &contract,
        r"# SC-TEST-001 Unit Fixture

## Variables and Units

| Symbol | Units | Meaning | Producer | Consumer |
| --- | --- | --- | --- | --- |
| `foo_depth` | `mm` | test depth | fixture | fixture |

## Symbol Alias Map

| Canonical symbol | Boundary/API name | Scope | Units check | Evidence |
| --- | --- | --- | --- | --- |
| `foo_depth` | `fixture.foo_depth` | runtime fixture | `mm` preserved | `[INFERENCE][Static]` |
",
    )
    .expect("fixture should be written");

    let output = run_lint(&contract);
    assert!(
        output.status.success(),
        "lint should accept compliant fixture; stdout={}, stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    fs::remove_dir_all(&dir).expect("temp dir should be removed");
}

#[test]
fn hphys0279_sc_unit_lint_rejects_missing_variables_and_alias_sections() {
    let dir = unique_temp_dir("hphys0279_missing_sections");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let contract = dir.join("SC-BAD-001.md");
    fs::write(&contract, "# SC-BAD-001 Missing Unit Sections\n")
        .expect("fixture should be written");

    let output = run_lint(&contract);
    assert!(
        !output.status.success(),
        "lint should reject missing sections; stdout={}, stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("SCUNIT-E-001"), "stderr={stderr}");
    assert!(stderr.contains("SCUNIT-E-005"), "stderr={stderr}");

    fs::remove_dir_all(&dir).expect("temp dir should be removed");
}

#[test]
fn hphys0279_sc_unit_lint_rejects_missing_alias_units_check() {
    let dir = unique_temp_dir("hphys0279_bad_alias");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let contract = dir.join("SC-BADALIAS-001.md");
    fs::write(
        &contract,
        r"# SC-BADALIAS-001 Bad Alias Fixture

## Variables and Units

| Symbol | Units | Meaning |
| --- | --- | --- |
| `foo_depth` | `mm` | test depth |

## Symbol Alias Map

| Canonical symbol | Boundary/API name | Scope | Units check |
| --- | --- | --- | --- |
| `foo_depth` | `fixture.foo_depth` | runtime fixture | TBD |
",
    )
    .expect("fixture should be written");

    let output = run_lint(&contract);
    assert!(
        !output.status.success(),
        "lint should reject placeholder alias units check; stdout={}, stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("SCUNIT-E-007"),
        "stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );

    fs::remove_dir_all(&dir).expect("temp dir should be removed");
}

#[test]
fn hphys0279_sc_unit_lint_cross_checks_registry_units() {
    let dir = unique_temp_dir("hphys0279_registry");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let registry = dir.join("units.rs");
    fs::write(
        &registry,
        r#"fn fixture_entries() {
    BoundaryUnitEntry::new(
        "foo_depth",
        &["fixture.foo_depth"],
        "m",
        Depth,
        NonNegativeFinite,
        "fixture-producer",
        "fixture-consumer",
        "SC-TEST-001",
        "SC-TEST-001#INV-TEST-001",
        TypedRequired,
        None,
        &[],
    );
}
"#,
    )
    .expect("registry fixture should be written");
    let contract = dir.join("SC-TEST-001.md");
    fs::write(
        &contract,
        r"# SC-TEST-001 Registry Fixture

## Variables and Units

| Symbol | Units | Meaning |
| --- | --- | --- |
| `foo_depth` | `mm` | test depth |

## Symbol Alias Map

| Canonical symbol | Boundary/API name | Scope | Units check |
| --- | --- | --- | --- |
| `foo_depth` | `fixture.foo_depth` | runtime fixture | `mm` preserved |
",
    )
    .expect("fixture should be written");

    let output = run_lint_with_registry(&contract, &registry);
    assert!(
        !output.status.success(),
        "lint should reject registry unit mismatch; stdout={}, stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("SCUNIT-E-004"), "stderr={stderr}");
    assert!(stderr.contains("registry requires 'm'"), "stderr={stderr}");

    fs::remove_dir_all(&dir).expect("temp dir should be removed");
}

#[test]
fn hphys0279_sc_unit_lint_fails_closed_when_registry_source_is_missing() {
    let dir = unique_temp_dir("hphys0279_missing_registry");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let contract = dir.join("SC-TEST-001.md");
    fs::write(
        &contract,
        r"# SC-TEST-001 Unit Fixture

## Variables and Units

| Symbol | Units | Meaning |
| --- | --- | --- |
| `foo_depth` | `m` | test depth |

## Symbol Alias Map

| Canonical symbol | Boundary/API name | Scope | Units check |
| --- | --- | --- | --- |
| `foo_depth` | `fixture.foo_depth` | runtime fixture | `m` preserved |
",
    )
    .expect("fixture should be written");

    let output = run_lint_with_registry(&contract, &dir.join("missing_units.rs"));
    assert!(
        !output.status.success(),
        "lint should fail closed when registry source is missing; stdout={}, stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("SCUNIT-E-010"),
        "stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );

    fs::remove_dir_all(&dir).expect("temp dir should be removed");
}

#[test]
fn hphys0279_sc_unit_lint_fails_closed_when_registry_source_is_unparseable() {
    let dir = unique_temp_dir("hphys0279_unparseable_registry");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let registry = dir.join("units.rs");
    fs::write(&registry, "pub fn no_boundary_entries_here() {}\n")
        .expect("registry fixture should be written");
    let contract = dir.join("SC-TEST-001.md");
    fs::write(
        &contract,
        r"# SC-TEST-001 Unit Fixture

## Variables and Units

| Symbol | Units | Meaning |
| --- | --- | --- |
| `foo_depth` | `m` | test depth |

## Symbol Alias Map

| Canonical symbol | Boundary/API name | Scope | Units check |
| --- | --- | --- | --- |
| `foo_depth` | `fixture.foo_depth` | runtime fixture | `m` preserved |
",
    )
    .expect("fixture should be written");

    let output = run_lint_with_registry(&contract, &registry);
    assert!(
        !output.status.success(),
        "lint should fail closed when registry source is unparseable; stdout={}, stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("SCUNIT-E-010"), "stderr={stderr}");
    assert!(
        stderr.contains("yielded no parseable entries"),
        "stderr={stderr}"
    );

    fs::remove_dir_all(&dir).expect("temp dir should be removed");
}

#[test]
fn hphys0279_sc_unit_lint_requires_registered_alias_rows() {
    let dir = unique_temp_dir("hphys0279_missing_alias_row");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let registry = dir.join("units.rs");
    fs::write(
        &registry,
        r#"fn fixture_entries() {
    BoundaryUnitEntry::new(
        "foo_depth",
        &["fixture.foo_depth"],
        "m",
        Depth,
        NonNegativeFinite,
        "fixture-producer",
        "fixture-consumer",
        "SC-TEST-001",
        "SC-TEST-001#INV-TEST-001",
        TypedRequired,
        None,
        &["fixture_publication.foo_depth:m"],
    );
}
"#,
    )
    .expect("registry fixture should be written");
    let contract = dir.join("SC-TEST-001.md");
    fs::write(
        &contract,
        r"# SC-TEST-001 Missing Alias Row Fixture

## Variables and Units

| Symbol | Units | Meaning |
| --- | --- | --- |
| `foo_depth` | `m` | test depth |

## Symbol Alias Map

| Canonical symbol | Boundary/API name | Scope | Units check |
| --- | --- | --- | --- |
| `foo_depth` | `foo_depth` | runtime fixture | `m` preserved |
",
    )
    .expect("fixture should be written");

    let output = run_lint_with_registry(&contract, &registry);
    assert!(
        !output.status.success(),
        "lint should reject missing registered alias rows; stdout={}, stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("SCUNIT-E-011"), "stderr={stderr}");
    assert!(stderr.contains("fixture.foo_depth"), "stderr={stderr}");
    assert!(
        stderr.contains("fixture_publication.foo_depth:m"),
        "stderr={stderr}"
    );

    fs::remove_dir_all(&dir).expect("temp dir should be removed");
}

#[test]
fn hphys0279_sc_unit_lint_requires_canonical_variables_coverage() {
    let dir = unique_temp_dir("hphys0279_alias_only_variables");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let registry = dir.join("units.rs");
    fs::write(
        &registry,
        r#"fn fixture_entries() {
    BoundaryUnitEntry::new(
        "foo_depth",
        &["fixture.foo_depth"],
        "m",
        Depth,
        NonNegativeFinite,
        "fixture-producer",
        "fixture-consumer",
        "SC-TEST-001",
        "SC-TEST-001#INV-TEST-001",
        TypedRequired,
        None,
        &[],
    );
}
"#,
    )
    .expect("registry fixture should be written");
    let contract = dir.join("SC-TEST-001.md");
    fs::write(
        &contract,
        r"# SC-TEST-001 Alias Only Variables Fixture

## Variables and Units

| Symbol | Units | Meaning |
| --- | --- | --- |
| `fixture.foo_depth` | `m` | test depth |

## Symbol Alias Map

| Canonical symbol | Boundary/API name | Scope | Units check |
| --- | --- | --- | --- |
| `foo_depth` | `fixture.foo_depth` | runtime fixture | `m` preserved |
",
    )
    .expect("fixture should be written");

    let output = run_lint_with_registry(&contract, &registry);
    assert!(
        !output.status.success(),
        "lint should reject alias-only Variables coverage; stdout={}, stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("SCUNIT-E-012"),
        "stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );

    fs::remove_dir_all(&dir).expect("temp dir should be removed");
}

#[test]
fn hphys0279_sc_unit_lint_reports_empty_variables_table_against_registry() {
    let dir = unique_temp_dir("hphys0279_empty_variables");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let registry = dir.join("units.rs");
    fs::write(
        &registry,
        r#"fn fixture_entries() {
    BoundaryUnitEntry::new(
        "foo_depth",
        &["fixture.foo_depth"],
        "m",
        Depth,
        NonNegativeFinite,
        "fixture-producer",
        "fixture-consumer",
        "SC-TEST-001",
        "SC-TEST-001#INV-TEST-001",
        TypedRequired,
        None,
        &[],
    );
}
"#,
    )
    .expect("registry fixture should be written");
    let contract = dir.join("SC-TEST-001.md");
    fs::write(
        &contract,
        r"# SC-TEST-001 Empty Variables Fixture

## Variables and Units

| Symbol | Units | Meaning |
| --- | --- | --- |

## Symbol Alias Map

| Canonical symbol | Boundary/API name | Scope | Units check |
| --- | --- | --- | --- |
| `foo_depth` | `fixture.foo_depth` | runtime fixture | `m` preserved |
",
    )
    .expect("fixture should be written");

    let output = run_lint_with_registry(&contract, &registry);
    assert!(
        !output.status.success(),
        "lint should reject empty Variables coverage for registered symbols; stdout={}, stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("SCUNIT-E-009"),
        "stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );

    fs::remove_dir_all(&dir).expect("temp dir should be removed");
}
