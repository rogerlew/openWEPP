use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::error::{AssuranceError, Result};
use crate::hash::sha256_bytes;

mod confined;
mod planner;

pub use planner::{V2Plan, V2PlanNode, V2PlanState, V2ReportPlan};

pub(crate) use confined::read_regular_confined;
use confined::validate_relative;

const V2_CATALOG_PATH: &str = "assurance/v2/catalog.yaml";
const SCHEMA_VERSION: u32 = 1;
const CONTRACT_VERSION: u32 = 1;
const SOURCE_STATE: &str = "internal_draft_sources";
const DRAFT: &str = "DRAFT";

const CATALOG_FIELDS: &[&str] = &[
    "schema_version",
    "contract_version",
    "source_state",
    "schemas",
    "reports",
];
const REPORT_FIELDS: &[&str] = &[
    "schema_version",
    "contract_version",
    "id",
    "version",
    "title",
    "owner",
    "lifecycle",
    "fixture_only",
    "authorship",
    "agent_assistance",
    "manuscript",
    "supplement",
    "dependencies",
    "units",
    "claims",
    "methods",
    "results",
    "figures",
    "references",
    "research_objects",
    "review",
    "publication",
];
const RESULT_FIELDS: &[&str] = &["schema_version", "result_id", "values"];
const CATALOG_SCHEMA_SOURCE_FIELDS: &[&str] = &["id", "path", "sha256"];
const CATALOG_REPORT_SOURCE_FIELDS: &[&str] = &[
    "id",
    "version",
    "title",
    "owner",
    "fixture_only",
    "manifest_path",
    "manifest_sha256",
];
const CONTENT_SOURCE_FIELDS: &[&str] = &[
    "id",
    "title",
    "owner",
    "path",
    "sha256",
    "media_type",
    "provenance",
    "creation_procedure",
    "claim_ids",
    "method_ids",
    "result_ids",
    "figure_ids",
    "reference_ids",
    "research_object_ids",
];
const DEPENDENCY_FIELDS: &[&str] = &[
    "id",
    "title",
    "owner",
    "kind",
    "provenance",
    "creation_procedure",
    "access",
    "license",
    "path",
    "sha256",
    "immutable_identity",
    "restriction_reason",
    "review_role",
];
const UNIT_FIELDS: &[&str] = &["id", "symbol", "quantity", "definition"];
const CLAIM_FIELDS: &[&str] = &[
    "id",
    "title",
    "owner",
    "statement",
    "claim_type",
    "scope_limit",
    "method_ids",
    "result_ids",
    "dependency_ids",
    "unit_ids",
    "reference_ids",
];
const METHOD_FIELDS: &[&str] = &[
    "id",
    "title",
    "owner",
    "description",
    "procedure",
    "dependency_ids",
    "unit_ids",
];
const RESULT_SOURCE_FIELDS: &[&str] = &[
    "id",
    "title",
    "owner",
    "path",
    "sha256",
    "media_type",
    "method_id",
    "dependency_ids",
    "unit_ids",
    "quantity_semantics",
    "precision_policy",
    "software_realization",
    "provenance",
    "creation_procedure",
];
const FIGURE_FIELDS: &[&str] = &[
    "id",
    "title",
    "owner",
    "kind",
    "result_ids",
    "generation_procedure",
    "alternative_text",
    "caption",
];
const REFERENCE_FIELDS: &[&str] = &[
    "id",
    "title",
    "owner",
    "citation",
    "immutable_identity",
    "access",
    "license",
    "dependency_id",
];
const RESEARCH_OBJECT_FIELDS: &[&str] = &[
    "id",
    "title",
    "owner",
    "access",
    "license",
    "path",
    "sha256",
    "restriction_reason",
    "review_role",
    "result_ids",
    "method_ids",
    "dependency_ids",
    "reproduction_instructions",
];
const REVIEW_FIELDS: &[&str] = &[
    "id",
    "title",
    "owner",
    "state",
    "decision",
    "reviewed_root",
    "approvers",
    "independence_assessment",
];
const PUBLICATION_FIELDS: &[&str] = &[
    "id",
    "title",
    "owner",
    "state",
    "public_path",
    "snapshot_id",
    "release_id",
    "export_authorized",
    "vendoring_authorized",
    "supersedes",
    "withdrawn",
];
const RESULT_VALUE_FIELDS: &[&str] = &["id", "value", "unit_id", "precision"];
const AUTHORSHIP_FIELDS: &[&str] = &[
    "id",
    "title",
    "owner",
    "draft_authors",
    "human_report_lead",
    "scientific_approver",
    "accountability_state",
    "external_peer_review_claimed",
];
const AGENT_ASSISTANCE_FIELDS: &[&str] = &[
    "id",
    "title",
    "owner",
    "procedure_version",
    "objective",
    "tool_model_identity",
    "input_dependency_ids",
    "exact_output_dependency_id",
    "human_disposition",
    "known_nondeterminism",
    "limitations",
    "independent_review",
    "provenance_complete",
    "review_entry_authorized",
];
const CATALOG_SCHEMA_DEFINITIONS: &[(&str, &[&str])] = &[
    ("schemaSource", CATALOG_SCHEMA_SOURCE_FIELDS),
    ("reportSource", CATALOG_REPORT_SOURCE_FIELDS),
];
const REPORT_SCHEMA_DEFINITIONS: &[(&str, &[&str])] = &[
    ("authorship", AUTHORSHIP_FIELDS),
    ("agentAssistance", AGENT_ASSISTANCE_FIELDS),
    ("contentSource", CONTENT_SOURCE_FIELDS),
    ("dependency", DEPENDENCY_FIELDS),
    ("unit", UNIT_FIELDS),
    ("claim", CLAIM_FIELDS),
    ("method", METHOD_FIELDS),
    ("result", RESULT_SOURCE_FIELDS),
    ("figure", FIGURE_FIELDS),
    ("reference", REFERENCE_FIELDS),
    ("researchObject", RESEARCH_OBJECT_FIELDS),
    ("review", REVIEW_FIELDS),
    ("publication", PUBLICATION_FIELDS),
];

/// Deterministic validation result for one or all admitted v2 sources.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct V2ValidationSummary {
    pub selected_report_count: usize,
    pub total_report_count: usize,
    pub public_report_count: usize,
    pub source_root_sha256: String,
    pub reports: Vec<V2ReportSummary>,
}

impl V2ValidationSummary {
    #[must_use]
    pub fn render(&self) -> String {
        use std::fmt::Write as _;

        let mut output = format!(
            "validation: PASS\npublic_reports: {}\nv2_reports_total: {}\n\
             v2_reports_selected: {}\nsource_root_sha256: {}\nreports:\n",
            self.public_report_count,
            self.total_report_count,
            self.selected_report_count,
            self.source_root_sha256
        );
        for report in &self.reports {
            let _ = writeln!(
                output,
                "  - id={} version={} lifecycle={} fixture_only={} source_root_sha256={}",
                report.id,
                report.version,
                report.lifecycle,
                report.fixture_only,
                report.source_root_sha256
            );
        }
        output
    }
}

/// Public, immutable identity summary for one admitted report source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct V2ReportSummary {
    pub id: String,
    pub version: String,
    pub lifecycle: String,
    pub fixture_only: bool,
    pub source_root_sha256: String,
}

#[derive(Debug)]
pub struct V2Repository {
    root: PathBuf,
    inputs: BTreeMap<PathBuf, String>,
    sources: BTreeMap<String, ReportSource>,
}

impl V2Repository {
    /// Loads and validates the v2 catalog and its schema registry.
    ///
    /// # Errors
    ///
    /// Returns typed catalog, schema, identity, or confinement errors. Report
    /// sources are traversed by `validate_report` or `validate_all`.
    pub fn open(root: impl AsRef<Path>) -> Result<Self> {
        let root = root
            .as_ref()
            .canonicalize()
            .map_err(|error| AssuranceError::io(".", error))?;
        let mut inputs = BTreeMap::new();
        let catalog_bytes = read_identified(&root, Path::new(V2_CATALOG_PATH), None, &mut inputs)?;
        let catalog: V2Catalog = parse_yaml(V2_CATALOG_PATH, &catalog_bytes)?;
        validate_catalog_header(&catalog)?;
        validate_schemas(&root, &catalog.schemas, &mut inputs)?;

        let mut sources = BTreeMap::new();
        let mut catalog_ids = BTreeSet::new();
        let mut manifest_paths = BTreeSet::new();
        for source in &catalog.reports {
            validate_report_source(source)?;
            require_unique(&mut catalog_ids, &source.id, "catalog report")?;
            require_unique_path(
                &mut manifest_paths,
                &source.manifest_path,
                "report manifest",
            )?;
            sources.insert(source.id.clone(), source.clone());
        }
        if sources.is_empty() {
            return Err(AssuranceError::Invalid(
                "v2 catalog must admit at least one internal source".to_owned(),
            ));
        }
        Ok(Self {
            root,
            inputs,
            sources,
        })
    }

    /// Revalidates input identities and summarizes every admitted v2 source.
    ///
    /// # Errors
    ///
    /// Returns drift when any source byte changed after loading.
    pub fn validate_all(&self) -> Result<V2ValidationSummary> {
        self.validate_sources(self.sources.values())
    }

    /// Revalidates input identities and summarizes one named v2 source.
    ///
    /// # Errors
    ///
    /// Returns an error for source drift or an unknown report identity.
    pub fn validate_report(&self, report_id: &str) -> Result<V2ValidationSummary> {
        let source = self.sources.get(report_id).ok_or_else(|| {
            AssuranceError::Invalid(format!("unknown v2 report ID '{report_id}'"))
        })?;
        self.validate_sources(std::iter::once(source))
    }

    /// Plans every admitted v2 report in stable report-ID order.
    ///
    /// # Errors
    ///
    /// Returns a typed error for an invalid graph or source contract. Missing
    /// declared local content is represented as a blocked plan node.
    pub fn plan_all(&self) -> Result<V2Plan> {
        self.verify_inputs()?;
        let sources = self.sources.values().collect::<Vec<_>>();
        planner::plan_sources(&self.root, &self.inputs, self.sources.len(), &sources)
    }

    /// Plans one named v2 report without traversing unselected reports.
    ///
    /// # Errors
    ///
    /// Returns a typed error for an unknown report or invalid selected graph.
    /// Missing declared local content is represented as a blocked plan node.
    pub fn plan_report(&self, report_id: &str) -> Result<V2Plan> {
        self.verify_inputs()?;
        let source = self.sources.get(report_id).ok_or_else(|| {
            AssuranceError::Invalid(format!("unknown v2 report ID '{report_id}'"))
        })?;
        planner::plan_sources(&self.root, &self.inputs, self.sources.len(), &[source])
    }

    fn verify_inputs(&self) -> Result<()> {
        for (path, expected) in &self.inputs {
            let observed = sha256_bytes(&read_regular_confined(&self.root, path)?);
            if observed != *expected {
                return Err(AssuranceError::Drift(format!(
                    "v2 assurance input changed after open: {}",
                    path.display()
                )));
            }
        }
        Ok(())
    }

    fn validate_sources<'a>(
        &self,
        sources: impl IntoIterator<Item = &'a ReportSource>,
    ) -> Result<V2ValidationSummary> {
        self.verify_inputs()?;
        let mut inputs = self.inputs.clone();
        let mut reports = Vec::new();
        for source in sources {
            let mut report_inputs = BTreeMap::new();
            let manifest_bytes = read_identified(
                &self.root,
                &source.manifest_path,
                Some(&source.manifest_sha256),
                &mut report_inputs,
            )?;
            let report: Report = parse_yaml(&source.manifest_path, &manifest_bytes)?;
            validate_catalog_binding(source, &report)?;
            validate_report(&self.root, &report, &mut report_inputs)?;
            inputs.extend(report_inputs.clone());
            reports.push(V2ReportSummary {
                id: report.id,
                version: report.version,
                lifecycle: report.lifecycle,
                fixture_only: report.fixture_only,
                source_root_sha256: digest_input_set("report-source-root:1", &report_inputs),
            });
        }
        Ok(V2ValidationSummary {
            selected_report_count: reports.len(),
            total_report_count: self.sources.len(),
            public_report_count: 0,
            source_root_sha256: digest_input_set("repository-source-root:1", &inputs),
            reports,
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct V2Catalog {
    schema_version: u32,
    contract_version: u32,
    source_state: String,
    schemas: Vec<SchemaSource>,
    reports: Vec<ReportSource>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SchemaSource {
    id: String,
    path: PathBuf,
    sha256: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReportSource {
    id: String,
    version: String,
    title: String,
    owner: String,
    fixture_only: bool,
    manifest_path: PathBuf,
    manifest_sha256: String,
}

fn validate_catalog_header(catalog: &V2Catalog) -> Result<()> {
    if catalog.schema_version != SCHEMA_VERSION || catalog.contract_version != CONTRACT_VERSION {
        return Err(AssuranceError::Invalid(
            "v2 catalog requires schema_version 1 and contract_version 1".to_owned(),
        ));
    }
    if catalog.source_state != SOURCE_STATE {
        return Err(AssuranceError::Invalid(format!(
            "v2 catalog source_state must be '{SOURCE_STATE}'"
        )));
    }
    Ok(())
}

fn validate_report_source(source: &ReportSource) -> Result<()> {
    validate_identity(&source.id, &source.title, &source.owner, "catalog report")?;
    validate_version(&source.version, "catalog report")?;
    validate_digest(&source.manifest_sha256, "report manifest")?;
    validate_relative(&source.manifest_path)?;
    if !source.fixture_only {
        return Err(AssuranceError::Invalid(
            "ASSURE-04A catalog admits only fixture_only sources".to_owned(),
        ));
    }
    Ok(())
}

fn validate_catalog_binding(source: &ReportSource, report: &Report) -> Result<()> {
    if source.id != report.id
        || source.version != report.version
        || source.title != report.title
        || source.owner != report.owner
        || source.fixture_only != report.fixture_only
    {
        return Err(AssuranceError::Invalid(format!(
            "catalog metadata does not match report manifest '{}'",
            source.id
        )));
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SchemaDocument {
    #[serde(rename = "$schema")]
    schema: String,
    #[serde(rename = "$id")]
    id: String,
    title: String,
    #[serde(rename = "type")]
    kind: String,
    #[serde(rename = "additionalProperties")]
    additional_properties: bool,
    required: Vec<String>,
    properties: BTreeMap<String, serde_json::Value>,
    #[serde(rename = "$defs", default)]
    definitions: BTreeMap<String, serde_json::Value>,
}

fn validate_schemas(
    root: &Path,
    sources: &[SchemaSource],
    inputs: &mut BTreeMap<PathBuf, String>,
) -> Result<()> {
    let expected = BTreeMap::from([
        ("openwepp:assurance:v2:catalog:1", CATALOG_FIELDS),
        ("openwepp:assurance:v2:report:1", REPORT_FIELDS),
        ("openwepp:assurance:v2:result:1", RESULT_FIELDS),
    ]);
    if sources.len() != expected.len() {
        return Err(AssuranceError::Invalid(
            "v2 catalog must bind exactly the catalog, report, and result schemas".to_owned(),
        ));
    }
    let mut observed = BTreeSet::new();
    let mut paths = BTreeSet::new();
    for source in sources {
        require_nonempty(&source.id, "schema ID")?;
        require_unique(&mut observed, &source.id, "schema")?;
        require_unique_path(&mut paths, &source.path, "schema")?;
        validate_digest(&source.sha256, "schema")?;
        let fields = expected.get(source.id.as_str()).ok_or_else(|| {
            AssuranceError::Invalid(format!("unknown v2 schema ID '{}'", source.id))
        })?;
        let bytes = read_identified(root, &source.path, Some(&source.sha256), inputs)?;
        let document: SchemaDocument = parse_json(&source.path, &bytes)?;
        validate_schema_document(source, &document, fields)?;
    }
    if observed != expected.keys().map(|id| (*id).to_owned()).collect() {
        return Err(AssuranceError::Invalid(
            "v2 schema identity set is incomplete".to_owned(),
        ));
    }
    Ok(())
}

fn validate_schema_document(
    source: &SchemaSource,
    document: &SchemaDocument,
    fields: &[&str],
) -> Result<()> {
    validate_schema_metadata(source, document)?;
    validate_schema_top_level_fields(source, document, fields)?;
    validate_schema_constants(source, document)?;
    let expected_definitions = expected_schema_definitions(&source.id)?;
    validate_schema_definitions(document, expected_definitions)?;
    if source.id == "openwepp:assurance:v2:result:1" {
        validate_result_value_schema(document)?;
    }
    Ok(())
}

fn validate_schema_metadata(source: &SchemaSource, document: &SchemaDocument) -> Result<()> {
    require_nonempty(&document.title, "schema title")?;
    if document.schema != "https://json-schema.org/draft/2020-12/schema"
        || document.id != source.id
        || document.kind != "object"
        || document.additional_properties
    {
        return Err(AssuranceError::Invalid(format!(
            "schema metadata contract mismatch for '{}'",
            source.id
        )));
    }
    Ok(())
}

fn validate_schema_top_level_fields(
    source: &SchemaSource,
    document: &SchemaDocument,
    fields: &[&str],
) -> Result<()> {
    let expected = fields.iter().map(|field| (*field).to_owned()).collect();
    let required = document.required.iter().cloned().collect::<BTreeSet<_>>();
    let properties = document.properties.keys().cloned().collect::<BTreeSet<_>>();
    if document.required.len() != required.len() || required != expected || properties != expected {
        return Err(AssuranceError::Invalid(format!(
            "schema required-field contract mismatch for '{}'",
            source.id
        )));
    }
    Ok(())
}

fn expected_schema_definitions(
    source_id: &str,
) -> Result<&'static [(&'static str, &'static [&'static str])]> {
    let definitions = match source_id {
        "openwepp:assurance:v2:catalog:1" => CATALOG_SCHEMA_DEFINITIONS,
        "openwepp:assurance:v2:report:1" => REPORT_SCHEMA_DEFINITIONS,
        "openwepp:assurance:v2:result:1" => &[],
        _ => {
            return Err(AssuranceError::Invalid(format!(
                "schema '{source_id}' has no executable definition contract"
            )));
        }
    };
    Ok(definitions)
}

fn validate_schema_definitions(
    document: &SchemaDocument,
    expected_definitions: &[(&str, &[&str])],
) -> Result<()> {
    for (name, definition) in &document.definitions {
        require_nonempty(name, "schema definition name")?;
        let required = validate_schema_definition(name, definition)?;
        match expected_definitions
            .iter()
            .find(|(expected_name, _)| *expected_name == name)
        {
            Some((_, expected_fields))
                if required.as_ref() == Some(&field_set(expected_fields)) => {}
            Some(_) => {
                return Err(AssuranceError::Invalid(format!(
                    "schema definition '{name}' disagrees with the executable typed contract"
                )));
            }
            None if required.is_some() => {
                return Err(AssuranceError::Invalid(format!(
                    "schema definition '{name}' is not part of the executable typed contract"
                )));
            }
            None => {}
        }
    }
    for (name, _) in expected_definitions {
        if !document.definitions.contains_key(*name) {
            return Err(AssuranceError::Invalid(format!(
                "schema definition '{name}' required by the executable typed contract is missing"
            )));
        }
    }
    Ok(())
}

fn validate_schema_constants(source: &SchemaSource, document: &SchemaDocument) -> Result<()> {
    let expected = match source.id.as_str() {
        "openwepp:assurance:v2:catalog:1" => vec![
            ("schema_version", serde_json::json!(SCHEMA_VERSION)),
            ("contract_version", serde_json::json!(CONTRACT_VERSION)),
            ("source_state", serde_json::json!(SOURCE_STATE)),
        ],
        "openwepp:assurance:v2:report:1" => vec![
            ("schema_version", serde_json::json!(SCHEMA_VERSION)),
            ("contract_version", serde_json::json!(CONTRACT_VERSION)),
            ("lifecycle", serde_json::json!(DRAFT)),
            ("fixture_only", serde_json::json!(true)),
        ],
        "openwepp:assurance:v2:result:1" => {
            vec![("schema_version", serde_json::json!(SCHEMA_VERSION))]
        }
        _ => Vec::new(),
    };
    for (field, value) in expected {
        let observed = document
            .properties
            .get(field)
            .and_then(|schema| schema.get("const"));
        if observed != Some(&value) {
            return Err(AssuranceError::Invalid(format!(
                "schema property '{field}' const disagrees with the executable typed contract"
            )));
        }
    }
    Ok(())
}

fn validate_schema_definition(
    name: &str,
    definition: &serde_json::Value,
) -> Result<Option<BTreeSet<String>>> {
    let object = definition.as_object().ok_or_else(|| {
        AssuranceError::Invalid(format!("schema definition '{name}' must be an object"))
    })?;
    let Some(required) = object.get("required") else {
        return Ok(None);
    };
    let required = required.as_array().ok_or_else(|| {
        AssuranceError::Invalid(format!(
            "schema definition '{name}' required fields must be an array"
        ))
    })?;
    let properties = object
        .get("properties")
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| {
            AssuranceError::Invalid(format!(
                "schema definition '{name}' with required fields needs properties"
            ))
        })?;
    if object.get("additionalProperties") != Some(&serde_json::Value::Bool(false)) {
        return Err(AssuranceError::Invalid(format!(
            "schema definition '{name}' must reject unknown fields"
        )));
    }
    let mut names = BTreeSet::new();
    for field in required {
        let field = field.as_str().ok_or_else(|| {
            AssuranceError::Invalid(format!(
                "schema definition '{name}' has a non-string required field"
            ))
        })?;
        if !names.insert(field) || !properties.contains_key(field) {
            return Err(AssuranceError::Invalid(format!(
                "schema definition '{name}' required-field contract is inconsistent"
            )));
        }
    }
    if names.len() != properties.len() {
        return Err(AssuranceError::Invalid(format!(
            "schema definition '{name}' has optional fields not admitted by the typed contract"
        )));
    }
    Ok(Some(names.into_iter().map(ToOwned::to_owned).collect()))
}

fn validate_result_value_schema(document: &SchemaDocument) -> Result<()> {
    let items = document
        .properties
        .get("values")
        .and_then(|values| values.get("items"))
        .ok_or_else(|| {
            AssuranceError::Invalid(
                "result schema is missing the executable result-value contract".to_owned(),
            )
        })?;
    let fields = validate_schema_definition("resultValue", items)?;
    if fields.as_ref() != Some(&field_set(RESULT_VALUE_FIELDS)) {
        return Err(AssuranceError::Invalid(
            "result-value schema disagrees with the executable typed contract".to_owned(),
        ));
    }
    Ok(())
}

fn field_set(fields: &[&str]) -> BTreeSet<String> {
    fields.iter().map(|field| (*field).to_owned()).collect()
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Report {
    schema_version: u32,
    contract_version: u32,
    id: String,
    version: String,
    title: String,
    owner: String,
    lifecycle: String,
    fixture_only: bool,
    authorship: Authorship,
    agent_assistance: AgentAssistance,
    manuscript: ContentSource,
    supplement: ContentSource,
    dependencies: Vec<Dependency>,
    units: Vec<Unit>,
    claims: Vec<Claim>,
    methods: Vec<Method>,
    results: Vec<ResultSource>,
    figures: Vec<Figure>,
    references: Vec<Reference>,
    research_objects: Vec<ResearchObject>,
    review: Review,
    publication: Publication,
}

/// A schema-required field whose explicit value may be `null`.
///
/// Unlike `Option<T>`, this wrapper has no Serde missing-field shortcut. The
/// containing record therefore rejects omission while still accepting an
/// explicit `null` as `None`.
#[derive(Debug, Default)]
enum RequiredNullable<T> {
    #[default]
    Missing,
    Null,
    Value(T),
}

impl<'de, T> Deserialize<'de> for RequiredNullable<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Option::<T>::deserialize(deserializer).map(|value| match value {
            Some(value) => Self::Value(value),
            None => Self::Null,
        })
    }
}

impl<T> RequiredNullable<T> {
    fn as_deref(&self) -> Option<&T::Target>
    where
        T: std::ops::Deref,
    {
        match self {
            Self::Value(value) => Some(&**value),
            Self::Missing | Self::Null => None,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Authorship {
    id: String,
    title: String,
    owner: String,
    draft_authors: Vec<String>,
    #[serde(default)]
    human_report_lead: RequiredNullable<String>,
    #[serde(default)]
    scientific_approver: RequiredNullable<String>,
    accountability_state: String,
    external_peer_review_claimed: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AgentAssistance {
    id: String,
    title: String,
    owner: String,
    procedure_version: String,
    objective: String,
    tool_model_identity: String,
    input_dependency_ids: Vec<String>,
    exact_output_dependency_id: String,
    human_disposition: String,
    known_nondeterminism: String,
    limitations: String,
    independent_review: String,
    provenance_complete: bool,
    review_entry_authorized: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ContentSource {
    id: String,
    title: String,
    owner: String,
    path: PathBuf,
    sha256: String,
    media_type: String,
    provenance: String,
    creation_procedure: String,
    claim_ids: Vec<String>,
    method_ids: Vec<String>,
    result_ids: Vec<String>,
    figure_ids: Vec<String>,
    reference_ids: Vec<String>,
    research_object_ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
enum DependencyKind {
    LocalContent,
    ExternalImmutable,
    Restricted,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Dependency {
    id: String,
    title: String,
    owner: String,
    kind: DependencyKind,
    provenance: String,
    creation_procedure: String,
    access: String,
    license: String,
    #[serde(default)]
    path: RequiredNullable<PathBuf>,
    #[serde(default)]
    sha256: RequiredNullable<String>,
    #[serde(default)]
    immutable_identity: RequiredNullable<String>,
    #[serde(default)]
    restriction_reason: RequiredNullable<String>,
    #[serde(default)]
    review_role: RequiredNullable<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Unit {
    id: String,
    symbol: String,
    quantity: String,
    definition: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Claim {
    id: String,
    title: String,
    owner: String,
    statement: String,
    #[serde(rename = "claim_type")]
    kind: String,
    scope_limit: String,
    method_ids: Vec<String>,
    result_ids: Vec<String>,
    dependency_ids: Vec<String>,
    unit_ids: Vec<String>,
    reference_ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Method {
    id: String,
    title: String,
    owner: String,
    description: String,
    procedure: String,
    dependency_ids: Vec<String>,
    unit_ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ResultSource {
    id: String,
    title: String,
    owner: String,
    path: PathBuf,
    sha256: String,
    media_type: String,
    method_id: String,
    dependency_ids: Vec<String>,
    unit_ids: Vec<String>,
    quantity_semantics: String,
    precision_policy: String,
    software_realization: String,
    provenance: String,
    creation_procedure: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ResultObject {
    schema_version: u32,
    result_id: String,
    values: Vec<ResultValue>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ResultValue {
    id: String,
    value: f64,
    unit_id: String,
    precision: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Figure {
    id: String,
    title: String,
    owner: String,
    kind: String,
    result_ids: Vec<String>,
    generation_procedure: String,
    alternative_text: String,
    caption: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Reference {
    id: String,
    title: String,
    owner: String,
    citation: String,
    immutable_identity: String,
    access: String,
    license: String,
    dependency_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ResearchObject {
    id: String,
    title: String,
    owner: String,
    access: String,
    license: String,
    #[serde(default)]
    path: RequiredNullable<PathBuf>,
    #[serde(default)]
    sha256: RequiredNullable<String>,
    #[serde(default)]
    restriction_reason: RequiredNullable<String>,
    #[serde(default)]
    review_role: RequiredNullable<String>,
    result_ids: Vec<String>,
    method_ids: Vec<String>,
    dependency_ids: Vec<String>,
    reproduction_instructions: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Review {
    id: String,
    title: String,
    owner: String,
    state: String,
    decision: String,
    #[serde(default)]
    reviewed_root: RequiredNullable<String>,
    approvers: Vec<String>,
    independence_assessment: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Publication {
    id: String,
    title: String,
    owner: String,
    state: String,
    #[serde(default)]
    public_path: RequiredNullable<PathBuf>,
    #[serde(default)]
    snapshot_id: RequiredNullable<String>,
    #[serde(default)]
    release_id: RequiredNullable<String>,
    export_authorized: bool,
    vendoring_authorized: bool,
    #[serde(default)]
    supersedes: RequiredNullable<String>,
    withdrawn: bool,
}

struct ReportIds {
    dependencies: BTreeSet<String>,
    units: BTreeSet<String>,
    claims: BTreeSet<String>,
    methods: BTreeSet<String>,
    results: BTreeSet<String>,
    figures: BTreeSet<String>,
    references: BTreeSet<String>,
    research_objects: BTreeSet<String>,
}

fn validate_report(
    root: &Path,
    report: &Report,
    inputs: &mut BTreeMap<PathBuf, String>,
) -> Result<()> {
    let ids = validate_report_structure(report)?;
    validate_content(root, &report.manuscript, &ids, inputs)?;
    validate_content(root, &report.supplement, &ids, inputs)?;
    for dependency in &report.dependencies {
        validate_dependency(root, dependency, inputs)?;
    }
    for source in &report.results {
        validate_result(root, source, &ids, inputs)?;
    }
    for object in &report.research_objects {
        validate_research_object(root, object, &ids, inputs)?;
    }
    Ok(())
}

fn validate_report_structure(report: &Report) -> Result<ReportIds> {
    validate_report_header(report)?;
    let ids = collect_report_ids(report)?;
    validate_authorship(&report.authorship)?;
    validate_agent_assistance(&report.agent_assistance, &ids)?;
    validate_content_contract(&report.manuscript, &ids)?;
    validate_content_contract(&report.supplement, &ids)?;
    for dependency in &report.dependencies {
        validate_dependency_shape(dependency)?;
    }
    for unit in &report.units {
        validate_unit(unit)?;
    }
    for claim in &report.claims {
        validate_claim(claim, &ids)?;
    }
    for method in &report.methods {
        validate_method(method, &ids)?;
    }
    for source in &report.results {
        validate_result_source(source, &ids)?;
    }
    for figure in &report.figures {
        validate_figure(figure, &ids)?;
    }
    for reference in &report.references {
        validate_reference(reference, &ids)?;
    }
    for object in &report.research_objects {
        validate_research_object_contract(object, &ids)?;
    }
    validate_review(&report.review)?;
    validate_publication(&report.publication)?;
    validate_no_unused(report)?;
    Ok(ids)
}

fn validate_report_header(report: &Report) -> Result<()> {
    if report.schema_version != SCHEMA_VERSION || report.contract_version != CONTRACT_VERSION {
        return Err(AssuranceError::Invalid(
            "v2 report requires schema_version 1 and contract_version 1".to_owned(),
        ));
    }
    validate_identity(&report.id, &report.title, &report.owner, "report")?;
    validate_version(&report.version, "report")?;
    if report.lifecycle != DRAFT || !report.fixture_only {
        return Err(AssuranceError::Invalid(
            "ASSURE-04A report must be DRAFT and fixture_only".to_owned(),
        ));
    }
    for (name, count) in [
        ("dependency", report.dependencies.len()),
        ("unit", report.units.len()),
        ("claim", report.claims.len()),
        ("method", report.methods.len()),
        ("result", report.results.len()),
        ("figure", report.figures.len()),
        ("reference", report.references.len()),
        ("research object", report.research_objects.len()),
    ] {
        if count == 0 {
            return Err(AssuranceError::Invalid(format!(
                "v2 fixture requires at least one {name} record"
            )));
        }
    }
    Ok(())
}

fn collect_report_ids(report: &Report) -> Result<ReportIds> {
    let mut all = BTreeSet::new();
    validate_container_ids(report, &mut all)?;
    validate_record_ids(report, &mut all)?;
    Ok(ReportIds {
        dependencies: report
            .dependencies
            .iter()
            .map(|value| value.id.clone())
            .collect(),
        units: report.units.iter().map(|value| value.id.clone()).collect(),
        claims: report.claims.iter().map(|value| value.id.clone()).collect(),
        methods: report
            .methods
            .iter()
            .map(|value| value.id.clone())
            .collect(),
        results: report
            .results
            .iter()
            .map(|value| value.id.clone())
            .collect(),
        figures: report
            .figures
            .iter()
            .map(|value| value.id.clone())
            .collect(),
        references: report
            .references
            .iter()
            .map(|value| value.id.clone())
            .collect(),
        research_objects: report
            .research_objects
            .iter()
            .map(|value| value.id.clone())
            .collect(),
    })
}

fn validate_container_ids(report: &Report, all: &mut BTreeSet<String>) -> Result<()> {
    insert_identity(
        all,
        &report.authorship.id,
        &report.authorship.title,
        &report.authorship.owner,
        "authorship",
    )?;
    insert_identity(
        all,
        &report.agent_assistance.id,
        &report.agent_assistance.title,
        &report.agent_assistance.owner,
        "agent assistance",
    )?;
    insert_identity(
        all,
        &report.manuscript.id,
        &report.manuscript.title,
        &report.manuscript.owner,
        "manuscript",
    )?;
    insert_identity(
        all,
        &report.supplement.id,
        &report.supplement.title,
        &report.supplement.owner,
        "supplement",
    )?;
    insert_identity(
        all,
        &report.review.id,
        &report.review.title,
        &report.review.owner,
        "review",
    )?;
    insert_identity(
        all,
        &report.publication.id,
        &report.publication.title,
        &report.publication.owner,
        "publication",
    )
}

fn validate_record_ids(report: &Report, all: &mut BTreeSet<String>) -> Result<()> {
    for value in &report.dependencies {
        insert_identity(all, &value.id, &value.title, &value.owner, "dependency")?;
    }
    for value in &report.units {
        validate_id(&value.id, "unit")?;
        require_unique(all, &value.id, "logical ID")?;
    }
    for value in &report.claims {
        insert_identity(all, &value.id, &value.title, &value.owner, "claim")?;
    }
    for value in &report.methods {
        insert_identity(all, &value.id, &value.title, &value.owner, "method")?;
    }
    for value in &report.results {
        insert_identity(all, &value.id, &value.title, &value.owner, "result")?;
    }
    for value in &report.figures {
        insert_identity(all, &value.id, &value.title, &value.owner, "figure")?;
    }
    for value in &report.references {
        insert_identity(all, &value.id, &value.title, &value.owner, "reference")?;
    }
    for value in &report.research_objects {
        insert_identity(
            all,
            &value.id,
            &value.title,
            &value.owner,
            "research object",
        )?;
    }
    Ok(())
}

fn insert_identity(
    ids: &mut BTreeSet<String>,
    id: &str,
    title: &str,
    owner: &str,
    kind: &str,
) -> Result<()> {
    validate_identity(id, title, owner, kind)?;
    require_unique(ids, id, "logical ID")
}

fn validate_authorship(authorship: &Authorship) -> Result<()> {
    if authorship.draft_authors.is_empty()
        || authorship
            .draft_authors
            .iter()
            .any(|author| author.trim().is_empty())
    {
        return Err(AssuranceError::Invalid(
            "draft authorship requires at least one named author".to_owned(),
        ));
    }
    require_absent(
        &authorship.human_report_lead,
        "authorship human_report_lead",
    )?;
    require_absent(
        &authorship.scientific_approver,
        "authorship scientific_approver",
    )?;
    if authorship.accountability_state != "unassigned_blocks_review"
        || authorship.external_peer_review_claimed
    {
        return Err(AssuranceError::Invalid(
            "ASSURE-04A fixture authorship must disclose unassigned human accountability and cannot claim external peer review"
                .to_owned(),
        ));
    }
    Ok(())
}

fn validate_agent_assistance(assistance: &AgentAssistance, ids: &ReportIds) -> Result<()> {
    for (value, name) in [
        (&assistance.procedure_version, "agent procedure_version"),
        (&assistance.objective, "agent objective"),
        (&assistance.tool_model_identity, "agent tool_model_identity"),
        (&assistance.human_disposition, "agent human_disposition"),
        (
            &assistance.known_nondeterminism,
            "agent known_nondeterminism",
        ),
        (&assistance.limitations, "agent limitations"),
        (&assistance.independent_review, "agent independent_review"),
    ] {
        require_nonempty(value, name)?;
    }
    validate_reference_list(
        &assistance.input_dependency_ids,
        &ids.dependencies,
        "agent input dependency",
        true,
    )?;
    require_known(
        &assistance.exact_output_dependency_id,
        &ids.dependencies,
        "agent exact-output dependency",
    )?;
    if assistance.provenance_complete || assistance.review_entry_authorized {
        return Err(AssuranceError::Invalid(
            "incomplete ASSURE-02 agent provenance must block review entry".to_owned(),
        ));
    }
    Ok(())
}

fn validate_content(
    root: &Path,
    content: &ContentSource,
    ids: &ReportIds,
    inputs: &mut BTreeMap<PathBuf, String>,
) -> Result<()> {
    validate_content_contract(content, ids)?;
    read_identified(root, &content.path, Some(&content.sha256), inputs)?;
    Ok(())
}

fn validate_content_contract(content: &ContentSource, ids: &ReportIds) -> Result<()> {
    if content.media_type != "text/markdown" {
        return Err(AssuranceError::Invalid(format!(
            "content '{}' media_type must be text/markdown",
            content.id
        )));
    }
    validate_relative(&content.path)?;
    validate_digest(&content.sha256, "content source")?;
    require_nonempty(&content.provenance, "content provenance")?;
    require_nonempty(&content.creation_procedure, "content creation_procedure")?;
    for (kind, references, known) in [
        ("claim", &content.claim_ids, &ids.claims),
        ("method", &content.method_ids, &ids.methods),
        ("result", &content.result_ids, &ids.results),
        ("figure", &content.figure_ids, &ids.figures),
        ("reference", &content.reference_ids, &ids.references),
        (
            "research object",
            &content.research_object_ids,
            &ids.research_objects,
        ),
    ] {
        validate_reference_list(references, known, kind, true)?;
    }
    Ok(())
}

fn validate_dependency(
    root: &Path,
    dependency: &Dependency,
    inputs: &mut BTreeMap<PathBuf, String>,
) -> Result<()> {
    validate_dependency_shape(dependency)?;
    if let DependencyKind::LocalContent = dependency.kind {
        let path = required_path(dependency.path.as_deref(), "local dependency path")?;
        let digest = required_text(dependency.sha256.as_deref(), "local dependency sha256")?;
        read_identified(root, path, Some(digest), inputs)?;
    }
    Ok(())
}

fn validate_dependency_shape(dependency: &Dependency) -> Result<()> {
    validate_dependency_metadata(dependency)?;
    match dependency.kind {
        DependencyKind::LocalContent => validate_local_dependency_shape(dependency),
        DependencyKind::ExternalImmutable => validate_external_dependency(dependency),
        DependencyKind::Restricted => validate_restricted_dependency(dependency),
    }
}

fn validate_dependency_metadata(dependency: &Dependency) -> Result<()> {
    require_nonempty(&dependency.provenance, "dependency provenance")?;
    require_nonempty(
        &dependency.creation_procedure,
        "dependency creation_procedure",
    )?;
    require_nonempty(&dependency.access, "dependency access")?;
    require_nonempty(&dependency.license, "dependency license")
}

fn validate_local_dependency_shape(dependency: &Dependency) -> Result<()> {
    require_absent(
        &dependency.immutable_identity,
        "local dependency immutable_identity",
    )?;
    require_absent(
        &dependency.restriction_reason,
        "local dependency restriction_reason",
    )?;
    require_absent(&dependency.review_role, "local dependency review_role")?;
    let path = required_path(dependency.path.as_deref(), "local dependency path")?;
    validate_relative(path)?;
    let digest = required_text(dependency.sha256.as_deref(), "local dependency sha256")?;
    validate_digest(digest, "local dependency")
}

fn validate_external_dependency(dependency: &Dependency) -> Result<()> {
    require_absent(&dependency.path, "external dependency path")?;
    require_absent(&dependency.sha256, "external dependency sha256")?;
    require_present_nonempty(
        dependency.immutable_identity.as_deref(),
        "external dependency immutable_identity",
    )?;
    require_absent(
        &dependency.restriction_reason,
        "external restriction_reason",
    )?;
    require_absent(&dependency.review_role, "external review_role")
}

fn validate_restricted_dependency(dependency: &Dependency) -> Result<()> {
    require_absent(&dependency.path, "restricted dependency path")?;
    require_absent(&dependency.sha256, "restricted dependency sha256")?;
    require_present_nonempty(
        dependency.immutable_identity.as_deref(),
        "restricted dependency immutable_identity",
    )?;
    require_present_nonempty(
        dependency.restriction_reason.as_deref(),
        "restricted dependency restriction_reason",
    )?;
    require_present_nonempty(
        dependency.review_role.as_deref(),
        "restricted dependency review_role",
    )
}

fn validate_unit(unit: &Unit) -> Result<()> {
    require_nonempty(&unit.symbol, "unit symbol")?;
    require_nonempty(&unit.quantity, "unit quantity")?;
    require_nonempty(&unit.definition, "unit definition")
}

fn validate_claim(claim: &Claim, ids: &ReportIds) -> Result<()> {
    require_nonempty(&claim.statement, "claim statement")?;
    require_nonempty(&claim.kind, "claim claim_type")?;
    require_nonempty(&claim.scope_limit, "claim scope_limit")?;
    validate_reference_list(&claim.method_ids, &ids.methods, "method", true)?;
    validate_reference_list(&claim.result_ids, &ids.results, "result", false)?;
    validate_reference_list(&claim.dependency_ids, &ids.dependencies, "dependency", true)?;
    validate_reference_list(&claim.unit_ids, &ids.units, "unit", false)?;
    validate_reference_list(&claim.reference_ids, &ids.references, "reference", true)
}

fn validate_method(method: &Method, ids: &ReportIds) -> Result<()> {
    require_nonempty(&method.description, "method description")?;
    require_nonempty(&method.procedure, "method procedure")?;
    validate_reference_list(
        &method.dependency_ids,
        &ids.dependencies,
        "dependency",
        true,
    )?;
    validate_reference_list(&method.unit_ids, &ids.units, "unit", false)
}

fn validate_result(
    root: &Path,
    source: &ResultSource,
    ids: &ReportIds,
    inputs: &mut BTreeMap<PathBuf, String>,
) -> Result<()> {
    validate_result_source(source, ids)?;
    let bytes = read_identified(root, &source.path, Some(&source.sha256), inputs)?;
    let result: ResultObject = parse_json(&source.path, &bytes)?;
    validate_result_object(source, &result, ids)
}

fn validate_result_source(source: &ResultSource, ids: &ReportIds) -> Result<()> {
    if source.media_type != "application/json" {
        return Err(AssuranceError::Invalid(format!(
            "result '{}' media_type must be application/json",
            source.id
        )));
    }
    validate_relative(&source.path)?;
    validate_digest(&source.sha256, "result source")?;
    require_nonempty(&source.quantity_semantics, "result quantity_semantics")?;
    require_nonempty(&source.precision_policy, "result precision_policy")?;
    require_nonempty(&source.provenance, "result provenance")?;
    require_nonempty(&source.creation_procedure, "result creation_procedure")?;
    require_known(&source.method_id, &ids.methods, "method")?;
    require_known(
        &source.software_realization,
        &ids.dependencies,
        "software realization dependency",
    )?;
    validate_reference_list(
        &source.dependency_ids,
        &ids.dependencies,
        "dependency",
        true,
    )?;
    validate_reference_list(&source.unit_ids, &ids.units, "unit", true)?;
    Ok(())
}

fn validate_result_object(
    source: &ResultSource,
    result: &ResultObject,
    ids: &ReportIds,
) -> Result<()> {
    if result.schema_version != SCHEMA_VERSION {
        return Err(AssuranceError::Invalid(
            "result schema_version must be 1".to_owned(),
        ));
    }
    if result.result_id != source.id {
        return Err(AssuranceError::Invalid(format!(
            "result object ID does not match source '{}'",
            source.id
        )));
    }
    if result.values.is_empty() {
        return Err(AssuranceError::Invalid(format!(
            "result '{}' must contain values",
            source.id
        )));
    }
    validate_result_values(source, &result.values, ids)
}

fn validate_result_values(
    source: &ResultSource,
    values: &[ResultValue],
    ids: &ReportIds,
) -> Result<()> {
    let declared = source.unit_ids.iter().cloned().collect::<BTreeSet<_>>();
    let mut observed = BTreeSet::new();
    for value in values {
        validate_id(&value.id, "result value")?;
        require_unique(&mut observed, &value.id, "result value")?;
        if !value.value.is_finite() {
            return Err(AssuranceError::Invalid(format!(
                "result value '{}' must be finite",
                value.id
            )));
        }
        require_known(&value.unit_id, &ids.units, "unit")?;
        require_known(&value.unit_id, &declared, "result-declared unit")?;
        require_nonempty(&value.precision, "result value precision")?;
    }
    Ok(())
}

fn validate_figure(figure: &Figure, ids: &ReportIds) -> Result<()> {
    if figure.kind != "result_bearing" && figure.kind != "conceptual" {
        return Err(AssuranceError::Invalid(format!(
            "figure '{}' has unsupported kind",
            figure.id
        )));
    }
    if figure.kind == "result_bearing" && figure.result_ids.is_empty() {
        return Err(AssuranceError::Invalid(format!(
            "result-bearing figure '{}' requires a result",
            figure.id
        )));
    }
    if figure.kind == "conceptual" && !figure.result_ids.is_empty() {
        return Err(AssuranceError::Invalid(format!(
            "conceptual figure '{}' cannot carry result identities",
            figure.id
        )));
    }
    validate_reference_list(&figure.result_ids, &ids.results, "result", false)?;
    require_nonempty(&figure.generation_procedure, "figure generation_procedure")?;
    require_nonempty(&figure.alternative_text, "figure alternative_text")?;
    require_nonempty(&figure.caption, "figure caption")
}

fn validate_reference(reference: &Reference, ids: &ReportIds) -> Result<()> {
    require_nonempty(&reference.citation, "reference citation")?;
    require_nonempty(
        &reference.immutable_identity,
        "reference immutable_identity",
    )?;
    require_nonempty(&reference.access, "reference access")?;
    require_nonempty(&reference.license, "reference license")?;
    require_known(&reference.dependency_id, &ids.dependencies, "dependency")
}

fn validate_research_object(
    root: &Path,
    object: &ResearchObject,
    ids: &ReportIds,
    inputs: &mut BTreeMap<PathBuf, String>,
) -> Result<()> {
    validate_research_object_contract(object, ids)?;
    if object.access == "public_safe" {
        let path = required_path(object.path.as_deref(), "public-safe research-object path")?;
        let digest = required_text(
            object.sha256.as_deref(),
            "public-safe research-object sha256",
        )?;
        read_identified(root, path, Some(digest), inputs)?;
    }
    Ok(())
}

fn validate_research_object_contract(object: &ResearchObject, ids: &ReportIds) -> Result<()> {
    validate_research_object_metadata(object)?;
    match object.access.as_str() {
        "public_safe" => validate_public_research_object_shape(object)?,
        "restricted" => validate_restricted_research_object(object)?,
        _ => {
            return Err(AssuranceError::Invalid(format!(
                "research object '{}' has unsupported access",
                object.id
            )));
        }
    }
    validate_research_object_relations(object, ids)
}

fn validate_research_object_metadata(object: &ResearchObject) -> Result<()> {
    require_nonempty(&object.access, "research-object access")?;
    require_nonempty(&object.license, "research-object license")?;
    require_nonempty(
        &object.reproduction_instructions,
        "research-object reproduction_instructions",
    )
}

fn validate_public_research_object_shape(object: &ResearchObject) -> Result<()> {
    require_absent(
        &object.restriction_reason,
        "public-safe research-object restriction_reason",
    )?;
    require_absent(
        &object.review_role,
        "public-safe research-object review_role",
    )?;
    let path = required_path(object.path.as_deref(), "public-safe research-object path")?;
    validate_relative(path)?;
    let digest = required_text(
        object.sha256.as_deref(),
        "public-safe research-object sha256",
    )?;
    validate_digest(digest, "public-safe research object")
}

fn validate_restricted_research_object(object: &ResearchObject) -> Result<()> {
    require_absent(&object.path, "restricted research-object path")?;
    require_absent(&object.sha256, "restricted research-object sha256")?;
    require_present_nonempty(
        object.restriction_reason.as_deref(),
        "restricted research-object restriction_reason",
    )?;
    require_present_nonempty(
        object.review_role.as_deref(),
        "restricted research-object review_role",
    )
}

fn validate_research_object_relations(object: &ResearchObject, ids: &ReportIds) -> Result<()> {
    validate_reference_list(&object.result_ids, &ids.results, "result", true)?;
    validate_reference_list(&object.method_ids, &ids.methods, "method", true)?;
    validate_reference_list(
        &object.dependency_ids,
        &ids.dependencies,
        "dependency",
        true,
    )
}

fn validate_review(review: &Review) -> Result<()> {
    require_absent(&review.reviewed_root, "review reviewed_root")?;
    if review.state != DRAFT
        || review.decision != "not_started"
        || !review.approvers.is_empty()
        || review.independence_assessment != "not_assessed"
    {
        return Err(AssuranceError::Invalid(
            "draft review cannot claim a decision, root, approver, or independence assessment"
                .to_owned(),
        ));
    }
    Ok(())
}

fn validate_publication(publication: &Publication) -> Result<()> {
    require_absent(&publication.public_path, "publication public_path")?;
    require_absent(&publication.snapshot_id, "publication snapshot_id")?;
    require_absent(&publication.release_id, "publication release_id")?;
    require_absent(&publication.supersedes, "publication supersedes")?;
    if publication.state != DRAFT
        || publication.export_authorized
        || publication.vendoring_authorized
        || publication.withdrawn
    {
        return Err(AssuranceError::Invalid(
            "draft publication cannot claim a public path, snapshot, release, export, vendoring, supersession, or withdrawal"
                .to_owned(),
        ));
    }
    Ok(())
}

fn validate_no_unused(report: &Report) -> Result<()> {
    let contents = [&report.manuscript, &report.supplement];
    let used_claims = union_lists(contents.iter().map(|content| &content.claim_ids));
    let used_methods = union_lists(
        contents
            .iter()
            .map(|content| &content.method_ids)
            .chain(report.claims.iter().map(|claim| &claim.method_ids))
            .chain(
                report
                    .research_objects
                    .iter()
                    .map(|object| &object.method_ids),
            ),
    );
    let used_results = union_lists(
        contents
            .iter()
            .map(|content| &content.result_ids)
            .chain(report.claims.iter().map(|claim| &claim.result_ids))
            .chain(report.figures.iter().map(|figure| &figure.result_ids))
            .chain(
                report
                    .research_objects
                    .iter()
                    .map(|object| &object.result_ids),
            ),
    );
    let used_figures = union_lists(contents.iter().map(|content| &content.figure_ids));
    let used_references = union_lists(
        contents
            .iter()
            .map(|content| &content.reference_ids)
            .chain(report.claims.iter().map(|claim| &claim.reference_ids)),
    );
    let used_objects = union_lists(contents.iter().map(|content| &content.research_object_ids));
    let used_dependencies = used_dependencies(report);
    let used_units = union_lists(
        report
            .claims
            .iter()
            .map(|claim| &claim.unit_ids)
            .chain(report.methods.iter().map(|method| &method.unit_ids))
            .chain(report.results.iter().map(|result| &result.unit_ids)),
    );

    require_all_used(
        "claim",
        report.claims.iter().map(|value| &value.id),
        &used_claims,
    )?;
    require_all_used(
        "method",
        report.methods.iter().map(|value| &value.id),
        &used_methods,
    )?;
    require_all_used(
        "result",
        report.results.iter().map(|value| &value.id),
        &used_results,
    )?;
    require_all_used(
        "figure",
        report.figures.iter().map(|value| &value.id),
        &used_figures,
    )?;
    require_all_used(
        "reference",
        report.references.iter().map(|value| &value.id),
        &used_references,
    )?;
    require_all_used(
        "research object",
        report.research_objects.iter().map(|value| &value.id),
        &used_objects,
    )?;
    require_all_used(
        "dependency",
        report.dependencies.iter().map(|value| &value.id),
        &used_dependencies,
    )?;
    require_all_used(
        "unit",
        report.units.iter().map(|value| &value.id),
        &used_units,
    )
}

fn used_dependencies(report: &Report) -> BTreeSet<String> {
    let mut used = report
        .agent_assistance
        .input_dependency_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    used.insert(report.agent_assistance.exact_output_dependency_id.clone());
    for claim in &report.claims {
        used.extend(claim.dependency_ids.iter().cloned());
    }
    for method in &report.methods {
        used.extend(method.dependency_ids.iter().cloned());
    }
    for result in &report.results {
        used.extend(result.dependency_ids.iter().cloned());
        used.insert(result.software_realization.clone());
    }
    for object in &report.research_objects {
        used.extend(object.dependency_ids.iter().cloned());
    }
    for reference in &report.references {
        used.insert(reference.dependency_id.clone());
    }
    used
}

fn union_lists<'a>(lists: impl Iterator<Item = &'a Vec<String>>) -> BTreeSet<String> {
    lists
        .flat_map(|values| values.iter().cloned())
        .collect::<BTreeSet<_>>()
}

fn require_all_used<'a>(
    kind: &str,
    declared: impl Iterator<Item = &'a String>,
    used: &BTreeSet<String>,
) -> Result<()> {
    for id in declared {
        if !used.contains(id) {
            return Err(AssuranceError::Invalid(format!("unused {kind} ID '{id}'")));
        }
    }
    Ok(())
}

fn validate_reference_list(
    references: &[String],
    known: &BTreeSet<String>,
    kind: &str,
    required: bool,
) -> Result<()> {
    if required && references.is_empty() {
        return Err(AssuranceError::Invalid(format!(
            "{kind} reference list cannot be empty"
        )));
    }
    let mut observed = BTreeSet::new();
    for id in references {
        require_known(id, known, kind)?;
        require_unique(&mut observed, id, &format!("{kind} reference"))?;
    }
    Ok(())
}

fn validate_identity(id: &str, title: &str, owner: &str, kind: &str) -> Result<()> {
    validate_id(id, kind)?;
    require_nonempty(title, &format!("{kind} title"))?;
    require_nonempty(owner, &format!("{kind} owner"))
}

fn validate_id(id: &str, kind: &str) -> Result<()> {
    if !id.as_bytes().first().is_some_and(u8::is_ascii_alphanumeric)
        || !id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b':'))
    {
        return Err(AssuranceError::Invalid(format!(
            "{kind} ID must start with an ASCII letter or digit and then use only letters, digits, '-', '_', '.', or ':'"
        )));
    }
    Ok(())
}

fn validate_version(version: &str, kind: &str) -> Result<()> {
    let components = version.split('.').collect::<Vec<_>>();
    if components.len() != 3
        || components.iter().any(|component| {
            component.is_empty()
                || (component.len() > 1 && component.starts_with('0'))
                || !component.bytes().all(|byte| byte.is_ascii_digit())
        })
    {
        return Err(AssuranceError::Invalid(format!(
            "{kind} version must be semantic MAJOR.MINOR.PATCH without leading zeros"
        )));
    }
    Ok(())
}

fn validate_digest(digest: &str, kind: &str) -> Result<()> {
    if digest.len() != 64
        || !digest
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        return Err(AssuranceError::Invalid(format!(
            "{kind} SHA-256 must be 64 lowercase hexadecimal characters"
        )));
    }
    Ok(())
}

fn require_nonempty(value: &str, name: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(AssuranceError::Invalid(format!("{name} cannot be empty")));
    }
    Ok(())
}

fn require_known(id: &str, known: &BTreeSet<String>, kind: &str) -> Result<()> {
    if !known.contains(id) {
        return Err(AssuranceError::Invalid(format!("unknown {kind} ID '{id}'")));
    }
    Ok(())
}

fn require_unique(values: &mut BTreeSet<String>, id: &str, kind: &str) -> Result<()> {
    if !values.insert(id.to_owned()) {
        return Err(AssuranceError::Invalid(format!(
            "duplicate {kind} ID '{id}'"
        )));
    }
    Ok(())
}

fn require_unique_path(values: &mut BTreeSet<PathBuf>, path: &Path, kind: &str) -> Result<()> {
    validate_relative(path)?;
    if !values.insert(path.to_path_buf()) {
        return Err(AssuranceError::Invalid(format!(
            "duplicate {kind} path '{}': paths cannot be identities",
            path.display()
        )));
    }
    Ok(())
}

fn required_text<'a>(value: Option<&'a str>, name: &str) -> Result<&'a str> {
    value.ok_or_else(|| AssuranceError::Invalid(format!("{name} is required")))
}

fn require_present_nonempty(value: Option<&str>, name: &str) -> Result<()> {
    require_nonempty(required_text(value, name)?, name)
}

fn required_path<'a>(value: Option<&'a Path>, name: &str) -> Result<&'a Path> {
    value.ok_or_else(|| AssuranceError::Invalid(format!("{name} is required")))
}

fn require_absent<T>(value: &RequiredNullable<T>, name: &str) -> Result<()> {
    match value {
        RequiredNullable::Missing => Err(AssuranceError::Invalid(format!(
            "required nullable field '{name}' is missing"
        ))),
        RequiredNullable::Value(_) => Err(AssuranceError::Invalid(format!("{name} must be null"))),
        RequiredNullable::Null => Ok(()),
    }
}

fn read_identified(
    root: &Path,
    path: &Path,
    expected: Option<&str>,
    inputs: &mut BTreeMap<PathBuf, String>,
) -> Result<Vec<u8>> {
    let bytes = read_regular_confined(root, path)?;
    let digest = sha256_bytes(&bytes);
    if let Some(expected) = expected {
        validate_digest(expected, "source")?;
        if digest != expected {
            return Err(AssuranceError::Invalid(format!(
                "SHA-256 mismatch for identified source '{}'",
                path.display()
            )));
        }
    }
    match inputs.insert(path.to_path_buf(), digest.clone()) {
        Some(prior) if prior != digest => Err(AssuranceError::Invalid(format!(
            "source path '{}' was bound to conflicting identities",
            path.display()
        ))),
        _ => Ok(bytes),
    }
}

fn parse_yaml<T: for<'de> Deserialize<'de>>(path: impl Into<PathBuf>, bytes: &[u8]) -> Result<T> {
    serde_yaml::from_slice(bytes).map_err(|error| AssuranceError::Parse {
        path: path.into(),
        message: error.to_string(),
    })
}

fn parse_json<T: for<'de> Deserialize<'de>>(path: &Path, bytes: &[u8]) -> Result<T> {
    serde_json::from_slice(bytes).map_err(|error| AssuranceError::Parse {
        path: path.to_path_buf(),
        message: error.to_string(),
    })
}

fn digest_input_set(domain: &str, inputs: &BTreeMap<PathBuf, String>) -> String {
    let mut material = format!("openwepp-assurance-v2:{domain}\n");
    for (path, digest) in inputs {
        material.push_str(&path.to_string_lossy());
        material.push(' ');
        material.push_str(digest);
        material.push('\n');
    }
    sha256_bytes(material.as_bytes())
}
