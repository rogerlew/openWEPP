use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::canonical::{parse_strict, sha256_bytes, validate_schema};
use crate::error::{ErrorClass, GatePolicyError, Result};

#[derive(Debug, Clone, Deserialize)]
pub struct ImpactMap {
    pub policy_id: String,
    pub policy_sha256: String,
    pub generation: u64,
    pub enforcement_status: String,
    pub unknown_path_action: String,
    pub entries: Vec<ImpactEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ImpactEntry {
    pub entry_id: String,
    pub matcher: Matcher,
    pub owner: String,
    pub semantic_surface: String,
    pub risk_floor: RiskClass,
    pub reason_codes: Vec<String>,
    pub affected_packages: Vec<String>,
    pub test_targets: Vec<String>,
    pub covering_test_targets: Vec<String>,
    pub contracts: Vec<String>,
    pub authority_suites: Vec<String>,
    pub assurance_watches: Vec<String>,
    pub gate_definition_ids: Vec<String>,
    pub documentation_paths: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Matcher {
    pub kind: String,
    pub value: String,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RiskClass {
    Editorial,
    BoundedComponent,
    IntegratedDomain,
    Critical,
}

impl RiskClass {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Editorial => "EDITORIAL",
            Self::BoundedComponent => "BOUNDED_COMPONENT",
            Self::IntegratedDomain => "INTEGRATED_DOMAIN",
            Self::Critical => "CRITICAL",
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GateRegistry {
    pub generation: u64,
    pub enforcement_status: String,
    pub definitions: Vec<GateDefinition>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GateDefinition {
    pub gate_definition_id: String,
    pub gate_family: String,
    pub target_template: String,
    pub risk_classes: Vec<RiskClass>,
    pub executor: Value,
    pub arguments_template: Vec<String>,
    pub environment_allowlist: Vec<String>,
    pub authority_class: String,
    pub outcome_policy: String,
    pub failure_classification: String,
    pub owner: String,
    pub investigation_owner: String,
    pub boundary: String,
    pub trust_requirement: String,
    pub reuse_class: String,
    pub inventory_mode: String,
    pub inventory_source: String,
    pub minimum_count: u64,
    pub acceptance: Value,
    pub timeout_seconds: u64,
    pub maximum_attempts: u64,
    pub permitted_retry_reasons: Vec<String>,
    pub artifact_contract: String,
    pub output_paths: Vec<String>,
    pub blocks_transition: String,
    pub identity_breakers: Vec<String>,
    pub prerequisite_definition_ids: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PolicyBundle {
    pub root: PathBuf,
    pub impact_map: ImpactMap,
    pub impact_map_value: Value,
    pub impact_map_sha256: String,
    pub registry: GateRegistry,
    pub registry_value: Value,
    pub execution_matrix_value: Value,
    definitions: BTreeMap<String, GateDefinition>,
}

impl PolicyBundle {
    /// Load and cross-check the shadow policy bundle rooted in a repository.
    ///
    /// # Errors
    ///
    /// Returns a typed error for unreadable, invalid, drifting, or cyclic policy.
    pub fn load(repo_root: &Path) -> Result<Self> {
        let root = repo_root.join("gate-policy/v1");
        let (impact_map, impact_map_value, impact_bytes) = load_impact_map(&root)?;
        let (registry, registry_value) = load_gate_registry(&root)?;
        let execution_matrix_value = load_execution_matrix(&root)?;
        validate_policy_posture(repo_root, &impact_map, &registry)?;
        let definitions = definition_map(&registry)?;
        verify_impact_bindings(&impact_map, &definitions)?;
        verify_definition_dag(&definitions)?;
        verify_adapter_digests(repo_root, &definitions)?;

        Ok(Self {
            root,
            impact_map,
            impact_map_value,
            impact_map_sha256: sha256_bytes(&impact_bytes),
            registry,
            registry_value,
            execution_matrix_value,
            definitions,
        })
    }

    #[must_use]
    pub fn definition(&self, id: &str) -> Option<&GateDefinition> {
        self.definitions.get(id)
    }

    #[must_use]
    pub fn definitions_for_risk(&self, risk: RiskClass) -> Vec<&GateDefinition> {
        self.definitions
            .values()
            .filter(|definition| definition.risk_classes.contains(&risk))
            .collect()
    }

    #[must_use]
    pub fn matching_entries(&self, path: &str) -> Vec<&ImpactEntry> {
        self.impact_map
            .entries
            .iter()
            .filter(|entry| matcher_matches(&entry.matcher, path))
            .collect()
    }
}

fn load_execution_matrix(root: &Path) -> Result<Value> {
    let value = load_json(&root.join("execution-matrix.json"))?;
    let schema = load_json(&root.join("schemas/execution-matrix.schema.json"))?;
    validate_schema(&schema, &value, "execution matrix")?;
    Ok(value)
}

fn verify_adapter_digests(
    repo_root: &Path,
    definitions: &BTreeMap<String, GateDefinition>,
) -> Result<()> {
    for definition in definitions
        .values()
        .filter(|definition| definition.executor["kind"] == "LEGACY_ADAPTER_V1")
    {
        let path = definition.arguments_template.get(1).ok_or_else(|| {
            GatePolicyError::new(
                ErrorClass::Policy,
                "GATE-ADAPTER-PATH",
                &definition.gate_definition_id,
            )
        })?;
        let actual = sha256_bytes(&read(&repo_root.join(path))?);
        if definition.executor["adapter_sha256"] != actual {
            return Err(GatePolicyError::new(
                ErrorClass::Policy,
                "GATE-ADAPTER-DIGEST",
                &definition.gate_definition_id,
            ));
        }
    }
    Ok(())
}

fn load_impact_map(root: &Path) -> Result<(ImpactMap, Value, Vec<u8>)> {
    let bytes = read(&root.join("impact-map.json"))?;
    let value = parse_strict(&bytes)?;
    let schema = load_json(&root.join("schemas/impact-map.schema.json"))?;
    validate_schema(&schema, &value, "impact map")?;
    let map = serde_json::from_value(value.clone()).map_err(|error| {
        GatePolicyError::new(ErrorClass::Policy, "GATE-IMPACT-DECODE", error.to_string())
    })?;
    Ok((map, value, bytes))
}

fn load_gate_registry(root: &Path) -> Result<(GateRegistry, Value)> {
    let bytes = read(&root.join("gate-definitions.json"))?;
    let value = parse_strict(&bytes)?;
    let schema = load_json(&root.join("schemas/gate-definitions.schema.json"))?;
    validate_schema(&schema, &value, "gate definitions")?;
    let registry = serde_json::from_value(value.clone()).map_err(|error| {
        GatePolicyError::new(
            ErrorClass::Policy,
            "GATE-DEFINITION-DECODE",
            error.to_string(),
        )
    })?;
    Ok((registry, value))
}

fn validate_policy_posture(
    repo_root: &Path,
    impact_map: &ImpactMap,
    registry: &GateRegistry,
) -> Result<()> {
    if impact_map.enforcement_status != "SHADOW"
        || registry.enforcement_status != "SHADOW"
        || impact_map.unknown_path_action != "ESCALATE_CRITICAL"
    {
        return Err(GatePolicyError::new(
            ErrorClass::Policy,
            "GATE-POLICY-NONSHADOW",
            "TESTGATE-PLAN-01 requires SHADOW policy and critical unknown fallback",
        ));
    }
    let strategy = read(&repo_root.join("docs/standards/testing-and-gate-strategy.md"))?;
    if impact_map.policy_sha256 == sha256_bytes(&strategy) {
        Ok(())
    } else {
        Err(GatePolicyError::new(
            ErrorClass::Policy,
            "GATE-POLICY-DIGEST-DRIFT",
            "impact map does not bind the current testing strategy",
        ))
    }
}

fn definition_map(registry: &GateRegistry) -> Result<BTreeMap<String, GateDefinition>> {
    let mut definitions = BTreeMap::new();
    for definition in &registry.definitions {
        if definitions
            .insert(definition.gate_definition_id.clone(), definition.clone())
            .is_some()
        {
            return Err(GatePolicyError::new(
                ErrorClass::Policy,
                "GATE-DEFINITION-DUPLICATE",
                format!("duplicate definition {}", definition.gate_definition_id),
            ));
        }
    }
    Ok(definitions)
}

fn verify_impact_bindings(
    impact_map: &ImpactMap,
    definitions: &BTreeMap<String, GateDefinition>,
) -> Result<()> {
    for entry in &impact_map.entries {
        if !matches!(
            entry.matcher.kind.as_str(),
            "exact_path" | "path_prefix" | "path_glob"
        ) {
            return Err(GatePolicyError::new(
                ErrorClass::Policy,
                "GATE-MATCHER-UNSUPPORTED",
                format!("{} uses {}", entry.entry_id, entry.matcher.kind),
            ));
        }
        if !entry.assurance_watches.is_empty() {
            return Err(GatePolicyError::new(
                ErrorClass::Policy,
                "GATE-ASSURANCE-WATCH-UNSUPPORTED",
                format!("{} requires assurance impact construction", entry.entry_id),
            ));
        }
        if !entry.contracts.is_empty() || !entry.authority_suites.is_empty() {
            return Err(GatePolicyError::new(
                ErrorClass::Policy,
                "GATE-SEMANTIC-OBLIGATION-UNSUPPORTED",
                format!(
                    "{} requires contract/authority executable binding",
                    entry.entry_id
                ),
            ));
        }
        for id in &entry.gate_definition_ids {
            if !definitions.contains_key(id) {
                return Err(GatePolicyError::new(
                    ErrorClass::Policy,
                    "GATE-DEFINITION-MISSING",
                    format!("impact entry {} references {id}", entry.entry_id),
                ));
            }
        }
        for target in entry
            .test_targets
            .iter()
            .chain(&entry.covering_test_targets)
        {
            let covered = entry.gate_definition_ids.iter().any(|id| {
                definitions.get(id).is_some_and(|definition| {
                    definition
                        .arguments_template
                        .iter()
                        .any(|argument| argument == target)
                })
            });
            if !covered {
                return Err(GatePolicyError::new(
                    ErrorClass::Policy,
                    "GATE-TEST-TARGET-UNCOVERED",
                    format!("{} does not bind {target}", entry.entry_id),
                ));
            }
        }
    }
    Ok(())
}

fn read(path: &Path) -> Result<Vec<u8>> {
    fs::read(path).map_err(|error| {
        GatePolicyError::new(
            ErrorClass::Io,
            "GATE-FILE-READ",
            format!("{}: {error}", path.display()),
        )
    })
}

fn load_json(path: &Path) -> Result<Value> {
    parse_strict(&read(path)?)
}

fn matcher_matches(matcher: &Matcher, path: &str) -> bool {
    match matcher.kind.as_str() {
        "exact_path" => path == matcher.value,
        "path_prefix" => {
            path == matcher.value.trim_end_matches('/')
                || path.starts_with(&format!("{}/", matcher.value.trim_end_matches('/')))
        }
        "path_glob" => wildmatch(matcher.value.as_bytes(), path.as_bytes()),
        _ => false,
    }
}

fn wildmatch(pattern: &[u8], value: &[u8]) -> bool {
    fn matches(
        pattern: &[u8],
        value: &[u8],
        pi: usize,
        vi: usize,
        seen: &mut BTreeSet<(usize, usize)>,
    ) -> bool {
        if !seen.insert((pi, vi)) {
            return false;
        }
        if pi == pattern.len() {
            return vi == value.len();
        }
        if pattern[pi] == b'*' {
            let double = pattern.get(pi + 1) == Some(&b'*');
            let next = pi + usize::from(double) + 1;
            return matches(pattern, value, next, vi, seen)
                || (vi < value.len()
                    && (double || value[vi] != b'/')
                    && matches(pattern, value, pi, vi + 1, seen));
        }
        vi < value.len()
            && (pattern[pi] == value[vi] || (pattern[pi] == b'?' && value[vi] != b'/'))
            && matches(pattern, value, pi + 1, vi + 1, seen)
    }
    matches(pattern, value, 0, 0, &mut BTreeSet::new())
}

fn verify_definition_dag(definitions: &BTreeMap<String, GateDefinition>) -> Result<()> {
    fn visit(
        id: &str,
        definitions: &BTreeMap<String, GateDefinition>,
        visiting: &mut BTreeSet<String>,
        visited: &mut BTreeSet<String>,
    ) -> Result<()> {
        if visited.contains(id) {
            return Ok(());
        }
        if !visiting.insert(id.to_owned()) {
            return Err(GatePolicyError::new(
                ErrorClass::Policy,
                "GATE-DEFINITION-CYCLE",
                format!("cycle at {id}"),
            ));
        }
        let definition = definitions.get(id).ok_or_else(|| {
            GatePolicyError::new(
                ErrorClass::Policy,
                "GATE-PREREQUISITE-MISSING",
                format!("missing prerequisite {id}"),
            )
        })?;
        for dependency in &definition.prerequisite_definition_ids {
            visit(dependency, definitions, visiting, visited)?;
        }
        visiting.remove(id);
        visited.insert(id.to_owned());
        Ok(())
    }

    let mut visiting = BTreeSet::new();
    let mut visited = BTreeSet::new();
    for id in definitions.keys() {
        visit(id, definitions, &mut visiting, &mut visited)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{Matcher, matcher_matches};

    #[test]
    fn component_prefix_and_git_style_glob_are_bounded() {
        let prefix = Matcher {
            kind: "path_prefix".to_owned(),
            value: "gate-policy/".to_owned(),
        };
        assert!(matcher_matches(&prefix, "gate-policy/v1/impact-map.json"));
        assert!(!matcher_matches(&prefix, "gate-policy-other/file"));
        let glob = Matcher {
            kind: "path_glob".to_owned(),
            value: "crates/**/src/*.rs".to_owned(),
        };
        assert!(matcher_matches(&glob, "crates/a/src/lib.rs"));
        assert!(!matcher_matches(&glob, "crates/a/tests/x.rs"));
    }
}
