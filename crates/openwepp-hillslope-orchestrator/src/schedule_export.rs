#![allow(clippy::module_name_repetitions)]

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use crate::{HillslopePhase, HillslopePhaseGraph, hillslope_consumer_adapter_for_phase};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduleNode {
    pub phase: HillslopePhase,
    pub rank: usize,
    pub consumer_adapter: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduleEdge {
    pub from: HillslopePhase,
    pub to: HillslopePhase,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduleExport {
    pub nodes: Vec<ScheduleNode>,
    pub edges: Vec<ScheduleEdge>,
    pub topological_order: Vec<HillslopePhase>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScheduleDiagnostic {
    GraphCycle,
    MissingCanonicalRoot,
    UnreachableFromCanonicalRoot {
        phase: HillslopePhase,
    },
    CanonicalOrderMismatch {
        canonical: Vec<&'static str>,
        topological: Vec<&'static str>,
    },
}

impl ScheduleDiagnostic {
    #[must_use]
    pub fn message(&self) -> String {
        match self {
            Self::GraphCycle => "schedule graph contains a cycle".to_owned(),
            Self::MissingCanonicalRoot => "schedule graph has no canonical root phase".to_owned(),
            Self::UnreachableFromCanonicalRoot { phase } => {
                let mut message = "phase unreachable from canonical root: ".to_owned();
                message.push_str(phase.as_str());
                message
            }
            Self::CanonicalOrderMismatch {
                canonical,
                topological,
            } => {
                let mut message =
                    "topological order does not match canonical phase order: canonical=".to_owned();
                message.push_str(&canonical.join(","));
                message.push_str(" topological=");
                message.push_str(&topological.join(","));
                message
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduleValidationReport {
    pub topological_order: Vec<HillslopePhase>,
    pub diagnostics: Vec<ScheduleDiagnostic>,
}

impl ScheduleValidationReport {
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.diagnostics.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduleNamedEdge {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduleDiff {
    pub added_nodes: Vec<String>,
    pub removed_nodes: Vec<String>,
    pub added_edges: Vec<ScheduleNamedEdge>,
    pub removed_edges: Vec<ScheduleNamedEdge>,
}

impl ScheduleDiff {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.added_nodes.is_empty()
            && self.removed_nodes.is_empty()
            && self.added_edges.is_empty()
            && self.removed_edges.is_empty()
    }
}

#[derive(Debug)]
pub enum ScheduleExportError {
    ValidationFailed(Vec<ScheduleDiagnostic>),
    JsonParse { message: String },
    InvalidJsonShape { message: String },
}

impl fmt::Display for ScheduleExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ValidationFailed(diagnostics) => {
                write!(f, "schedule validation failed")?;
                for diagnostic in diagnostics {
                    write!(f, "; {}", diagnostic.message())?;
                }
                Ok(())
            }
            Self::JsonParse { message } => write!(f, "schedule JSON parse failed: {message}"),
            Self::InvalidJsonShape { message } => {
                write!(f, "schedule JSON has invalid shape: {message}")
            }
        }
    }
}

impl Error for ScheduleExportError {}

#[must_use]
pub fn validate_hillslope_schedule_graph(graph: &HillslopePhaseGraph) -> ScheduleValidationReport {
    let canonical_order = HillslopePhaseGraph::canonical_order();
    let mut diagnostics = Vec::new();
    let topological_order = if let Some(order) = graph.topological_order() {
        order
    } else {
        diagnostics.push(ScheduleDiagnostic::GraphCycle);
        Vec::new()
    };

    let Some(root) = canonical_order.first().copied() else {
        diagnostics.push(ScheduleDiagnostic::MissingCanonicalRoot);
        return ScheduleValidationReport {
            topological_order,
            diagnostics,
        };
    };

    let mut adjacency: BTreeMap<HillslopePhase, BTreeSet<HillslopePhase>> = canonical_order
        .iter()
        .copied()
        .map(|phase| (phase, BTreeSet::new()))
        .collect();
    for edge in graph.dependency_edges() {
        adjacency
            .entry(edge.depends_on)
            .or_default()
            .insert(edge.phase);
    }

    let mut reachable = BTreeSet::new();
    let mut stack = vec![root];
    while let Some(phase) = stack.pop() {
        if !reachable.insert(phase) {
            continue;
        }

        if let Some(children) = adjacency.get(&phase) {
            for child in children.iter().rev() {
                stack.push(*child);
            }
        }
    }

    for phase in canonical_order {
        if !reachable.contains(&phase) {
            diagnostics.push(ScheduleDiagnostic::UnreachableFromCanonicalRoot { phase });
        }
    }

    if !topological_order.is_empty() && topological_order.as_slice() != canonical_order {
        diagnostics.push(ScheduleDiagnostic::CanonicalOrderMismatch {
            canonical: canonical_order.iter().map(|phase| phase.as_str()).collect(),
            topological: topological_order
                .iter()
                .map(|phase| phase.as_str())
                .collect(),
        });
    }

    ScheduleValidationReport {
        topological_order,
        diagnostics,
    }
}

pub fn canonical_hillslope_schedule_export() -> Result<ScheduleExport, ScheduleExportError> {
    ScheduleExport::from_graph(&HillslopePhaseGraph::canonical())
}

impl ScheduleExport {
    pub fn from_graph(graph: &HillslopePhaseGraph) -> Result<Self, ScheduleExportError> {
        let validation = validate_hillslope_schedule_graph(graph);
        if !validation.is_valid() {
            return Err(ScheduleExportError::ValidationFailed(
                validation.diagnostics,
            ));
        }

        let nodes = HillslopePhaseGraph::canonical_order()
            .iter()
            .copied()
            .map(|phase| ScheduleNode {
                phase,
                rank: phase.rank(),
                consumer_adapter: hillslope_consumer_adapter_for_phase(phase).as_str(),
            })
            .collect();
        let mut edges: Vec<ScheduleEdge> = graph
            .dependency_edges()
            .into_iter()
            .map(|edge| ScheduleEdge {
                from: edge.depends_on,
                to: edge.phase,
            })
            .collect();
        edges.sort_by_key(|edge| (edge.from.rank(), edge.to.rank()));

        Ok(Self {
            nodes,
            edges,
            topological_order: validation.topological_order,
        })
    }

    #[must_use]
    pub fn render_json(&self) -> String {
        let mut out = String::new();
        out.push_str("{\n");
        out.push_str("  \"nodes\": [\n");
        for (index, node) in self.nodes.iter().enumerate() {
            out.push_str("    {\n");
            out.push_str("      \"phase\": ");
            push_json_string(&mut out, node.phase.as_str());
            out.push_str(",\n");
            out.push_str("      \"rank\": ");
            out.push_str(&node.rank.to_string());
            out.push_str(",\n");
            out.push_str("      \"consumer_adapter\": ");
            push_json_string(&mut out, node.consumer_adapter);
            out.push('\n');
            out.push_str("    }");
            if index + 1 != self.nodes.len() {
                out.push(',');
            }
            out.push('\n');
        }
        out.push_str("  ],\n");
        out.push_str("  \"edges\": [\n");
        for (index, edge) in self.edges.iter().enumerate() {
            out.push_str("    {\n");
            out.push_str("      \"from\": ");
            push_json_string(&mut out, edge.from.as_str());
            out.push_str(",\n");
            out.push_str("      \"to\": ");
            push_json_string(&mut out, edge.to.as_str());
            out.push('\n');
            out.push_str("    }");
            if index + 1 != self.edges.len() {
                out.push(',');
            }
            out.push('\n');
        }
        out.push_str("  ],\n");
        out.push_str("  \"topological_order\": [\n");
        for (index, phase) in self.topological_order.iter().enumerate() {
            out.push_str("    ");
            push_json_string(&mut out, phase.as_str());
            if index + 1 != self.topological_order.len() {
                out.push(',');
            }
            out.push('\n');
        }
        out.push_str("  ]\n");
        out.push_str("}\n");
        out
    }

    #[must_use]
    pub fn render_mermaid(&self) -> String {
        let mut out = String::new();
        out.push_str("flowchart TD\n");
        for node in &self.nodes {
            out.push_str("    ");
            out.push_str(node.phase.as_str());
            out.push_str("[\"");
            out.push_str(node.phase.as_str());
            out.push_str("<br/>rank: ");
            out.push_str(&node.rank.to_string());
            out.push_str("<br/>adapter: ");
            out.push_str(node.consumer_adapter);
            out.push_str("\"]\n");
        }
        for edge in &self.edges {
            out.push_str("    ");
            out.push_str(edge.from.as_str());
            out.push_str(" --> ");
            out.push_str(edge.to.as_str());
            out.push('\n');
        }
        out
    }

    #[must_use]
    pub fn render_dot(&self) -> String {
        let mut out = String::new();
        out.push_str("digraph hillslope_phase_schedule {\n");
        out.push_str("    rankdir=TB;\n");
        out.push_str("    node [shape=box];\n");
        for node in &self.nodes {
            out.push_str("    ");
            push_dot_string(&mut out, node.phase.as_str());
            out.push_str(" [label=");
            let mut label = node.phase.as_str().to_owned();
            label.push_str("\nrank: ");
            label.push_str(&node.rank.to_string());
            label.push_str("\nadapter: ");
            label.push_str(node.consumer_adapter);
            push_dot_string(&mut out, &label);
            out.push_str("];\n");
        }
        for edge in &self.edges {
            out.push_str("    ");
            push_dot_string(&mut out, edge.from.as_str());
            out.push_str(" -> ");
            push_dot_string(&mut out, edge.to.as_str());
            out.push_str(";\n");
        }
        out.push_str("}\n");
        out
    }
}

pub fn diff_schedule_json(
    base_json: &str,
    head_json: &str,
) -> Result<ScheduleDiff, ScheduleExportError> {
    let base = ParsedSchedule::from_json(base_json)?;
    let head = ParsedSchedule::from_json(head_json)?;

    let added_nodes = head.nodes.difference(&base.nodes).cloned().collect();
    let removed_nodes = base.nodes.difference(&head.nodes).cloned().collect();
    let added_edges = head
        .edges
        .difference(&base.edges)
        .map(|(from, to)| ScheduleNamedEdge {
            from: from.clone(),
            to: to.clone(),
        })
        .collect();
    let removed_edges = base
        .edges
        .difference(&head.edges)
        .map(|(from, to)| ScheduleNamedEdge {
            from: from.clone(),
            to: to.clone(),
        })
        .collect();

    Ok(ScheduleDiff {
        added_nodes,
        removed_nodes,
        added_edges,
        removed_edges,
    })
}

#[must_use]
pub fn render_schedule_diff(diff: &ScheduleDiff) -> String {
    if diff.is_empty() {
        return "No schedule differences.\n".to_owned();
    }

    let mut out = String::new();
    push_diff_section(&mut out, "Added nodes", &diff.added_nodes);
    push_diff_section(&mut out, "Removed nodes", &diff.removed_nodes);
    push_edge_diff_section(&mut out, "Added edges", &diff.added_edges);
    push_edge_diff_section(&mut out, "Removed edges", &diff.removed_edges);
    out
}

fn push_diff_section(out: &mut String, title: &str, values: &[String]) {
    if values.is_empty() {
        return;
    }

    out.push_str(title);
    out.push_str(":\n");
    for value in values {
        out.push_str("- ");
        out.push_str(value);
        out.push('\n');
    }
}

fn push_edge_diff_section(out: &mut String, title: &str, values: &[ScheduleNamedEdge]) {
    if values.is_empty() {
        return;
    }

    out.push_str(title);
    out.push_str(":\n");
    for value in values {
        out.push_str("- ");
        out.push_str(&value.from);
        out.push_str(" -> ");
        out.push_str(&value.to);
        out.push('\n');
    }
}

fn push_json_string(out: &mut String, value: &str) {
    out.push('"');
    for character in value.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            value => out.push(value),
        }
    }
    out.push('"');
}

fn push_dot_string(out: &mut String, value: &str) {
    out.push('"');
    for character in value.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            value => out.push(value),
        }
    }
    out.push('"');
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedSchedule {
    nodes: BTreeSet<String>,
    edges: BTreeSet<(String, String)>,
}

impl ParsedSchedule {
    fn from_json(input: &str) -> Result<Self, ScheduleExportError> {
        let value = JsonParser::new(input).parse()?;
        let object = value.as_object("root")?;
        let nodes = object
            .get("nodes")
            .ok_or_else(|| ScheduleExportError::InvalidJsonShape {
                message: "missing root.nodes".to_owned(),
            })?
            .as_array("root.nodes")?;
        let edges = object
            .get("edges")
            .ok_or_else(|| ScheduleExportError::InvalidJsonShape {
                message: "missing root.edges".to_owned(),
            })?
            .as_array("root.edges")?;

        let mut parsed_nodes = BTreeSet::new();
        for node in nodes {
            let node_object = node.as_object("root.nodes[]")?;
            let phase = node_object
                .get("phase")
                .ok_or_else(|| ScheduleExportError::InvalidJsonShape {
                    message: "missing root.nodes[].phase".to_owned(),
                })?
                .as_string("root.nodes[].phase")?;
            parsed_nodes.insert(phase.to_owned());
        }

        let mut parsed_edges = BTreeSet::new();
        for edge in edges {
            let edge_object = edge.as_object("root.edges[]")?;
            let from = edge_object
                .get("from")
                .ok_or_else(|| ScheduleExportError::InvalidJsonShape {
                    message: "missing root.edges[].from".to_owned(),
                })?
                .as_string("root.edges[].from")?;
            let to = edge_object
                .get("to")
                .ok_or_else(|| ScheduleExportError::InvalidJsonShape {
                    message: "missing root.edges[].to".to_owned(),
                })?
                .as_string("root.edges[].to")?;
            parsed_edges.insert((from.to_owned(), to.to_owned()));
        }

        Ok(Self {
            nodes: parsed_nodes,
            edges: parsed_edges,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
enum JsonValue {
    Object(BTreeMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    String(String),
    Number,
    Boolean,
    Null,
}

impl JsonValue {
    fn as_object(
        &self,
        context: &'static str,
    ) -> Result<&BTreeMap<String, JsonValue>, ScheduleExportError> {
        match self {
            Self::Object(value) => Ok(value),
            _ => Err(ScheduleExportError::InvalidJsonShape {
                message: {
                    let mut message = context.to_owned();
                    message.push_str(" must be an object");
                    message
                },
            }),
        }
    }

    fn as_array(&self, context: &'static str) -> Result<&[JsonValue], ScheduleExportError> {
        match self {
            Self::Array(value) => Ok(value),
            _ => Err(ScheduleExportError::InvalidJsonShape {
                message: {
                    let mut message = context.to_owned();
                    message.push_str(" must be an array");
                    message
                },
            }),
        }
    }

    fn as_string(&self, context: &'static str) -> Result<&str, ScheduleExportError> {
        match self {
            Self::String(value) => Ok(value),
            _ => Err(ScheduleExportError::InvalidJsonShape {
                message: {
                    let mut message = context.to_owned();
                    message.push_str(" must be a string");
                    message
                },
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct JsonParser<'a> {
    input: &'a str,
    offset: usize,
}

impl<'a> JsonParser<'a> {
    const fn new(input: &'a str) -> Self {
        Self { input, offset: 0 }
    }

    fn parse(mut self) -> Result<JsonValue, ScheduleExportError> {
        let value = self.parse_value()?;
        self.skip_whitespace();
        if self.offset == self.input.len() {
            Ok(value)
        } else {
            Err(self.error("trailing content after JSON value"))
        }
    }

    fn parse_value(&mut self) -> Result<JsonValue, ScheduleExportError> {
        self.skip_whitespace();
        match self.peek_byte() {
            Some(b'{') => self.parse_object(),
            Some(b'[') => self.parse_array(),
            Some(b'"') => self.parse_string().map(JsonValue::String),
            Some(b't') => {
                self.consume_literal("true")?;
                Ok(JsonValue::Boolean)
            }
            Some(b'f') => {
                self.consume_literal("false")?;
                Ok(JsonValue::Boolean)
            }
            Some(b'n') => {
                self.consume_literal("null")?;
                Ok(JsonValue::Null)
            }
            Some(b'-' | b'0'..=b'9') => {
                self.parse_number()?;
                Ok(JsonValue::Number)
            }
            Some(_) => Err(self.error("unexpected byte while parsing JSON value")),
            None => Err(self.error("unexpected end of input while parsing JSON value")),
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, ScheduleExportError> {
        self.consume_byte(b'{')?;
        let mut values = BTreeMap::new();
        loop {
            self.skip_whitespace();
            if self.consume_byte_if(b'}') {
                break;
            }

            let key = self.parse_string()?;
            self.skip_whitespace();
            self.consume_byte(b':')?;
            let value = self.parse_value()?;
            values.insert(key, value);
            self.skip_whitespace();
            if self.consume_byte_if(b'}') {
                break;
            }
            self.consume_byte(b',')?;
        }

        Ok(JsonValue::Object(values))
    }

    fn parse_array(&mut self) -> Result<JsonValue, ScheduleExportError> {
        self.consume_byte(b'[')?;
        let mut values = Vec::new();
        loop {
            self.skip_whitespace();
            if self.consume_byte_if(b']') {
                break;
            }

            values.push(self.parse_value()?);
            self.skip_whitespace();
            if self.consume_byte_if(b']') {
                break;
            }
            self.consume_byte(b',')?;
        }

        Ok(JsonValue::Array(values))
    }

    fn parse_string(&mut self) -> Result<String, ScheduleExportError> {
        self.consume_byte(b'"')?;
        let mut value = String::new();
        loop {
            let Some(byte) = self.next_byte() else {
                return Err(self.error("unexpected end of input while parsing JSON string"));
            };

            match byte {
                b'"' => return Ok(value),
                b'\\' => {
                    let Some(escaped) = self.next_byte() else {
                        return Err(self.error("unexpected end of input in JSON string escape"));
                    };
                    match escaped {
                        b'"' => value.push('"'),
                        b'\\' => value.push('\\'),
                        b'/' => value.push('/'),
                        b'b' => value.push('\u{0008}'),
                        b'f' => value.push('\u{000C}'),
                        b'n' => value.push('\n'),
                        b'r' => value.push('\r'),
                        b't' => value.push('\t'),
                        b'u' => return Err(self.error("unicode JSON escapes are unsupported")),
                        _ => return Err(self.error("unsupported JSON string escape")),
                    }
                }
                0x00..=0x1f => return Err(self.error("control byte in JSON string")),
                value_byte => value.push(char::from(value_byte)),
            }
        }
    }

    fn parse_number(&mut self) -> Result<(), ScheduleExportError> {
        self.consume_byte_if(b'-');
        self.consume_digit()?;

        while matches!(self.peek_byte(), Some(b'0'..=b'9')) {
            self.offset += 1;
        }

        if self.consume_byte_if(b'.') {
            self.consume_digit()?;
            while matches!(self.peek_byte(), Some(b'0'..=b'9')) {
                self.offset += 1;
            }
        }

        if matches!(self.peek_byte(), Some(b'e' | b'E')) {
            self.offset += 1;
            if matches!(self.peek_byte(), Some(b'+' | b'-')) {
                self.offset += 1;
            }
            self.consume_digit()?;
            while matches!(self.peek_byte(), Some(b'0'..=b'9')) {
                self.offset += 1;
            }
        }

        Ok(())
    }

    fn consume_digit(&mut self) -> Result<(), ScheduleExportError> {
        match self.peek_byte() {
            Some(b'0'..=b'9') => {
                self.offset += 1;
                Ok(())
            }
            _ => Err(self.error("expected JSON digit")),
        }
    }

    fn consume_literal(&mut self, literal: &str) -> Result<(), ScheduleExportError> {
        if self.input[self.offset..].starts_with(literal) {
            self.offset += literal.len();
            Ok(())
        } else {
            Err(self.error("invalid JSON literal"))
        }
    }

    fn consume_byte(&mut self, expected: u8) -> Result<(), ScheduleExportError> {
        if self.consume_byte_if(expected) {
            Ok(())
        } else {
            Err(self.error("unexpected byte"))
        }
    }

    fn consume_byte_if(&mut self, expected: u8) -> bool {
        if self.peek_byte() == Some(expected) {
            self.offset += 1;
            true
        } else {
            false
        }
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek_byte(), Some(b' ' | b'\n' | b'\r' | b'\t')) {
            self.offset += 1;
        }
    }

    fn peek_byte(&self) -> Option<u8> {
        self.input.as_bytes().get(self.offset).copied()
    }

    fn next_byte(&mut self) -> Option<u8> {
        let value = self.peek_byte()?;
        self.offset += 1;
        Some(value)
    }

    fn error(&self, message: &'static str) -> ScheduleExportError {
        let mut text = message.to_owned();
        text.push_str(" at byte ");
        text.push_str(&self.offset.to_string());
        ScheduleExportError::JsonParse { message: text }
    }
}
