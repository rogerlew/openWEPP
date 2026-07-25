//! Prospective work-package validation without retroactive execution authority.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Component, Path};
use std::process::Command;

use serde_json::{Value, json};

use crate::canonical::{derived_id, parse_strict, sha256_bytes, validate_schema};
use crate::error::{ErrorClass, GatePolicyError, Result};

/// Validate package shape and base-commit authority for the current diff.
///
/// A package absent from the authenticated base produces a useful BLOCKED
/// artifact with `SCAFFOLD_COMMIT_REQUIRED`; it never authorizes execution.
///
/// # Errors
///
/// Returns a typed error only when repository state cannot be read or the
/// produced artifact violates its schema.
pub fn validate_package(repo: &Path, base: &str, package: &Path) -> Result<Value> {
    validate_package_path(package)?;
    let mut evidence = collect_package_audit_evidence(repo, base, package)?;
    let disposition = package_audit_disposition(&mut evidence)?;
    build_package_audit(repo, base, package, &evidence, disposition)
}

struct PackageAuditEvidence {
    current_digest: String,
    current_text: String,
    changed_paths: Vec<String>,
    base_text: Option<Vec<u8>>,
}

fn collect_package_audit_evidence(
    repo: &Path,
    base: &str,
    package: &Path,
) -> Result<PackageAuditEvidence> {
    let package_text = fs::read(package_absolute(repo, package)).map_err(|error| {
        package_error(
            "GATE-PACKAGE-READ",
            format!("{}: {error}", package.display()),
        )
    })?;
    let current_digest = sha256_bytes(&package_text);
    let current_text = String::from_utf8(package_text)
        .map_err(|error| package_error("GATE-PACKAGE-UTF8", error.to_string()))?;
    let changed_paths = changed_paths(repo, base)?;
    let base_text = git_show(repo, base, package)?;
    Ok(PackageAuditEvidence {
        current_digest,
        current_text,
        changed_paths,
        base_text,
    })
}

fn package_audit_disposition(evidence: &mut PackageAuditEvidence) -> Result<Disposition> {
    let disposition = match evidence.base_text.take() {
        None => (
            "BLOCKED",
            vec!["SCAFFOLD_COMMIT_REQUIRED"],
            None,
            parse_write_set(&evidence.current_text).unwrap_or_default(),
            evidence.changed_paths.clone(),
        ),
        Some(base_bytes) => {
            let base_digest = Some(sha256_bytes(&base_bytes));
            let base_text = String::from_utf8(base_bytes)
                .map_err(|error| package_error("GATE-PACKAGE-UTF8", error.to_string()))?;
            disposition(
                &base_text,
                &evidence.current_text,
                &evidence.changed_paths,
                base_digest,
            )
        }
    };
    Ok(disposition)
}

fn build_package_audit(
    repo: &Path,
    base: &str,
    package: &Path,
    evidence: &PackageAuditEvidence,
    disposition: Disposition,
) -> Result<Value> {
    let (status, reasons, base_digest, declared, unauthorized) = disposition;
    let mut audit = json!({
        "schema_version": "openwepp-package-audit-v1",
        "package_audit_id": "0".repeat(64),
        "status": status,
        "reason_codes": reasons,
        "base_commit": base,
        "package_path": package.to_string_lossy(),
        "base_package_sha256": base_digest,
        "current_package_sha256": evidence.current_digest,
        "declared_write_set": declared,
        "changed_paths": evidence.changed_paths,
        "unauthorized_paths": unauthorized,
    });
    audit["package_audit_id"] = Value::String(derived_id(&audit, "package_audit_id")?);
    let schema = read_json(&repo.join("gate-policy/v1/schemas/package-audit.schema.json"))?;
    validate_schema(&schema, &audit, "package audit")?;
    Ok(audit)
}

/// Reconstruct prospective package authority across each first-parent commit.
///
/// Package text is always read from the parent of the commit it authorizes. A
/// newly added package may establish its own future authority by covering only
/// changes below its package directory in the scaffold commit; it cannot cover
/// another path until a later commit.
///
/// # Errors
///
/// Returns a typed error when Git state, package bytes, or the canonical schema
/// cannot be read. Authority failures are represented as an `INVALID` artifact.
#[allow(clippy::too_many_lines)] // Keep the ordered chain state transition visible in one audit routine.
pub fn validate_package_chain(
    repo: &Path,
    base: &str,
    head: Option<&str>,
    intent_package: &Path,
) -> Result<Value> {
    validate_package_path(intent_package)?;
    let base_commit = resolve_commit(repo, base)?;
    let head_commit = resolve_commit(repo, head.unwrap_or("HEAD"))?;
    let intent_path = intent_package.to_string_lossy().into_owned();
    let intent = read_package_at(repo, &base_commit, &intent_path)?.ok_or_else(|| {
        package_error(
            "GATE-PACKAGE-CHAIN-ANCHOR-MISSING",
            intent_package.display().to_string(),
        )
    })?;
    if !intent.active {
        return Err(package_error(
            "GATE-PACKAGE-CHAIN-ANCHOR-INACTIVE",
            intent_package.display().to_string(),
        ));
    }
    let commits = first_parent_commits(repo, &base_commit, &head_commit)?;
    let changed_paths = changed_paths_between(repo, &base_commit, &head_commit)?;
    let mut introductions = BTreeMap::from([(
        intent_path.clone(),
        Introduction {
            sequence: 0,
            commit: base_commit.clone(),
        },
    )]);
    let mut planning_introductions = BTreeMap::<String, Introduction>::new();
    let mut steps = Vec::new();
    let mut reason_codes = if commits.is_empty() {
        vec!["ZERO_WORK_INCREMENT"]
    } else {
        Vec::new()
    };
    let mut unauthorized_paths = Vec::new();
    let mut expected_parent = base_commit.clone();

    for (sequence, commit) in commits.iter().enumerate() {
        let parent = first_parent(repo, commit)?;
        if parent != expected_parent {
            return Err(package_error(
                "GATE-PACKAGE-CHAIN-FIRST-PARENT",
                format!("expected {expected_parent} before {commit}, found {parent}"),
            ));
        }
        let mut outcome = validate_chain_step(
            repo,
            &parent,
            commit,
            sequence + 1,
            &introductions,
            &planning_introductions,
        )?;
        outcome.value["parent_tree"] = Value::String(resolve_tree(repo, &parent)?);
        outcome.value["commit_tree"] = Value::String(resolve_tree(repo, commit)?);
        for scaffold in &outcome.scaffolds {
            introductions.insert(
                scaffold.clone(),
                Introduction {
                    sequence: sequence + 1,
                    commit: commit.clone(),
                },
            );
        }
        for scaffold in &outcome.planning_scaffolds {
            planning_introductions.insert(
                scaffold.clone(),
                Introduction {
                    sequence: sequence + 1,
                    commit: commit.clone(),
                },
            );
        }
        expected_parent.clone_from(commit);
        if outcome.status == "INVALID" {
            reason_codes.extend(outcome.reason_codes.iter().copied());
            unauthorized_paths.extend(outcome.unauthorized_paths.iter().cloned());
        }
        steps.push(outcome.value);
        if !reason_codes.is_empty() {
            break;
        }
    }

    reason_codes.sort_unstable();
    reason_codes.dedup();
    unauthorized_paths.sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
    unauthorized_paths.dedup();
    let prompt_owner = active_prompt_owner(repo, &head_commit, &intent_path)?;
    if reason_codes.is_empty() && !chain_uses_intent(&steps, &intent_path) {
        reason_codes.push("INTENT_PACKAGE_NOT_PARTICIPATING");
    }
    let status = if reason_codes.is_empty() {
        "READY"
    } else {
        "INVALID"
    };
    let mut chain = json!({
        "schema_version": "openwepp-package-authority-chain-v1",
        "package_authority_chain_id": "0".repeat(64),
        "status": status,
        "reason_codes": reason_codes,
        "base_commit": base_commit,
        "head_commit": head_commit,
        "intent_package_path": intent_path,
        "intent_package_base_sha256": intent.digest,
        "prompt_owner": prompt_owner,
        "changed_paths": changed_paths,
        "unauthorized_paths": unauthorized_paths,
        "steps": steps,
    });
    chain["package_authority_chain_id"] =
        Value::String(derived_id(&chain, "package_authority_chain_id")?);
    let schema =
        read_json(&repo.join("gate-policy/v1/schemas/package-authority-chain.schema.json"))?;
    validate_schema(&schema, &chain, "package authority chain")?;
    Ok(chain)
}

#[derive(Clone)]
struct Introduction {
    sequence: usize,
    commit: String,
}

struct PackageData {
    digest: String,
    write_set: Vec<String>,
    status: String,
    active: bool,
}

struct ChainStep {
    status: &'static str,
    reason_codes: Vec<&'static str>,
    unauthorized_paths: Vec<String>,
    scaffolds: Vec<String>,
    planning_scaffolds: Vec<String>,
    value: Value,
}

#[allow(clippy::too_many_lines)] // The fail-closed authority cases form one ordered decision table.
fn validate_chain_step(
    repo: &Path,
    parent: &str,
    commit: &str,
    sequence: usize,
    introductions: &BTreeMap<String, Introduction>,
    planning_introductions: &BTreeMap<String, Introduction>,
) -> Result<ChainStep> {
    let changed = changed_paths_between(repo, parent, commit)?;
    let scaffolds = added_package_paths(repo, parent, commit)?;
    let planning = planning_paths(repo, parent, commit, &changed, planning_introductions)?;
    let scaffold_result = scaffold_authorities(repo, commit, sequence, &changed, &scaffolds)?;
    if let Some(failure) = scaffold_result.failure {
        return Ok(invalid_chain_step(
            parent,
            commit,
            sequence,
            &changed,
            failure,
            changed.clone(),
            scaffold_result.authorities,
            scaffolds,
        ));
    }
    if let Some(failure) =
        validate_changed_authority_packages(repo, commit, &changed, &scaffolds, introductions)?
    {
        return Ok(invalid_chain_step(
            parent,
            commit,
            sequence,
            &changed,
            failure,
            changed.clone(),
            scaffold_result.authorities,
            scaffolds,
        ));
    }
    let remaining = changed
        .iter()
        .filter(|path| {
            !scaffold_result.covered_paths.contains(path)
                && !planning.authorized_paths.contains(path)
        })
        .cloned()
        .collect::<Vec<_>>();
    let selected = if remaining.is_empty() {
        AuthoritySelection::None
    } else {
        compose_path_authorities(
            repo,
            parent,
            commit,
            &remaining,
            &changed,
            introductions,
            &planning.barriers,
        )?
    };
    let mut authorities = scaffold_result.authorities;
    match selected {
        AuthoritySelection::None if !remaining.is_empty() => Ok(invalid_chain_step(
            parent,
            commit,
            sequence,
            &changed,
            "NO_PREEXISTING_AUTHORITY",
            remaining,
            authorities,
            scaffolds,
        )),
        AuthoritySelection::Ambiguous(candidates) => {
            authorities.extend(candidates);
            Ok(invalid_chain_step(
                parent,
                commit,
                sequence,
                &changed,
                "AMBIGUOUS_PREEXISTING_AUTHORITY",
                remaining,
                authorities,
                scaffolds,
            ))
        }
        AuthoritySelection::One(authority) => {
            let primary = authority["package_path"].clone();
            authorities.push(authority);
            Ok(ready_chain_step(
                parent,
                commit,
                sequence,
                changed,
                primary,
                authorities,
                scaffolds,
                planning,
            ))
        }
        AuthoritySelection::Composed(selected) => {
            let primary = selected
                .iter()
                .max_by_key(|authority| authority["introduction_sequence"].as_u64())
                .map_or(Value::Null, |authority| authority["package_path"].clone());
            authorities.extend(selected);
            Ok(ready_chain_step(
                parent,
                commit,
                sequence,
                changed,
                primary,
                authorities,
                scaffolds,
                planning,
            ))
        }
        AuthoritySelection::None => {
            if authorities.is_empty() && !planning.authorized_paths.is_empty() {
                return Ok(ready_chain_step(
                    parent,
                    commit,
                    sequence,
                    changed,
                    Value::Null,
                    authorities,
                    scaffolds,
                    planning,
                ));
            }
            if authorities.len() != 1 {
                return Ok(invalid_chain_step(
                    parent,
                    commit,
                    sequence,
                    &changed,
                    "AMBIGUOUS_SCAFFOLD_AUTHORITY",
                    changed.clone(),
                    authorities,
                    scaffolds,
                ));
            }
            let primary = authorities[0]["package_path"].clone();
            if let AuthoritySelection::One(supervisor) = select_newest_authority(
                preexisting_authorities(repo, parent, commit, &changed, &changed, introductions)?,
            ) {
                authorities.push(supervisor);
            }
            Ok(ready_chain_step(
                parent,
                commit,
                sequence,
                changed,
                primary,
                authorities,
                scaffolds,
                planning,
            ))
        }
    }
}

struct ScaffoldResult {
    authorities: Vec<Value>,
    covered_paths: Vec<String>,
    failure: Option<&'static str>,
}

fn scaffold_authorities(
    repo: &Path,
    commit: &str,
    sequence: usize,
    changed: &[String],
    scaffolds: &[String],
) -> Result<ScaffoldResult> {
    let mut authorities = Vec::new();
    let mut covered_paths = Vec::new();
    for package in scaffolds {
        let package_path = Path::new(package);
        validate_package_path(package_path)?;
        let prefix = package_directory(package_path)?;
        if !paths_at(repo, &format!("{commit}^1"), &prefix)?.is_empty() {
            return Ok(ScaffoldResult {
                authorities,
                covered_paths,
                failure: Some("SCAFFOLD_DIRECTORY_PREEXISTING"),
            });
        }
        let data = match read_package_at(repo, commit, package) {
            Ok(Some(data)) => data,
            Ok(None) => {
                return Err(package_error("GATE-PACKAGE-CHAIN-SCAFFOLD-READ", package));
            }
            Err(error)
                if matches!(
                    error.code,
                    "GATE-PACKAGE-WRITE-SET-DUPLICATE"
                        | "GATE-PACKAGE-WRITE-SET-PATH"
                        | "GATE-PACKAGE-WRITE-SET-MISSING"
                        | "GATE-PACKAGE-STATUS-SCHEMA"
                        | "GATE-PACKAGE-UTF8"
                        | "GATE-PACKAGE-CHAIN-PACKAGE-MODE"
                ) =>
            {
                return Ok(ScaffoldResult {
                    authorities,
                    covered_paths,
                    failure: Some("SCAFFOLD_WRITE_SET_SCHEMA_INVALID"),
                });
            }
            Err(error) => return Err(error),
        };
        if !data.active {
            return Ok(ScaffoldResult {
                authorities,
                covered_paths,
                failure: Some("SCAFFOLD_PACKAGE_INACTIVE"),
            });
        }
        let own_paths = changed
            .iter()
            .filter(|path| path.starts_with(&prefix))
            .cloned()
            .collect::<Vec<_>>();
        let added_own_paths = added_paths_between(repo, &format!("{commit}^1"), commit, &prefix)?;
        if own_paths.is_empty()
            || own_paths != added_own_paths
            || own_paths
                .iter()
                .any(|path| !authorized_by(&data.write_set, path))
        {
            return Ok(ScaffoldResult {
                authorities,
                covered_paths,
                failure: Some("SCAFFOLD_SELF_AUTHORITY_INVALID"),
            });
        }
        for path in &own_paths {
            regular_blob_at(repo, commit, Path::new(path))?
                .ok_or_else(|| package_error("GATE-PACKAGE-CHAIN-SCAFFOLD-READ", path.clone()))?;
        }
        covered_paths.extend(own_paths.iter().cloned());
        authorities.push(json!({
            "package_path": package,
            "package_sha256": data.digest,
            "declared_write_set": data.write_set,
            "status": data.status,
            "authorizing": data.active,
            "introduced_commit": commit,
            "introduction_sequence": sequence,
            "role": "SCAFFOLD",
            "authorized_paths": own_paths,
        }));
    }
    Ok(ScaffoldResult {
        authorities,
        covered_paths,
        failure: None,
    })
}

fn validate_changed_authority_packages(
    repo: &Path,
    commit: &str,
    changed: &[String],
    scaffolds: &[String],
    introductions: &BTreeMap<String, Introduction>,
) -> Result<Option<&'static str>> {
    for package in changed.iter().filter(|path| {
        is_package_path(Path::new(path))
            && introductions.contains_key(*path)
            && !scaffolds.contains(*path)
    }) {
        match read_package_at(repo, commit, package) {
            Ok(Some(_)) => {}
            Ok(None) => return Ok(Some("PACKAGE_AUTHORITY_DELETED")),
            Err(error)
                if matches!(
                    error.code,
                    "GATE-PACKAGE-WRITE-SET-DUPLICATE"
                        | "GATE-PACKAGE-WRITE-SET-PATH"
                        | "GATE-PACKAGE-WRITE-SET-MISSING"
                        | "GATE-PACKAGE-STATUS-SCHEMA"
                        | "GATE-PACKAGE-UTF8"
                        | "GATE-PACKAGE-CHAIN-PACKAGE-MODE"
                ) =>
            {
                return Ok(Some("PACKAGE_AUTHORITY_CHILD_INVALID"));
            }
            Err(error) => return Err(error),
        }
    }
    Ok(None)
}

fn package_directory(package: &Path) -> Result<String> {
    let parent = package
        .parent()
        .ok_or_else(|| package_error("GATE-PACKAGE-PATH", package.display().to_string()))?;
    Ok(format!("{}/", parent.to_string_lossy()))
}

fn preexisting_authorities(
    repo: &Path,
    parent: &str,
    commit: &str,
    remaining: &[String],
    transition_paths: &[String],
    introductions: &BTreeMap<String, Introduction>,
) -> Result<Vec<(usize, Value)>> {
    if remaining.is_empty() {
        return Ok(Vec::new());
    }
    let mut candidates = Vec::new();
    let mut newest = None;
    let mut terminal_barrier = None;
    for package in package_paths_at(repo, parent)? {
        let Some(introduction) = introductions.get(&package) else {
            continue;
        };
        let Some(data) = read_package_at(repo, parent, &package)? else {
            continue;
        };
        let write_set_matches = remaining
            .iter()
            .all(|path| authorized_by(&data.write_set, path));
        let own_prefix = package_directory(Path::new(&package))?;
        let terminal_shadow =
            !data.active && remaining.iter().all(|path| path.starts_with(&own_prefix));
        let closure = terminal_shadow
            && closure_prompt_archive(repo, parent, commit, &package, transition_paths)?;
        if data.active {
            if !write_set_matches {
                continue;
            }
        } else if !terminal_shadow && !closure {
            // A terminal package only shadows authority within its own package
            // tree. Historical broad write sets must not suppress an older
            // aggregate authority for unrelated shared paths.
            continue;
        }
        if terminal_shadow && !closure {
            terminal_barrier = Some(
                terminal_barrier.map_or(introduction.sequence, |barrier: usize| {
                    barrier.max(introduction.sequence)
                }),
            );
        }
        if newest.is_none_or(|sequence| introduction.sequence > sequence) {
            newest = Some(introduction.sequence);
            candidates.clear();
        }
        if newest != Some(introduction.sequence) {
            continue;
        }
        if !data.active && !closure {
            continue;
        }
        candidates.push((
                introduction.sequence,
                json!({
                    "package_path": package,
                    "package_sha256": data.digest,
                    "declared_write_set": data.write_set,
                    "status": data.status,
                    "authorizing": data.active || closure,
                    "introduced_commit": introduction.commit,
                    "introduction_sequence": introduction.sequence,
                    "role": if closure { "CLOSURE" } else if introduction.sequence == 0 { "ANCHOR" } else { "PREEXISTING" },
                    "authorized_paths": remaining,
                }),
        ));
    }
    if let Some(barrier) = terminal_barrier {
        candidates.retain(|(sequence, _)| *sequence > barrier);
    }
    Ok(candidates)
}

enum AuthoritySelection {
    None,
    One(Value),
    Composed(Vec<Value>),
    Ambiguous(Vec<Value>),
}

fn compose_path_authorities(
    repo: &Path,
    parent: &str,
    commit: &str,
    paths: &[String],
    transition_paths: &[String],
    introductions: &BTreeMap<String, Introduction>,
    planning_barriers: &BTreeMap<String, usize>,
) -> Result<AuthoritySelection> {
    let mut allocations = BTreeMap::<String, Value>::new();
    for path in paths {
        let mut candidates = preexisting_authorities(
            repo,
            parent,
            commit,
            std::slice::from_ref(path),
            transition_paths,
            introductions,
        )?;
        if let Some(barrier) = planning_barriers.get(path) {
            candidates.retain(|(sequence, _)| sequence > barrier);
        }
        match select_newest_authority(candidates) {
            AuthoritySelection::One(mut authority) => {
                authority["authorized_paths"] = json!([path]);
                let package = authority["package_path"]
                    .as_str()
                    .ok_or_else(|| package_error("GATE-PACKAGE-CHAIN-AUTHORITY", path))?
                    .to_owned();
                if let Some(existing) = allocations.get_mut(&package) {
                    let assigned = existing["authorized_paths"]
                        .as_array_mut()
                        .ok_or_else(|| package_error("GATE-PACKAGE-CHAIN-AUTHORITY", &package))?;
                    assigned.push(Value::String(path.clone()));
                } else {
                    allocations.insert(package, authority);
                }
            }
            AuthoritySelection::Ambiguous(candidates) => {
                return Ok(AuthoritySelection::Ambiguous(candidates));
            }
            AuthoritySelection::None => return Ok(AuthoritySelection::None),
            AuthoritySelection::Composed(_) => {
                return Err(package_error(
                    "GATE-PACKAGE-CHAIN-AUTHORITY",
                    "nested path composition",
                ));
            }
        }
    }
    Ok(AuthoritySelection::Composed(
        allocations.into_values().collect(),
    ))
}

fn closure_prompt_archive(
    repo: &Path,
    parent: &str,
    commit: &str,
    package: &str,
    transition_paths: &[String],
) -> Result<bool> {
    let directory = package_directory(Path::new(package))?;
    let owned = transition_paths
        .iter()
        .filter(|path| path.starts_with(&directory))
        .collect::<Vec<_>>();
    if owned.len() != 2 {
        return Ok(false);
    }
    let package_dir = Path::new(package)
        .parent()
        .ok_or_else(|| package_error("GATE-PACKAGE-PATH", package))?;
    let active_dir = package_dir.join("prompts/active");
    let archived_dir = package_dir.join("prompts/archived");
    let is_markdown = |path: &&&String| {
        Path::new(**path)
            .extension()
            .is_some_and(|extension| extension == "md")
    };
    let Some(active) = owned
        .iter()
        .find(|path| Path::new(**path).parent() == Some(active_dir.as_path()) && is_markdown(path))
    else {
        return Ok(false);
    };
    let Some(archived) = owned.iter().find(|path| {
        Path::new(**path).parent() == Some(archived_dir.as_path()) && is_markdown(path)
    }) else {
        return Ok(false);
    };
    if Path::new(active).file_name() != Path::new(archived).file_name() {
        return Ok(false);
    }
    let active_prefix = format!("{}/", active_dir.to_string_lossy());
    let active_prompts = paths_at(repo, parent, &active_prefix)?
        .into_iter()
        .filter(|path| {
            let path = Path::new(path);
            path.parent() == Some(active_dir.as_path())
                && path.extension().is_some_and(|extension| extension == "md")
        })
        .collect::<Vec<_>>();
    if active_prompts.len() != 1 || active_prompts[0] != **active {
        return Ok(false);
    }
    let parent_active = regular_blob_at(repo, parent, Path::new(active))?;
    let parent_archived = regular_blob_at(repo, parent, Path::new(archived))?;
    let commit_active = regular_blob_at(repo, commit, Path::new(active))?;
    let commit_archived = regular_blob_at(repo, commit, Path::new(archived))?;
    Ok(parent_active.is_some()
        && parent_archived.is_none()
        && commit_active.is_none()
        && parent_active == commit_archived)
}

fn select_newest_authority(candidates: Vec<(usize, Value)>) -> AuthoritySelection {
    let Some(newest) = candidates.iter().map(|(sequence, _)| *sequence).max() else {
        return AuthoritySelection::None;
    };
    let mut newest_candidates = candidates
        .into_iter()
        .filter_map(|(sequence, value)| (sequence == newest).then_some(value))
        .collect::<Vec<_>>();
    if newest_candidates.len() == 1 {
        AuthoritySelection::One(newest_candidates.remove(0))
    } else {
        AuthoritySelection::Ambiguous(newest_candidates)
    }
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_pass_by_value,
    reason = "artifact constructor consumes one complete, named chain-step record"
)]
fn ready_chain_step(
    parent: &str,
    commit: &str,
    sequence: usize,
    changed_paths: Vec<String>,
    primary_package_path: Value,
    authorities: Vec<Value>,
    scaffolds: Vec<String>,
    planning: PlanningPaths,
) -> ChainStep {
    ChainStep {
        status: "READY",
        reason_codes: Vec::new(),
        unauthorized_paths: Vec::new(),
        scaffolds,
        planning_scaffolds: planning.scaffolds.clone(),
        value: json!({
            "sequence": sequence,
            "parent_commit": parent,
            "commit": commit,
            "status": "READY",
            "reason_codes": [],
            "changed_paths": changed_paths,
            "unauthorized_paths": [],
            "primary_package_path": primary_package_path,
            "authorities": authorities,
            "planning_scaffolds": planning.scaffolds,
            "planning_authorized_paths": planning.authorized_paths,
            "planning_authorities": planning.authorities,
        }),
    }
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_pass_by_value,
    reason = "artifact constructor consumes one complete, named chain-step record"
)]
fn invalid_chain_step(
    parent: &str,
    commit: &str,
    sequence: usize,
    changed_paths: &[String],
    reason: &'static str,
    unauthorized_paths: Vec<String>,
    authorities: Vec<Value>,
    scaffolds: Vec<String>,
) -> ChainStep {
    ChainStep {
        status: "INVALID",
        reason_codes: vec![reason],
        unauthorized_paths: unauthorized_paths.clone(),
        scaffolds,
        planning_scaffolds: Vec::new(),
        value: json!({
            "sequence": sequence,
            "parent_commit": parent,
            "commit": commit,
            "status": "INVALID",
            "reason_codes": [reason],
            "changed_paths": changed_paths,
            "unauthorized_paths": unauthorized_paths,
            "primary_package_path": null,
            "authorities": authorities,
            "planning_scaffolds": [],
            "planning_authorized_paths": [],
            "planning_authorities": [],
        }),
    }
}

struct PlanningPaths {
    scaffolds: Vec<String>,
    authorized_paths: Vec<String>,
    authorities: Vec<Value>,
    barriers: BTreeMap<String, usize>,
}

fn planning_paths(
    repo: &Path,
    parent: &str,
    commit: &str,
    changed: &[String],
    introductions: &BTreeMap<String, Introduction>,
) -> Result<PlanningPaths> {
    let added = added_paths_between(repo, parent, commit, "docs/work-packages")?;
    let mut scaffolds = Vec::new();
    let mut authorized_paths = Vec::new();
    let mut authorities = Vec::new();
    let mut barriers = BTreeMap::new();
    for path in changed.iter().filter(|path| is_planning_path(path)) {
        let newly_added = added.contains(path);
        if !newly_added && !introductions.contains_key(path) {
            continue;
        }
        let (authorizing_revision, introduced_commit) = if newly_added {
            (commit, commit)
        } else {
            (parent, introductions[path].commit.as_str())
        };
        let authorizing = planning_data_at(repo, authorizing_revision, path)?;
        planning_data_at(repo, commit, path)?;
        if !planning_status_authorizes(&authorizing.status) {
            if !newly_added {
                barriers.insert(path.clone(), introductions[path].sequence);
                continue;
            }
            return Err(package_error("GATE-PACKAGE-CHAIN-PLANNING-INACTIVE", path));
        }
        if newly_added {
            scaffolds.push(path.clone());
        }
        authorized_paths.push(path.clone());
        authorities.push(json!({
            "path": path,
            "document_sha256": authorizing.digest,
            "status": authorizing.status,
            "authorizing_revision": authorizing_revision,
            "introduced_commit": introduced_commit,
            "role": if newly_added { "PLANNING_SCAFFOLD" } else { "PLANNING_PREEXISTING" },
            "authorized_paths": [path],
        }));
    }
    Ok(PlanningPaths {
        scaffolds,
        authorized_paths,
        authorities,
        barriers,
    })
}

fn planning_status_authorizes(status: &str) -> bool {
    if matches!(status, "ACTIVE" | "QUEUED" | "EXECUTING") {
        return true;
    }
    let Some(progress) = status
        .strip_prefix("EXECUTING (")
        .and_then(|value| value.strip_suffix(" module packages complete)"))
    else {
        return false;
    };
    let Some((complete, total)) = progress.split_once(" of ") else {
        return false;
    };
    complete.parse::<usize>().is_ok_and(|complete| {
        total
            .parse::<usize>()
            .is_ok_and(|total| total > 0 && complete <= total)
    })
}

fn is_planning_path(path: &str) -> bool {
    let path = Path::new(path);
    path.parent() == Some(Path::new("docs/work-packages"))
        && path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.ends_with("-execplan.md"))
}

struct PlanningData {
    digest: String,
    status: String,
}

fn planning_data_at(repo: &Path, revision: &str, path: &str) -> Result<PlanningData> {
    let bytes = regular_blob_at(repo, revision, Path::new(path))?
        .ok_or_else(|| package_error("GATE-PACKAGE-CHAIN-PLANNING-READ", path))?;
    let digest = sha256_bytes(&bytes);
    let text = String::from_utf8(bytes)
        .map_err(|error| package_error("GATE-PACKAGE-UTF8", error.to_string()))?;
    Ok(PlanningData {
        digest,
        status: parse_status(&text)?,
    })
}

fn package_paths_at(repo: &Path, revision: &str) -> Result<Vec<String>> {
    let output = git(
        repo,
        &[
            "ls-tree",
            "-r",
            "--name-only",
            "-z",
            revision,
            "--",
            "docs/work-packages",
        ],
    )?;
    Ok(nul_paths(&output)?
        .into_iter()
        .filter(|path| is_package_path(Path::new(path)))
        .collect())
}

fn paths_at(repo: &Path, revision: &str, prefix: &str) -> Result<Vec<String>> {
    let output = git(
        repo,
        &["ls-tree", "-r", "--name-only", "-z", revision, "--", prefix],
    )?;
    nul_paths(&output)
}

fn added_package_paths(repo: &Path, parent: &str, commit: &str) -> Result<Vec<String>> {
    let output = git(
        repo,
        &[
            "diff",
            "--diff-filter=A",
            "--name-only",
            "-z",
            "--no-renames",
            parent,
            commit,
            "--",
            "docs/work-packages",
        ],
    )?;
    Ok(nul_paths(&output)?
        .into_iter()
        .filter(|path| is_package_path(Path::new(path)))
        .collect())
}

fn added_paths_between(
    repo: &Path,
    parent: &str,
    commit: &str,
    prefix: &str,
) -> Result<Vec<String>> {
    let output = git(
        repo,
        &[
            "diff",
            "--diff-filter=A",
            "--name-only",
            "-z",
            "--no-renames",
            parent,
            commit,
            "--",
            prefix,
        ],
    )?;
    let mut paths = nul_paths(&output)?;
    paths.sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
    paths.dedup();
    Ok(paths)
}

fn first_parent_commits(repo: &Path, base: &str, head: &str) -> Result<Vec<String>> {
    if base == head {
        return Ok(Vec::new());
    }
    let output = git(repo, &["rev-list", "--first-parent", "--reverse", head])?;
    let history = String::from_utf8(output)
        .map_err(|error| package_error("GATE-PACKAGE-GIT", error.to_string()))?;
    let commits = history.lines().collect::<Vec<_>>();
    let Some(base_index) = commits.iter().position(|commit| *commit == base) else {
        return Err(package_error(
            "GATE-PACKAGE-CHAIN-ANCESTRY",
            format!("{base} is not on the first-parent history of {head}"),
        ));
    };
    Ok(commits[base_index + 1..]
        .iter()
        .map(|commit| (*commit).to_owned())
        .collect())
}

fn first_parent(repo: &Path, commit: &str) -> Result<String> {
    let revision = format!("{commit}^1");
    resolve_commit(repo, &revision)
}

fn resolve_commit(repo: &Path, revision: &str) -> Result<String> {
    let commit = format!("{revision}^{{commit}}");
    let output = git(
        repo,
        &["rev-parse", "--verify", "--end-of-options", &commit],
    )?;
    String::from_utf8(output)
        .map(|value| value.trim().to_owned())
        .map_err(|error| package_error("GATE-PACKAGE-GIT", error.to_string()))
}

fn resolve_tree(repo: &Path, revision: &str) -> Result<String> {
    let tree = format!("{revision}^{{tree}}");
    let output = git(repo, &["rev-parse", "--verify", "--end-of-options", &tree])?;
    String::from_utf8(output)
        .map(|value| value.trim().to_owned())
        .map_err(|error| package_error("GATE-PACKAGE-GIT", error.to_string()))
}

fn changed_paths_between(repo: &Path, base: &str, head: &str) -> Result<Vec<String>> {
    let output = git(
        repo,
        &[
            "diff",
            "--name-only",
            "-z",
            "--no-renames",
            base,
            head,
            "--",
        ],
    )?;
    let mut paths = nul_paths(&output)?;
    paths.sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
    paths.dedup();
    Ok(paths)
}

fn authorized_by(write_set: &[String], path: &str) -> bool {
    write_set
        .iter()
        .any(|pattern| wildcard_match(pattern, path))
}

fn read_package_at(repo: &Path, revision: &str, package: &str) -> Result<Option<PackageData>> {
    let path = Path::new(package);
    validate_package_path(path)?;
    let Some(bytes) = regular_blob_at(repo, revision, path)? else {
        return Ok(None);
    };
    let digest = sha256_bytes(&bytes);
    let text = String::from_utf8(bytes)
        .map_err(|error| package_error("GATE-PACKAGE-UTF8", error.to_string()))?;
    let write_set = parse_write_set(&text)?;
    let status = parse_status(&text)?;
    Ok(Some(PackageData {
        digest,
        write_set,
        active: status_authorizes(&status),
        status,
    }))
}

fn status_authorizes(status: &str) -> bool {
    matches!(
        status,
        "ACTIVE"
            | "ACTIVE / QUALIFICATION / ORDER-6"
            | "ACTIVE / READY-QUALIFICATION"
            | "ACTIVE / READY-REPOSITORY-ATTESTATION"
            | "ACTIVE / REVIEW"
            | "ACTIVE / SCAFFOLD"
            | "EXECUTING"
            | "IMPLEMENTED / DUAL REVIEW PASS / TERMINAL VERIFICATION CORRECTION PENDING"
            | "IMPLEMENTED / DUAL REVIEW PASS / VERIFICATION PENDING"
            | "IMPLEMENTED / REVIEW PENDING"
            | "IMPLEMENTED-REVIEW-PENDING"
            | "QUEUED"
            | "QUEUED / ORDER-6"
            | "SCAFFOLDED"
            | "TERMINAL-VERIFICATION-PENDING"
            | "VERIFYING"
    )
}

fn active_prompt_owner(repo: &Path, head: &str, package: &str) -> Result<Value> {
    let package_path = Path::new(package);
    let package_data = read_package_at(repo, head, package)?
        .ok_or_else(|| package_error("GATE-PACKAGE-CHAIN-ANCHOR-MISSING", package.to_owned()))?;
    if !package_data.active {
        return Err(package_error(
            "GATE-PACKAGE-CHAIN-ANCHOR-INACTIVE",
            package.to_owned(),
        ));
    }
    let directory = package_path
        .parent()
        .ok_or_else(|| package_error("GATE-PACKAGE-PATH", package.to_owned()))?
        .join("prompts/active");
    let prefix = format!("{}/", directory.to_string_lossy());
    let prompts = paths_at(repo, head, &prefix)?
        .into_iter()
        .filter(|path| {
            let path = Path::new(path);
            path.parent() == Some(directory.as_path())
                && path.extension().is_some_and(|extension| extension == "md")
        })
        .collect::<Vec<_>>();
    if prompts.len() != 1 {
        return Err(package_error(
            "GATE-PACKAGE-CHAIN-PROMPT-OWNER",
            format!(
                "{package}: expected one active Markdown prompt, found {}",
                prompts.len()
            ),
        ));
    }
    let prompt_path = &prompts[0];
    let bytes = regular_blob_at(repo, head, Path::new(prompt_path))?
        .ok_or_else(|| package_error("GATE-PACKAGE-CHAIN-PROMPT-OWNER", prompt_path.clone()))?;
    Ok(json!({
        "package_path": package,
        "package_sha256": package_data.digest,
        "prompt_path": prompt_path,
        "prompt_sha256": sha256_bytes(&bytes),
    }))
}

fn chain_uses_intent(steps: &[Value], intent_package: &str) -> bool {
    steps.iter().any(|step| {
        step["authorities"]
            .as_array()
            .into_iter()
            .flatten()
            .any(|authority| authority["package_path"] == intent_package)
    })
}

fn regular_blob_at(repo: &Path, revision: &str, path: &Path) -> Result<Option<Vec<u8>>> {
    let path_text = path.to_string_lossy();
    let listing = git(repo, &["ls-tree", "-z", revision, "--", &path_text])?;
    if listing.is_empty() {
        return Ok(None);
    }
    let record = listing
        .strip_suffix(&[0])
        .ok_or_else(|| package_error("GATE-PACKAGE-GIT", "unterminated ls-tree record"))?;
    let separator = record
        .iter()
        .position(|byte| *byte == b'\t')
        .ok_or_else(|| package_error("GATE-PACKAGE-GIT", "invalid ls-tree record"))?;
    let metadata = &record[..separator];
    let listed_path = &record[separator + 1..];
    if listed_path != path_text.as_bytes() {
        return Err(package_error("GATE-PACKAGE-GIT", "ls-tree path mismatch"));
    }
    let metadata = String::from_utf8(metadata.to_vec())
        .map_err(|error| package_error("GATE-PACKAGE-GIT", error.to_string()))?;
    let fields = metadata.split_ascii_whitespace().collect::<Vec<_>>();
    if fields.len() != 3 || !matches!(fields[0], "100644" | "100755") || fields[1] != "blob" {
        return Err(package_error(
            "GATE-PACKAGE-CHAIN-PACKAGE-MODE",
            format!("{revision}:{path_text}: {metadata}"),
        ));
    }
    git_show(repo, revision, path)
}

fn parse_status(text: &str) -> Result<String> {
    let statuses = text
        .lines()
        .filter_map(|line| line.strip_prefix("Status:"))
        .map(|value| value.trim().trim_matches('`').to_owned())
        .collect::<Vec<_>>();
    if statuses.len() != 1 || statuses[0].is_empty() {
        Err(package_error(
            "GATE-PACKAGE-STATUS-SCHEMA",
            "expected exactly one nonempty Status field",
        ))
    } else {
        Ok(statuses[0].clone())
    }
}

type Disposition = (
    &'static str,
    Vec<&'static str>,
    Option<String>,
    Vec<String>,
    Vec<String>,
);

fn disposition(
    base_text: &str,
    current_text: &str,
    changed_paths: &[String],
    base_digest: Option<String>,
) -> Disposition {
    let Ok(base_set) = parse_write_set(base_text) else {
        return (
            "INVALID",
            vec!["BASE_WRITE_SET_SCHEMA_INVALID"],
            base_digest,
            Vec::new(),
            changed_paths.to_vec(),
        );
    };
    let Ok(current_set) = parse_write_set(current_text) else {
        return (
            "INVALID",
            vec!["CURRENT_WRITE_SET_SCHEMA_INVALID"],
            base_digest,
            Vec::new(),
            changed_paths.to_vec(),
        );
    };
    if current_set
        .iter()
        .any(|pattern| !base_set.contains(pattern))
    {
        return (
            "INVALID",
            vec!["RETROACTIVE_WRITE_SET_WIDENING"],
            base_digest,
            current_set,
            changed_paths.to_vec(),
        );
    }
    let unauthorized = changed_paths
        .iter()
        .filter(|path| {
            !base_set.iter().any(|pattern| wildcard_match(pattern, path))
                || !current_set
                    .iter()
                    .any(|pattern| wildcard_match(pattern, path))
        })
        .cloned()
        .collect::<Vec<_>>();
    if unauthorized.is_empty() {
        ("READY", Vec::new(), base_digest, current_set, unauthorized)
    } else {
        (
            "INVALID",
            vec!["UNDECLARED_CHANGED_PATH"],
            base_digest,
            current_set,
            unauthorized,
        )
    }
}

fn parse_write_set(text: &str) -> Result<Vec<String>> {
    let mut inside = false;
    let mut heading_seen = false;
    let mut patterns = Vec::new();
    for line in text.lines() {
        if matches!(line, "## Declared Write Set" | "## Intended Write Set") {
            if heading_seen {
                return Err(package_error(
                    "GATE-PACKAGE-WRITE-SET-DUPLICATE",
                    "duplicate heading",
                ));
            }
            heading_seen = true;
            inside = true;
            continue;
        }
        if inside && line.starts_with("## ") {
            inside = false;
            continue;
        }
        if inside && line.starts_with("- `") {
            let remainder = &line[3..];
            let Some(end) = remainder.find('`') else {
                return Err(package_error(
                    "GATE-PACKAGE-WRITE-SET-PATH",
                    line.to_owned(),
                ));
            };
            let pattern = &remainder[..end];
            let suffix = &remainder[end + 1..];
            if !suffix.is_empty() && !suffix.starts_with(' ') {
                return Err(package_error(
                    "GATE-PACKAGE-WRITE-SET-PATH",
                    line.to_owned(),
                ));
            }
            if pattern.is_empty() || pattern.contains("..") {
                return Err(package_error("GATE-PACKAGE-WRITE-SET-PATH", pattern));
            }
            patterns.push(pattern.to_owned());
        } else if inside && line.starts_with("- ") {
            return Err(package_error(
                "GATE-PACKAGE-WRITE-SET-PATH",
                line.to_owned(),
            ));
        }
    }
    patterns.sort();
    patterns.dedup();
    if patterns.is_empty() {
        Err(package_error(
            "GATE-PACKAGE-WRITE-SET-MISSING",
            "expected exact ## Declared Write Set or ## Intended Write Set heading",
        ))
    } else {
        Ok(patterns)
    }
}

fn changed_paths(repo: &Path, base: &str) -> Result<Vec<String>> {
    let diff = git(repo, &["diff", "--name-only", "-z", base, "--"])?;
    let mut paths = nul_paths(&diff)?;
    let untracked = git(repo, &["ls-files", "--others", "--exclude-standard", "-z"])?;
    paths.extend(nul_paths(&untracked)?);
    paths.sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
    paths.dedup();
    Ok(paths)
}

fn git_show(repo: &Path, base: &str, package: &Path) -> Result<Option<Vec<u8>>> {
    let object = format!("{base}:{}", package.to_string_lossy());
    let output = Command::new("git")
        .args(["show", &object])
        .current_dir(repo)
        .output()
        .map_err(|error| package_error("GATE-PACKAGE-GIT", error.to_string()))?;
    if output.status.success() {
        Ok(Some(output.stdout))
    } else {
        Ok(None)
    }
}

fn git(repo: &Path, args: &[&str]) -> Result<Vec<u8>> {
    let output = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .map_err(|error| package_error("GATE-PACKAGE-GIT", error.to_string()))?;
    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(package_error(
            "GATE-PACKAGE-GIT",
            String::from_utf8_lossy(&output.stderr),
        ))
    }
}

fn nul_paths(bytes: &[u8]) -> Result<Vec<String>> {
    bytes
        .split(|byte| *byte == 0)
        .filter(|item| !item.is_empty())
        .map(|item| {
            String::from_utf8(item.to_vec())
                .map_err(|error| package_error("GATE-PACKAGE-PATH-UTF8", error.to_string()))
        })
        .collect()
}

fn wildcard_match(pattern: &str, value: &str) -> bool {
    let pattern = pattern.as_bytes();
    let value = value.as_bytes();
    let (mut p, mut v, mut star, mut retry) = (0, 0, None, 0);
    while v < value.len() {
        if p < pattern.len() && (pattern[p] == b'?' || pattern[p] == value[v]) {
            p += 1;
            v += 1;
        } else if p < pattern.len() && pattern[p] == b'*' {
            star = Some(p);
            p += 1;
            retry = v;
        } else if let Some(index) = star {
            p = index + 1;
            retry += 1;
            v = retry;
        } else {
            return false;
        }
    }
    while p < pattern.len() && pattern[p] == b'*' {
        p += 1;
    }
    p == pattern.len()
}

fn validate_package_path(path: &Path) -> Result<()> {
    let valid = is_package_path(path);
    if valid {
        Ok(())
    } else {
        Err(package_error(
            "GATE-PACKAGE-PATH",
            path.display().to_string(),
        ))
    }
}

fn is_package_path(path: &Path) -> bool {
    let parts = path.components().collect::<Vec<_>>();
    !path.is_absolute()
        && parts.len() == 4
        && parts[0] == Component::Normal("docs".as_ref())
        && parts[1] == Component::Normal("work-packages".as_ref())
        && matches!(parts[2], Component::Normal(_))
        && parts[3] == Component::Normal("package.md".as_ref())
}

fn package_absolute(repo: &Path, package: &Path) -> std::path::PathBuf {
    repo.join(package)
}

fn read_json(path: &Path) -> Result<Value> {
    let bytes = fs::read(path).map_err(|error| {
        package_error(
            "GATE-PACKAGE-SCHEMA-READ",
            format!("{}: {error}", path.display()),
        )
    })?;
    parse_strict(&bytes)
}

fn package_error(code: &'static str, message: impl Into<String>) -> GatePolicyError {
    GatePolicyError::new(ErrorClass::Planning, code, message)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::sync::atomic::{AtomicU64, Ordering};

    use serde_json::{Value, json};

    use super::{
        parse_write_set, planning_status_authorizes, status_authorizes, validate_package,
        validate_package_chain, wildcard_match,
    };
    use crate::canonical::{derived_id, sha256_bytes};

    #[test]
    fn package_audit_entry_point_preserves_status_reasons_and_identity() {
        let ready = ChainFixture::new();
        ready.write_package("audit", "- `src/**`");
        let base = ready.commit("audit base");
        ready.write_source("src/ready.rs", "pub fn ready() {}\n");
        let package = ChainFixture::package("audit");
        let audit = validate_package(&ready.root, &base, &package).expect("ready package audit");
        assert_eq!(audit["status"], "READY");
        assert_eq!(audit["reason_codes"], json!([]));
        assert_eq!(audit["base_commit"], base);
        assert_eq!(audit["package_path"], "docs/work-packages/audit/package.md");
        assert_eq!(audit["declared_write_set"], json!(["src/**"]));
        assert_eq!(audit["changed_paths"], json!(["src/ready.rs"]));
        assert_eq!(audit["unauthorized_paths"], json!([]));
        assert_eq!(
            audit["current_package_sha256"],
            sha256_bytes(
                &fs::read(ready.root.join(&package)).expect("current package characterization")
            )
        );
        assert_derived_audit_identity(&audit);

        ready.write_source("tests/unowned.rs", "#[test] fn unowned() {}\n");
        let unauthorized =
            validate_package(&ready.root, &base, &package).expect("unauthorized package audit");
        assert_eq!(unauthorized["status"], "INVALID");
        assert_eq!(
            unauthorized["reason_codes"],
            json!(["UNDECLARED_CHANGED_PATH"])
        );
        assert_eq!(
            unauthorized["unauthorized_paths"],
            json!(["tests/unowned.rs"])
        );
        assert_derived_audit_identity(&unauthorized);

        ready.write_raw_package(
            "audit",
            "# Audit\n\nStatus: ACTIVE\n\n## Intended Write Set\n\n- `src/**`\n- `tests/**`\n",
        );
        let widened =
            validate_package(&ready.root, &base, &package).expect("widened package audit");
        assert_eq!(widened["status"], "INVALID");
        assert_eq!(
            widened["reason_codes"],
            json!(["RETROACTIVE_WRITE_SET_WIDENING"])
        );
        assert_eq!(widened["declared_write_set"], json!(["src/**", "tests/**"]));
        assert_derived_audit_identity(&widened);

        ready.write_raw_package("audit", "# Audit\n\nStatus: ACTIVE\n");
        let malformed_current =
            validate_package(&ready.root, &base, &package).expect("malformed current audit");
        assert_eq!(malformed_current["status"], "INVALID");
        assert_eq!(
            malformed_current["reason_codes"],
            json!(["CURRENT_WRITE_SET_SCHEMA_INVALID"])
        );
        assert_eq!(malformed_current["declared_write_set"], json!([]));
        assert_derived_audit_identity(&malformed_current);

        let malformed_base = ChainFixture::new();
        malformed_base.write_raw_package("audit", "# Audit\n\nStatus: ACTIVE\n");
        let malformed_base_commit = malformed_base.commit("malformed audit base");
        malformed_base.write_package("audit", "- `src/**`");
        let malformed = validate_package(
            &malformed_base.root,
            &malformed_base_commit,
            &ChainFixture::package("audit"),
        )
        .expect("malformed base audit");
        assert_eq!(malformed["status"], "INVALID");
        assert_eq!(
            malformed["reason_codes"],
            json!(["BASE_WRITE_SET_SCHEMA_INVALID"])
        );
        assert_derived_audit_identity(&malformed);

        let scaffold = ChainFixture::new();
        let scaffold_base = scaffold.commit("package absent base");
        scaffold.write_raw_package("new", "# New\n\nStatus: ACTIVE\n");
        let blocked = validate_package(
            &scaffold.root,
            &scaffold_base,
            &ChainFixture::package("new"),
        )
        .expect("scaffold audit");
        assert_eq!(blocked["status"], "BLOCKED");
        assert_eq!(blocked["reason_codes"], json!(["SCAFFOLD_COMMIT_REQUIRED"]));
        assert_eq!(blocked["base_package_sha256"], Value::Null);
        assert_eq!(blocked["declared_write_set"], json!([]));
        assert_eq!(blocked["unauthorized_paths"], blocked["changed_paths"]);
        assert_derived_audit_identity(&blocked);

        let invalid_path = validate_package(
            &scaffold.root,
            &scaffold_base,
            Path::new("docs/work-packages/../package.md"),
        )
        .expect_err("invalid package path");
        assert_eq!(invalid_path.code, "GATE-PACKAGE-PATH");
        let missing = validate_package(
            &scaffold.root,
            &scaffold_base,
            &ChainFixture::package("missing"),
        )
        .expect_err("missing current package");
        assert_eq!(missing.code, "GATE-PACKAGE-READ");

        let invalid_current = ChainFixture::new();
        invalid_current.write_package("audit", "- `src/**`");
        let invalid_current_base = invalid_current.commit("valid UTF-8 base");
        fs::write(
            invalid_current.root.join(ChainFixture::package("audit")),
            [0xff],
        )
        .expect("invalid current UTF-8");
        let current_utf8 = validate_package(
            &invalid_current.root,
            &invalid_current_base,
            &ChainFixture::package("audit"),
        )
        .expect_err("invalid current package UTF-8");
        assert_eq!(current_utf8.code, "GATE-PACKAGE-UTF8");

        let invalid_base = ChainFixture::new();
        let invalid_base_path = invalid_base.root.join(ChainFixture::package("audit"));
        fs::create_dir_all(invalid_base_path.parent().expect("invalid base parent"))
            .expect("invalid base directory");
        fs::write(&invalid_base_path, [0xff]).expect("invalid base UTF-8");
        let invalid_base_commit = invalid_base.commit("invalid UTF-8 base");
        invalid_base.write_package("audit", "- `src/**`");
        let base_utf8 = validate_package(
            &invalid_base.root,
            &invalid_base_commit,
            &ChainFixture::package("audit"),
        )
        .expect_err("invalid base package UTF-8");
        assert_eq!(base_utf8.code, "GATE-PACKAGE-UTF8");

        let invalid_git =
            validate_package(&ready.root, "not-a-commit", &ChainFixture::package("audit"))
                .expect_err("invalid Git base");
        assert_eq!(invalid_git.code, "GATE-PACKAGE-GIT");

        let missing_schema = ChainFixture::new();
        missing_schema.write_package("audit", "- `src/**`");
        let missing_schema_base = missing_schema.commit("schema fixture base");
        missing_schema.write_source("src/ready.rs", "pub fn ready() {}\n");
        fs::remove_file(
            missing_schema
                .root
                .join("gate-policy/v1/schemas/package-audit.schema.json"),
        )
        .expect("remove package audit schema");
        let schema_error = validate_package(
            &missing_schema.root,
            &missing_schema_base,
            &ChainFixture::package("audit"),
        )
        .expect_err("missing package audit schema");
        assert_eq!(schema_error.code, "GATE-PACKAGE-SCHEMA-READ");
    }

    fn assert_derived_audit_identity(audit: &Value) {
        let mut subject = audit.clone();
        subject["package_audit_id"] = json!("0".repeat(64));
        assert_eq!(
            audit["package_audit_id"],
            derived_id(&subject, "package_audit_id").expect("derived package audit identity")
        );
    }

    #[test]
    fn write_set_wildcard_matches_nested_paths() {
        assert!(wildcard_match(
            "crates/openwepp-gate-planner/**",
            "crates/openwepp-gate-planner/src/lib.rs"
        ));
        assert!(!wildcard_match(
            "tools/local_ci/**",
            "tools/release/tool.sh"
        ));
    }

    #[test]
    fn sequential_chain_admits_single_and_newer_prerequisite_authorities() {
        let fixture = ChainFixture::new();
        fixture.write_package("root", "- `docs/work-packages/**`\n- `src/**`");
        let base = fixture.commit("base authority");
        fixture.write_source("src/single.rs", "pub fn single() {}\n");
        let single_head = fixture.commit("single correction");
        let single = validate_package_chain(
            &fixture.root,
            &base,
            Some(&single_head),
            &ChainFixture::package("root"),
        )
        .expect("single authority chain");
        assert_eq!(single["status"], "READY", "{single}");
        assert_eq!(
            single["steps"][0]["authorities"][0]["package_path"],
            "docs/work-packages/root/package.md"
        );

        fixture.write_package("child", "- `docs/work-packages/child/**`\n- `src/child.rs`");
        let scaffold = fixture.commit("child scaffold");
        fixture.write_source("src/child.rs", "pub fn child() {}\n");
        let head = fixture.commit("child correction");
        let chain = validate_package_chain(
            &fixture.root,
            &single_head,
            Some(&head),
            &ChainFixture::package("root"),
        )
        .expect("sequential authority chain");
        assert_eq!(chain["status"], "READY", "{chain}");
        assert_eq!(chain["steps"][0]["commit"], scaffold);
        assert_eq!(chain["steps"][0]["authorities"][0]["role"], "SCAFFOLD");
        assert_eq!(
            chain["steps"][1]["authorities"][0]["package_path"],
            "docs/work-packages/child/package.md"
        );
    }

    #[test]
    fn sequential_chain_rejects_zero_ambiguous_and_retroactive_authority() {
        let zero = ChainFixture::new();
        zero.write_package("anchor", "- `docs/work-packages/anchor/**`");
        let zero_base = zero.commit("empty base");
        zero.write_source("src/unowned.rs", "pub fn unowned() {}\n");
        let zero_head = zero.commit("unowned correction");
        zero.assert_invalid(&zero_base, &zero_head, "NO_PREEXISTING_AUTHORITY");

        let ambiguous = ChainFixture::new();
        ambiguous.write_package("anchor", "- `docs/work-packages/**`");
        let ambiguous_base = ambiguous.commit("ambiguous base");
        ambiguous.write_package("one", "- `docs/work-packages/one/**`\n- `src/**`");
        ambiguous.write_package("two", "- `docs/work-packages/two/**`\n- `src/**`");
        ambiguous.commit("same-sequence scaffolds");
        ambiguous.write_source("src/value.rs", "pub fn value() {}\n");
        let ambiguous_head = ambiguous.commit("ambiguous correction");
        ambiguous.assert_invalid(
            &ambiguous_base,
            &ambiguous_head,
            "AMBIGUOUS_SCAFFOLD_AUTHORITY",
        );

        let retroactive = ChainFixture::new();
        retroactive.write_package("anchor", "- `docs/work-packages/anchor/**`");
        let retroactive_base = retroactive.commit("empty base");
        retroactive.write_package("late", "- `docs/work-packages/late/**`\n- `src/late.rs`");
        retroactive.write_source("src/late.rs", "pub fn late() {}\n");
        let retroactive_head = retroactive.commit("late authority and correction");
        retroactive.assert_invalid(
            &retroactive_base,
            &retroactive_head,
            "NO_PREEXISTING_AUTHORITY",
        );
    }

    #[test]
    fn sequential_chain_rejects_malformed_scaffolds_and_unmet_prerequisites() {
        let malformed = ChainFixture::new();
        malformed.write_package("anchor", "- `docs/work-packages/anchor/**`");
        let malformed_base = malformed.commit("empty base");
        malformed.write_raw_package("broken", "# Broken\n");
        let malformed_head = malformed.commit("malformed scaffold");
        malformed.assert_invalid(
            &malformed_base,
            &malformed_head,
            "SCAFFOLD_WRITE_SET_SCHEMA_INVALID",
        );

        let unmet = ChainFixture::new();
        unmet.write_package("anchor", "- `docs/work-packages/anchor/**`");
        let unmet_base = unmet.commit("empty base");
        unmet.write_package("child", "- `docs/work-packages/child/**`");
        unmet.write_source("docs/work-packages/README.md", "# Catalog\n");
        let unmet_head = unmet.commit("scaffold with unowned prerequisite");
        unmet.assert_invalid(&unmet_base, &unmet_head, "NO_PREEXISTING_AUTHORITY");
    }

    #[test]
    fn sequential_chain_uses_parent_version_for_prospective_amendments() {
        let fixture = ChainFixture::new();
        fixture.write_package("owner", "- `docs/work-packages/owner/**`");
        let base = fixture.commit("narrow base authority");
        fixture.write_package(
            "owner",
            "- `docs/work-packages/owner/**`\n- `src/amended.rs`",
        );
        fixture.commit("prospective authority amendment");
        fixture.write_source("src/amended.rs", "pub fn amended() {}\n");
        let head = fixture.commit("amended correction");
        let chain = validate_package_chain(
            &fixture.root,
            &base,
            Some(&head),
            &ChainFixture::package("owner"),
        )
        .expect("prospective amendment chain");
        assert_eq!(chain["status"], "READY", "{chain}");
        assert_eq!(chain["steps"].as_array().map(Vec::len), Some(2));
    }

    #[test]
    fn package_grammar_is_exact_and_status_is_bound() {
        assert_eq!(
            parse_write_set(
                "## Declared Write Set\n\n- `src/**` — implementation\n- `/tmp/external/**` (evidence)\n"
            )
            .expect("annotated write set"),
            ["/tmp/external/**", "src/**"]
        );
        for malformed in [
            "## Declared Write Set\n- `src/**`\n## Intended Write Set\n- `tests/**`\n",
            "## Intended Write Set\n- src/**\n",
        ] {
            assert!(parse_write_set(malformed).is_err());
        }
        assert!(status_authorizes("ACTIVE / READY-REPOSITORY-ATTESTATION"));
        assert!(status_authorizes("ACTIVE / QUALIFICATION / ORDER-6"));
        assert!(status_authorizes("QUEUED / ORDER-6"));
        assert!(status_authorizes("IMPLEMENTED / REVIEW PENDING"));
        assert!(!status_authorizes("COMPLETE"));
        assert!(!status_authorizes(
            "COMPLETE / READY-REPOSITORY-ATTESTATION"
        ));
        assert!(!status_authorizes("ACTIVE / BLOCKED"));
        assert!(!status_authorizes("active / blocked"));
        assert!(!status_authorizes("COMPLET"));
        assert!(!status_authorizes("PASS"));
        assert!(planning_status_authorizes(
            "EXECUTING (3 of 7 module packages complete)"
        ));
        assert!(!planning_status_authorizes("EXECUTING soon"));
    }

    #[test]
    fn explicit_anchor_excludes_stale_broad_packages() {
        let fixture = ChainFixture::new();
        fixture.write_package("stale", "- `src/**`");
        fixture.write_package("anchor", "- `docs/work-packages/anchor/**`");
        let base = fixture.commit("explicit narrow anchor");
        fixture.write_source("src/unowned.rs", "pub fn unowned() {}\n");
        let head = fixture.commit("stale broad package cannot authorize");
        fixture.assert_invalid(&base, &head, "NO_PREEXISTING_AUTHORITY");
    }

    #[test]
    fn sequential_chain_composes_distinct_path_owners() {
        let fixture = ChainFixture::new();
        fixture.write_package("anchor", "- `docs/work-packages/**`");
        let base = fixture.commit("anchor");
        fixture.write_package("one", "- `docs/work-packages/one/**`\n- `src/one.rs`");
        fixture.commit("first owner");
        fixture.write_package("two", "- `docs/work-packages/two/**`\n- `src/two.rs`");
        fixture.commit("second owner");
        fixture.write_source("src/one.rs", "pub fn one() {}\n");
        fixture.write_source("src/two.rs", "pub fn two() {}\n");
        let head = fixture.commit("composed correction");
        let chain = validate_package_chain(
            &fixture.root,
            &base,
            Some(&head),
            &ChainFixture::package("anchor"),
        )
        .expect("composed chain");
        assert_eq!(chain["status"], "READY");
        assert_eq!(
            chain["steps"][2]["authorities"].as_array().map(Vec::len),
            Some(2)
        );
    }

    #[test]
    fn malformed_child_amendment_fails_closed() {
        let fixture = ChainFixture::new();
        fixture.write_package("anchor", "- `docs/work-packages/**`");
        let base = fixture.commit("anchor");
        fixture.write_package("child", "- `docs/work-packages/child/**`");
        fixture.commit("child scaffold");
        fixture.write_raw_package("child", "# Child\n\nStatus: ACTIVE\n");
        let head = fixture.commit("malformed child amendment");
        fixture.assert_invalid(&base, &head, "PACKAGE_AUTHORITY_CHILD_INVALID");
    }

    #[test]
    fn inactive_scaffold_and_deleted_authority_fail_closed() {
        let inactive = ChainFixture::new();
        inactive.write_package("anchor", "- `docs/work-packages/**`");
        let inactive_base = inactive.commit("anchor");
        inactive.write_raw_package(
            "child",
            "# Child\n\nStatus: COMPLETE\n\n## Intended Write Set\n\n- `docs/work-packages/child/**`\n",
        );
        let inactive_head = inactive.commit("inactive scaffold");
        inactive.assert_invalid(&inactive_base, &inactive_head, "SCAFFOLD_PACKAGE_INACTIVE");

        let deleted = ChainFixture::new();
        deleted.write_package("anchor", "- `docs/work-packages/**`");
        let deleted_base = deleted.commit("anchor");
        deleted.write_package("child", "- `docs/work-packages/child/**`");
        deleted.commit("child scaffold");
        fs::remove_file(deleted.root.join("docs/work-packages/child/package.md"))
            .expect("delete child package");
        let deleted_head = deleted.commit("delete child authority");
        deleted.assert_invalid(&deleted_base, &deleted_head, "PACKAGE_AUTHORITY_DELETED");
    }

    #[test]
    fn terminal_package_only_authorizes_content_preserving_prompt_archive() {
        let fixture = ChainFixture::new();
        fixture.write_package("anchor", "- `docs/work-packages/anchor/**`");
        let base = fixture.commit("anchor");
        fixture.write_package("child", "- `docs/work-packages/child/**`");
        fixture.commit("child scaffold");
        fixture.write_raw_package(
            "child",
            "# Child\n\nStatus: COMPLETE\n\n## Intended Write Set\n\n- `docs/work-packages/child/**`\n",
        );
        fixture.write_source(
            "docs/work-packages/anchor/artifacts/participation.md",
            "# Participation\n",
        );
        fixture.commit("complete child");
        let active = fixture
            .root
            .join("docs/work-packages/child/prompts/active/kickoff.md");
        let archived = fixture
            .root
            .join("docs/work-packages/child/prompts/archived/kickoff.md");
        fs::create_dir_all(archived.parent().expect("archive parent")).expect("archive directory");
        fs::rename(active, archived).expect("archive prompt");
        let head = fixture.commit("archive completed prompt");
        let chain = validate_package_chain(
            &fixture.root,
            &base,
            Some(&head),
            &ChainFixture::package("anchor"),
        )
        .expect("closure archive chain");
        assert_eq!(chain["status"], "READY");
        assert_eq!(chain["steps"][2]["authorities"][0]["role"], "CLOSURE");

        let extra = ChainFixture::new();
        extra.write_package("anchor", "- `docs/work-packages/anchor/**`");
        let extra_base = extra.commit("anchor");
        extra.write_package("child", "- `docs/work-packages/child/**`");
        extra.commit("child scaffold");
        extra.write_raw_package(
            "child",
            "# Child\n\nStatus: COMPLETE\n\n## Intended Write Set\n\n- `docs/work-packages/child/**`\n",
        );
        extra.write_source(
            "docs/work-packages/child/prompts/active/extra.md",
            "# Extra\n",
        );
        extra.write_source(
            "docs/work-packages/anchor/artifacts/participation.md",
            "# Participation\n",
        );
        extra.commit("complete child with two prompts");
        let extra_active = extra
            .root
            .join("docs/work-packages/child/prompts/active/kickoff.md");
        let extra_archived = extra
            .root
            .join("docs/work-packages/child/prompts/archived/kickoff.md");
        fs::create_dir_all(extra_archived.parent().expect("archive parent"))
            .expect("archive directory");
        fs::rename(extra_active, extra_archived).expect("archive one of two prompts");
        let extra_head = extra.commit("incomplete prompt archive");
        extra.assert_invalid(&extra_base, &extra_head, "NO_PREEXISTING_AUTHORITY");

        let nested = ChainFixture::new();
        nested.write_package("anchor", "- `docs/work-packages/anchor/**`");
        let nested_base = nested.commit("anchor");
        nested.write_package("child", "- `docs/work-packages/child/**`");
        nested.commit("child scaffold");
        nested.write_raw_package(
            "child",
            "# Child\n\nStatus: COMPLETE\n\n## Intended Write Set\n\n- `docs/work-packages/child/**`\n",
        );
        nested.write_source(
            "docs/work-packages/child/artifacts/prompts/active/nested.md",
            "# Nested\n",
        );
        nested.write_source(
            "docs/work-packages/anchor/artifacts/participation.md",
            "# Participation\n",
        );
        nested.commit("complete child with nested prompt-like path");
        let nested_active = nested
            .root
            .join("docs/work-packages/child/artifacts/prompts/active/nested.md");
        let nested_archived = nested
            .root
            .join("docs/work-packages/child/artifacts/prompts/archived/nested.md");
        fs::create_dir_all(nested_archived.parent().expect("nested archive parent"))
            .expect("nested archive directory");
        fs::rename(nested_active, nested_archived).expect("move nested prompt-like path");
        let nested_head = nested.commit("nested prompt-like archive");
        nested.assert_invalid(&nested_base, &nested_head, "NO_PREEXISTING_AUTHORITY");

        let captured = ChainFixture::new();
        captured.write_package("anchor", "- `docs/work-packages/**`");
        let captured_base = captured.commit("broad anchor");
        captured.write_package("child", "- `docs/work-packages/child/**`");
        captured.commit("child scaffold");
        captured.write_raw_package(
            "child",
            "# Child\n\nStatus: COMPLETE\n\n## Intended Write Set\n\n- `docs/work-packages/child/package.md`\n",
        );
        captured.commit("complete child");
        captured.write_source(
            "docs/work-packages/child/artifacts/late.md",
            "# Late edit\n",
        );
        let captured_head = captured.commit("older broad owner cannot capture child");
        captured.assert_invalid(&captured_base, &captured_head, "NO_PREEXISTING_AUTHORITY");

        let shared = ChainFixture::new();
        shared.write_package("anchor", "- `docs/work-packages/**`");
        let shared_base = shared.commit("broad anchor");
        shared.write_package(
            "child",
            "- `docs/work-packages/child/**`\n- `docs/work-packages/README.md`",
        );
        shared.commit("child scaffold with shared path");
        shared.write_raw_package(
            "child",
            "# Child\n\nStatus: COMPLETE\n\n## Intended Write Set\n\n- `docs/work-packages/child/**`\n- `docs/work-packages/README.md`\n",
        );
        shared.commit("complete child with historical shared path");
        shared.write_source("docs/work-packages/README.md", "# Shared index\n");
        let shared_head = shared.commit("anchor retains unrelated shared path");
        let shared_chain = validate_package_chain(
            &shared.root,
            &shared_base,
            Some(&shared_head),
            &ChainFixture::package("anchor"),
        )
        .expect("inactive child must not shadow an unrelated shared path");
        assert_eq!(shared_chain["status"], "READY");

        let peer = ChainFixture::new();
        peer.write_package("anchor", "- `docs/work-packages/**`");
        let peer_base = peer.commit("broad anchor");
        peer.write_package("child", "- `docs/work-packages/child/**`");
        peer.write_package("sibling", "- `docs/work-packages/sibling/**`");
        peer.write_source("README.md", "# Scaffold supervisor participation\n");
        peer.commit("same-sequence child and sibling scaffolds");
        peer.write_raw_package(
            "child",
            "# Child\n\nStatus: COMPLETE\n\n## Intended Write Set\n\n- `docs/work-packages/child/package.md`\n",
        );
        peer.write_package(
            "sibling",
            "- `docs/work-packages/sibling/**`\n- `docs/work-packages/child/**`",
        );
        peer.commit("close child and prospectively widen sibling");
        peer.write_source(
            "docs/work-packages/child/artifacts/late.md",
            "# Same-sequence capture attempt\n",
        );
        let peer_head = peer.commit("same-sequence peer cannot capture child");
        peer.assert_invalid(&peer_base, &peer_head, "NO_PREEXISTING_AUTHORITY");

        let superseded = ChainFixture::new();
        superseded.write_package("anchor", "- `docs/work-packages/**`");
        let superseded_base = superseded.commit("broad anchor");
        superseded.write_package("child", "- `docs/work-packages/child/**`");
        superseded.commit("child scaffold");
        superseded.write_raw_package(
            "child",
            "# Child\n\nStatus: COMPLETE\n\n## Intended Write Set\n\n- `docs/work-packages/child/**`\n",
        );
        superseded.commit("complete child");
        superseded.write_package(
            "successor",
            "- `docs/work-packages/successor/**`\n- `docs/work-packages/child/**`",
        );
        superseded.commit("newer successor authority");
        superseded.write_source(
            "docs/work-packages/child/artifacts/later.md",
            "# Later authorized edit\n",
        );
        let superseded_head = superseded.commit("newer owner supersedes terminal child");
        let superseded_chain = validate_package_chain(
            &superseded.root,
            &superseded_base,
            Some(&superseded_head),
            &ChainFixture::package("anchor"),
        )
        .expect("newer successor chain");
        assert_eq!(superseded_chain["status"], "READY", "{superseded_chain}");
        assert_eq!(
            superseded_chain["steps"][3]["authorities"][0]["package_path"],
            "docs/work-packages/successor/package.md"
        );
    }

    #[cfg(unix)]
    #[test]
    fn symlinked_authority_child_fails_closed() {
        use std::os::unix::fs::symlink;

        let fixture = ChainFixture::new();
        fixture.write_package("anchor", "- `docs/work-packages/**`");
        let base = fixture.commit("anchor");
        fixture.write_package("child", "- `docs/work-packages/child/**`");
        fixture.commit("child scaffold");
        let package = fixture.root.join("docs/work-packages/child/package.md");
        fs::remove_file(&package).expect("remove regular package");
        symlink("../../../README.md", package).expect("symlink child package");
        let head = fixture.commit("symlink child authority");
        fixture.assert_invalid(&base, &head, "PACKAGE_AUTHORITY_CHILD_INVALID");
    }

    #[test]
    fn merge_is_atomic_against_its_first_parent() {
        let fixture = ChainFixture::new();
        fixture.write_package("anchor", "- `docs/work-packages/**`");
        let base = fixture.commit("anchor");
        fixture.write_package("child", "- `docs/work-packages/child/**`\n- `src/merge.rs`");
        fixture.write_source("src/merge.rs", "pub fn merged() {}\n");
        ChainFixture::git(&fixture.root, &["add", "."]);
        let tree = String::from_utf8(ChainFixture::git_output(&fixture.root, &["write-tree"]))
            .expect("UTF-8 tree")
            .trim()
            .to_owned();
        let base_tree = format!("{base}^{{tree}}");
        let side = String::from_utf8(ChainFixture::git_output(
            &fixture.root,
            &["commit-tree", &base_tree, "-p", &base, "-m", "side"],
        ))
        .expect("UTF-8 side commit")
        .trim()
        .to_owned();
        let merge = String::from_utf8(ChainFixture::git_output(
            &fixture.root,
            &[
                "commit-tree",
                &tree,
                "-p",
                &base,
                "-p",
                &side,
                "-m",
                "merge",
            ],
        ))
        .expect("UTF-8 merge commit")
        .trim()
        .to_owned();
        ChainFixture::git(&fixture.root, &["update-ref", "HEAD", &merge]);
        fixture.assert_invalid(&base, &merge, "NO_PREEXISTING_AUTHORITY");
    }

    #[test]
    fn top_level_execplan_is_narrow_validated_planning_state() {
        let fixture = ChainFixture::new();
        fixture.write_package("anchor", "- `docs/work-packages/**`");
        let base = fixture.commit("anchor");
        let plan = fixture.root.join("docs/work-packages/fixture-execplan.md");
        fs::write(&plan, "# Plan\n\nStatus: `QUEUED`\n").expect("planning scaffold");
        let scaffold = fixture.commit("planning scaffold");
        fixture.write_source(
            "docs/work-packages/anchor/artifacts/participation.md",
            "# Participation\n",
        );
        let participating = fixture.commit("anchor participation");
        let chain = validate_package_chain(
            &fixture.root,
            &base,
            Some(&participating),
            &ChainFixture::package("anchor"),
        )
        .expect("validated planning scaffold");
        assert_eq!(chain["status"], "READY");
        assert_eq!(chain["steps"][0]["authorities"], json!([]));
        assert_eq!(
            chain["steps"][0]["planning_authorized_paths"],
            json!(["docs/work-packages/fixture-execplan.md"])
        );
        assert_eq!(
            chain["steps"][0]["planning_authorities"][0]["status"],
            "QUEUED"
        );
        assert_eq!(chain["steps"][0]["commit"], scaffold);

        fs::write(&plan, "# Plan\n\nStatus: `COMPLETE`\n").expect("close plan");
        let closed = fixture.commit("close planning state");
        validate_package_chain(
            &fixture.root,
            &base,
            Some(&closed),
            &ChainFixture::package("anchor"),
        )
        .expect("prospectively close planning state");
        fs::write(&plan, "# Plan\n\nStatus: `COMPLETE`\n\nchanged\n").expect("change closed plan");
        let changed_closed = fixture.commit("change closed planning state");
        fixture.assert_invalid(&base, &changed_closed, "NO_PREEXISTING_AUTHORITY");

        let sibling = ChainFixture::new();
        sibling.write_package("anchor", "- `docs/work-packages/anchor/**`");
        let sibling_base = sibling.commit("anchor");
        fs::write(
            sibling.root.join("docs/work-packages/sibling-execplan.md"),
            "# Plan\n\nStatus: QUEUED\n",
        )
        .expect("sibling planning scaffold");
        sibling.write_source("src/unowned.rs", "pub fn unowned() {}\n");
        let sibling_head = sibling.commit("planning cannot cover sibling");
        sibling.assert_invalid(&sibling_base, &sibling_head, "NO_PREEXISTING_AUTHORITY");

        let malformed = ChainFixture::new();
        malformed.write_package("anchor", "- `docs/work-packages/anchor/**`");
        let malformed_base = malformed.commit("anchor");
        fs::write(
            malformed.root.join("docs/work-packages/bad-execplan.md"),
            "# Plan\n\nStatus: PASS\n",
        )
        .expect("malformed planning scaffold");
        let malformed_head = malformed.commit("malformed planning state");
        assert_eq!(
            validate_package_chain(
                &malformed.root,
                &malformed_base,
                Some(&malformed_head),
                &ChainFixture::package("anchor"),
            )
            .expect_err("unknown planning state")
            .code,
            "GATE-PACKAGE-CHAIN-PLANNING-INACTIVE"
        );

        #[cfg(unix)]
        {
            use std::os::unix::fs::symlink;

            let linked = ChainFixture::new();
            linked.write_package("anchor", "- `docs/work-packages/anchor/**`");
            let linked_base = linked.commit("anchor");
            symlink(
                "../../README.md",
                linked.root.join("docs/work-packages/link-execplan.md"),
            )
            .expect("symlink planning scaffold");
            let linked_head = linked.commit("symlink planning state");
            assert_eq!(
                validate_package_chain(
                    &linked.root,
                    &linked_base,
                    Some(&linked_head),
                    &ChainFixture::package("anchor"),
                )
                .expect_err("nonregular planning state")
                .code,
                "GATE-PACKAGE-CHAIN-PACKAGE-MODE"
            );
        }
    }

    static CHAIN_SEQUENCE: AtomicU64 = AtomicU64::new(0);

    struct ChainFixture {
        root: PathBuf,
    }

    impl ChainFixture {
        fn new() -> Self {
            let sequence = CHAIN_SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let root = std::env::temp_dir().join(format!(
                "openwepp-package-chain-{}-{sequence}",
                std::process::id()
            ));
            fs::create_dir_all(root.join("gate-policy/v1/schemas")).expect("schema directory");
            fs::write(
                root.join("gate-policy/v1/schemas/package-authority-chain.schema.json"),
                "{\"type\":\"object\"}\n",
            )
            .expect("permissive fixture schema");
            fs::write(
                root.join("gate-policy/v1/schemas/package-audit.schema.json"),
                "{\"type\":\"object\"}\n",
            )
            .expect("permissive package audit schema");
            Self::git(&root, &["init", "-q"]);
            Self::git(&root, &["config", "user.email", "test@example.invalid"]);
            Self::git(&root, &["config", "user.name", "Test"]);
            fs::write(root.join("README.md"), "# Fixture\n").expect("fixture root");
            Self { root }
        }

        fn write_package(&self, name: &str, write_set: &str) {
            self.write_raw_package(
                name,
                &format!("# {name}\n\nStatus: `ACTIVE`\n\n## Intended Write Set\n\n{write_set}\n"),
            );
            let active = self
                .root
                .join(format!("docs/work-packages/{name}/prompts/active"));
            fs::create_dir_all(&active).expect("active prompt directory");
            fs::write(active.join("kickoff.md"), "# Kickoff\n").expect("active prompt");
        }

        fn package(name: &str) -> PathBuf {
            PathBuf::from(format!("docs/work-packages/{name}/package.md"))
        }

        fn write_raw_package(&self, name: &str, text: &str) {
            let directory = self.root.join(format!("docs/work-packages/{name}"));
            fs::create_dir_all(&directory).expect("package directory");
            fs::write(directory.join("package.md"), text).expect("package text");
        }

        fn write_source(&self, path: &str, text: &str) {
            let path = self.root.join(path);
            fs::create_dir_all(path.parent().expect("source parent")).expect("source directory");
            fs::write(path, text).expect("source text");
        }

        fn commit(&self, message: &str) -> String {
            Self::git(&self.root, &["add", "."]);
            Self::git(&self.root, &["commit", "-qm", message]);
            String::from_utf8(Self::git_output(&self.root, &["rev-parse", "HEAD"]))
                .expect("UTF-8 commit")
                .trim()
                .to_owned()
        }

        fn assert_invalid(&self, base: &str, head: &str, reason: &str) {
            let chain =
                validate_package_chain(&self.root, base, Some(head), &Self::package("anchor"))
                    .expect("represented invalid chain");
            assert_eq!(chain["status"], "INVALID");
            assert!(
                chain["reason_codes"]
                    .as_array()
                    .is_some_and(|items| items.iter().any(|item| item == reason)),
                "missing {reason}: {chain}"
            );
        }

        fn git(root: &Path, arguments: &[&str]) {
            let output = Command::new("git")
                .args(arguments)
                .current_dir(root)
                .output()
                .expect("run git");
            assert!(
                output.status.success(),
                "git {arguments:?}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        fn git_output(root: &Path, arguments: &[&str]) -> Vec<u8> {
            let output = Command::new("git")
                .args(arguments)
                .current_dir(root)
                .output()
                .expect("run git");
            assert!(output.status.success(), "git {arguments:?}");
            output.stdout
        }
    }

    impl Drop for ChainFixture {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.root).expect("remove chain fixture");
        }
    }
}
