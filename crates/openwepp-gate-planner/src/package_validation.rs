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
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::{validate_package_chain, wildcard_match};

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
        fixture.write_package("root", "- `src/**`");
        let base = fixture.commit("base authority");
        fixture.write_source("src/single.rs", "pub fn single() {}\n");
        let single_head = fixture.commit("single correction");
        let single = validate_package_chain(&fixture.root, &base, Some(&single_head))
            .expect("single authority chain");
        assert_eq!(single["status"], "READY");
        assert_eq!(
            single["steps"][0]["authorities"][0]["package_path"],
            "docs/work-packages/root/package.md"
        );

        fixture.write_package("child", "- `docs/work-packages/child/**`\n- `src/child.rs`");
        let scaffold = fixture.commit("child scaffold");
        fixture.write_source("src/child.rs", "pub fn child() {}\n");
        let head = fixture.commit("child correction");
        let chain = validate_package_chain(&fixture.root, &single_head, Some(&head))
            .expect("sequential authority chain");
        assert_eq!(chain["status"], "READY");
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
        let zero_base = zero.commit("empty base");
        zero.write_source("src/unowned.rs", "pub fn unowned() {}\n");
        let zero_head = zero.commit("unowned correction");
        zero.assert_invalid(&zero_base, &zero_head, "NO_PREEXISTING_AUTHORITY");

        let ambiguous = ChainFixture::new();
        ambiguous.write_package("one", "- `src/**`");
        ambiguous.write_package("two", "- `src/**`");
        let ambiguous_base = ambiguous.commit("ambiguous base");
        ambiguous.write_source("src/value.rs", "pub fn value() {}\n");
        let ambiguous_head = ambiguous.commit("ambiguous correction");
        ambiguous.assert_invalid(
            &ambiguous_base,
            &ambiguous_head,
            "AMBIGUOUS_PREEXISTING_AUTHORITY",
        );

        let retroactive = ChainFixture::new();
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
        let malformed_base = malformed.commit("empty base");
        malformed.write_raw_package("broken", "# Broken\n");
        let malformed_head = malformed.commit("malformed scaffold");
        malformed.assert_invalid(
            &malformed_base,
            &malformed_head,
            "SCAFFOLD_WRITE_SET_SCHEMA_INVALID",
        );

        let unmet = ChainFixture::new();
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
        let chain = validate_package_chain(&fixture.root, &base, Some(&head))
            .expect("prospective amendment chain");
        assert_eq!(chain["status"], "READY");
        assert_eq!(chain["steps"].as_array().map(Vec::len), Some(2));
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
            let chain = validate_package_chain(&self.root, base, Some(head))
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
