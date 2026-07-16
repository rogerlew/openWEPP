use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use serde::Serialize;

use super::{
    ContentSource, Dependency, DependencyKind, Report, ReportIds, ReportSource, ResearchObject,
    ResultObject, digest_input_set, parse_json, parse_yaml, read_regular_confined,
    validate_catalog_binding, validate_report_structure, validate_result_object,
};
use crate::{AssuranceError, Result, sha256_bytes};

const PUBLICATION_STATE: &str = "v1_retired_zero_reports";
const PLANNER_TOOL_ID: &str = "tool:openwepp-assurance-planner:1";
const SCHEMA_PREFIX: &str = "assurance/v2/schemas/";

/// Mechanical state of one dependency-plan node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum V2PlanState {
    Current,
    Stale,
    Blocked,
    Selected,
}

impl fmt::Display for V2PlanState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Blocked => "blocked",
            Self::Selected => "selected",
        })
    }
}

/// One dependency node in stable dependency-first order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct V2PlanNode {
    pub id: String,
    pub kind: String,
    pub state: V2PlanState,
    pub identity: Option<String>,
    pub declared_identity: Option<String>,
    pub dependencies: Vec<String>,
    pub reason: String,
}

/// Deterministic plan for one admitted report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct V2ReportPlan {
    pub id: String,
    pub version: String,
    pub state: V2PlanState,
    pub source_root_sha256: Option<String>,
    pub target_id: String,
    pub nodes: Vec<V2PlanNode>,
}

/// Deterministic plan for a named selection or all admitted reports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct V2Plan {
    pub publication_state: String,
    pub public_report_count: usize,
    pub total_report_count: usize,
    pub selected_report_count: usize,
    pub reports: Vec<V2ReportPlan>,
}

impl V2Plan {
    /// Renders the typed plan for maintainers without absolute paths.
    #[must_use]
    pub fn render(&self) -> String {
        let mut output = format!(
            "plan: PASS\npublication_state: {}\npublic_reports: {}\n\
             v2_reports_total: {}\nv2_reports_selected: {}\nreports:\n",
            self.publication_state,
            self.public_report_count,
            self.total_report_count,
            self.selected_report_count
        );
        for report in &self.reports {
            let root = report
                .source_root_sha256
                .as_deref()
                .unwrap_or("unavailable");
            let _ = writeln!(
                output,
                "  - id={} version={} state={} target={} source_root_sha256={root}",
                report.id, report.version, report.state, report.target_id
            );
            for (order, node) in report.nodes.iter().enumerate() {
                let identity = node.identity.as_deref().unwrap_or("unavailable");
                let declared = node
                    .declared_identity
                    .as_deref()
                    .unwrap_or("not_applicable");
                let dependencies = if node.dependencies.is_empty() {
                    "none".to_owned()
                } else {
                    node.dependencies.join(",")
                };
                let _ = writeln!(
                    output,
                    "    - order={order} id={} kind={} state={} identity={} \
                     declared_identity={} dependencies={} reason={}",
                    node.id, node.kind, node.state, identity, declared, dependencies, node.reason
                );
            }
        }
        output
    }

    /// Renders the same typed plan as deterministic pretty JSON.
    ///
    /// # Errors
    ///
    /// Returns a typed serialization error if the plan cannot be encoded.
    pub fn render_json(&self) -> Result<String> {
        let mut output = serde_json::to_string_pretty(self)
            .map_err(|error| AssuranceError::Invalid(format!("plan JSON: {error}")))?;
        output.push('\n');
        Ok(output)
    }
}

pub(super) fn plan_sources(
    root: &Path,
    shared_inputs: &BTreeMap<PathBuf, String>,
    total_report_count: usize,
    sources: &[&ReportSource],
) -> Result<V2Plan> {
    let mut reports = sources
        .iter()
        .map(|source| plan_report_source(root, shared_inputs, source))
        .collect::<Result<Vec<_>>>()?;
    reports.sort_by(|left, right| left.id.cmp(&right.id));
    Ok(V2Plan {
        publication_state: PUBLICATION_STATE.to_owned(),
        public_report_count: 0,
        total_report_count,
        selected_report_count: reports.len(),
        reports,
    })
}

fn plan_report_source(
    root: &Path,
    shared_inputs: &BTreeMap<PathBuf, String>,
    source: &ReportSource,
) -> Result<V2ReportPlan> {
    let manifest_id = format!("source:manifest:{}", source.id);
    let target_id = format!("report:{}", source.id);
    let (mut manifest, bytes) = local_node(
        root,
        &manifest_id,
        "report_manifest",
        &source.manifest_path,
        &source.manifest_sha256,
        BTreeSet::new(),
    );
    let Some(bytes) = bytes else {
        return blocked_manifest_plan(source, shared_inputs, manifest, &target_id);
    };
    let report: Report = match parse_yaml(&source.manifest_path, &bytes) {
        Ok(report) => report,
        Err(error) => {
            manifest.intrinsic = V2PlanState::Blocked;
            manifest.reason = format!(
                "{} cannot be parsed for planning: {error}",
                source.manifest_path.display()
            );
            return blocked_manifest_plan(source, shared_inputs, manifest, &target_id);
        }
    };
    validate_catalog_binding(source, &report)?;
    let ids = validate_report_structure(&report)?;

    let observed_manifest = sha256_bytes(&bytes);
    let mut graph = Graph::default();
    add_shared_nodes(&mut graph, shared_inputs)?;
    graph.add(manifest)?;
    add_report_nodes(
        root,
        &mut graph,
        &report,
        &ids,
        &manifest_id,
        &observed_manifest,
    )?;
    let mut target_dependencies = BTreeSet::from([
        report.authorship.id.clone(),
        report.agent_assistance.id.clone(),
        report.manuscript.id.clone(),
        report.supplement.id.clone(),
        report.review.id.clone(),
        report.publication.id.clone(),
        PLANNER_TOOL_ID.to_owned(),
    ]);
    target_dependencies.extend(schema_node_ids(shared_inputs));
    graph.add(embedded_node(
        &target_id,
        "report_target",
        &observed_manifest,
        &source.manifest_sha256,
        target_dependencies,
    ))?;
    finish_report_plan(&report.id, &report.version, &target_id, graph)
}

fn blocked_manifest_plan(
    source: &ReportSource,
    shared_inputs: &BTreeMap<PathBuf, String>,
    manifest: DraftNode,
    target_id: &str,
) -> Result<V2ReportPlan> {
    let mut graph = Graph::default();
    add_shared_nodes(&mut graph, shared_inputs)?;
    let manifest_id = manifest.id.clone();
    graph.add(manifest)?;
    let mut dependencies = BTreeSet::from([manifest_id, PLANNER_TOOL_ID.to_owned()]);
    dependencies.extend(schema_node_ids(shared_inputs));
    graph.add(DraftNode::current(
        target_id,
        "report_target",
        None,
        None,
        dependencies,
        "report target is declared by the v2 catalog",
    ))?;
    finish_report_plan(&source.id, &source.version, target_id, graph)
}

fn finish_report_plan(
    report_id: &str,
    version: &str,
    target_id: &str,
    graph: Graph,
) -> Result<V2ReportPlan> {
    let nodes = graph.finalize(target_id)?;
    let target = nodes
        .iter()
        .find(|node| node.id == target_id)
        .ok_or_else(|| AssuranceError::Invalid("planner target disappeared".to_owned()))?;
    let source_root_sha256 = source_root(&nodes, target_id);
    Ok(V2ReportPlan {
        id: report_id.to_owned(),
        version: version.to_owned(),
        state: target.state,
        source_root_sha256,
        target_id: target_id.to_owned(),
        nodes,
    })
}

fn add_shared_nodes(graph: &mut Graph, shared_inputs: &BTreeMap<PathBuf, String>) -> Result<()> {
    graph.add(DraftNode::current(
        PLANNER_TOOL_ID,
        "planner_tool",
        Some("planner-contract:1".to_owned()),
        Some("planner-contract:1".to_owned()),
        BTreeSet::new(),
        "planner contract identity is current",
    ))?;
    for (path, digest) in shared_inputs {
        if path.to_string_lossy().starts_with(SCHEMA_PREFIX) {
            graph.add(DraftNode::current(
                &schema_node_id(path),
                "schema",
                Some(digest.clone()),
                Some(digest.clone()),
                BTreeSet::new(),
                "schema bytes matched the catalog identity during repository admission",
            ))?;
        }
    }
    Ok(())
}

fn add_report_nodes(
    root: &Path,
    graph: &mut Graph,
    report: &Report,
    ids: &ReportIds,
    manifest_id: &str,
    manifest_digest: &str,
) -> Result<()> {
    let expected_manifest = report_manifest_declared_identity(graph, manifest_id)?;
    add_accountability_nodes(
        graph,
        report,
        manifest_id,
        manifest_digest,
        &expected_manifest,
    )?;
    add_dependency_nodes(root, graph, report, manifest_id)?;
    add_unit_nodes(
        graph,
        report,
        manifest_id,
        manifest_digest,
        &expected_manifest,
    )?;
    add_claim_nodes(
        graph,
        report,
        manifest_id,
        manifest_digest,
        &expected_manifest,
    )?;
    add_method_nodes(
        graph,
        report,
        manifest_id,
        manifest_digest,
        &expected_manifest,
    )?;
    add_result_nodes(root, graph, report, ids, manifest_id)?;
    add_value_binding_nodes(
        graph,
        report,
        manifest_id,
        manifest_digest,
        &expected_manifest,
    )?;
    add_table_nodes(
        graph,
        report,
        manifest_id,
        manifest_digest,
        &expected_manifest,
    )?;
    add_figure_nodes(
        graph,
        report,
        manifest_id,
        manifest_digest,
        &expected_manifest,
    )?;
    add_reference_nodes(
        graph,
        report,
        manifest_id,
        manifest_digest,
        &expected_manifest,
    )?;
    add_research_object_nodes(
        root,
        graph,
        report,
        manifest_id,
        manifest_digest,
        &expected_manifest,
    )?;
    add_content_node(root, graph, &report.manuscript, manifest_id)?;
    add_content_node(root, graph, &report.supplement, manifest_id)?;
    add_lifecycle_nodes(
        graph,
        report,
        manifest_id,
        manifest_digest,
        &expected_manifest,
    )
}

fn report_manifest_declared_identity(graph: &Graph, manifest_id: &str) -> Result<String> {
    graph
        .nodes
        .get(manifest_id)
        .and_then(|node| node.declared_identity.clone())
        .ok_or_else(|| AssuranceError::Invalid("manifest declaration missing".to_owned()))
}

fn add_accountability_nodes(
    graph: &mut Graph,
    report: &Report,
    manifest_id: &str,
    observed_manifest: &str,
    expected_manifest: &str,
) -> Result<()> {
    graph.add(embedded_node(
        &report.authorship.id,
        "authorship",
        observed_manifest,
        expected_manifest,
        BTreeSet::from([manifest_id.to_owned()]),
    ))?;
    let mut assistance_dependencies = BTreeSet::from([manifest_id.to_owned()]);
    assistance_dependencies.extend(report.agent_assistance.input_dependency_ids.iter().cloned());
    assistance_dependencies.insert(report.agent_assistance.exact_output_dependency_id.clone());
    graph.add(embedded_node(
        &report.agent_assistance.id,
        "agent_assistance",
        observed_manifest,
        expected_manifest,
        assistance_dependencies,
    ))
}

fn add_dependency_nodes(
    root: &Path,
    graph: &mut Graph,
    report: &Report,
    manifest_id: &str,
) -> Result<()> {
    for dependency in &report.dependencies {
        let dependencies = BTreeSet::from([manifest_id.to_owned()]);
        let node = match dependency.kind {
            DependencyKind::LocalContent => local_dependency_node(root, dependency, dependencies)?,
            DependencyKind::ExternalImmutable => immutable_dependency_node(
                dependency,
                "external_dependency",
                dependencies,
                "immutable external identity is declared",
            )?,
            DependencyKind::Restricted => immutable_dependency_node(
                dependency,
                "restricted_dependency",
                dependencies,
                "restricted immutable identity is declared; protected content is not read",
            )?,
        };
        graph.add(node)?;
    }
    Ok(())
}

fn local_dependency_node(
    root: &Path,
    dependency: &Dependency,
    dependencies: BTreeSet<String>,
) -> Result<DraftNode> {
    let path = dependency.path.as_deref().ok_or_else(|| {
        AssuranceError::Invalid(format!("dependency '{}' has no path", dependency.id))
    })?;
    let digest = dependency.sha256.as_deref().ok_or_else(|| {
        AssuranceError::Invalid(format!("dependency '{}' has no digest", dependency.id))
    })?;
    Ok(local_node(
        root,
        &dependency.id,
        "local_dependency",
        path,
        digest,
        dependencies,
    )
    .0)
}

fn immutable_dependency_node(
    dependency: &Dependency,
    kind: &str,
    dependencies: BTreeSet<String>,
    reason: &str,
) -> Result<DraftNode> {
    let identity = dependency.immutable_identity.as_deref().ok_or_else(|| {
        AssuranceError::Invalid(format!(
            "dependency '{}' has no immutable identity",
            dependency.id
        ))
    })?;
    Ok(DraftNode::current(
        &dependency.id,
        kind,
        Some(identity.to_owned()),
        Some(identity.to_owned()),
        dependencies,
        reason,
    ))
}

fn add_unit_nodes(
    graph: &mut Graph,
    report: &Report,
    manifest_id: &str,
    observed_manifest: &str,
    expected_manifest: &str,
) -> Result<()> {
    for unit in &report.units {
        graph.add(embedded_node(
            &unit.id,
            "unit",
            observed_manifest,
            expected_manifest,
            BTreeSet::from([manifest_id.to_owned()]),
        ))?;
    }
    Ok(())
}

fn add_claim_nodes(
    graph: &mut Graph,
    report: &Report,
    manifest_id: &str,
    observed_manifest: &str,
    expected_manifest: &str,
) -> Result<()> {
    for claim in &report.claims {
        let mut dependencies = BTreeSet::from([manifest_id.to_owned()]);
        dependencies.extend(claim.method_ids.iter().cloned());
        dependencies.extend(claim.result_ids.iter().cloned());
        dependencies.extend(claim.dependency_ids.iter().cloned());
        dependencies.extend(claim.unit_ids.iter().cloned());
        dependencies.extend(claim.reference_ids.iter().cloned());
        graph.add(embedded_node(
            &claim.id,
            "claim",
            observed_manifest,
            expected_manifest,
            dependencies,
        ))?;
    }
    Ok(())
}

fn add_method_nodes(
    graph: &mut Graph,
    report: &Report,
    manifest_id: &str,
    observed_manifest: &str,
    expected_manifest: &str,
) -> Result<()> {
    for method in &report.methods {
        let mut dependencies = BTreeSet::from([manifest_id.to_owned()]);
        dependencies.extend(method.dependency_ids.iter().cloned());
        dependencies.extend(method.unit_ids.iter().cloned());
        graph.add(embedded_node(
            &method.id,
            "method",
            observed_manifest,
            expected_manifest,
            dependencies,
        ))?;
    }
    Ok(())
}

fn add_result_nodes(
    root: &Path,
    graph: &mut Graph,
    report: &Report,
    ids: &ReportIds,
    manifest_id: &str,
) -> Result<()> {
    for result in &report.results {
        let mut dependencies = BTreeSet::from([
            manifest_id.to_owned(),
            result.method_id.clone(),
            result.software_realization.clone(),
        ]);
        dependencies.extend(result.dependency_ids.iter().cloned());
        dependencies.extend(result.unit_ids.iter().cloned());
        let (node, bytes) = local_node(
            root,
            &result.id,
            "result",
            &result.path,
            &result.sha256,
            dependencies,
        );
        if node.intrinsic == V2PlanState::Current {
            let bytes = bytes.ok_or_else(|| {
                AssuranceError::Invalid(format!(
                    "current result '{}' has no readable bytes",
                    result.id
                ))
            })?;
            let object: ResultObject = parse_json(&result.path, &bytes)?;
            validate_result_object(result, &object, ids)?;
        }
        graph.add(node)?;
    }
    Ok(())
}

fn add_figure_nodes(
    graph: &mut Graph,
    report: &Report,
    manifest_id: &str,
    observed_manifest: &str,
    expected_manifest: &str,
) -> Result<()> {
    for figure in &report.figures {
        let mut dependencies = BTreeSet::from([manifest_id.to_owned()]);
        dependencies.extend(figure.result_ids.iter().cloned());
        dependencies.extend(figure.value_binding_ids.iter().cloned());
        graph.add(embedded_node(
            &figure.id,
            "figure",
            observed_manifest,
            expected_manifest,
            dependencies,
        ))?;
    }
    Ok(())
}

fn add_value_binding_nodes(
    graph: &mut Graph,
    report: &Report,
    manifest_id: &str,
    observed_manifest: &str,
    expected_manifest: &str,
) -> Result<()> {
    for binding in &report.value_bindings {
        graph.add(embedded_node(
            &binding.id,
            "value_binding",
            observed_manifest,
            expected_manifest,
            BTreeSet::from([
                manifest_id.to_owned(),
                binding.result_id.clone(),
                binding.unit_id.clone(),
            ]),
        ))?;
    }
    Ok(())
}

fn add_table_nodes(
    graph: &mut Graph,
    report: &Report,
    manifest_id: &str,
    observed_manifest: &str,
    expected_manifest: &str,
) -> Result<()> {
    for table in &report.tables {
        let mut dependencies = BTreeSet::from([manifest_id.to_owned()]);
        for row in &table.rows {
            dependencies.extend(row.value_binding_ids.iter().cloned());
        }
        for column in &table.columns {
            if let super::RequiredNullable::Value(unit_id) = &column.unit_id {
                dependencies.insert(unit_id.clone());
            }
        }
        graph.add(embedded_node(
            &table.id,
            "table",
            observed_manifest,
            expected_manifest,
            dependencies,
        ))?;
    }
    Ok(())
}

fn add_reference_nodes(
    graph: &mut Graph,
    report: &Report,
    manifest_id: &str,
    observed_manifest: &str,
    expected_manifest: &str,
) -> Result<()> {
    for reference in &report.references {
        graph.add(embedded_node(
            &reference.id,
            "reference",
            observed_manifest,
            expected_manifest,
            BTreeSet::from([manifest_id.to_owned(), reference.dependency_id.clone()]),
        ))?;
    }
    Ok(())
}

fn add_research_object_nodes(
    root: &Path,
    graph: &mut Graph,
    report: &Report,
    manifest_id: &str,
    observed_manifest: &str,
    expected_manifest: &str,
) -> Result<()> {
    for object in &report.research_objects {
        let mut dependencies = BTreeSet::from([manifest_id.to_owned()]);
        dependencies.extend(object.result_ids.iter().cloned());
        dependencies.extend(object.method_ids.iter().cloned());
        dependencies.extend(object.dependency_ids.iter().cloned());
        let node = research_object_node(
            root,
            object,
            dependencies,
            observed_manifest,
            expected_manifest,
        )?;
        graph.add(node)?;
    }
    Ok(())
}

fn research_object_node(
    root: &Path,
    object: &ResearchObject,
    dependencies: BTreeSet<String>,
    observed_manifest: &str,
    expected_manifest: &str,
) -> Result<DraftNode> {
    if object.access == "restricted" {
        return Ok(embedded_node(
            &object.id,
            "restricted_research_object",
            observed_manifest,
            expected_manifest,
            dependencies,
        ));
    }
    let path = object.path.as_deref().ok_or_else(|| {
        AssuranceError::Invalid(format!("research object '{}' has no path", object.id))
    })?;
    let digest = object.sha256.as_deref().ok_or_else(|| {
        AssuranceError::Invalid(format!("research object '{}' has no digest", object.id))
    })?;
    Ok(local_node(
        root,
        &object.id,
        "research_object",
        path,
        digest,
        dependencies,
    )
    .0)
}

fn add_content_node(
    root: &Path,
    graph: &mut Graph,
    content: &ContentSource,
    manifest_id: &str,
) -> Result<()> {
    let mut dependencies = BTreeSet::from([manifest_id.to_owned()]);
    dependencies.extend(content.claim_ids.iter().cloned());
    dependencies.extend(content.method_ids.iter().cloned());
    dependencies.extend(content.result_ids.iter().cloned());
    dependencies.extend(content.value_binding_ids.iter().cloned());
    dependencies.extend(content.table_ids.iter().cloned());
    dependencies.extend(content.figure_ids.iter().cloned());
    dependencies.extend(content.reference_ids.iter().cloned());
    dependencies.extend(content.research_object_ids.iter().cloned());
    graph.add(
        local_node(
            root,
            &content.id,
            "authored_content",
            &content.path,
            &content.sha256,
            dependencies,
        )
        .0,
    )
}

fn add_lifecycle_nodes(
    graph: &mut Graph,
    report: &Report,
    manifest_id: &str,
    observed_manifest: &str,
    expected_manifest: &str,
) -> Result<()> {
    graph.add(embedded_node(
        &report.review.id,
        "review",
        observed_manifest,
        expected_manifest,
        BTreeSet::from([manifest_id.to_owned()]),
    ))?;
    graph.add(embedded_node(
        &report.publication.id,
        "publication",
        observed_manifest,
        expected_manifest,
        BTreeSet::from([manifest_id.to_owned(), report.review.id.clone()]),
    ))
}

fn local_node(
    root: &Path,
    id: &str,
    kind: &str,
    path: &Path,
    declared: &str,
    dependencies: BTreeSet<String>,
) -> (DraftNode, Option<Vec<u8>>) {
    match read_regular_confined(root, path) {
        Ok(bytes) => {
            let observed = sha256_bytes(&bytes);
            let (state, reason) = if observed == declared {
                (
                    V2PlanState::Current,
                    format!("{} matches declared SHA-256", path.display()),
                )
            } else {
                (
                    V2PlanState::Stale,
                    format!(
                        "{} observed SHA-256 {observed} differs from declared {declared}",
                        path.display()
                    ),
                )
            };
            (
                DraftNode {
                    id: id.to_owned(),
                    kind: kind.to_owned(),
                    intrinsic: state,
                    identity: Some(observed),
                    declared_identity: Some(declared.to_owned()),
                    dependencies,
                    reason,
                },
                Some(bytes),
            )
        }
        Err(error) => (
            DraftNode {
                id: id.to_owned(),
                kind: kind.to_owned(),
                intrinsic: V2PlanState::Blocked,
                identity: None,
                declared_identity: Some(declared.to_owned()),
                dependencies,
                reason: format!("{} is unavailable: {error}", path.display()),
            },
            None,
        ),
    }
}

fn embedded_node(
    id: &str,
    kind: &str,
    observed_manifest: &str,
    expected_manifest: &str,
    dependencies: BTreeSet<String>,
) -> DraftNode {
    DraftNode::current(
        id,
        kind,
        Some(embedded_identity(observed_manifest, kind, id)),
        Some(embedded_identity(expected_manifest, kind, id)),
        dependencies,
        "logical record identity is bound to the report manifest",
    )
}

fn embedded_identity(manifest: &str, kind: &str, id: &str) -> String {
    sha256_bytes(format!("v2-embedded:1\0{manifest}\0{kind}\0{id}").as_bytes())
}

fn schema_node_ids(inputs: &BTreeMap<PathBuf, String>) -> BTreeSet<String> {
    inputs
        .keys()
        .filter(|path| path.to_string_lossy().starts_with(SCHEMA_PREFIX))
        .map(|path| schema_node_id(path))
        .collect()
}

fn schema_node_id(path: &Path) -> String {
    format!("source:schema:{}", path.display())
}

fn source_root(nodes: &[V2PlanNode], target_id: &str) -> Option<String> {
    if nodes.iter().any(|node| node.state == V2PlanState::Blocked) {
        return None;
    }
    let inputs = nodes
        .iter()
        .filter(|node| node.id != target_id)
        .filter_map(|node| {
            node.identity
                .as_ref()
                .map(|identity| (PathBuf::from(&node.id), identity.clone()))
        })
        .collect::<BTreeMap<_, _>>();
    Some(digest_input_set("planner-report-root:1", &inputs))
}

#[derive(Debug)]
struct DraftNode {
    id: String,
    kind: String,
    intrinsic: V2PlanState,
    identity: Option<String>,
    declared_identity: Option<String>,
    dependencies: BTreeSet<String>,
    reason: String,
}

impl DraftNode {
    fn current(
        id: &str,
        kind: &str,
        identity: Option<String>,
        declared_identity: Option<String>,
        dependencies: BTreeSet<String>,
        reason: &str,
    ) -> Self {
        Self {
            id: id.to_owned(),
            kind: kind.to_owned(),
            intrinsic: V2PlanState::Current,
            identity,
            declared_identity,
            dependencies,
            reason: reason.to_owned(),
        }
    }
}

#[derive(Default)]
struct Graph {
    nodes: BTreeMap<String, DraftNode>,
}

impl Graph {
    fn add(&mut self, node: DraftNode) -> Result<()> {
        let id = node.id.clone();
        if self.nodes.insert(id.clone(), node).is_some() {
            return Err(AssuranceError::Invalid(format!(
                "duplicate planner node '{id}'"
            )));
        }
        Ok(())
    }

    fn finalize(self, target_id: &str) -> Result<Vec<V2PlanNode>> {
        self.validate_edges()?;
        self.validate_reachability(target_id)?;
        let order = self.topological_order()?;
        self.resolve_states(order)
    }

    fn validate_edges(&self) -> Result<()> {
        for node in self.nodes.values() {
            for dependency in &node.dependencies {
                if !self.nodes.contains_key(dependency) {
                    return Err(AssuranceError::Invalid(format!(
                        "missing planner edge destination '{}' required by '{}'",
                        dependency, node.id
                    )));
                }
            }
        }
        Ok(())
    }

    fn validate_reachability(&self, target_id: &str) -> Result<()> {
        if !self.nodes.contains_key(target_id) {
            return Err(AssuranceError::Invalid(format!(
                "unknown planner target '{target_id}'"
            )));
        }
        let mut reachable = BTreeSet::new();
        let mut pending = vec![target_id.to_owned()];
        while let Some(id) = pending.pop() {
            if !reachable.insert(id.clone()) {
                continue;
            }
            let node = self
                .nodes
                .get(&id)
                .ok_or_else(|| AssuranceError::Invalid(format!("missing planner node '{id}'")))?;
            pending.extend(node.dependencies.iter().cloned());
        }
        if let Some(unused) = self.nodes.keys().find(|id| !reachable.contains(*id)) {
            return Err(AssuranceError::Invalid(format!(
                "unused planner node '{unused}'"
            )));
        }
        Ok(())
    }

    fn topological_order(&self) -> Result<Vec<String>> {
        let mut remaining = self
            .nodes
            .iter()
            .map(|(id, node)| (id.clone(), node.dependencies.len()))
            .collect::<BTreeMap<_, _>>();
        let mut consumers = BTreeMap::<String, BTreeSet<String>>::new();
        for node in self.nodes.values() {
            for dependency in &node.dependencies {
                consumers
                    .entry(dependency.clone())
                    .or_default()
                    .insert(node.id.clone());
            }
        }
        let mut ready = remaining
            .iter()
            .filter(|(_, count)| **count == 0)
            .map(|(id, _)| id.clone())
            .collect::<BTreeSet<_>>();
        let mut order = Vec::with_capacity(self.nodes.len());
        while let Some(id) = ready.pop_first() {
            order.push(id.clone());
            for consumer in consumers.get(&id).into_iter().flatten() {
                let count = remaining.get_mut(consumer).ok_or_else(|| {
                    AssuranceError::Invalid("planner ordering state missing".to_owned())
                })?;
                *count = count.checked_sub(1).ok_or_else(|| {
                    AssuranceError::Invalid("planner ordering underflow".to_owned())
                })?;
                if *count == 0 {
                    ready.insert(consumer.clone());
                }
            }
        }
        if order.len() != self.nodes.len() {
            return Err(AssuranceError::Invalid(
                "dependency planner graph contains a cycle".to_owned(),
            ));
        }
        Ok(order)
    }

    fn resolve_states(mut self, order: Vec<String>) -> Result<Vec<V2PlanNode>> {
        let mut states = BTreeMap::<String, V2PlanState>::new();
        let mut planned = Vec::with_capacity(order.len());
        for id in order {
            let node = self.nodes.remove(&id).ok_or_else(|| {
                AssuranceError::Invalid(format!("planner node '{id}' disappeared"))
            })?;
            let (state, reason) = resolve_node_state(&node, &states)?;
            states.insert(id.clone(), state);
            planned.push(V2PlanNode {
                id,
                kind: node.kind,
                state,
                identity: node.identity,
                declared_identity: node.declared_identity,
                dependencies: node.dependencies.into_iter().collect(),
                reason,
            });
        }
        Ok(planned)
    }
}

fn resolve_node_state(
    node: &DraftNode,
    states: &BTreeMap<String, V2PlanState>,
) -> Result<(V2PlanState, String)> {
    if node.intrinsic == V2PlanState::Blocked {
        return Ok((node.intrinsic, node.reason.clone()));
    }
    let blocked = dependencies_in_state(node, states, &[V2PlanState::Blocked])?;
    if !blocked.is_empty() {
        return Ok((
            V2PlanState::Blocked,
            format!("blocked by prerequisite(s): {}", blocked.join(",")),
        ));
    }
    if node.intrinsic == V2PlanState::Stale {
        return Ok((node.intrinsic, node.reason.clone()));
    }
    let changed =
        dependencies_in_state(node, states, &[V2PlanState::Stale, V2PlanState::Selected])?;
    if !changed.is_empty() {
        return Ok((
            V2PlanState::Selected,
            format!("selected by changed prerequisite(s): {}", changed.join(",")),
        ));
    }
    Ok((V2PlanState::Current, node.reason.clone()))
}

fn dependencies_in_state(
    node: &DraftNode,
    states: &BTreeMap<String, V2PlanState>,
    selected_states: &[V2PlanState],
) -> Result<Vec<String>> {
    node.dependencies
        .iter()
        .filter_map(|dependency| match states.get(dependency) {
            Some(state) if selected_states.contains(state) => Some(Ok(dependency.clone())),
            Some(_) => None,
            None => Some(Err(AssuranceError::Invalid(format!(
                "planner dependency '{dependency}' was not ordered"
            )))),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_rejects_missing_edge_cycle_and_unused_node() {
        let mut missing = Graph::default();
        missing
            .add(current("target", ["absent"]))
            .expect("add missing target");
        assert!(
            missing
                .finalize("target")
                .unwrap_err()
                .to_string()
                .contains("missing")
        );

        let mut cycle = Graph::default();
        cycle.add(current("a", ["b"])).expect("add a");
        cycle.add(current("b", ["a"])).expect("add b");
        assert!(
            cycle
                .finalize("a")
                .unwrap_err()
                .to_string()
                .contains("cycle")
        );

        let mut unused = Graph::default();
        unused.add(current("target", [])).expect("add target");
        unused.add(current("unused", [])).expect("add unused");
        assert!(
            unused
                .finalize("target")
                .unwrap_err()
                .to_string()
                .contains("unused")
        );
    }

    #[test]
    fn graph_propagates_stale_and_blocked_with_stable_order() {
        let mut stale = current("source", []);
        stale.intrinsic = V2PlanState::Stale;
        stale.reason = "changed".to_owned();
        let mut blocked = current("missing", []);
        blocked.intrinsic = V2PlanState::Blocked;
        blocked.reason = "unavailable".to_owned();
        let mut graph = Graph::default();
        graph.add(stale).expect("add stale");
        graph.add(blocked).expect("add blocked");
        graph
            .add(current("selected", ["source"]))
            .expect("add selected");
        graph
            .add(current("target", ["missing", "selected"]))
            .expect("add target");
        let nodes = graph.finalize("target").expect("resolve graph");
        assert_eq!(nodes[0].id, "missing");
        assert_eq!(nodes[1].id, "source");
        assert_eq!(nodes[2].state, V2PlanState::Selected);
        assert_eq!(nodes[3].state, V2PlanState::Blocked);
    }

    #[test]
    fn blocked_prerequisite_takes_precedence_over_intrinsic_staleness() {
        let mut blocked = current("missing", []);
        blocked.intrinsic = V2PlanState::Blocked;
        blocked.reason = "unavailable".to_owned();
        let mut stale_consumer = current("stale-consumer", ["missing"]);
        stale_consumer.intrinsic = V2PlanState::Stale;
        stale_consumer.reason = "changed".to_owned();
        let mut graph = Graph::default();
        graph.add(blocked).expect("add blocked");
        graph.add(stale_consumer).expect("add stale consumer");
        graph
            .add(current("target", ["stale-consumer"]))
            .expect("add target");
        let nodes = graph.finalize("target").expect("resolve graph");
        assert_eq!(nodes[1].state, V2PlanState::Blocked);
        assert_eq!(nodes[2].state, V2PlanState::Blocked);
    }

    fn current<const N: usize>(id: &str, dependencies: [&str; N]) -> DraftNode {
        DraftNode::current(
            id,
            "test",
            Some(id.to_owned()),
            Some(id.to_owned()),
            dependencies.into_iter().map(str::to_owned).collect(),
            "current",
        )
    }
}
