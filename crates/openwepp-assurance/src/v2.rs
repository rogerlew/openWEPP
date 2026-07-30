use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::error::{AssuranceError, Result};
use crate::hash::sha256_bytes;

mod amendment;
mod amendment_support;
mod assembly;
mod confined;
mod draft_adoption;
mod fixture;
mod identity;
mod lifecycle;
mod normalization;
mod planner;
mod publication;
mod receipt;
mod svg;
mod transaction;

pub use amendment::{
    V2AmendMode, V2AmendmentReceipt, V2Inspection, V2RecoveryAction, admit_report,
    admit_report_at_generation, adopt_report_source, adopt_report_source_at_generation,
    amend_attribution, amend_attribution_at_generation, amend_lifecycle,
    amend_lifecycle_at_generation, amend_normalize, amend_normalize_at_generation, amend_principal,
    amend_principal_at_generation, amend_role, amend_role_at_generation, inspect_report,
    rebind_implementation, recover_amendment, verify_generation,
};
pub use assembly::{V2AssemblyResult, V2AssemblySummary};
pub use fixture::{
    copy_v2_test_fixture, rebind_invalid_v2_test_fixture, rebind_v2_test_fixture,
    retain_v2_test_report,
};
pub use normalization::{
    V2NormalizationChange, V2NormalizationMode, V2NormalizationOptions, V2NormalizationReceipt,
};
pub use planner::{V2Plan, V2PlanNode, V2PlanState, V2ReportPlan};
pub use publication::{
    V2PublicationFault, V2PublicationOptions, V2PublicationResult, V2ReleaseIdentity,
    V2ReleaseVerification, V2ReviewRoots, V2TrustDomain, verify_v2_release_snapshot,
};
pub use receipt::V2ReceiptReportRoots;

pub(crate) use confined::read_regular_confined;
use confined::validate_relative;
use identity::{IDENTITY_LOCK_PATH, IdentityLock};
use lifecycle::{
    Principal, PrincipalKind, PrincipalRegistry, ReaderMetadata, digest_input_set, validate_date,
    validate_principal_registry, validate_reader_metadata, validate_review,
};

const V2_CATALOG_PATH: &str = "assurance/v2/catalog.yaml";
const CATALOG_SCHEMA_VERSION: u32 = 4;
const REPORT_SCHEMA_VERSION: u32 = 4;
const RESULT_SCHEMA_VERSION: u32 = 1;
const PRINCIPAL_SCHEMA_VERSION: u32 = 2;
const CONTRACT_VERSION: u32 = 4;
const SOURCE_STATE: &str = "internal_assurance_sources";
const DRAFT: &str = "DRAFT";

const CATALOG_FIELDS: &[&str] = &[
    "schema_version",
    "contract_version",
    "source_state",
    "trust_domain",
    "principal_registry",
    "schemas",
    "reports",
];
pub(super) const REPORT_FIELDS: &[&str] = &[
    "schema_version",
    "contract_version",
    "id",
    "version",
    "title",
    "owner",
    "lifecycle",
    "trust_domain",
    "fixture_only",
    "reader_metadata",
    "authorship",
    "agent_assistance",
    "manuscript",
    "supplement",
    "dependencies",
    "units",
    "claims",
    "methods",
    "results",
    "value_bindings",
    "tables",
    "figures",
    "references",
    "research_objects",
    "review",
    "publication",
];
const RESULT_FIELDS: &[&str] = &["schema_version", "result_id", "values"];
const CATALOG_SCHEMA_SOURCE_FIELDS: &[&str] = &["id", "path"];
const CATALOG_REPORT_SOURCE_FIELDS: &[&str] = &[
    "id",
    "version",
    "title",
    "owner",
    "trust_domain",
    "fixture_only",
    "manifest_path",
];
const CONTENT_IDENTITY_FIELDS: &[&str] = &["path"];
const READER_METADATA_FIELDS: &[&str] = &[
    "scientific_question",
    "assessed_process",
    "assessed_quantity",
    "realization",
    "related_model_narrative",
    "manuscript_date",
];
const CONTENT_SOURCE_FIELDS: &[&str] = &[
    "id",
    "title",
    "owner",
    "path",
    "media_type",
    "provenance",
    "creation_procedure",
    "claim_ids",
    "method_ids",
    "result_ids",
    "value_binding_ids",
    "table_ids",
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
    "value_binding_ids",
    "visualization",
    "generation_procedure",
    "alternative_text",
    "caption",
];
const RETAINED_FIGURE_FIELDS: &[&str] = &[
    "id",
    "title",
    "owner",
    "kind",
    "result_ids",
    "value_binding_ids",
    "visualization",
    "research_object_id",
    "ancillary_object_id",
    "generation_procedure",
    "alternative_text",
    "caption",
];
const VALUE_BINDING_FIELDS: &[&str] = &[
    "id",
    "title",
    "owner",
    "result_id",
    "value_id",
    "unit_id",
    "transform",
    "display",
];
const TABLE_FIELDS: &[&str] = &[
    "id",
    "title",
    "owner",
    "caption",
    "alternative_text",
    "row_header",
    "columns",
    "rows",
];
const TABLE_COLUMN_FIELDS: &[&str] = &["label", "unit_id"];
const TABLE_ROW_FIELDS: &[&str] = &["label", "value_binding_ids"];
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
    "review_charge",
    "build_maintainer_id",
    "material_producer_ids",
    "independence_assessment",
];
const FINDING_FIELDS: &[&str] = &[
    "id",
    "summary",
    "severity",
    "disposition",
    "rationale",
    "resolution",
    "verification",
    "verifier_id",
];
const APPROVAL_FIELDS: &[&str] = &[
    "role",
    "principal_id",
    "finding_ledger_root",
    "decision",
    "competence_basis",
    "independence_attestation",
    "approved_on",
];
const PUBLICATION_FIELDS: &[&str] = &[
    "id",
    "title",
    "owner",
    "state",
    "target_release_commit",
    "target_release_configuration",
    "prior_realization",
    "candidate_realization",
    "impact_assessment",
    "reproduction_disposition",
    "semantic_differences",
    "release_owner_id",
    "assurance_steward_id",
    "publication_date",
    "public_path",
    "export_authorized",
    "vendoring_authorized",
    "supersedes",
    "withdrawn",
];
const PRINCIPAL_REGISTRY_FIELDS: &[&str] = &["schema_version", "trust_domain", "principals"];
const PRINCIPAL_FIELDS: &[&str] = &[
    "id",
    "record_version",
    "supersedes",
    "display_name",
    "affiliations",
    "kind",
    "identity_authority",
    "identity_reference",
    "roles",
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
    ("contentIdentity", CONTENT_IDENTITY_FIELDS),
    ("schemaSource", CATALOG_SCHEMA_SOURCE_FIELDS),
    ("reportSource", CATALOG_REPORT_SOURCE_FIELDS),
];
const REPORT_SCHEMA_DEFINITIONS: &[(&str, &[&str])] = &[
    ("readerMetadata", READER_METADATA_FIELDS),
    ("authorship", AUTHORSHIP_FIELDS),
    ("agentAssistance", AGENT_ASSISTANCE_FIELDS),
    ("contentSource", CONTENT_SOURCE_FIELDS),
    ("dependency", DEPENDENCY_FIELDS),
    ("unit", UNIT_FIELDS),
    ("claim", CLAIM_FIELDS),
    ("method", METHOD_FIELDS),
    ("result", RESULT_SOURCE_FIELDS),
    ("valueBinding", VALUE_BINDING_FIELDS),
    ("table", TABLE_FIELDS),
    ("tableColumn", TABLE_COLUMN_FIELDS),
    ("tableRow", TABLE_ROW_FIELDS),
    ("generatedFigure", FIGURE_FIELDS),
    ("retainedFigure", RETAINED_FIGURE_FIELDS),
    ("reference", REFERENCE_FIELDS),
    ("researchObject", RESEARCH_OBJECT_FIELDS),
    ("review", REVIEW_FIELDS),
    ("finding", FINDING_FIELDS),
    ("approval", APPROVAL_FIELDS),
    ("publication", PUBLICATION_FIELDS),
];
const PRINCIPAL_SCHEMA_DEFINITIONS: &[(&str, &[&str])] = &[("principal", PRINCIPAL_FIELDS)];

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
    trust_domain: V2TrustDomain,
    principals: PrincipalRegistry,
    identity: IdentityLock,
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
        let identity = IdentityLock::load(&root)?;
        let identity_bytes = read_regular_confined(&root, Path::new(IDENTITY_LOCK_PATH))?;
        inputs.insert(
            PathBuf::from(IDENTITY_LOCK_PATH),
            sha256_bytes(&identity_bytes),
        );
        let catalog_path = Path::new(V2_CATALOG_PATH);
        let catalog_bytes = read_identified(
            &root,
            catalog_path,
            Some(identity.digest_for(catalog_path)?),
            &mut inputs,
        )?;
        let catalog: V2Catalog = parse_hydrated_yaml(V2_CATALOG_PATH, &catalog_bytes, &identity)?;
        validate_catalog_header(&catalog)?;
        validate_schemas(&root, &catalog.schemas, &mut inputs)?;
        validate_relative(&catalog.principal_registry.path)?;
        validate_digest(&catalog.principal_registry.sha256, "principal registry")?;
        let principal_bytes = read_identified(
            &root,
            &catalog.principal_registry.path,
            Some(&catalog.principal_registry.sha256),
            &mut inputs,
        )?;
        let principals: PrincipalRegistry =
            parse_yaml(&catalog.principal_registry.path, &principal_bytes)?;
        validate_principal_registry(&principals, catalog.trust_domain)?;

        let mut sources = BTreeMap::new();
        let mut catalog_ids = BTreeSet::new();
        let mut manifest_paths = BTreeSet::new();
        for source in &catalog.reports {
            validate_report_source(source)?;
            if source.trust_domain != catalog.trust_domain {
                return Err(AssuranceError::Invalid(format!(
                    "catalog report '{}' trust domain does not match the catalog",
                    source.id
                )));
            }
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
            trust_domain: catalog.trust_domain,
            principals,
            identity,
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

    /// Checks or applies canonical American-English normalization to one DRAFT
    /// report and mechanically rebinds its dependent identities.
    ///
    /// # Errors
    ///
    /// Returns a typed error for source drift, a non-DRAFT lifecycle, an
    /// unavailable converter, a required lexical change in check mode, or a
    /// failed source transaction.
    pub fn normalize_report(
        &self,
        report_id: &str,
        options: &V2NormalizationOptions,
    ) -> Result<V2NormalizationReceipt> {
        normalization::normalize_report(self, report_id, options)
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
        planner::plan_sources(
            &self.root,
            &self.inputs,
            &self.identity,
            self.sources.len(),
            &sources,
        )
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
        planner::plan_sources(
            &self.root,
            &self.inputs,
            &self.identity,
            self.sources.len(),
            &[source],
        )
    }

    /// Builds every admitted report into an explicit disposable staging root.
    ///
    /// # Errors
    ///
    /// Returns a typed source, plan, assembly, confinement, or I/O error.
    pub fn build_all(&self, staging_root: impl AsRef<Path>) -> Result<V2AssemblyResult> {
        self.assemble_all(staging_root.as_ref(), assembly::Operation::Build)
    }

    /// Builds one admitted report without touching unrelated staging subtrees.
    ///
    /// # Errors
    ///
    /// Returns a typed source, plan, assembly, confinement, or I/O error.
    pub fn build_report(
        &self,
        report_id: &str,
        staging_root: impl AsRef<Path>,
    ) -> Result<V2AssemblyResult> {
        self.assemble_report(report_id, staging_root.as_ref(), assembly::Operation::Build)
    }

    /// Checks every admitted staged report against a deterministic rebuild.
    ///
    /// # Errors
    ///
    /// Returns drift for any extra, missing, or changed staged byte.
    pub fn check_all(&self, staging_root: impl AsRef<Path>) -> Result<V2AssemblyResult> {
        self.assemble_all(staging_root.as_ref(), assembly::Operation::Check)
    }

    /// Checks one staged report without traversing unrelated report sources.
    ///
    /// # Errors
    ///
    /// Returns drift for any extra, missing, or changed selected-report byte.
    pub fn check_report(
        &self,
        report_id: &str,
        staging_root: impl AsRef<Path>,
    ) -> Result<V2AssemblyResult> {
        self.assemble_report(report_id, staging_root.as_ref(), assembly::Operation::Check)
    }

    /// Calculates the deterministic layered review roots for one staged report.
    ///
    /// # Errors
    ///
    /// Returns a typed source, staging, schema, or drift error.
    pub fn review_roots(
        &self,
        report_id: &str,
        staging_root: impl AsRef<Path>,
    ) -> Result<V2ReviewRoots> {
        publication::review_roots(self, report_id, staging_root.as_ref())
    }

    /// Publishes one production-domain approved report.
    ///
    /// # Errors
    ///
    /// Returns an error unless every review, transfer, staging, confinement,
    /// snapshot, and public-generation gate passes.
    pub fn publish_report(
        &self,
        report_id: &str,
        options: &V2PublicationOptions,
    ) -> Result<V2PublicationResult> {
        publication::publish(self, Some(report_id), options, V2TrustDomain::Production)
    }

    /// Publishes every production-domain approved report.
    ///
    /// # Errors
    ///
    /// Returns an error unless every selected report and exact-set gate passes.
    pub fn publish_all(&self, options: &V2PublicationOptions) -> Result<V2PublicationResult> {
        publication::publish(self, None, options, V2TrustDomain::Production)
    }

    /// Exercises one synthetic approved fixture in the isolated test domain.
    ///
    /// # Errors
    ///
    /// Returns an error unless the explicitly test-only fixture and all
    /// publication mechanics pass.
    pub fn publish_test_fixture_report(
        &self,
        report_id: &str,
        options: &V2PublicationOptions,
    ) -> Result<V2PublicationResult> {
        publication::publish(self, Some(report_id), options, V2TrustDomain::TestOnly)
    }

    /// Exercises every synthetic approved fixture in the isolated test domain.
    ///
    /// # Errors
    ///
    /// Returns an error unless the complete test-only set and mechanics pass.
    pub fn publish_all_test_fixtures(
        &self,
        options: &V2PublicationOptions,
    ) -> Result<V2PublicationResult> {
        publication::publish(self, None, options, V2TrustDomain::TestOnly)
    }

    fn assemble_all(
        &self,
        staging_root: &Path,
        operation: assembly::Operation,
    ) -> Result<V2AssemblyResult> {
        self.verify_inputs()?;
        let plan = self.plan_all()?;
        let sources = self.sources.values().collect::<Vec<_>>();
        assembly::execute(
            &self.root,
            staging_root,
            &self.inputs,
            &self.identity,
            &self.principals,
            &sources,
            &plan,
            operation,
        )
    }

    fn assemble_report(
        &self,
        report_id: &str,
        staging_root: &Path,
        operation: assembly::Operation,
    ) -> Result<V2AssemblyResult> {
        self.verify_inputs()?;
        let source = self.sources.get(report_id).ok_or_else(|| {
            AssuranceError::Invalid(format!("unknown v2 report ID '{report_id}'"))
        })?;
        let plan = self.plan_report(report_id)?;
        assembly::execute(
            &self.root,
            staging_root,
            &self.inputs,
            &self.identity,
            &self.principals,
            &[source],
            &plan,
            operation,
        )
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
        let principals_path = Path::new("assurance/v2/principals.yaml");
        let principals_value: serde_yaml::Value = parse_yaml(
            principals_path,
            &read_regular_confined(&self.root, principals_path)?,
        )?;
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
            let report: Report =
                parse_hydrated_yaml(&source.manifest_path, &manifest_bytes, &self.identity)?;
            validate_catalog_binding(source, &report)?;
            validate_report(&self.root, &report, &mut report_inputs)?;
            let report_value: serde_yaml::Value =
                parse_yaml(&source.manifest_path, &manifest_bytes)?;
            identity::verify_review_lock_current(
                &self.root,
                &self.identity,
                &source.id,
                &report_value,
                &principals_value,
            )?;
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
    trust_domain: V2TrustDomain,
    principal_registry: ContentIdentity,
    schemas: Vec<SchemaSource>,
    reports: Vec<ReportSource>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ContentIdentity {
    path: PathBuf,
    sha256: String,
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
    trust_domain: V2TrustDomain,
    fixture_only: bool,
    manifest_path: PathBuf,
    manifest_sha256: String,
}

fn validate_catalog_header(catalog: &V2Catalog) -> Result<()> {
    if catalog.schema_version != CATALOG_SCHEMA_VERSION
        || catalog.contract_version != CONTRACT_VERSION
    {
        return Err(AssuranceError::Invalid(
            "v2 catalog requires schema_version 4 and contract_version 4".to_owned(),
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
    if source.trust_domain == V2TrustDomain::TestOnly && !source.fixture_only {
        return Err(AssuranceError::Invalid(
            "test-only catalog reports must be fixture_only".to_owned(),
        ));
    }
    if source.trust_domain == V2TrustDomain::Production && source.fixture_only {
        return Err(AssuranceError::Invalid(
            "production catalog reports cannot be fixture_only".to_owned(),
        ));
    }
    Ok(())
}

fn validate_catalog_binding(source: &ReportSource, report: &Report) -> Result<()> {
    if source.id != report.id
        || source.version != report.version
        || source.title != report.title
        || source.owner != report.owner
        || source.trust_domain != report.trust_domain
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
        ("openwepp:assurance:v2:catalog:4", CATALOG_FIELDS),
        ("openwepp:assurance:v2:report:4", REPORT_FIELDS),
        ("openwepp:assurance:v2:result:1", RESULT_FIELDS),
        (
            "openwepp:assurance:v2:principals:2",
            PRINCIPAL_REGISTRY_FIELDS,
        ),
    ]);
    if sources.len() != expected.len() {
        return Err(AssuranceError::Invalid(
            "v2 catalog must bind exactly the catalog, report, result, and principal schemas"
                .to_owned(),
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
        "openwepp:assurance:v2:catalog:4" => CATALOG_SCHEMA_DEFINITIONS,
        "openwepp:assurance:v2:report:4" => REPORT_SCHEMA_DEFINITIONS,
        "openwepp:assurance:v2:result:1" => &[],
        "openwepp:assurance:v2:principals:2" => PRINCIPAL_SCHEMA_DEFINITIONS,
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
        "openwepp:assurance:v2:catalog:4" => vec![
            ("schema_version", serde_json::json!(CATALOG_SCHEMA_VERSION)),
            ("contract_version", serde_json::json!(CONTRACT_VERSION)),
            ("source_state", serde_json::json!(SOURCE_STATE)),
        ],
        "openwepp:assurance:v2:report:4" => vec![
            ("schema_version", serde_json::json!(REPORT_SCHEMA_VERSION)),
            ("contract_version", serde_json::json!(CONTRACT_VERSION)),
        ],
        "openwepp:assurance:v2:result:1" => {
            vec![("schema_version", serde_json::json!(RESULT_SCHEMA_VERSION))]
        }
        "openwepp:assurance:v2:principals:2" => {
            vec![(
                "schema_version",
                serde_json::json!(PRINCIPAL_SCHEMA_VERSION),
            )]
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
    trust_domain: V2TrustDomain,
    fixture_only: bool,
    reader_metadata: ReaderMetadata,
    authorship: Authorship,
    agent_assistance: AgentAssistance,
    manuscript: ContentSource,
    supplement: ContentSource,
    dependencies: Vec<Dependency>,
    units: Vec<Unit>,
    claims: Vec<Claim>,
    methods: Vec<Method>,
    results: Vec<ResultSource>,
    value_bindings: Vec<ValueBinding>,
    tables: Vec<Table>,
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
    value_binding_ids: Vec<String>,
    table_ids: Vec<String>,
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
struct ValueBinding {
    id: String,
    title: String,
    owner: String,
    result_id: String,
    value_id: String,
    unit_id: String,
    transform: String,
    display: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Table {
    id: String,
    title: String,
    owner: String,
    caption: String,
    alternative_text: String,
    row_header: String,
    columns: Vec<TableColumn>,
    rows: Vec<TableRow>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct TableColumn {
    label: String,
    #[serde(default)]
    unit_id: RequiredNullable<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct TableRow {
    label: String,
    value_binding_ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Figure {
    id: String,
    title: String,
    owner: String,
    kind: String,
    result_ids: Vec<String>,
    value_binding_ids: Vec<String>,
    visualization: String,
    #[serde(default)]
    research_object_id: RequiredNullable<String>,
    #[serde(default)]
    ancillary_object_id: RequiredNullable<String>,
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
    subject_root: RequiredNullable<String>,
    #[serde(default)]
    #[serde(rename = "review_charge")]
    charge: RequiredNullable<String>,
    #[serde(default)]
    build_maintainer_id: RequiredNullable<String>,
    material_producer_ids: Vec<String>,
    #[serde(default)]
    findings: Vec<Finding>,
    #[serde(default)]
    finding_ledger_root: RequiredNullable<String>,
    #[cfg(test)]
    #[serde(default)]
    approvals: Vec<Approval>,
    #[cfg(not(test))]
    #[serde(default)]
    approvals: Vec<serde_json::Value>,
    #[serde(default)]
    approval_lock_root: RequiredNullable<String>,
    independence_assessment: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Finding {
    id: String,
    summary: String,
    severity: String,
    disposition: String,
    #[serde(default)]
    rationale: RequiredNullable<String>,
    #[serde(default)]
    resolution: RequiredNullable<String>,
    #[serde(default)]
    verification: RequiredNullable<String>,
    #[serde(default)]
    verifier_id: RequiredNullable<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg(test)]
struct Approval {
    role: String,
    principal_id: String,
    finding_ledger_root: String,
    decision: String,
    competence_basis: String,
    independence_attestation: String,
    approved_on: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Publication {
    id: String,
    title: String,
    owner: String,
    state: String,
    #[serde(default)]
    approval_lock_root: RequiredNullable<String>,
    #[serde(default)]
    target_release_commit: RequiredNullable<String>,
    #[serde(default)]
    target_release_configuration: RequiredNullable<String>,
    #[serde(default)]
    prior_realization: RequiredNullable<String>,
    #[serde(default)]
    candidate_realization: RequiredNullable<String>,
    #[serde(default)]
    impact_assessment: RequiredNullable<String>,
    #[serde(default)]
    reproduction_disposition: RequiredNullable<String>,
    semantic_differences: Vec<String>,
    #[serde(default)]
    release_owner_id: RequiredNullable<String>,
    #[serde(default)]
    assurance_steward_id: RequiredNullable<String>,
    #[serde(default)]
    #[serde(rename = "publication_date")]
    date: RequiredNullable<String>,
    #[serde(default)]
    public_path: RequiredNullable<PathBuf>,
    #[serde(default)]
    release_transfer_root: RequiredNullable<String>,
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
    value_bindings: BTreeSet<String>,
    tables: BTreeSet<String>,
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
    read_identified(
        root,
        &Path::new("usersum").join(&report.reader_metadata.related_model_narrative),
        None,
        inputs,
    )?;
    Ok(())
}

fn validate_report_structure(report: &Report) -> Result<ReportIds> {
    validate_report_header(report)?;
    let ids = collect_report_ids(report)?;
    validate_authorship(&report.authorship, &report.lifecycle)?;
    validate_agent_assistance(&report.agent_assistance, &ids, &report.lifecycle)?;
    validate_report_sections(report, &ids)?;
    validate_review(&report.review)?;
    validate_publication(&report.publication, &report.lifecycle)?;
    let publication_matches = if report.lifecycle == "APPROVED" {
        matches!(report.publication.state.as_str(), DRAFT | "APPROVED")
    } else {
        report.publication.state == DRAFT
    };
    if report.review.state != report.lifecycle || !publication_matches {
        return Err(AssuranceError::Invalid(
            "report lifecycle, review state, and publication-transfer state disagree".to_owned(),
        ));
    }
    validate_no_unused(report)?;
    Ok(ids)
}

fn validate_report_sections(report: &Report, ids: &ReportIds) -> Result<()> {
    validate_content_contract(&report.manuscript, ids)?;
    validate_content_contract(&report.supplement, ids)?;
    for dependency in &report.dependencies {
        validate_dependency_shape(dependency)?;
    }
    for unit in &report.units {
        validate_unit(unit)?;
    }
    for claim in &report.claims {
        validate_claim(claim, ids)?;
    }
    for method in &report.methods {
        validate_method(method, ids)?;
    }
    for source in &report.results {
        validate_result_source(source, ids)?;
    }
    for binding in &report.value_bindings {
        validate_value_binding(binding, ids)?;
    }
    for table in &report.tables {
        validate_table(table, ids)?;
    }
    for figure in &report.figures {
        validate_figure(figure, report, ids)?;
    }
    for reference in &report.references {
        validate_reference(reference, ids)?;
    }
    for object in &report.research_objects {
        validate_research_object_contract(object, ids)?;
    }
    Ok(())
}

fn validate_report_header(report: &Report) -> Result<()> {
    if report.schema_version != REPORT_SCHEMA_VERSION || report.contract_version != CONTRACT_VERSION
    {
        return Err(AssuranceError::Invalid(
            "v2 report requires schema_version 4 and contract_version 4".to_owned(),
        ));
    }
    validate_identity(&report.id, &report.title, &report.owner, "report")?;
    validate_version(&report.version, "report")?;
    if !matches!(
        report.lifecycle.as_str(),
        DRAFT | "IN_REVIEW" | "APPROVED" | "WITHDRAWN" | "SUPERSEDED"
    ) {
        return Err(AssuranceError::Invalid(
            "v2 report lifecycle must be DRAFT, IN_REVIEW, APPROVED, WITHDRAWN, or SUPERSEDED"
                .to_owned(),
        ));
    }
    if (report.trust_domain == V2TrustDomain::TestOnly) != report.fixture_only {
        return Err(AssuranceError::Invalid(
            "test_only reports must be fixture_only and production reports must not be".to_owned(),
        ));
    }
    validate_reader_metadata(&report.reader_metadata)?;
    for (name, count) in [
        ("dependency", report.dependencies.len()),
        ("unit", report.units.len()),
        ("claim", report.claims.len()),
        ("method", report.methods.len()),
        ("result", report.results.len()),
        ("value binding", report.value_bindings.len()),
        ("table", report.tables.len()),
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
        value_bindings: report
            .value_bindings
            .iter()
            .map(|value| value.id.clone())
            .collect(),
        tables: report.tables.iter().map(|value| value.id.clone()).collect(),
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
    for value in &report.value_bindings {
        insert_identity(all, &value.id, &value.title, &value.owner, "value binding")?;
    }
    for value in &report.tables {
        insert_identity(all, &value.id, &value.title, &value.owner, "table")?;
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

fn validate_authorship(authorship: &Authorship, lifecycle: &str) -> Result<()> {
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
    if matches!(authorship.human_report_lead, RequiredNullable::Missing) {
        return Err(AssuranceError::Invalid(
            "required nullable field 'authorship human_report_lead' is missing".to_owned(),
        ));
    }
    if lifecycle == DRAFT {
        require_absent(
            &authorship.scientific_approver,
            "authorship scientific_approver",
        )?;
        let accountability_is_valid = match authorship.human_report_lead.as_deref() {
            None => authorship.accountability_state == "unassigned_blocks_review",
            Some(lead) => !lead.trim().is_empty() && authorship.accountability_state == "assigned",
        };
        if !accountability_is_valid || authorship.external_peer_review_claimed {
            return Err(AssuranceError::Invalid(
                "draft authorship must disclose assigned or explicitly unassigned human accountability and cannot claim external peer review"
                    .to_owned(),
            ));
        }
    } else {
        require_present_nonempty(
            authorship.human_report_lead.as_deref(),
            "authorship human_report_lead",
        )?;
        if authorship.accountability_state != "assigned" {
            return Err(AssuranceError::Invalid(
                "review-entry authorship requires assigned human accountability".to_owned(),
            ));
        }
    }
    Ok(())
}

fn validate_agent_assistance(
    assistance: &AgentAssistance,
    ids: &ReportIds,
    lifecycle: &str,
) -> Result<()> {
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
    if assistance.review_entry_authorized && !assistance.provenance_complete {
        return Err(AssuranceError::Invalid(
            "agent review-entry authorization requires complete provenance".to_owned(),
        ));
    }
    if lifecycle != DRAFT && !assistance.review_entry_authorized {
        return Err(AssuranceError::Invalid(
            "IN_REVIEW and APPROVED reports require authorized agent-assistance disposition"
                .to_owned(),
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
    for (kind, references, known, required) in [
        ("claim", &content.claim_ids, &ids.claims, true),
        ("method", &content.method_ids, &ids.methods, true),
        ("result", &content.result_ids, &ids.results, true),
        (
            "value binding",
            &content.value_binding_ids,
            &ids.value_bindings,
            true,
        ),
        ("table", &content.table_ids, &ids.tables, false),
        ("figure", &content.figure_ids, &ids.figures, true),
        ("reference", &content.reference_ids, &ids.references, true),
        (
            "research object",
            &content.research_object_ids,
            &ids.research_objects,
            true,
        ),
    ] {
        validate_reference_list(references, known, kind, required)?;
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
    require_unbound(&dependency.sha256, "external dependency sha256")?;
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
    require_unbound(&dependency.sha256, "restricted dependency sha256")?;
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
    if result.schema_version != RESULT_SCHEMA_VERSION {
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

fn validate_value_binding(binding: &ValueBinding, ids: &ReportIds) -> Result<()> {
    require_known(&binding.result_id, &ids.results, "result")?;
    validate_id(&binding.value_id, "result value")?;
    require_known(&binding.unit_id, &ids.units, "unit")?;
    if binding.transform != "identity" && binding.transform != "absolute" {
        return Err(AssuranceError::Invalid(format!(
            "value binding '{}' has unsupported transform",
            binding.id
        )));
    }
    validate_display(&binding.display).map(|_| ())
}

fn validate_display(display: &str) -> Result<(&str, Option<usize>)> {
    if display == "integer" {
        return Ok(("integer", None));
    }
    let (kind, precision) = display.split_once(':').ok_or_else(|| {
        AssuranceError::Invalid(format!("unsupported display precision '{display}'"))
    })?;
    if kind != "fixed" && kind != "scientific" {
        return Err(AssuranceError::Invalid(format!(
            "unsupported display precision '{display}'"
        )));
    }
    let precision = precision.parse::<usize>().map_err(|_| {
        AssuranceError::Invalid(format!("unsupported display precision '{display}'"))
    })?;
    if precision > 15
        || precision.to_string() != display.split_once(':').map_or("", |(_, value)| value)
    {
        return Err(AssuranceError::Invalid(format!(
            "unsupported display precision '{display}'"
        )));
    }
    Ok((kind, Some(precision)))
}

fn validate_table(table: &Table, ids: &ReportIds) -> Result<()> {
    require_nonempty(&table.caption, "table caption")?;
    require_nonempty(&table.alternative_text, "table alternative_text")?;
    require_nonempty(&table.row_header, "table row_header")?;
    if table.columns.is_empty() || table.rows.is_empty() {
        return Err(AssuranceError::Invalid(format!(
            "table '{}' requires columns and rows",
            table.id
        )));
    }
    validate_table_columns(table, ids)?;
    validate_table_rows(table, ids)
}

fn validate_table_columns(table: &Table, ids: &ReportIds) -> Result<()> {
    for column in &table.columns {
        require_nonempty(&column.label, "table column label")?;
        match &column.unit_id {
            RequiredNullable::Missing => {
                return Err(AssuranceError::Invalid(
                    "required nullable field 'table column unit_id' is missing".to_owned(),
                ));
            }
            RequiredNullable::Null => {}
            RequiredNullable::Value(unit_id) => require_known(unit_id, &ids.units, "unit")?,
        }
    }
    Ok(())
}

fn validate_table_rows(table: &Table, ids: &ReportIds) -> Result<()> {
    let mut row_labels = BTreeSet::new();
    for row in &table.rows {
        require_nonempty(&row.label, "table row label")?;
        require_unique(&mut row_labels, &row.label, "table row label")?;
        if row.value_binding_ids.len() != table.columns.len() {
            return Err(AssuranceError::Invalid(format!(
                "table '{}' row '{}' must contain one value binding per column",
                table.id, row.label
            )));
        }
        validate_reference_list(
            &row.value_binding_ids,
            &ids.value_bindings,
            "value binding",
            true,
        )?;
    }
    Ok(())
}

fn validate_figure(figure: &Figure, report: &Report, ids: &ReportIds) -> Result<()> {
    svg::validate_figure_contract(figure, report, ids)
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
    require_unbound(&object.sha256, "restricted research-object sha256")?;
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
    validate_reference_list(&object.result_ids, &ids.results, "result", false)?;
    validate_reference_list(&object.method_ids, &ids.methods, "method", true)?;
    validate_reference_list(
        &object.dependency_ids,
        &ids.dependencies,
        "dependency",
        true,
    )
}

fn validate_publication(publication: &Publication, lifecycle: &str) -> Result<()> {
    if publication.export_authorized || publication.vendoring_authorized {
        return Err(AssuranceError::Invalid(
            "ASSURE-04D does not authorize export or vendoring".to_owned(),
        ));
    }
    match lifecycle {
        "WITHDRAWN" if !publication.withdrawn || publication.supersedes.as_deref().is_some() => {
            return Err(AssuranceError::Invalid(
                "WITHDRAWN publication must set only the withdrawn terminal marker".to_owned(),
            ));
        }
        "SUPERSEDED" if publication.withdrawn || publication.supersedes.as_deref().is_none() => {
            return Err(AssuranceError::Invalid(
                "SUPERSEDED publication requires its superseding report identity".to_owned(),
            ));
        }
        "WITHDRAWN" | "SUPERSEDED" => {}
        _ => {
            if publication.withdrawn {
                return Err(AssuranceError::Invalid(
                    "nonterminal publication cannot be withdrawn".to_owned(),
                ));
            }
            require_absent(&publication.supersedes, "publication supersedes")?;
        }
    }
    match publication.state.as_str() {
        DRAFT => validate_draft_publication(publication),
        "APPROVED" => validate_approved_publication(publication),
        _ => Err(AssuranceError::Invalid(
            "publication state must be DRAFT or APPROVED".to_owned(),
        )),
    }
}

fn validate_draft_publication(publication: &Publication) -> Result<()> {
    require_generated_absent(&publication.approval_lock_root, "publication approval lock")?;
    require_generated_absent(
        &publication.release_transfer_root,
        "publication transfer root",
    )?;
    for (value, label) in [
        (
            &publication.target_release_commit,
            "publication release commit",
        ),
        (
            &publication.target_release_configuration,
            "publication release configuration",
        ),
        (
            &publication.prior_realization,
            "publication prior realization",
        ),
        (
            &publication.candidate_realization,
            "publication candidate realization",
        ),
        (
            &publication.impact_assessment,
            "publication impact assessment",
        ),
        (
            &publication.reproduction_disposition,
            "publication reproduction disposition",
        ),
        (&publication.release_owner_id, "publication release owner"),
        (
            &publication.assurance_steward_id,
            "publication assurance steward",
        ),
        (&publication.date, "publication date"),
    ] {
        require_absent(value, label)?;
    }
    require_absent(&publication.public_path, "publication public_path")?;
    if !publication.semantic_differences.is_empty() {
        return Err(AssuranceError::Invalid(
            "draft publication cannot claim semantic differences".to_owned(),
        ));
    }
    Ok(())
}

fn require_generated_absent<T>(value: &RequiredNullable<T>, name: &str) -> Result<()> {
    match value {
        RequiredNullable::Missing | RequiredNullable::Null => Ok(()),
        RequiredNullable::Value(_) => Err(AssuranceError::Invalid(format!(
            "generated field '{name}' cannot be stored in authored report source"
        ))),
    }
}

fn validate_approved_publication(publication: &Publication) -> Result<()> {
    require_generated_absent(&publication.approval_lock_root, "publication approval lock")?;
    require_generated_absent(
        &publication.release_transfer_root,
        "publication transfer root",
    )?;
    for (value, label) in [
        (
            publication.target_release_commit.as_deref(),
            "publication release commit",
        ),
        (
            publication.target_release_configuration.as_deref(),
            "publication release configuration",
        ),
        (
            publication.prior_realization.as_deref(),
            "publication prior realization",
        ),
        (
            publication.candidate_realization.as_deref(),
            "publication candidate realization",
        ),
        (
            publication.impact_assessment.as_deref(),
            "publication impact assessment",
        ),
        (
            publication.reproduction_disposition.as_deref(),
            "publication reproduction disposition",
        ),
        (
            publication.release_owner_id.as_deref(),
            "publication release owner",
        ),
        (
            publication.assurance_steward_id.as_deref(),
            "publication assurance steward",
        ),
    ] {
        require_present_nonempty(value, label)?;
    }
    let date = publication.date.as_deref().ok_or_else(|| {
        AssuranceError::Invalid("approved publication requires publication date".to_owned())
    })?;
    validate_date(date, "publication date")?;
    let path = publication.public_path.as_deref().ok_or_else(|| {
        AssuranceError::Invalid("approved publication requires public path".to_owned())
    })?;
    validate_relative(path)?;
    if publication.semantic_differences.is_empty()
        || publication
            .semantic_differences
            .iter()
            .any(|value| value.trim().is_empty())
    {
        return Err(AssuranceError::Invalid(
            "approved publication requires explicit semantic-difference disposition".to_owned(),
        ));
    }
    Ok(())
}

struct UsedReportRecords {
    claims: BTreeSet<String>,
    methods: BTreeSet<String>,
    results: BTreeSet<String>,
    value_bindings: BTreeSet<String>,
    tables: BTreeSet<String>,
    figures: BTreeSet<String>,
    references: BTreeSet<String>,
    research_objects: BTreeSet<String>,
    dependencies: BTreeSet<String>,
    units: BTreeSet<String>,
}

fn collect_used_records(report: &Report) -> UsedReportRecords {
    let contents = [&report.manuscript, &report.supplement];
    let claims = union_lists(contents.iter().map(|content| &content.claim_ids));
    let methods = union_lists(
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
    let mut results = union_lists(
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
    results.extend(
        report
            .value_bindings
            .iter()
            .map(|binding| binding.result_id.clone()),
    );
    let value_bindings = union_lists(
        contents
            .iter()
            .map(|content| &content.value_binding_ids)
            .chain(
                report
                    .tables
                    .iter()
                    .flat_map(|table| table.rows.iter())
                    .map(|row| &row.value_binding_ids),
            )
            .chain(
                report
                    .figures
                    .iter()
                    .map(|figure| &figure.value_binding_ids),
            ),
    );
    let tables = union_lists(contents.iter().map(|content| &content.table_ids));
    let figures = union_lists(contents.iter().map(|content| &content.figure_ids));
    let references = union_lists(
        contents
            .iter()
            .map(|content| &content.reference_ids)
            .chain(report.claims.iter().map(|claim| &claim.reference_ids)),
    );
    let research_objects = union_lists(contents.iter().map(|content| &content.research_object_ids));
    let dependencies = used_dependencies(report);
    let mut units = union_lists(
        report
            .claims
            .iter()
            .map(|claim| &claim.unit_ids)
            .chain(report.methods.iter().map(|method| &method.unit_ids))
            .chain(report.results.iter().map(|result| &result.unit_ids)),
    );
    units.extend(
        report
            .value_bindings
            .iter()
            .map(|binding| binding.unit_id.clone()),
    );
    units.extend(report.tables.iter().flat_map(|table| {
        table
            .columns
            .iter()
            .filter_map(|column| match &column.unit_id {
                RequiredNullable::Value(unit_id) => Some(unit_id.clone()),
                RequiredNullable::Missing | RequiredNullable::Null => None,
            })
    }));
    UsedReportRecords {
        claims,
        methods,
        results,
        value_bindings,
        tables,
        figures,
        references,
        research_objects,
        dependencies,
        units,
    }
}

fn validate_no_unused(report: &Report) -> Result<()> {
    let used = collect_used_records(report);
    require_all_used(
        "claim",
        report.claims.iter().map(|value| &value.id),
        &used.claims,
    )?;
    require_all_used(
        "method",
        report.methods.iter().map(|value| &value.id),
        &used.methods,
    )?;
    require_all_used(
        "result",
        report.results.iter().map(|value| &value.id),
        &used.results,
    )?;
    require_all_used(
        "value binding",
        report.value_bindings.iter().map(|value| &value.id),
        &used.value_bindings,
    )?;
    require_all_used(
        "table",
        report.tables.iter().map(|value| &value.id),
        &used.tables,
    )?;
    require_all_used(
        "figure",
        report.figures.iter().map(|value| &value.id),
        &used.figures,
    )?;
    require_all_used(
        "reference",
        report.references.iter().map(|value| &value.id),
        &used.references,
    )?;
    require_all_used(
        "research object",
        report.research_objects.iter().map(|value| &value.id),
        &used.research_objects,
    )?;
    require_all_used(
        "dependency",
        report.dependencies.iter().map(|value| &value.id),
        &used.dependencies,
    )?;
    require_all_used(
        "unit",
        report.units.iter().map(|value| &value.id),
        &used.units,
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

fn require_unbound<T>(value: &RequiredNullable<T>, name: &str) -> Result<()> {
    match value {
        RequiredNullable::Missing | RequiredNullable::Null => Ok(()),
        RequiredNullable::Value(_) => {
            Err(AssuranceError::Invalid(format!("{name} must be absent")))
        }
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

fn parse_hydrated_yaml<T: for<'de> Deserialize<'de>>(
    path: impl Into<PathBuf>,
    bytes: &[u8],
    identity: &IdentityLock,
) -> Result<T> {
    let path = path.into();
    let mut value: serde_yaml::Value =
        serde_yaml::from_slice(bytes).map_err(|error| AssuranceError::Parse {
            path: path.clone(),
            message: error.to_string(),
        })?;
    identity.hydrate_yaml(&mut value)?;
    serde_yaml::from_value(value).map_err(|error| AssuranceError::Parse {
        path,
        message: error.to_string(),
    })
}

fn parse_json<T: for<'de> Deserialize<'de>>(path: &Path, bytes: &[u8]) -> Result<T> {
    serde_json::from_slice(bytes).map_err(|error| AssuranceError::Parse {
        path: path.to_path_buf(),
        message: error.to_string(),
    })
}
