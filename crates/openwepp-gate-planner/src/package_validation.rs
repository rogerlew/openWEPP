//! Prospective work-package validation without retroactive execution authority.

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

    let (status, reasons, base_digest, declared, unauthorized) = match base_text {
        None => (
            "BLOCKED",
            vec!["SCAFFOLD_COMMIT_REQUIRED"],
            None,
            parse_write_set(&current_text).unwrap_or_default(),
            changed_paths.clone(),
        ),
        Some(base_bytes) => {
            let base_digest = Some(sha256_bytes(&base_bytes));
            let base_text = String::from_utf8(base_bytes)
                .map_err(|error| package_error("GATE-PACKAGE-UTF8", error.to_string()))?;
            disposition(&base_text, &current_text, &changed_paths, base_digest)
        }
    };
    let mut audit = json!({
        "schema_version": "openwepp-package-audit-v1",
        "package_audit_id": "0".repeat(64),
        "status": status,
        "reason_codes": reasons,
        "base_commit": base,
        "package_path": package.to_string_lossy(),
        "base_package_sha256": base_digest,
        "current_package_sha256": current_digest,
        "declared_write_set": declared,
        "changed_paths": changed_paths,
        "unauthorized_paths": unauthorized,
    });
    audit["package_audit_id"] = Value::String(derived_id(&audit, "package_audit_id")?);
    let schema = read_json(&repo.join("gate-policy/v1/schemas/package-audit.schema.json"))?;
    validate_schema(&schema, &audit, "package audit")?;
    Ok(audit)
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
    let mut patterns = Vec::new();
    for line in text.lines() {
        if line == "## Declared Write Set" {
            if inside {
                return Err(package_error(
                    "GATE-PACKAGE-WRITE-SET-DUPLICATE",
                    "duplicate heading",
                ));
            }
            inside = true;
            continue;
        }
        if inside && line.starts_with("## ") {
            break;
        }
        if inside && line.starts_with("- `") && line.ends_with('`') {
            let pattern = line.trim_start_matches("- `").trim_end_matches('`');
            if pattern.is_empty() || pattern.starts_with('/') || pattern.contains("..") {
                return Err(package_error("GATE-PACKAGE-WRITE-SET-PATH", pattern));
            }
            patterns.push(pattern.to_owned());
        }
    }
    patterns.sort();
    patterns.dedup();
    if patterns.is_empty() {
        Err(package_error(
            "GATE-PACKAGE-WRITE-SET-MISSING",
            "expected exact ## Declared Write Set heading",
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
    let valid = !path.is_absolute()
        && path.starts_with("docs/work-packages")
        && path.file_name().is_some_and(|name| name == "package.md")
        && path
            .components()
            .all(|part| matches!(part, Component::Normal(_)));
    if valid {
        Ok(())
    } else {
        Err(package_error(
            "GATE-PACKAGE-PATH",
            path.display().to_string(),
        ))
    }
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
    use super::wildcard_match;

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
}
