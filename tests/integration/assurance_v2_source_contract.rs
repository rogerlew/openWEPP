use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use openwepp_assurance::{AssuranceError, V2Repository, sha256_bytes};

const REPORT_ID: &str = "linear-groundwater-reservoir-recurrence";
const REPORT_PATH: &str =
    "assurance/v2/reports/linear-groundwater-reservoir-recurrence/report.yaml";
const CATALOG_PATH: &str = "assurance/v2/catalog.yaml";
const TWO_DAY_PATH: &str =
    "assurance/v2/reports/linear-groundwater-reservoir-recurrence/results/two-day-recurrence.json";
const CATALOG_SCHEMA_PATH: &str = "assurance/v2/schemas/catalog.schema.json";
const REPORT_SCHEMA_PATH: &str = "assurance/v2/schemas/report.schema.json";
const RESULT_SCHEMA_PATH: &str = "assurance/v2/schemas/result.schema.json";

#[test]
fn real_source_and_cli_validate_named_and_all_deterministically() {
    let root = repository_root();
    let repository = V2Repository::open(&root).expect("load v2 repository");
    let named = repository
        .validate_report(REPORT_ID)
        .expect("validate named source");
    let all = repository.validate_all().expect("validate all sources");
    assert_eq!(
        named, all,
        "the one-report fixture must share validation logic"
    );
    assert_eq!(all.selected_report_count, 1);
    assert_eq!(all.total_report_count, 1);
    assert_eq!(all.public_report_count, 0);
    assert_eq!(all.reports[0].id, REPORT_ID);
    assert_eq!(all.reports[0].version, "0.1.0");
    assert_eq!(all.reports[0].lifecycle, "DRAFT");
    assert!(all.reports[0].fixture_only);

    let rendered = all.render();
    assert!(rendered.contains("validation: PASS"));
    assert!(rendered.contains("v2_reports_selected: 1"));
    assert!(rendered.contains("public_reports: 0"));
    assert!(rendered.contains("source_root_sha256:"));
    assert_eq!(
        rendered,
        repository.validate_all().expect("repeat").render()
    );

    let cli_all = openwepp_assurance::cli::run(["openwepp-assurance", "validate", "--all"])
        .expect("validate all through real CLI");
    let cli_named =
        openwepp_assurance::cli::run(["openwepp-assurance", "validate", "--report", REPORT_ID])
            .expect("validate named source through real CLI");
    assert_eq!(cli_all, cli_named);
    assert_eq!(cli_all, rendered);
}

#[test]
fn real_sources_satisfy_the_declared_draft_2020_12_schemas() {
    let root = repository_root();
    let catalog_schema = json_value(&root.join(CATALOG_SCHEMA_PATH));
    let report_schema = json_value(&root.join(REPORT_SCHEMA_PATH));
    let result_schema = json_value(&root.join(RESULT_SCHEMA_PATH));

    assert_schema_accepts(
        &catalog_schema,
        &yaml_value(&root.join(CATALOG_PATH)),
        "catalog",
    );
    assert_schema_accepts(
        &report_schema,
        &yaml_value(&root.join(REPORT_PATH)),
        "report",
    );
    for path in [TWO_DAY_PATH, h2637_path()] {
        assert_schema_accepts(&result_schema, &json_value(&root.join(path)), path);
    }
}

#[test]
fn executable_schemas_reject_practical_identity_and_lifecycle_defects() {
    let root = repository_root();
    let catalog_schema = json_value(&root.join(CATALOG_SCHEMA_PATH));
    let report_schema = json_value(&root.join(REPORT_SCHEMA_PATH));
    let result_schema = json_value(&root.join(RESULT_SCHEMA_PATH));

    let mut catalog = yaml_value(&root.join(CATALOG_PATH));
    catalog["reports"][0]["version"] = serde_json::json!("01.0");
    assert_schema_rejects(&catalog_schema, &catalog, "non-semantic version");
    catalog["reports"][0]["version"] = serde_json::json!("0.1.0");
    catalog["reports"][0]["manifest_path"] = serde_json::json!("/tmp/report.yaml");
    assert_schema_rejects(&catalog_schema, &catalog, "absolute catalog path");

    let mut report = yaml_value(&root.join(REPORT_PATH));
    report["figures"][0]["kind"] = serde_json::json!("conceptual");
    assert_schema_rejects(
        &report_schema,
        &report,
        "conceptual figure with result binding",
    );
    report["figures"][0]["kind"] = serde_json::json!("result_bearing");
    report["dependencies"][0]["kind"] = serde_json::json!("restricted");
    assert_schema_rejects(
        &report_schema,
        &report,
        "restricted dependency with local path",
    );

    let mut result = json_value(&root.join(TWO_DAY_PATH));
    result["values"][0]["precision"] = serde_json::json!("");
    assert_schema_rejects(&result_schema, &result, "empty precision policy");
    result["values"][0]["precision"] = serde_json::json!("exact input");
    let duplicate = result["values"][0].clone();
    result["values"]
        .as_array_mut()
        .expect("result values")
        .push(duplicate);
    assert_schema_rejects(&result_schema, &result, "duplicate result value");
}

#[test]
fn report_specific_assembly_requires_an_explicit_staging_root() {
    for command in ["build", "check"] {
        let error =
            openwepp_assurance::cli::run(["openwepp-assurance", command, "--report", REPORT_ID])
                .expect_err("report-specific command must fail closed");
        assert!(matches!(error, AssuranceError::Usage(_)));
        assert!(error.to_string().contains("--staging-root"));
    }
    let named = openwepp_assurance::cli::run(["openwepp-assurance", "plan", "--report", REPORT_ID])
        .expect("ASSURE-04B enables named planning");
    assert!(named.contains(&format!("id={REPORT_ID}")));
    let plan = openwepp_assurance::cli::run(["openwepp-assurance", "plan", "--all"])
        .expect("v2 all plan retains zero-public boundary");
    assert!(plan.contains("publication_state: v1_retired_zero_reports"));
    assert!(plan.contains("public_reports: 0"));
}

#[test]
fn named_validation_isolated_from_an_unselected_broken_report() {
    let source = fixture("assure04a-named-selection");
    let broken_path = "assurance/v2/reports/broken/report.yaml";
    let broken_bytes = b"not: [valid\n";
    let path = source.path.join(broken_path);
    fs::create_dir_all(path.parent().expect("broken report parent"))
        .expect("create broken report parent");
    fs::write(&path, broken_bytes).expect("write broken report");
    let digest = sha256_bytes(broken_bytes);
    let catalog = source.path.join(CATALOG_PATH);
    let mut text = fs::read_to_string(&catalog).expect("read catalog");
    write!(
        text,
        "  - id: broken-report\n    version: 0.1.0\n    title: Broken unselected source\n    owner: test fixture\n    fixture_only: true\n    manifest_path: {broken_path}\n    manifest_sha256: {digest}\n"
    )
    .expect("extend catalog text");
    fs::write(catalog, text).expect("extend catalog");

    let repository = V2Repository::open(&source.path).expect("load two-report catalog");
    let named = repository
        .validate_report(REPORT_ID)
        .expect("validate selected complete source only");
    assert_eq!(named.selected_report_count, 1);
    assert_eq!(named.total_report_count, 2);
    assert!(repository.validate_all().is_err());
}

#[test]
fn unknown_missing_duplicate_unresolved_and_unused_fields_fail_closed() {
    let unknown = fixture("assure04a-unknown-field");
    mutate_report(
        &unknown.path,
        "schema_version: 2\n",
        "schema_version: 2\nunexpected_field: true\n",
    );
    assert_rejected(&unknown.path, "unknown field");

    let missing = fixture("assure04a-missing-field");
    mutate_report(
        &missing.path,
        "owner: openWEPP scientific assurance maintainers\nlifecycle: DRAFT\n",
        "lifecycle: DRAFT\n",
    );
    assert_rejected(&missing.path, "missing field");

    let duplicate = fixture("assure04a-duplicate-id");
    mutate_report(&duplicate.path, "  - id: GW-P05\n", "  - id: GW-P03\n");
    assert_rejected(&duplicate.path, "duplicate logical ID");

    let unresolved = fixture("assure04a-unresolved-id");
    mutate_report(
        &unresolved.path,
        "  claim_ids: [GW-P01, GW-P02, GW-P03, GW-P04, GW-P05, GW-P06, GW-P07, GW-P08, GW-P09]\n",
        "  claim_ids: [GW-P01, GW-P02, GW-P03, GW-P04, GW-P05, GW-P06, GW-P07, GW-P08, GW-MISSING]\n",
    );
    assert_rejected(&unresolved.path, "unknown claim ID");

    let unused = fixture("assure04a-unused-unit");
    mutate_report(
        &unused.path,
        "claims:\n",
        "  - id: unused_unit\n    symbol: unused\n    quantity: unused quantity\n    definition: deliberately unreachable\nclaims:\n",
    );
    assert_rejected(&unused.path, "unused unit ID");

    let unknown_unit = fixture("assure04a-unknown-unit");
    replace_in(
        &unknown_unit.path.join(TWO_DAY_PATH),
        "\"unit_id\": \"m3\"",
        "\"unit_id\": \"unregistered\"",
    );
    refresh_local_hash(&unknown_unit.path, TWO_DAY_PATH);
    refresh_report_hash(&unknown_unit.path);
    assert_rejected(&unknown_unit.path, "unknown unit ID");

    let wrong_family = fixture("assure04a-wrong-reference-family");
    mutate_report(
        &wrong_family.path,
        "    method_ids: [GW-METHOD-ANALYTICAL]\n",
        "    method_ids: [GW-DEP-SCIENCE-CONTRACT]\n",
    );
    assert_rejected(&wrong_family.path, "unknown method ID");
}

#[test]
fn schema_required_nullable_fields_cannot_be_omitted() {
    let cases = [
        (
            "authorship",
            "  human_report_lead: null\n",
            "",
            "human_report_lead",
        ),
        (
            "dependency",
            "    immutable_identity: null\n    restriction_reason: null\n",
            "    restriction_reason: null\n",
            "immutable_identity",
        ),
        (
            "research-object",
            "    sha256: e51dbd62c1316685b87007c665a24cd4af2bbe05237f33418e088e445bc67372\n    restriction_reason: null\n    review_role: null\n",
            "    sha256: e51dbd62c1316685b87007c665a24cd4af2bbe05237f33418e088e445bc67372\n    review_role: null\n",
            "restriction_reason",
        ),
        ("review", "  reviewed_root: null\n", "", "reviewed_root"),
        ("publication", "  public_path: null\n", "", "public_path"),
    ];

    for (family, old, new, missing_field) in cases {
        let source = fixture(&format!("assure04a-missing-nullable-{family}"));
        mutate_report(&source.path, old, new);
        assert_rejected(&source.path, missing_field);
    }
}

#[test]
fn content_schema_contract_and_report_versions_are_enforced() {
    let drift = fixture("assure04a-content-drift");
    let path = drift.path.join(TWO_DAY_PATH);
    fs::write(
        &path,
        format!("{}\n", fs::read_to_string(&path).expect("read result")),
    )
    .expect("drift result bytes");
    assert_rejected(&drift.path, "SHA-256 mismatch");

    let catalog_version = fixture("assure04a-catalog-version");
    replace_in(
        &catalog_version.path.join(CATALOG_PATH),
        "schema_version: 2",
        "schema_version: 1",
    );
    assert_rejected(&catalog_version.path, "catalog requires schema_version 2");

    let report_version = fixture("assure04a-report-version");
    mutate_report(
        &report_version.path,
        "contract_version: 2",
        "contract_version: 1",
    );
    assert_rejected(&report_version.path, "report requires schema_version 2");

    let semantic_version = fixture("assure04a-semantic-version");
    replace_in(
        &semantic_version.path.join(REPORT_PATH),
        "version: 0.1.0",
        "version: 00.1.0",
    );
    replace_in(
        &semantic_version.path.join(CATALOG_PATH),
        "version: 0.1.0",
        "version: 00.1.0",
    );
    refresh_report_hash(&semantic_version.path);
    assert_rejected(&semantic_version.path, "without leading zeros");

    let lexical_id = fixture("assure04a-lexical-id");
    replace_in(
        &lexical_id.path.join(REPORT_PATH),
        "id: linear-groundwater-reservoir-recurrence",
        "id: -leading-punctuation",
    );
    replace_in(
        &lexical_id.path.join(CATALOG_PATH),
        "id: linear-groundwater-reservoir-recurrence",
        "id: -leading-punctuation",
    );
    refresh_report_hash(&lexical_id.path);
    assert_rejected(&lexical_id.path, "must start with an ASCII letter or digit");

    let result_version = fixture("assure04a-result-version");
    replace_in(
        &result_version.path.join(TWO_DAY_PATH),
        "\"schema_version\": 1",
        "\"schema_version\": 2",
    );
    refresh_local_hash(&result_version.path, TWO_DAY_PATH);
    refresh_report_hash(&result_version.path);
    assert_rejected(&result_version.path, "result schema_version must be 1");

    let schema_drift = fixture("assure04a-schema-drift");
    let schema_path = "assurance/v2/schemas/result.schema.json";
    replace_in(
        &schema_drift.path.join(schema_path),
        "\"schema_version\", \"result_id\", \"values\"",
        "\"schema_version\", \"result_id\"",
    );
    refresh_catalog_hash(&schema_drift.path, schema_path);
    assert_rejected(&schema_drift.path, "required-field contract");

    let nested_schema_drift = fixture("assure04a-nested-schema-drift");
    let report_schema_path = "assurance/v2/schemas/report.schema.json";
    replace_in(
        &nested_schema_drift.path.join(report_schema_path),
        "\"required\": [\"id\", \"symbol\", \"quantity\", \"definition\"]",
        "\"required\": [\"id\", \"symbol\", \"quantity\", \"meaning\"]",
    );
    replace_in(
        &nested_schema_drift.path.join(report_schema_path),
        "\"definition\": { \"$ref\": \"#/$defs/text\" }",
        "\"meaning\": { \"$ref\": \"#/$defs/text\" }",
    );
    refresh_catalog_hash(&nested_schema_drift.path, report_schema_path);
    assert_rejected(&nested_schema_drift.path, "executable typed contract");

    let schema_constant_drift = fixture("assure04a-schema-constant-drift");
    replace_in(
        &schema_constant_drift.path.join(report_schema_path),
        "\"contract_version\": { \"const\": 2 }",
        "\"contract_version\": { \"const\": 1 }",
    );
    refresh_catalog_hash(&schema_constant_drift.path, report_schema_path);
    assert_rejected(
        &schema_constant_drift.path,
        "schema property 'contract_version' const",
    );
}

#[test]
fn paths_symlinks_and_special_entries_fail_closed() {
    let absolute = fixture("assure04a-absolute-path");
    mutate_report(
        &absolute.path,
        "  path: assurance/v2/reports/linear-groundwater-reservoir-recurrence/manuscript.md\n",
        "  path: /tmp/manuscript.md\n",
    );
    assert_rejected(&absolute.path, "confined relative path");

    let traversal = fixture("assure04a-traversal-path");
    mutate_report(
        &traversal.path,
        "  path: assurance/v2/reports/linear-groundwater-reservoir-recurrence/manuscript.md\n",
        "  path: ../manuscript.md\n",
    );
    assert_rejected(&traversal.path, "confined relative path");

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        use std::os::unix::net::UnixListener;

        let linked = fixture("assure04a-result-symlink");
        let result = linked.path.join(TWO_DAY_PATH);
        let outside = linked.path.join("outside-result.json");
        fs::rename(&result, &outside).expect("move result outside declared route");
        symlink(&outside, &result).expect("create result symlink");
        assert_rejected(&linked.path, "symlink");

        let special = fixture("assure04a-result-special");
        mutate_report(
            &special.path,
            &format!("    path: {TWO_DAY_PATH}\n"),
            "    path: special.sock\n",
        );
        let result = special.path.join("special.sock");
        let listener = UnixListener::bind(&result).expect("bind result socket");
        assert_rejected(&special.path, "regular file");
        drop(listener);
    }
}

#[test]
fn restricted_evidence_and_draft_lifecycle_contradictions_fail_closed() {
    let restricted = fixture("assure04a-restricted-leak");
    mutate_report(
        &restricted.path,
        "    kind: local_content\n    provenance: Assessed Rust implementation path.",
        "    kind: restricted\n    provenance: Assessed Rust implementation path.",
    );
    assert_rejected(&restricted.path, "restricted dependency");

    let accountability = fixture("assure04a-accountability-contradiction");
    mutate_report(
        &accountability.path,
        "  human_report_lead: null",
        "  human_report_lead: Unapproved Person",
    );
    assert_rejected(&accountability.path, "human_report_lead must be null");

    let agent_provenance = fixture("assure04a-agent-provenance-contradiction");
    mutate_report(
        &agent_provenance.path,
        "  provenance_complete: false",
        "  provenance_complete: true",
    );
    assert_rejected(
        &agent_provenance.path,
        "agent provenance must block review entry",
    );

    let review = fixture("assure04a-review-contradiction");
    mutate_report(
        &review.path,
        "  decision: not_started",
        "  decision: approved",
    );
    assert_rejected(&review.path, "draft review");

    let publication = fixture("assure04a-publication-contradiction");
    mutate_report(
        &publication.path,
        "  export_authorized: false",
        "  export_authorized: true",
    );
    assert_rejected(&publication.path, "draft publication");
}

#[test]
fn every_record_family_has_executable_field_consumption() {
    let cases = [
        (
            "content",
            "  media_type: text/markdown",
            "  media_type: text/plain",
            "media_type",
        ),
        (
            "dependency",
            "    license: repository_license\n    path: docs/specifications",
            "    license: ''\n    path: docs/specifications",
            "license",
        ),
        (
            "unit",
            "    definition: cubic metre",
            "    definition: ''",
            "definition",
        ),
        (
            "claim",
            "    statement: The assessed implementation",
            "    statement: '' # The assessed implementation",
            "statement",
        ),
        (
            "method",
            "    procedure: Compute expected values independently",
            "    procedure: '' # Compute expected values independently",
            "procedure",
        ),
        (
            "result",
            "    precision_policy: Preserve binary64 residual",
            "    precision_policy: '' # Preserve binary64 residual",
            "precision_policy",
        ),
        (
            "value-binding",
            "    transform: identity\n    display: integer",
            "    transform: unsupported\n    display: integer",
            "transform",
        ),
        (
            "table",
            "    row_header: Day",
            "    row_header: '' # Day",
            "row_header",
        ),
        (
            "figure",
            "    alternative_text: Tabular values",
            "    alternative_text: '' # Tabular values",
            "alternative_text",
        ),
        (
            "reference",
            "    immutable_identity: sha256:97ee",
            "    immutable_identity: '' # sha256:97ee",
            "immutable_identity",
        ),
        (
            "research-object",
            "    reproduction_instructions: Reapply",
            "    reproduction_instructions: '' # Reapply",
            "reproduction_instructions",
        ),
    ];
    for (label, old, new, expected) in cases {
        let source = fixture(&format!("assure04a-consume-{label}"));
        mutate_report(&source.path, old, new);
        assert_rejected(&source.path, expected);
    }
}

#[test]
fn protected_public_surface_remains_byte_identical() {
    let root = repository_root();
    let expected = [
        (
            "assurance/catalog.yaml",
            "cb9cb601739b64f8cb497c0999ca268859946f2ce7e57532de8c08b9ed30801f",
        ),
        (
            "assurance/templates/catalog.md",
            "65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70",
        ),
        (
            "assurance/generated/wepppy-usersum.yaml",
            "08b21cbe20ed059eb61f01f9965d8603779501bde90a728bc7a6c138a69258eb",
        ),
        (
            "usersum/assurance/README.md",
            "65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70",
        ),
    ];
    for (path, digest) in expected {
        assert_eq!(
            sha256_bytes(&fs::read(root.join(path)).expect("read protected file")),
            digest,
            "protected bytes changed at {path}"
        );
    }
}

fn assert_rejected(root: &Path, expected: &str) {
    let error = match V2Repository::open(root) {
        Ok(repository) => repository
            .validate_all()
            .expect_err("fixture source must fail closed"),
        Err(error) => error,
    };
    assert!(
        error.to_string().contains(expected),
        "expected '{expected}', observed '{error}'"
    );
}

fn mutate_report(root: &Path, old: &str, new: &str) {
    replace_in(&root.join(REPORT_PATH), old, new);
    refresh_report_hash(root);
}

fn refresh_report_hash(root: &Path) {
    let digest = sha256_bytes(&fs::read(root.join(REPORT_PATH)).expect("read report"));
    replace_prefixed_value(&root.join(CATALOG_PATH), "    manifest_sha256: ", &digest);
}

fn refresh_local_hash(root: &Path, relative: &str) {
    let digest = sha256_bytes(&fs::read(root.join(relative)).expect("read local source"));
    let report = root.join(REPORT_PATH);
    let text = fs::read_to_string(&report).expect("read report manifest");
    let path_line = format!("    path: {relative}\n");
    let start = text.find(&path_line).expect("find source path") + path_line.len();
    let tail = &text[start..];
    let hash_start = tail.find("    sha256: ").expect("find source hash") + start;
    let hash_end = text[hash_start..].find('\n').expect("hash line end") + hash_start;
    let mut updated = text;
    updated.replace_range(hash_start..hash_end, &format!("    sha256: {digest}"));
    fs::write(report, updated).expect("write refreshed source hash");
}

fn refresh_catalog_hash(root: &Path, relative: &str) {
    let digest = sha256_bytes(&fs::read(root.join(relative)).expect("read schema"));
    let catalog = root.join(CATALOG_PATH);
    let text = fs::read_to_string(&catalog).expect("read catalog");
    let path_line = format!("    path: {relative}\n");
    let start = text.find(&path_line).expect("find schema path") + path_line.len();
    let tail = &text[start..];
    let hash_start = tail.find("    sha256: ").expect("find schema hash") + start;
    let hash_end = text[hash_start..].find('\n').expect("hash line end") + hash_start;
    let mut updated = text;
    updated.replace_range(hash_start..hash_end, &format!("    sha256: {digest}"));
    fs::write(catalog, updated).expect("write refreshed schema hash");
}

fn replace_prefixed_value(path: &Path, prefix: &str, replacement: &str) {
    let text = fs::read_to_string(path).expect("read replacement target");
    let start = text.find(prefix).expect("find prefixed value") + prefix.len();
    let end = text[start..].find('\n').expect("value line end") + start;
    let mut updated = text;
    updated.replace_range(start..end, replacement);
    fs::write(path, updated).expect("write replacement target");
}

fn replace_in(path: &Path, old: &str, new: &str) {
    let text = fs::read_to_string(path).expect("read replacement target");
    assert!(text.contains(old), "replacement source missing: {old}");
    fs::write(path, text.replacen(old, new, 1)).expect("write replacement target");
}

fn fixture(label: &str) -> Scratch {
    let source = repository_root();
    let target = Scratch::new(label);
    copy_tree(
        &source.join("assurance/v2"),
        &target.path.join("assurance/v2"),
    );
    for relative in [
        "assurance/catalog.yaml",
        "assurance/templates/catalog.md",
        "assurance/generated/wepppy-usersum.yaml",
        "usersum/assurance/README.md",
        "docs/specifications/science-contracts/contracts/SC-GWBASEFLOW-001.md",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/groundwater.rs",
        "docs/work-packages/20260714-assure02-manuscript-first-assurance-architecture-001/artifacts/groundwater-current-tree-confirmation.md",
        "docs/work-packages/20260714-assure02-manuscript-first-assurance-architecture-001/artifacts/prototype-linear-groundwater-reservoir-evaluation.md",
        "docs/work-packages/20260709-laned-active-baseflow-export-closure-001/artifacts/consumer-path-proof.md",
        "docs/work-packages/20260708-groundwater-baseflow-laned-single-ofe-mofe-implementation-001/artifacts/consumer-path-proof.md",
        "docs/work-packages/20260713-integrated-validation-campaign-001/artifacts/final-conservation-and-consumer-evidence.md",
        "docs/work-packages/20260713-integrated-validation-campaign-001/artifacts/logs/final-reconstruction-arithmetic.log",
    ] {
        copy_file(&source, &target.path, relative);
    }
    target
}

fn h2637_path() -> &'static str {
    "assurance/v2/reports/linear-groundwater-reservoir-recurrence/results/h2637-ledger.json"
}

fn json_value(path: &Path) -> serde_json::Value {
    serde_json::from_slice(&fs::read(path).expect("read JSON source")).expect("parse JSON source")
}

fn yaml_value(path: &Path) -> serde_json::Value {
    let yaml: serde_yaml::Value =
        serde_yaml::from_slice(&fs::read(path).expect("read YAML source"))
            .expect("parse YAML source");
    serde_json::to_value(yaml).expect("convert YAML to JSON value")
}

fn assert_schema_accepts(schema: &serde_json::Value, instance: &serde_json::Value, label: &str) {
    let validator = jsonschema::draft202012::new(schema).expect("compile Draft 2020-12 schema");
    assert!(
        validator.is_valid(instance),
        "schema rejected valid {label}"
    );
}

fn assert_schema_rejects(schema: &serde_json::Value, instance: &serde_json::Value, label: &str) {
    let validator = jsonschema::draft202012::new(schema).expect("compile Draft 2020-12 schema");
    assert!(!validator.is_valid(instance), "schema accepted {label}");
}

fn copy_tree(source: &Path, target: &Path) {
    fs::create_dir_all(target).expect("create fixture tree");
    for entry in fs::read_dir(source).expect("read fixture source tree") {
        let entry = entry.expect("read fixture entry");
        let destination = target.join(entry.file_name());
        if entry.file_type().expect("fixture file type").is_dir() {
            copy_tree(&entry.path(), &destination);
        } else {
            fs::copy(entry.path(), destination).expect("copy fixture entry");
        }
    }
}

fn copy_file(source_root: &Path, target_root: &Path, relative: &str) {
    let target = target_root.join(relative);
    fs::create_dir_all(target.parent().expect("fixture parent")).expect("create fixture parent");
    fs::copy(source_root.join(relative), target).expect("copy fixture file");
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

struct Scratch {
    path: PathBuf,
}

impl Scratch {
    fn new(label: &str) -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let counter = COUNTER.fetch_add(1, Ordering::Relaxed);
        let path =
            std::env::temp_dir().join(format!("openwepp-{label}-{}-{counter}", std::process::id()));
        if path.exists() {
            fs::remove_dir_all(&path).expect("remove stale scratch directory");
        }
        fs::create_dir_all(&path).expect("create scratch directory");
        Self { path }
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        if self.path.exists() {
            fs::remove_dir_all(&self.path).expect("remove scratch directory");
        }
    }
}
