use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path, PathBuf};

use crate::error::{AssuranceError, Result};

const FORBIDDEN_MARKERS: &[&str] = &[
    "/home/",
    "/workdir/",
    "/root/",
    "/tmp/",
    "c:\\users\\",
    "begin private key",
    "begin rsa private key",
    "begin openssh private key",
    "begin ec private key",
    "begin dsa private key",
    "begin encrypted private key",
    "begin pgp private key block",
    "akia",
    "asia",
    "ghp_",
    "gho_",
    "ghu_",
    "ghs_",
    "ghr_",
    "github_pat_",
    "glpat-",
    "sk-proj-",
    "sk_live_",
    "rk_live_",
    "aiza",
    "xoxa-",
    "xoxb-",
    "xoxp-",
    "xoxr-",
    "xoxs-",
    "authorization: bearer",
    "password=",
    "token=",
    "secret=",
];

pub(crate) fn validate_public_scalar(value: &str, label: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(AssuranceError::Invalid(format!(
            "public {label} must be nonempty"
        )));
    }
    if value != value.trim()
        || value.chars().any(char::is_control)
        || contains_unsafe_markdown_scalar(value)
    {
        return Err(AssuranceError::Invalid(format!(
            "public {label} contains Markdown-active or control syntax"
        )));
    }
    reject_disclosure(value, label)
}

pub(crate) fn validate_public_markdown(
    documents: &BTreeMap<PathBuf, String>,
    allowed_paths: &BTreeSet<PathBuf>,
) -> Result<()> {
    for (path, text) in documents {
        reject_disclosure(text, &format!("Markdown {}", path.display()))?;
        reject_nonlocal_syntax(text, path)?;
        for destination in markdown_destinations(text, path)? {
            validate_destination(path, destination, allowed_paths)?;
        }
    }
    Ok(())
}

pub(crate) fn validate_export(text: &str) -> Result<()> {
    reject_disclosure(text, "wepppy export")?;
    let lower = text.to_ascii_lowercase();
    if lower.contains("http://")
        || lower.contains("https://")
        || lower.contains("file://")
        || lower.contains("javascript:")
        || lower.contains("data:")
    {
        return Err(AssuranceError::Invalid(
            "wepppy export contains a forbidden URI".to_owned(),
        ));
    }
    Ok(())
}

fn reject_disclosure(text: &str, label: &str) -> Result<()> {
    let lower = text.to_ascii_lowercase();
    if let Some(marker) = FORBIDDEN_MARKERS
        .iter()
        .find(|marker| lower.contains(**marker))
    {
        return Err(AssuranceError::Invalid(format!(
            "{label} contains forbidden private-path or secret marker '{marker}'"
        )));
    }
    if contains_long_sk_token(&lower) {
        return Err(AssuranceError::Invalid(format!(
            "{label} contains a forbidden token-like sk- credential"
        )));
    }
    if contains_absolute_path(text) {
        return Err(AssuranceError::Invalid(format!(
            "{label} contains a forbidden absolute filesystem path"
        )));
    }
    Ok(())
}

fn contains_unsafe_markdown_scalar(value: &str) -> bool {
    if value
        .bytes()
        .any(|byte| matches!(byte, b'[' | b']' | b'{' | b'}' | b'*' | b'`' | b'\\' | b'|'))
    {
        return true;
    }
    let bytes = value.as_bytes();
    for (index, byte) in bytes.iter().enumerate() {
        if *byte == b'_'
            && (index == 0
                || index + 1 == bytes.len()
                || !bytes[index - 1].is_ascii_alphanumeric()
                || !bytes[index + 1].is_ascii_alphanumeric())
        {
            return true;
        }
    }
    let trimmed = value.trim_start();
    let hyphen_only = trimmed.bytes().all(|byte| byte == b'-');
    trimmed.starts_with('#')
        || trimmed.starts_with('>')
        || trimmed.starts_with("- ")
        || trimmed.starts_with("+ ")
        || trimmed == "+"
        || hyphen_only
        || trimmed.starts_with("~~~")
        || starts_ordered_list(trimmed)
        || value.contains("~~")
        || value.contains('<')
        || value.contains('>')
}

fn starts_ordered_list(value: &str) -> bool {
    let digits = value.bytes().take_while(u8::is_ascii_digit).count();
    if digits == 0 {
        return false;
    }
    let remainder = &value.as_bytes()[digits..];
    remainder.first().is_some_and(|marker| {
        matches!(*marker, b'.' | b')')
            && (remainder.len() == 1 || remainder[1].is_ascii_whitespace())
    })
}

fn contains_absolute_path(text: &str) -> bool {
    let bytes = text.as_bytes();
    for index in 0..bytes.len() {
        let boundary = index == 0 || is_path_boundary(bytes[index - 1]);
        if !boundary {
            continue;
        }
        if bytes[index] == b'/'
            && bytes
                .get(index + 1)
                .is_none_or(|next| !next.is_ascii_whitespace())
        {
            return true;
        }
        if bytes[index] == b'\\' && bytes.get(index + 1) == Some(&b'\\') {
            return true;
        }
        if bytes[index].is_ascii_alphabetic()
            && bytes.get(index + 1) == Some(&b':')
            && bytes
                .get(index + 2)
                .is_some_and(|separator| matches!(separator, b'/' | b'\\'))
        {
            return true;
        }
    }
    false
}

fn is_path_boundary(byte: u8) -> bool {
    !byte.is_ascii_alphanumeric() && !matches!(byte, b'.' | b'_' | b'-' | b'~' | b'/' | b'\\')
}

fn contains_long_sk_token(text: &str) -> bool {
    let bytes = text.as_bytes();
    for (index, prefix) in bytes.windows(3).enumerate() {
        if prefix != b"sk-" || (index > 0 && bytes[index - 1].is_ascii_alphanumeric()) {
            continue;
        }
        let token_length = bytes[index + 3..]
            .iter()
            .take_while(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
            .count();
        if token_length >= 20 {
            return true;
        }
    }
    false
}

fn reject_nonlocal_syntax(text: &str, path: &Path) -> Result<()> {
    let lower = text.to_ascii_lowercase();
    for marker in [
        "http://",
        "https://",
        "file://",
        "mailto:",
        "javascript:",
        "data:",
        "<a ",
        "<img ",
        "href=",
        "src=",
    ] {
        if lower.contains(marker) {
            return Err(AssuranceError::Invalid(format!(
                "public Markdown {} contains forbidden link syntax '{marker}'",
                path.display()
            )));
        }
    }
    let body = if text.starts_with("<!-- Generated by openwepp-assurance;") {
        text.split_once("-->\n").map_or(text, |(_, body)| body)
    } else {
        text
    };
    if body.contains('<') || body.contains('>') {
        return Err(AssuranceError::Invalid(format!(
            "public Markdown {} contains raw HTML or angle-bracket syntax",
            path.display()
        )));
    }
    Ok(())
}

fn markdown_destinations<'a>(text: &'a str, path: &Path) -> Result<Vec<&'a str>> {
    let mut destinations = text
        .split("](")
        .skip(1)
        .filter_map(|tail| tail.split(')').next())
        .collect::<Vec<_>>();
    for line in text.lines() {
        let trimmed = line.trim_start();
        if !trimmed.starts_with('[') {
            continue;
        }
        let Some((_, target)) = trimmed.split_once("]:") else {
            continue;
        };
        let Some(destination) = target.split_whitespace().next() else {
            return Err(AssuranceError::Invalid(format!(
                "public Markdown {} contains an empty or multiline reference destination",
                path.display()
            )));
        };
        destinations.push(destination);
    }
    Ok(destinations)
}

fn validate_destination(
    source: &Path,
    raw_destination: &str,
    allowed_paths: &BTreeSet<PathBuf>,
) -> Result<()> {
    if raw_destination.contains('#') {
        return unsafe_link(source, raw_destination);
    }
    let destination = raw_destination;
    if destination.is_empty() {
        return Ok(());
    }
    if destination.starts_with('/')
        || destination.contains('%')
        || destination.contains('\\')
        || destination.contains('?')
    {
        return unsafe_link(source, raw_destination);
    }
    let parent = source.parent().ok_or_else(|| {
        AssuranceError::Invalid(format!(
            "public document has no parent: {}",
            source.display()
        ))
    })?;
    let resolved = normalize_join(parent, Path::new(destination))
        .ok_or_else(|| AssuranceError::Invalid("public link escapes usersum".to_owned()))?;
    if !resolved.starts_with("usersum") || !allowed_paths.contains(&resolved) {
        return unsafe_link(source, raw_destination);
    }
    Ok(())
}

fn normalize_join(base: &Path, relative: &Path) -> Option<PathBuf> {
    let mut parts = base
        .components()
        .filter_map(|component| match component {
            Component::Normal(value) => Some(value.to_owned()),
            _ => None,
        })
        .collect::<Vec<_>>();
    for component in relative.components() {
        match component {
            Component::Normal(value) => parts.push(value.to_owned()),
            Component::ParentDir => {
                parts.pop()?;
            }
            Component::CurDir => {}
            _ => return None,
        }
    }
    Some(parts.into_iter().collect())
}

fn unsafe_link<T>(source: &Path, destination: &str) -> Result<T> {
    Err(AssuranceError::Invalid(format!(
        "public link in {} is outside the declared usersum set: {destination}",
        source.display()
    )))
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};
    use std::path::PathBuf;

    use super::{validate_export, validate_public_markdown, validate_public_scalar};

    #[test]
    fn rejects_active_or_private_public_scalars() {
        for value in [
            "[internal](/admin/private)",
            "![image](outside.png)",
            "<script>",
            "/tmp/operator/result",
            "ghp_exampletoken",
            "-----BEGIN OPENSSH PRIVATE KEY-----",
            "-----BEGIN EC PRIVATE KEY-----",
            "sk-proj-exampletoken",
            "sk-1234567890abcdefghijklmnopqrstuv",
            "`code`",
            "**emphasis**",
            "[reference][id]",
            "{{NARRATIVE_LINK}}",
            "# injected heading",
            "- injected list item",
            "-",
            "+",
            "1.",
            "1)",
            "---",
            "----",
            "1. injected ordered item",
            "value | injected table cell",
            "    injected code",
            "hard break  ",
            "_injected_emphasis_",
            "/etc/shadow",
            "report,/etc/shadow",
            "report;/var/lib/private",
            "report;/@private/secret",
            "report;/+private/secret",
            "report;/私密/secret",
            "/",
            "C:\\Users\\operator\\secret.txt",
            "report,C:\\Users\\operator\\secret.txt",
            "\\\\server\\private\\secret.txt",
            "report;\\\\server\\private\\secret.txt",
        ] {
            assert!(validate_public_scalar(value, "test scalar").is_err());
        }
    }

    #[test]
    fn rejects_nonlocal_reference_uri_html_and_secret_markdown() {
        let path = PathBuf::from("usersum/assurance/test.md");
        let allowed = BTreeSet::from([path.clone(), PathBuf::from("usersum/assurance/allowed.md")]);
        for value in [
            "[root](/admin/private)",
            "[external](https://example.test)",
            "[reference][unsafe]\n\n[unsafe]:../../private.md",
            "[text][ref]\n\n[ref]:\n  ../../private.md",
            "<script>alert(1)</script>",
            "token ghp_exampletoken",
            "token sk-1234567890abcdefghijklmnopqrstuv",
            "key -----BEGIN DSA PRIVATE KEY-----",
            "private /home/operator/result",
            "private /var/lib/openwepp/secret",
        ] {
            let documents = BTreeMap::from([(path.clone(), value.to_owned())]);
            assert!(validate_public_markdown(&documents, &allowed).is_err());
        }
        let documents = BTreeMap::from([(path.clone(), "[allowed](allowed.md)".to_owned())]);
        assert!(validate_public_markdown(&documents, &allowed).is_ok());
        for value in ["[fragment](allowed.md#result)", "[local](#result)"] {
            let documents = BTreeMap::from([(path.clone(), value.to_owned())]);
            assert!(validate_public_markdown(&documents, &allowed).is_err());
        }
    }

    #[test]
    fn rejects_export_uris_and_secret_markers() {
        assert!(validate_export("rel_path: https://example.test/doc.md").is_err());
        assert!(validate_export("title: ghp_exampletoken").is_err());
        assert!(validate_export("title: sk-1234567890abcdefghijklmnopqrstuv").is_err());
        assert!(validate_export("title: /Users/operator/private.txt").is_err());
    }
}
