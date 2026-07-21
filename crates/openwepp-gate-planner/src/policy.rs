use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::assurance::validate_wildmatch_pattern;
use crate::canonical::{derived_id, digest, parse_strict, sha256_bytes, validate_schema};
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
pub struct AssuranceRegistry {
    pub policy_id: String,
    pub generation: u64,
    pub reports: Vec<AssuranceReport>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AssuranceReport {
    pub report_id: String,
    pub watch_generation: u64,
    pub source_root: String,
    pub assessed_realization_root: String,
    pub resolution_authority: AssuranceAuthority,
    pub watches: Vec<AssuranceWatch>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AssuranceAuthority {
    pub principal_id: Option<String>,
    pub role_id: String,
    pub role_record_sha256: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AssuranceWatch {
    pub watch_id: String,
    pub owner: String,
    pub kind: String,
    pub match_value: String,
    pub lifecycle_boundary: String,
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
    #[serde(default)]
    pub combined_quality_proofs: Vec<Value>,
    #[serde(default)]
    pub active_combined_quality_proof_id: Option<String>,
    pub definitions: Vec<GateDefinition>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GateDefinition {
    pub gate_definition_id: String,
    pub gate_family: String,
    pub execution_cost_class: String,
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
    pub assurance_registry: AssuranceRegistry,
    pub assurance_registry_value: Value,
    pub assurance_registry_sha256: String,
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
        verify_combined_quality_proofs(&registry)?;
        let execution_matrix_value = load_execution_matrix(&root)?;
        let (assurance_registry, assurance_registry_value, assurance_registry_bytes) =
            load_assurance_registry(repo_root, &root)?;
        validate_policy_posture(repo_root, &impact_map, &registry)?;
        verify_assurance_registry(&impact_map, &assurance_registry)?;
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
            assurance_registry,
            assurance_registry_value,
            assurance_registry_sha256: sha256_bytes(&assurance_registry_bytes),
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
            .filter(|definition| {
                definition.gate_definition_id != "combined-workspace-quality-v1"
                    && definition.risk_classes.contains(&risk)
            })
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

fn verify_combined_quality_proofs(registry: &GateRegistry) -> Result<()> {
    let mut ids = BTreeSet::new();
    for proof in &registry.combined_quality_proofs {
        let id = proof["proof_id"].as_str().ok_or_else(|| {
            GatePolicyError::new(
                ErrorClass::Policy,
                "GATE-COMBINED-PROOF-ID",
                "missing proof ID",
            )
        })?;
        if derived_id(proof, "proof_id")? != id || !ids.insert(id) {
            return Err(GatePolicyError::new(
                ErrorClass::Policy,
                "GATE-COMBINED-PROOF-ID",
                "combined-quality proof identity is invalid or duplicated",
            ));
        }
    }
    if registry
        .active_combined_quality_proof_id
        .as_ref()
        .is_some_and(|active| !ids.contains(active.as_str()))
    {
        return Err(GatePolicyError::new(
            ErrorClass::Policy,
            "GATE-COMBINED-ACTIVE-PROOF",
            "active combined-quality proof is absent from the reviewed registry",
        ));
    }
    Ok(())
}

#[derive(Deserialize)]
struct AssuranceCatalog {
    reports: Vec<AssuranceCatalogReport>,
}

#[derive(Deserialize)]
struct AssuranceCatalogReport {
    id: String,
}

#[derive(Deserialize)]
struct PrincipalCatalog {
    principals: Vec<PrincipalRecord>,
}

#[derive(Deserialize)]
struct PrincipalRecord {
    id: String,
    roles: Vec<String>,
    record_version: u64,
}

#[derive(Deserialize)]
struct ReviewLock {
    report_id: String,
    science_root: String,
    preapproval_realization_root: String,
    realization_root: Option<String>,
}

#[derive(Deserialize)]
struct AssuranceLifecycle {
    id: String,
    authorship: AssuranceAuthorship,
}

#[derive(Deserialize)]
struct AssuranceAuthorship {
    human_report_lead: Option<String>,
}

fn load_assurance_registry(
    repo_root: &Path,
    policy_root: &Path,
) -> Result<(AssuranceRegistry, Value, Vec<u8>)> {
    let bytes = read(&policy_root.join("assurance-registry.json"))?;
    let value = parse_strict(&bytes)?;
    let schema = load_json(&policy_root.join("schemas/assurance-registry.schema.json"))?;
    validate_schema(&schema, &value, "assurance registry")?;
    let registry: AssuranceRegistry = serde_json::from_value(value.clone()).map_err(|error| {
        GatePolicyError::new(
            ErrorClass::Policy,
            "GATE-ASSURANCE-REGISTRY-DECODE",
            error.to_string(),
        )
    })?;
    let catalog_bytes = read(&repo_root.join("assurance/v2/catalog.yaml"))?;
    let catalog: AssuranceCatalog = serde_yaml::from_slice(&catalog_bytes).map_err(|error| {
        GatePolicyError::new(
            ErrorClass::Policy,
            "GATE-ASSURANCE-CATALOG-DECODE",
            error.to_string(),
        )
    })?;
    let catalog_ids = catalog
        .reports
        .into_iter()
        .map(|report| report.id)
        .collect::<BTreeSet<_>>();
    let registry_ids = registry
        .reports
        .iter()
        .map(|report| report.report_id.clone())
        .collect::<BTreeSet<_>>();
    if catalog_ids != registry_ids || registry_ids.len() != registry.reports.len() {
        return Err(GatePolicyError::new(
            ErrorClass::Policy,
            "GATE-ASSURANCE-REGISTRY-COVERAGE",
            "assurance registry report set must equal the canonical catalog",
        ));
    }
    let principal_bytes = read(&repo_root.join("assurance/v2/principals.yaml"))?;
    let principals: PrincipalCatalog =
        serde_yaml::from_slice(&principal_bytes).map_err(|error| {
            GatePolicyError::new(
                ErrorClass::Policy,
                "GATE-ASSURANCE-PRINCIPAL-DECODE",
                error.to_string(),
            )
        })?;
    verify_resolution_authorities(&registry, &principals)?;
    verify_lifecycle_authorities(repo_root, &registry)?;
    verify_assessed_roots(repo_root, &registry)?;
    Ok((registry, value, bytes))
}

fn verify_lifecycle_authorities(repo_root: &Path, registry: &AssuranceRegistry) -> Result<()> {
    for report in &registry.reports {
        let path = repo_root.join(format!(
            "assurance/v2/reports/{}/report.yaml",
            report.report_id
        ));
        let lifecycle: AssuranceLifecycle =
            serde_yaml::from_slice(&read(&path)?).map_err(|error| {
                GatePolicyError::new(
                    ErrorClass::Policy,
                    "GATE-ASSURANCE-LIFECYCLE-DECODE",
                    error.to_string(),
                )
            })?;
        let authority = &report.resolution_authority;
        if lifecycle.id != report.report_id
            || authority.principal_id != lifecycle.authorship.human_report_lead
            || (authority.principal_id.is_some() && authority.role_id != "report_lead")
            || (authority.principal_id.is_none() && authority.role_id != "assurance_steward")
        {
            return Err(GatePolicyError::new(
                ErrorClass::Policy,
                "GATE-ASSURANCE-LIFECYCLE-AUTHORITY",
                &report.report_id,
            ));
        }
    }
    Ok(())
}

fn verify_assessed_roots(repo_root: &Path, registry: &AssuranceRegistry) -> Result<()> {
    for report in &registry.reports {
        let path = repo_root.join(format!(
            "assurance/v2/reports/{}/review.lock.json",
            report.report_id
        ));
        let lock: ReviewLock = serde_json::from_slice(&read(&path)?).map_err(|error| {
            GatePolicyError::new(
                ErrorClass::Policy,
                "GATE-ASSURANCE-REVIEW-LOCK-DECODE",
                error.to_string(),
            )
        })?;
        let assessed_root = lock
            .realization_root
            .as_deref()
            .unwrap_or(&lock.preapproval_realization_root);
        if lock.report_id != report.report_id
            || lock.science_root != report.source_root
            || assessed_root != report.assessed_realization_root
        {
            return Err(GatePolicyError::new(
                ErrorClass::Policy,
                "GATE-ASSURANCE-ASSESSED-ROOT",
                &report.report_id,
            ));
        }
    }
    Ok(())
}

fn verify_resolution_authorities(
    registry: &AssuranceRegistry,
    principals: &PrincipalCatalog,
) -> Result<()> {
    for report in &registry.reports {
        let authority = &report.resolution_authority;
        let Some(principal_id) = authority.principal_id.as_deref() else {
            continue;
        };
        let principal = principals
            .principals
            .iter()
            .find(|principal| principal.id == principal_id)
            .ok_or_else(|| {
                GatePolicyError::new(
                    ErrorClass::Policy,
                    "GATE-ASSURANCE-PRINCIPAL-UNKNOWN",
                    principal_id,
                )
            })?;
        if !principal.roles.contains(&authority.role_id) {
            return Err(GatePolicyError::new(
                ErrorClass::Policy,
                "GATE-ASSURANCE-ROLE-UNKNOWN",
                &authority.role_id,
            ));
        }
        let expected = digest(&json!({
            "principal_id": principal.id,
            "record_version": principal.record_version,
            "role_id": authority.role_id
        }))?;
        if authority.role_record_sha256.as_deref() != Some(expected.as_str()) {
            return Err(GatePolicyError::new(
                ErrorClass::Policy,
                "GATE-ASSURANCE-ROLE-DIGEST",
                &report.report_id,
            ));
        }
    }
    Ok(())
}

fn verify_assurance_registry(impact_map: &ImpactMap, registry: &AssuranceRegistry) -> Result<()> {
    if registry.policy_id != impact_map.policy_id {
        return Err(GatePolicyError::new(
            ErrorClass::Policy,
            "GATE-ASSURANCE-REGISTRY-POLICY",
            "assurance registry policy identity differs from the impact map",
        ));
    }
    let report_ids = registry
        .reports
        .iter()
        .map(|report| report.report_id.as_str())
        .collect::<Vec<_>>();
    require_sorted_unique(&report_ids, "GATE-ASSURANCE-REGISTRY-REPORT-ORDER")?;
    let mut watch_ids = BTreeSet::new();
    for report in &registry.reports {
        let authority = &report.resolution_authority;
        if authority.principal_id.is_some() != authority.role_record_sha256.is_some() {
            return Err(GatePolicyError::new(
                ErrorClass::Policy,
                "GATE-ASSURANCE-AUTHORITY-INCOMPLETE",
                &report.report_id,
            ));
        }
        let ids = report
            .watches
            .iter()
            .map(|watch| watch.watch_id.as_str())
            .collect::<Vec<_>>();
        require_sorted_unique(&ids, "GATE-ASSURANCE-REGISTRY-WATCH-ORDER")?;
        for id in ids {
            if !watch_ids.insert(id) {
                return Err(GatePolicyError::new(
                    ErrorClass::Policy,
                    "GATE-ASSURANCE-REGISTRY-WATCH-DUPLICATE",
                    id,
                ));
            }
        }
        for watch in &report.watches {
            verify_assurance_watch_value(watch)?;
        }
    }
    let known_watches = impact_map
        .entries
        .iter()
        .flat_map(|entry| entry.assurance_watches.iter())
        .collect::<BTreeSet<_>>();
    if let Some(unknown) = known_watches
        .iter()
        .find(|id| !watch_ids.contains(id.as_str()))
    {
        return Err(GatePolicyError::new(
            ErrorClass::Policy,
            "GATE-ASSURANCE-WATCH-UNKNOWN",
            (*unknown).clone(),
        ));
    }
    Ok(())
}

fn verify_assurance_watch_value(watch: &AssuranceWatch) -> Result<()> {
    let path_kind = matches!(
        watch.kind.as_str(),
        "exact_path" | "path_prefix" | "path_glob" | "result_procedure" | "builder_schema"
    );
    let invalid_component = watch
        .match_value
        .split('/')
        .any(|component| matches!(component, "." | ".."));
    if path_kind
        && (watch.match_value.starts_with('/') || watch.match_value.is_empty() || invalid_component)
    {
        Err(GatePolicyError::new(
            ErrorClass::Policy,
            "GATE-ASSURANCE-WATCH-PATH",
            &watch.watch_id,
        ))
    } else if watch.kind == "path_glob" && !validate_wildmatch_pattern(&watch.match_value) {
        Err(GatePolicyError::new(
            ErrorClass::Policy,
            "GATE-ASSURANCE-WATCH-GLOB",
            &watch.watch_id,
        ))
    } else {
        Ok(())
    }
}

fn require_sorted_unique(values: &[&str], code: &'static str) -> Result<()> {
    if values.windows(2).all(|pair| pair[0] < pair[1]) {
        Ok(())
    } else {
        Err(GatePolicyError::new(
            ErrorClass::Policy,
            code,
            "registry entries must be byte-sorted and unique",
        ))
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
    if impact_map.enforcement_status != "BLOCKING"
        || registry.enforcement_status != "BLOCKING"
        || impact_map.unknown_path_action != "ESCALATE_CRITICAL"
    {
        return Err(GatePolicyError::new(
            ErrorClass::Policy,
            "GATE-POLICY-NONBLOCKING",
            "TESTGATE requires BLOCKING policy and critical unknown fallback",
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
        if definition
            .environment_allowlist
            .iter()
            .any(|key| key == "RUSTUP_HOME")
            && !definition
                .environment_allowlist
                .iter()
                .any(|key| key == "RUSTUP_TOOLCHAIN")
        {
            return Err(GatePolicyError::new(
                ErrorClass::Policy,
                "GATE-RUSTUP-TOOLCHAIN-UNPINNED",
                format!(
                    "{} allows RUSTUP_HOME without RUSTUP_TOOLCHAIN",
                    definition.gate_definition_id
                ),
            ));
        }
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
        for id in &entry.gate_definition_ids {
            if !definitions.contains_key(id) {
                return Err(GatePolicyError::new(
                    ErrorClass::Policy,
                    "GATE-DEFINITION-MISSING",
                    format!("impact entry {} references {id}", entry.entry_id),
                ));
            }
        }
        if !entry.contracts.is_empty() {
            if entry.contracts.len() != 1 || entry.covering_test_targets.is_empty() {
                return Err(GatePolicyError::new(
                    ErrorClass::Policy,
                    "GATE-A1-BINDING-MISSING",
                    format!(
                        "{} requires one contract and at least one A1 hard-invariant target",
                        entry.entry_id
                    ),
                ));
            }
            for target in &entry.covering_test_targets {
                let hard_invariant = entry.gate_definition_ids.iter().any(|id| {
                    definitions.get(id).is_some_and(|definition| {
                        definition.authority_class == "A1"
                            && definition.executor["kind"] == "NEXTEST_V1"
                            && definition.outcome_policy == "BLOCKING"
                            && definition.failure_classification == "HARD_FAIL"
                            && definition.inventory_mode == "EXACT"
                            && matches!(
                                definition.inventory_source.as_str(),
                                "NEXTEST_PACKAGE"
                                    | "NEXTEST_PACKAGES"
                                    | "NEXTEST_WORKSPACE"
                                    | "NEXTEST_TEST_TARGET"
                            )
                            && definition
                                .arguments_template
                                .iter()
                                .any(|argument| argument == target)
                    })
                });
                if !hard_invariant {
                    return Err(GatePolicyError::new(
                        ErrorClass::Policy,
                        "GATE-A1-BINDING-MISSING",
                        format!(
                            "{} does not bind {target} to a hard-fail A1 gate",
                            entry.entry_id
                        ),
                    ));
                }
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
    use std::path::Path;

    use super::{AssuranceRegistry, Matcher, matcher_matches, verify_lifecycle_authorities};

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

    #[test]
    fn registry_authority_must_equal_the_report_lifecycle_selection() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let bytes =
            std::fs::read(root.join("gate-policy/v1/assurance-registry.json")).expect("registry");
        let mut registry: AssuranceRegistry =
            serde_json::from_slice(&bytes).expect("registry JSON");
        let snow = registry
            .reports
            .iter_mut()
            .find(|report| report.report_id == "snow-and-frozen-soil-process-evaluation")
            .expect("snow report");
        snow.resolution_authority.principal_id = None;
        let error = verify_lifecycle_authorities(&root, &registry)
            .expect_err("registry cannot substitute lifecycle authority");
        assert_eq!(error.code, "GATE-ASSURANCE-LIFECYCLE-AUTHORITY");

        let mut registry: AssuranceRegistry =
            serde_json::from_slice(&bytes).expect("registry JSON");
        let groundwater = registry
            .reports
            .iter_mut()
            .find(|report| report.report_id == "linear-groundwater-reservoir-recurrence")
            .expect("groundwater report");
        groundwater.resolution_authority.role_id = "arbitrary_unassigned_role".to_owned();
        let error = verify_lifecycle_authorities(&root, &registry)
            .expect_err("null principal cannot substitute the unresolved lifecycle role");
        assert_eq!(error.code, "GATE-ASSURANCE-LIFECYCLE-AUTHORITY");
    }
}
