//! Topology graph model and pre-execution validation gate for openWEPP.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;

use openwepp_sim_contract::closure::{
    ClosureCheckResult, ClosureSeverity, ClosureViolation, ClosureViolationDetails,
    ClosureViolationKind, check_equal_count, check_max, check_min,
};
use openwepp_sim_contract::status::{
    BoundaryClass, SimulationPhase, SimulationStatus, StatusClassification, StatusError,
};

/// Topology node kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum TopologyNodeKind {
    Hillslope,
    Channel,
    Impoundment,
}

impl TopologyNodeKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Hillslope => "hillslope",
            Self::Channel => "channel",
            Self::Impoundment => "impoundment",
        }
    }

    fn parse(value: &str) -> Option<Self> {
        match value {
            "HILLSLOPE" => Some(Self::Hillslope),
            "CHANNEL" => Some(Self::Channel),
            "IMPOUNDMENT" => Some(Self::Impoundment),
            _ => None,
        }
    }
}

/// Contributor slot from watershed structure topology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContributorSlot {
    Left,
    Right,
    Top,
}

impl ContributorSlot {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
            Self::Top => "top",
        }
    }
}

/// A typed node key in the topology graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct TopologyNodeKey {
    pub kind: TopologyNodeKind,
    pub id: u32,
}

impl TopologyNodeKey {
    #[must_use]
    pub const fn new(kind: TopologyNodeKind, id: u32) -> Self {
        Self { kind, id }
    }
}

/// Typed contributor triplet for one contributor kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContributorTriplet {
    pub left: u32,
    pub right: u32,
    pub top: u32,
}

impl ContributorTriplet {
    #[must_use]
    pub const fn new(left: u32, right: u32, top: u32) -> Self {
        Self { left, right, top }
    }

    #[must_use]
    pub fn slots(self) -> [(ContributorSlot, u32); 3] {
        [
            (ContributorSlot::Left, self.left),
            (ContributorSlot::Right, self.right),
            (ContributorSlot::Top, self.top),
        ]
    }

    #[must_use]
    pub fn non_zero_count(self) -> u32 {
        let mut count = 0_u32;
        for (_, value) in self.slots() {
            if value > 0 {
                count += 1;
            }
        }
        count
    }
}

/// Contributors for one downstream topology node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TopologyContributors {
    pub hillslopes: ContributorTriplet,
    pub channels: ContributorTriplet,
    pub impoundments: ContributorTriplet,
}

impl TopologyContributors {
    #[must_use]
    pub const fn new(
        hillslopes: ContributorTriplet,
        channels: ContributorTriplet,
        impoundments: ContributorTriplet,
    ) -> Self {
        Self {
            hillslopes,
            channels,
            impoundments,
        }
    }

    #[must_use]
    pub fn non_zero_count(self) -> u32 {
        self.hillslopes.non_zero_count()
            + self.channels.non_zero_count()
            + self.impoundments.non_zero_count()
    }

    #[must_use]
    pub fn references(self) -> [(TopologyNodeKind, ContributorSlot, u32); 9] {
        let h = self.hillslopes.slots();
        let c = self.channels.slots();
        let i = self.impoundments.slots();

        [
            (TopologyNodeKind::Hillslope, h[0].0, h[0].1),
            (TopologyNodeKind::Hillslope, h[1].0, h[1].1),
            (TopologyNodeKind::Hillslope, h[2].0, h[2].1),
            (TopologyNodeKind::Channel, c[0].0, c[0].1),
            (TopologyNodeKind::Channel, c[1].0, c[1].1),
            (TopologyNodeKind::Channel, c[2].0, c[2].1),
            (TopologyNodeKind::Impoundment, i[0].0, i[0].1),
            (TopologyNodeKind::Impoundment, i[1].0, i[1].1),
            (TopologyNodeKind::Impoundment, i[2].0, i[2].1),
        ]
    }
}

/// A downstream topology node with typed contributors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyNode {
    pub key: TopologyNodeKey,
    pub contributors: TopologyContributors,
}

impl TopologyNode {
    #[must_use]
    pub const fn new(key: TopologyNodeKey, contributors: TopologyContributors) -> Self {
        Self { key, contributors }
    }
}

/// Typed directed contributor edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TopologyEdge {
    pub from: TopologyNodeKey,
    pub to: TopologyNodeKey,
    pub slot: ContributorSlot,
}

/// Topology graph built from watershed structure-style rows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyGraph {
    hillslope_count: u32,
    declared_channel_count: u32,
    declared_impoundment_count: u32,
    nodes: Vec<TopologyNode>,
}

impl TopologyGraph {
    /// Build a topology graph from typed parts.
    #[must_use]
    pub fn new(
        hillslope_count: u32,
        declared_channel_count: u32,
        declared_impoundment_count: u32,
        mut nodes: Vec<TopologyNode>,
    ) -> Self {
        nodes.sort_by_key(|node| node.key);
        Self {
            hillslope_count,
            declared_channel_count,
            declared_impoundment_count,
            nodes,
        }
    }

    #[must_use]
    pub const fn hillslope_count(&self) -> u32 {
        self.hillslope_count
    }

    #[must_use]
    pub const fn declared_channel_count(&self) -> u32 {
        self.declared_channel_count
    }

    #[must_use]
    pub const fn declared_impoundment_count(&self) -> u32 {
        self.declared_impoundment_count
    }

    #[must_use]
    pub fn nodes(&self) -> &[TopologyNode] {
        self.nodes.as_slice()
    }

    #[must_use]
    pub fn observed_channel_count(&self) -> usize {
        self.nodes
            .iter()
            .filter(|node| node.key.kind == TopologyNodeKind::Channel)
            .count()
    }

    #[must_use]
    pub fn observed_impoundment_count(&self) -> usize {
        self.nodes
            .iter()
            .filter(|node| node.key.kind == TopologyNodeKind::Impoundment)
            .count()
    }

    #[must_use]
    pub fn node_keys(&self) -> BTreeSet<TopologyNodeKey> {
        self.nodes.iter().map(|node| node.key).collect()
    }

    #[must_use]
    pub fn edges(&self) -> Vec<TopologyEdge> {
        let mut edges = Vec::new();

        for node in &self.nodes {
            for (kind, slot, contributor_id) in node.contributors.references() {
                if contributor_id == 0 {
                    continue;
                }

                edges.push(TopologyEdge {
                    from: TopologyNodeKey::new(kind, contributor_id),
                    to: node.key,
                    slot,
                });
            }
        }

        edges
    }
}

/// Topology fixture parse errors.
#[derive(Debug)]
pub enum TopologyParseError {
    ReadError {
        path: String,
        source: std::io::Error,
    },
    MissingHeader {
        header: String,
    },
    HeaderFormat {
        line: usize,
        found: String,
    },
    HeaderValueParse {
        line: usize,
        header: String,
        value: String,
    },
    DuplicateHeader {
        line: usize,
        header: String,
    },
    NodeRecordFormat {
        line: usize,
        found: String,
    },
    UnknownNodeKind {
        line: usize,
        value: String,
    },
    NodeValueParse {
        line: usize,
        field: String,
        value: String,
    },
    DuplicateNode {
        line: usize,
        kind: TopologyNodeKind,
        id: u32,
    },
}

impl fmt::Display for TopologyParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(topology_parse_error_message(self).as_str())
    }
}

impl Error for TopologyParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ReadError { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// Validate gate error surface.
#[derive(Debug)]
pub enum TopologyValidationError {
    Status(StatusError),
}

impl fmt::Display for TopologyValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Status(source) => write!(f, "failed constructing topology status: {source}"),
        }
    }
}

impl Error for TopologyValidationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Status(source) => Some(source),
        }
    }
}

impl From<StatusError> for TopologyValidationError {
    fn from(value: StatusError) -> Self {
        Self::Status(value)
    }
}

/// Topology validation report for pre-execution gating.
#[derive(Debug, Clone)]
pub struct TopologyValidationReport {
    pub status: SimulationStatus,
    pub violations: Vec<ClosureViolation>,
}

impl TopologyValidationReport {
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.violations.is_empty() && self.status.classification() != StatusClassification::Failure
    }
}

/// Parse topology fixture text into a typed graph model.
///
/// Supported fixture format:
/// - Headers (required): `HILLSLOPES <count>`, `CHANNELS <count>`, `IMPOUNDMENTS <count>`
/// - Node rows:
///   `NODE <CHANNEL|IMPOUNDMENT> <id> H <l> <r> <t> C <l> <r> <t> I <l> <r> <t>`
///
/// Blank lines and lines starting with `#` are ignored.
///
/// # Errors
///
/// Returns `TopologyParseError` for malformed headers, malformed node rows,
/// unknown kinds, numeric parse failures, duplicate nodes, or missing headers.
pub fn parse_topology_fixture_str(input: &str) -> Result<TopologyGraph, TopologyParseError> {
    let mut fixture = TopologyFixtureBuilder::default();

    for (index, raw_line) in input.lines().enumerate() {
        fixture.accept_line(index + 1, raw_line)?;
    }

    fixture.finish()
}

/// Parse a topology fixture file into a typed graph model.
///
/// # Errors
///
/// Returns `TopologyParseError` when file read fails or fixture parsing fails.
pub fn parse_topology_fixture_path(
    path: impl AsRef<Path>,
) -> Result<TopologyGraph, TopologyParseError> {
    let path_ref = path.as_ref();
    let content = fs::read_to_string(path_ref).map_err(|source| TopologyParseError::ReadError {
        path: path_ref.display().to_string(),
        source,
    })?;

    parse_topology_fixture_str(content.as_str())
}

/// Run the pre-execution topology validation gate.
///
/// Validation enforces:
/// - declared channel/impoundment count closure,
/// - non-empty contributor closure for each downstream element,
/// - contributor reference domain checks,
/// - contributor reference existence checks,
/// - cycle rejection across channel/impoundment subgraph.
///
/// # Errors
///
/// Returns `TopologyValidationError` when status object construction fails.
pub fn validate_pre_execution_topology(
    graph: &TopologyGraph,
) -> Result<TopologyValidationReport, TopologyValidationError> {
    let mut violations: Vec<ClosureViolation> = Vec::new();

    collect_hillslope_count_violation(graph, &mut violations);
    collect_channel_count_violation(graph, &mut violations);
    collect_impoundment_count_violation(graph, &mut violations);

    let existing_ids = ExistingTopologyIds::from_graph(graph);
    for node in graph.nodes() {
        collect_node_violations(graph, node, &existing_ids, &mut violations);
    }

    collect_cycle_violation(graph, &mut violations);

    let status = if violations.is_empty() {
        SimulationStatus::ok(SimulationPhase::PreExecutionValidation, "TOPOLOGY-OK-001")?
    } else {
        SimulationStatus::failure(
            SimulationPhase::PreExecutionValidation,
            true,
            false,
            BoundaryClass::TopologyInvalid,
            "TOPOLOGY-E-VALIDATION-FAILED",
        )?
    };

    Ok(TopologyValidationReport { status, violations })
}

#[derive(Default)]
struct TopologyFixtureBuilder {
    hillslope_count: Option<u32>,
    channel_count: Option<u32>,
    impoundment_count: Option<u32>,
    nodes: Vec<TopologyNode>,
    seen_keys: BTreeSet<TopologyNodeKey>,
}

impl TopologyFixtureBuilder {
    fn accept_line(
        &mut self,
        line_number: usize,
        raw_line: &str,
    ) -> Result<(), TopologyParseError> {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            return Ok(());
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.is_empty() {
            return Ok(());
        }

        if tokens[0] == "NODE" {
            self.accept_node(line_number, line, tokens.as_slice())
        } else {
            self.accept_header(line_number, line, tokens.as_slice())
        }
    }

    fn accept_node(
        &mut self,
        line_number: usize,
        line: &str,
        tokens: &[&str],
    ) -> Result<(), TopologyParseError> {
        let node = parse_node_record(line_number, line, tokens)?;

        if !self.seen_keys.insert(node.key) {
            return Err(TopologyParseError::DuplicateNode {
                line: line_number,
                kind: node.key.kind,
                id: node.key.id,
            });
        }

        self.nodes.push(node);
        Ok(())
    }

    fn accept_header(
        &mut self,
        line_number: usize,
        line: &str,
        tokens: &[&str],
    ) -> Result<(), TopologyParseError> {
        if tokens.len() != 2 {
            return Err(TopologyParseError::HeaderFormat {
                line: line_number,
                found: line.to_string(),
            });
        }

        let value = parse_header_value(tokens, line_number)?;

        match tokens[0] {
            "HILLSLOPES" => {
                set_header_value(&mut self.hillslope_count, line_number, tokens[0], value)
            }
            "CHANNELS" => set_header_value(&mut self.channel_count, line_number, tokens[0], value),
            "IMPOUNDMENTS" => {
                set_header_value(&mut self.impoundment_count, line_number, tokens[0], value)
            }
            _ => Err(TopologyParseError::HeaderFormat {
                line: line_number,
                found: line.to_string(),
            }),
        }
    }

    fn finish(self) -> Result<TopologyGraph, TopologyParseError> {
        let hillslope_count =
            self.hillslope_count
                .ok_or_else(|| TopologyParseError::MissingHeader {
                    header: "HILLSLOPES".to_string(),
                })?;
        let channel_count =
            self.channel_count
                .ok_or_else(|| TopologyParseError::MissingHeader {
                    header: "CHANNELS".to_string(),
                })?;
        let impoundment_count =
            self.impoundment_count
                .ok_or_else(|| TopologyParseError::MissingHeader {
                    header: "IMPOUNDMENTS".to_string(),
                })?;

        Ok(TopologyGraph::new(
            hillslope_count,
            channel_count,
            impoundment_count,
            self.nodes,
        ))
    }
}

struct ExistingTopologyIds {
    channels: BTreeSet<u32>,
    impoundments: BTreeSet<u32>,
}

impl ExistingTopologyIds {
    fn from_graph(graph: &TopologyGraph) -> Self {
        let node_keys = graph.node_keys();
        let channels = node_keys
            .iter()
            .filter(|key| key.kind == TopologyNodeKind::Channel)
            .map(|key| key.id)
            .collect();
        let impoundments = node_keys
            .iter()
            .filter(|key| key.kind == TopologyNodeKind::Impoundment)
            .map(|key| key.id)
            .collect();

        Self {
            channels,
            impoundments,
        }
    }
}

fn topology_parse_error_message(error: &TopologyParseError) -> String {
    match error {
        TopologyParseError::ReadError { path, source } => {
            format!("failed reading topology fixture {path}: {source}")
        }
        TopologyParseError::MissingHeader { header } => {
            format!("missing required topology header {header}")
        }
        TopologyParseError::HeaderFormat { line, found } => {
            format!("invalid topology header format at line {line}: {found}")
        }
        TopologyParseError::HeaderValueParse {
            line,
            header,
            value,
        } => format!("failed parsing topology header value at line {line} for {header}: {value}"),
        TopologyParseError::DuplicateHeader { line, header } => {
            format!("duplicate topology header at line {line}: {header}")
        }
        TopologyParseError::NodeRecordFormat { line, found } => {
            format!("invalid topology node format at line {line}: {found}")
        }
        TopologyParseError::UnknownNodeKind { line, value } => {
            format!("unknown topology node kind at line {line}: {value}")
        }
        TopologyParseError::NodeValueParse { line, field, value } => {
            format!("invalid topology node value at line {line} for {field}: {value}")
        }
        TopologyParseError::DuplicateNode { line, kind, id } => {
            format!(
                "duplicate topology node at line {line}: {}:{id}",
                kind.as_str()
            )
        }
    }
}

fn parse_header_value(tokens: &[&str], line_number: usize) -> Result<u32, TopologyParseError> {
    tokens[1]
        .parse::<u32>()
        .map_err(|_| TopologyParseError::HeaderValueParse {
            line: line_number,
            header: tokens[0].to_string(),
            value: tokens[1].to_string(),
        })
}

fn parse_node_record(
    line_number: usize,
    line: &str,
    tokens: &[&str],
) -> Result<TopologyNode, TopologyParseError> {
    if tokens.len() != 15 {
        return Err(TopologyParseError::NodeRecordFormat {
            line: line_number,
            found: line.to_string(),
        });
    }

    let kind = parse_node_kind(tokens[1], line_number)?;
    let node_id = parse_u32(tokens[2], line_number, "id")?;
    validate_node_markers(line_number, line, tokens)?;
    let contributors = parse_contributors(tokens, line_number)?;

    Ok(TopologyNode::new(
        TopologyNodeKey::new(kind, node_id),
        contributors,
    ))
}

fn parse_node_kind(
    value: &str,
    line_number: usize,
) -> Result<TopologyNodeKind, TopologyParseError> {
    let kind =
        TopologyNodeKind::parse(value).ok_or_else(|| TopologyParseError::UnknownNodeKind {
            line: line_number,
            value: value.to_string(),
        })?;

    if kind == TopologyNodeKind::Hillslope {
        return Err(TopologyParseError::UnknownNodeKind {
            line: line_number,
            value: value.to_string(),
        });
    }

    Ok(kind)
}

fn validate_node_markers(
    line_number: usize,
    line: &str,
    tokens: &[&str],
) -> Result<(), TopologyParseError> {
    if tokens[3] != "H" || tokens[7] != "C" || tokens[11] != "I" {
        return Err(TopologyParseError::NodeRecordFormat {
            line: line_number,
            found: line.to_string(),
        });
    }

    Ok(())
}

fn parse_contributors(
    tokens: &[&str],
    line_number: usize,
) -> Result<TopologyContributors, TopologyParseError> {
    Ok(TopologyContributors::new(
        ContributorTriplet::new(
            parse_u32(tokens[4], line_number, "hillslope_left")?,
            parse_u32(tokens[5], line_number, "hillslope_right")?,
            parse_u32(tokens[6], line_number, "hillslope_top")?,
        ),
        ContributorTriplet::new(
            parse_u32(tokens[8], line_number, "channel_left")?,
            parse_u32(tokens[9], line_number, "channel_right")?,
            parse_u32(tokens[10], line_number, "channel_top")?,
        ),
        ContributorTriplet::new(
            parse_u32(tokens[12], line_number, "impoundment_left")?,
            parse_u32(tokens[13], line_number, "impoundment_right")?,
            parse_u32(tokens[14], line_number, "impoundment_top")?,
        ),
    ))
}

fn collect_hillslope_count_violation(
    graph: &TopologyGraph,
    violations: &mut Vec<ClosureViolation>,
) {
    collect_result(
        check_min(
            "INV-TOPO-001",
            "TOPO-E-001",
            "hillslope_count",
            f64::from(graph.hillslope_count),
            1.0,
        ),
        violations,
    );
}

fn collect_channel_count_violation(graph: &TopologyGraph, violations: &mut Vec<ClosureViolation>) {
    match usize::try_from(graph.declared_channel_count) {
        Ok(declared_channel_count) => {
            collect_result(
                check_equal_count(
                    "INV-TOPO-002",
                    "TOPO-E-002",
                    "declared_vs_observed_channel_count",
                    declared_channel_count,
                    graph.observed_channel_count(),
                ),
                violations,
            );
        }
        // COVERAGE-EXCLUDE: u32 counts fit usize on supported openWEPP targets;
        // keep the fail-closed fallback for unsupported narrower targets.
        Err(_) => {
            violations.push(ClosureViolation::new(
                "INV-TOPO-002",
                "TOPO-E-002",
                "declared_vs_observed_channel_count",
                ClosureViolationKind::CardinalityMismatch,
                ClosureSeverity::HardFail,
                ClosureViolationDetails::Cardinality {
                    expected: usize::MAX,
                    observed: graph.observed_channel_count(),
                },
            ));
        }
    }
}

fn collect_impoundment_count_violation(
    graph: &TopologyGraph,
    violations: &mut Vec<ClosureViolation>,
) {
    match usize::try_from(graph.declared_impoundment_count) {
        Ok(declared_impoundment_count) => {
            collect_result(
                check_equal_count(
                    "INV-TOPO-003",
                    "TOPO-E-003",
                    "declared_vs_observed_impoundment_count",
                    declared_impoundment_count,
                    graph.observed_impoundment_count(),
                ),
                violations,
            );
        }
        // COVERAGE-EXCLUDE: u32 counts fit usize on supported openWEPP targets;
        // keep the fail-closed fallback for unsupported narrower targets.
        Err(_) => {
            violations.push(ClosureViolation::new(
                "INV-TOPO-003",
                "TOPO-E-003",
                "declared_vs_observed_impoundment_count",
                ClosureViolationKind::CardinalityMismatch,
                ClosureSeverity::HardFail,
                ClosureViolationDetails::Cardinality {
                    expected: usize::MAX,
                    observed: graph.observed_impoundment_count(),
                },
            ));
        }
    }
}

fn collect_node_violations(
    graph: &TopologyGraph,
    node: &TopologyNode,
    existing_ids: &ExistingTopologyIds,
    violations: &mut Vec<ClosureViolation>,
) {
    let subject_base = format!("{}:{}", node.key.kind.as_str(), node.key.id);
    collect_contributor_count_violation(node, subject_base.as_str(), violations);

    for (kind, slot, reference_id) in node.contributors.references() {
        collect_reference_violation(
            graph,
            existing_ids,
            subject_base.as_str(),
            kind,
            slot,
            reference_id,
            violations,
        );
    }
}

fn collect_contributor_count_violation(
    node: &TopologyNode,
    subject_base: &str,
    violations: &mut Vec<ClosureViolation>,
) {
    collect_result(
        check_min(
            "INV-TOPO-004",
            "TOPO-E-004",
            format!("{subject_base}:contributor_count").as_str(),
            f64::from(node.contributors.non_zero_count()),
            1.0,
        ),
        violations,
    );
}

fn collect_reference_violation(
    graph: &TopologyGraph,
    existing_ids: &ExistingTopologyIds,
    subject_base: &str,
    kind: TopologyNodeKind,
    slot: ContributorSlot,
    reference_id: u32,
    violations: &mut Vec<ClosureViolation>,
) {
    if reference_id == 0 {
        return;
    }

    let reference_subject = format!("{subject_base}:{}:{}", kind.as_str(), slot.as_str());
    collect_reference_lower_bound(kind, reference_subject.as_str(), reference_id, violations);

    let upper_bound = reference_upper_bound(graph, kind);
    if upper_bound == 0 {
        push_zero_upper_bound_reference(kind, reference_subject, reference_id, violations);
        return;
    }

    collect_reference_upper_bound(
        kind,
        reference_subject.as_str(),
        reference_id,
        upper_bound,
        violations,
    );
    collect_reference_existence(
        graph,
        existing_ids,
        kind,
        reference_subject,
        reference_id,
        violations,
    );
}

fn collect_reference_lower_bound(
    kind: TopologyNodeKind,
    reference_subject: &str,
    reference_id: u32,
    violations: &mut Vec<ClosureViolation>,
) {
    collect_result(
        check_min(
            "INV-TOPO-006",
            reference_domain_message_id(kind),
            reference_subject,
            f64::from(reference_id),
            1.0,
        ),
        violations,
    );
}

fn reference_upper_bound(graph: &TopologyGraph, kind: TopologyNodeKind) -> u32 {
    match kind {
        TopologyNodeKind::Hillslope => graph.hillslope_count,
        TopologyNodeKind::Channel => graph.declared_channel_count,
        TopologyNodeKind::Impoundment => graph.declared_impoundment_count,
    }
}

fn push_zero_upper_bound_reference(
    kind: TopologyNodeKind,
    reference_subject: String,
    reference_id: u32,
    violations: &mut Vec<ClosureViolation>,
) {
    violations.push(ClosureViolation::new(
        "INV-TOPO-006",
        reference_domain_message_id(kind),
        reference_subject,
        ClosureViolationKind::DomainRange,
        ClosureSeverity::HardFail,
        ClosureViolationDetails::Range {
            value: f64::from(reference_id),
            minimum: 1.0,
            maximum: 0.0,
        },
    ));
}

fn collect_reference_upper_bound(
    kind: TopologyNodeKind,
    reference_subject: &str,
    reference_id: u32,
    upper_bound: u32,
    violations: &mut Vec<ClosureViolation>,
) {
    collect_result(
        check_max(
            "INV-TOPO-006",
            reference_domain_message_id(kind),
            reference_subject,
            f64::from(reference_id),
            f64::from(upper_bound),
        ),
        violations,
    );
}

fn collect_reference_existence(
    graph: &TopologyGraph,
    existing_ids: &ExistingTopologyIds,
    kind: TopologyNodeKind,
    reference_subject: String,
    reference_id: u32,
    violations: &mut Vec<ClosureViolation>,
) {
    match kind {
        TopologyNodeKind::Hillslope => {}
        TopologyNodeKind::Channel => collect_channel_reference_existence(
            graph,
            existing_ids,
            reference_subject,
            reference_id,
            violations,
        ),
        TopologyNodeKind::Impoundment => collect_impoundment_reference_existence(
            graph,
            existing_ids,
            reference_subject,
            reference_id,
            violations,
        ),
    }
}

fn collect_channel_reference_existence(
    graph: &TopologyGraph,
    existing_ids: &ExistingTopologyIds,
    reference_subject: String,
    reference_id: u32,
    violations: &mut Vec<ClosureViolation>,
) {
    if existing_ids.channels.contains(&reference_id) {
        return;
    }

    violations.push(ClosureViolation::new(
        "INV-TOPO-007",
        "TOPO-E-009",
        reference_subject,
        ClosureViolationKind::DomainRange,
        ClosureSeverity::HardFail,
        ClosureViolationDetails::Range {
            value: f64::from(reference_id),
            minimum: 1.0,
            maximum: f64::from(graph.declared_channel_count),
        },
    ));
}

fn collect_impoundment_reference_existence(
    graph: &TopologyGraph,
    existing_ids: &ExistingTopologyIds,
    reference_subject: String,
    reference_id: u32,
    violations: &mut Vec<ClosureViolation>,
) {
    if existing_ids.impoundments.contains(&reference_id) {
        return;
    }

    violations.push(ClosureViolation::new(
        "INV-TOPO-008",
        "TOPO-E-010",
        reference_subject,
        ClosureViolationKind::DomainRange,
        ClosureSeverity::HardFail,
        ClosureViolationDetails::Range {
            value: f64::from(reference_id),
            minimum: 1.0,
            maximum: f64::from(graph.declared_impoundment_count),
        },
    ));
}

fn collect_cycle_violation(graph: &TopologyGraph, violations: &mut Vec<ClosureViolation>) {
    if !has_channel_impoundment_cycle(graph) {
        return;
    }

    violations.push(ClosureViolation::new(
        "INV-TOPO-005",
        "TOPO-E-005",
        "directed_cycle_count",
        ClosureViolationKind::CardinalityMismatch,
        ClosureSeverity::HardFail,
        ClosureViolationDetails::Cardinality {
            expected: 0,
            observed: 1,
        },
    ));
}

fn parse_u32(token: &str, line: usize, field: &str) -> Result<u32, TopologyParseError> {
    token
        .parse::<u32>()
        .map_err(|_| TopologyParseError::NodeValueParse {
            line,
            field: field.to_string(),
            value: token.to_string(),
        })
}

fn set_header_value(
    target: &mut Option<u32>,
    line: usize,
    header: &str,
    value: u32,
) -> Result<(), TopologyParseError> {
    if target.is_some() {
        return Err(TopologyParseError::DuplicateHeader {
            line,
            header: header.to_string(),
        });
    }

    *target = Some(value);
    Ok(())
}

fn reference_domain_message_id(kind: TopologyNodeKind) -> &'static str {
    match kind {
        TopologyNodeKind::Hillslope => "TOPO-E-006",
        TopologyNodeKind::Channel => "TOPO-E-007",
        TopologyNodeKind::Impoundment => "TOPO-E-008",
    }
}

fn collect_result(result: ClosureCheckResult, output: &mut Vec<ClosureViolation>) {
    if let Err(violation) = result {
        output.push(*violation);
    }
}

fn has_channel_impoundment_cycle(graph: &TopologyGraph) -> bool {
    let mut adjacency: BTreeMap<TopologyNodeKey, BTreeSet<TopologyNodeKey>> = BTreeMap::new();
    let eligible: BTreeSet<TopologyNodeKey> = graph
        .nodes()
        .iter()
        .map(|node| node.key)
        .filter(|key| key.kind != TopologyNodeKind::Hillslope)
        .collect();

    for key in &eligible {
        adjacency.entry(*key).or_default();
    }

    for edge in graph.edges() {
        if edge.from.kind == TopologyNodeKind::Hillslope {
            continue;
        }

        if eligible.contains(&edge.from) && eligible.contains(&edge.to) {
            adjacency.entry(edge.from).or_default().insert(edge.to);
        }
    }

    let mut visiting: BTreeSet<TopologyNodeKey> = BTreeSet::new();
    let mut visited: BTreeSet<TopologyNodeKey> = BTreeSet::new();

    for start in &eligible {
        if dfs_has_cycle(*start, &adjacency, &mut visiting, &mut visited) {
            return true;
        }
    }

    false
}

fn dfs_has_cycle(
    node: TopologyNodeKey,
    adjacency: &BTreeMap<TopologyNodeKey, BTreeSet<TopologyNodeKey>>,
    visiting: &mut BTreeSet<TopologyNodeKey>,
    visited: &mut BTreeSet<TopologyNodeKey>,
) -> bool {
    if visited.contains(&node) {
        return false;
    }

    if !visiting.insert(node) {
        return true;
    }

    if let Some(neighbors) = adjacency.get(&node) {
        for neighbor in neighbors {
            if dfs_has_cycle(*neighbor, adjacency, visiting, visited) {
                return true;
            }
        }
    }

    visiting.remove(&node);
    visited.insert(node);
    false
}
